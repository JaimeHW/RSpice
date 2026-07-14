#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 15] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI_SHOT_NOISE", label: Some("Ibei shot noise"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_IBEI_FLICKER_NOISE", label: Some("Ibei flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_EI_IBEX_SHOT_NOISE", label: Some("Ibex shot noise"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_EI_IBEX_FLICKER_NOISE", label: Some("Ibex flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BP_IBEP_SHOT_NOISE", label: Some("Ibep shot noise"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_BP_IBEP_FLICKER_NOISE", label: Some("Ibep flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CX_RCX_THERMAL_NOISE", label: Some("rcx thermal noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CX_CI_RCI_THERMAL_NOISE", label: Some("rci thermal noise"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BX_RBX_THERMAL_NOISE", label: Some("rbx thermal noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BI_RBI_THERMAL_NOISE", label: Some("rbi thermal noise"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE_THERMAL_NOISE", label: Some("re thermal noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CX_RBP_THERMAL_NOISE", label: Some("rbp thermal noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_SI_PARASITIC_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("parasitic transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS_THERMAL_NOISE", label: Some("rs thermal noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
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
        let noise_source_active = match source_index {
            0 => {
                params.p1 != 0.0
            }
            1 => {
                params.p1 != 0.0
            }
            2 => {
                params.p1 != 0.0
            }
            3 => {
                params.p1 != 0.0
            }
            4 => {
                params.p1 != 0.0
            }
            5 => {
                params.p1 != 0.0
            }
            6 => {
                params.p1 != 0.0
            }
            7 => {
                params.p1 != 0.0
            }
            8 => {
                params.p1 != 0.0
            }
            9 => {
                params.p1 != 0.0
            }
            10 => {
                params.p1 != 0.0
            }
            11 => {
                params.p1 != 0.0
            }
            12 => {
                params.p1 != 0.0
            }
            13 => {
                params.p1 != 0.0
            }
            14 => {
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
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_0_e408: f64 = if ctx.analysis_initial_step() { 1.0 } else { 0.0 };
            noise_variable_172 = noise_metadata_schedule_0_e408;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_7_e452: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            noise_variable_175 = noise_metadata_schedule_7_e452;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_8_e458,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_175 != 0.0)) {
        (params.p10,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_8_e458;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_9_e467,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_175 == 0.0)) {
        let noise_metadata_schedule_9_e465: f64 = 1e-12;
        (noise_metadata_schedule_9_e465,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_9_e467;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_10_e469: f64 = if self.param_given[11] { 1.0 } else { 0.0 };
            noise_variable_176 = noise_metadata_schedule_10_e469;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_11_e475,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_176 != 0.0)) {
        (params.p11,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_11_e475;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_12_e484,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_176 == 0.0)) {
        let noise_metadata_schedule_12_e482: f64 = 1.0;
        (noise_metadata_schedule_12_e482,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_12_e484;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_13_e486: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            noise_variable_177 = noise_metadata_schedule_13_e486;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_14_e493,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_177 != 0.0)) {
        let noise_metadata_schedule_14_e491: f64 = 1.0;
        (noise_metadata_schedule_14_e491,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_14_e493;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_15_e495: f64 = if self.param_given[4] { 1.0 } else { 0.0 };
            noise_variable_178 = noise_metadata_schedule_15_e495;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_16_e505,) = {
    if (((noise_variable_172 != 0.0) && (noise_variable_177 == 0.0)) && (noise_variable_178 != 0.0)) {
        let noise_metadata_schedule_16_e503: f64 = (-1.0);
        (noise_metadata_schedule_16_e503,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_16_e505;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_17_e507: f64 = if self.param_given[5] { 1.0 } else { 0.0 };
            noise_variable_179 = noise_metadata_schedule_17_e507;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_18_e519,) = {
    if ((((noise_variable_172 != 0.0) && (noise_variable_177 == 0.0)) && (noise_variable_178 == 0.0)) && (noise_variable_179 != 0.0)) {
        (params.p5,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_18_e519;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_19_e533,) = {
    if ((((noise_variable_172 != 0.0) && (noise_variable_177 == 0.0)) && (noise_variable_178 == 0.0)) && (noise_variable_179 == 0.0)) {
        let noise_metadata_schedule_19_e531: f64 = 1.0;
        (noise_metadata_schedule_19_e531,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_19_e533;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_20_e538,) = {
    if (noise_variable_172 != 0.0) {
        let noise_metadata_schedule_20_e536: f64 = (params.p12).ln();
        (noise_metadata_schedule_20_e536,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_20_e538;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_21_e549,) = {
    if (noise_variable_172 != 0.0) {
        let (noise_metadata_schedule_21_e547,) = {
            if (params.p74 > 0.0) {
                let noise_metadata_schedule_21_e545: f64 = (1.0 / params.p74);
                (noise_metadata_schedule_21_e545,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_21_e547,)
    } else {
        (noise_variable_46,)
    }
};
            noise_variable_46 = noise_metadata_schedule_21_e549;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let (noise_metadata_schedule_22_e560,) = {
    if (noise_variable_172 != 0.0) {
        let (noise_metadata_schedule_22_e558,) = {
            if (params.p75 > 0.0) {
                let noise_metadata_schedule_22_e556: f64 = (1.0 / params.p75);
                (noise_metadata_schedule_22_e556,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_22_e558,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_22_e560;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_23_e571,) = {
    if (noise_variable_172 != 0.0) {
        let (noise_metadata_schedule_23_e569,) = {
            if (params.p20 > 0.0) {
                let noise_metadata_schedule_23_e567: f64 = (1.0 / params.p20);
                (noise_metadata_schedule_23_e567,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_23_e569,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_23_e571;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let (noise_metadata_schedule_27_e608,) = {
    if (noise_variable_172 != 0.0) {
        let noise_metadata_schedule_27_e606: f64 = (273.15 + params.p13);
        (noise_metadata_schedule_27_e606,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_27_e608;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_29_e610: f64 = ctx.temperature();
            let noise_metadata_schedule_29_e612: f64 = (noise_metadata_schedule_29_e610 + params.p0);
            let noise_metadata_schedule_29_e614: f64 = (noise_metadata_schedule_29_e612 - 273.15);
            noise_variable_38 = noise_metadata_schedule_29_e614;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_32_e624: f64 = (params.p14 + 1.0);
            let noise_metadata_schedule_32_e625: f64 = if noise_variable_38 < noise_metadata_schedule_32_e624 { 1.0 } else { 0.0 };
            noise_variable_182 = noise_metadata_schedule_32_e625;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_33_e636,) = {
    if (noise_variable_182 != 0.0) {
        let noise_metadata_schedule_33_e630: f64 = (noise_variable_38 - params.p14);
        let noise_metadata_schedule_33_e632: f64 = (noise_metadata_schedule_33_e630 - 1.0);
        let noise_metadata_schedule_33_e633: f64 = (noise_metadata_schedule_33_e632).exp();
        let noise_metadata_schedule_33_e634: f64 = (params.p14 + noise_metadata_schedule_33_e633);
        (noise_metadata_schedule_33_e634,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_33_e636;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_34_e640: f64 = (params.p15 - 1.0);
            let noise_metadata_schedule_34_e641: f64 = if noise_variable_38 > noise_metadata_schedule_34_e640 { 1.0 } else { 0.0 };
            noise_variable_183 = noise_metadata_schedule_34_e641;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_35_e655,) = {
    if ((noise_variable_182 == 0.0) && (noise_variable_183 != 0.0)) {
        let noise_metadata_schedule_35_e649: f64 = (params.p15 - noise_variable_38);
        let noise_metadata_schedule_35_e651: f64 = (noise_metadata_schedule_35_e649 - 1.0);
        let noise_metadata_schedule_35_e652: f64 = (noise_metadata_schedule_35_e651).exp();
        let noise_metadata_schedule_35_e653: f64 = (params.p15 - noise_metadata_schedule_35_e652);
        (noise_metadata_schedule_35_e653,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_35_e655;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_36_e663,) = {
    if ((noise_variable_182 == 0.0) && (noise_variable_183 == 0.0)) {
        (noise_variable_38,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_36_e663;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_37_e666: f64 = (noise_variable_38 + 273.15);
            noise_variable_39 = noise_metadata_schedule_37_e666;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_38_e669: f64 = (1.380662e-23 * noise_variable_39);
            let noise_metadata_schedule_38_e671: f64 = (noise_metadata_schedule_38_e669 / 1.602189e-19);
            noise_variable_73 = noise_metadata_schedule_38_e671;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_39_e674: f64 = (noise_variable_39 / noise_variable_40);
            noise_variable_41 = noise_metadata_schedule_39_e674;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_41_e687: f64 = if params.p90 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_184 = noise_metadata_schedule_41_e687;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_42_e706,) = {
    if (noise_variable_184 != 0.0) {
        let noise_metadata_schedule_42_e691: f64 = (params.p89 * noise_variable_73);
        let noise_metadata_schedule_42_e693: f64 = (-params.p88);
        let noise_metadata_schedule_42_e696: f64 = (params.p89 * noise_variable_73);
        let noise_metadata_schedule_42_e697: f64 = (noise_metadata_schedule_42_e693 / noise_metadata_schedule_42_e696);
        let noise_metadata_schedule_42_e698: f64 = (noise_metadata_schedule_42_e697).exp();
        let noise_metadata_schedule_42_e701: f64 = (noise_variable_166 / params.p90);
        let noise_metadata_schedule_42_e702: f64 = (noise_metadata_schedule_42_e698 + noise_metadata_schedule_42_e701);
        let noise_metadata_schedule_42_e703: f64 = (noise_metadata_schedule_42_e702).ln();
        let noise_metadata_schedule_42_e704: f64 = (noise_metadata_schedule_42_e691 * noise_metadata_schedule_42_e703);
        (noise_metadata_schedule_42_e704,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_42_e706;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_43_e711,) = {
    if (noise_variable_184 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_43_e711;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_44_e716: f64 = (params.p122 / params.p28);
            let noise_metadata_schedule_44_e717: f64 = (noise_variable_41).powf(noise_metadata_schedule_44_e716);
            let noise_metadata_schedule_44_e718: f64 = (params.p26 * noise_metadata_schedule_44_e717);
            let noise_metadata_schedule_44_e720: f64 = (-params.p113);
            let noise_metadata_schedule_44_e723: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_44_e724: f64 = (noise_metadata_schedule_44_e720 * noise_metadata_schedule_44_e723);
            let noise_metadata_schedule_44_e727: f64 = (noise_variable_73 * params.p28);
            let noise_metadata_schedule_44_e728: f64 = (noise_metadata_schedule_44_e724 / noise_metadata_schedule_44_e727);
            let noise_metadata_schedule_44_e729: f64 = (noise_metadata_schedule_44_e728).exp();
            let noise_metadata_schedule_44_e730: f64 = (noise_metadata_schedule_44_e718 * noise_metadata_schedule_44_e729);
            noise_variable_0 = noise_metadata_schedule_44_e730;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_45_e733: f64 = if noise_variable_0 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_185 = noise_metadata_schedule_45_e733;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_46_e740: f64 = if ((params.p72 > 0.0) && (noise_variable_166 > params.p72)) { 1.0 } else { 0.0 };
            noise_variable_186 = noise_metadata_schedule_46_e740;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_47_e769,) = {
    if ((noise_variable_185 != 0.0) && (noise_variable_186 != 0.0)) {
        let noise_metadata_schedule_47_e746: f64 = (params.p28 * noise_variable_73);
        let noise_metadata_schedule_47_e750: f64 = (0.5 * noise_variable_166);
        let noise_metadata_schedule_47_e753: f64 = (4.0 / params.p72);
        let noise_metadata_schedule_47_e755: f64 = (noise_metadata_schedule_47_e753).powf(params.p73);
        let noise_metadata_schedule_47_e756: f64 = (noise_metadata_schedule_47_e750 * noise_metadata_schedule_47_e755);
        let noise_metadata_schedule_47_e760: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_47_e761: f64 = (1.0 / noise_metadata_schedule_47_e760);
        let noise_metadata_schedule_47_e762: f64 = (noise_metadata_schedule_47_e756).powf(noise_metadata_schedule_47_e761);
        let noise_metadata_schedule_47_e764: f64 = (noise_metadata_schedule_47_e762 / noise_variable_0);
        let noise_metadata_schedule_47_e765: f64 = (1.0 + noise_metadata_schedule_47_e764);
        let noise_metadata_schedule_47_e766: f64 = (noise_metadata_schedule_47_e765).ln();
        let noise_metadata_schedule_47_e767: f64 = (noise_metadata_schedule_47_e746 * noise_metadata_schedule_47_e766);
        (noise_metadata_schedule_47_e767,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_47_e769;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_48_e785,) = {
    if ((noise_variable_185 != 0.0) && (noise_variable_186 == 0.0)) {
        let noise_metadata_schedule_48_e776: f64 = (params.p28 * noise_variable_73);
        let noise_metadata_schedule_48_e780: f64 = (noise_variable_166 / noise_variable_0);
        let noise_metadata_schedule_48_e781: f64 = (1.0 + noise_metadata_schedule_48_e780);
        let noise_metadata_schedule_48_e782: f64 = (noise_metadata_schedule_48_e781).ln();
        let noise_metadata_schedule_48_e783: f64 = (noise_metadata_schedule_48_e776 * noise_metadata_schedule_48_e782);
        (noise_metadata_schedule_48_e783,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_48_e785;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_49_e790,) = {
    if (noise_variable_185 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_49_e790;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_50_e795: f64 = (params.p125 / params.p29);
            let noise_metadata_schedule_50_e796: f64 = (noise_variable_41).powf(noise_metadata_schedule_50_e795);
            let noise_metadata_schedule_50_e797: f64 = (params.p27 * noise_metadata_schedule_50_e796);
            let noise_metadata_schedule_50_e799: f64 = (-params.p121);
            let noise_metadata_schedule_50_e802: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_50_e803: f64 = (noise_metadata_schedule_50_e799 * noise_metadata_schedule_50_e802);
            let noise_metadata_schedule_50_e806: f64 = (noise_variable_73 * params.p29);
            let noise_metadata_schedule_50_e807: f64 = (noise_metadata_schedule_50_e803 / noise_metadata_schedule_50_e806);
            let noise_metadata_schedule_50_e808: f64 = (noise_metadata_schedule_50_e807).exp();
            let noise_metadata_schedule_50_e809: f64 = (noise_metadata_schedule_50_e797 * noise_metadata_schedule_50_e808);
            noise_variable_1 = noise_metadata_schedule_50_e809;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_51_e816: f64 = if ((noise_variable_0 > 0.0) && (noise_variable_1 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_187 = noise_metadata_schedule_51_e816;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_52_e823: f64 = if ((params.p74 > 0.0) && (noise_variable_166 > params.p74)) { 1.0 } else { 0.0 };
            noise_variable_188 = noise_metadata_schedule_52_e823;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_53_e854,) = {
    if ((noise_variable_187 != 0.0) && (noise_variable_188 != 0.0)) {
        let noise_metadata_schedule_53_e829: f64 = (params.p29 * noise_variable_73);
        let noise_metadata_schedule_53_e833: f64 = (0.5 * noise_variable_166);
        let noise_metadata_schedule_53_e836: f64 = (4.0 / params.p74);
        let noise_metadata_schedule_53_e838: f64 = (noise_metadata_schedule_53_e836).powf(params.p73);
        let noise_metadata_schedule_53_e839: f64 = (noise_metadata_schedule_53_e833 * noise_metadata_schedule_53_e838);
        let noise_metadata_schedule_53_e843: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_53_e844: f64 = (1.0 / noise_metadata_schedule_53_e843);
        let noise_metadata_schedule_53_e845: f64 = (noise_metadata_schedule_53_e839).powf(noise_metadata_schedule_53_e844);
        let noise_metadata_schedule_53_e848: f64 = (noise_variable_0 * noise_variable_1);
        let noise_metadata_schedule_53_e849: f64 = (noise_metadata_schedule_53_e845 / noise_metadata_schedule_53_e848);
        let noise_metadata_schedule_53_e850: f64 = (1.0 + noise_metadata_schedule_53_e849);
        let noise_metadata_schedule_53_e851: f64 = (noise_metadata_schedule_53_e850).ln();
        let noise_metadata_schedule_53_e852: f64 = (noise_metadata_schedule_53_e829 * noise_metadata_schedule_53_e851);
        (noise_metadata_schedule_53_e852,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_53_e854;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_54_e872,) = {
    if ((noise_variable_187 != 0.0) && (noise_variable_188 == 0.0)) {
        let noise_metadata_schedule_54_e861: f64 = (params.p29 * noise_variable_73);
        let noise_metadata_schedule_54_e866: f64 = (noise_variable_0 * noise_variable_1);
        let noise_metadata_schedule_54_e867: f64 = (noise_variable_166 / noise_metadata_schedule_54_e866);
        let noise_metadata_schedule_54_e868: f64 = (1.0 + noise_metadata_schedule_54_e867);
        let noise_metadata_schedule_54_e869: f64 = (noise_metadata_schedule_54_e868).ln();
        let noise_metadata_schedule_54_e870: f64 = (noise_metadata_schedule_54_e861 * noise_metadata_schedule_54_e869);
        (noise_metadata_schedule_54_e870,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_54_e872;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_55_e877,) = {
    if (noise_variable_187 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_55_e877;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let noise_metadata_schedule_56_e882: f64 = (params.p122 / params.p33);
            let noise_metadata_schedule_56_e883: f64 = (noise_variable_41).powf(noise_metadata_schedule_56_e882);
            let noise_metadata_schedule_56_e884: f64 = (params.p31 * noise_metadata_schedule_56_e883);
            let noise_metadata_schedule_56_e886: f64 = (-params.p120);
            let noise_metadata_schedule_56_e889: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_56_e890: f64 = (noise_metadata_schedule_56_e886 * noise_metadata_schedule_56_e889);
            let noise_metadata_schedule_56_e893: f64 = (noise_variable_73 * params.p33);
            let noise_metadata_schedule_56_e894: f64 = (noise_metadata_schedule_56_e890 / noise_metadata_schedule_56_e893);
            let noise_metadata_schedule_56_e895: f64 = (noise_metadata_schedule_56_e894).exp();
            let noise_metadata_schedule_56_e896: f64 = (noise_metadata_schedule_56_e884 * noise_metadata_schedule_56_e895);
            noise_variable_5 = noise_metadata_schedule_56_e896;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let noise_metadata_schedule_57_e899: f64 = if noise_variable_5 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_189 = noise_metadata_schedule_57_e899;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let noise_metadata_schedule_58_e906: f64 = if ((params.p75 > 0.0) && (noise_variable_166 > params.p75)) { 1.0 } else { 0.0 };
            noise_variable_190 = noise_metadata_schedule_58_e906;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let (noise_metadata_schedule_59_e925,) = {
    if ((noise_variable_189 != 0.0) && (noise_variable_190 != 0.0)) {
        let noise_metadata_schedule_59_e912: f64 = (params.p33 * noise_variable_73);
        let noise_metadata_schedule_59_e916: f64 = (noise_variable_166 * noise_variable_166);
        let noise_metadata_schedule_59_e918: f64 = (noise_metadata_schedule_59_e916 * noise_variable_47);
        let noise_metadata_schedule_59_e920: f64 = (noise_metadata_schedule_59_e918 / noise_variable_5);
        let noise_metadata_schedule_59_e921: f64 = (1.0 + noise_metadata_schedule_59_e920);
        let noise_metadata_schedule_59_e922: f64 = (noise_metadata_schedule_59_e921).ln();
        let noise_metadata_schedule_59_e923: f64 = (noise_metadata_schedule_59_e912 * noise_metadata_schedule_59_e922);
        (noise_metadata_schedule_59_e923,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_59_e925;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let (noise_metadata_schedule_60_e941,) = {
    if ((noise_variable_189 != 0.0) && (noise_variable_190 == 0.0)) {
        let noise_metadata_schedule_60_e932: f64 = (params.p33 * noise_variable_73);
        let noise_metadata_schedule_60_e936: f64 = (noise_variable_166 / noise_variable_5);
        let noise_metadata_schedule_60_e937: f64 = (1.0 + noise_metadata_schedule_60_e936);
        let noise_metadata_schedule_60_e938: f64 = (noise_metadata_schedule_60_e937).ln();
        let noise_metadata_schedule_60_e939: f64 = (noise_metadata_schedule_60_e932 * noise_metadata_schedule_60_e938);
        (noise_metadata_schedule_60_e939,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_60_e941;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let (noise_metadata_schedule_61_e946,) = {
    if (noise_variable_189 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_61_e946;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_62_e951: f64 = (params.p123 / params.p56);
            let noise_metadata_schedule_62_e952: f64 = (noise_variable_41).powf(noise_metadata_schedule_62_e951);
            let noise_metadata_schedule_62_e953: f64 = (params.p54 * noise_metadata_schedule_62_e952);
            let noise_metadata_schedule_62_e955: f64 = (-params.p114);
            let noise_metadata_schedule_62_e958: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_62_e959: f64 = (noise_metadata_schedule_62_e955 * noise_metadata_schedule_62_e958);
            let noise_metadata_schedule_62_e962: f64 = (noise_variable_73 * params.p56);
            let noise_metadata_schedule_62_e963: f64 = (noise_metadata_schedule_62_e959 / noise_metadata_schedule_62_e962);
            let noise_metadata_schedule_62_e964: f64 = (noise_metadata_schedule_62_e963).exp();
            let noise_metadata_schedule_62_e965: f64 = (noise_metadata_schedule_62_e953 * noise_metadata_schedule_62_e964);
            noise_variable_3 = noise_metadata_schedule_62_e965;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_63_e968: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_191 = noise_metadata_schedule_63_e968;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_64_e981,) = {
    if (noise_variable_191 != 0.0) {
        let noise_metadata_schedule_64_e972: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_64_e976: f64 = (noise_variable_166 / noise_variable_3);
        let noise_metadata_schedule_64_e977: f64 = (1.0 + noise_metadata_schedule_64_e976);
        let noise_metadata_schedule_64_e978: f64 = (noise_metadata_schedule_64_e977).ln();
        let noise_metadata_schedule_64_e979: f64 = (noise_metadata_schedule_64_e972 * noise_metadata_schedule_64_e978);
        (noise_metadata_schedule_64_e979,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_64_e981;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_65_e986,) = {
    if (noise_variable_191 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_65_e986;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_66_e991: f64 = (params.p124 / params.p59);
            let noise_metadata_schedule_66_e992: f64 = (noise_variable_41).powf(noise_metadata_schedule_66_e991);
            let noise_metadata_schedule_66_e993: f64 = (params.p58 * noise_metadata_schedule_66_e992);
            let noise_metadata_schedule_66_e995: f64 = (-params.p117);
            let noise_metadata_schedule_66_e998: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_66_e999: f64 = (noise_metadata_schedule_66_e995 * noise_metadata_schedule_66_e998);
            let noise_metadata_schedule_66_e1002: f64 = (noise_variable_73 * params.p59);
            let noise_metadata_schedule_66_e1003: f64 = (noise_metadata_schedule_66_e999 / noise_metadata_schedule_66_e1002);
            let noise_metadata_schedule_66_e1004: f64 = (noise_metadata_schedule_66_e1003).exp();
            let noise_metadata_schedule_66_e1005: f64 = (noise_metadata_schedule_66_e993 * noise_metadata_schedule_66_e1004);
            noise_variable_6 = noise_metadata_schedule_66_e1005;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_67_e1008: f64 = if noise_variable_6 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_192 = noise_metadata_schedule_67_e1008;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_68_e1021,) = {
    if (noise_variable_192 != 0.0) {
        let noise_metadata_schedule_68_e1012: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_68_e1016: f64 = (noise_variable_166 / noise_variable_6);
        let noise_metadata_schedule_68_e1017: f64 = (1.0 + noise_metadata_schedule_68_e1016);
        let noise_metadata_schedule_68_e1018: f64 = (noise_metadata_schedule_68_e1017).ln();
        let noise_metadata_schedule_68_e1019: f64 = (noise_metadata_schedule_68_e1012 * noise_metadata_schedule_68_e1018);
        (noise_metadata_schedule_68_e1019,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_68_e1021;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_69_e1026,) = {
    if (noise_variable_192 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_69_e1026;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_70_e1031: f64 = (params.p123 / params.p61);
            let noise_metadata_schedule_70_e1032: f64 = (noise_variable_41).powf(noise_metadata_schedule_70_e1031);
            let noise_metadata_schedule_70_e1033: f64 = (params.p60 * noise_metadata_schedule_70_e1032);
            let noise_metadata_schedule_70_e1035: f64 = (-params.p115);
            let noise_metadata_schedule_70_e1038: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_70_e1039: f64 = (noise_metadata_schedule_70_e1035 * noise_metadata_schedule_70_e1038);
            let noise_metadata_schedule_70_e1042: f64 = (noise_variable_73 * params.p61);
            let noise_metadata_schedule_70_e1043: f64 = (noise_metadata_schedule_70_e1039 / noise_metadata_schedule_70_e1042);
            let noise_metadata_schedule_70_e1044: f64 = (noise_metadata_schedule_70_e1043).exp();
            let noise_metadata_schedule_70_e1045: f64 = (noise_metadata_schedule_70_e1033 * noise_metadata_schedule_70_e1044);
            noise_variable_4 = noise_metadata_schedule_70_e1045;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_71_e1048: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_193 = noise_metadata_schedule_71_e1048;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_72_e1061,) = {
    if (noise_variable_193 != 0.0) {
        let noise_metadata_schedule_72_e1052: f64 = (params.p61 * noise_variable_73);
        let noise_metadata_schedule_72_e1056: f64 = (noise_variable_166 / noise_variable_4);
        let noise_metadata_schedule_72_e1057: f64 = (1.0 + noise_metadata_schedule_72_e1056);
        let noise_metadata_schedule_72_e1058: f64 = (noise_metadata_schedule_72_e1057).ln();
        let noise_metadata_schedule_72_e1059: f64 = (noise_metadata_schedule_72_e1052 * noise_metadata_schedule_72_e1058);
        (noise_metadata_schedule_72_e1059,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_72_e1061;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_73_e1066,) = {
    if (noise_variable_193 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_73_e1066;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_74_e1071: f64 = (params.p124 / params.p63);
            let noise_metadata_schedule_74_e1072: f64 = (noise_variable_41).powf(noise_metadata_schedule_74_e1071);
            let noise_metadata_schedule_74_e1073: f64 = (params.p62 * noise_metadata_schedule_74_e1072);
            let noise_metadata_schedule_74_e1075: f64 = (-params.p118);
            let noise_metadata_schedule_74_e1078: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_74_e1079: f64 = (noise_metadata_schedule_74_e1075 * noise_metadata_schedule_74_e1078);
            let noise_metadata_schedule_74_e1082: f64 = (noise_variable_73 * params.p63);
            let noise_metadata_schedule_74_e1083: f64 = (noise_metadata_schedule_74_e1079 / noise_metadata_schedule_74_e1082);
            let noise_metadata_schedule_74_e1084: f64 = (noise_metadata_schedule_74_e1083).exp();
            let noise_metadata_schedule_74_e1085: f64 = (noise_metadata_schedule_74_e1073 * noise_metadata_schedule_74_e1084);
            noise_variable_7 = noise_metadata_schedule_74_e1085;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_75_e1088: f64 = if noise_variable_7 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_194 = noise_metadata_schedule_75_e1088;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_76_e1101,) = {
    if (noise_variable_194 != 0.0) {
        let noise_metadata_schedule_76_e1092: f64 = (params.p63 * noise_variable_73);
        let noise_metadata_schedule_76_e1096: f64 = (noise_variable_166 / noise_variable_7);
        let noise_metadata_schedule_76_e1097: f64 = (1.0 + noise_metadata_schedule_76_e1096);
        let noise_metadata_schedule_76_e1098: f64 = (noise_metadata_schedule_76_e1097).ln();
        let noise_metadata_schedule_76_e1099: f64 = (noise_metadata_schedule_76_e1092 * noise_metadata_schedule_76_e1098);
        (noise_metadata_schedule_76_e1099,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_76_e1101;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_77_e1106,) = {
    if (noise_variable_194 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_77_e1106;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_78_e1111: f64 = (params.p123 / params.p61);
            let noise_metadata_schedule_78_e1112: f64 = (noise_variable_41).powf(noise_metadata_schedule_78_e1111);
            let noise_metadata_schedule_78_e1113: f64 = (params.p64 * noise_metadata_schedule_78_e1112);
            let noise_metadata_schedule_78_e1115: f64 = (-params.p115);
            let noise_metadata_schedule_78_e1118: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_78_e1119: f64 = (noise_metadata_schedule_78_e1115 * noise_metadata_schedule_78_e1118);
            let noise_metadata_schedule_78_e1122: f64 = (noise_variable_73 * params.p61);
            let noise_metadata_schedule_78_e1123: f64 = (noise_metadata_schedule_78_e1119 / noise_metadata_schedule_78_e1122);
            let noise_metadata_schedule_78_e1124: f64 = (noise_metadata_schedule_78_e1123).exp();
            let noise_metadata_schedule_78_e1125: f64 = (noise_metadata_schedule_78_e1113 * noise_metadata_schedule_78_e1124);
            noise_variable_8 = noise_metadata_schedule_78_e1125;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_79_e1128: f64 = if noise_variable_8 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_195 = noise_metadata_schedule_79_e1128;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_80_e1141,) = {
    if (noise_variable_195 != 0.0) {
        let noise_metadata_schedule_80_e1132: f64 = (params.p61 * noise_variable_73);
        let noise_metadata_schedule_80_e1136: f64 = (noise_variable_166 / noise_variable_8);
        let noise_metadata_schedule_80_e1137: f64 = (1.0 + noise_metadata_schedule_80_e1136);
        let noise_metadata_schedule_80_e1138: f64 = (noise_metadata_schedule_80_e1137).ln();
        let noise_metadata_schedule_80_e1139: f64 = (noise_metadata_schedule_80_e1132 * noise_metadata_schedule_80_e1138);
        (noise_metadata_schedule_80_e1139,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_80_e1141;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_81_e1146,) = {
    if (noise_variable_195 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_81_e1146;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_82_e1151: f64 = (params.p124 / params.p63);
            let noise_metadata_schedule_82_e1152: f64 = (noise_variable_41).powf(noise_metadata_schedule_82_e1151);
            let noise_metadata_schedule_82_e1153: f64 = (params.p65 * noise_metadata_schedule_82_e1152);
            let noise_metadata_schedule_82_e1155: f64 = (-params.p118);
            let noise_metadata_schedule_82_e1158: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_82_e1159: f64 = (noise_metadata_schedule_82_e1155 * noise_metadata_schedule_82_e1158);
            let noise_metadata_schedule_82_e1162: f64 = (noise_variable_73 * params.p63);
            let noise_metadata_schedule_82_e1163: f64 = (noise_metadata_schedule_82_e1159 / noise_metadata_schedule_82_e1162);
            let noise_metadata_schedule_82_e1164: f64 = (noise_metadata_schedule_82_e1163).exp();
            let noise_metadata_schedule_82_e1165: f64 = (noise_metadata_schedule_82_e1153 * noise_metadata_schedule_82_e1164);
            noise_variable_9 = noise_metadata_schedule_82_e1165;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_83_e1168: f64 = if noise_variable_9 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_196 = noise_metadata_schedule_83_e1168;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_84_e1181,) = {
    if (noise_variable_196 != 0.0) {
        let noise_metadata_schedule_84_e1172: f64 = (params.p63 * noise_variable_73);
        let noise_metadata_schedule_84_e1176: f64 = (noise_variable_166 / noise_variable_9);
        let noise_metadata_schedule_84_e1177: f64 = (1.0 + noise_metadata_schedule_84_e1176);
        let noise_metadata_schedule_84_e1178: f64 = (noise_metadata_schedule_84_e1177).ln();
        let noise_metadata_schedule_84_e1179: f64 = (noise_metadata_schedule_84_e1172 * noise_metadata_schedule_84_e1178);
        (noise_metadata_schedule_84_e1179,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_84_e1181;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_85_e1186,) = {
    if (noise_variable_196 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_85_e1186;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            noise_variable_138 = (ctx.node_voltage(self.nodes[4]) - 0.0);
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_95_e1268: f64 = ctx.temperature();
            let noise_metadata_schedule_95_e1270: f64 = (noise_metadata_schedule_95_e1268 + params.p0);
            let noise_metadata_schedule_95_e1272: f64 = (noise_metadata_schedule_95_e1270 + noise_variable_138);
            let noise_metadata_schedule_95_e1274: f64 = (noise_metadata_schedule_95_e1272 - 273.15);
            noise_variable_38 = noise_metadata_schedule_95_e1274;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_96_e1278: f64 = (params.p14 + 1.0);
            let noise_metadata_schedule_96_e1279: f64 = if noise_variable_38 < noise_metadata_schedule_96_e1278 { 1.0 } else { 0.0 };
            noise_variable_199 = noise_metadata_schedule_96_e1279;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let (noise_metadata_schedule_97_e1290,) = {
    if (noise_variable_199 != 0.0) {
        let noise_metadata_schedule_97_e1284: f64 = (noise_variable_38 - params.p14);
        let noise_metadata_schedule_97_e1286: f64 = (noise_metadata_schedule_97_e1284 - 1.0);
        let noise_metadata_schedule_97_e1287: f64 = (noise_metadata_schedule_97_e1286).exp();
        let noise_metadata_schedule_97_e1288: f64 = (params.p14 + noise_metadata_schedule_97_e1287);
        (noise_metadata_schedule_97_e1288,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_97_e1290;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_98_e1294: f64 = (params.p15 - 1.0);
            let noise_metadata_schedule_98_e1295: f64 = if noise_variable_38 > noise_metadata_schedule_98_e1294 { 1.0 } else { 0.0 };
            noise_variable_200 = noise_metadata_schedule_98_e1295;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let (noise_metadata_schedule_99_e1309,) = {
    if ((noise_variable_199 == 0.0) && (noise_variable_200 != 0.0)) {
        let noise_metadata_schedule_99_e1303: f64 = (params.p15 - noise_variable_38);
        let noise_metadata_schedule_99_e1305: f64 = (noise_metadata_schedule_99_e1303 - 1.0);
        let noise_metadata_schedule_99_e1306: f64 = (noise_metadata_schedule_99_e1305).exp();
        let noise_metadata_schedule_99_e1307: f64 = (params.p15 - noise_metadata_schedule_99_e1306);
        (noise_metadata_schedule_99_e1307,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_99_e1309;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let (noise_metadata_schedule_100_e1317,) = {
    if ((noise_variable_199 == 0.0) && (noise_variable_200 == 0.0)) {
        (noise_variable_38,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_100_e1317;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_101_e1320: f64 = (noise_variable_38 + 273.15);
            noise_variable_39 = noise_metadata_schedule_101_e1320;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_102_e1323: f64 = (1.380662e-23 * noise_variable_39);
            let noise_metadata_schedule_102_e1325: f64 = (noise_metadata_schedule_102_e1323 / 1.602189e-19);
            noise_variable_73 = noise_metadata_schedule_102_e1325;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14) {
            let noise_metadata_schedule_103_e1328: f64 = (noise_variable_39 / noise_variable_40);
            noise_variable_41 = noise_metadata_schedule_103_e1328;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_104_e1331: f64 = (noise_variable_39 - noise_variable_40);
            noise_variable_42 = noise_metadata_schedule_104_e1331;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_105_e1335: f64 = (noise_variable_41).powf(params.p126);
            let noise_metadata_schedule_105_e1336: f64 = (params.p72 * noise_metadata_schedule_105_e1335);
            noise_variable_2 = noise_metadata_schedule_105_e1336;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_106_e1338: f64 = if self.param_given[109] { 1.0 } else { 0.0 };
            noise_variable_201 = noise_metadata_schedule_106_e1338;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_107_e1346,) = {
    if (noise_variable_201 != 0.0) {
        let noise_metadata_schedule_107_e1343: f64 = (noise_variable_41).powf(params.p109);
        let noise_metadata_schedule_107_e1344: f64 = (params.p16 * noise_metadata_schedule_107_e1343);
        (noise_metadata_schedule_107_e1344,)
    } else {
        (noise_variable_12,)
    }
};
            noise_variable_12 = noise_metadata_schedule_107_e1346;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_108_e1355,) = {
    if (noise_variable_201 == 0.0) {
        let noise_metadata_schedule_108_e1352: f64 = (noise_variable_41).powf(params.p107);
        let noise_metadata_schedule_108_e1353: f64 = (params.p16 * noise_metadata_schedule_108_e1352);
        (noise_metadata_schedule_108_e1353,)
    } else {
        (noise_variable_12,)
    }
};
            noise_variable_12 = noise_metadata_schedule_108_e1355;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_109_e1357: f64 = if self.param_given[108] { 1.0 } else { 0.0 };
            noise_variable_202 = noise_metadata_schedule_109_e1357;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_110_e1365,) = {
    if (noise_variable_202 != 0.0) {
        let noise_metadata_schedule_110_e1362: f64 = (noise_variable_41).powf(params.p108);
        let noise_metadata_schedule_110_e1363: f64 = (params.p17 * noise_metadata_schedule_110_e1362);
        (noise_metadata_schedule_110_e1363,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_110_e1365;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_111_e1374,) = {
    if (noise_variable_202 == 0.0) {
        let noise_metadata_schedule_111_e1371: f64 = (noise_variable_41).powf(params.p107);
        let noise_metadata_schedule_111_e1372: f64 = (params.p17 * noise_metadata_schedule_111_e1371);
        (noise_metadata_schedule_111_e1372,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_111_e1374;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_112_e1376: f64 = if self.param_given[106] { 1.0 } else { 0.0 };
            noise_variable_203 = noise_metadata_schedule_112_e1376;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_113_e1384,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_113_e1381: f64 = (noise_variable_41).powf(params.p106);
        let noise_metadata_schedule_113_e1382: f64 = (params.p21 * noise_metadata_schedule_113_e1381);
        (noise_metadata_schedule_113_e1382,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_113_e1384;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_114_e1393,) = {
    if (noise_variable_203 == 0.0) {
        let noise_metadata_schedule_114_e1390: f64 = (noise_variable_41).powf(params.p104);
        let noise_metadata_schedule_114_e1391: f64 = (params.p21 * noise_metadata_schedule_114_e1390);
        (noise_metadata_schedule_114_e1391,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_114_e1393;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_115_e1395: f64 = if self.param_given[105] { 1.0 } else { 0.0 };
            noise_variable_204 = noise_metadata_schedule_115_e1395;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_116_e1403,) = {
    if (noise_variable_204 != 0.0) {
        let noise_metadata_schedule_116_e1400: f64 = (noise_variable_41).powf(params.p105);
        let noise_metadata_schedule_116_e1401: f64 = (params.p22 * noise_metadata_schedule_116_e1400);
        (noise_metadata_schedule_116_e1401,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_116_e1403;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_117_e1412,) = {
    if (noise_variable_204 == 0.0) {
        let noise_metadata_schedule_117_e1409: f64 = (noise_variable_41).powf(params.p104);
        let noise_metadata_schedule_117_e1410: f64 = (params.p22 * noise_metadata_schedule_117_e1409);
        (noise_metadata_schedule_117_e1410,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_117_e1412;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_118_e1416: f64 = (noise_variable_41).powf(params.p103);
            let noise_metadata_schedule_118_e1417: f64 = (params.p23 * noise_metadata_schedule_118_e1416);
            noise_variable_16 = noise_metadata_schedule_118_e1417;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_119_e1421: f64 = (noise_variable_41).powf(params.p111);
            let noise_metadata_schedule_119_e1422: f64 = (params.p24 * noise_metadata_schedule_119_e1421);
            noise_variable_17 = noise_metadata_schedule_119_e1422;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_120_e1424: f64 = if self.param_given[110] { 1.0 } else { 0.0 };
            noise_variable_205 = noise_metadata_schedule_120_e1424;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_121_e1432,) = {
    if (noise_variable_205 != 0.0) {
        let noise_metadata_schedule_121_e1429: f64 = (noise_variable_41).powf(params.p110);
        let noise_metadata_schedule_121_e1430: f64 = (params.p25 * noise_metadata_schedule_121_e1429);
        (noise_metadata_schedule_121_e1430,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_121_e1432;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_122_e1441,) = {
    if (noise_variable_205 == 0.0) {
        let noise_metadata_schedule_122_e1438: f64 = (noise_variable_41).powf(params.p107);
        let noise_metadata_schedule_122_e1439: f64 = (params.p25 * noise_metadata_schedule_122_e1438);
        (noise_metadata_schedule_122_e1439,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_122_e1441;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_124_e1453: f64 = (params.p122 / params.p28);
            let noise_metadata_schedule_124_e1454: f64 = (noise_variable_41).powf(noise_metadata_schedule_124_e1453);
            let noise_metadata_schedule_124_e1455: f64 = (params.p26 * noise_metadata_schedule_124_e1454);
            let noise_metadata_schedule_124_e1457: f64 = (-params.p113);
            let noise_metadata_schedule_124_e1460: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_124_e1461: f64 = (noise_metadata_schedule_124_e1457 * noise_metadata_schedule_124_e1460);
            let noise_metadata_schedule_124_e1464: f64 = (noise_variable_73 * params.p28);
            let noise_metadata_schedule_124_e1465: f64 = (noise_metadata_schedule_124_e1461 / noise_metadata_schedule_124_e1464);
            let noise_metadata_schedule_124_e1466: f64 = (noise_metadata_schedule_124_e1465).exp();
            let noise_metadata_schedule_124_e1467: f64 = (noise_metadata_schedule_124_e1455 * noise_metadata_schedule_124_e1466);
            noise_variable_0 = noise_metadata_schedule_124_e1467;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_125_e1472: f64 = (params.p125 / params.p29);
            let noise_metadata_schedule_125_e1473: f64 = (noise_variable_41).powf(noise_metadata_schedule_125_e1472);
            let noise_metadata_schedule_125_e1474: f64 = (params.p27 * noise_metadata_schedule_125_e1473);
            let noise_metadata_schedule_125_e1476: f64 = (-params.p121);
            let noise_metadata_schedule_125_e1479: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_125_e1480: f64 = (noise_metadata_schedule_125_e1476 * noise_metadata_schedule_125_e1479);
            let noise_metadata_schedule_125_e1483: f64 = (noise_variable_73 * params.p29);
            let noise_metadata_schedule_125_e1484: f64 = (noise_metadata_schedule_125_e1480 / noise_metadata_schedule_125_e1483);
            let noise_metadata_schedule_125_e1485: f64 = (noise_metadata_schedule_125_e1484).exp();
            let noise_metadata_schedule_125_e1486: f64 = (noise_metadata_schedule_125_e1474 * noise_metadata_schedule_125_e1485);
            noise_variable_1 = noise_metadata_schedule_125_e1486;
        }
        if matches!(source_index, 12 | 13) {
            let noise_metadata_schedule_126_e1491: f64 = (params.p122 / params.p33);
            let noise_metadata_schedule_126_e1492: f64 = (noise_variable_41).powf(noise_metadata_schedule_126_e1491);
            let noise_metadata_schedule_126_e1493: f64 = (params.p31 * noise_metadata_schedule_126_e1492);
            let noise_metadata_schedule_126_e1495: f64 = (-params.p120);
            let noise_metadata_schedule_126_e1498: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_126_e1499: f64 = (noise_metadata_schedule_126_e1495 * noise_metadata_schedule_126_e1498);
            let noise_metadata_schedule_126_e1502: f64 = (noise_variable_73 * params.p33);
            let noise_metadata_schedule_126_e1503: f64 = (noise_metadata_schedule_126_e1499 / noise_metadata_schedule_126_e1502);
            let noise_metadata_schedule_126_e1504: f64 = (noise_metadata_schedule_126_e1503).exp();
            let noise_metadata_schedule_126_e1505: f64 = (noise_metadata_schedule_126_e1493 * noise_metadata_schedule_126_e1504);
            noise_variable_5 = noise_metadata_schedule_126_e1505;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_127_e1510: f64 = (params.p123 / params.p56);
            let noise_metadata_schedule_127_e1511: f64 = (noise_variable_41).powf(noise_metadata_schedule_127_e1510);
            let noise_metadata_schedule_127_e1512: f64 = (params.p54 * noise_metadata_schedule_127_e1511);
            let noise_metadata_schedule_127_e1514: f64 = (-params.p114);
            let noise_metadata_schedule_127_e1517: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_127_e1518: f64 = (noise_metadata_schedule_127_e1514 * noise_metadata_schedule_127_e1517);
            let noise_metadata_schedule_127_e1521: f64 = (noise_variable_73 * params.p56);
            let noise_metadata_schedule_127_e1522: f64 = (noise_metadata_schedule_127_e1518 / noise_metadata_schedule_127_e1521);
            let noise_metadata_schedule_127_e1523: f64 = (noise_metadata_schedule_127_e1522).exp();
            let noise_metadata_schedule_127_e1524: f64 = (noise_metadata_schedule_127_e1512 * noise_metadata_schedule_127_e1523);
            noise_variable_3 = noise_metadata_schedule_127_e1524;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_128_e1529: f64 = (params.p124 / params.p59);
            let noise_metadata_schedule_128_e1530: f64 = (noise_variable_41).powf(noise_metadata_schedule_128_e1529);
            let noise_metadata_schedule_128_e1531: f64 = (params.p58 * noise_metadata_schedule_128_e1530);
            let noise_metadata_schedule_128_e1533: f64 = (-params.p117);
            let noise_metadata_schedule_128_e1536: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_128_e1537: f64 = (noise_metadata_schedule_128_e1533 * noise_metadata_schedule_128_e1536);
            let noise_metadata_schedule_128_e1540: f64 = (noise_variable_73 * params.p59);
            let noise_metadata_schedule_128_e1541: f64 = (noise_metadata_schedule_128_e1537 / noise_metadata_schedule_128_e1540);
            let noise_metadata_schedule_128_e1542: f64 = (noise_metadata_schedule_128_e1541).exp();
            let noise_metadata_schedule_128_e1543: f64 = (noise_metadata_schedule_128_e1531 * noise_metadata_schedule_128_e1542);
            noise_variable_6 = noise_metadata_schedule_128_e1543;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_131_e1586: f64 = (params.p123 / params.p61);
            let noise_metadata_schedule_131_e1587: f64 = (noise_variable_41).powf(noise_metadata_schedule_131_e1586);
            let noise_metadata_schedule_131_e1588: f64 = (params.p64 * noise_metadata_schedule_131_e1587);
            let noise_metadata_schedule_131_e1590: f64 = (-params.p115);
            let noise_metadata_schedule_131_e1593: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_131_e1594: f64 = (noise_metadata_schedule_131_e1590 * noise_metadata_schedule_131_e1593);
            let noise_metadata_schedule_131_e1597: f64 = (noise_variable_73 * params.p61);
            let noise_metadata_schedule_131_e1598: f64 = (noise_metadata_schedule_131_e1594 / noise_metadata_schedule_131_e1597);
            let noise_metadata_schedule_131_e1599: f64 = (noise_metadata_schedule_131_e1598).exp();
            let noise_metadata_schedule_131_e1600: f64 = (noise_metadata_schedule_131_e1588 * noise_metadata_schedule_131_e1599);
            noise_variable_8 = noise_metadata_schedule_131_e1600;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_132_e1605: f64 = (params.p124 / params.p63);
            let noise_metadata_schedule_132_e1606: f64 = (noise_variable_41).powf(noise_metadata_schedule_132_e1605);
            let noise_metadata_schedule_132_e1607: f64 = (params.p65 * noise_metadata_schedule_132_e1606);
            let noise_metadata_schedule_132_e1609: f64 = (-params.p118);
            let noise_metadata_schedule_132_e1612: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_132_e1613: f64 = (noise_metadata_schedule_132_e1609 * noise_metadata_schedule_132_e1612);
            let noise_metadata_schedule_132_e1616: f64 = (noise_variable_73 * params.p63);
            let noise_metadata_schedule_132_e1617: f64 = (noise_metadata_schedule_132_e1613 / noise_metadata_schedule_132_e1616);
            let noise_metadata_schedule_132_e1618: f64 = (noise_metadata_schedule_132_e1617).exp();
            let noise_metadata_schedule_132_e1619: f64 = (noise_metadata_schedule_132_e1607 * noise_metadata_schedule_132_e1618);
            noise_variable_9 = noise_metadata_schedule_132_e1619;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_135_e1662: f64 = (noise_variable_42 * params.p129);
            let noise_metadata_schedule_135_e1663: f64 = (1.0 + noise_metadata_schedule_135_e1662);
            let noise_metadata_schedule_135_e1664: f64 = (params.p28 * noise_metadata_schedule_135_e1663);
            noise_variable_27 = noise_metadata_schedule_135_e1664;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_136_e1669: f64 = (noise_variable_42 * params.p129);
            let noise_metadata_schedule_136_e1670: f64 = (1.0 + noise_metadata_schedule_136_e1669);
            let noise_metadata_schedule_136_e1671: f64 = (params.p29 * noise_metadata_schedule_136_e1670);
            noise_variable_28 = noise_metadata_schedule_136_e1671;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_139_e1692: f64 = (noise_variable_42 * params.p92);
            let noise_metadata_schedule_139_e1693: f64 = (params.p91 + noise_metadata_schedule_139_e1692);
            let noise_metadata_schedule_139_e1694: f64 = (noise_variable_42 * noise_metadata_schedule_139_e1693);
            let noise_metadata_schedule_139_e1695: f64 = (1.0 + noise_metadata_schedule_139_e1694);
            let noise_metadata_schedule_139_e1696: f64 = (params.p88 * noise_metadata_schedule_139_e1695);
            noise_variable_31 = noise_metadata_schedule_139_e1696;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_140_e1701: f64 = (noise_variable_42 * params.p93);
            let noise_metadata_schedule_140_e1702: f64 = (1.0 + noise_metadata_schedule_140_e1701);
            let noise_metadata_schedule_140_e1703: f64 = (params.p89 * noise_metadata_schedule_140_e1702);
            noise_variable_32 = noise_metadata_schedule_140_e1703;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_141_e1707: f64 = (noise_variable_73 / noise_variable_41);
            let noise_metadata_schedule_141_e1708: f64 = (2.0 * noise_metadata_schedule_141_e1707);
            let noise_metadata_schedule_141_e1711: f64 = (0.5 * params.p37);
            let noise_metadata_schedule_141_e1713: f64 = (noise_metadata_schedule_141_e1711 * noise_variable_41);
            let noise_metadata_schedule_141_e1715: f64 = (noise_metadata_schedule_141_e1713 / noise_variable_73);
            let noise_metadata_schedule_141_e1716: f64 = (noise_metadata_schedule_141_e1715).exp();
            let noise_metadata_schedule_141_e1718: f64 = (-0.5);
            let noise_metadata_schedule_141_e1720: f64 = (noise_metadata_schedule_141_e1718 * params.p37);
            let noise_metadata_schedule_141_e1722: f64 = (noise_metadata_schedule_141_e1720 * noise_variable_41);
            let noise_metadata_schedule_141_e1724: f64 = (noise_metadata_schedule_141_e1722 / noise_variable_73);
            let noise_metadata_schedule_141_e1725: f64 = (noise_metadata_schedule_141_e1724).exp();
            let noise_metadata_schedule_141_e1726: f64 = (noise_metadata_schedule_141_e1716 - noise_metadata_schedule_141_e1725);
            let noise_metadata_schedule_141_e1727: f64 = (noise_metadata_schedule_141_e1726).ln();
            let noise_metadata_schedule_141_e1728: f64 = (noise_metadata_schedule_141_e1708 * noise_metadata_schedule_141_e1727);
            noise_variable_206 = noise_metadata_schedule_141_e1728;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_142_e1731: f64 = (noise_variable_206 * noise_variable_41);
            let noise_metadata_schedule_142_e1734: f64 = (3.0 * noise_variable_73);
            let noise_metadata_schedule_142_e1736: f64 = (noise_variable_41).ln();
            let noise_metadata_schedule_142_e1737: f64 = (noise_metadata_schedule_142_e1734 * noise_metadata_schedule_142_e1736);
            let noise_metadata_schedule_142_e1738: f64 = (noise_metadata_schedule_142_e1731 - noise_metadata_schedule_142_e1737);
            let noise_metadata_schedule_142_e1742: f64 = (noise_variable_41 - 1.0);
            let noise_metadata_schedule_142_e1743: f64 = (params.p114 * noise_metadata_schedule_142_e1742);
            let noise_metadata_schedule_142_e1744: f64 = (noise_metadata_schedule_142_e1738 - noise_metadata_schedule_142_e1743);
            noise_variable_207 = noise_metadata_schedule_142_e1744;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_143_e1748: f64 = (2.0 * noise_variable_73);
            let noise_metadata_schedule_143_e1754: f64 = (-noise_variable_207);
            let noise_metadata_schedule_143_e1756: f64 = (noise_metadata_schedule_143_e1754 / noise_variable_73);
            let noise_metadata_schedule_143_e1757: f64 = (noise_metadata_schedule_143_e1756).exp();
            let noise_metadata_schedule_143_e1758: f64 = (4.0 * noise_metadata_schedule_143_e1757);
            let noise_metadata_schedule_143_e1759: f64 = (1.0 + noise_metadata_schedule_143_e1758);
            let noise_metadata_schedule_143_e1760: f64 = (noise_metadata_schedule_143_e1759).sqrt();
            let noise_metadata_schedule_143_e1761: f64 = (1.0 + noise_metadata_schedule_143_e1760);
            let noise_metadata_schedule_143_e1762: f64 = (0.5 * noise_metadata_schedule_143_e1761);
            let noise_metadata_schedule_143_e1763: f64 = (noise_metadata_schedule_143_e1762).ln();
            let noise_metadata_schedule_143_e1764: f64 = (noise_metadata_schedule_143_e1748 * noise_metadata_schedule_143_e1763);
            let noise_metadata_schedule_143_e1765: f64 = (noise_variable_207 + noise_metadata_schedule_143_e1764);
            noise_variable_20 = noise_metadata_schedule_143_e1765;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_144_e1769: f64 = (noise_variable_73 / noise_variable_41);
            let noise_metadata_schedule_144_e1770: f64 = (2.0 * noise_metadata_schedule_144_e1769);
            let noise_metadata_schedule_144_e1773: f64 = (0.5 * params.p42);
            let noise_metadata_schedule_144_e1775: f64 = (noise_metadata_schedule_144_e1773 * noise_variable_41);
            let noise_metadata_schedule_144_e1777: f64 = (noise_metadata_schedule_144_e1775 / noise_variable_73);
            let noise_metadata_schedule_144_e1778: f64 = (noise_metadata_schedule_144_e1777).exp();
            let noise_metadata_schedule_144_e1780: f64 = (-0.5);
            let noise_metadata_schedule_144_e1782: f64 = (noise_metadata_schedule_144_e1780 * params.p42);
            let noise_metadata_schedule_144_e1784: f64 = (noise_metadata_schedule_144_e1782 * noise_variable_41);
            let noise_metadata_schedule_144_e1786: f64 = (noise_metadata_schedule_144_e1784 / noise_variable_73);
            let noise_metadata_schedule_144_e1787: f64 = (noise_metadata_schedule_144_e1786).exp();
            let noise_metadata_schedule_144_e1788: f64 = (noise_metadata_schedule_144_e1778 - noise_metadata_schedule_144_e1787);
            let noise_metadata_schedule_144_e1789: f64 = (noise_metadata_schedule_144_e1788).ln();
            let noise_metadata_schedule_144_e1790: f64 = (noise_metadata_schedule_144_e1770 * noise_metadata_schedule_144_e1789);
            noise_variable_208 = noise_metadata_schedule_144_e1790;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_145_e1793: f64 = (noise_variable_208 * noise_variable_41);
            let noise_metadata_schedule_145_e1796: f64 = (3.0 * noise_variable_73);
            let noise_metadata_schedule_145_e1798: f64 = (noise_variable_41).ln();
            let noise_metadata_schedule_145_e1799: f64 = (noise_metadata_schedule_145_e1796 * noise_metadata_schedule_145_e1798);
            let noise_metadata_schedule_145_e1800: f64 = (noise_metadata_schedule_145_e1793 - noise_metadata_schedule_145_e1799);
            let noise_metadata_schedule_145_e1804: f64 = (noise_variable_41 - 1.0);
            let noise_metadata_schedule_145_e1805: f64 = (params.p115 * noise_metadata_schedule_145_e1804);
            let noise_metadata_schedule_145_e1806: f64 = (noise_metadata_schedule_145_e1800 - noise_metadata_schedule_145_e1805);
            noise_variable_209 = noise_metadata_schedule_145_e1806;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_146_e1810: f64 = (2.0 * noise_variable_73);
            let noise_metadata_schedule_146_e1816: f64 = (-noise_variable_209);
            let noise_metadata_schedule_146_e1818: f64 = (noise_metadata_schedule_146_e1816 / noise_variable_73);
            let noise_metadata_schedule_146_e1819: f64 = (noise_metadata_schedule_146_e1818).exp();
            let noise_metadata_schedule_146_e1820: f64 = (4.0 * noise_metadata_schedule_146_e1819);
            let noise_metadata_schedule_146_e1821: f64 = (1.0 + noise_metadata_schedule_146_e1820);
            let noise_metadata_schedule_146_e1822: f64 = (noise_metadata_schedule_146_e1821).sqrt();
            let noise_metadata_schedule_146_e1823: f64 = (1.0 + noise_metadata_schedule_146_e1822);
            let noise_metadata_schedule_146_e1824: f64 = (0.5 * noise_metadata_schedule_146_e1823);
            let noise_metadata_schedule_146_e1825: f64 = (noise_metadata_schedule_146_e1824).ln();
            let noise_metadata_schedule_146_e1826: f64 = (noise_metadata_schedule_146_e1810 * noise_metadata_schedule_146_e1825);
            let noise_metadata_schedule_146_e1827: f64 = (noise_variable_209 + noise_metadata_schedule_146_e1826);
            noise_variable_21 = noise_metadata_schedule_146_e1827;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_154_e1921: f64 = (noise_variable_41).powf(params.p122);
            let noise_metadata_schedule_154_e1922: f64 = (params.p19 * noise_metadata_schedule_154_e1921);
            let noise_metadata_schedule_154_e1924: f64 = (-params.p113);
            let noise_metadata_schedule_154_e1927: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_154_e1928: f64 = (noise_metadata_schedule_154_e1924 * noise_metadata_schedule_154_e1927);
            let noise_metadata_schedule_154_e1930: f64 = (noise_metadata_schedule_154_e1928 / noise_variable_73);
            let noise_metadata_schedule_154_e1931: f64 = (noise_metadata_schedule_154_e1930).exp();
            let noise_metadata_schedule_154_e1932: f64 = (noise_metadata_schedule_154_e1922 * noise_metadata_schedule_154_e1931);
            noise_variable_33 = noise_metadata_schedule_154_e1932;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_155_e1936: f64 = (noise_variable_41).powf(params.p112);
            let noise_metadata_schedule_155_e1937: f64 = (params.p18 * noise_metadata_schedule_155_e1936);
            noise_variable_34 = noise_metadata_schedule_155_e1937;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_156_e1939: f64 = (-noise_variable_31);
            let noise_metadata_schedule_156_e1942: f64 = (noise_variable_32 * noise_variable_73);
            let noise_metadata_schedule_156_e1943: f64 = (noise_metadata_schedule_156_e1939 / noise_metadata_schedule_156_e1942);
            let noise_metadata_schedule_156_e1944: f64 = (noise_metadata_schedule_156_e1943).exp();
            noise_variable_35 = noise_metadata_schedule_156_e1944;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_157_e1949: f64 = (noise_variable_42 * params.p130);
            let noise_metadata_schedule_157_e1950: f64 = (1.0 + noise_metadata_schedule_157_e1949);
            let noise_metadata_schedule_157_e1951: f64 = (params.p70 * noise_metadata_schedule_157_e1950);
            noise_variable_36 = noise_metadata_schedule_157_e1951;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_158_e1956: f64 = (noise_variable_42 * params.p131);
            let noise_metadata_schedule_158_e1957: f64 = (1.0 + noise_metadata_schedule_158_e1956);
            let noise_metadata_schedule_158_e1958: f64 = (params.p71 * noise_metadata_schedule_158_e1957);
            noise_variable_37 = noise_metadata_schedule_158_e1958;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_159_e1966,) = {
    if (noise_variable_12 > 0.001) {
        let noise_metadata_schedule_159_e1964: f64 = (1.0 / noise_variable_12);
        (noise_metadata_schedule_159_e1964,)
    } else {
        (1000.0,)
    }
};
            noise_variable_53 = noise_metadata_schedule_159_e1966;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_160_e1974,) = {
    if (noise_variable_13 > 0.001) {
        let noise_metadata_schedule_160_e1972: f64 = (1.0 / noise_variable_13);
        (noise_metadata_schedule_160_e1972,)
    } else {
        (1000.0,)
    }
};
            noise_variable_54 = noise_metadata_schedule_160_e1974;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_161_e1982,) = {
    if (noise_variable_14 > 0.001) {
        let noise_metadata_schedule_161_e1980: f64 = (1.0 / noise_variable_14);
        (noise_metadata_schedule_161_e1980,)
    } else {
        (1000.0,)
    }
};
            noise_variable_55 = noise_metadata_schedule_161_e1982;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_162_e1990,) = {
    if (noise_variable_15 > 0.001) {
        let noise_metadata_schedule_162_e1988: f64 = (1.0 / noise_variable_15);
        (noise_metadata_schedule_162_e1988,)
    } else {
        (1000.0,)
    }
};
            noise_variable_56 = noise_metadata_schedule_162_e1990;
        }
        if matches!(source_index, 11) {
            let (noise_metadata_schedule_163_e1998,) = {
    if (noise_variable_16 > 0.001) {
        let noise_metadata_schedule_163_e1996: f64 = (1.0 / noise_variable_16);
        (noise_metadata_schedule_163_e1996,)
    } else {
        (1000.0,)
    }
};
            noise_variable_57 = noise_metadata_schedule_163_e1998;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_164_e2006,) = {
    if (noise_variable_18 > 0.001) {
        let noise_metadata_schedule_164_e2004: f64 = (1.0 / noise_variable_18);
        (noise_metadata_schedule_164_e2004,)
    } else {
        (1000.0,)
    }
};
            noise_variable_58 = noise_metadata_schedule_164_e2006;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_165_e2014,) = {
    if (noise_variable_17 > 0.001) {
        let noise_metadata_schedule_165_e2012: f64 = (1.0 / noise_variable_17);
        (noise_metadata_schedule_165_e2012,)
    } else {
        (1000.0,)
    }
};
            noise_variable_59 = noise_metadata_schedule_165_e2014;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_167_e2030,) = {
    if (noise_variable_36 > 0.0) {
        let noise_metadata_schedule_167_e2028: f64 = (1.0 / noise_variable_36);
        (noise_metadata_schedule_167_e2028,)
    } else {
        (0.0,)
    }
};
            noise_variable_43 = noise_metadata_schedule_167_e2030;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_168_e2038,) = {
    if (noise_variable_37 > 0.0) {
        let noise_metadata_schedule_168_e2036: f64 = (1.0 / noise_variable_37);
        (noise_metadata_schedule_168_e2036,)
    } else {
        (0.0,)
    }
};
            noise_variable_44 = noise_metadata_schedule_168_e2038;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_169_e2046,) = {
    if (noise_variable_2 > 0.0) {
        let noise_metadata_schedule_169_e2044: f64 = (1.0 / noise_variable_2);
        (noise_metadata_schedule_169_e2044,)
    } else {
        (0.0,)
    }
};
            noise_variable_45 = noise_metadata_schedule_169_e2046;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_170_e2054,) = {
    if (noise_variable_34 > 0.0) {
        let noise_metadata_schedule_170_e2052: f64 = (1.0 / noise_variable_34);
        (noise_metadata_schedule_170_e2052,)
    } else {
        (0.0,)
    }
};
            noise_variable_48 = noise_metadata_schedule_170_e2054;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_171_e2057: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
            noise_variable_143 = noise_metadata_schedule_171_e2057;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_172_e2060: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[9])));
            noise_variable_145 = noise_metadata_schedule_172_e2060;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_173_e2063: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_144 = noise_metadata_schedule_173_e2063;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_174_e2066: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_148 = noise_metadata_schedule_174_e2066;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let noise_metadata_schedule_176_e2072: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[10])));
            noise_variable_146 = noise_metadata_schedule_176_e2072;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_181_e2081: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_154 = noise_metadata_schedule_181_e2081;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 13) {
            let noise_metadata_schedule_186_e2088: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
            noise_variable_147 = noise_metadata_schedule_186_e2088;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_191_e2096: f64 = (-noise_variable_20);
            let noise_metadata_schedule_191_e2098: f64 = (noise_metadata_schedule_191_e2096 * params.p34);
            noise_variable_212 = noise_metadata_schedule_191_e2098;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_192_e2101: f64 = if params.p39 <= 0.0 { 1.0 } else { 0.0 };
            noise_variable_223 = noise_metadata_schedule_192_e2101;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_193_e2107,) = {
    if (noise_variable_223 != 0.0) {
        let noise_metadata_schedule_193_e2105: f64 = (noise_variable_143 + noise_variable_212);
        (noise_metadata_schedule_193_e2105,)
    } else {
        (noise_variable_213,)
    }
};
            noise_variable_213 = noise_metadata_schedule_193_e2107;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_194_e2110: f64 = if noise_variable_213 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_224 = noise_metadata_schedule_194_e2110;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_195_e2121,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_195_e2116: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_195_e2118: f64 = (-params.p38);
        let noise_metadata_schedule_195_e2119: f64 = (noise_metadata_schedule_195_e2116).powf(noise_metadata_schedule_195_e2118);
        (noise_metadata_schedule_195_e2119,)
    } else {
        (noise_variable_214,)
    }
};
            noise_variable_214 = noise_metadata_schedule_195_e2121;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_196_e2139,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_196_e2130: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_196_e2131: f64 = (noise_variable_214 * noise_metadata_schedule_196_e2130);
        let noise_metadata_schedule_196_e2132: f64 = (1.0 - noise_metadata_schedule_196_e2131);
        let noise_metadata_schedule_196_e2133: f64 = (noise_variable_20 * noise_metadata_schedule_196_e2132);
        let noise_metadata_schedule_196_e2136: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_196_e2137: f64 = (noise_metadata_schedule_196_e2133 / noise_metadata_schedule_196_e2136);
        (noise_metadata_schedule_196_e2137,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_196_e2139;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_197_e2161,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_197_e2147: f64 = (0.5 * params.p38);
        let noise_metadata_schedule_197_e2149: f64 = (noise_metadata_schedule_197_e2147 * noise_variable_213);
        let noise_metadata_schedule_197_e2153: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_197_e2154: f64 = (noise_variable_20 * noise_metadata_schedule_197_e2153);
        let noise_metadata_schedule_197_e2155: f64 = (noise_metadata_schedule_197_e2149 / noise_metadata_schedule_197_e2154);
        let noise_metadata_schedule_197_e2156: f64 = (1.0 + noise_metadata_schedule_197_e2155);
        let noise_metadata_schedule_197_e2157: f64 = (noise_variable_213 * noise_metadata_schedule_197_e2156);
        let noise_metadata_schedule_197_e2159: f64 = (noise_metadata_schedule_197_e2157 * noise_variable_214);
        (noise_metadata_schedule_197_e2159,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_197_e2161;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_198_e2184,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 == 0.0)) {
        let noise_metadata_schedule_198_e2171: f64 = (noise_variable_143 / noise_variable_20);
        let noise_metadata_schedule_198_e2172: f64 = (1.0 - noise_metadata_schedule_198_e2171);
        let noise_metadata_schedule_198_e2175: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_198_e2176: f64 = (noise_metadata_schedule_198_e2172).powf(noise_metadata_schedule_198_e2175);
        let noise_metadata_schedule_198_e2177: f64 = (1.0 - noise_metadata_schedule_198_e2176);
        let noise_metadata_schedule_198_e2178: f64 = (noise_variable_20 * noise_metadata_schedule_198_e2177);
        let noise_metadata_schedule_198_e2181: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_198_e2182: f64 = (noise_metadata_schedule_198_e2178 / noise_metadata_schedule_198_e2181);
        (noise_metadata_schedule_198_e2182,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_198_e2184;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_199_e2191,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_199_e2191;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_200_e2197,) = {
    if (noise_variable_223 != 0.0) {
        let noise_metadata_schedule_200_e2195: f64 = (noise_variable_215 + noise_variable_216);
        (noise_metadata_schedule_200_e2195,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_200_e2197;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_201_e2211,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_201_e2202: f64 = (noise_variable_212 * noise_variable_212);
        let noise_metadata_schedule_201_e2205: f64 = (4.0 * params.p39);
        let noise_metadata_schedule_201_e2207: f64 = (noise_metadata_schedule_201_e2205 * params.p39);
        let noise_metadata_schedule_201_e2208: f64 = (noise_metadata_schedule_201_e2202 + noise_metadata_schedule_201_e2207);
        let noise_metadata_schedule_201_e2209: f64 = (noise_metadata_schedule_201_e2208).sqrt();
        (noise_metadata_schedule_201_e2209,)
    } else {
        (noise_variable_217,)
    }
};
            noise_variable_217 = noise_metadata_schedule_201_e2211;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_202_e2221,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_202_e2215: f64 = (-0.5);
        let noise_metadata_schedule_202_e2218: f64 = (noise_variable_212 + noise_variable_217);
        let noise_metadata_schedule_202_e2219: f64 = (noise_metadata_schedule_202_e2215 * noise_metadata_schedule_202_e2218);
        (noise_metadata_schedule_202_e2219,)
    } else {
        (noise_variable_218,)
    }
};
            noise_variable_218 = noise_metadata_schedule_202_e2221;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_203_e2241,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_203_e2225: f64 = (-noise_variable_20);
        let noise_metadata_schedule_203_e2229: f64 = (noise_variable_218 / noise_variable_20);
        let noise_metadata_schedule_203_e2230: f64 = (1.0 - noise_metadata_schedule_203_e2229);
        let noise_metadata_schedule_203_e2233: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_203_e2234: f64 = (noise_metadata_schedule_203_e2230).powf(noise_metadata_schedule_203_e2233);
        let noise_metadata_schedule_203_e2235: f64 = (noise_metadata_schedule_203_e2225 * noise_metadata_schedule_203_e2234);
        let noise_metadata_schedule_203_e2238: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_203_e2239: f64 = (noise_metadata_schedule_203_e2235 / noise_metadata_schedule_203_e2238);
        (noise_metadata_schedule_203_e2239,)
    } else {
        (noise_variable_219,)
    }
};
            noise_variable_219 = noise_metadata_schedule_203_e2241;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_204_e2248,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_204_e2246: f64 = (noise_variable_143 + noise_variable_212);
        (noise_metadata_schedule_204_e2246,)
    } else {
        (noise_variable_220,)
    }
};
            noise_variable_220 = noise_metadata_schedule_204_e2248;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_205_e2262,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_205_e2253: f64 = (noise_variable_220 * noise_variable_220);
        let noise_metadata_schedule_205_e2256: f64 = (4.0 * params.p39);
        let noise_metadata_schedule_205_e2258: f64 = (noise_metadata_schedule_205_e2256 * params.p39);
        let noise_metadata_schedule_205_e2259: f64 = (noise_metadata_schedule_205_e2253 + noise_metadata_schedule_205_e2258);
        let noise_metadata_schedule_205_e2260: f64 = (noise_metadata_schedule_205_e2259).sqrt();
        (noise_metadata_schedule_205_e2260,)
    } else {
        (noise_variable_221,)
    }
};
            noise_variable_221 = noise_metadata_schedule_205_e2262;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_206_e2273,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_206_e2268: f64 = (noise_variable_220 - noise_variable_221);
        let noise_metadata_schedule_206_e2269: f64 = (0.5 * noise_metadata_schedule_206_e2268);
        let noise_metadata_schedule_206_e2271: f64 = (noise_metadata_schedule_206_e2269 - noise_variable_212);
        (noise_metadata_schedule_206_e2271,)
    } else {
        (noise_variable_222,)
    }
};
            noise_variable_222 = noise_metadata_schedule_206_e2273;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_207_e2293,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_207_e2277: f64 = (-noise_variable_20);
        let noise_metadata_schedule_207_e2281: f64 = (noise_variable_222 / noise_variable_20);
        let noise_metadata_schedule_207_e2282: f64 = (1.0 - noise_metadata_schedule_207_e2281);
        let noise_metadata_schedule_207_e2285: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_207_e2286: f64 = (noise_metadata_schedule_207_e2282).powf(noise_metadata_schedule_207_e2285);
        let noise_metadata_schedule_207_e2287: f64 = (noise_metadata_schedule_207_e2277 * noise_metadata_schedule_207_e2286);
        let noise_metadata_schedule_207_e2290: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_207_e2291: f64 = (noise_metadata_schedule_207_e2287 / noise_metadata_schedule_207_e2290);
        (noise_metadata_schedule_207_e2291,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_207_e2293;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_208_e2331,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_208_e2299: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_208_e2301: f64 = (-params.p38);
        let noise_metadata_schedule_208_e2302: f64 = (noise_metadata_schedule_208_e2299).powf(noise_metadata_schedule_208_e2301);
        let noise_metadata_schedule_208_e2305: f64 = (noise_variable_143 - noise_variable_222);
        let noise_metadata_schedule_208_e2307: f64 = (noise_metadata_schedule_208_e2305 + noise_variable_218);
        let noise_metadata_schedule_208_e2308: f64 = (noise_metadata_schedule_208_e2302 * noise_metadata_schedule_208_e2307);
        let noise_metadata_schedule_208_e2312: f64 = (0.5 * params.p38);
        let noise_metadata_schedule_208_e2315: f64 = (noise_variable_143 - noise_variable_222);
        let noise_metadata_schedule_208_e2317: f64 = (noise_metadata_schedule_208_e2315 + noise_variable_218);
        let noise_metadata_schedule_208_e2318: f64 = (noise_metadata_schedule_208_e2312 * noise_metadata_schedule_208_e2317);
        let noise_metadata_schedule_208_e2322: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_208_e2323: f64 = (noise_variable_20 * noise_metadata_schedule_208_e2322);
        let noise_metadata_schedule_208_e2324: f64 = (noise_metadata_schedule_208_e2318 / noise_metadata_schedule_208_e2323);
        let noise_metadata_schedule_208_e2325: f64 = (1.0 + noise_metadata_schedule_208_e2324);
        let noise_metadata_schedule_208_e2326: f64 = (noise_metadata_schedule_208_e2308 * noise_metadata_schedule_208_e2325);
        let noise_metadata_schedule_208_e2327: f64 = (noise_variable_215 + noise_metadata_schedule_208_e2326);
        let noise_metadata_schedule_208_e2329: f64 = (noise_metadata_schedule_208_e2327 - noise_variable_219);
        (noise_metadata_schedule_208_e2329,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_208_e2331;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_209_e2333: f64 = (-noise_variable_21);
            let noise_metadata_schedule_209_e2335: f64 = (noise_metadata_schedule_209_e2333 * params.p34);
            noise_variable_225 = noise_metadata_schedule_209_e2335;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_210_e2338: f64 = if params.p44 <= 0.0 { 1.0 } else { 0.0 };
            noise_variable_246 = noise_metadata_schedule_210_e2338;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_211_e2344,) = {
    if (noise_variable_246 != 0.0) {
        let noise_metadata_schedule_211_e2342: f64 = (noise_variable_144 + noise_variable_225);
        (noise_metadata_schedule_211_e2342,)
    } else {
        (noise_variable_226,)
    }
};
            noise_variable_226 = noise_metadata_schedule_211_e2344;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_212_e2347: f64 = if noise_variable_226 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_247 = noise_metadata_schedule_212_e2347;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_213_e2360,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_213_e2353: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_213_e2355: f64 = (-1.0);
        let noise_metadata_schedule_213_e2357: f64 = (noise_metadata_schedule_213_e2355 - params.p43);
        let noise_metadata_schedule_213_e2358: f64 = (noise_metadata_schedule_213_e2353).powf(noise_metadata_schedule_213_e2357);
        (noise_metadata_schedule_213_e2358,)
    } else {
        (noise_variable_227,)
    }
};
            noise_variable_227 = noise_metadata_schedule_213_e2360;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_214_e2382,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_214_e2369: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_214_e2370: f64 = (noise_variable_227 * noise_metadata_schedule_214_e2369);
        let noise_metadata_schedule_214_e2373: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_214_e2374: f64 = (noise_metadata_schedule_214_e2370 * noise_metadata_schedule_214_e2373);
        let noise_metadata_schedule_214_e2375: f64 = (1.0 - noise_metadata_schedule_214_e2374);
        let noise_metadata_schedule_214_e2376: f64 = (noise_variable_21 * noise_metadata_schedule_214_e2375);
        let noise_metadata_schedule_214_e2379: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_214_e2380: f64 = (noise_metadata_schedule_214_e2376 / noise_metadata_schedule_214_e2379);
        (noise_metadata_schedule_214_e2380,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_214_e2382;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_215_e2402,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_215_e2389: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_215_e2392: f64 = (0.5 * params.p43);
        let noise_metadata_schedule_215_e2394: f64 = (noise_metadata_schedule_215_e2392 * noise_variable_226);
        let noise_metadata_schedule_215_e2396: f64 = (noise_metadata_schedule_215_e2394 / noise_variable_21);
        let noise_metadata_schedule_215_e2397: f64 = (noise_metadata_schedule_215_e2389 + noise_metadata_schedule_215_e2396);
        let noise_metadata_schedule_215_e2398: f64 = (noise_variable_226 * noise_metadata_schedule_215_e2397);
        let noise_metadata_schedule_215_e2400: f64 = (noise_metadata_schedule_215_e2398 * noise_variable_227);
        (noise_metadata_schedule_215_e2400,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_215_e2402;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_216_e2408: f64 = (-params.p45);
            let noise_metadata_schedule_216_e2410: f64 = if ((params.p45 > 0.0) && (noise_variable_144 < noise_metadata_schedule_216_e2408)) { 1.0 } else { 0.0 };
            noise_variable_248 = noise_metadata_schedule_216_e2410;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_217_e2449,) = {
    if (((noise_variable_246 != 0.0) && (noise_variable_247 == 0.0)) && (noise_variable_248 != 0.0)) {
        let noise_metadata_schedule_217_e2422: f64 = (params.p45 / noise_variable_21);
        let noise_metadata_schedule_217_e2423: f64 = (1.0 + noise_metadata_schedule_217_e2422);
        let noise_metadata_schedule_217_e2426: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_217_e2427: f64 = (noise_metadata_schedule_217_e2423).powf(noise_metadata_schedule_217_e2426);
        let noise_metadata_schedule_217_e2431: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_217_e2434: f64 = (noise_variable_144 + params.p45);
        let noise_metadata_schedule_217_e2435: f64 = (noise_metadata_schedule_217_e2431 * noise_metadata_schedule_217_e2434);
        let noise_metadata_schedule_217_e2438: f64 = (noise_variable_21 + params.p45);
        let noise_metadata_schedule_217_e2439: f64 = (noise_metadata_schedule_217_e2435 / noise_metadata_schedule_217_e2438);
        let noise_metadata_schedule_217_e2440: f64 = (1.0 - noise_metadata_schedule_217_e2439);
        let noise_metadata_schedule_217_e2441: f64 = (noise_metadata_schedule_217_e2427 * noise_metadata_schedule_217_e2440);
        let noise_metadata_schedule_217_e2442: f64 = (1.0 - noise_metadata_schedule_217_e2441);
        let noise_metadata_schedule_217_e2443: f64 = (noise_variable_21 * noise_metadata_schedule_217_e2442);
        let noise_metadata_schedule_217_e2446: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_217_e2447: f64 = (noise_metadata_schedule_217_e2443 / noise_metadata_schedule_217_e2446);
        (noise_metadata_schedule_217_e2447,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_217_e2449;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_218_e2475,) = {
    if (((noise_variable_246 != 0.0) && (noise_variable_247 == 0.0)) && (noise_variable_248 == 0.0)) {
        let noise_metadata_schedule_218_e2462: f64 = (noise_variable_144 / noise_variable_21);
        let noise_metadata_schedule_218_e2463: f64 = (1.0 - noise_metadata_schedule_218_e2462);
        let noise_metadata_schedule_218_e2466: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_218_e2467: f64 = (noise_metadata_schedule_218_e2463).powf(noise_metadata_schedule_218_e2466);
        let noise_metadata_schedule_218_e2468: f64 = (1.0 - noise_metadata_schedule_218_e2467);
        let noise_metadata_schedule_218_e2469: f64 = (noise_variable_21 * noise_metadata_schedule_218_e2468);
        let noise_metadata_schedule_218_e2472: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_218_e2473: f64 = (noise_metadata_schedule_218_e2469 / noise_metadata_schedule_218_e2472);
        (noise_metadata_schedule_218_e2473,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_218_e2475;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_219_e2482,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_219_e2482;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_220_e2488,) = {
    if (noise_variable_246 != 0.0) {
        let noise_metadata_schedule_220_e2486: f64 = (noise_variable_228 + noise_variable_229);
        (noise_metadata_schedule_220_e2486,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_220_e2488;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_221_e2495: f64 = if ((params.p45 > 0.0) && (params.p46 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_249 = noise_metadata_schedule_221_e2495;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_222_e2508,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_222_e2502: f64 = (params.p45 + noise_variable_225);
        let noise_metadata_schedule_222_e2505: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_222_e2506: f64 = (noise_metadata_schedule_222_e2502 / noise_metadata_schedule_222_e2505);
        (noise_metadata_schedule_222_e2506,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_222_e2508;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_223_e2547,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_223_e2515: f64 = (2.0 * noise_variable_230);
        let noise_metadata_schedule_223_e2518: f64 = (noise_variable_230 - 1.0);
        let noise_metadata_schedule_223_e2521: f64 = (noise_variable_230 - 1.0);
        let noise_metadata_schedule_223_e2522: f64 = (noise_metadata_schedule_223_e2518 * noise_metadata_schedule_223_e2521);
        let noise_metadata_schedule_223_e2525: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_223_e2527: f64 = (noise_metadata_schedule_223_e2525 * params.p44);
        let noise_metadata_schedule_223_e2528: f64 = (noise_metadata_schedule_223_e2522 + noise_metadata_schedule_223_e2527);
        let noise_metadata_schedule_223_e2529: f64 = (noise_metadata_schedule_223_e2528).sqrt();
        let noise_metadata_schedule_223_e2532: f64 = (noise_variable_230 + 1.0);
        let noise_metadata_schedule_223_e2535: f64 = (noise_variable_230 + 1.0);
        let noise_metadata_schedule_223_e2536: f64 = (noise_metadata_schedule_223_e2532 * noise_metadata_schedule_223_e2535);
        let noise_metadata_schedule_223_e2539: f64 = (4.0 * params.p46);
        let noise_metadata_schedule_223_e2541: f64 = (noise_metadata_schedule_223_e2539 * params.p46);
        let noise_metadata_schedule_223_e2542: f64 = (noise_metadata_schedule_223_e2536 + noise_metadata_schedule_223_e2541);
        let noise_metadata_schedule_223_e2543: f64 = (noise_metadata_schedule_223_e2542).sqrt();
        let noise_metadata_schedule_223_e2544: f64 = (noise_metadata_schedule_223_e2529 + noise_metadata_schedule_223_e2543);
        let noise_metadata_schedule_223_e2545: f64 = (noise_metadata_schedule_223_e2515 / noise_metadata_schedule_223_e2544);
        (noise_metadata_schedule_223_e2545,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_223_e2547;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_224_e2564,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_224_e2556: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_224_e2557: f64 = (noise_variable_231 * noise_metadata_schedule_224_e2556);
        let noise_metadata_schedule_224_e2559: f64 = (noise_metadata_schedule_224_e2557 - params.p45);
        let noise_metadata_schedule_224_e2561: f64 = (noise_metadata_schedule_224_e2559 - noise_variable_225);
        let noise_metadata_schedule_224_e2562: f64 = (0.5 * noise_metadata_schedule_224_e2561);
        (noise_metadata_schedule_224_e2562,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_224_e2564;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_225_e2587,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_225_e2574: f64 = (noise_variable_232 / noise_variable_21);
        let noise_metadata_schedule_225_e2575: f64 = (1.0 - noise_metadata_schedule_225_e2574);
        let noise_metadata_schedule_225_e2578: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_225_e2579: f64 = (noise_metadata_schedule_225_e2575).powf(noise_metadata_schedule_225_e2578);
        let noise_metadata_schedule_225_e2580: f64 = (1.0 - noise_metadata_schedule_225_e2579);
        let noise_metadata_schedule_225_e2581: f64 = (noise_variable_21 * noise_metadata_schedule_225_e2580);
        let noise_metadata_schedule_225_e2584: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_225_e2585: f64 = (noise_metadata_schedule_225_e2581 / noise_metadata_schedule_225_e2584);
        (noise_metadata_schedule_225_e2585,)
    } else {
        (noise_variable_233,)
    }
};
            noise_variable_233 = noise_metadata_schedule_225_e2587;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_226_e2604,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_226_e2594: f64 = (2.0 * noise_variable_144);
        let noise_metadata_schedule_226_e2596: f64 = (noise_metadata_schedule_226_e2594 + params.p45);
        let noise_metadata_schedule_226_e2598: f64 = (noise_metadata_schedule_226_e2596 + noise_variable_225);
        let noise_metadata_schedule_226_e2601: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_226_e2602: f64 = (noise_metadata_schedule_226_e2598 / noise_metadata_schedule_226_e2601);
        (noise_metadata_schedule_226_e2602,)
    } else {
        (noise_variable_234,)
    }
};
            noise_variable_234 = noise_metadata_schedule_226_e2604;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_227_e2643,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_227_e2611: f64 = (2.0 * noise_variable_234);
        let noise_metadata_schedule_227_e2614: f64 = (noise_variable_234 - 1.0);
        let noise_metadata_schedule_227_e2617: f64 = (noise_variable_234 - 1.0);
        let noise_metadata_schedule_227_e2618: f64 = (noise_metadata_schedule_227_e2614 * noise_metadata_schedule_227_e2617);
        let noise_metadata_schedule_227_e2621: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_227_e2623: f64 = (noise_metadata_schedule_227_e2621 * params.p44);
        let noise_metadata_schedule_227_e2624: f64 = (noise_metadata_schedule_227_e2618 + noise_metadata_schedule_227_e2623);
        let noise_metadata_schedule_227_e2625: f64 = (noise_metadata_schedule_227_e2624).sqrt();
        let noise_metadata_schedule_227_e2628: f64 = (noise_variable_234 + 1.0);
        let noise_metadata_schedule_227_e2631: f64 = (noise_variable_234 + 1.0);
        let noise_metadata_schedule_227_e2632: f64 = (noise_metadata_schedule_227_e2628 * noise_metadata_schedule_227_e2631);
        let noise_metadata_schedule_227_e2635: f64 = (4.0 * params.p46);
        let noise_metadata_schedule_227_e2637: f64 = (noise_metadata_schedule_227_e2635 * params.p46);
        let noise_metadata_schedule_227_e2638: f64 = (noise_metadata_schedule_227_e2632 + noise_metadata_schedule_227_e2637);
        let noise_metadata_schedule_227_e2639: f64 = (noise_metadata_schedule_227_e2638).sqrt();
        let noise_metadata_schedule_227_e2640: f64 = (noise_metadata_schedule_227_e2625 + noise_metadata_schedule_227_e2639);
        let noise_metadata_schedule_227_e2641: f64 = (noise_metadata_schedule_227_e2611 / noise_metadata_schedule_227_e2640);
        (noise_metadata_schedule_227_e2641,)
    } else {
        (noise_variable_235,)
    }
};
            noise_variable_235 = noise_metadata_schedule_227_e2643;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_228_e2660,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_228_e2652: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_228_e2653: f64 = (noise_variable_235 * noise_metadata_schedule_228_e2652);
        let noise_metadata_schedule_228_e2655: f64 = (noise_metadata_schedule_228_e2653 - params.p45);
        let noise_metadata_schedule_228_e2657: f64 = (noise_metadata_schedule_228_e2655 - noise_variable_225);
        let noise_metadata_schedule_228_e2658: f64 = (0.5 * noise_metadata_schedule_228_e2657);
        (noise_metadata_schedule_228_e2658,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_228_e2660;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_229_e2683,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_229_e2670: f64 = (noise_variable_236 / noise_variable_21);
        let noise_metadata_schedule_229_e2671: f64 = (1.0 - noise_metadata_schedule_229_e2670);
        let noise_metadata_schedule_229_e2674: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_229_e2675: f64 = (noise_metadata_schedule_229_e2671).powf(noise_metadata_schedule_229_e2674);
        let noise_metadata_schedule_229_e2676: f64 = (1.0 - noise_metadata_schedule_229_e2675);
        let noise_metadata_schedule_229_e2677: f64 = (noise_variable_21 * noise_metadata_schedule_229_e2676);
        let noise_metadata_schedule_229_e2680: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_229_e2681: f64 = (noise_metadata_schedule_229_e2677 / noise_metadata_schedule_229_e2680);
        (noise_metadata_schedule_229_e2681,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_229_e2683;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_230_e2694,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_230_e2691: f64 = (noise_variable_235 + 1.0);
        let noise_metadata_schedule_230_e2692: f64 = (0.5 * noise_metadata_schedule_230_e2691);
        (noise_metadata_schedule_230_e2692,)
    } else {
        (noise_variable_237,)
    }
};
            noise_variable_237 = noise_metadata_schedule_230_e2694;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_231_e2708,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_231_e2702: f64 = (params.p45 / noise_variable_21);
        let noise_metadata_schedule_231_e2703: f64 = (1.0 + noise_metadata_schedule_231_e2702);
        let noise_metadata_schedule_231_e2705: f64 = (-params.p43);
        let noise_metadata_schedule_231_e2706: f64 = (noise_metadata_schedule_231_e2703).powf(noise_metadata_schedule_231_e2705);
        (noise_metadata_schedule_231_e2706,)
    } else {
        (noise_variable_238,)
    }
};
            noise_variable_238 = noise_metadata_schedule_231_e2708;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_232_e2722,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_232_e2716: f64 = (noise_variable_225 / noise_variable_21);
        let noise_metadata_schedule_232_e2717: f64 = (1.0 + noise_metadata_schedule_232_e2716);
        let noise_metadata_schedule_232_e2719: f64 = (-params.p43);
        let noise_metadata_schedule_232_e2720: f64 = (noise_metadata_schedule_232_e2717).powf(noise_metadata_schedule_232_e2719);
        (noise_metadata_schedule_232_e2720,)
    } else {
        (noise_variable_239,)
    }
};
            noise_variable_239 = noise_metadata_schedule_232_e2722;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_233_e2737,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_233_e2729: f64 = (1.0 - noise_variable_237);
        let noise_metadata_schedule_233_e2731: f64 = (noise_metadata_schedule_233_e2729 * noise_variable_238);
        let noise_metadata_schedule_233_e2734: f64 = (noise_variable_237 * noise_variable_239);
        let noise_metadata_schedule_233_e2735: f64 = (noise_metadata_schedule_233_e2731 + noise_metadata_schedule_233_e2734);
        (noise_metadata_schedule_233_e2735,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_233_e2737;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_234_e2750,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_234_e2744: f64 = (noise_variable_144 - noise_variable_236);
        let noise_metadata_schedule_234_e2746: f64 = (noise_metadata_schedule_234_e2744 + noise_variable_232);
        let noise_metadata_schedule_234_e2748: f64 = (noise_metadata_schedule_234_e2746 * noise_variable_240);
        (noise_metadata_schedule_234_e2748,)
    } else {
        (noise_variable_241,)
    }
};
            noise_variable_241 = noise_metadata_schedule_234_e2750;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_235_e2761,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_235_e2757: f64 = (noise_variable_241 + noise_variable_228);
        let noise_metadata_schedule_235_e2759: f64 = (noise_metadata_schedule_235_e2757 - noise_variable_233);
        (noise_metadata_schedule_235_e2759,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_235_e2761;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_236_e2778,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_236_e2769: f64 = (noise_variable_225 * noise_variable_225);
        let noise_metadata_schedule_236_e2772: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_236_e2774: f64 = (noise_metadata_schedule_236_e2772 * params.p44);
        let noise_metadata_schedule_236_e2775: f64 = (noise_metadata_schedule_236_e2769 + noise_metadata_schedule_236_e2774);
        let noise_metadata_schedule_236_e2776: f64 = (noise_metadata_schedule_236_e2775).sqrt();
        (noise_metadata_schedule_236_e2776,)
    } else {
        (noise_variable_242,)
    }
};
            noise_variable_242 = noise_metadata_schedule_236_e2778;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_237_e2791,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_237_e2785: f64 = (-0.5);
        let noise_metadata_schedule_237_e2788: f64 = (noise_variable_225 + noise_variable_242);
        let noise_metadata_schedule_237_e2789: f64 = (noise_metadata_schedule_237_e2785 * noise_metadata_schedule_237_e2788);
        (noise_metadata_schedule_237_e2789,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_237_e2791;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_238_e2814,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_238_e2798: f64 = (-noise_variable_21);
        let noise_metadata_schedule_238_e2802: f64 = (noise_variable_232 / noise_variable_21);
        let noise_metadata_schedule_238_e2803: f64 = (1.0 - noise_metadata_schedule_238_e2802);
        let noise_metadata_schedule_238_e2806: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_238_e2807: f64 = (noise_metadata_schedule_238_e2803).powf(noise_metadata_schedule_238_e2806);
        let noise_metadata_schedule_238_e2808: f64 = (noise_metadata_schedule_238_e2798 * noise_metadata_schedule_238_e2807);
        let noise_metadata_schedule_238_e2811: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_238_e2812: f64 = (noise_metadata_schedule_238_e2808 / noise_metadata_schedule_238_e2811);
        (noise_metadata_schedule_238_e2812,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_238_e2814;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_239_e2824,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_239_e2822: f64 = (noise_variable_144 + noise_variable_225);
        (noise_metadata_schedule_239_e2822,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_239_e2824;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_240_e2841,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_240_e2832: f64 = (noise_variable_244 * noise_variable_244);
        let noise_metadata_schedule_240_e2835: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_240_e2837: f64 = (noise_metadata_schedule_240_e2835 * params.p44);
        let noise_metadata_schedule_240_e2838: f64 = (noise_metadata_schedule_240_e2832 + noise_metadata_schedule_240_e2837);
        let noise_metadata_schedule_240_e2839: f64 = (noise_metadata_schedule_240_e2838).sqrt();
        (noise_metadata_schedule_240_e2839,)
    } else {
        (noise_variable_245,)
    }
};
            noise_variable_245 = noise_metadata_schedule_240_e2841;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_241_e2855,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_241_e2850: f64 = (noise_variable_244 - noise_variable_245);
        let noise_metadata_schedule_241_e2851: f64 = (0.5 * noise_metadata_schedule_241_e2850);
        let noise_metadata_schedule_241_e2853: f64 = (noise_metadata_schedule_241_e2851 - noise_variable_225);
        (noise_metadata_schedule_241_e2853,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_241_e2855;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_242_e2878,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_242_e2862: f64 = (-noise_variable_21);
        let noise_metadata_schedule_242_e2866: f64 = (noise_variable_236 / noise_variable_21);
        let noise_metadata_schedule_242_e2867: f64 = (1.0 - noise_metadata_schedule_242_e2866);
        let noise_metadata_schedule_242_e2870: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_242_e2871: f64 = (noise_metadata_schedule_242_e2867).powf(noise_metadata_schedule_242_e2870);
        let noise_metadata_schedule_242_e2872: f64 = (noise_metadata_schedule_242_e2862 * noise_metadata_schedule_242_e2871);
        let noise_metadata_schedule_242_e2875: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_242_e2876: f64 = (noise_metadata_schedule_242_e2872 / noise_metadata_schedule_242_e2875);
        (noise_metadata_schedule_242_e2876,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_242_e2878;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_243_e2901,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_243_e2887: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_243_e2889: f64 = (-params.p43);
        let noise_metadata_schedule_243_e2890: f64 = (noise_metadata_schedule_243_e2887).powf(noise_metadata_schedule_243_e2889);
        let noise_metadata_schedule_243_e2893: f64 = (noise_variable_144 - noise_variable_236);
        let noise_metadata_schedule_243_e2895: f64 = (noise_metadata_schedule_243_e2893 + noise_variable_232);
        let noise_metadata_schedule_243_e2896: f64 = (noise_metadata_schedule_243_e2890 * noise_metadata_schedule_243_e2895);
        let noise_metadata_schedule_243_e2897: f64 = (noise_variable_228 + noise_metadata_schedule_243_e2896);
        let noise_metadata_schedule_243_e2899: f64 = (noise_metadata_schedule_243_e2897 - noise_variable_243);
        (noise_metadata_schedule_243_e2899,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_243_e2901;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_244_e2905: f64 = (noise_variable_27 * noise_variable_73);
            let noise_metadata_schedule_244_e2906: f64 = (1.0 / noise_metadata_schedule_244_e2905);
            noise_variable_112 = noise_metadata_schedule_244_e2906;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_245_e2909: f64 = if noise_variable_143 < noise_variable_61 { 1.0 } else { 0.0 };
            noise_variable_250 = noise_metadata_schedule_245_e2909;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_246_e2916,) = {
    if (noise_variable_250 != 0.0) {
        let noise_metadata_schedule_246_e2913: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_246_e2914: f64 = (noise_metadata_schedule_246_e2913).exp();
        (noise_metadata_schedule_246_e2914,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_246_e2916;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_247_e2932,) = {
    if (noise_variable_250 == 0.0) {
        let noise_metadata_schedule_247_e2921: f64 = (noise_variable_61 * noise_variable_112);
        let noise_metadata_schedule_247_e2922: f64 = (noise_metadata_schedule_247_e2921).exp();
        let noise_metadata_schedule_247_e2926: f64 = (noise_variable_143 - noise_variable_61);
        let noise_metadata_schedule_247_e2928: f64 = (noise_metadata_schedule_247_e2926 * noise_variable_112);
        let noise_metadata_schedule_247_e2929: f64 = (1.0 + noise_metadata_schedule_247_e2928);
        let noise_metadata_schedule_247_e2930: f64 = (noise_metadata_schedule_247_e2922 * noise_metadata_schedule_247_e2929);
        (noise_metadata_schedule_247_e2930,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_247_e2932;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_248_e2936: f64 = (noise_variable_109 - 1.0);
            let noise_metadata_schedule_248_e2937: f64 = (noise_variable_0 * noise_metadata_schedule_248_e2936);
            noise_variable_74 = noise_metadata_schedule_248_e2937;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_249_e2941: f64 = (noise_variable_28 * noise_variable_73);
            let noise_metadata_schedule_249_e2942: f64 = (1.0 / noise_metadata_schedule_249_e2941);
            noise_variable_112 = noise_metadata_schedule_249_e2942;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let noise_metadata_schedule_250_e2945: f64 = if noise_variable_144 < noise_variable_62 { 1.0 } else { 0.0 };
            noise_variable_251 = noise_metadata_schedule_250_e2945;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_251_e2952,) = {
    if (noise_variable_251 != 0.0) {
        let noise_metadata_schedule_251_e2949: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_251_e2950: f64 = (noise_metadata_schedule_251_e2949).exp();
        (noise_metadata_schedule_251_e2950,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_251_e2952;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12 | 13) {
            let (noise_metadata_schedule_252_e2968,) = {
    if (noise_variable_251 == 0.0) {
        let noise_metadata_schedule_252_e2957: f64 = (noise_variable_62 * noise_variable_112);
        let noise_metadata_schedule_252_e2958: f64 = (noise_metadata_schedule_252_e2957).exp();
        let noise_metadata_schedule_252_e2962: f64 = (noise_variable_144 - noise_variable_62);
        let noise_metadata_schedule_252_e2964: f64 = (noise_metadata_schedule_252_e2962 * noise_variable_112);
        let noise_metadata_schedule_252_e2965: f64 = (1.0 + noise_metadata_schedule_252_e2964);
        let noise_metadata_schedule_252_e2966: f64 = (noise_metadata_schedule_252_e2958 * noise_metadata_schedule_252_e2965);
        (noise_metadata_schedule_252_e2966,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_252_e2968;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_253_e2971: f64 = (noise_variable_0 * noise_variable_1);
            let noise_metadata_schedule_253_e2974: f64 = (noise_variable_109 - 1.0);
            let noise_metadata_schedule_253_e2975: f64 = (noise_metadata_schedule_253_e2971 * noise_metadata_schedule_253_e2974);
            noise_variable_75 = noise_metadata_schedule_253_e2975;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_254_e2979: f64 = (noise_variable_114 * noise_variable_44);
            let noise_metadata_schedule_254_e2980: f64 = (1.0 + noise_metadata_schedule_254_e2979);
            let noise_metadata_schedule_254_e2983: f64 = (noise_variable_116 * noise_variable_43);
            let noise_metadata_schedule_254_e2984: f64 = (noise_metadata_schedule_254_e2980 + noise_metadata_schedule_254_e2983);
            let noise_metadata_schedule_254_e2986: f64 = (noise_metadata_schedule_254_e2984 - 0.0001);
            noise_variable_78 = noise_metadata_schedule_254_e2986;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_255_e2990: f64 = (noise_variable_78 * noise_variable_78);
            let noise_metadata_schedule_255_e2992: f64 = (noise_metadata_schedule_255_e2990 + 1e-8);
            let noise_metadata_schedule_255_e2993: f64 = (noise_metadata_schedule_255_e2992).sqrt();
            let noise_metadata_schedule_255_e2995: f64 = (noise_metadata_schedule_255_e2993 + noise_variable_78);
            let noise_metadata_schedule_255_e2996: f64 = (0.5 * noise_metadata_schedule_255_e2995);
            let noise_metadata_schedule_255_e2998: f64 = (noise_metadata_schedule_255_e2996 + 0.0001);
            noise_variable_79 = noise_metadata_schedule_255_e2998;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_256_e3001: f64 = (noise_variable_74 * noise_variable_45);
            let noise_metadata_schedule_256_e3004: f64 = (noise_variable_75 * noise_variable_46);
            let noise_metadata_schedule_256_e3005: f64 = (noise_metadata_schedule_256_e3001 + noise_metadata_schedule_256_e3004);
            noise_variable_80 = noise_metadata_schedule_256_e3005;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let noise_metadata_schedule_257_e3008: f64 = if params.p30 < 0.5 { 1.0 } else { 0.0 };
            noise_variable_252 = noise_metadata_schedule_257_e3008;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_258_e3020,) = {
    if (noise_variable_252 != 0.0) {
        let noise_metadata_schedule_258_e3013: f64 = (1.0 / params.p73);
        let noise_metadata_schedule_258_e3014: f64 = (noise_variable_79).powf(noise_metadata_schedule_258_e3013);
        let noise_metadata_schedule_258_e3017: f64 = (4.0 * noise_variable_80);
        let noise_metadata_schedule_258_e3018: f64 = (noise_metadata_schedule_258_e3014 + noise_metadata_schedule_258_e3017);
        (noise_metadata_schedule_258_e3018,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_258_e3020;
        }
        if matches!(source_index, 4 | 10) {
            let noise_metadata_schedule_259_e3023: f64 = if noise_variable_108 > 1e-8 { 1.0 } else { 0.0 };
            noise_variable_253 = noise_metadata_schedule_259_e3023;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_260_e3035,) = {
    if ((noise_variable_252 != 0.0) && (noise_variable_253 != 0.0)) {
        let noise_metadata_schedule_260_e3031: f64 = (noise_variable_108).powf(params.p73);
        let noise_metadata_schedule_260_e3032: f64 = (noise_variable_79 + noise_metadata_schedule_260_e3031);
        let noise_metadata_schedule_260_e3033: f64 = (0.5 * noise_metadata_schedule_260_e3032);
        (noise_metadata_schedule_260_e3033,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_260_e3035;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_261_e3048,) = {
    if ((noise_variable_252 != 0.0) && (noise_variable_253 == 0.0)) {
        let noise_metadata_schedule_261_e3044: f64 = (1e-8_f64).powf(params.p73);
        let noise_metadata_schedule_261_e3045: f64 = (noise_variable_79 + noise_metadata_schedule_261_e3044);
        let noise_metadata_schedule_261_e3046: f64 = (0.5 * noise_metadata_schedule_261_e3045);
        (noise_metadata_schedule_261_e3046,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_261_e3048;
        }
        if matches!(source_index, 4 | 10 | 12 | 13) {
            let (noise_metadata_schedule_262_e3057,) = {
    if (noise_variable_252 == 0.0) {
        let noise_metadata_schedule_262_e3054: f64 = (4.0 * noise_variable_80);
        let noise_metadata_schedule_262_e3055: f64 = (1.0 + noise_metadata_schedule_262_e3054);
        (noise_metadata_schedule_262_e3055,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_262_e3057;
        }
        if matches!(source_index, 4 | 10) {
            let noise_metadata_schedule_263_e3060: f64 = if noise_variable_108 > 1e-8 { 1.0 } else { 0.0 };
            noise_variable_254 = noise_metadata_schedule_263_e3060;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_264_e3075,) = {
    if ((noise_variable_252 == 0.0) && (noise_variable_254 != 0.0)) {
        let noise_metadata_schedule_264_e3067: f64 = (0.5 * noise_variable_79);
        let noise_metadata_schedule_264_e3071: f64 = (noise_variable_108).powf(params.p73);
        let noise_metadata_schedule_264_e3072: f64 = (1.0 + noise_metadata_schedule_264_e3071);
        let noise_metadata_schedule_264_e3073: f64 = (noise_metadata_schedule_264_e3067 * noise_metadata_schedule_264_e3072);
        (noise_metadata_schedule_264_e3073,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_264_e3075;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_265_e3091,) = {
    if ((noise_variable_252 == 0.0) && (noise_variable_254 == 0.0)) {
        let noise_metadata_schedule_265_e3083: f64 = (0.5 * noise_variable_79);
        let noise_metadata_schedule_265_e3087: f64 = (1e-8_f64).powf(params.p73);
        let noise_metadata_schedule_265_e3088: f64 = (1.0 + noise_metadata_schedule_265_e3087);
        let noise_metadata_schedule_265_e3089: f64 = (noise_metadata_schedule_265_e3083 * noise_metadata_schedule_265_e3088);
        (noise_metadata_schedule_265_e3089,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_265_e3091;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_267_e3097: f64 = (noise_variable_74 / noise_variable_81);
            noise_variable_76 = noise_metadata_schedule_267_e3097;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let noise_metadata_schedule_269_e3101: f64 = if params.p31 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_255 = noise_metadata_schedule_269_e3101;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let (noise_metadata_schedule_270_e3109,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_270_e3106: f64 = (params.p33 * noise_variable_73);
        let noise_metadata_schedule_270_e3107: f64 = (1.0 / noise_metadata_schedule_270_e3106);
        (noise_metadata_schedule_270_e3107,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_270_e3109;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let noise_metadata_schedule_271_e3112: f64 = if noise_variable_146 < noise_variable_63 { 1.0 } else { 0.0 };
            noise_variable_256 = noise_metadata_schedule_271_e3112;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let (noise_metadata_schedule_272_e3121,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_256 != 0.0)) {
        let noise_metadata_schedule_272_e3118: f64 = (noise_variable_146 * noise_variable_112);
        let noise_metadata_schedule_272_e3119: f64 = (noise_metadata_schedule_272_e3118).exp();
        (noise_metadata_schedule_272_e3119,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_272_e3121;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12 | 13) {
            let (noise_metadata_schedule_273_e3139,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_256 == 0.0)) {
        let noise_metadata_schedule_273_e3128: f64 = (noise_variable_63 * noise_variable_112);
        let noise_metadata_schedule_273_e3129: f64 = (noise_metadata_schedule_273_e3128).exp();
        let noise_metadata_schedule_273_e3133: f64 = (noise_variable_146 - noise_variable_63);
        let noise_metadata_schedule_273_e3135: f64 = (noise_metadata_schedule_273_e3133 * noise_variable_112);
        let noise_metadata_schedule_273_e3136: f64 = (1.0 + noise_metadata_schedule_273_e3135);
        let noise_metadata_schedule_273_e3137: f64 = (noise_metadata_schedule_273_e3129 * noise_metadata_schedule_273_e3136);
        (noise_metadata_schedule_273_e3137,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_273_e3139;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8 | 12 | 13) {
            let noise_metadata_schedule_274_e3142: f64 = if noise_variable_144 < noise_variable_63 { 1.0 } else { 0.0 };
            noise_variable_257 = noise_metadata_schedule_274_e3142;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8 | 12 | 13) {
            let (noise_metadata_schedule_275_e3151,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_257 != 0.0)) {
        let noise_metadata_schedule_275_e3148: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_275_e3149: f64 = (noise_metadata_schedule_275_e3148).exp();
        (noise_metadata_schedule_275_e3149,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_275_e3151;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8 | 12 | 13) {
            let (noise_metadata_schedule_276_e3169,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_257 == 0.0)) {
        let noise_metadata_schedule_276_e3158: f64 = (noise_variable_63 * noise_variable_112);
        let noise_metadata_schedule_276_e3159: f64 = (noise_metadata_schedule_276_e3158).exp();
        let noise_metadata_schedule_276_e3163: f64 = (noise_variable_144 - noise_variable_63);
        let noise_metadata_schedule_276_e3165: f64 = (noise_metadata_schedule_276_e3163 * noise_variable_112);
        let noise_metadata_schedule_276_e3166: f64 = (1.0 + noise_metadata_schedule_276_e3165);
        let noise_metadata_schedule_276_e3167: f64 = (noise_metadata_schedule_276_e3159 * noise_metadata_schedule_276_e3166);
        (noise_metadata_schedule_276_e3167,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_276_e3169;
        }
        if matches!(source_index, 12 | 13) {
            let (noise_metadata_schedule_277_e3185,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_277_e3174: f64 = (params.p32 * noise_variable_109);
        let noise_metadata_schedule_277_e3177: f64 = (1.0 - params.p32);
        let noise_metadata_schedule_277_e3179: f64 = (noise_metadata_schedule_277_e3177 * noise_variable_111);
        let noise_metadata_schedule_277_e3180: f64 = (noise_metadata_schedule_277_e3174 + noise_metadata_schedule_277_e3179);
        let noise_metadata_schedule_277_e3182: f64 = (noise_metadata_schedule_277_e3180 - 1.0);
        let noise_metadata_schedule_277_e3183: f64 = (noise_variable_5 * noise_metadata_schedule_277_e3182);
        (noise_metadata_schedule_277_e3183,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_277_e3185;
        }
        if matches!(source_index, 12 | 13) {
            let (noise_metadata_schedule_278_e3191,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_278_e3189: f64 = (noise_variable_82 * noise_variable_47);
        (noise_metadata_schedule_278_e3189,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_278_e3191;
        }
        if matches!(source_index, 12 | 13) {
            let (noise_metadata_schedule_279_e3199,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_279_e3196: f64 = (4.0 * noise_variable_85);
        let noise_metadata_schedule_279_e3197: f64 = (1.0 + noise_metadata_schedule_279_e3196);
        (noise_metadata_schedule_279_e3197,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_279_e3199;
        }
        if matches!(source_index, 12 | 13) {
            let noise_metadata_schedule_280_e3202: f64 = if noise_variable_108 > 1e-8 { 1.0 } else { 0.0 };
            noise_variable_258 = noise_metadata_schedule_280_e3202;
        }
        if matches!(source_index, 12 | 13) {
            let (noise_metadata_schedule_281_e3213,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_258 != 0.0)) {
        let noise_metadata_schedule_281_e3209: f64 = (noise_variable_108).sqrt();
        let noise_metadata_schedule_281_e3210: f64 = (1.0 + noise_metadata_schedule_281_e3209);
        let noise_metadata_schedule_281_e3211: f64 = (0.5 * noise_metadata_schedule_281_e3210);
        (noise_metadata_schedule_281_e3211,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_281_e3213;
        }
        if matches!(source_index, 12 | 13) {
            let (noise_metadata_schedule_282_e3225,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_258 == 0.0)) {
        let noise_metadata_schedule_282_e3221: f64 = (1e-8_f64).sqrt();
        let noise_metadata_schedule_282_e3222: f64 = (1.0 + noise_metadata_schedule_282_e3221);
        let noise_metadata_schedule_282_e3223: f64 = (0.5 * noise_metadata_schedule_282_e3222);
        (noise_metadata_schedule_282_e3223,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_282_e3225;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 13) {
            let noise_metadata_schedule_283_e3228: f64 = if noise_variable_147 < noise_variable_63 { 1.0 } else { 0.0 };
            noise_variable_259 = noise_metadata_schedule_283_e3228;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 13) {
            let (noise_metadata_schedule_284_e3237,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_259 != 0.0)) {
        let noise_metadata_schedule_284_e3234: f64 = (noise_variable_147 * noise_variable_112);
        let noise_metadata_schedule_284_e3235: f64 = (noise_metadata_schedule_284_e3234).exp();
        (noise_metadata_schedule_284_e3235,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_284_e3237;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 13) {
            let (noise_metadata_schedule_285_e3255,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_259 == 0.0)) {
        let noise_metadata_schedule_285_e3244: f64 = (noise_variable_63 * noise_variable_112);
        let noise_metadata_schedule_285_e3245: f64 = (noise_metadata_schedule_285_e3244).exp();
        let noise_metadata_schedule_285_e3249: f64 = (noise_variable_147 - noise_variable_63);
        let noise_metadata_schedule_285_e3251: f64 = (noise_metadata_schedule_285_e3249 * noise_variable_112);
        let noise_metadata_schedule_285_e3252: f64 = (1.0 + noise_metadata_schedule_285_e3251);
        let noise_metadata_schedule_285_e3253: f64 = (noise_metadata_schedule_285_e3245 * noise_metadata_schedule_285_e3252);
        (noise_metadata_schedule_285_e3253,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_285_e3255;
        }
        if matches!(source_index, 13) {
            let (noise_metadata_schedule_286_e3263,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_286_e3260: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_286_e3261: f64 = (noise_variable_5 * noise_metadata_schedule_286_e3260);
        (noise_metadata_schedule_286_e3261,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_286_e3263;
        }
        if matches!(source_index, 13) {
            let (noise_metadata_schedule_287_e3271,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_287_e3267: f64 = (noise_variable_82 - noise_variable_83);
        let noise_metadata_schedule_287_e3269: f64 = (noise_metadata_schedule_287_e3267 / noise_variable_86);
        (noise_metadata_schedule_287_e3269,)
    } else {
        (noise_variable_84,)
    }
};
            noise_variable_84 = noise_metadata_schedule_287_e3271;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_289_e3281,) = {
    if (noise_variable_255 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_289_e3281;
        }
        if matches!(source_index, 13) {
            let (noise_metadata_schedule_290_e3286,) = {
    if (noise_variable_255 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_84,)
    }
};
            noise_variable_84 = noise_metadata_schedule_290_e3286;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_291_e3289: f64 = if params.p55 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_260 = noise_metadata_schedule_291_e3289;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_292_e3297,) = {
    if (noise_variable_260 != 0.0) {
        let noise_metadata_schedule_292_e3294: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_292_e3295: f64 = (1.0 / noise_metadata_schedule_292_e3294);
        (noise_metadata_schedule_292_e3295,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_292_e3297;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_293_e3300: f64 = if noise_variable_143 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_261 = noise_metadata_schedule_293_e3300;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_294_e3309,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) {
        let noise_metadata_schedule_294_e3306: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_294_e3307: f64 = (noise_metadata_schedule_294_e3306).exp();
        (noise_metadata_schedule_294_e3307,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_294_e3309;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_295_e3327,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) {
        let noise_metadata_schedule_295_e3316: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_295_e3317: f64 = (noise_metadata_schedule_295_e3316).exp();
        let noise_metadata_schedule_295_e3321: f64 = (noise_variable_143 - noise_variable_65);
        let noise_metadata_schedule_295_e3323: f64 = (noise_metadata_schedule_295_e3321 * noise_variable_112);
        let noise_metadata_schedule_295_e3324: f64 = (1.0 + noise_metadata_schedule_295_e3323);
        let noise_metadata_schedule_295_e3325: f64 = (noise_metadata_schedule_295_e3317 * noise_metadata_schedule_295_e3324);
        (noise_metadata_schedule_295_e3325,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_295_e3327;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_296_e3335,) = {
    if (noise_variable_260 != 0.0) {
        let noise_metadata_schedule_296_e3332: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_296_e3333: f64 = (1.0 / noise_metadata_schedule_296_e3332);
        (noise_metadata_schedule_296_e3333,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_296_e3335;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_297_e3338: f64 = if noise_variable_143 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_262 = noise_metadata_schedule_297_e3338;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_298_e3347,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_298_e3344: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_298_e3345: f64 = (noise_metadata_schedule_298_e3344).exp();
        (noise_metadata_schedule_298_e3345,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_298_e3347;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_299_e3365,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_262 == 0.0)) {
        let noise_metadata_schedule_299_e3354: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_299_e3355: f64 = (noise_metadata_schedule_299_e3354).exp();
        let noise_metadata_schedule_299_e3359: f64 = (noise_variable_143 - noise_variable_66);
        let noise_metadata_schedule_299_e3361: f64 = (noise_metadata_schedule_299_e3359 * noise_variable_112);
        let noise_metadata_schedule_299_e3362: f64 = (1.0 + noise_metadata_schedule_299_e3361);
        let noise_metadata_schedule_299_e3363: f64 = (noise_metadata_schedule_299_e3355 * noise_metadata_schedule_299_e3362);
        (noise_metadata_schedule_299_e3363,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_299_e3365;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_300_e3368: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_263 = noise_metadata_schedule_300_e3368;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_301_e3392,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_301_e3377: f64 = (noise_variable_79 - 1.0);
        let noise_metadata_schedule_301_e3378: f64 = (params.p57 * noise_metadata_schedule_301_e3377);
        let noise_metadata_schedule_301_e3379: f64 = (1.0 + noise_metadata_schedule_301_e3378);
        let noise_metadata_schedule_301_e3380: f64 = (noise_variable_3 * noise_metadata_schedule_301_e3379);
        let noise_metadata_schedule_301_e3383: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_301_e3384: f64 = (noise_metadata_schedule_301_e3380 * noise_metadata_schedule_301_e3383);
        let noise_metadata_schedule_301_e3388: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_301_e3389: f64 = (noise_variable_6 * noise_metadata_schedule_301_e3388);
        let noise_metadata_schedule_301_e3390: f64 = (noise_metadata_schedule_301_e3384 + noise_metadata_schedule_301_e3389);
        (noise_metadata_schedule_301_e3390,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_301_e3392;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_302_e3409,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_263 == 0.0)) {
        let noise_metadata_schedule_302_e3400: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_302_e3401: f64 = (noise_variable_3 * noise_metadata_schedule_302_e3400);
        let noise_metadata_schedule_302_e3405: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_302_e3406: f64 = (noise_variable_6 * noise_metadata_schedule_302_e3405);
        let noise_metadata_schedule_302_e3407: f64 = (noise_metadata_schedule_302_e3401 + noise_metadata_schedule_302_e3406);
        (noise_metadata_schedule_302_e3407,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_302_e3409;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_303_e3412: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_264 = noise_metadata_schedule_303_e3412;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_304_e3421,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_264 != 0.0)) {
        let noise_metadata_schedule_304_e3417: f64 = (-noise_variable_31);
        let noise_metadata_schedule_304_e3419: f64 = (noise_metadata_schedule_304_e3417 - noise_variable_143);
        (noise_metadata_schedule_304_e3419,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_304_e3421;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_305_e3431,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_264 != 0.0)) {
        let noise_metadata_schedule_305_e3428: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_305_e3429: f64 = (1.0 / noise_metadata_schedule_305_e3428);
        (noise_metadata_schedule_305_e3429,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_305_e3431;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_306_e3434: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_265 = noise_metadata_schedule_306_e3434;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_307_e3445,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_264 != 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_307_e3442: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_307_e3443: f64 = (noise_metadata_schedule_307_e3442).exp();
        (noise_metadata_schedule_307_e3443,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_307_e3445;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_308_e3465,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_264 != 0.0)) && (noise_variable_265 == 0.0)) {
        let noise_metadata_schedule_308_e3454: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_308_e3455: f64 = (noise_metadata_schedule_308_e3454).exp();
        let noise_metadata_schedule_308_e3459: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_308_e3461: f64 = (noise_metadata_schedule_308_e3459 * noise_variable_112);
        let noise_metadata_schedule_308_e3462: f64 = (1.0 + noise_metadata_schedule_308_e3461);
        let noise_metadata_schedule_308_e3463: f64 = (noise_metadata_schedule_308_e3455 * noise_metadata_schedule_308_e3462);
        (noise_metadata_schedule_308_e3463,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_308_e3465;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_309_e3477,) = {
    if ((noise_variable_260 != 0.0) && (noise_variable_264 != 0.0)) {
        let noise_metadata_schedule_309_e3473: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_309_e3474: f64 = (params.p90 * noise_metadata_schedule_309_e3473);
        let noise_metadata_schedule_309_e3475: f64 = (noise_variable_87 - noise_metadata_schedule_309_e3474);
        (noise_metadata_schedule_309_e3475,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_309_e3477;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_310_e3481,) = {
    if (noise_variable_260 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_310_e3481;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_311_e3484: f64 = if params.p55 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_266 = noise_metadata_schedule_311_e3484;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_312_e3491,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_312_e3491;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_313_e3502,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) {
        let noise_metadata_schedule_313_e3499: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_313_e3500: f64 = (1.0 / noise_metadata_schedule_313_e3499);
        (noise_metadata_schedule_313_e3500,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_313_e3502;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_314_e3505: f64 = if noise_variable_145 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_267 = noise_metadata_schedule_314_e3505;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_315_e3517,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_315_e3514: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_315_e3515: f64 = (noise_metadata_schedule_315_e3514).exp();
        (noise_metadata_schedule_315_e3515,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_315_e3517;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_316_e3538,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 == 0.0)) {
        let noise_metadata_schedule_316_e3527: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_316_e3528: f64 = (noise_metadata_schedule_316_e3527).exp();
        let noise_metadata_schedule_316_e3532: f64 = (noise_variable_145 - noise_variable_65);
        let noise_metadata_schedule_316_e3534: f64 = (noise_metadata_schedule_316_e3532 * noise_variable_112);
        let noise_metadata_schedule_316_e3535: f64 = (1.0 + noise_metadata_schedule_316_e3534);
        let noise_metadata_schedule_316_e3536: f64 = (noise_metadata_schedule_316_e3528 * noise_metadata_schedule_316_e3535);
        (noise_metadata_schedule_316_e3536,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_316_e3538;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_317_e3549,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) {
        let noise_metadata_schedule_317_e3546: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_317_e3547: f64 = (1.0 / noise_metadata_schedule_317_e3546);
        (noise_metadata_schedule_317_e3547,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_317_e3549;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_318_e3552: f64 = if noise_variable_145 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_268 = noise_metadata_schedule_318_e3552;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_319_e3564,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_268 != 0.0)) {
        let noise_metadata_schedule_319_e3561: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_319_e3562: f64 = (noise_metadata_schedule_319_e3561).exp();
        (noise_metadata_schedule_319_e3562,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_319_e3564;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_320_e3585,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_268 == 0.0)) {
        let noise_metadata_schedule_320_e3574: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_320_e3575: f64 = (noise_metadata_schedule_320_e3574).exp();
        let noise_metadata_schedule_320_e3579: f64 = (noise_variable_145 - noise_variable_66);
        let noise_metadata_schedule_320_e3581: f64 = (noise_metadata_schedule_320_e3579 * noise_variable_112);
        let noise_metadata_schedule_320_e3582: f64 = (1.0 + noise_metadata_schedule_320_e3581);
        let noise_metadata_schedule_320_e3583: f64 = (noise_metadata_schedule_320_e3575 * noise_metadata_schedule_320_e3582);
        (noise_metadata_schedule_320_e3583,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_320_e3585;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_321_e3602,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) {
        let noise_metadata_schedule_321_e3593: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_321_e3594: f64 = (noise_variable_3 * noise_metadata_schedule_321_e3593);
        let noise_metadata_schedule_321_e3598: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_321_e3599: f64 = (noise_variable_6 * noise_metadata_schedule_321_e3598);
        let noise_metadata_schedule_321_e3600: f64 = (noise_metadata_schedule_321_e3594 + noise_metadata_schedule_321_e3599);
        (noise_metadata_schedule_321_e3600,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_321_e3602;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_322_e3605: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_269 = noise_metadata_schedule_322_e3605;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_323_e3617,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_269 != 0.0)) {
        let noise_metadata_schedule_323_e3613: f64 = (-noise_variable_31);
        let noise_metadata_schedule_323_e3615: f64 = (noise_metadata_schedule_323_e3613 - noise_variable_143);
        (noise_metadata_schedule_323_e3615,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_323_e3617;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_324_e3630,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_269 != 0.0)) {
        let noise_metadata_schedule_324_e3627: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_324_e3628: f64 = (1.0 / noise_metadata_schedule_324_e3627);
        (noise_metadata_schedule_324_e3628,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_324_e3630;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_325_e3633: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_270 = noise_metadata_schedule_325_e3633;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_326_e3647,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_269 != 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_326_e3644: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_326_e3645: f64 = (noise_metadata_schedule_326_e3644).exp();
        (noise_metadata_schedule_326_e3645,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_326_e3647;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_327_e3670,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_269 != 0.0)) && (noise_variable_270 == 0.0)) {
        let noise_metadata_schedule_327_e3659: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_327_e3660: f64 = (noise_metadata_schedule_327_e3659).exp();
        let noise_metadata_schedule_327_e3664: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_327_e3666: f64 = (noise_metadata_schedule_327_e3664 * noise_variable_112);
        let noise_metadata_schedule_327_e3667: f64 = (1.0 + noise_metadata_schedule_327_e3666);
        let noise_metadata_schedule_327_e3668: f64 = (noise_metadata_schedule_327_e3660 * noise_metadata_schedule_327_e3667);
        (noise_metadata_schedule_327_e3668,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_327_e3670;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_328_e3685,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_269 != 0.0)) {
        let noise_metadata_schedule_328_e3681: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_328_e3682: f64 = (params.p90 * noise_metadata_schedule_328_e3681);
        let noise_metadata_schedule_328_e3683: f64 = (noise_variable_88 - noise_metadata_schedule_328_e3682);
        (noise_metadata_schedule_328_e3683,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_328_e3685;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_329_e3697,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) {
        let noise_metadata_schedule_329_e3694: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_329_e3695: f64 = (1.0 / noise_metadata_schedule_329_e3694);
        (noise_metadata_schedule_329_e3695,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_329_e3697;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_330_e3700: f64 = if noise_variable_143 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_271 = noise_metadata_schedule_330_e3700;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_331_e3713,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_271 != 0.0)) {
        let noise_metadata_schedule_331_e3710: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_331_e3711: f64 = (noise_metadata_schedule_331_e3710).exp();
        (noise_metadata_schedule_331_e3711,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_331_e3713;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_332_e3735,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_271 == 0.0)) {
        let noise_metadata_schedule_332_e3724: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_332_e3725: f64 = (noise_metadata_schedule_332_e3724).exp();
        let noise_metadata_schedule_332_e3729: f64 = (noise_variable_143 - noise_variable_65);
        let noise_metadata_schedule_332_e3731: f64 = (noise_metadata_schedule_332_e3729 * noise_variable_112);
        let noise_metadata_schedule_332_e3732: f64 = (1.0 + noise_metadata_schedule_332_e3731);
        let noise_metadata_schedule_332_e3733: f64 = (noise_metadata_schedule_332_e3725 * noise_metadata_schedule_332_e3732);
        (noise_metadata_schedule_332_e3733,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_332_e3735;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_333_e3747,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) {
        let noise_metadata_schedule_333_e3744: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_333_e3745: f64 = (1.0 / noise_metadata_schedule_333_e3744);
        (noise_metadata_schedule_333_e3745,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_333_e3747;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_334_e3750: f64 = if noise_variable_143 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_272 = noise_metadata_schedule_334_e3750;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_335_e3763,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_335_e3760: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_335_e3761: f64 = (noise_metadata_schedule_335_e3760).exp();
        (noise_metadata_schedule_335_e3761,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_335_e3763;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_336_e3785,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_272 == 0.0)) {
        let noise_metadata_schedule_336_e3774: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_336_e3775: f64 = (noise_metadata_schedule_336_e3774).exp();
        let noise_metadata_schedule_336_e3779: f64 = (noise_variable_143 - noise_variable_66);
        let noise_metadata_schedule_336_e3781: f64 = (noise_metadata_schedule_336_e3779 * noise_variable_112);
        let noise_metadata_schedule_336_e3782: f64 = (1.0 + noise_metadata_schedule_336_e3781);
        let noise_metadata_schedule_336_e3783: f64 = (noise_metadata_schedule_336_e3775 * noise_metadata_schedule_336_e3782);
        (noise_metadata_schedule_336_e3783,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_336_e3785;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_337_e3788: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_273 = noise_metadata_schedule_337_e3788;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_338_e3818,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_273 != 0.0)) {
        let noise_metadata_schedule_338_e3802: f64 = (noise_variable_79 - 1.0);
        let noise_metadata_schedule_338_e3803: f64 = (params.p57 * noise_metadata_schedule_338_e3802);
        let noise_metadata_schedule_338_e3804: f64 = (1.0 + noise_metadata_schedule_338_e3803);
        let noise_metadata_schedule_338_e3805: f64 = (noise_variable_3 * noise_metadata_schedule_338_e3804);
        let noise_metadata_schedule_338_e3808: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_338_e3809: f64 = (noise_metadata_schedule_338_e3805 * noise_metadata_schedule_338_e3808);
        let noise_metadata_schedule_338_e3813: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_338_e3814: f64 = (noise_variable_6 * noise_metadata_schedule_338_e3813);
        let noise_metadata_schedule_338_e3815: f64 = (noise_metadata_schedule_338_e3809 + noise_metadata_schedule_338_e3814);
        let noise_metadata_schedule_338_e3816: f64 = (params.p55 * noise_metadata_schedule_338_e3815);
        (noise_metadata_schedule_338_e3816,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_338_e3818;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_339_e3841,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_273 == 0.0)) {
        let noise_metadata_schedule_339_e3831: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_339_e3832: f64 = (noise_variable_3 * noise_metadata_schedule_339_e3831);
        let noise_metadata_schedule_339_e3836: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_339_e3837: f64 = (noise_variable_6 * noise_metadata_schedule_339_e3836);
        let noise_metadata_schedule_339_e3838: f64 = (noise_metadata_schedule_339_e3832 + noise_metadata_schedule_339_e3837);
        let noise_metadata_schedule_339_e3839: f64 = (params.p55 * noise_metadata_schedule_339_e3838);
        (noise_metadata_schedule_339_e3839,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_339_e3841;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_340_e3844: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_274 = noise_metadata_schedule_340_e3844;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_341_e3857,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_274 != 0.0)) {
        let noise_metadata_schedule_341_e3853: f64 = (-noise_variable_31);
        let noise_metadata_schedule_341_e3855: f64 = (noise_metadata_schedule_341_e3853 - noise_variable_143);
        (noise_metadata_schedule_341_e3855,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_341_e3857;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_342_e3871,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_274 != 0.0)) {
        let noise_metadata_schedule_342_e3868: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_342_e3869: f64 = (1.0 / noise_metadata_schedule_342_e3868);
        (noise_metadata_schedule_342_e3869,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_342_e3871;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_343_e3874: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_275 = noise_metadata_schedule_343_e3874;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_344_e3889,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_274 != 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_344_e3886: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_344_e3887: f64 = (noise_metadata_schedule_344_e3886).exp();
        (noise_metadata_schedule_344_e3887,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_344_e3889;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_345_e3913,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_274 != 0.0)) && (noise_variable_275 == 0.0)) {
        let noise_metadata_schedule_345_e3902: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_345_e3903: f64 = (noise_metadata_schedule_345_e3902).exp();
        let noise_metadata_schedule_345_e3907: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_345_e3909: f64 = (noise_metadata_schedule_345_e3907 * noise_variable_112);
        let noise_metadata_schedule_345_e3910: f64 = (1.0 + noise_metadata_schedule_345_e3909);
        let noise_metadata_schedule_345_e3911: f64 = (noise_metadata_schedule_345_e3903 * noise_metadata_schedule_345_e3910);
        (noise_metadata_schedule_345_e3911,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_345_e3913;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_346_e3931,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_274 != 0.0)) {
        let noise_metadata_schedule_346_e3924: f64 = (params.p55 * params.p90);
        let noise_metadata_schedule_346_e3927: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_346_e3928: f64 = (noise_metadata_schedule_346_e3924 * noise_metadata_schedule_346_e3927);
        let noise_metadata_schedule_346_e3929: f64 = (noise_variable_87 - noise_metadata_schedule_346_e3928);
        (noise_metadata_schedule_346_e3929,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_346_e3931;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_347_e3943,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) {
        let noise_metadata_schedule_347_e3940: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_347_e3941: f64 = (1.0 / noise_metadata_schedule_347_e3940);
        (noise_metadata_schedule_347_e3941,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_347_e3943;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_348_e3946: f64 = if noise_variable_145 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_276 = noise_metadata_schedule_348_e3946;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_349_e3959,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_276 != 0.0)) {
        let noise_metadata_schedule_349_e3956: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_349_e3957: f64 = (noise_metadata_schedule_349_e3956).exp();
        (noise_metadata_schedule_349_e3957,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_349_e3959;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_350_e3981,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_276 == 0.0)) {
        let noise_metadata_schedule_350_e3970: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_350_e3971: f64 = (noise_metadata_schedule_350_e3970).exp();
        let noise_metadata_schedule_350_e3975: f64 = (noise_variable_145 - noise_variable_65);
        let noise_metadata_schedule_350_e3977: f64 = (noise_metadata_schedule_350_e3975 * noise_variable_112);
        let noise_metadata_schedule_350_e3978: f64 = (1.0 + noise_metadata_schedule_350_e3977);
        let noise_metadata_schedule_350_e3979: f64 = (noise_metadata_schedule_350_e3971 * noise_metadata_schedule_350_e3978);
        (noise_metadata_schedule_350_e3979,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_350_e3981;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_351_e3993,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) {
        let noise_metadata_schedule_351_e3990: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_351_e3991: f64 = (1.0 / noise_metadata_schedule_351_e3990);
        (noise_metadata_schedule_351_e3991,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_351_e3993;
        }
        if matches!(source_index, 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_352_e3996: f64 = if noise_variable_145 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_277 = noise_metadata_schedule_352_e3996;
        }
        if matches!(source_index, 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_353_e4009,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_353_e4006: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_353_e4007: f64 = (noise_metadata_schedule_353_e4006).exp();
        (noise_metadata_schedule_353_e4007,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_353_e4009;
        }
        if matches!(source_index, 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_354_e4031,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_277 == 0.0)) {
        let noise_metadata_schedule_354_e4020: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_354_e4021: f64 = (noise_metadata_schedule_354_e4020).exp();
        let noise_metadata_schedule_354_e4025: f64 = (noise_variable_145 - noise_variable_66);
        let noise_metadata_schedule_354_e4027: f64 = (noise_metadata_schedule_354_e4025 * noise_variable_112);
        let noise_metadata_schedule_354_e4028: f64 = (1.0 + noise_metadata_schedule_354_e4027);
        let noise_metadata_schedule_354_e4029: f64 = (noise_metadata_schedule_354_e4021 * noise_metadata_schedule_354_e4028);
        (noise_metadata_schedule_354_e4029,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_354_e4031;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_355_e4053,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) {
        let noise_metadata_schedule_355_e4039: f64 = (1.0 - params.p55);
        let noise_metadata_schedule_355_e4043: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_355_e4044: f64 = (noise_variable_3 * noise_metadata_schedule_355_e4043);
        let noise_metadata_schedule_355_e4048: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_355_e4049: f64 = (noise_variable_6 * noise_metadata_schedule_355_e4048);
        let noise_metadata_schedule_355_e4050: f64 = (noise_metadata_schedule_355_e4044 + noise_metadata_schedule_355_e4049);
        let noise_metadata_schedule_355_e4051: f64 = (noise_metadata_schedule_355_e4039 * noise_metadata_schedule_355_e4050);
        (noise_metadata_schedule_355_e4051,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_355_e4053;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let noise_metadata_schedule_356_e4056: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_278 = noise_metadata_schedule_356_e4056;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_357_e4069,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_278 != 0.0)) {
        let noise_metadata_schedule_357_e4065: f64 = (-noise_variable_31);
        let noise_metadata_schedule_357_e4067: f64 = (noise_metadata_schedule_357_e4065 - noise_variable_143);
        (noise_metadata_schedule_357_e4067,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_357_e4069;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_358_e4083,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_278 != 0.0)) {
        let noise_metadata_schedule_358_e4080: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_358_e4081: f64 = (1.0 / noise_metadata_schedule_358_e4080);
        (noise_metadata_schedule_358_e4081,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_358_e4083;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let noise_metadata_schedule_359_e4086: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_279 = noise_metadata_schedule_359_e4086;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_360_e4101,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_278 != 0.0)) && (noise_variable_279 != 0.0)) {
        let noise_metadata_schedule_360_e4098: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_360_e4099: f64 = (noise_metadata_schedule_360_e4098).exp();
        (noise_metadata_schedule_360_e4099,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_360_e4101;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_361_e4125,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_278 != 0.0)) && (noise_variable_279 == 0.0)) {
        let noise_metadata_schedule_361_e4114: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_361_e4115: f64 = (noise_metadata_schedule_361_e4114).exp();
        let noise_metadata_schedule_361_e4119: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_361_e4121: f64 = (noise_metadata_schedule_361_e4119 * noise_variable_112);
        let noise_metadata_schedule_361_e4122: f64 = (1.0 + noise_metadata_schedule_361_e4121);
        let noise_metadata_schedule_361_e4123: f64 = (noise_metadata_schedule_361_e4115 * noise_metadata_schedule_361_e4122);
        (noise_metadata_schedule_361_e4123,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_361_e4125;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_362_e4145,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_278 != 0.0)) {
        let noise_metadata_schedule_362_e4136: f64 = (1.0 - params.p55);
        let noise_metadata_schedule_362_e4138: f64 = (noise_metadata_schedule_362_e4136 * params.p90);
        let noise_metadata_schedule_362_e4141: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_362_e4142: f64 = (noise_metadata_schedule_362_e4138 * noise_metadata_schedule_362_e4141);
        let noise_metadata_schedule_362_e4143: f64 = (noise_variable_88 - noise_metadata_schedule_362_e4142);
        (noise_metadata_schedule_362_e4143,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_362_e4145;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_363_e4149: f64 = (params.p61 * noise_variable_73);
            let noise_metadata_schedule_363_e4150: f64 = (1.0 / noise_metadata_schedule_363_e4149);
            noise_variable_112 = noise_metadata_schedule_363_e4150;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_364_e4153: f64 = if noise_variable_144 < noise_variable_67 { 1.0 } else { 0.0 };
            noise_variable_280 = noise_metadata_schedule_364_e4153;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_365_e4160,) = {
    if (noise_variable_280 != 0.0) {
        let noise_metadata_schedule_365_e4157: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_365_e4158: f64 = (noise_metadata_schedule_365_e4157).exp();
        (noise_metadata_schedule_365_e4158,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_365_e4160;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_366_e4176,) = {
    if (noise_variable_280 == 0.0) {
        let noise_metadata_schedule_366_e4165: f64 = (noise_variable_67 * noise_variable_112);
        let noise_metadata_schedule_366_e4166: f64 = (noise_metadata_schedule_366_e4165).exp();
        let noise_metadata_schedule_366_e4170: f64 = (noise_variable_144 - noise_variable_67);
        let noise_metadata_schedule_366_e4172: f64 = (noise_metadata_schedule_366_e4170 * noise_variable_112);
        let noise_metadata_schedule_366_e4173: f64 = (1.0 + noise_metadata_schedule_366_e4172);
        let noise_metadata_schedule_366_e4174: f64 = (noise_metadata_schedule_366_e4166 * noise_metadata_schedule_366_e4173);
        (noise_metadata_schedule_366_e4174,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_366_e4176;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_367_e4180: f64 = (params.p63 * noise_variable_73);
            let noise_metadata_schedule_367_e4181: f64 = (1.0 / noise_metadata_schedule_367_e4180);
            noise_variable_112 = noise_metadata_schedule_367_e4181;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_368_e4184: f64 = if noise_variable_144 < noise_variable_68 { 1.0 } else { 0.0 };
            noise_variable_281 = noise_metadata_schedule_368_e4184;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_369_e4191,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_369_e4188: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_369_e4189: f64 = (noise_metadata_schedule_369_e4188).exp();
        (noise_metadata_schedule_369_e4189,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_369_e4191;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_370_e4207,) = {
    if (noise_variable_281 == 0.0) {
        let noise_metadata_schedule_370_e4196: f64 = (noise_variable_68 * noise_variable_112);
        let noise_metadata_schedule_370_e4197: f64 = (noise_metadata_schedule_370_e4196).exp();
        let noise_metadata_schedule_370_e4201: f64 = (noise_variable_144 - noise_variable_68);
        let noise_metadata_schedule_370_e4203: f64 = (noise_metadata_schedule_370_e4201 * noise_variable_112);
        let noise_metadata_schedule_370_e4204: f64 = (1.0 + noise_metadata_schedule_370_e4203);
        let noise_metadata_schedule_370_e4205: f64 = (noise_metadata_schedule_370_e4197 * noise_metadata_schedule_370_e4204);
        (noise_metadata_schedule_370_e4205,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_370_e4207;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_372_e4225: f64 = if ((params.p64 > 0.0) || (params.p65 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_282 = noise_metadata_schedule_372_e4225;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_373_e4233,) = {
    if (noise_variable_282 != 0.0) {
        let noise_metadata_schedule_373_e4230: f64 = (params.p61 * noise_variable_73);
        let noise_metadata_schedule_373_e4231: f64 = (1.0 / noise_metadata_schedule_373_e4230);
        (noise_metadata_schedule_373_e4231,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_373_e4233;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_374_e4236: f64 = if noise_variable_146 < noise_variable_69 { 1.0 } else { 0.0 };
            noise_variable_283 = noise_metadata_schedule_374_e4236;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_375_e4245,) = {
    if ((noise_variable_282 != 0.0) && (noise_variable_283 != 0.0)) {
        let noise_metadata_schedule_375_e4242: f64 = (noise_variable_146 * noise_variable_112);
        let noise_metadata_schedule_375_e4243: f64 = (noise_metadata_schedule_375_e4242).exp();
        (noise_metadata_schedule_375_e4243,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_375_e4245;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_376_e4263,) = {
    if ((noise_variable_282 != 0.0) && (noise_variable_283 == 0.0)) {
        let noise_metadata_schedule_376_e4252: f64 = (noise_variable_69 * noise_variable_112);
        let noise_metadata_schedule_376_e4253: f64 = (noise_metadata_schedule_376_e4252).exp();
        let noise_metadata_schedule_376_e4257: f64 = (noise_variable_146 - noise_variable_69);
        let noise_metadata_schedule_376_e4259: f64 = (noise_metadata_schedule_376_e4257 * noise_variable_112);
        let noise_metadata_schedule_376_e4260: f64 = (1.0 + noise_metadata_schedule_376_e4259);
        let noise_metadata_schedule_376_e4261: f64 = (noise_metadata_schedule_376_e4253 * noise_metadata_schedule_376_e4260);
        (noise_metadata_schedule_376_e4261,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_376_e4263;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_377_e4271,) = {
    if (noise_variable_282 != 0.0) {
        let noise_metadata_schedule_377_e4268: f64 = (params.p63 * noise_variable_73);
        let noise_metadata_schedule_377_e4269: f64 = (1.0 / noise_metadata_schedule_377_e4268);
        (noise_metadata_schedule_377_e4269,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_377_e4271;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_378_e4274: f64 = if noise_variable_146 < noise_variable_70 { 1.0 } else { 0.0 };
            noise_variable_284 = noise_metadata_schedule_378_e4274;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_379_e4283,) = {
    if ((noise_variable_282 != 0.0) && (noise_variable_284 != 0.0)) {
        let noise_metadata_schedule_379_e4280: f64 = (noise_variable_146 * noise_variable_112);
        let noise_metadata_schedule_379_e4281: f64 = (noise_metadata_schedule_379_e4280).exp();
        (noise_metadata_schedule_379_e4281,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_379_e4283;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_380_e4301,) = {
    if ((noise_variable_282 != 0.0) && (noise_variable_284 == 0.0)) {
        let noise_metadata_schedule_380_e4290: f64 = (noise_variable_70 * noise_variable_112);
        let noise_metadata_schedule_380_e4291: f64 = (noise_metadata_schedule_380_e4290).exp();
        let noise_metadata_schedule_380_e4295: f64 = (noise_variable_146 - noise_variable_70);
        let noise_metadata_schedule_380_e4297: f64 = (noise_metadata_schedule_380_e4295 * noise_variable_112);
        let noise_metadata_schedule_380_e4298: f64 = (1.0 + noise_metadata_schedule_380_e4297);
        let noise_metadata_schedule_380_e4299: f64 = (noise_metadata_schedule_380_e4291 * noise_metadata_schedule_380_e4298);
        (noise_metadata_schedule_380_e4299,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_380_e4301;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_381_e4315,) = {
    if (noise_variable_282 != 0.0) {
        let noise_metadata_schedule_381_e4306: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_381_e4307: f64 = (noise_variable_8 * noise_metadata_schedule_381_e4306);
        let noise_metadata_schedule_381_e4311: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_381_e4312: f64 = (noise_variable_9 * noise_metadata_schedule_381_e4311);
        let noise_metadata_schedule_381_e4313: f64 = (noise_metadata_schedule_381_e4307 + noise_metadata_schedule_381_e4312);
        (noise_metadata_schedule_381_e4313,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_381_e4315;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_382_e4320,) = {
    if (noise_variable_282 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_382_e4320;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_383_e4323: f64 = (noise_variable_144 / noise_variable_73);
            noise_variable_108 = noise_metadata_schedule_383_e4323;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_384_e4326: f64 = if noise_variable_108 < noise_variable_113 { 1.0 } else { 0.0 };
            noise_variable_285 = noise_metadata_schedule_384_e4326;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_385_e4331,) = {
    if (noise_variable_285 != 0.0) {
        let noise_metadata_schedule_385_e4329: f64 = (noise_variable_108).exp();
        (noise_metadata_schedule_385_e4329,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_385_e4331;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_386_e4343,) = {
    if (noise_variable_285 == 0.0) {
        let noise_metadata_schedule_386_e4335: f64 = (noise_variable_113).exp();
        let noise_metadata_schedule_386_e4339: f64 = (noise_variable_108 - noise_variable_113);
        let noise_metadata_schedule_386_e4340: f64 = (1.0 + noise_metadata_schedule_386_e4339);
        let noise_metadata_schedule_386_e4341: f64 = (noise_metadata_schedule_386_e4335 * noise_metadata_schedule_386_e4340);
        (noise_metadata_schedule_386_e4341,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_386_e4343;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_387_e4346: f64 = (noise_variable_148 / noise_variable_73);
            noise_variable_108 = noise_metadata_schedule_387_e4346;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_388_e4349: f64 = if noise_variable_108 < noise_variable_113 { 1.0 } else { 0.0 };
            noise_variable_286 = noise_metadata_schedule_388_e4349;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_389_e4354,) = {
    if (noise_variable_286 != 0.0) {
        let noise_metadata_schedule_389_e4352: f64 = (noise_variable_108).exp();
        (noise_metadata_schedule_389_e4352,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_389_e4354;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_390_e4366,) = {
    if (noise_variable_286 == 0.0) {
        let noise_metadata_schedule_390_e4358: f64 = (noise_variable_113).exp();
        let noise_metadata_schedule_390_e4362: f64 = (noise_variable_108 - noise_variable_113);
        let noise_metadata_schedule_390_e4363: f64 = (1.0 + noise_metadata_schedule_390_e4362);
        let noise_metadata_schedule_390_e4364: f64 = (noise_metadata_schedule_390_e4358 * noise_metadata_schedule_390_e4363);
        (noise_metadata_schedule_390_e4364,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_390_e4366;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_391_e4370: f64 = (noise_variable_33 * noise_variable_109);
            let noise_metadata_schedule_391_e4371: f64 = (1.0 + noise_metadata_schedule_391_e4370);
            let noise_metadata_schedule_391_e4372: f64 = (noise_metadata_schedule_391_e4371).sqrt();
            noise_variable_103 = noise_metadata_schedule_391_e4372;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_392_e4376: f64 = (noise_variable_33 * noise_variable_111);
            let noise_metadata_schedule_392_e4377: f64 = (1.0 + noise_metadata_schedule_392_e4376);
            let noise_metadata_schedule_392_e4378: f64 = (noise_metadata_schedule_392_e4377).sqrt();
            noise_variable_104 = noise_metadata_schedule_392_e4378;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_394_e4384: f64 = (noise_variable_103 + 1.0);
            let noise_metadata_schedule_394_e4387: f64 = (noise_variable_104 + 1.0);
            let noise_metadata_schedule_394_e4388: f64 = (noise_metadata_schedule_394_e4384 / noise_metadata_schedule_394_e4387);
            noise_variable_105 = noise_metadata_schedule_394_e4388;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_395_e4393: f64 = (noise_variable_103 - noise_variable_104);
            let noise_metadata_schedule_395_e4395: f64 = (noise_variable_105).ln();
            let noise_metadata_schedule_395_e4396: f64 = (noise_metadata_schedule_395_e4393 - noise_metadata_schedule_395_e4395);
            let noise_metadata_schedule_395_e4397: f64 = (noise_variable_73 * noise_metadata_schedule_395_e4396);
            let noise_metadata_schedule_395_e4398: f64 = (noise_variable_154 + noise_metadata_schedule_395_e4397);
            let noise_metadata_schedule_395_e4400: f64 = (noise_metadata_schedule_395_e4398 * noise_variable_54);
            noise_variable_106 = noise_metadata_schedule_395_e4400;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_396_e4403: f64 = (noise_variable_48 * noise_variable_106);
            let noise_metadata_schedule_396_e4408: f64 = (0.5 * noise_variable_48);
            let noise_metadata_schedule_396_e4410: f64 = (noise_metadata_schedule_396_e4408 * noise_variable_49);
            let noise_metadata_schedule_396_e4413: f64 = (noise_variable_154 * noise_variable_154);
            let noise_metadata_schedule_396_e4415: f64 = (noise_metadata_schedule_396_e4413 + 0.01);
            let noise_metadata_schedule_396_e4416: f64 = (noise_metadata_schedule_396_e4415).sqrt();
            let noise_metadata_schedule_396_e4417: f64 = (noise_metadata_schedule_396_e4410 * noise_metadata_schedule_396_e4416);
            let noise_metadata_schedule_396_e4418: f64 = (1.0 + noise_metadata_schedule_396_e4417);
            let noise_metadata_schedule_396_e4419: f64 = (noise_variable_54 * noise_metadata_schedule_396_e4418);
            let noise_metadata_schedule_396_e4420: f64 = (noise_metadata_schedule_396_e4403 / noise_metadata_schedule_396_e4419);
            noise_variable_107 = noise_metadata_schedule_396_e4420;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_397_e4425: f64 = (noise_variable_107 * noise_variable_107);
            let noise_metadata_schedule_397_e4426: f64 = (1.0 + noise_metadata_schedule_397_e4425);
            let noise_metadata_schedule_397_e4427: f64 = (noise_metadata_schedule_397_e4426).sqrt();
            let noise_metadata_schedule_397_e4428: f64 = (noise_variable_106 / noise_metadata_schedule_397_e4427);
            noise_variable_97 = noise_metadata_schedule_397_e4428;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_450_e4914: f64 = (noise_variable_165 * noise_variable_143);
            let noise_metadata_schedule_450_e4915: f64 = (noise_variable_87 + noise_metadata_schedule_450_e4914);
            noise_variable_87 = noise_metadata_schedule_450_e4915;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_451_e4919: f64 = (noise_variable_165 * noise_variable_145);
            let noise_metadata_schedule_451_e4920: f64 = (noise_variable_88 + noise_metadata_schedule_451_e4919);
            noise_variable_88 = noise_metadata_schedule_451_e4920;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_452_e4924: f64 = (noise_variable_165 * noise_variable_146);
            let noise_metadata_schedule_452_e4925: f64 = (noise_variable_91 + noise_metadata_schedule_452_e4924);
            noise_variable_91 = noise_metadata_schedule_452_e4925;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_456_e4943: f64 = noise_variable_162;
            let noise_metadata_schedule_456_e4945: f64 = (noise_metadata_schedule_456_e4943 * noise_variable_87);
            noise_variable_87 = noise_metadata_schedule_456_e4945;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_457_e4948: f64 = noise_variable_162;
            let noise_metadata_schedule_457_e4950: f64 = (noise_metadata_schedule_457_e4948 * noise_variable_88);
            noise_variable_88 = noise_metadata_schedule_457_e4950;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_458_e4953: f64 = noise_variable_162;
            let noise_metadata_schedule_458_e4955: f64 = (noise_metadata_schedule_458_e4953 * noise_variable_76);
            noise_variable_76 = noise_metadata_schedule_458_e4955;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_463_e4978: f64 = noise_variable_162;
            let noise_metadata_schedule_463_e4980: f64 = (noise_metadata_schedule_463_e4978 * noise_variable_91);
            noise_variable_91 = noise_metadata_schedule_463_e4980;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_465_e4986: f64 = noise_variable_162;
            let noise_metadata_schedule_465_e4988: f64 = (noise_metadata_schedule_465_e4986 * noise_variable_97);
            noise_variable_97 = noise_metadata_schedule_465_e4988;
        }
        if matches!(source_index, 13) {
            let noise_metadata_schedule_471_e5008: f64 = noise_variable_162;
            let noise_metadata_schedule_471_e5010: f64 = (noise_metadata_schedule_471_e5008 * noise_variable_84);
            noise_variable_84 = noise_metadata_schedule_471_e5010;
        }
        match source_index {
            0 => {
                let noise_0_psd_e6270: f64 = 1.0;
                let noise_0_psd_e183: f64 = 2.0;
                let noise_0_psd_e185: f64 = (noise_0_psd_e183 * 1.602189e-19);
                let noise_0_psd_e187: f64 = (noise_variable_87).abs();
                let noise_0_psd_e188: f64 = (noise_0_psd_e185 * noise_0_psd_e187);
                let noise_0_psd_e6271: f64 = (noise_0_psd_e6270 * noise_0_psd_e188);
                let psd = noise_0_psd_e6271;
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
                let noise_1_psd_e6273: f64 = 1.0;
                let noise_1_psd_e196: f64 = params.p98;
                let noise_1_psd_e199: f64 = noise_variable_87;
                let noise_1_psd_e200: f64 = (noise_1_psd_e199).abs();
                let noise_1_psd_e202: f64 = (noise_1_psd_e200).powf(params.p99);
                let noise_1_psd_e203: f64 = (noise_1_psd_e196 * noise_1_psd_e202);
                let noise_1_psd_e6274: f64 = (noise_1_psd_e6273 * noise_1_psd_e203);
                let psd = noise_1_psd_e6274;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = Some(params.p100);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            2 => {
                let noise_2_psd_e6276: f64 = 1.0;
                let noise_2_psd_e212: f64 = 2.0;
                let noise_2_psd_e214: f64 = (noise_2_psd_e212 * 1.602189e-19);
                let noise_2_psd_e216: f64 = (noise_variable_88).abs();
                let noise_2_psd_e217: f64 = (noise_2_psd_e214 * noise_2_psd_e216);
                let noise_2_psd_e6277: f64 = (noise_2_psd_e6276 * noise_2_psd_e217);
                let psd = noise_2_psd_e6277;
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
                let noise_3_psd_e6279: f64 = 1.0;
                let noise_3_psd_e225: f64 = params.p98;
                let noise_3_psd_e228: f64 = noise_variable_88;
                let noise_3_psd_e229: f64 = (noise_3_psd_e228).abs();
                let noise_3_psd_e231: f64 = (noise_3_psd_e229).powf(params.p99);
                let noise_3_psd_e232: f64 = (noise_3_psd_e225 * noise_3_psd_e231);
                let noise_3_psd_e6280: f64 = (noise_3_psd_e6279 * noise_3_psd_e232);
                let psd = noise_3_psd_e6280;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
                let exponent: Option<f64> = Some(params.p100);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            4 => {
                let noise_4_psd_e6282: f64 = 1.0;
                let noise_4_psd_e241: f64 = 2.0;
                let noise_4_psd_e243: f64 = (noise_4_psd_e241 * 1.602189e-19);
                let noise_4_psd_e245: f64 = (noise_variable_76).abs();
                let noise_4_psd_e246: f64 = (noise_4_psd_e243 * noise_4_psd_e245);
                let noise_4_psd_e6283: f64 = (noise_4_psd_e6282 * noise_4_psd_e246);
                let psd = noise_4_psd_e6283;
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
                let noise_5_psd_e6285: f64 = 1.0;
                let noise_5_psd_e254: f64 = 2.0;
                let noise_5_psd_e256: f64 = (noise_5_psd_e254 * 1.602189e-19);
                let noise_5_psd_e258: f64 = (noise_variable_91).abs();
                let noise_5_psd_e259: f64 = (noise_5_psd_e256 * noise_5_psd_e258);
                let noise_5_psd_e6286: f64 = (noise_5_psd_e6285 * noise_5_psd_e259);
                let psd = noise_5_psd_e6286;
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
                let noise_6_psd_e6288: f64 = 1.0;
                let noise_6_psd_e267: f64 = 1.0;
                let noise_6_psd_e269: f64 = (noise_6_psd_e267 * params.p98);
                let noise_6_psd_e272: f64 = noise_variable_91;
                let noise_6_psd_e273: f64 = (noise_6_psd_e272).abs();
                let noise_6_psd_e275: f64 = (noise_6_psd_e273).powf(params.p99);
                let noise_6_psd_e276: f64 = (noise_6_psd_e269 * noise_6_psd_e275);
                let noise_6_psd_e6289: f64 = (noise_6_psd_e6288 * noise_6_psd_e276);
                let psd = noise_6_psd_e6289;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
                let exponent: Option<f64> = Some(params.p100);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            7 => {
                let noise_7_psd_e6291: f64 = 1.0;
                let noise_7_psd_e285: f64 = 4.0;
                let noise_7_psd_e287: f64 = (noise_7_psd_e285 * 1.380662e-23);
                let noise_7_psd_e289: f64 = (noise_7_psd_e287 * noise_variable_39);
                let noise_7_psd_e291: f64 = (noise_7_psd_e289 * noise_variable_53);
                let noise_7_psd_e6292: f64 = (noise_7_psd_e6291 * noise_7_psd_e291);
                let psd = noise_7_psd_e6292;
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
                let noise_8_psd_e6294: f64 = 1.0;
                let noise_8_psd_e299: f64 = 4.0;
                let noise_8_psd_e301: f64 = (noise_8_psd_e299 * 1.380662e-23);
                let noise_8_psd_e303: f64 = (noise_8_psd_e301 * noise_variable_39);
                let noise_8_psd_e305: f64 = (noise_variable_97).abs();
                let noise_8_psd_e308: f64 = (1e-10 * noise_variable_54);
                let noise_8_psd_e309: f64 = (noise_8_psd_e305 + noise_8_psd_e308);
                let noise_8_psd_e311: f64 = (noise_variable_154).abs();
                let noise_8_psd_e313: f64 = (noise_8_psd_e311 + 1e-10);
                let noise_8_psd_e314: f64 = (noise_8_psd_e309 / noise_8_psd_e313);
                let noise_8_psd_e315: f64 = (noise_8_psd_e303 * noise_8_psd_e314);
                let noise_8_psd_e6295: f64 = (noise_8_psd_e6294 * noise_8_psd_e315);
                let psd = noise_8_psd_e6295;
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
                let noise_9_psd_e6297: f64 = 1.0;
                let noise_9_psd_e323: f64 = 4.0;
                let noise_9_psd_e325: f64 = (noise_9_psd_e323 * 1.380662e-23);
                let noise_9_psd_e327: f64 = (noise_9_psd_e325 * noise_variable_39);
                let noise_9_psd_e329: f64 = (noise_9_psd_e327 * noise_variable_55);
                let noise_9_psd_e6298: f64 = (noise_9_psd_e6297 * noise_9_psd_e329);
                let psd = noise_9_psd_e6298;
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
                let noise_10_psd_e6300: f64 = 1.0;
                let noise_10_psd_e337: f64 = 4.0;
                let noise_10_psd_e339: f64 = (noise_10_psd_e337 * 1.380662e-23);
                let noise_10_psd_e341: f64 = (noise_10_psd_e339 * noise_variable_39);
                let noise_10_psd_e343: f64 = (noise_10_psd_e341 * noise_variable_81);
                let noise_10_psd_e345: f64 = (noise_10_psd_e343 * noise_variable_56);
                let noise_10_psd_e6301: f64 = (noise_10_psd_e6300 * noise_10_psd_e345);
                let psd = noise_10_psd_e6301;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            11 => {
                let noise_11_psd_e6303: f64 = 1.0;
                let noise_11_psd_e353: f64 = 4.0;
                let noise_11_psd_e355: f64 = (noise_11_psd_e353 * 1.380662e-23);
                let noise_11_psd_e357: f64 = (noise_11_psd_e355 * noise_variable_39);
                let noise_11_psd_e359: f64 = (noise_11_psd_e357 * noise_variable_57);
                let noise_11_psd_e6304: f64 = (noise_11_psd_e6303 * noise_11_psd_e359);
                let psd = noise_11_psd_e6304;
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
                let noise_12_psd_e6306: f64 = 1.0;
                let noise_12_psd_e367: f64 = 4.0;
                let noise_12_psd_e369: f64 = (noise_12_psd_e367 * 1.380662e-23);
                let noise_12_psd_e371: f64 = (noise_12_psd_e369 * noise_variable_39);
                let noise_12_psd_e373: f64 = (noise_12_psd_e371 * noise_variable_86);
                let noise_12_psd_e375: f64 = (noise_12_psd_e373 * noise_variable_58);
                let noise_12_psd_e6307: f64 = (noise_12_psd_e6306 * noise_12_psd_e375);
                let psd = noise_12_psd_e6307;
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
                let noise_13_psd_e6309: f64 = 1.0;
                let noise_13_psd_e383: f64 = 2.0;
                let noise_13_psd_e385: f64 = (noise_13_psd_e383 * 1.602189e-19);
                let noise_13_psd_e387: f64 = (noise_variable_84).abs();
                let noise_13_psd_e388: f64 = (noise_13_psd_e385 * noise_13_psd_e387);
                let noise_13_psd_e6310: f64 = (noise_13_psd_e6309 * noise_13_psd_e388);
                let psd = noise_13_psd_e6310;
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
                let noise_14_psd_e6312: f64 = 1.0;
                let noise_14_psd_e396: f64 = 4.0;
                let noise_14_psd_e398: f64 = (noise_14_psd_e396 * 1.380662e-23);
                let noise_14_psd_e400: f64 = (noise_14_psd_e398 * noise_variable_39);
                let noise_14_psd_e402: f64 = (noise_14_psd_e400 * noise_variable_59);
                let noise_14_psd_e6313: f64 = (noise_14_psd_e6312 * noise_14_psd_e402);
                let psd = noise_14_psd_e6313;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

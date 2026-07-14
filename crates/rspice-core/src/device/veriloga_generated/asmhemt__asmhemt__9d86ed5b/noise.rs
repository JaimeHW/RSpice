#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 8] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS", label: Some("ids"), kind: GeneratedNoiseKind::White, equation: 65, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_FP4_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 66, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(18), name: "fp4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_FP4S_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 67, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(22), name: "fp4s", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 68, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 69, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 70, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 71, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 108, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 1 | 2 | 3 | 4) {
            noise_variable_361 = params.p34;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_activation_schedule_144_e2932: f64 = if params.p149 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_384 = noise_activation_schedule_144_e2932;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_activation_schedule_145_e2935: f64 = if noise_variable_361 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_385 = noise_activation_schedule_145_e2935;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_activation_schedule_146_e2941,) = {
    if ((noise_variable_384 != 0.0) && (noise_variable_385 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_361,)
    }
};
            noise_variable_361 = noise_activation_schedule_146_e2941;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4) {
            let noise_activation_schedule_608_e9221: f64 = if params.p260 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_429 = noise_activation_schedule_608_e9221;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_activation_schedule_610_e9298: f64 = if noise_variable_361 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_430 = noise_activation_schedule_610_e9298;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_activation_schedule_611_e9301: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_431 = noise_activation_schedule_611_e9301;
        }
        if matches!(source_index, 5 | 6) {
            let noise_activation_schedule_612_e9304: f64 = if params.p56 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_432 = noise_activation_schedule_612_e9304;
        }
        if matches!(source_index, 7) {
            let noise_activation_schedule_3162_e49409: f64 = if params.p259 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_567 = noise_activation_schedule_3162_e49409;
        }
        let noise_source_active = match source_index {
            0 => {
                noise_variable_429 != 0.0
            }
            1 => {
                let noise_1_activation_e1055: f64 = if (((noise_variable_429 != 0.0) && (noise_variable_430 != 0.0)) && (noise_variable_431 != 0.0)) { 1.0 } else { 0.0 };
                noise_1_activation_e1055 != 0.0
            }
            2 => {
                let noise_2_activation_e1073: f64 = if (((noise_variable_429 != 0.0) && (noise_variable_430 != 0.0)) && (noise_variable_431 != 0.0)) { 1.0 } else { 0.0 };
                noise_2_activation_e1073 != 0.0
            }
            3 => {
                let noise_3_activation_e1092: f64 = if (((noise_variable_429 != 0.0) && (noise_variable_430 != 0.0)) && (noise_variable_431 == 0.0)) { 1.0 } else { 0.0 };
                noise_3_activation_e1092 != 0.0
            }
            4 => {
                let noise_4_activation_e1111: f64 = if (((noise_variable_429 != 0.0) && (noise_variable_430 != 0.0)) && (noise_variable_431 == 0.0)) { 1.0 } else { 0.0 };
                noise_4_activation_e1111 != 0.0
            }
            5 => {
                noise_variable_432 != 0.0
            }
            6 => {
                noise_variable_432 != 0.0
            }
            7 => {
                noise_variable_567 != 0.0
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
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_186 = 1.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_213 = 0.0;
        }
        if matches!(source_index, 2 | 4) {
            noise_variable_214 = 0.0;
        }
        if matches!(source_index, 1 | 3) {
            noise_variable_215 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_216 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_209 = 0.0;
        }
        if matches!(source_index, 1 | 3) {
            noise_variable_210 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_211 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_212 = 0.0;
        }
        if matches!(source_index, 1 | 3) {
            noise_variable_185 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_231 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_243 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_255 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_267 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_279 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_291 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_303 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_315 = 0.0;
        }
        if matches!(source_index, 5) {
            noise_variable_206 = 0.0;
        }
        if matches!(source_index, 6) {
            noise_variable_207 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            noise_variable_182 = 0.01;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            noise_variable_183 = 0.01;
        }
        if matches!(source_index, 1 | 3) {
            noise_variable_144 = 0.0;
        }
        if matches!(source_index, 2 | 4) {
            noise_variable_145 = 0.0;
        }
        if matches!(source_index, 1 | 3) {
            noise_variable_142 = 0.0;
        }
        if matches!(source_index, 2 | 4) {
            noise_variable_143 = 0.0;
        }
        if matches!(source_index, 7) {
            noise_variable_48 = 1.0;
        }
        if matches!(source_index, 7) {
            noise_variable_56 = 1.0;
        }
        if matches!(source_index, 7) {
            noise_variable_64 = 1.0;
        }
        if matches!(source_index, 7) {
            noise_variable_72 = 1.0;
        }
        if matches!(source_index, 7) {
            noise_variable_52 = 1.0;
        }
        if matches!(source_index, 7) {
            noise_variable_60 = 1.0;
        }
        if matches!(source_index, 7) {
            noise_variable_68 = 1.0;
        }
        if matches!(source_index, 7) {
            noise_variable_76 = 1.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            noise_variable_321 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 6) {
            noise_variable_323 = 0.0;
        }
        if matches!(source_index, 5 | 6) {
            noise_variable_322 = 0.0;
        }
        if matches!(source_index, 5 | 6) {
            noise_variable_324 = 0.0;
        }
        if matches!(source_index, 5 | 6) {
            noise_variable_325 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            noise_variable_326 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 6) {
            noise_variable_327 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            noise_variable_328 = 1.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            noise_variable_329 = 1.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_339 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_344 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_345 = 0.0;
        }
        if matches!(source_index, 1 | 3) {
            noise_variable_341 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            noise_variable_340 = 0.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_346 = 0.0;
        }
        if matches!(source_index, 1 | 3) {
            noise_variable_366 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            noise_variable_365 = 0.0;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            noise_variable_361 = params.p34;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_metadata_schedule_144_e2932: f64 = if params.p149 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_384 = noise_metadata_schedule_144_e2932;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_metadata_schedule_145_e2935: f64 = if noise_variable_361 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_385 = noise_metadata_schedule_145_e2935;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_146_e2941,) = {
    if ((noise_variable_384 != 0.0) && (noise_variable_385 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_361,)
    }
};
            noise_variable_361 = noise_metadata_schedule_146_e2941;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_147_e2944: f64 = (params.p0 + 273.15);
            noise_variable_35 = noise_metadata_schedule_147_e2944;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_42 = (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8]));
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_43 = (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8]));
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_44 = (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7]));
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_46 = (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[8]));
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_47 = (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[7]));
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_41 = 1.0;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_154_e2953: f64 = if noise_variable_42 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_386 = noise_metadata_schedule_154_e2953;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_155_e2958,) = {
    if (noise_variable_386 != 0.0) {
        let noise_metadata_schedule_155_e2956: f64 = (-1.0);
        (noise_metadata_schedule_155_e2956,)
    } else {
        (noise_variable_41,)
    }
};
            noise_variable_41 = noise_metadata_schedule_155_e2958;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_156_e2964,) = {
    if (noise_variable_386 != 0.0) {
        let noise_metadata_schedule_156_e2962: f64 = (noise_variable_41 * noise_variable_42);
        (noise_metadata_schedule_156_e2962,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_156_e2964;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_157_e2968,) = {
    if (noise_variable_386 != 0.0) {
        (noise_variable_44,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_157_e2968;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_158_e2972,) = {
    if (noise_variable_386 != 0.0) {
        (noise_variable_47,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_158_e2972;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_159_e2977,) = {
    if (noise_variable_386 == 0.0) {
        (noise_variable_42,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_159_e2977;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_160_e2982,) = {
    if (noise_variable_386 == 0.0) {
        (noise_variable_43,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_160_e2982;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_161_e2987,) = {
    if (noise_variable_386 == 0.0) {
        (noise_variable_46,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_161_e2987;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_162_e2990: f64 = (noise_variable_38 * noise_variable_38);
            let noise_metadata_schedule_162_e2992: f64 = (noise_metadata_schedule_162_e2990 + 0.01);
            let noise_metadata_schedule_162_e2993: f64 = (noise_metadata_schedule_162_e2992).sqrt();
            let noise_metadata_schedule_162_e2995: f64 = (noise_metadata_schedule_162_e2993 - 0.1);
            noise_variable_140 = noise_metadata_schedule_162_e2995;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_163_e2998: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])) * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])));
            let noise_metadata_schedule_163_e3000: f64 = (noise_metadata_schedule_163_e2998 + 0.01);
            let noise_metadata_schedule_163_e3001: f64 = (noise_metadata_schedule_163_e3000).sqrt();
            let noise_metadata_schedule_163_e3003: f64 = (noise_metadata_schedule_163_e3001 - 0.1);
            noise_variable_141 = noise_metadata_schedule_163_e3003;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_164_e3004: f64 = ctx.temperature();
            let noise_metadata_schedule_164_e3006: f64 = (noise_metadata_schedule_164_e3004 + (ctx.node_voltage(self.nodes[4]) - 0.0));
            let noise_metadata_schedule_164_e3008: f64 = (noise_metadata_schedule_164_e3006 + params.p274);
            noise_variable_82 = noise_metadata_schedule_164_e3008;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_165_e3011: f64 = (8.617087e-5 * noise_variable_82);
            noise_variable_36 = noise_metadata_schedule_165_e3011;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_166_e3014: f64 = if params.p81 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_387 = noise_metadata_schedule_166_e3014;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_167_e3017: f64 = if params.p81 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_388 = noise_metadata_schedule_167_e3017;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_168_e3020: f64 = if params.p81 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_389 = noise_metadata_schedule_168_e3020;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_169_e3023: f64 = if params.p81 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_390 = noise_metadata_schedule_169_e3023;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_170_e3026: f64 = if params.p81 == 4.0 { 1.0 } else { 0.0 };
            noise_variable_391 = noise_metadata_schedule_170_e3026;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_171_e3029: f64 = if params.p81 == 5.0 { 1.0 } else { 0.0 };
            noise_variable_392 = noise_metadata_schedule_171_e3029;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_172_e3036,) = {
    if ((noise_variable_388 != 0.0) && (noise_variable_387 == 0.0)) {
        ((ctx.node_voltage(self.nodes[5]) - 0.0),)
    } else {
        (noise_variable_186,)
    }
};
            noise_variable_186 = noise_metadata_schedule_172_e3036;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_173_e3062,) = {
    if ((noise_variable_388 != 0.0) && (noise_variable_387 == 0.0)) {
        let noise_metadata_schedule_173_e3044: f64 = (noise_variable_186 + noise_variable_36);
        let noise_metadata_schedule_173_e3047: f64 = (noise_variable_186 - noise_variable_36);
        let noise_metadata_schedule_173_e3050: f64 = (noise_variable_186 - noise_variable_36);
        let noise_metadata_schedule_173_e3051: f64 = (noise_metadata_schedule_173_e3047 * noise_metadata_schedule_173_e3050);
        let noise_metadata_schedule_173_e3054: f64 = (0.25 * params.p128);
        let noise_metadata_schedule_173_e3056: f64 = (noise_metadata_schedule_173_e3054 * params.p128);
        let noise_metadata_schedule_173_e3057: f64 = (noise_metadata_schedule_173_e3051 + noise_metadata_schedule_173_e3056);
        let noise_metadata_schedule_173_e3058: f64 = (noise_metadata_schedule_173_e3057).sqrt();
        let noise_metadata_schedule_173_e3059: f64 = (noise_metadata_schedule_173_e3044 + noise_metadata_schedule_173_e3058);
        let noise_metadata_schedule_173_e3060: f64 = (0.5 * noise_metadata_schedule_173_e3059);
        (noise_metadata_schedule_173_e3060,)
    } else {
        (noise_variable_186,)
    }
};
            noise_variable_186 = noise_metadata_schedule_173_e3062;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_174_e3077,) = {
    if ((noise_variable_388 != 0.0) && (noise_variable_387 == 0.0)) {
        let noise_metadata_schedule_174_e3070: f64 = (-1.0);
        let noise_metadata_schedule_174_e3072: f64 = (noise_metadata_schedule_174_e3070 / noise_variable_186);
        let noise_metadata_schedule_174_e3073: f64 = { let limited_exp_arg = noise_metadata_schedule_174_e3072; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_174_e3074: f64 = (params.p101 * noise_metadata_schedule_174_e3073);
        let noise_metadata_schedule_174_e3075: f64 = (params.p100 + noise_metadata_schedule_174_e3074);
        (noise_metadata_schedule_174_e3075,)
    } else {
        (noise_variable_213,)
    }
};
            noise_variable_213 = noise_metadata_schedule_174_e3077;
        }
        if matches!(source_index, 2 | 4) {
            let (noise_metadata_schedule_175_e3092,) = {
    if ((noise_variable_388 != 0.0) && (noise_variable_387 == 0.0)) {
        let noise_metadata_schedule_175_e3085: f64 = (-1.0);
        let noise_metadata_schedule_175_e3087: f64 = (noise_metadata_schedule_175_e3085 / noise_variable_186);
        let noise_metadata_schedule_175_e3088: f64 = { let limited_exp_arg = noise_metadata_schedule_175_e3087; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_175_e3089: f64 = (params.p105 * noise_metadata_schedule_175_e3088);
        let noise_metadata_schedule_175_e3090: f64 = (params.p104 + noise_metadata_schedule_175_e3089);
        (noise_metadata_schedule_175_e3090,)
    } else {
        (noise_variable_214,)
    }
};
            noise_variable_214 = noise_metadata_schedule_175_e3092;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_176_e3107,) = {
    if ((noise_variable_388 != 0.0) && (noise_variable_387 == 0.0)) {
        let noise_metadata_schedule_176_e3100: f64 = (-1.0);
        let noise_metadata_schedule_176_e3102: f64 = (noise_metadata_schedule_176_e3100 / noise_variable_186);
        let noise_metadata_schedule_176_e3103: f64 = { let limited_exp_arg = noise_metadata_schedule_176_e3102; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_176_e3104: f64 = (params.p107 * noise_metadata_schedule_176_e3103);
        let noise_metadata_schedule_176_e3105: f64 = (params.p106 + noise_metadata_schedule_176_e3104);
        (noise_metadata_schedule_176_e3105,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_176_e3107;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_177_e3122,) = {
    if ((noise_variable_388 != 0.0) && (noise_variable_387 == 0.0)) {
        let noise_metadata_schedule_177_e3115: f64 = (-1.0);
        let noise_metadata_schedule_177_e3117: f64 = (noise_metadata_schedule_177_e3115 / noise_variable_186);
        let noise_metadata_schedule_177_e3118: f64 = { let limited_exp_arg = noise_metadata_schedule_177_e3117; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_177_e3119: f64 = (params.p103 * noise_metadata_schedule_177_e3118);
        let noise_metadata_schedule_177_e3120: f64 = (params.p102 + noise_metadata_schedule_177_e3119);
        (noise_metadata_schedule_177_e3120,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_177_e3122;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_179_e3146,) = {
    if ((noise_variable_389 != 0.0) && (!((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)))) {
        let noise_metadata_schedule_179_e3144: f64 = (params.p113 * (ctx.node_voltage(self.nodes[6]) - 0.0));
        (noise_metadata_schedule_179_e3144,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_179_e3146;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_180_e3164,) = {
    if ((noise_variable_389 != 0.0) && (!((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)))) {
        let noise_metadata_schedule_180_e3154: f64 = (-params.p116);
        let noise_metadata_schedule_180_e3156: f64 = (noise_metadata_schedule_180_e3154 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let noise_metadata_schedule_180_e3159: f64 = (params.p117 * (ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_180_e3160: f64 = (noise_metadata_schedule_180_e3156 + noise_metadata_schedule_180_e3159);
        let noise_metadata_schedule_180_e3162: f64 = (noise_metadata_schedule_180_e3160 + params.p118);
        (noise_metadata_schedule_180_e3162,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_180_e3164;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_181_e3175,) = {
    if ((noise_variable_389 != 0.0) && (!((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)))) {
        let noise_metadata_schedule_181_e3173: f64 = (params.p114 * (ctx.node_voltage(self.nodes[6]) - 0.0));
        (noise_metadata_schedule_181_e3173,)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_181_e3175;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_182_e3186,) = {
    if ((noise_variable_389 != 0.0) && (!((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)))) {
        let noise_metadata_schedule_182_e3184: f64 = (params.p115 * (ctx.node_voltage(self.nodes[6]) - 0.0));
        (noise_metadata_schedule_182_e3184,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_182_e3186;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_183_e3197,) = {
    if ((noise_variable_390 != 0.0) && (!(((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)))) {
        ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[1])),)
    } else {
        (noise_variable_147,)
    }
};
            noise_variable_147 = noise_metadata_schedule_183_e3197;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_184_e3216,) = {
    if ((noise_variable_390 != 0.0) && (!(((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)))) {
        let noise_metadata_schedule_184_e3210: f64 = (noise_variable_147 * params.p123);
        let noise_metadata_schedule_184_e3211: f64 = (1.0 + noise_metadata_schedule_184_e3210);
        let noise_metadata_schedule_184_e3212: f64 = (params.p124 / noise_metadata_schedule_184_e3211);
        let noise_metadata_schedule_184_e3214: f64 = (noise_metadata_schedule_184_e3212 * noise_variable_147);
        (noise_metadata_schedule_184_e3214,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_184_e3216;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_185_e3231,) = {
    if ((noise_variable_390 != 0.0) && (!(((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)))) {
        let noise_metadata_schedule_185_e3228: f64 = (noise_variable_147 - params.p127);
        let noise_metadata_schedule_185_e3229: f64 = (params.p125 * noise_metadata_schedule_185_e3228);
        (noise_metadata_schedule_185_e3229,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_185_e3231;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_187_e3280,) = {
    if ((noise_variable_390 != 0.0) && (!(((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)))) {
        let noise_metadata_schedule_187_e3271: f64 = (-2.0);
        let noise_metadata_schedule_187_e3274: f64 = ((ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[2])) - params.p10);
        let noise_metadata_schedule_187_e3275: f64 = (noise_metadata_schedule_187_e3271 * noise_metadata_schedule_187_e3274);
        let noise_metadata_schedule_187_e3277: f64 = (noise_metadata_schedule_187_e3275 / params.p122);
        let noise_metadata_schedule_187_e3278: f64 = (noise_metadata_schedule_187_e3277).exp();
        (noise_metadata_schedule_187_e3278,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_187_e3280;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_189_e3320,) = {
    if ((noise_variable_390 != 0.0) && (!(((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)))) {
        let noise_metadata_schedule_189_e3318: f64 = ((ctx.node_voltage(self.nodes[5]) - 0.0) / params.p121);
        (noise_metadata_schedule_189_e3318,)
    } else {
        (noise_variable_184,)
    }
};
            noise_variable_184 = noise_metadata_schedule_189_e3320;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_190_e3337,) = {
    if ((noise_variable_390 != 0.0) && (!(((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)))) {
        let noise_metadata_schedule_190_e3332: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_190_e3334: f64 = (noise_metadata_schedule_190_e3332).powf(params.p126);
        let noise_metadata_schedule_190_e3335: f64 = (noise_variable_184 * noise_metadata_schedule_190_e3334);
        (noise_metadata_schedule_190_e3335,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_190_e3337;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_191_e3351,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_191_e3349: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2]))).abs();
        (noise_metadata_schedule_191_e3349,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_191_e3351;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_193_e3387,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_193_e3385: f64 = ((ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[2]))).abs();
        (noise_metadata_schedule_193_e3385,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_193_e3387;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_195_e3425,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_195_e3422: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2]))).abs();
        let noise_metadata_schedule_195_e3423: f64 = ((ctx.node_voltage(self.nodes[12]) - 0.0) - noise_metadata_schedule_195_e3422);
        (noise_metadata_schedule_195_e3423,)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_195_e3425;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_196_e3457,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_196_e3439: f64 = noise_variable_337;
        let noise_metadata_schedule_196_e3442: f64 = noise_variable_337;
        let noise_metadata_schedule_196_e3445: f64 = noise_variable_337;
        let noise_metadata_schedule_196_e3446: f64 = (noise_metadata_schedule_196_e3442 * noise_metadata_schedule_196_e3445);
        let noise_metadata_schedule_196_e3449: f64 = (0.25 * 1e-30);
        let noise_metadata_schedule_196_e3451: f64 = (noise_metadata_schedule_196_e3449 * 1e-30);
        let noise_metadata_schedule_196_e3452: f64 = (noise_metadata_schedule_196_e3446 + noise_metadata_schedule_196_e3451);
        let noise_metadata_schedule_196_e3453: f64 = (noise_metadata_schedule_196_e3452).sqrt();
        let noise_metadata_schedule_196_e3454: f64 = (noise_metadata_schedule_196_e3439 + noise_metadata_schedule_196_e3453);
        let noise_metadata_schedule_196_e3455: f64 = (0.5 * noise_metadata_schedule_196_e3454);
        (noise_metadata_schedule_196_e3455,)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_196_e3457;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_197_e3473,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_197_e3470: f64 = ((ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[2]))).abs();
        let noise_metadata_schedule_197_e3471: f64 = ((ctx.node_voltage(self.nodes[14]) - 0.0) - noise_metadata_schedule_197_e3470);
        (noise_metadata_schedule_197_e3471,)
    } else {
        (noise_variable_342,)
    }
};
            noise_variable_342 = noise_metadata_schedule_197_e3473;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_198_e3505,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_198_e3487: f64 = noise_variable_342;
        let noise_metadata_schedule_198_e3490: f64 = noise_variable_342;
        let noise_metadata_schedule_198_e3493: f64 = noise_variable_342;
        let noise_metadata_schedule_198_e3494: f64 = (noise_metadata_schedule_198_e3490 * noise_metadata_schedule_198_e3493);
        let noise_metadata_schedule_198_e3497: f64 = (0.25 * 1e-30);
        let noise_metadata_schedule_198_e3499: f64 = (noise_metadata_schedule_198_e3497 * 1e-30);
        let noise_metadata_schedule_198_e3500: f64 = (noise_metadata_schedule_198_e3494 + noise_metadata_schedule_198_e3499);
        let noise_metadata_schedule_198_e3501: f64 = (noise_metadata_schedule_198_e3500).sqrt();
        let noise_metadata_schedule_198_e3502: f64 = (noise_metadata_schedule_198_e3487 + noise_metadata_schedule_198_e3501);
        let noise_metadata_schedule_198_e3503: f64 = (0.5 * noise_metadata_schedule_198_e3502);
        (noise_metadata_schedule_198_e3503,)
    } else {
        (noise_variable_342,)
    }
};
            noise_variable_342 = noise_metadata_schedule_198_e3505;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_199_e3520,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_199_e3518: f64 = (noise_variable_337 * params.p89);
        (noise_metadata_schedule_199_e3518,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_199_e3520;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_200_e3540,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_200_e3533: f64 = (noise_variable_337 * noise_variable_337);
        let noise_metadata_schedule_200_e3536: f64 = (params.p89 * params.p89);
        let noise_metadata_schedule_200_e3537: f64 = (noise_metadata_schedule_200_e3533 + noise_metadata_schedule_200_e3536);
        let noise_metadata_schedule_200_e3538: f64 = (noise_metadata_schedule_200_e3537).sqrt();
        (noise_metadata_schedule_200_e3538,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_200_e3540;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_201_e3560,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_201_e3553: f64 = (params.p91 * params.p10);
        let noise_metadata_schedule_201_e3554: f64 = (noise_metadata_schedule_201_e3553).abs();
        let noise_metadata_schedule_201_e3557: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_201_e3558: f64 = (noise_metadata_schedule_201_e3554 * noise_metadata_schedule_201_e3557);
        (noise_metadata_schedule_201_e3558,)
    } else {
        (noise_variable_339,)
    }
};
            noise_variable_339 = noise_metadata_schedule_201_e3560;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_202_e3575,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_202_e3573: f64 = (noise_variable_342 * params.p90);
        (noise_metadata_schedule_202_e3573,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_202_e3575;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_203_e3595,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_203_e3588: f64 = (noise_variable_342 * noise_variable_342);
        let noise_metadata_schedule_203_e3591: f64 = (params.p90 * params.p90);
        let noise_metadata_schedule_203_e3592: f64 = (noise_metadata_schedule_203_e3588 + noise_metadata_schedule_203_e3591);
        let noise_metadata_schedule_203_e3593: f64 = (noise_metadata_schedule_203_e3592).sqrt();
        (noise_metadata_schedule_203_e3593,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_203_e3595;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_204_e3615,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_204_e3608: f64 = (params.p92 * params.p10);
        let noise_metadata_schedule_204_e3609: f64 = (noise_metadata_schedule_204_e3608).abs();
        let noise_metadata_schedule_204_e3612: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_204_e3613: f64 = (noise_metadata_schedule_204_e3609 * noise_metadata_schedule_204_e3612);
        (noise_metadata_schedule_204_e3613,)
    } else {
        (noise_variable_344,)
    }
};
            noise_variable_344 = noise_metadata_schedule_204_e3615;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_205_e3630,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_205_e3628: f64 = (noise_variable_342 * params.p90);
        (noise_metadata_schedule_205_e3628,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_205_e3630;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_206_e3650,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_206_e3643: f64 = (noise_variable_342 * noise_variable_342);
        let noise_metadata_schedule_206_e3646: f64 = (params.p90 * params.p90);
        let noise_metadata_schedule_206_e3647: f64 = (noise_metadata_schedule_206_e3643 + noise_metadata_schedule_206_e3646);
        let noise_metadata_schedule_206_e3648: f64 = (noise_metadata_schedule_206_e3647).sqrt();
        (noise_metadata_schedule_206_e3648,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_206_e3650;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_207_e3670,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_207_e3663: f64 = (params.p93 * params.p13);
        let noise_metadata_schedule_207_e3664: f64 = (noise_metadata_schedule_207_e3663).abs();
        let noise_metadata_schedule_207_e3667: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_207_e3668: f64 = (noise_metadata_schedule_207_e3664 * noise_metadata_schedule_207_e3667);
        (noise_metadata_schedule_207_e3668,)
    } else {
        (noise_variable_345,)
    }
};
            noise_variable_345 = noise_metadata_schedule_207_e3670;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_208_e3685,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_208_e3683: f64 = (noise_variable_342 * params.p90);
        (noise_metadata_schedule_208_e3683,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_208_e3685;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_209_e3705,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_209_e3698: f64 = (noise_variable_342 * noise_variable_342);
        let noise_metadata_schedule_209_e3701: f64 = (params.p90 * params.p90);
        let noise_metadata_schedule_209_e3702: f64 = (noise_metadata_schedule_209_e3698 + noise_metadata_schedule_209_e3701);
        let noise_metadata_schedule_209_e3703: f64 = (noise_metadata_schedule_209_e3702).sqrt();
        (noise_metadata_schedule_209_e3703,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_209_e3705;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_210_e3725,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_210_e3718: f64 = (params.p94 * params.p17);
        let noise_metadata_schedule_210_e3719: f64 = (noise_metadata_schedule_210_e3718).abs();
        let noise_metadata_schedule_210_e3722: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_210_e3723: f64 = (noise_metadata_schedule_210_e3719 * noise_metadata_schedule_210_e3722);
        (noise_metadata_schedule_210_e3723,)
    } else {
        (noise_variable_346,)
    }
};
            noise_variable_346 = noise_metadata_schedule_210_e3725;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_211_e3740,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_211_e3738: f64 = (noise_variable_337 * params.p89);
        (noise_metadata_schedule_211_e3738,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_211_e3740;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_212_e3760,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_212_e3753: f64 = (noise_variable_337 * noise_variable_337);
        let noise_metadata_schedule_212_e3756: f64 = (params.p89 * params.p89);
        let noise_metadata_schedule_212_e3757: f64 = (noise_metadata_schedule_212_e3753 + noise_metadata_schedule_212_e3756);
        let noise_metadata_schedule_212_e3758: f64 = (noise_metadata_schedule_212_e3757).sqrt();
        (noise_metadata_schedule_212_e3758,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_212_e3760;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_213_e3780,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_213_e3773: f64 = (params.p95 * params.p36);
        let noise_metadata_schedule_213_e3774: f64 = (noise_metadata_schedule_213_e3773).abs();
        let noise_metadata_schedule_213_e3777: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_213_e3778: f64 = (noise_metadata_schedule_213_e3774 * noise_metadata_schedule_213_e3777);
        (noise_metadata_schedule_213_e3778,)
    } else {
        (noise_variable_340,)
    }
};
            noise_variable_340 = noise_metadata_schedule_213_e3780;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_214_e3795,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_214_e3793: f64 = (noise_variable_337 * params.p89);
        (noise_metadata_schedule_214_e3793,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_214_e3795;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_215_e3815,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_215_e3808: f64 = (noise_variable_337 * noise_variable_337);
        let noise_metadata_schedule_215_e3811: f64 = (params.p89 * params.p89);
        let noise_metadata_schedule_215_e3812: f64 = (noise_metadata_schedule_215_e3808 + noise_metadata_schedule_215_e3811);
        let noise_metadata_schedule_215_e3813: f64 = (noise_metadata_schedule_215_e3812).sqrt();
        (noise_metadata_schedule_215_e3813,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_215_e3815;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_216_e3835,) = {
    if ((noise_variable_391 != 0.0) && (!((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)))) {
        let noise_metadata_schedule_216_e3828: f64 = (params.p96 * params.p37);
        let noise_metadata_schedule_216_e3829: f64 = (noise_metadata_schedule_216_e3828).abs();
        let noise_metadata_schedule_216_e3832: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_216_e3833: f64 = (noise_metadata_schedule_216_e3829 * noise_metadata_schedule_216_e3832);
        (noise_metadata_schedule_216_e3833,)
    } else {
        (noise_variable_341,)
    }
};
            noise_variable_341 = noise_metadata_schedule_216_e3835;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_221_e3970,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        ((ctx.node_voltage(self.nodes[5]) - 0.0),)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_221_e3970;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_222_e3985,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        ((ctx.node_voltage(self.nodes[6]) - 0.0),)
    } else {
        (noise_variable_364,)
    }
};
            noise_variable_364 = noise_metadata_schedule_222_e3985;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_223_e4002,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_223_e4000: f64 = (noise_variable_337 * params.p89);
        (noise_metadata_schedule_223_e4000,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_223_e4002;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_224_e4024,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_224_e4017: f64 = (noise_variable_337 * noise_variable_337);
        let noise_metadata_schedule_224_e4020: f64 = (params.p89 * params.p89);
        let noise_metadata_schedule_224_e4021: f64 = (noise_metadata_schedule_224_e4017 + noise_metadata_schedule_224_e4020);
        let noise_metadata_schedule_224_e4022: f64 = (noise_metadata_schedule_224_e4021).sqrt();
        (noise_metadata_schedule_224_e4022,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_224_e4024;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_225_e4046,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_225_e4039: f64 = (params.p91 * params.p10);
        let noise_metadata_schedule_225_e4040: f64 = (noise_metadata_schedule_225_e4039).abs();
        let noise_metadata_schedule_225_e4043: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_225_e4044: f64 = (noise_metadata_schedule_225_e4040 * noise_metadata_schedule_225_e4043);
        (noise_metadata_schedule_225_e4044,)
    } else {
        (noise_variable_339,)
    }
};
            noise_variable_339 = noise_metadata_schedule_225_e4046;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_226_e4063,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_226_e4061: f64 = (noise_variable_337 * params.p89);
        (noise_metadata_schedule_226_e4061,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_226_e4063;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_227_e4085,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_227_e4078: f64 = (noise_variable_337 * noise_variable_337);
        let noise_metadata_schedule_227_e4081: f64 = (params.p89 * params.p89);
        let noise_metadata_schedule_227_e4082: f64 = (noise_metadata_schedule_227_e4078 + noise_metadata_schedule_227_e4081);
        let noise_metadata_schedule_227_e4083: f64 = (noise_metadata_schedule_227_e4082).sqrt();
        (noise_metadata_schedule_227_e4083,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_227_e4085;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_228_e4107,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_228_e4100: f64 = (params.p95 * params.p36);
        let noise_metadata_schedule_228_e4101: f64 = (noise_metadata_schedule_228_e4100).abs();
        let noise_metadata_schedule_228_e4104: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_228_e4105: f64 = (noise_metadata_schedule_228_e4101 * noise_metadata_schedule_228_e4104);
        (noise_metadata_schedule_228_e4105,)
    } else {
        (noise_variable_340,)
    }
};
            noise_variable_340 = noise_metadata_schedule_228_e4107;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_229_e4124,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_229_e4122: f64 = (noise_variable_337 * params.p89);
        (noise_metadata_schedule_229_e4122,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_229_e4124;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_230_e4146,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_230_e4139: f64 = (noise_variable_337 * noise_variable_337);
        let noise_metadata_schedule_230_e4142: f64 = (params.p89 * params.p89);
        let noise_metadata_schedule_230_e4143: f64 = (noise_metadata_schedule_230_e4139 + noise_metadata_schedule_230_e4142);
        let noise_metadata_schedule_230_e4144: f64 = (noise_metadata_schedule_230_e4143).sqrt();
        (noise_metadata_schedule_230_e4144,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_230_e4146;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_231_e4168,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_231_e4161: f64 = (params.p96 * params.p37);
        let noise_metadata_schedule_231_e4162: f64 = (noise_metadata_schedule_231_e4161).abs();
        let noise_metadata_schedule_231_e4165: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_231_e4166: f64 = (noise_metadata_schedule_231_e4162 * noise_metadata_schedule_231_e4165);
        (noise_metadata_schedule_231_e4166,)
    } else {
        (noise_variable_341,)
    }
};
            noise_variable_341 = noise_metadata_schedule_231_e4168;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_232_e4185,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_232_e4183: f64 = (noise_variable_364 * params.p90);
        (noise_metadata_schedule_232_e4183,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_232_e4185;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_233_e4207,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_233_e4200: f64 = (noise_variable_364 * noise_variable_364);
        let noise_metadata_schedule_233_e4203: f64 = (params.p90 * params.p90);
        let noise_metadata_schedule_233_e4204: f64 = (noise_metadata_schedule_233_e4200 + noise_metadata_schedule_233_e4203);
        let noise_metadata_schedule_233_e4205: f64 = (noise_metadata_schedule_233_e4204).sqrt();
        (noise_metadata_schedule_233_e4205,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_233_e4207;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_234_e4229,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_234_e4222: f64 = (params.p92 * params.p10);
        let noise_metadata_schedule_234_e4223: f64 = (noise_metadata_schedule_234_e4222).abs();
        let noise_metadata_schedule_234_e4226: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_234_e4227: f64 = (noise_metadata_schedule_234_e4223 * noise_metadata_schedule_234_e4226);
        (noise_metadata_schedule_234_e4227,)
    } else {
        (noise_variable_344,)
    }
};
            noise_variable_344 = noise_metadata_schedule_234_e4229;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_235_e4246,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_235_e4244: f64 = (noise_variable_364 * params.p90);
        (noise_metadata_schedule_235_e4244,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_235_e4246;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_236_e4268,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_236_e4261: f64 = (noise_variable_364 * noise_variable_364);
        let noise_metadata_schedule_236_e4264: f64 = (params.p90 * params.p90);
        let noise_metadata_schedule_236_e4265: f64 = (noise_metadata_schedule_236_e4261 + noise_metadata_schedule_236_e4264);
        let noise_metadata_schedule_236_e4266: f64 = (noise_metadata_schedule_236_e4265).sqrt();
        (noise_metadata_schedule_236_e4266,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_236_e4268;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_237_e4290,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_237_e4283: f64 = (params.p147 * params.p36);
        let noise_metadata_schedule_237_e4284: f64 = (noise_metadata_schedule_237_e4283).abs();
        let noise_metadata_schedule_237_e4287: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_237_e4288: f64 = (noise_metadata_schedule_237_e4284 * noise_metadata_schedule_237_e4287);
        (noise_metadata_schedule_237_e4288,)
    } else {
        (noise_variable_365,)
    }
};
            noise_variable_365 = noise_metadata_schedule_237_e4290;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_238_e4307,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_238_e4305: f64 = (noise_variable_364 * params.p90);
        (noise_metadata_schedule_238_e4305,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_238_e4307;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_239_e4329,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_239_e4322: f64 = (noise_variable_364 * noise_variable_364);
        let noise_metadata_schedule_239_e4325: f64 = (params.p90 * params.p90);
        let noise_metadata_schedule_239_e4326: f64 = (noise_metadata_schedule_239_e4322 + noise_metadata_schedule_239_e4325);
        let noise_metadata_schedule_239_e4327: f64 = (noise_metadata_schedule_239_e4326).sqrt();
        (noise_metadata_schedule_239_e4327,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_239_e4329;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_240_e4351,) = {
    if ((noise_variable_392 != 0.0) && (!(((((noise_variable_387 != 0.0) || (noise_variable_388 != 0.0)) || (noise_variable_389 != 0.0)) || (noise_variable_390 != 0.0)) || (noise_variable_391 != 0.0)))) {
        let noise_metadata_schedule_240_e4344: f64 = (params.p148 * params.p37);
        let noise_metadata_schedule_240_e4345: f64 = (noise_metadata_schedule_240_e4344).abs();
        let noise_metadata_schedule_240_e4348: f64 = (noise_variable_136 / noise_variable_90);
        let noise_metadata_schedule_240_e4349: f64 = (noise_metadata_schedule_240_e4345 * noise_metadata_schedule_240_e4348);
        (noise_metadata_schedule_240_e4349,)
    } else {
        (noise_variable_366,)
    }
};
            noise_variable_366 = noise_metadata_schedule_240_e4351;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_241_e4354: f64 = (params.p9 / params.p1);
            noise_variable_80 = noise_metadata_schedule_241_e4354;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_242_e4357: f64 = (params.p9 / params.p2);
            noise_variable_81 = noise_metadata_schedule_242_e4357;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_243_e4360: f64 = (1.0 + params.p26);
            let noise_metadata_schedule_243_e4363: f64 = (params.p27 + noise_variable_211);
            let noise_metadata_schedule_243_e4365: f64 = (noise_metadata_schedule_243_e4363 * noise_variable_140);
            let noise_metadata_schedule_243_e4366: f64 = (noise_metadata_schedule_243_e4360 + noise_metadata_schedule_243_e4365);
            noise_variable_146 = noise_metadata_schedule_243_e4366;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_244_e4369: f64 = (8.617087e-5 * noise_variable_82);
            let noise_metadata_schedule_244_e4371: f64 = (noise_metadata_schedule_244_e4369 * noise_variable_146);
            noise_variable_83 = noise_metadata_schedule_244_e4371;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_245_e4374: f64 = (params.p10 + noise_variable_339);
            let noise_metadata_schedule_245_e4376: f64 = (noise_metadata_schedule_245_e4374 + noise_variable_344);
            let noise_metadata_schedule_245_e4379: f64 = (params.p22 + noise_variable_212);
            let noise_metadata_schedule_245_e4381: f64 = (noise_metadata_schedule_245_e4379 - noise_variable_216);
            let noise_metadata_schedule_245_e4384: f64 = (noise_variable_140 * params.p23);
            let noise_metadata_schedule_245_e4385: f64 = (noise_metadata_schedule_245_e4381 * noise_metadata_schedule_245_e4384);
            let noise_metadata_schedule_245_e4388: f64 = (noise_variable_140 * noise_variable_140);
            let noise_metadata_schedule_245_e4391: f64 = (params.p23 * params.p23);
            let noise_metadata_schedule_245_e4392: f64 = (noise_metadata_schedule_245_e4388 + noise_metadata_schedule_245_e4391);
            let noise_metadata_schedule_245_e4393: f64 = (noise_metadata_schedule_245_e4392).sqrt();
            let noise_metadata_schedule_245_e4394: f64 = (noise_metadata_schedule_245_e4385 / noise_metadata_schedule_245_e4393);
            let noise_metadata_schedule_245_e4395: f64 = (noise_metadata_schedule_245_e4376 - noise_metadata_schedule_245_e4394);
            noise_variable_87 = noise_metadata_schedule_245_e4395;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_246_e4398: f64 = (noise_variable_82 / noise_variable_35);
            noise_variable_334 = noise_metadata_schedule_246_e4398;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_248_e4409: f64 = (noise_variable_334 - 1.0);
            let noise_metadata_schedule_248_e4411: f64 = (noise_metadata_schedule_248_e4409 * params.p24);
            let noise_metadata_schedule_248_e4412: f64 = (noise_variable_87 - noise_metadata_schedule_248_e4411);
            let noise_metadata_schedule_248_e4414: f64 = (noise_metadata_schedule_248_e4412 + noise_variable_209);
            let noise_metadata_schedule_248_e4416: f64 = (noise_metadata_schedule_248_e4414 + noise_variable_213);
            let noise_metadata_schedule_248_e4420: f64 = (noise_variable_81 + noise_variable_80);
            let noise_metadata_schedule_248_e4421: f64 = (noise_variable_81 / noise_metadata_schedule_248_e4420);
            let noise_metadata_schedule_248_e4423: f64 = (noise_metadata_schedule_248_e4421 * params.p11);
            let noise_metadata_schedule_248_e4425: f64 = (noise_metadata_schedule_248_e4423 * noise_variable_45);
            let noise_metadata_schedule_248_e4426: f64 = (noise_metadata_schedule_248_e4416 + noise_metadata_schedule_248_e4425);
            noise_variable_88 = noise_metadata_schedule_248_e4426;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_249_e4430: f64 = (2.0 * params.p4);
            let noise_metadata_schedule_249_e4432: f64 = (noise_metadata_schedule_249_e4430 * 1.602176634e-19);
            let noise_metadata_schedule_249_e4434: f64 = (noise_metadata_schedule_249_e4432 * 3.24e17);
            let noise_metadata_schedule_249_e4436: f64 = (noise_metadata_schedule_249_e4434 * noise_variable_83);
            let noise_metadata_schedule_249_e4438: f64 = (noise_metadata_schedule_249_e4436 * noise_variable_83);
            let noise_metadata_schedule_249_e4439: f64 = (params.p3 / noise_metadata_schedule_249_e4438);
            noise_variable_136 = noise_metadata_schedule_249_e4439;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_250_e4444: f64 = (noise_variable_136 * params.p30);
            let noise_metadata_schedule_250_e4445: f64 = (noise_metadata_schedule_250_e4444).ln();
            let noise_metadata_schedule_250_e4446: f64 = (noise_variable_83 * noise_metadata_schedule_250_e4445);
            let noise_metadata_schedule_250_e4447: f64 = (noise_variable_88 + noise_metadata_schedule_250_e4446);
            noise_variable_159 = noise_metadata_schedule_250_e4447;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_251_e4451: f64 = (noise_variable_40 - noise_variable_159);
            let noise_metadata_schedule_251_e4454: f64 = (noise_variable_40 - noise_variable_159);
            let noise_metadata_schedule_251_e4457: f64 = (noise_variable_40 - noise_variable_159);
            let noise_metadata_schedule_251_e4458: f64 = (noise_metadata_schedule_251_e4454 * noise_metadata_schedule_251_e4457);
            let noise_metadata_schedule_251_e4460: f64 = (noise_metadata_schedule_251_e4458 + 0.0001);
            let noise_metadata_schedule_251_e4461: f64 = (noise_metadata_schedule_251_e4460).sqrt();
            let noise_metadata_schedule_251_e4462: f64 = (noise_metadata_schedule_251_e4451 + noise_metadata_schedule_251_e4461);
            let noise_metadata_schedule_251_e4463: f64 = (0.5 * noise_metadata_schedule_251_e4462);
            let noise_metadata_schedule_251_e4465: f64 = (noise_metadata_schedule_251_e4463 + noise_variable_159);
            noise_variable_160 = noise_metadata_schedule_251_e4465;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_252_e4468: f64 = (noise_variable_160 - noise_variable_88);
            noise_variable_37 = noise_metadata_schedule_252_e4468;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_253_e4472: f64 = (1.602176634e-19 * 3.24e17);
            let noise_metadata_schedule_253_e4474: f64 = (noise_metadata_schedule_253_e4472 * noise_variable_83);
            let noise_metadata_schedule_253_e4475: f64 = (noise_variable_80 / noise_metadata_schedule_253_e4474);
            noise_variable_84 = noise_metadata_schedule_253_e4475;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_254_e4478: f64 = (2.718281828459045 / noise_variable_84);
            noise_variable_150 = noise_metadata_schedule_254_e4478;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_255_e4481: f64 = (1.0 / noise_variable_84);
            noise_variable_151 = noise_metadata_schedule_255_e4481;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_256_e4484: f64 = (noise_variable_80 / 1.602176634e-19);
            noise_variable_99 = noise_metadata_schedule_256_e4484;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_257_e4487: f64 = (0.5 * noise_variable_37);
            let noise_metadata_schedule_257_e4491: f64 = (noise_variable_37 * noise_variable_37);
            let noise_metadata_schedule_257_e4494: f64 = (4.0 * 0.3);
            let noise_metadata_schedule_257_e4496: f64 = (noise_metadata_schedule_257_e4494 * 0.3);
            let noise_metadata_schedule_257_e4497: f64 = (noise_metadata_schedule_257_e4491 + noise_metadata_schedule_257_e4496);
            let noise_metadata_schedule_257_e4498: f64 = (noise_metadata_schedule_257_e4497).sqrt();
            let noise_metadata_schedule_257_e4499: f64 = (0.5 * noise_metadata_schedule_257_e4498);
            let noise_metadata_schedule_257_e4500: f64 = (noise_metadata_schedule_257_e4487 + noise_metadata_schedule_257_e4499);
            noise_variable_154 = noise_metadata_schedule_257_e4500;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_258_e4503: f64 = (noise_variable_154 * noise_variable_150);
            let noise_metadata_schedule_258_e4506: f64 = (noise_variable_154 * noise_variable_154);
            let noise_metadata_schedule_258_e4509: f64 = (noise_variable_150 * noise_variable_150);
            let noise_metadata_schedule_258_e4510: f64 = (noise_metadata_schedule_258_e4506 + noise_metadata_schedule_258_e4509);
            let noise_metadata_schedule_258_e4511: f64 = (noise_metadata_schedule_258_e4510).sqrt();
            let noise_metadata_schedule_258_e4512: f64 = (noise_metadata_schedule_258_e4503 / noise_metadata_schedule_258_e4511);
            noise_variable_155 = noise_metadata_schedule_258_e4512;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_259_e4515: f64 = (noise_variable_154 * noise_variable_151);
            let noise_metadata_schedule_259_e4518: f64 = (noise_variable_154 * noise_variable_154);
            let noise_metadata_schedule_259_e4521: f64 = (noise_variable_151 * noise_variable_151);
            let noise_metadata_schedule_259_e4522: f64 = (noise_metadata_schedule_259_e4518 + noise_metadata_schedule_259_e4521);
            let noise_metadata_schedule_259_e4523: f64 = (noise_metadata_schedule_259_e4522).sqrt();
            let noise_metadata_schedule_259_e4524: f64 = (noise_metadata_schedule_259_e4515 / noise_metadata_schedule_259_e4523);
            noise_variable_130 = noise_metadata_schedule_259_e4524;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_260_e4530: f64 = (noise_variable_84 * noise_variable_155);
            let noise_metadata_schedule_260_e4531: f64 = (noise_metadata_schedule_260_e4530).ln();
            let noise_metadata_schedule_260_e4532: f64 = (1.0 - noise_metadata_schedule_260_e4531);
            let noise_metadata_schedule_260_e4533: f64 = (noise_variable_83 * noise_metadata_schedule_260_e4532);
            let noise_metadata_schedule_260_e4534: f64 = (noise_variable_154 + noise_metadata_schedule_260_e4533);
            let noise_metadata_schedule_260_e4537: f64 = (params.p28 / 3.0);
            let noise_metadata_schedule_260_e4540: f64 = (noise_variable_99 * noise_variable_154);
            let noise_metadata_schedule_260_e4542: f64 = (noise_metadata_schedule_260_e4540).powf(0.6666666666666666);
            let noise_metadata_schedule_260_e4543: f64 = (noise_metadata_schedule_260_e4537 * noise_metadata_schedule_260_e4542);
            let noise_metadata_schedule_260_e4544: f64 = (noise_metadata_schedule_260_e4534 - noise_metadata_schedule_260_e4543);
            let noise_metadata_schedule_260_e4549: f64 = (noise_variable_83 / noise_variable_130);
            let noise_metadata_schedule_260_e4550: f64 = (1.0 + noise_metadata_schedule_260_e4549);
            let noise_metadata_schedule_260_e4551: f64 = (noise_variable_154 * noise_metadata_schedule_260_e4550);
            let noise_metadata_schedule_260_e4554: f64 = (2.0 * params.p28);
            let noise_metadata_schedule_260_e4556: f64 = (noise_metadata_schedule_260_e4554 / 3.0);
            let noise_metadata_schedule_260_e4559: f64 = (noise_variable_99 * noise_variable_154);
            let noise_metadata_schedule_260_e4561: f64 = (noise_metadata_schedule_260_e4559).powf(0.6666666666666666);
            let noise_metadata_schedule_260_e4562: f64 = (noise_metadata_schedule_260_e4556 * noise_metadata_schedule_260_e4561);
            let noise_metadata_schedule_260_e4563: f64 = (noise_metadata_schedule_260_e4551 + noise_metadata_schedule_260_e4562);
            let noise_metadata_schedule_260_e4564: f64 = (noise_metadata_schedule_260_e4544 / noise_metadata_schedule_260_e4563);
            noise_variable_152 = noise_metadata_schedule_260_e4564;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_261_e4568: f64 = (2.0 * noise_variable_83);
            let noise_metadata_schedule_261_e4569: f64 = (noise_variable_37 / noise_metadata_schedule_261_e4568);
            noise_variable_136 = noise_metadata_schedule_261_e4569;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_262_e4572: f64 = if noise_variable_136 < 200.0 { 1.0 } else { 0.0 };
            noise_variable_393 = noise_metadata_schedule_262_e4572;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_263_e4579,) = {
    if (noise_variable_393 != 0.0) {
        let noise_metadata_schedule_263_e4576: f64 = (noise_variable_136 / 4.0);
        let noise_metadata_schedule_263_e4577: f64 = { let limited_exp_arg = noise_metadata_schedule_263_e4576; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (noise_metadata_schedule_263_e4577,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_263_e4579;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_264_e4589,) = {
    if (noise_variable_393 != 0.0) {
        let noise_metadata_schedule_264_e4582: f64 = (-3.0);
        let noise_metadata_schedule_264_e4584: f64 = (noise_metadata_schedule_264_e4582 * noise_variable_136);
        let noise_metadata_schedule_264_e4586: f64 = (noise_metadata_schedule_264_e4584 / 4.0);
        let noise_metadata_schedule_264_e4587: f64 = { let limited_exp_arg = noise_metadata_schedule_264_e4586; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (noise_metadata_schedule_264_e4587,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_264_e4589;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_265_e4626,) = {
    if (noise_variable_393 != 0.0) {
        let noise_metadata_schedule_265_e4593: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_265_e4595: f64 = (noise_metadata_schedule_265_e4593 * noise_variable_99);
        let noise_metadata_schedule_265_e4598: f64 = (3.0 * noise_variable_136);
        let noise_metadata_schedule_265_e4600: f64 = (noise_metadata_schedule_265_e4598 / 4.0);
        let noise_metadata_schedule_265_e4603: f64 = (noise_variable_90 + noise_variable_91);
        let noise_metadata_schedule_265_e4604: f64 = (noise_metadata_schedule_265_e4603).ln();
        let noise_metadata_schedule_265_e4605: f64 = (noise_metadata_schedule_265_e4600 + noise_metadata_schedule_265_e4604);
        let noise_metadata_schedule_265_e4606: f64 = (noise_metadata_schedule_265_e4595 * noise_metadata_schedule_265_e4605);
        let noise_metadata_schedule_265_e4609: f64 = (1.0 / noise_variable_152);
        let noise_metadata_schedule_265_e4612: f64 = (noise_variable_99 / 3.24e17);
        let noise_metadata_schedule_265_e4614: f64 = (-1.0);
        let noise_metadata_schedule_265_e4616: f64 = (noise_metadata_schedule_265_e4614 * noise_variable_37);
        let noise_metadata_schedule_265_e4619: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_265_e4620: f64 = (noise_metadata_schedule_265_e4616 / noise_metadata_schedule_265_e4619);
        let noise_metadata_schedule_265_e4621: f64 = { let limited_exp_arg = noise_metadata_schedule_265_e4620; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_265_e4622: f64 = (noise_metadata_schedule_265_e4612 * noise_metadata_schedule_265_e4621);
        let noise_metadata_schedule_265_e4623: f64 = (noise_metadata_schedule_265_e4609 + noise_metadata_schedule_265_e4622);
        let noise_metadata_schedule_265_e4624: f64 = (noise_metadata_schedule_265_e4606 / noise_metadata_schedule_265_e4623);
        (noise_metadata_schedule_265_e4624,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_265_e4626;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_266_e4659,) = {
    if (noise_variable_393 == 0.0) {
        let noise_metadata_schedule_266_e4631: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_266_e4633: f64 = (noise_metadata_schedule_266_e4631 * noise_variable_99);
        let noise_metadata_schedule_266_e4636: f64 = noise_variable_136;
        let noise_metadata_schedule_266_e4638: f64 = noise_metadata_schedule_266_e4636;
        let noise_metadata_schedule_266_e4639: f64 = (noise_metadata_schedule_266_e4633 * noise_metadata_schedule_266_e4638);
        let noise_metadata_schedule_266_e4642: f64 = (1.0 / noise_variable_152);
        let noise_metadata_schedule_266_e4645: f64 = (noise_variable_99 / 3.24e17);
        let noise_metadata_schedule_266_e4647: f64 = (-1.0);
        let noise_metadata_schedule_266_e4649: f64 = (noise_metadata_schedule_266_e4647 * noise_variable_37);
        let noise_metadata_schedule_266_e4652: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_266_e4653: f64 = (noise_metadata_schedule_266_e4649 / noise_metadata_schedule_266_e4652);
        let noise_metadata_schedule_266_e4654: f64 = { let limited_exp_arg = noise_metadata_schedule_266_e4653; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_266_e4655: f64 = (noise_metadata_schedule_266_e4645 * noise_metadata_schedule_266_e4654);
        let noise_metadata_schedule_266_e4656: f64 = (noise_metadata_schedule_266_e4642 + noise_metadata_schedule_266_e4655);
        let noise_metadata_schedule_266_e4657: f64 = (noise_metadata_schedule_266_e4639 / noise_metadata_schedule_266_e4656);
        (noise_metadata_schedule_266_e4657,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_266_e4659;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_267_e4663: f64 = (noise_variable_153 / noise_variable_99);
            let noise_metadata_schedule_267_e4664: f64 = (noise_variable_37 - noise_metadata_schedule_267_e4663);
            noise_variable_100 = noise_metadata_schedule_267_e4664;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_268_e4667: f64 = (noise_variable_100 - noise_variable_37);
            let noise_metadata_schedule_268_e4668: f64 = (noise_metadata_schedule_268_e4667).abs();
            let noise_metadata_schedule_268_e4670: f64 = if noise_metadata_schedule_268_e4668 > 1e-19 { 1.0 } else { 0.0 };
            noise_variable_394 = noise_metadata_schedule_268_e4670;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_269_e4676,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_269_e4674: f64 = (noise_variable_37 - noise_variable_100);
        (noise_metadata_schedule_269_e4674,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_269_e4676;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_270_e4695,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_270_e4680: f64 = (0.5 * noise_variable_101);
        let noise_metadata_schedule_270_e4684: f64 = (noise_variable_101 * noise_variable_101);
        let noise_metadata_schedule_270_e4687: f64 = (4.0 * 1e-9);
        let noise_metadata_schedule_270_e4689: f64 = (noise_metadata_schedule_270_e4687 * 1e-9);
        let noise_metadata_schedule_270_e4690: f64 = (noise_metadata_schedule_270_e4684 + noise_metadata_schedule_270_e4689);
        let noise_metadata_schedule_270_e4691: f64 = (noise_metadata_schedule_270_e4690).sqrt();
        let noise_metadata_schedule_270_e4692: f64 = (0.5 * noise_metadata_schedule_270_e4691);
        let noise_metadata_schedule_270_e4693: f64 = (noise_metadata_schedule_270_e4680 + noise_metadata_schedule_270_e4692);
        (noise_metadata_schedule_270_e4693,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_270_e4695;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_271_e4701,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_271_e4699: f64 = (noise_variable_99).powf(0.6666666666666666);
        (noise_metadata_schedule_271_e4699,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_271_e4701;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_272_e4707,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_272_e4705: f64 = (noise_variable_101).powf(0.6666666666666666);
        (noise_metadata_schedule_272_e4705,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_272_e4707;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_273_e4714,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_273_e4711: f64 = (-0.3333333333333333);
        let noise_metadata_schedule_273_e4712: f64 = (noise_variable_101).powf(noise_metadata_schedule_273_e4711);
        (noise_metadata_schedule_273_e4712,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_273_e4714;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_274_e4722,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_274_e4718: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_274_e4720: f64 = (noise_metadata_schedule_274_e4718 * noise_variable_90);
        (noise_metadata_schedule_274_e4720,)
    } else {
        (noise_variable_102,)
    }
};
            noise_variable_102 = noise_metadata_schedule_274_e4722;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_275_e4730,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_275_e4726: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_275_e4728: f64 = (noise_metadata_schedule_275_e4726 * noise_variable_90);
        (noise_metadata_schedule_275_e4728,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_275_e4730;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_276_e4740,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_276_e4734: f64 = (noise_variable_100 / noise_variable_83);
        let noise_metadata_schedule_276_e4737: f64 = (noise_variable_102 / noise_variable_83);
        let noise_metadata_schedule_276_e4738: f64 = (noise_metadata_schedule_276_e4734 - noise_metadata_schedule_276_e4737);
        (noise_metadata_schedule_276_e4738,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_276_e4740;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_277_e4750,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_277_e4744: f64 = (noise_variable_100 / noise_variable_83);
        let noise_metadata_schedule_277_e4747: f64 = (noise_variable_103 / noise_variable_83);
        let noise_metadata_schedule_277_e4748: f64 = (noise_metadata_schedule_277_e4744 - noise_metadata_schedule_277_e4747);
        (noise_metadata_schedule_277_e4748,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_277_e4750;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_278_e4832,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_278_e4754: f64 = (noise_variable_99 * noise_variable_101);
        let noise_metadata_schedule_278_e4757: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_278_e4764: f64 = (-37.0);
        let (noise_metadata_schedule_278_e4790,) = {
            if ((!(noise_variable_104 >= 37.0)) && (!(noise_variable_104 <= noise_metadata_schedule_278_e4764))) {
                let noise_metadata_schedule_278_e4769: f64 = (noise_variable_104).exp();
                let noise_metadata_schedule_278_e4771: f64 = (noise_metadata_schedule_278_e4769 + 1.0);
                let noise_metadata_schedule_278_e4772: f64 = (noise_metadata_schedule_278_e4771).ln();
                (noise_metadata_schedule_278_e4772,)
            } else {
                let noise_metadata_schedule_278_e4779: f64 = (-37.0);
                let (noise_metadata_schedule_278_e4789,) = {
                    if ((!(noise_variable_104 >= 37.0)) && (noise_variable_104 <= noise_metadata_schedule_278_e4779)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_278_e4788,) = {
                            if (noise_variable_104 >= 37.0) {
                                (noise_variable_104,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_278_e4788,)
                    }
                };
                (noise_metadata_schedule_278_e4789,)
            }
        };
        let noise_metadata_schedule_278_e4791: f64 = (noise_metadata_schedule_278_e4757 * noise_metadata_schedule_278_e4790);
        let noise_metadata_schedule_278_e4792: f64 = (noise_metadata_schedule_278_e4754 - noise_metadata_schedule_278_e4791);
        let noise_metadata_schedule_278_e4795: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_278_e4802: f64 = (-37.0);
        let (noise_metadata_schedule_278_e4828,) = {
            if ((!(noise_variable_105 >= 37.0)) && (!(noise_variable_105 <= noise_metadata_schedule_278_e4802))) {
                let noise_metadata_schedule_278_e4807: f64 = (noise_variable_105).exp();
                let noise_metadata_schedule_278_e4809: f64 = (noise_metadata_schedule_278_e4807 + 1.0);
                let noise_metadata_schedule_278_e4810: f64 = (noise_metadata_schedule_278_e4809).ln();
                (noise_metadata_schedule_278_e4810,)
            } else {
                let noise_metadata_schedule_278_e4817: f64 = (-37.0);
                let (noise_metadata_schedule_278_e4827,) = {
                    if ((!(noise_variable_105 >= 37.0)) && (noise_variable_105 <= noise_metadata_schedule_278_e4817)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_278_e4826,) = {
                            if (noise_variable_105 >= 37.0) {
                                (noise_variable_105,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_278_e4826,)
                    }
                };
                (noise_metadata_schedule_278_e4827,)
            }
        };
        let noise_metadata_schedule_278_e4829: f64 = (noise_metadata_schedule_278_e4795 * noise_metadata_schedule_278_e4828);
        let noise_metadata_schedule_278_e4830: f64 = (noise_metadata_schedule_278_e4792 - noise_metadata_schedule_278_e4829);
        (noise_metadata_schedule_278_e4830,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_278_e4832;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_279_e4840,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_279_e4836: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_279_e4838: f64 = (noise_metadata_schedule_279_e4836 * noise_variable_91);
        (noise_metadata_schedule_279_e4838,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_279_e4840;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_280_e4848,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_280_e4844: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_280_e4846: f64 = (noise_metadata_schedule_280_e4844 * noise_variable_91);
        (noise_metadata_schedule_280_e4846,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_280_e4848;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_281_e4861,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_281_e4851: f64 = { let limited_exp_arg = noise_variable_104; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_281_e4853: f64 = (noise_metadata_schedule_281_e4851 * 3.24e17);
        let noise_metadata_schedule_281_e4857: f64 = (0.6666666666666666 * noise_variable_107);
        let noise_metadata_schedule_281_e4858: f64 = (1.0 + noise_metadata_schedule_281_e4857);
        let noise_metadata_schedule_281_e4859: f64 = (noise_metadata_schedule_281_e4853 * noise_metadata_schedule_281_e4858);
        (noise_metadata_schedule_281_e4859,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_281_e4861;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_282_e4868,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_282_e4865: f64 = { let limited_exp_arg = noise_variable_104; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_282_e4866: f64 = (1.0 + noise_metadata_schedule_282_e4865);
        (noise_metadata_schedule_282_e4866,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_282_e4868;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_283_e4881,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_283_e4871: f64 = { let limited_exp_arg = noise_variable_105; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_283_e4873: f64 = (noise_metadata_schedule_283_e4871 * 3.24e17);
        let noise_metadata_schedule_283_e4877: f64 = (0.6666666666666666 * noise_variable_108);
        let noise_metadata_schedule_283_e4878: f64 = (1.0 + noise_metadata_schedule_283_e4877);
        let noise_metadata_schedule_283_e4879: f64 = (noise_metadata_schedule_283_e4873 * noise_metadata_schedule_283_e4878);
        (noise_metadata_schedule_283_e4879,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_283_e4881;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_284_e4888,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_284_e4885: f64 = { let limited_exp_arg = noise_variable_105; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_284_e4886: f64 = (1.0 + noise_metadata_schedule_284_e4885);
        (noise_metadata_schedule_284_e4886,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_284_e4888;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_285_e4903,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_285_e4891: f64 = (-1.0);
        let noise_metadata_schedule_285_e4893: f64 = (noise_metadata_schedule_285_e4891 * noise_variable_99);
        let noise_metadata_schedule_285_e4896: f64 = (noise_variable_109 / noise_variable_110);
        let noise_metadata_schedule_285_e4897: f64 = (noise_metadata_schedule_285_e4893 - noise_metadata_schedule_285_e4896);
        let noise_metadata_schedule_285_e4900: f64 = (noise_variable_111 / noise_variable_112);
        let noise_metadata_schedule_285_e4901: f64 = (noise_metadata_schedule_285_e4897 - noise_metadata_schedule_285_e4900);
        (noise_metadata_schedule_285_e4901,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_285_e4903;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_286_e4911,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_286_e4908: f64 = (noise_variable_106 / noise_variable_113);
        let noise_metadata_schedule_286_e4909: f64 = (noise_variable_100 - noise_metadata_schedule_286_e4908);
        (noise_metadata_schedule_286_e4909,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_286_e4911;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_287_e4917,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_287_e4915: f64 = (noise_variable_37 - noise_variable_114);
        (noise_metadata_schedule_287_e4915,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_287_e4917;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_288_e4936,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_288_e4921: f64 = (0.5 * noise_variable_115);
        let noise_metadata_schedule_288_e4925: f64 = (noise_variable_115 * noise_variable_115);
        let noise_metadata_schedule_288_e4928: f64 = (4.0 * 1e-9);
        let noise_metadata_schedule_288_e4930: f64 = (noise_metadata_schedule_288_e4928 * 1e-9);
        let noise_metadata_schedule_288_e4931: f64 = (noise_metadata_schedule_288_e4925 + noise_metadata_schedule_288_e4930);
        let noise_metadata_schedule_288_e4932: f64 = (noise_metadata_schedule_288_e4931).sqrt();
        let noise_metadata_schedule_288_e4933: f64 = (0.5 * noise_metadata_schedule_288_e4932);
        let noise_metadata_schedule_288_e4934: f64 = (noise_metadata_schedule_288_e4921 + noise_metadata_schedule_288_e4933);
        (noise_metadata_schedule_288_e4934,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_288_e4936;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_289_e4943,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_289_e4940: f64 = (-0.3333333333333333);
        let noise_metadata_schedule_289_e4941: f64 = (noise_variable_115).powf(noise_metadata_schedule_289_e4940);
        (noise_metadata_schedule_289_e4941,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_289_e4943;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_290_e4953,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_290_e4947: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_290_e4950: f64 = (noise_variable_115).powf(0.6666666666666666);
        let noise_metadata_schedule_290_e4951: f64 = (noise_metadata_schedule_290_e4947 * noise_metadata_schedule_290_e4950);
        (noise_metadata_schedule_290_e4951,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_290_e4953;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_291_e4963,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_291_e4957: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_291_e4960: f64 = (noise_variable_115).powf(0.6666666666666666);
        let noise_metadata_schedule_291_e4961: f64 = (noise_metadata_schedule_291_e4957 * noise_metadata_schedule_291_e4960);
        (noise_metadata_schedule_291_e4961,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_291_e4963;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_292_e4973,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_292_e4967: f64 = (noise_variable_114 / noise_variable_83);
        let noise_metadata_schedule_292_e4970: f64 = (noise_variable_116 / noise_variable_83);
        let noise_metadata_schedule_292_e4971: f64 = (noise_metadata_schedule_292_e4967 - noise_metadata_schedule_292_e4970);
        (noise_metadata_schedule_292_e4971,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_292_e4973;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_293_e4983,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_293_e4977: f64 = (noise_variable_114 / noise_variable_83);
        let noise_metadata_schedule_293_e4980: f64 = (noise_variable_117 / noise_variable_83);
        let noise_metadata_schedule_293_e4981: f64 = (noise_metadata_schedule_293_e4977 - noise_metadata_schedule_293_e4980);
        (noise_metadata_schedule_293_e4981,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_293_e4983;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_294_e5065,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_294_e4987: f64 = (noise_variable_99 * noise_variable_115);
        let noise_metadata_schedule_294_e4990: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_294_e4997: f64 = (-37.0);
        let (noise_metadata_schedule_294_e5023,) = {
            if ((!(noise_variable_118 >= 37.0)) && (!(noise_variable_118 <= noise_metadata_schedule_294_e4997))) {
                let noise_metadata_schedule_294_e5002: f64 = (noise_variable_118).exp();
                let noise_metadata_schedule_294_e5004: f64 = (noise_metadata_schedule_294_e5002 + 1.0);
                let noise_metadata_schedule_294_e5005: f64 = (noise_metadata_schedule_294_e5004).ln();
                (noise_metadata_schedule_294_e5005,)
            } else {
                let noise_metadata_schedule_294_e5012: f64 = (-37.0);
                let (noise_metadata_schedule_294_e5022,) = {
                    if ((!(noise_variable_118 >= 37.0)) && (noise_variable_118 <= noise_metadata_schedule_294_e5012)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_294_e5021,) = {
                            if (noise_variable_118 >= 37.0) {
                                (noise_variable_118,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_294_e5021,)
                    }
                };
                (noise_metadata_schedule_294_e5022,)
            }
        };
        let noise_metadata_schedule_294_e5024: f64 = (noise_metadata_schedule_294_e4990 * noise_metadata_schedule_294_e5023);
        let noise_metadata_schedule_294_e5025: f64 = (noise_metadata_schedule_294_e4987 - noise_metadata_schedule_294_e5024);
        let noise_metadata_schedule_294_e5028: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_294_e5035: f64 = (-37.0);
        let (noise_metadata_schedule_294_e5061,) = {
            if ((!(noise_variable_119 >= 37.0)) && (!(noise_variable_119 <= noise_metadata_schedule_294_e5035))) {
                let noise_metadata_schedule_294_e5040: f64 = (noise_variable_119).exp();
                let noise_metadata_schedule_294_e5042: f64 = (noise_metadata_schedule_294_e5040 + 1.0);
                let noise_metadata_schedule_294_e5043: f64 = (noise_metadata_schedule_294_e5042).ln();
                (noise_metadata_schedule_294_e5043,)
            } else {
                let noise_metadata_schedule_294_e5050: f64 = (-37.0);
                let (noise_metadata_schedule_294_e5060,) = {
                    if ((!(noise_variable_119 >= 37.0)) && (noise_variable_119 <= noise_metadata_schedule_294_e5050)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_294_e5059,) = {
                            if (noise_variable_119 >= 37.0) {
                                (noise_variable_119,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_294_e5059,)
                    }
                };
                (noise_metadata_schedule_294_e5060,)
            }
        };
        let noise_metadata_schedule_294_e5062: f64 = (noise_metadata_schedule_294_e5028 * noise_metadata_schedule_294_e5061);
        let noise_metadata_schedule_294_e5063: f64 = (noise_metadata_schedule_294_e5025 - noise_metadata_schedule_294_e5062);
        (noise_metadata_schedule_294_e5063,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_294_e5065;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_295_e5073,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_295_e5069: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_295_e5071: f64 = (noise_metadata_schedule_295_e5069 * noise_variable_137);
        (noise_metadata_schedule_295_e5071,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_295_e5073;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_296_e5081,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_296_e5077: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_296_e5079: f64 = (noise_metadata_schedule_296_e5077 * noise_variable_137);
        (noise_metadata_schedule_296_e5079,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_296_e5081;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_297_e5094,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_297_e5084: f64 = { let limited_exp_arg = noise_variable_118; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_297_e5086: f64 = (noise_metadata_schedule_297_e5084 * 3.24e17);
        let noise_metadata_schedule_297_e5090: f64 = (0.6666666666666666 * noise_variable_121);
        let noise_metadata_schedule_297_e5091: f64 = (1.0 + noise_metadata_schedule_297_e5090);
        let noise_metadata_schedule_297_e5092: f64 = (noise_metadata_schedule_297_e5086 * noise_metadata_schedule_297_e5091);
        (noise_metadata_schedule_297_e5092,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_297_e5094;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_298_e5101,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_298_e5098: f64 = { let limited_exp_arg = noise_variable_118; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_298_e5099: f64 = (1.0 + noise_metadata_schedule_298_e5098);
        (noise_metadata_schedule_298_e5099,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_298_e5101;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_299_e5114,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_299_e5104: f64 = { let limited_exp_arg = noise_variable_119; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_299_e5106: f64 = (noise_metadata_schedule_299_e5104 * 3.24e17);
        let noise_metadata_schedule_299_e5110: f64 = (0.6666666666666666 * noise_variable_122);
        let noise_metadata_schedule_299_e5111: f64 = (1.0 + noise_metadata_schedule_299_e5110);
        let noise_metadata_schedule_299_e5112: f64 = (noise_metadata_schedule_299_e5106 * noise_metadata_schedule_299_e5111);
        (noise_metadata_schedule_299_e5112,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_299_e5114;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_300_e5121,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_300_e5118: f64 = { let limited_exp_arg = noise_variable_119; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_300_e5119: f64 = (1.0 + noise_metadata_schedule_300_e5118);
        (noise_metadata_schedule_300_e5119,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_300_e5121;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_301_e5136,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_301_e5124: f64 = (-1.0);
        let noise_metadata_schedule_301_e5126: f64 = (noise_metadata_schedule_301_e5124 * noise_variable_99);
        let noise_metadata_schedule_301_e5129: f64 = (noise_variable_123 / noise_variable_124);
        let noise_metadata_schedule_301_e5130: f64 = (noise_metadata_schedule_301_e5126 - noise_metadata_schedule_301_e5129);
        let noise_metadata_schedule_301_e5133: f64 = (noise_variable_125 / noise_variable_126);
        let noise_metadata_schedule_301_e5134: f64 = (noise_metadata_schedule_301_e5130 - noise_metadata_schedule_301_e5133);
        (noise_metadata_schedule_301_e5134,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_301_e5136;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_302_e5144,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_302_e5141: f64 = (noise_variable_120 / noise_variable_127);
        let noise_metadata_schedule_302_e5142: f64 = (noise_variable_114 - noise_metadata_schedule_302_e5141);
        (noise_metadata_schedule_302_e5142,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_302_e5144;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_303_e5148,) = {
    if (noise_variable_394 != 0.0) {
        (noise_variable_128,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_303_e5148;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_304_e5153,) = {
    if (noise_variable_394 == 0.0) {
        (noise_variable_100,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_304_e5153;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_305_e5156: f64 = (params.p13 - noise_variable_345);
            noise_variable_347 = noise_metadata_schedule_305_e5156;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_306_e5159: f64 = (params.p17 - noise_variable_346);
            noise_variable_348 = noise_metadata_schedule_306_e5159;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_307_e5163: f64 = (noise_variable_82 / noise_variable_35);
            let noise_metadata_schedule_307_e5165: f64 = (noise_metadata_schedule_307_e5163).powf(params.p20);
            let noise_metadata_schedule_307_e5166: f64 = (noise_variable_347 * noise_metadata_schedule_307_e5165);
            noise_variable_97 = noise_metadata_schedule_307_e5166;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_308_e5170: f64 = (noise_variable_82 / noise_variable_35);
            let noise_metadata_schedule_308_e5172: f64 = (noise_metadata_schedule_308_e5170).powf(params.p19);
            let noise_metadata_schedule_308_e5173: f64 = (noise_variable_348 * noise_metadata_schedule_308_e5172);
            noise_variable_89 = noise_metadata_schedule_308_e5173;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_309_e5176: f64 = (noise_variable_80 / params.p9);
            let noise_metadata_schedule_309_e5179: f64 = (noise_variable_37 - noise_variable_129);
            let noise_metadata_schedule_309_e5180: f64 = (noise_metadata_schedule_309_e5179).abs();
            let noise_metadata_schedule_309_e5181: f64 = (noise_metadata_schedule_309_e5176 * noise_metadata_schedule_309_e5180);
            noise_variable_136 = noise_metadata_schedule_309_e5181;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_310_e5184: f64 = (noise_variable_81 / params.p9);
            let noise_metadata_schedule_310_e5187: f64 = (noise_variable_45 - noise_variable_129);
            let noise_metadata_schedule_310_e5188: f64 = (noise_metadata_schedule_310_e5187).abs();
            let noise_metadata_schedule_310_e5189: f64 = (noise_metadata_schedule_310_e5184 * noise_metadata_schedule_310_e5188);
            noise_variable_90 = noise_metadata_schedule_310_e5189;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_311_e5194: f64 = (params.p14 * noise_variable_136);
            let noise_metadata_schedule_311_e5195: f64 = (1.0 + noise_metadata_schedule_311_e5194);
            let noise_metadata_schedule_311_e5199: f64 = (noise_variable_136 * noise_variable_136);
            let noise_metadata_schedule_311_e5200: f64 = (params.p15 * noise_metadata_schedule_311_e5199);
            let noise_metadata_schedule_311_e5201: f64 = (noise_metadata_schedule_311_e5195 + noise_metadata_schedule_311_e5200);
            let noise_metadata_schedule_311_e5204: f64 = (params.p16 * noise_variable_90);
            let noise_metadata_schedule_311_e5205: f64 = (noise_metadata_schedule_311_e5201 + noise_metadata_schedule_311_e5204);
            let noise_metadata_schedule_311_e5206: f64 = (noise_variable_97 / noise_metadata_schedule_311_e5205);
            noise_variable_95 = noise_metadata_schedule_311_e5206;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_312_e5209: f64 = (2.0 * noise_variable_89);
            let noise_metadata_schedule_312_e5211: f64 = (noise_metadata_schedule_312_e5209 / noise_variable_95);
            noise_variable_136 = noise_metadata_schedule_312_e5211;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_313_e5214: f64 = (0.5 * noise_variable_37);
            let noise_metadata_schedule_313_e5218: f64 = (noise_variable_37 * noise_variable_37);
            let noise_metadata_schedule_313_e5221: f64 = (4.0 * 0.3);
            let noise_metadata_schedule_313_e5223: f64 = (noise_metadata_schedule_313_e5221 * 0.3);
            let noise_metadata_schedule_313_e5224: f64 = (noise_metadata_schedule_313_e5218 + noise_metadata_schedule_313_e5223);
            let noise_metadata_schedule_313_e5225: f64 = (noise_metadata_schedule_313_e5224).sqrt();
            let noise_metadata_schedule_313_e5226: f64 = (0.5 * noise_metadata_schedule_313_e5225);
            let noise_metadata_schedule_313_e5227: f64 = (noise_metadata_schedule_313_e5214 + noise_metadata_schedule_313_e5226);
            noise_variable_90 = noise_metadata_schedule_313_e5227;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_314_e5230: f64 = (noise_variable_136 * params.p3);
            let noise_metadata_schedule_314_e5232: f64 = (noise_metadata_schedule_314_e5230 * noise_variable_90);
            let noise_metadata_schedule_314_e5235: f64 = (noise_variable_136 * params.p3);
            let noise_metadata_schedule_314_e5237: f64 = (noise_metadata_schedule_314_e5235 + noise_variable_90);
            let noise_metadata_schedule_314_e5238: f64 = (noise_metadata_schedule_314_e5232 / noise_metadata_schedule_314_e5237);
            noise_variable_85 = noise_metadata_schedule_314_e5238;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_315_e5241: f64 = (noise_variable_38 / noise_variable_85);
            let noise_metadata_schedule_315_e5243: f64 = (noise_metadata_schedule_315_e5241).powf(params.p18);
            noise_variable_136 = noise_metadata_schedule_315_e5243;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_316_e5246: f64 = (1.0 + noise_variable_136);
            let noise_metadata_schedule_316_e5248: f64 = (-1.0);
            let noise_metadata_schedule_316_e5250: f64 = (noise_metadata_schedule_316_e5248 / params.p18);
            let noise_metadata_schedule_316_e5251: f64 = (noise_metadata_schedule_316_e5246).powf(noise_metadata_schedule_316_e5250);
            noise_variable_90 = noise_metadata_schedule_316_e5251;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_317_e5254: f64 = (noise_variable_38 * noise_variable_90);
            noise_variable_86 = noise_metadata_schedule_317_e5254;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_318_e5257: f64 = (noise_variable_37 - noise_variable_86);
            noise_variable_39 = noise_metadata_schedule_318_e5257;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_130 = noise_variable_39;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_320_e5261: f64 = (0.5 * noise_variable_130);
            let noise_metadata_schedule_320_e5265: f64 = (noise_variable_130 * noise_variable_130);
            let noise_metadata_schedule_320_e5268: f64 = (4.0 * 0.3);
            let noise_metadata_schedule_320_e5270: f64 = (noise_metadata_schedule_320_e5268 * 0.3);
            let noise_metadata_schedule_320_e5271: f64 = (noise_metadata_schedule_320_e5265 + noise_metadata_schedule_320_e5270);
            let noise_metadata_schedule_320_e5272: f64 = (noise_metadata_schedule_320_e5271).sqrt();
            let noise_metadata_schedule_320_e5273: f64 = (0.5 * noise_metadata_schedule_320_e5272);
            let noise_metadata_schedule_320_e5274: f64 = (noise_metadata_schedule_320_e5261 + noise_metadata_schedule_320_e5273);
            noise_variable_131 = noise_metadata_schedule_320_e5274;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            noise_variable_154 = noise_variable_131;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_322_e5278: f64 = (noise_variable_154 * noise_variable_150);
            let noise_metadata_schedule_322_e5281: f64 = (noise_variable_154 * noise_variable_154);
            let noise_metadata_schedule_322_e5284: f64 = (noise_variable_150 * noise_variable_150);
            let noise_metadata_schedule_322_e5285: f64 = (noise_metadata_schedule_322_e5281 + noise_metadata_schedule_322_e5284);
            let noise_metadata_schedule_322_e5286: f64 = (noise_metadata_schedule_322_e5285).sqrt();
            let noise_metadata_schedule_322_e5287: f64 = (noise_metadata_schedule_322_e5278 / noise_metadata_schedule_322_e5286);
            noise_variable_157 = noise_metadata_schedule_322_e5287;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_323_e5290: f64 = (noise_variable_154 * noise_variable_151);
            let noise_metadata_schedule_323_e5293: f64 = (noise_variable_154 * noise_variable_154);
            let noise_metadata_schedule_323_e5296: f64 = (noise_variable_151 * noise_variable_151);
            let noise_metadata_schedule_323_e5297: f64 = (noise_metadata_schedule_323_e5293 + noise_metadata_schedule_323_e5296);
            let noise_metadata_schedule_323_e5298: f64 = (noise_metadata_schedule_323_e5297).sqrt();
            let noise_metadata_schedule_323_e5299: f64 = (noise_metadata_schedule_323_e5290 / noise_metadata_schedule_323_e5298);
            noise_variable_158 = noise_metadata_schedule_323_e5299;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_324_e5305: f64 = (noise_variable_84 * noise_variable_157);
            let noise_metadata_schedule_324_e5306: f64 = (noise_metadata_schedule_324_e5305).ln();
            let noise_metadata_schedule_324_e5307: f64 = (1.0 - noise_metadata_schedule_324_e5306);
            let noise_metadata_schedule_324_e5308: f64 = (noise_variable_83 * noise_metadata_schedule_324_e5307);
            let noise_metadata_schedule_324_e5309: f64 = (noise_variable_154 + noise_metadata_schedule_324_e5308);
            let noise_metadata_schedule_324_e5312: f64 = (params.p28 / 3.0);
            let noise_metadata_schedule_324_e5315: f64 = (noise_variable_99 * noise_variable_154);
            let noise_metadata_schedule_324_e5317: f64 = (noise_metadata_schedule_324_e5315).powf(0.6666666666666666);
            let noise_metadata_schedule_324_e5318: f64 = (noise_metadata_schedule_324_e5312 * noise_metadata_schedule_324_e5317);
            let noise_metadata_schedule_324_e5319: f64 = (noise_metadata_schedule_324_e5309 - noise_metadata_schedule_324_e5318);
            let noise_metadata_schedule_324_e5324: f64 = (noise_variable_83 / noise_variable_158);
            let noise_metadata_schedule_324_e5325: f64 = (1.0 + noise_metadata_schedule_324_e5324);
            let noise_metadata_schedule_324_e5326: f64 = (noise_variable_154 * noise_metadata_schedule_324_e5325);
            let noise_metadata_schedule_324_e5329: f64 = (2.0 * params.p28);
            let noise_metadata_schedule_324_e5331: f64 = (noise_metadata_schedule_324_e5329 / 3.0);
            let noise_metadata_schedule_324_e5334: f64 = (noise_variable_99 * noise_variable_154);
            let noise_metadata_schedule_324_e5336: f64 = (noise_metadata_schedule_324_e5334).powf(0.6666666666666666);
            let noise_metadata_schedule_324_e5337: f64 = (noise_metadata_schedule_324_e5331 * noise_metadata_schedule_324_e5336);
            let noise_metadata_schedule_324_e5338: f64 = (noise_metadata_schedule_324_e5326 + noise_metadata_schedule_324_e5337);
            let noise_metadata_schedule_324_e5339: f64 = (noise_metadata_schedule_324_e5319 / noise_metadata_schedule_324_e5338);
            noise_variable_152 = noise_metadata_schedule_324_e5339;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_325_e5343: f64 = (2.0 * noise_variable_83);
            let noise_metadata_schedule_325_e5344: f64 = (noise_variable_130 / noise_metadata_schedule_325_e5343);
            noise_variable_136 = noise_metadata_schedule_325_e5344;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_326_e5347: f64 = if noise_variable_136 < 200.0 { 1.0 } else { 0.0 };
            noise_variable_395 = noise_metadata_schedule_326_e5347;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_327_e5354,) = {
    if (noise_variable_395 != 0.0) {
        let noise_metadata_schedule_327_e5351: f64 = (noise_variable_136 / 4.0);
        let noise_metadata_schedule_327_e5352: f64 = { let limited_exp_arg = noise_metadata_schedule_327_e5351; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (noise_metadata_schedule_327_e5352,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_327_e5354;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_328_e5364,) = {
    if (noise_variable_395 != 0.0) {
        let noise_metadata_schedule_328_e5357: f64 = (-3.0);
        let noise_metadata_schedule_328_e5359: f64 = (noise_metadata_schedule_328_e5357 * noise_variable_136);
        let noise_metadata_schedule_328_e5361: f64 = (noise_metadata_schedule_328_e5359 / 4.0);
        let noise_metadata_schedule_328_e5362: f64 = { let limited_exp_arg = noise_metadata_schedule_328_e5361; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (noise_metadata_schedule_328_e5362,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_328_e5364;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_329_e5401,) = {
    if (noise_variable_395 != 0.0) {
        let noise_metadata_schedule_329_e5368: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_329_e5370: f64 = (noise_metadata_schedule_329_e5368 * noise_variable_99);
        let noise_metadata_schedule_329_e5373: f64 = (3.0 * noise_variable_136);
        let noise_metadata_schedule_329_e5375: f64 = (noise_metadata_schedule_329_e5373 / 4.0);
        let noise_metadata_schedule_329_e5378: f64 = (noise_variable_90 + noise_variable_91);
        let noise_metadata_schedule_329_e5379: f64 = (noise_metadata_schedule_329_e5378).ln();
        let noise_metadata_schedule_329_e5380: f64 = (noise_metadata_schedule_329_e5375 + noise_metadata_schedule_329_e5379);
        let noise_metadata_schedule_329_e5381: f64 = (noise_metadata_schedule_329_e5370 * noise_metadata_schedule_329_e5380);
        let noise_metadata_schedule_329_e5384: f64 = (1.0 / noise_variable_152);
        let noise_metadata_schedule_329_e5387: f64 = (noise_variable_99 / 3.24e17);
        let noise_metadata_schedule_329_e5389: f64 = (-1.0);
        let noise_metadata_schedule_329_e5391: f64 = (noise_metadata_schedule_329_e5389 * noise_variable_130);
        let noise_metadata_schedule_329_e5394: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_329_e5395: f64 = (noise_metadata_schedule_329_e5391 / noise_metadata_schedule_329_e5394);
        let noise_metadata_schedule_329_e5396: f64 = { let limited_exp_arg = noise_metadata_schedule_329_e5395; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_329_e5397: f64 = (noise_metadata_schedule_329_e5387 * noise_metadata_schedule_329_e5396);
        let noise_metadata_schedule_329_e5398: f64 = (noise_metadata_schedule_329_e5384 + noise_metadata_schedule_329_e5397);
        let noise_metadata_schedule_329_e5399: f64 = (noise_metadata_schedule_329_e5381 / noise_metadata_schedule_329_e5398);
        (noise_metadata_schedule_329_e5399,)
    } else {
        (noise_variable_156,)
    }
};
            noise_variable_156 = noise_metadata_schedule_329_e5401;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_330_e5434,) = {
    if (noise_variable_395 == 0.0) {
        let noise_metadata_schedule_330_e5406: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_330_e5408: f64 = (noise_metadata_schedule_330_e5406 * noise_variable_99);
        let noise_metadata_schedule_330_e5411: f64 = noise_variable_136;
        let noise_metadata_schedule_330_e5413: f64 = noise_metadata_schedule_330_e5411;
        let noise_metadata_schedule_330_e5414: f64 = (noise_metadata_schedule_330_e5408 * noise_metadata_schedule_330_e5413);
        let noise_metadata_schedule_330_e5417: f64 = (1.0 / noise_variable_152);
        let noise_metadata_schedule_330_e5420: f64 = (noise_variable_99 / 3.24e17);
        let noise_metadata_schedule_330_e5422: f64 = (-1.0);
        let noise_metadata_schedule_330_e5424: f64 = (noise_metadata_schedule_330_e5422 * noise_variable_130);
        let noise_metadata_schedule_330_e5427: f64 = (2.0 * noise_variable_83);
        let noise_metadata_schedule_330_e5428: f64 = (noise_metadata_schedule_330_e5424 / noise_metadata_schedule_330_e5427);
        let noise_metadata_schedule_330_e5429: f64 = { let limited_exp_arg = noise_metadata_schedule_330_e5428; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_330_e5430: f64 = (noise_metadata_schedule_330_e5420 * noise_metadata_schedule_330_e5429);
        let noise_metadata_schedule_330_e5431: f64 = (noise_metadata_schedule_330_e5417 + noise_metadata_schedule_330_e5430);
        let noise_metadata_schedule_330_e5432: f64 = (noise_metadata_schedule_330_e5414 / noise_metadata_schedule_330_e5431);
        (noise_metadata_schedule_330_e5432,)
    } else {
        (noise_variable_156,)
    }
};
            noise_variable_156 = noise_metadata_schedule_330_e5434;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_331_e5438: f64 = (noise_variable_156 / noise_variable_99);
            let noise_metadata_schedule_331_e5439: f64 = (noise_variable_130 - noise_metadata_schedule_331_e5438);
            noise_variable_100 = noise_metadata_schedule_331_e5439;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_332_e5442: f64 = (noise_variable_100 - noise_variable_130);
            let noise_metadata_schedule_332_e5443: f64 = (noise_metadata_schedule_332_e5442).abs();
            let noise_metadata_schedule_332_e5445: f64 = if noise_metadata_schedule_332_e5443 > 1e-19 { 1.0 } else { 0.0 };
            noise_variable_396 = noise_metadata_schedule_332_e5445;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_333_e5451,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_333_e5449: f64 = (noise_variable_130 - noise_variable_100);
        (noise_metadata_schedule_333_e5449,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_333_e5451;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_334_e5470,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_334_e5455: f64 = (0.5 * noise_variable_101);
        let noise_metadata_schedule_334_e5459: f64 = (noise_variable_101 * noise_variable_101);
        let noise_metadata_schedule_334_e5462: f64 = (4.0 * 1e-9);
        let noise_metadata_schedule_334_e5464: f64 = (noise_metadata_schedule_334_e5462 * 1e-9);
        let noise_metadata_schedule_334_e5465: f64 = (noise_metadata_schedule_334_e5459 + noise_metadata_schedule_334_e5464);
        let noise_metadata_schedule_334_e5466: f64 = (noise_metadata_schedule_334_e5465).sqrt();
        let noise_metadata_schedule_334_e5467: f64 = (0.5 * noise_metadata_schedule_334_e5466);
        let noise_metadata_schedule_334_e5468: f64 = (noise_metadata_schedule_334_e5455 + noise_metadata_schedule_334_e5467);
        (noise_metadata_schedule_334_e5468,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_334_e5470;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_335_e5476,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_335_e5474: f64 = (noise_variable_99).powf(0.6666666666666666);
        (noise_metadata_schedule_335_e5474,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_335_e5476;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_336_e5482,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_336_e5480: f64 = (noise_variable_101).powf(0.6666666666666666);
        (noise_metadata_schedule_336_e5480,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_336_e5482;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_337_e5489,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_337_e5486: f64 = (-0.3333333333333333);
        let noise_metadata_schedule_337_e5487: f64 = (noise_variable_101).powf(noise_metadata_schedule_337_e5486);
        (noise_metadata_schedule_337_e5487,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_337_e5489;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_338_e5497,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_338_e5493: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_338_e5495: f64 = (noise_metadata_schedule_338_e5493 * noise_variable_90);
        (noise_metadata_schedule_338_e5495,)
    } else {
        (noise_variable_102,)
    }
};
            noise_variable_102 = noise_metadata_schedule_338_e5497;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_339_e5505,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_339_e5501: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_339_e5503: f64 = (noise_metadata_schedule_339_e5501 * noise_variable_90);
        (noise_metadata_schedule_339_e5503,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_339_e5505;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_340_e5515,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_340_e5509: f64 = (noise_variable_100 / noise_variable_83);
        let noise_metadata_schedule_340_e5512: f64 = (noise_variable_102 / noise_variable_83);
        let noise_metadata_schedule_340_e5513: f64 = (noise_metadata_schedule_340_e5509 - noise_metadata_schedule_340_e5512);
        (noise_metadata_schedule_340_e5513,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_340_e5515;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_341_e5525,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_341_e5519: f64 = (noise_variable_100 / noise_variable_83);
        let noise_metadata_schedule_341_e5522: f64 = (noise_variable_103 / noise_variable_83);
        let noise_metadata_schedule_341_e5523: f64 = (noise_metadata_schedule_341_e5519 - noise_metadata_schedule_341_e5522);
        (noise_metadata_schedule_341_e5523,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_341_e5525;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_342_e5607,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_342_e5529: f64 = (noise_variable_99 * noise_variable_101);
        let noise_metadata_schedule_342_e5532: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_342_e5539: f64 = (-37.0);
        let (noise_metadata_schedule_342_e5565,) = {
            if ((!(noise_variable_104 >= 37.0)) && (!(noise_variable_104 <= noise_metadata_schedule_342_e5539))) {
                let noise_metadata_schedule_342_e5544: f64 = (noise_variable_104).exp();
                let noise_metadata_schedule_342_e5546: f64 = (noise_metadata_schedule_342_e5544 + 1.0);
                let noise_metadata_schedule_342_e5547: f64 = (noise_metadata_schedule_342_e5546).ln();
                (noise_metadata_schedule_342_e5547,)
            } else {
                let noise_metadata_schedule_342_e5554: f64 = (-37.0);
                let (noise_metadata_schedule_342_e5564,) = {
                    if ((!(noise_variable_104 >= 37.0)) && (noise_variable_104 <= noise_metadata_schedule_342_e5554)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_342_e5563,) = {
                            if (noise_variable_104 >= 37.0) {
                                (noise_variable_104,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_342_e5563,)
                    }
                };
                (noise_metadata_schedule_342_e5564,)
            }
        };
        let noise_metadata_schedule_342_e5566: f64 = (noise_metadata_schedule_342_e5532 * noise_metadata_schedule_342_e5565);
        let noise_metadata_schedule_342_e5567: f64 = (noise_metadata_schedule_342_e5529 - noise_metadata_schedule_342_e5566);
        let noise_metadata_schedule_342_e5570: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_342_e5577: f64 = (-37.0);
        let (noise_metadata_schedule_342_e5603,) = {
            if ((!(noise_variable_105 >= 37.0)) && (!(noise_variable_105 <= noise_metadata_schedule_342_e5577))) {
                let noise_metadata_schedule_342_e5582: f64 = (noise_variable_105).exp();
                let noise_metadata_schedule_342_e5584: f64 = (noise_metadata_schedule_342_e5582 + 1.0);
                let noise_metadata_schedule_342_e5585: f64 = (noise_metadata_schedule_342_e5584).ln();
                (noise_metadata_schedule_342_e5585,)
            } else {
                let noise_metadata_schedule_342_e5592: f64 = (-37.0);
                let (noise_metadata_schedule_342_e5602,) = {
                    if ((!(noise_variable_105 >= 37.0)) && (noise_variable_105 <= noise_metadata_schedule_342_e5592)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_342_e5601,) = {
                            if (noise_variable_105 >= 37.0) {
                                (noise_variable_105,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_342_e5601,)
                    }
                };
                (noise_metadata_schedule_342_e5602,)
            }
        };
        let noise_metadata_schedule_342_e5604: f64 = (noise_metadata_schedule_342_e5570 * noise_metadata_schedule_342_e5603);
        let noise_metadata_schedule_342_e5605: f64 = (noise_metadata_schedule_342_e5567 - noise_metadata_schedule_342_e5604);
        (noise_metadata_schedule_342_e5605,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_342_e5607;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_343_e5615,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_343_e5611: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_343_e5613: f64 = (noise_metadata_schedule_343_e5611 * noise_variable_91);
        (noise_metadata_schedule_343_e5613,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_343_e5615;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_344_e5623,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_344_e5619: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_344_e5621: f64 = (noise_metadata_schedule_344_e5619 * noise_variable_91);
        (noise_metadata_schedule_344_e5621,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_344_e5623;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_345_e5636,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_345_e5626: f64 = { let limited_exp_arg = noise_variable_104; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_345_e5628: f64 = (noise_metadata_schedule_345_e5626 * 3.24e17);
        let noise_metadata_schedule_345_e5632: f64 = (0.6666666666666666 * noise_variable_107);
        let noise_metadata_schedule_345_e5633: f64 = (1.0 + noise_metadata_schedule_345_e5632);
        let noise_metadata_schedule_345_e5634: f64 = (noise_metadata_schedule_345_e5628 * noise_metadata_schedule_345_e5633);
        (noise_metadata_schedule_345_e5634,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_345_e5636;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_346_e5643,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_346_e5640: f64 = { let limited_exp_arg = noise_variable_104; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_346_e5641: f64 = (1.0 + noise_metadata_schedule_346_e5640);
        (noise_metadata_schedule_346_e5641,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_346_e5643;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_347_e5656,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_347_e5646: f64 = { let limited_exp_arg = noise_variable_105; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_347_e5648: f64 = (noise_metadata_schedule_347_e5646 * 3.24e17);
        let noise_metadata_schedule_347_e5652: f64 = (0.6666666666666666 * noise_variable_108);
        let noise_metadata_schedule_347_e5653: f64 = (1.0 + noise_metadata_schedule_347_e5652);
        let noise_metadata_schedule_347_e5654: f64 = (noise_metadata_schedule_347_e5648 * noise_metadata_schedule_347_e5653);
        (noise_metadata_schedule_347_e5654,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_347_e5656;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_348_e5663,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_348_e5660: f64 = { let limited_exp_arg = noise_variable_105; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_348_e5661: f64 = (1.0 + noise_metadata_schedule_348_e5660);
        (noise_metadata_schedule_348_e5661,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_348_e5663;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_349_e5678,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_349_e5666: f64 = (-1.0);
        let noise_metadata_schedule_349_e5668: f64 = (noise_metadata_schedule_349_e5666 * noise_variable_99);
        let noise_metadata_schedule_349_e5671: f64 = (noise_variable_109 / noise_variable_110);
        let noise_metadata_schedule_349_e5672: f64 = (noise_metadata_schedule_349_e5668 - noise_metadata_schedule_349_e5671);
        let noise_metadata_schedule_349_e5675: f64 = (noise_variable_111 / noise_variable_112);
        let noise_metadata_schedule_349_e5676: f64 = (noise_metadata_schedule_349_e5672 - noise_metadata_schedule_349_e5675);
        (noise_metadata_schedule_349_e5676,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_349_e5678;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_350_e5686,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_350_e5683: f64 = (noise_variable_106 / noise_variable_113);
        let noise_metadata_schedule_350_e5684: f64 = (noise_variable_100 - noise_metadata_schedule_350_e5683);
        (noise_metadata_schedule_350_e5684,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_350_e5686;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_351_e5692,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_351_e5690: f64 = (noise_variable_130 - noise_variable_114);
        (noise_metadata_schedule_351_e5690,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_351_e5692;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_352_e5711,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_352_e5696: f64 = (0.5 * noise_variable_115);
        let noise_metadata_schedule_352_e5700: f64 = (noise_variable_115 * noise_variable_115);
        let noise_metadata_schedule_352_e5703: f64 = (4.0 * 1e-9);
        let noise_metadata_schedule_352_e5705: f64 = (noise_metadata_schedule_352_e5703 * 1e-9);
        let noise_metadata_schedule_352_e5706: f64 = (noise_metadata_schedule_352_e5700 + noise_metadata_schedule_352_e5705);
        let noise_metadata_schedule_352_e5707: f64 = (noise_metadata_schedule_352_e5706).sqrt();
        let noise_metadata_schedule_352_e5708: f64 = (0.5 * noise_metadata_schedule_352_e5707);
        let noise_metadata_schedule_352_e5709: f64 = (noise_metadata_schedule_352_e5696 + noise_metadata_schedule_352_e5708);
        (noise_metadata_schedule_352_e5709,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_352_e5711;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_353_e5721,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_353_e5715: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_353_e5718: f64 = (noise_variable_115).powf(0.6666666666666666);
        let noise_metadata_schedule_353_e5719: f64 = (noise_metadata_schedule_353_e5715 * noise_metadata_schedule_353_e5718);
        (noise_metadata_schedule_353_e5719,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_353_e5721;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_354_e5731,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_354_e5725: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_354_e5728: f64 = (noise_variable_115).powf(0.6666666666666666);
        let noise_metadata_schedule_354_e5729: f64 = (noise_metadata_schedule_354_e5725 * noise_metadata_schedule_354_e5728);
        (noise_metadata_schedule_354_e5729,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_354_e5731;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_355_e5741,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_355_e5735: f64 = (noise_variable_114 / noise_variable_83);
        let noise_metadata_schedule_355_e5738: f64 = (noise_variable_116 / noise_variable_83);
        let noise_metadata_schedule_355_e5739: f64 = (noise_metadata_schedule_355_e5735 - noise_metadata_schedule_355_e5738);
        (noise_metadata_schedule_355_e5739,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_355_e5741;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_356_e5751,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_356_e5745: f64 = (noise_variable_114 / noise_variable_83);
        let noise_metadata_schedule_356_e5748: f64 = (noise_variable_117 / noise_variable_83);
        let noise_metadata_schedule_356_e5749: f64 = (noise_metadata_schedule_356_e5745 - noise_metadata_schedule_356_e5748);
        (noise_metadata_schedule_356_e5749,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_356_e5751;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_357_e5833,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_357_e5755: f64 = (noise_variable_99 * noise_variable_115);
        let noise_metadata_schedule_357_e5758: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_357_e5765: f64 = (-37.0);
        let (noise_metadata_schedule_357_e5791,) = {
            if ((!(noise_variable_118 >= 37.0)) && (!(noise_variable_118 <= noise_metadata_schedule_357_e5765))) {
                let noise_metadata_schedule_357_e5770: f64 = (noise_variable_118).exp();
                let noise_metadata_schedule_357_e5772: f64 = (noise_metadata_schedule_357_e5770 + 1.0);
                let noise_metadata_schedule_357_e5773: f64 = (noise_metadata_schedule_357_e5772).ln();
                (noise_metadata_schedule_357_e5773,)
            } else {
                let noise_metadata_schedule_357_e5780: f64 = (-37.0);
                let (noise_metadata_schedule_357_e5790,) = {
                    if ((!(noise_variable_118 >= 37.0)) && (noise_variable_118 <= noise_metadata_schedule_357_e5780)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_357_e5789,) = {
                            if (noise_variable_118 >= 37.0) {
                                (noise_variable_118,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_357_e5789,)
                    }
                };
                (noise_metadata_schedule_357_e5790,)
            }
        };
        let noise_metadata_schedule_357_e5792: f64 = (noise_metadata_schedule_357_e5758 * noise_metadata_schedule_357_e5791);
        let noise_metadata_schedule_357_e5793: f64 = (noise_metadata_schedule_357_e5755 - noise_metadata_schedule_357_e5792);
        let noise_metadata_schedule_357_e5796: f64 = (3.24e17 * noise_variable_83);
        let noise_metadata_schedule_357_e5803: f64 = (-37.0);
        let (noise_metadata_schedule_357_e5829,) = {
            if ((!(noise_variable_119 >= 37.0)) && (!(noise_variable_119 <= noise_metadata_schedule_357_e5803))) {
                let noise_metadata_schedule_357_e5808: f64 = (noise_variable_119).exp();
                let noise_metadata_schedule_357_e5810: f64 = (noise_metadata_schedule_357_e5808 + 1.0);
                let noise_metadata_schedule_357_e5811: f64 = (noise_metadata_schedule_357_e5810).ln();
                (noise_metadata_schedule_357_e5811,)
            } else {
                let noise_metadata_schedule_357_e5818: f64 = (-37.0);
                let (noise_metadata_schedule_357_e5828,) = {
                    if ((!(noise_variable_119 >= 37.0)) && (noise_variable_119 <= noise_metadata_schedule_357_e5818)) {
                        (0.0,)
                    } else {
                        let (noise_metadata_schedule_357_e5827,) = {
                            if (noise_variable_119 >= 37.0) {
                                (noise_variable_119,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_357_e5827,)
                    }
                };
                (noise_metadata_schedule_357_e5828,)
            }
        };
        let noise_metadata_schedule_357_e5830: f64 = (noise_metadata_schedule_357_e5796 * noise_metadata_schedule_357_e5829);
        let noise_metadata_schedule_357_e5831: f64 = (noise_metadata_schedule_357_e5793 - noise_metadata_schedule_357_e5830);
        (noise_metadata_schedule_357_e5831,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_357_e5833;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_358_e5844,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_358_e5837: f64 = (params.p28 * noise_variable_136);
        let noise_metadata_schedule_358_e5840: f64 = (-0.3333333333333333);
        let noise_metadata_schedule_358_e5841: f64 = (noise_variable_115).powf(noise_metadata_schedule_358_e5840);
        let noise_metadata_schedule_358_e5842: f64 = (noise_metadata_schedule_358_e5837 * noise_metadata_schedule_358_e5841);
        (noise_metadata_schedule_358_e5842,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_358_e5844;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_359_e5855,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_359_e5848: f64 = (params.p29 * noise_variable_136);
        let noise_metadata_schedule_359_e5851: f64 = (-0.3333333333333333);
        let noise_metadata_schedule_359_e5852: f64 = (noise_variable_115).powf(noise_metadata_schedule_359_e5851);
        let noise_metadata_schedule_359_e5853: f64 = (noise_metadata_schedule_359_e5848 * noise_metadata_schedule_359_e5852);
        (noise_metadata_schedule_359_e5853,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_359_e5855;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_360_e5868,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_360_e5858: f64 = { let limited_exp_arg = noise_variable_118; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_360_e5860: f64 = (noise_metadata_schedule_360_e5858 * 3.24e17);
        let noise_metadata_schedule_360_e5864: f64 = (0.6666666666666666 * noise_variable_121);
        let noise_metadata_schedule_360_e5865: f64 = (1.0 + noise_metadata_schedule_360_e5864);
        let noise_metadata_schedule_360_e5866: f64 = (noise_metadata_schedule_360_e5860 * noise_metadata_schedule_360_e5865);
        (noise_metadata_schedule_360_e5866,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_360_e5868;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_361_e5875,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_361_e5872: f64 = { let limited_exp_arg = noise_variable_118; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_361_e5873: f64 = (1.0 + noise_metadata_schedule_361_e5872);
        (noise_metadata_schedule_361_e5873,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_361_e5875;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_362_e5888,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_362_e5878: f64 = { let limited_exp_arg = noise_variable_119; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_362_e5880: f64 = (noise_metadata_schedule_362_e5878 * 3.24e17);
        let noise_metadata_schedule_362_e5884: f64 = (0.6666666666666666 * noise_variable_122);
        let noise_metadata_schedule_362_e5885: f64 = (1.0 + noise_metadata_schedule_362_e5884);
        let noise_metadata_schedule_362_e5886: f64 = (noise_metadata_schedule_362_e5880 * noise_metadata_schedule_362_e5885);
        (noise_metadata_schedule_362_e5886,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_362_e5888;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_363_e5895,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_363_e5892: f64 = { let limited_exp_arg = noise_variable_119; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_363_e5893: f64 = (1.0 + noise_metadata_schedule_363_e5892);
        (noise_metadata_schedule_363_e5893,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_363_e5895;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_364_e5910,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_364_e5898: f64 = (-1.0);
        let noise_metadata_schedule_364_e5900: f64 = (noise_metadata_schedule_364_e5898 * noise_variable_99);
        let noise_metadata_schedule_364_e5903: f64 = (noise_variable_123 / noise_variable_124);
        let noise_metadata_schedule_364_e5904: f64 = (noise_metadata_schedule_364_e5900 - noise_metadata_schedule_364_e5903);
        let noise_metadata_schedule_364_e5907: f64 = (noise_variable_125 / noise_variable_126);
        let noise_metadata_schedule_364_e5908: f64 = (noise_metadata_schedule_364_e5904 - noise_metadata_schedule_364_e5907);
        (noise_metadata_schedule_364_e5908,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_364_e5910;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_365_e5918,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_365_e5915: f64 = (noise_variable_120 / noise_variable_127);
        let noise_metadata_schedule_365_e5916: f64 = (noise_variable_114 - noise_metadata_schedule_365_e5915);
        (noise_metadata_schedule_365_e5916,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_365_e5918;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_366_e5924,) = {
    if (noise_variable_396 != 0.0) {
        let noise_metadata_schedule_366_e5922: f64 = (noise_variable_128 + noise_variable_86);
        (noise_metadata_schedule_366_e5922,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_366_e5924;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let (noise_metadata_schedule_367_e5931,) = {
    if (noise_variable_396 == 0.0) {
        let noise_metadata_schedule_367_e5929: f64 = (noise_variable_100 + noise_variable_86);
        (noise_metadata_schedule_367_e5929,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_367_e5931;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_368_e5935: f64 = (noise_variable_129 + noise_variable_132);
            let noise_metadata_schedule_368_e5936: f64 = (0.5 * noise_metadata_schedule_368_e5935);
            noise_variable_133 = noise_metadata_schedule_368_e5936;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_369_e5939: f64 = (noise_variable_132 - noise_variable_129);
            noise_variable_134 = noise_metadata_schedule_369_e5939;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_370_e5942: f64 = (noise_variable_37 - noise_variable_133);
            let noise_metadata_schedule_370_e5944: f64 = (noise_metadata_schedule_370_e5942 + noise_variable_83);
            let noise_metadata_schedule_370_e5946: f64 = (noise_metadata_schedule_370_e5944 * noise_variable_134);
            noise_variable_135 = noise_metadata_schedule_370_e5946;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_371_e5949: f64 = (noise_variable_80 / params.p9);
            let noise_metadata_schedule_371_e5952: f64 = (noise_variable_37 - noise_variable_133);
            let noise_metadata_schedule_371_e5953: f64 = (noise_metadata_schedule_371_e5952).abs();
            let noise_metadata_schedule_371_e5954: f64 = (noise_metadata_schedule_371_e5949 * noise_metadata_schedule_371_e5953);
            noise_variable_136 = noise_metadata_schedule_371_e5954;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_372_e5957: f64 = (noise_variable_81 / params.p9);
            let noise_metadata_schedule_372_e5960: f64 = (noise_variable_45 - noise_variable_129);
            let noise_metadata_schedule_372_e5961: f64 = (noise_metadata_schedule_372_e5960).abs();
            let noise_metadata_schedule_372_e5962: f64 = (noise_metadata_schedule_372_e5957 * noise_metadata_schedule_372_e5961);
            noise_variable_90 = noise_metadata_schedule_372_e5962;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_373_e5967: f64 = (params.p14 * noise_variable_136);
            let noise_metadata_schedule_373_e5968: f64 = (1.0 + noise_metadata_schedule_373_e5967);
            let noise_metadata_schedule_373_e5971: f64 = (params.p15 * noise_variable_136);
            let noise_metadata_schedule_373_e5973: f64 = (noise_metadata_schedule_373_e5971 * noise_variable_136);
            let noise_metadata_schedule_373_e5974: f64 = (noise_metadata_schedule_373_e5968 + noise_metadata_schedule_373_e5973);
            let noise_metadata_schedule_373_e5977: f64 = (params.p16 * noise_variable_90);
            let noise_metadata_schedule_373_e5978: f64 = (noise_metadata_schedule_373_e5974 + noise_metadata_schedule_373_e5977);
            let noise_metadata_schedule_373_e5979: f64 = (noise_variable_97 / noise_metadata_schedule_373_e5978);
            noise_variable_95 = noise_metadata_schedule_373_e5979;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_374_e5982: f64 = (noise_variable_95 * noise_variable_80);
            let noise_metadata_schedule_374_e5984: f64 = (noise_metadata_schedule_374_e5982 * params.p4);
            let noise_metadata_schedule_374_e5986: f64 = (noise_metadata_schedule_374_e5984 * params.p5);
            let noise_metadata_schedule_374_e5988: f64 = (noise_metadata_schedule_374_e5986 / params.p3);
            noise_variable_96 = noise_metadata_schedule_374_e5988;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_375_e5994: f64 = (noise_variable_140 - noise_variable_86);
            let noise_metadata_schedule_375_e5995: f64 = (params.p21 * noise_metadata_schedule_375_e5994);
            let noise_metadata_schedule_375_e5996: f64 = (1.0 + noise_metadata_schedule_375_e5995);
            let noise_metadata_schedule_375_e5997: f64 = (noise_variable_96 * noise_metadata_schedule_375_e5996);
            noise_variable_98 = noise_metadata_schedule_375_e5997;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_376_e6001: f64 = (params.p25 * params.p25);
            let noise_metadata_schedule_376_e6003: f64 = (noise_metadata_schedule_376_e6001 * noise_variable_134);
            let noise_metadata_schedule_376_e6005: f64 = (noise_metadata_schedule_376_e6003 * noise_variable_134);
            let noise_metadata_schedule_376_e6006: f64 = (1.0 + noise_metadata_schedule_376_e6005);
            let noise_metadata_schedule_376_e6007: f64 = (noise_metadata_schedule_376_e6006).sqrt();
            noise_variable_92 = noise_metadata_schedule_376_e6007;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_377_e6010: f64 = (noise_variable_98 / noise_variable_92);
            noise_variable_93 = noise_metadata_schedule_377_e6010;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 7) {
            let noise_metadata_schedule_378_e6013: f64 = (noise_variable_93 * noise_variable_135);
            noise_variable_94 = noise_metadata_schedule_378_e6013;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_379_e6019: f64 = (noise_variable_334 - 1.0);
            let noise_metadata_schedule_379_e6020: f64 = (params.p271 * noise_metadata_schedule_379_e6019);
            let noise_metadata_schedule_379_e6021: f64 = (1.0 + noise_metadata_schedule_379_e6020);
            let noise_metadata_schedule_379_e6022: f64 = (params.p269 * noise_metadata_schedule_379_e6021);
            noise_variable_333 = noise_metadata_schedule_379_e6022;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_380_e6028: f64 = (noise_variable_334 - 1.0);
            let noise_metadata_schedule_380_e6029: f64 = (params.p272 * noise_metadata_schedule_380_e6028);
            let noise_metadata_schedule_380_e6030: f64 = (1.0 + noise_metadata_schedule_380_e6029);
            let noise_metadata_schedule_380_e6031: f64 = (params.p270 * noise_metadata_schedule_380_e6030);
            noise_variable_335 = noise_metadata_schedule_380_e6031;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_381_e6037: f64 = (noise_variable_334 - 1.0);
            let noise_metadata_schedule_381_e6038: f64 = (params.p273 * noise_metadata_schedule_381_e6037);
            let noise_metadata_schedule_381_e6039: f64 = (1.0 + noise_metadata_schedule_381_e6038);
            let noise_metadata_schedule_381_e6040: f64 = (params.p268 * noise_metadata_schedule_381_e6039);
            noise_variable_336 = noise_metadata_schedule_381_e6040;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_382_e6043: f64 = if noise_variable_333 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_397 = noise_metadata_schedule_382_e6043;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_383_e6046: f64 = (noise_variable_141 - noise_variable_336);
            let noise_metadata_schedule_383_e6048: f64 = if noise_metadata_schedule_383_e6046 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_398 = noise_metadata_schedule_383_e6048;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_384_e6062,) = {
    if ((noise_variable_397 != 0.0) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_384_e6054: f64 = (noise_variable_141 - noise_variable_336);
        let noise_metadata_schedule_384_e6056: f64 = noise_metadata_schedule_384_e6054;
        let noise_metadata_schedule_384_e6059: f64 = (noise_variable_335 * noise_variable_36);
        let noise_metadata_schedule_384_e6060: f64 = (noise_metadata_schedule_384_e6056 / noise_metadata_schedule_384_e6059);
        (noise_metadata_schedule_384_e6060,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_384_e6062;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_385_e6065: f64 = if noise_variable_354 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_399 = noise_metadata_schedule_385_e6065;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_386_e6077,) = {
    if (((noise_variable_397 != 0.0) && (noise_variable_398 != 0.0)) && (noise_variable_399 != 0.0)) {
        let noise_metadata_schedule_386_e6074: f64 = (noise_variable_354 - 80.0);
        let noise_metadata_schedule_386_e6075: f64 = (1.0 + noise_metadata_schedule_386_e6074);
        (noise_metadata_schedule_386_e6075,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_386_e6077;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_387_e6085,) = {
    if (((noise_variable_397 != 0.0) && (noise_variable_398 != 0.0)) && (noise_variable_399 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_387_e6085;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_388_e6094,) = {
    if (((noise_variable_397 != 0.0) && (noise_variable_398 != 0.0)) && (noise_variable_399 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_388_e6094;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_389_e6103,) = {
    if ((noise_variable_397 != 0.0) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_389_e6100: f64 = (noise_variable_354).exp();
        let noise_metadata_schedule_389_e6101: f64 = (noise_variable_355 * noise_metadata_schedule_389_e6100);
        (noise_metadata_schedule_389_e6101,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_389_e6103;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_391_e6126,) = {
    if ((noise_variable_397 != 0.0) && (noise_variable_398 == 0.0)) {
        let noise_metadata_schedule_391_e6120: f64 = (noise_variable_141 - noise_variable_336);
        let noise_metadata_schedule_391_e6123: f64 = (noise_variable_335 * noise_variable_36);
        let noise_metadata_schedule_391_e6124: f64 = (noise_metadata_schedule_391_e6120 / noise_metadata_schedule_391_e6123);
        (noise_metadata_schedule_391_e6124,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_391_e6126;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_392_e6129: f64 = if noise_variable_354 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_400 = noise_metadata_schedule_392_e6129;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_393_e6142,) = {
    if (((noise_variable_397 != 0.0) && (noise_variable_398 == 0.0)) && (noise_variable_400 != 0.0)) {
        let noise_metadata_schedule_393_e6139: f64 = (noise_variable_354 - 80.0);
        let noise_metadata_schedule_393_e6140: f64 = (1.0 + noise_metadata_schedule_393_e6139);
        (noise_metadata_schedule_393_e6140,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_393_e6142;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_394_e6151,) = {
    if (((noise_variable_397 != 0.0) && (noise_variable_398 == 0.0)) && (noise_variable_400 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_394_e6151;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_395_e6161,) = {
    if (((noise_variable_397 != 0.0) && (noise_variable_398 == 0.0)) && (noise_variable_400 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_395_e6161;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_396_e6171,) = {
    if ((noise_variable_397 != 0.0) && (noise_variable_398 == 0.0)) {
        let noise_metadata_schedule_396_e6168: f64 = (noise_variable_354).exp();
        let noise_metadata_schedule_396_e6169: f64 = (noise_variable_355 * noise_metadata_schedule_396_e6168);
        (noise_metadata_schedule_396_e6169,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_396_e6171;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_399_e6190: f64 = (noise_variable_132 - noise_variable_129);
            noise_variable_90 = noise_metadata_schedule_399_e6190;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_400_e6193: f64 = (noise_variable_37 + noise_variable_83);
            let noise_metadata_schedule_400_e6195: f64 = (noise_metadata_schedule_400_e6193 - noise_variable_133);
            noise_variable_91 = noise_metadata_schedule_400_e6195;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_401_e6198: f64 = (noise_variable_80 * params.p4);
            let noise_metadata_schedule_401_e6200: f64 = (noise_metadata_schedule_401_e6198 * params.p5);
            let noise_metadata_schedule_401_e6202: f64 = (noise_metadata_schedule_401_e6200 * params.p3);
            let noise_metadata_schedule_401_e6205: f64 = (noise_variable_37 - noise_variable_133);
            let noise_metadata_schedule_401_e6208: f64 = (0.5 * noise_variable_90);
            let noise_metadata_schedule_401_e6210: f64 = (noise_metadata_schedule_401_e6208 * noise_variable_90);
            let noise_metadata_schedule_401_e6213: f64 = (6.0 * noise_variable_91);
            let noise_metadata_schedule_401_e6214: f64 = (noise_metadata_schedule_401_e6210 / noise_metadata_schedule_401_e6213);
            let noise_metadata_schedule_401_e6215: f64 = (noise_metadata_schedule_401_e6205 + noise_metadata_schedule_401_e6214);
            let noise_metadata_schedule_401_e6216: f64 = (noise_metadata_schedule_401_e6202 * noise_metadata_schedule_401_e6215);
            noise_variable_137 = noise_metadata_schedule_401_e6216;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_402_e6220: f64 = (noise_variable_137 / params.p233);
            let noise_metadata_schedule_402_e6221: f64 = (1e26 * noise_metadata_schedule_402_e6220);
            noise_variable_188 = noise_metadata_schedule_402_e6221;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_403_e6225: f64 = (noise_variable_188).powf(params.p232);
            let noise_metadata_schedule_403_e6226: f64 = (1.0 + noise_metadata_schedule_403_e6225);
            noise_variable_189 = noise_metadata_schedule_403_e6226;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_404_e6229: f64 = (params.p231 / noise_variable_189);
            noise_variable_190 = noise_metadata_schedule_404_e6229;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_405_e6233: f64 = (params.p1 + noise_variable_190);
            let noise_metadata_schedule_405_e6234: f64 = (params.p9 / noise_metadata_schedule_405_e6233);
            noise_variable_191 = noise_metadata_schedule_405_e6234;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_406_e6237: f64 = (noise_variable_191 * params.p4);
            let noise_metadata_schedule_406_e6239: f64 = (noise_metadata_schedule_406_e6237 * params.p5);
            let noise_metadata_schedule_406_e6241: f64 = (noise_metadata_schedule_406_e6239 * params.p3);
            let noise_metadata_schedule_406_e6244: f64 = (noise_variable_37 - noise_variable_133);
            let noise_metadata_schedule_406_e6247: f64 = (0.5 * noise_variable_90);
            let noise_metadata_schedule_406_e6249: f64 = (noise_metadata_schedule_406_e6247 * noise_variable_90);
            let noise_metadata_schedule_406_e6252: f64 = (6.0 * noise_variable_91);
            let noise_metadata_schedule_406_e6253: f64 = (noise_metadata_schedule_406_e6249 / noise_metadata_schedule_406_e6252);
            let noise_metadata_schedule_406_e6254: f64 = (noise_metadata_schedule_406_e6244 + noise_metadata_schedule_406_e6253);
            let noise_metadata_schedule_406_e6255: f64 = (noise_metadata_schedule_406_e6241 * noise_metadata_schedule_406_e6254);
            noise_variable_161 = noise_metadata_schedule_406_e6255;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_407_e6258: f64 = (noise_variable_37 + noise_variable_83);
            let noise_metadata_schedule_407_e6260: f64 = (noise_metadata_schedule_407_e6258 - noise_variable_133);
            noise_variable_136 = noise_metadata_schedule_407_e6260;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_408_e6264: f64 = (2.0 * noise_variable_132);
            let noise_metadata_schedule_408_e6265: f64 = (noise_variable_129 + noise_metadata_schedule_408_e6264);
            let noise_metadata_schedule_408_e6267: f64 = (noise_metadata_schedule_408_e6265 / 3.0);
            noise_variable_90 = noise_metadata_schedule_408_e6267;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_409_e6270: f64 = (1.0 / 12.0);
            let noise_metadata_schedule_409_e6273: f64 = (noise_variable_134 * noise_variable_134);
            let noise_metadata_schedule_409_e6274: f64 = (noise_metadata_schedule_409_e6270 * noise_metadata_schedule_409_e6273);
            let noise_metadata_schedule_409_e6276: f64 = (noise_metadata_schedule_409_e6274 / noise_variable_136);
            noise_variable_91 = noise_metadata_schedule_409_e6276;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6 | 7) {
            let noise_metadata_schedule_410_e6279: f64 = (1.0 / 120.0);
            let noise_metadata_schedule_410_e6282: f64 = (noise_variable_134 * noise_variable_134);
            let noise_metadata_schedule_410_e6284: f64 = (noise_metadata_schedule_410_e6282 * noise_variable_134);
            let noise_metadata_schedule_410_e6285: f64 = (noise_metadata_schedule_410_e6279 * noise_metadata_schedule_410_e6284);
            let noise_metadata_schedule_410_e6288: f64 = (noise_variable_136 * noise_variable_136);
            let noise_metadata_schedule_410_e6289: f64 = (noise_metadata_schedule_410_e6285 / noise_metadata_schedule_410_e6288);
            noise_variable_137 = noise_metadata_schedule_410_e6289;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_411_e6292: f64 = (noise_variable_191 * params.p4);
            let noise_metadata_schedule_411_e6294: f64 = (noise_metadata_schedule_411_e6292 * params.p3);
            let noise_metadata_schedule_411_e6296: f64 = (noise_metadata_schedule_411_e6294 * params.p5);
            let noise_metadata_schedule_411_e6298: f64 = (noise_metadata_schedule_411_e6296 * 0.5);
            let noise_metadata_schedule_411_e6299: f64 = (-noise_metadata_schedule_411_e6298);
            let noise_metadata_schedule_411_e6302: f64 = (noise_variable_37 - noise_variable_90);
            let noise_metadata_schedule_411_e6304: f64 = (noise_metadata_schedule_411_e6302 + noise_variable_91);
            let noise_metadata_schedule_411_e6306: f64 = (noise_metadata_schedule_411_e6304 + noise_variable_137);
            let noise_metadata_schedule_411_e6307: f64 = (noise_metadata_schedule_411_e6299 * noise_metadata_schedule_411_e6306);
            noise_variable_165 = noise_metadata_schedule_411_e6307;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_412_e6309: f64 = (-1.0);
            let noise_metadata_schedule_412_e6311: f64 = (noise_metadata_schedule_412_e6309 * noise_variable_161);
            let noise_metadata_schedule_412_e6314: f64 = noise_variable_165;
            let noise_metadata_schedule_412_e6315: f64 = (noise_metadata_schedule_412_e6311 - noise_metadata_schedule_412_e6314);
            noise_variable_166 = noise_metadata_schedule_412_e6315;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let noise_metadata_schedule_413_e6318: f64 = if noise_variable_41 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_401 = noise_metadata_schedule_413_e6318;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 7) {
            let (noise_metadata_schedule_414_e6322,) = {
    if (noise_variable_401 != 0.0) {
        (noise_variable_166,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_414_e6322;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_415_e6326,) = {
    if (noise_variable_401 != 0.0) {
        (noise_variable_165,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_415_e6326;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_416_e6330,) = {
    if (noise_variable_401 != 0.0) {
        (noise_variable_90,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_416_e6330;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_417_e6333: f64 = if params.p56 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_402 = noise_metadata_schedule_417_e6333;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_418_e6336: f64 = if params.p56 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_403 = noise_metadata_schedule_418_e6336;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_419_e6339: f64 = if params.p56 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_404 = noise_metadata_schedule_419_e6339;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_420_e6342: f64 = if params.p56 == 3.0 { 1.0 } else { 0.0 };
            noise_variable_405 = noise_metadata_schedule_420_e6342;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_421_e6345: f64 = if params.p56 == 4.0 { 1.0 } else { 0.0 };
            noise_variable_406 = noise_metadata_schedule_421_e6345;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_422_e6349,) = {
    if (noise_variable_402 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_422_e6349;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_423_e6353,) = {
    if (noise_variable_402 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_423_e6353;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_424_e6366,) = {
    if ((noise_variable_403 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_424_e6361: f64 = (params.p57 * 8.617087e-5);
        let noise_metadata_schedule_424_e6363: f64 = (noise_metadata_schedule_424_e6361 * noise_variable_82);
        let noise_metadata_schedule_424_e6364: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])) / noise_metadata_schedule_424_e6363);
        (noise_metadata_schedule_424_e6364,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_424_e6366;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_425_e6381,) = {
    if ((noise_variable_403 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_425_e6374: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_425_e6376: f64 = (noise_metadata_schedule_425_e6374 - 1.0);
        let noise_metadata_schedule_425_e6378: f64 = (noise_metadata_schedule_425_e6376 * params.p71);
        let noise_metadata_schedule_425_e6379: f64 = (params.p63 + noise_metadata_schedule_425_e6378);
        (noise_metadata_schedule_425_e6379,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_425_e6381;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_426_e6400,) = {
    if ((noise_variable_403 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_426_e6388: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_426_e6390: f64 = (noise_metadata_schedule_426_e6388 * params.p5);
        let noise_metadata_schedule_426_e6392: f64 = (noise_variable_137).abs();
        let noise_metadata_schedule_426_e6393: f64 = (noise_metadata_schedule_426_e6390 * noise_metadata_schedule_426_e6392);
        let noise_metadata_schedule_426_e6395: f64 = { let limited_exp_arg = noise_variable_136; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_426_e6397: f64 = (noise_metadata_schedule_426_e6395 - 1.0);
        let noise_metadata_schedule_426_e6398: f64 = (noise_metadata_schedule_426_e6393 * noise_metadata_schedule_426_e6397);
        (noise_metadata_schedule_426_e6398,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_426_e6400;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_427_e6413,) = {
    if ((noise_variable_403 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_427_e6408: f64 = (params.p60 * 8.617087e-5);
        let noise_metadata_schedule_427_e6410: f64 = (noise_metadata_schedule_427_e6408 * noise_variable_82);
        let noise_metadata_schedule_427_e6411: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])) / noise_metadata_schedule_427_e6410);
        (noise_metadata_schedule_427_e6411,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_427_e6413;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_428_e6428,) = {
    if ((noise_variable_403 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_428_e6421: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_428_e6423: f64 = (noise_metadata_schedule_428_e6421 - 1.0);
        let noise_metadata_schedule_428_e6425: f64 = (noise_metadata_schedule_428_e6423 * params.p72);
        let noise_metadata_schedule_428_e6426: f64 = (params.p64 + noise_metadata_schedule_428_e6425);
        (noise_metadata_schedule_428_e6426,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_428_e6428;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_429_e6447,) = {
    if ((noise_variable_403 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_429_e6435: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_429_e6437: f64 = (noise_metadata_schedule_429_e6435 * params.p5);
        let noise_metadata_schedule_429_e6439: f64 = (noise_variable_137).abs();
        let noise_metadata_schedule_429_e6440: f64 = (noise_metadata_schedule_429_e6437 * noise_metadata_schedule_429_e6439);
        let noise_metadata_schedule_429_e6442: f64 = { let limited_exp_arg = noise_variable_136; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_429_e6444: f64 = (noise_metadata_schedule_429_e6442 - 1.0);
        let noise_metadata_schedule_429_e6445: f64 = (noise_metadata_schedule_429_e6440 * noise_metadata_schedule_429_e6444);
        (noise_metadata_schedule_429_e6445,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_429_e6447;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_430_e6464,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_430_e6457: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_430_e6459: f64 = (noise_metadata_schedule_430_e6457 - 1.0);
        let noise_metadata_schedule_430_e6461: f64 = (noise_metadata_schedule_430_e6459 * params.p75);
        let noise_metadata_schedule_430_e6462: f64 = (params.p67 + noise_metadata_schedule_430_e6461);
        (noise_metadata_schedule_430_e6462,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_430_e6464;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_431_e6481,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_431_e6474: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_431_e6476: f64 = (noise_metadata_schedule_431_e6474 - 1.0);
        let noise_metadata_schedule_431_e6478: f64 = (noise_metadata_schedule_431_e6476 * params.p77);
        let noise_metadata_schedule_431_e6479: f64 = (params.p57 + noise_metadata_schedule_431_e6478);
        (noise_metadata_schedule_431_e6479,)
    } else {
        (noise_variable_328,)
    }
};
            noise_variable_328 = noise_metadata_schedule_431_e6481;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_432_e6498,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_432_e6491: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_432_e6493: f64 = (noise_metadata_schedule_432_e6491 - 1.0);
        let noise_metadata_schedule_432_e6495: f64 = (noise_metadata_schedule_432_e6493 * params.p79);
        let noise_metadata_schedule_432_e6496: f64 = (params.p61 + noise_metadata_schedule_432_e6495);
        (noise_metadata_schedule_432_e6496,)
    } else {
        (noise_variable_330,)
    }
};
            noise_variable_330 = noise_metadata_schedule_432_e6498;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_433_e6515,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_433_e6507: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])) - noise_variable_326);
        let noise_metadata_schedule_433_e6510: f64 = (noise_variable_328 * 8.617087e-5);
        let noise_metadata_schedule_433_e6512: f64 = (noise_metadata_schedule_433_e6510 * noise_variable_35);
        let noise_metadata_schedule_433_e6513: f64 = (noise_metadata_schedule_433_e6507 / noise_metadata_schedule_433_e6512);
        (noise_metadata_schedule_433_e6513,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_433_e6515;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_434_e6533,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_434_e6526: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_434_e6528: f64 = (noise_metadata_schedule_434_e6526 - 1.0);
        let noise_metadata_schedule_434_e6529: f64 = (params.p71 * noise_metadata_schedule_434_e6528);
        let noise_metadata_schedule_434_e6530: f64 = (noise_metadata_schedule_434_e6529).exp();
        let noise_metadata_schedule_434_e6531: f64 = (params.p63 * noise_metadata_schedule_434_e6530);
        (noise_metadata_schedule_434_e6531,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_434_e6533;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_435_e6554,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_435_e6542: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_435_e6544: f64 = (noise_metadata_schedule_435_e6542 * params.p5);
        let noise_metadata_schedule_435_e6546: f64 = (noise_variable_137).abs();
        let noise_metadata_schedule_435_e6547: f64 = (noise_metadata_schedule_435_e6544 * noise_metadata_schedule_435_e6546);
        let noise_metadata_schedule_435_e6549: f64 = { let limited_exp_arg = noise_variable_136; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_435_e6551: f64 = (noise_metadata_schedule_435_e6549 - 1.0);
        let noise_metadata_schedule_435_e6552: f64 = (noise_metadata_schedule_435_e6547 * noise_metadata_schedule_435_e6551);
        (noise_metadata_schedule_435_e6552,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_435_e6554;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_436_e6586,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_436_e6563: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_436_e6564: f64 = noise_metadata_schedule_436_e6563;
        let noise_metadata_schedule_436_e6568: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_436_e6569: f64 = noise_metadata_schedule_436_e6568;
        let noise_metadata_schedule_436_e6571: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_436_e6573: f64 = noise_metadata_schedule_436_e6571;
        let noise_metadata_schedule_436_e6575: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_436_e6577: f64 = noise_metadata_schedule_436_e6575;
        let noise_metadata_schedule_436_e6578: f64 = (noise_metadata_schedule_436_e6573 * noise_metadata_schedule_436_e6577);
        let noise_metadata_schedule_436_e6580: f64 = (noise_metadata_schedule_436_e6578 + 0.001);
        let noise_metadata_schedule_436_e6581: f64 = (noise_metadata_schedule_436_e6580).sqrt();
        let noise_metadata_schedule_436_e6582: f64 = (noise_metadata_schedule_436_e6569 - noise_metadata_schedule_436_e6581);
        let noise_metadata_schedule_436_e6583: f64 = (0.5 * noise_metadata_schedule_436_e6582);
        let noise_metadata_schedule_436_e6584: f64 = (noise_metadata_schedule_436_e6564 - noise_metadata_schedule_436_e6583);
        (noise_metadata_schedule_436_e6584,)
    } else {
        (noise_variable_321,)
    }
};
            noise_variable_321 = noise_metadata_schedule_436_e6586;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_437_e6597,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_437_e6595: f64 = (noise_variable_321 / params.p1);
        (noise_metadata_schedule_437_e6595,)
    } else {
        (noise_variable_322,)
    }
};
            noise_variable_322 = noise_metadata_schedule_437_e6597;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_438_e6609,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_438_e6605: f64 = (noise_variable_321).sqrt();
        let noise_metadata_schedule_438_e6607: f64 = (noise_metadata_schedule_438_e6605 + params.p69);
        (noise_metadata_schedule_438_e6607,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_438_e6609;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_439_e6624,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_439_e6619: f64 = (noise_variable_330 * 8.617087e-5);
        let noise_metadata_schedule_439_e6621: f64 = (noise_metadata_schedule_439_e6619 * noise_variable_35);
        let noise_metadata_schedule_439_e6622: f64 = (noise_variable_136 / noise_metadata_schedule_439_e6621);
        (noise_metadata_schedule_439_e6622,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_439_e6624;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_440_e6642,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_440_e6635: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_440_e6637: f64 = (noise_metadata_schedule_440_e6635 - 1.0);
        let noise_metadata_schedule_440_e6638: f64 = (params.p73 * noise_metadata_schedule_440_e6637);
        let noise_metadata_schedule_440_e6639: f64 = (noise_metadata_schedule_440_e6638).exp();
        let noise_metadata_schedule_440_e6640: f64 = (params.p65 * noise_metadata_schedule_440_e6639);
        (noise_metadata_schedule_440_e6640,)
    } else {
        (noise_variable_324,)
    }
};
            noise_variable_324 = noise_metadata_schedule_440_e6642;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_441_e6660,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_441_e6653: f64 = (noise_variable_322 * noise_variable_324);
        let noise_metadata_schedule_441_e6655: f64 = { let limited_exp_arg = noise_variable_90; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_441_e6656: f64 = (noise_metadata_schedule_441_e6653 * noise_metadata_schedule_441_e6655);
        let noise_metadata_schedule_441_e6657: f64 = (1.0 + noise_metadata_schedule_441_e6656);
        let noise_metadata_schedule_441_e6658: f64 = (noise_variable_206 * noise_metadata_schedule_441_e6657);
        (noise_metadata_schedule_441_e6658,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_441_e6660;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 6) {
            let (noise_metadata_schedule_442_e6677,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_442_e6670: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_442_e6672: f64 = (noise_metadata_schedule_442_e6670 - 1.0);
        let noise_metadata_schedule_442_e6674: f64 = (noise_metadata_schedule_442_e6672 * params.p76);
        let noise_metadata_schedule_442_e6675: f64 = (params.p68 + noise_metadata_schedule_442_e6674);
        (noise_metadata_schedule_442_e6675,)
    } else {
        (noise_variable_327,)
    }
};
            noise_variable_327 = noise_metadata_schedule_442_e6677;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_443_e6694,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_443_e6687: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_443_e6689: f64 = (noise_metadata_schedule_443_e6687 - 1.0);
        let noise_metadata_schedule_443_e6691: f64 = (noise_metadata_schedule_443_e6689 * params.p78);
        let noise_metadata_schedule_443_e6692: f64 = (params.p60 + noise_metadata_schedule_443_e6691);
        (noise_metadata_schedule_443_e6692,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_443_e6694;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 5 | 6) {
            let (noise_metadata_schedule_444_e6711,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_444_e6704: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_444_e6706: f64 = (noise_metadata_schedule_444_e6704 - 1.0);
        let noise_metadata_schedule_444_e6708: f64 = (noise_metadata_schedule_444_e6706 * params.p80);
        let noise_metadata_schedule_444_e6709: f64 = (params.p62 + noise_metadata_schedule_444_e6708);
        (noise_metadata_schedule_444_e6709,)
    } else {
        (noise_variable_331,)
    }
};
            noise_variable_331 = noise_metadata_schedule_444_e6711;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 6) {
            let (noise_metadata_schedule_445_e6728,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_445_e6720: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])) - noise_variable_327);
        let noise_metadata_schedule_445_e6723: f64 = (noise_variable_329 * 8.617087e-5);
        let noise_metadata_schedule_445_e6725: f64 = (noise_metadata_schedule_445_e6723 * noise_variable_35);
        let noise_metadata_schedule_445_e6726: f64 = (noise_metadata_schedule_445_e6720 / noise_metadata_schedule_445_e6725);
        (noise_metadata_schedule_445_e6726,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_445_e6728;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_446_e6746,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_446_e6739: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_446_e6741: f64 = (noise_metadata_schedule_446_e6739 - 1.0);
        let noise_metadata_schedule_446_e6742: f64 = (params.p72 * noise_metadata_schedule_446_e6741);
        let noise_metadata_schedule_446_e6743: f64 = (noise_metadata_schedule_446_e6742).exp();
        let noise_metadata_schedule_446_e6744: f64 = (params.p64 * noise_metadata_schedule_446_e6743);
        (noise_metadata_schedule_446_e6744,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_446_e6746;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_447_e6767,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_447_e6755: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_447_e6757: f64 = (noise_metadata_schedule_447_e6755 * params.p5);
        let noise_metadata_schedule_447_e6759: f64 = (noise_variable_137).abs();
        let noise_metadata_schedule_447_e6760: f64 = (noise_metadata_schedule_447_e6757 * noise_metadata_schedule_447_e6759);
        let noise_metadata_schedule_447_e6762: f64 = { let limited_exp_arg = noise_variable_136; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_447_e6764: f64 = (noise_metadata_schedule_447_e6762 - 1.0);
        let noise_metadata_schedule_447_e6765: f64 = (noise_metadata_schedule_447_e6760 * noise_metadata_schedule_447_e6764);
        (noise_metadata_schedule_447_e6765,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_447_e6767;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 6) {
            let (noise_metadata_schedule_448_e6799,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_448_e6776: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_448_e6777: f64 = noise_metadata_schedule_448_e6776;
        let noise_metadata_schedule_448_e6781: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_448_e6782: f64 = noise_metadata_schedule_448_e6781;
        let noise_metadata_schedule_448_e6784: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_448_e6786: f64 = noise_metadata_schedule_448_e6784;
        let noise_metadata_schedule_448_e6788: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_448_e6790: f64 = noise_metadata_schedule_448_e6788;
        let noise_metadata_schedule_448_e6791: f64 = (noise_metadata_schedule_448_e6786 * noise_metadata_schedule_448_e6790);
        let noise_metadata_schedule_448_e6793: f64 = (noise_metadata_schedule_448_e6791 + 0.001);
        let noise_metadata_schedule_448_e6794: f64 = (noise_metadata_schedule_448_e6793).sqrt();
        let noise_metadata_schedule_448_e6795: f64 = (noise_metadata_schedule_448_e6782 - noise_metadata_schedule_448_e6794);
        let noise_metadata_schedule_448_e6796: f64 = (0.5 * noise_metadata_schedule_448_e6795);
        let noise_metadata_schedule_448_e6797: f64 = (noise_metadata_schedule_448_e6777 - noise_metadata_schedule_448_e6796);
        (noise_metadata_schedule_448_e6797,)
    } else {
        (noise_variable_323,)
    }
};
            noise_variable_323 = noise_metadata_schedule_448_e6799;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_449_e6810,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_449_e6808: f64 = (noise_variable_323 / params.p1);
        (noise_metadata_schedule_449_e6808,)
    } else {
        (noise_variable_322,)
    }
};
            noise_variable_322 = noise_metadata_schedule_449_e6810;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 6) {
            let (noise_metadata_schedule_450_e6822,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_450_e6818: f64 = (noise_variable_323).sqrt();
        let noise_metadata_schedule_450_e6820: f64 = (noise_metadata_schedule_450_e6818 + params.p70);
        (noise_metadata_schedule_450_e6820,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_450_e6822;
        }
        if matches!(source_index, 1 | 2 | 3 | 4 | 6) {
            let (noise_metadata_schedule_451_e6837,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_451_e6832: f64 = (noise_variable_331 * 8.617087e-5);
        let noise_metadata_schedule_451_e6834: f64 = (noise_metadata_schedule_451_e6832 * noise_variable_35);
        let noise_metadata_schedule_451_e6835: f64 = (noise_variable_136 / noise_metadata_schedule_451_e6834);
        (noise_metadata_schedule_451_e6835,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_451_e6837;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_452_e6855,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_452_e6848: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_452_e6850: f64 = (noise_metadata_schedule_452_e6848 - 1.0);
        let noise_metadata_schedule_452_e6851: f64 = (params.p74 * noise_metadata_schedule_452_e6850);
        let noise_metadata_schedule_452_e6852: f64 = (noise_metadata_schedule_452_e6851).exp();
        let noise_metadata_schedule_452_e6853: f64 = (params.p66 * noise_metadata_schedule_452_e6852);
        (noise_metadata_schedule_452_e6853,)
    } else {
        (noise_variable_325,)
    }
};
            noise_variable_325 = noise_metadata_schedule_452_e6855;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_453_e6873,) = {
    if ((noise_variable_404 != 0.0) && (!((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)))) {
        let noise_metadata_schedule_453_e6866: f64 = (noise_variable_322 * noise_variable_325);
        let noise_metadata_schedule_453_e6868: f64 = { let limited_exp_arg = noise_variable_136; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let noise_metadata_schedule_453_e6869: f64 = (noise_metadata_schedule_453_e6866 * noise_metadata_schedule_453_e6868);
        let noise_metadata_schedule_453_e6870: f64 = (1.0 + noise_metadata_schedule_453_e6869);
        let noise_metadata_schedule_453_e6871: f64 = (noise_variable_207 * noise_metadata_schedule_453_e6870);
        (noise_metadata_schedule_453_e6871,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_453_e6873;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_454_e6892,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_454_e6885: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_454_e6887: f64 = (noise_metadata_schedule_454_e6885 - 1.0);
        let noise_metadata_schedule_454_e6889: f64 = (noise_metadata_schedule_454_e6887 * params.p75);
        let noise_metadata_schedule_454_e6890: f64 = (params.p67 + noise_metadata_schedule_454_e6889);
        (noise_metadata_schedule_454_e6890,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_454_e6892;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_455_e6911,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_455_e6904: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_455_e6906: f64 = (noise_metadata_schedule_455_e6904 - 1.0);
        let noise_metadata_schedule_455_e6908: f64 = (noise_metadata_schedule_455_e6906 * params.p77);
        let noise_metadata_schedule_455_e6909: f64 = (params.p57 + noise_metadata_schedule_455_e6908);
        (noise_metadata_schedule_455_e6909,)
    } else {
        (noise_variable_328,)
    }
};
            noise_variable_328 = noise_metadata_schedule_455_e6911;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_456_e6930,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_456_e6923: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_456_e6925: f64 = (noise_metadata_schedule_456_e6923 - 1.0);
        let noise_metadata_schedule_456_e6927: f64 = (noise_metadata_schedule_456_e6925 * params.p79);
        let noise_metadata_schedule_456_e6928: f64 = (params.p61 + noise_metadata_schedule_456_e6927);
        (noise_metadata_schedule_456_e6928,)
    } else {
        (noise_variable_330,)
    }
};
            noise_variable_330 = noise_metadata_schedule_456_e6930;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_457_e6950,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_457_e6943: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_457_e6945: f64 = (noise_metadata_schedule_457_e6943 - 1.0);
        let noise_metadata_schedule_457_e6946: f64 = (params.p73 * noise_metadata_schedule_457_e6945);
        let noise_metadata_schedule_457_e6947: f64 = (noise_metadata_schedule_457_e6946).exp();
        let noise_metadata_schedule_457_e6948: f64 = (params.p65 * noise_metadata_schedule_457_e6947);
        (noise_metadata_schedule_457_e6948,)
    } else {
        (noise_variable_324,)
    }
};
            noise_variable_324 = noise_metadata_schedule_457_e6950;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_458_e6976,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_458_e6961: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_458_e6963: f64 = (noise_metadata_schedule_458_e6961 * params.p5);
        let noise_metadata_schedule_458_e6965: f64 = (noise_metadata_schedule_458_e6963 * params.p63);
        let noise_metadata_schedule_458_e6969: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_458_e6971: f64 = (noise_metadata_schedule_458_e6969 - 1.0);
        let noise_metadata_schedule_458_e6972: f64 = (params.p71 * noise_metadata_schedule_458_e6971);
        let noise_metadata_schedule_458_e6973: f64 = (noise_metadata_schedule_458_e6972).exp();
        let noise_metadata_schedule_458_e6974: f64 = (noise_metadata_schedule_458_e6965 * noise_metadata_schedule_458_e6973);
        (noise_metadata_schedule_458_e6974,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_458_e6976;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_459_e6979: f64 = if noise_variable_137 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_407 = noise_metadata_schedule_459_e6979;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_460_e6982: f64 = if (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])) > 0.0 { 1.0 } else { 0.0 };
            noise_variable_408 = noise_metadata_schedule_460_e6982;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_461_e7003,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_408 != 0.0)) {
        let noise_metadata_schedule_461_e6997: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8]))).powf(params.p58);
        let noise_metadata_schedule_461_e7000: f64 = (noise_variable_328 * noise_variable_36);
        let noise_metadata_schedule_461_e7001: f64 = (noise_metadata_schedule_461_e6997 / noise_metadata_schedule_461_e7000);
        (noise_metadata_schedule_461_e7001,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_461_e7003;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_462_e7023,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_408 == 0.0)) {
        let noise_metadata_schedule_462_e7020: f64 = (noise_variable_328 * noise_variable_36);
        let noise_metadata_schedule_462_e7021: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])) / noise_metadata_schedule_462_e7020);
        (noise_metadata_schedule_462_e7021,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_462_e7023;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_463_e7026: f64 = if noise_variable_354 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_409 = noise_metadata_schedule_463_e7026;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_464_e7045,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_464_e7042: f64 = (noise_variable_354 - 80.0);
        let noise_metadata_schedule_464_e7043: f64 = (1.0 + noise_metadata_schedule_464_e7042);
        (noise_metadata_schedule_464_e7043,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_464_e7045;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_465_e7060,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_409 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_465_e7060;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_466_e7076,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_409 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_466_e7076;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_467_e7092,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) {
        let noise_metadata_schedule_467_e7089: f64 = (noise_variable_354).exp();
        let noise_metadata_schedule_467_e7090: f64 = (noise_variable_355 * noise_metadata_schedule_467_e7089);
        (noise_metadata_schedule_467_e7090,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_467_e7092;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_468_e7117,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) {
        let noise_metadata_schedule_468_e7106: f64 = (noise_variable_355 - 1.0);
        let noise_metadata_schedule_468_e7107: f64 = (noise_variable_137 * noise_metadata_schedule_468_e7106);
        let noise_metadata_schedule_468_e7109: f64 = (-noise_variable_326);
        let noise_metadata_schedule_468_e7112: f64 = (noise_variable_328 * noise_variable_36);
        let noise_metadata_schedule_468_e7113: f64 = (noise_metadata_schedule_468_e7109 / noise_metadata_schedule_468_e7112);
        let noise_metadata_schedule_468_e7114: f64 = (noise_metadata_schedule_468_e7113).exp();
        let noise_metadata_schedule_468_e7115: f64 = (noise_metadata_schedule_468_e7107 * noise_metadata_schedule_468_e7114);
        (noise_metadata_schedule_468_e7115,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_468_e7117;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_469_e7153,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) {
        let noise_metadata_schedule_469_e7130: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_469_e7131: f64 = noise_metadata_schedule_469_e7130;
        let noise_metadata_schedule_469_e7135: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_469_e7136: f64 = noise_metadata_schedule_469_e7135;
        let noise_metadata_schedule_469_e7138: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_469_e7140: f64 = noise_metadata_schedule_469_e7138;
        let noise_metadata_schedule_469_e7142: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_469_e7144: f64 = noise_metadata_schedule_469_e7142;
        let noise_metadata_schedule_469_e7145: f64 = (noise_metadata_schedule_469_e7140 * noise_metadata_schedule_469_e7144);
        let noise_metadata_schedule_469_e7147: f64 = (noise_metadata_schedule_469_e7145 + 0.001);
        let noise_metadata_schedule_469_e7148: f64 = (noise_metadata_schedule_469_e7147).sqrt();
        let noise_metadata_schedule_469_e7149: f64 = (noise_metadata_schedule_469_e7136 - noise_metadata_schedule_469_e7148);
        let noise_metadata_schedule_469_e7150: f64 = (0.5 * noise_metadata_schedule_469_e7149);
        let noise_metadata_schedule_469_e7151: f64 = (noise_metadata_schedule_469_e7131 - noise_metadata_schedule_469_e7150);
        (noise_metadata_schedule_469_e7151,)
    } else {
        (noise_variable_356,)
    }
};
            noise_variable_356 = noise_metadata_schedule_469_e7153;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_470_e7173,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) {
        let noise_metadata_schedule_470_e7165: f64 = (noise_variable_356).sqrt();
        let noise_metadata_schedule_470_e7167: f64 = (noise_metadata_schedule_470_e7165 + params.p69);
        let noise_metadata_schedule_470_e7170: f64 = (noise_variable_330 * noise_variable_36);
        let noise_metadata_schedule_470_e7171: f64 = (noise_metadata_schedule_470_e7167 / noise_metadata_schedule_470_e7170);
        (noise_metadata_schedule_470_e7171,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_470_e7173;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_471_e7176: f64 = if noise_variable_357 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_410 = noise_metadata_schedule_471_e7176;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_472_e7195,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_410 != 0.0)) {
        let noise_metadata_schedule_472_e7192: f64 = (noise_variable_357 - 80.0);
        let noise_metadata_schedule_472_e7193: f64 = (1.0 + noise_metadata_schedule_472_e7192);
        (noise_metadata_schedule_472_e7193,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_472_e7195;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_473_e7210,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_410 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_473_e7210;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_474_e7226,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) && (noise_variable_410 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_474_e7226;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_475_e7248,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) {
        let noise_metadata_schedule_475_e7240: f64 = (noise_variable_356 * noise_variable_324);
        let noise_metadata_schedule_475_e7242: f64 = (noise_metadata_schedule_475_e7240 * noise_variable_358);
        let noise_metadata_schedule_475_e7244: f64 = (noise_variable_357).exp();
        let noise_metadata_schedule_475_e7245: f64 = (noise_metadata_schedule_475_e7242 * noise_metadata_schedule_475_e7244);
        let noise_metadata_schedule_475_e7246: f64 = (1.0 + noise_metadata_schedule_475_e7245);
        (noise_metadata_schedule_475_e7246,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_475_e7248;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_476_e7263,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 != 0.0)) {
        let noise_metadata_schedule_476_e7261: f64 = (noise_variable_206 * noise_variable_358);
        (noise_metadata_schedule_476_e7261,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_476_e7263;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_477_e7277,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_407 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_477_e7277;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_478_e7296,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_478_e7289: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_478_e7291: f64 = (noise_metadata_schedule_478_e7289 - 1.0);
        let noise_metadata_schedule_478_e7293: f64 = (noise_metadata_schedule_478_e7291 * params.p76);
        let noise_metadata_schedule_478_e7294: f64 = (params.p68 + noise_metadata_schedule_478_e7293);
        (noise_metadata_schedule_478_e7294,)
    } else {
        (noise_variable_327,)
    }
};
            noise_variable_327 = noise_metadata_schedule_478_e7296;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_479_e7315,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_479_e7308: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_479_e7310: f64 = (noise_metadata_schedule_479_e7308 - 1.0);
        let noise_metadata_schedule_479_e7312: f64 = (noise_metadata_schedule_479_e7310 * params.p78);
        let noise_metadata_schedule_479_e7313: f64 = (params.p60 + noise_metadata_schedule_479_e7312);
        (noise_metadata_schedule_479_e7313,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_479_e7315;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_480_e7334,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_480_e7327: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_480_e7329: f64 = (noise_metadata_schedule_480_e7327 - 1.0);
        let noise_metadata_schedule_480_e7331: f64 = (noise_metadata_schedule_480_e7329 * params.p80);
        let noise_metadata_schedule_480_e7332: f64 = (params.p62 + noise_metadata_schedule_480_e7331);
        (noise_metadata_schedule_480_e7332,)
    } else {
        (noise_variable_331,)
    }
};
            noise_variable_331 = noise_metadata_schedule_480_e7334;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_481_e7354,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_481_e7347: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_481_e7349: f64 = (noise_metadata_schedule_481_e7347 - 1.0);
        let noise_metadata_schedule_481_e7350: f64 = (params.p74 * noise_metadata_schedule_481_e7349);
        let noise_metadata_schedule_481_e7351: f64 = (noise_metadata_schedule_481_e7350).exp();
        let noise_metadata_schedule_481_e7352: f64 = (params.p66 * noise_metadata_schedule_481_e7351);
        (noise_metadata_schedule_481_e7352,)
    } else {
        (noise_variable_325,)
    }
};
            noise_variable_325 = noise_metadata_schedule_481_e7354;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_482_e7380,) = {
    if ((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) {
        let noise_metadata_schedule_482_e7365: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_482_e7367: f64 = (noise_metadata_schedule_482_e7365 * params.p5);
        let noise_metadata_schedule_482_e7369: f64 = (noise_metadata_schedule_482_e7367 * params.p64);
        let noise_metadata_schedule_482_e7373: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_482_e7375: f64 = (noise_metadata_schedule_482_e7373 - 1.0);
        let noise_metadata_schedule_482_e7376: f64 = (params.p72 * noise_metadata_schedule_482_e7375);
        let noise_metadata_schedule_482_e7377: f64 = (noise_metadata_schedule_482_e7376).exp();
        let noise_metadata_schedule_482_e7378: f64 = (noise_metadata_schedule_482_e7369 * noise_metadata_schedule_482_e7377);
        (noise_metadata_schedule_482_e7378,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_482_e7380;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_483_e7383: f64 = if noise_variable_137 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_411 = noise_metadata_schedule_483_e7383;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_484_e7386: f64 = if (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])) > 0.0 { 1.0 } else { 0.0 };
            noise_variable_412 = noise_metadata_schedule_484_e7386;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_485_e7407,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_412 != 0.0)) {
        let noise_metadata_schedule_485_e7401: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7]))).powf(params.p59);
        let noise_metadata_schedule_485_e7404: f64 = (noise_variable_329 * noise_variable_36);
        let noise_metadata_schedule_485_e7405: f64 = (noise_metadata_schedule_485_e7401 / noise_metadata_schedule_485_e7404);
        (noise_metadata_schedule_485_e7405,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_485_e7407;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_486_e7427,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_412 == 0.0)) {
        let noise_metadata_schedule_486_e7424: f64 = (noise_variable_329 * noise_variable_36);
        let noise_metadata_schedule_486_e7425: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])) / noise_metadata_schedule_486_e7424);
        (noise_metadata_schedule_486_e7425,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_486_e7427;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_487_e7430: f64 = if noise_variable_354 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_413 = noise_metadata_schedule_487_e7430;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_488_e7449,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_413 != 0.0)) {
        let noise_metadata_schedule_488_e7446: f64 = (noise_variable_354 - 80.0);
        let noise_metadata_schedule_488_e7447: f64 = (1.0 + noise_metadata_schedule_488_e7446);
        (noise_metadata_schedule_488_e7447,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_488_e7449;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_489_e7464,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_413 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_489_e7464;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_490_e7480,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_413 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_490_e7480;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_491_e7496,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) {
        let noise_metadata_schedule_491_e7493: f64 = (noise_variable_354).exp();
        let noise_metadata_schedule_491_e7494: f64 = (noise_variable_355 * noise_metadata_schedule_491_e7493);
        (noise_metadata_schedule_491_e7494,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_491_e7496;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_492_e7521,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) {
        let noise_metadata_schedule_492_e7510: f64 = (noise_variable_355 - 1.0);
        let noise_metadata_schedule_492_e7511: f64 = (noise_variable_137 * noise_metadata_schedule_492_e7510);
        let noise_metadata_schedule_492_e7513: f64 = (-noise_variable_327);
        let noise_metadata_schedule_492_e7516: f64 = (noise_variable_329 * noise_variable_36);
        let noise_metadata_schedule_492_e7517: f64 = (noise_metadata_schedule_492_e7513 / noise_metadata_schedule_492_e7516);
        let noise_metadata_schedule_492_e7518: f64 = (noise_metadata_schedule_492_e7517).exp();
        let noise_metadata_schedule_492_e7519: f64 = (noise_metadata_schedule_492_e7511 * noise_metadata_schedule_492_e7518);
        (noise_metadata_schedule_492_e7519,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_492_e7521;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_493_e7557,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) {
        let noise_metadata_schedule_493_e7534: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_493_e7535: f64 = noise_metadata_schedule_493_e7534;
        let noise_metadata_schedule_493_e7539: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_493_e7540: f64 = noise_metadata_schedule_493_e7539;
        let noise_metadata_schedule_493_e7542: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_493_e7544: f64 = noise_metadata_schedule_493_e7542;
        let noise_metadata_schedule_493_e7546: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_493_e7548: f64 = noise_metadata_schedule_493_e7546;
        let noise_metadata_schedule_493_e7549: f64 = (noise_metadata_schedule_493_e7544 * noise_metadata_schedule_493_e7548);
        let noise_metadata_schedule_493_e7551: f64 = (noise_metadata_schedule_493_e7549 + 0.001);
        let noise_metadata_schedule_493_e7552: f64 = (noise_metadata_schedule_493_e7551).sqrt();
        let noise_metadata_schedule_493_e7553: f64 = (noise_metadata_schedule_493_e7540 - noise_metadata_schedule_493_e7552);
        let noise_metadata_schedule_493_e7554: f64 = (0.5 * noise_metadata_schedule_493_e7553);
        let noise_metadata_schedule_493_e7555: f64 = (noise_metadata_schedule_493_e7535 - noise_metadata_schedule_493_e7554);
        (noise_metadata_schedule_493_e7555,)
    } else {
        (noise_variable_356,)
    }
};
            noise_variable_356 = noise_metadata_schedule_493_e7557;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_494_e7577,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) {
        let noise_metadata_schedule_494_e7569: f64 = (noise_variable_356).sqrt();
        let noise_metadata_schedule_494_e7571: f64 = (noise_metadata_schedule_494_e7569 + params.p70);
        let noise_metadata_schedule_494_e7574: f64 = (noise_variable_331 * noise_variable_36);
        let noise_metadata_schedule_494_e7575: f64 = (noise_metadata_schedule_494_e7571 / noise_metadata_schedule_494_e7574);
        (noise_metadata_schedule_494_e7575,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_494_e7577;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_495_e7580: f64 = if noise_variable_357 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_414 = noise_metadata_schedule_495_e7580;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_496_e7599,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_414 != 0.0)) {
        let noise_metadata_schedule_496_e7596: f64 = (noise_variable_357 - 80.0);
        let noise_metadata_schedule_496_e7597: f64 = (1.0 + noise_metadata_schedule_496_e7596);
        (noise_metadata_schedule_496_e7597,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_496_e7599;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_497_e7614,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_414 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_497_e7614;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_498_e7630,) = {
    if ((((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) && (noise_variable_414 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_498_e7630;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_499_e7652,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) {
        let noise_metadata_schedule_499_e7644: f64 = (noise_variable_356 * noise_variable_325);
        let noise_metadata_schedule_499_e7646: f64 = (noise_metadata_schedule_499_e7644 * noise_variable_358);
        let noise_metadata_schedule_499_e7648: f64 = (noise_variable_357).exp();
        let noise_metadata_schedule_499_e7649: f64 = (noise_metadata_schedule_499_e7646 * noise_metadata_schedule_499_e7648);
        let noise_metadata_schedule_499_e7650: f64 = (1.0 + noise_metadata_schedule_499_e7649);
        (noise_metadata_schedule_499_e7650,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_499_e7652;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_500_e7667,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 != 0.0)) {
        let noise_metadata_schedule_500_e7665: f64 = (noise_variable_207 * noise_variable_358);
        (noise_metadata_schedule_500_e7665,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_500_e7667;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_501_e7681,) = {
    if (((noise_variable_405 != 0.0) && (!(((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)))) && (noise_variable_411 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_501_e7681;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_502_e7702,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_502_e7695: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_502_e7697: f64 = (noise_metadata_schedule_502_e7695 - 1.0);
        let noise_metadata_schedule_502_e7699: f64 = (noise_metadata_schedule_502_e7697 * params.p75);
        let noise_metadata_schedule_502_e7700: f64 = (params.p67 + noise_metadata_schedule_502_e7699);
        (noise_metadata_schedule_502_e7700,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_502_e7702;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_503_e7723,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_503_e7716: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_503_e7718: f64 = (noise_metadata_schedule_503_e7716 - 1.0);
        let noise_metadata_schedule_503_e7720: f64 = (noise_metadata_schedule_503_e7718 * params.p77);
        let noise_metadata_schedule_503_e7721: f64 = (params.p57 + noise_metadata_schedule_503_e7720);
        (noise_metadata_schedule_503_e7721,)
    } else {
        (noise_variable_328,)
    }
};
            noise_variable_328 = noise_metadata_schedule_503_e7723;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_504_e7744,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_504_e7737: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_504_e7739: f64 = (noise_metadata_schedule_504_e7737 - 1.0);
        let noise_metadata_schedule_504_e7741: f64 = (noise_metadata_schedule_504_e7739 * params.p79);
        let noise_metadata_schedule_504_e7742: f64 = (params.p61 + noise_metadata_schedule_504_e7741);
        (noise_metadata_schedule_504_e7742,)
    } else {
        (noise_variable_330,)
    }
};
            noise_variable_330 = noise_metadata_schedule_504_e7744;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_505_e7772,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_505_e7757: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_505_e7759: f64 = (noise_metadata_schedule_505_e7757 * params.p5);
        let noise_metadata_schedule_505_e7761: f64 = (noise_metadata_schedule_505_e7759 * params.p65);
        let noise_metadata_schedule_505_e7765: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_505_e7767: f64 = (noise_metadata_schedule_505_e7765 - 1.0);
        let noise_metadata_schedule_505_e7768: f64 = (params.p73 * noise_metadata_schedule_505_e7767);
        let noise_metadata_schedule_505_e7769: f64 = (noise_metadata_schedule_505_e7768).exp();
        let noise_metadata_schedule_505_e7770: f64 = (noise_metadata_schedule_505_e7761 * noise_metadata_schedule_505_e7769);
        (noise_metadata_schedule_505_e7770,)
    } else {
        (noise_variable_324,)
    }
};
            noise_variable_324 = noise_metadata_schedule_505_e7772;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_506_e7800,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_506_e7785: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_506_e7787: f64 = (noise_metadata_schedule_506_e7785 * params.p5);
        let noise_metadata_schedule_506_e7789: f64 = (noise_metadata_schedule_506_e7787 * params.p63);
        let noise_metadata_schedule_506_e7793: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_506_e7795: f64 = (noise_metadata_schedule_506_e7793 - 1.0);
        let noise_metadata_schedule_506_e7796: f64 = (params.p71 * noise_metadata_schedule_506_e7795);
        let noise_metadata_schedule_506_e7797: f64 = (noise_metadata_schedule_506_e7796).exp();
        let noise_metadata_schedule_506_e7798: f64 = (noise_metadata_schedule_506_e7789 * noise_metadata_schedule_506_e7797);
        (noise_metadata_schedule_506_e7798,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_506_e7800;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_507_e7803: f64 = if noise_variable_137 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_415 = noise_metadata_schedule_507_e7803;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_508_e7806: f64 = if (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])) > 0.0 { 1.0 } else { 0.0 };
            noise_variable_416 = noise_metadata_schedule_508_e7806;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_509_e7829,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_416 != 0.0)) {
        let noise_metadata_schedule_509_e7823: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8]))).powf(params.p58);
        let noise_metadata_schedule_509_e7826: f64 = (noise_variable_328 * noise_variable_36);
        let noise_metadata_schedule_509_e7827: f64 = (noise_metadata_schedule_509_e7823 / noise_metadata_schedule_509_e7826);
        (noise_metadata_schedule_509_e7827,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_509_e7829;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_510_e7851,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_416 == 0.0)) {
        let noise_metadata_schedule_510_e7848: f64 = (noise_variable_328 * noise_variable_36);
        let noise_metadata_schedule_510_e7849: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])) / noise_metadata_schedule_510_e7848);
        (noise_metadata_schedule_510_e7849,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_510_e7851;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_511_e7854: f64 = if noise_variable_354 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_417 = noise_metadata_schedule_511_e7854;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_512_e7875,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_417 != 0.0)) {
        let noise_metadata_schedule_512_e7872: f64 = (noise_variable_354 - 80.0);
        let noise_metadata_schedule_512_e7873: f64 = (1.0 + noise_metadata_schedule_512_e7872);
        (noise_metadata_schedule_512_e7873,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_512_e7875;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_513_e7892,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_417 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_513_e7892;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_514_e7910,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_417 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_514_e7910;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_515_e7928,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_515_e7925: f64 = (noise_variable_354).exp();
        let noise_metadata_schedule_515_e7926: f64 = (noise_variable_355 * noise_metadata_schedule_515_e7925);
        (noise_metadata_schedule_515_e7926,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_515_e7928;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_516_e7955,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_516_e7944: f64 = (noise_variable_355 - 1.0);
        let noise_metadata_schedule_516_e7945: f64 = (noise_variable_137 * noise_metadata_schedule_516_e7944);
        let noise_metadata_schedule_516_e7947: f64 = (-noise_variable_326);
        let noise_metadata_schedule_516_e7950: f64 = (noise_variable_328 * noise_variable_36);
        let noise_metadata_schedule_516_e7951: f64 = (noise_metadata_schedule_516_e7947 / noise_metadata_schedule_516_e7950);
        let noise_metadata_schedule_516_e7952: f64 = (noise_metadata_schedule_516_e7951).exp();
        let noise_metadata_schedule_516_e7953: f64 = (noise_metadata_schedule_516_e7945 * noise_metadata_schedule_516_e7952);
        (noise_metadata_schedule_516_e7953,)
    } else {
        (noise_variable_380,)
    }
};
            noise_variable_380 = noise_metadata_schedule_516_e7955;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_517_e7993,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_517_e7970: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_517_e7971: f64 = noise_metadata_schedule_517_e7970;
        let noise_metadata_schedule_517_e7975: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_517_e7976: f64 = noise_metadata_schedule_517_e7975;
        let noise_metadata_schedule_517_e7978: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_517_e7980: f64 = noise_metadata_schedule_517_e7978;
        let noise_metadata_schedule_517_e7982: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[8])));
        let noise_metadata_schedule_517_e7984: f64 = noise_metadata_schedule_517_e7982;
        let noise_metadata_schedule_517_e7985: f64 = (noise_metadata_schedule_517_e7980 * noise_metadata_schedule_517_e7984);
        let noise_metadata_schedule_517_e7987: f64 = noise_metadata_schedule_517_e7985;
        let noise_metadata_schedule_517_e7988: f64 = (noise_metadata_schedule_517_e7987).sqrt();
        let noise_metadata_schedule_517_e7989: f64 = (noise_metadata_schedule_517_e7976 - noise_metadata_schedule_517_e7988);
        let noise_metadata_schedule_517_e7990: f64 = (0.5 * noise_metadata_schedule_517_e7989);
        let noise_metadata_schedule_517_e7991: f64 = (noise_metadata_schedule_517_e7971 - noise_metadata_schedule_517_e7990);
        (noise_metadata_schedule_517_e7991,)
    } else {
        (noise_variable_356,)
    }
};
            noise_variable_356 = noise_metadata_schedule_517_e7993;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_518_e8015,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_518_e8007: f64 = (noise_variable_356).sqrt();
        let noise_metadata_schedule_518_e8009: f64 = (noise_metadata_schedule_518_e8007 + params.p69);
        let noise_metadata_schedule_518_e8012: f64 = (noise_variable_330 * noise_variable_36);
        let noise_metadata_schedule_518_e8013: f64 = (noise_metadata_schedule_518_e8009 / noise_metadata_schedule_518_e8012);
        (noise_metadata_schedule_518_e8013,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_518_e8015;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_519_e8018: f64 = if noise_variable_357 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_418 = noise_metadata_schedule_519_e8018;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_520_e8039,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_418 != 0.0)) {
        let noise_metadata_schedule_520_e8036: f64 = (noise_variable_357 - 80.0);
        let noise_metadata_schedule_520_e8037: f64 = (1.0 + noise_metadata_schedule_520_e8036);
        (noise_metadata_schedule_520_e8037,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_520_e8039;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_521_e8056,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_418 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_521_e8056;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_522_e8074,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) && (noise_variable_418 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_522_e8074;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_523_e8092,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_523_e8089: f64 = (noise_variable_357).exp();
        let noise_metadata_schedule_523_e8090: f64 = (noise_variable_358 * noise_metadata_schedule_523_e8089);
        (noise_metadata_schedule_523_e8090,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_523_e8092;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_524_e8116,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_524_e8110: f64 = (noise_variable_330 * noise_variable_36);
        let noise_metadata_schedule_524_e8111: f64 = (params.p69 / noise_metadata_schedule_524_e8110);
        let noise_metadata_schedule_524_e8112: f64 = (noise_metadata_schedule_524_e8111).exp();
        let noise_metadata_schedule_524_e8113: f64 = (noise_variable_358 - noise_metadata_schedule_524_e8112);
        let noise_metadata_schedule_524_e8114: f64 = (noise_variable_324 * noise_metadata_schedule_524_e8113);
        (noise_metadata_schedule_524_e8114,)
    } else {
        (noise_variable_381,)
    }
};
            noise_variable_381 = noise_metadata_schedule_524_e8116;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_525_e8133,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_525_e8131: f64 = (noise_variable_380 - noise_variable_381);
        (noise_metadata_schedule_525_e8131,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_525_e8133;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_526_e8149,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_415 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_526_e8149;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_527_e8170,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_527_e8163: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_527_e8165: f64 = (noise_metadata_schedule_527_e8163 - 1.0);
        let noise_metadata_schedule_527_e8167: f64 = (noise_metadata_schedule_527_e8165 * params.p76);
        let noise_metadata_schedule_527_e8168: f64 = (params.p68 + noise_metadata_schedule_527_e8167);
        (noise_metadata_schedule_527_e8168,)
    } else {
        (noise_variable_327,)
    }
};
            noise_variable_327 = noise_metadata_schedule_527_e8170;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_528_e8191,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_528_e8184: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_528_e8186: f64 = (noise_metadata_schedule_528_e8184 - 1.0);
        let noise_metadata_schedule_528_e8188: f64 = (noise_metadata_schedule_528_e8186 * params.p78);
        let noise_metadata_schedule_528_e8189: f64 = (params.p60 + noise_metadata_schedule_528_e8188);
        (noise_metadata_schedule_528_e8189,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_528_e8191;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_529_e8212,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_529_e8205: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_529_e8207: f64 = (noise_metadata_schedule_529_e8205 - 1.0);
        let noise_metadata_schedule_529_e8209: f64 = (noise_metadata_schedule_529_e8207 * params.p80);
        let noise_metadata_schedule_529_e8210: f64 = (params.p62 + noise_metadata_schedule_529_e8209);
        (noise_metadata_schedule_529_e8210,)
    } else {
        (noise_variable_331,)
    }
};
            noise_variable_331 = noise_metadata_schedule_529_e8212;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_530_e8240,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_530_e8225: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_530_e8227: f64 = (noise_metadata_schedule_530_e8225 * params.p5);
        let noise_metadata_schedule_530_e8229: f64 = (noise_metadata_schedule_530_e8227 * params.p66);
        let noise_metadata_schedule_530_e8233: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_530_e8235: f64 = (noise_metadata_schedule_530_e8233 - 1.0);
        let noise_metadata_schedule_530_e8236: f64 = (params.p74 * noise_metadata_schedule_530_e8235);
        let noise_metadata_schedule_530_e8237: f64 = (noise_metadata_schedule_530_e8236).exp();
        let noise_metadata_schedule_530_e8238: f64 = (noise_metadata_schedule_530_e8229 * noise_metadata_schedule_530_e8237);
        (noise_metadata_schedule_530_e8238,)
    } else {
        (noise_variable_325,)
    }
};
            noise_variable_325 = noise_metadata_schedule_530_e8240;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_531_e8268,) = {
    if ((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) {
        let noise_metadata_schedule_531_e8253: f64 = (params.p4 * params.p3);
        let noise_metadata_schedule_531_e8255: f64 = (noise_metadata_schedule_531_e8253 * params.p5);
        let noise_metadata_schedule_531_e8257: f64 = (noise_metadata_schedule_531_e8255 * params.p64);
        let noise_metadata_schedule_531_e8261: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_531_e8263: f64 = (noise_metadata_schedule_531_e8261 - 1.0);
        let noise_metadata_schedule_531_e8264: f64 = (params.p72 * noise_metadata_schedule_531_e8263);
        let noise_metadata_schedule_531_e8265: f64 = (noise_metadata_schedule_531_e8264).exp();
        let noise_metadata_schedule_531_e8266: f64 = (noise_metadata_schedule_531_e8257 * noise_metadata_schedule_531_e8265);
        (noise_metadata_schedule_531_e8266,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_531_e8268;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_532_e8271: f64 = if noise_variable_137 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_419 = noise_metadata_schedule_532_e8271;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_533_e8274: f64 = if (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])) > 0.0 { 1.0 } else { 0.0 };
            noise_variable_420 = noise_metadata_schedule_533_e8274;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_534_e8297,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_420 != 0.0)) {
        let noise_metadata_schedule_534_e8291: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7]))).powf(params.p59);
        let noise_metadata_schedule_534_e8294: f64 = (noise_variable_329 * noise_variable_36);
        let noise_metadata_schedule_534_e8295: f64 = (noise_metadata_schedule_534_e8291 / noise_metadata_schedule_534_e8294);
        (noise_metadata_schedule_534_e8295,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_534_e8297;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_535_e8319,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_420 == 0.0)) {
        let noise_metadata_schedule_535_e8316: f64 = (noise_variable_329 * noise_variable_36);
        let noise_metadata_schedule_535_e8317: f64 = ((ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])) / noise_metadata_schedule_535_e8316);
        (noise_metadata_schedule_535_e8317,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_535_e8319;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_536_e8322: f64 = if noise_variable_354 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_421 = noise_metadata_schedule_536_e8322;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_537_e8343,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_421 != 0.0)) {
        let noise_metadata_schedule_537_e8340: f64 = (noise_variable_354 - 80.0);
        let noise_metadata_schedule_537_e8341: f64 = (1.0 + noise_metadata_schedule_537_e8340);
        (noise_metadata_schedule_537_e8341,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_537_e8343;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_538_e8360,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_421 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_538_e8360;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_539_e8378,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_421 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_539_e8378;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_540_e8396,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_540_e8393: f64 = (noise_variable_354).exp();
        let noise_metadata_schedule_540_e8394: f64 = (noise_variable_355 * noise_metadata_schedule_540_e8393);
        (noise_metadata_schedule_540_e8394,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_540_e8396;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_541_e8423,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_541_e8412: f64 = (noise_variable_355 - 1.0);
        let noise_metadata_schedule_541_e8413: f64 = (noise_variable_137 * noise_metadata_schedule_541_e8412);
        let noise_metadata_schedule_541_e8415: f64 = (-noise_variable_327);
        let noise_metadata_schedule_541_e8418: f64 = (noise_variable_329 * noise_variable_36);
        let noise_metadata_schedule_541_e8419: f64 = (noise_metadata_schedule_541_e8415 / noise_metadata_schedule_541_e8418);
        let noise_metadata_schedule_541_e8420: f64 = (noise_metadata_schedule_541_e8419).exp();
        let noise_metadata_schedule_541_e8421: f64 = (noise_metadata_schedule_541_e8413 * noise_metadata_schedule_541_e8420);
        (noise_metadata_schedule_541_e8421,)
    } else {
        (noise_variable_380,)
    }
};
            noise_variable_380 = noise_metadata_schedule_541_e8423;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_542_e8461,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_542_e8438: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_542_e8439: f64 = noise_metadata_schedule_542_e8438;
        let noise_metadata_schedule_542_e8443: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_542_e8444: f64 = noise_metadata_schedule_542_e8443;
        let noise_metadata_schedule_542_e8446: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_542_e8448: f64 = noise_metadata_schedule_542_e8446;
        let noise_metadata_schedule_542_e8450: f64 = (-(ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[7])));
        let noise_metadata_schedule_542_e8452: f64 = noise_metadata_schedule_542_e8450;
        let noise_metadata_schedule_542_e8453: f64 = (noise_metadata_schedule_542_e8448 * noise_metadata_schedule_542_e8452);
        let noise_metadata_schedule_542_e8455: f64 = noise_metadata_schedule_542_e8453;
        let noise_metadata_schedule_542_e8456: f64 = (noise_metadata_schedule_542_e8455).sqrt();
        let noise_metadata_schedule_542_e8457: f64 = (noise_metadata_schedule_542_e8444 - noise_metadata_schedule_542_e8456);
        let noise_metadata_schedule_542_e8458: f64 = (0.5 * noise_metadata_schedule_542_e8457);
        let noise_metadata_schedule_542_e8459: f64 = (noise_metadata_schedule_542_e8439 - noise_metadata_schedule_542_e8458);
        (noise_metadata_schedule_542_e8459,)
    } else {
        (noise_variable_356,)
    }
};
            noise_variable_356 = noise_metadata_schedule_542_e8461;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_543_e8483,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_543_e8475: f64 = (noise_variable_356).sqrt();
        let noise_metadata_schedule_543_e8477: f64 = (noise_metadata_schedule_543_e8475 + params.p70);
        let noise_metadata_schedule_543_e8480: f64 = (noise_variable_331 * noise_variable_36);
        let noise_metadata_schedule_543_e8481: f64 = (noise_metadata_schedule_543_e8477 / noise_metadata_schedule_543_e8480);
        (noise_metadata_schedule_543_e8481,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_543_e8483;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_544_e8486: f64 = if noise_variable_357 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_422 = noise_metadata_schedule_544_e8486;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_545_e8507,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_422 != 0.0)) {
        let noise_metadata_schedule_545_e8504: f64 = (noise_variable_357 - 80.0);
        let noise_metadata_schedule_545_e8505: f64 = (1.0 + noise_metadata_schedule_545_e8504);
        (noise_metadata_schedule_545_e8505,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_545_e8507;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_546_e8524,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_422 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_546_e8524;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_547_e8542,) = {
    if ((((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_547_e8542;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_548_e8560,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_548_e8557: f64 = (noise_variable_357).exp();
        let noise_metadata_schedule_548_e8558: f64 = (noise_variable_358 * noise_metadata_schedule_548_e8557);
        (noise_metadata_schedule_548_e8558,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_548_e8560;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_549_e8584,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_549_e8578: f64 = (noise_variable_331 * noise_variable_36);
        let noise_metadata_schedule_549_e8579: f64 = (params.p70 / noise_metadata_schedule_549_e8578);
        let noise_metadata_schedule_549_e8580: f64 = (noise_metadata_schedule_549_e8579).exp();
        let noise_metadata_schedule_549_e8581: f64 = (noise_variable_358 - noise_metadata_schedule_549_e8580);
        let noise_metadata_schedule_549_e8582: f64 = (noise_variable_325 * noise_metadata_schedule_549_e8581);
        (noise_metadata_schedule_549_e8582,)
    } else {
        (noise_variable_381,)
    }
};
            noise_variable_381 = noise_metadata_schedule_549_e8584;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_550_e8601,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_550_e8599: f64 = (noise_variable_380 - noise_variable_381);
        (noise_metadata_schedule_550_e8599,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_550_e8601;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_551_e8617,) = {
    if (((noise_variable_406 != 0.0) && (!((((noise_variable_402 != 0.0) || (noise_variable_403 != 0.0)) || (noise_variable_404 != 0.0)) || (noise_variable_405 != 0.0)))) && (noise_variable_419 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_551_e8617;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_metadata_schedule_553_e8622: f64 = if self.param_given[45] { 1.0 } else { 0.0 };
            noise_variable_359 = noise_metadata_schedule_553_e8622;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_554_e8624: f64 = if self.param_given[44] { 1.0 } else { 0.0 };
            noise_variable_360 = noise_metadata_schedule_554_e8624;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            noise_variable_187 = noise_variable_154;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_metadata_schedule_556_e8628: f64 = if noise_variable_361 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_424 = noise_metadata_schedule_556_e8628;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_557_e8654,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_557_e8635: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_557_e8637: f64 = (noise_metadata_schedule_557_e8635 - 1.0);
        let noise_metadata_schedule_557_e8638: f64 = (params.p50 * noise_metadata_schedule_557_e8637);
        let noise_metadata_schedule_557_e8639: f64 = (1.0 - noise_metadata_schedule_557_e8638);
        let noise_metadata_schedule_557_e8640: f64 = (params.p36 * noise_metadata_schedule_557_e8639);
        let noise_metadata_schedule_557_e8642: f64 = (noise_metadata_schedule_557_e8640 - noise_variable_340);
        let noise_metadata_schedule_557_e8644: f64 = (noise_metadata_schedule_557_e8642 - noise_variable_365);
        let noise_metadata_schedule_557_e8647: f64 = (params.p12 / 1.602176634e-19);
        let noise_metadata_schedule_557_e8649: f64 = (noise_metadata_schedule_557_e8647 * noise_variable_45);
        let noise_metadata_schedule_557_e8651: f64 = (noise_metadata_schedule_557_e8649 * noise_variable_81);
        let noise_metadata_schedule_557_e8652: f64 = (noise_metadata_schedule_557_e8644 + noise_metadata_schedule_557_e8651);
        (noise_metadata_schedule_557_e8652,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_557_e8654;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_558_e8677,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_558_e8658: f64 = (1.0 + noise_variable_177);
        let noise_metadata_schedule_558_e8662: f64 = (1.0 + noise_variable_177);
        let noise_metadata_schedule_558_e8665: f64 = (noise_variable_177 - 1.0);
        let noise_metadata_schedule_558_e8668: f64 = (noise_variable_177 - 1.0);
        let noise_metadata_schedule_558_e8669: f64 = (noise_metadata_schedule_558_e8665 * noise_metadata_schedule_558_e8668);
        let noise_metadata_schedule_558_e8671: f64 = (noise_metadata_schedule_558_e8669 + 0.001);
        let noise_metadata_schedule_558_e8672: f64 = (noise_metadata_schedule_558_e8671).sqrt();
        let noise_metadata_schedule_558_e8673: f64 = (noise_metadata_schedule_558_e8662 - noise_metadata_schedule_558_e8672);
        let noise_metadata_schedule_558_e8674: f64 = (0.5 * noise_metadata_schedule_558_e8673);
        let noise_metadata_schedule_558_e8675: f64 = (noise_metadata_schedule_558_e8658 - noise_metadata_schedule_558_e8674);
        (noise_metadata_schedule_558_e8675,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_558_e8677;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_559_e8689,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_559_e8681: f64 = (1.602176634e-19 * noise_variable_177);
        let noise_metadata_schedule_559_e8685: f64 = (params.p38 * noise_variable_187);
        let noise_metadata_schedule_559_e8686: f64 = (1.0 + noise_metadata_schedule_559_e8685);
        let noise_metadata_schedule_559_e8687: f64 = (noise_metadata_schedule_559_e8681 * noise_metadata_schedule_559_e8686);
        (noise_metadata_schedule_559_e8687,)
    } else {
        (noise_variable_172,)
    }
};
            noise_variable_172 = noise_metadata_schedule_559_e8689;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_560_e8699,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_560_e8694: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_560_e8696: f64 = (noise_metadata_schedule_560_e8694).powf(params.p51);
        let noise_metadata_schedule_560_e8697: f64 = (params.p35 * noise_metadata_schedule_560_e8696);
        (noise_metadata_schedule_560_e8697,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_560_e8699;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_561_e8709,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_561_e8703: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_561_e8705: f64 = (noise_metadata_schedule_561_e8703 * noise_variable_172);
        let noise_metadata_schedule_561_e8707: f64 = (noise_metadata_schedule_561_e8705 * noise_variable_176);
        (noise_metadata_schedule_561_e8707,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_561_e8709;
        }
        if matches!(source_index, 2 | 4) {
            let (noise_metadata_schedule_562_e8719,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_562_e8714: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_562_e8716: f64 = (noise_metadata_schedule_562_e8714).powf(params.p52);
        let noise_metadata_schedule_562_e8717: f64 = (params.p40 * noise_metadata_schedule_562_e8716);
        (noise_metadata_schedule_562_e8717,)
    } else {
        (noise_variable_180,)
    }
};
            noise_variable_180 = noise_metadata_schedule_562_e8719;
        }
        if matches!(source_index, 2 | 4) {
            let (noise_metadata_schedule_563_e8731,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_563_e8724: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_563_e8726: f64 = (noise_metadata_schedule_563_e8724 * noise_variable_172);
        let noise_metadata_schedule_563_e8728: f64 = (noise_metadata_schedule_563_e8726 * noise_variable_180);
        let noise_metadata_schedule_563_e8729: f64 = (params.p46 / noise_metadata_schedule_563_e8728);
        (noise_metadata_schedule_563_e8729,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_563_e8731;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let noise_metadata_schedule_564_e8734: f64 = if noise_variable_359 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_425 = noise_metadata_schedule_564_e8734;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_565_e8742,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_565_e8740: f64 = (1.0 + params.p45);
        (noise_metadata_schedule_565_e8740,)
    } else {
        (noise_variable_350,)
    }
};
            noise_variable_350 = noise_metadata_schedule_565_e8742;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_566_e8751,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_566_e8747: f64 = (noise_variable_350).sqrt();
        let noise_metadata_schedule_566_e8749: f64 = (noise_metadata_schedule_566_e8747 * noise_variable_94);
        (noise_metadata_schedule_566_e8749,)
    } else {
        (noise_variable_351,)
    }
};
            noise_variable_351 = noise_metadata_schedule_566_e8751;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_567_e8759,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_567_e8757: f64 = (noise_variable_351 / noise_variable_173);
        (noise_metadata_schedule_567_e8757,)
    } else {
        (noise_variable_352,)
    }
};
            noise_variable_352 = noise_metadata_schedule_567_e8759;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_568_e8767,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_568_e8765: f64 = (noise_variable_352 * 2.0);
        (noise_metadata_schedule_568_e8765,)
    } else {
        (noise_variable_353,)
    }
};
            noise_variable_353 = noise_metadata_schedule_568_e8767;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_569_e8777,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_569_e8774: f64 = (noise_variable_352 * noise_variable_352);
        let noise_metadata_schedule_569_e8775: f64 = (noise_variable_350 + noise_metadata_schedule_569_e8774);
        (noise_metadata_schedule_569_e8775,)
    } else {
        (noise_variable_350,)
    }
};
            noise_variable_350 = noise_metadata_schedule_569_e8777;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_570_e8791,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_570_e8783: f64 = (noise_variable_350 - noise_variable_353);
        let noise_metadata_schedule_570_e8784: f64 = (noise_metadata_schedule_570_e8783).sqrt();
        let noise_metadata_schedule_570_e8787: f64 = (noise_variable_350 + noise_variable_353);
        let noise_metadata_schedule_570_e8788: f64 = (noise_metadata_schedule_570_e8787).sqrt();
        let noise_metadata_schedule_570_e8789: f64 = (noise_metadata_schedule_570_e8784 + noise_metadata_schedule_570_e8788);
        (noise_metadata_schedule_570_e8789,)
    } else {
        (noise_variable_350,)
    }
};
            noise_variable_350 = noise_metadata_schedule_570_e8791;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_571_e8801,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_571_e8797: f64 = (noise_variable_351 * 2.0);
        let noise_metadata_schedule_571_e8799: f64 = (noise_metadata_schedule_571_e8797 / noise_variable_350);
        (noise_metadata_schedule_571_e8799,)
    } else {
        (noise_variable_349,)
    }
};
            noise_variable_349 = noise_metadata_schedule_571_e8801;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_572_e8811,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_572_e8808: f64 = (noise_variable_349 / noise_variable_173);
        let noise_metadata_schedule_572_e8809: f64 = (1.0 - noise_metadata_schedule_572_e8808);
        (noise_metadata_schedule_572_e8809,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_572_e8811;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_573_e8821,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 == 0.0)) {
        let noise_metadata_schedule_573_e8818: f64 = (noise_variable_94 / noise_variable_173);
        let noise_metadata_schedule_573_e8819: f64 = (noise_metadata_schedule_573_e8818).abs();
        (noise_metadata_schedule_573_e8819,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_573_e8821;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_574_e8856,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 == 0.0)) {
        let noise_metadata_schedule_574_e8829: f64 = (noise_variable_182 + 0.9);
        let noise_metadata_schedule_574_e8832: f64 = (noise_variable_182 - 0.9);
        let noise_metadata_schedule_574_e8835: f64 = (noise_variable_182 - 0.9);
        let noise_metadata_schedule_574_e8836: f64 = (noise_metadata_schedule_574_e8832 * noise_metadata_schedule_574_e8835);
        let noise_metadata_schedule_574_e8839: f64 = (0.1 * 0.1);
        let noise_metadata_schedule_574_e8840: f64 = (noise_metadata_schedule_574_e8836 + noise_metadata_schedule_574_e8839);
        let noise_metadata_schedule_574_e8841: f64 = (noise_metadata_schedule_574_e8840).sqrt();
        let noise_metadata_schedule_574_e8842: f64 = (noise_metadata_schedule_574_e8829 - noise_metadata_schedule_574_e8841);
        let noise_metadata_schedule_574_e8846: f64 = (0.9 * 0.9);
        let noise_metadata_schedule_574_e8849: f64 = (0.1 * 0.1);
        let noise_metadata_schedule_574_e8850: f64 = (noise_metadata_schedule_574_e8846 + noise_metadata_schedule_574_e8849);
        let noise_metadata_schedule_574_e8851: f64 = (noise_metadata_schedule_574_e8850).sqrt();
        let noise_metadata_schedule_574_e8852: f64 = (0.9 - noise_metadata_schedule_574_e8851);
        let noise_metadata_schedule_574_e8853: f64 = (noise_metadata_schedule_574_e8842 - noise_metadata_schedule_574_e8852);
        let noise_metadata_schedule_574_e8854: f64 = (0.5 * noise_metadata_schedule_574_e8853);
        (noise_metadata_schedule_574_e8854,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_574_e8856;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_575_e8865,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 == 0.0)) {
        let noise_metadata_schedule_575_e8863: f64 = (noise_variable_183).powf(params.p42);
        (noise_metadata_schedule_575_e8863,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_575_e8865;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_576_e8874,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 == 0.0)) {
        let noise_metadata_schedule_576_e8872: f64 = (1.0 - noise_variable_136);
        (noise_metadata_schedule_576_e8872,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_576_e8874;
        }
        if matches!(source_index, 1 | 2 | 3 | 4) {
            let (noise_metadata_schedule_577_e8885,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_425 == 0.0)) {
        let noise_metadata_schedule_577_e8882: f64 = (1.0 / params.p42);
        let noise_metadata_schedule_577_e8883: f64 = (noise_variable_90).powf(noise_metadata_schedule_577_e8882);
        (noise_metadata_schedule_577_e8883,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_577_e8885;
        }
        if matches!(source_index, 2 | 4) {
            let (noise_metadata_schedule_578_e8891,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_578_e8889: f64 = (noise_variable_175 / noise_variable_91);
        (noise_metadata_schedule_578_e8889,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_578_e8891;
        }
        if matches!(source_index, 2 | 4) {
            let (noise_metadata_schedule_579_e8905,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_579_e8898: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_579_e8900: f64 = (noise_metadata_schedule_579_e8898 - 1.0);
        let noise_metadata_schedule_579_e8901: f64 = (params.p54 * noise_metadata_schedule_579_e8900);
        let noise_metadata_schedule_579_e8902: f64 = (1.0 + noise_metadata_schedule_579_e8901);
        let noise_metadata_schedule_579_e8903: f64 = (params.p48 * noise_metadata_schedule_579_e8902);
        (noise_metadata_schedule_579_e8903,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_579_e8905;
        }
        if matches!(source_index, 2 | 4) {
            let (noise_metadata_schedule_580_e8917,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_580_e8910: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_580_e8911: f64 = (noise_variable_178 / noise_metadata_schedule_580_e8910);
        let noise_metadata_schedule_580_e8913: f64 = (noise_metadata_schedule_580_e8911 + noise_variable_170);
        let noise_metadata_schedule_580_e8915: f64 = (noise_metadata_schedule_580_e8913 + noise_variable_214);
        (noise_metadata_schedule_580_e8915,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_580_e8917;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_581_e8943,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_581_e8924: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_581_e8926: f64 = (noise_metadata_schedule_581_e8924 - 1.0);
        let noise_metadata_schedule_581_e8927: f64 = (params.p50 * noise_metadata_schedule_581_e8926);
        let noise_metadata_schedule_581_e8928: f64 = (1.0 - noise_metadata_schedule_581_e8927);
        let noise_metadata_schedule_581_e8929: f64 = (params.p37 * noise_metadata_schedule_581_e8928);
        let noise_metadata_schedule_581_e8931: f64 = (noise_metadata_schedule_581_e8929 - noise_variable_341);
        let noise_metadata_schedule_581_e8933: f64 = (noise_metadata_schedule_581_e8931 - noise_variable_366);
        let noise_metadata_schedule_581_e8936: f64 = (params.p12 / 1.602176634e-19);
        let noise_metadata_schedule_581_e8938: f64 = (noise_metadata_schedule_581_e8936 * noise_variable_45);
        let noise_metadata_schedule_581_e8940: f64 = (noise_metadata_schedule_581_e8938 * noise_variable_81);
        let noise_metadata_schedule_581_e8941: f64 = (noise_metadata_schedule_581_e8933 + noise_metadata_schedule_581_e8940);
        (noise_metadata_schedule_581_e8941,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_581_e8943;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_582_e8966,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_582_e8947: f64 = (1.0 + noise_variable_177);
        let noise_metadata_schedule_582_e8951: f64 = (1.0 + noise_variable_177);
        let noise_metadata_schedule_582_e8954: f64 = (noise_variable_177 - 1.0);
        let noise_metadata_schedule_582_e8957: f64 = (noise_variable_177 - 1.0);
        let noise_metadata_schedule_582_e8958: f64 = (noise_metadata_schedule_582_e8954 * noise_metadata_schedule_582_e8957);
        let noise_metadata_schedule_582_e8960: f64 = (noise_metadata_schedule_582_e8958 + 0.001);
        let noise_metadata_schedule_582_e8961: f64 = (noise_metadata_schedule_582_e8960).sqrt();
        let noise_metadata_schedule_582_e8962: f64 = (noise_metadata_schedule_582_e8951 - noise_metadata_schedule_582_e8961);
        let noise_metadata_schedule_582_e8963: f64 = (0.5 * noise_metadata_schedule_582_e8962);
        let noise_metadata_schedule_582_e8964: f64 = (noise_metadata_schedule_582_e8947 - noise_metadata_schedule_582_e8963);
        (noise_metadata_schedule_582_e8964,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_582_e8966;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_583_e8978,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_583_e8970: f64 = (1.602176634e-19 * noise_variable_177);
        let noise_metadata_schedule_583_e8974: f64 = (params.p39 * noise_variable_187);
        let noise_metadata_schedule_583_e8975: f64 = (1.0 + noise_metadata_schedule_583_e8974);
        let noise_metadata_schedule_583_e8976: f64 = (noise_metadata_schedule_583_e8970 * noise_metadata_schedule_583_e8975);
        (noise_metadata_schedule_583_e8976,)
    } else {
        (noise_variable_172,)
    }
};
            noise_variable_172 = noise_metadata_schedule_583_e8978;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_584_e8988,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_584_e8982: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_584_e8984: f64 = (noise_metadata_schedule_584_e8982 * noise_variable_172);
        let noise_metadata_schedule_584_e8986: f64 = (noise_metadata_schedule_584_e8984 * noise_variable_176);
        (noise_metadata_schedule_584_e8986,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_584_e8988;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_585_e8998,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_585_e8993: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_585_e8995: f64 = (noise_metadata_schedule_585_e8993).powf(params.p53);
        let noise_metadata_schedule_585_e8996: f64 = (params.p41 * noise_metadata_schedule_585_e8995);
        (noise_metadata_schedule_585_e8996,)
    } else {
        (noise_variable_181,)
    }
};
            noise_variable_181 = noise_metadata_schedule_585_e8998;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_586_e9010,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_586_e9003: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_586_e9005: f64 = (noise_metadata_schedule_586_e9003 * noise_variable_172);
        let noise_metadata_schedule_586_e9007: f64 = (noise_metadata_schedule_586_e9005 * noise_variable_181);
        let noise_metadata_schedule_586_e9008: f64 = (params.p47 / noise_metadata_schedule_586_e9007);
        (noise_metadata_schedule_586_e9008,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_586_e9010;
        }
        if matches!(source_index, 1 | 3) {
            let noise_metadata_schedule_587_e9013: f64 = if noise_variable_360 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_426 = noise_metadata_schedule_587_e9013;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_588_e9021,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_588_e9019: f64 = (1.0 + params.p44);
        (noise_metadata_schedule_588_e9019,)
    } else {
        (noise_variable_350,)
    }
};
            noise_variable_350 = noise_metadata_schedule_588_e9021;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_589_e9030,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_589_e9026: f64 = (noise_variable_350).sqrt();
        let noise_metadata_schedule_589_e9028: f64 = (noise_metadata_schedule_589_e9026 * noise_variable_94);
        (noise_metadata_schedule_589_e9028,)
    } else {
        (noise_variable_351,)
    }
};
            noise_variable_351 = noise_metadata_schedule_589_e9030;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_590_e9038,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_590_e9036: f64 = (noise_variable_351 / noise_variable_173);
        (noise_metadata_schedule_590_e9036,)
    } else {
        (noise_variable_352,)
    }
};
            noise_variable_352 = noise_metadata_schedule_590_e9038;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_591_e9046,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_591_e9044: f64 = (noise_variable_352 * 2.0);
        (noise_metadata_schedule_591_e9044,)
    } else {
        (noise_variable_353,)
    }
};
            noise_variable_353 = noise_metadata_schedule_591_e9046;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_592_e9056,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_592_e9053: f64 = (noise_variable_352 * noise_variable_352);
        let noise_metadata_schedule_592_e9054: f64 = (noise_variable_350 + noise_metadata_schedule_592_e9053);
        (noise_metadata_schedule_592_e9054,)
    } else {
        (noise_variable_350,)
    }
};
            noise_variable_350 = noise_metadata_schedule_592_e9056;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_593_e9070,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_593_e9062: f64 = (noise_variable_350 - noise_variable_353);
        let noise_metadata_schedule_593_e9063: f64 = (noise_metadata_schedule_593_e9062).sqrt();
        let noise_metadata_schedule_593_e9066: f64 = (noise_variable_350 + noise_variable_353);
        let noise_metadata_schedule_593_e9067: f64 = (noise_metadata_schedule_593_e9066).sqrt();
        let noise_metadata_schedule_593_e9068: f64 = (noise_metadata_schedule_593_e9063 + noise_metadata_schedule_593_e9067);
        (noise_metadata_schedule_593_e9068,)
    } else {
        (noise_variable_350,)
    }
};
            noise_variable_350 = noise_metadata_schedule_593_e9070;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_594_e9080,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_594_e9076: f64 = (noise_variable_351 * 2.0);
        let noise_metadata_schedule_594_e9078: f64 = (noise_metadata_schedule_594_e9076 / noise_variable_350);
        (noise_metadata_schedule_594_e9078,)
    } else {
        (noise_variable_349,)
    }
};
            noise_variable_349 = noise_metadata_schedule_594_e9080;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_595_e9090,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_595_e9087: f64 = (noise_variable_349 / noise_variable_173);
        let noise_metadata_schedule_595_e9088: f64 = (1.0 - noise_metadata_schedule_595_e9087);
        (noise_metadata_schedule_595_e9088,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_595_e9090;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_596_e9100,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 == 0.0)) {
        let noise_metadata_schedule_596_e9097: f64 = (noise_variable_94 / noise_variable_173);
        let noise_metadata_schedule_596_e9098: f64 = (noise_metadata_schedule_596_e9097).abs();
        (noise_metadata_schedule_596_e9098,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_596_e9100;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_597_e9135,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 == 0.0)) {
        let noise_metadata_schedule_597_e9108: f64 = (noise_variable_182 + 0.9);
        let noise_metadata_schedule_597_e9111: f64 = (noise_variable_182 - 0.9);
        let noise_metadata_schedule_597_e9114: f64 = (noise_variable_182 - 0.9);
        let noise_metadata_schedule_597_e9115: f64 = (noise_metadata_schedule_597_e9111 * noise_metadata_schedule_597_e9114);
        let noise_metadata_schedule_597_e9118: f64 = (0.1 * 0.1);
        let noise_metadata_schedule_597_e9119: f64 = (noise_metadata_schedule_597_e9115 + noise_metadata_schedule_597_e9118);
        let noise_metadata_schedule_597_e9120: f64 = (noise_metadata_schedule_597_e9119).sqrt();
        let noise_metadata_schedule_597_e9121: f64 = (noise_metadata_schedule_597_e9108 - noise_metadata_schedule_597_e9120);
        let noise_metadata_schedule_597_e9125: f64 = (0.9 * 0.9);
        let noise_metadata_schedule_597_e9128: f64 = (0.1 * 0.1);
        let noise_metadata_schedule_597_e9129: f64 = (noise_metadata_schedule_597_e9125 + noise_metadata_schedule_597_e9128);
        let noise_metadata_schedule_597_e9130: f64 = (noise_metadata_schedule_597_e9129).sqrt();
        let noise_metadata_schedule_597_e9131: f64 = (0.9 - noise_metadata_schedule_597_e9130);
        let noise_metadata_schedule_597_e9132: f64 = (noise_metadata_schedule_597_e9121 - noise_metadata_schedule_597_e9131);
        let noise_metadata_schedule_597_e9133: f64 = (0.5 * noise_metadata_schedule_597_e9132);
        (noise_metadata_schedule_597_e9133,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_597_e9135;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_598_e9144,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 == 0.0)) {
        let noise_metadata_schedule_598_e9142: f64 = (noise_variable_183).powf(params.p43);
        (noise_metadata_schedule_598_e9142,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_598_e9144;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_599_e9153,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 == 0.0)) {
        let noise_metadata_schedule_599_e9151: f64 = (1.0 - noise_variable_136);
        (noise_metadata_schedule_599_e9151,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_599_e9153;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_600_e9164,) = {
    if ((noise_variable_424 != 0.0) && (noise_variable_426 == 0.0)) {
        let noise_metadata_schedule_600_e9161: f64 = (1.0 / params.p43);
        let noise_metadata_schedule_600_e9162: f64 = (noise_variable_90).powf(noise_metadata_schedule_600_e9161);
        (noise_metadata_schedule_600_e9162,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_600_e9164;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_601_e9170,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_601_e9168: f64 = (noise_variable_174 / noise_variable_91);
        (noise_metadata_schedule_601_e9168,)
    } else {
        (noise_variable_171,)
    }
};
            noise_variable_171 = noise_metadata_schedule_601_e9170;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_602_e9184,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_602_e9177: f64 = (noise_variable_82 / noise_variable_35);
        let noise_metadata_schedule_602_e9179: f64 = (noise_metadata_schedule_602_e9177 - 1.0);
        let noise_metadata_schedule_602_e9180: f64 = (params.p55 * noise_metadata_schedule_602_e9179);
        let noise_metadata_schedule_602_e9181: f64 = (1.0 + noise_metadata_schedule_602_e9180);
        let noise_metadata_schedule_602_e9182: f64 = (params.p49 * noise_metadata_schedule_602_e9181);
        (noise_metadata_schedule_602_e9182,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_602_e9184;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_603_e9200,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_603_e9189: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_603_e9190: f64 = (noise_variable_179 / noise_metadata_schedule_603_e9189);
        let noise_metadata_schedule_603_e9192: f64 = (noise_metadata_schedule_603_e9190 + noise_variable_171);
        let noise_metadata_schedule_603_e9194: f64 = (noise_metadata_schedule_603_e9192 + noise_variable_185);
        let noise_metadata_schedule_603_e9196: f64 = (noise_metadata_schedule_603_e9194 + noise_variable_210);
        let noise_metadata_schedule_603_e9198: f64 = (noise_metadata_schedule_603_e9196 + noise_variable_215);
        (noise_metadata_schedule_603_e9198,)
    } else {
        (noise_variable_144,)
    }
};
            noise_variable_144 = noise_metadata_schedule_603_e9200;
        }
        if matches!(source_index, 1 | 3) {
            let (noise_metadata_schedule_604_e9206,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_604_e9204: f64 = (1.0 / noise_variable_144);
        (noise_metadata_schedule_604_e9204,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_604_e9206;
        }
        if matches!(source_index, 2 | 4) {
            let (noise_metadata_schedule_605_e9212,) = {
    if (noise_variable_424 != 0.0) {
        let noise_metadata_schedule_605_e9210: f64 = (1.0 / noise_variable_145);
        (noise_metadata_schedule_605_e9210,)
    } else {
        (noise_variable_143,)
    }
};
            noise_variable_143 = noise_metadata_schedule_605_e9212;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_608_e9221: f64 = if params.p260 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_429 = noise_metadata_schedule_608_e9221;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_609_e9295,) = {
    if (noise_variable_429 != 0.0) {
        let noise_metadata_schedule_609_e9226: f64 = (noise_variable_94).max(1e-10);
        let noise_metadata_schedule_609_e9228: f64 = (noise_metadata_schedule_609_e9226 * params.p3);
        let noise_metadata_schedule_609_e9230: f64 = (noise_metadata_schedule_609_e9228 * params.p3);
        let noise_metadata_schedule_609_e9231: f64 = (params.p265 / noise_metadata_schedule_609_e9230);
        let noise_metadata_schedule_609_e9234: f64 = (4.0 * 8.617087e-5);
        let noise_metadata_schedule_609_e9236: f64 = (noise_metadata_schedule_609_e9234 * 1.602176634e-19);
        let noise_metadata_schedule_609_e9238: f64 = (noise_metadata_schedule_609_e9236 * noise_variable_82);
        let noise_metadata_schedule_609_e9240: f64 = (noise_metadata_schedule_609_e9238 * 1.602176634e-19);
        let noise_metadata_schedule_609_e9242: f64 = (noise_metadata_schedule_609_e9240 * params.p4);
        let noise_metadata_schedule_609_e9244: f64 = (noise_metadata_schedule_609_e9242 * params.p5);
        let noise_metadata_schedule_609_e9246: f64 = (noise_metadata_schedule_609_e9244 * noise_variable_80);
        let noise_metadata_schedule_609_e9248: f64 = (noise_metadata_schedule_609_e9246 * 1.602176634e-19);
        let noise_metadata_schedule_609_e9250: f64 = (noise_metadata_schedule_609_e9248 * params.p4);
        let noise_metadata_schedule_609_e9252: f64 = (noise_metadata_schedule_609_e9250 * params.p5);
        let noise_metadata_schedule_609_e9254: f64 = (noise_metadata_schedule_609_e9252 * noise_variable_80);
        let noise_metadata_schedule_609_e9255: f64 = (noise_metadata_schedule_609_e9231 * noise_metadata_schedule_609_e9254);
        let noise_metadata_schedule_609_e9258: f64 = (noise_variable_95 / noise_variable_92);
        let noise_metadata_schedule_609_e9261: f64 = (noise_variable_95 / noise_variable_92);
        let noise_metadata_schedule_609_e9262: f64 = (noise_metadata_schedule_609_e9258 * noise_metadata_schedule_609_e9261);
        let noise_metadata_schedule_609_e9263: f64 = (noise_metadata_schedule_609_e9255 * noise_metadata_schedule_609_e9262);
        let noise_metadata_schedule_609_e9266: f64 = (noise_variable_37 * noise_variable_37);
        let noise_metadata_schedule_609_e9268: f64 = (noise_metadata_schedule_609_e9266 * noise_variable_134);
        let noise_metadata_schedule_609_e9271: f64 = (noise_variable_132 * noise_variable_132);
        let noise_metadata_schedule_609_e9273: f64 = (noise_metadata_schedule_609_e9271 * noise_variable_132);
        let noise_metadata_schedule_609_e9276: f64 = (noise_variable_129 * noise_variable_129);
        let noise_metadata_schedule_609_e9278: f64 = (noise_metadata_schedule_609_e9276 * noise_variable_129);
        let noise_metadata_schedule_609_e9279: f64 = (noise_metadata_schedule_609_e9273 - noise_metadata_schedule_609_e9278);
        let noise_metadata_schedule_609_e9281: f64 = (noise_metadata_schedule_609_e9279 / 3.0);
        let noise_metadata_schedule_609_e9282: f64 = (noise_metadata_schedule_609_e9268 + noise_metadata_schedule_609_e9281);
        let noise_metadata_schedule_609_e9286: f64 = (noise_variable_132 * noise_variable_132);
        let noise_metadata_schedule_609_e9289: f64 = (noise_variable_129 * noise_variable_129);
        let noise_metadata_schedule_609_e9290: f64 = (noise_metadata_schedule_609_e9286 - noise_metadata_schedule_609_e9289);
        let noise_metadata_schedule_609_e9291: f64 = (noise_variable_37 * noise_metadata_schedule_609_e9290);
        let noise_metadata_schedule_609_e9292: f64 = (noise_metadata_schedule_609_e9282 - noise_metadata_schedule_609_e9291);
        let noise_metadata_schedule_609_e9293: f64 = (noise_metadata_schedule_609_e9263 * noise_metadata_schedule_609_e9292);
        (noise_metadata_schedule_609_e9293,)
    } else {
        (noise_variable_205,)
    }
};
            noise_variable_205 = noise_metadata_schedule_609_e9295;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_613_e9307: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_433 = noise_metadata_schedule_613_e9307;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_614_e9310: f64 = if params.p150 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_434 = noise_metadata_schedule_614_e9310;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_615_e9316,) = {
    if ((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) {
        ((ctx.node_voltage(self.nodes[15]) - ctx.node_voltage(self.nodes[7])),)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_615_e9316;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_621_e9359,) = {
    if ((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_621_e9359;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_622_e9362: f64 = if noise_variable_49 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_436 = noise_metadata_schedule_622_e9362;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_623_e9371,) = {
    if (((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) && (noise_variable_436 != 0.0)) {
        let noise_metadata_schedule_623_e9369: f64 = (-1.0);
        (noise_metadata_schedule_623_e9369,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_623_e9371;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_624_e9381,) = {
    if (((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) && (noise_variable_436 != 0.0)) {
        let noise_metadata_schedule_624_e9379: f64 = (noise_variable_48 * noise_variable_49);
        (noise_metadata_schedule_624_e9379,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_624_e9381;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_626_e9398,) = {
    if (((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) && (noise_variable_436 == 0.0)) {
        (noise_variable_49,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_626_e9398;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_628_e9420,) = {
    if ((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) {
        let noise_metadata_schedule_628_e9413: f64 = (noise_variable_231 * noise_variable_231);
        let noise_metadata_schedule_628_e9415: f64 = (noise_metadata_schedule_628_e9413 + 0.01);
        let noise_metadata_schedule_628_e9416: f64 = (noise_metadata_schedule_628_e9415).sqrt();
        let noise_metadata_schedule_628_e9418: f64 = (noise_metadata_schedule_628_e9416 - 0.1);
        (noise_metadata_schedule_628_e9418,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_628_e9420;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_629_e9432,) = {
    if ((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) {
        let noise_metadata_schedule_629_e9426: f64 = (1.0 + params.p165);
        let noise_metadata_schedule_629_e9429: f64 = (params.p166 * noise_variable_232);
        let noise_metadata_schedule_629_e9430: f64 = (noise_metadata_schedule_629_e9426 + noise_metadata_schedule_629_e9429);
        (noise_metadata_schedule_629_e9430,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_629_e9432;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_630_e9442,) = {
    if ((noise_variable_433 != 0.0) && (noise_variable_434 != 0.0)) {
        let noise_metadata_schedule_630_e9438: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_630_e9440: f64 = (noise_metadata_schedule_630_e9438 * noise_variable_146);
        (noise_metadata_schedule_630_e9440,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_630_e9442;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_778_e11821: f64 = if params.p150 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_442 = noise_metadata_schedule_778_e11821;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_783_e11859,) = {
    if ((noise_variable_433 == 0.0) && (noise_variable_442 != 0.0)) {
        let noise_metadata_schedule_783_e11857: f64 = (1.0 + params.p165);
        (noise_metadata_schedule_783_e11857,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_783_e11859;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_784_e11870,) = {
    if ((noise_variable_433 == 0.0) && (noise_variable_442 != 0.0)) {
        let noise_metadata_schedule_784_e11866: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_784_e11868: f64 = (noise_metadata_schedule_784_e11866 * noise_variable_146);
        (noise_metadata_schedule_784_e11868,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_784_e11870;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_922_e14236: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_448 = noise_metadata_schedule_922_e14236;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_923_e14239: f64 = if params.p151 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_449 = noise_metadata_schedule_923_e14239;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_924_e14245,) = {
    if ((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) {
        ((ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[19])),)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_924_e14245;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_930_e14288,) = {
    if ((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_930_e14288;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_931_e14291: f64 = if noise_variable_53 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_451 = noise_metadata_schedule_931_e14291;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_932_e14300,) = {
    if (((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) && (noise_variable_451 != 0.0)) {
        let noise_metadata_schedule_932_e14298: f64 = (-1.0);
        (noise_metadata_schedule_932_e14298,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_932_e14300;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_933_e14310,) = {
    if (((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) && (noise_variable_451 != 0.0)) {
        let noise_metadata_schedule_933_e14308: f64 = (noise_variable_52 * noise_variable_53);
        (noise_metadata_schedule_933_e14308,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_933_e14310;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_935_e14327,) = {
    if (((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) && (noise_variable_451 == 0.0)) {
        (noise_variable_53,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_935_e14327;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_937_e14349,) = {
    if ((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) {
        let noise_metadata_schedule_937_e14342: f64 = (noise_variable_243 * noise_variable_243);
        let noise_metadata_schedule_937_e14344: f64 = (noise_metadata_schedule_937_e14342 + 0.01);
        let noise_metadata_schedule_937_e14345: f64 = (noise_metadata_schedule_937_e14344).sqrt();
        let noise_metadata_schedule_937_e14347: f64 = (noise_metadata_schedule_937_e14345 - 0.1);
        (noise_metadata_schedule_937_e14347,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_937_e14349;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_938_e14361,) = {
    if ((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) {
        let noise_metadata_schedule_938_e14355: f64 = (1.0 + params.p165);
        let noise_metadata_schedule_938_e14358: f64 = (params.p166 * noise_variable_244);
        let noise_metadata_schedule_938_e14359: f64 = (noise_metadata_schedule_938_e14355 + noise_metadata_schedule_938_e14358);
        (noise_metadata_schedule_938_e14359,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_938_e14361;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_939_e14371,) = {
    if ((noise_variable_448 != 0.0) && (noise_variable_449 != 0.0)) {
        let noise_metadata_schedule_939_e14367: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_939_e14369: f64 = (noise_metadata_schedule_939_e14367 * noise_variable_146);
        (noise_metadata_schedule_939_e14369,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_939_e14371;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1087_e16750: f64 = if params.p151 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_457 = noise_metadata_schedule_1087_e16750;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1092_e16788,) = {
    if ((noise_variable_448 == 0.0) && (noise_variable_457 != 0.0)) {
        let noise_metadata_schedule_1092_e16786: f64 = (1.0 + params.p165);
        (noise_metadata_schedule_1092_e16786,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_1092_e16788;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1093_e16799,) = {
    if ((noise_variable_448 == 0.0) && (noise_variable_457 != 0.0)) {
        let noise_metadata_schedule_1093_e16795: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_1093_e16797: f64 = (noise_metadata_schedule_1093_e16795 * noise_variable_146);
        (noise_metadata_schedule_1093_e16797,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_1093_e16799;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1231_e19165: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_463 = noise_metadata_schedule_1231_e19165;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1232_e19168: f64 = if params.p152 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_464 = noise_metadata_schedule_1232_e19168;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1233_e19174,) = {
    if ((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) {
        ((ctx.node_voltage(self.nodes[16]) - ctx.node_voltage(self.nodes[15])),)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_1233_e19174;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1239_e19217,) = {
    if ((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_1239_e19217;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1240_e19220: f64 = if noise_variable_57 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_466 = noise_metadata_schedule_1240_e19220;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1241_e19229,) = {
    if (((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) && (noise_variable_466 != 0.0)) {
        let noise_metadata_schedule_1241_e19227: f64 = (-1.0);
        (noise_metadata_schedule_1241_e19227,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_1241_e19229;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1242_e19239,) = {
    if (((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) && (noise_variable_466 != 0.0)) {
        let noise_metadata_schedule_1242_e19237: f64 = (noise_variable_56 * noise_variable_57);
        (noise_metadata_schedule_1242_e19237,)
    } else {
        (noise_variable_255,)
    }
};
            noise_variable_255 = noise_metadata_schedule_1242_e19239;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1244_e19256,) = {
    if (((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) && (noise_variable_466 == 0.0)) {
        (noise_variable_57,)
    } else {
        (noise_variable_255,)
    }
};
            noise_variable_255 = noise_metadata_schedule_1244_e19256;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1246_e19278,) = {
    if ((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) {
        let noise_metadata_schedule_1246_e19271: f64 = (noise_variable_255 * noise_variable_255);
        let noise_metadata_schedule_1246_e19273: f64 = (noise_metadata_schedule_1246_e19271 + 0.01);
        let noise_metadata_schedule_1246_e19274: f64 = (noise_metadata_schedule_1246_e19273).sqrt();
        let noise_metadata_schedule_1246_e19276: f64 = (noise_metadata_schedule_1246_e19274 - 0.1);
        (noise_metadata_schedule_1246_e19276,)
    } else {
        (noise_variable_256,)
    }
};
            noise_variable_256 = noise_metadata_schedule_1246_e19278;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1247_e19290,) = {
    if ((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) {
        let noise_metadata_schedule_1247_e19284: f64 = (1.0 + params.p178);
        let noise_metadata_schedule_1247_e19287: f64 = (params.p179 * noise_variable_256);
        let noise_metadata_schedule_1247_e19288: f64 = (noise_metadata_schedule_1247_e19284 + noise_metadata_schedule_1247_e19287);
        (noise_metadata_schedule_1247_e19288,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_1247_e19290;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1248_e19300,) = {
    if ((noise_variable_463 != 0.0) && (noise_variable_464 != 0.0)) {
        let noise_metadata_schedule_1248_e19296: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_1248_e19298: f64 = (noise_metadata_schedule_1248_e19296 * noise_variable_146);
        (noise_metadata_schedule_1248_e19298,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_1248_e19300;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1396_e21679: f64 = if params.p152 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_472 = noise_metadata_schedule_1396_e21679;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1401_e21717,) = {
    if ((noise_variable_463 == 0.0) && (noise_variable_472 != 0.0)) {
        let noise_metadata_schedule_1401_e21715: f64 = (1.0 + params.p178);
        (noise_metadata_schedule_1401_e21715,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_1401_e21717;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1402_e21728,) = {
    if ((noise_variable_463 == 0.0) && (noise_variable_472 != 0.0)) {
        let noise_metadata_schedule_1402_e21724: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_1402_e21726: f64 = (noise_metadata_schedule_1402_e21724 * noise_variable_146);
        (noise_metadata_schedule_1402_e21726,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_1402_e21728;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1540_e24094: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_478 = noise_metadata_schedule_1540_e24094;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1541_e24097: f64 = if params.p153 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_479 = noise_metadata_schedule_1541_e24097;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1542_e24103,) = {
    if ((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) {
        ((ctx.node_voltage(self.nodes[19]) - ctx.node_voltage(self.nodes[20])),)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_1542_e24103;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1548_e24146,) = {
    if ((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_1548_e24146;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1549_e24149: f64 = if noise_variable_61 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_481 = noise_metadata_schedule_1549_e24149;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1550_e24158,) = {
    if (((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) && (noise_variable_481 != 0.0)) {
        let noise_metadata_schedule_1550_e24156: f64 = (-1.0);
        (noise_metadata_schedule_1550_e24156,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_1550_e24158;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1551_e24168,) = {
    if (((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) && (noise_variable_481 != 0.0)) {
        let noise_metadata_schedule_1551_e24166: f64 = (noise_variable_60 * noise_variable_61);
        (noise_metadata_schedule_1551_e24166,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_1551_e24168;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1553_e24185,) = {
    if (((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) && (noise_variable_481 == 0.0)) {
        (noise_variable_61,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_1553_e24185;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1555_e24207,) = {
    if ((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) {
        let noise_metadata_schedule_1555_e24200: f64 = (noise_variable_267 * noise_variable_267);
        let noise_metadata_schedule_1555_e24202: f64 = (noise_metadata_schedule_1555_e24200 + 0.01);
        let noise_metadata_schedule_1555_e24203: f64 = (noise_metadata_schedule_1555_e24202).sqrt();
        let noise_metadata_schedule_1555_e24205: f64 = (noise_metadata_schedule_1555_e24203 - 0.1);
        (noise_metadata_schedule_1555_e24205,)
    } else {
        (noise_variable_268,)
    }
};
            noise_variable_268 = noise_metadata_schedule_1555_e24207;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1556_e24219,) = {
    if ((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) {
        let noise_metadata_schedule_1556_e24213: f64 = (1.0 + params.p178);
        let noise_metadata_schedule_1556_e24216: f64 = (params.p179 * noise_variable_268);
        let noise_metadata_schedule_1556_e24217: f64 = (noise_metadata_schedule_1556_e24213 + noise_metadata_schedule_1556_e24216);
        (noise_metadata_schedule_1556_e24217,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_1556_e24219;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1557_e24229,) = {
    if ((noise_variable_478 != 0.0) && (noise_variable_479 != 0.0)) {
        let noise_metadata_schedule_1557_e24225: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_1557_e24227: f64 = (noise_metadata_schedule_1557_e24225 * noise_variable_146);
        (noise_metadata_schedule_1557_e24227,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_1557_e24229;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1705_e26608: f64 = if params.p153 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_487 = noise_metadata_schedule_1705_e26608;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1710_e26646,) = {
    if ((noise_variable_478 == 0.0) && (noise_variable_487 != 0.0)) {
        let noise_metadata_schedule_1710_e26644: f64 = (1.0 + params.p178);
        (noise_metadata_schedule_1710_e26644,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_1710_e26646;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1711_e26657,) = {
    if ((noise_variable_478 == 0.0) && (noise_variable_487 != 0.0)) {
        let noise_metadata_schedule_1711_e26653: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_1711_e26655: f64 = (noise_metadata_schedule_1711_e26653 * noise_variable_146);
        (noise_metadata_schedule_1711_e26655,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_1711_e26657;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1849_e29023: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_493 = noise_metadata_schedule_1849_e29023;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1850_e29026: f64 = if params.p154 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_494 = noise_metadata_schedule_1850_e29026;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1851_e29032,) = {
    if ((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) {
        ((ctx.node_voltage(self.nodes[17]) - ctx.node_voltage(self.nodes[16])),)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_1851_e29032;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1857_e29075,) = {
    if ((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_1857_e29075;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_1858_e29078: f64 = if noise_variable_65 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_496 = noise_metadata_schedule_1858_e29078;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1859_e29087,) = {
    if (((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) && (noise_variable_496 != 0.0)) {
        let noise_metadata_schedule_1859_e29085: f64 = (-1.0);
        (noise_metadata_schedule_1859_e29085,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_1859_e29087;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1860_e29097,) = {
    if (((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) && (noise_variable_496 != 0.0)) {
        let noise_metadata_schedule_1860_e29095: f64 = (noise_variable_64 * noise_variable_65);
        (noise_metadata_schedule_1860_e29095,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_1860_e29097;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1862_e29114,) = {
    if (((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) && (noise_variable_496 == 0.0)) {
        (noise_variable_65,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_1862_e29114;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1864_e29136,) = {
    if ((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) {
        let noise_metadata_schedule_1864_e29129: f64 = (noise_variable_279 * noise_variable_279);
        let noise_metadata_schedule_1864_e29131: f64 = (noise_metadata_schedule_1864_e29129 + 0.01);
        let noise_metadata_schedule_1864_e29132: f64 = (noise_metadata_schedule_1864_e29131).sqrt();
        let noise_metadata_schedule_1864_e29134: f64 = (noise_metadata_schedule_1864_e29132 - 0.1);
        (noise_metadata_schedule_1864_e29134,)
    } else {
        (noise_variable_280,)
    }
};
            noise_variable_280 = noise_metadata_schedule_1864_e29136;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1865_e29148,) = {
    if ((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) {
        let noise_metadata_schedule_1865_e29142: f64 = (1.0 + params.p191);
        let noise_metadata_schedule_1865_e29145: f64 = (params.p192 * noise_variable_280);
        let noise_metadata_schedule_1865_e29146: f64 = (noise_metadata_schedule_1865_e29142 + noise_metadata_schedule_1865_e29145);
        (noise_metadata_schedule_1865_e29146,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_1865_e29148;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_1866_e29158,) = {
    if ((noise_variable_493 != 0.0) && (noise_variable_494 != 0.0)) {
        let noise_metadata_schedule_1866_e29154: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_1866_e29156: f64 = (noise_metadata_schedule_1866_e29154 * noise_variable_146);
        (noise_metadata_schedule_1866_e29156,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_1866_e29158;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2014_e31537: f64 = if params.p154 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_502 = noise_metadata_schedule_2014_e31537;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2019_e31575,) = {
    if ((noise_variable_493 == 0.0) && (noise_variable_502 != 0.0)) {
        let noise_metadata_schedule_2019_e31573: f64 = (1.0 + params.p191);
        (noise_metadata_schedule_2019_e31573,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_2019_e31575;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2020_e31586,) = {
    if ((noise_variable_493 == 0.0) && (noise_variable_502 != 0.0)) {
        let noise_metadata_schedule_2020_e31582: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_2020_e31584: f64 = (noise_metadata_schedule_2020_e31582 * noise_variable_146);
        (noise_metadata_schedule_2020_e31584,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_2020_e31586;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2158_e33952: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_508 = noise_metadata_schedule_2158_e33952;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2159_e33955: f64 = if params.p155 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_509 = noise_metadata_schedule_2159_e33955;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2160_e33961,) = {
    if ((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) {
        ((ctx.node_voltage(self.nodes[20]) - ctx.node_voltage(self.nodes[21])),)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_2160_e33961;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2166_e34004,) = {
    if ((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_2166_e34004;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2167_e34007: f64 = if noise_variable_69 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_511 = noise_metadata_schedule_2167_e34007;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2168_e34016,) = {
    if (((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) && (noise_variable_511 != 0.0)) {
        let noise_metadata_schedule_2168_e34014: f64 = (-1.0);
        (noise_metadata_schedule_2168_e34014,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_2168_e34016;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2169_e34026,) = {
    if (((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) && (noise_variable_511 != 0.0)) {
        let noise_metadata_schedule_2169_e34024: f64 = (noise_variable_68 * noise_variable_69);
        (noise_metadata_schedule_2169_e34024,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_2169_e34026;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2171_e34043,) = {
    if (((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) && (noise_variable_511 == 0.0)) {
        (noise_variable_69,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_2171_e34043;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2173_e34065,) = {
    if ((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) {
        let noise_metadata_schedule_2173_e34058: f64 = (noise_variable_291 * noise_variable_291);
        let noise_metadata_schedule_2173_e34060: f64 = (noise_metadata_schedule_2173_e34058 + 0.01);
        let noise_metadata_schedule_2173_e34061: f64 = (noise_metadata_schedule_2173_e34060).sqrt();
        let noise_metadata_schedule_2173_e34063: f64 = (noise_metadata_schedule_2173_e34061 - 0.1);
        (noise_metadata_schedule_2173_e34063,)
    } else {
        (noise_variable_292,)
    }
};
            noise_variable_292 = noise_metadata_schedule_2173_e34065;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2174_e34077,) = {
    if ((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) {
        let noise_metadata_schedule_2174_e34071: f64 = (1.0 + params.p191);
        let noise_metadata_schedule_2174_e34074: f64 = (params.p192 * noise_variable_292);
        let noise_metadata_schedule_2174_e34075: f64 = (noise_metadata_schedule_2174_e34071 + noise_metadata_schedule_2174_e34074);
        (noise_metadata_schedule_2174_e34075,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_2174_e34077;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2175_e34087,) = {
    if ((noise_variable_508 != 0.0) && (noise_variable_509 != 0.0)) {
        let noise_metadata_schedule_2175_e34083: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_2175_e34085: f64 = (noise_metadata_schedule_2175_e34083 * noise_variable_146);
        (noise_metadata_schedule_2175_e34085,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_2175_e34087;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2323_e36466: f64 = if params.p155 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_517 = noise_metadata_schedule_2323_e36466;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2328_e36504,) = {
    if ((noise_variable_508 == 0.0) && (noise_variable_517 != 0.0)) {
        let noise_metadata_schedule_2328_e36502: f64 = (1.0 + params.p191);
        (noise_metadata_schedule_2328_e36502,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_2328_e36504;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2329_e36515,) = {
    if ((noise_variable_508 == 0.0) && (noise_variable_517 != 0.0)) {
        let noise_metadata_schedule_2329_e36511: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_2329_e36513: f64 = (noise_metadata_schedule_2329_e36511 * noise_variable_146);
        (noise_metadata_schedule_2329_e36513,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_2329_e36515;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2467_e38881: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_523 = noise_metadata_schedule_2467_e38881;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2468_e38884: f64 = if params.p156 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_524 = noise_metadata_schedule_2468_e38884;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2469_e38890,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        ((ctx.node_voltage(self.nodes[18]) - ctx.node_voltage(self.nodes[17])),)
    } else {
        (noise_variable_73,)
    }
};
            noise_variable_73 = noise_metadata_schedule_2469_e38890;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2475_e38933,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_72,)
    }
};
            noise_variable_72 = noise_metadata_schedule_2475_e38933;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2476_e38936: f64 = if noise_variable_73 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_526 = noise_metadata_schedule_2476_e38936;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2477_e38945,) = {
    if (((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) && (noise_variable_526 != 0.0)) {
        let noise_metadata_schedule_2477_e38943: f64 = (-1.0);
        (noise_metadata_schedule_2477_e38943,)
    } else {
        (noise_variable_72,)
    }
};
            noise_variable_72 = noise_metadata_schedule_2477_e38945;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2478_e38955,) = {
    if (((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) && (noise_variable_526 != 0.0)) {
        let noise_metadata_schedule_2478_e38953: f64 = (noise_variable_72 * noise_variable_73);
        (noise_metadata_schedule_2478_e38953,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_2478_e38955;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2480_e38972,) = {
    if (((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) && (noise_variable_526 == 0.0)) {
        (noise_variable_73,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_2480_e38972;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2482_e38994,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_2482_e38987: f64 = (noise_variable_303 * noise_variable_303);
        let noise_metadata_schedule_2482_e38989: f64 = (noise_metadata_schedule_2482_e38987 + 0.01);
        let noise_metadata_schedule_2482_e38990: f64 = (noise_metadata_schedule_2482_e38989).sqrt();
        let noise_metadata_schedule_2482_e38992: f64 = (noise_metadata_schedule_2482_e38990 - 0.1);
        (noise_metadata_schedule_2482_e38992,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_2482_e38994;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2483_e39006,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_2483_e39000: f64 = (1.0 + params.p204);
        let noise_metadata_schedule_2483_e39003: f64 = (params.p205 * noise_variable_304);
        let noise_metadata_schedule_2483_e39004: f64 = (noise_metadata_schedule_2483_e39000 + noise_metadata_schedule_2483_e39003);
        (noise_metadata_schedule_2483_e39004,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_2483_e39006;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2484_e39016,) = {
    if ((noise_variable_523 != 0.0) && (noise_variable_524 != 0.0)) {
        let noise_metadata_schedule_2484_e39012: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_2484_e39014: f64 = (noise_metadata_schedule_2484_e39012 * noise_variable_146);
        (noise_metadata_schedule_2484_e39014,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_2484_e39016;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2632_e41395: f64 = if params.p156 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_532 = noise_metadata_schedule_2632_e41395;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2637_e41433,) = {
    if ((noise_variable_523 == 0.0) && (noise_variable_532 != 0.0)) {
        let noise_metadata_schedule_2637_e41431: f64 = (1.0 + params.p204);
        (noise_metadata_schedule_2637_e41431,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_2637_e41433;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2638_e41444,) = {
    if ((noise_variable_523 == 0.0) && (noise_variable_532 != 0.0)) {
        let noise_metadata_schedule_2638_e41440: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_2638_e41442: f64 = (noise_metadata_schedule_2638_e41440 * noise_variable_146);
        (noise_metadata_schedule_2638_e41442,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_2638_e41444;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2776_e43810: f64 = if params.p149 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_538 = noise_metadata_schedule_2776_e43810;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2777_e43813: f64 = if params.p157 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_539 = noise_metadata_schedule_2777_e43813;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2778_e43819,) = {
    if ((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) {
        ((ctx.node_voltage(self.nodes[21]) - ctx.node_voltage(self.nodes[22])),)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_2778_e43819;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2784_e43862,) = {
    if ((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_2784_e43862;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2785_e43865: f64 = if noise_variable_77 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_541 = noise_metadata_schedule_2785_e43865;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2786_e43874,) = {
    if (((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) && (noise_variable_541 != 0.0)) {
        let noise_metadata_schedule_2786_e43872: f64 = (-1.0);
        (noise_metadata_schedule_2786_e43872,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_2786_e43874;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2787_e43884,) = {
    if (((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) && (noise_variable_541 != 0.0)) {
        let noise_metadata_schedule_2787_e43882: f64 = (noise_variable_76 * noise_variable_77);
        (noise_metadata_schedule_2787_e43882,)
    } else {
        (noise_variable_315,)
    }
};
            noise_variable_315 = noise_metadata_schedule_2787_e43884;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2789_e43901,) = {
    if (((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) && (noise_variable_541 == 0.0)) {
        (noise_variable_77,)
    } else {
        (noise_variable_315,)
    }
};
            noise_variable_315 = noise_metadata_schedule_2789_e43901;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2791_e43923,) = {
    if ((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) {
        let noise_metadata_schedule_2791_e43916: f64 = (noise_variable_315 * noise_variable_315);
        let noise_metadata_schedule_2791_e43918: f64 = (noise_metadata_schedule_2791_e43916 + 0.01);
        let noise_metadata_schedule_2791_e43919: f64 = (noise_metadata_schedule_2791_e43918).sqrt();
        let noise_metadata_schedule_2791_e43921: f64 = (noise_metadata_schedule_2791_e43919 - 0.1);
        (noise_metadata_schedule_2791_e43921,)
    } else {
        (noise_variable_316,)
    }
};
            noise_variable_316 = noise_metadata_schedule_2791_e43923;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2792_e43935,) = {
    if ((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) {
        let noise_metadata_schedule_2792_e43929: f64 = (1.0 + params.p204);
        let noise_metadata_schedule_2792_e43932: f64 = (params.p205 * noise_variable_316);
        let noise_metadata_schedule_2792_e43933: f64 = (noise_metadata_schedule_2792_e43929 + noise_metadata_schedule_2792_e43932);
        (noise_metadata_schedule_2792_e43933,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_2792_e43935;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2793_e43945,) = {
    if ((noise_variable_538 != 0.0) && (noise_variable_539 != 0.0)) {
        let noise_metadata_schedule_2793_e43941: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_2793_e43943: f64 = (noise_metadata_schedule_2793_e43941 * noise_variable_146);
        (noise_metadata_schedule_2793_e43943,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_2793_e43945;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_2941_e46324: f64 = if params.p157 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_547 = noise_metadata_schedule_2941_e46324;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2946_e46362,) = {
    if ((noise_variable_538 == 0.0) && (noise_variable_547 != 0.0)) {
        let noise_metadata_schedule_2946_e46360: f64 = (1.0 + params.p204);
        (noise_metadata_schedule_2946_e46360,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_2946_e46362;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_2947_e46373,) = {
    if ((noise_variable_538 == 0.0) && (noise_variable_547 != 0.0)) {
        let noise_metadata_schedule_2947_e46369: f64 = (8.617087e-5 * noise_variable_82);
        let noise_metadata_schedule_2947_e46371: f64 = (noise_metadata_schedule_2947_e46369 * noise_variable_146);
        (noise_metadata_schedule_2947_e46371,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_2947_e46373;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3099_e48883: f64 = if params.p255 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_558 = noise_metadata_schedule_3099_e48883;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3100_e48893,) = {
    if (noise_variable_558 != 0.0) {
        let noise_metadata_schedule_3100_e48887: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_3100_e48889: f64 = (noise_metadata_schedule_3100_e48887 * params.p210);
        let noise_metadata_schedule_3100_e48891: f64 = (noise_metadata_schedule_3100_e48889 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[2])));
        (noise_metadata_schedule_3100_e48891,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_3100_e48893;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3101_e48908,) = {
    if (noise_variable_558 != 0.0) {
        let noise_metadata_schedule_3101_e48897: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])) * params.p214);
        let noise_metadata_schedule_3101_e48900: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])) * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])));
        let noise_metadata_schedule_3101_e48903: f64 = (params.p214 * params.p214);
        let noise_metadata_schedule_3101_e48904: f64 = (noise_metadata_schedule_3101_e48900 + noise_metadata_schedule_3101_e48903);
        let noise_metadata_schedule_3101_e48905: f64 = (noise_metadata_schedule_3101_e48904).sqrt();
        let noise_metadata_schedule_3101_e48906: f64 = (noise_metadata_schedule_3101_e48897 / noise_metadata_schedule_3101_e48905);
        (noise_metadata_schedule_3101_e48906,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_3101_e48908;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3102_e48918,) = {
    if (noise_variable_558 != 0.0) {
        let noise_metadata_schedule_3102_e48914: f64 = (2.0 * params.p214);
        let noise_metadata_schedule_3102_e48915: f64 = (params.p211 / noise_metadata_schedule_3102_e48914);
        let noise_metadata_schedule_3102_e48916: f64 = (params.p213).min(noise_metadata_schedule_3102_e48915);
        (noise_metadata_schedule_3102_e48916,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_3102_e48918;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3103_e48934,) = {
    if (noise_variable_558 != 0.0) {
        let noise_metadata_schedule_3103_e48922: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_3103_e48924: f64 = (noise_metadata_schedule_3103_e48922 * params.p211);
        let noise_metadata_schedule_3103_e48927: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_3103_e48929: f64 = (noise_metadata_schedule_3103_e48927 * noise_variable_169);
        let noise_metadata_schedule_3103_e48931: f64 = (noise_metadata_schedule_3103_e48929 * noise_variable_168);
        let noise_metadata_schedule_3103_e48932: f64 = (noise_metadata_schedule_3103_e48924 - noise_metadata_schedule_3103_e48931);
        (noise_metadata_schedule_3103_e48932,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_3103_e48934;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3104_e48942,) = {
    if (noise_variable_558 != 0.0) {
        let noise_metadata_schedule_3104_e48938: f64 = (noise_variable_167).max(0.0);
        let noise_metadata_schedule_3104_e48940: f64 = (noise_metadata_schedule_3104_e48938 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[0])));
        (noise_metadata_schedule_3104_e48940,)
    } else {
        (noise_variable_163,)
    }
};
            noise_variable_163 = noise_metadata_schedule_3104_e48942;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3105_e48953,) = {
    if (noise_variable_558 == 0.0) {
        let noise_metadata_schedule_3105_e48947: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_3105_e48949: f64 = (noise_metadata_schedule_3105_e48947 * params.p210);
        let noise_metadata_schedule_3105_e48951: f64 = (noise_metadata_schedule_3105_e48949 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[2])));
        (noise_metadata_schedule_3105_e48951,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_3105_e48953;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3106_e48969,) = {
    if (noise_variable_558 == 0.0) {
        let noise_metadata_schedule_3106_e48958: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])) * params.p214);
        let noise_metadata_schedule_3106_e48961: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])) * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])));
        let noise_metadata_schedule_3106_e48964: f64 = (params.p214 * params.p214);
        let noise_metadata_schedule_3106_e48965: f64 = (noise_metadata_schedule_3106_e48961 + noise_metadata_schedule_3106_e48964);
        let noise_metadata_schedule_3106_e48966: f64 = (noise_metadata_schedule_3106_e48965).sqrt();
        let noise_metadata_schedule_3106_e48967: f64 = (noise_metadata_schedule_3106_e48958 / noise_metadata_schedule_3106_e48966);
        (noise_metadata_schedule_3106_e48967,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_3106_e48969;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3107_e48980,) = {
    if (noise_variable_558 == 0.0) {
        let noise_metadata_schedule_3107_e48976: f64 = (2.0 * params.p214);
        let noise_metadata_schedule_3107_e48977: f64 = (params.p211 / noise_metadata_schedule_3107_e48976);
        let noise_metadata_schedule_3107_e48978: f64 = (params.p213).min(noise_metadata_schedule_3107_e48977);
        (noise_metadata_schedule_3107_e48978,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_3107_e48980;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3108_e48997,) = {
    if (noise_variable_558 == 0.0) {
        let noise_metadata_schedule_3108_e48985: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_3108_e48987: f64 = (noise_metadata_schedule_3108_e48985 * params.p211);
        let noise_metadata_schedule_3108_e48990: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_3108_e48992: f64 = (noise_metadata_schedule_3108_e48990 * noise_variable_169);
        let noise_metadata_schedule_3108_e48994: f64 = (noise_metadata_schedule_3108_e48992 * noise_variable_168);
        let noise_metadata_schedule_3108_e48995: f64 = (noise_metadata_schedule_3108_e48987 - noise_metadata_schedule_3108_e48994);
        (noise_metadata_schedule_3108_e48995,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_3108_e48997;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3109_e49006,) = {
    if (noise_variable_558 == 0.0) {
        let noise_metadata_schedule_3109_e49002: f64 = (noise_variable_167).max(0.0);
        let noise_metadata_schedule_3109_e49004: f64 = (noise_metadata_schedule_3109_e49002 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
        (noise_metadata_schedule_3109_e49004,)
    } else {
        (noise_variable_163,)
    }
};
            noise_variable_163 = noise_metadata_schedule_3109_e49006;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3110_e49009: f64 = (params.p4 * params.p5);
            let noise_metadata_schedule_3110_e49011: f64 = (noise_metadata_schedule_3110_e49009 * params.p212);
            let noise_metadata_schedule_3110_e49013: f64 = (noise_metadata_schedule_3110_e49011 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[2])));
            noise_variable_164 = noise_metadata_schedule_3110_e49013;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3111_e49015: f64 = (-noise_variable_163);
            let noise_metadata_schedule_3111_e49017: f64 = (noise_metadata_schedule_3111_e49015 + noise_variable_164);
            noise_variable_217 = noise_metadata_schedule_3111_e49017;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3112_e49019: f64 = (-noise_variable_162);
            let noise_metadata_schedule_3112_e49021: f64 = (noise_metadata_schedule_3112_e49019 - noise_variable_164);
            noise_variable_218 = noise_metadata_schedule_3112_e49021;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3113_e49024: f64 = (noise_variable_165 + noise_variable_217);
            noise_variable_138 = noise_metadata_schedule_3113_e49024;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3114_e49027: f64 = (noise_variable_166 + noise_variable_218);
            noise_variable_139 = noise_metadata_schedule_3114_e49027;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3162_e49409: f64 = if params.p259 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_567 = noise_metadata_schedule_3162_e49409;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3163_e49423,) = {
    if (noise_variable_567 != 0.0) {
        let noise_metadata_schedule_3163_e49414: f64 = (noise_variable_37 - noise_variable_133);
        let noise_metadata_schedule_3163_e49416: f64 = (noise_metadata_schedule_3163_e49414 + noise_variable_83);
        let noise_metadata_schedule_3163_e49419: f64 = (noise_variable_134).max(1e-12);
        let noise_metadata_schedule_3163_e49420: f64 = (noise_metadata_schedule_3163_e49416 * noise_metadata_schedule_3163_e49419);
        let noise_metadata_schedule_3163_e49421: f64 = (params.p3 / noise_metadata_schedule_3163_e49420);
        (noise_metadata_schedule_3163_e49421,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_3163_e49423;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3164_e49441,) = {
    if (noise_variable_567 != 0.0) {
        let noise_metadata_schedule_3164_e49427: f64 = (noise_variable_83 * 1.602176634e-19);
        let noise_metadata_schedule_3164_e49429: f64 = (noise_metadata_schedule_3164_e49427 * 1.602176634e-19);
        let noise_metadata_schedule_3164_e49431: f64 = (noise_metadata_schedule_3164_e49429 * 1.602176634e-19);
        let noise_metadata_schedule_3164_e49434: f64 = (params.p4 * params.p5);
        let noise_metadata_schedule_3164_e49436: f64 = (noise_metadata_schedule_3164_e49434 * params.p3);
        let noise_metadata_schedule_3164_e49438: f64 = (noise_metadata_schedule_3164_e49436 * params.p3);
        let noise_metadata_schedule_3164_e49439: f64 = (noise_metadata_schedule_3164_e49431 / noise_metadata_schedule_3164_e49438);
        (noise_metadata_schedule_3164_e49439,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_3164_e49441;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3165_e49463,) = {
    if (noise_variable_567 != 0.0) {
        let noise_metadata_schedule_3165_e49445: f64 = (params.p261 * noise_variable_83);
        let noise_metadata_schedule_3165_e49447: f64 = (noise_metadata_schedule_3165_e49445 * noise_variable_80);
        let noise_metadata_schedule_3165_e49451: f64 = (noise_variable_138).max(1e-22);
        let noise_metadata_schedule_3165_e49452: f64 = (1.0 / noise_metadata_schedule_3165_e49451);
        let noise_metadata_schedule_3165_e49453: f64 = (noise_metadata_schedule_3165_e49447 * noise_metadata_schedule_3165_e49452);
        let noise_metadata_schedule_3165_e49458: f64 = (noise_variable_139).max(1e-22);
        let noise_metadata_schedule_3165_e49459: f64 = (noise_variable_138 / noise_metadata_schedule_3165_e49458);
        let noise_metadata_schedule_3165_e49460: f64 = (1.0 - noise_metadata_schedule_3165_e49459);
        let noise_metadata_schedule_3165_e49461: f64 = (noise_metadata_schedule_3165_e49453 * noise_metadata_schedule_3165_e49460);
        (noise_metadata_schedule_3165_e49461,)
    } else {
        (noise_variable_200,)
    }
};
            noise_variable_200 = noise_metadata_schedule_3165_e49463;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3166_e49482,) = {
    if (noise_variable_567 != 0.0) {
        let noise_metadata_schedule_3166_e49468: f64 = (params.p262 * noise_variable_83);
        let noise_metadata_schedule_3166_e49470: f64 = (noise_metadata_schedule_3166_e49468 * noise_variable_80);
        let noise_metadata_schedule_3166_e49471: f64 = (params.p261 + noise_metadata_schedule_3166_e49470);
        let noise_metadata_schedule_3166_e49474: f64 = (noise_variable_138).max(1e-22);
        let noise_metadata_schedule_3166_e49477: f64 = (noise_variable_139).max(1e-22);
        let noise_metadata_schedule_3166_e49478: f64 = (noise_metadata_schedule_3166_e49474 / noise_metadata_schedule_3166_e49477);
        let noise_metadata_schedule_3166_e49479: f64 = (noise_metadata_schedule_3166_e49478).ln();
        let noise_metadata_schedule_3166_e49480: f64 = (noise_metadata_schedule_3166_e49471 * noise_metadata_schedule_3166_e49479);
        (noise_metadata_schedule_3166_e49480,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_3166_e49482;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3167_e49496,) = {
    if (noise_variable_567 != 0.0) {
        let noise_metadata_schedule_3167_e49487: f64 = (params.p263 * noise_variable_83);
        let noise_metadata_schedule_3167_e49489: f64 = (noise_metadata_schedule_3167_e49487 * noise_variable_80);
        let noise_metadata_schedule_3167_e49490: f64 = (params.p262 + noise_metadata_schedule_3167_e49489);
        let noise_metadata_schedule_3167_e49493: f64 = (noise_variable_139 - noise_variable_138);
        let noise_metadata_schedule_3167_e49494: f64 = (noise_metadata_schedule_3167_e49490 * noise_metadata_schedule_3167_e49493);
        (noise_metadata_schedule_3167_e49494,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_3167_e49496;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3168_e49510,) = {
    if (noise_variable_567 != 0.0) {
        let noise_metadata_schedule_3168_e49500: f64 = (params.p263 / 2.0);
        let noise_metadata_schedule_3168_e49503: f64 = (noise_variable_138 * noise_variable_138);
        let noise_metadata_schedule_3168_e49506: f64 = (noise_variable_139 * noise_variable_139);
        let noise_metadata_schedule_3168_e49507: f64 = (noise_metadata_schedule_3168_e49503 - noise_metadata_schedule_3168_e49506);
        let noise_metadata_schedule_3168_e49508: f64 = (noise_metadata_schedule_3168_e49500 * noise_metadata_schedule_3168_e49507);
        (noise_metadata_schedule_3168_e49508,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_3168_e49510;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3169_e49532,) = {
    if (noise_variable_567 != 0.0) {
        let noise_metadata_schedule_3169_e49515: f64 = (noise_variable_94 * noise_variable_94);
        let noise_metadata_schedule_3169_e49516: f64 = (noise_variable_198 * noise_metadata_schedule_3169_e49515);
        let noise_metadata_schedule_3169_e49520: f64 = (noise_variable_80 * noise_variable_80);
        let noise_metadata_schedule_3169_e49521: f64 = (noise_variable_199 / noise_metadata_schedule_3169_e49520);
        let noise_metadata_schedule_3169_e49522: f64 = (noise_metadata_schedule_3169_e49516 * noise_metadata_schedule_3169_e49521);
        let noise_metadata_schedule_3169_e49525: f64 = (noise_variable_200 + noise_variable_201);
        let noise_metadata_schedule_3169_e49527: f64 = (noise_metadata_schedule_3169_e49525 + noise_variable_202);
        let noise_metadata_schedule_3169_e49529: f64 = (noise_metadata_schedule_3169_e49527 + noise_variable_203);
        let noise_metadata_schedule_3169_e49530: f64 = (noise_metadata_schedule_3169_e49522 * noise_metadata_schedule_3169_e49529);
        (noise_metadata_schedule_3169_e49530,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_3169_e49532;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_3170_e49535: f64 = if noise_variable_41 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_568 = noise_metadata_schedule_3170_e49535;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_3171_e49542,) = {
    if ((noise_variable_567 != 0.0) && (noise_variable_568 != 0.0)) {
        let noise_metadata_schedule_3171_e49540: f64 = (-noise_variable_204);
        (noise_metadata_schedule_3171_e49540,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_3171_e49542;
        }
        match source_index {
            0 => {
                let noise_0_psd_e49980: f64 = 1.0;
                let noise_0_psd_e1046: f64 = (noise_variable_205 * params.p6);
                let noise_0_psd_e49981: f64 = (noise_0_psd_e49980 * noise_0_psd_e1046);
                let psd = noise_0_psd_e49981;
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
                let noise_1_psd_e49983: f64 = 1.0;
                let noise_1_psd_e1058: f64 = (4.0 * noise_variable_36);
                let noise_1_psd_e1060: f64 = (noise_1_psd_e1058 * 1.602176634e-19);
                let noise_1_psd_e1062: f64 = (noise_1_psd_e1060 * noise_variable_142);
                let noise_1_psd_e1064: f64 = (noise_1_psd_e1062 * params.p6);
                let noise_1_psd_e49984: f64 = (noise_1_psd_e49983 * noise_1_psd_e1064);
                let psd = noise_1_psd_e49984;
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
                let noise_2_psd_e49986: f64 = 1.0;
                let noise_2_psd_e1076: f64 = (4.0 * noise_variable_36);
                let noise_2_psd_e1078: f64 = (noise_2_psd_e1076 * 1.602176634e-19);
                let noise_2_psd_e1080: f64 = (noise_2_psd_e1078 * noise_variable_143);
                let noise_2_psd_e1082: f64 = (noise_2_psd_e1080 * params.p6);
                let noise_2_psd_e49987: f64 = (noise_2_psd_e49986 * noise_2_psd_e1082);
                let psd = noise_2_psd_e49987;
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
                let noise_3_psd_e49989: f64 = 1.0;
                let noise_3_psd_e1095: f64 = (4.0 * noise_variable_36);
                let noise_3_psd_e1097: f64 = (noise_3_psd_e1095 * 1.602176634e-19);
                let noise_3_psd_e1099: f64 = (noise_3_psd_e1097 * noise_variable_142);
                let noise_3_psd_e1101: f64 = (noise_3_psd_e1099 * params.p6);
                let noise_3_psd_e49990: f64 = (noise_3_psd_e49989 * noise_3_psd_e1101);
                let psd = noise_3_psd_e49990;
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
                let noise_4_psd_e49992: f64 = 1.0;
                let noise_4_psd_e1114: f64 = (4.0 * noise_variable_36);
                let noise_4_psd_e1116: f64 = (noise_4_psd_e1114 * 1.602176634e-19);
                let noise_4_psd_e1118: f64 = (noise_4_psd_e1116 * noise_variable_143);
                let noise_4_psd_e1120: f64 = (noise_4_psd_e1118 * params.p6);
                let noise_4_psd_e49993: f64 = (noise_4_psd_e49992 * noise_4_psd_e1120);
                let psd = noise_4_psd_e49993;
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
                let noise_5_psd_e49995: f64 = 1.0;
                let noise_5_psd_e1128: f64 = (2.0 * 1.602176634e-19);
                let noise_5_psd_e1130: f64 = (noise_variable_206).abs();
                let noise_5_psd_e1131: f64 = (noise_5_psd_e1128 * noise_5_psd_e1130);
                let noise_5_psd_e1133: f64 = (noise_5_psd_e1131 * params.p6);
                let noise_5_psd_e49996: f64 = (noise_5_psd_e49995 * noise_5_psd_e1133);
                let psd = noise_5_psd_e49996;
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
                let noise_6_psd_e49998: f64 = 1.0;
                let noise_6_psd_e1141: f64 = (2.0 * 1.602176634e-19);
                let noise_6_psd_e1143: f64 = (noise_variable_207).abs();
                let noise_6_psd_e1144: f64 = (noise_6_psd_e1141 * noise_6_psd_e1143);
                let noise_6_psd_e1146: f64 = (noise_6_psd_e1144 * params.p6);
                let noise_6_psd_e49999: f64 = (noise_6_psd_e49998 * noise_6_psd_e1146);
                let psd = noise_6_psd_e49999;
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
                let noise_7_psd_e50001: f64 = 1.0;
                let noise_7_psd_e1466: f64 = (noise_variable_204 * params.p8);
                let noise_7_psd_e50002: f64 = (noise_7_psd_e50001 * noise_7_psd_e1466);
                let psd = noise_7_psd_e50002;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
                let exponent: Option<f64> = Some(params.p264);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

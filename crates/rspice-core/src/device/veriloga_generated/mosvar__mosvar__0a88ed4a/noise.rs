#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 7] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_CI_IGC", label: Some("Igc"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGOV", label: Some("Igov"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GII_RGSAL", label: Some("rgsal"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "gii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GII_GI_RGPV", label: Some("rgpv"), kind: GeneratedNoiseKind::White, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "gii", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_B_REND", label: Some("rend"), kind: GeneratedNoiseKind::White, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "b", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_RSUB", label: Some("rsub"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_RAC", label: Some("rac"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "bi", is_internal: false }, table_len: 0, table_log_interp: false },
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
        let noise_source_active = match source_index {
            0 => {
                params.p49 != 0.0
            }
            1 => {
                params.p49 != 0.0
            }
            2 => {
                params.p16 != 0.0
            }
            3 => {
                params.p16 != 0.0
            }
            4 => {
                params.p16 != 0.0
            }
            5 => {
                params.p16 != 0.0
            }
            6 => {
                params.p16 != 0.0
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
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_1_e189: f64 = (params.p20 / 3.9);
            let noise_metadata_schedule_1_e190: f64 = (3.453e-11 * noise_metadata_schedule_1_e189);
            let noise_metadata_schedule_1_e192: f64 = (noise_metadata_schedule_1_e190 / params.p19);
            noise_variable_11 = noise_metadata_schedule_1_e192;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_3_e205: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_3_e207: f64 = (noise_metadata_schedule_3_e205 * 1.045e-10);
            let noise_metadata_schedule_3_e209: f64 = (noise_metadata_schedule_3_e207 * params.p29);
            let noise_metadata_schedule_3_e210: f64 = (noise_metadata_schedule_3_e209).sqrt();
            let noise_metadata_schedule_3_e212: f64 = (noise_metadata_schedule_3_e210 / noise_variable_11);
            noise_variable_13 = noise_metadata_schedule_3_e212;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_4_e215: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_4_e217: f64 = (noise_metadata_schedule_4_e215 * 1.045e-10);
            let noise_metadata_schedule_4_e219: f64 = (noise_metadata_schedule_4_e217 * params.p54);
            let noise_metadata_schedule_4_e220: f64 = (noise_metadata_schedule_4_e219).sqrt();
            let noise_metadata_schedule_4_e222: f64 = (noise_metadata_schedule_4_e220 / noise_variable_11);
            noise_variable_109 = noise_metadata_schedule_4_e222;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_5_e225: f64 = if params.p30 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_144 = noise_metadata_schedule_5_e225;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_6_e237,) = {
    if (noise_variable_144 != 0.0) {
        let noise_metadata_schedule_6_e229: f64 = (0.4 * 5.951993);
        let noise_metadata_schedule_6_e231: f64 = (noise_metadata_schedule_6_e229 * params.p30);
        let noise_metadata_schedule_6_e234: f64 = (noise_variable_11).powf(0.6666666666666666);
        let noise_metadata_schedule_6_e235: f64 = (noise_metadata_schedule_6_e231 * noise_metadata_schedule_6_e234);
        (noise_metadata_schedule_6_e235,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_6_e237;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_7_e240: f64 = if params.p17 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_145 = noise_metadata_schedule_7_e240;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_8_e250,) = {
    if ((noise_variable_144 != 0.0) && (noise_variable_145 != 0.0)) {
        let noise_metadata_schedule_8_e246: f64 = (7.448711 / 5.951993);
        let noise_metadata_schedule_8_e248: f64 = (noise_metadata_schedule_8_e246 * noise_variable_54);
        (noise_metadata_schedule_8_e248,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_8_e250;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_9_e255,) = {
    if (noise_variable_144 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_9_e255;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_10_e258: f64 = if params.p17 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_146 = noise_metadata_schedule_10_e258;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_11_e264,) = {
    if (noise_variable_146 != 0.0) {
        let noise_metadata_schedule_11_e262: f64 = (0.3333333333333333 * params.p48);
        (noise_metadata_schedule_11_e262,)
    } else {
        (noise_variable_84,)
    }
};
            noise_variable_84 = noise_metadata_schedule_11_e264;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_12_e271,) = {
    if (noise_variable_146 == 0.0) {
        let noise_metadata_schedule_12_e269: f64 = (0.5 * params.p48);
        (noise_metadata_schedule_12_e269,)
    } else {
        (noise_variable_84,)
    }
};
            noise_variable_84 = noise_metadata_schedule_12_e271;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_13_e274: f64 = (params.p19 / 1e-9);
            noise_variable_141 = noise_metadata_schedule_13_e274;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_14_e277: f64 = (-273.0);
            let (noise_metadata_schedule_14_e282,) = {
    if (params.p11 > noise_metadata_schedule_14_e277) {
        (params.p11,)
    } else {
        let noise_metadata_schedule_14_e281: f64 = (-273.0);
        (noise_metadata_schedule_14_e281,)
    }
};
            noise_variable_16 = noise_metadata_schedule_14_e282;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_17_e291: f64 = (273.15 + noise_variable_16);
            noise_variable_17 = noise_metadata_schedule_17_e291;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_18_e292: f64 = ctx.temperature();
            let noise_metadata_schedule_18_e294: f64 = (noise_metadata_schedule_18_e292 + params.p3);
            let noise_metadata_schedule_18_e296: f64 = (noise_metadata_schedule_18_e294 - 273.15);
            noise_variable_142 = noise_metadata_schedule_18_e296;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_21_e305: f64 = (noise_variable_142 + 273.15);
            noise_variable_14 = noise_metadata_schedule_21_e305;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_22_e308: f64 = (noise_variable_14 * noise_variable_14);
            noise_variable_15 = noise_metadata_schedule_22_e308;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_23_e311: f64 = (noise_variable_14 - noise_variable_17);
            noise_variable_18 = noise_metadata_schedule_23_e311;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_24_e314: f64 = (noise_variable_14 / noise_variable_17);
            noise_variable_19 = noise_metadata_schedule_24_e314;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_25_e317: f64 = (noise_variable_17 / noise_variable_14);
            noise_variable_20 = noise_metadata_schedule_25_e317;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_26_e320: f64 = (noise_variable_14 * 1.3806505e-23);
            let noise_metadata_schedule_26_e322: f64 = (noise_metadata_schedule_26_e320 / 1.6021918e-19);
            noise_variable_25 = noise_metadata_schedule_26_e322;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_27_e325: f64 = (100.0 * noise_variable_25);
            let noise_metadata_schedule_27_e327: f64 = (noise_metadata_schedule_27_e325 * noise_variable_25);
            noise_variable_57 = noise_metadata_schedule_27_e327;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_28_e330: f64 = (1.0 / noise_variable_25);
            noise_variable_26 = noise_metadata_schedule_28_e330;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_29_e334: f64 = (noise_variable_18 * params.p42);
            let noise_metadata_schedule_29_e335: f64 = (params.p23 + noise_metadata_schedule_29_e334);
            noise_variable_28 = noise_metadata_schedule_29_e335;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_30_e338: f64 = (noise_variable_20).powf(params.p43);
            noise_variable_27 = noise_metadata_schedule_30_e338;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_31_e341: f64 = (params.p36 * noise_variable_27);
            noise_variable_29 = noise_metadata_schedule_31_e341;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_32_e344: f64 = (noise_variable_20).powf(params.p44);
            noise_variable_27 = noise_metadata_schedule_32_e344;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_33_e347: f64 = (params.p37 * noise_variable_27);
            noise_variable_30 = noise_metadata_schedule_33_e347;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_34_e350: f64 = (noise_variable_20).powf(params.p45);
            noise_variable_27 = noise_metadata_schedule_34_e350;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_35_e353: f64 = (params.p38 * noise_variable_27);
            noise_variable_31 = noise_metadata_schedule_35_e353;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_36_e356: f64 = (noise_variable_20).powf(params.p46);
            noise_variable_27 = noise_metadata_schedule_36_e356;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_37_e359: f64 = (params.p39 * noise_variable_27);
            noise_variable_32 = noise_metadata_schedule_37_e359;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_38_e362: f64 = (noise_variable_19).powf(params.p47);
            noise_variable_27 = noise_metadata_schedule_38_e362;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_39_e365: f64 = (params.p40 * noise_variable_27);
            noise_variable_33 = noise_metadata_schedule_39_e365;
        }
        if matches!(source_index, 2 | 3 | 4 | 5 | 6) {
            let noise_metadata_schedule_40_e368: f64 = (4.0 * 1.3806505e-23);
            let noise_metadata_schedule_40_e370: f64 = (noise_metadata_schedule_40_e368 * noise_variable_14);
            noise_variable_71 = noise_metadata_schedule_40_e370;
        }
        if matches!(source_index, 0 | 2 | 3 | 5 | 6) {
            noise_variable_21 = params.p1;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6) {
            noise_variable_22 = params.p0;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_47_e387: f64 = (noise_variable_21 + params.p31);
            noise_variable_23 = noise_metadata_schedule_47_e387;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_48_e390: f64 = (noise_variable_22 + params.p32);
            noise_variable_24 = noise_metadata_schedule_48_e390;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_51_e402: f64 = (noise_variable_14 * 3.05e-7);
            let noise_metadata_schedule_51_e403: f64 = (9.025e-5 + noise_metadata_schedule_51_e402);
            let noise_metadata_schedule_51_e404: f64 = (noise_variable_14 * noise_metadata_schedule_51_e403);
            let noise_metadata_schedule_51_e405: f64 = (1.179 - noise_metadata_schedule_51_e404);
            noise_variable_42 = noise_metadata_schedule_51_e405;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_52_e409: f64 = (0.00045 * noise_variable_14);
            let noise_metadata_schedule_52_e410: f64 = (1.045 + noise_metadata_schedule_52_e409);
            let noise_metadata_schedule_52_e414: f64 = (0.0014 * noise_variable_14);
            let noise_metadata_schedule_52_e415: f64 = (0.523 + noise_metadata_schedule_52_e414);
            let noise_metadata_schedule_52_e418: f64 = (1.48e-6 * noise_variable_15);
            let noise_metadata_schedule_52_e419: f64 = (noise_metadata_schedule_52_e415 - noise_metadata_schedule_52_e418);
            let noise_metadata_schedule_52_e420: f64 = (noise_metadata_schedule_52_e410 * noise_metadata_schedule_52_e419);
            let noise_metadata_schedule_52_e422: f64 = (noise_metadata_schedule_52_e420 * noise_variable_15);
            let noise_metadata_schedule_52_e424: f64 = (noise_metadata_schedule_52_e422 / 90000.0);
            noise_variable_48 = noise_metadata_schedule_52_e424;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_53_e427: f64 = (noise_variable_48).max(0.001);
            noise_variable_48 = noise_metadata_schedule_53_e427;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_54_e429: f64 = (noise_variable_48).sqrt();
            noise_variable_7 = noise_metadata_schedule_54_e429;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_55_e431: f64 = (noise_variable_7).sqrt();
            noise_variable_8 = noise_metadata_schedule_55_e431;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_56_e435: f64 = (2.5e25 * noise_variable_7);
            let noise_metadata_schedule_56_e437: f64 = (noise_metadata_schedule_56_e435 * noise_variable_8);
            let noise_metadata_schedule_56_e438: f64 = (1.0 / noise_metadata_schedule_56_e437);
            noise_variable_10 = noise_metadata_schedule_56_e438;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_57_e442: f64 = (2.0 * noise_variable_25);
            let noise_metadata_schedule_57_e445: f64 = (params.p24 * noise_variable_10);
            let noise_metadata_schedule_57_e446: f64 = (noise_metadata_schedule_57_e445).ln();
            let noise_metadata_schedule_57_e447: f64 = (noise_metadata_schedule_57_e442 * noise_metadata_schedule_57_e446);
            let noise_metadata_schedule_57_e448: f64 = (noise_variable_42 + noise_metadata_schedule_57_e447);
            noise_variable_47 = noise_metadata_schedule_57_e448;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_58_e452: f64 = (2.0 * noise_variable_25);
            let noise_metadata_schedule_58_e455: f64 = (params.p29 * noise_variable_10);
            let noise_metadata_schedule_58_e456: f64 = (noise_metadata_schedule_58_e455).ln();
            let noise_metadata_schedule_58_e457: f64 = (noise_metadata_schedule_58_e452 * noise_metadata_schedule_58_e456);
            let noise_metadata_schedule_58_e458: f64 = (noise_variable_42 + noise_metadata_schedule_58_e457);
            noise_variable_49 = noise_metadata_schedule_58_e458;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_59_e462: f64 = (6.0 * noise_variable_25);
            let noise_metadata_schedule_59_e463: f64 = (noise_variable_42 + noise_metadata_schedule_59_e462);
            noise_variable_135 = noise_metadata_schedule_59_e463;
        }
        if matches!(source_index, 0 | 1 | 6) {
            let noise_metadata_schedule_60_e465: f64 = (noise_variable_26).sqrt();
            noise_variable_6 = noise_metadata_schedule_60_e465;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_61_e468: f64 = (noise_variable_13 * noise_variable_6);
            noise_variable_35 = noise_metadata_schedule_61_e468;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_62_e471: f64 = (noise_variable_35 * noise_variable_35);
            noise_variable_38 = noise_metadata_schedule_62_e471;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_63_e474: f64 = (1.0 / noise_variable_38);
            noise_variable_39 = noise_metadata_schedule_63_e474;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_64_e478: f64 = (noise_variable_35 * 0.7071067811865475);
            let noise_metadata_schedule_64_e479: f64 = (1.0 + noise_metadata_schedule_64_e478);
            noise_variable_45 = noise_metadata_schedule_64_e479;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_65_e482: f64 = (1.0 / noise_variable_45);
            noise_variable_46 = noise_metadata_schedule_65_e482;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_66_e485: f64 = (1e-5 * noise_variable_45);
            noise_variable_41 = noise_metadata_schedule_66_e485;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_67_e488: f64 = (noise_variable_49 * noise_variable_26);
            noise_variable_51 = noise_metadata_schedule_67_e488;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_68_e491: f64 = (noise_variable_109 * noise_variable_6);
            noise_variable_110 = noise_metadata_schedule_68_e491;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_69_e494: f64 = (noise_variable_110 * noise_variable_110);
            noise_variable_111 = noise_metadata_schedule_69_e494;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_70_e498: f64 = (noise_variable_110 * 0.7071067811865475);
            let noise_metadata_schedule_70_e499: f64 = (1.0 + noise_metadata_schedule_70_e498);
            noise_variable_112 = noise_metadata_schedule_70_e499;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_71_e502: f64 = (1e-5 * noise_variable_112);
            noise_variable_113 = noise_metadata_schedule_71_e502;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_73_e517: f64 = if noise_variable_51 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_157 = noise_metadata_schedule_73_e517;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_74_e523,) = {
    if (noise_variable_157 != 0.0) {
        let noise_metadata_schedule_74_e520: f64 = (-noise_variable_51);
        let noise_metadata_schedule_74_e521: f64 = (noise_metadata_schedule_74_e520).exp();
        (noise_metadata_schedule_74_e521,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_74_e523;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_75_e550,) = {
    if (noise_variable_157 == 0.0) {
        let noise_metadata_schedule_75_e530: f64 = (noise_variable_51 - 460.51701859880916);
        let noise_metadata_schedule_75_e535: f64 = (noise_variable_51 - 460.51701859880916);
        let noise_metadata_schedule_75_e536: f64 = (0.5 * noise_metadata_schedule_75_e535);
        let noise_metadata_schedule_75_e540: f64 = (noise_variable_51 - 460.51701859880916);
        let noise_metadata_schedule_75_e542: f64 = (noise_metadata_schedule_75_e540 * 0.3333333333333333);
        let noise_metadata_schedule_75_e543: f64 = (1.0 + noise_metadata_schedule_75_e542);
        let noise_metadata_schedule_75_e544: f64 = (noise_metadata_schedule_75_e536 * noise_metadata_schedule_75_e543);
        let noise_metadata_schedule_75_e545: f64 = (1.0 + noise_metadata_schedule_75_e544);
        let noise_metadata_schedule_75_e546: f64 = (noise_metadata_schedule_75_e530 * noise_metadata_schedule_75_e545);
        let noise_metadata_schedule_75_e547: f64 = (1.0 + noise_metadata_schedule_75_e546);
        let noise_metadata_schedule_75_e548: f64 = (1e-200 / noise_metadata_schedule_75_e547);
        (noise_metadata_schedule_75_e548,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_75_e550;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_77_e575,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_77_e563: f64 = (noise_variable_29 * noise_variable_22);
        let noise_metadata_schedule_77_e567: f64 = (params.p2 - 1.0);
        let noise_metadata_schedule_77_e569: f64 = (noise_metadata_schedule_77_e567 * 9.0);
        let noise_metadata_schedule_77_e570: f64 = (3.0 + noise_metadata_schedule_77_e569);
        let noise_metadata_schedule_77_e572: f64 = (noise_metadata_schedule_77_e570 * noise_variable_21);
        let noise_metadata_schedule_77_e573: f64 = (noise_metadata_schedule_77_e563 / noise_metadata_schedule_77_e572);
        (noise_metadata_schedule_77_e573,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_77_e575;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_78_e583,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_78_e580: f64 = (noise_variable_22 * noise_variable_21);
        let noise_metadata_schedule_78_e581: f64 = (noise_variable_30 / noise_metadata_schedule_78_e580);
        (noise_metadata_schedule_78_e581,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_78_e583;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_79_e593,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_79_e589: f64 = (noise_variable_22 + params.p33);
        let noise_metadata_schedule_79_e590: f64 = (2.0 * noise_metadata_schedule_79_e589);
        let noise_metadata_schedule_79_e591: f64 = (noise_variable_31 / noise_metadata_schedule_79_e590);
        (noise_metadata_schedule_79_e591,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_79_e593;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_80_e605,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_80_e597: f64 = (noise_variable_32 * noise_variable_21);
        let noise_metadata_schedule_80_e601: f64 = (noise_variable_22 + params.p33);
        let noise_metadata_schedule_80_e602: f64 = (12.0 * noise_metadata_schedule_80_e601);
        let noise_metadata_schedule_80_e603: f64 = (noise_metadata_schedule_80_e597 / noise_metadata_schedule_80_e602);
        (noise_metadata_schedule_80_e603,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_80_e605;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_81_e619,) = {
    if (params.p16 != 0.0) {
        let (noise_metadata_schedule_81_e617,) = {
            if (noise_variable_62 > 0.001) {
                let (noise_metadata_schedule_81_e615,) = {
                    if (noise_variable_62 < 1000.0) {
                        (noise_variable_62,)
                    } else {
                        (1000.0,)
                    }
                };
                (noise_metadata_schedule_81_e615,)
            } else {
                (0.001,)
            }
        };
        (noise_metadata_schedule_81_e617,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_81_e619;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_82_e633,) = {
    if (params.p16 != 0.0) {
        let (noise_metadata_schedule_82_e631,) = {
            if (noise_variable_64 > 0.001) {
                let (noise_metadata_schedule_82_e629,) = {
                    if (noise_variable_64 < 100.0) {
                        (noise_variable_64,)
                    } else {
                        (100.0,)
                    }
                };
                (noise_metadata_schedule_82_e629,)
            } else {
                (0.001,)
            }
        };
        (noise_metadata_schedule_82_e631,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_82_e633;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_83_e647,) = {
    if (params.p16 != 0.0) {
        let (noise_metadata_schedule_83_e645,) = {
            if (noise_variable_68 > 0.001) {
                let (noise_metadata_schedule_83_e643,) = {
                    if (noise_variable_68 < 1000.0) {
                        (noise_variable_68,)
                    } else {
                        (1000.0,)
                    }
                };
                (noise_metadata_schedule_83_e643,)
            } else {
                (0.001,)
            }
        };
        (noise_metadata_schedule_83_e645,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_83_e647;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_84_e661,) = {
    if (params.p16 != 0.0) {
        let (noise_metadata_schedule_84_e659,) = {
            if (noise_variable_66 > 0.001) {
                let (noise_metadata_schedule_84_e657,) = {
                    if (noise_variable_66 < 1000.0) {
                        (noise_variable_66,)
                    } else {
                        (1000.0,)
                    }
                };
                (noise_metadata_schedule_84_e657,)
            } else {
                (0.001,)
            }
        };
        (noise_metadata_schedule_84_e659,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_84_e661;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_85_e675,) = {
    if (params.p16 != 0.0) {
        let (noise_metadata_schedule_85_e673,) = {
            if (noise_variable_33 > 0.001) {
                let (noise_metadata_schedule_85_e671,) = {
                    if (noise_variable_33 < 20.0) {
                        (noise_variable_33,)
                    } else {
                        (20.0,)
                    }
                };
                (noise_metadata_schedule_85_e671,)
            } else {
                (0.001,)
            }
        };
        (noise_metadata_schedule_85_e673,)
    } else {
        (noise_variable_33,)
    }
};
            noise_variable_33 = noise_metadata_schedule_85_e675;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_86_e681,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_86_e679: f64 = (1.0 / noise_variable_62);
        (noise_metadata_schedule_86_e679,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_86_e681;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_87_e687,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_87_e685: f64 = (1.0 / noise_variable_64);
        (noise_metadata_schedule_87_e685,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_87_e687;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_88_e693,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_88_e691: f64 = (1.0 / noise_variable_68);
        (noise_metadata_schedule_88_e691,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_88_e693;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_89_e699,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_89_e697: f64 = (1.0 / noise_variable_66);
        (noise_metadata_schedule_89_e697,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_89_e699;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_90_e709,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_90_e703: f64 = (12.0 * noise_variable_33);
        let noise_metadata_schedule_90_e705: f64 = (noise_metadata_schedule_90_e703 * noise_variable_22);
        let noise_metadata_schedule_90_e707: f64 = (noise_metadata_schedule_90_e705 / noise_variable_21);
        (noise_metadata_schedule_90_e707,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_90_e709;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_91_e714,) = {
    if (params.p16 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_91_e714;
        }
        if matches!(source_index, 3) {
            let (noise_metadata_schedule_92_e719,) = {
    if (params.p16 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_92_e719;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_93_e724,) = {
    if (params.p16 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_93_e724;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_94_e729,) = {
    if (params.p16 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_94_e729;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_95_e734,) = {
    if (params.p16 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_95_e734;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_96_e737: f64 = (noise_variable_71 * noise_variable_63);
            noise_variable_72 = noise_metadata_schedule_96_e737;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_97_e740: f64 = (noise_variable_71 * noise_variable_65);
            noise_variable_73 = noise_metadata_schedule_97_e740;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_98_e743: f64 = (noise_variable_71 * noise_variable_69);
            noise_variable_74 = noise_metadata_schedule_98_e743;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_99_e746: f64 = (noise_variable_71 * noise_variable_67);
            noise_variable_75 = noise_metadata_schedule_99_e746;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_100_e749: f64 = if params.p66 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_158 = noise_metadata_schedule_100_e749;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_101_e753,) = {
    if (noise_variable_158 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_101_e753;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_102_e760,) = {
    if (noise_variable_158 == 0.0) {
        let noise_metadata_schedule_102_e758: f64 = (noise_variable_71 * noise_variable_70);
        (noise_metadata_schedule_102_e758,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_102_e760;
        }
        if matches!(source_index, 1) {
            noise_variable_127 = 0.0;
        }
        if matches!(source_index, 0) {
            noise_variable_128 = 0.0;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_105_e772,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_105_e766: f64 = (params.p55 * noise_variable_24);
        let noise_metadata_schedule_105_e768: f64 = (noise_metadata_schedule_105_e766 * noise_variable_23);
        let noise_metadata_schedule_105_e770: f64 = (noise_metadata_schedule_105_e768 * 1000000000000.0);
        (noise_metadata_schedule_105_e770,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_105_e772;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_106_e784,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_106_e776: f64 = (2.0 * params.p56);
        let noise_metadata_schedule_106_e778: f64 = (noise_metadata_schedule_106_e776 * params.p53);
        let noise_metadata_schedule_106_e780: f64 = (noise_metadata_schedule_106_e778 * noise_variable_24);
        let noise_metadata_schedule_106_e782: f64 = (noise_metadata_schedule_106_e780 * 1000000000000.0);
        (noise_metadata_schedule_106_e782,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_106_e784;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_107_e794,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_107_e788: f64 = (params.p60 * noise_variable_24);
        let noise_metadata_schedule_107_e790: f64 = (noise_metadata_schedule_107_e788 * noise_variable_23);
        let noise_metadata_schedule_107_e792: f64 = (noise_metadata_schedule_107_e790 * 1000000000000.0);
        (noise_metadata_schedule_107_e792,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_107_e794;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_108_e806,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_108_e798: f64 = (2.0 * params.p61);
        let noise_metadata_schedule_108_e800: f64 = (noise_metadata_schedule_108_e798 * params.p53);
        let noise_metadata_schedule_108_e802: f64 = (noise_metadata_schedule_108_e800 * noise_variable_24);
        let noise_metadata_schedule_108_e804: f64 = (noise_metadata_schedule_108_e802 * 1000000000000.0);
        (noise_metadata_schedule_108_e804,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_108_e806;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_109_e812,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_109_e810: f64 = (noise_variable_19).powf(params.p52);
        (noise_metadata_schedule_109_e810,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_109_e812;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_110_e818,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_110_e816: f64 = (noise_variable_125 * noise_variable_119);
        (noise_metadata_schedule_110_e816,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_110_e818;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_111_e824,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_111_e822: f64 = (noise_variable_126 * noise_variable_119);
        (noise_metadata_schedule_111_e822,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_111_e824;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_112_e830,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_112_e828: f64 = (noise_variable_137 * noise_variable_119);
        (noise_metadata_schedule_112_e828,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_112_e830;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_113_e836,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_113_e834: f64 = (noise_variable_138 * noise_variable_119);
        (noise_metadata_schedule_113_e834,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_113_e836;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_114_e842,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_114_e840: f64 = (1.0 / params.p50);
        (noise_metadata_schedule_114_e840,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_114_e842;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_115_e848,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_115_e846: f64 = (1.0 / params.p51);
        (noise_metadata_schedule_115_e846,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_115_e848;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_116_e865,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_116_e852: f64 = (4.0 * 0.3333333333333333);
        let noise_metadata_schedule_116_e855: f64 = (2.0 * 1.6021918e-19);
        let noise_metadata_schedule_116_e857: f64 = (noise_metadata_schedule_116_e855 * 9.1093826e-31);
        let noise_metadata_schedule_116_e859: f64 = (noise_metadata_schedule_116_e857 * params.p50);
        let noise_metadata_schedule_116_e860: f64 = (noise_metadata_schedule_116_e859).sqrt();
        let noise_metadata_schedule_116_e861: f64 = (noise_metadata_schedule_116_e852 * noise_metadata_schedule_116_e860);
        let noise_metadata_schedule_116_e863: f64 = (noise_metadata_schedule_116_e861 / 1.05457168e-34);
        (noise_metadata_schedule_116_e863,)
    } else {
        (noise_variable_9,)
    }
};
            noise_variable_9 = noise_metadata_schedule_116_e865;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_117_e871,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_117_e869: f64 = (noise_variable_9 * params.p19);
        (noise_metadata_schedule_117_e869,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_117_e871;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_118_e875,) = {
    if (params.p49 != 0.0) {
        (noise_variable_122,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_118_e875;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_119_e892,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_119_e879: f64 = (4.0 * 0.3333333333333333);
        let noise_metadata_schedule_119_e882: f64 = (2.0 * 1.6021918e-19);
        let noise_metadata_schedule_119_e884: f64 = (noise_metadata_schedule_119_e882 * 9.1093826e-31);
        let noise_metadata_schedule_119_e886: f64 = (noise_metadata_schedule_119_e884 * params.p51);
        let noise_metadata_schedule_119_e887: f64 = (noise_metadata_schedule_119_e886).sqrt();
        let noise_metadata_schedule_119_e888: f64 = (noise_metadata_schedule_119_e879 * noise_metadata_schedule_119_e887);
        let noise_metadata_schedule_119_e890: f64 = (noise_metadata_schedule_119_e888 / 1.05457168e-34);
        (noise_metadata_schedule_119_e890,)
    } else {
        (noise_variable_9,)
    }
};
            noise_variable_9 = noise_metadata_schedule_119_e892;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_120_e898,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_120_e896: f64 = (noise_variable_9 * params.p19);
        (noise_metadata_schedule_120_e896,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_120_e898;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_121_e902,) = {
    if (params.p49 != 0.0) {
        (noise_variable_132,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_121_e902;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_122_e905: f64 = if params.p59 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_159 = noise_metadata_schedule_122_e905;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_123_e916,) = {
    if ((params.p49 != 0.0) && (noise_variable_159 != 0.0)) {
        let noise_metadata_schedule_123_e910: f64 = (-0.495);
        let noise_metadata_schedule_123_e912: f64 = (noise_metadata_schedule_123_e910 * params.p58);
        let noise_metadata_schedule_123_e914: f64 = (noise_metadata_schedule_123_e912 / params.p59);
        (noise_metadata_schedule_123_e914,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_123_e916;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_124_e923,) = {
    if ((params.p49 != 0.0) && (noise_variable_159 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_124_e923;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_125_e926: f64 = if params.p64 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_160 = noise_metadata_schedule_125_e926;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_126_e937,) = {
    if ((params.p49 != 0.0) && (noise_variable_160 != 0.0)) {
        let noise_metadata_schedule_126_e931: f64 = (-0.495);
        let noise_metadata_schedule_126_e933: f64 = (noise_metadata_schedule_126_e931 * params.p63);
        let noise_metadata_schedule_126_e935: f64 = (noise_metadata_schedule_126_e933 / params.p64);
        (noise_metadata_schedule_126_e935,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_126_e937;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_127_e944,) = {
    if ((params.p49 != 0.0) && (noise_variable_160 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_127_e944;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_128_e954,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_128_e949: f64 = (params.p17 * noise_variable_47);
        let noise_metadata_schedule_128_e951: f64 = (noise_metadata_schedule_128_e949 + noise_variable_42);
        let noise_metadata_schedule_128_e952: f64 = (0.5 * noise_metadata_schedule_128_e951);
        (noise_metadata_schedule_128_e952,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_128_e954;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_129_e964,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_129_e959: f64 = (params.p17 * noise_variable_135);
        let noise_metadata_schedule_129_e961: f64 = (noise_metadata_schedule_129_e959 + noise_variable_42);
        let noise_metadata_schedule_129_e962: f64 = (0.5 * noise_metadata_schedule_129_e961);
        (noise_metadata_schedule_129_e962,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_129_e964;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_130_e970,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_130_e968: f64 = (params.p57 * noise_variable_25);
        (noise_metadata_schedule_130_e968,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_130_e970;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_131_e976,) = {
    if (params.p49 != 0.0) {
        let noise_metadata_schedule_131_e974: f64 = (params.p62 * noise_variable_25);
        (noise_metadata_schedule_131_e974,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_131_e976;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_132_e981,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_132_e981;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_133_e986,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_133_e986;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_134_e991,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_134_e991;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_135_e996,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_135_e996;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_136_e1001,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_136_e1001;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_137_e1006,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_137_e1006;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_138_e1011,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_138_e1011;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_139_e1016,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_139_e1016;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_140_e1021,) = {
    if (params.p49 == 0.0) {
        (0.1,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_140_e1021;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_141_e1026,) = {
    if (params.p49 == 0.0) {
        (0.1,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_141_e1026;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_142_e1031,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_142_e1031;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_143_e1036,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_143_e1036;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_144_e1041,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_144_e1041;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_145_e1046,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_145_e1046;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_146_e1051,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_146_e1051;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_147_e1056,) = {
    if (params.p49 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_147_e1056;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_148_e1062: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
            let noise_metadata_schedule_148_e1063: f64 = (params.p17 * noise_metadata_schedule_148_e1062);
            let noise_metadata_schedule_148_e1065: f64 = noise_metadata_schedule_148_e1063;
            let (noise_metadata_schedule_148_e1156,) = {
    if (noise_metadata_schedule_148_e1065 > 1e-16) {
        let noise_metadata_schedule_148_e1073: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
        let noise_metadata_schedule_148_e1074: f64 = (params.p17 * noise_metadata_schedule_148_e1073);
        let noise_metadata_schedule_148_e1076: f64 = noise_metadata_schedule_148_e1074;
        let noise_metadata_schedule_148_e1080: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
        let noise_metadata_schedule_148_e1081: f64 = (params.p17 * noise_metadata_schedule_148_e1080);
        let noise_metadata_schedule_148_e1083: f64 = noise_metadata_schedule_148_e1081;
        let noise_metadata_schedule_148_e1087: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
        let noise_metadata_schedule_148_e1088: f64 = (params.p17 * noise_metadata_schedule_148_e1087);
        let noise_metadata_schedule_148_e1090: f64 = noise_metadata_schedule_148_e1088;
        let noise_metadata_schedule_148_e1091: f64 = (noise_metadata_schedule_148_e1083 * noise_metadata_schedule_148_e1090);
        let noise_metadata_schedule_148_e1093: f64 = (noise_metadata_schedule_148_e1091 + params.p28);
        let noise_metadata_schedule_148_e1094: f64 = (noise_metadata_schedule_148_e1093).sqrt();
        let noise_metadata_schedule_148_e1095: f64 = (noise_metadata_schedule_148_e1076 + noise_metadata_schedule_148_e1094);
        let noise_metadata_schedule_148_e1096: f64 = (0.5 * noise_metadata_schedule_148_e1095);
        let noise_metadata_schedule_148_e1097: f64 = noise_metadata_schedule_148_e1096;
        (noise_metadata_schedule_148_e1097,)
    } else {
        let noise_metadata_schedule_148_e1102: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
        let noise_metadata_schedule_148_e1103: f64 = (params.p17 * noise_metadata_schedule_148_e1102);
        let noise_metadata_schedule_148_e1104: f64 = (-noise_metadata_schedule_148_e1103);
        let (noise_metadata_schedule_148_e1155,) = {
            if (noise_metadata_schedule_148_e1104 > 1e-16) {
                let noise_metadata_schedule_148_e1110: f64 = (0.5 * params.p28);
                let noise_metadata_schedule_148_e1115: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
                let noise_metadata_schedule_148_e1116: f64 = (params.p17 * noise_metadata_schedule_148_e1115);
                let noise_metadata_schedule_148_e1117: f64 = (-noise_metadata_schedule_148_e1116);
                let noise_metadata_schedule_148_e1122: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
                let noise_metadata_schedule_148_e1123: f64 = (params.p17 * noise_metadata_schedule_148_e1122);
                let noise_metadata_schedule_148_e1124: f64 = (-noise_metadata_schedule_148_e1123);
                let noise_metadata_schedule_148_e1129: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
                let noise_metadata_schedule_148_e1130: f64 = (params.p17 * noise_metadata_schedule_148_e1129);
                let noise_metadata_schedule_148_e1131: f64 = (-noise_metadata_schedule_148_e1130);
                let noise_metadata_schedule_148_e1132: f64 = (noise_metadata_schedule_148_e1124 * noise_metadata_schedule_148_e1131);
                let noise_metadata_schedule_148_e1134: f64 = (noise_metadata_schedule_148_e1132 + params.p28);
                let noise_metadata_schedule_148_e1135: f64 = (noise_metadata_schedule_148_e1134).sqrt();
                let noise_metadata_schedule_148_e1136: f64 = (noise_metadata_schedule_148_e1117 + noise_metadata_schedule_148_e1135);
                let noise_metadata_schedule_148_e1137: f64 = (noise_metadata_schedule_148_e1110 / noise_metadata_schedule_148_e1136);
                let noise_metadata_schedule_148_e1138: f64 = noise_metadata_schedule_148_e1137;
                (noise_metadata_schedule_148_e1138,)
            } else {
                let noise_metadata_schedule_148_e1144: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - params.p27);
                let noise_metadata_schedule_148_e1145: f64 = (params.p17 * noise_metadata_schedule_148_e1144);
                let noise_metadata_schedule_148_e1147: f64 = noise_metadata_schedule_148_e1145;
                let noise_metadata_schedule_148_e1150: f64 = (1e-32 + params.p28);
                let noise_metadata_schedule_148_e1151: f64 = (noise_metadata_schedule_148_e1150).sqrt();
                let noise_metadata_schedule_148_e1152: f64 = (noise_metadata_schedule_148_e1147 + noise_metadata_schedule_148_e1151);
                let noise_metadata_schedule_148_e1153: f64 = (0.5 * noise_metadata_schedule_148_e1152);
                let noise_metadata_schedule_148_e1154: f64 = noise_metadata_schedule_148_e1153;
                (noise_metadata_schedule_148_e1154,)
            }
        };
        (noise_metadata_schedule_148_e1155,)
    }
};
            let noise_metadata_schedule_148_e1157: f64 = (params.p26 * noise_metadata_schedule_148_e1156);
            let noise_metadata_schedule_148_e1158: f64 = (1.0 + noise_metadata_schedule_148_e1157);
            noise_variable_108 = noise_metadata_schedule_148_e1158;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_149_e1162: f64 = (params.p25 - noise_variable_108);
            let (noise_metadata_schedule_149_e1221,) = {
    if (noise_metadata_schedule_149_e1162 > 1e-16) {
        let noise_metadata_schedule_149_e1169: f64 = (params.p25 - noise_variable_108);
        let noise_metadata_schedule_149_e1172: f64 = (params.p25 - noise_variable_108);
        let noise_metadata_schedule_149_e1175: f64 = (params.p25 - noise_variable_108);
        let noise_metadata_schedule_149_e1176: f64 = (noise_metadata_schedule_149_e1172 * noise_metadata_schedule_149_e1175);
        let noise_metadata_schedule_149_e1178: f64 = (noise_metadata_schedule_149_e1176 + 1e-6);
        let noise_metadata_schedule_149_e1179: f64 = (noise_metadata_schedule_149_e1178).sqrt();
        let noise_metadata_schedule_149_e1180: f64 = (noise_metadata_schedule_149_e1169 + noise_metadata_schedule_149_e1179);
        let noise_metadata_schedule_149_e1181: f64 = (0.5 * noise_metadata_schedule_149_e1180);
        let noise_metadata_schedule_149_e1182: f64 = (params.p25 - noise_metadata_schedule_149_e1181);
        (noise_metadata_schedule_149_e1182,)
    } else {
        let noise_metadata_schedule_149_e1185: f64 = (noise_variable_108 - params.p25);
        let (noise_metadata_schedule_149_e1220,) = {
            if (noise_metadata_schedule_149_e1185 > 1e-16) {
                let noise_metadata_schedule_149_e1191: f64 = (0.5 * 1e-6);
                let noise_metadata_schedule_149_e1194: f64 = (noise_variable_108 - params.p25);
                let noise_metadata_schedule_149_e1197: f64 = (noise_variable_108 - params.p25);
                let noise_metadata_schedule_149_e1200: f64 = (noise_variable_108 - params.p25);
                let noise_metadata_schedule_149_e1201: f64 = (noise_metadata_schedule_149_e1197 * noise_metadata_schedule_149_e1200);
                let noise_metadata_schedule_149_e1203: f64 = (noise_metadata_schedule_149_e1201 + 1e-6);
                let noise_metadata_schedule_149_e1204: f64 = (noise_metadata_schedule_149_e1203).sqrt();
                let noise_metadata_schedule_149_e1205: f64 = (noise_metadata_schedule_149_e1194 + noise_metadata_schedule_149_e1204);
                let noise_metadata_schedule_149_e1206: f64 = (noise_metadata_schedule_149_e1191 / noise_metadata_schedule_149_e1205);
                let noise_metadata_schedule_149_e1207: f64 = (params.p25 - noise_metadata_schedule_149_e1206);
                (noise_metadata_schedule_149_e1207,)
            } else {
                let noise_metadata_schedule_149_e1212: f64 = (params.p25 - noise_variable_108);
                let noise_metadata_schedule_149_e1215: f64 = (1e-32 + 1e-6);
                let noise_metadata_schedule_149_e1216: f64 = (noise_metadata_schedule_149_e1215).sqrt();
                let noise_metadata_schedule_149_e1217: f64 = (noise_metadata_schedule_149_e1212 + noise_metadata_schedule_149_e1216);
                let noise_metadata_schedule_149_e1218: f64 = (0.5 * noise_metadata_schedule_149_e1217);
                let noise_metadata_schedule_149_e1219: f64 = (params.p25 - noise_metadata_schedule_149_e1218);
                (noise_metadata_schedule_149_e1219,)
            }
        };
        (noise_metadata_schedule_149_e1220,)
    }
};
            let noise_metadata_schedule_149_e1222: f64 = (params.p24 * noise_metadata_schedule_149_e1221);
            noise_variable_107 = noise_metadata_schedule_149_e1222;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_150_e1225: f64 = (noise_variable_107 / 1e23);
            noise_variable_140 = noise_metadata_schedule_150_e1225;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_151_e1229: f64 = (2.0 * noise_variable_25);
            let noise_metadata_schedule_151_e1232: f64 = (noise_variable_107 * noise_variable_10);
            let noise_metadata_schedule_151_e1233: f64 = (noise_metadata_schedule_151_e1232).ln();
            let noise_metadata_schedule_151_e1234: f64 = (noise_metadata_schedule_151_e1229 * noise_metadata_schedule_151_e1233);
            let noise_metadata_schedule_151_e1235: f64 = (noise_variable_42 + noise_metadata_schedule_151_e1234);
            noise_variable_47 = noise_metadata_schedule_151_e1235;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_152_e1238: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_152_e1240: f64 = (noise_metadata_schedule_152_e1238 * 1.045e-10);
            let noise_metadata_schedule_152_e1242: f64 = (noise_metadata_schedule_152_e1240 * noise_variable_107);
            let noise_metadata_schedule_152_e1243: f64 = (noise_metadata_schedule_152_e1242).sqrt();
            let noise_metadata_schedule_152_e1245: f64 = (noise_metadata_schedule_152_e1243 / noise_variable_11);
            noise_variable_12 = noise_metadata_schedule_152_e1245;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_153_e1248: f64 = if params.p30 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_161 = noise_metadata_schedule_153_e1248;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_154_e1257,) = {
    if (noise_variable_161 != 0.0) {
        let noise_metadata_schedule_154_e1252: f64 = (noise_variable_12 * noise_variable_12);
        let noise_metadata_schedule_154_e1254: f64 = (noise_metadata_schedule_154_e1252 * noise_variable_47);
        let noise_metadata_schedule_154_e1255: f64 = (noise_metadata_schedule_154_e1254).sqrt();
        (noise_metadata_schedule_154_e1255,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_154_e1257;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_155_e1267,) = {
    if (noise_variable_161 != 0.0) {
        let noise_metadata_schedule_155_e1261: f64 = (0.75 * noise_variable_54);
        let noise_metadata_schedule_155_e1264: f64 = (noise_variable_55).powf(0.6666666666666666);
        let noise_metadata_schedule_155_e1265: f64 = (noise_metadata_schedule_155_e1261 * noise_metadata_schedule_155_e1264);
        (noise_metadata_schedule_155_e1265,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_155_e1267;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_156_e1273,) = {
    if (noise_variable_161 != 0.0) {
        let noise_metadata_schedule_156_e1271: f64 = (noise_variable_47 + noise_variable_56);
        (noise_metadata_schedule_156_e1271,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_156_e1273;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_157_e1287,) = {
    if (noise_variable_161 != 0.0) {
        let noise_metadata_schedule_157_e1279: f64 = (2.0 * 0.6666666666666666);
        let noise_metadata_schedule_157_e1281: f64 = (noise_metadata_schedule_157_e1279 * noise_variable_56);
        let noise_metadata_schedule_157_e1283: f64 = (noise_metadata_schedule_157_e1281 / noise_variable_55);
        let noise_metadata_schedule_157_e1284: f64 = (1.0 + noise_metadata_schedule_157_e1283);
        let noise_metadata_schedule_157_e1285: f64 = (noise_variable_12 * noise_metadata_schedule_157_e1284);
        (noise_metadata_schedule_157_e1285,)
    } else {
        (noise_variable_12,)
    }
};
            noise_variable_12 = noise_metadata_schedule_157_e1287;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_158_e1289: f64 = (noise_variable_26).sqrt();
            noise_variable_6 = noise_metadata_schedule_158_e1289;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_159_e1292: f64 = (noise_variable_12 * noise_variable_6);
            noise_variable_34 = noise_metadata_schedule_159_e1292;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_160_e1295: f64 = (noise_variable_34 * noise_variable_34);
            noise_variable_36 = noise_metadata_schedule_160_e1295;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_161_e1298: f64 = (1.0 / noise_variable_36);
            noise_variable_37 = noise_metadata_schedule_161_e1298;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_162_e1302: f64 = (noise_variable_34 * 0.7071067811865475);
            let noise_metadata_schedule_162_e1303: f64 = (1.0 + noise_metadata_schedule_162_e1302);
            noise_variable_43 = noise_metadata_schedule_162_e1303;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_163_e1306: f64 = (1.0 / noise_variable_43);
            noise_variable_44 = noise_metadata_schedule_163_e1306;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_164_e1309: f64 = (1e-5 * noise_variable_43);
            noise_variable_40 = noise_metadata_schedule_164_e1309;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_165_e1312: f64 = (noise_variable_47 * noise_variable_26);
            noise_variable_50 = noise_metadata_schedule_165_e1312;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_166_e1315: f64 = if noise_variable_50 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_162 = noise_metadata_schedule_166_e1315;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_167_e1321,) = {
    if (noise_variable_162 != 0.0) {
        let noise_metadata_schedule_167_e1318: f64 = (-noise_variable_50);
        let noise_metadata_schedule_167_e1319: f64 = (noise_metadata_schedule_167_e1318).exp();
        (noise_metadata_schedule_167_e1319,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_167_e1321;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_168_e1348,) = {
    if (noise_variable_162 == 0.0) {
        let noise_metadata_schedule_168_e1328: f64 = (noise_variable_50 - 460.51701859880916);
        let noise_metadata_schedule_168_e1333: f64 = (noise_variable_50 - 460.51701859880916);
        let noise_metadata_schedule_168_e1334: f64 = (0.5 * noise_metadata_schedule_168_e1333);
        let noise_metadata_schedule_168_e1338: f64 = (noise_variable_50 - 460.51701859880916);
        let noise_metadata_schedule_168_e1340: f64 = (noise_metadata_schedule_168_e1338 * 0.3333333333333333);
        let noise_metadata_schedule_168_e1341: f64 = (1.0 + noise_metadata_schedule_168_e1340);
        let noise_metadata_schedule_168_e1342: f64 = (noise_metadata_schedule_168_e1334 * noise_metadata_schedule_168_e1341);
        let noise_metadata_schedule_168_e1343: f64 = (1.0 + noise_metadata_schedule_168_e1342);
        let noise_metadata_schedule_168_e1344: f64 = (noise_metadata_schedule_168_e1328 * noise_metadata_schedule_168_e1343);
        let noise_metadata_schedule_168_e1345: f64 = (1.0 + noise_metadata_schedule_168_e1344);
        let noise_metadata_schedule_168_e1346: f64 = (1e-200 / noise_metadata_schedule_168_e1345);
        (noise_metadata_schedule_168_e1346,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_168_e1348;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_169_e1352: f64 = (-1.25);
            let noise_metadata_schedule_169_e1353: f64 = (noise_metadata_schedule_169_e1352).exp();
            let noise_metadata_schedule_169_e1355: f64 = (noise_metadata_schedule_169_e1353 + 1.25);
            let noise_metadata_schedule_169_e1357: f64 = (noise_metadata_schedule_169_e1355 - 1.0);
            let noise_metadata_schedule_169_e1358: f64 = (noise_metadata_schedule_169_e1357).sqrt();
            let noise_metadata_schedule_169_e1359: f64 = (noise_variable_34 * noise_metadata_schedule_169_e1358);
            let noise_metadata_schedule_169_e1360: f64 = (1.25 + noise_metadata_schedule_169_e1359);
            noise_variable_60 = noise_metadata_schedule_169_e1360;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_170_e1364: f64 = (-1.25);
            let noise_metadata_schedule_170_e1365: f64 = (noise_metadata_schedule_170_e1364).exp();
            let noise_metadata_schedule_170_e1367: f64 = (noise_metadata_schedule_170_e1365 + 1.25);
            let noise_metadata_schedule_170_e1369: f64 = (noise_metadata_schedule_170_e1367 - 1.0);
            let noise_metadata_schedule_170_e1370: f64 = (noise_metadata_schedule_170_e1369).sqrt();
            let noise_metadata_schedule_170_e1371: f64 = (noise_variable_110 * noise_metadata_schedule_170_e1370);
            let noise_metadata_schedule_170_e1372: f64 = (1.25 + noise_metadata_schedule_170_e1371);
            noise_variable_116 = noise_metadata_schedule_170_e1372;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_171_e1376: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])) - noise_variable_28);
            let noise_metadata_schedule_171_e1377: f64 = (params.p17 * noise_metadata_schedule_171_e1376);
            noise_variable_77 = noise_metadata_schedule_171_e1377;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_172_e1380: f64 = (noise_variable_77 * noise_variable_26);
            noise_variable_78 = noise_metadata_schedule_172_e1380;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_173_e1382: f64 = (noise_variable_78).abs();
            let noise_metadata_schedule_173_e1384: f64 = if noise_metadata_schedule_173_e1382 <= noise_variable_40 { 1.0 } else { 0.0 };
            noise_variable_184 = noise_metadata_schedule_173_e1384;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_174_e1394,) = {
    if (noise_variable_184 != 0.0) {
        let noise_metadata_schedule_174_e1388: f64 = (noise_variable_44 * noise_variable_44);
        let noise_metadata_schedule_174_e1390: f64 = (noise_metadata_schedule_174_e1388 * 0.1666666666666667);
        let noise_metadata_schedule_174_e1392: f64 = (noise_metadata_schedule_174_e1390 * 0.7071067811865475);
        (noise_metadata_schedule_174_e1392,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_174_e1394;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_175_e1412,) = {
    if (noise_variable_184 != 0.0) {
        let noise_metadata_schedule_175_e1398: f64 = (noise_variable_78 * noise_variable_44);
        let noise_metadata_schedule_175_e1403: f64 = (1.0 - noise_variable_52);
        let noise_metadata_schedule_175_e1404: f64 = (noise_variable_78 * noise_metadata_schedule_175_e1403);
        let noise_metadata_schedule_175_e1406: f64 = (noise_metadata_schedule_175_e1404 * noise_variable_34);
        let noise_metadata_schedule_175_e1408: f64 = (noise_metadata_schedule_175_e1406 * noise_variable_165);
        let noise_metadata_schedule_175_e1409: f64 = (1.0 + noise_metadata_schedule_175_e1408);
        let noise_metadata_schedule_175_e1410: f64 = (noise_metadata_schedule_175_e1398 * noise_metadata_schedule_175_e1409);
        (noise_metadata_schedule_175_e1410,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_175_e1412;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_176_e1415: f64 = (-noise_variable_40);
            let noise_metadata_schedule_176_e1416: f64 = if noise_variable_78 < noise_metadata_schedule_176_e1415 { 1.0 } else { 0.0 };
            noise_variable_185 = noise_metadata_schedule_176_e1416;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_177_e1424,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_177_e1422: f64 = (-noise_variable_78);
        (noise_metadata_schedule_177_e1422,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_177_e1424;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_178_e1435,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_178_e1431: f64 = (1.25 * noise_variable_166);
        let noise_metadata_schedule_178_e1433: f64 = (noise_metadata_schedule_178_e1431 * noise_variable_44);
        (noise_metadata_schedule_178_e1433,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_178_e1435;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_179_e1457,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_179_e1443: f64 = (noise_variable_167 + 10.0);
        let noise_metadata_schedule_179_e1446: f64 = (noise_variable_167 - 6.0);
        let noise_metadata_schedule_179_e1449: f64 = (noise_variable_167 - 6.0);
        let noise_metadata_schedule_179_e1450: f64 = (noise_metadata_schedule_179_e1446 * noise_metadata_schedule_179_e1449);
        let noise_metadata_schedule_179_e1452: f64 = (noise_metadata_schedule_179_e1450 + 64.0);
        let noise_metadata_schedule_179_e1453: f64 = (noise_metadata_schedule_179_e1452).sqrt();
        let noise_metadata_schedule_179_e1454: f64 = (noise_metadata_schedule_179_e1443 - noise_metadata_schedule_179_e1453);
        let noise_metadata_schedule_179_e1455: f64 = (0.5 * noise_metadata_schedule_179_e1454);
        (noise_metadata_schedule_179_e1455,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_179_e1457;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_180_e1466,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_180_e1464: f64 = (noise_variable_166 - noise_variable_174);
        (noise_metadata_schedule_180_e1464,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_180_e1466;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_181_e1481,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_181_e1473: f64 = (noise_variable_164 * noise_variable_164);
        let noise_metadata_schedule_181_e1477: f64 = (noise_variable_174 + 1.0);
        let noise_metadata_schedule_181_e1478: f64 = (noise_variable_36 * noise_metadata_schedule_181_e1477);
        let noise_metadata_schedule_181_e1479: f64 = (noise_metadata_schedule_181_e1473 + noise_metadata_schedule_181_e1478);
        (noise_metadata_schedule_181_e1479,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_181_e1481;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_182_e1492,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_182_e1488: f64 = (2.0 * noise_variable_164);
        let noise_metadata_schedule_182_e1490: f64 = (noise_metadata_schedule_182_e1488 - noise_variable_36);
        (noise_metadata_schedule_182_e1490,)
    } else {
        (noise_variable_171,)
    }
};
            noise_variable_171 = noise_metadata_schedule_182_e1492;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_183_e1505,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_183_e1498: f64 = (-noise_variable_174);
        let noise_metadata_schedule_183_e1501: f64 = (noise_variable_169 * noise_variable_37);
        let noise_metadata_schedule_183_e1502: f64 = (noise_metadata_schedule_183_e1501).ln();
        let noise_metadata_schedule_183_e1503: f64 = (noise_metadata_schedule_183_e1498 + noise_metadata_schedule_183_e1502);
        (noise_metadata_schedule_183_e1503,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_183_e1505;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_184_e1514,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_184_e1512: f64 = (noise_variable_169 + noise_variable_171);
        (noise_metadata_schedule_184_e1512,)
    } else {
        (noise_variable_186,)
    }
};
            noise_variable_186 = noise_metadata_schedule_184_e1514;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_185_e1533,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_185_e1521: f64 = (noise_variable_186 * noise_variable_186);
        let noise_metadata_schedule_185_e1524: f64 = (0.5 * noise_variable_171);
        let noise_metadata_schedule_185_e1526: f64 = (noise_metadata_schedule_185_e1524 * noise_variable_171);
        let noise_metadata_schedule_185_e1528: f64 = (noise_metadata_schedule_185_e1526 - noise_variable_169);
        let noise_metadata_schedule_185_e1530: f64 = (noise_metadata_schedule_185_e1528 * noise_variable_173);
        let noise_metadata_schedule_185_e1531: f64 = (noise_metadata_schedule_185_e1521 + noise_metadata_schedule_185_e1530);
        (noise_metadata_schedule_185_e1531,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_185_e1533;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_186_e1566,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_186_e1541: f64 = (noise_variable_169 * noise_variable_186);
        let noise_metadata_schedule_186_e1543: f64 = (noise_metadata_schedule_186_e1541 * noise_variable_173);
        let noise_metadata_schedule_186_e1547: f64 = (noise_variable_186 * noise_variable_173);
        let noise_metadata_schedule_186_e1549: f64 = (noise_metadata_schedule_186_e1547 * noise_variable_173);
        let noise_metadata_schedule_186_e1551: f64 = (noise_metadata_schedule_186_e1549 / noise_variable_187);
        let noise_metadata_schedule_186_e1553: f64 = (noise_metadata_schedule_186_e1551 * noise_variable_171);
        let noise_metadata_schedule_186_e1556: f64 = (noise_variable_171 * noise_variable_171);
        let noise_metadata_schedule_186_e1558: f64 = (noise_metadata_schedule_186_e1556 * 0.3333333333333333);
        let noise_metadata_schedule_186_e1560: f64 = (noise_metadata_schedule_186_e1558 - noise_variable_169);
        let noise_metadata_schedule_186_e1561: f64 = (noise_metadata_schedule_186_e1553 * noise_metadata_schedule_186_e1560);
        let noise_metadata_schedule_186_e1562: f64 = (noise_variable_187 + noise_metadata_schedule_186_e1561);
        let noise_metadata_schedule_186_e1563: f64 = (noise_metadata_schedule_186_e1543 / noise_metadata_schedule_186_e1562);
        let noise_metadata_schedule_186_e1564: f64 = (noise_variable_174 + noise_metadata_schedule_186_e1563);
        (noise_metadata_schedule_186_e1564,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_186_e1566;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_187_e1569: f64 = if noise_variable_168 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_188 = noise_metadata_schedule_187_e1569;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_188_e1579,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) && (noise_variable_188 != 0.0)) {
        let noise_metadata_schedule_188_e1577: f64 = (noise_variable_168).exp();
        (noise_metadata_schedule_188_e1577,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_188_e1579;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_189_e1611,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) && (noise_variable_188 == 0.0)) {
        let noise_metadata_schedule_189_e1591: f64 = (noise_variable_168 - 230.25850929940458);
        let noise_metadata_schedule_189_e1596: f64 = (noise_variable_168 - 230.25850929940458);
        let noise_metadata_schedule_189_e1597: f64 = (0.5 * noise_metadata_schedule_189_e1596);
        let noise_metadata_schedule_189_e1601: f64 = (noise_variable_168 - 230.25850929940458);
        let noise_metadata_schedule_189_e1603: f64 = (noise_metadata_schedule_189_e1601 * 0.3333333333333333);
        let noise_metadata_schedule_189_e1604: f64 = (1.0 + noise_metadata_schedule_189_e1603);
        let noise_metadata_schedule_189_e1605: f64 = (noise_metadata_schedule_189_e1597 * noise_metadata_schedule_189_e1604);
        let noise_metadata_schedule_189_e1606: f64 = (1.0 + noise_metadata_schedule_189_e1605);
        let noise_metadata_schedule_189_e1607: f64 = (noise_metadata_schedule_189_e1591 * noise_metadata_schedule_189_e1606);
        let noise_metadata_schedule_189_e1608: f64 = (1.0 + noise_metadata_schedule_189_e1607);
        let noise_metadata_schedule_189_e1609: f64 = (1e100 * noise_metadata_schedule_189_e1608);
        (noise_metadata_schedule_189_e1609,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_189_e1611;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_190_e1620,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_190_e1618: f64 = (1.0 / noise_variable_175);
        (noise_metadata_schedule_190_e1618,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_190_e1620;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_191_e1633,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_191_e1629: f64 = (noise_variable_168 * noise_variable_168);
        let noise_metadata_schedule_191_e1630: f64 = (2.0 + noise_metadata_schedule_191_e1629);
        let noise_metadata_schedule_191_e1631: f64 = (1.0 / noise_metadata_schedule_191_e1630);
        (noise_metadata_schedule_191_e1631,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_191_e1633;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_192_e1642,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_192_e1640: f64 = (noise_variable_166 - noise_variable_168);
        (noise_metadata_schedule_192_e1640,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_192_e1642;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_193_e1651,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_193_e1649: f64 = (noise_variable_52 * noise_variable_176);
        (noise_metadata_schedule_193_e1649,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_193_e1651;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_194_e1670,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_194_e1658: f64 = (2.0 * noise_variable_164);
        let noise_metadata_schedule_194_e1662: f64 = (noise_variable_175 - 1.0);
        let noise_metadata_schedule_194_e1664: f64 = (noise_metadata_schedule_194_e1662 - noise_variable_165);
        let noise_metadata_schedule_194_e1666: f64 = (noise_metadata_schedule_194_e1664 + noise_variable_52);
        let noise_metadata_schedule_194_e1667: f64 = (noise_variable_36 * noise_metadata_schedule_194_e1666);
        let noise_metadata_schedule_194_e1668: f64 = (noise_metadata_schedule_194_e1658 + noise_metadata_schedule_194_e1667);
        (noise_metadata_schedule_194_e1668,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_194_e1670;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_195_e1695,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_195_e1677: f64 = (noise_variable_164 * noise_variable_164);
        let noise_metadata_schedule_195_e1681: f64 = (noise_variable_175 - noise_variable_168);
        let noise_metadata_schedule_195_e1683: f64 = (noise_metadata_schedule_195_e1681 - 1.0);
        let noise_metadata_schedule_195_e1685: f64 = (noise_metadata_schedule_195_e1683 + noise_variable_165);
        let noise_metadata_schedule_195_e1689: f64 = (noise_variable_168 - 1.0);
        let noise_metadata_schedule_195_e1690: f64 = (noise_variable_52 * noise_metadata_schedule_195_e1689);
        let noise_metadata_schedule_195_e1691: f64 = (noise_metadata_schedule_195_e1685 + noise_metadata_schedule_195_e1690);
        let noise_metadata_schedule_195_e1692: f64 = (noise_variable_36 * noise_metadata_schedule_195_e1691);
        let noise_metadata_schedule_195_e1693: f64 = (noise_metadata_schedule_195_e1677 - noise_metadata_schedule_195_e1692);
        (noise_metadata_schedule_195_e1693,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_195_e1695;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_196_e1708,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_196_e1704: f64 = (noise_variable_175 + noise_variable_165);
        let noise_metadata_schedule_196_e1705: f64 = (noise_variable_36 * noise_metadata_schedule_196_e1704);
        let noise_metadata_schedule_196_e1706: f64 = (2.0 - noise_metadata_schedule_196_e1705);
        (noise_metadata_schedule_196_e1706,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_196_e1708;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_197_e1723,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_197_e1715: f64 = (noise_variable_177 * noise_variable_177);
        let noise_metadata_schedule_197_e1718: f64 = (2.0 * noise_variable_178);
        let noise_metadata_schedule_197_e1720: f64 = (noise_metadata_schedule_197_e1718 * noise_variable_164);
        let noise_metadata_schedule_197_e1721: f64 = (noise_metadata_schedule_197_e1715 - noise_metadata_schedule_197_e1720);
        (noise_metadata_schedule_197_e1721,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_197_e1723;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_198_e1740,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 != 0.0)) {
        let noise_metadata_schedule_198_e1729: f64 = (-noise_variable_168);
        let noise_metadata_schedule_198_e1732: f64 = (2.0 * noise_variable_178);
        let noise_metadata_schedule_198_e1735: f64 = (noise_variable_164).sqrt();
        let noise_metadata_schedule_198_e1736: f64 = (noise_variable_177 + noise_metadata_schedule_198_e1735);
        let noise_metadata_schedule_198_e1737: f64 = (noise_metadata_schedule_198_e1732 / noise_metadata_schedule_198_e1736);
        let noise_metadata_schedule_198_e1738: f64 = (noise_metadata_schedule_198_e1729 - noise_metadata_schedule_198_e1737);
        (noise_metadata_schedule_198_e1738,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_198_e1740;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_199_e1754,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_199_e1750: f64 = (noise_variable_34 * 0.7324648775608221);
        let noise_metadata_schedule_199_e1751: f64 = (1.25 + noise_metadata_schedule_199_e1750);
        let noise_metadata_schedule_199_e1752: f64 = (1.0 / noise_metadata_schedule_199_e1751);
        (noise_metadata_schedule_199_e1752,)
    } else {
        (noise_variable_163,)
    }
};
            noise_variable_163 = noise_metadata_schedule_199_e1754;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_200_e1770,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_200_e1762: f64 = (noise_variable_43 * 1.25);
        let noise_metadata_schedule_200_e1764: f64 = (noise_metadata_schedule_200_e1762 * noise_variable_163);
        let noise_metadata_schedule_200_e1766: f64 = (noise_metadata_schedule_200_e1764 - 1.0);
        let noise_metadata_schedule_200_e1768: f64 = (noise_metadata_schedule_200_e1766 * noise_variable_163);
        (noise_metadata_schedule_200_e1768,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_200_e1770;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_201_e1786,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_201_e1778: f64 = (noise_variable_78 * noise_variable_44);
        let noise_metadata_schedule_201_e1782: f64 = (noise_variable_179 * noise_variable_78);
        let noise_metadata_schedule_201_e1783: f64 = (1.0 + noise_metadata_schedule_201_e1782);
        let noise_metadata_schedule_201_e1784: f64 = (noise_metadata_schedule_201_e1778 * noise_metadata_schedule_201_e1783);
        (noise_metadata_schedule_201_e1784,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_201_e1786;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_202_e1788: f64 = (-noise_variable_182);
            let noise_metadata_schedule_202_e1790: f64 = (-230.25850929940458);
            let noise_metadata_schedule_202_e1791: f64 = if noise_metadata_schedule_202_e1788 > noise_metadata_schedule_202_e1790 { 1.0 } else { 0.0 };
            noise_variable_189 = noise_metadata_schedule_202_e1791;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_203_e1803,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_189 != 0.0)) {
        let noise_metadata_schedule_203_e1800: f64 = (-noise_variable_182);
        let noise_metadata_schedule_203_e1801: f64 = (noise_metadata_schedule_203_e1800).exp();
        (noise_metadata_schedule_203_e1801,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_203_e1803;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_204_e1842,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_189 == 0.0)) {
        let noise_metadata_schedule_204_e1815: f64 = (-230.25850929940458);
        let noise_metadata_schedule_204_e1817: f64 = (-noise_variable_182);
        let noise_metadata_schedule_204_e1818: f64 = (noise_metadata_schedule_204_e1815 - noise_metadata_schedule_204_e1817);
        let noise_metadata_schedule_204_e1822: f64 = (-230.25850929940458);
        let noise_metadata_schedule_204_e1824: f64 = (-noise_variable_182);
        let noise_metadata_schedule_204_e1825: f64 = (noise_metadata_schedule_204_e1822 - noise_metadata_schedule_204_e1824);
        let noise_metadata_schedule_204_e1826: f64 = (0.5 * noise_metadata_schedule_204_e1825);
        let noise_metadata_schedule_204_e1829: f64 = (-230.25850929940458);
        let noise_metadata_schedule_204_e1831: f64 = (-noise_variable_182);
        let noise_metadata_schedule_204_e1832: f64 = (noise_metadata_schedule_204_e1829 - noise_metadata_schedule_204_e1831);
        let noise_metadata_schedule_204_e1834: f64 = (noise_metadata_schedule_204_e1832 * 0.3333333333333333);
        let noise_metadata_schedule_204_e1835: f64 = (1.0 + noise_metadata_schedule_204_e1834);
        let noise_metadata_schedule_204_e1836: f64 = (noise_metadata_schedule_204_e1826 * noise_metadata_schedule_204_e1835);
        let noise_metadata_schedule_204_e1837: f64 = (1.0 + noise_metadata_schedule_204_e1836);
        let noise_metadata_schedule_204_e1838: f64 = (noise_metadata_schedule_204_e1818 * noise_metadata_schedule_204_e1837);
        let noise_metadata_schedule_204_e1839: f64 = (1.0 + noise_metadata_schedule_204_e1838);
        let noise_metadata_schedule_204_e1840: f64 = (1e-100 / noise_metadata_schedule_204_e1839);
        (noise_metadata_schedule_204_e1840,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_204_e1842;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_205_e1852,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_205_e1850: f64 = (1.0 - noise_variable_164);
        (noise_metadata_schedule_205_e1850,)
    } else {
        (noise_variable_181,)
    }
};
            noise_variable_181 = noise_metadata_schedule_205_e1852;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_206_e1875,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_206_e1861: f64 = (noise_variable_36 * 0.5);
        let noise_metadata_schedule_206_e1862: f64 = (noise_variable_78 + noise_metadata_schedule_206_e1861);
        let noise_metadata_schedule_206_e1867: f64 = (noise_variable_36 * 0.25);
        let noise_metadata_schedule_206_e1868: f64 = (noise_variable_78 + noise_metadata_schedule_206_e1867);
        let noise_metadata_schedule_206_e1870: f64 = (noise_metadata_schedule_206_e1868 - noise_variable_181);
        let noise_metadata_schedule_206_e1871: f64 = (noise_metadata_schedule_206_e1870).sqrt();
        let noise_metadata_schedule_206_e1872: f64 = (noise_variable_34 * noise_metadata_schedule_206_e1871);
        let noise_metadata_schedule_206_e1873: f64 = (noise_metadata_schedule_206_e1862 - noise_metadata_schedule_206_e1872);
        (noise_metadata_schedule_206_e1873,)
    } else {
        (noise_variable_180,)
    }
};
            noise_variable_180 = noise_metadata_schedule_206_e1875;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_207_e1885,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_207_e1883: f64 = (noise_variable_50 + 3.0);
        (noise_metadata_schedule_207_e1883,)
    } else {
        (noise_variable_172,)
    }
};
            noise_variable_172 = noise_metadata_schedule_207_e1885;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_208_e1965,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_208_e1893: f64 = (noise_variable_172 - noise_variable_180);
        let (noise_metadata_schedule_208_e1952,) = {
            if (noise_metadata_schedule_208_e1893 > 1e-16) {
                let noise_metadata_schedule_208_e1900: f64 = (noise_variable_172 - noise_variable_180);
                let noise_metadata_schedule_208_e1903: f64 = (noise_variable_172 - noise_variable_180);
                let noise_metadata_schedule_208_e1906: f64 = (noise_variable_172 - noise_variable_180);
                let noise_metadata_schedule_208_e1907: f64 = (noise_metadata_schedule_208_e1903 * noise_metadata_schedule_208_e1906);
                let noise_metadata_schedule_208_e1909: f64 = (noise_metadata_schedule_208_e1907 + 5.0);
                let noise_metadata_schedule_208_e1910: f64 = (noise_metadata_schedule_208_e1909).sqrt();
                let noise_metadata_schedule_208_e1911: f64 = (noise_metadata_schedule_208_e1900 + noise_metadata_schedule_208_e1910);
                let noise_metadata_schedule_208_e1912: f64 = (0.5 * noise_metadata_schedule_208_e1911);
                let noise_metadata_schedule_208_e1913: f64 = (noise_variable_172 - noise_metadata_schedule_208_e1912);
                (noise_metadata_schedule_208_e1913,)
            } else {
                let noise_metadata_schedule_208_e1916: f64 = (noise_variable_180 - noise_variable_172);
                let (noise_metadata_schedule_208_e1951,) = {
                    if (noise_metadata_schedule_208_e1916 > 1e-16) {
                        let noise_metadata_schedule_208_e1922: f64 = (0.5 * 5.0);
                        let noise_metadata_schedule_208_e1925: f64 = (noise_variable_180 - noise_variable_172);
                        let noise_metadata_schedule_208_e1928: f64 = (noise_variable_180 - noise_variable_172);
                        let noise_metadata_schedule_208_e1931: f64 = (noise_variable_180 - noise_variable_172);
                        let noise_metadata_schedule_208_e1932: f64 = (noise_metadata_schedule_208_e1928 * noise_metadata_schedule_208_e1931);
                        let noise_metadata_schedule_208_e1934: f64 = (noise_metadata_schedule_208_e1932 + 5.0);
                        let noise_metadata_schedule_208_e1935: f64 = (noise_metadata_schedule_208_e1934).sqrt();
                        let noise_metadata_schedule_208_e1936: f64 = (noise_metadata_schedule_208_e1925 + noise_metadata_schedule_208_e1935);
                        let noise_metadata_schedule_208_e1937: f64 = (noise_metadata_schedule_208_e1922 / noise_metadata_schedule_208_e1936);
                        let noise_metadata_schedule_208_e1938: f64 = (noise_variable_172 - noise_metadata_schedule_208_e1937);
                        (noise_metadata_schedule_208_e1938,)
                    } else {
                        let noise_metadata_schedule_208_e1943: f64 = (noise_variable_172 - noise_variable_180);
                        let noise_metadata_schedule_208_e1946: f64 = (1e-32 + 5.0);
                        let noise_metadata_schedule_208_e1947: f64 = (noise_metadata_schedule_208_e1946).sqrt();
                        let noise_metadata_schedule_208_e1948: f64 = (noise_metadata_schedule_208_e1943 + noise_metadata_schedule_208_e1947);
                        let noise_metadata_schedule_208_e1949: f64 = (0.5 * noise_metadata_schedule_208_e1948);
                        let noise_metadata_schedule_208_e1950: f64 = (noise_variable_172 - noise_metadata_schedule_208_e1949);
                        (noise_metadata_schedule_208_e1950,)
                    }
                };
                (noise_metadata_schedule_208_e1951,)
            }
        };
        let noise_metadata_schedule_208_e1957: f64 = (noise_variable_172 * noise_variable_172);
        let noise_metadata_schedule_208_e1959: f64 = (noise_metadata_schedule_208_e1957 + 5.0);
        let noise_metadata_schedule_208_e1960: f64 = (noise_metadata_schedule_208_e1959).sqrt();
        let noise_metadata_schedule_208_e1961: f64 = (noise_variable_172 - noise_metadata_schedule_208_e1960);
        let noise_metadata_schedule_208_e1962: f64 = (0.5 * noise_metadata_schedule_208_e1961);
        let noise_metadata_schedule_208_e1963: f64 = (noise_metadata_schedule_208_e1952 - noise_metadata_schedule_208_e1962);
        (noise_metadata_schedule_208_e1963,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_208_e1965;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_209_e1975,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_209_e1973: f64 = (noise_variable_78 - noise_variable_174);
        (noise_metadata_schedule_209_e1973,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_209_e1975;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_210_e1985,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_210_e1982: f64 = (-noise_variable_174);
        let noise_metadata_schedule_210_e1983: f64 = (noise_metadata_schedule_210_e1982).exp();
        (noise_metadata_schedule_210_e1983,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_210_e1985;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_211_e2011,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_211_e1994: f64 = (noise_variable_164 * noise_variable_164);
        let noise_metadata_schedule_211_e1998: f64 = (noise_variable_165 + noise_variable_174);
        let noise_metadata_schedule_211_e2000: f64 = (noise_metadata_schedule_211_e1998 - 1.0);
        let noise_metadata_schedule_211_e2004: f64 = (noise_variable_174 + 1.0);
        let noise_metadata_schedule_211_e2005: f64 = (noise_variable_52 * noise_metadata_schedule_211_e2004);
        let noise_metadata_schedule_211_e2006: f64 = (noise_metadata_schedule_211_e2000 - noise_metadata_schedule_211_e2005);
        let noise_metadata_schedule_211_e2007: f64 = (noise_variable_36 * noise_metadata_schedule_211_e2006);
        let noise_metadata_schedule_211_e2008: f64 = (noise_metadata_schedule_211_e1994 - noise_metadata_schedule_211_e2007);
        let noise_metadata_schedule_211_e2009: f64 = (1e-40_f64).max(noise_metadata_schedule_211_e2008);
        (noise_metadata_schedule_211_e2009,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_211_e2011;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_212_e2025,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_212_e2020: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_212_e2022: f64 = (noise_metadata_schedule_212_e2020 * noise_variable_165);
        let noise_metadata_schedule_212_e2023: f64 = (1.0 - noise_metadata_schedule_212_e2022);
        (noise_metadata_schedule_212_e2023,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_212_e2025;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_213_e2043,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_213_e2033: f64 = (2.0 * noise_variable_164);
        let noise_metadata_schedule_213_e2037: f64 = (1.0 - noise_variable_165);
        let noise_metadata_schedule_213_e2039: f64 = (noise_metadata_schedule_213_e2037 - noise_variable_52);
        let noise_metadata_schedule_213_e2040: f64 = (noise_variable_36 * noise_metadata_schedule_213_e2039);
        let noise_metadata_schedule_213_e2041: f64 = (noise_metadata_schedule_213_e2033 + noise_metadata_schedule_213_e2040);
        (noise_metadata_schedule_213_e2041,)
    } else {
        (noise_variable_171,)
    }
};
            noise_variable_171 = noise_metadata_schedule_213_e2043;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_214_e2058,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_214_e2051: f64 = (noise_variable_50 - noise_variable_174);
        let noise_metadata_schedule_214_e2054: f64 = (noise_variable_169 / noise_variable_36);
        let noise_metadata_schedule_214_e2055: f64 = (noise_metadata_schedule_214_e2054).ln();
        let noise_metadata_schedule_214_e2056: f64 = (noise_metadata_schedule_214_e2051 + noise_metadata_schedule_214_e2055);
        (noise_metadata_schedule_214_e2056,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_214_e2058;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_215_e2068,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_215_e2066: f64 = (noise_variable_169 + noise_variable_171);
        (noise_metadata_schedule_215_e2066,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_215_e2068;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_216_e2070: f64 = (noise_variable_173).abs();
            let noise_metadata_schedule_216_e2072: f64 = if noise_metadata_schedule_216_e2070 < 1e-120 { 1.0 } else { 0.0 };
            noise_variable_192 = noise_metadata_schedule_216_e2072;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_217_e2082,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_192 != 0.0)) {
        (noise_variable_174,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_217_e2082;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_218_e2107,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_192 == 0.0)) {
        let noise_metadata_schedule_218_e2093: f64 = (noise_variable_190 * noise_variable_190);
        let noise_metadata_schedule_218_e2096: f64 = (0.5 * noise_variable_171);
        let noise_metadata_schedule_218_e2098: f64 = (noise_metadata_schedule_218_e2096 * noise_variable_171);
        let noise_metadata_schedule_218_e2101: f64 = (noise_variable_169 * noise_variable_170);
        let noise_metadata_schedule_218_e2102: f64 = (noise_metadata_schedule_218_e2098 - noise_metadata_schedule_218_e2101);
        let noise_metadata_schedule_218_e2104: f64 = (noise_metadata_schedule_218_e2102 * noise_variable_173);
        let noise_metadata_schedule_218_e2105: f64 = (noise_metadata_schedule_218_e2093 + noise_metadata_schedule_218_e2104);
        (noise_metadata_schedule_218_e2105,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_218_e2107;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_219_e2146,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_192 == 0.0)) {
        let noise_metadata_schedule_219_e2119: f64 = (noise_variable_169 * noise_variable_190);
        let noise_metadata_schedule_219_e2121: f64 = (noise_metadata_schedule_219_e2119 * noise_variable_173);
        let noise_metadata_schedule_219_e2125: f64 = (noise_variable_190 * noise_variable_173);
        let noise_metadata_schedule_219_e2127: f64 = (noise_metadata_schedule_219_e2125 * noise_variable_173);
        let noise_metadata_schedule_219_e2129: f64 = (noise_metadata_schedule_219_e2127 / noise_variable_191);
        let noise_metadata_schedule_219_e2131: f64 = (noise_metadata_schedule_219_e2129 * noise_variable_171);
        let noise_metadata_schedule_219_e2134: f64 = (noise_variable_171 * noise_variable_171);
        let noise_metadata_schedule_219_e2136: f64 = (noise_metadata_schedule_219_e2134 * 0.3333333333333333);
        let noise_metadata_schedule_219_e2139: f64 = (noise_variable_169 * noise_variable_170);
        let noise_metadata_schedule_219_e2140: f64 = (noise_metadata_schedule_219_e2136 - noise_metadata_schedule_219_e2139);
        let noise_metadata_schedule_219_e2141: f64 = (noise_metadata_schedule_219_e2131 * noise_metadata_schedule_219_e2140);
        let noise_metadata_schedule_219_e2142: f64 = (noise_variable_191 + noise_metadata_schedule_219_e2141);
        let noise_metadata_schedule_219_e2143: f64 = (noise_metadata_schedule_219_e2121 / noise_metadata_schedule_219_e2142);
        let noise_metadata_schedule_219_e2144: f64 = (noise_variable_174 + noise_metadata_schedule_219_e2143);
        (noise_metadata_schedule_219_e2144,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_219_e2146;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_220_e2149: f64 = if noise_variable_183 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_193 = noise_metadata_schedule_220_e2149;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_221_e2160,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_193 != 0.0)) {
        let noise_metadata_schedule_221_e2158: f64 = (noise_variable_183).exp();
        (noise_metadata_schedule_221_e2158,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_221_e2160;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_222_e2172,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_193 != 0.0)) {
        let noise_metadata_schedule_222_e2170: f64 = (1.0 / noise_variable_175);
        (noise_metadata_schedule_222_e2170,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_222_e2172;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_223_e2184,) = {
    if (((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_193 != 0.0)) {
        let noise_metadata_schedule_223_e2182: f64 = (noise_variable_52 * noise_variable_175);
        (noise_metadata_schedule_223_e2182,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_223_e2184;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_224_e2188: f64 = (noise_variable_50 - 230.25850929940458);
            let noise_metadata_schedule_224_e2189: f64 = if noise_variable_183 > noise_metadata_schedule_224_e2188 { 1.0 } else { 0.0 };
            noise_variable_194 = noise_metadata_schedule_224_e2189;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_225_e2205,) = {
    if ((((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_193 == 0.0)) && (noise_variable_194 != 0.0)) {
        let noise_metadata_schedule_225_e2202: f64 = (noise_variable_183 - noise_variable_50);
        let noise_metadata_schedule_225_e2203: f64 = (noise_metadata_schedule_225_e2202).exp();
        (noise_metadata_schedule_225_e2203,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_225_e2205;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_226_e2220,) = {
    if ((((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_193 == 0.0)) && (noise_variable_194 != 0.0)) {
        let noise_metadata_schedule_226_e2218: f64 = (noise_variable_52 / noise_variable_175);
        (noise_metadata_schedule_226_e2218,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_226_e2220;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_227_e2262,) = {
    if ((((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_193 == 0.0)) && (noise_variable_194 == 0.0)) {
        let noise_metadata_schedule_227_e2236: f64 = (noise_variable_50 - noise_variable_183);
        let noise_metadata_schedule_227_e2238: f64 = (noise_metadata_schedule_227_e2236 - 230.25850929940458);
        let noise_metadata_schedule_227_e2243: f64 = (noise_variable_50 - noise_variable_183);
        let noise_metadata_schedule_227_e2245: f64 = (noise_metadata_schedule_227_e2243 - 230.25850929940458);
        let noise_metadata_schedule_227_e2246: f64 = (0.5 * noise_metadata_schedule_227_e2245);
        let noise_metadata_schedule_227_e2250: f64 = (noise_variable_50 - noise_variable_183);
        let noise_metadata_schedule_227_e2252: f64 = (noise_metadata_schedule_227_e2250 - 230.25850929940458);
        let noise_metadata_schedule_227_e2254: f64 = (noise_metadata_schedule_227_e2252 * 0.3333333333333333);
        let noise_metadata_schedule_227_e2255: f64 = (1.0 + noise_metadata_schedule_227_e2254);
        let noise_metadata_schedule_227_e2256: f64 = (noise_metadata_schedule_227_e2246 * noise_metadata_schedule_227_e2255);
        let noise_metadata_schedule_227_e2257: f64 = (1.0 + noise_metadata_schedule_227_e2256);
        let noise_metadata_schedule_227_e2258: f64 = (noise_metadata_schedule_227_e2238 * noise_metadata_schedule_227_e2257);
        let noise_metadata_schedule_227_e2259: f64 = (1.0 + noise_metadata_schedule_227_e2258);
        let noise_metadata_schedule_227_e2260: f64 = (1e-100 / noise_metadata_schedule_227_e2259);
        (noise_metadata_schedule_227_e2260,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_227_e2262;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_228_e2298,) = {
    if ((((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) && (noise_variable_193 == 0.0)) && (noise_variable_194 == 0.0)) {
        let noise_metadata_schedule_228_e2278: f64 = (noise_variable_183 - 230.25850929940458);
        let noise_metadata_schedule_228_e2283: f64 = (noise_variable_183 - 230.25850929940458);
        let noise_metadata_schedule_228_e2284: f64 = (0.5 * noise_metadata_schedule_228_e2283);
        let noise_metadata_schedule_228_e2288: f64 = (noise_variable_183 - 230.25850929940458);
        let noise_metadata_schedule_228_e2290: f64 = (noise_metadata_schedule_228_e2288 * 0.3333333333333333);
        let noise_metadata_schedule_228_e2291: f64 = (1.0 + noise_metadata_schedule_228_e2290);
        let noise_metadata_schedule_228_e2292: f64 = (noise_metadata_schedule_228_e2284 * noise_metadata_schedule_228_e2291);
        let noise_metadata_schedule_228_e2293: f64 = (1.0 + noise_metadata_schedule_228_e2292);
        let noise_metadata_schedule_228_e2294: f64 = (noise_metadata_schedule_228_e2278 * noise_metadata_schedule_228_e2293);
        let noise_metadata_schedule_228_e2295: f64 = (1.0 + noise_metadata_schedule_228_e2294);
        let noise_metadata_schedule_228_e2296: f64 = (1e-100 / noise_metadata_schedule_228_e2295);
        (noise_metadata_schedule_228_e2296,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_228_e2298;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_229_e2312,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_229_e2308: f64 = (noise_variable_183 * noise_variable_183);
        let noise_metadata_schedule_229_e2309: f64 = (2.0 + noise_metadata_schedule_229_e2308);
        let noise_metadata_schedule_229_e2310: f64 = (1.0 / noise_metadata_schedule_229_e2309);
        (noise_metadata_schedule_229_e2310,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_229_e2312;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_230_e2322,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_230_e2320: f64 = (noise_variable_78 - noise_variable_183);
        (noise_metadata_schedule_230_e2320,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_230_e2322;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_231_e2342,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_231_e2330: f64 = (2.0 * noise_variable_164);
        let noise_metadata_schedule_231_e2334: f64 = (1.0 - noise_variable_176);
        let noise_metadata_schedule_231_e2336: f64 = (noise_metadata_schedule_231_e2334 + noise_variable_175);
        let noise_metadata_schedule_231_e2338: f64 = (noise_metadata_schedule_231_e2336 - noise_variable_52);
        let noise_metadata_schedule_231_e2339: f64 = (noise_variable_36 * noise_metadata_schedule_231_e2338);
        let noise_metadata_schedule_231_e2340: f64 = (noise_metadata_schedule_231_e2330 + noise_metadata_schedule_231_e2339);
        (noise_metadata_schedule_231_e2340,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_231_e2342;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_232_e2368,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_232_e2350: f64 = (noise_variable_164 * noise_variable_164);
        let noise_metadata_schedule_232_e2354: f64 = (noise_variable_176 + noise_variable_183);
        let noise_metadata_schedule_232_e2356: f64 = (noise_metadata_schedule_232_e2354 - 1.0);
        let noise_metadata_schedule_232_e2358: f64 = (noise_metadata_schedule_232_e2356 + noise_variable_175);
        let noise_metadata_schedule_232_e2362: f64 = (noise_variable_183 + 1.0);
        let noise_metadata_schedule_232_e2363: f64 = (noise_variable_52 * noise_metadata_schedule_232_e2362);
        let noise_metadata_schedule_232_e2364: f64 = (noise_metadata_schedule_232_e2358 - noise_metadata_schedule_232_e2363);
        let noise_metadata_schedule_232_e2365: f64 = (noise_variable_36 * noise_metadata_schedule_232_e2364);
        let noise_metadata_schedule_232_e2366: f64 = (noise_metadata_schedule_232_e2350 - noise_metadata_schedule_232_e2365);
        (noise_metadata_schedule_232_e2366,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_232_e2368;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_233_e2382,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_233_e2378: f64 = (noise_variable_176 + noise_variable_175);
        let noise_metadata_schedule_233_e2379: f64 = (noise_variable_36 * noise_metadata_schedule_233_e2378);
        let noise_metadata_schedule_233_e2380: f64 = (2.0 - noise_metadata_schedule_233_e2379);
        (noise_metadata_schedule_233_e2380,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_233_e2382;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_234_e2398,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_234_e2390: f64 = (noise_variable_177 * noise_variable_177);
        let noise_metadata_schedule_234_e2393: f64 = (2.0 * noise_variable_178);
        let noise_metadata_schedule_234_e2395: f64 = (noise_metadata_schedule_234_e2393 * noise_variable_164);
        let noise_metadata_schedule_234_e2396: f64 = (noise_metadata_schedule_234_e2390 - noise_metadata_schedule_234_e2395);
        (noise_metadata_schedule_234_e2396,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_234_e2398;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_235_e2415,) = {
    if ((noise_variable_184 == 0.0) && (noise_variable_185 == 0.0)) {
        let noise_metadata_schedule_235_e2407: f64 = (2.0 * noise_variable_178);
        let noise_metadata_schedule_235_e2410: f64 = (noise_variable_164).sqrt();
        let noise_metadata_schedule_235_e2411: f64 = (noise_variable_177 + noise_metadata_schedule_235_e2410);
        let noise_metadata_schedule_235_e2412: f64 = (noise_metadata_schedule_235_e2407 / noise_metadata_schedule_235_e2411);
        let noise_metadata_schedule_235_e2413: f64 = (noise_variable_183 + noise_metadata_schedule_235_e2412);
        (noise_metadata_schedule_235_e2413,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_235_e2415;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_236_e2418: f64 = if params.p29 < 1e27 { 1.0 } else { 0.0 };
            noise_variable_195 = noise_metadata_schedule_236_e2418;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_237_e2433,) = {
    if (noise_variable_195 != 0.0) {
        let noise_metadata_schedule_237_e2421: f64 = (-params.p17);
        let noise_metadata_schedule_237_e2423: f64 = (noise_metadata_schedule_237_e2421 * params.p18);
        let noise_metadata_schedule_237_e2427: f64 = (noise_variable_79 * noise_variable_25);
        let noise_metadata_schedule_237_e2428: f64 = (noise_variable_77 - noise_metadata_schedule_237_e2427);
        let noise_metadata_schedule_237_e2429: f64 = (noise_metadata_schedule_237_e2423 * noise_metadata_schedule_237_e2428);
        let noise_metadata_schedule_237_e2431: f64 = (noise_metadata_schedule_237_e2429 * noise_variable_26);
        (noise_metadata_schedule_237_e2431,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_237_e2433;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_238_e2435: f64 = (noise_variable_80).abs();
            let noise_metadata_schedule_238_e2437: f64 = if noise_metadata_schedule_238_e2435 <= noise_variable_41 { 1.0 } else { 0.0 };
            noise_variable_217 = noise_metadata_schedule_238_e2437;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_239_e2449,) = {
    if ((noise_variable_195 != 0.0) && (noise_variable_217 != 0.0)) {
        let noise_metadata_schedule_239_e2443: f64 = (noise_variable_46 * noise_variable_46);
        let noise_metadata_schedule_239_e2445: f64 = (noise_metadata_schedule_239_e2443 * 0.1666666666666667);
        let noise_metadata_schedule_239_e2447: f64 = (noise_metadata_schedule_239_e2445 * 0.7071067811865475);
        (noise_metadata_schedule_239_e2447,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_239_e2449;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_240_e2469,) = {
    if ((noise_variable_195 != 0.0) && (noise_variable_217 != 0.0)) {
        let noise_metadata_schedule_240_e2455: f64 = (noise_variable_80 * noise_variable_46);
        let noise_metadata_schedule_240_e2460: f64 = (1.0 - noise_variable_53);
        let noise_metadata_schedule_240_e2461: f64 = (noise_variable_80 * noise_metadata_schedule_240_e2460);
        let noise_metadata_schedule_240_e2463: f64 = (noise_metadata_schedule_240_e2461 * noise_variable_35);
        let noise_metadata_schedule_240_e2465: f64 = (noise_metadata_schedule_240_e2463 * noise_variable_198);
        let noise_metadata_schedule_240_e2466: f64 = (1.0 + noise_metadata_schedule_240_e2465);
        let noise_metadata_schedule_240_e2467: f64 = (noise_metadata_schedule_240_e2455 * noise_metadata_schedule_240_e2466);
        (noise_metadata_schedule_240_e2467,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_240_e2469;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_241_e2472: f64 = (-noise_variable_41);
            let noise_metadata_schedule_241_e2473: f64 = if noise_variable_80 < noise_metadata_schedule_241_e2472 { 1.0 } else { 0.0 };
            noise_variable_218 = noise_metadata_schedule_241_e2473;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_242_e2483,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_242_e2481: f64 = (-noise_variable_80);
        (noise_metadata_schedule_242_e2481,)
    } else {
        (noise_variable_199,)
    }
};
            noise_variable_199 = noise_metadata_schedule_242_e2483;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_243_e2496,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_243_e2492: f64 = (1.25 * noise_variable_199);
        let noise_metadata_schedule_243_e2494: f64 = (noise_metadata_schedule_243_e2492 * noise_variable_46);
        (noise_metadata_schedule_243_e2494,)
    } else {
        (noise_variable_200,)
    }
};
            noise_variable_200 = noise_metadata_schedule_243_e2496;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_244_e2520,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_244_e2506: f64 = (noise_variable_200 + 10.0);
        let noise_metadata_schedule_244_e2509: f64 = (noise_variable_200 - 6.0);
        let noise_metadata_schedule_244_e2512: f64 = (noise_variable_200 - 6.0);
        let noise_metadata_schedule_244_e2513: f64 = (noise_metadata_schedule_244_e2509 * noise_metadata_schedule_244_e2512);
        let noise_metadata_schedule_244_e2515: f64 = (noise_metadata_schedule_244_e2513 + 64.0);
        let noise_metadata_schedule_244_e2516: f64 = (noise_metadata_schedule_244_e2515).sqrt();
        let noise_metadata_schedule_244_e2517: f64 = (noise_metadata_schedule_244_e2506 - noise_metadata_schedule_244_e2516);
        let noise_metadata_schedule_244_e2518: f64 = (0.5 * noise_metadata_schedule_244_e2517);
        (noise_metadata_schedule_244_e2518,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_244_e2520;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_245_e2531,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_245_e2529: f64 = (noise_variable_199 - noise_variable_207);
        (noise_metadata_schedule_245_e2529,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_245_e2531;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_246_e2548,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_246_e2540: f64 = (noise_variable_197 * noise_variable_197);
        let noise_metadata_schedule_246_e2544: f64 = (noise_variable_207 + 1.0);
        let noise_metadata_schedule_246_e2545: f64 = (noise_variable_38 * noise_metadata_schedule_246_e2544);
        let noise_metadata_schedule_246_e2546: f64 = (noise_metadata_schedule_246_e2540 + noise_metadata_schedule_246_e2545);
        (noise_metadata_schedule_246_e2546,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_246_e2548;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_247_e2561,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_247_e2557: f64 = (2.0 * noise_variable_197);
        let noise_metadata_schedule_247_e2559: f64 = (noise_metadata_schedule_247_e2557 - noise_variable_38);
        (noise_metadata_schedule_247_e2559,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_247_e2561;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_248_e2576,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_248_e2569: f64 = (-noise_variable_207);
        let noise_metadata_schedule_248_e2572: f64 = (noise_variable_202 * noise_variable_39);
        let noise_metadata_schedule_248_e2573: f64 = (noise_metadata_schedule_248_e2572).ln();
        let noise_metadata_schedule_248_e2574: f64 = (noise_metadata_schedule_248_e2569 + noise_metadata_schedule_248_e2573);
        (noise_metadata_schedule_248_e2574,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_248_e2576;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_249_e2587,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_249_e2585: f64 = (noise_variable_202 + noise_variable_204);
        (noise_metadata_schedule_249_e2585,)
    } else {
        (noise_variable_219,)
    }
};
            noise_variable_219 = noise_metadata_schedule_249_e2587;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_250_e2608,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_250_e2596: f64 = (noise_variable_219 * noise_variable_219);
        let noise_metadata_schedule_250_e2599: f64 = (0.5 * noise_variable_204);
        let noise_metadata_schedule_250_e2601: f64 = (noise_metadata_schedule_250_e2599 * noise_variable_204);
        let noise_metadata_schedule_250_e2603: f64 = (noise_metadata_schedule_250_e2601 - noise_variable_202);
        let noise_metadata_schedule_250_e2605: f64 = (noise_metadata_schedule_250_e2603 * noise_variable_206);
        let noise_metadata_schedule_250_e2606: f64 = (noise_metadata_schedule_250_e2596 + noise_metadata_schedule_250_e2605);
        (noise_metadata_schedule_250_e2606,)
    } else {
        (noise_variable_220,)
    }
};
            noise_variable_220 = noise_metadata_schedule_250_e2608;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_251_e2643,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_251_e2618: f64 = (noise_variable_202 * noise_variable_219);
        let noise_metadata_schedule_251_e2620: f64 = (noise_metadata_schedule_251_e2618 * noise_variable_206);
        let noise_metadata_schedule_251_e2624: f64 = (noise_variable_219 * noise_variable_206);
        let noise_metadata_schedule_251_e2626: f64 = (noise_metadata_schedule_251_e2624 * noise_variable_206);
        let noise_metadata_schedule_251_e2628: f64 = (noise_metadata_schedule_251_e2626 / noise_variable_220);
        let noise_metadata_schedule_251_e2630: f64 = (noise_metadata_schedule_251_e2628 * noise_variable_204);
        let noise_metadata_schedule_251_e2633: f64 = (noise_variable_204 * noise_variable_204);
        let noise_metadata_schedule_251_e2635: f64 = (noise_metadata_schedule_251_e2633 * 0.3333333333333333);
        let noise_metadata_schedule_251_e2637: f64 = (noise_metadata_schedule_251_e2635 - noise_variable_202);
        let noise_metadata_schedule_251_e2638: f64 = (noise_metadata_schedule_251_e2630 * noise_metadata_schedule_251_e2637);
        let noise_metadata_schedule_251_e2639: f64 = (noise_variable_220 + noise_metadata_schedule_251_e2638);
        let noise_metadata_schedule_251_e2640: f64 = (noise_metadata_schedule_251_e2620 / noise_metadata_schedule_251_e2639);
        let noise_metadata_schedule_251_e2641: f64 = (noise_variable_207 + noise_metadata_schedule_251_e2640);
        (noise_metadata_schedule_251_e2641,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_251_e2643;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_252_e2646: f64 = if noise_variable_201 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_221 = noise_metadata_schedule_252_e2646;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_253_e2658,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) && (noise_variable_221 != 0.0)) {
        let noise_metadata_schedule_253_e2656: f64 = (noise_variable_201).exp();
        (noise_metadata_schedule_253_e2656,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_253_e2658;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_254_e2692,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) && (noise_variable_221 == 0.0)) {
        let noise_metadata_schedule_254_e2672: f64 = (noise_variable_201 - 230.25850929940458);
        let noise_metadata_schedule_254_e2677: f64 = (noise_variable_201 - 230.25850929940458);
        let noise_metadata_schedule_254_e2678: f64 = (0.5 * noise_metadata_schedule_254_e2677);
        let noise_metadata_schedule_254_e2682: f64 = (noise_variable_201 - 230.25850929940458);
        let noise_metadata_schedule_254_e2684: f64 = (noise_metadata_schedule_254_e2682 * 0.3333333333333333);
        let noise_metadata_schedule_254_e2685: f64 = (1.0 + noise_metadata_schedule_254_e2684);
        let noise_metadata_schedule_254_e2686: f64 = (noise_metadata_schedule_254_e2678 * noise_metadata_schedule_254_e2685);
        let noise_metadata_schedule_254_e2687: f64 = (1.0 + noise_metadata_schedule_254_e2686);
        let noise_metadata_schedule_254_e2688: f64 = (noise_metadata_schedule_254_e2672 * noise_metadata_schedule_254_e2687);
        let noise_metadata_schedule_254_e2689: f64 = (1.0 + noise_metadata_schedule_254_e2688);
        let noise_metadata_schedule_254_e2690: f64 = (1e100 * noise_metadata_schedule_254_e2689);
        (noise_metadata_schedule_254_e2690,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_254_e2692;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_255_e2703,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_255_e2701: f64 = (1.0 / noise_variable_208);
        (noise_metadata_schedule_255_e2701,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_255_e2703;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_256_e2718,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_256_e2714: f64 = (noise_variable_201 * noise_variable_201);
        let noise_metadata_schedule_256_e2715: f64 = (2.0 + noise_metadata_schedule_256_e2714);
        let noise_metadata_schedule_256_e2716: f64 = (1.0 / noise_metadata_schedule_256_e2715);
        (noise_metadata_schedule_256_e2716,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_256_e2718;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_257_e2729,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_257_e2727: f64 = (noise_variable_199 - noise_variable_201);
        (noise_metadata_schedule_257_e2727,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_257_e2729;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_258_e2740,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_258_e2738: f64 = (noise_variable_53 * noise_variable_209);
        (noise_metadata_schedule_258_e2738,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_258_e2740;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_259_e2761,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_259_e2749: f64 = (2.0 * noise_variable_197);
        let noise_metadata_schedule_259_e2753: f64 = (noise_variable_208 - 1.0);
        let noise_metadata_schedule_259_e2755: f64 = (noise_metadata_schedule_259_e2753 - noise_variable_198);
        let noise_metadata_schedule_259_e2757: f64 = (noise_metadata_schedule_259_e2755 + noise_variable_53);
        let noise_metadata_schedule_259_e2758: f64 = (noise_variable_38 * noise_metadata_schedule_259_e2757);
        let noise_metadata_schedule_259_e2759: f64 = (noise_metadata_schedule_259_e2749 + noise_metadata_schedule_259_e2758);
        (noise_metadata_schedule_259_e2759,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_259_e2761;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_260_e2788,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_260_e2770: f64 = (noise_variable_197 * noise_variable_197);
        let noise_metadata_schedule_260_e2774: f64 = (noise_variable_208 - noise_variable_201);
        let noise_metadata_schedule_260_e2776: f64 = (noise_metadata_schedule_260_e2774 - 1.0);
        let noise_metadata_schedule_260_e2778: f64 = (noise_metadata_schedule_260_e2776 + noise_variable_198);
        let noise_metadata_schedule_260_e2782: f64 = (noise_variable_201 - 1.0);
        let noise_metadata_schedule_260_e2783: f64 = (noise_variable_53 * noise_metadata_schedule_260_e2782);
        let noise_metadata_schedule_260_e2784: f64 = (noise_metadata_schedule_260_e2778 + noise_metadata_schedule_260_e2783);
        let noise_metadata_schedule_260_e2785: f64 = (noise_variable_38 * noise_metadata_schedule_260_e2784);
        let noise_metadata_schedule_260_e2786: f64 = (noise_metadata_schedule_260_e2770 - noise_metadata_schedule_260_e2785);
        (noise_metadata_schedule_260_e2786,)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_260_e2788;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_261_e2803,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_261_e2799: f64 = (noise_variable_208 + noise_variable_198);
        let noise_metadata_schedule_261_e2800: f64 = (noise_variable_38 * noise_metadata_schedule_261_e2799);
        let noise_metadata_schedule_261_e2801: f64 = (2.0 - noise_metadata_schedule_261_e2800);
        (noise_metadata_schedule_261_e2801,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_261_e2803;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_262_e2820,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_262_e2812: f64 = (noise_variable_210 * noise_variable_210);
        let noise_metadata_schedule_262_e2815: f64 = (2.0 * noise_variable_211);
        let noise_metadata_schedule_262_e2817: f64 = (noise_metadata_schedule_262_e2815 * noise_variable_197);
        let noise_metadata_schedule_262_e2818: f64 = (noise_metadata_schedule_262_e2812 - noise_metadata_schedule_262_e2817);
        (noise_metadata_schedule_262_e2818,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_262_e2820;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_263_e2839,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 != 0.0)) {
        let noise_metadata_schedule_263_e2828: f64 = (-noise_variable_201);
        let noise_metadata_schedule_263_e2831: f64 = (2.0 * noise_variable_211);
        let noise_metadata_schedule_263_e2834: f64 = (noise_variable_197).sqrt();
        let noise_metadata_schedule_263_e2835: f64 = (noise_variable_210 + noise_metadata_schedule_263_e2834);
        let noise_metadata_schedule_263_e2836: f64 = (noise_metadata_schedule_263_e2831 / noise_metadata_schedule_263_e2835);
        let noise_metadata_schedule_263_e2837: f64 = (noise_metadata_schedule_263_e2828 - noise_metadata_schedule_263_e2836);
        (noise_metadata_schedule_263_e2837,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_263_e2839;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_264_e2855,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_264_e2851: f64 = (noise_variable_35 * 0.7324648775608221);
        let noise_metadata_schedule_264_e2852: f64 = (1.25 + noise_metadata_schedule_264_e2851);
        let noise_metadata_schedule_264_e2853: f64 = (1.0 / noise_metadata_schedule_264_e2852);
        (noise_metadata_schedule_264_e2853,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_264_e2855;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_265_e2873,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_265_e2865: f64 = (noise_variable_45 * 1.25);
        let noise_metadata_schedule_265_e2867: f64 = (noise_metadata_schedule_265_e2865 * noise_variable_196);
        let noise_metadata_schedule_265_e2869: f64 = (noise_metadata_schedule_265_e2867 - 1.0);
        let noise_metadata_schedule_265_e2871: f64 = (noise_metadata_schedule_265_e2869 * noise_variable_196);
        (noise_metadata_schedule_265_e2871,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_265_e2873;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_266_e2891,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_266_e2883: f64 = (noise_variable_80 * noise_variable_46);
        let noise_metadata_schedule_266_e2887: f64 = (noise_variable_212 * noise_variable_80);
        let noise_metadata_schedule_266_e2888: f64 = (1.0 + noise_metadata_schedule_266_e2887);
        let noise_metadata_schedule_266_e2889: f64 = (noise_metadata_schedule_266_e2883 * noise_metadata_schedule_266_e2888);
        (noise_metadata_schedule_266_e2889,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_266_e2891;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_267_e2893: f64 = (-noise_variable_215);
            let noise_metadata_schedule_267_e2895: f64 = (-230.25850929940458);
            let noise_metadata_schedule_267_e2896: f64 = if noise_metadata_schedule_267_e2893 > noise_metadata_schedule_267_e2895 { 1.0 } else { 0.0 };
            noise_variable_222 = noise_metadata_schedule_267_e2896;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_268_e2910,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_222 != 0.0)) {
        let noise_metadata_schedule_268_e2907: f64 = (-noise_variable_215);
        let noise_metadata_schedule_268_e2908: f64 = (noise_metadata_schedule_268_e2907).exp();
        (noise_metadata_schedule_268_e2908,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_268_e2910;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_269_e2951,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_222 == 0.0)) {
        let noise_metadata_schedule_269_e2924: f64 = (-230.25850929940458);
        let noise_metadata_schedule_269_e2926: f64 = (-noise_variable_215);
        let noise_metadata_schedule_269_e2927: f64 = (noise_metadata_schedule_269_e2924 - noise_metadata_schedule_269_e2926);
        let noise_metadata_schedule_269_e2931: f64 = (-230.25850929940458);
        let noise_metadata_schedule_269_e2933: f64 = (-noise_variable_215);
        let noise_metadata_schedule_269_e2934: f64 = (noise_metadata_schedule_269_e2931 - noise_metadata_schedule_269_e2933);
        let noise_metadata_schedule_269_e2935: f64 = (0.5 * noise_metadata_schedule_269_e2934);
        let noise_metadata_schedule_269_e2938: f64 = (-230.25850929940458);
        let noise_metadata_schedule_269_e2940: f64 = (-noise_variable_215);
        let noise_metadata_schedule_269_e2941: f64 = (noise_metadata_schedule_269_e2938 - noise_metadata_schedule_269_e2940);
        let noise_metadata_schedule_269_e2943: f64 = (noise_metadata_schedule_269_e2941 * 0.3333333333333333);
        let noise_metadata_schedule_269_e2944: f64 = (1.0 + noise_metadata_schedule_269_e2943);
        let noise_metadata_schedule_269_e2945: f64 = (noise_metadata_schedule_269_e2935 * noise_metadata_schedule_269_e2944);
        let noise_metadata_schedule_269_e2946: f64 = (1.0 + noise_metadata_schedule_269_e2945);
        let noise_metadata_schedule_269_e2947: f64 = (noise_metadata_schedule_269_e2927 * noise_metadata_schedule_269_e2946);
        let noise_metadata_schedule_269_e2948: f64 = (1.0 + noise_metadata_schedule_269_e2947);
        let noise_metadata_schedule_269_e2949: f64 = (1e-100 / noise_metadata_schedule_269_e2948);
        (noise_metadata_schedule_269_e2949,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_269_e2951;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_270_e2963,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_270_e2961: f64 = (1.0 - noise_variable_197);
        (noise_metadata_schedule_270_e2961,)
    } else {
        (noise_variable_214,)
    }
};
            noise_variable_214 = noise_metadata_schedule_270_e2963;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_271_e2988,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_271_e2974: f64 = (noise_variable_38 * 0.5);
        let noise_metadata_schedule_271_e2975: f64 = (noise_variable_80 + noise_metadata_schedule_271_e2974);
        let noise_metadata_schedule_271_e2980: f64 = (noise_variable_38 * 0.25);
        let noise_metadata_schedule_271_e2981: f64 = (noise_variable_80 + noise_metadata_schedule_271_e2980);
        let noise_metadata_schedule_271_e2983: f64 = (noise_metadata_schedule_271_e2981 - noise_variable_214);
        let noise_metadata_schedule_271_e2984: f64 = (noise_metadata_schedule_271_e2983).sqrt();
        let noise_metadata_schedule_271_e2985: f64 = (noise_variable_35 * noise_metadata_schedule_271_e2984);
        let noise_metadata_schedule_271_e2986: f64 = (noise_metadata_schedule_271_e2975 - noise_metadata_schedule_271_e2985);
        (noise_metadata_schedule_271_e2986,)
    } else {
        (noise_variable_213,)
    }
};
            noise_variable_213 = noise_metadata_schedule_271_e2988;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_272_e3000,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_272_e2998: f64 = (noise_variable_51 + 3.0);
        (noise_metadata_schedule_272_e2998,)
    } else {
        (noise_variable_205,)
    }
};
            noise_variable_205 = noise_metadata_schedule_272_e3000;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_273_e3082,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_273_e3010: f64 = (noise_variable_205 - noise_variable_213);
        let (noise_metadata_schedule_273_e3069,) = {
            if (noise_metadata_schedule_273_e3010 > 1e-16) {
                let noise_metadata_schedule_273_e3017: f64 = (noise_variable_205 - noise_variable_213);
                let noise_metadata_schedule_273_e3020: f64 = (noise_variable_205 - noise_variable_213);
                let noise_metadata_schedule_273_e3023: f64 = (noise_variable_205 - noise_variable_213);
                let noise_metadata_schedule_273_e3024: f64 = (noise_metadata_schedule_273_e3020 * noise_metadata_schedule_273_e3023);
                let noise_metadata_schedule_273_e3026: f64 = (noise_metadata_schedule_273_e3024 + 5.0);
                let noise_metadata_schedule_273_e3027: f64 = (noise_metadata_schedule_273_e3026).sqrt();
                let noise_metadata_schedule_273_e3028: f64 = (noise_metadata_schedule_273_e3017 + noise_metadata_schedule_273_e3027);
                let noise_metadata_schedule_273_e3029: f64 = (0.5 * noise_metadata_schedule_273_e3028);
                let noise_metadata_schedule_273_e3030: f64 = (noise_variable_205 - noise_metadata_schedule_273_e3029);
                (noise_metadata_schedule_273_e3030,)
            } else {
                let noise_metadata_schedule_273_e3033: f64 = (noise_variable_213 - noise_variable_205);
                let (noise_metadata_schedule_273_e3068,) = {
                    if (noise_metadata_schedule_273_e3033 > 1e-16) {
                        let noise_metadata_schedule_273_e3039: f64 = (0.5 * 5.0);
                        let noise_metadata_schedule_273_e3042: f64 = (noise_variable_213 - noise_variable_205);
                        let noise_metadata_schedule_273_e3045: f64 = (noise_variable_213 - noise_variable_205);
                        let noise_metadata_schedule_273_e3048: f64 = (noise_variable_213 - noise_variable_205);
                        let noise_metadata_schedule_273_e3049: f64 = (noise_metadata_schedule_273_e3045 * noise_metadata_schedule_273_e3048);
                        let noise_metadata_schedule_273_e3051: f64 = (noise_metadata_schedule_273_e3049 + 5.0);
                        let noise_metadata_schedule_273_e3052: f64 = (noise_metadata_schedule_273_e3051).sqrt();
                        let noise_metadata_schedule_273_e3053: f64 = (noise_metadata_schedule_273_e3042 + noise_metadata_schedule_273_e3052);
                        let noise_metadata_schedule_273_e3054: f64 = (noise_metadata_schedule_273_e3039 / noise_metadata_schedule_273_e3053);
                        let noise_metadata_schedule_273_e3055: f64 = (noise_variable_205 - noise_metadata_schedule_273_e3054);
                        (noise_metadata_schedule_273_e3055,)
                    } else {
                        let noise_metadata_schedule_273_e3060: f64 = (noise_variable_205 - noise_variable_213);
                        let noise_metadata_schedule_273_e3063: f64 = (1e-32 + 5.0);
                        let noise_metadata_schedule_273_e3064: f64 = (noise_metadata_schedule_273_e3063).sqrt();
                        let noise_metadata_schedule_273_e3065: f64 = (noise_metadata_schedule_273_e3060 + noise_metadata_schedule_273_e3064);
                        let noise_metadata_schedule_273_e3066: f64 = (0.5 * noise_metadata_schedule_273_e3065);
                        let noise_metadata_schedule_273_e3067: f64 = (noise_variable_205 - noise_metadata_schedule_273_e3066);
                        (noise_metadata_schedule_273_e3067,)
                    }
                };
                (noise_metadata_schedule_273_e3068,)
            }
        };
        let noise_metadata_schedule_273_e3074: f64 = (noise_variable_205 * noise_variable_205);
        let noise_metadata_schedule_273_e3076: f64 = (noise_metadata_schedule_273_e3074 + 5.0);
        let noise_metadata_schedule_273_e3077: f64 = (noise_metadata_schedule_273_e3076).sqrt();
        let noise_metadata_schedule_273_e3078: f64 = (noise_variable_205 - noise_metadata_schedule_273_e3077);
        let noise_metadata_schedule_273_e3079: f64 = (0.5 * noise_metadata_schedule_273_e3078);
        let noise_metadata_schedule_273_e3080: f64 = (noise_metadata_schedule_273_e3069 - noise_metadata_schedule_273_e3079);
        (noise_metadata_schedule_273_e3080,)
    } else {
        (noise_variable_207,)
    }
};
            noise_variable_207 = noise_metadata_schedule_273_e3082;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_274_e3094,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_274_e3092: f64 = (noise_variable_80 - noise_variable_207);
        (noise_metadata_schedule_274_e3092,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_274_e3094;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_275_e3106,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_275_e3103: f64 = (-noise_variable_207);
        let noise_metadata_schedule_275_e3104: f64 = (noise_metadata_schedule_275_e3103).exp();
        (noise_metadata_schedule_275_e3104,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_275_e3106;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_276_e3134,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_276_e3117: f64 = (noise_variable_197 * noise_variable_197);
        let noise_metadata_schedule_276_e3121: f64 = (noise_variable_198 + noise_variable_207);
        let noise_metadata_schedule_276_e3123: f64 = (noise_metadata_schedule_276_e3121 - 1.0);
        let noise_metadata_schedule_276_e3127: f64 = (noise_variable_207 + 1.0);
        let noise_metadata_schedule_276_e3128: f64 = (noise_variable_53 * noise_metadata_schedule_276_e3127);
        let noise_metadata_schedule_276_e3129: f64 = (noise_metadata_schedule_276_e3123 - noise_metadata_schedule_276_e3128);
        let noise_metadata_schedule_276_e3130: f64 = (noise_variable_38 * noise_metadata_schedule_276_e3129);
        let noise_metadata_schedule_276_e3131: f64 = (noise_metadata_schedule_276_e3117 - noise_metadata_schedule_276_e3130);
        let noise_metadata_schedule_276_e3132: f64 = (1e-40_f64).max(noise_metadata_schedule_276_e3131);
        (noise_metadata_schedule_276_e3132,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_276_e3134;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_277_e3150,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_277_e3145: f64 = (0.5 * noise_variable_38);
        let noise_metadata_schedule_277_e3147: f64 = (noise_metadata_schedule_277_e3145 * noise_variable_198);
        let noise_metadata_schedule_277_e3148: f64 = (1.0 - noise_metadata_schedule_277_e3147);
        (noise_metadata_schedule_277_e3148,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_277_e3150;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_278_e3170,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_278_e3160: f64 = (2.0 * noise_variable_197);
        let noise_metadata_schedule_278_e3164: f64 = (1.0 - noise_variable_198);
        let noise_metadata_schedule_278_e3166: f64 = (noise_metadata_schedule_278_e3164 - noise_variable_53);
        let noise_metadata_schedule_278_e3167: f64 = (noise_variable_38 * noise_metadata_schedule_278_e3166);
        let noise_metadata_schedule_278_e3168: f64 = (noise_metadata_schedule_278_e3160 + noise_metadata_schedule_278_e3167);
        (noise_metadata_schedule_278_e3168,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_278_e3170;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_279_e3187,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_279_e3180: f64 = (noise_variable_51 - noise_variable_207);
        let noise_metadata_schedule_279_e3183: f64 = (noise_variable_202 / noise_variable_38);
        let noise_metadata_schedule_279_e3184: f64 = (noise_metadata_schedule_279_e3183).ln();
        let noise_metadata_schedule_279_e3185: f64 = (noise_metadata_schedule_279_e3180 + noise_metadata_schedule_279_e3184);
        (noise_metadata_schedule_279_e3185,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_279_e3187;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_280_e3199,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_280_e3197: f64 = (noise_variable_202 + noise_variable_204);
        (noise_metadata_schedule_280_e3197,)
    } else {
        (noise_variable_223,)
    }
};
            noise_variable_223 = noise_metadata_schedule_280_e3199;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_281_e3201: f64 = (noise_variable_206).abs();
            let noise_metadata_schedule_281_e3203: f64 = if noise_metadata_schedule_281_e3201 < 1e-120 { 1.0 } else { 0.0 };
            noise_variable_225 = noise_metadata_schedule_281_e3203;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_282_e3215,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_225 != 0.0)) {
        (noise_variable_207,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_282_e3215;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_283_e3242,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_225 == 0.0)) {
        let noise_metadata_schedule_283_e3228: f64 = (noise_variable_223 * noise_variable_223);
        let noise_metadata_schedule_283_e3231: f64 = (0.5 * noise_variable_204);
        let noise_metadata_schedule_283_e3233: f64 = (noise_metadata_schedule_283_e3231 * noise_variable_204);
        let noise_metadata_schedule_283_e3236: f64 = (noise_variable_202 * noise_variable_203);
        let noise_metadata_schedule_283_e3237: f64 = (noise_metadata_schedule_283_e3233 - noise_metadata_schedule_283_e3236);
        let noise_metadata_schedule_283_e3239: f64 = (noise_metadata_schedule_283_e3237 * noise_variable_206);
        let noise_metadata_schedule_283_e3240: f64 = (noise_metadata_schedule_283_e3228 + noise_metadata_schedule_283_e3239);
        (noise_metadata_schedule_283_e3240,)
    } else {
        (noise_variable_224,)
    }
};
            noise_variable_224 = noise_metadata_schedule_283_e3242;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_284_e3283,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_225 == 0.0)) {
        let noise_metadata_schedule_284_e3256: f64 = (noise_variable_202 * noise_variable_223);
        let noise_metadata_schedule_284_e3258: f64 = (noise_metadata_schedule_284_e3256 * noise_variable_206);
        let noise_metadata_schedule_284_e3262: f64 = (noise_variable_223 * noise_variable_206);
        let noise_metadata_schedule_284_e3264: f64 = (noise_metadata_schedule_284_e3262 * noise_variable_206);
        let noise_metadata_schedule_284_e3266: f64 = (noise_metadata_schedule_284_e3264 / noise_variable_224);
        let noise_metadata_schedule_284_e3268: f64 = (noise_metadata_schedule_284_e3266 * noise_variable_204);
        let noise_metadata_schedule_284_e3271: f64 = (noise_variable_204 * noise_variable_204);
        let noise_metadata_schedule_284_e3273: f64 = (noise_metadata_schedule_284_e3271 * 0.3333333333333333);
        let noise_metadata_schedule_284_e3276: f64 = (noise_variable_202 * noise_variable_203);
        let noise_metadata_schedule_284_e3277: f64 = (noise_metadata_schedule_284_e3273 - noise_metadata_schedule_284_e3276);
        let noise_metadata_schedule_284_e3278: f64 = (noise_metadata_schedule_284_e3268 * noise_metadata_schedule_284_e3277);
        let noise_metadata_schedule_284_e3279: f64 = (noise_variable_224 + noise_metadata_schedule_284_e3278);
        let noise_metadata_schedule_284_e3280: f64 = (noise_metadata_schedule_284_e3258 / noise_metadata_schedule_284_e3279);
        let noise_metadata_schedule_284_e3281: f64 = (noise_variable_207 + noise_metadata_schedule_284_e3280);
        (noise_metadata_schedule_284_e3281,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_284_e3283;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_285_e3286: f64 = if noise_variable_216 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_226 = noise_metadata_schedule_285_e3286;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_286_e3299,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_226 != 0.0)) {
        let noise_metadata_schedule_286_e3297: f64 = (noise_variable_216).exp();
        (noise_metadata_schedule_286_e3297,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_286_e3299;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_287_e3313,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_226 != 0.0)) {
        let noise_metadata_schedule_287_e3311: f64 = (1.0 / noise_variable_208);
        (noise_metadata_schedule_287_e3311,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_287_e3313;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_288_e3327,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_226 != 0.0)) {
        let noise_metadata_schedule_288_e3325: f64 = (noise_variable_53 * noise_variable_208);
        (noise_metadata_schedule_288_e3325,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_288_e3327;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_289_e3331: f64 = (noise_variable_51 - 230.25850929940458);
            let noise_metadata_schedule_289_e3332: f64 = if noise_variable_216 > noise_metadata_schedule_289_e3331 { 1.0 } else { 0.0 };
            noise_variable_227 = noise_metadata_schedule_289_e3332;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_290_e3350,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_226 == 0.0)) && (noise_variable_227 != 0.0)) {
        let noise_metadata_schedule_290_e3347: f64 = (noise_variable_216 - noise_variable_51);
        let noise_metadata_schedule_290_e3348: f64 = (noise_metadata_schedule_290_e3347).exp();
        (noise_metadata_schedule_290_e3348,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_290_e3350;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_291_e3367,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_226 == 0.0)) && (noise_variable_227 != 0.0)) {
        let noise_metadata_schedule_291_e3365: f64 = (noise_variable_53 / noise_variable_208);
        (noise_metadata_schedule_291_e3365,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_291_e3367;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_292_e3411,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_226 == 0.0)) && (noise_variable_227 == 0.0)) {
        let noise_metadata_schedule_292_e3385: f64 = (noise_variable_51 - noise_variable_216);
        let noise_metadata_schedule_292_e3387: f64 = (noise_metadata_schedule_292_e3385 - 230.25850929940458);
        let noise_metadata_schedule_292_e3392: f64 = (noise_variable_51 - noise_variable_216);
        let noise_metadata_schedule_292_e3394: f64 = (noise_metadata_schedule_292_e3392 - 230.25850929940458);
        let noise_metadata_schedule_292_e3395: f64 = (0.5 * noise_metadata_schedule_292_e3394);
        let noise_metadata_schedule_292_e3399: f64 = (noise_variable_51 - noise_variable_216);
        let noise_metadata_schedule_292_e3401: f64 = (noise_metadata_schedule_292_e3399 - 230.25850929940458);
        let noise_metadata_schedule_292_e3403: f64 = (noise_metadata_schedule_292_e3401 * 0.3333333333333333);
        let noise_metadata_schedule_292_e3404: f64 = (1.0 + noise_metadata_schedule_292_e3403);
        let noise_metadata_schedule_292_e3405: f64 = (noise_metadata_schedule_292_e3395 * noise_metadata_schedule_292_e3404);
        let noise_metadata_schedule_292_e3406: f64 = (1.0 + noise_metadata_schedule_292_e3405);
        let noise_metadata_schedule_292_e3407: f64 = (noise_metadata_schedule_292_e3387 * noise_metadata_schedule_292_e3406);
        let noise_metadata_schedule_292_e3408: f64 = (1.0 + noise_metadata_schedule_292_e3407);
        let noise_metadata_schedule_292_e3409: f64 = (1e-100 / noise_metadata_schedule_292_e3408);
        (noise_metadata_schedule_292_e3409,)
    } else {
        (noise_variable_208,)
    }
};
            noise_variable_208 = noise_metadata_schedule_292_e3411;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_293_e3449,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) && (noise_variable_226 == 0.0)) && (noise_variable_227 == 0.0)) {
        let noise_metadata_schedule_293_e3429: f64 = (noise_variable_216 - 230.25850929940458);
        let noise_metadata_schedule_293_e3434: f64 = (noise_variable_216 - 230.25850929940458);
        let noise_metadata_schedule_293_e3435: f64 = (0.5 * noise_metadata_schedule_293_e3434);
        let noise_metadata_schedule_293_e3439: f64 = (noise_variable_216 - 230.25850929940458);
        let noise_metadata_schedule_293_e3441: f64 = (noise_metadata_schedule_293_e3439 * 0.3333333333333333);
        let noise_metadata_schedule_293_e3442: f64 = (1.0 + noise_metadata_schedule_293_e3441);
        let noise_metadata_schedule_293_e3443: f64 = (noise_metadata_schedule_293_e3435 * noise_metadata_schedule_293_e3442);
        let noise_metadata_schedule_293_e3444: f64 = (1.0 + noise_metadata_schedule_293_e3443);
        let noise_metadata_schedule_293_e3445: f64 = (noise_metadata_schedule_293_e3429 * noise_metadata_schedule_293_e3444);
        let noise_metadata_schedule_293_e3446: f64 = (1.0 + noise_metadata_schedule_293_e3445);
        let noise_metadata_schedule_293_e3447: f64 = (1e-100 / noise_metadata_schedule_293_e3446);
        (noise_metadata_schedule_293_e3447,)
    } else {
        (noise_variable_209,)
    }
};
            noise_variable_209 = noise_metadata_schedule_293_e3449;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_294_e3465,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_294_e3461: f64 = (noise_variable_216 * noise_variable_216);
        let noise_metadata_schedule_294_e3462: f64 = (2.0 + noise_metadata_schedule_294_e3461);
        let noise_metadata_schedule_294_e3463: f64 = (1.0 / noise_metadata_schedule_294_e3462);
        (noise_metadata_schedule_294_e3463,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_294_e3465;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_295_e3477,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_295_e3475: f64 = (noise_variable_80 - noise_variable_216);
        (noise_metadata_schedule_295_e3475,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_295_e3477;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_296_e3499,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_296_e3487: f64 = (2.0 * noise_variable_197);
        let noise_metadata_schedule_296_e3491: f64 = (1.0 - noise_variable_209);
        let noise_metadata_schedule_296_e3493: f64 = (noise_metadata_schedule_296_e3491 + noise_variable_208);
        let noise_metadata_schedule_296_e3495: f64 = (noise_metadata_schedule_296_e3493 - noise_variable_53);
        let noise_metadata_schedule_296_e3496: f64 = (noise_variable_38 * noise_metadata_schedule_296_e3495);
        let noise_metadata_schedule_296_e3497: f64 = (noise_metadata_schedule_296_e3487 + noise_metadata_schedule_296_e3496);
        (noise_metadata_schedule_296_e3497,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_296_e3499;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_297_e3527,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_297_e3509: f64 = (noise_variable_197 * noise_variable_197);
        let noise_metadata_schedule_297_e3513: f64 = (noise_variable_209 + noise_variable_216);
        let noise_metadata_schedule_297_e3515: f64 = (noise_metadata_schedule_297_e3513 - 1.0);
        let noise_metadata_schedule_297_e3517: f64 = (noise_metadata_schedule_297_e3515 + noise_variable_208);
        let noise_metadata_schedule_297_e3521: f64 = (noise_variable_216 + 1.0);
        let noise_metadata_schedule_297_e3522: f64 = (noise_variable_53 * noise_metadata_schedule_297_e3521);
        let noise_metadata_schedule_297_e3523: f64 = (noise_metadata_schedule_297_e3517 - noise_metadata_schedule_297_e3522);
        let noise_metadata_schedule_297_e3524: f64 = (noise_variable_38 * noise_metadata_schedule_297_e3523);
        let noise_metadata_schedule_297_e3525: f64 = (noise_metadata_schedule_297_e3509 - noise_metadata_schedule_297_e3524);
        (noise_metadata_schedule_297_e3525,)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_297_e3527;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_298_e3543,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_298_e3539: f64 = (noise_variable_209 + noise_variable_208);
        let noise_metadata_schedule_298_e3540: f64 = (noise_variable_38 * noise_metadata_schedule_298_e3539);
        let noise_metadata_schedule_298_e3541: f64 = (2.0 - noise_metadata_schedule_298_e3540);
        (noise_metadata_schedule_298_e3541,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_298_e3543;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_299_e3561,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_299_e3553: f64 = (noise_variable_210 * noise_variable_210);
        let noise_metadata_schedule_299_e3556: f64 = (2.0 * noise_variable_211);
        let noise_metadata_schedule_299_e3558: f64 = (noise_metadata_schedule_299_e3556 * noise_variable_197);
        let noise_metadata_schedule_299_e3559: f64 = (noise_metadata_schedule_299_e3553 - noise_metadata_schedule_299_e3558);
        (noise_metadata_schedule_299_e3559,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_299_e3561;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_300_e3580,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_217 == 0.0)) && (noise_variable_218 == 0.0)) {
        let noise_metadata_schedule_300_e3572: f64 = (2.0 * noise_variable_211);
        let noise_metadata_schedule_300_e3575: f64 = (noise_variable_197).sqrt();
        let noise_metadata_schedule_300_e3576: f64 = (noise_variable_210 + noise_metadata_schedule_300_e3575);
        let noise_metadata_schedule_300_e3577: f64 = (noise_metadata_schedule_300_e3572 / noise_metadata_schedule_300_e3576);
        let noise_metadata_schedule_300_e3578: f64 = (noise_variable_216 + noise_metadata_schedule_300_e3577);
        (noise_metadata_schedule_300_e3578,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_300_e3580;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_301_e3591,) = {
    if (noise_variable_195 != 0.0) {
        let noise_metadata_schedule_301_e3583: f64 = (-params.p17);
        let noise_metadata_schedule_301_e3585: f64 = (noise_metadata_schedule_301_e3583 * params.p18);
        let noise_metadata_schedule_301_e3587: f64 = (noise_metadata_schedule_301_e3585 * noise_variable_81);
        let noise_metadata_schedule_301_e3589: f64 = (noise_metadata_schedule_301_e3587 * noise_variable_25);
        (noise_metadata_schedule_301_e3589,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_301_e3591;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_302_e3599,) = {
    if (noise_variable_195 != 0.0) {
        let noise_metadata_schedule_302_e3595: f64 = (noise_variable_77 - noise_variable_82);
        let noise_metadata_schedule_302_e3597: f64 = (noise_metadata_schedule_302_e3595 / noise_variable_25);
        (noise_metadata_schedule_302_e3597,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_302_e3599;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_303_e3601: f64 = (noise_variable_78).abs();
            let noise_metadata_schedule_303_e3603: f64 = if noise_metadata_schedule_303_e3601 <= noise_variable_40 { 1.0 } else { 0.0 };
            noise_variable_249 = noise_metadata_schedule_303_e3603;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_304_e3615,) = {
    if ((noise_variable_195 != 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_304_e3609: f64 = (noise_variable_44 * noise_variable_44);
        let noise_metadata_schedule_304_e3611: f64 = (noise_metadata_schedule_304_e3609 * 0.1666666666666667);
        let noise_metadata_schedule_304_e3613: f64 = (noise_metadata_schedule_304_e3611 * 0.7071067811865475);
        (noise_metadata_schedule_304_e3613,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_304_e3615;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_305_e3635,) = {
    if ((noise_variable_195 != 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_305_e3621: f64 = (noise_variable_78 * noise_variable_44);
        let noise_metadata_schedule_305_e3626: f64 = (1.0 - noise_variable_52);
        let noise_metadata_schedule_305_e3627: f64 = (noise_variable_78 * noise_metadata_schedule_305_e3626);
        let noise_metadata_schedule_305_e3629: f64 = (noise_metadata_schedule_305_e3627 * noise_variable_34);
        let noise_metadata_schedule_305_e3631: f64 = (noise_metadata_schedule_305_e3629 * noise_variable_230);
        let noise_metadata_schedule_305_e3632: f64 = (1.0 + noise_metadata_schedule_305_e3631);
        let noise_metadata_schedule_305_e3633: f64 = (noise_metadata_schedule_305_e3621 * noise_metadata_schedule_305_e3632);
        (noise_metadata_schedule_305_e3633,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_305_e3635;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_306_e3638: f64 = (-noise_variable_40);
            let noise_metadata_schedule_306_e3639: f64 = if noise_variable_78 < noise_metadata_schedule_306_e3638 { 1.0 } else { 0.0 };
            noise_variable_250 = noise_metadata_schedule_306_e3639;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_307_e3649,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_307_e3647: f64 = (-noise_variable_78);
        (noise_metadata_schedule_307_e3647,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_307_e3649;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_308_e3662,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_308_e3658: f64 = (1.25 * noise_variable_231);
        let noise_metadata_schedule_308_e3660: f64 = (noise_metadata_schedule_308_e3658 * noise_variable_44);
        (noise_metadata_schedule_308_e3660,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_308_e3662;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_309_e3686,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_309_e3672: f64 = (noise_variable_232 + 10.0);
        let noise_metadata_schedule_309_e3675: f64 = (noise_variable_232 - 6.0);
        let noise_metadata_schedule_309_e3678: f64 = (noise_variable_232 - 6.0);
        let noise_metadata_schedule_309_e3679: f64 = (noise_metadata_schedule_309_e3675 * noise_metadata_schedule_309_e3678);
        let noise_metadata_schedule_309_e3681: f64 = (noise_metadata_schedule_309_e3679 + 64.0);
        let noise_metadata_schedule_309_e3682: f64 = (noise_metadata_schedule_309_e3681).sqrt();
        let noise_metadata_schedule_309_e3683: f64 = (noise_metadata_schedule_309_e3672 - noise_metadata_schedule_309_e3682);
        let noise_metadata_schedule_309_e3684: f64 = (0.5 * noise_metadata_schedule_309_e3683);
        (noise_metadata_schedule_309_e3684,)
    } else {
        (noise_variable_239,)
    }
};
            noise_variable_239 = noise_metadata_schedule_309_e3686;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_310_e3697,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_310_e3695: f64 = (noise_variable_231 - noise_variable_239);
        (noise_metadata_schedule_310_e3695,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_310_e3697;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_311_e3714,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_311_e3706: f64 = (noise_variable_229 * noise_variable_229);
        let noise_metadata_schedule_311_e3710: f64 = (noise_variable_239 + 1.0);
        let noise_metadata_schedule_311_e3711: f64 = (noise_variable_36 * noise_metadata_schedule_311_e3710);
        let noise_metadata_schedule_311_e3712: f64 = (noise_metadata_schedule_311_e3706 + noise_metadata_schedule_311_e3711);
        (noise_metadata_schedule_311_e3712,)
    } else {
        (noise_variable_234,)
    }
};
            noise_variable_234 = noise_metadata_schedule_311_e3714;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_312_e3727,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_312_e3723: f64 = (2.0 * noise_variable_229);
        let noise_metadata_schedule_312_e3725: f64 = (noise_metadata_schedule_312_e3723 - noise_variable_36);
        (noise_metadata_schedule_312_e3725,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_312_e3727;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_313_e3742,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_313_e3735: f64 = (-noise_variable_239);
        let noise_metadata_schedule_313_e3738: f64 = (noise_variable_234 * noise_variable_37);
        let noise_metadata_schedule_313_e3739: f64 = (noise_metadata_schedule_313_e3738).ln();
        let noise_metadata_schedule_313_e3740: f64 = (noise_metadata_schedule_313_e3735 + noise_metadata_schedule_313_e3739);
        (noise_metadata_schedule_313_e3740,)
    } else {
        (noise_variable_238,)
    }
};
            noise_variable_238 = noise_metadata_schedule_313_e3742;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_314_e3753,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_314_e3751: f64 = (noise_variable_234 + noise_variable_236);
        (noise_metadata_schedule_314_e3751,)
    } else {
        (noise_variable_251,)
    }
};
            noise_variable_251 = noise_metadata_schedule_314_e3753;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_315_e3774,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_315_e3762: f64 = (noise_variable_251 * noise_variable_251);
        let noise_metadata_schedule_315_e3765: f64 = (0.5 * noise_variable_236);
        let noise_metadata_schedule_315_e3767: f64 = (noise_metadata_schedule_315_e3765 * noise_variable_236);
        let noise_metadata_schedule_315_e3769: f64 = (noise_metadata_schedule_315_e3767 - noise_variable_234);
        let noise_metadata_schedule_315_e3771: f64 = (noise_metadata_schedule_315_e3769 * noise_variable_238);
        let noise_metadata_schedule_315_e3772: f64 = (noise_metadata_schedule_315_e3762 + noise_metadata_schedule_315_e3771);
        (noise_metadata_schedule_315_e3772,)
    } else {
        (noise_variable_252,)
    }
};
            noise_variable_252 = noise_metadata_schedule_315_e3774;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_316_e3809,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_316_e3784: f64 = (noise_variable_234 * noise_variable_251);
        let noise_metadata_schedule_316_e3786: f64 = (noise_metadata_schedule_316_e3784 * noise_variable_238);
        let noise_metadata_schedule_316_e3790: f64 = (noise_variable_251 * noise_variable_238);
        let noise_metadata_schedule_316_e3792: f64 = (noise_metadata_schedule_316_e3790 * noise_variable_238);
        let noise_metadata_schedule_316_e3794: f64 = (noise_metadata_schedule_316_e3792 / noise_variable_252);
        let noise_metadata_schedule_316_e3796: f64 = (noise_metadata_schedule_316_e3794 * noise_variable_236);
        let noise_metadata_schedule_316_e3799: f64 = (noise_variable_236 * noise_variable_236);
        let noise_metadata_schedule_316_e3801: f64 = (noise_metadata_schedule_316_e3799 * 0.3333333333333333);
        let noise_metadata_schedule_316_e3803: f64 = (noise_metadata_schedule_316_e3801 - noise_variable_234);
        let noise_metadata_schedule_316_e3804: f64 = (noise_metadata_schedule_316_e3796 * noise_metadata_schedule_316_e3803);
        let noise_metadata_schedule_316_e3805: f64 = (noise_variable_252 + noise_metadata_schedule_316_e3804);
        let noise_metadata_schedule_316_e3806: f64 = (noise_metadata_schedule_316_e3786 / noise_metadata_schedule_316_e3805);
        let noise_metadata_schedule_316_e3807: f64 = (noise_variable_239 + noise_metadata_schedule_316_e3806);
        (noise_metadata_schedule_316_e3807,)
    } else {
        (noise_variable_233,)
    }
};
            noise_variable_233 = noise_metadata_schedule_316_e3809;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_317_e3812: f64 = if noise_variable_233 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_253 = noise_metadata_schedule_317_e3812;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_318_e3824,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) && (noise_variable_253 != 0.0)) {
        let noise_metadata_schedule_318_e3822: f64 = (noise_variable_233).exp();
        (noise_metadata_schedule_318_e3822,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_318_e3824;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_319_e3858,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) && (noise_variable_253 == 0.0)) {
        let noise_metadata_schedule_319_e3838: f64 = (noise_variable_233 - 230.25850929940458);
        let noise_metadata_schedule_319_e3843: f64 = (noise_variable_233 - 230.25850929940458);
        let noise_metadata_schedule_319_e3844: f64 = (0.5 * noise_metadata_schedule_319_e3843);
        let noise_metadata_schedule_319_e3848: f64 = (noise_variable_233 - 230.25850929940458);
        let noise_metadata_schedule_319_e3850: f64 = (noise_metadata_schedule_319_e3848 * 0.3333333333333333);
        let noise_metadata_schedule_319_e3851: f64 = (1.0 + noise_metadata_schedule_319_e3850);
        let noise_metadata_schedule_319_e3852: f64 = (noise_metadata_schedule_319_e3844 * noise_metadata_schedule_319_e3851);
        let noise_metadata_schedule_319_e3853: f64 = (1.0 + noise_metadata_schedule_319_e3852);
        let noise_metadata_schedule_319_e3854: f64 = (noise_metadata_schedule_319_e3838 * noise_metadata_schedule_319_e3853);
        let noise_metadata_schedule_319_e3855: f64 = (1.0 + noise_metadata_schedule_319_e3854);
        let noise_metadata_schedule_319_e3856: f64 = (1e100 * noise_metadata_schedule_319_e3855);
        (noise_metadata_schedule_319_e3856,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_319_e3858;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_320_e3869,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_320_e3867: f64 = (1.0 / noise_variable_240);
        (noise_metadata_schedule_320_e3867,)
    } else {
        (noise_variable_241,)
    }
};
            noise_variable_241 = noise_metadata_schedule_320_e3869;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_321_e3884,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_321_e3880: f64 = (noise_variable_233 * noise_variable_233);
        let noise_metadata_schedule_321_e3881: f64 = (2.0 + noise_metadata_schedule_321_e3880);
        let noise_metadata_schedule_321_e3882: f64 = (1.0 / noise_metadata_schedule_321_e3881);
        (noise_metadata_schedule_321_e3882,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_321_e3884;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_322_e3895,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_322_e3893: f64 = (noise_variable_231 - noise_variable_233);
        (noise_metadata_schedule_322_e3893,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_322_e3895;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_323_e3906,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_323_e3904: f64 = (noise_variable_52 * noise_variable_241);
        (noise_metadata_schedule_323_e3904,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_323_e3906;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_324_e3927,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_324_e3915: f64 = (2.0 * noise_variable_229);
        let noise_metadata_schedule_324_e3919: f64 = (noise_variable_240 - 1.0);
        let noise_metadata_schedule_324_e3921: f64 = (noise_metadata_schedule_324_e3919 - noise_variable_230);
        let noise_metadata_schedule_324_e3923: f64 = (noise_metadata_schedule_324_e3921 + noise_variable_52);
        let noise_metadata_schedule_324_e3924: f64 = (noise_variable_36 * noise_metadata_schedule_324_e3923);
        let noise_metadata_schedule_324_e3925: f64 = (noise_metadata_schedule_324_e3915 + noise_metadata_schedule_324_e3924);
        (noise_metadata_schedule_324_e3925,)
    } else {
        (noise_variable_242,)
    }
};
            noise_variable_242 = noise_metadata_schedule_324_e3927;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_325_e3954,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_325_e3936: f64 = (noise_variable_229 * noise_variable_229);
        let noise_metadata_schedule_325_e3940: f64 = (noise_variable_240 - noise_variable_233);
        let noise_metadata_schedule_325_e3942: f64 = (noise_metadata_schedule_325_e3940 - 1.0);
        let noise_metadata_schedule_325_e3944: f64 = (noise_metadata_schedule_325_e3942 + noise_variable_230);
        let noise_metadata_schedule_325_e3948: f64 = (noise_variable_233 - 1.0);
        let noise_metadata_schedule_325_e3949: f64 = (noise_variable_52 * noise_metadata_schedule_325_e3948);
        let noise_metadata_schedule_325_e3950: f64 = (noise_metadata_schedule_325_e3944 + noise_metadata_schedule_325_e3949);
        let noise_metadata_schedule_325_e3951: f64 = (noise_variable_36 * noise_metadata_schedule_325_e3950);
        let noise_metadata_schedule_325_e3952: f64 = (noise_metadata_schedule_325_e3936 - noise_metadata_schedule_325_e3951);
        (noise_metadata_schedule_325_e3952,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_325_e3954;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_326_e3969,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_326_e3965: f64 = (noise_variable_240 + noise_variable_230);
        let noise_metadata_schedule_326_e3966: f64 = (noise_variable_36 * noise_metadata_schedule_326_e3965);
        let noise_metadata_schedule_326_e3967: f64 = (2.0 - noise_metadata_schedule_326_e3966);
        (noise_metadata_schedule_326_e3967,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_326_e3969;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_327_e3986,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_327_e3978: f64 = (noise_variable_242 * noise_variable_242);
        let noise_metadata_schedule_327_e3981: f64 = (2.0 * noise_variable_243);
        let noise_metadata_schedule_327_e3983: f64 = (noise_metadata_schedule_327_e3981 * noise_variable_229);
        let noise_metadata_schedule_327_e3984: f64 = (noise_metadata_schedule_327_e3978 - noise_metadata_schedule_327_e3983);
        (noise_metadata_schedule_327_e3984,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_327_e3986;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_328_e4005,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 != 0.0)) {
        let noise_metadata_schedule_328_e3994: f64 = (-noise_variable_233);
        let noise_metadata_schedule_328_e3997: f64 = (2.0 * noise_variable_243);
        let noise_metadata_schedule_328_e4000: f64 = (noise_variable_229).sqrt();
        let noise_metadata_schedule_328_e4001: f64 = (noise_variable_242 + noise_metadata_schedule_328_e4000);
        let noise_metadata_schedule_328_e4002: f64 = (noise_metadata_schedule_328_e3997 / noise_metadata_schedule_328_e4001);
        let noise_metadata_schedule_328_e4003: f64 = (noise_metadata_schedule_328_e3994 - noise_metadata_schedule_328_e4002);
        (noise_metadata_schedule_328_e4003,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_328_e4005;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_329_e4021,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_329_e4017: f64 = (noise_variable_34 * 0.7324648775608221);
        let noise_metadata_schedule_329_e4018: f64 = (1.25 + noise_metadata_schedule_329_e4017);
        let noise_metadata_schedule_329_e4019: f64 = (1.0 / noise_metadata_schedule_329_e4018);
        (noise_metadata_schedule_329_e4019,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_329_e4021;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_330_e4039,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_330_e4031: f64 = (noise_variable_43 * 1.25);
        let noise_metadata_schedule_330_e4033: f64 = (noise_metadata_schedule_330_e4031 * noise_variable_228);
        let noise_metadata_schedule_330_e4035: f64 = (noise_metadata_schedule_330_e4033 - 1.0);
        let noise_metadata_schedule_330_e4037: f64 = (noise_metadata_schedule_330_e4035 * noise_variable_228);
        (noise_metadata_schedule_330_e4037,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_330_e4039;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_331_e4057,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_331_e4049: f64 = (noise_variable_78 * noise_variable_44);
        let noise_metadata_schedule_331_e4053: f64 = (noise_variable_244 * noise_variable_78);
        let noise_metadata_schedule_331_e4054: f64 = (1.0 + noise_metadata_schedule_331_e4053);
        let noise_metadata_schedule_331_e4055: f64 = (noise_metadata_schedule_331_e4049 * noise_metadata_schedule_331_e4054);
        (noise_metadata_schedule_331_e4055,)
    } else {
        (noise_variable_247,)
    }
};
            noise_variable_247 = noise_metadata_schedule_331_e4057;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_332_e4059: f64 = (-noise_variable_247);
            let noise_metadata_schedule_332_e4061: f64 = (-230.25850929940458);
            let noise_metadata_schedule_332_e4062: f64 = if noise_metadata_schedule_332_e4059 > noise_metadata_schedule_332_e4061 { 1.0 } else { 0.0 };
            noise_variable_254 = noise_metadata_schedule_332_e4062;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_333_e4076,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_254 != 0.0)) {
        let noise_metadata_schedule_333_e4073: f64 = (-noise_variable_247);
        let noise_metadata_schedule_333_e4074: f64 = (noise_metadata_schedule_333_e4073).exp();
        (noise_metadata_schedule_333_e4074,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_333_e4076;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_334_e4117,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_254 == 0.0)) {
        let noise_metadata_schedule_334_e4090: f64 = (-230.25850929940458);
        let noise_metadata_schedule_334_e4092: f64 = (-noise_variable_247);
        let noise_metadata_schedule_334_e4093: f64 = (noise_metadata_schedule_334_e4090 - noise_metadata_schedule_334_e4092);
        let noise_metadata_schedule_334_e4097: f64 = (-230.25850929940458);
        let noise_metadata_schedule_334_e4099: f64 = (-noise_variable_247);
        let noise_metadata_schedule_334_e4100: f64 = (noise_metadata_schedule_334_e4097 - noise_metadata_schedule_334_e4099);
        let noise_metadata_schedule_334_e4101: f64 = (0.5 * noise_metadata_schedule_334_e4100);
        let noise_metadata_schedule_334_e4104: f64 = (-230.25850929940458);
        let noise_metadata_schedule_334_e4106: f64 = (-noise_variable_247);
        let noise_metadata_schedule_334_e4107: f64 = (noise_metadata_schedule_334_e4104 - noise_metadata_schedule_334_e4106);
        let noise_metadata_schedule_334_e4109: f64 = (noise_metadata_schedule_334_e4107 * 0.3333333333333333);
        let noise_metadata_schedule_334_e4110: f64 = (1.0 + noise_metadata_schedule_334_e4109);
        let noise_metadata_schedule_334_e4111: f64 = (noise_metadata_schedule_334_e4101 * noise_metadata_schedule_334_e4110);
        let noise_metadata_schedule_334_e4112: f64 = (1.0 + noise_metadata_schedule_334_e4111);
        let noise_metadata_schedule_334_e4113: f64 = (noise_metadata_schedule_334_e4093 * noise_metadata_schedule_334_e4112);
        let noise_metadata_schedule_334_e4114: f64 = (1.0 + noise_metadata_schedule_334_e4113);
        let noise_metadata_schedule_334_e4115: f64 = (1e-100 / noise_metadata_schedule_334_e4114);
        (noise_metadata_schedule_334_e4115,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_334_e4117;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_335_e4129,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_335_e4127: f64 = (1.0 - noise_variable_229);
        (noise_metadata_schedule_335_e4127,)
    } else {
        (noise_variable_246,)
    }
};
            noise_variable_246 = noise_metadata_schedule_335_e4129;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_336_e4154,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_336_e4140: f64 = (noise_variable_36 * 0.5);
        let noise_metadata_schedule_336_e4141: f64 = (noise_variable_78 + noise_metadata_schedule_336_e4140);
        let noise_metadata_schedule_336_e4146: f64 = (noise_variable_36 * 0.25);
        let noise_metadata_schedule_336_e4147: f64 = (noise_variable_78 + noise_metadata_schedule_336_e4146);
        let noise_metadata_schedule_336_e4149: f64 = (noise_metadata_schedule_336_e4147 - noise_variable_246);
        let noise_metadata_schedule_336_e4150: f64 = (noise_metadata_schedule_336_e4149).sqrt();
        let noise_metadata_schedule_336_e4151: f64 = (noise_variable_34 * noise_metadata_schedule_336_e4150);
        let noise_metadata_schedule_336_e4152: f64 = (noise_metadata_schedule_336_e4141 - noise_metadata_schedule_336_e4151);
        (noise_metadata_schedule_336_e4152,)
    } else {
        (noise_variable_245,)
    }
};
            noise_variable_245 = noise_metadata_schedule_336_e4154;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_337_e4166,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_337_e4164: f64 = (noise_variable_50 + 3.0);
        (noise_metadata_schedule_337_e4164,)
    } else {
        (noise_variable_237,)
    }
};
            noise_variable_237 = noise_metadata_schedule_337_e4166;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_338_e4248,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_338_e4176: f64 = (noise_variable_237 - noise_variable_245);
        let (noise_metadata_schedule_338_e4235,) = {
            if (noise_metadata_schedule_338_e4176 > 1e-16) {
                let noise_metadata_schedule_338_e4183: f64 = (noise_variable_237 - noise_variable_245);
                let noise_metadata_schedule_338_e4186: f64 = (noise_variable_237 - noise_variable_245);
                let noise_metadata_schedule_338_e4189: f64 = (noise_variable_237 - noise_variable_245);
                let noise_metadata_schedule_338_e4190: f64 = (noise_metadata_schedule_338_e4186 * noise_metadata_schedule_338_e4189);
                let noise_metadata_schedule_338_e4192: f64 = (noise_metadata_schedule_338_e4190 + 5.0);
                let noise_metadata_schedule_338_e4193: f64 = (noise_metadata_schedule_338_e4192).sqrt();
                let noise_metadata_schedule_338_e4194: f64 = (noise_metadata_schedule_338_e4183 + noise_metadata_schedule_338_e4193);
                let noise_metadata_schedule_338_e4195: f64 = (0.5 * noise_metadata_schedule_338_e4194);
                let noise_metadata_schedule_338_e4196: f64 = (noise_variable_237 - noise_metadata_schedule_338_e4195);
                (noise_metadata_schedule_338_e4196,)
            } else {
                let noise_metadata_schedule_338_e4199: f64 = (noise_variable_245 - noise_variable_237);
                let (noise_metadata_schedule_338_e4234,) = {
                    if (noise_metadata_schedule_338_e4199 > 1e-16) {
                        let noise_metadata_schedule_338_e4205: f64 = (0.5 * 5.0);
                        let noise_metadata_schedule_338_e4208: f64 = (noise_variable_245 - noise_variable_237);
                        let noise_metadata_schedule_338_e4211: f64 = (noise_variable_245 - noise_variable_237);
                        let noise_metadata_schedule_338_e4214: f64 = (noise_variable_245 - noise_variable_237);
                        let noise_metadata_schedule_338_e4215: f64 = (noise_metadata_schedule_338_e4211 * noise_metadata_schedule_338_e4214);
                        let noise_metadata_schedule_338_e4217: f64 = (noise_metadata_schedule_338_e4215 + 5.0);
                        let noise_metadata_schedule_338_e4218: f64 = (noise_metadata_schedule_338_e4217).sqrt();
                        let noise_metadata_schedule_338_e4219: f64 = (noise_metadata_schedule_338_e4208 + noise_metadata_schedule_338_e4218);
                        let noise_metadata_schedule_338_e4220: f64 = (noise_metadata_schedule_338_e4205 / noise_metadata_schedule_338_e4219);
                        let noise_metadata_schedule_338_e4221: f64 = (noise_variable_237 - noise_metadata_schedule_338_e4220);
                        (noise_metadata_schedule_338_e4221,)
                    } else {
                        let noise_metadata_schedule_338_e4226: f64 = (noise_variable_237 - noise_variable_245);
                        let noise_metadata_schedule_338_e4229: f64 = (1e-32 + 5.0);
                        let noise_metadata_schedule_338_e4230: f64 = (noise_metadata_schedule_338_e4229).sqrt();
                        let noise_metadata_schedule_338_e4231: f64 = (noise_metadata_schedule_338_e4226 + noise_metadata_schedule_338_e4230);
                        let noise_metadata_schedule_338_e4232: f64 = (0.5 * noise_metadata_schedule_338_e4231);
                        let noise_metadata_schedule_338_e4233: f64 = (noise_variable_237 - noise_metadata_schedule_338_e4232);
                        (noise_metadata_schedule_338_e4233,)
                    }
                };
                (noise_metadata_schedule_338_e4234,)
            }
        };
        let noise_metadata_schedule_338_e4240: f64 = (noise_variable_237 * noise_variable_237);
        let noise_metadata_schedule_338_e4242: f64 = (noise_metadata_schedule_338_e4240 + 5.0);
        let noise_metadata_schedule_338_e4243: f64 = (noise_metadata_schedule_338_e4242).sqrt();
        let noise_metadata_schedule_338_e4244: f64 = (noise_variable_237 - noise_metadata_schedule_338_e4243);
        let noise_metadata_schedule_338_e4245: f64 = (0.5 * noise_metadata_schedule_338_e4244);
        let noise_metadata_schedule_338_e4246: f64 = (noise_metadata_schedule_338_e4235 - noise_metadata_schedule_338_e4245);
        (noise_metadata_schedule_338_e4246,)
    } else {
        (noise_variable_239,)
    }
};
            noise_variable_239 = noise_metadata_schedule_338_e4248;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_339_e4260,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_339_e4258: f64 = (noise_variable_78 - noise_variable_239);
        (noise_metadata_schedule_339_e4258,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_339_e4260;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_340_e4272,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_340_e4269: f64 = (-noise_variable_239);
        let noise_metadata_schedule_340_e4270: f64 = (noise_metadata_schedule_340_e4269).exp();
        (noise_metadata_schedule_340_e4270,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_340_e4272;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_341_e4300,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_341_e4283: f64 = (noise_variable_229 * noise_variable_229);
        let noise_metadata_schedule_341_e4287: f64 = (noise_variable_230 + noise_variable_239);
        let noise_metadata_schedule_341_e4289: f64 = (noise_metadata_schedule_341_e4287 - 1.0);
        let noise_metadata_schedule_341_e4293: f64 = (noise_variable_239 + 1.0);
        let noise_metadata_schedule_341_e4294: f64 = (noise_variable_52 * noise_metadata_schedule_341_e4293);
        let noise_metadata_schedule_341_e4295: f64 = (noise_metadata_schedule_341_e4289 - noise_metadata_schedule_341_e4294);
        let noise_metadata_schedule_341_e4296: f64 = (noise_variable_36 * noise_metadata_schedule_341_e4295);
        let noise_metadata_schedule_341_e4297: f64 = (noise_metadata_schedule_341_e4283 - noise_metadata_schedule_341_e4296);
        let noise_metadata_schedule_341_e4298: f64 = (1e-40_f64).max(noise_metadata_schedule_341_e4297);
        (noise_metadata_schedule_341_e4298,)
    } else {
        (noise_variable_234,)
    }
};
            noise_variable_234 = noise_metadata_schedule_341_e4300;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_342_e4316,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_342_e4311: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_342_e4313: f64 = (noise_metadata_schedule_342_e4311 * noise_variable_230);
        let noise_metadata_schedule_342_e4314: f64 = (1.0 - noise_metadata_schedule_342_e4313);
        (noise_metadata_schedule_342_e4314,)
    } else {
        (noise_variable_235,)
    }
};
            noise_variable_235 = noise_metadata_schedule_342_e4316;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_343_e4336,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_343_e4326: f64 = (2.0 * noise_variable_229);
        let noise_metadata_schedule_343_e4330: f64 = (1.0 - noise_variable_230);
        let noise_metadata_schedule_343_e4332: f64 = (noise_metadata_schedule_343_e4330 - noise_variable_52);
        let noise_metadata_schedule_343_e4333: f64 = (noise_variable_36 * noise_metadata_schedule_343_e4332);
        let noise_metadata_schedule_343_e4334: f64 = (noise_metadata_schedule_343_e4326 + noise_metadata_schedule_343_e4333);
        (noise_metadata_schedule_343_e4334,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_343_e4336;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_344_e4353,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_344_e4346: f64 = (noise_variable_50 - noise_variable_239);
        let noise_metadata_schedule_344_e4349: f64 = (noise_variable_234 / noise_variable_36);
        let noise_metadata_schedule_344_e4350: f64 = (noise_metadata_schedule_344_e4349).ln();
        let noise_metadata_schedule_344_e4351: f64 = (noise_metadata_schedule_344_e4346 + noise_metadata_schedule_344_e4350);
        (noise_metadata_schedule_344_e4351,)
    } else {
        (noise_variable_238,)
    }
};
            noise_variable_238 = noise_metadata_schedule_344_e4353;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_345_e4365,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_345_e4363: f64 = (noise_variable_234 + noise_variable_236);
        (noise_metadata_schedule_345_e4363,)
    } else {
        (noise_variable_255,)
    }
};
            noise_variable_255 = noise_metadata_schedule_345_e4365;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_346_e4367: f64 = (noise_variable_238).abs();
            let noise_metadata_schedule_346_e4369: f64 = if noise_metadata_schedule_346_e4367 < 1e-120 { 1.0 } else { 0.0 };
            noise_variable_257 = noise_metadata_schedule_346_e4369;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_347_e4381,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_257 != 0.0)) {
        (noise_variable_239,)
    } else {
        (noise_variable_248,)
    }
};
            noise_variable_248 = noise_metadata_schedule_347_e4381;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_348_e4408,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_257 == 0.0)) {
        let noise_metadata_schedule_348_e4394: f64 = (noise_variable_255 * noise_variable_255);
        let noise_metadata_schedule_348_e4397: f64 = (0.5 * noise_variable_236);
        let noise_metadata_schedule_348_e4399: f64 = (noise_metadata_schedule_348_e4397 * noise_variable_236);
        let noise_metadata_schedule_348_e4402: f64 = (noise_variable_234 * noise_variable_235);
        let noise_metadata_schedule_348_e4403: f64 = (noise_metadata_schedule_348_e4399 - noise_metadata_schedule_348_e4402);
        let noise_metadata_schedule_348_e4405: f64 = (noise_metadata_schedule_348_e4403 * noise_variable_238);
        let noise_metadata_schedule_348_e4406: f64 = (noise_metadata_schedule_348_e4394 + noise_metadata_schedule_348_e4405);
        (noise_metadata_schedule_348_e4406,)
    } else {
        (noise_variable_256,)
    }
};
            noise_variable_256 = noise_metadata_schedule_348_e4408;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_349_e4449,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_257 == 0.0)) {
        let noise_metadata_schedule_349_e4422: f64 = (noise_variable_234 * noise_variable_255);
        let noise_metadata_schedule_349_e4424: f64 = (noise_metadata_schedule_349_e4422 * noise_variable_238);
        let noise_metadata_schedule_349_e4428: f64 = (noise_variable_255 * noise_variable_238);
        let noise_metadata_schedule_349_e4430: f64 = (noise_metadata_schedule_349_e4428 * noise_variable_238);
        let noise_metadata_schedule_349_e4432: f64 = (noise_metadata_schedule_349_e4430 / noise_variable_256);
        let noise_metadata_schedule_349_e4434: f64 = (noise_metadata_schedule_349_e4432 * noise_variable_236);
        let noise_metadata_schedule_349_e4437: f64 = (noise_variable_236 * noise_variable_236);
        let noise_metadata_schedule_349_e4439: f64 = (noise_metadata_schedule_349_e4437 * 0.3333333333333333);
        let noise_metadata_schedule_349_e4442: f64 = (noise_variable_234 * noise_variable_235);
        let noise_metadata_schedule_349_e4443: f64 = (noise_metadata_schedule_349_e4439 - noise_metadata_schedule_349_e4442);
        let noise_metadata_schedule_349_e4444: f64 = (noise_metadata_schedule_349_e4434 * noise_metadata_schedule_349_e4443);
        let noise_metadata_schedule_349_e4445: f64 = (noise_variable_256 + noise_metadata_schedule_349_e4444);
        let noise_metadata_schedule_349_e4446: f64 = (noise_metadata_schedule_349_e4424 / noise_metadata_schedule_349_e4445);
        let noise_metadata_schedule_349_e4447: f64 = (noise_variable_239 + noise_metadata_schedule_349_e4446);
        (noise_metadata_schedule_349_e4447,)
    } else {
        (noise_variable_248,)
    }
};
            noise_variable_248 = noise_metadata_schedule_349_e4449;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_350_e4452: f64 = if noise_variable_248 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_258 = noise_metadata_schedule_350_e4452;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_351_e4465,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_258 != 0.0)) {
        let noise_metadata_schedule_351_e4463: f64 = (noise_variable_248).exp();
        (noise_metadata_schedule_351_e4463,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_351_e4465;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_352_e4479,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_258 != 0.0)) {
        let noise_metadata_schedule_352_e4477: f64 = (1.0 / noise_variable_240);
        (noise_metadata_schedule_352_e4477,)
    } else {
        (noise_variable_241,)
    }
};
            noise_variable_241 = noise_metadata_schedule_352_e4479;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_353_e4493,) = {
    if ((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_258 != 0.0)) {
        let noise_metadata_schedule_353_e4491: f64 = (noise_variable_52 * noise_variable_240);
        (noise_metadata_schedule_353_e4491,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_353_e4493;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_354_e4497: f64 = (noise_variable_50 - 230.25850929940458);
            let noise_metadata_schedule_354_e4498: f64 = if noise_variable_248 > noise_metadata_schedule_354_e4497 { 1.0 } else { 0.0 };
            noise_variable_259 = noise_metadata_schedule_354_e4498;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_355_e4516,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_258 == 0.0)) && (noise_variable_259 != 0.0)) {
        let noise_metadata_schedule_355_e4513: f64 = (noise_variable_248 - noise_variable_50);
        let noise_metadata_schedule_355_e4514: f64 = (noise_metadata_schedule_355_e4513).exp();
        (noise_metadata_schedule_355_e4514,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_355_e4516;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_356_e4533,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_258 == 0.0)) && (noise_variable_259 != 0.0)) {
        let noise_metadata_schedule_356_e4531: f64 = (noise_variable_52 / noise_variable_240);
        (noise_metadata_schedule_356_e4531,)
    } else {
        (noise_variable_241,)
    }
};
            noise_variable_241 = noise_metadata_schedule_356_e4533;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_357_e4577,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_258 == 0.0)) && (noise_variable_259 == 0.0)) {
        let noise_metadata_schedule_357_e4551: f64 = (noise_variable_50 - noise_variable_248);
        let noise_metadata_schedule_357_e4553: f64 = (noise_metadata_schedule_357_e4551 - 230.25850929940458);
        let noise_metadata_schedule_357_e4558: f64 = (noise_variable_50 - noise_variable_248);
        let noise_metadata_schedule_357_e4560: f64 = (noise_metadata_schedule_357_e4558 - 230.25850929940458);
        let noise_metadata_schedule_357_e4561: f64 = (0.5 * noise_metadata_schedule_357_e4560);
        let noise_metadata_schedule_357_e4565: f64 = (noise_variable_50 - noise_variable_248);
        let noise_metadata_schedule_357_e4567: f64 = (noise_metadata_schedule_357_e4565 - 230.25850929940458);
        let noise_metadata_schedule_357_e4569: f64 = (noise_metadata_schedule_357_e4567 * 0.3333333333333333);
        let noise_metadata_schedule_357_e4570: f64 = (1.0 + noise_metadata_schedule_357_e4569);
        let noise_metadata_schedule_357_e4571: f64 = (noise_metadata_schedule_357_e4561 * noise_metadata_schedule_357_e4570);
        let noise_metadata_schedule_357_e4572: f64 = (1.0 + noise_metadata_schedule_357_e4571);
        let noise_metadata_schedule_357_e4573: f64 = (noise_metadata_schedule_357_e4553 * noise_metadata_schedule_357_e4572);
        let noise_metadata_schedule_357_e4574: f64 = (1.0 + noise_metadata_schedule_357_e4573);
        let noise_metadata_schedule_357_e4575: f64 = (1e-100 / noise_metadata_schedule_357_e4574);
        (noise_metadata_schedule_357_e4575,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_357_e4577;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_358_e4615,) = {
    if (((((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) && (noise_variable_258 == 0.0)) && (noise_variable_259 == 0.0)) {
        let noise_metadata_schedule_358_e4595: f64 = (noise_variable_248 - 230.25850929940458);
        let noise_metadata_schedule_358_e4600: f64 = (noise_variable_248 - 230.25850929940458);
        let noise_metadata_schedule_358_e4601: f64 = (0.5 * noise_metadata_schedule_358_e4600);
        let noise_metadata_schedule_358_e4605: f64 = (noise_variable_248 - 230.25850929940458);
        let noise_metadata_schedule_358_e4607: f64 = (noise_metadata_schedule_358_e4605 * 0.3333333333333333);
        let noise_metadata_schedule_358_e4608: f64 = (1.0 + noise_metadata_schedule_358_e4607);
        let noise_metadata_schedule_358_e4609: f64 = (noise_metadata_schedule_358_e4601 * noise_metadata_schedule_358_e4608);
        let noise_metadata_schedule_358_e4610: f64 = (1.0 + noise_metadata_schedule_358_e4609);
        let noise_metadata_schedule_358_e4611: f64 = (noise_metadata_schedule_358_e4595 * noise_metadata_schedule_358_e4610);
        let noise_metadata_schedule_358_e4612: f64 = (1.0 + noise_metadata_schedule_358_e4611);
        let noise_metadata_schedule_358_e4613: f64 = (1e-100 / noise_metadata_schedule_358_e4612);
        (noise_metadata_schedule_358_e4613,)
    } else {
        (noise_variable_241,)
    }
};
            noise_variable_241 = noise_metadata_schedule_358_e4615;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_359_e4631,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_359_e4627: f64 = (noise_variable_248 * noise_variable_248);
        let noise_metadata_schedule_359_e4628: f64 = (2.0 + noise_metadata_schedule_359_e4627);
        let noise_metadata_schedule_359_e4629: f64 = (1.0 / noise_metadata_schedule_359_e4628);
        (noise_metadata_schedule_359_e4629,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_359_e4631;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_360_e4643,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_360_e4641: f64 = (noise_variable_78 - noise_variable_248);
        (noise_metadata_schedule_360_e4641,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_360_e4643;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_361_e4665,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_361_e4653: f64 = (2.0 * noise_variable_229);
        let noise_metadata_schedule_361_e4657: f64 = (1.0 - noise_variable_241);
        let noise_metadata_schedule_361_e4659: f64 = (noise_metadata_schedule_361_e4657 + noise_variable_240);
        let noise_metadata_schedule_361_e4661: f64 = (noise_metadata_schedule_361_e4659 - noise_variable_52);
        let noise_metadata_schedule_361_e4662: f64 = (noise_variable_36 * noise_metadata_schedule_361_e4661);
        let noise_metadata_schedule_361_e4663: f64 = (noise_metadata_schedule_361_e4653 + noise_metadata_schedule_361_e4662);
        (noise_metadata_schedule_361_e4663,)
    } else {
        (noise_variable_242,)
    }
};
            noise_variable_242 = noise_metadata_schedule_361_e4665;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_362_e4693,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_362_e4675: f64 = (noise_variable_229 * noise_variable_229);
        let noise_metadata_schedule_362_e4679: f64 = (noise_variable_241 + noise_variable_248);
        let noise_metadata_schedule_362_e4681: f64 = (noise_metadata_schedule_362_e4679 - 1.0);
        let noise_metadata_schedule_362_e4683: f64 = (noise_metadata_schedule_362_e4681 + noise_variable_240);
        let noise_metadata_schedule_362_e4687: f64 = (noise_variable_248 + 1.0);
        let noise_metadata_schedule_362_e4688: f64 = (noise_variable_52 * noise_metadata_schedule_362_e4687);
        let noise_metadata_schedule_362_e4689: f64 = (noise_metadata_schedule_362_e4683 - noise_metadata_schedule_362_e4688);
        let noise_metadata_schedule_362_e4690: f64 = (noise_variable_36 * noise_metadata_schedule_362_e4689);
        let noise_metadata_schedule_362_e4691: f64 = (noise_metadata_schedule_362_e4675 - noise_metadata_schedule_362_e4690);
        (noise_metadata_schedule_362_e4691,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_362_e4693;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_363_e4709,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_363_e4705: f64 = (noise_variable_241 + noise_variable_240);
        let noise_metadata_schedule_363_e4706: f64 = (noise_variable_36 * noise_metadata_schedule_363_e4705);
        let noise_metadata_schedule_363_e4707: f64 = (2.0 - noise_metadata_schedule_363_e4706);
        (noise_metadata_schedule_363_e4707,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_363_e4709;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_364_e4727,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_364_e4719: f64 = (noise_variable_242 * noise_variable_242);
        let noise_metadata_schedule_364_e4722: f64 = (2.0 * noise_variable_243);
        let noise_metadata_schedule_364_e4724: f64 = (noise_metadata_schedule_364_e4722 * noise_variable_229);
        let noise_metadata_schedule_364_e4725: f64 = (noise_metadata_schedule_364_e4719 - noise_metadata_schedule_364_e4724);
        (noise_metadata_schedule_364_e4725,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_364_e4727;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_365_e4746,) = {
    if (((noise_variable_195 != 0.0) && (noise_variable_249 == 0.0)) && (noise_variable_250 == 0.0)) {
        let noise_metadata_schedule_365_e4738: f64 = (2.0 * noise_variable_243);
        let noise_metadata_schedule_365_e4741: f64 = (noise_variable_229).sqrt();
        let noise_metadata_schedule_365_e4742: f64 = (noise_variable_242 + noise_metadata_schedule_365_e4741);
        let noise_metadata_schedule_365_e4743: f64 = (noise_metadata_schedule_365_e4738 / noise_metadata_schedule_365_e4742);
        let noise_metadata_schedule_365_e4744: f64 = (noise_variable_248 + noise_metadata_schedule_365_e4743);
        (noise_metadata_schedule_365_e4744,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_365_e4746;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_367_e4758: f64 = if ((noise_variable_78 <= 0.0) || (params.p21 < 1.0)) { 1.0 } else { 0.0 };
            noise_variable_260 = noise_metadata_schedule_367_e4758;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_369_e4767,) = {
    if (noise_variable_260 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_369_e4767;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_370_e4770: f64 = if noise_variable_79 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_261 = noise_metadata_schedule_370_e4770;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_371_e4778,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_261 != 0.0)) {
        let noise_metadata_schedule_371_e4776: f64 = (noise_variable_79).exp();
        (noise_metadata_schedule_371_e4776,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_371_e4778;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_372_e4787,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_261 != 0.0)) {
        let noise_metadata_schedule_372_e4785: f64 = (1.0 / noise_variable_83);
        (noise_metadata_schedule_372_e4785,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_372_e4787;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_373_e4796,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_261 != 0.0)) {
        let noise_metadata_schedule_373_e4794: f64 = (noise_variable_52 * noise_variable_83);
        (noise_metadata_schedule_373_e4794,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_373_e4796;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_375_e4815: f64 = (noise_variable_50 - 230.25850929940458);
            let noise_metadata_schedule_375_e4816: f64 = if noise_variable_79 > noise_metadata_schedule_375_e4815 { 1.0 } else { 0.0 };
            noise_variable_262 = noise_metadata_schedule_375_e4816;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_376_e4829,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_376_e4826: f64 = (noise_variable_79 - noise_variable_50);
        let noise_metadata_schedule_376_e4827: f64 = (noise_metadata_schedule_376_e4826).exp();
        (noise_metadata_schedule_376_e4827,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_376_e4829;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_377_e4841,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_377_e4839: f64 = (noise_variable_52 / noise_variable_83);
        (noise_metadata_schedule_377_e4839,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_377_e4841;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_380_e4929,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_262 == 0.0)) {
        let noise_metadata_schedule_380_e4909: f64 = (noise_variable_79 - 230.25850929940458);
        let noise_metadata_schedule_380_e4914: f64 = (noise_variable_79 - 230.25850929940458);
        let noise_metadata_schedule_380_e4915: f64 = (0.5 * noise_metadata_schedule_380_e4914);
        let noise_metadata_schedule_380_e4919: f64 = (noise_variable_79 - 230.25850929940458);
        let noise_metadata_schedule_380_e4921: f64 = (noise_metadata_schedule_380_e4919 * 0.3333333333333333);
        let noise_metadata_schedule_380_e4922: f64 = (1.0 + noise_metadata_schedule_380_e4921);
        let noise_metadata_schedule_380_e4923: f64 = (noise_metadata_schedule_380_e4915 * noise_metadata_schedule_380_e4922);
        let noise_metadata_schedule_380_e4924: f64 = (1.0 + noise_metadata_schedule_380_e4923);
        let noise_metadata_schedule_380_e4925: f64 = (noise_metadata_schedule_380_e4909 * noise_metadata_schedule_380_e4924);
        let noise_metadata_schedule_380_e4926: f64 = (1.0 + noise_metadata_schedule_380_e4925);
        let noise_metadata_schedule_380_e4927: f64 = (1e-100 / noise_metadata_schedule_380_e4926);
        (noise_metadata_schedule_380_e4927,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_380_e4929;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_382_e4949: f64 = if noise_variable_79 < 1e-5 { 1.0 } else { 0.0 };
            noise_variable_263 = noise_metadata_schedule_382_e4949;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_383_e4972,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_383_e4956: f64 = (0.5 * noise_variable_79);
        let noise_metadata_schedule_383_e4958: f64 = (noise_metadata_schedule_383_e4956 * noise_variable_79);
        let noise_metadata_schedule_383_e4962: f64 = (0.3333333333333333 * noise_variable_79);
        let noise_metadata_schedule_383_e4966: f64 = (0.25 * noise_variable_79);
        let noise_metadata_schedule_383_e4967: f64 = (1.0 - noise_metadata_schedule_383_e4966);
        let noise_metadata_schedule_383_e4968: f64 = (noise_metadata_schedule_383_e4962 * noise_metadata_schedule_383_e4967);
        let noise_metadata_schedule_383_e4969: f64 = (1.0 - noise_metadata_schedule_383_e4968);
        let noise_metadata_schedule_383_e4970: f64 = (noise_metadata_schedule_383_e4958 * noise_metadata_schedule_383_e4969);
        (noise_metadata_schedule_383_e4970,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_383_e4972;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_385_e5011,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_385_e5001: f64 = (0.3333333333333333 * noise_variable_79);
        let noise_metadata_schedule_385_e5005: f64 = (0.25 * noise_variable_79);
        let noise_metadata_schedule_385_e5006: f64 = (1.0 - noise_metadata_schedule_385_e5005);
        let noise_metadata_schedule_385_e5007: f64 = (noise_metadata_schedule_385_e5001 * noise_metadata_schedule_385_e5006);
        let noise_metadata_schedule_385_e5008: f64 = (1.0 - noise_metadata_schedule_385_e5007);
        let noise_metadata_schedule_385_e5009: f64 = (noise_metadata_schedule_385_e5008).sqrt();
        (noise_metadata_schedule_385_e5009,)
    } else {
        (noise_variable_6,)
    }
};
            noise_variable_6 = noise_metadata_schedule_385_e5011;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_386_e5022,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_386_e5018: f64 = (0.7071067811865475 * noise_variable_79);
        let noise_metadata_schedule_386_e5020: f64 = (noise_metadata_schedule_386_e5018 * noise_variable_6);
        (noise_metadata_schedule_386_e5020,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_386_e5022;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_387_e5034,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_263 == 0.0)) {
        let noise_metadata_schedule_387_e5030: f64 = (noise_variable_79 - 1.0);
        let noise_metadata_schedule_387_e5032: f64 = (noise_metadata_schedule_387_e5030 + noise_variable_85);
        (noise_metadata_schedule_387_e5032,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_387_e5034;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_388_e5043,) = {
    if ((noise_variable_260 == 0.0) && (noise_variable_263 == 0.0)) {
        let noise_metadata_schedule_388_e5041: f64 = (noise_variable_86).sqrt();
        (noise_metadata_schedule_388_e5041,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_388_e5043;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_392_e5073: f64 = (noise_variable_77 + (ctx.node_voltage(self.nodes[6]) - 0.0));
            let noise_metadata_schedule_392_e5075: f64 = (noise_metadata_schedule_392_e5073 * noise_variable_26);
            noise_variable_94 = noise_metadata_schedule_392_e5075;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_393_e5077: f64 = (noise_variable_94).abs();
            let noise_metadata_schedule_393_e5079: f64 = if noise_metadata_schedule_393_e5077 <= noise_variable_40 { 1.0 } else { 0.0 };
            noise_variable_281 = noise_metadata_schedule_393_e5079;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_394_e5085,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_394_e5083: f64 = (noise_variable_94 / noise_variable_43);
        (noise_metadata_schedule_394_e5083,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_394_e5085;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_395_e5088: f64 = if noise_variable_94 > noise_variable_40 { 1.0 } else { 0.0 };
            noise_variable_282 = noise_metadata_schedule_395_e5088;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_396_e5103,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_396_e5095: f64 = (noise_variable_43 * 1.25);
        let noise_metadata_schedule_396_e5097: f64 = (noise_metadata_schedule_396_e5095 / noise_variable_60);
        let noise_metadata_schedule_396_e5099: f64 = (noise_metadata_schedule_396_e5097 - 1.0);
        let noise_metadata_schedule_396_e5101: f64 = (noise_metadata_schedule_396_e5099 / noise_variable_60);
        (noise_metadata_schedule_396_e5101,)
    } else {
        (noise_variable_276,)
    }
};
            noise_variable_276 = noise_metadata_schedule_396_e5103;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_397_e5118,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_397_e5110: f64 = (noise_variable_94 / noise_variable_43);
        let noise_metadata_schedule_397_e5114: f64 = (noise_variable_276 * noise_variable_94);
        let noise_metadata_schedule_397_e5115: f64 = (1.0 + noise_metadata_schedule_397_e5114);
        let noise_metadata_schedule_397_e5116: f64 = (noise_metadata_schedule_397_e5110 * noise_metadata_schedule_397_e5115);
        (noise_metadata_schedule_397_e5116,)
    } else {
        (noise_variable_277,)
    }
};
            noise_variable_277 = noise_metadata_schedule_397_e5118;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_398_e5121: f64 = if noise_variable_277 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_283 = noise_metadata_schedule_398_e5121;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_399_e5132,) = {
    if (((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) && (noise_variable_283 != 0.0)) {
        let noise_metadata_schedule_399_e5129: f64 = (-noise_variable_277);
        let noise_metadata_schedule_399_e5130: f64 = (noise_metadata_schedule_399_e5129).exp();
        (noise_metadata_schedule_399_e5130,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_399_e5132;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_400_e5164,) = {
    if (((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) && (noise_variable_283 == 0.0)) {
        let noise_metadata_schedule_400_e5144: f64 = (noise_variable_277 - 460.51701859880916);
        let noise_metadata_schedule_400_e5149: f64 = (noise_variable_277 - 460.51701859880916);
        let noise_metadata_schedule_400_e5150: f64 = (0.5 * noise_metadata_schedule_400_e5149);
        let noise_metadata_schedule_400_e5154: f64 = (noise_variable_277 - 460.51701859880916);
        let noise_metadata_schedule_400_e5156: f64 = (noise_metadata_schedule_400_e5154 * 0.3333333333333333);
        let noise_metadata_schedule_400_e5157: f64 = (1.0 + noise_metadata_schedule_400_e5156);
        let noise_metadata_schedule_400_e5158: f64 = (noise_metadata_schedule_400_e5150 * noise_metadata_schedule_400_e5157);
        let noise_metadata_schedule_400_e5159: f64 = (1.0 + noise_metadata_schedule_400_e5158);
        let noise_metadata_schedule_400_e5160: f64 = (noise_metadata_schedule_400_e5144 * noise_metadata_schedule_400_e5159);
        let noise_metadata_schedule_400_e5161: f64 = (1.0 + noise_metadata_schedule_400_e5160);
        let noise_metadata_schedule_400_e5162: f64 = (1e-200 / noise_metadata_schedule_400_e5161);
        (noise_metadata_schedule_400_e5162,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_400_e5164;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_401_e5173,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_401_e5171: f64 = (1.0 - noise_variable_275);
        (noise_metadata_schedule_401_e5171,)
    } else {
        (noise_variable_278,)
    }
};
            noise_variable_278 = noise_metadata_schedule_401_e5173;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_402_e5195,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_402_e5181: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_402_e5182: f64 = (noise_variable_94 + noise_metadata_schedule_402_e5181);
        let noise_metadata_schedule_402_e5187: f64 = (0.25 * noise_variable_36);
        let noise_metadata_schedule_402_e5188: f64 = (noise_variable_94 + noise_metadata_schedule_402_e5187);
        let noise_metadata_schedule_402_e5190: f64 = (noise_metadata_schedule_402_e5188 - noise_variable_278);
        let noise_metadata_schedule_402_e5191: f64 = (noise_metadata_schedule_402_e5190).sqrt();
        let noise_metadata_schedule_402_e5192: f64 = (noise_variable_34 * noise_metadata_schedule_402_e5191);
        let noise_metadata_schedule_402_e5193: f64 = (noise_metadata_schedule_402_e5182 - noise_metadata_schedule_402_e5192);
        (noise_metadata_schedule_402_e5193,)
    } else {
        (noise_variable_279,)
    }
};
            noise_variable_279 = noise_metadata_schedule_402_e5195;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_403_e5198: f64 = if noise_variable_279 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_284 = noise_metadata_schedule_403_e5198;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_404_e5209,) = {
    if (((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) && (noise_variable_284 != 0.0)) {
        let noise_metadata_schedule_404_e5206: f64 = (-noise_variable_279);
        let noise_metadata_schedule_404_e5207: f64 = (noise_metadata_schedule_404_e5206).exp();
        (noise_metadata_schedule_404_e5207,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_404_e5209;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_405_e5241,) = {
    if (((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) && (noise_variable_284 == 0.0)) {
        let noise_metadata_schedule_405_e5221: f64 = (noise_variable_279 - 460.51701859880916);
        let noise_metadata_schedule_405_e5226: f64 = (noise_variable_279 - 460.51701859880916);
        let noise_metadata_schedule_405_e5227: f64 = (0.5 * noise_metadata_schedule_405_e5226);
        let noise_metadata_schedule_405_e5231: f64 = (noise_variable_279 - 460.51701859880916);
        let noise_metadata_schedule_405_e5233: f64 = (noise_metadata_schedule_405_e5231 * 0.3333333333333333);
        let noise_metadata_schedule_405_e5234: f64 = (1.0 + noise_metadata_schedule_405_e5233);
        let noise_metadata_schedule_405_e5235: f64 = (noise_metadata_schedule_405_e5227 * noise_metadata_schedule_405_e5234);
        let noise_metadata_schedule_405_e5236: f64 = (1.0 + noise_metadata_schedule_405_e5235);
        let noise_metadata_schedule_405_e5237: f64 = (noise_metadata_schedule_405_e5221 * noise_metadata_schedule_405_e5236);
        let noise_metadata_schedule_405_e5238: f64 = (1.0 + noise_metadata_schedule_405_e5237);
        let noise_metadata_schedule_405_e5239: f64 = (1e-200 / noise_metadata_schedule_405_e5238);
        (noise_metadata_schedule_405_e5239,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_405_e5241;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_406_e5254,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_406_e5249: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_406_e5251: f64 = (noise_metadata_schedule_406_e5249 * noise_variable_271);
        let noise_metadata_schedule_406_e5252: f64 = (1.0 - noise_metadata_schedule_406_e5251);
        (noise_metadata_schedule_406_e5252,)
    } else {
        (noise_variable_272,)
    }
};
            noise_variable_272 = noise_metadata_schedule_406_e5254;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_407_e5271,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_407_e5262: f64 = (noise_variable_94 - noise_variable_279);
        let noise_metadata_schedule_407_e5263: f64 = (2.0 * noise_metadata_schedule_407_e5262);
        let noise_metadata_schedule_407_e5267: f64 = (1.0 - noise_variable_271);
        let noise_metadata_schedule_407_e5268: f64 = (noise_variable_36 * noise_metadata_schedule_407_e5267);
        let noise_metadata_schedule_407_e5269: f64 = (noise_metadata_schedule_407_e5263 + noise_metadata_schedule_407_e5268);
        (noise_metadata_schedule_407_e5269,)
    } else {
        (noise_variable_273,)
    }
};
            noise_variable_273 = noise_metadata_schedule_407_e5271;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_408_e5292,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_408_e5278: f64 = (noise_variable_94 - noise_variable_279);
        let noise_metadata_schedule_408_e5281: f64 = (noise_variable_94 - noise_variable_279);
        let noise_metadata_schedule_408_e5282: f64 = (noise_metadata_schedule_408_e5278 * noise_metadata_schedule_408_e5281);
        let noise_metadata_schedule_408_e5286: f64 = (noise_variable_279 - 1.0);
        let noise_metadata_schedule_408_e5288: f64 = (noise_metadata_schedule_408_e5286 + noise_variable_271);
        let noise_metadata_schedule_408_e5289: f64 = (noise_variable_36 * noise_metadata_schedule_408_e5288);
        let noise_metadata_schedule_408_e5290: f64 = (noise_metadata_schedule_408_e5282 - noise_metadata_schedule_408_e5289);
        (noise_metadata_schedule_408_e5290,)
    } else {
        (noise_variable_274,)
    }
};
            noise_variable_274 = noise_metadata_schedule_408_e5292;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_409_e5307,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_409_e5299: f64 = (noise_variable_273 * noise_variable_273);
        let noise_metadata_schedule_409_e5302: f64 = (4.0 * noise_variable_272);
        let noise_metadata_schedule_409_e5304: f64 = (noise_metadata_schedule_409_e5302 * noise_variable_274);
        let noise_metadata_schedule_409_e5305: f64 = (noise_metadata_schedule_409_e5299 - noise_metadata_schedule_409_e5304);
        (noise_metadata_schedule_409_e5305,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_409_e5307;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_410_e5321,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_410_e5314: f64 = (2.0 * noise_variable_274);
        let noise_metadata_schedule_410_e5317: f64 = (noise_variable_275).sqrt();
        let noise_metadata_schedule_410_e5318: f64 = (noise_variable_273 + noise_metadata_schedule_410_e5317);
        let noise_metadata_schedule_410_e5319: f64 = (noise_metadata_schedule_410_e5314 / noise_metadata_schedule_410_e5318);
        (noise_metadata_schedule_410_e5319,)
    } else {
        (noise_variable_280,)
    }
};
            noise_variable_280 = noise_metadata_schedule_410_e5321;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_411_e5330,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_411_e5328: f64 = (noise_variable_279 + noise_variable_280);
        (noise_metadata_schedule_411_e5328,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_411_e5330;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_412_e5339,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_412_e5337: f64 = (-noise_variable_94);
        (noise_metadata_schedule_412_e5337,)
    } else {
        (noise_variable_264,)
    }
};
            noise_variable_264 = noise_metadata_schedule_412_e5339;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_413_e5351,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_413_e5347: f64 = (1.25 * noise_variable_264);
        let noise_metadata_schedule_413_e5349: f64 = (noise_metadata_schedule_413_e5347 / noise_variable_43);
        (noise_metadata_schedule_413_e5349,)
    } else {
        (noise_variable_265,)
    }
};
            noise_variable_265 = noise_metadata_schedule_413_e5351;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_414_e5374,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_414_e5360: f64 = (noise_variable_265 + 10.0);
        let noise_metadata_schedule_414_e5363: f64 = (noise_variable_265 - 6.0);
        let noise_metadata_schedule_414_e5366: f64 = (noise_variable_265 - 6.0);
        let noise_metadata_schedule_414_e5367: f64 = (noise_metadata_schedule_414_e5363 * noise_metadata_schedule_414_e5366);
        let noise_metadata_schedule_414_e5369: f64 = (noise_metadata_schedule_414_e5367 + 64.0);
        let noise_metadata_schedule_414_e5370: f64 = (noise_metadata_schedule_414_e5369).sqrt();
        let noise_metadata_schedule_414_e5371: f64 = (noise_metadata_schedule_414_e5360 - noise_metadata_schedule_414_e5370);
        let noise_metadata_schedule_414_e5372: f64 = (0.5 * noise_metadata_schedule_414_e5371);
        (noise_metadata_schedule_414_e5372,)
    } else {
        (noise_variable_266,)
    }
};
            noise_variable_266 = noise_metadata_schedule_414_e5374;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_415_e5394,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_415_e5382: f64 = (noise_variable_264 - noise_variable_266);
        let noise_metadata_schedule_415_e5385: f64 = (noise_variable_264 - noise_variable_266);
        let noise_metadata_schedule_415_e5386: f64 = (noise_metadata_schedule_415_e5382 * noise_metadata_schedule_415_e5385);
        let noise_metadata_schedule_415_e5390: f64 = (noise_variable_266 + 1.0);
        let noise_metadata_schedule_415_e5391: f64 = (noise_variable_36 * noise_metadata_schedule_415_e5390);
        let noise_metadata_schedule_415_e5392: f64 = (noise_metadata_schedule_415_e5386 + noise_metadata_schedule_415_e5391);
        (noise_metadata_schedule_415_e5392,)
    } else {
        (noise_variable_267,)
    }
};
            noise_variable_267 = noise_metadata_schedule_415_e5394;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_416_e5408,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_416_e5403: f64 = (noise_variable_264 - noise_variable_266);
        let noise_metadata_schedule_416_e5404: f64 = (2.0 * noise_metadata_schedule_416_e5403);
        let noise_metadata_schedule_416_e5406: f64 = (noise_metadata_schedule_416_e5404 - noise_variable_36);
        (noise_metadata_schedule_416_e5406,)
    } else {
        (noise_variable_268,)
    }
};
            noise_variable_268 = noise_metadata_schedule_416_e5408;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_417_e5421,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_417_e5416: f64 = (noise_variable_267 / noise_variable_36);
        let noise_metadata_schedule_417_e5417: f64 = (noise_metadata_schedule_417_e5416).ln();
        let noise_metadata_schedule_417_e5419: f64 = (noise_metadata_schedule_417_e5417 - noise_variable_266);
        (noise_metadata_schedule_417_e5419,)
    } else {
        (noise_variable_269,)
    }
};
            noise_variable_269 = noise_metadata_schedule_417_e5421;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_418_e5431,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_418_e5429: f64 = (noise_variable_267 + noise_variable_268);
        (noise_metadata_schedule_418_e5429,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_418_e5431;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_419_e5451,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_419_e5439: f64 = (noise_variable_285 * noise_variable_285);
        let noise_metadata_schedule_419_e5442: f64 = (0.5 * noise_variable_268);
        let noise_metadata_schedule_419_e5444: f64 = (noise_metadata_schedule_419_e5442 * noise_variable_268);
        let noise_metadata_schedule_419_e5446: f64 = (noise_metadata_schedule_419_e5444 - noise_variable_267);
        let noise_metadata_schedule_419_e5448: f64 = (noise_metadata_schedule_419_e5446 * noise_variable_269);
        let noise_metadata_schedule_419_e5449: f64 = (noise_metadata_schedule_419_e5439 + noise_metadata_schedule_419_e5448);
        (noise_metadata_schedule_419_e5449,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_419_e5451;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_420_e5485,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_420_e5460: f64 = (noise_variable_267 * noise_variable_285);
        let noise_metadata_schedule_420_e5462: f64 = (noise_metadata_schedule_420_e5460 * noise_variable_269);
        let noise_metadata_schedule_420_e5466: f64 = (noise_variable_285 * noise_variable_269);
        let noise_metadata_schedule_420_e5468: f64 = (noise_metadata_schedule_420_e5466 * noise_variable_269);
        let noise_metadata_schedule_420_e5470: f64 = (noise_metadata_schedule_420_e5468 / noise_variable_286);
        let noise_metadata_schedule_420_e5472: f64 = (noise_metadata_schedule_420_e5470 * noise_variable_268);
        let noise_metadata_schedule_420_e5475: f64 = (noise_variable_268 * noise_variable_268);
        let noise_metadata_schedule_420_e5477: f64 = (noise_metadata_schedule_420_e5475 * 0.3333333333333333);
        let noise_metadata_schedule_420_e5479: f64 = (noise_metadata_schedule_420_e5477 - noise_variable_267);
        let noise_metadata_schedule_420_e5480: f64 = (noise_metadata_schedule_420_e5472 * noise_metadata_schedule_420_e5479);
        let noise_metadata_schedule_420_e5481: f64 = (noise_variable_286 + noise_metadata_schedule_420_e5480);
        let noise_metadata_schedule_420_e5482: f64 = (noise_metadata_schedule_420_e5462 / noise_metadata_schedule_420_e5481);
        let noise_metadata_schedule_420_e5483: f64 = (noise_variable_266 + noise_metadata_schedule_420_e5482);
        (noise_metadata_schedule_420_e5483,)
    } else {
        (noise_variable_270,)
    }
};
            noise_variable_270 = noise_metadata_schedule_420_e5485;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_421_e5487: f64 = (noise_variable_270).abs();
            let noise_metadata_schedule_421_e5489: f64 = if noise_metadata_schedule_421_e5487 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_287 = noise_metadata_schedule_421_e5489;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_422_e5500,) = {
    if (((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) && (noise_variable_287 != 0.0)) {
        let noise_metadata_schedule_422_e5498: f64 = (noise_variable_270).exp();
        (noise_metadata_schedule_422_e5498,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_422_e5500;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_423_e5503: f64 = (-230.25850929940458);
            let noise_metadata_schedule_423_e5504: f64 = if noise_variable_270 < noise_metadata_schedule_423_e5503 { 1.0 } else { 0.0 };
            noise_variable_288 = noise_metadata_schedule_423_e5504;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_424_e5542,) = {
    if ((((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) && (noise_variable_287 == 0.0)) && (noise_variable_288 != 0.0)) {
        let noise_metadata_schedule_424_e5518: f64 = (-230.25850929940458);
        let noise_metadata_schedule_424_e5520: f64 = (noise_metadata_schedule_424_e5518 - noise_variable_270);
        let noise_metadata_schedule_424_e5524: f64 = (-230.25850929940458);
        let noise_metadata_schedule_424_e5526: f64 = (noise_metadata_schedule_424_e5524 - noise_variable_270);
        let noise_metadata_schedule_424_e5527: f64 = (0.5 * noise_metadata_schedule_424_e5526);
        let noise_metadata_schedule_424_e5530: f64 = (-230.25850929940458);
        let noise_metadata_schedule_424_e5532: f64 = (noise_metadata_schedule_424_e5530 - noise_variable_270);
        let noise_metadata_schedule_424_e5534: f64 = (noise_metadata_schedule_424_e5532 * 0.3333333333333333);
        let noise_metadata_schedule_424_e5535: f64 = (1.0 + noise_metadata_schedule_424_e5534);
        let noise_metadata_schedule_424_e5536: f64 = (noise_metadata_schedule_424_e5527 * noise_metadata_schedule_424_e5535);
        let noise_metadata_schedule_424_e5537: f64 = (1.0 + noise_metadata_schedule_424_e5536);
        let noise_metadata_schedule_424_e5538: f64 = (noise_metadata_schedule_424_e5520 * noise_metadata_schedule_424_e5537);
        let noise_metadata_schedule_424_e5539: f64 = (1.0 + noise_metadata_schedule_424_e5538);
        let noise_metadata_schedule_424_e5540: f64 = (1e-100 / noise_metadata_schedule_424_e5539);
        (noise_metadata_schedule_424_e5540,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_424_e5542;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_425_e5578,) = {
    if ((((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) && (noise_variable_287 == 0.0)) && (noise_variable_288 == 0.0)) {
        let noise_metadata_schedule_425_e5558: f64 = (noise_variable_270 - 230.25850929940458);
        let noise_metadata_schedule_425_e5563: f64 = (noise_variable_270 - 230.25850929940458);
        let noise_metadata_schedule_425_e5564: f64 = (0.5 * noise_metadata_schedule_425_e5563);
        let noise_metadata_schedule_425_e5568: f64 = (noise_variable_270 - 230.25850929940458);
        let noise_metadata_schedule_425_e5570: f64 = (noise_metadata_schedule_425_e5568 * 0.3333333333333333);
        let noise_metadata_schedule_425_e5571: f64 = (1.0 + noise_metadata_schedule_425_e5570);
        let noise_metadata_schedule_425_e5572: f64 = (noise_metadata_schedule_425_e5564 * noise_metadata_schedule_425_e5571);
        let noise_metadata_schedule_425_e5573: f64 = (1.0 + noise_metadata_schedule_425_e5572);
        let noise_metadata_schedule_425_e5574: f64 = (noise_metadata_schedule_425_e5558 * noise_metadata_schedule_425_e5573);
        let noise_metadata_schedule_425_e5575: f64 = (1.0 + noise_metadata_schedule_425_e5574);
        let noise_metadata_schedule_425_e5576: f64 = (1e100 * noise_metadata_schedule_425_e5575);
        (noise_metadata_schedule_425_e5576,)
    } else {
        (noise_variable_271,)
    }
};
            noise_variable_271 = noise_metadata_schedule_425_e5578;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_426_e5592,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_426_e5587: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_426_e5589: f64 = (noise_metadata_schedule_426_e5587 * noise_variable_271);
        let noise_metadata_schedule_426_e5590: f64 = (1.0 - noise_metadata_schedule_426_e5589);
        (noise_metadata_schedule_426_e5590,)
    } else {
        (noise_variable_272,)
    }
};
            noise_variable_272 = noise_metadata_schedule_426_e5592;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_427_e5610,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_427_e5601: f64 = (noise_variable_264 - noise_variable_270);
        let noise_metadata_schedule_427_e5602: f64 = (2.0 * noise_metadata_schedule_427_e5601);
        let noise_metadata_schedule_427_e5606: f64 = (noise_variable_271 - 1.0);
        let noise_metadata_schedule_427_e5607: f64 = (noise_variable_36 * noise_metadata_schedule_427_e5606);
        let noise_metadata_schedule_427_e5608: f64 = (noise_metadata_schedule_427_e5602 + noise_metadata_schedule_427_e5607);
        (noise_metadata_schedule_427_e5608,)
    } else {
        (noise_variable_273,)
    }
};
            noise_variable_273 = noise_metadata_schedule_427_e5610;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_428_e5632,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_428_e5618: f64 = (noise_variable_264 - noise_variable_270);
        let noise_metadata_schedule_428_e5621: f64 = (noise_variable_264 - noise_variable_270);
        let noise_metadata_schedule_428_e5622: f64 = (noise_metadata_schedule_428_e5618 * noise_metadata_schedule_428_e5621);
        let noise_metadata_schedule_428_e5626: f64 = (noise_variable_270 + 1.0);
        let noise_metadata_schedule_428_e5628: f64 = (noise_metadata_schedule_428_e5626 - noise_variable_271);
        let noise_metadata_schedule_428_e5629: f64 = (noise_variable_36 * noise_metadata_schedule_428_e5628);
        let noise_metadata_schedule_428_e5630: f64 = (noise_metadata_schedule_428_e5622 + noise_metadata_schedule_428_e5629);
        (noise_metadata_schedule_428_e5630,)
    } else {
        (noise_variable_274,)
    }
};
            noise_variable_274 = noise_metadata_schedule_428_e5632;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_429_e5648,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_429_e5640: f64 = (noise_variable_273 * noise_variable_273);
        let noise_metadata_schedule_429_e5643: f64 = (4.0 * noise_variable_272);
        let noise_metadata_schedule_429_e5645: f64 = (noise_metadata_schedule_429_e5643 * noise_variable_274);
        let noise_metadata_schedule_429_e5646: f64 = (noise_metadata_schedule_429_e5640 - noise_metadata_schedule_429_e5645);
        (noise_metadata_schedule_429_e5646,)
    } else {
        (noise_variable_275,)
    }
};
            noise_variable_275 = noise_metadata_schedule_429_e5648;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_430_e5663,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_430_e5656: f64 = (2.0 * noise_variable_274);
        let noise_metadata_schedule_430_e5659: f64 = (noise_variable_275).sqrt();
        let noise_metadata_schedule_430_e5660: f64 = (noise_variable_273 + noise_metadata_schedule_430_e5659);
        let noise_metadata_schedule_430_e5661: f64 = (noise_metadata_schedule_430_e5656 / noise_metadata_schedule_430_e5660);
        (noise_metadata_schedule_430_e5661,)
    } else {
        (noise_variable_278,)
    }
};
            noise_variable_278 = noise_metadata_schedule_430_e5663;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_431_e5674,) = {
    if ((noise_variable_281 == 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_431_e5671: f64 = (noise_variable_270 + noise_variable_278);
        let noise_metadata_schedule_431_e5672: f64 = (-noise_metadata_schedule_431_e5671);
        (noise_metadata_schedule_431_e5672,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_431_e5674;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_433_e5680: f64 = if params.p29 < 1e27 { 1.0 } else { 0.0 };
            noise_variable_289 = noise_metadata_schedule_433_e5680;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_434_e5695,) = {
    if (noise_variable_289 != 0.0) {
        let noise_metadata_schedule_434_e5683: f64 = (-params.p17);
        let noise_metadata_schedule_434_e5685: f64 = (noise_metadata_schedule_434_e5683 * params.p18);
        let noise_metadata_schedule_434_e5689: f64 = (noise_variable_95 * noise_variable_25);
        let noise_metadata_schedule_434_e5690: f64 = (noise_variable_77 - noise_metadata_schedule_434_e5689);
        let noise_metadata_schedule_434_e5691: f64 = (noise_metadata_schedule_434_e5685 * noise_metadata_schedule_434_e5690);
        let noise_metadata_schedule_434_e5693: f64 = (noise_metadata_schedule_434_e5691 * noise_variable_26);
        (noise_metadata_schedule_434_e5693,)
    } else {
        (noise_variable_97,)
    }
};
            noise_variable_97 = noise_metadata_schedule_434_e5695;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_435_e5697: f64 = (noise_variable_97).abs();
            let noise_metadata_schedule_435_e5699: f64 = if noise_metadata_schedule_435_e5697 <= noise_variable_41 { 1.0 } else { 0.0 };
            noise_variable_311 = noise_metadata_schedule_435_e5699;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_436_e5711,) = {
    if ((noise_variable_289 != 0.0) && (noise_variable_311 != 0.0)) {
        let noise_metadata_schedule_436_e5705: f64 = (noise_variable_46 * noise_variable_46);
        let noise_metadata_schedule_436_e5707: f64 = (noise_metadata_schedule_436_e5705 * 0.1666666666666667);
        let noise_metadata_schedule_436_e5709: f64 = (noise_metadata_schedule_436_e5707 * 0.7071067811865475);
        (noise_metadata_schedule_436_e5709,)
    } else {
        (noise_variable_292,)
    }
};
            noise_variable_292 = noise_metadata_schedule_436_e5711;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_437_e5731,) = {
    if ((noise_variable_289 != 0.0) && (noise_variable_311 != 0.0)) {
        let noise_metadata_schedule_437_e5717: f64 = (noise_variable_97 * noise_variable_46);
        let noise_metadata_schedule_437_e5722: f64 = (1.0 - noise_variable_53);
        let noise_metadata_schedule_437_e5723: f64 = (noise_variable_97 * noise_metadata_schedule_437_e5722);
        let noise_metadata_schedule_437_e5725: f64 = (noise_metadata_schedule_437_e5723 * noise_variable_35);
        let noise_metadata_schedule_437_e5727: f64 = (noise_metadata_schedule_437_e5725 * noise_variable_292);
        let noise_metadata_schedule_437_e5728: f64 = (1.0 + noise_metadata_schedule_437_e5727);
        let noise_metadata_schedule_437_e5729: f64 = (noise_metadata_schedule_437_e5717 * noise_metadata_schedule_437_e5728);
        (noise_metadata_schedule_437_e5729,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_437_e5731;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_438_e5734: f64 = (-noise_variable_41);
            let noise_metadata_schedule_438_e5735: f64 = if noise_variable_97 < noise_metadata_schedule_438_e5734 { 1.0 } else { 0.0 };
            noise_variable_312 = noise_metadata_schedule_438_e5735;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_439_e5745,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_439_e5743: f64 = (-noise_variable_97);
        (noise_metadata_schedule_439_e5743,)
    } else {
        (noise_variable_293,)
    }
};
            noise_variable_293 = noise_metadata_schedule_439_e5745;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_440_e5758,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_440_e5754: f64 = (1.25 * noise_variable_293);
        let noise_metadata_schedule_440_e5756: f64 = (noise_metadata_schedule_440_e5754 * noise_variable_46);
        (noise_metadata_schedule_440_e5756,)
    } else {
        (noise_variable_294,)
    }
};
            noise_variable_294 = noise_metadata_schedule_440_e5758;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_441_e5782,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_441_e5768: f64 = (noise_variable_294 + 10.0);
        let noise_metadata_schedule_441_e5771: f64 = (noise_variable_294 - 6.0);
        let noise_metadata_schedule_441_e5774: f64 = (noise_variable_294 - 6.0);
        let noise_metadata_schedule_441_e5775: f64 = (noise_metadata_schedule_441_e5771 * noise_metadata_schedule_441_e5774);
        let noise_metadata_schedule_441_e5777: f64 = (noise_metadata_schedule_441_e5775 + 64.0);
        let noise_metadata_schedule_441_e5778: f64 = (noise_metadata_schedule_441_e5777).sqrt();
        let noise_metadata_schedule_441_e5779: f64 = (noise_metadata_schedule_441_e5768 - noise_metadata_schedule_441_e5778);
        let noise_metadata_schedule_441_e5780: f64 = (0.5 * noise_metadata_schedule_441_e5779);
        (noise_metadata_schedule_441_e5780,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_441_e5782;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_442_e5793,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_442_e5791: f64 = (noise_variable_293 - noise_variable_301);
        (noise_metadata_schedule_442_e5791,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_442_e5793;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_443_e5810,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_443_e5802: f64 = (noise_variable_291 * noise_variable_291);
        let noise_metadata_schedule_443_e5806: f64 = (noise_variable_301 + 1.0);
        let noise_metadata_schedule_443_e5807: f64 = (noise_variable_38 * noise_metadata_schedule_443_e5806);
        let noise_metadata_schedule_443_e5808: f64 = (noise_metadata_schedule_443_e5802 + noise_metadata_schedule_443_e5807);
        (noise_metadata_schedule_443_e5808,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_443_e5810;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_444_e5823,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_444_e5819: f64 = (2.0 * noise_variable_291);
        let noise_metadata_schedule_444_e5821: f64 = (noise_metadata_schedule_444_e5819 - noise_variable_38);
        (noise_metadata_schedule_444_e5821,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_444_e5823;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_445_e5838,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_445_e5831: f64 = (-noise_variable_301);
        let noise_metadata_schedule_445_e5834: f64 = (noise_variable_296 * noise_variable_39);
        let noise_metadata_schedule_445_e5835: f64 = (noise_metadata_schedule_445_e5834).ln();
        let noise_metadata_schedule_445_e5836: f64 = (noise_metadata_schedule_445_e5831 + noise_metadata_schedule_445_e5835);
        (noise_metadata_schedule_445_e5836,)
    } else {
        (noise_variable_300,)
    }
};
            noise_variable_300 = noise_metadata_schedule_445_e5838;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_446_e5849,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_446_e5847: f64 = (noise_variable_296 + noise_variable_298);
        (noise_metadata_schedule_446_e5847,)
    } else {
        (noise_variable_313,)
    }
};
            noise_variable_313 = noise_metadata_schedule_446_e5849;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_447_e5870,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_447_e5858: f64 = (noise_variable_313 * noise_variable_313);
        let noise_metadata_schedule_447_e5861: f64 = (0.5 * noise_variable_298);
        let noise_metadata_schedule_447_e5863: f64 = (noise_metadata_schedule_447_e5861 * noise_variable_298);
        let noise_metadata_schedule_447_e5865: f64 = (noise_metadata_schedule_447_e5863 - noise_variable_296);
        let noise_metadata_schedule_447_e5867: f64 = (noise_metadata_schedule_447_e5865 * noise_variable_300);
        let noise_metadata_schedule_447_e5868: f64 = (noise_metadata_schedule_447_e5858 + noise_metadata_schedule_447_e5867);
        (noise_metadata_schedule_447_e5868,)
    } else {
        (noise_variable_314,)
    }
};
            noise_variable_314 = noise_metadata_schedule_447_e5870;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_448_e5905,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_448_e5880: f64 = (noise_variable_296 * noise_variable_313);
        let noise_metadata_schedule_448_e5882: f64 = (noise_metadata_schedule_448_e5880 * noise_variable_300);
        let noise_metadata_schedule_448_e5886: f64 = (noise_variable_313 * noise_variable_300);
        let noise_metadata_schedule_448_e5888: f64 = (noise_metadata_schedule_448_e5886 * noise_variable_300);
        let noise_metadata_schedule_448_e5890: f64 = (noise_metadata_schedule_448_e5888 / noise_variable_314);
        let noise_metadata_schedule_448_e5892: f64 = (noise_metadata_schedule_448_e5890 * noise_variable_298);
        let noise_metadata_schedule_448_e5895: f64 = (noise_variable_298 * noise_variable_298);
        let noise_metadata_schedule_448_e5897: f64 = (noise_metadata_schedule_448_e5895 * 0.3333333333333333);
        let noise_metadata_schedule_448_e5899: f64 = (noise_metadata_schedule_448_e5897 - noise_variable_296);
        let noise_metadata_schedule_448_e5900: f64 = (noise_metadata_schedule_448_e5892 * noise_metadata_schedule_448_e5899);
        let noise_metadata_schedule_448_e5901: f64 = (noise_variable_314 + noise_metadata_schedule_448_e5900);
        let noise_metadata_schedule_448_e5902: f64 = (noise_metadata_schedule_448_e5882 / noise_metadata_schedule_448_e5901);
        let noise_metadata_schedule_448_e5903: f64 = (noise_variable_301 + noise_metadata_schedule_448_e5902);
        (noise_metadata_schedule_448_e5903,)
    } else {
        (noise_variable_295,)
    }
};
            noise_variable_295 = noise_metadata_schedule_448_e5905;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_449_e5908: f64 = if noise_variable_295 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_315 = noise_metadata_schedule_449_e5908;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_450_e5920,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_450_e5918: f64 = (noise_variable_295).exp();
        (noise_metadata_schedule_450_e5918,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_450_e5920;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_451_e5954,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) && (noise_variable_315 == 0.0)) {
        let noise_metadata_schedule_451_e5934: f64 = (noise_variable_295 - 230.25850929940458);
        let noise_metadata_schedule_451_e5939: f64 = (noise_variable_295 - 230.25850929940458);
        let noise_metadata_schedule_451_e5940: f64 = (0.5 * noise_metadata_schedule_451_e5939);
        let noise_metadata_schedule_451_e5944: f64 = (noise_variable_295 - 230.25850929940458);
        let noise_metadata_schedule_451_e5946: f64 = (noise_metadata_schedule_451_e5944 * 0.3333333333333333);
        let noise_metadata_schedule_451_e5947: f64 = (1.0 + noise_metadata_schedule_451_e5946);
        let noise_metadata_schedule_451_e5948: f64 = (noise_metadata_schedule_451_e5940 * noise_metadata_schedule_451_e5947);
        let noise_metadata_schedule_451_e5949: f64 = (1.0 + noise_metadata_schedule_451_e5948);
        let noise_metadata_schedule_451_e5950: f64 = (noise_metadata_schedule_451_e5934 * noise_metadata_schedule_451_e5949);
        let noise_metadata_schedule_451_e5951: f64 = (1.0 + noise_metadata_schedule_451_e5950);
        let noise_metadata_schedule_451_e5952: f64 = (1e100 * noise_metadata_schedule_451_e5951);
        (noise_metadata_schedule_451_e5952,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_451_e5954;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_452_e5965,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_452_e5963: f64 = (1.0 / noise_variable_302);
        (noise_metadata_schedule_452_e5963,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_452_e5965;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_453_e5980,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_453_e5976: f64 = (noise_variable_295 * noise_variable_295);
        let noise_metadata_schedule_453_e5977: f64 = (2.0 + noise_metadata_schedule_453_e5976);
        let noise_metadata_schedule_453_e5978: f64 = (1.0 / noise_metadata_schedule_453_e5977);
        (noise_metadata_schedule_453_e5978,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_453_e5980;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_454_e5991,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_454_e5989: f64 = (noise_variable_293 - noise_variable_295);
        (noise_metadata_schedule_454_e5989,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_454_e5991;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_455_e6002,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_455_e6000: f64 = (noise_variable_53 * noise_variable_303);
        (noise_metadata_schedule_455_e6000,)
    } else {
        (noise_variable_292,)
    }
};
            noise_variable_292 = noise_metadata_schedule_455_e6002;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_456_e6023,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_456_e6011: f64 = (2.0 * noise_variable_291);
        let noise_metadata_schedule_456_e6015: f64 = (noise_variable_302 - 1.0);
        let noise_metadata_schedule_456_e6017: f64 = (noise_metadata_schedule_456_e6015 - noise_variable_292);
        let noise_metadata_schedule_456_e6019: f64 = (noise_metadata_schedule_456_e6017 + noise_variable_53);
        let noise_metadata_schedule_456_e6020: f64 = (noise_variable_38 * noise_metadata_schedule_456_e6019);
        let noise_metadata_schedule_456_e6021: f64 = (noise_metadata_schedule_456_e6011 + noise_metadata_schedule_456_e6020);
        (noise_metadata_schedule_456_e6021,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_456_e6023;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_457_e6050,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_457_e6032: f64 = (noise_variable_291 * noise_variable_291);
        let noise_metadata_schedule_457_e6036: f64 = (noise_variable_302 - noise_variable_295);
        let noise_metadata_schedule_457_e6038: f64 = (noise_metadata_schedule_457_e6036 - 1.0);
        let noise_metadata_schedule_457_e6040: f64 = (noise_metadata_schedule_457_e6038 + noise_variable_292);
        let noise_metadata_schedule_457_e6044: f64 = (noise_variable_295 - 1.0);
        let noise_metadata_schedule_457_e6045: f64 = (noise_variable_53 * noise_metadata_schedule_457_e6044);
        let noise_metadata_schedule_457_e6046: f64 = (noise_metadata_schedule_457_e6040 + noise_metadata_schedule_457_e6045);
        let noise_metadata_schedule_457_e6047: f64 = (noise_variable_38 * noise_metadata_schedule_457_e6046);
        let noise_metadata_schedule_457_e6048: f64 = (noise_metadata_schedule_457_e6032 - noise_metadata_schedule_457_e6047);
        (noise_metadata_schedule_457_e6048,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_457_e6050;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_458_e6065,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_458_e6061: f64 = (noise_variable_302 + noise_variable_292);
        let noise_metadata_schedule_458_e6062: f64 = (noise_variable_38 * noise_metadata_schedule_458_e6061);
        let noise_metadata_schedule_458_e6063: f64 = (2.0 - noise_metadata_schedule_458_e6062);
        (noise_metadata_schedule_458_e6063,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_458_e6065;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_459_e6082,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_459_e6074: f64 = (noise_variable_304 * noise_variable_304);
        let noise_metadata_schedule_459_e6077: f64 = (2.0 * noise_variable_305);
        let noise_metadata_schedule_459_e6079: f64 = (noise_metadata_schedule_459_e6077 * noise_variable_291);
        let noise_metadata_schedule_459_e6080: f64 = (noise_metadata_schedule_459_e6074 - noise_metadata_schedule_459_e6079);
        (noise_metadata_schedule_459_e6080,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_459_e6082;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_460_e6101,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_460_e6090: f64 = (-noise_variable_295);
        let noise_metadata_schedule_460_e6093: f64 = (2.0 * noise_variable_305);
        let noise_metadata_schedule_460_e6096: f64 = (noise_variable_291).sqrt();
        let noise_metadata_schedule_460_e6097: f64 = (noise_variable_304 + noise_metadata_schedule_460_e6096);
        let noise_metadata_schedule_460_e6098: f64 = (noise_metadata_schedule_460_e6093 / noise_metadata_schedule_460_e6097);
        let noise_metadata_schedule_460_e6099: f64 = (noise_metadata_schedule_460_e6090 - noise_metadata_schedule_460_e6098);
        (noise_metadata_schedule_460_e6099,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_460_e6101;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_461_e6117,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_461_e6113: f64 = (noise_variable_35 * 0.7324648775608221);
        let noise_metadata_schedule_461_e6114: f64 = (1.25 + noise_metadata_schedule_461_e6113);
        let noise_metadata_schedule_461_e6115: f64 = (1.0 / noise_metadata_schedule_461_e6114);
        (noise_metadata_schedule_461_e6115,)
    } else {
        (noise_variable_290,)
    }
};
            noise_variable_290 = noise_metadata_schedule_461_e6117;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_462_e6135,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_462_e6127: f64 = (noise_variable_45 * 1.25);
        let noise_metadata_schedule_462_e6129: f64 = (noise_metadata_schedule_462_e6127 * noise_variable_290);
        let noise_metadata_schedule_462_e6131: f64 = (noise_metadata_schedule_462_e6129 - 1.0);
        let noise_metadata_schedule_462_e6133: f64 = (noise_metadata_schedule_462_e6131 * noise_variable_290);
        (noise_metadata_schedule_462_e6133,)
    } else {
        (noise_variable_306,)
    }
};
            noise_variable_306 = noise_metadata_schedule_462_e6135;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_463_e6153,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_463_e6145: f64 = (noise_variable_97 * noise_variable_46);
        let noise_metadata_schedule_463_e6149: f64 = (noise_variable_306 * noise_variable_97);
        let noise_metadata_schedule_463_e6150: f64 = (1.0 + noise_metadata_schedule_463_e6149);
        let noise_metadata_schedule_463_e6151: f64 = (noise_metadata_schedule_463_e6145 * noise_metadata_schedule_463_e6150);
        (noise_metadata_schedule_463_e6151,)
    } else {
        (noise_variable_309,)
    }
};
            noise_variable_309 = noise_metadata_schedule_463_e6153;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_464_e6155: f64 = (-noise_variable_309);
            let noise_metadata_schedule_464_e6157: f64 = (-230.25850929940458);
            let noise_metadata_schedule_464_e6158: f64 = if noise_metadata_schedule_464_e6155 > noise_metadata_schedule_464_e6157 { 1.0 } else { 0.0 };
            noise_variable_316 = noise_metadata_schedule_464_e6158;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_465_e6172,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_316 != 0.0)) {
        let noise_metadata_schedule_465_e6169: f64 = (-noise_variable_309);
        let noise_metadata_schedule_465_e6170: f64 = (noise_metadata_schedule_465_e6169).exp();
        (noise_metadata_schedule_465_e6170,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_465_e6172;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_466_e6213,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_316 == 0.0)) {
        let noise_metadata_schedule_466_e6186: f64 = (-230.25850929940458);
        let noise_metadata_schedule_466_e6188: f64 = (-noise_variable_309);
        let noise_metadata_schedule_466_e6189: f64 = (noise_metadata_schedule_466_e6186 - noise_metadata_schedule_466_e6188);
        let noise_metadata_schedule_466_e6193: f64 = (-230.25850929940458);
        let noise_metadata_schedule_466_e6195: f64 = (-noise_variable_309);
        let noise_metadata_schedule_466_e6196: f64 = (noise_metadata_schedule_466_e6193 - noise_metadata_schedule_466_e6195);
        let noise_metadata_schedule_466_e6197: f64 = (0.5 * noise_metadata_schedule_466_e6196);
        let noise_metadata_schedule_466_e6200: f64 = (-230.25850929940458);
        let noise_metadata_schedule_466_e6202: f64 = (-noise_variable_309);
        let noise_metadata_schedule_466_e6203: f64 = (noise_metadata_schedule_466_e6200 - noise_metadata_schedule_466_e6202);
        let noise_metadata_schedule_466_e6205: f64 = (noise_metadata_schedule_466_e6203 * 0.3333333333333333);
        let noise_metadata_schedule_466_e6206: f64 = (1.0 + noise_metadata_schedule_466_e6205);
        let noise_metadata_schedule_466_e6207: f64 = (noise_metadata_schedule_466_e6197 * noise_metadata_schedule_466_e6206);
        let noise_metadata_schedule_466_e6208: f64 = (1.0 + noise_metadata_schedule_466_e6207);
        let noise_metadata_schedule_466_e6209: f64 = (noise_metadata_schedule_466_e6189 * noise_metadata_schedule_466_e6208);
        let noise_metadata_schedule_466_e6210: f64 = (1.0 + noise_metadata_schedule_466_e6209);
        let noise_metadata_schedule_466_e6211: f64 = (1e-100 / noise_metadata_schedule_466_e6210);
        (noise_metadata_schedule_466_e6211,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_466_e6213;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_467_e6225,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_467_e6223: f64 = (1.0 - noise_variable_291);
        (noise_metadata_schedule_467_e6223,)
    } else {
        (noise_variable_308,)
    }
};
            noise_variable_308 = noise_metadata_schedule_467_e6225;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_468_e6250,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_468_e6236: f64 = (noise_variable_38 * 0.5);
        let noise_metadata_schedule_468_e6237: f64 = (noise_variable_97 + noise_metadata_schedule_468_e6236);
        let noise_metadata_schedule_468_e6242: f64 = (noise_variable_38 * 0.25);
        let noise_metadata_schedule_468_e6243: f64 = (noise_variable_97 + noise_metadata_schedule_468_e6242);
        let noise_metadata_schedule_468_e6245: f64 = (noise_metadata_schedule_468_e6243 - noise_variable_308);
        let noise_metadata_schedule_468_e6246: f64 = (noise_metadata_schedule_468_e6245).sqrt();
        let noise_metadata_schedule_468_e6247: f64 = (noise_variable_35 * noise_metadata_schedule_468_e6246);
        let noise_metadata_schedule_468_e6248: f64 = (noise_metadata_schedule_468_e6237 - noise_metadata_schedule_468_e6247);
        (noise_metadata_schedule_468_e6248,)
    } else {
        (noise_variable_307,)
    }
};
            noise_variable_307 = noise_metadata_schedule_468_e6250;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_469_e6262,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_469_e6260: f64 = (noise_variable_51 + 3.0);
        (noise_metadata_schedule_469_e6260,)
    } else {
        (noise_variable_299,)
    }
};
            noise_variable_299 = noise_metadata_schedule_469_e6262;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_470_e6344,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_470_e6272: f64 = (noise_variable_299 - noise_variable_307);
        let (noise_metadata_schedule_470_e6331,) = {
            if (noise_metadata_schedule_470_e6272 > 1e-16) {
                let noise_metadata_schedule_470_e6279: f64 = (noise_variable_299 - noise_variable_307);
                let noise_metadata_schedule_470_e6282: f64 = (noise_variable_299 - noise_variable_307);
                let noise_metadata_schedule_470_e6285: f64 = (noise_variable_299 - noise_variable_307);
                let noise_metadata_schedule_470_e6286: f64 = (noise_metadata_schedule_470_e6282 * noise_metadata_schedule_470_e6285);
                let noise_metadata_schedule_470_e6288: f64 = (noise_metadata_schedule_470_e6286 + 5.0);
                let noise_metadata_schedule_470_e6289: f64 = (noise_metadata_schedule_470_e6288).sqrt();
                let noise_metadata_schedule_470_e6290: f64 = (noise_metadata_schedule_470_e6279 + noise_metadata_schedule_470_e6289);
                let noise_metadata_schedule_470_e6291: f64 = (0.5 * noise_metadata_schedule_470_e6290);
                let noise_metadata_schedule_470_e6292: f64 = (noise_variable_299 - noise_metadata_schedule_470_e6291);
                (noise_metadata_schedule_470_e6292,)
            } else {
                let noise_metadata_schedule_470_e6295: f64 = (noise_variable_307 - noise_variable_299);
                let (noise_metadata_schedule_470_e6330,) = {
                    if (noise_metadata_schedule_470_e6295 > 1e-16) {
                        let noise_metadata_schedule_470_e6301: f64 = (0.5 * 5.0);
                        let noise_metadata_schedule_470_e6304: f64 = (noise_variable_307 - noise_variable_299);
                        let noise_metadata_schedule_470_e6307: f64 = (noise_variable_307 - noise_variable_299);
                        let noise_metadata_schedule_470_e6310: f64 = (noise_variable_307 - noise_variable_299);
                        let noise_metadata_schedule_470_e6311: f64 = (noise_metadata_schedule_470_e6307 * noise_metadata_schedule_470_e6310);
                        let noise_metadata_schedule_470_e6313: f64 = (noise_metadata_schedule_470_e6311 + 5.0);
                        let noise_metadata_schedule_470_e6314: f64 = (noise_metadata_schedule_470_e6313).sqrt();
                        let noise_metadata_schedule_470_e6315: f64 = (noise_metadata_schedule_470_e6304 + noise_metadata_schedule_470_e6314);
                        let noise_metadata_schedule_470_e6316: f64 = (noise_metadata_schedule_470_e6301 / noise_metadata_schedule_470_e6315);
                        let noise_metadata_schedule_470_e6317: f64 = (noise_variable_299 - noise_metadata_schedule_470_e6316);
                        (noise_metadata_schedule_470_e6317,)
                    } else {
                        let noise_metadata_schedule_470_e6322: f64 = (noise_variable_299 - noise_variable_307);
                        let noise_metadata_schedule_470_e6325: f64 = (1e-32 + 5.0);
                        let noise_metadata_schedule_470_e6326: f64 = (noise_metadata_schedule_470_e6325).sqrt();
                        let noise_metadata_schedule_470_e6327: f64 = (noise_metadata_schedule_470_e6322 + noise_metadata_schedule_470_e6326);
                        let noise_metadata_schedule_470_e6328: f64 = (0.5 * noise_metadata_schedule_470_e6327);
                        let noise_metadata_schedule_470_e6329: f64 = (noise_variable_299 - noise_metadata_schedule_470_e6328);
                        (noise_metadata_schedule_470_e6329,)
                    }
                };
                (noise_metadata_schedule_470_e6330,)
            }
        };
        let noise_metadata_schedule_470_e6336: f64 = (noise_variable_299 * noise_variable_299);
        let noise_metadata_schedule_470_e6338: f64 = (noise_metadata_schedule_470_e6336 + 5.0);
        let noise_metadata_schedule_470_e6339: f64 = (noise_metadata_schedule_470_e6338).sqrt();
        let noise_metadata_schedule_470_e6340: f64 = (noise_variable_299 - noise_metadata_schedule_470_e6339);
        let noise_metadata_schedule_470_e6341: f64 = (0.5 * noise_metadata_schedule_470_e6340);
        let noise_metadata_schedule_470_e6342: f64 = (noise_metadata_schedule_470_e6331 - noise_metadata_schedule_470_e6341);
        (noise_metadata_schedule_470_e6342,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_470_e6344;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_471_e6356,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_471_e6354: f64 = (noise_variable_97 - noise_variable_301);
        (noise_metadata_schedule_471_e6354,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_471_e6356;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_472_e6368,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_472_e6365: f64 = (-noise_variable_301);
        let noise_metadata_schedule_472_e6366: f64 = (noise_metadata_schedule_472_e6365).exp();
        (noise_metadata_schedule_472_e6366,)
    } else {
        (noise_variable_292,)
    }
};
            noise_variable_292 = noise_metadata_schedule_472_e6368;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_473_e6396,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_473_e6379: f64 = (noise_variable_291 * noise_variable_291);
        let noise_metadata_schedule_473_e6383: f64 = (noise_variable_292 + noise_variable_301);
        let noise_metadata_schedule_473_e6385: f64 = (noise_metadata_schedule_473_e6383 - 1.0);
        let noise_metadata_schedule_473_e6389: f64 = (noise_variable_301 + 1.0);
        let noise_metadata_schedule_473_e6390: f64 = (noise_variable_53 * noise_metadata_schedule_473_e6389);
        let noise_metadata_schedule_473_e6391: f64 = (noise_metadata_schedule_473_e6385 - noise_metadata_schedule_473_e6390);
        let noise_metadata_schedule_473_e6392: f64 = (noise_variable_38 * noise_metadata_schedule_473_e6391);
        let noise_metadata_schedule_473_e6393: f64 = (noise_metadata_schedule_473_e6379 - noise_metadata_schedule_473_e6392);
        let noise_metadata_schedule_473_e6394: f64 = (1e-40_f64).max(noise_metadata_schedule_473_e6393);
        (noise_metadata_schedule_473_e6394,)
    } else {
        (noise_variable_296,)
    }
};
            noise_variable_296 = noise_metadata_schedule_473_e6396;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_474_e6412,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_474_e6407: f64 = (0.5 * noise_variable_38);
        let noise_metadata_schedule_474_e6409: f64 = (noise_metadata_schedule_474_e6407 * noise_variable_292);
        let noise_metadata_schedule_474_e6410: f64 = (1.0 - noise_metadata_schedule_474_e6409);
        (noise_metadata_schedule_474_e6410,)
    } else {
        (noise_variable_297,)
    }
};
            noise_variable_297 = noise_metadata_schedule_474_e6412;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_475_e6432,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_475_e6422: f64 = (2.0 * noise_variable_291);
        let noise_metadata_schedule_475_e6426: f64 = (1.0 - noise_variable_292);
        let noise_metadata_schedule_475_e6428: f64 = (noise_metadata_schedule_475_e6426 - noise_variable_53);
        let noise_metadata_schedule_475_e6429: f64 = (noise_variable_38 * noise_metadata_schedule_475_e6428);
        let noise_metadata_schedule_475_e6430: f64 = (noise_metadata_schedule_475_e6422 + noise_metadata_schedule_475_e6429);
        (noise_metadata_schedule_475_e6430,)
    } else {
        (noise_variable_298,)
    }
};
            noise_variable_298 = noise_metadata_schedule_475_e6432;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_476_e6449,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_476_e6442: f64 = (noise_variable_51 - noise_variable_301);
        let noise_metadata_schedule_476_e6445: f64 = (noise_variable_296 / noise_variable_38);
        let noise_metadata_schedule_476_e6446: f64 = (noise_metadata_schedule_476_e6445).ln();
        let noise_metadata_schedule_476_e6447: f64 = (noise_metadata_schedule_476_e6442 + noise_metadata_schedule_476_e6446);
        (noise_metadata_schedule_476_e6447,)
    } else {
        (noise_variable_300,)
    }
};
            noise_variable_300 = noise_metadata_schedule_476_e6449;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_477_e6461,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_477_e6459: f64 = (noise_variable_296 + noise_variable_298);
        (noise_metadata_schedule_477_e6459,)
    } else {
        (noise_variable_317,)
    }
};
            noise_variable_317 = noise_metadata_schedule_477_e6461;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_478_e6463: f64 = (noise_variable_300).abs();
            let noise_metadata_schedule_478_e6465: f64 = if noise_metadata_schedule_478_e6463 < 1e-120 { 1.0 } else { 0.0 };
            noise_variable_319 = noise_metadata_schedule_478_e6465;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_479_e6477,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_319 != 0.0)) {
        (noise_variable_301,)
    } else {
        (noise_variable_310,)
    }
};
            noise_variable_310 = noise_metadata_schedule_479_e6477;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_480_e6504,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_319 == 0.0)) {
        let noise_metadata_schedule_480_e6490: f64 = (noise_variable_317 * noise_variable_317);
        let noise_metadata_schedule_480_e6493: f64 = (0.5 * noise_variable_298);
        let noise_metadata_schedule_480_e6495: f64 = (noise_metadata_schedule_480_e6493 * noise_variable_298);
        let noise_metadata_schedule_480_e6498: f64 = (noise_variable_296 * noise_variable_297);
        let noise_metadata_schedule_480_e6499: f64 = (noise_metadata_schedule_480_e6495 - noise_metadata_schedule_480_e6498);
        let noise_metadata_schedule_480_e6501: f64 = (noise_metadata_schedule_480_e6499 * noise_variable_300);
        let noise_metadata_schedule_480_e6502: f64 = (noise_metadata_schedule_480_e6490 + noise_metadata_schedule_480_e6501);
        (noise_metadata_schedule_480_e6502,)
    } else {
        (noise_variable_318,)
    }
};
            noise_variable_318 = noise_metadata_schedule_480_e6504;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_481_e6545,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_319 == 0.0)) {
        let noise_metadata_schedule_481_e6518: f64 = (noise_variable_296 * noise_variable_317);
        let noise_metadata_schedule_481_e6520: f64 = (noise_metadata_schedule_481_e6518 * noise_variable_300);
        let noise_metadata_schedule_481_e6524: f64 = (noise_variable_317 * noise_variable_300);
        let noise_metadata_schedule_481_e6526: f64 = (noise_metadata_schedule_481_e6524 * noise_variable_300);
        let noise_metadata_schedule_481_e6528: f64 = (noise_metadata_schedule_481_e6526 / noise_variable_318);
        let noise_metadata_schedule_481_e6530: f64 = (noise_metadata_schedule_481_e6528 * noise_variable_298);
        let noise_metadata_schedule_481_e6533: f64 = (noise_variable_298 * noise_variable_298);
        let noise_metadata_schedule_481_e6535: f64 = (noise_metadata_schedule_481_e6533 * 0.3333333333333333);
        let noise_metadata_schedule_481_e6538: f64 = (noise_variable_296 * noise_variable_297);
        let noise_metadata_schedule_481_e6539: f64 = (noise_metadata_schedule_481_e6535 - noise_metadata_schedule_481_e6538);
        let noise_metadata_schedule_481_e6540: f64 = (noise_metadata_schedule_481_e6530 * noise_metadata_schedule_481_e6539);
        let noise_metadata_schedule_481_e6541: f64 = (noise_variable_318 + noise_metadata_schedule_481_e6540);
        let noise_metadata_schedule_481_e6542: f64 = (noise_metadata_schedule_481_e6520 / noise_metadata_schedule_481_e6541);
        let noise_metadata_schedule_481_e6543: f64 = (noise_variable_301 + noise_metadata_schedule_481_e6542);
        (noise_metadata_schedule_481_e6543,)
    } else {
        (noise_variable_310,)
    }
};
            noise_variable_310 = noise_metadata_schedule_481_e6545;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_482_e6548: f64 = if noise_variable_310 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_320 = noise_metadata_schedule_482_e6548;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_483_e6561,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_483_e6559: f64 = (noise_variable_310).exp();
        (noise_metadata_schedule_483_e6559,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_483_e6561;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_484_e6575,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_484_e6573: f64 = (1.0 / noise_variable_302);
        (noise_metadata_schedule_484_e6573,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_484_e6575;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_485_e6589,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_485_e6587: f64 = (noise_variable_53 * noise_variable_302);
        (noise_metadata_schedule_485_e6587,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_485_e6589;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_486_e6593: f64 = (noise_variable_51 - 230.25850929940458);
            let noise_metadata_schedule_486_e6594: f64 = if noise_variable_310 > noise_metadata_schedule_486_e6593 { 1.0 } else { 0.0 };
            noise_variable_321 = noise_metadata_schedule_486_e6594;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_487_e6612,) = {
    if (((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_320 == 0.0)) && (noise_variable_321 != 0.0)) {
        let noise_metadata_schedule_487_e6609: f64 = (noise_variable_310 - noise_variable_51);
        let noise_metadata_schedule_487_e6610: f64 = (noise_metadata_schedule_487_e6609).exp();
        (noise_metadata_schedule_487_e6610,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_487_e6612;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_488_e6629,) = {
    if (((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_320 == 0.0)) && (noise_variable_321 != 0.0)) {
        let noise_metadata_schedule_488_e6627: f64 = (noise_variable_53 / noise_variable_302);
        (noise_metadata_schedule_488_e6627,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_488_e6629;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_489_e6673,) = {
    if (((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_320 == 0.0)) && (noise_variable_321 == 0.0)) {
        let noise_metadata_schedule_489_e6647: f64 = (noise_variable_51 - noise_variable_310);
        let noise_metadata_schedule_489_e6649: f64 = (noise_metadata_schedule_489_e6647 - 230.25850929940458);
        let noise_metadata_schedule_489_e6654: f64 = (noise_variable_51 - noise_variable_310);
        let noise_metadata_schedule_489_e6656: f64 = (noise_metadata_schedule_489_e6654 - 230.25850929940458);
        let noise_metadata_schedule_489_e6657: f64 = (0.5 * noise_metadata_schedule_489_e6656);
        let noise_metadata_schedule_489_e6661: f64 = (noise_variable_51 - noise_variable_310);
        let noise_metadata_schedule_489_e6663: f64 = (noise_metadata_schedule_489_e6661 - 230.25850929940458);
        let noise_metadata_schedule_489_e6665: f64 = (noise_metadata_schedule_489_e6663 * 0.3333333333333333);
        let noise_metadata_schedule_489_e6666: f64 = (1.0 + noise_metadata_schedule_489_e6665);
        let noise_metadata_schedule_489_e6667: f64 = (noise_metadata_schedule_489_e6657 * noise_metadata_schedule_489_e6666);
        let noise_metadata_schedule_489_e6668: f64 = (1.0 + noise_metadata_schedule_489_e6667);
        let noise_metadata_schedule_489_e6669: f64 = (noise_metadata_schedule_489_e6649 * noise_metadata_schedule_489_e6668);
        let noise_metadata_schedule_489_e6670: f64 = (1.0 + noise_metadata_schedule_489_e6669);
        let noise_metadata_schedule_489_e6671: f64 = (1e-100 / noise_metadata_schedule_489_e6670);
        (noise_metadata_schedule_489_e6671,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_489_e6673;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_490_e6711,) = {
    if (((((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) && (noise_variable_320 == 0.0)) && (noise_variable_321 == 0.0)) {
        let noise_metadata_schedule_490_e6691: f64 = (noise_variable_310 - 230.25850929940458);
        let noise_metadata_schedule_490_e6696: f64 = (noise_variable_310 - 230.25850929940458);
        let noise_metadata_schedule_490_e6697: f64 = (0.5 * noise_metadata_schedule_490_e6696);
        let noise_metadata_schedule_490_e6701: f64 = (noise_variable_310 - 230.25850929940458);
        let noise_metadata_schedule_490_e6703: f64 = (noise_metadata_schedule_490_e6701 * 0.3333333333333333);
        let noise_metadata_schedule_490_e6704: f64 = (1.0 + noise_metadata_schedule_490_e6703);
        let noise_metadata_schedule_490_e6705: f64 = (noise_metadata_schedule_490_e6697 * noise_metadata_schedule_490_e6704);
        let noise_metadata_schedule_490_e6706: f64 = (1.0 + noise_metadata_schedule_490_e6705);
        let noise_metadata_schedule_490_e6707: f64 = (noise_metadata_schedule_490_e6691 * noise_metadata_schedule_490_e6706);
        let noise_metadata_schedule_490_e6708: f64 = (1.0 + noise_metadata_schedule_490_e6707);
        let noise_metadata_schedule_490_e6709: f64 = (1e-100 / noise_metadata_schedule_490_e6708);
        (noise_metadata_schedule_490_e6709,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_490_e6711;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_491_e6727,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_491_e6723: f64 = (noise_variable_310 * noise_variable_310);
        let noise_metadata_schedule_491_e6724: f64 = (2.0 + noise_metadata_schedule_491_e6723);
        let noise_metadata_schedule_491_e6725: f64 = (1.0 / noise_metadata_schedule_491_e6724);
        (noise_metadata_schedule_491_e6725,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_491_e6727;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_492_e6739,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_492_e6737: f64 = (noise_variable_97 - noise_variable_310);
        (noise_metadata_schedule_492_e6737,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_492_e6739;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_493_e6761,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_493_e6749: f64 = (2.0 * noise_variable_291);
        let noise_metadata_schedule_493_e6753: f64 = (1.0 - noise_variable_303);
        let noise_metadata_schedule_493_e6755: f64 = (noise_metadata_schedule_493_e6753 + noise_variable_302);
        let noise_metadata_schedule_493_e6757: f64 = (noise_metadata_schedule_493_e6755 - noise_variable_53);
        let noise_metadata_schedule_493_e6758: f64 = (noise_variable_38 * noise_metadata_schedule_493_e6757);
        let noise_metadata_schedule_493_e6759: f64 = (noise_metadata_schedule_493_e6749 + noise_metadata_schedule_493_e6758);
        (noise_metadata_schedule_493_e6759,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_493_e6761;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_494_e6789,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_494_e6771: f64 = (noise_variable_291 * noise_variable_291);
        let noise_metadata_schedule_494_e6775: f64 = (noise_variable_303 + noise_variable_310);
        let noise_metadata_schedule_494_e6777: f64 = (noise_metadata_schedule_494_e6775 - 1.0);
        let noise_metadata_schedule_494_e6779: f64 = (noise_metadata_schedule_494_e6777 + noise_variable_302);
        let noise_metadata_schedule_494_e6783: f64 = (noise_variable_310 + 1.0);
        let noise_metadata_schedule_494_e6784: f64 = (noise_variable_53 * noise_metadata_schedule_494_e6783);
        let noise_metadata_schedule_494_e6785: f64 = (noise_metadata_schedule_494_e6779 - noise_metadata_schedule_494_e6784);
        let noise_metadata_schedule_494_e6786: f64 = (noise_variable_38 * noise_metadata_schedule_494_e6785);
        let noise_metadata_schedule_494_e6787: f64 = (noise_metadata_schedule_494_e6771 - noise_metadata_schedule_494_e6786);
        (noise_metadata_schedule_494_e6787,)
    } else {
        (noise_variable_305,)
    }
};
            noise_variable_305 = noise_metadata_schedule_494_e6789;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_495_e6805,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_495_e6801: f64 = (noise_variable_303 + noise_variable_302);
        let noise_metadata_schedule_495_e6802: f64 = (noise_variable_38 * noise_metadata_schedule_495_e6801);
        let noise_metadata_schedule_495_e6803: f64 = (2.0 - noise_metadata_schedule_495_e6802);
        (noise_metadata_schedule_495_e6803,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_495_e6805;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_496_e6823,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_496_e6815: f64 = (noise_variable_304 * noise_variable_304);
        let noise_metadata_schedule_496_e6818: f64 = (2.0 * noise_variable_305);
        let noise_metadata_schedule_496_e6820: f64 = (noise_metadata_schedule_496_e6818 * noise_variable_291);
        let noise_metadata_schedule_496_e6821: f64 = (noise_metadata_schedule_496_e6815 - noise_metadata_schedule_496_e6820);
        (noise_metadata_schedule_496_e6821,)
    } else {
        (noise_variable_291,)
    }
};
            noise_variable_291 = noise_metadata_schedule_496_e6823;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_497_e6842,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_311 == 0.0)) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_497_e6834: f64 = (2.0 * noise_variable_305);
        let noise_metadata_schedule_497_e6837: f64 = (noise_variable_291).sqrt();
        let noise_metadata_schedule_497_e6838: f64 = (noise_variable_304 + noise_metadata_schedule_497_e6837);
        let noise_metadata_schedule_497_e6839: f64 = (noise_metadata_schedule_497_e6834 / noise_metadata_schedule_497_e6838);
        let noise_metadata_schedule_497_e6840: f64 = (noise_variable_310 + noise_metadata_schedule_497_e6839);
        (noise_metadata_schedule_497_e6840,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_497_e6842;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_498_e6853,) = {
    if (noise_variable_289 != 0.0) {
        let noise_metadata_schedule_498_e6845: f64 = (-params.p17);
        let noise_metadata_schedule_498_e6847: f64 = (noise_metadata_schedule_498_e6845 * params.p18);
        let noise_metadata_schedule_498_e6849: f64 = (noise_metadata_schedule_498_e6847 * noise_variable_98);
        let noise_metadata_schedule_498_e6851: f64 = (noise_metadata_schedule_498_e6849 * noise_variable_25);
        (noise_metadata_schedule_498_e6851,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_498_e6853;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_499_e6863,) = {
    if (noise_variable_289 != 0.0) {
        let noise_metadata_schedule_499_e6857: f64 = (noise_variable_77 + (ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_499_e6859: f64 = (noise_metadata_schedule_499_e6857 - noise_variable_99);
        let noise_metadata_schedule_499_e6861: f64 = (noise_metadata_schedule_499_e6859 / noise_variable_25);
        (noise_metadata_schedule_499_e6861,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_499_e6863;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_500_e6865: f64 = (noise_variable_94).abs();
            let noise_metadata_schedule_500_e6867: f64 = if noise_metadata_schedule_500_e6865 <= noise_variable_40 { 1.0 } else { 0.0 };
            noise_variable_339 = noise_metadata_schedule_500_e6867;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_501_e6875,) = {
    if ((noise_variable_289 != 0.0) && (noise_variable_339 != 0.0)) {
        let noise_metadata_schedule_501_e6873: f64 = (noise_variable_94 / noise_variable_43);
        (noise_metadata_schedule_501_e6873,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_501_e6875;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_502_e6878: f64 = if noise_variable_94 > noise_variable_40 { 1.0 } else { 0.0 };
            noise_variable_340 = noise_metadata_schedule_502_e6878;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_503_e6895,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_503_e6887: f64 = (noise_variable_43 * 1.25);
        let noise_metadata_schedule_503_e6889: f64 = (noise_metadata_schedule_503_e6887 / noise_variable_60);
        let noise_metadata_schedule_503_e6891: f64 = (noise_metadata_schedule_503_e6889 - 1.0);
        let noise_metadata_schedule_503_e6893: f64 = (noise_metadata_schedule_503_e6891 / noise_variable_60);
        (noise_metadata_schedule_503_e6893,)
    } else {
        (noise_variable_334,)
    }
};
            noise_variable_334 = noise_metadata_schedule_503_e6895;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_504_e6912,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_504_e6904: f64 = (noise_variable_94 / noise_variable_43);
        let noise_metadata_schedule_504_e6908: f64 = (noise_variable_334 * noise_variable_94);
        let noise_metadata_schedule_504_e6909: f64 = (1.0 + noise_metadata_schedule_504_e6908);
        let noise_metadata_schedule_504_e6910: f64 = (noise_metadata_schedule_504_e6904 * noise_metadata_schedule_504_e6909);
        (noise_metadata_schedule_504_e6910,)
    } else {
        (noise_variable_335,)
    }
};
            noise_variable_335 = noise_metadata_schedule_504_e6912;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_505_e6915: f64 = if noise_variable_335 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_341 = noise_metadata_schedule_505_e6915;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_506_e6928,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) && (noise_variable_341 != 0.0)) {
        let noise_metadata_schedule_506_e6925: f64 = (-noise_variable_335);
        let noise_metadata_schedule_506_e6926: f64 = (noise_metadata_schedule_506_e6925).exp();
        (noise_metadata_schedule_506_e6926,)
    } else {
        (noise_variable_333,)
    }
};
            noise_variable_333 = noise_metadata_schedule_506_e6928;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_507_e6962,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) && (noise_variable_341 == 0.0)) {
        let noise_metadata_schedule_507_e6942: f64 = (noise_variable_335 - 460.51701859880916);
        let noise_metadata_schedule_507_e6947: f64 = (noise_variable_335 - 460.51701859880916);
        let noise_metadata_schedule_507_e6948: f64 = (0.5 * noise_metadata_schedule_507_e6947);
        let noise_metadata_schedule_507_e6952: f64 = (noise_variable_335 - 460.51701859880916);
        let noise_metadata_schedule_507_e6954: f64 = (noise_metadata_schedule_507_e6952 * 0.3333333333333333);
        let noise_metadata_schedule_507_e6955: f64 = (1.0 + noise_metadata_schedule_507_e6954);
        let noise_metadata_schedule_507_e6956: f64 = (noise_metadata_schedule_507_e6948 * noise_metadata_schedule_507_e6955);
        let noise_metadata_schedule_507_e6957: f64 = (1.0 + noise_metadata_schedule_507_e6956);
        let noise_metadata_schedule_507_e6958: f64 = (noise_metadata_schedule_507_e6942 * noise_metadata_schedule_507_e6957);
        let noise_metadata_schedule_507_e6959: f64 = (1.0 + noise_metadata_schedule_507_e6958);
        let noise_metadata_schedule_507_e6960: f64 = (1e-200 / noise_metadata_schedule_507_e6959);
        (noise_metadata_schedule_507_e6960,)
    } else {
        (noise_variable_333,)
    }
};
            noise_variable_333 = noise_metadata_schedule_507_e6962;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_508_e6973,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_508_e6971: f64 = (1.0 - noise_variable_333);
        (noise_metadata_schedule_508_e6971,)
    } else {
        (noise_variable_336,)
    }
};
            noise_variable_336 = noise_metadata_schedule_508_e6973;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_509_e6997,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_509_e6983: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_509_e6984: f64 = (noise_variable_94 + noise_metadata_schedule_509_e6983);
        let noise_metadata_schedule_509_e6989: f64 = (0.25 * noise_variable_36);
        let noise_metadata_schedule_509_e6990: f64 = (noise_variable_94 + noise_metadata_schedule_509_e6989);
        let noise_metadata_schedule_509_e6992: f64 = (noise_metadata_schedule_509_e6990 - noise_variable_336);
        let noise_metadata_schedule_509_e6993: f64 = (noise_metadata_schedule_509_e6992).sqrt();
        let noise_metadata_schedule_509_e6994: f64 = (noise_variable_34 * noise_metadata_schedule_509_e6993);
        let noise_metadata_schedule_509_e6995: f64 = (noise_metadata_schedule_509_e6984 - noise_metadata_schedule_509_e6994);
        (noise_metadata_schedule_509_e6995,)
    } else {
        (noise_variable_337,)
    }
};
            noise_variable_337 = noise_metadata_schedule_509_e6997;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_510_e7000: f64 = if noise_variable_337 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_342 = noise_metadata_schedule_510_e7000;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_511_e7013,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) && (noise_variable_342 != 0.0)) {
        let noise_metadata_schedule_511_e7010: f64 = (-noise_variable_337);
        let noise_metadata_schedule_511_e7011: f64 = (noise_metadata_schedule_511_e7010).exp();
        (noise_metadata_schedule_511_e7011,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_511_e7013;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_512_e7047,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) && (noise_variable_342 == 0.0)) {
        let noise_metadata_schedule_512_e7027: f64 = (noise_variable_337 - 460.51701859880916);
        let noise_metadata_schedule_512_e7032: f64 = (noise_variable_337 - 460.51701859880916);
        let noise_metadata_schedule_512_e7033: f64 = (0.5 * noise_metadata_schedule_512_e7032);
        let noise_metadata_schedule_512_e7037: f64 = (noise_variable_337 - 460.51701859880916);
        let noise_metadata_schedule_512_e7039: f64 = (noise_metadata_schedule_512_e7037 * 0.3333333333333333);
        let noise_metadata_schedule_512_e7040: f64 = (1.0 + noise_metadata_schedule_512_e7039);
        let noise_metadata_schedule_512_e7041: f64 = (noise_metadata_schedule_512_e7033 * noise_metadata_schedule_512_e7040);
        let noise_metadata_schedule_512_e7042: f64 = (1.0 + noise_metadata_schedule_512_e7041);
        let noise_metadata_schedule_512_e7043: f64 = (noise_metadata_schedule_512_e7027 * noise_metadata_schedule_512_e7042);
        let noise_metadata_schedule_512_e7044: f64 = (1.0 + noise_metadata_schedule_512_e7043);
        let noise_metadata_schedule_512_e7045: f64 = (1e-200 / noise_metadata_schedule_512_e7044);
        (noise_metadata_schedule_512_e7045,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_512_e7047;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_513_e7062,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_513_e7057: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_513_e7059: f64 = (noise_metadata_schedule_513_e7057 * noise_variable_329);
        let noise_metadata_schedule_513_e7060: f64 = (1.0 - noise_metadata_schedule_513_e7059);
        (noise_metadata_schedule_513_e7060,)
    } else {
        (noise_variable_330,)
    }
};
            noise_variable_330 = noise_metadata_schedule_513_e7062;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_514_e7081,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_514_e7072: f64 = (noise_variable_94 - noise_variable_337);
        let noise_metadata_schedule_514_e7073: f64 = (2.0 * noise_metadata_schedule_514_e7072);
        let noise_metadata_schedule_514_e7077: f64 = (1.0 - noise_variable_329);
        let noise_metadata_schedule_514_e7078: f64 = (noise_variable_36 * noise_metadata_schedule_514_e7077);
        let noise_metadata_schedule_514_e7079: f64 = (noise_metadata_schedule_514_e7073 + noise_metadata_schedule_514_e7078);
        (noise_metadata_schedule_514_e7079,)
    } else {
        (noise_variable_331,)
    }
};
            noise_variable_331 = noise_metadata_schedule_514_e7081;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_515_e7104,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_515_e7090: f64 = (noise_variable_94 - noise_variable_337);
        let noise_metadata_schedule_515_e7093: f64 = (noise_variable_94 - noise_variable_337);
        let noise_metadata_schedule_515_e7094: f64 = (noise_metadata_schedule_515_e7090 * noise_metadata_schedule_515_e7093);
        let noise_metadata_schedule_515_e7098: f64 = (noise_variable_337 - 1.0);
        let noise_metadata_schedule_515_e7100: f64 = (noise_metadata_schedule_515_e7098 + noise_variable_329);
        let noise_metadata_schedule_515_e7101: f64 = (noise_variable_36 * noise_metadata_schedule_515_e7100);
        let noise_metadata_schedule_515_e7102: f64 = (noise_metadata_schedule_515_e7094 - noise_metadata_schedule_515_e7101);
        (noise_metadata_schedule_515_e7102,)
    } else {
        (noise_variable_332,)
    }
};
            noise_variable_332 = noise_metadata_schedule_515_e7104;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_516_e7121,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_516_e7113: f64 = (noise_variable_331 * noise_variable_331);
        let noise_metadata_schedule_516_e7116: f64 = (4.0 * noise_variable_330);
        let noise_metadata_schedule_516_e7118: f64 = (noise_metadata_schedule_516_e7116 * noise_variable_332);
        let noise_metadata_schedule_516_e7119: f64 = (noise_metadata_schedule_516_e7113 - noise_metadata_schedule_516_e7118);
        (noise_metadata_schedule_516_e7119,)
    } else {
        (noise_variable_333,)
    }
};
            noise_variable_333 = noise_metadata_schedule_516_e7121;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_517_e7137,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_517_e7130: f64 = (2.0 * noise_variable_332);
        let noise_metadata_schedule_517_e7133: f64 = (noise_variable_333).sqrt();
        let noise_metadata_schedule_517_e7134: f64 = (noise_variable_331 + noise_metadata_schedule_517_e7133);
        let noise_metadata_schedule_517_e7135: f64 = (noise_metadata_schedule_517_e7130 / noise_metadata_schedule_517_e7134);
        (noise_metadata_schedule_517_e7135,)
    } else {
        (noise_variable_338,)
    }
};
            noise_variable_338 = noise_metadata_schedule_517_e7137;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_518_e7148,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 != 0.0)) {
        let noise_metadata_schedule_518_e7146: f64 = (noise_variable_337 + noise_variable_338);
        (noise_metadata_schedule_518_e7146,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_518_e7148;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_519_e7159,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_519_e7157: f64 = (-noise_variable_94);
        (noise_metadata_schedule_519_e7157,)
    } else {
        (noise_variable_322,)
    }
};
            noise_variable_322 = noise_metadata_schedule_519_e7159;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_520_e7173,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_520_e7169: f64 = (1.25 * noise_variable_322);
        let noise_metadata_schedule_520_e7171: f64 = (noise_metadata_schedule_520_e7169 / noise_variable_43);
        (noise_metadata_schedule_520_e7171,)
    } else {
        (noise_variable_323,)
    }
};
            noise_variable_323 = noise_metadata_schedule_520_e7173;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_521_e7198,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_521_e7184: f64 = (noise_variable_323 + 10.0);
        let noise_metadata_schedule_521_e7187: f64 = (noise_variable_323 - 6.0);
        let noise_metadata_schedule_521_e7190: f64 = (noise_variable_323 - 6.0);
        let noise_metadata_schedule_521_e7191: f64 = (noise_metadata_schedule_521_e7187 * noise_metadata_schedule_521_e7190);
        let noise_metadata_schedule_521_e7193: f64 = (noise_metadata_schedule_521_e7191 + 64.0);
        let noise_metadata_schedule_521_e7194: f64 = (noise_metadata_schedule_521_e7193).sqrt();
        let noise_metadata_schedule_521_e7195: f64 = (noise_metadata_schedule_521_e7184 - noise_metadata_schedule_521_e7194);
        let noise_metadata_schedule_521_e7196: f64 = (0.5 * noise_metadata_schedule_521_e7195);
        (noise_metadata_schedule_521_e7196,)
    } else {
        (noise_variable_324,)
    }
};
            noise_variable_324 = noise_metadata_schedule_521_e7198;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_522_e7220,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_522_e7208: f64 = (noise_variable_322 - noise_variable_324);
        let noise_metadata_schedule_522_e7211: f64 = (noise_variable_322 - noise_variable_324);
        let noise_metadata_schedule_522_e7212: f64 = (noise_metadata_schedule_522_e7208 * noise_metadata_schedule_522_e7211);
        let noise_metadata_schedule_522_e7216: f64 = (noise_variable_324 + 1.0);
        let noise_metadata_schedule_522_e7217: f64 = (noise_variable_36 * noise_metadata_schedule_522_e7216);
        let noise_metadata_schedule_522_e7218: f64 = (noise_metadata_schedule_522_e7212 + noise_metadata_schedule_522_e7217);
        (noise_metadata_schedule_522_e7218,)
    } else {
        (noise_variable_325,)
    }
};
            noise_variable_325 = noise_metadata_schedule_522_e7220;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_523_e7236,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_523_e7231: f64 = (noise_variable_322 - noise_variable_324);
        let noise_metadata_schedule_523_e7232: f64 = (2.0 * noise_metadata_schedule_523_e7231);
        let noise_metadata_schedule_523_e7234: f64 = (noise_metadata_schedule_523_e7232 - noise_variable_36);
        (noise_metadata_schedule_523_e7234,)
    } else {
        (noise_variable_326,)
    }
};
            noise_variable_326 = noise_metadata_schedule_523_e7236;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_524_e7251,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_524_e7246: f64 = (noise_variable_325 / noise_variable_36);
        let noise_metadata_schedule_524_e7247: f64 = (noise_metadata_schedule_524_e7246).ln();
        let noise_metadata_schedule_524_e7249: f64 = (noise_metadata_schedule_524_e7247 - noise_variable_324);
        (noise_metadata_schedule_524_e7249,)
    } else {
        (noise_variable_327,)
    }
};
            noise_variable_327 = noise_metadata_schedule_524_e7251;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_525_e7263,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_525_e7261: f64 = (noise_variable_325 + noise_variable_326);
        (noise_metadata_schedule_525_e7261,)
    } else {
        (noise_variable_343,)
    }
};
            noise_variable_343 = noise_metadata_schedule_525_e7263;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_526_e7285,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_526_e7273: f64 = (noise_variable_343 * noise_variable_343);
        let noise_metadata_schedule_526_e7276: f64 = (0.5 * noise_variable_326);
        let noise_metadata_schedule_526_e7278: f64 = (noise_metadata_schedule_526_e7276 * noise_variable_326);
        let noise_metadata_schedule_526_e7280: f64 = (noise_metadata_schedule_526_e7278 - noise_variable_325);
        let noise_metadata_schedule_526_e7282: f64 = (noise_metadata_schedule_526_e7280 * noise_variable_327);
        let noise_metadata_schedule_526_e7283: f64 = (noise_metadata_schedule_526_e7273 + noise_metadata_schedule_526_e7282);
        (noise_metadata_schedule_526_e7283,)
    } else {
        (noise_variable_344,)
    }
};
            noise_variable_344 = noise_metadata_schedule_526_e7285;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_527_e7321,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_527_e7296: f64 = (noise_variable_325 * noise_variable_343);
        let noise_metadata_schedule_527_e7298: f64 = (noise_metadata_schedule_527_e7296 * noise_variable_327);
        let noise_metadata_schedule_527_e7302: f64 = (noise_variable_343 * noise_variable_327);
        let noise_metadata_schedule_527_e7304: f64 = (noise_metadata_schedule_527_e7302 * noise_variable_327);
        let noise_metadata_schedule_527_e7306: f64 = (noise_metadata_schedule_527_e7304 / noise_variable_344);
        let noise_metadata_schedule_527_e7308: f64 = (noise_metadata_schedule_527_e7306 * noise_variable_326);
        let noise_metadata_schedule_527_e7311: f64 = (noise_variable_326 * noise_variable_326);
        let noise_metadata_schedule_527_e7313: f64 = (noise_metadata_schedule_527_e7311 * 0.3333333333333333);
        let noise_metadata_schedule_527_e7315: f64 = (noise_metadata_schedule_527_e7313 - noise_variable_325);
        let noise_metadata_schedule_527_e7316: f64 = (noise_metadata_schedule_527_e7308 * noise_metadata_schedule_527_e7315);
        let noise_metadata_schedule_527_e7317: f64 = (noise_variable_344 + noise_metadata_schedule_527_e7316);
        let noise_metadata_schedule_527_e7318: f64 = (noise_metadata_schedule_527_e7298 / noise_metadata_schedule_527_e7317);
        let noise_metadata_schedule_527_e7319: f64 = (noise_variable_324 + noise_metadata_schedule_527_e7318);
        (noise_metadata_schedule_527_e7319,)
    } else {
        (noise_variable_328,)
    }
};
            noise_variable_328 = noise_metadata_schedule_527_e7321;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_528_e7323: f64 = (noise_variable_328).abs();
            let noise_metadata_schedule_528_e7325: f64 = if noise_metadata_schedule_528_e7323 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_345 = noise_metadata_schedule_528_e7325;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_529_e7338,) = {
    if ((((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) && (noise_variable_345 != 0.0)) {
        let noise_metadata_schedule_529_e7336: f64 = (noise_variable_328).exp();
        (noise_metadata_schedule_529_e7336,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_529_e7338;
        }
        if matches!(source_index, 0 | 6) {
            let noise_metadata_schedule_530_e7341: f64 = (-230.25850929940458);
            let noise_metadata_schedule_530_e7342: f64 = if noise_variable_328 < noise_metadata_schedule_530_e7341 { 1.0 } else { 0.0 };
            noise_variable_346 = noise_metadata_schedule_530_e7342;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_531_e7382,) = {
    if (((((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) && (noise_variable_345 == 0.0)) && (noise_variable_346 != 0.0)) {
        let noise_metadata_schedule_531_e7358: f64 = (-230.25850929940458);
        let noise_metadata_schedule_531_e7360: f64 = (noise_metadata_schedule_531_e7358 - noise_variable_328);
        let noise_metadata_schedule_531_e7364: f64 = (-230.25850929940458);
        let noise_metadata_schedule_531_e7366: f64 = (noise_metadata_schedule_531_e7364 - noise_variable_328);
        let noise_metadata_schedule_531_e7367: f64 = (0.5 * noise_metadata_schedule_531_e7366);
        let noise_metadata_schedule_531_e7370: f64 = (-230.25850929940458);
        let noise_metadata_schedule_531_e7372: f64 = (noise_metadata_schedule_531_e7370 - noise_variable_328);
        let noise_metadata_schedule_531_e7374: f64 = (noise_metadata_schedule_531_e7372 * 0.3333333333333333);
        let noise_metadata_schedule_531_e7375: f64 = (1.0 + noise_metadata_schedule_531_e7374);
        let noise_metadata_schedule_531_e7376: f64 = (noise_metadata_schedule_531_e7367 * noise_metadata_schedule_531_e7375);
        let noise_metadata_schedule_531_e7377: f64 = (1.0 + noise_metadata_schedule_531_e7376);
        let noise_metadata_schedule_531_e7378: f64 = (noise_metadata_schedule_531_e7360 * noise_metadata_schedule_531_e7377);
        let noise_metadata_schedule_531_e7379: f64 = (1.0 + noise_metadata_schedule_531_e7378);
        let noise_metadata_schedule_531_e7380: f64 = (1e-100 / noise_metadata_schedule_531_e7379);
        (noise_metadata_schedule_531_e7380,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_531_e7382;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_532_e7420,) = {
    if (((((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) && (noise_variable_345 == 0.0)) && (noise_variable_346 == 0.0)) {
        let noise_metadata_schedule_532_e7400: f64 = (noise_variable_328 - 230.25850929940458);
        let noise_metadata_schedule_532_e7405: f64 = (noise_variable_328 - 230.25850929940458);
        let noise_metadata_schedule_532_e7406: f64 = (0.5 * noise_metadata_schedule_532_e7405);
        let noise_metadata_schedule_532_e7410: f64 = (noise_variable_328 - 230.25850929940458);
        let noise_metadata_schedule_532_e7412: f64 = (noise_metadata_schedule_532_e7410 * 0.3333333333333333);
        let noise_metadata_schedule_532_e7413: f64 = (1.0 + noise_metadata_schedule_532_e7412);
        let noise_metadata_schedule_532_e7414: f64 = (noise_metadata_schedule_532_e7406 * noise_metadata_schedule_532_e7413);
        let noise_metadata_schedule_532_e7415: f64 = (1.0 + noise_metadata_schedule_532_e7414);
        let noise_metadata_schedule_532_e7416: f64 = (noise_metadata_schedule_532_e7400 * noise_metadata_schedule_532_e7415);
        let noise_metadata_schedule_532_e7417: f64 = (1.0 + noise_metadata_schedule_532_e7416);
        let noise_metadata_schedule_532_e7418: f64 = (1e100 * noise_metadata_schedule_532_e7417);
        (noise_metadata_schedule_532_e7418,)
    } else {
        (noise_variable_329,)
    }
};
            noise_variable_329 = noise_metadata_schedule_532_e7420;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_533_e7436,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_533_e7431: f64 = (0.5 * noise_variable_36);
        let noise_metadata_schedule_533_e7433: f64 = (noise_metadata_schedule_533_e7431 * noise_variable_329);
        let noise_metadata_schedule_533_e7434: f64 = (1.0 - noise_metadata_schedule_533_e7433);
        (noise_metadata_schedule_533_e7434,)
    } else {
        (noise_variable_330,)
    }
};
            noise_variable_330 = noise_metadata_schedule_533_e7436;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_534_e7456,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_534_e7447: f64 = (noise_variable_322 - noise_variable_328);
        let noise_metadata_schedule_534_e7448: f64 = (2.0 * noise_metadata_schedule_534_e7447);
        let noise_metadata_schedule_534_e7452: f64 = (noise_variable_329 - 1.0);
        let noise_metadata_schedule_534_e7453: f64 = (noise_variable_36 * noise_metadata_schedule_534_e7452);
        let noise_metadata_schedule_534_e7454: f64 = (noise_metadata_schedule_534_e7448 + noise_metadata_schedule_534_e7453);
        (noise_metadata_schedule_534_e7454,)
    } else {
        (noise_variable_331,)
    }
};
            noise_variable_331 = noise_metadata_schedule_534_e7456;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_535_e7480,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_535_e7466: f64 = (noise_variable_322 - noise_variable_328);
        let noise_metadata_schedule_535_e7469: f64 = (noise_variable_322 - noise_variable_328);
        let noise_metadata_schedule_535_e7470: f64 = (noise_metadata_schedule_535_e7466 * noise_metadata_schedule_535_e7469);
        let noise_metadata_schedule_535_e7474: f64 = (noise_variable_328 + 1.0);
        let noise_metadata_schedule_535_e7476: f64 = (noise_metadata_schedule_535_e7474 - noise_variable_329);
        let noise_metadata_schedule_535_e7477: f64 = (noise_variable_36 * noise_metadata_schedule_535_e7476);
        let noise_metadata_schedule_535_e7478: f64 = (noise_metadata_schedule_535_e7470 + noise_metadata_schedule_535_e7477);
        (noise_metadata_schedule_535_e7478,)
    } else {
        (noise_variable_332,)
    }
};
            noise_variable_332 = noise_metadata_schedule_535_e7480;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_536_e7498,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_536_e7490: f64 = (noise_variable_331 * noise_variable_331);
        let noise_metadata_schedule_536_e7493: f64 = (4.0 * noise_variable_330);
        let noise_metadata_schedule_536_e7495: f64 = (noise_metadata_schedule_536_e7493 * noise_variable_332);
        let noise_metadata_schedule_536_e7496: f64 = (noise_metadata_schedule_536_e7490 - noise_metadata_schedule_536_e7495);
        (noise_metadata_schedule_536_e7496,)
    } else {
        (noise_variable_333,)
    }
};
            noise_variable_333 = noise_metadata_schedule_536_e7498;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_537_e7515,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_537_e7508: f64 = (2.0 * noise_variable_332);
        let noise_metadata_schedule_537_e7511: f64 = (noise_variable_333).sqrt();
        let noise_metadata_schedule_537_e7512: f64 = (noise_variable_331 + noise_metadata_schedule_537_e7511);
        let noise_metadata_schedule_537_e7513: f64 = (noise_metadata_schedule_537_e7508 / noise_metadata_schedule_537_e7512);
        (noise_metadata_schedule_537_e7513,)
    } else {
        (noise_variable_336,)
    }
};
            noise_variable_336 = noise_metadata_schedule_537_e7515;
        }
        if matches!(source_index, 0 | 6) {
            let (noise_metadata_schedule_538_e7528,) = {
    if (((noise_variable_289 != 0.0) && (noise_variable_339 == 0.0)) && (noise_variable_340 == 0.0)) {
        let noise_metadata_schedule_538_e7525: f64 = (noise_variable_328 + noise_variable_336);
        let noise_metadata_schedule_538_e7526: f64 = (-noise_metadata_schedule_538_e7525);
        (noise_metadata_schedule_538_e7526,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_538_e7528;
        }
        if matches!(source_index, 6) {
            noise_variable_83 = 0.0;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_542_e7543: f64 = if noise_variable_95 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_347 = noise_metadata_schedule_542_e7543;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_543_e7548,) = {
    if (noise_variable_347 != 0.0) {
        let noise_metadata_schedule_543_e7546: f64 = (noise_variable_95).exp();
        (noise_metadata_schedule_543_e7546,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_543_e7548;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_544_e7554,) = {
    if (noise_variable_347 != 0.0) {
        let noise_metadata_schedule_544_e7552: f64 = (1.0 / noise_variable_83);
        (noise_metadata_schedule_544_e7552,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_544_e7554;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_545_e7558: f64 = (noise_variable_50 - 230.25850929940458);
            let noise_metadata_schedule_545_e7559: f64 = if noise_variable_95 > noise_metadata_schedule_545_e7558 { 1.0 } else { 0.0 };
            noise_variable_348 = noise_metadata_schedule_545_e7559;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_546_e7569,) = {
    if ((noise_variable_347 == 0.0) && (noise_variable_348 != 0.0)) {
        let noise_metadata_schedule_546_e7566: f64 = (noise_variable_50 - noise_variable_95);
        let noise_metadata_schedule_546_e7567: f64 = (noise_metadata_schedule_546_e7566).exp();
        (noise_metadata_schedule_546_e7567,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_546_e7569;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_547_e7578,) = {
    if ((noise_variable_347 == 0.0) && (noise_variable_348 != 0.0)) {
        let noise_metadata_schedule_547_e7576: f64 = (noise_variable_52 * noise_variable_83);
        (noise_metadata_schedule_547_e7576,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_547_e7578;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_548_e7608,) = {
    if ((noise_variable_347 == 0.0) && (noise_variable_348 == 0.0)) {
        let noise_metadata_schedule_548_e7588: f64 = (noise_variable_95 - 230.25850929940458);
        let noise_metadata_schedule_548_e7593: f64 = (noise_variable_95 - 230.25850929940458);
        let noise_metadata_schedule_548_e7594: f64 = (0.5 * noise_metadata_schedule_548_e7593);
        let noise_metadata_schedule_548_e7598: f64 = (noise_variable_95 - 230.25850929940458);
        let noise_metadata_schedule_548_e7600: f64 = (noise_metadata_schedule_548_e7598 * 0.3333333333333333);
        let noise_metadata_schedule_548_e7601: f64 = (1.0 + noise_metadata_schedule_548_e7600);
        let noise_metadata_schedule_548_e7602: f64 = (noise_metadata_schedule_548_e7594 * noise_metadata_schedule_548_e7601);
        let noise_metadata_schedule_548_e7603: f64 = (1.0 + noise_metadata_schedule_548_e7602);
        let noise_metadata_schedule_548_e7604: f64 = (noise_metadata_schedule_548_e7588 * noise_metadata_schedule_548_e7603);
        let noise_metadata_schedule_548_e7605: f64 = (1.0 + noise_metadata_schedule_548_e7604);
        let noise_metadata_schedule_548_e7606: f64 = (1e-100 / noise_metadata_schedule_548_e7605);
        (noise_metadata_schedule_548_e7606,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_548_e7608;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_549_e7611: f64 = (-noise_variable_40);
            let noise_metadata_schedule_549_e7612: f64 = if noise_variable_95 < noise_metadata_schedule_549_e7611 { 1.0 } else { 0.0 };
            noise_variable_349 = noise_metadata_schedule_549_e7612;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_550_e7620,) = {
    if (noise_variable_349 != 0.0) {
        let noise_metadata_schedule_550_e7616: f64 = (noise_variable_85 + noise_variable_95);
        let noise_metadata_schedule_550_e7618: f64 = (noise_metadata_schedule_550_e7616 - 1.0);
        (noise_metadata_schedule_550_e7618,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_550_e7620;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_551_e7626,) = {
    if (noise_variable_349 != 0.0) {
        let noise_metadata_schedule_551_e7623: f64 = (noise_variable_86).sqrt();
        let noise_metadata_schedule_551_e7624: f64 = (-noise_metadata_schedule_551_e7623);
        (noise_metadata_schedule_551_e7624,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_551_e7626;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_552_e7628: f64 = (noise_variable_95).abs();
            let noise_metadata_schedule_552_e7630: f64 = if noise_metadata_schedule_552_e7628 <= noise_variable_40 { 1.0 } else { 0.0 };
            noise_variable_350 = noise_metadata_schedule_552_e7630;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_553_e7647,) = {
    if ((noise_variable_349 == 0.0) && (noise_variable_350 != 0.0)) {
        let noise_metadata_schedule_553_e7638: f64 = (0.3333333333333333 * noise_variable_95);
        let noise_metadata_schedule_553_e7642: f64 = (0.25 * noise_variable_95);
        let noise_metadata_schedule_553_e7643: f64 = (1.0 - noise_metadata_schedule_553_e7642);
        let noise_metadata_schedule_553_e7644: f64 = (noise_metadata_schedule_553_e7638 * noise_metadata_schedule_553_e7643);
        let noise_metadata_schedule_553_e7645: f64 = (1.0 - noise_metadata_schedule_553_e7644);
        (noise_metadata_schedule_553_e7645,)
    } else {
        (noise_variable_6,)
    }
};
            noise_variable_6 = noise_metadata_schedule_553_e7647;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_554_e7660,) = {
    if ((noise_variable_349 == 0.0) && (noise_variable_350 != 0.0)) {
        let noise_metadata_schedule_554_e7654: f64 = (0.5 * noise_variable_95);
        let noise_metadata_schedule_554_e7656: f64 = (noise_metadata_schedule_554_e7654 * noise_variable_95);
        let noise_metadata_schedule_554_e7658: f64 = (noise_metadata_schedule_554_e7656 * noise_variable_6);
        (noise_metadata_schedule_554_e7658,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_554_e7660;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_555_e7672,) = {
    if ((noise_variable_349 == 0.0) && (noise_variable_350 != 0.0)) {
        let noise_metadata_schedule_555_e7667: f64 = (0.7071067811865475 * noise_variable_95);
        let noise_metadata_schedule_555_e7669: f64 = (noise_variable_6).sqrt();
        let noise_metadata_schedule_555_e7670: f64 = (noise_metadata_schedule_555_e7667 * noise_metadata_schedule_555_e7669);
        (noise_metadata_schedule_555_e7670,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_555_e7672;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_556_e7684,) = {
    if ((noise_variable_349 == 0.0) && (noise_variable_350 == 0.0)) {
        let noise_metadata_schedule_556_e7680: f64 = (noise_variable_95 - 1.0);
        let noise_metadata_schedule_556_e7682: f64 = (noise_metadata_schedule_556_e7680 + noise_variable_85);
        (noise_metadata_schedule_556_e7682,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_556_e7684;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_557_e7693,) = {
    if ((noise_variable_349 == 0.0) && (noise_variable_350 == 0.0)) {
        let noise_metadata_schedule_557_e7691: f64 = (noise_variable_86).sqrt();
        (noise_metadata_schedule_557_e7691,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_557_e7693;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_558_e7696: f64 = (noise_variable_25 * noise_variable_88);
            let noise_metadata_schedule_558_e7698: f64 = (noise_metadata_schedule_558_e7696 * noise_variable_34);
            noise_variable_91 = noise_metadata_schedule_558_e7698;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_559_e7702: f64 = (1.0 + noise_variable_140);
            let noise_metadata_schedule_559_e7703: f64 = (1.62 * noise_metadata_schedule_559_e7702);
            let noise_metadata_schedule_559_e7706: f64 = (1.0 + noise_variable_140);
            let noise_metadata_schedule_559_e7707: f64 = (noise_metadata_schedule_559_e7703 * noise_metadata_schedule_559_e7706);
            let noise_metadata_schedule_559_e7711: f64 = (0.37 * noise_variable_141);
            let noise_metadata_schedule_559_e7712: f64 = (1.0 + noise_metadata_schedule_559_e7711);
            let noise_metadata_schedule_559_e7713: f64 = (noise_metadata_schedule_559_e7707 * noise_metadata_schedule_559_e7712);
            let noise_metadata_schedule_559_e7717: f64 = (0.37 * noise_variable_141);
            let noise_metadata_schedule_559_e7718: f64 = (1.0 + noise_metadata_schedule_559_e7717);
            let noise_metadata_schedule_559_e7719: f64 = (noise_metadata_schedule_559_e7713 * noise_metadata_schedule_559_e7718);
            let noise_metadata_schedule_559_e7721: f64 = (noise_metadata_schedule_559_e7719 * noise_variable_20);
            let noise_metadata_schedule_559_e7723: f64 = (noise_variable_20).sqrt();
            let noise_metadata_schedule_559_e7724: f64 = (noise_metadata_schedule_559_e7721 * noise_metadata_schedule_559_e7723);
            let noise_metadata_schedule_559_e7726: f64 = (noise_metadata_schedule_559_e7724 * noise_variable_25);
            let noise_metadata_schedule_559_e7728: f64 = (noise_metadata_schedule_559_e7726 * noise_variable_25);
            noise_variable_139 = noise_metadata_schedule_559_e7728;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_560_e7731: f64 = (-noise_variable_91);
            let noise_metadata_schedule_560_e7732: f64 = (noise_variable_91 - noise_metadata_schedule_560_e7731);
            let (noise_metadata_schedule_560_e7802,) = {
    if (noise_metadata_schedule_560_e7732 > 1e-16) {
        let noise_metadata_schedule_560_e7736: f64 = (-noise_variable_91);
        let noise_metadata_schedule_560_e7740: f64 = (-noise_variable_91);
        let noise_metadata_schedule_560_e7741: f64 = (noise_variable_91 - noise_metadata_schedule_560_e7740);
        let noise_metadata_schedule_560_e7744: f64 = (-noise_variable_91);
        let noise_metadata_schedule_560_e7745: f64 = (noise_variable_91 - noise_metadata_schedule_560_e7744);
        let noise_metadata_schedule_560_e7748: f64 = (-noise_variable_91);
        let noise_metadata_schedule_560_e7749: f64 = (noise_variable_91 - noise_metadata_schedule_560_e7748);
        let noise_metadata_schedule_560_e7750: f64 = (noise_metadata_schedule_560_e7745 * noise_metadata_schedule_560_e7749);
        let noise_metadata_schedule_560_e7752: f64 = (noise_metadata_schedule_560_e7750 + noise_variable_139);
        let noise_metadata_schedule_560_e7753: f64 = (noise_metadata_schedule_560_e7752).sqrt();
        let noise_metadata_schedule_560_e7754: f64 = (noise_metadata_schedule_560_e7741 + noise_metadata_schedule_560_e7753);
        let noise_metadata_schedule_560_e7755: f64 = (0.5 * noise_metadata_schedule_560_e7754);
        let noise_metadata_schedule_560_e7756: f64 = (noise_metadata_schedule_560_e7736 + noise_metadata_schedule_560_e7755);
        (noise_metadata_schedule_560_e7756,)
    } else {
        let noise_metadata_schedule_560_e7758: f64 = (-noise_variable_91);
        let noise_metadata_schedule_560_e7760: f64 = (noise_metadata_schedule_560_e7758 - noise_variable_91);
        let (noise_metadata_schedule_560_e7801,) = {
            if (noise_metadata_schedule_560_e7760 > 1e-16) {
                let noise_metadata_schedule_560_e7764: f64 = (-noise_variable_91);
                let noise_metadata_schedule_560_e7767: f64 = (0.5 * noise_variable_139);
                let noise_metadata_schedule_560_e7769: f64 = (-noise_variable_91);
                let noise_metadata_schedule_560_e7771: f64 = (noise_metadata_schedule_560_e7769 - noise_variable_91);
                let noise_metadata_schedule_560_e7773: f64 = (-noise_variable_91);
                let noise_metadata_schedule_560_e7775: f64 = (noise_metadata_schedule_560_e7773 - noise_variable_91);
                let noise_metadata_schedule_560_e7777: f64 = (-noise_variable_91);
                let noise_metadata_schedule_560_e7779: f64 = (noise_metadata_schedule_560_e7777 - noise_variable_91);
                let noise_metadata_schedule_560_e7780: f64 = (noise_metadata_schedule_560_e7775 * noise_metadata_schedule_560_e7779);
                let noise_metadata_schedule_560_e7782: f64 = (noise_metadata_schedule_560_e7780 + noise_variable_139);
                let noise_metadata_schedule_560_e7783: f64 = (noise_metadata_schedule_560_e7782).sqrt();
                let noise_metadata_schedule_560_e7784: f64 = (noise_metadata_schedule_560_e7771 + noise_metadata_schedule_560_e7783);
                let noise_metadata_schedule_560_e7785: f64 = (noise_metadata_schedule_560_e7767 / noise_metadata_schedule_560_e7784);
                let noise_metadata_schedule_560_e7786: f64 = (noise_metadata_schedule_560_e7764 + noise_metadata_schedule_560_e7785);
                (noise_metadata_schedule_560_e7786,)
            } else {
                let noise_metadata_schedule_560_e7788: f64 = (-noise_variable_91);
                let noise_metadata_schedule_560_e7792: f64 = (-noise_variable_91);
                let noise_metadata_schedule_560_e7793: f64 = (noise_variable_91 - noise_metadata_schedule_560_e7792);
                let noise_metadata_schedule_560_e7796: f64 = (1e-32 + noise_variable_139);
                let noise_metadata_schedule_560_e7797: f64 = (noise_metadata_schedule_560_e7796).sqrt();
                let noise_metadata_schedule_560_e7798: f64 = (noise_metadata_schedule_560_e7793 + noise_metadata_schedule_560_e7797);
                let noise_metadata_schedule_560_e7799: f64 = (0.5 * noise_metadata_schedule_560_e7798);
                let noise_metadata_schedule_560_e7800: f64 = (noise_metadata_schedule_560_e7788 + noise_metadata_schedule_560_e7799);
                (noise_metadata_schedule_560_e7800,)
            }
        };
        (noise_metadata_schedule_560_e7801,)
    }
};
            let noise_metadata_schedule_560_e7805: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
            let noise_metadata_schedule_560_e7807: f64 = (noise_metadata_schedule_560_e7805 - (ctx.node_voltage(self.nodes[6]) - 0.0));
            let (noise_metadata_schedule_560_e7874,) = {
    if (noise_metadata_schedule_560_e7807 > 1e-16) {
        let noise_metadata_schedule_560_e7813: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_560_e7815: f64 = (noise_metadata_schedule_560_e7813 - (ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_560_e7817: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_560_e7819: f64 = (noise_metadata_schedule_560_e7817 - (ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_560_e7821: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_560_e7823: f64 = (noise_metadata_schedule_560_e7821 - (ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_560_e7824: f64 = (noise_metadata_schedule_560_e7819 * noise_metadata_schedule_560_e7823);
        let noise_metadata_schedule_560_e7826: f64 = (noise_metadata_schedule_560_e7824 + noise_variable_139);
        let noise_metadata_schedule_560_e7827: f64 = (noise_metadata_schedule_560_e7826).sqrt();
        let noise_metadata_schedule_560_e7828: f64 = (noise_metadata_schedule_560_e7815 + noise_metadata_schedule_560_e7827);
        let noise_metadata_schedule_560_e7829: f64 = (0.5 * noise_metadata_schedule_560_e7828);
        let noise_metadata_schedule_560_e7830: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0) + noise_metadata_schedule_560_e7829);
        (noise_metadata_schedule_560_e7830,)
    } else {
        let noise_metadata_schedule_560_e7833: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
        let noise_metadata_schedule_560_e7834: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0) - noise_metadata_schedule_560_e7833);
        let (noise_metadata_schedule_560_e7873,) = {
            if (noise_metadata_schedule_560_e7834 > 1e-16) {
                let noise_metadata_schedule_560_e7840: f64 = (0.5 * noise_variable_139);
                let noise_metadata_schedule_560_e7843: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
                let noise_metadata_schedule_560_e7844: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0) - noise_metadata_schedule_560_e7843);
                let noise_metadata_schedule_560_e7847: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
                let noise_metadata_schedule_560_e7848: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0) - noise_metadata_schedule_560_e7847);
                let noise_metadata_schedule_560_e7851: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
                let noise_metadata_schedule_560_e7852: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0) - noise_metadata_schedule_560_e7851);
                let noise_metadata_schedule_560_e7853: f64 = (noise_metadata_schedule_560_e7848 * noise_metadata_schedule_560_e7852);
                let noise_metadata_schedule_560_e7855: f64 = (noise_metadata_schedule_560_e7853 + noise_variable_139);
                let noise_metadata_schedule_560_e7856: f64 = (noise_metadata_schedule_560_e7855).sqrt();
                let noise_metadata_schedule_560_e7857: f64 = (noise_metadata_schedule_560_e7844 + noise_metadata_schedule_560_e7856);
                let noise_metadata_schedule_560_e7858: f64 = (noise_metadata_schedule_560_e7840 / noise_metadata_schedule_560_e7857);
                let noise_metadata_schedule_560_e7859: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0) + noise_metadata_schedule_560_e7858);
                (noise_metadata_schedule_560_e7859,)
            } else {
                let noise_metadata_schedule_560_e7863: f64 = (-(ctx.node_voltage(self.nodes[6]) - 0.0));
                let noise_metadata_schedule_560_e7865: f64 = (noise_metadata_schedule_560_e7863 - (ctx.node_voltage(self.nodes[6]) - 0.0));
                let noise_metadata_schedule_560_e7868: f64 = (1e-32 + noise_variable_139);
                let noise_metadata_schedule_560_e7869: f64 = (noise_metadata_schedule_560_e7868).sqrt();
                let noise_metadata_schedule_560_e7870: f64 = (noise_metadata_schedule_560_e7865 + noise_metadata_schedule_560_e7869);
                let noise_metadata_schedule_560_e7871: f64 = (0.5 * noise_metadata_schedule_560_e7870);
                let noise_metadata_schedule_560_e7872: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0) + noise_metadata_schedule_560_e7871);
                (noise_metadata_schedule_560_e7872,)
            }
        };
        (noise_metadata_schedule_560_e7873,)
    }
};
            let noise_metadata_schedule_560_e7875: f64 = (noise_variable_84 * noise_metadata_schedule_560_e7874);
            let noise_metadata_schedule_560_e7876: f64 = (noise_metadata_schedule_560_e7802 + noise_metadata_schedule_560_e7875);
            noise_variable_59 = noise_metadata_schedule_560_e7876;
        }
        if matches!(source_index, 6) {
            noise_variable_58 = noise_variable_11;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_562_e7880: f64 = if noise_variable_54 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_351 = noise_metadata_schedule_562_e7880;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_563_e7899,) = {
    if (noise_variable_351 != 0.0) {
        let noise_metadata_schedule_563_e7887: f64 = (noise_variable_59 * noise_variable_59);
        let noise_metadata_schedule_563_e7889: f64 = (noise_metadata_schedule_563_e7887 + noise_variable_57);
        let noise_metadata_schedule_563_e7891: f64 = (-1.0);
        let noise_metadata_schedule_563_e7893: f64 = (noise_metadata_schedule_563_e7891 * 0.1666666666666667);
        let noise_metadata_schedule_563_e7894: f64 = (noise_metadata_schedule_563_e7889).powf(noise_metadata_schedule_563_e7893);
        let noise_metadata_schedule_563_e7895: f64 = (noise_variable_54 * noise_metadata_schedule_563_e7894);
        let noise_metadata_schedule_563_e7896: f64 = (1.0 + noise_metadata_schedule_563_e7895);
        let noise_metadata_schedule_563_e7897: f64 = (noise_variable_11 / noise_metadata_schedule_563_e7896);
        (noise_metadata_schedule_563_e7897,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_563_e7899;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_564_e7902: f64 = (-1.0);
            let noise_metadata_schedule_564_e7905: f64 = (10.0 - noise_variable_79);
            let (noise_metadata_schedule_564_e7964,) = {
    if (noise_metadata_schedule_564_e7905 > 1e-16) {
        let noise_metadata_schedule_564_e7912: f64 = (10.0 - noise_variable_79);
        let noise_metadata_schedule_564_e7915: f64 = (10.0 - noise_variable_79);
        let noise_metadata_schedule_564_e7918: f64 = (10.0 - noise_variable_79);
        let noise_metadata_schedule_564_e7919: f64 = (noise_metadata_schedule_564_e7915 * noise_metadata_schedule_564_e7918);
        let noise_metadata_schedule_564_e7921: f64 = (noise_metadata_schedule_564_e7919 + 0.01);
        let noise_metadata_schedule_564_e7922: f64 = (noise_metadata_schedule_564_e7921).sqrt();
        let noise_metadata_schedule_564_e7923: f64 = (noise_metadata_schedule_564_e7912 + noise_metadata_schedule_564_e7922);
        let noise_metadata_schedule_564_e7924: f64 = (0.5 * noise_metadata_schedule_564_e7923);
        let noise_metadata_schedule_564_e7925: f64 = (10.0 - noise_metadata_schedule_564_e7924);
        (noise_metadata_schedule_564_e7925,)
    } else {
        let noise_metadata_schedule_564_e7928: f64 = (noise_variable_79 - 10.0);
        let (noise_metadata_schedule_564_e7963,) = {
            if (noise_metadata_schedule_564_e7928 > 1e-16) {
                let noise_metadata_schedule_564_e7934: f64 = (0.5 * 0.01);
                let noise_metadata_schedule_564_e7937: f64 = (noise_variable_79 - 10.0);
                let noise_metadata_schedule_564_e7940: f64 = (noise_variable_79 - 10.0);
                let noise_metadata_schedule_564_e7943: f64 = (noise_variable_79 - 10.0);
                let noise_metadata_schedule_564_e7944: f64 = (noise_metadata_schedule_564_e7940 * noise_metadata_schedule_564_e7943);
                let noise_metadata_schedule_564_e7946: f64 = (noise_metadata_schedule_564_e7944 + 0.01);
                let noise_metadata_schedule_564_e7947: f64 = (noise_metadata_schedule_564_e7946).sqrt();
                let noise_metadata_schedule_564_e7948: f64 = (noise_metadata_schedule_564_e7937 + noise_metadata_schedule_564_e7947);
                let noise_metadata_schedule_564_e7949: f64 = (noise_metadata_schedule_564_e7934 / noise_metadata_schedule_564_e7948);
                let noise_metadata_schedule_564_e7950: f64 = (10.0 - noise_metadata_schedule_564_e7949);
                (noise_metadata_schedule_564_e7950,)
            } else {
                let noise_metadata_schedule_564_e7955: f64 = (10.0 - noise_variable_79);
                let noise_metadata_schedule_564_e7958: f64 = (1e-32 + 0.01);
                let noise_metadata_schedule_564_e7959: f64 = (noise_metadata_schedule_564_e7958).sqrt();
                let noise_metadata_schedule_564_e7960: f64 = (noise_metadata_schedule_564_e7955 + noise_metadata_schedule_564_e7959);
                let noise_metadata_schedule_564_e7961: f64 = (0.5 * noise_metadata_schedule_564_e7960);
                let noise_metadata_schedule_564_e7962: f64 = (10.0 - noise_metadata_schedule_564_e7961);
                (noise_metadata_schedule_564_e7962,)
            }
        };
        (noise_metadata_schedule_564_e7963,)
    }
};
            let noise_metadata_schedule_564_e7965: f64 = (noise_metadata_schedule_564_e7902 * noise_metadata_schedule_564_e7964);
            let noise_metadata_schedule_564_e7966: f64 = (noise_metadata_schedule_564_e7965).exp();
            let noise_metadata_schedule_564_e7967: f64 = (noise_variable_25 * noise_metadata_schedule_564_e7966);
            noise_variable_100 = noise_metadata_schedule_564_e7967;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_565_e7969: f64 = (noise_variable_100).sqrt();
            noise_variable_101 = noise_metadata_schedule_565_e7969;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_566_e7972: f64 = (noise_variable_12 * noise_variable_58);
            let noise_metadata_schedule_566_e7974: f64 = (noise_metadata_schedule_566_e7972 * noise_variable_101);
            noise_variable_102 = noise_metadata_schedule_566_e7974;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_567_e7977: f64 = (-noise_variable_77);
            let noise_metadata_schedule_567_e7980: f64 = (noise_variable_77 * noise_variable_77);
            let noise_metadata_schedule_567_e7982: f64 = (noise_metadata_schedule_567_e7980 + 0.04);
            let noise_metadata_schedule_567_e7983: f64 = (noise_metadata_schedule_567_e7982).sqrt();
            let noise_metadata_schedule_567_e7984: f64 = (noise_metadata_schedule_567_e7977 + noise_metadata_schedule_567_e7983);
            let noise_metadata_schedule_567_e7985: f64 = (0.5 * noise_metadata_schedule_567_e7984);
            noise_variable_103 = noise_metadata_schedule_567_e7985;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_568_e7988: f64 = (noise_variable_70 * noise_variable_102);
            let noise_metadata_schedule_568_e7992: f64 = (params.p41 * noise_variable_103);
            let noise_metadata_schedule_568_e7993: f64 = (1.0 + noise_metadata_schedule_568_e7992);
            let noise_metadata_schedule_568_e7994: f64 = (noise_metadata_schedule_568_e7988 / noise_metadata_schedule_568_e7993);
            noise_variable_104 = noise_metadata_schedule_568_e7994;
        }
        if matches!(source_index, 6) {
            let noise_metadata_schedule_569_e7997: f64 = if params.p66 == 2.0 { 1.0 } else { 0.0 };
            noise_variable_352 = noise_metadata_schedule_569_e7997;
        }
        if matches!(source_index, 6) {
            let (noise_metadata_schedule_570_e8003,) = {
    if (noise_variable_352 != 0.0) {
        let noise_metadata_schedule_570_e8001: f64 = (noise_variable_71 * noise_variable_104);
        (noise_metadata_schedule_570_e8001,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_570_e8003;
        }
        if matches!(source_index, 1) {
            noise_variable_136 = 0.0;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_572_e8007: f64 = (params.p18 * params.p17);
            let noise_metadata_schedule_572_e8009: f64 = (-1.0);
            let noise_metadata_schedule_572_e8010: f64 = if noise_metadata_schedule_572_e8007 == noise_metadata_schedule_572_e8009 { 1.0 } else { 0.0 };
            noise_variable_353 = noise_metadata_schedule_572_e8010;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_573_e8016,) = {
    if (noise_variable_353 != 0.0) {
        let noise_metadata_schedule_573_e8014: f64 = (params.p18 * noise_variable_42);
        (noise_metadata_schedule_573_e8014,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_573_e8016;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_574_e8020: f64 = ((ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[1])) - noise_variable_136);
            let noise_metadata_schedule_574_e8021: f64 = (params.p17 * noise_metadata_schedule_574_e8020);
            let noise_metadata_schedule_574_e8023: f64 = (noise_metadata_schedule_574_e8021 * noise_variable_26);
            noise_variable_114 = noise_metadata_schedule_574_e8023;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_575_e8034: f64 = if ((params.p49 != 0.0) && ((noise_variable_126 > 0.0) || (noise_variable_138 > 0.0))) { 1.0 } else { 0.0 };
            noise_variable_354 = noise_metadata_schedule_575_e8034;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_576_e8036: f64 = (noise_variable_114).abs();
            let noise_metadata_schedule_576_e8038: f64 = if noise_metadata_schedule_576_e8036 <= noise_variable_113 { 1.0 } else { 0.0 };
            noise_variable_372 = noise_metadata_schedule_576_e8038;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_577_e8046,) = {
    if ((noise_variable_354 != 0.0) && (noise_variable_372 != 0.0)) {
        let noise_metadata_schedule_577_e8044: f64 = (noise_variable_114 / noise_variable_112);
        (noise_metadata_schedule_577_e8044,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_577_e8046;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_578_e8049: f64 = if noise_variable_114 > noise_variable_113 { 1.0 } else { 0.0 };
            noise_variable_373 = noise_metadata_schedule_578_e8049;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_579_e8066,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_579_e8058: f64 = (noise_variable_112 * 1.25);
        let noise_metadata_schedule_579_e8060: f64 = (noise_metadata_schedule_579_e8058 / noise_variable_116);
        let noise_metadata_schedule_579_e8062: f64 = (noise_metadata_schedule_579_e8060 - 1.0);
        let noise_metadata_schedule_579_e8064: f64 = (noise_metadata_schedule_579_e8062 / noise_variable_116);
        (noise_metadata_schedule_579_e8064,)
    } else {
        (noise_variable_367,)
    }
};
            noise_variable_367 = noise_metadata_schedule_579_e8066;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_580_e8083,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_580_e8075: f64 = (noise_variable_114 / noise_variable_112);
        let noise_metadata_schedule_580_e8079: f64 = (noise_variable_367 * noise_variable_114);
        let noise_metadata_schedule_580_e8080: f64 = (1.0 + noise_metadata_schedule_580_e8079);
        let noise_metadata_schedule_580_e8081: f64 = (noise_metadata_schedule_580_e8075 * noise_metadata_schedule_580_e8080);
        (noise_metadata_schedule_580_e8081,)
    } else {
        (noise_variable_368,)
    }
};
            noise_variable_368 = noise_metadata_schedule_580_e8083;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_581_e8086: f64 = if noise_variable_368 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_374 = noise_metadata_schedule_581_e8086;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_582_e8099,) = {
    if ((((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) && (noise_variable_374 != 0.0)) {
        let noise_metadata_schedule_582_e8096: f64 = (-noise_variable_368);
        let noise_metadata_schedule_582_e8097: f64 = (noise_metadata_schedule_582_e8096).exp();
        (noise_metadata_schedule_582_e8097,)
    } else {
        (noise_variable_366,)
    }
};
            noise_variable_366 = noise_metadata_schedule_582_e8099;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_583_e8133,) = {
    if ((((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) && (noise_variable_374 == 0.0)) {
        let noise_metadata_schedule_583_e8113: f64 = (noise_variable_368 - 460.51701859880916);
        let noise_metadata_schedule_583_e8118: f64 = (noise_variable_368 - 460.51701859880916);
        let noise_metadata_schedule_583_e8119: f64 = (0.5 * noise_metadata_schedule_583_e8118);
        let noise_metadata_schedule_583_e8123: f64 = (noise_variable_368 - 460.51701859880916);
        let noise_metadata_schedule_583_e8125: f64 = (noise_metadata_schedule_583_e8123 * 0.3333333333333333);
        let noise_metadata_schedule_583_e8126: f64 = (1.0 + noise_metadata_schedule_583_e8125);
        let noise_metadata_schedule_583_e8127: f64 = (noise_metadata_schedule_583_e8119 * noise_metadata_schedule_583_e8126);
        let noise_metadata_schedule_583_e8128: f64 = (1.0 + noise_metadata_schedule_583_e8127);
        let noise_metadata_schedule_583_e8129: f64 = (noise_metadata_schedule_583_e8113 * noise_metadata_schedule_583_e8128);
        let noise_metadata_schedule_583_e8130: f64 = (1.0 + noise_metadata_schedule_583_e8129);
        let noise_metadata_schedule_583_e8131: f64 = (1e-200 / noise_metadata_schedule_583_e8130);
        (noise_metadata_schedule_583_e8131,)
    } else {
        (noise_variable_366,)
    }
};
            noise_variable_366 = noise_metadata_schedule_583_e8133;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_584_e8144,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_584_e8142: f64 = (1.0 - noise_variable_366);
        (noise_metadata_schedule_584_e8142,)
    } else {
        (noise_variable_369,)
    }
};
            noise_variable_369 = noise_metadata_schedule_584_e8144;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_585_e8168,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_585_e8154: f64 = (0.5 * noise_variable_111);
        let noise_metadata_schedule_585_e8155: f64 = (noise_variable_114 + noise_metadata_schedule_585_e8154);
        let noise_metadata_schedule_585_e8160: f64 = (0.25 * noise_variable_111);
        let noise_metadata_schedule_585_e8161: f64 = (noise_variable_114 + noise_metadata_schedule_585_e8160);
        let noise_metadata_schedule_585_e8163: f64 = (noise_metadata_schedule_585_e8161 - noise_variable_369);
        let noise_metadata_schedule_585_e8164: f64 = (noise_metadata_schedule_585_e8163).sqrt();
        let noise_metadata_schedule_585_e8165: f64 = (noise_variable_110 * noise_metadata_schedule_585_e8164);
        let noise_metadata_schedule_585_e8166: f64 = (noise_metadata_schedule_585_e8155 - noise_metadata_schedule_585_e8165);
        (noise_metadata_schedule_585_e8166,)
    } else {
        (noise_variable_370,)
    }
};
            noise_variable_370 = noise_metadata_schedule_585_e8168;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_586_e8171: f64 = if noise_variable_370 < 460.51701859880916 { 1.0 } else { 0.0 };
            noise_variable_375 = noise_metadata_schedule_586_e8171;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_587_e8184,) = {
    if ((((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) && (noise_variable_375 != 0.0)) {
        let noise_metadata_schedule_587_e8181: f64 = (-noise_variable_370);
        let noise_metadata_schedule_587_e8182: f64 = (noise_metadata_schedule_587_e8181).exp();
        (noise_metadata_schedule_587_e8182,)
    } else {
        (noise_variable_362,)
    }
};
            noise_variable_362 = noise_metadata_schedule_587_e8184;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_588_e8218,) = {
    if ((((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) && (noise_variable_375 == 0.0)) {
        let noise_metadata_schedule_588_e8198: f64 = (noise_variable_370 - 460.51701859880916);
        let noise_metadata_schedule_588_e8203: f64 = (noise_variable_370 - 460.51701859880916);
        let noise_metadata_schedule_588_e8204: f64 = (0.5 * noise_metadata_schedule_588_e8203);
        let noise_metadata_schedule_588_e8208: f64 = (noise_variable_370 - 460.51701859880916);
        let noise_metadata_schedule_588_e8210: f64 = (noise_metadata_schedule_588_e8208 * 0.3333333333333333);
        let noise_metadata_schedule_588_e8211: f64 = (1.0 + noise_metadata_schedule_588_e8210);
        let noise_metadata_schedule_588_e8212: f64 = (noise_metadata_schedule_588_e8204 * noise_metadata_schedule_588_e8211);
        let noise_metadata_schedule_588_e8213: f64 = (1.0 + noise_metadata_schedule_588_e8212);
        let noise_metadata_schedule_588_e8214: f64 = (noise_metadata_schedule_588_e8198 * noise_metadata_schedule_588_e8213);
        let noise_metadata_schedule_588_e8215: f64 = (1.0 + noise_metadata_schedule_588_e8214);
        let noise_metadata_schedule_588_e8216: f64 = (1e-200 / noise_metadata_schedule_588_e8215);
        (noise_metadata_schedule_588_e8216,)
    } else {
        (noise_variable_362,)
    }
};
            noise_variable_362 = noise_metadata_schedule_588_e8218;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_589_e8233,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_589_e8228: f64 = (0.5 * noise_variable_111);
        let noise_metadata_schedule_589_e8230: f64 = (noise_metadata_schedule_589_e8228 * noise_variable_362);
        let noise_metadata_schedule_589_e8231: f64 = (1.0 - noise_metadata_schedule_589_e8230);
        (noise_metadata_schedule_589_e8231,)
    } else {
        (noise_variable_363,)
    }
};
            noise_variable_363 = noise_metadata_schedule_589_e8233;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_590_e8252,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_590_e8243: f64 = (noise_variable_114 - noise_variable_370);
        let noise_metadata_schedule_590_e8244: f64 = (2.0 * noise_metadata_schedule_590_e8243);
        let noise_metadata_schedule_590_e8248: f64 = (1.0 - noise_variable_362);
        let noise_metadata_schedule_590_e8249: f64 = (noise_variable_111 * noise_metadata_schedule_590_e8248);
        let noise_metadata_schedule_590_e8250: f64 = (noise_metadata_schedule_590_e8244 + noise_metadata_schedule_590_e8249);
        (noise_metadata_schedule_590_e8250,)
    } else {
        (noise_variable_364,)
    }
};
            noise_variable_364 = noise_metadata_schedule_590_e8252;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_591_e8275,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_591_e8261: f64 = (noise_variable_114 - noise_variable_370);
        let noise_metadata_schedule_591_e8264: f64 = (noise_variable_114 - noise_variable_370);
        let noise_metadata_schedule_591_e8265: f64 = (noise_metadata_schedule_591_e8261 * noise_metadata_schedule_591_e8264);
        let noise_metadata_schedule_591_e8269: f64 = (noise_variable_370 - 1.0);
        let noise_metadata_schedule_591_e8271: f64 = (noise_metadata_schedule_591_e8269 + noise_variable_362);
        let noise_metadata_schedule_591_e8272: f64 = (noise_variable_111 * noise_metadata_schedule_591_e8271);
        let noise_metadata_schedule_591_e8273: f64 = (noise_metadata_schedule_591_e8265 - noise_metadata_schedule_591_e8272);
        (noise_metadata_schedule_591_e8273,)
    } else {
        (noise_variable_365,)
    }
};
            noise_variable_365 = noise_metadata_schedule_591_e8275;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_592_e8292,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_592_e8284: f64 = (noise_variable_364 * noise_variable_364);
        let noise_metadata_schedule_592_e8287: f64 = (4.0 * noise_variable_363);
        let noise_metadata_schedule_592_e8289: f64 = (noise_metadata_schedule_592_e8287 * noise_variable_365);
        let noise_metadata_schedule_592_e8290: f64 = (noise_metadata_schedule_592_e8284 - noise_metadata_schedule_592_e8289);
        (noise_metadata_schedule_592_e8290,)
    } else {
        (noise_variable_366,)
    }
};
            noise_variable_366 = noise_metadata_schedule_592_e8292;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_593_e8308,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_593_e8301: f64 = (2.0 * noise_variable_365);
        let noise_metadata_schedule_593_e8304: f64 = (noise_variable_366).sqrt();
        let noise_metadata_schedule_593_e8305: f64 = (noise_variable_364 + noise_metadata_schedule_593_e8304);
        let noise_metadata_schedule_593_e8306: f64 = (noise_metadata_schedule_593_e8301 / noise_metadata_schedule_593_e8305);
        (noise_metadata_schedule_593_e8306,)
    } else {
        (noise_variable_371,)
    }
};
            noise_variable_371 = noise_metadata_schedule_593_e8308;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_594_e8319,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_594_e8317: f64 = (noise_variable_370 + noise_variable_371);
        (noise_metadata_schedule_594_e8317,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_594_e8319;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_595_e8330,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_595_e8328: f64 = (-noise_variable_114);
        (noise_metadata_schedule_595_e8328,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_595_e8330;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_596_e8344,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_596_e8340: f64 = (1.25 * noise_variable_355);
        let noise_metadata_schedule_596_e8342: f64 = (noise_metadata_schedule_596_e8340 / noise_variable_112);
        (noise_metadata_schedule_596_e8342,)
    } else {
        (noise_variable_356,)
    }
};
            noise_variable_356 = noise_metadata_schedule_596_e8344;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_597_e8369,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_597_e8355: f64 = (noise_variable_356 + 10.0);
        let noise_metadata_schedule_597_e8358: f64 = (noise_variable_356 - 6.0);
        let noise_metadata_schedule_597_e8361: f64 = (noise_variable_356 - 6.0);
        let noise_metadata_schedule_597_e8362: f64 = (noise_metadata_schedule_597_e8358 * noise_metadata_schedule_597_e8361);
        let noise_metadata_schedule_597_e8364: f64 = (noise_metadata_schedule_597_e8362 + 64.0);
        let noise_metadata_schedule_597_e8365: f64 = (noise_metadata_schedule_597_e8364).sqrt();
        let noise_metadata_schedule_597_e8366: f64 = (noise_metadata_schedule_597_e8355 - noise_metadata_schedule_597_e8365);
        let noise_metadata_schedule_597_e8367: f64 = (0.5 * noise_metadata_schedule_597_e8366);
        (noise_metadata_schedule_597_e8367,)
    } else {
        (noise_variable_357,)
    }
};
            noise_variable_357 = noise_metadata_schedule_597_e8369;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_598_e8391,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_598_e8379: f64 = (noise_variable_355 - noise_variable_357);
        let noise_metadata_schedule_598_e8382: f64 = (noise_variable_355 - noise_variable_357);
        let noise_metadata_schedule_598_e8383: f64 = (noise_metadata_schedule_598_e8379 * noise_metadata_schedule_598_e8382);
        let noise_metadata_schedule_598_e8387: f64 = (noise_variable_357 + 1.0);
        let noise_metadata_schedule_598_e8388: f64 = (noise_variable_111 * noise_metadata_schedule_598_e8387);
        let noise_metadata_schedule_598_e8389: f64 = (noise_metadata_schedule_598_e8383 + noise_metadata_schedule_598_e8388);
        (noise_metadata_schedule_598_e8389,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_598_e8391;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_599_e8407,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_599_e8402: f64 = (noise_variable_355 - noise_variable_357);
        let noise_metadata_schedule_599_e8403: f64 = (2.0 * noise_metadata_schedule_599_e8402);
        let noise_metadata_schedule_599_e8405: f64 = (noise_metadata_schedule_599_e8403 - noise_variable_111);
        (noise_metadata_schedule_599_e8405,)
    } else {
        (noise_variable_359,)
    }
};
            noise_variable_359 = noise_metadata_schedule_599_e8407;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_600_e8422,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_600_e8417: f64 = (noise_variable_358 / noise_variable_111);
        let noise_metadata_schedule_600_e8418: f64 = (noise_metadata_schedule_600_e8417).ln();
        let noise_metadata_schedule_600_e8420: f64 = (noise_metadata_schedule_600_e8418 - noise_variable_357);
        (noise_metadata_schedule_600_e8420,)
    } else {
        (noise_variable_360,)
    }
};
            noise_variable_360 = noise_metadata_schedule_600_e8422;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_601_e8434,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_601_e8432: f64 = (noise_variable_358 + noise_variable_359);
        (noise_metadata_schedule_601_e8432,)
    } else {
        (noise_variable_376,)
    }
};
            noise_variable_376 = noise_metadata_schedule_601_e8434;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_602_e8456,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_602_e8444: f64 = (noise_variable_376 * noise_variable_376);
        let noise_metadata_schedule_602_e8447: f64 = (0.5 * noise_variable_359);
        let noise_metadata_schedule_602_e8449: f64 = (noise_metadata_schedule_602_e8447 * noise_variable_359);
        let noise_metadata_schedule_602_e8451: f64 = (noise_metadata_schedule_602_e8449 - noise_variable_358);
        let noise_metadata_schedule_602_e8453: f64 = (noise_metadata_schedule_602_e8451 * noise_variable_360);
        let noise_metadata_schedule_602_e8454: f64 = (noise_metadata_schedule_602_e8444 + noise_metadata_schedule_602_e8453);
        (noise_metadata_schedule_602_e8454,)
    } else {
        (noise_variable_377,)
    }
};
            noise_variable_377 = noise_metadata_schedule_602_e8456;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_603_e8492,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_603_e8467: f64 = (noise_variable_358 * noise_variable_376);
        let noise_metadata_schedule_603_e8469: f64 = (noise_metadata_schedule_603_e8467 * noise_variable_360);
        let noise_metadata_schedule_603_e8473: f64 = (noise_variable_376 * noise_variable_360);
        let noise_metadata_schedule_603_e8475: f64 = (noise_metadata_schedule_603_e8473 * noise_variable_360);
        let noise_metadata_schedule_603_e8477: f64 = (noise_metadata_schedule_603_e8475 / noise_variable_377);
        let noise_metadata_schedule_603_e8479: f64 = (noise_metadata_schedule_603_e8477 * noise_variable_359);
        let noise_metadata_schedule_603_e8482: f64 = (noise_variable_359 * noise_variable_359);
        let noise_metadata_schedule_603_e8484: f64 = (noise_metadata_schedule_603_e8482 * 0.3333333333333333);
        let noise_metadata_schedule_603_e8486: f64 = (noise_metadata_schedule_603_e8484 - noise_variable_358);
        let noise_metadata_schedule_603_e8487: f64 = (noise_metadata_schedule_603_e8479 * noise_metadata_schedule_603_e8486);
        let noise_metadata_schedule_603_e8488: f64 = (noise_variable_377 + noise_metadata_schedule_603_e8487);
        let noise_metadata_schedule_603_e8489: f64 = (noise_metadata_schedule_603_e8469 / noise_metadata_schedule_603_e8488);
        let noise_metadata_schedule_603_e8490: f64 = (noise_variable_357 + noise_metadata_schedule_603_e8489);
        (noise_metadata_schedule_603_e8490,)
    } else {
        (noise_variable_361,)
    }
};
            noise_variable_361 = noise_metadata_schedule_603_e8492;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_604_e8494: f64 = (noise_variable_361).abs();
            let noise_metadata_schedule_604_e8496: f64 = if noise_metadata_schedule_604_e8494 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_378 = noise_metadata_schedule_604_e8496;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_605_e8509,) = {
    if ((((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) && (noise_variable_378 != 0.0)) {
        let noise_metadata_schedule_605_e8507: f64 = (noise_variable_361).exp();
        (noise_metadata_schedule_605_e8507,)
    } else {
        (noise_variable_362,)
    }
};
            noise_variable_362 = noise_metadata_schedule_605_e8509;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_606_e8512: f64 = (-230.25850929940458);
            let noise_metadata_schedule_606_e8513: f64 = if noise_variable_361 < noise_metadata_schedule_606_e8512 { 1.0 } else { 0.0 };
            noise_variable_379 = noise_metadata_schedule_606_e8513;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_607_e8553,) = {
    if (((((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) && (noise_variable_378 == 0.0)) && (noise_variable_379 != 0.0)) {
        let noise_metadata_schedule_607_e8529: f64 = (-230.25850929940458);
        let noise_metadata_schedule_607_e8531: f64 = (noise_metadata_schedule_607_e8529 - noise_variable_361);
        let noise_metadata_schedule_607_e8535: f64 = (-230.25850929940458);
        let noise_metadata_schedule_607_e8537: f64 = (noise_metadata_schedule_607_e8535 - noise_variable_361);
        let noise_metadata_schedule_607_e8538: f64 = (0.5 * noise_metadata_schedule_607_e8537);
        let noise_metadata_schedule_607_e8541: f64 = (-230.25850929940458);
        let noise_metadata_schedule_607_e8543: f64 = (noise_metadata_schedule_607_e8541 - noise_variable_361);
        let noise_metadata_schedule_607_e8545: f64 = (noise_metadata_schedule_607_e8543 * 0.3333333333333333);
        let noise_metadata_schedule_607_e8546: f64 = (1.0 + noise_metadata_schedule_607_e8545);
        let noise_metadata_schedule_607_e8547: f64 = (noise_metadata_schedule_607_e8538 * noise_metadata_schedule_607_e8546);
        let noise_metadata_schedule_607_e8548: f64 = (1.0 + noise_metadata_schedule_607_e8547);
        let noise_metadata_schedule_607_e8549: f64 = (noise_metadata_schedule_607_e8531 * noise_metadata_schedule_607_e8548);
        let noise_metadata_schedule_607_e8550: f64 = (1.0 + noise_metadata_schedule_607_e8549);
        let noise_metadata_schedule_607_e8551: f64 = (1e-100 / noise_metadata_schedule_607_e8550);
        (noise_metadata_schedule_607_e8551,)
    } else {
        (noise_variable_362,)
    }
};
            noise_variable_362 = noise_metadata_schedule_607_e8553;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_608_e8591,) = {
    if (((((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) && (noise_variable_378 == 0.0)) && (noise_variable_379 == 0.0)) {
        let noise_metadata_schedule_608_e8571: f64 = (noise_variable_361 - 230.25850929940458);
        let noise_metadata_schedule_608_e8576: f64 = (noise_variable_361 - 230.25850929940458);
        let noise_metadata_schedule_608_e8577: f64 = (0.5 * noise_metadata_schedule_608_e8576);
        let noise_metadata_schedule_608_e8581: f64 = (noise_variable_361 - 230.25850929940458);
        let noise_metadata_schedule_608_e8583: f64 = (noise_metadata_schedule_608_e8581 * 0.3333333333333333);
        let noise_metadata_schedule_608_e8584: f64 = (1.0 + noise_metadata_schedule_608_e8583);
        let noise_metadata_schedule_608_e8585: f64 = (noise_metadata_schedule_608_e8577 * noise_metadata_schedule_608_e8584);
        let noise_metadata_schedule_608_e8586: f64 = (1.0 + noise_metadata_schedule_608_e8585);
        let noise_metadata_schedule_608_e8587: f64 = (noise_metadata_schedule_608_e8571 * noise_metadata_schedule_608_e8586);
        let noise_metadata_schedule_608_e8588: f64 = (1.0 + noise_metadata_schedule_608_e8587);
        let noise_metadata_schedule_608_e8589: f64 = (1e100 * noise_metadata_schedule_608_e8588);
        (noise_metadata_schedule_608_e8589,)
    } else {
        (noise_variable_362,)
    }
};
            noise_variable_362 = noise_metadata_schedule_608_e8591;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_609_e8607,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_609_e8602: f64 = (0.5 * noise_variable_111);
        let noise_metadata_schedule_609_e8604: f64 = (noise_metadata_schedule_609_e8602 * noise_variable_362);
        let noise_metadata_schedule_609_e8605: f64 = (1.0 - noise_metadata_schedule_609_e8604);
        (noise_metadata_schedule_609_e8605,)
    } else {
        (noise_variable_363,)
    }
};
            noise_variable_363 = noise_metadata_schedule_609_e8607;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_610_e8627,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_610_e8618: f64 = (noise_variable_355 - noise_variable_361);
        let noise_metadata_schedule_610_e8619: f64 = (2.0 * noise_metadata_schedule_610_e8618);
        let noise_metadata_schedule_610_e8623: f64 = (noise_variable_362 - 1.0);
        let noise_metadata_schedule_610_e8624: f64 = (noise_variable_111 * noise_metadata_schedule_610_e8623);
        let noise_metadata_schedule_610_e8625: f64 = (noise_metadata_schedule_610_e8619 + noise_metadata_schedule_610_e8624);
        (noise_metadata_schedule_610_e8625,)
    } else {
        (noise_variable_364,)
    }
};
            noise_variable_364 = noise_metadata_schedule_610_e8627;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_611_e8651,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_611_e8637: f64 = (noise_variable_355 - noise_variable_361);
        let noise_metadata_schedule_611_e8640: f64 = (noise_variable_355 - noise_variable_361);
        let noise_metadata_schedule_611_e8641: f64 = (noise_metadata_schedule_611_e8637 * noise_metadata_schedule_611_e8640);
        let noise_metadata_schedule_611_e8645: f64 = (noise_variable_361 + 1.0);
        let noise_metadata_schedule_611_e8647: f64 = (noise_metadata_schedule_611_e8645 - noise_variable_362);
        let noise_metadata_schedule_611_e8648: f64 = (noise_variable_111 * noise_metadata_schedule_611_e8647);
        let noise_metadata_schedule_611_e8649: f64 = (noise_metadata_schedule_611_e8641 + noise_metadata_schedule_611_e8648);
        (noise_metadata_schedule_611_e8649,)
    } else {
        (noise_variable_365,)
    }
};
            noise_variable_365 = noise_metadata_schedule_611_e8651;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_612_e8669,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_612_e8661: f64 = (noise_variable_364 * noise_variable_364);
        let noise_metadata_schedule_612_e8664: f64 = (4.0 * noise_variable_363);
        let noise_metadata_schedule_612_e8666: f64 = (noise_metadata_schedule_612_e8664 * noise_variable_365);
        let noise_metadata_schedule_612_e8667: f64 = (noise_metadata_schedule_612_e8661 - noise_metadata_schedule_612_e8666);
        (noise_metadata_schedule_612_e8667,)
    } else {
        (noise_variable_366,)
    }
};
            noise_variable_366 = noise_metadata_schedule_612_e8669;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_613_e8686,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_613_e8679: f64 = (2.0 * noise_variable_365);
        let noise_metadata_schedule_613_e8682: f64 = (noise_variable_366).sqrt();
        let noise_metadata_schedule_613_e8683: f64 = (noise_variable_364 + noise_metadata_schedule_613_e8682);
        let noise_metadata_schedule_613_e8684: f64 = (noise_metadata_schedule_613_e8679 / noise_metadata_schedule_613_e8683);
        (noise_metadata_schedule_613_e8684,)
    } else {
        (noise_variable_369,)
    }
};
            noise_variable_369 = noise_metadata_schedule_613_e8686;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_614_e8699,) = {
    if (((noise_variable_354 != 0.0) && (noise_variable_372 == 0.0)) && (noise_variable_373 == 0.0)) {
        let noise_metadata_schedule_614_e8696: f64 = (noise_variable_361 + noise_variable_369);
        let noise_metadata_schedule_614_e8697: f64 = (-noise_metadata_schedule_614_e8696);
        (noise_metadata_schedule_614_e8697,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_614_e8699;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_615_e8707,) = {
    if (noise_variable_354 != 0.0) {
        let noise_metadata_schedule_615_e8704: f64 = (noise_variable_114 - noise_variable_115);
        let noise_metadata_schedule_615_e8705: f64 = (noise_variable_25 * noise_metadata_schedule_615_e8704);
        (noise_metadata_schedule_615_e8705,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_615_e8707;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_616_e8712,) = {
    if (noise_variable_354 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_616_e8712;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_617_e8717,) = {
    if (noise_variable_354 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_617_e8717;
        }
        if matches!(source_index, 0) {
            noise_variable_4 = 0.0;
        }
        if matches!(source_index, 1) {
            noise_variable_5 = 0.0;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_620_e8726: f64 = if ((noise_variable_126 > 0.0) || (noise_variable_138 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_380 = noise_metadata_schedule_620_e8726;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_621_e8734,) = {
    if ((params.p49 != 0.0) && (noise_variable_380 != 0.0)) {
        let noise_metadata_schedule_621_e8732: f64 = (params.p17 * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[1])));
        (noise_metadata_schedule_621_e8732,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_621_e8734;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_622_e8740,) = {
    if ((params.p49 != 0.0) && (noise_variable_380 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_622_e8740;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_623_e8747: f64 = if ((params.p18 == 1.0) && (noise_variable_138 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_391 = noise_metadata_schedule_623_e8747;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_624_e8759,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) {
        let noise_metadata_schedule_624_e8755: f64 = (params.p17 * noise_variable_118);
        let noise_metadata_schedule_624_e8757: f64 = (noise_metadata_schedule_624_e8755 + noise_variable_129);
        (noise_metadata_schedule_624_e8757,)
    } else {
        (noise_variable_382,)
    }
};
            noise_variable_382 = noise_metadata_schedule_624_e8759;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_625_e8828,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) {
        let noise_metadata_schedule_625_e8767: f64 = (-noise_variable_382);
        let (noise_metadata_schedule_625_e8826,) = {
            if (noise_metadata_schedule_625_e8767 > 1e-16) {
                let noise_metadata_schedule_625_e8774: f64 = (-noise_variable_382);
                let noise_metadata_schedule_625_e8777: f64 = (-noise_variable_382);
                let noise_metadata_schedule_625_e8780: f64 = (-noise_variable_382);
                let noise_metadata_schedule_625_e8781: f64 = (noise_metadata_schedule_625_e8777 * noise_metadata_schedule_625_e8780);
                let noise_metadata_schedule_625_e8783: f64 = (noise_metadata_schedule_625_e8781 + 0.01);
                let noise_metadata_schedule_625_e8784: f64 = (noise_metadata_schedule_625_e8783).sqrt();
                let noise_metadata_schedule_625_e8785: f64 = (noise_metadata_schedule_625_e8774 + noise_metadata_schedule_625_e8784);
                let noise_metadata_schedule_625_e8786: f64 = (0.5 * noise_metadata_schedule_625_e8785);
                let noise_metadata_schedule_625_e8787: f64 = (noise_variable_382 + noise_metadata_schedule_625_e8786);
                (noise_metadata_schedule_625_e8787,)
            } else {
                let noise_metadata_schedule_625_e8790: f64 = noise_variable_382;
                let (noise_metadata_schedule_625_e8825,) = {
                    if (noise_metadata_schedule_625_e8790 > 1e-16) {
                        let noise_metadata_schedule_625_e8796: f64 = (0.5 * 0.01);
                        let noise_metadata_schedule_625_e8799: f64 = noise_variable_382;
                        let noise_metadata_schedule_625_e8802: f64 = noise_variable_382;
                        let noise_metadata_schedule_625_e8805: f64 = noise_variable_382;
                        let noise_metadata_schedule_625_e8806: f64 = (noise_metadata_schedule_625_e8802 * noise_metadata_schedule_625_e8805);
                        let noise_metadata_schedule_625_e8808: f64 = (noise_metadata_schedule_625_e8806 + 0.01);
                        let noise_metadata_schedule_625_e8809: f64 = (noise_metadata_schedule_625_e8808).sqrt();
                        let noise_metadata_schedule_625_e8810: f64 = (noise_metadata_schedule_625_e8799 + noise_metadata_schedule_625_e8809);
                        let noise_metadata_schedule_625_e8811: f64 = (noise_metadata_schedule_625_e8796 / noise_metadata_schedule_625_e8810);
                        let noise_metadata_schedule_625_e8812: f64 = (noise_variable_382 + noise_metadata_schedule_625_e8811);
                        (noise_metadata_schedule_625_e8812,)
                    } else {
                        let noise_metadata_schedule_625_e8817: f64 = (-noise_variable_382);
                        let noise_metadata_schedule_625_e8820: f64 = (1e-32 + 0.01);
                        let noise_metadata_schedule_625_e8821: f64 = (noise_metadata_schedule_625_e8820).sqrt();
                        let noise_metadata_schedule_625_e8822: f64 = (noise_metadata_schedule_625_e8817 + noise_metadata_schedule_625_e8821);
                        let noise_metadata_schedule_625_e8823: f64 = (0.5 * noise_metadata_schedule_625_e8822);
                        let noise_metadata_schedule_625_e8824: f64 = (noise_variable_382 + noise_metadata_schedule_625_e8823);
                        (noise_metadata_schedule_625_e8824,)
                    }
                };
                (noise_metadata_schedule_625_e8825,)
            }
        };
        (noise_metadata_schedule_625_e8826,)
    } else {
        (noise_variable_383,)
    }
};
            noise_variable_383 = noise_metadata_schedule_625_e8828;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_626_e8843,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) {
        let noise_metadata_schedule_626_e8836: f64 = (noise_variable_118 * noise_variable_118);
        let noise_metadata_schedule_626_e8838: f64 = (noise_metadata_schedule_626_e8836 + 1e-6);
        let noise_metadata_schedule_626_e8839: f64 = (noise_metadata_schedule_626_e8838).sqrt();
        let noise_metadata_schedule_626_e8841: f64 = (noise_metadata_schedule_626_e8839 * noise_variable_131);
        (noise_metadata_schedule_626_e8841,)
    } else {
        (noise_variable_384,)
    }
};
            noise_variable_384 = noise_metadata_schedule_626_e8843;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_627_e8846: f64 = if params.p64 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_392 = noise_metadata_schedule_627_e8846;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_628_e8917,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_392 != 0.0)) {
        let noise_metadata_schedule_628_e8856: f64 = (noise_variable_130 - noise_variable_384);
        let (noise_metadata_schedule_628_e8915,) = {
            if (noise_metadata_schedule_628_e8856 > 1e-16) {
                let noise_metadata_schedule_628_e8863: f64 = (noise_variable_130 - noise_variable_384);
                let noise_metadata_schedule_628_e8866: f64 = (noise_variable_130 - noise_variable_384);
                let noise_metadata_schedule_628_e8869: f64 = (noise_variable_130 - noise_variable_384);
                let noise_metadata_schedule_628_e8870: f64 = (noise_metadata_schedule_628_e8866 * noise_metadata_schedule_628_e8869);
                let noise_metadata_schedule_628_e8872: f64 = (noise_metadata_schedule_628_e8870 + 1e-6);
                let noise_metadata_schedule_628_e8873: f64 = (noise_metadata_schedule_628_e8872).sqrt();
                let noise_metadata_schedule_628_e8874: f64 = (noise_metadata_schedule_628_e8863 + noise_metadata_schedule_628_e8873);
                let noise_metadata_schedule_628_e8875: f64 = (0.5 * noise_metadata_schedule_628_e8874);
                let noise_metadata_schedule_628_e8876: f64 = (noise_variable_130 - noise_metadata_schedule_628_e8875);
                (noise_metadata_schedule_628_e8876,)
            } else {
                let noise_metadata_schedule_628_e8879: f64 = (noise_variable_384 - noise_variable_130);
                let (noise_metadata_schedule_628_e8914,) = {
                    if (noise_metadata_schedule_628_e8879 > 1e-16) {
                        let noise_metadata_schedule_628_e8885: f64 = (0.5 * 1e-6);
                        let noise_metadata_schedule_628_e8888: f64 = (noise_variable_384 - noise_variable_130);
                        let noise_metadata_schedule_628_e8891: f64 = (noise_variable_384 - noise_variable_130);
                        let noise_metadata_schedule_628_e8894: f64 = (noise_variable_384 - noise_variable_130);
                        let noise_metadata_schedule_628_e8895: f64 = (noise_metadata_schedule_628_e8891 * noise_metadata_schedule_628_e8894);
                        let noise_metadata_schedule_628_e8897: f64 = (noise_metadata_schedule_628_e8895 + 1e-6);
                        let noise_metadata_schedule_628_e8898: f64 = (noise_metadata_schedule_628_e8897).sqrt();
                        let noise_metadata_schedule_628_e8899: f64 = (noise_metadata_schedule_628_e8888 + noise_metadata_schedule_628_e8898);
                        let noise_metadata_schedule_628_e8900: f64 = (noise_metadata_schedule_628_e8885 / noise_metadata_schedule_628_e8899);
                        let noise_metadata_schedule_628_e8901: f64 = (noise_variable_130 - noise_metadata_schedule_628_e8900);
                        (noise_metadata_schedule_628_e8901,)
                    } else {
                        let noise_metadata_schedule_628_e8906: f64 = (noise_variable_130 - noise_variable_384);
                        let noise_metadata_schedule_628_e8909: f64 = (1e-32 + 1e-6);
                        let noise_metadata_schedule_628_e8910: f64 = (noise_metadata_schedule_628_e8909).sqrt();
                        let noise_metadata_schedule_628_e8911: f64 = (noise_metadata_schedule_628_e8906 + noise_metadata_schedule_628_e8910);
                        let noise_metadata_schedule_628_e8912: f64 = (0.5 * noise_metadata_schedule_628_e8911);
                        let noise_metadata_schedule_628_e8913: f64 = (noise_variable_130 - noise_metadata_schedule_628_e8912);
                        (noise_metadata_schedule_628_e8913,)
                    }
                };
                (noise_metadata_schedule_628_e8914,)
            }
        };
        (noise_metadata_schedule_628_e8915,)
    } else {
        (noise_variable_384,)
    }
};
            noise_variable_384 = noise_metadata_schedule_628_e8917;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_629_e8920: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_393 = noise_metadata_schedule_629_e8920;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_630_e8941,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_393 != 0.0)) {
        let noise_metadata_schedule_630_e8930: f64 = (params.p17 * noise_variable_115);
        let noise_metadata_schedule_630_e8933: f64 = (noise_variable_42 - noise_variable_134);
        let noise_metadata_schedule_630_e8935: f64 = (noise_metadata_schedule_630_e8933 + noise_variable_383);
        let noise_metadata_schedule_630_e8937: f64 = (noise_metadata_schedule_630_e8935 * noise_variable_26);
        let noise_metadata_schedule_630_e8938: f64 = (noise_metadata_schedule_630_e8930 + noise_metadata_schedule_630_e8937);
        let noise_metadata_schedule_630_e8939: f64 = (-noise_metadata_schedule_630_e8938);
        (noise_metadata_schedule_630_e8939,)
    } else {
        (noise_variable_385,)
    }
};
            noise_variable_385 = noise_metadata_schedule_630_e8941;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_631_e8963,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_393 == 0.0)) {
        let noise_metadata_schedule_631_e8952: f64 = (params.p17 * noise_variable_115);
        let noise_metadata_schedule_631_e8955: f64 = (noise_variable_42 - noise_variable_93);
        let noise_metadata_schedule_631_e8957: f64 = (noise_metadata_schedule_631_e8955 + noise_variable_383);
        let noise_metadata_schedule_631_e8959: f64 = (noise_metadata_schedule_631_e8957 * noise_variable_26);
        let noise_metadata_schedule_631_e8960: f64 = (noise_metadata_schedule_631_e8952 + noise_metadata_schedule_631_e8959);
        let noise_metadata_schedule_631_e8961: f64 = (-noise_metadata_schedule_631_e8960);
        (noise_metadata_schedule_631_e8961,)
    } else {
        (noise_variable_385,)
    }
};
            noise_variable_385 = noise_metadata_schedule_631_e8963;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_632_e8966: f64 = if noise_variable_385 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_394 = noise_metadata_schedule_632_e8966;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_633_e8980,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_394 != 0.0)) {
        let noise_metadata_schedule_633_e8976: f64 = (noise_variable_385).exp();
        let noise_metadata_schedule_633_e8977: f64 = (1.0 + noise_metadata_schedule_633_e8976);
        let noise_metadata_schedule_633_e8978: f64 = (noise_metadata_schedule_633_e8977).ln();
        (noise_metadata_schedule_633_e8978,)
    } else {
        (noise_variable_390,)
    }
};
            noise_variable_390 = noise_metadata_schedule_633_e8980;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_634_e8991,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_394 == 0.0)) {
        (noise_variable_385,)
    } else {
        (noise_variable_390,)
    }
};
            noise_variable_390 = noise_metadata_schedule_634_e8991;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_635_e9005,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) {
        let noise_metadata_schedule_635_e9000: f64 = (params.p17 * noise_variable_127);
        let noise_metadata_schedule_635_e9002: f64 = (noise_metadata_schedule_635_e9000 * noise_variable_26);
        let noise_metadata_schedule_635_e9003: f64 = (noise_variable_385 + noise_metadata_schedule_635_e9002);
        (noise_metadata_schedule_635_e9003,)
    } else {
        (noise_variable_386,)
    }
};
            noise_variable_386 = noise_metadata_schedule_635_e9005;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_636_e9008: f64 = if noise_variable_386 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_395 = noise_metadata_schedule_636_e9008;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_637_e9022,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_395 != 0.0)) {
        let noise_metadata_schedule_637_e9018: f64 = (noise_variable_386).exp();
        let noise_metadata_schedule_637_e9019: f64 = (1.0 + noise_metadata_schedule_637_e9018);
        let noise_metadata_schedule_637_e9020: f64 = (noise_metadata_schedule_637_e9019).ln();
        (noise_metadata_schedule_637_e9020,)
    } else {
        (noise_variable_387,)
    }
};
            noise_variable_387 = noise_metadata_schedule_637_e9022;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_638_e9033,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_395 == 0.0)) {
        (noise_variable_386,)
    } else {
        (noise_variable_387,)
    }
};
            noise_variable_387 = noise_metadata_schedule_638_e9033;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_639_e9052,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) {
        let noise_metadata_schedule_639_e9041: f64 = (-1.5);
        let noise_metadata_schedule_639_e9046: f64 = (params.p64 * noise_variable_384);
        let noise_metadata_schedule_639_e9047: f64 = (params.p63 + noise_metadata_schedule_639_e9046);
        let noise_metadata_schedule_639_e9048: f64 = (noise_variable_384 * noise_metadata_schedule_639_e9047);
        let noise_metadata_schedule_639_e9049: f64 = (noise_metadata_schedule_639_e9041 + noise_metadata_schedule_639_e9048);
        let noise_metadata_schedule_639_e9050: f64 = (noise_variable_133 * noise_metadata_schedule_639_e9049);
        (noise_metadata_schedule_639_e9050,)
    } else {
        (noise_variable_389,)
    }
};
            noise_variable_389 = noise_metadata_schedule_639_e9052;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_640_e9055: f64 = if noise_variable_389 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_396 = noise_metadata_schedule_640_e9055;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_641_e9079,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_396 != 0.0)) {
        let noise_metadata_schedule_641_e9068: f64 = (0.5 * noise_variable_389);
        let noise_metadata_schedule_641_e9072: f64 = (noise_variable_389 * 0.3333333333333333);
        let noise_metadata_schedule_641_e9073: f64 = (1.0 + noise_metadata_schedule_641_e9072);
        let noise_metadata_schedule_641_e9074: f64 = (noise_metadata_schedule_641_e9068 * noise_metadata_schedule_641_e9073);
        let noise_metadata_schedule_641_e9075: f64 = (1.0 + noise_metadata_schedule_641_e9074);
        let noise_metadata_schedule_641_e9076: f64 = (noise_variable_389 * noise_metadata_schedule_641_e9075);
        let noise_metadata_schedule_641_e9077: f64 = (1.0 + noise_metadata_schedule_641_e9076);
        (noise_metadata_schedule_641_e9077,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_641_e9079;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_642_e9082: f64 = (-230.25850929940458);
            let noise_metadata_schedule_642_e9083: f64 = if noise_variable_389 > noise_metadata_schedule_642_e9082 { 1.0 } else { 0.0 };
            noise_variable_397 = noise_metadata_schedule_642_e9083;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_643_e9097,) = {
    if (((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_396 == 0.0)) && (noise_variable_397 != 0.0)) {
        let noise_metadata_schedule_643_e9095: f64 = (noise_variable_389).exp();
        (noise_metadata_schedule_643_e9095,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_643_e9097;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_644_e9136,) = {
    if (((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) && (noise_variable_396 == 0.0)) && (noise_variable_397 == 0.0)) {
        let noise_metadata_schedule_644_e9112: f64 = (-230.25850929940458);
        let noise_metadata_schedule_644_e9114: f64 = (noise_metadata_schedule_644_e9112 - noise_variable_389);
        let noise_metadata_schedule_644_e9118: f64 = (-230.25850929940458);
        let noise_metadata_schedule_644_e9120: f64 = (noise_metadata_schedule_644_e9118 - noise_variable_389);
        let noise_metadata_schedule_644_e9121: f64 = (0.5 * noise_metadata_schedule_644_e9120);
        let noise_metadata_schedule_644_e9124: f64 = (-230.25850929940458);
        let noise_metadata_schedule_644_e9126: f64 = (noise_metadata_schedule_644_e9124 - noise_variable_389);
        let noise_metadata_schedule_644_e9128: f64 = (noise_metadata_schedule_644_e9126 * 0.3333333333333333);
        let noise_metadata_schedule_644_e9129: f64 = (1.0 + noise_metadata_schedule_644_e9128);
        let noise_metadata_schedule_644_e9130: f64 = (noise_metadata_schedule_644_e9121 * noise_metadata_schedule_644_e9129);
        let noise_metadata_schedule_644_e9131: f64 = (1.0 + noise_metadata_schedule_644_e9130);
        let noise_metadata_schedule_644_e9132: f64 = (noise_metadata_schedule_644_e9114 * noise_metadata_schedule_644_e9131);
        let noise_metadata_schedule_644_e9133: f64 = (1.0 + noise_metadata_schedule_644_e9132);
        let noise_metadata_schedule_644_e9134: f64 = (1e-100 / noise_metadata_schedule_644_e9133);
        (noise_metadata_schedule_644_e9134,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_644_e9136;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_645_e9152,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_391 != 0.0)) {
        let noise_metadata_schedule_645_e9144: f64 = (noise_variable_138 * noise_variable_388);
        let noise_metadata_schedule_645_e9146: f64 = (noise_metadata_schedule_645_e9144 * params.p17);
        let noise_metadata_schedule_645_e9149: f64 = (noise_variable_387 - noise_variable_390);
        let noise_metadata_schedule_645_e9150: f64 = (noise_metadata_schedule_645_e9146 * noise_metadata_schedule_645_e9149);
        (noise_metadata_schedule_645_e9150,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_645_e9152;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_646_e9155: f64 = if noise_variable_126 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_398 = noise_metadata_schedule_646_e9155;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_647_e9167,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_647_e9163: f64 = (params.p17 * noise_variable_118);
        let noise_metadata_schedule_647_e9165: f64 = (noise_metadata_schedule_647_e9163 + noise_variable_121);
        (noise_metadata_schedule_647_e9165,)
    } else {
        (noise_variable_381,)
    }
};
            noise_variable_381 = noise_metadata_schedule_647_e9167;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_648_e9236,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_648_e9175: f64 = noise_variable_381;
        let (noise_metadata_schedule_648_e9234,) = {
            if (noise_metadata_schedule_648_e9175 > 1e-16) {
                let noise_metadata_schedule_648_e9182: f64 = noise_variable_381;
                let noise_metadata_schedule_648_e9185: f64 = noise_variable_381;
                let noise_metadata_schedule_648_e9188: f64 = noise_variable_381;
                let noise_metadata_schedule_648_e9189: f64 = (noise_metadata_schedule_648_e9185 * noise_metadata_schedule_648_e9188);
                let noise_metadata_schedule_648_e9191: f64 = (noise_metadata_schedule_648_e9189 + 0.01);
                let noise_metadata_schedule_648_e9192: f64 = (noise_metadata_schedule_648_e9191).sqrt();
                let noise_metadata_schedule_648_e9193: f64 = (noise_metadata_schedule_648_e9182 + noise_metadata_schedule_648_e9192);
                let noise_metadata_schedule_648_e9194: f64 = (0.5 * noise_metadata_schedule_648_e9193);
                let noise_metadata_schedule_648_e9195: f64 = (noise_variable_381 - noise_metadata_schedule_648_e9194);
                (noise_metadata_schedule_648_e9195,)
            } else {
                let noise_metadata_schedule_648_e9198: f64 = (-noise_variable_381);
                let (noise_metadata_schedule_648_e9233,) = {
                    if (noise_metadata_schedule_648_e9198 > 1e-16) {
                        let noise_metadata_schedule_648_e9204: f64 = (0.5 * 0.01);
                        let noise_metadata_schedule_648_e9207: f64 = (-noise_variable_381);
                        let noise_metadata_schedule_648_e9210: f64 = (-noise_variable_381);
                        let noise_metadata_schedule_648_e9213: f64 = (-noise_variable_381);
                        let noise_metadata_schedule_648_e9214: f64 = (noise_metadata_schedule_648_e9210 * noise_metadata_schedule_648_e9213);
                        let noise_metadata_schedule_648_e9216: f64 = (noise_metadata_schedule_648_e9214 + 0.01);
                        let noise_metadata_schedule_648_e9217: f64 = (noise_metadata_schedule_648_e9216).sqrt();
                        let noise_metadata_schedule_648_e9218: f64 = (noise_metadata_schedule_648_e9207 + noise_metadata_schedule_648_e9217);
                        let noise_metadata_schedule_648_e9219: f64 = (noise_metadata_schedule_648_e9204 / noise_metadata_schedule_648_e9218);
                        let noise_metadata_schedule_648_e9220: f64 = (noise_variable_381 - noise_metadata_schedule_648_e9219);
                        (noise_metadata_schedule_648_e9220,)
                    } else {
                        let noise_metadata_schedule_648_e9225: f64 = noise_variable_381;
                        let noise_metadata_schedule_648_e9228: f64 = (1e-32 + 0.01);
                        let noise_metadata_schedule_648_e9229: f64 = (noise_metadata_schedule_648_e9228).sqrt();
                        let noise_metadata_schedule_648_e9230: f64 = (noise_metadata_schedule_648_e9225 + noise_metadata_schedule_648_e9229);
                        let noise_metadata_schedule_648_e9231: f64 = (0.5 * noise_metadata_schedule_648_e9230);
                        let noise_metadata_schedule_648_e9232: f64 = (noise_variable_381 - noise_metadata_schedule_648_e9231);
                        (noise_metadata_schedule_648_e9232,)
                    }
                };
                (noise_metadata_schedule_648_e9233,)
            }
        };
        (noise_metadata_schedule_648_e9234,)
    } else {
        (noise_variable_383,)
    }
};
            noise_variable_383 = noise_metadata_schedule_648_e9236;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_649_e9251,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_649_e9244: f64 = (noise_variable_118 * noise_variable_118);
        let noise_metadata_schedule_649_e9246: f64 = (noise_metadata_schedule_649_e9244 + 1e-6);
        let noise_metadata_schedule_649_e9247: f64 = (noise_metadata_schedule_649_e9246).sqrt();
        let noise_metadata_schedule_649_e9249: f64 = (noise_metadata_schedule_649_e9247 * noise_variable_124);
        (noise_metadata_schedule_649_e9249,)
    } else {
        (noise_variable_384,)
    }
};
            noise_variable_384 = noise_metadata_schedule_649_e9251;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_650_e9254: f64 = if params.p59 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_399 = noise_metadata_schedule_650_e9254;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_651_e9325,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_399 != 0.0)) {
        let noise_metadata_schedule_651_e9264: f64 = (noise_variable_120 - noise_variable_384);
        let (noise_metadata_schedule_651_e9323,) = {
            if (noise_metadata_schedule_651_e9264 > 1e-16) {
                let noise_metadata_schedule_651_e9271: f64 = (noise_variable_120 - noise_variable_384);
                let noise_metadata_schedule_651_e9274: f64 = (noise_variable_120 - noise_variable_384);
                let noise_metadata_schedule_651_e9277: f64 = (noise_variable_120 - noise_variable_384);
                let noise_metadata_schedule_651_e9278: f64 = (noise_metadata_schedule_651_e9274 * noise_metadata_schedule_651_e9277);
                let noise_metadata_schedule_651_e9280: f64 = (noise_metadata_schedule_651_e9278 + 1e-6);
                let noise_metadata_schedule_651_e9281: f64 = (noise_metadata_schedule_651_e9280).sqrt();
                let noise_metadata_schedule_651_e9282: f64 = (noise_metadata_schedule_651_e9271 + noise_metadata_schedule_651_e9281);
                let noise_metadata_schedule_651_e9283: f64 = (0.5 * noise_metadata_schedule_651_e9282);
                let noise_metadata_schedule_651_e9284: f64 = (noise_variable_120 - noise_metadata_schedule_651_e9283);
                (noise_metadata_schedule_651_e9284,)
            } else {
                let noise_metadata_schedule_651_e9287: f64 = (noise_variable_384 - noise_variable_120);
                let (noise_metadata_schedule_651_e9322,) = {
                    if (noise_metadata_schedule_651_e9287 > 1e-16) {
                        let noise_metadata_schedule_651_e9293: f64 = (0.5 * 1e-6);
                        let noise_metadata_schedule_651_e9296: f64 = (noise_variable_384 - noise_variable_120);
                        let noise_metadata_schedule_651_e9299: f64 = (noise_variable_384 - noise_variable_120);
                        let noise_metadata_schedule_651_e9302: f64 = (noise_variable_384 - noise_variable_120);
                        let noise_metadata_schedule_651_e9303: f64 = (noise_metadata_schedule_651_e9299 * noise_metadata_schedule_651_e9302);
                        let noise_metadata_schedule_651_e9305: f64 = (noise_metadata_schedule_651_e9303 + 1e-6);
                        let noise_metadata_schedule_651_e9306: f64 = (noise_metadata_schedule_651_e9305).sqrt();
                        let noise_metadata_schedule_651_e9307: f64 = (noise_metadata_schedule_651_e9296 + noise_metadata_schedule_651_e9306);
                        let noise_metadata_schedule_651_e9308: f64 = (noise_metadata_schedule_651_e9293 / noise_metadata_schedule_651_e9307);
                        let noise_metadata_schedule_651_e9309: f64 = (noise_variable_120 - noise_metadata_schedule_651_e9308);
                        (noise_metadata_schedule_651_e9309,)
                    } else {
                        let noise_metadata_schedule_651_e9314: f64 = (noise_variable_120 - noise_variable_384);
                        let noise_metadata_schedule_651_e9317: f64 = (1e-32 + 1e-6);
                        let noise_metadata_schedule_651_e9318: f64 = (noise_metadata_schedule_651_e9317).sqrt();
                        let noise_metadata_schedule_651_e9319: f64 = (noise_metadata_schedule_651_e9314 + noise_metadata_schedule_651_e9318);
                        let noise_metadata_schedule_651_e9320: f64 = (0.5 * noise_metadata_schedule_651_e9319);
                        let noise_metadata_schedule_651_e9321: f64 = (noise_variable_120 - noise_metadata_schedule_651_e9320);
                        (noise_metadata_schedule_651_e9321,)
                    }
                };
                (noise_metadata_schedule_651_e9322,)
            }
        };
        (noise_metadata_schedule_651_e9323,)
    } else {
        (noise_variable_384,)
    }
};
            noise_variable_384 = noise_metadata_schedule_651_e9325;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_652_e9328: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_400 = noise_metadata_schedule_652_e9328;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_653_e9346,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_400 != 0.0)) {
        let noise_metadata_schedule_653_e9338: f64 = (params.p17 * noise_variable_115);
        let noise_metadata_schedule_653_e9341: f64 = (noise_variable_383 - noise_variable_134);
        let noise_metadata_schedule_653_e9343: f64 = (noise_metadata_schedule_653_e9341 * noise_variable_26);
        let noise_metadata_schedule_653_e9344: f64 = (noise_metadata_schedule_653_e9338 + noise_metadata_schedule_653_e9343);
        (noise_metadata_schedule_653_e9344,)
    } else {
        (noise_variable_385,)
    }
};
            noise_variable_385 = noise_metadata_schedule_653_e9346;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_654_e9365,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_400 == 0.0)) {
        let noise_metadata_schedule_654_e9357: f64 = (params.p17 * noise_variable_115);
        let noise_metadata_schedule_654_e9360: f64 = (noise_variable_383 - noise_variable_93);
        let noise_metadata_schedule_654_e9362: f64 = (noise_metadata_schedule_654_e9360 * noise_variable_26);
        let noise_metadata_schedule_654_e9363: f64 = (noise_metadata_schedule_654_e9357 + noise_metadata_schedule_654_e9362);
        (noise_metadata_schedule_654_e9363,)
    } else {
        (noise_variable_385,)
    }
};
            noise_variable_385 = noise_metadata_schedule_654_e9365;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_655_e9368: f64 = if noise_variable_385 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_401 = noise_metadata_schedule_655_e9368;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_656_e9382,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_401 != 0.0)) {
        let noise_metadata_schedule_656_e9378: f64 = (noise_variable_385).exp();
        let noise_metadata_schedule_656_e9379: f64 = (1.0 + noise_metadata_schedule_656_e9378);
        let noise_metadata_schedule_656_e9380: f64 = (noise_metadata_schedule_656_e9379).ln();
        (noise_metadata_schedule_656_e9380,)
    } else {
        (noise_variable_390,)
    }
};
            noise_variable_390 = noise_metadata_schedule_656_e9382;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_657_e9393,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_401 == 0.0)) {
        (noise_variable_385,)
    } else {
        (noise_variable_390,)
    }
};
            noise_variable_390 = noise_metadata_schedule_657_e9393;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_658_e9407,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_658_e9402: f64 = (params.p17 * noise_variable_127);
        let noise_metadata_schedule_658_e9404: f64 = (noise_metadata_schedule_658_e9402 * noise_variable_26);
        let noise_metadata_schedule_658_e9405: f64 = (noise_variable_385 - noise_metadata_schedule_658_e9404);
        (noise_metadata_schedule_658_e9405,)
    } else {
        (noise_variable_386,)
    }
};
            noise_variable_386 = noise_metadata_schedule_658_e9407;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_659_e9410: f64 = if noise_variable_386 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_402 = noise_metadata_schedule_659_e9410;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_660_e9424,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_402 != 0.0)) {
        let noise_metadata_schedule_660_e9420: f64 = (noise_variable_386).exp();
        let noise_metadata_schedule_660_e9421: f64 = (1.0 + noise_metadata_schedule_660_e9420);
        let noise_metadata_schedule_660_e9422: f64 = (noise_metadata_schedule_660_e9421).ln();
        (noise_metadata_schedule_660_e9422,)
    } else {
        (noise_variable_387,)
    }
};
            noise_variable_387 = noise_metadata_schedule_660_e9424;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_661_e9435,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_402 == 0.0)) {
        (noise_variable_386,)
    } else {
        (noise_variable_387,)
    }
};
            noise_variable_387 = noise_metadata_schedule_661_e9435;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_662_e9454,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_662_e9443: f64 = (-1.5);
        let noise_metadata_schedule_662_e9448: f64 = (params.p59 * noise_variable_384);
        let noise_metadata_schedule_662_e9449: f64 = (params.p58 + noise_metadata_schedule_662_e9448);
        let noise_metadata_schedule_662_e9450: f64 = (noise_variable_384 * noise_metadata_schedule_662_e9449);
        let noise_metadata_schedule_662_e9451: f64 = (noise_metadata_schedule_662_e9443 + noise_metadata_schedule_662_e9450);
        let noise_metadata_schedule_662_e9452: f64 = (noise_variable_123 * noise_metadata_schedule_662_e9451);
        (noise_metadata_schedule_662_e9452,)
    } else {
        (noise_variable_389,)
    }
};
            noise_variable_389 = noise_metadata_schedule_662_e9454;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_663_e9456: f64 = (noise_variable_389).abs();
            let noise_metadata_schedule_663_e9458: f64 = if noise_metadata_schedule_663_e9456 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_403 = noise_metadata_schedule_663_e9458;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_664_e9469,) = {
    if ((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_403 != 0.0)) {
        let noise_metadata_schedule_664_e9467: f64 = (noise_variable_389).exp();
        (noise_metadata_schedule_664_e9467,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_664_e9469;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_665_e9472: f64 = (-230.25850929940458);
            let noise_metadata_schedule_665_e9473: f64 = if noise_variable_389 < noise_metadata_schedule_665_e9472 { 1.0 } else { 0.0 };
            noise_variable_404 = noise_metadata_schedule_665_e9473;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_666_e9511,) = {
    if (((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_403 == 0.0)) && (noise_variable_404 != 0.0)) {
        let noise_metadata_schedule_666_e9487: f64 = (-230.25850929940458);
        let noise_metadata_schedule_666_e9489: f64 = (noise_metadata_schedule_666_e9487 - noise_variable_389);
        let noise_metadata_schedule_666_e9493: f64 = (-230.25850929940458);
        let noise_metadata_schedule_666_e9495: f64 = (noise_metadata_schedule_666_e9493 - noise_variable_389);
        let noise_metadata_schedule_666_e9496: f64 = (0.5 * noise_metadata_schedule_666_e9495);
        let noise_metadata_schedule_666_e9499: f64 = (-230.25850929940458);
        let noise_metadata_schedule_666_e9501: f64 = (noise_metadata_schedule_666_e9499 - noise_variable_389);
        let noise_metadata_schedule_666_e9503: f64 = (noise_metadata_schedule_666_e9501 * 0.3333333333333333);
        let noise_metadata_schedule_666_e9504: f64 = (1.0 + noise_metadata_schedule_666_e9503);
        let noise_metadata_schedule_666_e9505: f64 = (noise_metadata_schedule_666_e9496 * noise_metadata_schedule_666_e9504);
        let noise_metadata_schedule_666_e9506: f64 = (1.0 + noise_metadata_schedule_666_e9505);
        let noise_metadata_schedule_666_e9507: f64 = (noise_metadata_schedule_666_e9489 * noise_metadata_schedule_666_e9506);
        let noise_metadata_schedule_666_e9508: f64 = (1.0 + noise_metadata_schedule_666_e9507);
        let noise_metadata_schedule_666_e9509: f64 = (1e-100 / noise_metadata_schedule_666_e9508);
        (noise_metadata_schedule_666_e9509,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_666_e9511;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_667_e9547,) = {
    if (((((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) && (noise_variable_403 == 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_667_e9527: f64 = (noise_variable_389 - 230.25850929940458);
        let noise_metadata_schedule_667_e9532: f64 = (noise_variable_389 - 230.25850929940458);
        let noise_metadata_schedule_667_e9533: f64 = (0.5 * noise_metadata_schedule_667_e9532);
        let noise_metadata_schedule_667_e9537: f64 = (noise_variable_389 - 230.25850929940458);
        let noise_metadata_schedule_667_e9539: f64 = (noise_metadata_schedule_667_e9537 * 0.3333333333333333);
        let noise_metadata_schedule_667_e9540: f64 = (1.0 + noise_metadata_schedule_667_e9539);
        let noise_metadata_schedule_667_e9541: f64 = (noise_metadata_schedule_667_e9533 * noise_metadata_schedule_667_e9540);
        let noise_metadata_schedule_667_e9542: f64 = (1.0 + noise_metadata_schedule_667_e9541);
        let noise_metadata_schedule_667_e9543: f64 = (noise_metadata_schedule_667_e9527 * noise_metadata_schedule_667_e9542);
        let noise_metadata_schedule_667_e9544: f64 = (1.0 + noise_metadata_schedule_667_e9543);
        let noise_metadata_schedule_667_e9545: f64 = (1e100 * noise_metadata_schedule_667_e9544);
        (noise_metadata_schedule_667_e9545,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_667_e9547;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_668_e9565,) = {
    if (((params.p49 != 0.0) && (noise_variable_380 != 0.0)) && (noise_variable_398 != 0.0)) {
        let noise_metadata_schedule_668_e9556: f64 = (noise_variable_126 * noise_variable_388);
        let noise_metadata_schedule_668_e9558: f64 = (noise_metadata_schedule_668_e9556 * params.p17);
        let noise_metadata_schedule_668_e9561: f64 = (noise_variable_390 - noise_variable_387);
        let noise_metadata_schedule_668_e9562: f64 = (noise_metadata_schedule_668_e9558 * noise_metadata_schedule_668_e9561);
        let noise_metadata_schedule_668_e9563: f64 = (noise_variable_5 + noise_metadata_schedule_668_e9562);
        (noise_metadata_schedule_668_e9563,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_668_e9565;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_669_e9572: f64 = if ((noise_variable_125 > 0.0) || (noise_variable_137 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_405 = noise_metadata_schedule_669_e9572;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_670_e9580,) = {
    if ((params.p49 != 0.0) && (noise_variable_405 != 0.0)) {
        let noise_metadata_schedule_670_e9578: f64 = (params.p17 * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])));
        (noise_metadata_schedule_670_e9578,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_670_e9580;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_671_e9590,) = {
    if ((params.p49 != 0.0) && (noise_variable_405 != 0.0)) {
        let noise_metadata_schedule_671_e9586: f64 = (noise_variable_78 - noise_variable_95);
        let noise_metadata_schedule_671_e9588: f64 = (noise_metadata_schedule_671_e9586 * noise_variable_25);
        (noise_metadata_schedule_671_e9588,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_671_e9590;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_672_e9596,) = {
    if ((params.p49 != 0.0) && (noise_variable_405 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_672_e9596;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_673_e9603: f64 = if ((params.p18 == 1.0) && (noise_variable_137 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_416 = noise_metadata_schedule_673_e9603;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_674_e9615,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) {
        let noise_metadata_schedule_674_e9611: f64 = (params.p17 * noise_variable_117);
        let noise_metadata_schedule_674_e9613: f64 = (noise_metadata_schedule_674_e9611 + noise_variable_129);
        (noise_metadata_schedule_674_e9613,)
    } else {
        (noise_variable_407,)
    }
};
            noise_variable_407 = noise_metadata_schedule_674_e9615;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_675_e9684,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) {
        let noise_metadata_schedule_675_e9623: f64 = (-noise_variable_407);
        let (noise_metadata_schedule_675_e9682,) = {
            if (noise_metadata_schedule_675_e9623 > 1e-16) {
                let noise_metadata_schedule_675_e9630: f64 = (-noise_variable_407);
                let noise_metadata_schedule_675_e9633: f64 = (-noise_variable_407);
                let noise_metadata_schedule_675_e9636: f64 = (-noise_variable_407);
                let noise_metadata_schedule_675_e9637: f64 = (noise_metadata_schedule_675_e9633 * noise_metadata_schedule_675_e9636);
                let noise_metadata_schedule_675_e9639: f64 = (noise_metadata_schedule_675_e9637 + 0.01);
                let noise_metadata_schedule_675_e9640: f64 = (noise_metadata_schedule_675_e9639).sqrt();
                let noise_metadata_schedule_675_e9641: f64 = (noise_metadata_schedule_675_e9630 + noise_metadata_schedule_675_e9640);
                let noise_metadata_schedule_675_e9642: f64 = (0.5 * noise_metadata_schedule_675_e9641);
                let noise_metadata_schedule_675_e9643: f64 = (noise_variable_407 + noise_metadata_schedule_675_e9642);
                (noise_metadata_schedule_675_e9643,)
            } else {
                let noise_metadata_schedule_675_e9646: f64 = noise_variable_407;
                let (noise_metadata_schedule_675_e9681,) = {
                    if (noise_metadata_schedule_675_e9646 > 1e-16) {
                        let noise_metadata_schedule_675_e9652: f64 = (0.5 * 0.01);
                        let noise_metadata_schedule_675_e9655: f64 = noise_variable_407;
                        let noise_metadata_schedule_675_e9658: f64 = noise_variable_407;
                        let noise_metadata_schedule_675_e9661: f64 = noise_variable_407;
                        let noise_metadata_schedule_675_e9662: f64 = (noise_metadata_schedule_675_e9658 * noise_metadata_schedule_675_e9661);
                        let noise_metadata_schedule_675_e9664: f64 = (noise_metadata_schedule_675_e9662 + 0.01);
                        let noise_metadata_schedule_675_e9665: f64 = (noise_metadata_schedule_675_e9664).sqrt();
                        let noise_metadata_schedule_675_e9666: f64 = (noise_metadata_schedule_675_e9655 + noise_metadata_schedule_675_e9665);
                        let noise_metadata_schedule_675_e9667: f64 = (noise_metadata_schedule_675_e9652 / noise_metadata_schedule_675_e9666);
                        let noise_metadata_schedule_675_e9668: f64 = (noise_variable_407 + noise_metadata_schedule_675_e9667);
                        (noise_metadata_schedule_675_e9668,)
                    } else {
                        let noise_metadata_schedule_675_e9673: f64 = (-noise_variable_407);
                        let noise_metadata_schedule_675_e9676: f64 = (1e-32 + 0.01);
                        let noise_metadata_schedule_675_e9677: f64 = (noise_metadata_schedule_675_e9676).sqrt();
                        let noise_metadata_schedule_675_e9678: f64 = (noise_metadata_schedule_675_e9673 + noise_metadata_schedule_675_e9677);
                        let noise_metadata_schedule_675_e9679: f64 = (0.5 * noise_metadata_schedule_675_e9678);
                        let noise_metadata_schedule_675_e9680: f64 = (noise_variable_407 + noise_metadata_schedule_675_e9679);
                        (noise_metadata_schedule_675_e9680,)
                    }
                };
                (noise_metadata_schedule_675_e9681,)
            }
        };
        (noise_metadata_schedule_675_e9682,)
    } else {
        (noise_variable_408,)
    }
};
            noise_variable_408 = noise_metadata_schedule_675_e9684;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_676_e9699,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) {
        let noise_metadata_schedule_676_e9692: f64 = (noise_variable_117 * noise_variable_117);
        let noise_metadata_schedule_676_e9694: f64 = (noise_metadata_schedule_676_e9692 + 1e-6);
        let noise_metadata_schedule_676_e9695: f64 = (noise_metadata_schedule_676_e9694).sqrt();
        let noise_metadata_schedule_676_e9697: f64 = (noise_metadata_schedule_676_e9695 * noise_variable_131);
        (noise_metadata_schedule_676_e9697,)
    } else {
        (noise_variable_409,)
    }
};
            noise_variable_409 = noise_metadata_schedule_676_e9699;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_677_e9702: f64 = if params.p64 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_417 = noise_metadata_schedule_677_e9702;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_678_e9773,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_417 != 0.0)) {
        let noise_metadata_schedule_678_e9712: f64 = (noise_variable_130 - noise_variable_409);
        let (noise_metadata_schedule_678_e9771,) = {
            if (noise_metadata_schedule_678_e9712 > 1e-16) {
                let noise_metadata_schedule_678_e9719: f64 = (noise_variable_130 - noise_variable_409);
                let noise_metadata_schedule_678_e9722: f64 = (noise_variable_130 - noise_variable_409);
                let noise_metadata_schedule_678_e9725: f64 = (noise_variable_130 - noise_variable_409);
                let noise_metadata_schedule_678_e9726: f64 = (noise_metadata_schedule_678_e9722 * noise_metadata_schedule_678_e9725);
                let noise_metadata_schedule_678_e9728: f64 = (noise_metadata_schedule_678_e9726 + 1e-6);
                let noise_metadata_schedule_678_e9729: f64 = (noise_metadata_schedule_678_e9728).sqrt();
                let noise_metadata_schedule_678_e9730: f64 = (noise_metadata_schedule_678_e9719 + noise_metadata_schedule_678_e9729);
                let noise_metadata_schedule_678_e9731: f64 = (0.5 * noise_metadata_schedule_678_e9730);
                let noise_metadata_schedule_678_e9732: f64 = (noise_variable_130 - noise_metadata_schedule_678_e9731);
                (noise_metadata_schedule_678_e9732,)
            } else {
                let noise_metadata_schedule_678_e9735: f64 = (noise_variable_409 - noise_variable_130);
                let (noise_metadata_schedule_678_e9770,) = {
                    if (noise_metadata_schedule_678_e9735 > 1e-16) {
                        let noise_metadata_schedule_678_e9741: f64 = (0.5 * 1e-6);
                        let noise_metadata_schedule_678_e9744: f64 = (noise_variable_409 - noise_variable_130);
                        let noise_metadata_schedule_678_e9747: f64 = (noise_variable_409 - noise_variable_130);
                        let noise_metadata_schedule_678_e9750: f64 = (noise_variable_409 - noise_variable_130);
                        let noise_metadata_schedule_678_e9751: f64 = (noise_metadata_schedule_678_e9747 * noise_metadata_schedule_678_e9750);
                        let noise_metadata_schedule_678_e9753: f64 = (noise_metadata_schedule_678_e9751 + 1e-6);
                        let noise_metadata_schedule_678_e9754: f64 = (noise_metadata_schedule_678_e9753).sqrt();
                        let noise_metadata_schedule_678_e9755: f64 = (noise_metadata_schedule_678_e9744 + noise_metadata_schedule_678_e9754);
                        let noise_metadata_schedule_678_e9756: f64 = (noise_metadata_schedule_678_e9741 / noise_metadata_schedule_678_e9755);
                        let noise_metadata_schedule_678_e9757: f64 = (noise_variable_130 - noise_metadata_schedule_678_e9756);
                        (noise_metadata_schedule_678_e9757,)
                    } else {
                        let noise_metadata_schedule_678_e9762: f64 = (noise_variable_130 - noise_variable_409);
                        let noise_metadata_schedule_678_e9765: f64 = (1e-32 + 1e-6);
                        let noise_metadata_schedule_678_e9766: f64 = (noise_metadata_schedule_678_e9765).sqrt();
                        let noise_metadata_schedule_678_e9767: f64 = (noise_metadata_schedule_678_e9762 + noise_metadata_schedule_678_e9766);
                        let noise_metadata_schedule_678_e9768: f64 = (0.5 * noise_metadata_schedule_678_e9767);
                        let noise_metadata_schedule_678_e9769: f64 = (noise_variable_130 - noise_metadata_schedule_678_e9768);
                        (noise_metadata_schedule_678_e9769,)
                    }
                };
                (noise_metadata_schedule_678_e9770,)
            }
        };
        (noise_metadata_schedule_678_e9771,)
    } else {
        (noise_variable_409,)
    }
};
            noise_variable_409 = noise_metadata_schedule_678_e9773;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_679_e9776: f64 = if 1.0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_418 = noise_metadata_schedule_679_e9776;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_680_e9797,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_418 != 0.0)) {
        let noise_metadata_schedule_680_e9786: f64 = (params.p17 * noise_variable_95);
        let noise_metadata_schedule_680_e9789: f64 = (noise_variable_42 - noise_variable_134);
        let noise_metadata_schedule_680_e9791: f64 = (noise_metadata_schedule_680_e9789 + noise_variable_408);
        let noise_metadata_schedule_680_e9793: f64 = (noise_metadata_schedule_680_e9791 * noise_variable_26);
        let noise_metadata_schedule_680_e9794: f64 = (noise_metadata_schedule_680_e9786 + noise_metadata_schedule_680_e9793);
        let noise_metadata_schedule_680_e9795: f64 = (-noise_metadata_schedule_680_e9794);
        (noise_metadata_schedule_680_e9795,)
    } else {
        (noise_variable_410,)
    }
};
            noise_variable_410 = noise_metadata_schedule_680_e9797;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_681_e9819,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_418 == 0.0)) {
        let noise_metadata_schedule_681_e9808: f64 = (params.p17 * noise_variable_95);
        let noise_metadata_schedule_681_e9811: f64 = (noise_variable_42 - noise_variable_93);
        let noise_metadata_schedule_681_e9813: f64 = (noise_metadata_schedule_681_e9811 + noise_variable_408);
        let noise_metadata_schedule_681_e9815: f64 = (noise_metadata_schedule_681_e9813 * noise_variable_26);
        let noise_metadata_schedule_681_e9816: f64 = (noise_metadata_schedule_681_e9808 + noise_metadata_schedule_681_e9815);
        let noise_metadata_schedule_681_e9817: f64 = (-noise_metadata_schedule_681_e9816);
        (noise_metadata_schedule_681_e9817,)
    } else {
        (noise_variable_410,)
    }
};
            noise_variable_410 = noise_metadata_schedule_681_e9819;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_682_e9822: f64 = if noise_variable_410 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_419 = noise_metadata_schedule_682_e9822;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_683_e9836,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_683_e9832: f64 = (noise_variable_410).exp();
        let noise_metadata_schedule_683_e9833: f64 = (1.0 + noise_metadata_schedule_683_e9832);
        let noise_metadata_schedule_683_e9834: f64 = (noise_metadata_schedule_683_e9833).ln();
        (noise_metadata_schedule_683_e9834,)
    } else {
        (noise_variable_415,)
    }
};
            noise_variable_415 = noise_metadata_schedule_683_e9836;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_684_e9847,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_419 == 0.0)) {
        (noise_variable_410,)
    } else {
        (noise_variable_415,)
    }
};
            noise_variable_415 = noise_metadata_schedule_684_e9847;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_685_e9861,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) {
        let noise_metadata_schedule_685_e9856: f64 = (params.p17 * noise_variable_128);
        let noise_metadata_schedule_685_e9858: f64 = (noise_metadata_schedule_685_e9856 * noise_variable_26);
        let noise_metadata_schedule_685_e9859: f64 = (noise_variable_410 + noise_metadata_schedule_685_e9858);
        (noise_metadata_schedule_685_e9859,)
    } else {
        (noise_variable_411,)
    }
};
            noise_variable_411 = noise_metadata_schedule_685_e9861;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_686_e9864: f64 = if noise_variable_411 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_420 = noise_metadata_schedule_686_e9864;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_687_e9878,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_420 != 0.0)) {
        let noise_metadata_schedule_687_e9874: f64 = (noise_variable_411).exp();
        let noise_metadata_schedule_687_e9875: f64 = (1.0 + noise_metadata_schedule_687_e9874);
        let noise_metadata_schedule_687_e9876: f64 = (noise_metadata_schedule_687_e9875).ln();
        (noise_metadata_schedule_687_e9876,)
    } else {
        (noise_variable_412,)
    }
};
            noise_variable_412 = noise_metadata_schedule_687_e9878;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_688_e9889,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_420 == 0.0)) {
        (noise_variable_411,)
    } else {
        (noise_variable_412,)
    }
};
            noise_variable_412 = noise_metadata_schedule_688_e9889;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_689_e9908,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) {
        let noise_metadata_schedule_689_e9897: f64 = (-1.5);
        let noise_metadata_schedule_689_e9902: f64 = (params.p64 * noise_variable_409);
        let noise_metadata_schedule_689_e9903: f64 = (params.p63 + noise_metadata_schedule_689_e9902);
        let noise_metadata_schedule_689_e9904: f64 = (noise_variable_409 * noise_metadata_schedule_689_e9903);
        let noise_metadata_schedule_689_e9905: f64 = (noise_metadata_schedule_689_e9897 + noise_metadata_schedule_689_e9904);
        let noise_metadata_schedule_689_e9906: f64 = (noise_variable_132 * noise_metadata_schedule_689_e9905);
        (noise_metadata_schedule_689_e9906,)
    } else {
        (noise_variable_414,)
    }
};
            noise_variable_414 = noise_metadata_schedule_689_e9908;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_690_e9911: f64 = if noise_variable_414 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_421 = noise_metadata_schedule_690_e9911;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_691_e9935,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_421 != 0.0)) {
        let noise_metadata_schedule_691_e9924: f64 = (0.5 * noise_variable_414);
        let noise_metadata_schedule_691_e9928: f64 = (noise_variable_414 * 0.3333333333333333);
        let noise_metadata_schedule_691_e9929: f64 = (1.0 + noise_metadata_schedule_691_e9928);
        let noise_metadata_schedule_691_e9930: f64 = (noise_metadata_schedule_691_e9924 * noise_metadata_schedule_691_e9929);
        let noise_metadata_schedule_691_e9931: f64 = (1.0 + noise_metadata_schedule_691_e9930);
        let noise_metadata_schedule_691_e9932: f64 = (noise_variable_414 * noise_metadata_schedule_691_e9931);
        let noise_metadata_schedule_691_e9933: f64 = (1.0 + noise_metadata_schedule_691_e9932);
        (noise_metadata_schedule_691_e9933,)
    } else {
        (noise_variable_413,)
    }
};
            noise_variable_413 = noise_metadata_schedule_691_e9935;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_692_e9938: f64 = (-230.25850929940458);
            let noise_metadata_schedule_692_e9939: f64 = if noise_variable_414 > noise_metadata_schedule_692_e9938 { 1.0 } else { 0.0 };
            noise_variable_422 = noise_metadata_schedule_692_e9939;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_693_e9953,) = {
    if (((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_421 == 0.0)) && (noise_variable_422 != 0.0)) {
        let noise_metadata_schedule_693_e9951: f64 = (noise_variable_414).exp();
        (noise_metadata_schedule_693_e9951,)
    } else {
        (noise_variable_413,)
    }
};
            noise_variable_413 = noise_metadata_schedule_693_e9953;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_694_e9992,) = {
    if (((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) && (noise_variable_421 == 0.0)) && (noise_variable_422 == 0.0)) {
        let noise_metadata_schedule_694_e9968: f64 = (-230.25850929940458);
        let noise_metadata_schedule_694_e9970: f64 = (noise_metadata_schedule_694_e9968 - noise_variable_414);
        let noise_metadata_schedule_694_e9974: f64 = (-230.25850929940458);
        let noise_metadata_schedule_694_e9976: f64 = (noise_metadata_schedule_694_e9974 - noise_variable_414);
        let noise_metadata_schedule_694_e9977: f64 = (0.5 * noise_metadata_schedule_694_e9976);
        let noise_metadata_schedule_694_e9980: f64 = (-230.25850929940458);
        let noise_metadata_schedule_694_e9982: f64 = (noise_metadata_schedule_694_e9980 - noise_variable_414);
        let noise_metadata_schedule_694_e9984: f64 = (noise_metadata_schedule_694_e9982 * 0.3333333333333333);
        let noise_metadata_schedule_694_e9985: f64 = (1.0 + noise_metadata_schedule_694_e9984);
        let noise_metadata_schedule_694_e9986: f64 = (noise_metadata_schedule_694_e9977 * noise_metadata_schedule_694_e9985);
        let noise_metadata_schedule_694_e9987: f64 = (1.0 + noise_metadata_schedule_694_e9986);
        let noise_metadata_schedule_694_e9988: f64 = (noise_metadata_schedule_694_e9970 * noise_metadata_schedule_694_e9987);
        let noise_metadata_schedule_694_e9989: f64 = (1.0 + noise_metadata_schedule_694_e9988);
        let noise_metadata_schedule_694_e9990: f64 = (1e-100 / noise_metadata_schedule_694_e9989);
        (noise_metadata_schedule_694_e9990,)
    } else {
        (noise_variable_413,)
    }
};
            noise_variable_413 = noise_metadata_schedule_694_e9992;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_695_e10008,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_416 != 0.0)) {
        let noise_metadata_schedule_695_e10000: f64 = (noise_variable_137 * noise_variable_413);
        let noise_metadata_schedule_695_e10002: f64 = (noise_metadata_schedule_695_e10000 * params.p17);
        let noise_metadata_schedule_695_e10005: f64 = (noise_variable_412 - noise_variable_415);
        let noise_metadata_schedule_695_e10006: f64 = (noise_metadata_schedule_695_e10002 * noise_metadata_schedule_695_e10005);
        (noise_metadata_schedule_695_e10006,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_695_e10008;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_696_e10011: f64 = if noise_variable_125 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_423 = noise_metadata_schedule_696_e10011;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_697_e10023,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) {
        let noise_metadata_schedule_697_e10019: f64 = (params.p17 * noise_variable_117);
        let noise_metadata_schedule_697_e10021: f64 = (noise_metadata_schedule_697_e10019 + noise_variable_121);
        (noise_metadata_schedule_697_e10021,)
    } else {
        (noise_variable_406,)
    }
};
            noise_variable_406 = noise_metadata_schedule_697_e10023;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_698_e10092,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) {
        let noise_metadata_schedule_698_e10031: f64 = noise_variable_406;
        let (noise_metadata_schedule_698_e10090,) = {
            if (noise_metadata_schedule_698_e10031 > 1e-16) {
                let noise_metadata_schedule_698_e10038: f64 = noise_variable_406;
                let noise_metadata_schedule_698_e10041: f64 = noise_variable_406;
                let noise_metadata_schedule_698_e10044: f64 = noise_variable_406;
                let noise_metadata_schedule_698_e10045: f64 = (noise_metadata_schedule_698_e10041 * noise_metadata_schedule_698_e10044);
                let noise_metadata_schedule_698_e10047: f64 = (noise_metadata_schedule_698_e10045 + 0.01);
                let noise_metadata_schedule_698_e10048: f64 = (noise_metadata_schedule_698_e10047).sqrt();
                let noise_metadata_schedule_698_e10049: f64 = (noise_metadata_schedule_698_e10038 + noise_metadata_schedule_698_e10048);
                let noise_metadata_schedule_698_e10050: f64 = (0.5 * noise_metadata_schedule_698_e10049);
                let noise_metadata_schedule_698_e10051: f64 = (noise_variable_406 - noise_metadata_schedule_698_e10050);
                (noise_metadata_schedule_698_e10051,)
            } else {
                let noise_metadata_schedule_698_e10054: f64 = (-noise_variable_406);
                let (noise_metadata_schedule_698_e10089,) = {
                    if (noise_metadata_schedule_698_e10054 > 1e-16) {
                        let noise_metadata_schedule_698_e10060: f64 = (0.5 * 0.01);
                        let noise_metadata_schedule_698_e10063: f64 = (-noise_variable_406);
                        let noise_metadata_schedule_698_e10066: f64 = (-noise_variable_406);
                        let noise_metadata_schedule_698_e10069: f64 = (-noise_variable_406);
                        let noise_metadata_schedule_698_e10070: f64 = (noise_metadata_schedule_698_e10066 * noise_metadata_schedule_698_e10069);
                        let noise_metadata_schedule_698_e10072: f64 = (noise_metadata_schedule_698_e10070 + 0.01);
                        let noise_metadata_schedule_698_e10073: f64 = (noise_metadata_schedule_698_e10072).sqrt();
                        let noise_metadata_schedule_698_e10074: f64 = (noise_metadata_schedule_698_e10063 + noise_metadata_schedule_698_e10073);
                        let noise_metadata_schedule_698_e10075: f64 = (noise_metadata_schedule_698_e10060 / noise_metadata_schedule_698_e10074);
                        let noise_metadata_schedule_698_e10076: f64 = (noise_variable_406 - noise_metadata_schedule_698_e10075);
                        (noise_metadata_schedule_698_e10076,)
                    } else {
                        let noise_metadata_schedule_698_e10081: f64 = noise_variable_406;
                        let noise_metadata_schedule_698_e10084: f64 = (1e-32 + 0.01);
                        let noise_metadata_schedule_698_e10085: f64 = (noise_metadata_schedule_698_e10084).sqrt();
                        let noise_metadata_schedule_698_e10086: f64 = (noise_metadata_schedule_698_e10081 + noise_metadata_schedule_698_e10085);
                        let noise_metadata_schedule_698_e10087: f64 = (0.5 * noise_metadata_schedule_698_e10086);
                        let noise_metadata_schedule_698_e10088: f64 = (noise_variable_406 - noise_metadata_schedule_698_e10087);
                        (noise_metadata_schedule_698_e10088,)
                    }
                };
                (noise_metadata_schedule_698_e10089,)
            }
        };
        (noise_metadata_schedule_698_e10090,)
    } else {
        (noise_variable_408,)
    }
};
            noise_variable_408 = noise_metadata_schedule_698_e10092;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_699_e10107,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) {
        let noise_metadata_schedule_699_e10100: f64 = (noise_variable_117 * noise_variable_117);
        let noise_metadata_schedule_699_e10102: f64 = (noise_metadata_schedule_699_e10100 + 1e-6);
        let noise_metadata_schedule_699_e10103: f64 = (noise_metadata_schedule_699_e10102).sqrt();
        let noise_metadata_schedule_699_e10105: f64 = (noise_metadata_schedule_699_e10103 * noise_variable_124);
        (noise_metadata_schedule_699_e10105,)
    } else {
        (noise_variable_409,)
    }
};
            noise_variable_409 = noise_metadata_schedule_699_e10107;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_700_e10110: f64 = if params.p59 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_424 = noise_metadata_schedule_700_e10110;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_701_e10181,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 != 0.0)) {
        let noise_metadata_schedule_701_e10120: f64 = (noise_variable_120 - noise_variable_409);
        let (noise_metadata_schedule_701_e10179,) = {
            if (noise_metadata_schedule_701_e10120 > 1e-16) {
                let noise_metadata_schedule_701_e10127: f64 = (noise_variable_120 - noise_variable_409);
                let noise_metadata_schedule_701_e10130: f64 = (noise_variable_120 - noise_variable_409);
                let noise_metadata_schedule_701_e10133: f64 = (noise_variable_120 - noise_variable_409);
                let noise_metadata_schedule_701_e10134: f64 = (noise_metadata_schedule_701_e10130 * noise_metadata_schedule_701_e10133);
                let noise_metadata_schedule_701_e10136: f64 = (noise_metadata_schedule_701_e10134 + 1e-6);
                let noise_metadata_schedule_701_e10137: f64 = (noise_metadata_schedule_701_e10136).sqrt();
                let noise_metadata_schedule_701_e10138: f64 = (noise_metadata_schedule_701_e10127 + noise_metadata_schedule_701_e10137);
                let noise_metadata_schedule_701_e10139: f64 = (0.5 * noise_metadata_schedule_701_e10138);
                let noise_metadata_schedule_701_e10140: f64 = (noise_variable_120 - noise_metadata_schedule_701_e10139);
                (noise_metadata_schedule_701_e10140,)
            } else {
                let noise_metadata_schedule_701_e10143: f64 = (noise_variable_409 - noise_variable_120);
                let (noise_metadata_schedule_701_e10178,) = {
                    if (noise_metadata_schedule_701_e10143 > 1e-16) {
                        let noise_metadata_schedule_701_e10149: f64 = (0.5 * 1e-6);
                        let noise_metadata_schedule_701_e10152: f64 = (noise_variable_409 - noise_variable_120);
                        let noise_metadata_schedule_701_e10155: f64 = (noise_variable_409 - noise_variable_120);
                        let noise_metadata_schedule_701_e10158: f64 = (noise_variable_409 - noise_variable_120);
                        let noise_metadata_schedule_701_e10159: f64 = (noise_metadata_schedule_701_e10155 * noise_metadata_schedule_701_e10158);
                        let noise_metadata_schedule_701_e10161: f64 = (noise_metadata_schedule_701_e10159 + 1e-6);
                        let noise_metadata_schedule_701_e10162: f64 = (noise_metadata_schedule_701_e10161).sqrt();
                        let noise_metadata_schedule_701_e10163: f64 = (noise_metadata_schedule_701_e10152 + noise_metadata_schedule_701_e10162);
                        let noise_metadata_schedule_701_e10164: f64 = (noise_metadata_schedule_701_e10149 / noise_metadata_schedule_701_e10163);
                        let noise_metadata_schedule_701_e10165: f64 = (noise_variable_120 - noise_metadata_schedule_701_e10164);
                        (noise_metadata_schedule_701_e10165,)
                    } else {
                        let noise_metadata_schedule_701_e10170: f64 = (noise_variable_120 - noise_variable_409);
                        let noise_metadata_schedule_701_e10173: f64 = (1e-32 + 1e-6);
                        let noise_metadata_schedule_701_e10174: f64 = (noise_metadata_schedule_701_e10173).sqrt();
                        let noise_metadata_schedule_701_e10175: f64 = (noise_metadata_schedule_701_e10170 + noise_metadata_schedule_701_e10174);
                        let noise_metadata_schedule_701_e10176: f64 = (0.5 * noise_metadata_schedule_701_e10175);
                        let noise_metadata_schedule_701_e10177: f64 = (noise_variable_120 - noise_metadata_schedule_701_e10176);
                        (noise_metadata_schedule_701_e10177,)
                    }
                };
                (noise_metadata_schedule_701_e10178,)
            }
        };
        (noise_metadata_schedule_701_e10179,)
    } else {
        (noise_variable_409,)
    }
};
            noise_variable_409 = noise_metadata_schedule_701_e10181;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_702_e10184: f64 = if 1.0 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_425 = noise_metadata_schedule_702_e10184;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_703_e10202,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_703_e10194: f64 = (params.p17 * noise_variable_95);
        let noise_metadata_schedule_703_e10197: f64 = (noise_variable_408 - noise_variable_134);
        let noise_metadata_schedule_703_e10199: f64 = (noise_metadata_schedule_703_e10197 * noise_variable_26);
        let noise_metadata_schedule_703_e10200: f64 = (noise_metadata_schedule_703_e10194 + noise_metadata_schedule_703_e10199);
        (noise_metadata_schedule_703_e10200,)
    } else {
        (noise_variable_410,)
    }
};
            noise_variable_410 = noise_metadata_schedule_703_e10202;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_704_e10221,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_425 == 0.0)) {
        let noise_metadata_schedule_704_e10213: f64 = (params.p17 * noise_variable_95);
        let noise_metadata_schedule_704_e10216: f64 = (noise_variable_408 - noise_variable_93);
        let noise_metadata_schedule_704_e10218: f64 = (noise_metadata_schedule_704_e10216 * noise_variable_26);
        let noise_metadata_schedule_704_e10219: f64 = (noise_metadata_schedule_704_e10213 + noise_metadata_schedule_704_e10218);
        (noise_metadata_schedule_704_e10219,)
    } else {
        (noise_variable_410,)
    }
};
            noise_variable_410 = noise_metadata_schedule_704_e10221;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_705_e10224: f64 = if noise_variable_410 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_426 = noise_metadata_schedule_705_e10224;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_706_e10238,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_426 != 0.0)) {
        let noise_metadata_schedule_706_e10234: f64 = (noise_variable_410).exp();
        let noise_metadata_schedule_706_e10235: f64 = (1.0 + noise_metadata_schedule_706_e10234);
        let noise_metadata_schedule_706_e10236: f64 = (noise_metadata_schedule_706_e10235).ln();
        (noise_metadata_schedule_706_e10236,)
    } else {
        (noise_variable_415,)
    }
};
            noise_variable_415 = noise_metadata_schedule_706_e10238;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_707_e10249,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_426 == 0.0)) {
        (noise_variable_410,)
    } else {
        (noise_variable_415,)
    }
};
            noise_variable_415 = noise_metadata_schedule_707_e10249;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_708_e10263,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) {
        let noise_metadata_schedule_708_e10258: f64 = (params.p17 * noise_variable_128);
        let noise_metadata_schedule_708_e10260: f64 = (noise_metadata_schedule_708_e10258 * noise_variable_26);
        let noise_metadata_schedule_708_e10261: f64 = (noise_variable_410 - noise_metadata_schedule_708_e10260);
        (noise_metadata_schedule_708_e10261,)
    } else {
        (noise_variable_411,)
    }
};
            noise_variable_411 = noise_metadata_schedule_708_e10263;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_709_e10266: f64 = if noise_variable_411 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_427 = noise_metadata_schedule_709_e10266;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_710_e10280,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_427 != 0.0)) {
        let noise_metadata_schedule_710_e10276: f64 = (noise_variable_411).exp();
        let noise_metadata_schedule_710_e10277: f64 = (1.0 + noise_metadata_schedule_710_e10276);
        let noise_metadata_schedule_710_e10278: f64 = (noise_metadata_schedule_710_e10277).ln();
        (noise_metadata_schedule_710_e10278,)
    } else {
        (noise_variable_412,)
    }
};
            noise_variable_412 = noise_metadata_schedule_710_e10280;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_711_e10291,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_427 == 0.0)) {
        (noise_variable_411,)
    } else {
        (noise_variable_412,)
    }
};
            noise_variable_412 = noise_metadata_schedule_711_e10291;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_712_e10310,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) {
        let noise_metadata_schedule_712_e10299: f64 = (-1.5);
        let noise_metadata_schedule_712_e10304: f64 = (params.p59 * noise_variable_409);
        let noise_metadata_schedule_712_e10305: f64 = (params.p58 + noise_metadata_schedule_712_e10304);
        let noise_metadata_schedule_712_e10306: f64 = (noise_variable_409 * noise_metadata_schedule_712_e10305);
        let noise_metadata_schedule_712_e10307: f64 = (noise_metadata_schedule_712_e10299 + noise_metadata_schedule_712_e10306);
        let noise_metadata_schedule_712_e10308: f64 = (noise_variable_122 * noise_metadata_schedule_712_e10307);
        (noise_metadata_schedule_712_e10308,)
    } else {
        (noise_variable_414,)
    }
};
            noise_variable_414 = noise_metadata_schedule_712_e10310;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_713_e10312: f64 = (noise_variable_414).abs();
            let noise_metadata_schedule_713_e10314: f64 = if noise_metadata_schedule_713_e10312 < 230.25850929940458 { 1.0 } else { 0.0 };
            noise_variable_428 = noise_metadata_schedule_713_e10314;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_714_e10325,) = {
    if ((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_428 != 0.0)) {
        let noise_metadata_schedule_714_e10323: f64 = (noise_variable_414).exp();
        (noise_metadata_schedule_714_e10323,)
    } else {
        (noise_variable_413,)
    }
};
            noise_variable_413 = noise_metadata_schedule_714_e10325;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_715_e10328: f64 = (-230.25850929940458);
            let noise_metadata_schedule_715_e10329: f64 = if noise_variable_414 < noise_metadata_schedule_715_e10328 { 1.0 } else { 0.0 };
            noise_variable_429 = noise_metadata_schedule_715_e10329;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_716_e10367,) = {
    if (((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_428 == 0.0)) && (noise_variable_429 != 0.0)) {
        let noise_metadata_schedule_716_e10343: f64 = (-230.25850929940458);
        let noise_metadata_schedule_716_e10345: f64 = (noise_metadata_schedule_716_e10343 - noise_variable_414);
        let noise_metadata_schedule_716_e10349: f64 = (-230.25850929940458);
        let noise_metadata_schedule_716_e10351: f64 = (noise_metadata_schedule_716_e10349 - noise_variable_414);
        let noise_metadata_schedule_716_e10352: f64 = (0.5 * noise_metadata_schedule_716_e10351);
        let noise_metadata_schedule_716_e10355: f64 = (-230.25850929940458);
        let noise_metadata_schedule_716_e10357: f64 = (noise_metadata_schedule_716_e10355 - noise_variable_414);
        let noise_metadata_schedule_716_e10359: f64 = (noise_metadata_schedule_716_e10357 * 0.3333333333333333);
        let noise_metadata_schedule_716_e10360: f64 = (1.0 + noise_metadata_schedule_716_e10359);
        let noise_metadata_schedule_716_e10361: f64 = (noise_metadata_schedule_716_e10352 * noise_metadata_schedule_716_e10360);
        let noise_metadata_schedule_716_e10362: f64 = (1.0 + noise_metadata_schedule_716_e10361);
        let noise_metadata_schedule_716_e10363: f64 = (noise_metadata_schedule_716_e10345 * noise_metadata_schedule_716_e10362);
        let noise_metadata_schedule_716_e10364: f64 = (1.0 + noise_metadata_schedule_716_e10363);
        let noise_metadata_schedule_716_e10365: f64 = (1e-100 / noise_metadata_schedule_716_e10364);
        (noise_metadata_schedule_716_e10365,)
    } else {
        (noise_variable_413,)
    }
};
            noise_variable_413 = noise_metadata_schedule_716_e10367;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_717_e10403,) = {
    if (((((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_428 == 0.0)) && (noise_variable_429 == 0.0)) {
        let noise_metadata_schedule_717_e10383: f64 = (noise_variable_414 - 230.25850929940458);
        let noise_metadata_schedule_717_e10388: f64 = (noise_variable_414 - 230.25850929940458);
        let noise_metadata_schedule_717_e10389: f64 = (0.5 * noise_metadata_schedule_717_e10388);
        let noise_metadata_schedule_717_e10393: f64 = (noise_variable_414 - 230.25850929940458);
        let noise_metadata_schedule_717_e10395: f64 = (noise_metadata_schedule_717_e10393 * 0.3333333333333333);
        let noise_metadata_schedule_717_e10396: f64 = (1.0 + noise_metadata_schedule_717_e10395);
        let noise_metadata_schedule_717_e10397: f64 = (noise_metadata_schedule_717_e10389 * noise_metadata_schedule_717_e10396);
        let noise_metadata_schedule_717_e10398: f64 = (1.0 + noise_metadata_schedule_717_e10397);
        let noise_metadata_schedule_717_e10399: f64 = (noise_metadata_schedule_717_e10383 * noise_metadata_schedule_717_e10398);
        let noise_metadata_schedule_717_e10400: f64 = (1.0 + noise_metadata_schedule_717_e10399);
        let noise_metadata_schedule_717_e10401: f64 = (1e100 * noise_metadata_schedule_717_e10400);
        (noise_metadata_schedule_717_e10401,)
    } else {
        (noise_variable_413,)
    }
};
            noise_variable_413 = noise_metadata_schedule_717_e10403;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_718_e10421,) = {
    if (((params.p49 != 0.0) && (noise_variable_405 != 0.0)) && (noise_variable_423 != 0.0)) {
        let noise_metadata_schedule_718_e10412: f64 = (noise_variable_125 * noise_variable_413);
        let noise_metadata_schedule_718_e10414: f64 = (noise_metadata_schedule_718_e10412 * params.p17);
        let noise_metadata_schedule_718_e10417: f64 = (noise_variable_415 - noise_variable_412);
        let noise_metadata_schedule_718_e10418: f64 = (noise_metadata_schedule_718_e10414 * noise_metadata_schedule_718_e10417);
        let noise_metadata_schedule_718_e10419: f64 = (noise_variable_4 + noise_metadata_schedule_718_e10418);
        (noise_metadata_schedule_718_e10419,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_718_e10421;
        }
        match source_index {
            0 => {
                let noise_0_psd_e10467: f64 = 1.0;
                let noise_0_psd_e134: f64 = (2.0 * 1.6021918e-19);
                let noise_0_psd_e136: f64 = (noise_variable_4).abs();
                let noise_0_psd_e137: f64 = (noise_0_psd_e134 * noise_0_psd_e136);
                let noise_0_psd_e10468: f64 = (noise_0_psd_e10467 * noise_0_psd_e137);
                let psd = noise_0_psd_e10468;
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
                let noise_1_psd_e10470: f64 = 1.0;
                let noise_1_psd_e145: f64 = (2.0 * 1.6021918e-19);
                let noise_1_psd_e147: f64 = (noise_variable_5).abs();
                let noise_1_psd_e148: f64 = (noise_1_psd_e145 * noise_1_psd_e147);
                let noise_1_psd_e10471: f64 = (noise_1_psd_e10470 * noise_1_psd_e148);
                let psd = noise_1_psd_e10471;
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
                let noise_2_psd_e10473: f64 = 1.0;
                let noise_2_psd_e10474: f64 = (noise_2_psd_e10473 * noise_variable_72);
                let psd = noise_2_psd_e10474;
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
                let noise_3_psd_e10476: f64 = 1.0;
                let noise_3_psd_e10477: f64 = (noise_3_psd_e10476 * noise_variable_73);
                let psd = noise_3_psd_e10477;
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
                let noise_4_psd_e10479: f64 = 1.0;
                let noise_4_psd_e10480: f64 = (noise_4_psd_e10479 * noise_variable_74);
                let psd = noise_4_psd_e10480;
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
                let noise_5_psd_e10482: f64 = 1.0;
                let noise_5_psd_e10483: f64 = (noise_5_psd_e10482 * noise_variable_75);
                let psd = noise_5_psd_e10483;
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
                let noise_6_psd_e10485: f64 = 1.0;
                let noise_6_psd_e10486: f64 = (noise_6_psd_e10485 * noise_variable_76);
                let psd = noise_6_psd_e10486;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

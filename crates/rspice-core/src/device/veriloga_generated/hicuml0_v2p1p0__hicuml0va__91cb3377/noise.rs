#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("rb"), kind: GeneratedNoiseKind::White, equation: 26, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_C_RCX", label: Some("rcx"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_EI_E_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("ibe"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 0) {
            let noise_activation_schedule_762_e7692: f64 = if ((params.p23 >= params.p111) || (params.p26 >= params.p111)) { 1.0 } else { 0.0 };
            noise_variable_364 = noise_activation_schedule_762_e7692;
        }
        if matches!(source_index, 1) {
            let noise_activation_schedule_763_e7695: f64 = if params.p29 >= params.p111 { 1.0 } else { 0.0 };
            noise_variable_365 = noise_activation_schedule_763_e7695;
        }
        if matches!(source_index, 2) {
            let noise_activation_schedule_764_e7698: f64 = if params.p28 >= params.p111 { 1.0 } else { 0.0 };
            noise_variable_366 = noise_activation_schedule_764_e7698;
        }
        let noise_source_active = match source_index {
            0 => {
                noise_variable_364 != 0.0
            }
            1 => {
                noise_variable_365 != 0.0
            }
            2 => {
                noise_variable_366 != 0.0
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
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_0_e259: f64 = (params.p110 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_183 = noise_metadata_schedule_0_e259;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_1_e262: f64 = (params.p110 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_184 = noise_metadata_schedule_1_e262;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let noise_metadata_schedule_2_e265: f64 = (params.p110 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            noise_variable_185 = noise_metadata_schedule_2_e265;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_3_e268: f64 = (noise_variable_185 - noise_variable_184);
            noise_variable_186 = noise_metadata_schedule_3_e268;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_9_e280: f64 = (params.p108 + 273.15);
            noise_variable_8 = noise_metadata_schedule_9_e280;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_10_e281: f64 = ctx.temperature();
            noise_variable_9 = noise_metadata_schedule_10_e281;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_11_e284: f64 = (1.3806226e-23 * noise_variable_8);
            let noise_metadata_schedule_11_e286: f64 = (noise_metadata_schedule_11_e284 / 1.602176462e-19);
            noise_variable_177 = noise_metadata_schedule_11_e286;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_12_e289: f64 = (params.p88 * noise_variable_8);
            noise_variable_172 = noise_metadata_schedule_12_e289;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let noise_metadata_schedule_13_e293: f64 = (params.p76 + params.p77);
            let noise_metadata_schedule_13_e294: f64 = (0.5 * noise_metadata_schedule_13_e293);
            noise_variable_173 = noise_metadata_schedule_13_e294;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_14_e298: f64 = (params.p76 + params.p78);
            let noise_metadata_schedule_14_e299: f64 = (0.5 * noise_metadata_schedule_14_e298);
            noise_variable_174 = noise_metadata_schedule_14_e299;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_15_e303: f64 = (params.p79 + params.p78);
            let noise_metadata_schedule_15_e304: f64 = (0.5 * noise_metadata_schedule_15_e303);
            noise_variable_175 = noise_metadata_schedule_15_e304;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let noise_metadata_schedule_16_e308: f64 = (1.602176462e-19 * params.p80);
            let noise_metadata_schedule_16_e310: f64 = (noise_metadata_schedule_16_e308 / 1.3806226e-23);
            let noise_metadata_schedule_16_e311: f64 = (3.0 - noise_metadata_schedule_16_e310);
            noise_variable_168 = noise_metadata_schedule_16_e311;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_17_e314: f64 = (noise_variable_168 + 1.0);
            let noise_metadata_schedule_17_e316: f64 = (noise_metadata_schedule_17_e314 - params.p87);
            noise_variable_169 = noise_metadata_schedule_17_e316;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_20_e327: f64 = (params.p76 - params.p77);
            noise_variable_176 = noise_metadata_schedule_20_e327;
        }
        if matches!(source_index, 0 | 5) {
            noise_variable_27 = params.p34;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_26_e348: f64 = (noise_variable_9 + params.p109);
            let noise_metadata_schedule_26_e350: f64 = noise_metadata_schedule_26_e348;
            noise_variable_4 = noise_metadata_schedule_26_e350;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_27_e353: f64 = (-100.0);
            let noise_metadata_schedule_27_e355: f64 = (noise_metadata_schedule_27_e353 + 273.15);
            let noise_metadata_schedule_27_e356: f64 = if noise_variable_4 < noise_metadata_schedule_27_e355 { 1.0 } else { 0.0 };
            noise_variable_247 = noise_metadata_schedule_27_e356;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_28_e363,) = {
    if (noise_variable_247 != 0.0) {
        let noise_metadata_schedule_28_e359: f64 = (-100.0);
        let noise_metadata_schedule_28_e361: f64 = (noise_metadata_schedule_28_e359 + 273.15);
        (noise_metadata_schedule_28_e361,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_28_e363;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_29_e367: f64 = (326.85 + 273.15);
            let noise_metadata_schedule_29_e368: f64 = if noise_variable_4 > noise_metadata_schedule_29_e367 { 1.0 } else { 0.0 };
            noise_variable_248 = noise_metadata_schedule_29_e368;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_30_e377,) = {
    if ((noise_variable_247 == 0.0) && (noise_variable_248 != 0.0)) {
        let noise_metadata_schedule_30_e375: f64 = (326.85 + 273.15);
        (noise_metadata_schedule_30_e375,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_30_e377;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let noise_metadata_schedule_31_e380: f64 = (1.3806226e-23 * noise_variable_4);
            let noise_metadata_schedule_31_e382: f64 = (noise_metadata_schedule_31_e380 / 1.602176462e-19);
            noise_variable_2 = noise_metadata_schedule_31_e382;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let noise_metadata_schedule_32_e385: f64 = (1.0 / noise_variable_2);
            noise_variable_3 = noise_metadata_schedule_32_e385;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_33_e388: f64 = (noise_variable_4 - noise_variable_8);
            noise_variable_7 = noise_metadata_schedule_33_e388;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_34_e391: f64 = (noise_variable_4 / noise_variable_8);
            noise_variable_5 = noise_metadata_schedule_34_e391;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_35_e393: f64 = (noise_variable_5).ln();
            noise_variable_6 = noise_metadata_schedule_35_e393;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let noise_metadata_schedule_36_e397: f64 = (noise_variable_5 - 1.0);
            let noise_metadata_schedule_36_e398: f64 = (noise_variable_3 * noise_metadata_schedule_36_e397);
            noise_variable_10 = noise_metadata_schedule_36_e398;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_37_e401: f64 = (0.5 * params.p35);
            let noise_metadata_schedule_37_e403: f64 = (noise_metadata_schedule_37_e401 / noise_variable_177);
            noise_variable_178 = noise_metadata_schedule_37_e403;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_38_e406: f64 = (2.0 * noise_variable_177);
            let noise_metadata_schedule_38_e408: f64 = (noise_variable_178).exp();
            let noise_metadata_schedule_38_e410: f64 = (-noise_variable_178);
            let noise_metadata_schedule_38_e411: f64 = (noise_metadata_schedule_38_e410).exp();
            let noise_metadata_schedule_38_e412: f64 = (noise_metadata_schedule_38_e408 - noise_metadata_schedule_38_e411);
            let noise_metadata_schedule_38_e413: f64 = (noise_metadata_schedule_38_e412).ln();
            let noise_metadata_schedule_38_e414: f64 = (noise_metadata_schedule_38_e406 * noise_metadata_schedule_38_e413);
            noise_variable_96 = noise_metadata_schedule_38_e414;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_39_e417: f64 = (noise_variable_96 * noise_variable_5);
            let noise_metadata_schedule_39_e421: f64 = (1.0 - noise_variable_5);
            let noise_metadata_schedule_39_e422: f64 = (noise_variable_173 * noise_metadata_schedule_39_e421);
            let noise_metadata_schedule_39_e423: f64 = (noise_metadata_schedule_39_e417 + noise_metadata_schedule_39_e422);
            let noise_metadata_schedule_39_e426: f64 = (noise_variable_168 * noise_variable_2);
            let noise_metadata_schedule_39_e428: f64 = (noise_metadata_schedule_39_e426 * noise_variable_6);
            let noise_metadata_schedule_39_e429: f64 = (noise_metadata_schedule_39_e423 - noise_metadata_schedule_39_e428);
            noise_variable_97 = noise_metadata_schedule_39_e429;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_40_e433: f64 = (2.0 * noise_variable_2);
            let noise_metadata_schedule_40_e439: f64 = (-noise_variable_97);
            let noise_metadata_schedule_40_e441: f64 = (noise_metadata_schedule_40_e439 * noise_variable_3);
            let noise_metadata_schedule_40_e442: f64 = (noise_metadata_schedule_40_e441).exp();
            let noise_metadata_schedule_40_e443: f64 = (4.0 * noise_metadata_schedule_40_e442);
            let noise_metadata_schedule_40_e444: f64 = (1.0 + noise_metadata_schedule_40_e443);
            let noise_metadata_schedule_40_e445: f64 = (noise_metadata_schedule_40_e444).sqrt();
            let noise_metadata_schedule_40_e446: f64 = (1.0 + noise_metadata_schedule_40_e445);
            let noise_metadata_schedule_40_e447: f64 = (0.5 * noise_metadata_schedule_40_e446);
            let noise_metadata_schedule_40_e448: f64 = (noise_metadata_schedule_40_e447).ln();
            let noise_metadata_schedule_40_e449: f64 = (noise_metadata_schedule_40_e433 * noise_metadata_schedule_40_e448);
            let noise_metadata_schedule_40_e450: f64 = (noise_variable_97 + noise_metadata_schedule_40_e449);
            noise_variable_16 = noise_metadata_schedule_40_e450;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_41_e455: f64 = (params.p35 / noise_variable_16);
            let noise_metadata_schedule_41_e456: f64 = (noise_metadata_schedule_41_e455).ln();
            let noise_metadata_schedule_41_e457: f64 = (params.p36 * noise_metadata_schedule_41_e456);
            let noise_metadata_schedule_41_e458: f64 = (noise_metadata_schedule_41_e457).exp();
            let noise_metadata_schedule_41_e459: f64 = (params.p34 * noise_metadata_schedule_41_e458);
            noise_variable_23 = noise_metadata_schedule_41_e459;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_42_e462: f64 = (params.p37 * noise_variable_16);
            let noise_metadata_schedule_42_e464: f64 = (noise_metadata_schedule_42_e462 / params.p35);
            noise_variable_43 = noise_metadata_schedule_42_e464;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_43_e467: f64 = (0.5 * params.p38);
            let noise_metadata_schedule_43_e469: f64 = (noise_metadata_schedule_43_e467 / noise_variable_177);
            noise_variable_178 = noise_metadata_schedule_43_e469;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_44_e472: f64 = (2.0 * noise_variable_177);
            let noise_metadata_schedule_44_e474: f64 = (noise_variable_178).exp();
            let noise_metadata_schedule_44_e476: f64 = (-noise_variable_178);
            let noise_metadata_schedule_44_e477: f64 = (noise_metadata_schedule_44_e476).exp();
            let noise_metadata_schedule_44_e478: f64 = (noise_metadata_schedule_44_e474 - noise_metadata_schedule_44_e477);
            let noise_metadata_schedule_44_e479: f64 = (noise_metadata_schedule_44_e478).ln();
            let noise_metadata_schedule_44_e480: f64 = (noise_metadata_schedule_44_e472 * noise_metadata_schedule_44_e479);
            noise_variable_96 = noise_metadata_schedule_44_e480;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_45_e483: f64 = (noise_variable_96 * noise_variable_5);
            let noise_metadata_schedule_45_e487: f64 = (1.0 - noise_variable_5);
            let noise_metadata_schedule_45_e488: f64 = (noise_variable_173 * noise_metadata_schedule_45_e487);
            let noise_metadata_schedule_45_e489: f64 = (noise_metadata_schedule_45_e483 + noise_metadata_schedule_45_e488);
            let noise_metadata_schedule_45_e492: f64 = (noise_variable_168 * noise_variable_2);
            let noise_metadata_schedule_45_e494: f64 = (noise_metadata_schedule_45_e492 * noise_variable_6);
            let noise_metadata_schedule_45_e495: f64 = (noise_metadata_schedule_45_e489 - noise_metadata_schedule_45_e494);
            noise_variable_97 = noise_metadata_schedule_45_e495;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_46_e499: f64 = (2.0 * noise_variable_2);
            let noise_metadata_schedule_46_e505: f64 = (-noise_variable_97);
            let noise_metadata_schedule_46_e507: f64 = (noise_metadata_schedule_46_e505 * noise_variable_3);
            let noise_metadata_schedule_46_e508: f64 = (noise_metadata_schedule_46_e507).exp();
            let noise_metadata_schedule_46_e509: f64 = (4.0 * noise_metadata_schedule_46_e508);
            let noise_metadata_schedule_46_e510: f64 = (1.0 + noise_metadata_schedule_46_e509);
            let noise_metadata_schedule_46_e511: f64 = (noise_metadata_schedule_46_e510).sqrt();
            let noise_metadata_schedule_46_e512: f64 = (1.0 + noise_metadata_schedule_46_e511);
            let noise_metadata_schedule_46_e513: f64 = (0.5 * noise_metadata_schedule_46_e512);
            let noise_metadata_schedule_46_e514: f64 = (noise_metadata_schedule_46_e513).ln();
            let noise_metadata_schedule_46_e515: f64 = (noise_metadata_schedule_46_e499 * noise_metadata_schedule_46_e514);
            let noise_metadata_schedule_46_e516: f64 = (noise_variable_97 + noise_metadata_schedule_46_e515);
            noise_variable_22 = noise_metadata_schedule_46_e516;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_47_e521: f64 = (params.p38 / noise_variable_22);
            let noise_metadata_schedule_47_e522: f64 = (noise_metadata_schedule_47_e521).ln();
            let noise_metadata_schedule_47_e523: f64 = (params.p39 * noise_metadata_schedule_47_e522);
            let noise_metadata_schedule_47_e524: f64 = (noise_metadata_schedule_47_e523).exp();
            let noise_metadata_schedule_47_e525: f64 = (noise_variable_27 * noise_metadata_schedule_47_e524);
            noise_variable_26 = noise_metadata_schedule_47_e525;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_48_e528: f64 = (params.p40 * noise_variable_22);
            let noise_metadata_schedule_48_e530: f64 = (noise_metadata_schedule_48_e528 / params.p38);
            noise_variable_44 = noise_metadata_schedule_48_e530;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let noise_metadata_schedule_49_e534: f64 = (params.p82 * noise_variable_6);
            let noise_metadata_schedule_49_e537: f64 = (params.p77 * noise_variable_10);
            let noise_metadata_schedule_49_e538: f64 = (noise_metadata_schedule_49_e534 + noise_metadata_schedule_49_e537);
            let noise_metadata_schedule_49_e539: f64 = (noise_metadata_schedule_49_e538).exp();
            let noise_metadata_schedule_49_e540: f64 = (params.p15 * noise_metadata_schedule_49_e539);
            noise_variable_13 = noise_metadata_schedule_49_e540;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let noise_metadata_schedule_50_e544: f64 = (0.5 * noise_variable_168);
            let noise_metadata_schedule_50_e546: f64 = (noise_metadata_schedule_50_e544 * noise_variable_6);
            let noise_metadata_schedule_50_e549: f64 = (0.5 * noise_variable_173);
            let noise_metadata_schedule_50_e551: f64 = (noise_metadata_schedule_50_e549 * noise_variable_10);
            let noise_metadata_schedule_50_e552: f64 = (noise_metadata_schedule_50_e546 + noise_metadata_schedule_50_e551);
            let noise_metadata_schedule_50_e553: f64 = (noise_metadata_schedule_50_e552).exp();
            let noise_metadata_schedule_50_e554: f64 = (params.p17 * noise_metadata_schedule_50_e553);
            noise_variable_12 = noise_metadata_schedule_50_e554;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_51_e557: f64 = (0.5 * params.p42);
            let noise_metadata_schedule_51_e559: f64 = (noise_metadata_schedule_51_e557 / noise_variable_177);
            noise_variable_178 = noise_metadata_schedule_51_e559;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_52_e562: f64 = (2.0 * noise_variable_177);
            let noise_metadata_schedule_52_e564: f64 = (noise_variable_178).exp();
            let noise_metadata_schedule_52_e566: f64 = (-noise_variable_178);
            let noise_metadata_schedule_52_e567: f64 = (noise_metadata_schedule_52_e566).exp();
            let noise_metadata_schedule_52_e568: f64 = (noise_metadata_schedule_52_e564 - noise_metadata_schedule_52_e567);
            let noise_metadata_schedule_52_e569: f64 = (noise_metadata_schedule_52_e568).ln();
            let noise_metadata_schedule_52_e570: f64 = (noise_metadata_schedule_52_e562 * noise_metadata_schedule_52_e569);
            noise_variable_96 = noise_metadata_schedule_52_e570;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_53_e573: f64 = (noise_variable_96 * noise_variable_5);
            let noise_metadata_schedule_53_e577: f64 = (1.0 - noise_variable_5);
            let noise_metadata_schedule_53_e578: f64 = (noise_variable_174 * noise_metadata_schedule_53_e577);
            let noise_metadata_schedule_53_e579: f64 = (noise_metadata_schedule_53_e573 + noise_metadata_schedule_53_e578);
            let noise_metadata_schedule_53_e582: f64 = (noise_variable_168 * noise_variable_2);
            let noise_metadata_schedule_53_e584: f64 = (noise_metadata_schedule_53_e582 * noise_variable_6);
            let noise_metadata_schedule_53_e585: f64 = (noise_metadata_schedule_53_e579 - noise_metadata_schedule_53_e584);
            noise_variable_97 = noise_metadata_schedule_53_e585;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_54_e589: f64 = (2.0 * noise_variable_2);
            let noise_metadata_schedule_54_e595: f64 = (-noise_variable_97);
            let noise_metadata_schedule_54_e597: f64 = (noise_metadata_schedule_54_e595 * noise_variable_3);
            let noise_metadata_schedule_54_e598: f64 = (noise_metadata_schedule_54_e597).exp();
            let noise_metadata_schedule_54_e599: f64 = (4.0 * noise_metadata_schedule_54_e598);
            let noise_metadata_schedule_54_e600: f64 = (1.0 + noise_metadata_schedule_54_e599);
            let noise_metadata_schedule_54_e601: f64 = (noise_metadata_schedule_54_e600).sqrt();
            let noise_metadata_schedule_54_e602: f64 = (1.0 + noise_metadata_schedule_54_e601);
            let noise_metadata_schedule_54_e603: f64 = (0.5 * noise_metadata_schedule_54_e602);
            let noise_metadata_schedule_54_e604: f64 = (noise_metadata_schedule_54_e603).ln();
            let noise_metadata_schedule_54_e605: f64 = (noise_metadata_schedule_54_e589 * noise_metadata_schedule_54_e604);
            let noise_metadata_schedule_54_e606: f64 = (noise_variable_97 + noise_metadata_schedule_54_e605);
            noise_variable_17 = noise_metadata_schedule_54_e606;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_55_e611: f64 = (params.p42 / noise_variable_17);
            let noise_metadata_schedule_55_e612: f64 = (noise_metadata_schedule_55_e611).ln();
            let noise_metadata_schedule_55_e613: f64 = (params.p43 * noise_metadata_schedule_55_e612);
            let noise_metadata_schedule_55_e614: f64 = (noise_metadata_schedule_55_e613).exp();
            let noise_metadata_schedule_55_e615: f64 = (params.p41 * noise_metadata_schedule_55_e614);
            noise_variable_24 = noise_metadata_schedule_55_e615;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_56_e619: f64 = (noise_variable_169 * noise_variable_6);
            let noise_metadata_schedule_56_e622: f64 = (params.p78 * noise_variable_10);
            let noise_metadata_schedule_56_e623: f64 = (noise_metadata_schedule_56_e619 + noise_metadata_schedule_56_e622);
            let noise_metadata_schedule_56_e624: f64 = (noise_metadata_schedule_56_e623).exp();
            let noise_metadata_schedule_56_e625: f64 = (params.p19 * noise_metadata_schedule_56_e624);
            noise_variable_14 = noise_metadata_schedule_56_e625;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_57_e629: f64 = (params.p81 * noise_variable_6);
            let noise_metadata_schedule_57_e632: f64 = (params.p76 * noise_variable_10);
            let noise_metadata_schedule_57_e633: f64 = (noise_metadata_schedule_57_e629 + noise_metadata_schedule_57_e632);
            let noise_metadata_schedule_57_e634: f64 = (noise_metadata_schedule_57_e633).exp();
            let noise_metadata_schedule_57_e635: f64 = (params.p1 * noise_metadata_schedule_57_e634);
            noise_variable_11 = noise_metadata_schedule_57_e635;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_58_e639: f64 = (params.p95 * noise_variable_6);
            let noise_metadata_schedule_58_e642: f64 = (params.p83 * noise_variable_10);
            let noise_metadata_schedule_58_e643: f64 = (noise_metadata_schedule_58_e639 - noise_metadata_schedule_58_e642);
            let noise_metadata_schedule_58_e644: f64 = (noise_metadata_schedule_58_e643).exp();
            let noise_metadata_schedule_58_e645: f64 = (params.p9 * noise_metadata_schedule_58_e644);
            noise_variable_15 = noise_metadata_schedule_58_e645;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_59_e649: f64 = (params.p87 - noise_variable_172);
            let noise_metadata_schedule_59_e651: f64 = (noise_metadata_schedule_59_e649 * noise_variable_6);
            let noise_metadata_schedule_59_e652: f64 = (noise_metadata_schedule_59_e651).exp();
            let noise_metadata_schedule_59_e653: f64 = (params.p62 * noise_metadata_schedule_59_e652);
            noise_variable_33 = noise_metadata_schedule_59_e653;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_60_e657: f64 = (params.p87 * noise_variable_6);
            let noise_metadata_schedule_60_e658: f64 = (noise_metadata_schedule_60_e657).exp();
            let noise_metadata_schedule_60_e659: f64 = (params.p61 * noise_metadata_schedule_60_e658);
            noise_variable_31 = noise_metadata_schedule_60_e659;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_61_e662: f64 = (1.0 / noise_variable_31);
            noise_variable_32 = noise_metadata_schedule_61_e662;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_62_e667: f64 = (params.p89 * noise_variable_7);
            let noise_metadata_schedule_62_e668: f64 = (1.0 + noise_metadata_schedule_62_e667);
            let noise_metadata_schedule_62_e669: f64 = (params.p64 * noise_metadata_schedule_62_e668);
            noise_variable_34 = noise_metadata_schedule_62_e669;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_63_e672: f64 = if params.p65 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_249 = noise_metadata_schedule_63_e672;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_64_e682,) = {
    if (noise_variable_249 != 0.0) {
        let noise_metadata_schedule_64_e678: f64 = (params.p90 * noise_variable_7);
        let noise_metadata_schedule_64_e679: f64 = (1.0 - noise_metadata_schedule_64_e678);
        let noise_metadata_schedule_64_e680: f64 = (params.p65 * noise_metadata_schedule_64_e679);
        (noise_metadata_schedule_64_e680,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_64_e682;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_65_e686,) = {
    if (noise_variable_249 != 0.0) {
        (params.p64,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_65_e686;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_66_e697,) = {
    if (noise_variable_249 == 0.0) {
        let noise_metadata_schedule_66_e693: f64 = (params.p89 * noise_variable_7);
        let noise_metadata_schedule_66_e694: f64 = (1.0 + noise_metadata_schedule_66_e693);
        let noise_metadata_schedule_66_e695: f64 = (params.p64 * noise_metadata_schedule_66_e694);
        (noise_metadata_schedule_66_e695,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_66_e697;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_67_e702,) = {
    if (noise_variable_249 == 0.0) {
        (params.p65,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_67_e702;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_68_e707: f64 = (params.p85 * noise_variable_7);
            let noise_metadata_schedule_68_e708: f64 = (1.0 + noise_metadata_schedule_68_e707);
            let noise_metadata_schedule_68_e711: f64 = (params.p86 * noise_variable_7);
            let noise_metadata_schedule_68_e713: f64 = (noise_metadata_schedule_68_e711 * noise_variable_7);
            let noise_metadata_schedule_68_e714: f64 = (noise_metadata_schedule_68_e708 + noise_metadata_schedule_68_e713);
            let noise_metadata_schedule_68_e715: f64 = (params.p54 * noise_metadata_schedule_68_e714);
            noise_variable_42 = noise_metadata_schedule_68_e715;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_78_e779: f64 = (params.p91 * noise_variable_6);
            let noise_metadata_schedule_78_e780: f64 = (noise_metadata_schedule_78_e779).exp();
            let noise_metadata_schedule_78_e781: f64 = (params.p23 * noise_metadata_schedule_78_e780);
            noise_variable_37 = noise_metadata_schedule_78_e781;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_79_e784: f64 = (0.5 * params.p46);
            let noise_metadata_schedule_79_e786: f64 = (noise_metadata_schedule_79_e784 / noise_variable_177);
            noise_variable_178 = noise_metadata_schedule_79_e786;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_80_e789: f64 = (2.0 * noise_variable_177);
            let noise_metadata_schedule_80_e791: f64 = (noise_variable_178).exp();
            let noise_metadata_schedule_80_e793: f64 = (-noise_variable_178);
            let noise_metadata_schedule_80_e794: f64 = (noise_metadata_schedule_80_e793).exp();
            let noise_metadata_schedule_80_e795: f64 = (noise_metadata_schedule_80_e791 - noise_metadata_schedule_80_e794);
            let noise_metadata_schedule_80_e796: f64 = (noise_metadata_schedule_80_e795).ln();
            let noise_metadata_schedule_80_e797: f64 = (noise_metadata_schedule_80_e789 * noise_metadata_schedule_80_e796);
            noise_variable_96 = noise_metadata_schedule_80_e797;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_81_e800: f64 = (noise_variable_96 * noise_variable_5);
            let noise_metadata_schedule_81_e804: f64 = (1.0 - noise_variable_5);
            let noise_metadata_schedule_81_e805: f64 = (noise_variable_174 * noise_metadata_schedule_81_e804);
            let noise_metadata_schedule_81_e806: f64 = (noise_metadata_schedule_81_e800 + noise_metadata_schedule_81_e805);
            let noise_metadata_schedule_81_e809: f64 = (noise_variable_168 * noise_variable_2);
            let noise_metadata_schedule_81_e811: f64 = (noise_metadata_schedule_81_e809 * noise_variable_6);
            let noise_metadata_schedule_81_e812: f64 = (noise_metadata_schedule_81_e806 - noise_metadata_schedule_81_e811);
            noise_variable_97 = noise_metadata_schedule_81_e812;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_82_e816: f64 = (2.0 * noise_variable_2);
            let noise_metadata_schedule_82_e822: f64 = (-noise_variable_97);
            let noise_metadata_schedule_82_e824: f64 = (noise_metadata_schedule_82_e822 * noise_variable_3);
            let noise_metadata_schedule_82_e825: f64 = (noise_metadata_schedule_82_e824).exp();
            let noise_metadata_schedule_82_e826: f64 = (4.0 * noise_metadata_schedule_82_e825);
            let noise_metadata_schedule_82_e827: f64 = (1.0 + noise_metadata_schedule_82_e826);
            let noise_metadata_schedule_82_e828: f64 = (noise_metadata_schedule_82_e827).sqrt();
            let noise_metadata_schedule_82_e829: f64 = (1.0 + noise_metadata_schedule_82_e828);
            let noise_metadata_schedule_82_e830: f64 = (0.5 * noise_metadata_schedule_82_e829);
            let noise_metadata_schedule_82_e831: f64 = (noise_metadata_schedule_82_e830).ln();
            let noise_metadata_schedule_82_e832: f64 = (noise_metadata_schedule_82_e816 * noise_metadata_schedule_82_e831);
            let noise_metadata_schedule_82_e833: f64 = (noise_variable_97 + noise_metadata_schedule_82_e832);
            noise_variable_18 = noise_metadata_schedule_82_e833;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_83_e838: f64 = (params.p46 / noise_variable_18);
            let noise_metadata_schedule_83_e839: f64 = (noise_metadata_schedule_83_e838).ln();
            let noise_metadata_schedule_83_e840: f64 = (params.p47 * noise_metadata_schedule_83_e839);
            let noise_metadata_schedule_83_e841: f64 = (noise_metadata_schedule_83_e840).exp();
            let noise_metadata_schedule_83_e842: f64 = (params.p45 * noise_metadata_schedule_83_e841);
            noise_variable_25 = noise_metadata_schedule_83_e842;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_84_e845: f64 = (0.5 * params.p51);
            let noise_metadata_schedule_84_e847: f64 = (noise_metadata_schedule_84_e845 / noise_variable_177);
            noise_variable_178 = noise_metadata_schedule_84_e847;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_85_e850: f64 = (2.0 * noise_variable_177);
            let noise_metadata_schedule_85_e852: f64 = (noise_variable_178).exp();
            let noise_metadata_schedule_85_e854: f64 = (-noise_variable_178);
            let noise_metadata_schedule_85_e855: f64 = (noise_metadata_schedule_85_e854).exp();
            let noise_metadata_schedule_85_e856: f64 = (noise_metadata_schedule_85_e852 - noise_metadata_schedule_85_e855);
            let noise_metadata_schedule_85_e857: f64 = (noise_metadata_schedule_85_e856).ln();
            let noise_metadata_schedule_85_e858: f64 = (noise_metadata_schedule_85_e850 * noise_metadata_schedule_85_e857);
            noise_variable_96 = noise_metadata_schedule_85_e858;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_86_e861: f64 = (noise_variable_96 * noise_variable_5);
            let noise_metadata_schedule_86_e865: f64 = (1.0 - noise_variable_5);
            let noise_metadata_schedule_86_e866: f64 = (noise_variable_175 * noise_metadata_schedule_86_e865);
            let noise_metadata_schedule_86_e867: f64 = (noise_metadata_schedule_86_e861 + noise_metadata_schedule_86_e866);
            let noise_metadata_schedule_86_e870: f64 = (noise_variable_168 * noise_variable_2);
            let noise_metadata_schedule_86_e872: f64 = (noise_metadata_schedule_86_e870 * noise_variable_6);
            let noise_metadata_schedule_86_e873: f64 = (noise_metadata_schedule_86_e867 - noise_metadata_schedule_86_e872);
            noise_variable_97 = noise_metadata_schedule_86_e873;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_91_e927: f64 = (params.p97 * noise_variable_6);
            let noise_metadata_schedule_91_e928: f64 = (noise_metadata_schedule_91_e927).exp();
            let noise_metadata_schedule_91_e929: f64 = (params.p7 * noise_metadata_schedule_91_e928);
            noise_variable_200 = noise_metadata_schedule_91_e929;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_92_e933: f64 = (params.p83 * noise_variable_3);
            let noise_metadata_schedule_92_e936: f64 = (params.p84 * noise_variable_6);
            let noise_metadata_schedule_92_e937: f64 = (noise_metadata_schedule_92_e936).exp();
            let noise_metadata_schedule_92_e939: f64 = (noise_metadata_schedule_92_e937 - 1.0);
            let noise_metadata_schedule_92_e940: f64 = (noise_metadata_schedule_92_e933 * noise_metadata_schedule_92_e939);
            let noise_metadata_schedule_92_e941: f64 = (noise_metadata_schedule_92_e940).exp();
            let noise_metadata_schedule_92_e942: f64 = (params.p6 / noise_metadata_schedule_92_e941);
            noise_variable_202 = noise_metadata_schedule_92_e942;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_93_e945: f64 = if params.p0 <= 200.0 { 1.0 } else { 0.0 };
            noise_variable_252 = noise_metadata_schedule_93_e945;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_94_e957,) = {
    if (noise_variable_252 != 0.0) {
        let noise_metadata_schedule_94_e952: f64 = (params.p102 * noise_variable_7);
        let noise_metadata_schedule_94_e953: f64 = (params.p101 + noise_metadata_schedule_94_e952);
        let noise_metadata_schedule_94_e954: f64 = (noise_variable_7 * noise_metadata_schedule_94_e953);
        let noise_metadata_schedule_94_e955: f64 = (1.0 + noise_metadata_schedule_94_e954);
        (noise_metadata_schedule_94_e955,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_94_e957;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_95_e965,) = {
    if (noise_variable_252 == 0.0) {
        let noise_metadata_schedule_95_e962: f64 = (params.p98 * noise_variable_6);
        let noise_metadata_schedule_95_e963: f64 = (noise_metadata_schedule_95_e962).exp();
        (noise_metadata_schedule_95_e963,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_95_e965;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_96_e968: f64 = (params.p12 * noise_variable_204);
            noise_variable_203 = noise_metadata_schedule_96_e968;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_97_e971: f64 = (params.p13 * noise_variable_204);
            let noise_metadata_schedule_97_e974: f64 = (noise_variable_176 * noise_variable_10);
            let noise_metadata_schedule_97_e975: f64 = (noise_metadata_schedule_97_e974).exp();
            let noise_metadata_schedule_97_e976: f64 = (noise_metadata_schedule_97_e971 * noise_metadata_schedule_97_e975);
            noise_variable_205 = noise_metadata_schedule_97_e976;
        }
        if matches!(source_index, 0 | 5) {
            noise_variable_206 = params.p14;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_99_e981: f64 = (params.p93 * noise_variable_6);
            let noise_metadata_schedule_99_e982: f64 = (noise_metadata_schedule_99_e981).exp();
            let noise_metadata_schedule_99_e983: f64 = (params.p29 * noise_metadata_schedule_99_e982);
            noise_variable_40 = noise_metadata_schedule_99_e983;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_100_e987: f64 = (params.p92 * noise_variable_6);
            let noise_metadata_schedule_100_e988: f64 = (noise_metadata_schedule_100_e987).exp();
            let noise_metadata_schedule_100_e989: f64 = (params.p26 * noise_metadata_schedule_100_e988);
            noise_variable_39 = noise_metadata_schedule_100_e989;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_101_e993: f64 = (params.p94 * noise_variable_6);
            let noise_metadata_schedule_101_e994: f64 = (noise_metadata_schedule_101_e993).exp();
            let noise_metadata_schedule_101_e995: f64 = (params.p28 * noise_metadata_schedule_101_e994);
            noise_variable_41 = noise_metadata_schedule_101_e995;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_103_e1014: f64 = if ((params.p103 != 0.0) && (params.p104 >= params.p111)) { 1.0 } else { 0.0 };
            noise_variable_253 = noise_metadata_schedule_103_e1014;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_104_e1022,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_104_e1018: f64 = (noise_variable_9 + params.p109);
        let noise_metadata_schedule_104_e1020: f64 = (noise_metadata_schedule_104_e1018 + (ctx.node_voltage(self.nodes[4]) - 0.0));
        (noise_metadata_schedule_104_e1020,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_104_e1022;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_105_e1025: f64 = (-100.0);
            let noise_metadata_schedule_105_e1027: f64 = (noise_metadata_schedule_105_e1025 + 273.15);
            let noise_metadata_schedule_105_e1028: f64 = if noise_variable_4 < noise_metadata_schedule_105_e1027 { 1.0 } else { 0.0 };
            noise_variable_254 = noise_metadata_schedule_105_e1028;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_106_e1037,) = {
    if ((noise_variable_253 != 0.0) && (noise_variable_254 != 0.0)) {
        let noise_metadata_schedule_106_e1033: f64 = (-100.0);
        let noise_metadata_schedule_106_e1035: f64 = (noise_metadata_schedule_106_e1033 + 273.15);
        (noise_metadata_schedule_106_e1035,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_106_e1037;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_107_e1041: f64 = (326.85 + 273.15);
            let noise_metadata_schedule_107_e1042: f64 = if noise_variable_4 > noise_metadata_schedule_107_e1041 { 1.0 } else { 0.0 };
            noise_variable_255 = noise_metadata_schedule_107_e1042;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_108_e1053,) = {
    if (((noise_variable_253 != 0.0) && (noise_variable_254 == 0.0)) && (noise_variable_255 != 0.0)) {
        let noise_metadata_schedule_108_e1051: f64 = (326.85 + 273.15);
        (noise_metadata_schedule_108_e1051,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_108_e1053;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let (noise_metadata_schedule_109_e1061,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_109_e1057: f64 = (1.3806226e-23 * noise_variable_4);
        let noise_metadata_schedule_109_e1059: f64 = (noise_metadata_schedule_109_e1057 / 1.602176462e-19);
        (noise_metadata_schedule_109_e1059,)
    } else {
        (noise_variable_2,)
    }
};
            noise_variable_2 = noise_metadata_schedule_109_e1061;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let (noise_metadata_schedule_110_e1067,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_110_e1065: f64 = (1.0 / noise_variable_2);
        (noise_metadata_schedule_110_e1065,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_110_e1067;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_111_e1073,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_111_e1071: f64 = (noise_variable_4 - noise_variable_8);
        (noise_metadata_schedule_111_e1071,)
    } else {
        (noise_variable_7,)
    }
};
            noise_variable_7 = noise_metadata_schedule_111_e1073;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_112_e1079,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_112_e1077: f64 = (noise_variable_4 / noise_variable_8);
        (noise_metadata_schedule_112_e1077,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_112_e1079;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_113_e1084,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_113_e1082: f64 = (noise_variable_5).ln();
        (noise_metadata_schedule_113_e1082,)
    } else {
        (noise_variable_6,)
    }
};
            noise_variable_6 = noise_metadata_schedule_113_e1084;
        }
        if matches!(source_index, 0 | 3 | 4 | 5) {
            let (noise_metadata_schedule_114_e1092,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_114_e1089: f64 = (noise_variable_5 - 1.0);
        let noise_metadata_schedule_114_e1090: f64 = (noise_variable_3 * noise_metadata_schedule_114_e1089);
        (noise_metadata_schedule_114_e1090,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_114_e1092;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_115_e1100,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_115_e1096: f64 = (0.5 * params.p35);
        let noise_metadata_schedule_115_e1098: f64 = (noise_metadata_schedule_115_e1096 / noise_variable_177);
        (noise_metadata_schedule_115_e1098,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_115_e1100;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_116_e1114,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_116_e1104: f64 = (2.0 * noise_variable_177);
        let noise_metadata_schedule_116_e1106: f64 = (noise_variable_178).exp();
        let noise_metadata_schedule_116_e1108: f64 = (-noise_variable_178);
        let noise_metadata_schedule_116_e1109: f64 = (noise_metadata_schedule_116_e1108).exp();
        let noise_metadata_schedule_116_e1110: f64 = (noise_metadata_schedule_116_e1106 - noise_metadata_schedule_116_e1109);
        let noise_metadata_schedule_116_e1111: f64 = (noise_metadata_schedule_116_e1110).ln();
        let noise_metadata_schedule_116_e1112: f64 = (noise_metadata_schedule_116_e1104 * noise_metadata_schedule_116_e1111);
        (noise_metadata_schedule_116_e1112,)
    } else {
        (noise_variable_96,)
    }
};
            noise_variable_96 = noise_metadata_schedule_116_e1114;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_117_e1132,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_117_e1118: f64 = (noise_variable_96 * noise_variable_5);
        let noise_metadata_schedule_117_e1122: f64 = (1.0 - noise_variable_5);
        let noise_metadata_schedule_117_e1123: f64 = (noise_variable_173 * noise_metadata_schedule_117_e1122);
        let noise_metadata_schedule_117_e1124: f64 = (noise_metadata_schedule_117_e1118 + noise_metadata_schedule_117_e1123);
        let noise_metadata_schedule_117_e1127: f64 = (noise_variable_168 * noise_variable_2);
        let noise_metadata_schedule_117_e1129: f64 = (noise_metadata_schedule_117_e1127 * noise_variable_6);
        let noise_metadata_schedule_117_e1130: f64 = (noise_metadata_schedule_117_e1124 - noise_metadata_schedule_117_e1129);
        (noise_metadata_schedule_117_e1130,)
    } else {
        (noise_variable_97,)
    }
};
            noise_variable_97 = noise_metadata_schedule_117_e1132;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_118_e1156,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_118_e1137: f64 = (2.0 * noise_variable_2);
        let noise_metadata_schedule_118_e1143: f64 = (-noise_variable_97);
        let noise_metadata_schedule_118_e1145: f64 = (noise_metadata_schedule_118_e1143 * noise_variable_3);
        let noise_metadata_schedule_118_e1146: f64 = (noise_metadata_schedule_118_e1145).exp();
        let noise_metadata_schedule_118_e1147: f64 = (4.0 * noise_metadata_schedule_118_e1146);
        let noise_metadata_schedule_118_e1148: f64 = (1.0 + noise_metadata_schedule_118_e1147);
        let noise_metadata_schedule_118_e1149: f64 = (noise_metadata_schedule_118_e1148).sqrt();
        let noise_metadata_schedule_118_e1150: f64 = (1.0 + noise_metadata_schedule_118_e1149);
        let noise_metadata_schedule_118_e1151: f64 = (0.5 * noise_metadata_schedule_118_e1150);
        let noise_metadata_schedule_118_e1152: f64 = (noise_metadata_schedule_118_e1151).ln();
        let noise_metadata_schedule_118_e1153: f64 = (noise_metadata_schedule_118_e1137 * noise_metadata_schedule_118_e1152);
        let noise_metadata_schedule_118_e1154: f64 = (noise_variable_97 + noise_metadata_schedule_118_e1153);
        (noise_metadata_schedule_118_e1154,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_118_e1156;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_119_e1168,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_119_e1162: f64 = (params.p35 / noise_variable_16);
        let noise_metadata_schedule_119_e1163: f64 = (noise_metadata_schedule_119_e1162).ln();
        let noise_metadata_schedule_119_e1164: f64 = (params.p36 * noise_metadata_schedule_119_e1163);
        let noise_metadata_schedule_119_e1165: f64 = (noise_metadata_schedule_119_e1164).exp();
        let noise_metadata_schedule_119_e1166: f64 = (params.p34 * noise_metadata_schedule_119_e1165);
        (noise_metadata_schedule_119_e1166,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_119_e1168;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_120_e1176,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_120_e1172: f64 = (params.p37 * noise_variable_16);
        let noise_metadata_schedule_120_e1174: f64 = (noise_metadata_schedule_120_e1172 / params.p35);
        (noise_metadata_schedule_120_e1174,)
    } else {
        (noise_variable_43,)
    }
};
            noise_variable_43 = noise_metadata_schedule_120_e1176;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_121_e1184,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_121_e1180: f64 = (0.5 * params.p38);
        let noise_metadata_schedule_121_e1182: f64 = (noise_metadata_schedule_121_e1180 / noise_variable_177);
        (noise_metadata_schedule_121_e1182,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_121_e1184;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_122_e1198,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_122_e1188: f64 = (2.0 * noise_variable_177);
        let noise_metadata_schedule_122_e1190: f64 = (noise_variable_178).exp();
        let noise_metadata_schedule_122_e1192: f64 = (-noise_variable_178);
        let noise_metadata_schedule_122_e1193: f64 = (noise_metadata_schedule_122_e1192).exp();
        let noise_metadata_schedule_122_e1194: f64 = (noise_metadata_schedule_122_e1190 - noise_metadata_schedule_122_e1193);
        let noise_metadata_schedule_122_e1195: f64 = (noise_metadata_schedule_122_e1194).ln();
        let noise_metadata_schedule_122_e1196: f64 = (noise_metadata_schedule_122_e1188 * noise_metadata_schedule_122_e1195);
        (noise_metadata_schedule_122_e1196,)
    } else {
        (noise_variable_96,)
    }
};
            noise_variable_96 = noise_metadata_schedule_122_e1198;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_123_e1216,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_123_e1202: f64 = (noise_variable_96 * noise_variable_5);
        let noise_metadata_schedule_123_e1206: f64 = (1.0 - noise_variable_5);
        let noise_metadata_schedule_123_e1207: f64 = (noise_variable_173 * noise_metadata_schedule_123_e1206);
        let noise_metadata_schedule_123_e1208: f64 = (noise_metadata_schedule_123_e1202 + noise_metadata_schedule_123_e1207);
        let noise_metadata_schedule_123_e1211: f64 = (noise_variable_168 * noise_variable_2);
        let noise_metadata_schedule_123_e1213: f64 = (noise_metadata_schedule_123_e1211 * noise_variable_6);
        let noise_metadata_schedule_123_e1214: f64 = (noise_metadata_schedule_123_e1208 - noise_metadata_schedule_123_e1213);
        (noise_metadata_schedule_123_e1214,)
    } else {
        (noise_variable_97,)
    }
};
            noise_variable_97 = noise_metadata_schedule_123_e1216;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_124_e1240,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_124_e1221: f64 = (2.0 * noise_variable_2);
        let noise_metadata_schedule_124_e1227: f64 = (-noise_variable_97);
        let noise_metadata_schedule_124_e1229: f64 = (noise_metadata_schedule_124_e1227 * noise_variable_3);
        let noise_metadata_schedule_124_e1230: f64 = (noise_metadata_schedule_124_e1229).exp();
        let noise_metadata_schedule_124_e1231: f64 = (4.0 * noise_metadata_schedule_124_e1230);
        let noise_metadata_schedule_124_e1232: f64 = (1.0 + noise_metadata_schedule_124_e1231);
        let noise_metadata_schedule_124_e1233: f64 = (noise_metadata_schedule_124_e1232).sqrt();
        let noise_metadata_schedule_124_e1234: f64 = (1.0 + noise_metadata_schedule_124_e1233);
        let noise_metadata_schedule_124_e1235: f64 = (0.5 * noise_metadata_schedule_124_e1234);
        let noise_metadata_schedule_124_e1236: f64 = (noise_metadata_schedule_124_e1235).ln();
        let noise_metadata_schedule_124_e1237: f64 = (noise_metadata_schedule_124_e1221 * noise_metadata_schedule_124_e1236);
        let noise_metadata_schedule_124_e1238: f64 = (noise_variable_97 + noise_metadata_schedule_124_e1237);
        (noise_metadata_schedule_124_e1238,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_124_e1240;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_125_e1252,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_125_e1246: f64 = (params.p38 / noise_variable_22);
        let noise_metadata_schedule_125_e1247: f64 = (noise_metadata_schedule_125_e1246).ln();
        let noise_metadata_schedule_125_e1248: f64 = (params.p39 * noise_metadata_schedule_125_e1247);
        let noise_metadata_schedule_125_e1249: f64 = (noise_metadata_schedule_125_e1248).exp();
        let noise_metadata_schedule_125_e1250: f64 = (noise_variable_27 * noise_metadata_schedule_125_e1249);
        (noise_metadata_schedule_125_e1250,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_125_e1252;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_126_e1260,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_126_e1256: f64 = (params.p40 * noise_variable_22);
        let noise_metadata_schedule_126_e1258: f64 = (noise_metadata_schedule_126_e1256 / params.p38);
        (noise_metadata_schedule_126_e1258,)
    } else {
        (noise_variable_44,)
    }
};
            noise_variable_44 = noise_metadata_schedule_126_e1260;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_127_e1273,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_127_e1265: f64 = (params.p82 * noise_variable_6);
        let noise_metadata_schedule_127_e1268: f64 = (params.p77 * noise_variable_10);
        let noise_metadata_schedule_127_e1269: f64 = (noise_metadata_schedule_127_e1265 + noise_metadata_schedule_127_e1268);
        let noise_metadata_schedule_127_e1270: f64 = (noise_metadata_schedule_127_e1269).exp();
        let noise_metadata_schedule_127_e1271: f64 = (params.p15 * noise_metadata_schedule_127_e1270);
        (noise_metadata_schedule_127_e1271,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_127_e1273;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_128_e1290,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_128_e1278: f64 = (0.5 * noise_variable_168);
        let noise_metadata_schedule_128_e1280: f64 = (noise_metadata_schedule_128_e1278 * noise_variable_6);
        let noise_metadata_schedule_128_e1283: f64 = (0.5 * noise_variable_173);
        let noise_metadata_schedule_128_e1285: f64 = (noise_metadata_schedule_128_e1283 * noise_variable_10);
        let noise_metadata_schedule_128_e1286: f64 = (noise_metadata_schedule_128_e1280 + noise_metadata_schedule_128_e1285);
        let noise_metadata_schedule_128_e1287: f64 = (noise_metadata_schedule_128_e1286).exp();
        let noise_metadata_schedule_128_e1288: f64 = (params.p17 * noise_metadata_schedule_128_e1287);
        (noise_metadata_schedule_128_e1288,)
    } else {
        (noise_variable_12,)
    }
};
            noise_variable_12 = noise_metadata_schedule_128_e1290;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_129_e1298,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_129_e1294: f64 = (0.5 * params.p42);
        let noise_metadata_schedule_129_e1296: f64 = (noise_metadata_schedule_129_e1294 / noise_variable_177);
        (noise_metadata_schedule_129_e1296,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_129_e1298;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_130_e1312,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_130_e1302: f64 = (2.0 * noise_variable_177);
        let noise_metadata_schedule_130_e1304: f64 = (noise_variable_178).exp();
        let noise_metadata_schedule_130_e1306: f64 = (-noise_variable_178);
        let noise_metadata_schedule_130_e1307: f64 = (noise_metadata_schedule_130_e1306).exp();
        let noise_metadata_schedule_130_e1308: f64 = (noise_metadata_schedule_130_e1304 - noise_metadata_schedule_130_e1307);
        let noise_metadata_schedule_130_e1309: f64 = (noise_metadata_schedule_130_e1308).ln();
        let noise_metadata_schedule_130_e1310: f64 = (noise_metadata_schedule_130_e1302 * noise_metadata_schedule_130_e1309);
        (noise_metadata_schedule_130_e1310,)
    } else {
        (noise_variable_96,)
    }
};
            noise_variable_96 = noise_metadata_schedule_130_e1312;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_131_e1330,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_131_e1316: f64 = (noise_variable_96 * noise_variable_5);
        let noise_metadata_schedule_131_e1320: f64 = (1.0 - noise_variable_5);
        let noise_metadata_schedule_131_e1321: f64 = (noise_variable_174 * noise_metadata_schedule_131_e1320);
        let noise_metadata_schedule_131_e1322: f64 = (noise_metadata_schedule_131_e1316 + noise_metadata_schedule_131_e1321);
        let noise_metadata_schedule_131_e1325: f64 = (noise_variable_168 * noise_variable_2);
        let noise_metadata_schedule_131_e1327: f64 = (noise_metadata_schedule_131_e1325 * noise_variable_6);
        let noise_metadata_schedule_131_e1328: f64 = (noise_metadata_schedule_131_e1322 - noise_metadata_schedule_131_e1327);
        (noise_metadata_schedule_131_e1328,)
    } else {
        (noise_variable_97,)
    }
};
            noise_variable_97 = noise_metadata_schedule_131_e1330;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_132_e1354,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_132_e1335: f64 = (2.0 * noise_variable_2);
        let noise_metadata_schedule_132_e1341: f64 = (-noise_variable_97);
        let noise_metadata_schedule_132_e1343: f64 = (noise_metadata_schedule_132_e1341 * noise_variable_3);
        let noise_metadata_schedule_132_e1344: f64 = (noise_metadata_schedule_132_e1343).exp();
        let noise_metadata_schedule_132_e1345: f64 = (4.0 * noise_metadata_schedule_132_e1344);
        let noise_metadata_schedule_132_e1346: f64 = (1.0 + noise_metadata_schedule_132_e1345);
        let noise_metadata_schedule_132_e1347: f64 = (noise_metadata_schedule_132_e1346).sqrt();
        let noise_metadata_schedule_132_e1348: f64 = (1.0 + noise_metadata_schedule_132_e1347);
        let noise_metadata_schedule_132_e1349: f64 = (0.5 * noise_metadata_schedule_132_e1348);
        let noise_metadata_schedule_132_e1350: f64 = (noise_metadata_schedule_132_e1349).ln();
        let noise_metadata_schedule_132_e1351: f64 = (noise_metadata_schedule_132_e1335 * noise_metadata_schedule_132_e1350);
        let noise_metadata_schedule_132_e1352: f64 = (noise_variable_97 + noise_metadata_schedule_132_e1351);
        (noise_metadata_schedule_132_e1352,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_132_e1354;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_133_e1366,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_133_e1360: f64 = (params.p42 / noise_variable_17);
        let noise_metadata_schedule_133_e1361: f64 = (noise_metadata_schedule_133_e1360).ln();
        let noise_metadata_schedule_133_e1362: f64 = (params.p43 * noise_metadata_schedule_133_e1361);
        let noise_metadata_schedule_133_e1363: f64 = (noise_metadata_schedule_133_e1362).exp();
        let noise_metadata_schedule_133_e1364: f64 = (params.p41 * noise_metadata_schedule_133_e1363);
        (noise_metadata_schedule_133_e1364,)
    } else {
        (noise_variable_24,)
    }
};
            noise_variable_24 = noise_metadata_schedule_133_e1366;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_134_e1379,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_134_e1371: f64 = (noise_variable_169 * noise_variable_6);
        let noise_metadata_schedule_134_e1374: f64 = (params.p78 * noise_variable_10);
        let noise_metadata_schedule_134_e1375: f64 = (noise_metadata_schedule_134_e1371 + noise_metadata_schedule_134_e1374);
        let noise_metadata_schedule_134_e1376: f64 = (noise_metadata_schedule_134_e1375).exp();
        let noise_metadata_schedule_134_e1377: f64 = (params.p19 * noise_metadata_schedule_134_e1376);
        (noise_metadata_schedule_134_e1377,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_134_e1379;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_135_e1392,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_135_e1384: f64 = (params.p81 * noise_variable_6);
        let noise_metadata_schedule_135_e1387: f64 = (params.p76 * noise_variable_10);
        let noise_metadata_schedule_135_e1388: f64 = (noise_metadata_schedule_135_e1384 + noise_metadata_schedule_135_e1387);
        let noise_metadata_schedule_135_e1389: f64 = (noise_metadata_schedule_135_e1388).exp();
        let noise_metadata_schedule_135_e1390: f64 = (params.p1 * noise_metadata_schedule_135_e1389);
        (noise_metadata_schedule_135_e1390,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_135_e1392;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_136_e1405,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_136_e1397: f64 = (params.p95 * noise_variable_6);
        let noise_metadata_schedule_136_e1400: f64 = (params.p83 * noise_variable_10);
        let noise_metadata_schedule_136_e1401: f64 = (noise_metadata_schedule_136_e1397 - noise_metadata_schedule_136_e1400);
        let noise_metadata_schedule_136_e1402: f64 = (noise_metadata_schedule_136_e1401).exp();
        let noise_metadata_schedule_136_e1403: f64 = (params.p9 * noise_metadata_schedule_136_e1402);
        (noise_metadata_schedule_136_e1403,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_136_e1405;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_137_e1416,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_137_e1410: f64 = (params.p87 - noise_variable_172);
        let noise_metadata_schedule_137_e1412: f64 = (noise_metadata_schedule_137_e1410 * noise_variable_6);
        let noise_metadata_schedule_137_e1413: f64 = (noise_metadata_schedule_137_e1412).exp();
        let noise_metadata_schedule_137_e1414: f64 = (params.p62 * noise_metadata_schedule_137_e1413);
        (noise_metadata_schedule_137_e1414,)
    } else {
        (noise_variable_33,)
    }
};
            noise_variable_33 = noise_metadata_schedule_137_e1416;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_138_e1425,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_138_e1421: f64 = (params.p87 * noise_variable_6);
        let noise_metadata_schedule_138_e1422: f64 = (noise_metadata_schedule_138_e1421).exp();
        let noise_metadata_schedule_138_e1423: f64 = (params.p61 * noise_metadata_schedule_138_e1422);
        (noise_metadata_schedule_138_e1423,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_138_e1425;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_139_e1431,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_139_e1429: f64 = (1.0 / noise_variable_31);
        (noise_metadata_schedule_139_e1429,)
    } else {
        (noise_variable_32,)
    }
};
            noise_variable_32 = noise_metadata_schedule_139_e1431;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_140_e1441,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_140_e1437: f64 = (params.p89 * noise_variable_7);
        let noise_metadata_schedule_140_e1438: f64 = (1.0 + noise_metadata_schedule_140_e1437);
        let noise_metadata_schedule_140_e1439: f64 = (params.p64 * noise_metadata_schedule_140_e1438);
        (noise_metadata_schedule_140_e1439,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_140_e1441;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_141_e1444: f64 = if params.p65 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_256 = noise_metadata_schedule_141_e1444;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_142_e1456,) = {
    if ((noise_variable_253 != 0.0) && (noise_variable_256 != 0.0)) {
        let noise_metadata_schedule_142_e1452: f64 = (params.p90 * noise_variable_7);
        let noise_metadata_schedule_142_e1453: f64 = (1.0 - noise_metadata_schedule_142_e1452);
        let noise_metadata_schedule_142_e1454: f64 = (params.p65 * noise_metadata_schedule_142_e1453);
        (noise_metadata_schedule_142_e1454,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_142_e1456;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_143_e1462,) = {
    if ((noise_variable_253 != 0.0) && (noise_variable_256 != 0.0)) {
        (params.p64,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_143_e1462;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_144_e1475,) = {
    if ((noise_variable_253 != 0.0) && (noise_variable_256 == 0.0)) {
        let noise_metadata_schedule_144_e1471: f64 = (params.p89 * noise_variable_7);
        let noise_metadata_schedule_144_e1472: f64 = (1.0 + noise_metadata_schedule_144_e1471);
        let noise_metadata_schedule_144_e1473: f64 = (params.p64 * noise_metadata_schedule_144_e1472);
        (noise_metadata_schedule_144_e1473,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_144_e1475;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_145_e1482,) = {
    if ((noise_variable_253 != 0.0) && (noise_variable_256 == 0.0)) {
        (params.p65,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_145_e1482;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_146_e1498,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_146_e1488: f64 = (params.p85 * noise_variable_7);
        let noise_metadata_schedule_146_e1489: f64 = (1.0 + noise_metadata_schedule_146_e1488);
        let noise_metadata_schedule_146_e1492: f64 = (params.p86 * noise_variable_7);
        let noise_metadata_schedule_146_e1494: f64 = (noise_metadata_schedule_146_e1492 * noise_variable_7);
        let noise_metadata_schedule_146_e1495: f64 = (noise_metadata_schedule_146_e1489 + noise_metadata_schedule_146_e1494);
        let noise_metadata_schedule_146_e1496: f64 = (params.p54 * noise_metadata_schedule_146_e1495);
        (noise_metadata_schedule_146_e1496,)
    } else {
        (noise_variable_42,)
    }
};
            noise_variable_42 = noise_metadata_schedule_146_e1498;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_156_e1582,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_156_e1578: f64 = (params.p91 * noise_variable_6);
        let noise_metadata_schedule_156_e1579: f64 = (noise_metadata_schedule_156_e1578).exp();
        let noise_metadata_schedule_156_e1580: f64 = (params.p23 * noise_metadata_schedule_156_e1579);
        (noise_metadata_schedule_156_e1580,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_156_e1582;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_157_e1590,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_157_e1586: f64 = (0.5 * params.p46);
        let noise_metadata_schedule_157_e1588: f64 = (noise_metadata_schedule_157_e1586 / noise_variable_177);
        (noise_metadata_schedule_157_e1588,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_157_e1590;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_158_e1604,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_158_e1594: f64 = (2.0 * noise_variable_177);
        let noise_metadata_schedule_158_e1596: f64 = (noise_variable_178).exp();
        let noise_metadata_schedule_158_e1598: f64 = (-noise_variable_178);
        let noise_metadata_schedule_158_e1599: f64 = (noise_metadata_schedule_158_e1598).exp();
        let noise_metadata_schedule_158_e1600: f64 = (noise_metadata_schedule_158_e1596 - noise_metadata_schedule_158_e1599);
        let noise_metadata_schedule_158_e1601: f64 = (noise_metadata_schedule_158_e1600).ln();
        let noise_metadata_schedule_158_e1602: f64 = (noise_metadata_schedule_158_e1594 * noise_metadata_schedule_158_e1601);
        (noise_metadata_schedule_158_e1602,)
    } else {
        (noise_variable_96,)
    }
};
            noise_variable_96 = noise_metadata_schedule_158_e1604;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_159_e1622,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_159_e1608: f64 = (noise_variable_96 * noise_variable_5);
        let noise_metadata_schedule_159_e1612: f64 = (1.0 - noise_variable_5);
        let noise_metadata_schedule_159_e1613: f64 = (noise_variable_174 * noise_metadata_schedule_159_e1612);
        let noise_metadata_schedule_159_e1614: f64 = (noise_metadata_schedule_159_e1608 + noise_metadata_schedule_159_e1613);
        let noise_metadata_schedule_159_e1617: f64 = (noise_variable_168 * noise_variable_2);
        let noise_metadata_schedule_159_e1619: f64 = (noise_metadata_schedule_159_e1617 * noise_variable_6);
        let noise_metadata_schedule_159_e1620: f64 = (noise_metadata_schedule_159_e1614 - noise_metadata_schedule_159_e1619);
        (noise_metadata_schedule_159_e1620,)
    } else {
        (noise_variable_97,)
    }
};
            noise_variable_97 = noise_metadata_schedule_159_e1622;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_160_e1646,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_160_e1627: f64 = (2.0 * noise_variable_2);
        let noise_metadata_schedule_160_e1633: f64 = (-noise_variable_97);
        let noise_metadata_schedule_160_e1635: f64 = (noise_metadata_schedule_160_e1633 * noise_variable_3);
        let noise_metadata_schedule_160_e1636: f64 = (noise_metadata_schedule_160_e1635).exp();
        let noise_metadata_schedule_160_e1637: f64 = (4.0 * noise_metadata_schedule_160_e1636);
        let noise_metadata_schedule_160_e1638: f64 = (1.0 + noise_metadata_schedule_160_e1637);
        let noise_metadata_schedule_160_e1639: f64 = (noise_metadata_schedule_160_e1638).sqrt();
        let noise_metadata_schedule_160_e1640: f64 = (1.0 + noise_metadata_schedule_160_e1639);
        let noise_metadata_schedule_160_e1641: f64 = (0.5 * noise_metadata_schedule_160_e1640);
        let noise_metadata_schedule_160_e1642: f64 = (noise_metadata_schedule_160_e1641).ln();
        let noise_metadata_schedule_160_e1643: f64 = (noise_metadata_schedule_160_e1627 * noise_metadata_schedule_160_e1642);
        let noise_metadata_schedule_160_e1644: f64 = (noise_variable_97 + noise_metadata_schedule_160_e1643);
        (noise_metadata_schedule_160_e1644,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_160_e1646;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_161_e1658,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_161_e1652: f64 = (params.p46 / noise_variable_18);
        let noise_metadata_schedule_161_e1653: f64 = (noise_metadata_schedule_161_e1652).ln();
        let noise_metadata_schedule_161_e1654: f64 = (params.p47 * noise_metadata_schedule_161_e1653);
        let noise_metadata_schedule_161_e1655: f64 = (noise_metadata_schedule_161_e1654).exp();
        let noise_metadata_schedule_161_e1656: f64 = (params.p45 * noise_metadata_schedule_161_e1655);
        (noise_metadata_schedule_161_e1656,)
    } else {
        (noise_variable_25,)
    }
};
            noise_variable_25 = noise_metadata_schedule_161_e1658;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_169_e1769,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_169_e1765: f64 = (params.p97 * noise_variable_6);
        let noise_metadata_schedule_169_e1766: f64 = (noise_metadata_schedule_169_e1765).exp();
        let noise_metadata_schedule_169_e1767: f64 = (params.p7 * noise_metadata_schedule_169_e1766);
        (noise_metadata_schedule_169_e1767,)
    } else {
        (noise_variable_200,)
    }
};
            noise_variable_200 = noise_metadata_schedule_169_e1769;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_170_e1785,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_170_e1774: f64 = (params.p83 * noise_variable_3);
        let noise_metadata_schedule_170_e1777: f64 = (params.p84 * noise_variable_6);
        let noise_metadata_schedule_170_e1778: f64 = (noise_metadata_schedule_170_e1777).exp();
        let noise_metadata_schedule_170_e1780: f64 = (noise_metadata_schedule_170_e1778 - 1.0);
        let noise_metadata_schedule_170_e1781: f64 = (noise_metadata_schedule_170_e1774 * noise_metadata_schedule_170_e1780);
        let noise_metadata_schedule_170_e1782: f64 = (noise_metadata_schedule_170_e1781).exp();
        let noise_metadata_schedule_170_e1783: f64 = (params.p6 / noise_metadata_schedule_170_e1782);
        (noise_metadata_schedule_170_e1783,)
    } else {
        (noise_variable_202,)
    }
};
            noise_variable_202 = noise_metadata_schedule_170_e1785;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_171_e1788: f64 = if params.p0 <= 200.0 { 1.0 } else { 0.0 };
            noise_variable_259 = noise_metadata_schedule_171_e1788;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_172_e1802,) = {
    if ((noise_variable_253 != 0.0) && (noise_variable_259 != 0.0)) {
        let noise_metadata_schedule_172_e1797: f64 = (params.p102 * noise_variable_7);
        let noise_metadata_schedule_172_e1798: f64 = (params.p101 + noise_metadata_schedule_172_e1797);
        let noise_metadata_schedule_172_e1799: f64 = (noise_variable_7 * noise_metadata_schedule_172_e1798);
        let noise_metadata_schedule_172_e1800: f64 = (1.0 + noise_metadata_schedule_172_e1799);
        (noise_metadata_schedule_172_e1800,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_172_e1802;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_173_e1812,) = {
    if ((noise_variable_253 != 0.0) && (noise_variable_259 == 0.0)) {
        let noise_metadata_schedule_173_e1809: f64 = (params.p98 * noise_variable_6);
        let noise_metadata_schedule_173_e1810: f64 = (noise_metadata_schedule_173_e1809).exp();
        (noise_metadata_schedule_173_e1810,)
    } else {
        (noise_variable_204,)
    }
};
            noise_variable_204 = noise_metadata_schedule_173_e1812;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_174_e1818,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_174_e1816: f64 = (params.p12 * noise_variable_204);
        (noise_metadata_schedule_174_e1816,)
    } else {
        (noise_variable_203,)
    }
};
            noise_variable_203 = noise_metadata_schedule_174_e1818;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_175_e1829,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_175_e1822: f64 = (params.p13 * noise_variable_204);
        let noise_metadata_schedule_175_e1825: f64 = (noise_variable_176 * noise_variable_10);
        let noise_metadata_schedule_175_e1826: f64 = (noise_metadata_schedule_175_e1825).exp();
        let noise_metadata_schedule_175_e1827: f64 = (noise_metadata_schedule_175_e1822 * noise_metadata_schedule_175_e1826);
        (noise_metadata_schedule_175_e1827,)
    } else {
        (noise_variable_205,)
    }
};
            noise_variable_205 = noise_metadata_schedule_175_e1829;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_176_e1833,) = {
    if (noise_variable_253 != 0.0) {
        (params.p14,)
    } else {
        (noise_variable_206,)
    }
};
            noise_variable_206 = noise_metadata_schedule_176_e1833;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_177_e1842,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_177_e1838: f64 = (params.p93 * noise_variable_6);
        let noise_metadata_schedule_177_e1839: f64 = (noise_metadata_schedule_177_e1838).exp();
        let noise_metadata_schedule_177_e1840: f64 = (params.p29 * noise_metadata_schedule_177_e1839);
        (noise_metadata_schedule_177_e1840,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_177_e1842;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_178_e1851,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_178_e1847: f64 = (params.p92 * noise_variable_6);
        let noise_metadata_schedule_178_e1848: f64 = (noise_metadata_schedule_178_e1847).exp();
        let noise_metadata_schedule_178_e1849: f64 = (params.p26 * noise_metadata_schedule_178_e1848);
        (noise_metadata_schedule_178_e1849,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_178_e1851;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_179_e1860,) = {
    if (noise_variable_253 != 0.0) {
        let noise_metadata_schedule_179_e1856: f64 = (params.p94 * noise_variable_6);
        let noise_metadata_schedule_179_e1857: f64 = (noise_metadata_schedule_179_e1856).exp();
        let noise_metadata_schedule_179_e1858: f64 = (params.p28 * noise_metadata_schedule_179_e1857);
        (noise_metadata_schedule_179_e1858,)
    } else {
        (noise_variable_41,)
    }
};
            noise_variable_41 = noise_metadata_schedule_179_e1860;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_181_e1878: f64 = if noise_variable_25 <= 1e-30 { 1.0 } else { 0.0 };
            noise_variable_260 = noise_metadata_schedule_181_e1878;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_182_e1884,) = {
    if (noise_variable_260 != 0.0) {
        let noise_metadata_schedule_182_e1882: f64 = (noise_variable_24 * params.p49);
        (noise_metadata_schedule_182_e1882,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_182_e1884;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_185_e1900,) = {
    if (noise_variable_260 != 0.0) {
        let noise_metadata_schedule_185_e1897: f64 = (1.0 - params.p49);
        let noise_metadata_schedule_185_e1898: f64 = (noise_variable_24 * noise_metadata_schedule_185_e1897);
        (noise_metadata_schedule_185_e1898,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_185_e1900;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_186_e1903: f64 = if params.p44 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_261 = noise_metadata_schedule_186_e1903;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_187_e1906: f64 = if noise_variable_113 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_262 = noise_metadata_schedule_187_e1906;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_188_e1916,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_188_e1914: f64 = (params.p43 / 4.0);
        (noise_metadata_schedule_188_e1914,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_188_e1916;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_189_e1926,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_189_e1924: f64 = (params.p44 - noise_variable_17);
        (noise_metadata_schedule_189_e1924,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_189_e1926;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_190_e1943,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_190_e1935: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_190_e1936: f64 = (-noise_metadata_schedule_190_e1935);
        let noise_metadata_schedule_190_e1938: f64 = (noise_metadata_schedule_190_e1936 / params.p43);
        let noise_metadata_schedule_190_e1939: f64 = (noise_metadata_schedule_190_e1938).exp();
        let noise_metadata_schedule_190_e1940: f64 = (1.0 - noise_metadata_schedule_190_e1939);
        let noise_metadata_schedule_190_e1941: f64 = (noise_variable_17 * noise_metadata_schedule_190_e1940);
        (noise_metadata_schedule_190_e1941,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_190_e1943;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_191_e1953,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_191_e1951: f64 = (2.4 * noise_variable_113);
        (noise_metadata_schedule_191_e1951,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_191_e1953;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_192_e1971,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_192_e1962: f64 = (noise_variable_50 - params.p43);
        let noise_metadata_schedule_192_e1965: f64 = (params.p44 / noise_variable_17);
        let noise_metadata_schedule_192_e1966: f64 = (noise_metadata_schedule_192_e1965).ln();
        let noise_metadata_schedule_192_e1967: f64 = (noise_metadata_schedule_192_e1962 * noise_metadata_schedule_192_e1966);
        let noise_metadata_schedule_192_e1968: f64 = (noise_metadata_schedule_192_e1967).exp();
        let noise_metadata_schedule_192_e1969: f64 = (noise_variable_113 * noise_metadata_schedule_192_e1968);
        (noise_metadata_schedule_192_e1969,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_192_e1971;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_193_e1983,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_193_e1979: f64 = (noise_variable_52 - noise_variable_183);
        let noise_metadata_schedule_193_e1981: f64 = (noise_metadata_schedule_193_e1979 * noise_variable_3);
        (noise_metadata_schedule_193_e1981,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_193_e1983;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_194_e1986: f64 = if noise_variable_56 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_263 = noise_metadata_schedule_194_e1986;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_195_e1997,) = {
    if ((((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_195_e1995: f64 = (noise_variable_56).exp();
        (noise_metadata_schedule_195_e1995,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_195_e1997;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_197_e2028,) = {
    if ((((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_197_e2023: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_197_e2024: f64 = (noise_metadata_schedule_197_e2023).ln();
        let noise_metadata_schedule_197_e2025: f64 = (noise_variable_2 * noise_metadata_schedule_197_e2024);
        let noise_metadata_schedule_197_e2026: f64 = (noise_variable_52 - noise_metadata_schedule_197_e2025);
        (noise_metadata_schedule_197_e2026,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_197_e2028;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_199_e2050,) = {
    if ((((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) && (noise_variable_263 == 0.0)) {
        (noise_variable_183,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_199_e2050;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_200_e2064,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_200_e2058: f64 = (0.1 * noise_variable_51);
        let noise_metadata_schedule_200_e2061: f64 = (4.0 * noise_variable_2);
        let noise_metadata_schedule_200_e2062: f64 = (noise_metadata_schedule_200_e2058 + noise_metadata_schedule_200_e2061);
        (noise_metadata_schedule_200_e2062,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_200_e2064;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_201_e2076,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_201_e2072: f64 = (noise_variable_51 + noise_variable_58);
        let noise_metadata_schedule_201_e2074: f64 = (noise_metadata_schedule_201_e2072 / noise_variable_55);
        (noise_metadata_schedule_201_e2074,)
    } else {
        (noise_variable_59,)
    }
};
            noise_variable_59 = noise_metadata_schedule_201_e2076;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_202_e2079: f64 = if noise_variable_59 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_264 = noise_metadata_schedule_202_e2079;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_203_e2090,) = {
    if ((((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) && (noise_variable_264 != 0.0)) {
        let noise_metadata_schedule_203_e2088: f64 = (noise_variable_59).exp();
        (noise_metadata_schedule_203_e2088,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_203_e2090;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_205_e2130,) = {
    if ((((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) && (noise_variable_264 != 0.0)) {
        let noise_metadata_schedule_205_e2113: f64 = (-noise_variable_51);
        let noise_metadata_schedule_205_e2117: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_205_e2118: f64 = (noise_metadata_schedule_205_e2117).ln();
        let noise_metadata_schedule_205_e2121: f64 = (noise_variable_51 + noise_variable_52);
        let noise_metadata_schedule_205_e2122: f64 = (-noise_metadata_schedule_205_e2121);
        let noise_metadata_schedule_205_e2124: f64 = (noise_metadata_schedule_205_e2122 / noise_variable_55);
        let noise_metadata_schedule_205_e2125: f64 = (noise_metadata_schedule_205_e2124).exp();
        let noise_metadata_schedule_205_e2126: f64 = (noise_metadata_schedule_205_e2118 - noise_metadata_schedule_205_e2125);
        let noise_metadata_schedule_205_e2127: f64 = (noise_variable_55 * noise_metadata_schedule_205_e2126);
        let noise_metadata_schedule_205_e2128: f64 = (noise_metadata_schedule_205_e2113 + noise_metadata_schedule_205_e2127);
        (noise_metadata_schedule_205_e2128,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_205_e2130;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_207_e2152,) = {
    if ((((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) && (noise_variable_264 == 0.0)) {
        (noise_variable_58,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_207_e2152;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_208_e2162,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_208_e2160: f64 = (noise_variable_183 - noise_variable_58);
        (noise_metadata_schedule_208_e2160,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_208_e2162;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_209_e2175,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_209_e2171: f64 = (noise_variable_58 / noise_variable_17);
        let noise_metadata_schedule_209_e2172: f64 = (1.0 - noise_metadata_schedule_209_e2171);
        let noise_metadata_schedule_209_e2173: f64 = (noise_metadata_schedule_209_e2172).ln();
        (noise_metadata_schedule_209_e2173,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_209_e2175;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_210_e2188,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_210_e2184: f64 = (noise_variable_60 / noise_variable_17);
        let noise_metadata_schedule_210_e2185: f64 = (1.0 - noise_metadata_schedule_210_e2184);
        let noise_metadata_schedule_210_e2186: f64 = (noise_metadata_schedule_210_e2185).ln();
        (noise_metadata_schedule_210_e2186,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_210_e2188;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_211_e2198,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_211_e2196: f64 = (1.0 - params.p43);
        (noise_metadata_schedule_211_e2196,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_211_e2198;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_212_e2208,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_212_e2206: f64 = (1.0 - noise_variable_50);
        (noise_metadata_schedule_212_e2206,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_212_e2208;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_217_e2285,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_217_e2278: f64 = (noise_variable_66 * noise_variable_67);
        let noise_metadata_schedule_217_e2279: f64 = (noise_metadata_schedule_217_e2278).exp();
        let noise_metadata_schedule_217_e2280: f64 = (1.0 - noise_metadata_schedule_217_e2279);
        let noise_metadata_schedule_217_e2281: f64 = (noise_variable_113 * noise_metadata_schedule_217_e2280);
        let noise_metadata_schedule_217_e2283: f64 = (noise_metadata_schedule_217_e2281 / noise_variable_67);
        (noise_metadata_schedule_217_e2283,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_217_e2285;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_218_e2302,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_218_e2295: f64 = (noise_variable_65 * noise_variable_68);
        let noise_metadata_schedule_218_e2296: f64 = (noise_metadata_schedule_218_e2295).exp();
        let noise_metadata_schedule_218_e2297: f64 = (1.0 - noise_metadata_schedule_218_e2296);
        let noise_metadata_schedule_218_e2298: f64 = (noise_variable_54 * noise_metadata_schedule_218_e2297);
        let noise_metadata_schedule_218_e2300: f64 = (noise_metadata_schedule_218_e2298 / noise_variable_68);
        (noise_metadata_schedule_218_e2300,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_218_e2302;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_219_e2319,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 != 0.0)) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_219_e2312: f64 = (noise_variable_66 * noise_variable_68);
        let noise_metadata_schedule_219_e2313: f64 = (noise_metadata_schedule_219_e2312).exp();
        let noise_metadata_schedule_219_e2314: f64 = (1.0 - noise_metadata_schedule_219_e2313);
        let noise_metadata_schedule_219_e2315: f64 = (noise_variable_54 * noise_metadata_schedule_219_e2314);
        let noise_metadata_schedule_219_e2317: f64 = (noise_metadata_schedule_219_e2315 / noise_variable_68);
        (noise_metadata_schedule_219_e2317,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_219_e2319;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_223_e2358: f64 = if noise_variable_113 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_265 = noise_metadata_schedule_223_e2358;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_224_e2376,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_224_e2368: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_224_e2369: f64 = (-noise_metadata_schedule_224_e2368);
        let noise_metadata_schedule_224_e2371: f64 = (noise_metadata_schedule_224_e2369 / params.p43);
        let noise_metadata_schedule_224_e2372: f64 = (noise_metadata_schedule_224_e2371).exp();
        let noise_metadata_schedule_224_e2373: f64 = (1.0 - noise_metadata_schedule_224_e2372);
        let noise_metadata_schedule_224_e2374: f64 = (noise_variable_17 * noise_metadata_schedule_224_e2373);
        (noise_metadata_schedule_224_e2374,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_224_e2376;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_225_e2389,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_225_e2385: f64 = (noise_variable_76 - noise_variable_183);
        let noise_metadata_schedule_225_e2387: f64 = (noise_metadata_schedule_225_e2385 * noise_variable_3);
        (noise_metadata_schedule_225_e2387,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_225_e2389;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_226_e2403,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_226_e2398: f64 = (noise_variable_80 * noise_variable_80);
        let noise_metadata_schedule_226_e2400: f64 = (noise_metadata_schedule_226_e2398 + 1.921812);
        let noise_metadata_schedule_226_e2401: f64 = (noise_metadata_schedule_226_e2400).sqrt();
        (noise_metadata_schedule_226_e2401,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_226_e2403;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_227_e2416,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_227_e2412: f64 = (noise_variable_80 + noise_variable_81);
        let noise_metadata_schedule_227_e2414: f64 = (noise_metadata_schedule_227_e2412 * 0.5);
        (noise_metadata_schedule_227_e2414,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_227_e2416;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_228_e2429,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_228_e2426: f64 = (noise_variable_2 * noise_variable_82);
        let noise_metadata_schedule_228_e2427: f64 = (noise_variable_76 - noise_metadata_schedule_228_e2426);
        (noise_metadata_schedule_228_e2427,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_228_e2429;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_230_e2454,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_230_e2450: f64 = (noise_variable_77 / noise_variable_17);
        let noise_metadata_schedule_230_e2451: f64 = (1.0 - noise_metadata_schedule_230_e2450);
        let noise_metadata_schedule_230_e2452: f64 = (noise_metadata_schedule_230_e2451).ln();
        (noise_metadata_schedule_230_e2452,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_230_e2454;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_233_e2508,) = {
    if (((noise_variable_260 != 0.0) && (noise_variable_261 == 0.0)) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_233_e2498: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_233_e2499: f64 = (noise_variable_78 * noise_metadata_schedule_233_e2498);
        let noise_metadata_schedule_233_e2500: f64 = (noise_metadata_schedule_233_e2499).exp();
        let noise_metadata_schedule_233_e2501: f64 = (1.0 - noise_metadata_schedule_233_e2500);
        let noise_metadata_schedule_233_e2502: f64 = (noise_variable_17 * noise_metadata_schedule_233_e2501);
        let noise_metadata_schedule_233_e2505: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_233_e2506: f64 = (noise_metadata_schedule_233_e2502 / noise_metadata_schedule_233_e2505);
        (noise_metadata_schedule_233_e2506,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_233_e2508;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_237_e2550,) = {
    if (noise_variable_260 == 0.0) {
        (noise_variable_24,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_237_e2550;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_238_e2557,) = {
    if (noise_variable_260 == 0.0) {
        let noise_metadata_schedule_238_e2555: f64 = (noise_variable_25 * params.p49);
        (noise_metadata_schedule_238_e2555,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_238_e2557;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_239_e2560: f64 = if params.p48 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_266 = noise_metadata_schedule_239_e2560;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_240_e2563: f64 = if noise_variable_112 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_267 = noise_metadata_schedule_240_e2563;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_241_e2574,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_241_e2572: f64 = (params.p47 / 4.0);
        (noise_metadata_schedule_241_e2572,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_241_e2574;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_242_e2585,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_242_e2583: f64 = (params.p48 - noise_variable_18);
        (noise_metadata_schedule_242_e2583,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_242_e2585;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_243_e2603,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_243_e2595: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_243_e2596: f64 = (-noise_metadata_schedule_243_e2595);
        let noise_metadata_schedule_243_e2598: f64 = (noise_metadata_schedule_243_e2596 / params.p47);
        let noise_metadata_schedule_243_e2599: f64 = (noise_metadata_schedule_243_e2598).exp();
        let noise_metadata_schedule_243_e2600: f64 = (1.0 - noise_metadata_schedule_243_e2599);
        let noise_metadata_schedule_243_e2601: f64 = (noise_variable_18 * noise_metadata_schedule_243_e2600);
        (noise_metadata_schedule_243_e2601,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_243_e2603;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_244_e2614,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_244_e2612: f64 = (2.4 * noise_variable_112);
        (noise_metadata_schedule_244_e2612,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_244_e2614;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_245_e2633,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_245_e2624: f64 = (noise_variable_50 - params.p47);
        let noise_metadata_schedule_245_e2627: f64 = (params.p48 / noise_variable_18);
        let noise_metadata_schedule_245_e2628: f64 = (noise_metadata_schedule_245_e2627).ln();
        let noise_metadata_schedule_245_e2629: f64 = (noise_metadata_schedule_245_e2624 * noise_metadata_schedule_245_e2628);
        let noise_metadata_schedule_245_e2630: f64 = (noise_metadata_schedule_245_e2629).exp();
        let noise_metadata_schedule_245_e2631: f64 = (noise_variable_112 * noise_metadata_schedule_245_e2630);
        (noise_metadata_schedule_245_e2631,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_245_e2633;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_246_e2646,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_246_e2642: f64 = (noise_variable_52 - noise_variable_184);
        let noise_metadata_schedule_246_e2644: f64 = (noise_metadata_schedule_246_e2642 * noise_variable_3);
        (noise_metadata_schedule_246_e2644,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_246_e2646;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_247_e2649: f64 = if noise_variable_56 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_268 = noise_metadata_schedule_247_e2649;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_248_e2661,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) && (noise_variable_268 != 0.0)) {
        let noise_metadata_schedule_248_e2659: f64 = (noise_variable_56).exp();
        (noise_metadata_schedule_248_e2659,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_248_e2661;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_250_e2694,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) && (noise_variable_268 != 0.0)) {
        let noise_metadata_schedule_250_e2689: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_250_e2690: f64 = (noise_metadata_schedule_250_e2689).ln();
        let noise_metadata_schedule_250_e2691: f64 = (noise_variable_2 * noise_metadata_schedule_250_e2690);
        let noise_metadata_schedule_250_e2692: f64 = (noise_variable_52 - noise_metadata_schedule_250_e2691);
        (noise_metadata_schedule_250_e2692,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_250_e2694;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_252_e2718,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) && (noise_variable_268 == 0.0)) {
        (noise_variable_184,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_252_e2718;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_253_e2733,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_253_e2727: f64 = (0.1 * noise_variable_51);
        let noise_metadata_schedule_253_e2730: f64 = (4.0 * noise_variable_2);
        let noise_metadata_schedule_253_e2731: f64 = (noise_metadata_schedule_253_e2727 + noise_metadata_schedule_253_e2730);
        (noise_metadata_schedule_253_e2731,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_253_e2733;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_254_e2746,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_254_e2742: f64 = (noise_variable_51 + noise_variable_58);
        let noise_metadata_schedule_254_e2744: f64 = (noise_metadata_schedule_254_e2742 / noise_variable_55);
        (noise_metadata_schedule_254_e2744,)
    } else {
        (noise_variable_59,)
    }
};
            noise_variable_59 = noise_metadata_schedule_254_e2746;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_255_e2749: f64 = if noise_variable_59 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_269 = noise_metadata_schedule_255_e2749;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_256_e2761,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) && (noise_variable_269 != 0.0)) {
        let noise_metadata_schedule_256_e2759: f64 = (noise_variable_59).exp();
        (noise_metadata_schedule_256_e2759,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_256_e2761;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_258_e2803,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) && (noise_variable_269 != 0.0)) {
        let noise_metadata_schedule_258_e2786: f64 = (-noise_variable_51);
        let noise_metadata_schedule_258_e2790: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_258_e2791: f64 = (noise_metadata_schedule_258_e2790).ln();
        let noise_metadata_schedule_258_e2794: f64 = (noise_variable_51 + noise_variable_52);
        let noise_metadata_schedule_258_e2795: f64 = (-noise_metadata_schedule_258_e2794);
        let noise_metadata_schedule_258_e2797: f64 = (noise_metadata_schedule_258_e2795 / noise_variable_55);
        let noise_metadata_schedule_258_e2798: f64 = (noise_metadata_schedule_258_e2797).exp();
        let noise_metadata_schedule_258_e2799: f64 = (noise_metadata_schedule_258_e2791 - noise_metadata_schedule_258_e2798);
        let noise_metadata_schedule_258_e2800: f64 = (noise_variable_55 * noise_metadata_schedule_258_e2799);
        let noise_metadata_schedule_258_e2801: f64 = (noise_metadata_schedule_258_e2786 + noise_metadata_schedule_258_e2800);
        (noise_metadata_schedule_258_e2801,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_258_e2803;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_260_e2827,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) && (noise_variable_269 == 0.0)) {
        (noise_variable_58,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_260_e2827;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_261_e2838,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_261_e2836: f64 = (noise_variable_184 - noise_variable_58);
        (noise_metadata_schedule_261_e2836,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_261_e2838;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_262_e2852,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_262_e2848: f64 = (noise_variable_58 / noise_variable_18);
        let noise_metadata_schedule_262_e2849: f64 = (1.0 - noise_metadata_schedule_262_e2848);
        let noise_metadata_schedule_262_e2850: f64 = (noise_metadata_schedule_262_e2849).ln();
        (noise_metadata_schedule_262_e2850,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_262_e2852;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_263_e2866,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_263_e2862: f64 = (noise_variable_60 / noise_variable_18);
        let noise_metadata_schedule_263_e2863: f64 = (1.0 - noise_metadata_schedule_263_e2862);
        let noise_metadata_schedule_263_e2864: f64 = (noise_metadata_schedule_263_e2863).ln();
        (noise_metadata_schedule_263_e2864,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_263_e2866;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_264_e2877,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_264_e2875: f64 = (1.0 - params.p47);
        (noise_metadata_schedule_264_e2875,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_264_e2877;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_265_e2888,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_265_e2886: f64 = (1.0 - noise_variable_50);
        (noise_metadata_schedule_265_e2886,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_265_e2888;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_270_e2970,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_270_e2963: f64 = (noise_variable_66 * noise_variable_67);
        let noise_metadata_schedule_270_e2964: f64 = (noise_metadata_schedule_270_e2963).exp();
        let noise_metadata_schedule_270_e2965: f64 = (1.0 - noise_metadata_schedule_270_e2964);
        let noise_metadata_schedule_270_e2966: f64 = (noise_variable_112 * noise_metadata_schedule_270_e2965);
        let noise_metadata_schedule_270_e2968: f64 = (noise_metadata_schedule_270_e2966 / noise_variable_67);
        (noise_metadata_schedule_270_e2968,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_270_e2970;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_271_e2988,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_271_e2981: f64 = (noise_variable_65 * noise_variable_68);
        let noise_metadata_schedule_271_e2982: f64 = (noise_metadata_schedule_271_e2981).exp();
        let noise_metadata_schedule_271_e2983: f64 = (1.0 - noise_metadata_schedule_271_e2982);
        let noise_metadata_schedule_271_e2984: f64 = (noise_variable_54 * noise_metadata_schedule_271_e2983);
        let noise_metadata_schedule_271_e2986: f64 = (noise_metadata_schedule_271_e2984 / noise_variable_68);
        (noise_metadata_schedule_271_e2986,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_271_e2988;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_272_e3006,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_272_e2999: f64 = (noise_variable_66 * noise_variable_68);
        let noise_metadata_schedule_272_e3000: f64 = (noise_metadata_schedule_272_e2999).exp();
        let noise_metadata_schedule_272_e3001: f64 = (1.0 - noise_metadata_schedule_272_e3000);
        let noise_metadata_schedule_272_e3002: f64 = (noise_variable_54 * noise_metadata_schedule_272_e3001);
        let noise_metadata_schedule_272_e3004: f64 = (noise_metadata_schedule_272_e3002 / noise_variable_68);
        (noise_metadata_schedule_272_e3004,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_272_e3006;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_276_e3048: f64 = if noise_variable_112 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_270 = noise_metadata_schedule_276_e3048;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_277_e3067,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_277_e3059: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_277_e3060: f64 = (-noise_metadata_schedule_277_e3059);
        let noise_metadata_schedule_277_e3062: f64 = (noise_metadata_schedule_277_e3060 / params.p47);
        let noise_metadata_schedule_277_e3063: f64 = (noise_metadata_schedule_277_e3062).exp();
        let noise_metadata_schedule_277_e3064: f64 = (1.0 - noise_metadata_schedule_277_e3063);
        let noise_metadata_schedule_277_e3065: f64 = (noise_variable_18 * noise_metadata_schedule_277_e3064);
        (noise_metadata_schedule_277_e3065,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_277_e3067;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_278_e3081,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_278_e3077: f64 = (noise_variable_76 - noise_variable_184);
        let noise_metadata_schedule_278_e3079: f64 = (noise_metadata_schedule_278_e3077 * noise_variable_3);
        (noise_metadata_schedule_278_e3079,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_278_e3081;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_279_e3096,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_279_e3091: f64 = (noise_variable_80 * noise_variable_80);
        let noise_metadata_schedule_279_e3093: f64 = (noise_metadata_schedule_279_e3091 + 1.921812);
        let noise_metadata_schedule_279_e3094: f64 = (noise_metadata_schedule_279_e3093).sqrt();
        (noise_metadata_schedule_279_e3094,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_279_e3096;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_280_e3110,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_280_e3106: f64 = (noise_variable_80 + noise_variable_81);
        let noise_metadata_schedule_280_e3108: f64 = (noise_metadata_schedule_280_e3106 * 0.5);
        (noise_metadata_schedule_280_e3108,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_280_e3110;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_281_e3124,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_281_e3121: f64 = (noise_variable_2 * noise_variable_82);
        let noise_metadata_schedule_281_e3122: f64 = (noise_variable_76 - noise_metadata_schedule_281_e3121);
        (noise_metadata_schedule_281_e3122,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_281_e3124;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_283_e3151,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_283_e3147: f64 = (noise_variable_77 / noise_variable_18);
        let noise_metadata_schedule_283_e3148: f64 = (1.0 - noise_metadata_schedule_283_e3147);
        let noise_metadata_schedule_283_e3149: f64 = (noise_metadata_schedule_283_e3148).ln();
        (noise_metadata_schedule_283_e3149,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_283_e3151;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_286_e3208,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_266 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_286_e3198: f64 = (1.0 - params.p47);
        let noise_metadata_schedule_286_e3199: f64 = (noise_variable_78 * noise_metadata_schedule_286_e3198);
        let noise_metadata_schedule_286_e3200: f64 = (noise_metadata_schedule_286_e3199).exp();
        let noise_metadata_schedule_286_e3201: f64 = (1.0 - noise_metadata_schedule_286_e3200);
        let noise_metadata_schedule_286_e3202: f64 = (noise_variable_18 * noise_metadata_schedule_286_e3201);
        let noise_metadata_schedule_286_e3205: f64 = (1.0 - params.p47);
        let noise_metadata_schedule_286_e3206: f64 = (noise_metadata_schedule_286_e3202 / noise_metadata_schedule_286_e3205);
        (noise_metadata_schedule_286_e3206,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_286_e3208;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_290_e3257,) = {
    if (noise_variable_260 == 0.0) {
        let noise_metadata_schedule_290_e3254: f64 = (1.0 - params.p49);
        let noise_metadata_schedule_290_e3255: f64 = (noise_variable_25 * noise_metadata_schedule_290_e3254);
        (noise_metadata_schedule_290_e3255,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_290_e3257;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_291_e3260: f64 = if params.p48 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_271 = noise_metadata_schedule_291_e3260;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_292_e3263: f64 = if noise_variable_113 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_272 = noise_metadata_schedule_292_e3263;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_293_e3274,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_293_e3272: f64 = (params.p47 / 4.0);
        (noise_metadata_schedule_293_e3272,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_293_e3274;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_294_e3285,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_294_e3283: f64 = (params.p48 - noise_variable_18);
        (noise_metadata_schedule_294_e3283,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_294_e3285;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_295_e3303,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_295_e3295: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_295_e3296: f64 = (-noise_metadata_schedule_295_e3295);
        let noise_metadata_schedule_295_e3298: f64 = (noise_metadata_schedule_295_e3296 / params.p47);
        let noise_metadata_schedule_295_e3299: f64 = (noise_metadata_schedule_295_e3298).exp();
        let noise_metadata_schedule_295_e3300: f64 = (1.0 - noise_metadata_schedule_295_e3299);
        let noise_metadata_schedule_295_e3301: f64 = (noise_variable_18 * noise_metadata_schedule_295_e3300);
        (noise_metadata_schedule_295_e3301,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_295_e3303;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_296_e3314,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_296_e3312: f64 = (2.4 * noise_variable_113);
        (noise_metadata_schedule_296_e3312,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_296_e3314;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_297_e3333,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_297_e3324: f64 = (noise_variable_50 - params.p47);
        let noise_metadata_schedule_297_e3327: f64 = (params.p48 / noise_variable_18);
        let noise_metadata_schedule_297_e3328: f64 = (noise_metadata_schedule_297_e3327).ln();
        let noise_metadata_schedule_297_e3329: f64 = (noise_metadata_schedule_297_e3324 * noise_metadata_schedule_297_e3328);
        let noise_metadata_schedule_297_e3330: f64 = (noise_metadata_schedule_297_e3329).exp();
        let noise_metadata_schedule_297_e3331: f64 = (noise_variable_113 * noise_metadata_schedule_297_e3330);
        (noise_metadata_schedule_297_e3331,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_297_e3333;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_298_e3346,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_298_e3342: f64 = (noise_variable_52 - noise_variable_183);
        let noise_metadata_schedule_298_e3344: f64 = (noise_metadata_schedule_298_e3342 * noise_variable_3);
        (noise_metadata_schedule_298_e3344,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_298_e3346;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_299_e3349: f64 = if noise_variable_56 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_273 = noise_metadata_schedule_299_e3349;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_300_e3361,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) && (noise_variable_273 != 0.0)) {
        let noise_metadata_schedule_300_e3359: f64 = (noise_variable_56).exp();
        (noise_metadata_schedule_300_e3359,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_300_e3361;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_302_e3394,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) && (noise_variable_273 != 0.0)) {
        let noise_metadata_schedule_302_e3389: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_302_e3390: f64 = (noise_metadata_schedule_302_e3389).ln();
        let noise_metadata_schedule_302_e3391: f64 = (noise_variable_2 * noise_metadata_schedule_302_e3390);
        let noise_metadata_schedule_302_e3392: f64 = (noise_variable_52 - noise_metadata_schedule_302_e3391);
        (noise_metadata_schedule_302_e3392,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_302_e3394;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_304_e3418,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) && (noise_variable_273 == 0.0)) {
        (noise_variable_183,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_304_e3418;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_305_e3433,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_305_e3427: f64 = (0.1 * noise_variable_51);
        let noise_metadata_schedule_305_e3430: f64 = (4.0 * noise_variable_2);
        let noise_metadata_schedule_305_e3431: f64 = (noise_metadata_schedule_305_e3427 + noise_metadata_schedule_305_e3430);
        (noise_metadata_schedule_305_e3431,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_305_e3433;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_306_e3446,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_306_e3442: f64 = (noise_variable_51 + noise_variable_58);
        let noise_metadata_schedule_306_e3444: f64 = (noise_metadata_schedule_306_e3442 / noise_variable_55);
        (noise_metadata_schedule_306_e3444,)
    } else {
        (noise_variable_59,)
    }
};
            noise_variable_59 = noise_metadata_schedule_306_e3446;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_307_e3449: f64 = if noise_variable_59 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_274 = noise_metadata_schedule_307_e3449;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_308_e3461,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) && (noise_variable_274 != 0.0)) {
        let noise_metadata_schedule_308_e3459: f64 = (noise_variable_59).exp();
        (noise_metadata_schedule_308_e3459,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_308_e3461;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_310_e3503,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) && (noise_variable_274 != 0.0)) {
        let noise_metadata_schedule_310_e3486: f64 = (-noise_variable_51);
        let noise_metadata_schedule_310_e3490: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_310_e3491: f64 = (noise_metadata_schedule_310_e3490).ln();
        let noise_metadata_schedule_310_e3494: f64 = (noise_variable_51 + noise_variable_52);
        let noise_metadata_schedule_310_e3495: f64 = (-noise_metadata_schedule_310_e3494);
        let noise_metadata_schedule_310_e3497: f64 = (noise_metadata_schedule_310_e3495 / noise_variable_55);
        let noise_metadata_schedule_310_e3498: f64 = (noise_metadata_schedule_310_e3497).exp();
        let noise_metadata_schedule_310_e3499: f64 = (noise_metadata_schedule_310_e3491 - noise_metadata_schedule_310_e3498);
        let noise_metadata_schedule_310_e3500: f64 = (noise_variable_55 * noise_metadata_schedule_310_e3499);
        let noise_metadata_schedule_310_e3501: f64 = (noise_metadata_schedule_310_e3486 + noise_metadata_schedule_310_e3500);
        (noise_metadata_schedule_310_e3501,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_310_e3503;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_312_e3527,) = {
    if ((((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) && (noise_variable_274 == 0.0)) {
        (noise_variable_58,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_312_e3527;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_313_e3538,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_313_e3536: f64 = (noise_variable_183 - noise_variable_58);
        (noise_metadata_schedule_313_e3536,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_313_e3538;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_314_e3552,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_314_e3548: f64 = (noise_variable_58 / noise_variable_18);
        let noise_metadata_schedule_314_e3549: f64 = (1.0 - noise_metadata_schedule_314_e3548);
        let noise_metadata_schedule_314_e3550: f64 = (noise_metadata_schedule_314_e3549).ln();
        (noise_metadata_schedule_314_e3550,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_314_e3552;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_315_e3566,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_315_e3562: f64 = (noise_variable_60 / noise_variable_18);
        let noise_metadata_schedule_315_e3563: f64 = (1.0 - noise_metadata_schedule_315_e3562);
        let noise_metadata_schedule_315_e3564: f64 = (noise_metadata_schedule_315_e3563).ln();
        (noise_metadata_schedule_315_e3564,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_315_e3566;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_316_e3577,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_316_e3575: f64 = (1.0 - params.p47);
        (noise_metadata_schedule_316_e3575,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_316_e3577;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_317_e3588,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_317_e3586: f64 = (1.0 - noise_variable_50);
        (noise_metadata_schedule_317_e3586,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_317_e3588;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_322_e3670,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_322_e3663: f64 = (noise_variable_66 * noise_variable_67);
        let noise_metadata_schedule_322_e3664: f64 = (noise_metadata_schedule_322_e3663).exp();
        let noise_metadata_schedule_322_e3665: f64 = (1.0 - noise_metadata_schedule_322_e3664);
        let noise_metadata_schedule_322_e3666: f64 = (noise_variable_113 * noise_metadata_schedule_322_e3665);
        let noise_metadata_schedule_322_e3668: f64 = (noise_metadata_schedule_322_e3666 / noise_variable_67);
        (noise_metadata_schedule_322_e3668,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_322_e3670;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_323_e3688,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_323_e3681: f64 = (noise_variable_65 * noise_variable_68);
        let noise_metadata_schedule_323_e3682: f64 = (noise_metadata_schedule_323_e3681).exp();
        let noise_metadata_schedule_323_e3683: f64 = (1.0 - noise_metadata_schedule_323_e3682);
        let noise_metadata_schedule_323_e3684: f64 = (noise_variable_54 * noise_metadata_schedule_323_e3683);
        let noise_metadata_schedule_323_e3686: f64 = (noise_metadata_schedule_323_e3684 / noise_variable_68);
        (noise_metadata_schedule_323_e3686,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_323_e3688;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_324_e3706,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 != 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_324_e3699: f64 = (noise_variable_66 * noise_variable_68);
        let noise_metadata_schedule_324_e3700: f64 = (noise_metadata_schedule_324_e3699).exp();
        let noise_metadata_schedule_324_e3701: f64 = (1.0 - noise_metadata_schedule_324_e3700);
        let noise_metadata_schedule_324_e3702: f64 = (noise_variable_54 * noise_metadata_schedule_324_e3701);
        let noise_metadata_schedule_324_e3704: f64 = (noise_metadata_schedule_324_e3702 / noise_variable_68);
        (noise_metadata_schedule_324_e3704,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_324_e3706;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_328_e3748: f64 = if noise_variable_113 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_275 = noise_metadata_schedule_328_e3748;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_329_e3767,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_329_e3759: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_329_e3760: f64 = (-noise_metadata_schedule_329_e3759);
        let noise_metadata_schedule_329_e3762: f64 = (noise_metadata_schedule_329_e3760 / params.p47);
        let noise_metadata_schedule_329_e3763: f64 = (noise_metadata_schedule_329_e3762).exp();
        let noise_metadata_schedule_329_e3764: f64 = (1.0 - noise_metadata_schedule_329_e3763);
        let noise_metadata_schedule_329_e3765: f64 = (noise_variable_18 * noise_metadata_schedule_329_e3764);
        (noise_metadata_schedule_329_e3765,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_329_e3767;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_330_e3781,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_330_e3777: f64 = (noise_variable_76 - noise_variable_183);
        let noise_metadata_schedule_330_e3779: f64 = (noise_metadata_schedule_330_e3777 * noise_variable_3);
        (noise_metadata_schedule_330_e3779,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_330_e3781;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_331_e3796,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_331_e3791: f64 = (noise_variable_80 * noise_variable_80);
        let noise_metadata_schedule_331_e3793: f64 = (noise_metadata_schedule_331_e3791 + 1.921812);
        let noise_metadata_schedule_331_e3794: f64 = (noise_metadata_schedule_331_e3793).sqrt();
        (noise_metadata_schedule_331_e3794,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_331_e3796;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_332_e3810,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_332_e3806: f64 = (noise_variable_80 + noise_variable_81);
        let noise_metadata_schedule_332_e3808: f64 = (noise_metadata_schedule_332_e3806 * 0.5);
        (noise_metadata_schedule_332_e3808,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_332_e3810;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_333_e3824,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_333_e3821: f64 = (noise_variable_2 * noise_variable_82);
        let noise_metadata_schedule_333_e3822: f64 = (noise_variable_76 - noise_metadata_schedule_333_e3821);
        (noise_metadata_schedule_333_e3822,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_333_e3824;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_335_e3851,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_335_e3847: f64 = (noise_variable_77 / noise_variable_18);
        let noise_metadata_schedule_335_e3848: f64 = (1.0 - noise_metadata_schedule_335_e3847);
        let noise_metadata_schedule_335_e3849: f64 = (noise_metadata_schedule_335_e3848).ln();
        (noise_metadata_schedule_335_e3849,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_335_e3851;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_338_e3908,) = {
    if (((noise_variable_260 == 0.0) && (noise_variable_271 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_338_e3898: f64 = (1.0 - params.p47);
        let noise_metadata_schedule_338_e3899: f64 = (noise_variable_78 * noise_metadata_schedule_338_e3898);
        let noise_metadata_schedule_338_e3900: f64 = (noise_metadata_schedule_338_e3899).exp();
        let noise_metadata_schedule_338_e3901: f64 = (1.0 - noise_metadata_schedule_338_e3900);
        let noise_metadata_schedule_338_e3902: f64 = (noise_variable_18 * noise_metadata_schedule_338_e3901);
        let noise_metadata_schedule_338_e3905: f64 = (1.0 - params.p47);
        let noise_metadata_schedule_338_e3906: f64 = (noise_metadata_schedule_338_e3902 / noise_metadata_schedule_338_e3905);
        (noise_metadata_schedule_338_e3906,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_338_e3908;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_342_e3951: f64 = if params.p44 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_276 = noise_metadata_schedule_342_e3951;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_343_e3954: f64 = if noise_variable_111 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_277 = noise_metadata_schedule_343_e3954;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_344_e3962,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_344_e3960: f64 = (params.p43 / 4.0);
        (noise_metadata_schedule_344_e3960,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_344_e3962;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_345_e3970,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_345_e3968: f64 = (params.p44 - noise_variable_17);
        (noise_metadata_schedule_345_e3968,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_345_e3970;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_346_e3985,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_346_e3977: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_346_e3978: f64 = (-noise_metadata_schedule_346_e3977);
        let noise_metadata_schedule_346_e3980: f64 = (noise_metadata_schedule_346_e3978 / params.p43);
        let noise_metadata_schedule_346_e3981: f64 = (noise_metadata_schedule_346_e3980).exp();
        let noise_metadata_schedule_346_e3982: f64 = (1.0 - noise_metadata_schedule_346_e3981);
        let noise_metadata_schedule_346_e3983: f64 = (noise_variable_17 * noise_metadata_schedule_346_e3982);
        (noise_metadata_schedule_346_e3983,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_346_e3985;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_347_e3993,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_347_e3991: f64 = (2.4 * noise_variable_111);
        (noise_metadata_schedule_347_e3991,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_347_e3993;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_348_e4009,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_348_e4000: f64 = (noise_variable_50 - params.p43);
        let noise_metadata_schedule_348_e4003: f64 = (params.p44 / noise_variable_17);
        let noise_metadata_schedule_348_e4004: f64 = (noise_metadata_schedule_348_e4003).ln();
        let noise_metadata_schedule_348_e4005: f64 = (noise_metadata_schedule_348_e4000 * noise_metadata_schedule_348_e4004);
        let noise_metadata_schedule_348_e4006: f64 = (noise_metadata_schedule_348_e4005).exp();
        let noise_metadata_schedule_348_e4007: f64 = (noise_variable_111 * noise_metadata_schedule_348_e4006);
        (noise_metadata_schedule_348_e4007,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_348_e4009;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_349_e4019,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_349_e4015: f64 = (noise_variable_52 - noise_variable_184);
        let noise_metadata_schedule_349_e4017: f64 = (noise_metadata_schedule_349_e4015 * noise_variable_3);
        (noise_metadata_schedule_349_e4017,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_349_e4019;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_350_e4022: f64 = if noise_variable_56 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_278 = noise_metadata_schedule_350_e4022;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_351_e4031,) = {
    if (((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) && (noise_variable_278 != 0.0)) {
        let noise_metadata_schedule_351_e4029: f64 = (noise_variable_56).exp();
        (noise_metadata_schedule_351_e4029,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_351_e4031;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_353_e4058,) = {
    if (((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) && (noise_variable_278 != 0.0)) {
        let noise_metadata_schedule_353_e4053: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_353_e4054: f64 = (noise_metadata_schedule_353_e4053).ln();
        let noise_metadata_schedule_353_e4055: f64 = (noise_variable_2 * noise_metadata_schedule_353_e4054);
        let noise_metadata_schedule_353_e4056: f64 = (noise_variable_52 - noise_metadata_schedule_353_e4055);
        (noise_metadata_schedule_353_e4056,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_353_e4058;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_355_e4076,) = {
    if (((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) && (noise_variable_278 == 0.0)) {
        (noise_variable_184,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_355_e4076;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_356_e4088,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_356_e4082: f64 = (0.1 * noise_variable_51);
        let noise_metadata_schedule_356_e4085: f64 = (4.0 * noise_variable_2);
        let noise_metadata_schedule_356_e4086: f64 = (noise_metadata_schedule_356_e4082 + noise_metadata_schedule_356_e4085);
        (noise_metadata_schedule_356_e4086,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_356_e4088;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_357_e4098,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_357_e4094: f64 = (noise_variable_51 + noise_variable_58);
        let noise_metadata_schedule_357_e4096: f64 = (noise_metadata_schedule_357_e4094 / noise_variable_55);
        (noise_metadata_schedule_357_e4096,)
    } else {
        (noise_variable_59,)
    }
};
            noise_variable_59 = noise_metadata_schedule_357_e4098;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_358_e4101: f64 = if noise_variable_59 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_279 = noise_metadata_schedule_358_e4101;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_359_e4110,) = {
    if (((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) && (noise_variable_279 != 0.0)) {
        let noise_metadata_schedule_359_e4108: f64 = (noise_variable_59).exp();
        (noise_metadata_schedule_359_e4108,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_359_e4110;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_361_e4146,) = {
    if (((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) && (noise_variable_279 != 0.0)) {
        let noise_metadata_schedule_361_e4129: f64 = (-noise_variable_51);
        let noise_metadata_schedule_361_e4133: f64 = (1.0 + noise_variable_57);
        let noise_metadata_schedule_361_e4134: f64 = (noise_metadata_schedule_361_e4133).ln();
        let noise_metadata_schedule_361_e4137: f64 = (noise_variable_51 + noise_variable_52);
        let noise_metadata_schedule_361_e4138: f64 = (-noise_metadata_schedule_361_e4137);
        let noise_metadata_schedule_361_e4140: f64 = (noise_metadata_schedule_361_e4138 / noise_variable_55);
        let noise_metadata_schedule_361_e4141: f64 = (noise_metadata_schedule_361_e4140).exp();
        let noise_metadata_schedule_361_e4142: f64 = (noise_metadata_schedule_361_e4134 - noise_metadata_schedule_361_e4141);
        let noise_metadata_schedule_361_e4143: f64 = (noise_variable_55 * noise_metadata_schedule_361_e4142);
        let noise_metadata_schedule_361_e4144: f64 = (noise_metadata_schedule_361_e4129 + noise_metadata_schedule_361_e4143);
        (noise_metadata_schedule_361_e4144,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_361_e4146;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_363_e4164,) = {
    if (((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) && (noise_variable_279 == 0.0)) {
        (noise_variable_58,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_363_e4164;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_364_e4172,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_364_e4170: f64 = (noise_variable_184 - noise_variable_58);
        (noise_metadata_schedule_364_e4170,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_364_e4172;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_365_e4183,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_365_e4179: f64 = (noise_variable_58 / noise_variable_17);
        let noise_metadata_schedule_365_e4180: f64 = (1.0 - noise_metadata_schedule_365_e4179);
        let noise_metadata_schedule_365_e4181: f64 = (noise_metadata_schedule_365_e4180).ln();
        (noise_metadata_schedule_365_e4181,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_365_e4183;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_366_e4194,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_366_e4190: f64 = (noise_variable_60 / noise_variable_17);
        let noise_metadata_schedule_366_e4191: f64 = (1.0 - noise_metadata_schedule_366_e4190);
        let noise_metadata_schedule_366_e4192: f64 = (noise_metadata_schedule_366_e4191).ln();
        (noise_metadata_schedule_366_e4192,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_366_e4194;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_367_e4202,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_367_e4200: f64 = (1.0 - params.p43);
        (noise_metadata_schedule_367_e4200,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_367_e4202;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_368_e4210,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_368_e4208: f64 = (1.0 - noise_variable_50);
        (noise_metadata_schedule_368_e4208,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_368_e4210;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_373_e4277,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_373_e4270: f64 = (noise_variable_66 * noise_variable_67);
        let noise_metadata_schedule_373_e4271: f64 = (noise_metadata_schedule_373_e4270).exp();
        let noise_metadata_schedule_373_e4272: f64 = (1.0 - noise_metadata_schedule_373_e4271);
        let noise_metadata_schedule_373_e4273: f64 = (noise_variable_111 * noise_metadata_schedule_373_e4272);
        let noise_metadata_schedule_373_e4275: f64 = (noise_metadata_schedule_373_e4273 / noise_variable_67);
        (noise_metadata_schedule_373_e4275,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_373_e4277;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_374_e4292,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_374_e4285: f64 = (noise_variable_65 * noise_variable_68);
        let noise_metadata_schedule_374_e4286: f64 = (noise_metadata_schedule_374_e4285).exp();
        let noise_metadata_schedule_374_e4287: f64 = (1.0 - noise_metadata_schedule_374_e4286);
        let noise_metadata_schedule_374_e4288: f64 = (noise_variable_54 * noise_metadata_schedule_374_e4287);
        let noise_metadata_schedule_374_e4290: f64 = (noise_metadata_schedule_374_e4288 / noise_variable_68);
        (noise_metadata_schedule_374_e4290,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_374_e4292;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_375_e4307,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_375_e4300: f64 = (noise_variable_66 * noise_variable_68);
        let noise_metadata_schedule_375_e4301: f64 = (noise_metadata_schedule_375_e4300).exp();
        let noise_metadata_schedule_375_e4302: f64 = (1.0 - noise_metadata_schedule_375_e4301);
        let noise_metadata_schedule_375_e4303: f64 = (noise_variable_54 * noise_metadata_schedule_375_e4302);
        let noise_metadata_schedule_375_e4305: f64 = (noise_metadata_schedule_375_e4303 / noise_variable_68);
        (noise_metadata_schedule_375_e4305,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_375_e4307;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_376_e4323,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_376_e4313: f64 = (noise_variable_62 + noise_variable_63);
        let noise_metadata_schedule_376_e4315: f64 = (noise_metadata_schedule_376_e4313 - noise_variable_64);
        let noise_metadata_schedule_376_e4317: f64 = (noise_metadata_schedule_376_e4315 * noise_variable_17);
        let noise_metadata_schedule_376_e4320: f64 = (noise_variable_53 * noise_variable_61);
        let noise_metadata_schedule_376_e4321: f64 = (noise_metadata_schedule_376_e4317 + noise_metadata_schedule_376_e4320);
        (noise_metadata_schedule_376_e4321,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_376_e4323;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_378_e4337,) = {
    if ((noise_variable_276 != 0.0) && (noise_variable_277 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_378_e4337;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_379_e4340: f64 = if noise_variable_111 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_280 = noise_metadata_schedule_379_e4340;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_380_e4356,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_380_e4348: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_380_e4349: f64 = (-noise_metadata_schedule_380_e4348);
        let noise_metadata_schedule_380_e4351: f64 = (noise_metadata_schedule_380_e4349 / params.p43);
        let noise_metadata_schedule_380_e4352: f64 = (noise_metadata_schedule_380_e4351).exp();
        let noise_metadata_schedule_380_e4353: f64 = (1.0 - noise_metadata_schedule_380_e4352);
        let noise_metadata_schedule_380_e4354: f64 = (noise_variable_17 * noise_metadata_schedule_380_e4353);
        (noise_metadata_schedule_380_e4354,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_380_e4356;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_381_e4367,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_381_e4363: f64 = (noise_variable_76 - noise_variable_184);
        let noise_metadata_schedule_381_e4365: f64 = (noise_metadata_schedule_381_e4363 * noise_variable_3);
        (noise_metadata_schedule_381_e4365,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_381_e4367;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_382_e4379,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_382_e4374: f64 = (noise_variable_80 * noise_variable_80);
        let noise_metadata_schedule_382_e4376: f64 = (noise_metadata_schedule_382_e4374 + 1.921812);
        let noise_metadata_schedule_382_e4377: f64 = (noise_metadata_schedule_382_e4376).sqrt();
        (noise_metadata_schedule_382_e4377,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_382_e4379;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_383_e4390,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_383_e4386: f64 = (noise_variable_80 + noise_variable_81);
        let noise_metadata_schedule_383_e4388: f64 = (noise_metadata_schedule_383_e4386 * 0.5);
        (noise_metadata_schedule_383_e4388,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_383_e4390;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_384_e4401,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_384_e4398: f64 = (noise_variable_2 * noise_variable_82);
        let noise_metadata_schedule_384_e4399: f64 = (noise_variable_76 - noise_metadata_schedule_384_e4398);
        (noise_metadata_schedule_384_e4399,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_384_e4401;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_386_e4422,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_386_e4418: f64 = (noise_variable_77 / noise_variable_17);
        let noise_metadata_schedule_386_e4419: f64 = (1.0 - noise_metadata_schedule_386_e4418);
        let noise_metadata_schedule_386_e4420: f64 = (noise_metadata_schedule_386_e4419).ln();
        (noise_metadata_schedule_386_e4420,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_386_e4422;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_389_e4470,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_389_e4460: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_389_e4461: f64 = (noise_variable_78 * noise_metadata_schedule_389_e4460);
        let noise_metadata_schedule_389_e4462: f64 = (noise_metadata_schedule_389_e4461).exp();
        let noise_metadata_schedule_389_e4463: f64 = (1.0 - noise_metadata_schedule_389_e4462);
        let noise_metadata_schedule_389_e4464: f64 = (noise_variable_17 * noise_metadata_schedule_389_e4463);
        let noise_metadata_schedule_389_e4467: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_389_e4468: f64 = (noise_metadata_schedule_389_e4464 / noise_metadata_schedule_389_e4467);
        (noise_metadata_schedule_389_e4468,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_389_e4470;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_390_e4485,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 != 0.0)) {
        let noise_metadata_schedule_390_e4480: f64 = (noise_variable_184 - noise_variable_77);
        let noise_metadata_schedule_390_e4481: f64 = (2.4 * noise_metadata_schedule_390_e4480);
        let noise_metadata_schedule_390_e4482: f64 = (noise_variable_79 + noise_metadata_schedule_390_e4481);
        let noise_metadata_schedule_390_e4483: f64 = (noise_variable_111 * noise_metadata_schedule_390_e4482);
        (noise_metadata_schedule_390_e4483,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_390_e4485;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_392_e4501,) = {
    if ((noise_variable_276 == 0.0) && (noise_variable_280 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_392_e4501;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_394_e4507: f64 = if noise_variable_111 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_281 = noise_metadata_schedule_394_e4507;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_395_e4520,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_395_e4512: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_395_e4513: f64 = (-noise_metadata_schedule_395_e4512);
        let noise_metadata_schedule_395_e4515: f64 = (noise_metadata_schedule_395_e4513 / params.p43);
        let noise_metadata_schedule_395_e4516: f64 = (noise_metadata_schedule_395_e4515).exp();
        let noise_metadata_schedule_395_e4517: f64 = (1.0 - noise_metadata_schedule_395_e4516);
        let noise_metadata_schedule_395_e4518: f64 = (noise_variable_17 * noise_metadata_schedule_395_e4517);
        (noise_metadata_schedule_395_e4518,)
    } else {
        (noise_variable_282,)
    }
};
            noise_variable_282 = noise_metadata_schedule_395_e4520;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_396_e4528,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_396_e4524: f64 = (noise_variable_282 - noise_variable_184);
        let noise_metadata_schedule_396_e4526: f64 = (noise_metadata_schedule_396_e4524 * noise_variable_3);
        (noise_metadata_schedule_396_e4526,)
    } else {
        (noise_variable_283,)
    }
};
            noise_variable_283 = noise_metadata_schedule_396_e4528;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_397_e4537,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_397_e4532: f64 = (noise_variable_283 * noise_variable_283);
        let noise_metadata_schedule_397_e4534: f64 = (noise_metadata_schedule_397_e4532 + 1.921812);
        let noise_metadata_schedule_397_e4535: f64 = (noise_metadata_schedule_397_e4534).sqrt();
        (noise_metadata_schedule_397_e4535,)
    } else {
        (noise_variable_284,)
    }
};
            noise_variable_284 = noise_metadata_schedule_397_e4537;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_398_e4545,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_398_e4541: f64 = (noise_variable_283 + noise_variable_284);
        let noise_metadata_schedule_398_e4543: f64 = (noise_metadata_schedule_398_e4541 * 0.5);
        (noise_metadata_schedule_398_e4543,)
    } else {
        (noise_variable_285,)
    }
};
            noise_variable_285 = noise_metadata_schedule_398_e4545;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_399_e4553,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_399_e4550: f64 = (noise_variable_2 * noise_variable_285);
        let noise_metadata_schedule_399_e4551: f64 = (noise_variable_282 - noise_metadata_schedule_399_e4550);
        (noise_metadata_schedule_399_e4551,)
    } else {
        (noise_variable_286,)
    }
};
            noise_variable_286 = noise_metadata_schedule_399_e4553;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_400_e4559,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_400_e4557: f64 = (noise_variable_285 / noise_variable_284);
        (noise_metadata_schedule_400_e4557,)
    } else {
        (noise_variable_287,)
    }
};
            noise_variable_287 = noise_metadata_schedule_400_e4559;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_401_e4584,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_401_e4563: f64 = (-params.p43);
        let noise_metadata_schedule_401_e4567: f64 = (noise_variable_286 / noise_variable_17);
        let noise_metadata_schedule_401_e4568: f64 = (1.0 - noise_metadata_schedule_401_e4567);
        let noise_metadata_schedule_401_e4569: f64 = (noise_metadata_schedule_401_e4568).ln();
        let noise_metadata_schedule_401_e4570: f64 = (noise_metadata_schedule_401_e4563 * noise_metadata_schedule_401_e4569);
        let noise_metadata_schedule_401_e4571: f64 = (noise_metadata_schedule_401_e4570).exp();
        let noise_metadata_schedule_401_e4572: f64 = (noise_variable_111 * noise_metadata_schedule_401_e4571);
        let noise_metadata_schedule_401_e4574: f64 = (noise_metadata_schedule_401_e4572 * noise_variable_287);
        let noise_metadata_schedule_401_e4577: f64 = (2.4 * noise_variable_111);
        let noise_metadata_schedule_401_e4580: f64 = (1.0 - noise_variable_287);
        let noise_metadata_schedule_401_e4581: f64 = (noise_metadata_schedule_401_e4577 * noise_metadata_schedule_401_e4580);
        let noise_metadata_schedule_401_e4582: f64 = (noise_metadata_schedule_401_e4574 + noise_metadata_schedule_401_e4581);
        (noise_metadata_schedule_401_e4582,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_401_e4584;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_402_e4589,) = {
    if (noise_variable_281 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_402_e4589;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_403_e4592: f64 = if params.p65 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_288 = noise_metadata_schedule_403_e4592;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_404_e4598,) = {
    if (noise_variable_288 != 0.0) {
        let noise_metadata_schedule_404_e4596: f64 = (noise_variable_38 - noise_variable_184);
        (noise_metadata_schedule_404_e4596,)
    } else {
        (noise_variable_143,)
    }
};
            noise_variable_143 = noise_metadata_schedule_404_e4598;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_405_e4605,) = {
    if (noise_variable_288 == 0.0) {
        let noise_metadata_schedule_405_e4603: f64 = (noise_variable_186 - noise_variable_34);
        (noise_metadata_schedule_405_e4603,)
    } else {
        (noise_variable_143,)
    }
};
            noise_variable_143 = noise_metadata_schedule_405_e4605;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_406_e4608: f64 = (noise_variable_143 * noise_variable_3);
            let noise_metadata_schedule_406_e4610: f64 = (noise_metadata_schedule_406_e4608 - 1.0);
            noise_variable_289 = noise_metadata_schedule_406_e4610;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_407_e4615: f64 = (noise_variable_289 * noise_variable_289);
            let noise_metadata_schedule_407_e4617: f64 = (noise_metadata_schedule_407_e4615 + 1.921812);
            let noise_metadata_schedule_407_e4618: f64 = (noise_metadata_schedule_407_e4617).sqrt();
            let noise_metadata_schedule_407_e4619: f64 = (noise_variable_289 + noise_metadata_schedule_407_e4618);
            let noise_metadata_schedule_407_e4621: f64 = (noise_metadata_schedule_407_e4619 / 2.0);
            let noise_metadata_schedule_407_e4622: f64 = (1.0 + noise_metadata_schedule_407_e4621);
            let noise_metadata_schedule_407_e4624: f64 = (noise_metadata_schedule_407_e4622 * noise_variable_2);
            noise_variable_290 = noise_metadata_schedule_407_e4624;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_408_e4627: f64 = (noise_variable_290 / noise_variable_33);
            noise_variable_291 = noise_metadata_schedule_408_e4627;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_409_e4630: f64 = (noise_variable_290 * noise_variable_32);
            noise_variable_292 = noise_metadata_schedule_409_e4630;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_410_e4634: f64 = (noise_variable_291).ln();
            let noise_metadata_schedule_410_e4635: f64 = (params.p67 * noise_metadata_schedule_410_e4634);
            let noise_metadata_schedule_410_e4636: f64 = (noise_metadata_schedule_410_e4635).exp();
            let noise_metadata_schedule_410_e4637: f64 = (1.0 + noise_metadata_schedule_410_e4636);
            let noise_metadata_schedule_410_e4638: f64 = (noise_metadata_schedule_410_e4637).ln();
            let noise_metadata_schedule_410_e4640: f64 = (noise_metadata_schedule_410_e4638 / params.p67);
            let noise_metadata_schedule_410_e4641: f64 = (noise_metadata_schedule_410_e4640).exp();
            noise_variable_293 = noise_metadata_schedule_410_e4641;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_411_e4644: f64 = (noise_variable_292 / noise_variable_293);
            noise_variable_294 = noise_metadata_schedule_411_e4644;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_412_e4647: f64 = (noise_variable_290 - noise_variable_33);
            let noise_metadata_schedule_412_e4649: f64 = (noise_metadata_schedule_412_e4647 / params.p63);
            noise_variable_295 = noise_metadata_schedule_412_e4649;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_413_e4656: f64 = (noise_variable_295 * noise_variable_295);
            let noise_metadata_schedule_413_e4658: f64 = (noise_metadata_schedule_413_e4656 + params.p66);
            let noise_metadata_schedule_413_e4659: f64 = (noise_metadata_schedule_413_e4658).sqrt();
            let noise_metadata_schedule_413_e4660: f64 = (noise_variable_295 + noise_metadata_schedule_413_e4659);
            let noise_metadata_schedule_413_e4661: f64 = (0.5 * noise_metadata_schedule_413_e4660);
            let noise_metadata_schedule_413_e4662: f64 = (1.0 + noise_metadata_schedule_413_e4661);
            let noise_metadata_schedule_413_e4663: f64 = (noise_variable_294 * noise_metadata_schedule_413_e4662);
            noise_variable_142 = noise_metadata_schedule_413_e4663;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_414_e4670: f64 = if ((noise_variable_107 > 0.0) && (noise_variable_111 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_296 = noise_metadata_schedule_414_e4670;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_415_e4676,) = {
    if (noise_variable_296 != 0.0) {
        let noise_metadata_schedule_415_e4674: f64 = (noise_variable_111 / noise_variable_107);
        (noise_metadata_schedule_415_e4674,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_415_e4676;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_416_e4682,) = {
    if (noise_variable_296 != 0.0) {
        let noise_metadata_schedule_416_e4680: f64 = (noise_variable_103 / noise_variable_111);
        (noise_metadata_schedule_416_e4680,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_416_e4682;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_417_e4687,) = {
    if (noise_variable_296 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_417_e4687;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_418_e4692,) = {
    if (noise_variable_296 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_418_e4692;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_419_e4695: f64 = if noise_variable_23 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_297 = noise_metadata_schedule_419_e4695;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_420_e4708,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_420_e4700: f64 = (noise_variable_43).ln();
        let noise_metadata_schedule_420_e4701: f64 = (-noise_metadata_schedule_420_e4700);
        let noise_metadata_schedule_420_e4703: f64 = (noise_metadata_schedule_420_e4701 / params.p36);
        let noise_metadata_schedule_420_e4704: f64 = (noise_metadata_schedule_420_e4703).exp();
        let noise_metadata_schedule_420_e4705: f64 = (1.0 - noise_metadata_schedule_420_e4704);
        let noise_metadata_schedule_420_e4706: f64 = (noise_variable_16 * noise_metadata_schedule_420_e4705);
        (noise_metadata_schedule_420_e4706,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_420_e4708;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_421_e4716,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_421_e4712: f64 = (noise_variable_76 - noise_variable_185);
        let noise_metadata_schedule_421_e4714: f64 = (noise_metadata_schedule_421_e4712 * noise_variable_3);
        (noise_metadata_schedule_421_e4714,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_421_e4716;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_422_e4725,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_422_e4720: f64 = (noise_variable_80 * noise_variable_80);
        let noise_metadata_schedule_422_e4722: f64 = (noise_metadata_schedule_422_e4720 + 1.921812);
        let noise_metadata_schedule_422_e4723: f64 = (noise_metadata_schedule_422_e4722).sqrt();
        (noise_metadata_schedule_422_e4723,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_422_e4725;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_423_e4733,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_423_e4729: f64 = (noise_variable_80 + noise_variable_81);
        let noise_metadata_schedule_423_e4731: f64 = (noise_metadata_schedule_423_e4729 * 0.5);
        (noise_metadata_schedule_423_e4731,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_423_e4733;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_424_e4741,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_424_e4738: f64 = (noise_variable_2 * noise_variable_82);
        let noise_metadata_schedule_424_e4739: f64 = (noise_variable_76 - noise_metadata_schedule_424_e4738);
        (noise_metadata_schedule_424_e4739,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_424_e4741;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_426_e4756,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_426_e4752: f64 = (noise_variable_77 / noise_variable_16);
        let noise_metadata_schedule_426_e4753: f64 = (1.0 - noise_metadata_schedule_426_e4752);
        let noise_metadata_schedule_426_e4754: f64 = (noise_metadata_schedule_426_e4753).ln();
        (noise_metadata_schedule_426_e4754,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_426_e4756;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_429_e4795,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_429_e4785: f64 = (1.0 - params.p36);
        let noise_metadata_schedule_429_e4786: f64 = (noise_variable_78 * noise_metadata_schedule_429_e4785);
        let noise_metadata_schedule_429_e4787: f64 = (noise_metadata_schedule_429_e4786).exp();
        let noise_metadata_schedule_429_e4788: f64 = (1.0 - noise_metadata_schedule_429_e4787);
        let noise_metadata_schedule_429_e4789: f64 = (noise_variable_16 * noise_metadata_schedule_429_e4788);
        let noise_metadata_schedule_429_e4792: f64 = (1.0 - params.p36);
        let noise_metadata_schedule_429_e4793: f64 = (noise_metadata_schedule_429_e4789 / noise_metadata_schedule_429_e4792);
        (noise_metadata_schedule_429_e4793,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_429_e4795;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_430_e4807,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_430_e4802: f64 = (noise_variable_185 - noise_variable_77);
        let noise_metadata_schedule_430_e4803: f64 = (noise_variable_43 * noise_metadata_schedule_430_e4802);
        let noise_metadata_schedule_430_e4804: f64 = (noise_variable_79 + noise_metadata_schedule_430_e4803);
        let noise_metadata_schedule_430_e4805: f64 = (noise_variable_23 * noise_metadata_schedule_430_e4804);
        (noise_metadata_schedule_430_e4805,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_430_e4807;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_432_e4817,) = {
    if (noise_variable_297 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_432_e4817;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_433_e4820: f64 = (noise_variable_98 / noise_variable_23);
            noise_variable_102 = noise_metadata_schedule_433_e4820;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_434_e4823: f64 = if params.p0 <= 200.0 { 1.0 } else { 0.0 };
            noise_variable_298 = noise_metadata_schedule_434_e4823;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_435_e4826: f64 = if noise_variable_26 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_299 = noise_metadata_schedule_435_e4826;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_436_e4841,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_436_e4833: f64 = (noise_variable_44).ln();
        let noise_metadata_schedule_436_e4834: f64 = (-noise_metadata_schedule_436_e4833);
        let noise_metadata_schedule_436_e4836: f64 = (noise_metadata_schedule_436_e4834 / params.p39);
        let noise_metadata_schedule_436_e4837: f64 = (noise_metadata_schedule_436_e4836).exp();
        let noise_metadata_schedule_436_e4838: f64 = (1.0 - noise_metadata_schedule_436_e4837);
        let noise_metadata_schedule_436_e4839: f64 = (noise_variable_22 * noise_metadata_schedule_436_e4838);
        (noise_metadata_schedule_436_e4839,)
    } else {
        (noise_variable_76,)
    }
};
            noise_variable_76 = noise_metadata_schedule_436_e4841;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_437_e4851,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_437_e4847: f64 = (noise_variable_76 - noise_variable_185);
        let noise_metadata_schedule_437_e4849: f64 = (noise_metadata_schedule_437_e4847 * noise_variable_3);
        (noise_metadata_schedule_437_e4849,)
    } else {
        (noise_variable_80,)
    }
};
            noise_variable_80 = noise_metadata_schedule_437_e4851;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_438_e4862,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_438_e4857: f64 = (noise_variable_80 * noise_variable_80);
        let noise_metadata_schedule_438_e4859: f64 = (noise_metadata_schedule_438_e4857 + 1.921812);
        let noise_metadata_schedule_438_e4860: f64 = (noise_metadata_schedule_438_e4859).sqrt();
        (noise_metadata_schedule_438_e4860,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_438_e4862;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_439_e4872,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_439_e4868: f64 = (noise_variable_80 + noise_variable_81);
        let noise_metadata_schedule_439_e4870: f64 = (noise_metadata_schedule_439_e4868 * 0.5);
        (noise_metadata_schedule_439_e4870,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_439_e4872;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_440_e4882,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_440_e4879: f64 = (noise_variable_2 * noise_variable_82);
        let noise_metadata_schedule_440_e4880: f64 = (noise_variable_76 - noise_metadata_schedule_440_e4879);
        (noise_metadata_schedule_440_e4880,)
    } else {
        (noise_variable_77,)
    }
};
            noise_variable_77 = noise_metadata_schedule_440_e4882;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_442_e4901,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_442_e4897: f64 = (noise_variable_77 / noise_variable_22);
        let noise_metadata_schedule_442_e4898: f64 = (1.0 - noise_metadata_schedule_442_e4897);
        let noise_metadata_schedule_442_e4899: f64 = (noise_metadata_schedule_442_e4898).ln();
        (noise_metadata_schedule_442_e4899,)
    } else {
        (noise_variable_78,)
    }
};
            noise_variable_78 = noise_metadata_schedule_442_e4901;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_445_e4946,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_445_e4936: f64 = (1.0 - params.p39);
        let noise_metadata_schedule_445_e4937: f64 = (noise_variable_78 * noise_metadata_schedule_445_e4936);
        let noise_metadata_schedule_445_e4938: f64 = (noise_metadata_schedule_445_e4937).exp();
        let noise_metadata_schedule_445_e4939: f64 = (1.0 - noise_metadata_schedule_445_e4938);
        let noise_metadata_schedule_445_e4940: f64 = (noise_variable_22 * noise_metadata_schedule_445_e4939);
        let noise_metadata_schedule_445_e4943: f64 = (1.0 - params.p39);
        let noise_metadata_schedule_445_e4944: f64 = (noise_metadata_schedule_445_e4940 / noise_metadata_schedule_445_e4943);
        (noise_metadata_schedule_445_e4944,)
    } else {
        (noise_variable_79,)
    }
};
            noise_variable_79 = noise_metadata_schedule_445_e4946;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_446_e4960,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 != 0.0)) {
        let noise_metadata_schedule_446_e4955: f64 = (noise_variable_185 - noise_variable_77);
        let noise_metadata_schedule_446_e4956: f64 = (noise_variable_44 * noise_metadata_schedule_446_e4955);
        let noise_metadata_schedule_446_e4957: f64 = (noise_variable_79 + noise_metadata_schedule_446_e4956);
        let noise_metadata_schedule_446_e4958: f64 = (noise_variable_26 * noise_metadata_schedule_446_e4957);
        (noise_metadata_schedule_446_e4958,)
    } else {
        (noise_variable_100,)
    }
};
            noise_variable_100 = noise_metadata_schedule_446_e4960;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_448_e4974,) = {
    if ((noise_variable_298 != 0.0) && (noise_variable_299 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_100,)
    }
};
            noise_variable_100 = noise_metadata_schedule_448_e4974;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_449_e4980,) = {
    if (noise_variable_298 != 0.0) {
        let noise_metadata_schedule_449_e4978: f64 = (noise_variable_100 / noise_variable_26);
        (noise_metadata_schedule_449_e4978,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_449_e4980;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_450_e4984,) = {
    if (noise_variable_298 != 0.0) {
        (noise_variable_22,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_450_e4984;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_451_e4988,) = {
    if (noise_variable_298 != 0.0) {
        (params.p39,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_451_e4988;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_452_e4993,) = {
    if (noise_variable_298 == 0.0) {
        (noise_variable_102,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_452_e4993;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_453_e4998,) = {
    if (noise_variable_298 == 0.0) {
        (noise_variable_16,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_453_e4998;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_454_e5003,) = {
    if (noise_variable_298 == 0.0) {
        (params.p36,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_454_e5003;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_455_e5006: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_300 = noise_metadata_schedule_455_e5006;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_456_e5010,) = {
    if (noise_variable_300 != 0.0) {
        (1.0,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_456_e5010;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_457_e5017,) = {
    if (noise_variable_300 == 0.0) {
        let noise_metadata_schedule_457_e5015: f64 = (params.p8 * noise_variable_2);
        (noise_metadata_schedule_457_e5015,)
    } else {
        (noise_variable_301,)
    }
};
            noise_variable_301 = noise_metadata_schedule_457_e5017;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_458_e5026,) = {
    if (noise_variable_300 == 0.0) {
        let noise_metadata_schedule_458_e5022: f64 = (noise_variable_20 - noise_variable_185);
        let noise_metadata_schedule_458_e5024: f64 = (noise_metadata_schedule_458_e5022 / noise_variable_301);
        (noise_metadata_schedule_458_e5024,)
    } else {
        (noise_variable_302,)
    }
};
            noise_variable_302 = noise_metadata_schedule_458_e5026;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_459_e5044,) = {
    if (noise_variable_300 == 0.0) {
        let noise_metadata_schedule_459_e5034: f64 = (noise_variable_302 * noise_variable_302);
        let noise_metadata_schedule_459_e5036: f64 = (noise_metadata_schedule_459_e5034 + 1.921812);
        let noise_metadata_schedule_459_e5037: f64 = (noise_metadata_schedule_459_e5036).sqrt();
        let noise_metadata_schedule_459_e5038: f64 = (noise_variable_302 + noise_metadata_schedule_459_e5037);
        let noise_metadata_schedule_459_e5039: f64 = (noise_variable_301 * noise_metadata_schedule_459_e5038);
        let noise_metadata_schedule_459_e5041: f64 = (noise_metadata_schedule_459_e5039 * 0.5);
        let noise_metadata_schedule_459_e5042: f64 = (noise_variable_20 - noise_metadata_schedule_459_e5041);
        (noise_metadata_schedule_459_e5042,)
    } else {
        (noise_variable_303,)
    }
};
            noise_variable_303 = noise_metadata_schedule_459_e5044;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_460_e5061,) = {
    if (noise_variable_300 == 0.0) {
        let noise_metadata_schedule_460_e5053: f64 = (noise_variable_303 / noise_variable_20);
        let noise_metadata_schedule_460_e5054: f64 = (1.0 - noise_metadata_schedule_460_e5053);
        let noise_metadata_schedule_460_e5055: f64 = (noise_metadata_schedule_460_e5054).ln();
        let noise_metadata_schedule_460_e5056: f64 = (noise_variable_21 * noise_metadata_schedule_460_e5055);
        let noise_metadata_schedule_460_e5057: f64 = (noise_metadata_schedule_460_e5056).exp();
        let noise_metadata_schedule_460_e5058: f64 = (1.0 - noise_metadata_schedule_460_e5057);
        let noise_metadata_schedule_460_e5059: f64 = (noise_variable_200 * noise_metadata_schedule_460_e5058);
        (noise_metadata_schedule_460_e5059,)
    } else {
        (noise_variable_304,)
    }
};
            noise_variable_304 = noise_metadata_schedule_460_e5061;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_461_e5063: f64 = (noise_variable_304).abs();
            let noise_metadata_schedule_461_e5065: f64 = if noise_metadata_schedule_461_e5063 >= 0.001 { 1.0 } else { 0.0 };
            noise_variable_305 = noise_metadata_schedule_461_e5065;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_462_e5077,) = {
    if ((noise_variable_300 == 0.0) && (noise_variable_305 != 0.0)) {
        let noise_metadata_schedule_462_e5071: f64 = (noise_variable_304).exp();
        let noise_metadata_schedule_462_e5073: f64 = (noise_metadata_schedule_462_e5071 - 1.0);
        let noise_metadata_schedule_462_e5075: f64 = (noise_metadata_schedule_462_e5073 / noise_variable_304);
        (noise_metadata_schedule_462_e5075,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_462_e5077;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_463_e5089,) = {
    if ((noise_variable_300 == 0.0) && (noise_variable_305 == 0.0)) {
        let noise_metadata_schedule_463_e5086: f64 = (noise_variable_304 * 0.5);
        let noise_metadata_schedule_463_e5087: f64 = (1.0 + noise_metadata_schedule_463_e5086);
        (noise_metadata_schedule_463_e5087,)
    } else {
        (noise_variable_201,)
    }
};
            noise_variable_201 = noise_metadata_schedule_463_e5089;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_464_e5092: f64 = (noise_variable_201 * noise_variable_101);
            noise_variable_159 = noise_metadata_schedule_464_e5092;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_465_e5096: f64 = (noise_variable_159 / noise_variable_202);
            let noise_metadata_schedule_465_e5097: f64 = (1.0 + noise_metadata_schedule_465_e5096);
            let noise_metadata_schedule_465_e5100: f64 = (noise_variable_103 / params.p5);
            let noise_metadata_schedule_465_e5101: f64 = (noise_metadata_schedule_465_e5097 + noise_metadata_schedule_465_e5100);
            noise_variable_116 = noise_metadata_schedule_465_e5101;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_466_e5104: f64 = (20.0 * noise_variable_116);
            let noise_metadata_schedule_466_e5106: f64 = (noise_metadata_schedule_466_e5104 - 1.0);
            noise_variable_131 = noise_metadata_schedule_466_e5106;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_467_e5112: f64 = (noise_variable_131 * noise_variable_131);
            let noise_metadata_schedule_467_e5114: f64 = (noise_metadata_schedule_467_e5112 + 1.921812);
            let noise_metadata_schedule_467_e5115: f64 = (noise_metadata_schedule_467_e5114).sqrt();
            let noise_metadata_schedule_467_e5116: f64 = (noise_variable_131 + noise_metadata_schedule_467_e5115);
            let noise_metadata_schedule_467_e5118: f64 = (noise_metadata_schedule_467_e5116 / 2.0);
            let noise_metadata_schedule_467_e5119: f64 = (1.0 + noise_metadata_schedule_467_e5118);
            let noise_metadata_schedule_467_e5120: f64 = (0.025 * noise_metadata_schedule_467_e5119);
            noise_variable_115 = noise_metadata_schedule_467_e5120;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_468_e5125: f64 = (noise_variable_114 - 1.0);
            let noise_metadata_schedule_468_e5126: f64 = (params.p55 * noise_metadata_schedule_468_e5125);
            let noise_metadata_schedule_468_e5127: f64 = (noise_variable_42 + noise_metadata_schedule_468_e5126);
            let noise_metadata_schedule_468_e5131: f64 = (1.0 / noise_variable_114);
            let noise_metadata_schedule_468_e5133: f64 = (noise_metadata_schedule_468_e5131 - 1.0);
            let noise_metadata_schedule_468_e5134: f64 = (params.p56 * noise_metadata_schedule_468_e5133);
            let noise_metadata_schedule_468_e5135: f64 = (noise_metadata_schedule_468_e5127 + noise_metadata_schedule_468_e5134);
            noise_variable_117 = noise_metadata_schedule_468_e5135;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_469_e5138: f64 = if params.p10 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_306 = noise_metadata_schedule_469_e5138;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_470_e5146,) = {
    if (noise_variable_306 != 0.0) {
        let noise_metadata_schedule_470_e5142: f64 = (noise_variable_117 / noise_variable_42);
        let noise_metadata_schedule_470_e5144: f64 = (noise_metadata_schedule_470_e5142 - 1.0);
        (noise_metadata_schedule_470_e5144,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_470_e5146;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_471_e5154,) = {
    if (noise_variable_306 != 0.0) {
        let noise_metadata_schedule_471_e5151: f64 = (1.0 + noise_variable_130);
        let noise_metadata_schedule_471_e5152: f64 = (noise_variable_15 / noise_metadata_schedule_471_e5151);
        (noise_metadata_schedule_471_e5152,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_471_e5154;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_472_e5159,) = {
    if (noise_variable_306 == 0.0) {
        (noise_variable_15,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_472_e5159;
        }
        if matches!(source_index, 0 | 5) {
            noise_variable_119 = params.p11;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_474_e5164: f64 = (params.p3 * noise_variable_2);
            let noise_metadata_schedule_474_e5165: f64 = (noise_variable_185 / noise_metadata_schedule_474_e5164);
            noise_variable_180 = noise_metadata_schedule_474_e5165;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_475_e5168: f64 = if noise_variable_180 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_307 = noise_metadata_schedule_475_e5168;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_476_e5176,) = {
    if (noise_variable_307 != 0.0) {
        let noise_metadata_schedule_476_e5173: f64 = (noise_variable_180 - 80.0);
        let noise_metadata_schedule_476_e5174: f64 = (1.0 + noise_metadata_schedule_476_e5173);
        (noise_metadata_schedule_476_e5174,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_476_e5176;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_477_e5180,) = {
    if (noise_variable_307 != 0.0) {
        (80.0,)
    } else {
        (noise_variable_180,)
    }
};
            noise_variable_180 = noise_metadata_schedule_477_e5180;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_478_e5185,) = {
    if (noise_variable_307 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_478_e5185;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_479_e5188: f64 = { let limexp_arg = noise_variable_180; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_479_e5189: f64 = (noise_variable_179 * noise_metadata_schedule_479_e5188);
            noise_variable_179 = noise_metadata_schedule_479_e5189;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_480_e5192: f64 = (noise_variable_11 * noise_variable_179);
            noise_variable_120 = noise_metadata_schedule_480_e5192;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_481_e5196: f64 = (params.p4 * noise_variable_2);
            let noise_metadata_schedule_481_e5197: f64 = (noise_variable_184 / noise_metadata_schedule_481_e5196);
            noise_variable_182 = noise_metadata_schedule_481_e5197;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_482_e5200: f64 = if noise_variable_182 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_308 = noise_metadata_schedule_482_e5200;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_483_e5208,) = {
    if (noise_variable_308 != 0.0) {
        let noise_metadata_schedule_483_e5205: f64 = (noise_variable_182 - 80.0);
        let noise_metadata_schedule_483_e5206: f64 = (1.0 + noise_metadata_schedule_483_e5205);
        (noise_metadata_schedule_483_e5206,)
    } else {
        (noise_variable_181,)
    }
};
            noise_variable_181 = noise_metadata_schedule_483_e5208;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_484_e5212,) = {
    if (noise_variable_308 != 0.0) {
        (80.0,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_484_e5212;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_485_e5217,) = {
    if (noise_variable_308 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_181,)
    }
};
            noise_variable_181 = noise_metadata_schedule_485_e5217;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_486_e5220: f64 = { let limexp_arg = noise_variable_182; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_486_e5221: f64 = (noise_variable_181 * noise_metadata_schedule_486_e5220);
            noise_variable_181 = noise_metadata_schedule_486_e5221;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_487_e5224: f64 = (noise_variable_11 * noise_variable_181);
            noise_variable_121 = noise_metadata_schedule_487_e5224;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_488_e5227: f64 = if params.p13 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_309 = noise_metadata_schedule_488_e5227;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_489_e5251,) = {
    if (noise_variable_309 != 0.0) {
        let noise_metadata_schedule_489_e5231: f64 = (noise_variable_120 / noise_variable_118);
        let noise_metadata_schedule_489_e5234: f64 = (noise_variable_121 / noise_variable_119);
        let noise_metadata_schedule_489_e5235: f64 = (noise_metadata_schedule_489_e5231 + noise_metadata_schedule_489_e5234);
        let noise_metadata_schedule_489_e5240: f64 = (noise_variable_120 / noise_variable_142);
        let noise_metadata_schedule_489_e5241: f64 = (noise_variable_120 * noise_metadata_schedule_489_e5240);
        let noise_metadata_schedule_489_e5244: f64 = (noise_variable_205 / noise_variable_203);
        let noise_metadata_schedule_489_e5245: f64 = (noise_metadata_schedule_489_e5241 * noise_metadata_schedule_489_e5244);
        let noise_metadata_schedule_489_e5246: f64 = (noise_metadata_schedule_489_e5245).ln();
        let noise_metadata_schedule_489_e5247: f64 = (0.6666 * noise_metadata_schedule_489_e5246);
        let noise_metadata_schedule_489_e5248: f64 = (noise_metadata_schedule_489_e5247).exp();
        let noise_metadata_schedule_489_e5249: f64 = (noise_metadata_schedule_489_e5235 + noise_metadata_schedule_489_e5248);
        (noise_metadata_schedule_489_e5249,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_489_e5251;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_490_e5279,) = {
    if (noise_variable_309 != 0.0) {
        let noise_metadata_schedule_490_e5255: f64 = (noise_variable_120 / noise_variable_118);
        let noise_metadata_schedule_490_e5258: f64 = (noise_variable_121 / noise_variable_119);
        let noise_metadata_schedule_490_e5259: f64 = (noise_metadata_schedule_490_e5255 + noise_metadata_schedule_490_e5258);
        let noise_metadata_schedule_490_e5262: f64 = (noise_variable_120 / noise_variable_203);
        let noise_metadata_schedule_490_e5263: f64 = (noise_metadata_schedule_490_e5259 + noise_metadata_schedule_490_e5262);
        let noise_metadata_schedule_490_e5268: f64 = (noise_variable_120 / noise_variable_142);
        let noise_metadata_schedule_490_e5269: f64 = (noise_variable_120 * noise_metadata_schedule_490_e5268);
        let noise_metadata_schedule_490_e5272: f64 = (noise_variable_205 / noise_variable_203);
        let noise_metadata_schedule_490_e5273: f64 = (noise_metadata_schedule_490_e5269 * noise_metadata_schedule_490_e5272);
        let noise_metadata_schedule_490_e5274: f64 = (noise_metadata_schedule_490_e5273).ln();
        let noise_metadata_schedule_490_e5275: f64 = (0.6666 * noise_metadata_schedule_490_e5274);
        let noise_metadata_schedule_490_e5276: f64 = (noise_metadata_schedule_490_e5275).exp();
        let noise_metadata_schedule_490_e5277: f64 = (noise_metadata_schedule_490_e5263 + noise_metadata_schedule_490_e5276);
        (noise_metadata_schedule_490_e5277,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_490_e5279;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_491_e5290,) = {
    if (noise_variable_309 == 0.0) {
        let noise_metadata_schedule_491_e5284: f64 = (noise_variable_120 / noise_variable_118);
        let noise_metadata_schedule_491_e5287: f64 = (noise_variable_121 / noise_variable_119);
        let noise_metadata_schedule_491_e5288: f64 = (noise_metadata_schedule_491_e5284 + noise_metadata_schedule_491_e5287);
        (noise_metadata_schedule_491_e5288,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_491_e5290;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_492_e5305,) = {
    if (noise_variable_309 == 0.0) {
        let noise_metadata_schedule_492_e5295: f64 = (noise_variable_120 / noise_variable_118);
        let noise_metadata_schedule_492_e5298: f64 = (noise_variable_121 / noise_variable_119);
        let noise_metadata_schedule_492_e5299: f64 = (noise_metadata_schedule_492_e5295 + noise_metadata_schedule_492_e5298);
        let noise_metadata_schedule_492_e5302: f64 = (noise_variable_120 / noise_variable_203);
        let noise_metadata_schedule_492_e5303: f64 = (noise_metadata_schedule_492_e5299 + noise_metadata_schedule_492_e5302);
        (noise_metadata_schedule_492_e5303,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_492_e5305;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_493_e5309: f64 = (noise_variable_115 * noise_variable_115);
            let noise_metadata_schedule_493_e5311: f64 = (noise_metadata_schedule_493_e5309 + noise_variable_123);
            let noise_metadata_schedule_493_e5312: f64 = (noise_metadata_schedule_493_e5311).sqrt();
            let noise_metadata_schedule_493_e5313: f64 = (noise_variable_115 + noise_metadata_schedule_493_e5312);
            noise_variable_128 = noise_metadata_schedule_493_e5313;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_494_e5317: f64 = (noise_variable_115 * noise_variable_115);
            let noise_metadata_schedule_494_e5319: f64 = (noise_metadata_schedule_494_e5317 + noise_variable_124);
            let noise_metadata_schedule_494_e5320: f64 = (noise_metadata_schedule_494_e5319).sqrt();
            let noise_metadata_schedule_494_e5321: f64 = (noise_variable_115 + noise_metadata_schedule_494_e5320);
            noise_variable_129 = noise_metadata_schedule_494_e5321;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_495_e5324: f64 = (noise_variable_124 - noise_variable_123);
            noise_variable_207 = noise_metadata_schedule_495_e5324;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_496_e5326: f64 = (noise_variable_207).abs();
            let noise_metadata_schedule_496_e5328: f64 = if noise_metadata_schedule_496_e5326 > 1e-8 { 1.0 } else { 0.0 };
            noise_variable_310 = noise_metadata_schedule_496_e5328;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_497_e5342,) = {
    if (noise_variable_310 != 0.0) {
        let noise_metadata_schedule_497_e5334: f64 = (1.0 + noise_variable_206);
        let noise_metadata_schedule_497_e5335: f64 = (noise_variable_142 / noise_metadata_schedule_497_e5334);
        let noise_metadata_schedule_497_e5337: f64 = (noise_metadata_schedule_497_e5335 / noise_variable_120);
        let noise_metadata_schedule_497_e5339: f64 = (noise_metadata_schedule_497_e5337 * noise_variable_128);
        let noise_metadata_schedule_497_e5340: f64 = (1.0 - noise_metadata_schedule_497_e5339);
        (noise_metadata_schedule_497_e5340,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_497_e5342;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_498_e5358,) = {
    if (noise_variable_310 != 0.0) {
        let noise_metadata_schedule_498_e5348: f64 = (1.0 + noise_variable_206);
        let noise_metadata_schedule_498_e5349: f64 = (noise_variable_142 / noise_metadata_schedule_498_e5348);
        let noise_metadata_schedule_498_e5351: f64 = (noise_metadata_schedule_498_e5349 / noise_variable_120);
        let noise_metadata_schedule_498_e5354: f64 = (noise_variable_129 - noise_variable_128);
        let noise_metadata_schedule_498_e5355: f64 = (noise_metadata_schedule_498_e5351 * noise_metadata_schedule_498_e5354);
        let noise_metadata_schedule_498_e5356: f64 = (1.0 + noise_metadata_schedule_498_e5355);
        (noise_metadata_schedule_498_e5356,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_498_e5358;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_499_e5364,) = {
    if (noise_variable_310 != 0.0) {
        let noise_metadata_schedule_499_e5362: f64 = (noise_variable_150 / noise_variable_151);
        (noise_metadata_schedule_499_e5362,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_499_e5364;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_500_e5382,) = {
    if (noise_variable_310 != 0.0) {
        let noise_metadata_schedule_500_e5368: f64 = (noise_variable_149 * noise_variable_149);
        let noise_metadata_schedule_500_e5370: f64 = (noise_metadata_schedule_500_e5368 + 0.01);
        let noise_metadata_schedule_500_e5371: f64 = (noise_metadata_schedule_500_e5370).sqrt();
        let noise_metadata_schedule_500_e5373: f64 = (noise_metadata_schedule_500_e5371 + noise_variable_149);
        let noise_metadata_schedule_500_e5377: f64 = (1.0 + 0.01);
        let noise_metadata_schedule_500_e5378: f64 = (noise_metadata_schedule_500_e5377).sqrt();
        let noise_metadata_schedule_500_e5379: f64 = (1.0 + noise_metadata_schedule_500_e5378);
        let noise_metadata_schedule_500_e5380: f64 = (noise_metadata_schedule_500_e5373 / noise_metadata_schedule_500_e5379);
        (noise_metadata_schedule_500_e5380,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_500_e5382;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_501_e5387,) = {
    if (noise_variable_310 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_501_e5387;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_502_e5390: f64 = if params.p2 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_311 = noise_metadata_schedule_502_e5390;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_503_e5393: f64 = if params.p13 != 0.0 { 1.0 } else { 0.0 };
            noise_variable_312 = noise_metadata_schedule_503_e5393;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_504_e5427,) = {
    if ((noise_variable_311 != 0.0) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_504_e5399: f64 = (noise_variable_120 / noise_variable_118);
        let noise_metadata_schedule_504_e5402: f64 = (noise_variable_121 / noise_variable_119);
        let noise_metadata_schedule_504_e5403: f64 = (noise_metadata_schedule_504_e5399 + noise_metadata_schedule_504_e5402);
        let noise_metadata_schedule_504_e5406: f64 = (noise_variable_120 / noise_variable_203);
        let noise_metadata_schedule_504_e5408: f64 = (noise_metadata_schedule_504_e5406 * noise_variable_146);
        let noise_metadata_schedule_504_e5410: f64 = (noise_metadata_schedule_504_e5408 * noise_variable_146);
        let noise_metadata_schedule_504_e5411: f64 = (noise_metadata_schedule_504_e5403 + noise_metadata_schedule_504_e5410);
        let noise_metadata_schedule_504_e5416: f64 = (noise_variable_120 / noise_variable_142);
        let noise_metadata_schedule_504_e5417: f64 = (noise_variable_120 * noise_metadata_schedule_504_e5416);
        let noise_metadata_schedule_504_e5420: f64 = (noise_variable_205 / noise_variable_203);
        let noise_metadata_schedule_504_e5421: f64 = (noise_metadata_schedule_504_e5417 * noise_metadata_schedule_504_e5420);
        let noise_metadata_schedule_504_e5422: f64 = (noise_metadata_schedule_504_e5421).ln();
        let noise_metadata_schedule_504_e5423: f64 = (0.6666 * noise_metadata_schedule_504_e5422);
        let noise_metadata_schedule_504_e5424: f64 = (noise_metadata_schedule_504_e5423).exp();
        let noise_metadata_schedule_504_e5425: f64 = (noise_metadata_schedule_504_e5411 + noise_metadata_schedule_504_e5424);
        (noise_metadata_schedule_504_e5425,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_504_e5427;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_505_e5448,) = {
    if ((noise_variable_311 != 0.0) && (noise_variable_312 == 0.0)) {
        let noise_metadata_schedule_505_e5434: f64 = (noise_variable_120 / noise_variable_118);
        let noise_metadata_schedule_505_e5437: f64 = (noise_variable_121 / noise_variable_119);
        let noise_metadata_schedule_505_e5438: f64 = (noise_metadata_schedule_505_e5434 + noise_metadata_schedule_505_e5437);
        let noise_metadata_schedule_505_e5441: f64 = (noise_variable_120 / noise_variable_203);
        let noise_metadata_schedule_505_e5443: f64 = (noise_metadata_schedule_505_e5441 * noise_variable_146);
        let noise_metadata_schedule_505_e5445: f64 = (noise_metadata_schedule_505_e5443 * noise_variable_146);
        let noise_metadata_schedule_505_e5446: f64 = (noise_metadata_schedule_505_e5438 + noise_metadata_schedule_505_e5445);
        (noise_metadata_schedule_505_e5446,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_505_e5448;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_506_e5459,) = {
    if (noise_variable_311 != 0.0) {
        let noise_metadata_schedule_506_e5453: f64 = (noise_variable_115 * noise_variable_115);
        let noise_metadata_schedule_506_e5455: f64 = (noise_metadata_schedule_506_e5453 + noise_variable_122);
        let noise_metadata_schedule_506_e5456: f64 = (noise_metadata_schedule_506_e5455).sqrt();
        let noise_metadata_schedule_506_e5457: f64 = (noise_variable_115 + noise_metadata_schedule_506_e5456);
        (noise_metadata_schedule_506_e5457,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_506_e5459;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_507_e5466,) = {
    if (noise_variable_311 == 0.0) {
        let noise_metadata_schedule_507_e5464: f64 = (1.0 / 3.0);
        (noise_metadata_schedule_507_e5464,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_507_e5466;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_508_e5474,) = {
    if (noise_variable_311 == 0.0) {
        let noise_metadata_schedule_508_e5470: f64 = (-2.0);
        let noise_metadata_schedule_508_e5472: f64 = (noise_metadata_schedule_508_e5470 * noise_variable_115);
        (noise_metadata_schedule_508_e5472,)
    } else {
        (noise_variable_84,)
    }
};
            noise_variable_84 = noise_metadata_schedule_508_e5474;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_509_e5481: f64 = if ((params.p9 == 1000000.0) && (params.p12 == 1000000.0)) { 1.0 } else { 0.0 };
            noise_variable_313 = noise_metadata_schedule_509_e5481;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_510_e5488,) = {
    if ((noise_variable_311 == 0.0) && (noise_variable_313 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_510_e5488;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_511_e5511,) = {
    if ((noise_variable_311 == 0.0) && (noise_variable_313 == 0.0)) {
        let noise_metadata_schedule_511_e5496: f64 = (noise_variable_120 / noise_variable_118);
        let noise_metadata_schedule_511_e5499: f64 = (noise_variable_121 / noise_variable_119);
        let noise_metadata_schedule_511_e5500: f64 = (noise_metadata_schedule_511_e5496 + noise_metadata_schedule_511_e5499);
        let noise_metadata_schedule_511_e5503: f64 = (noise_variable_120 / noise_variable_203);
        let noise_metadata_schedule_511_e5505: f64 = (noise_metadata_schedule_511_e5503 * noise_variable_146);
        let noise_metadata_schedule_511_e5507: f64 = (noise_metadata_schedule_511_e5505 * noise_variable_146);
        let noise_metadata_schedule_511_e5508: f64 = (noise_metadata_schedule_511_e5500 + noise_metadata_schedule_511_e5507);
        let noise_metadata_schedule_511_e5509: f64 = (-noise_metadata_schedule_511_e5508);
        (noise_metadata_schedule_511_e5509,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_511_e5511;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_512_e5525,) = {
    if (noise_variable_311 == 0.0) {
        let noise_metadata_schedule_512_e5515: f64 = (-noise_variable_120);
        let noise_metadata_schedule_512_e5517: f64 = (noise_metadata_schedule_512_e5515 * noise_variable_120);
        let noise_metadata_schedule_512_e5519: f64 = (noise_metadata_schedule_512_e5517 / noise_variable_142);
        let noise_metadata_schedule_512_e5521: f64 = (noise_metadata_schedule_512_e5519 * noise_variable_205);
        let noise_metadata_schedule_512_e5523: f64 = (noise_metadata_schedule_512_e5521 / noise_variable_203);
        (noise_metadata_schedule_512_e5523,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_512_e5525;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_513_e5532,) = {
    if (noise_variable_311 == 0.0) {
        let noise_metadata_schedule_513_e5530: f64 = (noise_variable_84 * noise_variable_84);
        (noise_metadata_schedule_513_e5530,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_513_e5532;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_514_e5541,) = {
    if (noise_variable_311 == 0.0) {
        let noise_metadata_schedule_514_e5538: f64 = (noise_variable_87 * noise_variable_83);
        let noise_metadata_schedule_514_e5539: f64 = (noise_variable_85 - noise_metadata_schedule_514_e5538);
        (noise_metadata_schedule_514_e5539,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_514_e5541;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_515_e5560,) = {
    if (noise_variable_311 == 0.0) {
        let noise_metadata_schedule_515_e5546: f64 = (2.0 * noise_variable_84);
        let noise_metadata_schedule_515_e5548: f64 = (noise_metadata_schedule_515_e5546 * noise_variable_87);
        let noise_metadata_schedule_515_e5550: f64 = (noise_metadata_schedule_515_e5548 / 27.0);
        let noise_metadata_schedule_515_e5553: f64 = (noise_variable_84 * noise_variable_85);
        let noise_metadata_schedule_515_e5555: f64 = (noise_metadata_schedule_515_e5553 * noise_variable_83);
        let noise_metadata_schedule_515_e5556: f64 = (noise_metadata_schedule_515_e5550 - noise_metadata_schedule_515_e5555);
        let noise_metadata_schedule_515_e5558: f64 = (noise_metadata_schedule_515_e5556 + noise_variable_86);
        (noise_metadata_schedule_515_e5558,)
    } else {
        (noise_variable_89,)
    }
};
            noise_variable_89 = noise_metadata_schedule_515_e5560;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_516_e5577,) = {
    if (noise_variable_311 == 0.0) {
        let noise_metadata_schedule_516_e5565: f64 = (noise_variable_89 * noise_variable_89);
        let noise_metadata_schedule_516_e5567: f64 = (noise_metadata_schedule_516_e5565 * 0.25);
        let noise_metadata_schedule_516_e5570: f64 = (noise_variable_88 * noise_variable_88);
        let noise_metadata_schedule_516_e5572: f64 = (noise_metadata_schedule_516_e5570 * noise_variable_88);
        let noise_metadata_schedule_516_e5574: f64 = (noise_metadata_schedule_516_e5572 / 27.0);
        let noise_metadata_schedule_516_e5575: f64 = (noise_metadata_schedule_516_e5567 + noise_metadata_schedule_516_e5574);
        (noise_metadata_schedule_516_e5575,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_516_e5577;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_517_e5579: f64 = (noise_variable_90).abs();
            let noise_metadata_schedule_517_e5581: f64 = if noise_metadata_schedule_517_e5579 < 1e-10 { 1.0 } else { 0.0 };
            noise_variable_314 = noise_metadata_schedule_517_e5581;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_518_e5596,) = {
    if ((noise_variable_311 == 0.0) && (noise_variable_314 != 0.0)) {
        let noise_metadata_schedule_518_e5588: f64 = (3.0 * noise_variable_89);
        let noise_metadata_schedule_518_e5590: f64 = (noise_metadata_schedule_518_e5588 / noise_variable_88);
        let noise_metadata_schedule_518_e5593: f64 = (noise_variable_84 * noise_variable_83);
        let noise_metadata_schedule_518_e5594: f64 = (noise_metadata_schedule_518_e5590 - noise_metadata_schedule_518_e5593);
        (noise_metadata_schedule_518_e5594,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_518_e5596;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_519_e5599: f64 = if noise_variable_90 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_315 = noise_metadata_schedule_519_e5599;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_520_e5612,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_520_e5608: f64 = (-noise_variable_89);
        let noise_metadata_schedule_520_e5610: f64 = (noise_metadata_schedule_520_e5608 * 0.5);
        (noise_metadata_schedule_520_e5610,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_520_e5612;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_521_e5623,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_521_e5621: f64 = (noise_variable_90).sqrt();
        (noise_metadata_schedule_521_e5621,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_521_e5623;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_522_e5635,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_522_e5633: f64 = (noise_variable_92 + noise_variable_93);
        (noise_metadata_schedule_522_e5633,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_522_e5635;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_523_e5638: f64 = if noise_variable_87 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_316 = noise_metadata_schedule_523_e5638;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_524_e5654,) = {
    if ((((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) && (noise_variable_316 != 0.0)) {
        let noise_metadata_schedule_524_e5650: f64 = (noise_variable_87).ln();
        let noise_metadata_schedule_524_e5651: f64 = (noise_variable_83 * noise_metadata_schedule_524_e5650);
        let noise_metadata_schedule_524_e5652: f64 = (noise_metadata_schedule_524_e5651).exp();
        (noise_metadata_schedule_524_e5652,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_524_e5654;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_525_e5673,) = {
    if ((((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) && (noise_variable_316 == 0.0)) {
        let noise_metadata_schedule_525_e5667: f64 = (-noise_variable_87);
        let noise_metadata_schedule_525_e5668: f64 = (noise_metadata_schedule_525_e5667).ln();
        let noise_metadata_schedule_525_e5669: f64 = (noise_variable_83 * noise_metadata_schedule_525_e5668);
        let noise_metadata_schedule_525_e5670: f64 = (noise_metadata_schedule_525_e5669).exp();
        let noise_metadata_schedule_525_e5671: f64 = (-noise_metadata_schedule_525_e5670);
        (noise_metadata_schedule_525_e5671,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_525_e5673;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_526_e5685,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_526_e5683: f64 = (noise_variable_92 - noise_variable_93);
        (noise_metadata_schedule_526_e5683,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_526_e5685;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_527_e5688: f64 = if noise_variable_87 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_317 = noise_metadata_schedule_527_e5688;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_528_e5704,) = {
    if ((((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) && (noise_variable_317 != 0.0)) {
        let noise_metadata_schedule_528_e5700: f64 = (noise_variable_87).ln();
        let noise_metadata_schedule_528_e5701: f64 = (noise_variable_83 * noise_metadata_schedule_528_e5700);
        let noise_metadata_schedule_528_e5702: f64 = (noise_metadata_schedule_528_e5701).exp();
        (noise_metadata_schedule_528_e5702,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_528_e5704;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_529_e5723,) = {
    if ((((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) && (noise_variable_317 == 0.0)) {
        let noise_metadata_schedule_529_e5717: f64 = (-noise_variable_87);
        let noise_metadata_schedule_529_e5718: f64 = (noise_metadata_schedule_529_e5717).ln();
        let noise_metadata_schedule_529_e5719: f64 = (noise_variable_83 * noise_metadata_schedule_529_e5718);
        let noise_metadata_schedule_529_e5720: f64 = (noise_metadata_schedule_529_e5719).exp();
        let noise_metadata_schedule_529_e5721: f64 = (-noise_metadata_schedule_529_e5720);
        (noise_metadata_schedule_529_e5721,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_529_e5723;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_530_e5739,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_530_e5733: f64 = (noise_variable_94 + noise_variable_95);
        let noise_metadata_schedule_530_e5736: f64 = (noise_variable_84 * noise_variable_83);
        let noise_metadata_schedule_530_e5737: f64 = (noise_metadata_schedule_530_e5733 - noise_metadata_schedule_530_e5736);
        (noise_metadata_schedule_530_e5737,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_530_e5739;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_531_e5763,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 == 0.0)) {
        let noise_metadata_schedule_531_e5749: f64 = (-noise_variable_89);
        let noise_metadata_schedule_531_e5751: f64 = (noise_metadata_schedule_531_e5749 * 0.5);
        let noise_metadata_schedule_531_e5753: f64 = (-27.0);
        let noise_metadata_schedule_531_e5756: f64 = (noise_variable_88 * noise_variable_88);
        let noise_metadata_schedule_531_e5758: f64 = (noise_metadata_schedule_531_e5756 * noise_variable_88);
        let noise_metadata_schedule_531_e5759: f64 = (noise_metadata_schedule_531_e5753 / noise_metadata_schedule_531_e5758);
        let noise_metadata_schedule_531_e5760: f64 = (noise_metadata_schedule_531_e5759).sqrt();
        let noise_metadata_schedule_531_e5761: f64 = (noise_metadata_schedule_531_e5751 * noise_metadata_schedule_531_e5760);
        (noise_metadata_schedule_531_e5761,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_531_e5763;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_532_e5776,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 == 0.0)) {
        let noise_metadata_schedule_532_e5774: f64 = (noise_variable_87 * noise_variable_87);
        (noise_metadata_schedule_532_e5774,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_532_e5776;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_533_e5779: f64 = if noise_variable_87 >= 0.0 { 1.0 } else { 0.0 };
            noise_variable_318 = noise_metadata_schedule_533_e5779;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_534_e5802,) = {
    if ((((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 == 0.0)) && (noise_variable_318 != 0.0)) {
        let noise_metadata_schedule_534_e5792: f64 = (3.141592653589793 / 2.0);
        let noise_metadata_schedule_534_e5796: f64 = (1.0 - noise_variable_92);
        let noise_metadata_schedule_534_e5797: f64 = (noise_variable_92 / noise_metadata_schedule_534_e5796);
        let noise_metadata_schedule_534_e5798: f64 = (noise_metadata_schedule_534_e5797).sqrt();
        let noise_metadata_schedule_534_e5799: f64 = (noise_metadata_schedule_534_e5798).atan();
        let noise_metadata_schedule_534_e5800: f64 = (noise_metadata_schedule_534_e5792 - noise_metadata_schedule_534_e5799);
        (noise_metadata_schedule_534_e5800,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_534_e5802;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_535_e5826,) = {
    if ((((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 == 0.0)) && (noise_variable_318 == 0.0)) {
        let noise_metadata_schedule_535_e5816: f64 = (3.141592653589793 / 2.0);
        let noise_metadata_schedule_535_e5820: f64 = (1.0 - noise_variable_92);
        let noise_metadata_schedule_535_e5821: f64 = (noise_variable_92 / noise_metadata_schedule_535_e5820);
        let noise_metadata_schedule_535_e5822: f64 = (noise_metadata_schedule_535_e5821).sqrt();
        let noise_metadata_schedule_535_e5823: f64 = (noise_metadata_schedule_535_e5822).atan();
        let noise_metadata_schedule_535_e5824: f64 = (noise_metadata_schedule_535_e5816 + noise_metadata_schedule_535_e5823);
        (noise_metadata_schedule_535_e5824,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_535_e5826;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_536_e5852,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 == 0.0)) {
        let noise_metadata_schedule_536_e5836: f64 = (-4.0);
        let noise_metadata_schedule_536_e5838: f64 = (noise_metadata_schedule_536_e5836 * noise_variable_88);
        let noise_metadata_schedule_536_e5840: f64 = (noise_metadata_schedule_536_e5838 * noise_variable_83);
        let noise_metadata_schedule_536_e5841: f64 = (noise_metadata_schedule_536_e5840).sqrt();
        let noise_metadata_schedule_536_e5844: f64 = (noise_variable_83 * noise_variable_87);
        let noise_metadata_schedule_536_e5845: f64 = (noise_metadata_schedule_536_e5844).cos();
        let noise_metadata_schedule_536_e5846: f64 = (noise_metadata_schedule_536_e5841 * noise_metadata_schedule_536_e5845);
        let noise_metadata_schedule_536_e5849: f64 = (noise_variable_84 * noise_variable_83);
        let noise_metadata_schedule_536_e5850: f64 = (noise_metadata_schedule_536_e5846 - noise_metadata_schedule_536_e5849);
        (noise_metadata_schedule_536_e5850,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_536_e5852;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_537_e5863,) = {
    if (((noise_variable_311 == 0.0) && (noise_variable_314 == 0.0)) && (noise_variable_315 == 0.0)) {
        (noise_variable_87,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_537_e5863;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_538_e5868,) = {
    if (noise_variable_311 == 0.0) {
        (noise_variable_91,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_538_e5868;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_539_e5871: f64 = if noise_variable_125 < 1e-20 { 1.0 } else { 0.0 };
            noise_variable_319 = noise_metadata_schedule_539_e5871;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_540_e5875,) = {
    if (noise_variable_319 != 0.0) {
        (1e-20,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_540_e5875;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_541_e5878: f64 = (noise_variable_120 / noise_variable_125);
            noise_variable_126 = noise_metadata_schedule_541_e5878;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_542_e5881: f64 = (noise_variable_121 / noise_variable_125);
            noise_variable_127 = noise_metadata_schedule_542_e5881;
        }
        if matches!(source_index, 0 | 5) {
            let noise_metadata_schedule_543_e5884: f64 = if noise_variable_126 < 1e-20 { 1.0 } else { 0.0 };
            noise_variable_320 = noise_metadata_schedule_543_e5884;
        }
        if matches!(source_index, 0 | 5) {
            let (noise_metadata_schedule_544_e5888,) = {
    if (noise_variable_320 != 0.0) {
        (1e-20,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_544_e5888;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let noise_metadata_schedule_556_e5950: f64 = if params.p15 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_321 = noise_metadata_schedule_556_e5950;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_557_e5958,) = {
    if (noise_variable_321 != 0.0) {
        let noise_metadata_schedule_557_e5955: f64 = (params.p16 * noise_variable_2);
        let noise_metadata_schedule_557_e5956: f64 = (noise_variable_185 / noise_metadata_schedule_557_e5955);
        (noise_metadata_schedule_557_e5956,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_557_e5958;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let noise_metadata_schedule_558_e5961: f64 = if noise_variable_48 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_322 = noise_metadata_schedule_558_e5961;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_559_e5971,) = {
    if ((noise_variable_321 != 0.0) && (noise_variable_322 != 0.0)) {
        let noise_metadata_schedule_559_e5968: f64 = (noise_variable_48 - 80.0);
        let noise_metadata_schedule_559_e5969: f64 = (1.0 + noise_metadata_schedule_559_e5968);
        (noise_metadata_schedule_559_e5969,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_559_e5971;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_560_e5977,) = {
    if ((noise_variable_321 != 0.0) && (noise_variable_322 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_560_e5977;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_561_e5984,) = {
    if ((noise_variable_321 != 0.0) && (noise_variable_322 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_561_e5984;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_562_e5995,) = {
    if (noise_variable_321 != 0.0) {
        let noise_metadata_schedule_562_e5989: f64 = { let limexp_arg = noise_variable_48; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_562_e5990: f64 = (noise_variable_49 * noise_metadata_schedule_562_e5989);
        let noise_metadata_schedule_562_e5992: f64 = (noise_metadata_schedule_562_e5990 - 1.0);
        let noise_metadata_schedule_562_e5993: f64 = (noise_variable_13 * noise_metadata_schedule_562_e5992);
        (noise_metadata_schedule_562_e5993,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_562_e5995;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_563_e6000,) = {
    if (noise_variable_321 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_563_e6000;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let noise_metadata_schedule_564_e6003: f64 = if params.p17 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_323 = noise_metadata_schedule_564_e6003;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_565_e6011,) = {
    if (noise_variable_323 != 0.0) {
        let noise_metadata_schedule_565_e6008: f64 = (params.p18 * noise_variable_2);
        let noise_metadata_schedule_565_e6009: f64 = (noise_variable_185 / noise_metadata_schedule_565_e6008);
        (noise_metadata_schedule_565_e6009,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_565_e6011;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let noise_metadata_schedule_566_e6014: f64 = if noise_variable_48 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_324 = noise_metadata_schedule_566_e6014;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_567_e6024,) = {
    if ((noise_variable_323 != 0.0) && (noise_variable_324 != 0.0)) {
        let noise_metadata_schedule_567_e6021: f64 = (noise_variable_48 - 80.0);
        let noise_metadata_schedule_567_e6022: f64 = (1.0 + noise_metadata_schedule_567_e6021);
        (noise_metadata_schedule_567_e6022,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_567_e6024;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_568_e6030,) = {
    if ((noise_variable_323 != 0.0) && (noise_variable_324 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_568_e6030;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_569_e6037,) = {
    if ((noise_variable_323 != 0.0) && (noise_variable_324 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_569_e6037;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_570_e6048,) = {
    if (noise_variable_323 != 0.0) {
        let noise_metadata_schedule_570_e6042: f64 = { let limexp_arg = noise_variable_48; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_570_e6043: f64 = (noise_variable_49 * noise_metadata_schedule_570_e6042);
        let noise_metadata_schedule_570_e6045: f64 = (noise_metadata_schedule_570_e6043 - 1.0);
        let noise_metadata_schedule_570_e6046: f64 = (noise_variable_12 * noise_metadata_schedule_570_e6045);
        (noise_metadata_schedule_570_e6046,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_570_e6048;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let (noise_metadata_schedule_571_e6053,) = {
    if (noise_variable_323 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_571_e6053;
        }
        if matches!(source_index, 0 | 3 | 4) {
            let noise_metadata_schedule_572_e6056: f64 = (noise_variable_134 + noise_variable_135);
            noise_variable_195 = noise_metadata_schedule_572_e6056;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_573_e6059: f64 = if params.p19 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_325 = noise_metadata_schedule_573_e6059;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_574_e6067,) = {
    if (noise_variable_325 != 0.0) {
        let noise_metadata_schedule_574_e6064: f64 = (params.p20 * noise_variable_2);
        let noise_metadata_schedule_574_e6065: f64 = (noise_variable_184 / noise_metadata_schedule_574_e6064);
        (noise_metadata_schedule_574_e6065,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_574_e6067;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_575_e6070: f64 = if noise_variable_48 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_326 = noise_metadata_schedule_575_e6070;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_576_e6080,) = {
    if ((noise_variable_325 != 0.0) && (noise_variable_326 != 0.0)) {
        let noise_metadata_schedule_576_e6077: f64 = (noise_variable_48 - 80.0);
        let noise_metadata_schedule_576_e6078: f64 = (1.0 + noise_metadata_schedule_576_e6077);
        (noise_metadata_schedule_576_e6078,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_576_e6080;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_577_e6086,) = {
    if ((noise_variable_325 != 0.0) && (noise_variable_326 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_48,)
    }
};
            noise_variable_48 = noise_metadata_schedule_577_e6086;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_578_e6093,) = {
    if ((noise_variable_325 != 0.0) && (noise_variable_326 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_578_e6093;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_579_e6104,) = {
    if (noise_variable_325 != 0.0) {
        let noise_metadata_schedule_579_e6098: f64 = { let limexp_arg = noise_variable_48; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_579_e6099: f64 = (noise_variable_49 * noise_metadata_schedule_579_e6098);
        let noise_metadata_schedule_579_e6101: f64 = (noise_metadata_schedule_579_e6099 - 1.0);
        let noise_metadata_schedule_579_e6102: f64 = (noise_variable_14 * noise_metadata_schedule_579_e6101);
        (noise_metadata_schedule_579_e6102,)
    } else {
        (noise_variable_192,)
    }
};
            noise_variable_192 = noise_metadata_schedule_579_e6104;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_580_e6109,) = {
    if (noise_variable_325 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_192,)
    }
};
            noise_variable_192 = noise_metadata_schedule_580_e6109;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_581_e6112: f64 = (noise_variable_195 + noise_variable_192);
            noise_variable_136 = noise_metadata_schedule_581_e6112;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_645_e6766: f64 = if noise_variable_37 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_340 = noise_metadata_schedule_645_e6766;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_646_e6786,) = {
    if (noise_variable_340 != 0.0) {
        let noise_metadata_schedule_646_e6771: f64 = (noise_variable_102 / params.p24);
        let noise_metadata_schedule_646_e6772: f64 = (1.0 + noise_metadata_schedule_646_e6771);
        let noise_metadata_schedule_646_e6775: f64 = (noise_variable_103 / params.p25);
        let noise_metadata_schedule_646_e6776: f64 = (noise_metadata_schedule_646_e6772 + noise_metadata_schedule_646_e6775);
        let noise_metadata_schedule_646_e6779: f64 = (noise_variable_126 / noise_variable_118);
        let noise_metadata_schedule_646_e6780: f64 = (noise_metadata_schedule_646_e6776 + noise_metadata_schedule_646_e6779);
        let noise_metadata_schedule_646_e6783: f64 = (noise_variable_127 / noise_variable_119);
        let noise_metadata_schedule_646_e6784: f64 = (noise_metadata_schedule_646_e6780 + noise_metadata_schedule_646_e6783);
        (noise_metadata_schedule_646_e6784,)
    } else {
        (noise_variable_160,)
    }
};
            noise_variable_160 = noise_metadata_schedule_646_e6786;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_647_e6799,) = {
    if (noise_variable_340 != 0.0) {
        let noise_metadata_schedule_647_e6792: f64 = (noise_variable_160 * noise_variable_160);
        let noise_metadata_schedule_647_e6794: f64 = (noise_metadata_schedule_647_e6792 + 0.01);
        let noise_metadata_schedule_647_e6795: f64 = (noise_metadata_schedule_647_e6794).sqrt();
        let noise_metadata_schedule_647_e6796: f64 = (noise_variable_160 + noise_metadata_schedule_647_e6795);
        let noise_metadata_schedule_647_e6797: f64 = (0.5 * noise_metadata_schedule_647_e6796);
        (noise_metadata_schedule_647_e6797,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_647_e6799;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_648_e6805,) = {
    if (noise_variable_340 != 0.0) {
        let noise_metadata_schedule_648_e6803: f64 = (noise_variable_37 / noise_variable_161);
        (noise_metadata_schedule_648_e6803,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_648_e6805;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_649_e6808: f64 = if noise_variable_136 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_341 = noise_metadata_schedule_649_e6808;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_650_e6820,) = {
    if ((noise_variable_340 != 0.0) && (noise_variable_341 != 0.0)) {
        let noise_metadata_schedule_650_e6814: f64 = (params.p27 * noise_variable_158);
        let noise_metadata_schedule_650_e6816: f64 = (noise_metadata_schedule_650_e6814 * noise_variable_136);
        let noise_metadata_schedule_650_e6818: f64 = (noise_metadata_schedule_650_e6816 * noise_variable_3);
        (noise_metadata_schedule_650_e6818,)
    } else {
        (noise_variable_157,)
    }
};
            noise_variable_157 = noise_metadata_schedule_650_e6820;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_651_e6823: f64 = if noise_variable_157 < 1e-6 { 1.0 } else { 0.0 };
            noise_variable_342 = noise_metadata_schedule_651_e6823;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_652_e6837,) = {
    if (((noise_variable_340 != 0.0) && (noise_variable_341 != 0.0)) && (noise_variable_342 != 0.0)) {
        let noise_metadata_schedule_652_e6833: f64 = (0.5 * noise_variable_157);
        let noise_metadata_schedule_652_e6834: f64 = (1.0 - noise_metadata_schedule_652_e6833);
        let noise_metadata_schedule_652_e6835: f64 = (noise_variable_158 * noise_metadata_schedule_652_e6834);
        (noise_metadata_schedule_652_e6835,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_652_e6837;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_653_e6853,) = {
    if (((noise_variable_340 != 0.0) && (noise_variable_341 != 0.0)) && (noise_variable_342 == 0.0)) {
        let noise_metadata_schedule_653_e6847: f64 = (noise_variable_157 + 1.0);
        let noise_metadata_schedule_653_e6848: f64 = (noise_metadata_schedule_653_e6847).ln();
        let noise_metadata_schedule_653_e6849: f64 = (noise_variable_158 * noise_metadata_schedule_653_e6848);
        let noise_metadata_schedule_653_e6851: f64 = (noise_metadata_schedule_653_e6849 / noise_variable_157);
        (noise_metadata_schedule_653_e6851,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_653_e6853;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_654_e6858,) = {
    if (noise_variable_340 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_654_e6858;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_655_e6861: f64 = (noise_variable_158 + noise_variable_39);
            noise_variable_156 = noise_metadata_schedule_655_e6861;
        }
        if matches!(source_index, 5) {
            noise_variable_211 = noise_variable_126;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_726_e7532: f64 = if ((params.p73 != 0.0) && (params.p54 != 0.0)) { 1.0 } else { 0.0 };
            noise_variable_355 = noise_metadata_schedule_726_e7532;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_730_e7554,) = {
    if (noise_variable_355 != 0.0) {
        ((ctx.node_voltage(self.nodes[9]) - 0.0),)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_730_e7554;
        }
        if matches!(source_index, 3 | 4) {
            let noise_metadata_schedule_749_e7627: f64 = (params.p110 * noise_variable_195);
            noise_variable_195 = noise_metadata_schedule_749_e7627;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_752_e7637: f64 = (noise_variable_211 - noise_variable_127);
            let noise_metadata_schedule_752_e7638: f64 = (params.p110 * noise_metadata_schedule_752_e7637);
            noise_variable_132 = noise_metadata_schedule_752_e7638;
        }
        if matches!(source_index, 0 | 1 | 2) {
            let noise_metadata_schedule_761_e7683: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_761_e7685: f64 = (noise_metadata_schedule_761_e7683 * noise_variable_4);
            noise_variable_361 = noise_metadata_schedule_761_e7685;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_765_e7701: f64 = (noise_variable_195).abs();
            let noise_metadata_schedule_765_e7703: f64 = (noise_metadata_schedule_765_e7701).powf(params.p75);
            let noise_metadata_schedule_765_e7704: f64 = (params.p74 * noise_metadata_schedule_765_e7703);
            noise_variable_362 = noise_metadata_schedule_765_e7704;
        }
        if matches!(source_index, 4 | 5) {
            let noise_metadata_schedule_766_e7707: f64 = (2.0 * 1.602176462e-19);
            noise_variable_363 = noise_metadata_schedule_766_e7707;
        }
        match source_index {
            0 => {
                let noise_0_psd_e7957: f64 = 1.0;
                let noise_0_psd_e220: f64 = (noise_variable_361 / noise_variable_156);
                let noise_0_psd_e7958: f64 = (noise_0_psd_e7957 * noise_0_psd_e220);
                let psd = noise_0_psd_e7958;
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
                let noise_1_psd_e7960: f64 = 1.0;
                let noise_1_psd_e228: f64 = (noise_variable_361 / noise_variable_40);
                let noise_1_psd_e7961: f64 = (noise_1_psd_e7960 * noise_1_psd_e228);
                let psd = noise_1_psd_e7961;
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
                let noise_2_psd_e7963: f64 = 1.0;
                let noise_2_psd_e236: f64 = (noise_variable_361 / noise_variable_41);
                let noise_2_psd_e7964: f64 = (noise_2_psd_e7963 * noise_2_psd_e236);
                let psd = noise_2_psd_e7964;
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
                let noise_3_psd_e7966: f64 = 1.0;
                let noise_3_psd_e7967: f64 = (noise_3_psd_e7966 * noise_variable_362);
                let psd = noise_3_psd_e7967;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            4 => {
                let noise_4_psd_e7969: f64 = 1.0;
                let noise_4_psd_e247: f64 = (noise_variable_195).abs();
                let noise_4_psd_e248: f64 = (noise_variable_363 * noise_4_psd_e247);
                let noise_4_psd_e7970: f64 = (noise_4_psd_e7969 * noise_4_psd_e248);
                let psd = noise_4_psd_e7970;
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
                let noise_5_psd_e7972: f64 = 1.0;
                let noise_5_psd_e253: f64 = (noise_variable_132).abs();
                let noise_5_psd_e254: f64 = (noise_variable_363 * noise_5_psd_e253);
                let noise_5_psd_e7973: f64 = (noise_5_psd_e7972 * noise_5_psd_e254);
                let psd = noise_5_psd_e7973;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

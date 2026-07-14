#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 13] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI_SHOT_NOISE", label: Some("Ibei shot noise"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_IBEI_FLICKER_NOISE", label: Some("Ibei flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_EI_IBEX_SHOT_NOISE", label: Some("Ibex shot noise"), kind: GeneratedNoiseKind::White, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_EI_IBEX_FLICKER_NOISE", label: Some("Ibex flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BP_IBEP_SHOT_NOISE", label: Some("Ibep shot noise"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_BP_IBEP_FLICKER_NOISE", label: Some("Ibep flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CX_RCX_THERMAL_NOISE", label: Some("rcx thermal noise"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CX_CI_RCI_THERMAL_NOISE", label: Some("rci thermal noise"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BX_RBX_THERMAL_NOISE", label: Some("rbx thermal noise"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BI_RBI_THERMAL_NOISE", label: Some("rbi thermal noise"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE_THERMAL_NOISE", label: Some("re thermal noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CX_RBP_THERMAL_NOISE", label: Some("rbp thermal noise"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let noise_metadata_schedule_0_e376: f64 = if ctx.analysis_initial_step() { 1.0 } else { 0.0 };
            noise_variable_172 = noise_metadata_schedule_0_e376;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_7_e420: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            noise_variable_175 = noise_metadata_schedule_7_e420;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_8_e426,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_175 != 0.0)) {
        (params.p10,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_8_e426;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_9_e435,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_175 == 0.0)) {
        let noise_metadata_schedule_9_e433: f64 = 1e-12;
        (noise_metadata_schedule_9_e433,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_9_e435;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_10_e437: f64 = if self.param_given[11] { 1.0 } else { 0.0 };
            noise_variable_176 = noise_metadata_schedule_10_e437;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_11_e443,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_176 != 0.0)) {
        (params.p11,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_11_e443;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_12_e452,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_176 == 0.0)) {
        let noise_metadata_schedule_12_e450: f64 = 1.0;
        (noise_metadata_schedule_12_e450,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_12_e452;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_13_e454: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            noise_variable_177 = noise_metadata_schedule_13_e454;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_14_e461,) = {
    if ((noise_variable_172 != 0.0) && (noise_variable_177 != 0.0)) {
        let noise_metadata_schedule_14_e459: f64 = 1.0;
        (noise_metadata_schedule_14_e459,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_14_e461;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_15_e463: f64 = if self.param_given[4] { 1.0 } else { 0.0 };
            noise_variable_178 = noise_metadata_schedule_15_e463;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_16_e473,) = {
    if (((noise_variable_172 != 0.0) && (noise_variable_177 == 0.0)) && (noise_variable_178 != 0.0)) {
        let noise_metadata_schedule_16_e471: f64 = (-1.0);
        (noise_metadata_schedule_16_e471,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_16_e473;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_17_e475: f64 = if self.param_given[5] { 1.0 } else { 0.0 };
            noise_variable_179 = noise_metadata_schedule_17_e475;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_18_e487,) = {
    if ((((noise_variable_172 != 0.0) && (noise_variable_177 == 0.0)) && (noise_variable_178 == 0.0)) && (noise_variable_179 != 0.0)) {
        (params.p5,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_18_e487;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_19_e501,) = {
    if ((((noise_variable_172 != 0.0) && (noise_variable_177 == 0.0)) && (noise_variable_178 == 0.0)) && (noise_variable_179 == 0.0)) {
        let noise_metadata_schedule_19_e499: f64 = 1.0;
        (noise_metadata_schedule_19_e499,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_19_e501;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_20_e506,) = {
    if (noise_variable_172 != 0.0) {
        let noise_metadata_schedule_20_e504: f64 = (params.p12).ln();
        (noise_metadata_schedule_20_e504,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_20_e506;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let (noise_metadata_schedule_21_e517,) = {
    if (noise_variable_172 != 0.0) {
        let (noise_metadata_schedule_21_e515,) = {
            if (params.p74 > 0.0) {
                let noise_metadata_schedule_21_e513: f64 = (1.0 / params.p74);
                (noise_metadata_schedule_21_e513,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_21_e515,)
    } else {
        (noise_variable_46,)
    }
};
            noise_variable_46 = noise_metadata_schedule_21_e517;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let (noise_metadata_schedule_22_e528,) = {
    if (noise_variable_172 != 0.0) {
        let (noise_metadata_schedule_22_e526,) = {
            if (params.p75 > 0.0) {
                let noise_metadata_schedule_22_e524: f64 = (1.0 / params.p75);
                (noise_metadata_schedule_22_e524,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_22_e526,)
    } else {
        (noise_variable_47,)
    }
};
            noise_variable_47 = noise_metadata_schedule_22_e528;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_23_e539,) = {
    if (noise_variable_172 != 0.0) {
        let (noise_metadata_schedule_23_e537,) = {
            if (params.p20 > 0.0) {
                let noise_metadata_schedule_23_e535: f64 = (1.0 / params.p20);
                (noise_metadata_schedule_23_e535,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_23_e537,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_23_e539;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let (noise_metadata_schedule_27_e576,) = {
    if (noise_variable_172 != 0.0) {
        let noise_metadata_schedule_27_e574: f64 = (273.15 + params.p13);
        (noise_metadata_schedule_27_e574,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_27_e576;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_29_e578: f64 = ctx.temperature();
            let noise_metadata_schedule_29_e580: f64 = (noise_metadata_schedule_29_e578 + params.p0);
            let noise_metadata_schedule_29_e582: f64 = (noise_metadata_schedule_29_e580 - 273.15);
            noise_variable_38 = noise_metadata_schedule_29_e582;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_32_e592: f64 = (params.p14 + 1.0);
            let noise_metadata_schedule_32_e593: f64 = if noise_variable_38 < noise_metadata_schedule_32_e592 { 1.0 } else { 0.0 };
            noise_variable_182 = noise_metadata_schedule_32_e593;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_33_e604,) = {
    if (noise_variable_182 != 0.0) {
        let noise_metadata_schedule_33_e598: f64 = (noise_variable_38 - params.p14);
        let noise_metadata_schedule_33_e600: f64 = (noise_metadata_schedule_33_e598 - 1.0);
        let noise_metadata_schedule_33_e601: f64 = (noise_metadata_schedule_33_e600).exp();
        let noise_metadata_schedule_33_e602: f64 = (params.p14 + noise_metadata_schedule_33_e601);
        (noise_metadata_schedule_33_e602,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_33_e604;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_34_e608: f64 = (params.p15 - 1.0);
            let noise_metadata_schedule_34_e609: f64 = if noise_variable_38 > noise_metadata_schedule_34_e608 { 1.0 } else { 0.0 };
            noise_variable_183 = noise_metadata_schedule_34_e609;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_35_e623,) = {
    if ((noise_variable_182 == 0.0) && (noise_variable_183 != 0.0)) {
        let noise_metadata_schedule_35_e617: f64 = (params.p15 - noise_variable_38);
        let noise_metadata_schedule_35_e619: f64 = (noise_metadata_schedule_35_e617 - 1.0);
        let noise_metadata_schedule_35_e620: f64 = (noise_metadata_schedule_35_e619).exp();
        let noise_metadata_schedule_35_e621: f64 = (params.p15 - noise_metadata_schedule_35_e620);
        (noise_metadata_schedule_35_e621,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_35_e623;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_36_e631,) = {
    if ((noise_variable_182 == 0.0) && (noise_variable_183 == 0.0)) {
        (noise_variable_38,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_36_e631;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_37_e634: f64 = (noise_variable_38 + 273.15);
            noise_variable_39 = noise_metadata_schedule_37_e634;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_38_e637: f64 = (1.380662e-23 * noise_variable_39);
            let noise_metadata_schedule_38_e639: f64 = (noise_metadata_schedule_38_e637 / 1.602189e-19);
            noise_variable_73 = noise_metadata_schedule_38_e639;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_39_e642: f64 = (noise_variable_39 / noise_variable_40);
            noise_variable_41 = noise_metadata_schedule_39_e642;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_41_e655: f64 = if params.p90 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_184 = noise_metadata_schedule_41_e655;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_42_e674,) = {
    if (noise_variable_184 != 0.0) {
        let noise_metadata_schedule_42_e659: f64 = (params.p89 * noise_variable_73);
        let noise_metadata_schedule_42_e661: f64 = (-params.p88);
        let noise_metadata_schedule_42_e664: f64 = (params.p89 * noise_variable_73);
        let noise_metadata_schedule_42_e665: f64 = (noise_metadata_schedule_42_e661 / noise_metadata_schedule_42_e664);
        let noise_metadata_schedule_42_e666: f64 = (noise_metadata_schedule_42_e665).exp();
        let noise_metadata_schedule_42_e669: f64 = (noise_variable_166 / params.p90);
        let noise_metadata_schedule_42_e670: f64 = (noise_metadata_schedule_42_e666 + noise_metadata_schedule_42_e669);
        let noise_metadata_schedule_42_e671: f64 = (noise_metadata_schedule_42_e670).ln();
        let noise_metadata_schedule_42_e672: f64 = (noise_metadata_schedule_42_e659 * noise_metadata_schedule_42_e671);
        (noise_metadata_schedule_42_e672,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_42_e674;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_43_e679,) = {
    if (noise_variable_184 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_43_e679;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_44_e684: f64 = (params.p122 / params.p28);
            let noise_metadata_schedule_44_e685: f64 = (noise_variable_41).powf(noise_metadata_schedule_44_e684);
            let noise_metadata_schedule_44_e686: f64 = (params.p26 * noise_metadata_schedule_44_e685);
            let noise_metadata_schedule_44_e688: f64 = (-params.p113);
            let noise_metadata_schedule_44_e691: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_44_e692: f64 = (noise_metadata_schedule_44_e688 * noise_metadata_schedule_44_e691);
            let noise_metadata_schedule_44_e695: f64 = (noise_variable_73 * params.p28);
            let noise_metadata_schedule_44_e696: f64 = (noise_metadata_schedule_44_e692 / noise_metadata_schedule_44_e695);
            let noise_metadata_schedule_44_e697: f64 = (noise_metadata_schedule_44_e696).exp();
            let noise_metadata_schedule_44_e698: f64 = (noise_metadata_schedule_44_e686 * noise_metadata_schedule_44_e697);
            noise_variable_0 = noise_metadata_schedule_44_e698;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_45_e701: f64 = if noise_variable_0 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_185 = noise_metadata_schedule_45_e701;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_46_e708: f64 = if ((params.p72 > 0.0) && (noise_variable_166 > params.p72)) { 1.0 } else { 0.0 };
            noise_variable_186 = noise_metadata_schedule_46_e708;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_47_e737,) = {
    if ((noise_variable_185 != 0.0) && (noise_variable_186 != 0.0)) {
        let noise_metadata_schedule_47_e714: f64 = (params.p28 * noise_variable_73);
        let noise_metadata_schedule_47_e718: f64 = (0.5 * noise_variable_166);
        let noise_metadata_schedule_47_e721: f64 = (4.0 / params.p72);
        let noise_metadata_schedule_47_e723: f64 = (noise_metadata_schedule_47_e721).powf(params.p73);
        let noise_metadata_schedule_47_e724: f64 = (noise_metadata_schedule_47_e718 * noise_metadata_schedule_47_e723);
        let noise_metadata_schedule_47_e728: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_47_e729: f64 = (1.0 / noise_metadata_schedule_47_e728);
        let noise_metadata_schedule_47_e730: f64 = (noise_metadata_schedule_47_e724).powf(noise_metadata_schedule_47_e729);
        let noise_metadata_schedule_47_e732: f64 = (noise_metadata_schedule_47_e730 / noise_variable_0);
        let noise_metadata_schedule_47_e733: f64 = (1.0 + noise_metadata_schedule_47_e732);
        let noise_metadata_schedule_47_e734: f64 = (noise_metadata_schedule_47_e733).ln();
        let noise_metadata_schedule_47_e735: f64 = (noise_metadata_schedule_47_e714 * noise_metadata_schedule_47_e734);
        (noise_metadata_schedule_47_e735,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_47_e737;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_48_e753,) = {
    if ((noise_variable_185 != 0.0) && (noise_variable_186 == 0.0)) {
        let noise_metadata_schedule_48_e744: f64 = (params.p28 * noise_variable_73);
        let noise_metadata_schedule_48_e748: f64 = (noise_variable_166 / noise_variable_0);
        let noise_metadata_schedule_48_e749: f64 = (1.0 + noise_metadata_schedule_48_e748);
        let noise_metadata_schedule_48_e750: f64 = (noise_metadata_schedule_48_e749).ln();
        let noise_metadata_schedule_48_e751: f64 = (noise_metadata_schedule_48_e744 * noise_metadata_schedule_48_e750);
        (noise_metadata_schedule_48_e751,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_48_e753;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_49_e758,) = {
    if (noise_variable_185 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_49_e758;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_50_e763: f64 = (params.p125 / params.p29);
            let noise_metadata_schedule_50_e764: f64 = (noise_variable_41).powf(noise_metadata_schedule_50_e763);
            let noise_metadata_schedule_50_e765: f64 = (params.p27 * noise_metadata_schedule_50_e764);
            let noise_metadata_schedule_50_e767: f64 = (-params.p121);
            let noise_metadata_schedule_50_e770: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_50_e771: f64 = (noise_metadata_schedule_50_e767 * noise_metadata_schedule_50_e770);
            let noise_metadata_schedule_50_e774: f64 = (noise_variable_73 * params.p29);
            let noise_metadata_schedule_50_e775: f64 = (noise_metadata_schedule_50_e771 / noise_metadata_schedule_50_e774);
            let noise_metadata_schedule_50_e776: f64 = (noise_metadata_schedule_50_e775).exp();
            let noise_metadata_schedule_50_e777: f64 = (noise_metadata_schedule_50_e765 * noise_metadata_schedule_50_e776);
            noise_variable_1 = noise_metadata_schedule_50_e777;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_51_e784: f64 = if ((noise_variable_0 > 0.0) && (noise_variable_1 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_187 = noise_metadata_schedule_51_e784;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_52_e791: f64 = if ((params.p74 > 0.0) && (noise_variable_166 > params.p74)) { 1.0 } else { 0.0 };
            noise_variable_188 = noise_metadata_schedule_52_e791;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_53_e822,) = {
    if ((noise_variable_187 != 0.0) && (noise_variable_188 != 0.0)) {
        let noise_metadata_schedule_53_e797: f64 = (params.p29 * noise_variable_73);
        let noise_metadata_schedule_53_e801: f64 = (0.5 * noise_variable_166);
        let noise_metadata_schedule_53_e804: f64 = (4.0 / params.p74);
        let noise_metadata_schedule_53_e806: f64 = (noise_metadata_schedule_53_e804).powf(params.p73);
        let noise_metadata_schedule_53_e807: f64 = (noise_metadata_schedule_53_e801 * noise_metadata_schedule_53_e806);
        let noise_metadata_schedule_53_e811: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_53_e812: f64 = (1.0 / noise_metadata_schedule_53_e811);
        let noise_metadata_schedule_53_e813: f64 = (noise_metadata_schedule_53_e807).powf(noise_metadata_schedule_53_e812);
        let noise_metadata_schedule_53_e816: f64 = (noise_variable_0 * noise_variable_1);
        let noise_metadata_schedule_53_e817: f64 = (noise_metadata_schedule_53_e813 / noise_metadata_schedule_53_e816);
        let noise_metadata_schedule_53_e818: f64 = (1.0 + noise_metadata_schedule_53_e817);
        let noise_metadata_schedule_53_e819: f64 = (noise_metadata_schedule_53_e818).ln();
        let noise_metadata_schedule_53_e820: f64 = (noise_metadata_schedule_53_e797 * noise_metadata_schedule_53_e819);
        (noise_metadata_schedule_53_e820,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_53_e822;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_54_e840,) = {
    if ((noise_variable_187 != 0.0) && (noise_variable_188 == 0.0)) {
        let noise_metadata_schedule_54_e829: f64 = (params.p29 * noise_variable_73);
        let noise_metadata_schedule_54_e834: f64 = (noise_variable_0 * noise_variable_1);
        let noise_metadata_schedule_54_e835: f64 = (noise_variable_166 / noise_metadata_schedule_54_e834);
        let noise_metadata_schedule_54_e836: f64 = (1.0 + noise_metadata_schedule_54_e835);
        let noise_metadata_schedule_54_e837: f64 = (noise_metadata_schedule_54_e836).ln();
        let noise_metadata_schedule_54_e838: f64 = (noise_metadata_schedule_54_e829 * noise_metadata_schedule_54_e837);
        (noise_metadata_schedule_54_e838,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_54_e840;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_55_e845,) = {
    if (noise_variable_187 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_55_e845;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let noise_metadata_schedule_56_e850: f64 = (params.p122 / params.p33);
            let noise_metadata_schedule_56_e851: f64 = (noise_variable_41).powf(noise_metadata_schedule_56_e850);
            let noise_metadata_schedule_56_e852: f64 = (params.p31 * noise_metadata_schedule_56_e851);
            let noise_metadata_schedule_56_e854: f64 = (-params.p120);
            let noise_metadata_schedule_56_e857: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_56_e858: f64 = (noise_metadata_schedule_56_e854 * noise_metadata_schedule_56_e857);
            let noise_metadata_schedule_56_e861: f64 = (noise_variable_73 * params.p33);
            let noise_metadata_schedule_56_e862: f64 = (noise_metadata_schedule_56_e858 / noise_metadata_schedule_56_e861);
            let noise_metadata_schedule_56_e863: f64 = (noise_metadata_schedule_56_e862).exp();
            let noise_metadata_schedule_56_e864: f64 = (noise_metadata_schedule_56_e852 * noise_metadata_schedule_56_e863);
            noise_variable_5 = noise_metadata_schedule_56_e864;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let noise_metadata_schedule_57_e867: f64 = if noise_variable_5 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_189 = noise_metadata_schedule_57_e867;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let noise_metadata_schedule_58_e874: f64 = if ((params.p75 > 0.0) && (noise_variable_166 > params.p75)) { 1.0 } else { 0.0 };
            noise_variable_190 = noise_metadata_schedule_58_e874;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let (noise_metadata_schedule_59_e893,) = {
    if ((noise_variable_189 != 0.0) && (noise_variable_190 != 0.0)) {
        let noise_metadata_schedule_59_e880: f64 = (params.p33 * noise_variable_73);
        let noise_metadata_schedule_59_e884: f64 = (noise_variable_166 * noise_variable_166);
        let noise_metadata_schedule_59_e886: f64 = (noise_metadata_schedule_59_e884 * noise_variable_47);
        let noise_metadata_schedule_59_e888: f64 = (noise_metadata_schedule_59_e886 / noise_variable_5);
        let noise_metadata_schedule_59_e889: f64 = (1.0 + noise_metadata_schedule_59_e888);
        let noise_metadata_schedule_59_e890: f64 = (noise_metadata_schedule_59_e889).ln();
        let noise_metadata_schedule_59_e891: f64 = (noise_metadata_schedule_59_e880 * noise_metadata_schedule_59_e890);
        (noise_metadata_schedule_59_e891,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_59_e893;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let (noise_metadata_schedule_60_e909,) = {
    if ((noise_variable_189 != 0.0) && (noise_variable_190 == 0.0)) {
        let noise_metadata_schedule_60_e900: f64 = (params.p33 * noise_variable_73);
        let noise_metadata_schedule_60_e904: f64 = (noise_variable_166 / noise_variable_5);
        let noise_metadata_schedule_60_e905: f64 = (1.0 + noise_metadata_schedule_60_e904);
        let noise_metadata_schedule_60_e906: f64 = (noise_metadata_schedule_60_e905).ln();
        let noise_metadata_schedule_60_e907: f64 = (noise_metadata_schedule_60_e900 * noise_metadata_schedule_60_e906);
        (noise_metadata_schedule_60_e907,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_60_e909;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let (noise_metadata_schedule_61_e914,) = {
    if (noise_variable_189 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_61_e914;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_62_e919: f64 = (params.p123 / params.p56);
            let noise_metadata_schedule_62_e920: f64 = (noise_variable_41).powf(noise_metadata_schedule_62_e919);
            let noise_metadata_schedule_62_e921: f64 = (params.p54 * noise_metadata_schedule_62_e920);
            let noise_metadata_schedule_62_e923: f64 = (-params.p114);
            let noise_metadata_schedule_62_e926: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_62_e927: f64 = (noise_metadata_schedule_62_e923 * noise_metadata_schedule_62_e926);
            let noise_metadata_schedule_62_e930: f64 = (noise_variable_73 * params.p56);
            let noise_metadata_schedule_62_e931: f64 = (noise_metadata_schedule_62_e927 / noise_metadata_schedule_62_e930);
            let noise_metadata_schedule_62_e932: f64 = (noise_metadata_schedule_62_e931).exp();
            let noise_metadata_schedule_62_e933: f64 = (noise_metadata_schedule_62_e921 * noise_metadata_schedule_62_e932);
            noise_variable_3 = noise_metadata_schedule_62_e933;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_63_e936: f64 = if noise_variable_3 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_191 = noise_metadata_schedule_63_e936;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_64_e949,) = {
    if (noise_variable_191 != 0.0) {
        let noise_metadata_schedule_64_e940: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_64_e944: f64 = (noise_variable_166 / noise_variable_3);
        let noise_metadata_schedule_64_e945: f64 = (1.0 + noise_metadata_schedule_64_e944);
        let noise_metadata_schedule_64_e946: f64 = (noise_metadata_schedule_64_e945).ln();
        let noise_metadata_schedule_64_e947: f64 = (noise_metadata_schedule_64_e940 * noise_metadata_schedule_64_e946);
        (noise_metadata_schedule_64_e947,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_64_e949;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_65_e954,) = {
    if (noise_variable_191 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_65_e954;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_66_e959: f64 = (params.p124 / params.p59);
            let noise_metadata_schedule_66_e960: f64 = (noise_variable_41).powf(noise_metadata_schedule_66_e959);
            let noise_metadata_schedule_66_e961: f64 = (params.p58 * noise_metadata_schedule_66_e960);
            let noise_metadata_schedule_66_e963: f64 = (-params.p117);
            let noise_metadata_schedule_66_e966: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_66_e967: f64 = (noise_metadata_schedule_66_e963 * noise_metadata_schedule_66_e966);
            let noise_metadata_schedule_66_e970: f64 = (noise_variable_73 * params.p59);
            let noise_metadata_schedule_66_e971: f64 = (noise_metadata_schedule_66_e967 / noise_metadata_schedule_66_e970);
            let noise_metadata_schedule_66_e972: f64 = (noise_metadata_schedule_66_e971).exp();
            let noise_metadata_schedule_66_e973: f64 = (noise_metadata_schedule_66_e961 * noise_metadata_schedule_66_e972);
            noise_variable_6 = noise_metadata_schedule_66_e973;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_67_e976: f64 = if noise_variable_6 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_192 = noise_metadata_schedule_67_e976;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_68_e989,) = {
    if (noise_variable_192 != 0.0) {
        let noise_metadata_schedule_68_e980: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_68_e984: f64 = (noise_variable_166 / noise_variable_6);
        let noise_metadata_schedule_68_e985: f64 = (1.0 + noise_metadata_schedule_68_e984);
        let noise_metadata_schedule_68_e986: f64 = (noise_metadata_schedule_68_e985).ln();
        let noise_metadata_schedule_68_e987: f64 = (noise_metadata_schedule_68_e980 * noise_metadata_schedule_68_e986);
        (noise_metadata_schedule_68_e987,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_68_e989;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_69_e994,) = {
    if (noise_variable_192 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_66,)
    }
};
            noise_variable_66 = noise_metadata_schedule_69_e994;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_70_e999: f64 = (params.p123 / params.p61);
            let noise_metadata_schedule_70_e1000: f64 = (noise_variable_41).powf(noise_metadata_schedule_70_e999);
            let noise_metadata_schedule_70_e1001: f64 = (params.p60 * noise_metadata_schedule_70_e1000);
            let noise_metadata_schedule_70_e1003: f64 = (-params.p115);
            let noise_metadata_schedule_70_e1006: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_70_e1007: f64 = (noise_metadata_schedule_70_e1003 * noise_metadata_schedule_70_e1006);
            let noise_metadata_schedule_70_e1010: f64 = (noise_variable_73 * params.p61);
            let noise_metadata_schedule_70_e1011: f64 = (noise_metadata_schedule_70_e1007 / noise_metadata_schedule_70_e1010);
            let noise_metadata_schedule_70_e1012: f64 = (noise_metadata_schedule_70_e1011).exp();
            let noise_metadata_schedule_70_e1013: f64 = (noise_metadata_schedule_70_e1001 * noise_metadata_schedule_70_e1012);
            noise_variable_4 = noise_metadata_schedule_70_e1013;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_71_e1016: f64 = if noise_variable_4 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_193 = noise_metadata_schedule_71_e1016;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_72_e1029,) = {
    if (noise_variable_193 != 0.0) {
        let noise_metadata_schedule_72_e1020: f64 = (params.p61 * noise_variable_73);
        let noise_metadata_schedule_72_e1024: f64 = (noise_variable_166 / noise_variable_4);
        let noise_metadata_schedule_72_e1025: f64 = (1.0 + noise_metadata_schedule_72_e1024);
        let noise_metadata_schedule_72_e1026: f64 = (noise_metadata_schedule_72_e1025).ln();
        let noise_metadata_schedule_72_e1027: f64 = (noise_metadata_schedule_72_e1020 * noise_metadata_schedule_72_e1026);
        (noise_metadata_schedule_72_e1027,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_72_e1029;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_73_e1034,) = {
    if (noise_variable_193 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_73_e1034;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_74_e1039: f64 = (params.p124 / params.p63);
            let noise_metadata_schedule_74_e1040: f64 = (noise_variable_41).powf(noise_metadata_schedule_74_e1039);
            let noise_metadata_schedule_74_e1041: f64 = (params.p62 * noise_metadata_schedule_74_e1040);
            let noise_metadata_schedule_74_e1043: f64 = (-params.p118);
            let noise_metadata_schedule_74_e1046: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_74_e1047: f64 = (noise_metadata_schedule_74_e1043 * noise_metadata_schedule_74_e1046);
            let noise_metadata_schedule_74_e1050: f64 = (noise_variable_73 * params.p63);
            let noise_metadata_schedule_74_e1051: f64 = (noise_metadata_schedule_74_e1047 / noise_metadata_schedule_74_e1050);
            let noise_metadata_schedule_74_e1052: f64 = (noise_metadata_schedule_74_e1051).exp();
            let noise_metadata_schedule_74_e1053: f64 = (noise_metadata_schedule_74_e1041 * noise_metadata_schedule_74_e1052);
            noise_variable_7 = noise_metadata_schedule_74_e1053;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_75_e1056: f64 = if noise_variable_7 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_194 = noise_metadata_schedule_75_e1056;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_76_e1069,) = {
    if (noise_variable_194 != 0.0) {
        let noise_metadata_schedule_76_e1060: f64 = (params.p63 * noise_variable_73);
        let noise_metadata_schedule_76_e1064: f64 = (noise_variable_166 / noise_variable_7);
        let noise_metadata_schedule_76_e1065: f64 = (1.0 + noise_metadata_schedule_76_e1064);
        let noise_metadata_schedule_76_e1066: f64 = (noise_metadata_schedule_76_e1065).ln();
        let noise_metadata_schedule_76_e1067: f64 = (noise_metadata_schedule_76_e1060 * noise_metadata_schedule_76_e1066);
        (noise_metadata_schedule_76_e1067,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_76_e1069;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_77_e1074,) = {
    if (noise_variable_194 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_77_e1074;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_78_e1079: f64 = (params.p123 / params.p61);
            let noise_metadata_schedule_78_e1080: f64 = (noise_variable_41).powf(noise_metadata_schedule_78_e1079);
            let noise_metadata_schedule_78_e1081: f64 = (params.p64 * noise_metadata_schedule_78_e1080);
            let noise_metadata_schedule_78_e1083: f64 = (-params.p115);
            let noise_metadata_schedule_78_e1086: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_78_e1087: f64 = (noise_metadata_schedule_78_e1083 * noise_metadata_schedule_78_e1086);
            let noise_metadata_schedule_78_e1090: f64 = (noise_variable_73 * params.p61);
            let noise_metadata_schedule_78_e1091: f64 = (noise_metadata_schedule_78_e1087 / noise_metadata_schedule_78_e1090);
            let noise_metadata_schedule_78_e1092: f64 = (noise_metadata_schedule_78_e1091).exp();
            let noise_metadata_schedule_78_e1093: f64 = (noise_metadata_schedule_78_e1081 * noise_metadata_schedule_78_e1092);
            noise_variable_8 = noise_metadata_schedule_78_e1093;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_79_e1096: f64 = if noise_variable_8 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_195 = noise_metadata_schedule_79_e1096;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_80_e1109,) = {
    if (noise_variable_195 != 0.0) {
        let noise_metadata_schedule_80_e1100: f64 = (params.p61 * noise_variable_73);
        let noise_metadata_schedule_80_e1104: f64 = (noise_variable_166 / noise_variable_8);
        let noise_metadata_schedule_80_e1105: f64 = (1.0 + noise_metadata_schedule_80_e1104);
        let noise_metadata_schedule_80_e1106: f64 = (noise_metadata_schedule_80_e1105).ln();
        let noise_metadata_schedule_80_e1107: f64 = (noise_metadata_schedule_80_e1100 * noise_metadata_schedule_80_e1106);
        (noise_metadata_schedule_80_e1107,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_80_e1109;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_81_e1114,) = {
    if (noise_variable_195 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_81_e1114;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_82_e1119: f64 = (params.p124 / params.p63);
            let noise_metadata_schedule_82_e1120: f64 = (noise_variable_41).powf(noise_metadata_schedule_82_e1119);
            let noise_metadata_schedule_82_e1121: f64 = (params.p65 * noise_metadata_schedule_82_e1120);
            let noise_metadata_schedule_82_e1123: f64 = (-params.p118);
            let noise_metadata_schedule_82_e1126: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_82_e1127: f64 = (noise_metadata_schedule_82_e1123 * noise_metadata_schedule_82_e1126);
            let noise_metadata_schedule_82_e1130: f64 = (noise_variable_73 * params.p63);
            let noise_metadata_schedule_82_e1131: f64 = (noise_metadata_schedule_82_e1127 / noise_metadata_schedule_82_e1130);
            let noise_metadata_schedule_82_e1132: f64 = (noise_metadata_schedule_82_e1131).exp();
            let noise_metadata_schedule_82_e1133: f64 = (noise_metadata_schedule_82_e1121 * noise_metadata_schedule_82_e1132);
            noise_variable_9 = noise_metadata_schedule_82_e1133;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_83_e1136: f64 = if noise_variable_9 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_196 = noise_metadata_schedule_83_e1136;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_84_e1149,) = {
    if (noise_variable_196 != 0.0) {
        let noise_metadata_schedule_84_e1140: f64 = (params.p63 * noise_variable_73);
        let noise_metadata_schedule_84_e1144: f64 = (noise_variable_166 / noise_variable_9);
        let noise_metadata_schedule_84_e1145: f64 = (1.0 + noise_metadata_schedule_84_e1144);
        let noise_metadata_schedule_84_e1146: f64 = (noise_metadata_schedule_84_e1145).ln();
        let noise_metadata_schedule_84_e1147: f64 = (noise_metadata_schedule_84_e1140 * noise_metadata_schedule_84_e1146);
        (noise_metadata_schedule_84_e1147,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_84_e1149;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_85_e1154,) = {
    if (noise_variable_196 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_85_e1154;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            noise_variable_138 = (ctx.node_voltage(self.nodes[3]) - 0.0);
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let noise_metadata_schedule_95_e1236: f64 = ctx.temperature();
            let noise_metadata_schedule_95_e1238: f64 = (noise_metadata_schedule_95_e1236 + params.p0);
            let noise_metadata_schedule_95_e1240: f64 = (noise_metadata_schedule_95_e1238 + noise_variable_138);
            let noise_metadata_schedule_95_e1242: f64 = (noise_metadata_schedule_95_e1240 - 273.15);
            noise_variable_38 = noise_metadata_schedule_95_e1242;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let noise_metadata_schedule_96_e1246: f64 = (params.p14 + 1.0);
            let noise_metadata_schedule_96_e1247: f64 = if noise_variable_38 < noise_metadata_schedule_96_e1246 { 1.0 } else { 0.0 };
            noise_variable_199 = noise_metadata_schedule_96_e1247;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let (noise_metadata_schedule_97_e1258,) = {
    if (noise_variable_199 != 0.0) {
        let noise_metadata_schedule_97_e1252: f64 = (noise_variable_38 - params.p14);
        let noise_metadata_schedule_97_e1254: f64 = (noise_metadata_schedule_97_e1252 - 1.0);
        let noise_metadata_schedule_97_e1255: f64 = (noise_metadata_schedule_97_e1254).exp();
        let noise_metadata_schedule_97_e1256: f64 = (params.p14 + noise_metadata_schedule_97_e1255);
        (noise_metadata_schedule_97_e1256,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_97_e1258;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let noise_metadata_schedule_98_e1262: f64 = (params.p15 - 1.0);
            let noise_metadata_schedule_98_e1263: f64 = if noise_variable_38 > noise_metadata_schedule_98_e1262 { 1.0 } else { 0.0 };
            noise_variable_200 = noise_metadata_schedule_98_e1263;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let (noise_metadata_schedule_99_e1277,) = {
    if ((noise_variable_199 == 0.0) && (noise_variable_200 != 0.0)) {
        let noise_metadata_schedule_99_e1271: f64 = (params.p15 - noise_variable_38);
        let noise_metadata_schedule_99_e1273: f64 = (noise_metadata_schedule_99_e1271 - 1.0);
        let noise_metadata_schedule_99_e1274: f64 = (noise_metadata_schedule_99_e1273).exp();
        let noise_metadata_schedule_99_e1275: f64 = (params.p15 - noise_metadata_schedule_99_e1274);
        (noise_metadata_schedule_99_e1275,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_99_e1277;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let (noise_metadata_schedule_100_e1285,) = {
    if ((noise_variable_199 == 0.0) && (noise_variable_200 == 0.0)) {
        (noise_variable_38,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_100_e1285;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let noise_metadata_schedule_101_e1288: f64 = (noise_variable_38 + 273.15);
            noise_variable_39 = noise_metadata_schedule_101_e1288;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_102_e1291: f64 = (1.380662e-23 * noise_variable_39);
            let noise_metadata_schedule_102_e1293: f64 = (noise_metadata_schedule_102_e1291 / 1.602189e-19);
            noise_variable_73 = noise_metadata_schedule_102_e1293;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12) {
            let noise_metadata_schedule_103_e1296: f64 = (noise_variable_39 / noise_variable_40);
            noise_variable_41 = noise_metadata_schedule_103_e1296;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_104_e1299: f64 = (noise_variable_39 - noise_variable_40);
            noise_variable_42 = noise_metadata_schedule_104_e1299;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let noise_metadata_schedule_105_e1303: f64 = (noise_variable_41).powf(params.p126);
            let noise_metadata_schedule_105_e1304: f64 = (params.p72 * noise_metadata_schedule_105_e1303);
            noise_variable_2 = noise_metadata_schedule_105_e1304;
        }
        if matches!(source_index, 7) {
            let noise_metadata_schedule_106_e1306: f64 = if self.param_given[109] { 1.0 } else { 0.0 };
            noise_variable_201 = noise_metadata_schedule_106_e1306;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_107_e1314,) = {
    if (noise_variable_201 != 0.0) {
        let noise_metadata_schedule_107_e1311: f64 = (noise_variable_41).powf(params.p109);
        let noise_metadata_schedule_107_e1312: f64 = (params.p16 * noise_metadata_schedule_107_e1311);
        (noise_metadata_schedule_107_e1312,)
    } else {
        (noise_variable_12,)
    }
};
            noise_variable_12 = noise_metadata_schedule_107_e1314;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_108_e1323,) = {
    if (noise_variable_201 == 0.0) {
        let noise_metadata_schedule_108_e1320: f64 = (noise_variable_41).powf(params.p107);
        let noise_metadata_schedule_108_e1321: f64 = (params.p16 * noise_metadata_schedule_108_e1320);
        (noise_metadata_schedule_108_e1321,)
    } else {
        (noise_variable_12,)
    }
};
            noise_variable_12 = noise_metadata_schedule_108_e1323;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_109_e1325: f64 = if self.param_given[108] { 1.0 } else { 0.0 };
            noise_variable_202 = noise_metadata_schedule_109_e1325;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_110_e1333,) = {
    if (noise_variable_202 != 0.0) {
        let noise_metadata_schedule_110_e1330: f64 = (noise_variable_41).powf(params.p108);
        let noise_metadata_schedule_110_e1331: f64 = (params.p17 * noise_metadata_schedule_110_e1330);
        (noise_metadata_schedule_110_e1331,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_110_e1333;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_111_e1342,) = {
    if (noise_variable_202 == 0.0) {
        let noise_metadata_schedule_111_e1339: f64 = (noise_variable_41).powf(params.p107);
        let noise_metadata_schedule_111_e1340: f64 = (params.p17 * noise_metadata_schedule_111_e1339);
        (noise_metadata_schedule_111_e1340,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_111_e1342;
        }
        if matches!(source_index, 9) {
            let noise_metadata_schedule_112_e1344: f64 = if self.param_given[106] { 1.0 } else { 0.0 };
            noise_variable_203 = noise_metadata_schedule_112_e1344;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_113_e1352,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_113_e1349: f64 = (noise_variable_41).powf(params.p106);
        let noise_metadata_schedule_113_e1350: f64 = (params.p21 * noise_metadata_schedule_113_e1349);
        (noise_metadata_schedule_113_e1350,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_113_e1352;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_114_e1361,) = {
    if (noise_variable_203 == 0.0) {
        let noise_metadata_schedule_114_e1358: f64 = (noise_variable_41).powf(params.p104);
        let noise_metadata_schedule_114_e1359: f64 = (params.p21 * noise_metadata_schedule_114_e1358);
        (noise_metadata_schedule_114_e1359,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_114_e1361;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_115_e1363: f64 = if self.param_given[105] { 1.0 } else { 0.0 };
            noise_variable_204 = noise_metadata_schedule_115_e1363;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_116_e1371,) = {
    if (noise_variable_204 != 0.0) {
        let noise_metadata_schedule_116_e1368: f64 = (noise_variable_41).powf(params.p105);
        let noise_metadata_schedule_116_e1369: f64 = (params.p22 * noise_metadata_schedule_116_e1368);
        (noise_metadata_schedule_116_e1369,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_116_e1371;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_117_e1380,) = {
    if (noise_variable_204 == 0.0) {
        let noise_metadata_schedule_117_e1377: f64 = (noise_variable_41).powf(params.p104);
        let noise_metadata_schedule_117_e1378: f64 = (params.p22 * noise_metadata_schedule_117_e1377);
        (noise_metadata_schedule_117_e1378,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_117_e1380;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_118_e1384: f64 = (noise_variable_41).powf(params.p103);
            let noise_metadata_schedule_118_e1385: f64 = (params.p23 * noise_metadata_schedule_118_e1384);
            noise_variable_16 = noise_metadata_schedule_118_e1385;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_120_e1392: f64 = if self.param_given[110] { 1.0 } else { 0.0 };
            noise_variable_205 = noise_metadata_schedule_120_e1392;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_121_e1400,) = {
    if (noise_variable_205 != 0.0) {
        let noise_metadata_schedule_121_e1397: f64 = (noise_variable_41).powf(params.p110);
        let noise_metadata_schedule_121_e1398: f64 = (params.p25 * noise_metadata_schedule_121_e1397);
        (noise_metadata_schedule_121_e1398,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_121_e1400;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_122_e1409,) = {
    if (noise_variable_205 == 0.0) {
        let noise_metadata_schedule_122_e1406: f64 = (noise_variable_41).powf(params.p107);
        let noise_metadata_schedule_122_e1407: f64 = (params.p25 * noise_metadata_schedule_122_e1406);
        (noise_metadata_schedule_122_e1407,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_122_e1409;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let noise_metadata_schedule_124_e1421: f64 = (params.p122 / params.p28);
            let noise_metadata_schedule_124_e1422: f64 = (noise_variable_41).powf(noise_metadata_schedule_124_e1421);
            let noise_metadata_schedule_124_e1423: f64 = (params.p26 * noise_metadata_schedule_124_e1422);
            let noise_metadata_schedule_124_e1425: f64 = (-params.p113);
            let noise_metadata_schedule_124_e1428: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_124_e1429: f64 = (noise_metadata_schedule_124_e1425 * noise_metadata_schedule_124_e1428);
            let noise_metadata_schedule_124_e1432: f64 = (noise_variable_73 * params.p28);
            let noise_metadata_schedule_124_e1433: f64 = (noise_metadata_schedule_124_e1429 / noise_metadata_schedule_124_e1432);
            let noise_metadata_schedule_124_e1434: f64 = (noise_metadata_schedule_124_e1433).exp();
            let noise_metadata_schedule_124_e1435: f64 = (noise_metadata_schedule_124_e1423 * noise_metadata_schedule_124_e1434);
            noise_variable_0 = noise_metadata_schedule_124_e1435;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let noise_metadata_schedule_125_e1440: f64 = (params.p125 / params.p29);
            let noise_metadata_schedule_125_e1441: f64 = (noise_variable_41).powf(noise_metadata_schedule_125_e1440);
            let noise_metadata_schedule_125_e1442: f64 = (params.p27 * noise_metadata_schedule_125_e1441);
            let noise_metadata_schedule_125_e1444: f64 = (-params.p121);
            let noise_metadata_schedule_125_e1447: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_125_e1448: f64 = (noise_metadata_schedule_125_e1444 * noise_metadata_schedule_125_e1447);
            let noise_metadata_schedule_125_e1451: f64 = (noise_variable_73 * params.p29);
            let noise_metadata_schedule_125_e1452: f64 = (noise_metadata_schedule_125_e1448 / noise_metadata_schedule_125_e1451);
            let noise_metadata_schedule_125_e1453: f64 = (noise_metadata_schedule_125_e1452).exp();
            let noise_metadata_schedule_125_e1454: f64 = (noise_metadata_schedule_125_e1442 * noise_metadata_schedule_125_e1453);
            noise_variable_1 = noise_metadata_schedule_125_e1454;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_126_e1459: f64 = (params.p122 / params.p33);
            let noise_metadata_schedule_126_e1460: f64 = (noise_variable_41).powf(noise_metadata_schedule_126_e1459);
            let noise_metadata_schedule_126_e1461: f64 = (params.p31 * noise_metadata_schedule_126_e1460);
            let noise_metadata_schedule_126_e1463: f64 = (-params.p120);
            let noise_metadata_schedule_126_e1466: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_126_e1467: f64 = (noise_metadata_schedule_126_e1463 * noise_metadata_schedule_126_e1466);
            let noise_metadata_schedule_126_e1470: f64 = (noise_variable_73 * params.p33);
            let noise_metadata_schedule_126_e1471: f64 = (noise_metadata_schedule_126_e1467 / noise_metadata_schedule_126_e1470);
            let noise_metadata_schedule_126_e1472: f64 = (noise_metadata_schedule_126_e1471).exp();
            let noise_metadata_schedule_126_e1473: f64 = (noise_metadata_schedule_126_e1461 * noise_metadata_schedule_126_e1472);
            noise_variable_5 = noise_metadata_schedule_126_e1473;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_127_e1478: f64 = (params.p123 / params.p56);
            let noise_metadata_schedule_127_e1479: f64 = (noise_variable_41).powf(noise_metadata_schedule_127_e1478);
            let noise_metadata_schedule_127_e1480: f64 = (params.p54 * noise_metadata_schedule_127_e1479);
            let noise_metadata_schedule_127_e1482: f64 = (-params.p114);
            let noise_metadata_schedule_127_e1485: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_127_e1486: f64 = (noise_metadata_schedule_127_e1482 * noise_metadata_schedule_127_e1485);
            let noise_metadata_schedule_127_e1489: f64 = (noise_variable_73 * params.p56);
            let noise_metadata_schedule_127_e1490: f64 = (noise_metadata_schedule_127_e1486 / noise_metadata_schedule_127_e1489);
            let noise_metadata_schedule_127_e1491: f64 = (noise_metadata_schedule_127_e1490).exp();
            let noise_metadata_schedule_127_e1492: f64 = (noise_metadata_schedule_127_e1480 * noise_metadata_schedule_127_e1491);
            noise_variable_3 = noise_metadata_schedule_127_e1492;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_128_e1497: f64 = (params.p124 / params.p59);
            let noise_metadata_schedule_128_e1498: f64 = (noise_variable_41).powf(noise_metadata_schedule_128_e1497);
            let noise_metadata_schedule_128_e1499: f64 = (params.p58 * noise_metadata_schedule_128_e1498);
            let noise_metadata_schedule_128_e1501: f64 = (-params.p117);
            let noise_metadata_schedule_128_e1504: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_128_e1505: f64 = (noise_metadata_schedule_128_e1501 * noise_metadata_schedule_128_e1504);
            let noise_metadata_schedule_128_e1508: f64 = (noise_variable_73 * params.p59);
            let noise_metadata_schedule_128_e1509: f64 = (noise_metadata_schedule_128_e1505 / noise_metadata_schedule_128_e1508);
            let noise_metadata_schedule_128_e1510: f64 = (noise_metadata_schedule_128_e1509).exp();
            let noise_metadata_schedule_128_e1511: f64 = (noise_metadata_schedule_128_e1499 * noise_metadata_schedule_128_e1510);
            noise_variable_6 = noise_metadata_schedule_128_e1511;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_131_e1554: f64 = (params.p123 / params.p61);
            let noise_metadata_schedule_131_e1555: f64 = (noise_variable_41).powf(noise_metadata_schedule_131_e1554);
            let noise_metadata_schedule_131_e1556: f64 = (params.p64 * noise_metadata_schedule_131_e1555);
            let noise_metadata_schedule_131_e1558: f64 = (-params.p115);
            let noise_metadata_schedule_131_e1561: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_131_e1562: f64 = (noise_metadata_schedule_131_e1558 * noise_metadata_schedule_131_e1561);
            let noise_metadata_schedule_131_e1565: f64 = (noise_variable_73 * params.p61);
            let noise_metadata_schedule_131_e1566: f64 = (noise_metadata_schedule_131_e1562 / noise_metadata_schedule_131_e1565);
            let noise_metadata_schedule_131_e1567: f64 = (noise_metadata_schedule_131_e1566).exp();
            let noise_metadata_schedule_131_e1568: f64 = (noise_metadata_schedule_131_e1556 * noise_metadata_schedule_131_e1567);
            noise_variable_8 = noise_metadata_schedule_131_e1568;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_132_e1573: f64 = (params.p124 / params.p63);
            let noise_metadata_schedule_132_e1574: f64 = (noise_variable_41).powf(noise_metadata_schedule_132_e1573);
            let noise_metadata_schedule_132_e1575: f64 = (params.p65 * noise_metadata_schedule_132_e1574);
            let noise_metadata_schedule_132_e1577: f64 = (-params.p118);
            let noise_metadata_schedule_132_e1580: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_132_e1581: f64 = (noise_metadata_schedule_132_e1577 * noise_metadata_schedule_132_e1580);
            let noise_metadata_schedule_132_e1584: f64 = (noise_variable_73 * params.p63);
            let noise_metadata_schedule_132_e1585: f64 = (noise_metadata_schedule_132_e1581 / noise_metadata_schedule_132_e1584);
            let noise_metadata_schedule_132_e1586: f64 = (noise_metadata_schedule_132_e1585).exp();
            let noise_metadata_schedule_132_e1587: f64 = (noise_metadata_schedule_132_e1575 * noise_metadata_schedule_132_e1586);
            noise_variable_9 = noise_metadata_schedule_132_e1587;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_135_e1630: f64 = (noise_variable_42 * params.p129);
            let noise_metadata_schedule_135_e1631: f64 = (1.0 + noise_metadata_schedule_135_e1630);
            let noise_metadata_schedule_135_e1632: f64 = (params.p28 * noise_metadata_schedule_135_e1631);
            noise_variable_27 = noise_metadata_schedule_135_e1632;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_136_e1637: f64 = (noise_variable_42 * params.p129);
            let noise_metadata_schedule_136_e1638: f64 = (1.0 + noise_metadata_schedule_136_e1637);
            let noise_metadata_schedule_136_e1639: f64 = (params.p29 * noise_metadata_schedule_136_e1638);
            noise_variable_28 = noise_metadata_schedule_136_e1639;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_139_e1660: f64 = (noise_variable_42 * params.p92);
            let noise_metadata_schedule_139_e1661: f64 = (params.p91 + noise_metadata_schedule_139_e1660);
            let noise_metadata_schedule_139_e1662: f64 = (noise_variable_42 * noise_metadata_schedule_139_e1661);
            let noise_metadata_schedule_139_e1663: f64 = (1.0 + noise_metadata_schedule_139_e1662);
            let noise_metadata_schedule_139_e1664: f64 = (params.p88 * noise_metadata_schedule_139_e1663);
            noise_variable_31 = noise_metadata_schedule_139_e1664;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_140_e1669: f64 = (noise_variable_42 * params.p93);
            let noise_metadata_schedule_140_e1670: f64 = (1.0 + noise_metadata_schedule_140_e1669);
            let noise_metadata_schedule_140_e1671: f64 = (params.p89 * noise_metadata_schedule_140_e1670);
            noise_variable_32 = noise_metadata_schedule_140_e1671;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_141_e1675: f64 = (noise_variable_73 / noise_variable_41);
            let noise_metadata_schedule_141_e1676: f64 = (2.0 * noise_metadata_schedule_141_e1675);
            let noise_metadata_schedule_141_e1679: f64 = (0.5 * params.p37);
            let noise_metadata_schedule_141_e1681: f64 = (noise_metadata_schedule_141_e1679 * noise_variable_41);
            let noise_metadata_schedule_141_e1683: f64 = (noise_metadata_schedule_141_e1681 / noise_variable_73);
            let noise_metadata_schedule_141_e1684: f64 = (noise_metadata_schedule_141_e1683).exp();
            let noise_metadata_schedule_141_e1686: f64 = (-0.5);
            let noise_metadata_schedule_141_e1688: f64 = (noise_metadata_schedule_141_e1686 * params.p37);
            let noise_metadata_schedule_141_e1690: f64 = (noise_metadata_schedule_141_e1688 * noise_variable_41);
            let noise_metadata_schedule_141_e1692: f64 = (noise_metadata_schedule_141_e1690 / noise_variable_73);
            let noise_metadata_schedule_141_e1693: f64 = (noise_metadata_schedule_141_e1692).exp();
            let noise_metadata_schedule_141_e1694: f64 = (noise_metadata_schedule_141_e1684 - noise_metadata_schedule_141_e1693);
            let noise_metadata_schedule_141_e1695: f64 = (noise_metadata_schedule_141_e1694).ln();
            let noise_metadata_schedule_141_e1696: f64 = (noise_metadata_schedule_141_e1676 * noise_metadata_schedule_141_e1695);
            noise_variable_206 = noise_metadata_schedule_141_e1696;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_142_e1699: f64 = (noise_variable_206 * noise_variable_41);
            let noise_metadata_schedule_142_e1702: f64 = (3.0 * noise_variable_73);
            let noise_metadata_schedule_142_e1704: f64 = (noise_variable_41).ln();
            let noise_metadata_schedule_142_e1705: f64 = (noise_metadata_schedule_142_e1702 * noise_metadata_schedule_142_e1704);
            let noise_metadata_schedule_142_e1706: f64 = (noise_metadata_schedule_142_e1699 - noise_metadata_schedule_142_e1705);
            let noise_metadata_schedule_142_e1710: f64 = (noise_variable_41 - 1.0);
            let noise_metadata_schedule_142_e1711: f64 = (params.p114 * noise_metadata_schedule_142_e1710);
            let noise_metadata_schedule_142_e1712: f64 = (noise_metadata_schedule_142_e1706 - noise_metadata_schedule_142_e1711);
            noise_variable_207 = noise_metadata_schedule_142_e1712;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_143_e1716: f64 = (2.0 * noise_variable_73);
            let noise_metadata_schedule_143_e1722: f64 = (-noise_variable_207);
            let noise_metadata_schedule_143_e1724: f64 = (noise_metadata_schedule_143_e1722 / noise_variable_73);
            let noise_metadata_schedule_143_e1725: f64 = (noise_metadata_schedule_143_e1724).exp();
            let noise_metadata_schedule_143_e1726: f64 = (4.0 * noise_metadata_schedule_143_e1725);
            let noise_metadata_schedule_143_e1727: f64 = (1.0 + noise_metadata_schedule_143_e1726);
            let noise_metadata_schedule_143_e1728: f64 = (noise_metadata_schedule_143_e1727).sqrt();
            let noise_metadata_schedule_143_e1729: f64 = (1.0 + noise_metadata_schedule_143_e1728);
            let noise_metadata_schedule_143_e1730: f64 = (0.5 * noise_metadata_schedule_143_e1729);
            let noise_metadata_schedule_143_e1731: f64 = (noise_metadata_schedule_143_e1730).ln();
            let noise_metadata_schedule_143_e1732: f64 = (noise_metadata_schedule_143_e1716 * noise_metadata_schedule_143_e1731);
            let noise_metadata_schedule_143_e1733: f64 = (noise_variable_207 + noise_metadata_schedule_143_e1732);
            noise_variable_20 = noise_metadata_schedule_143_e1733;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_144_e1737: f64 = (noise_variable_73 / noise_variable_41);
            let noise_metadata_schedule_144_e1738: f64 = (2.0 * noise_metadata_schedule_144_e1737);
            let noise_metadata_schedule_144_e1741: f64 = (0.5 * params.p42);
            let noise_metadata_schedule_144_e1743: f64 = (noise_metadata_schedule_144_e1741 * noise_variable_41);
            let noise_metadata_schedule_144_e1745: f64 = (noise_metadata_schedule_144_e1743 / noise_variable_73);
            let noise_metadata_schedule_144_e1746: f64 = (noise_metadata_schedule_144_e1745).exp();
            let noise_metadata_schedule_144_e1748: f64 = (-0.5);
            let noise_metadata_schedule_144_e1750: f64 = (noise_metadata_schedule_144_e1748 * params.p42);
            let noise_metadata_schedule_144_e1752: f64 = (noise_metadata_schedule_144_e1750 * noise_variable_41);
            let noise_metadata_schedule_144_e1754: f64 = (noise_metadata_schedule_144_e1752 / noise_variable_73);
            let noise_metadata_schedule_144_e1755: f64 = (noise_metadata_schedule_144_e1754).exp();
            let noise_metadata_schedule_144_e1756: f64 = (noise_metadata_schedule_144_e1746 - noise_metadata_schedule_144_e1755);
            let noise_metadata_schedule_144_e1757: f64 = (noise_metadata_schedule_144_e1756).ln();
            let noise_metadata_schedule_144_e1758: f64 = (noise_metadata_schedule_144_e1738 * noise_metadata_schedule_144_e1757);
            noise_variable_208 = noise_metadata_schedule_144_e1758;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_145_e1761: f64 = (noise_variable_208 * noise_variable_41);
            let noise_metadata_schedule_145_e1764: f64 = (3.0 * noise_variable_73);
            let noise_metadata_schedule_145_e1766: f64 = (noise_variable_41).ln();
            let noise_metadata_schedule_145_e1767: f64 = (noise_metadata_schedule_145_e1764 * noise_metadata_schedule_145_e1766);
            let noise_metadata_schedule_145_e1768: f64 = (noise_metadata_schedule_145_e1761 - noise_metadata_schedule_145_e1767);
            let noise_metadata_schedule_145_e1772: f64 = (noise_variable_41 - 1.0);
            let noise_metadata_schedule_145_e1773: f64 = (params.p115 * noise_metadata_schedule_145_e1772);
            let noise_metadata_schedule_145_e1774: f64 = (noise_metadata_schedule_145_e1768 - noise_metadata_schedule_145_e1773);
            noise_variable_209 = noise_metadata_schedule_145_e1774;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_146_e1778: f64 = (2.0 * noise_variable_73);
            let noise_metadata_schedule_146_e1784: f64 = (-noise_variable_209);
            let noise_metadata_schedule_146_e1786: f64 = (noise_metadata_schedule_146_e1784 / noise_variable_73);
            let noise_metadata_schedule_146_e1787: f64 = (noise_metadata_schedule_146_e1786).exp();
            let noise_metadata_schedule_146_e1788: f64 = (4.0 * noise_metadata_schedule_146_e1787);
            let noise_metadata_schedule_146_e1789: f64 = (1.0 + noise_metadata_schedule_146_e1788);
            let noise_metadata_schedule_146_e1790: f64 = (noise_metadata_schedule_146_e1789).sqrt();
            let noise_metadata_schedule_146_e1791: f64 = (1.0 + noise_metadata_schedule_146_e1790);
            let noise_metadata_schedule_146_e1792: f64 = (0.5 * noise_metadata_schedule_146_e1791);
            let noise_metadata_schedule_146_e1793: f64 = (noise_metadata_schedule_146_e1792).ln();
            let noise_metadata_schedule_146_e1794: f64 = (noise_metadata_schedule_146_e1778 * noise_metadata_schedule_146_e1793);
            let noise_metadata_schedule_146_e1795: f64 = (noise_variable_209 + noise_metadata_schedule_146_e1794);
            noise_variable_21 = noise_metadata_schedule_146_e1795;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_154_e1889: f64 = (noise_variable_41).powf(params.p122);
            let noise_metadata_schedule_154_e1890: f64 = (params.p19 * noise_metadata_schedule_154_e1889);
            let noise_metadata_schedule_154_e1892: f64 = (-params.p113);
            let noise_metadata_schedule_154_e1895: f64 = (1.0 - noise_variable_41);
            let noise_metadata_schedule_154_e1896: f64 = (noise_metadata_schedule_154_e1892 * noise_metadata_schedule_154_e1895);
            let noise_metadata_schedule_154_e1898: f64 = (noise_metadata_schedule_154_e1896 / noise_variable_73);
            let noise_metadata_schedule_154_e1899: f64 = (noise_metadata_schedule_154_e1898).exp();
            let noise_metadata_schedule_154_e1900: f64 = (noise_metadata_schedule_154_e1890 * noise_metadata_schedule_154_e1899);
            noise_variable_33 = noise_metadata_schedule_154_e1900;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_155_e1904: f64 = (noise_variable_41).powf(params.p112);
            let noise_metadata_schedule_155_e1905: f64 = (params.p18 * noise_metadata_schedule_155_e1904);
            noise_variable_34 = noise_metadata_schedule_155_e1905;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_156_e1907: f64 = (-noise_variable_31);
            let noise_metadata_schedule_156_e1910: f64 = (noise_variable_32 * noise_variable_73);
            let noise_metadata_schedule_156_e1911: f64 = (noise_metadata_schedule_156_e1907 / noise_metadata_schedule_156_e1910);
            let noise_metadata_schedule_156_e1912: f64 = (noise_metadata_schedule_156_e1911).exp();
            noise_variable_35 = noise_metadata_schedule_156_e1912;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_157_e1917: f64 = (noise_variable_42 * params.p130);
            let noise_metadata_schedule_157_e1918: f64 = (1.0 + noise_metadata_schedule_157_e1917);
            let noise_metadata_schedule_157_e1919: f64 = (params.p70 * noise_metadata_schedule_157_e1918);
            noise_variable_36 = noise_metadata_schedule_157_e1919;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_158_e1924: f64 = (noise_variable_42 * params.p131);
            let noise_metadata_schedule_158_e1925: f64 = (1.0 + noise_metadata_schedule_158_e1924);
            let noise_metadata_schedule_158_e1926: f64 = (params.p71 * noise_metadata_schedule_158_e1925);
            noise_variable_37 = noise_metadata_schedule_158_e1926;
        }
        if matches!(source_index, 7) {
            let (noise_metadata_schedule_159_e1934,) = {
    if (noise_variable_12 > 0.001) {
        let noise_metadata_schedule_159_e1932: f64 = (1.0 / noise_variable_12);
        (noise_metadata_schedule_159_e1932,)
    } else {
        (1000.0,)
    }
};
            noise_variable_53 = noise_metadata_schedule_159_e1934;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_160_e1942,) = {
    if (noise_variable_13 > 0.001) {
        let noise_metadata_schedule_160_e1940: f64 = (1.0 / noise_variable_13);
        (noise_metadata_schedule_160_e1940,)
    } else {
        (1000.0,)
    }
};
            noise_variable_54 = noise_metadata_schedule_160_e1942;
        }
        if matches!(source_index, 9) {
            let (noise_metadata_schedule_161_e1950,) = {
    if (noise_variable_14 > 0.001) {
        let noise_metadata_schedule_161_e1948: f64 = (1.0 / noise_variable_14);
        (noise_metadata_schedule_161_e1948,)
    } else {
        (1000.0,)
    }
};
            noise_variable_55 = noise_metadata_schedule_161_e1950;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_162_e1958,) = {
    if (noise_variable_15 > 0.001) {
        let noise_metadata_schedule_162_e1956: f64 = (1.0 / noise_variable_15);
        (noise_metadata_schedule_162_e1956,)
    } else {
        (1000.0,)
    }
};
            noise_variable_56 = noise_metadata_schedule_162_e1958;
        }
        if matches!(source_index, 11) {
            let (noise_metadata_schedule_163_e1966,) = {
    if (noise_variable_16 > 0.001) {
        let noise_metadata_schedule_163_e1964: f64 = (1.0 / noise_variable_16);
        (noise_metadata_schedule_163_e1964,)
    } else {
        (1000.0,)
    }
};
            noise_variable_57 = noise_metadata_schedule_163_e1966;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_164_e1974,) = {
    if (noise_variable_18 > 0.001) {
        let noise_metadata_schedule_164_e1972: f64 = (1.0 / noise_variable_18);
        (noise_metadata_schedule_164_e1972,)
    } else {
        (1000.0,)
    }
};
            noise_variable_58 = noise_metadata_schedule_164_e1974;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_167_e1998,) = {
    if (noise_variable_36 > 0.0) {
        let noise_metadata_schedule_167_e1996: f64 = (1.0 / noise_variable_36);
        (noise_metadata_schedule_167_e1996,)
    } else {
        (0.0,)
    }
};
            noise_variable_43 = noise_metadata_schedule_167_e1998;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_168_e2006,) = {
    if (noise_variable_37 > 0.0) {
        let noise_metadata_schedule_168_e2004: f64 = (1.0 / noise_variable_37);
        (noise_metadata_schedule_168_e2004,)
    } else {
        (0.0,)
    }
};
            noise_variable_44 = noise_metadata_schedule_168_e2006;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let (noise_metadata_schedule_169_e2014,) = {
    if (noise_variable_2 > 0.0) {
        let noise_metadata_schedule_169_e2012: f64 = (1.0 / noise_variable_2);
        (noise_metadata_schedule_169_e2012,)
    } else {
        (0.0,)
    }
};
            noise_variable_45 = noise_metadata_schedule_169_e2014;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_170_e2022,) = {
    if (noise_variable_34 > 0.0) {
        let noise_metadata_schedule_170_e2020: f64 = (1.0 / noise_variable_34);
        (noise_metadata_schedule_170_e2020,)
    } else {
        (0.0,)
    }
};
            noise_variable_48 = noise_metadata_schedule_170_e2022;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_171_e2025: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_143 = noise_metadata_schedule_171_e2025;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_172_e2028: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[8])));
            noise_variable_145 = noise_metadata_schedule_172_e2028;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_173_e2031: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_144 = noise_metadata_schedule_173_e2031;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_174_e2034: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_148 = noise_metadata_schedule_174_e2034;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let noise_metadata_schedule_176_e2040: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[9])));
            noise_variable_146 = noise_metadata_schedule_176_e2040;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_181_e2049: f64 = (noise_variable_162 * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_154 = noise_metadata_schedule_181_e2049;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_188_e2057: f64 = (-noise_variable_20);
            let noise_metadata_schedule_188_e2059: f64 = (noise_metadata_schedule_188_e2057 * params.p34);
            noise_variable_212 = noise_metadata_schedule_188_e2059;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_189_e2062: f64 = if params.p39 <= 0.0 { 1.0 } else { 0.0 };
            noise_variable_223 = noise_metadata_schedule_189_e2062;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_190_e2068,) = {
    if (noise_variable_223 != 0.0) {
        let noise_metadata_schedule_190_e2066: f64 = (noise_variable_143 + noise_variable_212);
        (noise_metadata_schedule_190_e2066,)
    } else {
        (noise_variable_213,)
    }
};
            noise_variable_213 = noise_metadata_schedule_190_e2068;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_191_e2071: f64 = if noise_variable_213 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_224 = noise_metadata_schedule_191_e2071;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_192_e2082,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_192_e2077: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_192_e2079: f64 = (-params.p38);
        let noise_metadata_schedule_192_e2080: f64 = (noise_metadata_schedule_192_e2077).powf(noise_metadata_schedule_192_e2079);
        (noise_metadata_schedule_192_e2080,)
    } else {
        (noise_variable_214,)
    }
};
            noise_variable_214 = noise_metadata_schedule_192_e2082;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_193_e2100,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_193_e2091: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_193_e2092: f64 = (noise_variable_214 * noise_metadata_schedule_193_e2091);
        let noise_metadata_schedule_193_e2093: f64 = (1.0 - noise_metadata_schedule_193_e2092);
        let noise_metadata_schedule_193_e2094: f64 = (noise_variable_20 * noise_metadata_schedule_193_e2093);
        let noise_metadata_schedule_193_e2097: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_193_e2098: f64 = (noise_metadata_schedule_193_e2094 / noise_metadata_schedule_193_e2097);
        (noise_metadata_schedule_193_e2098,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_193_e2100;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_194_e2122,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_194_e2108: f64 = (0.5 * params.p38);
        let noise_metadata_schedule_194_e2110: f64 = (noise_metadata_schedule_194_e2108 * noise_variable_213);
        let noise_metadata_schedule_194_e2114: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_194_e2115: f64 = (noise_variable_20 * noise_metadata_schedule_194_e2114);
        let noise_metadata_schedule_194_e2116: f64 = (noise_metadata_schedule_194_e2110 / noise_metadata_schedule_194_e2115);
        let noise_metadata_schedule_194_e2117: f64 = (1.0 + noise_metadata_schedule_194_e2116);
        let noise_metadata_schedule_194_e2118: f64 = (noise_variable_213 * noise_metadata_schedule_194_e2117);
        let noise_metadata_schedule_194_e2120: f64 = (noise_metadata_schedule_194_e2118 * noise_variable_214);
        (noise_metadata_schedule_194_e2120,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_194_e2122;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_195_e2145,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 == 0.0)) {
        let noise_metadata_schedule_195_e2132: f64 = (noise_variable_143 / noise_variable_20);
        let noise_metadata_schedule_195_e2133: f64 = (1.0 - noise_metadata_schedule_195_e2132);
        let noise_metadata_schedule_195_e2136: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_195_e2137: f64 = (noise_metadata_schedule_195_e2133).powf(noise_metadata_schedule_195_e2136);
        let noise_metadata_schedule_195_e2138: f64 = (1.0 - noise_metadata_schedule_195_e2137);
        let noise_metadata_schedule_195_e2139: f64 = (noise_variable_20 * noise_metadata_schedule_195_e2138);
        let noise_metadata_schedule_195_e2142: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_195_e2143: f64 = (noise_metadata_schedule_195_e2139 / noise_metadata_schedule_195_e2142);
        (noise_metadata_schedule_195_e2143,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_195_e2145;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_196_e2152,) = {
    if ((noise_variable_223 != 0.0) && (noise_variable_224 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_216,)
    }
};
            noise_variable_216 = noise_metadata_schedule_196_e2152;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_197_e2158,) = {
    if (noise_variable_223 != 0.0) {
        let noise_metadata_schedule_197_e2156: f64 = (noise_variable_215 + noise_variable_216);
        (noise_metadata_schedule_197_e2156,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_197_e2158;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_198_e2172,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_198_e2163: f64 = (noise_variable_212 * noise_variable_212);
        let noise_metadata_schedule_198_e2166: f64 = (4.0 * params.p39);
        let noise_metadata_schedule_198_e2168: f64 = (noise_metadata_schedule_198_e2166 * params.p39);
        let noise_metadata_schedule_198_e2169: f64 = (noise_metadata_schedule_198_e2163 + noise_metadata_schedule_198_e2168);
        let noise_metadata_schedule_198_e2170: f64 = (noise_metadata_schedule_198_e2169).sqrt();
        (noise_metadata_schedule_198_e2170,)
    } else {
        (noise_variable_217,)
    }
};
            noise_variable_217 = noise_metadata_schedule_198_e2172;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_199_e2182,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_199_e2176: f64 = (-0.5);
        let noise_metadata_schedule_199_e2179: f64 = (noise_variable_212 + noise_variable_217);
        let noise_metadata_schedule_199_e2180: f64 = (noise_metadata_schedule_199_e2176 * noise_metadata_schedule_199_e2179);
        (noise_metadata_schedule_199_e2180,)
    } else {
        (noise_variable_218,)
    }
};
            noise_variable_218 = noise_metadata_schedule_199_e2182;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_200_e2202,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_200_e2186: f64 = (-noise_variable_20);
        let noise_metadata_schedule_200_e2190: f64 = (noise_variable_218 / noise_variable_20);
        let noise_metadata_schedule_200_e2191: f64 = (1.0 - noise_metadata_schedule_200_e2190);
        let noise_metadata_schedule_200_e2194: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_200_e2195: f64 = (noise_metadata_schedule_200_e2191).powf(noise_metadata_schedule_200_e2194);
        let noise_metadata_schedule_200_e2196: f64 = (noise_metadata_schedule_200_e2186 * noise_metadata_schedule_200_e2195);
        let noise_metadata_schedule_200_e2199: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_200_e2200: f64 = (noise_metadata_schedule_200_e2196 / noise_metadata_schedule_200_e2199);
        (noise_metadata_schedule_200_e2200,)
    } else {
        (noise_variable_219,)
    }
};
            noise_variable_219 = noise_metadata_schedule_200_e2202;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_201_e2209,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_201_e2207: f64 = (noise_variable_143 + noise_variable_212);
        (noise_metadata_schedule_201_e2207,)
    } else {
        (noise_variable_220,)
    }
};
            noise_variable_220 = noise_metadata_schedule_201_e2209;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_202_e2223,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_202_e2214: f64 = (noise_variable_220 * noise_variable_220);
        let noise_metadata_schedule_202_e2217: f64 = (4.0 * params.p39);
        let noise_metadata_schedule_202_e2219: f64 = (noise_metadata_schedule_202_e2217 * params.p39);
        let noise_metadata_schedule_202_e2220: f64 = (noise_metadata_schedule_202_e2214 + noise_metadata_schedule_202_e2219);
        let noise_metadata_schedule_202_e2221: f64 = (noise_metadata_schedule_202_e2220).sqrt();
        (noise_metadata_schedule_202_e2221,)
    } else {
        (noise_variable_221,)
    }
};
            noise_variable_221 = noise_metadata_schedule_202_e2223;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_203_e2234,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_203_e2229: f64 = (noise_variable_220 - noise_variable_221);
        let noise_metadata_schedule_203_e2230: f64 = (0.5 * noise_metadata_schedule_203_e2229);
        let noise_metadata_schedule_203_e2232: f64 = (noise_metadata_schedule_203_e2230 - noise_variable_212);
        (noise_metadata_schedule_203_e2232,)
    } else {
        (noise_variable_222,)
    }
};
            noise_variable_222 = noise_metadata_schedule_203_e2234;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_204_e2254,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_204_e2238: f64 = (-noise_variable_20);
        let noise_metadata_schedule_204_e2242: f64 = (noise_variable_222 / noise_variable_20);
        let noise_metadata_schedule_204_e2243: f64 = (1.0 - noise_metadata_schedule_204_e2242);
        let noise_metadata_schedule_204_e2246: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_204_e2247: f64 = (noise_metadata_schedule_204_e2243).powf(noise_metadata_schedule_204_e2246);
        let noise_metadata_schedule_204_e2248: f64 = (noise_metadata_schedule_204_e2238 * noise_metadata_schedule_204_e2247);
        let noise_metadata_schedule_204_e2251: f64 = (1.0 - params.p38);
        let noise_metadata_schedule_204_e2252: f64 = (noise_metadata_schedule_204_e2248 / noise_metadata_schedule_204_e2251);
        (noise_metadata_schedule_204_e2252,)
    } else {
        (noise_variable_215,)
    }
};
            noise_variable_215 = noise_metadata_schedule_204_e2254;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_205_e2292,) = {
    if (noise_variable_223 == 0.0) {
        let noise_metadata_schedule_205_e2260: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_205_e2262: f64 = (-params.p38);
        let noise_metadata_schedule_205_e2263: f64 = (noise_metadata_schedule_205_e2260).powf(noise_metadata_schedule_205_e2262);
        let noise_metadata_schedule_205_e2266: f64 = (noise_variable_143 - noise_variable_222);
        let noise_metadata_schedule_205_e2268: f64 = (noise_metadata_schedule_205_e2266 + noise_variable_218);
        let noise_metadata_schedule_205_e2269: f64 = (noise_metadata_schedule_205_e2263 * noise_metadata_schedule_205_e2268);
        let noise_metadata_schedule_205_e2273: f64 = (0.5 * params.p38);
        let noise_metadata_schedule_205_e2276: f64 = (noise_variable_143 - noise_variable_222);
        let noise_metadata_schedule_205_e2278: f64 = (noise_metadata_schedule_205_e2276 + noise_variable_218);
        let noise_metadata_schedule_205_e2279: f64 = (noise_metadata_schedule_205_e2273 * noise_metadata_schedule_205_e2278);
        let noise_metadata_schedule_205_e2283: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_205_e2284: f64 = (noise_variable_20 * noise_metadata_schedule_205_e2283);
        let noise_metadata_schedule_205_e2285: f64 = (noise_metadata_schedule_205_e2279 / noise_metadata_schedule_205_e2284);
        let noise_metadata_schedule_205_e2286: f64 = (1.0 + noise_metadata_schedule_205_e2285);
        let noise_metadata_schedule_205_e2287: f64 = (noise_metadata_schedule_205_e2269 * noise_metadata_schedule_205_e2286);
        let noise_metadata_schedule_205_e2288: f64 = (noise_variable_215 + noise_metadata_schedule_205_e2287);
        let noise_metadata_schedule_205_e2290: f64 = (noise_metadata_schedule_205_e2288 - noise_variable_219);
        (noise_metadata_schedule_205_e2290,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_205_e2292;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_206_e2294: f64 = (-noise_variable_21);
            let noise_metadata_schedule_206_e2296: f64 = (noise_metadata_schedule_206_e2294 * params.p34);
            noise_variable_225 = noise_metadata_schedule_206_e2296;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_207_e2299: f64 = if params.p44 <= 0.0 { 1.0 } else { 0.0 };
            noise_variable_246 = noise_metadata_schedule_207_e2299;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_208_e2305,) = {
    if (noise_variable_246 != 0.0) {
        let noise_metadata_schedule_208_e2303: f64 = (noise_variable_144 + noise_variable_225);
        (noise_metadata_schedule_208_e2303,)
    } else {
        (noise_variable_226,)
    }
};
            noise_variable_226 = noise_metadata_schedule_208_e2305;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_209_e2308: f64 = if noise_variable_226 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_247 = noise_metadata_schedule_209_e2308;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_210_e2321,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_210_e2314: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_210_e2316: f64 = (-1.0);
        let noise_metadata_schedule_210_e2318: f64 = (noise_metadata_schedule_210_e2316 - params.p43);
        let noise_metadata_schedule_210_e2319: f64 = (noise_metadata_schedule_210_e2314).powf(noise_metadata_schedule_210_e2318);
        (noise_metadata_schedule_210_e2319,)
    } else {
        (noise_variable_227,)
    }
};
            noise_variable_227 = noise_metadata_schedule_210_e2321;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_211_e2343,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_211_e2330: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_211_e2331: f64 = (noise_variable_227 * noise_metadata_schedule_211_e2330);
        let noise_metadata_schedule_211_e2334: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_211_e2335: f64 = (noise_metadata_schedule_211_e2331 * noise_metadata_schedule_211_e2334);
        let noise_metadata_schedule_211_e2336: f64 = (1.0 - noise_metadata_schedule_211_e2335);
        let noise_metadata_schedule_211_e2337: f64 = (noise_variable_21 * noise_metadata_schedule_211_e2336);
        let noise_metadata_schedule_211_e2340: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_211_e2341: f64 = (noise_metadata_schedule_211_e2337 / noise_metadata_schedule_211_e2340);
        (noise_metadata_schedule_211_e2341,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_211_e2343;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_212_e2363,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_212_e2350: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_212_e2353: f64 = (0.5 * params.p43);
        let noise_metadata_schedule_212_e2355: f64 = (noise_metadata_schedule_212_e2353 * noise_variable_226);
        let noise_metadata_schedule_212_e2357: f64 = (noise_metadata_schedule_212_e2355 / noise_variable_21);
        let noise_metadata_schedule_212_e2358: f64 = (noise_metadata_schedule_212_e2350 + noise_metadata_schedule_212_e2357);
        let noise_metadata_schedule_212_e2359: f64 = (noise_variable_226 * noise_metadata_schedule_212_e2358);
        let noise_metadata_schedule_212_e2361: f64 = (noise_metadata_schedule_212_e2359 * noise_variable_227);
        (noise_metadata_schedule_212_e2361,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_212_e2363;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_213_e2369: f64 = (-params.p45);
            let noise_metadata_schedule_213_e2371: f64 = if ((params.p45 > 0.0) && (noise_variable_144 < noise_metadata_schedule_213_e2369)) { 1.0 } else { 0.0 };
            noise_variable_248 = noise_metadata_schedule_213_e2371;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_214_e2410,) = {
    if (((noise_variable_246 != 0.0) && (noise_variable_247 == 0.0)) && (noise_variable_248 != 0.0)) {
        let noise_metadata_schedule_214_e2383: f64 = (params.p45 / noise_variable_21);
        let noise_metadata_schedule_214_e2384: f64 = (1.0 + noise_metadata_schedule_214_e2383);
        let noise_metadata_schedule_214_e2387: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_214_e2388: f64 = (noise_metadata_schedule_214_e2384).powf(noise_metadata_schedule_214_e2387);
        let noise_metadata_schedule_214_e2392: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_214_e2395: f64 = (noise_variable_144 + params.p45);
        let noise_metadata_schedule_214_e2396: f64 = (noise_metadata_schedule_214_e2392 * noise_metadata_schedule_214_e2395);
        let noise_metadata_schedule_214_e2399: f64 = (noise_variable_21 + params.p45);
        let noise_metadata_schedule_214_e2400: f64 = (noise_metadata_schedule_214_e2396 / noise_metadata_schedule_214_e2399);
        let noise_metadata_schedule_214_e2401: f64 = (1.0 - noise_metadata_schedule_214_e2400);
        let noise_metadata_schedule_214_e2402: f64 = (noise_metadata_schedule_214_e2388 * noise_metadata_schedule_214_e2401);
        let noise_metadata_schedule_214_e2403: f64 = (1.0 - noise_metadata_schedule_214_e2402);
        let noise_metadata_schedule_214_e2404: f64 = (noise_variable_21 * noise_metadata_schedule_214_e2403);
        let noise_metadata_schedule_214_e2407: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_214_e2408: f64 = (noise_metadata_schedule_214_e2404 / noise_metadata_schedule_214_e2407);
        (noise_metadata_schedule_214_e2408,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_214_e2410;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_215_e2436,) = {
    if (((noise_variable_246 != 0.0) && (noise_variable_247 == 0.0)) && (noise_variable_248 == 0.0)) {
        let noise_metadata_schedule_215_e2423: f64 = (noise_variable_144 / noise_variable_21);
        let noise_metadata_schedule_215_e2424: f64 = (1.0 - noise_metadata_schedule_215_e2423);
        let noise_metadata_schedule_215_e2427: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_215_e2428: f64 = (noise_metadata_schedule_215_e2424).powf(noise_metadata_schedule_215_e2427);
        let noise_metadata_schedule_215_e2429: f64 = (1.0 - noise_metadata_schedule_215_e2428);
        let noise_metadata_schedule_215_e2430: f64 = (noise_variable_21 * noise_metadata_schedule_215_e2429);
        let noise_metadata_schedule_215_e2433: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_215_e2434: f64 = (noise_metadata_schedule_215_e2430 / noise_metadata_schedule_215_e2433);
        (noise_metadata_schedule_215_e2434,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_215_e2436;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_216_e2443,) = {
    if ((noise_variable_246 != 0.0) && (noise_variable_247 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_216_e2443;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_217_e2449,) = {
    if (noise_variable_246 != 0.0) {
        let noise_metadata_schedule_217_e2447: f64 = (noise_variable_228 + noise_variable_229);
        (noise_metadata_schedule_217_e2447,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_217_e2449;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_218_e2456: f64 = if ((params.p45 > 0.0) && (params.p46 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_249 = noise_metadata_schedule_218_e2456;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_219_e2469,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_219_e2463: f64 = (params.p45 + noise_variable_225);
        let noise_metadata_schedule_219_e2466: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_219_e2467: f64 = (noise_metadata_schedule_219_e2463 / noise_metadata_schedule_219_e2466);
        (noise_metadata_schedule_219_e2467,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_219_e2469;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_220_e2508,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_220_e2476: f64 = (2.0 * noise_variable_230);
        let noise_metadata_schedule_220_e2479: f64 = (noise_variable_230 - 1.0);
        let noise_metadata_schedule_220_e2482: f64 = (noise_variable_230 - 1.0);
        let noise_metadata_schedule_220_e2483: f64 = (noise_metadata_schedule_220_e2479 * noise_metadata_schedule_220_e2482);
        let noise_metadata_schedule_220_e2486: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_220_e2488: f64 = (noise_metadata_schedule_220_e2486 * params.p44);
        let noise_metadata_schedule_220_e2489: f64 = (noise_metadata_schedule_220_e2483 + noise_metadata_schedule_220_e2488);
        let noise_metadata_schedule_220_e2490: f64 = (noise_metadata_schedule_220_e2489).sqrt();
        let noise_metadata_schedule_220_e2493: f64 = (noise_variable_230 + 1.0);
        let noise_metadata_schedule_220_e2496: f64 = (noise_variable_230 + 1.0);
        let noise_metadata_schedule_220_e2497: f64 = (noise_metadata_schedule_220_e2493 * noise_metadata_schedule_220_e2496);
        let noise_metadata_schedule_220_e2500: f64 = (4.0 * params.p46);
        let noise_metadata_schedule_220_e2502: f64 = (noise_metadata_schedule_220_e2500 * params.p46);
        let noise_metadata_schedule_220_e2503: f64 = (noise_metadata_schedule_220_e2497 + noise_metadata_schedule_220_e2502);
        let noise_metadata_schedule_220_e2504: f64 = (noise_metadata_schedule_220_e2503).sqrt();
        let noise_metadata_schedule_220_e2505: f64 = (noise_metadata_schedule_220_e2490 + noise_metadata_schedule_220_e2504);
        let noise_metadata_schedule_220_e2506: f64 = (noise_metadata_schedule_220_e2476 / noise_metadata_schedule_220_e2505);
        (noise_metadata_schedule_220_e2506,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_220_e2508;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_221_e2525,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_221_e2517: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_221_e2518: f64 = (noise_variable_231 * noise_metadata_schedule_221_e2517);
        let noise_metadata_schedule_221_e2520: f64 = (noise_metadata_schedule_221_e2518 - params.p45);
        let noise_metadata_schedule_221_e2522: f64 = (noise_metadata_schedule_221_e2520 - noise_variable_225);
        let noise_metadata_schedule_221_e2523: f64 = (0.5 * noise_metadata_schedule_221_e2522);
        (noise_metadata_schedule_221_e2523,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_221_e2525;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_222_e2548,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_222_e2535: f64 = (noise_variable_232 / noise_variable_21);
        let noise_metadata_schedule_222_e2536: f64 = (1.0 - noise_metadata_schedule_222_e2535);
        let noise_metadata_schedule_222_e2539: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_222_e2540: f64 = (noise_metadata_schedule_222_e2536).powf(noise_metadata_schedule_222_e2539);
        let noise_metadata_schedule_222_e2541: f64 = (1.0 - noise_metadata_schedule_222_e2540);
        let noise_metadata_schedule_222_e2542: f64 = (noise_variable_21 * noise_metadata_schedule_222_e2541);
        let noise_metadata_schedule_222_e2545: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_222_e2546: f64 = (noise_metadata_schedule_222_e2542 / noise_metadata_schedule_222_e2545);
        (noise_metadata_schedule_222_e2546,)
    } else {
        (noise_variable_233,)
    }
};
            noise_variable_233 = noise_metadata_schedule_222_e2548;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_223_e2565,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_223_e2555: f64 = (2.0 * noise_variable_144);
        let noise_metadata_schedule_223_e2557: f64 = (noise_metadata_schedule_223_e2555 + params.p45);
        let noise_metadata_schedule_223_e2559: f64 = (noise_metadata_schedule_223_e2557 + noise_variable_225);
        let noise_metadata_schedule_223_e2562: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_223_e2563: f64 = (noise_metadata_schedule_223_e2559 / noise_metadata_schedule_223_e2562);
        (noise_metadata_schedule_223_e2563,)
    } else {
        (noise_variable_234,)
    }
};
            noise_variable_234 = noise_metadata_schedule_223_e2565;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_224_e2604,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_224_e2572: f64 = (2.0 * noise_variable_234);
        let noise_metadata_schedule_224_e2575: f64 = (noise_variable_234 - 1.0);
        let noise_metadata_schedule_224_e2578: f64 = (noise_variable_234 - 1.0);
        let noise_metadata_schedule_224_e2579: f64 = (noise_metadata_schedule_224_e2575 * noise_metadata_schedule_224_e2578);
        let noise_metadata_schedule_224_e2582: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_224_e2584: f64 = (noise_metadata_schedule_224_e2582 * params.p44);
        let noise_metadata_schedule_224_e2585: f64 = (noise_metadata_schedule_224_e2579 + noise_metadata_schedule_224_e2584);
        let noise_metadata_schedule_224_e2586: f64 = (noise_metadata_schedule_224_e2585).sqrt();
        let noise_metadata_schedule_224_e2589: f64 = (noise_variable_234 + 1.0);
        let noise_metadata_schedule_224_e2592: f64 = (noise_variable_234 + 1.0);
        let noise_metadata_schedule_224_e2593: f64 = (noise_metadata_schedule_224_e2589 * noise_metadata_schedule_224_e2592);
        let noise_metadata_schedule_224_e2596: f64 = (4.0 * params.p46);
        let noise_metadata_schedule_224_e2598: f64 = (noise_metadata_schedule_224_e2596 * params.p46);
        let noise_metadata_schedule_224_e2599: f64 = (noise_metadata_schedule_224_e2593 + noise_metadata_schedule_224_e2598);
        let noise_metadata_schedule_224_e2600: f64 = (noise_metadata_schedule_224_e2599).sqrt();
        let noise_metadata_schedule_224_e2601: f64 = (noise_metadata_schedule_224_e2586 + noise_metadata_schedule_224_e2600);
        let noise_metadata_schedule_224_e2602: f64 = (noise_metadata_schedule_224_e2572 / noise_metadata_schedule_224_e2601);
        (noise_metadata_schedule_224_e2602,)
    } else {
        (noise_variable_235,)
    }
};
            noise_variable_235 = noise_metadata_schedule_224_e2604;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_225_e2621,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_225_e2613: f64 = (params.p45 - noise_variable_225);
        let noise_metadata_schedule_225_e2614: f64 = (noise_variable_235 * noise_metadata_schedule_225_e2613);
        let noise_metadata_schedule_225_e2616: f64 = (noise_metadata_schedule_225_e2614 - params.p45);
        let noise_metadata_schedule_225_e2618: f64 = (noise_metadata_schedule_225_e2616 - noise_variable_225);
        let noise_metadata_schedule_225_e2619: f64 = (0.5 * noise_metadata_schedule_225_e2618);
        (noise_metadata_schedule_225_e2619,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_225_e2621;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_226_e2644,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_226_e2631: f64 = (noise_variable_236 / noise_variable_21);
        let noise_metadata_schedule_226_e2632: f64 = (1.0 - noise_metadata_schedule_226_e2631);
        let noise_metadata_schedule_226_e2635: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_226_e2636: f64 = (noise_metadata_schedule_226_e2632).powf(noise_metadata_schedule_226_e2635);
        let noise_metadata_schedule_226_e2637: f64 = (1.0 - noise_metadata_schedule_226_e2636);
        let noise_metadata_schedule_226_e2638: f64 = (noise_variable_21 * noise_metadata_schedule_226_e2637);
        let noise_metadata_schedule_226_e2641: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_226_e2642: f64 = (noise_metadata_schedule_226_e2638 / noise_metadata_schedule_226_e2641);
        (noise_metadata_schedule_226_e2642,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_226_e2644;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_227_e2655,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_227_e2652: f64 = (noise_variable_235 + 1.0);
        let noise_metadata_schedule_227_e2653: f64 = (0.5 * noise_metadata_schedule_227_e2652);
        (noise_metadata_schedule_227_e2653,)
    } else {
        (noise_variable_237,)
    }
};
            noise_variable_237 = noise_metadata_schedule_227_e2655;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_228_e2669,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_228_e2663: f64 = (params.p45 / noise_variable_21);
        let noise_metadata_schedule_228_e2664: f64 = (1.0 + noise_metadata_schedule_228_e2663);
        let noise_metadata_schedule_228_e2666: f64 = (-params.p43);
        let noise_metadata_schedule_228_e2667: f64 = (noise_metadata_schedule_228_e2664).powf(noise_metadata_schedule_228_e2666);
        (noise_metadata_schedule_228_e2667,)
    } else {
        (noise_variable_238,)
    }
};
            noise_variable_238 = noise_metadata_schedule_228_e2669;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_229_e2683,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_229_e2677: f64 = (noise_variable_225 / noise_variable_21);
        let noise_metadata_schedule_229_e2678: f64 = (1.0 + noise_metadata_schedule_229_e2677);
        let noise_metadata_schedule_229_e2680: f64 = (-params.p43);
        let noise_metadata_schedule_229_e2681: f64 = (noise_metadata_schedule_229_e2678).powf(noise_metadata_schedule_229_e2680);
        (noise_metadata_schedule_229_e2681,)
    } else {
        (noise_variable_239,)
    }
};
            noise_variable_239 = noise_metadata_schedule_229_e2683;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_230_e2698,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_230_e2690: f64 = (1.0 - noise_variable_237);
        let noise_metadata_schedule_230_e2692: f64 = (noise_metadata_schedule_230_e2690 * noise_variable_238);
        let noise_metadata_schedule_230_e2695: f64 = (noise_variable_237 * noise_variable_239);
        let noise_metadata_schedule_230_e2696: f64 = (noise_metadata_schedule_230_e2692 + noise_metadata_schedule_230_e2695);
        (noise_metadata_schedule_230_e2696,)
    } else {
        (noise_variable_240,)
    }
};
            noise_variable_240 = noise_metadata_schedule_230_e2698;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_231_e2711,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_231_e2705: f64 = (noise_variable_144 - noise_variable_236);
        let noise_metadata_schedule_231_e2707: f64 = (noise_metadata_schedule_231_e2705 + noise_variable_232);
        let noise_metadata_schedule_231_e2709: f64 = (noise_metadata_schedule_231_e2707 * noise_variable_240);
        (noise_metadata_schedule_231_e2709,)
    } else {
        (noise_variable_241,)
    }
};
            noise_variable_241 = noise_metadata_schedule_231_e2711;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_232_e2722,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 != 0.0)) {
        let noise_metadata_schedule_232_e2718: f64 = (noise_variable_241 + noise_variable_228);
        let noise_metadata_schedule_232_e2720: f64 = (noise_metadata_schedule_232_e2718 - noise_variable_233);
        (noise_metadata_schedule_232_e2720,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_232_e2722;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_233_e2739,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_233_e2730: f64 = (noise_variable_225 * noise_variable_225);
        let noise_metadata_schedule_233_e2733: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_233_e2735: f64 = (noise_metadata_schedule_233_e2733 * params.p44);
        let noise_metadata_schedule_233_e2736: f64 = (noise_metadata_schedule_233_e2730 + noise_metadata_schedule_233_e2735);
        let noise_metadata_schedule_233_e2737: f64 = (noise_metadata_schedule_233_e2736).sqrt();
        (noise_metadata_schedule_233_e2737,)
    } else {
        (noise_variable_242,)
    }
};
            noise_variable_242 = noise_metadata_schedule_233_e2739;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_234_e2752,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_234_e2746: f64 = (-0.5);
        let noise_metadata_schedule_234_e2749: f64 = (noise_variable_225 + noise_variable_242);
        let noise_metadata_schedule_234_e2750: f64 = (noise_metadata_schedule_234_e2746 * noise_metadata_schedule_234_e2749);
        (noise_metadata_schedule_234_e2750,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_234_e2752;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_235_e2775,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_235_e2759: f64 = (-noise_variable_21);
        let noise_metadata_schedule_235_e2763: f64 = (noise_variable_232 / noise_variable_21);
        let noise_metadata_schedule_235_e2764: f64 = (1.0 - noise_metadata_schedule_235_e2763);
        let noise_metadata_schedule_235_e2767: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_235_e2768: f64 = (noise_metadata_schedule_235_e2764).powf(noise_metadata_schedule_235_e2767);
        let noise_metadata_schedule_235_e2769: f64 = (noise_metadata_schedule_235_e2759 * noise_metadata_schedule_235_e2768);
        let noise_metadata_schedule_235_e2772: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_235_e2773: f64 = (noise_metadata_schedule_235_e2769 / noise_metadata_schedule_235_e2772);
        (noise_metadata_schedule_235_e2773,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_235_e2775;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_236_e2785,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_236_e2783: f64 = (noise_variable_144 + noise_variable_225);
        (noise_metadata_schedule_236_e2783,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_236_e2785;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_237_e2802,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_237_e2793: f64 = (noise_variable_244 * noise_variable_244);
        let noise_metadata_schedule_237_e2796: f64 = (4.0 * params.p44);
        let noise_metadata_schedule_237_e2798: f64 = (noise_metadata_schedule_237_e2796 * params.p44);
        let noise_metadata_schedule_237_e2799: f64 = (noise_metadata_schedule_237_e2793 + noise_metadata_schedule_237_e2798);
        let noise_metadata_schedule_237_e2800: f64 = (noise_metadata_schedule_237_e2799).sqrt();
        (noise_metadata_schedule_237_e2800,)
    } else {
        (noise_variable_245,)
    }
};
            noise_variable_245 = noise_metadata_schedule_237_e2802;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_238_e2816,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_238_e2811: f64 = (noise_variable_244 - noise_variable_245);
        let noise_metadata_schedule_238_e2812: f64 = (0.5 * noise_metadata_schedule_238_e2811);
        let noise_metadata_schedule_238_e2814: f64 = (noise_metadata_schedule_238_e2812 - noise_variable_225);
        (noise_metadata_schedule_238_e2814,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_238_e2816;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_239_e2839,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_239_e2823: f64 = (-noise_variable_21);
        let noise_metadata_schedule_239_e2827: f64 = (noise_variable_236 / noise_variable_21);
        let noise_metadata_schedule_239_e2828: f64 = (1.0 - noise_metadata_schedule_239_e2827);
        let noise_metadata_schedule_239_e2831: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_239_e2832: f64 = (noise_metadata_schedule_239_e2828).powf(noise_metadata_schedule_239_e2831);
        let noise_metadata_schedule_239_e2833: f64 = (noise_metadata_schedule_239_e2823 * noise_metadata_schedule_239_e2832);
        let noise_metadata_schedule_239_e2836: f64 = (1.0 - params.p43);
        let noise_metadata_schedule_239_e2837: f64 = (noise_metadata_schedule_239_e2833 / noise_metadata_schedule_239_e2836);
        (noise_metadata_schedule_239_e2837,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_239_e2839;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let (noise_metadata_schedule_240_e2862,) = {
    if ((noise_variable_246 == 0.0) && (noise_variable_249 == 0.0)) {
        let noise_metadata_schedule_240_e2848: f64 = (1.0 - params.p34);
        let noise_metadata_schedule_240_e2850: f64 = (-params.p43);
        let noise_metadata_schedule_240_e2851: f64 = (noise_metadata_schedule_240_e2848).powf(noise_metadata_schedule_240_e2850);
        let noise_metadata_schedule_240_e2854: f64 = (noise_variable_144 - noise_variable_236);
        let noise_metadata_schedule_240_e2856: f64 = (noise_metadata_schedule_240_e2854 + noise_variable_232);
        let noise_metadata_schedule_240_e2857: f64 = (noise_metadata_schedule_240_e2851 * noise_metadata_schedule_240_e2856);
        let noise_metadata_schedule_240_e2858: f64 = (noise_variable_228 + noise_metadata_schedule_240_e2857);
        let noise_metadata_schedule_240_e2860: f64 = (noise_metadata_schedule_240_e2858 - noise_variable_243);
        (noise_metadata_schedule_240_e2860,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_240_e2862;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_241_e2866: f64 = (noise_variable_27 * noise_variable_73);
            let noise_metadata_schedule_241_e2867: f64 = (1.0 / noise_metadata_schedule_241_e2866);
            noise_variable_112 = noise_metadata_schedule_241_e2867;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_242_e2870: f64 = if noise_variable_143 < noise_variable_61 { 1.0 } else { 0.0 };
            noise_variable_250 = noise_metadata_schedule_242_e2870;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_243_e2877,) = {
    if (noise_variable_250 != 0.0) {
        let noise_metadata_schedule_243_e2874: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_243_e2875: f64 = (noise_metadata_schedule_243_e2874).exp();
        (noise_metadata_schedule_243_e2875,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_243_e2877;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_244_e2893,) = {
    if (noise_variable_250 == 0.0) {
        let noise_metadata_schedule_244_e2882: f64 = (noise_variable_61 * noise_variable_112);
        let noise_metadata_schedule_244_e2883: f64 = (noise_metadata_schedule_244_e2882).exp();
        let noise_metadata_schedule_244_e2887: f64 = (noise_variable_143 - noise_variable_61);
        let noise_metadata_schedule_244_e2889: f64 = (noise_metadata_schedule_244_e2887 * noise_variable_112);
        let noise_metadata_schedule_244_e2890: f64 = (1.0 + noise_metadata_schedule_244_e2889);
        let noise_metadata_schedule_244_e2891: f64 = (noise_metadata_schedule_244_e2883 * noise_metadata_schedule_244_e2890);
        (noise_metadata_schedule_244_e2891,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_244_e2893;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let noise_metadata_schedule_245_e2897: f64 = (noise_variable_109 - 1.0);
            let noise_metadata_schedule_245_e2898: f64 = (noise_variable_0 * noise_metadata_schedule_245_e2897);
            noise_variable_74 = noise_metadata_schedule_245_e2898;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_246_e2902: f64 = (noise_variable_28 * noise_variable_73);
            let noise_metadata_schedule_246_e2903: f64 = (1.0 / noise_metadata_schedule_246_e2902);
            noise_variable_112 = noise_metadata_schedule_246_e2903;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let noise_metadata_schedule_247_e2906: f64 = if noise_variable_144 < noise_variable_62 { 1.0 } else { 0.0 };
            noise_variable_251 = noise_metadata_schedule_247_e2906;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_248_e2913,) = {
    if (noise_variable_251 != 0.0) {
        let noise_metadata_schedule_248_e2910: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_248_e2911: f64 = (noise_metadata_schedule_248_e2910).exp();
        (noise_metadata_schedule_248_e2911,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_248_e2913;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 | 10 | 12) {
            let (noise_metadata_schedule_249_e2929,) = {
    if (noise_variable_251 == 0.0) {
        let noise_metadata_schedule_249_e2918: f64 = (noise_variable_62 * noise_variable_112);
        let noise_metadata_schedule_249_e2919: f64 = (noise_metadata_schedule_249_e2918).exp();
        let noise_metadata_schedule_249_e2923: f64 = (noise_variable_144 - noise_variable_62);
        let noise_metadata_schedule_249_e2925: f64 = (noise_metadata_schedule_249_e2923 * noise_variable_112);
        let noise_metadata_schedule_249_e2926: f64 = (1.0 + noise_metadata_schedule_249_e2925);
        let noise_metadata_schedule_249_e2927: f64 = (noise_metadata_schedule_249_e2919 * noise_metadata_schedule_249_e2926);
        (noise_metadata_schedule_249_e2927,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_249_e2929;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let noise_metadata_schedule_250_e2932: f64 = (noise_variable_0 * noise_variable_1);
            let noise_metadata_schedule_250_e2935: f64 = (noise_variable_109 - 1.0);
            let noise_metadata_schedule_250_e2936: f64 = (noise_metadata_schedule_250_e2932 * noise_metadata_schedule_250_e2935);
            noise_variable_75 = noise_metadata_schedule_250_e2936;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_251_e2940: f64 = (noise_variable_114 * noise_variable_44);
            let noise_metadata_schedule_251_e2941: f64 = (1.0 + noise_metadata_schedule_251_e2940);
            let noise_metadata_schedule_251_e2944: f64 = (noise_variable_116 * noise_variable_43);
            let noise_metadata_schedule_251_e2945: f64 = (noise_metadata_schedule_251_e2941 + noise_metadata_schedule_251_e2944);
            let noise_metadata_schedule_251_e2947: f64 = (noise_metadata_schedule_251_e2945 - 0.0001);
            noise_variable_78 = noise_metadata_schedule_251_e2947;
        }
        if matches!(source_index, 0 | 1 | 4 | 10 | 12) {
            let noise_metadata_schedule_252_e2951: f64 = (noise_variable_78 * noise_variable_78);
            let noise_metadata_schedule_252_e2953: f64 = (noise_metadata_schedule_252_e2951 + 1e-8);
            let noise_metadata_schedule_252_e2954: f64 = (noise_metadata_schedule_252_e2953).sqrt();
            let noise_metadata_schedule_252_e2956: f64 = (noise_metadata_schedule_252_e2954 + noise_variable_78);
            let noise_metadata_schedule_252_e2957: f64 = (0.5 * noise_metadata_schedule_252_e2956);
            let noise_metadata_schedule_252_e2959: f64 = (noise_metadata_schedule_252_e2957 + 0.0001);
            noise_variable_79 = noise_metadata_schedule_252_e2959;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let noise_metadata_schedule_253_e2962: f64 = (noise_variable_74 * noise_variable_45);
            let noise_metadata_schedule_253_e2965: f64 = (noise_variable_75 * noise_variable_46);
            let noise_metadata_schedule_253_e2966: f64 = (noise_metadata_schedule_253_e2962 + noise_metadata_schedule_253_e2965);
            noise_variable_80 = noise_metadata_schedule_253_e2966;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let noise_metadata_schedule_254_e2969: f64 = if params.p30 < 0.5 { 1.0 } else { 0.0 };
            noise_variable_252 = noise_metadata_schedule_254_e2969;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let (noise_metadata_schedule_255_e2981,) = {
    if (noise_variable_252 != 0.0) {
        let noise_metadata_schedule_255_e2974: f64 = (1.0 / params.p73);
        let noise_metadata_schedule_255_e2975: f64 = (noise_variable_79).powf(noise_metadata_schedule_255_e2974);
        let noise_metadata_schedule_255_e2978: f64 = (4.0 * noise_variable_80);
        let noise_metadata_schedule_255_e2979: f64 = (noise_metadata_schedule_255_e2975 + noise_metadata_schedule_255_e2978);
        (noise_metadata_schedule_255_e2979,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_255_e2981;
        }
        if matches!(source_index, 4 | 10) {
            let noise_metadata_schedule_256_e2984: f64 = if noise_variable_108 > 1e-8 { 1.0 } else { 0.0 };
            noise_variable_253 = noise_metadata_schedule_256_e2984;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_257_e2996,) = {
    if ((noise_variable_252 != 0.0) && (noise_variable_253 != 0.0)) {
        let noise_metadata_schedule_257_e2992: f64 = (noise_variable_108).powf(params.p73);
        let noise_metadata_schedule_257_e2993: f64 = (noise_variable_79 + noise_metadata_schedule_257_e2992);
        let noise_metadata_schedule_257_e2994: f64 = (0.5 * noise_metadata_schedule_257_e2993);
        (noise_metadata_schedule_257_e2994,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_257_e2996;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_258_e3009,) = {
    if ((noise_variable_252 != 0.0) && (noise_variable_253 == 0.0)) {
        let noise_metadata_schedule_258_e3005: f64 = (1e-8_f64).powf(params.p73);
        let noise_metadata_schedule_258_e3006: f64 = (noise_variable_79 + noise_metadata_schedule_258_e3005);
        let noise_metadata_schedule_258_e3007: f64 = (0.5 * noise_metadata_schedule_258_e3006);
        (noise_metadata_schedule_258_e3007,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_258_e3009;
        }
        if matches!(source_index, 4 | 10 | 12) {
            let (noise_metadata_schedule_259_e3018,) = {
    if (noise_variable_252 == 0.0) {
        let noise_metadata_schedule_259_e3015: f64 = (4.0 * noise_variable_80);
        let noise_metadata_schedule_259_e3016: f64 = (1.0 + noise_metadata_schedule_259_e3015);
        (noise_metadata_schedule_259_e3016,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_259_e3018;
        }
        if matches!(source_index, 4 | 10) {
            let noise_metadata_schedule_260_e3021: f64 = if noise_variable_108 > 1e-8 { 1.0 } else { 0.0 };
            noise_variable_254 = noise_metadata_schedule_260_e3021;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_261_e3036,) = {
    if ((noise_variable_252 == 0.0) && (noise_variable_254 != 0.0)) {
        let noise_metadata_schedule_261_e3028: f64 = (0.5 * noise_variable_79);
        let noise_metadata_schedule_261_e3032: f64 = (noise_variable_108).powf(params.p73);
        let noise_metadata_schedule_261_e3033: f64 = (1.0 + noise_metadata_schedule_261_e3032);
        let noise_metadata_schedule_261_e3034: f64 = (noise_metadata_schedule_261_e3028 * noise_metadata_schedule_261_e3033);
        (noise_metadata_schedule_261_e3034,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_261_e3036;
        }
        if matches!(source_index, 4 | 10) {
            let (noise_metadata_schedule_262_e3052,) = {
    if ((noise_variable_252 == 0.0) && (noise_variable_254 == 0.0)) {
        let noise_metadata_schedule_262_e3044: f64 = (0.5 * noise_variable_79);
        let noise_metadata_schedule_262_e3048: f64 = (1e-8_f64).powf(params.p73);
        let noise_metadata_schedule_262_e3049: f64 = (1.0 + noise_metadata_schedule_262_e3048);
        let noise_metadata_schedule_262_e3050: f64 = (noise_metadata_schedule_262_e3044 * noise_metadata_schedule_262_e3049);
        (noise_metadata_schedule_262_e3050,)
    } else {
        (noise_variable_81,)
    }
};
            noise_variable_81 = noise_metadata_schedule_262_e3052;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_264_e3058: f64 = (noise_variable_74 / noise_variable_81);
            noise_variable_76 = noise_metadata_schedule_264_e3058;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let noise_metadata_schedule_266_e3062: f64 = if params.p31 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_255 = noise_metadata_schedule_266_e3062;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let (noise_metadata_schedule_267_e3070,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_267_e3067: f64 = (params.p33 * noise_variable_73);
        let noise_metadata_schedule_267_e3068: f64 = (1.0 / noise_metadata_schedule_267_e3067);
        (noise_metadata_schedule_267_e3068,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_267_e3070;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let noise_metadata_schedule_268_e3073: f64 = if noise_variable_146 < noise_variable_63 { 1.0 } else { 0.0 };
            noise_variable_256 = noise_metadata_schedule_268_e3073;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let (noise_metadata_schedule_269_e3082,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_256 != 0.0)) {
        let noise_metadata_schedule_269_e3079: f64 = (noise_variable_146 * noise_variable_112);
        let noise_metadata_schedule_269_e3080: f64 = (noise_metadata_schedule_269_e3079).exp();
        (noise_metadata_schedule_269_e3080,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_269_e3082;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8 | 12) {
            let (noise_metadata_schedule_270_e3100,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_256 == 0.0)) {
        let noise_metadata_schedule_270_e3089: f64 = (noise_variable_63 * noise_variable_112);
        let noise_metadata_schedule_270_e3090: f64 = (noise_metadata_schedule_270_e3089).exp();
        let noise_metadata_schedule_270_e3094: f64 = (noise_variable_146 - noise_variable_63);
        let noise_metadata_schedule_270_e3096: f64 = (noise_metadata_schedule_270_e3094 * noise_variable_112);
        let noise_metadata_schedule_270_e3097: f64 = (1.0 + noise_metadata_schedule_270_e3096);
        let noise_metadata_schedule_270_e3098: f64 = (noise_metadata_schedule_270_e3090 * noise_metadata_schedule_270_e3097);
        (noise_metadata_schedule_270_e3098,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_270_e3100;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8 | 12) {
            let noise_metadata_schedule_271_e3103: f64 = if noise_variable_144 < noise_variable_63 { 1.0 } else { 0.0 };
            noise_variable_257 = noise_metadata_schedule_271_e3103;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8 | 12) {
            let (noise_metadata_schedule_272_e3112,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_257 != 0.0)) {
        let noise_metadata_schedule_272_e3109: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_272_e3110: f64 = (noise_metadata_schedule_272_e3109).exp();
        (noise_metadata_schedule_272_e3110,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_272_e3112;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8 | 12) {
            let (noise_metadata_schedule_273_e3130,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_257 == 0.0)) {
        let noise_metadata_schedule_273_e3119: f64 = (noise_variable_63 * noise_variable_112);
        let noise_metadata_schedule_273_e3120: f64 = (noise_metadata_schedule_273_e3119).exp();
        let noise_metadata_schedule_273_e3124: f64 = (noise_variable_144 - noise_variable_63);
        let noise_metadata_schedule_273_e3126: f64 = (noise_metadata_schedule_273_e3124 * noise_variable_112);
        let noise_metadata_schedule_273_e3127: f64 = (1.0 + noise_metadata_schedule_273_e3126);
        let noise_metadata_schedule_273_e3128: f64 = (noise_metadata_schedule_273_e3120 * noise_metadata_schedule_273_e3127);
        (noise_metadata_schedule_273_e3128,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_273_e3130;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_274_e3146,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_274_e3135: f64 = (params.p32 * noise_variable_109);
        let noise_metadata_schedule_274_e3138: f64 = (1.0 - params.p32);
        let noise_metadata_schedule_274_e3140: f64 = (noise_metadata_schedule_274_e3138 * noise_variable_111);
        let noise_metadata_schedule_274_e3141: f64 = (noise_metadata_schedule_274_e3135 + noise_metadata_schedule_274_e3140);
        let noise_metadata_schedule_274_e3143: f64 = (noise_metadata_schedule_274_e3141 - 1.0);
        let noise_metadata_schedule_274_e3144: f64 = (noise_variable_5 * noise_metadata_schedule_274_e3143);
        (noise_metadata_schedule_274_e3144,)
    } else {
        (noise_variable_82,)
    }
};
            noise_variable_82 = noise_metadata_schedule_274_e3146;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_275_e3152,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_275_e3150: f64 = (noise_variable_82 * noise_variable_47);
        (noise_metadata_schedule_275_e3150,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_275_e3152;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_276_e3160,) = {
    if (noise_variable_255 != 0.0) {
        let noise_metadata_schedule_276_e3157: f64 = (4.0 * noise_variable_85);
        let noise_metadata_schedule_276_e3158: f64 = (1.0 + noise_metadata_schedule_276_e3157);
        (noise_metadata_schedule_276_e3158,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_276_e3160;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_277_e3163: f64 = if noise_variable_108 > 1e-8 { 1.0 } else { 0.0 };
            noise_variable_258 = noise_metadata_schedule_277_e3163;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_278_e3174,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_258 != 0.0)) {
        let noise_metadata_schedule_278_e3170: f64 = (noise_variable_108).sqrt();
        let noise_metadata_schedule_278_e3171: f64 = (1.0 + noise_metadata_schedule_278_e3170);
        let noise_metadata_schedule_278_e3172: f64 = (0.5 * noise_metadata_schedule_278_e3171);
        (noise_metadata_schedule_278_e3172,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_278_e3174;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_279_e3186,) = {
    if ((noise_variable_255 != 0.0) && (noise_variable_258 == 0.0)) {
        let noise_metadata_schedule_279_e3182: f64 = (1e-8_f64).sqrt();
        let noise_metadata_schedule_279_e3183: f64 = (1.0 + noise_metadata_schedule_279_e3182);
        let noise_metadata_schedule_279_e3184: f64 = (0.5 * noise_metadata_schedule_279_e3183);
        (noise_metadata_schedule_279_e3184,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_279_e3186;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_281_e3196,) = {
    if (noise_variable_255 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_281_e3196;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_282_e3199: f64 = if params.p55 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_259 = noise_metadata_schedule_282_e3199;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_283_e3207,) = {
    if (noise_variable_259 != 0.0) {
        let noise_metadata_schedule_283_e3204: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_283_e3205: f64 = (1.0 / noise_metadata_schedule_283_e3204);
        (noise_metadata_schedule_283_e3205,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_283_e3207;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_284_e3210: f64 = if noise_variable_143 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_260 = noise_metadata_schedule_284_e3210;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_285_e3219,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_260 != 0.0)) {
        let noise_metadata_schedule_285_e3216: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_285_e3217: f64 = (noise_metadata_schedule_285_e3216).exp();
        (noise_metadata_schedule_285_e3217,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_285_e3219;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_286_e3237,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_260 == 0.0)) {
        let noise_metadata_schedule_286_e3226: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_286_e3227: f64 = (noise_metadata_schedule_286_e3226).exp();
        let noise_metadata_schedule_286_e3231: f64 = (noise_variable_143 - noise_variable_65);
        let noise_metadata_schedule_286_e3233: f64 = (noise_metadata_schedule_286_e3231 * noise_variable_112);
        let noise_metadata_schedule_286_e3234: f64 = (1.0 + noise_metadata_schedule_286_e3233);
        let noise_metadata_schedule_286_e3235: f64 = (noise_metadata_schedule_286_e3227 * noise_metadata_schedule_286_e3234);
        (noise_metadata_schedule_286_e3235,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_286_e3237;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_287_e3245,) = {
    if (noise_variable_259 != 0.0) {
        let noise_metadata_schedule_287_e3242: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_287_e3243: f64 = (1.0 / noise_metadata_schedule_287_e3242);
        (noise_metadata_schedule_287_e3243,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_287_e3245;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_288_e3248: f64 = if noise_variable_143 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_261 = noise_metadata_schedule_288_e3248;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_289_e3257,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_261 != 0.0)) {
        let noise_metadata_schedule_289_e3254: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_289_e3255: f64 = (noise_metadata_schedule_289_e3254).exp();
        (noise_metadata_schedule_289_e3255,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_289_e3257;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_290_e3275,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_261 == 0.0)) {
        let noise_metadata_schedule_290_e3264: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_290_e3265: f64 = (noise_metadata_schedule_290_e3264).exp();
        let noise_metadata_schedule_290_e3269: f64 = (noise_variable_143 - noise_variable_66);
        let noise_metadata_schedule_290_e3271: f64 = (noise_metadata_schedule_290_e3269 * noise_variable_112);
        let noise_metadata_schedule_290_e3272: f64 = (1.0 + noise_metadata_schedule_290_e3271);
        let noise_metadata_schedule_290_e3273: f64 = (noise_metadata_schedule_290_e3265 * noise_metadata_schedule_290_e3272);
        (noise_metadata_schedule_290_e3273,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_290_e3275;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_291_e3278: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_262 = noise_metadata_schedule_291_e3278;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_292_e3302,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_262 != 0.0)) {
        let noise_metadata_schedule_292_e3287: f64 = (noise_variable_79 - 1.0);
        let noise_metadata_schedule_292_e3288: f64 = (params.p57 * noise_metadata_schedule_292_e3287);
        let noise_metadata_schedule_292_e3289: f64 = (1.0 + noise_metadata_schedule_292_e3288);
        let noise_metadata_schedule_292_e3290: f64 = (noise_variable_3 * noise_metadata_schedule_292_e3289);
        let noise_metadata_schedule_292_e3293: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_292_e3294: f64 = (noise_metadata_schedule_292_e3290 * noise_metadata_schedule_292_e3293);
        let noise_metadata_schedule_292_e3298: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_292_e3299: f64 = (noise_variable_6 * noise_metadata_schedule_292_e3298);
        let noise_metadata_schedule_292_e3300: f64 = (noise_metadata_schedule_292_e3294 + noise_metadata_schedule_292_e3299);
        (noise_metadata_schedule_292_e3300,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_292_e3302;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_293_e3319,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_262 == 0.0)) {
        let noise_metadata_schedule_293_e3310: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_293_e3311: f64 = (noise_variable_3 * noise_metadata_schedule_293_e3310);
        let noise_metadata_schedule_293_e3315: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_293_e3316: f64 = (noise_variable_6 * noise_metadata_schedule_293_e3315);
        let noise_metadata_schedule_293_e3317: f64 = (noise_metadata_schedule_293_e3311 + noise_metadata_schedule_293_e3316);
        (noise_metadata_schedule_293_e3317,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_293_e3319;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_294_e3322: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_263 = noise_metadata_schedule_294_e3322;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_295_e3331,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_295_e3327: f64 = (-noise_variable_31);
        let noise_metadata_schedule_295_e3329: f64 = (noise_metadata_schedule_295_e3327 - noise_variable_143);
        (noise_metadata_schedule_295_e3329,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_295_e3331;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_296_e3341,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_296_e3338: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_296_e3339: f64 = (1.0 / noise_metadata_schedule_296_e3338);
        (noise_metadata_schedule_296_e3339,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_296_e3341;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_297_e3344: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_264 = noise_metadata_schedule_297_e3344;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_298_e3355,) = {
    if (((noise_variable_259 != 0.0) && (noise_variable_263 != 0.0)) && (noise_variable_264 != 0.0)) {
        let noise_metadata_schedule_298_e3352: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_298_e3353: f64 = (noise_metadata_schedule_298_e3352).exp();
        (noise_metadata_schedule_298_e3353,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_298_e3355;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_299_e3375,) = {
    if (((noise_variable_259 != 0.0) && (noise_variable_263 != 0.0)) && (noise_variable_264 == 0.0)) {
        let noise_metadata_schedule_299_e3364: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_299_e3365: f64 = (noise_metadata_schedule_299_e3364).exp();
        let noise_metadata_schedule_299_e3369: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_299_e3371: f64 = (noise_metadata_schedule_299_e3369 * noise_variable_112);
        let noise_metadata_schedule_299_e3372: f64 = (1.0 + noise_metadata_schedule_299_e3371);
        let noise_metadata_schedule_299_e3373: f64 = (noise_metadata_schedule_299_e3365 * noise_metadata_schedule_299_e3372);
        (noise_metadata_schedule_299_e3373,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_299_e3375;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_300_e3387,) = {
    if ((noise_variable_259 != 0.0) && (noise_variable_263 != 0.0)) {
        let noise_metadata_schedule_300_e3383: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_300_e3384: f64 = (params.p90 * noise_metadata_schedule_300_e3383);
        let noise_metadata_schedule_300_e3385: f64 = (noise_variable_87 - noise_metadata_schedule_300_e3384);
        (noise_metadata_schedule_300_e3385,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_300_e3387;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_301_e3391,) = {
    if (noise_variable_259 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_301_e3391;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_302_e3394: f64 = if params.p55 == 0.0 { 1.0 } else { 0.0 };
            noise_variable_265 = noise_metadata_schedule_302_e3394;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_303_e3401,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_303_e3401;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_304_e3412,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_304_e3409: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_304_e3410: f64 = (1.0 / noise_metadata_schedule_304_e3409);
        (noise_metadata_schedule_304_e3410,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_304_e3412;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_305_e3415: f64 = if noise_variable_145 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_266 = noise_metadata_schedule_305_e3415;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_306_e3427,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_266 != 0.0)) {
        let noise_metadata_schedule_306_e3424: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_306_e3425: f64 = (noise_metadata_schedule_306_e3424).exp();
        (noise_metadata_schedule_306_e3425,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_306_e3427;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_307_e3448,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_266 == 0.0)) {
        let noise_metadata_schedule_307_e3437: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_307_e3438: f64 = (noise_metadata_schedule_307_e3437).exp();
        let noise_metadata_schedule_307_e3442: f64 = (noise_variable_145 - noise_variable_65);
        let noise_metadata_schedule_307_e3444: f64 = (noise_metadata_schedule_307_e3442 * noise_variable_112);
        let noise_metadata_schedule_307_e3445: f64 = (1.0 + noise_metadata_schedule_307_e3444);
        let noise_metadata_schedule_307_e3446: f64 = (noise_metadata_schedule_307_e3438 * noise_metadata_schedule_307_e3445);
        (noise_metadata_schedule_307_e3446,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_307_e3448;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_308_e3459,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_308_e3456: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_308_e3457: f64 = (1.0 / noise_metadata_schedule_308_e3456);
        (noise_metadata_schedule_308_e3457,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_308_e3459;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_309_e3462: f64 = if noise_variable_145 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_267 = noise_metadata_schedule_309_e3462;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_310_e3474,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_267 != 0.0)) {
        let noise_metadata_schedule_310_e3471: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_310_e3472: f64 = (noise_metadata_schedule_310_e3471).exp();
        (noise_metadata_schedule_310_e3472,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_310_e3474;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_311_e3495,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_267 == 0.0)) {
        let noise_metadata_schedule_311_e3484: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_311_e3485: f64 = (noise_metadata_schedule_311_e3484).exp();
        let noise_metadata_schedule_311_e3489: f64 = (noise_variable_145 - noise_variable_66);
        let noise_metadata_schedule_311_e3491: f64 = (noise_metadata_schedule_311_e3489 * noise_variable_112);
        let noise_metadata_schedule_311_e3492: f64 = (1.0 + noise_metadata_schedule_311_e3491);
        let noise_metadata_schedule_311_e3493: f64 = (noise_metadata_schedule_311_e3485 * noise_metadata_schedule_311_e3492);
        (noise_metadata_schedule_311_e3493,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_311_e3495;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_312_e3512,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) {
        let noise_metadata_schedule_312_e3503: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_312_e3504: f64 = (noise_variable_3 * noise_metadata_schedule_312_e3503);
        let noise_metadata_schedule_312_e3508: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_312_e3509: f64 = (noise_variable_6 * noise_metadata_schedule_312_e3508);
        let noise_metadata_schedule_312_e3510: f64 = (noise_metadata_schedule_312_e3504 + noise_metadata_schedule_312_e3509);
        (noise_metadata_schedule_312_e3510,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_312_e3512;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_313_e3515: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_268 = noise_metadata_schedule_313_e3515;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_314_e3527,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_268 != 0.0)) {
        let noise_metadata_schedule_314_e3523: f64 = (-noise_variable_31);
        let noise_metadata_schedule_314_e3525: f64 = (noise_metadata_schedule_314_e3523 - noise_variable_143);
        (noise_metadata_schedule_314_e3525,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_314_e3527;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_315_e3540,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_268 != 0.0)) {
        let noise_metadata_schedule_315_e3537: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_315_e3538: f64 = (1.0 / noise_metadata_schedule_315_e3537);
        (noise_metadata_schedule_315_e3538,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_315_e3540;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_316_e3543: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_269 = noise_metadata_schedule_316_e3543;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_317_e3557,) = {
    if ((((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_268 != 0.0)) && (noise_variable_269 != 0.0)) {
        let noise_metadata_schedule_317_e3554: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_317_e3555: f64 = (noise_metadata_schedule_317_e3554).exp();
        (noise_metadata_schedule_317_e3555,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_317_e3557;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_318_e3580,) = {
    if ((((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_268 != 0.0)) && (noise_variable_269 == 0.0)) {
        let noise_metadata_schedule_318_e3569: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_318_e3570: f64 = (noise_metadata_schedule_318_e3569).exp();
        let noise_metadata_schedule_318_e3574: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_318_e3576: f64 = (noise_metadata_schedule_318_e3574 * noise_variable_112);
        let noise_metadata_schedule_318_e3577: f64 = (1.0 + noise_metadata_schedule_318_e3576);
        let noise_metadata_schedule_318_e3578: f64 = (noise_metadata_schedule_318_e3570 * noise_metadata_schedule_318_e3577);
        (noise_metadata_schedule_318_e3578,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_318_e3580;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_319_e3595,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 != 0.0)) && (noise_variable_268 != 0.0)) {
        let noise_metadata_schedule_319_e3591: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_319_e3592: f64 = (params.p90 * noise_metadata_schedule_319_e3591);
        let noise_metadata_schedule_319_e3593: f64 = (noise_variable_88 - noise_metadata_schedule_319_e3592);
        (noise_metadata_schedule_319_e3593,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_319_e3595;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_320_e3607,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) {
        let noise_metadata_schedule_320_e3604: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_320_e3605: f64 = (1.0 / noise_metadata_schedule_320_e3604);
        (noise_metadata_schedule_320_e3605,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_320_e3607;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_321_e3610: f64 = if noise_variable_143 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_270 = noise_metadata_schedule_321_e3610;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_322_e3623,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_270 != 0.0)) {
        let noise_metadata_schedule_322_e3620: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_322_e3621: f64 = (noise_metadata_schedule_322_e3620).exp();
        (noise_metadata_schedule_322_e3621,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_322_e3623;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_323_e3645,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_270 == 0.0)) {
        let noise_metadata_schedule_323_e3634: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_323_e3635: f64 = (noise_metadata_schedule_323_e3634).exp();
        let noise_metadata_schedule_323_e3639: f64 = (noise_variable_143 - noise_variable_65);
        let noise_metadata_schedule_323_e3641: f64 = (noise_metadata_schedule_323_e3639 * noise_variable_112);
        let noise_metadata_schedule_323_e3642: f64 = (1.0 + noise_metadata_schedule_323_e3641);
        let noise_metadata_schedule_323_e3643: f64 = (noise_metadata_schedule_323_e3635 * noise_metadata_schedule_323_e3642);
        (noise_metadata_schedule_323_e3643,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_323_e3645;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_324_e3657,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) {
        let noise_metadata_schedule_324_e3654: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_324_e3655: f64 = (1.0 / noise_metadata_schedule_324_e3654);
        (noise_metadata_schedule_324_e3655,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_324_e3657;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_325_e3660: f64 = if noise_variable_143 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_271 = noise_metadata_schedule_325_e3660;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_326_e3673,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_271 != 0.0)) {
        let noise_metadata_schedule_326_e3670: f64 = (noise_variable_143 * noise_variable_112);
        let noise_metadata_schedule_326_e3671: f64 = (noise_metadata_schedule_326_e3670).exp();
        (noise_metadata_schedule_326_e3671,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_326_e3673;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_327_e3695,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_271 == 0.0)) {
        let noise_metadata_schedule_327_e3684: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_327_e3685: f64 = (noise_metadata_schedule_327_e3684).exp();
        let noise_metadata_schedule_327_e3689: f64 = (noise_variable_143 - noise_variable_66);
        let noise_metadata_schedule_327_e3691: f64 = (noise_metadata_schedule_327_e3689 * noise_variable_112);
        let noise_metadata_schedule_327_e3692: f64 = (1.0 + noise_metadata_schedule_327_e3691);
        let noise_metadata_schedule_327_e3693: f64 = (noise_metadata_schedule_327_e3685 * noise_metadata_schedule_327_e3692);
        (noise_metadata_schedule_327_e3693,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_327_e3695;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_328_e3698: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_272 = noise_metadata_schedule_328_e3698;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_329_e3728,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_272 != 0.0)) {
        let noise_metadata_schedule_329_e3712: f64 = (noise_variable_79 - 1.0);
        let noise_metadata_schedule_329_e3713: f64 = (params.p57 * noise_metadata_schedule_329_e3712);
        let noise_metadata_schedule_329_e3714: f64 = (1.0 + noise_metadata_schedule_329_e3713);
        let noise_metadata_schedule_329_e3715: f64 = (noise_variable_3 * noise_metadata_schedule_329_e3714);
        let noise_metadata_schedule_329_e3718: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_329_e3719: f64 = (noise_metadata_schedule_329_e3715 * noise_metadata_schedule_329_e3718);
        let noise_metadata_schedule_329_e3723: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_329_e3724: f64 = (noise_variable_6 * noise_metadata_schedule_329_e3723);
        let noise_metadata_schedule_329_e3725: f64 = (noise_metadata_schedule_329_e3719 + noise_metadata_schedule_329_e3724);
        let noise_metadata_schedule_329_e3726: f64 = (params.p55 * noise_metadata_schedule_329_e3725);
        (noise_metadata_schedule_329_e3726,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_329_e3728;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_330_e3751,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_272 == 0.0)) {
        let noise_metadata_schedule_330_e3741: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_330_e3742: f64 = (noise_variable_3 * noise_metadata_schedule_330_e3741);
        let noise_metadata_schedule_330_e3746: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_330_e3747: f64 = (noise_variable_6 * noise_metadata_schedule_330_e3746);
        let noise_metadata_schedule_330_e3748: f64 = (noise_metadata_schedule_330_e3742 + noise_metadata_schedule_330_e3747);
        let noise_metadata_schedule_330_e3749: f64 = (params.p55 * noise_metadata_schedule_330_e3748);
        (noise_metadata_schedule_330_e3749,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_330_e3751;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_331_e3754: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_273 = noise_metadata_schedule_331_e3754;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_332_e3767,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_273 != 0.0)) {
        let noise_metadata_schedule_332_e3763: f64 = (-noise_variable_31);
        let noise_metadata_schedule_332_e3765: f64 = (noise_metadata_schedule_332_e3763 - noise_variable_143);
        (noise_metadata_schedule_332_e3765,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_332_e3767;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_333_e3781,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_273 != 0.0)) {
        let noise_metadata_schedule_333_e3778: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_333_e3779: f64 = (1.0 / noise_metadata_schedule_333_e3778);
        (noise_metadata_schedule_333_e3779,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_333_e3781;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let noise_metadata_schedule_334_e3784: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_274 = noise_metadata_schedule_334_e3784;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_335_e3799,) = {
    if ((((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_273 != 0.0)) && (noise_variable_274 != 0.0)) {
        let noise_metadata_schedule_335_e3796: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_335_e3797: f64 = (noise_metadata_schedule_335_e3796).exp();
        (noise_metadata_schedule_335_e3797,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_335_e3799;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 8) {
            let (noise_metadata_schedule_336_e3823,) = {
    if ((((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_273 != 0.0)) && (noise_variable_274 == 0.0)) {
        let noise_metadata_schedule_336_e3812: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_336_e3813: f64 = (noise_metadata_schedule_336_e3812).exp();
        let noise_metadata_schedule_336_e3817: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_336_e3819: f64 = (noise_metadata_schedule_336_e3817 * noise_variable_112);
        let noise_metadata_schedule_336_e3820: f64 = (1.0 + noise_metadata_schedule_336_e3819);
        let noise_metadata_schedule_336_e3821: f64 = (noise_metadata_schedule_336_e3813 * noise_metadata_schedule_336_e3820);
        (noise_metadata_schedule_336_e3821,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_336_e3823;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_337_e3841,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_273 != 0.0)) {
        let noise_metadata_schedule_337_e3834: f64 = (params.p55 * params.p90);
        let noise_metadata_schedule_337_e3837: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_337_e3838: f64 = (noise_metadata_schedule_337_e3834 * noise_metadata_schedule_337_e3837);
        let noise_metadata_schedule_337_e3839: f64 = (noise_variable_87 - noise_metadata_schedule_337_e3838);
        (noise_metadata_schedule_337_e3839,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_337_e3841;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_338_e3853,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) {
        let noise_metadata_schedule_338_e3850: f64 = (params.p56 * noise_variable_73);
        let noise_metadata_schedule_338_e3851: f64 = (1.0 / noise_metadata_schedule_338_e3850);
        (noise_metadata_schedule_338_e3851,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_338_e3853;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let noise_metadata_schedule_339_e3856: f64 = if noise_variable_145 < noise_variable_65 { 1.0 } else { 0.0 };
            noise_variable_275 = noise_metadata_schedule_339_e3856;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_340_e3869,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_275 != 0.0)) {
        let noise_metadata_schedule_340_e3866: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_340_e3867: f64 = (noise_metadata_schedule_340_e3866).exp();
        (noise_metadata_schedule_340_e3867,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_340_e3869;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_341_e3891,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_275 == 0.0)) {
        let noise_metadata_schedule_341_e3880: f64 = (noise_variable_65 * noise_variable_112);
        let noise_metadata_schedule_341_e3881: f64 = (noise_metadata_schedule_341_e3880).exp();
        let noise_metadata_schedule_341_e3885: f64 = (noise_variable_145 - noise_variable_65);
        let noise_metadata_schedule_341_e3887: f64 = (noise_metadata_schedule_341_e3885 * noise_variable_112);
        let noise_metadata_schedule_341_e3888: f64 = (1.0 + noise_metadata_schedule_341_e3887);
        let noise_metadata_schedule_341_e3889: f64 = (noise_metadata_schedule_341_e3881 * noise_metadata_schedule_341_e3888);
        (noise_metadata_schedule_341_e3889,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_341_e3891;
        }
        if matches!(source_index, 2 | 3 | 5 | 6 | 8) {
            let (noise_metadata_schedule_342_e3903,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) {
        let noise_metadata_schedule_342_e3900: f64 = (params.p59 * noise_variable_73);
        let noise_metadata_schedule_342_e3901: f64 = (1.0 / noise_metadata_schedule_342_e3900);
        (noise_metadata_schedule_342_e3901,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_342_e3903;
        }
        if matches!(source_index, 2 | 3 | 5 | 6) {
            let noise_metadata_schedule_343_e3906: f64 = if noise_variable_145 < noise_variable_66 { 1.0 } else { 0.0 };
            noise_variable_276 = noise_metadata_schedule_343_e3906;
        }
        if matches!(source_index, 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_344_e3919,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_276 != 0.0)) {
        let noise_metadata_schedule_344_e3916: f64 = (noise_variable_145 * noise_variable_112);
        let noise_metadata_schedule_344_e3917: f64 = (noise_metadata_schedule_344_e3916).exp();
        (noise_metadata_schedule_344_e3917,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_344_e3919;
        }
        if matches!(source_index, 2 | 3 | 5 | 6) {
            let (noise_metadata_schedule_345_e3941,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_276 == 0.0)) {
        let noise_metadata_schedule_345_e3930: f64 = (noise_variable_66 * noise_variable_112);
        let noise_metadata_schedule_345_e3931: f64 = (noise_metadata_schedule_345_e3930).exp();
        let noise_metadata_schedule_345_e3935: f64 = (noise_variable_145 - noise_variable_66);
        let noise_metadata_schedule_345_e3937: f64 = (noise_metadata_schedule_345_e3935 * noise_variable_112);
        let noise_metadata_schedule_345_e3938: f64 = (1.0 + noise_metadata_schedule_345_e3937);
        let noise_metadata_schedule_345_e3939: f64 = (noise_metadata_schedule_345_e3931 * noise_metadata_schedule_345_e3938);
        (noise_metadata_schedule_345_e3939,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_345_e3941;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_346_e3963,) = {
    if ((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) {
        let noise_metadata_schedule_346_e3949: f64 = (1.0 - params.p55);
        let noise_metadata_schedule_346_e3953: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_346_e3954: f64 = (noise_variable_3 * noise_metadata_schedule_346_e3953);
        let noise_metadata_schedule_346_e3958: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_346_e3959: f64 = (noise_variable_6 * noise_metadata_schedule_346_e3958);
        let noise_metadata_schedule_346_e3960: f64 = (noise_metadata_schedule_346_e3954 + noise_metadata_schedule_346_e3959);
        let noise_metadata_schedule_346_e3961: f64 = (noise_metadata_schedule_346_e3949 * noise_metadata_schedule_346_e3960);
        (noise_metadata_schedule_346_e3961,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_346_e3963;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let noise_metadata_schedule_347_e3966: f64 = if params.p88 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_277 = noise_metadata_schedule_347_e3966;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_348_e3979,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_348_e3975: f64 = (-noise_variable_31);
        let noise_metadata_schedule_348_e3977: f64 = (noise_metadata_schedule_348_e3975 - noise_variable_143);
        (noise_metadata_schedule_348_e3977,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_348_e3979;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_349_e3993,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_349_e3990: f64 = (noise_variable_32 * noise_variable_73);
        let noise_metadata_schedule_349_e3991: f64 = (1.0 / noise_metadata_schedule_349_e3990);
        (noise_metadata_schedule_349_e3991,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_349_e3993;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let noise_metadata_schedule_350_e3996: f64 = if noise_variable_150 < noise_variable_64 { 1.0 } else { 0.0 };
            noise_variable_278 = noise_metadata_schedule_350_e3996;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_351_e4011,) = {
    if ((((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_277 != 0.0)) && (noise_variable_278 != 0.0)) {
        let noise_metadata_schedule_351_e4008: f64 = (noise_variable_150 * noise_variable_112);
        let noise_metadata_schedule_351_e4009: f64 = (noise_metadata_schedule_351_e4008).exp();
        (noise_metadata_schedule_351_e4009,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_351_e4011;
        }
        if matches!(source_index, 2 | 3 | 8) {
            let (noise_metadata_schedule_352_e4035,) = {
    if ((((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_277 != 0.0)) && (noise_variable_278 == 0.0)) {
        let noise_metadata_schedule_352_e4024: f64 = (noise_variable_64 * noise_variable_112);
        let noise_metadata_schedule_352_e4025: f64 = (noise_metadata_schedule_352_e4024).exp();
        let noise_metadata_schedule_352_e4029: f64 = (noise_variable_150 - noise_variable_64);
        let noise_metadata_schedule_352_e4031: f64 = (noise_metadata_schedule_352_e4029 * noise_variable_112);
        let noise_metadata_schedule_352_e4032: f64 = (1.0 + noise_metadata_schedule_352_e4031);
        let noise_metadata_schedule_352_e4033: f64 = (noise_metadata_schedule_352_e4025 * noise_metadata_schedule_352_e4032);
        (noise_metadata_schedule_352_e4033,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_352_e4035;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_353_e4055,) = {
    if (((noise_variable_259 == 0.0) && (noise_variable_265 == 0.0)) && (noise_variable_277 != 0.0)) {
        let noise_metadata_schedule_353_e4046: f64 = (1.0 - params.p55);
        let noise_metadata_schedule_353_e4048: f64 = (noise_metadata_schedule_353_e4046 * params.p90);
        let noise_metadata_schedule_353_e4051: f64 = (noise_variable_111 - noise_variable_35);
        let noise_metadata_schedule_353_e4052: f64 = (noise_metadata_schedule_353_e4048 * noise_metadata_schedule_353_e4051);
        let noise_metadata_schedule_353_e4053: f64 = (noise_variable_88 - noise_metadata_schedule_353_e4052);
        (noise_metadata_schedule_353_e4053,)
    } else {
        (noise_variable_88,)
    }
};
            noise_variable_88 = noise_metadata_schedule_353_e4055;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_354_e4059: f64 = (params.p61 * noise_variable_73);
            let noise_metadata_schedule_354_e4060: f64 = (1.0 / noise_metadata_schedule_354_e4059);
            noise_variable_112 = noise_metadata_schedule_354_e4060;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_355_e4063: f64 = if noise_variable_144 < noise_variable_67 { 1.0 } else { 0.0 };
            noise_variable_279 = noise_metadata_schedule_355_e4063;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_356_e4070,) = {
    if (noise_variable_279 != 0.0) {
        let noise_metadata_schedule_356_e4067: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_356_e4068: f64 = (noise_metadata_schedule_356_e4067).exp();
        (noise_metadata_schedule_356_e4068,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_356_e4070;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_357_e4086,) = {
    if (noise_variable_279 == 0.0) {
        let noise_metadata_schedule_357_e4075: f64 = (noise_variable_67 * noise_variable_112);
        let noise_metadata_schedule_357_e4076: f64 = (noise_metadata_schedule_357_e4075).exp();
        let noise_metadata_schedule_357_e4080: f64 = (noise_variable_144 - noise_variable_67);
        let noise_metadata_schedule_357_e4082: f64 = (noise_metadata_schedule_357_e4080 * noise_variable_112);
        let noise_metadata_schedule_357_e4083: f64 = (1.0 + noise_metadata_schedule_357_e4082);
        let noise_metadata_schedule_357_e4084: f64 = (noise_metadata_schedule_357_e4076 * noise_metadata_schedule_357_e4083);
        (noise_metadata_schedule_357_e4084,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_357_e4086;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_358_e4090: f64 = (params.p63 * noise_variable_73);
            let noise_metadata_schedule_358_e4091: f64 = (1.0 / noise_metadata_schedule_358_e4090);
            noise_variable_112 = noise_metadata_schedule_358_e4091;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_359_e4094: f64 = if noise_variable_144 < noise_variable_68 { 1.0 } else { 0.0 };
            noise_variable_280 = noise_metadata_schedule_359_e4094;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_360_e4101,) = {
    if (noise_variable_280 != 0.0) {
        let noise_metadata_schedule_360_e4098: f64 = (noise_variable_144 * noise_variable_112);
        let noise_metadata_schedule_360_e4099: f64 = (noise_metadata_schedule_360_e4098).exp();
        (noise_metadata_schedule_360_e4099,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_360_e4101;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_361_e4117,) = {
    if (noise_variable_280 == 0.0) {
        let noise_metadata_schedule_361_e4106: f64 = (noise_variable_68 * noise_variable_112);
        let noise_metadata_schedule_361_e4107: f64 = (noise_metadata_schedule_361_e4106).exp();
        let noise_metadata_schedule_361_e4111: f64 = (noise_variable_144 - noise_variable_68);
        let noise_metadata_schedule_361_e4113: f64 = (noise_metadata_schedule_361_e4111 * noise_variable_112);
        let noise_metadata_schedule_361_e4114: f64 = (1.0 + noise_metadata_schedule_361_e4113);
        let noise_metadata_schedule_361_e4115: f64 = (noise_metadata_schedule_361_e4107 * noise_metadata_schedule_361_e4114);
        (noise_metadata_schedule_361_e4115,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_361_e4117;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_363_e4135: f64 = if ((params.p64 > 0.0) || (params.p65 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_281 = noise_metadata_schedule_363_e4135;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_364_e4143,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_364_e4140: f64 = (params.p61 * noise_variable_73);
        let noise_metadata_schedule_364_e4141: f64 = (1.0 / noise_metadata_schedule_364_e4140);
        (noise_metadata_schedule_364_e4141,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_364_e4143;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let noise_metadata_schedule_365_e4146: f64 = if noise_variable_146 < noise_variable_69 { 1.0 } else { 0.0 };
            noise_variable_282 = noise_metadata_schedule_365_e4146;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_366_e4155,) = {
    if ((noise_variable_281 != 0.0) && (noise_variable_282 != 0.0)) {
        let noise_metadata_schedule_366_e4152: f64 = (noise_variable_146 * noise_variable_112);
        let noise_metadata_schedule_366_e4153: f64 = (noise_metadata_schedule_366_e4152).exp();
        (noise_metadata_schedule_366_e4153,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_366_e4155;
        }
        if matches!(source_index, 5 | 6 | 8) {
            let (noise_metadata_schedule_367_e4173,) = {
    if ((noise_variable_281 != 0.0) && (noise_variable_282 == 0.0)) {
        let noise_metadata_schedule_367_e4162: f64 = (noise_variable_69 * noise_variable_112);
        let noise_metadata_schedule_367_e4163: f64 = (noise_metadata_schedule_367_e4162).exp();
        let noise_metadata_schedule_367_e4167: f64 = (noise_variable_146 - noise_variable_69);
        let noise_metadata_schedule_367_e4169: f64 = (noise_metadata_schedule_367_e4167 * noise_variable_112);
        let noise_metadata_schedule_367_e4170: f64 = (1.0 + noise_metadata_schedule_367_e4169);
        let noise_metadata_schedule_367_e4171: f64 = (noise_metadata_schedule_367_e4163 * noise_metadata_schedule_367_e4170);
        (noise_metadata_schedule_367_e4171,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_367_e4173;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_368_e4181,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_368_e4178: f64 = (params.p63 * noise_variable_73);
        let noise_metadata_schedule_368_e4179: f64 = (1.0 / noise_metadata_schedule_368_e4178);
        (noise_metadata_schedule_368_e4179,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_368_e4181;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_369_e4184: f64 = if noise_variable_146 < noise_variable_70 { 1.0 } else { 0.0 };
            noise_variable_283 = noise_metadata_schedule_369_e4184;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_370_e4193,) = {
    if ((noise_variable_281 != 0.0) && (noise_variable_283 != 0.0)) {
        let noise_metadata_schedule_370_e4190: f64 = (noise_variable_146 * noise_variable_112);
        let noise_metadata_schedule_370_e4191: f64 = (noise_metadata_schedule_370_e4190).exp();
        (noise_metadata_schedule_370_e4191,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_370_e4193;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_371_e4211,) = {
    if ((noise_variable_281 != 0.0) && (noise_variable_283 == 0.0)) {
        let noise_metadata_schedule_371_e4200: f64 = (noise_variable_70 * noise_variable_112);
        let noise_metadata_schedule_371_e4201: f64 = (noise_metadata_schedule_371_e4200).exp();
        let noise_metadata_schedule_371_e4205: f64 = (noise_variable_146 - noise_variable_70);
        let noise_metadata_schedule_371_e4207: f64 = (noise_metadata_schedule_371_e4205 * noise_variable_112);
        let noise_metadata_schedule_371_e4208: f64 = (1.0 + noise_metadata_schedule_371_e4207);
        let noise_metadata_schedule_371_e4209: f64 = (noise_metadata_schedule_371_e4201 * noise_metadata_schedule_371_e4208);
        (noise_metadata_schedule_371_e4209,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_371_e4211;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_372_e4225,) = {
    if (noise_variable_281 != 0.0) {
        let noise_metadata_schedule_372_e4216: f64 = (noise_variable_109 - 1.0);
        let noise_metadata_schedule_372_e4217: f64 = (noise_variable_8 * noise_metadata_schedule_372_e4216);
        let noise_metadata_schedule_372_e4221: f64 = (noise_variable_110 - 1.0);
        let noise_metadata_schedule_372_e4222: f64 = (noise_variable_9 * noise_metadata_schedule_372_e4221);
        let noise_metadata_schedule_372_e4223: f64 = (noise_metadata_schedule_372_e4217 + noise_metadata_schedule_372_e4222);
        (noise_metadata_schedule_372_e4223,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_372_e4225;
        }
        if matches!(source_index, 5 | 6) {
            let (noise_metadata_schedule_373_e4230,) = {
    if (noise_variable_281 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_373_e4230;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_374_e4233: f64 = (noise_variable_144 / noise_variable_73);
            noise_variable_108 = noise_metadata_schedule_374_e4233;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_375_e4236: f64 = if noise_variable_108 < noise_variable_113 { 1.0 } else { 0.0 };
            noise_variable_284 = noise_metadata_schedule_375_e4236;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_376_e4241,) = {
    if (noise_variable_284 != 0.0) {
        let noise_metadata_schedule_376_e4239: f64 = (noise_variable_108).exp();
        (noise_metadata_schedule_376_e4239,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_376_e4241;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_377_e4253,) = {
    if (noise_variable_284 == 0.0) {
        let noise_metadata_schedule_377_e4245: f64 = (noise_variable_113).exp();
        let noise_metadata_schedule_377_e4249: f64 = (noise_variable_108 - noise_variable_113);
        let noise_metadata_schedule_377_e4250: f64 = (1.0 + noise_metadata_schedule_377_e4249);
        let noise_metadata_schedule_377_e4251: f64 = (noise_metadata_schedule_377_e4245 * noise_metadata_schedule_377_e4250);
        (noise_metadata_schedule_377_e4251,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_377_e4253;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_378_e4256: f64 = (noise_variable_148 / noise_variable_73);
            noise_variable_108 = noise_metadata_schedule_378_e4256;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_379_e4259: f64 = if noise_variable_108 < noise_variable_113 { 1.0 } else { 0.0 };
            noise_variable_285 = noise_metadata_schedule_379_e4259;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_380_e4264,) = {
    if (noise_variable_285 != 0.0) {
        let noise_metadata_schedule_380_e4262: f64 = (noise_variable_108).exp();
        (noise_metadata_schedule_380_e4262,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_380_e4264;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_381_e4276,) = {
    if (noise_variable_285 == 0.0) {
        let noise_metadata_schedule_381_e4268: f64 = (noise_variable_113).exp();
        let noise_metadata_schedule_381_e4272: f64 = (noise_variable_108 - noise_variable_113);
        let noise_metadata_schedule_381_e4273: f64 = (1.0 + noise_metadata_schedule_381_e4272);
        let noise_metadata_schedule_381_e4274: f64 = (noise_metadata_schedule_381_e4268 * noise_metadata_schedule_381_e4273);
        (noise_metadata_schedule_381_e4274,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_381_e4276;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_382_e4280: f64 = (noise_variable_33 * noise_variable_109);
            let noise_metadata_schedule_382_e4281: f64 = (1.0 + noise_metadata_schedule_382_e4280);
            let noise_metadata_schedule_382_e4282: f64 = (noise_metadata_schedule_382_e4281).sqrt();
            noise_variable_103 = noise_metadata_schedule_382_e4282;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_383_e4286: f64 = (noise_variable_33 * noise_variable_111);
            let noise_metadata_schedule_383_e4287: f64 = (1.0 + noise_metadata_schedule_383_e4286);
            let noise_metadata_schedule_383_e4288: f64 = (noise_metadata_schedule_383_e4287).sqrt();
            noise_variable_104 = noise_metadata_schedule_383_e4288;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_385_e4294: f64 = (noise_variable_103 + 1.0);
            let noise_metadata_schedule_385_e4297: f64 = (noise_variable_104 + 1.0);
            let noise_metadata_schedule_385_e4298: f64 = (noise_metadata_schedule_385_e4294 / noise_metadata_schedule_385_e4297);
            noise_variable_105 = noise_metadata_schedule_385_e4298;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_386_e4303: f64 = (noise_variable_103 - noise_variable_104);
            let noise_metadata_schedule_386_e4305: f64 = (noise_variable_105).ln();
            let noise_metadata_schedule_386_e4306: f64 = (noise_metadata_schedule_386_e4303 - noise_metadata_schedule_386_e4305);
            let noise_metadata_schedule_386_e4307: f64 = (noise_variable_73 * noise_metadata_schedule_386_e4306);
            let noise_metadata_schedule_386_e4308: f64 = (noise_variable_154 + noise_metadata_schedule_386_e4307);
            let noise_metadata_schedule_386_e4310: f64 = (noise_metadata_schedule_386_e4308 * noise_variable_54);
            noise_variable_106 = noise_metadata_schedule_386_e4310;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_387_e4313: f64 = (noise_variable_48 * noise_variable_106);
            let noise_metadata_schedule_387_e4318: f64 = (0.5 * noise_variable_48);
            let noise_metadata_schedule_387_e4320: f64 = (noise_metadata_schedule_387_e4318 * noise_variable_49);
            let noise_metadata_schedule_387_e4323: f64 = (noise_variable_154 * noise_variable_154);
            let noise_metadata_schedule_387_e4325: f64 = (noise_metadata_schedule_387_e4323 + 0.01);
            let noise_metadata_schedule_387_e4326: f64 = (noise_metadata_schedule_387_e4325).sqrt();
            let noise_metadata_schedule_387_e4327: f64 = (noise_metadata_schedule_387_e4320 * noise_metadata_schedule_387_e4326);
            let noise_metadata_schedule_387_e4328: f64 = (1.0 + noise_metadata_schedule_387_e4327);
            let noise_metadata_schedule_387_e4329: f64 = (noise_variable_54 * noise_metadata_schedule_387_e4328);
            let noise_metadata_schedule_387_e4330: f64 = (noise_metadata_schedule_387_e4313 / noise_metadata_schedule_387_e4329);
            noise_variable_107 = noise_metadata_schedule_387_e4330;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_388_e4335: f64 = (noise_variable_107 * noise_variable_107);
            let noise_metadata_schedule_388_e4336: f64 = (1.0 + noise_metadata_schedule_388_e4335);
            let noise_metadata_schedule_388_e4337: f64 = (noise_metadata_schedule_388_e4336).sqrt();
            let noise_metadata_schedule_388_e4338: f64 = (noise_variable_106 / noise_metadata_schedule_388_e4337);
            noise_variable_97 = noise_metadata_schedule_388_e4338;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_429_e4707: f64 = (noise_variable_165 * noise_variable_143);
            let noise_metadata_schedule_429_e4708: f64 = (noise_variable_87 + noise_metadata_schedule_429_e4707);
            noise_variable_87 = noise_metadata_schedule_429_e4708;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_430_e4712: f64 = (noise_variable_165 * noise_variable_145);
            let noise_metadata_schedule_430_e4713: f64 = (noise_variable_88 + noise_metadata_schedule_430_e4712);
            noise_variable_88 = noise_metadata_schedule_430_e4713;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_431_e4717: f64 = (noise_variable_165 * noise_variable_146);
            let noise_metadata_schedule_431_e4718: f64 = (noise_variable_91 + noise_metadata_schedule_431_e4717);
            noise_variable_91 = noise_metadata_schedule_431_e4718;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_434_e4731: f64 = noise_variable_162;
            let noise_metadata_schedule_434_e4733: f64 = (noise_metadata_schedule_434_e4731 * noise_variable_87);
            noise_variable_87 = noise_metadata_schedule_434_e4733;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_435_e4736: f64 = noise_variable_162;
            let noise_metadata_schedule_435_e4738: f64 = (noise_metadata_schedule_435_e4736 * noise_variable_88);
            noise_variable_88 = noise_metadata_schedule_435_e4738;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_436_e4741: f64 = noise_variable_162;
            let noise_metadata_schedule_436_e4743: f64 = (noise_metadata_schedule_436_e4741 * noise_variable_76);
            noise_variable_76 = noise_metadata_schedule_436_e4743;
        }
        if matches!(source_index, 5 | 6) {
            let noise_metadata_schedule_441_e4766: f64 = noise_variable_162;
            let noise_metadata_schedule_441_e4768: f64 = (noise_metadata_schedule_441_e4766 * noise_variable_91);
            noise_variable_91 = noise_metadata_schedule_441_e4768;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_443_e4774: f64 = noise_variable_162;
            let noise_metadata_schedule_443_e4776: f64 = (noise_metadata_schedule_443_e4774 * noise_variable_97);
            noise_variable_97 = noise_metadata_schedule_443_e4776;
        }
        match source_index {
            0 => {
                let noise_0_psd_e5755: f64 = 1.0;
                let noise_0_psd_e178: f64 = 2.0;
                let noise_0_psd_e180: f64 = (noise_0_psd_e178 * 1.602189e-19);
                let noise_0_psd_e182: f64 = (noise_variable_87).abs();
                let noise_0_psd_e183: f64 = (noise_0_psd_e180 * noise_0_psd_e182);
                let noise_0_psd_e5756: f64 = (noise_0_psd_e5755 * noise_0_psd_e183);
                let psd = noise_0_psd_e5756;
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
                let noise_1_psd_e5758: f64 = 1.0;
                let noise_1_psd_e191: f64 = params.p98;
                let noise_1_psd_e194: f64 = noise_variable_87;
                let noise_1_psd_e195: f64 = (noise_1_psd_e194).abs();
                let noise_1_psd_e197: f64 = (noise_1_psd_e195).powf(params.p99);
                let noise_1_psd_e198: f64 = (noise_1_psd_e191 * noise_1_psd_e197);
                let noise_1_psd_e5759: f64 = (noise_1_psd_e5758 * noise_1_psd_e198);
                let psd = noise_1_psd_e5759;
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
                let noise_2_psd_e5761: f64 = 1.0;
                let noise_2_psd_e207: f64 = 2.0;
                let noise_2_psd_e209: f64 = (noise_2_psd_e207 * 1.602189e-19);
                let noise_2_psd_e211: f64 = (noise_variable_88).abs();
                let noise_2_psd_e212: f64 = (noise_2_psd_e209 * noise_2_psd_e211);
                let noise_2_psd_e5762: f64 = (noise_2_psd_e5761 * noise_2_psd_e212);
                let psd = noise_2_psd_e5762;
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
                let noise_3_psd_e5764: f64 = 1.0;
                let noise_3_psd_e220: f64 = params.p98;
                let noise_3_psd_e223: f64 = noise_variable_88;
                let noise_3_psd_e224: f64 = (noise_3_psd_e223).abs();
                let noise_3_psd_e226: f64 = (noise_3_psd_e224).powf(params.p99);
                let noise_3_psd_e227: f64 = (noise_3_psd_e220 * noise_3_psd_e226);
                let noise_3_psd_e5765: f64 = (noise_3_psd_e5764 * noise_3_psd_e227);
                let psd = noise_3_psd_e5765;
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
                let noise_4_psd_e5767: f64 = 1.0;
                let noise_4_psd_e236: f64 = 2.0;
                let noise_4_psd_e238: f64 = (noise_4_psd_e236 * 1.602189e-19);
                let noise_4_psd_e240: f64 = (noise_variable_76).abs();
                let noise_4_psd_e241: f64 = (noise_4_psd_e238 * noise_4_psd_e240);
                let noise_4_psd_e5768: f64 = (noise_4_psd_e5767 * noise_4_psd_e241);
                let psd = noise_4_psd_e5768;
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
                let noise_5_psd_e5770: f64 = 1.0;
                let noise_5_psd_e249: f64 = 2.0;
                let noise_5_psd_e251: f64 = (noise_5_psd_e249 * 1.602189e-19);
                let noise_5_psd_e253: f64 = (noise_variable_91).abs();
                let noise_5_psd_e254: f64 = (noise_5_psd_e251 * noise_5_psd_e253);
                let noise_5_psd_e5771: f64 = (noise_5_psd_e5770 * noise_5_psd_e254);
                let psd = noise_5_psd_e5771;
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
                let noise_6_psd_e5773: f64 = 1.0;
                let noise_6_psd_e262: f64 = 1.0;
                let noise_6_psd_e264: f64 = (noise_6_psd_e262 * params.p98);
                let noise_6_psd_e267: f64 = noise_variable_91;
                let noise_6_psd_e268: f64 = (noise_6_psd_e267).abs();
                let noise_6_psd_e270: f64 = (noise_6_psd_e268).powf(params.p99);
                let noise_6_psd_e271: f64 = (noise_6_psd_e264 * noise_6_psd_e270);
                let noise_6_psd_e5774: f64 = (noise_6_psd_e5773 * noise_6_psd_e271);
                let psd = noise_6_psd_e5774;
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
                let noise_7_psd_e5776: f64 = 1.0;
                let noise_7_psd_e280: f64 = 4.0;
                let noise_7_psd_e282: f64 = (noise_7_psd_e280 * 1.380662e-23);
                let noise_7_psd_e284: f64 = (noise_7_psd_e282 * noise_variable_39);
                let noise_7_psd_e286: f64 = (noise_7_psd_e284 * noise_variable_53);
                let noise_7_psd_e5777: f64 = (noise_7_psd_e5776 * noise_7_psd_e286);
                let psd = noise_7_psd_e5777;
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
                let noise_8_psd_e5779: f64 = 1.0;
                let noise_8_psd_e294: f64 = 4.0;
                let noise_8_psd_e296: f64 = (noise_8_psd_e294 * 1.380662e-23);
                let noise_8_psd_e298: f64 = (noise_8_psd_e296 * noise_variable_39);
                let noise_8_psd_e300: f64 = (noise_variable_97).abs();
                let noise_8_psd_e303: f64 = (1e-10 * noise_variable_54);
                let noise_8_psd_e304: f64 = (noise_8_psd_e300 + noise_8_psd_e303);
                let noise_8_psd_e306: f64 = (noise_variable_154).abs();
                let noise_8_psd_e308: f64 = (noise_8_psd_e306 + 1e-10);
                let noise_8_psd_e309: f64 = (noise_8_psd_e304 / noise_8_psd_e308);
                let noise_8_psd_e310: f64 = (noise_8_psd_e298 * noise_8_psd_e309);
                let noise_8_psd_e5780: f64 = (noise_8_psd_e5779 * noise_8_psd_e310);
                let psd = noise_8_psd_e5780;
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
                let noise_9_psd_e5782: f64 = 1.0;
                let noise_9_psd_e318: f64 = 4.0;
                let noise_9_psd_e320: f64 = (noise_9_psd_e318 * 1.380662e-23);
                let noise_9_psd_e322: f64 = (noise_9_psd_e320 * noise_variable_39);
                let noise_9_psd_e324: f64 = (noise_9_psd_e322 * noise_variable_55);
                let noise_9_psd_e5783: f64 = (noise_9_psd_e5782 * noise_9_psd_e324);
                let psd = noise_9_psd_e5783;
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
                let noise_10_psd_e5785: f64 = 1.0;
                let noise_10_psd_e332: f64 = 4.0;
                let noise_10_psd_e334: f64 = (noise_10_psd_e332 * 1.380662e-23);
                let noise_10_psd_e336: f64 = (noise_10_psd_e334 * noise_variable_39);
                let noise_10_psd_e338: f64 = (noise_10_psd_e336 * noise_variable_81);
                let noise_10_psd_e340: f64 = (noise_10_psd_e338 * noise_variable_56);
                let noise_10_psd_e5786: f64 = (noise_10_psd_e5785 * noise_10_psd_e340);
                let psd = noise_10_psd_e5786;
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
                let noise_11_psd_e5788: f64 = 1.0;
                let noise_11_psd_e348: f64 = 4.0;
                let noise_11_psd_e350: f64 = (noise_11_psd_e348 * 1.380662e-23);
                let noise_11_psd_e352: f64 = (noise_11_psd_e350 * noise_variable_39);
                let noise_11_psd_e354: f64 = (noise_11_psd_e352 * noise_variable_57);
                let noise_11_psd_e5789: f64 = (noise_11_psd_e5788 * noise_11_psd_e354);
                let psd = noise_11_psd_e5789;
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
                let noise_12_psd_e5791: f64 = 1.0;
                let noise_12_psd_e362: f64 = 4.0;
                let noise_12_psd_e364: f64 = (noise_12_psd_e362 * 1.380662e-23);
                let noise_12_psd_e366: f64 = (noise_12_psd_e364 * noise_variable_39);
                let noise_12_psd_e368: f64 = (noise_12_psd_e366 * noise_variable_86);
                let noise_12_psd_e370: f64 = (noise_12_psd_e368 * noise_variable_58);
                let noise_12_psd_e5792: f64 = (noise_12_psd_e5791 * noise_12_psd_e370);
                let psd = noise_12_psd_e5792;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 19] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BP_RBX", label: Some("rbx"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_BI_RBI", label: Some("rbi"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_C_RCX", label: Some("rcx"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_EI_E_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_S_RSU", label: Some("rsu"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BP_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_EI_E_FLICKER_RE", label: Some("flicker_re"), kind: GeneratedNoiseKind::Flicker, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEBTB", label: Some("ibebtb"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_EI_IBEP", label: Some("ibep"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_CI_IBCI", label: Some("ibci"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_CI_IBCBTB", label: Some("ibcbtb"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CI_IJBCX", label: Some("ijbcx"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_CI_IJSC", label: Some("ijsc"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_IBEI", label: Some("ibei"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "n1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 67, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "n2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 70, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI", label: Some("ibei"), kind: GeneratedNoiseKind::White, equation: 71, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 0) {
            let noise_activation_schedule_1111_e13006: f64 = if ((params.p90 >= params.p149) && (params.p90 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_525 = noise_activation_schedule_1111_e13006;
        }
        if matches!(source_index, 1) {
            let noise_activation_schedule_1112_e13013: f64 = if ((params.p89 >= params.p149) && (params.p89 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_526 = noise_activation_schedule_1112_e13013;
        }
        if matches!(source_index, 2) {
            let noise_activation_schedule_1113_e13020: f64 = if ((params.p96 >= params.p149) && (params.p96 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_527 = noise_activation_schedule_1113_e13020;
        }
        if matches!(source_index, 3) {
            let noise_activation_schedule_1114_e13027: f64 = if ((params.p95 >= params.p149) && (params.p95 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_528 = noise_activation_schedule_1114_e13027;
        }
        if matches!(source_index, 4) {
            let noise_activation_schedule_1115_e13034: f64 = if ((params.p102 >= params.p149) && (params.p102 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_529 = noise_activation_schedule_1115_e13034;
        }
        if matches!(source_index, 5 | 6) {
            let noise_activation_schedule_1117_e13045: f64 = (-1.0);
            let noise_activation_schedule_1117_e13046: f64 = if params.p112 == noise_activation_schedule_1117_e13045 { 1.0 } else { 0.0 };
            noise_variable_530 = noise_activation_schedule_1117_e13046;
        }
        if matches!(source_index, 7) {
            let noise_activation_schedule_1118_e13053: f64 = if ((params.p95 >= params.p149) && (params.p95 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_531 = noise_activation_schedule_1118_e13053;
        }
        if matches!(source_index, 8) {
            let noise_activation_schedule_1122_e13074: f64 = if params.p0 >= 320.0 { 1.0 } else { 0.0 };
            noise_variable_532 = noise_activation_schedule_1122_e13074;
        }
        if matches!(source_index, 15 | 16 | 17 | 18) {
            let noise_activation_schedule_1123_e13085: f64 = if ((params.p109 == 1.0) && ((params.p88 > 0.0) && (params.p87 > 0.0))) { 1.0 } else { 0.0 };
            noise_variable_533 = noise_activation_schedule_1123_e13085;
        }
        let noise_source_active = match source_index {
            0 => {
                noise_variable_525 != 0.0
            }
            1 => {
                noise_variable_526 != 0.0
            }
            2 => {
                noise_variable_527 != 0.0
            }
            3 => {
                noise_variable_528 != 0.0
            }
            4 => {
                noise_variable_529 != 0.0
            }
            5 => {
                noise_variable_530 != 0.0
            }
            6 => {
                let noise_6_activation_e448: f64 = if (noise_variable_530 == 0.0) { 1.0 } else { 0.0 };
                noise_6_activation_e448 != 0.0
            }
            7 => {
                noise_variable_531 != 0.0
            }
            8 => {
                noise_variable_532 != 0.0
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
                noise_variable_533 != 0.0
            }
            16 => {
                noise_variable_533 != 0.0
            }
            17 => {
                let noise_17_activation_e565: f64 = if (noise_variable_533 == 0.0) { 1.0 } else { 0.0 };
                noise_17_activation_e565 != 0.0
            }
            18 => {
                let noise_18_activation_e575: f64 = if (noise_variable_533 == 0.0) { 1.0 } else { 0.0 };
                noise_18_activation_e575 != 0.0
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
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_0_e596: f64 = (params.p148 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_202 = noise_metadata_schedule_0_e596;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 16 | 17) {
            let noise_metadata_schedule_1_e599: f64 = (params.p148 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_203 = noise_metadata_schedule_1_e599;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_2_e602: f64 = (noise_variable_202 - noise_variable_203);
            noise_variable_204 = noise_metadata_schedule_2_e602;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 12 | 13 | 14 | 16 | 17) {
            let noise_metadata_schedule_3_e605: f64 = (params.p148 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[6])));
            noise_variable_205 = noise_metadata_schedule_3_e605;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_4_e608: f64 = (params.p148 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_206 = noise_metadata_schedule_4_e608;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_6_e614: f64 = (params.p148 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_208 = noise_metadata_schedule_6_e614;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_8_e620: f64 = if params.p0 <= 310.0 { 1.0 } else { 0.0 };
            noise_variable_279 = noise_metadata_schedule_8_e620;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_9_e624,) = {
    if (noise_variable_279 != 0.0) {
        (1.6021918e-19,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_9_e624;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_10_e628,) = {
    if (noise_variable_279 != 0.0) {
        (1.3806226e-23,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_10_e628;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_11_e633,) = {
    if (noise_variable_279 == 0.0) {
        (1.602176634e-19,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_11_e633;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_12_e638,) = {
    if (noise_variable_279 == 0.0) {
        (1.380649e-23,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_12_e638;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_14_e644: f64 = (params.p146 + 273.15);
            noise_variable_8 = noise_metadata_schedule_14_e644;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_15_e645: f64 = ctx.temperature();
            noise_variable_9 = noise_metadata_schedule_15_e645;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_16_e648: f64 = (noise_variable_1 / noise_variable_0);
            noise_variable_2 = noise_metadata_schedule_16_e648;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_17_e651: f64 = (noise_variable_2 * 300.0);
            noise_variable_3 = noise_metadata_schedule_17_e651;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_18_e654: f64 = (noise_variable_2 * noise_variable_8);
            noise_variable_6 = noise_metadata_schedule_18_e654;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_19_e657: f64 = (1.0 / noise_variable_6);
            noise_variable_7 = noise_metadata_schedule_19_e657;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_20_e660: f64 = (params.p121 * noise_variable_8);
            let noise_metadata_schedule_20_e662: f64 = (noise_variable_8).ln();
            let noise_metadata_schedule_20_e663: f64 = (noise_metadata_schedule_20_e660 * noise_metadata_schedule_20_e662);
            noise_variable_276 = noise_metadata_schedule_20_e663;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_21_e666: f64 = (params.p122 * noise_variable_8);
            noise_variable_277 = noise_metadata_schedule_21_e666;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_22_e669: f64 = (params.p131 * noise_variable_8);
            noise_variable_56 = noise_metadata_schedule_22_e669;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_23_e672: f64 = (params.p117 + noise_variable_276);
            let noise_metadata_schedule_23_e674: f64 = (noise_metadata_schedule_23_e672 + noise_variable_277);
            noise_variable_88 = noise_metadata_schedule_23_e674;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_24_e677: f64 = (params.p118 + noise_variable_276);
            let noise_metadata_schedule_24_e679: f64 = (noise_metadata_schedule_24_e677 + noise_variable_277);
            noise_variable_89 = noise_metadata_schedule_24_e679;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_25_e682: f64 = (params.p119 + noise_variable_276);
            let noise_metadata_schedule_25_e684: f64 = (noise_metadata_schedule_25_e682 + noise_variable_277);
            noise_variable_90 = noise_metadata_schedule_25_e684;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_26_e687: f64 = (noise_variable_88 + noise_variable_89);
            let noise_metadata_schedule_26_e689: f64 = (noise_metadata_schedule_26_e687 * 0.5);
            noise_variable_91 = noise_metadata_schedule_26_e689;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_27_e692: f64 = (noise_variable_88 + noise_variable_90);
            let noise_metadata_schedule_27_e694: f64 = (noise_metadata_schedule_27_e692 * 0.5);
            noise_variable_92 = noise_metadata_schedule_27_e694;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_28_e697: f64 = (params.p117 + params.p118);
            let noise_metadata_schedule_28_e699: f64 = (noise_metadata_schedule_28_e697 * 0.5);
            noise_variable_77 = noise_metadata_schedule_28_e699;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_29_e702: f64 = (params.p117 + params.p119);
            let noise_metadata_schedule_29_e704: f64 = (noise_metadata_schedule_29_e702 * 0.5);
            noise_variable_78 = noise_metadata_schedule_29_e704;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_30_e707: f64 = (params.p120 + params.p119);
            let noise_metadata_schedule_30_e709: f64 = (noise_metadata_schedule_30_e707 * 0.5);
            noise_variable_79 = noise_metadata_schedule_30_e709;
        }
        if matches!(source_index, 1 | 8 | 10 | 11 | 12 | 13 | 14 | 16 | 17) {
            let noise_metadata_schedule_31_e713: f64 = (params.p121 / noise_variable_2);
            let noise_metadata_schedule_31_e714: f64 = (3.0 - noise_metadata_schedule_31_e713);
            noise_variable_76 = noise_metadata_schedule_31_e714;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_32_e717: f64 = (noise_variable_76 + 1.0);
            let noise_metadata_schedule_32_e719: f64 = (noise_metadata_schedule_32_e717 - params.p130);
            noise_variable_80 = noise_metadata_schedule_32_e719;
        }
        if matches!(source_index, 13) {
            let noise_metadata_schedule_33_e722: f64 = (noise_variable_76 + 1.0);
            let noise_metadata_schedule_33_e724: f64 = (noise_metadata_schedule_33_e722 - params.p138);
            noise_variable_81 = noise_metadata_schedule_33_e724;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_34_e727: f64 = (noise_variable_76 - 1.5);
            noise_variable_82 = noise_metadata_schedule_34_e727;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_50_e802: f64 = if params.p0 <= 300.0 { 1.0 } else { 0.0 };
            noise_variable_282 = noise_metadata_schedule_50_e802;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_51_e806,) = {
    if (noise_variable_282 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_223,)
    }
};
            noise_variable_223 = noise_metadata_schedule_51_e806;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_52_e811,) = {
    if (noise_variable_282 == 0.0) {
        (0.7,)
    } else {
        (noise_variable_223,)
    }
};
            noise_variable_223 = noise_metadata_schedule_52_e811;
        }
        if matches!(source_index, 10) {
            noise_variable_244 = 0.0;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_54_e819: f64 = if ((params.p32 > 0.0) && (params.p47 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_283 = noise_metadata_schedule_54_e819;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_55_e823,) = {
    if (noise_variable_283 != 0.0) {
        (1.0,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_55_e823;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_56_e828,) = {
    if (noise_variable_283 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_243,)
    }
};
            noise_variable_243 = noise_metadata_schedule_56_e828;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_61_e856: f64 = if ((params.p115 >= 0.01) || (params.p116 >= 0.01)) { 1.0 } else { 0.0 };
            noise_variable_286 = noise_metadata_schedule_61_e856;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_62_e864,) = {
    if (noise_variable_286 != 0.0) {
        let noise_metadata_schedule_62_e861: f64 = (params.p115 - params.p116);
        let noise_metadata_schedule_62_e862: f64 = (0.5 * noise_metadata_schedule_62_e861);
        (noise_metadata_schedule_62_e862,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_62_e864;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_63_e867: f64 = if params.p116 < params.p115 { 1.0 } else { 0.0 };
            noise_variable_287 = noise_metadata_schedule_63_e867;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_64_e873,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_287 != 0.0)) {
        (params.p116,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_64_e873;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_65_e879,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_287 != 0.0)) {
        (params.p115,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_65_e879;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_66_e886,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_287 == 0.0)) {
        (params.p115,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_66_e886;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_67_e893,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_287 == 0.0)) {
        (params.p116,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_67_e893;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_68_e896: f64 = if noise_variable_229 < 0.01 { 1.0 } else { 0.0 };
            noise_variable_288 = noise_metadata_schedule_68_e896;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_69_e902,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 != 0.0)) {
        (1000000000.0,)
    } else {
        (noise_variable_225,)
    }
};
            noise_variable_225 = noise_metadata_schedule_69_e902;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_70_e908,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 != 0.0)) {
        (1000000000.0,)
    } else {
        (noise_variable_226,)
    }
};
            noise_variable_226 = noise_metadata_schedule_70_e908;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_71_e914,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 != 0.0)) {
        (170000000.0,)
    } else {
        (noise_variable_227,)
    }
};
            noise_variable_227 = noise_metadata_schedule_71_e914;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_72_e920,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 != 0.0)) {
        (170000000.0,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_72_e920;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_73_e929,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 != 0.0)) {
        let noise_metadata_schedule_73_e926: f64 = (1.0 + noise_variable_230);
        let noise_metadata_schedule_73_e927: f64 = (noise_metadata_schedule_73_e926).ln();
        (noise_metadata_schedule_73_e927,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_73_e929;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_74_e938,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 == 0.0)) {
        let noise_metadata_schedule_74_e936: f64 = (1.0 / params.p115);
        (noise_metadata_schedule_74_e936,)
    } else {
        (noise_variable_225,)
    }
};
            noise_variable_225 = noise_metadata_schedule_74_e938;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_75_e947,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 == 0.0)) {
        let noise_metadata_schedule_75_e945: f64 = (1.0 / params.p116);
        (noise_metadata_schedule_75_e945,)
    } else {
        (noise_variable_226,)
    }
};
            noise_variable_226 = noise_metadata_schedule_75_e947;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_76_e956,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 == 0.0)) {
        let noise_metadata_schedule_76_e954: f64 = (params.p115 / 6.0);
        (noise_metadata_schedule_76_e954,)
    } else {
        (noise_variable_227,)
    }
};
            noise_variable_227 = noise_metadata_schedule_76_e956;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_77_e965,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 == 0.0)) {
        let noise_metadata_schedule_77_e963: f64 = (params.p116 / 6.0);
        (noise_metadata_schedule_77_e963,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_77_e965;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_78_e979,) = {
    if ((noise_variable_286 != 0.0) && (noise_variable_288 == 0.0)) {
        let noise_metadata_schedule_78_e972: f64 = (1.0 + params.p115);
        let noise_metadata_schedule_78_e975: f64 = (1.0 + params.p116);
        let noise_metadata_schedule_78_e976: f64 = (noise_metadata_schedule_78_e972 / noise_metadata_schedule_78_e975);
        let noise_metadata_schedule_78_e977: f64 = (noise_metadata_schedule_78_e976).ln();
        (noise_metadata_schedule_78_e977,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_78_e979;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_79_e984,) = {
    if (noise_variable_286 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_232,)
    }
};
            noise_variable_232 = noise_metadata_schedule_79_e984;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_80_e989,) = {
    if (noise_variable_286 == 0.0) {
        (1000000000.0,)
    } else {
        (noise_variable_225,)
    }
};
            noise_variable_225 = noise_metadata_schedule_80_e989;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_81_e994,) = {
    if (noise_variable_286 == 0.0) {
        (1000000000.0,)
    } else {
        (noise_variable_226,)
    }
};
            noise_variable_226 = noise_metadata_schedule_81_e994;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_82_e999,) = {
    if (noise_variable_286 == 0.0) {
        (170000000.0,)
    } else {
        (noise_variable_227,)
    }
};
            noise_variable_227 = noise_metadata_schedule_82_e999;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_83_e1004,) = {
    if (noise_variable_286 == 0.0) {
        (170000000.0,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_83_e1004;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_84_e1009,) = {
    if (noise_variable_286 == 0.0) {
        (params.p116,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_84_e1009;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_85_e1014,) = {
    if (noise_variable_286 == 0.0) {
        (params.p115,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_85_e1014;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_86_e1019,) = {
    if (noise_variable_286 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_231,)
    }
};
            noise_variable_231 = noise_metadata_schedule_86_e1019;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_87_e1022: f64 = (noise_variable_9 + params.p147);
            let noise_metadata_schedule_87_e1024: f64 = noise_metadata_schedule_87_e1022;
            noise_variable_10 = noise_metadata_schedule_87_e1024;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_88_e1027: f64 = (-200.0);
            let noise_metadata_schedule_88_e1029: f64 = (noise_metadata_schedule_88_e1027 + 273.15);
            let noise_metadata_schedule_88_e1030: f64 = if noise_variable_10 < noise_metadata_schedule_88_e1029 { 1.0 } else { 0.0 };
            noise_variable_289 = noise_metadata_schedule_88_e1030;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_89_e1037,) = {
    if (noise_variable_289 != 0.0) {
        let noise_metadata_schedule_89_e1033: f64 = (-200.0);
        let noise_metadata_schedule_89_e1035: f64 = (noise_metadata_schedule_89_e1033 + 273.15);
        (noise_metadata_schedule_89_e1035,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_89_e1037;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_90_e1041: f64 = (326.85 + 273.15);
            let noise_metadata_schedule_90_e1042: f64 = if noise_variable_10 > noise_metadata_schedule_90_e1041 { 1.0 } else { 0.0 };
            noise_variable_290 = noise_metadata_schedule_90_e1042;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_91_e1051,) = {
    if ((noise_variable_289 == 0.0) && (noise_variable_290 != 0.0)) {
        let noise_metadata_schedule_91_e1049: f64 = (326.85 + 273.15);
        (noise_metadata_schedule_91_e1049,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_91_e1051;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_92_e1054: f64 = (noise_variable_2 * noise_variable_10);
            noise_variable_4 = noise_metadata_schedule_92_e1054;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_93_e1057: f64 = (1.0 / noise_variable_4);
            noise_variable_5 = noise_metadata_schedule_93_e1057;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_94_e1060: f64 = (noise_variable_10 - noise_variable_8);
            noise_variable_14 = noise_metadata_schedule_94_e1060;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_95_e1063: f64 = (noise_variable_8 / noise_variable_10);
            noise_variable_12 = noise_metadata_schedule_95_e1063;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_96_e1066: f64 = (noise_variable_10 / noise_variable_8);
            noise_variable_11 = noise_metadata_schedule_96_e1066;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_97_e1068: f64 = (noise_variable_11).ln();
            noise_variable_13 = noise_metadata_schedule_97_e1068;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_98_e1071: f64 = (params.p121 * noise_variable_10);
            let noise_metadata_schedule_98_e1073: f64 = (noise_variable_10).ln();
            let noise_metadata_schedule_98_e1074: f64 = (noise_metadata_schedule_98_e1071 * noise_metadata_schedule_98_e1073);
            noise_variable_74 = noise_metadata_schedule_98_e1074;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_99_e1077: f64 = (params.p122 * noise_variable_10);
            noise_variable_75 = noise_metadata_schedule_99_e1077;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_100_e1080: f64 = (params.p117 + noise_variable_74);
            let noise_metadata_schedule_100_e1082: f64 = (noise_metadata_schedule_100_e1080 + noise_variable_75);
            noise_variable_84 = noise_metadata_schedule_100_e1082;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_101_e1085: f64 = (params.p118 + noise_variable_74);
            let noise_metadata_schedule_101_e1087: f64 = (noise_metadata_schedule_101_e1085 + noise_variable_75);
            noise_variable_83 = noise_metadata_schedule_101_e1087;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_102_e1090: f64 = (params.p119 + noise_variable_74);
            let noise_metadata_schedule_102_e1092: f64 = (noise_metadata_schedule_102_e1090 + noise_variable_75);
            noise_variable_85 = noise_metadata_schedule_102_e1092;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_103_e1095: f64 = (noise_variable_84 + noise_variable_83);
            let noise_metadata_schedule_103_e1097: f64 = (noise_metadata_schedule_103_e1095 * 0.5);
            noise_variable_86 = noise_metadata_schedule_103_e1097;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_104_e1100: f64 = (noise_variable_84 + noise_variable_85);
            let noise_metadata_schedule_104_e1102: f64 = (noise_metadata_schedule_104_e1100 * 0.5);
            noise_variable_87 = noise_metadata_schedule_104_e1102;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_105_e1105: f64 = if params.p39 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_291 = noise_metadata_schedule_105_e1105;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_106_e1127,) = {
    if (noise_variable_291 != 0.0) {
        let noise_metadata_schedule_106_e1109: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_106_e1112: f64 = (params.p40 * 0.5);
        let noise_metadata_schedule_106_e1114: f64 = (noise_metadata_schedule_106_e1112 * noise_variable_7);
        let noise_metadata_schedule_106_e1115: f64 = (noise_metadata_schedule_106_e1114).exp();
        let noise_metadata_schedule_106_e1117: f64 = (-0.5);
        let noise_metadata_schedule_106_e1119: f64 = (noise_metadata_schedule_106_e1117 * params.p40);
        let noise_metadata_schedule_106_e1121: f64 = (noise_metadata_schedule_106_e1119 * noise_variable_7);
        let noise_metadata_schedule_106_e1122: f64 = (noise_metadata_schedule_106_e1121).exp();
        let noise_metadata_schedule_106_e1123: f64 = (noise_metadata_schedule_106_e1115 - noise_metadata_schedule_106_e1122);
        let noise_metadata_schedule_106_e1124: f64 = (noise_metadata_schedule_106_e1123).ln();
        let noise_metadata_schedule_106_e1125: f64 = (noise_metadata_schedule_106_e1109 * noise_metadata_schedule_106_e1124);
        (noise_metadata_schedule_106_e1125,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_106_e1127;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_107_e1145,) = {
    if (noise_variable_291 != 0.0) {
        let noise_metadata_schedule_107_e1131: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_107_e1135: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_107_e1136: f64 = (noise_variable_77 * noise_metadata_schedule_107_e1135);
        let noise_metadata_schedule_107_e1137: f64 = (noise_metadata_schedule_107_e1131 + noise_metadata_schedule_107_e1136);
        let noise_metadata_schedule_107_e1140: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_107_e1142: f64 = (noise_metadata_schedule_107_e1140 * noise_variable_13);
        let noise_metadata_schedule_107_e1143: f64 = (noise_metadata_schedule_107_e1137 - noise_metadata_schedule_107_e1142);
        (noise_metadata_schedule_107_e1143,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_107_e1145;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_108_e1169,) = {
    if (noise_variable_291 != 0.0) {
        let noise_metadata_schedule_108_e1150: f64 = (2.0 * noise_variable_4);
        let noise_metadata_schedule_108_e1156: f64 = (-noise_variable_165);
        let noise_metadata_schedule_108_e1158: f64 = (noise_metadata_schedule_108_e1156 * noise_variable_5);
        let noise_metadata_schedule_108_e1159: f64 = (noise_metadata_schedule_108_e1158).exp();
        let noise_metadata_schedule_108_e1160: f64 = (4.0 * noise_metadata_schedule_108_e1159);
        let noise_metadata_schedule_108_e1161: f64 = (1.0 + noise_metadata_schedule_108_e1160);
        let noise_metadata_schedule_108_e1162: f64 = (noise_metadata_schedule_108_e1161).sqrt();
        let noise_metadata_schedule_108_e1163: f64 = (1.0 + noise_metadata_schedule_108_e1162);
        let noise_metadata_schedule_108_e1164: f64 = (0.5 * noise_metadata_schedule_108_e1163);
        let noise_metadata_schedule_108_e1165: f64 = (noise_metadata_schedule_108_e1164).ln();
        let noise_metadata_schedule_108_e1166: f64 = (noise_metadata_schedule_108_e1150 * noise_metadata_schedule_108_e1165);
        let noise_metadata_schedule_108_e1167: f64 = (noise_variable_165 + noise_metadata_schedule_108_e1166);
        (noise_metadata_schedule_108_e1167,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_108_e1169;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_109_e1181,) = {
    if (noise_variable_291 != 0.0) {
        let noise_metadata_schedule_109_e1175: f64 = (params.p40 / noise_variable_27);
        let noise_metadata_schedule_109_e1176: f64 = (noise_metadata_schedule_109_e1175).ln();
        let noise_metadata_schedule_109_e1177: f64 = (params.p41 * noise_metadata_schedule_109_e1176);
        let noise_metadata_schedule_109_e1178: f64 = (noise_metadata_schedule_109_e1177).exp();
        let noise_metadata_schedule_109_e1179: f64 = (params.p39 * noise_metadata_schedule_109_e1178);
        (noise_metadata_schedule_109_e1179,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_109_e1181;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_110_e1186,) = {
    if (noise_variable_291 != 0.0) {
        let noise_metadata_schedule_110_e1184: f64 = (params.p42).abs();
        (noise_metadata_schedule_110_e1184,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_110_e1186;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_111_e1189: f64 = if params.p42 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_292 = noise_metadata_schedule_111_e1189;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_112_e1199,) = {
    if ((noise_variable_291 != 0.0) && (noise_variable_292 != 0.0)) {
        let noise_metadata_schedule_112_e1195: f64 = (params.p42 * noise_variable_27);
        let noise_metadata_schedule_112_e1197: f64 = (noise_metadata_schedule_112_e1195 / params.p40);
        (noise_metadata_schedule_112_e1197,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_112_e1199;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_113_e1204,) = {
    if (noise_variable_291 == 0.0) {
        (params.p39,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_113_e1204;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_114_e1209,) = {
    if (noise_variable_291 == 0.0) {
        (params.p40,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_114_e1209;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_115_e1214,) = {
    if (noise_variable_291 == 0.0) {
        (params.p42,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_115_e1214;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 15 | 18) {
            let noise_metadata_schedule_116_e1218: f64 = (params.p124 * noise_variable_13);
            let noise_metadata_schedule_116_e1221: f64 = (params.p118 * noise_variable_7);
            let noise_metadata_schedule_116_e1224: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_116_e1225: f64 = (noise_metadata_schedule_116_e1221 * noise_metadata_schedule_116_e1224);
            let noise_metadata_schedule_116_e1226: f64 = (noise_metadata_schedule_116_e1218 + noise_metadata_schedule_116_e1225);
            let noise_metadata_schedule_116_e1227: f64 = (noise_metadata_schedule_116_e1226).exp();
            let noise_metadata_schedule_116_e1228: f64 = (params.p14 * noise_metadata_schedule_116_e1227);
            noise_variable_22 = noise_metadata_schedule_116_e1228;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_118_e1249: f64 = if params.p47 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_293 = noise_metadata_schedule_118_e1249;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_119_e1271,) = {
    if (noise_variable_293 != 0.0) {
        let noise_metadata_schedule_119_e1253: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_119_e1256: f64 = (params.p48 * 0.5);
        let noise_metadata_schedule_119_e1258: f64 = (noise_metadata_schedule_119_e1256 * noise_variable_7);
        let noise_metadata_schedule_119_e1259: f64 = (noise_metadata_schedule_119_e1258).exp();
        let noise_metadata_schedule_119_e1261: f64 = (-0.5);
        let noise_metadata_schedule_119_e1263: f64 = (noise_metadata_schedule_119_e1261 * params.p48);
        let noise_metadata_schedule_119_e1265: f64 = (noise_metadata_schedule_119_e1263 * noise_variable_7);
        let noise_metadata_schedule_119_e1266: f64 = (noise_metadata_schedule_119_e1265).exp();
        let noise_metadata_schedule_119_e1267: f64 = (noise_metadata_schedule_119_e1259 - noise_metadata_schedule_119_e1266);
        let noise_metadata_schedule_119_e1268: f64 = (noise_metadata_schedule_119_e1267).ln();
        let noise_metadata_schedule_119_e1269: f64 = (noise_metadata_schedule_119_e1253 * noise_metadata_schedule_119_e1268);
        (noise_metadata_schedule_119_e1269,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_119_e1271;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_120_e1289,) = {
    if (noise_variable_293 != 0.0) {
        let noise_metadata_schedule_120_e1275: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_120_e1279: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_120_e1280: f64 = (noise_variable_78 * noise_metadata_schedule_120_e1279);
        let noise_metadata_schedule_120_e1281: f64 = (noise_metadata_schedule_120_e1275 + noise_metadata_schedule_120_e1280);
        let noise_metadata_schedule_120_e1284: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_120_e1286: f64 = (noise_metadata_schedule_120_e1284 * noise_variable_13);
        let noise_metadata_schedule_120_e1287: f64 = (noise_metadata_schedule_120_e1281 - noise_metadata_schedule_120_e1286);
        (noise_metadata_schedule_120_e1287,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_120_e1289;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_121_e1313,) = {
    if (noise_variable_293 != 0.0) {
        let noise_metadata_schedule_121_e1294: f64 = (2.0 * noise_variable_4);
        let noise_metadata_schedule_121_e1300: f64 = (-noise_variable_165);
        let noise_metadata_schedule_121_e1302: f64 = (noise_metadata_schedule_121_e1300 * noise_variable_5);
        let noise_metadata_schedule_121_e1303: f64 = (noise_metadata_schedule_121_e1302).exp();
        let noise_metadata_schedule_121_e1304: f64 = (4.0 * noise_metadata_schedule_121_e1303);
        let noise_metadata_schedule_121_e1305: f64 = (1.0 + noise_metadata_schedule_121_e1304);
        let noise_metadata_schedule_121_e1306: f64 = (noise_metadata_schedule_121_e1305).sqrt();
        let noise_metadata_schedule_121_e1307: f64 = (1.0 + noise_metadata_schedule_121_e1306);
        let noise_metadata_schedule_121_e1308: f64 = (0.5 * noise_metadata_schedule_121_e1307);
        let noise_metadata_schedule_121_e1309: f64 = (noise_metadata_schedule_121_e1308).ln();
        let noise_metadata_schedule_121_e1310: f64 = (noise_metadata_schedule_121_e1294 * noise_metadata_schedule_121_e1309);
        let noise_metadata_schedule_121_e1311: f64 = (noise_variable_165 + noise_metadata_schedule_121_e1310);
        (noise_metadata_schedule_121_e1311,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_121_e1313;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_122_e1325,) = {
    if (noise_variable_293 != 0.0) {
        let noise_metadata_schedule_122_e1319: f64 = (params.p48 / noise_variable_34);
        let noise_metadata_schedule_122_e1320: f64 = (noise_metadata_schedule_122_e1319).ln();
        let noise_metadata_schedule_122_e1321: f64 = (params.p49 * noise_metadata_schedule_122_e1320);
        let noise_metadata_schedule_122_e1322: f64 = (noise_metadata_schedule_122_e1321).exp();
        let noise_metadata_schedule_122_e1323: f64 = (params.p47 * noise_metadata_schedule_122_e1322);
        (noise_metadata_schedule_122_e1323,)
    } else {
        (noise_variable_33,)
    }
};
            noise_variable_33 = noise_metadata_schedule_122_e1325;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_123_e1330,) = {
    if (noise_variable_293 != 0.0) {
        let noise_metadata_schedule_123_e1328: f64 = (params.p50).abs();
        (noise_metadata_schedule_123_e1328,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_123_e1330;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_124_e1333: f64 = if params.p50 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_294 = noise_metadata_schedule_124_e1333;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_125_e1343,) = {
    if ((noise_variable_293 != 0.0) && (noise_variable_294 != 0.0)) {
        let noise_metadata_schedule_125_e1339: f64 = (params.p50 * noise_variable_34);
        let noise_metadata_schedule_125_e1341: f64 = (noise_metadata_schedule_125_e1339 / params.p48);
        (noise_metadata_schedule_125_e1341,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_125_e1343;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_126_e1348,) = {
    if (noise_variable_293 == 0.0) {
        (params.p47,)
    } else {
        (noise_variable_33,)
    }
};
            noise_variable_33 = noise_metadata_schedule_126_e1348;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_127_e1353,) = {
    if (noise_variable_293 == 0.0) {
        (params.p48,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_127_e1353;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_128_e1358,) = {
    if (noise_variable_293 == 0.0) {
        (params.p50,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_128_e1358;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_129_e1361: f64 = if params.p0 <= 300.0 { 1.0 } else { 0.0 };
            noise_variable_295 = noise_metadata_schedule_129_e1361;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_130_e1365,) = {
    if (noise_variable_295 != 0.0) {
        (2.4,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_130_e1365;
        }
        if matches!(source_index, 11) {
            let noise_metadata_schedule_131_e1369: f64 = (noise_variable_80 * noise_variable_13);
            let noise_metadata_schedule_131_e1372: f64 = (params.p119 * noise_variable_7);
            let noise_metadata_schedule_131_e1375: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_131_e1376: f64 = (noise_metadata_schedule_131_e1372 * noise_metadata_schedule_131_e1375);
            let noise_metadata_schedule_131_e1377: f64 = (noise_metadata_schedule_131_e1369 + noise_metadata_schedule_131_e1376);
            let noise_metadata_schedule_131_e1378: f64 = (noise_metadata_schedule_131_e1377).exp();
            let noise_metadata_schedule_131_e1379: f64 = (params.p23 * noise_metadata_schedule_131_e1378);
            noise_variable_32 = noise_metadata_schedule_131_e1379;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_132_e1385: f64 = (noise_variable_27 / params.p40);
            let noise_metadata_schedule_132_e1386: f64 = (noise_metadata_schedule_132_e1385).ln();
            let noise_metadata_schedule_132_e1387: f64 = (params.p41 * noise_metadata_schedule_132_e1386);
            let noise_metadata_schedule_132_e1388: f64 = (noise_metadata_schedule_132_e1387).exp();
            let noise_metadata_schedule_132_e1389: f64 = (2.0 - noise_metadata_schedule_132_e1388);
            let noise_metadata_schedule_132_e1390: f64 = (params.p2 * noise_metadata_schedule_132_e1389);
            noise_variable_16 = noise_metadata_schedule_132_e1390;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_133_e1394: f64 = (params.p123 * noise_variable_13);
            let noise_metadata_schedule_133_e1397: f64 = (params.p117 * noise_variable_7);
            let noise_metadata_schedule_133_e1400: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_133_e1401: f64 = (noise_metadata_schedule_133_e1397 * noise_metadata_schedule_133_e1400);
            let noise_metadata_schedule_133_e1402: f64 = (noise_metadata_schedule_133_e1394 + noise_metadata_schedule_133_e1401);
            let noise_metadata_schedule_133_e1403: f64 = (noise_metadata_schedule_133_e1402).exp();
            let noise_metadata_schedule_133_e1404: f64 = (params.p1 * noise_metadata_schedule_133_e1403);
            noise_variable_15 = noise_metadata_schedule_133_e1404;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_134_e1408: f64 = (params.p126 * noise_variable_13);
            let noise_metadata_schedule_134_e1409: f64 = (noise_metadata_schedule_134_e1408).exp();
            let noise_metadata_schedule_134_e1410: f64 = (params.p10 * noise_metadata_schedule_134_e1409);
            noise_variable_18 = noise_metadata_schedule_134_e1410;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_135_e1416: f64 = (params.p8 - 1.0);
            let noise_metadata_schedule_135_e1417: f64 = (noise_metadata_schedule_135_e1416).abs();
            let noise_metadata_schedule_135_e1420: f64 = if ((params.p0 <= 300.0) && (noise_metadata_schedule_135_e1417 < 1e-5)) { 1.0 } else { 0.0 };
            noise_variable_296 = noise_metadata_schedule_135_e1420;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_136_e1436,) = {
    if (noise_variable_296 != 0.0) {
        let noise_metadata_schedule_136_e1425: f64 = (params.p125 * noise_variable_5);
        let noise_metadata_schedule_136_e1428: f64 = (params.p127 * noise_variable_13);
        let noise_metadata_schedule_136_e1429: f64 = (noise_metadata_schedule_136_e1428).exp();
        let noise_metadata_schedule_136_e1431: f64 = (noise_metadata_schedule_136_e1429 - 1.0);
        let noise_metadata_schedule_136_e1432: f64 = (noise_metadata_schedule_136_e1425 * noise_metadata_schedule_136_e1431);
        let noise_metadata_schedule_136_e1433: f64 = (noise_metadata_schedule_136_e1432).exp();
        let noise_metadata_schedule_136_e1434: f64 = (params.p9 * noise_metadata_schedule_136_e1433);
        (noise_metadata_schedule_136_e1434,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_136_e1436;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_137_e1453,) = {
    if (noise_variable_296 == 0.0) {
        let noise_metadata_schedule_137_e1442: f64 = (params.p125 * noise_variable_5);
        let noise_metadata_schedule_137_e1445: f64 = (params.p127 * noise_variable_13);
        let noise_metadata_schedule_137_e1446: f64 = (noise_metadata_schedule_137_e1445).exp();
        let noise_metadata_schedule_137_e1448: f64 = (noise_metadata_schedule_137_e1446 - 1.0);
        let noise_metadata_schedule_137_e1449: f64 = (noise_metadata_schedule_137_e1442 * noise_metadata_schedule_137_e1448);
        let noise_metadata_schedule_137_e1450: f64 = (noise_metadata_schedule_137_e1449).exp();
        let noise_metadata_schedule_137_e1451: f64 = (params.p8 * noise_metadata_schedule_137_e1450);
        (noise_metadata_schedule_137_e1451,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_137_e1453;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_138_e1457: f64 = (params.p125 * noise_variable_7);
            let noise_metadata_schedule_138_e1460: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_138_e1461: f64 = (noise_metadata_schedule_138_e1457 * noise_metadata_schedule_138_e1460);
            let noise_metadata_schedule_138_e1462: f64 = (noise_metadata_schedule_138_e1461).exp();
            let noise_metadata_schedule_138_e1463: f64 = (params.p3 * noise_metadata_schedule_138_e1462);
            noise_variable_19 = noise_metadata_schedule_138_e1463;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_139_e1467: f64 = (params.p117 - params.p118);
            let noise_metadata_schedule_139_e1469: f64 = (noise_metadata_schedule_139_e1467 * noise_variable_7);
            let noise_metadata_schedule_139_e1472: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_139_e1473: f64 = (noise_metadata_schedule_139_e1469 * noise_metadata_schedule_139_e1472);
            let noise_metadata_schedule_139_e1474: f64 = (noise_metadata_schedule_139_e1473).exp();
            let noise_metadata_schedule_139_e1475: f64 = (params.p4 * noise_metadata_schedule_139_e1474);
            noise_variable_20 = noise_metadata_schedule_139_e1475;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_140_e1479: f64 = (params.p117 - params.p119);
            let noise_metadata_schedule_140_e1481: f64 = (noise_metadata_schedule_140_e1479 * noise_variable_7);
            let noise_metadata_schedule_140_e1484: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_140_e1485: f64 = (noise_metadata_schedule_140_e1481 * noise_metadata_schedule_140_e1484);
            let noise_metadata_schedule_140_e1486: f64 = (noise_metadata_schedule_140_e1485).exp();
            let noise_metadata_schedule_140_e1487: f64 = (params.p6 * noise_metadata_schedule_140_e1486);
            noise_variable_21 = noise_metadata_schedule_140_e1487;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_141_e1491: f64 = (params.p130 - noise_variable_56);
            let noise_metadata_schedule_141_e1493: f64 = (noise_metadata_schedule_141_e1491 * noise_variable_13);
            let noise_metadata_schedule_141_e1494: f64 = (noise_metadata_schedule_141_e1493).exp();
            let noise_metadata_schedule_141_e1495: f64 = (params.p75 * noise_metadata_schedule_141_e1494);
            noise_variable_55 = noise_metadata_schedule_141_e1495;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_142_e1499: f64 = (params.p130 * noise_variable_13);
            let noise_metadata_schedule_142_e1500: f64 = (noise_metadata_schedule_142_e1499).exp();
            let noise_metadata_schedule_142_e1501: f64 = (params.p74 * noise_metadata_schedule_142_e1500);
            noise_variable_53 = noise_metadata_schedule_142_e1501;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_143_e1504: f64 = (1.0 / noise_variable_53);
            noise_variable_54 = noise_metadata_schedule_143_e1504;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_144_e1507: f64 = if params.p79 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_297 = noise_metadata_schedule_144_e1507;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_145_e1517,) = {
    if (noise_variable_297 != 0.0) {
        let noise_metadata_schedule_145_e1513: f64 = (params.p133 * noise_variable_14);
        let noise_metadata_schedule_145_e1514: f64 = (1.0 - noise_metadata_schedule_145_e1513);
        let noise_metadata_schedule_145_e1515: f64 = (params.p79 * noise_metadata_schedule_145_e1514);
        (noise_metadata_schedule_145_e1515,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_145_e1517;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_146_e1521,) = {
    if (noise_variable_297 != 0.0) {
        (params.p78,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_146_e1521;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_147_e1532,) = {
    if (noise_variable_297 == 0.0) {
        let noise_metadata_schedule_147_e1528: f64 = (params.p132 * noise_variable_14);
        let noise_metadata_schedule_147_e1529: f64 = (1.0 + noise_metadata_schedule_147_e1528);
        let noise_metadata_schedule_147_e1530: f64 = (params.p78 * noise_metadata_schedule_147_e1529);
        (noise_metadata_schedule_147_e1530,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_147_e1532;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_148_e1537,) = {
    if (noise_variable_297 == 0.0) {
        (params.p79,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_148_e1537;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_149_e1542: f64 = (params.p128 * noise_variable_14);
            let noise_metadata_schedule_149_e1543: f64 = (1.0 + noise_metadata_schedule_149_e1542);
            let noise_metadata_schedule_149_e1546: f64 = (params.p129 * noise_variable_14);
            let noise_metadata_schedule_149_e1548: f64 = (noise_metadata_schedule_149_e1546 * noise_variable_14);
            let noise_metadata_schedule_149_e1549: f64 = (noise_metadata_schedule_149_e1543 + noise_metadata_schedule_149_e1548);
            let noise_metadata_schedule_149_e1550: f64 = (params.p66 * noise_metadata_schedule_149_e1549);
            noise_variable_59 = noise_metadata_schedule_149_e1550;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            noise_variable_61 = params.p69;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_151_e1555: f64 = (params.p130 - 1.0);
            let noise_metadata_schedule_151_e1557: f64 = (noise_metadata_schedule_151_e1555 * noise_variable_13);
            let noise_metadata_schedule_151_e1558: f64 = (noise_metadata_schedule_151_e1557).exp();
            let noise_metadata_schedule_151_e1559: f64 = (params.p71 * noise_metadata_schedule_151_e1558);
            noise_variable_60 = noise_metadata_schedule_151_e1559;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_152_e1562: f64 = if noise_variable_243 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_298 = noise_metadata_schedule_152_e1562;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_153_e1571,) = {
    if (noise_variable_298 != 0.0) {
        let noise_metadata_schedule_153_e1567: f64 = (params.p139 * noise_variable_14);
        let noise_metadata_schedule_153_e1568: f64 = (noise_metadata_schedule_153_e1567).exp();
        let noise_metadata_schedule_153_e1569: f64 = (params.p32 * noise_metadata_schedule_153_e1568);
        (noise_metadata_schedule_153_e1569,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_153_e1571;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_154_e1580,) = {
    if (noise_variable_298 != 0.0) {
        let noise_metadata_schedule_154_e1576: f64 = (params.p140 * noise_variable_14);
        let noise_metadata_schedule_154_e1577: f64 = (noise_metadata_schedule_154_e1576).exp();
        let noise_metadata_schedule_154_e1578: f64 = (params.p33 * noise_metadata_schedule_154_e1577);
        (noise_metadata_schedule_154_e1578,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_154_e1580;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_155_e1585,) = {
    if (noise_variable_298 == 0.0) {
        (params.p32,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_155_e1585;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_156_e1590,) = {
    if (noise_variable_298 == 0.0) {
        (params.p33,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_156_e1590;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_157_e1597: f64 = if ((params.p37 > 0.0) && (noise_variable_203 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_299 = noise_metadata_schedule_157_e1597;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_158_e1601,) = {
    if (noise_variable_299 != 0.0) {
        (params.p37,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_158_e1601;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_159_e1605,) = {
    if (noise_variable_299 != 0.0) {
        (params.p38,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_159_e1605;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_160_e1612: f64 = if ((params.p47 > 0.0) && (params.p48 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_300 = noise_metadata_schedule_160_e1612;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_161_e1620,) = {
    if ((noise_variable_299 != 0.0) && (noise_variable_300 != 0.0)) {
        let noise_metadata_schedule_161_e1618: f64 = (noise_variable_92 / noise_variable_87);
        (noise_metadata_schedule_161_e1618,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_161_e1620;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_162_e1628,) = {
    if ((noise_variable_299 != 0.0) && (noise_variable_300 != 0.0)) {
        let noise_metadata_schedule_162_e1626: f64 = (noise_variable_34 / params.p48);
        (noise_metadata_schedule_162_e1626,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_162_e1628;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_163_e1641,) = {
    if ((noise_variable_299 != 0.0) && (noise_variable_300 != 0.0)) {
        let noise_metadata_schedule_163_e1633: f64 = (noise_variable_169).sqrt();
        let noise_metadata_schedule_163_e1635: f64 = (noise_metadata_schedule_163_e1633 * noise_variable_170);
        let noise_metadata_schedule_163_e1637: f64 = (noise_metadata_schedule_163_e1635 * noise_variable_33);
        let noise_metadata_schedule_163_e1639: f64 = (noise_metadata_schedule_163_e1637 / params.p47);
        (noise_metadata_schedule_163_e1639,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_163_e1641;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_164_e1651,) = {
    if ((noise_variable_299 != 0.0) && (noise_variable_300 != 0.0)) {
        let noise_metadata_schedule_164_e1647: f64 = (params.p37 * noise_variable_168);
        let noise_metadata_schedule_164_e1649: f64 = (noise_metadata_schedule_164_e1647 * noise_variable_170);
        (noise_metadata_schedule_164_e1649,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_164_e1651;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_165_e1661,) = {
    if ((noise_variable_299 != 0.0) && (noise_variable_300 != 0.0)) {
        let noise_metadata_schedule_165_e1658: f64 = (noise_variable_168 * noise_variable_169);
        let noise_metadata_schedule_165_e1659: f64 = (params.p38 / noise_metadata_schedule_165_e1658);
        (noise_metadata_schedule_165_e1659,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_165_e1661;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_166_e1666,) = {
    if (noise_variable_299 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_166_e1666;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_167_e1671,) = {
    if (noise_variable_299 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_167_e1671;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_168_e1675: f64 = (params.p134 * noise_variable_13);
            let noise_metadata_schedule_168_e1676: f64 = (noise_metadata_schedule_168_e1675).exp();
            let noise_metadata_schedule_168_e1677: f64 = (params.p89 * noise_metadata_schedule_168_e1676);
            noise_variable_69 = noise_metadata_schedule_168_e1677;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_169_e1680: f64 = if params.p43 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_301 = noise_metadata_schedule_169_e1680;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_170_e1702,) = {
    if (noise_variable_301 != 0.0) {
        let noise_metadata_schedule_170_e1684: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_170_e1687: f64 = (params.p44 * 0.5);
        let noise_metadata_schedule_170_e1689: f64 = (noise_metadata_schedule_170_e1687 * noise_variable_7);
        let noise_metadata_schedule_170_e1690: f64 = (noise_metadata_schedule_170_e1689).exp();
        let noise_metadata_schedule_170_e1692: f64 = (-0.5);
        let noise_metadata_schedule_170_e1694: f64 = (noise_metadata_schedule_170_e1692 * params.p44);
        let noise_metadata_schedule_170_e1696: f64 = (noise_metadata_schedule_170_e1694 * noise_variable_7);
        let noise_metadata_schedule_170_e1697: f64 = (noise_metadata_schedule_170_e1696).exp();
        let noise_metadata_schedule_170_e1698: f64 = (noise_metadata_schedule_170_e1690 - noise_metadata_schedule_170_e1697);
        let noise_metadata_schedule_170_e1699: f64 = (noise_metadata_schedule_170_e1698).ln();
        let noise_metadata_schedule_170_e1700: f64 = (noise_metadata_schedule_170_e1684 * noise_metadata_schedule_170_e1699);
        (noise_metadata_schedule_170_e1700,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_170_e1702;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_171_e1720,) = {
    if (noise_variable_301 != 0.0) {
        let noise_metadata_schedule_171_e1706: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_171_e1710: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_171_e1711: f64 = (noise_variable_77 * noise_metadata_schedule_171_e1710);
        let noise_metadata_schedule_171_e1712: f64 = (noise_metadata_schedule_171_e1706 + noise_metadata_schedule_171_e1711);
        let noise_metadata_schedule_171_e1715: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_171_e1717: f64 = (noise_metadata_schedule_171_e1715 * noise_variable_13);
        let noise_metadata_schedule_171_e1718: f64 = (noise_metadata_schedule_171_e1712 - noise_metadata_schedule_171_e1717);
        (noise_metadata_schedule_171_e1718,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_171_e1720;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_172_e1744,) = {
    if (noise_variable_301 != 0.0) {
        let noise_metadata_schedule_172_e1725: f64 = (2.0 * noise_variable_4);
        let noise_metadata_schedule_172_e1731: f64 = (-noise_variable_165);
        let noise_metadata_schedule_172_e1733: f64 = (noise_metadata_schedule_172_e1731 * noise_variable_5);
        let noise_metadata_schedule_172_e1734: f64 = (noise_metadata_schedule_172_e1733).exp();
        let noise_metadata_schedule_172_e1735: f64 = (4.0 * noise_metadata_schedule_172_e1734);
        let noise_metadata_schedule_172_e1736: f64 = (1.0 + noise_metadata_schedule_172_e1735);
        let noise_metadata_schedule_172_e1737: f64 = (noise_metadata_schedule_172_e1736).sqrt();
        let noise_metadata_schedule_172_e1738: f64 = (1.0 + noise_metadata_schedule_172_e1737);
        let noise_metadata_schedule_172_e1739: f64 = (0.5 * noise_metadata_schedule_172_e1738);
        let noise_metadata_schedule_172_e1740: f64 = (noise_metadata_schedule_172_e1739).ln();
        let noise_metadata_schedule_172_e1741: f64 = (noise_metadata_schedule_172_e1725 * noise_metadata_schedule_172_e1740);
        let noise_metadata_schedule_172_e1742: f64 = (noise_variable_165 + noise_metadata_schedule_172_e1741);
        (noise_metadata_schedule_172_e1742,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_172_e1744;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_173_e1756,) = {
    if (noise_variable_301 != 0.0) {
        let noise_metadata_schedule_173_e1750: f64 = (params.p44 / noise_variable_30);
        let noise_metadata_schedule_173_e1751: f64 = (noise_metadata_schedule_173_e1750).ln();
        let noise_metadata_schedule_173_e1752: f64 = (params.p45 * noise_metadata_schedule_173_e1751);
        let noise_metadata_schedule_173_e1753: f64 = (noise_metadata_schedule_173_e1752).exp();
        let noise_metadata_schedule_173_e1754: f64 = (params.p43 * noise_metadata_schedule_173_e1753);
        (noise_metadata_schedule_173_e1754,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_173_e1756;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_174_e1761,) = {
    if (noise_variable_301 != 0.0) {
        let noise_metadata_schedule_174_e1759: f64 = (params.p46).abs();
        (noise_metadata_schedule_174_e1759,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_174_e1761;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_175_e1764: f64 = if params.p46 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_302 = noise_metadata_schedule_175_e1764;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_176_e1774,) = {
    if ((noise_variable_301 != 0.0) && (noise_variable_302 != 0.0)) {
        let noise_metadata_schedule_176_e1770: f64 = (params.p46 * noise_variable_30);
        let noise_metadata_schedule_176_e1772: f64 = (noise_metadata_schedule_176_e1770 / params.p44);
        (noise_metadata_schedule_176_e1772,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_176_e1774;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_177_e1779,) = {
    if (noise_variable_301 == 0.0) {
        (params.p43,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_177_e1779;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_178_e1784,) = {
    if (noise_variable_301 == 0.0) {
        (params.p44,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_178_e1784;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_179_e1789,) = {
    if (noise_variable_301 == 0.0) {
        (params.p46,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_179_e1789;
        }
        if matches!(source_index, 5 | 6 | 7 | 9) {
            let noise_metadata_schedule_180_e1793: f64 = (params.p124 * noise_variable_13);
            let noise_metadata_schedule_180_e1796: f64 = (params.p118 * noise_variable_7);
            let noise_metadata_schedule_180_e1799: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_180_e1800: f64 = (noise_metadata_schedule_180_e1796 * noise_metadata_schedule_180_e1799);
            let noise_metadata_schedule_180_e1801: f64 = (noise_metadata_schedule_180_e1793 + noise_metadata_schedule_180_e1800);
            let noise_metadata_schedule_180_e1802: f64 = (noise_metadata_schedule_180_e1801).exp();
            let noise_metadata_schedule_180_e1803: f64 = (params.p18 * noise_metadata_schedule_180_e1802);
            noise_variable_23 = noise_metadata_schedule_180_e1803;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_182_e1832: f64 = if ((params.p27 > 0.0) && ((noise_variable_205 < noise_variable_223) || (noise_variable_202 < noise_variable_223))) { 1.0 } else { 0.0 };
            noise_variable_303 = noise_metadata_schedule_182_e1832;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_183_e1836,) = {
    if (noise_variable_303 != 0.0) {
        (1.0,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_183_e1836;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_184_e1840,) = {
    if (noise_variable_303 != 0.0) {
        (1.0,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_184_e1840;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_185_e1846,) = {
    if (noise_variable_303 != 0.0) {
        let noise_metadata_schedule_185_e1844: f64 = (noise_variable_91 / noise_variable_86);
        (noise_metadata_schedule_185_e1844,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_185_e1846;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_186_e1857: f64 = if (((params.p29 == 1.0) && (params.p43 > 0.0)) && (params.p44 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_304 = noise_metadata_schedule_186_e1857;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_187_e1865,) = {
    if ((noise_variable_303 != 0.0) && (noise_variable_304 != 0.0)) {
        let noise_metadata_schedule_187_e1863: f64 = (noise_variable_30 / params.p44);
        (noise_metadata_schedule_187_e1863,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_187_e1865;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_188_e1880,) = {
    if ((noise_variable_303 != 0.0) && (noise_variable_304 != 0.0)) {
        let noise_metadata_schedule_188_e1871: f64 = (noise_variable_29 / params.p43);
        let noise_metadata_schedule_188_e1873: f64 = (noise_variable_169).sqrt();
        let noise_metadata_schedule_188_e1874: f64 = (noise_metadata_schedule_188_e1871 * noise_metadata_schedule_188_e1873);
        let noise_metadata_schedule_188_e1876: f64 = (noise_metadata_schedule_188_e1874 * noise_variable_170);
        let noise_metadata_schedule_188_e1878: f64 = (noise_metadata_schedule_188_e1876 * noise_variable_170);
        (noise_metadata_schedule_188_e1878,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_188_e1880;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_189_e1895,) = {
    if ((noise_variable_303 != 0.0) && (noise_variable_304 != 0.0)) {
        let noise_metadata_schedule_189_e1886: f64 = (params.p43 / noise_variable_29);
        let noise_metadata_schedule_189_e1889: f64 = (-1.5);
        let noise_metadata_schedule_189_e1890: f64 = (noise_variable_169).powf(noise_metadata_schedule_189_e1889);
        let noise_metadata_schedule_189_e1891: f64 = (noise_metadata_schedule_189_e1886 * noise_metadata_schedule_189_e1890);
        let noise_metadata_schedule_189_e1893: f64 = (noise_metadata_schedule_189_e1891 / noise_variable_170);
        (noise_metadata_schedule_189_e1893,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_189_e1895;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_190_e1906: f64 = if (((params.p29 == 0.0) && (params.p39 > 0.0)) && (params.p40 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_305 = noise_metadata_schedule_190_e1906;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_191_e1917,) = {
    if (((noise_variable_303 != 0.0) && (noise_variable_304 == 0.0)) && (noise_variable_305 != 0.0)) {
        let noise_metadata_schedule_191_e1915: f64 = (noise_variable_27 / params.p40);
        (noise_metadata_schedule_191_e1915,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_191_e1917;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_192_e1935,) = {
    if (((noise_variable_303 != 0.0) && (noise_variable_304 == 0.0)) && (noise_variable_305 != 0.0)) {
        let noise_metadata_schedule_192_e1926: f64 = (noise_variable_26 / params.p39);
        let noise_metadata_schedule_192_e1928: f64 = (noise_variable_169).sqrt();
        let noise_metadata_schedule_192_e1929: f64 = (noise_metadata_schedule_192_e1926 * noise_metadata_schedule_192_e1928);
        let noise_metadata_schedule_192_e1931: f64 = (noise_metadata_schedule_192_e1929 * noise_variable_170);
        let noise_metadata_schedule_192_e1933: f64 = (noise_metadata_schedule_192_e1931 * noise_variable_170);
        (noise_metadata_schedule_192_e1933,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_192_e1935;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_193_e1953,) = {
    if (((noise_variable_303 != 0.0) && (noise_variable_304 == 0.0)) && (noise_variable_305 != 0.0)) {
        let noise_metadata_schedule_193_e1944: f64 = (params.p39 / noise_variable_26);
        let noise_metadata_schedule_193_e1947: f64 = (-1.5);
        let noise_metadata_schedule_193_e1948: f64 = (noise_variable_169).powf(noise_metadata_schedule_193_e1947);
        let noise_metadata_schedule_193_e1949: f64 = (noise_metadata_schedule_193_e1944 * noise_metadata_schedule_193_e1948);
        let noise_metadata_schedule_193_e1951: f64 = (noise_metadata_schedule_193_e1949 / noise_variable_170);
        (noise_metadata_schedule_193_e1951,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_193_e1953;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_194_e1959,) = {
    if (noise_variable_303 != 0.0) {
        let noise_metadata_schedule_194_e1957: f64 = (params.p27 * noise_variable_167);
        (noise_metadata_schedule_194_e1957,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_194_e1959;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_195_e1965,) = {
    if (noise_variable_303 != 0.0) {
        let noise_metadata_schedule_195_e1963: f64 = (params.p28 * noise_variable_166);
        (noise_metadata_schedule_195_e1963,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_195_e1965;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_196_e1970,) = {
    if (noise_variable_303 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_196_e1970;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_197_e1975,) = {
    if (noise_variable_303 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_197_e1975;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_199_e1987: f64 = if 1.0 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_306 = noise_metadata_schedule_199_e1987;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_200_e2009,) = {
    if (noise_variable_306 != 0.0) {
        let noise_metadata_schedule_200_e1991: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_200_e1994: f64 = (params.p53 * 0.5);
        let noise_metadata_schedule_200_e1996: f64 = (noise_metadata_schedule_200_e1994 * noise_variable_7);
        let noise_metadata_schedule_200_e1997: f64 = (noise_metadata_schedule_200_e1996).exp();
        let noise_metadata_schedule_200_e1999: f64 = (-0.5);
        let noise_metadata_schedule_200_e2001: f64 = (noise_metadata_schedule_200_e1999 * params.p53);
        let noise_metadata_schedule_200_e2003: f64 = (noise_metadata_schedule_200_e2001 * noise_variable_7);
        let noise_metadata_schedule_200_e2004: f64 = (noise_metadata_schedule_200_e2003).exp();
        let noise_metadata_schedule_200_e2005: f64 = (noise_metadata_schedule_200_e1997 - noise_metadata_schedule_200_e2004);
        let noise_metadata_schedule_200_e2006: f64 = (noise_metadata_schedule_200_e2005).ln();
        let noise_metadata_schedule_200_e2007: f64 = (noise_metadata_schedule_200_e1991 * noise_metadata_schedule_200_e2006);
        (noise_metadata_schedule_200_e2007,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_200_e2009;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_201_e2027,) = {
    if (noise_variable_306 != 0.0) {
        let noise_metadata_schedule_201_e2013: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_201_e2017: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_201_e2018: f64 = (noise_variable_78 * noise_metadata_schedule_201_e2017);
        let noise_metadata_schedule_201_e2019: f64 = (noise_metadata_schedule_201_e2013 + noise_metadata_schedule_201_e2018);
        let noise_metadata_schedule_201_e2022: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_201_e2024: f64 = (noise_metadata_schedule_201_e2022 * noise_variable_13);
        let noise_metadata_schedule_201_e2025: f64 = (noise_metadata_schedule_201_e2019 - noise_metadata_schedule_201_e2024);
        (noise_metadata_schedule_201_e2025,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_201_e2027;
        }
        if matches!(source_index, 13) {
            let noise_metadata_schedule_214_e2113: f64 = (noise_variable_81 * noise_variable_13);
            let noise_metadata_schedule_214_e2116: f64 = (params.p119 * noise_variable_7);
            let noise_metadata_schedule_214_e2119: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_214_e2120: f64 = (noise_metadata_schedule_214_e2116 * noise_metadata_schedule_214_e2119);
            let noise_metadata_schedule_214_e2121: f64 = (noise_metadata_schedule_214_e2113 + noise_metadata_schedule_214_e2120);
            let noise_metadata_schedule_214_e2122: f64 = (noise_metadata_schedule_214_e2121).exp();
            let noise_metadata_schedule_214_e2123: f64 = (params.p25 * noise_metadata_schedule_214_e2122);
            noise_variable_36 = noise_metadata_schedule_214_e2123;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_215_e2126: f64 = if params.p0 <= 300.0 { 1.0 } else { 0.0 };
            noise_variable_309 = noise_metadata_schedule_215_e2126;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_216_e2129: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_310 = noise_metadata_schedule_216_e2129;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_217_e2153,) = {
    if ((noise_variable_309 != 0.0) && (noise_variable_310 != 0.0)) {
        let noise_metadata_schedule_217_e2135: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_217_e2138: f64 = (params.p58 * 0.5);
        let noise_metadata_schedule_217_e2140: f64 = (noise_metadata_schedule_217_e2138 * noise_variable_7);
        let noise_metadata_schedule_217_e2141: f64 = (noise_metadata_schedule_217_e2140).exp();
        let noise_metadata_schedule_217_e2143: f64 = (-0.5);
        let noise_metadata_schedule_217_e2145: f64 = (noise_metadata_schedule_217_e2143 * params.p58);
        let noise_metadata_schedule_217_e2147: f64 = (noise_metadata_schedule_217_e2145 * noise_variable_7);
        let noise_metadata_schedule_217_e2148: f64 = (noise_metadata_schedule_217_e2147).exp();
        let noise_metadata_schedule_217_e2149: f64 = (noise_metadata_schedule_217_e2141 - noise_metadata_schedule_217_e2148);
        let noise_metadata_schedule_217_e2150: f64 = (noise_metadata_schedule_217_e2149).ln();
        let noise_metadata_schedule_217_e2151: f64 = (noise_metadata_schedule_217_e2135 * noise_metadata_schedule_217_e2150);
        (noise_metadata_schedule_217_e2151,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_217_e2153;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_218_e2173,) = {
    if ((noise_variable_309 != 0.0) && (noise_variable_310 != 0.0)) {
        let noise_metadata_schedule_218_e2159: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_218_e2163: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_218_e2164: f64 = (noise_variable_79 * noise_metadata_schedule_218_e2163);
        let noise_metadata_schedule_218_e2165: f64 = (noise_metadata_schedule_218_e2159 + noise_metadata_schedule_218_e2164);
        let noise_metadata_schedule_218_e2168: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_218_e2170: f64 = (noise_metadata_schedule_218_e2168 * noise_variable_13);
        let noise_metadata_schedule_218_e2171: f64 = (noise_metadata_schedule_218_e2165 - noise_metadata_schedule_218_e2170);
        (noise_metadata_schedule_218_e2171,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_218_e2173;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_228_e2267: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_312 = noise_metadata_schedule_228_e2267;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_229_e2292,) = {
    if ((noise_variable_309 == 0.0) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_229_e2274: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_229_e2277: f64 = (params.p58 * 0.5);
        let noise_metadata_schedule_229_e2279: f64 = (noise_metadata_schedule_229_e2277 * noise_variable_7);
        let noise_metadata_schedule_229_e2280: f64 = (noise_metadata_schedule_229_e2279).exp();
        let noise_metadata_schedule_229_e2282: f64 = (-0.5);
        let noise_metadata_schedule_229_e2284: f64 = (noise_metadata_schedule_229_e2282 * params.p58);
        let noise_metadata_schedule_229_e2286: f64 = (noise_metadata_schedule_229_e2284 * noise_variable_7);
        let noise_metadata_schedule_229_e2287: f64 = (noise_metadata_schedule_229_e2286).exp();
        let noise_metadata_schedule_229_e2288: f64 = (noise_metadata_schedule_229_e2280 - noise_metadata_schedule_229_e2287);
        let noise_metadata_schedule_229_e2289: f64 = (noise_metadata_schedule_229_e2288).ln();
        let noise_metadata_schedule_229_e2290: f64 = (noise_metadata_schedule_229_e2274 * noise_metadata_schedule_229_e2289);
        (noise_metadata_schedule_229_e2290,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_229_e2292;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_230_e2313,) = {
    if ((noise_variable_309 == 0.0) && (noise_variable_312 != 0.0)) {
        let noise_metadata_schedule_230_e2299: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_230_e2303: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_230_e2304: f64 = (noise_variable_79 * noise_metadata_schedule_230_e2303);
        let noise_metadata_schedule_230_e2305: f64 = (noise_metadata_schedule_230_e2299 + noise_metadata_schedule_230_e2304);
        let noise_metadata_schedule_230_e2308: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_230_e2310: f64 = (noise_metadata_schedule_230_e2308 * noise_variable_13);
        let noise_metadata_schedule_230_e2311: f64 = (noise_metadata_schedule_230_e2305 - noise_metadata_schedule_230_e2310);
        (noise_metadata_schedule_230_e2311,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_230_e2313;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_240_e2416: f64 = (noise_variable_82 * noise_variable_13);
            let noise_metadata_schedule_240_e2419: f64 = (params.p120 * noise_variable_7);
            let noise_metadata_schedule_240_e2422: f64 = (1.0 - noise_variable_12);
            let noise_metadata_schedule_240_e2423: f64 = (noise_metadata_schedule_240_e2419 * noise_metadata_schedule_240_e2422);
            let noise_metadata_schedule_240_e2424: f64 = (noise_metadata_schedule_240_e2416 + noise_metadata_schedule_240_e2423);
            let noise_metadata_schedule_240_e2425: f64 = (noise_metadata_schedule_240_e2424).exp();
            let noise_metadata_schedule_240_e2426: f64 = (params.p99 * noise_metadata_schedule_240_e2425);
            noise_variable_45 = noise_metadata_schedule_240_e2426;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_243_e2451: f64 = if params.p63 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_314 = noise_metadata_schedule_243_e2451;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_244_e2454: f64 = if params.p62 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_315 = noise_metadata_schedule_244_e2454;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_245_e2478,) = {
    if ((noise_variable_314 != 0.0) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_245_e2460: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_245_e2463: f64 = (params.p63 * 0.5);
        let noise_metadata_schedule_245_e2465: f64 = (noise_metadata_schedule_245_e2463 * noise_variable_7);
        let noise_metadata_schedule_245_e2466: f64 = (noise_metadata_schedule_245_e2465).exp();
        let noise_metadata_schedule_245_e2468: f64 = (-0.5);
        let noise_metadata_schedule_245_e2470: f64 = (noise_metadata_schedule_245_e2468 * params.p63);
        let noise_metadata_schedule_245_e2472: f64 = (noise_metadata_schedule_245_e2470 * noise_variable_7);
        let noise_metadata_schedule_245_e2473: f64 = (noise_metadata_schedule_245_e2472).exp();
        let noise_metadata_schedule_245_e2474: f64 = (noise_metadata_schedule_245_e2466 - noise_metadata_schedule_245_e2473);
        let noise_metadata_schedule_245_e2475: f64 = (noise_metadata_schedule_245_e2474).ln();
        let noise_metadata_schedule_245_e2476: f64 = (noise_metadata_schedule_245_e2460 * noise_metadata_schedule_245_e2475);
        (noise_metadata_schedule_245_e2476,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_245_e2478;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_246_e2498,) = {
    if ((noise_variable_314 != 0.0) && (noise_variable_315 != 0.0)) {
        let noise_metadata_schedule_246_e2484: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_246_e2488: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_246_e2489: f64 = (noise_variable_79 * noise_metadata_schedule_246_e2488);
        let noise_metadata_schedule_246_e2490: f64 = (noise_metadata_schedule_246_e2484 + noise_metadata_schedule_246_e2489);
        let noise_metadata_schedule_246_e2493: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_246_e2495: f64 = (noise_metadata_schedule_246_e2493 * noise_variable_13);
        let noise_metadata_schedule_246_e2496: f64 = (noise_metadata_schedule_246_e2490 - noise_metadata_schedule_246_e2495);
        (noise_metadata_schedule_246_e2496,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_246_e2498;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_258_e2604: f64 = (params.p136 * noise_variable_13);
            let noise_metadata_schedule_258_e2605: f64 = (noise_metadata_schedule_258_e2604).exp();
            let noise_metadata_schedule_258_e2606: f64 = (params.p96 * noise_metadata_schedule_258_e2605);
            noise_variable_72 = noise_metadata_schedule_258_e2606;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_259_e2610: f64 = (params.p135 * noise_variable_13);
            let noise_metadata_schedule_259_e2611: f64 = (noise_metadata_schedule_259_e2610).exp();
            let noise_metadata_schedule_259_e2612: f64 = (params.p90 * noise_metadata_schedule_259_e2611);
            noise_variable_71 = noise_metadata_schedule_259_e2612;
        }
        if matches!(source_index, 3 | 5 | 6 | 7) {
            let noise_metadata_schedule_260_e2616: f64 = (params.p137 * noise_variable_13);
            let noise_metadata_schedule_260_e2617: f64 = (noise_metadata_schedule_260_e2616).exp();
            let noise_metadata_schedule_260_e2618: f64 = (params.p95 * noise_metadata_schedule_260_e2617);
            noise_variable_73 = noise_metadata_schedule_260_e2618;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_262_e2641: f64 = if (((params.p141 != 0.0) && (params.p142 >= params.p149)) && (params.p142 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_317 = noise_metadata_schedule_262_e2641;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_263_e2649,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_263_e2645: f64 = (noise_variable_9 + params.p147);
        let noise_metadata_schedule_263_e2647: f64 = (noise_metadata_schedule_263_e2645 + (ctx.node_voltage(self.nodes[4]) - 0.0));
        (noise_metadata_schedule_263_e2647,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_263_e2649;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_264_e2652: f64 = (-200.0);
            let noise_metadata_schedule_264_e2654: f64 = (noise_metadata_schedule_264_e2652 + 273.15);
            let noise_metadata_schedule_264_e2655: f64 = if noise_variable_10 < noise_metadata_schedule_264_e2654 { 1.0 } else { 0.0 };
            noise_variable_318 = noise_metadata_schedule_264_e2655;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_265_e2664,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_318 != 0.0)) {
        let noise_metadata_schedule_265_e2660: f64 = (-200.0);
        let noise_metadata_schedule_265_e2662: f64 = (noise_metadata_schedule_265_e2660 + 273.15);
        (noise_metadata_schedule_265_e2662,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_265_e2664;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_266_e2668: f64 = (326.85 + 273.15);
            let noise_metadata_schedule_266_e2669: f64 = if noise_variable_10 > noise_metadata_schedule_266_e2668 { 1.0 } else { 0.0 };
            noise_variable_319 = noise_metadata_schedule_266_e2669;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_267_e2680,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_318 == 0.0)) && (noise_variable_319 != 0.0)) {
        let noise_metadata_schedule_267_e2678: f64 = (326.85 + 273.15);
        (noise_metadata_schedule_267_e2678,)
    } else {
        (noise_variable_10,)
    }
};
            noise_variable_10 = noise_metadata_schedule_267_e2680;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_268_e2686,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_268_e2684: f64 = (noise_variable_2 * noise_variable_10);
        (noise_metadata_schedule_268_e2684,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_268_e2686;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_269_e2692,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_269_e2690: f64 = (1.0 / noise_variable_4);
        (noise_metadata_schedule_269_e2690,)
    } else {
        (noise_variable_5,)
    }
};
            noise_variable_5 = noise_metadata_schedule_269_e2692;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_270_e2698,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_270_e2696: f64 = (noise_variable_10 - noise_variable_8);
        (noise_metadata_schedule_270_e2696,)
    } else {
        (noise_variable_14,)
    }
};
            noise_variable_14 = noise_metadata_schedule_270_e2698;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_271_e2704,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_271_e2702: f64 = (noise_variable_8 / noise_variable_10);
        (noise_metadata_schedule_271_e2702,)
    } else {
        (noise_variable_12,)
    }
};
            noise_variable_12 = noise_metadata_schedule_271_e2704;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_272_e2710,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_272_e2708: f64 = (noise_variable_10 / noise_variable_8);
        (noise_metadata_schedule_272_e2708,)
    } else {
        (noise_variable_11,)
    }
};
            noise_variable_11 = noise_metadata_schedule_272_e2710;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let (noise_metadata_schedule_273_e2715,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_273_e2713: f64 = (noise_variable_11).ln();
        (noise_metadata_schedule_273_e2713,)
    } else {
        (noise_variable_13,)
    }
};
            noise_variable_13 = noise_metadata_schedule_273_e2715;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_274_e2724,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_274_e2719: f64 = (params.p121 * noise_variable_10);
        let noise_metadata_schedule_274_e2721: f64 = (noise_variable_10).ln();
        let noise_metadata_schedule_274_e2722: f64 = (noise_metadata_schedule_274_e2719 * noise_metadata_schedule_274_e2721);
        (noise_metadata_schedule_274_e2722,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_274_e2724;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_275_e2730,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_275_e2728: f64 = (params.p122 * noise_variable_10);
        (noise_metadata_schedule_275_e2728,)
    } else {
        (noise_variable_75,)
    }
};
            noise_variable_75 = noise_metadata_schedule_275_e2730;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_276_e2738,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_276_e2734: f64 = (params.p117 + noise_variable_74);
        let noise_metadata_schedule_276_e2736: f64 = (noise_metadata_schedule_276_e2734 + noise_variable_75);
        (noise_metadata_schedule_276_e2736,)
    } else {
        (noise_variable_84,)
    }
};
            noise_variable_84 = noise_metadata_schedule_276_e2738;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_277_e2746,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_277_e2742: f64 = (params.p118 + noise_variable_74);
        let noise_metadata_schedule_277_e2744: f64 = (noise_metadata_schedule_277_e2742 + noise_variable_75);
        (noise_metadata_schedule_277_e2744,)
    } else {
        (noise_variable_83,)
    }
};
            noise_variable_83 = noise_metadata_schedule_277_e2746;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_278_e2754,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_278_e2750: f64 = (params.p119 + noise_variable_74);
        let noise_metadata_schedule_278_e2752: f64 = (noise_metadata_schedule_278_e2750 + noise_variable_75);
        (noise_metadata_schedule_278_e2752,)
    } else {
        (noise_variable_85,)
    }
};
            noise_variable_85 = noise_metadata_schedule_278_e2754;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_279_e2762,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_279_e2758: f64 = (noise_variable_84 + noise_variable_83);
        let noise_metadata_schedule_279_e2760: f64 = (noise_metadata_schedule_279_e2758 * 0.5);
        (noise_metadata_schedule_279_e2760,)
    } else {
        (noise_variable_86,)
    }
};
            noise_variable_86 = noise_metadata_schedule_279_e2762;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_280_e2770,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_280_e2766: f64 = (noise_variable_84 + noise_variable_85);
        let noise_metadata_schedule_280_e2768: f64 = (noise_metadata_schedule_280_e2766 * 0.5);
        (noise_metadata_schedule_280_e2768,)
    } else {
        (noise_variable_87,)
    }
};
            noise_variable_87 = noise_metadata_schedule_280_e2770;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_281_e2773: f64 = if params.p39 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_320 = noise_metadata_schedule_281_e2773;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_282_e2797,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_282_e2779: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_282_e2782: f64 = (params.p40 * 0.5);
        let noise_metadata_schedule_282_e2784: f64 = (noise_metadata_schedule_282_e2782 * noise_variable_7);
        let noise_metadata_schedule_282_e2785: f64 = (noise_metadata_schedule_282_e2784).exp();
        let noise_metadata_schedule_282_e2787: f64 = (-0.5);
        let noise_metadata_schedule_282_e2789: f64 = (noise_metadata_schedule_282_e2787 * params.p40);
        let noise_metadata_schedule_282_e2791: f64 = (noise_metadata_schedule_282_e2789 * noise_variable_7);
        let noise_metadata_schedule_282_e2792: f64 = (noise_metadata_schedule_282_e2791).exp();
        let noise_metadata_schedule_282_e2793: f64 = (noise_metadata_schedule_282_e2785 - noise_metadata_schedule_282_e2792);
        let noise_metadata_schedule_282_e2794: f64 = (noise_metadata_schedule_282_e2793).ln();
        let noise_metadata_schedule_282_e2795: f64 = (noise_metadata_schedule_282_e2779 * noise_metadata_schedule_282_e2794);
        (noise_metadata_schedule_282_e2795,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_282_e2797;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_283_e2817,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_283_e2803: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_283_e2807: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_283_e2808: f64 = (noise_variable_77 * noise_metadata_schedule_283_e2807);
        let noise_metadata_schedule_283_e2809: f64 = (noise_metadata_schedule_283_e2803 + noise_metadata_schedule_283_e2808);
        let noise_metadata_schedule_283_e2812: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_283_e2814: f64 = (noise_metadata_schedule_283_e2812 * noise_variable_13);
        let noise_metadata_schedule_283_e2815: f64 = (noise_metadata_schedule_283_e2809 - noise_metadata_schedule_283_e2814);
        (noise_metadata_schedule_283_e2815,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_283_e2817;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_284_e2843,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_284_e2824: f64 = (2.0 * noise_variable_4);
        let noise_metadata_schedule_284_e2830: f64 = (-noise_variable_165);
        let noise_metadata_schedule_284_e2832: f64 = (noise_metadata_schedule_284_e2830 * noise_variable_5);
        let noise_metadata_schedule_284_e2833: f64 = (noise_metadata_schedule_284_e2832).exp();
        let noise_metadata_schedule_284_e2834: f64 = (4.0 * noise_metadata_schedule_284_e2833);
        let noise_metadata_schedule_284_e2835: f64 = (1.0 + noise_metadata_schedule_284_e2834);
        let noise_metadata_schedule_284_e2836: f64 = (noise_metadata_schedule_284_e2835).sqrt();
        let noise_metadata_schedule_284_e2837: f64 = (1.0 + noise_metadata_schedule_284_e2836);
        let noise_metadata_schedule_284_e2838: f64 = (0.5 * noise_metadata_schedule_284_e2837);
        let noise_metadata_schedule_284_e2839: f64 = (noise_metadata_schedule_284_e2838).ln();
        let noise_metadata_schedule_284_e2840: f64 = (noise_metadata_schedule_284_e2824 * noise_metadata_schedule_284_e2839);
        let noise_metadata_schedule_284_e2841: f64 = (noise_variable_165 + noise_metadata_schedule_284_e2840);
        (noise_metadata_schedule_284_e2841,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_284_e2843;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_285_e2857,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_285_e2851: f64 = (params.p40 / noise_variable_27);
        let noise_metadata_schedule_285_e2852: f64 = (noise_metadata_schedule_285_e2851).ln();
        let noise_metadata_schedule_285_e2853: f64 = (params.p41 * noise_metadata_schedule_285_e2852);
        let noise_metadata_schedule_285_e2854: f64 = (noise_metadata_schedule_285_e2853).exp();
        let noise_metadata_schedule_285_e2855: f64 = (params.p39 * noise_metadata_schedule_285_e2854);
        (noise_metadata_schedule_285_e2855,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_285_e2857;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_286_e2864,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 != 0.0)) {
        let noise_metadata_schedule_286_e2862: f64 = (params.p42).abs();
        (noise_metadata_schedule_286_e2862,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_286_e2864;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_287_e2867: f64 = if params.p42 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_321 = noise_metadata_schedule_287_e2867;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_288_e2879,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_320 != 0.0)) && (noise_variable_321 != 0.0)) {
        let noise_metadata_schedule_288_e2875: f64 = (params.p42 * noise_variable_27);
        let noise_metadata_schedule_288_e2877: f64 = (noise_metadata_schedule_288_e2875 / params.p40);
        (noise_metadata_schedule_288_e2877,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_288_e2879;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_289_e2886,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 == 0.0)) {
        (params.p39,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_289_e2886;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_290_e2893,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 == 0.0)) {
        (params.p40,)
    } else {
        (noise_variable_27,)
    }
};
            noise_variable_27 = noise_metadata_schedule_290_e2893;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_291_e2900,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_320 == 0.0)) {
        (params.p42,)
    } else {
        (noise_variable_28,)
    }
};
            noise_variable_28 = noise_metadata_schedule_291_e2900;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 15 | 18) {
            let (noise_metadata_schedule_292_e2917,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_292_e2905: f64 = (params.p124 * noise_variable_13);
        let noise_metadata_schedule_292_e2908: f64 = (params.p118 * noise_variable_7);
        let noise_metadata_schedule_292_e2911: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_292_e2912: f64 = (noise_metadata_schedule_292_e2908 * noise_metadata_schedule_292_e2911);
        let noise_metadata_schedule_292_e2913: f64 = (noise_metadata_schedule_292_e2905 + noise_metadata_schedule_292_e2912);
        let noise_metadata_schedule_292_e2914: f64 = (noise_metadata_schedule_292_e2913).exp();
        let noise_metadata_schedule_292_e2915: f64 = (params.p14 * noise_metadata_schedule_292_e2914);
        (noise_metadata_schedule_292_e2915,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_292_e2917;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_294_e2941: f64 = if params.p47 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_322 = noise_metadata_schedule_294_e2941;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_295_e2965,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 != 0.0)) {
        let noise_metadata_schedule_295_e2947: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_295_e2950: f64 = (params.p48 * 0.5);
        let noise_metadata_schedule_295_e2952: f64 = (noise_metadata_schedule_295_e2950 * noise_variable_7);
        let noise_metadata_schedule_295_e2953: f64 = (noise_metadata_schedule_295_e2952).exp();
        let noise_metadata_schedule_295_e2955: f64 = (-0.5);
        let noise_metadata_schedule_295_e2957: f64 = (noise_metadata_schedule_295_e2955 * params.p48);
        let noise_metadata_schedule_295_e2959: f64 = (noise_metadata_schedule_295_e2957 * noise_variable_7);
        let noise_metadata_schedule_295_e2960: f64 = (noise_metadata_schedule_295_e2959).exp();
        let noise_metadata_schedule_295_e2961: f64 = (noise_metadata_schedule_295_e2953 - noise_metadata_schedule_295_e2960);
        let noise_metadata_schedule_295_e2962: f64 = (noise_metadata_schedule_295_e2961).ln();
        let noise_metadata_schedule_295_e2963: f64 = (noise_metadata_schedule_295_e2947 * noise_metadata_schedule_295_e2962);
        (noise_metadata_schedule_295_e2963,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_295_e2965;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_296_e2985,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 != 0.0)) {
        let noise_metadata_schedule_296_e2971: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_296_e2975: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_296_e2976: f64 = (noise_variable_78 * noise_metadata_schedule_296_e2975);
        let noise_metadata_schedule_296_e2977: f64 = (noise_metadata_schedule_296_e2971 + noise_metadata_schedule_296_e2976);
        let noise_metadata_schedule_296_e2980: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_296_e2982: f64 = (noise_metadata_schedule_296_e2980 * noise_variable_13);
        let noise_metadata_schedule_296_e2983: f64 = (noise_metadata_schedule_296_e2977 - noise_metadata_schedule_296_e2982);
        (noise_metadata_schedule_296_e2983,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_296_e2985;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_297_e3011,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 != 0.0)) {
        let noise_metadata_schedule_297_e2992: f64 = (2.0 * noise_variable_4);
        let noise_metadata_schedule_297_e2998: f64 = (-noise_variable_165);
        let noise_metadata_schedule_297_e3000: f64 = (noise_metadata_schedule_297_e2998 * noise_variable_5);
        let noise_metadata_schedule_297_e3001: f64 = (noise_metadata_schedule_297_e3000).exp();
        let noise_metadata_schedule_297_e3002: f64 = (4.0 * noise_metadata_schedule_297_e3001);
        let noise_metadata_schedule_297_e3003: f64 = (1.0 + noise_metadata_schedule_297_e3002);
        let noise_metadata_schedule_297_e3004: f64 = (noise_metadata_schedule_297_e3003).sqrt();
        let noise_metadata_schedule_297_e3005: f64 = (1.0 + noise_metadata_schedule_297_e3004);
        let noise_metadata_schedule_297_e3006: f64 = (0.5 * noise_metadata_schedule_297_e3005);
        let noise_metadata_schedule_297_e3007: f64 = (noise_metadata_schedule_297_e3006).ln();
        let noise_metadata_schedule_297_e3008: f64 = (noise_metadata_schedule_297_e2992 * noise_metadata_schedule_297_e3007);
        let noise_metadata_schedule_297_e3009: f64 = (noise_variable_165 + noise_metadata_schedule_297_e3008);
        (noise_metadata_schedule_297_e3009,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_297_e3011;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_298_e3025,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 != 0.0)) {
        let noise_metadata_schedule_298_e3019: f64 = (params.p48 / noise_variable_34);
        let noise_metadata_schedule_298_e3020: f64 = (noise_metadata_schedule_298_e3019).ln();
        let noise_metadata_schedule_298_e3021: f64 = (params.p49 * noise_metadata_schedule_298_e3020);
        let noise_metadata_schedule_298_e3022: f64 = (noise_metadata_schedule_298_e3021).exp();
        let noise_metadata_schedule_298_e3023: f64 = (params.p47 * noise_metadata_schedule_298_e3022);
        (noise_metadata_schedule_298_e3023,)
    } else {
        (noise_variable_33,)
    }
};
            noise_variable_33 = noise_metadata_schedule_298_e3025;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_299_e3032,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 != 0.0)) {
        let noise_metadata_schedule_299_e3030: f64 = (params.p50).abs();
        (noise_metadata_schedule_299_e3030,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_299_e3032;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_300_e3035: f64 = if params.p50 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_323 = noise_metadata_schedule_300_e3035;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_301_e3047,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_322 != 0.0)) && (noise_variable_323 != 0.0)) {
        let noise_metadata_schedule_301_e3043: f64 = (params.p50 * noise_variable_34);
        let noise_metadata_schedule_301_e3045: f64 = (noise_metadata_schedule_301_e3043 / params.p48);
        (noise_metadata_schedule_301_e3045,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_301_e3047;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_302_e3054,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 == 0.0)) {
        (params.p47,)
    } else {
        (noise_variable_33,)
    }
};
            noise_variable_33 = noise_metadata_schedule_302_e3054;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_303_e3061,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 == 0.0)) {
        (params.p48,)
    } else {
        (noise_variable_34,)
    }
};
            noise_variable_34 = noise_metadata_schedule_303_e3061;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_304_e3068,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_322 == 0.0)) {
        (params.p50,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_304_e3068;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_305_e3071: f64 = if params.p0 <= 300.0 { 1.0 } else { 0.0 };
            noise_variable_324 = noise_metadata_schedule_305_e3071;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_306_e3077,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_324 != 0.0)) {
        (2.4,)
    } else {
        (noise_variable_35,)
    }
};
            noise_variable_35 = noise_metadata_schedule_306_e3077;
        }
        if matches!(source_index, 11) {
            let (noise_metadata_schedule_307_e3094,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_307_e3082: f64 = (noise_variable_80 * noise_variable_13);
        let noise_metadata_schedule_307_e3085: f64 = (params.p119 * noise_variable_7);
        let noise_metadata_schedule_307_e3088: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_307_e3089: f64 = (noise_metadata_schedule_307_e3085 * noise_metadata_schedule_307_e3088);
        let noise_metadata_schedule_307_e3090: f64 = (noise_metadata_schedule_307_e3082 + noise_metadata_schedule_307_e3089);
        let noise_metadata_schedule_307_e3091: f64 = (noise_metadata_schedule_307_e3090).exp();
        let noise_metadata_schedule_307_e3092: f64 = (params.p23 * noise_metadata_schedule_307_e3091);
        (noise_metadata_schedule_307_e3092,)
    } else {
        (noise_variable_32,)
    }
};
            noise_variable_32 = noise_metadata_schedule_307_e3094;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_308_e3108,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_308_e3101: f64 = (noise_variable_27 / params.p40);
        let noise_metadata_schedule_308_e3102: f64 = (noise_metadata_schedule_308_e3101).ln();
        let noise_metadata_schedule_308_e3103: f64 = (params.p41 * noise_metadata_schedule_308_e3102);
        let noise_metadata_schedule_308_e3104: f64 = (noise_metadata_schedule_308_e3103).exp();
        let noise_metadata_schedule_308_e3105: f64 = (2.0 - noise_metadata_schedule_308_e3104);
        let noise_metadata_schedule_308_e3106: f64 = (params.p2 * noise_metadata_schedule_308_e3105);
        (noise_metadata_schedule_308_e3106,)
    } else {
        (noise_variable_16,)
    }
};
            noise_variable_16 = noise_metadata_schedule_308_e3108;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_309_e3125,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_309_e3113: f64 = (params.p123 * noise_variable_13);
        let noise_metadata_schedule_309_e3116: f64 = (params.p117 * noise_variable_7);
        let noise_metadata_schedule_309_e3119: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_309_e3120: f64 = (noise_metadata_schedule_309_e3116 * noise_metadata_schedule_309_e3119);
        let noise_metadata_schedule_309_e3121: f64 = (noise_metadata_schedule_309_e3113 + noise_metadata_schedule_309_e3120);
        let noise_metadata_schedule_309_e3122: f64 = (noise_metadata_schedule_309_e3121).exp();
        let noise_metadata_schedule_309_e3123: f64 = (params.p1 * noise_metadata_schedule_309_e3122);
        (noise_metadata_schedule_309_e3123,)
    } else {
        (noise_variable_15,)
    }
};
            noise_variable_15 = noise_metadata_schedule_309_e3125;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_310_e3134,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_310_e3130: f64 = (params.p126 * noise_variable_13);
        let noise_metadata_schedule_310_e3131: f64 = (noise_metadata_schedule_310_e3130).exp();
        let noise_metadata_schedule_310_e3132: f64 = (params.p10 * noise_metadata_schedule_310_e3131);
        (noise_metadata_schedule_310_e3132,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_310_e3134;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_311_e3140: f64 = (params.p8 - 1.0);
            let noise_metadata_schedule_311_e3141: f64 = (noise_metadata_schedule_311_e3140).abs();
            let noise_metadata_schedule_311_e3144: f64 = if ((params.p0 <= 300.0) && (noise_metadata_schedule_311_e3141 < 1e-5)) { 1.0 } else { 0.0 };
            noise_variable_325 = noise_metadata_schedule_311_e3144;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_312_e3162,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_325 != 0.0)) {
        let noise_metadata_schedule_312_e3151: f64 = (params.p125 * noise_variable_5);
        let noise_metadata_schedule_312_e3154: f64 = (params.p127 * noise_variable_13);
        let noise_metadata_schedule_312_e3155: f64 = (noise_metadata_schedule_312_e3154).exp();
        let noise_metadata_schedule_312_e3157: f64 = (noise_metadata_schedule_312_e3155 - 1.0);
        let noise_metadata_schedule_312_e3158: f64 = (noise_metadata_schedule_312_e3151 * noise_metadata_schedule_312_e3157);
        let noise_metadata_schedule_312_e3159: f64 = (noise_metadata_schedule_312_e3158).exp();
        let noise_metadata_schedule_312_e3160: f64 = (params.p9 * noise_metadata_schedule_312_e3159);
        (noise_metadata_schedule_312_e3160,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_312_e3162;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_313_e3181,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_325 == 0.0)) {
        let noise_metadata_schedule_313_e3170: f64 = (params.p125 * noise_variable_5);
        let noise_metadata_schedule_313_e3173: f64 = (params.p127 * noise_variable_13);
        let noise_metadata_schedule_313_e3174: f64 = (noise_metadata_schedule_313_e3173).exp();
        let noise_metadata_schedule_313_e3176: f64 = (noise_metadata_schedule_313_e3174 - 1.0);
        let noise_metadata_schedule_313_e3177: f64 = (noise_metadata_schedule_313_e3170 * noise_metadata_schedule_313_e3176);
        let noise_metadata_schedule_313_e3178: f64 = (noise_metadata_schedule_313_e3177).exp();
        let noise_metadata_schedule_313_e3179: f64 = (params.p8 * noise_metadata_schedule_313_e3178);
        (noise_metadata_schedule_313_e3179,)
    } else {
        (noise_variable_17,)
    }
};
            noise_variable_17 = noise_metadata_schedule_313_e3181;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_314_e3194,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_314_e3186: f64 = (params.p125 * noise_variable_7);
        let noise_metadata_schedule_314_e3189: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_314_e3190: f64 = (noise_metadata_schedule_314_e3186 * noise_metadata_schedule_314_e3189);
        let noise_metadata_schedule_314_e3191: f64 = (noise_metadata_schedule_314_e3190).exp();
        let noise_metadata_schedule_314_e3192: f64 = (params.p3 * noise_metadata_schedule_314_e3191);
        (noise_metadata_schedule_314_e3192,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_314_e3194;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_315_e3209,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_315_e3199: f64 = (params.p117 - params.p118);
        let noise_metadata_schedule_315_e3201: f64 = (noise_metadata_schedule_315_e3199 * noise_variable_7);
        let noise_metadata_schedule_315_e3204: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_315_e3205: f64 = (noise_metadata_schedule_315_e3201 * noise_metadata_schedule_315_e3204);
        let noise_metadata_schedule_315_e3206: f64 = (noise_metadata_schedule_315_e3205).exp();
        let noise_metadata_schedule_315_e3207: f64 = (params.p4 * noise_metadata_schedule_315_e3206);
        (noise_metadata_schedule_315_e3207,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_315_e3209;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_316_e3224,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_316_e3214: f64 = (params.p117 - params.p119);
        let noise_metadata_schedule_316_e3216: f64 = (noise_metadata_schedule_316_e3214 * noise_variable_7);
        let noise_metadata_schedule_316_e3219: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_316_e3220: f64 = (noise_metadata_schedule_316_e3216 * noise_metadata_schedule_316_e3219);
        let noise_metadata_schedule_316_e3221: f64 = (noise_metadata_schedule_316_e3220).exp();
        let noise_metadata_schedule_316_e3222: f64 = (params.p6 * noise_metadata_schedule_316_e3221);
        (noise_metadata_schedule_316_e3222,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_316_e3224;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_317_e3235,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_317_e3229: f64 = (params.p130 - noise_variable_56);
        let noise_metadata_schedule_317_e3231: f64 = (noise_metadata_schedule_317_e3229 * noise_variable_13);
        let noise_metadata_schedule_317_e3232: f64 = (noise_metadata_schedule_317_e3231).exp();
        let noise_metadata_schedule_317_e3233: f64 = (params.p75 * noise_metadata_schedule_317_e3232);
        (noise_metadata_schedule_317_e3233,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_317_e3235;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_318_e3244,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_318_e3240: f64 = (params.p130 * noise_variable_13);
        let noise_metadata_schedule_318_e3241: f64 = (noise_metadata_schedule_318_e3240).exp();
        let noise_metadata_schedule_318_e3242: f64 = (params.p74 * noise_metadata_schedule_318_e3241);
        (noise_metadata_schedule_318_e3242,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_318_e3244;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_319_e3250,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_319_e3248: f64 = (1.0 / noise_variable_53);
        (noise_metadata_schedule_319_e3248,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_319_e3250;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_320_e3253: f64 = if params.p79 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_326 = noise_metadata_schedule_320_e3253;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_321_e3265,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_326 != 0.0)) {
        let noise_metadata_schedule_321_e3261: f64 = (params.p133 * noise_variable_14);
        let noise_metadata_schedule_321_e3262: f64 = (1.0 - noise_metadata_schedule_321_e3261);
        let noise_metadata_schedule_321_e3263: f64 = (params.p79 * noise_metadata_schedule_321_e3262);
        (noise_metadata_schedule_321_e3263,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_321_e3265;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_322_e3271,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_326 != 0.0)) {
        (params.p78,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_322_e3271;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_323_e3284,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_326 == 0.0)) {
        let noise_metadata_schedule_323_e3280: f64 = (params.p132 * noise_variable_14);
        let noise_metadata_schedule_323_e3281: f64 = (1.0 + noise_metadata_schedule_323_e3280);
        let noise_metadata_schedule_323_e3282: f64 = (params.p78 * noise_metadata_schedule_323_e3281);
        (noise_metadata_schedule_323_e3282,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_323_e3284;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_324_e3291,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_326 == 0.0)) {
        (params.p79,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_324_e3291;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_325_e3307,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_325_e3297: f64 = (params.p128 * noise_variable_14);
        let noise_metadata_schedule_325_e3298: f64 = (1.0 + noise_metadata_schedule_325_e3297);
        let noise_metadata_schedule_325_e3301: f64 = (params.p129 * noise_variable_14);
        let noise_metadata_schedule_325_e3303: f64 = (noise_metadata_schedule_325_e3301 * noise_variable_14);
        let noise_metadata_schedule_325_e3304: f64 = (noise_metadata_schedule_325_e3298 + noise_metadata_schedule_325_e3303);
        let noise_metadata_schedule_325_e3305: f64 = (params.p66 * noise_metadata_schedule_325_e3304);
        (noise_metadata_schedule_325_e3305,)
    } else {
        (noise_variable_59,)
    }
};
            noise_variable_59 = noise_metadata_schedule_325_e3307;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_326_e3311,) = {
    if (noise_variable_317 != 0.0) {
        (params.p69,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_326_e3311;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_327_e3322,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_327_e3316: f64 = (params.p130 - 1.0);
        let noise_metadata_schedule_327_e3318: f64 = (noise_metadata_schedule_327_e3316 * noise_variable_13);
        let noise_metadata_schedule_327_e3319: f64 = (noise_metadata_schedule_327_e3318).exp();
        let noise_metadata_schedule_327_e3320: f64 = (params.p71 * noise_metadata_schedule_327_e3319);
        (noise_metadata_schedule_327_e3320,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_327_e3322;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_328_e3325: f64 = if noise_variable_243 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_327 = noise_metadata_schedule_328_e3325;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_329_e3336,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_327 != 0.0)) {
        let noise_metadata_schedule_329_e3332: f64 = (params.p139 * noise_variable_14);
        let noise_metadata_schedule_329_e3333: f64 = (noise_metadata_schedule_329_e3332).exp();
        let noise_metadata_schedule_329_e3334: f64 = (params.p32 * noise_metadata_schedule_329_e3333);
        (noise_metadata_schedule_329_e3334,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_329_e3336;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_330_e3347,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_327 != 0.0)) {
        let noise_metadata_schedule_330_e3343: f64 = (params.p140 * noise_variable_14);
        let noise_metadata_schedule_330_e3344: f64 = (noise_metadata_schedule_330_e3343).exp();
        let noise_metadata_schedule_330_e3345: f64 = (params.p33 * noise_metadata_schedule_330_e3344);
        (noise_metadata_schedule_330_e3345,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_330_e3347;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_331_e3354,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_327 == 0.0)) {
        (params.p32,)
    } else {
        (noise_variable_63,)
    }
};
            noise_variable_63 = noise_metadata_schedule_331_e3354;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_332_e3361,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_327 == 0.0)) {
        (params.p33,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_332_e3361;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_333_e3368: f64 = if ((params.p37 > 0.0) && (noise_variable_203 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_328 = noise_metadata_schedule_333_e3368;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_334_e3374,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_328 != 0.0)) {
        (params.p37,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_334_e3374;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_335_e3380,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_328 != 0.0)) {
        (params.p38,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_335_e3380;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_336_e3387: f64 = if ((params.p47 > 0.0) && (params.p48 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_329 = noise_metadata_schedule_336_e3387;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_337_e3397,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_328 != 0.0)) && (noise_variable_329 != 0.0)) {
        let noise_metadata_schedule_337_e3395: f64 = (noise_variable_92 / noise_variable_87);
        (noise_metadata_schedule_337_e3395,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_337_e3397;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_338_e3407,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_328 != 0.0)) && (noise_variable_329 != 0.0)) {
        let noise_metadata_schedule_338_e3405: f64 = (noise_variable_34 / params.p48);
        (noise_metadata_schedule_338_e3405,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_338_e3407;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_339_e3422,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_328 != 0.0)) && (noise_variable_329 != 0.0)) {
        let noise_metadata_schedule_339_e3414: f64 = (noise_variable_169).sqrt();
        let noise_metadata_schedule_339_e3416: f64 = (noise_metadata_schedule_339_e3414 * noise_variable_170);
        let noise_metadata_schedule_339_e3418: f64 = (noise_metadata_schedule_339_e3416 * noise_variable_33);
        let noise_metadata_schedule_339_e3420: f64 = (noise_metadata_schedule_339_e3418 / params.p47);
        (noise_metadata_schedule_339_e3420,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_339_e3422;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_340_e3434,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_328 != 0.0)) && (noise_variable_329 != 0.0)) {
        let noise_metadata_schedule_340_e3430: f64 = (params.p37 * noise_variable_168);
        let noise_metadata_schedule_340_e3432: f64 = (noise_metadata_schedule_340_e3430 * noise_variable_170);
        (noise_metadata_schedule_340_e3432,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_340_e3434;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_341_e3446,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_328 != 0.0)) && (noise_variable_329 != 0.0)) {
        let noise_metadata_schedule_341_e3443: f64 = (noise_variable_168 * noise_variable_169);
        let noise_metadata_schedule_341_e3444: f64 = (params.p38 / noise_metadata_schedule_341_e3443);
        (noise_metadata_schedule_341_e3444,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_341_e3446;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_342_e3453,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_328 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_67,)
    }
};
            noise_variable_67 = noise_metadata_schedule_342_e3453;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_343_e3460,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_328 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_68,)
    }
};
            noise_variable_68 = noise_metadata_schedule_343_e3460;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_344_e3469,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_344_e3465: f64 = (params.p134 * noise_variable_13);
        let noise_metadata_schedule_344_e3466: f64 = (noise_metadata_schedule_344_e3465).exp();
        let noise_metadata_schedule_344_e3467: f64 = (params.p89 * noise_metadata_schedule_344_e3466);
        (noise_metadata_schedule_344_e3467,)
    } else {
        (noise_variable_69,)
    }
};
            noise_variable_69 = noise_metadata_schedule_344_e3469;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_345_e3472: f64 = if params.p43 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_330 = noise_metadata_schedule_345_e3472;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_346_e3496,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 != 0.0)) {
        let noise_metadata_schedule_346_e3478: f64 = (2.0 * noise_variable_6);
        let noise_metadata_schedule_346_e3481: f64 = (params.p44 * 0.5);
        let noise_metadata_schedule_346_e3483: f64 = (noise_metadata_schedule_346_e3481 * noise_variable_7);
        let noise_metadata_schedule_346_e3484: f64 = (noise_metadata_schedule_346_e3483).exp();
        let noise_metadata_schedule_346_e3486: f64 = (-0.5);
        let noise_metadata_schedule_346_e3488: f64 = (noise_metadata_schedule_346_e3486 * params.p44);
        let noise_metadata_schedule_346_e3490: f64 = (noise_metadata_schedule_346_e3488 * noise_variable_7);
        let noise_metadata_schedule_346_e3491: f64 = (noise_metadata_schedule_346_e3490).exp();
        let noise_metadata_schedule_346_e3492: f64 = (noise_metadata_schedule_346_e3484 - noise_metadata_schedule_346_e3491);
        let noise_metadata_schedule_346_e3493: f64 = (noise_metadata_schedule_346_e3492).ln();
        let noise_metadata_schedule_346_e3494: f64 = (noise_metadata_schedule_346_e3478 * noise_metadata_schedule_346_e3493);
        (noise_metadata_schedule_346_e3494,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_346_e3496;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_347_e3516,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 != 0.0)) {
        let noise_metadata_schedule_347_e3502: f64 = (noise_variable_164 * noise_variable_11);
        let noise_metadata_schedule_347_e3506: f64 = (1.0 - noise_variable_11);
        let noise_metadata_schedule_347_e3507: f64 = (noise_variable_77 * noise_metadata_schedule_347_e3506);
        let noise_metadata_schedule_347_e3508: f64 = (noise_metadata_schedule_347_e3502 + noise_metadata_schedule_347_e3507);
        let noise_metadata_schedule_347_e3511: f64 = (noise_variable_76 * noise_variable_4);
        let noise_metadata_schedule_347_e3513: f64 = (noise_metadata_schedule_347_e3511 * noise_variable_13);
        let noise_metadata_schedule_347_e3514: f64 = (noise_metadata_schedule_347_e3508 - noise_metadata_schedule_347_e3513);
        (noise_metadata_schedule_347_e3514,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_347_e3516;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_348_e3542,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 != 0.0)) {
        let noise_metadata_schedule_348_e3523: f64 = (2.0 * noise_variable_4);
        let noise_metadata_schedule_348_e3529: f64 = (-noise_variable_165);
        let noise_metadata_schedule_348_e3531: f64 = (noise_metadata_schedule_348_e3529 * noise_variable_5);
        let noise_metadata_schedule_348_e3532: f64 = (noise_metadata_schedule_348_e3531).exp();
        let noise_metadata_schedule_348_e3533: f64 = (4.0 * noise_metadata_schedule_348_e3532);
        let noise_metadata_schedule_348_e3534: f64 = (1.0 + noise_metadata_schedule_348_e3533);
        let noise_metadata_schedule_348_e3535: f64 = (noise_metadata_schedule_348_e3534).sqrt();
        let noise_metadata_schedule_348_e3536: f64 = (1.0 + noise_metadata_schedule_348_e3535);
        let noise_metadata_schedule_348_e3537: f64 = (0.5 * noise_metadata_schedule_348_e3536);
        let noise_metadata_schedule_348_e3538: f64 = (noise_metadata_schedule_348_e3537).ln();
        let noise_metadata_schedule_348_e3539: f64 = (noise_metadata_schedule_348_e3523 * noise_metadata_schedule_348_e3538);
        let noise_metadata_schedule_348_e3540: f64 = (noise_variable_165 + noise_metadata_schedule_348_e3539);
        (noise_metadata_schedule_348_e3540,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_348_e3542;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_349_e3556,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 != 0.0)) {
        let noise_metadata_schedule_349_e3550: f64 = (params.p44 / noise_variable_30);
        let noise_metadata_schedule_349_e3551: f64 = (noise_metadata_schedule_349_e3550).ln();
        let noise_metadata_schedule_349_e3552: f64 = (params.p45 * noise_metadata_schedule_349_e3551);
        let noise_metadata_schedule_349_e3553: f64 = (noise_metadata_schedule_349_e3552).exp();
        let noise_metadata_schedule_349_e3554: f64 = (params.p43 * noise_metadata_schedule_349_e3553);
        (noise_metadata_schedule_349_e3554,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_349_e3556;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_350_e3563,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 != 0.0)) {
        let noise_metadata_schedule_350_e3561: f64 = (params.p46).abs();
        (noise_metadata_schedule_350_e3561,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_350_e3563;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_351_e3566: f64 = if params.p46 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_331 = noise_metadata_schedule_351_e3566;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_352_e3578,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_330 != 0.0)) && (noise_variable_331 != 0.0)) {
        let noise_metadata_schedule_352_e3574: f64 = (params.p46 * noise_variable_30);
        let noise_metadata_schedule_352_e3576: f64 = (noise_metadata_schedule_352_e3574 / params.p44);
        (noise_metadata_schedule_352_e3576,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_352_e3578;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_353_e3585,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 == 0.0)) {
        (params.p43,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_353_e3585;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_354_e3592,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 == 0.0)) {
        (params.p44,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_354_e3592;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_355_e3599,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_330 == 0.0)) {
        (params.p46,)
    } else {
        (noise_variable_31,)
    }
};
            noise_variable_31 = noise_metadata_schedule_355_e3599;
        }
        if matches!(source_index, 5 | 6 | 7 | 9) {
            let (noise_metadata_schedule_356_e3616,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_356_e3604: f64 = (params.p124 * noise_variable_13);
        let noise_metadata_schedule_356_e3607: f64 = (params.p118 * noise_variable_7);
        let noise_metadata_schedule_356_e3610: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_356_e3611: f64 = (noise_metadata_schedule_356_e3607 * noise_metadata_schedule_356_e3610);
        let noise_metadata_schedule_356_e3612: f64 = (noise_metadata_schedule_356_e3604 + noise_metadata_schedule_356_e3611);
        let noise_metadata_schedule_356_e3613: f64 = (noise_metadata_schedule_356_e3612).exp();
        let noise_metadata_schedule_356_e3614: f64 = (params.p18 * noise_metadata_schedule_356_e3613);
        (noise_metadata_schedule_356_e3614,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_356_e3616;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_358_e3648: f64 = if ((params.p27 > 0.0) && ((noise_variable_205 < noise_variable_223) || (noise_variable_202 < noise_variable_223))) { 1.0 } else { 0.0 };
            noise_variable_332 = noise_metadata_schedule_358_e3648;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_359_e3654,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_359_e3654;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_360_e3660,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) {
        (1.0,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_360_e3660;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_361_e3668,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) {
        let noise_metadata_schedule_361_e3666: f64 = (noise_variable_91 / noise_variable_86);
        (noise_metadata_schedule_361_e3666,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_361_e3668;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_362_e3679: f64 = if (((params.p29 == 1.0) && (params.p43 > 0.0)) && (params.p44 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_333 = noise_metadata_schedule_362_e3679;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_363_e3689,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) && (noise_variable_333 != 0.0)) {
        let noise_metadata_schedule_363_e3687: f64 = (noise_variable_30 / params.p44);
        (noise_metadata_schedule_363_e3687,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_363_e3689;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_364_e3706,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) && (noise_variable_333 != 0.0)) {
        let noise_metadata_schedule_364_e3697: f64 = (noise_variable_29 / params.p43);
        let noise_metadata_schedule_364_e3699: f64 = (noise_variable_169).sqrt();
        let noise_metadata_schedule_364_e3700: f64 = (noise_metadata_schedule_364_e3697 * noise_metadata_schedule_364_e3699);
        let noise_metadata_schedule_364_e3702: f64 = (noise_metadata_schedule_364_e3700 * noise_variable_170);
        let noise_metadata_schedule_364_e3704: f64 = (noise_metadata_schedule_364_e3702 * noise_variable_170);
        (noise_metadata_schedule_364_e3704,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_364_e3706;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_365_e3723,) = {
    if (((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) && (noise_variable_333 != 0.0)) {
        let noise_metadata_schedule_365_e3714: f64 = (params.p43 / noise_variable_29);
        let noise_metadata_schedule_365_e3717: f64 = (-1.5);
        let noise_metadata_schedule_365_e3718: f64 = (noise_variable_169).powf(noise_metadata_schedule_365_e3717);
        let noise_metadata_schedule_365_e3719: f64 = (noise_metadata_schedule_365_e3714 * noise_metadata_schedule_365_e3718);
        let noise_metadata_schedule_365_e3721: f64 = (noise_metadata_schedule_365_e3719 / noise_variable_170);
        (noise_metadata_schedule_365_e3721,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_365_e3723;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_366_e3734: f64 = if (((params.p29 == 0.0) && (params.p39 > 0.0)) && (params.p40 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_334 = noise_metadata_schedule_366_e3734;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_367_e3747,) = {
    if ((((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) && (noise_variable_333 == 0.0)) && (noise_variable_334 != 0.0)) {
        let noise_metadata_schedule_367_e3745: f64 = (noise_variable_27 / params.p40);
        (noise_metadata_schedule_367_e3745,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_367_e3747;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_368_e3767,) = {
    if ((((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) && (noise_variable_333 == 0.0)) && (noise_variable_334 != 0.0)) {
        let noise_metadata_schedule_368_e3758: f64 = (noise_variable_26 / params.p39);
        let noise_metadata_schedule_368_e3760: f64 = (noise_variable_169).sqrt();
        let noise_metadata_schedule_368_e3761: f64 = (noise_metadata_schedule_368_e3758 * noise_metadata_schedule_368_e3760);
        let noise_metadata_schedule_368_e3763: f64 = (noise_metadata_schedule_368_e3761 * noise_variable_170);
        let noise_metadata_schedule_368_e3765: f64 = (noise_metadata_schedule_368_e3763 * noise_variable_170);
        (noise_metadata_schedule_368_e3765,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_368_e3767;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_369_e3787,) = {
    if ((((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) && (noise_variable_333 == 0.0)) && (noise_variable_334 != 0.0)) {
        let noise_metadata_schedule_369_e3778: f64 = (params.p39 / noise_variable_26);
        let noise_metadata_schedule_369_e3781: f64 = (-1.5);
        let noise_metadata_schedule_369_e3782: f64 = (noise_variable_169).powf(noise_metadata_schedule_369_e3781);
        let noise_metadata_schedule_369_e3783: f64 = (noise_metadata_schedule_369_e3778 * noise_metadata_schedule_369_e3782);
        let noise_metadata_schedule_369_e3785: f64 = (noise_metadata_schedule_369_e3783 / noise_variable_170);
        (noise_metadata_schedule_369_e3785,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_369_e3787;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_370_e3795,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) {
        let noise_metadata_schedule_370_e3793: f64 = (params.p27 * noise_variable_167);
        (noise_metadata_schedule_370_e3793,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_370_e3795;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_371_e3803,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_332 != 0.0)) {
        let noise_metadata_schedule_371_e3801: f64 = (params.p28 * noise_variable_166);
        (noise_metadata_schedule_371_e3801,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_371_e3803;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_372_e3810,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_332 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_64,)
    }
};
            noise_variable_64 = noise_metadata_schedule_372_e3810;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_373_e3817,) = {
    if ((noise_variable_317 != 0.0) && (noise_variable_332 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_65,)
    }
};
            noise_variable_65 = noise_metadata_schedule_373_e3817;
        }
        if matches!(source_index, 13) {
            let (noise_metadata_schedule_390_e3997,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_390_e3985: f64 = (noise_variable_81 * noise_variable_13);
        let noise_metadata_schedule_390_e3988: f64 = (params.p119 * noise_variable_7);
        let noise_metadata_schedule_390_e3991: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_390_e3992: f64 = (noise_metadata_schedule_390_e3988 * noise_metadata_schedule_390_e3991);
        let noise_metadata_schedule_390_e3993: f64 = (noise_metadata_schedule_390_e3985 + noise_metadata_schedule_390_e3992);
        let noise_metadata_schedule_390_e3994: f64 = (noise_metadata_schedule_390_e3993).exp();
        let noise_metadata_schedule_390_e3995: f64 = (params.p25 * noise_metadata_schedule_390_e3994);
        (noise_metadata_schedule_390_e3995,)
    } else {
        (noise_variable_36,)
    }
};
            noise_variable_36 = noise_metadata_schedule_390_e3997;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_416_e4343,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_416_e4331: f64 = (noise_variable_82 * noise_variable_13);
        let noise_metadata_schedule_416_e4334: f64 = (params.p120 * noise_variable_7);
        let noise_metadata_schedule_416_e4337: f64 = (1.0 - noise_variable_12);
        let noise_metadata_schedule_416_e4338: f64 = (noise_metadata_schedule_416_e4334 * noise_metadata_schedule_416_e4337);
        let noise_metadata_schedule_416_e4339: f64 = (noise_metadata_schedule_416_e4331 + noise_metadata_schedule_416_e4338);
        let noise_metadata_schedule_416_e4340: f64 = (noise_metadata_schedule_416_e4339).exp();
        let noise_metadata_schedule_416_e4341: f64 = (params.p99 * noise_metadata_schedule_416_e4340);
        (noise_metadata_schedule_416_e4341,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_416_e4343;
        }
        if matches!(source_index, 2) {
            let (noise_metadata_schedule_434_e4556,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_434_e4552: f64 = (params.p136 * noise_variable_13);
        let noise_metadata_schedule_434_e4553: f64 = (noise_metadata_schedule_434_e4552).exp();
        let noise_metadata_schedule_434_e4554: f64 = (params.p96 * noise_metadata_schedule_434_e4553);
        (noise_metadata_schedule_434_e4554,)
    } else {
        (noise_variable_72,)
    }
};
            noise_variable_72 = noise_metadata_schedule_434_e4556;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_435_e4565,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_435_e4561: f64 = (params.p135 * noise_variable_13);
        let noise_metadata_schedule_435_e4562: f64 = (noise_metadata_schedule_435_e4561).exp();
        let noise_metadata_schedule_435_e4563: f64 = (params.p90 * noise_metadata_schedule_435_e4562);
        (noise_metadata_schedule_435_e4563,)
    } else {
        (noise_variable_71,)
    }
};
            noise_variable_71 = noise_metadata_schedule_435_e4565;
        }
        if matches!(source_index, 3 | 5 | 6 | 7) {
            let (noise_metadata_schedule_436_e4574,) = {
    if (noise_variable_317 != 0.0) {
        let noise_metadata_schedule_436_e4570: f64 = (params.p137 * noise_variable_13);
        let noise_metadata_schedule_436_e4571: f64 = (noise_metadata_schedule_436_e4570).exp();
        let noise_metadata_schedule_436_e4572: f64 = (params.p95 * noise_metadata_schedule_436_e4571);
        (noise_metadata_schedule_436_e4572,)
    } else {
        (noise_variable_73,)
    }
};
            noise_variable_73 = noise_metadata_schedule_436_e4574;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 11 | 13 | 14 | 15 | 18) {
            let noise_metadata_schedule_438_e4592: f64 = if params.p14 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_364 = noise_metadata_schedule_438_e4592;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 11 | 13 | 14 | 15 | 18) {
            let (noise_metadata_schedule_439_e4600,) = {
    if (noise_variable_364 != 0.0) {
        let noise_metadata_schedule_439_e4597: f64 = (params.p15 * noise_variable_4);
        let noise_metadata_schedule_439_e4598: f64 = (noise_variable_202 / noise_metadata_schedule_439_e4597);
        (noise_metadata_schedule_439_e4598,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_439_e4600;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 11 | 13 | 14 | 15 | 18) {
            let noise_metadata_schedule_440_e4603: f64 = if noise_variable_93 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_365 = noise_metadata_schedule_440_e4603;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 11 | 13 | 14 | 15 | 18) {
            let (noise_metadata_schedule_441_e4613,) = {
    if ((noise_variable_364 != 0.0) && (noise_variable_365 != 0.0)) {
        let noise_metadata_schedule_441_e4610: f64 = (noise_variable_93 - 80.0);
        let noise_metadata_schedule_441_e4611: f64 = (1.0 + noise_metadata_schedule_441_e4610);
        (noise_metadata_schedule_441_e4611,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_441_e4613;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 11 | 13 | 14 | 15 | 18) {
            let (noise_metadata_schedule_442_e4619,) = {
    if ((noise_variable_364 != 0.0) && (noise_variable_365 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_442_e4619;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 9 | 11 | 13 | 14 | 15 | 18) {
            let (noise_metadata_schedule_443_e4626,) = {
    if ((noise_variable_364 != 0.0) && (noise_variable_365 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_443_e4626;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 15 | 18) {
            let (noise_metadata_schedule_444_e4637,) = {
    if (noise_variable_364 != 0.0) {
        let noise_metadata_schedule_444_e4631: f64 = { let limexp_arg = noise_variable_93; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_444_e4632: f64 = (noise_variable_94 * noise_metadata_schedule_444_e4631);
        let noise_metadata_schedule_444_e4634: f64 = (noise_metadata_schedule_444_e4632 - 1.0);
        let noise_metadata_schedule_444_e4635: f64 = (noise_variable_22 * noise_metadata_schedule_444_e4634);
        (noise_metadata_schedule_444_e4635,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_444_e4637;
        }
        if matches!(source_index, 1 | 5 | 6 | 7 | 15 | 18) {
            let (noise_metadata_schedule_445_e4642,) = {
    if (noise_variable_364 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_445_e4642;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let noise_metadata_schedule_446_e4645: f64 = if params.p16 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_366 = noise_metadata_schedule_446_e4645;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_447_e4653,) = {
    if (noise_variable_366 != 0.0) {
        let noise_metadata_schedule_447_e4650: f64 = (params.p17 * noise_variable_4);
        let noise_metadata_schedule_447_e4651: f64 = (noise_variable_202 / noise_metadata_schedule_447_e4650);
        (noise_metadata_schedule_447_e4651,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_447_e4653;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let noise_metadata_schedule_448_e4656: f64 = if noise_variable_93 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_367 = noise_metadata_schedule_448_e4656;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_449_e4666,) = {
    if ((noise_variable_366 != 0.0) && (noise_variable_367 != 0.0)) {
        let noise_metadata_schedule_449_e4663: f64 = (noise_variable_93 - 80.0);
        let noise_metadata_schedule_449_e4664: f64 = (1.0 + noise_metadata_schedule_449_e4663);
        (noise_metadata_schedule_449_e4664,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_449_e4666;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_450_e4672,) = {
    if ((noise_variable_366 != 0.0) && (noise_variable_367 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_450_e4672;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_451_e4679,) = {
    if ((noise_variable_366 != 0.0) && (noise_variable_367 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_451_e4679;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_454_e4699: f64 = (noise_variable_202 * noise_variable_5);
            let noise_metadata_schedule_454_e4701: f64 = (noise_metadata_schedule_454_e4699 / params.p13);
            let noise_metadata_schedule_454_e4702: f64 = { let limexp_arg = noise_metadata_schedule_454_e4701; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_454_e4703: f64 = (noise_variable_15 * noise_metadata_schedule_454_e4702);
            noise_variable_350 = noise_metadata_schedule_454_e4703;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_455_e4707: f64 = (noise_variable_203 * noise_variable_5);
            let noise_metadata_schedule_455_e4708: f64 = { let limexp_arg = noise_metadata_schedule_455_e4707; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_455_e4709: f64 = (noise_variable_15 * noise_metadata_schedule_455_e4708);
            noise_variable_351 = noise_metadata_schedule_455_e4709;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_456_e4712: f64 = if noise_variable_26 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_368 = noise_metadata_schedule_456_e4712;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_457_e4725,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_457_e4717: f64 = (noise_variable_28).ln();
        let noise_metadata_schedule_457_e4718: f64 = (-noise_metadata_schedule_457_e4717);
        let noise_metadata_schedule_457_e4720: f64 = (noise_metadata_schedule_457_e4718 / params.p41);
        let noise_metadata_schedule_457_e4721: f64 = (noise_metadata_schedule_457_e4720).exp();
        let noise_metadata_schedule_457_e4722: f64 = (1.0 - noise_metadata_schedule_457_e4721);
        let noise_metadata_schedule_457_e4723: f64 = (noise_variable_27 * noise_metadata_schedule_457_e4722);
        (noise_metadata_schedule_457_e4723,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_457_e4725;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_458_e4733,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_458_e4729: f64 = (noise_variable_137 - noise_variable_202);
        let noise_metadata_schedule_458_e4731: f64 = (noise_metadata_schedule_458_e4729 * noise_variable_5);
        (noise_metadata_schedule_458_e4731,)
    } else {
        (noise_variable_141,)
    }
};
            noise_variable_141 = noise_metadata_schedule_458_e4733;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_459_e4742,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_459_e4737: f64 = (noise_variable_141 * noise_variable_141);
        let noise_metadata_schedule_459_e4739: f64 = (noise_metadata_schedule_459_e4737 + 1.921812);
        let noise_metadata_schedule_459_e4740: f64 = (noise_metadata_schedule_459_e4739).sqrt();
        (noise_metadata_schedule_459_e4740,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_459_e4742;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_460_e4750,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_460_e4746: f64 = (noise_variable_141 + noise_variable_142);
        let noise_metadata_schedule_460_e4748: f64 = (noise_metadata_schedule_460_e4746 * 0.5);
        (noise_metadata_schedule_460_e4748,)
    } else {
        (noise_variable_143,)
    }
};
            noise_variable_143 = noise_metadata_schedule_460_e4750;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_461_e4758,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_461_e4755: f64 = (noise_variable_4 * noise_variable_143);
        let noise_metadata_schedule_461_e4756: f64 = (noise_variable_137 - noise_metadata_schedule_461_e4755);
        (noise_metadata_schedule_461_e4756,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_461_e4758;
        }
        if matches!(source_index, 8 | 10 | 12) {
            let (noise_metadata_schedule_462_e4764,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_462_e4762: f64 = (noise_variable_143 / noise_variable_142);
        (noise_metadata_schedule_462_e4762,)
    } else {
        (noise_variable_144,)
    }
};
            noise_variable_144 = noise_metadata_schedule_462_e4764;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_463_e4773,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_463_e4769: f64 = (noise_variable_138 / noise_variable_27);
        let noise_metadata_schedule_463_e4770: f64 = (1.0 - noise_metadata_schedule_463_e4769);
        let noise_metadata_schedule_463_e4771: f64 = (noise_metadata_schedule_463_e4770).ln();
        (noise_metadata_schedule_463_e4771,)
    } else {
        (noise_variable_139,)
    }
};
            noise_variable_139 = noise_metadata_schedule_463_e4773;
        }
        if matches!(source_index, 8 | 10 | 12) {
            let (noise_metadata_schedule_464_e4783,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_464_e4776: f64 = (-params.p41);
        let noise_metadata_schedule_464_e4778: f64 = (noise_metadata_schedule_464_e4776 * noise_variable_139);
        let noise_metadata_schedule_464_e4779: f64 = (noise_metadata_schedule_464_e4778).exp();
        let noise_metadata_schedule_464_e4781: f64 = (noise_metadata_schedule_464_e4779 * noise_variable_144);
        (noise_metadata_schedule_464_e4781,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_464_e4783;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_465_e4795,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_465_e4790: f64 = (1.0 - noise_variable_144);
        let noise_metadata_schedule_465_e4791: f64 = (noise_variable_28 * noise_metadata_schedule_465_e4790);
        let noise_metadata_schedule_465_e4792: f64 = (noise_variable_145 + noise_metadata_schedule_465_e4791);
        let noise_metadata_schedule_465_e4793: f64 = (noise_variable_26 * noise_metadata_schedule_465_e4792);
        (noise_metadata_schedule_465_e4793,)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_465_e4795;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_466_e4812,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_466_e4802: f64 = (1.0 - params.p41);
        let noise_metadata_schedule_466_e4803: f64 = (noise_variable_139 * noise_metadata_schedule_466_e4802);
        let noise_metadata_schedule_466_e4804: f64 = (noise_metadata_schedule_466_e4803).exp();
        let noise_metadata_schedule_466_e4805: f64 = (1.0 - noise_metadata_schedule_466_e4804);
        let noise_metadata_schedule_466_e4806: f64 = (noise_variable_27 * noise_metadata_schedule_466_e4805);
        let noise_metadata_schedule_466_e4809: f64 = (1.0 - params.p41);
        let noise_metadata_schedule_466_e4810: f64 = (noise_metadata_schedule_466_e4806 / noise_metadata_schedule_466_e4809);
        (noise_metadata_schedule_466_e4810,)
    } else {
        (noise_variable_140,)
    }
};
            noise_variable_140 = noise_metadata_schedule_466_e4812;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_467_e4824,) = {
    if (noise_variable_368 != 0.0) {
        let noise_metadata_schedule_467_e4819: f64 = (noise_variable_202 - noise_variable_138);
        let noise_metadata_schedule_467_e4820: f64 = (noise_variable_28 * noise_metadata_schedule_467_e4819);
        let noise_metadata_schedule_467_e4821: f64 = (noise_variable_140 + noise_metadata_schedule_467_e4820);
        let noise_metadata_schedule_467_e4822: f64 = (noise_variable_26 * noise_metadata_schedule_467_e4821);
        (noise_metadata_schedule_467_e4822,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_467_e4824;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_468_e4829,) = {
    if (noise_variable_368 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_211,)
    }
};
            noise_variable_211 = noise_metadata_schedule_468_e4829;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_469_e4834,) = {
    if (noise_variable_368 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_469_e4834;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_470_e4837: f64 = if params.p51 < 100.0 { 1.0 } else { 0.0 };
            noise_variable_369 = noise_metadata_schedule_470_e4837;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_471_e4840: f64 = if noise_variable_33 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_370 = noise_metadata_schedule_471_e4840;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_472_e4848,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_472_e4846: f64 = (params.p49 / 4.0);
        (noise_metadata_schedule_472_e4846,)
    } else {
        (noise_variable_113,)
    }
};
            noise_variable_113 = noise_metadata_schedule_472_e4848;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_473_e4856,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_473_e4854: f64 = (params.p51 - noise_variable_34);
        (noise_metadata_schedule_473_e4854,)
    } else {
        (noise_variable_114,)
    }
};
            noise_variable_114 = noise_metadata_schedule_473_e4856;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_474_e4871,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_474_e4863: f64 = (noise_variable_35).ln();
        let noise_metadata_schedule_474_e4864: f64 = (-noise_metadata_schedule_474_e4863);
        let noise_metadata_schedule_474_e4866: f64 = (noise_metadata_schedule_474_e4864 / params.p49);
        let noise_metadata_schedule_474_e4867: f64 = (noise_metadata_schedule_474_e4866).exp();
        let noise_metadata_schedule_474_e4868: f64 = (1.0 - noise_metadata_schedule_474_e4867);
        let noise_metadata_schedule_474_e4869: f64 = (noise_variable_34 * noise_metadata_schedule_474_e4868);
        (noise_metadata_schedule_474_e4869,)
    } else {
        (noise_variable_115,)
    }
};
            noise_variable_115 = noise_metadata_schedule_474_e4871;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_475_e4879,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_475_e4877: f64 = (noise_variable_35 * noise_variable_33);
        (noise_metadata_schedule_475_e4877,)
    } else {
        (noise_variable_116,)
    }
};
            noise_variable_116 = noise_metadata_schedule_475_e4879;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_476_e4895,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_476_e4886: f64 = (noise_variable_113 - params.p49);
        let noise_metadata_schedule_476_e4889: f64 = (params.p51 / noise_variable_34);
        let noise_metadata_schedule_476_e4890: f64 = (noise_metadata_schedule_476_e4889).ln();
        let noise_metadata_schedule_476_e4891: f64 = (noise_metadata_schedule_476_e4886 * noise_metadata_schedule_476_e4890);
        let noise_metadata_schedule_476_e4892: f64 = (noise_metadata_schedule_476_e4891).exp();
        let noise_metadata_schedule_476_e4893: f64 = (noise_variable_33 * noise_metadata_schedule_476_e4892);
        (noise_metadata_schedule_476_e4893,)
    } else {
        (noise_variable_117,)
    }
};
            noise_variable_117 = noise_metadata_schedule_476_e4895;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_477_e4905,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_477_e4901: f64 = (noise_variable_115 - noise_variable_203);
        let noise_metadata_schedule_477_e4903: f64 = (noise_metadata_schedule_477_e4901 * noise_variable_5);
        (noise_metadata_schedule_477_e4903,)
    } else {
        (noise_variable_119,)
    }
};
            noise_variable_119 = noise_metadata_schedule_477_e4905;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_478_e4908: f64 = if noise_variable_119 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_371 = noise_metadata_schedule_478_e4908;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_479_e4917,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_371 != 0.0)) {
        let noise_metadata_schedule_479_e4915: f64 = (noise_variable_119).exp();
        (noise_metadata_schedule_479_e4915,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_479_e4917;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_480_e4929,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_371 != 0.0)) {
        let noise_metadata_schedule_480_e4926: f64 = (1.0 + noise_variable_120);
        let noise_metadata_schedule_480_e4927: f64 = (noise_variable_120 / noise_metadata_schedule_480_e4926);
        (noise_metadata_schedule_480_e4927,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_480_e4929;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_481_e4944,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_371 != 0.0)) {
        let noise_metadata_schedule_481_e4939: f64 = (1.0 + noise_variable_120);
        let noise_metadata_schedule_481_e4940: f64 = (noise_metadata_schedule_481_e4939).ln();
        let noise_metadata_schedule_481_e4941: f64 = (noise_variable_4 * noise_metadata_schedule_481_e4940);
        let noise_metadata_schedule_481_e4942: f64 = (noise_variable_115 - noise_metadata_schedule_481_e4941);
        (noise_metadata_schedule_481_e4942,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_481_e4944;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_482_e4953,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_371 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_121,)
    }
};
            noise_variable_121 = noise_metadata_schedule_482_e4953;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_483_e4962,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_371 == 0.0)) {
        (noise_variable_203,)
    } else {
        (noise_variable_122,)
    }
};
            noise_variable_122 = noise_metadata_schedule_483_e4962;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_484_e4974,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_484_e4968: f64 = (0.1 * noise_variable_114);
        let noise_metadata_schedule_484_e4971: f64 = (4.0 * noise_variable_4);
        let noise_metadata_schedule_484_e4972: f64 = (noise_metadata_schedule_484_e4968 + noise_metadata_schedule_484_e4971);
        (noise_metadata_schedule_484_e4972,)
    } else {
        (noise_variable_118,)
    }
};
            noise_variable_118 = noise_metadata_schedule_484_e4974;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_485_e4984,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_485_e4980: f64 = (noise_variable_114 + noise_variable_122);
        let noise_metadata_schedule_485_e4982: f64 = (noise_metadata_schedule_485_e4980 / noise_variable_118);
        (noise_metadata_schedule_485_e4982,)
    } else {
        (noise_variable_123,)
    }
};
            noise_variable_123 = noise_metadata_schedule_485_e4984;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_486_e4987: f64 = if noise_variable_123 < 80.0 { 1.0 } else { 0.0 };
            noise_variable_372 = noise_metadata_schedule_486_e4987;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_487_e4996,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_372 != 0.0)) {
        let noise_metadata_schedule_487_e4994: f64 = (noise_variable_123).exp();
        (noise_metadata_schedule_487_e4994,)
    } else {
        (noise_variable_120,)
    }
};
            noise_variable_120 = noise_metadata_schedule_487_e4996;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_488_e5008,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_372 != 0.0)) {
        let noise_metadata_schedule_488_e5005: f64 = (1.0 + noise_variable_120);
        let noise_metadata_schedule_488_e5006: f64 = (noise_variable_120 / noise_metadata_schedule_488_e5005);
        (noise_metadata_schedule_488_e5006,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_488_e5008;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_489_e5032,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_372 != 0.0)) {
        let noise_metadata_schedule_489_e5015: f64 = (-noise_variable_114);
        let noise_metadata_schedule_489_e5019: f64 = (1.0 + noise_variable_120);
        let noise_metadata_schedule_489_e5020: f64 = (noise_metadata_schedule_489_e5019).ln();
        let noise_metadata_schedule_489_e5023: f64 = (noise_variable_114 + noise_variable_115);
        let noise_metadata_schedule_489_e5024: f64 = (-noise_metadata_schedule_489_e5023);
        let noise_metadata_schedule_489_e5026: f64 = (noise_metadata_schedule_489_e5024 / noise_variable_118);
        let noise_metadata_schedule_489_e5027: f64 = (noise_metadata_schedule_489_e5026).exp();
        let noise_metadata_schedule_489_e5028: f64 = (noise_metadata_schedule_489_e5020 - noise_metadata_schedule_489_e5027);
        let noise_metadata_schedule_489_e5029: f64 = (noise_variable_118 * noise_metadata_schedule_489_e5028);
        let noise_metadata_schedule_489_e5030: f64 = (noise_metadata_schedule_489_e5015 + noise_metadata_schedule_489_e5029);
        (noise_metadata_schedule_489_e5030,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_489_e5032;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_490_e5041,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_372 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_124,)
    }
};
            noise_variable_124 = noise_metadata_schedule_490_e5041;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_491_e5050,) = {
    if (((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) && (noise_variable_372 == 0.0)) {
        (noise_variable_122,)
    } else {
        (noise_variable_125,)
    }
};
            noise_variable_125 = noise_metadata_schedule_491_e5050;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_492_e5058,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_492_e5056: f64 = (noise_variable_203 - noise_variable_122);
        (noise_metadata_schedule_492_e5056,)
    } else {
        (noise_variable_126,)
    }
};
            noise_variable_126 = noise_metadata_schedule_492_e5058;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_493_e5069,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_493_e5065: f64 = (noise_variable_122 / noise_variable_34);
        let noise_metadata_schedule_493_e5066: f64 = (1.0 - noise_metadata_schedule_493_e5065);
        let noise_metadata_schedule_493_e5067: f64 = (noise_metadata_schedule_493_e5066).ln();
        (noise_metadata_schedule_493_e5067,)
    } else {
        (noise_variable_130,)
    }
};
            noise_variable_130 = noise_metadata_schedule_493_e5069;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_494_e5080,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_494_e5076: f64 = (noise_variable_125 / noise_variable_34);
        let noise_metadata_schedule_494_e5077: f64 = (1.0 - noise_metadata_schedule_494_e5076);
        let noise_metadata_schedule_494_e5078: f64 = (noise_metadata_schedule_494_e5077).ln();
        (noise_metadata_schedule_494_e5078,)
    } else {
        (noise_variable_131,)
    }
};
            noise_variable_131 = noise_metadata_schedule_494_e5080;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_495_e5088,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_495_e5086: f64 = (1.0 - params.p49);
        (noise_metadata_schedule_495_e5086,)
    } else {
        (noise_variable_132,)
    }
};
            noise_variable_132 = noise_metadata_schedule_495_e5088;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_496_e5096,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_496_e5094: f64 = (1.0 - noise_variable_113);
        (noise_metadata_schedule_496_e5094,)
    } else {
        (noise_variable_133,)
    }
};
            noise_variable_133 = noise_metadata_schedule_496_e5096;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_497_e5112,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_497_e5103: f64 = (-params.p49);
        let noise_metadata_schedule_497_e5104: f64 = (noise_variable_131 * noise_metadata_schedule_497_e5103);
        let noise_metadata_schedule_497_e5105: f64 = (noise_metadata_schedule_497_e5104).exp();
        let noise_metadata_schedule_497_e5106: f64 = (noise_variable_33 * noise_metadata_schedule_497_e5105);
        let noise_metadata_schedule_497_e5108: f64 = (noise_metadata_schedule_497_e5106 * noise_variable_121);
        let noise_metadata_schedule_497_e5110: f64 = (noise_metadata_schedule_497_e5108 * noise_variable_124);
        (noise_metadata_schedule_497_e5110,)
    } else {
        (noise_variable_134,)
    }
};
            noise_variable_134 = noise_metadata_schedule_497_e5112;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_498_e5128,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_498_e5119: f64 = (-noise_variable_113);
        let noise_metadata_schedule_498_e5120: f64 = (noise_variable_130 * noise_metadata_schedule_498_e5119);
        let noise_metadata_schedule_498_e5121: f64 = (noise_metadata_schedule_498_e5120).exp();
        let noise_metadata_schedule_498_e5122: f64 = (noise_variable_117 * noise_metadata_schedule_498_e5121);
        let noise_metadata_schedule_498_e5125: f64 = (1.0 - noise_variable_124);
        let noise_metadata_schedule_498_e5126: f64 = (noise_metadata_schedule_498_e5122 * noise_metadata_schedule_498_e5125);
        (noise_metadata_schedule_498_e5126,)
    } else {
        (noise_variable_135,)
    }
};
            noise_variable_135 = noise_metadata_schedule_498_e5128;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_499_e5138,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_499_e5135: f64 = (1.0 - noise_variable_121);
        let noise_metadata_schedule_499_e5136: f64 = (noise_variable_116 * noise_metadata_schedule_499_e5135);
        (noise_metadata_schedule_499_e5136,)
    } else {
        (noise_variable_136,)
    }
};
            noise_variable_136 = noise_metadata_schedule_499_e5138;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_500_e5148,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_500_e5144: f64 = (noise_variable_134 + noise_variable_135);
        let noise_metadata_schedule_500_e5146: f64 = (noise_metadata_schedule_500_e5144 + noise_variable_136);
        (noise_metadata_schedule_500_e5146,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_500_e5148;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_501_e5163,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_501_e5156: f64 = (noise_variable_131 * noise_variable_132);
        let noise_metadata_schedule_501_e5157: f64 = (noise_metadata_schedule_501_e5156).exp();
        let noise_metadata_schedule_501_e5158: f64 = (1.0 - noise_metadata_schedule_501_e5157);
        let noise_metadata_schedule_501_e5159: f64 = (noise_variable_33 * noise_metadata_schedule_501_e5158);
        let noise_metadata_schedule_501_e5161: f64 = (noise_metadata_schedule_501_e5159 / noise_variable_132);
        (noise_metadata_schedule_501_e5161,)
    } else {
        (noise_variable_127,)
    }
};
            noise_variable_127 = noise_metadata_schedule_501_e5163;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_502_e5178,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_502_e5171: f64 = (noise_variable_130 * noise_variable_133);
        let noise_metadata_schedule_502_e5172: f64 = (noise_metadata_schedule_502_e5171).exp();
        let noise_metadata_schedule_502_e5173: f64 = (1.0 - noise_metadata_schedule_502_e5172);
        let noise_metadata_schedule_502_e5174: f64 = (noise_variable_117 * noise_metadata_schedule_502_e5173);
        let noise_metadata_schedule_502_e5176: f64 = (noise_metadata_schedule_502_e5174 / noise_variable_133);
        (noise_metadata_schedule_502_e5176,)
    } else {
        (noise_variable_128,)
    }
};
            noise_variable_128 = noise_metadata_schedule_502_e5178;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_503_e5193,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_503_e5186: f64 = (noise_variable_131 * noise_variable_133);
        let noise_metadata_schedule_503_e5187: f64 = (noise_metadata_schedule_503_e5186).exp();
        let noise_metadata_schedule_503_e5188: f64 = (1.0 - noise_metadata_schedule_503_e5187);
        let noise_metadata_schedule_503_e5189: f64 = (noise_variable_117 * noise_metadata_schedule_503_e5188);
        let noise_metadata_schedule_503_e5191: f64 = (noise_metadata_schedule_503_e5189 / noise_variable_133);
        (noise_metadata_schedule_503_e5191,)
    } else {
        (noise_variable_129,)
    }
};
            noise_variable_129 = noise_metadata_schedule_503_e5193;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_504_e5209,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 != 0.0)) {
        let noise_metadata_schedule_504_e5199: f64 = (noise_variable_127 + noise_variable_128);
        let noise_metadata_schedule_504_e5201: f64 = (noise_metadata_schedule_504_e5199 - noise_variable_129);
        let noise_metadata_schedule_504_e5203: f64 = (noise_metadata_schedule_504_e5201 * noise_variable_34);
        let noise_metadata_schedule_504_e5206: f64 = (noise_variable_116 * noise_variable_126);
        let noise_metadata_schedule_504_e5207: f64 = (noise_metadata_schedule_504_e5203 + noise_metadata_schedule_504_e5206);
        (noise_metadata_schedule_504_e5207,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_504_e5209;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_505_e5216,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_505_e5216;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_506_e5223,) = {
    if ((noise_variable_369 != 0.0) && (noise_variable_370 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_506_e5223;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_507_e5226: f64 = if noise_variable_33 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_373 = noise_metadata_schedule_507_e5226;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_508_e5242,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_508_e5234: f64 = (noise_variable_35).ln();
        let noise_metadata_schedule_508_e5235: f64 = (-noise_metadata_schedule_508_e5234);
        let noise_metadata_schedule_508_e5237: f64 = (noise_metadata_schedule_508_e5235 / params.p49);
        let noise_metadata_schedule_508_e5238: f64 = (noise_metadata_schedule_508_e5237).exp();
        let noise_metadata_schedule_508_e5239: f64 = (1.0 - noise_metadata_schedule_508_e5238);
        let noise_metadata_schedule_508_e5240: f64 = (noise_variable_34 * noise_metadata_schedule_508_e5239);
        (noise_metadata_schedule_508_e5240,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_508_e5242;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_509_e5253,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_509_e5249: f64 = (noise_variable_137 - noise_variable_203);
        let noise_metadata_schedule_509_e5251: f64 = (noise_metadata_schedule_509_e5249 * noise_variable_5);
        (noise_metadata_schedule_509_e5251,)
    } else {
        (noise_variable_141,)
    }
};
            noise_variable_141 = noise_metadata_schedule_509_e5253;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_510_e5265,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_510_e5260: f64 = (noise_variable_141 * noise_variable_141);
        let noise_metadata_schedule_510_e5262: f64 = (noise_metadata_schedule_510_e5260 + 1.921812);
        let noise_metadata_schedule_510_e5263: f64 = (noise_metadata_schedule_510_e5262).sqrt();
        (noise_metadata_schedule_510_e5263,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_510_e5265;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_511_e5276,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_511_e5272: f64 = (noise_variable_141 + noise_variable_142);
        let noise_metadata_schedule_511_e5274: f64 = (noise_metadata_schedule_511_e5272 * 0.5);
        (noise_metadata_schedule_511_e5274,)
    } else {
        (noise_variable_143,)
    }
};
            noise_variable_143 = noise_metadata_schedule_511_e5276;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_512_e5287,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_512_e5284: f64 = (noise_variable_4 * noise_variable_143);
        let noise_metadata_schedule_512_e5285: f64 = (noise_variable_137 - noise_metadata_schedule_512_e5284);
        (noise_metadata_schedule_512_e5285,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_512_e5287;
        }
        if matches!(source_index, 8 | 10 | 12) {
            let (noise_metadata_schedule_513_e5296,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_513_e5294: f64 = (noise_variable_143 / noise_variable_142);
        (noise_metadata_schedule_513_e5294,)
    } else {
        (noise_variable_144,)
    }
};
            noise_variable_144 = noise_metadata_schedule_513_e5296;
        }
        if matches!(source_index, 1 | 8 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_514_e5308,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_514_e5304: f64 = (noise_variable_138 / noise_variable_34);
        let noise_metadata_schedule_514_e5305: f64 = (1.0 - noise_metadata_schedule_514_e5304);
        let noise_metadata_schedule_514_e5306: f64 = (noise_metadata_schedule_514_e5305).ln();
        (noise_metadata_schedule_514_e5306,)
    } else {
        (noise_variable_139,)
    }
};
            noise_variable_139 = noise_metadata_schedule_514_e5308;
        }
        if matches!(source_index, 8 | 10 | 12) {
            let (noise_metadata_schedule_515_e5321,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_515_e5314: f64 = (-params.p49);
        let noise_metadata_schedule_515_e5316: f64 = (noise_metadata_schedule_515_e5314 * noise_variable_139);
        let noise_metadata_schedule_515_e5317: f64 = (noise_metadata_schedule_515_e5316).exp();
        let noise_metadata_schedule_515_e5319: f64 = (noise_metadata_schedule_515_e5317 * noise_variable_144);
        (noise_metadata_schedule_515_e5319,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_515_e5321;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_516_e5336,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_516_e5331: f64 = (1.0 - noise_variable_144);
        let noise_metadata_schedule_516_e5332: f64 = (noise_variable_35 * noise_metadata_schedule_516_e5331);
        let noise_metadata_schedule_516_e5333: f64 = (noise_variable_145 + noise_metadata_schedule_516_e5332);
        let noise_metadata_schedule_516_e5334: f64 = (noise_variable_33 * noise_metadata_schedule_516_e5333);
        (noise_metadata_schedule_516_e5334,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_516_e5336;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_517_e5356,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_517_e5346: f64 = (1.0 - params.p49);
        let noise_metadata_schedule_517_e5347: f64 = (noise_variable_139 * noise_metadata_schedule_517_e5346);
        let noise_metadata_schedule_517_e5348: f64 = (noise_metadata_schedule_517_e5347).exp();
        let noise_metadata_schedule_517_e5349: f64 = (1.0 - noise_metadata_schedule_517_e5348);
        let noise_metadata_schedule_517_e5350: f64 = (noise_variable_34 * noise_metadata_schedule_517_e5349);
        let noise_metadata_schedule_517_e5353: f64 = (1.0 - params.p49);
        let noise_metadata_schedule_517_e5354: f64 = (noise_metadata_schedule_517_e5350 / noise_metadata_schedule_517_e5353);
        (noise_metadata_schedule_517_e5354,)
    } else {
        (noise_variable_140,)
    }
};
            noise_variable_140 = noise_metadata_schedule_517_e5356;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_518_e5371,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 != 0.0)) {
        let noise_metadata_schedule_518_e5366: f64 = (noise_variable_203 - noise_variable_138);
        let noise_metadata_schedule_518_e5367: f64 = (noise_variable_35 * noise_metadata_schedule_518_e5366);
        let noise_metadata_schedule_518_e5368: f64 = (noise_variable_140 + noise_metadata_schedule_518_e5367);
        let noise_metadata_schedule_518_e5369: f64 = (noise_variable_33 * noise_metadata_schedule_518_e5368);
        (noise_metadata_schedule_518_e5369,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_518_e5371;
        }
        if matches!(source_index, 10 | 12) {
            let (noise_metadata_schedule_519_e5379,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_210,)
    }
};
            noise_variable_210 = noise_metadata_schedule_519_e5379;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_520_e5387,) = {
    if ((noise_variable_369 == 0.0) && (noise_variable_373 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_520_e5387;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_521_e5390: f64 = if params.p10 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_374 = noise_metadata_schedule_521_e5390;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_522_e5396,) = {
    if (noise_variable_374 != 0.0) {
        let noise_metadata_schedule_522_e5394: f64 = (params.p11 * noise_variable_4);
        (noise_metadata_schedule_522_e5394,)
    } else {
        (noise_variable_375,)
    }
};
            noise_variable_375 = noise_metadata_schedule_522_e5396;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_523_e5404,) = {
    if (noise_variable_374 != 0.0) {
        let noise_metadata_schedule_523_e5400: f64 = (noise_variable_27 - noise_variable_202);
        let noise_metadata_schedule_523_e5402: f64 = (noise_metadata_schedule_523_e5400 / noise_variable_375);
        (noise_metadata_schedule_523_e5402,)
    } else {
        (noise_variable_376,)
    }
};
            noise_variable_376 = noise_metadata_schedule_523_e5404;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_524_e5421,) = {
    if (noise_variable_374 != 0.0) {
        let noise_metadata_schedule_524_e5411: f64 = (noise_variable_376 * noise_variable_376);
        let noise_metadata_schedule_524_e5413: f64 = (noise_metadata_schedule_524_e5411 + 1.921812);
        let noise_metadata_schedule_524_e5414: f64 = (noise_metadata_schedule_524_e5413).sqrt();
        let noise_metadata_schedule_524_e5415: f64 = (noise_variable_376 + noise_metadata_schedule_524_e5414);
        let noise_metadata_schedule_524_e5416: f64 = (noise_variable_375 * noise_metadata_schedule_524_e5415);
        let noise_metadata_schedule_524_e5418: f64 = (noise_metadata_schedule_524_e5416 * 0.5);
        let noise_metadata_schedule_524_e5419: f64 = (noise_variable_27 - noise_metadata_schedule_524_e5418);
        (noise_metadata_schedule_524_e5419,)
    } else {
        (noise_variable_377,)
    }
};
            noise_variable_377 = noise_metadata_schedule_524_e5421;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_525_e5437,) = {
    if (noise_variable_374 != 0.0) {
        let noise_metadata_schedule_525_e5429: f64 = (noise_variable_377 / noise_variable_27);
        let noise_metadata_schedule_525_e5430: f64 = (1.0 - noise_metadata_schedule_525_e5429);
        let noise_metadata_schedule_525_e5431: f64 = (noise_metadata_schedule_525_e5430).ln();
        let noise_metadata_schedule_525_e5432: f64 = (params.p41 * noise_metadata_schedule_525_e5431);
        let noise_metadata_schedule_525_e5433: f64 = (noise_metadata_schedule_525_e5432).exp();
        let noise_metadata_schedule_525_e5434: f64 = (1.0 - noise_metadata_schedule_525_e5433);
        let noise_metadata_schedule_525_e5435: f64 = (noise_variable_18 * noise_metadata_schedule_525_e5434);
        (noise_metadata_schedule_525_e5435,)
    } else {
        (noise_variable_378,)
    }
};
            noise_variable_378 = noise_metadata_schedule_525_e5437;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_526_e5439: f64 = (noise_variable_378).abs();
            let noise_metadata_schedule_526_e5441: f64 = if noise_metadata_schedule_526_e5439 > 0.001 { 1.0 } else { 0.0 };
            noise_variable_379 = noise_metadata_schedule_526_e5441;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_527_e5454,) = {
    if ((noise_variable_374 != 0.0) && (noise_variable_379 != 0.0)) {
        let noise_metadata_schedule_527_e5447: f64 = (noise_variable_378).exp();
        let noise_metadata_schedule_527_e5449: f64 = (noise_metadata_schedule_527_e5447 - 1.0);
        let noise_metadata_schedule_527_e5450: f64 = (noise_variable_17 * noise_metadata_schedule_527_e5449);
        let noise_metadata_schedule_527_e5452: f64 = (noise_metadata_schedule_527_e5450 / noise_variable_378);
        (noise_metadata_schedule_527_e5452,)
    } else {
        (noise_variable_346,)
    }
};
            noise_variable_346 = noise_metadata_schedule_527_e5454;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_528_e5467,) = {
    if ((noise_variable_374 != 0.0) && (noise_variable_379 == 0.0)) {
        let noise_metadata_schedule_528_e5463: f64 = (noise_variable_378 * 0.5);
        let noise_metadata_schedule_528_e5464: f64 = (1.0 + noise_metadata_schedule_528_e5463);
        let noise_metadata_schedule_528_e5465: f64 = (noise_variable_17 * noise_metadata_schedule_528_e5464);
        (noise_metadata_schedule_528_e5465,)
    } else {
        (noise_variable_346,)
    }
};
            noise_variable_346 = noise_metadata_schedule_528_e5467;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_529_e5472,) = {
    if (noise_variable_374 == 0.0) {
        (noise_variable_17,)
    } else {
        (noise_variable_346,)
    }
};
            noise_variable_346 = noise_metadata_schedule_529_e5472;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_530_e5476: f64 = (noise_variable_346 * noise_variable_179);
            let noise_metadata_schedule_530_e5477: f64 = (noise_variable_16 + noise_metadata_schedule_530_e5476);
            let noise_metadata_schedule_530_e5480: f64 = (params.p12 * noise_variable_178);
            let noise_metadata_schedule_530_e5481: f64 = (noise_metadata_schedule_530_e5477 + noise_metadata_schedule_530_e5480);
            noise_variable_352 = noise_metadata_schedule_530_e5481;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_531_e5484: f64 = (0.05 * noise_variable_16);
            noise_variable_353 = noise_metadata_schedule_531_e5484;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_532_e5487: f64 = (noise_variable_352 / noise_variable_353);
            let noise_metadata_schedule_532_e5489: f64 = (noise_metadata_schedule_532_e5487 - 1.0);
            noise_variable_347 = noise_metadata_schedule_532_e5489;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_533_e5495: f64 = (noise_variable_347 * noise_variable_347);
            let noise_metadata_schedule_533_e5497: f64 = (noise_metadata_schedule_533_e5495 + 1.921812);
            let noise_metadata_schedule_533_e5498: f64 = (noise_metadata_schedule_533_e5497).sqrt();
            let noise_metadata_schedule_533_e5499: f64 = (noise_variable_347 + noise_metadata_schedule_533_e5498);
            let noise_metadata_schedule_533_e5501: f64 = (noise_metadata_schedule_533_e5499 * 0.5);
            let noise_metadata_schedule_533_e5502: f64 = (1.0 + noise_metadata_schedule_533_e5501);
            let noise_metadata_schedule_533_e5503: f64 = (noise_variable_353 * noise_metadata_schedule_533_e5502);
            noise_variable_352 = noise_metadata_schedule_533_e5503;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_534_e5507: f64 = (2.4_f64).ln();
            let noise_metadata_schedule_534_e5508: f64 = (-noise_metadata_schedule_534_e5507);
            let noise_metadata_schedule_534_e5510: f64 = (noise_metadata_schedule_534_e5508 / params.p49);
            let noise_metadata_schedule_534_e5511: f64 = (noise_metadata_schedule_534_e5510).exp();
            let noise_metadata_schedule_534_e5512: f64 = (1.0 - noise_metadata_schedule_534_e5511);
            let noise_metadata_schedule_534_e5513: f64 = (noise_variable_34 * noise_metadata_schedule_534_e5512);
            noise_variable_380 = noise_metadata_schedule_534_e5513;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_535_e5516: f64 = (noise_variable_380 - noise_variable_203);
            let noise_metadata_schedule_535_e5518: f64 = (noise_metadata_schedule_535_e5516 * noise_variable_5);
            noise_variable_381 = noise_metadata_schedule_535_e5518;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_536_e5521: f64 = (noise_variable_381 * noise_variable_381);
            let noise_metadata_schedule_536_e5523: f64 = (noise_metadata_schedule_536_e5521 + 1.921812);
            let noise_metadata_schedule_536_e5524: f64 = (noise_metadata_schedule_536_e5523).sqrt();
            noise_variable_382 = noise_metadata_schedule_536_e5524;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_537_e5527: f64 = (noise_variable_381 + noise_variable_382);
            let noise_metadata_schedule_537_e5529: f64 = (noise_metadata_schedule_537_e5527 * 0.5);
            noise_variable_383 = noise_metadata_schedule_537_e5529;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_538_e5533: f64 = (noise_variable_4 * noise_variable_383);
            let noise_metadata_schedule_538_e5534: f64 = (noise_variable_380 - noise_metadata_schedule_538_e5533);
            noise_variable_384 = noise_metadata_schedule_538_e5534;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_539_e5537: f64 = (noise_variable_383 / noise_variable_382);
            noise_variable_385 = noise_metadata_schedule_539_e5537;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_540_e5539: f64 = (-params.p49);
            let noise_metadata_schedule_540_e5543: f64 = (noise_variable_384 / noise_variable_34);
            let noise_metadata_schedule_540_e5544: f64 = (1.0 - noise_metadata_schedule_540_e5543);
            let noise_metadata_schedule_540_e5545: f64 = (noise_metadata_schedule_540_e5544).ln();
            let noise_metadata_schedule_540_e5546: f64 = (noise_metadata_schedule_540_e5539 * noise_metadata_schedule_540_e5545);
            let noise_metadata_schedule_540_e5547: f64 = (noise_metadata_schedule_540_e5546).exp();
            let noise_metadata_schedule_540_e5549: f64 = (noise_metadata_schedule_540_e5547 * noise_variable_385);
            let noise_metadata_schedule_540_e5553: f64 = (1.0 - noise_variable_385);
            let noise_metadata_schedule_540_e5554: f64 = (2.4 * noise_metadata_schedule_540_e5553);
            let noise_metadata_schedule_540_e5555: f64 = (noise_metadata_schedule_540_e5549 + noise_metadata_schedule_540_e5554);
            noise_variable_361 = noise_metadata_schedule_540_e5555;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_541_e5560: f64 = (1.0 / noise_variable_361);
            let noise_metadata_schedule_541_e5562: f64 = (noise_metadata_schedule_541_e5560 - 1.0);
            let noise_metadata_schedule_541_e5563: f64 = (params.p67 * noise_metadata_schedule_541_e5562);
            let noise_metadata_schedule_541_e5564: f64 = (noise_variable_59 + noise_metadata_schedule_541_e5563);
            let noise_metadata_schedule_541_e5568: f64 = (noise_variable_361 - 1.0);
            let noise_metadata_schedule_541_e5569: f64 = (params.p68 * noise_metadata_schedule_541_e5568);
            let noise_metadata_schedule_541_e5570: f64 = (noise_metadata_schedule_541_e5564 + noise_metadata_schedule_541_e5569);
            noise_variable_357 = noise_metadata_schedule_541_e5570;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_542_e5573: f64 = if params.p79 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_386 = noise_metadata_schedule_542_e5573;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_543_e5579,) = {
    if (noise_variable_386 != 0.0) {
        let noise_metadata_schedule_543_e5577: f64 = (noise_variable_58 - noise_variable_203);
        (noise_metadata_schedule_543_e5577,)
    } else {
        (noise_variable_363,)
    }
};
            noise_variable_363 = noise_metadata_schedule_543_e5579;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_544_e5586,) = {
    if (noise_variable_386 == 0.0) {
        let noise_metadata_schedule_544_e5584: f64 = (noise_variable_204 - noise_variable_57);
        (noise_metadata_schedule_544_e5584,)
    } else {
        (noise_variable_363,)
    }
};
            noise_variable_363 = noise_metadata_schedule_544_e5586;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_545_e5589: f64 = if params.p0 <= 300.0 { 1.0 } else { 0.0 };
            noise_variable_394 = noise_metadata_schedule_545_e5589;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_546_e5597,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_546_e5593: f64 = (noise_variable_363 - noise_variable_4);
        let noise_metadata_schedule_546_e5595: f64 = (noise_metadata_schedule_546_e5593 * noise_variable_5);
        (noise_metadata_schedule_546_e5595,)
    } else {
        (noise_variable_387,)
    }
};
            noise_variable_387 = noise_metadata_schedule_546_e5597;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_547_e5614,) = {
    if (noise_variable_394 != 0.0) {
        let noise_metadata_schedule_547_e5604: f64 = (noise_variable_387 * noise_variable_387);
        let noise_metadata_schedule_547_e5606: f64 = (noise_metadata_schedule_547_e5604 + 1.921812);
        let noise_metadata_schedule_547_e5607: f64 = (noise_metadata_schedule_547_e5606).sqrt();
        let noise_metadata_schedule_547_e5608: f64 = (noise_variable_387 + noise_metadata_schedule_547_e5607);
        let noise_metadata_schedule_547_e5610: f64 = (noise_metadata_schedule_547_e5608 * 0.5);
        let noise_metadata_schedule_547_e5611: f64 = (noise_variable_4 * noise_metadata_schedule_547_e5610);
        let noise_metadata_schedule_547_e5612: f64 = (noise_variable_4 + noise_metadata_schedule_547_e5611);
        (noise_metadata_schedule_547_e5612,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_547_e5614;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_548_e5621,) = {
    if (noise_variable_394 == 0.0) {
        let noise_metadata_schedule_548_e5619: f64 = (noise_variable_363 / noise_variable_3);
        (noise_metadata_schedule_548_e5619,)
    } else {
        (noise_variable_387,)
    }
};
            noise_variable_387 = noise_metadata_schedule_548_e5621;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_549_e5637,) = {
    if (noise_variable_394 == 0.0) {
        let noise_metadata_schedule_549_e5628: f64 = (noise_variable_387 * noise_variable_387);
        let noise_metadata_schedule_549_e5630: f64 = (noise_metadata_schedule_549_e5628 + params.p80);
        let noise_metadata_schedule_549_e5631: f64 = (noise_metadata_schedule_549_e5630).sqrt();
        let noise_metadata_schedule_549_e5632: f64 = (noise_variable_387 + noise_metadata_schedule_549_e5631);
        let noise_metadata_schedule_549_e5634: f64 = (noise_metadata_schedule_549_e5632 * 0.5);
        let noise_metadata_schedule_549_e5635: f64 = (noise_variable_3 * noise_metadata_schedule_549_e5634);
        (noise_metadata_schedule_549_e5635,)
    } else {
        (noise_variable_388,)
    }
};
            noise_variable_388 = noise_metadata_schedule_549_e5637;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_550_e5640: f64 = (noise_variable_388 / noise_variable_55);
            noise_variable_389 = noise_metadata_schedule_550_e5640;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_551_e5643: f64 = (noise_variable_388 * noise_variable_54);
            noise_variable_390 = noise_metadata_schedule_551_e5643;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_552_e5647: f64 = (noise_variable_389).ln();
            let noise_metadata_schedule_552_e5648: f64 = (params.p77 * noise_metadata_schedule_552_e5647);
            let noise_metadata_schedule_552_e5649: f64 = (noise_metadata_schedule_552_e5648).exp();
            let noise_metadata_schedule_552_e5650: f64 = (1.0 + noise_metadata_schedule_552_e5649);
            let noise_metadata_schedule_552_e5651: f64 = (noise_metadata_schedule_552_e5650).ln();
            let noise_metadata_schedule_552_e5653: f64 = (noise_metadata_schedule_552_e5651 / params.p77);
            let noise_metadata_schedule_552_e5654: f64 = (noise_metadata_schedule_552_e5653).exp();
            noise_variable_391 = noise_metadata_schedule_552_e5654;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_553_e5657: f64 = (noise_variable_390 / noise_variable_391);
            noise_variable_392 = noise_metadata_schedule_553_e5657;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_554_e5660: f64 = (noise_variable_388 - noise_variable_55);
            let noise_metadata_schedule_554_e5662: f64 = (noise_metadata_schedule_554_e5660 / params.p76);
            noise_variable_393 = noise_metadata_schedule_554_e5662;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_555_e5669: f64 = (noise_variable_393 * noise_variable_393);
            let noise_metadata_schedule_555_e5671: f64 = (noise_metadata_schedule_555_e5669 + params.p81);
            let noise_metadata_schedule_555_e5672: f64 = (noise_metadata_schedule_555_e5671).sqrt();
            let noise_metadata_schedule_555_e5673: f64 = (noise_variable_393 + noise_metadata_schedule_555_e5672);
            let noise_metadata_schedule_555_e5674: f64 = (0.5 * noise_metadata_schedule_555_e5673);
            let noise_metadata_schedule_555_e5675: f64 = (1.0 + noise_metadata_schedule_555_e5674);
            let noise_metadata_schedule_555_e5676: f64 = (noise_variable_392 * noise_metadata_schedule_555_e5675);
            noise_variable_362 = noise_metadata_schedule_555_e5676;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            noise_variable_348 = noise_variable_352;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_557_e5684: f64 = if ((noise_variable_357 > 0.0) || (params.p85 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_395 = noise_metadata_schedule_557_e5684;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_558_e5690,) = {
    if (noise_variable_395 != 0.0) {
        let noise_metadata_schedule_558_e5688: f64 = (0.5 * noise_variable_352);
        (noise_metadata_schedule_558_e5688,)
    } else {
        (noise_variable_396,)
    }
};
            noise_variable_396 = noise_metadata_schedule_558_e5690;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_559_e5693: f64 = if params.p0 <= 300.0 { 1.0 } else { 0.0 };
            noise_variable_397 = noise_metadata_schedule_559_e5693;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_560_e5712,) = {
    if ((noise_variable_395 != 0.0) && (noise_variable_397 != 0.0)) {
        let noise_metadata_schedule_560_e5700: f64 = (noise_variable_396 * noise_variable_396);
        let noise_metadata_schedule_560_e5703: f64 = (noise_variable_357 * noise_variable_350);
        let noise_metadata_schedule_560_e5704: f64 = (noise_metadata_schedule_560_e5700 + noise_metadata_schedule_560_e5703);
        let noise_metadata_schedule_560_e5707: f64 = (params.p85 * noise_variable_351);
        let noise_metadata_schedule_560_e5708: f64 = (noise_metadata_schedule_560_e5704 + noise_metadata_schedule_560_e5707);
        let noise_metadata_schedule_560_e5709: f64 = (noise_metadata_schedule_560_e5708).sqrt();
        let noise_metadata_schedule_560_e5710: f64 = (noise_variable_396 + noise_metadata_schedule_560_e5709);
        (noise_metadata_schedule_560_e5710,)
    } else {
        (noise_variable_348,)
    }
};
            noise_variable_348 = noise_metadata_schedule_560_e5712;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_561_e5734,) = {
    if ((noise_variable_395 != 0.0) && (noise_variable_397 == 0.0)) {
        let noise_metadata_schedule_561_e5720: f64 = (noise_variable_396 * noise_variable_396);
        let noise_metadata_schedule_561_e5723: f64 = (noise_variable_19 * noise_variable_59);
        let noise_metadata_schedule_561_e5725: f64 = (noise_metadata_schedule_561_e5723 * noise_variable_350);
        let noise_metadata_schedule_561_e5726: f64 = (noise_metadata_schedule_561_e5720 + noise_metadata_schedule_561_e5725);
        let noise_metadata_schedule_561_e5729: f64 = (params.p85 * noise_variable_351);
        let noise_metadata_schedule_561_e5730: f64 = (noise_metadata_schedule_561_e5726 + noise_metadata_schedule_561_e5729);
        let noise_metadata_schedule_561_e5731: f64 = (noise_metadata_schedule_561_e5730).sqrt();
        let noise_metadata_schedule_561_e5732: f64 = (noise_variable_396 + noise_metadata_schedule_561_e5731);
        (noise_metadata_schedule_561_e5732,)
    } else {
        (noise_variable_348,)
    }
};
            noise_variable_348 = noise_metadata_schedule_561_e5734;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_562_e5737: f64 = (noise_variable_350 / noise_variable_348);
            noise_variable_217 = noise_metadata_schedule_562_e5737;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_563_e5740: f64 = (noise_variable_351 / noise_variable_348);
            noise_variable_218 = noise_metadata_schedule_563_e5740;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            noise_variable_219 = noise_variable_357;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_565_e5744: f64 = (noise_variable_357 * noise_variable_217);
            noise_variable_355 = noise_metadata_schedule_565_e5744;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_566_e5747: f64 = if params.p0 >= 310.0 { 1.0 } else { 0.0 };
            noise_variable_398 = noise_metadata_schedule_566_e5747;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_567_e5753,) = {
    if (noise_variable_398 != 0.0) {
        let noise_metadata_schedule_567_e5751: f64 = (noise_variable_19 * noise_variable_59);
        (noise_metadata_schedule_567_e5751,)
    } else {
        (noise_variable_359,)
    }
};
            noise_variable_359 = noise_metadata_schedule_567_e5753;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_568_e5759,) = {
    if (noise_variable_398 != 0.0) {
        let noise_metadata_schedule_568_e5757: f64 = (noise_variable_359 * noise_variable_217);
        (noise_metadata_schedule_568_e5757,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_568_e5759;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_569_e5766,) = {
    if (noise_variable_398 == 0.0) {
        let noise_metadata_schedule_569_e5764: f64 = (noise_variable_19 * noise_variable_355);
        (noise_metadata_schedule_569_e5764,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_569_e5766;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_570_e5773,) = {
    if (noise_variable_398 == 0.0) {
        let noise_metadata_schedule_570_e5771: f64 = (noise_variable_19 * noise_variable_219);
        (noise_metadata_schedule_570_e5771,)
    } else {
        (noise_variable_359,)
    }
};
            noise_variable_359 = noise_metadata_schedule_570_e5773;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            noise_variable_354 = 0.0;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_572_e5778: f64 = (1e-6 * noise_variable_362);
            let noise_metadata_schedule_572_e5783: f64 = if ((noise_variable_217 >= noise_metadata_schedule_572_e5778) || (params.p0 >= 320.0)) { 1.0 } else { 0.0 };
            noise_variable_399 = noise_metadata_schedule_572_e5783;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_573_e5789,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_573_e5787: f64 = (noise_variable_217 / noise_variable_362);
        (noise_metadata_schedule_573_e5787,)
    } else {
        (noise_variable_96,)
    }
};
            noise_variable_96 = noise_metadata_schedule_573_e5789;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_574_e5799,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_574_e5794: f64 = (noise_variable_96).ln();
        let noise_metadata_schedule_574_e5795: f64 = (params.p70 * noise_metadata_schedule_574_e5794);
        let noise_metadata_schedule_574_e5796: f64 = (noise_metadata_schedule_574_e5795).exp();
        let noise_metadata_schedule_574_e5797: f64 = (noise_variable_61 * noise_metadata_schedule_574_e5796);
        (noise_metadata_schedule_574_e5797,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_574_e5799;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_575_e5809,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_575_e5803: f64 = (noise_variable_98 * noise_variable_217);
        let noise_metadata_schedule_575_e5806: f64 = (1.0 + params.p70);
        let noise_metadata_schedule_575_e5807: f64 = (noise_metadata_schedule_575_e5803 / noise_metadata_schedule_575_e5806);
        (noise_metadata_schedule_575_e5807,)
    } else {
        (noise_variable_97,)
    }
};
            noise_variable_97 = noise_metadata_schedule_575_e5809;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_576_e5814: f64 = (params.p75 / params.p74);
            let noise_metadata_schedule_576_e5815: f64 = (0.05 * noise_metadata_schedule_576_e5814);
            let noise_metadata_schedule_576_e5816: f64 = if params.p83 < noise_metadata_schedule_576_e5815 { 1.0 } else { 0.0 };
            noise_variable_400 = noise_metadata_schedule_576_e5816;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_577_e5822,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_400 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_577_e5822;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_578_e5828,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_400 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_578_e5828;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_579_e5839,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_400 == 0.0)) {
        let noise_metadata_schedule_579_e5835: f64 = (noise_variable_217 - noise_variable_362);
        let noise_metadata_schedule_579_e5837: f64 = (noise_metadata_schedule_579_e5835 / params.p83);
        (noise_metadata_schedule_579_e5837,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_579_e5839;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_580_e5842: f64 = (-10000000000.0);
            let noise_metadata_schedule_580_e5843: f64 = if noise_variable_107 < noise_metadata_schedule_580_e5842 { 1.0 } else { 0.0 };
            noise_variable_401 = noise_metadata_schedule_580_e5843;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_581_e5853,) = {
    if (((noise_variable_399 != 0.0) && (noise_variable_400 == 0.0)) && (noise_variable_401 != 0.0)) {
        let noise_metadata_schedule_581_e5851: f64 = (-10000000000.0);
        (noise_metadata_schedule_581_e5851,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_581_e5853;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_582_e5865,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_400 == 0.0)) {
        let noise_metadata_schedule_582_e5860: f64 = (noise_variable_107 * noise_variable_107);
        let noise_metadata_schedule_582_e5862: f64 = (noise_metadata_schedule_582_e5860 + params.p84);
        let noise_metadata_schedule_582_e5863: f64 = (noise_metadata_schedule_582_e5862).sqrt();
        (noise_metadata_schedule_582_e5863,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_582_e5865;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_583_e5880,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_400 == 0.0)) {
        let noise_metadata_schedule_583_e5872: f64 = (-2.0);
        let noise_metadata_schedule_583_e5875: f64 = (noise_variable_107 + noise_variable_95);
        let noise_metadata_schedule_583_e5876: f64 = (noise_metadata_schedule_583_e5872 / noise_metadata_schedule_583_e5875);
        let noise_metadata_schedule_583_e5877: f64 = (noise_metadata_schedule_583_e5876).exp();
        let noise_metadata_schedule_583_e5878: f64 = (params.p82 * noise_metadata_schedule_583_e5877);
        (noise_metadata_schedule_583_e5878,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_583_e5880;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_584_e5897,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_400 == 0.0)) {
        let noise_metadata_schedule_584_e5887: f64 = (2.0 * noise_variable_111);
        let noise_metadata_schedule_584_e5890: f64 = (params.p83 * noise_variable_95);
        let noise_metadata_schedule_584_e5893: f64 = (noise_variable_107 + noise_variable_95);
        let noise_metadata_schedule_584_e5894: f64 = (noise_metadata_schedule_584_e5890 * noise_metadata_schedule_584_e5893);
        let noise_metadata_schedule_584_e5895: f64 = (noise_metadata_schedule_584_e5887 / noise_metadata_schedule_584_e5894);
        (noise_metadata_schedule_584_e5895,)
    } else {
        (noise_variable_112,)
    }
};
            noise_variable_112 = noise_metadata_schedule_584_e5897;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_585_e5912,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_585_e5901: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_585_e5903: f64 = (noise_metadata_schedule_585_e5901 * noise_variable_60);
        let noise_metadata_schedule_585_e5906: f64 = (noise_variable_111 * noise_variable_5);
        let noise_metadata_schedule_585_e5907: f64 = (noise_metadata_schedule_585_e5906).exp();
        let noise_metadata_schedule_585_e5909: f64 = (noise_metadata_schedule_585_e5907 - 1.0);
        let noise_metadata_schedule_585_e5910: f64 = (noise_metadata_schedule_585_e5903 * noise_metadata_schedule_585_e5909);
        (noise_metadata_schedule_585_e5910,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_585_e5912;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_586_e5933,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_586_e5917: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_586_e5919: f64 = (noise_metadata_schedule_586_e5917 * noise_variable_60);
        let noise_metadata_schedule_586_e5921: f64 = (noise_metadata_schedule_586_e5919 * noise_variable_217);
        let noise_metadata_schedule_586_e5924: f64 = (noise_variable_111 * noise_variable_5);
        let noise_metadata_schedule_586_e5925: f64 = (noise_metadata_schedule_586_e5924).exp();
        let noise_metadata_schedule_586_e5926: f64 = (noise_metadata_schedule_586_e5921 * noise_metadata_schedule_586_e5925);
        let noise_metadata_schedule_586_e5928: f64 = (noise_metadata_schedule_586_e5926 * noise_variable_5);
        let noise_metadata_schedule_586_e5930: f64 = (noise_metadata_schedule_586_e5928 * noise_variable_112);
        let noise_metadata_schedule_586_e5931: f64 = (noise_variable_99 + noise_metadata_schedule_586_e5930);
        (noise_metadata_schedule_586_e5931,)
    } else {
        (noise_variable_100,)
    }
};
            noise_variable_100 = noise_metadata_schedule_586_e5933;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_587_e5941,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_587_e5938: f64 = (1.0 / noise_variable_96);
        let noise_metadata_schedule_587_e5939: f64 = (1.0 - noise_metadata_schedule_587_e5938);
        (noise_metadata_schedule_587_e5939,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_587_e5941;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_588_e5959,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_588_e5946: f64 = (noise_variable_108 * noise_variable_108);
        let noise_metadata_schedule_588_e5948: f64 = (noise_metadata_schedule_588_e5946 + params.p72);
        let noise_metadata_schedule_588_e5949: f64 = (noise_metadata_schedule_588_e5948).sqrt();
        let noise_metadata_schedule_588_e5950: f64 = (noise_variable_108 + noise_metadata_schedule_588_e5949);
        let noise_metadata_schedule_588_e5954: f64 = (1.0 + params.p72);
        let noise_metadata_schedule_588_e5955: f64 = (noise_metadata_schedule_588_e5954).sqrt();
        let noise_metadata_schedule_588_e5956: f64 = (1.0 + noise_metadata_schedule_588_e5955);
        let noise_metadata_schedule_588_e5957: f64 = (noise_metadata_schedule_588_e5950 / noise_metadata_schedule_588_e5956);
        (noise_metadata_schedule_588_e5957,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_588_e5959;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_589_e5968,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_589_e5963: f64 = (noise_variable_111 - params.p82);
        let noise_metadata_schedule_589_e5965: f64 = (noise_metadata_schedule_589_e5963 * noise_variable_5);
        let noise_metadata_schedule_589_e5966: f64 = (noise_metadata_schedule_589_e5965).exp();
        (noise_metadata_schedule_589_e5966,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_589_e5968;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_590_e5978,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_590_e5972: f64 = (noise_variable_60 * noise_variable_109);
        let noise_metadata_schedule_590_e5974: f64 = (noise_metadata_schedule_590_e5972 * noise_variable_109);
        let noise_metadata_schedule_590_e5976: f64 = (noise_metadata_schedule_590_e5974 * noise_variable_110);
        (noise_metadata_schedule_590_e5976,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_590_e5978;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_591_e6001,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_591_e5986: f64 = (noise_variable_108 * noise_variable_108);
        let noise_metadata_schedule_591_e5988: f64 = (noise_metadata_schedule_591_e5986 + params.p72);
        let noise_metadata_schedule_591_e5989: f64 = (noise_metadata_schedule_591_e5988).sqrt();
        let noise_metadata_schedule_591_e5990: f64 = (noise_variable_96 * noise_metadata_schedule_591_e5989);
        let noise_metadata_schedule_591_e5991: f64 = (2.0 / noise_metadata_schedule_591_e5990);
        let noise_metadata_schedule_591_e5992: f64 = (1.0 + noise_metadata_schedule_591_e5991);
        let noise_metadata_schedule_591_e5995: f64 = (noise_variable_5 * noise_variable_217);
        let noise_metadata_schedule_591_e5997: f64 = (noise_metadata_schedule_591_e5995 * noise_variable_112);
        let noise_metadata_schedule_591_e5998: f64 = (noise_metadata_schedule_591_e5992 + noise_metadata_schedule_591_e5997);
        let noise_metadata_schedule_591_e5999: f64 = (noise_variable_101 * noise_metadata_schedule_591_e5998);
        (noise_metadata_schedule_591_e5999,)
    } else {
        (noise_variable_102,)
    }
};
            noise_variable_102 = noise_metadata_schedule_591_e6001;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_592_e6011: f64 = (noise_variable_109 * params.p115);
            let noise_metadata_schedule_592_e6017: f64 = (noise_variable_109 * params.p116);
            let noise_metadata_schedule_592_e6020: f64 = if ((((params.p115 < 0.01) && (params.p116 < 0.01)) && (noise_metadata_schedule_592_e6011 < 0.005)) && (noise_metadata_schedule_592_e6017 < 0.005)) { 1.0 } else { 0.0 };
            noise_variable_402 = noise_metadata_schedule_592_e6020;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_593_e6030,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 != 0.0)) {
        let noise_metadata_schedule_593_e6026: f64 = (params.p73 * noise_variable_101);
        let noise_metadata_schedule_593_e6028: f64 = (noise_metadata_schedule_593_e6026 * noise_variable_217);
        (noise_metadata_schedule_593_e6028,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_593_e6030;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_594_e6038,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 != 0.0)) {
        let noise_metadata_schedule_594_e6036: f64 = (params.p73 * noise_variable_102);
        (noise_metadata_schedule_594_e6036,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_594_e6038;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_595_e6047,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_595_e6045: f64 = (1.0 - noise_variable_109);
        (noise_metadata_schedule_595_e6045,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_595_e6047;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_596_e6069,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_596_e6054: f64 = (noise_variable_146 - 1.0);
        let noise_metadata_schedule_596_e6057: f64 = (1.0 - noise_variable_108);
        let noise_metadata_schedule_596_e6058: f64 = (noise_metadata_schedule_596_e6054 * noise_metadata_schedule_596_e6057);
        let noise_metadata_schedule_596_e6061: f64 = (noise_variable_108 * noise_variable_108);
        let noise_metadata_schedule_596_e6063: f64 = (noise_metadata_schedule_596_e6061 + params.p72);
        let noise_metadata_schedule_596_e6064: f64 = (noise_metadata_schedule_596_e6063).sqrt();
        let noise_metadata_schedule_596_e6066: f64 = (noise_metadata_schedule_596_e6064 * noise_variable_217);
        let noise_metadata_schedule_596_e6067: f64 = (noise_metadata_schedule_596_e6058 / noise_metadata_schedule_596_e6066);
        (noise_metadata_schedule_596_e6067,)
    } else {
        (noise_variable_147,)
    }
};
            noise_variable_147 = noise_metadata_schedule_596_e6069;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_597_e6071: f64 = (noise_variable_232).abs();
            let noise_metadata_schedule_597_e6073: f64 = if noise_metadata_schedule_597_e6071 > 0.001 { 1.0 } else { 0.0 };
            noise_variable_403 = noise_metadata_schedule_597_e6073;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_598_e6087,) = {
    if (((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) {
        let noise_metadata_schedule_598_e6082: f64 = (noise_variable_146 - 1.0);
        let noise_metadata_schedule_598_e6084: f64 = (noise_metadata_schedule_598_e6082 * noise_variable_231);
        let noise_metadata_schedule_598_e6085: f64 = (noise_metadata_schedule_598_e6084).exp();
        (noise_metadata_schedule_598_e6085,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_598_e6087;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_599_e6090: f64 = if noise_variable_229 < 0.01 { 1.0 } else { 0.0 };
            noise_variable_404 = noise_metadata_schedule_599_e6090;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_600_e6107,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 != 0.0)) {
        let noise_metadata_schedule_600_e6101: f64 = (1.0 - noise_variable_151);
        let noise_metadata_schedule_600_e6104: f64 = (noise_variable_151 * noise_variable_230);
        let noise_metadata_schedule_600_e6105: f64 = (noise_metadata_schedule_600_e6101 / noise_metadata_schedule_600_e6104);
        (noise_metadata_schedule_600_e6105,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_600_e6107;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_601_e6122,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 != 0.0)) {
        let noise_metadata_schedule_601_e6119: f64 = (noise_variable_230 * noise_variable_149);
        let noise_metadata_schedule_601_e6120: f64 = (1.0 + noise_metadata_schedule_601_e6119);
        (noise_metadata_schedule_601_e6120,)
    } else {
        (noise_variable_148,)
    }
};
            noise_variable_148 = noise_metadata_schedule_601_e6122;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_602_e6154,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 != 0.0)) {
        let noise_metadata_schedule_602_e6134: f64 = (noise_variable_230 * noise_variable_149);
        let noise_metadata_schedule_602_e6138: f64 = (0.25 * noise_variable_230);
        let noise_metadata_schedule_602_e6140: f64 = (noise_metadata_schedule_602_e6138 * noise_variable_149);
        let noise_metadata_schedule_602_e6141: f64 = (0.5 + noise_metadata_schedule_602_e6140);
        let noise_metadata_schedule_602_e6142: f64 = (noise_metadata_schedule_602_e6134 * noise_metadata_schedule_602_e6141);
        let noise_metadata_schedule_602_e6145: f64 = (noise_variable_148).ln();
        let noise_metadata_schedule_602_e6146: f64 = (0.5 * noise_metadata_schedule_602_e6145);
        let noise_metadata_schedule_602_e6147: f64 = (noise_metadata_schedule_602_e6142 - noise_metadata_schedule_602_e6146);
        let noise_metadata_schedule_602_e6148: f64 = (2.0 * noise_metadata_schedule_602_e6147);
        let noise_metadata_schedule_602_e6150: f64 = (noise_metadata_schedule_602_e6148 / noise_variable_230);
        let noise_metadata_schedule_602_e6152: f64 = (noise_metadata_schedule_602_e6150 / noise_variable_230);
        (noise_metadata_schedule_602_e6152,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_602_e6154;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_603_e6172,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 != 0.0)) {
        let noise_metadata_schedule_603_e6164: f64 = (-noise_variable_231);
        let noise_metadata_schedule_603_e6166: f64 = (noise_metadata_schedule_603_e6164 * noise_variable_147);
        let noise_metadata_schedule_603_e6169: f64 = (noise_variable_151 * noise_variable_230);
        let noise_metadata_schedule_603_e6170: f64 = (noise_metadata_schedule_603_e6166 / noise_metadata_schedule_603_e6169);
        (noise_metadata_schedule_603_e6170,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_603_e6172;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_604_e6191,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 != 0.0)) {
        let noise_metadata_schedule_604_e6183: f64 = (1.0 + noise_variable_148);
        let noise_metadata_schedule_604_e6185: f64 = (noise_metadata_schedule_604_e6183 * noise_variable_149);
        let noise_metadata_schedule_604_e6187: f64 = (noise_metadata_schedule_604_e6185 * noise_variable_150);
        let noise_metadata_schedule_604_e6189: f64 = (noise_metadata_schedule_604_e6187 / noise_variable_148);
        (noise_metadata_schedule_604_e6189,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_604_e6191;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_605_e6207,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_605_e6204: f64 = (noise_variable_151 * params.p115);
        let noise_metadata_schedule_605_e6205: f64 = (params.p116 - noise_metadata_schedule_605_e6204);
        (noise_metadata_schedule_605_e6205,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_605_e6207;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_606_e6223,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_606_e6219: f64 = (noise_variable_151 - 1.0);
        let noise_metadata_schedule_606_e6221: f64 = (noise_metadata_schedule_606_e6219 / noise_variable_152);
        (noise_metadata_schedule_606_e6221,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_606_e6223;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_607_e6239,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_607_e6236: f64 = (params.p116 * noise_variable_149);
        let noise_metadata_schedule_607_e6237: f64 = (1.0 + noise_metadata_schedule_607_e6236);
        (noise_metadata_schedule_607_e6237,)
    } else {
        (noise_variable_160,)
    }
};
            noise_variable_160 = noise_metadata_schedule_607_e6239;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_608_e6252,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_608_e6250: f64 = (noise_variable_160).ln();
        (noise_metadata_schedule_608_e6250,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_608_e6252;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_609_e6266,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_609_e6264: f64 = (noise_variable_227 * noise_variable_226);
        (noise_metadata_schedule_609_e6264,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_609_e6266;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_610_e6292,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_610_e6279: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_610_e6280: f64 = (noise_variable_161 * noise_metadata_schedule_610_e6279);
        let noise_metadata_schedule_610_e6282: f64 = (noise_metadata_schedule_610_e6280 * noise_variable_226);
        let noise_metadata_schedule_610_e6286: f64 = (noise_variable_227 * noise_variable_149);
        let noise_metadata_schedule_610_e6287: f64 = (noise_variable_162 + noise_metadata_schedule_610_e6286);
        let noise_metadata_schedule_610_e6289: f64 = (noise_metadata_schedule_610_e6287 * noise_variable_149);
        let noise_metadata_schedule_610_e6290: f64 = (noise_metadata_schedule_610_e6282 + noise_metadata_schedule_610_e6289);
        (noise_metadata_schedule_610_e6290,)
    } else {
        (noise_variable_157,)
    }
};
            noise_variable_157 = noise_metadata_schedule_610_e6292;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_611_e6316,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_611_e6304: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_611_e6306: f64 = (noise_metadata_schedule_611_e6304 / noise_variable_160);
        let noise_metadata_schedule_611_e6308: f64 = (noise_metadata_schedule_611_e6306 + noise_variable_162);
        let noise_metadata_schedule_611_e6311: f64 = (noise_variable_149 * noise_variable_227);
        let noise_metadata_schedule_611_e6313: f64 = (noise_metadata_schedule_611_e6311 * 2.0);
        let noise_metadata_schedule_611_e6314: f64 = (noise_metadata_schedule_611_e6308 + noise_metadata_schedule_611_e6313);
        (noise_metadata_schedule_611_e6314,)
    } else {
        (noise_variable_159,)
    }
};
            noise_variable_159 = noise_metadata_schedule_611_e6316;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_612_e6332,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_612_e6329: f64 = (params.p115 * noise_variable_149);
        let noise_metadata_schedule_612_e6330: f64 = (1.0 + noise_metadata_schedule_612_e6329);
        (noise_metadata_schedule_612_e6330,)
    } else {
        (noise_variable_160,)
    }
};
            noise_variable_160 = noise_metadata_schedule_612_e6332;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_613_e6345,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_613_e6343: f64 = (noise_variable_160).ln();
        (noise_metadata_schedule_613_e6343,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_613_e6345;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_614_e6359,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_614_e6357: f64 = (noise_variable_228 * noise_variable_225);
        (noise_metadata_schedule_614_e6357,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_614_e6359;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_615_e6385,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_615_e6372: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_615_e6373: f64 = (noise_variable_161 * noise_metadata_schedule_615_e6372);
        let noise_metadata_schedule_615_e6375: f64 = (noise_metadata_schedule_615_e6373 * noise_variable_225);
        let noise_metadata_schedule_615_e6379: f64 = (noise_variable_228 * noise_variable_149);
        let noise_metadata_schedule_615_e6380: f64 = (noise_variable_162 + noise_metadata_schedule_615_e6379);
        let noise_metadata_schedule_615_e6382: f64 = (noise_metadata_schedule_615_e6380 * noise_variable_149);
        let noise_metadata_schedule_615_e6383: f64 = (noise_metadata_schedule_615_e6375 + noise_metadata_schedule_615_e6382);
        (noise_metadata_schedule_615_e6383,)
    } else {
        (noise_variable_156,)
    }
};
            noise_variable_156 = noise_metadata_schedule_615_e6385;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_616_e6409,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_616_e6397: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_616_e6399: f64 = (noise_metadata_schedule_616_e6397 / noise_variable_160);
        let noise_metadata_schedule_616_e6401: f64 = (noise_metadata_schedule_616_e6399 + noise_variable_162);
        let noise_metadata_schedule_616_e6404: f64 = (noise_variable_149 * noise_variable_228);
        let noise_metadata_schedule_616_e6406: f64 = (noise_metadata_schedule_616_e6404 * 2.0);
        let noise_metadata_schedule_616_e6407: f64 = (noise_metadata_schedule_616_e6401 + noise_metadata_schedule_616_e6406);
        (noise_metadata_schedule_616_e6407,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_616_e6409;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_617_e6425,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_617_e6421: f64 = (noise_variable_157 - noise_variable_156);
        let noise_metadata_schedule_617_e6423: f64 = (noise_metadata_schedule_617_e6421 / noise_variable_232);
        (noise_metadata_schedule_617_e6423,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_617_e6425;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_618_e6450,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_618_e6436: f64 = (-2.0);
        let noise_metadata_schedule_618_e6438: f64 = (noise_metadata_schedule_618_e6436 * noise_variable_232);
        let noise_metadata_schedule_618_e6441: f64 = (noise_variable_152 * noise_variable_152);
        let noise_metadata_schedule_618_e6442: f64 = (noise_metadata_schedule_618_e6438 / noise_metadata_schedule_618_e6441);
        let noise_metadata_schedule_618_e6444: f64 = (noise_metadata_schedule_618_e6442 * noise_variable_151);
        let noise_metadata_schedule_618_e6446: f64 = (noise_metadata_schedule_618_e6444 * noise_variable_231);
        let noise_metadata_schedule_618_e6448: f64 = (noise_metadata_schedule_618_e6446 * noise_variable_147);
        (noise_metadata_schedule_618_e6448,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_618_e6450;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_619_e6468,) = {
    if ((((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 != 0.0)) && (noise_variable_404 == 0.0)) {
        let noise_metadata_schedule_619_e6462: f64 = (noise_variable_159 - noise_variable_158);
        let noise_metadata_schedule_619_e6464: f64 = (noise_metadata_schedule_619_e6462 * noise_variable_150);
        let noise_metadata_schedule_619_e6466: f64 = (noise_metadata_schedule_619_e6464 / noise_variable_232);
        (noise_metadata_schedule_619_e6466,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_619_e6468;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_620_e6486,) = {
    if (((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 == 0.0)) {
        let noise_metadata_schedule_620_e6478: f64 = (1.0 - noise_variable_146);
        let noise_metadata_schedule_620_e6482: f64 = (noise_variable_146 * params.p115);
        let noise_metadata_schedule_620_e6483: f64 = (1.0 + noise_metadata_schedule_620_e6482);
        let noise_metadata_schedule_620_e6484: f64 = (noise_metadata_schedule_620_e6478 / noise_metadata_schedule_620_e6483);
        (noise_metadata_schedule_620_e6484,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_620_e6486;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_621_e6500,) = {
    if (((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 == 0.0)) {
        let noise_metadata_schedule_621_e6497: f64 = (params.p115 * noise_variable_149);
        let noise_metadata_schedule_621_e6498: f64 = (1.0 + noise_metadata_schedule_621_e6497);
        (noise_metadata_schedule_621_e6498,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_621_e6500;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_622_e6522,) = {
    if (((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 == 0.0)) {
        let noise_metadata_schedule_622_e6510: f64 = (noise_variable_149 * noise_variable_149);
        let noise_metadata_schedule_622_e6514: f64 = (noise_variable_227 * 2.0);
        let noise_metadata_schedule_622_e6516: f64 = (noise_metadata_schedule_622_e6514 * noise_variable_149);
        let noise_metadata_schedule_622_e6517: f64 = (1.0 + noise_metadata_schedule_622_e6516);
        let noise_metadata_schedule_622_e6518: f64 = (noise_metadata_schedule_622_e6510 * noise_metadata_schedule_622_e6517);
        let noise_metadata_schedule_622_e6520: f64 = (noise_metadata_schedule_622_e6518 / noise_variable_153);
        (noise_metadata_schedule_622_e6520,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_622_e6522;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_623_e6541,) = {
    if (((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 == 0.0)) {
        let noise_metadata_schedule_623_e6531: f64 = (-noise_variable_147);
        let noise_metadata_schedule_623_e6533: f64 = (noise_metadata_schedule_623_e6531 * noise_variable_153);
        let noise_metadata_schedule_623_e6537: f64 = (noise_variable_146 * params.p115);
        let noise_metadata_schedule_623_e6538: f64 = (1.0 + noise_metadata_schedule_623_e6537);
        let noise_metadata_schedule_623_e6539: f64 = (noise_metadata_schedule_623_e6533 / noise_metadata_schedule_623_e6538);
        (noise_metadata_schedule_623_e6539,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_623_e6541;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_624_e6561,) = {
    if (((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) && (noise_variable_403 == 0.0)) {
        let noise_metadata_schedule_624_e6554: f64 = (noise_variable_153 * noise_variable_153);
        let noise_metadata_schedule_624_e6555: f64 = (1.0 / noise_metadata_schedule_624_e6554);
        let noise_metadata_schedule_624_e6556: f64 = (1.0 + noise_metadata_schedule_624_e6555);
        let noise_metadata_schedule_624_e6557: f64 = (noise_variable_149 * noise_metadata_schedule_624_e6556);
        let noise_metadata_schedule_624_e6559: f64 = (noise_metadata_schedule_624_e6557 * noise_variable_150);
        (noise_metadata_schedule_624_e6559,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_624_e6561;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_625_e6572,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_625_e6568: f64 = (params.p73 * noise_variable_60);
        let noise_metadata_schedule_625_e6570: f64 = (noise_metadata_schedule_625_e6568 * noise_variable_110);
        (noise_metadata_schedule_625_e6570,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_625_e6572;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_626_e6581,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_626_e6579: f64 = (noise_variable_166 * noise_variable_154);
        (noise_metadata_schedule_626_e6579,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_626_e6581;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_627_e6590,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_627_e6588: f64 = (noise_variable_167 * noise_variable_217);
        (noise_metadata_schedule_627_e6588,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_627_e6590;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_628_e6609,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_402 == 0.0)) {
        let noise_metadata_schedule_628_e6598: f64 = (noise_variable_105 * noise_variable_112);
        let noise_metadata_schedule_628_e6600: f64 = (noise_metadata_schedule_628_e6598 * noise_variable_5);
        let noise_metadata_schedule_628_e6601: f64 = (noise_variable_167 + noise_metadata_schedule_628_e6600);
        let noise_metadata_schedule_628_e6604: f64 = (noise_variable_166 * noise_variable_217);
        let noise_metadata_schedule_628_e6606: f64 = (noise_metadata_schedule_628_e6604 * noise_variable_155);
        let noise_metadata_schedule_628_e6607: f64 = (noise_metadata_schedule_628_e6601 + noise_metadata_schedule_628_e6606);
        (noise_metadata_schedule_628_e6607,)
    } else {
        (noise_variable_106,)
    }
};
            noise_variable_106 = noise_metadata_schedule_628_e6609;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_629_e6619,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_629_e6613: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_629_e6615: f64 = (noise_metadata_schedule_629_e6613 * noise_variable_101);
        let noise_metadata_schedule_629_e6617: f64 = (noise_metadata_schedule_629_e6615 * noise_variable_217);
        (noise_metadata_schedule_629_e6617,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_629_e6619;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_630_e6627,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_630_e6623: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_630_e6625: f64 = (noise_metadata_schedule_630_e6623 * noise_variable_102);
        (noise_metadata_schedule_630_e6625,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_630_e6627;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_631_e6635,) = {
    if (noise_variable_399 != 0.0) {
        let noise_metadata_schedule_631_e6631: f64 = (noise_variable_99 * noise_variable_217);
        let noise_metadata_schedule_631_e6633: f64 = (noise_metadata_schedule_631_e6631 + noise_variable_103);
        (noise_metadata_schedule_631_e6633,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_631_e6635;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_632_e6638: f64 = if params.p0 >= 310.0 { 1.0 } else { 0.0 };
            noise_variable_405 = noise_metadata_schedule_632_e6638;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_633_e6650,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 != 0.0)) {
        let noise_metadata_schedule_633_e6644: f64 = (noise_variable_355 + noise_variable_354);
        let noise_metadata_schedule_633_e6646: f64 = (noise_metadata_schedule_633_e6644 + noise_variable_97);
        let noise_metadata_schedule_633_e6648: f64 = (noise_metadata_schedule_633_e6646 + noise_variable_105);
        (noise_metadata_schedule_633_e6648,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_633_e6650;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_634_e6664,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 != 0.0)) {
        let noise_metadata_schedule_634_e6657: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_634_e6658: f64 = (noise_variable_219 + noise_metadata_schedule_634_e6657);
        let noise_metadata_schedule_634_e6660: f64 = (noise_metadata_schedule_634_e6658 + noise_variable_98);
        let noise_metadata_schedule_634_e6662: f64 = (noise_metadata_schedule_634_e6660 + noise_variable_106);
        (noise_metadata_schedule_634_e6662,)
    } else {
        (noise_variable_219,)
    }
};
            noise_variable_219 = noise_metadata_schedule_634_e6664;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_635_e6682,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 != 0.0)) {
        let noise_metadata_schedule_635_e6671: f64 = (params.p5 * noise_variable_354);
        let noise_metadata_schedule_635_e6672: f64 = (noise_variable_358 + noise_metadata_schedule_635_e6671);
        let noise_metadata_schedule_635_e6675: f64 = (noise_variable_20 * noise_variable_97);
        let noise_metadata_schedule_635_e6676: f64 = (noise_metadata_schedule_635_e6672 + noise_metadata_schedule_635_e6675);
        let noise_metadata_schedule_635_e6679: f64 = (noise_variable_21 * noise_variable_105);
        let noise_metadata_schedule_635_e6680: f64 = (noise_metadata_schedule_635_e6676 + noise_metadata_schedule_635_e6679);
        (noise_metadata_schedule_635_e6680,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_635_e6682;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_636_e6702,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 != 0.0)) {
        let noise_metadata_schedule_636_e6690: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_636_e6691: f64 = (params.p5 * noise_metadata_schedule_636_e6690);
        let noise_metadata_schedule_636_e6692: f64 = (noise_variable_359 + noise_metadata_schedule_636_e6691);
        let noise_metadata_schedule_636_e6695: f64 = (noise_variable_20 * noise_variable_98);
        let noise_metadata_schedule_636_e6696: f64 = (noise_metadata_schedule_636_e6692 + noise_metadata_schedule_636_e6695);
        let noise_metadata_schedule_636_e6699: f64 = (noise_variable_21 * noise_variable_106);
        let noise_metadata_schedule_636_e6700: f64 = (noise_metadata_schedule_636_e6696 + noise_metadata_schedule_636_e6699);
        (noise_metadata_schedule_636_e6700,)
    } else {
        (noise_variable_359,)
    }
};
            noise_variable_359 = noise_metadata_schedule_636_e6702;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_637_e6721,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 == 0.0)) {
        let noise_metadata_schedule_637_e6709: f64 = (noise_variable_19 * noise_variable_355);
        let noise_metadata_schedule_637_e6711: f64 = (noise_metadata_schedule_637_e6709 + noise_variable_354);
        let noise_metadata_schedule_637_e6714: f64 = (noise_variable_20 * noise_variable_97);
        let noise_metadata_schedule_637_e6715: f64 = (noise_metadata_schedule_637_e6711 + noise_metadata_schedule_637_e6714);
        let noise_metadata_schedule_637_e6718: f64 = (noise_variable_21 * noise_variable_105);
        let noise_metadata_schedule_637_e6719: f64 = (noise_metadata_schedule_637_e6715 + noise_metadata_schedule_637_e6718);
        (noise_metadata_schedule_637_e6719,)
    } else {
        (noise_variable_358,)
    }
};
            noise_variable_358 = noise_metadata_schedule_637_e6721;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_638_e6734,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 == 0.0)) {
        let noise_metadata_schedule_638_e6728: f64 = (noise_variable_355 + noise_variable_354);
        let noise_metadata_schedule_638_e6730: f64 = (noise_metadata_schedule_638_e6728 + noise_variable_97);
        let noise_metadata_schedule_638_e6732: f64 = (noise_metadata_schedule_638_e6730 + noise_variable_105);
        (noise_metadata_schedule_638_e6732,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_638_e6734;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_639_e6755,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 == 0.0)) {
        let noise_metadata_schedule_639_e6741: f64 = (noise_variable_19 * noise_variable_219);
        let noise_metadata_schedule_639_e6744: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_639_e6745: f64 = (noise_metadata_schedule_639_e6741 + noise_metadata_schedule_639_e6744);
        let noise_metadata_schedule_639_e6748: f64 = (noise_variable_20 * noise_variable_98);
        let noise_metadata_schedule_639_e6749: f64 = (noise_metadata_schedule_639_e6745 + noise_metadata_schedule_639_e6748);
        let noise_metadata_schedule_639_e6752: f64 = (noise_variable_21 * noise_variable_106);
        let noise_metadata_schedule_639_e6753: f64 = (noise_metadata_schedule_639_e6749 + noise_metadata_schedule_639_e6752);
        (noise_metadata_schedule_639_e6753,)
    } else {
        (noise_variable_359,)
    }
};
            noise_variable_359 = noise_metadata_schedule_639_e6755;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_640_e6770,) = {
    if ((noise_variable_399 != 0.0) && (noise_variable_405 == 0.0)) {
        let noise_metadata_schedule_640_e6763: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_640_e6764: f64 = (noise_variable_219 + noise_metadata_schedule_640_e6763);
        let noise_metadata_schedule_640_e6766: f64 = (noise_metadata_schedule_640_e6764 + noise_variable_98);
        let noise_metadata_schedule_640_e6768: f64 = (noise_metadata_schedule_640_e6766 + noise_variable_106);
        (noise_metadata_schedule_640_e6768,)
    } else {
        (noise_variable_219,)
    }
};
            noise_variable_219 = noise_metadata_schedule_640_e6770;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_641_e6773: f64 = (params.p85 * noise_variable_218);
            noise_variable_356 = noise_metadata_schedule_641_e6773;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            noise_variable_224 = 0.0;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let noise_metadata_schedule_643_e6781: f64 = 1e-5;
            let noise_metadata_schedule_643_e6783: f64 = (noise_metadata_schedule_643_e6781 * noise_variable_348);
            let noise_metadata_schedule_643_e6792: f64 = 1e-5;
            let noise_metadata_schedule_643_e6794: f64 = (noise_metadata_schedule_643_e6792 * noise_variable_348);
            let noise_metadata_schedule_643_e6797: f64 = if (((params.p0 >= 310.0) && (noise_variable_358 > noise_metadata_schedule_643_e6783)) || ((params.p0 <= 300.0) && (noise_variable_355 > noise_metadata_schedule_643_e6794))) { 1.0 } else { 0.0 };
            noise_variable_406 = noise_metadata_schedule_643_e6797;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_644_e6806,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_644_e6801: f64 = (noise_variable_357 * noise_variable_217);
        let noise_metadata_schedule_644_e6803: f64 = (noise_metadata_schedule_644_e6801 * noise_variable_358);
        let noise_metadata_schedule_644_e6804: f64 = (noise_metadata_schedule_644_e6803).sqrt();
        (noise_metadata_schedule_644_e6804,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_644_e6806;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_645_e6816,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_645_e6810: f64 = (noise_variable_352 + noise_variable_355);
        let noise_metadata_schedule_645_e6813: f64 = (params.p7 * noise_variable_356);
        let noise_metadata_schedule_645_e6814: f64 = (noise_metadata_schedule_645_e6810 + noise_metadata_schedule_645_e6813);
        (noise_metadata_schedule_645_e6814,)
    } else {
        (noise_variable_348,)
    }
};
            noise_variable_348 = noise_metadata_schedule_645_e6816;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_646_e6820,) = {
    if (noise_variable_406 != 0.0) {
        (noise_variable_348,)
    } else {
        (noise_variable_349,)
    }
};
            noise_variable_349 = noise_metadata_schedule_646_e6820;
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let mut noise_metadata_schedule_647_iterations = 0usize;
            loop {
                let noise_metadata_schedule_647_condition_e6823: f64 = (noise_variable_349).abs();
                let noise_metadata_schedule_647_condition_e6826: f64 = 1e-5;
                let noise_metadata_schedule_647_condition_e6828: f64 = (noise_variable_348).abs();
                let noise_metadata_schedule_647_condition_e6829: f64 = (noise_metadata_schedule_647_condition_e6826 * noise_metadata_schedule_647_condition_e6828);
                let noise_metadata_schedule_647_condition_e6835: f64 = if ((noise_variable_406 != 0.0) && ((noise_metadata_schedule_647_condition_e6823 >= noise_metadata_schedule_647_condition_e6829) && (noise_variable_224 <= 100.0))) { 1.0 } else { 0.0 };
                if noise_metadata_schedule_647_condition_e6835 == 0.0 { break; }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_0_e6841,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_0_e6839: f64 = (noise_variable_350 / noise_variable_348);
        (noise_metadata_schedule_647_body_0_e6839,)
    } else {
        (noise_variable_217,)
    }
};
                    noise_variable_217 = noise_metadata_schedule_647_body_0_e6841;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_1_e6847,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_1_e6845: f64 = (noise_variable_351 / noise_variable_348);
        (noise_metadata_schedule_647_body_1_e6845,)
    } else {
        (noise_variable_218,)
    }
};
                    noise_variable_218 = noise_metadata_schedule_647_body_1_e6847;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_2_e6851,) = {
    if (noise_variable_406 != 0.0) {
        (noise_variable_357,)
    } else {
        (noise_variable_219,)
    }
};
                    noise_variable_219 = noise_metadata_schedule_647_body_2_e6851;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_3_e6857,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_3_e6855: f64 = (noise_variable_357 * noise_variable_217);
        (noise_metadata_schedule_647_body_3_e6855,)
    } else {
        (noise_variable_355,)
    }
};
                    noise_variable_355 = noise_metadata_schedule_647_body_3_e6857;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_4_e6860: f64 = if params.p0 >= 310.0 { 1.0 } else { 0.0 };
                    noise_variable_408 = noise_metadata_schedule_647_body_4_e6860;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_5_e6868,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_408 != 0.0)) {
        let noise_metadata_schedule_647_body_5_e6866: f64 = (noise_variable_19 * noise_variable_59);
        (noise_metadata_schedule_647_body_5_e6866,)
    } else {
        (noise_variable_359,)
    }
};
                    noise_variable_359 = noise_metadata_schedule_647_body_5_e6868;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_6_e6876,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_408 != 0.0)) {
        let noise_metadata_schedule_647_body_6_e6874: f64 = (noise_variable_359 * noise_variable_217);
        (noise_metadata_schedule_647_body_6_e6874,)
    } else {
        (noise_variable_358,)
    }
};
                    noise_variable_358 = noise_metadata_schedule_647_body_6_e6876;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_7_e6885,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_408 == 0.0)) {
        let noise_metadata_schedule_647_body_7_e6883: f64 = (noise_variable_19 * noise_variable_355);
        (noise_metadata_schedule_647_body_7_e6883,)
    } else {
        (noise_variable_358,)
    }
};
                    noise_variable_358 = noise_metadata_schedule_647_body_7_e6885;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_8_e6894,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_408 == 0.0)) {
        let noise_metadata_schedule_647_body_8_e6892: f64 = (noise_variable_19 * noise_variable_219);
        (noise_metadata_schedule_647_body_8_e6892,)
    } else {
        (noise_variable_359,)
    }
};
                    noise_variable_359 = noise_metadata_schedule_647_body_8_e6894;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_9_e6898,) = {
    if (noise_variable_406 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_354,)
    }
};
                    noise_variable_354 = noise_metadata_schedule_647_body_9_e6898;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_10_e6902: f64 = (1e-6 * noise_variable_362);
                    let noise_metadata_schedule_647_body_10_e6907: f64 = if ((noise_variable_217 >= noise_metadata_schedule_647_body_10_e6902) || (params.p0 >= 320.0)) { 1.0 } else { 0.0 };
                    noise_variable_409 = noise_metadata_schedule_647_body_10_e6907;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_11_e6915,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_11_e6913: f64 = (noise_variable_217 / noise_variable_362);
        (noise_metadata_schedule_647_body_11_e6913,)
    } else {
        (noise_variable_96,)
    }
};
                    noise_variable_96 = noise_metadata_schedule_647_body_11_e6915;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_12_e6927,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_12_e6922: f64 = (noise_variable_96).ln();
        let noise_metadata_schedule_647_body_12_e6923: f64 = (params.p70 * noise_metadata_schedule_647_body_12_e6922);
        let noise_metadata_schedule_647_body_12_e6924: f64 = (noise_metadata_schedule_647_body_12_e6923).exp();
        let noise_metadata_schedule_647_body_12_e6925: f64 = (noise_variable_61 * noise_metadata_schedule_647_body_12_e6924);
        (noise_metadata_schedule_647_body_12_e6925,)
    } else {
        (noise_variable_98,)
    }
};
                    noise_variable_98 = noise_metadata_schedule_647_body_12_e6927;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_13_e6939,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_13_e6933: f64 = (noise_variable_98 * noise_variable_217);
        let noise_metadata_schedule_647_body_13_e6936: f64 = (1.0 + params.p70);
        let noise_metadata_schedule_647_body_13_e6937: f64 = (noise_metadata_schedule_647_body_13_e6933 / noise_metadata_schedule_647_body_13_e6936);
        (noise_metadata_schedule_647_body_13_e6937,)
    } else {
        (noise_variable_97,)
    }
};
                    noise_variable_97 = noise_metadata_schedule_647_body_13_e6939;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_14_e6944: f64 = (params.p75 / params.p74);
                    let noise_metadata_schedule_647_body_14_e6945: f64 = (0.05 * noise_metadata_schedule_647_body_14_e6944);
                    let noise_metadata_schedule_647_body_14_e6946: f64 = if params.p83 < noise_metadata_schedule_647_body_14_e6945 { 1.0 } else { 0.0 };
                    noise_variable_410 = noise_metadata_schedule_647_body_14_e6946;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_15_e6954,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_410 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_111,)
    }
};
                    noise_variable_111 = noise_metadata_schedule_647_body_15_e6954;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_16_e6962,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_410 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_112,)
    }
};
                    noise_variable_112 = noise_metadata_schedule_647_body_16_e6962;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_17_e6975,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_410 == 0.0)) {
        let noise_metadata_schedule_647_body_17_e6971: f64 = (noise_variable_217 - noise_variable_362);
        let noise_metadata_schedule_647_body_17_e6973: f64 = (noise_metadata_schedule_647_body_17_e6971 / params.p83);
        (noise_metadata_schedule_647_body_17_e6973,)
    } else {
        (noise_variable_107,)
    }
};
                    noise_variable_107 = noise_metadata_schedule_647_body_17_e6975;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_18_e6978: f64 = (-10000000000.0);
                    let noise_metadata_schedule_647_body_18_e6979: f64 = if noise_variable_107 < noise_metadata_schedule_647_body_18_e6978 { 1.0 } else { 0.0 };
                    noise_variable_411 = noise_metadata_schedule_647_body_18_e6979;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_19_e6991,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_410 == 0.0)) && (noise_variable_411 != 0.0)) {
        let noise_metadata_schedule_647_body_19_e6989: f64 = (-10000000000.0);
        (noise_metadata_schedule_647_body_19_e6989,)
    } else {
        (noise_variable_107,)
    }
};
                    noise_variable_107 = noise_metadata_schedule_647_body_19_e6991;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_20_e7005,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_410 == 0.0)) {
        let noise_metadata_schedule_647_body_20_e7000: f64 = (noise_variable_107 * noise_variable_107);
        let noise_metadata_schedule_647_body_20_e7002: f64 = (noise_metadata_schedule_647_body_20_e7000 + params.p84);
        let noise_metadata_schedule_647_body_20_e7003: f64 = (noise_metadata_schedule_647_body_20_e7002).sqrt();
        (noise_metadata_schedule_647_body_20_e7003,)
    } else {
        (noise_variable_95,)
    }
};
                    noise_variable_95 = noise_metadata_schedule_647_body_20_e7005;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_21_e7022,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_410 == 0.0)) {
        let noise_metadata_schedule_647_body_21_e7014: f64 = (-2.0);
        let noise_metadata_schedule_647_body_21_e7017: f64 = (noise_variable_107 + noise_variable_95);
        let noise_metadata_schedule_647_body_21_e7018: f64 = (noise_metadata_schedule_647_body_21_e7014 / noise_metadata_schedule_647_body_21_e7017);
        let noise_metadata_schedule_647_body_21_e7019: f64 = (noise_metadata_schedule_647_body_21_e7018).exp();
        let noise_metadata_schedule_647_body_21_e7020: f64 = (params.p82 * noise_metadata_schedule_647_body_21_e7019);
        (noise_metadata_schedule_647_body_21_e7020,)
    } else {
        (noise_variable_111,)
    }
};
                    noise_variable_111 = noise_metadata_schedule_647_body_21_e7022;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_22_e7041,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_410 == 0.0)) {
        let noise_metadata_schedule_647_body_22_e7031: f64 = (2.0 * noise_variable_111);
        let noise_metadata_schedule_647_body_22_e7034: f64 = (params.p83 * noise_variable_95);
        let noise_metadata_schedule_647_body_22_e7037: f64 = (noise_variable_107 + noise_variable_95);
        let noise_metadata_schedule_647_body_22_e7038: f64 = (noise_metadata_schedule_647_body_22_e7034 * noise_metadata_schedule_647_body_22_e7037);
        let noise_metadata_schedule_647_body_22_e7039: f64 = (noise_metadata_schedule_647_body_22_e7031 / noise_metadata_schedule_647_body_22_e7038);
        (noise_metadata_schedule_647_body_22_e7039,)
    } else {
        (noise_variable_112,)
    }
};
                    noise_variable_112 = noise_metadata_schedule_647_body_22_e7041;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_23_e7058,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_23_e7047: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_647_body_23_e7049: f64 = (noise_metadata_schedule_647_body_23_e7047 * noise_variable_60);
        let noise_metadata_schedule_647_body_23_e7052: f64 = (noise_variable_111 * noise_variable_5);
        let noise_metadata_schedule_647_body_23_e7053: f64 = (noise_metadata_schedule_647_body_23_e7052).exp();
        let noise_metadata_schedule_647_body_23_e7055: f64 = (noise_metadata_schedule_647_body_23_e7053 - 1.0);
        let noise_metadata_schedule_647_body_23_e7056: f64 = (noise_metadata_schedule_647_body_23_e7049 * noise_metadata_schedule_647_body_23_e7055);
        (noise_metadata_schedule_647_body_23_e7056,)
    } else {
        (noise_variable_99,)
    }
};
                    noise_variable_99 = noise_metadata_schedule_647_body_23_e7058;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_24_e7081,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_24_e7065: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_647_body_24_e7067: f64 = (noise_metadata_schedule_647_body_24_e7065 * noise_variable_60);
        let noise_metadata_schedule_647_body_24_e7069: f64 = (noise_metadata_schedule_647_body_24_e7067 * noise_variable_217);
        let noise_metadata_schedule_647_body_24_e7072: f64 = (noise_variable_111 * noise_variable_5);
        let noise_metadata_schedule_647_body_24_e7073: f64 = (noise_metadata_schedule_647_body_24_e7072).exp();
        let noise_metadata_schedule_647_body_24_e7074: f64 = (noise_metadata_schedule_647_body_24_e7069 * noise_metadata_schedule_647_body_24_e7073);
        let noise_metadata_schedule_647_body_24_e7076: f64 = (noise_metadata_schedule_647_body_24_e7074 * noise_variable_5);
        let noise_metadata_schedule_647_body_24_e7078: f64 = (noise_metadata_schedule_647_body_24_e7076 * noise_variable_112);
        let noise_metadata_schedule_647_body_24_e7079: f64 = (noise_variable_99 + noise_metadata_schedule_647_body_24_e7078);
        (noise_metadata_schedule_647_body_24_e7079,)
    } else {
        (noise_variable_100,)
    }
};
                    noise_variable_100 = noise_metadata_schedule_647_body_24_e7081;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_25_e7091,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_25_e7088: f64 = (1.0 / noise_variable_96);
        let noise_metadata_schedule_647_body_25_e7089: f64 = (1.0 - noise_metadata_schedule_647_body_25_e7088);
        (noise_metadata_schedule_647_body_25_e7089,)
    } else {
        (noise_variable_108,)
    }
};
                    noise_variable_108 = noise_metadata_schedule_647_body_25_e7091;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_26_e7111,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_26_e7098: f64 = (noise_variable_108 * noise_variable_108);
        let noise_metadata_schedule_647_body_26_e7100: f64 = (noise_metadata_schedule_647_body_26_e7098 + params.p72);
        let noise_metadata_schedule_647_body_26_e7101: f64 = (noise_metadata_schedule_647_body_26_e7100).sqrt();
        let noise_metadata_schedule_647_body_26_e7102: f64 = (noise_variable_108 + noise_metadata_schedule_647_body_26_e7101);
        let noise_metadata_schedule_647_body_26_e7106: f64 = (1.0 + params.p72);
        let noise_metadata_schedule_647_body_26_e7107: f64 = (noise_metadata_schedule_647_body_26_e7106).sqrt();
        let noise_metadata_schedule_647_body_26_e7108: f64 = (1.0 + noise_metadata_schedule_647_body_26_e7107);
        let noise_metadata_schedule_647_body_26_e7109: f64 = (noise_metadata_schedule_647_body_26_e7102 / noise_metadata_schedule_647_body_26_e7108);
        (noise_metadata_schedule_647_body_26_e7109,)
    } else {
        (noise_variable_109,)
    }
};
                    noise_variable_109 = noise_metadata_schedule_647_body_26_e7111;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_27_e7122,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_27_e7117: f64 = (noise_variable_111 - params.p82);
        let noise_metadata_schedule_647_body_27_e7119: f64 = (noise_metadata_schedule_647_body_27_e7117 * noise_variable_5);
        let noise_metadata_schedule_647_body_27_e7120: f64 = (noise_metadata_schedule_647_body_27_e7119).exp();
        (noise_metadata_schedule_647_body_27_e7120,)
    } else {
        (noise_variable_110,)
    }
};
                    noise_variable_110 = noise_metadata_schedule_647_body_27_e7122;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_28_e7134,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_28_e7128: f64 = (noise_variable_60 * noise_variable_109);
        let noise_metadata_schedule_647_body_28_e7130: f64 = (noise_metadata_schedule_647_body_28_e7128 * noise_variable_109);
        let noise_metadata_schedule_647_body_28_e7132: f64 = (noise_metadata_schedule_647_body_28_e7130 * noise_variable_110);
        (noise_metadata_schedule_647_body_28_e7132,)
    } else {
        (noise_variable_101,)
    }
};
                    noise_variable_101 = noise_metadata_schedule_647_body_28_e7134;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_29_e7159,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_29_e7144: f64 = (noise_variable_108 * noise_variable_108);
        let noise_metadata_schedule_647_body_29_e7146: f64 = (noise_metadata_schedule_647_body_29_e7144 + params.p72);
        let noise_metadata_schedule_647_body_29_e7147: f64 = (noise_metadata_schedule_647_body_29_e7146).sqrt();
        let noise_metadata_schedule_647_body_29_e7148: f64 = (noise_variable_96 * noise_metadata_schedule_647_body_29_e7147);
        let noise_metadata_schedule_647_body_29_e7149: f64 = (2.0 / noise_metadata_schedule_647_body_29_e7148);
        let noise_metadata_schedule_647_body_29_e7150: f64 = (1.0 + noise_metadata_schedule_647_body_29_e7149);
        let noise_metadata_schedule_647_body_29_e7153: f64 = (noise_variable_5 * noise_variable_217);
        let noise_metadata_schedule_647_body_29_e7155: f64 = (noise_metadata_schedule_647_body_29_e7153 * noise_variable_112);
        let noise_metadata_schedule_647_body_29_e7156: f64 = (noise_metadata_schedule_647_body_29_e7150 + noise_metadata_schedule_647_body_29_e7155);
        let noise_metadata_schedule_647_body_29_e7157: f64 = (noise_variable_101 * noise_metadata_schedule_647_body_29_e7156);
        (noise_metadata_schedule_647_body_29_e7157,)
    } else {
        (noise_variable_102,)
    }
};
                    noise_variable_102 = noise_metadata_schedule_647_body_29_e7159;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_30_e7169: f64 = (noise_variable_109 * params.p115);
                    let noise_metadata_schedule_647_body_30_e7175: f64 = (noise_variable_109 * params.p116);
                    let noise_metadata_schedule_647_body_30_e7178: f64 = if ((((params.p115 < 0.01) && (params.p116 < 0.01)) && (noise_metadata_schedule_647_body_30_e7169 < 0.005)) && (noise_metadata_schedule_647_body_30_e7175 < 0.005)) { 1.0 } else { 0.0 };
                    noise_variable_412 = noise_metadata_schedule_647_body_30_e7178;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_31_e7190,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 != 0.0)) {
        let noise_metadata_schedule_647_body_31_e7186: f64 = (params.p73 * noise_variable_101);
        let noise_metadata_schedule_647_body_31_e7188: f64 = (noise_metadata_schedule_647_body_31_e7186 * noise_variable_217);
        (noise_metadata_schedule_647_body_31_e7188,)
    } else {
        (noise_variable_105,)
    }
};
                    noise_variable_105 = noise_metadata_schedule_647_body_31_e7190;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_32_e7200,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 != 0.0)) {
        let noise_metadata_schedule_647_body_32_e7198: f64 = (params.p73 * noise_variable_102);
        (noise_metadata_schedule_647_body_32_e7198,)
    } else {
        (noise_variable_106,)
    }
};
                    noise_variable_106 = noise_metadata_schedule_647_body_32_e7200;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_33_e7211,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) {
        let noise_metadata_schedule_647_body_33_e7209: f64 = (1.0 - noise_variable_109);
        (noise_metadata_schedule_647_body_33_e7209,)
    } else {
        (noise_variable_146,)
    }
};
                    noise_variable_146 = noise_metadata_schedule_647_body_33_e7211;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_34_e7235,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) {
        let noise_metadata_schedule_647_body_34_e7220: f64 = (noise_variable_146 - 1.0);
        let noise_metadata_schedule_647_body_34_e7223: f64 = (1.0 - noise_variable_108);
        let noise_metadata_schedule_647_body_34_e7224: f64 = (noise_metadata_schedule_647_body_34_e7220 * noise_metadata_schedule_647_body_34_e7223);
        let noise_metadata_schedule_647_body_34_e7227: f64 = (noise_variable_108 * noise_variable_108);
        let noise_metadata_schedule_647_body_34_e7229: f64 = (noise_metadata_schedule_647_body_34_e7227 + params.p72);
        let noise_metadata_schedule_647_body_34_e7230: f64 = (noise_metadata_schedule_647_body_34_e7229).sqrt();
        let noise_metadata_schedule_647_body_34_e7232: f64 = (noise_metadata_schedule_647_body_34_e7230 * noise_variable_217);
        let noise_metadata_schedule_647_body_34_e7233: f64 = (noise_metadata_schedule_647_body_34_e7224 / noise_metadata_schedule_647_body_34_e7232);
        (noise_metadata_schedule_647_body_34_e7233,)
    } else {
        (noise_variable_147,)
    }
};
                    noise_variable_147 = noise_metadata_schedule_647_body_34_e7235;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_35_e7237: f64 = (noise_variable_232).abs();
                    let noise_metadata_schedule_647_body_35_e7239: f64 = if noise_metadata_schedule_647_body_35_e7237 > 0.001 { 1.0 } else { 0.0 };
                    noise_variable_413 = noise_metadata_schedule_647_body_35_e7239;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_36_e7255,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) {
        let noise_metadata_schedule_647_body_36_e7250: f64 = (noise_variable_146 - 1.0);
        let noise_metadata_schedule_647_body_36_e7252: f64 = (noise_metadata_schedule_647_body_36_e7250 * noise_variable_231);
        let noise_metadata_schedule_647_body_36_e7253: f64 = (noise_metadata_schedule_647_body_36_e7252).exp();
        (noise_metadata_schedule_647_body_36_e7253,)
    } else {
        (noise_variable_151,)
    }
};
                    noise_variable_151 = noise_metadata_schedule_647_body_36_e7255;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_37_e7258: f64 = if noise_variable_229 < 0.01 { 1.0 } else { 0.0 };
                    noise_variable_414 = noise_metadata_schedule_647_body_37_e7258;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_38_e7277,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 != 0.0)) {
        let noise_metadata_schedule_647_body_38_e7271: f64 = (1.0 - noise_variable_151);
        let noise_metadata_schedule_647_body_38_e7274: f64 = (noise_variable_151 * noise_variable_230);
        let noise_metadata_schedule_647_body_38_e7275: f64 = (noise_metadata_schedule_647_body_38_e7271 / noise_metadata_schedule_647_body_38_e7274);
        (noise_metadata_schedule_647_body_38_e7275,)
    } else {
        (noise_variable_149,)
    }
};
                    noise_variable_149 = noise_metadata_schedule_647_body_38_e7277;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_39_e7294,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 != 0.0)) {
        let noise_metadata_schedule_647_body_39_e7291: f64 = (noise_variable_230 * noise_variable_149);
        let noise_metadata_schedule_647_body_39_e7292: f64 = (1.0 + noise_metadata_schedule_647_body_39_e7291);
        (noise_metadata_schedule_647_body_39_e7292,)
    } else {
        (noise_variable_148,)
    }
};
                    noise_variable_148 = noise_metadata_schedule_647_body_39_e7294;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_40_e7328,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 != 0.0)) {
        let noise_metadata_schedule_647_body_40_e7308: f64 = (noise_variable_230 * noise_variable_149);
        let noise_metadata_schedule_647_body_40_e7312: f64 = (0.25 * noise_variable_230);
        let noise_metadata_schedule_647_body_40_e7314: f64 = (noise_metadata_schedule_647_body_40_e7312 * noise_variable_149);
        let noise_metadata_schedule_647_body_40_e7315: f64 = (0.5 + noise_metadata_schedule_647_body_40_e7314);
        let noise_metadata_schedule_647_body_40_e7316: f64 = (noise_metadata_schedule_647_body_40_e7308 * noise_metadata_schedule_647_body_40_e7315);
        let noise_metadata_schedule_647_body_40_e7319: f64 = (noise_variable_148).ln();
        let noise_metadata_schedule_647_body_40_e7320: f64 = (0.5 * noise_metadata_schedule_647_body_40_e7319);
        let noise_metadata_schedule_647_body_40_e7321: f64 = (noise_metadata_schedule_647_body_40_e7316 - noise_metadata_schedule_647_body_40_e7320);
        let noise_metadata_schedule_647_body_40_e7322: f64 = (2.0 * noise_metadata_schedule_647_body_40_e7321);
        let noise_metadata_schedule_647_body_40_e7324: f64 = (noise_metadata_schedule_647_body_40_e7322 / noise_variable_230);
        let noise_metadata_schedule_647_body_40_e7326: f64 = (noise_metadata_schedule_647_body_40_e7324 / noise_variable_230);
        (noise_metadata_schedule_647_body_40_e7326,)
    } else {
        (noise_variable_154,)
    }
};
                    noise_variable_154 = noise_metadata_schedule_647_body_40_e7328;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_41_e7348,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 != 0.0)) {
        let noise_metadata_schedule_647_body_41_e7340: f64 = (-noise_variable_231);
        let noise_metadata_schedule_647_body_41_e7342: f64 = (noise_metadata_schedule_647_body_41_e7340 * noise_variable_147);
        let noise_metadata_schedule_647_body_41_e7345: f64 = (noise_variable_151 * noise_variable_230);
        let noise_metadata_schedule_647_body_41_e7346: f64 = (noise_metadata_schedule_647_body_41_e7342 / noise_metadata_schedule_647_body_41_e7345);
        (noise_metadata_schedule_647_body_41_e7346,)
    } else {
        (noise_variable_150,)
    }
};
                    noise_variable_150 = noise_metadata_schedule_647_body_41_e7348;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_42_e7369,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 != 0.0)) {
        let noise_metadata_schedule_647_body_42_e7361: f64 = (1.0 + noise_variable_148);
        let noise_metadata_schedule_647_body_42_e7363: f64 = (noise_metadata_schedule_647_body_42_e7361 * noise_variable_149);
        let noise_metadata_schedule_647_body_42_e7365: f64 = (noise_metadata_schedule_647_body_42_e7363 * noise_variable_150);
        let noise_metadata_schedule_647_body_42_e7367: f64 = (noise_metadata_schedule_647_body_42_e7365 / noise_variable_148);
        (noise_metadata_schedule_647_body_42_e7367,)
    } else {
        (noise_variable_155,)
    }
};
                    noise_variable_155 = noise_metadata_schedule_647_body_42_e7369;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_43_e7387,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_43_e7384: f64 = (noise_variable_151 * params.p115);
        let noise_metadata_schedule_647_body_43_e7385: f64 = (params.p116 - noise_metadata_schedule_647_body_43_e7384);
        (noise_metadata_schedule_647_body_43_e7385,)
    } else {
        (noise_variable_152,)
    }
};
                    noise_variable_152 = noise_metadata_schedule_647_body_43_e7387;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_44_e7405,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_44_e7401: f64 = (noise_variable_151 - 1.0);
        let noise_metadata_schedule_647_body_44_e7403: f64 = (noise_metadata_schedule_647_body_44_e7401 / noise_variable_152);
        (noise_metadata_schedule_647_body_44_e7403,)
    } else {
        (noise_variable_149,)
    }
};
                    noise_variable_149 = noise_metadata_schedule_647_body_44_e7405;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_45_e7423,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_45_e7420: f64 = (params.p116 * noise_variable_149);
        let noise_metadata_schedule_647_body_45_e7421: f64 = (1.0 + noise_metadata_schedule_647_body_45_e7420);
        (noise_metadata_schedule_647_body_45_e7421,)
    } else {
        (noise_variable_160,)
    }
};
                    noise_variable_160 = noise_metadata_schedule_647_body_45_e7423;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_46_e7438,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_46_e7436: f64 = (noise_variable_160).ln();
        (noise_metadata_schedule_647_body_46_e7436,)
    } else {
        (noise_variable_161,)
    }
};
                    noise_variable_161 = noise_metadata_schedule_647_body_46_e7438;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_47_e7454,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_47_e7452: f64 = (noise_variable_227 * noise_variable_226);
        (noise_metadata_schedule_647_body_47_e7452,)
    } else {
        (noise_variable_162,)
    }
};
                    noise_variable_162 = noise_metadata_schedule_647_body_47_e7454;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_48_e7482,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_48_e7469: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_647_body_48_e7470: f64 = (noise_variable_161 * noise_metadata_schedule_647_body_48_e7469);
        let noise_metadata_schedule_647_body_48_e7472: f64 = (noise_metadata_schedule_647_body_48_e7470 * noise_variable_226);
        let noise_metadata_schedule_647_body_48_e7476: f64 = (noise_variable_227 * noise_variable_149);
        let noise_metadata_schedule_647_body_48_e7477: f64 = (noise_variable_162 + noise_metadata_schedule_647_body_48_e7476);
        let noise_metadata_schedule_647_body_48_e7479: f64 = (noise_metadata_schedule_647_body_48_e7477 * noise_variable_149);
        let noise_metadata_schedule_647_body_48_e7480: f64 = (noise_metadata_schedule_647_body_48_e7472 + noise_metadata_schedule_647_body_48_e7479);
        (noise_metadata_schedule_647_body_48_e7480,)
    } else {
        (noise_variable_157,)
    }
};
                    noise_variable_157 = noise_metadata_schedule_647_body_48_e7482;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_49_e7508,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_49_e7496: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_647_body_49_e7498: f64 = (noise_metadata_schedule_647_body_49_e7496 / noise_variable_160);
        let noise_metadata_schedule_647_body_49_e7500: f64 = (noise_metadata_schedule_647_body_49_e7498 + noise_variable_162);
        let noise_metadata_schedule_647_body_49_e7503: f64 = (noise_variable_149 * noise_variable_227);
        let noise_metadata_schedule_647_body_49_e7505: f64 = (noise_metadata_schedule_647_body_49_e7503 * 2.0);
        let noise_metadata_schedule_647_body_49_e7506: f64 = (noise_metadata_schedule_647_body_49_e7500 + noise_metadata_schedule_647_body_49_e7505);
        (noise_metadata_schedule_647_body_49_e7506,)
    } else {
        (noise_variable_159,)
    }
};
                    noise_variable_159 = noise_metadata_schedule_647_body_49_e7508;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_50_e7526,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_50_e7523: f64 = (params.p115 * noise_variable_149);
        let noise_metadata_schedule_647_body_50_e7524: f64 = (1.0 + noise_metadata_schedule_647_body_50_e7523);
        (noise_metadata_schedule_647_body_50_e7524,)
    } else {
        (noise_variable_160,)
    }
};
                    noise_variable_160 = noise_metadata_schedule_647_body_50_e7526;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_51_e7541,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_51_e7539: f64 = (noise_variable_160).ln();
        (noise_metadata_schedule_647_body_51_e7539,)
    } else {
        (noise_variable_161,)
    }
};
                    noise_variable_161 = noise_metadata_schedule_647_body_51_e7541;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_52_e7557,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_52_e7555: f64 = (noise_variable_228 * noise_variable_225);
        (noise_metadata_schedule_647_body_52_e7555,)
    } else {
        (noise_variable_162,)
    }
};
                    noise_variable_162 = noise_metadata_schedule_647_body_52_e7557;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_53_e7585,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_53_e7572: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_647_body_53_e7573: f64 = (noise_variable_161 * noise_metadata_schedule_647_body_53_e7572);
        let noise_metadata_schedule_647_body_53_e7575: f64 = (noise_metadata_schedule_647_body_53_e7573 * noise_variable_225);
        let noise_metadata_schedule_647_body_53_e7579: f64 = (noise_variable_228 * noise_variable_149);
        let noise_metadata_schedule_647_body_53_e7580: f64 = (noise_variable_162 + noise_metadata_schedule_647_body_53_e7579);
        let noise_metadata_schedule_647_body_53_e7582: f64 = (noise_metadata_schedule_647_body_53_e7580 * noise_variable_149);
        let noise_metadata_schedule_647_body_53_e7583: f64 = (noise_metadata_schedule_647_body_53_e7575 + noise_metadata_schedule_647_body_53_e7582);
        (noise_metadata_schedule_647_body_53_e7583,)
    } else {
        (noise_variable_156,)
    }
};
                    noise_variable_156 = noise_metadata_schedule_647_body_53_e7585;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_54_e7611,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_54_e7599: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_647_body_54_e7601: f64 = (noise_metadata_schedule_647_body_54_e7599 / noise_variable_160);
        let noise_metadata_schedule_647_body_54_e7603: f64 = (noise_metadata_schedule_647_body_54_e7601 + noise_variable_162);
        let noise_metadata_schedule_647_body_54_e7606: f64 = (noise_variable_149 * noise_variable_228);
        let noise_metadata_schedule_647_body_54_e7608: f64 = (noise_metadata_schedule_647_body_54_e7606 * 2.0);
        let noise_metadata_schedule_647_body_54_e7609: f64 = (noise_metadata_schedule_647_body_54_e7603 + noise_metadata_schedule_647_body_54_e7608);
        (noise_metadata_schedule_647_body_54_e7609,)
    } else {
        (noise_variable_158,)
    }
};
                    noise_variable_158 = noise_metadata_schedule_647_body_54_e7611;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_55_e7629,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_55_e7625: f64 = (noise_variable_157 - noise_variable_156);
        let noise_metadata_schedule_647_body_55_e7627: f64 = (noise_metadata_schedule_647_body_55_e7625 / noise_variable_232);
        (noise_metadata_schedule_647_body_55_e7627,)
    } else {
        (noise_variable_154,)
    }
};
                    noise_variable_154 = noise_metadata_schedule_647_body_55_e7629;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_56_e7656,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_56_e7642: f64 = (-2.0);
        let noise_metadata_schedule_647_body_56_e7644: f64 = (noise_metadata_schedule_647_body_56_e7642 * noise_variable_232);
        let noise_metadata_schedule_647_body_56_e7647: f64 = (noise_variable_152 * noise_variable_152);
        let noise_metadata_schedule_647_body_56_e7648: f64 = (noise_metadata_schedule_647_body_56_e7644 / noise_metadata_schedule_647_body_56_e7647);
        let noise_metadata_schedule_647_body_56_e7650: f64 = (noise_metadata_schedule_647_body_56_e7648 * noise_variable_151);
        let noise_metadata_schedule_647_body_56_e7652: f64 = (noise_metadata_schedule_647_body_56_e7650 * noise_variable_231);
        let noise_metadata_schedule_647_body_56_e7654: f64 = (noise_metadata_schedule_647_body_56_e7652 * noise_variable_147);
        (noise_metadata_schedule_647_body_56_e7654,)
    } else {
        (noise_variable_150,)
    }
};
                    noise_variable_150 = noise_metadata_schedule_647_body_56_e7656;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_57_e7676,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 != 0.0)) && (noise_variable_414 == 0.0)) {
        let noise_metadata_schedule_647_body_57_e7670: f64 = (noise_variable_159 - noise_variable_158);
        let noise_metadata_schedule_647_body_57_e7672: f64 = (noise_metadata_schedule_647_body_57_e7670 * noise_variable_150);
        let noise_metadata_schedule_647_body_57_e7674: f64 = (noise_metadata_schedule_647_body_57_e7672 / noise_variable_232);
        (noise_metadata_schedule_647_body_57_e7674,)
    } else {
        (noise_variable_155,)
    }
};
                    noise_variable_155 = noise_metadata_schedule_647_body_57_e7676;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_58_e7696,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 == 0.0)) {
        let noise_metadata_schedule_647_body_58_e7688: f64 = (1.0 - noise_variable_146);
        let noise_metadata_schedule_647_body_58_e7692: f64 = (noise_variable_146 * params.p115);
        let noise_metadata_schedule_647_body_58_e7693: f64 = (1.0 + noise_metadata_schedule_647_body_58_e7692);
        let noise_metadata_schedule_647_body_58_e7694: f64 = (noise_metadata_schedule_647_body_58_e7688 / noise_metadata_schedule_647_body_58_e7693);
        (noise_metadata_schedule_647_body_58_e7694,)
    } else {
        (noise_variable_149,)
    }
};
                    noise_variable_149 = noise_metadata_schedule_647_body_58_e7696;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_59_e7712,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 == 0.0)) {
        let noise_metadata_schedule_647_body_59_e7709: f64 = (params.p115 * noise_variable_149);
        let noise_metadata_schedule_647_body_59_e7710: f64 = (1.0 + noise_metadata_schedule_647_body_59_e7709);
        (noise_metadata_schedule_647_body_59_e7710,)
    } else {
        (noise_variable_153,)
    }
};
                    noise_variable_153 = noise_metadata_schedule_647_body_59_e7712;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_60_e7736,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 == 0.0)) {
        let noise_metadata_schedule_647_body_60_e7724: f64 = (noise_variable_149 * noise_variable_149);
        let noise_metadata_schedule_647_body_60_e7728: f64 = (noise_variable_227 * 2.0);
        let noise_metadata_schedule_647_body_60_e7730: f64 = (noise_metadata_schedule_647_body_60_e7728 * noise_variable_149);
        let noise_metadata_schedule_647_body_60_e7731: f64 = (1.0 + noise_metadata_schedule_647_body_60_e7730);
        let noise_metadata_schedule_647_body_60_e7732: f64 = (noise_metadata_schedule_647_body_60_e7724 * noise_metadata_schedule_647_body_60_e7731);
        let noise_metadata_schedule_647_body_60_e7734: f64 = (noise_metadata_schedule_647_body_60_e7732 / noise_variable_153);
        (noise_metadata_schedule_647_body_60_e7734,)
    } else {
        (noise_variable_154,)
    }
};
                    noise_variable_154 = noise_metadata_schedule_647_body_60_e7736;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_61_e7757,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 == 0.0)) {
        let noise_metadata_schedule_647_body_61_e7747: f64 = (-noise_variable_147);
        let noise_metadata_schedule_647_body_61_e7749: f64 = (noise_metadata_schedule_647_body_61_e7747 * noise_variable_153);
        let noise_metadata_schedule_647_body_61_e7753: f64 = (noise_variable_146 * params.p115);
        let noise_metadata_schedule_647_body_61_e7754: f64 = (1.0 + noise_metadata_schedule_647_body_61_e7753);
        let noise_metadata_schedule_647_body_61_e7755: f64 = (noise_metadata_schedule_647_body_61_e7749 / noise_metadata_schedule_647_body_61_e7754);
        (noise_metadata_schedule_647_body_61_e7755,)
    } else {
        (noise_variable_150,)
    }
};
                    noise_variable_150 = noise_metadata_schedule_647_body_61_e7757;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_62_e7779,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) && (noise_variable_413 == 0.0)) {
        let noise_metadata_schedule_647_body_62_e7772: f64 = (noise_variable_153 * noise_variable_153);
        let noise_metadata_schedule_647_body_62_e7773: f64 = (1.0 / noise_metadata_schedule_647_body_62_e7772);
        let noise_metadata_schedule_647_body_62_e7774: f64 = (1.0 + noise_metadata_schedule_647_body_62_e7773);
        let noise_metadata_schedule_647_body_62_e7775: f64 = (noise_variable_149 * noise_metadata_schedule_647_body_62_e7774);
        let noise_metadata_schedule_647_body_62_e7777: f64 = (noise_metadata_schedule_647_body_62_e7775 * noise_variable_150);
        (noise_metadata_schedule_647_body_62_e7777,)
    } else {
        (noise_variable_155,)
    }
};
                    noise_variable_155 = noise_metadata_schedule_647_body_62_e7779;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_63_e7792,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) {
        let noise_metadata_schedule_647_body_63_e7788: f64 = (params.p73 * noise_variable_60);
        let noise_metadata_schedule_647_body_63_e7790: f64 = (noise_metadata_schedule_647_body_63_e7788 * noise_variable_110);
        (noise_metadata_schedule_647_body_63_e7790,)
    } else {
        (noise_variable_166,)
    }
};
                    noise_variable_166 = noise_metadata_schedule_647_body_63_e7792;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_64_e7803,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) {
        let noise_metadata_schedule_647_body_64_e7801: f64 = (noise_variable_166 * noise_variable_154);
        (noise_metadata_schedule_647_body_64_e7801,)
    } else {
        (noise_variable_167,)
    }
};
                    noise_variable_167 = noise_metadata_schedule_647_body_64_e7803;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_65_e7814,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) {
        let noise_metadata_schedule_647_body_65_e7812: f64 = (noise_variable_167 * noise_variable_217);
        (noise_metadata_schedule_647_body_65_e7812,)
    } else {
        (noise_variable_105,)
    }
};
                    noise_variable_105 = noise_metadata_schedule_647_body_65_e7814;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_66_e7835,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_412 == 0.0)) {
        let noise_metadata_schedule_647_body_66_e7824: f64 = (noise_variable_105 * noise_variable_112);
        let noise_metadata_schedule_647_body_66_e7826: f64 = (noise_metadata_schedule_647_body_66_e7824 * noise_variable_5);
        let noise_metadata_schedule_647_body_66_e7827: f64 = (noise_variable_167 + noise_metadata_schedule_647_body_66_e7826);
        let noise_metadata_schedule_647_body_66_e7830: f64 = (noise_variable_166 * noise_variable_217);
        let noise_metadata_schedule_647_body_66_e7832: f64 = (noise_metadata_schedule_647_body_66_e7830 * noise_variable_155);
        let noise_metadata_schedule_647_body_66_e7833: f64 = (noise_metadata_schedule_647_body_66_e7827 + noise_metadata_schedule_647_body_66_e7832);
        (noise_metadata_schedule_647_body_66_e7833,)
    } else {
        (noise_variable_106,)
    }
};
                    noise_variable_106 = noise_metadata_schedule_647_body_66_e7835;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_67_e7847,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_67_e7841: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_647_body_67_e7843: f64 = (noise_metadata_schedule_647_body_67_e7841 * noise_variable_101);
        let noise_metadata_schedule_647_body_67_e7845: f64 = (noise_metadata_schedule_647_body_67_e7843 * noise_variable_217);
        (noise_metadata_schedule_647_body_67_e7845,)
    } else {
        (noise_variable_103,)
    }
};
                    noise_variable_103 = noise_metadata_schedule_647_body_67_e7847;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_68_e7857,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_68_e7853: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_647_body_68_e7855: f64 = (noise_metadata_schedule_647_body_68_e7853 * noise_variable_102);
        (noise_metadata_schedule_647_body_68_e7855,)
    } else {
        (noise_variable_104,)
    }
};
                    noise_variable_104 = noise_metadata_schedule_647_body_68_e7857;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_69_e7867,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) {
        let noise_metadata_schedule_647_body_69_e7863: f64 = (noise_variable_99 * noise_variable_217);
        let noise_metadata_schedule_647_body_69_e7865: f64 = (noise_metadata_schedule_647_body_69_e7863 + noise_variable_103);
        (noise_metadata_schedule_647_body_69_e7865,)
    } else {
        (noise_variable_354,)
    }
};
                    noise_variable_354 = noise_metadata_schedule_647_body_69_e7867;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_70_e7870: f64 = if params.p0 >= 310.0 { 1.0 } else { 0.0 };
                    noise_variable_415 = noise_metadata_schedule_647_body_70_e7870;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_71_e7884,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_647_body_71_e7878: f64 = (noise_variable_355 + noise_variable_354);
        let noise_metadata_schedule_647_body_71_e7880: f64 = (noise_metadata_schedule_647_body_71_e7878 + noise_variable_97);
        let noise_metadata_schedule_647_body_71_e7882: f64 = (noise_metadata_schedule_647_body_71_e7880 + noise_variable_105);
        (noise_metadata_schedule_647_body_71_e7882,)
    } else {
        (noise_variable_355,)
    }
};
                    noise_variable_355 = noise_metadata_schedule_647_body_71_e7884;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_72_e7900,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_647_body_72_e7893: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_647_body_72_e7894: f64 = (noise_variable_219 + noise_metadata_schedule_647_body_72_e7893);
        let noise_metadata_schedule_647_body_72_e7896: f64 = (noise_metadata_schedule_647_body_72_e7894 + noise_variable_98);
        let noise_metadata_schedule_647_body_72_e7898: f64 = (noise_metadata_schedule_647_body_72_e7896 + noise_variable_106);
        (noise_metadata_schedule_647_body_72_e7898,)
    } else {
        (noise_variable_219,)
    }
};
                    noise_variable_219 = noise_metadata_schedule_647_body_72_e7900;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_73_e7920,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_647_body_73_e7909: f64 = (params.p5 * noise_variable_354);
        let noise_metadata_schedule_647_body_73_e7910: f64 = (noise_variable_358 + noise_metadata_schedule_647_body_73_e7909);
        let noise_metadata_schedule_647_body_73_e7913: f64 = (noise_variable_20 * noise_variable_97);
        let noise_metadata_schedule_647_body_73_e7914: f64 = (noise_metadata_schedule_647_body_73_e7910 + noise_metadata_schedule_647_body_73_e7913);
        let noise_metadata_schedule_647_body_73_e7917: f64 = (noise_variable_21 * noise_variable_105);
        let noise_metadata_schedule_647_body_73_e7918: f64 = (noise_metadata_schedule_647_body_73_e7914 + noise_metadata_schedule_647_body_73_e7917);
        (noise_metadata_schedule_647_body_73_e7918,)
    } else {
        (noise_variable_358,)
    }
};
                    noise_variable_358 = noise_metadata_schedule_647_body_73_e7920;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_74_e7942,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 != 0.0)) {
        let noise_metadata_schedule_647_body_74_e7930: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_647_body_74_e7931: f64 = (params.p5 * noise_metadata_schedule_647_body_74_e7930);
        let noise_metadata_schedule_647_body_74_e7932: f64 = (noise_variable_359 + noise_metadata_schedule_647_body_74_e7931);
        let noise_metadata_schedule_647_body_74_e7935: f64 = (noise_variable_20 * noise_variable_98);
        let noise_metadata_schedule_647_body_74_e7936: f64 = (noise_metadata_schedule_647_body_74_e7932 + noise_metadata_schedule_647_body_74_e7935);
        let noise_metadata_schedule_647_body_74_e7939: f64 = (noise_variable_21 * noise_variable_106);
        let noise_metadata_schedule_647_body_74_e7940: f64 = (noise_metadata_schedule_647_body_74_e7936 + noise_metadata_schedule_647_body_74_e7939);
        (noise_metadata_schedule_647_body_74_e7940,)
    } else {
        (noise_variable_359,)
    }
};
                    noise_variable_359 = noise_metadata_schedule_647_body_74_e7942;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_75_e7963,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 == 0.0)) {
        let noise_metadata_schedule_647_body_75_e7951: f64 = (noise_variable_19 * noise_variable_355);
        let noise_metadata_schedule_647_body_75_e7953: f64 = (noise_metadata_schedule_647_body_75_e7951 + noise_variable_354);
        let noise_metadata_schedule_647_body_75_e7956: f64 = (noise_variable_20 * noise_variable_97);
        let noise_metadata_schedule_647_body_75_e7957: f64 = (noise_metadata_schedule_647_body_75_e7953 + noise_metadata_schedule_647_body_75_e7956);
        let noise_metadata_schedule_647_body_75_e7960: f64 = (noise_variable_21 * noise_variable_105);
        let noise_metadata_schedule_647_body_75_e7961: f64 = (noise_metadata_schedule_647_body_75_e7957 + noise_metadata_schedule_647_body_75_e7960);
        (noise_metadata_schedule_647_body_75_e7961,)
    } else {
        (noise_variable_358,)
    }
};
                    noise_variable_358 = noise_metadata_schedule_647_body_75_e7963;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_76_e7978,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 == 0.0)) {
        let noise_metadata_schedule_647_body_76_e7972: f64 = (noise_variable_355 + noise_variable_354);
        let noise_metadata_schedule_647_body_76_e7974: f64 = (noise_metadata_schedule_647_body_76_e7972 + noise_variable_97);
        let noise_metadata_schedule_647_body_76_e7976: f64 = (noise_metadata_schedule_647_body_76_e7974 + noise_variable_105);
        (noise_metadata_schedule_647_body_76_e7976,)
    } else {
        (noise_variable_355,)
    }
};
                    noise_variable_355 = noise_metadata_schedule_647_body_76_e7978;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_77_e8001,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 == 0.0)) {
        let noise_metadata_schedule_647_body_77_e7987: f64 = (noise_variable_19 * noise_variable_219);
        let noise_metadata_schedule_647_body_77_e7990: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_647_body_77_e7991: f64 = (noise_metadata_schedule_647_body_77_e7987 + noise_metadata_schedule_647_body_77_e7990);
        let noise_metadata_schedule_647_body_77_e7994: f64 = (noise_variable_20 * noise_variable_98);
        let noise_metadata_schedule_647_body_77_e7995: f64 = (noise_metadata_schedule_647_body_77_e7991 + noise_metadata_schedule_647_body_77_e7994);
        let noise_metadata_schedule_647_body_77_e7998: f64 = (noise_variable_21 * noise_variable_106);
        let noise_metadata_schedule_647_body_77_e7999: f64 = (noise_metadata_schedule_647_body_77_e7995 + noise_metadata_schedule_647_body_77_e7998);
        (noise_metadata_schedule_647_body_77_e7999,)
    } else {
        (noise_variable_359,)
    }
};
                    noise_variable_359 = noise_metadata_schedule_647_body_77_e8001;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_78_e8018,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_409 != 0.0)) && (noise_variable_415 == 0.0)) {
        let noise_metadata_schedule_647_body_78_e8011: f64 = (noise_variable_100 + noise_variable_104);
        let noise_metadata_schedule_647_body_78_e8012: f64 = (noise_variable_219 + noise_metadata_schedule_647_body_78_e8011);
        let noise_metadata_schedule_647_body_78_e8014: f64 = (noise_metadata_schedule_647_body_78_e8012 + noise_variable_98);
        let noise_metadata_schedule_647_body_78_e8016: f64 = (noise_metadata_schedule_647_body_78_e8014 + noise_variable_106);
        (noise_metadata_schedule_647_body_78_e8016,)
    } else {
        (noise_variable_219,)
    }
};
                    noise_variable_219 = noise_metadata_schedule_647_body_78_e8018;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_79_e8026,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_79_e8022: f64 = (params.p7 * params.p85);
        let noise_metadata_schedule_647_body_79_e8024: f64 = (noise_metadata_schedule_647_body_79_e8022 * noise_variable_218);
        (noise_metadata_schedule_647_body_79_e8024,)
    } else {
        (noise_variable_360,)
    }
};
                    noise_variable_360 = noise_metadata_schedule_647_body_79_e8026;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_80_e8047,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_80_e8031: f64 = (noise_variable_352 + noise_variable_358);
        let noise_metadata_schedule_647_body_80_e8033: f64 = (noise_metadata_schedule_647_body_80_e8031 + noise_variable_360);
        let noise_metadata_schedule_647_body_80_e8034: f64 = (noise_variable_348 - noise_metadata_schedule_647_body_80_e8033);
        let noise_metadata_schedule_647_body_80_e8035: f64 = (-noise_metadata_schedule_647_body_80_e8034);
        let noise_metadata_schedule_647_body_80_e8039: f64 = (noise_variable_359 * noise_variable_217);
        let noise_metadata_schedule_647_body_80_e8041: f64 = (noise_metadata_schedule_647_body_80_e8039 + noise_variable_360);
        let noise_metadata_schedule_647_body_80_e8043: f64 = (noise_metadata_schedule_647_body_80_e8041 / noise_variable_348);
        let noise_metadata_schedule_647_body_80_e8044: f64 = (1.0 + noise_metadata_schedule_647_body_80_e8043);
        let noise_metadata_schedule_647_body_80_e8045: f64 = (noise_metadata_schedule_647_body_80_e8035 / noise_metadata_schedule_647_body_80_e8044);
        (noise_metadata_schedule_647_body_80_e8045,)
    } else {
        (noise_variable_349,)
    }
};
                    noise_variable_349 = noise_metadata_schedule_647_body_80_e8047;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_81_e8054,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_81_e8051: f64 = (0.3 * noise_variable_348);
        let noise_metadata_schedule_647_body_81_e8052: f64 = (noise_metadata_schedule_647_body_81_e8051).abs();
        (noise_metadata_schedule_647_body_81_e8052,)
    } else {
        (noise_variable_407,)
    }
};
                    noise_variable_407 = noise_metadata_schedule_647_body_81_e8054;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_82_e8056: f64 = (noise_variable_349).abs();
                    let noise_metadata_schedule_647_body_82_e8058: f64 = if noise_metadata_schedule_647_body_82_e8056 > noise_variable_407 { 1.0 } else { 0.0 };
                    noise_variable_416 = noise_metadata_schedule_647_body_82_e8058;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let noise_metadata_schedule_647_body_83_e8061: f64 = if noise_variable_349 >= 0.0 { 1.0 } else { 0.0 };
                    noise_variable_417 = noise_metadata_schedule_647_body_83_e8061;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_84_e8069,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_416 != 0.0)) && (noise_variable_417 != 0.0)) {
        (noise_variable_407,)
    } else {
        (noise_variable_349,)
    }
};
                    noise_variable_349 = noise_metadata_schedule_647_body_84_e8069;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_85_e8079,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_416 != 0.0)) && (noise_variable_417 == 0.0)) {
        let noise_metadata_schedule_647_body_85_e8077: f64 = (-noise_variable_407);
        (noise_metadata_schedule_647_body_85_e8077,)
    } else {
        (noise_variable_349,)
    }
};
                    noise_variable_349 = noise_metadata_schedule_647_body_85_e8079;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_86_e8085,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_86_e8083: f64 = (noise_variable_348 + noise_variable_349);
        (noise_metadata_schedule_647_body_86_e8083,)
    } else {
        (noise_variable_348,)
    }
};
                    noise_variable_348 = noise_metadata_schedule_647_body_86_e8085;
                }
                if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
                    let (noise_metadata_schedule_647_body_87_e8091,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_647_body_87_e8089: f64 = (noise_variable_224 + 1.0);
        (noise_metadata_schedule_647_body_87_e8089,)
    } else {
        (noise_variable_224,)
    }
};
                    noise_variable_224 = noise_metadata_schedule_647_body_87_e8091;
                }
                noise_metadata_schedule_647_iterations += 1;
                assert!(noise_metadata_schedule_647_iterations <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A noise evaluation loop exceeded iteration limit");
            }
        }
        if matches!(source_index, 1 | 10 | 12 | 16 | 17) {
            let (noise_metadata_schedule_648_e8097,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_648_e8095: f64 = (noise_variable_350 / noise_variable_348);
        (noise_metadata_schedule_648_e8095,)
    } else {
        (noise_variable_217,)
    }
};
            noise_variable_217 = noise_metadata_schedule_648_e8097;
        }
        if matches!(source_index, 16 | 17) {
            let (noise_metadata_schedule_649_e8103,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_649_e8101: f64 = (noise_variable_351 / noise_variable_348);
        (noise_metadata_schedule_649_e8101,)
    } else {
        (noise_variable_218,)
    }
};
            noise_variable_218 = noise_metadata_schedule_649_e8103;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_651_e8113,) = {
    if (noise_variable_406 != 0.0) {
        let noise_metadata_schedule_651_e8111: f64 = (noise_variable_357 * noise_variable_217);
        (noise_metadata_schedule_651_e8111,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_651_e8113;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_657_e8154,) = {
    if (noise_variable_406 != 0.0) {
        (0.0,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_657_e8154;
        }
        if matches!(source_index, 1 | 12) {
            let noise_metadata_schedule_658_e8158: f64 = (1e-6 * noise_variable_362);
            let noise_metadata_schedule_658_e8163: f64 = if ((noise_variable_217 >= noise_metadata_schedule_658_e8158) || (params.p0 >= 320.0)) { 1.0 } else { 0.0 };
            noise_variable_419 = noise_metadata_schedule_658_e8163;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_659_e8171,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_659_e8169: f64 = (noise_variable_217 / noise_variable_362);
        (noise_metadata_schedule_659_e8169,)
    } else {
        (noise_variable_96,)
    }
};
            noise_variable_96 = noise_metadata_schedule_659_e8171;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_660_e8183,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_660_e8178: f64 = (noise_variable_96).ln();
        let noise_metadata_schedule_660_e8179: f64 = (params.p70 * noise_metadata_schedule_660_e8178);
        let noise_metadata_schedule_660_e8180: f64 = (noise_metadata_schedule_660_e8179).exp();
        let noise_metadata_schedule_660_e8181: f64 = (noise_variable_61 * noise_metadata_schedule_660_e8180);
        (noise_metadata_schedule_660_e8181,)
    } else {
        (noise_variable_98,)
    }
};
            noise_variable_98 = noise_metadata_schedule_660_e8183;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_661_e8195,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_661_e8189: f64 = (noise_variable_98 * noise_variable_217);
        let noise_metadata_schedule_661_e8192: f64 = (1.0 + params.p70);
        let noise_metadata_schedule_661_e8193: f64 = (noise_metadata_schedule_661_e8189 / noise_metadata_schedule_661_e8192);
        (noise_metadata_schedule_661_e8193,)
    } else {
        (noise_variable_97,)
    }
};
            noise_variable_97 = noise_metadata_schedule_661_e8195;
        }
        if matches!(source_index, 1 | 12) {
            let noise_metadata_schedule_662_e8200: f64 = (params.p75 / params.p74);
            let noise_metadata_schedule_662_e8201: f64 = (0.05 * noise_metadata_schedule_662_e8200);
            let noise_metadata_schedule_662_e8202: f64 = if params.p83 < noise_metadata_schedule_662_e8201 { 1.0 } else { 0.0 };
            noise_variable_420 = noise_metadata_schedule_662_e8202;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_663_e8210,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_420 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_663_e8210;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_665_e8231,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_420 == 0.0)) {
        let noise_metadata_schedule_665_e8227: f64 = (noise_variable_217 - noise_variable_362);
        let noise_metadata_schedule_665_e8229: f64 = (noise_metadata_schedule_665_e8227 / params.p83);
        (noise_metadata_schedule_665_e8229,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_665_e8231;
        }
        if matches!(source_index, 1 | 12) {
            let noise_metadata_schedule_666_e8234: f64 = (-10000000000.0);
            let noise_metadata_schedule_666_e8235: f64 = if noise_variable_107 < noise_metadata_schedule_666_e8234 { 1.0 } else { 0.0 };
            noise_variable_421 = noise_metadata_schedule_666_e8235;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_667_e8247,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_420 == 0.0)) && (noise_variable_421 != 0.0)) {
        let noise_metadata_schedule_667_e8245: f64 = (-10000000000.0);
        (noise_metadata_schedule_667_e8245,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_667_e8247;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_668_e8261,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_420 == 0.0)) {
        let noise_metadata_schedule_668_e8256: f64 = (noise_variable_107 * noise_variable_107);
        let noise_metadata_schedule_668_e8258: f64 = (noise_metadata_schedule_668_e8256 + params.p84);
        let noise_metadata_schedule_668_e8259: f64 = (noise_metadata_schedule_668_e8258).sqrt();
        (noise_metadata_schedule_668_e8259,)
    } else {
        (noise_variable_95,)
    }
};
            noise_variable_95 = noise_metadata_schedule_668_e8261;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_669_e8278,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_420 == 0.0)) {
        let noise_metadata_schedule_669_e8270: f64 = (-2.0);
        let noise_metadata_schedule_669_e8273: f64 = (noise_variable_107 + noise_variable_95);
        let noise_metadata_schedule_669_e8274: f64 = (noise_metadata_schedule_669_e8270 / noise_metadata_schedule_669_e8273);
        let noise_metadata_schedule_669_e8275: f64 = (noise_metadata_schedule_669_e8274).exp();
        let noise_metadata_schedule_669_e8276: f64 = (params.p82 * noise_metadata_schedule_669_e8275);
        (noise_metadata_schedule_669_e8276,)
    } else {
        (noise_variable_111,)
    }
};
            noise_variable_111 = noise_metadata_schedule_669_e8278;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_671_e8314,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_671_e8303: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_671_e8305: f64 = (noise_metadata_schedule_671_e8303 * noise_variable_60);
        let noise_metadata_schedule_671_e8308: f64 = (noise_variable_111 * noise_variable_5);
        let noise_metadata_schedule_671_e8309: f64 = (noise_metadata_schedule_671_e8308).exp();
        let noise_metadata_schedule_671_e8311: f64 = (noise_metadata_schedule_671_e8309 - 1.0);
        let noise_metadata_schedule_671_e8312: f64 = (noise_metadata_schedule_671_e8305 * noise_metadata_schedule_671_e8311);
        (noise_metadata_schedule_671_e8312,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_671_e8314;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_673_e8347,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_673_e8344: f64 = (1.0 / noise_variable_96);
        let noise_metadata_schedule_673_e8345: f64 = (1.0 - noise_metadata_schedule_673_e8344);
        (noise_metadata_schedule_673_e8345,)
    } else {
        (noise_variable_108,)
    }
};
            noise_variable_108 = noise_metadata_schedule_673_e8347;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_674_e8367,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_674_e8354: f64 = (noise_variable_108 * noise_variable_108);
        let noise_metadata_schedule_674_e8356: f64 = (noise_metadata_schedule_674_e8354 + params.p72);
        let noise_metadata_schedule_674_e8357: f64 = (noise_metadata_schedule_674_e8356).sqrt();
        let noise_metadata_schedule_674_e8358: f64 = (noise_variable_108 + noise_metadata_schedule_674_e8357);
        let noise_metadata_schedule_674_e8362: f64 = (1.0 + params.p72);
        let noise_metadata_schedule_674_e8363: f64 = (noise_metadata_schedule_674_e8362).sqrt();
        let noise_metadata_schedule_674_e8364: f64 = (1.0 + noise_metadata_schedule_674_e8363);
        let noise_metadata_schedule_674_e8365: f64 = (noise_metadata_schedule_674_e8358 / noise_metadata_schedule_674_e8364);
        (noise_metadata_schedule_674_e8365,)
    } else {
        (noise_variable_109,)
    }
};
            noise_variable_109 = noise_metadata_schedule_674_e8367;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_675_e8378,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_675_e8373: f64 = (noise_variable_111 - params.p82);
        let noise_metadata_schedule_675_e8375: f64 = (noise_metadata_schedule_675_e8373 * noise_variable_5);
        let noise_metadata_schedule_675_e8376: f64 = (noise_metadata_schedule_675_e8375).exp();
        (noise_metadata_schedule_675_e8376,)
    } else {
        (noise_variable_110,)
    }
};
            noise_variable_110 = noise_metadata_schedule_675_e8378;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_676_e8390,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_676_e8384: f64 = (noise_variable_60 * noise_variable_109);
        let noise_metadata_schedule_676_e8386: f64 = (noise_metadata_schedule_676_e8384 * noise_variable_109);
        let noise_metadata_schedule_676_e8388: f64 = (noise_metadata_schedule_676_e8386 * noise_variable_110);
        (noise_metadata_schedule_676_e8388,)
    } else {
        (noise_variable_101,)
    }
};
            noise_variable_101 = noise_metadata_schedule_676_e8390;
        }
        if matches!(source_index, 1 | 12) {
            let noise_metadata_schedule_678_e8425: f64 = (noise_variable_109 * params.p115);
            let noise_metadata_schedule_678_e8431: f64 = (noise_variable_109 * params.p116);
            let noise_metadata_schedule_678_e8434: f64 = if ((((params.p115 < 0.01) && (params.p116 < 0.01)) && (noise_metadata_schedule_678_e8425 < 0.005)) && (noise_metadata_schedule_678_e8431 < 0.005)) { 1.0 } else { 0.0 };
            noise_variable_422 = noise_metadata_schedule_678_e8434;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_679_e8446,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 != 0.0)) {
        let noise_metadata_schedule_679_e8442: f64 = (params.p73 * noise_variable_101);
        let noise_metadata_schedule_679_e8444: f64 = (noise_metadata_schedule_679_e8442 * noise_variable_217);
        (noise_metadata_schedule_679_e8444,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_679_e8446;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_681_e8467,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) {
        let noise_metadata_schedule_681_e8465: f64 = (1.0 - noise_variable_109);
        (noise_metadata_schedule_681_e8465,)
    } else {
        (noise_variable_146,)
    }
};
            noise_variable_146 = noise_metadata_schedule_681_e8467;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_683_e8493: f64 = (noise_variable_232).abs();
            let noise_metadata_schedule_683_e8495: f64 = if noise_metadata_schedule_683_e8493 > 0.001 { 1.0 } else { 0.0 };
            noise_variable_423 = noise_metadata_schedule_683_e8495;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_684_e8511,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) {
        let noise_metadata_schedule_684_e8506: f64 = (noise_variable_146 - 1.0);
        let noise_metadata_schedule_684_e8508: f64 = (noise_metadata_schedule_684_e8506 * noise_variable_231);
        let noise_metadata_schedule_684_e8509: f64 = (noise_metadata_schedule_684_e8508).exp();
        (noise_metadata_schedule_684_e8509,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_684_e8511;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_685_e8514: f64 = if noise_variable_229 < 0.01 { 1.0 } else { 0.0 };
            noise_variable_424 = noise_metadata_schedule_685_e8514;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_686_e8533,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 != 0.0)) {
        let noise_metadata_schedule_686_e8527: f64 = (1.0 - noise_variable_151);
        let noise_metadata_schedule_686_e8530: f64 = (noise_variable_151 * noise_variable_230);
        let noise_metadata_schedule_686_e8531: f64 = (noise_metadata_schedule_686_e8527 / noise_metadata_schedule_686_e8530);
        (noise_metadata_schedule_686_e8531,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_686_e8533;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_687_e8550,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 != 0.0)) {
        let noise_metadata_schedule_687_e8547: f64 = (noise_variable_230 * noise_variable_149);
        let noise_metadata_schedule_687_e8548: f64 = (1.0 + noise_metadata_schedule_687_e8547);
        (noise_metadata_schedule_687_e8548,)
    } else {
        (noise_variable_148,)
    }
};
            noise_variable_148 = noise_metadata_schedule_687_e8550;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_688_e8584,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 != 0.0)) {
        let noise_metadata_schedule_688_e8564: f64 = (noise_variable_230 * noise_variable_149);
        let noise_metadata_schedule_688_e8568: f64 = (0.25 * noise_variable_230);
        let noise_metadata_schedule_688_e8570: f64 = (noise_metadata_schedule_688_e8568 * noise_variable_149);
        let noise_metadata_schedule_688_e8571: f64 = (0.5 + noise_metadata_schedule_688_e8570);
        let noise_metadata_schedule_688_e8572: f64 = (noise_metadata_schedule_688_e8564 * noise_metadata_schedule_688_e8571);
        let noise_metadata_schedule_688_e8575: f64 = (noise_variable_148).ln();
        let noise_metadata_schedule_688_e8576: f64 = (0.5 * noise_metadata_schedule_688_e8575);
        let noise_metadata_schedule_688_e8577: f64 = (noise_metadata_schedule_688_e8572 - noise_metadata_schedule_688_e8576);
        let noise_metadata_schedule_688_e8578: f64 = (2.0 * noise_metadata_schedule_688_e8577);
        let noise_metadata_schedule_688_e8580: f64 = (noise_metadata_schedule_688_e8578 / noise_variable_230);
        let noise_metadata_schedule_688_e8582: f64 = (noise_metadata_schedule_688_e8580 / noise_variable_230);
        (noise_metadata_schedule_688_e8582,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_688_e8584;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_691_e8643,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_691_e8640: f64 = (noise_variable_151 * params.p115);
        let noise_metadata_schedule_691_e8641: f64 = (params.p116 - noise_metadata_schedule_691_e8640);
        (noise_metadata_schedule_691_e8641,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_691_e8643;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_692_e8661,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_692_e8657: f64 = (noise_variable_151 - 1.0);
        let noise_metadata_schedule_692_e8659: f64 = (noise_metadata_schedule_692_e8657 / noise_variable_152);
        (noise_metadata_schedule_692_e8659,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_692_e8661;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_693_e8679,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_693_e8676: f64 = (params.p116 * noise_variable_149);
        let noise_metadata_schedule_693_e8677: f64 = (1.0 + noise_metadata_schedule_693_e8676);
        (noise_metadata_schedule_693_e8677,)
    } else {
        (noise_variable_160,)
    }
};
            noise_variable_160 = noise_metadata_schedule_693_e8679;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_694_e8694,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_694_e8692: f64 = (noise_variable_160).ln();
        (noise_metadata_schedule_694_e8692,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_694_e8694;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_695_e8710,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_695_e8708: f64 = (noise_variable_227 * noise_variable_226);
        (noise_metadata_schedule_695_e8708,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_695_e8710;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_696_e8738,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_696_e8725: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_696_e8726: f64 = (noise_variable_161 * noise_metadata_schedule_696_e8725);
        let noise_metadata_schedule_696_e8728: f64 = (noise_metadata_schedule_696_e8726 * noise_variable_226);
        let noise_metadata_schedule_696_e8732: f64 = (noise_variable_227 * noise_variable_149);
        let noise_metadata_schedule_696_e8733: f64 = (noise_variable_162 + noise_metadata_schedule_696_e8732);
        let noise_metadata_schedule_696_e8735: f64 = (noise_metadata_schedule_696_e8733 * noise_variable_149);
        let noise_metadata_schedule_696_e8736: f64 = (noise_metadata_schedule_696_e8728 + noise_metadata_schedule_696_e8735);
        (noise_metadata_schedule_696_e8736,)
    } else {
        (noise_variable_157,)
    }
};
            noise_variable_157 = noise_metadata_schedule_696_e8738;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_698_e8782,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_698_e8779: f64 = (params.p115 * noise_variable_149);
        let noise_metadata_schedule_698_e8780: f64 = (1.0 + noise_metadata_schedule_698_e8779);
        (noise_metadata_schedule_698_e8780,)
    } else {
        (noise_variable_160,)
    }
};
            noise_variable_160 = noise_metadata_schedule_698_e8782;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_699_e8797,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_699_e8795: f64 = (noise_variable_160).ln();
        (noise_metadata_schedule_699_e8795,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_699_e8797;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_700_e8813,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_700_e8811: f64 = (noise_variable_228 * noise_variable_225);
        (noise_metadata_schedule_700_e8811,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_700_e8813;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_701_e8841,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_701_e8828: f64 = (0.5 - noise_variable_162);
        let noise_metadata_schedule_701_e8829: f64 = (noise_variable_161 * noise_metadata_schedule_701_e8828);
        let noise_metadata_schedule_701_e8831: f64 = (noise_metadata_schedule_701_e8829 * noise_variable_225);
        let noise_metadata_schedule_701_e8835: f64 = (noise_variable_228 * noise_variable_149);
        let noise_metadata_schedule_701_e8836: f64 = (noise_variable_162 + noise_metadata_schedule_701_e8835);
        let noise_metadata_schedule_701_e8838: f64 = (noise_metadata_schedule_701_e8836 * noise_variable_149);
        let noise_metadata_schedule_701_e8839: f64 = (noise_metadata_schedule_701_e8831 + noise_metadata_schedule_701_e8838);
        (noise_metadata_schedule_701_e8839,)
    } else {
        (noise_variable_156,)
    }
};
            noise_variable_156 = noise_metadata_schedule_701_e8841;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_703_e8885,) = {
    if (((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 != 0.0)) && (noise_variable_424 == 0.0)) {
        let noise_metadata_schedule_703_e8881: f64 = (noise_variable_157 - noise_variable_156);
        let noise_metadata_schedule_703_e8883: f64 = (noise_metadata_schedule_703_e8881 / noise_variable_232);
        (noise_metadata_schedule_703_e8883,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_703_e8885;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_706_e8952,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 == 0.0)) {
        let noise_metadata_schedule_706_e8944: f64 = (1.0 - noise_variable_146);
        let noise_metadata_schedule_706_e8948: f64 = (noise_variable_146 * params.p115);
        let noise_metadata_schedule_706_e8949: f64 = (1.0 + noise_metadata_schedule_706_e8948);
        let noise_metadata_schedule_706_e8950: f64 = (noise_metadata_schedule_706_e8944 / noise_metadata_schedule_706_e8949);
        (noise_metadata_schedule_706_e8950,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_706_e8952;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_707_e8968,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 == 0.0)) {
        let noise_metadata_schedule_707_e8965: f64 = (params.p115 * noise_variable_149);
        let noise_metadata_schedule_707_e8966: f64 = (1.0 + noise_metadata_schedule_707_e8965);
        (noise_metadata_schedule_707_e8966,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_707_e8968;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_708_e8992,) = {
    if ((((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) && (noise_variable_423 == 0.0)) {
        let noise_metadata_schedule_708_e8980: f64 = (noise_variable_149 * noise_variable_149);
        let noise_metadata_schedule_708_e8984: f64 = (noise_variable_227 * 2.0);
        let noise_metadata_schedule_708_e8986: f64 = (noise_metadata_schedule_708_e8984 * noise_variable_149);
        let noise_metadata_schedule_708_e8987: f64 = (1.0 + noise_metadata_schedule_708_e8986);
        let noise_metadata_schedule_708_e8988: f64 = (noise_metadata_schedule_708_e8980 * noise_metadata_schedule_708_e8987);
        let noise_metadata_schedule_708_e8990: f64 = (noise_metadata_schedule_708_e8988 / noise_variable_153);
        (noise_metadata_schedule_708_e8990,)
    } else {
        (noise_variable_154,)
    }
};
            noise_variable_154 = noise_metadata_schedule_708_e8992;
        }
        if matches!(source_index, 1 | 12) {
            let (noise_metadata_schedule_711_e9048,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) {
        let noise_metadata_schedule_711_e9044: f64 = (params.p73 * noise_variable_60);
        let noise_metadata_schedule_711_e9046: f64 = (noise_metadata_schedule_711_e9044 * noise_variable_110);
        (noise_metadata_schedule_711_e9046,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_711_e9048;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_712_e9059,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) {
        let noise_metadata_schedule_712_e9057: f64 = (noise_variable_166 * noise_variable_154);
        (noise_metadata_schedule_712_e9057,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_712_e9059;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_713_e9070,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_422 == 0.0)) {
        let noise_metadata_schedule_713_e9068: f64 = (noise_variable_167 * noise_variable_217);
        (noise_metadata_schedule_713_e9068,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_713_e9070;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_715_e9103,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_715_e9097: f64 = (1.0 - params.p73);
        let noise_metadata_schedule_715_e9099: f64 = (noise_metadata_schedule_715_e9097 * noise_variable_101);
        let noise_metadata_schedule_715_e9101: f64 = (noise_metadata_schedule_715_e9099 * noise_variable_217);
        (noise_metadata_schedule_715_e9101,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_715_e9103;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_717_e9123,) = {
    if ((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) {
        let noise_metadata_schedule_717_e9119: f64 = (noise_variable_99 * noise_variable_217);
        let noise_metadata_schedule_717_e9121: f64 = (noise_metadata_schedule_717_e9119 + noise_variable_103);
        (noise_metadata_schedule_717_e9121,)
    } else {
        (noise_variable_354,)
    }
};
            noise_variable_354 = noise_metadata_schedule_717_e9123;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_718_e9126: f64 = if params.p0 >= 310.0 { 1.0 } else { 0.0 };
            noise_variable_425 = noise_metadata_schedule_718_e9126;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_719_e9140,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_425 != 0.0)) {
        let noise_metadata_schedule_719_e9134: f64 = (noise_variable_355 + noise_variable_354);
        let noise_metadata_schedule_719_e9136: f64 = (noise_metadata_schedule_719_e9134 + noise_variable_97);
        let noise_metadata_schedule_719_e9138: f64 = (noise_metadata_schedule_719_e9136 + noise_variable_105);
        (noise_metadata_schedule_719_e9138,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_719_e9140;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_724_e9234,) = {
    if (((noise_variable_406 != 0.0) && (noise_variable_419 != 0.0)) && (noise_variable_425 == 0.0)) {
        let noise_metadata_schedule_724_e9228: f64 = (noise_variable_355 + noise_variable_354);
        let noise_metadata_schedule_724_e9230: f64 = (noise_metadata_schedule_724_e9228 + noise_variable_97);
        let noise_metadata_schedule_724_e9232: f64 = (noise_metadata_schedule_724_e9230 + noise_variable_105);
        (noise_metadata_schedule_724_e9232,)
    } else {
        (noise_variable_355,)
    }
};
            noise_variable_355 = noise_metadata_schedule_724_e9234;
        }
        if matches!(source_index, 16 | 17) {
            let noise_metadata_schedule_728_e9283: f64 = (noise_variable_217 - noise_variable_218);
            noise_variable_184 = noise_metadata_schedule_728_e9283;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let noise_metadata_schedule_735_e9310: f64 = if params.p23 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_426 = noise_metadata_schedule_735_e9310;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_736_e9318,) = {
    if (noise_variable_426 != 0.0) {
        let noise_metadata_schedule_736_e9315: f64 = (params.p24 * noise_variable_4);
        let noise_metadata_schedule_736_e9316: f64 = (noise_variable_203 / noise_metadata_schedule_736_e9315);
        (noise_metadata_schedule_736_e9316,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_736_e9318;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let noise_metadata_schedule_737_e9321: f64 = if noise_variable_93 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_427 = noise_metadata_schedule_737_e9321;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_738_e9331,) = {
    if ((noise_variable_426 != 0.0) && (noise_variable_427 != 0.0)) {
        let noise_metadata_schedule_738_e9328: f64 = (noise_variable_93 - 80.0);
        let noise_metadata_schedule_738_e9329: f64 = (1.0 + noise_metadata_schedule_738_e9328);
        (noise_metadata_schedule_738_e9329,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_738_e9331;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_739_e9337,) = {
    if ((noise_variable_426 != 0.0) && (noise_variable_427 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_739_e9337;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 11 | 13 | 14) {
            let (noise_metadata_schedule_740_e9344,) = {
    if ((noise_variable_426 != 0.0) && (noise_variable_427 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_740_e9344;
        }
        if matches!(source_index, 11) {
            let (noise_metadata_schedule_741_e9355,) = {
    if (noise_variable_426 != 0.0) {
        let noise_metadata_schedule_741_e9349: f64 = { let limexp_arg = noise_variable_93; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_741_e9350: f64 = (noise_variable_94 * noise_metadata_schedule_741_e9349);
        let noise_metadata_schedule_741_e9352: f64 = (noise_metadata_schedule_741_e9350 - 1.0);
        let noise_metadata_schedule_741_e9353: f64 = (noise_variable_32 * noise_metadata_schedule_741_e9352);
        (noise_metadata_schedule_741_e9353,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_741_e9355;
        }
        if matches!(source_index, 11) {
            let (noise_metadata_schedule_742_e9360,) = {
    if (noise_variable_426 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_742_e9360;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_743_e9367: f64 = if ((params.p37 > 0.0) && (noise_variable_203 < 0.0)) { 1.0 } else { 0.0 };
            noise_variable_428 = noise_metadata_schedule_743_e9367;
        }
        if matches!(source_index, 12) {
            let noise_metadata_schedule_744_e9374: f64 = if ((noise_variable_33 > 0.0) && (noise_variable_34 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_429 = noise_metadata_schedule_744_e9374;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_745_e9390,) = {
    if ((noise_variable_428 != 0.0) && (noise_variable_429 != 0.0)) {
        let noise_metadata_schedule_745_e9380: f64 = (1.0 / params.p49);
        let noise_metadata_schedule_745_e9382: f64 = (noise_metadata_schedule_745_e9380 - 1.0);
        let noise_metadata_schedule_745_e9385: f64 = (noise_variable_210 / noise_variable_33);
        let noise_metadata_schedule_745_e9386: f64 = (noise_metadata_schedule_745_e9385).ln();
        let noise_metadata_schedule_745_e9387: f64 = (noise_metadata_schedule_745_e9382 * noise_metadata_schedule_745_e9386);
        let noise_metadata_schedule_745_e9388: f64 = (noise_metadata_schedule_745_e9387).exp();
        (noise_metadata_schedule_745_e9388,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_745_e9390;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_746_e9403,) = {
    if ((noise_variable_428 != 0.0) && (noise_variable_429 != 0.0)) {
        let noise_metadata_schedule_746_e9395: f64 = (-noise_variable_67);
        let noise_metadata_schedule_746_e9397: f64 = (noise_metadata_schedule_746_e9395 * noise_variable_203);
        let noise_metadata_schedule_746_e9400: f64 = (noise_variable_34 * noise_variable_168);
        let noise_metadata_schedule_746_e9401: f64 = (noise_metadata_schedule_746_e9397 / noise_metadata_schedule_746_e9400);
        (noise_metadata_schedule_746_e9401,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_746_e9403;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_747_e9415,) = {
    if ((noise_variable_428 != 0.0) && (noise_variable_429 != 0.0)) {
        let noise_metadata_schedule_747_e9409: f64 = (-noise_variable_68);
        let noise_metadata_schedule_747_e9411: f64 = (noise_metadata_schedule_747_e9409 * noise_variable_168);
        let noise_metadata_schedule_747_e9412: f64 = (noise_metadata_schedule_747_e9411).exp();
        let noise_metadata_schedule_747_e9413: f64 = (noise_variable_166 * noise_metadata_schedule_747_e9412);
        (noise_metadata_schedule_747_e9413,)
    } else {
        (noise_variable_193,)
    }
};
            noise_variable_193 = noise_metadata_schedule_747_e9415;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_748_e9422,) = {
    if ((noise_variable_428 != 0.0) && (noise_variable_429 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_193,)
    }
};
            noise_variable_193 = noise_metadata_schedule_748_e9422;
        }
        if matches!(source_index, 12) {
            let (noise_metadata_schedule_749_e9427,) = {
    if (noise_variable_428 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_193,)
    }
};
            noise_variable_193 = noise_metadata_schedule_749_e9427;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_750_e9430: f64 = if noise_variable_243 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_430 = noise_metadata_schedule_750_e9430;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_751_e9436,) = {
    if (noise_variable_430 != 0.0) {
        let noise_metadata_schedule_751_e9434: f64 = (noise_variable_34 - noise_variable_203);
        (noise_metadata_schedule_751_e9434,)
    } else {
        (noise_variable_431,)
    }
};
            noise_variable_431 = noise_metadata_schedule_751_e9436;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_752_e9439: f64 = if noise_variable_431 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_437 = noise_metadata_schedule_752_e9439;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_753_e9442: f64 = if params.p35 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_438 = noise_metadata_schedule_753_e9442;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_754_e9450,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_438 != 0.0)) {
        (0.1,)
    } else {
        (noise_variable_441,)
    }
};
            noise_variable_441 = noise_metadata_schedule_754_e9450;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_755_e9460,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_438 != 0.0)) {
        let noise_metadata_schedule_755_e9458: f64 = (noise_variable_210 / noise_variable_33);
        (noise_metadata_schedule_755_e9458,)
    } else {
        (noise_variable_440,)
    }
};
            noise_variable_440 = noise_metadata_schedule_755_e9460;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_756_e9476,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_438 != 0.0)) {
        let noise_metadata_schedule_756_e9468: f64 = (params.p35 * noise_variable_55);
        let noise_metadata_schedule_756_e9470: f64 = (noise_metadata_schedule_756_e9468 * noise_variable_54);
        let noise_metadata_schedule_756_e9473: f64 = (params.p36 * noise_variable_217);
        let noise_metadata_schedule_756_e9474: f64 = (noise_metadata_schedule_756_e9470 + noise_metadata_schedule_756_e9473);
        (noise_metadata_schedule_756_e9474,)
    } else {
        (noise_variable_439,)
    }
};
            noise_variable_439 = noise_metadata_schedule_756_e9476;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_757_e9504,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_438 != 0.0)) {
        let noise_metadata_schedule_757_e9485: f64 = (noise_variable_440 / noise_variable_441);
        let noise_metadata_schedule_757_e9486: f64 = (noise_metadata_schedule_757_e9485).exp();
        let noise_metadata_schedule_757_e9488: f64 = (noise_metadata_schedule_757_e9486 - 2.0);
        let noise_metadata_schedule_757_e9493: f64 = (noise_variable_217 / noise_variable_439);
        let noise_metadata_schedule_757_e9494: f64 = (1.0 - noise_metadata_schedule_757_e9493);
        let noise_metadata_schedule_757_e9496: f64 = (noise_metadata_schedule_757_e9494 / noise_variable_441);
        let noise_metadata_schedule_757_e9497: f64 = (noise_metadata_schedule_757_e9496).cosh();
        let noise_metadata_schedule_757_e9498: f64 = (2.0 * noise_metadata_schedule_757_e9497);
        let noise_metadata_schedule_757_e9499: f64 = (noise_metadata_schedule_757_e9488 + noise_metadata_schedule_757_e9498);
        let noise_metadata_schedule_757_e9500: f64 = (noise_metadata_schedule_757_e9499).ln();
        let noise_metadata_schedule_757_e9501: f64 = (noise_variable_441 * noise_metadata_schedule_757_e9500);
        let noise_metadata_schedule_757_e9502: f64 = (noise_metadata_schedule_757_e9501).sqrt();
        (noise_metadata_schedule_757_e9502,)
    } else {
        (noise_variable_436,)
    }
};
            noise_variable_436 = noise_metadata_schedule_757_e9504;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_758_e9513,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_438 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_436,)
    }
};
            noise_variable_436 = noise_metadata_schedule_758_e9513;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_759_e9521,) = {
    if ((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) {
        let noise_metadata_schedule_759_e9519: f64 = (noise_variable_62 / noise_variable_210);
        (noise_metadata_schedule_759_e9519,)
    } else {
        (noise_variable_432,)
    }
};
            noise_variable_432 = noise_metadata_schedule_759_e9521;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_760_e9529,) = {
    if ((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) {
        let noise_metadata_schedule_760_e9527: f64 = (noise_variable_62 / noise_variable_33);
        (noise_metadata_schedule_760_e9527,)
    } else {
        (noise_variable_433,)
    }
};
            noise_variable_433 = noise_metadata_schedule_760_e9529;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_761_e9532: f64 = if noise_variable_431 > noise_variable_433 { 1.0 } else { 0.0 };
            noise_variable_442 = noise_metadata_schedule_761_e9532;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_762_e9548,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_442 != 0.0)) {
        let noise_metadata_schedule_762_e9540: f64 = (-noise_variable_432);
        let noise_metadata_schedule_762_e9543: f64 = (noise_variable_433 * noise_variable_436);
        let noise_metadata_schedule_762_e9544: f64 = (noise_metadata_schedule_762_e9540 / noise_metadata_schedule_762_e9543);
        let noise_metadata_schedule_762_e9545: f64 = (noise_metadata_schedule_762_e9544).exp();
        let noise_metadata_schedule_762_e9546: f64 = (noise_variable_63 * noise_metadata_schedule_762_e9545);
        (noise_metadata_schedule_762_e9546,)
    } else {
        (noise_variable_434,)
    }
};
            noise_variable_434 = noise_metadata_schedule_762_e9548;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_763_e9568,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_442 != 0.0)) {
        let noise_metadata_schedule_763_e9559: f64 = (noise_variable_432 / noise_variable_433);
        let noise_metadata_schedule_763_e9560: f64 = (1.0 + noise_metadata_schedule_763_e9559);
        let noise_metadata_schedule_763_e9563: f64 = (noise_variable_431 - noise_variable_433);
        let noise_metadata_schedule_763_e9564: f64 = (noise_metadata_schedule_763_e9560 * noise_metadata_schedule_763_e9563);
        let noise_metadata_schedule_763_e9565: f64 = (noise_variable_433 + noise_metadata_schedule_763_e9564);
        let noise_metadata_schedule_763_e9566: f64 = (noise_variable_434 * noise_metadata_schedule_763_e9565);
        (noise_metadata_schedule_763_e9566,)
    } else {
        (noise_variable_435,)
    }
};
            noise_variable_435 = noise_metadata_schedule_763_e9568;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_764_e9587,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_442 == 0.0)) {
        let noise_metadata_schedule_764_e9577: f64 = (noise_variable_63 * noise_variable_431);
        let noise_metadata_schedule_764_e9579: f64 = (-noise_variable_432);
        let noise_metadata_schedule_764_e9582: f64 = (noise_variable_431 * noise_variable_436);
        let noise_metadata_schedule_764_e9583: f64 = (noise_metadata_schedule_764_e9579 / noise_metadata_schedule_764_e9582);
        let noise_metadata_schedule_764_e9584: f64 = (noise_metadata_schedule_764_e9583).exp();
        let noise_metadata_schedule_764_e9585: f64 = (noise_metadata_schedule_764_e9577 * noise_metadata_schedule_764_e9584);
        (noise_metadata_schedule_764_e9585,)
    } else {
        (noise_variable_435,)
    }
};
            noise_variable_435 = noise_metadata_schedule_764_e9587;
        }
        if matches!(source_index, 10) {
            let noise_metadata_schedule_765_e9590: f64 = if params.p34 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_443 = noise_metadata_schedule_765_e9590;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_766_e9602,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_443 != 0.0)) {
        let noise_metadata_schedule_766_e9599: f64 = (params.p34 * noise_variable_435);
        let noise_metadata_schedule_766_e9600: f64 = (1.0 - noise_metadata_schedule_766_e9599);
        (noise_metadata_schedule_766_e9600,)
    } else {
        (noise_variable_444,)
    }
};
            noise_variable_444 = noise_metadata_schedule_766_e9602;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_767_e9615,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_443 != 0.0)) {
        let noise_metadata_schedule_767_e9610: f64 = (noise_variable_444 * noise_variable_444);
        let noise_metadata_schedule_767_e9612: f64 = (noise_metadata_schedule_767_e9610 + 0.0001);
        let noise_metadata_schedule_767_e9613: f64 = (noise_metadata_schedule_767_e9612).sqrt();
        (noise_metadata_schedule_767_e9613,)
    } else {
        (noise_variable_445,)
    }
};
            noise_variable_445 = noise_metadata_schedule_767_e9615;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_768_e9627,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_443 != 0.0)) {
        let noise_metadata_schedule_768_e9624: f64 = (noise_variable_444 + noise_variable_445);
        let noise_metadata_schedule_768_e9625: f64 = (0.5 * noise_metadata_schedule_768_e9624);
        (noise_metadata_schedule_768_e9625,)
    } else {
        (noise_variable_446,)
    }
};
            noise_variable_446 = noise_metadata_schedule_768_e9627;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_769_e9639,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_443 != 0.0)) {
        let noise_metadata_schedule_769_e9635: f64 = (noise_variable_217 * noise_variable_435);
        let noise_metadata_schedule_769_e9637: f64 = (noise_metadata_schedule_769_e9635 / noise_variable_446);
        (noise_metadata_schedule_769_e9637,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_769_e9639;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_770_e9650,) = {
    if (((noise_variable_430 != 0.0) && (noise_variable_437 != 0.0)) && (noise_variable_443 == 0.0)) {
        let noise_metadata_schedule_770_e9648: f64 = (noise_variable_217 * noise_variable_435);
        (noise_metadata_schedule_770_e9648,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_770_e9650;
        }
        if matches!(source_index, 10) {
            let (noise_metadata_schedule_771_e9657,) = {
    if ((noise_variable_430 != 0.0) && (noise_variable_437 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_771_e9657;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_773_e9663: f64 = if noise_variable_69 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_447 = noise_metadata_schedule_773_e9663;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_774_e9671,) = {
    if (noise_variable_447 != 0.0) {
        let noise_metadata_schedule_774_e9667: f64 = (1.0 + params.p92);
        let noise_metadata_schedule_774_e9669: f64 = (noise_metadata_schedule_774_e9667 * noise_variable_16);
        (noise_metadata_schedule_774_e9669,)
    } else {
        (noise_variable_449,)
    }
};
            noise_variable_449 = noise_metadata_schedule_774_e9671;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_775_e9679,) = {
    if (noise_variable_447 != 0.0) {
        let noise_metadata_schedule_775_e9675: f64 = (noise_variable_179 + noise_variable_178);
        let noise_metadata_schedule_775_e9677: f64 = (noise_metadata_schedule_775_e9675 + noise_variable_355);
        (noise_metadata_schedule_775_e9677,)
    } else {
        (noise_variable_451,)
    }
};
            noise_variable_451 = noise_metadata_schedule_775_e9679;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_776_e9687,) = {
    if (noise_variable_447 != 0.0) {
        let noise_metadata_schedule_776_e9684: f64 = (noise_variable_451 / noise_variable_449);
        let noise_metadata_schedule_776_e9685: f64 = (1.0 + noise_metadata_schedule_776_e9684);
        (noise_metadata_schedule_776_e9685,)
    } else {
        (noise_variable_448,)
    }
};
            noise_variable_448 = noise_metadata_schedule_776_e9687;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_777_e9700,) = {
    if (noise_variable_447 != 0.0) {
        let noise_metadata_schedule_777_e9693: f64 = (noise_variable_448 * noise_variable_448);
        let noise_metadata_schedule_777_e9695: f64 = (noise_metadata_schedule_777_e9693 + 0.01);
        let noise_metadata_schedule_777_e9696: f64 = (noise_metadata_schedule_777_e9695).sqrt();
        let noise_metadata_schedule_777_e9697: f64 = (noise_variable_448 + noise_metadata_schedule_777_e9696);
        let noise_metadata_schedule_777_e9698: f64 = (0.5 * noise_metadata_schedule_777_e9697);
        (noise_metadata_schedule_777_e9698,)
    } else {
        (noise_variable_452,)
    }
};
            noise_variable_452 = noise_metadata_schedule_777_e9700;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_778_e9706,) = {
    if (noise_variable_447 != 0.0) {
        let noise_metadata_schedule_778_e9704: f64 = (noise_variable_69 / noise_variable_452);
        (noise_metadata_schedule_778_e9704,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_778_e9706;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_779_e9709: f64 = if noise_variable_185 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_453 = noise_metadata_schedule_779_e9709;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_780_e9721,) = {
    if ((noise_variable_447 != 0.0) && (noise_variable_453 != 0.0)) {
        let noise_metadata_schedule_780_e9715: f64 = (noise_variable_70 * noise_variable_185);
        let noise_metadata_schedule_780_e9717: f64 = (noise_metadata_schedule_780_e9715 * params.p91);
        let noise_metadata_schedule_780_e9719: f64 = (noise_metadata_schedule_780_e9717 * noise_variable_5);
        (noise_metadata_schedule_780_e9719,)
    } else {
        (noise_variable_450,)
    }
};
            noise_variable_450 = noise_metadata_schedule_780_e9721;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_781_e9724: f64 = if noise_variable_450 < 1e-6 { 1.0 } else { 0.0 };
            noise_variable_454 = noise_metadata_schedule_781_e9724;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_782_e9738,) = {
    if (((noise_variable_447 != 0.0) && (noise_variable_453 != 0.0)) && (noise_variable_454 != 0.0)) {
        let noise_metadata_schedule_782_e9734: f64 = (0.5 * noise_variable_450);
        let noise_metadata_schedule_782_e9735: f64 = (1.0 - noise_metadata_schedule_782_e9734);
        let noise_metadata_schedule_782_e9736: f64 = (noise_variable_70 * noise_metadata_schedule_782_e9735);
        (noise_metadata_schedule_782_e9736,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_782_e9738;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_783_e9754,) = {
    if (((noise_variable_447 != 0.0) && (noise_variable_453 != 0.0)) && (noise_variable_454 == 0.0)) {
        let noise_metadata_schedule_783_e9748: f64 = (1.0 + noise_variable_450);
        let noise_metadata_schedule_783_e9749: f64 = (noise_metadata_schedule_783_e9748).ln();
        let noise_metadata_schedule_783_e9750: f64 = (noise_variable_70 * noise_metadata_schedule_783_e9749);
        let noise_metadata_schedule_783_e9752: f64 = (noise_metadata_schedule_783_e9750 / noise_variable_450);
        (noise_metadata_schedule_783_e9752,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_783_e9754;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_784_e9757: f64 = if noise_variable_355 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_455 = noise_metadata_schedule_784_e9757;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_785_e9773,) = {
    if ((noise_variable_447 != 0.0) && (noise_variable_455 != 0.0)) {
        let noise_metadata_schedule_785_e9765: f64 = (noise_variable_355 * params.p94);
        let noise_metadata_schedule_785_e9766: f64 = (noise_variable_179 + noise_metadata_schedule_785_e9765);
        let noise_metadata_schedule_785_e9767: f64 = (noise_variable_70 * noise_metadata_schedule_785_e9766);
        let noise_metadata_schedule_785_e9770: f64 = (noise_variable_179 + noise_variable_355);
        let noise_metadata_schedule_785_e9771: f64 = (noise_metadata_schedule_785_e9767 / noise_metadata_schedule_785_e9770);
        (noise_metadata_schedule_785_e9771,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_785_e9773;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_786_e9778,) = {
    if (noise_variable_447 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_70,)
    }
};
            noise_variable_70 = noise_metadata_schedule_786_e9778;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 13 | 14) {
            let noise_metadata_schedule_787_e9781: f64 = if params.p18 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_456 = noise_metadata_schedule_787_e9781;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 13 | 14) {
            let (noise_metadata_schedule_788_e9789,) = {
    if (noise_variable_456 != 0.0) {
        let noise_metadata_schedule_788_e9786: f64 = (params.p19 * noise_variable_4);
        let noise_metadata_schedule_788_e9787: f64 = (noise_variable_205 / noise_metadata_schedule_788_e9786);
        (noise_metadata_schedule_788_e9787,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_788_e9789;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 13 | 14) {
            let noise_metadata_schedule_789_e9792: f64 = if noise_variable_93 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_457 = noise_metadata_schedule_789_e9792;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 13 | 14) {
            let (noise_metadata_schedule_790_e9802,) = {
    if ((noise_variable_456 != 0.0) && (noise_variable_457 != 0.0)) {
        let noise_metadata_schedule_790_e9799: f64 = (noise_variable_93 - 80.0);
        let noise_metadata_schedule_790_e9800: f64 = (1.0 + noise_metadata_schedule_790_e9799);
        (noise_metadata_schedule_790_e9800,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_790_e9802;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 13 | 14) {
            let (noise_metadata_schedule_791_e9808,) = {
    if ((noise_variable_456 != 0.0) && (noise_variable_457 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_791_e9808;
        }
        if matches!(source_index, 5 | 6 | 7 | 9 | 13 | 14) {
            let (noise_metadata_schedule_792_e9815,) = {
    if ((noise_variable_456 != 0.0) && (noise_variable_457 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_792_e9815;
        }
        if matches!(source_index, 5 | 6 | 7 | 9) {
            let (noise_metadata_schedule_793_e9826,) = {
    if (noise_variable_456 != 0.0) {
        let noise_metadata_schedule_793_e9820: f64 = { let limexp_arg = noise_variable_93; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_793_e9821: f64 = (noise_variable_94 * noise_metadata_schedule_793_e9820);
        let noise_metadata_schedule_793_e9823: f64 = (noise_metadata_schedule_793_e9821 - 1.0);
        let noise_metadata_schedule_793_e9824: f64 = (noise_variable_23 * noise_metadata_schedule_793_e9823);
        (noise_metadata_schedule_793_e9824,)
    } else {
        (noise_variable_188,)
    }
};
            noise_variable_188 = noise_metadata_schedule_793_e9826;
        }
        if matches!(source_index, 5 | 6 | 7 | 9) {
            let (noise_metadata_schedule_794_e9831,) = {
    if (noise_variable_456 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_188,)
    }
};
            noise_variable_188 = noise_metadata_schedule_794_e9831;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_795_e9834: f64 = if params.p20 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_458 = noise_metadata_schedule_795_e9834;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_796_e9842,) = {
    if (noise_variable_458 != 0.0) {
        let noise_metadata_schedule_796_e9839: f64 = (params.p21 * noise_variable_4);
        let noise_metadata_schedule_796_e9840: f64 = (noise_variable_205 / noise_metadata_schedule_796_e9839);
        (noise_metadata_schedule_796_e9840,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_796_e9842;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_797_e9845: f64 = if noise_variable_93 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_459 = noise_metadata_schedule_797_e9845;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_798_e9855,) = {
    if ((noise_variable_458 != 0.0) && (noise_variable_459 != 0.0)) {
        let noise_metadata_schedule_798_e9852: f64 = (noise_variable_93 - 80.0);
        let noise_metadata_schedule_798_e9853: f64 = (1.0 + noise_metadata_schedule_798_e9852);
        (noise_metadata_schedule_798_e9853,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_798_e9855;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_799_e9861,) = {
    if ((noise_variable_458 != 0.0) && (noise_variable_459 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_799_e9861;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_800_e9868,) = {
    if ((noise_variable_458 != 0.0) && (noise_variable_459 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_800_e9868;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_803_e9887: f64 = if noise_variable_29 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_460 = noise_metadata_schedule_803_e9887;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_804_e9900,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_804_e9892: f64 = (noise_variable_31).ln();
        let noise_metadata_schedule_804_e9893: f64 = (-noise_metadata_schedule_804_e9892);
        let noise_metadata_schedule_804_e9895: f64 = (noise_metadata_schedule_804_e9893 / params.p45);
        let noise_metadata_schedule_804_e9896: f64 = (noise_metadata_schedule_804_e9895).exp();
        let noise_metadata_schedule_804_e9897: f64 = (1.0 - noise_metadata_schedule_804_e9896);
        let noise_metadata_schedule_804_e9898: f64 = (noise_variable_30 * noise_metadata_schedule_804_e9897);
        (noise_metadata_schedule_804_e9898,)
    } else {
        (noise_variable_137,)
    }
};
            noise_variable_137 = noise_metadata_schedule_804_e9900;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_805_e9908,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_805_e9904: f64 = (noise_variable_137 - noise_variable_205);
        let noise_metadata_schedule_805_e9906: f64 = (noise_metadata_schedule_805_e9904 * noise_variable_5);
        (noise_metadata_schedule_805_e9906,)
    } else {
        (noise_variable_141,)
    }
};
            noise_variable_141 = noise_metadata_schedule_805_e9908;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_806_e9917,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_806_e9912: f64 = (noise_variable_141 * noise_variable_141);
        let noise_metadata_schedule_806_e9914: f64 = (noise_metadata_schedule_806_e9912 + 1.921812);
        let noise_metadata_schedule_806_e9915: f64 = (noise_metadata_schedule_806_e9914).sqrt();
        (noise_metadata_schedule_806_e9915,)
    } else {
        (noise_variable_142,)
    }
};
            noise_variable_142 = noise_metadata_schedule_806_e9917;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_807_e9925,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_807_e9921: f64 = (noise_variable_141 + noise_variable_142);
        let noise_metadata_schedule_807_e9923: f64 = (noise_metadata_schedule_807_e9921 * 0.5);
        (noise_metadata_schedule_807_e9923,)
    } else {
        (noise_variable_143,)
    }
};
            noise_variable_143 = noise_metadata_schedule_807_e9925;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_808_e9933,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_808_e9930: f64 = (noise_variable_4 * noise_variable_143);
        let noise_metadata_schedule_808_e9931: f64 = (noise_variable_137 - noise_metadata_schedule_808_e9930);
        (noise_metadata_schedule_808_e9931,)
    } else {
        (noise_variable_138,)
    }
};
            noise_variable_138 = noise_metadata_schedule_808_e9933;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_809_e9939,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_809_e9937: f64 = (noise_variable_143 / noise_variable_142);
        (noise_metadata_schedule_809_e9937,)
    } else {
        (noise_variable_144,)
    }
};
            noise_variable_144 = noise_metadata_schedule_809_e9939;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_810_e9948,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_810_e9944: f64 = (noise_variable_138 / noise_variable_30);
        let noise_metadata_schedule_810_e9945: f64 = (1.0 - noise_metadata_schedule_810_e9944);
        let noise_metadata_schedule_810_e9946: f64 = (noise_metadata_schedule_810_e9945).ln();
        (noise_metadata_schedule_810_e9946,)
    } else {
        (noise_variable_139,)
    }
};
            noise_variable_139 = noise_metadata_schedule_810_e9948;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_811_e9958,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_811_e9951: f64 = (-params.p45);
        let noise_metadata_schedule_811_e9953: f64 = (noise_metadata_schedule_811_e9951 * noise_variable_139);
        let noise_metadata_schedule_811_e9954: f64 = (noise_metadata_schedule_811_e9953).exp();
        let noise_metadata_schedule_811_e9956: f64 = (noise_metadata_schedule_811_e9954 * noise_variable_144);
        (noise_metadata_schedule_811_e9956,)
    } else {
        (noise_variable_145,)
    }
};
            noise_variable_145 = noise_metadata_schedule_811_e9958;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_812_e9970,) = {
    if (noise_variable_460 != 0.0) {
        let noise_metadata_schedule_812_e9965: f64 = (1.0 - noise_variable_144);
        let noise_metadata_schedule_812_e9966: f64 = (noise_variable_31 * noise_metadata_schedule_812_e9965);
        let noise_metadata_schedule_812_e9967: f64 = (noise_variable_145 + noise_metadata_schedule_812_e9966);
        let noise_metadata_schedule_812_e9968: f64 = (noise_variable_29 * noise_metadata_schedule_812_e9967);
        (noise_metadata_schedule_812_e9968,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_812_e9970;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_815_e10004,) = {
    if (noise_variable_460 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_212,)
    }
};
            noise_variable_212 = noise_metadata_schedule_815_e10004;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_817_e10020: f64 = if ((params.p27 > 0.0) && ((noise_variable_205 < noise_variable_223) || (noise_variable_202 < noise_variable_223))) { 1.0 } else { 0.0 };
            noise_variable_461 = noise_metadata_schedule_817_e10020;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_818_e10031: f64 = if (((params.p29 == 1.0) && (noise_variable_29 > 0.0)) && (noise_variable_30 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_464 = noise_metadata_schedule_818_e10031;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_819_e10047,) = {
    if ((noise_variable_461 != 0.0) && (noise_variable_464 != 0.0)) {
        let noise_metadata_schedule_819_e10038: f64 = (1.0 / params.p45);
        let noise_metadata_schedule_819_e10039: f64 = (1.0 - noise_metadata_schedule_819_e10038);
        let noise_metadata_schedule_819_e10042: f64 = (noise_variable_212 / noise_variable_29);
        let noise_metadata_schedule_819_e10043: f64 = (noise_metadata_schedule_819_e10042).ln();
        let noise_metadata_schedule_819_e10044: f64 = (noise_metadata_schedule_819_e10039 * noise_metadata_schedule_819_e10043);
        let noise_metadata_schedule_819_e10045: f64 = (noise_metadata_schedule_819_e10044).exp();
        (noise_metadata_schedule_819_e10045,)
    } else {
        (noise_variable_462,)
    }
};
            noise_variable_462 = noise_metadata_schedule_819_e10047;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_820_e10060,) = {
    if ((noise_variable_461 != 0.0) && (noise_variable_464 != 0.0)) {
        let noise_metadata_schedule_820_e10053: f64 = (noise_variable_205 / noise_variable_30);
        let noise_metadata_schedule_820_e10054: f64 = (-noise_metadata_schedule_820_e10053);
        let noise_metadata_schedule_820_e10056: f64 = (noise_metadata_schedule_820_e10054 * noise_variable_64);
        let noise_metadata_schedule_820_e10058: f64 = (noise_metadata_schedule_820_e10056 * noise_variable_462);
        (noise_metadata_schedule_820_e10058,)
    } else {
        (noise_variable_463,)
    }
};
            noise_variable_463 = noise_metadata_schedule_820_e10060;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_821_e10072,) = {
    if ((noise_variable_461 != 0.0) && (noise_variable_464 != 0.0)) {
        let noise_metadata_schedule_821_e10066: f64 = (-noise_variable_65);
        let noise_metadata_schedule_821_e10068: f64 = (noise_metadata_schedule_821_e10066 / noise_variable_462);
        let noise_metadata_schedule_821_e10069: f64 = (noise_metadata_schedule_821_e10068).exp();
        let noise_metadata_schedule_821_e10070: f64 = (noise_variable_463 * noise_metadata_schedule_821_e10069);
        (noise_metadata_schedule_821_e10070,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_821_e10072;
        }
        if matches!(source_index, 8) {
            let noise_metadata_schedule_822_e10083: f64 = if (((params.p29 == 0.0) && (noise_variable_26 > 0.0)) && (noise_variable_27 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_465 = noise_metadata_schedule_822_e10083;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_823_e10102,) = {
    if (((noise_variable_461 != 0.0) && (noise_variable_464 == 0.0)) && (noise_variable_465 != 0.0)) {
        let noise_metadata_schedule_823_e10093: f64 = (1.0 / params.p41);
        let noise_metadata_schedule_823_e10094: f64 = (1.0 - noise_metadata_schedule_823_e10093);
        let noise_metadata_schedule_823_e10097: f64 = (noise_variable_211 / noise_variable_26);
        let noise_metadata_schedule_823_e10098: f64 = (noise_metadata_schedule_823_e10097).ln();
        let noise_metadata_schedule_823_e10099: f64 = (noise_metadata_schedule_823_e10094 * noise_metadata_schedule_823_e10098);
        let noise_metadata_schedule_823_e10100: f64 = (noise_metadata_schedule_823_e10099).exp();
        (noise_metadata_schedule_823_e10100,)
    } else {
        (noise_variable_462,)
    }
};
            noise_variable_462 = noise_metadata_schedule_823_e10102;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_824_e10118,) = {
    if (((noise_variable_461 != 0.0) && (noise_variable_464 == 0.0)) && (noise_variable_465 != 0.0)) {
        let noise_metadata_schedule_824_e10111: f64 = (noise_variable_202 / noise_variable_27);
        let noise_metadata_schedule_824_e10112: f64 = (-noise_metadata_schedule_824_e10111);
        let noise_metadata_schedule_824_e10114: f64 = (noise_metadata_schedule_824_e10112 * noise_variable_64);
        let noise_metadata_schedule_824_e10116: f64 = (noise_metadata_schedule_824_e10114 * noise_variable_462);
        (noise_metadata_schedule_824_e10116,)
    } else {
        (noise_variable_463,)
    }
};
            noise_variable_463 = noise_metadata_schedule_824_e10118;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_825_e10133,) = {
    if (((noise_variable_461 != 0.0) && (noise_variable_464 == 0.0)) && (noise_variable_465 != 0.0)) {
        let noise_metadata_schedule_825_e10127: f64 = (-noise_variable_65);
        let noise_metadata_schedule_825_e10129: f64 = (noise_metadata_schedule_825_e10127 / noise_variable_462);
        let noise_metadata_schedule_825_e10130: f64 = (noise_metadata_schedule_825_e10129).exp();
        let noise_metadata_schedule_825_e10131: f64 = (noise_variable_463 * noise_metadata_schedule_825_e10130);
        (noise_metadata_schedule_825_e10131,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_825_e10133;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_826_e10143,) = {
    if (((noise_variable_461 != 0.0) && (noise_variable_464 == 0.0)) && (noise_variable_465 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_826_e10143;
        }
        if matches!(source_index, 8) {
            let (noise_metadata_schedule_827_e10148,) = {
    if (noise_variable_461 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_827_e10148;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_880_e10712: f64 = if params.p25 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_471 = noise_metadata_schedule_880_e10712;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_881_e10720,) = {
    if (noise_variable_471 != 0.0) {
        let noise_metadata_schedule_881_e10717: f64 = (params.p26 * noise_variable_4);
        let noise_metadata_schedule_881_e10718: f64 = (noise_variable_206 / noise_metadata_schedule_881_e10717);
        (noise_metadata_schedule_881_e10718,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_881_e10720;
        }
        if matches!(source_index, 13 | 14) {
            let noise_metadata_schedule_882_e10723: f64 = if noise_variable_93 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_472 = noise_metadata_schedule_882_e10723;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_883_e10733,) = {
    if ((noise_variable_471 != 0.0) && (noise_variable_472 != 0.0)) {
        let noise_metadata_schedule_883_e10730: f64 = (noise_variable_93 - 80.0);
        let noise_metadata_schedule_883_e10731: f64 = (1.0 + noise_metadata_schedule_883_e10730);
        (noise_metadata_schedule_883_e10731,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_883_e10733;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_884_e10739,) = {
    if ((noise_variable_471 != 0.0) && (noise_variable_472 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_884_e10739;
        }
        if matches!(source_index, 13 | 14) {
            let (noise_metadata_schedule_885_e10746,) = {
    if ((noise_variable_471 != 0.0) && (noise_variable_472 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_885_e10746;
        }
        if matches!(source_index, 13) {
            let (noise_metadata_schedule_886_e10757,) = {
    if (noise_variable_471 != 0.0) {
        let noise_metadata_schedule_886_e10751: f64 = { let limexp_arg = noise_variable_93; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_886_e10752: f64 = (noise_variable_94 * noise_metadata_schedule_886_e10751);
        let noise_metadata_schedule_886_e10754: f64 = (noise_metadata_schedule_886_e10752 - 1.0);
        let noise_metadata_schedule_886_e10755: f64 = (noise_variable_36 * noise_metadata_schedule_886_e10754);
        (noise_metadata_schedule_886_e10755,)
    } else {
        (noise_variable_194,)
    }
};
            noise_variable_194 = noise_metadata_schedule_886_e10757;
        }
        if matches!(source_index, 13) {
            let (noise_metadata_schedule_887_e10762,) = {
    if (noise_variable_471 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_194,)
    }
};
            noise_variable_194 = noise_metadata_schedule_887_e10762;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_1054_e12592: f64 = if params.p99 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_494 = noise_metadata_schedule_1054_e12592;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_1055_e12600,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_1055_e12597: f64 = (params.p100 * noise_variable_4);
        let noise_metadata_schedule_1055_e12598: f64 = (noise_variable_208 / noise_metadata_schedule_1055_e12597);
        (noise_metadata_schedule_1055_e12598,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_1055_e12600;
        }
        if matches!(source_index, 14) {
            let noise_metadata_schedule_1056_e12603: f64 = if noise_variable_93 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_495 = noise_metadata_schedule_1056_e12603;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_1057_e12613,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 != 0.0)) {
        let noise_metadata_schedule_1057_e12610: f64 = (noise_variable_93 - 80.0);
        let noise_metadata_schedule_1057_e12611: f64 = (1.0 + noise_metadata_schedule_1057_e12610);
        (noise_metadata_schedule_1057_e12611,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_1057_e12613;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_1058_e12619,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_1058_e12619;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_1059_e12626,) = {
    if ((noise_variable_494 != 0.0) && (noise_variable_495 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_94,)
    }
};
            noise_variable_94 = noise_metadata_schedule_1059_e12626;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_1060_e12637,) = {
    if (noise_variable_494 != 0.0) {
        let noise_metadata_schedule_1060_e12631: f64 = { let limexp_arg = noise_variable_93; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_1060_e12632: f64 = (noise_variable_94 * noise_metadata_schedule_1060_e12631);
        let noise_metadata_schedule_1060_e12634: f64 = (noise_metadata_schedule_1060_e12632 - 1.0);
        let noise_metadata_schedule_1060_e12635: f64 = (noise_variable_45 * noise_metadata_schedule_1060_e12634);
        (noise_metadata_schedule_1060_e12635,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_1060_e12637;
        }
        if matches!(source_index, 14) {
            let (noise_metadata_schedule_1061_e12642,) = {
    if (noise_variable_494 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_1061_e12642;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4) {
            let noise_metadata_schedule_1110_e12997: f64 = (4.0 * noise_variable_1);
            let noise_metadata_schedule_1110_e12999: f64 = (noise_metadata_schedule_1110_e12997 * noise_variable_10);
            noise_variable_521 = noise_metadata_schedule_1110_e12999;
        }
        if matches!(source_index, 5 | 6 | 7) {
            let noise_metadata_schedule_1116_e13038: f64 = (noise_variable_185 + noise_variable_188);
            let noise_metadata_schedule_1116_e13039: f64 = (noise_metadata_schedule_1116_e13038).abs();
            let noise_metadata_schedule_1116_e13041: f64 = (noise_metadata_schedule_1116_e13039).powf(params.p111);
            let noise_metadata_schedule_1116_e13042: f64 = (params.p110 * noise_metadata_schedule_1116_e13041);
            noise_variable_523 = noise_metadata_schedule_1116_e13042;
        }
        if matches!(source_index, 5 | 6 | 7) {
            let noise_metadata_schedule_1118_e13053: f64 = if ((params.p95 >= params.p149) && (params.p95 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_531 = noise_metadata_schedule_1118_e13053;
        }
        if matches!(source_index, 5 | 6 | 7) {
            let (noise_metadata_schedule_1119_e13059,) = {
    if (noise_variable_531 != 0.0) {
        let noise_metadata_schedule_1119_e13057: f64 = ((ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[2])) / noise_variable_73);
        (noise_metadata_schedule_1119_e13057,)
    } else {
        (noise_variable_524,)
    }
};
            noise_variable_524 = noise_metadata_schedule_1119_e13059;
        }
        if matches!(source_index, 5 | 6 | 7) {
            let (noise_metadata_schedule_1120_e13068,) = {
    if (noise_variable_531 != 0.0) {
        let noise_metadata_schedule_1120_e13063: f64 = (noise_variable_524).abs();
        let noise_metadata_schedule_1120_e13065: f64 = (noise_metadata_schedule_1120_e13063).powf(params.p114);
        let noise_metadata_schedule_1120_e13066: f64 = (params.p113 * noise_metadata_schedule_1120_e13065);
        (noise_metadata_schedule_1120_e13066,)
    } else {
        (noise_variable_523,)
    }
};
            noise_variable_523 = noise_metadata_schedule_1120_e13068;
        }
        if matches!(source_index, 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18) {
            let noise_metadata_schedule_1121_e13071: f64 = (2.0 * noise_variable_0);
            noise_variable_522 = noise_metadata_schedule_1121_e13071;
        }
        match source_index {
            0 => {
                let noise_0_psd_e13444: f64 = 1.0;
                let noise_0_psd_e403: f64 = (noise_variable_521 / noise_variable_71);
                let noise_0_psd_e13445: f64 = (noise_0_psd_e13444 * noise_0_psd_e403);
                let psd = noise_0_psd_e13445;
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
                let noise_1_psd_e13447: f64 = 1.0;
                let noise_1_psd_e411: f64 = (noise_variable_521 / noise_variable_70);
                let noise_1_psd_e13448: f64 = (noise_1_psd_e13447 * noise_1_psd_e411);
                let psd = noise_1_psd_e13448;
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
                let noise_2_psd_e13450: f64 = 1.0;
                let noise_2_psd_e419: f64 = (noise_variable_521 / noise_variable_72);
                let noise_2_psd_e13451: f64 = (noise_2_psd_e13450 * noise_2_psd_e419);
                let psd = noise_2_psd_e13451;
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
                let noise_3_psd_e13453: f64 = 1.0;
                let noise_3_psd_e427: f64 = (noise_variable_521 / noise_variable_73);
                let noise_3_psd_e13454: f64 = (noise_3_psd_e13453 * noise_3_psd_e427);
                let psd = noise_3_psd_e13454;
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
                let noise_4_psd_e13456: f64 = 1.0;
                let noise_4_psd_e435: f64 = (noise_variable_521 / params.p102);
                let noise_4_psd_e13457: f64 = (noise_4_psd_e13456 * noise_4_psd_e435);
                let psd = noise_4_psd_e13457;
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
                let noise_5_psd_e13459: f64 = 1.0;
                let noise_5_psd_e13460: f64 = (noise_5_psd_e13459 * noise_variable_523);
                let psd = noise_5_psd_e13460;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            6 => {
                let noise_6_psd_e13462: f64 = 1.0;
                let noise_6_psd_e13463: f64 = (noise_6_psd_e13462 * noise_variable_523);
                let psd = noise_6_psd_e13463;
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
                let noise_7_psd_e13465: f64 = 1.0;
                let noise_7_psd_e13466: f64 = (noise_7_psd_e13465 * noise_variable_523);
                let psd = noise_7_psd_e13466;
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
                let noise_8_psd_e13468: f64 = 1.0;
                let noise_8_psd_e465: f64 = (noise_variable_191).abs();
                let noise_8_psd_e466: f64 = (noise_variable_522 * noise_8_psd_e465);
                let noise_8_psd_e13469: f64 = (noise_8_psd_e13468 * noise_8_psd_e466);
                let psd = noise_8_psd_e13469;
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
                let noise_9_psd_e13471: f64 = 1.0;
                let noise_9_psd_e473: f64 = (noise_variable_188).abs();
                let noise_9_psd_e474: f64 = (noise_variable_522 * noise_9_psd_e473);
                let noise_9_psd_e13472: f64 = (noise_9_psd_e13471 * noise_9_psd_e474);
                let psd = noise_9_psd_e13472;
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
                let noise_10_psd_e13474: f64 = 1.0;
                let noise_10_psd_e479: f64 = (noise_variable_522 * noise_variable_244);
                let noise_10_psd_e13475: f64 = (noise_10_psd_e13474 * noise_10_psd_e479);
                let psd = noise_10_psd_e13475;
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
                let noise_11_psd_e13477: f64 = 1.0;
                let noise_11_psd_e484: f64 = (noise_variable_187).abs();
                let noise_11_psd_e485: f64 = (noise_variable_522 * noise_11_psd_e484);
                let noise_11_psd_e13478: f64 = (noise_11_psd_e13477 * noise_11_psd_e485);
                let psd = noise_11_psd_e13478;
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
                let noise_12_psd_e13480: f64 = 1.0;
                let noise_12_psd_e490: f64 = (noise_variable_193).abs();
                let noise_12_psd_e491: f64 = (noise_variable_522 * noise_12_psd_e490);
                let noise_12_psd_e13481: f64 = (noise_12_psd_e13480 * noise_12_psd_e491);
                let psd = noise_12_psd_e13481;
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
                let noise_13_psd_e13483: f64 = 1.0;
                let noise_13_psd_e496: f64 = (noise_variable_194).abs();
                let noise_13_psd_e497: f64 = (noise_variable_522 * noise_13_psd_e496);
                let noise_13_psd_e13484: f64 = (noise_13_psd_e13483 * noise_13_psd_e497);
                let psd = noise_13_psd_e13484;
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
                let noise_14_psd_e13486: f64 = 1.0;
                let noise_14_psd_e502: f64 = (noise_variable_195).abs();
                let noise_14_psd_e503: f64 = (noise_variable_522 * noise_14_psd_e502);
                let noise_14_psd_e13487: f64 = (noise_14_psd_e13486 * noise_14_psd_e503);
                let psd = noise_14_psd_e13487;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            15 => {
                let noise_15_psd_e13489: f64 = 1.0;
                let noise_15_psd_e509: f64 = (noise_variable_185).abs();
                let noise_15_psd_e510: f64 = (noise_variable_522 * noise_15_psd_e509);
                let noise_15_psd_e13490: f64 = (noise_15_psd_e13489 * noise_15_psd_e510);
                let psd = noise_15_psd_e13490;
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
                let noise_16_psd_e13492: f64 = 1.0;
                let noise_16_psd_e549: f64 = (noise_variable_184).abs();
                let noise_16_psd_e550: f64 = (noise_variable_522 * noise_16_psd_e549);
                let noise_16_psd_e13493: f64 = (noise_16_psd_e13492 * noise_16_psd_e550);
                let psd = noise_16_psd_e13493;
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
                let noise_17_psd_e13495: f64 = 1.0;
                let noise_17_psd_e568: f64 = (noise_variable_184).abs();
                let noise_17_psd_e569: f64 = (noise_variable_522 * noise_17_psd_e568);
                let noise_17_psd_e13496: f64 = (noise_17_psd_e13495 * noise_17_psd_e569);
                let psd = noise_17_psd_e13496;
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
                let noise_18_psd_e13498: f64 = 1.0;
                let noise_18_psd_e578: f64 = (noise_variable_185).abs();
                let noise_18_psd_e579: f64 = (noise_variable_522 * noise_18_psd_e578);
                let noise_18_psd_e13499: f64 = (noise_18_psd_e13498 * noise_18_psd_e579);
                let psd = noise_18_psd_e13499;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

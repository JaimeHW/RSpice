#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_I2_I1_BODY_THERMAL_NOISE", label: Some("body thermal noise"), kind: GeneratedNoiseKind::White, equation: 12, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_I2_I1_BODY_1_F_NOISE", label: Some("body 1/f noise"), kind: GeneratedNoiseKind::Flicker, equation: 13, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_I1_END_1_RESISTANCE_THERMAL_NOISE", label: Some("end 1 resistance thermal noise"), kind: GeneratedNoiseKind::White, equation: 14, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_I2_END_2_RESISTANCE_THERMAL_NOISE", label: Some("end 2 resistance thermal noise"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "n2", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_NC_I1_END_1_PARASITIC_SHOT_NOISE", label: Some("end 1 parasitic shot noise"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "nc", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "i1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_NC_I2_END_2_PARASITIC_SHOT_NOISE", label: Some("end 2 parasitic shot noise"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "nc", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "i2", is_internal: true }, table_len: 0, table_log_interp: false },
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
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_5_e279: f64 = (0.01 * params.p23);
            let noise_activation_schedule_5_e280: f64 = (1.0 - noise_activation_schedule_5_e279);
            let noise_activation_schedule_5_e282: f64 = (noise_activation_schedule_5_e280 * params.p22);
            let noise_activation_schedule_5_e284: f64 = (noise_activation_schedule_5_e282 * 1000000.0);
            noise_variable_13 = noise_activation_schedule_5_e284;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_6_e287: f64 = (noise_variable_13 * noise_variable_13);
            noise_variable_14 = noise_activation_schedule_6_e287;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_7_e290: f64 = (273.15 + params.p28);
            noise_variable_15 = noise_activation_schedule_7_e290;
        }
        if matches!(source_index, 4) {
            let noise_activation_schedule_28_e384: f64 = (params.p3 * noise_variable_14);
            noise_variable_31 = noise_activation_schedule_28_e384;
        }
        if matches!(source_index, 4) {
            let noise_activation_schedule_29_e387: f64 = (params.p4 * noise_variable_13);
            noise_variable_32 = noise_activation_schedule_29_e387;
        }
        if matches!(source_index, 5) {
            let noise_activation_schedule_30_e390: f64 = (params.p6 * noise_variable_14);
            noise_variable_33 = noise_activation_schedule_30_e390;
        }
        if matches!(source_index, 5) {
            let noise_activation_schedule_31_e393: f64 = (params.p7 * noise_variable_13);
            noise_variable_34 = noise_activation_schedule_31_e393;
        }
        if matches!(source_index, 4 | 5) {
            noise_variable_10 = (ctx.node_voltage(self.nodes[3]) - 0.0);
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_112_e1137: f64 = ctx.temperature();
            let noise_activation_schedule_112_e1139: f64 = (noise_activation_schedule_112_e1137 + params.p9);
            let noise_activation_schedule_112_e1141: f64 = (noise_activation_schedule_112_e1139 + noise_variable_10);
            let noise_activation_schedule_112_e1143: f64 = (noise_activation_schedule_112_e1141 - 273.15);
            noise_variable_23 = noise_activation_schedule_112_e1143;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_113_e1147: f64 = (params.p35 + 1.0);
            let noise_activation_schedule_113_e1148: f64 = if noise_variable_23 < noise_activation_schedule_113_e1147 { 1.0 } else { 0.0 };
            noise_variable_134 = noise_activation_schedule_113_e1148;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_activation_schedule_114_e1159,) = {
    if (noise_variable_134 != 0.0) {
        let noise_activation_schedule_114_e1153: f64 = (noise_variable_23 - params.p35);
        let noise_activation_schedule_114_e1155: f64 = (noise_activation_schedule_114_e1153 - 1.0);
        let noise_activation_schedule_114_e1156: f64 = (noise_activation_schedule_114_e1155).exp();
        let noise_activation_schedule_114_e1157: f64 = (params.p35 + noise_activation_schedule_114_e1156);
        (noise_activation_schedule_114_e1157,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_activation_schedule_114_e1159;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_115_e1163: f64 = (params.p36 - 1.0);
            let noise_activation_schedule_115_e1164: f64 = if noise_variable_23 > noise_activation_schedule_115_e1163 { 1.0 } else { 0.0 };
            noise_variable_135 = noise_activation_schedule_115_e1164;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_activation_schedule_116_e1178,) = {
    if ((noise_variable_134 == 0.0) && (noise_variable_135 != 0.0)) {
        let noise_activation_schedule_116_e1172: f64 = (params.p36 - noise_variable_23);
        let noise_activation_schedule_116_e1174: f64 = (noise_activation_schedule_116_e1172 - 1.0);
        let noise_activation_schedule_116_e1175: f64 = (noise_activation_schedule_116_e1174).exp();
        let noise_activation_schedule_116_e1176: f64 = (params.p36 - noise_activation_schedule_116_e1175);
        (noise_activation_schedule_116_e1176,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_activation_schedule_116_e1178;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_activation_schedule_117_e1186,) = {
    if ((noise_variable_134 == 0.0) && (noise_variable_135 == 0.0)) {
        (noise_variable_23,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_activation_schedule_117_e1186;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_118_e1189: f64 = (noise_variable_23 + 273.15);
            noise_variable_24 = noise_activation_schedule_118_e1189;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_119_e1192: f64 = (1.3806505e-23 * noise_variable_24);
            let noise_activation_schedule_119_e1194: f64 = (noise_activation_schedule_119_e1192 / 1.60217653e-19);
            noise_variable_70 = noise_activation_schedule_119_e1194;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_120_e1197: f64 = (noise_variable_24 / noise_variable_15);
            noise_variable_68 = noise_activation_schedule_120_e1197;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_133_e1298: f64 = if params.p69 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_138 = noise_activation_schedule_133_e1298;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_activation_schedule_134_e1319,) = {
    if (noise_variable_138 != 0.0) {
        let noise_activation_schedule_134_e1302: f64 = (-params.p90);
        let noise_activation_schedule_134_e1305: f64 = (1.0 - noise_variable_68);
        let noise_activation_schedule_134_e1306: f64 = (noise_activation_schedule_134_e1302 * noise_activation_schedule_134_e1305);
        let noise_activation_schedule_134_e1308: f64 = (noise_activation_schedule_134_e1306 / noise_variable_70);
        let noise_activation_schedule_134_e1311: f64 = (noise_variable_68).ln();
        let noise_activation_schedule_134_e1312: f64 = (params.p91 * noise_activation_schedule_134_e1311);
        let noise_activation_schedule_134_e1313: f64 = (noise_activation_schedule_134_e1308 + noise_activation_schedule_134_e1312);
        let noise_activation_schedule_134_e1315: f64 = (noise_activation_schedule_134_e1313 / params.p70);
        let noise_activation_schedule_134_e1316: f64 = (noise_activation_schedule_134_e1315).exp();
        let noise_activation_schedule_134_e1317: f64 = (params.p69 * noise_activation_schedule_134_e1316);
        (noise_activation_schedule_134_e1317,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_activation_schedule_134_e1319;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_activation_schedule_136_e1337,) = {
    if (noise_variable_138 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_activation_schedule_136_e1337;
        }
        if matches!(source_index, 4 | 5) {
            let noise_activation_schedule_138_e1345: f64 = if params.p76 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_139 = noise_activation_schedule_138_e1345;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_activation_schedule_139_e1366,) = {
    if (noise_variable_139 != 0.0) {
        let noise_activation_schedule_139_e1349: f64 = (-params.p90);
        let noise_activation_schedule_139_e1352: f64 = (1.0 - noise_variable_68);
        let noise_activation_schedule_139_e1353: f64 = (noise_activation_schedule_139_e1349 * noise_activation_schedule_139_e1352);
        let noise_activation_schedule_139_e1355: f64 = (noise_activation_schedule_139_e1353 / noise_variable_70);
        let noise_activation_schedule_139_e1358: f64 = (noise_variable_68).ln();
        let noise_activation_schedule_139_e1359: f64 = (params.p91 * noise_activation_schedule_139_e1358);
        let noise_activation_schedule_139_e1360: f64 = (noise_activation_schedule_139_e1355 + noise_activation_schedule_139_e1359);
        let noise_activation_schedule_139_e1362: f64 = (noise_activation_schedule_139_e1360 / params.p77);
        let noise_activation_schedule_139_e1363: f64 = (noise_activation_schedule_139_e1362).exp();
        let noise_activation_schedule_139_e1364: f64 = (params.p76 * noise_activation_schedule_139_e1363);
        (noise_activation_schedule_139_e1364,)
    } else {
        (noise_variable_75,)
    }
};
            noise_variable_75 = noise_activation_schedule_139_e1366;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_activation_schedule_141_e1384,) = {
    if (noise_variable_139 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_75,)
    }
};
            noise_variable_75 = noise_activation_schedule_141_e1384;
        }
        if matches!(source_index, 4) {
            let noise_activation_schedule_143_e1392: f64 = (noise_variable_31 * noise_variable_74);
            let noise_activation_schedule_143_e1395: f64 = (noise_variable_32 * noise_variable_75);
            let noise_activation_schedule_143_e1396: f64 = (noise_activation_schedule_143_e1392 + noise_activation_schedule_143_e1395);
            noise_variable_84 = noise_activation_schedule_143_e1396;
        }
        if matches!(source_index, 5) {
            let noise_activation_schedule_144_e1399: f64 = (noise_variable_33 * noise_variable_74);
            let noise_activation_schedule_144_e1402: f64 = (noise_variable_34 * noise_variable_75);
            let noise_activation_schedule_144_e1403: f64 = (noise_activation_schedule_144_e1399 + noise_activation_schedule_144_e1402);
            noise_variable_85 = noise_activation_schedule_144_e1403;
        }
        if matches!(source_index, 4) {
            let noise_activation_schedule_491_e5047: f64 = if noise_variable_84 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_326 = noise_activation_schedule_491_e5047;
        }
        if matches!(source_index, 5) {
            let noise_activation_schedule_492_e5050: f64 = if noise_variable_85 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_327 = noise_activation_schedule_492_e5050;
        }
        let noise_source_active = match source_index {
            0 => {
                params.p13 != 0.0
            }
            1 => {
                params.p13 != 0.0
            }
            2 => {
                params.p13 != 0.0
            }
            3 => {
                params.p13 != 0.0
            }
            4 => {
                let noise_4_activation_e225: f64 = if ((params.p13 != 0.0) && (noise_variable_326 != 0.0)) { 1.0 } else { 0.0 };
                noise_4_activation_e225 != 0.0
            }
            5 => {
                let noise_5_activation_e245: f64 = if ((params.p13 != 0.0) && (noise_variable_327 != 0.0)) { 1.0 } else { 0.0 };
                noise_5_activation_e245 != 0.0
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
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_3_e272: f64 = self.multiplicity;
            noise_variable_12 = noise_metadata_schedule_3_e272;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_5_e279: f64 = (0.01 * params.p23);
            let noise_metadata_schedule_5_e280: f64 = (1.0 - noise_metadata_schedule_5_e279);
            let noise_metadata_schedule_5_e282: f64 = (noise_metadata_schedule_5_e280 * params.p22);
            let noise_metadata_schedule_5_e284: f64 = (noise_metadata_schedule_5_e282 * 1000000.0);
            noise_variable_13 = noise_metadata_schedule_5_e284;
        }
        if matches!(source_index, 4 | 5) {
            let noise_metadata_schedule_6_e287: f64 = (noise_variable_13 * noise_variable_13);
            noise_variable_14 = noise_metadata_schedule_6_e287;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_7_e290: f64 = (273.15 + params.p28);
            noise_variable_15 = noise_metadata_schedule_7_e290;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_9_e293: f64 = ctx.temperature();
            let noise_metadata_schedule_9_e295: f64 = (noise_metadata_schedule_9_e293 + params.p9);
            let noise_metadata_schedule_9_e297: f64 = (noise_metadata_schedule_9_e295 - 273.15);
            noise_variable_23 = noise_metadata_schedule_9_e297;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_12_e307: f64 = (params.p35 + 1.0);
            let noise_metadata_schedule_12_e308: f64 = if noise_variable_23 < noise_metadata_schedule_12_e307 { 1.0 } else { 0.0 };
            noise_variable_114 = noise_metadata_schedule_12_e308;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_13_e319,) = {
    if (noise_variable_114 != 0.0) {
        let noise_metadata_schedule_13_e313: f64 = (noise_variable_23 - params.p35);
        let noise_metadata_schedule_13_e315: f64 = (noise_metadata_schedule_13_e313 - 1.0);
        let noise_metadata_schedule_13_e316: f64 = (noise_metadata_schedule_13_e315).exp();
        let noise_metadata_schedule_13_e317: f64 = (params.p35 + noise_metadata_schedule_13_e316);
        (noise_metadata_schedule_13_e317,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_13_e319;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_14_e323: f64 = (params.p36 - 1.0);
            let noise_metadata_schedule_14_e324: f64 = if noise_variable_23 > noise_metadata_schedule_14_e323 { 1.0 } else { 0.0 };
            noise_variable_115 = noise_metadata_schedule_14_e324;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_15_e338,) = {
    if ((noise_variable_114 == 0.0) && (noise_variable_115 != 0.0)) {
        let noise_metadata_schedule_15_e332: f64 = (params.p36 - noise_variable_23);
        let noise_metadata_schedule_15_e334: f64 = (noise_metadata_schedule_15_e332 - 1.0);
        let noise_metadata_schedule_15_e335: f64 = (noise_metadata_schedule_15_e334).exp();
        let noise_metadata_schedule_15_e336: f64 = (params.p36 - noise_metadata_schedule_15_e335);
        (noise_metadata_schedule_15_e336,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_15_e338;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_16_e346,) = {
    if ((noise_variable_114 == 0.0) && (noise_variable_115 == 0.0)) {
        (noise_variable_23,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_16_e346;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_17_e349: f64 = (noise_variable_23 + 273.15);
            noise_variable_24 = noise_metadata_schedule_17_e349;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_18_e352: f64 = (1.3806505e-23 * noise_variable_24);
            let noise_metadata_schedule_18_e354: f64 = (noise_metadata_schedule_18_e352 / 1.60217653e-19);
            noise_variable_71 = noise_metadata_schedule_18_e354;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_20_e360: f64 = (noise_variable_24 - noise_variable_15);
            noise_variable_69 = noise_metadata_schedule_20_e360;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_21_e363: f64 = (params.p0 * noise_variable_13);
            noise_variable_26 = noise_metadata_schedule_21_e363;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_22_e366: f64 = (params.p1 * noise_variable_13);
            noise_variable_27 = noise_metadata_schedule_22_e366;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_27_e381: f64 = (params.p2 * noise_variable_13);
            noise_variable_30 = noise_metadata_schedule_27_e381;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_28_e384: f64 = (params.p3 * noise_variable_14);
            noise_variable_31 = noise_metadata_schedule_28_e384;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_29_e387: f64 = (params.p4 * noise_variable_13);
            noise_variable_32 = noise_metadata_schedule_29_e387;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_30_e390: f64 = (params.p6 * noise_variable_14);
            noise_variable_33 = noise_metadata_schedule_30_e390;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_31_e393: f64 = (params.p7 * noise_variable_13);
            noise_variable_34 = noise_metadata_schedule_31_e393;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_32_e396: f64 = (noise_variable_27 * noise_variable_26);
            noise_variable_35 = noise_metadata_schedule_32_e396;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_34_e413: f64 = if params.p5 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_34_e416: f64 = if params.p8 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_34_e417: f64 = (noise_metadata_schedule_34_e413 + noise_metadata_schedule_34_e416);
            let noise_metadata_schedule_34_e418: f64 = (0.5 * noise_metadata_schedule_34_e417);
            let noise_metadata_schedule_34_e422: f64 = (params.p44 / noise_variable_26);
            let noise_metadata_schedule_34_e423: f64 = (params.p43 + noise_metadata_schedule_34_e422);
            let noise_metadata_schedule_34_e424: f64 = (noise_metadata_schedule_34_e418 * noise_metadata_schedule_34_e423);
            noise_variable_25 = noise_metadata_schedule_34_e424;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_35_e427: f64 = (noise_variable_26 + params.p38);
            let noise_metadata_schedule_35_e430: f64 = (params.p39 / noise_variable_26);
            let noise_metadata_schedule_35_e431: f64 = (noise_metadata_schedule_35_e427 + noise_metadata_schedule_35_e430);
            let noise_metadata_schedule_35_e435: f64 = (-noise_variable_26);
            let noise_metadata_schedule_35_e437: f64 = (noise_metadata_schedule_35_e435 / params.p41);
            let noise_metadata_schedule_35_e438: f64 = (noise_metadata_schedule_35_e437).exp();
            let noise_metadata_schedule_35_e439: f64 = (1.0 - noise_metadata_schedule_35_e438);
            let noise_metadata_schedule_35_e440: f64 = (params.p42 * noise_metadata_schedule_35_e439);
            let noise_metadata_schedule_35_e441: f64 = (noise_metadata_schedule_35_e431 + noise_metadata_schedule_35_e440);
            let noise_metadata_schedule_35_e445: f64 = (params.p40 * noise_variable_30);
            let noise_metadata_schedule_35_e447: f64 = (noise_metadata_schedule_35_e445 / noise_variable_35);
            let noise_metadata_schedule_35_e448: f64 = (1.0 - noise_metadata_schedule_35_e447);
            let noise_metadata_schedule_35_e449: f64 = (noise_metadata_schedule_35_e441 / noise_metadata_schedule_35_e448);
            noise_variable_4 = noise_metadata_schedule_35_e449;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_36_e452: f64 = (noise_variable_27 + noise_variable_25);
            noise_variable_3 = noise_metadata_schedule_36_e452;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_37_e456,) = {
    if (params.p127 != 0.0) {
        (noise_variable_4,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_37_e456;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_38_e460,) = {
    if (params.p127 != 0.0) {
        (noise_variable_3,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_38_e460;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_39_e465,) = {
    if (params.p127 == 0.0) {
        (noise_variable_26,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_39_e465;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_40_e470,) = {
    if (params.p127 == 0.0) {
        (noise_variable_27,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_40_e470;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_41_e487,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_41_e475: f64 = (params.p119 * params.p122);
        let noise_metadata_schedule_41_e476: f64 = (noise_variable_4 + noise_metadata_schedule_41_e475);
        let noise_metadata_schedule_41_e479: f64 = (params.p11 * params.p125);
        let noise_metadata_schedule_41_e482: f64 = (noise_variable_12 * noise_variable_37);
        let noise_metadata_schedule_41_e483: f64 = (noise_metadata_schedule_41_e482).sqrt();
        let noise_metadata_schedule_41_e484: f64 = (noise_metadata_schedule_41_e479 / noise_metadata_schedule_41_e483);
        let noise_metadata_schedule_41_e485: f64 = (noise_metadata_schedule_41_e476 + noise_metadata_schedule_41_e484);
        (noise_metadata_schedule_41_e485,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_41_e487;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_42_e504,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_42_e492: f64 = (params.p120 * params.p123);
        let noise_metadata_schedule_42_e493: f64 = (noise_variable_3 + noise_metadata_schedule_42_e492);
        let noise_metadata_schedule_42_e496: f64 = (params.p12 * params.p126);
        let noise_metadata_schedule_42_e499: f64 = (noise_variable_12 * noise_variable_38);
        let noise_metadata_schedule_42_e500: f64 = (noise_metadata_schedule_42_e499).sqrt();
        let noise_metadata_schedule_42_e501: f64 = (noise_metadata_schedule_42_e496 / noise_metadata_schedule_42_e500);
        let noise_metadata_schedule_42_e502: f64 = (noise_metadata_schedule_42_e493 + noise_metadata_schedule_42_e501);
        (noise_metadata_schedule_42_e502,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_42_e504;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_43_e524,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_43_e509: f64 = (params.p118 * params.p121);
        let noise_metadata_schedule_43_e512: f64 = (params.p10 * params.p124);
        let noise_metadata_schedule_43_e515: f64 = (noise_variable_12 * noise_variable_37);
        let noise_metadata_schedule_43_e517: f64 = (noise_metadata_schedule_43_e515 * noise_variable_38);
        let noise_metadata_schedule_43_e518: f64 = (noise_metadata_schedule_43_e517).sqrt();
        let noise_metadata_schedule_43_e519: f64 = (noise_metadata_schedule_43_e512 / noise_metadata_schedule_43_e518);
        let noise_metadata_schedule_43_e520: f64 = (noise_metadata_schedule_43_e509 + noise_metadata_schedule_43_e519);
        let noise_metadata_schedule_43_e521: f64 = (0.01 * noise_metadata_schedule_43_e520);
        let noise_metadata_schedule_43_e522: f64 = (noise_metadata_schedule_43_e521).exp();
        (noise_metadata_schedule_43_e522,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_43_e524;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_44_e535: f64 = if ((params.p119 != 0.0) && ((params.p125 > 0.0) || (params.p122 > 0.0))) { 1.0 } else { 0.0 };
            noise_variable_120 = noise_metadata_schedule_44_e535;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_45_e547,) = {
    if ((params.p16 == 0.0) && (noise_variable_120 != 0.0)) {
        let noise_metadata_schedule_45_e543: f64 = (noise_variable_12 * noise_variable_37);
        let noise_metadata_schedule_45_e544: f64 = (noise_metadata_schedule_45_e543).sqrt();
        let noise_metadata_schedule_45_e545: f64 = (params.p125 / noise_metadata_schedule_45_e544);
        (noise_metadata_schedule_45_e545,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_45_e547;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_46_e565,) = {
    if ((params.p16 == 0.0) && (noise_variable_120 != 0.0)) {
        let noise_metadata_schedule_46_e556: f64 = (params.p122 * params.p122);
        let noise_metadata_schedule_46_e559: f64 = (noise_variable_39 * noise_variable_39);
        let noise_metadata_schedule_46_e560: f64 = (noise_metadata_schedule_46_e556 + noise_metadata_schedule_46_e559);
        let noise_metadata_schedule_46_e561: f64 = (noise_metadata_schedule_46_e560).sqrt();
        let noise_metadata_schedule_46_e562: f64 = (params.p119 * noise_metadata_schedule_46_e561);
        let noise_metadata_schedule_46_e563: f64 = (noise_variable_4 + noise_metadata_schedule_46_e562);
        (noise_metadata_schedule_46_e563,)
    } else {
        (noise_variable_4,)
    }
};
            noise_variable_4 = noise_metadata_schedule_46_e565;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_47_e576: f64 = if ((params.p120 != 0.0) && ((params.p126 > 0.0) || (params.p123 > 0.0))) { 1.0 } else { 0.0 };
            noise_variable_121 = noise_metadata_schedule_47_e576;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_48_e588,) = {
    if ((params.p16 == 0.0) && (noise_variable_121 != 0.0)) {
        let noise_metadata_schedule_48_e584: f64 = (noise_variable_12 * noise_variable_38);
        let noise_metadata_schedule_48_e585: f64 = (noise_metadata_schedule_48_e584).sqrt();
        let noise_metadata_schedule_48_e586: f64 = (params.p126 / noise_metadata_schedule_48_e585);
        (noise_metadata_schedule_48_e586,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_48_e588;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_49_e606,) = {
    if ((params.p16 == 0.0) && (noise_variable_121 != 0.0)) {
        let noise_metadata_schedule_49_e597: f64 = (params.p123 * params.p123);
        let noise_metadata_schedule_49_e600: f64 = (noise_variable_39 * noise_variable_39);
        let noise_metadata_schedule_49_e601: f64 = (noise_metadata_schedule_49_e597 + noise_metadata_schedule_49_e600);
        let noise_metadata_schedule_49_e602: f64 = (noise_metadata_schedule_49_e601).sqrt();
        let noise_metadata_schedule_49_e603: f64 = (params.p120 * noise_metadata_schedule_49_e602);
        let noise_metadata_schedule_49_e604: f64 = (noise_variable_3 + noise_metadata_schedule_49_e603);
        (noise_metadata_schedule_49_e604,)
    } else {
        (noise_variable_3,)
    }
};
            noise_variable_3 = noise_metadata_schedule_49_e606;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_50_e617: f64 = if ((params.p118 != 0.0) && ((params.p124 > 0.0) || (params.p121 > 0.0))) { 1.0 } else { 0.0 };
            noise_variable_122 = noise_metadata_schedule_50_e617;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_51_e631,) = {
    if ((params.p16 == 0.0) && (noise_variable_122 != 0.0)) {
        let noise_metadata_schedule_51_e625: f64 = (noise_variable_12 * noise_variable_37);
        let noise_metadata_schedule_51_e627: f64 = (noise_metadata_schedule_51_e625 * noise_variable_38);
        let noise_metadata_schedule_51_e628: f64 = (noise_metadata_schedule_51_e627).sqrt();
        let noise_metadata_schedule_51_e629: f64 = (params.p124 / noise_metadata_schedule_51_e628);
        (noise_metadata_schedule_51_e629,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_51_e631;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_52_e650,) = {
    if ((params.p16 == 0.0) && (noise_variable_122 != 0.0)) {
        let noise_metadata_schedule_52_e638: f64 = (0.01 * params.p118);
        let noise_metadata_schedule_52_e641: f64 = (params.p121 * params.p121);
        let noise_metadata_schedule_52_e644: f64 = (noise_variable_39 * noise_variable_39);
        let noise_metadata_schedule_52_e645: f64 = (noise_metadata_schedule_52_e641 + noise_metadata_schedule_52_e644);
        let noise_metadata_schedule_52_e646: f64 = (noise_metadata_schedule_52_e645).sqrt();
        let noise_metadata_schedule_52_e647: f64 = (noise_metadata_schedule_52_e638 * noise_metadata_schedule_52_e646);
        let noise_metadata_schedule_52_e648: f64 = (noise_metadata_schedule_52_e647).exp();
        (noise_metadata_schedule_52_e648,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_52_e650;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_53_e658,) = {
    if ((params.p16 == 0.0) && (noise_variable_122 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_40,)
    }
};
            noise_variable_40 = noise_metadata_schedule_53_e658;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_56_e667: f64 = (noise_variable_3 + params.p45);
            noise_variable_28 = noise_metadata_schedule_56_e667;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_58_e674,) = {
    if (params.p53 != 0.0) {
        (noise_variable_4,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_58_e674;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_59_e678,) = {
    if (params.p53 != 0.0) {
        (noise_variable_3,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_59_e678;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_60_e683,) = {
    if (params.p53 == 0.0) {
        (noise_variable_26,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_60_e683;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_61_e688,) = {
    if (params.p53 == 0.0) {
        (noise_variable_27,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_61_e688;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_62_e692: f64 = (noise_variable_38).powf(params.p56);
            let noise_metadata_schedule_62_e693: f64 = (1.0 / noise_metadata_schedule_62_e692);
            noise_variable_42 = noise_metadata_schedule_62_e693;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_63_e697: f64 = (noise_variable_37).powf(params.p58);
            let noise_metadata_schedule_63_e698: f64 = (1.0 / noise_metadata_schedule_63_e697);
            noise_variable_43 = noise_metadata_schedule_63_e698;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_64_e703: f64 = (params.p55 * noise_variable_42);
            let noise_metadata_schedule_64_e704: f64 = (1.0 + noise_metadata_schedule_64_e703);
            let noise_metadata_schedule_64_e705: f64 = (params.p54 * noise_metadata_schedule_64_e704);
            let noise_metadata_schedule_64_e709: f64 = (params.p57 * noise_variable_43);
            let noise_metadata_schedule_64_e710: f64 = (1.0 + noise_metadata_schedule_64_e709);
            let noise_metadata_schedule_64_e711: f64 = (noise_metadata_schedule_64_e705 * noise_metadata_schedule_64_e710);
            let noise_metadata_schedule_64_e715: f64 = (params.p59 * noise_variable_42);
            let noise_metadata_schedule_64_e717: f64 = (noise_metadata_schedule_64_e715 * noise_variable_43);
            let noise_metadata_schedule_64_e718: f64 = (1.0 + noise_metadata_schedule_64_e717);
            let noise_metadata_schedule_64_e719: f64 = (noise_metadata_schedule_64_e711 * noise_metadata_schedule_64_e718);
            let noise_metadata_schedule_64_e725: f64 = (noise_variable_69 * params.p104);
            let noise_metadata_schedule_64_e726: f64 = (params.p103 + noise_metadata_schedule_64_e725);
            let noise_metadata_schedule_64_e727: f64 = (noise_variable_69 * noise_metadata_schedule_64_e726);
            let noise_metadata_schedule_64_e728: f64 = (1.0 + noise_metadata_schedule_64_e727);
            let noise_metadata_schedule_64_e729: f64 = (noise_metadata_schedule_64_e719 * noise_metadata_schedule_64_e728);
            noise_variable_41 = noise_metadata_schedule_64_e729;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_65_e735,) = {
    if (noise_variable_41 > 0.1) {
        (noise_variable_41,)
    } else {
        (0.1,)
    }
};
            noise_variable_41 = noise_metadata_schedule_65_e735;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_66_e737: f64 = (noise_variable_41).sqrt();
            let noise_metadata_schedule_66_e740: f64 = (noise_variable_41 + 10000.0);
            let noise_metadata_schedule_66_e741: f64 = (noise_metadata_schedule_66_e737 / noise_metadata_schedule_66_e740);
            noise_variable_44 = noise_metadata_schedule_66_e741;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_67_e759,) = {
    if (params.p15 != 0.0) {
        (0.0,)
    } else {
        let noise_metadata_schedule_67_e747: f64 = (params.p50 * noise_variable_37);
        let noise_metadata_schedule_67_e750: f64 = (params.p51 * noise_variable_38);
        let noise_metadata_schedule_67_e751: f64 = (noise_metadata_schedule_67_e747 + noise_metadata_schedule_67_e750);
        let noise_metadata_schedule_67_e753: f64 = (noise_metadata_schedule_67_e751 + params.p52);
        let noise_metadata_schedule_67_e756: f64 = (noise_variable_37 * noise_variable_38);
        let noise_metadata_schedule_67_e757: f64 = (noise_metadata_schedule_67_e753 / noise_metadata_schedule_67_e756);
        let noise_metadata_schedule_67_e758: f64 = (params.p49 + noise_metadata_schedule_67_e757);
        (noise_metadata_schedule_67_e758,)
    }
};
            noise_variable_45 = noise_metadata_schedule_67_e759;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_68_e762: f64 = if noise_variable_45 < noise_variable_44 { 1.0 } else { 0.0 };
            noise_variable_126 = noise_metadata_schedule_68_e762;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_69_e771,) = {
    if (noise_variable_126 != 0.0) {
        let (noise_metadata_schedule_69_e769,) = {
            if (noise_variable_45 > 0.0) {
                (noise_variable_45,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_69_e769,)
    } else {
        (noise_variable_45,)
    }
};
            noise_variable_45 = noise_metadata_schedule_69_e771;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_70_e777,) = {
    if (noise_variable_126 != 0.0) {
        let noise_metadata_schedule_70_e775: f64 = (noise_variable_44 * noise_variable_44);
        (noise_metadata_schedule_70_e775,)
    } else {
        (noise_variable_46,)
    }
};
            noise_variable_46 = noise_metadata_schedule_70_e777;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_71_e784,) = {
    if (noise_variable_126 == 0.0) {
        let noise_metadata_schedule_71_e782: f64 = (noise_variable_45 * noise_variable_45);
        (noise_metadata_schedule_71_e782,)
    } else {
        (noise_variable_46,)
    }
};
            noise_variable_46 = noise_metadata_schedule_71_e784;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_72_e787: f64 = (0.5 / noise_variable_46);
            let noise_metadata_schedule_72_e790: f64 = (noise_variable_41 * 0.5);
            let noise_metadata_schedule_72_e791: f64 = (noise_metadata_schedule_72_e787 - noise_metadata_schedule_72_e790);
            noise_variable_48 = noise_metadata_schedule_72_e791;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_73_e794: f64 = if params.p63 > 1.0 { 1.0 } else { 0.0 };
            noise_variable_127 = noise_metadata_schedule_73_e794;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_74_e804,) = {
    if (noise_variable_127 != 0.0) {
        let noise_metadata_schedule_74_e799: f64 = (2.0 * params.p64);
        let noise_metadata_schedule_74_e801: f64 = (noise_metadata_schedule_74_e799 / noise_variable_46);
        let noise_metadata_schedule_74_e802: f64 = (noise_variable_48 - noise_metadata_schedule_74_e801);
        (noise_metadata_schedule_74_e802,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_74_e804;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_75_e814,) = {
    if (noise_variable_127 != 0.0) {
        let noise_metadata_schedule_75_e808: f64 = (0.1666666666666667 / noise_variable_46);
        let noise_metadata_schedule_75_e811: f64 = (noise_variable_41 * 0.5);
        let noise_metadata_schedule_75_e812: f64 = (noise_metadata_schedule_75_e808 - noise_metadata_schedule_75_e811);
        (noise_metadata_schedule_75_e812,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_75_e814;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_76_e817: f64 = if params.p63 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_128 = noise_metadata_schedule_76_e817;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_77_e831,) = {
    if ((noise_variable_127 == 0.0) && (noise_variable_128 != 0.0)) {
        let noise_metadata_schedule_77_e825: f64 = (2.0 * params.p64);
        let noise_metadata_schedule_77_e827: f64 = (noise_metadata_schedule_77_e825 / noise_variable_46);
        let noise_metadata_schedule_77_e828: f64 = (noise_metadata_schedule_77_e827).sqrt();
        let noise_metadata_schedule_77_e829: f64 = (noise_variable_48 - noise_metadata_schedule_77_e828);
        (noise_metadata_schedule_77_e829,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_77_e831;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_78_e838,) = {
    if ((noise_variable_127 == 0.0) && (noise_variable_128 != 0.0)) {
        (0.0,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_78_e838;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_79_e846,) = {
    if ((noise_variable_127 == 0.0) && (noise_variable_128 == 0.0)) {
        (noise_variable_48,)
    } else {
        (noise_variable_49,)
    }
};
            noise_variable_49 = noise_metadata_schedule_79_e846;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_80_e854,) = {
    if ((noise_variable_127 == 0.0) && (noise_variable_128 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_50,)
    }
};
            noise_variable_50 = noise_metadata_schedule_80_e854;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_81_e859: f64 = (params.p48 / noise_variable_3);
            let noise_metadata_schedule_81_e860: f64 = (1.0 + noise_metadata_schedule_81_e859);
            let noise_metadata_schedule_81_e861: f64 = (params.p47 / noise_metadata_schedule_81_e860);
            noise_variable_106 = noise_metadata_schedule_81_e861;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_82_e864: f64 = if params.p63 > 1.0 { 1.0 } else { 0.0 };
            noise_variable_129 = noise_metadata_schedule_82_e864;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_83_e870,) = {
    if (noise_variable_129 != 0.0) {
        let noise_metadata_schedule_83_e868: f64 = (params.p46 * noise_variable_71);
        (noise_metadata_schedule_83_e868,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_83_e870;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_84_e891,) = {
    if (noise_variable_129 != 0.0) {
        let (noise_metadata_schedule_84_e889,) = {
            if (params.p63 > 2.0) {
                let noise_metadata_schedule_84_e877: f64 = (0.55 * noise_variable_71);
                let noise_metadata_schedule_84_e880: f64 = (-noise_variable_106);
                let noise_metadata_schedule_84_e882: f64 = (noise_metadata_schedule_84_e880 / noise_variable_71);
                let noise_metadata_schedule_84_e883: f64 = (noise_metadata_schedule_84_e882).exp();
                let noise_metadata_schedule_84_e884: f64 = (1.0 + noise_metadata_schedule_84_e883);
                let noise_metadata_schedule_84_e885: f64 = (noise_metadata_schedule_84_e877 * noise_metadata_schedule_84_e884);
                (noise_metadata_schedule_84_e885,)
            } else {
                let noise_metadata_schedule_84_e888: f64 = (1.1 * noise_variable_71);
                (noise_metadata_schedule_84_e888,)
            }
        };
        (noise_metadata_schedule_84_e889,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_84_e891;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_85_e894: f64 = if params.p63 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_130 = noise_metadata_schedule_85_e894;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_86_e905,) = {
    if ((noise_variable_129 == 0.0) && (noise_variable_130 != 0.0)) {
        let noise_metadata_schedule_86_e901: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_86_e903: f64 = (noise_metadata_schedule_86_e901 * noise_variable_71);
        (noise_metadata_schedule_86_e903,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_86_e905;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_87_e916,) = {
    if ((noise_variable_129 == 0.0) && (noise_variable_130 != 0.0)) {
        let noise_metadata_schedule_87_e912: f64 = (4.0 * noise_variable_106);
        let noise_metadata_schedule_87_e914: f64 = (noise_metadata_schedule_87_e912 * noise_variable_106);
        (noise_metadata_schedule_87_e914,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_87_e916;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_88_e926,) = {
    if ((noise_variable_129 == 0.0) && (noise_variable_130 == 0.0)) {
        let noise_metadata_schedule_88_e924: f64 = (params.p46 * noise_variable_71);
        (noise_metadata_schedule_88_e924,)
    } else {
        (noise_variable_105,)
    }
};
            noise_variable_105 = noise_metadata_schedule_88_e926;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_89_e938,) = {
    if ((noise_variable_129 == 0.0) && (noise_variable_130 == 0.0)) {
        let noise_metadata_schedule_89_e934: f64 = (4.0 * noise_variable_106);
        let noise_metadata_schedule_89_e936: f64 = (noise_metadata_schedule_89_e934 * noise_variable_106);
        (noise_metadata_schedule_89_e936,)
    } else {
        (noise_variable_107,)
    }
};
            noise_variable_107 = noise_metadata_schedule_89_e938;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_90_e941: f64 = (params.p37 * noise_variable_40);
            let noise_metadata_schedule_90_e944: f64 = (noise_variable_3 / noise_variable_4);
            let noise_metadata_schedule_90_e945: f64 = (noise_metadata_schedule_90_e941 * noise_metadata_schedule_90_e944);
            let noise_metadata_schedule_90_e949: f64 = (noise_variable_41).sqrt();
            let noise_metadata_schedule_90_e950: f64 = (noise_variable_45 * noise_metadata_schedule_90_e949);
            let noise_metadata_schedule_90_e951: f64 = (1.0 - noise_metadata_schedule_90_e950);
            let noise_metadata_schedule_90_e952: f64 = (noise_metadata_schedule_90_e945 * noise_metadata_schedule_90_e951);
            noise_variable_5 = noise_metadata_schedule_90_e952;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_92_e962: f64 = if ((params.p66 > 0.0) && (params.p5 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_132 = noise_metadata_schedule_92_e962;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_93_e972,) = {
    if (noise_variable_132 != 0.0) {
        let noise_metadata_schedule_93_e967: f64 = (params.p67 / noise_variable_26);
        let noise_metadata_schedule_93_e968: f64 = (params.p66 + noise_metadata_schedule_93_e967);
        let noise_metadata_schedule_93_e970: f64 = (noise_metadata_schedule_93_e968 / params.p5);
        (noise_metadata_schedule_93_e970,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_93_e972;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_94_e977,) = {
    if (noise_variable_132 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_54,)
    }
};
            noise_variable_54 = noise_metadata_schedule_94_e977;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_95_e984: f64 = if ((params.p66 > 0.0) && (params.p8 > 0.0)) { 1.0 } else { 0.0 };
            noise_variable_133 = noise_metadata_schedule_95_e984;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_96_e994,) = {
    if (noise_variable_133 != 0.0) {
        let noise_metadata_schedule_96_e989: f64 = (params.p67 / noise_variable_26);
        let noise_metadata_schedule_96_e990: f64 = (params.p66 + noise_metadata_schedule_96_e989);
        let noise_metadata_schedule_96_e992: f64 = (noise_metadata_schedule_96_e990 / params.p8);
        (noise_metadata_schedule_96_e992,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_96_e994;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_97_e999,) = {
    if (noise_variable_133 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_55,)
    }
};
            noise_variable_55 = noise_metadata_schedule_97_e999;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_102_e1053: f64 = (params.p97 / noise_variable_4);
            let noise_metadata_schedule_102_e1054: f64 = (params.p93 + noise_metadata_schedule_102_e1053);
            let noise_metadata_schedule_102_e1058: f64 = if params.p5 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_102_e1061: f64 = if params.p8 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_102_e1062: f64 = (noise_metadata_schedule_102_e1058 + noise_metadata_schedule_102_e1061);
            let noise_metadata_schedule_102_e1063: f64 = (0.5 * noise_metadata_schedule_102_e1062);
            let noise_metadata_schedule_102_e1067: f64 = (params.p99 / noise_variable_4);
            let noise_metadata_schedule_102_e1068: f64 = (params.p95 + noise_metadata_schedule_102_e1067);
            let noise_metadata_schedule_102_e1069: f64 = (noise_metadata_schedule_102_e1063 * noise_metadata_schedule_102_e1068);
            let noise_metadata_schedule_102_e1071: f64 = (noise_metadata_schedule_102_e1069 / noise_variable_3);
            let noise_metadata_schedule_102_e1072: f64 = (noise_metadata_schedule_102_e1054 + noise_metadata_schedule_102_e1071);
            noise_variable_52 = noise_metadata_schedule_102_e1072;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_103_e1076: f64 = (params.p98 / noise_variable_4);
            let noise_metadata_schedule_103_e1077: f64 = (params.p94 + noise_metadata_schedule_103_e1076);
            let noise_metadata_schedule_103_e1081: f64 = if params.p5 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_103_e1084: f64 = if params.p8 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_103_e1085: f64 = (noise_metadata_schedule_103_e1081 + noise_metadata_schedule_103_e1084);
            let noise_metadata_schedule_103_e1086: f64 = (0.5 * noise_metadata_schedule_103_e1085);
            let noise_metadata_schedule_103_e1090: f64 = (params.p100 / noise_variable_4);
            let noise_metadata_schedule_103_e1091: f64 = (params.p96 + noise_metadata_schedule_103_e1090);
            let noise_metadata_schedule_103_e1092: f64 = (noise_metadata_schedule_103_e1086 * noise_metadata_schedule_103_e1091);
            let noise_metadata_schedule_103_e1094: f64 = (noise_metadata_schedule_103_e1092 / noise_variable_3);
            let noise_metadata_schedule_103_e1095: f64 = (noise_metadata_schedule_103_e1077 + noise_metadata_schedule_103_e1094);
            noise_variable_53 = noise_metadata_schedule_103_e1095;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            noise_variable_10 = (ctx.node_voltage(self.nodes[3]) - 0.0);
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_109_e1126: f64 = (-params.p21);
            let noise_metadata_schedule_109_e1128: f64 = (noise_metadata_schedule_109_e1126 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_64 = noise_metadata_schedule_109_e1128;
        }
        if matches!(source_index, 0 | 1 | 4) {
            let noise_metadata_schedule_110_e1130: f64 = (-params.p21);
            let noise_metadata_schedule_110_e1132: f64 = (noise_metadata_schedule_110_e1130 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_65 = noise_metadata_schedule_110_e1132;
        }
        if matches!(source_index, 0 | 1 | 5) {
            let noise_metadata_schedule_111_e1134: f64 = (-params.p21);
            let noise_metadata_schedule_111_e1136: f64 = (noise_metadata_schedule_111_e1134 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            noise_variable_66 = noise_metadata_schedule_111_e1136;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_112_e1137: f64 = ctx.temperature();
            let noise_metadata_schedule_112_e1139: f64 = (noise_metadata_schedule_112_e1137 + params.p9);
            let noise_metadata_schedule_112_e1141: f64 = (noise_metadata_schedule_112_e1139 + noise_variable_10);
            let noise_metadata_schedule_112_e1143: f64 = (noise_metadata_schedule_112_e1141 - 273.15);
            noise_variable_23 = noise_metadata_schedule_112_e1143;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_113_e1147: f64 = (params.p35 + 1.0);
            let noise_metadata_schedule_113_e1148: f64 = if noise_variable_23 < noise_metadata_schedule_113_e1147 { 1.0 } else { 0.0 };
            noise_variable_134 = noise_metadata_schedule_113_e1148;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_114_e1159,) = {
    if (noise_variable_134 != 0.0) {
        let noise_metadata_schedule_114_e1153: f64 = (noise_variable_23 - params.p35);
        let noise_metadata_schedule_114_e1155: f64 = (noise_metadata_schedule_114_e1153 - 1.0);
        let noise_metadata_schedule_114_e1156: f64 = (noise_metadata_schedule_114_e1155).exp();
        let noise_metadata_schedule_114_e1157: f64 = (params.p35 + noise_metadata_schedule_114_e1156);
        (noise_metadata_schedule_114_e1157,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_114_e1159;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_115_e1163: f64 = (params.p36 - 1.0);
            let noise_metadata_schedule_115_e1164: f64 = if noise_variable_23 > noise_metadata_schedule_115_e1163 { 1.0 } else { 0.0 };
            noise_variable_135 = noise_metadata_schedule_115_e1164;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_116_e1178,) = {
    if ((noise_variable_134 == 0.0) && (noise_variable_135 != 0.0)) {
        let noise_metadata_schedule_116_e1172: f64 = (params.p36 - noise_variable_23);
        let noise_metadata_schedule_116_e1174: f64 = (noise_metadata_schedule_116_e1172 - 1.0);
        let noise_metadata_schedule_116_e1175: f64 = (noise_metadata_schedule_116_e1174).exp();
        let noise_metadata_schedule_116_e1176: f64 = (params.p36 - noise_metadata_schedule_116_e1175);
        (noise_metadata_schedule_116_e1176,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_116_e1178;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let (noise_metadata_schedule_117_e1186,) = {
    if ((noise_variable_134 == 0.0) && (noise_variable_135 == 0.0)) {
        (noise_variable_23,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_117_e1186;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_118_e1189: f64 = (noise_variable_23 + 273.15);
            noise_variable_24 = noise_metadata_schedule_118_e1189;
        }
        if matches!(source_index, 4 | 5) {
            let noise_metadata_schedule_119_e1192: f64 = (1.3806505e-23 * noise_variable_24);
            let noise_metadata_schedule_119_e1194: f64 = (noise_metadata_schedule_119_e1192 / 1.60217653e-19);
            noise_variable_70 = noise_metadata_schedule_119_e1194;
        }
        if matches!(source_index, 0 | 1 | 4 | 5) {
            let noise_metadata_schedule_120_e1197: f64 = (noise_variable_24 / noise_variable_15);
            noise_variable_68 = noise_metadata_schedule_120_e1197;
        }
        if matches!(source_index, 0 | 1 | 2 | 3 | 4 | 5) {
            let noise_metadata_schedule_121_e1200: f64 = (noise_variable_24 - noise_variable_15);
            noise_variable_69 = noise_metadata_schedule_121_e1200;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_122_e1206: f64 = (noise_variable_69 * noise_variable_53);
            let noise_metadata_schedule_122_e1207: f64 = (noise_variable_52 + noise_metadata_schedule_122_e1206);
            let noise_metadata_schedule_122_e1208: f64 = (noise_variable_69 * noise_metadata_schedule_122_e1207);
            let noise_metadata_schedule_122_e1209: f64 = (1.0 + noise_metadata_schedule_122_e1208);
            noise_variable_57 = noise_metadata_schedule_122_e1209;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_123_e1213: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_123_e1214: f64 = if noise_variable_57 < noise_metadata_schedule_123_e1213 { 1.0 } else { 0.0 };
            noise_variable_136 = noise_metadata_schedule_123_e1214;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_124_e1229,) = {
    if (noise_variable_136 != 0.0) {
        let noise_metadata_schedule_124_e1221: f64 = (noise_variable_57 - 0.01);
        let noise_metadata_schedule_124_e1222: f64 = (10.0 * noise_metadata_schedule_124_e1221);
        let noise_metadata_schedule_124_e1224: f64 = (noise_metadata_schedule_124_e1222 - 1.0);
        let noise_metadata_schedule_124_e1225: f64 = (noise_metadata_schedule_124_e1224).exp();
        let noise_metadata_schedule_124_e1226: f64 = (0.1 * noise_metadata_schedule_124_e1225);
        let noise_metadata_schedule_124_e1227: f64 = (0.01 + noise_metadata_schedule_124_e1226);
        (noise_metadata_schedule_124_e1227,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_124_e1229;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_125_e1234,) = {
    if (noise_variable_136 == 0.0) {
        (noise_variable_57,)
    } else {
        (noise_variable_57,)
    }
};
            noise_variable_57 = noise_metadata_schedule_125_e1234;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_126_e1249,) = {
    if (params.p63 != 0.0) {
        let noise_metadata_schedule_126_e1241: f64 = (noise_variable_41).sqrt();
        let noise_metadata_schedule_126_e1242: f64 = (noise_variable_45 * noise_metadata_schedule_126_e1241);
        let noise_metadata_schedule_126_e1243: f64 = (1.0 - noise_metadata_schedule_126_e1242);
        let noise_metadata_schedule_126_e1244: f64 = (noise_variable_5 * noise_metadata_schedule_126_e1243);
        let noise_metadata_schedule_126_e1246: f64 = (noise_metadata_schedule_126_e1244 * noise_variable_57);
        let noise_metadata_schedule_126_e1247: f64 = (1.0 / noise_metadata_schedule_126_e1246);
        (noise_metadata_schedule_126_e1247,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_126_e1249;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_127_e1258,) = {
    if (params.p63 == 0.0) {
        let noise_metadata_schedule_127_e1255: f64 = (noise_variable_5 * noise_variable_57);
        let noise_metadata_schedule_127_e1256: f64 = (1.0 / noise_metadata_schedule_127_e1255);
        (noise_metadata_schedule_127_e1256,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_127_e1258;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_128_e1264: f64 = (noise_variable_69 * params.p102);
            let noise_metadata_schedule_128_e1265: f64 = (params.p101 + noise_metadata_schedule_128_e1264);
            let noise_metadata_schedule_128_e1266: f64 = (noise_variable_69 * noise_metadata_schedule_128_e1265);
            let noise_metadata_schedule_128_e1267: f64 = (1.0 + noise_metadata_schedule_128_e1266);
            noise_variable_58 = noise_metadata_schedule_128_e1267;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_129_e1271: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_129_e1272: f64 = if noise_variable_58 < noise_metadata_schedule_129_e1271 { 1.0 } else { 0.0 };
            noise_variable_137 = noise_metadata_schedule_129_e1272;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_130_e1287,) = {
    if (noise_variable_137 != 0.0) {
        let noise_metadata_schedule_130_e1279: f64 = (noise_variable_58 - 0.01);
        let noise_metadata_schedule_130_e1280: f64 = (10.0 * noise_metadata_schedule_130_e1279);
        let noise_metadata_schedule_130_e1282: f64 = (noise_metadata_schedule_130_e1280 - 1.0);
        let noise_metadata_schedule_130_e1283: f64 = (noise_metadata_schedule_130_e1282).exp();
        let noise_metadata_schedule_130_e1284: f64 = (0.1 * noise_metadata_schedule_130_e1283);
        let noise_metadata_schedule_130_e1285: f64 = (0.01 + noise_metadata_schedule_130_e1284);
        (noise_metadata_schedule_130_e1285,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_130_e1287;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_131_e1292,) = {
    if (noise_variable_137 == 0.0) {
        (noise_variable_58,)
    } else {
        (noise_variable_58,)
    }
};
            noise_variable_58 = noise_metadata_schedule_131_e1292;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_132_e1295: f64 = (noise_variable_68).powf(params.p92);
            noise_variable_59 = noise_metadata_schedule_132_e1295;
        }
        if matches!(source_index, 4 | 5) {
            let noise_metadata_schedule_133_e1298: f64 = if params.p69 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_138 = noise_metadata_schedule_133_e1298;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_134_e1319,) = {
    if (noise_variable_138 != 0.0) {
        let noise_metadata_schedule_134_e1302: f64 = (-params.p90);
        let noise_metadata_schedule_134_e1305: f64 = (1.0 - noise_variable_68);
        let noise_metadata_schedule_134_e1306: f64 = (noise_metadata_schedule_134_e1302 * noise_metadata_schedule_134_e1305);
        let noise_metadata_schedule_134_e1308: f64 = (noise_metadata_schedule_134_e1306 / noise_variable_70);
        let noise_metadata_schedule_134_e1311: f64 = (noise_variable_68).ln();
        let noise_metadata_schedule_134_e1312: f64 = (params.p91 * noise_metadata_schedule_134_e1311);
        let noise_metadata_schedule_134_e1313: f64 = (noise_metadata_schedule_134_e1308 + noise_metadata_schedule_134_e1312);
        let noise_metadata_schedule_134_e1315: f64 = (noise_metadata_schedule_134_e1313 / params.p70);
        let noise_metadata_schedule_134_e1316: f64 = (noise_metadata_schedule_134_e1315).exp();
        let noise_metadata_schedule_134_e1317: f64 = (params.p69 * noise_metadata_schedule_134_e1316);
        (noise_metadata_schedule_134_e1317,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_134_e1319;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_135_e1332,) = {
    if (noise_variable_138 != 0.0) {
        let noise_metadata_schedule_135_e1323: f64 = (params.p70 * noise_variable_70);
        let noise_metadata_schedule_135_e1327: f64 = (params.p27 / noise_variable_74);
        let noise_metadata_schedule_135_e1328: f64 = (1.0 + noise_metadata_schedule_135_e1327);
        let noise_metadata_schedule_135_e1329: f64 = (noise_metadata_schedule_135_e1328).ln();
        let noise_metadata_schedule_135_e1330: f64 = (noise_metadata_schedule_135_e1323 * noise_metadata_schedule_135_e1329);
        (noise_metadata_schedule_135_e1330,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_135_e1332;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_136_e1337,) = {
    if (noise_variable_138 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_74,)
    }
};
            noise_variable_74 = noise_metadata_schedule_136_e1337;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_137_e1342,) = {
    if (noise_variable_138 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_61,)
    }
};
            noise_variable_61 = noise_metadata_schedule_137_e1342;
        }
        if matches!(source_index, 4 | 5) {
            let noise_metadata_schedule_138_e1345: f64 = if params.p76 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_139 = noise_metadata_schedule_138_e1345;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_139_e1366,) = {
    if (noise_variable_139 != 0.0) {
        let noise_metadata_schedule_139_e1349: f64 = (-params.p90);
        let noise_metadata_schedule_139_e1352: f64 = (1.0 - noise_variable_68);
        let noise_metadata_schedule_139_e1353: f64 = (noise_metadata_schedule_139_e1349 * noise_metadata_schedule_139_e1352);
        let noise_metadata_schedule_139_e1355: f64 = (noise_metadata_schedule_139_e1353 / noise_variable_70);
        let noise_metadata_schedule_139_e1358: f64 = (noise_variable_68).ln();
        let noise_metadata_schedule_139_e1359: f64 = (params.p91 * noise_metadata_schedule_139_e1358);
        let noise_metadata_schedule_139_e1360: f64 = (noise_metadata_schedule_139_e1355 + noise_metadata_schedule_139_e1359);
        let noise_metadata_schedule_139_e1362: f64 = (noise_metadata_schedule_139_e1360 / params.p77);
        let noise_metadata_schedule_139_e1363: f64 = (noise_metadata_schedule_139_e1362).exp();
        let noise_metadata_schedule_139_e1364: f64 = (params.p76 * noise_metadata_schedule_139_e1363);
        (noise_metadata_schedule_139_e1364,)
    } else {
        (noise_variable_75,)
    }
};
            noise_variable_75 = noise_metadata_schedule_139_e1366;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_140_e1379,) = {
    if (noise_variable_139 != 0.0) {
        let noise_metadata_schedule_140_e1370: f64 = (params.p77 * noise_variable_70);
        let noise_metadata_schedule_140_e1374: f64 = (params.p27 / noise_variable_75);
        let noise_metadata_schedule_140_e1375: f64 = (1.0 + noise_metadata_schedule_140_e1374);
        let noise_metadata_schedule_140_e1376: f64 = (noise_metadata_schedule_140_e1375).ln();
        let noise_metadata_schedule_140_e1377: f64 = (noise_metadata_schedule_140_e1370 * noise_metadata_schedule_140_e1376);
        (noise_metadata_schedule_140_e1377,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_140_e1379;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_141_e1384,) = {
    if (noise_variable_139 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_75,)
    }
};
            noise_variable_75 = noise_metadata_schedule_141_e1384;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_142_e1389,) = {
    if (noise_variable_139 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_142_e1389;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_143_e1392: f64 = (noise_variable_31 * noise_variable_74);
            let noise_metadata_schedule_143_e1395: f64 = (noise_variable_32 * noise_variable_75);
            let noise_metadata_schedule_143_e1396: f64 = (noise_metadata_schedule_143_e1392 + noise_metadata_schedule_143_e1395);
            noise_variable_84 = noise_metadata_schedule_143_e1396;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_144_e1399: f64 = (noise_variable_33 * noise_variable_74);
            let noise_metadata_schedule_144_e1402: f64 = (noise_variable_34 * noise_variable_75);
            let noise_metadata_schedule_144_e1403: f64 = (noise_metadata_schedule_144_e1399 + noise_metadata_schedule_144_e1402);
            noise_variable_85 = noise_metadata_schedule_144_e1403;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_159_e1595: f64 = (noise_variable_69 * params.p108);
            let noise_metadata_schedule_159_e1596: f64 = (1.0 + noise_metadata_schedule_159_e1595);
            let noise_metadata_schedule_159_e1598: f64 = (noise_metadata_schedule_159_e1596 * params.p86);
            noise_variable_80 = noise_metadata_schedule_159_e1598;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_160_e1604,) = {
    if (noise_variable_80 > 0.0) {
        (noise_variable_80,)
    } else {
        (0.0,)
    }
};
            noise_variable_80 = noise_metadata_schedule_160_e1604;
        }
        if matches!(source_index, 4 | 5) {
            let noise_metadata_schedule_161_e1607: f64 = if params.p83 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_146 = noise_metadata_schedule_161_e1607;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_162_e1621,) = {
    if (noise_variable_146 != 0.0) {
        let noise_metadata_schedule_162_e1615: f64 = (noise_variable_69 * params.p106);
        let noise_metadata_schedule_162_e1616: f64 = (params.p105 + noise_metadata_schedule_162_e1615);
        let noise_metadata_schedule_162_e1617: f64 = (noise_variable_69 * noise_metadata_schedule_162_e1616);
        let noise_metadata_schedule_162_e1618: f64 = (1.0 + noise_metadata_schedule_162_e1617);
        let noise_metadata_schedule_162_e1619: f64 = (params.p83 * noise_metadata_schedule_162_e1618);
        (noise_metadata_schedule_162_e1619,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_162_e1621;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_163_e1630,) = {
    if (noise_variable_146 != 0.0) {
        let (noise_metadata_schedule_163_e1628,) = {
            if (noise_variable_103 > 0.0) {
                (noise_variable_103,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_163_e1628,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_163_e1630;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_164_e1640,) = {
    if (noise_variable_146 != 0.0) {
        let noise_metadata_schedule_164_e1636: f64 = (params.p107 * noise_variable_69);
        let noise_metadata_schedule_164_e1637: f64 = (1.0 + noise_metadata_schedule_164_e1636);
        let noise_metadata_schedule_164_e1638: f64 = (params.p85 * noise_metadata_schedule_164_e1637);
        (noise_metadata_schedule_164_e1638,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_164_e1640;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_165_e1659,) = {
    if (noise_variable_146 != 0.0) {
        let noise_metadata_schedule_165_e1644: f64 = (noise_variable_104 * noise_variable_70);
        let noise_metadata_schedule_165_e1646: f64 = (-noise_variable_103);
        let noise_metadata_schedule_165_e1649: f64 = (noise_variable_104 * noise_variable_70);
        let noise_metadata_schedule_165_e1650: f64 = (noise_metadata_schedule_165_e1646 / noise_metadata_schedule_165_e1649);
        let noise_metadata_schedule_165_e1651: f64 = (noise_metadata_schedule_165_e1650).exp();
        let noise_metadata_schedule_165_e1654: f64 = (params.p27 / params.p84);
        let noise_metadata_schedule_165_e1655: f64 = (noise_metadata_schedule_165_e1651 + noise_metadata_schedule_165_e1654);
        let noise_metadata_schedule_165_e1656: f64 = (noise_metadata_schedule_165_e1655).ln();
        let noise_metadata_schedule_165_e1657: f64 = (noise_metadata_schedule_165_e1644 * noise_metadata_schedule_165_e1656);
        (noise_metadata_schedule_165_e1657,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_165_e1659;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_166_e1664,) = {
    if (noise_variable_146 == 0.0) {
        (params.p83,)
    } else {
        (noise_variable_103,)
    }
};
            noise_variable_103 = noise_metadata_schedule_166_e1664;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_167_e1669,) = {
    if (noise_variable_146 == 0.0) {
        (params.p85,)
    } else {
        (noise_variable_104,)
    }
};
            noise_variable_104 = noise_metadata_schedule_167_e1669;
        }
        if matches!(source_index, 4 | 5) {
            let (noise_metadata_schedule_168_e1674,) = {
    if (noise_variable_146 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_62,)
    }
};
            noise_variable_62 = noise_metadata_schedule_168_e1674;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_169_e1680: f64 = if ((params.p60 > 0.0) && (params.p15 == 0.0)) { 1.0 } else { 0.0 };
            noise_variable_147 = noise_metadata_schedule_169_e1680;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_170_e1690,) = {
    if ((noise_variable_147 != 0.0) && (params.p62 != 0.0)) {
        let noise_metadata_schedule_170_e1686: f64 = (params.p61 * noise_variable_59);
        let noise_metadata_schedule_170_e1688: f64 = (noise_metadata_schedule_170_e1686 * noise_variable_57);
        (noise_metadata_schedule_170_e1688,)
    } else {
        (noise_variable_72,)
    }
};
            noise_variable_72 = noise_metadata_schedule_170_e1690;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_171_e1700,) = {
    if ((noise_variable_147 != 0.0) && (params.p62 != 0.0)) {
        let noise_metadata_schedule_171_e1696: f64 = (params.p60 * noise_variable_59);
        let noise_metadata_schedule_171_e1698: f64 = (noise_metadata_schedule_171_e1696 * noise_variable_57);
        (noise_metadata_schedule_171_e1698,)
    } else {
        (noise_variable_73,)
    }
};
            noise_variable_73 = noise_metadata_schedule_171_e1700;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_172_e1707,) = {
    if ((noise_variable_147 != 0.0) && (params.p62 == 0.0)) {
        (params.p61,)
    } else {
        (noise_variable_72,)
    }
};
            noise_variable_72 = noise_metadata_schedule_172_e1707;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_173_e1714,) = {
    if ((noise_variable_147 != 0.0) && (params.p62 == 0.0)) {
        (params.p60,)
    } else {
        (noise_variable_73,)
    }
};
            noise_variable_73 = noise_metadata_schedule_173_e1714;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_174_e1737,) = {
    if (noise_variable_147 != 0.0) {
        let noise_metadata_schedule_174_e1718: f64 = (noise_variable_72 * noise_variable_72);
        let noise_metadata_schedule_174_e1721: f64 = (4.0 * params.p65);
        let noise_metadata_schedule_174_e1723: f64 = (noise_metadata_schedule_174_e1721 * params.p65);
        let noise_metadata_schedule_174_e1725: f64 = (noise_metadata_schedule_174_e1723 * noise_variable_73);
        let noise_metadata_schedule_174_e1727: f64 = (noise_metadata_schedule_174_e1725 * noise_variable_73);
        let noise_metadata_schedule_174_e1728: f64 = (noise_metadata_schedule_174_e1718 + noise_metadata_schedule_174_e1727);
        let noise_metadata_schedule_174_e1729: f64 = (noise_metadata_schedule_174_e1728).sqrt();
        let noise_metadata_schedule_174_e1732: f64 = (2.0 * params.p65);
        let noise_metadata_schedule_174_e1734: f64 = (noise_metadata_schedule_174_e1732 * noise_variable_73);
        let noise_metadata_schedule_174_e1735: f64 = (noise_metadata_schedule_174_e1729 - noise_metadata_schedule_174_e1734);
        (noise_metadata_schedule_174_e1735,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_174_e1737;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_175_e1745,) = {
    if (noise_variable_147 != 0.0) {
        let noise_metadata_schedule_175_e1741: f64 = (params.p65 * noise_variable_19);
        let noise_metadata_schedule_175_e1743: f64 = (noise_metadata_schedule_175_e1741 / noise_variable_73);
        (noise_metadata_schedule_175_e1743,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_175_e1745;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_176_e1760,) = {
    if (noise_variable_147 != 0.0) {
        let noise_metadata_schedule_176_e1749: f64 = (noise_variable_19 * noise_variable_19);
        let noise_metadata_schedule_176_e1752: f64 = (noise_variable_73 * noise_variable_73);
        let noise_metadata_schedule_176_e1753: f64 = (noise_metadata_schedule_176_e1749 / noise_metadata_schedule_176_e1752);
        let noise_metadata_schedule_176_e1756: f64 = (4.0 * noise_variable_20);
        let noise_metadata_schedule_176_e1757: f64 = (noise_metadata_schedule_176_e1753 + noise_metadata_schedule_176_e1756);
        let noise_metadata_schedule_176_e1758: f64 = (noise_metadata_schedule_176_e1757).sqrt();
        (noise_metadata_schedule_176_e1758,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_176_e1760;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_177_e1766,) = {
    if (noise_variable_147 != 0.0) {
        let noise_metadata_schedule_177_e1764: f64 = (noise_variable_73 - noise_variable_72);
        (noise_metadata_schedule_177_e1764,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_177_e1766;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_178_e1772,) = {
    if (noise_variable_147 != 0.0) {
        let noise_metadata_schedule_178_e1770: f64 = (1.0 / noise_variable_73);
        (noise_metadata_schedule_178_e1770,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_178_e1772;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_179_e1777,) = {
    if (noise_variable_147 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_19,)
    }
};
            noise_variable_19 = noise_metadata_schedule_179_e1777;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_180_e1782,) = {
    if (noise_variable_147 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_20,)
    }
};
            noise_variable_20 = noise_metadata_schedule_180_e1782;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_181_e1787,) = {
    if (noise_variable_147 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_21,)
    }
};
            noise_variable_21 = noise_metadata_schedule_181_e1787;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_182_e1792,) = {
    if (noise_variable_147 == 0.0) {
        (1000.0,)
    } else {
        (noise_variable_22,)
    }
};
            noise_variable_22 = noise_metadata_schedule_182_e1792;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_183_e1797,) = {
    if (noise_variable_147 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_18,)
    }
};
            noise_variable_18 = noise_metadata_schedule_183_e1797;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_184_e1800: f64 = (noise_variable_28 * noise_variable_22);
            noise_variable_51 = noise_metadata_schedule_184_e1800;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_185_e1803: f64 = if noise_variable_51 > 100000.0 { 1.0 } else { 0.0 };
            noise_variable_148 = noise_metadata_schedule_185_e1803;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_186_e1807,) = {
    if (noise_variable_148 != 0.0) {
        (100000.0,)
    } else {
        (noise_variable_51,)
    }
};
            noise_variable_51 = noise_metadata_schedule_186_e1807;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_187_e1810: f64 = if noise_variable_64 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_199 = noise_metadata_schedule_187_e1810;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_188_e1815,) = {
    if (noise_variable_199 != 0.0) {
        let noise_metadata_schedule_188_e1813: f64 = (-1.0);
        (noise_metadata_schedule_188_e1813,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_188_e1815;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_189_e1820,) = {
    if (noise_variable_199 != 0.0) {
        let noise_metadata_schedule_189_e1818: f64 = (-noise_variable_66);
        (noise_metadata_schedule_189_e1818,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_189_e1820;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_190_e1825,) = {
    if (noise_variable_199 != 0.0) {
        let noise_metadata_schedule_190_e1823: f64 = (-noise_variable_64);
        (noise_metadata_schedule_190_e1823,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_190_e1825;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_191_e1830,) = {
    if (noise_variable_199 == 0.0) {
        (1.0,)
    } else {
        (noise_variable_149,)
    }
};
            noise_variable_149 = noise_metadata_schedule_191_e1830;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_192_e1836,) = {
    if (noise_variable_199 == 0.0) {
        let noise_metadata_schedule_192_e1834: f64 = (-noise_variable_65);
        (noise_metadata_schedule_192_e1834,)
    } else {
        (noise_variable_150,)
    }
};
            noise_variable_150 = noise_metadata_schedule_192_e1836;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_193_e1841,) = {
    if (noise_variable_199 == 0.0) {
        (noise_variable_64,)
    } else {
        (noise_variable_151,)
    }
};
            noise_variable_151 = noise_metadata_schedule_193_e1841;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_194_e1844: f64 = if noise_variable_150 > noise_variable_49 { 1.0 } else { 0.0 };
            noise_variable_200 = noise_metadata_schedule_194_e1844;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_195_e1860,) = {
    if (noise_variable_200 != 0.0) {
        let noise_metadata_schedule_195_e1851: f64 = (noise_variable_49 - noise_variable_150);
        let noise_metadata_schedule_195_e1853: f64 = (noise_metadata_schedule_195_e1851 / noise_variable_105);
        let noise_metadata_schedule_195_e1854: f64 = (noise_metadata_schedule_195_e1853).exp();
        let noise_metadata_schedule_195_e1855: f64 = (1.0 + noise_metadata_schedule_195_e1854);
        let noise_metadata_schedule_195_e1856: f64 = (noise_metadata_schedule_195_e1855).ln();
        let noise_metadata_schedule_195_e1857: f64 = (noise_variable_105 * noise_metadata_schedule_195_e1856);
        let noise_metadata_schedule_195_e1858: f64 = (noise_variable_49 - noise_metadata_schedule_195_e1857);
        (noise_metadata_schedule_195_e1858,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_195_e1860;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_196_e1877,) = {
    if (noise_variable_200 == 0.0) {
        let noise_metadata_schedule_196_e1868: f64 = (noise_variable_150 - noise_variable_49);
        let noise_metadata_schedule_196_e1870: f64 = (noise_metadata_schedule_196_e1868 / noise_variable_105);
        let noise_metadata_schedule_196_e1871: f64 = (noise_metadata_schedule_196_e1870).exp();
        let noise_metadata_schedule_196_e1872: f64 = (1.0 + noise_metadata_schedule_196_e1871);
        let noise_metadata_schedule_196_e1873: f64 = (noise_metadata_schedule_196_e1872).ln();
        let noise_metadata_schedule_196_e1874: f64 = (noise_variable_105 * noise_metadata_schedule_196_e1873);
        let noise_metadata_schedule_196_e1875: f64 = (noise_variable_150 - noise_metadata_schedule_196_e1874);
        (noise_metadata_schedule_196_e1875,)
    } else {
        (noise_variable_152,)
    }
};
            noise_variable_152 = noise_metadata_schedule_196_e1877;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_197_e1880: f64 = (-0.4);
            let noise_metadata_schedule_197_e1885: f64 = (noise_variable_49 - noise_variable_152);
            let (noise_metadata_schedule_197_e1891,) = {
    if (noise_variable_151 < noise_metadata_schedule_197_e1885) {
        (noise_variable_151,)
    } else {
        let noise_metadata_schedule_197_e1890: f64 = (noise_variable_49 - noise_variable_152);
        (noise_metadata_schedule_197_e1890,)
    }
};
            let noise_metadata_schedule_197_e1892: f64 = (noise_variable_41 + noise_metadata_schedule_197_e1891);
            let noise_metadata_schedule_197_e1893: f64 = (noise_metadata_schedule_197_e1880 * noise_metadata_schedule_197_e1892);
            let noise_metadata_schedule_197_e1894: f64 = if noise_variable_152 < noise_metadata_schedule_197_e1893 { 1.0 } else { 0.0 };
            noise_variable_201 = noise_metadata_schedule_197_e1894;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_198_e1914,) = {
    if ((params.p63 != 0.0) && (noise_variable_201 != 0.0)) {
        let noise_metadata_schedule_198_e1899: f64 = (-0.4);
        let noise_metadata_schedule_198_e1904: f64 = (noise_variable_49 - noise_variable_152);
        let (noise_metadata_schedule_198_e1910,) = {
            if (noise_variable_151 < noise_metadata_schedule_198_e1904) {
                (noise_variable_151,)
            } else {
                let noise_metadata_schedule_198_e1909: f64 = (noise_variable_49 - noise_variable_152);
                (noise_metadata_schedule_198_e1909,)
            }
        };
        let noise_metadata_schedule_198_e1911: f64 = (noise_variable_41 + noise_metadata_schedule_198_e1910);
        let noise_metadata_schedule_198_e1912: f64 = (noise_metadata_schedule_198_e1899 * noise_metadata_schedule_198_e1911);
        (noise_metadata_schedule_198_e1912,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_198_e1914;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_199_e1921,) = {
    if ((params.p63 != 0.0) && (noise_variable_201 == 0.0)) {
        (noise_variable_152,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_199_e1921;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_200_e1924: f64 = (-0.4);
            let noise_metadata_schedule_200_e1926: f64 = (noise_metadata_schedule_200_e1924 * noise_variable_41);
            let noise_metadata_schedule_200_e1927: f64 = if noise_variable_152 < noise_metadata_schedule_200_e1926 { 1.0 } else { 0.0 };
            noise_variable_202 = noise_metadata_schedule_200_e1927;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_201_e1937,) = {
    if ((params.p63 == 0.0) && (noise_variable_202 != 0.0)) {
        let noise_metadata_schedule_201_e1933: f64 = (-0.4);
        let noise_metadata_schedule_201_e1935: f64 = (noise_metadata_schedule_201_e1933 * noise_variable_41);
        (noise_metadata_schedule_201_e1935,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_201_e1937;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_202_e1945,) = {
    if ((params.p63 == 0.0) && (noise_variable_202 == 0.0)) {
        (noise_variable_152,)
    } else {
        (noise_variable_153,)
    }
};
            noise_variable_153 = noise_metadata_schedule_202_e1945;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_203_e1949: f64 = (2.0 * noise_variable_153);
            let noise_metadata_schedule_203_e1950: f64 = (noise_variable_41 + noise_metadata_schedule_203_e1949);
            noise_variable_154 = noise_metadata_schedule_203_e1950;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_204_e1953: f64 = if noise_variable_18 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_203 = noise_metadata_schedule_204_e1953;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_205_e1963,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_205_e1957: f64 = (noise_variable_46 * noise_variable_154);
        let noise_metadata_schedule_205_e1959: f64 = (noise_metadata_schedule_205_e1957 * noise_variable_154);
        let noise_metadata_schedule_205_e1961: f64 = (noise_metadata_schedule_205_e1959 - noise_variable_154);
        (noise_metadata_schedule_205_e1961,)
    } else {
        (noise_variable_155,)
    }
};
            noise_variable_155 = noise_metadata_schedule_205_e1963;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_206_e1974,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_206_e1966: f64 = (-1.0);
        let noise_metadata_schedule_206_e1969: f64 = (3.0 * noise_variable_46);
        let noise_metadata_schedule_206_e1971: f64 = (noise_metadata_schedule_206_e1969 * noise_variable_154);
        let noise_metadata_schedule_206_e1972: f64 = (noise_metadata_schedule_206_e1966 + noise_metadata_schedule_206_e1971);
        (noise_metadata_schedule_206_e1972,)
    } else {
        (params.p3,)
    }
};
            noise_variable_156 = noise_metadata_schedule_206_e1974;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_207_e1986,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_207_e1979: f64 = (9.0 / 4.0);
        let noise_metadata_schedule_207_e1982: f64 = (noise_variable_154 / noise_variable_51);
        let noise_metadata_schedule_207_e1983: f64 = (noise_metadata_schedule_207_e1979 + noise_metadata_schedule_207_e1982);
        let noise_metadata_schedule_207_e1984: f64 = (noise_variable_46 * noise_metadata_schedule_207_e1983);
        (noise_metadata_schedule_207_e1984,)
    } else {
        (params.p6,)
    }
};
            noise_variable_157 = noise_metadata_schedule_207_e1986;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_208_e1994,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_208_e1990: f64 = (1.5 * noise_variable_46);
        let noise_metadata_schedule_208_e1992: f64 = (noise_metadata_schedule_208_e1990 / noise_variable_51);
        (noise_metadata_schedule_208_e1992,)
    } else {
        (noise_variable_158,)
    }
};
            noise_variable_158 = noise_metadata_schedule_208_e1994;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_209_e2004,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_209_e1998: f64 = (4.0 * noise_variable_51);
        let noise_metadata_schedule_209_e2000: f64 = (noise_metadata_schedule_209_e1998 * noise_variable_51);
        let noise_metadata_schedule_209_e2002: f64 = (noise_metadata_schedule_209_e2000 / noise_variable_46);
        (noise_metadata_schedule_209_e2002,)
    } else {
        (noise_variable_159,)
    }
};
            noise_variable_159 = noise_metadata_schedule_209_e2004;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_210_e2010,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_210_e2008: f64 = (noise_variable_155 * noise_variable_159);
        (noise_metadata_schedule_210_e2008,)
    } else {
        (noise_variable_160,)
    }
};
            noise_variable_160 = noise_metadata_schedule_210_e2010;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_211_e2016,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_211_e2014: f64 = (params.p3 * noise_variable_159);
        (noise_metadata_schedule_211_e2014,)
    } else {
        (noise_variable_161,)
    }
};
            noise_variable_161 = noise_metadata_schedule_211_e2016;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_212_e2022,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_212_e2020: f64 = (params.p6 * noise_variable_159);
        (noise_metadata_schedule_212_e2020,)
    } else {
        (noise_variable_162,)
    }
};
            noise_variable_162 = noise_metadata_schedule_212_e2022;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_213_e2028,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_213_e2026: f64 = (noise_variable_158 * noise_variable_159);
        (noise_metadata_schedule_213_e2026,)
    } else {
        (noise_variable_163,)
    }
};
            noise_variable_163 = noise_metadata_schedule_213_e2028;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_214_e2034,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_214_e2032: f64 = (noise_variable_163 * noise_variable_163);
        (noise_metadata_schedule_214_e2032,)
    } else {
        (noise_variable_164,)
    }
};
            noise_variable_164 = noise_metadata_schedule_214_e2034;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_215_e2039,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_215_e2037: f64 = (-noise_variable_162);
        (noise_metadata_schedule_215_e2037,)
    } else {
        (noise_variable_165,)
    }
};
            noise_variable_165 = noise_metadata_schedule_215_e2039;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_216_e2049,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_216_e2043: f64 = (noise_variable_163 * noise_variable_161);
        let noise_metadata_schedule_216_e2046: f64 = (4.0 * noise_variable_160);
        let noise_metadata_schedule_216_e2047: f64 = (noise_metadata_schedule_216_e2043 - noise_metadata_schedule_216_e2046);
        (noise_metadata_schedule_216_e2047,)
    } else {
        (noise_variable_166,)
    }
};
            noise_variable_166 = noise_metadata_schedule_216_e2049;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_217_e2065,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_217_e2053: f64 = (4.0 * noise_variable_162);
        let noise_metadata_schedule_217_e2055: f64 = (noise_metadata_schedule_217_e2053 * noise_variable_160);
        let noise_metadata_schedule_217_e2058: f64 = (noise_variable_161 * noise_variable_161);
        let noise_metadata_schedule_217_e2059: f64 = (noise_metadata_schedule_217_e2055 - noise_metadata_schedule_217_e2058);
        let noise_metadata_schedule_217_e2062: f64 = (noise_variable_160 * noise_variable_164);
        let noise_metadata_schedule_217_e2063: f64 = (noise_metadata_schedule_217_e2059 - noise_metadata_schedule_217_e2062);
        (noise_metadata_schedule_217_e2063,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_217_e2065;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_218_e2075,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_218_e2070: f64 = (noise_variable_165 * noise_variable_165);
        let noise_metadata_schedule_218_e2072: f64 = (noise_metadata_schedule_218_e2070 * 0.3333333333333333);
        let noise_metadata_schedule_218_e2073: f64 = (noise_variable_166 - noise_metadata_schedule_218_e2072);
        (noise_metadata_schedule_218_e2073,)
    } else {
        (noise_variable_168,)
    }
};
            noise_variable_168 = noise_metadata_schedule_218_e2075;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_219_e2089,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_219_e2082: f64 = (2.0 * noise_variable_168);
        let noise_metadata_schedule_219_e2083: f64 = (noise_variable_166 + noise_metadata_schedule_219_e2082);
        let noise_metadata_schedule_219_e2084: f64 = (noise_variable_165 * noise_metadata_schedule_219_e2083);
        let noise_metadata_schedule_219_e2086: f64 = (noise_metadata_schedule_219_e2084 / 9.0);
        let noise_metadata_schedule_219_e2087: f64 = (noise_variable_167 - noise_metadata_schedule_219_e2086);
        (noise_metadata_schedule_219_e2087,)
    } else {
        (noise_variable_169,)
    }
};
            noise_variable_169 = noise_metadata_schedule_219_e2089;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_220_e2099,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_220_e2093: f64 = (noise_variable_168 * noise_variable_168);
        let noise_metadata_schedule_220_e2095: f64 = (noise_metadata_schedule_220_e2093 * noise_variable_168);
        let noise_metadata_schedule_220_e2097: f64 = (noise_metadata_schedule_220_e2095 / 27.0);
        (noise_metadata_schedule_220_e2097,)
    } else {
        (noise_variable_170,)
    }
};
            noise_variable_170 = noise_metadata_schedule_220_e2099;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_221_e2109,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_221_e2103: f64 = (0.25 * noise_variable_169);
        let noise_metadata_schedule_221_e2105: f64 = (noise_metadata_schedule_221_e2103 * noise_variable_169);
        let noise_metadata_schedule_221_e2107: f64 = (noise_metadata_schedule_221_e2105 + noise_variable_170);
        (noise_metadata_schedule_221_e2107,)
    } else {
        (noise_variable_171,)
    }
};
            noise_variable_171 = noise_metadata_schedule_221_e2109;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_222_e2114,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_222_e2112: f64 = (noise_variable_171).sqrt();
        (noise_metadata_schedule_222_e2112,)
    } else {
        (noise_variable_172,)
    }
};
            noise_variable_172 = noise_metadata_schedule_222_e2114;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_223_e2117: f64 = if noise_variable_169 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_204 = noise_metadata_schedule_223_e2117;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_224_e2128,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_204 != 0.0)) {
        let noise_metadata_schedule_224_e2122: f64 = (-0.5);
        let noise_metadata_schedule_224_e2124: f64 = (noise_metadata_schedule_224_e2122 * noise_variable_169);
        let noise_metadata_schedule_224_e2126: f64 = (noise_metadata_schedule_224_e2124 + noise_variable_172);
        (noise_metadata_schedule_224_e2126,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_224_e2128;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_225_e2137,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_204 != 0.0)) {
        let noise_metadata_schedule_225_e2133: f64 = (-noise_variable_170);
        let noise_metadata_schedule_225_e2135: f64 = (noise_metadata_schedule_225_e2133 / noise_variable_173);
        (noise_metadata_schedule_225_e2135,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_225_e2137;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_226_e2149,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_204 == 0.0)) {
        let noise_metadata_schedule_226_e2143: f64 = (-0.5);
        let noise_metadata_schedule_226_e2145: f64 = (noise_metadata_schedule_226_e2143 * noise_variable_169);
        let noise_metadata_schedule_226_e2147: f64 = (noise_metadata_schedule_226_e2145 - noise_variable_172);
        (noise_metadata_schedule_226_e2147,)
    } else {
        (noise_variable_174,)
    }
};
            noise_variable_174 = noise_metadata_schedule_226_e2149;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_227_e2159,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_204 == 0.0)) {
        let noise_metadata_schedule_227_e2155: f64 = (-noise_variable_170);
        let noise_metadata_schedule_227_e2157: f64 = (noise_metadata_schedule_227_e2155 / noise_variable_174);
        (noise_metadata_schedule_227_e2157,)
    } else {
        (noise_variable_173,)
    }
};
            noise_variable_173 = noise_metadata_schedule_227_e2159;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_228_e2162: f64 = if noise_variable_173 > 1e-6 { 1.0 } else { 0.0 };
            noise_variable_205 = noise_metadata_schedule_228_e2162;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_229_e2170,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_205 != 0.0)) {
        let noise_metadata_schedule_229_e2168: f64 = (noise_variable_173).powf(0.3333333333333333);
        (noise_metadata_schedule_229_e2168,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_229_e2170;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_230_e2173: f64 = (-1e-6);
            let noise_metadata_schedule_230_e2174: f64 = if noise_variable_173 < noise_metadata_schedule_230_e2173 { 1.0 } else { 0.0 };
            noise_variable_206 = noise_metadata_schedule_230_e2174;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_231_e2187,) = {
    if (((noise_variable_203 != 0.0) && (noise_variable_205 == 0.0)) && (noise_variable_206 != 0.0)) {
        let noise_metadata_schedule_231_e2182: f64 = (-noise_variable_173);
        let noise_metadata_schedule_231_e2184: f64 = (noise_metadata_schedule_231_e2182).powf(0.3333333333333333);
        let noise_metadata_schedule_231_e2185: f64 = (-noise_metadata_schedule_231_e2184);
        (noise_metadata_schedule_231_e2185,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_231_e2187;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_232_e2199,) = {
    if (((noise_variable_203 != 0.0) && (noise_variable_205 == 0.0)) && (noise_variable_206 == 0.0)) {
        let noise_metadata_schedule_232_e2197: f64 = (10000.0 * noise_variable_173);
        (noise_metadata_schedule_232_e2197,)
    } else {
        (noise_variable_175,)
    }
};
            noise_variable_175 = noise_metadata_schedule_232_e2199;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_233_e2202: f64 = if noise_variable_174 > 1e-6 { 1.0 } else { 0.0 };
            noise_variable_207 = noise_metadata_schedule_233_e2202;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_234_e2210,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_207 != 0.0)) {
        let noise_metadata_schedule_234_e2208: f64 = (noise_variable_174).powf(0.3333333333333333);
        (noise_metadata_schedule_234_e2208,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_234_e2210;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_235_e2213: f64 = (-1e-6);
            let noise_metadata_schedule_235_e2214: f64 = if noise_variable_174 < noise_metadata_schedule_235_e2213 { 1.0 } else { 0.0 };
            noise_variable_208 = noise_metadata_schedule_235_e2214;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_236_e2227,) = {
    if (((noise_variable_203 != 0.0) && (noise_variable_207 == 0.0)) && (noise_variable_208 != 0.0)) {
        let noise_metadata_schedule_236_e2222: f64 = (-noise_variable_174);
        let noise_metadata_schedule_236_e2224: f64 = (noise_metadata_schedule_236_e2222).powf(0.3333333333333333);
        let noise_metadata_schedule_236_e2225: f64 = (-noise_metadata_schedule_236_e2224);
        (noise_metadata_schedule_236_e2225,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_236_e2227;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_237_e2239,) = {
    if (((noise_variable_203 != 0.0) && (noise_variable_207 == 0.0)) && (noise_variable_208 == 0.0)) {
        let noise_metadata_schedule_237_e2237: f64 = (10000.0 * noise_variable_174);
        (noise_metadata_schedule_237_e2237,)
    } else {
        (noise_variable_176,)
    }
};
            noise_variable_176 = noise_metadata_schedule_237_e2239;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_238_e2249,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_238_e2243: f64 = (noise_variable_175 + noise_variable_176);
        let noise_metadata_schedule_238_e2246: f64 = (noise_variable_165 * 0.3333333333333333);
        let noise_metadata_schedule_238_e2247: f64 = (noise_metadata_schedule_238_e2243 - noise_metadata_schedule_238_e2246);
        (noise_metadata_schedule_238_e2247,)
    } else {
        (noise_variable_177,)
    }
};
            noise_variable_177 = noise_metadata_schedule_238_e2249;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_239_e2260,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_239_e2253: f64 = (0.25 * noise_variable_164);
        let noise_metadata_schedule_239_e2255: f64 = (noise_metadata_schedule_239_e2253 - noise_variable_162);
        let noise_metadata_schedule_239_e2257: f64 = (noise_metadata_schedule_239_e2255 + noise_variable_177);
        let noise_metadata_schedule_239_e2258: f64 = (noise_metadata_schedule_239_e2257).sqrt();
        (noise_metadata_schedule_239_e2258,)
    } else {
        (noise_variable_167,)
    }
};
            noise_variable_167 = noise_metadata_schedule_239_e2260;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_240_e2274,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_240_e2264: f64 = (0.75 * noise_variable_164);
        let noise_metadata_schedule_240_e2267: f64 = (noise_variable_167 * noise_variable_167);
        let noise_metadata_schedule_240_e2268: f64 = (noise_metadata_schedule_240_e2264 - noise_metadata_schedule_240_e2267);
        let noise_metadata_schedule_240_e2271: f64 = (2.0 * noise_variable_162);
        let noise_metadata_schedule_240_e2272: f64 = (noise_metadata_schedule_240_e2268 - noise_metadata_schedule_240_e2271);
        (noise_metadata_schedule_240_e2272,)
    } else {
        (noise_variable_178,)
    }
};
            noise_variable_178 = noise_metadata_schedule_240_e2274;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_241_e2292,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_241_e2278: f64 = (noise_variable_163 * noise_variable_162);
        let noise_metadata_schedule_241_e2281: f64 = (2.0 * noise_variable_161);
        let noise_metadata_schedule_241_e2282: f64 = (noise_metadata_schedule_241_e2278 - noise_metadata_schedule_241_e2281);
        let noise_metadata_schedule_241_e2285: f64 = (0.25 * noise_variable_164);
        let noise_metadata_schedule_241_e2287: f64 = (noise_metadata_schedule_241_e2285 * noise_variable_163);
        let noise_metadata_schedule_241_e2288: f64 = (noise_metadata_schedule_241_e2282 - noise_metadata_schedule_241_e2287);
        let noise_metadata_schedule_241_e2290: f64 = (noise_metadata_schedule_241_e2288 / noise_variable_167);
        (noise_metadata_schedule_241_e2290,)
    } else {
        (noise_variable_179,)
    }
};
            noise_variable_179 = noise_metadata_schedule_241_e2292;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_242_e2298,) = {
    if (noise_variable_203 != 0.0) {
        let noise_metadata_schedule_242_e2296: f64 = (noise_variable_178 + noise_variable_179);
        (noise_metadata_schedule_242_e2296,)
    } else {
        (noise_variable_180,)
    }
};
            noise_variable_180 = noise_metadata_schedule_242_e2298;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_243_e2301: f64 = if noise_variable_180 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_209 = noise_metadata_schedule_243_e2301;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_244_e2308,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_209 != 0.0)) {
        let noise_metadata_schedule_244_e2306: f64 = (noise_variable_180).sqrt();
        (noise_metadata_schedule_244_e2306,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_244_e2308;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_245_e2323,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_209 != 0.0)) {
        let noise_metadata_schedule_245_e2313: f64 = (-0.25);
        let noise_metadata_schedule_245_e2315: f64 = (noise_metadata_schedule_245_e2313 * noise_variable_163);
        let noise_metadata_schedule_245_e2319: f64 = (noise_variable_182 + noise_variable_167);
        let noise_metadata_schedule_245_e2320: f64 = (0.5 * noise_metadata_schedule_245_e2319);
        let noise_metadata_schedule_245_e2321: f64 = (noise_metadata_schedule_245_e2315 + noise_metadata_schedule_245_e2320);
        (noise_metadata_schedule_245_e2321,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_245_e2323;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_246_e2332,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_209 == 0.0)) {
        let noise_metadata_schedule_246_e2330: f64 = (noise_variable_178 - noise_variable_179);
        (noise_metadata_schedule_246_e2330,)
    } else {
        (noise_variable_181,)
    }
};
            noise_variable_181 = noise_metadata_schedule_246_e2332;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_247_e2345,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_209 == 0.0)) {
        let noise_metadata_schedule_247_e2339: f64 = (noise_variable_181 * noise_variable_181);
        let noise_metadata_schedule_247_e2341: f64 = (noise_metadata_schedule_247_e2339 + 0.0001);
        let noise_metadata_schedule_247_e2342: f64 = (noise_metadata_schedule_247_e2341).sqrt();
        let noise_metadata_schedule_247_e2343: f64 = (noise_metadata_schedule_247_e2342).sqrt();
        (noise_metadata_schedule_247_e2343,)
    } else {
        (noise_variable_182,)
    }
};
            noise_variable_182 = noise_metadata_schedule_247_e2345;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_248_e2361,) = {
    if ((noise_variable_203 != 0.0) && (noise_variable_209 == 0.0)) {
        let noise_metadata_schedule_248_e2351: f64 = (-0.25);
        let noise_metadata_schedule_248_e2353: f64 = (noise_metadata_schedule_248_e2351 * noise_variable_163);
        let noise_metadata_schedule_248_e2357: f64 = (noise_variable_182 - noise_variable_167);
        let noise_metadata_schedule_248_e2358: f64 = (0.5 * noise_metadata_schedule_248_e2357);
        let noise_metadata_schedule_248_e2359: f64 = (noise_metadata_schedule_248_e2353 + noise_metadata_schedule_248_e2358);
        (noise_metadata_schedule_248_e2359,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_248_e2361;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_249_e2364: f64 = if noise_variable_153 > noise_variable_50 { 1.0 } else { 0.0 };
            noise_variable_210 = noise_metadata_schedule_249_e2364;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_250_e2375,) = {
    if ((noise_variable_203 == 0.0) && (noise_variable_210 != 0.0)) {
        let noise_metadata_schedule_250_e2372: f64 = (noise_variable_48 - noise_variable_153);
        let noise_metadata_schedule_250_e2373: f64 = (noise_variable_46 * noise_metadata_schedule_250_e2372);
        (noise_metadata_schedule_250_e2373,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_250_e2375;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_251_e2405,) = {
    if ((noise_variable_203 == 0.0) && (noise_variable_210 != 0.0)) {
        let noise_metadata_schedule_251_e2384: f64 = (2.0 * noise_variable_198);
        let noise_metadata_schedule_251_e2385: f64 = (1.0 - noise_metadata_schedule_251_e2384);
        let noise_metadata_schedule_251_e2386: f64 = (2.0 * noise_metadata_schedule_251_e2385);
        let noise_metadata_schedule_251_e2389: f64 = (noise_variable_48 - noise_variable_153);
        let noise_metadata_schedule_251_e2390: f64 = (noise_metadata_schedule_251_e2386 * noise_metadata_schedule_251_e2389);
        let noise_metadata_schedule_251_e2394: f64 = (3.0 * noise_variable_198);
        let noise_metadata_schedule_251_e2395: f64 = (1.0 - noise_metadata_schedule_251_e2394);
        let noise_metadata_schedule_251_e2399: f64 = (1.5 * noise_variable_198);
        let noise_metadata_schedule_251_e2400: f64 = (1.0 - noise_metadata_schedule_251_e2399);
        let noise_metadata_schedule_251_e2401: f64 = (noise_metadata_schedule_251_e2400).sqrt();
        let noise_metadata_schedule_251_e2402: f64 = (noise_metadata_schedule_251_e2395 + noise_metadata_schedule_251_e2401);
        let noise_metadata_schedule_251_e2403: f64 = (noise_metadata_schedule_251_e2390 / noise_metadata_schedule_251_e2402);
        (noise_metadata_schedule_251_e2403,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_251_e2405;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_252_e2417,) = {
    if ((noise_variable_203 == 0.0) && (noise_variable_210 == 0.0)) {
        let noise_metadata_schedule_252_e2413: f64 = (3.0 * noise_variable_46);
        let noise_metadata_schedule_252_e2415: f64 = (noise_metadata_schedule_252_e2413 * noise_variable_154);
        (noise_metadata_schedule_252_e2415,)
    } else {
        (noise_variable_198,)
    }
};
            noise_variable_198 = noise_metadata_schedule_252_e2417;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_253_e2436,) = {
    if ((noise_variable_203 == 0.0) && (noise_variable_210 == 0.0)) {
        let noise_metadata_schedule_253_e2425: f64 = (1.0 - noise_variable_198);
        let noise_metadata_schedule_253_e2428: f64 = (1.0 + noise_variable_198);
        let noise_metadata_schedule_253_e2429: f64 = (noise_metadata_schedule_253_e2428).sqrt();
        let noise_metadata_schedule_253_e2430: f64 = (noise_metadata_schedule_253_e2425 + noise_metadata_schedule_253_e2429);
        let noise_metadata_schedule_253_e2433: f64 = (4.5 * noise_variable_46);
        let noise_metadata_schedule_253_e2434: f64 = (noise_metadata_schedule_253_e2430 / noise_metadata_schedule_253_e2433);
        (noise_metadata_schedule_253_e2434,)
    } else {
        (noise_variable_183,)
    }
};
            noise_variable_183 = noise_metadata_schedule_253_e2436;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_254_e2443: f64 = if ((params.p63 > 1.0) && (noise_variable_45 > 1e-9)) { 1.0 } else { 0.0 };
            noise_variable_211 = noise_metadata_schedule_254_e2443;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_255_e2449,) = {
    if (noise_variable_211 != 0.0) {
        let noise_metadata_schedule_255_e2447: f64 = (noise_variable_183 + noise_variable_71);
        (noise_metadata_schedule_255_e2447,)
    } else {
        (noise_variable_193,)
    }
};
            noise_variable_193 = noise_metadata_schedule_255_e2449;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_256_e2458,) = {
    if (noise_variable_211 != 0.0) {
        let noise_metadata_schedule_256_e2454: f64 = (noise_variable_154 + noise_variable_183);
        let noise_metadata_schedule_256_e2455: f64 = (noise_metadata_schedule_256_e2454).sqrt();
        let noise_metadata_schedule_256_e2456: f64 = (noise_variable_45 * noise_metadata_schedule_256_e2455);
        (noise_metadata_schedule_256_e2456,)
    } else {
        (noise_variable_194,)
    }
};
            noise_variable_194 = noise_metadata_schedule_256_e2458;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_257_e2461: f64 = if noise_variable_18 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_212 = noise_metadata_schedule_257_e2461;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_258_e2475,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 != 0.0)) {
        let noise_metadata_schedule_258_e2468: f64 = (noise_variable_193 / noise_variable_28);
        let noise_metadata_schedule_258_e2470: f64 = (noise_metadata_schedule_258_e2468 - noise_variable_19);
        let noise_metadata_schedule_258_e2471: f64 = (0.5 * noise_metadata_schedule_258_e2470);
        let noise_metadata_schedule_258_e2473: f64 = (noise_metadata_schedule_258_e2471 * noise_variable_18);
        (noise_metadata_schedule_258_e2473,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_258_e2475;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_259_e2489,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 != 0.0)) {
        let noise_metadata_schedule_259_e2482: f64 = (noise_variable_193 / noise_variable_28);
        let noise_metadata_schedule_259_e2484: f64 = (noise_metadata_schedule_259_e2482 + noise_variable_19);
        let noise_metadata_schedule_259_e2485: f64 = (0.5 * noise_metadata_schedule_259_e2484);
        let noise_metadata_schedule_259_e2487: f64 = (noise_metadata_schedule_259_e2485 * noise_variable_18);
        (noise_metadata_schedule_259_e2487,)
    } else {
        (noise_variable_186,)
    }
};
            noise_variable_186 = noise_metadata_schedule_259_e2489;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_260_e2500,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 != 0.0)) {
        let noise_metadata_schedule_260_e2495: f64 = (noise_variable_185 * noise_variable_185);
        let noise_metadata_schedule_260_e2497: f64 = (noise_metadata_schedule_260_e2495 + noise_variable_20);
        let noise_metadata_schedule_260_e2498: f64 = (noise_metadata_schedule_260_e2497).sqrt();
        (noise_metadata_schedule_260_e2498,)
    } else {
        (noise_variable_188,)
    }
};
            noise_variable_188 = noise_metadata_schedule_260_e2500;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_261_e2511,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 != 0.0)) {
        let noise_metadata_schedule_261_e2506: f64 = (noise_variable_186 * noise_variable_186);
        let noise_metadata_schedule_261_e2508: f64 = (noise_metadata_schedule_261_e2506 + noise_variable_20);
        let noise_metadata_schedule_261_e2509: f64 = (noise_metadata_schedule_261_e2508).sqrt();
        (noise_metadata_schedule_261_e2509,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_261_e2511;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_262_e2521,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 != 0.0)) {
        let noise_metadata_schedule_262_e2517: f64 = (noise_variable_188 + noise_variable_187);
        let noise_metadata_schedule_262_e2519: f64 = (noise_metadata_schedule_262_e2517 - noise_variable_21);
        (noise_metadata_schedule_262_e2519,)
    } else {
        (noise_variable_189,)
    }
};
            noise_variable_189 = noise_metadata_schedule_262_e2521;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_263_e2539,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 != 0.0)) {
        let noise_metadata_schedule_263_e2528: f64 = (noise_variable_185 / noise_variable_188);
        let noise_metadata_schedule_263_e2531: f64 = (noise_variable_186 / noise_variable_187);
        let noise_metadata_schedule_263_e2532: f64 = (noise_metadata_schedule_263_e2528 + noise_metadata_schedule_263_e2531);
        let noise_metadata_schedule_263_e2533: f64 = (0.5 * noise_metadata_schedule_263_e2532);
        let noise_metadata_schedule_263_e2535: f64 = (noise_metadata_schedule_263_e2533 * noise_variable_18);
        let noise_metadata_schedule_263_e2537: f64 = (noise_metadata_schedule_263_e2535 / noise_variable_28);
        (noise_metadata_schedule_263_e2537,)
    } else {
        (noise_variable_195,)
    }
};
            noise_variable_195 = noise_metadata_schedule_263_e2539;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_264_e2564,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 != 0.0)) {
        let noise_metadata_schedule_264_e2545: f64 = (2.0 * noise_variable_194);
        let noise_metadata_schedule_264_e2548: f64 = (1.0 - noise_variable_194);
        let noise_metadata_schedule_264_e2549: f64 = (noise_metadata_schedule_264_e2545 * noise_metadata_schedule_264_e2548);
        let noise_metadata_schedule_264_e2553: f64 = (noise_variable_195 * noise_variable_193);
        let noise_metadata_schedule_264_e2556: f64 = (1.0 + noise_variable_189);
        let noise_metadata_schedule_264_e2557: f64 = (noise_metadata_schedule_264_e2553 / noise_metadata_schedule_264_e2556);
        let noise_metadata_schedule_264_e2558: f64 = (1.0 - noise_metadata_schedule_264_e2557);
        let noise_metadata_schedule_264_e2559: f64 = (noise_metadata_schedule_264_e2549 * noise_metadata_schedule_264_e2558);
        let noise_metadata_schedule_264_e2561: f64 = (noise_metadata_schedule_264_e2559 / noise_variable_193);
        let noise_metadata_schedule_264_e2562: f64 = (noise_metadata_schedule_264_e2561).sqrt();
        (noise_metadata_schedule_264_e2562,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_264_e2564;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_265_e2580,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_212 == 0.0)) {
        let noise_metadata_schedule_265_e2571: f64 = (2.0 * noise_variable_194);
        let noise_metadata_schedule_265_e2574: f64 = (1.0 - noise_variable_194);
        let noise_metadata_schedule_265_e2575: f64 = (noise_metadata_schedule_265_e2571 * noise_metadata_schedule_265_e2574);
        let noise_metadata_schedule_265_e2577: f64 = (noise_metadata_schedule_265_e2575 / noise_variable_193);
        let noise_metadata_schedule_265_e2578: f64 = (noise_metadata_schedule_265_e2577).sqrt();
        (noise_metadata_schedule_265_e2578,)
    } else {
        (noise_variable_196,)
    }
};
            noise_variable_196 = noise_metadata_schedule_265_e2580;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_266_e2594,) = {
    if (noise_variable_211 != 0.0) {
        let noise_metadata_schedule_266_e2585: f64 = (noise_variable_154 + noise_variable_183);
        let noise_metadata_schedule_266_e2586: f64 = (noise_variable_46 * noise_metadata_schedule_266_e2585);
        let noise_metadata_schedule_266_e2589: f64 = (noise_variable_196 * noise_variable_196);
        let noise_metadata_schedule_266_e2590: f64 = (noise_metadata_schedule_266_e2586 / noise_metadata_schedule_266_e2589);
        let noise_metadata_schedule_266_e2592: f64 = (noise_metadata_schedule_266_e2590 - noise_variable_193);
        (noise_metadata_schedule_266_e2592,)
    } else {
        (noise_variable_197,)
    }
};
            noise_variable_197 = noise_metadata_schedule_266_e2594;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_267_e2606,) = {
    if (noise_variable_211 != 0.0) {
        let noise_metadata_schedule_267_e2599: f64 = (params.p47 * noise_variable_183);
        let noise_metadata_schedule_267_e2602: f64 = (params.p47 + noise_variable_193);
        let noise_metadata_schedule_267_e2603: f64 = (noise_metadata_schedule_267_e2599 / noise_metadata_schedule_267_e2602);
        let noise_metadata_schedule_267_e2604: f64 = (noise_variable_107 + noise_metadata_schedule_267_e2603);
        (noise_metadata_schedule_267_e2604,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_267_e2606;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_268_e2614,) = {
    if (noise_variable_211 != 0.0) {
        let noise_metadata_schedule_268_e2610: f64 = (4.0 * noise_variable_191);
        let noise_metadata_schedule_268_e2612: f64 = (noise_metadata_schedule_268_e2610 * noise_variable_191);
        (noise_metadata_schedule_268_e2612,)
    } else {
        (noise_variable_192,)
    }
};
            noise_variable_192 = noise_metadata_schedule_268_e2614;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_269_e2644,) = {
    if (noise_variable_211 != 0.0) {
        let noise_metadata_schedule_269_e2618: f64 = (2.0 * noise_variable_151);
        let noise_metadata_schedule_269_e2620: f64 = (noise_metadata_schedule_269_e2618 * noise_variable_193);
        let noise_metadata_schedule_269_e2623: f64 = (noise_variable_151 - noise_variable_193);
        let noise_metadata_schedule_269_e2626: f64 = (noise_variable_151 - noise_variable_193);
        let noise_metadata_schedule_269_e2627: f64 = (noise_metadata_schedule_269_e2623 * noise_metadata_schedule_269_e2626);
        let noise_metadata_schedule_269_e2629: f64 = (noise_metadata_schedule_269_e2627 + noise_variable_192);
        let noise_metadata_schedule_269_e2630: f64 = (noise_metadata_schedule_269_e2629).sqrt();
        let noise_metadata_schedule_269_e2633: f64 = (noise_variable_151 + noise_variable_193);
        let noise_metadata_schedule_269_e2636: f64 = (noise_variable_151 + noise_variable_193);
        let noise_metadata_schedule_269_e2637: f64 = (noise_metadata_schedule_269_e2633 * noise_metadata_schedule_269_e2636);
        let noise_metadata_schedule_269_e2639: f64 = (noise_metadata_schedule_269_e2637 + noise_variable_192);
        let noise_metadata_schedule_269_e2640: f64 = (noise_metadata_schedule_269_e2639).sqrt();
        let noise_metadata_schedule_269_e2641: f64 = (noise_metadata_schedule_269_e2630 + noise_metadata_schedule_269_e2640);
        let noise_metadata_schedule_269_e2642: f64 = (noise_metadata_schedule_269_e2620 / noise_metadata_schedule_269_e2641);
        (noise_metadata_schedule_269_e2642,)
    } else {
        (noise_variable_184,)
    }
};
            noise_variable_184 = noise_metadata_schedule_269_e2644;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_270_e2647: f64 = if params.p63 > 2.0 { 1.0 } else { 0.0 };
            noise_variable_213 = noise_metadata_schedule_270_e2647;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_271_e2661,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_213 != 0.0)) {
        let noise_metadata_schedule_271_e2654: f64 = (params.p47 * noise_variable_184);
        let noise_metadata_schedule_271_e2657: f64 = (params.p47 + noise_variable_193);
        let noise_metadata_schedule_271_e2658: f64 = (noise_metadata_schedule_271_e2654 / noise_metadata_schedule_271_e2657);
        let noise_metadata_schedule_271_e2659: f64 = (noise_variable_107 + noise_metadata_schedule_271_e2658);
        (noise_metadata_schedule_271_e2659,)
    } else {
        (noise_variable_191,)
    }
};
            noise_variable_191 = noise_metadata_schedule_271_e2661;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_272_e2671,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_213 != 0.0)) {
        let noise_metadata_schedule_272_e2667: f64 = (4.0 * noise_variable_191);
        let noise_metadata_schedule_272_e2669: f64 = (noise_metadata_schedule_272_e2667 * noise_variable_191);
        (noise_metadata_schedule_272_e2669,)
    } else {
        (noise_variable_192,)
    }
};
            noise_variable_192 = noise_metadata_schedule_272_e2671;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_273_e2703,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_213 != 0.0)) {
        let noise_metadata_schedule_273_e2677: f64 = (2.0 * noise_variable_151);
        let noise_metadata_schedule_273_e2679: f64 = (noise_metadata_schedule_273_e2677 * noise_variable_193);
        let noise_metadata_schedule_273_e2682: f64 = (noise_variable_151 - noise_variable_193);
        let noise_metadata_schedule_273_e2685: f64 = (noise_variable_151 - noise_variable_193);
        let noise_metadata_schedule_273_e2686: f64 = (noise_metadata_schedule_273_e2682 * noise_metadata_schedule_273_e2685);
        let noise_metadata_schedule_273_e2688: f64 = (noise_metadata_schedule_273_e2686 + noise_variable_192);
        let noise_metadata_schedule_273_e2689: f64 = (noise_metadata_schedule_273_e2688).sqrt();
        let noise_metadata_schedule_273_e2692: f64 = (noise_variable_151 + noise_variable_193);
        let noise_metadata_schedule_273_e2695: f64 = (noise_variable_151 + noise_variable_193);
        let noise_metadata_schedule_273_e2696: f64 = (noise_metadata_schedule_273_e2692 * noise_metadata_schedule_273_e2695);
        let noise_metadata_schedule_273_e2698: f64 = (noise_metadata_schedule_273_e2696 + noise_variable_192);
        let noise_metadata_schedule_273_e2699: f64 = (noise_metadata_schedule_273_e2698).sqrt();
        let noise_metadata_schedule_273_e2700: f64 = (noise_metadata_schedule_273_e2689 + noise_metadata_schedule_273_e2699);
        let noise_metadata_schedule_273_e2701: f64 = (noise_metadata_schedule_273_e2679 / noise_metadata_schedule_273_e2700);
        (noise_metadata_schedule_273_e2701,)
    } else {
        (noise_variable_184,)
    }
};
            noise_variable_184 = noise_metadata_schedule_273_e2703;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_274_e2714,) = {
    if (noise_variable_211 != 0.0) {
        let noise_metadata_schedule_274_e2709: f64 = (noise_variable_197 + noise_variable_184);
        let noise_metadata_schedule_274_e2710: f64 = (noise_metadata_schedule_274_e2709).sqrt();
        let noise_metadata_schedule_274_e2711: f64 = (noise_variable_196 * noise_metadata_schedule_274_e2710);
        let noise_metadata_schedule_274_e2712: f64 = (1.0 - noise_metadata_schedule_274_e2711);
        (noise_metadata_schedule_274_e2712,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_274_e2714;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_275_e2717: f64 = if noise_variable_18 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_214 = noise_metadata_schedule_275_e2717;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_276_e2731,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_214 != 0.0)) {
        let noise_metadata_schedule_276_e2724: f64 = (noise_variable_184 / noise_variable_28);
        let noise_metadata_schedule_276_e2726: f64 = (noise_metadata_schedule_276_e2724 - noise_variable_19);
        let noise_metadata_schedule_276_e2727: f64 = (0.5 * noise_metadata_schedule_276_e2726);
        let noise_metadata_schedule_276_e2729: f64 = (noise_metadata_schedule_276_e2727 * noise_variable_18);
        (noise_metadata_schedule_276_e2729,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_276_e2731;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_277_e2745,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_214 != 0.0)) {
        let noise_metadata_schedule_277_e2738: f64 = (noise_variable_184 / noise_variable_28);
        let noise_metadata_schedule_277_e2740: f64 = (noise_metadata_schedule_277_e2738 + noise_variable_19);
        let noise_metadata_schedule_277_e2741: f64 = (0.5 * noise_metadata_schedule_277_e2740);
        let noise_metadata_schedule_277_e2743: f64 = (noise_metadata_schedule_277_e2741 * noise_variable_18);
        (noise_metadata_schedule_277_e2743,)
    } else {
        (noise_variable_186,)
    }
};
            noise_variable_186 = noise_metadata_schedule_277_e2745;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_278_e2756,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_214 != 0.0)) {
        let noise_metadata_schedule_278_e2751: f64 = (noise_variable_185 * noise_variable_185);
        let noise_metadata_schedule_278_e2753: f64 = (noise_metadata_schedule_278_e2751 + noise_variable_20);
        let noise_metadata_schedule_278_e2754: f64 = (noise_metadata_schedule_278_e2753).sqrt();
        (noise_metadata_schedule_278_e2754,)
    } else {
        (noise_variable_188,)
    }
};
            noise_variable_188 = noise_metadata_schedule_278_e2756;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_279_e2767,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_214 != 0.0)) {
        let noise_metadata_schedule_279_e2762: f64 = (noise_variable_186 * noise_variable_186);
        let noise_metadata_schedule_279_e2764: f64 = (noise_metadata_schedule_279_e2762 + noise_variable_20);
        let noise_metadata_schedule_279_e2765: f64 = (noise_metadata_schedule_279_e2764).sqrt();
        (noise_metadata_schedule_279_e2765,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_279_e2767;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_280_e2777,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_214 != 0.0)) {
        let noise_metadata_schedule_280_e2773: f64 = (noise_variable_188 + noise_variable_187);
        let noise_metadata_schedule_280_e2775: f64 = (noise_metadata_schedule_280_e2773 - noise_variable_21);
        (noise_metadata_schedule_280_e2775,)
    } else {
        (noise_variable_189,)
    }
};
            noise_variable_189 = noise_metadata_schedule_280_e2777;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_281_e2784,) = {
    if ((noise_variable_211 != 0.0) && (noise_variable_214 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_189,)
    }
};
            noise_variable_189 = noise_metadata_schedule_281_e2784;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_282_e2815,) = {
    if (noise_variable_211 == 0.0) {
        let noise_metadata_schedule_282_e2789: f64 = (2.0 * noise_variable_151);
        let noise_metadata_schedule_282_e2791: f64 = (noise_metadata_schedule_282_e2789 * noise_variable_183);
        let noise_metadata_schedule_282_e2794: f64 = (noise_variable_151 - noise_variable_183);
        let noise_metadata_schedule_282_e2797: f64 = (noise_variable_151 - noise_variable_183);
        let noise_metadata_schedule_282_e2798: f64 = (noise_metadata_schedule_282_e2794 * noise_metadata_schedule_282_e2797);
        let noise_metadata_schedule_282_e2800: f64 = (noise_metadata_schedule_282_e2798 + noise_variable_107);
        let noise_metadata_schedule_282_e2801: f64 = (noise_metadata_schedule_282_e2800).sqrt();
        let noise_metadata_schedule_282_e2804: f64 = (noise_variable_151 + noise_variable_183);
        let noise_metadata_schedule_282_e2807: f64 = (noise_variable_151 + noise_variable_183);
        let noise_metadata_schedule_282_e2808: f64 = (noise_metadata_schedule_282_e2804 * noise_metadata_schedule_282_e2807);
        let noise_metadata_schedule_282_e2810: f64 = (noise_metadata_schedule_282_e2808 + noise_variable_107);
        let noise_metadata_schedule_282_e2811: f64 = (noise_metadata_schedule_282_e2810).sqrt();
        let noise_metadata_schedule_282_e2812: f64 = (noise_metadata_schedule_282_e2801 + noise_metadata_schedule_282_e2811);
        let noise_metadata_schedule_282_e2813: f64 = (noise_metadata_schedule_282_e2791 / noise_metadata_schedule_282_e2812);
        (noise_metadata_schedule_282_e2813,)
    } else {
        (noise_variable_184,)
    }
};
            noise_variable_184 = noise_metadata_schedule_282_e2815;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_283_e2818: f64 = if noise_variable_18 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_215 = noise_metadata_schedule_283_e2818;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_284_e2833,) = {
    if ((noise_variable_211 == 0.0) && (noise_variable_215 != 0.0)) {
        let noise_metadata_schedule_284_e2826: f64 = (noise_variable_184 / noise_variable_28);
        let noise_metadata_schedule_284_e2828: f64 = (noise_metadata_schedule_284_e2826 - noise_variable_19);
        let noise_metadata_schedule_284_e2829: f64 = (0.5 * noise_metadata_schedule_284_e2828);
        let noise_metadata_schedule_284_e2831: f64 = (noise_metadata_schedule_284_e2829 * noise_variable_18);
        (noise_metadata_schedule_284_e2831,)
    } else {
        (noise_variable_185,)
    }
};
            noise_variable_185 = noise_metadata_schedule_284_e2833;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_285_e2848,) = {
    if ((noise_variable_211 == 0.0) && (noise_variable_215 != 0.0)) {
        let noise_metadata_schedule_285_e2841: f64 = (noise_variable_184 / noise_variable_28);
        let noise_metadata_schedule_285_e2843: f64 = (noise_metadata_schedule_285_e2841 + noise_variable_19);
        let noise_metadata_schedule_285_e2844: f64 = (0.5 * noise_metadata_schedule_285_e2843);
        let noise_metadata_schedule_285_e2846: f64 = (noise_metadata_schedule_285_e2844 * noise_variable_18);
        (noise_metadata_schedule_285_e2846,)
    } else {
        (noise_variable_186,)
    }
};
            noise_variable_186 = noise_metadata_schedule_285_e2848;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_286_e2860,) = {
    if ((noise_variable_211 == 0.0) && (noise_variable_215 != 0.0)) {
        let noise_metadata_schedule_286_e2855: f64 = (noise_variable_185 * noise_variable_185);
        let noise_metadata_schedule_286_e2857: f64 = (noise_metadata_schedule_286_e2855 + noise_variable_20);
        let noise_metadata_schedule_286_e2858: f64 = (noise_metadata_schedule_286_e2857).sqrt();
        (noise_metadata_schedule_286_e2858,)
    } else {
        (noise_variable_188,)
    }
};
            noise_variable_188 = noise_metadata_schedule_286_e2860;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_287_e2872,) = {
    if ((noise_variable_211 == 0.0) && (noise_variable_215 != 0.0)) {
        let noise_metadata_schedule_287_e2867: f64 = (noise_variable_186 * noise_variable_186);
        let noise_metadata_schedule_287_e2869: f64 = (noise_metadata_schedule_287_e2867 + noise_variable_20);
        let noise_metadata_schedule_287_e2870: f64 = (noise_metadata_schedule_287_e2869).sqrt();
        (noise_metadata_schedule_287_e2870,)
    } else {
        (noise_variable_187,)
    }
};
            noise_variable_187 = noise_metadata_schedule_287_e2872;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_288_e2883,) = {
    if ((noise_variable_211 == 0.0) && (noise_variable_215 != 0.0)) {
        let noise_metadata_schedule_288_e2879: f64 = (noise_variable_188 + noise_variable_187);
        let noise_metadata_schedule_288_e2881: f64 = (noise_metadata_schedule_288_e2879 - noise_variable_21);
        (noise_metadata_schedule_288_e2881,)
    } else {
        (noise_variable_189,)
    }
};
            noise_variable_189 = noise_metadata_schedule_288_e2883;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_289_e2891,) = {
    if ((noise_variable_211 == 0.0) && (noise_variable_215 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_189,)
    }
};
            noise_variable_189 = noise_metadata_schedule_289_e2891;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_290_e2903,) = {
    if (noise_variable_211 == 0.0) {
        let noise_metadata_schedule_290_e2898: f64 = (noise_variable_154 + noise_variable_184);
        let noise_metadata_schedule_290_e2899: f64 = (noise_metadata_schedule_290_e2898).sqrt();
        let noise_metadata_schedule_290_e2900: f64 = (noise_variable_45 * noise_metadata_schedule_290_e2899);
        let noise_metadata_schedule_290_e2901: f64 = (1.0 - noise_metadata_schedule_290_e2900);
        (noise_metadata_schedule_290_e2901,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_290_e2903;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_291_e2906: f64 = if noise_variable_190 < params.p64 { 1.0 } else { 0.0 };
            noise_variable_216 = noise_metadata_schedule_291_e2906;
        }
        if matches!(source_index, 0 | 1) {
            let (noise_metadata_schedule_292_e2910,) = {
    if (noise_variable_216 != 0.0) {
        (params.p64,)
    } else {
        (noise_variable_190,)
    }
};
            noise_variable_190 = noise_metadata_schedule_292_e2910;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_293_e2913: f64 = (noise_variable_29 * noise_variable_190);
            let noise_metadata_schedule_293_e2916: f64 = (1.0 + noise_variable_189);
            let noise_metadata_schedule_293_e2917: f64 = (noise_metadata_schedule_293_e2913 / noise_metadata_schedule_293_e2916);
            noise_variable_63 = noise_metadata_schedule_293_e2917;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_294_e2920: f64 = (noise_variable_149 * noise_variable_63);
            let noise_metadata_schedule_294_e2922: f64 = (noise_metadata_schedule_294_e2920 * noise_variable_184);
            noise_variable_81 = noise_metadata_schedule_294_e2922;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_295_e2925: f64 = if noise_variable_84 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_217 = noise_metadata_schedule_295_e2925;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_296_e2931,) = {
    if (noise_variable_217 != 0.0) {
        let noise_metadata_schedule_296_e2929: f64 = (noise_variable_31 * noise_variable_74);
        (noise_metadata_schedule_296_e2929,)
    } else {
        (noise_variable_218,)
    }
};
            noise_variable_218 = noise_metadata_schedule_296_e2931;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_297_e2937,) = {
    if (noise_variable_217 != 0.0) {
        let noise_metadata_schedule_297_e2935: f64 = (noise_variable_32 * noise_variable_75);
        (noise_metadata_schedule_297_e2935,)
    } else {
        (noise_variable_219,)
    }
};
            noise_variable_219 = noise_metadata_schedule_297_e2937;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_298_e2940: f64 = if noise_variable_218 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_224 = noise_metadata_schedule_298_e2940;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_299_e2950,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_299_e2947: f64 = (params.p70 * noise_variable_70);
        let noise_metadata_schedule_299_e2948: f64 = (1.0 / noise_metadata_schedule_299_e2947);
        (noise_metadata_schedule_299_e2948,)
    } else {
        (noise_variable_220,)
    }
};
            noise_variable_220 = noise_metadata_schedule_299_e2950;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_300_e2953: f64 = if noise_variable_65 < noise_variable_61 { 1.0 } else { 0.0 };
            noise_variable_225 = noise_metadata_schedule_300_e2953;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_301_e2964,) = {
    if (((noise_variable_217 != 0.0) && (noise_variable_224 != 0.0)) && (noise_variable_225 != 0.0)) {
        let noise_metadata_schedule_301_e2961: f64 = (noise_variable_65 * noise_variable_220);
        let noise_metadata_schedule_301_e2962: f64 = (noise_metadata_schedule_301_e2961).exp();
        (noise_metadata_schedule_301_e2962,)
    } else {
        (noise_variable_221,)
    }
};
            noise_variable_221 = noise_metadata_schedule_301_e2964;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_302_e2984,) = {
    if (((noise_variable_217 != 0.0) && (noise_variable_224 != 0.0)) && (noise_variable_225 == 0.0)) {
        let noise_metadata_schedule_302_e2973: f64 = (noise_variable_61 * noise_variable_220);
        let noise_metadata_schedule_302_e2974: f64 = (noise_metadata_schedule_302_e2973).exp();
        let noise_metadata_schedule_302_e2978: f64 = (noise_variable_65 - noise_variable_61);
        let noise_metadata_schedule_302_e2980: f64 = (noise_metadata_schedule_302_e2978 * noise_variable_220);
        let noise_metadata_schedule_302_e2981: f64 = (1.0 + noise_metadata_schedule_302_e2980);
        let noise_metadata_schedule_302_e2982: f64 = (noise_metadata_schedule_302_e2974 * noise_metadata_schedule_302_e2981);
        (noise_metadata_schedule_302_e2982,)
    } else {
        (noise_variable_221,)
    }
};
            noise_variable_221 = noise_metadata_schedule_302_e2984;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_303_e2994,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_224 != 0.0)) {
        let noise_metadata_schedule_303_e2991: f64 = (noise_variable_221 - 1.0);
        let noise_metadata_schedule_303_e2992: f64 = (noise_variable_218 * noise_metadata_schedule_303_e2991);
        (noise_metadata_schedule_303_e2992,)
    } else {
        (noise_variable_222,)
    }
};
            noise_variable_222 = noise_metadata_schedule_303_e2994;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_304_e3001,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_224 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_222,)
    }
};
            noise_variable_222 = noise_metadata_schedule_304_e3001;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_305_e3004: f64 = if noise_variable_219 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_226 = noise_metadata_schedule_305_e3004;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_306_e3014,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_226 != 0.0)) {
        let noise_metadata_schedule_306_e3011: f64 = (params.p77 * noise_variable_70);
        let noise_metadata_schedule_306_e3012: f64 = (1.0 / noise_metadata_schedule_306_e3011);
        (noise_metadata_schedule_306_e3012,)
    } else {
        (noise_variable_220,)
    }
};
            noise_variable_220 = noise_metadata_schedule_306_e3014;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_307_e3017: f64 = if noise_variable_65 < noise_variable_60 { 1.0 } else { 0.0 };
            noise_variable_227 = noise_metadata_schedule_307_e3017;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_308_e3028,) = {
    if (((noise_variable_217 != 0.0) && (noise_variable_226 != 0.0)) && (noise_variable_227 != 0.0)) {
        let noise_metadata_schedule_308_e3025: f64 = (noise_variable_65 * noise_variable_220);
        let noise_metadata_schedule_308_e3026: f64 = (noise_metadata_schedule_308_e3025).exp();
        (noise_metadata_schedule_308_e3026,)
    } else {
        (noise_variable_221,)
    }
};
            noise_variable_221 = noise_metadata_schedule_308_e3028;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_309_e3048,) = {
    if (((noise_variable_217 != 0.0) && (noise_variable_226 != 0.0)) && (noise_variable_227 == 0.0)) {
        let noise_metadata_schedule_309_e3037: f64 = (noise_variable_60 * noise_variable_220);
        let noise_metadata_schedule_309_e3038: f64 = (noise_metadata_schedule_309_e3037).exp();
        let noise_metadata_schedule_309_e3042: f64 = (noise_variable_65 - noise_variable_60);
        let noise_metadata_schedule_309_e3044: f64 = (noise_metadata_schedule_309_e3042 * noise_variable_220);
        let noise_metadata_schedule_309_e3045: f64 = (1.0 + noise_metadata_schedule_309_e3044);
        let noise_metadata_schedule_309_e3046: f64 = (noise_metadata_schedule_309_e3038 * noise_metadata_schedule_309_e3045);
        (noise_metadata_schedule_309_e3046,)
    } else {
        (noise_variable_221,)
    }
};
            noise_variable_221 = noise_metadata_schedule_309_e3048;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_310_e3058,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_226 != 0.0)) {
        let noise_metadata_schedule_310_e3055: f64 = (noise_variable_221 - 1.0);
        let noise_metadata_schedule_310_e3056: f64 = (noise_variable_219 * noise_metadata_schedule_310_e3055);
        (noise_metadata_schedule_310_e3056,)
    } else {
        (noise_variable_223,)
    }
};
            noise_variable_223 = noise_metadata_schedule_310_e3058;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_311_e3065,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_226 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_223,)
    }
};
            noise_variable_223 = noise_metadata_schedule_311_e3065;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_312_e3071,) = {
    if (noise_variable_217 != 0.0) {
        let noise_metadata_schedule_312_e3069: f64 = (noise_variable_222 + noise_variable_223);
        (noise_metadata_schedule_312_e3069,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_312_e3071;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_313_e3074: f64 = if noise_variable_103 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_231 = noise_metadata_schedule_313_e3074;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_314_e3083,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_231 != 0.0)) {
        let noise_metadata_schedule_314_e3079: f64 = (-noise_variable_103);
        let noise_metadata_schedule_314_e3081: f64 = (noise_metadata_schedule_314_e3079 - noise_variable_65);
        (noise_metadata_schedule_314_e3081,)
    } else {
        (noise_variable_228,)
    }
};
            noise_variable_228 = noise_metadata_schedule_314_e3083;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_315_e3093,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_231 != 0.0)) {
        let noise_metadata_schedule_315_e3090: f64 = (noise_variable_104 * noise_variable_70);
        let noise_metadata_schedule_315_e3091: f64 = (1.0 / noise_metadata_schedule_315_e3090);
        (noise_metadata_schedule_315_e3091,)
    } else {
        (noise_variable_229,)
    }
};
            noise_variable_229 = noise_metadata_schedule_315_e3093;
        }
        if matches!(source_index, 4) {
            let noise_metadata_schedule_316_e3096: f64 = if noise_variable_228 < noise_variable_62 { 1.0 } else { 0.0 };
            noise_variable_232 = noise_metadata_schedule_316_e3096;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_317_e3107,) = {
    if (((noise_variable_217 != 0.0) && (noise_variable_231 != 0.0)) && (noise_variable_232 != 0.0)) {
        let noise_metadata_schedule_317_e3104: f64 = (noise_variable_228 * noise_variable_229);
        let noise_metadata_schedule_317_e3105: f64 = (noise_metadata_schedule_317_e3104).exp();
        (noise_metadata_schedule_317_e3105,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_317_e3107;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_318_e3127,) = {
    if (((noise_variable_217 != 0.0) && (noise_variable_231 != 0.0)) && (noise_variable_232 == 0.0)) {
        let noise_metadata_schedule_318_e3116: f64 = (noise_variable_62 * noise_variable_229);
        let noise_metadata_schedule_318_e3117: f64 = (noise_metadata_schedule_318_e3116).exp();
        let noise_metadata_schedule_318_e3121: f64 = (noise_variable_228 - noise_variable_62);
        let noise_metadata_schedule_318_e3123: f64 = (noise_metadata_schedule_318_e3121 * noise_variable_229);
        let noise_metadata_schedule_318_e3124: f64 = (1.0 + noise_metadata_schedule_318_e3123);
        let noise_metadata_schedule_318_e3125: f64 = (noise_metadata_schedule_318_e3117 * noise_metadata_schedule_318_e3124);
        (noise_metadata_schedule_318_e3125,)
    } else {
        (noise_variable_230,)
    }
};
            noise_variable_230 = noise_metadata_schedule_318_e3127;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_319_e3142,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_231 != 0.0)) {
        let noise_metadata_schedule_319_e3132: f64 = (-params.p84);
        let noise_metadata_schedule_319_e3135: f64 = (-noise_variable_103);
        let noise_metadata_schedule_319_e3137: f64 = (noise_metadata_schedule_319_e3135 * noise_variable_229);
        let noise_metadata_schedule_319_e3138: f64 = (noise_metadata_schedule_319_e3137).exp();
        let noise_metadata_schedule_319_e3139: f64 = (noise_variable_230 - noise_metadata_schedule_319_e3138);
        let noise_metadata_schedule_319_e3140: f64 = (noise_metadata_schedule_319_e3132 * noise_metadata_schedule_319_e3139);
        (noise_metadata_schedule_319_e3140,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_319_e3142;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_320_e3149,) = {
    if ((noise_variable_217 != 0.0) && (noise_variable_231 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_320_e3149;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_322_e3164,) = {
    if (noise_variable_217 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_90,)
    }
};
            noise_variable_90 = noise_metadata_schedule_322_e3164;
        }
        if matches!(source_index, 4) {
            let (noise_metadata_schedule_323_e3169,) = {
    if (noise_variable_217 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_92,)
    }
};
            noise_variable_92 = noise_metadata_schedule_323_e3169;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_325_e3177: f64 = if noise_variable_85 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_233 = noise_metadata_schedule_325_e3177;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_326_e3183,) = {
    if (noise_variable_233 != 0.0) {
        let noise_metadata_schedule_326_e3181: f64 = (noise_variable_33 * noise_variable_74);
        (noise_metadata_schedule_326_e3181,)
    } else {
        (noise_variable_234,)
    }
};
            noise_variable_234 = noise_metadata_schedule_326_e3183;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_327_e3189,) = {
    if (noise_variable_233 != 0.0) {
        let noise_metadata_schedule_327_e3187: f64 = (noise_variable_34 * noise_variable_75);
        (noise_metadata_schedule_327_e3187,)
    } else {
        (noise_variable_235,)
    }
};
            noise_variable_235 = noise_metadata_schedule_327_e3189;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_328_e3192: f64 = if noise_variable_234 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_240 = noise_metadata_schedule_328_e3192;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_329_e3202,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_240 != 0.0)) {
        let noise_metadata_schedule_329_e3199: f64 = (params.p70 * noise_variable_70);
        let noise_metadata_schedule_329_e3200: f64 = (1.0 / noise_metadata_schedule_329_e3199);
        (noise_metadata_schedule_329_e3200,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_329_e3202;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_330_e3205: f64 = if noise_variable_66 < noise_variable_61 { 1.0 } else { 0.0 };
            noise_variable_241 = noise_metadata_schedule_330_e3205;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_331_e3216,) = {
    if (((noise_variable_233 != 0.0) && (noise_variable_240 != 0.0)) && (noise_variable_241 != 0.0)) {
        let noise_metadata_schedule_331_e3213: f64 = (noise_variable_66 * noise_variable_236);
        let noise_metadata_schedule_331_e3214: f64 = (noise_metadata_schedule_331_e3213).exp();
        (noise_metadata_schedule_331_e3214,)
    } else {
        (noise_variable_237,)
    }
};
            noise_variable_237 = noise_metadata_schedule_331_e3216;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_332_e3236,) = {
    if (((noise_variable_233 != 0.0) && (noise_variable_240 != 0.0)) && (noise_variable_241 == 0.0)) {
        let noise_metadata_schedule_332_e3225: f64 = (noise_variable_61 * noise_variable_236);
        let noise_metadata_schedule_332_e3226: f64 = (noise_metadata_schedule_332_e3225).exp();
        let noise_metadata_schedule_332_e3230: f64 = (noise_variable_66 - noise_variable_61);
        let noise_metadata_schedule_332_e3232: f64 = (noise_metadata_schedule_332_e3230 * noise_variable_236);
        let noise_metadata_schedule_332_e3233: f64 = (1.0 + noise_metadata_schedule_332_e3232);
        let noise_metadata_schedule_332_e3234: f64 = (noise_metadata_schedule_332_e3226 * noise_metadata_schedule_332_e3233);
        (noise_metadata_schedule_332_e3234,)
    } else {
        (noise_variable_237,)
    }
};
            noise_variable_237 = noise_metadata_schedule_332_e3236;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_333_e3246,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_240 != 0.0)) {
        let noise_metadata_schedule_333_e3243: f64 = (noise_variable_237 - 1.0);
        let noise_metadata_schedule_333_e3244: f64 = (noise_variable_234 * noise_metadata_schedule_333_e3243);
        (noise_metadata_schedule_333_e3244,)
    } else {
        (noise_variable_238,)
    }
};
            noise_variable_238 = noise_metadata_schedule_333_e3246;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_334_e3253,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_240 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_238,)
    }
};
            noise_variable_238 = noise_metadata_schedule_334_e3253;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_335_e3256: f64 = if noise_variable_235 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_242 = noise_metadata_schedule_335_e3256;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_336_e3266,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_242 != 0.0)) {
        let noise_metadata_schedule_336_e3263: f64 = (params.p77 * noise_variable_70);
        let noise_metadata_schedule_336_e3264: f64 = (1.0 / noise_metadata_schedule_336_e3263);
        (noise_metadata_schedule_336_e3264,)
    } else {
        (noise_variable_236,)
    }
};
            noise_variable_236 = noise_metadata_schedule_336_e3266;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_337_e3269: f64 = if noise_variable_66 < noise_variable_60 { 1.0 } else { 0.0 };
            noise_variable_243 = noise_metadata_schedule_337_e3269;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_338_e3280,) = {
    if (((noise_variable_233 != 0.0) && (noise_variable_242 != 0.0)) && (noise_variable_243 != 0.0)) {
        let noise_metadata_schedule_338_e3277: f64 = (noise_variable_66 * noise_variable_236);
        let noise_metadata_schedule_338_e3278: f64 = (noise_metadata_schedule_338_e3277).exp();
        (noise_metadata_schedule_338_e3278,)
    } else {
        (noise_variable_237,)
    }
};
            noise_variable_237 = noise_metadata_schedule_338_e3280;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_339_e3300,) = {
    if (((noise_variable_233 != 0.0) && (noise_variable_242 != 0.0)) && (noise_variable_243 == 0.0)) {
        let noise_metadata_schedule_339_e3289: f64 = (noise_variable_60 * noise_variable_236);
        let noise_metadata_schedule_339_e3290: f64 = (noise_metadata_schedule_339_e3289).exp();
        let noise_metadata_schedule_339_e3294: f64 = (noise_variable_66 - noise_variable_60);
        let noise_metadata_schedule_339_e3296: f64 = (noise_metadata_schedule_339_e3294 * noise_variable_236);
        let noise_metadata_schedule_339_e3297: f64 = (1.0 + noise_metadata_schedule_339_e3296);
        let noise_metadata_schedule_339_e3298: f64 = (noise_metadata_schedule_339_e3290 * noise_metadata_schedule_339_e3297);
        (noise_metadata_schedule_339_e3298,)
    } else {
        (noise_variable_237,)
    }
};
            noise_variable_237 = noise_metadata_schedule_339_e3300;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_340_e3310,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_242 != 0.0)) {
        let noise_metadata_schedule_340_e3307: f64 = (noise_variable_237 - 1.0);
        let noise_metadata_schedule_340_e3308: f64 = (noise_variable_235 * noise_metadata_schedule_340_e3307);
        (noise_metadata_schedule_340_e3308,)
    } else {
        (noise_variable_239,)
    }
};
            noise_variable_239 = noise_metadata_schedule_340_e3310;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_341_e3317,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_242 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_239,)
    }
};
            noise_variable_239 = noise_metadata_schedule_341_e3317;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_342_e3323,) = {
    if (noise_variable_233 != 0.0) {
        let noise_metadata_schedule_342_e3321: f64 = (noise_variable_238 + noise_variable_239);
        (noise_metadata_schedule_342_e3321,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_342_e3323;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_343_e3326: f64 = if noise_variable_103 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_247 = noise_metadata_schedule_343_e3326;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_344_e3335,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_344_e3331: f64 = (-noise_variable_103);
        let noise_metadata_schedule_344_e3333: f64 = (noise_metadata_schedule_344_e3331 - noise_variable_66);
        (noise_metadata_schedule_344_e3333,)
    } else {
        (noise_variable_244,)
    }
};
            noise_variable_244 = noise_metadata_schedule_344_e3335;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_345_e3345,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_345_e3342: f64 = (noise_variable_104 * noise_variable_70);
        let noise_metadata_schedule_345_e3343: f64 = (1.0 / noise_metadata_schedule_345_e3342);
        (noise_metadata_schedule_345_e3343,)
    } else {
        (noise_variable_245,)
    }
};
            noise_variable_245 = noise_metadata_schedule_345_e3345;
        }
        if matches!(source_index, 5) {
            let noise_metadata_schedule_346_e3348: f64 = if noise_variable_244 < noise_variable_62 { 1.0 } else { 0.0 };
            noise_variable_248 = noise_metadata_schedule_346_e3348;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_347_e3359,) = {
    if (((noise_variable_233 != 0.0) && (noise_variable_247 != 0.0)) && (noise_variable_248 != 0.0)) {
        let noise_metadata_schedule_347_e3356: f64 = (noise_variable_244 * noise_variable_245);
        let noise_metadata_schedule_347_e3357: f64 = (noise_metadata_schedule_347_e3356).exp();
        (noise_metadata_schedule_347_e3357,)
    } else {
        (noise_variable_246,)
    }
};
            noise_variable_246 = noise_metadata_schedule_347_e3359;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_348_e3379,) = {
    if (((noise_variable_233 != 0.0) && (noise_variable_247 != 0.0)) && (noise_variable_248 == 0.0)) {
        let noise_metadata_schedule_348_e3368: f64 = (noise_variable_62 * noise_variable_245);
        let noise_metadata_schedule_348_e3369: f64 = (noise_metadata_schedule_348_e3368).exp();
        let noise_metadata_schedule_348_e3373: f64 = (noise_variable_244 - noise_variable_62);
        let noise_metadata_schedule_348_e3375: f64 = (noise_metadata_schedule_348_e3373 * noise_variable_245);
        let noise_metadata_schedule_348_e3376: f64 = (1.0 + noise_metadata_schedule_348_e3375);
        let noise_metadata_schedule_348_e3377: f64 = (noise_metadata_schedule_348_e3369 * noise_metadata_schedule_348_e3376);
        (noise_metadata_schedule_348_e3377,)
    } else {
        (noise_variable_246,)
    }
};
            noise_variable_246 = noise_metadata_schedule_348_e3379;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_349_e3394,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_247 != 0.0)) {
        let noise_metadata_schedule_349_e3384: f64 = (-params.p84);
        let noise_metadata_schedule_349_e3387: f64 = (-noise_variable_103);
        let noise_metadata_schedule_349_e3389: f64 = (noise_metadata_schedule_349_e3387 * noise_variable_245);
        let noise_metadata_schedule_349_e3390: f64 = (noise_metadata_schedule_349_e3389).exp();
        let noise_metadata_schedule_349_e3391: f64 = (noise_variable_246 - noise_metadata_schedule_349_e3390);
        let noise_metadata_schedule_349_e3392: f64 = (noise_metadata_schedule_349_e3384 * noise_metadata_schedule_349_e3391);
        (noise_metadata_schedule_349_e3392,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_349_e3394;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_350_e3401,) = {
    if ((noise_variable_233 != 0.0) && (noise_variable_247 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_350_e3401;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_352_e3416,) = {
    if (noise_variable_233 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_91,)
    }
};
            noise_variable_91 = noise_metadata_schedule_352_e3416;
        }
        if matches!(source_index, 5) {
            let (noise_metadata_schedule_353_e3421,) = {
    if (noise_variable_233 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_93,)
    }
};
            noise_variable_93 = noise_metadata_schedule_353_e3421;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_372_e3616: f64 = (-params.p21);
            let noise_metadata_schedule_372_e3618: f64 = (noise_metadata_schedule_372_e3616 * noise_variable_81);
            noise_variable_81 = noise_metadata_schedule_372_e3618;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_477_e4949,) = {
    if ((params.p13 != 0.0) && (params.p89 != 0.0)) {
        (noise_variable_3,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_477_e4949;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_478_e4955,) = {
    if ((params.p13 != 0.0) && (params.p89 != 0.0)) {
        (noise_variable_4,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_478_e4955;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_479_e4962,) = {
    if ((params.p13 != 0.0) && (params.p89 == 0.0)) {
        (noise_variable_27,)
    } else {
        (noise_variable_37,)
    }
};
            noise_variable_37 = noise_metadata_schedule_479_e4962;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_480_e4969,) = {
    if ((params.p13 != 0.0) && (params.p89 == 0.0)) {
        (noise_variable_26,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_480_e4969;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_481_e4979,) = {
    if (params.p13 != 0.0) {
        let noise_metadata_schedule_481_e4973: f64 = (4.0 * 1.3806505e-23);
        let noise_metadata_schedule_481_e4975: f64 = (noise_metadata_schedule_481_e4973 * noise_variable_24);
        let noise_metadata_schedule_481_e4977: f64 = (noise_metadata_schedule_481_e4975 * noise_variable_63);
        (noise_metadata_schedule_481_e4977,)
    } else {
        (noise_variable_99,)
    }
};
            noise_variable_99 = noise_metadata_schedule_481_e4979;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_482_e4994,) = {
    if (params.p13 != 0.0) {
        let noise_metadata_schedule_482_e4984: f64 = (noise_variable_81 / noise_variable_38);
        let noise_metadata_schedule_482_e4985: f64 = (noise_metadata_schedule_482_e4984).abs();
        let noise_metadata_schedule_482_e4987: f64 = (noise_metadata_schedule_482_e4985).powf(params.p87);
        let noise_metadata_schedule_482_e4988: f64 = (noise_variable_80 * noise_metadata_schedule_482_e4987);
        let noise_metadata_schedule_482_e4990: f64 = (noise_metadata_schedule_482_e4988 * noise_variable_38);
        let noise_metadata_schedule_482_e4992: f64 = (noise_metadata_schedule_482_e4990 / noise_variable_37);
        (noise_metadata_schedule_482_e4992,)
    } else {
        (noise_variable_100,)
    }
};
            noise_variable_100 = noise_metadata_schedule_482_e4994;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_483_e4997: f64 = if noise_variable_81 < 0.0 { 1.0 } else { 0.0 };
            noise_variable_323 = noise_metadata_schedule_483_e4997;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_484_e5004,) = {
    if ((params.p13 != 0.0) && (noise_variable_323 != 0.0)) {
        let noise_metadata_schedule_484_e5002: f64 = (-noise_variable_100);
        (noise_metadata_schedule_484_e5002,)
    } else {
        (noise_variable_100,)
    }
};
            noise_variable_100 = noise_metadata_schedule_484_e5004;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_485_e5007: f64 = if noise_variable_54 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_324 = noise_metadata_schedule_485_e5007;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_486_e5017,) = {
    if ((params.p13 != 0.0) && (noise_variable_324 != 0.0)) {
        let noise_metadata_schedule_486_e5014: f64 = (noise_variable_54 * noise_variable_58);
        let noise_metadata_schedule_486_e5015: f64 = (1.0 / noise_metadata_schedule_486_e5014);
        (noise_metadata_schedule_486_e5015,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_486_e5017;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_487_e5024,) = {
    if ((params.p13 != 0.0) && (noise_variable_324 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_487_e5024;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_488_e5027: f64 = if noise_variable_55 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_325 = noise_metadata_schedule_488_e5027;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_489_e5037,) = {
    if ((params.p13 != 0.0) && (noise_variable_325 != 0.0)) {
        let noise_metadata_schedule_489_e5034: f64 = (noise_variable_55 * noise_variable_58);
        let noise_metadata_schedule_489_e5035: f64 = (1.0 / noise_metadata_schedule_489_e5034);
        (noise_metadata_schedule_489_e5035,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_489_e5037;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_490_e5044,) = {
    if ((params.p13 != 0.0) && (noise_variable_325 == 0.0)) {
        (0.0,)
    } else {
        (noise_variable_56,)
    }
};
            noise_variable_56 = noise_metadata_schedule_490_e5044;
        }
        match source_index {
            0 => {
                let noise_0_psd_e5105: f64 = 1.0;
                let noise_0_psd_e5106: f64 = (noise_0_psd_e5105 * noise_variable_99);
                let psd = noise_0_psd_e5106;
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
                let noise_1_psd_e5108: f64 = 1.0;
                let noise_1_psd_e5109: f64 = (noise_1_psd_e5108 * noise_variable_100);
                let psd = noise_1_psd_e5109;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = Some(params.p88);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            2 => {
                let noise_2_psd_e5111: f64 = 1.0;
                let noise_2_psd_e202: f64 = (4.0 * 1.3806505e-23);
                let noise_2_psd_e204: f64 = (noise_2_psd_e202 * noise_variable_24);
                let noise_2_psd_e206: f64 = (noise_2_psd_e204 * noise_variable_56);
                let noise_2_psd_e5112: f64 = (noise_2_psd_e5111 * noise_2_psd_e206);
                let psd = noise_2_psd_e5112;
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
                let noise_3_psd_e5114: f64 = 1.0;
                let noise_3_psd_e214: f64 = (4.0 * 1.3806505e-23);
                let noise_3_psd_e216: f64 = (noise_3_psd_e214 * noise_variable_24);
                let noise_3_psd_e218: f64 = (noise_3_psd_e216 * noise_variable_56);
                let noise_3_psd_e5115: f64 = (noise_3_psd_e5114 * noise_3_psd_e218);
                let psd = noise_3_psd_e5115;
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
                let noise_4_psd_e5117: f64 = 1.0;
                let noise_4_psd_e228: f64 = (2.0 * 1.60217653e-19);
                let noise_4_psd_e232: f64 = (2.0 * noise_variable_84);
                let noise_4_psd_e233: f64 = (noise_variable_90 + noise_4_psd_e232);
                let noise_4_psd_e234: f64 = (noise_4_psd_e233).abs();
                let noise_4_psd_e236: f64 = (noise_variable_92).abs();
                let noise_4_psd_e237: f64 = (noise_4_psd_e234 + noise_4_psd_e236);
                let noise_4_psd_e238: f64 = (noise_4_psd_e228 * noise_4_psd_e237);
                let noise_4_psd_e5118: f64 = (noise_4_psd_e5117 * noise_4_psd_e238);
                let psd = noise_4_psd_e5118;
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
                let noise_5_psd_e5120: f64 = 1.0;
                let noise_5_psd_e248: f64 = (2.0 * 1.60217653e-19);
                let noise_5_psd_e252: f64 = (2.0 * noise_variable_85);
                let noise_5_psd_e253: f64 = (noise_variable_91 + noise_5_psd_e252);
                let noise_5_psd_e254: f64 = (noise_5_psd_e253).abs();
                let noise_5_psd_e256: f64 = (noise_variable_93).abs();
                let noise_5_psd_e257: f64 = (noise_5_psd_e254 + noise_5_psd_e256);
                let noise_5_psd_e258: f64 = (noise_5_psd_e248 * noise_5_psd_e257);
                let noise_5_psd_e5121: f64 = (noise_5_psd_e5120 * noise_5_psd_e258);
                let psd = noise_5_psd_e5121;
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

#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

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
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 329];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            params.p13 != 0.0
        };
        let noise_source_1_active = {
            params.p13 != 0.0
        };
        let noise_source_2_active = {
            params.p13 != 0.0
        };
        let noise_source_3_active = {
            params.p13 != 0.0
        };
        let noise_source_4_active = {
            let noise_4_activation_e225: f64 = if ((params.p13 != 0.0) && (w[326] != 0.0)) { 1.0 } else { 0.0 };
            noise_4_activation_e225 != 0.0
        };
        let noise_source_5_active = {
            let noise_5_activation_e245: f64 = if ((params.p13 != 0.0) && (w[327] != 0.0)) { 1.0 } else { 0.0 };
            noise_5_activation_e245 != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active, noise_source_4_active, noise_source_5_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3) | ((noise_source_4_active as u128) << 4) | ((noise_source_5_active as u128) << 5)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_2(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_3(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_4(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_5(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_6(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e5105: f64 = 1.0;
            let noise_0_psd_e5106: f64 = (noise_0_psd_e5105 * w[99]);
            let psd = noise_0_psd_e5106;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[1] {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_1_psd_e5108: f64 = 1.0;
            let noise_1_psd_e5109: f64 = (noise_1_psd_e5108 * w[100]);
            let psd = noise_1_psd_e5109;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(params.p88);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[2] {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_2_psd_e5111: f64 = 1.0;
            let noise_2_psd_e202: f64 = (4.0 * 1.3806505e-23);
            let noise_2_psd_e204: f64 = (noise_2_psd_e202 * w[24]);
            let noise_2_psd_e206: f64 = (noise_2_psd_e204 * w[56]);
            let noise_2_psd_e5112: f64 = (noise_2_psd_e5111 * noise_2_psd_e206);
            let psd = noise_2_psd_e5112;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[3] {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_3_psd_e5114: f64 = 1.0;
            let noise_3_psd_e214: f64 = (4.0 * 1.3806505e-23);
            let noise_3_psd_e216: f64 = (noise_3_psd_e214 * w[24]);
            let noise_3_psd_e218: f64 = (noise_3_psd_e216 * w[56]);
            let noise_3_psd_e5115: f64 = (noise_3_psd_e5114 * noise_3_psd_e218);
            let psd = noise_3_psd_e5115;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[4] {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_4_psd_e5117: f64 = 1.0;
            let noise_4_psd_e228: f64 = (2.0 * 1.60217653e-19);
            let noise_4_psd_e232: f64 = (2.0 * w[84]);
            let noise_4_psd_e233: f64 = (w[90] + noise_4_psd_e232);
            let noise_4_psd_e234: f64 = (noise_4_psd_e233).abs();
            let noise_4_psd_e236: f64 = (w[92]).abs();
            let noise_4_psd_e237: f64 = (noise_4_psd_e234 + noise_4_psd_e236);
            let noise_4_psd_e238: f64 = (noise_4_psd_e228 * noise_4_psd_e237);
            let noise_4_psd_e5118: f64 = (noise_4_psd_e5117 * noise_4_psd_e238);
            let psd = noise_4_psd_e5118;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[5] {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_5_psd_e5120: f64 = 1.0;
            let noise_5_psd_e248: f64 = (2.0 * 1.60217653e-19);
            let noise_5_psd_e252: f64 = (2.0 * w[85]);
            let noise_5_psd_e253: f64 = (w[91] + noise_5_psd_e252);
            let noise_5_psd_e254: f64 = (noise_5_psd_e253).abs();
            let noise_5_psd_e256: f64 = (w[93]).abs();
            let noise_5_psd_e257: f64 = (noise_5_psd_e254 + noise_5_psd_e256);
            let noise_5_psd_e258: f64 = (noise_5_psd_e248 * noise_5_psd_e257);
            let noise_5_psd_e5121: f64 = (noise_5_psd_e5120 * noise_5_psd_e258);
            let psd = noise_5_psd_e5121;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329]) {
        let params = &*self.params;
        let noise_activation_schedule_5_0_e279: f64 = (0.01 * params.p23);
        let noise_activation_schedule_5_0_e280: f64 = (1.0 - noise_activation_schedule_5_0_e279);
        let noise_activation_schedule_5_0_e282: f64 = (noise_activation_schedule_5_0_e280 * params.p22);
        let noise_activation_schedule_5_0_e284: f64 = (noise_activation_schedule_5_0_e282 * 1000000.0);
        w[13] = noise_activation_schedule_5_0_e284;
        let noise_activation_schedule_6_0_e287: f64 = (w[13] * w[13]);
        w[14] = noise_activation_schedule_6_0_e287;
        let noise_activation_schedule_7_0_e290: f64 = (273.15 + params.p28);
        w[15] = noise_activation_schedule_7_0_e290;
        let noise_activation_schedule_28_0_e384: f64 = (params.p3 * w[14]);
        w[31] = noise_activation_schedule_28_0_e384;
        let noise_activation_schedule_29_0_e387: f64 = (params.p4 * w[13]);
        w[32] = noise_activation_schedule_29_0_e387;
        let noise_activation_schedule_30_0_e390: f64 = (params.p6 * w[14]);
        w[33] = noise_activation_schedule_30_0_e390;
        let noise_activation_schedule_31_0_e393: f64 = (params.p7 * w[13]);
        w[34] = noise_activation_schedule_31_0_e393;
        w[10] = (ctx.node_voltage(self.nodes[3]) - 0.0);
        let noise_activation_schedule_112_0_e1137: f64 = ctx.temperature();
        let noise_activation_schedule_112_0_e1139: f64 = (noise_activation_schedule_112_0_e1137 + params.p9);
        let noise_activation_schedule_112_0_e1141: f64 = (noise_activation_schedule_112_0_e1139 + w[10]);
        let noise_activation_schedule_112_0_e1143: f64 = (noise_activation_schedule_112_0_e1141 - 273.15);
        w[23] = noise_activation_schedule_112_0_e1143;
        let noise_activation_schedule_113_0_e1147: f64 = (params.p35 + 1.0);
        let noise_activation_schedule_113_0_e1148: f64 = if w[23] < noise_activation_schedule_113_0_e1147 { 1.0 } else { 0.0 };
        w[134] = noise_activation_schedule_113_0_e1148;
        let (noise_activation_schedule_114_0_e1159,) = {
    if (w[134] != 0.0) {
        let noise_activation_schedule_114_0_e1153: f64 = (w[23] - params.p35);
        let noise_activation_schedule_114_0_e1155: f64 = (noise_activation_schedule_114_0_e1153 - 1.0);
        let noise_activation_schedule_114_0_e1156: f64 = (noise_activation_schedule_114_0_e1155).exp();
        let noise_activation_schedule_114_0_e1157: f64 = (params.p35 + noise_activation_schedule_114_0_e1156);
        (noise_activation_schedule_114_0_e1157,)
    } else {
        (w[23],)
    }
};
        w[23] = noise_activation_schedule_114_0_e1159;
        let noise_activation_schedule_115_0_e1163: f64 = (params.p36 - 1.0);
        let noise_activation_schedule_115_0_e1164: f64 = if w[23] > noise_activation_schedule_115_0_e1163 { 1.0 } else { 0.0 };
        w[135] = noise_activation_schedule_115_0_e1164;
        let (noise_activation_schedule_116_0_e1178,) = {
    if ((w[134] == 0.0) && (w[135] != 0.0)) {
        let noise_activation_schedule_116_0_e1172: f64 = (params.p36 - w[23]);
        let noise_activation_schedule_116_0_e1174: f64 = (noise_activation_schedule_116_0_e1172 - 1.0);
        let noise_activation_schedule_116_0_e1175: f64 = (noise_activation_schedule_116_0_e1174).exp();
        let noise_activation_schedule_116_0_e1176: f64 = (params.p36 - noise_activation_schedule_116_0_e1175);
        (noise_activation_schedule_116_0_e1176,)
    } else {
        (w[23],)
    }
};
        w[23] = noise_activation_schedule_116_0_e1178;
        let (noise_activation_schedule_117_0_e1186,) = {
    if ((w[134] == 0.0) && (w[135] == 0.0)) {
        (w[23],)
    } else {
        (w[23],)
    }
};
        w[23] = noise_activation_schedule_117_0_e1186;
        let noise_activation_schedule_118_0_e1189: f64 = (w[23] + 273.15);
        w[24] = noise_activation_schedule_118_0_e1189;
        let noise_activation_schedule_119_0_e1192: f64 = (1.3806505e-23 * w[24]);
        let noise_activation_schedule_119_0_e1194: f64 = (noise_activation_schedule_119_0_e1192 / 1.60217653e-19);
        w[70] = noise_activation_schedule_119_0_e1194;
        let noise_activation_schedule_120_0_e1197: f64 = (w[24] / w[15]);
        w[68] = noise_activation_schedule_120_0_e1197;
        let noise_activation_schedule_133_0_e1298: f64 = if params.p69 > 0.0 { 1.0 } else { 0.0 };
        w[138] = noise_activation_schedule_133_0_e1298;
        let (noise_activation_schedule_134_0_e1319,) = {
    if (w[138] != 0.0) {
        let noise_activation_schedule_134_0_e1302: f64 = (-params.p90);
        let noise_activation_schedule_134_0_e1305: f64 = (1.0 - w[68]);
        let noise_activation_schedule_134_0_e1306: f64 = (noise_activation_schedule_134_0_e1302 * noise_activation_schedule_134_0_e1305);
        let noise_activation_schedule_134_0_e1308: f64 = (noise_activation_schedule_134_0_e1306 / w[70]);
        let noise_activation_schedule_134_0_e1311: f64 = (w[68]).ln();
        let noise_activation_schedule_134_0_e1312: f64 = (params.p91 * noise_activation_schedule_134_0_e1311);
        let noise_activation_schedule_134_0_e1313: f64 = (noise_activation_schedule_134_0_e1308 + noise_activation_schedule_134_0_e1312);
        let noise_activation_schedule_134_0_e1315: f64 = (noise_activation_schedule_134_0_e1313 / params.p70);
        let noise_activation_schedule_134_0_e1316: f64 = (noise_activation_schedule_134_0_e1315).exp();
        let noise_activation_schedule_134_0_e1317: f64 = (params.p69 * noise_activation_schedule_134_0_e1316);
        (noise_activation_schedule_134_0_e1317,)
    } else {
        (w[74],)
    }
};
        w[74] = noise_activation_schedule_134_0_e1319;
        let (noise_activation_schedule_136_0_e1337,) = {
    if (w[138] == 0.0) {
        (0.0,)
    } else {
        (w[74],)
    }
};
        w[74] = noise_activation_schedule_136_0_e1337;
        let noise_activation_schedule_138_0_e1345: f64 = if params.p76 > 0.0 { 1.0 } else { 0.0 };
        w[139] = noise_activation_schedule_138_0_e1345;
        let (noise_activation_schedule_139_0_e1366,) = {
    if (w[139] != 0.0) {
        let noise_activation_schedule_139_0_e1349: f64 = (-params.p90);
        let noise_activation_schedule_139_0_e1352: f64 = (1.0 - w[68]);
        let noise_activation_schedule_139_0_e1353: f64 = (noise_activation_schedule_139_0_e1349 * noise_activation_schedule_139_0_e1352);
        let noise_activation_schedule_139_0_e1355: f64 = (noise_activation_schedule_139_0_e1353 / w[70]);
        let noise_activation_schedule_139_0_e1358: f64 = (w[68]).ln();
        let noise_activation_schedule_139_0_e1359: f64 = (params.p91 * noise_activation_schedule_139_0_e1358);
        let noise_activation_schedule_139_0_e1360: f64 = (noise_activation_schedule_139_0_e1355 + noise_activation_schedule_139_0_e1359);
        let noise_activation_schedule_139_0_e1362: f64 = (noise_activation_schedule_139_0_e1360 / params.p77);
        let noise_activation_schedule_139_0_e1363: f64 = (noise_activation_schedule_139_0_e1362).exp();
        let noise_activation_schedule_139_0_e1364: f64 = (params.p76 * noise_activation_schedule_139_0_e1363);
        (noise_activation_schedule_139_0_e1364,)
    } else {
        (w[75],)
    }
};
        w[75] = noise_activation_schedule_139_0_e1366;
        let (noise_activation_schedule_141_0_e1384,) = {
    if (w[139] == 0.0) {
        (0.0,)
    } else {
        (w[75],)
    }
};
        w[75] = noise_activation_schedule_141_0_e1384;
        let noise_activation_schedule_143_0_e1392: f64 = (w[31] * w[74]);
        let noise_activation_schedule_143_0_e1395: f64 = (w[32] * w[75]);
        let noise_activation_schedule_143_0_e1396: f64 = (noise_activation_schedule_143_0_e1392 + noise_activation_schedule_143_0_e1395);
        w[84] = noise_activation_schedule_143_0_e1396;
        let noise_activation_schedule_144_0_e1399: f64 = (w[33] * w[74]);
        let noise_activation_schedule_144_0_e1402: f64 = (w[34] * w[75]);
        let noise_activation_schedule_144_0_e1403: f64 = (noise_activation_schedule_144_0_e1399 + noise_activation_schedule_144_0_e1402);
        w[85] = noise_activation_schedule_144_0_e1403;
        let noise_activation_schedule_491_0_e5047: f64 = if w[84] > 0.0 { 1.0 } else { 0.0 };
        w[326] = noise_activation_schedule_491_0_e5047;
        let noise_activation_schedule_492_0_e5050: f64 = if w[85] > 0.0 { 1.0 } else { 0.0 };
        w[327] = noise_activation_schedule_492_0_e5050;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_3_0_e272: f64 = self.multiplicity;
            w[12] = noise_metadata_schedule_3_0_e272;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_5_0_e279: f64 = (0.01 * params.p23);
            let noise_metadata_schedule_5_0_e280: f64 = (1.0 - noise_metadata_schedule_5_0_e279);
            let noise_metadata_schedule_5_0_e282: f64 = (noise_metadata_schedule_5_0_e280 * params.p22);
            let noise_metadata_schedule_5_0_e284: f64 = (noise_metadata_schedule_5_0_e282 * 1000000.0);
            w[13] = noise_metadata_schedule_5_0_e284;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_6_0_e287: f64 = (w[13] * w[13]);
            w[14] = noise_metadata_schedule_6_0_e287;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_7_0_e290: f64 = (273.15 + params.p28);
            w[15] = noise_metadata_schedule_7_0_e290;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_9_0_e293: f64 = ctx.temperature();
            let noise_metadata_schedule_9_0_e295: f64 = (noise_metadata_schedule_9_0_e293 + params.p9);
            let noise_metadata_schedule_9_0_e297: f64 = (noise_metadata_schedule_9_0_e295 - 273.15);
            w[23] = noise_metadata_schedule_9_0_e297;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_12_0_e307: f64 = (params.p35 + 1.0);
            let noise_metadata_schedule_12_0_e308: f64 = if w[23] < noise_metadata_schedule_12_0_e307 { 1.0 } else { 0.0 };
            w[114] = noise_metadata_schedule_12_0_e308;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_13_0_e319,) = {
    if (w[114] != 0.0) {
        let noise_metadata_schedule_13_0_e313: f64 = (w[23] - params.p35);
        let noise_metadata_schedule_13_0_e315: f64 = (noise_metadata_schedule_13_0_e313 - 1.0);
        let noise_metadata_schedule_13_0_e316: f64 = (noise_metadata_schedule_13_0_e315).exp();
        let noise_metadata_schedule_13_0_e317: f64 = (params.p35 + noise_metadata_schedule_13_0_e316);
        (noise_metadata_schedule_13_0_e317,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_13_0_e319;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_14_0_e323: f64 = (params.p36 - 1.0);
            let noise_metadata_schedule_14_0_e324: f64 = if w[23] > noise_metadata_schedule_14_0_e323 { 1.0 } else { 0.0 };
            w[115] = noise_metadata_schedule_14_0_e324;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_15_0_e338,) = {
    if ((w[114] == 0.0) && (w[115] != 0.0)) {
        let noise_metadata_schedule_15_0_e332: f64 = (params.p36 - w[23]);
        let noise_metadata_schedule_15_0_e334: f64 = (noise_metadata_schedule_15_0_e332 - 1.0);
        let noise_metadata_schedule_15_0_e335: f64 = (noise_metadata_schedule_15_0_e334).exp();
        let noise_metadata_schedule_15_0_e336: f64 = (params.p36 - noise_metadata_schedule_15_0_e335);
        (noise_metadata_schedule_15_0_e336,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_15_0_e338;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_16_0_e346,) = {
    if ((w[114] == 0.0) && (w[115] == 0.0)) {
        (w[23],)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_16_0_e346;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_17_0_e349: f64 = (w[23] + 273.15);
            w[24] = noise_metadata_schedule_17_0_e349;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_18_0_e352: f64 = (1.3806505e-23 * w[24]);
            let noise_metadata_schedule_18_0_e354: f64 = (noise_metadata_schedule_18_0_e352 / 1.60217653e-19);
            w[71] = noise_metadata_schedule_18_0_e354;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_20_0_e360: f64 = (w[24] - w[15]);
            w[69] = noise_metadata_schedule_20_0_e360;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_21_0_e363: f64 = (params.p0 * w[13]);
            w[26] = noise_metadata_schedule_21_0_e363;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_22_0_e366: f64 = (params.p1 * w[13]);
            w[27] = noise_metadata_schedule_22_0_e366;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_27_0_e381: f64 = (params.p2 * w[13]);
            w[30] = noise_metadata_schedule_27_0_e381;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_28_0_e384: f64 = (params.p3 * w[14]);
            w[31] = noise_metadata_schedule_28_0_e384;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_29_0_e387: f64 = (params.p4 * w[13]);
            w[32] = noise_metadata_schedule_29_0_e387;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_30_0_e390: f64 = (params.p6 * w[14]);
            w[33] = noise_metadata_schedule_30_0_e390;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_31_0_e393: f64 = (params.p7 * w[13]);
            w[34] = noise_metadata_schedule_31_0_e393;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_32_0_e396: f64 = (w[27] * w[26]);
            w[35] = noise_metadata_schedule_32_0_e396;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_34_0_e413: f64 = if params.p5 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_34_0_e416: f64 = if params.p8 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_34_0_e417: f64 = (noise_metadata_schedule_34_0_e413 + noise_metadata_schedule_34_0_e416);
            let noise_metadata_schedule_34_0_e418: f64 = (0.5 * noise_metadata_schedule_34_0_e417);
            let noise_metadata_schedule_34_0_e422: f64 = (params.p44 / w[26]);
            let noise_metadata_schedule_34_0_e423: f64 = (params.p43 + noise_metadata_schedule_34_0_e422);
            let noise_metadata_schedule_34_0_e424: f64 = (noise_metadata_schedule_34_0_e418 * noise_metadata_schedule_34_0_e423);
            w[25] = noise_metadata_schedule_34_0_e424;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_35_0_e427: f64 = (w[26] + params.p38);
            let noise_metadata_schedule_35_0_e430: f64 = (params.p39 / w[26]);
            let noise_metadata_schedule_35_0_e431: f64 = (noise_metadata_schedule_35_0_e427 + noise_metadata_schedule_35_0_e430);
            let noise_metadata_schedule_35_0_e435: f64 = (-w[26]);
            let noise_metadata_schedule_35_0_e437: f64 = (noise_metadata_schedule_35_0_e435 / params.p41);
            let noise_metadata_schedule_35_0_e438: f64 = (noise_metadata_schedule_35_0_e437).exp();
            let noise_metadata_schedule_35_0_e439: f64 = (1.0 - noise_metadata_schedule_35_0_e438);
            let noise_metadata_schedule_35_0_e440: f64 = (params.p42 * noise_metadata_schedule_35_0_e439);
            let noise_metadata_schedule_35_0_e441: f64 = (noise_metadata_schedule_35_0_e431 + noise_metadata_schedule_35_0_e440);
            let noise_metadata_schedule_35_0_e445: f64 = (params.p40 * w[30]);
            let noise_metadata_schedule_35_0_e447: f64 = (noise_metadata_schedule_35_0_e445 / w[35]);
            let noise_metadata_schedule_35_0_e448: f64 = (1.0 - noise_metadata_schedule_35_0_e447);
            let noise_metadata_schedule_35_0_e449: f64 = (noise_metadata_schedule_35_0_e441 / noise_metadata_schedule_35_0_e448);
            w[4] = noise_metadata_schedule_35_0_e449;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_36_0_e452: f64 = (w[27] + w[25]);
            w[3] = noise_metadata_schedule_36_0_e452;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_37_0_e456,) = {
    if (params.p127 != 0.0) {
        (w[4],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_37_0_e456;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_38_0_e460,) = {
    if (params.p127 != 0.0) {
        (w[3],)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_38_0_e460;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_39_0_e465,) = {
    if (params.p127 == 0.0) {
        (w[26],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_39_0_e465;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_40_0_e470,) = {
    if (params.p127 == 0.0) {
        (w[27],)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_40_0_e470;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_41_0_e487,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_41_0_e475: f64 = (params.p119 * params.p122);
        let noise_metadata_schedule_41_0_e476: f64 = (w[4] + noise_metadata_schedule_41_0_e475);
        let noise_metadata_schedule_41_0_e479: f64 = (params.p11 * params.p125);
        let noise_metadata_schedule_41_0_e482: f64 = (w[12] * w[37]);
        let noise_metadata_schedule_41_0_e483: f64 = (noise_metadata_schedule_41_0_e482).sqrt();
        let noise_metadata_schedule_41_0_e484: f64 = (noise_metadata_schedule_41_0_e479 / noise_metadata_schedule_41_0_e483);
        let noise_metadata_schedule_41_0_e485: f64 = (noise_metadata_schedule_41_0_e476 + noise_metadata_schedule_41_0_e484);
        (noise_metadata_schedule_41_0_e485,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_41_0_e487;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_42_0_e504,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_42_0_e492: f64 = (params.p120 * params.p123);
        let noise_metadata_schedule_42_0_e493: f64 = (w[3] + noise_metadata_schedule_42_0_e492);
        let noise_metadata_schedule_42_0_e496: f64 = (params.p12 * params.p126);
        let noise_metadata_schedule_42_0_e499: f64 = (w[12] * w[38]);
        let noise_metadata_schedule_42_0_e500: f64 = (noise_metadata_schedule_42_0_e499).sqrt();
        let noise_metadata_schedule_42_0_e501: f64 = (noise_metadata_schedule_42_0_e496 / noise_metadata_schedule_42_0_e500);
        let noise_metadata_schedule_42_0_e502: f64 = (noise_metadata_schedule_42_0_e493 + noise_metadata_schedule_42_0_e501);
        (noise_metadata_schedule_42_0_e502,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_42_0_e504;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_43_0_e524,) = {
    if (params.p16 != 0.0) {
        let noise_metadata_schedule_43_0_e509: f64 = (params.p118 * params.p121);
        let noise_metadata_schedule_43_0_e512: f64 = (params.p10 * params.p124);
        let noise_metadata_schedule_43_0_e515: f64 = (w[12] * w[37]);
        let noise_metadata_schedule_43_0_e517: f64 = (noise_metadata_schedule_43_0_e515 * w[38]);
        let noise_metadata_schedule_43_0_e518: f64 = (noise_metadata_schedule_43_0_e517).sqrt();
        let noise_metadata_schedule_43_0_e519: f64 = (noise_metadata_schedule_43_0_e512 / noise_metadata_schedule_43_0_e518);
        let noise_metadata_schedule_43_0_e520: f64 = (noise_metadata_schedule_43_0_e509 + noise_metadata_schedule_43_0_e519);
        let noise_metadata_schedule_43_0_e521: f64 = (0.01 * noise_metadata_schedule_43_0_e520);
        let noise_metadata_schedule_43_0_e522: f64 = (noise_metadata_schedule_43_0_e521).exp();
        (noise_metadata_schedule_43_0_e522,)
    } else {
        (w[40],)
    }
};
            w[40] = noise_metadata_schedule_43_0_e524;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_44_0_e535: f64 = if ((params.p119 != 0.0) && ((params.p125 > 0.0) || (params.p122 > 0.0))) { 1.0 } else { 0.0 };
            w[120] = noise_metadata_schedule_44_0_e535;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_45_0_e547,) = {
    if ((params.p16 == 0.0) && (w[120] != 0.0)) {
        let noise_metadata_schedule_45_0_e543: f64 = (w[12] * w[37]);
        let noise_metadata_schedule_45_0_e544: f64 = (noise_metadata_schedule_45_0_e543).sqrt();
        let noise_metadata_schedule_45_0_e545: f64 = (params.p125 / noise_metadata_schedule_45_0_e544);
        (noise_metadata_schedule_45_0_e545,)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_45_0_e547;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_46_0_e565,) = {
    if ((params.p16 == 0.0) && (w[120] != 0.0)) {
        let noise_metadata_schedule_46_0_e556: f64 = (params.p122 * params.p122);
        let noise_metadata_schedule_46_0_e559: f64 = (w[39] * w[39]);
        let noise_metadata_schedule_46_0_e560: f64 = (noise_metadata_schedule_46_0_e556 + noise_metadata_schedule_46_0_e559);
        let noise_metadata_schedule_46_0_e561: f64 = (noise_metadata_schedule_46_0_e560).sqrt();
        let noise_metadata_schedule_46_0_e562: f64 = (params.p119 * noise_metadata_schedule_46_0_e561);
        let noise_metadata_schedule_46_0_e563: f64 = (w[4] + noise_metadata_schedule_46_0_e562);
        (noise_metadata_schedule_46_0_e563,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_46_0_e565;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_47_0_e576: f64 = if ((params.p120 != 0.0) && ((params.p126 > 0.0) || (params.p123 > 0.0))) { 1.0 } else { 0.0 };
            w[121] = noise_metadata_schedule_47_0_e576;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_48_0_e588,) = {
    if ((params.p16 == 0.0) && (w[121] != 0.0)) {
        let noise_metadata_schedule_48_0_e584: f64 = (w[12] * w[38]);
        let noise_metadata_schedule_48_0_e585: f64 = (noise_metadata_schedule_48_0_e584).sqrt();
        let noise_metadata_schedule_48_0_e586: f64 = (params.p126 / noise_metadata_schedule_48_0_e585);
        (noise_metadata_schedule_48_0_e586,)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_48_0_e588;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_49_0_e606,) = {
    if ((params.p16 == 0.0) && (w[121] != 0.0)) {
        let noise_metadata_schedule_49_0_e597: f64 = (params.p123 * params.p123);
        let noise_metadata_schedule_49_0_e600: f64 = (w[39] * w[39]);
        let noise_metadata_schedule_49_0_e601: f64 = (noise_metadata_schedule_49_0_e597 + noise_metadata_schedule_49_0_e600);
        let noise_metadata_schedule_49_0_e602: f64 = (noise_metadata_schedule_49_0_e601).sqrt();
        let noise_metadata_schedule_49_0_e603: f64 = (params.p120 * noise_metadata_schedule_49_0_e602);
        let noise_metadata_schedule_49_0_e604: f64 = (w[3] + noise_metadata_schedule_49_0_e603);
        (noise_metadata_schedule_49_0_e604,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_49_0_e606;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_50_0_e617: f64 = if ((params.p118 != 0.0) && ((params.p124 > 0.0) || (params.p121 > 0.0))) { 1.0 } else { 0.0 };
            w[122] = noise_metadata_schedule_50_0_e617;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_51_0_e631,) = {
    if ((params.p16 == 0.0) && (w[122] != 0.0)) {
        let noise_metadata_schedule_51_0_e625: f64 = (w[12] * w[37]);
        let noise_metadata_schedule_51_0_e627: f64 = (noise_metadata_schedule_51_0_e625 * w[38]);
        let noise_metadata_schedule_51_0_e628: f64 = (noise_metadata_schedule_51_0_e627).sqrt();
        let noise_metadata_schedule_51_0_e629: f64 = (params.p124 / noise_metadata_schedule_51_0_e628);
        (noise_metadata_schedule_51_0_e629,)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_51_0_e631;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_52_0_e650,) = {
    if ((params.p16 == 0.0) && (w[122] != 0.0)) {
        let noise_metadata_schedule_52_0_e638: f64 = (0.01 * params.p118);
        let noise_metadata_schedule_52_0_e641: f64 = (params.p121 * params.p121);
        let noise_metadata_schedule_52_0_e644: f64 = (w[39] * w[39]);
        let noise_metadata_schedule_52_0_e645: f64 = (noise_metadata_schedule_52_0_e641 + noise_metadata_schedule_52_0_e644);
        let noise_metadata_schedule_52_0_e646: f64 = (noise_metadata_schedule_52_0_e645).sqrt();
        let noise_metadata_schedule_52_0_e647: f64 = (noise_metadata_schedule_52_0_e638 * noise_metadata_schedule_52_0_e646);
        let noise_metadata_schedule_52_0_e648: f64 = (noise_metadata_schedule_52_0_e647).exp();
        (noise_metadata_schedule_52_0_e648,)
    } else {
        (w[40],)
    }
};
            w[40] = noise_metadata_schedule_52_0_e650;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_53_0_e658,) = {
    if ((params.p16 == 0.0) && (w[122] == 0.0)) {
        (1.0,)
    } else {
        (w[40],)
    }
};
            w[40] = noise_metadata_schedule_53_0_e658;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_56_0_e667: f64 = (w[3] + params.p45);
            w[28] = noise_metadata_schedule_56_0_e667;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_58_0_e674,) = {
    if (params.p53 != 0.0) {
        (w[4],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_58_0_e674;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_59_0_e678,) = {
    if (params.p53 != 0.0) {
        (w[3],)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_59_0_e678;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_60_0_e683,) = {
    if (params.p53 == 0.0) {
        (w[26],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_60_0_e683;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_61_0_e688,) = {
    if (params.p53 == 0.0) {
        (w[27],)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_61_0_e688;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_62_0_e692: f64 = (w[38]).powf(params.p56);
            let noise_metadata_schedule_62_0_e693: f64 = (1.0 / noise_metadata_schedule_62_0_e692);
            w[42] = noise_metadata_schedule_62_0_e693;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_63_0_e697: f64 = (w[37]).powf(params.p58);
            let noise_metadata_schedule_63_0_e698: f64 = (1.0 / noise_metadata_schedule_63_0_e697);
            w[43] = noise_metadata_schedule_63_0_e698;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_64_0_e703: f64 = (params.p55 * w[42]);
            let noise_metadata_schedule_64_0_e704: f64 = (1.0 + noise_metadata_schedule_64_0_e703);
            let noise_metadata_schedule_64_0_e705: f64 = (params.p54 * noise_metadata_schedule_64_0_e704);
            let noise_metadata_schedule_64_0_e709: f64 = (params.p57 * w[43]);
            let noise_metadata_schedule_64_0_e710: f64 = (1.0 + noise_metadata_schedule_64_0_e709);
            let noise_metadata_schedule_64_0_e711: f64 = (noise_metadata_schedule_64_0_e705 * noise_metadata_schedule_64_0_e710);
            let noise_metadata_schedule_64_0_e715: f64 = (params.p59 * w[42]);
            let noise_metadata_schedule_64_0_e717: f64 = (noise_metadata_schedule_64_0_e715 * w[43]);
            let noise_metadata_schedule_64_0_e718: f64 = (1.0 + noise_metadata_schedule_64_0_e717);
            let noise_metadata_schedule_64_0_e719: f64 = (noise_metadata_schedule_64_0_e711 * noise_metadata_schedule_64_0_e718);
            let noise_metadata_schedule_64_0_e725: f64 = (w[69] * params.p104);
            let noise_metadata_schedule_64_0_e726: f64 = (params.p103 + noise_metadata_schedule_64_0_e725);
            let noise_metadata_schedule_64_0_e727: f64 = (w[69] * noise_metadata_schedule_64_0_e726);
            let noise_metadata_schedule_64_0_e728: f64 = (1.0 + noise_metadata_schedule_64_0_e727);
            let noise_metadata_schedule_64_0_e729: f64 = (noise_metadata_schedule_64_0_e719 * noise_metadata_schedule_64_0_e728);
            w[41] = noise_metadata_schedule_64_0_e729;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_65_0_e735,) = {
    if (w[41] > 0.1) {
        (w[41],)
    } else {
        (0.1,)
    }
};
            w[41] = noise_metadata_schedule_65_0_e735;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_66_0_e737: f64 = (w[41]).sqrt();
            let noise_metadata_schedule_66_0_e740: f64 = (w[41] + 10000.0);
            let noise_metadata_schedule_66_0_e741: f64 = (noise_metadata_schedule_66_0_e737 / noise_metadata_schedule_66_0_e740);
            w[44] = noise_metadata_schedule_66_0_e741;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_67_0_e759,) = {
    if (params.p15 != 0.0) {
        (0.0,)
    } else {
        let noise_metadata_schedule_67_0_e747: f64 = (params.p50 * w[37]);
        let noise_metadata_schedule_67_0_e750: f64 = (params.p51 * w[38]);
        let noise_metadata_schedule_67_0_e751: f64 = (noise_metadata_schedule_67_0_e747 + noise_metadata_schedule_67_0_e750);
        let noise_metadata_schedule_67_0_e753: f64 = (noise_metadata_schedule_67_0_e751 + params.p52);
        let noise_metadata_schedule_67_0_e756: f64 = (w[37] * w[38]);
        let noise_metadata_schedule_67_0_e757: f64 = (noise_metadata_schedule_67_0_e753 / noise_metadata_schedule_67_0_e756);
        let noise_metadata_schedule_67_0_e758: f64 = (params.p49 + noise_metadata_schedule_67_0_e757);
        (noise_metadata_schedule_67_0_e758,)
    }
};
            w[45] = noise_metadata_schedule_67_0_e759;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_68_0_e762: f64 = if w[45] < w[44] { 1.0 } else { 0.0 };
            w[126] = noise_metadata_schedule_68_0_e762;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_69_0_e771,) = {
    if (w[126] != 0.0) {
        let (noise_metadata_schedule_69_0_e769,) = {
            if (w[45] > 0.0) {
                (w[45],)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_69_0_e769,)
    } else {
        (w[45],)
    }
};
            w[45] = noise_metadata_schedule_69_0_e771;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_70_0_e777,) = {
    if (w[126] != 0.0) {
        let noise_metadata_schedule_70_0_e775: f64 = (w[44] * w[44]);
        (noise_metadata_schedule_70_0_e775,)
    } else {
        (w[46],)
    }
};
            w[46] = noise_metadata_schedule_70_0_e777;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_71_0_e784,) = {
    if (w[126] == 0.0) {
        let noise_metadata_schedule_71_0_e782: f64 = (w[45] * w[45]);
        (noise_metadata_schedule_71_0_e782,)
    } else {
        (w[46],)
    }
};
            w[46] = noise_metadata_schedule_71_0_e784;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_72_0_e787: f64 = (0.5 / w[46]);
            let noise_metadata_schedule_72_0_e790: f64 = (w[41] * 0.5);
            let noise_metadata_schedule_72_0_e791: f64 = (noise_metadata_schedule_72_0_e787 - noise_metadata_schedule_72_0_e790);
            w[48] = noise_metadata_schedule_72_0_e791;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_73_0_e794: f64 = if params.p63 > 1.0 { 1.0 } else { 0.0 };
            w[127] = noise_metadata_schedule_73_0_e794;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_74_0_e804,) = {
    if (w[127] != 0.0) {
        let noise_metadata_schedule_74_0_e799: f64 = (2.0 * params.p64);
        let noise_metadata_schedule_74_0_e801: f64 = (noise_metadata_schedule_74_0_e799 / w[46]);
        let noise_metadata_schedule_74_0_e802: f64 = (w[48] - noise_metadata_schedule_74_0_e801);
        (noise_metadata_schedule_74_0_e802,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_74_0_e804;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_75_0_e814,) = {
    if (w[127] != 0.0) {
        let noise_metadata_schedule_75_0_e808: f64 = (0.1666666666666667 / w[46]);
        let noise_metadata_schedule_75_0_e811: f64 = (w[41] * 0.5);
        let noise_metadata_schedule_75_0_e812: f64 = (noise_metadata_schedule_75_0_e808 - noise_metadata_schedule_75_0_e811);
        (noise_metadata_schedule_75_0_e812,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_75_0_e814;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_76_0_e817: f64 = if params.p63 > 0.0 { 1.0 } else { 0.0 };
            w[128] = noise_metadata_schedule_76_0_e817;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_77_0_e831,) = {
    if ((w[127] == 0.0) && (w[128] != 0.0)) {
        let noise_metadata_schedule_77_0_e825: f64 = (2.0 * params.p64);
        let noise_metadata_schedule_77_0_e827: f64 = (noise_metadata_schedule_77_0_e825 / w[46]);
        let noise_metadata_schedule_77_0_e828: f64 = (noise_metadata_schedule_77_0_e827).sqrt();
        let noise_metadata_schedule_77_0_e829: f64 = (w[48] - noise_metadata_schedule_77_0_e828);
        (noise_metadata_schedule_77_0_e829,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_77_0_e831;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_78_0_e838,) = {
    if ((w[127] == 0.0) && (w[128] != 0.0)) {
        (0.0,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_78_0_e838;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_79_0_e846,) = {
    if ((w[127] == 0.0) && (w[128] == 0.0)) {
        (w[48],)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_79_0_e846;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_80_0_e854,) = {
    if ((w[127] == 0.0) && (w[128] == 0.0)) {
        (0.0,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_80_0_e854;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_81_0_e859: f64 = (params.p48 / w[3]);
            let noise_metadata_schedule_81_0_e860: f64 = (1.0 + noise_metadata_schedule_81_0_e859);
            let noise_metadata_schedule_81_0_e861: f64 = (params.p47 / noise_metadata_schedule_81_0_e860);
            w[106] = noise_metadata_schedule_81_0_e861;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_82_0_e864: f64 = if params.p63 > 1.0 { 1.0 } else { 0.0 };
            w[129] = noise_metadata_schedule_82_0_e864;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_83_0_e870,) = {
    if (w[129] != 0.0) {
        let noise_metadata_schedule_83_0_e868: f64 = (params.p46 * w[71]);
        (noise_metadata_schedule_83_0_e868,)
    } else {
        (w[105],)
    }
};
            w[105] = noise_metadata_schedule_83_0_e870;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_84_0_e891,) = {
    if (w[129] != 0.0) {
        let (noise_metadata_schedule_84_0_e889,) = {
            if (params.p63 > 2.0) {
                let noise_metadata_schedule_84_0_e877: f64 = (0.55 * w[71]);
                let noise_metadata_schedule_84_0_e880: f64 = (-w[106]);
                let noise_metadata_schedule_84_0_e882: f64 = (noise_metadata_schedule_84_0_e880 / w[71]);
                let noise_metadata_schedule_84_0_e883: f64 = (noise_metadata_schedule_84_0_e882).exp();
                let noise_metadata_schedule_84_0_e884: f64 = (1.0 + noise_metadata_schedule_84_0_e883);
                let noise_metadata_schedule_84_0_e885: f64 = (noise_metadata_schedule_84_0_e877 * noise_metadata_schedule_84_0_e884);
                (noise_metadata_schedule_84_0_e885,)
            } else {
                let noise_metadata_schedule_84_0_e888: f64 = (1.1 * w[71]);
                (noise_metadata_schedule_84_0_e888,)
            }
        };
        (noise_metadata_schedule_84_0_e889,)
    } else {
        (w[107],)
    }
};
            w[107] = noise_metadata_schedule_84_0_e891;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_85_0_e894: f64 = if params.p63 > 0.0 { 1.0 } else { 0.0 };
            w[130] = noise_metadata_schedule_85_0_e894;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_86_0_e905,) = {
    if ((w[129] == 0.0) && (w[130] != 0.0)) {
        let noise_metadata_schedule_86_0_e901: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_86_0_e903: f64 = (noise_metadata_schedule_86_0_e901 * w[71]);
        (noise_metadata_schedule_86_0_e903,)
    } else {
        (w[105],)
    }
};
            w[105] = noise_metadata_schedule_86_0_e905;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_87_0_e916,) = {
    if ((w[129] == 0.0) && (w[130] != 0.0)) {
        let noise_metadata_schedule_87_0_e912: f64 = (4.0 * w[106]);
        let noise_metadata_schedule_87_0_e914: f64 = (noise_metadata_schedule_87_0_e912 * w[106]);
        (noise_metadata_schedule_87_0_e914,)
    } else {
        (w[107],)
    }
};
            w[107] = noise_metadata_schedule_87_0_e916;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_88_0_e926,) = {
    if ((w[129] == 0.0) && (w[130] == 0.0)) {
        let noise_metadata_schedule_88_0_e924: f64 = (params.p46 * w[71]);
        (noise_metadata_schedule_88_0_e924,)
    } else {
        (w[105],)
    }
};
            w[105] = noise_metadata_schedule_88_0_e926;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_89_0_e938,) = {
    if ((w[129] == 0.0) && (w[130] == 0.0)) {
        let noise_metadata_schedule_89_0_e934: f64 = (4.0 * w[106]);
        let noise_metadata_schedule_89_0_e936: f64 = (noise_metadata_schedule_89_0_e934 * w[106]);
        (noise_metadata_schedule_89_0_e936,)
    } else {
        (w[107],)
    }
};
            w[107] = noise_metadata_schedule_89_0_e938;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_90_0_e941: f64 = (params.p37 * w[40]);
            let noise_metadata_schedule_90_0_e944: f64 = (w[3] / w[4]);
            let noise_metadata_schedule_90_0_e945: f64 = (noise_metadata_schedule_90_0_e941 * noise_metadata_schedule_90_0_e944);
            let noise_metadata_schedule_90_0_e949: f64 = (w[41]).sqrt();
            let noise_metadata_schedule_90_0_e950: f64 = (w[45] * noise_metadata_schedule_90_0_e949);
            let noise_metadata_schedule_90_0_e951: f64 = (1.0 - noise_metadata_schedule_90_0_e950);
            let noise_metadata_schedule_90_0_e952: f64 = (noise_metadata_schedule_90_0_e945 * noise_metadata_schedule_90_0_e951);
            w[5] = noise_metadata_schedule_90_0_e952;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_92_0_e962: f64 = if ((params.p66 > 0.0) && (params.p5 > 0.0)) { 1.0 } else { 0.0 };
            w[132] = noise_metadata_schedule_92_0_e962;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_93_0_e972,) = {
    if (w[132] != 0.0) {
        let noise_metadata_schedule_93_0_e967: f64 = (params.p67 / w[26]);
        let noise_metadata_schedule_93_0_e968: f64 = (params.p66 + noise_metadata_schedule_93_0_e967);
        let noise_metadata_schedule_93_0_e970: f64 = (noise_metadata_schedule_93_0_e968 / params.p5);
        (noise_metadata_schedule_93_0_e970,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_93_0_e972;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_94_0_e977,) = {
    if (w[132] == 0.0) {
        (0.0,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_94_0_e977;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_95_0_e984: f64 = if ((params.p66 > 0.0) && (params.p8 > 0.0)) { 1.0 } else { 0.0 };
            w[133] = noise_metadata_schedule_95_0_e984;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_96_0_e994,) = {
    if (w[133] != 0.0) {
        let noise_metadata_schedule_96_0_e989: f64 = (params.p67 / w[26]);
        let noise_metadata_schedule_96_0_e990: f64 = (params.p66 + noise_metadata_schedule_96_0_e989);
        let noise_metadata_schedule_96_0_e992: f64 = (noise_metadata_schedule_96_0_e990 / params.p8);
        (noise_metadata_schedule_96_0_e992,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_96_0_e994;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_97_0_e999,) = {
    if (w[133] == 0.0) {
        (0.0,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_97_0_e999;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_102_0_e1053: f64 = (params.p97 / w[4]);
            let noise_metadata_schedule_102_0_e1054: f64 = (params.p93 + noise_metadata_schedule_102_0_e1053);
            let noise_metadata_schedule_102_0_e1058: f64 = if params.p5 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_102_0_e1061: f64 = if params.p8 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_102_0_e1062: f64 = (noise_metadata_schedule_102_0_e1058 + noise_metadata_schedule_102_0_e1061);
            let noise_metadata_schedule_102_0_e1063: f64 = (0.5 * noise_metadata_schedule_102_0_e1062);
            let noise_metadata_schedule_102_0_e1067: f64 = (params.p99 / w[4]);
            let noise_metadata_schedule_102_0_e1068: f64 = (params.p95 + noise_metadata_schedule_102_0_e1067);
            let noise_metadata_schedule_102_0_e1069: f64 = (noise_metadata_schedule_102_0_e1063 * noise_metadata_schedule_102_0_e1068);
            let noise_metadata_schedule_102_0_e1071: f64 = (noise_metadata_schedule_102_0_e1069 / w[3]);
            let noise_metadata_schedule_102_0_e1072: f64 = (noise_metadata_schedule_102_0_e1054 + noise_metadata_schedule_102_0_e1071);
            w[52] = noise_metadata_schedule_102_0_e1072;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_103_0_e1076: f64 = (params.p98 / w[4]);
            let noise_metadata_schedule_103_0_e1077: f64 = (params.p94 + noise_metadata_schedule_103_0_e1076);
            let noise_metadata_schedule_103_0_e1081: f64 = if params.p5 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_103_0_e1084: f64 = if params.p8 > 0.0 { 1.0 } else { 0.0 };
            let noise_metadata_schedule_103_0_e1085: f64 = (noise_metadata_schedule_103_0_e1081 + noise_metadata_schedule_103_0_e1084);
            let noise_metadata_schedule_103_0_e1086: f64 = (0.5 * noise_metadata_schedule_103_0_e1085);
            let noise_metadata_schedule_103_0_e1090: f64 = (params.p100 / w[4]);
            let noise_metadata_schedule_103_0_e1091: f64 = (params.p96 + noise_metadata_schedule_103_0_e1090);
            let noise_metadata_schedule_103_0_e1092: f64 = (noise_metadata_schedule_103_0_e1086 * noise_metadata_schedule_103_0_e1091);
            let noise_metadata_schedule_103_0_e1094: f64 = (noise_metadata_schedule_103_0_e1092 / w[3]);
            let noise_metadata_schedule_103_0_e1095: f64 = (noise_metadata_schedule_103_0_e1077 + noise_metadata_schedule_103_0_e1094);
            w[53] = noise_metadata_schedule_103_0_e1095;
        }
        if (active[0] & 0x3f) != 0 {
            w[10] = (ctx.node_voltage(self.nodes[3]) - 0.0);
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_109_0_e1126: f64 = (-params.p21);
            let noise_metadata_schedule_109_0_e1128: f64 = (noise_metadata_schedule_109_0_e1126 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            w[64] = noise_metadata_schedule_109_0_e1128;
        }
        if (active[0] & 0x13) != 0 {
            let noise_metadata_schedule_110_0_e1130: f64 = (-params.p21);
            let noise_metadata_schedule_110_0_e1132: f64 = (noise_metadata_schedule_110_0_e1130 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[4])));
            w[65] = noise_metadata_schedule_110_0_e1132;
        }
        if (active[0] & 0x23) != 0 {
            let noise_metadata_schedule_111_0_e1134: f64 = (-params.p21);
            let noise_metadata_schedule_111_0_e1136: f64 = (noise_metadata_schedule_111_0_e1134 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            w[66] = noise_metadata_schedule_111_0_e1136;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_112_0_e1137: f64 = ctx.temperature();
            let noise_metadata_schedule_112_0_e1139: f64 = (noise_metadata_schedule_112_0_e1137 + params.p9);
            let noise_metadata_schedule_112_0_e1141: f64 = (noise_metadata_schedule_112_0_e1139 + w[10]);
            let noise_metadata_schedule_112_0_e1143: f64 = (noise_metadata_schedule_112_0_e1141 - 273.15);
            w[23] = noise_metadata_schedule_112_0_e1143;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_113_0_e1147: f64 = (params.p35 + 1.0);
            let noise_metadata_schedule_113_0_e1148: f64 = if w[23] < noise_metadata_schedule_113_0_e1147 { 1.0 } else { 0.0 };
            w[134] = noise_metadata_schedule_113_0_e1148;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_114_0_e1159,) = {
    if (w[134] != 0.0) {
        let noise_metadata_schedule_114_0_e1153: f64 = (w[23] - params.p35);
        let noise_metadata_schedule_114_0_e1155: f64 = (noise_metadata_schedule_114_0_e1153 - 1.0);
        let noise_metadata_schedule_114_0_e1156: f64 = (noise_metadata_schedule_114_0_e1155).exp();
        let noise_metadata_schedule_114_0_e1157: f64 = (params.p35 + noise_metadata_schedule_114_0_e1156);
        (noise_metadata_schedule_114_0_e1157,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_114_0_e1159;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_115_0_e1163: f64 = (params.p36 - 1.0);
            let noise_metadata_schedule_115_0_e1164: f64 = if w[23] > noise_metadata_schedule_115_0_e1163 { 1.0 } else { 0.0 };
            w[135] = noise_metadata_schedule_115_0_e1164;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_116_0_e1178,) = {
    if ((w[134] == 0.0) && (w[135] != 0.0)) {
        let noise_metadata_schedule_116_0_e1172: f64 = (params.p36 - w[23]);
        let noise_metadata_schedule_116_0_e1174: f64 = (noise_metadata_schedule_116_0_e1172 - 1.0);
        let noise_metadata_schedule_116_0_e1175: f64 = (noise_metadata_schedule_116_0_e1174).exp();
        let noise_metadata_schedule_116_0_e1176: f64 = (params.p36 - noise_metadata_schedule_116_0_e1175);
        (noise_metadata_schedule_116_0_e1176,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_116_0_e1178;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_117_0_e1186,) = {
    if ((w[134] == 0.0) && (w[135] == 0.0)) {
        (w[23],)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_117_0_e1186;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_118_0_e1189: f64 = (w[23] + 273.15);
            w[24] = noise_metadata_schedule_118_0_e1189;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_119_0_e1192: f64 = (1.3806505e-23 * w[24]);
            let noise_metadata_schedule_119_0_e1194: f64 = (noise_metadata_schedule_119_0_e1192 / 1.60217653e-19);
            w[70] = noise_metadata_schedule_119_0_e1194;
        }
        if (active[0] & 0x33) != 0 {
            let noise_metadata_schedule_120_0_e1197: f64 = (w[24] / w[15]);
            w[68] = noise_metadata_schedule_120_0_e1197;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_121_0_e1200: f64 = (w[24] - w[15]);
            w[69] = noise_metadata_schedule_121_0_e1200;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_122_0_e1206: f64 = (w[69] * w[53]);
            let noise_metadata_schedule_122_0_e1207: f64 = (w[52] + noise_metadata_schedule_122_0_e1206);
            let noise_metadata_schedule_122_0_e1208: f64 = (w[69] * noise_metadata_schedule_122_0_e1207);
            let noise_metadata_schedule_122_0_e1209: f64 = (1.0 + noise_metadata_schedule_122_0_e1208);
            w[57] = noise_metadata_schedule_122_0_e1209;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_123_0_e1213: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_123_0_e1214: f64 = if w[57] < noise_metadata_schedule_123_0_e1213 { 1.0 } else { 0.0 };
            w[136] = noise_metadata_schedule_123_0_e1214;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_124_0_e1229,) = {
    if (w[136] != 0.0) {
        let noise_metadata_schedule_124_0_e1221: f64 = (w[57] - 0.01);
        let noise_metadata_schedule_124_0_e1222: f64 = (10.0 * noise_metadata_schedule_124_0_e1221);
        let noise_metadata_schedule_124_0_e1224: f64 = (noise_metadata_schedule_124_0_e1222 - 1.0);
        let noise_metadata_schedule_124_0_e1225: f64 = (noise_metadata_schedule_124_0_e1224).exp();
        let noise_metadata_schedule_124_0_e1226: f64 = (0.1 * noise_metadata_schedule_124_0_e1225);
        let noise_metadata_schedule_124_0_e1227: f64 = (0.01 + noise_metadata_schedule_124_0_e1226);
        (noise_metadata_schedule_124_0_e1227,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_124_0_e1229;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_125_0_e1234,) = {
    if (w[136] == 0.0) {
        (w[57],)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_125_0_e1234;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_126_0_e1249,) = {
    if (params.p63 != 0.0) {
        let noise_metadata_schedule_126_0_e1241: f64 = (w[41]).sqrt();
        let noise_metadata_schedule_126_0_e1242: f64 = (w[45] * noise_metadata_schedule_126_0_e1241);
        let noise_metadata_schedule_126_0_e1243: f64 = (1.0 - noise_metadata_schedule_126_0_e1242);
        let noise_metadata_schedule_126_0_e1244: f64 = (w[5] * noise_metadata_schedule_126_0_e1243);
        let noise_metadata_schedule_126_0_e1246: f64 = (noise_metadata_schedule_126_0_e1244 * w[57]);
        let noise_metadata_schedule_126_0_e1247: f64 = (1.0 / noise_metadata_schedule_126_0_e1246);
        (noise_metadata_schedule_126_0_e1247,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_126_0_e1249;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_127_0_e1258,) = {
    if (params.p63 == 0.0) {
        let noise_metadata_schedule_127_0_e1255: f64 = (w[5] * w[57]);
        let noise_metadata_schedule_127_0_e1256: f64 = (1.0 / noise_metadata_schedule_127_0_e1255);
        (noise_metadata_schedule_127_0_e1256,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_127_0_e1258;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_128_0_e1264: f64 = (w[69] * params.p102);
            let noise_metadata_schedule_128_0_e1265: f64 = (params.p101 + noise_metadata_schedule_128_0_e1264);
            let noise_metadata_schedule_128_0_e1266: f64 = (w[69] * noise_metadata_schedule_128_0_e1265);
            let noise_metadata_schedule_128_0_e1267: f64 = (1.0 + noise_metadata_schedule_128_0_e1266);
            w[58] = noise_metadata_schedule_128_0_e1267;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_129_0_e1271: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_129_0_e1272: f64 = if w[58] < noise_metadata_schedule_129_0_e1271 { 1.0 } else { 0.0 };
            w[137] = noise_metadata_schedule_129_0_e1272;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_130_0_e1287,) = {
    if (w[137] != 0.0) {
        let noise_metadata_schedule_130_0_e1279: f64 = (w[58] - 0.01);
        let noise_metadata_schedule_130_0_e1280: f64 = (10.0 * noise_metadata_schedule_130_0_e1279);
        let noise_metadata_schedule_130_0_e1282: f64 = (noise_metadata_schedule_130_0_e1280 - 1.0);
        let noise_metadata_schedule_130_0_e1283: f64 = (noise_metadata_schedule_130_0_e1282).exp();
        let noise_metadata_schedule_130_0_e1284: f64 = (0.1 * noise_metadata_schedule_130_0_e1283);
        let noise_metadata_schedule_130_0_e1285: f64 = (0.01 + noise_metadata_schedule_130_0_e1284);
        (noise_metadata_schedule_130_0_e1285,)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_130_0_e1287;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_131_0_e1292,) = {
    if (w[137] == 0.0) {
        (w[58],)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_131_0_e1292;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_132_0_e1295: f64 = (w[68]).powf(params.p92);
            w[59] = noise_metadata_schedule_132_0_e1295;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_133_0_e1298: f64 = if params.p69 > 0.0 { 1.0 } else { 0.0 };
            w[138] = noise_metadata_schedule_133_0_e1298;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_134_0_e1319,) = {
    if (w[138] != 0.0) {
        let noise_metadata_schedule_134_0_e1302: f64 = (-params.p90);
        let noise_metadata_schedule_134_0_e1305: f64 = (1.0 - w[68]);
        let noise_metadata_schedule_134_0_e1306: f64 = (noise_metadata_schedule_134_0_e1302 * noise_metadata_schedule_134_0_e1305);
        let noise_metadata_schedule_134_0_e1308: f64 = (noise_metadata_schedule_134_0_e1306 / w[70]);
        let noise_metadata_schedule_134_0_e1311: f64 = (w[68]).ln();
        let noise_metadata_schedule_134_0_e1312: f64 = (params.p91 * noise_metadata_schedule_134_0_e1311);
        let noise_metadata_schedule_134_0_e1313: f64 = (noise_metadata_schedule_134_0_e1308 + noise_metadata_schedule_134_0_e1312);
        let noise_metadata_schedule_134_0_e1315: f64 = (noise_metadata_schedule_134_0_e1313 / params.p70);
        let noise_metadata_schedule_134_0_e1316: f64 = (noise_metadata_schedule_134_0_e1315).exp();
        let noise_metadata_schedule_134_0_e1317: f64 = (params.p69 * noise_metadata_schedule_134_0_e1316);
        (noise_metadata_schedule_134_0_e1317,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_134_0_e1319;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_135_0_e1332,) = {
    if (w[138] != 0.0) {
        let noise_metadata_schedule_135_0_e1323: f64 = (params.p70 * w[70]);
        let noise_metadata_schedule_135_0_e1327: f64 = (params.p27 / w[74]);
        let noise_metadata_schedule_135_0_e1328: f64 = (1.0 + noise_metadata_schedule_135_0_e1327);
        let noise_metadata_schedule_135_0_e1329: f64 = (noise_metadata_schedule_135_0_e1328).ln();
        let noise_metadata_schedule_135_0_e1330: f64 = (noise_metadata_schedule_135_0_e1323 * noise_metadata_schedule_135_0_e1329);
        (noise_metadata_schedule_135_0_e1330,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_135_0_e1332;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_136_0_e1337,) = {
    if (w[138] == 0.0) {
        (0.0,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_136_0_e1337;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_137_0_e1342,) = {
    if (w[138] == 0.0) {
        (0.0,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_137_0_e1342;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_138_0_e1345: f64 = if params.p76 > 0.0 { 1.0 } else { 0.0 };
            w[139] = noise_metadata_schedule_138_0_e1345;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_139_0_e1366,) = {
    if (w[139] != 0.0) {
        let noise_metadata_schedule_139_0_e1349: f64 = (-params.p90);
        let noise_metadata_schedule_139_0_e1352: f64 = (1.0 - w[68]);
        let noise_metadata_schedule_139_0_e1353: f64 = (noise_metadata_schedule_139_0_e1349 * noise_metadata_schedule_139_0_e1352);
        let noise_metadata_schedule_139_0_e1355: f64 = (noise_metadata_schedule_139_0_e1353 / w[70]);
        let noise_metadata_schedule_139_0_e1358: f64 = (w[68]).ln();
        let noise_metadata_schedule_139_0_e1359: f64 = (params.p91 * noise_metadata_schedule_139_0_e1358);
        let noise_metadata_schedule_139_0_e1360: f64 = (noise_metadata_schedule_139_0_e1355 + noise_metadata_schedule_139_0_e1359);
        let noise_metadata_schedule_139_0_e1362: f64 = (noise_metadata_schedule_139_0_e1360 / params.p77);
        let noise_metadata_schedule_139_0_e1363: f64 = (noise_metadata_schedule_139_0_e1362).exp();
        let noise_metadata_schedule_139_0_e1364: f64 = (params.p76 * noise_metadata_schedule_139_0_e1363);
        (noise_metadata_schedule_139_0_e1364,)
    } else {
        (w[75],)
    }
};
            w[75] = noise_metadata_schedule_139_0_e1366;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_140_0_e1379,) = {
    if (w[139] != 0.0) {
        let noise_metadata_schedule_140_0_e1370: f64 = (params.p77 * w[70]);
        let noise_metadata_schedule_140_0_e1374: f64 = (params.p27 / w[75]);
        let noise_metadata_schedule_140_0_e1375: f64 = (1.0 + noise_metadata_schedule_140_0_e1374);
        let noise_metadata_schedule_140_0_e1376: f64 = (noise_metadata_schedule_140_0_e1375).ln();
        let noise_metadata_schedule_140_0_e1377: f64 = (noise_metadata_schedule_140_0_e1370 * noise_metadata_schedule_140_0_e1376);
        (noise_metadata_schedule_140_0_e1377,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_140_0_e1379;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_141_0_e1384,) = {
    if (w[139] == 0.0) {
        (0.0,)
    } else {
        (w[75],)
    }
};
            w[75] = noise_metadata_schedule_141_0_e1384;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_142_0_e1389,) = {
    if (w[139] == 0.0) {
        (0.0,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_142_0_e1389;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_143_0_e1392: f64 = (w[31] * w[74]);
            let noise_metadata_schedule_143_0_e1395: f64 = (w[32] * w[75]);
            let noise_metadata_schedule_143_0_e1396: f64 = (noise_metadata_schedule_143_0_e1392 + noise_metadata_schedule_143_0_e1395);
            w[84] = noise_metadata_schedule_143_0_e1396;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_144_0_e1399: f64 = (w[33] * w[74]);
            let noise_metadata_schedule_144_0_e1402: f64 = (w[34] * w[75]);
            let noise_metadata_schedule_144_0_e1403: f64 = (noise_metadata_schedule_144_0_e1399 + noise_metadata_schedule_144_0_e1402);
            w[85] = noise_metadata_schedule_144_0_e1403;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_159_0_e1595: f64 = (w[69] * params.p108);
            let noise_metadata_schedule_159_0_e1596: f64 = (1.0 + noise_metadata_schedule_159_0_e1595);
            let noise_metadata_schedule_159_0_e1598: f64 = (noise_metadata_schedule_159_0_e1596 * params.p86);
            w[80] = noise_metadata_schedule_159_0_e1598;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_160_0_e1604,) = {
    if (w[80] > 0.0) {
        (w[80],)
    } else {
        (0.0,)
    }
};
            w[80] = noise_metadata_schedule_160_0_e1604;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_161_0_e1607: f64 = if params.p83 > 0.0 { 1.0 } else { 0.0 };
            w[146] = noise_metadata_schedule_161_0_e1607;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_162_0_e1621,) = {
    if (w[146] != 0.0) {
        let noise_metadata_schedule_162_0_e1615: f64 = (w[69] * params.p106);
        let noise_metadata_schedule_162_0_e1616: f64 = (params.p105 + noise_metadata_schedule_162_0_e1615);
        let noise_metadata_schedule_162_0_e1617: f64 = (w[69] * noise_metadata_schedule_162_0_e1616);
        let noise_metadata_schedule_162_0_e1618: f64 = (1.0 + noise_metadata_schedule_162_0_e1617);
        let noise_metadata_schedule_162_0_e1619: f64 = (params.p83 * noise_metadata_schedule_162_0_e1618);
        (noise_metadata_schedule_162_0_e1619,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_162_0_e1621;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_163_0_e1630,) = {
    if (w[146] != 0.0) {
        let (noise_metadata_schedule_163_0_e1628,) = {
            if (w[103] > 0.0) {
                (w[103],)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_163_0_e1628,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_163_0_e1630;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_164_0_e1640,) = {
    if (w[146] != 0.0) {
        let noise_metadata_schedule_164_0_e1636: f64 = (params.p107 * w[69]);
        let noise_metadata_schedule_164_0_e1637: f64 = (1.0 + noise_metadata_schedule_164_0_e1636);
        let noise_metadata_schedule_164_0_e1638: f64 = (params.p85 * noise_metadata_schedule_164_0_e1637);
        (noise_metadata_schedule_164_0_e1638,)
    } else {
        (w[104],)
    }
};
            w[104] = noise_metadata_schedule_164_0_e1640;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_165_0_e1659,) = {
    if (w[146] != 0.0) {
        let noise_metadata_schedule_165_0_e1644: f64 = (w[104] * w[70]);
        let noise_metadata_schedule_165_0_e1646: f64 = (-w[103]);
        let noise_metadata_schedule_165_0_e1649: f64 = (w[104] * w[70]);
        let noise_metadata_schedule_165_0_e1650: f64 = (noise_metadata_schedule_165_0_e1646 / noise_metadata_schedule_165_0_e1649);
        let noise_metadata_schedule_165_0_e1651: f64 = (noise_metadata_schedule_165_0_e1650).exp();
        let noise_metadata_schedule_165_0_e1654: f64 = (params.p27 / params.p84);
        let noise_metadata_schedule_165_0_e1655: f64 = (noise_metadata_schedule_165_0_e1651 + noise_metadata_schedule_165_0_e1654);
        let noise_metadata_schedule_165_0_e1656: f64 = (noise_metadata_schedule_165_0_e1655).ln();
        let noise_metadata_schedule_165_0_e1657: f64 = (noise_metadata_schedule_165_0_e1644 * noise_metadata_schedule_165_0_e1656);
        (noise_metadata_schedule_165_0_e1657,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_165_0_e1659;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_166_0_e1664,) = {
    if (w[146] == 0.0) {
        (params.p83,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_166_0_e1664;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_167_0_e1669,) = {
    if (w[146] == 0.0) {
        (params.p85,)
    } else {
        (w[104],)
    }
};
            w[104] = noise_metadata_schedule_167_0_e1669;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_168_0_e1674,) = {
    if (w[146] == 0.0) {
        (1.0,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_168_0_e1674;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_169_0_e1680: f64 = if ((params.p60 > 0.0) && (params.p15 == 0.0)) { 1.0 } else { 0.0 };
            w[147] = noise_metadata_schedule_169_0_e1680;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_170_0_e1690,) = {
    if ((w[147] != 0.0) && (params.p62 != 0.0)) {
        let noise_metadata_schedule_170_0_e1686: f64 = (params.p61 * w[59]);
        let noise_metadata_schedule_170_0_e1688: f64 = (noise_metadata_schedule_170_0_e1686 * w[57]);
        (noise_metadata_schedule_170_0_e1688,)
    } else {
        (w[72],)
    }
};
            w[72] = noise_metadata_schedule_170_0_e1690;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_171_0_e1700,) = {
    if ((w[147] != 0.0) && (params.p62 != 0.0)) {
        let noise_metadata_schedule_171_0_e1696: f64 = (params.p60 * w[59]);
        let noise_metadata_schedule_171_0_e1698: f64 = (noise_metadata_schedule_171_0_e1696 * w[57]);
        (noise_metadata_schedule_171_0_e1698,)
    } else {
        (w[73],)
    }
};
            w[73] = noise_metadata_schedule_171_0_e1700;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_172_0_e1707,) = {
    if ((w[147] != 0.0) && (params.p62 == 0.0)) {
        (params.p61,)
    } else {
        (w[72],)
    }
};
            w[72] = noise_metadata_schedule_172_0_e1707;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_173_0_e1714,) = {
    if ((w[147] != 0.0) && (params.p62 == 0.0)) {
        (params.p60,)
    } else {
        (w[73],)
    }
};
            w[73] = noise_metadata_schedule_173_0_e1714;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_174_0_e1737,) = {
    if (w[147] != 0.0) {
        let noise_metadata_schedule_174_0_e1718: f64 = (w[72] * w[72]);
        let noise_metadata_schedule_174_0_e1721: f64 = (4.0 * params.p65);
        let noise_metadata_schedule_174_0_e1723: f64 = (noise_metadata_schedule_174_0_e1721 * params.p65);
        let noise_metadata_schedule_174_0_e1725: f64 = (noise_metadata_schedule_174_0_e1723 * w[73]);
        let noise_metadata_schedule_174_0_e1727: f64 = (noise_metadata_schedule_174_0_e1725 * w[73]);
        let noise_metadata_schedule_174_0_e1728: f64 = (noise_metadata_schedule_174_0_e1718 + noise_metadata_schedule_174_0_e1727);
        let noise_metadata_schedule_174_0_e1729: f64 = (noise_metadata_schedule_174_0_e1728).sqrt();
        let noise_metadata_schedule_174_0_e1732: f64 = (2.0 * params.p65);
        let noise_metadata_schedule_174_0_e1734: f64 = (noise_metadata_schedule_174_0_e1732 * w[73]);
        let noise_metadata_schedule_174_0_e1735: f64 = (noise_metadata_schedule_174_0_e1729 - noise_metadata_schedule_174_0_e1734);
        (noise_metadata_schedule_174_0_e1735,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_174_0_e1737;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_175_0_e1745,) = {
    if (w[147] != 0.0) {
        let noise_metadata_schedule_175_0_e1741: f64 = (params.p65 * w[19]);
        let noise_metadata_schedule_175_0_e1743: f64 = (noise_metadata_schedule_175_0_e1741 / w[73]);
        (noise_metadata_schedule_175_0_e1743,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_175_0_e1745;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_176_0_e1760,) = {
    if (w[147] != 0.0) {
        let noise_metadata_schedule_176_0_e1749: f64 = (w[19] * w[19]);
        let noise_metadata_schedule_176_0_e1752: f64 = (w[73] * w[73]);
        let noise_metadata_schedule_176_0_e1753: f64 = (noise_metadata_schedule_176_0_e1749 / noise_metadata_schedule_176_0_e1752);
        let noise_metadata_schedule_176_0_e1756: f64 = (4.0 * w[20]);
        let noise_metadata_schedule_176_0_e1757: f64 = (noise_metadata_schedule_176_0_e1753 + noise_metadata_schedule_176_0_e1756);
        let noise_metadata_schedule_176_0_e1758: f64 = (noise_metadata_schedule_176_0_e1757).sqrt();
        (noise_metadata_schedule_176_0_e1758,)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_176_0_e1760;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_177_0_e1766,) = {
    if (w[147] != 0.0) {
        let noise_metadata_schedule_177_0_e1764: f64 = (w[73] - w[72]);
        (noise_metadata_schedule_177_0_e1764,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_177_0_e1766;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_178_0_e1772,) = {
    if (w[147] != 0.0) {
        let noise_metadata_schedule_178_0_e1770: f64 = (1.0 / w[73]);
        (noise_metadata_schedule_178_0_e1770,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_178_0_e1772;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_179_0_e1777,) = {
    if (w[147] == 0.0) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_179_0_e1777;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_180_0_e1782,) = {
    if (w[147] == 0.0) {
        (0.0,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_180_0_e1782;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_181_0_e1787,) = {
    if (w[147] == 0.0) {
        (0.0,)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_181_0_e1787;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_182_0_e1792,) = {
    if (w[147] == 0.0) {
        (1000.0,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_182_0_e1792;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_183_0_e1797,) = {
    if (w[147] == 0.0) {
        (0.0,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_183_0_e1797;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_184_0_e1800: f64 = (w[28] * w[22]);
            w[51] = noise_metadata_schedule_184_0_e1800;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_185_0_e1803: f64 = if w[51] > 100000.0 { 1.0 } else { 0.0 };
            w[148] = noise_metadata_schedule_185_0_e1803;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_186_0_e1807,) = {
    if (w[148] != 0.0) {
        (100000.0,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_186_0_e1807;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_187_0_e1810: f64 = if w[64] < 0.0 { 1.0 } else { 0.0 };
            w[199] = noise_metadata_schedule_187_0_e1810;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_188_0_e1815,) = {
    if (w[199] != 0.0) {
        let noise_metadata_schedule_188_0_e1813: f64 = (-1.0);
        (noise_metadata_schedule_188_0_e1813,)
    } else {
        (w[149],)
    }
};
            w[149] = noise_metadata_schedule_188_0_e1815;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_189_0_e1820,) = {
    if (w[199] != 0.0) {
        let noise_metadata_schedule_189_0_e1818: f64 = (-w[66]);
        (noise_metadata_schedule_189_0_e1818,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_189_0_e1820;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_190_0_e1825,) = {
    if (w[199] != 0.0) {
        let noise_metadata_schedule_190_0_e1823: f64 = (-w[64]);
        (noise_metadata_schedule_190_0_e1823,)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_190_0_e1825;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_191_0_e1830,) = {
    if (w[199] == 0.0) {
        (1.0,)
    } else {
        (w[149],)
    }
};
            w[149] = noise_metadata_schedule_191_0_e1830;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_192_0_e1836,) = {
    if (w[199] == 0.0) {
        let noise_metadata_schedule_192_0_e1834: f64 = (-w[65]);
        (noise_metadata_schedule_192_0_e1834,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_192_0_e1836;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_193_0_e1841,) = {
    if (w[199] == 0.0) {
        (w[64],)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_193_0_e1841;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_194_0_e1844: f64 = if w[150] > w[49] { 1.0 } else { 0.0 };
            w[200] = noise_metadata_schedule_194_0_e1844;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_195_0_e1860,) = {
    if (w[200] != 0.0) {
        let noise_metadata_schedule_195_0_e1851: f64 = (w[49] - w[150]);
        let noise_metadata_schedule_195_0_e1853: f64 = (noise_metadata_schedule_195_0_e1851 / w[105]);
        let noise_metadata_schedule_195_0_e1854: f64 = (noise_metadata_schedule_195_0_e1853).exp();
        let noise_metadata_schedule_195_0_e1855: f64 = (1.0 + noise_metadata_schedule_195_0_e1854);
        let noise_metadata_schedule_195_0_e1856: f64 = (noise_metadata_schedule_195_0_e1855).ln();
        let noise_metadata_schedule_195_0_e1857: f64 = (w[105] * noise_metadata_schedule_195_0_e1856);
        let noise_metadata_schedule_195_0_e1858: f64 = (w[49] - noise_metadata_schedule_195_0_e1857);
        (noise_metadata_schedule_195_0_e1858,)
    } else {
        (w[152],)
    }
};
            w[152] = noise_metadata_schedule_195_0_e1860;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_196_0_e1877,) = {
    if (w[200] == 0.0) {
        let noise_metadata_schedule_196_0_e1868: f64 = (w[150] - w[49]);
        let noise_metadata_schedule_196_0_e1870: f64 = (noise_metadata_schedule_196_0_e1868 / w[105]);
        let noise_metadata_schedule_196_0_e1871: f64 = (noise_metadata_schedule_196_0_e1870).exp();
        let noise_metadata_schedule_196_0_e1872: f64 = (1.0 + noise_metadata_schedule_196_0_e1871);
        let noise_metadata_schedule_196_0_e1873: f64 = (noise_metadata_schedule_196_0_e1872).ln();
        let noise_metadata_schedule_196_0_e1874: f64 = (w[105] * noise_metadata_schedule_196_0_e1873);
        let noise_metadata_schedule_196_0_e1875: f64 = (w[150] - noise_metadata_schedule_196_0_e1874);
        (noise_metadata_schedule_196_0_e1875,)
    } else {
        (w[152],)
    }
};
            w[152] = noise_metadata_schedule_196_0_e1877;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_197_0_e1880: f64 = (-0.4);
            let noise_metadata_schedule_197_0_e1885: f64 = (w[49] - w[152]);
            let (noise_metadata_schedule_197_0_e1891,) = {
    if (w[151] < noise_metadata_schedule_197_0_e1885) {
        (w[151],)
    } else {
        let noise_metadata_schedule_197_0_e1890: f64 = (w[49] - w[152]);
        (noise_metadata_schedule_197_0_e1890,)
    }
};
            let noise_metadata_schedule_197_0_e1892: f64 = (w[41] + noise_metadata_schedule_197_0_e1891);
            let noise_metadata_schedule_197_0_e1893: f64 = (noise_metadata_schedule_197_0_e1880 * noise_metadata_schedule_197_0_e1892);
            let noise_metadata_schedule_197_0_e1894: f64 = if w[152] < noise_metadata_schedule_197_0_e1893 { 1.0 } else { 0.0 };
            w[201] = noise_metadata_schedule_197_0_e1894;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_198_0_e1914,) = {
    if ((params.p63 != 0.0) && (w[201] != 0.0)) {
        let noise_metadata_schedule_198_0_e1899: f64 = (-0.4);
        let noise_metadata_schedule_198_0_e1904: f64 = (w[49] - w[152]);
        let (noise_metadata_schedule_198_0_e1910,) = {
            if (w[151] < noise_metadata_schedule_198_0_e1904) {
                (w[151],)
            } else {
                let noise_metadata_schedule_198_0_e1909: f64 = (w[49] - w[152]);
                (noise_metadata_schedule_198_0_e1909,)
            }
        };
        let noise_metadata_schedule_198_0_e1911: f64 = (w[41] + noise_metadata_schedule_198_0_e1910);
        let noise_metadata_schedule_198_0_e1912: f64 = (noise_metadata_schedule_198_0_e1899 * noise_metadata_schedule_198_0_e1911);
        (noise_metadata_schedule_198_0_e1912,)
    } else {
        (w[153],)
    }
};
            w[153] = noise_metadata_schedule_198_0_e1914;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_199_0_e1921,) = {
    if ((params.p63 != 0.0) && (w[201] == 0.0)) {
        (w[152],)
    } else {
        (w[153],)
    }
};
            w[153] = noise_metadata_schedule_199_0_e1921;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_200_0_e1924: f64 = (-0.4);
            let noise_metadata_schedule_200_0_e1926: f64 = (noise_metadata_schedule_200_0_e1924 * w[41]);
            let noise_metadata_schedule_200_0_e1927: f64 = if w[152] < noise_metadata_schedule_200_0_e1926 { 1.0 } else { 0.0 };
            w[202] = noise_metadata_schedule_200_0_e1927;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_201_0_e1937,) = {
    if ((params.p63 == 0.0) && (w[202] != 0.0)) {
        let noise_metadata_schedule_201_0_e1933: f64 = (-0.4);
        let noise_metadata_schedule_201_0_e1935: f64 = (noise_metadata_schedule_201_0_e1933 * w[41]);
        (noise_metadata_schedule_201_0_e1935,)
    } else {
        (w[153],)
    }
};
            w[153] = noise_metadata_schedule_201_0_e1937;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_202_0_e1945,) = {
    if ((params.p63 == 0.0) && (w[202] == 0.0)) {
        (w[152],)
    } else {
        (w[153],)
    }
};
            w[153] = noise_metadata_schedule_202_0_e1945;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_203_0_e1949: f64 = (2.0 * w[153]);
            let noise_metadata_schedule_203_0_e1950: f64 = (w[41] + noise_metadata_schedule_203_0_e1949);
            w[154] = noise_metadata_schedule_203_0_e1950;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_204_0_e1953: f64 = if w[18] > 0.0 { 1.0 } else { 0.0 };
            w[203] = noise_metadata_schedule_204_0_e1953;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_205_0_e1963,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_205_0_e1957: f64 = (w[46] * w[154]);
        let noise_metadata_schedule_205_0_e1959: f64 = (noise_metadata_schedule_205_0_e1957 * w[154]);
        let noise_metadata_schedule_205_0_e1961: f64 = (noise_metadata_schedule_205_0_e1959 - w[154]);
        (noise_metadata_schedule_205_0_e1961,)
    } else {
        (w[155],)
    }
};
            w[155] = noise_metadata_schedule_205_0_e1963;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_206_0_e1974,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_206_0_e1966: f64 = (-1.0);
        let noise_metadata_schedule_206_0_e1969: f64 = (3.0 * w[46]);
        let noise_metadata_schedule_206_0_e1971: f64 = (noise_metadata_schedule_206_0_e1969 * w[154]);
        let noise_metadata_schedule_206_0_e1972: f64 = (noise_metadata_schedule_206_0_e1966 + noise_metadata_schedule_206_0_e1971);
        (noise_metadata_schedule_206_0_e1972,)
    } else {
        (params.p3,)
    }
};
            w[156] = noise_metadata_schedule_206_0_e1974;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_207_0_e1986,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_207_0_e1979: f64 = (9.0 / 4.0);
        let noise_metadata_schedule_207_0_e1982: f64 = (w[154] / w[51]);
        let noise_metadata_schedule_207_0_e1983: f64 = (noise_metadata_schedule_207_0_e1979 + noise_metadata_schedule_207_0_e1982);
        let noise_metadata_schedule_207_0_e1984: f64 = (w[46] * noise_metadata_schedule_207_0_e1983);
        (noise_metadata_schedule_207_0_e1984,)
    } else {
        (params.p6,)
    }
};
            w[157] = noise_metadata_schedule_207_0_e1986;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_208_0_e1994,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_208_0_e1990: f64 = (1.5 * w[46]);
        let noise_metadata_schedule_208_0_e1992: f64 = (noise_metadata_schedule_208_0_e1990 / w[51]);
        (noise_metadata_schedule_208_0_e1992,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_208_0_e1994;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_209_0_e2004,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_209_0_e1998: f64 = (4.0 * w[51]);
        let noise_metadata_schedule_209_0_e2000: f64 = (noise_metadata_schedule_209_0_e1998 * w[51]);
        let noise_metadata_schedule_209_0_e2002: f64 = (noise_metadata_schedule_209_0_e2000 / w[46]);
        (noise_metadata_schedule_209_0_e2002,)
    } else {
        (w[159],)
    }
};
            w[159] = noise_metadata_schedule_209_0_e2004;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_210_0_e2010,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_210_0_e2008: f64 = (w[155] * w[159]);
        (noise_metadata_schedule_210_0_e2008,)
    } else {
        (w[160],)
    }
};
            w[160] = noise_metadata_schedule_210_0_e2010;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_211_0_e2016,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_211_0_e2014: f64 = (params.p3 * w[159]);
        (noise_metadata_schedule_211_0_e2014,)
    } else {
        (w[161],)
    }
};
            w[161] = noise_metadata_schedule_211_0_e2016;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_212_0_e2022,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_212_0_e2020: f64 = (params.p6 * w[159]);
        (noise_metadata_schedule_212_0_e2020,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_212_0_e2022;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_213_0_e2028,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_213_0_e2026: f64 = (w[158] * w[159]);
        (noise_metadata_schedule_213_0_e2026,)
    } else {
        (w[163],)
    }
};
            w[163] = noise_metadata_schedule_213_0_e2028;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_214_0_e2034,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_214_0_e2032: f64 = (w[163] * w[163]);
        (noise_metadata_schedule_214_0_e2032,)
    } else {
        (w[164],)
    }
};
            w[164] = noise_metadata_schedule_214_0_e2034;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_215_0_e2039,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_215_0_e2037: f64 = (-w[162]);
        (noise_metadata_schedule_215_0_e2037,)
    } else {
        (w[165],)
    }
};
            w[165] = noise_metadata_schedule_215_0_e2039;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_216_0_e2049,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_216_0_e2043: f64 = (w[163] * w[161]);
        let noise_metadata_schedule_216_0_e2046: f64 = (4.0 * w[160]);
        let noise_metadata_schedule_216_0_e2047: f64 = (noise_metadata_schedule_216_0_e2043 - noise_metadata_schedule_216_0_e2046);
        (noise_metadata_schedule_216_0_e2047,)
    } else {
        (w[166],)
    }
};
            w[166] = noise_metadata_schedule_216_0_e2049;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_217_0_e2065,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_217_0_e2053: f64 = (4.0 * w[162]);
        let noise_metadata_schedule_217_0_e2055: f64 = (noise_metadata_schedule_217_0_e2053 * w[160]);
        let noise_metadata_schedule_217_0_e2058: f64 = (w[161] * w[161]);
        let noise_metadata_schedule_217_0_e2059: f64 = (noise_metadata_schedule_217_0_e2055 - noise_metadata_schedule_217_0_e2058);
        let noise_metadata_schedule_217_0_e2062: f64 = (w[160] * w[164]);
        let noise_metadata_schedule_217_0_e2063: f64 = (noise_metadata_schedule_217_0_e2059 - noise_metadata_schedule_217_0_e2062);
        (noise_metadata_schedule_217_0_e2063,)
    } else {
        (w[167],)
    }
};
            w[167] = noise_metadata_schedule_217_0_e2065;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_218_0_e2075,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_218_0_e2070: f64 = (w[165] * w[165]);
        let noise_metadata_schedule_218_0_e2072: f64 = (noise_metadata_schedule_218_0_e2070 * 0.3333333333333333);
        let noise_metadata_schedule_218_0_e2073: f64 = (w[166] - noise_metadata_schedule_218_0_e2072);
        (noise_metadata_schedule_218_0_e2073,)
    } else {
        (w[168],)
    }
};
            w[168] = noise_metadata_schedule_218_0_e2075;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_219_0_e2089,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_219_0_e2082: f64 = (2.0 * w[168]);
        let noise_metadata_schedule_219_0_e2083: f64 = (w[166] + noise_metadata_schedule_219_0_e2082);
        let noise_metadata_schedule_219_0_e2084: f64 = (w[165] * noise_metadata_schedule_219_0_e2083);
        let noise_metadata_schedule_219_0_e2086: f64 = (noise_metadata_schedule_219_0_e2084 / 9.0);
        let noise_metadata_schedule_219_0_e2087: f64 = (w[167] - noise_metadata_schedule_219_0_e2086);
        (noise_metadata_schedule_219_0_e2087,)
    } else {
        (w[169],)
    }
};
            w[169] = noise_metadata_schedule_219_0_e2089;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_220_0_e2099,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_220_0_e2093: f64 = (w[168] * w[168]);
        let noise_metadata_schedule_220_0_e2095: f64 = (noise_metadata_schedule_220_0_e2093 * w[168]);
        let noise_metadata_schedule_220_0_e2097: f64 = (noise_metadata_schedule_220_0_e2095 / 27.0);
        (noise_metadata_schedule_220_0_e2097,)
    } else {
        (w[170],)
    }
};
            w[170] = noise_metadata_schedule_220_0_e2099;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_221_0_e2109,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_221_0_e2103: f64 = (0.25 * w[169]);
        let noise_metadata_schedule_221_0_e2105: f64 = (noise_metadata_schedule_221_0_e2103 * w[169]);
        let noise_metadata_schedule_221_0_e2107: f64 = (noise_metadata_schedule_221_0_e2105 + w[170]);
        (noise_metadata_schedule_221_0_e2107,)
    } else {
        (w[171],)
    }
};
            w[171] = noise_metadata_schedule_221_0_e2109;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_222_0_e2114,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_222_0_e2112: f64 = (w[171]).sqrt();
        (noise_metadata_schedule_222_0_e2112,)
    } else {
        (w[172],)
    }
};
            w[172] = noise_metadata_schedule_222_0_e2114;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_223_0_e2117: f64 = if w[169] < 0.0 { 1.0 } else { 0.0 };
            w[204] = noise_metadata_schedule_223_0_e2117;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_224_0_e2128,) = {
    if ((w[203] != 0.0) && (w[204] != 0.0)) {
        let noise_metadata_schedule_224_0_e2122: f64 = (-0.5);
        let noise_metadata_schedule_224_0_e2124: f64 = (noise_metadata_schedule_224_0_e2122 * w[169]);
        let noise_metadata_schedule_224_0_e2126: f64 = (noise_metadata_schedule_224_0_e2124 + w[172]);
        (noise_metadata_schedule_224_0_e2126,)
    } else {
        (w[173],)
    }
};
            w[173] = noise_metadata_schedule_224_0_e2128;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_225_0_e2137,) = {
    if ((w[203] != 0.0) && (w[204] != 0.0)) {
        let noise_metadata_schedule_225_0_e2133: f64 = (-w[170]);
        let noise_metadata_schedule_225_0_e2135: f64 = (noise_metadata_schedule_225_0_e2133 / w[173]);
        (noise_metadata_schedule_225_0_e2135,)
    } else {
        (w[174],)
    }
};
            w[174] = noise_metadata_schedule_225_0_e2137;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_226_0_e2149,) = {
    if ((w[203] != 0.0) && (w[204] == 0.0)) {
        let noise_metadata_schedule_226_0_e2143: f64 = (-0.5);
        let noise_metadata_schedule_226_0_e2145: f64 = (noise_metadata_schedule_226_0_e2143 * w[169]);
        let noise_metadata_schedule_226_0_e2147: f64 = (noise_metadata_schedule_226_0_e2145 - w[172]);
        (noise_metadata_schedule_226_0_e2147,)
    } else {
        (w[174],)
    }
};
            w[174] = noise_metadata_schedule_226_0_e2149;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_227_0_e2159,) = {
    if ((w[203] != 0.0) && (w[204] == 0.0)) {
        let noise_metadata_schedule_227_0_e2155: f64 = (-w[170]);
        let noise_metadata_schedule_227_0_e2157: f64 = (noise_metadata_schedule_227_0_e2155 / w[174]);
        (noise_metadata_schedule_227_0_e2157,)
    } else {
        (w[173],)
    }
};
            w[173] = noise_metadata_schedule_227_0_e2159;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_228_0_e2162: f64 = if w[173] > 1e-6 { 1.0 } else { 0.0 };
            w[205] = noise_metadata_schedule_228_0_e2162;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_229_0_e2170,) = {
    if ((w[203] != 0.0) && (w[205] != 0.0)) {
        let noise_metadata_schedule_229_0_e2168: f64 = (w[173]).powf(0.3333333333333333);
        (noise_metadata_schedule_229_0_e2168,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_229_0_e2170;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_230_0_e2173: f64 = (-1e-6);
            let noise_metadata_schedule_230_0_e2174: f64 = if w[173] < noise_metadata_schedule_230_0_e2173 { 1.0 } else { 0.0 };
            w[206] = noise_metadata_schedule_230_0_e2174;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_231_0_e2187,) = {
    if (((w[203] != 0.0) && (w[205] == 0.0)) && (w[206] != 0.0)) {
        let noise_metadata_schedule_231_0_e2182: f64 = (-w[173]);
        let noise_metadata_schedule_231_0_e2184: f64 = (noise_metadata_schedule_231_0_e2182).powf(0.3333333333333333);
        let noise_metadata_schedule_231_0_e2185: f64 = (-noise_metadata_schedule_231_0_e2184);
        (noise_metadata_schedule_231_0_e2185,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_231_0_e2187;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_232_0_e2199,) = {
    if (((w[203] != 0.0) && (w[205] == 0.0)) && (w[206] == 0.0)) {
        let noise_metadata_schedule_232_0_e2197: f64 = (10000.0 * w[173]);
        (noise_metadata_schedule_232_0_e2197,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_232_0_e2199;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_233_0_e2202: f64 = if w[174] > 1e-6 { 1.0 } else { 0.0 };
            w[207] = noise_metadata_schedule_233_0_e2202;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_234_0_e2210,) = {
    if ((w[203] != 0.0) && (w[207] != 0.0)) {
        let noise_metadata_schedule_234_0_e2208: f64 = (w[174]).powf(0.3333333333333333);
        (noise_metadata_schedule_234_0_e2208,)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_234_0_e2210;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_235_0_e2213: f64 = (-1e-6);
            let noise_metadata_schedule_235_0_e2214: f64 = if w[174] < noise_metadata_schedule_235_0_e2213 { 1.0 } else { 0.0 };
            w[208] = noise_metadata_schedule_235_0_e2214;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_236_0_e2227,) = {
    if (((w[203] != 0.0) && (w[207] == 0.0)) && (w[208] != 0.0)) {
        let noise_metadata_schedule_236_0_e2222: f64 = (-w[174]);
        let noise_metadata_schedule_236_0_e2224: f64 = (noise_metadata_schedule_236_0_e2222).powf(0.3333333333333333);
        let noise_metadata_schedule_236_0_e2225: f64 = (-noise_metadata_schedule_236_0_e2224);
        (noise_metadata_schedule_236_0_e2225,)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_236_0_e2227;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_237_0_e2239,) = {
    if (((w[203] != 0.0) && (w[207] == 0.0)) && (w[208] == 0.0)) {
        let noise_metadata_schedule_237_0_e2237: f64 = (10000.0 * w[174]);
        (noise_metadata_schedule_237_0_e2237,)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_237_0_e2239;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_238_0_e2249,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_238_0_e2243: f64 = (w[175] + w[176]);
        let noise_metadata_schedule_238_0_e2246: f64 = (w[165] * 0.3333333333333333);
        let noise_metadata_schedule_238_0_e2247: f64 = (noise_metadata_schedule_238_0_e2243 - noise_metadata_schedule_238_0_e2246);
        (noise_metadata_schedule_238_0_e2247,)
    } else {
        (w[177],)
    }
};
            w[177] = noise_metadata_schedule_238_0_e2249;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_239_0_e2260,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_239_0_e2253: f64 = (0.25 * w[164]);
        let noise_metadata_schedule_239_0_e2255: f64 = (noise_metadata_schedule_239_0_e2253 - w[162]);
        let noise_metadata_schedule_239_0_e2257: f64 = (noise_metadata_schedule_239_0_e2255 + w[177]);
        let noise_metadata_schedule_239_0_e2258: f64 = (noise_metadata_schedule_239_0_e2257).sqrt();
        (noise_metadata_schedule_239_0_e2258,)
    } else {
        (w[167],)
    }
};
            w[167] = noise_metadata_schedule_239_0_e2260;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_240_0_e2274,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_240_0_e2264: f64 = (0.75 * w[164]);
        let noise_metadata_schedule_240_0_e2267: f64 = (w[167] * w[167]);
        let noise_metadata_schedule_240_0_e2268: f64 = (noise_metadata_schedule_240_0_e2264 - noise_metadata_schedule_240_0_e2267);
        let noise_metadata_schedule_240_0_e2271: f64 = (2.0 * w[162]);
        let noise_metadata_schedule_240_0_e2272: f64 = (noise_metadata_schedule_240_0_e2268 - noise_metadata_schedule_240_0_e2271);
        (noise_metadata_schedule_240_0_e2272,)
    } else {
        (w[178],)
    }
};
            w[178] = noise_metadata_schedule_240_0_e2274;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_241_0_e2292,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_241_0_e2278: f64 = (w[163] * w[162]);
        let noise_metadata_schedule_241_0_e2281: f64 = (2.0 * w[161]);
        let noise_metadata_schedule_241_0_e2282: f64 = (noise_metadata_schedule_241_0_e2278 - noise_metadata_schedule_241_0_e2281);
        let noise_metadata_schedule_241_0_e2285: f64 = (0.25 * w[164]);
        let noise_metadata_schedule_241_0_e2287: f64 = (noise_metadata_schedule_241_0_e2285 * w[163]);
        let noise_metadata_schedule_241_0_e2288: f64 = (noise_metadata_schedule_241_0_e2282 - noise_metadata_schedule_241_0_e2287);
        let noise_metadata_schedule_241_0_e2290: f64 = (noise_metadata_schedule_241_0_e2288 / w[167]);
        (noise_metadata_schedule_241_0_e2290,)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_241_0_e2292;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_242_0_e2298,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_242_0_e2296: f64 = (w[178] + w[179]);
        (noise_metadata_schedule_242_0_e2296,)
    } else {
        (w[180],)
    }
};
            w[180] = noise_metadata_schedule_242_0_e2298;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_243_0_e2301: f64 = if w[180] > 0.0 { 1.0 } else { 0.0 };
            w[209] = noise_metadata_schedule_243_0_e2301;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_244_0_e2308,) = {
    if ((w[203] != 0.0) && (w[209] != 0.0)) {
        let noise_metadata_schedule_244_0_e2306: f64 = (w[180]).sqrt();
        (noise_metadata_schedule_244_0_e2306,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_244_0_e2308;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_245_0_e2323,) = {
    if ((w[203] != 0.0) && (w[209] != 0.0)) {
        let noise_metadata_schedule_245_0_e2313: f64 = (-0.25);
        let noise_metadata_schedule_245_0_e2315: f64 = (noise_metadata_schedule_245_0_e2313 * w[163]);
        let noise_metadata_schedule_245_0_e2319: f64 = (w[182] + w[167]);
        let noise_metadata_schedule_245_0_e2320: f64 = (0.5 * noise_metadata_schedule_245_0_e2319);
        let noise_metadata_schedule_245_0_e2321: f64 = (noise_metadata_schedule_245_0_e2315 + noise_metadata_schedule_245_0_e2320);
        (noise_metadata_schedule_245_0_e2321,)
    } else {
        (w[183],)
    }
};
            w[183] = noise_metadata_schedule_245_0_e2323;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_246_0_e2332,) = {
    if ((w[203] != 0.0) && (w[209] == 0.0)) {
        let noise_metadata_schedule_246_0_e2330: f64 = (w[178] - w[179]);
        (noise_metadata_schedule_246_0_e2330,)
    } else {
        (w[181],)
    }
};
            w[181] = noise_metadata_schedule_246_0_e2332;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_247_0_e2345,) = {
    if ((w[203] != 0.0) && (w[209] == 0.0)) {
        let noise_metadata_schedule_247_0_e2339: f64 = (w[181] * w[181]);
        let noise_metadata_schedule_247_0_e2341: f64 = (noise_metadata_schedule_247_0_e2339 + 0.0001);
        let noise_metadata_schedule_247_0_e2342: f64 = (noise_metadata_schedule_247_0_e2341).sqrt();
        let noise_metadata_schedule_247_0_e2343: f64 = (noise_metadata_schedule_247_0_e2342).sqrt();
        (noise_metadata_schedule_247_0_e2343,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_247_0_e2345;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_248_0_e2361,) = {
    if ((w[203] != 0.0) && (w[209] == 0.0)) {
        let noise_metadata_schedule_248_0_e2351: f64 = (-0.25);
        let noise_metadata_schedule_248_0_e2353: f64 = (noise_metadata_schedule_248_0_e2351 * w[163]);
        let noise_metadata_schedule_248_0_e2357: f64 = (w[182] - w[167]);
        let noise_metadata_schedule_248_0_e2358: f64 = (0.5 * noise_metadata_schedule_248_0_e2357);
        let noise_metadata_schedule_248_0_e2359: f64 = (noise_metadata_schedule_248_0_e2353 + noise_metadata_schedule_248_0_e2358);
        (noise_metadata_schedule_248_0_e2359,)
    } else {
        (w[183],)
    }
};
            w[183] = noise_metadata_schedule_248_0_e2361;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_249_0_e2364: f64 = if w[153] > w[50] { 1.0 } else { 0.0 };
            w[210] = noise_metadata_schedule_249_0_e2364;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_250_0_e2375,) = {
    if ((w[203] == 0.0) && (w[210] != 0.0)) {
        let noise_metadata_schedule_250_0_e2372: f64 = (w[48] - w[153]);
        let noise_metadata_schedule_250_0_e2373: f64 = (w[46] * noise_metadata_schedule_250_0_e2372);
        (noise_metadata_schedule_250_0_e2373,)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_250_0_e2375;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_251_0_e2405,) = {
    if ((w[203] == 0.0) && (w[210] != 0.0)) {
        let noise_metadata_schedule_251_0_e2384: f64 = (2.0 * w[198]);
        let noise_metadata_schedule_251_0_e2385: f64 = (1.0 - noise_metadata_schedule_251_0_e2384);
        let noise_metadata_schedule_251_0_e2386: f64 = (2.0 * noise_metadata_schedule_251_0_e2385);
        let noise_metadata_schedule_251_0_e2389: f64 = (w[48] - w[153]);
        let noise_metadata_schedule_251_0_e2390: f64 = (noise_metadata_schedule_251_0_e2386 * noise_metadata_schedule_251_0_e2389);
        let noise_metadata_schedule_251_0_e2394: f64 = (3.0 * w[198]);
        let noise_metadata_schedule_251_0_e2395: f64 = (1.0 - noise_metadata_schedule_251_0_e2394);
        let noise_metadata_schedule_251_0_e2399: f64 = (1.5 * w[198]);
        let noise_metadata_schedule_251_0_e2400: f64 = (1.0 - noise_metadata_schedule_251_0_e2399);
        let noise_metadata_schedule_251_0_e2401: f64 = (noise_metadata_schedule_251_0_e2400).sqrt();
        let noise_metadata_schedule_251_0_e2402: f64 = (noise_metadata_schedule_251_0_e2395 + noise_metadata_schedule_251_0_e2401);
        let noise_metadata_schedule_251_0_e2403: f64 = (noise_metadata_schedule_251_0_e2390 / noise_metadata_schedule_251_0_e2402);
        (noise_metadata_schedule_251_0_e2403,)
    } else {
        (w[183],)
    }
};
            w[183] = noise_metadata_schedule_251_0_e2405;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_252_0_e2417,) = {
    if ((w[203] == 0.0) && (w[210] == 0.0)) {
        let noise_metadata_schedule_252_0_e2413: f64 = (3.0 * w[46]);
        let noise_metadata_schedule_252_0_e2415: f64 = (noise_metadata_schedule_252_0_e2413 * w[154]);
        (noise_metadata_schedule_252_0_e2415,)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_252_0_e2417;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_253_0_e2436,) = {
    if ((w[203] == 0.0) && (w[210] == 0.0)) {
        let noise_metadata_schedule_253_0_e2425: f64 = (1.0 - w[198]);
        let noise_metadata_schedule_253_0_e2428: f64 = (1.0 + w[198]);
        let noise_metadata_schedule_253_0_e2429: f64 = (noise_metadata_schedule_253_0_e2428).sqrt();
        let noise_metadata_schedule_253_0_e2430: f64 = (noise_metadata_schedule_253_0_e2425 + noise_metadata_schedule_253_0_e2429);
        let noise_metadata_schedule_253_0_e2433: f64 = (4.5 * w[46]);
        let noise_metadata_schedule_253_0_e2434: f64 = (noise_metadata_schedule_253_0_e2430 / noise_metadata_schedule_253_0_e2433);
        (noise_metadata_schedule_253_0_e2434,)
    } else {
        (w[183],)
    }
};
            w[183] = noise_metadata_schedule_253_0_e2436;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_254_0_e2443: f64 = if ((params.p63 > 1.0) && (w[45] > 1e-9)) { 1.0 } else { 0.0 };
            w[211] = noise_metadata_schedule_254_0_e2443;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_255_0_e2449,) = {
    if (w[211] != 0.0) {
        let noise_metadata_schedule_255_0_e2447: f64 = (w[183] + w[71]);
        (noise_metadata_schedule_255_0_e2447,)
    } else {
        (w[193],)
    }
};
            w[193] = noise_metadata_schedule_255_0_e2449;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_256_0_e2458,) = {
    if (w[211] != 0.0) {
        let noise_metadata_schedule_256_0_e2454: f64 = (w[154] + w[183]);
        let noise_metadata_schedule_256_0_e2455: f64 = (noise_metadata_schedule_256_0_e2454).sqrt();
        let noise_metadata_schedule_256_0_e2456: f64 = (w[45] * noise_metadata_schedule_256_0_e2455);
        (noise_metadata_schedule_256_0_e2456,)
    } else {
        (w[194],)
    }
};
            w[194] = noise_metadata_schedule_256_0_e2458;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_257_0_e2461: f64 = if w[18] > 0.0 { 1.0 } else { 0.0 };
            w[212] = noise_metadata_schedule_257_0_e2461;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_258_0_e2475,) = {
    if ((w[211] != 0.0) && (w[212] != 0.0)) {
        let noise_metadata_schedule_258_0_e2468: f64 = (w[193] / w[28]);
        let noise_metadata_schedule_258_0_e2470: f64 = (noise_metadata_schedule_258_0_e2468 - w[19]);
        let noise_metadata_schedule_258_0_e2471: f64 = (0.5 * noise_metadata_schedule_258_0_e2470);
        let noise_metadata_schedule_258_0_e2473: f64 = (noise_metadata_schedule_258_0_e2471 * w[18]);
        (noise_metadata_schedule_258_0_e2473,)
    } else {
        (w[185],)
    }
};
            w[185] = noise_metadata_schedule_258_0_e2475;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_259_0_e2489,) = {
    if ((w[211] != 0.0) && (w[212] != 0.0)) {
        let noise_metadata_schedule_259_0_e2482: f64 = (w[193] / w[28]);
        let noise_metadata_schedule_259_0_e2484: f64 = (noise_metadata_schedule_259_0_e2482 + w[19]);
        let noise_metadata_schedule_259_0_e2485: f64 = (0.5 * noise_metadata_schedule_259_0_e2484);
        let noise_metadata_schedule_259_0_e2487: f64 = (noise_metadata_schedule_259_0_e2485 * w[18]);
        (noise_metadata_schedule_259_0_e2487,)
    } else {
        (w[186],)
    }
};
            w[186] = noise_metadata_schedule_259_0_e2489;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_260_0_e2500,) = {
    if ((w[211] != 0.0) && (w[212] != 0.0)) {
        let noise_metadata_schedule_260_0_e2495: f64 = (w[185] * w[185]);
        let noise_metadata_schedule_260_0_e2497: f64 = (noise_metadata_schedule_260_0_e2495 + w[20]);
        let noise_metadata_schedule_260_0_e2498: f64 = (noise_metadata_schedule_260_0_e2497).sqrt();
        (noise_metadata_schedule_260_0_e2498,)
    } else {
        (w[188],)
    }
};
            w[188] = noise_metadata_schedule_260_0_e2500;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_261_0_e2511,) = {
    if ((w[211] != 0.0) && (w[212] != 0.0)) {
        let noise_metadata_schedule_261_0_e2506: f64 = (w[186] * w[186]);
        let noise_metadata_schedule_261_0_e2508: f64 = (noise_metadata_schedule_261_0_e2506 + w[20]);
        let noise_metadata_schedule_261_0_e2509: f64 = (noise_metadata_schedule_261_0_e2508).sqrt();
        (noise_metadata_schedule_261_0_e2509,)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_261_0_e2511;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_262_0_e2521,) = {
    if ((w[211] != 0.0) && (w[212] != 0.0)) {
        let noise_metadata_schedule_262_0_e2517: f64 = (w[188] + w[187]);
        let noise_metadata_schedule_262_0_e2519: f64 = (noise_metadata_schedule_262_0_e2517 - w[21]);
        (noise_metadata_schedule_262_0_e2519,)
    } else {
        (w[189],)
    }
};
            w[189] = noise_metadata_schedule_262_0_e2521;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_263_0_e2539,) = {
    if ((w[211] != 0.0) && (w[212] != 0.0)) {
        let noise_metadata_schedule_263_0_e2528: f64 = (w[185] / w[188]);
        let noise_metadata_schedule_263_0_e2531: f64 = (w[186] / w[187]);
        let noise_metadata_schedule_263_0_e2532: f64 = (noise_metadata_schedule_263_0_e2528 + noise_metadata_schedule_263_0_e2531);
        let noise_metadata_schedule_263_0_e2533: f64 = (0.5 * noise_metadata_schedule_263_0_e2532);
        let noise_metadata_schedule_263_0_e2535: f64 = (noise_metadata_schedule_263_0_e2533 * w[18]);
        let noise_metadata_schedule_263_0_e2537: f64 = (noise_metadata_schedule_263_0_e2535 / w[28]);
        (noise_metadata_schedule_263_0_e2537,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_263_0_e2539;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_264_0_e2564,) = {
    if ((w[211] != 0.0) && (w[212] != 0.0)) {
        let noise_metadata_schedule_264_0_e2545: f64 = (2.0 * w[194]);
        let noise_metadata_schedule_264_0_e2548: f64 = (1.0 - w[194]);
        let noise_metadata_schedule_264_0_e2549: f64 = (noise_metadata_schedule_264_0_e2545 * noise_metadata_schedule_264_0_e2548);
        let noise_metadata_schedule_264_0_e2553: f64 = (w[195] * w[193]);
        let noise_metadata_schedule_264_0_e2556: f64 = (1.0 + w[189]);
        let noise_metadata_schedule_264_0_e2557: f64 = (noise_metadata_schedule_264_0_e2553 / noise_metadata_schedule_264_0_e2556);
        let noise_metadata_schedule_264_0_e2558: f64 = (1.0 - noise_metadata_schedule_264_0_e2557);
        let noise_metadata_schedule_264_0_e2559: f64 = (noise_metadata_schedule_264_0_e2549 * noise_metadata_schedule_264_0_e2558);
        let noise_metadata_schedule_264_0_e2561: f64 = (noise_metadata_schedule_264_0_e2559 / w[193]);
        let noise_metadata_schedule_264_0_e2562: f64 = (noise_metadata_schedule_264_0_e2561).sqrt();
        (noise_metadata_schedule_264_0_e2562,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_264_0_e2564;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_265_0_e2580,) = {
    if ((w[211] != 0.0) && (w[212] == 0.0)) {
        let noise_metadata_schedule_265_0_e2571: f64 = (2.0 * w[194]);
        let noise_metadata_schedule_265_0_e2574: f64 = (1.0 - w[194]);
        let noise_metadata_schedule_265_0_e2575: f64 = (noise_metadata_schedule_265_0_e2571 * noise_metadata_schedule_265_0_e2574);
        let noise_metadata_schedule_265_0_e2577: f64 = (noise_metadata_schedule_265_0_e2575 / w[193]);
        let noise_metadata_schedule_265_0_e2578: f64 = (noise_metadata_schedule_265_0_e2577).sqrt();
        (noise_metadata_schedule_265_0_e2578,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_265_0_e2580;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_266_0_e2594,) = {
    if (w[211] != 0.0) {
        let noise_metadata_schedule_266_0_e2585: f64 = (w[154] + w[183]);
        let noise_metadata_schedule_266_0_e2586: f64 = (w[46] * noise_metadata_schedule_266_0_e2585);
        let noise_metadata_schedule_266_0_e2589: f64 = (w[196] * w[196]);
        let noise_metadata_schedule_266_0_e2590: f64 = (noise_metadata_schedule_266_0_e2586 / noise_metadata_schedule_266_0_e2589);
        let noise_metadata_schedule_266_0_e2592: f64 = (noise_metadata_schedule_266_0_e2590 - w[193]);
        (noise_metadata_schedule_266_0_e2592,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_266_0_e2594;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_267_0_e2606,) = {
    if (w[211] != 0.0) {
        let noise_metadata_schedule_267_0_e2599: f64 = (params.p47 * w[183]);
        let noise_metadata_schedule_267_0_e2602: f64 = (params.p47 + w[193]);
        let noise_metadata_schedule_267_0_e2603: f64 = (noise_metadata_schedule_267_0_e2599 / noise_metadata_schedule_267_0_e2602);
        let noise_metadata_schedule_267_0_e2604: f64 = (w[107] + noise_metadata_schedule_267_0_e2603);
        (noise_metadata_schedule_267_0_e2604,)
    } else {
        (w[191],)
    }
};
            w[191] = noise_metadata_schedule_267_0_e2606;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_268_0_e2614,) = {
    if (w[211] != 0.0) {
        let noise_metadata_schedule_268_0_e2610: f64 = (4.0 * w[191]);
        let noise_metadata_schedule_268_0_e2612: f64 = (noise_metadata_schedule_268_0_e2610 * w[191]);
        (noise_metadata_schedule_268_0_e2612,)
    } else {
        (w[192],)
    }
};
            w[192] = noise_metadata_schedule_268_0_e2614;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_269_0_e2644,) = {
    if (w[211] != 0.0) {
        let noise_metadata_schedule_269_0_e2618: f64 = (2.0 * w[151]);
        let noise_metadata_schedule_269_0_e2620: f64 = (noise_metadata_schedule_269_0_e2618 * w[193]);
        let noise_metadata_schedule_269_0_e2623: f64 = (w[151] - w[193]);
        let noise_metadata_schedule_269_0_e2626: f64 = (w[151] - w[193]);
        let noise_metadata_schedule_269_0_e2627: f64 = (noise_metadata_schedule_269_0_e2623 * noise_metadata_schedule_269_0_e2626);
        let noise_metadata_schedule_269_0_e2629: f64 = (noise_metadata_schedule_269_0_e2627 + w[192]);
        let noise_metadata_schedule_269_0_e2630: f64 = (noise_metadata_schedule_269_0_e2629).sqrt();
        let noise_metadata_schedule_269_0_e2633: f64 = (w[151] + w[193]);
        let noise_metadata_schedule_269_0_e2636: f64 = (w[151] + w[193]);
        let noise_metadata_schedule_269_0_e2637: f64 = (noise_metadata_schedule_269_0_e2633 * noise_metadata_schedule_269_0_e2636);
        let noise_metadata_schedule_269_0_e2639: f64 = (noise_metadata_schedule_269_0_e2637 + w[192]);
        let noise_metadata_schedule_269_0_e2640: f64 = (noise_metadata_schedule_269_0_e2639).sqrt();
        let noise_metadata_schedule_269_0_e2641: f64 = (noise_metadata_schedule_269_0_e2630 + noise_metadata_schedule_269_0_e2640);
        let noise_metadata_schedule_269_0_e2642: f64 = (noise_metadata_schedule_269_0_e2620 / noise_metadata_schedule_269_0_e2641);
        (noise_metadata_schedule_269_0_e2642,)
    } else {
        (w[184],)
    }
};
            w[184] = noise_metadata_schedule_269_0_e2644;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_270_0_e2647: f64 = if params.p63 > 2.0 { 1.0 } else { 0.0 };
            w[213] = noise_metadata_schedule_270_0_e2647;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_271_0_e2661,) = {
    if ((w[211] != 0.0) && (w[213] != 0.0)) {
        let noise_metadata_schedule_271_0_e2654: f64 = (params.p47 * w[184]);
        let noise_metadata_schedule_271_0_e2657: f64 = (params.p47 + w[193]);
        let noise_metadata_schedule_271_0_e2658: f64 = (noise_metadata_schedule_271_0_e2654 / noise_metadata_schedule_271_0_e2657);
        let noise_metadata_schedule_271_0_e2659: f64 = (w[107] + noise_metadata_schedule_271_0_e2658);
        (noise_metadata_schedule_271_0_e2659,)
    } else {
        (w[191],)
    }
};
            w[191] = noise_metadata_schedule_271_0_e2661;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_272_0_e2671,) = {
    if ((w[211] != 0.0) && (w[213] != 0.0)) {
        let noise_metadata_schedule_272_0_e2667: f64 = (4.0 * w[191]);
        let noise_metadata_schedule_272_0_e2669: f64 = (noise_metadata_schedule_272_0_e2667 * w[191]);
        (noise_metadata_schedule_272_0_e2669,)
    } else {
        (w[192],)
    }
};
            w[192] = noise_metadata_schedule_272_0_e2671;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_273_0_e2703,) = {
    if ((w[211] != 0.0) && (w[213] != 0.0)) {
        let noise_metadata_schedule_273_0_e2677: f64 = (2.0 * w[151]);
        let noise_metadata_schedule_273_0_e2679: f64 = (noise_metadata_schedule_273_0_e2677 * w[193]);
        let noise_metadata_schedule_273_0_e2682: f64 = (w[151] - w[193]);
        let noise_metadata_schedule_273_0_e2685: f64 = (w[151] - w[193]);
        let noise_metadata_schedule_273_0_e2686: f64 = (noise_metadata_schedule_273_0_e2682 * noise_metadata_schedule_273_0_e2685);
        let noise_metadata_schedule_273_0_e2688: f64 = (noise_metadata_schedule_273_0_e2686 + w[192]);
        let noise_metadata_schedule_273_0_e2689: f64 = (noise_metadata_schedule_273_0_e2688).sqrt();
        let noise_metadata_schedule_273_0_e2692: f64 = (w[151] + w[193]);
        let noise_metadata_schedule_273_0_e2695: f64 = (w[151] + w[193]);
        let noise_metadata_schedule_273_0_e2696: f64 = (noise_metadata_schedule_273_0_e2692 * noise_metadata_schedule_273_0_e2695);
        let noise_metadata_schedule_273_0_e2698: f64 = (noise_metadata_schedule_273_0_e2696 + w[192]);
        let noise_metadata_schedule_273_0_e2699: f64 = (noise_metadata_schedule_273_0_e2698).sqrt();
        let noise_metadata_schedule_273_0_e2700: f64 = (noise_metadata_schedule_273_0_e2689 + noise_metadata_schedule_273_0_e2699);
        let noise_metadata_schedule_273_0_e2701: f64 = (noise_metadata_schedule_273_0_e2679 / noise_metadata_schedule_273_0_e2700);
        (noise_metadata_schedule_273_0_e2701,)
    } else {
        (w[184],)
    }
};
            w[184] = noise_metadata_schedule_273_0_e2703;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_274_0_e2714,) = {
    if (w[211] != 0.0) {
        let noise_metadata_schedule_274_0_e2709: f64 = (w[197] + w[184]);
        let noise_metadata_schedule_274_0_e2710: f64 = (noise_metadata_schedule_274_0_e2709).sqrt();
        let noise_metadata_schedule_274_0_e2711: f64 = (w[196] * noise_metadata_schedule_274_0_e2710);
        let noise_metadata_schedule_274_0_e2712: f64 = (1.0 - noise_metadata_schedule_274_0_e2711);
        (noise_metadata_schedule_274_0_e2712,)
    } else {
        (w[190],)
    }
};
            w[190] = noise_metadata_schedule_274_0_e2714;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_275_0_e2717: f64 = if w[18] > 0.0 { 1.0 } else { 0.0 };
            w[214] = noise_metadata_schedule_275_0_e2717;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_276_0_e2731,) = {
    if ((w[211] != 0.0) && (w[214] != 0.0)) {
        let noise_metadata_schedule_276_0_e2724: f64 = (w[184] / w[28]);
        let noise_metadata_schedule_276_0_e2726: f64 = (noise_metadata_schedule_276_0_e2724 - w[19]);
        let noise_metadata_schedule_276_0_e2727: f64 = (0.5 * noise_metadata_schedule_276_0_e2726);
        let noise_metadata_schedule_276_0_e2729: f64 = (noise_metadata_schedule_276_0_e2727 * w[18]);
        (noise_metadata_schedule_276_0_e2729,)
    } else {
        (w[185],)
    }
};
            w[185] = noise_metadata_schedule_276_0_e2731;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_277_0_e2745,) = {
    if ((w[211] != 0.0) && (w[214] != 0.0)) {
        let noise_metadata_schedule_277_0_e2738: f64 = (w[184] / w[28]);
        let noise_metadata_schedule_277_0_e2740: f64 = (noise_metadata_schedule_277_0_e2738 + w[19]);
        let noise_metadata_schedule_277_0_e2741: f64 = (0.5 * noise_metadata_schedule_277_0_e2740);
        let noise_metadata_schedule_277_0_e2743: f64 = (noise_metadata_schedule_277_0_e2741 * w[18]);
        (noise_metadata_schedule_277_0_e2743,)
    } else {
        (w[186],)
    }
};
            w[186] = noise_metadata_schedule_277_0_e2745;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_278_0_e2756,) = {
    if ((w[211] != 0.0) && (w[214] != 0.0)) {
        let noise_metadata_schedule_278_0_e2751: f64 = (w[185] * w[185]);
        let noise_metadata_schedule_278_0_e2753: f64 = (noise_metadata_schedule_278_0_e2751 + w[20]);
        let noise_metadata_schedule_278_0_e2754: f64 = (noise_metadata_schedule_278_0_e2753).sqrt();
        (noise_metadata_schedule_278_0_e2754,)
    } else {
        (w[188],)
    }
};
            w[188] = noise_metadata_schedule_278_0_e2756;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_279_0_e2767,) = {
    if ((w[211] != 0.0) && (w[214] != 0.0)) {
        let noise_metadata_schedule_279_0_e2762: f64 = (w[186] * w[186]);
        let noise_metadata_schedule_279_0_e2764: f64 = (noise_metadata_schedule_279_0_e2762 + w[20]);
        let noise_metadata_schedule_279_0_e2765: f64 = (noise_metadata_schedule_279_0_e2764).sqrt();
        (noise_metadata_schedule_279_0_e2765,)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_279_0_e2767;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_280_0_e2777,) = {
    if ((w[211] != 0.0) && (w[214] != 0.0)) {
        let noise_metadata_schedule_280_0_e2773: f64 = (w[188] + w[187]);
        let noise_metadata_schedule_280_0_e2775: f64 = (noise_metadata_schedule_280_0_e2773 - w[21]);
        (noise_metadata_schedule_280_0_e2775,)
    } else {
        (w[189],)
    }
};
            w[189] = noise_metadata_schedule_280_0_e2777;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_281_0_e2784,) = {
    if ((w[211] != 0.0) && (w[214] == 0.0)) {
        (0.0,)
    } else {
        (w[189],)
    }
};
            w[189] = noise_metadata_schedule_281_0_e2784;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_282_0_e2815,) = {
    if (w[211] == 0.0) {
        let noise_metadata_schedule_282_0_e2789: f64 = (2.0 * w[151]);
        let noise_metadata_schedule_282_0_e2791: f64 = (noise_metadata_schedule_282_0_e2789 * w[183]);
        let noise_metadata_schedule_282_0_e2794: f64 = (w[151] - w[183]);
        let noise_metadata_schedule_282_0_e2797: f64 = (w[151] - w[183]);
        let noise_metadata_schedule_282_0_e2798: f64 = (noise_metadata_schedule_282_0_e2794 * noise_metadata_schedule_282_0_e2797);
        let noise_metadata_schedule_282_0_e2800: f64 = (noise_metadata_schedule_282_0_e2798 + w[107]);
        let noise_metadata_schedule_282_0_e2801: f64 = (noise_metadata_schedule_282_0_e2800).sqrt();
        let noise_metadata_schedule_282_0_e2804: f64 = (w[151] + w[183]);
        let noise_metadata_schedule_282_0_e2807: f64 = (w[151] + w[183]);
        let noise_metadata_schedule_282_0_e2808: f64 = (noise_metadata_schedule_282_0_e2804 * noise_metadata_schedule_282_0_e2807);
        let noise_metadata_schedule_282_0_e2810: f64 = (noise_metadata_schedule_282_0_e2808 + w[107]);
        let noise_metadata_schedule_282_0_e2811: f64 = (noise_metadata_schedule_282_0_e2810).sqrt();
        let noise_metadata_schedule_282_0_e2812: f64 = (noise_metadata_schedule_282_0_e2801 + noise_metadata_schedule_282_0_e2811);
        let noise_metadata_schedule_282_0_e2813: f64 = (noise_metadata_schedule_282_0_e2791 / noise_metadata_schedule_282_0_e2812);
        (noise_metadata_schedule_282_0_e2813,)
    } else {
        (w[184],)
    }
};
            w[184] = noise_metadata_schedule_282_0_e2815;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_283_0_e2818: f64 = if w[18] > 0.0 { 1.0 } else { 0.0 };
            w[215] = noise_metadata_schedule_283_0_e2818;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_284_0_e2833,) = {
    if ((w[211] == 0.0) && (w[215] != 0.0)) {
        let noise_metadata_schedule_284_0_e2826: f64 = (w[184] / w[28]);
        let noise_metadata_schedule_284_0_e2828: f64 = (noise_metadata_schedule_284_0_e2826 - w[19]);
        let noise_metadata_schedule_284_0_e2829: f64 = (0.5 * noise_metadata_schedule_284_0_e2828);
        let noise_metadata_schedule_284_0_e2831: f64 = (noise_metadata_schedule_284_0_e2829 * w[18]);
        (noise_metadata_schedule_284_0_e2831,)
    } else {
        (w[185],)
    }
};
            w[185] = noise_metadata_schedule_284_0_e2833;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_285_0_e2848,) = {
    if ((w[211] == 0.0) && (w[215] != 0.0)) {
        let noise_metadata_schedule_285_0_e2841: f64 = (w[184] / w[28]);
        let noise_metadata_schedule_285_0_e2843: f64 = (noise_metadata_schedule_285_0_e2841 + w[19]);
        let noise_metadata_schedule_285_0_e2844: f64 = (0.5 * noise_metadata_schedule_285_0_e2843);
        let noise_metadata_schedule_285_0_e2846: f64 = (noise_metadata_schedule_285_0_e2844 * w[18]);
        (noise_metadata_schedule_285_0_e2846,)
    } else {
        (w[186],)
    }
};
            w[186] = noise_metadata_schedule_285_0_e2848;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_286_0_e2860,) = {
    if ((w[211] == 0.0) && (w[215] != 0.0)) {
        let noise_metadata_schedule_286_0_e2855: f64 = (w[185] * w[185]);
        let noise_metadata_schedule_286_0_e2857: f64 = (noise_metadata_schedule_286_0_e2855 + w[20]);
        let noise_metadata_schedule_286_0_e2858: f64 = (noise_metadata_schedule_286_0_e2857).sqrt();
        (noise_metadata_schedule_286_0_e2858,)
    } else {
        (w[188],)
    }
};
            w[188] = noise_metadata_schedule_286_0_e2860;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_287_0_e2872,) = {
    if ((w[211] == 0.0) && (w[215] != 0.0)) {
        let noise_metadata_schedule_287_0_e2867: f64 = (w[186] * w[186]);
        let noise_metadata_schedule_287_0_e2869: f64 = (noise_metadata_schedule_287_0_e2867 + w[20]);
        let noise_metadata_schedule_287_0_e2870: f64 = (noise_metadata_schedule_287_0_e2869).sqrt();
        (noise_metadata_schedule_287_0_e2870,)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_287_0_e2872;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_288_0_e2883,) = {
    if ((w[211] == 0.0) && (w[215] != 0.0)) {
        let noise_metadata_schedule_288_0_e2879: f64 = (w[188] + w[187]);
        let noise_metadata_schedule_288_0_e2881: f64 = (noise_metadata_schedule_288_0_e2879 - w[21]);
        (noise_metadata_schedule_288_0_e2881,)
    } else {
        (w[189],)
    }
};
            w[189] = noise_metadata_schedule_288_0_e2883;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_289_0_e2891,) = {
    if ((w[211] == 0.0) && (w[215] == 0.0)) {
        (0.0,)
    } else {
        (w[189],)
    }
};
            w[189] = noise_metadata_schedule_289_0_e2891;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_290_0_e2903,) = {
    if (w[211] == 0.0) {
        let noise_metadata_schedule_290_0_e2898: f64 = (w[154] + w[184]);
        let noise_metadata_schedule_290_0_e2899: f64 = (noise_metadata_schedule_290_0_e2898).sqrt();
        let noise_metadata_schedule_290_0_e2900: f64 = (w[45] * noise_metadata_schedule_290_0_e2899);
        let noise_metadata_schedule_290_0_e2901: f64 = (1.0 - noise_metadata_schedule_290_0_e2900);
        (noise_metadata_schedule_290_0_e2901,)
    } else {
        (w[190],)
    }
};
            w[190] = noise_metadata_schedule_290_0_e2903;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_291_0_e2906: f64 = if w[190] < params.p64 { 1.0 } else { 0.0 };
            w[216] = noise_metadata_schedule_291_0_e2906;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_292_0_e2910,) = {
    if (w[216] != 0.0) {
        (params.p64,)
    } else {
        (w[190],)
    }
};
            w[190] = noise_metadata_schedule_292_0_e2910;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_293_0_e2913: f64 = (w[29] * w[190]);
            let noise_metadata_schedule_293_0_e2916: f64 = (1.0 + w[189]);
            let noise_metadata_schedule_293_0_e2917: f64 = (noise_metadata_schedule_293_0_e2913 / noise_metadata_schedule_293_0_e2916);
            w[63] = noise_metadata_schedule_293_0_e2917;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_294_0_e2920: f64 = (w[149] * w[63]);
            let noise_metadata_schedule_294_0_e2922: f64 = (noise_metadata_schedule_294_0_e2920 * w[184]);
            w[81] = noise_metadata_schedule_294_0_e2922;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_295_0_e2925: f64 = if w[84] > 0.0 { 1.0 } else { 0.0 };
            w[217] = noise_metadata_schedule_295_0_e2925;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_296_0_e2931,) = {
    if (w[217] != 0.0) {
        let noise_metadata_schedule_296_0_e2929: f64 = (w[31] * w[74]);
        (noise_metadata_schedule_296_0_e2929,)
    } else {
        (w[218],)
    }
};
            w[218] = noise_metadata_schedule_296_0_e2931;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_297_0_e2937,) = {
    if (w[217] != 0.0) {
        let noise_metadata_schedule_297_0_e2935: f64 = (w[32] * w[75]);
        (noise_metadata_schedule_297_0_e2935,)
    } else {
        (w[219],)
    }
};
            w[219] = noise_metadata_schedule_297_0_e2937;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_298_0_e2940: f64 = if w[218] > 0.0 { 1.0 } else { 0.0 };
            w[224] = noise_metadata_schedule_298_0_e2940;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_299_0_e2950,) = {
    if ((w[217] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_299_0_e2947: f64 = (params.p70 * w[70]);
        let noise_metadata_schedule_299_0_e2948: f64 = (1.0 / noise_metadata_schedule_299_0_e2947);
        (noise_metadata_schedule_299_0_e2948,)
    } else {
        (w[220],)
    }
};
            w[220] = noise_metadata_schedule_299_0_e2950;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_300_0_e2953: f64 = if w[65] < w[61] { 1.0 } else { 0.0 };
            w[225] = noise_metadata_schedule_300_0_e2953;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_301_0_e2964,) = {
    if (((w[217] != 0.0) && (w[224] != 0.0)) && (w[225] != 0.0)) {
        let noise_metadata_schedule_301_0_e2961: f64 = (w[65] * w[220]);
        let noise_metadata_schedule_301_0_e2962: f64 = (noise_metadata_schedule_301_0_e2961).exp();
        (noise_metadata_schedule_301_0_e2962,)
    } else {
        (w[221],)
    }
};
            w[221] = noise_metadata_schedule_301_0_e2964;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_302_0_e2984,) = {
    if (((w[217] != 0.0) && (w[224] != 0.0)) && (w[225] == 0.0)) {
        let noise_metadata_schedule_302_0_e2973: f64 = (w[61] * w[220]);
        let noise_metadata_schedule_302_0_e2974: f64 = (noise_metadata_schedule_302_0_e2973).exp();
        let noise_metadata_schedule_302_0_e2978: f64 = (w[65] - w[61]);
        let noise_metadata_schedule_302_0_e2980: f64 = (noise_metadata_schedule_302_0_e2978 * w[220]);
        let noise_metadata_schedule_302_0_e2981: f64 = (1.0 + noise_metadata_schedule_302_0_e2980);
        let noise_metadata_schedule_302_0_e2982: f64 = (noise_metadata_schedule_302_0_e2974 * noise_metadata_schedule_302_0_e2981);
        (noise_metadata_schedule_302_0_e2982,)
    } else {
        (w[221],)
    }
};
            w[221] = noise_metadata_schedule_302_0_e2984;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_303_0_e2994,) = {
    if ((w[217] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_303_0_e2991: f64 = (w[221] - 1.0);
        let noise_metadata_schedule_303_0_e2992: f64 = (w[218] * noise_metadata_schedule_303_0_e2991);
        (noise_metadata_schedule_303_0_e2992,)
    } else {
        (w[222],)
    }
};
            w[222] = noise_metadata_schedule_303_0_e2994;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_304_0_e3001,) = {
    if ((w[217] != 0.0) && (w[224] == 0.0)) {
        (0.0,)
    } else {
        (w[222],)
    }
};
            w[222] = noise_metadata_schedule_304_0_e3001;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_305_0_e3004: f64 = if w[219] > 0.0 { 1.0 } else { 0.0 };
            w[226] = noise_metadata_schedule_305_0_e3004;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_306_0_e3014,) = {
    if ((w[217] != 0.0) && (w[226] != 0.0)) {
        let noise_metadata_schedule_306_0_e3011: f64 = (params.p77 * w[70]);
        let noise_metadata_schedule_306_0_e3012: f64 = (1.0 / noise_metadata_schedule_306_0_e3011);
        (noise_metadata_schedule_306_0_e3012,)
    } else {
        (w[220],)
    }
};
            w[220] = noise_metadata_schedule_306_0_e3014;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_307_0_e3017: f64 = if w[65] < w[60] { 1.0 } else { 0.0 };
            w[227] = noise_metadata_schedule_307_0_e3017;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_308_0_e3028,) = {
    if (((w[217] != 0.0) && (w[226] != 0.0)) && (w[227] != 0.0)) {
        let noise_metadata_schedule_308_0_e3025: f64 = (w[65] * w[220]);
        let noise_metadata_schedule_308_0_e3026: f64 = (noise_metadata_schedule_308_0_e3025).exp();
        (noise_metadata_schedule_308_0_e3026,)
    } else {
        (w[221],)
    }
};
            w[221] = noise_metadata_schedule_308_0_e3028;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_309_0_e3048,) = {
    if (((w[217] != 0.0) && (w[226] != 0.0)) && (w[227] == 0.0)) {
        let noise_metadata_schedule_309_0_e3037: f64 = (w[60] * w[220]);
        let noise_metadata_schedule_309_0_e3038: f64 = (noise_metadata_schedule_309_0_e3037).exp();
        let noise_metadata_schedule_309_0_e3042: f64 = (w[65] - w[60]);
        let noise_metadata_schedule_309_0_e3044: f64 = (noise_metadata_schedule_309_0_e3042 * w[220]);
        let noise_metadata_schedule_309_0_e3045: f64 = (1.0 + noise_metadata_schedule_309_0_e3044);
        let noise_metadata_schedule_309_0_e3046: f64 = (noise_metadata_schedule_309_0_e3038 * noise_metadata_schedule_309_0_e3045);
        (noise_metadata_schedule_309_0_e3046,)
    } else {
        (w[221],)
    }
};
            w[221] = noise_metadata_schedule_309_0_e3048;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_310_0_e3058,) = {
    if ((w[217] != 0.0) && (w[226] != 0.0)) {
        let noise_metadata_schedule_310_0_e3055: f64 = (w[221] - 1.0);
        let noise_metadata_schedule_310_0_e3056: f64 = (w[219] * noise_metadata_schedule_310_0_e3055);
        (noise_metadata_schedule_310_0_e3056,)
    } else {
        (w[223],)
    }
};
            w[223] = noise_metadata_schedule_310_0_e3058;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_311_0_e3065,) = {
    if ((w[217] != 0.0) && (w[226] == 0.0)) {
        (0.0,)
    } else {
        (w[223],)
    }
};
            w[223] = noise_metadata_schedule_311_0_e3065;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_312_0_e3071,) = {
    if (w[217] != 0.0) {
        let noise_metadata_schedule_312_0_e3069: f64 = (w[222] + w[223]);
        (noise_metadata_schedule_312_0_e3069,)
    } else {
        (w[90],)
    }
};
            w[90] = noise_metadata_schedule_312_0_e3071;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_313_0_e3074: f64 = if w[103] > 0.0 { 1.0 } else { 0.0 };
            w[231] = noise_metadata_schedule_313_0_e3074;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_314_0_e3083,) = {
    if ((w[217] != 0.0) && (w[231] != 0.0)) {
        let noise_metadata_schedule_314_0_e3079: f64 = (-w[103]);
        let noise_metadata_schedule_314_0_e3081: f64 = (noise_metadata_schedule_314_0_e3079 - w[65]);
        (noise_metadata_schedule_314_0_e3081,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_314_0_e3083;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_315_0_e3093,) = {
    if ((w[217] != 0.0) && (w[231] != 0.0)) {
        let noise_metadata_schedule_315_0_e3090: f64 = (w[104] * w[70]);
        let noise_metadata_schedule_315_0_e3091: f64 = (1.0 / noise_metadata_schedule_315_0_e3090);
        (noise_metadata_schedule_315_0_e3091,)
    } else {
        (w[229],)
    }
};
            w[229] = noise_metadata_schedule_315_0_e3093;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_316_0_e3096: f64 = if w[228] < w[62] { 1.0 } else { 0.0 };
            w[232] = noise_metadata_schedule_316_0_e3096;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_317_0_e3107,) = {
    if (((w[217] != 0.0) && (w[231] != 0.0)) && (w[232] != 0.0)) {
        let noise_metadata_schedule_317_0_e3104: f64 = (w[228] * w[229]);
        let noise_metadata_schedule_317_0_e3105: f64 = (noise_metadata_schedule_317_0_e3104).exp();
        (noise_metadata_schedule_317_0_e3105,)
    } else {
        (w[230],)
    }
};
            w[230] = noise_metadata_schedule_317_0_e3107;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_318_0_e3127,) = {
    if (((w[217] != 0.0) && (w[231] != 0.0)) && (w[232] == 0.0)) {
        let noise_metadata_schedule_318_0_e3116: f64 = (w[62] * w[229]);
        let noise_metadata_schedule_318_0_e3117: f64 = (noise_metadata_schedule_318_0_e3116).exp();
        let noise_metadata_schedule_318_0_e3121: f64 = (w[228] - w[62]);
        let noise_metadata_schedule_318_0_e3123: f64 = (noise_metadata_schedule_318_0_e3121 * w[229]);
        let noise_metadata_schedule_318_0_e3124: f64 = (1.0 + noise_metadata_schedule_318_0_e3123);
        let noise_metadata_schedule_318_0_e3125: f64 = (noise_metadata_schedule_318_0_e3117 * noise_metadata_schedule_318_0_e3124);
        (noise_metadata_schedule_318_0_e3125,)
    } else {
        (w[230],)
    }
};
            w[230] = noise_metadata_schedule_318_0_e3127;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_319_0_e3142,) = {
    if ((w[217] != 0.0) && (w[231] != 0.0)) {
        let noise_metadata_schedule_319_0_e3132: f64 = (-params.p84);
        let noise_metadata_schedule_319_0_e3135: f64 = (-w[103]);
        let noise_metadata_schedule_319_0_e3137: f64 = (noise_metadata_schedule_319_0_e3135 * w[229]);
        let noise_metadata_schedule_319_0_e3138: f64 = (noise_metadata_schedule_319_0_e3137).exp();
        let noise_metadata_schedule_319_0_e3139: f64 = (w[230] - noise_metadata_schedule_319_0_e3138);
        let noise_metadata_schedule_319_0_e3140: f64 = (noise_metadata_schedule_319_0_e3132 * noise_metadata_schedule_319_0_e3139);
        (noise_metadata_schedule_319_0_e3140,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_319_0_e3142;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_320_0_e3149,) = {
    if ((w[217] != 0.0) && (w[231] == 0.0)) {
        (0.0,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_320_0_e3149;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_322_0_e3164,) = {
    if (w[217] == 0.0) {
        (0.0,)
    } else {
        (w[90],)
    }
};
            w[90] = noise_metadata_schedule_322_0_e3164;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_323_0_e3169,) = {
    if (w[217] == 0.0) {
        (0.0,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_323_0_e3169;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_325_0_e3177: f64 = if w[85] > 0.0 { 1.0 } else { 0.0 };
            w[233] = noise_metadata_schedule_325_0_e3177;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_326_0_e3183,) = {
    if (w[233] != 0.0) {
        let noise_metadata_schedule_326_0_e3181: f64 = (w[33] * w[74]);
        (noise_metadata_schedule_326_0_e3181,)
    } else {
        (w[234],)
    }
};
            w[234] = noise_metadata_schedule_326_0_e3183;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_327_0_e3189,) = {
    if (w[233] != 0.0) {
        let noise_metadata_schedule_327_0_e3187: f64 = (w[34] * w[75]);
        (noise_metadata_schedule_327_0_e3187,)
    } else {
        (w[235],)
    }
};
            w[235] = noise_metadata_schedule_327_0_e3189;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_328_0_e3192: f64 = if w[234] > 0.0 { 1.0 } else { 0.0 };
            w[240] = noise_metadata_schedule_328_0_e3192;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_329_0_e3202,) = {
    if ((w[233] != 0.0) && (w[240] != 0.0)) {
        let noise_metadata_schedule_329_0_e3199: f64 = (params.p70 * w[70]);
        let noise_metadata_schedule_329_0_e3200: f64 = (1.0 / noise_metadata_schedule_329_0_e3199);
        (noise_metadata_schedule_329_0_e3200,)
    } else {
        (w[236],)
    }
};
            w[236] = noise_metadata_schedule_329_0_e3202;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_330_0_e3205: f64 = if w[66] < w[61] { 1.0 } else { 0.0 };
            w[241] = noise_metadata_schedule_330_0_e3205;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_331_0_e3216,) = {
    if (((w[233] != 0.0) && (w[240] != 0.0)) && (w[241] != 0.0)) {
        let noise_metadata_schedule_331_0_e3213: f64 = (w[66] * w[236]);
        let noise_metadata_schedule_331_0_e3214: f64 = (noise_metadata_schedule_331_0_e3213).exp();
        (noise_metadata_schedule_331_0_e3214,)
    } else {
        (w[237],)
    }
};
            w[237] = noise_metadata_schedule_331_0_e3216;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_332_0_e3236,) = {
    if (((w[233] != 0.0) && (w[240] != 0.0)) && (w[241] == 0.0)) {
        let noise_metadata_schedule_332_0_e3225: f64 = (w[61] * w[236]);
        let noise_metadata_schedule_332_0_e3226: f64 = (noise_metadata_schedule_332_0_e3225).exp();
        let noise_metadata_schedule_332_0_e3230: f64 = (w[66] - w[61]);
        let noise_metadata_schedule_332_0_e3232: f64 = (noise_metadata_schedule_332_0_e3230 * w[236]);
        let noise_metadata_schedule_332_0_e3233: f64 = (1.0 + noise_metadata_schedule_332_0_e3232);
        let noise_metadata_schedule_332_0_e3234: f64 = (noise_metadata_schedule_332_0_e3226 * noise_metadata_schedule_332_0_e3233);
        (noise_metadata_schedule_332_0_e3234,)
    } else {
        (w[237],)
    }
};
            w[237] = noise_metadata_schedule_332_0_e3236;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_333_0_e3246,) = {
    if ((w[233] != 0.0) && (w[240] != 0.0)) {
        let noise_metadata_schedule_333_0_e3243: f64 = (w[237] - 1.0);
        let noise_metadata_schedule_333_0_e3244: f64 = (w[234] * noise_metadata_schedule_333_0_e3243);
        (noise_metadata_schedule_333_0_e3244,)
    } else {
        (w[238],)
    }
};
            w[238] = noise_metadata_schedule_333_0_e3246;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 329], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_334_0_e3253,) = {
    if ((w[233] != 0.0) && (w[240] == 0.0)) {
        (0.0,)
    } else {
        (w[238],)
    }
};
            w[238] = noise_metadata_schedule_334_0_e3253;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_335_0_e3256: f64 = if w[235] > 0.0 { 1.0 } else { 0.0 };
            w[242] = noise_metadata_schedule_335_0_e3256;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_336_0_e3266,) = {
    if ((w[233] != 0.0) && (w[242] != 0.0)) {
        let noise_metadata_schedule_336_0_e3263: f64 = (params.p77 * w[70]);
        let noise_metadata_schedule_336_0_e3264: f64 = (1.0 / noise_metadata_schedule_336_0_e3263);
        (noise_metadata_schedule_336_0_e3264,)
    } else {
        (w[236],)
    }
};
            w[236] = noise_metadata_schedule_336_0_e3266;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_337_0_e3269: f64 = if w[66] < w[60] { 1.0 } else { 0.0 };
            w[243] = noise_metadata_schedule_337_0_e3269;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_338_0_e3280,) = {
    if (((w[233] != 0.0) && (w[242] != 0.0)) && (w[243] != 0.0)) {
        let noise_metadata_schedule_338_0_e3277: f64 = (w[66] * w[236]);
        let noise_metadata_schedule_338_0_e3278: f64 = (noise_metadata_schedule_338_0_e3277).exp();
        (noise_metadata_schedule_338_0_e3278,)
    } else {
        (w[237],)
    }
};
            w[237] = noise_metadata_schedule_338_0_e3280;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_339_0_e3300,) = {
    if (((w[233] != 0.0) && (w[242] != 0.0)) && (w[243] == 0.0)) {
        let noise_metadata_schedule_339_0_e3289: f64 = (w[60] * w[236]);
        let noise_metadata_schedule_339_0_e3290: f64 = (noise_metadata_schedule_339_0_e3289).exp();
        let noise_metadata_schedule_339_0_e3294: f64 = (w[66] - w[60]);
        let noise_metadata_schedule_339_0_e3296: f64 = (noise_metadata_schedule_339_0_e3294 * w[236]);
        let noise_metadata_schedule_339_0_e3297: f64 = (1.0 + noise_metadata_schedule_339_0_e3296);
        let noise_metadata_schedule_339_0_e3298: f64 = (noise_metadata_schedule_339_0_e3290 * noise_metadata_schedule_339_0_e3297);
        (noise_metadata_schedule_339_0_e3298,)
    } else {
        (w[237],)
    }
};
            w[237] = noise_metadata_schedule_339_0_e3300;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_340_0_e3310,) = {
    if ((w[233] != 0.0) && (w[242] != 0.0)) {
        let noise_metadata_schedule_340_0_e3307: f64 = (w[237] - 1.0);
        let noise_metadata_schedule_340_0_e3308: f64 = (w[235] * noise_metadata_schedule_340_0_e3307);
        (noise_metadata_schedule_340_0_e3308,)
    } else {
        (w[239],)
    }
};
            w[239] = noise_metadata_schedule_340_0_e3310;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_341_0_e3317,) = {
    if ((w[233] != 0.0) && (w[242] == 0.0)) {
        (0.0,)
    } else {
        (w[239],)
    }
};
            w[239] = noise_metadata_schedule_341_0_e3317;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_342_0_e3323,) = {
    if (w[233] != 0.0) {
        let noise_metadata_schedule_342_0_e3321: f64 = (w[238] + w[239]);
        (noise_metadata_schedule_342_0_e3321,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_342_0_e3323;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_343_0_e3326: f64 = if w[103] > 0.0 { 1.0 } else { 0.0 };
            w[247] = noise_metadata_schedule_343_0_e3326;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_344_0_e3335,) = {
    if ((w[233] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_344_0_e3331: f64 = (-w[103]);
        let noise_metadata_schedule_344_0_e3333: f64 = (noise_metadata_schedule_344_0_e3331 - w[66]);
        (noise_metadata_schedule_344_0_e3333,)
    } else {
        (w[244],)
    }
};
            w[244] = noise_metadata_schedule_344_0_e3335;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_345_0_e3345,) = {
    if ((w[233] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_345_0_e3342: f64 = (w[104] * w[70]);
        let noise_metadata_schedule_345_0_e3343: f64 = (1.0 / noise_metadata_schedule_345_0_e3342);
        (noise_metadata_schedule_345_0_e3343,)
    } else {
        (w[245],)
    }
};
            w[245] = noise_metadata_schedule_345_0_e3345;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_346_0_e3348: f64 = if w[244] < w[62] { 1.0 } else { 0.0 };
            w[248] = noise_metadata_schedule_346_0_e3348;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_347_0_e3359,) = {
    if (((w[233] != 0.0) && (w[247] != 0.0)) && (w[248] != 0.0)) {
        let noise_metadata_schedule_347_0_e3356: f64 = (w[244] * w[245]);
        let noise_metadata_schedule_347_0_e3357: f64 = (noise_metadata_schedule_347_0_e3356).exp();
        (noise_metadata_schedule_347_0_e3357,)
    } else {
        (w[246],)
    }
};
            w[246] = noise_metadata_schedule_347_0_e3359;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_348_0_e3379,) = {
    if (((w[233] != 0.0) && (w[247] != 0.0)) && (w[248] == 0.0)) {
        let noise_metadata_schedule_348_0_e3368: f64 = (w[62] * w[245]);
        let noise_metadata_schedule_348_0_e3369: f64 = (noise_metadata_schedule_348_0_e3368).exp();
        let noise_metadata_schedule_348_0_e3373: f64 = (w[244] - w[62]);
        let noise_metadata_schedule_348_0_e3375: f64 = (noise_metadata_schedule_348_0_e3373 * w[245]);
        let noise_metadata_schedule_348_0_e3376: f64 = (1.0 + noise_metadata_schedule_348_0_e3375);
        let noise_metadata_schedule_348_0_e3377: f64 = (noise_metadata_schedule_348_0_e3369 * noise_metadata_schedule_348_0_e3376);
        (noise_metadata_schedule_348_0_e3377,)
    } else {
        (w[246],)
    }
};
            w[246] = noise_metadata_schedule_348_0_e3379;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_349_0_e3394,) = {
    if ((w[233] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_349_0_e3384: f64 = (-params.p84);
        let noise_metadata_schedule_349_0_e3387: f64 = (-w[103]);
        let noise_metadata_schedule_349_0_e3389: f64 = (noise_metadata_schedule_349_0_e3387 * w[245]);
        let noise_metadata_schedule_349_0_e3390: f64 = (noise_metadata_schedule_349_0_e3389).exp();
        let noise_metadata_schedule_349_0_e3391: f64 = (w[246] - noise_metadata_schedule_349_0_e3390);
        let noise_metadata_schedule_349_0_e3392: f64 = (noise_metadata_schedule_349_0_e3384 * noise_metadata_schedule_349_0_e3391);
        (noise_metadata_schedule_349_0_e3392,)
    } else {
        (w[93],)
    }
};
            w[93] = noise_metadata_schedule_349_0_e3394;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_350_0_e3401,) = {
    if ((w[233] != 0.0) && (w[247] == 0.0)) {
        (0.0,)
    } else {
        (w[93],)
    }
};
            w[93] = noise_metadata_schedule_350_0_e3401;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_352_0_e3416,) = {
    if (w[233] == 0.0) {
        (0.0,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_352_0_e3416;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_353_0_e3421,) = {
    if (w[233] == 0.0) {
        (0.0,)
    } else {
        (w[93],)
    }
};
            w[93] = noise_metadata_schedule_353_0_e3421;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_372_0_e3616: f64 = (-params.p21);
            let noise_metadata_schedule_372_0_e3618: f64 = (noise_metadata_schedule_372_0_e3616 * w[81]);
            w[81] = noise_metadata_schedule_372_0_e3618;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_477_0_e4949,) = {
    if ((params.p13 != 0.0) && (params.p89 != 0.0)) {
        (w[3],)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_477_0_e4949;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_478_0_e4955,) = {
    if ((params.p13 != 0.0) && (params.p89 != 0.0)) {
        (w[4],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_478_0_e4955;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_479_0_e4962,) = {
    if ((params.p13 != 0.0) && (params.p89 == 0.0)) {
        (w[27],)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_479_0_e4962;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_480_0_e4969,) = {
    if ((params.p13 != 0.0) && (params.p89 == 0.0)) {
        (w[26],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_480_0_e4969;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_481_0_e4979,) = {
    if (params.p13 != 0.0) {
        let noise_metadata_schedule_481_0_e4973: f64 = (4.0 * 1.3806505e-23);
        let noise_metadata_schedule_481_0_e4975: f64 = (noise_metadata_schedule_481_0_e4973 * w[24]);
        let noise_metadata_schedule_481_0_e4977: f64 = (noise_metadata_schedule_481_0_e4975 * w[63]);
        (noise_metadata_schedule_481_0_e4977,)
    } else {
        (w[99],)
    }
};
            w[99] = noise_metadata_schedule_481_0_e4979;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_482_0_e4994,) = {
    if (params.p13 != 0.0) {
        let noise_metadata_schedule_482_0_e4984: f64 = (w[81] / w[38]);
        let noise_metadata_schedule_482_0_e4985: f64 = (noise_metadata_schedule_482_0_e4984).abs();
        let noise_metadata_schedule_482_0_e4987: f64 = (noise_metadata_schedule_482_0_e4985).powf(params.p87);
        let noise_metadata_schedule_482_0_e4988: f64 = (w[80] * noise_metadata_schedule_482_0_e4987);
        let noise_metadata_schedule_482_0_e4990: f64 = (noise_metadata_schedule_482_0_e4988 * w[38]);
        let noise_metadata_schedule_482_0_e4992: f64 = (noise_metadata_schedule_482_0_e4990 / w[37]);
        (noise_metadata_schedule_482_0_e4992,)
    } else {
        (w[100],)
    }
};
            w[100] = noise_metadata_schedule_482_0_e4994;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_483_0_e4997: f64 = if w[81] < 0.0 { 1.0 } else { 0.0 };
            w[323] = noise_metadata_schedule_483_0_e4997;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_484_0_e5004,) = {
    if ((params.p13 != 0.0) && (w[323] != 0.0)) {
        let noise_metadata_schedule_484_0_e5002: f64 = (-w[100]);
        (noise_metadata_schedule_484_0_e5002,)
    } else {
        (w[100],)
    }
};
            w[100] = noise_metadata_schedule_484_0_e5004;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_485_0_e5007: f64 = if w[54] > 0.0 { 1.0 } else { 0.0 };
            w[324] = noise_metadata_schedule_485_0_e5007;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_486_0_e5017,) = {
    if ((params.p13 != 0.0) && (w[324] != 0.0)) {
        let noise_metadata_schedule_486_0_e5014: f64 = (w[54] * w[58]);
        let noise_metadata_schedule_486_0_e5015: f64 = (1.0 / noise_metadata_schedule_486_0_e5014);
        (noise_metadata_schedule_486_0_e5015,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_486_0_e5017;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_487_0_e5024,) = {
    if ((params.p13 != 0.0) && (w[324] == 0.0)) {
        (0.0,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_487_0_e5024;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_488_0_e5027: f64 = if w[55] > 0.0 { 1.0 } else { 0.0 };
            w[325] = noise_metadata_schedule_488_0_e5027;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_489_0_e5037,) = {
    if ((params.p13 != 0.0) && (w[325] != 0.0)) {
        let noise_metadata_schedule_489_0_e5034: f64 = (w[55] * w[58]);
        let noise_metadata_schedule_489_0_e5035: f64 = (1.0 / noise_metadata_schedule_489_0_e5034);
        (noise_metadata_schedule_489_0_e5035,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_489_0_e5037;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_490_0_e5044,) = {
    if ((params.p13 != 0.0) && (w[325] == 0.0)) {
        (0.0,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_490_0_e5044;
        }
    }
}

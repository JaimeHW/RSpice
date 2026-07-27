#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GGI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GGI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 29, is_current: false, branch_ordinal: Some(8), pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 33, is_current: false, branch_ordinal: Some(12), pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_DII_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 37, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(17), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(18), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 145];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            params[0] != 0.0
        };
        let noise_source_1_active = {
            let noise_1_activation_e250: f64 = if ((w[125] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_1_activation_e250 != 0.0
        };
        let noise_source_2_active = {
            let noise_2_activation_e279: f64 = if ((w[126] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_2_activation_e279 != 0.0
        };
        let noise_source_3_active = {
            let noise_3_activation_e308: f64 = if ((w[127] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_3_activation_e308 != 0.0
        };
        let noise_source_4_active = {
            let noise_4_activation_e336: f64 = if ((w[128] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_4_activation_e336 != 0.0
        };
        let noise_source_5_active = {
            let noise_5_activation_e344: f64 = if ((w[128] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_5_activation_e344 != 0.0
        };
        let noise_source_6_active = {
            let noise_6_activation_e358: f64 = if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_6_activation_e358 != 0.0
        };
        let noise_source_7_active = {
            let noise_7_activation_e378: f64 = if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_7_activation_e378 != 0.0
        };
        let noise_source_8_active = {
            let noise_8_activation_e435: f64 = if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_8_activation_e435 != 0.0
        };
        let noise_source_9_active = {
            let noise_9_activation_e446: f64 = if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_9_activation_e446 != 0.0
        };
        let noise_source_10_active = {
            let noise_10_activation_e458: f64 = if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_10_activation_e458 != 0.0
        };
        let noise_source_11_active = {
            let noise_11_activation_e472: f64 = if ((((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) && (w[142] != 0.0)) { 1.0 } else { 0.0 };
            noise_11_activation_e472 != 0.0
        };
        let noise_source_12_active = {
            params[0] != 0.0
        };
        let noise_source_13_active = {
            params[0] != 0.0
        };
        let noise_source_14_active = {
            let noise_14_activation_e509: f64 = if ((params[0] != 0.0) && (w[143] != 0.0)) { 1.0 } else { 0.0 };
            noise_14_activation_e509 != 0.0
        };
        let noise_source_15_active = {
            let noise_15_activation_e523: f64 = if ((params[0] != 0.0) && (w[143] != 0.0)) { 1.0 } else { 0.0 };
            noise_15_activation_e523 != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active, noise_source_4_active, noise_source_5_active, noise_source_6_active, noise_source_7_active, noise_source_8_active, noise_source_9_active, noise_source_10_active, noise_source_11_active, noise_source_12_active, noise_source_13_active, noise_source_14_active, noise_source_15_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3) | ((noise_source_4_active as u128) << 4) | ((noise_source_5_active as u128) << 5) | ((noise_source_6_active as u128) << 6) | ((noise_source_7_active as u128) << 7) | ((noise_source_8_active as u128) << 8) | ((noise_source_9_active as u128) << 9) | ((noise_source_10_active as u128) << 10) | ((noise_source_11_active as u128) << 11) | ((noise_source_12_active as u128) << 12) | ((noise_source_13_active as u128) << 13) | ((noise_source_14_active as u128) << 14) | ((noise_source_15_active as u128) << 15)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_2(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e3085: f64 = 1.0;
            let noise_0_psd_e222: f64 = (4.0 * 1.3806503e-23);
            let noise_0_psd_e224: f64 = (noise_0_psd_e222 * w[15]);
            let noise_0_psd_e226: f64 = (noise_0_psd_e224 * params[51]);
            let noise_0_psd_e3086: f64 = (noise_0_psd_e3085 * noise_0_psd_e226);
            let psd = noise_0_psd_e3086;
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
            let noise_1_psd_e3088: f64 = 1.0;
            let noise_1_psd_e253: f64 = (4.0 * 1.3806503e-23);
            let noise_1_psd_e255: f64 = (noise_1_psd_e253 * w[15]);
            let noise_1_psd_e257: f64 = (noise_1_psd_e255 * params[46]);
            let noise_1_psd_e3089: f64 = (noise_1_psd_e3088 * noise_1_psd_e257);
            let psd = noise_1_psd_e3089;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[2] {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_2_psd_e3091: f64 = 1.0;
            let noise_2_psd_e282: f64 = (4.0 * 1.3806503e-23);
            let noise_2_psd_e284: f64 = (noise_2_psd_e282 * w[15]);
            let noise_2_psd_e286: f64 = (noise_2_psd_e284 * w[50]);
            let noise_2_psd_e3092: f64 = (noise_2_psd_e3091 * noise_2_psd_e286);
            let psd = noise_2_psd_e3092;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[3] {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_3_psd_e3094: f64 = 1.0;
            let noise_3_psd_e311: f64 = (4.0 * 1.3806503e-23);
            let noise_3_psd_e313: f64 = (noise_3_psd_e311 * w[15]);
            let noise_3_psd_e315: f64 = (noise_3_psd_e313 * w[49]);
            let noise_3_psd_e3095: f64 = (noise_3_psd_e3094 * noise_3_psd_e315);
            let psd = noise_3_psd_e3095;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[4] {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_4_psd_e3097: f64 = 1.0;
            let noise_4_psd_e3098: f64 = (noise_4_psd_e3097 * w[131]);
            let psd = noise_4_psd_e3098;
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
            let noise_5_psd_e3100: f64 = 1.0;
            let noise_5_psd_e347: f64 = (w[131] * params[96]);
            let noise_5_psd_e3101: f64 = (noise_5_psd_e3100 * noise_5_psd_e347);
            let psd = noise_5_psd_e3101;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = Some(params[98]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[6] {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_6_psd_e3103: f64 = 1.0;
            let noise_6_psd_e3104: f64 = (noise_6_psd_e3103 * w[140]);
            let psd = noise_6_psd_e3104;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[7] {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_7_psd_e3106: f64 = 1.0;
            let noise_7_psd_e3107: f64 = (noise_7_psd_e3106 * w[140]);
            let psd = noise_7_psd_e3107;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[8] {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_8_psd_e3109: f64 = 1.0;
            let noise_8_psd_e3110: f64 = (noise_8_psd_e3109 * w[134]);
            let psd = noise_8_psd_e3110;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[9] {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_9_psd_e3112: f64 = 1.0;
            let noise_9_psd_e3113: f64 = (noise_9_psd_e3112 * w[135]);
            let psd = noise_9_psd_e3113;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = Some(2.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[10] {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_10_psd_e3115: f64 = 1.0;
            let noise_10_psd_e3116: f64 = (noise_10_psd_e3115 * w[141]);
            let psd = noise_10_psd_e3116;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = Some(1.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[11] {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_11_psd_e3118: f64 = 1.0;
            let noise_11_psd_e476: f64 = (w[18]).powf(params[91]);
            let noise_11_psd_e477: f64 = (params[90] * noise_11_psd_e476);
            let noise_11_psd_e3119: f64 = (noise_11_psd_e3118 * noise_11_psd_e477);
            let psd = noise_11_psd_e3119;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = Some(params[92]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[12] {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_12_psd_e3121: f64 = 1.0;
            let noise_12_psd_e488: f64 = (2.0 * 1.602176462e-19);
            let noise_12_psd_e490: f64 = (w[7]).abs();
            let noise_12_psd_e491: f64 = (noise_12_psd_e488 * noise_12_psd_e490);
            let noise_12_psd_e3122: f64 = (noise_12_psd_e3121 * noise_12_psd_e491);
            let psd = noise_12_psd_e3122;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[13] {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_13_psd_e3124: f64 = 1.0;
            let noise_13_psd_e499: f64 = (2.0 * 1.602176462e-19);
            let noise_13_psd_e501: f64 = (w[9]).abs();
            let noise_13_psd_e502: f64 = (noise_13_psd_e499 * noise_13_psd_e501);
            let noise_13_psd_e3125: f64 = (noise_13_psd_e3124 * noise_13_psd_e502);
            let psd = noise_13_psd_e3125;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[14] {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_14_psd_e3127: f64 = 1.0;
            let noise_14_psd_e512: f64 = (w[7]).abs();
            let noise_14_psd_e514: f64 = (noise_14_psd_e512).powf(params[91]);
            let noise_14_psd_e515: f64 = (params[90] * noise_14_psd_e514);
            let noise_14_psd_e3128: f64 = (noise_14_psd_e3127 * noise_14_psd_e515);
            let psd = noise_14_psd_e3128;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = Some(params[92]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[15] {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_15_psd_e3130: f64 = 1.0;
            let noise_15_psd_e526: f64 = (w[9]).abs();
            let noise_15_psd_e528: f64 = (noise_15_psd_e526).powf(params[91]);
            let noise_15_psd_e529: f64 = (params[90] * noise_15_psd_e528);
            let noise_15_psd_e3131: f64 = (noise_15_psd_e3130 * noise_15_psd_e529);
            let psd = noise_15_psd_e3131;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = Some(params[92]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 145]) {
        let params = &*self.params;
        let noise_activation_schedule_215_0_e2856: f64 = if params[46] > 0.0 { 1.0 } else { 0.0 };
        w[125] = noise_activation_schedule_215_0_e2856;
        let noise_activation_schedule_216_0_e2859: f64 = if params[50] > 0.0 { 1.0 } else { 0.0 };
        w[126] = noise_activation_schedule_216_0_e2859;
        let noise_activation_schedule_217_0_e2866: f64 = if ((params[47] > 0.0) || (params[48] > 0.0)) { 1.0 } else { 0.0 };
        w[127] = noise_activation_schedule_217_0_e2866;
        let noise_activation_schedule_221_0_e2882: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
        w[128] = noise_activation_schedule_221_0_e2882;
        let noise_activation_schedule_222_0_e2885: f64 = if params[7] == 1.0 { 1.0 } else { 0.0 };
        w[129] = noise_activation_schedule_222_0_e2885;
        let noise_activation_schedule_236_0_e3077: f64 = if params[90] > 0.0 { 1.0 } else { 0.0 };
        w[142] = noise_activation_schedule_236_0_e3077;
        let noise_activation_schedule_237_0_e3080: f64 = if params[90] > 0.0 { 1.0 } else { 0.0 };
        w[143] = noise_activation_schedule_237_0_e3080;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 145], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xf73c) != 0 {
            w[3] = (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[8]));
        }
        if (active[0] & 0xf73c) != 0 {
            w[4] = (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[5]));
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_2_0_e573: f64 = (-w[4]);
            w[6] = noise_metadata_schedule_2_0_e573;
        }
        if (active[0] & 0xf73c) != 0 {
            w[5] = (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[8]));
        }
        if (active[0] & 0x5000) != 0 {
            w[96] = (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[8]));
        }
        if (active[0] & 0xa030) != 0 {
            w[97] = w[4];
        }
        if (active[0] & 0xf73c) != 0 {
            w[11] = (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[8]));
        }
        if (active[0] & 0x830) != 0 {
            w[18] = (ctx.node_voltage(self.nodes[16]) - 0.0);
        }
        if (active[0] & 0x700) != 0 {
            w[98] = 0.0;
        }
        if (active[0] & 0xa030) != 0 {
            w[25] = 0.0;
        }
        if (active[0] & 0x5000) != 0 {
            w[24] = 0.0;
        }
        if (active[0] & 0xf7ff) != 0 {
            let noise_metadata_schedule_15_0_e587: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            w[101] = noise_metadata_schedule_15_0_e587;
        }
        if (active[0] & 0xf7ff) != 0 {
            let (noise_metadata_schedule_16_0_e593,) = {
    if (w[101] != 0.0) {
        let noise_metadata_schedule_16_0_e591: f64 = (params[3] + 273.15);
        (noise_metadata_schedule_16_0_e591,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_16_0_e593;
        }
        if (active[0] & 0xf7ff) != 0 {
            let (noise_metadata_schedule_17_0_e600,) = {
    if (w[101] == 0.0) {
        let noise_metadata_schedule_17_0_e596: f64 = ctx.temperature();
        let noise_metadata_schedule_17_0_e598: f64 = (noise_metadata_schedule_17_0_e596 + params[2]);
        (noise_metadata_schedule_17_0_e598,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_17_0_e600;
        }
        if (active[0] & 0xf7fc) != 0 {
            let noise_metadata_schedule_18_0_e602: f64 = if self.param_given[100] { 1.0 } else { 0.0 };
            w[102] = noise_metadata_schedule_18_0_e602;
        }
        if (active[0] & 0xf7fc) != 0 {
            let (noise_metadata_schedule_19_0_e608,) = {
    if (w[102] != 0.0) {
        let noise_metadata_schedule_19_0_e606: f64 = (params[100] + 273.15);
        (noise_metadata_schedule_19_0_e606,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_19_0_e608;
        }
        if (active[0] & 0xf7fc) != 0 {
            let (noise_metadata_schedule_20_0_e615,) = {
    if (w[102] == 0.0) {
        let noise_metadata_schedule_20_0_e613: f64 = (27.0 + 273.15);
        (noise_metadata_schedule_20_0_e613,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_20_0_e615;
        }
        if (active[0] & 0xf7ff) != 0 {
            let (noise_metadata_schedule_21_0_e622,) = {
    if (params[1] != 0.0) {
        let noise_metadata_schedule_21_0_e619: f64 = ((ctx.node_voltage(self.nodes[3]) - 0.0)).abs();
        let noise_metadata_schedule_21_0_e620: f64 = (w[15] + noise_metadata_schedule_21_0_e619);
        (noise_metadata_schedule_21_0_e620,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_21_0_e622;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_22_0_e624: f64 = (w[15] * THERMAL_VOLTAGE_PER_K);
            w[13] = noise_metadata_schedule_22_0_e624;
        }
        if (active[0] & 0xf7fc) != 0 {
            let noise_metadata_schedule_23_0_e627: f64 = (w[15] - w[14]);
            let noise_metadata_schedule_23_0_e628: f64 = (noise_metadata_schedule_23_0_e627).abs();
            w[16] = noise_metadata_schedule_23_0_e628;
        }
        if (active[0] & 0xf7fc) != 0 {
            let noise_metadata_schedule_24_0_e635: f64 = if ((w[16] > 0.0) || (params[66] > 0.0)) { 1.0 } else { 0.0 };
            w[103] = noise_metadata_schedule_24_0_e635;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_26_0_e657,) = {
    if (w[103] != 0.0) {
        let noise_metadata_schedule_26_0_e652: f64 = (w[16]).abs();
        let noise_metadata_schedule_26_0_e653: f64 = (params[68] * noise_metadata_schedule_26_0_e652);
        let noise_metadata_schedule_26_0_e654: f64 = (1.0 + noise_metadata_schedule_26_0_e653);
        let noise_metadata_schedule_26_0_e655: f64 = (params[8] * noise_metadata_schedule_26_0_e654);
        (noise_metadata_schedule_26_0_e655,)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_26_0_e657;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_27_0_e668,) = {
    if (w[103] != 0.0) {
        let noise_metadata_schedule_27_0_e663: f64 = (w[16]).abs();
        let noise_metadata_schedule_27_0_e664: f64 = (params[80] * noise_metadata_schedule_27_0_e663);
        let noise_metadata_schedule_27_0_e665: f64 = (1.0 + noise_metadata_schedule_27_0_e664);
        let noise_metadata_schedule_27_0_e666: f64 = (params[20] * noise_metadata_schedule_27_0_e665);
        (noise_metadata_schedule_27_0_e666,)
    } else {
        (w[43],)
    }
};
            w[43] = noise_metadata_schedule_27_0_e668;
        }
        if (active[0] & 0x2c0) != 0 {
            let (noise_metadata_schedule_28_0_e679,) = {
    if (w[103] != 0.0) {
        let noise_metadata_schedule_28_0_e674: f64 = (w[16]).abs();
        let noise_metadata_schedule_28_0_e675: f64 = (params[72] * noise_metadata_schedule_28_0_e674);
        let noise_metadata_schedule_28_0_e676: f64 = (1.0 + noise_metadata_schedule_28_0_e675);
        let noise_metadata_schedule_28_0_e677: f64 = (params[26] * noise_metadata_schedule_28_0_e676);
        (noise_metadata_schedule_28_0_e677,)
    } else {
        (w[44],)
    }
};
            w[44] = noise_metadata_schedule_28_0_e679;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_32_0_e720,) = {
    if (w[103] != 0.0) {
        let noise_metadata_schedule_32_0_e717: f64 = (params[78] * w[16]);
        let noise_metadata_schedule_32_0_e718: f64 = (params[9] + noise_metadata_schedule_32_0_e717);
        (noise_metadata_schedule_32_0_e718,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_32_0_e720;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_35_0_e748,) = {
    if (w[103] != 0.0) {
        let noise_metadata_schedule_35_0_e745: f64 = (params[79] * w[16]);
        let noise_metadata_schedule_35_0_e746: f64 = (params[45] + noise_metadata_schedule_35_0_e745);
        (noise_metadata_schedule_35_0_e746,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_35_0_e748;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_36_0_e756,) = {
    if (w[103] != 0.0) {
        let noise_metadata_schedule_36_0_e753: f64 = (params[81] * w[16]);
        let noise_metadata_schedule_36_0_e754: f64 = (params[21] + noise_metadata_schedule_36_0_e753);
        (noise_metadata_schedule_36_0_e754,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_36_0_e756;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_42_0_e828,) = {
    if (w[103] == 0.0) {
        (params[8],)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_42_0_e828;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_43_0_e833,) = {
    if (w[103] == 0.0) {
        (params[20],)
    } else {
        (w[43],)
    }
};
            w[43] = noise_metadata_schedule_43_0_e833;
        }
        if (active[0] & 0x2c0) != 0 {
            let (noise_metadata_schedule_44_0_e838,) = {
    if (w[103] == 0.0) {
        (params[26],)
    } else {
        (w[44],)
    }
};
            w[44] = noise_metadata_schedule_44_0_e838;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_50_0_e868,) = {
    if (w[103] == 0.0) {
        (params[9],)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_50_0_e868;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_53_0_e883,) = {
    if (w[103] == 0.0) {
        (params[45],)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_53_0_e883;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_54_0_e888,) = {
    if (w[103] == 0.0) {
        (params[21],)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_54_0_e888;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_55_0_e894: f64 = if ((!self.param_given[43]) && self.param_given[44]) { 1.0 } else { 0.0 };
            w[105] = noise_metadata_schedule_55_0_e894;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_56_0_e902,) = {
    if (w[105] != 0.0) {
        let noise_metadata_schedule_56_0_e898: f64 = (0.5 / params[44]);
        let noise_metadata_schedule_56_0_e900: f64 = (noise_metadata_schedule_56_0_e898 / w[13]);
        (noise_metadata_schedule_56_0_e900,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_56_0_e902;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_57_0_e907,) = {
    if (w[105] == 0.0) {
        (params[43],)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_57_0_e907;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_58_0_e910: f64 = (params[19] * w[5]);
            let noise_metadata_schedule_58_0_e911: f64 = (noise_metadata_schedule_58_0_e910).cosh();
            w[63] = noise_metadata_schedule_58_0_e911;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_59_0_e914: f64 = (params[64] * w[11]);
            w[12] = noise_metadata_schedule_59_0_e914;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_60_0_e921: f64 = (w[63] * w[63]);
            let noise_metadata_schedule_60_0_e922: f64 = (1e-12 + noise_metadata_schedule_60_0_e921);
            let noise_metadata_schedule_60_0_e923: f64 = (params[18] / noise_metadata_schedule_60_0_e922);
            let noise_metadata_schedule_60_0_e924: f64 = (1.0 + noise_metadata_schedule_60_0_e923);
            let noise_metadata_schedule_60_0_e925: f64 = (params[11] * noise_metadata_schedule_60_0_e924);
            w[59] = noise_metadata_schedule_60_0_e925;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_61_0_e930: f64 = (w[16]).abs();
            let noise_metadata_schedule_61_0_e931: f64 = (params[69] * noise_metadata_schedule_61_0_e930);
            let noise_metadata_schedule_61_0_e932: f64 = (1.0 + noise_metadata_schedule_61_0_e931);
            let noise_metadata_schedule_61_0_e933: f64 = (w[59] * noise_metadata_schedule_61_0_e932);
            w[60] = noise_metadata_schedule_61_0_e933;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_62_0_e938: f64 = (w[16]).abs();
            let noise_metadata_schedule_62_0_e939: f64 = (params[70] * noise_metadata_schedule_62_0_e938);
            let noise_metadata_schedule_62_0_e940: f64 = (1.0 + noise_metadata_schedule_62_0_e939);
            let noise_metadata_schedule_62_0_e941: f64 = (params[13] * noise_metadata_schedule_62_0_e940);
            w[61] = noise_metadata_schedule_62_0_e941;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_63_0_e944: f64 = (w[54] - params[10]);
            let noise_metadata_schedule_63_0_e948: f64 = (params[15] * w[5]);
            let noise_metadata_schedule_63_0_e949: f64 = (noise_metadata_schedule_63_0_e948).tanh();
            let noise_metadata_schedule_63_0_e950: f64 = (params[10] * noise_metadata_schedule_63_0_e949);
            let noise_metadata_schedule_63_0_e951: f64 = (noise_metadata_schedule_63_0_e944 + noise_metadata_schedule_63_0_e950);
            let noise_metadata_schedule_63_0_e953: f64 = (noise_metadata_schedule_63_0_e951 - w[12]);
            let noise_metadata_schedule_63_0_e957: f64 = (w[6] - w[53]);
            let noise_metadata_schedule_63_0_e958: f64 = (params[22] * noise_metadata_schedule_63_0_e957);
            let noise_metadata_schedule_63_0_e961: f64 = (w[6] - w[53]);
            let noise_metadata_schedule_63_0_e962: f64 = (noise_metadata_schedule_63_0_e958 * noise_metadata_schedule_63_0_e961);
            let noise_metadata_schedule_63_0_e963: f64 = (noise_metadata_schedule_63_0_e953 - noise_metadata_schedule_63_0_e962);
            w[62] = noise_metadata_schedule_63_0_e963;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_64_0_e968: f64 = (w[16]).abs();
            let noise_metadata_schedule_64_0_e969: f64 = (params[78] * noise_metadata_schedule_64_0_e968);
            let noise_metadata_schedule_64_0_e970: f64 = (1.0 + noise_metadata_schedule_64_0_e969);
            let noise_metadata_schedule_64_0_e971: f64 = (w[62] * noise_metadata_schedule_64_0_e970);
            w[58] = noise_metadata_schedule_64_0_e971;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_65_0_e974: f64 = (w[3] - w[58]);
            w[64] = noise_metadata_schedule_65_0_e974;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_66_0_e977: f64 = (w[64] * w[64]);
            w[65] = noise_metadata_schedule_66_0_e977;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_67_0_e980: f64 = (w[60] * w[64]);
            let noise_metadata_schedule_67_0_e983: f64 = (params[12] * w[65]);
            let noise_metadata_schedule_67_0_e984: f64 = (noise_metadata_schedule_67_0_e980 + noise_metadata_schedule_67_0_e983);
            let noise_metadata_schedule_67_0_e987: f64 = (w[61] * w[64]);
            let noise_metadata_schedule_67_0_e989: f64 = (noise_metadata_schedule_67_0_e987 * w[65]);
            let noise_metadata_schedule_67_0_e990: f64 = (noise_metadata_schedule_67_0_e984 + noise_metadata_schedule_67_0_e989);
            w[17] = noise_metadata_schedule_67_0_e990;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_68_0_e993: f64 = (w[17]).tanh();
            let noise_metadata_schedule_68_0_e994: f64 = (1.0 + noise_metadata_schedule_68_0_e993);
            w[75] = noise_metadata_schedule_68_0_e994;
        }
        if (active[0] & 0x70c) != 0 {
            let noise_metadata_schedule_69_0_e998: f64 = { let limexp_arg = w[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_69_0_e1000: f64 = (-w[17]);
            let noise_metadata_schedule_69_0_e1001: f64 = { let limexp_arg = noise_metadata_schedule_69_0_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_69_0_e1002: f64 = (noise_metadata_schedule_69_0_e998 - noise_metadata_schedule_69_0_e1001);
            let noise_metadata_schedule_69_0_e1003: f64 = (0.5 * noise_metadata_schedule_69_0_e1002);
            let noise_metadata_schedule_69_0_e1004: f64 = (noise_metadata_schedule_69_0_e1003).tanh();
            let noise_metadata_schedule_69_0_e1005: f64 = (1.0 + noise_metadata_schedule_69_0_e1004);
            w[76] = noise_metadata_schedule_69_0_e1005;
        }
        if (active[0] & 0x730) != 0 {
            let noise_metadata_schedule_70_0_e1009: f64 = (params[15] * w[75]);
            let noise_metadata_schedule_70_0_e1010: f64 = (params[14] + noise_metadata_schedule_70_0_e1009);
            w[0] = noise_metadata_schedule_70_0_e1010;
        }
        if (active[0] & 0x730) != 0 {
            let noise_metadata_schedule_71_0_e1013: f64 = (w[0] * w[5]);
            let noise_metadata_schedule_71_0_e1014: f64 = (noise_metadata_schedule_71_0_e1013).tanh();
            w[79] = noise_metadata_schedule_71_0_e1014;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_72_0_e1017: f64 = if params[4] == 0.0 { 1.0 } else { 0.0 };
            w[106] = noise_metadata_schedule_72_0_e1017;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_73_0_e1020: f64 = if params[4] == 1.0 { 1.0 } else { 0.0 };
            w[107] = noise_metadata_schedule_73_0_e1020;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_74_0_e1023: f64 = if params[4] == 2.0 { 1.0 } else { 0.0 };
            w[108] = noise_metadata_schedule_74_0_e1023;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_75_0_e1026: f64 = if params[4] == 3.0 { 1.0 } else { 0.0 };
            w[109] = noise_metadata_schedule_75_0_e1026;
        }
        if (active[0] & 0x700) != 0 {
            let noise_metadata_schedule_76_0_e1029: f64 = if params[4] == 4.0 { 1.0 } else { 0.0 };
            w[110] = noise_metadata_schedule_76_0_e1029;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_77_0_e1050,) = {
    if (w[106] != 0.0) {
        let noise_metadata_schedule_77_0_e1033: f64 = (w[39] * w[75]);
        let noise_metadata_schedule_77_0_e1035: f64 = (noise_metadata_schedule_77_0_e1033 * w[79]);
        let noise_metadata_schedule_77_0_e1039: f64 = (params[16] * w[5]);
        let noise_metadata_schedule_77_0_e1040: f64 = (1.0 + noise_metadata_schedule_77_0_e1039);
        let noise_metadata_schedule_77_0_e1044: f64 = (w[6] - w[53]);
        let noise_metadata_schedule_77_0_e1045: f64 = { let limexp_arg = noise_metadata_schedule_77_0_e1044; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_77_0_e1046: f64 = (w[43] * noise_metadata_schedule_77_0_e1045);
        let noise_metadata_schedule_77_0_e1047: f64 = (noise_metadata_schedule_77_0_e1040 + noise_metadata_schedule_77_0_e1046);
        let noise_metadata_schedule_77_0_e1048: f64 = (noise_metadata_schedule_77_0_e1035 * noise_metadata_schedule_77_0_e1047);
        (noise_metadata_schedule_77_0_e1048,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_77_0_e1050;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_78_0_e1059,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_78_0_e1057: f64 = (w[4] - w[58]);
        (noise_metadata_schedule_78_0_e1057,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_78_0_e1059;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_79_0_e1068,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_79_0_e1066: f64 = (w[63] * w[63]);
        (noise_metadata_schedule_79_0_e1066,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_79_0_e1068;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_80_0_e1077,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_80_0_e1075: f64 = (w[64] * w[63]);
        (noise_metadata_schedule_80_0_e1075,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_80_0_e1077;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_81_0_e1094,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_81_0_e1084: f64 = (w[60] * w[63]);
        let noise_metadata_schedule_81_0_e1087: f64 = (params[12] * w[64]);
        let noise_metadata_schedule_81_0_e1088: f64 = (noise_metadata_schedule_81_0_e1084 + noise_metadata_schedule_81_0_e1087);
        let noise_metadata_schedule_81_0_e1091: f64 = (w[61] * w[65]);
        let noise_metadata_schedule_81_0_e1092: f64 = (noise_metadata_schedule_81_0_e1088 + noise_metadata_schedule_81_0_e1091);
        (noise_metadata_schedule_81_0_e1092,)
    } else {
        (w[71],)
    }
};
            w[71] = noise_metadata_schedule_81_0_e1094;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_82_0_e1104,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_82_0_e1101: f64 = (w[71]).tanh();
        let noise_metadata_schedule_82_0_e1102: f64 = (1.0 + noise_metadata_schedule_82_0_e1101);
        (noise_metadata_schedule_82_0_e1102,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_82_0_e1104;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_83_0_e1115,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_83_0_e1112: f64 = (params[15] * w[77]);
        let noise_metadata_schedule_83_0_e1113: f64 = (params[14] + noise_metadata_schedule_83_0_e1112);
        (noise_metadata_schedule_83_0_e1113,)
    } else {
        (w[72],)
    }
};
            w[72] = noise_metadata_schedule_83_0_e1115;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_84_0_e1126,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_84_0_e1123: f64 = (params[17] * w[75]);
        let noise_metadata_schedule_84_0_e1124: f64 = (params[16] + noise_metadata_schedule_84_0_e1123);
        (noise_metadata_schedule_84_0_e1124,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_84_0_e1126;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 145], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_85_0_e1154,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_85_0_e1133: f64 = (w[39] * w[75]);
        let noise_metadata_schedule_85_0_e1136: f64 = (1.0 + w[79]);
        let noise_metadata_schedule_85_0_e1137: f64 = (noise_metadata_schedule_85_0_e1133 * noise_metadata_schedule_85_0_e1136);
        let noise_metadata_schedule_85_0_e1141: f64 = (w[69] * w[5]);
        let noise_metadata_schedule_85_0_e1142: f64 = (1.0 + noise_metadata_schedule_85_0_e1141);
        let noise_metadata_schedule_85_0_e1147: f64 = (w[5] - w[53]);
        let noise_metadata_schedule_85_0_e1148: f64 = (params[23] * noise_metadata_schedule_85_0_e1147);
        let noise_metadata_schedule_85_0_e1149: f64 = { let limexp_arg = noise_metadata_schedule_85_0_e1148; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_85_0_e1150: f64 = (w[43] * noise_metadata_schedule_85_0_e1149);
        let noise_metadata_schedule_85_0_e1151: f64 = (noise_metadata_schedule_85_0_e1142 + noise_metadata_schedule_85_0_e1150);
        let noise_metadata_schedule_85_0_e1152: f64 = (noise_metadata_schedule_85_0_e1137 * noise_metadata_schedule_85_0_e1151);
        (noise_metadata_schedule_85_0_e1152,)
    } else {
        (w[73],)
    }
};
            w[73] = noise_metadata_schedule_85_0_e1154;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_86_0_e1165,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_86_0_e1162: f64 = (params[17] * w[77]);
        let noise_metadata_schedule_86_0_e1163: f64 = (params[16] + noise_metadata_schedule_86_0_e1162);
        (noise_metadata_schedule_86_0_e1163,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_86_0_e1165;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_87_0_e1175,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_87_0_e1172: f64 = (w[72] * w[5]);
        let noise_metadata_schedule_87_0_e1173: f64 = (noise_metadata_schedule_87_0_e1172).tanh();
        (noise_metadata_schedule_87_0_e1173,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_87_0_e1175;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_88_0_e1194,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_88_0_e1182: f64 = (w[39] * w[77]);
        let noise_metadata_schedule_88_0_e1185: f64 = (1.0 - w[80]);
        let noise_metadata_schedule_88_0_e1186: f64 = (noise_metadata_schedule_88_0_e1182 * noise_metadata_schedule_88_0_e1185);
        let noise_metadata_schedule_88_0_e1190: f64 = (w[67] * w[5]);
        let noise_metadata_schedule_88_0_e1191: f64 = (1.0 - noise_metadata_schedule_88_0_e1190);
        let noise_metadata_schedule_88_0_e1192: f64 = (noise_metadata_schedule_88_0_e1186 * noise_metadata_schedule_88_0_e1191);
        (noise_metadata_schedule_88_0_e1192,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_88_0_e1194;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_89_0_e1205,) = {
    if ((w[107] != 0.0) && (w[106] == 0.0)) {
        let noise_metadata_schedule_89_0_e1202: f64 = (w[73] - w[74]);
        let noise_metadata_schedule_89_0_e1203: f64 = (0.5 * noise_metadata_schedule_89_0_e1202);
        (noise_metadata_schedule_89_0_e1203,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_89_0_e1205;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_90_0_e1216,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_90_0_e1214: f64 = (w[3] - w[58]);
        (noise_metadata_schedule_90_0_e1214,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_90_0_e1216;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_91_0_e1227,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_91_0_e1225: f64 = (w[63] * w[63]);
        (noise_metadata_schedule_91_0_e1225,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_91_0_e1227;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_92_0_e1248,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_92_0_e1238: f64 = (params[12] * w[64]);
        let noise_metadata_schedule_92_0_e1239: f64 = (w[63] + noise_metadata_schedule_92_0_e1238);
        let noise_metadata_schedule_92_0_e1242: f64 = (w[61] * w[64]);
        let noise_metadata_schedule_92_0_e1244: f64 = (noise_metadata_schedule_92_0_e1242 * w[63]);
        let noise_metadata_schedule_92_0_e1245: f64 = (noise_metadata_schedule_92_0_e1239 + noise_metadata_schedule_92_0_e1244);
        let noise_metadata_schedule_92_0_e1246: f64 = (w[60] * noise_metadata_schedule_92_0_e1245);
        (noise_metadata_schedule_92_0_e1246,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_92_0_e1248;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_93_0_e1267,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_93_0_e1258: f64 = { let limexp_arg = w[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_93_0_e1260: f64 = (-w[17]);
        let noise_metadata_schedule_93_0_e1261: f64 = { let limexp_arg = noise_metadata_schedule_93_0_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_93_0_e1262: f64 = (noise_metadata_schedule_93_0_e1258 - noise_metadata_schedule_93_0_e1261);
        let noise_metadata_schedule_93_0_e1263: f64 = (0.5 * noise_metadata_schedule_93_0_e1262);
        let noise_metadata_schedule_93_0_e1264: f64 = (noise_metadata_schedule_93_0_e1263).tanh();
        let noise_metadata_schedule_93_0_e1265: f64 = (1.0 + noise_metadata_schedule_93_0_e1264);
        (noise_metadata_schedule_93_0_e1265,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_93_0_e1267;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_94_0_e1280,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_94_0_e1277: f64 = (params[15] * w[76]);
        let noise_metadata_schedule_94_0_e1278: f64 = (params[14] + noise_metadata_schedule_94_0_e1277);
        (noise_metadata_schedule_94_0_e1278,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_94_0_e1280;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_95_0_e1292,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_95_0_e1289: f64 = (w[1] * w[5]);
        let noise_metadata_schedule_95_0_e1290: f64 = (noise_metadata_schedule_95_0_e1289).tanh();
        (noise_metadata_schedule_95_0_e1290,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_95_0_e1292;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_96_0_e1305,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_96_0_e1302: f64 = (params[17] * w[76]);
        let noise_metadata_schedule_96_0_e1303: f64 = (params[16] + noise_metadata_schedule_96_0_e1302);
        (noise_metadata_schedule_96_0_e1303,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_96_0_e1305;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_97_0_e1333,) = {
    if ((w[108] != 0.0) && (!((w[106] != 0.0) || (w[107] != 0.0)))) {
        let noise_metadata_schedule_97_0_e1314: f64 = (w[39] * w[76]);
        let noise_metadata_schedule_97_0_e1316: f64 = (noise_metadata_schedule_97_0_e1314 * w[81]);
        let noise_metadata_schedule_97_0_e1320: f64 = (w[69] * w[5]);
        let noise_metadata_schedule_97_0_e1321: f64 = (1.0 + noise_metadata_schedule_97_0_e1320);
        let noise_metadata_schedule_97_0_e1326: f64 = (w[6] - w[53]);
        let noise_metadata_schedule_97_0_e1327: f64 = (params[23] * noise_metadata_schedule_97_0_e1326);
        let noise_metadata_schedule_97_0_e1328: f64 = { let limexp_arg = noise_metadata_schedule_97_0_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_97_0_e1329: f64 = (w[43] * noise_metadata_schedule_97_0_e1328);
        let noise_metadata_schedule_97_0_e1330: f64 = (noise_metadata_schedule_97_0_e1321 + noise_metadata_schedule_97_0_e1329);
        let noise_metadata_schedule_97_0_e1331: f64 = (noise_metadata_schedule_97_0_e1316 * noise_metadata_schedule_97_0_e1330);
        (noise_metadata_schedule_97_0_e1331,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_97_0_e1333;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_98_0_e1346,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_98_0_e1344: f64 = (w[3] - w[58]);
        (noise_metadata_schedule_98_0_e1344,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_98_0_e1346;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_99_0_e1359,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_99_0_e1357: f64 = (w[63] * w[63]);
        (noise_metadata_schedule_99_0_e1357,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_99_0_e1359;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_100_0_e1382,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_100_0_e1372: f64 = (params[12] * w[64]);
        let noise_metadata_schedule_100_0_e1373: f64 = (w[63] + noise_metadata_schedule_100_0_e1372);
        let noise_metadata_schedule_100_0_e1376: f64 = (w[61] * w[64]);
        let noise_metadata_schedule_100_0_e1378: f64 = (noise_metadata_schedule_100_0_e1376 * w[63]);
        let noise_metadata_schedule_100_0_e1379: f64 = (noise_metadata_schedule_100_0_e1373 + noise_metadata_schedule_100_0_e1378);
        let noise_metadata_schedule_100_0_e1380: f64 = (w[60] * noise_metadata_schedule_100_0_e1379);
        (noise_metadata_schedule_100_0_e1380,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_100_0_e1382;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_101_0_e1395,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_101_0_e1393: f64 = (w[4] - w[58]);
        (noise_metadata_schedule_101_0_e1393,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_101_0_e1395;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_102_0_e1408,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_102_0_e1406: f64 = (w[65] * w[65]);
        (noise_metadata_schedule_102_0_e1406,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_102_0_e1408;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_103_0_e1431,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_103_0_e1421: f64 = (params[12] * w[66]);
        let noise_metadata_schedule_103_0_e1422: f64 = (w[65] + noise_metadata_schedule_103_0_e1421);
        let noise_metadata_schedule_103_0_e1425: f64 = (w[61] * w[65]);
        let noise_metadata_schedule_103_0_e1427: f64 = (noise_metadata_schedule_103_0_e1425 * w[66]);
        let noise_metadata_schedule_103_0_e1428: f64 = (noise_metadata_schedule_103_0_e1422 + noise_metadata_schedule_103_0_e1427);
        let noise_metadata_schedule_103_0_e1429: f64 = (w[60] * noise_metadata_schedule_103_0_e1428);
        (noise_metadata_schedule_103_0_e1429,)
    } else {
        (w[71],)
    }
};
            w[71] = noise_metadata_schedule_103_0_e1431;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_104_0_e1452,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_104_0_e1443: f64 = { let limexp_arg = w[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_104_0_e1445: f64 = (-w[17]);
        let noise_metadata_schedule_104_0_e1446: f64 = { let limexp_arg = noise_metadata_schedule_104_0_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_104_0_e1447: f64 = (noise_metadata_schedule_104_0_e1443 - noise_metadata_schedule_104_0_e1446);
        let noise_metadata_schedule_104_0_e1448: f64 = (0.5 * noise_metadata_schedule_104_0_e1447);
        let noise_metadata_schedule_104_0_e1449: f64 = (noise_metadata_schedule_104_0_e1448).tanh();
        let noise_metadata_schedule_104_0_e1450: f64 = (1.0 + noise_metadata_schedule_104_0_e1449);
        (noise_metadata_schedule_104_0_e1450,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_104_0_e1452;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_105_0_e1473,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_105_0_e1464: f64 = { let limexp_arg = w[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_105_0_e1466: f64 = (-w[71]);
        let noise_metadata_schedule_105_0_e1467: f64 = { let limexp_arg = noise_metadata_schedule_105_0_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_105_0_e1468: f64 = (noise_metadata_schedule_105_0_e1464 - noise_metadata_schedule_105_0_e1467);
        let noise_metadata_schedule_105_0_e1469: f64 = (0.5 * noise_metadata_schedule_105_0_e1468);
        let noise_metadata_schedule_105_0_e1470: f64 = (noise_metadata_schedule_105_0_e1469).tanh();
        let noise_metadata_schedule_105_0_e1471: f64 = (1.0 + noise_metadata_schedule_105_0_e1470);
        (noise_metadata_schedule_105_0_e1471,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_105_0_e1473;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_106_0_e1488,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_106_0_e1485: f64 = (params[15] * w[76]);
        let noise_metadata_schedule_106_0_e1486: f64 = (params[14] + noise_metadata_schedule_106_0_e1485);
        (noise_metadata_schedule_106_0_e1486,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_106_0_e1488;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_107_0_e1503,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_107_0_e1500: f64 = (params[15] * w[78]);
        let noise_metadata_schedule_107_0_e1501: f64 = (params[14] + noise_metadata_schedule_107_0_e1500);
        (noise_metadata_schedule_107_0_e1501,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_107_0_e1503;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_108_0_e1517,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_108_0_e1514: f64 = (w[1] * w[5]);
        let noise_metadata_schedule_108_0_e1515: f64 = (noise_metadata_schedule_108_0_e1514).tanh();
        (noise_metadata_schedule_108_0_e1515,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_108_0_e1517;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_109_0_e1531,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_109_0_e1528: f64 = (w[2] * w[5]);
        let noise_metadata_schedule_109_0_e1529: f64 = (noise_metadata_schedule_109_0_e1528).tanh();
        (noise_metadata_schedule_109_0_e1529,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_109_0_e1531;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_110_0_e1546,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_110_0_e1543: f64 = (params[17] * w[78]);
        let noise_metadata_schedule_110_0_e1544: f64 = (params[16] + noise_metadata_schedule_110_0_e1543);
        (noise_metadata_schedule_110_0_e1544,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_110_0_e1546;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_111_0_e1561,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_111_0_e1558: f64 = (params[17] * w[76]);
        let noise_metadata_schedule_111_0_e1559: f64 = (params[16] + noise_metadata_schedule_111_0_e1558);
        (noise_metadata_schedule_111_0_e1559,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_111_0_e1561;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_112_0_e1593,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_112_0_e1572: f64 = (w[39] * w[76]);
        let noise_metadata_schedule_112_0_e1575: f64 = (1.0 + w[81]);
        let noise_metadata_schedule_112_0_e1576: f64 = (noise_metadata_schedule_112_0_e1572 * noise_metadata_schedule_112_0_e1575);
        let noise_metadata_schedule_112_0_e1580: f64 = (w[70] * w[5]);
        let noise_metadata_schedule_112_0_e1581: f64 = (1.0 + noise_metadata_schedule_112_0_e1580);
        let noise_metadata_schedule_112_0_e1586: f64 = (w[5] - w[53]);
        let noise_metadata_schedule_112_0_e1587: f64 = (params[23] * noise_metadata_schedule_112_0_e1586);
        let noise_metadata_schedule_112_0_e1588: f64 = { let limexp_arg = noise_metadata_schedule_112_0_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_112_0_e1589: f64 = (w[43] * noise_metadata_schedule_112_0_e1588);
        let noise_metadata_schedule_112_0_e1590: f64 = (noise_metadata_schedule_112_0_e1581 + noise_metadata_schedule_112_0_e1589);
        let noise_metadata_schedule_112_0_e1591: f64 = (noise_metadata_schedule_112_0_e1576 * noise_metadata_schedule_112_0_e1590);
        (noise_metadata_schedule_112_0_e1591,)
    } else {
        (w[73],)
    }
};
            w[73] = noise_metadata_schedule_112_0_e1593;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_113_0_e1616,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_113_0_e1604: f64 = (w[39] * w[78]);
        let noise_metadata_schedule_113_0_e1607: f64 = (1.0 - w[82]);
        let noise_metadata_schedule_113_0_e1608: f64 = (noise_metadata_schedule_113_0_e1604 * noise_metadata_schedule_113_0_e1607);
        let noise_metadata_schedule_113_0_e1612: f64 = (w[68] * w[5]);
        let noise_metadata_schedule_113_0_e1613: f64 = (1.0 - noise_metadata_schedule_113_0_e1612);
        let noise_metadata_schedule_113_0_e1614: f64 = (noise_metadata_schedule_113_0_e1608 * noise_metadata_schedule_113_0_e1613);
        (noise_metadata_schedule_113_0_e1614,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_113_0_e1616;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_114_0_e1631,) = {
    if ((w[109] != 0.0) && (!(((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)))) {
        let noise_metadata_schedule_114_0_e1628: f64 = (w[73] - w[74]);
        let noise_metadata_schedule_114_0_e1629: f64 = (0.5 * noise_metadata_schedule_114_0_e1628);
        (noise_metadata_schedule_114_0_e1629,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_114_0_e1631;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_115_0_e1648,) = {
    if ((w[110] != 0.0) && (!((((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)) || (w[109] != 0.0)))) {
        let noise_metadata_schedule_115_0_e1645: f64 = (params[17] * w[75]);
        let noise_metadata_schedule_115_0_e1646: f64 = (params[16] + noise_metadata_schedule_115_0_e1645);
        (noise_metadata_schedule_115_0_e1646,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_115_0_e1648;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_116_0_e1665,) = {
    if ((w[110] != 0.0) && (!((((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)) || (w[109] != 0.0)))) {
        let noise_metadata_schedule_116_0_e1662: f64 = (params[15] * w[76]);
        let noise_metadata_schedule_116_0_e1663: f64 = (params[14] + noise_metadata_schedule_116_0_e1662);
        (noise_metadata_schedule_116_0_e1663,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_116_0_e1665;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_117_0_e1681,) = {
    if ((w[110] != 0.0) && (!((((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)) || (w[109] != 0.0)))) {
        let noise_metadata_schedule_117_0_e1678: f64 = (w[1] * w[5]);
        let noise_metadata_schedule_117_0_e1679: f64 = (noise_metadata_schedule_117_0_e1678).tanh();
        (noise_metadata_schedule_117_0_e1679,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_117_0_e1681;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_118_0_e1697,) = {
    if ((w[110] != 0.0) && (!((((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)) || (w[109] != 0.0)))) {
        let noise_metadata_schedule_118_0_e1694: f64 = (w[1] * w[11]);
        let noise_metadata_schedule_118_0_e1695: f64 = (noise_metadata_schedule_118_0_e1694).tanh();
        (noise_metadata_schedule_118_0_e1695,)
    } else {
        (w[83],)
    }
};
            w[83] = noise_metadata_schedule_118_0_e1697;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_119_0_e1737,) = {
    if ((w[110] != 0.0) && (!((((w[106] != 0.0) || (w[107] != 0.0)) || (w[108] != 0.0)) || (w[109] != 0.0)))) {
        let noise_metadata_schedule_119_0_e1710: f64 = (w[39] * w[75]);
        let noise_metadata_schedule_119_0_e1714: f64 = (params[65] * w[83]);
        let noise_metadata_schedule_119_0_e1715: f64 = (w[81] + noise_metadata_schedule_119_0_e1714);
        let noise_metadata_schedule_119_0_e1716: f64 = (noise_metadata_schedule_119_0_e1710 * noise_metadata_schedule_119_0_e1715);
        let noise_metadata_schedule_119_0_e1722: f64 = (params[65] * w[11]);
        let noise_metadata_schedule_119_0_e1723: f64 = (w[5] + noise_metadata_schedule_119_0_e1722);
        let noise_metadata_schedule_119_0_e1724: f64 = (w[69] * noise_metadata_schedule_119_0_e1723);
        let noise_metadata_schedule_119_0_e1725: f64 = (1.0 + noise_metadata_schedule_119_0_e1724);
        let noise_metadata_schedule_119_0_e1730: f64 = (w[5] - w[53]);
        let noise_metadata_schedule_119_0_e1731: f64 = (params[23] * noise_metadata_schedule_119_0_e1730);
        let noise_metadata_schedule_119_0_e1732: f64 = { let limexp_arg = noise_metadata_schedule_119_0_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_119_0_e1733: f64 = (w[43] * noise_metadata_schedule_119_0_e1732);
        let noise_metadata_schedule_119_0_e1734: f64 = (noise_metadata_schedule_119_0_e1725 + noise_metadata_schedule_119_0_e1733);
        let noise_metadata_schedule_119_0_e1735: f64 = (noise_metadata_schedule_119_0_e1716 * noise_metadata_schedule_119_0_e1734);
        (noise_metadata_schedule_119_0_e1735,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_119_0_e1737;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_120_0_e1748: f64 = if (((params[4] == 0.0) || (params[4] == 1.0)) || (params[4] == 4.0)) { 1.0 } else { 0.0 };
            w[111] = noise_metadata_schedule_120_0_e1748;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_122_0_e1766,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_122_0_e1763: f64 = (params[48] * w[75]);
        let noise_metadata_schedule_122_0_e1764: f64 = (params[47] + noise_metadata_schedule_122_0_e1763);
        (noise_metadata_schedule_122_0_e1764,)
    } else {
        (w[41],)
    }
};
            w[41] = noise_metadata_schedule_122_0_e1766;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 145], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_123_0_e1774,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_123_0_e1771: f64 = (params[48] * w[75]);
        let noise_metadata_schedule_123_0_e1772: f64 = (params[50] + noise_metadata_schedule_123_0_e1771);
        (noise_metadata_schedule_123_0_e1772,)
    } else {
        (w[42],)
    }
};
            w[42] = noise_metadata_schedule_123_0_e1774;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_125_0_e1794,) = {
    if (w[111] == 0.0) {
        let noise_metadata_schedule_125_0_e1791: f64 = (params[48] * w[76]);
        let noise_metadata_schedule_125_0_e1792: f64 = (params[47] + noise_metadata_schedule_125_0_e1791);
        (noise_metadata_schedule_125_0_e1792,)
    } else {
        (w[41],)
    }
};
            w[41] = noise_metadata_schedule_125_0_e1794;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_126_0_e1803,) = {
    if (w[111] == 0.0) {
        let noise_metadata_schedule_126_0_e1800: f64 = (params[48] * w[76]);
        let noise_metadata_schedule_126_0_e1801: f64 = (params[50] + noise_metadata_schedule_126_0_e1800);
        (noise_metadata_schedule_126_0_e1801,)
    } else {
        (w[42],)
    }
};
            w[42] = noise_metadata_schedule_126_0_e1803;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_127_0_e1808: f64 = (w[16]).abs();
            let noise_metadata_schedule_127_0_e1809: f64 = (params[76] * noise_metadata_schedule_127_0_e1808);
            let noise_metadata_schedule_127_0_e1810: f64 = (1.0 + noise_metadata_schedule_127_0_e1809);
            let noise_metadata_schedule_127_0_e1811: f64 = (w[42] * noise_metadata_schedule_127_0_e1810);
            w[50] = noise_metadata_schedule_127_0_e1811;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_128_0_e1816: f64 = (w[16]).abs();
            let noise_metadata_schedule_128_0_e1817: f64 = (params[76] * noise_metadata_schedule_128_0_e1816);
            let noise_metadata_schedule_128_0_e1818: f64 = (1.0 + noise_metadata_schedule_128_0_e1817);
            let noise_metadata_schedule_128_0_e1819: f64 = (w[41] * noise_metadata_schedule_128_0_e1818);
            w[49] = noise_metadata_schedule_128_0_e1819;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_130_0_e1830: f64 = if params[5] == 0.0 { 1.0 } else { 0.0 };
            w[112] = noise_metadata_schedule_130_0_e1830;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_131_0_e1841,) = {
    if (w[112] != 0.0) {
        let noise_metadata_schedule_131_0_e1834: f64 = (-1.0);
        let noise_metadata_schedule_131_0_e1836: f64 = (noise_metadata_schedule_131_0_e1834 * w[57]);
        let noise_metadata_schedule_131_0_e1837: f64 = (noise_metadata_schedule_131_0_e1836).tanh();
        let noise_metadata_schedule_131_0_e1838: f64 = (w[19] * noise_metadata_schedule_131_0_e1837);
        let noise_metadata_schedule_131_0_e1839: f64 = { let limexp_arg = noise_metadata_schedule_131_0_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_131_0_e1839,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_131_0_e1841;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_132_0_e1849,) = {
    if (w[112] != 0.0) {
        let noise_metadata_schedule_132_0_e1846: f64 = (w[96] - w[57]);
        let noise_metadata_schedule_132_0_e1847: f64 = noise_metadata_schedule_132_0_e1846;
        (noise_metadata_schedule_132_0_e1847,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_132_0_e1849;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_133_0_e1856,) = {
    if (w[112] != 0.0) {
        let noise_metadata_schedule_133_0_e1852: f64 = (-w[96]);
        let noise_metadata_schedule_133_0_e1854: f64 = (noise_metadata_schedule_133_0_e1852 - params[83]);
        (noise_metadata_schedule_133_0_e1854,)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_133_0_e1856;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_134_0_e1864,) = {
    if (w[112] != 0.0) {
        let noise_metadata_schedule_134_0_e1861: f64 = (w[97] - w[57]);
        let noise_metadata_schedule_134_0_e1862: f64 = noise_metadata_schedule_134_0_e1861;
        (noise_metadata_schedule_134_0_e1862,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_134_0_e1864;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_135_0_e1871,) = {
    if (w[112] != 0.0) {
        let noise_metadata_schedule_135_0_e1867: f64 = (-w[97]);
        let noise_metadata_schedule_135_0_e1869: f64 = (noise_metadata_schedule_135_0_e1867 - params[84]);
        (noise_metadata_schedule_135_0_e1869,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_135_0_e1871;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_136_0_e1880,) = {
    if (w[112] == 0.0) {
        let noise_metadata_schedule_136_0_e1875: f64 = (-w[19]);
        let noise_metadata_schedule_136_0_e1877: f64 = (noise_metadata_schedule_136_0_e1875 * w[57]);
        let noise_metadata_schedule_136_0_e1878: f64 = { let limexp_arg = noise_metadata_schedule_136_0_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_136_0_e1878,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_136_0_e1880;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_137_0_e1889,) = {
    if (w[112] == 0.0) {
        let noise_metadata_schedule_137_0_e1884: f64 = (-params[85]);
        let noise_metadata_schedule_137_0_e1886: f64 = (noise_metadata_schedule_137_0_e1884 * params[83]);
        let noise_metadata_schedule_137_0_e1887: f64 = { let limexp_arg = noise_metadata_schedule_137_0_e1886; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_137_0_e1887,)
    } else {
        (w[24],)
    }
};
            w[24] = noise_metadata_schedule_137_0_e1889;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_138_0_e1898,) = {
    if (w[112] == 0.0) {
        let noise_metadata_schedule_138_0_e1893: f64 = (-params[85]);
        let noise_metadata_schedule_138_0_e1895: f64 = (noise_metadata_schedule_138_0_e1893 * params[84]);
        let noise_metadata_schedule_138_0_e1896: f64 = { let limexp_arg = noise_metadata_schedule_138_0_e1895; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_138_0_e1896,)
    } else {
        (w[25],)
    }
};
            w[25] = noise_metadata_schedule_138_0_e1898;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_139_0_e1901: f64 = if params[5] == 1.0 { 1.0 } else { 0.0 };
            w[113] = noise_metadata_schedule_139_0_e1901;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_140_0_e1911,) = {
    if ((w[112] == 0.0) && (w[113] != 0.0)) {
        let noise_metadata_schedule_140_0_e1908: f64 = (w[96] - w[57]);
        let noise_metadata_schedule_140_0_e1909: f64 = (noise_metadata_schedule_140_0_e1908).tanh();
        (noise_metadata_schedule_140_0_e1909,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_140_0_e1911;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_141_0_e1921,) = {
    if ((w[112] == 0.0) && (w[113] != 0.0)) {
        let noise_metadata_schedule_141_0_e1918: f64 = (w[97] - w[57]);
        let noise_metadata_schedule_141_0_e1919: f64 = (noise_metadata_schedule_141_0_e1918).tanh();
        (noise_metadata_schedule_141_0_e1919,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_141_0_e1921;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_142_0_e1931,) = {
    if ((w[112] == 0.0) && (w[113] == 0.0)) {
        let noise_metadata_schedule_142_0_e1929: f64 = (w[96] - w[57]);
        (noise_metadata_schedule_142_0_e1929,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_142_0_e1931;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_143_0_e1941,) = {
    if ((w[112] == 0.0) && (w[113] == 0.0)) {
        let noise_metadata_schedule_143_0_e1939: f64 = (w[97] - w[57]);
        (noise_metadata_schedule_143_0_e1939,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_143_0_e1941;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_144_0_e1949,) = {
    if (w[112] == 0.0) {
        let noise_metadata_schedule_144_0_e1945: f64 = (-w[96]);
        let noise_metadata_schedule_144_0_e1947: f64 = (noise_metadata_schedule_144_0_e1945 - params[83]);
        (noise_metadata_schedule_144_0_e1947,)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_144_0_e1949;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_145_0_e1957,) = {
    if (w[112] == 0.0) {
        let noise_metadata_schedule_145_0_e1953: f64 = (-w[97]);
        let noise_metadata_schedule_145_0_e1955: f64 = (noise_metadata_schedule_145_0_e1953 - params[84]);
        (noise_metadata_schedule_145_0_e1955,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_145_0_e1957;
        }
        if (active[0] & 0x5000) != 0 {
            let noise_metadata_schedule_146_0_e1960: f64 = (params[85] * w[21]);
            let noise_metadata_schedule_146_0_e1961: f64 = { let limexp_arg = noise_metadata_schedule_146_0_e1960; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_146_0_e1963: f64 = (noise_metadata_schedule_146_0_e1961 - w[24]);
            w[8] = noise_metadata_schedule_146_0_e1963;
        }
        if (active[0] & 0x5000) != 0 {
            let noise_metadata_schedule_147_0_e1967: f64 = (w[19] * w[20]);
            let noise_metadata_schedule_147_0_e1968: f64 = { let limexp_arg = noise_metadata_schedule_147_0_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_147_0_e1971: f64 = (0.001 * params[82]);
            let noise_metadata_schedule_147_0_e1973: f64 = (noise_metadata_schedule_147_0_e1971 * w[8]);
            let noise_metadata_schedule_147_0_e1974: f64 = (noise_metadata_schedule_147_0_e1968 - noise_metadata_schedule_147_0_e1973);
            let noise_metadata_schedule_147_0_e1976: f64 = (noise_metadata_schedule_147_0_e1974 - w[63]);
            let noise_metadata_schedule_147_0_e1977: f64 = (params[42] * noise_metadata_schedule_147_0_e1976);
            w[7] = noise_metadata_schedule_147_0_e1977;
        }
        if (active[0] & 0xa030) != 0 {
            let noise_metadata_schedule_148_0_e1980: f64 = (params[85] * w[23]);
            let noise_metadata_schedule_148_0_e1981: f64 = { let limexp_arg = noise_metadata_schedule_148_0_e1980; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_148_0_e1983: f64 = (noise_metadata_schedule_148_0_e1981 - w[25]);
            w[10] = noise_metadata_schedule_148_0_e1983;
        }
        if (active[0] & 0xa030) != 0 {
            let noise_metadata_schedule_149_0_e1987: f64 = (w[19] * w[22]);
            let noise_metadata_schedule_149_0_e1988: f64 = { let limexp_arg = noise_metadata_schedule_149_0_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_149_0_e1991: f64 = (0.001 * params[82]);
            let noise_metadata_schedule_149_0_e1993: f64 = (noise_metadata_schedule_149_0_e1991 * w[10]);
            let noise_metadata_schedule_149_0_e1994: f64 = (noise_metadata_schedule_149_0_e1988 - noise_metadata_schedule_149_0_e1993);
            let noise_metadata_schedule_149_0_e1996: f64 = (noise_metadata_schedule_149_0_e1994 - w[63]);
            let noise_metadata_schedule_149_0_e1997: f64 = (params[42] * noise_metadata_schedule_149_0_e1996);
            w[9] = noise_metadata_schedule_149_0_e1997;
        }
        if (active[0] & 0x700) != 0 {
            let noise_metadata_schedule_218_0_e2869: f64 = 0.0;
            w[99] = noise_metadata_schedule_218_0_e2869;
        }
        if (active[0] & 0x700) != 0 {
            let noise_metadata_schedule_219_0_e2874: f64 = (w[99] * params[50]);
            let noise_metadata_schedule_219_0_e2875: f64 = (1.0 + noise_metadata_schedule_219_0_e2874);
            let noise_metadata_schedule_219_0_e2876: f64 = (w[99] / noise_metadata_schedule_219_0_e2875);
            w[99] = noise_metadata_schedule_219_0_e2876;
        }
        if (active[0] & 0x7f0) != 0 {
            let noise_metadata_schedule_221_0_e2882: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[128] = noise_metadata_schedule_221_0_e2882;
        }
        if (active[0] & 0x7c0) != 0 {
            let noise_metadata_schedule_222_0_e2885: f64 = if params[7] == 1.0 { 1.0 } else { 0.0 };
            w[129] = noise_metadata_schedule_222_0_e2885;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_224_0_e2896,) = {
    if (w[128] != 0.0) {
        let noise_metadata_schedule_224_0_e2891: f64 = (w[18]).abs();
        let noise_metadata_schedule_224_0_e2893: f64 = (w[9]).abs();
        let noise_metadata_schedule_224_0_e2894: f64 = (noise_metadata_schedule_224_0_e2891 + noise_metadata_schedule_224_0_e2893);
        (noise_metadata_schedule_224_0_e2894,)
    } else {
        (w[132],)
    }
};
            w[132] = noise_metadata_schedule_224_0_e2896;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_225_0_e2917,) = {
    if (w[128] != 0.0) {
        let noise_metadata_schedule_225_0_e2900: f64 = (params[93] + 273.15);
        let noise_metadata_schedule_225_0_e2904: f64 = (params[95] * w[75]);
        let noise_metadata_schedule_225_0_e2906: f64 = (w[79]).abs();
        let noise_metadata_schedule_225_0_e2907: f64 = (noise_metadata_schedule_225_0_e2904 * noise_metadata_schedule_225_0_e2906);
        let noise_metadata_schedule_225_0_e2911: f64 = (params[16] * w[5]);
        let noise_metadata_schedule_225_0_e2912: f64 = (1.0 + noise_metadata_schedule_225_0_e2911);
        let noise_metadata_schedule_225_0_e2913: f64 = (noise_metadata_schedule_225_0_e2907 * noise_metadata_schedule_225_0_e2912);
        let noise_metadata_schedule_225_0_e2914: f64 = (1.0 + noise_metadata_schedule_225_0_e2913);
        let noise_metadata_schedule_225_0_e2915: f64 = (noise_metadata_schedule_225_0_e2900 * noise_metadata_schedule_225_0_e2914);
        (noise_metadata_schedule_225_0_e2915,)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_225_0_e2917;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_226_0_e2941,) = {
    if (w[128] != 0.0) {
        let noise_metadata_schedule_226_0_e2921: f64 = (params[99] * 4.0);
        let noise_metadata_schedule_226_0_e2923: f64 = (noise_metadata_schedule_226_0_e2921 * 1.3806503e-23);
        let noise_metadata_schedule_226_0_e2925: f64 = (noise_metadata_schedule_226_0_e2923 * w[15]);
        let noise_metadata_schedule_226_0_e2928: f64 = (w[133] / w[15]);
        let noise_metadata_schedule_226_0_e2930: f64 = (noise_metadata_schedule_226_0_e2928 * w[132]);
        let noise_metadata_schedule_226_0_e2933: f64 = (params[94] * w[132]);
        let noise_metadata_schedule_226_0_e2935: f64 = (noise_metadata_schedule_226_0_e2933 * w[132]);
        let noise_metadata_schedule_226_0_e2936: f64 = (noise_metadata_schedule_226_0_e2930 + noise_metadata_schedule_226_0_e2935);
        let noise_metadata_schedule_226_0_e2937: f64 = (noise_metadata_schedule_226_0_e2936).abs();
        let noise_metadata_schedule_226_0_e2938: f64 = (noise_metadata_schedule_226_0_e2937).sqrt();
        let noise_metadata_schedule_226_0_e2939: f64 = (noise_metadata_schedule_226_0_e2925 * noise_metadata_schedule_226_0_e2938);
        (noise_metadata_schedule_226_0_e2939,)
    } else {
        (w[131],)
    }
};
            w[131] = noise_metadata_schedule_226_0_e2941;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_227_0_e2958,) = {
    if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) {
        let noise_metadata_schedule_227_0_e2950: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_227_0_e2952: f64 = (noise_metadata_schedule_227_0_e2950 * w[15]);
        let noise_metadata_schedule_227_0_e2954: f64 = (noise_metadata_schedule_227_0_e2952 * w[99]);
        let noise_metadata_schedule_227_0_e2956: f64 = (noise_metadata_schedule_227_0_e2954 * params[87]);
        (noise_metadata_schedule_227_0_e2956,)
    } else {
        (w[134],)
    }
};
            w[134] = noise_metadata_schedule_227_0_e2958;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_228_0_e2961: f64 = if w[99] > 0.0 { 1.0 } else { 0.0 };
            w[136] = noise_metadata_schedule_228_0_e2961;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_229_0_e2984,) = {
    if ((((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) && (w[136] != 0.0)) {
        let noise_metadata_schedule_229_0_e2972: f64 = (w[44] * w[44]);
        let noise_metadata_schedule_229_0_e2974: f64 = (noise_metadata_schedule_229_0_e2972 * 4.0);
        let noise_metadata_schedule_229_0_e2976: f64 = (noise_metadata_schedule_229_0_e2974 * 1.3806503e-23);
        let noise_metadata_schedule_229_0_e2978: f64 = (noise_metadata_schedule_229_0_e2976 * w[15]);
        let noise_metadata_schedule_229_0_e2980: f64 = (noise_metadata_schedule_229_0_e2978 * params[86]);
        let noise_metadata_schedule_229_0_e2982: f64 = (noise_metadata_schedule_229_0_e2980 / w[99]);
        (noise_metadata_schedule_229_0_e2982,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_229_0_e2984;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_230_0_e2996,) = {
    if ((((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) && (w[136] == 0.0)) {
        (0.0,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_230_0_e2996;
        }
        if (active[0] & 0xc0) != 0 {
            let (noise_metadata_schedule_231_0_e3018,) = {
    if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) {
        let noise_metadata_schedule_231_0_e3005: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_231_0_e3007: f64 = (noise_metadata_schedule_231_0_e3005 * w[15]);
        let noise_metadata_schedule_231_0_e3009: f64 = (noise_metadata_schedule_231_0_e3007 * params[88]);
        let noise_metadata_schedule_231_0_e3011: f64 = (noise_metadata_schedule_231_0_e3009 * w[44]);
        let noise_metadata_schedule_231_0_e3014: f64 = (params[87] * params[86]);
        let noise_metadata_schedule_231_0_e3015: f64 = (noise_metadata_schedule_231_0_e3014).sqrt();
        let noise_metadata_schedule_231_0_e3016: f64 = (noise_metadata_schedule_231_0_e3011 * noise_metadata_schedule_231_0_e3015);
        (noise_metadata_schedule_231_0_e3016,)
    } else {
        (w[140],)
    }
};
            w[140] = noise_metadata_schedule_231_0_e3018;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_235_0_e3074,) = {
    if (((w[129] != 0.0) && (w[128] == 0.0)) && (params[0] != 0.0)) {
        let noise_metadata_schedule_235_0_e3064: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_235_0_e3066: f64 = (noise_metadata_schedule_235_0_e3064 * w[15]);
        let noise_metadata_schedule_235_0_e3068: f64 = (noise_metadata_schedule_235_0_e3066 * w[99]);
        let noise_metadata_schedule_235_0_e3070: f64 = (noise_metadata_schedule_235_0_e3068 * params[87]);
        let noise_metadata_schedule_235_0_e3072: f64 = (noise_metadata_schedule_235_0_e3070 * params[89]);
        (noise_metadata_schedule_235_0_e3072,)
    } else {
        (w[141],)
    }
};
            w[141] = noise_metadata_schedule_235_0_e3074;
        }
    }
}

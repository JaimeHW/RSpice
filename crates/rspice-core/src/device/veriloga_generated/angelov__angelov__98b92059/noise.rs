#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 26, is_current: false, branch_ordinal: Some(7), pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 30, is_current: false, branch_ordinal: Some(11), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_D_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 35, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 125];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            let noise_0_activation_e178: f64 = if ((w[100] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_0_activation_e178 != 0.0
        };
        let noise_source_1_active = {
            let noise_1_activation_e221: f64 = if ((w[102] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_1_activation_e221 != 0.0
        };
        let noise_source_2_active = {
            let noise_2_activation_e259: f64 = if ((w[104] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_2_activation_e259 != 0.0
        };
        let noise_source_3_active = {
            let noise_3_activation_e295: f64 = if ((w[105] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_3_activation_e295 != 0.0
        };
        let noise_source_4_active = {
            let noise_4_activation_e327: f64 = if ((w[107] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_4_activation_e327 != 0.0
        };
        let noise_source_5_active = {
            let noise_5_activation_e335: f64 = if ((w[107] != 0.0) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_5_activation_e335 != 0.0
        };
        let noise_source_6_active = {
            let noise_6_activation_e349: f64 = if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_6_activation_e349 != 0.0
        };
        let noise_source_7_active = {
            let noise_7_activation_e369: f64 = if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_7_activation_e369 != 0.0
        };
        let noise_source_8_active = {
            let noise_8_activation_e426: f64 = if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_8_activation_e426 != 0.0
        };
        let noise_source_9_active = {
            let noise_9_activation_e437: f64 = if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_9_activation_e437 != 0.0
        };
        let noise_source_10_active = {
            let noise_10_activation_e449: f64 = if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) { 1.0 } else { 0.0 };
            noise_10_activation_e449 != 0.0
        };
        let noise_source_11_active = {
            let noise_11_activation_e463: f64 = if ((((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) && (w[122] != 0.0)) { 1.0 } else { 0.0 };
            noise_11_activation_e463 != 0.0
        };
        let noise_source_12_active = {
            params[0] != 0.0
        };
        let noise_source_13_active = {
            params[0] != 0.0
        };
        let noise_source_14_active = {
            let noise_14_activation_e500: f64 = if ((params[0] != 0.0) && (w[123] != 0.0)) { 1.0 } else { 0.0 };
            noise_14_activation_e500 != 0.0
        };
        let noise_source_15_active = {
            let noise_15_activation_e514: f64 = if ((params[0] != 0.0) && (w[123] != 0.0)) { 1.0 } else { 0.0 };
            noise_15_activation_e514 != 0.0
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
            let noise_0_psd_e2273: f64 = 1.0;
            let noise_0_psd_e181: f64 = (4.0 * 1.3806503e-23);
            let noise_0_psd_e183: f64 = (noise_0_psd_e181 * w[11]);
            let noise_0_psd_e185: f64 = (noise_0_psd_e183 * params[47]);
            let noise_0_psd_e2274: f64 = (noise_0_psd_e2273 * noise_0_psd_e185);
            let psd = noise_0_psd_e2274;
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
            let noise_1_psd_e2276: f64 = 1.0;
            let noise_1_psd_e224: f64 = (4.0 * 1.3806503e-23);
            let noise_1_psd_e226: f64 = (noise_1_psd_e224 * w[11]);
            let noise_1_psd_e228: f64 = (noise_1_psd_e226 * params[42]);
            let noise_1_psd_e2277: f64 = (noise_1_psd_e2276 * noise_1_psd_e228);
            let psd = noise_1_psd_e2277;
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
            let noise_2_psd_e2279: f64 = 1.0;
            let noise_2_psd_e262: f64 = (4.0 * 1.3806503e-23);
            let noise_2_psd_e264: f64 = (noise_2_psd_e262 * w[11]);
            let noise_2_psd_e266: f64 = (noise_2_psd_e264 * w[36]);
            let noise_2_psd_e2280: f64 = (noise_2_psd_e2279 * noise_2_psd_e266);
            let psd = noise_2_psd_e2280;
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
            let noise_3_psd_e2282: f64 = 1.0;
            let noise_3_psd_e298: f64 = (4.0 * 1.3806503e-23);
            let noise_3_psd_e300: f64 = (noise_3_psd_e298 * w[11]);
            let noise_3_psd_e302: f64 = (noise_3_psd_e300 * w[28]);
            let noise_3_psd_e2283: f64 = (noise_3_psd_e2282 * noise_3_psd_e302);
            let psd = noise_3_psd_e2283;
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
            let noise_4_psd_e2285: f64 = 1.0;
            let noise_4_psd_e2286: f64 = (noise_4_psd_e2285 * w[110]);
            let psd = noise_4_psd_e2286;
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
            let noise_5_psd_e2288: f64 = 1.0;
            let noise_5_psd_e338: f64 = (w[110] * params[81]);
            let noise_5_psd_e2289: f64 = (noise_5_psd_e2288 * noise_5_psd_e338);
            let psd = noise_5_psd_e2289;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = Some(params[83]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[6] {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_6_psd_e2291: f64 = 1.0;
            let noise_6_psd_e2292: f64 = (noise_6_psd_e2291 * w[120]);
            let psd = noise_6_psd_e2292;
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
            let noise_7_psd_e2294: f64 = 1.0;
            let noise_7_psd_e2295: f64 = (noise_7_psd_e2294 * w[120]);
            let psd = noise_7_psd_e2295;
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
            let noise_8_psd_e2297: f64 = 1.0;
            let noise_8_psd_e2298: f64 = (noise_8_psd_e2297 * w[113]);
            let psd = noise_8_psd_e2298;
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
            let noise_9_psd_e2300: f64 = 1.0;
            let noise_9_psd_e2301: f64 = (noise_9_psd_e2300 * w[114]);
            let psd = noise_9_psd_e2301;
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
            let noise_10_psd_e2303: f64 = 1.0;
            let noise_10_psd_e2304: f64 = (noise_10_psd_e2303 * w[121]);
            let psd = noise_10_psd_e2304;
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
            let noise_11_psd_e2306: f64 = 1.0;
            let noise_11_psd_e467: f64 = (w[14]).powf(params[76]);
            let noise_11_psd_e468: f64 = (params[75] * noise_11_psd_e467);
            let noise_11_psd_e2307: f64 = (noise_11_psd_e2306 * noise_11_psd_e468);
            let psd = noise_11_psd_e2307;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = Some(params[77]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[12] {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_12_psd_e2309: f64 = 1.0;
            let noise_12_psd_e479: f64 = (2.0 * 1.602176462e-19);
            let noise_12_psd_e481: f64 = (w[7]).abs();
            let noise_12_psd_e482: f64 = (noise_12_psd_e479 * noise_12_psd_e481);
            let noise_12_psd_e2310: f64 = (noise_12_psd_e2309 * noise_12_psd_e482);
            let psd = noise_12_psd_e2310;
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
            let noise_13_psd_e2312: f64 = 1.0;
            let noise_13_psd_e490: f64 = (2.0 * 1.602176462e-19);
            let noise_13_psd_e492: f64 = (w[8]).abs();
            let noise_13_psd_e493: f64 = (noise_13_psd_e490 * noise_13_psd_e492);
            let noise_13_psd_e2313: f64 = (noise_13_psd_e2312 * noise_13_psd_e493);
            let psd = noise_13_psd_e2313;
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
            let noise_14_psd_e2315: f64 = 1.0;
            let noise_14_psd_e503: f64 = (w[7]).abs();
            let noise_14_psd_e505: f64 = (noise_14_psd_e503).powf(params[76]);
            let noise_14_psd_e506: f64 = (params[75] * noise_14_psd_e505);
            let noise_14_psd_e2316: f64 = (noise_14_psd_e2315 * noise_14_psd_e506);
            let psd = noise_14_psd_e2316;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = Some(params[77]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[15] {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_15_psd_e2318: f64 = 1.0;
            let noise_15_psd_e517: f64 = (w[8]).abs();
            let noise_15_psd_e519: f64 = (noise_15_psd_e517).powf(params[76]);
            let noise_15_psd_e520: f64 = (params[75] * noise_15_psd_e519);
            let noise_15_psd_e2319: f64 = (noise_15_psd_e2318 * noise_15_psd_e520);
            let psd = noise_15_psd_e2319;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = Some(params[77]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 125]) {
        let params = &*self.params;
        let noise_activation_schedule_161_0_e2034: f64 = if params[47] > 0.0 { 1.0 } else { 0.0 };
        w[100] = noise_activation_schedule_161_0_e2034;
        let noise_activation_schedule_163_0_e2040: f64 = if params[42] > 0.0 { 1.0 } else { 0.0 };
        w[102] = noise_activation_schedule_163_0_e2040;
        let noise_activation_schedule_165_0_e2046: f64 = if params[46] > 0.0 { 1.0 } else { 0.0 };
        w[104] = noise_activation_schedule_165_0_e2046;
        let noise_activation_schedule_166_0_e2053: f64 = if ((params[43] > 0.0) || (params[44] > 0.0)) { 1.0 } else { 0.0 };
        w[105] = noise_activation_schedule_166_0_e2053;
        let noise_activation_schedule_168_0_e2059: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
        w[107] = noise_activation_schedule_168_0_e2059;
        let noise_activation_schedule_169_0_e2062: f64 = if params[7] == 1.0 { 1.0 } else { 0.0 };
        w[108] = noise_activation_schedule_169_0_e2062;
        let noise_activation_schedule_184_0_e2265: f64 = if params[75] > 0.0 { 1.0 } else { 0.0 };
        w[122] = noise_activation_schedule_184_0_e2265;
        let noise_activation_schedule_185_0_e2268: f64 = if params[75] > 0.0 { 1.0 } else { 0.0 };
        w[123] = noise_activation_schedule_185_0_e2268;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 125], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xf73c) != 0 {
            w[4] = (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[5]));
        }
        if (active[0] & 0xf73c) != 0 {
            w[3] = (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[3]));
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_2_0_e564: f64 = (-w[3]);
            w[6] = noise_metadata_schedule_2_0_e564;
        }
        if (active[0] & 0xf73c) != 0 {
            w[5] = (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[5]));
        }
        if (active[0] & 0x5000) != 0 {
            w[79] = w[4];
        }
        if (active[0] & 0xa030) != 0 {
            w[80] = (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[3]));
        }
        if (active[0] & 0x830) != 0 {
            w[14] = (ctx.node_voltage(self.nodes[13]) - 0.0);
        }
        if (active[0] & 0x700) != 0 {
            w[81] = 0.0;
        }
        if (active[0] & 0xf7ff) != 0 {
            let noise_metadata_schedule_12_0_e575: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            w[82] = noise_metadata_schedule_12_0_e575;
        }
        if (active[0] & 0xf7ff) != 0 {
            let (noise_metadata_schedule_13_0_e581,) = {
    if (w[82] != 0.0) {
        let noise_metadata_schedule_13_0_e579: f64 = (params[3] + 273.15);
        (noise_metadata_schedule_13_0_e579,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_13_0_e581;
        }
        if (active[0] & 0xf7ff) != 0 {
            let (noise_metadata_schedule_14_0_e588,) = {
    if (w[82] == 0.0) {
        let noise_metadata_schedule_14_0_e584: f64 = ctx.temperature();
        let noise_metadata_schedule_14_0_e586: f64 = (noise_metadata_schedule_14_0_e584 + params[2]);
        (noise_metadata_schedule_14_0_e586,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_14_0_e588;
        }
        if (active[0] & 0xf7fc) != 0 {
            let noise_metadata_schedule_15_0_e590: f64 = if self.param_given[85] { 1.0 } else { 0.0 };
            w[83] = noise_metadata_schedule_15_0_e590;
        }
        if (active[0] & 0xf7fc) != 0 {
            let (noise_metadata_schedule_16_0_e596,) = {
    if (w[83] != 0.0) {
        let noise_metadata_schedule_16_0_e594: f64 = (params[85] + 273.15);
        (noise_metadata_schedule_16_0_e594,)
    } else {
        (w[10],)
    }
};
            w[10] = noise_metadata_schedule_16_0_e596;
        }
        if (active[0] & 0xf7fc) != 0 {
            let (noise_metadata_schedule_17_0_e603,) = {
    if (w[83] == 0.0) {
        let noise_metadata_schedule_17_0_e601: f64 = (27.0 + 273.15);
        (noise_metadata_schedule_17_0_e601,)
    } else {
        (w[10],)
    }
};
            w[10] = noise_metadata_schedule_17_0_e603;
        }
        if (active[0] & 0xf7ff) != 0 {
            let (noise_metadata_schedule_18_0_e610,) = {
    if (params[1] != 0.0) {
        let noise_metadata_schedule_18_0_e607: f64 = ((ctx.node_voltage(self.nodes[11]) - 0.0)).abs();
        let noise_metadata_schedule_18_0_e608: f64 = (w[11] + noise_metadata_schedule_18_0_e607);
        (noise_metadata_schedule_18_0_e608,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_18_0_e610;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_19_0_e612: f64 = (w[11] * THERMAL_VOLTAGE_PER_K);
            w[9] = noise_metadata_schedule_19_0_e612;
        }
        if (active[0] & 0xf7fc) != 0 {
            let noise_metadata_schedule_20_0_e615: f64 = (w[11] - w[10]);
            let noise_metadata_schedule_20_0_e616: f64 = (noise_metadata_schedule_20_0_e615).abs();
            w[12] = noise_metadata_schedule_20_0_e616;
        }
        if (active[0] & 0xf7fc) != 0 {
            let noise_metadata_schedule_21_0_e623: f64 = if ((w[12] > 0.0) || (params[57] > 0.0)) { 1.0 } else { 0.0 };
            w[84] = noise_metadata_schedule_21_0_e623;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_23_0_e643,) = {
    if (w[84] != 0.0) {
        let noise_metadata_schedule_23_0_e639: f64 = (params[59] * w[12]);
        let noise_metadata_schedule_23_0_e640: f64 = (1.0 + noise_metadata_schedule_23_0_e639);
        let noise_metadata_schedule_23_0_e641: f64 = (params[8] * noise_metadata_schedule_23_0_e640);
        (noise_metadata_schedule_23_0_e641,)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_23_0_e643;
        }
        if (active[0] & 0x73c) != 0 {
            let (noise_metadata_schedule_24_0_e653,) = {
    if (w[84] != 0.0) {
        let noise_metadata_schedule_24_0_e649: f64 = (params[60] * w[12]);
        let noise_metadata_schedule_24_0_e650: f64 = (1.0 + noise_metadata_schedule_24_0_e649);
        let noise_metadata_schedule_24_0_e651: f64 = (params[11] * noise_metadata_schedule_24_0_e650);
        (noise_metadata_schedule_24_0_e651,)
    } else {
        (w[45],)
    }
};
            w[45] = noise_metadata_schedule_24_0_e653;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_25_0_e663,) = {
    if (w[84] != 0.0) {
        let noise_metadata_schedule_25_0_e659: f64 = (params[63] * w[12]);
        let noise_metadata_schedule_25_0_e660: f64 = (1.0 + noise_metadata_schedule_25_0_e659);
        let noise_metadata_schedule_25_0_e661: f64 = (params[20] * noise_metadata_schedule_25_0_e660);
        (noise_metadata_schedule_25_0_e661,)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_25_0_e663;
        }
        if (active[0] & 0x2c0) != 0 {
            let (noise_metadata_schedule_26_0_e673,) = {
    if (w[84] != 0.0) {
        let noise_metadata_schedule_26_0_e669: f64 = (params[61] * w[12]);
        let noise_metadata_schedule_26_0_e670: f64 = (1.0 + noise_metadata_schedule_26_0_e669);
        let noise_metadata_schedule_26_0_e671: f64 = (params[25] * noise_metadata_schedule_26_0_e670);
        (noise_metadata_schedule_26_0_e671,)
    } else {
        (w[31],)
    }
};
            w[31] = noise_metadata_schedule_26_0_e673;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_30_0_e711,) = {
    if (w[84] != 0.0) {
        let noise_metadata_schedule_30_0_e708: f64 = (params[68] * w[12]);
        let noise_metadata_schedule_30_0_e709: f64 = (params[9] + noise_metadata_schedule_30_0_e708);
        (noise_metadata_schedule_30_0_e709,)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_30_0_e711;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_33_0_e739,) = {
    if (w[84] != 0.0) {
        let noise_metadata_schedule_33_0_e736: f64 = (params[69] * w[12]);
        let noise_metadata_schedule_33_0_e737: f64 = (params[41] + noise_metadata_schedule_33_0_e736);
        (noise_metadata_schedule_33_0_e737,)
    } else {
        (w[42],)
    }
};
            w[42] = noise_metadata_schedule_33_0_e739;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_34_0_e747,) = {
    if (w[84] != 0.0) {
        let noise_metadata_schedule_34_0_e744: f64 = (params[70] * w[12]);
        let noise_metadata_schedule_34_0_e745: f64 = (params[21] + noise_metadata_schedule_34_0_e744);
        (noise_metadata_schedule_34_0_e745,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_34_0_e747;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_35_0_e752,) = {
    if (w[84] == 0.0) {
        (params[8],)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_35_0_e752;
        }
        if (active[0] & 0x73c) != 0 {
            let (noise_metadata_schedule_36_0_e757,) = {
    if (w[84] == 0.0) {
        (params[11],)
    } else {
        (w[45],)
    }
};
            w[45] = noise_metadata_schedule_36_0_e757;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_37_0_e762,) = {
    if (w[84] == 0.0) {
        (params[20],)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_37_0_e762;
        }
        if (active[0] & 0x2c0) != 0 {
            let (noise_metadata_schedule_38_0_e767,) = {
    if (w[84] == 0.0) {
        (params[25],)
    } else {
        (w[31],)
    }
};
            w[31] = noise_metadata_schedule_38_0_e767;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_42_0_e787,) = {
    if (w[84] == 0.0) {
        (params[9],)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_42_0_e787;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_45_0_e802,) = {
    if (w[84] == 0.0) {
        (params[41],)
    } else {
        (w[42],)
    }
};
            w[42] = noise_metadata_schedule_45_0_e802;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_46_0_e807,) = {
    if (w[84] == 0.0) {
        (params[21],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_46_0_e807;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_47_0_e813: f64 = if ((!self.param_given[39]) && self.param_given[40]) { 1.0 } else { 0.0 };
            w[85] = noise_metadata_schedule_47_0_e813;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_48_0_e821,) = {
    if (w[85] != 0.0) {
        let noise_metadata_schedule_48_0_e817: f64 = (0.5 / params[40]);
        let noise_metadata_schedule_48_0_e819: f64 = (noise_metadata_schedule_48_0_e817 / w[9]);
        (noise_metadata_schedule_48_0_e819,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_48_0_e821;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_49_0_e826,) = {
    if (w[85] == 0.0) {
        (params[39],)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_49_0_e826;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_50_0_e829: f64 = (params[19] * w[5]);
            let noise_metadata_schedule_50_0_e830: f64 = (noise_metadata_schedule_50_0_e829).cosh();
            w[47] = noise_metadata_schedule_50_0_e830;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_51_0_e836: f64 = (w[47] * w[47]);
            let noise_metadata_schedule_51_0_e837: f64 = (params[18] / noise_metadata_schedule_51_0_e836);
            let noise_metadata_schedule_51_0_e838: f64 = (1.0 + noise_metadata_schedule_51_0_e837);
            let noise_metadata_schedule_51_0_e839: f64 = (w[45] * noise_metadata_schedule_51_0_e838);
            w[44] = noise_metadata_schedule_51_0_e839;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_52_0_e842: f64 = (w[39] - params[10]);
            let noise_metadata_schedule_52_0_e846: f64 = (params[15] * w[5]);
            let noise_metadata_schedule_52_0_e847: f64 = (noise_metadata_schedule_52_0_e846).tanh();
            let noise_metadata_schedule_52_0_e848: f64 = (params[10] * noise_metadata_schedule_52_0_e847);
            let noise_metadata_schedule_52_0_e849: f64 = (noise_metadata_schedule_52_0_e842 + noise_metadata_schedule_52_0_e848);
            let noise_metadata_schedule_52_0_e853: f64 = (w[6] - params[21]);
            let noise_metadata_schedule_52_0_e854: f64 = (params[22] * noise_metadata_schedule_52_0_e853);
            let noise_metadata_schedule_52_0_e857: f64 = (w[6] - w[38]);
            let noise_metadata_schedule_52_0_e858: f64 = (noise_metadata_schedule_52_0_e854 * noise_metadata_schedule_52_0_e857);
            let noise_metadata_schedule_52_0_e859: f64 = (noise_metadata_schedule_52_0_e849 - noise_metadata_schedule_52_0_e858);
            w[46] = noise_metadata_schedule_52_0_e859;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_53_0_e862: f64 = (w[4] - w[46]);
            w[48] = noise_metadata_schedule_53_0_e862;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_54_0_e865: f64 = (w[48] * w[48]);
            w[49] = noise_metadata_schedule_54_0_e865;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_55_0_e868: f64 = (w[44] * w[48]);
            let noise_metadata_schedule_55_0_e871: f64 = (params[12] * w[49]);
            let noise_metadata_schedule_55_0_e872: f64 = (noise_metadata_schedule_55_0_e868 + noise_metadata_schedule_55_0_e871);
            let noise_metadata_schedule_55_0_e875: f64 = (params[13] * w[48]);
            let noise_metadata_schedule_55_0_e877: f64 = (noise_metadata_schedule_55_0_e875 * w[49]);
            let noise_metadata_schedule_55_0_e878: f64 = (noise_metadata_schedule_55_0_e872 + noise_metadata_schedule_55_0_e877);
            w[13] = noise_metadata_schedule_55_0_e878;
        }
        if (active[0] & 0x73c) != 0 {
            let noise_metadata_schedule_56_0_e881: f64 = (w[13]).tanh();
            let noise_metadata_schedule_56_0_e882: f64 = (1.0 + noise_metadata_schedule_56_0_e881);
            w[59] = noise_metadata_schedule_56_0_e882;
        }
        if (active[0] & 0x70c) != 0 {
            let noise_metadata_schedule_57_0_e886: f64 = { let limexp_arg = w[13]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_57_0_e888: f64 = (-w[13]);
            let noise_metadata_schedule_57_0_e889: f64 = { let limexp_arg = noise_metadata_schedule_57_0_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_57_0_e890: f64 = (noise_metadata_schedule_57_0_e886 - noise_metadata_schedule_57_0_e889);
            let noise_metadata_schedule_57_0_e891: f64 = (0.5 * noise_metadata_schedule_57_0_e890);
            let noise_metadata_schedule_57_0_e892: f64 = (noise_metadata_schedule_57_0_e891).tanh();
            let noise_metadata_schedule_57_0_e893: f64 = (1.0 + noise_metadata_schedule_57_0_e892);
            w[60] = noise_metadata_schedule_57_0_e893;
        }
        if (active[0] & 0x730) != 0 {
            let noise_metadata_schedule_58_0_e897: f64 = (params[15] * w[59]);
            let noise_metadata_schedule_58_0_e898: f64 = (params[14] + noise_metadata_schedule_58_0_e897);
            w[0] = noise_metadata_schedule_58_0_e898;
        }
        if (active[0] & 0x730) != 0 {
            let noise_metadata_schedule_59_0_e901: f64 = (w[0] * w[5]);
            let noise_metadata_schedule_59_0_e902: f64 = (noise_metadata_schedule_59_0_e901).tanh();
            w[63] = noise_metadata_schedule_59_0_e902;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_60_0_e905: f64 = if params[4] == 0.0 { 1.0 } else { 0.0 };
            w[86] = noise_metadata_schedule_60_0_e905;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_61_0_e908: f64 = if params[4] == 1.0 { 1.0 } else { 0.0 };
            w[87] = noise_metadata_schedule_61_0_e908;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_62_0_e911: f64 = if params[4] == 2.0 { 1.0 } else { 0.0 };
            w[88] = noise_metadata_schedule_62_0_e911;
        }
        if (active[0] & 0xf73c) != 0 {
            let noise_metadata_schedule_63_0_e914: f64 = if params[4] == 3.0 { 1.0 } else { 0.0 };
            w[89] = noise_metadata_schedule_63_0_e914;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_64_0_e935,) = {
    if (w[86] != 0.0) {
        let noise_metadata_schedule_64_0_e918: f64 = (w[26] * w[59]);
        let noise_metadata_schedule_64_0_e920: f64 = (noise_metadata_schedule_64_0_e918 * w[63]);
        let noise_metadata_schedule_64_0_e924: f64 = (params[16] * w[5]);
        let noise_metadata_schedule_64_0_e925: f64 = (1.0 + noise_metadata_schedule_64_0_e924);
        let noise_metadata_schedule_64_0_e929: f64 = (w[6] - w[38]);
        let noise_metadata_schedule_64_0_e930: f64 = { let limexp_arg = noise_metadata_schedule_64_0_e929; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_64_0_e931: f64 = (w[30] * noise_metadata_schedule_64_0_e930);
        let noise_metadata_schedule_64_0_e932: f64 = (noise_metadata_schedule_64_0_e925 + noise_metadata_schedule_64_0_e931);
        let noise_metadata_schedule_64_0_e933: f64 = (noise_metadata_schedule_64_0_e920 * noise_metadata_schedule_64_0_e932);
        (noise_metadata_schedule_64_0_e933,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_64_0_e935;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_65_0_e944,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_65_0_e942: f64 = (w[3] - w[46]);
        (noise_metadata_schedule_65_0_e942,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_65_0_e944;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_66_0_e953,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_66_0_e951: f64 = (w[47] * w[47]);
        (noise_metadata_schedule_66_0_e951,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_66_0_e953;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_67_0_e962,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_67_0_e960: f64 = (w[48] * w[47]);
        (noise_metadata_schedule_67_0_e960,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_67_0_e962;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_68_0_e979,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_68_0_e969: f64 = (w[44] * w[47]);
        let noise_metadata_schedule_68_0_e972: f64 = (params[12] * w[48]);
        let noise_metadata_schedule_68_0_e973: f64 = (noise_metadata_schedule_68_0_e969 + noise_metadata_schedule_68_0_e972);
        let noise_metadata_schedule_68_0_e976: f64 = (params[13] * w[49]);
        let noise_metadata_schedule_68_0_e977: f64 = (noise_metadata_schedule_68_0_e973 + noise_metadata_schedule_68_0_e976);
        (noise_metadata_schedule_68_0_e977,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_68_0_e979;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_69_0_e989,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_69_0_e986: f64 = (w[55]).tanh();
        let noise_metadata_schedule_69_0_e987: f64 = (1.0 + noise_metadata_schedule_69_0_e986);
        (noise_metadata_schedule_69_0_e987,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_69_0_e989;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_70_0_e1000,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_70_0_e997: f64 = (params[15] * w[61]);
        let noise_metadata_schedule_70_0_e998: f64 = (params[14] + noise_metadata_schedule_70_0_e997);
        (noise_metadata_schedule_70_0_e998,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_70_0_e1000;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_71_0_e1011,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_71_0_e1008: f64 = (params[17] * w[59]);
        let noise_metadata_schedule_71_0_e1009: f64 = (params[16] + noise_metadata_schedule_71_0_e1008);
        (noise_metadata_schedule_71_0_e1009,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_71_0_e1011;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_72_0_e1037,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_72_0_e1018: f64 = (w[26] * w[59]);
        let noise_metadata_schedule_72_0_e1021: f64 = (1.0 + w[63]);
        let noise_metadata_schedule_72_0_e1022: f64 = (noise_metadata_schedule_72_0_e1018 * noise_metadata_schedule_72_0_e1021);
        let noise_metadata_schedule_72_0_e1026: f64 = (w[53] * w[5]);
        let noise_metadata_schedule_72_0_e1027: f64 = (1.0 + noise_metadata_schedule_72_0_e1026);
        let noise_metadata_schedule_72_0_e1031: f64 = (w[5] - w[38]);
        let noise_metadata_schedule_72_0_e1032: f64 = { let limexp_arg = noise_metadata_schedule_72_0_e1031; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_72_0_e1033: f64 = (w[30] * noise_metadata_schedule_72_0_e1032);
        let noise_metadata_schedule_72_0_e1034: f64 = (noise_metadata_schedule_72_0_e1027 + noise_metadata_schedule_72_0_e1033);
        let noise_metadata_schedule_72_0_e1035: f64 = (noise_metadata_schedule_72_0_e1022 * noise_metadata_schedule_72_0_e1034);
        (noise_metadata_schedule_72_0_e1035,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_72_0_e1037;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_73_0_e1048,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_73_0_e1045: f64 = (params[17] * w[61]);
        let noise_metadata_schedule_73_0_e1046: f64 = (params[16] + noise_metadata_schedule_73_0_e1045);
        (noise_metadata_schedule_73_0_e1046,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_73_0_e1048;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 125], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_74_0_e1058,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_74_0_e1055: f64 = (w[56] * w[5]);
        let noise_metadata_schedule_74_0_e1056: f64 = (noise_metadata_schedule_74_0_e1055).tanh();
        (noise_metadata_schedule_74_0_e1056,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_74_0_e1058;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_75_0_e1077,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_75_0_e1065: f64 = (w[26] * w[61]);
        let noise_metadata_schedule_75_0_e1068: f64 = (1.0 - w[64]);
        let noise_metadata_schedule_75_0_e1069: f64 = (noise_metadata_schedule_75_0_e1065 * noise_metadata_schedule_75_0_e1068);
        let noise_metadata_schedule_75_0_e1073: f64 = (w[51] * w[5]);
        let noise_metadata_schedule_75_0_e1074: f64 = (1.0 - noise_metadata_schedule_75_0_e1073);
        let noise_metadata_schedule_75_0_e1075: f64 = (noise_metadata_schedule_75_0_e1069 * noise_metadata_schedule_75_0_e1074);
        (noise_metadata_schedule_75_0_e1075,)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_75_0_e1077;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_76_0_e1088,) = {
    if ((w[87] != 0.0) && (w[86] == 0.0)) {
        let noise_metadata_schedule_76_0_e1085: f64 = (w[57] - w[58]);
        let noise_metadata_schedule_76_0_e1086: f64 = (0.5 * noise_metadata_schedule_76_0_e1085);
        (noise_metadata_schedule_76_0_e1086,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_76_0_e1088;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_77_0_e1099,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_77_0_e1097: f64 = (w[4] - w[46]);
        (noise_metadata_schedule_77_0_e1097,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_77_0_e1099;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_78_0_e1110,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_78_0_e1108: f64 = (w[47] * w[47]);
        (noise_metadata_schedule_78_0_e1108,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_78_0_e1110;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_79_0_e1131,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_79_0_e1121: f64 = (params[12] * w[48]);
        let noise_metadata_schedule_79_0_e1122: f64 = (w[47] + noise_metadata_schedule_79_0_e1121);
        let noise_metadata_schedule_79_0_e1125: f64 = (params[13] * w[48]);
        let noise_metadata_schedule_79_0_e1127: f64 = (noise_metadata_schedule_79_0_e1125 * w[47]);
        let noise_metadata_schedule_79_0_e1128: f64 = (noise_metadata_schedule_79_0_e1122 + noise_metadata_schedule_79_0_e1127);
        let noise_metadata_schedule_79_0_e1129: f64 = (w[44] * noise_metadata_schedule_79_0_e1128);
        (noise_metadata_schedule_79_0_e1129,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_79_0_e1131;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_80_0_e1150,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_80_0_e1141: f64 = { let limexp_arg = w[13]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_80_0_e1143: f64 = (-w[13]);
        let noise_metadata_schedule_80_0_e1144: f64 = { let limexp_arg = noise_metadata_schedule_80_0_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_80_0_e1145: f64 = (noise_metadata_schedule_80_0_e1141 - noise_metadata_schedule_80_0_e1144);
        let noise_metadata_schedule_80_0_e1146: f64 = (0.5 * noise_metadata_schedule_80_0_e1145);
        let noise_metadata_schedule_80_0_e1147: f64 = (noise_metadata_schedule_80_0_e1146).tanh();
        let noise_metadata_schedule_80_0_e1148: f64 = (1.0 + noise_metadata_schedule_80_0_e1147);
        (noise_metadata_schedule_80_0_e1148,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_80_0_e1150;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_81_0_e1163,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_81_0_e1160: f64 = (params[15] * w[60]);
        let noise_metadata_schedule_81_0_e1161: f64 = (params[14] + noise_metadata_schedule_81_0_e1160);
        (noise_metadata_schedule_81_0_e1161,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_81_0_e1163;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_82_0_e1175,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_82_0_e1172: f64 = (w[1] * w[5]);
        let noise_metadata_schedule_82_0_e1173: f64 = (noise_metadata_schedule_82_0_e1172).tanh();
        (noise_metadata_schedule_82_0_e1173,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_82_0_e1175;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_83_0_e1188,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_83_0_e1185: f64 = (params[17] * w[60]);
        let noise_metadata_schedule_83_0_e1186: f64 = (params[16] + noise_metadata_schedule_83_0_e1185);
        (noise_metadata_schedule_83_0_e1186,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_83_0_e1188;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_84_0_e1214,) = {
    if ((w[88] != 0.0) && (!((w[86] != 0.0) || (w[87] != 0.0)))) {
        let noise_metadata_schedule_84_0_e1197: f64 = (w[26] * w[60]);
        let noise_metadata_schedule_84_0_e1199: f64 = (noise_metadata_schedule_84_0_e1197 * w[65]);
        let noise_metadata_schedule_84_0_e1203: f64 = (w[53] * w[5]);
        let noise_metadata_schedule_84_0_e1204: f64 = (1.0 + noise_metadata_schedule_84_0_e1203);
        let noise_metadata_schedule_84_0_e1208: f64 = (w[6] - w[38]);
        let noise_metadata_schedule_84_0_e1209: f64 = { let limexp_arg = noise_metadata_schedule_84_0_e1208; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_84_0_e1210: f64 = (w[30] * noise_metadata_schedule_84_0_e1209);
        let noise_metadata_schedule_84_0_e1211: f64 = (noise_metadata_schedule_84_0_e1204 + noise_metadata_schedule_84_0_e1210);
        let noise_metadata_schedule_84_0_e1212: f64 = (noise_metadata_schedule_84_0_e1199 * noise_metadata_schedule_84_0_e1211);
        (noise_metadata_schedule_84_0_e1212,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_84_0_e1214;
        }
        if (active[0] & 0xf73c) != 0 {
            let (noise_metadata_schedule_85_0_e1227,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_85_0_e1225: f64 = (w[4] - w[46]);
        (noise_metadata_schedule_85_0_e1225,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_85_0_e1227;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_86_0_e1240,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_86_0_e1238: f64 = (w[47] * w[47]);
        (noise_metadata_schedule_86_0_e1238,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_86_0_e1240;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_87_0_e1263,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_87_0_e1253: f64 = (params[12] * w[48]);
        let noise_metadata_schedule_87_0_e1254: f64 = (w[47] + noise_metadata_schedule_87_0_e1253);
        let noise_metadata_schedule_87_0_e1257: f64 = (params[13] * w[48]);
        let noise_metadata_schedule_87_0_e1259: f64 = (noise_metadata_schedule_87_0_e1257 * w[47]);
        let noise_metadata_schedule_87_0_e1260: f64 = (noise_metadata_schedule_87_0_e1254 + noise_metadata_schedule_87_0_e1259);
        let noise_metadata_schedule_87_0_e1261: f64 = (w[44] * noise_metadata_schedule_87_0_e1260);
        (noise_metadata_schedule_87_0_e1261,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_87_0_e1263;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_88_0_e1276,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_88_0_e1274: f64 = (w[3] - w[46]);
        (noise_metadata_schedule_88_0_e1274,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_88_0_e1276;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_89_0_e1289,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_89_0_e1287: f64 = (w[49] * w[49]);
        (noise_metadata_schedule_89_0_e1287,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_89_0_e1289;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_90_0_e1312,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_90_0_e1302: f64 = (params[12] * w[50]);
        let noise_metadata_schedule_90_0_e1303: f64 = (w[49] + noise_metadata_schedule_90_0_e1302);
        let noise_metadata_schedule_90_0_e1306: f64 = (params[13] * w[49]);
        let noise_metadata_schedule_90_0_e1308: f64 = (noise_metadata_schedule_90_0_e1306 * w[50]);
        let noise_metadata_schedule_90_0_e1309: f64 = (noise_metadata_schedule_90_0_e1303 + noise_metadata_schedule_90_0_e1308);
        let noise_metadata_schedule_90_0_e1310: f64 = (w[44] * noise_metadata_schedule_90_0_e1309);
        (noise_metadata_schedule_90_0_e1310,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_90_0_e1312;
        }
        if (active[0] & 0x70c) != 0 {
            let (noise_metadata_schedule_91_0_e1333,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_91_0_e1324: f64 = { let limexp_arg = w[13]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_91_0_e1326: f64 = (-w[13]);
        let noise_metadata_schedule_91_0_e1327: f64 = { let limexp_arg = noise_metadata_schedule_91_0_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_91_0_e1328: f64 = (noise_metadata_schedule_91_0_e1324 - noise_metadata_schedule_91_0_e1327);
        let noise_metadata_schedule_91_0_e1329: f64 = (0.5 * noise_metadata_schedule_91_0_e1328);
        let noise_metadata_schedule_91_0_e1330: f64 = (noise_metadata_schedule_91_0_e1329).tanh();
        let noise_metadata_schedule_91_0_e1331: f64 = (1.0 + noise_metadata_schedule_91_0_e1330);
        (noise_metadata_schedule_91_0_e1331,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_91_0_e1333;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_92_0_e1354,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_92_0_e1345: f64 = { let limexp_arg = w[55]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_92_0_e1347: f64 = (-w[55]);
        let noise_metadata_schedule_92_0_e1348: f64 = { let limexp_arg = noise_metadata_schedule_92_0_e1347; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_92_0_e1349: f64 = (noise_metadata_schedule_92_0_e1345 - noise_metadata_schedule_92_0_e1348);
        let noise_metadata_schedule_92_0_e1350: f64 = (0.5 * noise_metadata_schedule_92_0_e1349);
        let noise_metadata_schedule_92_0_e1351: f64 = (noise_metadata_schedule_92_0_e1350).tanh();
        let noise_metadata_schedule_92_0_e1352: f64 = (1.0 + noise_metadata_schedule_92_0_e1351);
        (noise_metadata_schedule_92_0_e1352,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_92_0_e1354;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_93_0_e1369,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_93_0_e1366: f64 = (params[15] * w[60]);
        let noise_metadata_schedule_93_0_e1367: f64 = (params[14] + noise_metadata_schedule_93_0_e1366);
        (noise_metadata_schedule_93_0_e1367,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_93_0_e1369;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_94_0_e1384,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_94_0_e1381: f64 = (params[15] * w[62]);
        let noise_metadata_schedule_94_0_e1382: f64 = (params[14] + noise_metadata_schedule_94_0_e1381);
        (noise_metadata_schedule_94_0_e1382,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_94_0_e1384;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_95_0_e1398,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_95_0_e1395: f64 = (w[1] * w[5]);
        let noise_metadata_schedule_95_0_e1396: f64 = (noise_metadata_schedule_95_0_e1395).tanh();
        (noise_metadata_schedule_95_0_e1396,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_95_0_e1398;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_96_0_e1412,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_96_0_e1409: f64 = (w[2] * w[5]);
        let noise_metadata_schedule_96_0_e1410: f64 = (noise_metadata_schedule_96_0_e1409).tanh();
        (noise_metadata_schedule_96_0_e1410,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_96_0_e1412;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_97_0_e1427,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_97_0_e1424: f64 = (params[17] * w[62]);
        let noise_metadata_schedule_97_0_e1425: f64 = (params[16] + noise_metadata_schedule_97_0_e1424);
        (noise_metadata_schedule_97_0_e1425,)
    } else {
        (w[52],)
    }
};
            w[52] = noise_metadata_schedule_97_0_e1427;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_98_0_e1442,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_98_0_e1439: f64 = (params[17] * w[60]);
        let noise_metadata_schedule_98_0_e1440: f64 = (params[16] + noise_metadata_schedule_98_0_e1439);
        (noise_metadata_schedule_98_0_e1440,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_98_0_e1442;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_99_0_e1472,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_99_0_e1453: f64 = (w[26] * w[60]);
        let noise_metadata_schedule_99_0_e1456: f64 = (1.0 + w[65]);
        let noise_metadata_schedule_99_0_e1457: f64 = (noise_metadata_schedule_99_0_e1453 * noise_metadata_schedule_99_0_e1456);
        let noise_metadata_schedule_99_0_e1461: f64 = (w[54] * w[5]);
        let noise_metadata_schedule_99_0_e1462: f64 = (1.0 + noise_metadata_schedule_99_0_e1461);
        let noise_metadata_schedule_99_0_e1466: f64 = (w[5] - w[38]);
        let noise_metadata_schedule_99_0_e1467: f64 = { let limexp_arg = noise_metadata_schedule_99_0_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_99_0_e1468: f64 = (w[30] * noise_metadata_schedule_99_0_e1467);
        let noise_metadata_schedule_99_0_e1469: f64 = (noise_metadata_schedule_99_0_e1462 + noise_metadata_schedule_99_0_e1468);
        let noise_metadata_schedule_99_0_e1470: f64 = (noise_metadata_schedule_99_0_e1457 * noise_metadata_schedule_99_0_e1469);
        (noise_metadata_schedule_99_0_e1470,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_99_0_e1472;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_100_0_e1495,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_100_0_e1483: f64 = (w[26] * w[62]);
        let noise_metadata_schedule_100_0_e1486: f64 = (1.0 - w[66]);
        let noise_metadata_schedule_100_0_e1487: f64 = (noise_metadata_schedule_100_0_e1483 * noise_metadata_schedule_100_0_e1486);
        let noise_metadata_schedule_100_0_e1491: f64 = (w[52] * w[5]);
        let noise_metadata_schedule_100_0_e1492: f64 = (1.0 - noise_metadata_schedule_100_0_e1491);
        let noise_metadata_schedule_100_0_e1493: f64 = (noise_metadata_schedule_100_0_e1487 * noise_metadata_schedule_100_0_e1492);
        (noise_metadata_schedule_100_0_e1493,)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_100_0_e1495;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_101_0_e1510,) = {
    if ((w[89] != 0.0) && (!(((w[86] != 0.0) || (w[87] != 0.0)) || (w[88] != 0.0)))) {
        let noise_metadata_schedule_101_0_e1507: f64 = (w[57] - w[58]);
        let noise_metadata_schedule_101_0_e1508: f64 = (0.5 * noise_metadata_schedule_101_0_e1507);
        (noise_metadata_schedule_101_0_e1508,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_101_0_e1510;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_102_0_e1517: f64 = if ((params[4] == 0.0) || (params[4] == 1.0)) { 1.0 } else { 0.0 };
            w[90] = noise_metadata_schedule_102_0_e1517;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_104_0_e1535,) = {
    if (w[90] != 0.0) {
        let noise_metadata_schedule_104_0_e1532: f64 = (params[44] * w[59]);
        let noise_metadata_schedule_104_0_e1533: f64 = (params[43] + noise_metadata_schedule_104_0_e1532);
        (noise_metadata_schedule_104_0_e1533,)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_104_0_e1535;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_105_0_e1543,) = {
    if (w[90] != 0.0) {
        let noise_metadata_schedule_105_0_e1540: f64 = (params[44] * w[59]);
        let noise_metadata_schedule_105_0_e1541: f64 = (params[46] + noise_metadata_schedule_105_0_e1540);
        (noise_metadata_schedule_105_0_e1541,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_105_0_e1543;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_107_0_e1563,) = {
    if (w[90] == 0.0) {
        let noise_metadata_schedule_107_0_e1560: f64 = (params[44] * w[60]);
        let noise_metadata_schedule_107_0_e1561: f64 = (params[43] + noise_metadata_schedule_107_0_e1560);
        (noise_metadata_schedule_107_0_e1561,)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_107_0_e1563;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_108_0_e1572,) = {
    if (w[90] == 0.0) {
        let noise_metadata_schedule_108_0_e1569: f64 = (params[44] * w[60]);
        let noise_metadata_schedule_108_0_e1570: f64 = (params[46] + noise_metadata_schedule_108_0_e1569);
        (noise_metadata_schedule_108_0_e1570,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_108_0_e1572;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_109_0_e1577: f64 = if ((w[12] != 0.0) || (params[57] > 0.0)) { 1.0 } else { 0.0 };
            w[91] = noise_metadata_schedule_109_0_e1577;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_110_0_e1587,) = {
    if (w[91] != 0.0) {
        let noise_metadata_schedule_110_0_e1583: f64 = (params[66] * w[12]);
        let noise_metadata_schedule_110_0_e1584: f64 = (1.0 + noise_metadata_schedule_110_0_e1583);
        let noise_metadata_schedule_110_0_e1585: f64 = (w[29] * noise_metadata_schedule_110_0_e1584);
        (noise_metadata_schedule_110_0_e1585,)
    } else {
        (w[36],)
    }
};
            w[36] = noise_metadata_schedule_110_0_e1587;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_114_0_e1617,) = {
    if (w[91] == 0.0) {
        (w[29],)
    } else {
        (w[36],)
    }
};
            w[36] = noise_metadata_schedule_114_0_e1617;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_116_0_e1625: f64 = if params[5] == 0.0 { 1.0 } else { 0.0 };
            w[92] = noise_metadata_schedule_116_0_e1625;
        }
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_117_0_e1636,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_117_0_e1629: f64 = (-1.0);
        let noise_metadata_schedule_117_0_e1631: f64 = (noise_metadata_schedule_117_0_e1629 * w[42]);
        let noise_metadata_schedule_117_0_e1632: f64 = (noise_metadata_schedule_117_0_e1631).tanh();
        let noise_metadata_schedule_117_0_e1633: f64 = (w[15] * noise_metadata_schedule_117_0_e1632);
        let noise_metadata_schedule_117_0_e1634: f64 = { let limexp_arg = noise_metadata_schedule_117_0_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_117_0_e1634,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_117_0_e1636;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_118_0_e1644,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_118_0_e1641: f64 = (w[79] - w[42]);
        let noise_metadata_schedule_118_0_e1642: f64 = noise_metadata_schedule_118_0_e1641;
        (noise_metadata_schedule_118_0_e1642,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_118_0_e1644;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_119_0_e1652,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_119_0_e1649: f64 = (w[80] - w[42]);
        let noise_metadata_schedule_119_0_e1650: f64 = noise_metadata_schedule_119_0_e1649;
        (noise_metadata_schedule_119_0_e1650,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_119_0_e1652;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 125], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xf030) != 0 {
            let (noise_metadata_schedule_120_0_e1661,) = {
    if (w[92] == 0.0) {
        let noise_metadata_schedule_120_0_e1656: f64 = (-w[15]);
        let noise_metadata_schedule_120_0_e1658: f64 = (noise_metadata_schedule_120_0_e1656 * w[42]);
        let noise_metadata_schedule_120_0_e1659: f64 = { let limexp_arg = noise_metadata_schedule_120_0_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (noise_metadata_schedule_120_0_e1659,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_120_0_e1661;
        }
        if (active[0] & 0xf030) != 0 {
            let noise_metadata_schedule_121_0_e1664: f64 = if params[5] == 1.0 { 1.0 } else { 0.0 };
            w[93] = noise_metadata_schedule_121_0_e1664;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_122_0_e1674,) = {
    if ((w[92] == 0.0) && (w[93] != 0.0)) {
        let noise_metadata_schedule_122_0_e1671: f64 = (w[79] - w[42]);
        let noise_metadata_schedule_122_0_e1672: f64 = (noise_metadata_schedule_122_0_e1671).tanh();
        (noise_metadata_schedule_122_0_e1672,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_122_0_e1674;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_123_0_e1684,) = {
    if ((w[92] == 0.0) && (w[93] != 0.0)) {
        let noise_metadata_schedule_123_0_e1681: f64 = (w[80] - w[42]);
        let noise_metadata_schedule_123_0_e1682: f64 = (noise_metadata_schedule_123_0_e1681).tanh();
        (noise_metadata_schedule_123_0_e1682,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_123_0_e1684;
        }
        if (active[0] & 0x5000) != 0 {
            let (noise_metadata_schedule_124_0_e1694,) = {
    if ((w[92] == 0.0) && (w[93] == 0.0)) {
        let noise_metadata_schedule_124_0_e1692: f64 = (w[79] - w[42]);
        (noise_metadata_schedule_124_0_e1692,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_124_0_e1694;
        }
        if (active[0] & 0xa030) != 0 {
            let (noise_metadata_schedule_125_0_e1704,) = {
    if ((w[92] == 0.0) && (w[93] == 0.0)) {
        let noise_metadata_schedule_125_0_e1702: f64 = (w[80] - w[42]);
        (noise_metadata_schedule_125_0_e1702,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_125_0_e1704;
        }
        if (active[0] & 0x5000) != 0 {
            let noise_metadata_schedule_126_0_e1708: f64 = (w[15] * w[16]);
            let noise_metadata_schedule_126_0_e1709: f64 = { let limexp_arg = noise_metadata_schedule_126_0_e1708; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_126_0_e1711: f64 = (noise_metadata_schedule_126_0_e1709 - w[47]);
            let noise_metadata_schedule_126_0_e1712: f64 = (params[38] * noise_metadata_schedule_126_0_e1711);
            w[7] = noise_metadata_schedule_126_0_e1712;
        }
        if (active[0] & 0xa030) != 0 {
            let noise_metadata_schedule_127_0_e1716: f64 = (w[15] * w[17]);
            let noise_metadata_schedule_127_0_e1717: f64 = { let limexp_arg = noise_metadata_schedule_127_0_e1716; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_127_0_e1719: f64 = (noise_metadata_schedule_127_0_e1717 - w[47]);
            let noise_metadata_schedule_127_0_e1720: f64 = (params[38] * noise_metadata_schedule_127_0_e1719);
            w[8] = noise_metadata_schedule_127_0_e1720;
        }
        if (active[0] & 0x7f0) != 0 {
            let noise_metadata_schedule_168_0_e2059: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[107] = noise_metadata_schedule_168_0_e2059;
        }
        if (active[0] & 0x7c0) != 0 {
            let noise_metadata_schedule_169_0_e2062: f64 = if params[7] == 1.0 { 1.0 } else { 0.0 };
            w[108] = noise_metadata_schedule_169_0_e2062;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_171_0_e2073,) = {
    if (w[107] != 0.0) {
        let noise_metadata_schedule_171_0_e2068: f64 = (w[14]).abs();
        let noise_metadata_schedule_171_0_e2070: f64 = (w[8]).abs();
        let noise_metadata_schedule_171_0_e2071: f64 = (noise_metadata_schedule_171_0_e2068 + noise_metadata_schedule_171_0_e2070);
        (noise_metadata_schedule_171_0_e2071,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_171_0_e2073;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_172_0_e2094,) = {
    if (w[107] != 0.0) {
        let noise_metadata_schedule_172_0_e2077: f64 = (params[78] + 273.15);
        let noise_metadata_schedule_172_0_e2081: f64 = (params[80] * w[59]);
        let noise_metadata_schedule_172_0_e2083: f64 = (w[63]).abs();
        let noise_metadata_schedule_172_0_e2084: f64 = (noise_metadata_schedule_172_0_e2081 * noise_metadata_schedule_172_0_e2083);
        let noise_metadata_schedule_172_0_e2088: f64 = (params[16] * w[5]);
        let noise_metadata_schedule_172_0_e2089: f64 = (1.0 + noise_metadata_schedule_172_0_e2088);
        let noise_metadata_schedule_172_0_e2090: f64 = (noise_metadata_schedule_172_0_e2084 * noise_metadata_schedule_172_0_e2089);
        let noise_metadata_schedule_172_0_e2091: f64 = (1.0 + noise_metadata_schedule_172_0_e2090);
        let noise_metadata_schedule_172_0_e2092: f64 = (noise_metadata_schedule_172_0_e2077 * noise_metadata_schedule_172_0_e2091);
        (noise_metadata_schedule_172_0_e2092,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_172_0_e2094;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_173_0_e2118,) = {
    if (w[107] != 0.0) {
        let noise_metadata_schedule_173_0_e2098: f64 = (params[84] * 4.0);
        let noise_metadata_schedule_173_0_e2100: f64 = (noise_metadata_schedule_173_0_e2098 * 1.3806503e-23);
        let noise_metadata_schedule_173_0_e2102: f64 = (noise_metadata_schedule_173_0_e2100 * w[11]);
        let noise_metadata_schedule_173_0_e2105: f64 = (w[112] / w[11]);
        let noise_metadata_schedule_173_0_e2107: f64 = (noise_metadata_schedule_173_0_e2105 * w[111]);
        let noise_metadata_schedule_173_0_e2110: f64 = (params[79] * w[111]);
        let noise_metadata_schedule_173_0_e2112: f64 = (noise_metadata_schedule_173_0_e2110 * w[111]);
        let noise_metadata_schedule_173_0_e2113: f64 = (noise_metadata_schedule_173_0_e2107 + noise_metadata_schedule_173_0_e2112);
        let noise_metadata_schedule_173_0_e2114: f64 = (noise_metadata_schedule_173_0_e2113).abs();
        let noise_metadata_schedule_173_0_e2115: f64 = (noise_metadata_schedule_173_0_e2114).sqrt();
        let noise_metadata_schedule_173_0_e2116: f64 = (noise_metadata_schedule_173_0_e2102 * noise_metadata_schedule_173_0_e2115);
        (noise_metadata_schedule_173_0_e2116,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_173_0_e2118;
        }
        if (active[0] & 0x700) != 0 {
            let (noise_metadata_schedule_174_0_e2129,) = {
    if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) {
        let noise_metadata_schedule_174_0_e2127: f64 = 0.0;
        (noise_metadata_schedule_174_0_e2127,)
    } else {
        (w[115],)
    }
};
            w[115] = noise_metadata_schedule_174_0_e2129;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_175_0_e2146,) = {
    if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) {
        let noise_metadata_schedule_175_0_e2138: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_175_0_e2140: f64 = (noise_metadata_schedule_175_0_e2138 * w[11]);
        let noise_metadata_schedule_175_0_e2142: f64 = (noise_metadata_schedule_175_0_e2140 * w[115]);
        let noise_metadata_schedule_175_0_e2144: f64 = (noise_metadata_schedule_175_0_e2142 * params[72]);
        (noise_metadata_schedule_175_0_e2144,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_175_0_e2146;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_176_0_e2149: f64 = if w[115] > 0.0 { 1.0 } else { 0.0 };
            w[116] = noise_metadata_schedule_176_0_e2149;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_177_0_e2172,) = {
    if ((((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) && (w[116] != 0.0)) {
        let noise_metadata_schedule_177_0_e2160: f64 = (w[31] * w[31]);
        let noise_metadata_schedule_177_0_e2162: f64 = (noise_metadata_schedule_177_0_e2160 * 4.0);
        let noise_metadata_schedule_177_0_e2164: f64 = (noise_metadata_schedule_177_0_e2162 * 1.3806503e-23);
        let noise_metadata_schedule_177_0_e2166: f64 = (noise_metadata_schedule_177_0_e2164 * w[11]);
        let noise_metadata_schedule_177_0_e2168: f64 = (noise_metadata_schedule_177_0_e2166 * params[71]);
        let noise_metadata_schedule_177_0_e2170: f64 = (noise_metadata_schedule_177_0_e2168 / w[115]);
        (noise_metadata_schedule_177_0_e2170,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_177_0_e2172;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_178_0_e2184,) = {
    if ((((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) && (w[116] == 0.0)) {
        (0.0,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_178_0_e2184;
        }
        if (active[0] & 0xc0) != 0 {
            let (noise_metadata_schedule_179_0_e2206,) = {
    if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) {
        let noise_metadata_schedule_179_0_e2193: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_179_0_e2195: f64 = (noise_metadata_schedule_179_0_e2193 * w[11]);
        let noise_metadata_schedule_179_0_e2197: f64 = (noise_metadata_schedule_179_0_e2195 * params[73]);
        let noise_metadata_schedule_179_0_e2199: f64 = (noise_metadata_schedule_179_0_e2197 * w[31]);
        let noise_metadata_schedule_179_0_e2202: f64 = (params[72] * params[71]);
        let noise_metadata_schedule_179_0_e2203: f64 = (noise_metadata_schedule_179_0_e2202).sqrt();
        let noise_metadata_schedule_179_0_e2204: f64 = (noise_metadata_schedule_179_0_e2199 * noise_metadata_schedule_179_0_e2203);
        (noise_metadata_schedule_179_0_e2204,)
    } else {
        (w[120],)
    }
};
            w[120] = noise_metadata_schedule_179_0_e2206;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_183_0_e2262,) = {
    if (((w[108] != 0.0) && (w[107] == 0.0)) && (params[0] != 0.0)) {
        let noise_metadata_schedule_183_0_e2252: f64 = (4.0 * 1.3806503e-23);
        let noise_metadata_schedule_183_0_e2254: f64 = (noise_metadata_schedule_183_0_e2252 * w[11]);
        let noise_metadata_schedule_183_0_e2256: f64 = (noise_metadata_schedule_183_0_e2254 * w[115]);
        let noise_metadata_schedule_183_0_e2258: f64 = (noise_metadata_schedule_183_0_e2256 * params[72]);
        let noise_metadata_schedule_183_0_e2260: f64 = (noise_metadata_schedule_183_0_e2258 * params[74]);
        (noise_metadata_schedule_183_0_e2260,)
    } else {
        (w[121],)
    }
};
            w[121] = noise_metadata_schedule_183_0_e2262;
        }
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 25] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 585];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            true
        };
        let noise_source_1_active = {
            true
        };
        let noise_source_2_active = {
            true
        };
        let noise_source_3_active = {
            true
        };
        let noise_source_4_active = {
            true
        };
        let noise_source_5_active = {
            true
        };
        let noise_source_6_active = {
            true
        };
        let noise_source_7_active = {
            true
        };
        let noise_source_8_active = {
            true
        };
        let noise_source_9_active = {
            true
        };
        let noise_source_10_active = {
            true
        };
        let noise_source_11_active = {
            true
        };
        let noise_source_12_active = {
            true
        };
        let noise_source_13_active = {
            true
        };
        let noise_source_14_active = {
            true
        };
        let noise_source_15_active = {
            w[579] != 0.0
        };
        let noise_source_16_active = {
            let noise_16_activation_e457: f64 = if (w[579] == 0.0) { 1.0 } else { 0.0 };
            noise_16_activation_e457 != 0.0
        };
        let noise_source_17_active = {
            let noise_17_activation_e467: f64 = if ((w[580] != 0.0) && (w[581] != 0.0)) { 1.0 } else { 0.0 };
            noise_17_activation_e467 != 0.0
        };
        let noise_source_18_active = {
            let noise_18_activation_e477: f64 = if ((w[580] != 0.0) && (w[581] != 0.0)) { 1.0 } else { 0.0 };
            noise_18_activation_e477 != 0.0
        };
        let noise_source_19_active = {
            let noise_19_activation_e487: f64 = if ((w[580] != 0.0) && (w[581] != 0.0)) { 1.0 } else { 0.0 };
            noise_19_activation_e487 != 0.0
        };
        let noise_source_20_active = {
            let noise_20_activation_e498: f64 = if ((w[580] != 0.0) && (w[581] == 0.0)) { 1.0 } else { 0.0 };
            noise_20_activation_e498 != 0.0
        };
        let noise_source_21_active = {
            let noise_21_activation_e509: f64 = if ((w[580] != 0.0) && (w[581] == 0.0)) { 1.0 } else { 0.0 };
            noise_21_activation_e509 != 0.0
        };
        let noise_source_22_active = {
            let noise_22_activation_e520: f64 = if ((w[580] == 0.0) && (w[582] != 0.0)) { 1.0 } else { 0.0 };
            noise_22_activation_e520 != 0.0
        };
        let noise_source_23_active = {
            let noise_23_activation_e531: f64 = if ((w[580] == 0.0) && (w[582] != 0.0)) { 1.0 } else { 0.0 };
            noise_23_activation_e531 != 0.0
        };
        let noise_source_24_active = {
            let noise_24_activation_e543: f64 = if ((w[580] == 0.0) && (w[582] == 0.0)) { 1.0 } else { 0.0 };
            noise_24_activation_e543 != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active, noise_source_4_active, noise_source_5_active, noise_source_6_active, noise_source_7_active, noise_source_8_active, noise_source_9_active, noise_source_10_active, noise_source_11_active, noise_source_12_active, noise_source_13_active, noise_source_14_active, noise_source_15_active, noise_source_16_active, noise_source_17_active, noise_source_18_active, noise_source_19_active, noise_source_20_active, noise_source_21_active, noise_source_22_active, noise_source_23_active, noise_source_24_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3) | ((noise_source_4_active as u128) << 4) | ((noise_source_5_active as u128) << 5) | ((noise_source_6_active as u128) << 6) | ((noise_source_7_active as u128) << 7) | ((noise_source_8_active as u128) << 8) | ((noise_source_9_active as u128) << 9) | ((noise_source_10_active as u128) << 10) | ((noise_source_11_active as u128) << 11) | ((noise_source_12_active as u128) << 12) | ((noise_source_13_active as u128) << 13) | ((noise_source_14_active as u128) << 14) | ((noise_source_15_active as u128) << 15) | ((noise_source_16_active as u128) << 16) | ((noise_source_17_active as u128) << 17) | ((noise_source_18_active as u128) << 18) | ((noise_source_19_active as u128) << 19) | ((noise_source_20_active as u128) << 20) | ((noise_source_21_active as u128) << 21) | ((noise_source_22_active as u128) << 22) | ((noise_source_23_active as u128) << 23) | ((noise_source_24_active as u128) << 24)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_2(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_3(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_4(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_5(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_6(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_7(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_8(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_9(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_10(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e7995: f64 = 1.0;
            let noise_0_psd_e361: f64 = (w[294] * params.p1);
            let noise_0_psd_e7996: f64 = (noise_0_psd_e7995 * noise_0_psd_e361);
            let psd = noise_0_psd_e7996;
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
            let noise_1_psd_e7998: f64 = 1.0;
            let noise_1_psd_e375: f64 = (w[306] * params.p1);
            let noise_1_psd_e7999: f64 = (noise_1_psd_e7998 * noise_1_psd_e375);
            let psd = noise_1_psd_e7999;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[2] {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_2_psd_e8001: f64 = 1.0;
            let noise_2_psd_e380: f64 = (w[295] * params.p1);
            let noise_2_psd_e8002: f64 = (noise_2_psd_e8001 * noise_2_psd_e380);
            let psd = noise_2_psd_e8002;
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
            let noise_3_psd_e8004: f64 = 1.0;
            let noise_3_psd_e385: f64 = (w[288] * params.p1);
            let noise_3_psd_e8005: f64 = (noise_3_psd_e8004 * noise_3_psd_e385);
            let psd = noise_3_psd_e8005;
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
            let noise_4_psd_e8007: f64 = 1.0;
            let noise_4_psd_e390: f64 = (w[289] * params.p1);
            let noise_4_psd_e8008: f64 = (noise_4_psd_e8007 * noise_4_psd_e390);
            let psd = noise_4_psd_e8008;
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
            let noise_5_psd_e8010: f64 = 1.0;
            let noise_5_psd_e395: f64 = (w[293] * params.p1);
            let noise_5_psd_e8011: f64 = (noise_5_psd_e8010 * noise_5_psd_e395);
            let psd = noise_5_psd_e8011;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[6] {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_6_psd_e8013: f64 = 1.0;
            let noise_6_psd_e400: f64 = (w[296] * params.p1);
            let noise_6_psd_e8014: f64 = (noise_6_psd_e8013 * noise_6_psd_e400);
            let psd = noise_6_psd_e8014;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = Some(1.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[7] {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_7_psd_e8016: f64 = 1.0;
            let noise_7_psd_e406: f64 = (w[297] * params.p1);
            let noise_7_psd_e8017: f64 = (noise_7_psd_e8016 * noise_7_psd_e406);
            let psd = noise_7_psd_e8017;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = Some(1.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[8] {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_8_psd_e8019: f64 = 1.0;
            let noise_8_psd_e412: f64 = (w[298] * params.p1);
            let noise_8_psd_e8020: f64 = (noise_8_psd_e8019 * noise_8_psd_e412);
            let psd = noise_8_psd_e8020;
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
            let noise_9_psd_e8022: f64 = 1.0;
            let noise_9_psd_e417: f64 = (w[299] * params.p1);
            let noise_9_psd_e8023: f64 = (noise_9_psd_e8022 * noise_9_psd_e417);
            let psd = noise_9_psd_e8023;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[10] {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_10_psd_e8025: f64 = 1.0;
            let noise_10_psd_e422: f64 = (w[300] * params.p1);
            let noise_10_psd_e8026: f64 = (noise_10_psd_e8025 * noise_10_psd_e422);
            let psd = noise_10_psd_e8026;
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
            let noise_11_psd_e8028: f64 = 1.0;
            let noise_11_psd_e428: f64 = (w[302] * params.p1);
            let noise_11_psd_e8029: f64 = (noise_11_psd_e8028 * noise_11_psd_e428);
            let psd = noise_11_psd_e8029;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[12] {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_12_psd_e8031: f64 = 1.0;
            let noise_12_psd_e433: f64 = (w[304] * params.p1);
            let noise_12_psd_e8032: f64 = (noise_12_psd_e8031 * noise_12_psd_e433);
            let psd = noise_12_psd_e8032;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = Some(1.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[13] {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_13_psd_e8034: f64 = 1.0;
            let noise_13_psd_e439: f64 = (w[303] * params.p1);
            let noise_13_psd_e8035: f64 = (noise_13_psd_e8034 * noise_13_psd_e439);
            let psd = noise_13_psd_e8035;
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
            let noise_14_psd_e8037: f64 = 1.0;
            let noise_14_psd_e444: f64 = (w[305] * params.p1);
            let noise_14_psd_e8038: f64 = (noise_14_psd_e8037 * noise_14_psd_e444);
            let psd = noise_14_psd_e8038;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = Some(1.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[15] {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_15_psd_e8040: f64 = 1.0;
            let noise_15_psd_e451: f64 = (w[301] * params.p1);
            let noise_15_psd_e8041: f64 = (noise_15_psd_e8040 * noise_15_psd_e451);
            let psd = noise_15_psd_e8041;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[16] {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_16_psd_e8043: f64 = 1.0;
            let noise_16_psd_e460: f64 = (w[301] * params.p1);
            let noise_16_psd_e8044: f64 = (noise_16_psd_e8043 * noise_16_psd_e460);
            let psd = noise_16_psd_e8044;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[17] {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_17_psd_e8046: f64 = 1.0;
            let noise_17_psd_e470: f64 = (w[290] * params.p1);
            let noise_17_psd_e8047: f64 = (noise_17_psd_e8046 * noise_17_psd_e470);
            let psd = noise_17_psd_e8047;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[18] {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_18_psd_e8049: f64 = 1.0;
            let noise_18_psd_e480: f64 = (w[291] * params.p1);
            let noise_18_psd_e8050: f64 = (noise_18_psd_e8049 * noise_18_psd_e480);
            let psd = noise_18_psd_e8050;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[19] {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_19_psd_e8052: f64 = 1.0;
            let noise_19_psd_e490: f64 = (w[292] * params.p1);
            let noise_19_psd_e8053: f64 = (noise_19_psd_e8052 * noise_19_psd_e490);
            let psd = noise_19_psd_e8053;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[20] {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_20_psd_e8055: f64 = 1.0;
            let noise_20_psd_e501: f64 = (w[290] * params.p1);
            let noise_20_psd_e8056: f64 = (noise_20_psd_e8055 * noise_20_psd_e501);
            let psd = noise_20_psd_e8056;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[21] {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_21_psd_e8058: f64 = 1.0;
            let noise_21_psd_e512: f64 = (w[291] * params.p1);
            let noise_21_psd_e8059: f64 = (noise_21_psd_e8058 * noise_21_psd_e512);
            let psd = noise_21_psd_e8059;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 21, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[22] {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_22_psd_e8061: f64 = 1.0;
            let noise_22_psd_e523: f64 = (w[290] * params.p1);
            let noise_22_psd_e8062: f64 = (noise_22_psd_e8061 * noise_22_psd_e523);
            let psd = noise_22_psd_e8062;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 22, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[23] {
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_23_psd_e8064: f64 = 1.0;
            let noise_23_psd_e534: f64 = (w[292] * params.p1);
            let noise_23_psd_e8065: f64 = (noise_23_psd_e8064 * noise_23_psd_e534);
            let psd = noise_23_psd_e8065;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 23, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 23, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(23, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[24] {
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_24_psd_e8067: f64 = 1.0;
            let noise_24_psd_e546: f64 = (w[290] * params.p1);
            let noise_24_psd_e8068: f64 = (noise_24_psd_e8067 * noise_24_psd_e546);
            let psd = noise_24_psd_e8068;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 24, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585]) {
        let params = &*self.params;
        let noise_activation_schedule_702_0_e6945: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
        w[579] = noise_activation_schedule_702_0_e6945;
        let noise_activation_schedule_703_0_e6948: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
        w[580] = noise_activation_schedule_703_0_e6948;
        let noise_activation_schedule_704_0_e6951: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
        w[581] = noise_activation_schedule_704_0_e6951;
        let noise_activation_schedule_705_0_e6954: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
        w[582] = noise_activation_schedule_705_0_e6954;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_0_0_e553: f64 = if params.p3 == 1.0 { 1.0 } else { 0.0 };
            w[447] = noise_metadata_schedule_0_0_e553;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_1_0_e557,) = {
    if (w[447] != 0.0) {
        (70300000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_1_0_e557;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_2_0_e561,) = {
    if (w[447] != 0.0) {
        (123000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_2_0_e561;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3_0_e566,) = {
    if (w[447] == 0.0) {
        (158000000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_3_0_e566;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_4_0_e571,) = {
    if (w[447] == 0.0) {
        (204000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_4_0_e571;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_5_0_e574: f64 = (1.0 - params.p32);
            w[153] = noise_metadata_schedule_5_0_e574;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_6_0_e577: f64 = (params.p4 + 273.15);
            w[3] = noise_metadata_schedule_6_0_e577;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_7_0_e578: f64 = ctx.temperature();
            let noise_metadata_schedule_7_0_e580: f64 = (noise_metadata_schedule_7_0_e578 + params.p0);
            w[5] = noise_metadata_schedule_7_0_e580;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let noise_metadata_schedule_9_0_e586: f64 = if params.p141 == 0.0 { 1.0 } else { 0.0 };
            w[448] = noise_metadata_schedule_9_0_e586;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let (noise_metadata_schedule_10_0_e590,) = {
    if (w[448] != 0.0) {
        (1e-12,)
    } else {
        (w[321],)
    }
};
            w[321] = noise_metadata_schedule_10_0_e590;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let (noise_metadata_schedule_11_0_e595,) = {
    if (w[448] == 0.0) {
        (params.p141,)
    } else {
        (w[321],)
    }
};
            w[321] = noise_metadata_schedule_11_0_e595;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let noise_metadata_schedule_12_0_e598: f64 = (w[321] * params.p1);
            w[322] = noise_metadata_schedule_12_0_e598;
        }
        if (active[0] & 0x1fe0000) != 0 {
            let noise_metadata_schedule_13_0_e601: f64 = (1.0 / w[322]);
            w[323] = noise_metadata_schedule_13_0_e601;
        }
        if (active[0] & 0x1ffe7) != 0 {
            w[52] = 0.001;
        }
        if (active[0] & 0x1ffe7) != 0 {
            w[318] = 0.001;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_16_0_e607: f64 = (2.0 - params.p66);
            let noise_metadata_schedule_16_0_e608: f64 = (2.0_f64).powf(noise_metadata_schedule_16_0_e607);
            w[62] = noise_metadata_schedule_16_0_e608;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_17_0_e611: f64 = (1.0 / w[62]);
            w[63] = noise_metadata_schedule_17_0_e611;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_18_0_e615: f64 = (params.p114 * w[3]);
            let noise_metadata_schedule_18_0_e617: f64 = (noise_metadata_schedule_18_0_e615 * w[3]);
            let noise_metadata_schedule_18_0_e620: f64 = (w[3] + params.p115);
            let noise_metadata_schedule_18_0_e621: f64 = (noise_metadata_schedule_18_0_e617 / noise_metadata_schedule_18_0_e620);
            let noise_metadata_schedule_18_0_e622: f64 = (params.p113 + noise_metadata_schedule_18_0_e621);
            let noise_metadata_schedule_18_0_e624: f64 = (noise_metadata_schedule_18_0_e622 - 0.05);
            let noise_metadata_schedule_18_0_e626: f64 = (noise_metadata_schedule_18_0_e624 / 0.1);
            w[265] = noise_metadata_schedule_18_0_e626;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_19_0_e630: f64 = (params.p114 * w[3]);
            let noise_metadata_schedule_19_0_e632: f64 = (noise_metadata_schedule_19_0_e630 * w[3]);
            let noise_metadata_schedule_19_0_e635: f64 = (w[3] + params.p115);
            let noise_metadata_schedule_19_0_e636: f64 = (noise_metadata_schedule_19_0_e632 / noise_metadata_schedule_19_0_e635);
            let noise_metadata_schedule_19_0_e637: f64 = (params.p113 + noise_metadata_schedule_19_0_e636);
            let noise_metadata_schedule_19_0_e639: f64 = if noise_metadata_schedule_19_0_e637 < 0.05 { 1.0 } else { 0.0 };
            w[449] = noise_metadata_schedule_19_0_e639;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_20_0_e651,) = {
    if (w[449] != 0.0) {
        let noise_metadata_schedule_20_0_e645: f64 = (w[265]).exp();
        let noise_metadata_schedule_20_0_e646: f64 = (1.0 + noise_metadata_schedule_20_0_e645);
        let noise_metadata_schedule_20_0_e647: f64 = (noise_metadata_schedule_20_0_e646).ln();
        let noise_metadata_schedule_20_0_e648: f64 = (0.1 * noise_metadata_schedule_20_0_e647);
        let noise_metadata_schedule_20_0_e649: f64 = (0.05 + noise_metadata_schedule_20_0_e648);
        (noise_metadata_schedule_20_0_e649,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_20_0_e651;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_21_0_e675,) = {
    if (w[449] == 0.0) {
        let noise_metadata_schedule_21_0_e657: f64 = (params.p114 * w[3]);
        let noise_metadata_schedule_21_0_e659: f64 = (noise_metadata_schedule_21_0_e657 * w[3]);
        let noise_metadata_schedule_21_0_e662: f64 = (w[3] + params.p115);
        let noise_metadata_schedule_21_0_e663: f64 = (noise_metadata_schedule_21_0_e659 / noise_metadata_schedule_21_0_e662);
        let noise_metadata_schedule_21_0_e664: f64 = (params.p113 + noise_metadata_schedule_21_0_e663);
        let noise_metadata_schedule_21_0_e668: f64 = (-w[265]);
        let noise_metadata_schedule_21_0_e669: f64 = (noise_metadata_schedule_21_0_e668).exp();
        let noise_metadata_schedule_21_0_e670: f64 = (1.0 + noise_metadata_schedule_21_0_e669);
        let noise_metadata_schedule_21_0_e671: f64 = (noise_metadata_schedule_21_0_e670).ln();
        let noise_metadata_schedule_21_0_e672: f64 = (0.1 * noise_metadata_schedule_21_0_e671);
        let noise_metadata_schedule_21_0_e673: f64 = (noise_metadata_schedule_21_0_e664 + noise_metadata_schedule_21_0_e672);
        (noise_metadata_schedule_21_0_e673,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_21_0_e675;
        }
        if (active[0] & 0x18006) != 0 {
            w[71] = params.p113;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_23_0_e679: f64 = (1.0 / w[71]);
            w[72] = noise_metadata_schedule_23_0_e679;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_24_0_e682: f64 = (1.0 / params.p65);
            w[64] = noise_metadata_schedule_24_0_e682;
        }
        if (active[0] & 0x18002) != 0 {
            w[75] = params.p70;
        }
        if (active[0] & 0x18002) != 0 {
            w[76] = params.p71;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_27_0_e688: f64 = (2.0 - w[76]);
            let noise_metadata_schedule_27_0_e689: f64 = (2.0_f64).powf(noise_metadata_schedule_27_0_e688);
            w[79] = noise_metadata_schedule_27_0_e689;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_28_0_e692: f64 = (1.0 / w[79]);
            w[89] = noise_metadata_schedule_28_0_e692;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_29_0_e696: f64 = (params.p117 * w[3]);
            let noise_metadata_schedule_29_0_e698: f64 = (noise_metadata_schedule_29_0_e696 * w[3]);
            let noise_metadata_schedule_29_0_e701: f64 = (w[3] + params.p118);
            let noise_metadata_schedule_29_0_e702: f64 = (noise_metadata_schedule_29_0_e698 / noise_metadata_schedule_29_0_e701);
            let noise_metadata_schedule_29_0_e703: f64 = (params.p116 + noise_metadata_schedule_29_0_e702);
            let noise_metadata_schedule_29_0_e705: f64 = (noise_metadata_schedule_29_0_e703 - 0.05);
            let noise_metadata_schedule_29_0_e707: f64 = (noise_metadata_schedule_29_0_e705 / 0.1);
            w[265] = noise_metadata_schedule_29_0_e707;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_30_0_e711: f64 = (params.p117 * w[3]);
            let noise_metadata_schedule_30_0_e713: f64 = (noise_metadata_schedule_30_0_e711 * w[3]);
            let noise_metadata_schedule_30_0_e716: f64 = (w[3] + params.p118);
            let noise_metadata_schedule_30_0_e717: f64 = (noise_metadata_schedule_30_0_e713 / noise_metadata_schedule_30_0_e716);
            let noise_metadata_schedule_30_0_e718: f64 = (params.p116 + noise_metadata_schedule_30_0_e717);
            let noise_metadata_schedule_30_0_e720: f64 = if noise_metadata_schedule_30_0_e718 < 0.05 { 1.0 } else { 0.0 };
            w[450] = noise_metadata_schedule_30_0_e720;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_31_0_e732,) = {
    if (w[450] != 0.0) {
        let noise_metadata_schedule_31_0_e726: f64 = (w[265]).exp();
        let noise_metadata_schedule_31_0_e727: f64 = (1.0 + noise_metadata_schedule_31_0_e726);
        let noise_metadata_schedule_31_0_e728: f64 = (noise_metadata_schedule_31_0_e727).ln();
        let noise_metadata_schedule_31_0_e729: f64 = (0.1 * noise_metadata_schedule_31_0_e728);
        let noise_metadata_schedule_31_0_e730: f64 = (0.05 + noise_metadata_schedule_31_0_e729);
        (noise_metadata_schedule_31_0_e730,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_31_0_e732;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_32_0_e756,) = {
    if (w[450] == 0.0) {
        let noise_metadata_schedule_32_0_e738: f64 = (params.p117 * w[3]);
        let noise_metadata_schedule_32_0_e740: f64 = (noise_metadata_schedule_32_0_e738 * w[3]);
        let noise_metadata_schedule_32_0_e743: f64 = (w[3] + params.p118);
        let noise_metadata_schedule_32_0_e744: f64 = (noise_metadata_schedule_32_0_e740 / noise_metadata_schedule_32_0_e743);
        let noise_metadata_schedule_32_0_e745: f64 = (params.p116 + noise_metadata_schedule_32_0_e744);
        let noise_metadata_schedule_32_0_e749: f64 = (-w[265]);
        let noise_metadata_schedule_32_0_e750: f64 = (noise_metadata_schedule_32_0_e749).exp();
        let noise_metadata_schedule_32_0_e751: f64 = (1.0 + noise_metadata_schedule_32_0_e750);
        let noise_metadata_schedule_32_0_e752: f64 = (noise_metadata_schedule_32_0_e751).ln();
        let noise_metadata_schedule_32_0_e753: f64 = (0.1 * noise_metadata_schedule_32_0_e752);
        let noise_metadata_schedule_32_0_e754: f64 = (noise_metadata_schedule_32_0_e745 + noise_metadata_schedule_32_0_e753);
        (noise_metadata_schedule_32_0_e754,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_32_0_e756;
        }
        if (active[0] & 0x18002) != 0 {
            w[87] = params.p116;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_34_0_e760: f64 = (1.0 / w[87]);
            w[86] = noise_metadata_schedule_34_0_e760;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_35_0_e763: f64 = (1.0 / w[75]);
            w[66] = noise_metadata_schedule_35_0_e763;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_36_0_e767: f64 = (1.0 / params.p82);
            let noise_metadata_schedule_36_0_e768: f64 = (1.0 - noise_metadata_schedule_36_0_e767);
            w[324] = noise_metadata_schedule_36_0_e768;
        }
        if (active[0] & 0x44) != 0 {
            w[154] = 0.0;
        }
        if (active[0] & 0x140) != 0 {
            w[155] = 0.0;
        }
        if (active[0] & 0x6000) != 0 {
            w[172] = 0.0;
        }
        if (active[0] & 0x6000) != 0 {
            w[171] = 1.0;
        }
        if (active[0] & 0x2) != 0 {
            w[199] = 0.0;
        }
        if (active[0] & 0x2) != 0 {
            w[201] = 0.0;
        }
        if (active[0] & 0x44) != 0 {
            w[53] = 0.0;
        }
        if (active[0] & 0x44) != 0 {
            w[54] = 0.0;
        }
        if (active[0] & 0x140) != 0 {
            w[45] = 0.0;
        }
        if (active[0] & 0x1ffffff) != 0 {
            w[207] = (ctx.node_voltage(self.nodes[3]) - 0.0);
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_51_0_e785: f64 = if w[207] < 0.0 { 1.0 } else { 0.0 };
            w[451] = noise_metadata_schedule_51_0_e785;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let (noise_metadata_schedule_52_0_e793,) = {
    if (w[451] != 0.0) {
        let noise_metadata_schedule_52_0_e789: f64 = (1.0 - w[207]);
        let noise_metadata_schedule_52_0_e790: f64 = (noise_metadata_schedule_52_0_e789).ln();
        let noise_metadata_schedule_52_0_e791: f64 = (-noise_metadata_schedule_52_0_e790);
        (noise_metadata_schedule_52_0_e791,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_52_0_e793;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_53_0_e796: f64 = if w[207] < params.p124 { 1.0 } else { 0.0 };
            w[452] = noise_metadata_schedule_53_0_e796;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let (noise_metadata_schedule_54_0_e800,) = {
    if (w[452] != 0.0) {
        (w[207],)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_54_0_e800;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let (noise_metadata_schedule_55_0_e812,) = {
    if (w[452] == 0.0) {
        let noise_metadata_schedule_55_0_e807: f64 = (w[207] - params.p124);
        let noise_metadata_schedule_55_0_e808: f64 = (1.0 + noise_metadata_schedule_55_0_e807);
        let noise_metadata_schedule_55_0_e809: f64 = (noise_metadata_schedule_55_0_e808).ln();
        let noise_metadata_schedule_55_0_e810: f64 = (params.p124 + noise_metadata_schedule_55_0_e809);
        (noise_metadata_schedule_55_0_e810,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_55_0_e812;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_56_0_e815: f64 = (w[5] + w[11]);
            w[2] = noise_metadata_schedule_56_0_e815;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_57_0_e818: f64 = (w[2] / w[3]);
            w[4] = noise_metadata_schedule_57_0_e818;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_58_0_e821: f64 = (8.617086918058125e-5 * w[2]);
            w[6] = noise_metadata_schedule_58_0_e821;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_59_0_e824: f64 = (8.617086918058125e-5 * w[3]);
            w[7] = noise_metadata_schedule_59_0_e824;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_60_0_e827: f64 = (1.0 / w[6]);
            w[8] = noise_metadata_schedule_60_0_e827;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_61_0_e830: f64 = (1.0 / w[7]);
            w[9] = noise_metadata_schedule_61_0_e830;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_62_0_e833: f64 = (w[8] - w[9]);
            w[10] = noise_metadata_schedule_62_0_e833;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_63_0_e836: f64 = (w[2] - w[3]);
            w[12] = noise_metadata_schedule_63_0_e836;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_64_0_e838: f64 = (w[4]).ln();
            w[260] = noise_metadata_schedule_64_0_e838;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_65_0_e842: f64 = (params.p114 * w[2]);
            let noise_metadata_schedule_65_0_e844: f64 = (noise_metadata_schedule_65_0_e842 * w[2]);
            let noise_metadata_schedule_65_0_e847: f64 = (w[2] + params.p115);
            let noise_metadata_schedule_65_0_e848: f64 = (noise_metadata_schedule_65_0_e844 / noise_metadata_schedule_65_0_e847);
            let noise_metadata_schedule_65_0_e849: f64 = (w[74] - noise_metadata_schedule_65_0_e848);
            let noise_metadata_schedule_65_0_e851: f64 = (noise_metadata_schedule_65_0_e849 - 0.05);
            let noise_metadata_schedule_65_0_e853: f64 = (noise_metadata_schedule_65_0_e851 / 0.1);
            w[265] = noise_metadata_schedule_65_0_e853;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_66_0_e857: f64 = (params.p114 * w[2]);
            let noise_metadata_schedule_66_0_e859: f64 = (noise_metadata_schedule_66_0_e857 * w[2]);
            let noise_metadata_schedule_66_0_e862: f64 = (w[2] + params.p115);
            let noise_metadata_schedule_66_0_e863: f64 = (noise_metadata_schedule_66_0_e859 / noise_metadata_schedule_66_0_e862);
            let noise_metadata_schedule_66_0_e864: f64 = (w[74] - noise_metadata_schedule_66_0_e863);
            let noise_metadata_schedule_66_0_e866: f64 = if noise_metadata_schedule_66_0_e864 < 0.05 { 1.0 } else { 0.0 };
            w[453] = noise_metadata_schedule_66_0_e866;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_67_0_e878,) = {
    if (w[453] != 0.0) {
        let noise_metadata_schedule_67_0_e872: f64 = (w[265]).exp();
        let noise_metadata_schedule_67_0_e873: f64 = (1.0 + noise_metadata_schedule_67_0_e872);
        let noise_metadata_schedule_67_0_e874: f64 = (noise_metadata_schedule_67_0_e873).ln();
        let noise_metadata_schedule_67_0_e875: f64 = (0.1 * noise_metadata_schedule_67_0_e874);
        let noise_metadata_schedule_67_0_e876: f64 = (0.05 + noise_metadata_schedule_67_0_e875);
        (noise_metadata_schedule_67_0_e876,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_67_0_e878;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_68_0_e902,) = {
    if (w[453] == 0.0) {
        let noise_metadata_schedule_68_0_e884: f64 = (params.p114 * w[2]);
        let noise_metadata_schedule_68_0_e886: f64 = (noise_metadata_schedule_68_0_e884 * w[2]);
        let noise_metadata_schedule_68_0_e889: f64 = (w[2] + params.p115);
        let noise_metadata_schedule_68_0_e890: f64 = (noise_metadata_schedule_68_0_e886 / noise_metadata_schedule_68_0_e889);
        let noise_metadata_schedule_68_0_e891: f64 = (w[74] - noise_metadata_schedule_68_0_e890);
        let noise_metadata_schedule_68_0_e895: f64 = (-w[265]);
        let noise_metadata_schedule_68_0_e896: f64 = (noise_metadata_schedule_68_0_e895).exp();
        let noise_metadata_schedule_68_0_e897: f64 = (1.0 + noise_metadata_schedule_68_0_e896);
        let noise_metadata_schedule_68_0_e898: f64 = (noise_metadata_schedule_68_0_e897).ln();
        let noise_metadata_schedule_68_0_e899: f64 = (0.1 * noise_metadata_schedule_68_0_e898);
        let noise_metadata_schedule_68_0_e900: f64 = (noise_metadata_schedule_68_0_e891 + noise_metadata_schedule_68_0_e899);
        (noise_metadata_schedule_68_0_e900,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_68_0_e902;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_69_0_e906: f64 = (params.p117 * w[2]);
            let noise_metadata_schedule_69_0_e908: f64 = (noise_metadata_schedule_69_0_e906 * w[2]);
            let noise_metadata_schedule_69_0_e911: f64 = (w[2] + params.p118);
            let noise_metadata_schedule_69_0_e912: f64 = (noise_metadata_schedule_69_0_e908 / noise_metadata_schedule_69_0_e911);
            let noise_metadata_schedule_69_0_e913: f64 = (w[88] - noise_metadata_schedule_69_0_e912);
            let noise_metadata_schedule_69_0_e915: f64 = (noise_metadata_schedule_69_0_e913 - 0.05);
            let noise_metadata_schedule_69_0_e917: f64 = (noise_metadata_schedule_69_0_e915 / 0.1);
            w[265] = noise_metadata_schedule_69_0_e917;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_70_0_e921: f64 = (params.p117 * w[2]);
            let noise_metadata_schedule_70_0_e923: f64 = (noise_metadata_schedule_70_0_e921 * w[2]);
            let noise_metadata_schedule_70_0_e926: f64 = (w[2] + params.p118);
            let noise_metadata_schedule_70_0_e927: f64 = (noise_metadata_schedule_70_0_e923 / noise_metadata_schedule_70_0_e926);
            let noise_metadata_schedule_70_0_e928: f64 = (w[88] - noise_metadata_schedule_70_0_e927);
            let noise_metadata_schedule_70_0_e930: f64 = if noise_metadata_schedule_70_0_e928 < 0.05 { 1.0 } else { 0.0 };
            w[454] = noise_metadata_schedule_70_0_e930;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_71_0_e942,) = {
    if (w[454] != 0.0) {
        let noise_metadata_schedule_71_0_e936: f64 = (w[265]).exp();
        let noise_metadata_schedule_71_0_e937: f64 = (1.0 + noise_metadata_schedule_71_0_e936);
        let noise_metadata_schedule_71_0_e938: f64 = (noise_metadata_schedule_71_0_e937).ln();
        let noise_metadata_schedule_71_0_e939: f64 = (0.1 * noise_metadata_schedule_71_0_e938);
        let noise_metadata_schedule_71_0_e940: f64 = (0.05 + noise_metadata_schedule_71_0_e939);
        (noise_metadata_schedule_71_0_e940,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_71_0_e942;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_72_0_e966,) = {
    if (w[454] == 0.0) {
        let noise_metadata_schedule_72_0_e948: f64 = (params.p117 * w[2]);
        let noise_metadata_schedule_72_0_e950: f64 = (noise_metadata_schedule_72_0_e948 * w[2]);
        let noise_metadata_schedule_72_0_e953: f64 = (w[2] + params.p118);
        let noise_metadata_schedule_72_0_e954: f64 = (noise_metadata_schedule_72_0_e950 / noise_metadata_schedule_72_0_e953);
        let noise_metadata_schedule_72_0_e955: f64 = (w[88] - noise_metadata_schedule_72_0_e954);
        let noise_metadata_schedule_72_0_e959: f64 = (-w[265]);
        let noise_metadata_schedule_72_0_e960: f64 = (noise_metadata_schedule_72_0_e959).exp();
        let noise_metadata_schedule_72_0_e961: f64 = (1.0 + noise_metadata_schedule_72_0_e960);
        let noise_metadata_schedule_72_0_e962: f64 = (noise_metadata_schedule_72_0_e961).ln();
        let noise_metadata_schedule_72_0_e963: f64 = (0.1 * noise_metadata_schedule_72_0_e962);
        let noise_metadata_schedule_72_0_e964: f64 = (noise_metadata_schedule_72_0_e955 + noise_metadata_schedule_72_0_e963);
        (noise_metadata_schedule_72_0_e964,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_72_0_e966;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_73_0_e968: f64 = (-3.0);
            let noise_metadata_schedule_73_0_e970: f64 = (noise_metadata_schedule_73_0_e968 * w[6]);
            let noise_metadata_schedule_73_0_e972: f64 = (noise_metadata_schedule_73_0_e970 * w[260]);
            let noise_metadata_schedule_73_0_e975: f64 = (params.p65 * w[4]);
            let noise_metadata_schedule_73_0_e976: f64 = (noise_metadata_schedule_73_0_e972 + noise_metadata_schedule_73_0_e975);
            let noise_metadata_schedule_73_0_e979: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_73_0_e981: f64 = (noise_metadata_schedule_73_0_e979 * params.p104);
            let noise_metadata_schedule_73_0_e982: f64 = (noise_metadata_schedule_73_0_e976 + noise_metadata_schedule_73_0_e981);
            w[13] = noise_metadata_schedule_73_0_e982;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_74_0_e985: f64 = (0.05 - w[13]);
            let noise_metadata_schedule_74_0_e987: f64 = (noise_metadata_schedule_74_0_e985 / w[6]);
            w[265] = noise_metadata_schedule_74_0_e987;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_75_0_e990: f64 = if 0.05 < w[13] { 1.0 } else { 0.0 };
            w[455] = noise_metadata_schedule_75_0_e990;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_76_0_e1002,) = {
    if (w[455] != 0.0) {
        let noise_metadata_schedule_76_0_e996: f64 = (w[265]).exp();
        let noise_metadata_schedule_76_0_e997: f64 = (1.0 + noise_metadata_schedule_76_0_e996);
        let noise_metadata_schedule_76_0_e998: f64 = (noise_metadata_schedule_76_0_e997).ln();
        let noise_metadata_schedule_76_0_e999: f64 = (w[6] * noise_metadata_schedule_76_0_e998);
        let noise_metadata_schedule_76_0_e1000: f64 = (w[13] + noise_metadata_schedule_76_0_e999);
        (noise_metadata_schedule_76_0_e1000,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_76_0_e1002;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_77_0_e1016,) = {
    if (w[455] == 0.0) {
        let noise_metadata_schedule_77_0_e1009: f64 = (-w[265]);
        let noise_metadata_schedule_77_0_e1010: f64 = (noise_metadata_schedule_77_0_e1009).exp();
        let noise_metadata_schedule_77_0_e1011: f64 = (1.0 + noise_metadata_schedule_77_0_e1010);
        let noise_metadata_schedule_77_0_e1012: f64 = (noise_metadata_schedule_77_0_e1011).ln();
        let noise_metadata_schedule_77_0_e1013: f64 = (w[6] * noise_metadata_schedule_77_0_e1012);
        let noise_metadata_schedule_77_0_e1014: f64 = (0.05 + noise_metadata_schedule_77_0_e1013);
        (noise_metadata_schedule_77_0_e1014,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_77_0_e1016;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_78_0_e1018: f64 = (-3.0);
            let noise_metadata_schedule_78_0_e1020: f64 = (noise_metadata_schedule_78_0_e1018 * w[6]);
            let noise_metadata_schedule_78_0_e1022: f64 = (noise_metadata_schedule_78_0_e1020 * w[260]);
            let noise_metadata_schedule_78_0_e1025: f64 = (params.p63 * w[4]);
            let noise_metadata_schedule_78_0_e1026: f64 = (noise_metadata_schedule_78_0_e1022 + noise_metadata_schedule_78_0_e1025);
            let noise_metadata_schedule_78_0_e1029: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_78_0_e1031: f64 = (noise_metadata_schedule_78_0_e1029 * params.p109);
            let noise_metadata_schedule_78_0_e1032: f64 = (noise_metadata_schedule_78_0_e1026 + noise_metadata_schedule_78_0_e1031);
            w[15] = noise_metadata_schedule_78_0_e1032;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_79_0_e1035: f64 = (0.05 - w[15]);
            let noise_metadata_schedule_79_0_e1037: f64 = (noise_metadata_schedule_79_0_e1035 / w[6]);
            w[265] = noise_metadata_schedule_79_0_e1037;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_80_0_e1040: f64 = if 0.05 < w[15] { 1.0 } else { 0.0 };
            w[456] = noise_metadata_schedule_80_0_e1040;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_81_0_e1052,) = {
    if (w[456] != 0.0) {
        let noise_metadata_schedule_81_0_e1046: f64 = (w[265]).exp();
        let noise_metadata_schedule_81_0_e1047: f64 = (1.0 + noise_metadata_schedule_81_0_e1046);
        let noise_metadata_schedule_81_0_e1048: f64 = (noise_metadata_schedule_81_0_e1047).ln();
        let noise_metadata_schedule_81_0_e1049: f64 = (w[6] * noise_metadata_schedule_81_0_e1048);
        let noise_metadata_schedule_81_0_e1050: f64 = (w[15] + noise_metadata_schedule_81_0_e1049);
        (noise_metadata_schedule_81_0_e1050,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_81_0_e1052;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_82_0_e1066,) = {
    if (w[456] == 0.0) {
        let noise_metadata_schedule_82_0_e1059: f64 = (-w[265]);
        let noise_metadata_schedule_82_0_e1060: f64 = (noise_metadata_schedule_82_0_e1059).exp();
        let noise_metadata_schedule_82_0_e1061: f64 = (1.0 + noise_metadata_schedule_82_0_e1060);
        let noise_metadata_schedule_82_0_e1062: f64 = (noise_metadata_schedule_82_0_e1061).ln();
        let noise_metadata_schedule_82_0_e1063: f64 = (w[6] * noise_metadata_schedule_82_0_e1062);
        let noise_metadata_schedule_82_0_e1064: f64 = (0.05 + noise_metadata_schedule_82_0_e1063);
        (noise_metadata_schedule_82_0_e1064,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_82_0_e1066;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_88_0_e1118: f64 = (-3.0);
            let noise_metadata_schedule_88_0_e1120: f64 = (noise_metadata_schedule_88_0_e1118 * w[6]);
            let noise_metadata_schedule_88_0_e1122: f64 = (noise_metadata_schedule_88_0_e1120 * w[260]);
            let noise_metadata_schedule_88_0_e1125: f64 = (params.p70 * w[4]);
            let noise_metadata_schedule_88_0_e1126: f64 = (noise_metadata_schedule_88_0_e1122 + noise_metadata_schedule_88_0_e1125);
            let noise_metadata_schedule_88_0_e1129: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_88_0_e1131: f64 = (noise_metadata_schedule_88_0_e1129 * params.p109);
            let noise_metadata_schedule_88_0_e1132: f64 = (noise_metadata_schedule_88_0_e1126 + noise_metadata_schedule_88_0_e1131);
            w[18] = noise_metadata_schedule_88_0_e1132;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_89_0_e1135: f64 = (0.05 - w[18]);
            let noise_metadata_schedule_89_0_e1137: f64 = (noise_metadata_schedule_89_0_e1135 / w[6]);
            w[265] = noise_metadata_schedule_89_0_e1137;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_90_0_e1140: f64 = if 0.05 < w[18] { 1.0 } else { 0.0 };
            w[458] = noise_metadata_schedule_90_0_e1140;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_91_0_e1152,) = {
    if (w[458] != 0.0) {
        let noise_metadata_schedule_91_0_e1146: f64 = (w[265]).exp();
        let noise_metadata_schedule_91_0_e1147: f64 = (1.0 + noise_metadata_schedule_91_0_e1146);
        let noise_metadata_schedule_91_0_e1148: f64 = (noise_metadata_schedule_91_0_e1147).ln();
        let noise_metadata_schedule_91_0_e1149: f64 = (w[6] * noise_metadata_schedule_91_0_e1148);
        let noise_metadata_schedule_91_0_e1150: f64 = (w[18] + noise_metadata_schedule_91_0_e1149);
        (noise_metadata_schedule_91_0_e1150,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_91_0_e1152;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_92_0_e1166,) = {
    if (w[458] == 0.0) {
        let noise_metadata_schedule_92_0_e1159: f64 = (-w[265]);
        let noise_metadata_schedule_92_0_e1160: f64 = (noise_metadata_schedule_92_0_e1159).exp();
        let noise_metadata_schedule_92_0_e1161: f64 = (1.0 + noise_metadata_schedule_92_0_e1160);
        let noise_metadata_schedule_92_0_e1162: f64 = (noise_metadata_schedule_92_0_e1161).ln();
        let noise_metadata_schedule_92_0_e1163: f64 = (w[6] * noise_metadata_schedule_92_0_e1162);
        let noise_metadata_schedule_92_0_e1164: f64 = (0.05 + noise_metadata_schedule_92_0_e1163);
        (noise_metadata_schedule_92_0_e1164,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_92_0_e1166;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_93_0_e1168: f64 = (-3.0);
            let noise_metadata_schedule_93_0_e1170: f64 = (noise_metadata_schedule_93_0_e1168 * w[6]);
            let noise_metadata_schedule_93_0_e1172: f64 = (noise_metadata_schedule_93_0_e1170 * w[260]);
            let noise_metadata_schedule_93_0_e1175: f64 = (w[75] * w[4]);
            let noise_metadata_schedule_93_0_e1176: f64 = (noise_metadata_schedule_93_0_e1172 + noise_metadata_schedule_93_0_e1175);
            let noise_metadata_schedule_93_0_e1179: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_93_0_e1181: f64 = (noise_metadata_schedule_93_0_e1179 * params.p109);
            let noise_metadata_schedule_93_0_e1182: f64 = (noise_metadata_schedule_93_0_e1176 + noise_metadata_schedule_93_0_e1181);
            w[20] = noise_metadata_schedule_93_0_e1182;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_94_0_e1185: f64 = (0.05 - w[20]);
            let noise_metadata_schedule_94_0_e1187: f64 = (noise_metadata_schedule_94_0_e1185 / w[6]);
            w[265] = noise_metadata_schedule_94_0_e1187;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_95_0_e1190: f64 = if 0.05 < w[20] { 1.0 } else { 0.0 };
            w[459] = noise_metadata_schedule_95_0_e1190;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_96_0_e1202,) = {
    if (w[459] != 0.0) {
        let noise_metadata_schedule_96_0_e1196: f64 = (w[265]).exp();
        let noise_metadata_schedule_96_0_e1197: f64 = (1.0 + noise_metadata_schedule_96_0_e1196);
        let noise_metadata_schedule_96_0_e1198: f64 = (noise_metadata_schedule_96_0_e1197).ln();
        let noise_metadata_schedule_96_0_e1199: f64 = (w[6] * noise_metadata_schedule_96_0_e1198);
        let noise_metadata_schedule_96_0_e1200: f64 = (w[20] + noise_metadata_schedule_96_0_e1199);
        (noise_metadata_schedule_96_0_e1200,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_96_0_e1202;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_97_0_e1216,) = {
    if (w[459] == 0.0) {
        let noise_metadata_schedule_97_0_e1209: f64 = (-w[265]);
        let noise_metadata_schedule_97_0_e1210: f64 = (noise_metadata_schedule_97_0_e1209).exp();
        let noise_metadata_schedule_97_0_e1211: f64 = (1.0 + noise_metadata_schedule_97_0_e1210);
        let noise_metadata_schedule_97_0_e1212: f64 = (noise_metadata_schedule_97_0_e1211).ln();
        let noise_metadata_schedule_97_0_e1213: f64 = (w[6] * noise_metadata_schedule_97_0_e1212);
        let noise_metadata_schedule_97_0_e1214: f64 = (0.05 + noise_metadata_schedule_97_0_e1213);
        (noise_metadata_schedule_97_0_e1214,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_97_0_e1216;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_98_0_e1218: f64 = (-3.0);
            let noise_metadata_schedule_98_0_e1220: f64 = (noise_metadata_schedule_98_0_e1218 * w[6]);
            let noise_metadata_schedule_98_0_e1222: f64 = (noise_metadata_schedule_98_0_e1220 * w[260]);
            let noise_metadata_schedule_98_0_e1225: f64 = (params.p26 * w[4]);
            let noise_metadata_schedule_98_0_e1226: f64 = (noise_metadata_schedule_98_0_e1222 + noise_metadata_schedule_98_0_e1225);
            let noise_metadata_schedule_98_0_e1229: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_98_0_e1231: f64 = (noise_metadata_schedule_98_0_e1229 * params.p108);
            let noise_metadata_schedule_98_0_e1232: f64 = (noise_metadata_schedule_98_0_e1226 + noise_metadata_schedule_98_0_e1231);
            w[56] = noise_metadata_schedule_98_0_e1232;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_99_0_e1235: f64 = (0.05 - w[56]);
            let noise_metadata_schedule_99_0_e1237: f64 = (noise_metadata_schedule_99_0_e1235 / w[6]);
            w[265] = noise_metadata_schedule_99_0_e1237;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_100_0_e1240: f64 = if 0.05 < w[56] { 1.0 } else { 0.0 };
            w[460] = noise_metadata_schedule_100_0_e1240;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_101_0_e1252,) = {
    if (w[460] != 0.0) {
        let noise_metadata_schedule_101_0_e1246: f64 = (w[265]).exp();
        let noise_metadata_schedule_101_0_e1247: f64 = (1.0 + noise_metadata_schedule_101_0_e1246);
        let noise_metadata_schedule_101_0_e1248: f64 = (noise_metadata_schedule_101_0_e1247).ln();
        let noise_metadata_schedule_101_0_e1249: f64 = (w[6] * noise_metadata_schedule_101_0_e1248);
        let noise_metadata_schedule_101_0_e1250: f64 = (w[56] + noise_metadata_schedule_101_0_e1249);
        (noise_metadata_schedule_101_0_e1250,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_101_0_e1252;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_102_0_e1266,) = {
    if (w[460] == 0.0) {
        let noise_metadata_schedule_102_0_e1259: f64 = (-w[265]);
        let noise_metadata_schedule_102_0_e1260: f64 = (noise_metadata_schedule_102_0_e1259).exp();
        let noise_metadata_schedule_102_0_e1261: f64 = (1.0 + noise_metadata_schedule_102_0_e1260);
        let noise_metadata_schedule_102_0_e1262: f64 = (noise_metadata_schedule_102_0_e1261).ln();
        let noise_metadata_schedule_102_0_e1263: f64 = (w[6] * noise_metadata_schedule_102_0_e1262);
        let noise_metadata_schedule_102_0_e1264: f64 = (0.05 + noise_metadata_schedule_102_0_e1263);
        (noise_metadata_schedule_102_0_e1264,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_102_0_e1266;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_103_0_e1269: f64 = (1.0 / w[14]);
            w[65] = noise_metadata_schedule_103_0_e1269;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_104_0_e1272: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_104_0_e1272;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_105_0_e1275: f64 = (params.p65 * w[65]);
            let noise_metadata_schedule_105_0_e1277: f64 = (noise_metadata_schedule_105_0_e1275).powf(params.p66);
            w[73] = noise_metadata_schedule_105_0_e1277;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_106_0_e1280: f64 = (w[75] * w[67]);
            let noise_metadata_schedule_106_0_e1282: f64 = (noise_metadata_schedule_106_0_e1280).powf(w[76]);
            w[90] = noise_metadata_schedule_106_0_e1282;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_108_0_e1288: f64 = (1.0 - params.p74);
            let noise_metadata_schedule_108_0_e1291: f64 = (params.p70 / w[17]);
            let noise_metadata_schedule_108_0_e1293: f64 = (noise_metadata_schedule_108_0_e1291).powf(params.p71);
            let noise_metadata_schedule_108_0_e1294: f64 = (noise_metadata_schedule_108_0_e1288 * noise_metadata_schedule_108_0_e1293);
            let noise_metadata_schedule_108_0_e1296: f64 = (noise_metadata_schedule_108_0_e1294 + params.p74);
            w[26] = noise_metadata_schedule_108_0_e1296;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_109_0_e1299: f64 = (1.0 / w[26]);
            w[27] = noise_metadata_schedule_109_0_e1299;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_111_0_e1305: f64 = (params.p74 * w[27]);
            w[25] = noise_metadata_schedule_111_0_e1305;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_112_0_e1309: f64 = (w[260] * params.p96);
            let noise_metadata_schedule_112_0_e1310: f64 = (noise_metadata_schedule_112_0_e1309).exp();
            let noise_metadata_schedule_112_0_e1311: f64 = (params.p53 * noise_metadata_schedule_112_0_e1310);
            w[28] = noise_metadata_schedule_112_0_e1311;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_113_0_e1314: f64 = if w[28] < w[322] { 1.0 } else { 0.0 };
            w[461] = noise_metadata_schedule_113_0_e1314;
        }
        if (active[0] & 0xa) != 0 {
            let (noise_metadata_schedule_114_0_e1318,) = {
    if (w[461] != 0.0) {
        (w[322],)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_114_0_e1318;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_115_0_e1323: f64 = (params.p97 - params.p95);
            let noise_metadata_schedule_115_0_e1324: f64 = (w[260] * noise_metadata_schedule_115_0_e1323);
            let noise_metadata_schedule_115_0_e1325: f64 = (noise_metadata_schedule_115_0_e1324).exp();
            let noise_metadata_schedule_115_0_e1326: f64 = (params.p55 * noise_metadata_schedule_115_0_e1325);
            w[29] = noise_metadata_schedule_115_0_e1326;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_116_0_e1330: f64 = (w[260] * params.p100);
            let noise_metadata_schedule_116_0_e1331: f64 = (noise_metadata_schedule_116_0_e1330).exp();
            let noise_metadata_schedule_116_0_e1332: f64 = (params.p54 * noise_metadata_schedule_116_0_e1331);
            w[30] = noise_metadata_schedule_116_0_e1332;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_117_0_e1335: f64 = if w[30] < w[322] { 1.0 } else { 0.0 };
            w[462] = noise_metadata_schedule_117_0_e1335;
        }
        if (active[0] & 0x12) != 0 {
            let (noise_metadata_schedule_118_0_e1339,) = {
    if (w[462] != 0.0) {
        (w[322],)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_118_0_e1339;
        }
        if (active[0] & 0x153fe00) != 0 {
            let noise_metadata_schedule_119_0_e1343: f64 = (w[260] * params.p101);
            let noise_metadata_schedule_119_0_e1344: f64 = (noise_metadata_schedule_119_0_e1343).exp();
            let noise_metadata_schedule_119_0_e1345: f64 = (params.p56 * noise_metadata_schedule_119_0_e1344);
            w[32] = noise_metadata_schedule_119_0_e1345;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_120_0_e1349: f64 = (w[260] * params.p103);
            let noise_metadata_schedule_120_0_e1350: f64 = (noise_metadata_schedule_120_0_e1349).exp();
            let noise_metadata_schedule_120_0_e1351: f64 = (params.p57 * noise_metadata_schedule_120_0_e1350);
            w[33] = noise_metadata_schedule_120_0_e1351;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_121_0_e1355: f64 = (w[260] * params.p103);
            let noise_metadata_schedule_121_0_e1356: f64 = (noise_metadata_schedule_121_0_e1355).exp();
            let noise_metadata_schedule_121_0_e1357: f64 = (params.p58 * noise_metadata_schedule_121_0_e1356);
            w[34] = noise_metadata_schedule_121_0_e1357;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_122_0_e1361: f64 = (w[260] * params.p98);
            let noise_metadata_schedule_122_0_e1362: f64 = (noise_metadata_schedule_122_0_e1361).exp();
            let noise_metadata_schedule_122_0_e1363: f64 = (params.p59 * noise_metadata_schedule_122_0_e1362);
            w[31] = noise_metadata_schedule_122_0_e1363;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_123_0_e1366: f64 = if params.p121 != 0.0 { 1.0 } else { 0.0 };
            w[463] = noise_metadata_schedule_123_0_e1366;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_124_0_e1376,) = {
    if (w[463] != 0.0) {
        let noise_metadata_schedule_124_0_e1372: f64 = (w[12] * params.p121);
        let noise_metadata_schedule_124_0_e1373: f64 = (1.0 + noise_metadata_schedule_124_0_e1372);
        let noise_metadata_schedule_124_0_e1374: f64 = (params.p9 * noise_metadata_schedule_124_0_e1373);
        (noise_metadata_schedule_124_0_e1374,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_124_0_e1376;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_125_0_e1384,) = {
    if (w[463] != 0.0) {
        let noise_metadata_schedule_125_0_e1380: f64 = (w[50] - 1.0);
        let noise_metadata_schedule_125_0_e1382: f64 = (noise_metadata_schedule_125_0_e1380 / w[52]);
        (noise_metadata_schedule_125_0_e1382,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_125_0_e1384;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_126_0_e1387: f64 = if w[50] < 1.0 { 1.0 } else { 0.0 };
            w[464] = noise_metadata_schedule_126_0_e1387;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_127_0_e1401,) = {
    if ((w[463] != 0.0) && (w[464] != 0.0)) {
        let noise_metadata_schedule_127_0_e1395: f64 = (w[265]).exp();
        let noise_metadata_schedule_127_0_e1396: f64 = (1.0 + noise_metadata_schedule_127_0_e1395);
        let noise_metadata_schedule_127_0_e1397: f64 = (noise_metadata_schedule_127_0_e1396).ln();
        let noise_metadata_schedule_127_0_e1398: f64 = (w[52] * noise_metadata_schedule_127_0_e1397);
        let noise_metadata_schedule_127_0_e1399: f64 = (1.0 + noise_metadata_schedule_127_0_e1398);
        (noise_metadata_schedule_127_0_e1399,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_127_0_e1401;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_128_0_e1417,) = {
    if ((w[463] != 0.0) && (w[464] == 0.0)) {
        let noise_metadata_schedule_128_0_e1410: f64 = (-w[265]);
        let noise_metadata_schedule_128_0_e1411: f64 = (noise_metadata_schedule_128_0_e1410).exp();
        let noise_metadata_schedule_128_0_e1412: f64 = (1.0 + noise_metadata_schedule_128_0_e1411);
        let noise_metadata_schedule_128_0_e1413: f64 = (noise_metadata_schedule_128_0_e1412).ln();
        let noise_metadata_schedule_128_0_e1414: f64 = (w[52] * noise_metadata_schedule_128_0_e1413);
        let noise_metadata_schedule_128_0_e1415: f64 = (w[50] + noise_metadata_schedule_128_0_e1414);
        (noise_metadata_schedule_128_0_e1415,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_128_0_e1417;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_129_0_e1425,) = {
    if (w[463] != 0.0) {
        let noise_metadata_schedule_129_0_e1422: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_129_0_e1423: f64 = (w[50] - noise_metadata_schedule_129_0_e1422);
        (noise_metadata_schedule_129_0_e1423,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_129_0_e1425;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_130_0_e1430,) = {
    if (w[463] == 0.0) {
        (params.p9,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_130_0_e1430;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_131_0_e1433: f64 = if params.p122 != 0.0 { 1.0 } else { 0.0 };
            w[465] = noise_metadata_schedule_131_0_e1433;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_132_0_e1443,) = {
    if (w[465] != 0.0) {
        let noise_metadata_schedule_132_0_e1439: f64 = (w[12] * params.p122);
        let noise_metadata_schedule_132_0_e1440: f64 = (1.0 + noise_metadata_schedule_132_0_e1439);
        let noise_metadata_schedule_132_0_e1441: f64 = (params.p10 * noise_metadata_schedule_132_0_e1440);
        (noise_metadata_schedule_132_0_e1441,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_132_0_e1443;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_133_0_e1451,) = {
    if (w[465] != 0.0) {
        let noise_metadata_schedule_133_0_e1447: f64 = (w[51] - 1.0);
        let noise_metadata_schedule_133_0_e1449: f64 = (noise_metadata_schedule_133_0_e1447 / w[52]);
        (noise_metadata_schedule_133_0_e1449,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_133_0_e1451;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_134_0_e1454: f64 = if w[51] < 1.0 { 1.0 } else { 0.0 };
            w[466] = noise_metadata_schedule_134_0_e1454;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_135_0_e1468,) = {
    if ((w[465] != 0.0) && (w[466] != 0.0)) {
        let noise_metadata_schedule_135_0_e1462: f64 = (w[265]).exp();
        let noise_metadata_schedule_135_0_e1463: f64 = (1.0 + noise_metadata_schedule_135_0_e1462);
        let noise_metadata_schedule_135_0_e1464: f64 = (noise_metadata_schedule_135_0_e1463).ln();
        let noise_metadata_schedule_135_0_e1465: f64 = (w[52] * noise_metadata_schedule_135_0_e1464);
        let noise_metadata_schedule_135_0_e1466: f64 = (1.0 + noise_metadata_schedule_135_0_e1465);
        (noise_metadata_schedule_135_0_e1466,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_135_0_e1468;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_136_0_e1484,) = {
    if ((w[465] != 0.0) && (w[466] == 0.0)) {
        let noise_metadata_schedule_136_0_e1477: f64 = (-w[265]);
        let noise_metadata_schedule_136_0_e1478: f64 = (noise_metadata_schedule_136_0_e1477).exp();
        let noise_metadata_schedule_136_0_e1479: f64 = (1.0 + noise_metadata_schedule_136_0_e1478);
        let noise_metadata_schedule_136_0_e1480: f64 = (noise_metadata_schedule_136_0_e1479).ln();
        let noise_metadata_schedule_136_0_e1481: f64 = (w[52] * noise_metadata_schedule_136_0_e1480);
        let noise_metadata_schedule_136_0_e1482: f64 = (w[51] + noise_metadata_schedule_136_0_e1481);
        (noise_metadata_schedule_136_0_e1482,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_136_0_e1484;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_137_0_e1492,) = {
    if (w[465] != 0.0) {
        let noise_metadata_schedule_137_0_e1489: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_137_0_e1490: f64 = (w[51] - noise_metadata_schedule_137_0_e1489);
        (noise_metadata_schedule_137_0_e1490,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_137_0_e1492;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_138_0_e1497,) = {
    if (w[465] == 0.0) {
        (params.p10,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_138_0_e1497;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_139_0_e1502: f64 = (params.p123 * w[12]);
            let noise_metadata_schedule_139_0_e1503: f64 = (1.0 + noise_metadata_schedule_139_0_e1502);
            let noise_metadata_schedule_139_0_e1504: f64 = (params.p42 * noise_metadata_schedule_139_0_e1503);
            w[317] = noise_metadata_schedule_139_0_e1504;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_140_0_e1507: f64 = (w[318] * w[318]);
            w[267] = noise_metadata_schedule_140_0_e1507;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_141_0_e1510: f64 = (w[317] * w[317]);
            w[268] = noise_metadata_schedule_141_0_e1510;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_142_0_e1513: f64 = if w[317] < 0.0 { 1.0 } else { 0.0 };
            w[467] = noise_metadata_schedule_142_0_e1513;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_143_0_e1526,) = {
    if (w[467] != 0.0) {
        let noise_metadata_schedule_143_0_e1517: f64 = (0.5 * w[267]);
        let noise_metadata_schedule_143_0_e1520: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_143_0_e1521: f64 = (noise_metadata_schedule_143_0_e1520).sqrt();
        let noise_metadata_schedule_143_0_e1523: f64 = (noise_metadata_schedule_143_0_e1521 - w[317]);
        let noise_metadata_schedule_143_0_e1524: f64 = (noise_metadata_schedule_143_0_e1517 / noise_metadata_schedule_143_0_e1523);
        (noise_metadata_schedule_143_0_e1524,)
    } else {
        (w[316],)
    }
};
            w[316] = noise_metadata_schedule_143_0_e1526;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_144_0_e1538,) = {
    if (w[467] == 0.0) {
        let noise_metadata_schedule_144_0_e1532: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_144_0_e1533: f64 = (noise_metadata_schedule_144_0_e1532).sqrt();
        let noise_metadata_schedule_144_0_e1535: f64 = (noise_metadata_schedule_144_0_e1533 + w[317]);
        let noise_metadata_schedule_144_0_e1536: f64 = (0.5 * noise_metadata_schedule_144_0_e1535);
        (noise_metadata_schedule_144_0_e1536,)
    } else {
        (w[316],)
    }
};
            w[316] = noise_metadata_schedule_144_0_e1538;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_145_0_e1543: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_145_0_e1545: f64 = (noise_metadata_schedule_145_0_e1543 - params.p95);
            let noise_metadata_schedule_145_0_e1547: f64 = (noise_metadata_schedule_145_0_e1545 + params.p120);
            let noise_metadata_schedule_145_0_e1548: f64 = (w[260] * noise_metadata_schedule_145_0_e1547);
            let noise_metadata_schedule_145_0_e1550: f64 = (noise_metadata_schedule_145_0_e1548 / w[48]);
            let noise_metadata_schedule_145_0_e1551: f64 = (noise_metadata_schedule_145_0_e1550).exp();
            let noise_metadata_schedule_145_0_e1552: f64 = (params.p8 * noise_metadata_schedule_145_0_e1551);
            let noise_metadata_schedule_145_0_e1554: f64 = (-params.p104);
            let noise_metadata_schedule_145_0_e1556: f64 = (noise_metadata_schedule_145_0_e1554 * w[10]);
            let noise_metadata_schedule_145_0_e1558: f64 = (noise_metadata_schedule_145_0_e1556 / w[48]);
            let noise_metadata_schedule_145_0_e1559: f64 = (noise_metadata_schedule_145_0_e1558).exp();
            let noise_metadata_schedule_145_0_e1560: f64 = (noise_metadata_schedule_145_0_e1552 * noise_metadata_schedule_145_0_e1559);
            w[35] = noise_metadata_schedule_145_0_e1560;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_146_0_e1565: f64 = (1.0 - params.p97);
            let noise_metadata_schedule_146_0_e1566: f64 = (w[260] * noise_metadata_schedule_146_0_e1565);
            let noise_metadata_schedule_146_0_e1567: f64 = (noise_metadata_schedule_146_0_e1566).exp();
            let noise_metadata_schedule_146_0_e1568: f64 = (params.p11 * noise_metadata_schedule_146_0_e1567);
            w[36] = noise_metadata_schedule_146_0_e1568;
        }
        if (active[0] & 0x7800) != 0 {
            let noise_metadata_schedule_147_0_e1573: f64 = (1.0 - params.p102);
            let noise_metadata_schedule_147_0_e1574: f64 = (w[260] * noise_metadata_schedule_147_0_e1573);
            let noise_metadata_schedule_147_0_e1575: f64 = (noise_metadata_schedule_147_0_e1574).exp();
            let noise_metadata_schedule_147_0_e1576: f64 = (params.p29 * noise_metadata_schedule_147_0_e1575);
            w[37] = noise_metadata_schedule_147_0_e1576;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_148_0_e1582: f64 = (2.0 * params.p20);
            let noise_metadata_schedule_148_0_e1583: f64 = (6.0 - noise_metadata_schedule_148_0_e1582);
            let noise_metadata_schedule_148_0_e1584: f64 = (w[260] * noise_metadata_schedule_148_0_e1583);
            let noise_metadata_schedule_148_0_e1585: f64 = (noise_metadata_schedule_148_0_e1584).exp();
            let noise_metadata_schedule_148_0_e1586: f64 = (params.p19 * noise_metadata_schedule_148_0_e1585);
            let noise_metadata_schedule_148_0_e1588: f64 = (-params.p112);
            let noise_metadata_schedule_148_0_e1590: f64 = (noise_metadata_schedule_148_0_e1588 * w[10]);
            let noise_metadata_schedule_148_0_e1592: f64 = (noise_metadata_schedule_148_0_e1590 / params.p20);
            let noise_metadata_schedule_148_0_e1593: f64 = (noise_metadata_schedule_148_0_e1592).exp();
            let noise_metadata_schedule_148_0_e1594: f64 = (noise_metadata_schedule_148_0_e1586 * noise_metadata_schedule_148_0_e1593);
            w[38] = noise_metadata_schedule_148_0_e1594;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_149_0_e1600: f64 = (2.0 * params.p31);
            let noise_metadata_schedule_149_0_e1601: f64 = (6.0 - noise_metadata_schedule_149_0_e1600);
            let noise_metadata_schedule_149_0_e1602: f64 = (w[260] * noise_metadata_schedule_149_0_e1601);
            let noise_metadata_schedule_149_0_e1603: f64 = (noise_metadata_schedule_149_0_e1602).exp();
            let noise_metadata_schedule_149_0_e1604: f64 = (params.p30 * noise_metadata_schedule_149_0_e1603);
            let noise_metadata_schedule_149_0_e1606: f64 = (-params.p109);
            let noise_metadata_schedule_149_0_e1608: f64 = (noise_metadata_schedule_149_0_e1606 * w[10]);
            let noise_metadata_schedule_149_0_e1610: f64 = (noise_metadata_schedule_149_0_e1608 / params.p31);
            let noise_metadata_schedule_149_0_e1611: f64 = (noise_metadata_schedule_149_0_e1610).exp();
            let noise_metadata_schedule_149_0_e1612: f64 = (noise_metadata_schedule_149_0_e1604 * noise_metadata_schedule_149_0_e1611);
            w[39] = noise_metadata_schedule_149_0_e1612;
        }
        if (active[0] & 0x46) != 0 {
            let noise_metadata_schedule_150_0_e1617: f64 = (4.0 - params.p96);
            let noise_metadata_schedule_150_0_e1619: f64 = (noise_metadata_schedule_150_0_e1617 + params.p120);
            let noise_metadata_schedule_150_0_e1620: f64 = (w[260] * noise_metadata_schedule_150_0_e1619);
            let noise_metadata_schedule_150_0_e1622: f64 = (noise_metadata_schedule_150_0_e1620 / params.p16);
            let noise_metadata_schedule_150_0_e1623: f64 = (noise_metadata_schedule_150_0_e1622).exp();
            let noise_metadata_schedule_150_0_e1624: f64 = (params.p15 * noise_metadata_schedule_150_0_e1623);
            let noise_metadata_schedule_150_0_e1626: f64 = (-params.p110);
            let noise_metadata_schedule_150_0_e1628: f64 = (noise_metadata_schedule_150_0_e1626 * w[10]);
            let noise_metadata_schedule_150_0_e1630: f64 = (noise_metadata_schedule_150_0_e1628 / params.p16);
            let noise_metadata_schedule_150_0_e1631: f64 = (noise_metadata_schedule_150_0_e1630).exp();
            let noise_metadata_schedule_150_0_e1632: f64 = (noise_metadata_schedule_150_0_e1624 * noise_metadata_schedule_150_0_e1631);
            w[42] = noise_metadata_schedule_150_0_e1632;
        }
        if (active[0] & 0x140) != 0 {
            let noise_metadata_schedule_151_0_e1637: f64 = (4.0 - params.p96);
            let noise_metadata_schedule_151_0_e1639: f64 = (noise_metadata_schedule_151_0_e1637 + params.p120);
            let noise_metadata_schedule_151_0_e1640: f64 = (w[260] * noise_metadata_schedule_151_0_e1639);
            let noise_metadata_schedule_151_0_e1642: f64 = (noise_metadata_schedule_151_0_e1640 / params.p18);
            let noise_metadata_schedule_151_0_e1643: f64 = (noise_metadata_schedule_151_0_e1642).exp();
            let noise_metadata_schedule_151_0_e1644: f64 = (params.p17 * noise_metadata_schedule_151_0_e1643);
            let noise_metadata_schedule_151_0_e1646: f64 = (-params.p110);
            let noise_metadata_schedule_151_0_e1648: f64 = (noise_metadata_schedule_151_0_e1646 * w[10]);
            let noise_metadata_schedule_151_0_e1650: f64 = (noise_metadata_schedule_151_0_e1648 / params.p18);
            let noise_metadata_schedule_151_0_e1651: f64 = (noise_metadata_schedule_151_0_e1650).exp();
            let noise_metadata_schedule_151_0_e1652: f64 = (noise_metadata_schedule_151_0_e1644 * noise_metadata_schedule_151_0_e1651);
            w[44] = noise_metadata_schedule_151_0_e1652;
        }
        if (active[0] & 0x144) != 0 {
            let noise_metadata_schedule_152_0_e1655: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            w[468] = noise_metadata_schedule_152_0_e1655;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_153_0_e1667,) = {
    if (w[468] != 0.0) {
        let noise_metadata_schedule_153_0_e1659: f64 = (-params.p106);
        let noise_metadata_schedule_153_0_e1661: f64 = (noise_metadata_schedule_153_0_e1659 * w[10]);
        let noise_metadata_schedule_153_0_e1663: f64 = (noise_metadata_schedule_153_0_e1661 / params.p16);
        let noise_metadata_schedule_153_0_e1664: f64 = (noise_metadata_schedule_153_0_e1663).exp();
        let noise_metadata_schedule_153_0_e1665: f64 = (params.p24 * noise_metadata_schedule_153_0_e1664);
        (noise_metadata_schedule_153_0_e1665,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_153_0_e1667;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_154_0_e1677,) = {
    if (w[468] != 0.0) {
        let noise_metadata_schedule_154_0_e1671: f64 = (-params.p105);
        let noise_metadata_schedule_154_0_e1673: f64 = (noise_metadata_schedule_154_0_e1671 * w[10]);
        let noise_metadata_schedule_154_0_e1674: f64 = (noise_metadata_schedule_154_0_e1673).exp();
        let noise_metadata_schedule_154_0_e1675: f64 = (params.p27 * noise_metadata_schedule_154_0_e1674);
        (noise_metadata_schedule_154_0_e1675,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_154_0_e1677;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_155_0_e1689,) = {
    if (w[468] != 0.0) {
        let noise_metadata_schedule_155_0_e1681: f64 = (-params.p107);
        let noise_metadata_schedule_155_0_e1683: f64 = (noise_metadata_schedule_155_0_e1681 * w[10]);
        let noise_metadata_schedule_155_0_e1685: f64 = (noise_metadata_schedule_155_0_e1683 / params.p18);
        let noise_metadata_schedule_155_0_e1686: f64 = (noise_metadata_schedule_155_0_e1685).exp();
        let noise_metadata_schedule_155_0_e1687: f64 = (params.p25 * noise_metadata_schedule_155_0_e1686);
        (noise_metadata_schedule_155_0_e1687,)
    } else {
        (w[45],)
    }
};
            w[45] = noise_metadata_schedule_155_0_e1689;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_156_0_e1694: f64 = (4.0 - params.p102);
            let noise_metadata_schedule_156_0_e1696: f64 = (noise_metadata_schedule_156_0_e1694 + params.p120);
            let noise_metadata_schedule_156_0_e1697: f64 = (w[260] * noise_metadata_schedule_156_0_e1696);
            let noise_metadata_schedule_156_0_e1698: f64 = (noise_metadata_schedule_156_0_e1697).exp();
            let noise_metadata_schedule_156_0_e1699: f64 = (params.p28 * noise_metadata_schedule_156_0_e1698);
            let noise_metadata_schedule_156_0_e1701: f64 = (-params.p111);
            let noise_metadata_schedule_156_0_e1703: f64 = (noise_metadata_schedule_156_0_e1701 * w[10]);
            let noise_metadata_schedule_156_0_e1704: f64 = (noise_metadata_schedule_156_0_e1703).exp();
            let noise_metadata_schedule_156_0_e1705: f64 = (noise_metadata_schedule_156_0_e1699 * noise_metadata_schedule_156_0_e1704);
            w[43] = noise_metadata_schedule_156_0_e1705;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_157_0_e1711: f64 = (2.0 * params.p22);
            let noise_metadata_schedule_157_0_e1712: f64 = (6.0 - noise_metadata_schedule_157_0_e1711);
            let noise_metadata_schedule_157_0_e1713: f64 = (w[260] * noise_metadata_schedule_157_0_e1712);
            let noise_metadata_schedule_157_0_e1714: f64 = (noise_metadata_schedule_157_0_e1713).exp();
            let noise_metadata_schedule_157_0_e1715: f64 = (params.p21 * noise_metadata_schedule_157_0_e1714);
            let noise_metadata_schedule_157_0_e1717: f64 = (-params.p112);
            let noise_metadata_schedule_157_0_e1719: f64 = (noise_metadata_schedule_157_0_e1717 * w[10]);
            let noise_metadata_schedule_157_0_e1721: f64 = (noise_metadata_schedule_157_0_e1719 / params.p22);
            let noise_metadata_schedule_157_0_e1722: f64 = (noise_metadata_schedule_157_0_e1721).exp();
            let noise_metadata_schedule_157_0_e1723: f64 = (noise_metadata_schedule_157_0_e1715 * noise_metadata_schedule_157_0_e1722);
            w[46] = noise_metadata_schedule_157_0_e1723;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_158_0_e1728: f64 = (4.0 / params.p137);
            let noise_metadata_schedule_158_0_e1729: f64 = (w[260] * noise_metadata_schedule_158_0_e1728);
            let noise_metadata_schedule_158_0_e1730: f64 = (noise_metadata_schedule_158_0_e1729).exp();
            let noise_metadata_schedule_158_0_e1731: f64 = (params.p136 * noise_metadata_schedule_158_0_e1730);
            let noise_metadata_schedule_158_0_e1733: f64 = (-params.p112);
            let noise_metadata_schedule_158_0_e1735: f64 = (noise_metadata_schedule_158_0_e1733 * w[10]);
            let noise_metadata_schedule_158_0_e1737: f64 = (noise_metadata_schedule_158_0_e1735 / params.p137);
            let noise_metadata_schedule_158_0_e1738: f64 = (noise_metadata_schedule_158_0_e1737).exp();
            let noise_metadata_schedule_158_0_e1739: f64 = (noise_metadata_schedule_158_0_e1731 * noise_metadata_schedule_158_0_e1738);
            w[47] = noise_metadata_schedule_158_0_e1739;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_159_0_e1742: f64 = (w[4]).sqrt();
            let noise_metadata_schedule_159_0_e1743: f64 = (params.p142 * noise_metadata_schedule_159_0_e1742);
            let noise_metadata_schedule_159_0_e1746: f64 = (params.p144 * w[12]);
            let noise_metadata_schedule_159_0_e1747: f64 = (noise_metadata_schedule_159_0_e1746).exp();
            let noise_metadata_schedule_159_0_e1748: f64 = (noise_metadata_schedule_159_0_e1743 * noise_metadata_schedule_159_0_e1747);
            w[332] = noise_metadata_schedule_159_0_e1748;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_160_0_e1751: f64 = (w[70] * w[72]);
            let noise_metadata_schedule_160_0_e1753: f64 = (-0.5);
            let noise_metadata_schedule_160_0_e1754: f64 = (noise_metadata_schedule_160_0_e1751).powf(noise_metadata_schedule_160_0_e1753);
            w[261] = noise_metadata_schedule_160_0_e1754;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_161_0_e1757: f64 = (1.0 / w[73]);
            w[262] = noise_metadata_schedule_161_0_e1757;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_162_0_e1760: f64 = (params.p34 * w[70]);
            let noise_metadata_schedule_162_0_e1762: f64 = (noise_metadata_schedule_162_0_e1760 * w[70]);
            let noise_metadata_schedule_162_0_e1764: f64 = (noise_metadata_schedule_162_0_e1762 * w[261]);
            let noise_metadata_schedule_162_0_e1766: f64 = (noise_metadata_schedule_162_0_e1764 * w[262]);
            let noise_metadata_schedule_162_0_e1768: f64 = (noise_metadata_schedule_162_0_e1766 * params.p65);
            let noise_metadata_schedule_162_0_e1770: f64 = (noise_metadata_schedule_162_0_e1768 * w[65]);
            let noise_metadata_schedule_162_0_e1772: f64 = (noise_metadata_schedule_162_0_e1770 * w[72]);
            let noise_metadata_schedule_162_0_e1774: f64 = (noise_metadata_schedule_162_0_e1772 * w[72]);
            w[61] = noise_metadata_schedule_162_0_e1774;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_163_0_e1777: f64 = (params.p33 * w[261]);
            let noise_metadata_schedule_163_0_e1779: f64 = (noise_metadata_schedule_163_0_e1777 * w[14]);
            let noise_metadata_schedule_163_0_e1781: f64 = (noise_metadata_schedule_163_0_e1779 * w[14]);
            let noise_metadata_schedule_163_0_e1783: f64 = (noise_metadata_schedule_163_0_e1781 * w[64]);
            let noise_metadata_schedule_163_0_e1785: f64 = (noise_metadata_schedule_163_0_e1783 * w[64]);
            let noise_metadata_schedule_163_0_e1787: f64 = (noise_metadata_schedule_163_0_e1785 * w[73]);
            let noise_metadata_schedule_163_0_e1790: f64 = (params.p34 - w[61]);
            let noise_metadata_schedule_163_0_e1791: f64 = (noise_metadata_schedule_163_0_e1790).exp();
            let noise_metadata_schedule_163_0_e1792: f64 = (noise_metadata_schedule_163_0_e1787 * noise_metadata_schedule_163_0_e1791);
            w[58] = noise_metadata_schedule_163_0_e1792;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_164_0_e1795: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_164_0_e1795;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_165_0_e1798: f64 = (w[85] * w[86]);
            let noise_metadata_schedule_165_0_e1800: f64 = (-0.5);
            let noise_metadata_schedule_165_0_e1801: f64 = (noise_metadata_schedule_165_0_e1798).powf(noise_metadata_schedule_165_0_e1800);
            w[263] = noise_metadata_schedule_165_0_e1801;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_166_0_e1804: f64 = (1.0 / w[90]);
            w[264] = noise_metadata_schedule_166_0_e1804;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_167_0_e1807: f64 = (params.p36 * w[85]);
            let noise_metadata_schedule_167_0_e1809: f64 = (noise_metadata_schedule_167_0_e1807 * w[85]);
            let noise_metadata_schedule_167_0_e1811: f64 = (noise_metadata_schedule_167_0_e1809 * w[263]);
            let noise_metadata_schedule_167_0_e1813: f64 = (noise_metadata_schedule_167_0_e1811 * w[264]);
            let noise_metadata_schedule_167_0_e1815: f64 = (noise_metadata_schedule_167_0_e1813 * w[75]);
            let noise_metadata_schedule_167_0_e1817: f64 = (noise_metadata_schedule_167_0_e1815 * w[67]);
            let noise_metadata_schedule_167_0_e1819: f64 = (noise_metadata_schedule_167_0_e1817 * w[86]);
            let noise_metadata_schedule_167_0_e1821: f64 = (noise_metadata_schedule_167_0_e1819 * w[86]);
            w[83] = noise_metadata_schedule_167_0_e1821;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_168_0_e1824: f64 = (params.p35 * w[263]);
            let noise_metadata_schedule_168_0_e1826: f64 = (noise_metadata_schedule_168_0_e1824 * w[19]);
            let noise_metadata_schedule_168_0_e1828: f64 = (noise_metadata_schedule_168_0_e1826 * w[19]);
            let noise_metadata_schedule_168_0_e1830: f64 = (noise_metadata_schedule_168_0_e1828 * w[66]);
            let noise_metadata_schedule_168_0_e1832: f64 = (noise_metadata_schedule_168_0_e1830 * w[66]);
            let noise_metadata_schedule_168_0_e1834: f64 = (noise_metadata_schedule_168_0_e1832 * w[90]);
            let noise_metadata_schedule_168_0_e1837: f64 = (params.p36 - w[83]);
            let noise_metadata_schedule_168_0_e1838: f64 = (noise_metadata_schedule_168_0_e1837).exp();
            let noise_metadata_schedule_168_0_e1839: f64 = (noise_metadata_schedule_168_0_e1834 * noise_metadata_schedule_168_0_e1838);
            w[84] = noise_metadata_schedule_168_0_e1839;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_169_0_e1842: f64 = (w[260] * params.p95);
            let noise_metadata_schedule_169_0_e1843: f64 = (noise_metadata_schedule_169_0_e1842).exp();
            w[261] = noise_metadata_schedule_169_0_e1843;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_170_0_e1846: f64 = (params.p13 * w[261]);
            let noise_metadata_schedule_170_0_e1848: f64 = (noise_metadata_schedule_170_0_e1846 * w[27]);
            w[40] = noise_metadata_schedule_170_0_e1848;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_171_0_e1851: f64 = (params.p12 * w[261]);
            let noise_metadata_schedule_171_0_e1853: f64 = (noise_metadata_schedule_171_0_e1851 * w[262]);
            w[41] = noise_metadata_schedule_171_0_e1853;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_177_0_e1905: f64 = (w[2] - 300.0);
            w[101] = noise_metadata_schedule_177_0_e1905;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_178_0_e1908: f64 = if w[2] < 525.0 { 1.0 } else { 0.0 };
            w[469] = noise_metadata_schedule_178_0_e1908;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_179_0_e1924,) = {
    if (w[469] != 0.0) {
        let noise_metadata_schedule_179_0_e1914: f64 = (0.00072 * w[101]);
        let noise_metadata_schedule_179_0_e1915: f64 = (1.0 + noise_metadata_schedule_179_0_e1914);
        let noise_metadata_schedule_179_0_e1918: f64 = (1.6e-6 * w[101]);
        let noise_metadata_schedule_179_0_e1920: f64 = (noise_metadata_schedule_179_0_e1918 * w[101]);
        let noise_metadata_schedule_179_0_e1921: f64 = (noise_metadata_schedule_179_0_e1915 - noise_metadata_schedule_179_0_e1920);
        let noise_metadata_schedule_179_0_e1922: f64 = (w[1] * noise_metadata_schedule_179_0_e1921);
        (noise_metadata_schedule_179_0_e1922,)
    } else {
        (w[99],)
    }
};
            w[99] = noise_metadata_schedule_179_0_e1924;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_180_0_e1931,) = {
    if (w[469] == 0.0) {
        let noise_metadata_schedule_180_0_e1929: f64 = (w[1] * 1.081);
        (noise_metadata_schedule_180_0_e1929,)
    } else {
        (w[99],)
    }
};
            w[99] = noise_metadata_schedule_180_0_e1931;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_181_0_e1935: f64 = (w[260] * params.p95);
            let noise_metadata_schedule_181_0_e1936: f64 = (noise_metadata_schedule_181_0_e1935).exp();
            let noise_metadata_schedule_181_0_e1937: f64 = (params.p91 * noise_metadata_schedule_181_0_e1936);
            w[100] = noise_metadata_schedule_181_0_e1937;
        }
        if (active[0] & 0x1520000) != 0 {
            let noise_metadata_schedule_183_0_e1947: f64 = if params.p56 > 0.0 { 1.0 } else { 0.0 };
            w[470] = noise_metadata_schedule_183_0_e1947;
        }
        if (active[0] & 0x1520000) != 0 {
            let (noise_metadata_schedule_184_0_e1953,) = {
    if (w[470] != 0.0) {
        let noise_metadata_schedule_184_0_e1951: f64 = (1.0 / w[32]);
        (noise_metadata_schedule_184_0_e1951,)
    } else {
        (w[104],)
    }
};
            w[104] = noise_metadata_schedule_184_0_e1953;
        }
        if (active[0] & 0x1520000) != 0 {
            let noise_metadata_schedule_185_0_e1956: f64 = if w[104] > w[323] { 1.0 } else { 0.0 };
            w[471] = noise_metadata_schedule_185_0_e1956;
        }
        if (active[0] & 0x1520000) != 0 {
            let (noise_metadata_schedule_186_0_e1962,) = {
    if ((w[470] != 0.0) && (w[471] != 0.0)) {
        (w[323],)
    } else {
        (w[104],)
    }
};
            w[104] = noise_metadata_schedule_186_0_e1962;
        }
        if (active[0] & 0x1520000) != 0 {
            let (noise_metadata_schedule_187_0_e1967,) = {
    if (w[470] == 0.0) {
        (0.0,)
    } else {
        (w[104],)
    }
};
            w[104] = noise_metadata_schedule_187_0_e1967;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_188_0_e1970: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            w[472] = noise_metadata_schedule_188_0_e1970;
        }
        if (active[0] & 0x240000) != 0 {
            let (noise_metadata_schedule_189_0_e1976,) = {
    if (w[472] != 0.0) {
        let noise_metadata_schedule_189_0_e1974: f64 = (1.0 / w[33]);
        (noise_metadata_schedule_189_0_e1974,)
    } else {
        (w[105],)
    }
};
            w[105] = noise_metadata_schedule_189_0_e1976;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_190_0_e1979: f64 = if w[105] > w[323] { 1.0 } else { 0.0 };
            w[473] = noise_metadata_schedule_190_0_e1979;
        }
        if (active[0] & 0x240000) != 0 {
            let (noise_metadata_schedule_191_0_e1985,) = {
    if ((w[472] != 0.0) && (w[473] != 0.0)) {
        (w[323],)
    } else {
        (w[105],)
    }
};
            w[105] = noise_metadata_schedule_191_0_e1985;
        }
        if (active[0] & 0x240000) != 0 {
            let (noise_metadata_schedule_192_0_e1990,) = {
    if (w[472] == 0.0) {
        (0.0,)
    } else {
        (w[105],)
    }
};
            w[105] = noise_metadata_schedule_192_0_e1990;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_193_0_e1993: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            w[474] = noise_metadata_schedule_193_0_e1993;
        }
        if (active[0] & 0x880000) != 0 {
            let (noise_metadata_schedule_194_0_e1999,) = {
    if (w[474] != 0.0) {
        let noise_metadata_schedule_194_0_e1997: f64 = (1.0 / w[34]);
        (noise_metadata_schedule_194_0_e1997,)
    } else {
        (w[106],)
    }
};
            w[106] = noise_metadata_schedule_194_0_e1999;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_195_0_e2002: f64 = if w[106] > w[323] { 1.0 } else { 0.0 };
            w[475] = noise_metadata_schedule_195_0_e2002;
        }
        if (active[0] & 0x880000) != 0 {
            let (noise_metadata_schedule_196_0_e2008,) = {
    if ((w[474] != 0.0) && (w[475] != 0.0)) {
        (w[323],)
    } else {
        (w[106],)
    }
};
            w[106] = noise_metadata_schedule_196_0_e2008;
        }
        if (active[0] & 0x880000) != 0 {
            let (noise_metadata_schedule_197_0_e2013,) = {
    if (w[474] == 0.0) {
        (0.0,)
    } else {
        (w[106],)
    }
};
            w[106] = noise_metadata_schedule_197_0_e2013;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_198_0_e2016: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            w[236] = noise_metadata_schedule_198_0_e2016;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_199_0_e2019: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[8])));
            w[237] = noise_metadata_schedule_199_0_e2019;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_200_0_e2022: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[4])));
            w[238] = noise_metadata_schedule_200_0_e2022;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_201_0_e2025: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            w[239] = noise_metadata_schedule_201_0_e2025;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_202_0_e2028: f64 = (params.p3 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            w[240] = noise_metadata_schedule_202_0_e2028;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_203_0_e2031: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            w[242] = noise_metadata_schedule_203_0_e2031;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_205_0_e2037: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            w[246] = noise_metadata_schedule_205_0_e2037;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_207_0_e2043: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            w[250] = noise_metadata_schedule_207_0_e2043;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_208_0_e2046: f64 = (params.p3 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[7])));
            w[244] = noise_metadata_schedule_208_0_e2046;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_209_0_e2049: f64 = (params.p3 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
            w[243] = noise_metadata_schedule_209_0_e2049;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_210_0_e2052: f64 = (w[240] + w[237]);
            let noise_metadata_schedule_210_0_e2054: f64 = (noise_metadata_schedule_210_0_e2052 - w[242]);
            let noise_metadata_schedule_210_0_e2056: f64 = (noise_metadata_schedule_210_0_e2054 - w[244]);
            w[241] = noise_metadata_schedule_210_0_e2056;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_211_0_e2058: f64 = (-w[250]);
            let noise_metadata_schedule_211_0_e2060: f64 = (noise_metadata_schedule_211_0_e2058 + w[246]);
            let noise_metadata_schedule_211_0_e2062: f64 = (noise_metadata_schedule_211_0_e2060 + w[241]);
            let noise_metadata_schedule_211_0_e2064: f64 = (noise_metadata_schedule_211_0_e2062 - w[243]);
            w[248] = noise_metadata_schedule_211_0_e2064;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_212_0_e2067: f64 = (w[250] + w[248]);
            w[247] = noise_metadata_schedule_212_0_e2067;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_213_0_e2070: f64 = (w[237] * w[8]);
            let noise_metadata_schedule_213_0_e2072: f64 = if noise_metadata_schedule_213_0_e2070 < params.p138 { 1.0 } else { 0.0 };
            w[476] = noise_metadata_schedule_213_0_e2072;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_214_0_e2079,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_214_0_e2076: f64 = (w[237] * w[8]);
        let noise_metadata_schedule_214_0_e2077: f64 = (noise_metadata_schedule_214_0_e2076).exp();
        (noise_metadata_schedule_214_0_e2077,)
    } else {
        (w[251],)
    }
};
            w[251] = noise_metadata_schedule_214_0_e2079;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_215_0_e2085,) = {
    if (w[476] == 0.0) {
        let noise_metadata_schedule_215_0_e2083: f64 = (params.p138).exp();
        (noise_metadata_schedule_215_0_e2083,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_215_0_e2085;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_216_0_e2098,) = {
    if (w[476] == 0.0) {
        let noise_metadata_schedule_216_0_e2092: f64 = (w[237] * w[8]);
        let noise_metadata_schedule_216_0_e2094: f64 = (noise_metadata_schedule_216_0_e2092 - params.p138);
        let noise_metadata_schedule_216_0_e2095: f64 = (1.0 + noise_metadata_schedule_216_0_e2094);
        let noise_metadata_schedule_216_0_e2096: f64 = (w[281] * noise_metadata_schedule_216_0_e2095);
        (noise_metadata_schedule_216_0_e2096,)
    } else {
        (w[251],)
    }
};
            w[251] = noise_metadata_schedule_216_0_e2098;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_217_0_e2101: f64 = (w[238] * w[8]);
            let noise_metadata_schedule_217_0_e2103: f64 = (noise_metadata_schedule_217_0_e2101 / w[48]);
            let noise_metadata_schedule_217_0_e2105: f64 = if noise_metadata_schedule_217_0_e2103 < params.p138 { 1.0 } else { 0.0 };
            w[477] = noise_metadata_schedule_217_0_e2105;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_218_0_e2114,) = {
    if (w[477] != 0.0) {
        let noise_metadata_schedule_218_0_e2109: f64 = (w[238] * w[8]);
        let noise_metadata_schedule_218_0_e2111: f64 = (noise_metadata_schedule_218_0_e2109 / w[48]);
        let noise_metadata_schedule_218_0_e2112: f64 = (noise_metadata_schedule_218_0_e2111).exp();
        (noise_metadata_schedule_218_0_e2112,)
    } else {
        (w[252],)
    }
};
            w[252] = noise_metadata_schedule_218_0_e2114;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_219_0_e2120,) = {
    if (w[477] == 0.0) {
        let noise_metadata_schedule_219_0_e2118: f64 = (params.p138).exp();
        (noise_metadata_schedule_219_0_e2118,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_219_0_e2120;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_220_0_e2135,) = {
    if (w[477] == 0.0) {
        let noise_metadata_schedule_220_0_e2127: f64 = (w[238] * w[8]);
        let noise_metadata_schedule_220_0_e2129: f64 = (noise_metadata_schedule_220_0_e2127 / w[48]);
        let noise_metadata_schedule_220_0_e2131: f64 = (noise_metadata_schedule_220_0_e2129 - params.p138);
        let noise_metadata_schedule_220_0_e2132: f64 = (1.0 + noise_metadata_schedule_220_0_e2131);
        let noise_metadata_schedule_220_0_e2133: f64 = (w[281] * noise_metadata_schedule_220_0_e2132);
        (noise_metadata_schedule_220_0_e2133,)
    } else {
        (w[252],)
    }
};
            w[252] = noise_metadata_schedule_220_0_e2135;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_221_0_e2138: f64 = (w[241] * w[8]);
            let noise_metadata_schedule_221_0_e2140: f64 = if noise_metadata_schedule_221_0_e2138 < params.p138 { 1.0 } else { 0.0 };
            w[478] = noise_metadata_schedule_221_0_e2140;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_222_0_e2147,) = {
    if (w[478] != 0.0) {
        let noise_metadata_schedule_222_0_e2144: f64 = (w[241] * w[8]);
        let noise_metadata_schedule_222_0_e2145: f64 = (noise_metadata_schedule_222_0_e2144).exp();
        (noise_metadata_schedule_222_0_e2145,)
    } else {
        (w[254],)
    }
};
            w[254] = noise_metadata_schedule_222_0_e2147;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_223_0_e2153,) = {
    if (w[478] == 0.0) {
        let noise_metadata_schedule_223_0_e2151: f64 = (params.p138).exp();
        (noise_metadata_schedule_223_0_e2151,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_223_0_e2153;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_224_0_e2166,) = {
    if (w[478] == 0.0) {
        let noise_metadata_schedule_224_0_e2160: f64 = (w[241] * w[8]);
        let noise_metadata_schedule_224_0_e2162: f64 = (noise_metadata_schedule_224_0_e2160 - params.p138);
        let noise_metadata_schedule_224_0_e2163: f64 = (1.0 + noise_metadata_schedule_224_0_e2162);
        let noise_metadata_schedule_224_0_e2164: f64 = (w[281] * noise_metadata_schedule_224_0_e2163);
        (noise_metadata_schedule_224_0_e2164,)
    } else {
        (w[254],)
    }
};
            w[254] = noise_metadata_schedule_224_0_e2166;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_225_0_e2169: f64 = (w[240] * w[8]);
            let noise_metadata_schedule_225_0_e2171: f64 = if noise_metadata_schedule_225_0_e2169 < params.p138 { 1.0 } else { 0.0 };
            w[479] = noise_metadata_schedule_225_0_e2171;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_226_0_e2178,) = {
    if (w[479] != 0.0) {
        let noise_metadata_schedule_226_0_e2175: f64 = (w[240] * w[8]);
        let noise_metadata_schedule_226_0_e2176: f64 = (noise_metadata_schedule_226_0_e2175).exp();
        (noise_metadata_schedule_226_0_e2176,)
    } else {
        (w[253],)
    }
};
            w[253] = noise_metadata_schedule_226_0_e2178;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_227_0_e2184,) = {
    if (w[479] == 0.0) {
        let noise_metadata_schedule_227_0_e2182: f64 = (params.p138).exp();
        (noise_metadata_schedule_227_0_e2182,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_227_0_e2184;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_228_0_e2197,) = {
    if (w[479] == 0.0) {
        let noise_metadata_schedule_228_0_e2191: f64 = (w[240] * w[8]);
        let noise_metadata_schedule_228_0_e2193: f64 = (noise_metadata_schedule_228_0_e2191 - params.p138);
        let noise_metadata_schedule_228_0_e2194: f64 = (1.0 + noise_metadata_schedule_228_0_e2193);
        let noise_metadata_schedule_228_0_e2195: f64 = (w[281] * noise_metadata_schedule_228_0_e2194);
        (noise_metadata_schedule_228_0_e2195,)
    } else {
        (w[253],)
    }
};
            w[253] = noise_metadata_schedule_228_0_e2197;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_229_0_e2200: f64 = (w[247] * w[8]);
            let noise_metadata_schedule_229_0_e2202: f64 = if noise_metadata_schedule_229_0_e2200 < params.p138 { 1.0 } else { 0.0 };
            w[480] = noise_metadata_schedule_229_0_e2202;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_230_0_e2209,) = {
    if (w[480] != 0.0) {
        let noise_metadata_schedule_230_0_e2206: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_230_0_e2207: f64 = (noise_metadata_schedule_230_0_e2206).exp();
        (noise_metadata_schedule_230_0_e2207,)
    } else {
        (w[255],)
    }
};
            w[255] = noise_metadata_schedule_230_0_e2209;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_231_0_e2215,) = {
    if (w[480] == 0.0) {
        let noise_metadata_schedule_231_0_e2213: f64 = (params.p138).exp();
        (noise_metadata_schedule_231_0_e2213,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_231_0_e2215;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_232_0_e2228,) = {
    if (w[480] == 0.0) {
        let noise_metadata_schedule_232_0_e2222: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_232_0_e2224: f64 = (noise_metadata_schedule_232_0_e2222 - params.p138);
        let noise_metadata_schedule_232_0_e2225: f64 = (1.0 + noise_metadata_schedule_232_0_e2224);
        let noise_metadata_schedule_232_0_e2226: f64 = (w[281] * noise_metadata_schedule_232_0_e2225);
        (noise_metadata_schedule_232_0_e2226,)
    } else {
        (w[255],)
    }
};
            w[255] = noise_metadata_schedule_232_0_e2228;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_233_0_e2231: f64 = (w[247] - w[16]);
            let noise_metadata_schedule_233_0_e2233: f64 = (noise_metadata_schedule_233_0_e2231 * w[8]);
            let noise_metadata_schedule_233_0_e2235: f64 = if noise_metadata_schedule_233_0_e2233 < params.p138 { 1.0 } else { 0.0 };
            w[481] = noise_metadata_schedule_233_0_e2235;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_235_0_e2250,) = {
    if (w[481] == 0.0) {
        let noise_metadata_schedule_235_0_e2248: f64 = (params.p138).exp();
        (noise_metadata_schedule_235_0_e2248,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_235_0_e2250;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_237_0_e2268: f64 = (w[241] - w[16]);
            let noise_metadata_schedule_237_0_e2270: f64 = (noise_metadata_schedule_237_0_e2268 * w[8]);
            let noise_metadata_schedule_237_0_e2272: f64 = if noise_metadata_schedule_237_0_e2270 < params.p138 { 1.0 } else { 0.0 };
            w[482] = noise_metadata_schedule_237_0_e2272;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_239_0_e2287,) = {
    if (w[482] == 0.0) {
        let noise_metadata_schedule_239_0_e2285: f64 = (params.p138).exp();
        (noise_metadata_schedule_239_0_e2285,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_239_0_e2287;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_241_0_e2305: f64 = (w[237] - w[16]);
            let noise_metadata_schedule_241_0_e2307: f64 = (noise_metadata_schedule_241_0_e2305 * w[8]);
            let noise_metadata_schedule_241_0_e2309: f64 = if noise_metadata_schedule_241_0_e2307 < params.p138 { 1.0 } else { 0.0 };
            w[483] = noise_metadata_schedule_241_0_e2309;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_242_0_e2318,) = {
    if (w[483] != 0.0) {
        let noise_metadata_schedule_242_0_e2313: f64 = (w[237] - w[16]);
        let noise_metadata_schedule_242_0_e2315: f64 = (noise_metadata_schedule_242_0_e2313 * w[8]);
        let noise_metadata_schedule_242_0_e2316: f64 = (noise_metadata_schedule_242_0_e2315).exp();
        (noise_metadata_schedule_242_0_e2316,)
    } else {
        (w[257],)
    }
};
            w[257] = noise_metadata_schedule_242_0_e2318;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_243_0_e2324,) = {
    if (w[483] == 0.0) {
        let noise_metadata_schedule_243_0_e2322: f64 = (params.p138).exp();
        (noise_metadata_schedule_243_0_e2322,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_243_0_e2324;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_244_0_e2339,) = {
    if (w[483] == 0.0) {
        let noise_metadata_schedule_244_0_e2331: f64 = (w[237] - w[16]);
        let noise_metadata_schedule_244_0_e2333: f64 = (noise_metadata_schedule_244_0_e2331 * w[8]);
        let noise_metadata_schedule_244_0_e2335: f64 = (noise_metadata_schedule_244_0_e2333 - params.p138);
        let noise_metadata_schedule_244_0_e2336: f64 = (1.0 + noise_metadata_schedule_244_0_e2335);
        let noise_metadata_schedule_244_0_e2337: f64 = (w[281] * noise_metadata_schedule_244_0_e2336);
        (noise_metadata_schedule_244_0_e2337,)
    } else {
        (w[257],)
    }
};
            w[257] = noise_metadata_schedule_244_0_e2339;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_245_0_e2342: f64 = (w[236] - w[16]);
            let noise_metadata_schedule_245_0_e2344: f64 = (noise_metadata_schedule_245_0_e2342 * w[8]);
            let noise_metadata_schedule_245_0_e2346: f64 = if noise_metadata_schedule_245_0_e2344 < params.p138 { 1.0 } else { 0.0 };
            w[484] = noise_metadata_schedule_245_0_e2346;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_246_0_e2355,) = {
    if (w[484] != 0.0) {
        let noise_metadata_schedule_246_0_e2350: f64 = (w[236] - w[16]);
        let noise_metadata_schedule_246_0_e2352: f64 = (noise_metadata_schedule_246_0_e2350 * w[8]);
        let noise_metadata_schedule_246_0_e2353: f64 = (noise_metadata_schedule_246_0_e2352).exp();
        (noise_metadata_schedule_246_0_e2353,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_246_0_e2355;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_247_0_e2361,) = {
    if (w[484] == 0.0) {
        let noise_metadata_schedule_247_0_e2359: f64 = (params.p138).exp();
        (noise_metadata_schedule_247_0_e2359,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_247_0_e2361;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_248_0_e2376,) = {
    if (w[484] == 0.0) {
        let noise_metadata_schedule_248_0_e2368: f64 = (w[236] - w[16]);
        let noise_metadata_schedule_248_0_e2370: f64 = (noise_metadata_schedule_248_0_e2368 * w[8]);
        let noise_metadata_schedule_248_0_e2372: f64 = (noise_metadata_schedule_248_0_e2370 - params.p138);
        let noise_metadata_schedule_248_0_e2373: f64 = (1.0 + noise_metadata_schedule_248_0_e2372);
        let noise_metadata_schedule_248_0_e2374: f64 = (w[281] * noise_metadata_schedule_248_0_e2373);
        (noise_metadata_schedule_248_0_e2374,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_248_0_e2376;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_249_0_e2380: f64 = (4.0 * w[257]);
            let noise_metadata_schedule_249_0_e2381: f64 = (1.0 + noise_metadata_schedule_249_0_e2380);
            let noise_metadata_schedule_249_0_e2382: f64 = (noise_metadata_schedule_249_0_e2381).sqrt();
            w[107] = noise_metadata_schedule_249_0_e2382;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_250_0_e2386: f64 = (4.0 * w[259]);
            let noise_metadata_schedule_250_0_e2387: f64 = (1.0 + noise_metadata_schedule_250_0_e2386);
            let noise_metadata_schedule_250_0_e2388: f64 = (noise_metadata_schedule_250_0_e2387).sqrt();
            w[108] = noise_metadata_schedule_250_0_e2388;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_251_0_e2391: f64 = (2.0 * w[259]);
            let noise_metadata_schedule_251_0_e2394: f64 = (1.0 + w[108]);
            let noise_metadata_schedule_251_0_e2395: f64 = (noise_metadata_schedule_251_0_e2391 / noise_metadata_schedule_251_0_e2394);
            w[109] = noise_metadata_schedule_251_0_e2395;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_252_0_e2398: f64 = if w[109] < params.p140 { 1.0 } else { 0.0 };
            w[485] = noise_metadata_schedule_252_0_e2398;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_253_0_e2402,) = {
    if (w[485] != 0.0) {
        (params.p140,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_253_0_e2402;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_254_0_e2406: f64 = (w[107] - w[108]);
            let noise_metadata_schedule_254_0_e2409: f64 = (w[107] + 1.0);
            let noise_metadata_schedule_254_0_e2412: f64 = (w[108] + 1.0);
            let noise_metadata_schedule_254_0_e2413: f64 = (noise_metadata_schedule_254_0_e2409 / noise_metadata_schedule_254_0_e2412);
            let noise_metadata_schedule_254_0_e2414: f64 = (noise_metadata_schedule_254_0_e2413).ln();
            let noise_metadata_schedule_254_0_e2415: f64 = (noise_metadata_schedule_254_0_e2406 - noise_metadata_schedule_254_0_e2414);
            let noise_metadata_schedule_254_0_e2416: f64 = (w[6] * noise_metadata_schedule_254_0_e2415);
            w[110] = noise_metadata_schedule_254_0_e2416;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_255_0_e2419: f64 = (w[110] + w[242]);
            let noise_metadata_schedule_255_0_e2421: f64 = (noise_metadata_schedule_255_0_e2419 / w[31]);
            w[111] = noise_metadata_schedule_255_0_e2421;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_256_0_e2424: f64 = if w[111] > 0.0 { 1.0 } else { 0.0 };
            w[486] = noise_metadata_schedule_256_0_e2424;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_257_0_e2427: f64 = if w[236] < 100.0 { 1.0 } else { 0.0 };
            w[487] = noise_metadata_schedule_257_0_e2427;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_258_0_e2433,) = {
    if ((w[486] != 0.0) && (w[487] != 0.0)) {
        (w[236],)
    } else {
        (w[283],)
    }
};
            w[283] = noise_metadata_schedule_258_0_e2433;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_259_0_e2447,) = {
    if ((w[486] != 0.0) && (w[487] == 0.0)) {
        let noise_metadata_schedule_259_0_e2442: f64 = (w[236] - 100.0);
        let noise_metadata_schedule_259_0_e2443: f64 = (1.0 + noise_metadata_schedule_259_0_e2442);
        let noise_metadata_schedule_259_0_e2444: f64 = (noise_metadata_schedule_259_0_e2443).ln();
        let noise_metadata_schedule_259_0_e2445: f64 = (100.0 + noise_metadata_schedule_259_0_e2444);
        (noise_metadata_schedule_259_0_e2445,)
    } else {
        (w[283],)
    }
};
            w[283] = noise_metadata_schedule_259_0_e2447;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_260_0_e2468,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_260_0_e2452: f64 = (2.0 * w[6]);
        let noise_metadata_schedule_260_0_e2455: f64 = (0.5 * w[111]);
        let noise_metadata_schedule_260_0_e2457: f64 = (noise_metadata_schedule_260_0_e2455 * w[31]);
        let noise_metadata_schedule_260_0_e2459: f64 = (noise_metadata_schedule_260_0_e2457 * w[8]);
        let noise_metadata_schedule_260_0_e2461: f64 = (noise_metadata_schedule_260_0_e2459 + 1.0);
        let noise_metadata_schedule_260_0_e2462: f64 = (noise_metadata_schedule_260_0_e2461).ln();
        let noise_metadata_schedule_260_0_e2463: f64 = (noise_metadata_schedule_260_0_e2452 * noise_metadata_schedule_260_0_e2462);
        let noise_metadata_schedule_260_0_e2464: f64 = (w[16] + noise_metadata_schedule_260_0_e2463);
        let noise_metadata_schedule_260_0_e2466: f64 = (noise_metadata_schedule_260_0_e2464 - w[283]);
        (noise_metadata_schedule_260_0_e2466,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_260_0_e2468;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_261_0_e2474,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_261_0_e2472: f64 = (0.2 * w[16]);
        (noise_metadata_schedule_261_0_e2472,)
    } else {
        (w[278],)
    }
};
            w[278] = noise_metadata_schedule_261_0_e2474;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_262_0_e2480,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_262_0_e2478: f64 = (w[278] * w[278]);
        (noise_metadata_schedule_262_0_e2478,)
    } else {
        (w[267],)
    }
};
            w[267] = noise_metadata_schedule_262_0_e2480;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_263_0_e2486,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_263_0_e2484: f64 = (w[112] * w[112]);
        (noise_metadata_schedule_263_0_e2484,)
    } else {
        (w[268],)
    }
};
            w[268] = noise_metadata_schedule_263_0_e2486;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_264_0_e2489: f64 = if w[112] < 0.0 { 1.0 } else { 0.0 };
            w[488] = noise_metadata_schedule_264_0_e2489;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_265_0_e2504,) = {
    if ((w[486] != 0.0) && (w[488] != 0.0)) {
        let noise_metadata_schedule_265_0_e2495: f64 = (0.5 * w[267]);
        let noise_metadata_schedule_265_0_e2498: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_265_0_e2499: f64 = (noise_metadata_schedule_265_0_e2498).sqrt();
        let noise_metadata_schedule_265_0_e2501: f64 = (noise_metadata_schedule_265_0_e2499 - w[112]);
        let noise_metadata_schedule_265_0_e2502: f64 = (noise_metadata_schedule_265_0_e2495 / noise_metadata_schedule_265_0_e2501);
        (noise_metadata_schedule_265_0_e2502,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_265_0_e2504;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_266_0_e2518,) = {
    if ((w[486] != 0.0) && (w[488] == 0.0)) {
        let noise_metadata_schedule_266_0_e2512: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_266_0_e2513: f64 = (noise_metadata_schedule_266_0_e2512).sqrt();
        let noise_metadata_schedule_266_0_e2515: f64 = (noise_metadata_schedule_266_0_e2513 + w[112]);
        let noise_metadata_schedule_266_0_e2516: f64 = (0.5 * noise_metadata_schedule_266_0_e2515);
        (noise_metadata_schedule_266_0_e2516,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_266_0_e2518;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_267_0_e2536,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_267_0_e2524: f64 = (params.p61 * params.p60);
        let noise_metadata_schedule_267_0_e2525: f64 = (w[113] + noise_metadata_schedule_267_0_e2524);
        let noise_metadata_schedule_267_0_e2526: f64 = (w[113] * noise_metadata_schedule_267_0_e2525);
        let noise_metadata_schedule_267_0_e2531: f64 = (params.p61 * w[31]);
        let noise_metadata_schedule_267_0_e2532: f64 = (w[113] + noise_metadata_schedule_267_0_e2531);
        let noise_metadata_schedule_267_0_e2533: f64 = (params.p60 * noise_metadata_schedule_267_0_e2532);
        let noise_metadata_schedule_267_0_e2534: f64 = (noise_metadata_schedule_267_0_e2526 / noise_metadata_schedule_267_0_e2533);
        (noise_metadata_schedule_267_0_e2534,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_267_0_e2536;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_268_0_e2542,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_268_0_e2540: f64 = (w[111] / w[114]);
        (noise_metadata_schedule_268_0_e2540,)
    } else {
        (w[271],)
    }
};
            w[271] = noise_metadata_schedule_268_0_e2542;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_269_0_e2550,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_269_0_e2546: f64 = (w[271] - 1.0);
        let noise_metadata_schedule_269_0_e2548: f64 = (noise_metadata_schedule_269_0_e2546 / params.p62);
        (noise_metadata_schedule_269_0_e2548,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_269_0_e2550;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_270_0_e2553: f64 = if w[271] < 1.0 { 1.0 } else { 0.0 };
            w[489] = noise_metadata_schedule_270_0_e2553;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_271_0_e2567,) = {
    if ((w[486] != 0.0) && (w[489] != 0.0)) {
        let noise_metadata_schedule_271_0_e2561: f64 = (w[265]).exp();
        let noise_metadata_schedule_271_0_e2562: f64 = (1.0 + noise_metadata_schedule_271_0_e2561);
        let noise_metadata_schedule_271_0_e2563: f64 = (noise_metadata_schedule_271_0_e2562).ln();
        let noise_metadata_schedule_271_0_e2564: f64 = (params.p62 * noise_metadata_schedule_271_0_e2563);
        let noise_metadata_schedule_271_0_e2565: f64 = (1.0 + noise_metadata_schedule_271_0_e2564);
        (noise_metadata_schedule_271_0_e2565,)
    } else {
        (w[269],)
    }
};
            w[269] = noise_metadata_schedule_271_0_e2567;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_272_0_e2583,) = {
    if ((w[486] != 0.0) && (w[489] == 0.0)) {
        let noise_metadata_schedule_272_0_e2576: f64 = (-w[265]);
        let noise_metadata_schedule_272_0_e2577: f64 = (noise_metadata_schedule_272_0_e2576).exp();
        let noise_metadata_schedule_272_0_e2578: f64 = (1.0 + noise_metadata_schedule_272_0_e2577);
        let noise_metadata_schedule_272_0_e2579: f64 = (noise_metadata_schedule_272_0_e2578).ln();
        let noise_metadata_schedule_272_0_e2580: f64 = (params.p62 * noise_metadata_schedule_272_0_e2579);
        let noise_metadata_schedule_272_0_e2581: f64 = (w[271] + noise_metadata_schedule_272_0_e2580);
        (noise_metadata_schedule_272_0_e2581,)
    } else {
        (w[269],)
    }
};
            w[269] = noise_metadata_schedule_272_0_e2583;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_273_0_e2600,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_273_0_e2590: f64 = (-1.0);
        let noise_metadata_schedule_273_0_e2592: f64 = (noise_metadata_schedule_273_0_e2590 / params.p62);
        let noise_metadata_schedule_273_0_e2593: f64 = (noise_metadata_schedule_273_0_e2592).exp();
        let noise_metadata_schedule_273_0_e2594: f64 = (1.0 + noise_metadata_schedule_273_0_e2593);
        let noise_metadata_schedule_273_0_e2595: f64 = (noise_metadata_schedule_273_0_e2594).ln();
        let noise_metadata_schedule_273_0_e2596: f64 = (params.p62 * noise_metadata_schedule_273_0_e2595);
        let noise_metadata_schedule_273_0_e2597: f64 = (1.0 + noise_metadata_schedule_273_0_e2596);
        let noise_metadata_schedule_273_0_e2598: f64 = (w[269] / noise_metadata_schedule_273_0_e2597);
        (noise_metadata_schedule_273_0_e2598,)
    } else {
        (w[115],)
    }
};
            w[115] = noise_metadata_schedule_273_0_e2600;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_274_0_e2608,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_274_0_e2605: f64 = (params.p61 * params.p60);
        let noise_metadata_schedule_274_0_e2606: f64 = (w[113] / noise_metadata_schedule_274_0_e2605);
        (noise_metadata_schedule_274_0_e2606,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_274_0_e2608;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_275_0_e2633,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_275_0_e2614: f64 = (4.0 * w[115]);
        let noise_metadata_schedule_275_0_e2616: f64 = (noise_metadata_schedule_275_0_e2614 * w[116]);
        let noise_metadata_schedule_275_0_e2619: f64 = (1.0 + w[116]);
        let noise_metadata_schedule_275_0_e2620: f64 = (noise_metadata_schedule_275_0_e2616 * noise_metadata_schedule_275_0_e2619);
        let noise_metadata_schedule_275_0_e2621: f64 = (1.0 + noise_metadata_schedule_275_0_e2620);
        let noise_metadata_schedule_275_0_e2622: f64 = (noise_metadata_schedule_275_0_e2621).sqrt();
        let noise_metadata_schedule_275_0_e2623: f64 = (1.0 + noise_metadata_schedule_275_0_e2622);
        let noise_metadata_schedule_275_0_e2626: f64 = (2.0 * w[115]);
        let noise_metadata_schedule_275_0_e2629: f64 = (1.0 + w[116]);
        let noise_metadata_schedule_275_0_e2630: f64 = (noise_metadata_schedule_275_0_e2626 * noise_metadata_schedule_275_0_e2629);
        let noise_metadata_schedule_275_0_e2631: f64 = (noise_metadata_schedule_275_0_e2623 / noise_metadata_schedule_275_0_e2630);
        (noise_metadata_schedule_275_0_e2631,)
    } else {
        (w[117],)
    }
};
            w[117] = noise_metadata_schedule_275_0_e2633;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_276_0_e2649,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_276_0_e2637: f64 = (1.0 - w[117]);
        let noise_metadata_schedule_276_0_e2640: f64 = (w[109] * w[117]);
        let noise_metadata_schedule_276_0_e2641: f64 = (noise_metadata_schedule_276_0_e2637 + noise_metadata_schedule_276_0_e2640);
        let noise_metadata_schedule_276_0_e2645: f64 = (w[109] * w[117]);
        let noise_metadata_schedule_276_0_e2646: f64 = (1.0 + noise_metadata_schedule_276_0_e2645);
        let noise_metadata_schedule_276_0_e2647: f64 = (noise_metadata_schedule_276_0_e2641 / noise_metadata_schedule_276_0_e2646);
        (noise_metadata_schedule_276_0_e2647,)
    } else {
        (w[118],)
    }
};
            w[118] = noise_metadata_schedule_276_0_e2649;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_277_0_e2661,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_277_0_e2653: f64 = (0.5 * w[111]);
        let noise_metadata_schedule_277_0_e2655: f64 = (noise_metadata_schedule_277_0_e2653 * w[31]);
        let noise_metadata_schedule_277_0_e2657: f64 = (noise_metadata_schedule_277_0_e2655 * w[118]);
        let noise_metadata_schedule_277_0_e2659: f64 = (noise_metadata_schedule_277_0_e2657 * w[8]);
        (noise_metadata_schedule_277_0_e2659,)
    } else {
        (w[120],)
    }
};
            w[120] = noise_metadata_schedule_277_0_e2661;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_278_0_e2675,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_278_0_e2665: f64 = (2.0 * w[120]);
        let noise_metadata_schedule_278_0_e2669: f64 = (w[109] + w[120]);
        let noise_metadata_schedule_278_0_e2671: f64 = (noise_metadata_schedule_278_0_e2669 + 1.0);
        let noise_metadata_schedule_278_0_e2672: f64 = (w[109] * noise_metadata_schedule_278_0_e2671);
        let noise_metadata_schedule_278_0_e2673: f64 = (noise_metadata_schedule_278_0_e2665 + noise_metadata_schedule_278_0_e2672);
        (noise_metadata_schedule_278_0_e2673,)
    } else {
        (w[272],)
    }
};
            w[272] = noise_metadata_schedule_278_0_e2675;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_279_0_e2683,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_279_0_e2680: f64 = (w[120] - 1.0);
        let noise_metadata_schedule_279_0_e2681: f64 = (0.5 * noise_metadata_schedule_279_0_e2680);
        (noise_metadata_schedule_279_0_e2681,)
    } else {
        (w[121],)
    }
};
            w[121] = noise_metadata_schedule_279_0_e2683;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_280_0_e2691,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_280_0_e2687: f64 = (w[121] * w[121]);
        let noise_metadata_schedule_280_0_e2689: f64 = (noise_metadata_schedule_280_0_e2687 + w[272]);
        (noise_metadata_schedule_280_0_e2689,)
    } else {
        (w[266],)
    }
};
            w[266] = noise_metadata_schedule_280_0_e2691;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_281_0_e2694: f64 = if w[120] >= 1.0 { 1.0 } else { 0.0 };
            w[490] = noise_metadata_schedule_281_0_e2694;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_282_0_e2703,) = {
    if ((w[486] != 0.0) && (w[490] != 0.0)) {
        let noise_metadata_schedule_282_0_e2700: f64 = (w[266]).sqrt();
        let noise_metadata_schedule_282_0_e2701: f64 = (w[121] + noise_metadata_schedule_282_0_e2700);
        (noise_metadata_schedule_282_0_e2701,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_282_0_e2703;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_283_0_e2715,) = {
    if ((w[486] != 0.0) && (w[490] == 0.0)) {
        let noise_metadata_schedule_283_0_e2710: f64 = (w[266]).sqrt();
        let noise_metadata_schedule_283_0_e2712: f64 = (noise_metadata_schedule_283_0_e2710 - w[121]);
        let noise_metadata_schedule_283_0_e2713: f64 = (w[272] / noise_metadata_schedule_283_0_e2712);
        (noise_metadata_schedule_283_0_e2713,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_283_0_e2715;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_284_0_e2718: f64 = if w[122] < params.p139 { 1.0 } else { 0.0 };
            w[491] = noise_metadata_schedule_284_0_e2718;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_285_0_e2724,) = {
    if ((w[486] != 0.0) && (w[491] != 0.0)) {
        (params.p139,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_285_0_e2724;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_286_0_e2737,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_286_0_e2729: f64 = (w[122] + 1.0);
        let noise_metadata_schedule_286_0_e2730: f64 = (w[122] * noise_metadata_schedule_286_0_e2729);
        let noise_metadata_schedule_286_0_e2733: f64 = (w[16] * w[8]);
        let noise_metadata_schedule_286_0_e2734: f64 = (noise_metadata_schedule_286_0_e2733).exp();
        let noise_metadata_schedule_286_0_e2735: f64 = (noise_metadata_schedule_286_0_e2730 * noise_metadata_schedule_286_0_e2734);
        (noise_metadata_schedule_286_0_e2735,)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_286_0_e2737;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_287_0_e2747,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_287_0_e2741: f64 = (0.5 * params.p60);
        let noise_metadata_schedule_287_0_e2744: f64 = (w[111] - params.p61);
        let noise_metadata_schedule_287_0_e2745: f64 = (noise_metadata_schedule_287_0_e2741 * noise_metadata_schedule_287_0_e2744);
        (noise_metadata_schedule_287_0_e2745,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_287_0_e2747;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_288_0_e2757,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_288_0_e2751: f64 = (params.p60 * w[31]);
        let noise_metadata_schedule_288_0_e2753: f64 = (noise_metadata_schedule_288_0_e2751 * params.p61);
        let noise_metadata_schedule_288_0_e2755: f64 = (noise_metadata_schedule_288_0_e2753 * w[111]);
        (noise_metadata_schedule_288_0_e2755,)
    } else {
        (w[127],)
    }
};
            w[127] = noise_metadata_schedule_288_0_e2757;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_289_0_e2768,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_289_0_e2762: f64 = (w[126] * w[126]);
        let noise_metadata_schedule_289_0_e2764: f64 = (noise_metadata_schedule_289_0_e2762 + w[127]);
        let noise_metadata_schedule_289_0_e2765: f64 = (noise_metadata_schedule_289_0_e2764).sqrt();
        let noise_metadata_schedule_289_0_e2766: f64 = (w[126] + noise_metadata_schedule_289_0_e2765);
        (noise_metadata_schedule_289_0_e2766,)
    } else {
        (w[128],)
    }
};
            w[128] = noise_metadata_schedule_289_0_e2768;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_290_0_e2771: f64 = if params.p72 == 0.0 { 1.0 } else { 0.0 };
            w[492] = noise_metadata_schedule_290_0_e2771;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_291_0_e2779,) = {
    if ((w[486] != 0.0) && (w[492] != 0.0)) {
        let noise_metadata_schedule_291_0_e2777: f64 = (w[17] * 0.1);
        (noise_metadata_schedule_291_0_e2777,)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_291_0_e2779;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_292_0_e2796,) = {
    if ((w[486] != 0.0) && (w[492] == 0.0)) {
        let noise_metadata_schedule_292_0_e2788: f64 = (2.0 * w[111]);
        let noise_metadata_schedule_292_0_e2791: f64 = (w[111] + w[114]);
        let noise_metadata_schedule_292_0_e2792: f64 = (noise_metadata_schedule_292_0_e2788 / noise_metadata_schedule_292_0_e2791);
        let noise_metadata_schedule_292_0_e2793: f64 = (0.1 + noise_metadata_schedule_292_0_e2792);
        let noise_metadata_schedule_292_0_e2794: f64 = (w[17] * noise_metadata_schedule_292_0_e2793);
        (noise_metadata_schedule_292_0_e2794,)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_292_0_e2796;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_293_0_e2806,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_293_0_e2800: f64 = (params.p61 * w[111]);
        let noise_metadata_schedule_293_0_e2803: f64 = (params.p61 + w[111]);
        let noise_metadata_schedule_293_0_e2804: f64 = (noise_metadata_schedule_293_0_e2800 / noise_metadata_schedule_293_0_e2803);
        (noise_metadata_schedule_293_0_e2804,)
    } else {
        (w[130],)
    }
};
            w[130] = noise_metadata_schedule_293_0_e2806;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_294_0_e2814,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_294_0_e2811: f64 = (params.p61 + w[111]);
        let noise_metadata_schedule_294_0_e2812: f64 = (params.p61 / noise_metadata_schedule_294_0_e2811);
        (noise_metadata_schedule_294_0_e2812,)
    } else {
        (w[202],)
    }
};
            w[202] = noise_metadata_schedule_294_0_e2814;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_296_0_e2830,) = {
    if (w[486] == 0.0) {
        let noise_metadata_schedule_296_0_e2824: f64 = (2.0 * w[257]);
        let noise_metadata_schedule_296_0_e2827: f64 = (1.0 + w[107]);
        let noise_metadata_schedule_296_0_e2828: f64 = (noise_metadata_schedule_296_0_e2824 / noise_metadata_schedule_296_0_e2827);
        (noise_metadata_schedule_296_0_e2828,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_296_0_e2830;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_297_0_e2835,) = {
    if (w[486] == 0.0) {
        (w[251],)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_297_0_e2835;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_298_0_e2837: f64 = (w[242]).abs();
            let noise_metadata_schedule_298_0_e2840: f64 = (1e-5 * w[6]);
            let noise_metadata_schedule_298_0_e2843: f64 = (w[110]).abs();
            let noise_metadata_schedule_298_0_e2846: f64 = (1e-40 * w[6]);
            let noise_metadata_schedule_298_0_e2849: f64 = (w[107] + w[108]);
            let noise_metadata_schedule_298_0_e2850: f64 = (noise_metadata_schedule_298_0_e2846 * noise_metadata_schedule_298_0_e2849);
            let noise_metadata_schedule_298_0_e2852: f64 = if ((noise_metadata_schedule_298_0_e2837 < noise_metadata_schedule_298_0_e2840) || (noise_metadata_schedule_298_0_e2843 < noise_metadata_schedule_298_0_e2850)) { 1.0 } else { 0.0 };
            w[493] = noise_metadata_schedule_298_0_e2852;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_299_0_e2863,) = {
    if ((w[486] == 0.0) && (w[493] != 0.0)) {
        let noise_metadata_schedule_299_0_e2860: f64 = (w[122] + w[109]);
        let noise_metadata_schedule_299_0_e2861: f64 = (0.5 * noise_metadata_schedule_299_0_e2860);
        (noise_metadata_schedule_299_0_e2861,)
    } else {
        (w[131],)
    }
};
            w[131] = noise_metadata_schedule_299_0_e2863;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_300_0_e2874,) = {
    if ((w[486] == 0.0) && (w[493] != 0.0)) {
        let noise_metadata_schedule_300_0_e2871: f64 = (w[131] + 1.0);
        let noise_metadata_schedule_300_0_e2872: f64 = (w[131] / noise_metadata_schedule_300_0_e2871);
        (noise_metadata_schedule_300_0_e2872,)
    } else {
        (w[118],)
    }
};
            w[118] = noise_metadata_schedule_300_0_e2874;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_301_0_e2888,) = {
    if ((w[486] == 0.0) && (w[493] == 0.0)) {
        let noise_metadata_schedule_301_0_e2883: f64 = (w[110] + w[237]);
        let noise_metadata_schedule_301_0_e2885: f64 = (noise_metadata_schedule_301_0_e2883 - w[236]);
        let noise_metadata_schedule_301_0_e2886: f64 = (w[110] / noise_metadata_schedule_301_0_e2885);
        (noise_metadata_schedule_301_0_e2886,)
    } else {
        (w[118],)
    }
};
            w[118] = noise_metadata_schedule_301_0_e2888;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_302_0_e2893,) = {
    if (w[486] == 0.0) {
        (w[242],)
    } else {
        (w[128],)
    }
};
            w[128] = noise_metadata_schedule_302_0_e2893;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_303_0_e2900,) = {
    if (w[486] == 0.0) {
        let noise_metadata_schedule_303_0_e2898: f64 = (0.1 * w[17]);
        (noise_metadata_schedule_303_0_e2898,)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_303_0_e2900;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_304_0_e2905,) = {
    if (w[486] == 0.0) {
        (w[111],)
    } else {
        (w[130],)
    }
};
            w[130] = noise_metadata_schedule_304_0_e2905;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_305_0_e2914,) = {
    if (w[486] == 0.0) {
        let noise_metadata_schedule_305_0_e2911: f64 = (w[130] / params.p61);
        let noise_metadata_schedule_305_0_e2912: f64 = (1.0 - noise_metadata_schedule_305_0_e2911);
        (noise_metadata_schedule_305_0_e2912,)
    } else {
        (w[202],)
    }
};
            w[202] = noise_metadata_schedule_305_0_e2914;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_306_0_e2919: f64 = (-1.0);
            let noise_metadata_schedule_306_0_e2921: f64 = (noise_metadata_schedule_306_0_e2919 / params.p66);
            let noise_metadata_schedule_306_0_e2922: f64 = (3.0_f64).powf(noise_metadata_schedule_306_0_e2921);
            let noise_metadata_schedule_306_0_e2923: f64 = (1.0 - noise_metadata_schedule_306_0_e2922);
            let noise_metadata_schedule_306_0_e2924: f64 = (w[14] * noise_metadata_schedule_306_0_e2923);
            w[132] = noise_metadata_schedule_306_0_e2924;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_307_0_e2927: f64 = (0.1 * w[14]);
            w[279] = noise_metadata_schedule_307_0_e2927;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_308_0_e2930: f64 = (w[238] - w[132]);
            let noise_metadata_schedule_308_0_e2932: f64 = (noise_metadata_schedule_308_0_e2930 / w[279]);
            w[265] = noise_metadata_schedule_308_0_e2932;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_309_0_e2935: f64 = if w[238] < w[132] { 1.0 } else { 0.0 };
            w[494] = noise_metadata_schedule_309_0_e2935;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_310_0_e2947,) = {
    if (w[494] != 0.0) {
        let noise_metadata_schedule_310_0_e2941: f64 = (w[265]).exp();
        let noise_metadata_schedule_310_0_e2942: f64 = (1.0 + noise_metadata_schedule_310_0_e2941);
        let noise_metadata_schedule_310_0_e2943: f64 = (noise_metadata_schedule_310_0_e2942).ln();
        let noise_metadata_schedule_310_0_e2944: f64 = (w[279] * noise_metadata_schedule_310_0_e2943);
        let noise_metadata_schedule_310_0_e2945: f64 = (w[238] - noise_metadata_schedule_310_0_e2944);
        (noise_metadata_schedule_310_0_e2945,)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_310_0_e2947;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_311_0_e2961,) = {
    if (w[494] == 0.0) {
        let noise_metadata_schedule_311_0_e2954: f64 = (-w[265]);
        let noise_metadata_schedule_311_0_e2955: f64 = (noise_metadata_schedule_311_0_e2954).exp();
        let noise_metadata_schedule_311_0_e2956: f64 = (1.0 + noise_metadata_schedule_311_0_e2955);
        let noise_metadata_schedule_311_0_e2957: f64 = (noise_metadata_schedule_311_0_e2956).ln();
        let noise_metadata_schedule_311_0_e2958: f64 = (w[279] * noise_metadata_schedule_311_0_e2957);
        let noise_metadata_schedule_311_0_e2959: f64 = (w[132] - noise_metadata_schedule_311_0_e2958);
        (noise_metadata_schedule_311_0_e2959,)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_311_0_e2961;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_312_0_e2965: f64 = (w[133] * w[65]);
            let noise_metadata_schedule_312_0_e2966: f64 = (1.0 - noise_metadata_schedule_312_0_e2965);
            let noise_metadata_schedule_312_0_e2969: f64 = (1.0 - params.p66);
            let noise_metadata_schedule_312_0_e2970: f64 = (noise_metadata_schedule_312_0_e2966).powf(noise_metadata_schedule_312_0_e2969);
            w[59] = noise_metadata_schedule_312_0_e2970;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_313_0_e2974: f64 = (1.0 - params.p66);
            let noise_metadata_schedule_313_0_e2975: f64 = (w[14] / noise_metadata_schedule_313_0_e2974);
            let noise_metadata_schedule_313_0_e2978: f64 = (1.0 - w[59]);
            let noise_metadata_schedule_313_0_e2979: f64 = (noise_metadata_schedule_313_0_e2975 * noise_metadata_schedule_313_0_e2978);
            let noise_metadata_schedule_313_0_e2983: f64 = (w[238] - w[133]);
            let noise_metadata_schedule_313_0_e2984: f64 = (3.0 * noise_metadata_schedule_313_0_e2983);
            let noise_metadata_schedule_313_0_e2985: f64 = (noise_metadata_schedule_313_0_e2979 + noise_metadata_schedule_313_0_e2984);
            w[134] = noise_metadata_schedule_313_0_e2985;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_314_0_e2988: f64 = if params.p73 == 1.0 { 1.0 } else { 0.0 };
            w[495] = noise_metadata_schedule_314_0_e2988;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_315_0_e2992,) = {
    if (w[495] != 0.0) {
        (w[236],)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_315_0_e2992;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_316_0_e2995: f64 = if params.p73 == 2.0 { 1.0 } else { 0.0 };
            w[496] = noise_metadata_schedule_316_0_e2995;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_317_0_e3004,) = {
    if ((w[495] == 0.0) && (w[496] != 0.0)) {
        let noise_metadata_schedule_317_0_e3002: f64 = (w[236] + w[128]);
        (noise_metadata_schedule_317_0_e3002,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_317_0_e3004;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_318_0_e3012,) = {
    if ((w[495] == 0.0) && (w[496] == 0.0)) {
        (w[237],)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_318_0_e3012;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_319_0_e3015: f64 = (2.0 - w[25]);
            let noise_metadata_schedule_319_0_e3018: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_319_0_e3019: f64 = (noise_metadata_schedule_319_0_e3015 / noise_metadata_schedule_319_0_e3018);
            w[136] = noise_metadata_schedule_319_0_e3019;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_320_0_e3024: f64 = (-1.0);
            let noise_metadata_schedule_320_0_e3026: f64 = (noise_metadata_schedule_320_0_e3024 / params.p71);
            let noise_metadata_schedule_320_0_e3027: f64 = (w[136]).powf(noise_metadata_schedule_320_0_e3026);
            let noise_metadata_schedule_320_0_e3028: f64 = (1.0 - noise_metadata_schedule_320_0_e3027);
            let noise_metadata_schedule_320_0_e3029: f64 = (w[17] * noise_metadata_schedule_320_0_e3028);
            w[137] = noise_metadata_schedule_320_0_e3029;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_321_0_e3032: f64 = (w[135] - w[137]);
            let noise_metadata_schedule_321_0_e3034: f64 = (noise_metadata_schedule_321_0_e3032 / w[129]);
            w[265] = noise_metadata_schedule_321_0_e3034;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_322_0_e3037: f64 = if w[135] < w[137] { 1.0 } else { 0.0 };
            w[497] = noise_metadata_schedule_322_0_e3037;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_323_0_e3049,) = {
    if (w[497] != 0.0) {
        let noise_metadata_schedule_323_0_e3043: f64 = (w[265]).exp();
        let noise_metadata_schedule_323_0_e3044: f64 = (1.0 + noise_metadata_schedule_323_0_e3043);
        let noise_metadata_schedule_323_0_e3045: f64 = (noise_metadata_schedule_323_0_e3044).ln();
        let noise_metadata_schedule_323_0_e3046: f64 = (w[129] * noise_metadata_schedule_323_0_e3045);
        let noise_metadata_schedule_323_0_e3047: f64 = (w[135] - noise_metadata_schedule_323_0_e3046);
        (noise_metadata_schedule_323_0_e3047,)
    } else {
        (w[138],)
    }
};
            w[138] = noise_metadata_schedule_323_0_e3049;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_324_0_e3063,) = {
    if (w[497] == 0.0) {
        let noise_metadata_schedule_324_0_e3056: f64 = (-w[265]);
        let noise_metadata_schedule_324_0_e3057: f64 = (noise_metadata_schedule_324_0_e3056).exp();
        let noise_metadata_schedule_324_0_e3058: f64 = (1.0 + noise_metadata_schedule_324_0_e3057);
        let noise_metadata_schedule_324_0_e3059: f64 = (noise_metadata_schedule_324_0_e3058).ln();
        let noise_metadata_schedule_324_0_e3060: f64 = (w[129] * noise_metadata_schedule_324_0_e3059);
        let noise_metadata_schedule_324_0_e3061: f64 = (w[137] - noise_metadata_schedule_324_0_e3060);
        (noise_metadata_schedule_324_0_e3061,)
    } else {
        (w[138],)
    }
};
            w[138] = noise_metadata_schedule_324_0_e3063;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_325_0_e3066: f64 = (w[202]).powf(params.p75);
            w[139] = noise_metadata_schedule_325_0_e3066;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_326_0_e3070: f64 = (1.0 - params.p71);
            let noise_metadata_schedule_326_0_e3071: f64 = (w[17] / noise_metadata_schedule_326_0_e3070);
            let noise_metadata_schedule_326_0_e3077: f64 = (w[138] / w[17]);
            let noise_metadata_schedule_326_0_e3078: f64 = (1.0 - noise_metadata_schedule_326_0_e3077);
            let noise_metadata_schedule_326_0_e3081: f64 = (1.0 - params.p71);
            let noise_metadata_schedule_326_0_e3082: f64 = (noise_metadata_schedule_326_0_e3078).powf(noise_metadata_schedule_326_0_e3081);
            let noise_metadata_schedule_326_0_e3083: f64 = (w[139] * noise_metadata_schedule_326_0_e3082);
            let noise_metadata_schedule_326_0_e3084: f64 = (1.0 - noise_metadata_schedule_326_0_e3083);
            let noise_metadata_schedule_326_0_e3085: f64 = (noise_metadata_schedule_326_0_e3071 * noise_metadata_schedule_326_0_e3084);
            let noise_metadata_schedule_326_0_e3088: f64 = (w[139] * w[136]);
            let noise_metadata_schedule_326_0_e3091: f64 = (w[135] - w[138]);
            let noise_metadata_schedule_326_0_e3092: f64 = (noise_metadata_schedule_326_0_e3088 * noise_metadata_schedule_326_0_e3091);
            let noise_metadata_schedule_326_0_e3093: f64 = (noise_metadata_schedule_326_0_e3085 + noise_metadata_schedule_326_0_e3092);
            w[140] = noise_metadata_schedule_326_0_e3093;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_327_0_e3096: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_327_0_e3098: f64 = (noise_metadata_schedule_327_0_e3096 * w[140]);
            let noise_metadata_schedule_327_0_e3101: f64 = (w[25] * w[236]);
            let noise_metadata_schedule_327_0_e3102: f64 = (noise_metadata_schedule_327_0_e3098 + noise_metadata_schedule_327_0_e3101);
            w[141] = noise_metadata_schedule_327_0_e3102;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_328_0_e3105: f64 = (4.0 * w[35]);
            let noise_metadata_schedule_328_0_e3107: f64 = (noise_metadata_schedule_328_0_e3105 / w[36]);
            w[142] = noise_metadata_schedule_328_0_e3107;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_329_0_e3110: f64 = (w[142] * w[252]);
            w[143] = noise_metadata_schedule_329_0_e3110;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_330_0_e3115: f64 = (1.0 + w[143]);
            let noise_metadata_schedule_330_0_e3116: f64 = (noise_metadata_schedule_330_0_e3115).sqrt();
            let noise_metadata_schedule_330_0_e3117: f64 = (1.0 + noise_metadata_schedule_330_0_e3116);
            let noise_metadata_schedule_330_0_e3118: f64 = (w[143] / noise_metadata_schedule_330_0_e3117);
            w[145] = noise_metadata_schedule_330_0_e3118;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_331_0_e3122: f64 = (1.0 / w[49]);
            let noise_metadata_schedule_331_0_e3123: f64 = (w[124]).powf(noise_metadata_schedule_331_0_e3122);
            w[125] = noise_metadata_schedule_331_0_e3123;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_332_0_e3126: f64 = (w[142] * w[125]);
            w[144] = noise_metadata_schedule_332_0_e3126;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_333_0_e3131: f64 = (1.0 + w[144]);
            let noise_metadata_schedule_333_0_e3132: f64 = (noise_metadata_schedule_333_0_e3131).sqrt();
            let noise_metadata_schedule_333_0_e3133: f64 = (1.0 + noise_metadata_schedule_333_0_e3132);
            let noise_metadata_schedule_333_0_e3134: f64 = (w[144] / noise_metadata_schedule_333_0_e3133);
            w[146] = noise_metadata_schedule_333_0_e3134;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_334_0_e3137: f64 = if params.p91 == 0.0 { 1.0 } else { 0.0 };
            w[498] = noise_metadata_schedule_334_0_e3137;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_335_0_e3149,) = {
    if (w[498] != 0.0) {
        let noise_metadata_schedule_335_0_e3142: f64 = (w[134] / w[41]);
        let noise_metadata_schedule_335_0_e3143: f64 = (1.0 + noise_metadata_schedule_335_0_e3142);
        let noise_metadata_schedule_335_0_e3146: f64 = (w[141] / w[40]);
        let noise_metadata_schedule_335_0_e3147: f64 = (noise_metadata_schedule_335_0_e3143 + noise_metadata_schedule_335_0_e3146);
        (noise_metadata_schedule_335_0_e3147,)
    } else {
        (w[147],)
    }
};
            w[147] = noise_metadata_schedule_335_0_e3149;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_336_0_e3162,) = {
    if (w[498] == 0.0) {
        let noise_metadata_schedule_336_0_e3154: f64 = (w[134] / w[41]);
        let noise_metadata_schedule_336_0_e3156: f64 = (noise_metadata_schedule_336_0_e3154 + 1.0);
        let noise_metadata_schedule_336_0_e3158: f64 = (noise_metadata_schedule_336_0_e3156 * w[100]);
        let noise_metadata_schedule_336_0_e3160: f64 = (noise_metadata_schedule_336_0_e3158 * w[8]);
        (noise_metadata_schedule_336_0_e3160,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_336_0_e3162;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_337_0_e3174,) = {
    if (w[498] == 0.0) {
        let noise_metadata_schedule_337_0_e3166: f64 = (-w[141]);
        let noise_metadata_schedule_337_0_e3168: f64 = (noise_metadata_schedule_337_0_e3166 / w[40]);
        let noise_metadata_schedule_337_0_e3170: f64 = (noise_metadata_schedule_337_0_e3168 * w[100]);
        let noise_metadata_schedule_337_0_e3172: f64 = (noise_metadata_schedule_337_0_e3170 * w[8]);
        (noise_metadata_schedule_337_0_e3172,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_337_0_e3174;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_338_0_e3190,) = {
    if (w[498] == 0.0) {
        let noise_metadata_schedule_338_0_e3178: f64 = (w[275]).exp();
        let noise_metadata_schedule_338_0_e3180: f64 = (w[276]).exp();
        let noise_metadata_schedule_338_0_e3181: f64 = (noise_metadata_schedule_338_0_e3178 - noise_metadata_schedule_338_0_e3180);
        let noise_metadata_schedule_338_0_e3184: f64 = (w[100] * w[8]);
        let noise_metadata_schedule_338_0_e3185: f64 = (noise_metadata_schedule_338_0_e3184).exp();
        let noise_metadata_schedule_338_0_e3187: f64 = (noise_metadata_schedule_338_0_e3185 - 1.0);
        let noise_metadata_schedule_338_0_e3188: f64 = (noise_metadata_schedule_338_0_e3181 / noise_metadata_schedule_338_0_e3187);
        (noise_metadata_schedule_338_0_e3188,)
    } else {
        (w[147],)
    }
};
            w[147] = noise_metadata_schedule_338_0_e3190;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_339_0_e3193: f64 = (0.1 * 0.1);
            w[267] = noise_metadata_schedule_339_0_e3193;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_340_0_e3196: f64 = (w[147] * w[147]);
            w[268] = noise_metadata_schedule_340_0_e3196;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_341_0_e3199: f64 = if w[147] < 0.0 { 1.0 } else { 0.0 };
            w[499] = noise_metadata_schedule_341_0_e3199;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_342_0_e3212,) = {
    if (w[499] != 0.0) {
        let noise_metadata_schedule_342_0_e3203: f64 = (0.5 * w[267]);
        let noise_metadata_schedule_342_0_e3206: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_342_0_e3207: f64 = (noise_metadata_schedule_342_0_e3206).sqrt();
        let noise_metadata_schedule_342_0_e3209: f64 = (noise_metadata_schedule_342_0_e3207 - w[147]);
        let noise_metadata_schedule_342_0_e3210: f64 = (noise_metadata_schedule_342_0_e3203 / noise_metadata_schedule_342_0_e3209);
        (noise_metadata_schedule_342_0_e3210,)
    } else {
        (w[148],)
    }
};
            w[148] = noise_metadata_schedule_342_0_e3212;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_343_0_e3224,) = {
    if (w[499] == 0.0) {
        let noise_metadata_schedule_343_0_e3218: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_343_0_e3219: f64 = (noise_metadata_schedule_343_0_e3218).sqrt();
        let noise_metadata_schedule_343_0_e3221: f64 = (noise_metadata_schedule_343_0_e3219 + w[147]);
        let noise_metadata_schedule_343_0_e3222: f64 = (0.5 * noise_metadata_schedule_343_0_e3221);
        (noise_metadata_schedule_343_0_e3222,)
    } else {
        (w[148],)
    }
};
            w[148] = noise_metadata_schedule_343_0_e3224;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_344_0_e3230: f64 = (w[145] + w[146]);
            let noise_metadata_schedule_344_0_e3231: f64 = (0.5 * noise_metadata_schedule_344_0_e3230);
            let noise_metadata_schedule_344_0_e3232: f64 = (1.0 + noise_metadata_schedule_344_0_e3231);
            let noise_metadata_schedule_344_0_e3233: f64 = (w[148] * noise_metadata_schedule_344_0_e3232);
            w[149] = noise_metadata_schedule_344_0_e3233;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_345_0_e3236: f64 = (params.p14 * w[35]);
            let noise_metadata_schedule_345_0_e3238: f64 = (noise_metadata_schedule_345_0_e3236 * w[125]);
            w[150] = noise_metadata_schedule_345_0_e3238;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_346_0_e3241: f64 = (w[35] * w[252]);
            w[151] = noise_metadata_schedule_346_0_e3241;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_347_0_e3244: f64 = (w[151] - w[150]);
            let noise_metadata_schedule_347_0_e3246: f64 = (noise_metadata_schedule_347_0_e3244 / w[149]);
            w[152] = noise_metadata_schedule_347_0_e3246;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_348_0_e3249: f64 = w[238];
            let noise_metadata_schedule_348_0_e3251: f64 = (noise_metadata_schedule_348_0_e3249 / 0.0001);
            w[265] = noise_metadata_schedule_348_0_e3251;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_349_0_e3254: f64 = if w[238] < 0.0 { 1.0 } else { 0.0 };
            w[500] = noise_metadata_schedule_349_0_e3254;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_350_0_e3266,) = {
    if (w[500] != 0.0) {
        let noise_metadata_schedule_350_0_e3260: f64 = (w[265]).exp();
        let noise_metadata_schedule_350_0_e3261: f64 = (1.0 + noise_metadata_schedule_350_0_e3260);
        let noise_metadata_schedule_350_0_e3262: f64 = (noise_metadata_schedule_350_0_e3261).ln();
        let noise_metadata_schedule_350_0_e3263: f64 = (0.0001 * noise_metadata_schedule_350_0_e3262);
        let noise_metadata_schedule_350_0_e3264: f64 = noise_metadata_schedule_350_0_e3263;
        (noise_metadata_schedule_350_0_e3264,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_350_0_e3266;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_351_0_e3280,) = {
    if (w[500] == 0.0) {
        let noise_metadata_schedule_351_0_e3273: f64 = (-w[265]);
        let noise_metadata_schedule_351_0_e3274: f64 = (noise_metadata_schedule_351_0_e3273).exp();
        let noise_metadata_schedule_351_0_e3275: f64 = (1.0 + noise_metadata_schedule_351_0_e3274);
        let noise_metadata_schedule_351_0_e3276: f64 = (noise_metadata_schedule_351_0_e3275).ln();
        let noise_metadata_schedule_351_0_e3277: f64 = (0.0001 * noise_metadata_schedule_351_0_e3276);
        let noise_metadata_schedule_351_0_e3278: f64 = (w[238] + noise_metadata_schedule_351_0_e3277);
        (noise_metadata_schedule_351_0_e3278,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_351_0_e3280;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_352_0_e3283: f64 = (w[282] / params.p143);
            w[284] = noise_metadata_schedule_352_0_e3283;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_353_0_e3286: f64 = if w[284] < params.p138 { 1.0 } else { 0.0 };
            w[501] = noise_metadata_schedule_353_0_e3286;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_354_0_e3291,) = {
    if (w[501] != 0.0) {
        let noise_metadata_schedule_354_0_e3289: f64 = (w[284]).exp();
        (noise_metadata_schedule_354_0_e3289,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_354_0_e3291;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_355_0_e3297,) = {
    if (w[501] == 0.0) {
        let noise_metadata_schedule_355_0_e3295: f64 = (params.p138).exp();
        (noise_metadata_schedule_355_0_e3295,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_355_0_e3297;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_356_0_e3308,) = {
    if (w[501] == 0.0) {
        let noise_metadata_schedule_356_0_e3304: f64 = (w[284] - params.p138);
        let noise_metadata_schedule_356_0_e3305: f64 = (1.0 + noise_metadata_schedule_356_0_e3304);
        let noise_metadata_schedule_356_0_e3306: f64 = (w[281] * noise_metadata_schedule_356_0_e3305);
        (noise_metadata_schedule_356_0_e3306,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_356_0_e3308;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_357_0_e3312: f64 = (w[285] - 1.0);
            let noise_metadata_schedule_357_0_e3313: f64 = (w[332] * noise_metadata_schedule_357_0_e3312);
            w[333] = noise_metadata_schedule_357_0_e3313;
        }
        if (active[0] & 0x6) != 0 {
            let noise_metadata_schedule_358_0_e3316: f64 = (w[238] - params.p145);
            let noise_metadata_schedule_358_0_e3318: f64 = (noise_metadata_schedule_358_0_e3316 / 0.001);
            w[265] = noise_metadata_schedule_358_0_e3318;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_359_0_e3321: f64 = if w[238] < params.p145 { 1.0 } else { 0.0 };
            w[502] = noise_metadata_schedule_359_0_e3321;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_360_0_e3333,) = {
    if (w[502] != 0.0) {
        let noise_metadata_schedule_360_0_e3327: f64 = (w[265]).exp();
        let noise_metadata_schedule_360_0_e3328: f64 = (1.0 + noise_metadata_schedule_360_0_e3327);
        let noise_metadata_schedule_360_0_e3329: f64 = (noise_metadata_schedule_360_0_e3328).ln();
        let noise_metadata_schedule_360_0_e3330: f64 = (0.001 * noise_metadata_schedule_360_0_e3329);
        let noise_metadata_schedule_360_0_e3331: f64 = (w[238] - noise_metadata_schedule_360_0_e3330);
        (noise_metadata_schedule_360_0_e3331,)
    } else {
        (w[286],)
    }
};
            w[286] = noise_metadata_schedule_360_0_e3333;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_361_0_e3347,) = {
    if (w[502] == 0.0) {
        let noise_metadata_schedule_361_0_e3340: f64 = (-w[265]);
        let noise_metadata_schedule_361_0_e3341: f64 = (noise_metadata_schedule_361_0_e3340).exp();
        let noise_metadata_schedule_361_0_e3342: f64 = (1.0 + noise_metadata_schedule_361_0_e3341);
        let noise_metadata_schedule_361_0_e3343: f64 = (noise_metadata_schedule_361_0_e3342).ln();
        let noise_metadata_schedule_361_0_e3344: f64 = (0.001 * noise_metadata_schedule_361_0_e3343);
        let noise_metadata_schedule_361_0_e3345: f64 = (params.p145 - noise_metadata_schedule_361_0_e3344);
        (noise_metadata_schedule_361_0_e3345,)
    } else {
        (w[286],)
    }
};
            w[286] = noise_metadata_schedule_361_0_e3347;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_362_0_e3350: f64 = (params.p146 * w[286]);
            let noise_metadata_schedule_362_0_e3353: f64 = (params.p145 - w[286]);
            let noise_metadata_schedule_362_0_e3355: f64 = {let pb=noise_metadata_schedule_362_0_e3353;pb*pb};
            let noise_metadata_schedule_362_0_e3356: f64 = (noise_metadata_schedule_362_0_e3350 * noise_metadata_schedule_362_0_e3355);
            w[334] = noise_metadata_schedule_362_0_e3356;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_363_0_e3359: f64 = (w[238] * w[8]);
            let noise_metadata_schedule_363_0_e3361: f64 = (noise_metadata_schedule_363_0_e3359 / params.p16);
            let noise_metadata_schedule_363_0_e3363: f64 = if noise_metadata_schedule_363_0_e3361 < params.p138 { 1.0 } else { 0.0 };
            w[503] = noise_metadata_schedule_363_0_e3363;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_364_0_e3372,) = {
    if (w[503] != 0.0) {
        let noise_metadata_schedule_364_0_e3367: f64 = (w[238] * w[8]);
        let noise_metadata_schedule_364_0_e3369: f64 = (noise_metadata_schedule_364_0_e3367 / params.p16);
        let noise_metadata_schedule_364_0_e3370: f64 = (noise_metadata_schedule_364_0_e3369).exp();
        (noise_metadata_schedule_364_0_e3370,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_364_0_e3372;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_365_0_e3378,) = {
    if (w[503] == 0.0) {
        let noise_metadata_schedule_365_0_e3376: f64 = (params.p138).exp();
        (noise_metadata_schedule_365_0_e3376,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_365_0_e3378;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_366_0_e3393,) = {
    if (w[503] == 0.0) {
        let noise_metadata_schedule_366_0_e3385: f64 = (w[238] * w[8]);
        let noise_metadata_schedule_366_0_e3387: f64 = (noise_metadata_schedule_366_0_e3385 / params.p16);
        let noise_metadata_schedule_366_0_e3389: f64 = (noise_metadata_schedule_366_0_e3387 - params.p138);
        let noise_metadata_schedule_366_0_e3390: f64 = (1.0 + noise_metadata_schedule_366_0_e3389);
        let noise_metadata_schedule_366_0_e3391: f64 = (w[281] * noise_metadata_schedule_366_0_e3390);
        (noise_metadata_schedule_366_0_e3391,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_366_0_e3393;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_367_0_e3396: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            w[504] = noise_metadata_schedule_367_0_e3396;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_368_0_e3399: f64 = (w[238] - w[55]);
            let noise_metadata_schedule_368_0_e3401: f64 = (noise_metadata_schedule_368_0_e3399 * w[8]);
            let noise_metadata_schedule_368_0_e3403: f64 = if noise_metadata_schedule_368_0_e3401 < params.p138 { 1.0 } else { 0.0 };
            w[505] = noise_metadata_schedule_368_0_e3403;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_369_0_e3414,) = {
    if ((w[504] != 0.0) && (w[505] != 0.0)) {
        let noise_metadata_schedule_369_0_e3409: f64 = (w[238] - w[55]);
        let noise_metadata_schedule_369_0_e3411: f64 = (noise_metadata_schedule_369_0_e3409 * w[8]);
        let noise_metadata_schedule_369_0_e3412: f64 = (noise_metadata_schedule_369_0_e3411).exp();
        (noise_metadata_schedule_369_0_e3412,)
    } else {
        (w[284],)
    }
};
            w[284] = noise_metadata_schedule_369_0_e3414;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_370_0_e3422,) = {
    if ((w[504] != 0.0) && (w[505] == 0.0)) {
        let noise_metadata_schedule_370_0_e3420: f64 = (params.p138).exp();
        (noise_metadata_schedule_370_0_e3420,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_370_0_e3422;
        }
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_371_0_e3439,) = {
    if ((w[504] != 0.0) && (w[505] == 0.0)) {
        let noise_metadata_schedule_371_0_e3431: f64 = (w[238] - w[55]);
        let noise_metadata_schedule_371_0_e3433: f64 = (noise_metadata_schedule_371_0_e3431 * w[8]);
        let noise_metadata_schedule_371_0_e3435: f64 = (noise_metadata_schedule_371_0_e3433 - params.p138);
        let noise_metadata_schedule_371_0_e3436: f64 = (1.0 + noise_metadata_schedule_371_0_e3435);
        let noise_metadata_schedule_371_0_e3437: f64 = (w[281] * noise_metadata_schedule_371_0_e3436);
        (noise_metadata_schedule_371_0_e3437,)
    } else {
        (w[284],)
    }
};
            w[284] = noise_metadata_schedule_371_0_e3439;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_372_0_e3442: f64 = (w[152] / w[35]);
            let noise_metadata_schedule_372_0_e3444: f64 = (noise_metadata_schedule_372_0_e3442 - 1000.0);
            let noise_metadata_schedule_372_0_e3446: f64 = if noise_metadata_schedule_372_0_e3444 < 40.0 { 1.0 } else { 0.0 };
            w[506] = noise_metadata_schedule_372_0_e3446;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_373_0_e3457,) = {
    if ((w[504] != 0.0) && (w[506] != 0.0)) {
        let noise_metadata_schedule_373_0_e3452: f64 = (w[152] / w[35]);
        let noise_metadata_schedule_373_0_e3454: f64 = (noise_metadata_schedule_373_0_e3452 - 1000.0);
        let noise_metadata_schedule_373_0_e3455: f64 = (noise_metadata_schedule_373_0_e3454).exp();
        (noise_metadata_schedule_373_0_e3455,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_373_0_e3457;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_374_0_e3465,) = {
    if ((w[504] != 0.0) && (w[506] == 0.0)) {
        let noise_metadata_schedule_374_0_e3463: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_374_0_e3463,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_374_0_e3465;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_375_0_e3482,) = {
    if ((w[504] != 0.0) && (w[506] == 0.0)) {
        let noise_metadata_schedule_375_0_e3474: f64 = (w[152] / w[35]);
        let noise_metadata_schedule_375_0_e3476: f64 = (noise_metadata_schedule_375_0_e3474 - 1000.0);
        let noise_metadata_schedule_375_0_e3478: f64 = (noise_metadata_schedule_375_0_e3476 - 40.0);
        let noise_metadata_schedule_375_0_e3479: f64 = (1.0 + noise_metadata_schedule_375_0_e3478);
        let noise_metadata_schedule_375_0_e3480: f64 = (w[281] * noise_metadata_schedule_375_0_e3479);
        (noise_metadata_schedule_375_0_e3480,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_375_0_e3482;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_376_0_e3525,) = {
    if (w[504] != 0.0) {
        let noise_metadata_schedule_376_0_e3487: f64 = (w[282] - 1.0);
        let noise_metadata_schedule_376_0_e3488: f64 = (w[42] * noise_metadata_schedule_376_0_e3487);
        let noise_metadata_schedule_376_0_e3491: f64 = (w[53] * 2.0);
        let noise_metadata_schedule_376_0_e3494: f64 = (w[282] - 1.0);
        let noise_metadata_schedule_376_0_e3495: f64 = (noise_metadata_schedule_376_0_e3491 * noise_metadata_schedule_376_0_e3494);
        let noise_metadata_schedule_376_0_e3500: f64 = (4.0 * w[284]);
        let noise_metadata_schedule_376_0_e3501: f64 = (1.0 + noise_metadata_schedule_376_0_e3500);
        let noise_metadata_schedule_376_0_e3502: f64 = (noise_metadata_schedule_376_0_e3501).sqrt();
        let noise_metadata_schedule_376_0_e3503: f64 = (1.0 + noise_metadata_schedule_376_0_e3502);
        let noise_metadata_schedule_376_0_e3504: f64 = (noise_metadata_schedule_376_0_e3495 / noise_metadata_schedule_376_0_e3503);
        let noise_metadata_schedule_376_0_e3508: f64 = (w[141] / w[40]);
        let noise_metadata_schedule_376_0_e3509: f64 = (1.0 + noise_metadata_schedule_376_0_e3508);
        let noise_metadata_schedule_376_0_e3510: f64 = (noise_metadata_schedule_376_0_e3504 * noise_metadata_schedule_376_0_e3509);
        let noise_metadata_schedule_376_0_e3511: f64 = (noise_metadata_schedule_376_0_e3488 + noise_metadata_schedule_376_0_e3510);
        let noise_metadata_schedule_376_0_e3515: f64 = (w[124] - 1.0);
        let noise_metadata_schedule_376_0_e3516: f64 = (w[54] * noise_metadata_schedule_376_0_e3515);
        let noise_metadata_schedule_376_0_e3518: f64 = (noise_metadata_schedule_376_0_e3516 * w[285]);
        let noise_metadata_schedule_376_0_e3521: f64 = (1.0 + w[285]);
        let noise_metadata_schedule_376_0_e3522: f64 = (noise_metadata_schedule_376_0_e3518 / noise_metadata_schedule_376_0_e3521);
        let noise_metadata_schedule_376_0_e3523: f64 = (noise_metadata_schedule_376_0_e3511 + noise_metadata_schedule_376_0_e3522);
        (noise_metadata_schedule_376_0_e3523,)
    } else {
        (w[154],)
    }
};
            w[154] = noise_metadata_schedule_376_0_e3525;
        }
        if (active[0] & 0x44) != 0 {
            let noise_metadata_schedule_377_0_e3528: f64 = if params.p92 == 0.0 { 1.0 } else { 0.0 };
            w[507] = noise_metadata_schedule_377_0_e3528;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_378_0_e3539,) = {
    if ((w[504] == 0.0) && (w[507] != 0.0)) {
        let noise_metadata_schedule_378_0_e3536: f64 = (w[282] - 1.0);
        let noise_metadata_schedule_378_0_e3537: f64 = (w[42] * noise_metadata_schedule_378_0_e3536);
        (noise_metadata_schedule_378_0_e3537,)
    } else {
        (w[154],)
    }
};
            w[154] = noise_metadata_schedule_378_0_e3539;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_379_0_e3569,) = {
    if ((w[504] == 0.0) && (w[507] == 0.0)) {
        let noise_metadata_schedule_379_0_e3548: f64 = (1.0 - params.p92);
        let noise_metadata_schedule_379_0_e3551: f64 = (w[282] - 1.0);
        let noise_metadata_schedule_379_0_e3552: f64 = (noise_metadata_schedule_379_0_e3548 * noise_metadata_schedule_379_0_e3551);
        let noise_metadata_schedule_379_0_e3556: f64 = (w[282] + w[124]);
        let noise_metadata_schedule_379_0_e3558: f64 = (noise_metadata_schedule_379_0_e3556 - 2.0);
        let noise_metadata_schedule_379_0_e3559: f64 = (params.p92 * noise_metadata_schedule_379_0_e3558);
        let noise_metadata_schedule_379_0_e3563: f64 = (w[141] / w[40]);
        let noise_metadata_schedule_379_0_e3564: f64 = (1.0 + noise_metadata_schedule_379_0_e3563);
        let noise_metadata_schedule_379_0_e3565: f64 = (noise_metadata_schedule_379_0_e3559 * noise_metadata_schedule_379_0_e3564);
        let noise_metadata_schedule_379_0_e3566: f64 = (noise_metadata_schedule_379_0_e3552 + noise_metadata_schedule_379_0_e3565);
        let noise_metadata_schedule_379_0_e3567: f64 = (w[42] * noise_metadata_schedule_379_0_e3566);
        (noise_metadata_schedule_379_0_e3567,)
    } else {
        (w[154],)
    }
};
            w[154] = noise_metadata_schedule_379_0_e3569;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_380_0_e3572: f64 = (w[239] * w[8]);
            let noise_metadata_schedule_380_0_e3574: f64 = (noise_metadata_schedule_380_0_e3572 / params.p18);
            let noise_metadata_schedule_380_0_e3576: f64 = if noise_metadata_schedule_380_0_e3574 < params.p138 { 1.0 } else { 0.0 };
            w[508] = noise_metadata_schedule_380_0_e3576;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_381_0_e3585,) = {
    if (w[508] != 0.0) {
        let noise_metadata_schedule_381_0_e3580: f64 = (w[239] * w[8]);
        let noise_metadata_schedule_381_0_e3582: f64 = (noise_metadata_schedule_381_0_e3580 / params.p18);
        let noise_metadata_schedule_381_0_e3583: f64 = (noise_metadata_schedule_381_0_e3582).exp();
        (noise_metadata_schedule_381_0_e3583,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_381_0_e3585;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_382_0_e3591,) = {
    if (w[508] == 0.0) {
        let noise_metadata_schedule_382_0_e3589: f64 = (params.p138).exp();
        (noise_metadata_schedule_382_0_e3589,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_382_0_e3591;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_383_0_e3606,) = {
    if (w[508] == 0.0) {
        let noise_metadata_schedule_383_0_e3598: f64 = (w[239] * w[8]);
        let noise_metadata_schedule_383_0_e3600: f64 = (noise_metadata_schedule_383_0_e3598 / params.p18);
        let noise_metadata_schedule_383_0_e3602: f64 = (noise_metadata_schedule_383_0_e3600 - params.p138);
        let noise_metadata_schedule_383_0_e3603: f64 = (1.0 + noise_metadata_schedule_383_0_e3602);
        let noise_metadata_schedule_383_0_e3604: f64 = (w[281] * noise_metadata_schedule_383_0_e3603);
        (noise_metadata_schedule_383_0_e3604,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_383_0_e3606;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_384_0_e3609: f64 = if params.p23 == 1.0 { 1.0 } else { 0.0 };
            w[509] = noise_metadata_schedule_384_0_e3609;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_385_0_e3612: f64 = (w[239] - w[55]);
            let noise_metadata_schedule_385_0_e3614: f64 = (noise_metadata_schedule_385_0_e3612 * w[8]);
            let noise_metadata_schedule_385_0_e3616: f64 = if noise_metadata_schedule_385_0_e3614 < params.p138 { 1.0 } else { 0.0 };
            w[510] = noise_metadata_schedule_385_0_e3616;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_386_0_e3627,) = {
    if ((w[509] != 0.0) && (w[510] != 0.0)) {
        let noise_metadata_schedule_386_0_e3622: f64 = (w[239] - w[55]);
        let noise_metadata_schedule_386_0_e3624: f64 = (noise_metadata_schedule_386_0_e3622 * w[8]);
        let noise_metadata_schedule_386_0_e3625: f64 = (noise_metadata_schedule_386_0_e3624).exp();
        (noise_metadata_schedule_386_0_e3625,)
    } else {
        (w[284],)
    }
};
            w[284] = noise_metadata_schedule_386_0_e3627;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_387_0_e3635,) = {
    if ((w[509] != 0.0) && (w[510] == 0.0)) {
        let noise_metadata_schedule_387_0_e3633: f64 = (params.p138).exp();
        (noise_metadata_schedule_387_0_e3633,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_387_0_e3635;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_388_0_e3652,) = {
    if ((w[509] != 0.0) && (w[510] == 0.0)) {
        let noise_metadata_schedule_388_0_e3644: f64 = (w[239] - w[55]);
        let noise_metadata_schedule_388_0_e3646: f64 = (noise_metadata_schedule_388_0_e3644 * w[8]);
        let noise_metadata_schedule_388_0_e3648: f64 = (noise_metadata_schedule_388_0_e3646 - params.p138);
        let noise_metadata_schedule_388_0_e3649: f64 = (1.0 + noise_metadata_schedule_388_0_e3648);
        let noise_metadata_schedule_388_0_e3650: f64 = (w[281] * noise_metadata_schedule_388_0_e3649);
        (noise_metadata_schedule_388_0_e3650,)
    } else {
        (w[284],)
    }
};
            w[284] = noise_metadata_schedule_388_0_e3652;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_389_0_e3677,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_389_0_e3657: f64 = (w[282] - 1.0);
        let noise_metadata_schedule_389_0_e3658: f64 = (w[44] * noise_metadata_schedule_389_0_e3657);
        let noise_metadata_schedule_389_0_e3661: f64 = (w[45] * 2.0);
        let noise_metadata_schedule_389_0_e3664: f64 = (w[282] - 1.0);
        let noise_metadata_schedule_389_0_e3665: f64 = (noise_metadata_schedule_389_0_e3661 * noise_metadata_schedule_389_0_e3664);
        let noise_metadata_schedule_389_0_e3670: f64 = (4.0 * w[284]);
        let noise_metadata_schedule_389_0_e3671: f64 = (1.0 + noise_metadata_schedule_389_0_e3670);
        let noise_metadata_schedule_389_0_e3672: f64 = (noise_metadata_schedule_389_0_e3671).sqrt();
        let noise_metadata_schedule_389_0_e3673: f64 = (1.0 + noise_metadata_schedule_389_0_e3672);
        let noise_metadata_schedule_389_0_e3674: f64 = (noise_metadata_schedule_389_0_e3665 / noise_metadata_schedule_389_0_e3673);
        let noise_metadata_schedule_389_0_e3675: f64 = (noise_metadata_schedule_389_0_e3658 + noise_metadata_schedule_389_0_e3674);
        (noise_metadata_schedule_389_0_e3675,)
    } else {
        (w[155],)
    }
};
            w[155] = noise_metadata_schedule_389_0_e3677;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_390_0_e3686,) = {
    if (w[509] == 0.0) {
        let noise_metadata_schedule_390_0_e3683: f64 = (w[282] - 1.0);
        let noise_metadata_schedule_390_0_e3684: f64 = (w[44] * noise_metadata_schedule_390_0_e3683);
        (noise_metadata_schedule_390_0_e3684,)
    } else {
        (w[155],)
    }
};
            w[155] = noise_metadata_schedule_390_0_e3686;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_391_0_e3689: f64 = (w[238] * w[8]);
            let noise_metadata_schedule_391_0_e3691: f64 = (noise_metadata_schedule_391_0_e3689 / params.p20);
            let noise_metadata_schedule_391_0_e3693: f64 = if noise_metadata_schedule_391_0_e3691 < params.p138 { 1.0 } else { 0.0 };
            w[511] = noise_metadata_schedule_391_0_e3693;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_392_0_e3702,) = {
    if (w[511] != 0.0) {
        let noise_metadata_schedule_392_0_e3697: f64 = (w[238] * w[8]);
        let noise_metadata_schedule_392_0_e3699: f64 = (noise_metadata_schedule_392_0_e3697 / params.p20);
        let noise_metadata_schedule_392_0_e3700: f64 = (noise_metadata_schedule_392_0_e3699).exp();
        (noise_metadata_schedule_392_0_e3700,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_392_0_e3702;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_393_0_e3708,) = {
    if (w[511] == 0.0) {
        let noise_metadata_schedule_393_0_e3706: f64 = (params.p138).exp();
        (noise_metadata_schedule_393_0_e3706,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_393_0_e3708;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_394_0_e3723,) = {
    if (w[511] == 0.0) {
        let noise_metadata_schedule_394_0_e3715: f64 = (w[238] * w[8]);
        let noise_metadata_schedule_394_0_e3717: f64 = (noise_metadata_schedule_394_0_e3715 / params.p20);
        let noise_metadata_schedule_394_0_e3719: f64 = (noise_metadata_schedule_394_0_e3717 - params.p138);
        let noise_metadata_schedule_394_0_e3720: f64 = (1.0 + noise_metadata_schedule_394_0_e3719);
        let noise_metadata_schedule_394_0_e3721: f64 = (w[281] * noise_metadata_schedule_394_0_e3720);
        (noise_metadata_schedule_394_0_e3721,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_394_0_e3723;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_395_0_e3727: f64 = (w[282] - 1.0);
            let noise_metadata_schedule_395_0_e3728: f64 = (w[38] * noise_metadata_schedule_395_0_e3727);
            w[156] = noise_metadata_schedule_395_0_e3728;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_396_0_e3731: f64 = (w[239] * w[8]);
            let noise_metadata_schedule_396_0_e3733: f64 = (noise_metadata_schedule_396_0_e3731 / params.p22);
            let noise_metadata_schedule_396_0_e3735: f64 = if noise_metadata_schedule_396_0_e3733 < params.p138 { 1.0 } else { 0.0 };
            w[512] = noise_metadata_schedule_396_0_e3735;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_397_0_e3744,) = {
    if (w[512] != 0.0) {
        let noise_metadata_schedule_397_0_e3739: f64 = (w[239] * w[8]);
        let noise_metadata_schedule_397_0_e3741: f64 = (noise_metadata_schedule_397_0_e3739 / params.p22);
        let noise_metadata_schedule_397_0_e3742: f64 = (noise_metadata_schedule_397_0_e3741).exp();
        (noise_metadata_schedule_397_0_e3742,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_397_0_e3744;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_398_0_e3750,) = {
    if (w[512] == 0.0) {
        let noise_metadata_schedule_398_0_e3748: f64 = (params.p138).exp();
        (noise_metadata_schedule_398_0_e3748,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_398_0_e3750;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_399_0_e3765,) = {
    if (w[512] == 0.0) {
        let noise_metadata_schedule_399_0_e3757: f64 = (w[239] * w[8]);
        let noise_metadata_schedule_399_0_e3759: f64 = (noise_metadata_schedule_399_0_e3757 / params.p22);
        let noise_metadata_schedule_399_0_e3761: f64 = (noise_metadata_schedule_399_0_e3759 - params.p138);
        let noise_metadata_schedule_399_0_e3762: f64 = (1.0 + noise_metadata_schedule_399_0_e3761);
        let noise_metadata_schedule_399_0_e3763: f64 = (w[281] * noise_metadata_schedule_399_0_e3762);
        (noise_metadata_schedule_399_0_e3763,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_399_0_e3765;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_400_0_e3769: f64 = (w[282] - 1.0);
            let noise_metadata_schedule_400_0_e3770: f64 = (w[46] * noise_metadata_schedule_400_0_e3769);
            w[158] = noise_metadata_schedule_400_0_e3770;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_401_0_e3773: f64 = (w[241] * w[8]);
            let noise_metadata_schedule_401_0_e3775: f64 = (noise_metadata_schedule_401_0_e3773 / params.p31);
            let noise_metadata_schedule_401_0_e3777: f64 = if noise_metadata_schedule_401_0_e3775 < params.p138 { 1.0 } else { 0.0 };
            w[513] = noise_metadata_schedule_401_0_e3777;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_402_0_e3786,) = {
    if (w[513] != 0.0) {
        let noise_metadata_schedule_402_0_e3781: f64 = (w[241] * w[8]);
        let noise_metadata_schedule_402_0_e3783: f64 = (noise_metadata_schedule_402_0_e3781 / params.p31);
        let noise_metadata_schedule_402_0_e3784: f64 = (noise_metadata_schedule_402_0_e3783).exp();
        (noise_metadata_schedule_402_0_e3784,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_402_0_e3786;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_403_0_e3792,) = {
    if (w[513] == 0.0) {
        let noise_metadata_schedule_403_0_e3790: f64 = (params.p138).exp();
        (noise_metadata_schedule_403_0_e3790,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_403_0_e3792;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_404_0_e3807,) = {
    if (w[513] == 0.0) {
        let noise_metadata_schedule_404_0_e3799: f64 = (w[241] * w[8]);
        let noise_metadata_schedule_404_0_e3801: f64 = (noise_metadata_schedule_404_0_e3799 / params.p31);
        let noise_metadata_schedule_404_0_e3803: f64 = (noise_metadata_schedule_404_0_e3801 - params.p138);
        let noise_metadata_schedule_404_0_e3804: f64 = (1.0 + noise_metadata_schedule_404_0_e3803);
        let noise_metadata_schedule_404_0_e3805: f64 = (w[281] * noise_metadata_schedule_404_0_e3804);
        (noise_metadata_schedule_404_0_e3805,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_404_0_e3807;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_405_0_e3811: f64 = (w[282] - 1.0);
            let noise_metadata_schedule_405_0_e3812: f64 = (w[39] * noise_metadata_schedule_405_0_e3811);
            w[157] = noise_metadata_schedule_405_0_e3812;
        }
        if (active[0] & 0x18186) != 0 {
            let noise_metadata_schedule_406_0_e3815: f64 = (w[239] * w[8]);
            let noise_metadata_schedule_406_0_e3817: f64 = (noise_metadata_schedule_406_0_e3815 / params.p137);
            let noise_metadata_schedule_406_0_e3819: f64 = if noise_metadata_schedule_406_0_e3817 < params.p138 { 1.0 } else { 0.0 };
            w[514] = noise_metadata_schedule_406_0_e3819;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_407_0_e3828,) = {
    if (w[514] != 0.0) {
        let noise_metadata_schedule_407_0_e3823: f64 = (w[239] * w[8]);
        let noise_metadata_schedule_407_0_e3825: f64 = (noise_metadata_schedule_407_0_e3823 / params.p137);
        let noise_metadata_schedule_407_0_e3826: f64 = (noise_metadata_schedule_407_0_e3825).exp();
        (noise_metadata_schedule_407_0_e3826,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_407_0_e3828;
        }
        if (active[0] & 0x18186) != 0 {
            let (noise_metadata_schedule_408_0_e3834,) = {
    if (w[514] == 0.0) {
        let noise_metadata_schedule_408_0_e3832: f64 = (params.p138).exp();
        (noise_metadata_schedule_408_0_e3832,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_408_0_e3834;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_409_0_e3849,) = {
    if (w[514] == 0.0) {
        let noise_metadata_schedule_409_0_e3841: f64 = (w[239] * w[8]);
        let noise_metadata_schedule_409_0_e3843: f64 = (noise_metadata_schedule_409_0_e3841 / params.p137);
        let noise_metadata_schedule_409_0_e3845: f64 = (noise_metadata_schedule_409_0_e3843 - params.p138);
        let noise_metadata_schedule_409_0_e3846: f64 = (1.0 + noise_metadata_schedule_409_0_e3845);
        let noise_metadata_schedule_409_0_e3847: f64 = (w[281] * noise_metadata_schedule_409_0_e3846);
        (noise_metadata_schedule_409_0_e3847,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_409_0_e3849;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_410_0_e3853: f64 = (w[282] - 1.0);
            let noise_metadata_schedule_410_0_e3854: f64 = (w[47] * noise_metadata_schedule_410_0_e3853);
            w[159] = noise_metadata_schedule_410_0_e3854;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_411_0_e3865: f64 = if (((params.p33 > 0.0) && (params.p34 > 0.0)) && (w[238] < 0.0)) { 1.0 } else { 0.0 };
            w[515] = noise_metadata_schedule_411_0_e3865;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_412_0_e3871: f64 = (2.0 * w[59]);
            let noise_metadata_schedule_412_0_e3872: f64 = (w[62] / noise_metadata_schedule_412_0_e3871);
            let noise_metadata_schedule_412_0_e3873: f64 = (1.0 - noise_metadata_schedule_412_0_e3872);
            let noise_metadata_schedule_412_0_e3874: f64 = (w[61] * noise_metadata_schedule_412_0_e3873);
            let noise_metadata_schedule_412_0_e3876: f64 = if noise_metadata_schedule_412_0_e3874 < params.p138 { 1.0 } else { 0.0 };
            w[516] = noise_metadata_schedule_412_0_e3876;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_413_0_e3891,) = {
    if ((w[515] != 0.0) && (w[516] != 0.0)) {
        let noise_metadata_schedule_413_0_e3885: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_413_0_e3886: f64 = (w[62] / noise_metadata_schedule_413_0_e3885);
        let noise_metadata_schedule_413_0_e3887: f64 = (1.0 - noise_metadata_schedule_413_0_e3886);
        let noise_metadata_schedule_413_0_e3888: f64 = (w[61] * noise_metadata_schedule_413_0_e3887);
        let noise_metadata_schedule_413_0_e3889: f64 = (noise_metadata_schedule_413_0_e3888).exp();
        (noise_metadata_schedule_413_0_e3889,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_413_0_e3891;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_414_0_e3899,) = {
    if ((w[515] != 0.0) && (w[516] == 0.0)) {
        let noise_metadata_schedule_414_0_e3897: f64 = (params.p138).exp();
        (noise_metadata_schedule_414_0_e3897,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_414_0_e3899;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_415_0_e3920,) = {
    if ((w[515] != 0.0) && (w[516] == 0.0)) {
        let noise_metadata_schedule_415_0_e3911: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_415_0_e3912: f64 = (w[62] / noise_metadata_schedule_415_0_e3911);
        let noise_metadata_schedule_415_0_e3913: f64 = (1.0 - noise_metadata_schedule_415_0_e3912);
        let noise_metadata_schedule_415_0_e3914: f64 = (w[61] * noise_metadata_schedule_415_0_e3913);
        let noise_metadata_schedule_415_0_e3916: f64 = (noise_metadata_schedule_415_0_e3914 - params.p138);
        let noise_metadata_schedule_415_0_e3917: f64 = (1.0 + noise_metadata_schedule_415_0_e3916);
        let noise_metadata_schedule_415_0_e3918: f64 = (w[281] * noise_metadata_schedule_415_0_e3917);
        (noise_metadata_schedule_415_0_e3918,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_415_0_e3920;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_416_0_e3926,) = {
    if (w[515] != 0.0) {
        let noise_metadata_schedule_416_0_e3924: f64 = (w[238] * w[65]);
        (noise_metadata_schedule_416_0_e3924,)
    } else {
        (w[261],)
    }
};
            w[261] = noise_metadata_schedule_416_0_e3926;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_417_0_e3970,) = {
    if (w[515] != 0.0) {
        let noise_metadata_schedule_417_0_e3930: f64 = (w[261] * w[261]);
        let noise_metadata_schedule_417_0_e3932: f64 = (noise_metadata_schedule_417_0_e3930 + 1e-30);
        let noise_metadata_schedule_417_0_e3933: f64 = (noise_metadata_schedule_417_0_e3932).sqrt();
        let noise_metadata_schedule_417_0_e3935: f64 = (-2.0);
        let noise_metadata_schedule_417_0_e3937: f64 = (noise_metadata_schedule_417_0_e3935 - params.p66);
        let noise_metadata_schedule_417_0_e3938: f64 = (noise_metadata_schedule_417_0_e3933).powf(noise_metadata_schedule_417_0_e3937);
        let noise_metadata_schedule_417_0_e3943: f64 = (params.p66 * params.p66);
        let noise_metadata_schedule_417_0_e3944: f64 = (1.0 - noise_metadata_schedule_417_0_e3943);
        let noise_metadata_schedule_417_0_e3947: f64 = (3.0 * w[261]);
        let noise_metadata_schedule_417_0_e3950: f64 = (params.p66 - 1.0);
        let noise_metadata_schedule_417_0_e3951: f64 = (noise_metadata_schedule_417_0_e3947 * noise_metadata_schedule_417_0_e3950);
        let noise_metadata_schedule_417_0_e3952: f64 = (noise_metadata_schedule_417_0_e3944 - noise_metadata_schedule_417_0_e3951);
        let noise_metadata_schedule_417_0_e3953: f64 = (params.p66 * noise_metadata_schedule_417_0_e3952);
        let noise_metadata_schedule_417_0_e3956: f64 = (6.0 * w[261]);
        let noise_metadata_schedule_417_0_e3958: f64 = (noise_metadata_schedule_417_0_e3956 * w[261]);
        let noise_metadata_schedule_417_0_e3961: f64 = (params.p66 - 1.0);
        let noise_metadata_schedule_417_0_e3963: f64 = (noise_metadata_schedule_417_0_e3961 + w[261]);
        let noise_metadata_schedule_417_0_e3964: f64 = (noise_metadata_schedule_417_0_e3958 * noise_metadata_schedule_417_0_e3963);
        let noise_metadata_schedule_417_0_e3965: f64 = (noise_metadata_schedule_417_0_e3953 - noise_metadata_schedule_417_0_e3964);
        let noise_metadata_schedule_417_0_e3966: f64 = (noise_metadata_schedule_417_0_e3938 * noise_metadata_schedule_417_0_e3965);
        let noise_metadata_schedule_417_0_e3968: f64 = (noise_metadata_schedule_417_0_e3966 * 0.16666666666666666);
        (noise_metadata_schedule_417_0_e3968,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_417_0_e3970;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_418_0_e3982,) = {
    if (w[515] != 0.0) {
        let noise_metadata_schedule_418_0_e3974: f64 = (w[238] * w[62]);
        let noise_metadata_schedule_418_0_e3976: f64 = (noise_metadata_schedule_418_0_e3974 * w[61]);
        let noise_metadata_schedule_418_0_e3979: f64 = (w[70] * w[60]);
        let noise_metadata_schedule_418_0_e3980: f64 = (noise_metadata_schedule_418_0_e3976 / noise_metadata_schedule_418_0_e3979);
        (noise_metadata_schedule_418_0_e3980,)
    } else {
        (w[261],)
    }
};
            w[261] = noise_metadata_schedule_418_0_e3982;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_419_0_e3985: f64 = (-0.001);
            let noise_metadata_schedule_419_0_e3986: f64 = if w[261] < noise_metadata_schedule_419_0_e3985 { 1.0 } else { 0.0 };
            w[517] = noise_metadata_schedule_419_0_e3986;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_420_0_e3989: f64 = if w[261] < params.p138 { 1.0 } else { 0.0 };
            w[518] = noise_metadata_schedule_420_0_e3989;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_421_0_e3998,) = {
    if (((w[515] != 0.0) && (w[517] != 0.0)) && (w[518] != 0.0)) {
        let noise_metadata_schedule_421_0_e3996: f64 = (w[261]).exp();
        (noise_metadata_schedule_421_0_e3996,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_421_0_e3998;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_422_0_e4008,) = {
    if (((w[515] != 0.0) && (w[517] != 0.0)) && (w[518] == 0.0)) {
        let noise_metadata_schedule_422_0_e4006: f64 = (params.p138).exp();
        (noise_metadata_schedule_422_0_e4006,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_422_0_e4008;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_423_0_e4023,) = {
    if (((w[515] != 0.0) && (w[517] != 0.0)) && (w[518] == 0.0)) {
        let noise_metadata_schedule_423_0_e4019: f64 = (w[261] - params.p138);
        let noise_metadata_schedule_423_0_e4020: f64 = (1.0 + noise_metadata_schedule_423_0_e4019);
        let noise_metadata_schedule_423_0_e4021: f64 = (w[281] * noise_metadata_schedule_423_0_e4020);
        (noise_metadata_schedule_423_0_e4021,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_423_0_e4023;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_424_0_e4038,) = {
    if ((w[515] != 0.0) && (w[517] != 0.0)) {
        let noise_metadata_schedule_424_0_e4028: f64 = (-w[238]);
        let noise_metadata_schedule_424_0_e4032: f64 = (1.0 - w[91]);
        let noise_metadata_schedule_424_0_e4034: f64 = (noise_metadata_schedule_424_0_e4032 / w[261]);
        let noise_metadata_schedule_424_0_e4035: f64 = (1.0 + noise_metadata_schedule_424_0_e4034);
        let noise_metadata_schedule_424_0_e4036: f64 = (noise_metadata_schedule_424_0_e4028 * noise_metadata_schedule_424_0_e4035);
        (noise_metadata_schedule_424_0_e4036,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_424_0_e4038;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_425_0_e4061,) = {
    if ((w[515] != 0.0) && (w[517] == 0.0)) {
        let noise_metadata_schedule_425_0_e4045: f64 = (w[238] * 0.5);
        let noise_metadata_schedule_425_0_e4047: f64 = (noise_metadata_schedule_425_0_e4045 * w[261]);
        let noise_metadata_schedule_425_0_e4051: f64 = (w[261] * 0.3333333333333333);
        let noise_metadata_schedule_425_0_e4055: f64 = (0.25 * w[261]);
        let noise_metadata_schedule_425_0_e4056: f64 = (1.0 + noise_metadata_schedule_425_0_e4055);
        let noise_metadata_schedule_425_0_e4057: f64 = (noise_metadata_schedule_425_0_e4051 * noise_metadata_schedule_425_0_e4056);
        let noise_metadata_schedule_425_0_e4058: f64 = (1.0 + noise_metadata_schedule_425_0_e4057);
        let noise_metadata_schedule_425_0_e4059: f64 = (noise_metadata_schedule_425_0_e4047 * noise_metadata_schedule_425_0_e4058);
        (noise_metadata_schedule_425_0_e4059,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_425_0_e4061;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_426_0_e4077,) = {
    if (w[515] != 0.0) {
        let noise_metadata_schedule_426_0_e4065: f64 = (2.0 * w[58]);
        let noise_metadata_schedule_426_0_e4067: f64 = (noise_metadata_schedule_426_0_e4065 * w[69]);
        let noise_metadata_schedule_426_0_e4069: f64 = (noise_metadata_schedule_426_0_e4067 * w[59]);
        let noise_metadata_schedule_426_0_e4071: f64 = (noise_metadata_schedule_426_0_e4069 * w[68]);
        let noise_metadata_schedule_426_0_e4073: f64 = (noise_metadata_schedule_426_0_e4071 * w[65]);
        let noise_metadata_schedule_426_0_e4075: f64 = (noise_metadata_schedule_426_0_e4073 * w[63]);
        (noise_metadata_schedule_426_0_e4075,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_426_0_e4077;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_428_0_e4087,) = {
    if (w[515] == 0.0) {
        (0.0,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_428_0_e4087;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_429_0_e4098: f64 = if (((params.p35 > 0.0) && (params.p36 > 0.0)) && (w[236] < 0.0)) { 1.0 } else { 0.0 };
            w[519] = noise_metadata_schedule_429_0_e4098;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_430_0_e4110,) = {
    if (w[519] != 0.0) {
        let noise_metadata_schedule_430_0_e4103: f64 = (w[236] * w[67]);
        let noise_metadata_schedule_430_0_e4104: f64 = (1.0 - noise_metadata_schedule_430_0_e4103);
        let noise_metadata_schedule_430_0_e4107: f64 = (1.0 - w[76]);
        let noise_metadata_schedule_430_0_e4108: f64 = (noise_metadata_schedule_430_0_e4104).powf(noise_metadata_schedule_430_0_e4107);
        (noise_metadata_schedule_430_0_e4108,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_430_0_e4110;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_431_0_e4116: f64 = (2.0 * w[77]);
            let noise_metadata_schedule_431_0_e4117: f64 = (w[79] / noise_metadata_schedule_431_0_e4116);
            let noise_metadata_schedule_431_0_e4118: f64 = (1.0 - noise_metadata_schedule_431_0_e4117);
            let noise_metadata_schedule_431_0_e4119: f64 = (w[83] * noise_metadata_schedule_431_0_e4118);
            let noise_metadata_schedule_431_0_e4121: f64 = if noise_metadata_schedule_431_0_e4119 < params.p138 { 1.0 } else { 0.0 };
            w[520] = noise_metadata_schedule_431_0_e4121;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_432_0_e4136,) = {
    if ((w[519] != 0.0) && (w[520] != 0.0)) {
        let noise_metadata_schedule_432_0_e4130: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_432_0_e4131: f64 = (w[79] / noise_metadata_schedule_432_0_e4130);
        let noise_metadata_schedule_432_0_e4132: f64 = (1.0 - noise_metadata_schedule_432_0_e4131);
        let noise_metadata_schedule_432_0_e4133: f64 = (w[83] * noise_metadata_schedule_432_0_e4132);
        let noise_metadata_schedule_432_0_e4134: f64 = (noise_metadata_schedule_432_0_e4133).exp();
        (noise_metadata_schedule_432_0_e4134,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_432_0_e4136;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_433_0_e4144,) = {
    if ((w[519] != 0.0) && (w[520] == 0.0)) {
        let noise_metadata_schedule_433_0_e4142: f64 = (params.p138).exp();
        (noise_metadata_schedule_433_0_e4142,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_433_0_e4144;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_434_0_e4165,) = {
    if ((w[519] != 0.0) && (w[520] == 0.0)) {
        let noise_metadata_schedule_434_0_e4156: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_434_0_e4157: f64 = (w[79] / noise_metadata_schedule_434_0_e4156);
        let noise_metadata_schedule_434_0_e4158: f64 = (1.0 - noise_metadata_schedule_434_0_e4157);
        let noise_metadata_schedule_434_0_e4159: f64 = (w[83] * noise_metadata_schedule_434_0_e4158);
        let noise_metadata_schedule_434_0_e4161: f64 = (noise_metadata_schedule_434_0_e4159 - params.p138);
        let noise_metadata_schedule_434_0_e4162: f64 = (1.0 + noise_metadata_schedule_434_0_e4161);
        let noise_metadata_schedule_434_0_e4163: f64 = (w[281] * noise_metadata_schedule_434_0_e4162);
        (noise_metadata_schedule_434_0_e4163,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_434_0_e4165;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_435_0_e4171,) = {
    if (w[519] != 0.0) {
        let noise_metadata_schedule_435_0_e4169: f64 = (w[236] * w[67]);
        (noise_metadata_schedule_435_0_e4169,)
    } else {
        (w[263],)
    }
};
            w[263] = noise_metadata_schedule_435_0_e4171;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_436_0_e4215,) = {
    if (w[519] != 0.0) {
        let noise_metadata_schedule_436_0_e4175: f64 = (w[263] * w[263]);
        let noise_metadata_schedule_436_0_e4177: f64 = (noise_metadata_schedule_436_0_e4175 + 1e-30);
        let noise_metadata_schedule_436_0_e4178: f64 = (noise_metadata_schedule_436_0_e4177).sqrt();
        let noise_metadata_schedule_436_0_e4180: f64 = (-2.0);
        let noise_metadata_schedule_436_0_e4182: f64 = (noise_metadata_schedule_436_0_e4180 - w[76]);
        let noise_metadata_schedule_436_0_e4183: f64 = (noise_metadata_schedule_436_0_e4178).powf(noise_metadata_schedule_436_0_e4182);
        let noise_metadata_schedule_436_0_e4188: f64 = (w[76] * w[76]);
        let noise_metadata_schedule_436_0_e4189: f64 = (1.0 - noise_metadata_schedule_436_0_e4188);
        let noise_metadata_schedule_436_0_e4192: f64 = (3.0 * w[263]);
        let noise_metadata_schedule_436_0_e4195: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_436_0_e4196: f64 = (noise_metadata_schedule_436_0_e4192 * noise_metadata_schedule_436_0_e4195);
        let noise_metadata_schedule_436_0_e4197: f64 = (noise_metadata_schedule_436_0_e4189 - noise_metadata_schedule_436_0_e4196);
        let noise_metadata_schedule_436_0_e4198: f64 = (w[76] * noise_metadata_schedule_436_0_e4197);
        let noise_metadata_schedule_436_0_e4201: f64 = (6.0 * w[263]);
        let noise_metadata_schedule_436_0_e4203: f64 = (noise_metadata_schedule_436_0_e4201 * w[263]);
        let noise_metadata_schedule_436_0_e4206: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_436_0_e4208: f64 = (noise_metadata_schedule_436_0_e4206 + w[263]);
        let noise_metadata_schedule_436_0_e4209: f64 = (noise_metadata_schedule_436_0_e4203 * noise_metadata_schedule_436_0_e4208);
        let noise_metadata_schedule_436_0_e4210: f64 = (noise_metadata_schedule_436_0_e4198 - noise_metadata_schedule_436_0_e4209);
        let noise_metadata_schedule_436_0_e4211: f64 = (noise_metadata_schedule_436_0_e4183 * noise_metadata_schedule_436_0_e4210);
        let noise_metadata_schedule_436_0_e4213: f64 = (noise_metadata_schedule_436_0_e4211 * 0.16666666666666666);
        (noise_metadata_schedule_436_0_e4213,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_436_0_e4215;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_437_0_e4227,) = {
    if (w[519] != 0.0) {
        let noise_metadata_schedule_437_0_e4219: f64 = (w[236] * w[79]);
        let noise_metadata_schedule_437_0_e4221: f64 = (noise_metadata_schedule_437_0_e4219 * w[83]);
        let noise_metadata_schedule_437_0_e4224: f64 = (w[85] * w[80]);
        let noise_metadata_schedule_437_0_e4225: f64 = (noise_metadata_schedule_437_0_e4221 / noise_metadata_schedule_437_0_e4224);
        (noise_metadata_schedule_437_0_e4225,)
    } else {
        (w[263],)
    }
};
            w[263] = noise_metadata_schedule_437_0_e4227;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_438_0_e4230: f64 = (-0.001);
            let noise_metadata_schedule_438_0_e4231: f64 = if w[263] < noise_metadata_schedule_438_0_e4230 { 1.0 } else { 0.0 };
            w[521] = noise_metadata_schedule_438_0_e4231;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_439_0_e4234: f64 = if w[263] < params.p138 { 1.0 } else { 0.0 };
            w[522] = noise_metadata_schedule_439_0_e4234;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_440_0_e4243,) = {
    if (((w[519] != 0.0) && (w[521] != 0.0)) && (w[522] != 0.0)) {
        let noise_metadata_schedule_440_0_e4241: f64 = (w[263]).exp();
        (noise_metadata_schedule_440_0_e4241,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_440_0_e4243;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_441_0_e4253,) = {
    if (((w[519] != 0.0) && (w[521] != 0.0)) && (w[522] == 0.0)) {
        let noise_metadata_schedule_441_0_e4251: f64 = (params.p138).exp();
        (noise_metadata_schedule_441_0_e4251,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_441_0_e4253;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_442_0_e4268,) = {
    if (((w[519] != 0.0) && (w[521] != 0.0)) && (w[522] == 0.0)) {
        let noise_metadata_schedule_442_0_e4264: f64 = (w[263] - params.p138);
        let noise_metadata_schedule_442_0_e4265: f64 = (1.0 + noise_metadata_schedule_442_0_e4264);
        let noise_metadata_schedule_442_0_e4266: f64 = (w[281] * noise_metadata_schedule_442_0_e4265);
        (noise_metadata_schedule_442_0_e4266,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_442_0_e4268;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_443_0_e4283,) = {
    if ((w[519] != 0.0) && (w[521] != 0.0)) {
        let noise_metadata_schedule_443_0_e4273: f64 = (-w[236]);
        let noise_metadata_schedule_443_0_e4277: f64 = (1.0 - w[92]);
        let noise_metadata_schedule_443_0_e4279: f64 = (noise_metadata_schedule_443_0_e4277 / w[263]);
        let noise_metadata_schedule_443_0_e4280: f64 = (1.0 + noise_metadata_schedule_443_0_e4279);
        let noise_metadata_schedule_443_0_e4281: f64 = (noise_metadata_schedule_443_0_e4273 * noise_metadata_schedule_443_0_e4280);
        (noise_metadata_schedule_443_0_e4281,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_443_0_e4283;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_444_0_e4306,) = {
    if ((w[519] != 0.0) && (w[521] == 0.0)) {
        let noise_metadata_schedule_444_0_e4290: f64 = (w[236] * 0.5);
        let noise_metadata_schedule_444_0_e4292: f64 = (noise_metadata_schedule_444_0_e4290 * w[263]);
        let noise_metadata_schedule_444_0_e4296: f64 = (w[263] * 0.3333333333333333);
        let noise_metadata_schedule_444_0_e4300: f64 = (0.25 * w[263]);
        let noise_metadata_schedule_444_0_e4301: f64 = (1.0 + noise_metadata_schedule_444_0_e4300);
        let noise_metadata_schedule_444_0_e4302: f64 = (noise_metadata_schedule_444_0_e4296 * noise_metadata_schedule_444_0_e4301);
        let noise_metadata_schedule_444_0_e4303: f64 = (1.0 + noise_metadata_schedule_444_0_e4302);
        let noise_metadata_schedule_444_0_e4304: f64 = (noise_metadata_schedule_444_0_e4292 * noise_metadata_schedule_444_0_e4303);
        (noise_metadata_schedule_444_0_e4304,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_444_0_e4306;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_445_0_e4322,) = {
    if (w[519] != 0.0) {
        let noise_metadata_schedule_445_0_e4310: f64 = (2.0 * w[84]);
        let noise_metadata_schedule_445_0_e4312: f64 = (noise_metadata_schedule_445_0_e4310 * w[81]);
        let noise_metadata_schedule_445_0_e4314: f64 = (noise_metadata_schedule_445_0_e4312 * w[77]);
        let noise_metadata_schedule_445_0_e4316: f64 = (noise_metadata_schedule_445_0_e4314 * w[78]);
        let noise_metadata_schedule_445_0_e4318: f64 = (noise_metadata_schedule_445_0_e4316 * w[67]);
        let noise_metadata_schedule_445_0_e4320: f64 = (noise_metadata_schedule_445_0_e4318 * w[89]);
        (noise_metadata_schedule_445_0_e4320,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_445_0_e4322;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_447_0_e4332,) = {
    if (w[519] == 0.0) {
        (0.0,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_447_0_e4332;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_452_0_e4359: f64 = (2.0 * w[43]);
            let noise_metadata_schedule_452_0_e4362: f64 = (w[254] - 1.0);
            let noise_metadata_schedule_452_0_e4363: f64 = (noise_metadata_schedule_452_0_e4359 * noise_metadata_schedule_452_0_e4362);
            let noise_metadata_schedule_452_0_e4368: f64 = (4.0 * w[43]);
            let noise_metadata_schedule_452_0_e4370: f64 = (noise_metadata_schedule_452_0_e4368 / w[37]);
            let noise_metadata_schedule_452_0_e4372: f64 = (noise_metadata_schedule_452_0_e4370 * w[254]);
            let noise_metadata_schedule_452_0_e4373: f64 = (1.0 + noise_metadata_schedule_452_0_e4372);
            let noise_metadata_schedule_452_0_e4374: f64 = (noise_metadata_schedule_452_0_e4373).sqrt();
            let noise_metadata_schedule_452_0_e4375: f64 = (1.0 + noise_metadata_schedule_452_0_e4374);
            let noise_metadata_schedule_452_0_e4376: f64 = (noise_metadata_schedule_452_0_e4363 / noise_metadata_schedule_452_0_e4375);
            w[160] = noise_metadata_schedule_452_0_e4376;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_453_0_e4383: f64 = if ((params.p5 > 0.0) && (params.p32 > 0.0)) { 1.0 } else { 0.0 };
            w[523] = noise_metadata_schedule_453_0_e4383;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_454_0_e4389,) = {
    if (w[523] != 0.0) {
        let noise_metadata_schedule_454_0_e4387: f64 = (w[160] * w[153]);
        (noise_metadata_schedule_454_0_e4387,)
    } else {
        (w[160],)
    }
};
            w[160] = noise_metadata_schedule_454_0_e4389;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_455_0_e4414,) = {
    if (w[523] != 0.0) {
        let noise_metadata_schedule_455_0_e4393: f64 = (params.p32 * 2.0);
        let noise_metadata_schedule_455_0_e4395: f64 = (noise_metadata_schedule_455_0_e4393 * w[43]);
        let noise_metadata_schedule_455_0_e4398: f64 = (w[255] - 1.0);
        let noise_metadata_schedule_455_0_e4399: f64 = (noise_metadata_schedule_455_0_e4395 * noise_metadata_schedule_455_0_e4398);
        let noise_metadata_schedule_455_0_e4404: f64 = (4.0 * w[43]);
        let noise_metadata_schedule_455_0_e4406: f64 = (noise_metadata_schedule_455_0_e4404 / w[37]);
        let noise_metadata_schedule_455_0_e4408: f64 = (noise_metadata_schedule_455_0_e4406 * w[255]);
        let noise_metadata_schedule_455_0_e4409: f64 = (1.0 + noise_metadata_schedule_455_0_e4408);
        let noise_metadata_schedule_455_0_e4410: f64 = (noise_metadata_schedule_455_0_e4409).sqrt();
        let noise_metadata_schedule_455_0_e4411: f64 = (1.0 + noise_metadata_schedule_455_0_e4410);
        let noise_metadata_schedule_455_0_e4412: f64 = (noise_metadata_schedule_455_0_e4399 / noise_metadata_schedule_455_0_e4411);
        (noise_metadata_schedule_455_0_e4412,)
    } else {
        (w[167],)
    }
};
            w[167] = noise_metadata_schedule_455_0_e4414;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_456_0_e4418,) = {
    if (w[523] != 0.0) {
        (0.0,)
    } else {
        (w[168],)
    }
};
            w[168] = noise_metadata_schedule_456_0_e4418;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_457_0_e4421: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            w[524] = noise_metadata_schedule_457_0_e4421;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_458_0_e4431,) = {
    if ((w[523] != 0.0) && (w[524] != 0.0)) {
        let noise_metadata_schedule_458_0_e4427: f64 = (params.p32 * w[43]);
        let noise_metadata_schedule_458_0_e4429: f64 = (noise_metadata_schedule_458_0_e4427 * w[32]);
        (noise_metadata_schedule_458_0_e4429,)
    } else {
        (w[277],)
    }
};
            w[277] = noise_metadata_schedule_458_0_e4431;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_459_0_e4444,) = {
    if ((w[523] != 0.0) && (w[524] != 0.0)) {
        let noise_metadata_schedule_459_0_e4439: f64 = (w[277] * w[8]);
        let noise_metadata_schedule_459_0_e4440: f64 = (noise_metadata_schedule_459_0_e4439).ln();
        let noise_metadata_schedule_459_0_e4441: f64 = (2.0 - noise_metadata_schedule_459_0_e4440);
        let noise_metadata_schedule_459_0_e4442: f64 = (w[6] * noise_metadata_schedule_459_0_e4441);
        (noise_metadata_schedule_459_0_e4442,)
    } else {
        (w[169],)
    }
};
            w[169] = noise_metadata_schedule_459_0_e4444;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_8(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_460_0_e4452,) = {
    if ((w[523] != 0.0) && (w[524] != 0.0)) {
        let noise_metadata_schedule_460_0_e4450: f64 = (w[247] - w[169]);
        (noise_metadata_schedule_460_0_e4450,)
    } else {
        (w[270],)
    }
};
            w[270] = noise_metadata_schedule_460_0_e4452;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_461_0_e4460,) = {
    if ((w[523] != 0.0) && (w[524] != 0.0)) {
        let noise_metadata_schedule_461_0_e4458: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_461_0_e4458,)
    } else {
        (w[267],)
    }
};
            w[267] = noise_metadata_schedule_461_0_e4460;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_462_0_e4468,) = {
    if ((w[523] != 0.0) && (w[524] != 0.0)) {
        let noise_metadata_schedule_462_0_e4466: f64 = (w[270] * w[270]);
        (noise_metadata_schedule_462_0_e4466,)
    } else {
        (w[268],)
    }
};
            w[268] = noise_metadata_schedule_462_0_e4468;
        }
        if (active[0] & 0x6000) != 0 {
            let noise_metadata_schedule_463_0_e4471: f64 = if w[270] < 0.0 { 1.0 } else { 0.0 };
            w[525] = noise_metadata_schedule_463_0_e4471;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_464_0_e4488,) = {
    if (((w[523] != 0.0) && (w[524] != 0.0)) && (w[525] != 0.0)) {
        let noise_metadata_schedule_464_0_e4479: f64 = (0.5 * w[267]);
        let noise_metadata_schedule_464_0_e4482: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_464_0_e4483: f64 = (noise_metadata_schedule_464_0_e4482).sqrt();
        let noise_metadata_schedule_464_0_e4485: f64 = (noise_metadata_schedule_464_0_e4483 - w[270]);
        let noise_metadata_schedule_464_0_e4486: f64 = (noise_metadata_schedule_464_0_e4479 / noise_metadata_schedule_464_0_e4485);
        (noise_metadata_schedule_464_0_e4486,)
    } else {
        (w[170],)
    }
};
            w[170] = noise_metadata_schedule_464_0_e4488;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_465_0_e4504,) = {
    if (((w[523] != 0.0) && (w[524] != 0.0)) && (w[525] == 0.0)) {
        let noise_metadata_schedule_465_0_e4498: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_465_0_e4499: f64 = (noise_metadata_schedule_465_0_e4498).sqrt();
        let noise_metadata_schedule_465_0_e4501: f64 = (noise_metadata_schedule_465_0_e4499 + w[270]);
        let noise_metadata_schedule_465_0_e4502: f64 = (0.5 * noise_metadata_schedule_465_0_e4501);
        (noise_metadata_schedule_465_0_e4502,)
    } else {
        (w[170],)
    }
};
            w[170] = noise_metadata_schedule_465_0_e4504;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_466_0_e4520,) = {
    if ((w[523] != 0.0) && (w[524] != 0.0)) {
        let noise_metadata_schedule_466_0_e4512: f64 = (w[167] + w[168]);
        let noise_metadata_schedule_466_0_e4514: f64 = (noise_metadata_schedule_466_0_e4512 * w[32]);
        let noise_metadata_schedule_466_0_e4515: f64 = (w[277] + noise_metadata_schedule_466_0_e4514);
        let noise_metadata_schedule_466_0_e4517: f64 = (noise_metadata_schedule_466_0_e4515 + w[170]);
        let noise_metadata_schedule_466_0_e4518: f64 = (w[170] / noise_metadata_schedule_466_0_e4517);
        (noise_metadata_schedule_466_0_e4518,)
    } else {
        (w[171],)
    }
};
            w[171] = noise_metadata_schedule_466_0_e4520;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_470_0_e4548,) = {
    if ((w[523] != 0.0) && (w[524] == 0.0)) {
        (1.0,)
    } else {
        (w[171],)
    }
};
            w[171] = noise_metadata_schedule_470_0_e4548;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_471_0_e4554,) = {
    if (w[523] != 0.0) {
        let noise_metadata_schedule_471_0_e4552: f64 = (w[171] * w[167]);
        (noise_metadata_schedule_471_0_e4552,)
    } else {
        (w[172],)
    }
};
            w[172] = noise_metadata_schedule_471_0_e4554;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_472_0_e4557: f64 = if params.p83 == 1.0 { 1.0 } else { 0.0 };
            w[526] = noise_metadata_schedule_472_0_e4557;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_473_0_e4563,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_473_0_e4561: f64 = (w[240] + w[236]);
        (noise_metadata_schedule_473_0_e4561,)
    } else {
        (w[328],)
    }
};
            w[328] = noise_metadata_schedule_473_0_e4563;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_474_0_e4569,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_474_0_e4567: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_474_0_e4567,)
    } else {
        (w[267],)
    }
};
            w[267] = noise_metadata_schedule_474_0_e4569;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_475_0_e4581,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_475_0_e4572: f64 = (-1.0);
        let noise_metadata_schedule_475_0_e4574: f64 = (noise_metadata_schedule_475_0_e4572 * w[328]);
        let noise_metadata_schedule_475_0_e4576: f64 = (-1.0);
        let noise_metadata_schedule_475_0_e4577: f64 = (noise_metadata_schedule_475_0_e4574 * noise_metadata_schedule_475_0_e4576);
        let noise_metadata_schedule_475_0_e4579: f64 = (noise_metadata_schedule_475_0_e4577 * w[328]);
        (noise_metadata_schedule_475_0_e4579,)
    } else {
        (w[268],)
    }
};
            w[268] = noise_metadata_schedule_475_0_e4581;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_476_0_e4583: f64 = (-1.0);
            let noise_metadata_schedule_476_0_e4585: f64 = (noise_metadata_schedule_476_0_e4583 * w[328]);
            let noise_metadata_schedule_476_0_e4587: f64 = if noise_metadata_schedule_476_0_e4585 < 0.0 { 1.0 } else { 0.0 };
            w[527] = noise_metadata_schedule_476_0_e4587;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_477_0_e4605,) = {
    if ((w[526] != 0.0) && (w[527] != 0.0)) {
        let noise_metadata_schedule_477_0_e4593: f64 = (0.5 * w[267]);
        let noise_metadata_schedule_477_0_e4596: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_477_0_e4597: f64 = (noise_metadata_schedule_477_0_e4596).sqrt();
        let noise_metadata_schedule_477_0_e4599: f64 = (-1.0);
        let noise_metadata_schedule_477_0_e4601: f64 = (noise_metadata_schedule_477_0_e4599 * w[328]);
        let noise_metadata_schedule_477_0_e4602: f64 = (noise_metadata_schedule_477_0_e4597 - noise_metadata_schedule_477_0_e4601);
        let noise_metadata_schedule_477_0_e4603: f64 = (noise_metadata_schedule_477_0_e4593 / noise_metadata_schedule_477_0_e4602);
        (noise_metadata_schedule_477_0_e4603,)
    } else {
        (w[329],)
    }
};
            w[329] = noise_metadata_schedule_477_0_e4605;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_478_0_e4622,) = {
    if ((w[526] != 0.0) && (w[527] == 0.0)) {
        let noise_metadata_schedule_478_0_e4613: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_478_0_e4614: f64 = (noise_metadata_schedule_478_0_e4613).sqrt();
        let noise_metadata_schedule_478_0_e4616: f64 = (-1.0);
        let noise_metadata_schedule_478_0_e4618: f64 = (noise_metadata_schedule_478_0_e4616 * w[328]);
        let noise_metadata_schedule_478_0_e4619: f64 = (noise_metadata_schedule_478_0_e4614 + noise_metadata_schedule_478_0_e4618);
        let noise_metadata_schedule_478_0_e4620: f64 = (0.5 * noise_metadata_schedule_478_0_e4619);
        (noise_metadata_schedule_478_0_e4620,)
    } else {
        (w[329],)
    }
};
            w[329] = noise_metadata_schedule_478_0_e4622;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_479_0_e4632,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_479_0_e4628: f64 = (w[324]).powf(params.p81);
        let noise_metadata_schedule_479_0_e4629: f64 = (1.0 - noise_metadata_schedule_479_0_e4628);
        let noise_metadata_schedule_479_0_e4630: f64 = (1.0 / noise_metadata_schedule_479_0_e4629);
        (noise_metadata_schedule_479_0_e4630,)
    } else {
        (w[330],)
    }
};
            w[330] = noise_metadata_schedule_479_0_e4632;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_480_0_e4638,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_480_0_e4636: f64 = (w[324] * params.p80);
        (noise_metadata_schedule_480_0_e4636,)
    } else {
        (w[325],)
    }
};
            w[325] = noise_metadata_schedule_480_0_e4638;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_481_0_e4654,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_481_0_e4642: f64 = (w[330] * w[330]);
        let noise_metadata_schedule_481_0_e4646: f64 = (params.p81 - 1.0);
        let noise_metadata_schedule_481_0_e4647: f64 = (w[324]).powf(noise_metadata_schedule_481_0_e4646);
        let noise_metadata_schedule_481_0_e4648: f64 = (noise_metadata_schedule_481_0_e4642 * noise_metadata_schedule_481_0_e4647);
        let noise_metadata_schedule_481_0_e4650: f64 = (noise_metadata_schedule_481_0_e4648 * params.p81);
        let noise_metadata_schedule_481_0_e4652: f64 = (noise_metadata_schedule_481_0_e4650 / params.p80);
        (noise_metadata_schedule_481_0_e4652,)
    } else {
        (w[327],)
    }
};
            w[327] = noise_metadata_schedule_481_0_e4654;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_482_0_e4657: f64 = if w[329] < w[325] { 1.0 } else { 0.0 };
            w[528] = noise_metadata_schedule_482_0_e4657;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_483_0_e4671,) = {
    if ((w[526] != 0.0) && (w[528] != 0.0)) {
        let noise_metadata_schedule_483_0_e4665: f64 = (w[329] / params.p80);
        let noise_metadata_schedule_483_0_e4667: f64 = (noise_metadata_schedule_483_0_e4665).powf(params.p81);
        let noise_metadata_schedule_483_0_e4668: f64 = (1.0 - noise_metadata_schedule_483_0_e4667);
        let noise_metadata_schedule_483_0_e4669: f64 = (1.0 / noise_metadata_schedule_483_0_e4668);
        (noise_metadata_schedule_483_0_e4669,)
    } else {
        (w[326],)
    }
};
            w[326] = noise_metadata_schedule_483_0_e4671;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_484_0_e4684,) = {
    if ((w[526] != 0.0) && (w[528] == 0.0)) {
        let noise_metadata_schedule_484_0_e4679: f64 = (w[329] - w[325]);
        let noise_metadata_schedule_484_0_e4681: f64 = (noise_metadata_schedule_484_0_e4679 * w[327]);
        let noise_metadata_schedule_484_0_e4682: f64 = (w[330] + noise_metadata_schedule_484_0_e4681);
        (noise_metadata_schedule_484_0_e4682,)
    } else {
        (w[326],)
    }
};
            w[326] = noise_metadata_schedule_484_0_e4684;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_485_0_e4689,) = {
    if (w[526] == 0.0) {
        (1.0,)
    } else {
        (w[326],)
    }
};
            w[326] = noise_metadata_schedule_485_0_e4689;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_486_0_e4692: f64 = (w[82] * w[326]);
            w[82] = noise_metadata_schedule_486_0_e4692;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_487_0_e4695: f64 = (w[160] * w[326]);
            w[160] = noise_metadata_schedule_487_0_e4695;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_488_0_e4698: f64 = (w[157] * w[326]);
            w[157] = noise_metadata_schedule_488_0_e4698;
        }
        if (active[0] & 0x6000) != 0 {
            let noise_metadata_schedule_489_0_e4701: f64 = (w[172] * w[326]);
            w[172] = noise_metadata_schedule_489_0_e4701;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_490_0_e4705: f64 = (w[134] / w[41]);
            let noise_metadata_schedule_490_0_e4706: f64 = (1.0 + noise_metadata_schedule_490_0_e4705);
            let noise_metadata_schedule_490_0_e4709: f64 = (w[141] / w[40]);
            let noise_metadata_schedule_490_0_e4710: f64 = (noise_metadata_schedule_490_0_e4706 + noise_metadata_schedule_490_0_e4709);
            w[175] = noise_metadata_schedule_490_0_e4710;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_491_0_e4713: f64 = (0.1 * 0.1);
            w[267] = noise_metadata_schedule_491_0_e4713;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_492_0_e4716: f64 = (w[175] * w[175]);
            w[268] = noise_metadata_schedule_492_0_e4716;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_493_0_e4719: f64 = if w[175] < 0.0 { 1.0 } else { 0.0 };
            w[529] = noise_metadata_schedule_493_0_e4719;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_494_0_e4732,) = {
    if (w[529] != 0.0) {
        let noise_metadata_schedule_494_0_e4723: f64 = (0.5 * w[267]);
        let noise_metadata_schedule_494_0_e4726: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_494_0_e4727: f64 = (noise_metadata_schedule_494_0_e4726).sqrt();
        let noise_metadata_schedule_494_0_e4729: f64 = (noise_metadata_schedule_494_0_e4727 - w[175]);
        let noise_metadata_schedule_494_0_e4730: f64 = (noise_metadata_schedule_494_0_e4723 / noise_metadata_schedule_494_0_e4729);
        (noise_metadata_schedule_494_0_e4730,)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_494_0_e4732;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_495_0_e4744,) = {
    if (w[529] == 0.0) {
        let noise_metadata_schedule_495_0_e4738: f64 = (w[268] + w[267]);
        let noise_metadata_schedule_495_0_e4739: f64 = (noise_metadata_schedule_495_0_e4738).sqrt();
        let noise_metadata_schedule_495_0_e4741: f64 = (noise_metadata_schedule_495_0_e4739 + w[175]);
        let noise_metadata_schedule_495_0_e4742: f64 = (0.5 * noise_metadata_schedule_495_0_e4741);
        (noise_metadata_schedule_495_0_e4742,)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_495_0_e4744;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_496_0_e4750: f64 = (w[145] + w[146]);
            let noise_metadata_schedule_496_0_e4751: f64 = (0.5 * noise_metadata_schedule_496_0_e4750);
            let noise_metadata_schedule_496_0_e4752: f64 = (1.0 + noise_metadata_schedule_496_0_e4751);
            let noise_metadata_schedule_496_0_e4753: f64 = (w[176] * noise_metadata_schedule_496_0_e4752);
            w[177] = noise_metadata_schedule_496_0_e4753;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_497_0_e4756: f64 = (w[29] / w[177]);
            w[179] = noise_metadata_schedule_497_0_e4756;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_498_0_e4759: f64 = if w[179] < w[322] { 1.0 } else { 0.0 };
            w[530] = noise_metadata_schedule_498_0_e4759;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_499_0_e4763,) = {
    if (w[530] != 0.0) {
        (w[322],)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_499_0_e4763;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_500_0_e4766: f64 = (3.0 * w[179]);
            w[178] = noise_metadata_schedule_500_0_e4766;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_502_0_e4780: f64 = if w[152] > 0.0 { 1.0 } else { 0.0 };
            w[531] = noise_metadata_schedule_502_0_e4780;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_503_0_e4783: f64 = if params.p38 == 1.0 { 1.0 } else { 0.0 };
            w[532] = noise_metadata_schedule_503_0_e4783;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_504_0_e4786: f64 = if w[236] < params.p43 { 1.0 } else { 0.0 };
            w[533] = noise_metadata_schedule_504_0_e4786;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_505_0_e4788: f64 = (-w[152]);
            let noise_metadata_schedule_505_0_e4790: f64 = (noise_metadata_schedule_505_0_e4788 / params.p41);
            let noise_metadata_schedule_505_0_e4792: f64 = if noise_metadata_schedule_505_0_e4790 < params.p138 { 1.0 } else { 0.0 };
            w[534] = noise_metadata_schedule_505_0_e4792;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_506_0_e4806,) = {
    if ((((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) && (w[534] != 0.0)) {
        let noise_metadata_schedule_506_0_e4801: f64 = (-w[152]);
        let noise_metadata_schedule_506_0_e4803: f64 = (noise_metadata_schedule_506_0_e4801 / params.p41);
        let noise_metadata_schedule_506_0_e4804: f64 = (noise_metadata_schedule_506_0_e4803).exp();
        (noise_metadata_schedule_506_0_e4804,)
    } else {
        (w[314],)
    }
};
            w[314] = noise_metadata_schedule_506_0_e4806;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_507_0_e4818,) = {
    if ((((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) && (w[534] == 0.0)) {
        let noise_metadata_schedule_507_0_e4816: f64 = (params.p138).exp();
        (noise_metadata_schedule_507_0_e4816,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_507_0_e4818;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_508_0_e4838,) = {
    if ((((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) && (w[534] == 0.0)) {
        let noise_metadata_schedule_508_0_e4830: f64 = (-w[152]);
        let noise_metadata_schedule_508_0_e4832: f64 = (noise_metadata_schedule_508_0_e4830 / params.p41);
        let noise_metadata_schedule_508_0_e4834: f64 = (noise_metadata_schedule_508_0_e4832 - params.p138);
        let noise_metadata_schedule_508_0_e4835: f64 = (1.0 + noise_metadata_schedule_508_0_e4834);
        let noise_metadata_schedule_508_0_e4836: f64 = (w[281] * noise_metadata_schedule_508_0_e4835);
        (noise_metadata_schedule_508_0_e4836,)
    } else {
        (w[314],)
    }
};
            w[314] = noise_metadata_schedule_508_0_e4838;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_509_0_e4850,) = {
    if (((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) {
        let noise_metadata_schedule_509_0_e4846: f64 = (params.p43 - w[236]);
        let noise_metadata_schedule_509_0_e4848: f64 = (noise_metadata_schedule_509_0_e4846 * w[314]);
        (noise_metadata_schedule_509_0_e4848,)
    } else {
        (w[315],)
    }
};
            w[315] = noise_metadata_schedule_509_0_e4850;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_510_0_e4852: f64 = (-w[316]);
            let noise_metadata_schedule_510_0_e4855: f64 = (w[315]).powf(params.p40);
            let noise_metadata_schedule_510_0_e4856: f64 = (noise_metadata_schedule_510_0_e4852 * noise_metadata_schedule_510_0_e4855);
            let noise_metadata_schedule_510_0_e4858: f64 = if noise_metadata_schedule_510_0_e4856 < params.p138 { 1.0 } else { 0.0 };
            w[535] = noise_metadata_schedule_510_0_e4858;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_511_0_e4874,) = {
    if ((((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) && (w[535] != 0.0)) {
        let noise_metadata_schedule_511_0_e4867: f64 = (-w[316]);
        let noise_metadata_schedule_511_0_e4870: f64 = (w[315]).powf(params.p40);
        let noise_metadata_schedule_511_0_e4871: f64 = (noise_metadata_schedule_511_0_e4867 * noise_metadata_schedule_511_0_e4870);
        let noise_metadata_schedule_511_0_e4872: f64 = (noise_metadata_schedule_511_0_e4871).exp();
        (noise_metadata_schedule_511_0_e4872,)
    } else {
        (w[319],)
    }
};
            w[319] = noise_metadata_schedule_511_0_e4874;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_512_0_e4886,) = {
    if ((((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) && (w[535] == 0.0)) {
        let noise_metadata_schedule_512_0_e4884: f64 = (params.p138).exp();
        (noise_metadata_schedule_512_0_e4884,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_512_0_e4886;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_513_0_e4908,) = {
    if ((((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) && (w[535] == 0.0)) {
        let noise_metadata_schedule_513_0_e4898: f64 = (-w[316]);
        let noise_metadata_schedule_513_0_e4901: f64 = (w[315]).powf(params.p40);
        let noise_metadata_schedule_513_0_e4902: f64 = (noise_metadata_schedule_513_0_e4898 * noise_metadata_schedule_513_0_e4901);
        let noise_metadata_schedule_513_0_e4904: f64 = (noise_metadata_schedule_513_0_e4902 - params.p138);
        let noise_metadata_schedule_513_0_e4905: f64 = (1.0 + noise_metadata_schedule_513_0_e4904);
        let noise_metadata_schedule_513_0_e4906: f64 = (w[281] * noise_metadata_schedule_513_0_e4905);
        (noise_metadata_schedule_513_0_e4906,)
    } else {
        (w[319],)
    }
};
            w[319] = noise_metadata_schedule_513_0_e4908;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_514_0_e4922,) = {
    if (((w[531] != 0.0) && (w[532] != 0.0)) && (w[533] != 0.0)) {
        let noise_metadata_schedule_514_0_e4916: f64 = (params.p39 / w[316]);
        let noise_metadata_schedule_514_0_e4918: f64 = (noise_metadata_schedule_514_0_e4916 * w[315]);
        let noise_metadata_schedule_514_0_e4920: f64 = (noise_metadata_schedule_514_0_e4918 * w[319]);
        (noise_metadata_schedule_514_0_e4920,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_514_0_e4922;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_515_0_e4925: f64 = if params.p38 == 2.0 { 1.0 } else { 0.0 };
            w[536] = noise_metadata_schedule_515_0_e4925;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_516_0_e4928: f64 = if w[236] < w[16] { 1.0 } else { 0.0 };
            w[537] = noise_metadata_schedule_516_0_e4928;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_517_0_e4945,) = {
    if ((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_517_0_e4939: f64 = (2.0 * params.p45);
        let noise_metadata_schedule_517_0_e4942: f64 = (params.p44 * params.p44);
        let noise_metadata_schedule_517_0_e4943: f64 = (noise_metadata_schedule_517_0_e4939 / noise_metadata_schedule_517_0_e4942);
        (noise_metadata_schedule_517_0_e4943,)
    } else {
        (w[188],)
    }
};
            w[188] = noise_metadata_schedule_517_0_e4945;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_9(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_518_0_e4960,) = {
    if ((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_518_0_e4956: f64 = (w[16] - w[236]);
        let noise_metadata_schedule_518_0_e4958: f64 = (noise_metadata_schedule_518_0_e4956 / w[202]);
        (noise_metadata_schedule_518_0_e4958,)
    } else {
        (w[266],)
    }
};
            w[266] = noise_metadata_schedule_518_0_e4960;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_519_0_e4976,) = {
    if ((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_519_0_e4971: f64 = (2.0 * w[266]);
        let noise_metadata_schedule_519_0_e4973: f64 = (noise_metadata_schedule_519_0_e4971 / w[188]);
        let noise_metadata_schedule_519_0_e4974: f64 = (noise_metadata_schedule_519_0_e4973).sqrt();
        (noise_metadata_schedule_519_0_e4974,)
    } else {
        (w[189],)
    }
};
            w[189] = noise_metadata_schedule_519_0_e4976;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_520_0_e4979: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            w[538] = noise_metadata_schedule_520_0_e4979;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_521_0_e4992,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] != 0.0)) {
        (params.p44,)
    } else {
        (w[190],)
    }
};
            w[190] = noise_metadata_schedule_521_0_e4992;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_522_0_e5010,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] == 0.0)) {
        let noise_metadata_schedule_522_0_e5007: f64 = (0.5 * w[118]);
        let noise_metadata_schedule_522_0_e5008: f64 = (1.0 - noise_metadata_schedule_522_0_e5007);
        (noise_metadata_schedule_522_0_e5008,)
    } else {
        (w[119],)
    }
};
            w[119] = noise_metadata_schedule_522_0_e5010;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_523_0_e5028,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] == 0.0)) {
        let noise_metadata_schedule_523_0_e5024: f64 = (params.p44 * w[119]);
        let noise_metadata_schedule_523_0_e5026: f64 = (noise_metadata_schedule_523_0_e5024 * w[119]);
        (noise_metadata_schedule_523_0_e5026,)
    } else {
        (w[190],)
    }
};
            w[190] = noise_metadata_schedule_523_0_e5028;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_524_0_e5050,) = {
    if ((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_524_0_e5039: f64 = (w[189] * w[190]);
        let noise_metadata_schedule_524_0_e5042: f64 = (w[189] * w[189]);
        let noise_metadata_schedule_524_0_e5045: f64 = (w[190] * w[190]);
        let noise_metadata_schedule_524_0_e5046: f64 = (noise_metadata_schedule_524_0_e5042 + noise_metadata_schedule_524_0_e5045);
        let noise_metadata_schedule_524_0_e5047: f64 = (noise_metadata_schedule_524_0_e5046).sqrt();
        let noise_metadata_schedule_524_0_e5048: f64 = (noise_metadata_schedule_524_0_e5039 / noise_metadata_schedule_524_0_e5047);
        (noise_metadata_schedule_524_0_e5048,)
    } else {
        (w[191],)
    }
};
            w[191] = noise_metadata_schedule_524_0_e5050;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_525_0_e5065,) = {
    if ((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_525_0_e5061: f64 = (w[16] - w[236]);
        let noise_metadata_schedule_525_0_e5063: f64 = (noise_metadata_schedule_525_0_e5061 / w[191]);
        (noise_metadata_schedule_525_0_e5063,)
    } else {
        (w[192],)
    }
};
            w[192] = noise_metadata_schedule_525_0_e5065;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_526_0_e5084,) = {
    if ((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_526_0_e5077: f64 = (0.5 * w[191]);
        let noise_metadata_schedule_526_0_e5079: f64 = (noise_metadata_schedule_526_0_e5077 * w[188]);
        let noise_metadata_schedule_526_0_e5081: f64 = (noise_metadata_schedule_526_0_e5079 * w[202]);
        let noise_metadata_schedule_526_0_e5082: f64 = (w[192] + noise_metadata_schedule_526_0_e5081);
        (noise_metadata_schedule_526_0_e5082,)
    } else {
        (w[193],)
    }
};
            w[193] = noise_metadata_schedule_526_0_e5084;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_527_0_e5087: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            w[539] = noise_metadata_schedule_527_0_e5087;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_528_0_e5100,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[539] != 0.0)) {
        (w[193],)
    } else {
        (w[194],)
    }
};
            w[194] = noise_metadata_schedule_528_0_e5100;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_529_0_e5124,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[539] == 0.0)) {
        let noise_metadata_schedule_529_0_e5115: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_529_0_e5119: f64 = (2.0 * w[118]);
        let noise_metadata_schedule_529_0_e5120: f64 = (1.0 + noise_metadata_schedule_529_0_e5119);
        let noise_metadata_schedule_529_0_e5121: f64 = (noise_metadata_schedule_529_0_e5115 * noise_metadata_schedule_529_0_e5120);
        let noise_metadata_schedule_529_0_e5122: f64 = (1.0 + noise_metadata_schedule_529_0_e5121);
        (noise_metadata_schedule_529_0_e5122,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_529_0_e5124;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_530_0_e5146,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[539] == 0.0)) {
        let noise_metadata_schedule_530_0_e5138: f64 = (1.0 + params.p46);
        let noise_metadata_schedule_530_0_e5142: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_530_0_e5143: f64 = (1.0 + noise_metadata_schedule_530_0_e5142);
        let noise_metadata_schedule_530_0_e5144: f64 = (noise_metadata_schedule_530_0_e5138 / noise_metadata_schedule_530_0_e5143);
        (noise_metadata_schedule_530_0_e5144,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_530_0_e5146;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_531_0_e5174,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[539] == 0.0)) {
        let noise_metadata_schedule_531_0_e5161: f64 = (0.5 * w[191]);
        let noise_metadata_schedule_531_0_e5163: f64 = (noise_metadata_schedule_531_0_e5161 * w[188]);
        let noise_metadata_schedule_531_0_e5168: f64 = (params.p61 * w[195]);
        let noise_metadata_schedule_531_0_e5169: f64 = (w[152] / noise_metadata_schedule_531_0_e5168);
        let noise_metadata_schedule_531_0_e5170: f64 = (w[196] - noise_metadata_schedule_531_0_e5169);
        let noise_metadata_schedule_531_0_e5171: f64 = (noise_metadata_schedule_531_0_e5163 * noise_metadata_schedule_531_0_e5170);
        let noise_metadata_schedule_531_0_e5172: f64 = (w[192] - noise_metadata_schedule_531_0_e5171);
        (noise_metadata_schedule_531_0_e5172,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_531_0_e5174;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_532_0_e5204,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[539] == 0.0)) {
        let noise_metadata_schedule_532_0_e5188: f64 = (w[197] - w[193]);
        let noise_metadata_schedule_532_0_e5191: f64 = (w[197] - w[193]);
        let noise_metadata_schedule_532_0_e5192: f64 = (noise_metadata_schedule_532_0_e5188 * noise_metadata_schedule_532_0_e5191);
        let noise_metadata_schedule_532_0_e5195: f64 = (0.1 * w[192]);
        let noise_metadata_schedule_532_0_e5197: f64 = (noise_metadata_schedule_532_0_e5195 * w[192]);
        let noise_metadata_schedule_532_0_e5199: f64 = (noise_metadata_schedule_532_0_e5197 * w[130]);
        let noise_metadata_schedule_532_0_e5201: f64 = (noise_metadata_schedule_532_0_e5199 / params.p61);
        let noise_metadata_schedule_532_0_e5202: f64 = (noise_metadata_schedule_532_0_e5192 + noise_metadata_schedule_532_0_e5201);
        (noise_metadata_schedule_532_0_e5202,)
    } else {
        (w[266],)
    }
};
            w[266] = noise_metadata_schedule_532_0_e5204;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_533_0_e5225,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[539] == 0.0)) {
        let noise_metadata_schedule_533_0_e5219: f64 = (w[197] + w[193]);
        let noise_metadata_schedule_533_0_e5221: f64 = (w[266]).sqrt();
        let noise_metadata_schedule_533_0_e5222: f64 = (noise_metadata_schedule_533_0_e5219 + noise_metadata_schedule_533_0_e5221);
        let noise_metadata_schedule_533_0_e5223: f64 = (0.5 * noise_metadata_schedule_533_0_e5222);
        (noise_metadata_schedule_533_0_e5223,)
    } else {
        (w[194],)
    }
};
            w[194] = noise_metadata_schedule_533_0_e5225;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_534_0_e5240,) = {
    if ((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_534_0_e5236: f64 = (w[194] - w[192]);
        let noise_metadata_schedule_534_0_e5238: f64 = (noise_metadata_schedule_534_0_e5236 / w[194]);
        (noise_metadata_schedule_534_0_e5238,)
    } else {
        (w[273],)
    }
};
            w[273] = noise_metadata_schedule_534_0_e5240;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_535_0_e5242: f64 = (w[273]).abs();
            let noise_metadata_schedule_535_0_e5244: f64 = if noise_metadata_schedule_535_0_e5242 > 1e-7 { 1.0 } else { 0.0 };
            w[540] = noise_metadata_schedule_535_0_e5244;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_536_0_e5261,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[540] != 0.0)) {
        let noise_metadata_schedule_536_0_e5257: f64 = (0.5 * w[191]);
        let noise_metadata_schedule_536_0_e5259: f64 = (noise_metadata_schedule_536_0_e5257 / w[273]);
        (noise_metadata_schedule_536_0_e5259,)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_536_0_e5261;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_537_0_e5298,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[540] != 0.0)) {
        let noise_metadata_schedule_537_0_e5274: f64 = (w[0] / w[99]);
        let noise_metadata_schedule_537_0_e5276: f64 = (noise_metadata_schedule_537_0_e5274 * w[194]);
        let noise_metadata_schedule_537_0_e5278: f64 = (noise_metadata_schedule_537_0_e5276 * w[198]);
        let noise_metadata_schedule_537_0_e5280: f64 = (-w[99]);
        let noise_metadata_schedule_537_0_e5282: f64 = (noise_metadata_schedule_537_0_e5280 / w[194]);
        let noise_metadata_schedule_537_0_e5283: f64 = (noise_metadata_schedule_537_0_e5282).exp();
        let noise_metadata_schedule_537_0_e5285: f64 = (-w[99]);
        let noise_metadata_schedule_537_0_e5287: f64 = (noise_metadata_schedule_537_0_e5285 / w[194]);
        let noise_metadata_schedule_537_0_e5291: f64 = (w[190] / w[198]);
        let noise_metadata_schedule_537_0_e5292: f64 = (1.0 + noise_metadata_schedule_537_0_e5291);
        let noise_metadata_schedule_537_0_e5293: f64 = (noise_metadata_schedule_537_0_e5287 * noise_metadata_schedule_537_0_e5292);
        let noise_metadata_schedule_537_0_e5294: f64 = (noise_metadata_schedule_537_0_e5293).exp();
        let noise_metadata_schedule_537_0_e5295: f64 = (noise_metadata_schedule_537_0_e5283 - noise_metadata_schedule_537_0_e5294);
        let noise_metadata_schedule_537_0_e5296: f64 = (noise_metadata_schedule_537_0_e5278 * noise_metadata_schedule_537_0_e5295);
        (noise_metadata_schedule_537_0_e5296,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_537_0_e5298;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_538_0_e5320,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[540] == 0.0)) {
        let noise_metadata_schedule_538_0_e5312: f64 = (w[0] * w[190]);
        let noise_metadata_schedule_538_0_e5314: f64 = (-w[99]);
        let noise_metadata_schedule_538_0_e5316: f64 = (noise_metadata_schedule_538_0_e5314 / w[194]);
        let noise_metadata_schedule_538_0_e5317: f64 = (noise_metadata_schedule_538_0_e5316).exp();
        let noise_metadata_schedule_538_0_e5318: f64 = (noise_metadata_schedule_538_0_e5312 * noise_metadata_schedule_538_0_e5317);
        (noise_metadata_schedule_538_0_e5318,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_538_0_e5320;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_539_0_e5323: f64 = if params.p38 == 3.0 { 1.0 } else { 0.0 };
            w[541] = noise_metadata_schedule_539_0_e5323;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_540_0_e5326: f64 = if w[236] < params.p43 { 1.0 } else { 0.0 };
            w[542] = noise_metadata_schedule_540_0_e5326;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_541_0_e5354,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) {
        let noise_metadata_schedule_541_0_e5340: f64 = (params.p43 - w[236]);
        let noise_metadata_schedule_541_0_e5342: f64 = (noise_metadata_schedule_541_0_e5340).powf(params.p40);
        let noise_metadata_schedule_541_0_e5347: f64 = (params.p47 + w[152]);
        let noise_metadata_schedule_541_0_e5348: f64 = (w[152] / noise_metadata_schedule_541_0_e5347);
        let noise_metadata_schedule_541_0_e5349: f64 = (1.0 - noise_metadata_schedule_541_0_e5348);
        let noise_metadata_schedule_541_0_e5351: f64 = (noise_metadata_schedule_541_0_e5349).powf(params.p48);
        let noise_metadata_schedule_541_0_e5352: f64 = (noise_metadata_schedule_541_0_e5342 * noise_metadata_schedule_541_0_e5351);
        (noise_metadata_schedule_541_0_e5352,)
    } else {
        (w[203],)
    }
};
            w[203] = noise_metadata_schedule_541_0_e5354;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_542_0_e5357: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            w[543] = noise_metadata_schedule_542_0_e5357;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_543_0_e5373,) = {
    if ((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[543] != 0.0)) {
        (w[203],)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_543_0_e5373;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_544_0_e5394,) = {
    if ((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[543] == 0.0)) {
        let noise_metadata_schedule_544_0_e5390: f64 = (w[152] - params.p51);
        let noise_metadata_schedule_544_0_e5392: f64 = (noise_metadata_schedule_544_0_e5390 / params.p47);
        (noise_metadata_schedule_544_0_e5392,)
    } else {
        (w[205],)
    }
};
            w[205] = noise_metadata_schedule_544_0_e5394;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_545_0_e5415,) = {
    if ((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[543] == 0.0)) {
        let noise_metadata_schedule_545_0_e5411: f64 = (w[205] - 1.0);
        let noise_metadata_schedule_545_0_e5413: f64 = (noise_metadata_schedule_545_0_e5411 / params.p50);
        (noise_metadata_schedule_545_0_e5413,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_545_0_e5415;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_546_0_e5418: f64 = if w[205] < 1.0 { 1.0 } else { 0.0 };
            w[544] = noise_metadata_schedule_546_0_e5418;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_547_0_e5445,) = {
    if (((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[543] == 0.0)) && (w[544] != 0.0)) {
        let noise_metadata_schedule_547_0_e5439: f64 = (w[265]).exp();
        let noise_metadata_schedule_547_0_e5440: f64 = (1.0 + noise_metadata_schedule_547_0_e5439);
        let noise_metadata_schedule_547_0_e5441: f64 = (noise_metadata_schedule_547_0_e5440).ln();
        let noise_metadata_schedule_547_0_e5442: f64 = (params.p50 * noise_metadata_schedule_547_0_e5441);
        let noise_metadata_schedule_547_0_e5443: f64 = (1.0 + noise_metadata_schedule_547_0_e5442);
        (noise_metadata_schedule_547_0_e5443,)
    } else {
        (w[206],)
    }
};
            w[206] = noise_metadata_schedule_547_0_e5445;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_548_0_e5474,) = {
    if (((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[543] == 0.0)) && (w[544] == 0.0)) {
        let noise_metadata_schedule_548_0_e5467: f64 = (-w[265]);
        let noise_metadata_schedule_548_0_e5468: f64 = (noise_metadata_schedule_548_0_e5467).exp();
        let noise_metadata_schedule_548_0_e5469: f64 = (1.0 + noise_metadata_schedule_548_0_e5468);
        let noise_metadata_schedule_548_0_e5470: f64 = (noise_metadata_schedule_548_0_e5469).ln();
        let noise_metadata_schedule_548_0_e5471: f64 = (params.p50 * noise_metadata_schedule_548_0_e5470);
        let noise_metadata_schedule_548_0_e5472: f64 = (w[205] + noise_metadata_schedule_548_0_e5471);
        (noise_metadata_schedule_548_0_e5472,)
    } else {
        (w[206],)
    }
};
            w[206] = noise_metadata_schedule_548_0_e5474;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_549_0_e5495,) = {
    if ((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[543] == 0.0)) {
        let noise_metadata_schedule_549_0_e5492: f64 = (w[206]).powf(params.p49);
        let noise_metadata_schedule_549_0_e5493: f64 = (w[203] * noise_metadata_schedule_549_0_e5492);
        (noise_metadata_schedule_549_0_e5493,)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_549_0_e5495;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_550_0_e5497: f64 = (-w[316]);
            let noise_metadata_schedule_550_0_e5499: f64 = (noise_metadata_schedule_550_0_e5497 * w[204]);
            let noise_metadata_schedule_550_0_e5501: f64 = if noise_metadata_schedule_550_0_e5499 < params.p138 { 1.0 } else { 0.0 };
            w[545] = noise_metadata_schedule_550_0_e5501;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_551_0_e5521,) = {
    if ((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[545] != 0.0)) {
        let noise_metadata_schedule_551_0_e5516: f64 = (-w[316]);
        let noise_metadata_schedule_551_0_e5518: f64 = (noise_metadata_schedule_551_0_e5516 * w[204]);
        let noise_metadata_schedule_551_0_e5519: f64 = (noise_metadata_schedule_551_0_e5518).exp();
        (noise_metadata_schedule_551_0_e5519,)
    } else {
        (w[319],)
    }
};
            w[319] = noise_metadata_schedule_551_0_e5521;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_552_0_e5539,) = {
    if ((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[545] == 0.0)) {
        let noise_metadata_schedule_552_0_e5537: f64 = (params.p138).exp();
        (noise_metadata_schedule_552_0_e5537,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_552_0_e5539;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_553_0_e5565,) = {
    if ((((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) && (w[545] == 0.0)) {
        let noise_metadata_schedule_553_0_e5557: f64 = (-w[316]);
        let noise_metadata_schedule_553_0_e5559: f64 = (noise_metadata_schedule_553_0_e5557 * w[204]);
        let noise_metadata_schedule_553_0_e5561: f64 = (noise_metadata_schedule_553_0_e5559 - params.p138);
        let noise_metadata_schedule_553_0_e5562: f64 = (1.0 + noise_metadata_schedule_553_0_e5561);
        let noise_metadata_schedule_553_0_e5563: f64 = (w[281] * noise_metadata_schedule_553_0_e5562);
        (noise_metadata_schedule_553_0_e5563,)
    } else {
        (w[319],)
    }
};
            w[319] = noise_metadata_schedule_553_0_e5565;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_554_0_e5587,) = {
    if (((((w[531] != 0.0) && (w[532] == 0.0)) && (w[536] == 0.0)) && (w[541] != 0.0)) && (w[542] != 0.0)) {
        let noise_metadata_schedule_554_0_e5579: f64 = (params.p39 / w[316]);
        let noise_metadata_schedule_554_0_e5582: f64 = (params.p43 - w[236]);
        let noise_metadata_schedule_554_0_e5583: f64 = (noise_metadata_schedule_554_0_e5579 * noise_metadata_schedule_554_0_e5582);
        let noise_metadata_schedule_554_0_e5585: f64 = (noise_metadata_schedule_554_0_e5583 * w[319]);
        (noise_metadata_schedule_554_0_e5585,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_554_0_e5587;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_555_0_e5590: f64 = if w[199] > 0.0 { 1.0 } else { 0.0 };
            w[546] = noise_metadata_schedule_555_0_e5590;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_556_0_e5593: f64 = if params.p52 == 1.0 { 1.0 } else { 0.0 };
            w[547] = noise_metadata_schedule_556_0_e5593;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_557_0_e5619,) = {
    if (((w[531] != 0.0) && (w[546] != 0.0)) && (w[547] != 0.0)) {
        let noise_metadata_schedule_557_0_e5603: f64 = (w[30] + w[178]);
        let noise_metadata_schedule_557_0_e5604: f64 = (w[152] * noise_metadata_schedule_557_0_e5603);
        let noise_metadata_schedule_557_0_e5605: f64 = (w[6] / noise_metadata_schedule_557_0_e5604);
        let noise_metadata_schedule_557_0_e5608: f64 = (w[149] / w[35]);
        let noise_metadata_schedule_557_0_e5610: f64 = (noise_metadata_schedule_557_0_e5608 * w[42]);
        let noise_metadata_schedule_557_0_e5611: f64 = (noise_metadata_schedule_557_0_e5605 + noise_metadata_schedule_557_0_e5610);
        let noise_metadata_schedule_557_0_e5615: f64 = (w[30] + w[178]);
        let noise_metadata_schedule_557_0_e5616: f64 = (w[28] / noise_metadata_schedule_557_0_e5615);
        let noise_metadata_schedule_557_0_e5617: f64 = (noise_metadata_schedule_557_0_e5611 + noise_metadata_schedule_557_0_e5616);
        (noise_metadata_schedule_557_0_e5617,)
    } else {
        (w[200],)
    }
};
            w[200] = noise_metadata_schedule_557_0_e5619;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_558_0_e5622: f64 = if params.p38 == 3.0 { 1.0 } else { 0.0 };
            w[548] = noise_metadata_schedule_558_0_e5622;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_559_0_e5636,) = {
    if ((((w[531] != 0.0) && (w[546] != 0.0)) && (w[547] != 0.0)) && (w[548] != 0.0)) {
        let noise_metadata_schedule_559_0_e5632: f64 = (w[199] - w[200]);
        let noise_metadata_schedule_559_0_e5634: f64 = (noise_metadata_schedule_559_0_e5632 / 1e-6);
        (noise_metadata_schedule_559_0_e5634,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_559_0_e5636;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_560_0_e5639: f64 = if w[199] < w[200] { 1.0 } else { 0.0 };
            w[549] = noise_metadata_schedule_560_0_e5639;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_561_0_e5659,) = {
    if (((((w[531] != 0.0) && (w[546] != 0.0)) && (w[547] != 0.0)) && (w[548] != 0.0)) && (w[549] != 0.0)) {
        let noise_metadata_schedule_561_0_e5653: f64 = (w[265]).exp();
        let noise_metadata_schedule_561_0_e5654: f64 = (1.0 + noise_metadata_schedule_561_0_e5653);
        let noise_metadata_schedule_561_0_e5655: f64 = (noise_metadata_schedule_561_0_e5654).ln();
        let noise_metadata_schedule_561_0_e5656: f64 = (1e-6 * noise_metadata_schedule_561_0_e5655);
        let noise_metadata_schedule_561_0_e5657: f64 = (w[199] - noise_metadata_schedule_561_0_e5656);
        (noise_metadata_schedule_561_0_e5657,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_561_0_e5659;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_562_0_e5681,) = {
    if (((((w[531] != 0.0) && (w[546] != 0.0)) && (w[547] != 0.0)) && (w[548] != 0.0)) && (w[549] == 0.0)) {
        let noise_metadata_schedule_562_0_e5674: f64 = (-w[265]);
        let noise_metadata_schedule_562_0_e5675: f64 = (noise_metadata_schedule_562_0_e5674).exp();
        let noise_metadata_schedule_562_0_e5676: f64 = (1.0 + noise_metadata_schedule_562_0_e5675);
        let noise_metadata_schedule_562_0_e5677: f64 = (noise_metadata_schedule_562_0_e5676).ln();
        let noise_metadata_schedule_562_0_e5678: f64 = (1e-6 * noise_metadata_schedule_562_0_e5677);
        let noise_metadata_schedule_562_0_e5679: f64 = (w[200] - noise_metadata_schedule_562_0_e5678);
        (noise_metadata_schedule_562_0_e5679,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_562_0_e5681;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_10(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 585], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_563_0_e5693,) = {
    if ((((w[531] != 0.0) && (w[546] != 0.0)) && (w[547] != 0.0)) && (w[548] != 0.0)) {
        let noise_metadata_schedule_563_0_e5691: f64 = (w[152] * w[199]);
        (noise_metadata_schedule_563_0_e5691,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_563_0_e5693;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_564_0_e5712,) = {
    if ((((w[531] != 0.0) && (w[546] != 0.0)) && (w[547] != 0.0)) && (w[548] == 0.0)) {
        let noise_metadata_schedule_564_0_e5704: f64 = (w[152] * w[199]);
        let noise_metadata_schedule_564_0_e5706: f64 = (noise_metadata_schedule_564_0_e5704 * w[200]);
        let noise_metadata_schedule_564_0_e5709: f64 = (w[199] + w[200]);
        let noise_metadata_schedule_564_0_e5710: f64 = (noise_metadata_schedule_564_0_e5706 / noise_metadata_schedule_564_0_e5709);
        (noise_metadata_schedule_564_0_e5710,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_564_0_e5712;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_565_0_e5723,) = {
    if (((w[531] != 0.0) && (w[546] != 0.0)) && (w[547] == 0.0)) {
        let noise_metadata_schedule_565_0_e5721: f64 = (w[152] * w[199]);
        (noise_metadata_schedule_565_0_e5721,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_565_0_e5723;
        }
        if (active[0] & 0x1fe0038) != 0 {
            let noise_metadata_schedule_656_0_e6673: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_656_0_e6675: f64 = (noise_metadata_schedule_656_0_e6673 * w[2]);
            w[287] = noise_metadata_schedule_656_0_e6675;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_657_0_e6678: f64 = (w[287] / w[28]);
            w[288] = noise_metadata_schedule_657_0_e6678;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_658_0_e6681: f64 = (w[287] / w[30]);
            w[289] = noise_metadata_schedule_658_0_e6681;
        }
        if (active[0] & 0x1520000) != 0 {
            let noise_metadata_schedule_659_0_e6684: f64 = (w[287] * w[104]);
            w[290] = noise_metadata_schedule_659_0_e6684;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_660_0_e6687: f64 = (w[287] * w[105]);
            w[291] = noise_metadata_schedule_660_0_e6687;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_661_0_e6690: f64 = (w[287] * w[106]);
            w[292] = noise_metadata_schedule_661_0_e6690;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_662_0_e6693: f64 = (w[287] / w[178]);
            let noise_metadata_schedule_662_0_e6696: f64 = (4.0 * w[253]);
            let noise_metadata_schedule_662_0_e6698: f64 = (noise_metadata_schedule_662_0_e6696 + 5.0);
            let noise_metadata_schedule_662_0_e6699: f64 = (noise_metadata_schedule_662_0_e6693 * noise_metadata_schedule_662_0_e6698);
            let noise_metadata_schedule_662_0_e6701: f64 = (noise_metadata_schedule_662_0_e6699 * 0.3333333333333333);
            w[293] = noise_metadata_schedule_662_0_e6701;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_663_0_e6704: f64 = (w[151] + w[150]);
            let noise_metadata_schedule_663_0_e6706: f64 = (noise_metadata_schedule_663_0_e6704 / w[149]);
            w[309] = noise_metadata_schedule_663_0_e6706;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_664_0_e6709: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_664_0_e6711: f64 = (w[309]).abs();
            let noise_metadata_schedule_664_0_e6712: f64 = (noise_metadata_schedule_664_0_e6709 * noise_metadata_schedule_664_0_e6711);
            w[294] = noise_metadata_schedule_664_0_e6712;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_665_0_e6715: f64 = if params.p129 > 0.0 { 1.0 } else { 0.0 };
            w[569] = noise_metadata_schedule_665_0_e6715;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_666_0_e6722,) = {
    if (w[569] != 0.0) {
        let noise_metadata_schedule_666_0_e6719: f64 = (w[201] / w[309]);
        let noise_metadata_schedule_666_0_e6720: f64 = (noise_metadata_schedule_666_0_e6719).abs();
        (noise_metadata_schedule_666_0_e6720,)
    } else {
        (w[310],)
    }
};
            w[310] = noise_metadata_schedule_666_0_e6722;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_667_0_e6727,) = {
    if (w[569] == 0.0) {
        (0.0,)
    } else {
        (w[310],)
    }
};
            w[310] = noise_metadata_schedule_667_0_e6727;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_668_0_e6730: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_668_0_e6732: f64 = (noise_metadata_schedule_668_0_e6730 * w[201]);
            let noise_metadata_schedule_668_0_e6735: f64 = (w[310] + 1.0);
            let noise_metadata_schedule_668_0_e6736: f64 = (noise_metadata_schedule_668_0_e6732 * noise_metadata_schedule_668_0_e6735);
            w[306] = noise_metadata_schedule_668_0_e6736;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_677_0_e6788: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_677_0_e6791: f64 = (w[154] + w[156]);
            let noise_metadata_schedule_677_0_e6793: f64 = (noise_metadata_schedule_677_0_e6791 - w[57]);
            let noise_metadata_schedule_677_0_e6795: f64 = (noise_metadata_schedule_677_0_e6793 + w[334]);
            let noise_metadata_schedule_677_0_e6797: f64 = (noise_metadata_schedule_677_0_e6795 + w[333]);
            let noise_metadata_schedule_677_0_e6798: f64 = (noise_metadata_schedule_677_0_e6797).abs();
            let noise_metadata_schedule_677_0_e6799: f64 = (noise_metadata_schedule_677_0_e6788 * noise_metadata_schedule_677_0_e6798);
            w[295] = noise_metadata_schedule_677_0_e6799;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_678_0_e6802: f64 = (w[154] + w[155]);
            w[307] = noise_metadata_schedule_678_0_e6802;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_679_0_e6805: f64 = (w[307]).abs();
            let noise_metadata_schedule_679_0_e6807: f64 = (noise_metadata_schedule_679_0_e6805).powf(params.p125);
            let noise_metadata_schedule_679_0_e6808: f64 = (params.p127 * noise_metadata_schedule_679_0_e6807);
            w[296] = noise_metadata_schedule_679_0_e6808;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_680_0_e6811: f64 = if w[307] < 0.0 { 1.0 } else { 0.0 };
            w[573] = noise_metadata_schedule_680_0_e6811;
        }
        if (active[0] & 0x40) != 0 {
            let (noise_metadata_schedule_681_0_e6816,) = {
    if (w[573] != 0.0) {
        let noise_metadata_schedule_681_0_e6814: f64 = (-w[296]);
        (noise_metadata_schedule_681_0_e6814,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_681_0_e6816;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_682_0_e6819: f64 = (w[156] + w[158]);
            let noise_metadata_schedule_682_0_e6821: f64 = (noise_metadata_schedule_682_0_e6819 + w[159]);
            w[308] = noise_metadata_schedule_682_0_e6821;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_683_0_e6824: f64 = (w[308]).abs();
            let noise_metadata_schedule_683_0_e6826: f64 = (noise_metadata_schedule_683_0_e6824).powf(params.p126);
            let noise_metadata_schedule_683_0_e6827: f64 = (params.p128 * noise_metadata_schedule_683_0_e6826);
            w[297] = noise_metadata_schedule_683_0_e6827;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_684_0_e6830: f64 = if w[308] < 0.0 { 1.0 } else { 0.0 };
            w[574] = noise_metadata_schedule_684_0_e6830;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_685_0_e6835,) = {
    if (w[574] != 0.0) {
        let noise_metadata_schedule_685_0_e6833: f64 = (-w[297]);
        (noise_metadata_schedule_685_0_e6833,)
    } else {
        (w[297],)
    }
};
            w[297] = noise_metadata_schedule_685_0_e6835;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_686_0_e6838: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_686_0_e6841: f64 = (w[155] + w[158]);
            let noise_metadata_schedule_686_0_e6843: f64 = (noise_metadata_schedule_686_0_e6841 + w[159]);
            let noise_metadata_schedule_686_0_e6844: f64 = (noise_metadata_schedule_686_0_e6843).abs();
            let noise_metadata_schedule_686_0_e6845: f64 = (noise_metadata_schedule_686_0_e6838 * noise_metadata_schedule_686_0_e6844);
            w[298] = noise_metadata_schedule_686_0_e6845;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_687_0_e6848: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_687_0_e6850: f64 = (w[157]).abs();
            let noise_metadata_schedule_687_0_e6851: f64 = (noise_metadata_schedule_687_0_e6848 * noise_metadata_schedule_687_0_e6850);
            w[299] = noise_metadata_schedule_687_0_e6851;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_688_0_e6854: f64 = (w[157]).abs();
            let noise_metadata_schedule_688_0_e6856: f64 = (noise_metadata_schedule_688_0_e6854).powf(params.p125);
            let noise_metadata_schedule_688_0_e6857: f64 = (params.p127 * noise_metadata_schedule_688_0_e6856);
            w[300] = noise_metadata_schedule_688_0_e6857;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_689_0_e6860: f64 = if w[157] < 0.0 { 1.0 } else { 0.0 };
            w[575] = noise_metadata_schedule_689_0_e6860;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_690_0_e6865,) = {
    if (w[575] != 0.0) {
        let noise_metadata_schedule_690_0_e6863: f64 = (-w[300]);
        (noise_metadata_schedule_690_0_e6863,)
    } else {
        (w[300],)
    }
};
            w[300] = noise_metadata_schedule_690_0_e6865;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_691_0_e6868: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_691_0_e6870: f64 = (w[82]).abs();
            let noise_metadata_schedule_691_0_e6871: f64 = (noise_metadata_schedule_691_0_e6868 * noise_metadata_schedule_691_0_e6870);
            w[301] = noise_metadata_schedule_691_0_e6871;
        }
        if (active[0] & 0x800) != 0 {
            let noise_metadata_schedule_692_0_e6874: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_692_0_e6876: f64 = (w[160]).abs();
            let noise_metadata_schedule_692_0_e6877: f64 = (noise_metadata_schedule_692_0_e6874 * noise_metadata_schedule_692_0_e6876);
            w[302] = noise_metadata_schedule_692_0_e6877;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_693_0_e6882: f64 = (params.p5 * params.p32);
            let noise_metadata_schedule_693_0_e6883: f64 = (1.0 - noise_metadata_schedule_693_0_e6882);
            let noise_metadata_schedule_693_0_e6884: f64 = (params.p127 * noise_metadata_schedule_693_0_e6883);
            let noise_metadata_schedule_693_0_e6886: f64 = (w[160]).abs();
            let noise_metadata_schedule_693_0_e6890: f64 = (params.p5 * params.p32);
            let noise_metadata_schedule_693_0_e6891: f64 = (1.0 - noise_metadata_schedule_693_0_e6890);
            let noise_metadata_schedule_693_0_e6892: f64 = (noise_metadata_schedule_693_0_e6886 / noise_metadata_schedule_693_0_e6891);
            let noise_metadata_schedule_693_0_e6894: f64 = (noise_metadata_schedule_693_0_e6892).powf(params.p125);
            let noise_metadata_schedule_693_0_e6895: f64 = (noise_metadata_schedule_693_0_e6884 * noise_metadata_schedule_693_0_e6894);
            w[304] = noise_metadata_schedule_693_0_e6895;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_694_0_e6898: f64 = if w[160] < 0.0 { 1.0 } else { 0.0 };
            w[576] = noise_metadata_schedule_694_0_e6898;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_695_0_e6903,) = {
    if (w[576] != 0.0) {
        let noise_metadata_schedule_695_0_e6901: f64 = (-w[304]);
        (noise_metadata_schedule_695_0_e6901,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_695_0_e6903;
        }
        if (active[0] & 0x2000) != 0 {
            let noise_metadata_schedule_696_0_e6906: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_696_0_e6908: f64 = (w[172]).abs();
            let noise_metadata_schedule_696_0_e6909: f64 = (noise_metadata_schedule_696_0_e6906 * noise_metadata_schedule_696_0_e6908);
            let noise_metadata_schedule_696_0_e6911: f64 = (noise_metadata_schedule_696_0_e6909 * params.p5);
            w[303] = noise_metadata_schedule_696_0_e6911;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_697_0_e6914: f64 = if params.p32 == 0.0 { 1.0 } else { 0.0 };
            w[577] = noise_metadata_schedule_697_0_e6914;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_698_0_e6918,) = {
    if (w[577] != 0.0) {
        (0.0,)
    } else {
        (w[305],)
    }
};
            w[305] = noise_metadata_schedule_698_0_e6918;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_699_0_e6934,) = {
    if (w[577] == 0.0) {
        let noise_metadata_schedule_699_0_e6923: f64 = (params.p127 * params.p5);
        let noise_metadata_schedule_699_0_e6925: f64 = (noise_metadata_schedule_699_0_e6923 * params.p32);
        let noise_metadata_schedule_699_0_e6927: f64 = (w[172]).abs();
        let noise_metadata_schedule_699_0_e6929: f64 = (noise_metadata_schedule_699_0_e6927 / params.p32);
        let noise_metadata_schedule_699_0_e6931: f64 = (noise_metadata_schedule_699_0_e6929).powf(params.p125);
        let noise_metadata_schedule_699_0_e6932: f64 = (noise_metadata_schedule_699_0_e6925 * noise_metadata_schedule_699_0_e6931);
        (noise_metadata_schedule_699_0_e6932,)
    } else {
        (w[305],)
    }
};
            w[305] = noise_metadata_schedule_699_0_e6934;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_700_0_e6937: f64 = if w[172] < 0.0 { 1.0 } else { 0.0 };
            w[578] = noise_metadata_schedule_700_0_e6937;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_701_0_e6942,) = {
    if (w[578] != 0.0) {
        let noise_metadata_schedule_701_0_e6940: f64 = (-w[305]);
        (noise_metadata_schedule_701_0_e6940,)
    } else {
        (w[305],)
    }
};
            w[305] = noise_metadata_schedule_701_0_e6942;
        }
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 25] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 571];
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
            w[565] != 0.0
        };
        let noise_source_16_active = {
            let noise_16_activation_e445: f64 = if (w[565] == 0.0) { 1.0 } else { 0.0 };
            noise_16_activation_e445 != 0.0
        };
        let noise_source_17_active = {
            let noise_17_activation_e455: f64 = if ((w[566] != 0.0) && (w[567] != 0.0)) { 1.0 } else { 0.0 };
            noise_17_activation_e455 != 0.0
        };
        let noise_source_18_active = {
            let noise_18_activation_e465: f64 = if ((w[566] != 0.0) && (w[567] != 0.0)) { 1.0 } else { 0.0 };
            noise_18_activation_e465 != 0.0
        };
        let noise_source_19_active = {
            let noise_19_activation_e475: f64 = if ((w[566] != 0.0) && (w[567] != 0.0)) { 1.0 } else { 0.0 };
            noise_19_activation_e475 != 0.0
        };
        let noise_source_20_active = {
            let noise_20_activation_e486: f64 = if ((w[566] != 0.0) && (w[567] == 0.0)) { 1.0 } else { 0.0 };
            noise_20_activation_e486 != 0.0
        };
        let noise_source_21_active = {
            let noise_21_activation_e497: f64 = if ((w[566] != 0.0) && (w[567] == 0.0)) { 1.0 } else { 0.0 };
            noise_21_activation_e497 != 0.0
        };
        let noise_source_22_active = {
            let noise_22_activation_e508: f64 = if ((w[566] == 0.0) && (w[568] != 0.0)) { 1.0 } else { 0.0 };
            noise_22_activation_e508 != 0.0
        };
        let noise_source_23_active = {
            let noise_23_activation_e519: f64 = if ((w[566] == 0.0) && (w[568] != 0.0)) { 1.0 } else { 0.0 };
            noise_23_activation_e519 != 0.0
        };
        let noise_source_24_active = {
            let noise_24_activation_e531: f64 = if ((w[566] == 0.0) && (w[568] == 0.0)) { 1.0 } else { 0.0 };
            noise_24_activation_e531 != 0.0
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
            let noise_0_psd_e7754: f64 = 1.0;
            let noise_0_psd_e349: f64 = (w[288] * params[1]);
            let noise_0_psd_e7755: f64 = (noise_0_psd_e7754 * noise_0_psd_e349);
            let psd = noise_0_psd_e7755;
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
            let noise_1_psd_e7757: f64 = 1.0;
            let noise_1_psd_e363: f64 = (w[300] * params[1]);
            let noise_1_psd_e7758: f64 = (noise_1_psd_e7757 * noise_1_psd_e363);
            let psd = noise_1_psd_e7758;
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
            let noise_2_psd_e7760: f64 = 1.0;
            let noise_2_psd_e368: f64 = (w[289] * params[1]);
            let noise_2_psd_e7761: f64 = (noise_2_psd_e7760 * noise_2_psd_e368);
            let psd = noise_2_psd_e7761;
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
            let noise_3_psd_e7763: f64 = 1.0;
            let noise_3_psd_e373: f64 = (w[282] * params[1]);
            let noise_3_psd_e7764: f64 = (noise_3_psd_e7763 * noise_3_psd_e373);
            let psd = noise_3_psd_e7764;
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
            let noise_4_psd_e7766: f64 = 1.0;
            let noise_4_psd_e378: f64 = (w[283] * params[1]);
            let noise_4_psd_e7767: f64 = (noise_4_psd_e7766 * noise_4_psd_e378);
            let psd = noise_4_psd_e7767;
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
            let noise_5_psd_e7769: f64 = 1.0;
            let noise_5_psd_e383: f64 = (w[287] * params[1]);
            let noise_5_psd_e7770: f64 = (noise_5_psd_e7769 * noise_5_psd_e383);
            let psd = noise_5_psd_e7770;
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
            let noise_6_psd_e7772: f64 = 1.0;
            let noise_6_psd_e388: f64 = (w[290] * params[1]);
            let noise_6_psd_e7773: f64 = (noise_6_psd_e7772 * noise_6_psd_e388);
            let psd = noise_6_psd_e7773;
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
            let noise_7_psd_e7775: f64 = 1.0;
            let noise_7_psd_e394: f64 = (w[291] * params[1]);
            let noise_7_psd_e7776: f64 = (noise_7_psd_e7775 * noise_7_psd_e394);
            let psd = noise_7_psd_e7776;
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
            let noise_8_psd_e7778: f64 = 1.0;
            let noise_8_psd_e400: f64 = (w[292] * params[1]);
            let noise_8_psd_e7779: f64 = (noise_8_psd_e7778 * noise_8_psd_e400);
            let psd = noise_8_psd_e7779;
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
            let noise_9_psd_e7781: f64 = 1.0;
            let noise_9_psd_e405: f64 = (w[293] * params[1]);
            let noise_9_psd_e7782: f64 = (noise_9_psd_e7781 * noise_9_psd_e405);
            let psd = noise_9_psd_e7782;
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
            let noise_10_psd_e7784: f64 = 1.0;
            let noise_10_psd_e410: f64 = (w[294] * params[1]);
            let noise_10_psd_e7785: f64 = (noise_10_psd_e7784 * noise_10_psd_e410);
            let psd = noise_10_psd_e7785;
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
            let noise_11_psd_e7787: f64 = 1.0;
            let noise_11_psd_e416: f64 = (w[296] * params[1]);
            let noise_11_psd_e7788: f64 = (noise_11_psd_e7787 * noise_11_psd_e416);
            let psd = noise_11_psd_e7788;
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
            let noise_12_psd_e7790: f64 = 1.0;
            let noise_12_psd_e421: f64 = (w[298] * params[1]);
            let noise_12_psd_e7791: f64 = (noise_12_psd_e7790 * noise_12_psd_e421);
            let psd = noise_12_psd_e7791;
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
            let noise_13_psd_e7793: f64 = 1.0;
            let noise_13_psd_e427: f64 = (w[297] * params[1]);
            let noise_13_psd_e7794: f64 = (noise_13_psd_e7793 * noise_13_psd_e427);
            let psd = noise_13_psd_e7794;
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
            let noise_14_psd_e7796: f64 = 1.0;
            let noise_14_psd_e432: f64 = (w[299] * params[1]);
            let noise_14_psd_e7797: f64 = (noise_14_psd_e7796 * noise_14_psd_e432);
            let psd = noise_14_psd_e7797;
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
            let noise_15_psd_e7799: f64 = 1.0;
            let noise_15_psd_e439: f64 = (w[295] * params[1]);
            let noise_15_psd_e7800: f64 = (noise_15_psd_e7799 * noise_15_psd_e439);
            let psd = noise_15_psd_e7800;
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
            let noise_16_psd_e7802: f64 = 1.0;
            let noise_16_psd_e448: f64 = (w[295] * params[1]);
            let noise_16_psd_e7803: f64 = (noise_16_psd_e7802 * noise_16_psd_e448);
            let psd = noise_16_psd_e7803;
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
            let noise_17_psd_e7805: f64 = 1.0;
            let noise_17_psd_e458: f64 = (w[284] * params[1]);
            let noise_17_psd_e7806: f64 = (noise_17_psd_e7805 * noise_17_psd_e458);
            let psd = noise_17_psd_e7806;
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
            let noise_18_psd_e7808: f64 = 1.0;
            let noise_18_psd_e468: f64 = (w[285] * params[1]);
            let noise_18_psd_e7809: f64 = (noise_18_psd_e7808 * noise_18_psd_e468);
            let psd = noise_18_psd_e7809;
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
            let noise_19_psd_e7811: f64 = 1.0;
            let noise_19_psd_e478: f64 = (w[286] * params[1]);
            let noise_19_psd_e7812: f64 = (noise_19_psd_e7811 * noise_19_psd_e478);
            let psd = noise_19_psd_e7812;
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
            let noise_20_psd_e7814: f64 = 1.0;
            let noise_20_psd_e489: f64 = (w[284] * params[1]);
            let noise_20_psd_e7815: f64 = (noise_20_psd_e7814 * noise_20_psd_e489);
            let psd = noise_20_psd_e7815;
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
            let noise_21_psd_e7817: f64 = 1.0;
            let noise_21_psd_e500: f64 = (w[285] * params[1]);
            let noise_21_psd_e7818: f64 = (noise_21_psd_e7817 * noise_21_psd_e500);
            let psd = noise_21_psd_e7818;
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
            let noise_22_psd_e7820: f64 = 1.0;
            let noise_22_psd_e511: f64 = (w[284] * params[1]);
            let noise_22_psd_e7821: f64 = (noise_22_psd_e7820 * noise_22_psd_e511);
            let psd = noise_22_psd_e7821;
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
            let noise_23_psd_e7823: f64 = 1.0;
            let noise_23_psd_e522: f64 = (w[286] * params[1]);
            let noise_23_psd_e7824: f64 = (noise_23_psd_e7823 * noise_23_psd_e522);
            let psd = noise_23_psd_e7824;
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
            let noise_24_psd_e7826: f64 = 1.0;
            let noise_24_psd_e534: f64 = (w[284] * params[1]);
            let noise_24_psd_e7827: f64 = (noise_24_psd_e7826 * noise_24_psd_e534);
            let psd = noise_24_psd_e7827;
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
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571]) {
        let params = &*self.params;
        let noise_activation_schedule_683_0_e6711: f64 = if params[23] == 1.0 { 1.0 } else { 0.0 };
        w[565] = noise_activation_schedule_683_0_e6711;
        let noise_activation_schedule_684_0_e6714: f64 = if params[57] > 0.0 { 1.0 } else { 0.0 };
        w[566] = noise_activation_schedule_684_0_e6714;
        let noise_activation_schedule_685_0_e6717: f64 = if params[58] > 0.0 { 1.0 } else { 0.0 };
        w[567] = noise_activation_schedule_685_0_e6717;
        let noise_activation_schedule_686_0_e6720: f64 = if params[58] > 0.0 { 1.0 } else { 0.0 };
        w[568] = noise_activation_schedule_686_0_e6720;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_0_0_e541: f64 = if params[3] == 1.0 { 1.0 } else { 0.0 };
            w[439] = noise_metadata_schedule_0_0_e541;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_1_0_e545,) = {
    if (w[439] != 0.0) {
        (70300000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_1_0_e545;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_2_0_e549,) = {
    if (w[439] != 0.0) {
        (123000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_2_0_e549;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3_0_e554,) = {
    if (w[439] == 0.0) {
        (158000000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_3_0_e554;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_4_0_e559,) = {
    if (w[439] == 0.0) {
        (204000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_4_0_e559;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_5_0_e562: f64 = (1.0 - params[32]);
            w[150] = noise_metadata_schedule_5_0_e562;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_6_0_e565: f64 = (params[4] + 273.15);
            w[3] = noise_metadata_schedule_6_0_e565;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_7_0_e566: f64 = ctx.temperature();
            let noise_metadata_schedule_7_0_e568: f64 = (noise_metadata_schedule_7_0_e566 + params[0]);
            w[5] = noise_metadata_schedule_7_0_e568;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let noise_metadata_schedule_9_0_e574: f64 = if params[137] == 0.0 { 1.0 } else { 0.0 };
            w[440] = noise_metadata_schedule_9_0_e574;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let (noise_metadata_schedule_10_0_e578,) = {
    if (w[440] != 0.0) {
        (1e-12,)
    } else {
        (w[315],)
    }
};
            w[315] = noise_metadata_schedule_10_0_e578;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let (noise_metadata_schedule_11_0_e583,) = {
    if (w[440] == 0.0) {
        (params[137],)
    } else {
        (w[315],)
    }
};
            w[315] = noise_metadata_schedule_11_0_e583;
        }
        if (active[0] & 0x1fe003a) != 0 {
            let noise_metadata_schedule_12_0_e586: f64 = (w[315] * params[1]);
            w[316] = noise_metadata_schedule_12_0_e586;
        }
        if (active[0] & 0x1fe0000) != 0 {
            let noise_metadata_schedule_13_0_e589: f64 = (1.0 / w[316]);
            w[317] = noise_metadata_schedule_13_0_e589;
        }
        if (active[0] & 0x1ffe7) != 0 {
            w[52] = 0.001;
        }
        if (active[0] & 0x1ffe7) != 0 {
            w[312] = 0.001;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_16_0_e595: f64 = (2.0 - params[66]);
            let noise_metadata_schedule_16_0_e596: f64 = (2.0_f64).powf(noise_metadata_schedule_16_0_e595);
            w[62] = noise_metadata_schedule_16_0_e596;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_17_0_e599: f64 = (1.0 / w[62]);
            w[63] = noise_metadata_schedule_17_0_e599;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_18_0_e603: f64 = (params[114] * w[3]);
            let noise_metadata_schedule_18_0_e605: f64 = (noise_metadata_schedule_18_0_e603 * w[3]);
            let noise_metadata_schedule_18_0_e608: f64 = (w[3] + params[115]);
            let noise_metadata_schedule_18_0_e609: f64 = (noise_metadata_schedule_18_0_e605 / noise_metadata_schedule_18_0_e608);
            let noise_metadata_schedule_18_0_e610: f64 = (params[113] + noise_metadata_schedule_18_0_e609);
            let noise_metadata_schedule_18_0_e612: f64 = (noise_metadata_schedule_18_0_e610 - 0.05);
            let noise_metadata_schedule_18_0_e614: f64 = (noise_metadata_schedule_18_0_e612 / 0.1);
            w[259] = noise_metadata_schedule_18_0_e614;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_19_0_e618: f64 = (params[114] * w[3]);
            let noise_metadata_schedule_19_0_e620: f64 = (noise_metadata_schedule_19_0_e618 * w[3]);
            let noise_metadata_schedule_19_0_e623: f64 = (w[3] + params[115]);
            let noise_metadata_schedule_19_0_e624: f64 = (noise_metadata_schedule_19_0_e620 / noise_metadata_schedule_19_0_e623);
            let noise_metadata_schedule_19_0_e625: f64 = (params[113] + noise_metadata_schedule_19_0_e624);
            let noise_metadata_schedule_19_0_e627: f64 = if noise_metadata_schedule_19_0_e625 < 0.05 { 1.0 } else { 0.0 };
            w[441] = noise_metadata_schedule_19_0_e627;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_20_0_e639,) = {
    if (w[441] != 0.0) {
        let noise_metadata_schedule_20_0_e633: f64 = (w[259]).exp();
        let noise_metadata_schedule_20_0_e634: f64 = (1.0 + noise_metadata_schedule_20_0_e633);
        let noise_metadata_schedule_20_0_e635: f64 = (noise_metadata_schedule_20_0_e634).ln();
        let noise_metadata_schedule_20_0_e636: f64 = (0.1 * noise_metadata_schedule_20_0_e635);
        let noise_metadata_schedule_20_0_e637: f64 = (0.05 + noise_metadata_schedule_20_0_e636);
        (noise_metadata_schedule_20_0_e637,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_20_0_e639;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_21_0_e663,) = {
    if (w[441] == 0.0) {
        let noise_metadata_schedule_21_0_e645: f64 = (params[114] * w[3]);
        let noise_metadata_schedule_21_0_e647: f64 = (noise_metadata_schedule_21_0_e645 * w[3]);
        let noise_metadata_schedule_21_0_e650: f64 = (w[3] + params[115]);
        let noise_metadata_schedule_21_0_e651: f64 = (noise_metadata_schedule_21_0_e647 / noise_metadata_schedule_21_0_e650);
        let noise_metadata_schedule_21_0_e652: f64 = (params[113] + noise_metadata_schedule_21_0_e651);
        let noise_metadata_schedule_21_0_e656: f64 = (-w[259]);
        let noise_metadata_schedule_21_0_e657: f64 = (noise_metadata_schedule_21_0_e656).exp();
        let noise_metadata_schedule_21_0_e658: f64 = (1.0 + noise_metadata_schedule_21_0_e657);
        let noise_metadata_schedule_21_0_e659: f64 = (noise_metadata_schedule_21_0_e658).ln();
        let noise_metadata_schedule_21_0_e660: f64 = (0.1 * noise_metadata_schedule_21_0_e659);
        let noise_metadata_schedule_21_0_e661: f64 = (noise_metadata_schedule_21_0_e652 + noise_metadata_schedule_21_0_e660);
        (noise_metadata_schedule_21_0_e661,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_21_0_e663;
        }
        if (active[0] & 0x18006) != 0 {
            w[71] = params[113];
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_23_0_e667: f64 = (1.0 / w[71]);
            w[72] = noise_metadata_schedule_23_0_e667;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_24_0_e670: f64 = (1.0 / params[65]);
            w[64] = noise_metadata_schedule_24_0_e670;
        }
        if (active[0] & 0x18002) != 0 {
            w[75] = params[70];
        }
        if (active[0] & 0x18002) != 0 {
            w[76] = params[71];
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_27_0_e676: f64 = (2.0 - w[76]);
            let noise_metadata_schedule_27_0_e677: f64 = (2.0_f64).powf(noise_metadata_schedule_27_0_e676);
            w[79] = noise_metadata_schedule_27_0_e677;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_28_0_e680: f64 = (1.0 / w[79]);
            w[89] = noise_metadata_schedule_28_0_e680;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_29_0_e684: f64 = (params[117] * w[3]);
            let noise_metadata_schedule_29_0_e686: f64 = (noise_metadata_schedule_29_0_e684 * w[3]);
            let noise_metadata_schedule_29_0_e689: f64 = (w[3] + params[118]);
            let noise_metadata_schedule_29_0_e690: f64 = (noise_metadata_schedule_29_0_e686 / noise_metadata_schedule_29_0_e689);
            let noise_metadata_schedule_29_0_e691: f64 = (params[116] + noise_metadata_schedule_29_0_e690);
            let noise_metadata_schedule_29_0_e693: f64 = (noise_metadata_schedule_29_0_e691 - 0.05);
            let noise_metadata_schedule_29_0_e695: f64 = (noise_metadata_schedule_29_0_e693 / 0.1);
            w[259] = noise_metadata_schedule_29_0_e695;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_30_0_e699: f64 = (params[117] * w[3]);
            let noise_metadata_schedule_30_0_e701: f64 = (noise_metadata_schedule_30_0_e699 * w[3]);
            let noise_metadata_schedule_30_0_e704: f64 = (w[3] + params[118]);
            let noise_metadata_schedule_30_0_e705: f64 = (noise_metadata_schedule_30_0_e701 / noise_metadata_schedule_30_0_e704);
            let noise_metadata_schedule_30_0_e706: f64 = (params[116] + noise_metadata_schedule_30_0_e705);
            let noise_metadata_schedule_30_0_e708: f64 = if noise_metadata_schedule_30_0_e706 < 0.05 { 1.0 } else { 0.0 };
            w[442] = noise_metadata_schedule_30_0_e708;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_31_0_e720,) = {
    if (w[442] != 0.0) {
        let noise_metadata_schedule_31_0_e714: f64 = (w[259]).exp();
        let noise_metadata_schedule_31_0_e715: f64 = (1.0 + noise_metadata_schedule_31_0_e714);
        let noise_metadata_schedule_31_0_e716: f64 = (noise_metadata_schedule_31_0_e715).ln();
        let noise_metadata_schedule_31_0_e717: f64 = (0.1 * noise_metadata_schedule_31_0_e716);
        let noise_metadata_schedule_31_0_e718: f64 = (0.05 + noise_metadata_schedule_31_0_e717);
        (noise_metadata_schedule_31_0_e718,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_31_0_e720;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_32_0_e744,) = {
    if (w[442] == 0.0) {
        let noise_metadata_schedule_32_0_e726: f64 = (params[117] * w[3]);
        let noise_metadata_schedule_32_0_e728: f64 = (noise_metadata_schedule_32_0_e726 * w[3]);
        let noise_metadata_schedule_32_0_e731: f64 = (w[3] + params[118]);
        let noise_metadata_schedule_32_0_e732: f64 = (noise_metadata_schedule_32_0_e728 / noise_metadata_schedule_32_0_e731);
        let noise_metadata_schedule_32_0_e733: f64 = (params[116] + noise_metadata_schedule_32_0_e732);
        let noise_metadata_schedule_32_0_e737: f64 = (-w[259]);
        let noise_metadata_schedule_32_0_e738: f64 = (noise_metadata_schedule_32_0_e737).exp();
        let noise_metadata_schedule_32_0_e739: f64 = (1.0 + noise_metadata_schedule_32_0_e738);
        let noise_metadata_schedule_32_0_e740: f64 = (noise_metadata_schedule_32_0_e739).ln();
        let noise_metadata_schedule_32_0_e741: f64 = (0.1 * noise_metadata_schedule_32_0_e740);
        let noise_metadata_schedule_32_0_e742: f64 = (noise_metadata_schedule_32_0_e733 + noise_metadata_schedule_32_0_e741);
        (noise_metadata_schedule_32_0_e742,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_32_0_e744;
        }
        if (active[0] & 0x18002) != 0 {
            w[87] = params[116];
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_34_0_e748: f64 = (1.0 / w[87]);
            w[86] = noise_metadata_schedule_34_0_e748;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_35_0_e751: f64 = (1.0 / w[75]);
            w[66] = noise_metadata_schedule_35_0_e751;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_36_0_e755: f64 = (1.0 / params[82]);
            let noise_metadata_schedule_36_0_e756: f64 = (1.0 - noise_metadata_schedule_36_0_e755);
            w[318] = noise_metadata_schedule_36_0_e756;
        }
        if (active[0] & 0x44) != 0 {
            w[151] = 0.0;
        }
        if (active[0] & 0x140) != 0 {
            w[152] = 0.0;
        }
        if (active[0] & 0x6000) != 0 {
            w[169] = 0.0;
        }
        if (active[0] & 0x6000) != 0 {
            w[168] = 1.0;
        }
        if (active[0] & 0x2) != 0 {
            w[196] = 0.0;
        }
        if (active[0] & 0x2) != 0 {
            w[198] = 0.0;
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
            w[11] = 0.0;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_51_0_e773: f64 = (w[5] + w[11]);
            w[2] = noise_metadata_schedule_51_0_e773;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_52_0_e776: f64 = (w[2] / w[3]);
            w[4] = noise_metadata_schedule_52_0_e776;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_53_0_e779: f64 = (8.617086918058125e-5 * w[2]);
            w[6] = noise_metadata_schedule_53_0_e779;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_54_0_e782: f64 = (8.617086918058125e-5 * w[3]);
            w[7] = noise_metadata_schedule_54_0_e782;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_55_0_e785: f64 = (1.0 / w[6]);
            w[8] = noise_metadata_schedule_55_0_e785;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_56_0_e788: f64 = (1.0 / w[7]);
            w[9] = noise_metadata_schedule_56_0_e788;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_57_0_e791: f64 = (w[8] - w[9]);
            w[10] = noise_metadata_schedule_57_0_e791;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_58_0_e794: f64 = (w[2] - w[3]);
            w[12] = noise_metadata_schedule_58_0_e794;
        }
        if (active[0] & 0x1ffffff) != 0 {
            let noise_metadata_schedule_59_0_e796: f64 = (w[4]).ln();
            w[254] = noise_metadata_schedule_59_0_e796;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_60_0_e800: f64 = (params[114] * w[2]);
            let noise_metadata_schedule_60_0_e802: f64 = (noise_metadata_schedule_60_0_e800 * w[2]);
            let noise_metadata_schedule_60_0_e805: f64 = (w[2] + params[115]);
            let noise_metadata_schedule_60_0_e806: f64 = (noise_metadata_schedule_60_0_e802 / noise_metadata_schedule_60_0_e805);
            let noise_metadata_schedule_60_0_e807: f64 = (w[74] - noise_metadata_schedule_60_0_e806);
            let noise_metadata_schedule_60_0_e809: f64 = (noise_metadata_schedule_60_0_e807 - 0.05);
            let noise_metadata_schedule_60_0_e811: f64 = (noise_metadata_schedule_60_0_e809 / 0.1);
            w[259] = noise_metadata_schedule_60_0_e811;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_61_0_e815: f64 = (params[114] * w[2]);
            let noise_metadata_schedule_61_0_e817: f64 = (noise_metadata_schedule_61_0_e815 * w[2]);
            let noise_metadata_schedule_61_0_e820: f64 = (w[2] + params[115]);
            let noise_metadata_schedule_61_0_e821: f64 = (noise_metadata_schedule_61_0_e817 / noise_metadata_schedule_61_0_e820);
            let noise_metadata_schedule_61_0_e822: f64 = (w[74] - noise_metadata_schedule_61_0_e821);
            let noise_metadata_schedule_61_0_e824: f64 = if noise_metadata_schedule_61_0_e822 < 0.05 { 1.0 } else { 0.0 };
            w[443] = noise_metadata_schedule_61_0_e824;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_62_0_e836,) = {
    if (w[443] != 0.0) {
        let noise_metadata_schedule_62_0_e830: f64 = (w[259]).exp();
        let noise_metadata_schedule_62_0_e831: f64 = (1.0 + noise_metadata_schedule_62_0_e830);
        let noise_metadata_schedule_62_0_e832: f64 = (noise_metadata_schedule_62_0_e831).ln();
        let noise_metadata_schedule_62_0_e833: f64 = (0.1 * noise_metadata_schedule_62_0_e832);
        let noise_metadata_schedule_62_0_e834: f64 = (0.05 + noise_metadata_schedule_62_0_e833);
        (noise_metadata_schedule_62_0_e834,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_62_0_e836;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_63_0_e860,) = {
    if (w[443] == 0.0) {
        let noise_metadata_schedule_63_0_e842: f64 = (params[114] * w[2]);
        let noise_metadata_schedule_63_0_e844: f64 = (noise_metadata_schedule_63_0_e842 * w[2]);
        let noise_metadata_schedule_63_0_e847: f64 = (w[2] + params[115]);
        let noise_metadata_schedule_63_0_e848: f64 = (noise_metadata_schedule_63_0_e844 / noise_metadata_schedule_63_0_e847);
        let noise_metadata_schedule_63_0_e849: f64 = (w[74] - noise_metadata_schedule_63_0_e848);
        let noise_metadata_schedule_63_0_e853: f64 = (-w[259]);
        let noise_metadata_schedule_63_0_e854: f64 = (noise_metadata_schedule_63_0_e853).exp();
        let noise_metadata_schedule_63_0_e855: f64 = (1.0 + noise_metadata_schedule_63_0_e854);
        let noise_metadata_schedule_63_0_e856: f64 = (noise_metadata_schedule_63_0_e855).ln();
        let noise_metadata_schedule_63_0_e857: f64 = (0.1 * noise_metadata_schedule_63_0_e856);
        let noise_metadata_schedule_63_0_e858: f64 = (noise_metadata_schedule_63_0_e849 + noise_metadata_schedule_63_0_e857);
        (noise_metadata_schedule_63_0_e858,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_63_0_e860;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_64_0_e864: f64 = (params[117] * w[2]);
            let noise_metadata_schedule_64_0_e866: f64 = (noise_metadata_schedule_64_0_e864 * w[2]);
            let noise_metadata_schedule_64_0_e869: f64 = (w[2] + params[118]);
            let noise_metadata_schedule_64_0_e870: f64 = (noise_metadata_schedule_64_0_e866 / noise_metadata_schedule_64_0_e869);
            let noise_metadata_schedule_64_0_e871: f64 = (w[88] - noise_metadata_schedule_64_0_e870);
            let noise_metadata_schedule_64_0_e873: f64 = (noise_metadata_schedule_64_0_e871 - 0.05);
            let noise_metadata_schedule_64_0_e875: f64 = (noise_metadata_schedule_64_0_e873 / 0.1);
            w[259] = noise_metadata_schedule_64_0_e875;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_65_0_e879: f64 = (params[117] * w[2]);
            let noise_metadata_schedule_65_0_e881: f64 = (noise_metadata_schedule_65_0_e879 * w[2]);
            let noise_metadata_schedule_65_0_e884: f64 = (w[2] + params[118]);
            let noise_metadata_schedule_65_0_e885: f64 = (noise_metadata_schedule_65_0_e881 / noise_metadata_schedule_65_0_e884);
            let noise_metadata_schedule_65_0_e886: f64 = (w[88] - noise_metadata_schedule_65_0_e885);
            let noise_metadata_schedule_65_0_e888: f64 = if noise_metadata_schedule_65_0_e886 < 0.05 { 1.0 } else { 0.0 };
            w[444] = noise_metadata_schedule_65_0_e888;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_66_0_e900,) = {
    if (w[444] != 0.0) {
        let noise_metadata_schedule_66_0_e894: f64 = (w[259]).exp();
        let noise_metadata_schedule_66_0_e895: f64 = (1.0 + noise_metadata_schedule_66_0_e894);
        let noise_metadata_schedule_66_0_e896: f64 = (noise_metadata_schedule_66_0_e895).ln();
        let noise_metadata_schedule_66_0_e897: f64 = (0.1 * noise_metadata_schedule_66_0_e896);
        let noise_metadata_schedule_66_0_e898: f64 = (0.05 + noise_metadata_schedule_66_0_e897);
        (noise_metadata_schedule_66_0_e898,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_66_0_e900;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_67_0_e924,) = {
    if (w[444] == 0.0) {
        let noise_metadata_schedule_67_0_e906: f64 = (params[117] * w[2]);
        let noise_metadata_schedule_67_0_e908: f64 = (noise_metadata_schedule_67_0_e906 * w[2]);
        let noise_metadata_schedule_67_0_e911: f64 = (w[2] + params[118]);
        let noise_metadata_schedule_67_0_e912: f64 = (noise_metadata_schedule_67_0_e908 / noise_metadata_schedule_67_0_e911);
        let noise_metadata_schedule_67_0_e913: f64 = (w[88] - noise_metadata_schedule_67_0_e912);
        let noise_metadata_schedule_67_0_e917: f64 = (-w[259]);
        let noise_metadata_schedule_67_0_e918: f64 = (noise_metadata_schedule_67_0_e917).exp();
        let noise_metadata_schedule_67_0_e919: f64 = (1.0 + noise_metadata_schedule_67_0_e918);
        let noise_metadata_schedule_67_0_e920: f64 = (noise_metadata_schedule_67_0_e919).ln();
        let noise_metadata_schedule_67_0_e921: f64 = (0.1 * noise_metadata_schedule_67_0_e920);
        let noise_metadata_schedule_67_0_e922: f64 = (noise_metadata_schedule_67_0_e913 + noise_metadata_schedule_67_0_e921);
        (noise_metadata_schedule_67_0_e922,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_67_0_e924;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_68_0_e926: f64 = (-3.0);
            let noise_metadata_schedule_68_0_e928: f64 = (noise_metadata_schedule_68_0_e926 * w[6]);
            let noise_metadata_schedule_68_0_e930: f64 = (noise_metadata_schedule_68_0_e928 * w[254]);
            let noise_metadata_schedule_68_0_e933: f64 = (params[65] * w[4]);
            let noise_metadata_schedule_68_0_e934: f64 = (noise_metadata_schedule_68_0_e930 + noise_metadata_schedule_68_0_e933);
            let noise_metadata_schedule_68_0_e937: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_68_0_e939: f64 = (noise_metadata_schedule_68_0_e937 * params[104]);
            let noise_metadata_schedule_68_0_e940: f64 = (noise_metadata_schedule_68_0_e934 + noise_metadata_schedule_68_0_e939);
            w[13] = noise_metadata_schedule_68_0_e940;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_69_0_e943: f64 = (0.05 - w[13]);
            let noise_metadata_schedule_69_0_e945: f64 = (noise_metadata_schedule_69_0_e943 / w[6]);
            w[259] = noise_metadata_schedule_69_0_e945;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_70_0_e948: f64 = if 0.05 < w[13] { 1.0 } else { 0.0 };
            w[445] = noise_metadata_schedule_70_0_e948;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_71_0_e960,) = {
    if (w[445] != 0.0) {
        let noise_metadata_schedule_71_0_e954: f64 = (w[259]).exp();
        let noise_metadata_schedule_71_0_e955: f64 = (1.0 + noise_metadata_schedule_71_0_e954);
        let noise_metadata_schedule_71_0_e956: f64 = (noise_metadata_schedule_71_0_e955).ln();
        let noise_metadata_schedule_71_0_e957: f64 = (w[6] * noise_metadata_schedule_71_0_e956);
        let noise_metadata_schedule_71_0_e958: f64 = (w[13] + noise_metadata_schedule_71_0_e957);
        (noise_metadata_schedule_71_0_e958,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_71_0_e960;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_72_0_e974,) = {
    if (w[445] == 0.0) {
        let noise_metadata_schedule_72_0_e967: f64 = (-w[259]);
        let noise_metadata_schedule_72_0_e968: f64 = (noise_metadata_schedule_72_0_e967).exp();
        let noise_metadata_schedule_72_0_e969: f64 = (1.0 + noise_metadata_schedule_72_0_e968);
        let noise_metadata_schedule_72_0_e970: f64 = (noise_metadata_schedule_72_0_e969).ln();
        let noise_metadata_schedule_72_0_e971: f64 = (w[6] * noise_metadata_schedule_72_0_e970);
        let noise_metadata_schedule_72_0_e972: f64 = (0.05 + noise_metadata_schedule_72_0_e971);
        (noise_metadata_schedule_72_0_e972,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_72_0_e974;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_73_0_e976: f64 = (-3.0);
            let noise_metadata_schedule_73_0_e978: f64 = (noise_metadata_schedule_73_0_e976 * w[6]);
            let noise_metadata_schedule_73_0_e980: f64 = (noise_metadata_schedule_73_0_e978 * w[254]);
            let noise_metadata_schedule_73_0_e983: f64 = (params[63] * w[4]);
            let noise_metadata_schedule_73_0_e984: f64 = (noise_metadata_schedule_73_0_e980 + noise_metadata_schedule_73_0_e983);
            let noise_metadata_schedule_73_0_e987: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_73_0_e989: f64 = (noise_metadata_schedule_73_0_e987 * params[109]);
            let noise_metadata_schedule_73_0_e990: f64 = (noise_metadata_schedule_73_0_e984 + noise_metadata_schedule_73_0_e989);
            w[15] = noise_metadata_schedule_73_0_e990;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_74_0_e993: f64 = (0.05 - w[15]);
            let noise_metadata_schedule_74_0_e995: f64 = (noise_metadata_schedule_74_0_e993 / w[6]);
            w[259] = noise_metadata_schedule_74_0_e995;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_75_0_e998: f64 = if 0.05 < w[15] { 1.0 } else { 0.0 };
            w[446] = noise_metadata_schedule_75_0_e998;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_76_0_e1010,) = {
    if (w[446] != 0.0) {
        let noise_metadata_schedule_76_0_e1004: f64 = (w[259]).exp();
        let noise_metadata_schedule_76_0_e1005: f64 = (1.0 + noise_metadata_schedule_76_0_e1004);
        let noise_metadata_schedule_76_0_e1006: f64 = (noise_metadata_schedule_76_0_e1005).ln();
        let noise_metadata_schedule_76_0_e1007: f64 = (w[6] * noise_metadata_schedule_76_0_e1006);
        let noise_metadata_schedule_76_0_e1008: f64 = (w[15] + noise_metadata_schedule_76_0_e1007);
        (noise_metadata_schedule_76_0_e1008,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_76_0_e1010;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_77_0_e1024,) = {
    if (w[446] == 0.0) {
        let noise_metadata_schedule_77_0_e1017: f64 = (-w[259]);
        let noise_metadata_schedule_77_0_e1018: f64 = (noise_metadata_schedule_77_0_e1017).exp();
        let noise_metadata_schedule_77_0_e1019: f64 = (1.0 + noise_metadata_schedule_77_0_e1018);
        let noise_metadata_schedule_77_0_e1020: f64 = (noise_metadata_schedule_77_0_e1019).ln();
        let noise_metadata_schedule_77_0_e1021: f64 = (w[6] * noise_metadata_schedule_77_0_e1020);
        let noise_metadata_schedule_77_0_e1022: f64 = (0.05 + noise_metadata_schedule_77_0_e1021);
        (noise_metadata_schedule_77_0_e1022,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_77_0_e1024;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_83_0_e1076: f64 = (-3.0);
            let noise_metadata_schedule_83_0_e1078: f64 = (noise_metadata_schedule_83_0_e1076 * w[6]);
            let noise_metadata_schedule_83_0_e1080: f64 = (noise_metadata_schedule_83_0_e1078 * w[254]);
            let noise_metadata_schedule_83_0_e1083: f64 = (params[70] * w[4]);
            let noise_metadata_schedule_83_0_e1084: f64 = (noise_metadata_schedule_83_0_e1080 + noise_metadata_schedule_83_0_e1083);
            let noise_metadata_schedule_83_0_e1087: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_83_0_e1089: f64 = (noise_metadata_schedule_83_0_e1087 * params[109]);
            let noise_metadata_schedule_83_0_e1090: f64 = (noise_metadata_schedule_83_0_e1084 + noise_metadata_schedule_83_0_e1089);
            w[18] = noise_metadata_schedule_83_0_e1090;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_84_0_e1093: f64 = (0.05 - w[18]);
            let noise_metadata_schedule_84_0_e1095: f64 = (noise_metadata_schedule_84_0_e1093 / w[6]);
            w[259] = noise_metadata_schedule_84_0_e1095;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_85_0_e1098: f64 = if 0.05 < w[18] { 1.0 } else { 0.0 };
            w[448] = noise_metadata_schedule_85_0_e1098;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_86_0_e1110,) = {
    if (w[448] != 0.0) {
        let noise_metadata_schedule_86_0_e1104: f64 = (w[259]).exp();
        let noise_metadata_schedule_86_0_e1105: f64 = (1.0 + noise_metadata_schedule_86_0_e1104);
        let noise_metadata_schedule_86_0_e1106: f64 = (noise_metadata_schedule_86_0_e1105).ln();
        let noise_metadata_schedule_86_0_e1107: f64 = (w[6] * noise_metadata_schedule_86_0_e1106);
        let noise_metadata_schedule_86_0_e1108: f64 = (w[18] + noise_metadata_schedule_86_0_e1107);
        (noise_metadata_schedule_86_0_e1108,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_86_0_e1110;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_87_0_e1124,) = {
    if (w[448] == 0.0) {
        let noise_metadata_schedule_87_0_e1117: f64 = (-w[259]);
        let noise_metadata_schedule_87_0_e1118: f64 = (noise_metadata_schedule_87_0_e1117).exp();
        let noise_metadata_schedule_87_0_e1119: f64 = (1.0 + noise_metadata_schedule_87_0_e1118);
        let noise_metadata_schedule_87_0_e1120: f64 = (noise_metadata_schedule_87_0_e1119).ln();
        let noise_metadata_schedule_87_0_e1121: f64 = (w[6] * noise_metadata_schedule_87_0_e1120);
        let noise_metadata_schedule_87_0_e1122: f64 = (0.05 + noise_metadata_schedule_87_0_e1121);
        (noise_metadata_schedule_87_0_e1122,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_87_0_e1124;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_88_0_e1126: f64 = (-3.0);
            let noise_metadata_schedule_88_0_e1128: f64 = (noise_metadata_schedule_88_0_e1126 * w[6]);
            let noise_metadata_schedule_88_0_e1130: f64 = (noise_metadata_schedule_88_0_e1128 * w[254]);
            let noise_metadata_schedule_88_0_e1133: f64 = (w[75] * w[4]);
            let noise_metadata_schedule_88_0_e1134: f64 = (noise_metadata_schedule_88_0_e1130 + noise_metadata_schedule_88_0_e1133);
            let noise_metadata_schedule_88_0_e1137: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_88_0_e1139: f64 = (noise_metadata_schedule_88_0_e1137 * params[109]);
            let noise_metadata_schedule_88_0_e1140: f64 = (noise_metadata_schedule_88_0_e1134 + noise_metadata_schedule_88_0_e1139);
            w[20] = noise_metadata_schedule_88_0_e1140;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_89_0_e1143: f64 = (0.05 - w[20]);
            let noise_metadata_schedule_89_0_e1145: f64 = (noise_metadata_schedule_89_0_e1143 / w[6]);
            w[259] = noise_metadata_schedule_89_0_e1145;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_90_0_e1148: f64 = if 0.05 < w[20] { 1.0 } else { 0.0 };
            w[449] = noise_metadata_schedule_90_0_e1148;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_91_0_e1160,) = {
    if (w[449] != 0.0) {
        let noise_metadata_schedule_91_0_e1154: f64 = (w[259]).exp();
        let noise_metadata_schedule_91_0_e1155: f64 = (1.0 + noise_metadata_schedule_91_0_e1154);
        let noise_metadata_schedule_91_0_e1156: f64 = (noise_metadata_schedule_91_0_e1155).ln();
        let noise_metadata_schedule_91_0_e1157: f64 = (w[6] * noise_metadata_schedule_91_0_e1156);
        let noise_metadata_schedule_91_0_e1158: f64 = (w[20] + noise_metadata_schedule_91_0_e1157);
        (noise_metadata_schedule_91_0_e1158,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_91_0_e1160;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_92_0_e1174,) = {
    if (w[449] == 0.0) {
        let noise_metadata_schedule_92_0_e1167: f64 = (-w[259]);
        let noise_metadata_schedule_92_0_e1168: f64 = (noise_metadata_schedule_92_0_e1167).exp();
        let noise_metadata_schedule_92_0_e1169: f64 = (1.0 + noise_metadata_schedule_92_0_e1168);
        let noise_metadata_schedule_92_0_e1170: f64 = (noise_metadata_schedule_92_0_e1169).ln();
        let noise_metadata_schedule_92_0_e1171: f64 = (w[6] * noise_metadata_schedule_92_0_e1170);
        let noise_metadata_schedule_92_0_e1172: f64 = (0.05 + noise_metadata_schedule_92_0_e1171);
        (noise_metadata_schedule_92_0_e1172,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_92_0_e1174;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_93_0_e1176: f64 = (-3.0);
            let noise_metadata_schedule_93_0_e1178: f64 = (noise_metadata_schedule_93_0_e1176 * w[6]);
            let noise_metadata_schedule_93_0_e1180: f64 = (noise_metadata_schedule_93_0_e1178 * w[254]);
            let noise_metadata_schedule_93_0_e1183: f64 = (params[26] * w[4]);
            let noise_metadata_schedule_93_0_e1184: f64 = (noise_metadata_schedule_93_0_e1180 + noise_metadata_schedule_93_0_e1183);
            let noise_metadata_schedule_93_0_e1187: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_93_0_e1189: f64 = (noise_metadata_schedule_93_0_e1187 * params[108]);
            let noise_metadata_schedule_93_0_e1190: f64 = (noise_metadata_schedule_93_0_e1184 + noise_metadata_schedule_93_0_e1189);
            w[56] = noise_metadata_schedule_93_0_e1190;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_94_0_e1193: f64 = (0.05 - w[56]);
            let noise_metadata_schedule_94_0_e1195: f64 = (noise_metadata_schedule_94_0_e1193 / w[6]);
            w[259] = noise_metadata_schedule_94_0_e1195;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_95_0_e1198: f64 = if 0.05 < w[56] { 1.0 } else { 0.0 };
            w[450] = noise_metadata_schedule_95_0_e1198;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_96_0_e1210,) = {
    if (w[450] != 0.0) {
        let noise_metadata_schedule_96_0_e1204: f64 = (w[259]).exp();
        let noise_metadata_schedule_96_0_e1205: f64 = (1.0 + noise_metadata_schedule_96_0_e1204);
        let noise_metadata_schedule_96_0_e1206: f64 = (noise_metadata_schedule_96_0_e1205).ln();
        let noise_metadata_schedule_96_0_e1207: f64 = (w[6] * noise_metadata_schedule_96_0_e1206);
        let noise_metadata_schedule_96_0_e1208: f64 = (w[56] + noise_metadata_schedule_96_0_e1207);
        (noise_metadata_schedule_96_0_e1208,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_96_0_e1210;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_97_0_e1224,) = {
    if (w[450] == 0.0) {
        let noise_metadata_schedule_97_0_e1217: f64 = (-w[259]);
        let noise_metadata_schedule_97_0_e1218: f64 = (noise_metadata_schedule_97_0_e1217).exp();
        let noise_metadata_schedule_97_0_e1219: f64 = (1.0 + noise_metadata_schedule_97_0_e1218);
        let noise_metadata_schedule_97_0_e1220: f64 = (noise_metadata_schedule_97_0_e1219).ln();
        let noise_metadata_schedule_97_0_e1221: f64 = (w[6] * noise_metadata_schedule_97_0_e1220);
        let noise_metadata_schedule_97_0_e1222: f64 = (0.05 + noise_metadata_schedule_97_0_e1221);
        (noise_metadata_schedule_97_0_e1222,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_97_0_e1224;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_98_0_e1227: f64 = (1.0 / w[14]);
            w[65] = noise_metadata_schedule_98_0_e1227;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_99_0_e1230: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_99_0_e1230;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_100_0_e1233: f64 = (params[65] * w[65]);
            let noise_metadata_schedule_100_0_e1235: f64 = (noise_metadata_schedule_100_0_e1233).powf(params[66]);
            w[73] = noise_metadata_schedule_100_0_e1235;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_101_0_e1238: f64 = (w[75] * w[67]);
            let noise_metadata_schedule_101_0_e1240: f64 = (noise_metadata_schedule_101_0_e1238).powf(w[76]);
            w[90] = noise_metadata_schedule_101_0_e1240;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_103_0_e1246: f64 = (1.0 - params[74]);
            let noise_metadata_schedule_103_0_e1249: f64 = (params[70] / w[17]);
            let noise_metadata_schedule_103_0_e1251: f64 = (noise_metadata_schedule_103_0_e1249).powf(params[71]);
            let noise_metadata_schedule_103_0_e1252: f64 = (noise_metadata_schedule_103_0_e1246 * noise_metadata_schedule_103_0_e1251);
            let noise_metadata_schedule_103_0_e1254: f64 = (noise_metadata_schedule_103_0_e1252 + params[74]);
            w[26] = noise_metadata_schedule_103_0_e1254;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_104_0_e1257: f64 = (1.0 / w[26]);
            w[27] = noise_metadata_schedule_104_0_e1257;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_106_0_e1263: f64 = (params[74] * w[27]);
            w[25] = noise_metadata_schedule_106_0_e1263;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_107_0_e1267: f64 = (w[254] * params[96]);
            let noise_metadata_schedule_107_0_e1268: f64 = (noise_metadata_schedule_107_0_e1267).exp();
            let noise_metadata_schedule_107_0_e1269: f64 = (params[53] * noise_metadata_schedule_107_0_e1268);
            w[28] = noise_metadata_schedule_107_0_e1269;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_108_0_e1272: f64 = if w[28] < w[316] { 1.0 } else { 0.0 };
            w[451] = noise_metadata_schedule_108_0_e1272;
        }
        if (active[0] & 0xa) != 0 {
            let (noise_metadata_schedule_109_0_e1276,) = {
    if (w[451] != 0.0) {
        (w[316],)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_109_0_e1276;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_110_0_e1281: f64 = (params[97] - params[95]);
            let noise_metadata_schedule_110_0_e1282: f64 = (w[254] * noise_metadata_schedule_110_0_e1281);
            let noise_metadata_schedule_110_0_e1283: f64 = (noise_metadata_schedule_110_0_e1282).exp();
            let noise_metadata_schedule_110_0_e1284: f64 = (params[55] * noise_metadata_schedule_110_0_e1283);
            w[29] = noise_metadata_schedule_110_0_e1284;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_111_0_e1288: f64 = (w[254] * params[100]);
            let noise_metadata_schedule_111_0_e1289: f64 = (noise_metadata_schedule_111_0_e1288).exp();
            let noise_metadata_schedule_111_0_e1290: f64 = (params[54] * noise_metadata_schedule_111_0_e1289);
            w[30] = noise_metadata_schedule_111_0_e1290;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_112_0_e1293: f64 = if w[30] < w[316] { 1.0 } else { 0.0 };
            w[452] = noise_metadata_schedule_112_0_e1293;
        }
        if (active[0] & 0x12) != 0 {
            let (noise_metadata_schedule_113_0_e1297,) = {
    if (w[452] != 0.0) {
        (w[316],)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_113_0_e1297;
        }
        if (active[0] & 0x153fe00) != 0 {
            let noise_metadata_schedule_114_0_e1301: f64 = (w[254] * params[101]);
            let noise_metadata_schedule_114_0_e1302: f64 = (noise_metadata_schedule_114_0_e1301).exp();
            let noise_metadata_schedule_114_0_e1303: f64 = (params[56] * noise_metadata_schedule_114_0_e1302);
            w[32] = noise_metadata_schedule_114_0_e1303;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_115_0_e1307: f64 = (w[254] * params[103]);
            let noise_metadata_schedule_115_0_e1308: f64 = (noise_metadata_schedule_115_0_e1307).exp();
            let noise_metadata_schedule_115_0_e1309: f64 = (params[57] * noise_metadata_schedule_115_0_e1308);
            w[33] = noise_metadata_schedule_115_0_e1309;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_116_0_e1313: f64 = (w[254] * params[103]);
            let noise_metadata_schedule_116_0_e1314: f64 = (noise_metadata_schedule_116_0_e1313).exp();
            let noise_metadata_schedule_116_0_e1315: f64 = (params[58] * noise_metadata_schedule_116_0_e1314);
            w[34] = noise_metadata_schedule_116_0_e1315;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_117_0_e1319: f64 = (w[254] * params[98]);
            let noise_metadata_schedule_117_0_e1320: f64 = (noise_metadata_schedule_117_0_e1319).exp();
            let noise_metadata_schedule_117_0_e1321: f64 = (params[59] * noise_metadata_schedule_117_0_e1320);
            w[31] = noise_metadata_schedule_117_0_e1321;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_118_0_e1324: f64 = if params[121] != 0.0 { 1.0 } else { 0.0 };
            w[453] = noise_metadata_schedule_118_0_e1324;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_119_0_e1334,) = {
    if (w[453] != 0.0) {
        let noise_metadata_schedule_119_0_e1330: f64 = (w[12] * params[121]);
        let noise_metadata_schedule_119_0_e1331: f64 = (1.0 + noise_metadata_schedule_119_0_e1330);
        let noise_metadata_schedule_119_0_e1332: f64 = (params[9] * noise_metadata_schedule_119_0_e1331);
        (noise_metadata_schedule_119_0_e1332,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_119_0_e1334;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_120_0_e1342,) = {
    if (w[453] != 0.0) {
        let noise_metadata_schedule_120_0_e1338: f64 = (w[50] - 1.0);
        let noise_metadata_schedule_120_0_e1340: f64 = (noise_metadata_schedule_120_0_e1338 / w[52]);
        (noise_metadata_schedule_120_0_e1340,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_120_0_e1342;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_121_0_e1345: f64 = if w[50] < 1.0 { 1.0 } else { 0.0 };
            w[454] = noise_metadata_schedule_121_0_e1345;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_122_0_e1359,) = {
    if ((w[453] != 0.0) && (w[454] != 0.0)) {
        let noise_metadata_schedule_122_0_e1353: f64 = (w[259]).exp();
        let noise_metadata_schedule_122_0_e1354: f64 = (1.0 + noise_metadata_schedule_122_0_e1353);
        let noise_metadata_schedule_122_0_e1355: f64 = (noise_metadata_schedule_122_0_e1354).ln();
        let noise_metadata_schedule_122_0_e1356: f64 = (w[52] * noise_metadata_schedule_122_0_e1355);
        let noise_metadata_schedule_122_0_e1357: f64 = (1.0 + noise_metadata_schedule_122_0_e1356);
        (noise_metadata_schedule_122_0_e1357,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_122_0_e1359;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_123_0_e1375,) = {
    if ((w[453] != 0.0) && (w[454] == 0.0)) {
        let noise_metadata_schedule_123_0_e1368: f64 = (-w[259]);
        let noise_metadata_schedule_123_0_e1369: f64 = (noise_metadata_schedule_123_0_e1368).exp();
        let noise_metadata_schedule_123_0_e1370: f64 = (1.0 + noise_metadata_schedule_123_0_e1369);
        let noise_metadata_schedule_123_0_e1371: f64 = (noise_metadata_schedule_123_0_e1370).ln();
        let noise_metadata_schedule_123_0_e1372: f64 = (w[52] * noise_metadata_schedule_123_0_e1371);
        let noise_metadata_schedule_123_0_e1373: f64 = (w[50] + noise_metadata_schedule_123_0_e1372);
        (noise_metadata_schedule_123_0_e1373,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_123_0_e1375;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_124_0_e1383,) = {
    if (w[453] != 0.0) {
        let noise_metadata_schedule_124_0_e1380: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_124_0_e1381: f64 = (w[50] - noise_metadata_schedule_124_0_e1380);
        (noise_metadata_schedule_124_0_e1381,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_124_0_e1383;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_125_0_e1388,) = {
    if (w[453] == 0.0) {
        (params[9],)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_125_0_e1388;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_126_0_e1391: f64 = if params[122] != 0.0 { 1.0 } else { 0.0 };
            w[455] = noise_metadata_schedule_126_0_e1391;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_127_0_e1401,) = {
    if (w[455] != 0.0) {
        let noise_metadata_schedule_127_0_e1397: f64 = (w[12] * params[122]);
        let noise_metadata_schedule_127_0_e1398: f64 = (1.0 + noise_metadata_schedule_127_0_e1397);
        let noise_metadata_schedule_127_0_e1399: f64 = (params[10] * noise_metadata_schedule_127_0_e1398);
        (noise_metadata_schedule_127_0_e1399,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_127_0_e1401;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_128_0_e1409,) = {
    if (w[455] != 0.0) {
        let noise_metadata_schedule_128_0_e1405: f64 = (w[51] - 1.0);
        let noise_metadata_schedule_128_0_e1407: f64 = (noise_metadata_schedule_128_0_e1405 / w[52]);
        (noise_metadata_schedule_128_0_e1407,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_128_0_e1409;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_129_0_e1412: f64 = if w[51] < 1.0 { 1.0 } else { 0.0 };
            w[456] = noise_metadata_schedule_129_0_e1412;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_130_0_e1426,) = {
    if ((w[455] != 0.0) && (w[456] != 0.0)) {
        let noise_metadata_schedule_130_0_e1420: f64 = (w[259]).exp();
        let noise_metadata_schedule_130_0_e1421: f64 = (1.0 + noise_metadata_schedule_130_0_e1420);
        let noise_metadata_schedule_130_0_e1422: f64 = (noise_metadata_schedule_130_0_e1421).ln();
        let noise_metadata_schedule_130_0_e1423: f64 = (w[52] * noise_metadata_schedule_130_0_e1422);
        let noise_metadata_schedule_130_0_e1424: f64 = (1.0 + noise_metadata_schedule_130_0_e1423);
        (noise_metadata_schedule_130_0_e1424,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_130_0_e1426;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_131_0_e1442,) = {
    if ((w[455] != 0.0) && (w[456] == 0.0)) {
        let noise_metadata_schedule_131_0_e1435: f64 = (-w[259]);
        let noise_metadata_schedule_131_0_e1436: f64 = (noise_metadata_schedule_131_0_e1435).exp();
        let noise_metadata_schedule_131_0_e1437: f64 = (1.0 + noise_metadata_schedule_131_0_e1436);
        let noise_metadata_schedule_131_0_e1438: f64 = (noise_metadata_schedule_131_0_e1437).ln();
        let noise_metadata_schedule_131_0_e1439: f64 = (w[52] * noise_metadata_schedule_131_0_e1438);
        let noise_metadata_schedule_131_0_e1440: f64 = (w[51] + noise_metadata_schedule_131_0_e1439);
        (noise_metadata_schedule_131_0_e1440,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_131_0_e1442;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_132_0_e1450,) = {
    if (w[455] != 0.0) {
        let noise_metadata_schedule_132_0_e1447: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_132_0_e1448: f64 = (w[51] - noise_metadata_schedule_132_0_e1447);
        (noise_metadata_schedule_132_0_e1448,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_132_0_e1450;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_133_0_e1455,) = {
    if (w[455] == 0.0) {
        (params[10],)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_133_0_e1455;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_134_0_e1460: f64 = (params[123] * w[12]);
            let noise_metadata_schedule_134_0_e1461: f64 = (1.0 + noise_metadata_schedule_134_0_e1460);
            let noise_metadata_schedule_134_0_e1462: f64 = (params[42] * noise_metadata_schedule_134_0_e1461);
            w[311] = noise_metadata_schedule_134_0_e1462;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_135_0_e1465: f64 = (w[312] * w[312]);
            w[261] = noise_metadata_schedule_135_0_e1465;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_136_0_e1468: f64 = (w[311] * w[311]);
            w[262] = noise_metadata_schedule_136_0_e1468;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_137_0_e1471: f64 = if w[311] < 0.0 { 1.0 } else { 0.0 };
            w[457] = noise_metadata_schedule_137_0_e1471;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_138_0_e1484,) = {
    if (w[457] != 0.0) {
        let noise_metadata_schedule_138_0_e1475: f64 = (0.5 * w[261]);
        let noise_metadata_schedule_138_0_e1478: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_138_0_e1479: f64 = (noise_metadata_schedule_138_0_e1478).sqrt();
        let noise_metadata_schedule_138_0_e1481: f64 = (noise_metadata_schedule_138_0_e1479 - w[311]);
        let noise_metadata_schedule_138_0_e1482: f64 = (noise_metadata_schedule_138_0_e1475 / noise_metadata_schedule_138_0_e1481);
        (noise_metadata_schedule_138_0_e1482,)
    } else {
        (w[310],)
    }
};
            w[310] = noise_metadata_schedule_138_0_e1484;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_139_0_e1496,) = {
    if (w[457] == 0.0) {
        let noise_metadata_schedule_139_0_e1490: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_139_0_e1491: f64 = (noise_metadata_schedule_139_0_e1490).sqrt();
        let noise_metadata_schedule_139_0_e1493: f64 = (noise_metadata_schedule_139_0_e1491 + w[311]);
        let noise_metadata_schedule_139_0_e1494: f64 = (0.5 * noise_metadata_schedule_139_0_e1493);
        (noise_metadata_schedule_139_0_e1494,)
    } else {
        (w[310],)
    }
};
            w[310] = noise_metadata_schedule_139_0_e1496;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_140_0_e1501: f64 = (4.0 - params[97]);
            let noise_metadata_schedule_140_0_e1503: f64 = (noise_metadata_schedule_140_0_e1501 - params[95]);
            let noise_metadata_schedule_140_0_e1505: f64 = (noise_metadata_schedule_140_0_e1503 + params[120]);
            let noise_metadata_schedule_140_0_e1506: f64 = (w[254] * noise_metadata_schedule_140_0_e1505);
            let noise_metadata_schedule_140_0_e1508: f64 = (noise_metadata_schedule_140_0_e1506 / w[48]);
            let noise_metadata_schedule_140_0_e1509: f64 = (noise_metadata_schedule_140_0_e1508).exp();
            let noise_metadata_schedule_140_0_e1510: f64 = (params[8] * noise_metadata_schedule_140_0_e1509);
            let noise_metadata_schedule_140_0_e1512: f64 = (-params[104]);
            let noise_metadata_schedule_140_0_e1514: f64 = (noise_metadata_schedule_140_0_e1512 * w[10]);
            let noise_metadata_schedule_140_0_e1516: f64 = (noise_metadata_schedule_140_0_e1514 / w[48]);
            let noise_metadata_schedule_140_0_e1517: f64 = (noise_metadata_schedule_140_0_e1516).exp();
            let noise_metadata_schedule_140_0_e1518: f64 = (noise_metadata_schedule_140_0_e1510 * noise_metadata_schedule_140_0_e1517);
            w[35] = noise_metadata_schedule_140_0_e1518;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_141_0_e1523: f64 = (1.0 - params[97]);
            let noise_metadata_schedule_141_0_e1524: f64 = (w[254] * noise_metadata_schedule_141_0_e1523);
            let noise_metadata_schedule_141_0_e1525: f64 = (noise_metadata_schedule_141_0_e1524).exp();
            let noise_metadata_schedule_141_0_e1526: f64 = (params[11] * noise_metadata_schedule_141_0_e1525);
            w[36] = noise_metadata_schedule_141_0_e1526;
        }
        if (active[0] & 0x7800) != 0 {
            let noise_metadata_schedule_142_0_e1531: f64 = (1.0 - params[102]);
            let noise_metadata_schedule_142_0_e1532: f64 = (w[254] * noise_metadata_schedule_142_0_e1531);
            let noise_metadata_schedule_142_0_e1533: f64 = (noise_metadata_schedule_142_0_e1532).exp();
            let noise_metadata_schedule_142_0_e1534: f64 = (params[29] * noise_metadata_schedule_142_0_e1533);
            w[37] = noise_metadata_schedule_142_0_e1534;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_143_0_e1540: f64 = (2.0 * params[20]);
            let noise_metadata_schedule_143_0_e1541: f64 = (6.0 - noise_metadata_schedule_143_0_e1540);
            let noise_metadata_schedule_143_0_e1542: f64 = (w[254] * noise_metadata_schedule_143_0_e1541);
            let noise_metadata_schedule_143_0_e1543: f64 = (noise_metadata_schedule_143_0_e1542).exp();
            let noise_metadata_schedule_143_0_e1544: f64 = (params[19] * noise_metadata_schedule_143_0_e1543);
            let noise_metadata_schedule_143_0_e1546: f64 = (-params[112]);
            let noise_metadata_schedule_143_0_e1548: f64 = (noise_metadata_schedule_143_0_e1546 * w[10]);
            let noise_metadata_schedule_143_0_e1550: f64 = (noise_metadata_schedule_143_0_e1548 / params[20]);
            let noise_metadata_schedule_143_0_e1551: f64 = (noise_metadata_schedule_143_0_e1550).exp();
            let noise_metadata_schedule_143_0_e1552: f64 = (noise_metadata_schedule_143_0_e1544 * noise_metadata_schedule_143_0_e1551);
            w[38] = noise_metadata_schedule_143_0_e1552;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_144_0_e1558: f64 = (2.0 * params[31]);
            let noise_metadata_schedule_144_0_e1559: f64 = (6.0 - noise_metadata_schedule_144_0_e1558);
            let noise_metadata_schedule_144_0_e1560: f64 = (w[254] * noise_metadata_schedule_144_0_e1559);
            let noise_metadata_schedule_144_0_e1561: f64 = (noise_metadata_schedule_144_0_e1560).exp();
            let noise_metadata_schedule_144_0_e1562: f64 = (params[30] * noise_metadata_schedule_144_0_e1561);
            let noise_metadata_schedule_144_0_e1564: f64 = (-params[109]);
            let noise_metadata_schedule_144_0_e1566: f64 = (noise_metadata_schedule_144_0_e1564 * w[10]);
            let noise_metadata_schedule_144_0_e1568: f64 = (noise_metadata_schedule_144_0_e1566 / params[31]);
            let noise_metadata_schedule_144_0_e1569: f64 = (noise_metadata_schedule_144_0_e1568).exp();
            let noise_metadata_schedule_144_0_e1570: f64 = (noise_metadata_schedule_144_0_e1562 * noise_metadata_schedule_144_0_e1569);
            w[39] = noise_metadata_schedule_144_0_e1570;
        }
        if (active[0] & 0x46) != 0 {
            let noise_metadata_schedule_145_0_e1575: f64 = (4.0 - params[96]);
            let noise_metadata_schedule_145_0_e1577: f64 = (noise_metadata_schedule_145_0_e1575 + params[120]);
            let noise_metadata_schedule_145_0_e1578: f64 = (w[254] * noise_metadata_schedule_145_0_e1577);
            let noise_metadata_schedule_145_0_e1580: f64 = (noise_metadata_schedule_145_0_e1578 / params[16]);
            let noise_metadata_schedule_145_0_e1581: f64 = (noise_metadata_schedule_145_0_e1580).exp();
            let noise_metadata_schedule_145_0_e1582: f64 = (params[15] * noise_metadata_schedule_145_0_e1581);
            let noise_metadata_schedule_145_0_e1584: f64 = (-params[110]);
            let noise_metadata_schedule_145_0_e1586: f64 = (noise_metadata_schedule_145_0_e1584 * w[10]);
            let noise_metadata_schedule_145_0_e1588: f64 = (noise_metadata_schedule_145_0_e1586 / params[16]);
            let noise_metadata_schedule_145_0_e1589: f64 = (noise_metadata_schedule_145_0_e1588).exp();
            let noise_metadata_schedule_145_0_e1590: f64 = (noise_metadata_schedule_145_0_e1582 * noise_metadata_schedule_145_0_e1589);
            w[42] = noise_metadata_schedule_145_0_e1590;
        }
        if (active[0] & 0x140) != 0 {
            let noise_metadata_schedule_146_0_e1595: f64 = (4.0 - params[96]);
            let noise_metadata_schedule_146_0_e1597: f64 = (noise_metadata_schedule_146_0_e1595 + params[120]);
            let noise_metadata_schedule_146_0_e1598: f64 = (w[254] * noise_metadata_schedule_146_0_e1597);
            let noise_metadata_schedule_146_0_e1600: f64 = (noise_metadata_schedule_146_0_e1598 / params[18]);
            let noise_metadata_schedule_146_0_e1601: f64 = (noise_metadata_schedule_146_0_e1600).exp();
            let noise_metadata_schedule_146_0_e1602: f64 = (params[17] * noise_metadata_schedule_146_0_e1601);
            let noise_metadata_schedule_146_0_e1604: f64 = (-params[110]);
            let noise_metadata_schedule_146_0_e1606: f64 = (noise_metadata_schedule_146_0_e1604 * w[10]);
            let noise_metadata_schedule_146_0_e1608: f64 = (noise_metadata_schedule_146_0_e1606 / params[18]);
            let noise_metadata_schedule_146_0_e1609: f64 = (noise_metadata_schedule_146_0_e1608).exp();
            let noise_metadata_schedule_146_0_e1610: f64 = (noise_metadata_schedule_146_0_e1602 * noise_metadata_schedule_146_0_e1609);
            w[44] = noise_metadata_schedule_146_0_e1610;
        }
        if (active[0] & 0x144) != 0 {
            let noise_metadata_schedule_147_0_e1613: f64 = if params[23] == 1.0 { 1.0 } else { 0.0 };
            w[458] = noise_metadata_schedule_147_0_e1613;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_148_0_e1625,) = {
    if (w[458] != 0.0) {
        let noise_metadata_schedule_148_0_e1617: f64 = (-params[106]);
        let noise_metadata_schedule_148_0_e1619: f64 = (noise_metadata_schedule_148_0_e1617 * w[10]);
        let noise_metadata_schedule_148_0_e1621: f64 = (noise_metadata_schedule_148_0_e1619 / params[16]);
        let noise_metadata_schedule_148_0_e1622: f64 = (noise_metadata_schedule_148_0_e1621).exp();
        let noise_metadata_schedule_148_0_e1623: f64 = (params[24] * noise_metadata_schedule_148_0_e1622);
        (noise_metadata_schedule_148_0_e1623,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_148_0_e1625;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_149_0_e1635,) = {
    if (w[458] != 0.0) {
        let noise_metadata_schedule_149_0_e1629: f64 = (-params[105]);
        let noise_metadata_schedule_149_0_e1631: f64 = (noise_metadata_schedule_149_0_e1629 * w[10]);
        let noise_metadata_schedule_149_0_e1632: f64 = (noise_metadata_schedule_149_0_e1631).exp();
        let noise_metadata_schedule_149_0_e1633: f64 = (params[27] * noise_metadata_schedule_149_0_e1632);
        (noise_metadata_schedule_149_0_e1633,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_149_0_e1635;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_150_0_e1647,) = {
    if (w[458] != 0.0) {
        let noise_metadata_schedule_150_0_e1639: f64 = (-params[107]);
        let noise_metadata_schedule_150_0_e1641: f64 = (noise_metadata_schedule_150_0_e1639 * w[10]);
        let noise_metadata_schedule_150_0_e1643: f64 = (noise_metadata_schedule_150_0_e1641 / params[18]);
        let noise_metadata_schedule_150_0_e1644: f64 = (noise_metadata_schedule_150_0_e1643).exp();
        let noise_metadata_schedule_150_0_e1645: f64 = (params[25] * noise_metadata_schedule_150_0_e1644);
        (noise_metadata_schedule_150_0_e1645,)
    } else {
        (w[45],)
    }
};
            w[45] = noise_metadata_schedule_150_0_e1647;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_151_0_e1652: f64 = (4.0 - params[102]);
            let noise_metadata_schedule_151_0_e1654: f64 = (noise_metadata_schedule_151_0_e1652 + params[120]);
            let noise_metadata_schedule_151_0_e1655: f64 = (w[254] * noise_metadata_schedule_151_0_e1654);
            let noise_metadata_schedule_151_0_e1656: f64 = (noise_metadata_schedule_151_0_e1655).exp();
            let noise_metadata_schedule_151_0_e1657: f64 = (params[28] * noise_metadata_schedule_151_0_e1656);
            let noise_metadata_schedule_151_0_e1659: f64 = (-params[111]);
            let noise_metadata_schedule_151_0_e1661: f64 = (noise_metadata_schedule_151_0_e1659 * w[10]);
            let noise_metadata_schedule_151_0_e1662: f64 = (noise_metadata_schedule_151_0_e1661).exp();
            let noise_metadata_schedule_151_0_e1663: f64 = (noise_metadata_schedule_151_0_e1657 * noise_metadata_schedule_151_0_e1662);
            w[43] = noise_metadata_schedule_151_0_e1663;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_152_0_e1669: f64 = (2.0 * params[22]);
            let noise_metadata_schedule_152_0_e1670: f64 = (6.0 - noise_metadata_schedule_152_0_e1669);
            let noise_metadata_schedule_152_0_e1671: f64 = (w[254] * noise_metadata_schedule_152_0_e1670);
            let noise_metadata_schedule_152_0_e1672: f64 = (noise_metadata_schedule_152_0_e1671).exp();
            let noise_metadata_schedule_152_0_e1673: f64 = (params[21] * noise_metadata_schedule_152_0_e1672);
            let noise_metadata_schedule_152_0_e1675: f64 = (-params[112]);
            let noise_metadata_schedule_152_0_e1677: f64 = (noise_metadata_schedule_152_0_e1675 * w[10]);
            let noise_metadata_schedule_152_0_e1679: f64 = (noise_metadata_schedule_152_0_e1677 / params[22]);
            let noise_metadata_schedule_152_0_e1680: f64 = (noise_metadata_schedule_152_0_e1679).exp();
            let noise_metadata_schedule_152_0_e1681: f64 = (noise_metadata_schedule_152_0_e1673 * noise_metadata_schedule_152_0_e1680);
            w[46] = noise_metadata_schedule_152_0_e1681;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_153_0_e1686: f64 = (4.0 / params[133]);
            let noise_metadata_schedule_153_0_e1687: f64 = (w[254] * noise_metadata_schedule_153_0_e1686);
            let noise_metadata_schedule_153_0_e1688: f64 = (noise_metadata_schedule_153_0_e1687).exp();
            let noise_metadata_schedule_153_0_e1689: f64 = (params[132] * noise_metadata_schedule_153_0_e1688);
            let noise_metadata_schedule_153_0_e1691: f64 = (-params[112]);
            let noise_metadata_schedule_153_0_e1693: f64 = (noise_metadata_schedule_153_0_e1691 * w[10]);
            let noise_metadata_schedule_153_0_e1695: f64 = (noise_metadata_schedule_153_0_e1693 / params[133]);
            let noise_metadata_schedule_153_0_e1696: f64 = (noise_metadata_schedule_153_0_e1695).exp();
            let noise_metadata_schedule_153_0_e1697: f64 = (noise_metadata_schedule_153_0_e1689 * noise_metadata_schedule_153_0_e1696);
            w[47] = noise_metadata_schedule_153_0_e1697;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_154_0_e1700: f64 = (w[4]).sqrt();
            let noise_metadata_schedule_154_0_e1701: f64 = (params[138] * noise_metadata_schedule_154_0_e1700);
            let noise_metadata_schedule_154_0_e1704: f64 = (params[140] * w[12]);
            let noise_metadata_schedule_154_0_e1705: f64 = (noise_metadata_schedule_154_0_e1704).exp();
            let noise_metadata_schedule_154_0_e1706: f64 = (noise_metadata_schedule_154_0_e1701 * noise_metadata_schedule_154_0_e1705);
            w[325] = noise_metadata_schedule_154_0_e1706;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_155_0_e1709: f64 = (w[70] * w[72]);
            let noise_metadata_schedule_155_0_e1711: f64 = (-0.5);
            let noise_metadata_schedule_155_0_e1712: f64 = (noise_metadata_schedule_155_0_e1709).powf(noise_metadata_schedule_155_0_e1711);
            w[255] = noise_metadata_schedule_155_0_e1712;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_156_0_e1715: f64 = (1.0 / w[73]);
            w[256] = noise_metadata_schedule_156_0_e1715;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_157_0_e1718: f64 = (params[34] * w[70]);
            let noise_metadata_schedule_157_0_e1720: f64 = (noise_metadata_schedule_157_0_e1718 * w[70]);
            let noise_metadata_schedule_157_0_e1722: f64 = (noise_metadata_schedule_157_0_e1720 * w[255]);
            let noise_metadata_schedule_157_0_e1724: f64 = (noise_metadata_schedule_157_0_e1722 * w[256]);
            let noise_metadata_schedule_157_0_e1726: f64 = (noise_metadata_schedule_157_0_e1724 * params[65]);
            let noise_metadata_schedule_157_0_e1728: f64 = (noise_metadata_schedule_157_0_e1726 * w[65]);
            let noise_metadata_schedule_157_0_e1730: f64 = (noise_metadata_schedule_157_0_e1728 * w[72]);
            let noise_metadata_schedule_157_0_e1732: f64 = (noise_metadata_schedule_157_0_e1730 * w[72]);
            w[61] = noise_metadata_schedule_157_0_e1732;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_158_0_e1735: f64 = (params[33] * w[255]);
            let noise_metadata_schedule_158_0_e1737: f64 = (noise_metadata_schedule_158_0_e1735 * w[14]);
            let noise_metadata_schedule_158_0_e1739: f64 = (noise_metadata_schedule_158_0_e1737 * w[14]);
            let noise_metadata_schedule_158_0_e1741: f64 = (noise_metadata_schedule_158_0_e1739 * w[64]);
            let noise_metadata_schedule_158_0_e1743: f64 = (noise_metadata_schedule_158_0_e1741 * w[64]);
            let noise_metadata_schedule_158_0_e1745: f64 = (noise_metadata_schedule_158_0_e1743 * w[73]);
            let noise_metadata_schedule_158_0_e1748: f64 = (params[34] - w[61]);
            let noise_metadata_schedule_158_0_e1749: f64 = (noise_metadata_schedule_158_0_e1748).exp();
            let noise_metadata_schedule_158_0_e1750: f64 = (noise_metadata_schedule_158_0_e1745 * noise_metadata_schedule_158_0_e1749);
            w[58] = noise_metadata_schedule_158_0_e1750;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_159_0_e1753: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_159_0_e1753;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_160_0_e1756: f64 = (w[85] * w[86]);
            let noise_metadata_schedule_160_0_e1758: f64 = (-0.5);
            let noise_metadata_schedule_160_0_e1759: f64 = (noise_metadata_schedule_160_0_e1756).powf(noise_metadata_schedule_160_0_e1758);
            w[257] = noise_metadata_schedule_160_0_e1759;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_161_0_e1762: f64 = (1.0 / w[90]);
            w[258] = noise_metadata_schedule_161_0_e1762;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_162_0_e1765: f64 = (params[36] * w[85]);
            let noise_metadata_schedule_162_0_e1767: f64 = (noise_metadata_schedule_162_0_e1765 * w[85]);
            let noise_metadata_schedule_162_0_e1769: f64 = (noise_metadata_schedule_162_0_e1767 * w[257]);
            let noise_metadata_schedule_162_0_e1771: f64 = (noise_metadata_schedule_162_0_e1769 * w[258]);
            let noise_metadata_schedule_162_0_e1773: f64 = (noise_metadata_schedule_162_0_e1771 * w[75]);
            let noise_metadata_schedule_162_0_e1775: f64 = (noise_metadata_schedule_162_0_e1773 * w[67]);
            let noise_metadata_schedule_162_0_e1777: f64 = (noise_metadata_schedule_162_0_e1775 * w[86]);
            let noise_metadata_schedule_162_0_e1779: f64 = (noise_metadata_schedule_162_0_e1777 * w[86]);
            w[83] = noise_metadata_schedule_162_0_e1779;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_163_0_e1782: f64 = (params[35] * w[257]);
            let noise_metadata_schedule_163_0_e1784: f64 = (noise_metadata_schedule_163_0_e1782 * w[19]);
            let noise_metadata_schedule_163_0_e1786: f64 = (noise_metadata_schedule_163_0_e1784 * w[19]);
            let noise_metadata_schedule_163_0_e1788: f64 = (noise_metadata_schedule_163_0_e1786 * w[66]);
            let noise_metadata_schedule_163_0_e1790: f64 = (noise_metadata_schedule_163_0_e1788 * w[66]);
            let noise_metadata_schedule_163_0_e1792: f64 = (noise_metadata_schedule_163_0_e1790 * w[90]);
            let noise_metadata_schedule_163_0_e1795: f64 = (params[36] - w[83]);
            let noise_metadata_schedule_163_0_e1796: f64 = (noise_metadata_schedule_163_0_e1795).exp();
            let noise_metadata_schedule_163_0_e1797: f64 = (noise_metadata_schedule_163_0_e1792 * noise_metadata_schedule_163_0_e1796);
            w[84] = noise_metadata_schedule_163_0_e1797;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_164_0_e1800: f64 = (w[254] * params[95]);
            let noise_metadata_schedule_164_0_e1801: f64 = (noise_metadata_schedule_164_0_e1800).exp();
            w[255] = noise_metadata_schedule_164_0_e1801;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_165_0_e1804: f64 = (params[13] * w[255]);
            let noise_metadata_schedule_165_0_e1806: f64 = (noise_metadata_schedule_165_0_e1804 * w[27]);
            w[40] = noise_metadata_schedule_165_0_e1806;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_166_0_e1809: f64 = (params[12] * w[255]);
            let noise_metadata_schedule_166_0_e1811: f64 = (noise_metadata_schedule_166_0_e1809 * w[256]);
            w[41] = noise_metadata_schedule_166_0_e1811;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_172_0_e1863: f64 = (w[2] - 300.0);
            w[100] = noise_metadata_schedule_172_0_e1863;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_173_0_e1866: f64 = if w[2] < 525.0 { 1.0 } else { 0.0 };
            w[459] = noise_metadata_schedule_173_0_e1866;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_174_0_e1882,) = {
    if (w[459] != 0.0) {
        let noise_metadata_schedule_174_0_e1872: f64 = (0.00072 * w[100]);
        let noise_metadata_schedule_174_0_e1873: f64 = (1.0 + noise_metadata_schedule_174_0_e1872);
        let noise_metadata_schedule_174_0_e1876: f64 = (1.6e-6 * w[100]);
        let noise_metadata_schedule_174_0_e1878: f64 = (noise_metadata_schedule_174_0_e1876 * w[100]);
        let noise_metadata_schedule_174_0_e1879: f64 = (noise_metadata_schedule_174_0_e1873 - noise_metadata_schedule_174_0_e1878);
        let noise_metadata_schedule_174_0_e1880: f64 = (w[1] * noise_metadata_schedule_174_0_e1879);
        (noise_metadata_schedule_174_0_e1880,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_174_0_e1882;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_175_0_e1889,) = {
    if (w[459] == 0.0) {
        let noise_metadata_schedule_175_0_e1887: f64 = (w[1] * 1.081);
        (noise_metadata_schedule_175_0_e1887,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_175_0_e1889;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_176_0_e1893: f64 = (w[254] * params[95]);
            let noise_metadata_schedule_176_0_e1894: f64 = (noise_metadata_schedule_176_0_e1893).exp();
            let noise_metadata_schedule_176_0_e1895: f64 = (params[91] * noise_metadata_schedule_176_0_e1894);
            w[99] = noise_metadata_schedule_176_0_e1895;
        }
        if (active[0] & 0x1520000) != 0 {
            let noise_metadata_schedule_177_0_e1898: f64 = if params[56] > 0.0 { 1.0 } else { 0.0 };
            w[460] = noise_metadata_schedule_177_0_e1898;
        }
        if (active[0] & 0x1520000) != 0 {
            let (noise_metadata_schedule_178_0_e1904,) = {
    if (w[460] != 0.0) {
        let noise_metadata_schedule_178_0_e1902: f64 = (1.0 / w[32]);
        (noise_metadata_schedule_178_0_e1902,)
    } else {
        (w[101],)
    }
};
            w[101] = noise_metadata_schedule_178_0_e1904;
        }
        if (active[0] & 0x1520000) != 0 {
            let noise_metadata_schedule_179_0_e1907: f64 = if w[101] > w[317] { 1.0 } else { 0.0 };
            w[461] = noise_metadata_schedule_179_0_e1907;
        }
        if (active[0] & 0x1520000) != 0 {
            let (noise_metadata_schedule_180_0_e1913,) = {
    if ((w[460] != 0.0) && (w[461] != 0.0)) {
        (w[317],)
    } else {
        (w[101],)
    }
};
            w[101] = noise_metadata_schedule_180_0_e1913;
        }
        if (active[0] & 0x1520000) != 0 {
            let (noise_metadata_schedule_181_0_e1918,) = {
    if (w[460] == 0.0) {
        (0.0,)
    } else {
        (w[101],)
    }
};
            w[101] = noise_metadata_schedule_181_0_e1918;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_182_0_e1921: f64 = if params[57] > 0.0 { 1.0 } else { 0.0 };
            w[462] = noise_metadata_schedule_182_0_e1921;
        }
        if (active[0] & 0x240000) != 0 {
            let (noise_metadata_schedule_183_0_e1927,) = {
    if (w[462] != 0.0) {
        let noise_metadata_schedule_183_0_e1925: f64 = (1.0 / w[33]);
        (noise_metadata_schedule_183_0_e1925,)
    } else {
        (w[102],)
    }
};
            w[102] = noise_metadata_schedule_183_0_e1927;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_184_0_e1930: f64 = if w[102] > w[317] { 1.0 } else { 0.0 };
            w[463] = noise_metadata_schedule_184_0_e1930;
        }
        if (active[0] & 0x240000) != 0 {
            let (noise_metadata_schedule_185_0_e1936,) = {
    if ((w[462] != 0.0) && (w[463] != 0.0)) {
        (w[317],)
    } else {
        (w[102],)
    }
};
            w[102] = noise_metadata_schedule_185_0_e1936;
        }
        if (active[0] & 0x240000) != 0 {
            let (noise_metadata_schedule_186_0_e1941,) = {
    if (w[462] == 0.0) {
        (0.0,)
    } else {
        (w[102],)
    }
};
            w[102] = noise_metadata_schedule_186_0_e1941;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_187_0_e1944: f64 = if params[58] > 0.0 { 1.0 } else { 0.0 };
            w[464] = noise_metadata_schedule_187_0_e1944;
        }
        if (active[0] & 0x880000) != 0 {
            let (noise_metadata_schedule_188_0_e1950,) = {
    if (w[464] != 0.0) {
        let noise_metadata_schedule_188_0_e1948: f64 = (1.0 / w[34]);
        (noise_metadata_schedule_188_0_e1948,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_188_0_e1950;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_189_0_e1953: f64 = if w[103] > w[317] { 1.0 } else { 0.0 };
            w[465] = noise_metadata_schedule_189_0_e1953;
        }
        if (active[0] & 0x880000) != 0 {
            let (noise_metadata_schedule_190_0_e1959,) = {
    if ((w[464] != 0.0) && (w[465] != 0.0)) {
        (w[317],)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_190_0_e1959;
        }
        if (active[0] & 0x880000) != 0 {
            let (noise_metadata_schedule_191_0_e1964,) = {
    if (w[464] == 0.0) {
        (0.0,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_191_0_e1964;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_192_0_e1967: f64 = (params[3] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            w[230] = noise_metadata_schedule_192_0_e1967;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_193_0_e1970: f64 = (params[3] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[7])));
            w[231] = noise_metadata_schedule_193_0_e1970;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_194_0_e1973: f64 = (params[3] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[3])));
            w[232] = noise_metadata_schedule_194_0_e1973;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_195_0_e1976: f64 = (params[3] * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[3])));
            w[233] = noise_metadata_schedule_195_0_e1976;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_196_0_e1979: f64 = (params[3] * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])));
            w[234] = noise_metadata_schedule_196_0_e1979;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_197_0_e1982: f64 = (params[3] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            w[236] = noise_metadata_schedule_197_0_e1982;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_199_0_e1988: f64 = (params[3] * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[4])));
            w[240] = noise_metadata_schedule_199_0_e1988;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_201_0_e1994: f64 = (params[3] * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            w[244] = noise_metadata_schedule_201_0_e1994;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_202_0_e1997: f64 = (params[3] * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[6])));
            w[238] = noise_metadata_schedule_202_0_e1997;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_203_0_e2000: f64 = (params[3] * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
            w[237] = noise_metadata_schedule_203_0_e2000;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_204_0_e2003: f64 = (w[234] + w[231]);
            let noise_metadata_schedule_204_0_e2005: f64 = (noise_metadata_schedule_204_0_e2003 - w[236]);
            let noise_metadata_schedule_204_0_e2007: f64 = (noise_metadata_schedule_204_0_e2005 - w[238]);
            w[235] = noise_metadata_schedule_204_0_e2007;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_205_0_e2009: f64 = (-w[244]);
            let noise_metadata_schedule_205_0_e2011: f64 = (noise_metadata_schedule_205_0_e2009 + w[240]);
            let noise_metadata_schedule_205_0_e2013: f64 = (noise_metadata_schedule_205_0_e2011 + w[235]);
            let noise_metadata_schedule_205_0_e2015: f64 = (noise_metadata_schedule_205_0_e2013 - w[237]);
            w[242] = noise_metadata_schedule_205_0_e2015;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_206_0_e2018: f64 = (w[244] + w[242]);
            w[241] = noise_metadata_schedule_206_0_e2018;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_207_0_e2021: f64 = (w[231] * w[8]);
            let noise_metadata_schedule_207_0_e2023: f64 = if noise_metadata_schedule_207_0_e2021 < params[134] { 1.0 } else { 0.0 };
            w[466] = noise_metadata_schedule_207_0_e2023;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_208_0_e2030,) = {
    if (w[466] != 0.0) {
        let noise_metadata_schedule_208_0_e2027: f64 = (w[231] * w[8]);
        let noise_metadata_schedule_208_0_e2028: f64 = (noise_metadata_schedule_208_0_e2027).exp();
        (noise_metadata_schedule_208_0_e2028,)
    } else {
        (w[245],)
    }
};
            w[245] = noise_metadata_schedule_208_0_e2030;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_209_0_e2036,) = {
    if (w[466] == 0.0) {
        let noise_metadata_schedule_209_0_e2034: f64 = (params[134]).exp();
        (noise_metadata_schedule_209_0_e2034,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_209_0_e2036;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_210_0_e2049,) = {
    if (w[466] == 0.0) {
        let noise_metadata_schedule_210_0_e2043: f64 = (w[231] * w[8]);
        let noise_metadata_schedule_210_0_e2045: f64 = (noise_metadata_schedule_210_0_e2043 - params[134]);
        let noise_metadata_schedule_210_0_e2046: f64 = (1.0 + noise_metadata_schedule_210_0_e2045);
        let noise_metadata_schedule_210_0_e2047: f64 = (w[275] * noise_metadata_schedule_210_0_e2046);
        (noise_metadata_schedule_210_0_e2047,)
    } else {
        (w[245],)
    }
};
            w[245] = noise_metadata_schedule_210_0_e2049;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_211_0_e2052: f64 = (w[232] * w[8]);
            let noise_metadata_schedule_211_0_e2054: f64 = (noise_metadata_schedule_211_0_e2052 / w[48]);
            let noise_metadata_schedule_211_0_e2056: f64 = if noise_metadata_schedule_211_0_e2054 < params[134] { 1.0 } else { 0.0 };
            w[467] = noise_metadata_schedule_211_0_e2056;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_212_0_e2065,) = {
    if (w[467] != 0.0) {
        let noise_metadata_schedule_212_0_e2060: f64 = (w[232] * w[8]);
        let noise_metadata_schedule_212_0_e2062: f64 = (noise_metadata_schedule_212_0_e2060 / w[48]);
        let noise_metadata_schedule_212_0_e2063: f64 = (noise_metadata_schedule_212_0_e2062).exp();
        (noise_metadata_schedule_212_0_e2063,)
    } else {
        (w[246],)
    }
};
            w[246] = noise_metadata_schedule_212_0_e2065;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_213_0_e2071,) = {
    if (w[467] == 0.0) {
        let noise_metadata_schedule_213_0_e2069: f64 = (params[134]).exp();
        (noise_metadata_schedule_213_0_e2069,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_213_0_e2071;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_214_0_e2086,) = {
    if (w[467] == 0.0) {
        let noise_metadata_schedule_214_0_e2078: f64 = (w[232] * w[8]);
        let noise_metadata_schedule_214_0_e2080: f64 = (noise_metadata_schedule_214_0_e2078 / w[48]);
        let noise_metadata_schedule_214_0_e2082: f64 = (noise_metadata_schedule_214_0_e2080 - params[134]);
        let noise_metadata_schedule_214_0_e2083: f64 = (1.0 + noise_metadata_schedule_214_0_e2082);
        let noise_metadata_schedule_214_0_e2084: f64 = (w[275] * noise_metadata_schedule_214_0_e2083);
        (noise_metadata_schedule_214_0_e2084,)
    } else {
        (w[246],)
    }
};
            w[246] = noise_metadata_schedule_214_0_e2086;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_215_0_e2089: f64 = (w[235] * w[8]);
            let noise_metadata_schedule_215_0_e2091: f64 = if noise_metadata_schedule_215_0_e2089 < params[134] { 1.0 } else { 0.0 };
            w[468] = noise_metadata_schedule_215_0_e2091;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_216_0_e2098,) = {
    if (w[468] != 0.0) {
        let noise_metadata_schedule_216_0_e2095: f64 = (w[235] * w[8]);
        let noise_metadata_schedule_216_0_e2096: f64 = (noise_metadata_schedule_216_0_e2095).exp();
        (noise_metadata_schedule_216_0_e2096,)
    } else {
        (w[248],)
    }
};
            w[248] = noise_metadata_schedule_216_0_e2098;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_217_0_e2104,) = {
    if (w[468] == 0.0) {
        let noise_metadata_schedule_217_0_e2102: f64 = (params[134]).exp();
        (noise_metadata_schedule_217_0_e2102,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_217_0_e2104;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_218_0_e2117,) = {
    if (w[468] == 0.0) {
        let noise_metadata_schedule_218_0_e2111: f64 = (w[235] * w[8]);
        let noise_metadata_schedule_218_0_e2113: f64 = (noise_metadata_schedule_218_0_e2111 - params[134]);
        let noise_metadata_schedule_218_0_e2114: f64 = (1.0 + noise_metadata_schedule_218_0_e2113);
        let noise_metadata_schedule_218_0_e2115: f64 = (w[275] * noise_metadata_schedule_218_0_e2114);
        (noise_metadata_schedule_218_0_e2115,)
    } else {
        (w[248],)
    }
};
            w[248] = noise_metadata_schedule_218_0_e2117;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_219_0_e2120: f64 = (w[234] * w[8]);
            let noise_metadata_schedule_219_0_e2122: f64 = if noise_metadata_schedule_219_0_e2120 < params[134] { 1.0 } else { 0.0 };
            w[469] = noise_metadata_schedule_219_0_e2122;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_220_0_e2129,) = {
    if (w[469] != 0.0) {
        let noise_metadata_schedule_220_0_e2126: f64 = (w[234] * w[8]);
        let noise_metadata_schedule_220_0_e2127: f64 = (noise_metadata_schedule_220_0_e2126).exp();
        (noise_metadata_schedule_220_0_e2127,)
    } else {
        (w[247],)
    }
};
            w[247] = noise_metadata_schedule_220_0_e2129;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_221_0_e2135,) = {
    if (w[469] == 0.0) {
        let noise_metadata_schedule_221_0_e2133: f64 = (params[134]).exp();
        (noise_metadata_schedule_221_0_e2133,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_221_0_e2135;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_222_0_e2148,) = {
    if (w[469] == 0.0) {
        let noise_metadata_schedule_222_0_e2142: f64 = (w[234] * w[8]);
        let noise_metadata_schedule_222_0_e2144: f64 = (noise_metadata_schedule_222_0_e2142 - params[134]);
        let noise_metadata_schedule_222_0_e2145: f64 = (1.0 + noise_metadata_schedule_222_0_e2144);
        let noise_metadata_schedule_222_0_e2146: f64 = (w[275] * noise_metadata_schedule_222_0_e2145);
        (noise_metadata_schedule_222_0_e2146,)
    } else {
        (w[247],)
    }
};
            w[247] = noise_metadata_schedule_222_0_e2148;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_223_0_e2151: f64 = (w[241] * w[8]);
            let noise_metadata_schedule_223_0_e2153: f64 = if noise_metadata_schedule_223_0_e2151 < params[134] { 1.0 } else { 0.0 };
            w[470] = noise_metadata_schedule_223_0_e2153;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_224_0_e2160,) = {
    if (w[470] != 0.0) {
        let noise_metadata_schedule_224_0_e2157: f64 = (w[241] * w[8]);
        let noise_metadata_schedule_224_0_e2158: f64 = (noise_metadata_schedule_224_0_e2157).exp();
        (noise_metadata_schedule_224_0_e2158,)
    } else {
        (w[249],)
    }
};
            w[249] = noise_metadata_schedule_224_0_e2160;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_225_0_e2166,) = {
    if (w[470] == 0.0) {
        let noise_metadata_schedule_225_0_e2164: f64 = (params[134]).exp();
        (noise_metadata_schedule_225_0_e2164,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_225_0_e2166;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_226_0_e2179,) = {
    if (w[470] == 0.0) {
        let noise_metadata_schedule_226_0_e2173: f64 = (w[241] * w[8]);
        let noise_metadata_schedule_226_0_e2175: f64 = (noise_metadata_schedule_226_0_e2173 - params[134]);
        let noise_metadata_schedule_226_0_e2176: f64 = (1.0 + noise_metadata_schedule_226_0_e2175);
        let noise_metadata_schedule_226_0_e2177: f64 = (w[275] * noise_metadata_schedule_226_0_e2176);
        (noise_metadata_schedule_226_0_e2177,)
    } else {
        (w[249],)
    }
};
            w[249] = noise_metadata_schedule_226_0_e2179;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_227_0_e2182: f64 = (w[241] - w[16]);
            let noise_metadata_schedule_227_0_e2184: f64 = (noise_metadata_schedule_227_0_e2182 * w[8]);
            let noise_metadata_schedule_227_0_e2186: f64 = if noise_metadata_schedule_227_0_e2184 < params[134] { 1.0 } else { 0.0 };
            w[471] = noise_metadata_schedule_227_0_e2186;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_229_0_e2201,) = {
    if (w[471] == 0.0) {
        let noise_metadata_schedule_229_0_e2199: f64 = (params[134]).exp();
        (noise_metadata_schedule_229_0_e2199,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_229_0_e2201;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_231_0_e2219: f64 = (w[235] - w[16]);
            let noise_metadata_schedule_231_0_e2221: f64 = (noise_metadata_schedule_231_0_e2219 * w[8]);
            let noise_metadata_schedule_231_0_e2223: f64 = if noise_metadata_schedule_231_0_e2221 < params[134] { 1.0 } else { 0.0 };
            w[472] = noise_metadata_schedule_231_0_e2223;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_233_0_e2238,) = {
    if (w[472] == 0.0) {
        let noise_metadata_schedule_233_0_e2236: f64 = (params[134]).exp();
        (noise_metadata_schedule_233_0_e2236,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_233_0_e2238;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_235_0_e2256: f64 = (w[231] - w[16]);
            let noise_metadata_schedule_235_0_e2258: f64 = (noise_metadata_schedule_235_0_e2256 * w[8]);
            let noise_metadata_schedule_235_0_e2260: f64 = if noise_metadata_schedule_235_0_e2258 < params[134] { 1.0 } else { 0.0 };
            w[473] = noise_metadata_schedule_235_0_e2260;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_236_0_e2269,) = {
    if (w[473] != 0.0) {
        let noise_metadata_schedule_236_0_e2264: f64 = (w[231] - w[16]);
        let noise_metadata_schedule_236_0_e2266: f64 = (noise_metadata_schedule_236_0_e2264 * w[8]);
        let noise_metadata_schedule_236_0_e2267: f64 = (noise_metadata_schedule_236_0_e2266).exp();
        (noise_metadata_schedule_236_0_e2267,)
    } else {
        (w[251],)
    }
};
            w[251] = noise_metadata_schedule_236_0_e2269;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_237_0_e2275,) = {
    if (w[473] == 0.0) {
        let noise_metadata_schedule_237_0_e2273: f64 = (params[134]).exp();
        (noise_metadata_schedule_237_0_e2273,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_237_0_e2275;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_238_0_e2290,) = {
    if (w[473] == 0.0) {
        let noise_metadata_schedule_238_0_e2282: f64 = (w[231] - w[16]);
        let noise_metadata_schedule_238_0_e2284: f64 = (noise_metadata_schedule_238_0_e2282 * w[8]);
        let noise_metadata_schedule_238_0_e2286: f64 = (noise_metadata_schedule_238_0_e2284 - params[134]);
        let noise_metadata_schedule_238_0_e2287: f64 = (1.0 + noise_metadata_schedule_238_0_e2286);
        let noise_metadata_schedule_238_0_e2288: f64 = (w[275] * noise_metadata_schedule_238_0_e2287);
        (noise_metadata_schedule_238_0_e2288,)
    } else {
        (w[251],)
    }
};
            w[251] = noise_metadata_schedule_238_0_e2290;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_239_0_e2293: f64 = (w[230] - w[16]);
            let noise_metadata_schedule_239_0_e2295: f64 = (noise_metadata_schedule_239_0_e2293 * w[8]);
            let noise_metadata_schedule_239_0_e2297: f64 = if noise_metadata_schedule_239_0_e2295 < params[134] { 1.0 } else { 0.0 };
            w[474] = noise_metadata_schedule_239_0_e2297;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_240_0_e2306,) = {
    if (w[474] != 0.0) {
        let noise_metadata_schedule_240_0_e2301: f64 = (w[230] - w[16]);
        let noise_metadata_schedule_240_0_e2303: f64 = (noise_metadata_schedule_240_0_e2301 * w[8]);
        let noise_metadata_schedule_240_0_e2304: f64 = (noise_metadata_schedule_240_0_e2303).exp();
        (noise_metadata_schedule_240_0_e2304,)
    } else {
        (w[253],)
    }
};
            w[253] = noise_metadata_schedule_240_0_e2306;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_241_0_e2312,) = {
    if (w[474] == 0.0) {
        let noise_metadata_schedule_241_0_e2310: f64 = (params[134]).exp();
        (noise_metadata_schedule_241_0_e2310,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_241_0_e2312;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_242_0_e2327,) = {
    if (w[474] == 0.0) {
        let noise_metadata_schedule_242_0_e2319: f64 = (w[230] - w[16]);
        let noise_metadata_schedule_242_0_e2321: f64 = (noise_metadata_schedule_242_0_e2319 * w[8]);
        let noise_metadata_schedule_242_0_e2323: f64 = (noise_metadata_schedule_242_0_e2321 - params[134]);
        let noise_metadata_schedule_242_0_e2324: f64 = (1.0 + noise_metadata_schedule_242_0_e2323);
        let noise_metadata_schedule_242_0_e2325: f64 = (w[275] * noise_metadata_schedule_242_0_e2324);
        (noise_metadata_schedule_242_0_e2325,)
    } else {
        (w[253],)
    }
};
            w[253] = noise_metadata_schedule_242_0_e2327;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_243_0_e2331: f64 = (4.0 * w[251]);
            let noise_metadata_schedule_243_0_e2332: f64 = (1.0 + noise_metadata_schedule_243_0_e2331);
            let noise_metadata_schedule_243_0_e2333: f64 = (noise_metadata_schedule_243_0_e2332).sqrt();
            w[104] = noise_metadata_schedule_243_0_e2333;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_244_0_e2337: f64 = (4.0 * w[253]);
            let noise_metadata_schedule_244_0_e2338: f64 = (1.0 + noise_metadata_schedule_244_0_e2337);
            let noise_metadata_schedule_244_0_e2339: f64 = (noise_metadata_schedule_244_0_e2338).sqrt();
            w[105] = noise_metadata_schedule_244_0_e2339;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_245_0_e2342: f64 = (2.0 * w[253]);
            let noise_metadata_schedule_245_0_e2345: f64 = (1.0 + w[105]);
            let noise_metadata_schedule_245_0_e2346: f64 = (noise_metadata_schedule_245_0_e2342 / noise_metadata_schedule_245_0_e2345);
            w[106] = noise_metadata_schedule_245_0_e2346;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_246_0_e2349: f64 = if w[106] < params[136] { 1.0 } else { 0.0 };
            w[475] = noise_metadata_schedule_246_0_e2349;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_247_0_e2353,) = {
    if (w[475] != 0.0) {
        (params[136],)
    } else {
        (w[106],)
    }
};
            w[106] = noise_metadata_schedule_247_0_e2353;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_248_0_e2357: f64 = (w[104] - w[105]);
            let noise_metadata_schedule_248_0_e2360: f64 = (w[104] + 1.0);
            let noise_metadata_schedule_248_0_e2363: f64 = (w[105] + 1.0);
            let noise_metadata_schedule_248_0_e2364: f64 = (noise_metadata_schedule_248_0_e2360 / noise_metadata_schedule_248_0_e2363);
            let noise_metadata_schedule_248_0_e2365: f64 = (noise_metadata_schedule_248_0_e2364).ln();
            let noise_metadata_schedule_248_0_e2366: f64 = (noise_metadata_schedule_248_0_e2357 - noise_metadata_schedule_248_0_e2365);
            let noise_metadata_schedule_248_0_e2367: f64 = (w[6] * noise_metadata_schedule_248_0_e2366);
            w[107] = noise_metadata_schedule_248_0_e2367;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_249_0_e2370: f64 = (w[107] + w[236]);
            let noise_metadata_schedule_249_0_e2372: f64 = (noise_metadata_schedule_249_0_e2370 / w[31]);
            w[108] = noise_metadata_schedule_249_0_e2372;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_250_0_e2375: f64 = if w[108] > 0.0 { 1.0 } else { 0.0 };
            w[476] = noise_metadata_schedule_250_0_e2375;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_251_0_e2378: f64 = if w[230] < 100.0 { 1.0 } else { 0.0 };
            w[477] = noise_metadata_schedule_251_0_e2378;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_252_0_e2384,) = {
    if ((w[476] != 0.0) && (w[477] != 0.0)) {
        (w[230],)
    } else {
        (w[277],)
    }
};
            w[277] = noise_metadata_schedule_252_0_e2384;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_253_0_e2398,) = {
    if ((w[476] != 0.0) && (w[477] == 0.0)) {
        let noise_metadata_schedule_253_0_e2393: f64 = (w[230] - 100.0);
        let noise_metadata_schedule_253_0_e2394: f64 = (1.0 + noise_metadata_schedule_253_0_e2393);
        let noise_metadata_schedule_253_0_e2395: f64 = (noise_metadata_schedule_253_0_e2394).ln();
        let noise_metadata_schedule_253_0_e2396: f64 = (100.0 + noise_metadata_schedule_253_0_e2395);
        (noise_metadata_schedule_253_0_e2396,)
    } else {
        (w[277],)
    }
};
            w[277] = noise_metadata_schedule_253_0_e2398;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_254_0_e2419,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_254_0_e2403: f64 = (2.0 * w[6]);
        let noise_metadata_schedule_254_0_e2406: f64 = (0.5 * w[108]);
        let noise_metadata_schedule_254_0_e2408: f64 = (noise_metadata_schedule_254_0_e2406 * w[31]);
        let noise_metadata_schedule_254_0_e2410: f64 = (noise_metadata_schedule_254_0_e2408 * w[8]);
        let noise_metadata_schedule_254_0_e2412: f64 = (noise_metadata_schedule_254_0_e2410 + 1.0);
        let noise_metadata_schedule_254_0_e2413: f64 = (noise_metadata_schedule_254_0_e2412).ln();
        let noise_metadata_schedule_254_0_e2414: f64 = (noise_metadata_schedule_254_0_e2403 * noise_metadata_schedule_254_0_e2413);
        let noise_metadata_schedule_254_0_e2415: f64 = (w[16] + noise_metadata_schedule_254_0_e2414);
        let noise_metadata_schedule_254_0_e2417: f64 = (noise_metadata_schedule_254_0_e2415 - w[277]);
        (noise_metadata_schedule_254_0_e2417,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_254_0_e2419;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_255_0_e2425,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_255_0_e2423: f64 = (0.2 * w[16]);
        (noise_metadata_schedule_255_0_e2423,)
    } else {
        (w[272],)
    }
};
            w[272] = noise_metadata_schedule_255_0_e2425;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_256_0_e2431,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_256_0_e2429: f64 = (w[272] * w[272]);
        (noise_metadata_schedule_256_0_e2429,)
    } else {
        (w[261],)
    }
};
            w[261] = noise_metadata_schedule_256_0_e2431;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_257_0_e2437,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_257_0_e2435: f64 = (w[109] * w[109]);
        (noise_metadata_schedule_257_0_e2435,)
    } else {
        (w[262],)
    }
};
            w[262] = noise_metadata_schedule_257_0_e2437;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_258_0_e2440: f64 = if w[109] < 0.0 { 1.0 } else { 0.0 };
            w[478] = noise_metadata_schedule_258_0_e2440;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_259_0_e2455,) = {
    if ((w[476] != 0.0) && (w[478] != 0.0)) {
        let noise_metadata_schedule_259_0_e2446: f64 = (0.5 * w[261]);
        let noise_metadata_schedule_259_0_e2449: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_259_0_e2450: f64 = (noise_metadata_schedule_259_0_e2449).sqrt();
        let noise_metadata_schedule_259_0_e2452: f64 = (noise_metadata_schedule_259_0_e2450 - w[109]);
        let noise_metadata_schedule_259_0_e2453: f64 = (noise_metadata_schedule_259_0_e2446 / noise_metadata_schedule_259_0_e2452);
        (noise_metadata_schedule_259_0_e2453,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_259_0_e2455;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_260_0_e2469,) = {
    if ((w[476] != 0.0) && (w[478] == 0.0)) {
        let noise_metadata_schedule_260_0_e2463: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_260_0_e2464: f64 = (noise_metadata_schedule_260_0_e2463).sqrt();
        let noise_metadata_schedule_260_0_e2466: f64 = (noise_metadata_schedule_260_0_e2464 + w[109]);
        let noise_metadata_schedule_260_0_e2467: f64 = (0.5 * noise_metadata_schedule_260_0_e2466);
        (noise_metadata_schedule_260_0_e2467,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_260_0_e2469;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_261_0_e2487,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_261_0_e2475: f64 = (params[61] * params[60]);
        let noise_metadata_schedule_261_0_e2476: f64 = (w[110] + noise_metadata_schedule_261_0_e2475);
        let noise_metadata_schedule_261_0_e2477: f64 = (w[110] * noise_metadata_schedule_261_0_e2476);
        let noise_metadata_schedule_261_0_e2482: f64 = (params[61] * w[31]);
        let noise_metadata_schedule_261_0_e2483: f64 = (w[110] + noise_metadata_schedule_261_0_e2482);
        let noise_metadata_schedule_261_0_e2484: f64 = (params[60] * noise_metadata_schedule_261_0_e2483);
        let noise_metadata_schedule_261_0_e2485: f64 = (noise_metadata_schedule_261_0_e2477 / noise_metadata_schedule_261_0_e2484);
        (noise_metadata_schedule_261_0_e2485,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_261_0_e2487;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_262_0_e2493,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_262_0_e2491: f64 = (w[108] / w[111]);
        (noise_metadata_schedule_262_0_e2491,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_262_0_e2493;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_263_0_e2501,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_263_0_e2497: f64 = (w[265] - 1.0);
        let noise_metadata_schedule_263_0_e2499: f64 = (noise_metadata_schedule_263_0_e2497 / params[62]);
        (noise_metadata_schedule_263_0_e2499,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_263_0_e2501;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_264_0_e2504: f64 = if w[265] < 1.0 { 1.0 } else { 0.0 };
            w[479] = noise_metadata_schedule_264_0_e2504;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_265_0_e2518,) = {
    if ((w[476] != 0.0) && (w[479] != 0.0)) {
        let noise_metadata_schedule_265_0_e2512: f64 = (w[259]).exp();
        let noise_metadata_schedule_265_0_e2513: f64 = (1.0 + noise_metadata_schedule_265_0_e2512);
        let noise_metadata_schedule_265_0_e2514: f64 = (noise_metadata_schedule_265_0_e2513).ln();
        let noise_metadata_schedule_265_0_e2515: f64 = (params[62] * noise_metadata_schedule_265_0_e2514);
        let noise_metadata_schedule_265_0_e2516: f64 = (1.0 + noise_metadata_schedule_265_0_e2515);
        (noise_metadata_schedule_265_0_e2516,)
    } else {
        (w[263],)
    }
};
            w[263] = noise_metadata_schedule_265_0_e2518;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_266_0_e2534,) = {
    if ((w[476] != 0.0) && (w[479] == 0.0)) {
        let noise_metadata_schedule_266_0_e2527: f64 = (-w[259]);
        let noise_metadata_schedule_266_0_e2528: f64 = (noise_metadata_schedule_266_0_e2527).exp();
        let noise_metadata_schedule_266_0_e2529: f64 = (1.0 + noise_metadata_schedule_266_0_e2528);
        let noise_metadata_schedule_266_0_e2530: f64 = (noise_metadata_schedule_266_0_e2529).ln();
        let noise_metadata_schedule_266_0_e2531: f64 = (params[62] * noise_metadata_schedule_266_0_e2530);
        let noise_metadata_schedule_266_0_e2532: f64 = (w[265] + noise_metadata_schedule_266_0_e2531);
        (noise_metadata_schedule_266_0_e2532,)
    } else {
        (w[263],)
    }
};
            w[263] = noise_metadata_schedule_266_0_e2534;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_267_0_e2551,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_267_0_e2541: f64 = (-1.0);
        let noise_metadata_schedule_267_0_e2543: f64 = (noise_metadata_schedule_267_0_e2541 / params[62]);
        let noise_metadata_schedule_267_0_e2544: f64 = (noise_metadata_schedule_267_0_e2543).exp();
        let noise_metadata_schedule_267_0_e2545: f64 = (1.0 + noise_metadata_schedule_267_0_e2544);
        let noise_metadata_schedule_267_0_e2546: f64 = (noise_metadata_schedule_267_0_e2545).ln();
        let noise_metadata_schedule_267_0_e2547: f64 = (params[62] * noise_metadata_schedule_267_0_e2546);
        let noise_metadata_schedule_267_0_e2548: f64 = (1.0 + noise_metadata_schedule_267_0_e2547);
        let noise_metadata_schedule_267_0_e2549: f64 = (w[263] / noise_metadata_schedule_267_0_e2548);
        (noise_metadata_schedule_267_0_e2549,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_267_0_e2551;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_268_0_e2559,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_268_0_e2556: f64 = (params[61] * params[60]);
        let noise_metadata_schedule_268_0_e2557: f64 = (w[110] / noise_metadata_schedule_268_0_e2556);
        (noise_metadata_schedule_268_0_e2557,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_268_0_e2559;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_269_0_e2584,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_269_0_e2565: f64 = (4.0 * w[112]);
        let noise_metadata_schedule_269_0_e2567: f64 = (noise_metadata_schedule_269_0_e2565 * w[113]);
        let noise_metadata_schedule_269_0_e2570: f64 = (1.0 + w[113]);
        let noise_metadata_schedule_269_0_e2571: f64 = (noise_metadata_schedule_269_0_e2567 * noise_metadata_schedule_269_0_e2570);
        let noise_metadata_schedule_269_0_e2572: f64 = (1.0 + noise_metadata_schedule_269_0_e2571);
        let noise_metadata_schedule_269_0_e2573: f64 = (noise_metadata_schedule_269_0_e2572).sqrt();
        let noise_metadata_schedule_269_0_e2574: f64 = (1.0 + noise_metadata_schedule_269_0_e2573);
        let noise_metadata_schedule_269_0_e2577: f64 = (2.0 * w[112]);
        let noise_metadata_schedule_269_0_e2580: f64 = (1.0 + w[113]);
        let noise_metadata_schedule_269_0_e2581: f64 = (noise_metadata_schedule_269_0_e2577 * noise_metadata_schedule_269_0_e2580);
        let noise_metadata_schedule_269_0_e2582: f64 = (noise_metadata_schedule_269_0_e2574 / noise_metadata_schedule_269_0_e2581);
        (noise_metadata_schedule_269_0_e2582,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_269_0_e2584;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_270_0_e2600,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_270_0_e2588: f64 = (1.0 - w[114]);
        let noise_metadata_schedule_270_0_e2591: f64 = (w[106] * w[114]);
        let noise_metadata_schedule_270_0_e2592: f64 = (noise_metadata_schedule_270_0_e2588 + noise_metadata_schedule_270_0_e2591);
        let noise_metadata_schedule_270_0_e2596: f64 = (w[106] * w[114]);
        let noise_metadata_schedule_270_0_e2597: f64 = (1.0 + noise_metadata_schedule_270_0_e2596);
        let noise_metadata_schedule_270_0_e2598: f64 = (noise_metadata_schedule_270_0_e2592 / noise_metadata_schedule_270_0_e2597);
        (noise_metadata_schedule_270_0_e2598,)
    } else {
        (w[115],)
    }
};
            w[115] = noise_metadata_schedule_270_0_e2600;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_271_0_e2612,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_271_0_e2604: f64 = (0.5 * w[108]);
        let noise_metadata_schedule_271_0_e2606: f64 = (noise_metadata_schedule_271_0_e2604 * w[31]);
        let noise_metadata_schedule_271_0_e2608: f64 = (noise_metadata_schedule_271_0_e2606 * w[115]);
        let noise_metadata_schedule_271_0_e2610: f64 = (noise_metadata_schedule_271_0_e2608 * w[8]);
        (noise_metadata_schedule_271_0_e2610,)
    } else {
        (w[117],)
    }
};
            w[117] = noise_metadata_schedule_271_0_e2612;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_272_0_e2626,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_272_0_e2616: f64 = (2.0 * w[117]);
        let noise_metadata_schedule_272_0_e2620: f64 = (w[106] + w[117]);
        let noise_metadata_schedule_272_0_e2622: f64 = (noise_metadata_schedule_272_0_e2620 + 1.0);
        let noise_metadata_schedule_272_0_e2623: f64 = (w[106] * noise_metadata_schedule_272_0_e2622);
        let noise_metadata_schedule_272_0_e2624: f64 = (noise_metadata_schedule_272_0_e2616 + noise_metadata_schedule_272_0_e2623);
        (noise_metadata_schedule_272_0_e2624,)
    } else {
        (w[266],)
    }
};
            w[266] = noise_metadata_schedule_272_0_e2626;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_273_0_e2634,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_273_0_e2631: f64 = (w[117] - 1.0);
        let noise_metadata_schedule_273_0_e2632: f64 = (0.5 * noise_metadata_schedule_273_0_e2631);
        (noise_metadata_schedule_273_0_e2632,)
    } else {
        (w[118],)
    }
};
            w[118] = noise_metadata_schedule_273_0_e2634;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_274_0_e2642,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_274_0_e2638: f64 = (w[118] * w[118]);
        let noise_metadata_schedule_274_0_e2640: f64 = (noise_metadata_schedule_274_0_e2638 + w[266]);
        (noise_metadata_schedule_274_0_e2640,)
    } else {
        (w[260],)
    }
};
            w[260] = noise_metadata_schedule_274_0_e2642;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_275_0_e2645: f64 = if w[117] >= 1.0 { 1.0 } else { 0.0 };
            w[480] = noise_metadata_schedule_275_0_e2645;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_276_0_e2654,) = {
    if ((w[476] != 0.0) && (w[480] != 0.0)) {
        let noise_metadata_schedule_276_0_e2651: f64 = (w[260]).sqrt();
        let noise_metadata_schedule_276_0_e2652: f64 = (w[118] + noise_metadata_schedule_276_0_e2651);
        (noise_metadata_schedule_276_0_e2652,)
    } else {
        (w[119],)
    }
};
            w[119] = noise_metadata_schedule_276_0_e2654;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_277_0_e2666,) = {
    if ((w[476] != 0.0) && (w[480] == 0.0)) {
        let noise_metadata_schedule_277_0_e2661: f64 = (w[260]).sqrt();
        let noise_metadata_schedule_277_0_e2663: f64 = (noise_metadata_schedule_277_0_e2661 - w[118]);
        let noise_metadata_schedule_277_0_e2664: f64 = (w[266] / noise_metadata_schedule_277_0_e2663);
        (noise_metadata_schedule_277_0_e2664,)
    } else {
        (w[119],)
    }
};
            w[119] = noise_metadata_schedule_277_0_e2666;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_278_0_e2669: f64 = if w[119] < params[135] { 1.0 } else { 0.0 };
            w[481] = noise_metadata_schedule_278_0_e2669;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_279_0_e2675,) = {
    if ((w[476] != 0.0) && (w[481] != 0.0)) {
        (params[135],)
    } else {
        (w[119],)
    }
};
            w[119] = noise_metadata_schedule_279_0_e2675;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_280_0_e2688,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_280_0_e2680: f64 = (w[119] + 1.0);
        let noise_metadata_schedule_280_0_e2681: f64 = (w[119] * noise_metadata_schedule_280_0_e2680);
        let noise_metadata_schedule_280_0_e2684: f64 = (w[16] * w[8]);
        let noise_metadata_schedule_280_0_e2685: f64 = (noise_metadata_schedule_280_0_e2684).exp();
        let noise_metadata_schedule_280_0_e2686: f64 = (noise_metadata_schedule_280_0_e2681 * noise_metadata_schedule_280_0_e2685);
        (noise_metadata_schedule_280_0_e2686,)
    } else {
        (w[121],)
    }
};
            w[121] = noise_metadata_schedule_280_0_e2688;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_281_0_e2698,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_281_0_e2692: f64 = (0.5 * params[60]);
        let noise_metadata_schedule_281_0_e2695: f64 = (w[108] - params[61]);
        let noise_metadata_schedule_281_0_e2696: f64 = (noise_metadata_schedule_281_0_e2692 * noise_metadata_schedule_281_0_e2695);
        (noise_metadata_schedule_281_0_e2696,)
    } else {
        (w[123],)
    }
};
            w[123] = noise_metadata_schedule_281_0_e2698;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_282_0_e2708,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_282_0_e2702: f64 = (params[60] * w[31]);
        let noise_metadata_schedule_282_0_e2704: f64 = (noise_metadata_schedule_282_0_e2702 * params[61]);
        let noise_metadata_schedule_282_0_e2706: f64 = (noise_metadata_schedule_282_0_e2704 * w[108]);
        (noise_metadata_schedule_282_0_e2706,)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_282_0_e2708;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_283_0_e2719,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_283_0_e2713: f64 = (w[123] * w[123]);
        let noise_metadata_schedule_283_0_e2715: f64 = (noise_metadata_schedule_283_0_e2713 + w[124]);
        let noise_metadata_schedule_283_0_e2716: f64 = (noise_metadata_schedule_283_0_e2715).sqrt();
        let noise_metadata_schedule_283_0_e2717: f64 = (w[123] + noise_metadata_schedule_283_0_e2716);
        (noise_metadata_schedule_283_0_e2717,)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_283_0_e2719;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_284_0_e2722: f64 = if params[72] == 0.0 { 1.0 } else { 0.0 };
            w[482] = noise_metadata_schedule_284_0_e2722;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_285_0_e2730,) = {
    if ((w[476] != 0.0) && (w[482] != 0.0)) {
        let noise_metadata_schedule_285_0_e2728: f64 = (w[17] * 0.1);
        (noise_metadata_schedule_285_0_e2728,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_285_0_e2730;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_286_0_e2747,) = {
    if ((w[476] != 0.0) && (w[482] == 0.0)) {
        let noise_metadata_schedule_286_0_e2739: f64 = (2.0 * w[108]);
        let noise_metadata_schedule_286_0_e2742: f64 = (w[108] + w[111]);
        let noise_metadata_schedule_286_0_e2743: f64 = (noise_metadata_schedule_286_0_e2739 / noise_metadata_schedule_286_0_e2742);
        let noise_metadata_schedule_286_0_e2744: f64 = (0.1 + noise_metadata_schedule_286_0_e2743);
        let noise_metadata_schedule_286_0_e2745: f64 = (w[17] * noise_metadata_schedule_286_0_e2744);
        (noise_metadata_schedule_286_0_e2745,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_286_0_e2747;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_287_0_e2757,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_287_0_e2751: f64 = (params[61] * w[108]);
        let noise_metadata_schedule_287_0_e2754: f64 = (params[61] + w[108]);
        let noise_metadata_schedule_287_0_e2755: f64 = (noise_metadata_schedule_287_0_e2751 / noise_metadata_schedule_287_0_e2754);
        (noise_metadata_schedule_287_0_e2755,)
    } else {
        (w[127],)
    }
};
            w[127] = noise_metadata_schedule_287_0_e2757;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_288_0_e2765,) = {
    if (w[476] != 0.0) {
        let noise_metadata_schedule_288_0_e2762: f64 = (params[61] + w[108]);
        let noise_metadata_schedule_288_0_e2763: f64 = (params[61] / noise_metadata_schedule_288_0_e2762);
        (noise_metadata_schedule_288_0_e2763,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_288_0_e2765;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_290_0_e2781,) = {
    if (w[476] == 0.0) {
        let noise_metadata_schedule_290_0_e2775: f64 = (2.0 * w[251]);
        let noise_metadata_schedule_290_0_e2778: f64 = (1.0 + w[104]);
        let noise_metadata_schedule_290_0_e2779: f64 = (noise_metadata_schedule_290_0_e2775 / noise_metadata_schedule_290_0_e2778);
        (noise_metadata_schedule_290_0_e2779,)
    } else {
        (w[119],)
    }
};
            w[119] = noise_metadata_schedule_290_0_e2781;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_291_0_e2786,) = {
    if (w[476] == 0.0) {
        (w[245],)
    } else {
        (w[121],)
    }
};
            w[121] = noise_metadata_schedule_291_0_e2786;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_292_0_e2788: f64 = (w[236]).abs();
            let noise_metadata_schedule_292_0_e2791: f64 = (1e-5 * w[6]);
            let noise_metadata_schedule_292_0_e2794: f64 = (w[107]).abs();
            let noise_metadata_schedule_292_0_e2797: f64 = (1e-40 * w[6]);
            let noise_metadata_schedule_292_0_e2800: f64 = (w[104] + w[105]);
            let noise_metadata_schedule_292_0_e2801: f64 = (noise_metadata_schedule_292_0_e2797 * noise_metadata_schedule_292_0_e2800);
            let noise_metadata_schedule_292_0_e2803: f64 = if ((noise_metadata_schedule_292_0_e2788 < noise_metadata_schedule_292_0_e2791) || (noise_metadata_schedule_292_0_e2794 < noise_metadata_schedule_292_0_e2801)) { 1.0 } else { 0.0 };
            w[483] = noise_metadata_schedule_292_0_e2803;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_293_0_e2814,) = {
    if ((w[476] == 0.0) && (w[483] != 0.0)) {
        let noise_metadata_schedule_293_0_e2811: f64 = (w[119] + w[106]);
        let noise_metadata_schedule_293_0_e2812: f64 = (0.5 * noise_metadata_schedule_293_0_e2811);
        (noise_metadata_schedule_293_0_e2812,)
    } else {
        (w[128],)
    }
};
            w[128] = noise_metadata_schedule_293_0_e2814;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_294_0_e2825,) = {
    if ((w[476] == 0.0) && (w[483] != 0.0)) {
        let noise_metadata_schedule_294_0_e2822: f64 = (w[128] + 1.0);
        let noise_metadata_schedule_294_0_e2823: f64 = (w[128] / noise_metadata_schedule_294_0_e2822);
        (noise_metadata_schedule_294_0_e2823,)
    } else {
        (w[115],)
    }
};
            w[115] = noise_metadata_schedule_294_0_e2825;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_295_0_e2839,) = {
    if ((w[476] == 0.0) && (w[483] == 0.0)) {
        let noise_metadata_schedule_295_0_e2834: f64 = (w[107] + w[231]);
        let noise_metadata_schedule_295_0_e2836: f64 = (noise_metadata_schedule_295_0_e2834 - w[230]);
        let noise_metadata_schedule_295_0_e2837: f64 = (w[107] / noise_metadata_schedule_295_0_e2836);
        (noise_metadata_schedule_295_0_e2837,)
    } else {
        (w[115],)
    }
};
            w[115] = noise_metadata_schedule_295_0_e2839;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_296_0_e2844,) = {
    if (w[476] == 0.0) {
        (w[236],)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_296_0_e2844;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_297_0_e2851,) = {
    if (w[476] == 0.0) {
        let noise_metadata_schedule_297_0_e2849: f64 = (0.1 * w[17]);
        (noise_metadata_schedule_297_0_e2849,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_297_0_e2851;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_298_0_e2856,) = {
    if (w[476] == 0.0) {
        (w[108],)
    } else {
        (w[127],)
    }
};
            w[127] = noise_metadata_schedule_298_0_e2856;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_299_0_e2865,) = {
    if (w[476] == 0.0) {
        let noise_metadata_schedule_299_0_e2862: f64 = (w[127] / params[61]);
        let noise_metadata_schedule_299_0_e2863: f64 = (1.0 - noise_metadata_schedule_299_0_e2862);
        (noise_metadata_schedule_299_0_e2863,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_299_0_e2865;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_300_0_e2870: f64 = (-1.0);
            let noise_metadata_schedule_300_0_e2872: f64 = (noise_metadata_schedule_300_0_e2870 / params[66]);
            let noise_metadata_schedule_300_0_e2873: f64 = (3.0_f64).powf(noise_metadata_schedule_300_0_e2872);
            let noise_metadata_schedule_300_0_e2874: f64 = (1.0 - noise_metadata_schedule_300_0_e2873);
            let noise_metadata_schedule_300_0_e2875: f64 = (w[14] * noise_metadata_schedule_300_0_e2874);
            w[129] = noise_metadata_schedule_300_0_e2875;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_301_0_e2878: f64 = (0.1 * w[14]);
            w[273] = noise_metadata_schedule_301_0_e2878;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_302_0_e2881: f64 = (w[232] - w[129]);
            let noise_metadata_schedule_302_0_e2883: f64 = (noise_metadata_schedule_302_0_e2881 / w[273]);
            w[259] = noise_metadata_schedule_302_0_e2883;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_303_0_e2886: f64 = if w[232] < w[129] { 1.0 } else { 0.0 };
            w[484] = noise_metadata_schedule_303_0_e2886;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_304_0_e2898,) = {
    if (w[484] != 0.0) {
        let noise_metadata_schedule_304_0_e2892: f64 = (w[259]).exp();
        let noise_metadata_schedule_304_0_e2893: f64 = (1.0 + noise_metadata_schedule_304_0_e2892);
        let noise_metadata_schedule_304_0_e2894: f64 = (noise_metadata_schedule_304_0_e2893).ln();
        let noise_metadata_schedule_304_0_e2895: f64 = (w[273] * noise_metadata_schedule_304_0_e2894);
        let noise_metadata_schedule_304_0_e2896: f64 = (w[232] - noise_metadata_schedule_304_0_e2895);
        (noise_metadata_schedule_304_0_e2896,)
    } else {
        (w[130],)
    }
};
            w[130] = noise_metadata_schedule_304_0_e2898;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_305_0_e2912,) = {
    if (w[484] == 0.0) {
        let noise_metadata_schedule_305_0_e2905: f64 = (-w[259]);
        let noise_metadata_schedule_305_0_e2906: f64 = (noise_metadata_schedule_305_0_e2905).exp();
        let noise_metadata_schedule_305_0_e2907: f64 = (1.0 + noise_metadata_schedule_305_0_e2906);
        let noise_metadata_schedule_305_0_e2908: f64 = (noise_metadata_schedule_305_0_e2907).ln();
        let noise_metadata_schedule_305_0_e2909: f64 = (w[273] * noise_metadata_schedule_305_0_e2908);
        let noise_metadata_schedule_305_0_e2910: f64 = (w[129] - noise_metadata_schedule_305_0_e2909);
        (noise_metadata_schedule_305_0_e2910,)
    } else {
        (w[130],)
    }
};
            w[130] = noise_metadata_schedule_305_0_e2912;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_306_0_e2916: f64 = (w[130] * w[65]);
            let noise_metadata_schedule_306_0_e2917: f64 = (1.0 - noise_metadata_schedule_306_0_e2916);
            let noise_metadata_schedule_306_0_e2920: f64 = (1.0 - params[66]);
            let noise_metadata_schedule_306_0_e2921: f64 = (noise_metadata_schedule_306_0_e2917).powf(noise_metadata_schedule_306_0_e2920);
            w[59] = noise_metadata_schedule_306_0_e2921;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_307_0_e2925: f64 = (1.0 - params[66]);
            let noise_metadata_schedule_307_0_e2926: f64 = (w[14] / noise_metadata_schedule_307_0_e2925);
            let noise_metadata_schedule_307_0_e2929: f64 = (1.0 - w[59]);
            let noise_metadata_schedule_307_0_e2930: f64 = (noise_metadata_schedule_307_0_e2926 * noise_metadata_schedule_307_0_e2929);
            let noise_metadata_schedule_307_0_e2934: f64 = (w[232] - w[130]);
            let noise_metadata_schedule_307_0_e2935: f64 = (3.0 * noise_metadata_schedule_307_0_e2934);
            let noise_metadata_schedule_307_0_e2936: f64 = (noise_metadata_schedule_307_0_e2930 + noise_metadata_schedule_307_0_e2935);
            w[131] = noise_metadata_schedule_307_0_e2936;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_308_0_e2939: f64 = if params[73] == 1.0 { 1.0 } else { 0.0 };
            w[485] = noise_metadata_schedule_308_0_e2939;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_309_0_e2943,) = {
    if (w[485] != 0.0) {
        (w[230],)
    } else {
        (w[132],)
    }
};
            w[132] = noise_metadata_schedule_309_0_e2943;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_310_0_e2946: f64 = if params[73] == 2.0 { 1.0 } else { 0.0 };
            w[486] = noise_metadata_schedule_310_0_e2946;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_311_0_e2955,) = {
    if ((w[485] == 0.0) && (w[486] != 0.0)) {
        let noise_metadata_schedule_311_0_e2953: f64 = (w[230] + w[125]);
        (noise_metadata_schedule_311_0_e2953,)
    } else {
        (w[132],)
    }
};
            w[132] = noise_metadata_schedule_311_0_e2955;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_312_0_e2963,) = {
    if ((w[485] == 0.0) && (w[486] == 0.0)) {
        (w[231],)
    } else {
        (w[132],)
    }
};
            w[132] = noise_metadata_schedule_312_0_e2963;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_313_0_e2966: f64 = (2.0 - w[25]);
            let noise_metadata_schedule_313_0_e2969: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_313_0_e2970: f64 = (noise_metadata_schedule_313_0_e2966 / noise_metadata_schedule_313_0_e2969);
            w[133] = noise_metadata_schedule_313_0_e2970;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_314_0_e2975: f64 = (-1.0);
            let noise_metadata_schedule_314_0_e2977: f64 = (noise_metadata_schedule_314_0_e2975 / params[71]);
            let noise_metadata_schedule_314_0_e2978: f64 = (w[133]).powf(noise_metadata_schedule_314_0_e2977);
            let noise_metadata_schedule_314_0_e2979: f64 = (1.0 - noise_metadata_schedule_314_0_e2978);
            let noise_metadata_schedule_314_0_e2980: f64 = (w[17] * noise_metadata_schedule_314_0_e2979);
            w[134] = noise_metadata_schedule_314_0_e2980;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_315_0_e2983: f64 = (w[132] - w[134]);
            let noise_metadata_schedule_315_0_e2985: f64 = (noise_metadata_schedule_315_0_e2983 / w[126]);
            w[259] = noise_metadata_schedule_315_0_e2985;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_316_0_e2988: f64 = if w[132] < w[134] { 1.0 } else { 0.0 };
            w[487] = noise_metadata_schedule_316_0_e2988;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_317_0_e3000,) = {
    if (w[487] != 0.0) {
        let noise_metadata_schedule_317_0_e2994: f64 = (w[259]).exp();
        let noise_metadata_schedule_317_0_e2995: f64 = (1.0 + noise_metadata_schedule_317_0_e2994);
        let noise_metadata_schedule_317_0_e2996: f64 = (noise_metadata_schedule_317_0_e2995).ln();
        let noise_metadata_schedule_317_0_e2997: f64 = (w[126] * noise_metadata_schedule_317_0_e2996);
        let noise_metadata_schedule_317_0_e2998: f64 = (w[132] - noise_metadata_schedule_317_0_e2997);
        (noise_metadata_schedule_317_0_e2998,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_317_0_e3000;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let (noise_metadata_schedule_318_0_e3014,) = {
    if (w[487] == 0.0) {
        let noise_metadata_schedule_318_0_e3007: f64 = (-w[259]);
        let noise_metadata_schedule_318_0_e3008: f64 = (noise_metadata_schedule_318_0_e3007).exp();
        let noise_metadata_schedule_318_0_e3009: f64 = (1.0 + noise_metadata_schedule_318_0_e3008);
        let noise_metadata_schedule_318_0_e3010: f64 = (noise_metadata_schedule_318_0_e3009).ln();
        let noise_metadata_schedule_318_0_e3011: f64 = (w[126] * noise_metadata_schedule_318_0_e3010);
        let noise_metadata_schedule_318_0_e3012: f64 = (w[134] - noise_metadata_schedule_318_0_e3011);
        (noise_metadata_schedule_318_0_e3012,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_318_0_e3014;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_319_0_e3017: f64 = (w[199]).powf(params[75]);
            w[136] = noise_metadata_schedule_319_0_e3017;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_320_0_e3021: f64 = (1.0 - params[71]);
            let noise_metadata_schedule_320_0_e3022: f64 = (w[17] / noise_metadata_schedule_320_0_e3021);
            let noise_metadata_schedule_320_0_e3028: f64 = (w[135] / w[17]);
            let noise_metadata_schedule_320_0_e3029: f64 = (1.0 - noise_metadata_schedule_320_0_e3028);
            let noise_metadata_schedule_320_0_e3032: f64 = (1.0 - params[71]);
            let noise_metadata_schedule_320_0_e3033: f64 = (noise_metadata_schedule_320_0_e3029).powf(noise_metadata_schedule_320_0_e3032);
            let noise_metadata_schedule_320_0_e3034: f64 = (w[136] * noise_metadata_schedule_320_0_e3033);
            let noise_metadata_schedule_320_0_e3035: f64 = (1.0 - noise_metadata_schedule_320_0_e3034);
            let noise_metadata_schedule_320_0_e3036: f64 = (noise_metadata_schedule_320_0_e3022 * noise_metadata_schedule_320_0_e3035);
            let noise_metadata_schedule_320_0_e3039: f64 = (w[136] * w[133]);
            let noise_metadata_schedule_320_0_e3042: f64 = (w[132] - w[135]);
            let noise_metadata_schedule_320_0_e3043: f64 = (noise_metadata_schedule_320_0_e3039 * noise_metadata_schedule_320_0_e3042);
            let noise_metadata_schedule_320_0_e3044: f64 = (noise_metadata_schedule_320_0_e3036 + noise_metadata_schedule_320_0_e3043);
            w[137] = noise_metadata_schedule_320_0_e3044;
        }
        if (active[0] & 0x1ffe7) != 0 {
            let noise_metadata_schedule_321_0_e3047: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_321_0_e3049: f64 = (noise_metadata_schedule_321_0_e3047 * w[137]);
            let noise_metadata_schedule_321_0_e3052: f64 = (w[25] * w[230]);
            let noise_metadata_schedule_321_0_e3053: f64 = (noise_metadata_schedule_321_0_e3049 + noise_metadata_schedule_321_0_e3052);
            w[138] = noise_metadata_schedule_321_0_e3053;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_322_0_e3056: f64 = (4.0 * w[35]);
            let noise_metadata_schedule_322_0_e3058: f64 = (noise_metadata_schedule_322_0_e3056 / w[36]);
            w[139] = noise_metadata_schedule_322_0_e3058;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_323_0_e3061: f64 = (w[139] * w[246]);
            w[140] = noise_metadata_schedule_323_0_e3061;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_324_0_e3066: f64 = (1.0 + w[140]);
            let noise_metadata_schedule_324_0_e3067: f64 = (noise_metadata_schedule_324_0_e3066).sqrt();
            let noise_metadata_schedule_324_0_e3068: f64 = (1.0 + noise_metadata_schedule_324_0_e3067);
            let noise_metadata_schedule_324_0_e3069: f64 = (w[140] / noise_metadata_schedule_324_0_e3068);
            w[142] = noise_metadata_schedule_324_0_e3069;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_325_0_e3073: f64 = (1.0 / w[49]);
            let noise_metadata_schedule_325_0_e3074: f64 = (w[121]).powf(noise_metadata_schedule_325_0_e3073);
            w[122] = noise_metadata_schedule_325_0_e3074;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_326_0_e3077: f64 = (w[139] * w[122]);
            w[141] = noise_metadata_schedule_326_0_e3077;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_327_0_e3082: f64 = (1.0 + w[141]);
            let noise_metadata_schedule_327_0_e3083: f64 = (noise_metadata_schedule_327_0_e3082).sqrt();
            let noise_metadata_schedule_327_0_e3084: f64 = (1.0 + noise_metadata_schedule_327_0_e3083);
            let noise_metadata_schedule_327_0_e3085: f64 = (w[141] / noise_metadata_schedule_327_0_e3084);
            w[143] = noise_metadata_schedule_327_0_e3085;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_328_0_e3088: f64 = if params[91] == 0.0 { 1.0 } else { 0.0 };
            w[488] = noise_metadata_schedule_328_0_e3088;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_329_0_e3100,) = {
    if (w[488] != 0.0) {
        let noise_metadata_schedule_329_0_e3093: f64 = (w[131] / w[41]);
        let noise_metadata_schedule_329_0_e3094: f64 = (1.0 + noise_metadata_schedule_329_0_e3093);
        let noise_metadata_schedule_329_0_e3097: f64 = (w[138] / w[40]);
        let noise_metadata_schedule_329_0_e3098: f64 = (noise_metadata_schedule_329_0_e3094 + noise_metadata_schedule_329_0_e3097);
        (noise_metadata_schedule_329_0_e3098,)
    } else {
        (w[144],)
    }
};
            w[144] = noise_metadata_schedule_329_0_e3100;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_330_0_e3113,) = {
    if (w[488] == 0.0) {
        let noise_metadata_schedule_330_0_e3105: f64 = (w[131] / w[41]);
        let noise_metadata_schedule_330_0_e3107: f64 = (noise_metadata_schedule_330_0_e3105 + 1.0);
        let noise_metadata_schedule_330_0_e3109: f64 = (noise_metadata_schedule_330_0_e3107 * w[99]);
        let noise_metadata_schedule_330_0_e3111: f64 = (noise_metadata_schedule_330_0_e3109 * w[8]);
        (noise_metadata_schedule_330_0_e3111,)
    } else {
        (w[269],)
    }
};
            w[269] = noise_metadata_schedule_330_0_e3113;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_331_0_e3125,) = {
    if (w[488] == 0.0) {
        let noise_metadata_schedule_331_0_e3117: f64 = (-w[138]);
        let noise_metadata_schedule_331_0_e3119: f64 = (noise_metadata_schedule_331_0_e3117 / w[40]);
        let noise_metadata_schedule_331_0_e3121: f64 = (noise_metadata_schedule_331_0_e3119 * w[99]);
        let noise_metadata_schedule_331_0_e3123: f64 = (noise_metadata_schedule_331_0_e3121 * w[8]);
        (noise_metadata_schedule_331_0_e3123,)
    } else {
        (w[270],)
    }
};
            w[270] = noise_metadata_schedule_331_0_e3125;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let (noise_metadata_schedule_332_0_e3141,) = {
    if (w[488] == 0.0) {
        let noise_metadata_schedule_332_0_e3129: f64 = (w[269]).exp();
        let noise_metadata_schedule_332_0_e3131: f64 = (w[270]).exp();
        let noise_metadata_schedule_332_0_e3132: f64 = (noise_metadata_schedule_332_0_e3129 - noise_metadata_schedule_332_0_e3131);
        let noise_metadata_schedule_332_0_e3135: f64 = (w[99] * w[8]);
        let noise_metadata_schedule_332_0_e3136: f64 = (noise_metadata_schedule_332_0_e3135).exp();
        let noise_metadata_schedule_332_0_e3138: f64 = (noise_metadata_schedule_332_0_e3136 - 1.0);
        let noise_metadata_schedule_332_0_e3139: f64 = (noise_metadata_schedule_332_0_e3132 / noise_metadata_schedule_332_0_e3138);
        (noise_metadata_schedule_332_0_e3139,)
    } else {
        (w[144],)
    }
};
            w[144] = noise_metadata_schedule_332_0_e3141;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_333_0_e3144: f64 = (0.1 * 0.1);
            w[261] = noise_metadata_schedule_333_0_e3144;
        }
        if (active[0] & 0x1ffc7) != 0 {
            let noise_metadata_schedule_334_0_e3147: f64 = (w[144] * w[144]);
            w[262] = noise_metadata_schedule_334_0_e3147;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_335_0_e3150: f64 = if w[144] < 0.0 { 1.0 } else { 0.0 };
            w[489] = noise_metadata_schedule_335_0_e3150;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_336_0_e3163,) = {
    if (w[489] != 0.0) {
        let noise_metadata_schedule_336_0_e3154: f64 = (0.5 * w[261]);
        let noise_metadata_schedule_336_0_e3157: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_336_0_e3158: f64 = (noise_metadata_schedule_336_0_e3157).sqrt();
        let noise_metadata_schedule_336_0_e3160: f64 = (noise_metadata_schedule_336_0_e3158 - w[144]);
        let noise_metadata_schedule_336_0_e3161: f64 = (noise_metadata_schedule_336_0_e3154 / noise_metadata_schedule_336_0_e3160);
        (noise_metadata_schedule_336_0_e3161,)
    } else {
        (w[145],)
    }
};
            w[145] = noise_metadata_schedule_336_0_e3163;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_337_0_e3175,) = {
    if (w[489] == 0.0) {
        let noise_metadata_schedule_337_0_e3169: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_337_0_e3170: f64 = (noise_metadata_schedule_337_0_e3169).sqrt();
        let noise_metadata_schedule_337_0_e3172: f64 = (noise_metadata_schedule_337_0_e3170 + w[144]);
        let noise_metadata_schedule_337_0_e3173: f64 = (0.5 * noise_metadata_schedule_337_0_e3172);
        (noise_metadata_schedule_337_0_e3173,)
    } else {
        (w[145],)
    }
};
            w[145] = noise_metadata_schedule_337_0_e3175;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_338_0_e3181: f64 = (w[142] + w[143]);
            let noise_metadata_schedule_338_0_e3182: f64 = (0.5 * noise_metadata_schedule_338_0_e3181);
            let noise_metadata_schedule_338_0_e3183: f64 = (1.0 + noise_metadata_schedule_338_0_e3182);
            let noise_metadata_schedule_338_0_e3184: f64 = (w[145] * noise_metadata_schedule_338_0_e3183);
            w[146] = noise_metadata_schedule_338_0_e3184;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_339_0_e3187: f64 = (params[14] * w[35]);
            let noise_metadata_schedule_339_0_e3189: f64 = (noise_metadata_schedule_339_0_e3187 * w[122]);
            w[147] = noise_metadata_schedule_339_0_e3189;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_340_0_e3192: f64 = (w[35] * w[246]);
            w[148] = noise_metadata_schedule_340_0_e3192;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_341_0_e3195: f64 = (w[148] - w[147]);
            let noise_metadata_schedule_341_0_e3197: f64 = (noise_metadata_schedule_341_0_e3195 / w[146]);
            w[149] = noise_metadata_schedule_341_0_e3197;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_342_0_e3200: f64 = w[232];
            let noise_metadata_schedule_342_0_e3202: f64 = (noise_metadata_schedule_342_0_e3200 / 0.0001);
            w[259] = noise_metadata_schedule_342_0_e3202;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_343_0_e3205: f64 = if w[232] < 0.0 { 1.0 } else { 0.0 };
            w[490] = noise_metadata_schedule_343_0_e3205;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_344_0_e3217,) = {
    if (w[490] != 0.0) {
        let noise_metadata_schedule_344_0_e3211: f64 = (w[259]).exp();
        let noise_metadata_schedule_344_0_e3212: f64 = (1.0 + noise_metadata_schedule_344_0_e3211);
        let noise_metadata_schedule_344_0_e3213: f64 = (noise_metadata_schedule_344_0_e3212).ln();
        let noise_metadata_schedule_344_0_e3214: f64 = (0.0001 * noise_metadata_schedule_344_0_e3213);
        let noise_metadata_schedule_344_0_e3215: f64 = noise_metadata_schedule_344_0_e3214;
        (noise_metadata_schedule_344_0_e3215,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_344_0_e3217;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_345_0_e3231,) = {
    if (w[490] == 0.0) {
        let noise_metadata_schedule_345_0_e3224: f64 = (-w[259]);
        let noise_metadata_schedule_345_0_e3225: f64 = (noise_metadata_schedule_345_0_e3224).exp();
        let noise_metadata_schedule_345_0_e3226: f64 = (1.0 + noise_metadata_schedule_345_0_e3225);
        let noise_metadata_schedule_345_0_e3227: f64 = (noise_metadata_schedule_345_0_e3226).ln();
        let noise_metadata_schedule_345_0_e3228: f64 = (0.0001 * noise_metadata_schedule_345_0_e3227);
        let noise_metadata_schedule_345_0_e3229: f64 = (w[232] + noise_metadata_schedule_345_0_e3228);
        (noise_metadata_schedule_345_0_e3229,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_345_0_e3231;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_346_0_e3234: f64 = (w[276] / params[139]);
            w[278] = noise_metadata_schedule_346_0_e3234;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_347_0_e3237: f64 = if w[278] < params[134] { 1.0 } else { 0.0 };
            w[491] = noise_metadata_schedule_347_0_e3237;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_348_0_e3242,) = {
    if (w[491] != 0.0) {
        let noise_metadata_schedule_348_0_e3240: f64 = (w[278]).exp();
        (noise_metadata_schedule_348_0_e3240,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_348_0_e3242;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_349_0_e3248,) = {
    if (w[491] == 0.0) {
        let noise_metadata_schedule_349_0_e3246: f64 = (params[134]).exp();
        (noise_metadata_schedule_349_0_e3246,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_349_0_e3248;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_350_0_e3259,) = {
    if (w[491] == 0.0) {
        let noise_metadata_schedule_350_0_e3255: f64 = (w[278] - params[134]);
        let noise_metadata_schedule_350_0_e3256: f64 = (1.0 + noise_metadata_schedule_350_0_e3255);
        let noise_metadata_schedule_350_0_e3257: f64 = (w[275] * noise_metadata_schedule_350_0_e3256);
        (noise_metadata_schedule_350_0_e3257,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_350_0_e3259;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_351_0_e3263: f64 = (w[279] - 1.0);
            let noise_metadata_schedule_351_0_e3264: f64 = (w[325] * noise_metadata_schedule_351_0_e3263);
            w[326] = noise_metadata_schedule_351_0_e3264;
        }
        if (active[0] & 0x6) != 0 {
            let noise_metadata_schedule_352_0_e3267: f64 = (w[232] - params[141]);
            let noise_metadata_schedule_352_0_e3269: f64 = (noise_metadata_schedule_352_0_e3267 / 0.001);
            w[259] = noise_metadata_schedule_352_0_e3269;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_353_0_e3272: f64 = if w[232] < params[141] { 1.0 } else { 0.0 };
            w[492] = noise_metadata_schedule_353_0_e3272;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_354_0_e3284,) = {
    if (w[492] != 0.0) {
        let noise_metadata_schedule_354_0_e3278: f64 = (w[259]).exp();
        let noise_metadata_schedule_354_0_e3279: f64 = (1.0 + noise_metadata_schedule_354_0_e3278);
        let noise_metadata_schedule_354_0_e3280: f64 = (noise_metadata_schedule_354_0_e3279).ln();
        let noise_metadata_schedule_354_0_e3281: f64 = (0.001 * noise_metadata_schedule_354_0_e3280);
        let noise_metadata_schedule_354_0_e3282: f64 = (w[232] - noise_metadata_schedule_354_0_e3281);
        (noise_metadata_schedule_354_0_e3282,)
    } else {
        (w[280],)
    }
};
            w[280] = noise_metadata_schedule_354_0_e3284;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_355_0_e3298,) = {
    if (w[492] == 0.0) {
        let noise_metadata_schedule_355_0_e3291: f64 = (-w[259]);
        let noise_metadata_schedule_355_0_e3292: f64 = (noise_metadata_schedule_355_0_e3291).exp();
        let noise_metadata_schedule_355_0_e3293: f64 = (1.0 + noise_metadata_schedule_355_0_e3292);
        let noise_metadata_schedule_355_0_e3294: f64 = (noise_metadata_schedule_355_0_e3293).ln();
        let noise_metadata_schedule_355_0_e3295: f64 = (0.001 * noise_metadata_schedule_355_0_e3294);
        let noise_metadata_schedule_355_0_e3296: f64 = (params[141] - noise_metadata_schedule_355_0_e3295);
        (noise_metadata_schedule_355_0_e3296,)
    } else {
        (w[280],)
    }
};
            w[280] = noise_metadata_schedule_355_0_e3298;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_356_0_e3301: f64 = (params[142] * w[280]);
            let noise_metadata_schedule_356_0_e3304: f64 = (params[141] - w[280]);
            let noise_metadata_schedule_356_0_e3306: f64 = {let pb=noise_metadata_schedule_356_0_e3304;pb*pb};
            let noise_metadata_schedule_356_0_e3307: f64 = (noise_metadata_schedule_356_0_e3301 * noise_metadata_schedule_356_0_e3306);
            w[327] = noise_metadata_schedule_356_0_e3307;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_357_0_e3310: f64 = (w[232] * w[8]);
            let noise_metadata_schedule_357_0_e3312: f64 = (noise_metadata_schedule_357_0_e3310 / params[16]);
            let noise_metadata_schedule_357_0_e3314: f64 = if noise_metadata_schedule_357_0_e3312 < params[134] { 1.0 } else { 0.0 };
            w[493] = noise_metadata_schedule_357_0_e3314;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_358_0_e3323,) = {
    if (w[493] != 0.0) {
        let noise_metadata_schedule_358_0_e3318: f64 = (w[232] * w[8]);
        let noise_metadata_schedule_358_0_e3320: f64 = (noise_metadata_schedule_358_0_e3318 / params[16]);
        let noise_metadata_schedule_358_0_e3321: f64 = (noise_metadata_schedule_358_0_e3320).exp();
        (noise_metadata_schedule_358_0_e3321,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_358_0_e3323;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_359_0_e3329,) = {
    if (w[493] == 0.0) {
        let noise_metadata_schedule_359_0_e3327: f64 = (params[134]).exp();
        (noise_metadata_schedule_359_0_e3327,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_359_0_e3329;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_360_0_e3344,) = {
    if (w[493] == 0.0) {
        let noise_metadata_schedule_360_0_e3336: f64 = (w[232] * w[8]);
        let noise_metadata_schedule_360_0_e3338: f64 = (noise_metadata_schedule_360_0_e3336 / params[16]);
        let noise_metadata_schedule_360_0_e3340: f64 = (noise_metadata_schedule_360_0_e3338 - params[134]);
        let noise_metadata_schedule_360_0_e3341: f64 = (1.0 + noise_metadata_schedule_360_0_e3340);
        let noise_metadata_schedule_360_0_e3342: f64 = (w[275] * noise_metadata_schedule_360_0_e3341);
        (noise_metadata_schedule_360_0_e3342,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_360_0_e3344;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_361_0_e3347: f64 = if params[23] == 1.0 { 1.0 } else { 0.0 };
            w[494] = noise_metadata_schedule_361_0_e3347;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_362_0_e3350: f64 = (w[232] - w[55]);
            let noise_metadata_schedule_362_0_e3352: f64 = (noise_metadata_schedule_362_0_e3350 * w[8]);
            let noise_metadata_schedule_362_0_e3354: f64 = if noise_metadata_schedule_362_0_e3352 < params[134] { 1.0 } else { 0.0 };
            w[495] = noise_metadata_schedule_362_0_e3354;
        }
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_363_0_e3365,) = {
    if ((w[494] != 0.0) && (w[495] != 0.0)) {
        let noise_metadata_schedule_363_0_e3360: f64 = (w[232] - w[55]);
        let noise_metadata_schedule_363_0_e3362: f64 = (noise_metadata_schedule_363_0_e3360 * w[8]);
        let noise_metadata_schedule_363_0_e3363: f64 = (noise_metadata_schedule_363_0_e3362).exp();
        (noise_metadata_schedule_363_0_e3363,)
    } else {
        (w[278],)
    }
};
            w[278] = noise_metadata_schedule_363_0_e3365;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_364_0_e3373,) = {
    if ((w[494] != 0.0) && (w[495] == 0.0)) {
        let noise_metadata_schedule_364_0_e3371: f64 = (params[134]).exp();
        (noise_metadata_schedule_364_0_e3371,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_364_0_e3373;
        }
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_365_0_e3390,) = {
    if ((w[494] != 0.0) && (w[495] == 0.0)) {
        let noise_metadata_schedule_365_0_e3382: f64 = (w[232] - w[55]);
        let noise_metadata_schedule_365_0_e3384: f64 = (noise_metadata_schedule_365_0_e3382 * w[8]);
        let noise_metadata_schedule_365_0_e3386: f64 = (noise_metadata_schedule_365_0_e3384 - params[134]);
        let noise_metadata_schedule_365_0_e3387: f64 = (1.0 + noise_metadata_schedule_365_0_e3386);
        let noise_metadata_schedule_365_0_e3388: f64 = (w[275] * noise_metadata_schedule_365_0_e3387);
        (noise_metadata_schedule_365_0_e3388,)
    } else {
        (w[278],)
    }
};
            w[278] = noise_metadata_schedule_365_0_e3390;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_366_0_e3393: f64 = (w[149] / w[35]);
            let noise_metadata_schedule_366_0_e3395: f64 = (noise_metadata_schedule_366_0_e3393 - 1000.0);
            let noise_metadata_schedule_366_0_e3397: f64 = if noise_metadata_schedule_366_0_e3395 < 40.0 { 1.0 } else { 0.0 };
            w[496] = noise_metadata_schedule_366_0_e3397;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_367_0_e3408,) = {
    if ((w[494] != 0.0) && (w[496] != 0.0)) {
        let noise_metadata_schedule_367_0_e3403: f64 = (w[149] / w[35]);
        let noise_metadata_schedule_367_0_e3405: f64 = (noise_metadata_schedule_367_0_e3403 - 1000.0);
        let noise_metadata_schedule_367_0_e3406: f64 = (noise_metadata_schedule_367_0_e3405).exp();
        (noise_metadata_schedule_367_0_e3406,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_367_0_e3408;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_368_0_e3416,) = {
    if ((w[494] != 0.0) && (w[496] == 0.0)) {
        let noise_metadata_schedule_368_0_e3414: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_368_0_e3414,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_368_0_e3416;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_369_0_e3433,) = {
    if ((w[494] != 0.0) && (w[496] == 0.0)) {
        let noise_metadata_schedule_369_0_e3425: f64 = (w[149] / w[35]);
        let noise_metadata_schedule_369_0_e3427: f64 = (noise_metadata_schedule_369_0_e3425 - 1000.0);
        let noise_metadata_schedule_369_0_e3429: f64 = (noise_metadata_schedule_369_0_e3427 - 40.0);
        let noise_metadata_schedule_369_0_e3430: f64 = (1.0 + noise_metadata_schedule_369_0_e3429);
        let noise_metadata_schedule_369_0_e3431: f64 = (w[275] * noise_metadata_schedule_369_0_e3430);
        (noise_metadata_schedule_369_0_e3431,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_369_0_e3433;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_370_0_e3476,) = {
    if (w[494] != 0.0) {
        let noise_metadata_schedule_370_0_e3438: f64 = (w[276] - 1.0);
        let noise_metadata_schedule_370_0_e3439: f64 = (w[42] * noise_metadata_schedule_370_0_e3438);
        let noise_metadata_schedule_370_0_e3442: f64 = (w[53] * 2.0);
        let noise_metadata_schedule_370_0_e3445: f64 = (w[276] - 1.0);
        let noise_metadata_schedule_370_0_e3446: f64 = (noise_metadata_schedule_370_0_e3442 * noise_metadata_schedule_370_0_e3445);
        let noise_metadata_schedule_370_0_e3451: f64 = (4.0 * w[278]);
        let noise_metadata_schedule_370_0_e3452: f64 = (1.0 + noise_metadata_schedule_370_0_e3451);
        let noise_metadata_schedule_370_0_e3453: f64 = (noise_metadata_schedule_370_0_e3452).sqrt();
        let noise_metadata_schedule_370_0_e3454: f64 = (1.0 + noise_metadata_schedule_370_0_e3453);
        let noise_metadata_schedule_370_0_e3455: f64 = (noise_metadata_schedule_370_0_e3446 / noise_metadata_schedule_370_0_e3454);
        let noise_metadata_schedule_370_0_e3459: f64 = (w[138] / w[40]);
        let noise_metadata_schedule_370_0_e3460: f64 = (1.0 + noise_metadata_schedule_370_0_e3459);
        let noise_metadata_schedule_370_0_e3461: f64 = (noise_metadata_schedule_370_0_e3455 * noise_metadata_schedule_370_0_e3460);
        let noise_metadata_schedule_370_0_e3462: f64 = (noise_metadata_schedule_370_0_e3439 + noise_metadata_schedule_370_0_e3461);
        let noise_metadata_schedule_370_0_e3466: f64 = (w[121] - 1.0);
        let noise_metadata_schedule_370_0_e3467: f64 = (w[54] * noise_metadata_schedule_370_0_e3466);
        let noise_metadata_schedule_370_0_e3469: f64 = (noise_metadata_schedule_370_0_e3467 * w[279]);
        let noise_metadata_schedule_370_0_e3472: f64 = (1.0 + w[279]);
        let noise_metadata_schedule_370_0_e3473: f64 = (noise_metadata_schedule_370_0_e3469 / noise_metadata_schedule_370_0_e3472);
        let noise_metadata_schedule_370_0_e3474: f64 = (noise_metadata_schedule_370_0_e3462 + noise_metadata_schedule_370_0_e3473);
        (noise_metadata_schedule_370_0_e3474,)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_370_0_e3476;
        }
        if (active[0] & 0x44) != 0 {
            let noise_metadata_schedule_371_0_e3479: f64 = if params[92] == 0.0 { 1.0 } else { 0.0 };
            w[497] = noise_metadata_schedule_371_0_e3479;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_372_0_e3490,) = {
    if ((w[494] == 0.0) && (w[497] != 0.0)) {
        let noise_metadata_schedule_372_0_e3487: f64 = (w[276] - 1.0);
        let noise_metadata_schedule_372_0_e3488: f64 = (w[42] * noise_metadata_schedule_372_0_e3487);
        (noise_metadata_schedule_372_0_e3488,)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_372_0_e3490;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_373_0_e3520,) = {
    if ((w[494] == 0.0) && (w[497] == 0.0)) {
        let noise_metadata_schedule_373_0_e3499: f64 = (1.0 - params[92]);
        let noise_metadata_schedule_373_0_e3502: f64 = (w[276] - 1.0);
        let noise_metadata_schedule_373_0_e3503: f64 = (noise_metadata_schedule_373_0_e3499 * noise_metadata_schedule_373_0_e3502);
        let noise_metadata_schedule_373_0_e3507: f64 = (w[276] + w[121]);
        let noise_metadata_schedule_373_0_e3509: f64 = (noise_metadata_schedule_373_0_e3507 - 2.0);
        let noise_metadata_schedule_373_0_e3510: f64 = (params[92] * noise_metadata_schedule_373_0_e3509);
        let noise_metadata_schedule_373_0_e3514: f64 = (w[138] / w[40]);
        let noise_metadata_schedule_373_0_e3515: f64 = (1.0 + noise_metadata_schedule_373_0_e3514);
        let noise_metadata_schedule_373_0_e3516: f64 = (noise_metadata_schedule_373_0_e3510 * noise_metadata_schedule_373_0_e3515);
        let noise_metadata_schedule_373_0_e3517: f64 = (noise_metadata_schedule_373_0_e3503 + noise_metadata_schedule_373_0_e3516);
        let noise_metadata_schedule_373_0_e3518: f64 = (w[42] * noise_metadata_schedule_373_0_e3517);
        (noise_metadata_schedule_373_0_e3518,)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_373_0_e3520;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_374_0_e3523: f64 = (w[233] * w[8]);
            let noise_metadata_schedule_374_0_e3525: f64 = (noise_metadata_schedule_374_0_e3523 / params[18]);
            let noise_metadata_schedule_374_0_e3527: f64 = if noise_metadata_schedule_374_0_e3525 < params[134] { 1.0 } else { 0.0 };
            w[498] = noise_metadata_schedule_374_0_e3527;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_375_0_e3536,) = {
    if (w[498] != 0.0) {
        let noise_metadata_schedule_375_0_e3531: f64 = (w[233] * w[8]);
        let noise_metadata_schedule_375_0_e3533: f64 = (noise_metadata_schedule_375_0_e3531 / params[18]);
        let noise_metadata_schedule_375_0_e3534: f64 = (noise_metadata_schedule_375_0_e3533).exp();
        (noise_metadata_schedule_375_0_e3534,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_375_0_e3536;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_376_0_e3542,) = {
    if (w[498] == 0.0) {
        let noise_metadata_schedule_376_0_e3540: f64 = (params[134]).exp();
        (noise_metadata_schedule_376_0_e3540,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_376_0_e3542;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_377_0_e3557,) = {
    if (w[498] == 0.0) {
        let noise_metadata_schedule_377_0_e3549: f64 = (w[233] * w[8]);
        let noise_metadata_schedule_377_0_e3551: f64 = (noise_metadata_schedule_377_0_e3549 / params[18]);
        let noise_metadata_schedule_377_0_e3553: f64 = (noise_metadata_schedule_377_0_e3551 - params[134]);
        let noise_metadata_schedule_377_0_e3554: f64 = (1.0 + noise_metadata_schedule_377_0_e3553);
        let noise_metadata_schedule_377_0_e3555: f64 = (w[275] * noise_metadata_schedule_377_0_e3554);
        (noise_metadata_schedule_377_0_e3555,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_377_0_e3557;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_378_0_e3560: f64 = if params[23] == 1.0 { 1.0 } else { 0.0 };
            w[499] = noise_metadata_schedule_378_0_e3560;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_379_0_e3563: f64 = (w[233] - w[55]);
            let noise_metadata_schedule_379_0_e3565: f64 = (noise_metadata_schedule_379_0_e3563 * w[8]);
            let noise_metadata_schedule_379_0_e3567: f64 = if noise_metadata_schedule_379_0_e3565 < params[134] { 1.0 } else { 0.0 };
            w[500] = noise_metadata_schedule_379_0_e3567;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_380_0_e3578,) = {
    if ((w[499] != 0.0) && (w[500] != 0.0)) {
        let noise_metadata_schedule_380_0_e3573: f64 = (w[233] - w[55]);
        let noise_metadata_schedule_380_0_e3575: f64 = (noise_metadata_schedule_380_0_e3573 * w[8]);
        let noise_metadata_schedule_380_0_e3576: f64 = (noise_metadata_schedule_380_0_e3575).exp();
        (noise_metadata_schedule_380_0_e3576,)
    } else {
        (w[278],)
    }
};
            w[278] = noise_metadata_schedule_380_0_e3578;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_381_0_e3586,) = {
    if ((w[499] != 0.0) && (w[500] == 0.0)) {
        let noise_metadata_schedule_381_0_e3584: f64 = (params[134]).exp();
        (noise_metadata_schedule_381_0_e3584,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_381_0_e3586;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_382_0_e3603,) = {
    if ((w[499] != 0.0) && (w[500] == 0.0)) {
        let noise_metadata_schedule_382_0_e3595: f64 = (w[233] - w[55]);
        let noise_metadata_schedule_382_0_e3597: f64 = (noise_metadata_schedule_382_0_e3595 * w[8]);
        let noise_metadata_schedule_382_0_e3599: f64 = (noise_metadata_schedule_382_0_e3597 - params[134]);
        let noise_metadata_schedule_382_0_e3600: f64 = (1.0 + noise_metadata_schedule_382_0_e3599);
        let noise_metadata_schedule_382_0_e3601: f64 = (w[275] * noise_metadata_schedule_382_0_e3600);
        (noise_metadata_schedule_382_0_e3601,)
    } else {
        (w[278],)
    }
};
            w[278] = noise_metadata_schedule_382_0_e3603;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_383_0_e3628,) = {
    if (w[499] != 0.0) {
        let noise_metadata_schedule_383_0_e3608: f64 = (w[276] - 1.0);
        let noise_metadata_schedule_383_0_e3609: f64 = (w[44] * noise_metadata_schedule_383_0_e3608);
        let noise_metadata_schedule_383_0_e3612: f64 = (w[45] * 2.0);
        let noise_metadata_schedule_383_0_e3615: f64 = (w[276] - 1.0);
        let noise_metadata_schedule_383_0_e3616: f64 = (noise_metadata_schedule_383_0_e3612 * noise_metadata_schedule_383_0_e3615);
        let noise_metadata_schedule_383_0_e3621: f64 = (4.0 * w[278]);
        let noise_metadata_schedule_383_0_e3622: f64 = (1.0 + noise_metadata_schedule_383_0_e3621);
        let noise_metadata_schedule_383_0_e3623: f64 = (noise_metadata_schedule_383_0_e3622).sqrt();
        let noise_metadata_schedule_383_0_e3624: f64 = (1.0 + noise_metadata_schedule_383_0_e3623);
        let noise_metadata_schedule_383_0_e3625: f64 = (noise_metadata_schedule_383_0_e3616 / noise_metadata_schedule_383_0_e3624);
        let noise_metadata_schedule_383_0_e3626: f64 = (noise_metadata_schedule_383_0_e3609 + noise_metadata_schedule_383_0_e3625);
        (noise_metadata_schedule_383_0_e3626,)
    } else {
        (w[152],)
    }
};
            w[152] = noise_metadata_schedule_383_0_e3628;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_384_0_e3637,) = {
    if (w[499] == 0.0) {
        let noise_metadata_schedule_384_0_e3634: f64 = (w[276] - 1.0);
        let noise_metadata_schedule_384_0_e3635: f64 = (w[44] * noise_metadata_schedule_384_0_e3634);
        (noise_metadata_schedule_384_0_e3635,)
    } else {
        (w[152],)
    }
};
            w[152] = noise_metadata_schedule_384_0_e3637;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_385_0_e3640: f64 = (w[232] * w[8]);
            let noise_metadata_schedule_385_0_e3642: f64 = (noise_metadata_schedule_385_0_e3640 / params[20]);
            let noise_metadata_schedule_385_0_e3644: f64 = if noise_metadata_schedule_385_0_e3642 < params[134] { 1.0 } else { 0.0 };
            w[501] = noise_metadata_schedule_385_0_e3644;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_386_0_e3653,) = {
    if (w[501] != 0.0) {
        let noise_metadata_schedule_386_0_e3648: f64 = (w[232] * w[8]);
        let noise_metadata_schedule_386_0_e3650: f64 = (noise_metadata_schedule_386_0_e3648 / params[20]);
        let noise_metadata_schedule_386_0_e3651: f64 = (noise_metadata_schedule_386_0_e3650).exp();
        (noise_metadata_schedule_386_0_e3651,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_386_0_e3653;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_387_0_e3659,) = {
    if (w[501] == 0.0) {
        let noise_metadata_schedule_387_0_e3657: f64 = (params[134]).exp();
        (noise_metadata_schedule_387_0_e3657,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_387_0_e3659;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_388_0_e3674,) = {
    if (w[501] == 0.0) {
        let noise_metadata_schedule_388_0_e3666: f64 = (w[232] * w[8]);
        let noise_metadata_schedule_388_0_e3668: f64 = (noise_metadata_schedule_388_0_e3666 / params[20]);
        let noise_metadata_schedule_388_0_e3670: f64 = (noise_metadata_schedule_388_0_e3668 - params[134]);
        let noise_metadata_schedule_388_0_e3671: f64 = (1.0 + noise_metadata_schedule_388_0_e3670);
        let noise_metadata_schedule_388_0_e3672: f64 = (w[275] * noise_metadata_schedule_388_0_e3671);
        (noise_metadata_schedule_388_0_e3672,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_388_0_e3674;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_389_0_e3678: f64 = (w[276] - 1.0);
            let noise_metadata_schedule_389_0_e3679: f64 = (w[38] * noise_metadata_schedule_389_0_e3678);
            w[153] = noise_metadata_schedule_389_0_e3679;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_390_0_e3682: f64 = (w[233] * w[8]);
            let noise_metadata_schedule_390_0_e3684: f64 = (noise_metadata_schedule_390_0_e3682 / params[22]);
            let noise_metadata_schedule_390_0_e3686: f64 = if noise_metadata_schedule_390_0_e3684 < params[134] { 1.0 } else { 0.0 };
            w[502] = noise_metadata_schedule_390_0_e3686;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_391_0_e3695,) = {
    if (w[502] != 0.0) {
        let noise_metadata_schedule_391_0_e3690: f64 = (w[233] * w[8]);
        let noise_metadata_schedule_391_0_e3692: f64 = (noise_metadata_schedule_391_0_e3690 / params[22]);
        let noise_metadata_schedule_391_0_e3693: f64 = (noise_metadata_schedule_391_0_e3692).exp();
        (noise_metadata_schedule_391_0_e3693,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_391_0_e3695;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_392_0_e3701,) = {
    if (w[502] == 0.0) {
        let noise_metadata_schedule_392_0_e3699: f64 = (params[134]).exp();
        (noise_metadata_schedule_392_0_e3699,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_392_0_e3701;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_393_0_e3716,) = {
    if (w[502] == 0.0) {
        let noise_metadata_schedule_393_0_e3708: f64 = (w[233] * w[8]);
        let noise_metadata_schedule_393_0_e3710: f64 = (noise_metadata_schedule_393_0_e3708 / params[22]);
        let noise_metadata_schedule_393_0_e3712: f64 = (noise_metadata_schedule_393_0_e3710 - params[134]);
        let noise_metadata_schedule_393_0_e3713: f64 = (1.0 + noise_metadata_schedule_393_0_e3712);
        let noise_metadata_schedule_393_0_e3714: f64 = (w[275] * noise_metadata_schedule_393_0_e3713);
        (noise_metadata_schedule_393_0_e3714,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_393_0_e3716;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_394_0_e3720: f64 = (w[276] - 1.0);
            let noise_metadata_schedule_394_0_e3721: f64 = (w[46] * noise_metadata_schedule_394_0_e3720);
            w[155] = noise_metadata_schedule_394_0_e3721;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_395_0_e3724: f64 = (w[235] * w[8]);
            let noise_metadata_schedule_395_0_e3726: f64 = (noise_metadata_schedule_395_0_e3724 / params[31]);
            let noise_metadata_schedule_395_0_e3728: f64 = if noise_metadata_schedule_395_0_e3726 < params[134] { 1.0 } else { 0.0 };
            w[503] = noise_metadata_schedule_395_0_e3728;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_396_0_e3737,) = {
    if (w[503] != 0.0) {
        let noise_metadata_schedule_396_0_e3732: f64 = (w[235] * w[8]);
        let noise_metadata_schedule_396_0_e3734: f64 = (noise_metadata_schedule_396_0_e3732 / params[31]);
        let noise_metadata_schedule_396_0_e3735: f64 = (noise_metadata_schedule_396_0_e3734).exp();
        (noise_metadata_schedule_396_0_e3735,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_396_0_e3737;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_397_0_e3743,) = {
    if (w[503] == 0.0) {
        let noise_metadata_schedule_397_0_e3741: f64 = (params[134]).exp();
        (noise_metadata_schedule_397_0_e3741,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_397_0_e3743;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_398_0_e3758,) = {
    if (w[503] == 0.0) {
        let noise_metadata_schedule_398_0_e3750: f64 = (w[235] * w[8]);
        let noise_metadata_schedule_398_0_e3752: f64 = (noise_metadata_schedule_398_0_e3750 / params[31]);
        let noise_metadata_schedule_398_0_e3754: f64 = (noise_metadata_schedule_398_0_e3752 - params[134]);
        let noise_metadata_schedule_398_0_e3755: f64 = (1.0 + noise_metadata_schedule_398_0_e3754);
        let noise_metadata_schedule_398_0_e3756: f64 = (w[275] * noise_metadata_schedule_398_0_e3755);
        (noise_metadata_schedule_398_0_e3756,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_398_0_e3758;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_399_0_e3762: f64 = (w[276] - 1.0);
            let noise_metadata_schedule_399_0_e3763: f64 = (w[39] * noise_metadata_schedule_399_0_e3762);
            w[154] = noise_metadata_schedule_399_0_e3763;
        }
        if (active[0] & 0x18186) != 0 {
            let noise_metadata_schedule_400_0_e3766: f64 = (w[233] * w[8]);
            let noise_metadata_schedule_400_0_e3768: f64 = (noise_metadata_schedule_400_0_e3766 / params[133]);
            let noise_metadata_schedule_400_0_e3770: f64 = if noise_metadata_schedule_400_0_e3768 < params[134] { 1.0 } else { 0.0 };
            w[504] = noise_metadata_schedule_400_0_e3770;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_401_0_e3779,) = {
    if (w[504] != 0.0) {
        let noise_metadata_schedule_401_0_e3774: f64 = (w[233] * w[8]);
        let noise_metadata_schedule_401_0_e3776: f64 = (noise_metadata_schedule_401_0_e3774 / params[133]);
        let noise_metadata_schedule_401_0_e3777: f64 = (noise_metadata_schedule_401_0_e3776).exp();
        (noise_metadata_schedule_401_0_e3777,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_401_0_e3779;
        }
        if (active[0] & 0x18186) != 0 {
            let (noise_metadata_schedule_402_0_e3785,) = {
    if (w[504] == 0.0) {
        let noise_metadata_schedule_402_0_e3783: f64 = (params[134]).exp();
        (noise_metadata_schedule_402_0_e3783,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_402_0_e3785;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_403_0_e3800,) = {
    if (w[504] == 0.0) {
        let noise_metadata_schedule_403_0_e3792: f64 = (w[233] * w[8]);
        let noise_metadata_schedule_403_0_e3794: f64 = (noise_metadata_schedule_403_0_e3792 / params[133]);
        let noise_metadata_schedule_403_0_e3796: f64 = (noise_metadata_schedule_403_0_e3794 - params[134]);
        let noise_metadata_schedule_403_0_e3797: f64 = (1.0 + noise_metadata_schedule_403_0_e3796);
        let noise_metadata_schedule_403_0_e3798: f64 = (w[275] * noise_metadata_schedule_403_0_e3797);
        (noise_metadata_schedule_403_0_e3798,)
    } else {
        (w[276],)
    }
};
            w[276] = noise_metadata_schedule_403_0_e3800;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_404_0_e3804: f64 = (w[276] - 1.0);
            let noise_metadata_schedule_404_0_e3805: f64 = (w[47] * noise_metadata_schedule_404_0_e3804);
            w[156] = noise_metadata_schedule_404_0_e3805;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_405_0_e3816: f64 = if (((params[33] > 0.0) && (params[34] > 0.0)) && (w[232] < 0.0)) { 1.0 } else { 0.0 };
            w[505] = noise_metadata_schedule_405_0_e3816;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_406_0_e3822: f64 = (2.0 * w[59]);
            let noise_metadata_schedule_406_0_e3823: f64 = (w[62] / noise_metadata_schedule_406_0_e3822);
            let noise_metadata_schedule_406_0_e3824: f64 = (1.0 - noise_metadata_schedule_406_0_e3823);
            let noise_metadata_schedule_406_0_e3825: f64 = (w[61] * noise_metadata_schedule_406_0_e3824);
            let noise_metadata_schedule_406_0_e3827: f64 = if noise_metadata_schedule_406_0_e3825 < params[134] { 1.0 } else { 0.0 };
            w[506] = noise_metadata_schedule_406_0_e3827;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_407_0_e3842,) = {
    if ((w[505] != 0.0) && (w[506] != 0.0)) {
        let noise_metadata_schedule_407_0_e3836: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_407_0_e3837: f64 = (w[62] / noise_metadata_schedule_407_0_e3836);
        let noise_metadata_schedule_407_0_e3838: f64 = (1.0 - noise_metadata_schedule_407_0_e3837);
        let noise_metadata_schedule_407_0_e3839: f64 = (w[61] * noise_metadata_schedule_407_0_e3838);
        let noise_metadata_schedule_407_0_e3840: f64 = (noise_metadata_schedule_407_0_e3839).exp();
        (noise_metadata_schedule_407_0_e3840,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_407_0_e3842;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_408_0_e3850,) = {
    if ((w[505] != 0.0) && (w[506] == 0.0)) {
        let noise_metadata_schedule_408_0_e3848: f64 = (params[134]).exp();
        (noise_metadata_schedule_408_0_e3848,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_408_0_e3850;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_409_0_e3871,) = {
    if ((w[505] != 0.0) && (w[506] == 0.0)) {
        let noise_metadata_schedule_409_0_e3862: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_409_0_e3863: f64 = (w[62] / noise_metadata_schedule_409_0_e3862);
        let noise_metadata_schedule_409_0_e3864: f64 = (1.0 - noise_metadata_schedule_409_0_e3863);
        let noise_metadata_schedule_409_0_e3865: f64 = (w[61] * noise_metadata_schedule_409_0_e3864);
        let noise_metadata_schedule_409_0_e3867: f64 = (noise_metadata_schedule_409_0_e3865 - params[134]);
        let noise_metadata_schedule_409_0_e3868: f64 = (1.0 + noise_metadata_schedule_409_0_e3867);
        let noise_metadata_schedule_409_0_e3869: f64 = (w[275] * noise_metadata_schedule_409_0_e3868);
        (noise_metadata_schedule_409_0_e3869,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_409_0_e3871;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_410_0_e3877,) = {
    if (w[505] != 0.0) {
        let noise_metadata_schedule_410_0_e3875: f64 = (w[232] * w[65]);
        (noise_metadata_schedule_410_0_e3875,)
    } else {
        (w[255],)
    }
};
            w[255] = noise_metadata_schedule_410_0_e3877;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_411_0_e3921,) = {
    if (w[505] != 0.0) {
        let noise_metadata_schedule_411_0_e3881: f64 = (w[255] * w[255]);
        let noise_metadata_schedule_411_0_e3883: f64 = (noise_metadata_schedule_411_0_e3881 + 1e-30);
        let noise_metadata_schedule_411_0_e3884: f64 = (noise_metadata_schedule_411_0_e3883).sqrt();
        let noise_metadata_schedule_411_0_e3886: f64 = (-2.0);
        let noise_metadata_schedule_411_0_e3888: f64 = (noise_metadata_schedule_411_0_e3886 - params[66]);
        let noise_metadata_schedule_411_0_e3889: f64 = (noise_metadata_schedule_411_0_e3884).powf(noise_metadata_schedule_411_0_e3888);
        let noise_metadata_schedule_411_0_e3894: f64 = (params[66] * params[66]);
        let noise_metadata_schedule_411_0_e3895: f64 = (1.0 - noise_metadata_schedule_411_0_e3894);
        let noise_metadata_schedule_411_0_e3898: f64 = (3.0 * w[255]);
        let noise_metadata_schedule_411_0_e3901: f64 = (params[66] - 1.0);
        let noise_metadata_schedule_411_0_e3902: f64 = (noise_metadata_schedule_411_0_e3898 * noise_metadata_schedule_411_0_e3901);
        let noise_metadata_schedule_411_0_e3903: f64 = (noise_metadata_schedule_411_0_e3895 - noise_metadata_schedule_411_0_e3902);
        let noise_metadata_schedule_411_0_e3904: f64 = (params[66] * noise_metadata_schedule_411_0_e3903);
        let noise_metadata_schedule_411_0_e3907: f64 = (6.0 * w[255]);
        let noise_metadata_schedule_411_0_e3909: f64 = (noise_metadata_schedule_411_0_e3907 * w[255]);
        let noise_metadata_schedule_411_0_e3912: f64 = (params[66] - 1.0);
        let noise_metadata_schedule_411_0_e3914: f64 = (noise_metadata_schedule_411_0_e3912 + w[255]);
        let noise_metadata_schedule_411_0_e3915: f64 = (noise_metadata_schedule_411_0_e3909 * noise_metadata_schedule_411_0_e3914);
        let noise_metadata_schedule_411_0_e3916: f64 = (noise_metadata_schedule_411_0_e3904 - noise_metadata_schedule_411_0_e3915);
        let noise_metadata_schedule_411_0_e3917: f64 = (noise_metadata_schedule_411_0_e3889 * noise_metadata_schedule_411_0_e3916);
        let noise_metadata_schedule_411_0_e3919: f64 = (noise_metadata_schedule_411_0_e3917 * 0.16666666666666666);
        (noise_metadata_schedule_411_0_e3919,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_411_0_e3921;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_412_0_e3933,) = {
    if (w[505] != 0.0) {
        let noise_metadata_schedule_412_0_e3925: f64 = (w[232] * w[62]);
        let noise_metadata_schedule_412_0_e3927: f64 = (noise_metadata_schedule_412_0_e3925 * w[61]);
        let noise_metadata_schedule_412_0_e3930: f64 = (w[70] * w[60]);
        let noise_metadata_schedule_412_0_e3931: f64 = (noise_metadata_schedule_412_0_e3927 / noise_metadata_schedule_412_0_e3930);
        (noise_metadata_schedule_412_0_e3931,)
    } else {
        (w[255],)
    }
};
            w[255] = noise_metadata_schedule_412_0_e3933;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_413_0_e3936: f64 = (-0.001);
            let noise_metadata_schedule_413_0_e3937: f64 = if w[255] < noise_metadata_schedule_413_0_e3936 { 1.0 } else { 0.0 };
            w[507] = noise_metadata_schedule_413_0_e3937;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_414_0_e3940: f64 = if w[255] < params[134] { 1.0 } else { 0.0 };
            w[508] = noise_metadata_schedule_414_0_e3940;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_415_0_e3949,) = {
    if (((w[505] != 0.0) && (w[507] != 0.0)) && (w[508] != 0.0)) {
        let noise_metadata_schedule_415_0_e3947: f64 = (w[255]).exp();
        (noise_metadata_schedule_415_0_e3947,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_415_0_e3949;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_416_0_e3959,) = {
    if (((w[505] != 0.0) && (w[507] != 0.0)) && (w[508] == 0.0)) {
        let noise_metadata_schedule_416_0_e3957: f64 = (params[134]).exp();
        (noise_metadata_schedule_416_0_e3957,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_416_0_e3959;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_417_0_e3974,) = {
    if (((w[505] != 0.0) && (w[507] != 0.0)) && (w[508] == 0.0)) {
        let noise_metadata_schedule_417_0_e3970: f64 = (w[255] - params[134]);
        let noise_metadata_schedule_417_0_e3971: f64 = (1.0 + noise_metadata_schedule_417_0_e3970);
        let noise_metadata_schedule_417_0_e3972: f64 = (w[275] * noise_metadata_schedule_417_0_e3971);
        (noise_metadata_schedule_417_0_e3972,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_417_0_e3974;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_418_0_e3989,) = {
    if ((w[505] != 0.0) && (w[507] != 0.0)) {
        let noise_metadata_schedule_418_0_e3979: f64 = (-w[232]);
        let noise_metadata_schedule_418_0_e3983: f64 = (1.0 - w[91]);
        let noise_metadata_schedule_418_0_e3985: f64 = (noise_metadata_schedule_418_0_e3983 / w[255]);
        let noise_metadata_schedule_418_0_e3986: f64 = (1.0 + noise_metadata_schedule_418_0_e3985);
        let noise_metadata_schedule_418_0_e3987: f64 = (noise_metadata_schedule_418_0_e3979 * noise_metadata_schedule_418_0_e3986);
        (noise_metadata_schedule_418_0_e3987,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_418_0_e3989;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_419_0_e4012,) = {
    if ((w[505] != 0.0) && (w[507] == 0.0)) {
        let noise_metadata_schedule_419_0_e3996: f64 = (w[232] * 0.5);
        let noise_metadata_schedule_419_0_e3998: f64 = (noise_metadata_schedule_419_0_e3996 * w[255]);
        let noise_metadata_schedule_419_0_e4002: f64 = (w[255] * 0.3333333333333333);
        let noise_metadata_schedule_419_0_e4006: f64 = (0.25 * w[255]);
        let noise_metadata_schedule_419_0_e4007: f64 = (1.0 + noise_metadata_schedule_419_0_e4006);
        let noise_metadata_schedule_419_0_e4008: f64 = (noise_metadata_schedule_419_0_e4002 * noise_metadata_schedule_419_0_e4007);
        let noise_metadata_schedule_419_0_e4009: f64 = (1.0 + noise_metadata_schedule_419_0_e4008);
        let noise_metadata_schedule_419_0_e4010: f64 = (noise_metadata_schedule_419_0_e3998 * noise_metadata_schedule_419_0_e4009);
        (noise_metadata_schedule_419_0_e4010,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_419_0_e4012;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_420_0_e4028,) = {
    if (w[505] != 0.0) {
        let noise_metadata_schedule_420_0_e4016: f64 = (2.0 * w[58]);
        let noise_metadata_schedule_420_0_e4018: f64 = (noise_metadata_schedule_420_0_e4016 * w[69]);
        let noise_metadata_schedule_420_0_e4020: f64 = (noise_metadata_schedule_420_0_e4018 * w[59]);
        let noise_metadata_schedule_420_0_e4022: f64 = (noise_metadata_schedule_420_0_e4020 * w[68]);
        let noise_metadata_schedule_420_0_e4024: f64 = (noise_metadata_schedule_420_0_e4022 * w[65]);
        let noise_metadata_schedule_420_0_e4026: f64 = (noise_metadata_schedule_420_0_e4024 * w[63]);
        (noise_metadata_schedule_420_0_e4026,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_420_0_e4028;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_422_0_e4038,) = {
    if (w[505] == 0.0) {
        (0.0,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_422_0_e4038;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_423_0_e4049: f64 = if (((params[35] > 0.0) && (params[36] > 0.0)) && (w[230] < 0.0)) { 1.0 } else { 0.0 };
            w[509] = noise_metadata_schedule_423_0_e4049;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_424_0_e4061,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_424_0_e4054: f64 = (w[230] * w[67]);
        let noise_metadata_schedule_424_0_e4055: f64 = (1.0 - noise_metadata_schedule_424_0_e4054);
        let noise_metadata_schedule_424_0_e4058: f64 = (1.0 - w[76]);
        let noise_metadata_schedule_424_0_e4059: f64 = (noise_metadata_schedule_424_0_e4055).powf(noise_metadata_schedule_424_0_e4058);
        (noise_metadata_schedule_424_0_e4059,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_424_0_e4061;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_425_0_e4067: f64 = (2.0 * w[77]);
            let noise_metadata_schedule_425_0_e4068: f64 = (w[79] / noise_metadata_schedule_425_0_e4067);
            let noise_metadata_schedule_425_0_e4069: f64 = (1.0 - noise_metadata_schedule_425_0_e4068);
            let noise_metadata_schedule_425_0_e4070: f64 = (w[83] * noise_metadata_schedule_425_0_e4069);
            let noise_metadata_schedule_425_0_e4072: f64 = if noise_metadata_schedule_425_0_e4070 < params[134] { 1.0 } else { 0.0 };
            w[510] = noise_metadata_schedule_425_0_e4072;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_426_0_e4087,) = {
    if ((w[509] != 0.0) && (w[510] != 0.0)) {
        let noise_metadata_schedule_426_0_e4081: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_426_0_e4082: f64 = (w[79] / noise_metadata_schedule_426_0_e4081);
        let noise_metadata_schedule_426_0_e4083: f64 = (1.0 - noise_metadata_schedule_426_0_e4082);
        let noise_metadata_schedule_426_0_e4084: f64 = (w[83] * noise_metadata_schedule_426_0_e4083);
        let noise_metadata_schedule_426_0_e4085: f64 = (noise_metadata_schedule_426_0_e4084).exp();
        (noise_metadata_schedule_426_0_e4085,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_426_0_e4087;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_427_0_e4095,) = {
    if ((w[509] != 0.0) && (w[510] == 0.0)) {
        let noise_metadata_schedule_427_0_e4093: f64 = (params[134]).exp();
        (noise_metadata_schedule_427_0_e4093,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_427_0_e4095;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_428_0_e4116,) = {
    if ((w[509] != 0.0) && (w[510] == 0.0)) {
        let noise_metadata_schedule_428_0_e4107: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_428_0_e4108: f64 = (w[79] / noise_metadata_schedule_428_0_e4107);
        let noise_metadata_schedule_428_0_e4109: f64 = (1.0 - noise_metadata_schedule_428_0_e4108);
        let noise_metadata_schedule_428_0_e4110: f64 = (w[83] * noise_metadata_schedule_428_0_e4109);
        let noise_metadata_schedule_428_0_e4112: f64 = (noise_metadata_schedule_428_0_e4110 - params[134]);
        let noise_metadata_schedule_428_0_e4113: f64 = (1.0 + noise_metadata_schedule_428_0_e4112);
        let noise_metadata_schedule_428_0_e4114: f64 = (w[275] * noise_metadata_schedule_428_0_e4113);
        (noise_metadata_schedule_428_0_e4114,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_428_0_e4116;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_429_0_e4122,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_429_0_e4120: f64 = (w[230] * w[67]);
        (noise_metadata_schedule_429_0_e4120,)
    } else {
        (w[257],)
    }
};
            w[257] = noise_metadata_schedule_429_0_e4122;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_430_0_e4166,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_430_0_e4126: f64 = (w[257] * w[257]);
        let noise_metadata_schedule_430_0_e4128: f64 = (noise_metadata_schedule_430_0_e4126 + 1e-30);
        let noise_metadata_schedule_430_0_e4129: f64 = (noise_metadata_schedule_430_0_e4128).sqrt();
        let noise_metadata_schedule_430_0_e4131: f64 = (-2.0);
        let noise_metadata_schedule_430_0_e4133: f64 = (noise_metadata_schedule_430_0_e4131 - w[76]);
        let noise_metadata_schedule_430_0_e4134: f64 = (noise_metadata_schedule_430_0_e4129).powf(noise_metadata_schedule_430_0_e4133);
        let noise_metadata_schedule_430_0_e4139: f64 = (w[76] * w[76]);
        let noise_metadata_schedule_430_0_e4140: f64 = (1.0 - noise_metadata_schedule_430_0_e4139);
        let noise_metadata_schedule_430_0_e4143: f64 = (3.0 * w[257]);
        let noise_metadata_schedule_430_0_e4146: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_430_0_e4147: f64 = (noise_metadata_schedule_430_0_e4143 * noise_metadata_schedule_430_0_e4146);
        let noise_metadata_schedule_430_0_e4148: f64 = (noise_metadata_schedule_430_0_e4140 - noise_metadata_schedule_430_0_e4147);
        let noise_metadata_schedule_430_0_e4149: f64 = (w[76] * noise_metadata_schedule_430_0_e4148);
        let noise_metadata_schedule_430_0_e4152: f64 = (6.0 * w[257]);
        let noise_metadata_schedule_430_0_e4154: f64 = (noise_metadata_schedule_430_0_e4152 * w[257]);
        let noise_metadata_schedule_430_0_e4157: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_430_0_e4159: f64 = (noise_metadata_schedule_430_0_e4157 + w[257]);
        let noise_metadata_schedule_430_0_e4160: f64 = (noise_metadata_schedule_430_0_e4154 * noise_metadata_schedule_430_0_e4159);
        let noise_metadata_schedule_430_0_e4161: f64 = (noise_metadata_schedule_430_0_e4149 - noise_metadata_schedule_430_0_e4160);
        let noise_metadata_schedule_430_0_e4162: f64 = (noise_metadata_schedule_430_0_e4134 * noise_metadata_schedule_430_0_e4161);
        let noise_metadata_schedule_430_0_e4164: f64 = (noise_metadata_schedule_430_0_e4162 * 0.16666666666666666);
        (noise_metadata_schedule_430_0_e4164,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_430_0_e4166;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_431_0_e4178,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_431_0_e4170: f64 = (w[230] * w[79]);
        let noise_metadata_schedule_431_0_e4172: f64 = (noise_metadata_schedule_431_0_e4170 * w[83]);
        let noise_metadata_schedule_431_0_e4175: f64 = (w[85] * w[80]);
        let noise_metadata_schedule_431_0_e4176: f64 = (noise_metadata_schedule_431_0_e4172 / noise_metadata_schedule_431_0_e4175);
        (noise_metadata_schedule_431_0_e4176,)
    } else {
        (w[257],)
    }
};
            w[257] = noise_metadata_schedule_431_0_e4178;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_432_0_e4181: f64 = (-0.001);
            let noise_metadata_schedule_432_0_e4182: f64 = if w[257] < noise_metadata_schedule_432_0_e4181 { 1.0 } else { 0.0 };
            w[511] = noise_metadata_schedule_432_0_e4182;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_433_0_e4185: f64 = if w[257] < params[134] { 1.0 } else { 0.0 };
            w[512] = noise_metadata_schedule_433_0_e4185;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_434_0_e4194,) = {
    if (((w[509] != 0.0) && (w[511] != 0.0)) && (w[512] != 0.0)) {
        let noise_metadata_schedule_434_0_e4192: f64 = (w[257]).exp();
        (noise_metadata_schedule_434_0_e4192,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_434_0_e4194;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_435_0_e4204,) = {
    if (((w[509] != 0.0) && (w[511] != 0.0)) && (w[512] == 0.0)) {
        let noise_metadata_schedule_435_0_e4202: f64 = (params[134]).exp();
        (noise_metadata_schedule_435_0_e4202,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_435_0_e4204;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_436_0_e4219,) = {
    if (((w[509] != 0.0) && (w[511] != 0.0)) && (w[512] == 0.0)) {
        let noise_metadata_schedule_436_0_e4215: f64 = (w[257] - params[134]);
        let noise_metadata_schedule_436_0_e4216: f64 = (1.0 + noise_metadata_schedule_436_0_e4215);
        let noise_metadata_schedule_436_0_e4217: f64 = (w[275] * noise_metadata_schedule_436_0_e4216);
        (noise_metadata_schedule_436_0_e4217,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_436_0_e4219;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_437_0_e4234,) = {
    if ((w[509] != 0.0) && (w[511] != 0.0)) {
        let noise_metadata_schedule_437_0_e4224: f64 = (-w[230]);
        let noise_metadata_schedule_437_0_e4228: f64 = (1.0 - w[92]);
        let noise_metadata_schedule_437_0_e4230: f64 = (noise_metadata_schedule_437_0_e4228 / w[257]);
        let noise_metadata_schedule_437_0_e4231: f64 = (1.0 + noise_metadata_schedule_437_0_e4230);
        let noise_metadata_schedule_437_0_e4232: f64 = (noise_metadata_schedule_437_0_e4224 * noise_metadata_schedule_437_0_e4231);
        (noise_metadata_schedule_437_0_e4232,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_437_0_e4234;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_438_0_e4257,) = {
    if ((w[509] != 0.0) && (w[511] == 0.0)) {
        let noise_metadata_schedule_438_0_e4241: f64 = (w[230] * 0.5);
        let noise_metadata_schedule_438_0_e4243: f64 = (noise_metadata_schedule_438_0_e4241 * w[257]);
        let noise_metadata_schedule_438_0_e4247: f64 = (w[257] * 0.3333333333333333);
        let noise_metadata_schedule_438_0_e4251: f64 = (0.25 * w[257]);
        let noise_metadata_schedule_438_0_e4252: f64 = (1.0 + noise_metadata_schedule_438_0_e4251);
        let noise_metadata_schedule_438_0_e4253: f64 = (noise_metadata_schedule_438_0_e4247 * noise_metadata_schedule_438_0_e4252);
        let noise_metadata_schedule_438_0_e4254: f64 = (1.0 + noise_metadata_schedule_438_0_e4253);
        let noise_metadata_schedule_438_0_e4255: f64 = (noise_metadata_schedule_438_0_e4243 * noise_metadata_schedule_438_0_e4254);
        (noise_metadata_schedule_438_0_e4255,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_438_0_e4257;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_439_0_e4273,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_439_0_e4261: f64 = (2.0 * w[84]);
        let noise_metadata_schedule_439_0_e4263: f64 = (noise_metadata_schedule_439_0_e4261 * w[81]);
        let noise_metadata_schedule_439_0_e4265: f64 = (noise_metadata_schedule_439_0_e4263 * w[77]);
        let noise_metadata_schedule_439_0_e4267: f64 = (noise_metadata_schedule_439_0_e4265 * w[78]);
        let noise_metadata_schedule_439_0_e4269: f64 = (noise_metadata_schedule_439_0_e4267 * w[67]);
        let noise_metadata_schedule_439_0_e4271: f64 = (noise_metadata_schedule_439_0_e4269 * w[89]);
        (noise_metadata_schedule_439_0_e4271,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_439_0_e4273;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_441_0_e4283,) = {
    if (w[509] == 0.0) {
        (0.0,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_441_0_e4283;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_446_0_e4310: f64 = (2.0 * w[43]);
            let noise_metadata_schedule_446_0_e4313: f64 = (w[248] - 1.0);
            let noise_metadata_schedule_446_0_e4314: f64 = (noise_metadata_schedule_446_0_e4310 * noise_metadata_schedule_446_0_e4313);
            let noise_metadata_schedule_446_0_e4319: f64 = (4.0 * w[43]);
            let noise_metadata_schedule_446_0_e4321: f64 = (noise_metadata_schedule_446_0_e4319 / w[37]);
            let noise_metadata_schedule_446_0_e4323: f64 = (noise_metadata_schedule_446_0_e4321 * w[248]);
            let noise_metadata_schedule_446_0_e4324: f64 = (1.0 + noise_metadata_schedule_446_0_e4323);
            let noise_metadata_schedule_446_0_e4325: f64 = (noise_metadata_schedule_446_0_e4324).sqrt();
            let noise_metadata_schedule_446_0_e4326: f64 = (1.0 + noise_metadata_schedule_446_0_e4325);
            let noise_metadata_schedule_446_0_e4327: f64 = (noise_metadata_schedule_446_0_e4314 / noise_metadata_schedule_446_0_e4326);
            w[157] = noise_metadata_schedule_446_0_e4327;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_447_0_e4334: f64 = if ((params[5] > 0.0) && (params[32] > 0.0)) { 1.0 } else { 0.0 };
            w[513] = noise_metadata_schedule_447_0_e4334;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_448_0_e4340,) = {
    if (w[513] != 0.0) {
        let noise_metadata_schedule_448_0_e4338: f64 = (w[157] * w[150]);
        (noise_metadata_schedule_448_0_e4338,)
    } else {
        (w[157],)
    }
};
            w[157] = noise_metadata_schedule_448_0_e4340;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_449_0_e4365,) = {
    if (w[513] != 0.0) {
        let noise_metadata_schedule_449_0_e4344: f64 = (params[32] * 2.0);
        let noise_metadata_schedule_449_0_e4346: f64 = (noise_metadata_schedule_449_0_e4344 * w[43]);
        let noise_metadata_schedule_449_0_e4349: f64 = (w[249] - 1.0);
        let noise_metadata_schedule_449_0_e4350: f64 = (noise_metadata_schedule_449_0_e4346 * noise_metadata_schedule_449_0_e4349);
        let noise_metadata_schedule_449_0_e4355: f64 = (4.0 * w[43]);
        let noise_metadata_schedule_449_0_e4357: f64 = (noise_metadata_schedule_449_0_e4355 / w[37]);
        let noise_metadata_schedule_449_0_e4359: f64 = (noise_metadata_schedule_449_0_e4357 * w[249]);
        let noise_metadata_schedule_449_0_e4360: f64 = (1.0 + noise_metadata_schedule_449_0_e4359);
        let noise_metadata_schedule_449_0_e4361: f64 = (noise_metadata_schedule_449_0_e4360).sqrt();
        let noise_metadata_schedule_449_0_e4362: f64 = (1.0 + noise_metadata_schedule_449_0_e4361);
        let noise_metadata_schedule_449_0_e4363: f64 = (noise_metadata_schedule_449_0_e4350 / noise_metadata_schedule_449_0_e4362);
        (noise_metadata_schedule_449_0_e4363,)
    } else {
        (w[164],)
    }
};
            w[164] = noise_metadata_schedule_449_0_e4365;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_450_0_e4369,) = {
    if (w[513] != 0.0) {
        (0.0,)
    } else {
        (w[165],)
    }
};
            w[165] = noise_metadata_schedule_450_0_e4369;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_451_0_e4372: f64 = if params[5] == 1.0 { 1.0 } else { 0.0 };
            w[514] = noise_metadata_schedule_451_0_e4372;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_452_0_e4382,) = {
    if ((w[513] != 0.0) && (w[514] != 0.0)) {
        let noise_metadata_schedule_452_0_e4378: f64 = (params[32] * w[43]);
        let noise_metadata_schedule_452_0_e4380: f64 = (noise_metadata_schedule_452_0_e4378 * w[32]);
        (noise_metadata_schedule_452_0_e4380,)
    } else {
        (w[271],)
    }
};
            w[271] = noise_metadata_schedule_452_0_e4382;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_453_0_e4395,) = {
    if ((w[513] != 0.0) && (w[514] != 0.0)) {
        let noise_metadata_schedule_453_0_e4390: f64 = (w[271] * w[8]);
        let noise_metadata_schedule_453_0_e4391: f64 = (noise_metadata_schedule_453_0_e4390).ln();
        let noise_metadata_schedule_453_0_e4392: f64 = (2.0 - noise_metadata_schedule_453_0_e4391);
        let noise_metadata_schedule_453_0_e4393: f64 = (w[6] * noise_metadata_schedule_453_0_e4392);
        (noise_metadata_schedule_453_0_e4393,)
    } else {
        (w[166],)
    }
};
            w[166] = noise_metadata_schedule_453_0_e4395;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_454_0_e4403,) = {
    if ((w[513] != 0.0) && (w[514] != 0.0)) {
        let noise_metadata_schedule_454_0_e4401: f64 = (w[241] - w[166]);
        (noise_metadata_schedule_454_0_e4401,)
    } else {
        (w[264],)
    }
};
            w[264] = noise_metadata_schedule_454_0_e4403;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_455_0_e4411,) = {
    if ((w[513] != 0.0) && (w[514] != 0.0)) {
        let noise_metadata_schedule_455_0_e4409: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_455_0_e4409,)
    } else {
        (w[261],)
    }
};
            w[261] = noise_metadata_schedule_455_0_e4411;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_456_0_e4419,) = {
    if ((w[513] != 0.0) && (w[514] != 0.0)) {
        let noise_metadata_schedule_456_0_e4417: f64 = (w[264] * w[264]);
        (noise_metadata_schedule_456_0_e4417,)
    } else {
        (w[262],)
    }
};
            w[262] = noise_metadata_schedule_456_0_e4419;
        }
        if (active[0] & 0x6000) != 0 {
            let noise_metadata_schedule_457_0_e4422: f64 = if w[264] < 0.0 { 1.0 } else { 0.0 };
            w[515] = noise_metadata_schedule_457_0_e4422;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_8(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_458_0_e4439,) = {
    if (((w[513] != 0.0) && (w[514] != 0.0)) && (w[515] != 0.0)) {
        let noise_metadata_schedule_458_0_e4430: f64 = (0.5 * w[261]);
        let noise_metadata_schedule_458_0_e4433: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_458_0_e4434: f64 = (noise_metadata_schedule_458_0_e4433).sqrt();
        let noise_metadata_schedule_458_0_e4436: f64 = (noise_metadata_schedule_458_0_e4434 - w[264]);
        let noise_metadata_schedule_458_0_e4437: f64 = (noise_metadata_schedule_458_0_e4430 / noise_metadata_schedule_458_0_e4436);
        (noise_metadata_schedule_458_0_e4437,)
    } else {
        (w[167],)
    }
};
            w[167] = noise_metadata_schedule_458_0_e4439;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_459_0_e4455,) = {
    if (((w[513] != 0.0) && (w[514] != 0.0)) && (w[515] == 0.0)) {
        let noise_metadata_schedule_459_0_e4449: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_459_0_e4450: f64 = (noise_metadata_schedule_459_0_e4449).sqrt();
        let noise_metadata_schedule_459_0_e4452: f64 = (noise_metadata_schedule_459_0_e4450 + w[264]);
        let noise_metadata_schedule_459_0_e4453: f64 = (0.5 * noise_metadata_schedule_459_0_e4452);
        (noise_metadata_schedule_459_0_e4453,)
    } else {
        (w[167],)
    }
};
            w[167] = noise_metadata_schedule_459_0_e4455;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_460_0_e4471,) = {
    if ((w[513] != 0.0) && (w[514] != 0.0)) {
        let noise_metadata_schedule_460_0_e4463: f64 = (w[164] + w[165]);
        let noise_metadata_schedule_460_0_e4465: f64 = (noise_metadata_schedule_460_0_e4463 * w[32]);
        let noise_metadata_schedule_460_0_e4466: f64 = (w[271] + noise_metadata_schedule_460_0_e4465);
        let noise_metadata_schedule_460_0_e4468: f64 = (noise_metadata_schedule_460_0_e4466 + w[167]);
        let noise_metadata_schedule_460_0_e4469: f64 = (w[167] / noise_metadata_schedule_460_0_e4468);
        (noise_metadata_schedule_460_0_e4469,)
    } else {
        (w[168],)
    }
};
            w[168] = noise_metadata_schedule_460_0_e4471;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_464_0_e4499,) = {
    if ((w[513] != 0.0) && (w[514] == 0.0)) {
        (1.0,)
    } else {
        (w[168],)
    }
};
            w[168] = noise_metadata_schedule_464_0_e4499;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_465_0_e4505,) = {
    if (w[513] != 0.0) {
        let noise_metadata_schedule_465_0_e4503: f64 = (w[168] * w[164]);
        (noise_metadata_schedule_465_0_e4503,)
    } else {
        (w[169],)
    }
};
            w[169] = noise_metadata_schedule_465_0_e4505;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_466_0_e4508: f64 = if params[83] == 1.0 { 1.0 } else { 0.0 };
            w[516] = noise_metadata_schedule_466_0_e4508;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_467_0_e4514,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_467_0_e4512: f64 = (w[234] + w[230]);
        (noise_metadata_schedule_467_0_e4512,)
    } else {
        (w[322],)
    }
};
            w[322] = noise_metadata_schedule_467_0_e4514;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_468_0_e4520,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_468_0_e4518: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_468_0_e4518,)
    } else {
        (w[261],)
    }
};
            w[261] = noise_metadata_schedule_468_0_e4520;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_469_0_e4532,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_469_0_e4523: f64 = (-1.0);
        let noise_metadata_schedule_469_0_e4525: f64 = (noise_metadata_schedule_469_0_e4523 * w[322]);
        let noise_metadata_schedule_469_0_e4527: f64 = (-1.0);
        let noise_metadata_schedule_469_0_e4528: f64 = (noise_metadata_schedule_469_0_e4525 * noise_metadata_schedule_469_0_e4527);
        let noise_metadata_schedule_469_0_e4530: f64 = (noise_metadata_schedule_469_0_e4528 * w[322]);
        (noise_metadata_schedule_469_0_e4530,)
    } else {
        (w[262],)
    }
};
            w[262] = noise_metadata_schedule_469_0_e4532;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_470_0_e4534: f64 = (-1.0);
            let noise_metadata_schedule_470_0_e4536: f64 = (noise_metadata_schedule_470_0_e4534 * w[322]);
            let noise_metadata_schedule_470_0_e4538: f64 = if noise_metadata_schedule_470_0_e4536 < 0.0 { 1.0 } else { 0.0 };
            w[517] = noise_metadata_schedule_470_0_e4538;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_471_0_e4556,) = {
    if ((w[516] != 0.0) && (w[517] != 0.0)) {
        let noise_metadata_schedule_471_0_e4544: f64 = (0.5 * w[261]);
        let noise_metadata_schedule_471_0_e4547: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_471_0_e4548: f64 = (noise_metadata_schedule_471_0_e4547).sqrt();
        let noise_metadata_schedule_471_0_e4550: f64 = (-1.0);
        let noise_metadata_schedule_471_0_e4552: f64 = (noise_metadata_schedule_471_0_e4550 * w[322]);
        let noise_metadata_schedule_471_0_e4553: f64 = (noise_metadata_schedule_471_0_e4548 - noise_metadata_schedule_471_0_e4552);
        let noise_metadata_schedule_471_0_e4554: f64 = (noise_metadata_schedule_471_0_e4544 / noise_metadata_schedule_471_0_e4553);
        (noise_metadata_schedule_471_0_e4554,)
    } else {
        (w[323],)
    }
};
            w[323] = noise_metadata_schedule_471_0_e4556;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_472_0_e4573,) = {
    if ((w[516] != 0.0) && (w[517] == 0.0)) {
        let noise_metadata_schedule_472_0_e4564: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_472_0_e4565: f64 = (noise_metadata_schedule_472_0_e4564).sqrt();
        let noise_metadata_schedule_472_0_e4567: f64 = (-1.0);
        let noise_metadata_schedule_472_0_e4569: f64 = (noise_metadata_schedule_472_0_e4567 * w[322]);
        let noise_metadata_schedule_472_0_e4570: f64 = (noise_metadata_schedule_472_0_e4565 + noise_metadata_schedule_472_0_e4569);
        let noise_metadata_schedule_472_0_e4571: f64 = (0.5 * noise_metadata_schedule_472_0_e4570);
        (noise_metadata_schedule_472_0_e4571,)
    } else {
        (w[323],)
    }
};
            w[323] = noise_metadata_schedule_472_0_e4573;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_473_0_e4583,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_473_0_e4579: f64 = (w[318]).powf(params[81]);
        let noise_metadata_schedule_473_0_e4580: f64 = (1.0 - noise_metadata_schedule_473_0_e4579);
        let noise_metadata_schedule_473_0_e4581: f64 = (1.0 / noise_metadata_schedule_473_0_e4580);
        (noise_metadata_schedule_473_0_e4581,)
    } else {
        (w[324],)
    }
};
            w[324] = noise_metadata_schedule_473_0_e4583;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_474_0_e4589,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_474_0_e4587: f64 = (w[318] * params[80]);
        (noise_metadata_schedule_474_0_e4587,)
    } else {
        (w[319],)
    }
};
            w[319] = noise_metadata_schedule_474_0_e4589;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_475_0_e4605,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_475_0_e4593: f64 = (w[324] * w[324]);
        let noise_metadata_schedule_475_0_e4597: f64 = (params[81] - 1.0);
        let noise_metadata_schedule_475_0_e4598: f64 = (w[318]).powf(noise_metadata_schedule_475_0_e4597);
        let noise_metadata_schedule_475_0_e4599: f64 = (noise_metadata_schedule_475_0_e4593 * noise_metadata_schedule_475_0_e4598);
        let noise_metadata_schedule_475_0_e4601: f64 = (noise_metadata_schedule_475_0_e4599 * params[81]);
        let noise_metadata_schedule_475_0_e4603: f64 = (noise_metadata_schedule_475_0_e4601 / params[80]);
        (noise_metadata_schedule_475_0_e4603,)
    } else {
        (w[321],)
    }
};
            w[321] = noise_metadata_schedule_475_0_e4605;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_476_0_e4608: f64 = if w[323] < w[319] { 1.0 } else { 0.0 };
            w[518] = noise_metadata_schedule_476_0_e4608;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_477_0_e4622,) = {
    if ((w[516] != 0.0) && (w[518] != 0.0)) {
        let noise_metadata_schedule_477_0_e4616: f64 = (w[323] / params[80]);
        let noise_metadata_schedule_477_0_e4618: f64 = (noise_metadata_schedule_477_0_e4616).powf(params[81]);
        let noise_metadata_schedule_477_0_e4619: f64 = (1.0 - noise_metadata_schedule_477_0_e4618);
        let noise_metadata_schedule_477_0_e4620: f64 = (1.0 / noise_metadata_schedule_477_0_e4619);
        (noise_metadata_schedule_477_0_e4620,)
    } else {
        (w[320],)
    }
};
            w[320] = noise_metadata_schedule_477_0_e4622;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_478_0_e4635,) = {
    if ((w[516] != 0.0) && (w[518] == 0.0)) {
        let noise_metadata_schedule_478_0_e4630: f64 = (w[323] - w[319]);
        let noise_metadata_schedule_478_0_e4632: f64 = (noise_metadata_schedule_478_0_e4630 * w[321]);
        let noise_metadata_schedule_478_0_e4633: f64 = (w[324] + noise_metadata_schedule_478_0_e4632);
        (noise_metadata_schedule_478_0_e4633,)
    } else {
        (w[320],)
    }
};
            w[320] = noise_metadata_schedule_478_0_e4635;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_479_0_e4640,) = {
    if (w[516] == 0.0) {
        (1.0,)
    } else {
        (w[320],)
    }
};
            w[320] = noise_metadata_schedule_479_0_e4640;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_480_0_e4643: f64 = (w[82] * w[320]);
            w[82] = noise_metadata_schedule_480_0_e4643;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_481_0_e4646: f64 = (w[157] * w[320]);
            w[157] = noise_metadata_schedule_481_0_e4646;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_482_0_e4649: f64 = (w[154] * w[320]);
            w[154] = noise_metadata_schedule_482_0_e4649;
        }
        if (active[0] & 0x6000) != 0 {
            let noise_metadata_schedule_483_0_e4652: f64 = (w[169] * w[320]);
            w[169] = noise_metadata_schedule_483_0_e4652;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_484_0_e4656: f64 = (w[131] / w[41]);
            let noise_metadata_schedule_484_0_e4657: f64 = (1.0 + noise_metadata_schedule_484_0_e4656);
            let noise_metadata_schedule_484_0_e4660: f64 = (w[138] / w[40]);
            let noise_metadata_schedule_484_0_e4661: f64 = (noise_metadata_schedule_484_0_e4657 + noise_metadata_schedule_484_0_e4660);
            w[172] = noise_metadata_schedule_484_0_e4661;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_485_0_e4664: f64 = (0.1 * 0.1);
            w[261] = noise_metadata_schedule_485_0_e4664;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_486_0_e4667: f64 = (w[172] * w[172]);
            w[262] = noise_metadata_schedule_486_0_e4667;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_487_0_e4670: f64 = if w[172] < 0.0 { 1.0 } else { 0.0 };
            w[519] = noise_metadata_schedule_487_0_e4670;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_488_0_e4683,) = {
    if (w[519] != 0.0) {
        let noise_metadata_schedule_488_0_e4674: f64 = (0.5 * w[261]);
        let noise_metadata_schedule_488_0_e4677: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_488_0_e4678: f64 = (noise_metadata_schedule_488_0_e4677).sqrt();
        let noise_metadata_schedule_488_0_e4680: f64 = (noise_metadata_schedule_488_0_e4678 - w[172]);
        let noise_metadata_schedule_488_0_e4681: f64 = (noise_metadata_schedule_488_0_e4674 / noise_metadata_schedule_488_0_e4680);
        (noise_metadata_schedule_488_0_e4681,)
    } else {
        (w[173],)
    }
};
            w[173] = noise_metadata_schedule_488_0_e4683;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_489_0_e4695,) = {
    if (w[519] == 0.0) {
        let noise_metadata_schedule_489_0_e4689: f64 = (w[262] + w[261]);
        let noise_metadata_schedule_489_0_e4690: f64 = (noise_metadata_schedule_489_0_e4689).sqrt();
        let noise_metadata_schedule_489_0_e4692: f64 = (noise_metadata_schedule_489_0_e4690 + w[172]);
        let noise_metadata_schedule_489_0_e4693: f64 = (0.5 * noise_metadata_schedule_489_0_e4692);
        (noise_metadata_schedule_489_0_e4693,)
    } else {
        (w[173],)
    }
};
            w[173] = noise_metadata_schedule_489_0_e4695;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_490_0_e4701: f64 = (w[142] + w[143]);
            let noise_metadata_schedule_490_0_e4702: f64 = (0.5 * noise_metadata_schedule_490_0_e4701);
            let noise_metadata_schedule_490_0_e4703: f64 = (1.0 + noise_metadata_schedule_490_0_e4702);
            let noise_metadata_schedule_490_0_e4704: f64 = (w[173] * noise_metadata_schedule_490_0_e4703);
            w[174] = noise_metadata_schedule_490_0_e4704;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_491_0_e4707: f64 = (w[29] / w[174]);
            w[176] = noise_metadata_schedule_491_0_e4707;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_492_0_e4710: f64 = if w[176] < w[316] { 1.0 } else { 0.0 };
            w[520] = noise_metadata_schedule_492_0_e4710;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_493_0_e4714,) = {
    if (w[520] != 0.0) {
        (w[316],)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_493_0_e4714;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_494_0_e4717: f64 = (3.0 * w[176]);
            w[175] = noise_metadata_schedule_494_0_e4717;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_496_0_e4731: f64 = if w[149] > 0.0 { 1.0 } else { 0.0 };
            w[521] = noise_metadata_schedule_496_0_e4731;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_497_0_e4734: f64 = if params[38] == 1.0 { 1.0 } else { 0.0 };
            w[522] = noise_metadata_schedule_497_0_e4734;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_498_0_e4737: f64 = if w[230] < params[43] { 1.0 } else { 0.0 };
            w[523] = noise_metadata_schedule_498_0_e4737;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_499_0_e4739: f64 = (-w[149]);
            let noise_metadata_schedule_499_0_e4741: f64 = (noise_metadata_schedule_499_0_e4739 / params[41]);
            let noise_metadata_schedule_499_0_e4743: f64 = if noise_metadata_schedule_499_0_e4741 < params[134] { 1.0 } else { 0.0 };
            w[524] = noise_metadata_schedule_499_0_e4743;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_500_0_e4757,) = {
    if ((((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) && (w[524] != 0.0)) {
        let noise_metadata_schedule_500_0_e4752: f64 = (-w[149]);
        let noise_metadata_schedule_500_0_e4754: f64 = (noise_metadata_schedule_500_0_e4752 / params[41]);
        let noise_metadata_schedule_500_0_e4755: f64 = (noise_metadata_schedule_500_0_e4754).exp();
        (noise_metadata_schedule_500_0_e4755,)
    } else {
        (w[308],)
    }
};
            w[308] = noise_metadata_schedule_500_0_e4757;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_501_0_e4769,) = {
    if ((((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) && (w[524] == 0.0)) {
        let noise_metadata_schedule_501_0_e4767: f64 = (params[134]).exp();
        (noise_metadata_schedule_501_0_e4767,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_501_0_e4769;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_502_0_e4789,) = {
    if ((((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) && (w[524] == 0.0)) {
        let noise_metadata_schedule_502_0_e4781: f64 = (-w[149]);
        let noise_metadata_schedule_502_0_e4783: f64 = (noise_metadata_schedule_502_0_e4781 / params[41]);
        let noise_metadata_schedule_502_0_e4785: f64 = (noise_metadata_schedule_502_0_e4783 - params[134]);
        let noise_metadata_schedule_502_0_e4786: f64 = (1.0 + noise_metadata_schedule_502_0_e4785);
        let noise_metadata_schedule_502_0_e4787: f64 = (w[275] * noise_metadata_schedule_502_0_e4786);
        (noise_metadata_schedule_502_0_e4787,)
    } else {
        (w[308],)
    }
};
            w[308] = noise_metadata_schedule_502_0_e4789;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_503_0_e4801,) = {
    if (((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) {
        let noise_metadata_schedule_503_0_e4797: f64 = (params[43] - w[230]);
        let noise_metadata_schedule_503_0_e4799: f64 = (noise_metadata_schedule_503_0_e4797 * w[308]);
        (noise_metadata_schedule_503_0_e4799,)
    } else {
        (w[309],)
    }
};
            w[309] = noise_metadata_schedule_503_0_e4801;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_504_0_e4803: f64 = (-w[310]);
            let noise_metadata_schedule_504_0_e4806: f64 = (w[309]).powf(params[40]);
            let noise_metadata_schedule_504_0_e4807: f64 = (noise_metadata_schedule_504_0_e4803 * noise_metadata_schedule_504_0_e4806);
            let noise_metadata_schedule_504_0_e4809: f64 = if noise_metadata_schedule_504_0_e4807 < params[134] { 1.0 } else { 0.0 };
            w[525] = noise_metadata_schedule_504_0_e4809;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_505_0_e4825,) = {
    if ((((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) && (w[525] != 0.0)) {
        let noise_metadata_schedule_505_0_e4818: f64 = (-w[310]);
        let noise_metadata_schedule_505_0_e4821: f64 = (w[309]).powf(params[40]);
        let noise_metadata_schedule_505_0_e4822: f64 = (noise_metadata_schedule_505_0_e4818 * noise_metadata_schedule_505_0_e4821);
        let noise_metadata_schedule_505_0_e4823: f64 = (noise_metadata_schedule_505_0_e4822).exp();
        (noise_metadata_schedule_505_0_e4823,)
    } else {
        (w[313],)
    }
};
            w[313] = noise_metadata_schedule_505_0_e4825;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_506_0_e4837,) = {
    if ((((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) && (w[525] == 0.0)) {
        let noise_metadata_schedule_506_0_e4835: f64 = (params[134]).exp();
        (noise_metadata_schedule_506_0_e4835,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_506_0_e4837;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_507_0_e4859,) = {
    if ((((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) && (w[525] == 0.0)) {
        let noise_metadata_schedule_507_0_e4849: f64 = (-w[310]);
        let noise_metadata_schedule_507_0_e4852: f64 = (w[309]).powf(params[40]);
        let noise_metadata_schedule_507_0_e4853: f64 = (noise_metadata_schedule_507_0_e4849 * noise_metadata_schedule_507_0_e4852);
        let noise_metadata_schedule_507_0_e4855: f64 = (noise_metadata_schedule_507_0_e4853 - params[134]);
        let noise_metadata_schedule_507_0_e4856: f64 = (1.0 + noise_metadata_schedule_507_0_e4855);
        let noise_metadata_schedule_507_0_e4857: f64 = (w[275] * noise_metadata_schedule_507_0_e4856);
        (noise_metadata_schedule_507_0_e4857,)
    } else {
        (w[313],)
    }
};
            w[313] = noise_metadata_schedule_507_0_e4859;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_508_0_e4873,) = {
    if (((w[521] != 0.0) && (w[522] != 0.0)) && (w[523] != 0.0)) {
        let noise_metadata_schedule_508_0_e4867: f64 = (params[39] / w[310]);
        let noise_metadata_schedule_508_0_e4869: f64 = (noise_metadata_schedule_508_0_e4867 * w[309]);
        let noise_metadata_schedule_508_0_e4871: f64 = (noise_metadata_schedule_508_0_e4869 * w[313]);
        (noise_metadata_schedule_508_0_e4871,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_508_0_e4873;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_509_0_e4876: f64 = if params[38] == 2.0 { 1.0 } else { 0.0 };
            w[526] = noise_metadata_schedule_509_0_e4876;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_510_0_e4879: f64 = if w[230] < w[16] { 1.0 } else { 0.0 };
            w[527] = noise_metadata_schedule_510_0_e4879;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_511_0_e4896,) = {
    if ((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) {
        let noise_metadata_schedule_511_0_e4890: f64 = (2.0 * params[45]);
        let noise_metadata_schedule_511_0_e4893: f64 = (params[44] * params[44]);
        let noise_metadata_schedule_511_0_e4894: f64 = (noise_metadata_schedule_511_0_e4890 / noise_metadata_schedule_511_0_e4893);
        (noise_metadata_schedule_511_0_e4894,)
    } else {
        (w[185],)
    }
};
            w[185] = noise_metadata_schedule_511_0_e4896;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_512_0_e4911,) = {
    if ((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) {
        let noise_metadata_schedule_512_0_e4907: f64 = (w[16] - w[230]);
        let noise_metadata_schedule_512_0_e4909: f64 = (noise_metadata_schedule_512_0_e4907 / w[199]);
        (noise_metadata_schedule_512_0_e4909,)
    } else {
        (w[260],)
    }
};
            w[260] = noise_metadata_schedule_512_0_e4911;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_513_0_e4927,) = {
    if ((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) {
        let noise_metadata_schedule_513_0_e4922: f64 = (2.0 * w[260]);
        let noise_metadata_schedule_513_0_e4924: f64 = (noise_metadata_schedule_513_0_e4922 / w[185]);
        let noise_metadata_schedule_513_0_e4925: f64 = (noise_metadata_schedule_513_0_e4924).sqrt();
        (noise_metadata_schedule_513_0_e4925,)
    } else {
        (w[186],)
    }
};
            w[186] = noise_metadata_schedule_513_0_e4927;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_514_0_e4930: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[528] = noise_metadata_schedule_514_0_e4930;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_515_0_e4943,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[528] != 0.0)) {
        (params[44],)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_515_0_e4943;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_9(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_516_0_e4961,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[528] == 0.0)) {
        let noise_metadata_schedule_516_0_e4958: f64 = (0.5 * w[115]);
        let noise_metadata_schedule_516_0_e4959: f64 = (1.0 - noise_metadata_schedule_516_0_e4958);
        (noise_metadata_schedule_516_0_e4959,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_516_0_e4961;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_517_0_e4979,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[528] == 0.0)) {
        let noise_metadata_schedule_517_0_e4975: f64 = (params[44] * w[116]);
        let noise_metadata_schedule_517_0_e4977: f64 = (noise_metadata_schedule_517_0_e4975 * w[116]);
        (noise_metadata_schedule_517_0_e4977,)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_517_0_e4979;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_518_0_e5001,) = {
    if ((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) {
        let noise_metadata_schedule_518_0_e4990: f64 = (w[186] * w[187]);
        let noise_metadata_schedule_518_0_e4993: f64 = (w[186] * w[186]);
        let noise_metadata_schedule_518_0_e4996: f64 = (w[187] * w[187]);
        let noise_metadata_schedule_518_0_e4997: f64 = (noise_metadata_schedule_518_0_e4993 + noise_metadata_schedule_518_0_e4996);
        let noise_metadata_schedule_518_0_e4998: f64 = (noise_metadata_schedule_518_0_e4997).sqrt();
        let noise_metadata_schedule_518_0_e4999: f64 = (noise_metadata_schedule_518_0_e4990 / noise_metadata_schedule_518_0_e4998);
        (noise_metadata_schedule_518_0_e4999,)
    } else {
        (w[188],)
    }
};
            w[188] = noise_metadata_schedule_518_0_e5001;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_519_0_e5016,) = {
    if ((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) {
        let noise_metadata_schedule_519_0_e5012: f64 = (w[16] - w[230]);
        let noise_metadata_schedule_519_0_e5014: f64 = (noise_metadata_schedule_519_0_e5012 / w[188]);
        (noise_metadata_schedule_519_0_e5014,)
    } else {
        (w[189],)
    }
};
            w[189] = noise_metadata_schedule_519_0_e5016;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_520_0_e5035,) = {
    if ((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) {
        let noise_metadata_schedule_520_0_e5028: f64 = (0.5 * w[188]);
        let noise_metadata_schedule_520_0_e5030: f64 = (noise_metadata_schedule_520_0_e5028 * w[185]);
        let noise_metadata_schedule_520_0_e5032: f64 = (noise_metadata_schedule_520_0_e5030 * w[199]);
        let noise_metadata_schedule_520_0_e5033: f64 = (w[189] + noise_metadata_schedule_520_0_e5032);
        (noise_metadata_schedule_520_0_e5033,)
    } else {
        (w[190],)
    }
};
            w[190] = noise_metadata_schedule_520_0_e5035;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_521_0_e5038: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[529] = noise_metadata_schedule_521_0_e5038;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_522_0_e5051,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[529] != 0.0)) {
        (w[190],)
    } else {
        (w[191],)
    }
};
            w[191] = noise_metadata_schedule_522_0_e5051;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_523_0_e5075,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[529] == 0.0)) {
        let noise_metadata_schedule_523_0_e5066: f64 = (2.0 * params[46]);
        let noise_metadata_schedule_523_0_e5070: f64 = (2.0 * w[115]);
        let noise_metadata_schedule_523_0_e5071: f64 = (1.0 + noise_metadata_schedule_523_0_e5070);
        let noise_metadata_schedule_523_0_e5072: f64 = (noise_metadata_schedule_523_0_e5066 * noise_metadata_schedule_523_0_e5071);
        let noise_metadata_schedule_523_0_e5073: f64 = (1.0 + noise_metadata_schedule_523_0_e5072);
        (noise_metadata_schedule_523_0_e5073,)
    } else {
        (w[192],)
    }
};
            w[192] = noise_metadata_schedule_523_0_e5075;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_524_0_e5097,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[529] == 0.0)) {
        let noise_metadata_schedule_524_0_e5089: f64 = (1.0 + params[46]);
        let noise_metadata_schedule_524_0_e5093: f64 = (2.0 * params[46]);
        let noise_metadata_schedule_524_0_e5094: f64 = (1.0 + noise_metadata_schedule_524_0_e5093);
        let noise_metadata_schedule_524_0_e5095: f64 = (noise_metadata_schedule_524_0_e5089 / noise_metadata_schedule_524_0_e5094);
        (noise_metadata_schedule_524_0_e5095,)
    } else {
        (w[193],)
    }
};
            w[193] = noise_metadata_schedule_524_0_e5097;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_525_0_e5125,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[529] == 0.0)) {
        let noise_metadata_schedule_525_0_e5112: f64 = (0.5 * w[188]);
        let noise_metadata_schedule_525_0_e5114: f64 = (noise_metadata_schedule_525_0_e5112 * w[185]);
        let noise_metadata_schedule_525_0_e5119: f64 = (params[61] * w[192]);
        let noise_metadata_schedule_525_0_e5120: f64 = (w[149] / noise_metadata_schedule_525_0_e5119);
        let noise_metadata_schedule_525_0_e5121: f64 = (w[193] - noise_metadata_schedule_525_0_e5120);
        let noise_metadata_schedule_525_0_e5122: f64 = (noise_metadata_schedule_525_0_e5114 * noise_metadata_schedule_525_0_e5121);
        let noise_metadata_schedule_525_0_e5123: f64 = (w[189] - noise_metadata_schedule_525_0_e5122);
        (noise_metadata_schedule_525_0_e5123,)
    } else {
        (w[194],)
    }
};
            w[194] = noise_metadata_schedule_525_0_e5125;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_526_0_e5155,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[529] == 0.0)) {
        let noise_metadata_schedule_526_0_e5139: f64 = (w[194] - w[190]);
        let noise_metadata_schedule_526_0_e5142: f64 = (w[194] - w[190]);
        let noise_metadata_schedule_526_0_e5143: f64 = (noise_metadata_schedule_526_0_e5139 * noise_metadata_schedule_526_0_e5142);
        let noise_metadata_schedule_526_0_e5146: f64 = (0.1 * w[189]);
        let noise_metadata_schedule_526_0_e5148: f64 = (noise_metadata_schedule_526_0_e5146 * w[189]);
        let noise_metadata_schedule_526_0_e5150: f64 = (noise_metadata_schedule_526_0_e5148 * w[127]);
        let noise_metadata_schedule_526_0_e5152: f64 = (noise_metadata_schedule_526_0_e5150 / params[61]);
        let noise_metadata_schedule_526_0_e5153: f64 = (noise_metadata_schedule_526_0_e5143 + noise_metadata_schedule_526_0_e5152);
        (noise_metadata_schedule_526_0_e5153,)
    } else {
        (w[260],)
    }
};
            w[260] = noise_metadata_schedule_526_0_e5155;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_527_0_e5176,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[529] == 0.0)) {
        let noise_metadata_schedule_527_0_e5170: f64 = (w[194] + w[190]);
        let noise_metadata_schedule_527_0_e5172: f64 = (w[260]).sqrt();
        let noise_metadata_schedule_527_0_e5173: f64 = (noise_metadata_schedule_527_0_e5170 + noise_metadata_schedule_527_0_e5172);
        let noise_metadata_schedule_527_0_e5174: f64 = (0.5 * noise_metadata_schedule_527_0_e5173);
        (noise_metadata_schedule_527_0_e5174,)
    } else {
        (w[191],)
    }
};
            w[191] = noise_metadata_schedule_527_0_e5176;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_528_0_e5191,) = {
    if ((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) {
        let noise_metadata_schedule_528_0_e5187: f64 = (w[191] - w[189]);
        let noise_metadata_schedule_528_0_e5189: f64 = (noise_metadata_schedule_528_0_e5187 / w[191]);
        (noise_metadata_schedule_528_0_e5189,)
    } else {
        (w[267],)
    }
};
            w[267] = noise_metadata_schedule_528_0_e5191;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_529_0_e5193: f64 = (w[267]).abs();
            let noise_metadata_schedule_529_0_e5195: f64 = if noise_metadata_schedule_529_0_e5193 > 1e-7 { 1.0 } else { 0.0 };
            w[530] = noise_metadata_schedule_529_0_e5195;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_530_0_e5212,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[530] != 0.0)) {
        let noise_metadata_schedule_530_0_e5208: f64 = (0.5 * w[188]);
        let noise_metadata_schedule_530_0_e5210: f64 = (noise_metadata_schedule_530_0_e5208 / w[267]);
        (noise_metadata_schedule_530_0_e5210,)
    } else {
        (w[195],)
    }
};
            w[195] = noise_metadata_schedule_530_0_e5212;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_531_0_e5249,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[530] != 0.0)) {
        let noise_metadata_schedule_531_0_e5225: f64 = (w[0] / w[98]);
        let noise_metadata_schedule_531_0_e5227: f64 = (noise_metadata_schedule_531_0_e5225 * w[191]);
        let noise_metadata_schedule_531_0_e5229: f64 = (noise_metadata_schedule_531_0_e5227 * w[195]);
        let noise_metadata_schedule_531_0_e5231: f64 = (-w[98]);
        let noise_metadata_schedule_531_0_e5233: f64 = (noise_metadata_schedule_531_0_e5231 / w[191]);
        let noise_metadata_schedule_531_0_e5234: f64 = (noise_metadata_schedule_531_0_e5233).exp();
        let noise_metadata_schedule_531_0_e5236: f64 = (-w[98]);
        let noise_metadata_schedule_531_0_e5238: f64 = (noise_metadata_schedule_531_0_e5236 / w[191]);
        let noise_metadata_schedule_531_0_e5242: f64 = (w[187] / w[195]);
        let noise_metadata_schedule_531_0_e5243: f64 = (1.0 + noise_metadata_schedule_531_0_e5242);
        let noise_metadata_schedule_531_0_e5244: f64 = (noise_metadata_schedule_531_0_e5238 * noise_metadata_schedule_531_0_e5243);
        let noise_metadata_schedule_531_0_e5245: f64 = (noise_metadata_schedule_531_0_e5244).exp();
        let noise_metadata_schedule_531_0_e5246: f64 = (noise_metadata_schedule_531_0_e5234 - noise_metadata_schedule_531_0_e5245);
        let noise_metadata_schedule_531_0_e5247: f64 = (noise_metadata_schedule_531_0_e5229 * noise_metadata_schedule_531_0_e5246);
        (noise_metadata_schedule_531_0_e5247,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_531_0_e5249;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_532_0_e5271,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] != 0.0)) && (w[527] != 0.0)) && (w[530] == 0.0)) {
        let noise_metadata_schedule_532_0_e5263: f64 = (w[0] * w[187]);
        let noise_metadata_schedule_532_0_e5265: f64 = (-w[98]);
        let noise_metadata_schedule_532_0_e5267: f64 = (noise_metadata_schedule_532_0_e5265 / w[191]);
        let noise_metadata_schedule_532_0_e5268: f64 = (noise_metadata_schedule_532_0_e5267).exp();
        let noise_metadata_schedule_532_0_e5269: f64 = (noise_metadata_schedule_532_0_e5263 * noise_metadata_schedule_532_0_e5268);
        (noise_metadata_schedule_532_0_e5269,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_532_0_e5271;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_533_0_e5274: f64 = if params[38] == 3.0 { 1.0 } else { 0.0 };
            w[531] = noise_metadata_schedule_533_0_e5274;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_534_0_e5277: f64 = if w[230] < params[43] { 1.0 } else { 0.0 };
            w[532] = noise_metadata_schedule_534_0_e5277;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_535_0_e5305,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) {
        let noise_metadata_schedule_535_0_e5291: f64 = (params[43] - w[230]);
        let noise_metadata_schedule_535_0_e5293: f64 = (noise_metadata_schedule_535_0_e5291).powf(params[40]);
        let noise_metadata_schedule_535_0_e5298: f64 = (params[47] + w[149]);
        let noise_metadata_schedule_535_0_e5299: f64 = (w[149] / noise_metadata_schedule_535_0_e5298);
        let noise_metadata_schedule_535_0_e5300: f64 = (1.0 - noise_metadata_schedule_535_0_e5299);
        let noise_metadata_schedule_535_0_e5302: f64 = (noise_metadata_schedule_535_0_e5300).powf(params[48]);
        let noise_metadata_schedule_535_0_e5303: f64 = (noise_metadata_schedule_535_0_e5293 * noise_metadata_schedule_535_0_e5302);
        (noise_metadata_schedule_535_0_e5303,)
    } else {
        (w[200],)
    }
};
            w[200] = noise_metadata_schedule_535_0_e5305;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_536_0_e5308: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[533] = noise_metadata_schedule_536_0_e5308;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_537_0_e5324,) = {
    if ((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[533] != 0.0)) {
        (w[200],)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_537_0_e5324;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_538_0_e5345,) = {
    if ((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[533] == 0.0)) {
        let noise_metadata_schedule_538_0_e5341: f64 = (w[149] - params[51]);
        let noise_metadata_schedule_538_0_e5343: f64 = (noise_metadata_schedule_538_0_e5341 / params[47]);
        (noise_metadata_schedule_538_0_e5343,)
    } else {
        (w[202],)
    }
};
            w[202] = noise_metadata_schedule_538_0_e5345;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_539_0_e5366,) = {
    if ((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[533] == 0.0)) {
        let noise_metadata_schedule_539_0_e5362: f64 = (w[202] - 1.0);
        let noise_metadata_schedule_539_0_e5364: f64 = (noise_metadata_schedule_539_0_e5362 / params[50]);
        (noise_metadata_schedule_539_0_e5364,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_539_0_e5366;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_540_0_e5369: f64 = if w[202] < 1.0 { 1.0 } else { 0.0 };
            w[534] = noise_metadata_schedule_540_0_e5369;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_541_0_e5396,) = {
    if (((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[533] == 0.0)) && (w[534] != 0.0)) {
        let noise_metadata_schedule_541_0_e5390: f64 = (w[259]).exp();
        let noise_metadata_schedule_541_0_e5391: f64 = (1.0 + noise_metadata_schedule_541_0_e5390);
        let noise_metadata_schedule_541_0_e5392: f64 = (noise_metadata_schedule_541_0_e5391).ln();
        let noise_metadata_schedule_541_0_e5393: f64 = (params[50] * noise_metadata_schedule_541_0_e5392);
        let noise_metadata_schedule_541_0_e5394: f64 = (1.0 + noise_metadata_schedule_541_0_e5393);
        (noise_metadata_schedule_541_0_e5394,)
    } else {
        (w[203],)
    }
};
            w[203] = noise_metadata_schedule_541_0_e5396;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_542_0_e5425,) = {
    if (((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[533] == 0.0)) && (w[534] == 0.0)) {
        let noise_metadata_schedule_542_0_e5418: f64 = (-w[259]);
        let noise_metadata_schedule_542_0_e5419: f64 = (noise_metadata_schedule_542_0_e5418).exp();
        let noise_metadata_schedule_542_0_e5420: f64 = (1.0 + noise_metadata_schedule_542_0_e5419);
        let noise_metadata_schedule_542_0_e5421: f64 = (noise_metadata_schedule_542_0_e5420).ln();
        let noise_metadata_schedule_542_0_e5422: f64 = (params[50] * noise_metadata_schedule_542_0_e5421);
        let noise_metadata_schedule_542_0_e5423: f64 = (w[202] + noise_metadata_schedule_542_0_e5422);
        (noise_metadata_schedule_542_0_e5423,)
    } else {
        (w[203],)
    }
};
            w[203] = noise_metadata_schedule_542_0_e5425;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_543_0_e5446,) = {
    if ((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[533] == 0.0)) {
        let noise_metadata_schedule_543_0_e5443: f64 = (w[203]).powf(params[49]);
        let noise_metadata_schedule_543_0_e5444: f64 = (w[200] * noise_metadata_schedule_543_0_e5443);
        (noise_metadata_schedule_543_0_e5444,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_543_0_e5446;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_544_0_e5448: f64 = (-w[310]);
            let noise_metadata_schedule_544_0_e5450: f64 = (noise_metadata_schedule_544_0_e5448 * w[201]);
            let noise_metadata_schedule_544_0_e5452: f64 = if noise_metadata_schedule_544_0_e5450 < params[134] { 1.0 } else { 0.0 };
            w[535] = noise_metadata_schedule_544_0_e5452;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_545_0_e5472,) = {
    if ((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[535] != 0.0)) {
        let noise_metadata_schedule_545_0_e5467: f64 = (-w[310]);
        let noise_metadata_schedule_545_0_e5469: f64 = (noise_metadata_schedule_545_0_e5467 * w[201]);
        let noise_metadata_schedule_545_0_e5470: f64 = (noise_metadata_schedule_545_0_e5469).exp();
        (noise_metadata_schedule_545_0_e5470,)
    } else {
        (w[313],)
    }
};
            w[313] = noise_metadata_schedule_545_0_e5472;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_546_0_e5490,) = {
    if ((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[535] == 0.0)) {
        let noise_metadata_schedule_546_0_e5488: f64 = (params[134]).exp();
        (noise_metadata_schedule_546_0_e5488,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_546_0_e5490;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_547_0_e5516,) = {
    if ((((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) && (w[535] == 0.0)) {
        let noise_metadata_schedule_547_0_e5508: f64 = (-w[310]);
        let noise_metadata_schedule_547_0_e5510: f64 = (noise_metadata_schedule_547_0_e5508 * w[201]);
        let noise_metadata_schedule_547_0_e5512: f64 = (noise_metadata_schedule_547_0_e5510 - params[134]);
        let noise_metadata_schedule_547_0_e5513: f64 = (1.0 + noise_metadata_schedule_547_0_e5512);
        let noise_metadata_schedule_547_0_e5514: f64 = (w[275] * noise_metadata_schedule_547_0_e5513);
        (noise_metadata_schedule_547_0_e5514,)
    } else {
        (w[313],)
    }
};
            w[313] = noise_metadata_schedule_547_0_e5516;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_548_0_e5538,) = {
    if (((((w[521] != 0.0) && (w[522] == 0.0)) && (w[526] == 0.0)) && (w[531] != 0.0)) && (w[532] != 0.0)) {
        let noise_metadata_schedule_548_0_e5530: f64 = (params[39] / w[310]);
        let noise_metadata_schedule_548_0_e5533: f64 = (params[43] - w[230]);
        let noise_metadata_schedule_548_0_e5534: f64 = (noise_metadata_schedule_548_0_e5530 * noise_metadata_schedule_548_0_e5533);
        let noise_metadata_schedule_548_0_e5536: f64 = (noise_metadata_schedule_548_0_e5534 * w[313]);
        (noise_metadata_schedule_548_0_e5536,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_548_0_e5538;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_549_0_e5541: f64 = if w[196] > 0.0 { 1.0 } else { 0.0 };
            w[536] = noise_metadata_schedule_549_0_e5541;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_550_0_e5544: f64 = if params[52] == 1.0 { 1.0 } else { 0.0 };
            w[537] = noise_metadata_schedule_550_0_e5544;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_551_0_e5570,) = {
    if (((w[521] != 0.0) && (w[536] != 0.0)) && (w[537] != 0.0)) {
        let noise_metadata_schedule_551_0_e5554: f64 = (w[30] + w[175]);
        let noise_metadata_schedule_551_0_e5555: f64 = (w[149] * noise_metadata_schedule_551_0_e5554);
        let noise_metadata_schedule_551_0_e5556: f64 = (w[6] / noise_metadata_schedule_551_0_e5555);
        let noise_metadata_schedule_551_0_e5559: f64 = (w[146] / w[35]);
        let noise_metadata_schedule_551_0_e5561: f64 = (noise_metadata_schedule_551_0_e5559 * w[42]);
        let noise_metadata_schedule_551_0_e5562: f64 = (noise_metadata_schedule_551_0_e5556 + noise_metadata_schedule_551_0_e5561);
        let noise_metadata_schedule_551_0_e5566: f64 = (w[30] + w[175]);
        let noise_metadata_schedule_551_0_e5567: f64 = (w[28] / noise_metadata_schedule_551_0_e5566);
        let noise_metadata_schedule_551_0_e5568: f64 = (noise_metadata_schedule_551_0_e5562 + noise_metadata_schedule_551_0_e5567);
        (noise_metadata_schedule_551_0_e5568,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_551_0_e5570;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_552_0_e5573: f64 = if params[38] == 3.0 { 1.0 } else { 0.0 };
            w[538] = noise_metadata_schedule_552_0_e5573;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_553_0_e5587,) = {
    if ((((w[521] != 0.0) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] != 0.0)) {
        let noise_metadata_schedule_553_0_e5583: f64 = (w[196] - w[197]);
        let noise_metadata_schedule_553_0_e5585: f64 = (noise_metadata_schedule_553_0_e5583 / 1e-6);
        (noise_metadata_schedule_553_0_e5585,)
    } else {
        (w[259],)
    }
};
            w[259] = noise_metadata_schedule_553_0_e5587;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_554_0_e5590: f64 = if w[196] < w[197] { 1.0 } else { 0.0 };
            w[539] = noise_metadata_schedule_554_0_e5590;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_555_0_e5610,) = {
    if (((((w[521] != 0.0) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] != 0.0)) && (w[539] != 0.0)) {
        let noise_metadata_schedule_555_0_e5604: f64 = (w[259]).exp();
        let noise_metadata_schedule_555_0_e5605: f64 = (1.0 + noise_metadata_schedule_555_0_e5604);
        let noise_metadata_schedule_555_0_e5606: f64 = (noise_metadata_schedule_555_0_e5605).ln();
        let noise_metadata_schedule_555_0_e5607: f64 = (1e-6 * noise_metadata_schedule_555_0_e5606);
        let noise_metadata_schedule_555_0_e5608: f64 = (w[196] - noise_metadata_schedule_555_0_e5607);
        (noise_metadata_schedule_555_0_e5608,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_555_0_e5610;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_556_0_e5632,) = {
    if (((((w[521] != 0.0) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] != 0.0)) && (w[539] == 0.0)) {
        let noise_metadata_schedule_556_0_e5625: f64 = (-w[259]);
        let noise_metadata_schedule_556_0_e5626: f64 = (noise_metadata_schedule_556_0_e5625).exp();
        let noise_metadata_schedule_556_0_e5627: f64 = (1.0 + noise_metadata_schedule_556_0_e5626);
        let noise_metadata_schedule_556_0_e5628: f64 = (noise_metadata_schedule_556_0_e5627).ln();
        let noise_metadata_schedule_556_0_e5629: f64 = (1e-6 * noise_metadata_schedule_556_0_e5628);
        let noise_metadata_schedule_556_0_e5630: f64 = (w[197] - noise_metadata_schedule_556_0_e5629);
        (noise_metadata_schedule_556_0_e5630,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_556_0_e5632;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_557_0_e5644,) = {
    if ((((w[521] != 0.0) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] != 0.0)) {
        let noise_metadata_schedule_557_0_e5642: f64 = (w[149] * w[196]);
        (noise_metadata_schedule_557_0_e5642,)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_557_0_e5644;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_558_0_e5663,) = {
    if ((((w[521] != 0.0) && (w[536] != 0.0)) && (w[537] != 0.0)) && (w[538] == 0.0)) {
        let noise_metadata_schedule_558_0_e5655: f64 = (w[149] * w[196]);
        let noise_metadata_schedule_558_0_e5657: f64 = (noise_metadata_schedule_558_0_e5655 * w[197]);
        let noise_metadata_schedule_558_0_e5660: f64 = (w[196] + w[197]);
        let noise_metadata_schedule_558_0_e5661: f64 = (noise_metadata_schedule_558_0_e5657 / noise_metadata_schedule_558_0_e5660);
        (noise_metadata_schedule_558_0_e5661,)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_558_0_e5663;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_559_0_e5674,) = {
    if (((w[521] != 0.0) && (w[536] != 0.0)) && (w[537] == 0.0)) {
        let noise_metadata_schedule_559_0_e5672: f64 = (w[149] * w[196]);
        (noise_metadata_schedule_559_0_e5672,)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_559_0_e5674;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_10(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 571], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1fe0038) != 0 {
            let noise_metadata_schedule_637_0_e6439: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_637_0_e6441: f64 = (noise_metadata_schedule_637_0_e6439 * w[2]);
            w[281] = noise_metadata_schedule_637_0_e6441;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_638_0_e6444: f64 = (w[281] / w[28]);
            w[282] = noise_metadata_schedule_638_0_e6444;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_639_0_e6447: f64 = (w[281] / w[30]);
            w[283] = noise_metadata_schedule_639_0_e6447;
        }
        if (active[0] & 0x1520000) != 0 {
            let noise_metadata_schedule_640_0_e6450: f64 = (w[281] * w[101]);
            w[284] = noise_metadata_schedule_640_0_e6450;
        }
        if (active[0] & 0x240000) != 0 {
            let noise_metadata_schedule_641_0_e6453: f64 = (w[281] * w[102]);
            w[285] = noise_metadata_schedule_641_0_e6453;
        }
        if (active[0] & 0x880000) != 0 {
            let noise_metadata_schedule_642_0_e6456: f64 = (w[281] * w[103]);
            w[286] = noise_metadata_schedule_642_0_e6456;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_643_0_e6459: f64 = (w[281] / w[175]);
            let noise_metadata_schedule_643_0_e6462: f64 = (4.0 * w[247]);
            let noise_metadata_schedule_643_0_e6464: f64 = (noise_metadata_schedule_643_0_e6462 + 5.0);
            let noise_metadata_schedule_643_0_e6465: f64 = (noise_metadata_schedule_643_0_e6459 * noise_metadata_schedule_643_0_e6464);
            let noise_metadata_schedule_643_0_e6467: f64 = (noise_metadata_schedule_643_0_e6465 * 0.3333333333333333);
            w[287] = noise_metadata_schedule_643_0_e6467;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_644_0_e6470: f64 = (w[148] + w[147]);
            let noise_metadata_schedule_644_0_e6472: f64 = (noise_metadata_schedule_644_0_e6470 / w[146]);
            w[303] = noise_metadata_schedule_644_0_e6472;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_645_0_e6475: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_645_0_e6477: f64 = (w[303]).abs();
            let noise_metadata_schedule_645_0_e6478: f64 = (noise_metadata_schedule_645_0_e6475 * noise_metadata_schedule_645_0_e6477);
            w[288] = noise_metadata_schedule_645_0_e6478;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_646_0_e6481: f64 = if params[129] > 0.0 { 1.0 } else { 0.0 };
            w[555] = noise_metadata_schedule_646_0_e6481;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_647_0_e6488,) = {
    if (w[555] != 0.0) {
        let noise_metadata_schedule_647_0_e6485: f64 = (w[198] / w[303]);
        let noise_metadata_schedule_647_0_e6486: f64 = (noise_metadata_schedule_647_0_e6485).abs();
        (noise_metadata_schedule_647_0_e6486,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_647_0_e6488;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_648_0_e6493,) = {
    if (w[555] == 0.0) {
        (0.0,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_648_0_e6493;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_649_0_e6496: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_649_0_e6498: f64 = (noise_metadata_schedule_649_0_e6496 * w[198]);
            let noise_metadata_schedule_649_0_e6501: f64 = (w[304] + 1.0);
            let noise_metadata_schedule_649_0_e6502: f64 = (noise_metadata_schedule_649_0_e6498 * noise_metadata_schedule_649_0_e6501);
            w[300] = noise_metadata_schedule_649_0_e6502;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_658_0_e6554: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_658_0_e6557: f64 = (w[151] + w[153]);
            let noise_metadata_schedule_658_0_e6559: f64 = (noise_metadata_schedule_658_0_e6557 - w[57]);
            let noise_metadata_schedule_658_0_e6561: f64 = (noise_metadata_schedule_658_0_e6559 + w[327]);
            let noise_metadata_schedule_658_0_e6563: f64 = (noise_metadata_schedule_658_0_e6561 + w[326]);
            let noise_metadata_schedule_658_0_e6564: f64 = (noise_metadata_schedule_658_0_e6563).abs();
            let noise_metadata_schedule_658_0_e6565: f64 = (noise_metadata_schedule_658_0_e6554 * noise_metadata_schedule_658_0_e6564);
            w[289] = noise_metadata_schedule_658_0_e6565;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_659_0_e6568: f64 = (w[151] + w[152]);
            w[301] = noise_metadata_schedule_659_0_e6568;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_660_0_e6571: f64 = (w[301]).abs();
            let noise_metadata_schedule_660_0_e6573: f64 = (noise_metadata_schedule_660_0_e6571).powf(params[125]);
            let noise_metadata_schedule_660_0_e6574: f64 = (params[127] * noise_metadata_schedule_660_0_e6573);
            w[290] = noise_metadata_schedule_660_0_e6574;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_661_0_e6577: f64 = if w[301] < 0.0 { 1.0 } else { 0.0 };
            w[559] = noise_metadata_schedule_661_0_e6577;
        }
        if (active[0] & 0x40) != 0 {
            let (noise_metadata_schedule_662_0_e6582,) = {
    if (w[559] != 0.0) {
        let noise_metadata_schedule_662_0_e6580: f64 = (-w[290]);
        (noise_metadata_schedule_662_0_e6580,)
    } else {
        (w[290],)
    }
};
            w[290] = noise_metadata_schedule_662_0_e6582;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_663_0_e6585: f64 = (w[153] + w[155]);
            let noise_metadata_schedule_663_0_e6587: f64 = (noise_metadata_schedule_663_0_e6585 + w[156]);
            w[302] = noise_metadata_schedule_663_0_e6587;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_664_0_e6590: f64 = (w[302]).abs();
            let noise_metadata_schedule_664_0_e6592: f64 = (noise_metadata_schedule_664_0_e6590).powf(params[126]);
            let noise_metadata_schedule_664_0_e6593: f64 = (params[128] * noise_metadata_schedule_664_0_e6592);
            w[291] = noise_metadata_schedule_664_0_e6593;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_665_0_e6596: f64 = if w[302] < 0.0 { 1.0 } else { 0.0 };
            w[560] = noise_metadata_schedule_665_0_e6596;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_666_0_e6601,) = {
    if (w[560] != 0.0) {
        let noise_metadata_schedule_666_0_e6599: f64 = (-w[291]);
        (noise_metadata_schedule_666_0_e6599,)
    } else {
        (w[291],)
    }
};
            w[291] = noise_metadata_schedule_666_0_e6601;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_667_0_e6604: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_667_0_e6607: f64 = (w[152] + w[155]);
            let noise_metadata_schedule_667_0_e6609: f64 = (noise_metadata_schedule_667_0_e6607 + w[156]);
            let noise_metadata_schedule_667_0_e6610: f64 = (noise_metadata_schedule_667_0_e6609).abs();
            let noise_metadata_schedule_667_0_e6611: f64 = (noise_metadata_schedule_667_0_e6604 * noise_metadata_schedule_667_0_e6610);
            w[292] = noise_metadata_schedule_667_0_e6611;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_668_0_e6614: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_668_0_e6616: f64 = (w[154]).abs();
            let noise_metadata_schedule_668_0_e6617: f64 = (noise_metadata_schedule_668_0_e6614 * noise_metadata_schedule_668_0_e6616);
            w[293] = noise_metadata_schedule_668_0_e6617;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_669_0_e6620: f64 = (w[154]).abs();
            let noise_metadata_schedule_669_0_e6622: f64 = (noise_metadata_schedule_669_0_e6620).powf(params[125]);
            let noise_metadata_schedule_669_0_e6623: f64 = (params[127] * noise_metadata_schedule_669_0_e6622);
            w[294] = noise_metadata_schedule_669_0_e6623;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_670_0_e6626: f64 = if w[154] < 0.0 { 1.0 } else { 0.0 };
            w[561] = noise_metadata_schedule_670_0_e6626;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_671_0_e6631,) = {
    if (w[561] != 0.0) {
        let noise_metadata_schedule_671_0_e6629: f64 = (-w[294]);
        (noise_metadata_schedule_671_0_e6629,)
    } else {
        (w[294],)
    }
};
            w[294] = noise_metadata_schedule_671_0_e6631;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_672_0_e6634: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_672_0_e6636: f64 = (w[82]).abs();
            let noise_metadata_schedule_672_0_e6637: f64 = (noise_metadata_schedule_672_0_e6634 * noise_metadata_schedule_672_0_e6636);
            w[295] = noise_metadata_schedule_672_0_e6637;
        }
        if (active[0] & 0x800) != 0 {
            let noise_metadata_schedule_673_0_e6640: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_673_0_e6642: f64 = (w[157]).abs();
            let noise_metadata_schedule_673_0_e6643: f64 = (noise_metadata_schedule_673_0_e6640 * noise_metadata_schedule_673_0_e6642);
            w[296] = noise_metadata_schedule_673_0_e6643;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_674_0_e6648: f64 = (params[5] * params[32]);
            let noise_metadata_schedule_674_0_e6649: f64 = (1.0 - noise_metadata_schedule_674_0_e6648);
            let noise_metadata_schedule_674_0_e6650: f64 = (params[127] * noise_metadata_schedule_674_0_e6649);
            let noise_metadata_schedule_674_0_e6652: f64 = (w[157]).abs();
            let noise_metadata_schedule_674_0_e6656: f64 = (params[5] * params[32]);
            let noise_metadata_schedule_674_0_e6657: f64 = (1.0 - noise_metadata_schedule_674_0_e6656);
            let noise_metadata_schedule_674_0_e6658: f64 = (noise_metadata_schedule_674_0_e6652 / noise_metadata_schedule_674_0_e6657);
            let noise_metadata_schedule_674_0_e6660: f64 = (noise_metadata_schedule_674_0_e6658).powf(params[125]);
            let noise_metadata_schedule_674_0_e6661: f64 = (noise_metadata_schedule_674_0_e6650 * noise_metadata_schedule_674_0_e6660);
            w[298] = noise_metadata_schedule_674_0_e6661;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_675_0_e6664: f64 = if w[157] < 0.0 { 1.0 } else { 0.0 };
            w[562] = noise_metadata_schedule_675_0_e6664;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_676_0_e6669,) = {
    if (w[562] != 0.0) {
        let noise_metadata_schedule_676_0_e6667: f64 = (-w[298]);
        (noise_metadata_schedule_676_0_e6667,)
    } else {
        (w[298],)
    }
};
            w[298] = noise_metadata_schedule_676_0_e6669;
        }
        if (active[0] & 0x2000) != 0 {
            let noise_metadata_schedule_677_0_e6672: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_677_0_e6674: f64 = (w[169]).abs();
            let noise_metadata_schedule_677_0_e6675: f64 = (noise_metadata_schedule_677_0_e6672 * noise_metadata_schedule_677_0_e6674);
            let noise_metadata_schedule_677_0_e6677: f64 = (noise_metadata_schedule_677_0_e6675 * params[5]);
            w[297] = noise_metadata_schedule_677_0_e6677;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_678_0_e6680: f64 = if params[32] == 0.0 { 1.0 } else { 0.0 };
            w[563] = noise_metadata_schedule_678_0_e6680;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_679_0_e6684,) = {
    if (w[563] != 0.0) {
        (0.0,)
    } else {
        (w[299],)
    }
};
            w[299] = noise_metadata_schedule_679_0_e6684;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_680_0_e6700,) = {
    if (w[563] == 0.0) {
        let noise_metadata_schedule_680_0_e6689: f64 = (params[127] * params[5]);
        let noise_metadata_schedule_680_0_e6691: f64 = (noise_metadata_schedule_680_0_e6689 * params[32]);
        let noise_metadata_schedule_680_0_e6693: f64 = (w[169]).abs();
        let noise_metadata_schedule_680_0_e6695: f64 = (noise_metadata_schedule_680_0_e6693 / params[32]);
        let noise_metadata_schedule_680_0_e6697: f64 = (noise_metadata_schedule_680_0_e6695).powf(params[125]);
        let noise_metadata_schedule_680_0_e6698: f64 = (noise_metadata_schedule_680_0_e6691 * noise_metadata_schedule_680_0_e6697);
        (noise_metadata_schedule_680_0_e6698,)
    } else {
        (w[299],)
    }
};
            w[299] = noise_metadata_schedule_680_0_e6700;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_681_0_e6703: f64 = if w[169] < 0.0 { 1.0 } else { 0.0 };
            w[564] = noise_metadata_schedule_681_0_e6703;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_682_0_e6708,) = {
    if (w[564] != 0.0) {
        let noise_metadata_schedule_682_0_e6706: f64 = (-w[299]);
        (noise_metadata_schedule_682_0_e6706,)
    } else {
        (w[299],)
    }
};
            w[299] = noise_metadata_schedule_682_0_e6708;
        }
    }
}

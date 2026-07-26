#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 28] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_S_ISUB_INT", label: Some("isub_int"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_S_ISUB", label: Some("isub"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_S_XISUB", label: Some("xisub"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 63, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 64, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 630];
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
            w[624] != 0.0
        };
        let noise_source_16_active = {
            let noise_16_activation_e496: f64 = if (w[624] == 0.0) { 1.0 } else { 0.0 };
            noise_16_activation_e496 != 0.0
        };
        let noise_source_17_active = {
            true
        };
        let noise_source_18_active = {
            true
        };
        let noise_source_19_active = {
            true
        };
        let noise_source_20_active = {
            let noise_20_activation_e521: f64 = if ((w[625] != 0.0) && (w[626] != 0.0)) { 1.0 } else { 0.0 };
            noise_20_activation_e521 != 0.0
        };
        let noise_source_21_active = {
            let noise_21_activation_e531: f64 = if ((w[625] != 0.0) && (w[626] != 0.0)) { 1.0 } else { 0.0 };
            noise_21_activation_e531 != 0.0
        };
        let noise_source_22_active = {
            let noise_22_activation_e541: f64 = if ((w[625] != 0.0) && (w[626] != 0.0)) { 1.0 } else { 0.0 };
            noise_22_activation_e541 != 0.0
        };
        let noise_source_23_active = {
            let noise_23_activation_e552: f64 = if ((w[625] != 0.0) && (w[626] == 0.0)) { 1.0 } else { 0.0 };
            noise_23_activation_e552 != 0.0
        };
        let noise_source_24_active = {
            let noise_24_activation_e563: f64 = if ((w[625] != 0.0) && (w[626] == 0.0)) { 1.0 } else { 0.0 };
            noise_24_activation_e563 != 0.0
        };
        let noise_source_25_active = {
            let noise_25_activation_e574: f64 = if ((w[625] == 0.0) && (w[627] != 0.0)) { 1.0 } else { 0.0 };
            noise_25_activation_e574 != 0.0
        };
        let noise_source_26_active = {
            let noise_26_activation_e585: f64 = if ((w[625] == 0.0) && (w[627] != 0.0)) { 1.0 } else { 0.0 };
            noise_26_activation_e585 != 0.0
        };
        let noise_source_27_active = {
            let noise_27_activation_e597: f64 = if ((w[625] == 0.0) && (w[627] == 0.0)) { 1.0 } else { 0.0 };
            noise_27_activation_e597 != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active, noise_source_4_active, noise_source_5_active, noise_source_6_active, noise_source_7_active, noise_source_8_active, noise_source_9_active, noise_source_10_active, noise_source_11_active, noise_source_12_active, noise_source_13_active, noise_source_14_active, noise_source_15_active, noise_source_16_active, noise_source_17_active, noise_source_18_active, noise_source_19_active, noise_source_20_active, noise_source_21_active, noise_source_22_active, noise_source_23_active, noise_source_24_active, noise_source_25_active, noise_source_26_active, noise_source_27_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3) | ((noise_source_4_active as u128) << 4) | ((noise_source_5_active as u128) << 5) | ((noise_source_6_active as u128) << 6) | ((noise_source_7_active as u128) << 7) | ((noise_source_8_active as u128) << 8) | ((noise_source_9_active as u128) << 9) | ((noise_source_10_active as u128) << 10) | ((noise_source_11_active as u128) << 11) | ((noise_source_12_active as u128) << 12) | ((noise_source_13_active as u128) << 13) | ((noise_source_14_active as u128) << 14) | ((noise_source_15_active as u128) << 15) | ((noise_source_16_active as u128) << 16) | ((noise_source_17_active as u128) << 17) | ((noise_source_18_active as u128) << 18) | ((noise_source_19_active as u128) << 19) | ((noise_source_20_active as u128) << 20) | ((noise_source_21_active as u128) << 21) | ((noise_source_22_active as u128) << 22) | ((noise_source_23_active as u128) << 23) | ((noise_source_24_active as u128) << 24) | ((noise_source_25_active as u128) << 25) | ((noise_source_26_active as u128) << 26) | ((noise_source_27_active as u128) << 27)];
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
        self.noise_metadata_schedule_part_11(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e8670: f64 = 1.0;
            let noise_0_psd_e400: f64 = (w[315] * params.p1);
            let noise_0_psd_e8671: f64 = (noise_0_psd_e8670 * noise_0_psd_e400);
            let psd = noise_0_psd_e8671;
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
            let noise_1_psd_e8673: f64 = 1.0;
            let noise_1_psd_e414: f64 = (w[327] * params.p1);
            let noise_1_psd_e8674: f64 = (noise_1_psd_e8673 * noise_1_psd_e414);
            let psd = noise_1_psd_e8674;
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
            let noise_2_psd_e8676: f64 = 1.0;
            let noise_2_psd_e419: f64 = (w[316] * params.p1);
            let noise_2_psd_e8677: f64 = (noise_2_psd_e8676 * noise_2_psd_e419);
            let psd = noise_2_psd_e8677;
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
            let noise_3_psd_e8679: f64 = 1.0;
            let noise_3_psd_e424: f64 = (w[309] * params.p1);
            let noise_3_psd_e8680: f64 = (noise_3_psd_e8679 * noise_3_psd_e424);
            let psd = noise_3_psd_e8680;
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
            let noise_4_psd_e8682: f64 = 1.0;
            let noise_4_psd_e429: f64 = (w[310] * params.p1);
            let noise_4_psd_e8683: f64 = (noise_4_psd_e8682 * noise_4_psd_e429);
            let psd = noise_4_psd_e8683;
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
            let noise_5_psd_e8685: f64 = 1.0;
            let noise_5_psd_e434: f64 = (w[314] * params.p1);
            let noise_5_psd_e8686: f64 = (noise_5_psd_e8685 * noise_5_psd_e434);
            let psd = noise_5_psd_e8686;
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
            let noise_6_psd_e8688: f64 = 1.0;
            let noise_6_psd_e439: f64 = (w[317] * params.p1);
            let noise_6_psd_e8689: f64 = (noise_6_psd_e8688 * noise_6_psd_e439);
            let psd = noise_6_psd_e8689;
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
            let noise_7_psd_e8691: f64 = 1.0;
            let noise_7_psd_e445: f64 = (w[318] * params.p1);
            let noise_7_psd_e8692: f64 = (noise_7_psd_e8691 * noise_7_psd_e445);
            let psd = noise_7_psd_e8692;
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
            let noise_8_psd_e8694: f64 = 1.0;
            let noise_8_psd_e451: f64 = (w[319] * params.p1);
            let noise_8_psd_e8695: f64 = (noise_8_psd_e8694 * noise_8_psd_e451);
            let psd = noise_8_psd_e8695;
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
            let noise_9_psd_e8697: f64 = 1.0;
            let noise_9_psd_e456: f64 = (w[320] * params.p1);
            let noise_9_psd_e8698: f64 = (noise_9_psd_e8697 * noise_9_psd_e456);
            let psd = noise_9_psd_e8698;
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
            let noise_10_psd_e8700: f64 = 1.0;
            let noise_10_psd_e461: f64 = (w[321] * params.p1);
            let noise_10_psd_e8701: f64 = (noise_10_psd_e8700 * noise_10_psd_e461);
            let psd = noise_10_psd_e8701;
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
            let noise_11_psd_e8703: f64 = 1.0;
            let noise_11_psd_e467: f64 = (w[323] * params.p1);
            let noise_11_psd_e8704: f64 = (noise_11_psd_e8703 * noise_11_psd_e467);
            let psd = noise_11_psd_e8704;
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
            let noise_12_psd_e8706: f64 = 1.0;
            let noise_12_psd_e472: f64 = (w[325] * params.p1);
            let noise_12_psd_e8707: f64 = (noise_12_psd_e8706 * noise_12_psd_e472);
            let psd = noise_12_psd_e8707;
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
            let noise_13_psd_e8709: f64 = 1.0;
            let noise_13_psd_e478: f64 = (w[324] * params.p1);
            let noise_13_psd_e8710: f64 = (noise_13_psd_e8709 * noise_13_psd_e478);
            let psd = noise_13_psd_e8710;
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
            let noise_14_psd_e8712: f64 = 1.0;
            let noise_14_psd_e483: f64 = (w[326] * params.p1);
            let noise_14_psd_e8713: f64 = (noise_14_psd_e8712 * noise_14_psd_e483);
            let psd = noise_14_psd_e8713;
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
            let noise_15_psd_e8715: f64 = 1.0;
            let noise_15_psd_e490: f64 = (w[322] * params.p1);
            let noise_15_psd_e8716: f64 = (noise_15_psd_e8715 * noise_15_psd_e490);
            let psd = noise_15_psd_e8716;
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
            let noise_16_psd_e8718: f64 = 1.0;
            let noise_16_psd_e499: f64 = (w[322] * params.p1);
            let noise_16_psd_e8719: f64 = (noise_16_psd_e8718 * noise_16_psd_e499);
            let psd = noise_16_psd_e8719;
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
            let noise_17_psd_e8721: f64 = 1.0;
            let noise_17_psd_e506: f64 = (w[330] * params.p1);
            let noise_17_psd_e8722: f64 = (noise_17_psd_e8721 * noise_17_psd_e506);
            let psd = noise_17_psd_e8722;
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
            let noise_18_psd_e8724: f64 = 1.0;
            let noise_18_psd_e511: f64 = (w[331] * params.p1);
            let noise_18_psd_e8725: f64 = (noise_18_psd_e8724 * noise_18_psd_e511);
            let psd = noise_18_psd_e8725;
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
            let noise_19_psd_e8727: f64 = 1.0;
            let noise_19_psd_e516: f64 = (w[332] * params.p1);
            let noise_19_psd_e8728: f64 = (noise_19_psd_e8727 * noise_19_psd_e516);
            let psd = noise_19_psd_e8728;
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
            let noise_20_psd_e8730: f64 = 1.0;
            let noise_20_psd_e524: f64 = (w[311] * params.p1);
            let noise_20_psd_e8731: f64 = (noise_20_psd_e8730 * noise_20_psd_e524);
            let psd = noise_20_psd_e8731;
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
            let noise_21_psd_e8733: f64 = 1.0;
            let noise_21_psd_e534: f64 = (w[312] * params.p1);
            let noise_21_psd_e8734: f64 = (noise_21_psd_e8733 * noise_21_psd_e534);
            let psd = noise_21_psd_e8734;
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
            let noise_22_psd_e8736: f64 = 1.0;
            let noise_22_psd_e544: f64 = (w[313] * params.p1);
            let noise_22_psd_e8737: f64 = (noise_22_psd_e8736 * noise_22_psd_e544);
            let psd = noise_22_psd_e8737;
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
            let noise_23_psd_e8739: f64 = 1.0;
            let noise_23_psd_e555: f64 = (w[311] * params.p1);
            let noise_23_psd_e8740: f64 = (noise_23_psd_e8739 * noise_23_psd_e555);
            let psd = noise_23_psd_e8740;
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
            let noise_24_psd_e8742: f64 = 1.0;
            let noise_24_psd_e566: f64 = (w[312] * params.p1);
            let noise_24_psd_e8743: f64 = (noise_24_psd_e8742 * noise_24_psd_e566);
            let psd = noise_24_psd_e8743;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 24, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 24, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(24, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[25] {
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_25_psd_e8745: f64 = 1.0;
            let noise_25_psd_e577: f64 = (w[311] * params.p1);
            let noise_25_psd_e8746: f64 = (noise_25_psd_e8745 * noise_25_psd_e577);
            let psd = noise_25_psd_e8746;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 25, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 25, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(25, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[26] {
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_26_psd_e8748: f64 = 1.0;
            let noise_26_psd_e588: f64 = (w[313] * params.p1);
            let noise_26_psd_e8749: f64 = (noise_26_psd_e8748 * noise_26_psd_e588);
            let psd = noise_26_psd_e8749;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 26, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 26, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(26, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[27] {
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_27_psd_e8751: f64 = 1.0;
            let noise_27_psd_e600: f64 = (w[311] * params.p1);
            let noise_27_psd_e8752: f64 = (noise_27_psd_e8751 * noise_27_psd_e600);
            let psd = noise_27_psd_e8752;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 27, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 27, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(27, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630]) {
        let params = &*self.params;
        let noise_activation_schedule_751_0_e7552: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
        w[624] = noise_activation_schedule_751_0_e7552;
        let noise_activation_schedule_752_0_e7555: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
        w[625] = noise_activation_schedule_752_0_e7555;
        let noise_activation_schedule_753_0_e7558: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
        w[626] = noise_activation_schedule_753_0_e7558;
        let noise_activation_schedule_754_0_e7561: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
        w[627] = noise_activation_schedule_754_0_e7561;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_0_0_e607: f64 = if params.p3 == 1.0 { 1.0 } else { 0.0 };
            w[484] = noise_metadata_schedule_0_0_e607;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_1_0_e611,) = {
    if (w[484] != 0.0) {
        (70300000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_1_0_e611;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_2_0_e615,) = {
    if (w[484] != 0.0) {
        (123000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_2_0_e615;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3_0_e620,) = {
    if (w[484] == 0.0) {
        (158000000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_3_0_e620;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_4_0_e625,) = {
    if (w[484] == 0.0) {
        (204000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_4_0_e625;
        }
        if (active[0] & 0x41800) != 0 {
            let noise_metadata_schedule_5_0_e628: f64 = (1.0 - params.p33);
            w[160] = noise_metadata_schedule_5_0_e628;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_6_0_e631: f64 = (params.p4 + 273.15);
            w[3] = noise_metadata_schedule_6_0_e631;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_7_0_e632: f64 = ctx.temperature();
            let noise_metadata_schedule_7_0_e634: f64 = (noise_metadata_schedule_7_0_e632 + params.p0);
            w[5] = noise_metadata_schedule_7_0_e634;
        }
        if (active[0] & 0xff0003a) != 0 {
            let noise_metadata_schedule_9_0_e640: f64 = if params.p154 == 0.0 { 1.0 } else { 0.0 };
            w[485] = noise_metadata_schedule_9_0_e640;
        }
        if (active[0] & 0xff0003a) != 0 {
            let (noise_metadata_schedule_10_0_e644,) = {
    if (w[485] != 0.0) {
        (1e-12,)
    } else {
        (w[345],)
    }
};
            w[345] = noise_metadata_schedule_10_0_e644;
        }
        if (active[0] & 0xff0003a) != 0 {
            let (noise_metadata_schedule_11_0_e649,) = {
    if (w[485] == 0.0) {
        (params.p154,)
    } else {
        (w[345],)
    }
};
            w[345] = noise_metadata_schedule_11_0_e649;
        }
        if (active[0] & 0xff0003a) != 0 {
            let noise_metadata_schedule_12_0_e652: f64 = (w[345] * params.p1);
            w[346] = noise_metadata_schedule_12_0_e652;
        }
        if (active[0] & 0xff00000) != 0 {
            let noise_metadata_schedule_13_0_e655: f64 = (1.0 / w[346]);
            w[347] = noise_metadata_schedule_13_0_e655;
        }
        if (active[0] & 0xfffe7) != 0 {
            w[52] = 0.001;
        }
        if (active[0] & 0x9ffe7) != 0 {
            w[342] = 0.001;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_19_0_e673: f64 = (2.0 - params.p67);
            let noise_metadata_schedule_19_0_e674: f64 = (2.0_f64).powf(noise_metadata_schedule_19_0_e673);
            w[62] = noise_metadata_schedule_19_0_e674;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_20_0_e677: f64 = (1.0 / w[62]);
            w[63] = noise_metadata_schedule_20_0_e677;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_21_0_e681: f64 = (params.p115 * w[3]);
            let noise_metadata_schedule_21_0_e683: f64 = (noise_metadata_schedule_21_0_e681 * w[3]);
            let noise_metadata_schedule_21_0_e686: f64 = (w[3] + params.p116);
            let noise_metadata_schedule_21_0_e687: f64 = (noise_metadata_schedule_21_0_e683 / noise_metadata_schedule_21_0_e686);
            let noise_metadata_schedule_21_0_e688: f64 = (params.p114 + noise_metadata_schedule_21_0_e687);
            let noise_metadata_schedule_21_0_e690: f64 = (noise_metadata_schedule_21_0_e688 - 0.05);
            let noise_metadata_schedule_21_0_e692: f64 = (noise_metadata_schedule_21_0_e690 / 0.1);
            w[285] = noise_metadata_schedule_21_0_e692;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_22_0_e696: f64 = (params.p115 * w[3]);
            let noise_metadata_schedule_22_0_e698: f64 = (noise_metadata_schedule_22_0_e696 * w[3]);
            let noise_metadata_schedule_22_0_e701: f64 = (w[3] + params.p116);
            let noise_metadata_schedule_22_0_e702: f64 = (noise_metadata_schedule_22_0_e698 / noise_metadata_schedule_22_0_e701);
            let noise_metadata_schedule_22_0_e703: f64 = (params.p114 + noise_metadata_schedule_22_0_e702);
            let noise_metadata_schedule_22_0_e705: f64 = if noise_metadata_schedule_22_0_e703 < 0.05 { 1.0 } else { 0.0 };
            w[487] = noise_metadata_schedule_22_0_e705;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_23_0_e717,) = {
    if (w[487] != 0.0) {
        let noise_metadata_schedule_23_0_e711: f64 = (w[285]).exp();
        let noise_metadata_schedule_23_0_e712: f64 = (1.0 + noise_metadata_schedule_23_0_e711);
        let noise_metadata_schedule_23_0_e713: f64 = (noise_metadata_schedule_23_0_e712).ln();
        let noise_metadata_schedule_23_0_e714: f64 = (0.1 * noise_metadata_schedule_23_0_e713);
        let noise_metadata_schedule_23_0_e715: f64 = (0.05 + noise_metadata_schedule_23_0_e714);
        (noise_metadata_schedule_23_0_e715,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_23_0_e717;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_24_0_e741,) = {
    if (w[487] == 0.0) {
        let noise_metadata_schedule_24_0_e723: f64 = (params.p115 * w[3]);
        let noise_metadata_schedule_24_0_e725: f64 = (noise_metadata_schedule_24_0_e723 * w[3]);
        let noise_metadata_schedule_24_0_e728: f64 = (w[3] + params.p116);
        let noise_metadata_schedule_24_0_e729: f64 = (noise_metadata_schedule_24_0_e725 / noise_metadata_schedule_24_0_e728);
        let noise_metadata_schedule_24_0_e730: f64 = (params.p114 + noise_metadata_schedule_24_0_e729);
        let noise_metadata_schedule_24_0_e734: f64 = (-w[285]);
        let noise_metadata_schedule_24_0_e735: f64 = (noise_metadata_schedule_24_0_e734).exp();
        let noise_metadata_schedule_24_0_e736: f64 = (1.0 + noise_metadata_schedule_24_0_e735);
        let noise_metadata_schedule_24_0_e737: f64 = (noise_metadata_schedule_24_0_e736).ln();
        let noise_metadata_schedule_24_0_e738: f64 = (0.1 * noise_metadata_schedule_24_0_e737);
        let noise_metadata_schedule_24_0_e739: f64 = (noise_metadata_schedule_24_0_e730 + noise_metadata_schedule_24_0_e738);
        (noise_metadata_schedule_24_0_e739,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_24_0_e741;
        }
        if (active[0] & 0x18006) != 0 {
            w[71] = params.p114;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_26_0_e745: f64 = (1.0 / w[71]);
            w[72] = noise_metadata_schedule_26_0_e745;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_27_0_e748: f64 = (1.0 / params.p66);
            w[64] = noise_metadata_schedule_27_0_e748;
        }
        if (active[0] & 0x18002) != 0 {
            w[75] = params.p71;
        }
        if (active[0] & 0x18002) != 0 {
            w[76] = params.p72;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_30_0_e754: f64 = (2.0 - w[76]);
            let noise_metadata_schedule_30_0_e755: f64 = (2.0_f64).powf(noise_metadata_schedule_30_0_e754);
            w[79] = noise_metadata_schedule_30_0_e755;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_31_0_e758: f64 = (1.0 / w[79]);
            w[89] = noise_metadata_schedule_31_0_e758;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_32_0_e762: f64 = (params.p118 * w[3]);
            let noise_metadata_schedule_32_0_e764: f64 = (noise_metadata_schedule_32_0_e762 * w[3]);
            let noise_metadata_schedule_32_0_e767: f64 = (w[3] + params.p119);
            let noise_metadata_schedule_32_0_e768: f64 = (noise_metadata_schedule_32_0_e764 / noise_metadata_schedule_32_0_e767);
            let noise_metadata_schedule_32_0_e769: f64 = (params.p117 + noise_metadata_schedule_32_0_e768);
            let noise_metadata_schedule_32_0_e771: f64 = (noise_metadata_schedule_32_0_e769 - 0.05);
            let noise_metadata_schedule_32_0_e773: f64 = (noise_metadata_schedule_32_0_e771 / 0.1);
            w[285] = noise_metadata_schedule_32_0_e773;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_33_0_e777: f64 = (params.p118 * w[3]);
            let noise_metadata_schedule_33_0_e779: f64 = (noise_metadata_schedule_33_0_e777 * w[3]);
            let noise_metadata_schedule_33_0_e782: f64 = (w[3] + params.p119);
            let noise_metadata_schedule_33_0_e783: f64 = (noise_metadata_schedule_33_0_e779 / noise_metadata_schedule_33_0_e782);
            let noise_metadata_schedule_33_0_e784: f64 = (params.p117 + noise_metadata_schedule_33_0_e783);
            let noise_metadata_schedule_33_0_e786: f64 = if noise_metadata_schedule_33_0_e784 < 0.05 { 1.0 } else { 0.0 };
            w[488] = noise_metadata_schedule_33_0_e786;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_34_0_e798,) = {
    if (w[488] != 0.0) {
        let noise_metadata_schedule_34_0_e792: f64 = (w[285]).exp();
        let noise_metadata_schedule_34_0_e793: f64 = (1.0 + noise_metadata_schedule_34_0_e792);
        let noise_metadata_schedule_34_0_e794: f64 = (noise_metadata_schedule_34_0_e793).ln();
        let noise_metadata_schedule_34_0_e795: f64 = (0.1 * noise_metadata_schedule_34_0_e794);
        let noise_metadata_schedule_34_0_e796: f64 = (0.05 + noise_metadata_schedule_34_0_e795);
        (noise_metadata_schedule_34_0_e796,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_34_0_e798;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_35_0_e822,) = {
    if (w[488] == 0.0) {
        let noise_metadata_schedule_35_0_e804: f64 = (params.p118 * w[3]);
        let noise_metadata_schedule_35_0_e806: f64 = (noise_metadata_schedule_35_0_e804 * w[3]);
        let noise_metadata_schedule_35_0_e809: f64 = (w[3] + params.p119);
        let noise_metadata_schedule_35_0_e810: f64 = (noise_metadata_schedule_35_0_e806 / noise_metadata_schedule_35_0_e809);
        let noise_metadata_schedule_35_0_e811: f64 = (params.p117 + noise_metadata_schedule_35_0_e810);
        let noise_metadata_schedule_35_0_e815: f64 = (-w[285]);
        let noise_metadata_schedule_35_0_e816: f64 = (noise_metadata_schedule_35_0_e815).exp();
        let noise_metadata_schedule_35_0_e817: f64 = (1.0 + noise_metadata_schedule_35_0_e816);
        let noise_metadata_schedule_35_0_e818: f64 = (noise_metadata_schedule_35_0_e817).ln();
        let noise_metadata_schedule_35_0_e819: f64 = (0.1 * noise_metadata_schedule_35_0_e818);
        let noise_metadata_schedule_35_0_e820: f64 = (noise_metadata_schedule_35_0_e811 + noise_metadata_schedule_35_0_e819);
        (noise_metadata_schedule_35_0_e820,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_35_0_e822;
        }
        if (active[0] & 0x18002) != 0 {
            w[87] = params.p117;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_37_0_e826: f64 = (1.0 / w[87]);
            w[86] = noise_metadata_schedule_37_0_e826;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_38_0_e829: f64 = (1.0 / w[75]);
            w[66] = noise_metadata_schedule_38_0_e829;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_39_0_e833: f64 = (1.0 / params.p83);
            let noise_metadata_schedule_39_0_e834: f64 = (1.0 - noise_metadata_schedule_39_0_e833);
            w[349] = noise_metadata_schedule_39_0_e834;
        }
        if (active[0] & 0x44) != 0 {
            w[161] = 0.0;
        }
        if (active[0] & 0x140) != 0 {
            w[162] = 0.0;
        }
        if (active[0] & 0x6000) != 0 {
            w[179] = 0.0;
        }
        if (active[0] & 0x86000) != 0 {
            w[178] = 1.0;
        }
        if (active[0] & 0x2) != 0 {
            w[210] = 0.0;
        }
        if (active[0] & 0x2) != 0 {
            w[212] = 0.0;
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
        if (active[0] & 0xfffffff) != 0 {
            w[218] = (ctx.node_voltage(self.nodes[4]) - 0.0);
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_54_0_e851: f64 = if w[218] < 0.0 { 1.0 } else { 0.0 };
            w[489] = noise_metadata_schedule_54_0_e851;
        }
        if (active[0] & 0xfffffff) != 0 {
            let (noise_metadata_schedule_55_0_e859,) = {
    if (w[489] != 0.0) {
        let noise_metadata_schedule_55_0_e855: f64 = (1.0 - w[218]);
        let noise_metadata_schedule_55_0_e856: f64 = (noise_metadata_schedule_55_0_e855).ln();
        let noise_metadata_schedule_55_0_e857: f64 = (-noise_metadata_schedule_55_0_e856);
        (noise_metadata_schedule_55_0_e857,)
    } else {
        (w[218],)
    }
};
            w[218] = noise_metadata_schedule_55_0_e859;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_56_0_e862: f64 = if w[218] < params.p125 { 1.0 } else { 0.0 };
            w[490] = noise_metadata_schedule_56_0_e862;
        }
        if (active[0] & 0xfffffff) != 0 {
            let (noise_metadata_schedule_57_0_e866,) = {
    if (w[490] != 0.0) {
        (w[218],)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_57_0_e866;
        }
        if (active[0] & 0xfffffff) != 0 {
            let (noise_metadata_schedule_58_0_e878,) = {
    if (w[490] == 0.0) {
        let noise_metadata_schedule_58_0_e873: f64 = (w[218] - params.p125);
        let noise_metadata_schedule_58_0_e874: f64 = (1.0 + noise_metadata_schedule_58_0_e873);
        let noise_metadata_schedule_58_0_e875: f64 = (noise_metadata_schedule_58_0_e874).ln();
        let noise_metadata_schedule_58_0_e876: f64 = (params.p125 + noise_metadata_schedule_58_0_e875);
        (noise_metadata_schedule_58_0_e876,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_58_0_e878;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_59_0_e881: f64 = (w[5] + w[11]);
            w[2] = noise_metadata_schedule_59_0_e881;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_60_0_e884: f64 = (w[2] / w[3]);
            w[4] = noise_metadata_schedule_60_0_e884;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_61_0_e887: f64 = (8.617086918058125e-5 * w[2]);
            w[6] = noise_metadata_schedule_61_0_e887;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_62_0_e890: f64 = (8.617086918058125e-5 * w[3]);
            w[7] = noise_metadata_schedule_62_0_e890;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_63_0_e893: f64 = (1.0 / w[6]);
            w[8] = noise_metadata_schedule_63_0_e893;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_64_0_e896: f64 = (1.0 / w[7]);
            w[9] = noise_metadata_schedule_64_0_e896;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_65_0_e899: f64 = (w[8] - w[9]);
            w[10] = noise_metadata_schedule_65_0_e899;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_66_0_e902: f64 = (w[2] - w[3]);
            w[12] = noise_metadata_schedule_66_0_e902;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_67_0_e904: f64 = (w[4]).ln();
            w[280] = noise_metadata_schedule_67_0_e904;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_68_0_e908: f64 = (params.p115 * w[2]);
            let noise_metadata_schedule_68_0_e910: f64 = (noise_metadata_schedule_68_0_e908 * w[2]);
            let noise_metadata_schedule_68_0_e913: f64 = (w[2] + params.p116);
            let noise_metadata_schedule_68_0_e914: f64 = (noise_metadata_schedule_68_0_e910 / noise_metadata_schedule_68_0_e913);
            let noise_metadata_schedule_68_0_e915: f64 = (w[74] - noise_metadata_schedule_68_0_e914);
            let noise_metadata_schedule_68_0_e917: f64 = (noise_metadata_schedule_68_0_e915 - 0.05);
            let noise_metadata_schedule_68_0_e919: f64 = (noise_metadata_schedule_68_0_e917 / 0.1);
            w[285] = noise_metadata_schedule_68_0_e919;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_69_0_e923: f64 = (params.p115 * w[2]);
            let noise_metadata_schedule_69_0_e925: f64 = (noise_metadata_schedule_69_0_e923 * w[2]);
            let noise_metadata_schedule_69_0_e928: f64 = (w[2] + params.p116);
            let noise_metadata_schedule_69_0_e929: f64 = (noise_metadata_schedule_69_0_e925 / noise_metadata_schedule_69_0_e928);
            let noise_metadata_schedule_69_0_e930: f64 = (w[74] - noise_metadata_schedule_69_0_e929);
            let noise_metadata_schedule_69_0_e932: f64 = if noise_metadata_schedule_69_0_e930 < 0.05 { 1.0 } else { 0.0 };
            w[491] = noise_metadata_schedule_69_0_e932;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_70_0_e944,) = {
    if (w[491] != 0.0) {
        let noise_metadata_schedule_70_0_e938: f64 = (w[285]).exp();
        let noise_metadata_schedule_70_0_e939: f64 = (1.0 + noise_metadata_schedule_70_0_e938);
        let noise_metadata_schedule_70_0_e940: f64 = (noise_metadata_schedule_70_0_e939).ln();
        let noise_metadata_schedule_70_0_e941: f64 = (0.1 * noise_metadata_schedule_70_0_e940);
        let noise_metadata_schedule_70_0_e942: f64 = (0.05 + noise_metadata_schedule_70_0_e941);
        (noise_metadata_schedule_70_0_e942,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_70_0_e944;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_71_0_e968,) = {
    if (w[491] == 0.0) {
        let noise_metadata_schedule_71_0_e950: f64 = (params.p115 * w[2]);
        let noise_metadata_schedule_71_0_e952: f64 = (noise_metadata_schedule_71_0_e950 * w[2]);
        let noise_metadata_schedule_71_0_e955: f64 = (w[2] + params.p116);
        let noise_metadata_schedule_71_0_e956: f64 = (noise_metadata_schedule_71_0_e952 / noise_metadata_schedule_71_0_e955);
        let noise_metadata_schedule_71_0_e957: f64 = (w[74] - noise_metadata_schedule_71_0_e956);
        let noise_metadata_schedule_71_0_e961: f64 = (-w[285]);
        let noise_metadata_schedule_71_0_e962: f64 = (noise_metadata_schedule_71_0_e961).exp();
        let noise_metadata_schedule_71_0_e963: f64 = (1.0 + noise_metadata_schedule_71_0_e962);
        let noise_metadata_schedule_71_0_e964: f64 = (noise_metadata_schedule_71_0_e963).ln();
        let noise_metadata_schedule_71_0_e965: f64 = (0.1 * noise_metadata_schedule_71_0_e964);
        let noise_metadata_schedule_71_0_e966: f64 = (noise_metadata_schedule_71_0_e957 + noise_metadata_schedule_71_0_e965);
        (noise_metadata_schedule_71_0_e966,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_71_0_e968;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_72_0_e972: f64 = (params.p118 * w[2]);
            let noise_metadata_schedule_72_0_e974: f64 = (noise_metadata_schedule_72_0_e972 * w[2]);
            let noise_metadata_schedule_72_0_e977: f64 = (w[2] + params.p119);
            let noise_metadata_schedule_72_0_e978: f64 = (noise_metadata_schedule_72_0_e974 / noise_metadata_schedule_72_0_e977);
            let noise_metadata_schedule_72_0_e979: f64 = (w[88] - noise_metadata_schedule_72_0_e978);
            let noise_metadata_schedule_72_0_e981: f64 = (noise_metadata_schedule_72_0_e979 - 0.05);
            let noise_metadata_schedule_72_0_e983: f64 = (noise_metadata_schedule_72_0_e981 / 0.1);
            w[285] = noise_metadata_schedule_72_0_e983;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_73_0_e987: f64 = (params.p118 * w[2]);
            let noise_metadata_schedule_73_0_e989: f64 = (noise_metadata_schedule_73_0_e987 * w[2]);
            let noise_metadata_schedule_73_0_e992: f64 = (w[2] + params.p119);
            let noise_metadata_schedule_73_0_e993: f64 = (noise_metadata_schedule_73_0_e989 / noise_metadata_schedule_73_0_e992);
            let noise_metadata_schedule_73_0_e994: f64 = (w[88] - noise_metadata_schedule_73_0_e993);
            let noise_metadata_schedule_73_0_e996: f64 = if noise_metadata_schedule_73_0_e994 < 0.05 { 1.0 } else { 0.0 };
            w[492] = noise_metadata_schedule_73_0_e996;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_74_0_e1008,) = {
    if (w[492] != 0.0) {
        let noise_metadata_schedule_74_0_e1002: f64 = (w[285]).exp();
        let noise_metadata_schedule_74_0_e1003: f64 = (1.0 + noise_metadata_schedule_74_0_e1002);
        let noise_metadata_schedule_74_0_e1004: f64 = (noise_metadata_schedule_74_0_e1003).ln();
        let noise_metadata_schedule_74_0_e1005: f64 = (0.1 * noise_metadata_schedule_74_0_e1004);
        let noise_metadata_schedule_74_0_e1006: f64 = (0.05 + noise_metadata_schedule_74_0_e1005);
        (noise_metadata_schedule_74_0_e1006,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_74_0_e1008;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_75_0_e1032,) = {
    if (w[492] == 0.0) {
        let noise_metadata_schedule_75_0_e1014: f64 = (params.p118 * w[2]);
        let noise_metadata_schedule_75_0_e1016: f64 = (noise_metadata_schedule_75_0_e1014 * w[2]);
        let noise_metadata_schedule_75_0_e1019: f64 = (w[2] + params.p119);
        let noise_metadata_schedule_75_0_e1020: f64 = (noise_metadata_schedule_75_0_e1016 / noise_metadata_schedule_75_0_e1019);
        let noise_metadata_schedule_75_0_e1021: f64 = (w[88] - noise_metadata_schedule_75_0_e1020);
        let noise_metadata_schedule_75_0_e1025: f64 = (-w[285]);
        let noise_metadata_schedule_75_0_e1026: f64 = (noise_metadata_schedule_75_0_e1025).exp();
        let noise_metadata_schedule_75_0_e1027: f64 = (1.0 + noise_metadata_schedule_75_0_e1026);
        let noise_metadata_schedule_75_0_e1028: f64 = (noise_metadata_schedule_75_0_e1027).ln();
        let noise_metadata_schedule_75_0_e1029: f64 = (0.1 * noise_metadata_schedule_75_0_e1028);
        let noise_metadata_schedule_75_0_e1030: f64 = (noise_metadata_schedule_75_0_e1021 + noise_metadata_schedule_75_0_e1029);
        (noise_metadata_schedule_75_0_e1030,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_75_0_e1032;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_76_0_e1034: f64 = (-3.0);
            let noise_metadata_schedule_76_0_e1036: f64 = (noise_metadata_schedule_76_0_e1034 * w[6]);
            let noise_metadata_schedule_76_0_e1038: f64 = (noise_metadata_schedule_76_0_e1036 * w[280]);
            let noise_metadata_schedule_76_0_e1041: f64 = (params.p66 * w[4]);
            let noise_metadata_schedule_76_0_e1042: f64 = (noise_metadata_schedule_76_0_e1038 + noise_metadata_schedule_76_0_e1041);
            let noise_metadata_schedule_76_0_e1045: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_76_0_e1047: f64 = (noise_metadata_schedule_76_0_e1045 * params.p105);
            let noise_metadata_schedule_76_0_e1048: f64 = (noise_metadata_schedule_76_0_e1042 + noise_metadata_schedule_76_0_e1047);
            w[13] = noise_metadata_schedule_76_0_e1048;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_77_0_e1051: f64 = (0.05 - w[13]);
            let noise_metadata_schedule_77_0_e1053: f64 = (noise_metadata_schedule_77_0_e1051 / w[6]);
            w[285] = noise_metadata_schedule_77_0_e1053;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_78_0_e1056: f64 = if 0.05 < w[13] { 1.0 } else { 0.0 };
            w[493] = noise_metadata_schedule_78_0_e1056;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_79_0_e1068,) = {
    if (w[493] != 0.0) {
        let noise_metadata_schedule_79_0_e1062: f64 = (w[285]).exp();
        let noise_metadata_schedule_79_0_e1063: f64 = (1.0 + noise_metadata_schedule_79_0_e1062);
        let noise_metadata_schedule_79_0_e1064: f64 = (noise_metadata_schedule_79_0_e1063).ln();
        let noise_metadata_schedule_79_0_e1065: f64 = (w[6] * noise_metadata_schedule_79_0_e1064);
        let noise_metadata_schedule_79_0_e1066: f64 = (w[13] + noise_metadata_schedule_79_0_e1065);
        (noise_metadata_schedule_79_0_e1066,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_79_0_e1068;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_80_0_e1082,) = {
    if (w[493] == 0.0) {
        let noise_metadata_schedule_80_0_e1075: f64 = (-w[285]);
        let noise_metadata_schedule_80_0_e1076: f64 = (noise_metadata_schedule_80_0_e1075).exp();
        let noise_metadata_schedule_80_0_e1077: f64 = (1.0 + noise_metadata_schedule_80_0_e1076);
        let noise_metadata_schedule_80_0_e1078: f64 = (noise_metadata_schedule_80_0_e1077).ln();
        let noise_metadata_schedule_80_0_e1079: f64 = (w[6] * noise_metadata_schedule_80_0_e1078);
        let noise_metadata_schedule_80_0_e1080: f64 = (0.05 + noise_metadata_schedule_80_0_e1079);
        (noise_metadata_schedule_80_0_e1080,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_80_0_e1082;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_81_0_e1084: f64 = (-3.0);
            let noise_metadata_schedule_81_0_e1086: f64 = (noise_metadata_schedule_81_0_e1084 * w[6]);
            let noise_metadata_schedule_81_0_e1088: f64 = (noise_metadata_schedule_81_0_e1086 * w[280]);
            let noise_metadata_schedule_81_0_e1091: f64 = (params.p64 * w[4]);
            let noise_metadata_schedule_81_0_e1092: f64 = (noise_metadata_schedule_81_0_e1088 + noise_metadata_schedule_81_0_e1091);
            let noise_metadata_schedule_81_0_e1095: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_81_0_e1097: f64 = (noise_metadata_schedule_81_0_e1095 * params.p110);
            let noise_metadata_schedule_81_0_e1098: f64 = (noise_metadata_schedule_81_0_e1092 + noise_metadata_schedule_81_0_e1097);
            w[15] = noise_metadata_schedule_81_0_e1098;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_82_0_e1101: f64 = (0.05 - w[15]);
            let noise_metadata_schedule_82_0_e1103: f64 = (noise_metadata_schedule_82_0_e1101 / w[6]);
            w[285] = noise_metadata_schedule_82_0_e1103;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_83_0_e1106: f64 = if 0.05 < w[15] { 1.0 } else { 0.0 };
            w[494] = noise_metadata_schedule_83_0_e1106;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_84_0_e1118,) = {
    if (w[494] != 0.0) {
        let noise_metadata_schedule_84_0_e1112: f64 = (w[285]).exp();
        let noise_metadata_schedule_84_0_e1113: f64 = (1.0 + noise_metadata_schedule_84_0_e1112);
        let noise_metadata_schedule_84_0_e1114: f64 = (noise_metadata_schedule_84_0_e1113).ln();
        let noise_metadata_schedule_84_0_e1115: f64 = (w[6] * noise_metadata_schedule_84_0_e1114);
        let noise_metadata_schedule_84_0_e1116: f64 = (w[15] + noise_metadata_schedule_84_0_e1115);
        (noise_metadata_schedule_84_0_e1116,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_84_0_e1118;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_85_0_e1132,) = {
    if (w[494] == 0.0) {
        let noise_metadata_schedule_85_0_e1125: f64 = (-w[285]);
        let noise_metadata_schedule_85_0_e1126: f64 = (noise_metadata_schedule_85_0_e1125).exp();
        let noise_metadata_schedule_85_0_e1127: f64 = (1.0 + noise_metadata_schedule_85_0_e1126);
        let noise_metadata_schedule_85_0_e1128: f64 = (noise_metadata_schedule_85_0_e1127).ln();
        let noise_metadata_schedule_85_0_e1129: f64 = (w[6] * noise_metadata_schedule_85_0_e1128);
        let noise_metadata_schedule_85_0_e1130: f64 = (0.05 + noise_metadata_schedule_85_0_e1129);
        (noise_metadata_schedule_85_0_e1130,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_85_0_e1132;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_91_0_e1184: f64 = (-3.0);
            let noise_metadata_schedule_91_0_e1186: f64 = (noise_metadata_schedule_91_0_e1184 * w[6]);
            let noise_metadata_schedule_91_0_e1188: f64 = (noise_metadata_schedule_91_0_e1186 * w[280]);
            let noise_metadata_schedule_91_0_e1191: f64 = (params.p71 * w[4]);
            let noise_metadata_schedule_91_0_e1192: f64 = (noise_metadata_schedule_91_0_e1188 + noise_metadata_schedule_91_0_e1191);
            let noise_metadata_schedule_91_0_e1195: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_91_0_e1197: f64 = (noise_metadata_schedule_91_0_e1195 * params.p110);
            let noise_metadata_schedule_91_0_e1198: f64 = (noise_metadata_schedule_91_0_e1192 + noise_metadata_schedule_91_0_e1197);
            w[18] = noise_metadata_schedule_91_0_e1198;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_92_0_e1201: f64 = (0.05 - w[18]);
            let noise_metadata_schedule_92_0_e1203: f64 = (noise_metadata_schedule_92_0_e1201 / w[6]);
            w[285] = noise_metadata_schedule_92_0_e1203;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_93_0_e1206: f64 = if 0.05 < w[18] { 1.0 } else { 0.0 };
            w[496] = noise_metadata_schedule_93_0_e1206;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_94_0_e1218,) = {
    if (w[496] != 0.0) {
        let noise_metadata_schedule_94_0_e1212: f64 = (w[285]).exp();
        let noise_metadata_schedule_94_0_e1213: f64 = (1.0 + noise_metadata_schedule_94_0_e1212);
        let noise_metadata_schedule_94_0_e1214: f64 = (noise_metadata_schedule_94_0_e1213).ln();
        let noise_metadata_schedule_94_0_e1215: f64 = (w[6] * noise_metadata_schedule_94_0_e1214);
        let noise_metadata_schedule_94_0_e1216: f64 = (w[18] + noise_metadata_schedule_94_0_e1215);
        (noise_metadata_schedule_94_0_e1216,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_94_0_e1218;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_95_0_e1232,) = {
    if (w[496] == 0.0) {
        let noise_metadata_schedule_95_0_e1225: f64 = (-w[285]);
        let noise_metadata_schedule_95_0_e1226: f64 = (noise_metadata_schedule_95_0_e1225).exp();
        let noise_metadata_schedule_95_0_e1227: f64 = (1.0 + noise_metadata_schedule_95_0_e1226);
        let noise_metadata_schedule_95_0_e1228: f64 = (noise_metadata_schedule_95_0_e1227).ln();
        let noise_metadata_schedule_95_0_e1229: f64 = (w[6] * noise_metadata_schedule_95_0_e1228);
        let noise_metadata_schedule_95_0_e1230: f64 = (0.05 + noise_metadata_schedule_95_0_e1229);
        (noise_metadata_schedule_95_0_e1230,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_95_0_e1232;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_96_0_e1234: f64 = (-3.0);
            let noise_metadata_schedule_96_0_e1236: f64 = (noise_metadata_schedule_96_0_e1234 * w[6]);
            let noise_metadata_schedule_96_0_e1238: f64 = (noise_metadata_schedule_96_0_e1236 * w[280]);
            let noise_metadata_schedule_96_0_e1241: f64 = (w[75] * w[4]);
            let noise_metadata_schedule_96_0_e1242: f64 = (noise_metadata_schedule_96_0_e1238 + noise_metadata_schedule_96_0_e1241);
            let noise_metadata_schedule_96_0_e1245: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_96_0_e1247: f64 = (noise_metadata_schedule_96_0_e1245 * params.p110);
            let noise_metadata_schedule_96_0_e1248: f64 = (noise_metadata_schedule_96_0_e1242 + noise_metadata_schedule_96_0_e1247);
            w[20] = noise_metadata_schedule_96_0_e1248;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_97_0_e1251: f64 = (0.05 - w[20]);
            let noise_metadata_schedule_97_0_e1253: f64 = (noise_metadata_schedule_97_0_e1251 / w[6]);
            w[285] = noise_metadata_schedule_97_0_e1253;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_98_0_e1256: f64 = if 0.05 < w[20] { 1.0 } else { 0.0 };
            w[497] = noise_metadata_schedule_98_0_e1256;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_99_0_e1268,) = {
    if (w[497] != 0.0) {
        let noise_metadata_schedule_99_0_e1262: f64 = (w[285]).exp();
        let noise_metadata_schedule_99_0_e1263: f64 = (1.0 + noise_metadata_schedule_99_0_e1262);
        let noise_metadata_schedule_99_0_e1264: f64 = (noise_metadata_schedule_99_0_e1263).ln();
        let noise_metadata_schedule_99_0_e1265: f64 = (w[6] * noise_metadata_schedule_99_0_e1264);
        let noise_metadata_schedule_99_0_e1266: f64 = (w[20] + noise_metadata_schedule_99_0_e1265);
        (noise_metadata_schedule_99_0_e1266,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_99_0_e1268;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_100_0_e1282,) = {
    if (w[497] == 0.0) {
        let noise_metadata_schedule_100_0_e1275: f64 = (-w[285]);
        let noise_metadata_schedule_100_0_e1276: f64 = (noise_metadata_schedule_100_0_e1275).exp();
        let noise_metadata_schedule_100_0_e1277: f64 = (1.0 + noise_metadata_schedule_100_0_e1276);
        let noise_metadata_schedule_100_0_e1278: f64 = (noise_metadata_schedule_100_0_e1277).ln();
        let noise_metadata_schedule_100_0_e1279: f64 = (w[6] * noise_metadata_schedule_100_0_e1278);
        let noise_metadata_schedule_100_0_e1280: f64 = (0.05 + noise_metadata_schedule_100_0_e1279);
        (noise_metadata_schedule_100_0_e1280,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_100_0_e1282;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_101_0_e1284: f64 = (-3.0);
            let noise_metadata_schedule_101_0_e1286: f64 = (noise_metadata_schedule_101_0_e1284 * w[6]);
            let noise_metadata_schedule_101_0_e1288: f64 = (noise_metadata_schedule_101_0_e1286 * w[280]);
            let noise_metadata_schedule_101_0_e1291: f64 = (params.p27 * w[4]);
            let noise_metadata_schedule_101_0_e1292: f64 = (noise_metadata_schedule_101_0_e1288 + noise_metadata_schedule_101_0_e1291);
            let noise_metadata_schedule_101_0_e1295: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_101_0_e1297: f64 = (noise_metadata_schedule_101_0_e1295 * params.p109);
            let noise_metadata_schedule_101_0_e1298: f64 = (noise_metadata_schedule_101_0_e1292 + noise_metadata_schedule_101_0_e1297);
            w[56] = noise_metadata_schedule_101_0_e1298;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_102_0_e1301: f64 = (0.05 - w[56]);
            let noise_metadata_schedule_102_0_e1303: f64 = (noise_metadata_schedule_102_0_e1301 / w[6]);
            w[285] = noise_metadata_schedule_102_0_e1303;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_103_0_e1306: f64 = if 0.05 < w[56] { 1.0 } else { 0.0 };
            w[498] = noise_metadata_schedule_103_0_e1306;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_104_0_e1318,) = {
    if (w[498] != 0.0) {
        let noise_metadata_schedule_104_0_e1312: f64 = (w[285]).exp();
        let noise_metadata_schedule_104_0_e1313: f64 = (1.0 + noise_metadata_schedule_104_0_e1312);
        let noise_metadata_schedule_104_0_e1314: f64 = (noise_metadata_schedule_104_0_e1313).ln();
        let noise_metadata_schedule_104_0_e1315: f64 = (w[6] * noise_metadata_schedule_104_0_e1314);
        let noise_metadata_schedule_104_0_e1316: f64 = (w[56] + noise_metadata_schedule_104_0_e1315);
        (noise_metadata_schedule_104_0_e1316,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_104_0_e1318;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_105_0_e1332,) = {
    if (w[498] == 0.0) {
        let noise_metadata_schedule_105_0_e1325: f64 = (-w[285]);
        let noise_metadata_schedule_105_0_e1326: f64 = (noise_metadata_schedule_105_0_e1325).exp();
        let noise_metadata_schedule_105_0_e1327: f64 = (1.0 + noise_metadata_schedule_105_0_e1326);
        let noise_metadata_schedule_105_0_e1328: f64 = (noise_metadata_schedule_105_0_e1327).ln();
        let noise_metadata_schedule_105_0_e1329: f64 = (w[6] * noise_metadata_schedule_105_0_e1328);
        let noise_metadata_schedule_105_0_e1330: f64 = (0.05 + noise_metadata_schedule_105_0_e1329);
        (noise_metadata_schedule_105_0_e1330,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_105_0_e1332;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_106_0_e1334: f64 = (-3.0);
            let noise_metadata_schedule_106_0_e1336: f64 = (noise_metadata_schedule_106_0_e1334 * w[6]);
            let noise_metadata_schedule_106_0_e1338: f64 = (noise_metadata_schedule_106_0_e1336 * w[280]);
            let noise_metadata_schedule_106_0_e1341: f64 = (params.p138 * w[4]);
            let noise_metadata_schedule_106_0_e1342: f64 = (noise_metadata_schedule_106_0_e1338 + noise_metadata_schedule_106_0_e1341);
            let noise_metadata_schedule_106_0_e1345: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_106_0_e1347: f64 = (noise_metadata_schedule_106_0_e1345 * params.p140);
            let noise_metadata_schedule_106_0_e1348: f64 = (noise_metadata_schedule_106_0_e1342 + noise_metadata_schedule_106_0_e1347);
            w[104] = noise_metadata_schedule_106_0_e1348;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_107_0_e1351: f64 = (0.05 - w[104]);
            let noise_metadata_schedule_107_0_e1353: f64 = (noise_metadata_schedule_107_0_e1351 / w[6]);
            w[285] = noise_metadata_schedule_107_0_e1353;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_111_0_e1385: f64 = (1.0 / w[14]);
            w[65] = noise_metadata_schedule_111_0_e1385;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_112_0_e1388: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_112_0_e1388;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_113_0_e1391: f64 = (params.p66 * w[65]);
            let noise_metadata_schedule_113_0_e1393: f64 = (noise_metadata_schedule_113_0_e1391).powf(params.p67);
            w[73] = noise_metadata_schedule_113_0_e1393;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_114_0_e1396: f64 = (w[75] * w[67]);
            let noise_metadata_schedule_114_0_e1398: f64 = (noise_metadata_schedule_114_0_e1396).powf(w[76]);
            w[90] = noise_metadata_schedule_114_0_e1398;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_117_0_e1411: f64 = (1.0 - params.p75);
            let noise_metadata_schedule_117_0_e1414: f64 = (params.p71 / w[17]);
            let noise_metadata_schedule_117_0_e1416: f64 = (noise_metadata_schedule_117_0_e1414).powf(params.p72);
            let noise_metadata_schedule_117_0_e1417: f64 = (noise_metadata_schedule_117_0_e1411 * noise_metadata_schedule_117_0_e1416);
            let noise_metadata_schedule_117_0_e1419: f64 = (noise_metadata_schedule_117_0_e1417 + params.p75);
            w[26] = noise_metadata_schedule_117_0_e1419;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_118_0_e1422: f64 = (1.0 / w[26]);
            w[27] = noise_metadata_schedule_118_0_e1422;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_120_0_e1428: f64 = (params.p75 * w[27]);
            w[25] = noise_metadata_schedule_120_0_e1428;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_121_0_e1432: f64 = (w[280] * params.p97);
            let noise_metadata_schedule_121_0_e1433: f64 = (noise_metadata_schedule_121_0_e1432).exp();
            let noise_metadata_schedule_121_0_e1434: f64 = (params.p54 * noise_metadata_schedule_121_0_e1433);
            w[28] = noise_metadata_schedule_121_0_e1434;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_122_0_e1437: f64 = if w[28] < w[346] { 1.0 } else { 0.0 };
            w[500] = noise_metadata_schedule_122_0_e1437;
        }
        if (active[0] & 0xa) != 0 {
            let (noise_metadata_schedule_123_0_e1441,) = {
    if (w[500] != 0.0) {
        (w[346],)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_123_0_e1441;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_124_0_e1446: f64 = (params.p98 - params.p96);
            let noise_metadata_schedule_124_0_e1447: f64 = (w[280] * noise_metadata_schedule_124_0_e1446);
            let noise_metadata_schedule_124_0_e1448: f64 = (noise_metadata_schedule_124_0_e1447).exp();
            let noise_metadata_schedule_124_0_e1449: f64 = (params.p56 * noise_metadata_schedule_124_0_e1448);
            w[29] = noise_metadata_schedule_124_0_e1449;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_125_0_e1453: f64 = (w[280] * params.p101);
            let noise_metadata_schedule_125_0_e1454: f64 = (noise_metadata_schedule_125_0_e1453).exp();
            let noise_metadata_schedule_125_0_e1455: f64 = (params.p55 * noise_metadata_schedule_125_0_e1454);
            w[30] = noise_metadata_schedule_125_0_e1455;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_126_0_e1458: f64 = if w[30] < w[346] { 1.0 } else { 0.0 };
            w[501] = noise_metadata_schedule_126_0_e1458;
        }
        if (active[0] & 0x12) != 0 {
            let (noise_metadata_schedule_127_0_e1462,) = {
    if (w[501] != 0.0) {
        (w[346],)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_127_0_e1462;
        }
        if (active[0] & 0xa99fe00) != 0 {
            let noise_metadata_schedule_128_0_e1466: f64 = (w[280] * params.p102);
            let noise_metadata_schedule_128_0_e1467: f64 = (noise_metadata_schedule_128_0_e1466).exp();
            let noise_metadata_schedule_128_0_e1468: f64 = (params.p57 * noise_metadata_schedule_128_0_e1467);
            w[32] = noise_metadata_schedule_128_0_e1468;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_129_0_e1472: f64 = (w[280] * params.p104);
            let noise_metadata_schedule_129_0_e1473: f64 = (noise_metadata_schedule_129_0_e1472).exp();
            let noise_metadata_schedule_129_0_e1474: f64 = (params.p58 * noise_metadata_schedule_129_0_e1473);
            w[33] = noise_metadata_schedule_129_0_e1474;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_130_0_e1478: f64 = (w[280] * params.p104);
            let noise_metadata_schedule_130_0_e1479: f64 = (noise_metadata_schedule_130_0_e1478).exp();
            let noise_metadata_schedule_130_0_e1480: f64 = (params.p59 * noise_metadata_schedule_130_0_e1479);
            w[34] = noise_metadata_schedule_130_0_e1480;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_131_0_e1484: f64 = (w[280] * params.p99);
            let noise_metadata_schedule_131_0_e1485: f64 = (noise_metadata_schedule_131_0_e1484).exp();
            let noise_metadata_schedule_131_0_e1486: f64 = (params.p60 * noise_metadata_schedule_131_0_e1485);
            w[31] = noise_metadata_schedule_131_0_e1486;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_132_0_e1489: f64 = if params.p122 != 0.0 { 1.0 } else { 0.0 };
            w[502] = noise_metadata_schedule_132_0_e1489;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_133_0_e1499,) = {
    if (w[502] != 0.0) {
        let noise_metadata_schedule_133_0_e1495: f64 = (w[12] * params.p122);
        let noise_metadata_schedule_133_0_e1496: f64 = (1.0 + noise_metadata_schedule_133_0_e1495);
        let noise_metadata_schedule_133_0_e1497: f64 = (params.p10 * noise_metadata_schedule_133_0_e1496);
        (noise_metadata_schedule_133_0_e1497,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_133_0_e1499;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_134_0_e1507,) = {
    if (w[502] != 0.0) {
        let noise_metadata_schedule_134_0_e1503: f64 = (w[50] - 1.0);
        let noise_metadata_schedule_134_0_e1505: f64 = (noise_metadata_schedule_134_0_e1503 / w[52]);
        (noise_metadata_schedule_134_0_e1505,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_134_0_e1507;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_135_0_e1510: f64 = if w[50] < 1.0 { 1.0 } else { 0.0 };
            w[503] = noise_metadata_schedule_135_0_e1510;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_136_0_e1524,) = {
    if ((w[502] != 0.0) && (w[503] != 0.0)) {
        let noise_metadata_schedule_136_0_e1518: f64 = (w[285]).exp();
        let noise_metadata_schedule_136_0_e1519: f64 = (1.0 + noise_metadata_schedule_136_0_e1518);
        let noise_metadata_schedule_136_0_e1520: f64 = (noise_metadata_schedule_136_0_e1519).ln();
        let noise_metadata_schedule_136_0_e1521: f64 = (w[52] * noise_metadata_schedule_136_0_e1520);
        let noise_metadata_schedule_136_0_e1522: f64 = (1.0 + noise_metadata_schedule_136_0_e1521);
        (noise_metadata_schedule_136_0_e1522,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_136_0_e1524;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_137_0_e1540,) = {
    if ((w[502] != 0.0) && (w[503] == 0.0)) {
        let noise_metadata_schedule_137_0_e1533: f64 = (-w[285]);
        let noise_metadata_schedule_137_0_e1534: f64 = (noise_metadata_schedule_137_0_e1533).exp();
        let noise_metadata_schedule_137_0_e1535: f64 = (1.0 + noise_metadata_schedule_137_0_e1534);
        let noise_metadata_schedule_137_0_e1536: f64 = (noise_metadata_schedule_137_0_e1535).ln();
        let noise_metadata_schedule_137_0_e1537: f64 = (w[52] * noise_metadata_schedule_137_0_e1536);
        let noise_metadata_schedule_137_0_e1538: f64 = (w[50] + noise_metadata_schedule_137_0_e1537);
        (noise_metadata_schedule_137_0_e1538,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_137_0_e1540;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_138_0_e1548,) = {
    if (w[502] != 0.0) {
        let noise_metadata_schedule_138_0_e1545: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_138_0_e1546: f64 = (w[50] - noise_metadata_schedule_138_0_e1545);
        (noise_metadata_schedule_138_0_e1546,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_138_0_e1548;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_139_0_e1553,) = {
    if (w[502] == 0.0) {
        (params.p10,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_139_0_e1553;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_140_0_e1556: f64 = if params.p123 != 0.0 { 1.0 } else { 0.0 };
            w[504] = noise_metadata_schedule_140_0_e1556;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_141_0_e1566,) = {
    if (w[504] != 0.0) {
        let noise_metadata_schedule_141_0_e1562: f64 = (w[12] * params.p123);
        let noise_metadata_schedule_141_0_e1563: f64 = (1.0 + noise_metadata_schedule_141_0_e1562);
        let noise_metadata_schedule_141_0_e1564: f64 = (params.p11 * noise_metadata_schedule_141_0_e1563);
        (noise_metadata_schedule_141_0_e1564,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_141_0_e1566;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_142_0_e1574,) = {
    if (w[504] != 0.0) {
        let noise_metadata_schedule_142_0_e1570: f64 = (w[51] - 1.0);
        let noise_metadata_schedule_142_0_e1572: f64 = (noise_metadata_schedule_142_0_e1570 / w[52]);
        (noise_metadata_schedule_142_0_e1572,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_142_0_e1574;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_143_0_e1577: f64 = if w[51] < 1.0 { 1.0 } else { 0.0 };
            w[505] = noise_metadata_schedule_143_0_e1577;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_144_0_e1591,) = {
    if ((w[504] != 0.0) && (w[505] != 0.0)) {
        let noise_metadata_schedule_144_0_e1585: f64 = (w[285]).exp();
        let noise_metadata_schedule_144_0_e1586: f64 = (1.0 + noise_metadata_schedule_144_0_e1585);
        let noise_metadata_schedule_144_0_e1587: f64 = (noise_metadata_schedule_144_0_e1586).ln();
        let noise_metadata_schedule_144_0_e1588: f64 = (w[52] * noise_metadata_schedule_144_0_e1587);
        let noise_metadata_schedule_144_0_e1589: f64 = (1.0 + noise_metadata_schedule_144_0_e1588);
        (noise_metadata_schedule_144_0_e1589,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_144_0_e1591;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_145_0_e1607,) = {
    if ((w[504] != 0.0) && (w[505] == 0.0)) {
        let noise_metadata_schedule_145_0_e1600: f64 = (-w[285]);
        let noise_metadata_schedule_145_0_e1601: f64 = (noise_metadata_schedule_145_0_e1600).exp();
        let noise_metadata_schedule_145_0_e1602: f64 = (1.0 + noise_metadata_schedule_145_0_e1601);
        let noise_metadata_schedule_145_0_e1603: f64 = (noise_metadata_schedule_145_0_e1602).ln();
        let noise_metadata_schedule_145_0_e1604: f64 = (w[52] * noise_metadata_schedule_145_0_e1603);
        let noise_metadata_schedule_145_0_e1605: f64 = (w[51] + noise_metadata_schedule_145_0_e1604);
        (noise_metadata_schedule_145_0_e1605,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_145_0_e1607;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_146_0_e1615,) = {
    if (w[504] != 0.0) {
        let noise_metadata_schedule_146_0_e1612: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_146_0_e1613: f64 = (w[51] - noise_metadata_schedule_146_0_e1612);
        (noise_metadata_schedule_146_0_e1613,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_146_0_e1615;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_147_0_e1620,) = {
    if (w[504] == 0.0) {
        (params.p11,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_147_0_e1620;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_148_0_e1625: f64 = (params.p124 * w[12]);
            let noise_metadata_schedule_148_0_e1626: f64 = (1.0 + noise_metadata_schedule_148_0_e1625);
            let noise_metadata_schedule_148_0_e1627: f64 = (params.p43 * noise_metadata_schedule_148_0_e1626);
            w[341] = noise_metadata_schedule_148_0_e1627;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_149_0_e1630: f64 = (w[342] * w[342]);
            w[287] = noise_metadata_schedule_149_0_e1630;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_150_0_e1633: f64 = (w[341] * w[341]);
            w[288] = noise_metadata_schedule_150_0_e1633;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_151_0_e1636: f64 = if w[341] < 0.0 { 1.0 } else { 0.0 };
            w[506] = noise_metadata_schedule_151_0_e1636;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_152_0_e1649,) = {
    if (w[506] != 0.0) {
        let noise_metadata_schedule_152_0_e1640: f64 = (0.5 * w[287]);
        let noise_metadata_schedule_152_0_e1643: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_152_0_e1644: f64 = (noise_metadata_schedule_152_0_e1643).sqrt();
        let noise_metadata_schedule_152_0_e1646: f64 = (noise_metadata_schedule_152_0_e1644 - w[341]);
        let noise_metadata_schedule_152_0_e1647: f64 = (noise_metadata_schedule_152_0_e1640 / noise_metadata_schedule_152_0_e1646);
        (noise_metadata_schedule_152_0_e1647,)
    } else {
        (w[340],)
    }
};
            w[340] = noise_metadata_schedule_152_0_e1649;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_153_0_e1661,) = {
    if (w[506] == 0.0) {
        let noise_metadata_schedule_153_0_e1655: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_153_0_e1656: f64 = (noise_metadata_schedule_153_0_e1655).sqrt();
        let noise_metadata_schedule_153_0_e1658: f64 = (noise_metadata_schedule_153_0_e1656 + w[341]);
        let noise_metadata_schedule_153_0_e1659: f64 = (0.5 * noise_metadata_schedule_153_0_e1658);
        (noise_metadata_schedule_153_0_e1659,)
    } else {
        (w[340],)
    }
};
            w[340] = noise_metadata_schedule_153_0_e1661;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_154_0_e1666: f64 = (4.0 - params.p98);
            let noise_metadata_schedule_154_0_e1668: f64 = (noise_metadata_schedule_154_0_e1666 - params.p96);
            let noise_metadata_schedule_154_0_e1670: f64 = (noise_metadata_schedule_154_0_e1668 + params.p121);
            let noise_metadata_schedule_154_0_e1671: f64 = (w[280] * noise_metadata_schedule_154_0_e1670);
            let noise_metadata_schedule_154_0_e1673: f64 = (noise_metadata_schedule_154_0_e1671 / w[48]);
            let noise_metadata_schedule_154_0_e1674: f64 = (noise_metadata_schedule_154_0_e1673).exp();
            let noise_metadata_schedule_154_0_e1675: f64 = (params.p9 * noise_metadata_schedule_154_0_e1674);
            let noise_metadata_schedule_154_0_e1677: f64 = (-params.p105);
            let noise_metadata_schedule_154_0_e1679: f64 = (noise_metadata_schedule_154_0_e1677 * w[10]);
            let noise_metadata_schedule_154_0_e1681: f64 = (noise_metadata_schedule_154_0_e1679 / w[48]);
            let noise_metadata_schedule_154_0_e1682: f64 = (noise_metadata_schedule_154_0_e1681).exp();
            let noise_metadata_schedule_154_0_e1683: f64 = (noise_metadata_schedule_154_0_e1675 * noise_metadata_schedule_154_0_e1682);
            w[35] = noise_metadata_schedule_154_0_e1683;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_155_0_e1688: f64 = (1.0 - params.p98);
            let noise_metadata_schedule_155_0_e1689: f64 = (w[280] * noise_metadata_schedule_155_0_e1688);
            let noise_metadata_schedule_155_0_e1690: f64 = (noise_metadata_schedule_155_0_e1689).exp();
            let noise_metadata_schedule_155_0_e1691: f64 = (params.p12 * noise_metadata_schedule_155_0_e1690);
            w[36] = noise_metadata_schedule_155_0_e1691;
        }
        if (active[0] & 0x87800) != 0 {
            let noise_metadata_schedule_156_0_e1696: f64 = (1.0 - params.p103);
            let noise_metadata_schedule_156_0_e1697: f64 = (w[280] * noise_metadata_schedule_156_0_e1696);
            let noise_metadata_schedule_156_0_e1698: f64 = (noise_metadata_schedule_156_0_e1697).exp();
            let noise_metadata_schedule_156_0_e1699: f64 = (params.p30 * noise_metadata_schedule_156_0_e1698);
            w[37] = noise_metadata_schedule_156_0_e1699;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_157_0_e1705: f64 = (2.0 * params.p21);
            let noise_metadata_schedule_157_0_e1706: f64 = (6.0 - noise_metadata_schedule_157_0_e1705);
            let noise_metadata_schedule_157_0_e1707: f64 = (w[280] * noise_metadata_schedule_157_0_e1706);
            let noise_metadata_schedule_157_0_e1708: f64 = (noise_metadata_schedule_157_0_e1707).exp();
            let noise_metadata_schedule_157_0_e1709: f64 = (params.p20 * noise_metadata_schedule_157_0_e1708);
            let noise_metadata_schedule_157_0_e1711: f64 = (-params.p113);
            let noise_metadata_schedule_157_0_e1713: f64 = (noise_metadata_schedule_157_0_e1711 * w[10]);
            let noise_metadata_schedule_157_0_e1715: f64 = (noise_metadata_schedule_157_0_e1713 / params.p21);
            let noise_metadata_schedule_157_0_e1716: f64 = (noise_metadata_schedule_157_0_e1715).exp();
            let noise_metadata_schedule_157_0_e1717: f64 = (noise_metadata_schedule_157_0_e1709 * noise_metadata_schedule_157_0_e1716);
            w[38] = noise_metadata_schedule_157_0_e1717;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_158_0_e1723: f64 = (2.0 * params.p32);
            let noise_metadata_schedule_158_0_e1724: f64 = (6.0 - noise_metadata_schedule_158_0_e1723);
            let noise_metadata_schedule_158_0_e1725: f64 = (w[280] * noise_metadata_schedule_158_0_e1724);
            let noise_metadata_schedule_158_0_e1726: f64 = (noise_metadata_schedule_158_0_e1725).exp();
            let noise_metadata_schedule_158_0_e1727: f64 = (params.p31 * noise_metadata_schedule_158_0_e1726);
            let noise_metadata_schedule_158_0_e1729: f64 = (-params.p110);
            let noise_metadata_schedule_158_0_e1731: f64 = (noise_metadata_schedule_158_0_e1729 * w[10]);
            let noise_metadata_schedule_158_0_e1733: f64 = (noise_metadata_schedule_158_0_e1731 / params.p32);
            let noise_metadata_schedule_158_0_e1734: f64 = (noise_metadata_schedule_158_0_e1733).exp();
            let noise_metadata_schedule_158_0_e1735: f64 = (noise_metadata_schedule_158_0_e1727 * noise_metadata_schedule_158_0_e1734);
            w[39] = noise_metadata_schedule_158_0_e1735;
        }
        if (active[0] & 0x46) != 0 {
            let noise_metadata_schedule_159_0_e1740: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_159_0_e1742: f64 = (noise_metadata_schedule_159_0_e1740 + params.p121);
            let noise_metadata_schedule_159_0_e1743: f64 = (w[280] * noise_metadata_schedule_159_0_e1742);
            let noise_metadata_schedule_159_0_e1745: f64 = (noise_metadata_schedule_159_0_e1743 / params.p17);
            let noise_metadata_schedule_159_0_e1746: f64 = (noise_metadata_schedule_159_0_e1745).exp();
            let noise_metadata_schedule_159_0_e1747: f64 = (params.p16 * noise_metadata_schedule_159_0_e1746);
            let noise_metadata_schedule_159_0_e1749: f64 = (-params.p111);
            let noise_metadata_schedule_159_0_e1751: f64 = (noise_metadata_schedule_159_0_e1749 * w[10]);
            let noise_metadata_schedule_159_0_e1753: f64 = (noise_metadata_schedule_159_0_e1751 / params.p17);
            let noise_metadata_schedule_159_0_e1754: f64 = (noise_metadata_schedule_159_0_e1753).exp();
            let noise_metadata_schedule_159_0_e1755: f64 = (noise_metadata_schedule_159_0_e1747 * noise_metadata_schedule_159_0_e1754);
            w[42] = noise_metadata_schedule_159_0_e1755;
        }
        if (active[0] & 0x140) != 0 {
            let noise_metadata_schedule_160_0_e1760: f64 = (4.0 - params.p97);
            let noise_metadata_schedule_160_0_e1762: f64 = (noise_metadata_schedule_160_0_e1760 + params.p121);
            let noise_metadata_schedule_160_0_e1763: f64 = (w[280] * noise_metadata_schedule_160_0_e1762);
            let noise_metadata_schedule_160_0_e1765: f64 = (noise_metadata_schedule_160_0_e1763 / params.p19);
            let noise_metadata_schedule_160_0_e1766: f64 = (noise_metadata_schedule_160_0_e1765).exp();
            let noise_metadata_schedule_160_0_e1767: f64 = (params.p18 * noise_metadata_schedule_160_0_e1766);
            let noise_metadata_schedule_160_0_e1769: f64 = (-params.p111);
            let noise_metadata_schedule_160_0_e1771: f64 = (noise_metadata_schedule_160_0_e1769 * w[10]);
            let noise_metadata_schedule_160_0_e1773: f64 = (noise_metadata_schedule_160_0_e1771 / params.p19);
            let noise_metadata_schedule_160_0_e1774: f64 = (noise_metadata_schedule_160_0_e1773).exp();
            let noise_metadata_schedule_160_0_e1775: f64 = (noise_metadata_schedule_160_0_e1767 * noise_metadata_schedule_160_0_e1774);
            w[44] = noise_metadata_schedule_160_0_e1775;
        }
        if (active[0] & 0x144) != 0 {
            let noise_metadata_schedule_161_0_e1778: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            w[507] = noise_metadata_schedule_161_0_e1778;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_162_0_e1790,) = {
    if (w[507] != 0.0) {
        let noise_metadata_schedule_162_0_e1782: f64 = (-params.p107);
        let noise_metadata_schedule_162_0_e1784: f64 = (noise_metadata_schedule_162_0_e1782 * w[10]);
        let noise_metadata_schedule_162_0_e1786: f64 = (noise_metadata_schedule_162_0_e1784 / params.p17);
        let noise_metadata_schedule_162_0_e1787: f64 = (noise_metadata_schedule_162_0_e1786).exp();
        let noise_metadata_schedule_162_0_e1788: f64 = (params.p25 * noise_metadata_schedule_162_0_e1787);
        (noise_metadata_schedule_162_0_e1788,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_162_0_e1790;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_163_0_e1800,) = {
    if (w[507] != 0.0) {
        let noise_metadata_schedule_163_0_e1794: f64 = (-params.p106);
        let noise_metadata_schedule_163_0_e1796: f64 = (noise_metadata_schedule_163_0_e1794 * w[10]);
        let noise_metadata_schedule_163_0_e1797: f64 = (noise_metadata_schedule_163_0_e1796).exp();
        let noise_metadata_schedule_163_0_e1798: f64 = (params.p28 * noise_metadata_schedule_163_0_e1797);
        (noise_metadata_schedule_163_0_e1798,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_163_0_e1800;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_164_0_e1812,) = {
    if (w[507] != 0.0) {
        let noise_metadata_schedule_164_0_e1804: f64 = (-params.p108);
        let noise_metadata_schedule_164_0_e1806: f64 = (noise_metadata_schedule_164_0_e1804 * w[10]);
        let noise_metadata_schedule_164_0_e1808: f64 = (noise_metadata_schedule_164_0_e1806 / params.p19);
        let noise_metadata_schedule_164_0_e1809: f64 = (noise_metadata_schedule_164_0_e1808).exp();
        let noise_metadata_schedule_164_0_e1810: f64 = (params.p26 * noise_metadata_schedule_164_0_e1809);
        (noise_metadata_schedule_164_0_e1810,)
    } else {
        (w[45],)
    }
};
            w[45] = noise_metadata_schedule_164_0_e1812;
        }
        if (active[0] & 0x9fe00) != 0 {
            let noise_metadata_schedule_165_0_e1817: f64 = (4.0 - params.p103);
            let noise_metadata_schedule_165_0_e1819: f64 = (noise_metadata_schedule_165_0_e1817 + params.p121);
            let noise_metadata_schedule_165_0_e1820: f64 = (w[280] * noise_metadata_schedule_165_0_e1819);
            let noise_metadata_schedule_165_0_e1821: f64 = (noise_metadata_schedule_165_0_e1820).exp();
            let noise_metadata_schedule_165_0_e1822: f64 = (params.p29 * noise_metadata_schedule_165_0_e1821);
            let noise_metadata_schedule_165_0_e1824: f64 = (-params.p112);
            let noise_metadata_schedule_165_0_e1826: f64 = (noise_metadata_schedule_165_0_e1824 * w[10]);
            let noise_metadata_schedule_165_0_e1827: f64 = (noise_metadata_schedule_165_0_e1826).exp();
            let noise_metadata_schedule_165_0_e1828: f64 = (noise_metadata_schedule_165_0_e1822 * noise_metadata_schedule_165_0_e1827);
            w[43] = noise_metadata_schedule_165_0_e1828;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_166_0_e1834: f64 = (2.0 * params.p23);
            let noise_metadata_schedule_166_0_e1835: f64 = (6.0 - noise_metadata_schedule_166_0_e1834);
            let noise_metadata_schedule_166_0_e1836: f64 = (w[280] * noise_metadata_schedule_166_0_e1835);
            let noise_metadata_schedule_166_0_e1837: f64 = (noise_metadata_schedule_166_0_e1836).exp();
            let noise_metadata_schedule_166_0_e1838: f64 = (params.p22 * noise_metadata_schedule_166_0_e1837);
            let noise_metadata_schedule_166_0_e1840: f64 = (-params.p113);
            let noise_metadata_schedule_166_0_e1842: f64 = (noise_metadata_schedule_166_0_e1840 * w[10]);
            let noise_metadata_schedule_166_0_e1844: f64 = (noise_metadata_schedule_166_0_e1842 / params.p23);
            let noise_metadata_schedule_166_0_e1845: f64 = (noise_metadata_schedule_166_0_e1844).exp();
            let noise_metadata_schedule_166_0_e1846: f64 = (noise_metadata_schedule_166_0_e1838 * noise_metadata_schedule_166_0_e1845);
            w[46] = noise_metadata_schedule_166_0_e1846;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_167_0_e1851: f64 = (4.0 / params.p150);
            let noise_metadata_schedule_167_0_e1852: f64 = (w[280] * noise_metadata_schedule_167_0_e1851);
            let noise_metadata_schedule_167_0_e1853: f64 = (noise_metadata_schedule_167_0_e1852).exp();
            let noise_metadata_schedule_167_0_e1854: f64 = (params.p149 * noise_metadata_schedule_167_0_e1853);
            let noise_metadata_schedule_167_0_e1856: f64 = (-params.p113);
            let noise_metadata_schedule_167_0_e1858: f64 = (noise_metadata_schedule_167_0_e1856 * w[10]);
            let noise_metadata_schedule_167_0_e1860: f64 = (noise_metadata_schedule_167_0_e1858 / params.p150);
            let noise_metadata_schedule_167_0_e1861: f64 = (noise_metadata_schedule_167_0_e1860).exp();
            let noise_metadata_schedule_167_0_e1862: f64 = (noise_metadata_schedule_167_0_e1854 * noise_metadata_schedule_167_0_e1861);
            w[47] = noise_metadata_schedule_167_0_e1862;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_168_0_e1865: f64 = (w[4]).sqrt();
            let noise_metadata_schedule_168_0_e1866: f64 = (params.p155 * noise_metadata_schedule_168_0_e1865);
            let noise_metadata_schedule_168_0_e1869: f64 = (params.p157 * w[12]);
            let noise_metadata_schedule_168_0_e1870: f64 = (noise_metadata_schedule_168_0_e1869).exp();
            let noise_metadata_schedule_168_0_e1871: f64 = (noise_metadata_schedule_168_0_e1866 * noise_metadata_schedule_168_0_e1870);
            w[357] = noise_metadata_schedule_168_0_e1871;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_169_0_e1874: f64 = (w[70] * w[72]);
            let noise_metadata_schedule_169_0_e1876: f64 = (-0.5);
            let noise_metadata_schedule_169_0_e1877: f64 = (noise_metadata_schedule_169_0_e1874).powf(noise_metadata_schedule_169_0_e1876);
            w[281] = noise_metadata_schedule_169_0_e1877;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_170_0_e1880: f64 = (1.0 / w[73]);
            w[282] = noise_metadata_schedule_170_0_e1880;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_171_0_e1883: f64 = (params.p35 * w[70]);
            let noise_metadata_schedule_171_0_e1885: f64 = (noise_metadata_schedule_171_0_e1883 * w[70]);
            let noise_metadata_schedule_171_0_e1887: f64 = (noise_metadata_schedule_171_0_e1885 * w[281]);
            let noise_metadata_schedule_171_0_e1889: f64 = (noise_metadata_schedule_171_0_e1887 * w[282]);
            let noise_metadata_schedule_171_0_e1891: f64 = (noise_metadata_schedule_171_0_e1889 * params.p66);
            let noise_metadata_schedule_171_0_e1893: f64 = (noise_metadata_schedule_171_0_e1891 * w[65]);
            let noise_metadata_schedule_171_0_e1895: f64 = (noise_metadata_schedule_171_0_e1893 * w[72]);
            let noise_metadata_schedule_171_0_e1897: f64 = (noise_metadata_schedule_171_0_e1895 * w[72]);
            w[61] = noise_metadata_schedule_171_0_e1897;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_172_0_e1900: f64 = (params.p34 * w[281]);
            let noise_metadata_schedule_172_0_e1902: f64 = (noise_metadata_schedule_172_0_e1900 * w[14]);
            let noise_metadata_schedule_172_0_e1904: f64 = (noise_metadata_schedule_172_0_e1902 * w[14]);
            let noise_metadata_schedule_172_0_e1906: f64 = (noise_metadata_schedule_172_0_e1904 * w[64]);
            let noise_metadata_schedule_172_0_e1908: f64 = (noise_metadata_schedule_172_0_e1906 * w[64]);
            let noise_metadata_schedule_172_0_e1910: f64 = (noise_metadata_schedule_172_0_e1908 * w[73]);
            let noise_metadata_schedule_172_0_e1913: f64 = (params.p35 - w[61]);
            let noise_metadata_schedule_172_0_e1914: f64 = (noise_metadata_schedule_172_0_e1913).exp();
            let noise_metadata_schedule_172_0_e1915: f64 = (noise_metadata_schedule_172_0_e1910 * noise_metadata_schedule_172_0_e1914);
            w[58] = noise_metadata_schedule_172_0_e1915;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_173_0_e1918: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_173_0_e1918;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_174_0_e1921: f64 = (w[85] * w[86]);
            let noise_metadata_schedule_174_0_e1923: f64 = (-0.5);
            let noise_metadata_schedule_174_0_e1924: f64 = (noise_metadata_schedule_174_0_e1921).powf(noise_metadata_schedule_174_0_e1923);
            w[283] = noise_metadata_schedule_174_0_e1924;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_175_0_e1927: f64 = (1.0 / w[90]);
            w[284] = noise_metadata_schedule_175_0_e1927;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_176_0_e1930: f64 = (params.p37 * w[85]);
            let noise_metadata_schedule_176_0_e1932: f64 = (noise_metadata_schedule_176_0_e1930 * w[85]);
            let noise_metadata_schedule_176_0_e1934: f64 = (noise_metadata_schedule_176_0_e1932 * w[283]);
            let noise_metadata_schedule_176_0_e1936: f64 = (noise_metadata_schedule_176_0_e1934 * w[284]);
            let noise_metadata_schedule_176_0_e1938: f64 = (noise_metadata_schedule_176_0_e1936 * w[75]);
            let noise_metadata_schedule_176_0_e1940: f64 = (noise_metadata_schedule_176_0_e1938 * w[67]);
            let noise_metadata_schedule_176_0_e1942: f64 = (noise_metadata_schedule_176_0_e1940 * w[86]);
            let noise_metadata_schedule_176_0_e1944: f64 = (noise_metadata_schedule_176_0_e1942 * w[86]);
            w[83] = noise_metadata_schedule_176_0_e1944;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_177_0_e1947: f64 = (params.p36 * w[283]);
            let noise_metadata_schedule_177_0_e1949: f64 = (noise_metadata_schedule_177_0_e1947 * w[19]);
            let noise_metadata_schedule_177_0_e1951: f64 = (noise_metadata_schedule_177_0_e1949 * w[19]);
            let noise_metadata_schedule_177_0_e1953: f64 = (noise_metadata_schedule_177_0_e1951 * w[66]);
            let noise_metadata_schedule_177_0_e1955: f64 = (noise_metadata_schedule_177_0_e1953 * w[66]);
            let noise_metadata_schedule_177_0_e1957: f64 = (noise_metadata_schedule_177_0_e1955 * w[90]);
            let noise_metadata_schedule_177_0_e1960: f64 = (params.p37 - w[83]);
            let noise_metadata_schedule_177_0_e1961: f64 = (noise_metadata_schedule_177_0_e1960).exp();
            let noise_metadata_schedule_177_0_e1962: f64 = (noise_metadata_schedule_177_0_e1957 * noise_metadata_schedule_177_0_e1961);
            w[84] = noise_metadata_schedule_177_0_e1962;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_178_0_e1965: f64 = (w[280] * params.p96);
            let noise_metadata_schedule_178_0_e1966: f64 = (noise_metadata_schedule_178_0_e1965).exp();
            w[281] = noise_metadata_schedule_178_0_e1966;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_179_0_e1969: f64 = (params.p14 * w[281]);
            let noise_metadata_schedule_179_0_e1971: f64 = (noise_metadata_schedule_179_0_e1969 * w[27]);
            w[40] = noise_metadata_schedule_179_0_e1971;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_180_0_e1974: f64 = (params.p13 * w[281]);
            let noise_metadata_schedule_180_0_e1976: f64 = (noise_metadata_schedule_180_0_e1974 * w[282]);
            w[41] = noise_metadata_schedule_180_0_e1976;
        }
        if (active[0] & 0xffe00) != 0 {
            let noise_metadata_schedule_181_0_e1981: f64 = (4.0 - params.p141);
            let noise_metadata_schedule_181_0_e1982: f64 = (w[280] * noise_metadata_schedule_181_0_e1981);
            let noise_metadata_schedule_181_0_e1983: f64 = (noise_metadata_schedule_181_0_e1982).exp();
            let noise_metadata_schedule_181_0_e1984: f64 = (params.p133 * noise_metadata_schedule_181_0_e1983);
            let noise_metadata_schedule_181_0_e1986: f64 = (-params.p140);
            let noise_metadata_schedule_181_0_e1988: f64 = (noise_metadata_schedule_181_0_e1986 * w[10]);
            let noise_metadata_schedule_181_0_e1989: f64 = (noise_metadata_schedule_181_0_e1988).exp();
            let noise_metadata_schedule_181_0_e1990: f64 = (noise_metadata_schedule_181_0_e1984 * noise_metadata_schedule_181_0_e1989);
            w[107] = noise_metadata_schedule_181_0_e1990;
        }
        if (active[0] & 0xe6000) != 0 {
            let noise_metadata_schedule_183_0_e2011: f64 = (1.0 - params.p141);
            let noise_metadata_schedule_183_0_e2012: f64 = (w[280] * noise_metadata_schedule_183_0_e2011);
            let noise_metadata_schedule_183_0_e2013: f64 = (noise_metadata_schedule_183_0_e2012).exp();
            let noise_metadata_schedule_183_0_e2014: f64 = (params.p135 * noise_metadata_schedule_183_0_e2013);
            w[109] = noise_metadata_schedule_183_0_e2014;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_190_0_e2074: f64 = (w[2] - 300.0);
            w[101] = noise_metadata_schedule_190_0_e2074;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_191_0_e2077: f64 = if w[2] < 525.0 { 1.0 } else { 0.0 };
            w[508] = noise_metadata_schedule_191_0_e2077;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_192_0_e2093,) = {
    if (w[508] != 0.0) {
        let noise_metadata_schedule_192_0_e2083: f64 = (0.00072 * w[101]);
        let noise_metadata_schedule_192_0_e2084: f64 = (1.0 + noise_metadata_schedule_192_0_e2083);
        let noise_metadata_schedule_192_0_e2087: f64 = (1.6e-6 * w[101]);
        let noise_metadata_schedule_192_0_e2089: f64 = (noise_metadata_schedule_192_0_e2087 * w[101]);
        let noise_metadata_schedule_192_0_e2090: f64 = (noise_metadata_schedule_192_0_e2084 - noise_metadata_schedule_192_0_e2089);
        let noise_metadata_schedule_192_0_e2091: f64 = (w[1] * noise_metadata_schedule_192_0_e2090);
        (noise_metadata_schedule_192_0_e2091,)
    } else {
        (w[99],)
    }
};
            w[99] = noise_metadata_schedule_192_0_e2093;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_193_0_e2100,) = {
    if (w[508] == 0.0) {
        let noise_metadata_schedule_193_0_e2098: f64 = (w[1] * 1.081);
        (noise_metadata_schedule_193_0_e2098,)
    } else {
        (w[99],)
    }
};
            w[99] = noise_metadata_schedule_193_0_e2100;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_194_0_e2104: f64 = (w[280] * params.p96);
            let noise_metadata_schedule_194_0_e2105: f64 = (noise_metadata_schedule_194_0_e2104).exp();
            let noise_metadata_schedule_194_0_e2106: f64 = (params.p92 * noise_metadata_schedule_194_0_e2105);
            w[100] = noise_metadata_schedule_194_0_e2106;
        }
        if (active[0] & 0xa900000) != 0 {
            let noise_metadata_schedule_196_0_e2116: f64 = if params.p57 > 0.0 { 1.0 } else { 0.0 };
            w[509] = noise_metadata_schedule_196_0_e2116;
        }
        if (active[0] & 0xa900000) != 0 {
            let (noise_metadata_schedule_197_0_e2122,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_197_0_e2120: f64 = (1.0 / w[32]);
        (noise_metadata_schedule_197_0_e2120,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_197_0_e2122;
        }
        if (active[0] & 0xa900000) != 0 {
            let noise_metadata_schedule_198_0_e2125: f64 = if w[111] > w[347] { 1.0 } else { 0.0 };
            w[510] = noise_metadata_schedule_198_0_e2125;
        }
        if (active[0] & 0xa900000) != 0 {
            let (noise_metadata_schedule_199_0_e2131,) = {
    if ((w[509] != 0.0) && (w[510] != 0.0)) {
        (w[347],)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_199_0_e2131;
        }
        if (active[0] & 0xa900000) != 0 {
            let (noise_metadata_schedule_200_0_e2136,) = {
    if (w[509] == 0.0) {
        (0.0,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_200_0_e2136;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_201_0_e2139: f64 = if params.p58 > 0.0 { 1.0 } else { 0.0 };
            w[511] = noise_metadata_schedule_201_0_e2139;
        }
        if (active[0] & 0x1200000) != 0 {
            let (noise_metadata_schedule_202_0_e2145,) = {
    if (w[511] != 0.0) {
        let noise_metadata_schedule_202_0_e2143: f64 = (1.0 / w[33]);
        (noise_metadata_schedule_202_0_e2143,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_202_0_e2145;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_203_0_e2148: f64 = if w[112] > w[347] { 1.0 } else { 0.0 };
            w[512] = noise_metadata_schedule_203_0_e2148;
        }
        if (active[0] & 0x1200000) != 0 {
            let (noise_metadata_schedule_204_0_e2154,) = {
    if ((w[511] != 0.0) && (w[512] != 0.0)) {
        (w[347],)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_204_0_e2154;
        }
        if (active[0] & 0x1200000) != 0 {
            let (noise_metadata_schedule_205_0_e2159,) = {
    if (w[511] == 0.0) {
        (0.0,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_205_0_e2159;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_206_0_e2162: f64 = if params.p59 > 0.0 { 1.0 } else { 0.0 };
            w[513] = noise_metadata_schedule_206_0_e2162;
        }
        if (active[0] & 0x4400000) != 0 {
            let (noise_metadata_schedule_207_0_e2168,) = {
    if (w[513] != 0.0) {
        let noise_metadata_schedule_207_0_e2166: f64 = (1.0 / w[34]);
        (noise_metadata_schedule_207_0_e2166,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_207_0_e2168;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_208_0_e2171: f64 = if w[113] > w[347] { 1.0 } else { 0.0 };
            w[514] = noise_metadata_schedule_208_0_e2171;
        }
        if (active[0] & 0x4400000) != 0 {
            let (noise_metadata_schedule_209_0_e2177,) = {
    if ((w[513] != 0.0) && (w[514] != 0.0)) {
        (w[347],)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_209_0_e2177;
        }
        if (active[0] & 0x4400000) != 0 {
            let (noise_metadata_schedule_210_0_e2182,) = {
    if (w[513] == 0.0) {
        (0.0,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_210_0_e2182;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_211_0_e2185: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            w[250] = noise_metadata_schedule_211_0_e2185;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_212_0_e2188: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[9])));
            w[251] = noise_metadata_schedule_212_0_e2188;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_213_0_e2191: f64 = (params.p3 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[5])));
            w[252] = noise_metadata_schedule_213_0_e2191;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_214_0_e2194: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[5])));
            w[253] = noise_metadata_schedule_214_0_e2194;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_215_0_e2197: f64 = (params.p3 * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            w[254] = noise_metadata_schedule_215_0_e2197;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_216_0_e2200: f64 = (params.p3 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[8])));
            w[259] = noise_metadata_schedule_216_0_e2200;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_217_0_e2203: f64 = (params.p3 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
            w[256] = noise_metadata_schedule_217_0_e2203;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_219_0_e2209: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
            w[266] = noise_metadata_schedule_219_0_e2209;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_221_0_e2215: f64 = (params.p3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            w[270] = noise_metadata_schedule_221_0_e2215;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_222_0_e2218: f64 = (params.p3 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[8])));
            w[258] = noise_metadata_schedule_222_0_e2218;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_223_0_e2221: f64 = (params.p3 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[11])));
            w[257] = noise_metadata_schedule_223_0_e2221;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_224_0_e2224: f64 = (w[254] + w[251]);
            let noise_metadata_schedule_224_0_e2226: f64 = (noise_metadata_schedule_224_0_e2224 - w[256]);
            let noise_metadata_schedule_224_0_e2228: f64 = (noise_metadata_schedule_224_0_e2226 - w[258]);
            w[255] = noise_metadata_schedule_224_0_e2228;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_225_0_e2230: f64 = (-w[270]);
            let noise_metadata_schedule_225_0_e2232: f64 = (noise_metadata_schedule_225_0_e2230 + w[266]);
            let noise_metadata_schedule_225_0_e2234: f64 = (noise_metadata_schedule_225_0_e2232 + w[255]);
            let noise_metadata_schedule_225_0_e2236: f64 = (noise_metadata_schedule_225_0_e2234 - w[257]);
            w[268] = noise_metadata_schedule_225_0_e2236;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_226_0_e2239: f64 = (w[270] + w[268]);
            w[267] = noise_metadata_schedule_226_0_e2239;
        }
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_227_0_e2242: f64 = (w[259] - w[258]);
            w[261] = noise_metadata_schedule_227_0_e2242;
        }
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_228_0_e2245: f64 = (w[261] - w[257]);
            w[260] = noise_metadata_schedule_228_0_e2245;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_229_0_e2248: f64 = (w[251] * w[8]);
            let noise_metadata_schedule_229_0_e2250: f64 = if noise_metadata_schedule_229_0_e2248 < params.p151 { 1.0 } else { 0.0 };
            w[515] = noise_metadata_schedule_229_0_e2250;
        }
        if (active[0] & 0x387e7) != 0 {
            let (noise_metadata_schedule_230_0_e2257,) = {
    if (w[515] != 0.0) {
        let noise_metadata_schedule_230_0_e2254: f64 = (w[251] * w[8]);
        let noise_metadata_schedule_230_0_e2255: f64 = (noise_metadata_schedule_230_0_e2254).exp();
        (noise_metadata_schedule_230_0_e2255,)
    } else {
        (w[271],)
    }
};
            w[271] = noise_metadata_schedule_230_0_e2257;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_231_0_e2263,) = {
    if (w[515] == 0.0) {
        let noise_metadata_schedule_231_0_e2261: f64 = (params.p151).exp();
        (noise_metadata_schedule_231_0_e2261,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_231_0_e2263;
        }
        if (active[0] & 0x387e7) != 0 {
            let (noise_metadata_schedule_232_0_e2276,) = {
    if (w[515] == 0.0) {
        let noise_metadata_schedule_232_0_e2270: f64 = (w[251] * w[8]);
        let noise_metadata_schedule_232_0_e2272: f64 = (noise_metadata_schedule_232_0_e2270 - params.p151);
        let noise_metadata_schedule_232_0_e2273: f64 = (1.0 + noise_metadata_schedule_232_0_e2272);
        let noise_metadata_schedule_232_0_e2274: f64 = (w[301] * noise_metadata_schedule_232_0_e2273);
        (noise_metadata_schedule_232_0_e2274,)
    } else {
        (w[271],)
    }
};
            w[271] = noise_metadata_schedule_232_0_e2276;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_233_0_e2279: f64 = (w[252] * w[8]);
            let noise_metadata_schedule_233_0_e2281: f64 = (noise_metadata_schedule_233_0_e2279 / w[48]);
            let noise_metadata_schedule_233_0_e2283: f64 = if noise_metadata_schedule_233_0_e2281 < params.p151 { 1.0 } else { 0.0 };
            w[516] = noise_metadata_schedule_233_0_e2283;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_234_0_e2292,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_234_0_e2287: f64 = (w[252] * w[8]);
        let noise_metadata_schedule_234_0_e2289: f64 = (noise_metadata_schedule_234_0_e2287 / w[48]);
        let noise_metadata_schedule_234_0_e2290: f64 = (noise_metadata_schedule_234_0_e2289).exp();
        (noise_metadata_schedule_234_0_e2290,)
    } else {
        (w[272],)
    }
};
            w[272] = noise_metadata_schedule_234_0_e2292;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_235_0_e2298,) = {
    if (w[516] == 0.0) {
        let noise_metadata_schedule_235_0_e2296: f64 = (params.p151).exp();
        (noise_metadata_schedule_235_0_e2296,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_235_0_e2298;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_236_0_e2313,) = {
    if (w[516] == 0.0) {
        let noise_metadata_schedule_236_0_e2305: f64 = (w[252] * w[8]);
        let noise_metadata_schedule_236_0_e2307: f64 = (noise_metadata_schedule_236_0_e2305 / w[48]);
        let noise_metadata_schedule_236_0_e2309: f64 = (noise_metadata_schedule_236_0_e2307 - params.p151);
        let noise_metadata_schedule_236_0_e2310: f64 = (1.0 + noise_metadata_schedule_236_0_e2309);
        let noise_metadata_schedule_236_0_e2311: f64 = (w[301] * noise_metadata_schedule_236_0_e2310);
        (noise_metadata_schedule_236_0_e2311,)
    } else {
        (w[272],)
    }
};
            w[272] = noise_metadata_schedule_236_0_e2313;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_237_0_e2316: f64 = (w[255] * w[8]);
            let noise_metadata_schedule_237_0_e2318: f64 = if noise_metadata_schedule_237_0_e2316 < params.p151 { 1.0 } else { 0.0 };
            w[517] = noise_metadata_schedule_237_0_e2318;
        }
        if (active[0] & 0x41800) != 0 {
            let (noise_metadata_schedule_238_0_e2325,) = {
    if (w[517] != 0.0) {
        let noise_metadata_schedule_238_0_e2322: f64 = (w[255] * w[8]);
        let noise_metadata_schedule_238_0_e2323: f64 = (noise_metadata_schedule_238_0_e2322).exp();
        (noise_metadata_schedule_238_0_e2323,)
    } else {
        (w[274],)
    }
};
            w[274] = noise_metadata_schedule_238_0_e2325;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_239_0_e2331,) = {
    if (w[517] == 0.0) {
        let noise_metadata_schedule_239_0_e2329: f64 = (params.p151).exp();
        (noise_metadata_schedule_239_0_e2329,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_239_0_e2331;
        }
        if (active[0] & 0x41800) != 0 {
            let (noise_metadata_schedule_240_0_e2344,) = {
    if (w[517] == 0.0) {
        let noise_metadata_schedule_240_0_e2338: f64 = (w[255] * w[8]);
        let noise_metadata_schedule_240_0_e2340: f64 = (noise_metadata_schedule_240_0_e2338 - params.p151);
        let noise_metadata_schedule_240_0_e2341: f64 = (1.0 + noise_metadata_schedule_240_0_e2340);
        let noise_metadata_schedule_240_0_e2342: f64 = (w[301] * noise_metadata_schedule_240_0_e2341);
        (noise_metadata_schedule_240_0_e2342,)
    } else {
        (w[274],)
    }
};
            w[274] = noise_metadata_schedule_240_0_e2344;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_241_0_e2347: f64 = (w[254] * w[8]);
            let noise_metadata_schedule_241_0_e2349: f64 = if noise_metadata_schedule_241_0_e2347 < params.p151 { 1.0 } else { 0.0 };
            w[518] = noise_metadata_schedule_241_0_e2349;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_242_0_e2356,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_242_0_e2353: f64 = (w[254] * w[8]);
        let noise_metadata_schedule_242_0_e2354: f64 = (noise_metadata_schedule_242_0_e2353).exp();
        (noise_metadata_schedule_242_0_e2354,)
    } else {
        (w[273],)
    }
};
            w[273] = noise_metadata_schedule_242_0_e2356;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_243_0_e2362,) = {
    if (w[518] == 0.0) {
        let noise_metadata_schedule_243_0_e2360: f64 = (params.p151).exp();
        (noise_metadata_schedule_243_0_e2360,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_243_0_e2362;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_244_0_e2375,) = {
    if (w[518] == 0.0) {
        let noise_metadata_schedule_244_0_e2369: f64 = (w[254] * w[8]);
        let noise_metadata_schedule_244_0_e2371: f64 = (noise_metadata_schedule_244_0_e2369 - params.p151);
        let noise_metadata_schedule_244_0_e2372: f64 = (1.0 + noise_metadata_schedule_244_0_e2371);
        let noise_metadata_schedule_244_0_e2373: f64 = (w[301] * noise_metadata_schedule_244_0_e2372);
        (noise_metadata_schedule_244_0_e2373,)
    } else {
        (w[273],)
    }
};
            w[273] = noise_metadata_schedule_244_0_e2375;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_245_0_e2378: f64 = (w[267] * w[8]);
            let noise_metadata_schedule_245_0_e2380: f64 = if noise_metadata_schedule_245_0_e2378 < params.p151 { 1.0 } else { 0.0 };
            w[519] = noise_metadata_schedule_245_0_e2380;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_246_0_e2387,) = {
    if (w[519] != 0.0) {
        let noise_metadata_schedule_246_0_e2384: f64 = (w[267] * w[8]);
        let noise_metadata_schedule_246_0_e2385: f64 = (noise_metadata_schedule_246_0_e2384).exp();
        (noise_metadata_schedule_246_0_e2385,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_246_0_e2387;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_247_0_e2393,) = {
    if (w[519] == 0.0) {
        let noise_metadata_schedule_247_0_e2391: f64 = (params.p151).exp();
        (noise_metadata_schedule_247_0_e2391,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_247_0_e2393;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_248_0_e2406,) = {
    if (w[519] == 0.0) {
        let noise_metadata_schedule_248_0_e2400: f64 = (w[267] * w[8]);
        let noise_metadata_schedule_248_0_e2402: f64 = (noise_metadata_schedule_248_0_e2400 - params.p151);
        let noise_metadata_schedule_248_0_e2403: f64 = (1.0 + noise_metadata_schedule_248_0_e2402);
        let noise_metadata_schedule_248_0_e2404: f64 = (w[301] * noise_metadata_schedule_248_0_e2403);
        (noise_metadata_schedule_248_0_e2404,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_248_0_e2406;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_249_0_e2409: f64 = (w[259] * w[8]);
            let noise_metadata_schedule_249_0_e2411: f64 = if noise_metadata_schedule_249_0_e2409 < params.p151 { 1.0 } else { 0.0 };
            w[520] = noise_metadata_schedule_249_0_e2411;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_250_0_e2418,) = {
    if (w[520] != 0.0) {
        let noise_metadata_schedule_250_0_e2415: f64 = (w[259] * w[8]);
        let noise_metadata_schedule_250_0_e2416: f64 = (noise_metadata_schedule_250_0_e2415).exp();
        (noise_metadata_schedule_250_0_e2416,)
    } else {
        (w[262],)
    }
};
            w[262] = noise_metadata_schedule_250_0_e2418;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_251_0_e2424,) = {
    if (w[520] == 0.0) {
        let noise_metadata_schedule_251_0_e2422: f64 = (params.p151).exp();
        (noise_metadata_schedule_251_0_e2422,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_251_0_e2424;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_252_0_e2437,) = {
    if (w[520] == 0.0) {
        let noise_metadata_schedule_252_0_e2431: f64 = (w[259] * w[8]);
        let noise_metadata_schedule_252_0_e2433: f64 = (noise_metadata_schedule_252_0_e2431 - params.p151);
        let noise_metadata_schedule_252_0_e2434: f64 = (1.0 + noise_metadata_schedule_252_0_e2433);
        let noise_metadata_schedule_252_0_e2435: f64 = (w[301] * noise_metadata_schedule_252_0_e2434);
        (noise_metadata_schedule_252_0_e2435,)
    } else {
        (w[262],)
    }
};
            w[262] = noise_metadata_schedule_252_0_e2437;
        }
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_253_0_e2440: f64 = (w[260] * w[8]);
            let noise_metadata_schedule_253_0_e2442: f64 = if noise_metadata_schedule_253_0_e2440 < params.p151 { 1.0 } else { 0.0 };
            w[521] = noise_metadata_schedule_253_0_e2442;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_254_0_e2449,) = {
    if (w[521] != 0.0) {
        let noise_metadata_schedule_254_0_e2446: f64 = (w[260] * w[8]);
        let noise_metadata_schedule_254_0_e2447: f64 = (noise_metadata_schedule_254_0_e2446).exp();
        (noise_metadata_schedule_254_0_e2447,)
    } else {
        (w[263],)
    }
};
            w[263] = noise_metadata_schedule_254_0_e2449;
        }
        if (active[0] & 0xdffe7) != 0 {
            let (noise_metadata_schedule_255_0_e2455,) = {
    if (w[521] == 0.0) {
        let noise_metadata_schedule_255_0_e2453: f64 = (params.p151).exp();
        (noise_metadata_schedule_255_0_e2453,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_255_0_e2455;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_256_0_e2468,) = {
    if (w[521] == 0.0) {
        let noise_metadata_schedule_256_0_e2462: f64 = (w[260] * w[8]);
        let noise_metadata_schedule_256_0_e2464: f64 = (noise_metadata_schedule_256_0_e2462 - params.p151);
        let noise_metadata_schedule_256_0_e2465: f64 = (1.0 + noise_metadata_schedule_256_0_e2464);
        let noise_metadata_schedule_256_0_e2466: f64 = (w[301] * noise_metadata_schedule_256_0_e2465);
        (noise_metadata_schedule_256_0_e2466,)
    } else {
        (w[263],)
    }
};
            w[263] = noise_metadata_schedule_256_0_e2468;
        }
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_257_0_e2471: f64 = (w[261] * w[8]);
            let noise_metadata_schedule_257_0_e2473: f64 = if noise_metadata_schedule_257_0_e2471 < params.p151 { 1.0 } else { 0.0 };
            w[522] = noise_metadata_schedule_257_0_e2473;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_258_0_e2480,) = {
    if (w[522] != 0.0) {
        let noise_metadata_schedule_258_0_e2477: f64 = (w[261] * w[8]);
        let noise_metadata_schedule_258_0_e2478: f64 = (noise_metadata_schedule_258_0_e2477).exp();
        (noise_metadata_schedule_258_0_e2478,)
    } else {
        (w[264],)
    }
};
            w[264] = noise_metadata_schedule_258_0_e2480;
        }
        if (active[0] & 0xdffe7) != 0 {
            let (noise_metadata_schedule_259_0_e2486,) = {
    if (w[522] == 0.0) {
        let noise_metadata_schedule_259_0_e2484: f64 = (params.p151).exp();
        (noise_metadata_schedule_259_0_e2484,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_259_0_e2486;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_260_0_e2499,) = {
    if (w[522] == 0.0) {
        let noise_metadata_schedule_260_0_e2493: f64 = (w[261] * w[8]);
        let noise_metadata_schedule_260_0_e2495: f64 = (noise_metadata_schedule_260_0_e2493 - params.p151);
        let noise_metadata_schedule_260_0_e2496: f64 = (1.0 + noise_metadata_schedule_260_0_e2495);
        let noise_metadata_schedule_260_0_e2497: f64 = (w[301] * noise_metadata_schedule_260_0_e2496);
        (noise_metadata_schedule_260_0_e2497,)
    } else {
        (w[264],)
    }
};
            w[264] = noise_metadata_schedule_260_0_e2499;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_261_0_e2502: f64 = (w[267] - w[16]);
            let noise_metadata_schedule_261_0_e2504: f64 = (noise_metadata_schedule_261_0_e2502 * w[8]);
            let noise_metadata_schedule_261_0_e2506: f64 = if noise_metadata_schedule_261_0_e2504 < params.p151 { 1.0 } else { 0.0 };
            w[523] = noise_metadata_schedule_261_0_e2506;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_263_0_e2521,) = {
    if (w[523] == 0.0) {
        let noise_metadata_schedule_263_0_e2519: f64 = (params.p151).exp();
        (noise_metadata_schedule_263_0_e2519,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_263_0_e2521;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_265_0_e2539: f64 = (w[255] - w[16]);
            let noise_metadata_schedule_265_0_e2541: f64 = (noise_metadata_schedule_265_0_e2539 * w[8]);
            let noise_metadata_schedule_265_0_e2543: f64 = if noise_metadata_schedule_265_0_e2541 < params.p151 { 1.0 } else { 0.0 };
            w[524] = noise_metadata_schedule_265_0_e2543;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_267_0_e2558,) = {
    if (w[524] == 0.0) {
        let noise_metadata_schedule_267_0_e2556: f64 = (params.p151).exp();
        (noise_metadata_schedule_267_0_e2556,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_267_0_e2558;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_269_0_e2576: f64 = (w[251] - w[16]);
            let noise_metadata_schedule_269_0_e2578: f64 = (noise_metadata_schedule_269_0_e2576 * w[8]);
            let noise_metadata_schedule_269_0_e2580: f64 = if noise_metadata_schedule_269_0_e2578 < params.p151 { 1.0 } else { 0.0 };
            w[525] = noise_metadata_schedule_269_0_e2580;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_270_0_e2589,) = {
    if (w[525] != 0.0) {
        let noise_metadata_schedule_270_0_e2584: f64 = (w[251] - w[16]);
        let noise_metadata_schedule_270_0_e2586: f64 = (noise_metadata_schedule_270_0_e2584 * w[8]);
        let noise_metadata_schedule_270_0_e2587: f64 = (noise_metadata_schedule_270_0_e2586).exp();
        (noise_metadata_schedule_270_0_e2587,)
    } else {
        (w[277],)
    }
};
            w[277] = noise_metadata_schedule_270_0_e2589;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_271_0_e2595,) = {
    if (w[525] == 0.0) {
        let noise_metadata_schedule_271_0_e2593: f64 = (params.p151).exp();
        (noise_metadata_schedule_271_0_e2593,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_271_0_e2595;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_272_0_e2610,) = {
    if (w[525] == 0.0) {
        let noise_metadata_schedule_272_0_e2602: f64 = (w[251] - w[16]);
        let noise_metadata_schedule_272_0_e2604: f64 = (noise_metadata_schedule_272_0_e2602 * w[8]);
        let noise_metadata_schedule_272_0_e2606: f64 = (noise_metadata_schedule_272_0_e2604 - params.p151);
        let noise_metadata_schedule_272_0_e2607: f64 = (1.0 + noise_metadata_schedule_272_0_e2606);
        let noise_metadata_schedule_272_0_e2608: f64 = (w[301] * noise_metadata_schedule_272_0_e2607);
        (noise_metadata_schedule_272_0_e2608,)
    } else {
        (w[277],)
    }
};
            w[277] = noise_metadata_schedule_272_0_e2610;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_273_0_e2613: f64 = (w[250] - w[16]);
            let noise_metadata_schedule_273_0_e2615: f64 = (noise_metadata_schedule_273_0_e2613 * w[8]);
            let noise_metadata_schedule_273_0_e2617: f64 = if noise_metadata_schedule_273_0_e2615 < params.p151 { 1.0 } else { 0.0 };
            w[526] = noise_metadata_schedule_273_0_e2617;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_274_0_e2626,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_274_0_e2621: f64 = (w[250] - w[16]);
        let noise_metadata_schedule_274_0_e2623: f64 = (noise_metadata_schedule_274_0_e2621 * w[8]);
        let noise_metadata_schedule_274_0_e2624: f64 = (noise_metadata_schedule_274_0_e2623).exp();
        (noise_metadata_schedule_274_0_e2624,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_274_0_e2626;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_275_0_e2632,) = {
    if (w[526] == 0.0) {
        let noise_metadata_schedule_275_0_e2630: f64 = (params.p151).exp();
        (noise_metadata_schedule_275_0_e2630,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_275_0_e2632;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_276_0_e2647,) = {
    if (w[526] == 0.0) {
        let noise_metadata_schedule_276_0_e2639: f64 = (w[250] - w[16]);
        let noise_metadata_schedule_276_0_e2641: f64 = (noise_metadata_schedule_276_0_e2639 * w[8]);
        let noise_metadata_schedule_276_0_e2643: f64 = (noise_metadata_schedule_276_0_e2641 - params.p151);
        let noise_metadata_schedule_276_0_e2644: f64 = (1.0 + noise_metadata_schedule_276_0_e2643);
        let noise_metadata_schedule_276_0_e2645: f64 = (w[301] * noise_metadata_schedule_276_0_e2644);
        (noise_metadata_schedule_276_0_e2645,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_276_0_e2647;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_277_0_e2651: f64 = (4.0 * w[277]);
            let noise_metadata_schedule_277_0_e2652: f64 = (1.0 + noise_metadata_schedule_277_0_e2651);
            let noise_metadata_schedule_277_0_e2653: f64 = (noise_metadata_schedule_277_0_e2652).sqrt();
            w[114] = noise_metadata_schedule_277_0_e2653;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_278_0_e2657: f64 = (4.0 * w[279]);
            let noise_metadata_schedule_278_0_e2658: f64 = (1.0 + noise_metadata_schedule_278_0_e2657);
            let noise_metadata_schedule_278_0_e2659: f64 = (noise_metadata_schedule_278_0_e2658).sqrt();
            w[115] = noise_metadata_schedule_278_0_e2659;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_279_0_e2662: f64 = (2.0 * w[279]);
            let noise_metadata_schedule_279_0_e2665: f64 = (1.0 + w[115]);
            let noise_metadata_schedule_279_0_e2666: f64 = (noise_metadata_schedule_279_0_e2662 / noise_metadata_schedule_279_0_e2665);
            w[116] = noise_metadata_schedule_279_0_e2666;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_280_0_e2669: f64 = if w[116] < params.p153 { 1.0 } else { 0.0 };
            w[527] = noise_metadata_schedule_280_0_e2669;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_281_0_e2673,) = {
    if (w[527] != 0.0) {
        (params.p153,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_281_0_e2673;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_282_0_e2677: f64 = (w[114] - w[115]);
            let noise_metadata_schedule_282_0_e2680: f64 = (w[114] + 1.0);
            let noise_metadata_schedule_282_0_e2683: f64 = (w[115] + 1.0);
            let noise_metadata_schedule_282_0_e2684: f64 = (noise_metadata_schedule_282_0_e2680 / noise_metadata_schedule_282_0_e2683);
            let noise_metadata_schedule_282_0_e2685: f64 = (noise_metadata_schedule_282_0_e2684).ln();
            let noise_metadata_schedule_282_0_e2686: f64 = (noise_metadata_schedule_282_0_e2677 - noise_metadata_schedule_282_0_e2685);
            let noise_metadata_schedule_282_0_e2687: f64 = (w[6] * noise_metadata_schedule_282_0_e2686);
            w[117] = noise_metadata_schedule_282_0_e2687;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_283_0_e2690: f64 = (w[117] + w[256]);
            let noise_metadata_schedule_283_0_e2692: f64 = (noise_metadata_schedule_283_0_e2690 / w[31]);
            w[118] = noise_metadata_schedule_283_0_e2692;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_284_0_e2695: f64 = if w[118] > 0.0 { 1.0 } else { 0.0 };
            w[528] = noise_metadata_schedule_284_0_e2695;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_285_0_e2698: f64 = if w[250] < 100.0 { 1.0 } else { 0.0 };
            w[529] = noise_metadata_schedule_285_0_e2698;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_286_0_e2704,) = {
    if ((w[528] != 0.0) && (w[529] != 0.0)) {
        (w[250],)
    } else {
        (w[303],)
    }
};
            w[303] = noise_metadata_schedule_286_0_e2704;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_287_0_e2718,) = {
    if ((w[528] != 0.0) && (w[529] == 0.0)) {
        let noise_metadata_schedule_287_0_e2713: f64 = (w[250] - 100.0);
        let noise_metadata_schedule_287_0_e2714: f64 = (1.0 + noise_metadata_schedule_287_0_e2713);
        let noise_metadata_schedule_287_0_e2715: f64 = (noise_metadata_schedule_287_0_e2714).ln();
        let noise_metadata_schedule_287_0_e2716: f64 = (100.0 + noise_metadata_schedule_287_0_e2715);
        (noise_metadata_schedule_287_0_e2716,)
    } else {
        (w[303],)
    }
};
            w[303] = noise_metadata_schedule_287_0_e2718;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_288_0_e2739,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_288_0_e2723: f64 = (2.0 * w[6]);
        let noise_metadata_schedule_288_0_e2726: f64 = (0.5 * w[118]);
        let noise_metadata_schedule_288_0_e2728: f64 = (noise_metadata_schedule_288_0_e2726 * w[31]);
        let noise_metadata_schedule_288_0_e2730: f64 = (noise_metadata_schedule_288_0_e2728 * w[8]);
        let noise_metadata_schedule_288_0_e2732: f64 = (noise_metadata_schedule_288_0_e2730 + 1.0);
        let noise_metadata_schedule_288_0_e2733: f64 = (noise_metadata_schedule_288_0_e2732).ln();
        let noise_metadata_schedule_288_0_e2734: f64 = (noise_metadata_schedule_288_0_e2723 * noise_metadata_schedule_288_0_e2733);
        let noise_metadata_schedule_288_0_e2735: f64 = (w[16] + noise_metadata_schedule_288_0_e2734);
        let noise_metadata_schedule_288_0_e2737: f64 = (noise_metadata_schedule_288_0_e2735 - w[303]);
        (noise_metadata_schedule_288_0_e2737,)
    } else {
        (w[119],)
    }
};
            w[119] = noise_metadata_schedule_288_0_e2739;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_289_0_e2745,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_289_0_e2743: f64 = (0.2 * w[16]);
        (noise_metadata_schedule_289_0_e2743,)
    } else {
        (w[298],)
    }
};
            w[298] = noise_metadata_schedule_289_0_e2745;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_290_0_e2751,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_290_0_e2749: f64 = (w[298] * w[298]);
        (noise_metadata_schedule_290_0_e2749,)
    } else {
        (w[287],)
    }
};
            w[287] = noise_metadata_schedule_290_0_e2751;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_291_0_e2757,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_291_0_e2755: f64 = (w[119] * w[119]);
        (noise_metadata_schedule_291_0_e2755,)
    } else {
        (w[288],)
    }
};
            w[288] = noise_metadata_schedule_291_0_e2757;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_292_0_e2760: f64 = if w[119] < 0.0 { 1.0 } else { 0.0 };
            w[530] = noise_metadata_schedule_292_0_e2760;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_293_0_e2775,) = {
    if ((w[528] != 0.0) && (w[530] != 0.0)) {
        let noise_metadata_schedule_293_0_e2766: f64 = (0.5 * w[287]);
        let noise_metadata_schedule_293_0_e2769: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_293_0_e2770: f64 = (noise_metadata_schedule_293_0_e2769).sqrt();
        let noise_metadata_schedule_293_0_e2772: f64 = (noise_metadata_schedule_293_0_e2770 - w[119]);
        let noise_metadata_schedule_293_0_e2773: f64 = (noise_metadata_schedule_293_0_e2766 / noise_metadata_schedule_293_0_e2772);
        (noise_metadata_schedule_293_0_e2773,)
    } else {
        (w[120],)
    }
};
            w[120] = noise_metadata_schedule_293_0_e2775;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_294_0_e2789,) = {
    if ((w[528] != 0.0) && (w[530] == 0.0)) {
        let noise_metadata_schedule_294_0_e2783: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_294_0_e2784: f64 = (noise_metadata_schedule_294_0_e2783).sqrt();
        let noise_metadata_schedule_294_0_e2786: f64 = (noise_metadata_schedule_294_0_e2784 + w[119]);
        let noise_metadata_schedule_294_0_e2787: f64 = (0.5 * noise_metadata_schedule_294_0_e2786);
        (noise_metadata_schedule_294_0_e2787,)
    } else {
        (w[120],)
    }
};
            w[120] = noise_metadata_schedule_294_0_e2789;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_295_0_e2807,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_295_0_e2795: f64 = (params.p62 * params.p61);
        let noise_metadata_schedule_295_0_e2796: f64 = (w[120] + noise_metadata_schedule_295_0_e2795);
        let noise_metadata_schedule_295_0_e2797: f64 = (w[120] * noise_metadata_schedule_295_0_e2796);
        let noise_metadata_schedule_295_0_e2802: f64 = (params.p62 * w[31]);
        let noise_metadata_schedule_295_0_e2803: f64 = (w[120] + noise_metadata_schedule_295_0_e2802);
        let noise_metadata_schedule_295_0_e2804: f64 = (params.p61 * noise_metadata_schedule_295_0_e2803);
        let noise_metadata_schedule_295_0_e2805: f64 = (noise_metadata_schedule_295_0_e2797 / noise_metadata_schedule_295_0_e2804);
        (noise_metadata_schedule_295_0_e2805,)
    } else {
        (w[121],)
    }
};
            w[121] = noise_metadata_schedule_295_0_e2807;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_296_0_e2813,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_296_0_e2811: f64 = (w[118] / w[121]);
        (noise_metadata_schedule_296_0_e2811,)
    } else {
        (w[291],)
    }
};
            w[291] = noise_metadata_schedule_296_0_e2813;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_297_0_e2821,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_297_0_e2817: f64 = (w[291] - 1.0);
        let noise_metadata_schedule_297_0_e2819: f64 = (noise_metadata_schedule_297_0_e2817 / params.p63);
        (noise_metadata_schedule_297_0_e2819,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_297_0_e2821;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_298_0_e2824: f64 = if w[291] < 1.0 { 1.0 } else { 0.0 };
            w[531] = noise_metadata_schedule_298_0_e2824;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_299_0_e2838,) = {
    if ((w[528] != 0.0) && (w[531] != 0.0)) {
        let noise_metadata_schedule_299_0_e2832: f64 = (w[285]).exp();
        let noise_metadata_schedule_299_0_e2833: f64 = (1.0 + noise_metadata_schedule_299_0_e2832);
        let noise_metadata_schedule_299_0_e2834: f64 = (noise_metadata_schedule_299_0_e2833).ln();
        let noise_metadata_schedule_299_0_e2835: f64 = (params.p63 * noise_metadata_schedule_299_0_e2834);
        let noise_metadata_schedule_299_0_e2836: f64 = (1.0 + noise_metadata_schedule_299_0_e2835);
        (noise_metadata_schedule_299_0_e2836,)
    } else {
        (w[289],)
    }
};
            w[289] = noise_metadata_schedule_299_0_e2838;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_300_0_e2854,) = {
    if ((w[528] != 0.0) && (w[531] == 0.0)) {
        let noise_metadata_schedule_300_0_e2847: f64 = (-w[285]);
        let noise_metadata_schedule_300_0_e2848: f64 = (noise_metadata_schedule_300_0_e2847).exp();
        let noise_metadata_schedule_300_0_e2849: f64 = (1.0 + noise_metadata_schedule_300_0_e2848);
        let noise_metadata_schedule_300_0_e2850: f64 = (noise_metadata_schedule_300_0_e2849).ln();
        let noise_metadata_schedule_300_0_e2851: f64 = (params.p63 * noise_metadata_schedule_300_0_e2850);
        let noise_metadata_schedule_300_0_e2852: f64 = (w[291] + noise_metadata_schedule_300_0_e2851);
        (noise_metadata_schedule_300_0_e2852,)
    } else {
        (w[289],)
    }
};
            w[289] = noise_metadata_schedule_300_0_e2854;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_301_0_e2871,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_301_0_e2861: f64 = (-1.0);
        let noise_metadata_schedule_301_0_e2863: f64 = (noise_metadata_schedule_301_0_e2861 / params.p63);
        let noise_metadata_schedule_301_0_e2864: f64 = (noise_metadata_schedule_301_0_e2863).exp();
        let noise_metadata_schedule_301_0_e2865: f64 = (1.0 + noise_metadata_schedule_301_0_e2864);
        let noise_metadata_schedule_301_0_e2866: f64 = (noise_metadata_schedule_301_0_e2865).ln();
        let noise_metadata_schedule_301_0_e2867: f64 = (params.p63 * noise_metadata_schedule_301_0_e2866);
        let noise_metadata_schedule_301_0_e2868: f64 = (1.0 + noise_metadata_schedule_301_0_e2867);
        let noise_metadata_schedule_301_0_e2869: f64 = (w[289] / noise_metadata_schedule_301_0_e2868);
        (noise_metadata_schedule_301_0_e2869,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_301_0_e2871;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_302_0_e2879,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_302_0_e2876: f64 = (params.p62 * params.p61);
        let noise_metadata_schedule_302_0_e2877: f64 = (w[120] / noise_metadata_schedule_302_0_e2876);
        (noise_metadata_schedule_302_0_e2877,)
    } else {
        (w[123],)
    }
};
            w[123] = noise_metadata_schedule_302_0_e2879;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_303_0_e2904,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_303_0_e2885: f64 = (4.0 * w[122]);
        let noise_metadata_schedule_303_0_e2887: f64 = (noise_metadata_schedule_303_0_e2885 * w[123]);
        let noise_metadata_schedule_303_0_e2890: f64 = (1.0 + w[123]);
        let noise_metadata_schedule_303_0_e2891: f64 = (noise_metadata_schedule_303_0_e2887 * noise_metadata_schedule_303_0_e2890);
        let noise_metadata_schedule_303_0_e2892: f64 = (1.0 + noise_metadata_schedule_303_0_e2891);
        let noise_metadata_schedule_303_0_e2893: f64 = (noise_metadata_schedule_303_0_e2892).sqrt();
        let noise_metadata_schedule_303_0_e2894: f64 = (1.0 + noise_metadata_schedule_303_0_e2893);
        let noise_metadata_schedule_303_0_e2897: f64 = (2.0 * w[122]);
        let noise_metadata_schedule_303_0_e2900: f64 = (1.0 + w[123]);
        let noise_metadata_schedule_303_0_e2901: f64 = (noise_metadata_schedule_303_0_e2897 * noise_metadata_schedule_303_0_e2900);
        let noise_metadata_schedule_303_0_e2902: f64 = (noise_metadata_schedule_303_0_e2894 / noise_metadata_schedule_303_0_e2901);
        (noise_metadata_schedule_303_0_e2902,)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_303_0_e2904;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_304_0_e2920,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_304_0_e2908: f64 = (1.0 - w[124]);
        let noise_metadata_schedule_304_0_e2911: f64 = (w[116] * w[124]);
        let noise_metadata_schedule_304_0_e2912: f64 = (noise_metadata_schedule_304_0_e2908 + noise_metadata_schedule_304_0_e2911);
        let noise_metadata_schedule_304_0_e2916: f64 = (w[116] * w[124]);
        let noise_metadata_schedule_304_0_e2917: f64 = (1.0 + noise_metadata_schedule_304_0_e2916);
        let noise_metadata_schedule_304_0_e2918: f64 = (noise_metadata_schedule_304_0_e2912 / noise_metadata_schedule_304_0_e2917);
        (noise_metadata_schedule_304_0_e2918,)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_304_0_e2920;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_305_0_e2932,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_305_0_e2924: f64 = (0.5 * w[118]);
        let noise_metadata_schedule_305_0_e2926: f64 = (noise_metadata_schedule_305_0_e2924 * w[31]);
        let noise_metadata_schedule_305_0_e2928: f64 = (noise_metadata_schedule_305_0_e2926 * w[125]);
        let noise_metadata_schedule_305_0_e2930: f64 = (noise_metadata_schedule_305_0_e2928 * w[8]);
        (noise_metadata_schedule_305_0_e2930,)
    } else {
        (w[127],)
    }
};
            w[127] = noise_metadata_schedule_305_0_e2932;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_306_0_e2946,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_306_0_e2936: f64 = (2.0 * w[127]);
        let noise_metadata_schedule_306_0_e2940: f64 = (w[116] + w[127]);
        let noise_metadata_schedule_306_0_e2942: f64 = (noise_metadata_schedule_306_0_e2940 + 1.0);
        let noise_metadata_schedule_306_0_e2943: f64 = (w[116] * noise_metadata_schedule_306_0_e2942);
        let noise_metadata_schedule_306_0_e2944: f64 = (noise_metadata_schedule_306_0_e2936 + noise_metadata_schedule_306_0_e2943);
        (noise_metadata_schedule_306_0_e2944,)
    } else {
        (w[292],)
    }
};
            w[292] = noise_metadata_schedule_306_0_e2946;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_307_0_e2954,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_307_0_e2951: f64 = (w[127] - 1.0);
        let noise_metadata_schedule_307_0_e2952: f64 = (0.5 * noise_metadata_schedule_307_0_e2951);
        (noise_metadata_schedule_307_0_e2952,)
    } else {
        (w[128],)
    }
};
            w[128] = noise_metadata_schedule_307_0_e2954;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_308_0_e2962,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_308_0_e2958: f64 = (w[128] * w[128]);
        let noise_metadata_schedule_308_0_e2960: f64 = (noise_metadata_schedule_308_0_e2958 + w[292]);
        (noise_metadata_schedule_308_0_e2960,)
    } else {
        (w[286],)
    }
};
            w[286] = noise_metadata_schedule_308_0_e2962;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_309_0_e2965: f64 = if w[127] >= 1.0 { 1.0 } else { 0.0 };
            w[532] = noise_metadata_schedule_309_0_e2965;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_310_0_e2974,) = {
    if ((w[528] != 0.0) && (w[532] != 0.0)) {
        let noise_metadata_schedule_310_0_e2971: f64 = (w[286]).sqrt();
        let noise_metadata_schedule_310_0_e2972: f64 = (w[128] + noise_metadata_schedule_310_0_e2971);
        (noise_metadata_schedule_310_0_e2972,)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_310_0_e2974;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_311_0_e2986,) = {
    if ((w[528] != 0.0) && (w[532] == 0.0)) {
        let noise_metadata_schedule_311_0_e2981: f64 = (w[286]).sqrt();
        let noise_metadata_schedule_311_0_e2983: f64 = (noise_metadata_schedule_311_0_e2981 - w[128]);
        let noise_metadata_schedule_311_0_e2984: f64 = (w[292] / noise_metadata_schedule_311_0_e2983);
        (noise_metadata_schedule_311_0_e2984,)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_311_0_e2986;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_312_0_e2989: f64 = if w[129] < params.p152 { 1.0 } else { 0.0 };
            w[533] = noise_metadata_schedule_312_0_e2989;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_313_0_e2995,) = {
    if ((w[528] != 0.0) && (w[533] != 0.0)) {
        (params.p152,)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_313_0_e2995;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_314_0_e3008,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_314_0_e3000: f64 = (w[129] + 1.0);
        let noise_metadata_schedule_314_0_e3001: f64 = (w[129] * noise_metadata_schedule_314_0_e3000);
        let noise_metadata_schedule_314_0_e3004: f64 = (w[16] * w[8]);
        let noise_metadata_schedule_314_0_e3005: f64 = (noise_metadata_schedule_314_0_e3004).exp();
        let noise_metadata_schedule_314_0_e3006: f64 = (noise_metadata_schedule_314_0_e3001 * noise_metadata_schedule_314_0_e3005);
        (noise_metadata_schedule_314_0_e3006,)
    } else {
        (w[131],)
    }
};
            w[131] = noise_metadata_schedule_314_0_e3008;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_315_0_e3018,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_315_0_e3012: f64 = (0.5 * params.p61);
        let noise_metadata_schedule_315_0_e3015: f64 = (w[118] - params.p62);
        let noise_metadata_schedule_315_0_e3016: f64 = (noise_metadata_schedule_315_0_e3012 * noise_metadata_schedule_315_0_e3015);
        (noise_metadata_schedule_315_0_e3016,)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_315_0_e3018;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_316_0_e3028,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_316_0_e3022: f64 = (params.p61 * w[31]);
        let noise_metadata_schedule_316_0_e3024: f64 = (noise_metadata_schedule_316_0_e3022 * params.p62);
        let noise_metadata_schedule_316_0_e3026: f64 = (noise_metadata_schedule_316_0_e3024 * w[118]);
        (noise_metadata_schedule_316_0_e3026,)
    } else {
        (w[134],)
    }
};
            w[134] = noise_metadata_schedule_316_0_e3028;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_317_0_e3039,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_317_0_e3033: f64 = (w[133] * w[133]);
        let noise_metadata_schedule_317_0_e3035: f64 = (noise_metadata_schedule_317_0_e3033 + w[134]);
        let noise_metadata_schedule_317_0_e3036: f64 = (noise_metadata_schedule_317_0_e3035).sqrt();
        let noise_metadata_schedule_317_0_e3037: f64 = (w[133] + noise_metadata_schedule_317_0_e3036);
        (noise_metadata_schedule_317_0_e3037,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_317_0_e3039;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_318_0_e3042: f64 = if params.p73 == 0.0 { 1.0 } else { 0.0 };
            w[534] = noise_metadata_schedule_318_0_e3042;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_319_0_e3050,) = {
    if ((w[528] != 0.0) && (w[534] != 0.0)) {
        let noise_metadata_schedule_319_0_e3048: f64 = (w[17] * 0.1);
        (noise_metadata_schedule_319_0_e3048,)
    } else {
        (w[136],)
    }
};
            w[136] = noise_metadata_schedule_319_0_e3050;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_320_0_e3067,) = {
    if ((w[528] != 0.0) && (w[534] == 0.0)) {
        let noise_metadata_schedule_320_0_e3059: f64 = (2.0 * w[118]);
        let noise_metadata_schedule_320_0_e3062: f64 = (w[118] + w[121]);
        let noise_metadata_schedule_320_0_e3063: f64 = (noise_metadata_schedule_320_0_e3059 / noise_metadata_schedule_320_0_e3062);
        let noise_metadata_schedule_320_0_e3064: f64 = (0.1 + noise_metadata_schedule_320_0_e3063);
        let noise_metadata_schedule_320_0_e3065: f64 = (w[17] * noise_metadata_schedule_320_0_e3064);
        (noise_metadata_schedule_320_0_e3065,)
    } else {
        (w[136],)
    }
};
            w[136] = noise_metadata_schedule_320_0_e3067;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_321_0_e3077,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_321_0_e3071: f64 = (params.p62 * w[118]);
        let noise_metadata_schedule_321_0_e3074: f64 = (params.p62 + w[118]);
        let noise_metadata_schedule_321_0_e3075: f64 = (noise_metadata_schedule_321_0_e3071 / noise_metadata_schedule_321_0_e3074);
        (noise_metadata_schedule_321_0_e3075,)
    } else {
        (w[137],)
    }
};
            w[137] = noise_metadata_schedule_321_0_e3077;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_322_0_e3085,) = {
    if (w[528] != 0.0) {
        let noise_metadata_schedule_322_0_e3082: f64 = (params.p62 + w[118]);
        let noise_metadata_schedule_322_0_e3083: f64 = (params.p62 / noise_metadata_schedule_322_0_e3082);
        (noise_metadata_schedule_322_0_e3083,)
    } else {
        (w[213],)
    }
};
            w[213] = noise_metadata_schedule_322_0_e3085;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_324_0_e3101,) = {
    if (w[528] == 0.0) {
        let noise_metadata_schedule_324_0_e3095: f64 = (2.0 * w[277]);
        let noise_metadata_schedule_324_0_e3098: f64 = (1.0 + w[114]);
        let noise_metadata_schedule_324_0_e3099: f64 = (noise_metadata_schedule_324_0_e3095 / noise_metadata_schedule_324_0_e3098);
        (noise_metadata_schedule_324_0_e3099,)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_324_0_e3101;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_325_0_e3106,) = {
    if (w[528] == 0.0) {
        (w[271],)
    } else {
        (w[131],)
    }
};
            w[131] = noise_metadata_schedule_325_0_e3106;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_326_0_e3108: f64 = (w[256]).abs();
            let noise_metadata_schedule_326_0_e3111: f64 = (1e-5 * w[6]);
            let noise_metadata_schedule_326_0_e3114: f64 = (w[117]).abs();
            let noise_metadata_schedule_326_0_e3117: f64 = (1e-40 * w[6]);
            let noise_metadata_schedule_326_0_e3120: f64 = (w[114] + w[115]);
            let noise_metadata_schedule_326_0_e3121: f64 = (noise_metadata_schedule_326_0_e3117 * noise_metadata_schedule_326_0_e3120);
            let noise_metadata_schedule_326_0_e3123: f64 = if ((noise_metadata_schedule_326_0_e3108 < noise_metadata_schedule_326_0_e3111) || (noise_metadata_schedule_326_0_e3114 < noise_metadata_schedule_326_0_e3121)) { 1.0 } else { 0.0 };
            w[535] = noise_metadata_schedule_326_0_e3123;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_327_0_e3134,) = {
    if ((w[528] == 0.0) && (w[535] != 0.0)) {
        let noise_metadata_schedule_327_0_e3131: f64 = (w[129] + w[116]);
        let noise_metadata_schedule_327_0_e3132: f64 = (0.5 * noise_metadata_schedule_327_0_e3131);
        (noise_metadata_schedule_327_0_e3132,)
    } else {
        (w[138],)
    }
};
            w[138] = noise_metadata_schedule_327_0_e3134;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_328_0_e3145,) = {
    if ((w[528] == 0.0) && (w[535] != 0.0)) {
        let noise_metadata_schedule_328_0_e3142: f64 = (w[138] + 1.0);
        let noise_metadata_schedule_328_0_e3143: f64 = (w[138] / noise_metadata_schedule_328_0_e3142);
        (noise_metadata_schedule_328_0_e3143,)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_328_0_e3145;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_329_0_e3159,) = {
    if ((w[528] == 0.0) && (w[535] == 0.0)) {
        let noise_metadata_schedule_329_0_e3154: f64 = (w[117] + w[251]);
        let noise_metadata_schedule_329_0_e3156: f64 = (noise_metadata_schedule_329_0_e3154 - w[250]);
        let noise_metadata_schedule_329_0_e3157: f64 = (w[117] / noise_metadata_schedule_329_0_e3156);
        (noise_metadata_schedule_329_0_e3157,)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_329_0_e3159;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_330_0_e3164,) = {
    if (w[528] == 0.0) {
        (w[256],)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_330_0_e3164;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_331_0_e3171,) = {
    if (w[528] == 0.0) {
        let noise_metadata_schedule_331_0_e3169: f64 = (0.1 * w[17]);
        (noise_metadata_schedule_331_0_e3169,)
    } else {
        (w[136],)
    }
};
            w[136] = noise_metadata_schedule_331_0_e3171;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_332_0_e3176,) = {
    if (w[528] == 0.0) {
        (w[118],)
    } else {
        (w[137],)
    }
};
            w[137] = noise_metadata_schedule_332_0_e3176;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_333_0_e3185,) = {
    if (w[528] == 0.0) {
        let noise_metadata_schedule_333_0_e3182: f64 = (w[137] / params.p62);
        let noise_metadata_schedule_333_0_e3183: f64 = (1.0 - noise_metadata_schedule_333_0_e3182);
        (noise_metadata_schedule_333_0_e3183,)
    } else {
        (w[213],)
    }
};
            w[213] = noise_metadata_schedule_333_0_e3185;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_334_0_e3190: f64 = (-1.0);
            let noise_metadata_schedule_334_0_e3192: f64 = (noise_metadata_schedule_334_0_e3190 / params.p67);
            let noise_metadata_schedule_334_0_e3193: f64 = (3.0_f64).powf(noise_metadata_schedule_334_0_e3192);
            let noise_metadata_schedule_334_0_e3194: f64 = (1.0 - noise_metadata_schedule_334_0_e3193);
            let noise_metadata_schedule_334_0_e3195: f64 = (w[14] * noise_metadata_schedule_334_0_e3194);
            w[139] = noise_metadata_schedule_334_0_e3195;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_335_0_e3198: f64 = (0.1 * w[14]);
            w[299] = noise_metadata_schedule_335_0_e3198;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_336_0_e3201: f64 = (w[252] - w[139]);
            let noise_metadata_schedule_336_0_e3203: f64 = (noise_metadata_schedule_336_0_e3201 / w[299]);
            w[285] = noise_metadata_schedule_336_0_e3203;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_337_0_e3206: f64 = if w[252] < w[139] { 1.0 } else { 0.0 };
            w[536] = noise_metadata_schedule_337_0_e3206;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_338_0_e3218,) = {
    if (w[536] != 0.0) {
        let noise_metadata_schedule_338_0_e3212: f64 = (w[285]).exp();
        let noise_metadata_schedule_338_0_e3213: f64 = (1.0 + noise_metadata_schedule_338_0_e3212);
        let noise_metadata_schedule_338_0_e3214: f64 = (noise_metadata_schedule_338_0_e3213).ln();
        let noise_metadata_schedule_338_0_e3215: f64 = (w[299] * noise_metadata_schedule_338_0_e3214);
        let noise_metadata_schedule_338_0_e3216: f64 = (w[252] - noise_metadata_schedule_338_0_e3215);
        (noise_metadata_schedule_338_0_e3216,)
    } else {
        (w[140],)
    }
};
            w[140] = noise_metadata_schedule_338_0_e3218;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_339_0_e3232,) = {
    if (w[536] == 0.0) {
        let noise_metadata_schedule_339_0_e3225: f64 = (-w[285]);
        let noise_metadata_schedule_339_0_e3226: f64 = (noise_metadata_schedule_339_0_e3225).exp();
        let noise_metadata_schedule_339_0_e3227: f64 = (1.0 + noise_metadata_schedule_339_0_e3226);
        let noise_metadata_schedule_339_0_e3228: f64 = (noise_metadata_schedule_339_0_e3227).ln();
        let noise_metadata_schedule_339_0_e3229: f64 = (w[299] * noise_metadata_schedule_339_0_e3228);
        let noise_metadata_schedule_339_0_e3230: f64 = (w[139] - noise_metadata_schedule_339_0_e3229);
        (noise_metadata_schedule_339_0_e3230,)
    } else {
        (w[140],)
    }
};
            w[140] = noise_metadata_schedule_339_0_e3232;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_340_0_e3236: f64 = (w[140] * w[65]);
            let noise_metadata_schedule_340_0_e3237: f64 = (1.0 - noise_metadata_schedule_340_0_e3236);
            let noise_metadata_schedule_340_0_e3240: f64 = (1.0 - params.p67);
            let noise_metadata_schedule_340_0_e3241: f64 = (noise_metadata_schedule_340_0_e3237).powf(noise_metadata_schedule_340_0_e3240);
            w[59] = noise_metadata_schedule_340_0_e3241;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_341_0_e3245: f64 = (1.0 - params.p67);
            let noise_metadata_schedule_341_0_e3246: f64 = (w[14] / noise_metadata_schedule_341_0_e3245);
            let noise_metadata_schedule_341_0_e3249: f64 = (1.0 - w[59]);
            let noise_metadata_schedule_341_0_e3250: f64 = (noise_metadata_schedule_341_0_e3246 * noise_metadata_schedule_341_0_e3249);
            let noise_metadata_schedule_341_0_e3254: f64 = (w[252] - w[140]);
            let noise_metadata_schedule_341_0_e3255: f64 = (3.0 * noise_metadata_schedule_341_0_e3254);
            let noise_metadata_schedule_341_0_e3256: f64 = (noise_metadata_schedule_341_0_e3250 + noise_metadata_schedule_341_0_e3255);
            w[141] = noise_metadata_schedule_341_0_e3256;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_342_0_e3259: f64 = if params.p74 == 1.0 { 1.0 } else { 0.0 };
            w[537] = noise_metadata_schedule_342_0_e3259;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_343_0_e3263,) = {
    if (w[537] != 0.0) {
        (w[250],)
    } else {
        (w[142],)
    }
};
            w[142] = noise_metadata_schedule_343_0_e3263;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_344_0_e3266: f64 = if params.p74 == 2.0 { 1.0 } else { 0.0 };
            w[538] = noise_metadata_schedule_344_0_e3266;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_345_0_e3275,) = {
    if ((w[537] == 0.0) && (w[538] != 0.0)) {
        let noise_metadata_schedule_345_0_e3273: f64 = (w[250] + w[135]);
        (noise_metadata_schedule_345_0_e3273,)
    } else {
        (w[142],)
    }
};
            w[142] = noise_metadata_schedule_345_0_e3275;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_346_0_e3283,) = {
    if ((w[537] == 0.0) && (w[538] == 0.0)) {
        (w[251],)
    } else {
        (w[142],)
    }
};
            w[142] = noise_metadata_schedule_346_0_e3283;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_347_0_e3286: f64 = (2.0 - w[25]);
            let noise_metadata_schedule_347_0_e3289: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_347_0_e3290: f64 = (noise_metadata_schedule_347_0_e3286 / noise_metadata_schedule_347_0_e3289);
            w[143] = noise_metadata_schedule_347_0_e3290;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_348_0_e3295: f64 = (-1.0);
            let noise_metadata_schedule_348_0_e3297: f64 = (noise_metadata_schedule_348_0_e3295 / params.p72);
            let noise_metadata_schedule_348_0_e3298: f64 = (w[143]).powf(noise_metadata_schedule_348_0_e3297);
            let noise_metadata_schedule_348_0_e3299: f64 = (1.0 - noise_metadata_schedule_348_0_e3298);
            let noise_metadata_schedule_348_0_e3300: f64 = (w[17] * noise_metadata_schedule_348_0_e3299);
            w[144] = noise_metadata_schedule_348_0_e3300;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_349_0_e3303: f64 = (w[142] - w[144]);
            let noise_metadata_schedule_349_0_e3305: f64 = (noise_metadata_schedule_349_0_e3303 / w[136]);
            w[285] = noise_metadata_schedule_349_0_e3305;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_350_0_e3308: f64 = if w[142] < w[144] { 1.0 } else { 0.0 };
            w[539] = noise_metadata_schedule_350_0_e3308;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_351_0_e3320,) = {
    if (w[539] != 0.0) {
        let noise_metadata_schedule_351_0_e3314: f64 = (w[285]).exp();
        let noise_metadata_schedule_351_0_e3315: f64 = (1.0 + noise_metadata_schedule_351_0_e3314);
        let noise_metadata_schedule_351_0_e3316: f64 = (noise_metadata_schedule_351_0_e3315).ln();
        let noise_metadata_schedule_351_0_e3317: f64 = (w[136] * noise_metadata_schedule_351_0_e3316);
        let noise_metadata_schedule_351_0_e3318: f64 = (w[142] - noise_metadata_schedule_351_0_e3317);
        (noise_metadata_schedule_351_0_e3318,)
    } else {
        (w[145],)
    }
};
            w[145] = noise_metadata_schedule_351_0_e3320;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_352_0_e3334,) = {
    if (w[539] == 0.0) {
        let noise_metadata_schedule_352_0_e3327: f64 = (-w[285]);
        let noise_metadata_schedule_352_0_e3328: f64 = (noise_metadata_schedule_352_0_e3327).exp();
        let noise_metadata_schedule_352_0_e3329: f64 = (1.0 + noise_metadata_schedule_352_0_e3328);
        let noise_metadata_schedule_352_0_e3330: f64 = (noise_metadata_schedule_352_0_e3329).ln();
        let noise_metadata_schedule_352_0_e3331: f64 = (w[136] * noise_metadata_schedule_352_0_e3330);
        let noise_metadata_schedule_352_0_e3332: f64 = (w[144] - noise_metadata_schedule_352_0_e3331);
        (noise_metadata_schedule_352_0_e3332,)
    } else {
        (w[145],)
    }
};
            w[145] = noise_metadata_schedule_352_0_e3334;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_353_0_e3337: f64 = (w[213]).powf(params.p76);
            w[146] = noise_metadata_schedule_353_0_e3337;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_354_0_e3341: f64 = (1.0 - params.p72);
            let noise_metadata_schedule_354_0_e3342: f64 = (w[17] / noise_metadata_schedule_354_0_e3341);
            let noise_metadata_schedule_354_0_e3348: f64 = (w[145] / w[17]);
            let noise_metadata_schedule_354_0_e3349: f64 = (1.0 - noise_metadata_schedule_354_0_e3348);
            let noise_metadata_schedule_354_0_e3352: f64 = (1.0 - params.p72);
            let noise_metadata_schedule_354_0_e3353: f64 = (noise_metadata_schedule_354_0_e3349).powf(noise_metadata_schedule_354_0_e3352);
            let noise_metadata_schedule_354_0_e3354: f64 = (w[146] * noise_metadata_schedule_354_0_e3353);
            let noise_metadata_schedule_354_0_e3355: f64 = (1.0 - noise_metadata_schedule_354_0_e3354);
            let noise_metadata_schedule_354_0_e3356: f64 = (noise_metadata_schedule_354_0_e3342 * noise_metadata_schedule_354_0_e3355);
            let noise_metadata_schedule_354_0_e3359: f64 = (w[146] * w[143]);
            let noise_metadata_schedule_354_0_e3362: f64 = (w[142] - w[145]);
            let noise_metadata_schedule_354_0_e3363: f64 = (noise_metadata_schedule_354_0_e3359 * noise_metadata_schedule_354_0_e3362);
            let noise_metadata_schedule_354_0_e3364: f64 = (noise_metadata_schedule_354_0_e3356 + noise_metadata_schedule_354_0_e3363);
            w[147] = noise_metadata_schedule_354_0_e3364;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_355_0_e3367: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_355_0_e3369: f64 = (noise_metadata_schedule_355_0_e3367 * w[147]);
            let noise_metadata_schedule_355_0_e3372: f64 = (w[25] * w[250]);
            let noise_metadata_schedule_355_0_e3373: f64 = (noise_metadata_schedule_355_0_e3369 + noise_metadata_schedule_355_0_e3372);
            w[148] = noise_metadata_schedule_355_0_e3373;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_356_0_e3376: f64 = (4.0 * w[35]);
            let noise_metadata_schedule_356_0_e3378: f64 = (noise_metadata_schedule_356_0_e3376 / w[36]);
            w[149] = noise_metadata_schedule_356_0_e3378;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_357_0_e3381: f64 = (w[149] * w[272]);
            w[150] = noise_metadata_schedule_357_0_e3381;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_358_0_e3386: f64 = (1.0 + w[150]);
            let noise_metadata_schedule_358_0_e3387: f64 = (noise_metadata_schedule_358_0_e3386).sqrt();
            let noise_metadata_schedule_358_0_e3388: f64 = (1.0 + noise_metadata_schedule_358_0_e3387);
            let noise_metadata_schedule_358_0_e3389: f64 = (w[150] / noise_metadata_schedule_358_0_e3388);
            w[152] = noise_metadata_schedule_358_0_e3389;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_359_0_e3393: f64 = (1.0 / w[49]);
            let noise_metadata_schedule_359_0_e3394: f64 = (w[131]).powf(noise_metadata_schedule_359_0_e3393);
            w[132] = noise_metadata_schedule_359_0_e3394;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_360_0_e3397: f64 = (w[149] * w[132]);
            w[151] = noise_metadata_schedule_360_0_e3397;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_361_0_e3402: f64 = (1.0 + w[151]);
            let noise_metadata_schedule_361_0_e3403: f64 = (noise_metadata_schedule_361_0_e3402).sqrt();
            let noise_metadata_schedule_361_0_e3404: f64 = (1.0 + noise_metadata_schedule_361_0_e3403);
            let noise_metadata_schedule_361_0_e3405: f64 = (w[151] / noise_metadata_schedule_361_0_e3404);
            w[153] = noise_metadata_schedule_361_0_e3405;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_362_0_e3408: f64 = if params.p92 == 0.0 { 1.0 } else { 0.0 };
            w[540] = noise_metadata_schedule_362_0_e3408;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_363_0_e3420,) = {
    if (w[540] != 0.0) {
        let noise_metadata_schedule_363_0_e3413: f64 = (w[141] / w[41]);
        let noise_metadata_schedule_363_0_e3414: f64 = (1.0 + noise_metadata_schedule_363_0_e3413);
        let noise_metadata_schedule_363_0_e3417: f64 = (w[148] / w[40]);
        let noise_metadata_schedule_363_0_e3418: f64 = (noise_metadata_schedule_363_0_e3414 + noise_metadata_schedule_363_0_e3417);
        (noise_metadata_schedule_363_0_e3418,)
    } else {
        (w[154],)
    }
};
            w[154] = noise_metadata_schedule_363_0_e3420;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_364_0_e3433,) = {
    if (w[540] == 0.0) {
        let noise_metadata_schedule_364_0_e3425: f64 = (w[141] / w[41]);
        let noise_metadata_schedule_364_0_e3427: f64 = (noise_metadata_schedule_364_0_e3425 + 1.0);
        let noise_metadata_schedule_364_0_e3429: f64 = (noise_metadata_schedule_364_0_e3427 * w[100]);
        let noise_metadata_schedule_364_0_e3431: f64 = (noise_metadata_schedule_364_0_e3429 * w[8]);
        (noise_metadata_schedule_364_0_e3431,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_364_0_e3433;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_365_0_e3445,) = {
    if (w[540] == 0.0) {
        let noise_metadata_schedule_365_0_e3437: f64 = (-w[148]);
        let noise_metadata_schedule_365_0_e3439: f64 = (noise_metadata_schedule_365_0_e3437 / w[40]);
        let noise_metadata_schedule_365_0_e3441: f64 = (noise_metadata_schedule_365_0_e3439 * w[100]);
        let noise_metadata_schedule_365_0_e3443: f64 = (noise_metadata_schedule_365_0_e3441 * w[8]);
        (noise_metadata_schedule_365_0_e3443,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_365_0_e3445;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_366_0_e3461,) = {
    if (w[540] == 0.0) {
        let noise_metadata_schedule_366_0_e3449: f64 = (w[295]).exp();
        let noise_metadata_schedule_366_0_e3451: f64 = (w[296]).exp();
        let noise_metadata_schedule_366_0_e3452: f64 = (noise_metadata_schedule_366_0_e3449 - noise_metadata_schedule_366_0_e3451);
        let noise_metadata_schedule_366_0_e3455: f64 = (w[100] * w[8]);
        let noise_metadata_schedule_366_0_e3456: f64 = (noise_metadata_schedule_366_0_e3455).exp();
        let noise_metadata_schedule_366_0_e3458: f64 = (noise_metadata_schedule_366_0_e3456 - 1.0);
        let noise_metadata_schedule_366_0_e3459: f64 = (noise_metadata_schedule_366_0_e3452 / noise_metadata_schedule_366_0_e3458);
        (noise_metadata_schedule_366_0_e3459,)
    } else {
        (w[154],)
    }
};
            w[154] = noise_metadata_schedule_366_0_e3461;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_367_0_e3464: f64 = (0.1 * 0.1);
            w[287] = noise_metadata_schedule_367_0_e3464;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_368_0_e3467: f64 = (w[154] * w[154]);
            w[288] = noise_metadata_schedule_368_0_e3467;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_369_0_e3470: f64 = if w[154] < 0.0 { 1.0 } else { 0.0 };
            w[541] = noise_metadata_schedule_369_0_e3470;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_370_0_e3483,) = {
    if (w[541] != 0.0) {
        let noise_metadata_schedule_370_0_e3474: f64 = (0.5 * w[287]);
        let noise_metadata_schedule_370_0_e3477: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_370_0_e3478: f64 = (noise_metadata_schedule_370_0_e3477).sqrt();
        let noise_metadata_schedule_370_0_e3480: f64 = (noise_metadata_schedule_370_0_e3478 - w[154]);
        let noise_metadata_schedule_370_0_e3481: f64 = (noise_metadata_schedule_370_0_e3474 / noise_metadata_schedule_370_0_e3480);
        (noise_metadata_schedule_370_0_e3481,)
    } else {
        (w[155],)
    }
};
            w[155] = noise_metadata_schedule_370_0_e3483;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_371_0_e3495,) = {
    if (w[541] == 0.0) {
        let noise_metadata_schedule_371_0_e3489: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_371_0_e3490: f64 = (noise_metadata_schedule_371_0_e3489).sqrt();
        let noise_metadata_schedule_371_0_e3492: f64 = (noise_metadata_schedule_371_0_e3490 + w[154]);
        let noise_metadata_schedule_371_0_e3493: f64 = (0.5 * noise_metadata_schedule_371_0_e3492);
        (noise_metadata_schedule_371_0_e3493,)
    } else {
        (w[155],)
    }
};
            w[155] = noise_metadata_schedule_371_0_e3495;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_372_0_e3501: f64 = (w[152] + w[153]);
            let noise_metadata_schedule_372_0_e3502: f64 = (0.5 * noise_metadata_schedule_372_0_e3501);
            let noise_metadata_schedule_372_0_e3503: f64 = (1.0 + noise_metadata_schedule_372_0_e3502);
            let noise_metadata_schedule_372_0_e3504: f64 = (w[155] * noise_metadata_schedule_372_0_e3503);
            w[156] = noise_metadata_schedule_372_0_e3504;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_373_0_e3507: f64 = (params.p15 * w[35]);
            let noise_metadata_schedule_373_0_e3509: f64 = (noise_metadata_schedule_373_0_e3507 * w[132]);
            w[157] = noise_metadata_schedule_373_0_e3509;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_374_0_e3512: f64 = (w[35] * w[272]);
            w[158] = noise_metadata_schedule_374_0_e3512;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_375_0_e3515: f64 = (w[158] - w[157]);
            let noise_metadata_schedule_375_0_e3517: f64 = (noise_metadata_schedule_375_0_e3515 / w[156]);
            w[159] = noise_metadata_schedule_375_0_e3517;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_376_0_e3520: f64 = w[252];
            let noise_metadata_schedule_376_0_e3522: f64 = (noise_metadata_schedule_376_0_e3520 / 0.0001);
            w[285] = noise_metadata_schedule_376_0_e3522;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_377_0_e3525: f64 = if w[252] < 0.0 { 1.0 } else { 0.0 };
            w[542] = noise_metadata_schedule_377_0_e3525;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_378_0_e3537,) = {
    if (w[542] != 0.0) {
        let noise_metadata_schedule_378_0_e3531: f64 = (w[285]).exp();
        let noise_metadata_schedule_378_0_e3532: f64 = (1.0 + noise_metadata_schedule_378_0_e3531);
        let noise_metadata_schedule_378_0_e3533: f64 = (noise_metadata_schedule_378_0_e3532).ln();
        let noise_metadata_schedule_378_0_e3534: f64 = (0.0001 * noise_metadata_schedule_378_0_e3533);
        let noise_metadata_schedule_378_0_e3535: f64 = noise_metadata_schedule_378_0_e3534;
        (noise_metadata_schedule_378_0_e3535,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_378_0_e3537;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_379_0_e3551,) = {
    if (w[542] == 0.0) {
        let noise_metadata_schedule_379_0_e3544: f64 = (-w[285]);
        let noise_metadata_schedule_379_0_e3545: f64 = (noise_metadata_schedule_379_0_e3544).exp();
        let noise_metadata_schedule_379_0_e3546: f64 = (1.0 + noise_metadata_schedule_379_0_e3545);
        let noise_metadata_schedule_379_0_e3547: f64 = (noise_metadata_schedule_379_0_e3546).ln();
        let noise_metadata_schedule_379_0_e3548: f64 = (0.0001 * noise_metadata_schedule_379_0_e3547);
        let noise_metadata_schedule_379_0_e3549: f64 = (w[252] + noise_metadata_schedule_379_0_e3548);
        (noise_metadata_schedule_379_0_e3549,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_379_0_e3551;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_380_0_e3554: f64 = (w[302] / params.p156);
            w[304] = noise_metadata_schedule_380_0_e3554;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_381_0_e3557: f64 = if w[304] < params.p151 { 1.0 } else { 0.0 };
            w[543] = noise_metadata_schedule_381_0_e3557;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_382_0_e3562,) = {
    if (w[543] != 0.0) {
        let noise_metadata_schedule_382_0_e3560: f64 = (w[304]).exp();
        (noise_metadata_schedule_382_0_e3560,)
    } else {
        (w[305],)
    }
};
            w[305] = noise_metadata_schedule_382_0_e3562;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_383_0_e3568,) = {
    if (w[543] == 0.0) {
        let noise_metadata_schedule_383_0_e3566: f64 = (params.p151).exp();
        (noise_metadata_schedule_383_0_e3566,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_383_0_e3568;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_384_0_e3579,) = {
    if (w[543] == 0.0) {
        let noise_metadata_schedule_384_0_e3575: f64 = (w[304] - params.p151);
        let noise_metadata_schedule_384_0_e3576: f64 = (1.0 + noise_metadata_schedule_384_0_e3575);
        let noise_metadata_schedule_384_0_e3577: f64 = (w[301] * noise_metadata_schedule_384_0_e3576);
        (noise_metadata_schedule_384_0_e3577,)
    } else {
        (w[305],)
    }
};
            w[305] = noise_metadata_schedule_384_0_e3579;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_385_0_e3583: f64 = (w[305] - 1.0);
            let noise_metadata_schedule_385_0_e3584: f64 = (w[357] * noise_metadata_schedule_385_0_e3583);
            w[358] = noise_metadata_schedule_385_0_e3584;
        }
        if (active[0] & 0x6) != 0 {
            let noise_metadata_schedule_386_0_e3587: f64 = (w[252] - params.p158);
            let noise_metadata_schedule_386_0_e3589: f64 = (noise_metadata_schedule_386_0_e3587 / 0.001);
            w[285] = noise_metadata_schedule_386_0_e3589;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_387_0_e3592: f64 = if w[252] < params.p158 { 1.0 } else { 0.0 };
            w[544] = noise_metadata_schedule_387_0_e3592;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_388_0_e3604,) = {
    if (w[544] != 0.0) {
        let noise_metadata_schedule_388_0_e3598: f64 = (w[285]).exp();
        let noise_metadata_schedule_388_0_e3599: f64 = (1.0 + noise_metadata_schedule_388_0_e3598);
        let noise_metadata_schedule_388_0_e3600: f64 = (noise_metadata_schedule_388_0_e3599).ln();
        let noise_metadata_schedule_388_0_e3601: f64 = (0.001 * noise_metadata_schedule_388_0_e3600);
        let noise_metadata_schedule_388_0_e3602: f64 = (w[252] - noise_metadata_schedule_388_0_e3601);
        (noise_metadata_schedule_388_0_e3602,)
    } else {
        (w[306],)
    }
};
            w[306] = noise_metadata_schedule_388_0_e3604;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_389_0_e3618,) = {
    if (w[544] == 0.0) {
        let noise_metadata_schedule_389_0_e3611: f64 = (-w[285]);
        let noise_metadata_schedule_389_0_e3612: f64 = (noise_metadata_schedule_389_0_e3611).exp();
        let noise_metadata_schedule_389_0_e3613: f64 = (1.0 + noise_metadata_schedule_389_0_e3612);
        let noise_metadata_schedule_389_0_e3614: f64 = (noise_metadata_schedule_389_0_e3613).ln();
        let noise_metadata_schedule_389_0_e3615: f64 = (0.001 * noise_metadata_schedule_389_0_e3614);
        let noise_metadata_schedule_389_0_e3616: f64 = (params.p158 - noise_metadata_schedule_389_0_e3615);
        (noise_metadata_schedule_389_0_e3616,)
    } else {
        (w[306],)
    }
};
            w[306] = noise_metadata_schedule_389_0_e3618;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_390_0_e3621: f64 = (params.p159 * w[306]);
            let noise_metadata_schedule_390_0_e3624: f64 = (params.p158 - w[306]);
            let noise_metadata_schedule_390_0_e3626: f64 = {let pb=noise_metadata_schedule_390_0_e3624;pb*pb};
            let noise_metadata_schedule_390_0_e3627: f64 = (noise_metadata_schedule_390_0_e3621 * noise_metadata_schedule_390_0_e3626);
            w[359] = noise_metadata_schedule_390_0_e3627;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_391_0_e3630: f64 = (w[252] * w[8]);
            let noise_metadata_schedule_391_0_e3632: f64 = (noise_metadata_schedule_391_0_e3630 / params.p17);
            let noise_metadata_schedule_391_0_e3634: f64 = if noise_metadata_schedule_391_0_e3632 < params.p151 { 1.0 } else { 0.0 };
            w[545] = noise_metadata_schedule_391_0_e3634;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_392_0_e3643,) = {
    if (w[545] != 0.0) {
        let noise_metadata_schedule_392_0_e3638: f64 = (w[252] * w[8]);
        let noise_metadata_schedule_392_0_e3640: f64 = (noise_metadata_schedule_392_0_e3638 / params.p17);
        let noise_metadata_schedule_392_0_e3641: f64 = (noise_metadata_schedule_392_0_e3640).exp();
        (noise_metadata_schedule_392_0_e3641,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_392_0_e3643;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_393_0_e3649,) = {
    if (w[545] == 0.0) {
        let noise_metadata_schedule_393_0_e3647: f64 = (params.p151).exp();
        (noise_metadata_schedule_393_0_e3647,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_393_0_e3649;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_394_0_e3664,) = {
    if (w[545] == 0.0) {
        let noise_metadata_schedule_394_0_e3656: f64 = (w[252] * w[8]);
        let noise_metadata_schedule_394_0_e3658: f64 = (noise_metadata_schedule_394_0_e3656 / params.p17);
        let noise_metadata_schedule_394_0_e3660: f64 = (noise_metadata_schedule_394_0_e3658 - params.p151);
        let noise_metadata_schedule_394_0_e3661: f64 = (1.0 + noise_metadata_schedule_394_0_e3660);
        let noise_metadata_schedule_394_0_e3662: f64 = (w[301] * noise_metadata_schedule_394_0_e3661);
        (noise_metadata_schedule_394_0_e3662,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_394_0_e3664;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_395_0_e3667: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            w[546] = noise_metadata_schedule_395_0_e3667;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_396_0_e3670: f64 = (w[252] - w[55]);
            let noise_metadata_schedule_396_0_e3672: f64 = (noise_metadata_schedule_396_0_e3670 * w[8]);
            let noise_metadata_schedule_396_0_e3674: f64 = if noise_metadata_schedule_396_0_e3672 < params.p151 { 1.0 } else { 0.0 };
            w[547] = noise_metadata_schedule_396_0_e3674;
        }
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_397_0_e3685,) = {
    if ((w[546] != 0.0) && (w[547] != 0.0)) {
        let noise_metadata_schedule_397_0_e3680: f64 = (w[252] - w[55]);
        let noise_metadata_schedule_397_0_e3682: f64 = (noise_metadata_schedule_397_0_e3680 * w[8]);
        let noise_metadata_schedule_397_0_e3683: f64 = (noise_metadata_schedule_397_0_e3682).exp();
        (noise_metadata_schedule_397_0_e3683,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_397_0_e3685;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_398_0_e3693,) = {
    if ((w[546] != 0.0) && (w[547] == 0.0)) {
        let noise_metadata_schedule_398_0_e3691: f64 = (params.p151).exp();
        (noise_metadata_schedule_398_0_e3691,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_398_0_e3693;
        }
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_399_0_e3710,) = {
    if ((w[546] != 0.0) && (w[547] == 0.0)) {
        let noise_metadata_schedule_399_0_e3702: f64 = (w[252] - w[55]);
        let noise_metadata_schedule_399_0_e3704: f64 = (noise_metadata_schedule_399_0_e3702 * w[8]);
        let noise_metadata_schedule_399_0_e3706: f64 = (noise_metadata_schedule_399_0_e3704 - params.p151);
        let noise_metadata_schedule_399_0_e3707: f64 = (1.0 + noise_metadata_schedule_399_0_e3706);
        let noise_metadata_schedule_399_0_e3708: f64 = (w[301] * noise_metadata_schedule_399_0_e3707);
        (noise_metadata_schedule_399_0_e3708,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_399_0_e3710;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_400_0_e3713: f64 = (w[159] / w[35]);
            let noise_metadata_schedule_400_0_e3715: f64 = (noise_metadata_schedule_400_0_e3713 - 1000.0);
            let noise_metadata_schedule_400_0_e3717: f64 = if noise_metadata_schedule_400_0_e3715 < 40.0 { 1.0 } else { 0.0 };
            w[548] = noise_metadata_schedule_400_0_e3717;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_401_0_e3728,) = {
    if ((w[546] != 0.0) && (w[548] != 0.0)) {
        let noise_metadata_schedule_401_0_e3723: f64 = (w[159] / w[35]);
        let noise_metadata_schedule_401_0_e3725: f64 = (noise_metadata_schedule_401_0_e3723 - 1000.0);
        let noise_metadata_schedule_401_0_e3726: f64 = (noise_metadata_schedule_401_0_e3725).exp();
        (noise_metadata_schedule_401_0_e3726,)
    } else {
        (w[305],)
    }
};
            w[305] = noise_metadata_schedule_401_0_e3728;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_402_0_e3736,) = {
    if ((w[546] != 0.0) && (w[548] == 0.0)) {
        let noise_metadata_schedule_402_0_e3734: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_402_0_e3734,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_402_0_e3736;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_403_0_e3753,) = {
    if ((w[546] != 0.0) && (w[548] == 0.0)) {
        let noise_metadata_schedule_403_0_e3745: f64 = (w[159] / w[35]);
        let noise_metadata_schedule_403_0_e3747: f64 = (noise_metadata_schedule_403_0_e3745 - 1000.0);
        let noise_metadata_schedule_403_0_e3749: f64 = (noise_metadata_schedule_403_0_e3747 - 40.0);
        let noise_metadata_schedule_403_0_e3750: f64 = (1.0 + noise_metadata_schedule_403_0_e3749);
        let noise_metadata_schedule_403_0_e3751: f64 = (w[301] * noise_metadata_schedule_403_0_e3750);
        (noise_metadata_schedule_403_0_e3751,)
    } else {
        (w[305],)
    }
};
            w[305] = noise_metadata_schedule_403_0_e3753;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_404_0_e3796,) = {
    if (w[546] != 0.0) {
        let noise_metadata_schedule_404_0_e3758: f64 = (w[302] - 1.0);
        let noise_metadata_schedule_404_0_e3759: f64 = (w[42] * noise_metadata_schedule_404_0_e3758);
        let noise_metadata_schedule_404_0_e3762: f64 = (w[53] * 2.0);
        let noise_metadata_schedule_404_0_e3765: f64 = (w[302] - 1.0);
        let noise_metadata_schedule_404_0_e3766: f64 = (noise_metadata_schedule_404_0_e3762 * noise_metadata_schedule_404_0_e3765);
        let noise_metadata_schedule_404_0_e3771: f64 = (4.0 * w[304]);
        let noise_metadata_schedule_404_0_e3772: f64 = (1.0 + noise_metadata_schedule_404_0_e3771);
        let noise_metadata_schedule_404_0_e3773: f64 = (noise_metadata_schedule_404_0_e3772).sqrt();
        let noise_metadata_schedule_404_0_e3774: f64 = (1.0 + noise_metadata_schedule_404_0_e3773);
        let noise_metadata_schedule_404_0_e3775: f64 = (noise_metadata_schedule_404_0_e3766 / noise_metadata_schedule_404_0_e3774);
        let noise_metadata_schedule_404_0_e3779: f64 = (w[148] / w[40]);
        let noise_metadata_schedule_404_0_e3780: f64 = (1.0 + noise_metadata_schedule_404_0_e3779);
        let noise_metadata_schedule_404_0_e3781: f64 = (noise_metadata_schedule_404_0_e3775 * noise_metadata_schedule_404_0_e3780);
        let noise_metadata_schedule_404_0_e3782: f64 = (noise_metadata_schedule_404_0_e3759 + noise_metadata_schedule_404_0_e3781);
        let noise_metadata_schedule_404_0_e3786: f64 = (w[131] - 1.0);
        let noise_metadata_schedule_404_0_e3787: f64 = (w[54] * noise_metadata_schedule_404_0_e3786);
        let noise_metadata_schedule_404_0_e3789: f64 = (noise_metadata_schedule_404_0_e3787 * w[305]);
        let noise_metadata_schedule_404_0_e3792: f64 = (1.0 + w[305]);
        let noise_metadata_schedule_404_0_e3793: f64 = (noise_metadata_schedule_404_0_e3789 / noise_metadata_schedule_404_0_e3792);
        let noise_metadata_schedule_404_0_e3794: f64 = (noise_metadata_schedule_404_0_e3782 + noise_metadata_schedule_404_0_e3793);
        (noise_metadata_schedule_404_0_e3794,)
    } else {
        (w[161],)
    }
};
            w[161] = noise_metadata_schedule_404_0_e3796;
        }
        if (active[0] & 0x44) != 0 {
            let noise_metadata_schedule_405_0_e3799: f64 = if params.p93 == 0.0 { 1.0 } else { 0.0 };
            w[549] = noise_metadata_schedule_405_0_e3799;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_406_0_e3810,) = {
    if ((w[546] == 0.0) && (w[549] != 0.0)) {
        let noise_metadata_schedule_406_0_e3807: f64 = (w[302] - 1.0);
        let noise_metadata_schedule_406_0_e3808: f64 = (w[42] * noise_metadata_schedule_406_0_e3807);
        (noise_metadata_schedule_406_0_e3808,)
    } else {
        (w[161],)
    }
};
            w[161] = noise_metadata_schedule_406_0_e3810;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_407_0_e3840,) = {
    if ((w[546] == 0.0) && (w[549] == 0.0)) {
        let noise_metadata_schedule_407_0_e3819: f64 = (1.0 - params.p93);
        let noise_metadata_schedule_407_0_e3822: f64 = (w[302] - 1.0);
        let noise_metadata_schedule_407_0_e3823: f64 = (noise_metadata_schedule_407_0_e3819 * noise_metadata_schedule_407_0_e3822);
        let noise_metadata_schedule_407_0_e3827: f64 = (w[302] + w[131]);
        let noise_metadata_schedule_407_0_e3829: f64 = (noise_metadata_schedule_407_0_e3827 - 2.0);
        let noise_metadata_schedule_407_0_e3830: f64 = (params.p93 * noise_metadata_schedule_407_0_e3829);
        let noise_metadata_schedule_407_0_e3834: f64 = (w[148] / w[40]);
        let noise_metadata_schedule_407_0_e3835: f64 = (1.0 + noise_metadata_schedule_407_0_e3834);
        let noise_metadata_schedule_407_0_e3836: f64 = (noise_metadata_schedule_407_0_e3830 * noise_metadata_schedule_407_0_e3835);
        let noise_metadata_schedule_407_0_e3837: f64 = (noise_metadata_schedule_407_0_e3823 + noise_metadata_schedule_407_0_e3836);
        let noise_metadata_schedule_407_0_e3838: f64 = (w[42] * noise_metadata_schedule_407_0_e3837);
        (noise_metadata_schedule_407_0_e3838,)
    } else {
        (w[161],)
    }
};
            w[161] = noise_metadata_schedule_407_0_e3840;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_408_0_e3843: f64 = (w[253] * w[8]);
            let noise_metadata_schedule_408_0_e3845: f64 = (noise_metadata_schedule_408_0_e3843 / params.p19);
            let noise_metadata_schedule_408_0_e3847: f64 = if noise_metadata_schedule_408_0_e3845 < params.p151 { 1.0 } else { 0.0 };
            w[550] = noise_metadata_schedule_408_0_e3847;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_409_0_e3856,) = {
    if (w[550] != 0.0) {
        let noise_metadata_schedule_409_0_e3851: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_409_0_e3853: f64 = (noise_metadata_schedule_409_0_e3851 / params.p19);
        let noise_metadata_schedule_409_0_e3854: f64 = (noise_metadata_schedule_409_0_e3853).exp();
        (noise_metadata_schedule_409_0_e3854,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_409_0_e3856;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_410_0_e3862,) = {
    if (w[550] == 0.0) {
        let noise_metadata_schedule_410_0_e3860: f64 = (params.p151).exp();
        (noise_metadata_schedule_410_0_e3860,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_410_0_e3862;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_411_0_e3877,) = {
    if (w[550] == 0.0) {
        let noise_metadata_schedule_411_0_e3869: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_411_0_e3871: f64 = (noise_metadata_schedule_411_0_e3869 / params.p19);
        let noise_metadata_schedule_411_0_e3873: f64 = (noise_metadata_schedule_411_0_e3871 - params.p151);
        let noise_metadata_schedule_411_0_e3874: f64 = (1.0 + noise_metadata_schedule_411_0_e3873);
        let noise_metadata_schedule_411_0_e3875: f64 = (w[301] * noise_metadata_schedule_411_0_e3874);
        (noise_metadata_schedule_411_0_e3875,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_411_0_e3877;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_412_0_e3880: f64 = if params.p24 == 1.0 { 1.0 } else { 0.0 };
            w[551] = noise_metadata_schedule_412_0_e3880;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_413_0_e3883: f64 = (w[253] - w[55]);
            let noise_metadata_schedule_413_0_e3885: f64 = (noise_metadata_schedule_413_0_e3883 * w[8]);
            let noise_metadata_schedule_413_0_e3887: f64 = if noise_metadata_schedule_413_0_e3885 < params.p151 { 1.0 } else { 0.0 };
            w[552] = noise_metadata_schedule_413_0_e3887;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_414_0_e3898,) = {
    if ((w[551] != 0.0) && (w[552] != 0.0)) {
        let noise_metadata_schedule_414_0_e3893: f64 = (w[253] - w[55]);
        let noise_metadata_schedule_414_0_e3895: f64 = (noise_metadata_schedule_414_0_e3893 * w[8]);
        let noise_metadata_schedule_414_0_e3896: f64 = (noise_metadata_schedule_414_0_e3895).exp();
        (noise_metadata_schedule_414_0_e3896,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_414_0_e3898;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_415_0_e3906,) = {
    if ((w[551] != 0.0) && (w[552] == 0.0)) {
        let noise_metadata_schedule_415_0_e3904: f64 = (params.p151).exp();
        (noise_metadata_schedule_415_0_e3904,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_415_0_e3906;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_416_0_e3923,) = {
    if ((w[551] != 0.0) && (w[552] == 0.0)) {
        let noise_metadata_schedule_416_0_e3915: f64 = (w[253] - w[55]);
        let noise_metadata_schedule_416_0_e3917: f64 = (noise_metadata_schedule_416_0_e3915 * w[8]);
        let noise_metadata_schedule_416_0_e3919: f64 = (noise_metadata_schedule_416_0_e3917 - params.p151);
        let noise_metadata_schedule_416_0_e3920: f64 = (1.0 + noise_metadata_schedule_416_0_e3919);
        let noise_metadata_schedule_416_0_e3921: f64 = (w[301] * noise_metadata_schedule_416_0_e3920);
        (noise_metadata_schedule_416_0_e3921,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_416_0_e3923;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_417_0_e3948,) = {
    if (w[551] != 0.0) {
        let noise_metadata_schedule_417_0_e3928: f64 = (w[302] - 1.0);
        let noise_metadata_schedule_417_0_e3929: f64 = (w[44] * noise_metadata_schedule_417_0_e3928);
        let noise_metadata_schedule_417_0_e3932: f64 = (w[45] * 2.0);
        let noise_metadata_schedule_417_0_e3935: f64 = (w[302] - 1.0);
        let noise_metadata_schedule_417_0_e3936: f64 = (noise_metadata_schedule_417_0_e3932 * noise_metadata_schedule_417_0_e3935);
        let noise_metadata_schedule_417_0_e3941: f64 = (4.0 * w[304]);
        let noise_metadata_schedule_417_0_e3942: f64 = (1.0 + noise_metadata_schedule_417_0_e3941);
        let noise_metadata_schedule_417_0_e3943: f64 = (noise_metadata_schedule_417_0_e3942).sqrt();
        let noise_metadata_schedule_417_0_e3944: f64 = (1.0 + noise_metadata_schedule_417_0_e3943);
        let noise_metadata_schedule_417_0_e3945: f64 = (noise_metadata_schedule_417_0_e3936 / noise_metadata_schedule_417_0_e3944);
        let noise_metadata_schedule_417_0_e3946: f64 = (noise_metadata_schedule_417_0_e3929 + noise_metadata_schedule_417_0_e3945);
        (noise_metadata_schedule_417_0_e3946,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_417_0_e3948;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_418_0_e3957,) = {
    if (w[551] == 0.0) {
        let noise_metadata_schedule_418_0_e3954: f64 = (w[302] - 1.0);
        let noise_metadata_schedule_418_0_e3955: f64 = (w[44] * noise_metadata_schedule_418_0_e3954);
        (noise_metadata_schedule_418_0_e3955,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_418_0_e3957;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_419_0_e3960: f64 = (w[252] * w[8]);
            let noise_metadata_schedule_419_0_e3962: f64 = (noise_metadata_schedule_419_0_e3960 / params.p21);
            let noise_metadata_schedule_419_0_e3964: f64 = if noise_metadata_schedule_419_0_e3962 < params.p151 { 1.0 } else { 0.0 };
            w[553] = noise_metadata_schedule_419_0_e3964;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_420_0_e3973,) = {
    if (w[553] != 0.0) {
        let noise_metadata_schedule_420_0_e3968: f64 = (w[252] * w[8]);
        let noise_metadata_schedule_420_0_e3970: f64 = (noise_metadata_schedule_420_0_e3968 / params.p21);
        let noise_metadata_schedule_420_0_e3971: f64 = (noise_metadata_schedule_420_0_e3970).exp();
        (noise_metadata_schedule_420_0_e3971,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_420_0_e3973;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_421_0_e3979,) = {
    if (w[553] == 0.0) {
        let noise_metadata_schedule_421_0_e3977: f64 = (params.p151).exp();
        (noise_metadata_schedule_421_0_e3977,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_421_0_e3979;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_422_0_e3994,) = {
    if (w[553] == 0.0) {
        let noise_metadata_schedule_422_0_e3986: f64 = (w[252] * w[8]);
        let noise_metadata_schedule_422_0_e3988: f64 = (noise_metadata_schedule_422_0_e3986 / params.p21);
        let noise_metadata_schedule_422_0_e3990: f64 = (noise_metadata_schedule_422_0_e3988 - params.p151);
        let noise_metadata_schedule_422_0_e3991: f64 = (1.0 + noise_metadata_schedule_422_0_e3990);
        let noise_metadata_schedule_422_0_e3992: f64 = (w[301] * noise_metadata_schedule_422_0_e3991);
        (noise_metadata_schedule_422_0_e3992,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_422_0_e3994;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_423_0_e3998: f64 = (w[302] - 1.0);
            let noise_metadata_schedule_423_0_e3999: f64 = (w[38] * noise_metadata_schedule_423_0_e3998);
            w[163] = noise_metadata_schedule_423_0_e3999;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_424_0_e4002: f64 = (w[253] * w[8]);
            let noise_metadata_schedule_424_0_e4004: f64 = (noise_metadata_schedule_424_0_e4002 / params.p23);
            let noise_metadata_schedule_424_0_e4006: f64 = if noise_metadata_schedule_424_0_e4004 < params.p151 { 1.0 } else { 0.0 };
            w[554] = noise_metadata_schedule_424_0_e4006;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_425_0_e4015,) = {
    if (w[554] != 0.0) {
        let noise_metadata_schedule_425_0_e4010: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_425_0_e4012: f64 = (noise_metadata_schedule_425_0_e4010 / params.p23);
        let noise_metadata_schedule_425_0_e4013: f64 = (noise_metadata_schedule_425_0_e4012).exp();
        (noise_metadata_schedule_425_0_e4013,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_425_0_e4015;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_426_0_e4021,) = {
    if (w[554] == 0.0) {
        let noise_metadata_schedule_426_0_e4019: f64 = (params.p151).exp();
        (noise_metadata_schedule_426_0_e4019,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_426_0_e4021;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_427_0_e4036,) = {
    if (w[554] == 0.0) {
        let noise_metadata_schedule_427_0_e4028: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_427_0_e4030: f64 = (noise_metadata_schedule_427_0_e4028 / params.p23);
        let noise_metadata_schedule_427_0_e4032: f64 = (noise_metadata_schedule_427_0_e4030 - params.p151);
        let noise_metadata_schedule_427_0_e4033: f64 = (1.0 + noise_metadata_schedule_427_0_e4032);
        let noise_metadata_schedule_427_0_e4034: f64 = (w[301] * noise_metadata_schedule_427_0_e4033);
        (noise_metadata_schedule_427_0_e4034,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_427_0_e4036;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_428_0_e4040: f64 = (w[302] - 1.0);
            let noise_metadata_schedule_428_0_e4041: f64 = (w[46] * noise_metadata_schedule_428_0_e4040);
            w[165] = noise_metadata_schedule_428_0_e4041;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_429_0_e4044: f64 = (w[255] * w[8]);
            let noise_metadata_schedule_429_0_e4046: f64 = (noise_metadata_schedule_429_0_e4044 / params.p32);
            let noise_metadata_schedule_429_0_e4048: f64 = if noise_metadata_schedule_429_0_e4046 < params.p151 { 1.0 } else { 0.0 };
            w[555] = noise_metadata_schedule_429_0_e4048;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_430_0_e4057,) = {
    if (w[555] != 0.0) {
        let noise_metadata_schedule_430_0_e4052: f64 = (w[255] * w[8]);
        let noise_metadata_schedule_430_0_e4054: f64 = (noise_metadata_schedule_430_0_e4052 / params.p32);
        let noise_metadata_schedule_430_0_e4055: f64 = (noise_metadata_schedule_430_0_e4054).exp();
        (noise_metadata_schedule_430_0_e4055,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_430_0_e4057;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_431_0_e4063,) = {
    if (w[555] == 0.0) {
        let noise_metadata_schedule_431_0_e4061: f64 = (params.p151).exp();
        (noise_metadata_schedule_431_0_e4061,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_431_0_e4063;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_432_0_e4078,) = {
    if (w[555] == 0.0) {
        let noise_metadata_schedule_432_0_e4070: f64 = (w[255] * w[8]);
        let noise_metadata_schedule_432_0_e4072: f64 = (noise_metadata_schedule_432_0_e4070 / params.p32);
        let noise_metadata_schedule_432_0_e4074: f64 = (noise_metadata_schedule_432_0_e4072 - params.p151);
        let noise_metadata_schedule_432_0_e4075: f64 = (1.0 + noise_metadata_schedule_432_0_e4074);
        let noise_metadata_schedule_432_0_e4076: f64 = (w[301] * noise_metadata_schedule_432_0_e4075);
        (noise_metadata_schedule_432_0_e4076,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_432_0_e4078;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_433_0_e4082: f64 = (w[302] - 1.0);
            let noise_metadata_schedule_433_0_e4083: f64 = (w[39] * noise_metadata_schedule_433_0_e4082);
            w[164] = noise_metadata_schedule_433_0_e4083;
        }
        if (active[0] & 0x18186) != 0 {
            let noise_metadata_schedule_434_0_e4086: f64 = (w[253] * w[8]);
            let noise_metadata_schedule_434_0_e4088: f64 = (noise_metadata_schedule_434_0_e4086 / params.p150);
            let noise_metadata_schedule_434_0_e4090: f64 = if noise_metadata_schedule_434_0_e4088 < params.p151 { 1.0 } else { 0.0 };
            w[556] = noise_metadata_schedule_434_0_e4090;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_435_0_e4099,) = {
    if (w[556] != 0.0) {
        let noise_metadata_schedule_435_0_e4094: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_435_0_e4096: f64 = (noise_metadata_schedule_435_0_e4094 / params.p150);
        let noise_metadata_schedule_435_0_e4097: f64 = (noise_metadata_schedule_435_0_e4096).exp();
        (noise_metadata_schedule_435_0_e4097,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_435_0_e4099;
        }
        if (active[0] & 0x18186) != 0 {
            let (noise_metadata_schedule_436_0_e4105,) = {
    if (w[556] == 0.0) {
        let noise_metadata_schedule_436_0_e4103: f64 = (params.p151).exp();
        (noise_metadata_schedule_436_0_e4103,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_436_0_e4105;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_437_0_e4120,) = {
    if (w[556] == 0.0) {
        let noise_metadata_schedule_437_0_e4112: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_437_0_e4114: f64 = (noise_metadata_schedule_437_0_e4112 / params.p150);
        let noise_metadata_schedule_437_0_e4116: f64 = (noise_metadata_schedule_437_0_e4114 - params.p151);
        let noise_metadata_schedule_437_0_e4117: f64 = (1.0 + noise_metadata_schedule_437_0_e4116);
        let noise_metadata_schedule_437_0_e4118: f64 = (w[301] * noise_metadata_schedule_437_0_e4117);
        (noise_metadata_schedule_437_0_e4118,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_437_0_e4120;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_438_0_e4124: f64 = (w[302] - 1.0);
            let noise_metadata_schedule_438_0_e4125: f64 = (w[47] * noise_metadata_schedule_438_0_e4124);
            w[166] = noise_metadata_schedule_438_0_e4125;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_439_0_e4136: f64 = if (((params.p34 > 0.0) && (params.p35 > 0.0)) && (w[252] < 0.0)) { 1.0 } else { 0.0 };
            w[557] = noise_metadata_schedule_439_0_e4136;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_440_0_e4142: f64 = (2.0 * w[59]);
            let noise_metadata_schedule_440_0_e4143: f64 = (w[62] / noise_metadata_schedule_440_0_e4142);
            let noise_metadata_schedule_440_0_e4144: f64 = (1.0 - noise_metadata_schedule_440_0_e4143);
            let noise_metadata_schedule_440_0_e4145: f64 = (w[61] * noise_metadata_schedule_440_0_e4144);
            let noise_metadata_schedule_440_0_e4147: f64 = if noise_metadata_schedule_440_0_e4145 < params.p151 { 1.0 } else { 0.0 };
            w[558] = noise_metadata_schedule_440_0_e4147;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_441_0_e4162,) = {
    if ((w[557] != 0.0) && (w[558] != 0.0)) {
        let noise_metadata_schedule_441_0_e4156: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_441_0_e4157: f64 = (w[62] / noise_metadata_schedule_441_0_e4156);
        let noise_metadata_schedule_441_0_e4158: f64 = (1.0 - noise_metadata_schedule_441_0_e4157);
        let noise_metadata_schedule_441_0_e4159: f64 = (w[61] * noise_metadata_schedule_441_0_e4158);
        let noise_metadata_schedule_441_0_e4160: f64 = (noise_metadata_schedule_441_0_e4159).exp();
        (noise_metadata_schedule_441_0_e4160,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_441_0_e4162;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_442_0_e4170,) = {
    if ((w[557] != 0.0) && (w[558] == 0.0)) {
        let noise_metadata_schedule_442_0_e4168: f64 = (params.p151).exp();
        (noise_metadata_schedule_442_0_e4168,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_442_0_e4170;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_443_0_e4191,) = {
    if ((w[557] != 0.0) && (w[558] == 0.0)) {
        let noise_metadata_schedule_443_0_e4182: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_443_0_e4183: f64 = (w[62] / noise_metadata_schedule_443_0_e4182);
        let noise_metadata_schedule_443_0_e4184: f64 = (1.0 - noise_metadata_schedule_443_0_e4183);
        let noise_metadata_schedule_443_0_e4185: f64 = (w[61] * noise_metadata_schedule_443_0_e4184);
        let noise_metadata_schedule_443_0_e4187: f64 = (noise_metadata_schedule_443_0_e4185 - params.p151);
        let noise_metadata_schedule_443_0_e4188: f64 = (1.0 + noise_metadata_schedule_443_0_e4187);
        let noise_metadata_schedule_443_0_e4189: f64 = (w[301] * noise_metadata_schedule_443_0_e4188);
        (noise_metadata_schedule_443_0_e4189,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_443_0_e4191;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_444_0_e4197,) = {
    if (w[557] != 0.0) {
        let noise_metadata_schedule_444_0_e4195: f64 = (w[252] * w[65]);
        (noise_metadata_schedule_444_0_e4195,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_444_0_e4197;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_445_0_e4241,) = {
    if (w[557] != 0.0) {
        let noise_metadata_schedule_445_0_e4201: f64 = (w[281] * w[281]);
        let noise_metadata_schedule_445_0_e4203: f64 = (noise_metadata_schedule_445_0_e4201 + 1e-30);
        let noise_metadata_schedule_445_0_e4204: f64 = (noise_metadata_schedule_445_0_e4203).sqrt();
        let noise_metadata_schedule_445_0_e4206: f64 = (-2.0);
        let noise_metadata_schedule_445_0_e4208: f64 = (noise_metadata_schedule_445_0_e4206 - params.p67);
        let noise_metadata_schedule_445_0_e4209: f64 = (noise_metadata_schedule_445_0_e4204).powf(noise_metadata_schedule_445_0_e4208);
        let noise_metadata_schedule_445_0_e4214: f64 = (params.p67 * params.p67);
        let noise_metadata_schedule_445_0_e4215: f64 = (1.0 - noise_metadata_schedule_445_0_e4214);
        let noise_metadata_schedule_445_0_e4218: f64 = (3.0 * w[281]);
        let noise_metadata_schedule_445_0_e4221: f64 = (params.p67 - 1.0);
        let noise_metadata_schedule_445_0_e4222: f64 = (noise_metadata_schedule_445_0_e4218 * noise_metadata_schedule_445_0_e4221);
        let noise_metadata_schedule_445_0_e4223: f64 = (noise_metadata_schedule_445_0_e4215 - noise_metadata_schedule_445_0_e4222);
        let noise_metadata_schedule_445_0_e4224: f64 = (params.p67 * noise_metadata_schedule_445_0_e4223);
        let noise_metadata_schedule_445_0_e4227: f64 = (6.0 * w[281]);
        let noise_metadata_schedule_445_0_e4229: f64 = (noise_metadata_schedule_445_0_e4227 * w[281]);
        let noise_metadata_schedule_445_0_e4232: f64 = (params.p67 - 1.0);
        let noise_metadata_schedule_445_0_e4234: f64 = (noise_metadata_schedule_445_0_e4232 + w[281]);
        let noise_metadata_schedule_445_0_e4235: f64 = (noise_metadata_schedule_445_0_e4229 * noise_metadata_schedule_445_0_e4234);
        let noise_metadata_schedule_445_0_e4236: f64 = (noise_metadata_schedule_445_0_e4224 - noise_metadata_schedule_445_0_e4235);
        let noise_metadata_schedule_445_0_e4237: f64 = (noise_metadata_schedule_445_0_e4209 * noise_metadata_schedule_445_0_e4236);
        let noise_metadata_schedule_445_0_e4239: f64 = (noise_metadata_schedule_445_0_e4237 * 0.16666666666666666);
        (noise_metadata_schedule_445_0_e4239,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_445_0_e4241;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_446_0_e4253,) = {
    if (w[557] != 0.0) {
        let noise_metadata_schedule_446_0_e4245: f64 = (w[252] * w[62]);
        let noise_metadata_schedule_446_0_e4247: f64 = (noise_metadata_schedule_446_0_e4245 * w[61]);
        let noise_metadata_schedule_446_0_e4250: f64 = (w[70] * w[60]);
        let noise_metadata_schedule_446_0_e4251: f64 = (noise_metadata_schedule_446_0_e4247 / noise_metadata_schedule_446_0_e4250);
        (noise_metadata_schedule_446_0_e4251,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_446_0_e4253;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_447_0_e4256: f64 = (-0.001);
            let noise_metadata_schedule_447_0_e4257: f64 = if w[281] < noise_metadata_schedule_447_0_e4256 { 1.0 } else { 0.0 };
            w[559] = noise_metadata_schedule_447_0_e4257;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_448_0_e4260: f64 = if w[281] < params.p151 { 1.0 } else { 0.0 };
            w[560] = noise_metadata_schedule_448_0_e4260;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_449_0_e4269,) = {
    if (((w[557] != 0.0) && (w[559] != 0.0)) && (w[560] != 0.0)) {
        let noise_metadata_schedule_449_0_e4267: f64 = (w[281]).exp();
        (noise_metadata_schedule_449_0_e4267,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_449_0_e4269;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_450_0_e4279,) = {
    if (((w[557] != 0.0) && (w[559] != 0.0)) && (w[560] == 0.0)) {
        let noise_metadata_schedule_450_0_e4277: f64 = (params.p151).exp();
        (noise_metadata_schedule_450_0_e4277,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_450_0_e4279;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_451_0_e4294,) = {
    if (((w[557] != 0.0) && (w[559] != 0.0)) && (w[560] == 0.0)) {
        let noise_metadata_schedule_451_0_e4290: f64 = (w[281] - params.p151);
        let noise_metadata_schedule_451_0_e4291: f64 = (1.0 + noise_metadata_schedule_451_0_e4290);
        let noise_metadata_schedule_451_0_e4292: f64 = (w[301] * noise_metadata_schedule_451_0_e4291);
        (noise_metadata_schedule_451_0_e4292,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_451_0_e4294;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_452_0_e4309,) = {
    if ((w[557] != 0.0) && (w[559] != 0.0)) {
        let noise_metadata_schedule_452_0_e4299: f64 = (-w[252]);
        let noise_metadata_schedule_452_0_e4303: f64 = (1.0 - w[91]);
        let noise_metadata_schedule_452_0_e4305: f64 = (noise_metadata_schedule_452_0_e4303 / w[281]);
        let noise_metadata_schedule_452_0_e4306: f64 = (1.0 + noise_metadata_schedule_452_0_e4305);
        let noise_metadata_schedule_452_0_e4307: f64 = (noise_metadata_schedule_452_0_e4299 * noise_metadata_schedule_452_0_e4306);
        (noise_metadata_schedule_452_0_e4307,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_452_0_e4309;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_453_0_e4332,) = {
    if ((w[557] != 0.0) && (w[559] == 0.0)) {
        let noise_metadata_schedule_453_0_e4316: f64 = (w[252] * 0.5);
        let noise_metadata_schedule_453_0_e4318: f64 = (noise_metadata_schedule_453_0_e4316 * w[281]);
        let noise_metadata_schedule_453_0_e4322: f64 = (w[281] * 0.3333333333333333);
        let noise_metadata_schedule_453_0_e4326: f64 = (0.25 * w[281]);
        let noise_metadata_schedule_453_0_e4327: f64 = (1.0 + noise_metadata_schedule_453_0_e4326);
        let noise_metadata_schedule_453_0_e4328: f64 = (noise_metadata_schedule_453_0_e4322 * noise_metadata_schedule_453_0_e4327);
        let noise_metadata_schedule_453_0_e4329: f64 = (1.0 + noise_metadata_schedule_453_0_e4328);
        let noise_metadata_schedule_453_0_e4330: f64 = (noise_metadata_schedule_453_0_e4318 * noise_metadata_schedule_453_0_e4329);
        (noise_metadata_schedule_453_0_e4330,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_453_0_e4332;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_454_0_e4348,) = {
    if (w[557] != 0.0) {
        let noise_metadata_schedule_454_0_e4336: f64 = (2.0 * w[58]);
        let noise_metadata_schedule_454_0_e4338: f64 = (noise_metadata_schedule_454_0_e4336 * w[69]);
        let noise_metadata_schedule_454_0_e4340: f64 = (noise_metadata_schedule_454_0_e4338 * w[59]);
        let noise_metadata_schedule_454_0_e4342: f64 = (noise_metadata_schedule_454_0_e4340 * w[68]);
        let noise_metadata_schedule_454_0_e4344: f64 = (noise_metadata_schedule_454_0_e4342 * w[65]);
        let noise_metadata_schedule_454_0_e4346: f64 = (noise_metadata_schedule_454_0_e4344 * w[63]);
        (noise_metadata_schedule_454_0_e4346,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_454_0_e4348;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_456_0_e4358,) = {
    if (w[557] == 0.0) {
        (0.0,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_456_0_e4358;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_457_0_e4369: f64 = if (((params.p36 > 0.0) && (params.p37 > 0.0)) && (w[250] < 0.0)) { 1.0 } else { 0.0 };
            w[561] = noise_metadata_schedule_457_0_e4369;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_458_0_e4381,) = {
    if (w[561] != 0.0) {
        let noise_metadata_schedule_458_0_e4374: f64 = (w[250] * w[67]);
        let noise_metadata_schedule_458_0_e4375: f64 = (1.0 - noise_metadata_schedule_458_0_e4374);
        let noise_metadata_schedule_458_0_e4378: f64 = (1.0 - w[76]);
        let noise_metadata_schedule_458_0_e4379: f64 = (noise_metadata_schedule_458_0_e4375).powf(noise_metadata_schedule_458_0_e4378);
        (noise_metadata_schedule_458_0_e4379,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_458_0_e4381;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_459_0_e4387: f64 = (2.0 * w[77]);
            let noise_metadata_schedule_459_0_e4388: f64 = (w[79] / noise_metadata_schedule_459_0_e4387);
            let noise_metadata_schedule_459_0_e4389: f64 = (1.0 - noise_metadata_schedule_459_0_e4388);
            let noise_metadata_schedule_459_0_e4390: f64 = (w[83] * noise_metadata_schedule_459_0_e4389);
            let noise_metadata_schedule_459_0_e4392: f64 = if noise_metadata_schedule_459_0_e4390 < params.p151 { 1.0 } else { 0.0 };
            w[562] = noise_metadata_schedule_459_0_e4392;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_460_0_e4407,) = {
    if ((w[561] != 0.0) && (w[562] != 0.0)) {
        let noise_metadata_schedule_460_0_e4401: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_460_0_e4402: f64 = (w[79] / noise_metadata_schedule_460_0_e4401);
        let noise_metadata_schedule_460_0_e4403: f64 = (1.0 - noise_metadata_schedule_460_0_e4402);
        let noise_metadata_schedule_460_0_e4404: f64 = (w[83] * noise_metadata_schedule_460_0_e4403);
        let noise_metadata_schedule_460_0_e4405: f64 = (noise_metadata_schedule_460_0_e4404).exp();
        (noise_metadata_schedule_460_0_e4405,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_460_0_e4407;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_461_0_e4415,) = {
    if ((w[561] != 0.0) && (w[562] == 0.0)) {
        let noise_metadata_schedule_461_0_e4413: f64 = (params.p151).exp();
        (noise_metadata_schedule_461_0_e4413,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_461_0_e4415;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_462_0_e4436,) = {
    if ((w[561] != 0.0) && (w[562] == 0.0)) {
        let noise_metadata_schedule_462_0_e4427: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_462_0_e4428: f64 = (w[79] / noise_metadata_schedule_462_0_e4427);
        let noise_metadata_schedule_462_0_e4429: f64 = (1.0 - noise_metadata_schedule_462_0_e4428);
        let noise_metadata_schedule_462_0_e4430: f64 = (w[83] * noise_metadata_schedule_462_0_e4429);
        let noise_metadata_schedule_462_0_e4432: f64 = (noise_metadata_schedule_462_0_e4430 - params.p151);
        let noise_metadata_schedule_462_0_e4433: f64 = (1.0 + noise_metadata_schedule_462_0_e4432);
        let noise_metadata_schedule_462_0_e4434: f64 = (w[301] * noise_metadata_schedule_462_0_e4433);
        (noise_metadata_schedule_462_0_e4434,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_462_0_e4436;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_463_0_e4442,) = {
    if (w[561] != 0.0) {
        let noise_metadata_schedule_463_0_e4440: f64 = (w[250] * w[67]);
        (noise_metadata_schedule_463_0_e4440,)
    } else {
        (w[283],)
    }
};
            w[283] = noise_metadata_schedule_463_0_e4442;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_464_0_e4486,) = {
    if (w[561] != 0.0) {
        let noise_metadata_schedule_464_0_e4446: f64 = (w[283] * w[283]);
        let noise_metadata_schedule_464_0_e4448: f64 = (noise_metadata_schedule_464_0_e4446 + 1e-30);
        let noise_metadata_schedule_464_0_e4449: f64 = (noise_metadata_schedule_464_0_e4448).sqrt();
        let noise_metadata_schedule_464_0_e4451: f64 = (-2.0);
        let noise_metadata_schedule_464_0_e4453: f64 = (noise_metadata_schedule_464_0_e4451 - w[76]);
        let noise_metadata_schedule_464_0_e4454: f64 = (noise_metadata_schedule_464_0_e4449).powf(noise_metadata_schedule_464_0_e4453);
        let noise_metadata_schedule_464_0_e4459: f64 = (w[76] * w[76]);
        let noise_metadata_schedule_464_0_e4460: f64 = (1.0 - noise_metadata_schedule_464_0_e4459);
        let noise_metadata_schedule_464_0_e4463: f64 = (3.0 * w[283]);
        let noise_metadata_schedule_464_0_e4466: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_464_0_e4467: f64 = (noise_metadata_schedule_464_0_e4463 * noise_metadata_schedule_464_0_e4466);
        let noise_metadata_schedule_464_0_e4468: f64 = (noise_metadata_schedule_464_0_e4460 - noise_metadata_schedule_464_0_e4467);
        let noise_metadata_schedule_464_0_e4469: f64 = (w[76] * noise_metadata_schedule_464_0_e4468);
        let noise_metadata_schedule_464_0_e4472: f64 = (6.0 * w[283]);
        let noise_metadata_schedule_464_0_e4474: f64 = (noise_metadata_schedule_464_0_e4472 * w[283]);
        let noise_metadata_schedule_464_0_e4477: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_464_0_e4479: f64 = (noise_metadata_schedule_464_0_e4477 + w[283]);
        let noise_metadata_schedule_464_0_e4480: f64 = (noise_metadata_schedule_464_0_e4474 * noise_metadata_schedule_464_0_e4479);
        let noise_metadata_schedule_464_0_e4481: f64 = (noise_metadata_schedule_464_0_e4469 - noise_metadata_schedule_464_0_e4480);
        let noise_metadata_schedule_464_0_e4482: f64 = (noise_metadata_schedule_464_0_e4454 * noise_metadata_schedule_464_0_e4481);
        let noise_metadata_schedule_464_0_e4484: f64 = (noise_metadata_schedule_464_0_e4482 * 0.16666666666666666);
        (noise_metadata_schedule_464_0_e4484,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_464_0_e4486;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_465_0_e4498,) = {
    if (w[561] != 0.0) {
        let noise_metadata_schedule_465_0_e4490: f64 = (w[250] * w[79]);
        let noise_metadata_schedule_465_0_e4492: f64 = (noise_metadata_schedule_465_0_e4490 * w[83]);
        let noise_metadata_schedule_465_0_e4495: f64 = (w[85] * w[80]);
        let noise_metadata_schedule_465_0_e4496: f64 = (noise_metadata_schedule_465_0_e4492 / noise_metadata_schedule_465_0_e4495);
        (noise_metadata_schedule_465_0_e4496,)
    } else {
        (w[283],)
    }
};
            w[283] = noise_metadata_schedule_465_0_e4498;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_466_0_e4501: f64 = (-0.001);
            let noise_metadata_schedule_466_0_e4502: f64 = if w[283] < noise_metadata_schedule_466_0_e4501 { 1.0 } else { 0.0 };
            w[563] = noise_metadata_schedule_466_0_e4502;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_467_0_e4505: f64 = if w[283] < params.p151 { 1.0 } else { 0.0 };
            w[564] = noise_metadata_schedule_467_0_e4505;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_468_0_e4514,) = {
    if (((w[561] != 0.0) && (w[563] != 0.0)) && (w[564] != 0.0)) {
        let noise_metadata_schedule_468_0_e4512: f64 = (w[283]).exp();
        (noise_metadata_schedule_468_0_e4512,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_468_0_e4514;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_8(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_469_0_e4524,) = {
    if (((w[561] != 0.0) && (w[563] != 0.0)) && (w[564] == 0.0)) {
        let noise_metadata_schedule_469_0_e4522: f64 = (params.p151).exp();
        (noise_metadata_schedule_469_0_e4522,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_469_0_e4524;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_470_0_e4539,) = {
    if (((w[561] != 0.0) && (w[563] != 0.0)) && (w[564] == 0.0)) {
        let noise_metadata_schedule_470_0_e4535: f64 = (w[283] - params.p151);
        let noise_metadata_schedule_470_0_e4536: f64 = (1.0 + noise_metadata_schedule_470_0_e4535);
        let noise_metadata_schedule_470_0_e4537: f64 = (w[301] * noise_metadata_schedule_470_0_e4536);
        (noise_metadata_schedule_470_0_e4537,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_470_0_e4539;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_471_0_e4554,) = {
    if ((w[561] != 0.0) && (w[563] != 0.0)) {
        let noise_metadata_schedule_471_0_e4544: f64 = (-w[250]);
        let noise_metadata_schedule_471_0_e4548: f64 = (1.0 - w[92]);
        let noise_metadata_schedule_471_0_e4550: f64 = (noise_metadata_schedule_471_0_e4548 / w[283]);
        let noise_metadata_schedule_471_0_e4551: f64 = (1.0 + noise_metadata_schedule_471_0_e4550);
        let noise_metadata_schedule_471_0_e4552: f64 = (noise_metadata_schedule_471_0_e4544 * noise_metadata_schedule_471_0_e4551);
        (noise_metadata_schedule_471_0_e4552,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_471_0_e4554;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_472_0_e4577,) = {
    if ((w[561] != 0.0) && (w[563] == 0.0)) {
        let noise_metadata_schedule_472_0_e4561: f64 = (w[250] * 0.5);
        let noise_metadata_schedule_472_0_e4563: f64 = (noise_metadata_schedule_472_0_e4561 * w[283]);
        let noise_metadata_schedule_472_0_e4567: f64 = (w[283] * 0.3333333333333333);
        let noise_metadata_schedule_472_0_e4571: f64 = (0.25 * w[283]);
        let noise_metadata_schedule_472_0_e4572: f64 = (1.0 + noise_metadata_schedule_472_0_e4571);
        let noise_metadata_schedule_472_0_e4573: f64 = (noise_metadata_schedule_472_0_e4567 * noise_metadata_schedule_472_0_e4572);
        let noise_metadata_schedule_472_0_e4574: f64 = (1.0 + noise_metadata_schedule_472_0_e4573);
        let noise_metadata_schedule_472_0_e4575: f64 = (noise_metadata_schedule_472_0_e4563 * noise_metadata_schedule_472_0_e4574);
        (noise_metadata_schedule_472_0_e4575,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_472_0_e4577;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_473_0_e4593,) = {
    if (w[561] != 0.0) {
        let noise_metadata_schedule_473_0_e4581: f64 = (2.0 * w[84]);
        let noise_metadata_schedule_473_0_e4583: f64 = (noise_metadata_schedule_473_0_e4581 * w[81]);
        let noise_metadata_schedule_473_0_e4585: f64 = (noise_metadata_schedule_473_0_e4583 * w[77]);
        let noise_metadata_schedule_473_0_e4587: f64 = (noise_metadata_schedule_473_0_e4585 * w[78]);
        let noise_metadata_schedule_473_0_e4589: f64 = (noise_metadata_schedule_473_0_e4587 * w[67]);
        let noise_metadata_schedule_473_0_e4591: f64 = (noise_metadata_schedule_473_0_e4589 * w[89]);
        (noise_metadata_schedule_473_0_e4591,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_473_0_e4593;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_475_0_e4603,) = {
    if (w[561] == 0.0) {
        (0.0,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_475_0_e4603;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_480_0_e4630: f64 = (2.0 * w[43]);
            let noise_metadata_schedule_480_0_e4633: f64 = (w[274] - 1.0);
            let noise_metadata_schedule_480_0_e4634: f64 = (noise_metadata_schedule_480_0_e4630 * noise_metadata_schedule_480_0_e4633);
            let noise_metadata_schedule_480_0_e4639: f64 = (4.0 * w[43]);
            let noise_metadata_schedule_480_0_e4641: f64 = (noise_metadata_schedule_480_0_e4639 / w[37]);
            let noise_metadata_schedule_480_0_e4643: f64 = (noise_metadata_schedule_480_0_e4641 * w[274]);
            let noise_metadata_schedule_480_0_e4644: f64 = (1.0 + noise_metadata_schedule_480_0_e4643);
            let noise_metadata_schedule_480_0_e4645: f64 = (noise_metadata_schedule_480_0_e4644).sqrt();
            let noise_metadata_schedule_480_0_e4646: f64 = (1.0 + noise_metadata_schedule_480_0_e4645);
            let noise_metadata_schedule_480_0_e4647: f64 = (noise_metadata_schedule_480_0_e4634 / noise_metadata_schedule_480_0_e4646);
            w[167] = noise_metadata_schedule_480_0_e4647;
        }
        if (active[0] & 0x60000) != 0 {
            let noise_metadata_schedule_481_0_e4650: f64 = if params.p8 == 1.0 { 1.0 } else { 0.0 };
            w[565] = noise_metadata_schedule_481_0_e4650;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_482_0_e4679,) = {
    if (w[565] != 0.0) {
        let noise_metadata_schedule_482_0_e4654: f64 = (params.p143 * 2.0);
        let noise_metadata_schedule_482_0_e4656: f64 = (noise_metadata_schedule_482_0_e4654 * w[107]);
        let noise_metadata_schedule_482_0_e4659: f64 = (w[271] - w[262]);
        let noise_metadata_schedule_482_0_e4660: f64 = (noise_metadata_schedule_482_0_e4656 * noise_metadata_schedule_482_0_e4659);
        let noise_metadata_schedule_482_0_e4666: f64 = (w[107] / w[109]);
        let noise_metadata_schedule_482_0_e4667: f64 = (4.0 * noise_metadata_schedule_482_0_e4666);
        let noise_metadata_schedule_482_0_e4671: f64 = (params.p144 * w[262]);
        let noise_metadata_schedule_482_0_e4672: f64 = (w[271] + noise_metadata_schedule_482_0_e4671);
        let noise_metadata_schedule_482_0_e4673: f64 = (noise_metadata_schedule_482_0_e4667 * noise_metadata_schedule_482_0_e4672);
        let noise_metadata_schedule_482_0_e4674: f64 = (1.0 + noise_metadata_schedule_482_0_e4673);
        let noise_metadata_schedule_482_0_e4675: f64 = (noise_metadata_schedule_482_0_e4674).sqrt();
        let noise_metadata_schedule_482_0_e4676: f64 = (1.0 + noise_metadata_schedule_482_0_e4675);
        let noise_metadata_schedule_482_0_e4677: f64 = (noise_metadata_schedule_482_0_e4660 / noise_metadata_schedule_482_0_e4676);
        (noise_metadata_schedule_482_0_e4677,)
    } else {
        (w[185],)
    }
};
            w[185] = noise_metadata_schedule_482_0_e4679;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_483_0_e4710,) = {
    if (w[565] != 0.0) {
        let noise_metadata_schedule_483_0_e4683: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_483_0_e4685: f64 = (noise_metadata_schedule_483_0_e4683 * 2.0);
        let noise_metadata_schedule_483_0_e4687: f64 = (noise_metadata_schedule_483_0_e4685 * w[107]);
        let noise_metadata_schedule_483_0_e4690: f64 = (w[274] - w[264]);
        let noise_metadata_schedule_483_0_e4691: f64 = (noise_metadata_schedule_483_0_e4687 * noise_metadata_schedule_483_0_e4690);
        let noise_metadata_schedule_483_0_e4697: f64 = (w[107] / w[109]);
        let noise_metadata_schedule_483_0_e4698: f64 = (4.0 * noise_metadata_schedule_483_0_e4697);
        let noise_metadata_schedule_483_0_e4702: f64 = (params.p144 * w[264]);
        let noise_metadata_schedule_483_0_e4703: f64 = (w[274] + noise_metadata_schedule_483_0_e4702);
        let noise_metadata_schedule_483_0_e4704: f64 = (noise_metadata_schedule_483_0_e4698 * noise_metadata_schedule_483_0_e4703);
        let noise_metadata_schedule_483_0_e4705: f64 = (1.0 + noise_metadata_schedule_483_0_e4704);
        let noise_metadata_schedule_483_0_e4706: f64 = (noise_metadata_schedule_483_0_e4705).sqrt();
        let noise_metadata_schedule_483_0_e4707: f64 = (1.0 + noise_metadata_schedule_483_0_e4706);
        let noise_metadata_schedule_483_0_e4708: f64 = (noise_metadata_schedule_483_0_e4691 / noise_metadata_schedule_483_0_e4707);
        (noise_metadata_schedule_483_0_e4708,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_483_0_e4710;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_484_0_e4736,) = {
    if (w[565] == 0.0) {
        let noise_metadata_schedule_484_0_e4715: f64 = (params.p143 * 2.0);
        let noise_metadata_schedule_484_0_e4717: f64 = (noise_metadata_schedule_484_0_e4715 * w[107]);
        let noise_metadata_schedule_484_0_e4720: f64 = (w[271] - 1.0);
        let noise_metadata_schedule_484_0_e4721: f64 = (noise_metadata_schedule_484_0_e4717 * noise_metadata_schedule_484_0_e4720);
        let noise_metadata_schedule_484_0_e4727: f64 = (w[107] / w[109]);
        let noise_metadata_schedule_484_0_e4728: f64 = (4.0 * noise_metadata_schedule_484_0_e4727);
        let noise_metadata_schedule_484_0_e4730: f64 = (noise_metadata_schedule_484_0_e4728 * w[271]);
        let noise_metadata_schedule_484_0_e4731: f64 = (1.0 + noise_metadata_schedule_484_0_e4730);
        let noise_metadata_schedule_484_0_e4732: f64 = (noise_metadata_schedule_484_0_e4731).sqrt();
        let noise_metadata_schedule_484_0_e4733: f64 = (1.0 + noise_metadata_schedule_484_0_e4732);
        let noise_metadata_schedule_484_0_e4734: f64 = (noise_metadata_schedule_484_0_e4721 / noise_metadata_schedule_484_0_e4733);
        (noise_metadata_schedule_484_0_e4734,)
    } else {
        (w[185],)
    }
};
            w[185] = noise_metadata_schedule_484_0_e4736;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_485_0_e4764,) = {
    if (w[565] == 0.0) {
        let noise_metadata_schedule_485_0_e4741: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_485_0_e4743: f64 = (noise_metadata_schedule_485_0_e4741 * 2.0);
        let noise_metadata_schedule_485_0_e4745: f64 = (noise_metadata_schedule_485_0_e4743 * w[107]);
        let noise_metadata_schedule_485_0_e4748: f64 = (w[274] - 1.0);
        let noise_metadata_schedule_485_0_e4749: f64 = (noise_metadata_schedule_485_0_e4745 * noise_metadata_schedule_485_0_e4748);
        let noise_metadata_schedule_485_0_e4755: f64 = (w[107] / w[109]);
        let noise_metadata_schedule_485_0_e4756: f64 = (4.0 * noise_metadata_schedule_485_0_e4755);
        let noise_metadata_schedule_485_0_e4758: f64 = (noise_metadata_schedule_485_0_e4756 * w[274]);
        let noise_metadata_schedule_485_0_e4759: f64 = (1.0 + noise_metadata_schedule_485_0_e4758);
        let noise_metadata_schedule_485_0_e4760: f64 = (noise_metadata_schedule_485_0_e4759).sqrt();
        let noise_metadata_schedule_485_0_e4761: f64 = (1.0 + noise_metadata_schedule_485_0_e4760);
        let noise_metadata_schedule_485_0_e4762: f64 = (noise_metadata_schedule_485_0_e4749 / noise_metadata_schedule_485_0_e4761);
        (noise_metadata_schedule_485_0_e4762,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_485_0_e4764;
        }
        if (active[0] & 0x80000) != 0 {
            w[183] = 0.0;
        }
        if (active[0] & 0xdfe00) != 0 {
            let noise_metadata_schedule_488_0_e4798: f64 = if ((params.p5 > 0.0) && (params.p33 > 0.0)) { 1.0 } else { 0.0 };
            w[566] = noise_metadata_schedule_488_0_e4798;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_489_0_e4804,) = {
    if (w[566] != 0.0) {
        let noise_metadata_schedule_489_0_e4802: f64 = (w[167] * w[160]);
        (noise_metadata_schedule_489_0_e4802,)
    } else {
        (w[167],)
    }
};
            w[167] = noise_metadata_schedule_489_0_e4804;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_490_0_e4810,) = {
    if (w[566] != 0.0) {
        let noise_metadata_schedule_490_0_e4808: f64 = (w[182] * w[160]);
        (noise_metadata_schedule_490_0_e4808,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_490_0_e4810;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_491_0_e4835,) = {
    if (w[566] != 0.0) {
        let noise_metadata_schedule_491_0_e4814: f64 = (params.p33 * 2.0);
        let noise_metadata_schedule_491_0_e4816: f64 = (noise_metadata_schedule_491_0_e4814 * w[43]);
        let noise_metadata_schedule_491_0_e4819: f64 = (w[275] - 1.0);
        let noise_metadata_schedule_491_0_e4820: f64 = (noise_metadata_schedule_491_0_e4816 * noise_metadata_schedule_491_0_e4819);
        let noise_metadata_schedule_491_0_e4825: f64 = (4.0 * w[43]);
        let noise_metadata_schedule_491_0_e4827: f64 = (noise_metadata_schedule_491_0_e4825 / w[37]);
        let noise_metadata_schedule_491_0_e4829: f64 = (noise_metadata_schedule_491_0_e4827 * w[275]);
        let noise_metadata_schedule_491_0_e4830: f64 = (1.0 + noise_metadata_schedule_491_0_e4829);
        let noise_metadata_schedule_491_0_e4831: f64 = (noise_metadata_schedule_491_0_e4830).sqrt();
        let noise_metadata_schedule_491_0_e4832: f64 = (1.0 + noise_metadata_schedule_491_0_e4831);
        let noise_metadata_schedule_491_0_e4833: f64 = (noise_metadata_schedule_491_0_e4820 / noise_metadata_schedule_491_0_e4832);
        (noise_metadata_schedule_491_0_e4833,)
    } else {
        (w[174],)
    }
};
            w[174] = noise_metadata_schedule_491_0_e4835;
        }
        if (active[0] & 0x86000) != 0 {
            let noise_metadata_schedule_492_0_e4838: f64 = if params.p8 == 1.0 { 1.0 } else { 0.0 };
            w[567] = noise_metadata_schedule_492_0_e4838;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_493_0_e4873,) = {
    if ((w[566] != 0.0) && (w[567] != 0.0)) {
        let noise_metadata_schedule_493_0_e4844: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_493_0_e4846: f64 = (noise_metadata_schedule_493_0_e4844 * params.p33);
        let noise_metadata_schedule_493_0_e4848: f64 = (noise_metadata_schedule_493_0_e4846 * 2.0);
        let noise_metadata_schedule_493_0_e4850: f64 = (noise_metadata_schedule_493_0_e4848 * w[107]);
        let noise_metadata_schedule_493_0_e4853: f64 = (w[275] - w[263]);
        let noise_metadata_schedule_493_0_e4854: f64 = (noise_metadata_schedule_493_0_e4850 * noise_metadata_schedule_493_0_e4853);
        let noise_metadata_schedule_493_0_e4859: f64 = (4.0 * w[107]);
        let noise_metadata_schedule_493_0_e4861: f64 = (noise_metadata_schedule_493_0_e4859 / w[109]);
        let noise_metadata_schedule_493_0_e4865: f64 = (params.p144 * w[263]);
        let noise_metadata_schedule_493_0_e4866: f64 = (w[275] + noise_metadata_schedule_493_0_e4865);
        let noise_metadata_schedule_493_0_e4867: f64 = (noise_metadata_schedule_493_0_e4861 * noise_metadata_schedule_493_0_e4866);
        let noise_metadata_schedule_493_0_e4868: f64 = (1.0 + noise_metadata_schedule_493_0_e4867);
        let noise_metadata_schedule_493_0_e4869: f64 = (noise_metadata_schedule_493_0_e4868).sqrt();
        let noise_metadata_schedule_493_0_e4870: f64 = (1.0 + noise_metadata_schedule_493_0_e4869);
        let noise_metadata_schedule_493_0_e4871: f64 = (noise_metadata_schedule_493_0_e4854 / noise_metadata_schedule_493_0_e4870);
        (noise_metadata_schedule_493_0_e4871,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_493_0_e4873;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_494_0_e4905,) = {
    if ((w[566] != 0.0) && (w[567] == 0.0)) {
        let noise_metadata_schedule_494_0_e4880: f64 = (1.0 - params.p143);
        let noise_metadata_schedule_494_0_e4882: f64 = (noise_metadata_schedule_494_0_e4880 * params.p33);
        let noise_metadata_schedule_494_0_e4884: f64 = (noise_metadata_schedule_494_0_e4882 * 2.0);
        let noise_metadata_schedule_494_0_e4886: f64 = (noise_metadata_schedule_494_0_e4884 * w[107]);
        let noise_metadata_schedule_494_0_e4889: f64 = (w[275] - 1.0);
        let noise_metadata_schedule_494_0_e4890: f64 = (noise_metadata_schedule_494_0_e4886 * noise_metadata_schedule_494_0_e4889);
        let noise_metadata_schedule_494_0_e4895: f64 = (4.0 * w[107]);
        let noise_metadata_schedule_494_0_e4897: f64 = (noise_metadata_schedule_494_0_e4895 / w[109]);
        let noise_metadata_schedule_494_0_e4899: f64 = (noise_metadata_schedule_494_0_e4897 * w[275]);
        let noise_metadata_schedule_494_0_e4900: f64 = (1.0 + noise_metadata_schedule_494_0_e4899);
        let noise_metadata_schedule_494_0_e4901: f64 = (noise_metadata_schedule_494_0_e4900).sqrt();
        let noise_metadata_schedule_494_0_e4902: f64 = (1.0 + noise_metadata_schedule_494_0_e4901);
        let noise_metadata_schedule_494_0_e4903: f64 = (noise_metadata_schedule_494_0_e4890 / noise_metadata_schedule_494_0_e4902);
        (noise_metadata_schedule_494_0_e4903,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_494_0_e4905;
        }
        if (active[0] & 0x9fe00) != 0 {
            let noise_metadata_schedule_495_0_e4908: f64 = if params.p5 == 1.0 { 1.0 } else { 0.0 };
            w[568] = noise_metadata_schedule_495_0_e4908;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_496_0_e4920,) = {
    if ((w[566] != 0.0) && (w[568] != 0.0)) {
        let noise_metadata_schedule_496_0_e4915: f64 = (w[43] + w[107]);
        let noise_metadata_schedule_496_0_e4916: f64 = (params.p33 * noise_metadata_schedule_496_0_e4915);
        let noise_metadata_schedule_496_0_e4918: f64 = (noise_metadata_schedule_496_0_e4916 * w[32]);
        (noise_metadata_schedule_496_0_e4918,)
    } else {
        (w[297],)
    }
};
            w[297] = noise_metadata_schedule_496_0_e4920;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_497_0_e4933,) = {
    if ((w[566] != 0.0) && (w[568] != 0.0)) {
        let noise_metadata_schedule_497_0_e4928: f64 = (w[297] * w[8]);
        let noise_metadata_schedule_497_0_e4929: f64 = (noise_metadata_schedule_497_0_e4928).ln();
        let noise_metadata_schedule_497_0_e4930: f64 = (2.0 - noise_metadata_schedule_497_0_e4929);
        let noise_metadata_schedule_497_0_e4931: f64 = (w[6] * noise_metadata_schedule_497_0_e4930);
        (noise_metadata_schedule_497_0_e4931,)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_497_0_e4933;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_498_0_e4941,) = {
    if ((w[566] != 0.0) && (w[568] != 0.0)) {
        let noise_metadata_schedule_498_0_e4939: f64 = (w[267] - w[176]);
        (noise_metadata_schedule_498_0_e4939,)
    } else {
        (w[290],)
    }
};
            w[290] = noise_metadata_schedule_498_0_e4941;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_499_0_e4949,) = {
    if ((w[566] != 0.0) && (w[568] != 0.0)) {
        let noise_metadata_schedule_499_0_e4947: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_499_0_e4947,)
    } else {
        (w[287],)
    }
};
            w[287] = noise_metadata_schedule_499_0_e4949;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_500_0_e4957,) = {
    if ((w[566] != 0.0) && (w[568] != 0.0)) {
        let noise_metadata_schedule_500_0_e4955: f64 = (w[290] * w[290]);
        (noise_metadata_schedule_500_0_e4955,)
    } else {
        (w[288],)
    }
};
            w[288] = noise_metadata_schedule_500_0_e4957;
        }
        if (active[0] & 0x86000) != 0 {
            let noise_metadata_schedule_501_0_e4960: f64 = if w[290] < 0.0 { 1.0 } else { 0.0 };
            w[569] = noise_metadata_schedule_501_0_e4960;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_502_0_e4977,) = {
    if (((w[566] != 0.0) && (w[568] != 0.0)) && (w[569] != 0.0)) {
        let noise_metadata_schedule_502_0_e4968: f64 = (0.5 * w[287]);
        let noise_metadata_schedule_502_0_e4971: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_502_0_e4972: f64 = (noise_metadata_schedule_502_0_e4971).sqrt();
        let noise_metadata_schedule_502_0_e4974: f64 = (noise_metadata_schedule_502_0_e4972 - w[290]);
        let noise_metadata_schedule_502_0_e4975: f64 = (noise_metadata_schedule_502_0_e4968 / noise_metadata_schedule_502_0_e4974);
        (noise_metadata_schedule_502_0_e4975,)
    } else {
        (w[177],)
    }
};
            w[177] = noise_metadata_schedule_502_0_e4977;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_503_0_e4993,) = {
    if (((w[566] != 0.0) && (w[568] != 0.0)) && (w[569] == 0.0)) {
        let noise_metadata_schedule_503_0_e4987: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_503_0_e4988: f64 = (noise_metadata_schedule_503_0_e4987).sqrt();
        let noise_metadata_schedule_503_0_e4990: f64 = (noise_metadata_schedule_503_0_e4988 + w[290]);
        let noise_metadata_schedule_503_0_e4991: f64 = (0.5 * noise_metadata_schedule_503_0_e4990);
        (noise_metadata_schedule_503_0_e4991,)
    } else {
        (w[177],)
    }
};
            w[177] = noise_metadata_schedule_503_0_e4993;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_504_0_e5009,) = {
    if ((w[566] != 0.0) && (w[568] != 0.0)) {
        let noise_metadata_schedule_504_0_e5001: f64 = (w[174] + w[175]);
        let noise_metadata_schedule_504_0_e5003: f64 = (noise_metadata_schedule_504_0_e5001 * w[32]);
        let noise_metadata_schedule_504_0_e5004: f64 = (w[297] + noise_metadata_schedule_504_0_e5003);
        let noise_metadata_schedule_504_0_e5006: f64 = (noise_metadata_schedule_504_0_e5004 + w[177]);
        let noise_metadata_schedule_504_0_e5007: f64 = (w[177] / noise_metadata_schedule_504_0_e5006);
        (noise_metadata_schedule_504_0_e5007,)
    } else {
        (w[178],)
    }
};
            w[178] = noise_metadata_schedule_504_0_e5009;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_508_0_e5037,) = {
    if ((w[566] != 0.0) && (w[568] == 0.0)) {
        (1.0,)
    } else {
        (w[178],)
    }
};
            w[178] = noise_metadata_schedule_508_0_e5037;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_509_0_e5043,) = {
    if (w[566] != 0.0) {
        let noise_metadata_schedule_509_0_e5041: f64 = (w[178] * w[174]);
        (noise_metadata_schedule_509_0_e5041,)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_509_0_e5043;
        }
        if (active[0] & 0x80000) != 0 {
            let (noise_metadata_schedule_510_0_e5049,) = {
    if (w[566] != 0.0) {
        let noise_metadata_schedule_510_0_e5047: f64 = (w[178] * w[175]);
        (noise_metadata_schedule_510_0_e5047,)
    } else {
        (w[183],)
    }
};
            w[183] = noise_metadata_schedule_510_0_e5049;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_511_0_e5052: f64 = if params.p84 == 1.0 { 1.0 } else { 0.0 };
            w[570] = noise_metadata_schedule_511_0_e5052;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_512_0_e5058,) = {
    if (w[570] != 0.0) {
        let noise_metadata_schedule_512_0_e5056: f64 = (w[254] + w[250]);
        (noise_metadata_schedule_512_0_e5056,)
    } else {
        (w[353],)
    }
};
            w[353] = noise_metadata_schedule_512_0_e5058;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_513_0_e5064,) = {
    if (w[570] != 0.0) {
        let noise_metadata_schedule_513_0_e5062: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_513_0_e5062,)
    } else {
        (w[287],)
    }
};
            w[287] = noise_metadata_schedule_513_0_e5064;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_514_0_e5076,) = {
    if (w[570] != 0.0) {
        let noise_metadata_schedule_514_0_e5067: f64 = (-1.0);
        let noise_metadata_schedule_514_0_e5069: f64 = (noise_metadata_schedule_514_0_e5067 * w[353]);
        let noise_metadata_schedule_514_0_e5071: f64 = (-1.0);
        let noise_metadata_schedule_514_0_e5072: f64 = (noise_metadata_schedule_514_0_e5069 * noise_metadata_schedule_514_0_e5071);
        let noise_metadata_schedule_514_0_e5074: f64 = (noise_metadata_schedule_514_0_e5072 * w[353]);
        (noise_metadata_schedule_514_0_e5074,)
    } else {
        (w[288],)
    }
};
            w[288] = noise_metadata_schedule_514_0_e5076;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_515_0_e5078: f64 = (-1.0);
            let noise_metadata_schedule_515_0_e5080: f64 = (noise_metadata_schedule_515_0_e5078 * w[353]);
            let noise_metadata_schedule_515_0_e5082: f64 = if noise_metadata_schedule_515_0_e5080 < 0.0 { 1.0 } else { 0.0 };
            w[571] = noise_metadata_schedule_515_0_e5082;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_516_0_e5100,) = {
    if ((w[570] != 0.0) && (w[571] != 0.0)) {
        let noise_metadata_schedule_516_0_e5088: f64 = (0.5 * w[287]);
        let noise_metadata_schedule_516_0_e5091: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_516_0_e5092: f64 = (noise_metadata_schedule_516_0_e5091).sqrt();
        let noise_metadata_schedule_516_0_e5094: f64 = (-1.0);
        let noise_metadata_schedule_516_0_e5096: f64 = (noise_metadata_schedule_516_0_e5094 * w[353]);
        let noise_metadata_schedule_516_0_e5097: f64 = (noise_metadata_schedule_516_0_e5092 - noise_metadata_schedule_516_0_e5096);
        let noise_metadata_schedule_516_0_e5098: f64 = (noise_metadata_schedule_516_0_e5088 / noise_metadata_schedule_516_0_e5097);
        (noise_metadata_schedule_516_0_e5098,)
    } else {
        (w[354],)
    }
};
            w[354] = noise_metadata_schedule_516_0_e5100;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_9(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_517_0_e5117,) = {
    if ((w[570] != 0.0) && (w[571] == 0.0)) {
        let noise_metadata_schedule_517_0_e5108: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_517_0_e5109: f64 = (noise_metadata_schedule_517_0_e5108).sqrt();
        let noise_metadata_schedule_517_0_e5111: f64 = (-1.0);
        let noise_metadata_schedule_517_0_e5113: f64 = (noise_metadata_schedule_517_0_e5111 * w[353]);
        let noise_metadata_schedule_517_0_e5114: f64 = (noise_metadata_schedule_517_0_e5109 + noise_metadata_schedule_517_0_e5113);
        let noise_metadata_schedule_517_0_e5115: f64 = (0.5 * noise_metadata_schedule_517_0_e5114);
        (noise_metadata_schedule_517_0_e5115,)
    } else {
        (w[354],)
    }
};
            w[354] = noise_metadata_schedule_517_0_e5117;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_518_0_e5127,) = {
    if (w[570] != 0.0) {
        let noise_metadata_schedule_518_0_e5123: f64 = (w[349]).powf(params.p82);
        let noise_metadata_schedule_518_0_e5124: f64 = (1.0 - noise_metadata_schedule_518_0_e5123);
        let noise_metadata_schedule_518_0_e5125: f64 = (1.0 / noise_metadata_schedule_518_0_e5124);
        (noise_metadata_schedule_518_0_e5125,)
    } else {
        (w[355],)
    }
};
            w[355] = noise_metadata_schedule_518_0_e5127;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_519_0_e5133,) = {
    if (w[570] != 0.0) {
        let noise_metadata_schedule_519_0_e5131: f64 = (w[349] * params.p81);
        (noise_metadata_schedule_519_0_e5131,)
    } else {
        (w[350],)
    }
};
            w[350] = noise_metadata_schedule_519_0_e5133;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_520_0_e5149,) = {
    if (w[570] != 0.0) {
        let noise_metadata_schedule_520_0_e5137: f64 = (w[355] * w[355]);
        let noise_metadata_schedule_520_0_e5141: f64 = (params.p82 - 1.0);
        let noise_metadata_schedule_520_0_e5142: f64 = (w[349]).powf(noise_metadata_schedule_520_0_e5141);
        let noise_metadata_schedule_520_0_e5143: f64 = (noise_metadata_schedule_520_0_e5137 * noise_metadata_schedule_520_0_e5142);
        let noise_metadata_schedule_520_0_e5145: f64 = (noise_metadata_schedule_520_0_e5143 * params.p82);
        let noise_metadata_schedule_520_0_e5147: f64 = (noise_metadata_schedule_520_0_e5145 / params.p81);
        (noise_metadata_schedule_520_0_e5147,)
    } else {
        (w[352],)
    }
};
            w[352] = noise_metadata_schedule_520_0_e5149;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_521_0_e5152: f64 = if w[354] < w[350] { 1.0 } else { 0.0 };
            w[572] = noise_metadata_schedule_521_0_e5152;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_522_0_e5166,) = {
    if ((w[570] != 0.0) && (w[572] != 0.0)) {
        let noise_metadata_schedule_522_0_e5160: f64 = (w[354] / params.p81);
        let noise_metadata_schedule_522_0_e5162: f64 = (noise_metadata_schedule_522_0_e5160).powf(params.p82);
        let noise_metadata_schedule_522_0_e5163: f64 = (1.0 - noise_metadata_schedule_522_0_e5162);
        let noise_metadata_schedule_522_0_e5164: f64 = (1.0 / noise_metadata_schedule_522_0_e5163);
        (noise_metadata_schedule_522_0_e5164,)
    } else {
        (w[351],)
    }
};
            w[351] = noise_metadata_schedule_522_0_e5166;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_523_0_e5179,) = {
    if ((w[570] != 0.0) && (w[572] == 0.0)) {
        let noise_metadata_schedule_523_0_e5174: f64 = (w[354] - w[350]);
        let noise_metadata_schedule_523_0_e5176: f64 = (noise_metadata_schedule_523_0_e5174 * w[352]);
        let noise_metadata_schedule_523_0_e5177: f64 = (w[355] + noise_metadata_schedule_523_0_e5176);
        (noise_metadata_schedule_523_0_e5177,)
    } else {
        (w[351],)
    }
};
            w[351] = noise_metadata_schedule_523_0_e5179;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_524_0_e5184,) = {
    if (w[570] == 0.0) {
        (1.0,)
    } else {
        (w[351],)
    }
};
            w[351] = noise_metadata_schedule_524_0_e5184;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_525_0_e5187: f64 = (w[82] * w[351]);
            w[82] = noise_metadata_schedule_525_0_e5187;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_526_0_e5190: f64 = (w[167] * w[351]);
            w[167] = noise_metadata_schedule_526_0_e5190;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_527_0_e5193: f64 = (w[164] * w[351]);
            w[164] = noise_metadata_schedule_527_0_e5193;
        }
        if (active[0] & 0x6000) != 0 {
            let noise_metadata_schedule_528_0_e5196: f64 = (w[179] * w[351]);
            w[179] = noise_metadata_schedule_528_0_e5196;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_529_0_e5200: f64 = (w[141] / w[41]);
            let noise_metadata_schedule_529_0_e5201: f64 = (1.0 + noise_metadata_schedule_529_0_e5200);
            let noise_metadata_schedule_529_0_e5204: f64 = (w[148] / w[40]);
            let noise_metadata_schedule_529_0_e5205: f64 = (noise_metadata_schedule_529_0_e5201 + noise_metadata_schedule_529_0_e5204);
            w[186] = noise_metadata_schedule_529_0_e5205;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_530_0_e5208: f64 = (0.1 * 0.1);
            w[287] = noise_metadata_schedule_530_0_e5208;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_531_0_e5211: f64 = (w[186] * w[186]);
            w[288] = noise_metadata_schedule_531_0_e5211;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_532_0_e5214: f64 = if w[186] < 0.0 { 1.0 } else { 0.0 };
            w[573] = noise_metadata_schedule_532_0_e5214;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_533_0_e5227,) = {
    if (w[573] != 0.0) {
        let noise_metadata_schedule_533_0_e5218: f64 = (0.5 * w[287]);
        let noise_metadata_schedule_533_0_e5221: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_533_0_e5222: f64 = (noise_metadata_schedule_533_0_e5221).sqrt();
        let noise_metadata_schedule_533_0_e5224: f64 = (noise_metadata_schedule_533_0_e5222 - w[186]);
        let noise_metadata_schedule_533_0_e5225: f64 = (noise_metadata_schedule_533_0_e5218 / noise_metadata_schedule_533_0_e5224);
        (noise_metadata_schedule_533_0_e5225,)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_533_0_e5227;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_534_0_e5239,) = {
    if (w[573] == 0.0) {
        let noise_metadata_schedule_534_0_e5233: f64 = (w[288] + w[287]);
        let noise_metadata_schedule_534_0_e5234: f64 = (noise_metadata_schedule_534_0_e5233).sqrt();
        let noise_metadata_schedule_534_0_e5236: f64 = (noise_metadata_schedule_534_0_e5234 + w[186]);
        let noise_metadata_schedule_534_0_e5237: f64 = (0.5 * noise_metadata_schedule_534_0_e5236);
        (noise_metadata_schedule_534_0_e5237,)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_534_0_e5239;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_535_0_e5245: f64 = (w[152] + w[153]);
            let noise_metadata_schedule_535_0_e5246: f64 = (0.5 * noise_metadata_schedule_535_0_e5245);
            let noise_metadata_schedule_535_0_e5247: f64 = (1.0 + noise_metadata_schedule_535_0_e5246);
            let noise_metadata_schedule_535_0_e5248: f64 = (w[187] * noise_metadata_schedule_535_0_e5247);
            w[188] = noise_metadata_schedule_535_0_e5248;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_536_0_e5251: f64 = (w[29] / w[188]);
            w[190] = noise_metadata_schedule_536_0_e5251;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_537_0_e5254: f64 = if w[190] < w[346] { 1.0 } else { 0.0 };
            w[574] = noise_metadata_schedule_537_0_e5254;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_538_0_e5258,) = {
    if (w[574] != 0.0) {
        (w[346],)
    } else {
        (w[190],)
    }
};
            w[190] = noise_metadata_schedule_538_0_e5258;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_539_0_e5261: f64 = (3.0 * w[190]);
            w[189] = noise_metadata_schedule_539_0_e5261;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_541_0_e5275: f64 = if w[159] > 0.0 { 1.0 } else { 0.0 };
            w[575] = noise_metadata_schedule_541_0_e5275;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_542_0_e5278: f64 = if params.p39 == 1.0 { 1.0 } else { 0.0 };
            w[576] = noise_metadata_schedule_542_0_e5278;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_543_0_e5281: f64 = if w[250] < params.p44 { 1.0 } else { 0.0 };
            w[577] = noise_metadata_schedule_543_0_e5281;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_544_0_e5283: f64 = (-w[159]);
            let noise_metadata_schedule_544_0_e5285: f64 = (noise_metadata_schedule_544_0_e5283 / params.p42);
            let noise_metadata_schedule_544_0_e5287: f64 = if noise_metadata_schedule_544_0_e5285 < params.p151 { 1.0 } else { 0.0 };
            w[578] = noise_metadata_schedule_544_0_e5287;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_545_0_e5301,) = {
    if ((((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) && (w[578] != 0.0)) {
        let noise_metadata_schedule_545_0_e5296: f64 = (-w[159]);
        let noise_metadata_schedule_545_0_e5298: f64 = (noise_metadata_schedule_545_0_e5296 / params.p42);
        let noise_metadata_schedule_545_0_e5299: f64 = (noise_metadata_schedule_545_0_e5298).exp();
        (noise_metadata_schedule_545_0_e5299,)
    } else {
        (w[338],)
    }
};
            w[338] = noise_metadata_schedule_545_0_e5301;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_546_0_e5313,) = {
    if ((((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) && (w[578] == 0.0)) {
        let noise_metadata_schedule_546_0_e5311: f64 = (params.p151).exp();
        (noise_metadata_schedule_546_0_e5311,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_546_0_e5313;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_547_0_e5333,) = {
    if ((((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) && (w[578] == 0.0)) {
        let noise_metadata_schedule_547_0_e5325: f64 = (-w[159]);
        let noise_metadata_schedule_547_0_e5327: f64 = (noise_metadata_schedule_547_0_e5325 / params.p42);
        let noise_metadata_schedule_547_0_e5329: f64 = (noise_metadata_schedule_547_0_e5327 - params.p151);
        let noise_metadata_schedule_547_0_e5330: f64 = (1.0 + noise_metadata_schedule_547_0_e5329);
        let noise_metadata_schedule_547_0_e5331: f64 = (w[301] * noise_metadata_schedule_547_0_e5330);
        (noise_metadata_schedule_547_0_e5331,)
    } else {
        (w[338],)
    }
};
            w[338] = noise_metadata_schedule_547_0_e5333;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_548_0_e5345,) = {
    if (((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) {
        let noise_metadata_schedule_548_0_e5341: f64 = (params.p44 - w[250]);
        let noise_metadata_schedule_548_0_e5343: f64 = (noise_metadata_schedule_548_0_e5341 * w[338]);
        (noise_metadata_schedule_548_0_e5343,)
    } else {
        (w[339],)
    }
};
            w[339] = noise_metadata_schedule_548_0_e5345;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_549_0_e5347: f64 = (-w[340]);
            let noise_metadata_schedule_549_0_e5350: f64 = (w[339]).powf(params.p41);
            let noise_metadata_schedule_549_0_e5351: f64 = (noise_metadata_schedule_549_0_e5347 * noise_metadata_schedule_549_0_e5350);
            let noise_metadata_schedule_549_0_e5353: f64 = if noise_metadata_schedule_549_0_e5351 < params.p151 { 1.0 } else { 0.0 };
            w[579] = noise_metadata_schedule_549_0_e5353;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_550_0_e5369,) = {
    if ((((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) && (w[579] != 0.0)) {
        let noise_metadata_schedule_550_0_e5362: f64 = (-w[340]);
        let noise_metadata_schedule_550_0_e5365: f64 = (w[339]).powf(params.p41);
        let noise_metadata_schedule_550_0_e5366: f64 = (noise_metadata_schedule_550_0_e5362 * noise_metadata_schedule_550_0_e5365);
        let noise_metadata_schedule_550_0_e5367: f64 = (noise_metadata_schedule_550_0_e5366).exp();
        (noise_metadata_schedule_550_0_e5367,)
    } else {
        (w[343],)
    }
};
            w[343] = noise_metadata_schedule_550_0_e5369;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_551_0_e5381,) = {
    if ((((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) && (w[579] == 0.0)) {
        let noise_metadata_schedule_551_0_e5379: f64 = (params.p151).exp();
        (noise_metadata_schedule_551_0_e5379,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_551_0_e5381;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_552_0_e5403,) = {
    if ((((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) && (w[579] == 0.0)) {
        let noise_metadata_schedule_552_0_e5393: f64 = (-w[340]);
        let noise_metadata_schedule_552_0_e5396: f64 = (w[339]).powf(params.p41);
        let noise_metadata_schedule_552_0_e5397: f64 = (noise_metadata_schedule_552_0_e5393 * noise_metadata_schedule_552_0_e5396);
        let noise_metadata_schedule_552_0_e5399: f64 = (noise_metadata_schedule_552_0_e5397 - params.p151);
        let noise_metadata_schedule_552_0_e5400: f64 = (1.0 + noise_metadata_schedule_552_0_e5399);
        let noise_metadata_schedule_552_0_e5401: f64 = (w[301] * noise_metadata_schedule_552_0_e5400);
        (noise_metadata_schedule_552_0_e5401,)
    } else {
        (w[343],)
    }
};
            w[343] = noise_metadata_schedule_552_0_e5403;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_553_0_e5417,) = {
    if (((w[575] != 0.0) && (w[576] != 0.0)) && (w[577] != 0.0)) {
        let noise_metadata_schedule_553_0_e5411: f64 = (params.p40 / w[340]);
        let noise_metadata_schedule_553_0_e5413: f64 = (noise_metadata_schedule_553_0_e5411 * w[339]);
        let noise_metadata_schedule_553_0_e5415: f64 = (noise_metadata_schedule_553_0_e5413 * w[343]);
        (noise_metadata_schedule_553_0_e5415,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_553_0_e5417;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_554_0_e5420: f64 = if params.p39 == 2.0 { 1.0 } else { 0.0 };
            w[580] = noise_metadata_schedule_554_0_e5420;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_555_0_e5423: f64 = if w[250] < w[16] { 1.0 } else { 0.0 };
            w[581] = noise_metadata_schedule_555_0_e5423;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_556_0_e5440,) = {
    if ((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_556_0_e5434: f64 = (2.0 * params.p46);
        let noise_metadata_schedule_556_0_e5437: f64 = (params.p45 * params.p45);
        let noise_metadata_schedule_556_0_e5438: f64 = (noise_metadata_schedule_556_0_e5434 / noise_metadata_schedule_556_0_e5437);
        (noise_metadata_schedule_556_0_e5438,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_556_0_e5440;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_557_0_e5455,) = {
    if ((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_557_0_e5451: f64 = (w[16] - w[250]);
        let noise_metadata_schedule_557_0_e5453: f64 = (noise_metadata_schedule_557_0_e5451 / w[213]);
        (noise_metadata_schedule_557_0_e5453,)
    } else {
        (w[286],)
    }
};
            w[286] = noise_metadata_schedule_557_0_e5455;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_558_0_e5471,) = {
    if ((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_558_0_e5466: f64 = (2.0 * w[286]);
        let noise_metadata_schedule_558_0_e5468: f64 = (noise_metadata_schedule_558_0_e5466 / w[199]);
        let noise_metadata_schedule_558_0_e5469: f64 = (noise_metadata_schedule_558_0_e5468).sqrt();
        (noise_metadata_schedule_558_0_e5469,)
    } else {
        (w[200],)
    }
};
            w[200] = noise_metadata_schedule_558_0_e5471;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_559_0_e5474: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            w[582] = noise_metadata_schedule_559_0_e5474;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_560_0_e5487,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] != 0.0)) {
        (params.p45,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_560_0_e5487;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_561_0_e5505,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] == 0.0)) {
        let noise_metadata_schedule_561_0_e5502: f64 = (0.5 * w[125]);
        let noise_metadata_schedule_561_0_e5503: f64 = (1.0 - noise_metadata_schedule_561_0_e5502);
        (noise_metadata_schedule_561_0_e5503,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_561_0_e5505;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_562_0_e5523,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] == 0.0)) {
        let noise_metadata_schedule_562_0_e5519: f64 = (params.p45 * w[126]);
        let noise_metadata_schedule_562_0_e5521: f64 = (noise_metadata_schedule_562_0_e5519 * w[126]);
        (noise_metadata_schedule_562_0_e5521,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_562_0_e5523;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_563_0_e5545,) = {
    if ((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_563_0_e5534: f64 = (w[200] * w[201]);
        let noise_metadata_schedule_563_0_e5537: f64 = (w[200] * w[200]);
        let noise_metadata_schedule_563_0_e5540: f64 = (w[201] * w[201]);
        let noise_metadata_schedule_563_0_e5541: f64 = (noise_metadata_schedule_563_0_e5537 + noise_metadata_schedule_563_0_e5540);
        let noise_metadata_schedule_563_0_e5542: f64 = (noise_metadata_schedule_563_0_e5541).sqrt();
        let noise_metadata_schedule_563_0_e5543: f64 = (noise_metadata_schedule_563_0_e5534 / noise_metadata_schedule_563_0_e5542);
        (noise_metadata_schedule_563_0_e5543,)
    } else {
        (w[202],)
    }
};
            w[202] = noise_metadata_schedule_563_0_e5545;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_564_0_e5560,) = {
    if ((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_564_0_e5556: f64 = (w[16] - w[250]);
        let noise_metadata_schedule_564_0_e5558: f64 = (noise_metadata_schedule_564_0_e5556 / w[202]);
        (noise_metadata_schedule_564_0_e5558,)
    } else {
        (w[203],)
    }
};
            w[203] = noise_metadata_schedule_564_0_e5560;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_565_0_e5579,) = {
    if ((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_565_0_e5572: f64 = (0.5 * w[202]);
        let noise_metadata_schedule_565_0_e5574: f64 = (noise_metadata_schedule_565_0_e5572 * w[199]);
        let noise_metadata_schedule_565_0_e5576: f64 = (noise_metadata_schedule_565_0_e5574 * w[213]);
        let noise_metadata_schedule_565_0_e5577: f64 = (w[203] + noise_metadata_schedule_565_0_e5576);
        (noise_metadata_schedule_565_0_e5577,)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_565_0_e5579;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_566_0_e5582: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            w[583] = noise_metadata_schedule_566_0_e5582;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_567_0_e5595,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[583] != 0.0)) {
        (w[204],)
    } else {
        (w[205],)
    }
};
            w[205] = noise_metadata_schedule_567_0_e5595;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_568_0_e5619,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[583] == 0.0)) {
        let noise_metadata_schedule_568_0_e5610: f64 = (2.0 * params.p47);
        let noise_metadata_schedule_568_0_e5614: f64 = (2.0 * w[125]);
        let noise_metadata_schedule_568_0_e5615: f64 = (1.0 + noise_metadata_schedule_568_0_e5614);
        let noise_metadata_schedule_568_0_e5616: f64 = (noise_metadata_schedule_568_0_e5610 * noise_metadata_schedule_568_0_e5615);
        let noise_metadata_schedule_568_0_e5617: f64 = (1.0 + noise_metadata_schedule_568_0_e5616);
        (noise_metadata_schedule_568_0_e5617,)
    } else {
        (w[206],)
    }
};
            w[206] = noise_metadata_schedule_568_0_e5619;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_569_0_e5641,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[583] == 0.0)) {
        let noise_metadata_schedule_569_0_e5633: f64 = (1.0 + params.p47);
        let noise_metadata_schedule_569_0_e5637: f64 = (2.0 * params.p47);
        let noise_metadata_schedule_569_0_e5638: f64 = (1.0 + noise_metadata_schedule_569_0_e5637);
        let noise_metadata_schedule_569_0_e5639: f64 = (noise_metadata_schedule_569_0_e5633 / noise_metadata_schedule_569_0_e5638);
        (noise_metadata_schedule_569_0_e5639,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_569_0_e5641;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_570_0_e5669,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[583] == 0.0)) {
        let noise_metadata_schedule_570_0_e5656: f64 = (0.5 * w[202]);
        let noise_metadata_schedule_570_0_e5658: f64 = (noise_metadata_schedule_570_0_e5656 * w[199]);
        let noise_metadata_schedule_570_0_e5663: f64 = (params.p62 * w[206]);
        let noise_metadata_schedule_570_0_e5664: f64 = (w[159] / noise_metadata_schedule_570_0_e5663);
        let noise_metadata_schedule_570_0_e5665: f64 = (w[207] - noise_metadata_schedule_570_0_e5664);
        let noise_metadata_schedule_570_0_e5666: f64 = (noise_metadata_schedule_570_0_e5658 * noise_metadata_schedule_570_0_e5665);
        let noise_metadata_schedule_570_0_e5667: f64 = (w[203] - noise_metadata_schedule_570_0_e5666);
        (noise_metadata_schedule_570_0_e5667,)
    } else {
        (w[208],)
    }
};
            w[208] = noise_metadata_schedule_570_0_e5669;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_10(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_571_0_e5699,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[583] == 0.0)) {
        let noise_metadata_schedule_571_0_e5683: f64 = (w[208] - w[204]);
        let noise_metadata_schedule_571_0_e5686: f64 = (w[208] - w[204]);
        let noise_metadata_schedule_571_0_e5687: f64 = (noise_metadata_schedule_571_0_e5683 * noise_metadata_schedule_571_0_e5686);
        let noise_metadata_schedule_571_0_e5690: f64 = (0.1 * w[203]);
        let noise_metadata_schedule_571_0_e5692: f64 = (noise_metadata_schedule_571_0_e5690 * w[203]);
        let noise_metadata_schedule_571_0_e5694: f64 = (noise_metadata_schedule_571_0_e5692 * w[137]);
        let noise_metadata_schedule_571_0_e5696: f64 = (noise_metadata_schedule_571_0_e5694 / params.p62);
        let noise_metadata_schedule_571_0_e5697: f64 = (noise_metadata_schedule_571_0_e5687 + noise_metadata_schedule_571_0_e5696);
        (noise_metadata_schedule_571_0_e5697,)
    } else {
        (w[286],)
    }
};
            w[286] = noise_metadata_schedule_571_0_e5699;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_572_0_e5720,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[583] == 0.0)) {
        let noise_metadata_schedule_572_0_e5714: f64 = (w[208] + w[204]);
        let noise_metadata_schedule_572_0_e5716: f64 = (w[286]).sqrt();
        let noise_metadata_schedule_572_0_e5717: f64 = (noise_metadata_schedule_572_0_e5714 + noise_metadata_schedule_572_0_e5716);
        let noise_metadata_schedule_572_0_e5718: f64 = (0.5 * noise_metadata_schedule_572_0_e5717);
        (noise_metadata_schedule_572_0_e5718,)
    } else {
        (w[205],)
    }
};
            w[205] = noise_metadata_schedule_572_0_e5720;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_573_0_e5735,) = {
    if ((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_573_0_e5731: f64 = (w[205] - w[203]);
        let noise_metadata_schedule_573_0_e5733: f64 = (noise_metadata_schedule_573_0_e5731 / w[205]);
        (noise_metadata_schedule_573_0_e5733,)
    } else {
        (w[293],)
    }
};
            w[293] = noise_metadata_schedule_573_0_e5735;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_574_0_e5737: f64 = (w[293]).abs();
            let noise_metadata_schedule_574_0_e5739: f64 = if noise_metadata_schedule_574_0_e5737 > 1e-7 { 1.0 } else { 0.0 };
            w[584] = noise_metadata_schedule_574_0_e5739;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_575_0_e5756,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[584] != 0.0)) {
        let noise_metadata_schedule_575_0_e5752: f64 = (0.5 * w[202]);
        let noise_metadata_schedule_575_0_e5754: f64 = (noise_metadata_schedule_575_0_e5752 / w[293]);
        (noise_metadata_schedule_575_0_e5754,)
    } else {
        (w[209],)
    }
};
            w[209] = noise_metadata_schedule_575_0_e5756;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_576_0_e5793,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[584] != 0.0)) {
        let noise_metadata_schedule_576_0_e5769: f64 = (w[0] / w[99]);
        let noise_metadata_schedule_576_0_e5771: f64 = (noise_metadata_schedule_576_0_e5769 * w[205]);
        let noise_metadata_schedule_576_0_e5773: f64 = (noise_metadata_schedule_576_0_e5771 * w[209]);
        let noise_metadata_schedule_576_0_e5775: f64 = (-w[99]);
        let noise_metadata_schedule_576_0_e5777: f64 = (noise_metadata_schedule_576_0_e5775 / w[205]);
        let noise_metadata_schedule_576_0_e5778: f64 = (noise_metadata_schedule_576_0_e5777).exp();
        let noise_metadata_schedule_576_0_e5780: f64 = (-w[99]);
        let noise_metadata_schedule_576_0_e5782: f64 = (noise_metadata_schedule_576_0_e5780 / w[205]);
        let noise_metadata_schedule_576_0_e5786: f64 = (w[201] / w[209]);
        let noise_metadata_schedule_576_0_e5787: f64 = (1.0 + noise_metadata_schedule_576_0_e5786);
        let noise_metadata_schedule_576_0_e5788: f64 = (noise_metadata_schedule_576_0_e5782 * noise_metadata_schedule_576_0_e5787);
        let noise_metadata_schedule_576_0_e5789: f64 = (noise_metadata_schedule_576_0_e5788).exp();
        let noise_metadata_schedule_576_0_e5790: f64 = (noise_metadata_schedule_576_0_e5778 - noise_metadata_schedule_576_0_e5789);
        let noise_metadata_schedule_576_0_e5791: f64 = (noise_metadata_schedule_576_0_e5773 * noise_metadata_schedule_576_0_e5790);
        (noise_metadata_schedule_576_0_e5791,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_576_0_e5793;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_577_0_e5815,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[584] == 0.0)) {
        let noise_metadata_schedule_577_0_e5807: f64 = (w[0] * w[201]);
        let noise_metadata_schedule_577_0_e5809: f64 = (-w[99]);
        let noise_metadata_schedule_577_0_e5811: f64 = (noise_metadata_schedule_577_0_e5809 / w[205]);
        let noise_metadata_schedule_577_0_e5812: f64 = (noise_metadata_schedule_577_0_e5811).exp();
        let noise_metadata_schedule_577_0_e5813: f64 = (noise_metadata_schedule_577_0_e5807 * noise_metadata_schedule_577_0_e5812);
        (noise_metadata_schedule_577_0_e5813,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_577_0_e5815;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_578_0_e5818: f64 = if params.p39 == 3.0 { 1.0 } else { 0.0 };
            w[585] = noise_metadata_schedule_578_0_e5818;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_579_0_e5821: f64 = if w[250] < params.p44 { 1.0 } else { 0.0 };
            w[586] = noise_metadata_schedule_579_0_e5821;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_580_0_e5849,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) {
        let noise_metadata_schedule_580_0_e5835: f64 = (params.p44 - w[250]);
        let noise_metadata_schedule_580_0_e5837: f64 = (noise_metadata_schedule_580_0_e5835).powf(params.p41);
        let noise_metadata_schedule_580_0_e5842: f64 = (params.p48 + w[159]);
        let noise_metadata_schedule_580_0_e5843: f64 = (w[159] / noise_metadata_schedule_580_0_e5842);
        let noise_metadata_schedule_580_0_e5844: f64 = (1.0 - noise_metadata_schedule_580_0_e5843);
        let noise_metadata_schedule_580_0_e5846: f64 = (noise_metadata_schedule_580_0_e5844).powf(params.p49);
        let noise_metadata_schedule_580_0_e5847: f64 = (noise_metadata_schedule_580_0_e5837 * noise_metadata_schedule_580_0_e5846);
        (noise_metadata_schedule_580_0_e5847,)
    } else {
        (w[214],)
    }
};
            w[214] = noise_metadata_schedule_580_0_e5849;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_581_0_e5852: f64 = if params.p7 == 0.0 { 1.0 } else { 0.0 };
            w[587] = noise_metadata_schedule_581_0_e5852;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_582_0_e5868,) = {
    if ((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[587] != 0.0)) {
        (w[214],)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_582_0_e5868;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_583_0_e5889,) = {
    if ((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[587] == 0.0)) {
        let noise_metadata_schedule_583_0_e5885: f64 = (w[159] - params.p52);
        let noise_metadata_schedule_583_0_e5887: f64 = (noise_metadata_schedule_583_0_e5885 / params.p48);
        (noise_metadata_schedule_583_0_e5887,)
    } else {
        (w[216],)
    }
};
            w[216] = noise_metadata_schedule_583_0_e5889;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_584_0_e5910,) = {
    if ((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[587] == 0.0)) {
        let noise_metadata_schedule_584_0_e5906: f64 = (w[216] - 1.0);
        let noise_metadata_schedule_584_0_e5908: f64 = (noise_metadata_schedule_584_0_e5906 / params.p51);
        (noise_metadata_schedule_584_0_e5908,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_584_0_e5910;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_585_0_e5913: f64 = if w[216] < 1.0 { 1.0 } else { 0.0 };
            w[588] = noise_metadata_schedule_585_0_e5913;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_586_0_e5940,) = {
    if (((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[587] == 0.0)) && (w[588] != 0.0)) {
        let noise_metadata_schedule_586_0_e5934: f64 = (w[285]).exp();
        let noise_metadata_schedule_586_0_e5935: f64 = (1.0 + noise_metadata_schedule_586_0_e5934);
        let noise_metadata_schedule_586_0_e5936: f64 = (noise_metadata_schedule_586_0_e5935).ln();
        let noise_metadata_schedule_586_0_e5937: f64 = (params.p51 * noise_metadata_schedule_586_0_e5936);
        let noise_metadata_schedule_586_0_e5938: f64 = (1.0 + noise_metadata_schedule_586_0_e5937);
        (noise_metadata_schedule_586_0_e5938,)
    } else {
        (w[217],)
    }
};
            w[217] = noise_metadata_schedule_586_0_e5940;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_587_0_e5969,) = {
    if (((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[587] == 0.0)) && (w[588] == 0.0)) {
        let noise_metadata_schedule_587_0_e5962: f64 = (-w[285]);
        let noise_metadata_schedule_587_0_e5963: f64 = (noise_metadata_schedule_587_0_e5962).exp();
        let noise_metadata_schedule_587_0_e5964: f64 = (1.0 + noise_metadata_schedule_587_0_e5963);
        let noise_metadata_schedule_587_0_e5965: f64 = (noise_metadata_schedule_587_0_e5964).ln();
        let noise_metadata_schedule_587_0_e5966: f64 = (params.p51 * noise_metadata_schedule_587_0_e5965);
        let noise_metadata_schedule_587_0_e5967: f64 = (w[216] + noise_metadata_schedule_587_0_e5966);
        (noise_metadata_schedule_587_0_e5967,)
    } else {
        (w[217],)
    }
};
            w[217] = noise_metadata_schedule_587_0_e5969;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_588_0_e5990,) = {
    if ((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[587] == 0.0)) {
        let noise_metadata_schedule_588_0_e5987: f64 = (w[217]).powf(params.p50);
        let noise_metadata_schedule_588_0_e5988: f64 = (w[214] * noise_metadata_schedule_588_0_e5987);
        (noise_metadata_schedule_588_0_e5988,)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_588_0_e5990;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_589_0_e5992: f64 = (-w[340]);
            let noise_metadata_schedule_589_0_e5994: f64 = (noise_metadata_schedule_589_0_e5992 * w[215]);
            let noise_metadata_schedule_589_0_e5996: f64 = if noise_metadata_schedule_589_0_e5994 < params.p151 { 1.0 } else { 0.0 };
            w[589] = noise_metadata_schedule_589_0_e5996;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_590_0_e6016,) = {
    if ((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[589] != 0.0)) {
        let noise_metadata_schedule_590_0_e6011: f64 = (-w[340]);
        let noise_metadata_schedule_590_0_e6013: f64 = (noise_metadata_schedule_590_0_e6011 * w[215]);
        let noise_metadata_schedule_590_0_e6014: f64 = (noise_metadata_schedule_590_0_e6013).exp();
        (noise_metadata_schedule_590_0_e6014,)
    } else {
        (w[343],)
    }
};
            w[343] = noise_metadata_schedule_590_0_e6016;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_591_0_e6034,) = {
    if ((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[589] == 0.0)) {
        let noise_metadata_schedule_591_0_e6032: f64 = (params.p151).exp();
        (noise_metadata_schedule_591_0_e6032,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_591_0_e6034;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_592_0_e6060,) = {
    if ((((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) && (w[589] == 0.0)) {
        let noise_metadata_schedule_592_0_e6052: f64 = (-w[340]);
        let noise_metadata_schedule_592_0_e6054: f64 = (noise_metadata_schedule_592_0_e6052 * w[215]);
        let noise_metadata_schedule_592_0_e6056: f64 = (noise_metadata_schedule_592_0_e6054 - params.p151);
        let noise_metadata_schedule_592_0_e6057: f64 = (1.0 + noise_metadata_schedule_592_0_e6056);
        let noise_metadata_schedule_592_0_e6058: f64 = (w[301] * noise_metadata_schedule_592_0_e6057);
        (noise_metadata_schedule_592_0_e6058,)
    } else {
        (w[343],)
    }
};
            w[343] = noise_metadata_schedule_592_0_e6060;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_593_0_e6082,) = {
    if (((((w[575] != 0.0) && (w[576] == 0.0)) && (w[580] == 0.0)) && (w[585] != 0.0)) && (w[586] != 0.0)) {
        let noise_metadata_schedule_593_0_e6074: f64 = (params.p40 / w[340]);
        let noise_metadata_schedule_593_0_e6077: f64 = (params.p44 - w[250]);
        let noise_metadata_schedule_593_0_e6078: f64 = (noise_metadata_schedule_593_0_e6074 * noise_metadata_schedule_593_0_e6077);
        let noise_metadata_schedule_593_0_e6080: f64 = (noise_metadata_schedule_593_0_e6078 * w[343]);
        (noise_metadata_schedule_593_0_e6080,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_593_0_e6082;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_594_0_e6085: f64 = if w[210] > 0.0 { 1.0 } else { 0.0 };
            w[590] = noise_metadata_schedule_594_0_e6085;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_595_0_e6088: f64 = if params.p53 == 1.0 { 1.0 } else { 0.0 };
            w[591] = noise_metadata_schedule_595_0_e6088;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_596_0_e6114,) = {
    if (((w[575] != 0.0) && (w[590] != 0.0)) && (w[591] != 0.0)) {
        let noise_metadata_schedule_596_0_e6098: f64 = (w[30] + w[189]);
        let noise_metadata_schedule_596_0_e6099: f64 = (w[159] * noise_metadata_schedule_596_0_e6098);
        let noise_metadata_schedule_596_0_e6100: f64 = (w[6] / noise_metadata_schedule_596_0_e6099);
        let noise_metadata_schedule_596_0_e6103: f64 = (w[156] / w[35]);
        let noise_metadata_schedule_596_0_e6105: f64 = (noise_metadata_schedule_596_0_e6103 * w[42]);
        let noise_metadata_schedule_596_0_e6106: f64 = (noise_metadata_schedule_596_0_e6100 + noise_metadata_schedule_596_0_e6105);
        let noise_metadata_schedule_596_0_e6110: f64 = (w[30] + w[189]);
        let noise_metadata_schedule_596_0_e6111: f64 = (w[28] / noise_metadata_schedule_596_0_e6110);
        let noise_metadata_schedule_596_0_e6112: f64 = (noise_metadata_schedule_596_0_e6106 + noise_metadata_schedule_596_0_e6111);
        (noise_metadata_schedule_596_0_e6112,)
    } else {
        (w[211],)
    }
};
            w[211] = noise_metadata_schedule_596_0_e6114;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_597_0_e6117: f64 = if params.p39 == 3.0 { 1.0 } else { 0.0 };
            w[592] = noise_metadata_schedule_597_0_e6117;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_598_0_e6131,) = {
    if ((((w[575] != 0.0) && (w[590] != 0.0)) && (w[591] != 0.0)) && (w[592] != 0.0)) {
        let noise_metadata_schedule_598_0_e6127: f64 = (w[210] - w[211]);
        let noise_metadata_schedule_598_0_e6129: f64 = (noise_metadata_schedule_598_0_e6127 / 1e-6);
        (noise_metadata_schedule_598_0_e6129,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_598_0_e6131;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_599_0_e6134: f64 = if w[210] < w[211] { 1.0 } else { 0.0 };
            w[593] = noise_metadata_schedule_599_0_e6134;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_600_0_e6154,) = {
    if (((((w[575] != 0.0) && (w[590] != 0.0)) && (w[591] != 0.0)) && (w[592] != 0.0)) && (w[593] != 0.0)) {
        let noise_metadata_schedule_600_0_e6148: f64 = (w[285]).exp();
        let noise_metadata_schedule_600_0_e6149: f64 = (1.0 + noise_metadata_schedule_600_0_e6148);
        let noise_metadata_schedule_600_0_e6150: f64 = (noise_metadata_schedule_600_0_e6149).ln();
        let noise_metadata_schedule_600_0_e6151: f64 = (1e-6 * noise_metadata_schedule_600_0_e6150);
        let noise_metadata_schedule_600_0_e6152: f64 = (w[210] - noise_metadata_schedule_600_0_e6151);
        (noise_metadata_schedule_600_0_e6152,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_600_0_e6154;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_601_0_e6176,) = {
    if (((((w[575] != 0.0) && (w[590] != 0.0)) && (w[591] != 0.0)) && (w[592] != 0.0)) && (w[593] == 0.0)) {
        let noise_metadata_schedule_601_0_e6169: f64 = (-w[285]);
        let noise_metadata_schedule_601_0_e6170: f64 = (noise_metadata_schedule_601_0_e6169).exp();
        let noise_metadata_schedule_601_0_e6171: f64 = (1.0 + noise_metadata_schedule_601_0_e6170);
        let noise_metadata_schedule_601_0_e6172: f64 = (noise_metadata_schedule_601_0_e6171).ln();
        let noise_metadata_schedule_601_0_e6173: f64 = (1e-6 * noise_metadata_schedule_601_0_e6172);
        let noise_metadata_schedule_601_0_e6174: f64 = (w[211] - noise_metadata_schedule_601_0_e6173);
        (noise_metadata_schedule_601_0_e6174,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_601_0_e6176;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_602_0_e6188,) = {
    if ((((w[575] != 0.0) && (w[590] != 0.0)) && (w[591] != 0.0)) && (w[592] != 0.0)) {
        let noise_metadata_schedule_602_0_e6186: f64 = (w[159] * w[210]);
        (noise_metadata_schedule_602_0_e6186,)
    } else {
        (w[212],)
    }
};
            w[212] = noise_metadata_schedule_602_0_e6188;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_603_0_e6207,) = {
    if ((((w[575] != 0.0) && (w[590] != 0.0)) && (w[591] != 0.0)) && (w[592] == 0.0)) {
        let noise_metadata_schedule_603_0_e6199: f64 = (w[159] * w[210]);
        let noise_metadata_schedule_603_0_e6201: f64 = (noise_metadata_schedule_603_0_e6199 * w[211]);
        let noise_metadata_schedule_603_0_e6204: f64 = (w[210] + w[211]);
        let noise_metadata_schedule_603_0_e6205: f64 = (noise_metadata_schedule_603_0_e6201 / noise_metadata_schedule_603_0_e6204);
        (noise_metadata_schedule_603_0_e6205,)
    } else {
        (w[212],)
    }
};
            w[212] = noise_metadata_schedule_603_0_e6207;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_604_0_e6218,) = {
    if (((w[575] != 0.0) && (w[590] != 0.0)) && (w[591] == 0.0)) {
        let noise_metadata_schedule_604_0_e6216: f64 = (w[159] * w[210]);
        (noise_metadata_schedule_604_0_e6216,)
    } else {
        (w[212],)
    }
};
            w[212] = noise_metadata_schedule_604_0_e6218;
        }
        if (active[0] & 0xff00038) != 0 {
            let noise_metadata_schedule_702_0_e7262: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_702_0_e7264: f64 = (noise_metadata_schedule_702_0_e7262 * w[2]);
            w[308] = noise_metadata_schedule_702_0_e7264;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_703_0_e7267: f64 = (w[308] / w[28]);
            w[309] = noise_metadata_schedule_703_0_e7267;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_704_0_e7270: f64 = (w[308] / w[30]);
            w[310] = noise_metadata_schedule_704_0_e7270;
        }
        if (active[0] & 0xa900000) != 0 {
            let noise_metadata_schedule_705_0_e7273: f64 = (w[308] * w[111]);
            w[311] = noise_metadata_schedule_705_0_e7273;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_706_0_e7276: f64 = (w[308] * w[112]);
            w[312] = noise_metadata_schedule_706_0_e7276;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_707_0_e7279: f64 = (w[308] * w[113]);
            w[313] = noise_metadata_schedule_707_0_e7279;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_708_0_e7282: f64 = (w[308] / w[189]);
            let noise_metadata_schedule_708_0_e7285: f64 = (4.0 * w[273]);
            let noise_metadata_schedule_708_0_e7287: f64 = (noise_metadata_schedule_708_0_e7285 + 5.0);
            let noise_metadata_schedule_708_0_e7288: f64 = (noise_metadata_schedule_708_0_e7282 * noise_metadata_schedule_708_0_e7287);
            let noise_metadata_schedule_708_0_e7290: f64 = (noise_metadata_schedule_708_0_e7288 * 0.3333333333333333);
            w[314] = noise_metadata_schedule_708_0_e7290;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_709_0_e7293: f64 = (w[158] + w[157]);
            let noise_metadata_schedule_709_0_e7295: f64 = (noise_metadata_schedule_709_0_e7293 / w[156]);
            w[333] = noise_metadata_schedule_709_0_e7295;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_710_0_e7298: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_710_0_e7300: f64 = (w[333]).abs();
            let noise_metadata_schedule_710_0_e7301: f64 = (noise_metadata_schedule_710_0_e7298 * noise_metadata_schedule_710_0_e7300);
            w[315] = noise_metadata_schedule_710_0_e7301;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_711_0_e7304: f64 = if params.p130 > 0.0 { 1.0 } else { 0.0 };
            w[614] = noise_metadata_schedule_711_0_e7304;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_712_0_e7311,) = {
    if (w[614] != 0.0) {
        let noise_metadata_schedule_712_0_e7308: f64 = (w[212] / w[333]);
        let noise_metadata_schedule_712_0_e7309: f64 = (noise_metadata_schedule_712_0_e7308).abs();
        (noise_metadata_schedule_712_0_e7309,)
    } else {
        (w[334],)
    }
};
            w[334] = noise_metadata_schedule_712_0_e7311;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_713_0_e7316,) = {
    if (w[614] == 0.0) {
        (0.0,)
    } else {
        (w[334],)
    }
};
            w[334] = noise_metadata_schedule_713_0_e7316;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_714_0_e7319: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_714_0_e7321: f64 = (noise_metadata_schedule_714_0_e7319 * w[212]);
            let noise_metadata_schedule_714_0_e7324: f64 = (w[334] + 1.0);
            let noise_metadata_schedule_714_0_e7325: f64 = (noise_metadata_schedule_714_0_e7321 * noise_metadata_schedule_714_0_e7324);
            w[327] = noise_metadata_schedule_714_0_e7325;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_723_0_e7377: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_723_0_e7380: f64 = (w[161] + w[163]);
            let noise_metadata_schedule_723_0_e7382: f64 = (noise_metadata_schedule_723_0_e7380 - w[57]);
            let noise_metadata_schedule_723_0_e7384: f64 = (noise_metadata_schedule_723_0_e7382 + w[359]);
            let noise_metadata_schedule_723_0_e7386: f64 = (noise_metadata_schedule_723_0_e7384 + w[358]);
            let noise_metadata_schedule_723_0_e7387: f64 = (noise_metadata_schedule_723_0_e7386).abs();
            let noise_metadata_schedule_723_0_e7388: f64 = (noise_metadata_schedule_723_0_e7377 * noise_metadata_schedule_723_0_e7387);
            w[316] = noise_metadata_schedule_723_0_e7388;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_724_0_e7391: f64 = (w[161] + w[162]);
            w[328] = noise_metadata_schedule_724_0_e7391;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_725_0_e7394: f64 = (w[328]).abs();
            let noise_metadata_schedule_725_0_e7396: f64 = (noise_metadata_schedule_725_0_e7394).powf(params.p126);
            let noise_metadata_schedule_725_0_e7397: f64 = (params.p128 * noise_metadata_schedule_725_0_e7396);
            w[317] = noise_metadata_schedule_725_0_e7397;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_726_0_e7400: f64 = if w[328] < 0.0 { 1.0 } else { 0.0 };
            w[618] = noise_metadata_schedule_726_0_e7400;
        }
        if (active[0] & 0x40) != 0 {
            let (noise_metadata_schedule_727_0_e7405,) = {
    if (w[618] != 0.0) {
        let noise_metadata_schedule_727_0_e7403: f64 = (-w[317]);
        (noise_metadata_schedule_727_0_e7403,)
    } else {
        (w[317],)
    }
};
            w[317] = noise_metadata_schedule_727_0_e7405;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_728_0_e7408: f64 = (w[163] + w[165]);
            let noise_metadata_schedule_728_0_e7410: f64 = (noise_metadata_schedule_728_0_e7408 + w[166]);
            w[329] = noise_metadata_schedule_728_0_e7410;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_729_0_e7413: f64 = (w[329]).abs();
            let noise_metadata_schedule_729_0_e7415: f64 = (noise_metadata_schedule_729_0_e7413).powf(params.p127);
            let noise_metadata_schedule_729_0_e7416: f64 = (params.p129 * noise_metadata_schedule_729_0_e7415);
            w[318] = noise_metadata_schedule_729_0_e7416;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_730_0_e7419: f64 = if w[329] < 0.0 { 1.0 } else { 0.0 };
            w[619] = noise_metadata_schedule_730_0_e7419;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_11(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 630], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_731_0_e7424,) = {
    if (w[619] != 0.0) {
        let noise_metadata_schedule_731_0_e7422: f64 = (-w[318]);
        (noise_metadata_schedule_731_0_e7422,)
    } else {
        (w[318],)
    }
};
            w[318] = noise_metadata_schedule_731_0_e7424;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_732_0_e7427: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_732_0_e7430: f64 = (w[162] + w[165]);
            let noise_metadata_schedule_732_0_e7432: f64 = (noise_metadata_schedule_732_0_e7430 + w[166]);
            let noise_metadata_schedule_732_0_e7433: f64 = (noise_metadata_schedule_732_0_e7432).abs();
            let noise_metadata_schedule_732_0_e7434: f64 = (noise_metadata_schedule_732_0_e7427 * noise_metadata_schedule_732_0_e7433);
            w[319] = noise_metadata_schedule_732_0_e7434;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_733_0_e7437: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_733_0_e7439: f64 = (w[164]).abs();
            let noise_metadata_schedule_733_0_e7440: f64 = (noise_metadata_schedule_733_0_e7437 * noise_metadata_schedule_733_0_e7439);
            w[320] = noise_metadata_schedule_733_0_e7440;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_734_0_e7443: f64 = (w[164]).abs();
            let noise_metadata_schedule_734_0_e7445: f64 = (noise_metadata_schedule_734_0_e7443).powf(params.p126);
            let noise_metadata_schedule_734_0_e7446: f64 = (params.p128 * noise_metadata_schedule_734_0_e7445);
            w[321] = noise_metadata_schedule_734_0_e7446;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_735_0_e7449: f64 = if w[164] < 0.0 { 1.0 } else { 0.0 };
            w[620] = noise_metadata_schedule_735_0_e7449;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_736_0_e7454,) = {
    if (w[620] != 0.0) {
        let noise_metadata_schedule_736_0_e7452: f64 = (-w[321]);
        (noise_metadata_schedule_736_0_e7452,)
    } else {
        (w[321],)
    }
};
            w[321] = noise_metadata_schedule_736_0_e7454;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_737_0_e7457: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_737_0_e7459: f64 = (w[82]).abs();
            let noise_metadata_schedule_737_0_e7460: f64 = (noise_metadata_schedule_737_0_e7457 * noise_metadata_schedule_737_0_e7459);
            w[322] = noise_metadata_schedule_737_0_e7460;
        }
        if (active[0] & 0x800) != 0 {
            let noise_metadata_schedule_738_0_e7463: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_738_0_e7465: f64 = (w[167]).abs();
            let noise_metadata_schedule_738_0_e7466: f64 = (noise_metadata_schedule_738_0_e7463 * noise_metadata_schedule_738_0_e7465);
            w[323] = noise_metadata_schedule_738_0_e7466;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_739_0_e7471: f64 = (params.p5 * params.p33);
            let noise_metadata_schedule_739_0_e7472: f64 = (1.0 - noise_metadata_schedule_739_0_e7471);
            let noise_metadata_schedule_739_0_e7473: f64 = (params.p128 * noise_metadata_schedule_739_0_e7472);
            let noise_metadata_schedule_739_0_e7475: f64 = (w[167]).abs();
            let noise_metadata_schedule_739_0_e7479: f64 = (params.p5 * params.p33);
            let noise_metadata_schedule_739_0_e7480: f64 = (1.0 - noise_metadata_schedule_739_0_e7479);
            let noise_metadata_schedule_739_0_e7481: f64 = (noise_metadata_schedule_739_0_e7475 / noise_metadata_schedule_739_0_e7480);
            let noise_metadata_schedule_739_0_e7483: f64 = (noise_metadata_schedule_739_0_e7481).powf(params.p126);
            let noise_metadata_schedule_739_0_e7484: f64 = (noise_metadata_schedule_739_0_e7473 * noise_metadata_schedule_739_0_e7483);
            w[325] = noise_metadata_schedule_739_0_e7484;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_740_0_e7487: f64 = if w[167] < 0.0 { 1.0 } else { 0.0 };
            w[621] = noise_metadata_schedule_740_0_e7487;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_741_0_e7492,) = {
    if (w[621] != 0.0) {
        let noise_metadata_schedule_741_0_e7490: f64 = (-w[325]);
        (noise_metadata_schedule_741_0_e7490,)
    } else {
        (w[325],)
    }
};
            w[325] = noise_metadata_schedule_741_0_e7492;
        }
        if (active[0] & 0x2000) != 0 {
            let noise_metadata_schedule_742_0_e7495: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_742_0_e7497: f64 = (w[179]).abs();
            let noise_metadata_schedule_742_0_e7498: f64 = (noise_metadata_schedule_742_0_e7495 * noise_metadata_schedule_742_0_e7497);
            let noise_metadata_schedule_742_0_e7500: f64 = (noise_metadata_schedule_742_0_e7498 * params.p5);
            w[324] = noise_metadata_schedule_742_0_e7500;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_743_0_e7503: f64 = if params.p33 == 0.0 { 1.0 } else { 0.0 };
            w[622] = noise_metadata_schedule_743_0_e7503;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_744_0_e7507,) = {
    if (w[622] != 0.0) {
        (0.0,)
    } else {
        (w[326],)
    }
};
            w[326] = noise_metadata_schedule_744_0_e7507;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_745_0_e7523,) = {
    if (w[622] == 0.0) {
        let noise_metadata_schedule_745_0_e7512: f64 = (params.p128 * params.p5);
        let noise_metadata_schedule_745_0_e7514: f64 = (noise_metadata_schedule_745_0_e7512 * params.p33);
        let noise_metadata_schedule_745_0_e7516: f64 = (w[179]).abs();
        let noise_metadata_schedule_745_0_e7518: f64 = (noise_metadata_schedule_745_0_e7516 / params.p33);
        let noise_metadata_schedule_745_0_e7520: f64 = (noise_metadata_schedule_745_0_e7518).powf(params.p126);
        let noise_metadata_schedule_745_0_e7521: f64 = (noise_metadata_schedule_745_0_e7514 * noise_metadata_schedule_745_0_e7520);
        (noise_metadata_schedule_745_0_e7521,)
    } else {
        (w[326],)
    }
};
            w[326] = noise_metadata_schedule_745_0_e7523;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_746_0_e7526: f64 = if w[179] < 0.0 { 1.0 } else { 0.0 };
            w[623] = noise_metadata_schedule_746_0_e7526;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_747_0_e7531,) = {
    if (w[623] != 0.0) {
        let noise_metadata_schedule_747_0_e7529: f64 = (-w[326]);
        (noise_metadata_schedule_747_0_e7529,)
    } else {
        (w[326],)
    }
};
            w[326] = noise_metadata_schedule_747_0_e7531;
        }
        if (active[0] & 0x20000) != 0 {
            let noise_metadata_schedule_748_0_e7534: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_748_0_e7536: f64 = (w[185]).abs();
            let noise_metadata_schedule_748_0_e7537: f64 = (noise_metadata_schedule_748_0_e7534 * noise_metadata_schedule_748_0_e7536);
            w[330] = noise_metadata_schedule_748_0_e7537;
        }
        if (active[0] & 0x40000) != 0 {
            let noise_metadata_schedule_749_0_e7540: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_749_0_e7542: f64 = (w[182]).abs();
            let noise_metadata_schedule_749_0_e7543: f64 = (noise_metadata_schedule_749_0_e7540 * noise_metadata_schedule_749_0_e7542);
            w[331] = noise_metadata_schedule_749_0_e7543;
        }
        if (active[0] & 0x80000) != 0 {
            let noise_metadata_schedule_750_0_e7546: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_750_0_e7548: f64 = (w[183]).abs();
            let noise_metadata_schedule_750_0_e7549: f64 = (noise_metadata_schedule_750_0_e7546 * noise_metadata_schedule_750_0_e7548);
            w[332] = noise_metadata_schedule_750_0_e7549;
        }
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 28] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_NOI_GND_IN", label: Some("in"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "noi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_E1_IB2E1", label: Some("ib2e1"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_E1_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_B1_RBC", label: Some("rbc"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_B2_RBV", label: Some("rbv"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B2_E1_IB2E1_F", label: Some("ib2e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_E1_IB1E1_F", label: Some("ib1e1_f"), kind: GeneratedNoiseKind::Flicker, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_E1_IB1E1", label: Some("ib1e1"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "e1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IB3", label: Some("ib3"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IB3_F", label: Some("ib3_f"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_C4_IEX", label: Some("iex"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B1_C4_IEX_F", label: Some("iex_f"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_C3_XIEX", label: Some("xiex"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_B_C3_XIEX_F", label: Some("xiex_f"), kind: GeneratedNoiseKind::Flicker, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C1_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C2_B2_IZTCB", label: Some("iztcb"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "c2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B2_S_ISUB_INT", label: Some("isub_int"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "b2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B1_S_ISUB", label: Some("isub"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "b1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_S_XISUB", label: Some("xisub"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C4_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C3_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C3_C1_RCBLX", label: Some("rcblx"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "c3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C4_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C4_C1_RCBLI", label: Some("rcbli"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "c4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_C1_RCC", label: Some("rcc"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "c1", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 616];
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
            w[610] != 0.0
        };
        let noise_source_16_active = {
            let noise_16_activation_e484: f64 = if (w[610] == 0.0) { 1.0 } else { 0.0 };
            noise_16_activation_e484 != 0.0
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
            let noise_20_activation_e509: f64 = if ((w[611] != 0.0) && (w[612] != 0.0)) { 1.0 } else { 0.0 };
            noise_20_activation_e509 != 0.0
        };
        let noise_source_21_active = {
            let noise_21_activation_e519: f64 = if ((w[611] != 0.0) && (w[612] != 0.0)) { 1.0 } else { 0.0 };
            noise_21_activation_e519 != 0.0
        };
        let noise_source_22_active = {
            let noise_22_activation_e529: f64 = if ((w[611] != 0.0) && (w[612] != 0.0)) { 1.0 } else { 0.0 };
            noise_22_activation_e529 != 0.0
        };
        let noise_source_23_active = {
            let noise_23_activation_e540: f64 = if ((w[611] != 0.0) && (w[612] == 0.0)) { 1.0 } else { 0.0 };
            noise_23_activation_e540 != 0.0
        };
        let noise_source_24_active = {
            let noise_24_activation_e551: f64 = if ((w[611] != 0.0) && (w[612] == 0.0)) { 1.0 } else { 0.0 };
            noise_24_activation_e551 != 0.0
        };
        let noise_source_25_active = {
            let noise_25_activation_e562: f64 = if ((w[611] == 0.0) && (w[613] != 0.0)) { 1.0 } else { 0.0 };
            noise_25_activation_e562 != 0.0
        };
        let noise_source_26_active = {
            let noise_26_activation_e573: f64 = if ((w[611] == 0.0) && (w[613] != 0.0)) { 1.0 } else { 0.0 };
            noise_26_activation_e573 != 0.0
        };
        let noise_source_27_active = {
            let noise_27_activation_e585: f64 = if ((w[611] == 0.0) && (w[613] == 0.0)) { 1.0 } else { 0.0 };
            noise_27_activation_e585 != 0.0
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
            let noise_0_psd_e8407: f64 = 1.0;
            let noise_0_psd_e388: f64 = (w[309] * params[1]);
            let noise_0_psd_e8408: f64 = (noise_0_psd_e8407 * noise_0_psd_e388);
            let psd = noise_0_psd_e8408;
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
            let noise_1_psd_e8410: f64 = 1.0;
            let noise_1_psd_e402: f64 = (w[321] * params[1]);
            let noise_1_psd_e8411: f64 = (noise_1_psd_e8410 * noise_1_psd_e402);
            let psd = noise_1_psd_e8411;
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
            let noise_2_psd_e8413: f64 = 1.0;
            let noise_2_psd_e407: f64 = (w[310] * params[1]);
            let noise_2_psd_e8414: f64 = (noise_2_psd_e8413 * noise_2_psd_e407);
            let psd = noise_2_psd_e8414;
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
            let noise_3_psd_e8416: f64 = 1.0;
            let noise_3_psd_e412: f64 = (w[303] * params[1]);
            let noise_3_psd_e8417: f64 = (noise_3_psd_e8416 * noise_3_psd_e412);
            let psd = noise_3_psd_e8417;
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
            let noise_4_psd_e8419: f64 = 1.0;
            let noise_4_psd_e417: f64 = (w[304] * params[1]);
            let noise_4_psd_e8420: f64 = (noise_4_psd_e8419 * noise_4_psd_e417);
            let psd = noise_4_psd_e8420;
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
            let noise_5_psd_e8422: f64 = 1.0;
            let noise_5_psd_e422: f64 = (w[308] * params[1]);
            let noise_5_psd_e8423: f64 = (noise_5_psd_e8422 * noise_5_psd_e422);
            let psd = noise_5_psd_e8423;
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
            let noise_6_psd_e8425: f64 = 1.0;
            let noise_6_psd_e427: f64 = (w[311] * params[1]);
            let noise_6_psd_e8426: f64 = (noise_6_psd_e8425 * noise_6_psd_e427);
            let psd = noise_6_psd_e8426;
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
            let noise_7_psd_e8428: f64 = 1.0;
            let noise_7_psd_e433: f64 = (w[312] * params[1]);
            let noise_7_psd_e8429: f64 = (noise_7_psd_e8428 * noise_7_psd_e433);
            let psd = noise_7_psd_e8429;
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
            let noise_8_psd_e8431: f64 = 1.0;
            let noise_8_psd_e439: f64 = (w[313] * params[1]);
            let noise_8_psd_e8432: f64 = (noise_8_psd_e8431 * noise_8_psd_e439);
            let psd = noise_8_psd_e8432;
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
            let noise_9_psd_e8434: f64 = 1.0;
            let noise_9_psd_e444: f64 = (w[314] * params[1]);
            let noise_9_psd_e8435: f64 = (noise_9_psd_e8434 * noise_9_psd_e444);
            let psd = noise_9_psd_e8435;
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
            let noise_10_psd_e8437: f64 = 1.0;
            let noise_10_psd_e449: f64 = (w[315] * params[1]);
            let noise_10_psd_e8438: f64 = (noise_10_psd_e8437 * noise_10_psd_e449);
            let psd = noise_10_psd_e8438;
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
            let noise_11_psd_e8440: f64 = 1.0;
            let noise_11_psd_e455: f64 = (w[317] * params[1]);
            let noise_11_psd_e8441: f64 = (noise_11_psd_e8440 * noise_11_psd_e455);
            let psd = noise_11_psd_e8441;
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
            let noise_12_psd_e8443: f64 = 1.0;
            let noise_12_psd_e460: f64 = (w[319] * params[1]);
            let noise_12_psd_e8444: f64 = (noise_12_psd_e8443 * noise_12_psd_e460);
            let psd = noise_12_psd_e8444;
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
            let noise_13_psd_e8446: f64 = 1.0;
            let noise_13_psd_e466: f64 = (w[318] * params[1]);
            let noise_13_psd_e8447: f64 = (noise_13_psd_e8446 * noise_13_psd_e466);
            let psd = noise_13_psd_e8447;
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
            let noise_14_psd_e8449: f64 = 1.0;
            let noise_14_psd_e471: f64 = (w[320] * params[1]);
            let noise_14_psd_e8450: f64 = (noise_14_psd_e8449 * noise_14_psd_e471);
            let psd = noise_14_psd_e8450;
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
            let noise_15_psd_e8452: f64 = 1.0;
            let noise_15_psd_e478: f64 = (w[316] * params[1]);
            let noise_15_psd_e8453: f64 = (noise_15_psd_e8452 * noise_15_psd_e478);
            let psd = noise_15_psd_e8453;
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
            let noise_16_psd_e8455: f64 = 1.0;
            let noise_16_psd_e487: f64 = (w[316] * params[1]);
            let noise_16_psd_e8456: f64 = (noise_16_psd_e8455 * noise_16_psd_e487);
            let psd = noise_16_psd_e8456;
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
            let noise_17_psd_e8458: f64 = 1.0;
            let noise_17_psd_e494: f64 = (w[324] * params[1]);
            let noise_17_psd_e8459: f64 = (noise_17_psd_e8458 * noise_17_psd_e494);
            let psd = noise_17_psd_e8459;
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
            let noise_18_psd_e8461: f64 = 1.0;
            let noise_18_psd_e499: f64 = (w[325] * params[1]);
            let noise_18_psd_e8462: f64 = (noise_18_psd_e8461 * noise_18_psd_e499);
            let psd = noise_18_psd_e8462;
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
            let noise_19_psd_e8464: f64 = 1.0;
            let noise_19_psd_e504: f64 = (w[326] * params[1]);
            let noise_19_psd_e8465: f64 = (noise_19_psd_e8464 * noise_19_psd_e504);
            let psd = noise_19_psd_e8465;
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
            let noise_20_psd_e8467: f64 = 1.0;
            let noise_20_psd_e512: f64 = (w[305] * params[1]);
            let noise_20_psd_e8468: f64 = (noise_20_psd_e8467 * noise_20_psd_e512);
            let psd = noise_20_psd_e8468;
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
            let noise_21_psd_e8470: f64 = 1.0;
            let noise_21_psd_e522: f64 = (w[306] * params[1]);
            let noise_21_psd_e8471: f64 = (noise_21_psd_e8470 * noise_21_psd_e522);
            let psd = noise_21_psd_e8471;
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
            let noise_22_psd_e8473: f64 = 1.0;
            let noise_22_psd_e532: f64 = (w[307] * params[1]);
            let noise_22_psd_e8474: f64 = (noise_22_psd_e8473 * noise_22_psd_e532);
            let psd = noise_22_psd_e8474;
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
            let noise_23_psd_e8476: f64 = 1.0;
            let noise_23_psd_e543: f64 = (w[305] * params[1]);
            let noise_23_psd_e8477: f64 = (noise_23_psd_e8476 * noise_23_psd_e543);
            let psd = noise_23_psd_e8477;
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
            let noise_24_psd_e8479: f64 = 1.0;
            let noise_24_psd_e554: f64 = (w[306] * params[1]);
            let noise_24_psd_e8480: f64 = (noise_24_psd_e8479 * noise_24_psd_e554);
            let psd = noise_24_psd_e8480;
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
            let noise_25_psd_e8482: f64 = 1.0;
            let noise_25_psd_e565: f64 = (w[305] * params[1]);
            let noise_25_psd_e8483: f64 = (noise_25_psd_e8482 * noise_25_psd_e565);
            let psd = noise_25_psd_e8483;
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
            let noise_26_psd_e8485: f64 = 1.0;
            let noise_26_psd_e576: f64 = (w[307] * params[1]);
            let noise_26_psd_e8486: f64 = (noise_26_psd_e8485 * noise_26_psd_e576);
            let psd = noise_26_psd_e8486;
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
            let noise_27_psd_e8488: f64 = 1.0;
            let noise_27_psd_e588: f64 = (w[305] * params[1]);
            let noise_27_psd_e8489: f64 = (noise_27_psd_e8488 * noise_27_psd_e588);
            let psd = noise_27_psd_e8489;
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
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616]) {
        let params = &*self.params;
        let noise_activation_schedule_732_0_e7296: f64 = if params[24] == 1.0 { 1.0 } else { 0.0 };
        w[610] = noise_activation_schedule_732_0_e7296;
        let noise_activation_schedule_733_0_e7299: f64 = if params[58] > 0.0 { 1.0 } else { 0.0 };
        w[611] = noise_activation_schedule_733_0_e7299;
        let noise_activation_schedule_734_0_e7302: f64 = if params[59] > 0.0 { 1.0 } else { 0.0 };
        w[612] = noise_activation_schedule_734_0_e7302;
        let noise_activation_schedule_735_0_e7305: f64 = if params[59] > 0.0 { 1.0 } else { 0.0 };
        w[613] = noise_activation_schedule_735_0_e7305;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_0_0_e595: f64 = if params[3] == 1.0 { 1.0 } else { 0.0 };
            w[476] = noise_metadata_schedule_0_0_e595;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_1_0_e599,) = {
    if (w[476] != 0.0) {
        (70300000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_1_0_e599;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_2_0_e603,) = {
    if (w[476] != 0.0) {
        (123000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_2_0_e603;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3_0_e608,) = {
    if (w[476] == 0.0) {
        (158000000.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_3_0_e608;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_4_0_e613,) = {
    if (w[476] == 0.0) {
        (204000000.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_4_0_e613;
        }
        if (active[0] & 0x41800) != 0 {
            let noise_metadata_schedule_5_0_e616: f64 = (1.0 - params[33]);
            w[157] = noise_metadata_schedule_5_0_e616;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_6_0_e619: f64 = (params[4] + 273.15);
            w[3] = noise_metadata_schedule_6_0_e619;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_7_0_e620: f64 = ctx.temperature();
            let noise_metadata_schedule_7_0_e622: f64 = (noise_metadata_schedule_7_0_e620 + params[0]);
            w[5] = noise_metadata_schedule_7_0_e622;
        }
        if (active[0] & 0xff0003a) != 0 {
            let noise_metadata_schedule_9_0_e628: f64 = if params[150] == 0.0 { 1.0 } else { 0.0 };
            w[477] = noise_metadata_schedule_9_0_e628;
        }
        if (active[0] & 0xff0003a) != 0 {
            let (noise_metadata_schedule_10_0_e632,) = {
    if (w[477] != 0.0) {
        (1e-12,)
    } else {
        (w[339],)
    }
};
            w[339] = noise_metadata_schedule_10_0_e632;
        }
        if (active[0] & 0xff0003a) != 0 {
            let (noise_metadata_schedule_11_0_e637,) = {
    if (w[477] == 0.0) {
        (params[150],)
    } else {
        (w[339],)
    }
};
            w[339] = noise_metadata_schedule_11_0_e637;
        }
        if (active[0] & 0xff0003a) != 0 {
            let noise_metadata_schedule_12_0_e640: f64 = (w[339] * params[1]);
            w[340] = noise_metadata_schedule_12_0_e640;
        }
        if (active[0] & 0xff00000) != 0 {
            let noise_metadata_schedule_13_0_e643: f64 = (1.0 / w[340]);
            w[341] = noise_metadata_schedule_13_0_e643;
        }
        if (active[0] & 0xfffe7) != 0 {
            w[52] = 0.001;
        }
        if (active[0] & 0x9ffe7) != 0 {
            w[336] = 0.001;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_19_0_e661: f64 = (2.0 - params[67]);
            let noise_metadata_schedule_19_0_e662: f64 = (2.0_f64).powf(noise_metadata_schedule_19_0_e661);
            w[62] = noise_metadata_schedule_19_0_e662;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_20_0_e665: f64 = (1.0 / w[62]);
            w[63] = noise_metadata_schedule_20_0_e665;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_21_0_e669: f64 = (params[115] * w[3]);
            let noise_metadata_schedule_21_0_e671: f64 = (noise_metadata_schedule_21_0_e669 * w[3]);
            let noise_metadata_schedule_21_0_e674: f64 = (w[3] + params[116]);
            let noise_metadata_schedule_21_0_e675: f64 = (noise_metadata_schedule_21_0_e671 / noise_metadata_schedule_21_0_e674);
            let noise_metadata_schedule_21_0_e676: f64 = (params[114] + noise_metadata_schedule_21_0_e675);
            let noise_metadata_schedule_21_0_e678: f64 = (noise_metadata_schedule_21_0_e676 - 0.05);
            let noise_metadata_schedule_21_0_e680: f64 = (noise_metadata_schedule_21_0_e678 / 0.1);
            w[279] = noise_metadata_schedule_21_0_e680;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_22_0_e684: f64 = (params[115] * w[3]);
            let noise_metadata_schedule_22_0_e686: f64 = (noise_metadata_schedule_22_0_e684 * w[3]);
            let noise_metadata_schedule_22_0_e689: f64 = (w[3] + params[116]);
            let noise_metadata_schedule_22_0_e690: f64 = (noise_metadata_schedule_22_0_e686 / noise_metadata_schedule_22_0_e689);
            let noise_metadata_schedule_22_0_e691: f64 = (params[114] + noise_metadata_schedule_22_0_e690);
            let noise_metadata_schedule_22_0_e693: f64 = if noise_metadata_schedule_22_0_e691 < 0.05 { 1.0 } else { 0.0 };
            w[479] = noise_metadata_schedule_22_0_e693;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_23_0_e705,) = {
    if (w[479] != 0.0) {
        let noise_metadata_schedule_23_0_e699: f64 = (w[279]).exp();
        let noise_metadata_schedule_23_0_e700: f64 = (1.0 + noise_metadata_schedule_23_0_e699);
        let noise_metadata_schedule_23_0_e701: f64 = (noise_metadata_schedule_23_0_e700).ln();
        let noise_metadata_schedule_23_0_e702: f64 = (0.1 * noise_metadata_schedule_23_0_e701);
        let noise_metadata_schedule_23_0_e703: f64 = (0.05 + noise_metadata_schedule_23_0_e702);
        (noise_metadata_schedule_23_0_e703,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_23_0_e705;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_24_0_e729,) = {
    if (w[479] == 0.0) {
        let noise_metadata_schedule_24_0_e711: f64 = (params[115] * w[3]);
        let noise_metadata_schedule_24_0_e713: f64 = (noise_metadata_schedule_24_0_e711 * w[3]);
        let noise_metadata_schedule_24_0_e716: f64 = (w[3] + params[116]);
        let noise_metadata_schedule_24_0_e717: f64 = (noise_metadata_schedule_24_0_e713 / noise_metadata_schedule_24_0_e716);
        let noise_metadata_schedule_24_0_e718: f64 = (params[114] + noise_metadata_schedule_24_0_e717);
        let noise_metadata_schedule_24_0_e722: f64 = (-w[279]);
        let noise_metadata_schedule_24_0_e723: f64 = (noise_metadata_schedule_24_0_e722).exp();
        let noise_metadata_schedule_24_0_e724: f64 = (1.0 + noise_metadata_schedule_24_0_e723);
        let noise_metadata_schedule_24_0_e725: f64 = (noise_metadata_schedule_24_0_e724).ln();
        let noise_metadata_schedule_24_0_e726: f64 = (0.1 * noise_metadata_schedule_24_0_e725);
        let noise_metadata_schedule_24_0_e727: f64 = (noise_metadata_schedule_24_0_e718 + noise_metadata_schedule_24_0_e726);
        (noise_metadata_schedule_24_0_e727,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_24_0_e729;
        }
        if (active[0] & 0x18006) != 0 {
            w[71] = params[114];
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_26_0_e733: f64 = (1.0 / w[71]);
            w[72] = noise_metadata_schedule_26_0_e733;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_27_0_e736: f64 = (1.0 / params[66]);
            w[64] = noise_metadata_schedule_27_0_e736;
        }
        if (active[0] & 0x18002) != 0 {
            w[75] = params[71];
        }
        if (active[0] & 0x18002) != 0 {
            w[76] = params[72];
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_30_0_e742: f64 = (2.0 - w[76]);
            let noise_metadata_schedule_30_0_e743: f64 = (2.0_f64).powf(noise_metadata_schedule_30_0_e742);
            w[79] = noise_metadata_schedule_30_0_e743;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_31_0_e746: f64 = (1.0 / w[79]);
            w[89] = noise_metadata_schedule_31_0_e746;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_32_0_e750: f64 = (params[118] * w[3]);
            let noise_metadata_schedule_32_0_e752: f64 = (noise_metadata_schedule_32_0_e750 * w[3]);
            let noise_metadata_schedule_32_0_e755: f64 = (w[3] + params[119]);
            let noise_metadata_schedule_32_0_e756: f64 = (noise_metadata_schedule_32_0_e752 / noise_metadata_schedule_32_0_e755);
            let noise_metadata_schedule_32_0_e757: f64 = (params[117] + noise_metadata_schedule_32_0_e756);
            let noise_metadata_schedule_32_0_e759: f64 = (noise_metadata_schedule_32_0_e757 - 0.05);
            let noise_metadata_schedule_32_0_e761: f64 = (noise_metadata_schedule_32_0_e759 / 0.1);
            w[279] = noise_metadata_schedule_32_0_e761;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_33_0_e765: f64 = (params[118] * w[3]);
            let noise_metadata_schedule_33_0_e767: f64 = (noise_metadata_schedule_33_0_e765 * w[3]);
            let noise_metadata_schedule_33_0_e770: f64 = (w[3] + params[119]);
            let noise_metadata_schedule_33_0_e771: f64 = (noise_metadata_schedule_33_0_e767 / noise_metadata_schedule_33_0_e770);
            let noise_metadata_schedule_33_0_e772: f64 = (params[117] + noise_metadata_schedule_33_0_e771);
            let noise_metadata_schedule_33_0_e774: f64 = if noise_metadata_schedule_33_0_e772 < 0.05 { 1.0 } else { 0.0 };
            w[480] = noise_metadata_schedule_33_0_e774;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_34_0_e786,) = {
    if (w[480] != 0.0) {
        let noise_metadata_schedule_34_0_e780: f64 = (w[279]).exp();
        let noise_metadata_schedule_34_0_e781: f64 = (1.0 + noise_metadata_schedule_34_0_e780);
        let noise_metadata_schedule_34_0_e782: f64 = (noise_metadata_schedule_34_0_e781).ln();
        let noise_metadata_schedule_34_0_e783: f64 = (0.1 * noise_metadata_schedule_34_0_e782);
        let noise_metadata_schedule_34_0_e784: f64 = (0.05 + noise_metadata_schedule_34_0_e783);
        (noise_metadata_schedule_34_0_e784,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_34_0_e786;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_35_0_e810,) = {
    if (w[480] == 0.0) {
        let noise_metadata_schedule_35_0_e792: f64 = (params[118] * w[3]);
        let noise_metadata_schedule_35_0_e794: f64 = (noise_metadata_schedule_35_0_e792 * w[3]);
        let noise_metadata_schedule_35_0_e797: f64 = (w[3] + params[119]);
        let noise_metadata_schedule_35_0_e798: f64 = (noise_metadata_schedule_35_0_e794 / noise_metadata_schedule_35_0_e797);
        let noise_metadata_schedule_35_0_e799: f64 = (params[117] + noise_metadata_schedule_35_0_e798);
        let noise_metadata_schedule_35_0_e803: f64 = (-w[279]);
        let noise_metadata_schedule_35_0_e804: f64 = (noise_metadata_schedule_35_0_e803).exp();
        let noise_metadata_schedule_35_0_e805: f64 = (1.0 + noise_metadata_schedule_35_0_e804);
        let noise_metadata_schedule_35_0_e806: f64 = (noise_metadata_schedule_35_0_e805).ln();
        let noise_metadata_schedule_35_0_e807: f64 = (0.1 * noise_metadata_schedule_35_0_e806);
        let noise_metadata_schedule_35_0_e808: f64 = (noise_metadata_schedule_35_0_e799 + noise_metadata_schedule_35_0_e807);
        (noise_metadata_schedule_35_0_e808,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_35_0_e810;
        }
        if (active[0] & 0x18002) != 0 {
            w[87] = params[117];
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_37_0_e814: f64 = (1.0 / w[87]);
            w[86] = noise_metadata_schedule_37_0_e814;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_38_0_e817: f64 = (1.0 / w[75]);
            w[66] = noise_metadata_schedule_38_0_e817;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_39_0_e821: f64 = (1.0 / params[83]);
            let noise_metadata_schedule_39_0_e822: f64 = (1.0 - noise_metadata_schedule_39_0_e821);
            w[343] = noise_metadata_schedule_39_0_e822;
        }
        if (active[0] & 0x44) != 0 {
            w[158] = 0.0;
        }
        if (active[0] & 0x140) != 0 {
            w[159] = 0.0;
        }
        if (active[0] & 0x6000) != 0 {
            w[176] = 0.0;
        }
        if (active[0] & 0x86000) != 0 {
            w[175] = 1.0;
        }
        if (active[0] & 0x2) != 0 {
            w[207] = 0.0;
        }
        if (active[0] & 0x2) != 0 {
            w[209] = 0.0;
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
            w[11] = 0.0;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_54_0_e839: f64 = (w[5] + w[11]);
            w[2] = noise_metadata_schedule_54_0_e839;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_55_0_e842: f64 = (w[2] / w[3]);
            w[4] = noise_metadata_schedule_55_0_e842;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_56_0_e845: f64 = (8.617086918058125e-5 * w[2]);
            w[6] = noise_metadata_schedule_56_0_e845;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_57_0_e848: f64 = (8.617086918058125e-5 * w[3]);
            w[7] = noise_metadata_schedule_57_0_e848;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_58_0_e851: f64 = (1.0 / w[6]);
            w[8] = noise_metadata_schedule_58_0_e851;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_59_0_e854: f64 = (1.0 / w[7]);
            w[9] = noise_metadata_schedule_59_0_e854;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_60_0_e857: f64 = (w[8] - w[9]);
            w[10] = noise_metadata_schedule_60_0_e857;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_61_0_e860: f64 = (w[2] - w[3]);
            w[12] = noise_metadata_schedule_61_0_e860;
        }
        if (active[0] & 0xfffffff) != 0 {
            let noise_metadata_schedule_62_0_e862: f64 = (w[4]).ln();
            w[274] = noise_metadata_schedule_62_0_e862;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_63_0_e866: f64 = (params[115] * w[2]);
            let noise_metadata_schedule_63_0_e868: f64 = (noise_metadata_schedule_63_0_e866 * w[2]);
            let noise_metadata_schedule_63_0_e871: f64 = (w[2] + params[116]);
            let noise_metadata_schedule_63_0_e872: f64 = (noise_metadata_schedule_63_0_e868 / noise_metadata_schedule_63_0_e871);
            let noise_metadata_schedule_63_0_e873: f64 = (w[74] - noise_metadata_schedule_63_0_e872);
            let noise_metadata_schedule_63_0_e875: f64 = (noise_metadata_schedule_63_0_e873 - 0.05);
            let noise_metadata_schedule_63_0_e877: f64 = (noise_metadata_schedule_63_0_e875 / 0.1);
            w[279] = noise_metadata_schedule_63_0_e877;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_64_0_e881: f64 = (params[115] * w[2]);
            let noise_metadata_schedule_64_0_e883: f64 = (noise_metadata_schedule_64_0_e881 * w[2]);
            let noise_metadata_schedule_64_0_e886: f64 = (w[2] + params[116]);
            let noise_metadata_schedule_64_0_e887: f64 = (noise_metadata_schedule_64_0_e883 / noise_metadata_schedule_64_0_e886);
            let noise_metadata_schedule_64_0_e888: f64 = (w[74] - noise_metadata_schedule_64_0_e887);
            let noise_metadata_schedule_64_0_e890: f64 = if noise_metadata_schedule_64_0_e888 < 0.05 { 1.0 } else { 0.0 };
            w[481] = noise_metadata_schedule_64_0_e890;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_65_0_e902,) = {
    if (w[481] != 0.0) {
        let noise_metadata_schedule_65_0_e896: f64 = (w[279]).exp();
        let noise_metadata_schedule_65_0_e897: f64 = (1.0 + noise_metadata_schedule_65_0_e896);
        let noise_metadata_schedule_65_0_e898: f64 = (noise_metadata_schedule_65_0_e897).ln();
        let noise_metadata_schedule_65_0_e899: f64 = (0.1 * noise_metadata_schedule_65_0_e898);
        let noise_metadata_schedule_65_0_e900: f64 = (0.05 + noise_metadata_schedule_65_0_e899);
        (noise_metadata_schedule_65_0_e900,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_65_0_e902;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_66_0_e926,) = {
    if (w[481] == 0.0) {
        let noise_metadata_schedule_66_0_e908: f64 = (params[115] * w[2]);
        let noise_metadata_schedule_66_0_e910: f64 = (noise_metadata_schedule_66_0_e908 * w[2]);
        let noise_metadata_schedule_66_0_e913: f64 = (w[2] + params[116]);
        let noise_metadata_schedule_66_0_e914: f64 = (noise_metadata_schedule_66_0_e910 / noise_metadata_schedule_66_0_e913);
        let noise_metadata_schedule_66_0_e915: f64 = (w[74] - noise_metadata_schedule_66_0_e914);
        let noise_metadata_schedule_66_0_e919: f64 = (-w[279]);
        let noise_metadata_schedule_66_0_e920: f64 = (noise_metadata_schedule_66_0_e919).exp();
        let noise_metadata_schedule_66_0_e921: f64 = (1.0 + noise_metadata_schedule_66_0_e920);
        let noise_metadata_schedule_66_0_e922: f64 = (noise_metadata_schedule_66_0_e921).ln();
        let noise_metadata_schedule_66_0_e923: f64 = (0.1 * noise_metadata_schedule_66_0_e922);
        let noise_metadata_schedule_66_0_e924: f64 = (noise_metadata_schedule_66_0_e915 + noise_metadata_schedule_66_0_e923);
        (noise_metadata_schedule_66_0_e924,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_66_0_e926;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_67_0_e930: f64 = (params[118] * w[2]);
            let noise_metadata_schedule_67_0_e932: f64 = (noise_metadata_schedule_67_0_e930 * w[2]);
            let noise_metadata_schedule_67_0_e935: f64 = (w[2] + params[119]);
            let noise_metadata_schedule_67_0_e936: f64 = (noise_metadata_schedule_67_0_e932 / noise_metadata_schedule_67_0_e935);
            let noise_metadata_schedule_67_0_e937: f64 = (w[88] - noise_metadata_schedule_67_0_e936);
            let noise_metadata_schedule_67_0_e939: f64 = (noise_metadata_schedule_67_0_e937 - 0.05);
            let noise_metadata_schedule_67_0_e941: f64 = (noise_metadata_schedule_67_0_e939 / 0.1);
            w[279] = noise_metadata_schedule_67_0_e941;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_68_0_e945: f64 = (params[118] * w[2]);
            let noise_metadata_schedule_68_0_e947: f64 = (noise_metadata_schedule_68_0_e945 * w[2]);
            let noise_metadata_schedule_68_0_e950: f64 = (w[2] + params[119]);
            let noise_metadata_schedule_68_0_e951: f64 = (noise_metadata_schedule_68_0_e947 / noise_metadata_schedule_68_0_e950);
            let noise_metadata_schedule_68_0_e952: f64 = (w[88] - noise_metadata_schedule_68_0_e951);
            let noise_metadata_schedule_68_0_e954: f64 = if noise_metadata_schedule_68_0_e952 < 0.05 { 1.0 } else { 0.0 };
            w[482] = noise_metadata_schedule_68_0_e954;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_69_0_e966,) = {
    if (w[482] != 0.0) {
        let noise_metadata_schedule_69_0_e960: f64 = (w[279]).exp();
        let noise_metadata_schedule_69_0_e961: f64 = (1.0 + noise_metadata_schedule_69_0_e960);
        let noise_metadata_schedule_69_0_e962: f64 = (noise_metadata_schedule_69_0_e961).ln();
        let noise_metadata_schedule_69_0_e963: f64 = (0.1 * noise_metadata_schedule_69_0_e962);
        let noise_metadata_schedule_69_0_e964: f64 = (0.05 + noise_metadata_schedule_69_0_e963);
        (noise_metadata_schedule_69_0_e964,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_69_0_e966;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_70_0_e990,) = {
    if (w[482] == 0.0) {
        let noise_metadata_schedule_70_0_e972: f64 = (params[118] * w[2]);
        let noise_metadata_schedule_70_0_e974: f64 = (noise_metadata_schedule_70_0_e972 * w[2]);
        let noise_metadata_schedule_70_0_e977: f64 = (w[2] + params[119]);
        let noise_metadata_schedule_70_0_e978: f64 = (noise_metadata_schedule_70_0_e974 / noise_metadata_schedule_70_0_e977);
        let noise_metadata_schedule_70_0_e979: f64 = (w[88] - noise_metadata_schedule_70_0_e978);
        let noise_metadata_schedule_70_0_e983: f64 = (-w[279]);
        let noise_metadata_schedule_70_0_e984: f64 = (noise_metadata_schedule_70_0_e983).exp();
        let noise_metadata_schedule_70_0_e985: f64 = (1.0 + noise_metadata_schedule_70_0_e984);
        let noise_metadata_schedule_70_0_e986: f64 = (noise_metadata_schedule_70_0_e985).ln();
        let noise_metadata_schedule_70_0_e987: f64 = (0.1 * noise_metadata_schedule_70_0_e986);
        let noise_metadata_schedule_70_0_e988: f64 = (noise_metadata_schedule_70_0_e979 + noise_metadata_schedule_70_0_e987);
        (noise_metadata_schedule_70_0_e988,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_70_0_e990;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_71_0_e992: f64 = (-3.0);
            let noise_metadata_schedule_71_0_e994: f64 = (noise_metadata_schedule_71_0_e992 * w[6]);
            let noise_metadata_schedule_71_0_e996: f64 = (noise_metadata_schedule_71_0_e994 * w[274]);
            let noise_metadata_schedule_71_0_e999: f64 = (params[66] * w[4]);
            let noise_metadata_schedule_71_0_e1000: f64 = (noise_metadata_schedule_71_0_e996 + noise_metadata_schedule_71_0_e999);
            let noise_metadata_schedule_71_0_e1003: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_71_0_e1005: f64 = (noise_metadata_schedule_71_0_e1003 * params[105]);
            let noise_metadata_schedule_71_0_e1006: f64 = (noise_metadata_schedule_71_0_e1000 + noise_metadata_schedule_71_0_e1005);
            w[13] = noise_metadata_schedule_71_0_e1006;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_72_0_e1009: f64 = (0.05 - w[13]);
            let noise_metadata_schedule_72_0_e1011: f64 = (noise_metadata_schedule_72_0_e1009 / w[6]);
            w[279] = noise_metadata_schedule_72_0_e1011;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_73_0_e1014: f64 = if 0.05 < w[13] { 1.0 } else { 0.0 };
            w[483] = noise_metadata_schedule_73_0_e1014;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_74_0_e1026,) = {
    if (w[483] != 0.0) {
        let noise_metadata_schedule_74_0_e1020: f64 = (w[279]).exp();
        let noise_metadata_schedule_74_0_e1021: f64 = (1.0 + noise_metadata_schedule_74_0_e1020);
        let noise_metadata_schedule_74_0_e1022: f64 = (noise_metadata_schedule_74_0_e1021).ln();
        let noise_metadata_schedule_74_0_e1023: f64 = (w[6] * noise_metadata_schedule_74_0_e1022);
        let noise_metadata_schedule_74_0_e1024: f64 = (w[13] + noise_metadata_schedule_74_0_e1023);
        (noise_metadata_schedule_74_0_e1024,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_74_0_e1026;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_75_0_e1040,) = {
    if (w[483] == 0.0) {
        let noise_metadata_schedule_75_0_e1033: f64 = (-w[279]);
        let noise_metadata_schedule_75_0_e1034: f64 = (noise_metadata_schedule_75_0_e1033).exp();
        let noise_metadata_schedule_75_0_e1035: f64 = (1.0 + noise_metadata_schedule_75_0_e1034);
        let noise_metadata_schedule_75_0_e1036: f64 = (noise_metadata_schedule_75_0_e1035).ln();
        let noise_metadata_schedule_75_0_e1037: f64 = (w[6] * noise_metadata_schedule_75_0_e1036);
        let noise_metadata_schedule_75_0_e1038: f64 = (0.05 + noise_metadata_schedule_75_0_e1037);
        (noise_metadata_schedule_75_0_e1038,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_75_0_e1040;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_76_0_e1042: f64 = (-3.0);
            let noise_metadata_schedule_76_0_e1044: f64 = (noise_metadata_schedule_76_0_e1042 * w[6]);
            let noise_metadata_schedule_76_0_e1046: f64 = (noise_metadata_schedule_76_0_e1044 * w[274]);
            let noise_metadata_schedule_76_0_e1049: f64 = (params[64] * w[4]);
            let noise_metadata_schedule_76_0_e1050: f64 = (noise_metadata_schedule_76_0_e1046 + noise_metadata_schedule_76_0_e1049);
            let noise_metadata_schedule_76_0_e1053: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_76_0_e1055: f64 = (noise_metadata_schedule_76_0_e1053 * params[110]);
            let noise_metadata_schedule_76_0_e1056: f64 = (noise_metadata_schedule_76_0_e1050 + noise_metadata_schedule_76_0_e1055);
            w[15] = noise_metadata_schedule_76_0_e1056;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_77_0_e1059: f64 = (0.05 - w[15]);
            let noise_metadata_schedule_77_0_e1061: f64 = (noise_metadata_schedule_77_0_e1059 / w[6]);
            w[279] = noise_metadata_schedule_77_0_e1061;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_78_0_e1064: f64 = if 0.05 < w[15] { 1.0 } else { 0.0 };
            w[484] = noise_metadata_schedule_78_0_e1064;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_79_0_e1076,) = {
    if (w[484] != 0.0) {
        let noise_metadata_schedule_79_0_e1070: f64 = (w[279]).exp();
        let noise_metadata_schedule_79_0_e1071: f64 = (1.0 + noise_metadata_schedule_79_0_e1070);
        let noise_metadata_schedule_79_0_e1072: f64 = (noise_metadata_schedule_79_0_e1071).ln();
        let noise_metadata_schedule_79_0_e1073: f64 = (w[6] * noise_metadata_schedule_79_0_e1072);
        let noise_metadata_schedule_79_0_e1074: f64 = (w[15] + noise_metadata_schedule_79_0_e1073);
        (noise_metadata_schedule_79_0_e1074,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_79_0_e1076;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_80_0_e1090,) = {
    if (w[484] == 0.0) {
        let noise_metadata_schedule_80_0_e1083: f64 = (-w[279]);
        let noise_metadata_schedule_80_0_e1084: f64 = (noise_metadata_schedule_80_0_e1083).exp();
        let noise_metadata_schedule_80_0_e1085: f64 = (1.0 + noise_metadata_schedule_80_0_e1084);
        let noise_metadata_schedule_80_0_e1086: f64 = (noise_metadata_schedule_80_0_e1085).ln();
        let noise_metadata_schedule_80_0_e1087: f64 = (w[6] * noise_metadata_schedule_80_0_e1086);
        let noise_metadata_schedule_80_0_e1088: f64 = (0.05 + noise_metadata_schedule_80_0_e1087);
        (noise_metadata_schedule_80_0_e1088,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_80_0_e1090;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_86_0_e1142: f64 = (-3.0);
            let noise_metadata_schedule_86_0_e1144: f64 = (noise_metadata_schedule_86_0_e1142 * w[6]);
            let noise_metadata_schedule_86_0_e1146: f64 = (noise_metadata_schedule_86_0_e1144 * w[274]);
            let noise_metadata_schedule_86_0_e1149: f64 = (params[71] * w[4]);
            let noise_metadata_schedule_86_0_e1150: f64 = (noise_metadata_schedule_86_0_e1146 + noise_metadata_schedule_86_0_e1149);
            let noise_metadata_schedule_86_0_e1153: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_86_0_e1155: f64 = (noise_metadata_schedule_86_0_e1153 * params[110]);
            let noise_metadata_schedule_86_0_e1156: f64 = (noise_metadata_schedule_86_0_e1150 + noise_metadata_schedule_86_0_e1155);
            w[18] = noise_metadata_schedule_86_0_e1156;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_87_0_e1159: f64 = (0.05 - w[18]);
            let noise_metadata_schedule_87_0_e1161: f64 = (noise_metadata_schedule_87_0_e1159 / w[6]);
            w[279] = noise_metadata_schedule_87_0_e1161;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_88_0_e1164: f64 = if 0.05 < w[18] { 1.0 } else { 0.0 };
            w[486] = noise_metadata_schedule_88_0_e1164;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_89_0_e1176,) = {
    if (w[486] != 0.0) {
        let noise_metadata_schedule_89_0_e1170: f64 = (w[279]).exp();
        let noise_metadata_schedule_89_0_e1171: f64 = (1.0 + noise_metadata_schedule_89_0_e1170);
        let noise_metadata_schedule_89_0_e1172: f64 = (noise_metadata_schedule_89_0_e1171).ln();
        let noise_metadata_schedule_89_0_e1173: f64 = (w[6] * noise_metadata_schedule_89_0_e1172);
        let noise_metadata_schedule_89_0_e1174: f64 = (w[18] + noise_metadata_schedule_89_0_e1173);
        (noise_metadata_schedule_89_0_e1174,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_89_0_e1176;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_90_0_e1190,) = {
    if (w[486] == 0.0) {
        let noise_metadata_schedule_90_0_e1183: f64 = (-w[279]);
        let noise_metadata_schedule_90_0_e1184: f64 = (noise_metadata_schedule_90_0_e1183).exp();
        let noise_metadata_schedule_90_0_e1185: f64 = (1.0 + noise_metadata_schedule_90_0_e1184);
        let noise_metadata_schedule_90_0_e1186: f64 = (noise_metadata_schedule_90_0_e1185).ln();
        let noise_metadata_schedule_90_0_e1187: f64 = (w[6] * noise_metadata_schedule_90_0_e1186);
        let noise_metadata_schedule_90_0_e1188: f64 = (0.05 + noise_metadata_schedule_90_0_e1187);
        (noise_metadata_schedule_90_0_e1188,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_90_0_e1190;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_91_0_e1192: f64 = (-3.0);
            let noise_metadata_schedule_91_0_e1194: f64 = (noise_metadata_schedule_91_0_e1192 * w[6]);
            let noise_metadata_schedule_91_0_e1196: f64 = (noise_metadata_schedule_91_0_e1194 * w[274]);
            let noise_metadata_schedule_91_0_e1199: f64 = (w[75] * w[4]);
            let noise_metadata_schedule_91_0_e1200: f64 = (noise_metadata_schedule_91_0_e1196 + noise_metadata_schedule_91_0_e1199);
            let noise_metadata_schedule_91_0_e1203: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_91_0_e1205: f64 = (noise_metadata_schedule_91_0_e1203 * params[110]);
            let noise_metadata_schedule_91_0_e1206: f64 = (noise_metadata_schedule_91_0_e1200 + noise_metadata_schedule_91_0_e1205);
            w[20] = noise_metadata_schedule_91_0_e1206;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_92_0_e1209: f64 = (0.05 - w[20]);
            let noise_metadata_schedule_92_0_e1211: f64 = (noise_metadata_schedule_92_0_e1209 / w[6]);
            w[279] = noise_metadata_schedule_92_0_e1211;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_93_0_e1214: f64 = if 0.05 < w[20] { 1.0 } else { 0.0 };
            w[487] = noise_metadata_schedule_93_0_e1214;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_94_0_e1226,) = {
    if (w[487] != 0.0) {
        let noise_metadata_schedule_94_0_e1220: f64 = (w[279]).exp();
        let noise_metadata_schedule_94_0_e1221: f64 = (1.0 + noise_metadata_schedule_94_0_e1220);
        let noise_metadata_schedule_94_0_e1222: f64 = (noise_metadata_schedule_94_0_e1221).ln();
        let noise_metadata_schedule_94_0_e1223: f64 = (w[6] * noise_metadata_schedule_94_0_e1222);
        let noise_metadata_schedule_94_0_e1224: f64 = (w[20] + noise_metadata_schedule_94_0_e1223);
        (noise_metadata_schedule_94_0_e1224,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_94_0_e1226;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_95_0_e1240,) = {
    if (w[487] == 0.0) {
        let noise_metadata_schedule_95_0_e1233: f64 = (-w[279]);
        let noise_metadata_schedule_95_0_e1234: f64 = (noise_metadata_schedule_95_0_e1233).exp();
        let noise_metadata_schedule_95_0_e1235: f64 = (1.0 + noise_metadata_schedule_95_0_e1234);
        let noise_metadata_schedule_95_0_e1236: f64 = (noise_metadata_schedule_95_0_e1235).ln();
        let noise_metadata_schedule_95_0_e1237: f64 = (w[6] * noise_metadata_schedule_95_0_e1236);
        let noise_metadata_schedule_95_0_e1238: f64 = (0.05 + noise_metadata_schedule_95_0_e1237);
        (noise_metadata_schedule_95_0_e1238,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_95_0_e1240;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_96_0_e1242: f64 = (-3.0);
            let noise_metadata_schedule_96_0_e1244: f64 = (noise_metadata_schedule_96_0_e1242 * w[6]);
            let noise_metadata_schedule_96_0_e1246: f64 = (noise_metadata_schedule_96_0_e1244 * w[274]);
            let noise_metadata_schedule_96_0_e1249: f64 = (params[27] * w[4]);
            let noise_metadata_schedule_96_0_e1250: f64 = (noise_metadata_schedule_96_0_e1246 + noise_metadata_schedule_96_0_e1249);
            let noise_metadata_schedule_96_0_e1253: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_96_0_e1255: f64 = (noise_metadata_schedule_96_0_e1253 * params[109]);
            let noise_metadata_schedule_96_0_e1256: f64 = (noise_metadata_schedule_96_0_e1250 + noise_metadata_schedule_96_0_e1255);
            w[56] = noise_metadata_schedule_96_0_e1256;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_97_0_e1259: f64 = (0.05 - w[56]);
            let noise_metadata_schedule_97_0_e1261: f64 = (noise_metadata_schedule_97_0_e1259 / w[6]);
            w[279] = noise_metadata_schedule_97_0_e1261;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_98_0_e1264: f64 = if 0.05 < w[56] { 1.0 } else { 0.0 };
            w[488] = noise_metadata_schedule_98_0_e1264;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_99_0_e1276,) = {
    if (w[488] != 0.0) {
        let noise_metadata_schedule_99_0_e1270: f64 = (w[279]).exp();
        let noise_metadata_schedule_99_0_e1271: f64 = (1.0 + noise_metadata_schedule_99_0_e1270);
        let noise_metadata_schedule_99_0_e1272: f64 = (noise_metadata_schedule_99_0_e1271).ln();
        let noise_metadata_schedule_99_0_e1273: f64 = (w[6] * noise_metadata_schedule_99_0_e1272);
        let noise_metadata_schedule_99_0_e1274: f64 = (w[56] + noise_metadata_schedule_99_0_e1273);
        (noise_metadata_schedule_99_0_e1274,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_99_0_e1276;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_100_0_e1290,) = {
    if (w[488] == 0.0) {
        let noise_metadata_schedule_100_0_e1283: f64 = (-w[279]);
        let noise_metadata_schedule_100_0_e1284: f64 = (noise_metadata_schedule_100_0_e1283).exp();
        let noise_metadata_schedule_100_0_e1285: f64 = (1.0 + noise_metadata_schedule_100_0_e1284);
        let noise_metadata_schedule_100_0_e1286: f64 = (noise_metadata_schedule_100_0_e1285).ln();
        let noise_metadata_schedule_100_0_e1287: f64 = (w[6] * noise_metadata_schedule_100_0_e1286);
        let noise_metadata_schedule_100_0_e1288: f64 = (0.05 + noise_metadata_schedule_100_0_e1287);
        (noise_metadata_schedule_100_0_e1288,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_100_0_e1290;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_101_0_e1292: f64 = (-3.0);
            let noise_metadata_schedule_101_0_e1294: f64 = (noise_metadata_schedule_101_0_e1292 * w[6]);
            let noise_metadata_schedule_101_0_e1296: f64 = (noise_metadata_schedule_101_0_e1294 * w[274]);
            let noise_metadata_schedule_101_0_e1299: f64 = (params[138] * w[4]);
            let noise_metadata_schedule_101_0_e1300: f64 = (noise_metadata_schedule_101_0_e1296 + noise_metadata_schedule_101_0_e1299);
            let noise_metadata_schedule_101_0_e1303: f64 = (1.0 - w[4]);
            let noise_metadata_schedule_101_0_e1305: f64 = (noise_metadata_schedule_101_0_e1303 * params[140]);
            let noise_metadata_schedule_101_0_e1306: f64 = (noise_metadata_schedule_101_0_e1300 + noise_metadata_schedule_101_0_e1305);
            w[101] = noise_metadata_schedule_101_0_e1306;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_102_0_e1309: f64 = (0.05 - w[101]);
            let noise_metadata_schedule_102_0_e1311: f64 = (noise_metadata_schedule_102_0_e1309 / w[6]);
            w[279] = noise_metadata_schedule_102_0_e1311;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_106_0_e1343: f64 = (1.0 / w[14]);
            w[65] = noise_metadata_schedule_106_0_e1343;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_107_0_e1346: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_107_0_e1346;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_108_0_e1349: f64 = (params[66] * w[65]);
            let noise_metadata_schedule_108_0_e1351: f64 = (noise_metadata_schedule_108_0_e1349).powf(params[67]);
            w[73] = noise_metadata_schedule_108_0_e1351;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_109_0_e1354: f64 = (w[75] * w[67]);
            let noise_metadata_schedule_109_0_e1356: f64 = (noise_metadata_schedule_109_0_e1354).powf(w[76]);
            w[90] = noise_metadata_schedule_109_0_e1356;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_112_0_e1369: f64 = (1.0 - params[75]);
            let noise_metadata_schedule_112_0_e1372: f64 = (params[71] / w[17]);
            let noise_metadata_schedule_112_0_e1374: f64 = (noise_metadata_schedule_112_0_e1372).powf(params[72]);
            let noise_metadata_schedule_112_0_e1375: f64 = (noise_metadata_schedule_112_0_e1369 * noise_metadata_schedule_112_0_e1374);
            let noise_metadata_schedule_112_0_e1377: f64 = (noise_metadata_schedule_112_0_e1375 + params[75]);
            w[26] = noise_metadata_schedule_112_0_e1377;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_113_0_e1380: f64 = (1.0 / w[26]);
            w[27] = noise_metadata_schedule_113_0_e1380;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_115_0_e1386: f64 = (params[75] * w[27]);
            w[25] = noise_metadata_schedule_115_0_e1386;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_116_0_e1390: f64 = (w[274] * params[97]);
            let noise_metadata_schedule_116_0_e1391: f64 = (noise_metadata_schedule_116_0_e1390).exp();
            let noise_metadata_schedule_116_0_e1392: f64 = (params[54] * noise_metadata_schedule_116_0_e1391);
            w[28] = noise_metadata_schedule_116_0_e1392;
        }
        if (active[0] & 0xa) != 0 {
            let noise_metadata_schedule_117_0_e1395: f64 = if w[28] < w[340] { 1.0 } else { 0.0 };
            w[490] = noise_metadata_schedule_117_0_e1395;
        }
        if (active[0] & 0xa) != 0 {
            let (noise_metadata_schedule_118_0_e1399,) = {
    if (w[490] != 0.0) {
        (w[340],)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_118_0_e1399;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_119_0_e1404: f64 = (params[98] - params[96]);
            let noise_metadata_schedule_119_0_e1405: f64 = (w[274] * noise_metadata_schedule_119_0_e1404);
            let noise_metadata_schedule_119_0_e1406: f64 = (noise_metadata_schedule_119_0_e1405).exp();
            let noise_metadata_schedule_119_0_e1407: f64 = (params[56] * noise_metadata_schedule_119_0_e1406);
            w[29] = noise_metadata_schedule_119_0_e1407;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_120_0_e1411: f64 = (w[274] * params[101]);
            let noise_metadata_schedule_120_0_e1412: f64 = (noise_metadata_schedule_120_0_e1411).exp();
            let noise_metadata_schedule_120_0_e1413: f64 = (params[55] * noise_metadata_schedule_120_0_e1412);
            w[30] = noise_metadata_schedule_120_0_e1413;
        }
        if (active[0] & 0x12) != 0 {
            let noise_metadata_schedule_121_0_e1416: f64 = if w[30] < w[340] { 1.0 } else { 0.0 };
            w[491] = noise_metadata_schedule_121_0_e1416;
        }
        if (active[0] & 0x12) != 0 {
            let (noise_metadata_schedule_122_0_e1420,) = {
    if (w[491] != 0.0) {
        (w[340],)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_122_0_e1420;
        }
        if (active[0] & 0xa99fe00) != 0 {
            let noise_metadata_schedule_123_0_e1424: f64 = (w[274] * params[102]);
            let noise_metadata_schedule_123_0_e1425: f64 = (noise_metadata_schedule_123_0_e1424).exp();
            let noise_metadata_schedule_123_0_e1426: f64 = (params[57] * noise_metadata_schedule_123_0_e1425);
            w[32] = noise_metadata_schedule_123_0_e1426;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_124_0_e1430: f64 = (w[274] * params[104]);
            let noise_metadata_schedule_124_0_e1431: f64 = (noise_metadata_schedule_124_0_e1430).exp();
            let noise_metadata_schedule_124_0_e1432: f64 = (params[58] * noise_metadata_schedule_124_0_e1431);
            w[33] = noise_metadata_schedule_124_0_e1432;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_125_0_e1436: f64 = (w[274] * params[104]);
            let noise_metadata_schedule_125_0_e1437: f64 = (noise_metadata_schedule_125_0_e1436).exp();
            let noise_metadata_schedule_125_0_e1438: f64 = (params[59] * noise_metadata_schedule_125_0_e1437);
            w[34] = noise_metadata_schedule_125_0_e1438;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_126_0_e1442: f64 = (w[274] * params[99]);
            let noise_metadata_schedule_126_0_e1443: f64 = (noise_metadata_schedule_126_0_e1442).exp();
            let noise_metadata_schedule_126_0_e1444: f64 = (params[60] * noise_metadata_schedule_126_0_e1443);
            w[31] = noise_metadata_schedule_126_0_e1444;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_127_0_e1447: f64 = if params[122] != 0.0 { 1.0 } else { 0.0 };
            w[492] = noise_metadata_schedule_127_0_e1447;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_128_0_e1457,) = {
    if (w[492] != 0.0) {
        let noise_metadata_schedule_128_0_e1453: f64 = (w[12] * params[122]);
        let noise_metadata_schedule_128_0_e1454: f64 = (1.0 + noise_metadata_schedule_128_0_e1453);
        let noise_metadata_schedule_128_0_e1455: f64 = (params[10] * noise_metadata_schedule_128_0_e1454);
        (noise_metadata_schedule_128_0_e1455,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_128_0_e1457;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_129_0_e1465,) = {
    if (w[492] != 0.0) {
        let noise_metadata_schedule_129_0_e1461: f64 = (w[50] - 1.0);
        let noise_metadata_schedule_129_0_e1463: f64 = (noise_metadata_schedule_129_0_e1461 / w[52]);
        (noise_metadata_schedule_129_0_e1463,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_129_0_e1465;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_130_0_e1468: f64 = if w[50] < 1.0 { 1.0 } else { 0.0 };
            w[493] = noise_metadata_schedule_130_0_e1468;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_131_0_e1482,) = {
    if ((w[492] != 0.0) && (w[493] != 0.0)) {
        let noise_metadata_schedule_131_0_e1476: f64 = (w[279]).exp();
        let noise_metadata_schedule_131_0_e1477: f64 = (1.0 + noise_metadata_schedule_131_0_e1476);
        let noise_metadata_schedule_131_0_e1478: f64 = (noise_metadata_schedule_131_0_e1477).ln();
        let noise_metadata_schedule_131_0_e1479: f64 = (w[52] * noise_metadata_schedule_131_0_e1478);
        let noise_metadata_schedule_131_0_e1480: f64 = (1.0 + noise_metadata_schedule_131_0_e1479);
        (noise_metadata_schedule_131_0_e1480,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_131_0_e1482;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_132_0_e1498,) = {
    if ((w[492] != 0.0) && (w[493] == 0.0)) {
        let noise_metadata_schedule_132_0_e1491: f64 = (-w[279]);
        let noise_metadata_schedule_132_0_e1492: f64 = (noise_metadata_schedule_132_0_e1491).exp();
        let noise_metadata_schedule_132_0_e1493: f64 = (1.0 + noise_metadata_schedule_132_0_e1492);
        let noise_metadata_schedule_132_0_e1494: f64 = (noise_metadata_schedule_132_0_e1493).ln();
        let noise_metadata_schedule_132_0_e1495: f64 = (w[52] * noise_metadata_schedule_132_0_e1494);
        let noise_metadata_schedule_132_0_e1496: f64 = (w[50] + noise_metadata_schedule_132_0_e1495);
        (noise_metadata_schedule_132_0_e1496,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_132_0_e1498;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_133_0_e1506,) = {
    if (w[492] != 0.0) {
        let noise_metadata_schedule_133_0_e1503: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_133_0_e1504: f64 = (w[50] - noise_metadata_schedule_133_0_e1503);
        (noise_metadata_schedule_133_0_e1504,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_133_0_e1506;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_134_0_e1511,) = {
    if (w[492] == 0.0) {
        (params[10],)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_134_0_e1511;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_135_0_e1514: f64 = if params[123] != 0.0 { 1.0 } else { 0.0 };
            w[494] = noise_metadata_schedule_135_0_e1514;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_136_0_e1524,) = {
    if (w[494] != 0.0) {
        let noise_metadata_schedule_136_0_e1520: f64 = (w[12] * params[123]);
        let noise_metadata_schedule_136_0_e1521: f64 = (1.0 + noise_metadata_schedule_136_0_e1520);
        let noise_metadata_schedule_136_0_e1522: f64 = (params[11] * noise_metadata_schedule_136_0_e1521);
        (noise_metadata_schedule_136_0_e1522,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_136_0_e1524;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_137_0_e1532,) = {
    if (w[494] != 0.0) {
        let noise_metadata_schedule_137_0_e1528: f64 = (w[51] - 1.0);
        let noise_metadata_schedule_137_0_e1530: f64 = (noise_metadata_schedule_137_0_e1528 / w[52]);
        (noise_metadata_schedule_137_0_e1530,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_137_0_e1532;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_138_0_e1535: f64 = if w[51] < 1.0 { 1.0 } else { 0.0 };
            w[495] = noise_metadata_schedule_138_0_e1535;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_139_0_e1549,) = {
    if ((w[494] != 0.0) && (w[495] != 0.0)) {
        let noise_metadata_schedule_139_0_e1543: f64 = (w[279]).exp();
        let noise_metadata_schedule_139_0_e1544: f64 = (1.0 + noise_metadata_schedule_139_0_e1543);
        let noise_metadata_schedule_139_0_e1545: f64 = (noise_metadata_schedule_139_0_e1544).ln();
        let noise_metadata_schedule_139_0_e1546: f64 = (w[52] * noise_metadata_schedule_139_0_e1545);
        let noise_metadata_schedule_139_0_e1547: f64 = (1.0 + noise_metadata_schedule_139_0_e1546);
        (noise_metadata_schedule_139_0_e1547,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_139_0_e1549;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_140_0_e1565,) = {
    if ((w[494] != 0.0) && (w[495] == 0.0)) {
        let noise_metadata_schedule_140_0_e1558: f64 = (-w[279]);
        let noise_metadata_schedule_140_0_e1559: f64 = (noise_metadata_schedule_140_0_e1558).exp();
        let noise_metadata_schedule_140_0_e1560: f64 = (1.0 + noise_metadata_schedule_140_0_e1559);
        let noise_metadata_schedule_140_0_e1561: f64 = (noise_metadata_schedule_140_0_e1560).ln();
        let noise_metadata_schedule_140_0_e1562: f64 = (w[52] * noise_metadata_schedule_140_0_e1561);
        let noise_metadata_schedule_140_0_e1563: f64 = (w[51] + noise_metadata_schedule_140_0_e1562);
        (noise_metadata_schedule_140_0_e1563,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_140_0_e1565;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_141_0_e1573,) = {
    if (w[494] != 0.0) {
        let noise_metadata_schedule_141_0_e1570: f64 = (w[52] * 0.6931471805599453);
        let noise_metadata_schedule_141_0_e1571: f64 = (w[51] - noise_metadata_schedule_141_0_e1570);
        (noise_metadata_schedule_141_0_e1571,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_141_0_e1573;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_142_0_e1578,) = {
    if (w[494] == 0.0) {
        (params[11],)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_142_0_e1578;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_143_0_e1583: f64 = (params[124] * w[12]);
            let noise_metadata_schedule_143_0_e1584: f64 = (1.0 + noise_metadata_schedule_143_0_e1583);
            let noise_metadata_schedule_143_0_e1585: f64 = (params[43] * noise_metadata_schedule_143_0_e1584);
            w[335] = noise_metadata_schedule_143_0_e1585;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_144_0_e1588: f64 = (w[336] * w[336]);
            w[281] = noise_metadata_schedule_144_0_e1588;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_145_0_e1591: f64 = (w[335] * w[335]);
            w[282] = noise_metadata_schedule_145_0_e1591;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_146_0_e1594: f64 = if w[335] < 0.0 { 1.0 } else { 0.0 };
            w[496] = noise_metadata_schedule_146_0_e1594;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_147_0_e1607,) = {
    if (w[496] != 0.0) {
        let noise_metadata_schedule_147_0_e1598: f64 = (0.5 * w[281]);
        let noise_metadata_schedule_147_0_e1601: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_147_0_e1602: f64 = (noise_metadata_schedule_147_0_e1601).sqrt();
        let noise_metadata_schedule_147_0_e1604: f64 = (noise_metadata_schedule_147_0_e1602 - w[335]);
        let noise_metadata_schedule_147_0_e1605: f64 = (noise_metadata_schedule_147_0_e1598 / noise_metadata_schedule_147_0_e1604);
        (noise_metadata_schedule_147_0_e1605,)
    } else {
        (w[334],)
    }
};
            w[334] = noise_metadata_schedule_147_0_e1607;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_148_0_e1619,) = {
    if (w[496] == 0.0) {
        let noise_metadata_schedule_148_0_e1613: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_148_0_e1614: f64 = (noise_metadata_schedule_148_0_e1613).sqrt();
        let noise_metadata_schedule_148_0_e1616: f64 = (noise_metadata_schedule_148_0_e1614 + w[335]);
        let noise_metadata_schedule_148_0_e1617: f64 = (0.5 * noise_metadata_schedule_148_0_e1616);
        (noise_metadata_schedule_148_0_e1617,)
    } else {
        (w[334],)
    }
};
            w[334] = noise_metadata_schedule_148_0_e1619;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_149_0_e1624: f64 = (4.0 - params[98]);
            let noise_metadata_schedule_149_0_e1626: f64 = (noise_metadata_schedule_149_0_e1624 - params[96]);
            let noise_metadata_schedule_149_0_e1628: f64 = (noise_metadata_schedule_149_0_e1626 + params[121]);
            let noise_metadata_schedule_149_0_e1629: f64 = (w[274] * noise_metadata_schedule_149_0_e1628);
            let noise_metadata_schedule_149_0_e1631: f64 = (noise_metadata_schedule_149_0_e1629 / w[48]);
            let noise_metadata_schedule_149_0_e1632: f64 = (noise_metadata_schedule_149_0_e1631).exp();
            let noise_metadata_schedule_149_0_e1633: f64 = (params[9] * noise_metadata_schedule_149_0_e1632);
            let noise_metadata_schedule_149_0_e1635: f64 = (-params[105]);
            let noise_metadata_schedule_149_0_e1637: f64 = (noise_metadata_schedule_149_0_e1635 * w[10]);
            let noise_metadata_schedule_149_0_e1639: f64 = (noise_metadata_schedule_149_0_e1637 / w[48]);
            let noise_metadata_schedule_149_0_e1640: f64 = (noise_metadata_schedule_149_0_e1639).exp();
            let noise_metadata_schedule_149_0_e1641: f64 = (noise_metadata_schedule_149_0_e1633 * noise_metadata_schedule_149_0_e1640);
            w[35] = noise_metadata_schedule_149_0_e1641;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_150_0_e1646: f64 = (1.0 - params[98]);
            let noise_metadata_schedule_150_0_e1647: f64 = (w[274] * noise_metadata_schedule_150_0_e1646);
            let noise_metadata_schedule_150_0_e1648: f64 = (noise_metadata_schedule_150_0_e1647).exp();
            let noise_metadata_schedule_150_0_e1649: f64 = (params[12] * noise_metadata_schedule_150_0_e1648);
            w[36] = noise_metadata_schedule_150_0_e1649;
        }
        if (active[0] & 0x87800) != 0 {
            let noise_metadata_schedule_151_0_e1654: f64 = (1.0 - params[103]);
            let noise_metadata_schedule_151_0_e1655: f64 = (w[274] * noise_metadata_schedule_151_0_e1654);
            let noise_metadata_schedule_151_0_e1656: f64 = (noise_metadata_schedule_151_0_e1655).exp();
            let noise_metadata_schedule_151_0_e1657: f64 = (params[30] * noise_metadata_schedule_151_0_e1656);
            w[37] = noise_metadata_schedule_151_0_e1657;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_152_0_e1663: f64 = (2.0 * params[21]);
            let noise_metadata_schedule_152_0_e1664: f64 = (6.0 - noise_metadata_schedule_152_0_e1663);
            let noise_metadata_schedule_152_0_e1665: f64 = (w[274] * noise_metadata_schedule_152_0_e1664);
            let noise_metadata_schedule_152_0_e1666: f64 = (noise_metadata_schedule_152_0_e1665).exp();
            let noise_metadata_schedule_152_0_e1667: f64 = (params[20] * noise_metadata_schedule_152_0_e1666);
            let noise_metadata_schedule_152_0_e1669: f64 = (-params[113]);
            let noise_metadata_schedule_152_0_e1671: f64 = (noise_metadata_schedule_152_0_e1669 * w[10]);
            let noise_metadata_schedule_152_0_e1673: f64 = (noise_metadata_schedule_152_0_e1671 / params[21]);
            let noise_metadata_schedule_152_0_e1674: f64 = (noise_metadata_schedule_152_0_e1673).exp();
            let noise_metadata_schedule_152_0_e1675: f64 = (noise_metadata_schedule_152_0_e1667 * noise_metadata_schedule_152_0_e1674);
            w[38] = noise_metadata_schedule_152_0_e1675;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_153_0_e1681: f64 = (2.0 * params[32]);
            let noise_metadata_schedule_153_0_e1682: f64 = (6.0 - noise_metadata_schedule_153_0_e1681);
            let noise_metadata_schedule_153_0_e1683: f64 = (w[274] * noise_metadata_schedule_153_0_e1682);
            let noise_metadata_schedule_153_0_e1684: f64 = (noise_metadata_schedule_153_0_e1683).exp();
            let noise_metadata_schedule_153_0_e1685: f64 = (params[31] * noise_metadata_schedule_153_0_e1684);
            let noise_metadata_schedule_153_0_e1687: f64 = (-params[110]);
            let noise_metadata_schedule_153_0_e1689: f64 = (noise_metadata_schedule_153_0_e1687 * w[10]);
            let noise_metadata_schedule_153_0_e1691: f64 = (noise_metadata_schedule_153_0_e1689 / params[32]);
            let noise_metadata_schedule_153_0_e1692: f64 = (noise_metadata_schedule_153_0_e1691).exp();
            let noise_metadata_schedule_153_0_e1693: f64 = (noise_metadata_schedule_153_0_e1685 * noise_metadata_schedule_153_0_e1692);
            w[39] = noise_metadata_schedule_153_0_e1693;
        }
        if (active[0] & 0x46) != 0 {
            let noise_metadata_schedule_154_0_e1698: f64 = (4.0 - params[97]);
            let noise_metadata_schedule_154_0_e1700: f64 = (noise_metadata_schedule_154_0_e1698 + params[121]);
            let noise_metadata_schedule_154_0_e1701: f64 = (w[274] * noise_metadata_schedule_154_0_e1700);
            let noise_metadata_schedule_154_0_e1703: f64 = (noise_metadata_schedule_154_0_e1701 / params[17]);
            let noise_metadata_schedule_154_0_e1704: f64 = (noise_metadata_schedule_154_0_e1703).exp();
            let noise_metadata_schedule_154_0_e1705: f64 = (params[16] * noise_metadata_schedule_154_0_e1704);
            let noise_metadata_schedule_154_0_e1707: f64 = (-params[111]);
            let noise_metadata_schedule_154_0_e1709: f64 = (noise_metadata_schedule_154_0_e1707 * w[10]);
            let noise_metadata_schedule_154_0_e1711: f64 = (noise_metadata_schedule_154_0_e1709 / params[17]);
            let noise_metadata_schedule_154_0_e1712: f64 = (noise_metadata_schedule_154_0_e1711).exp();
            let noise_metadata_schedule_154_0_e1713: f64 = (noise_metadata_schedule_154_0_e1705 * noise_metadata_schedule_154_0_e1712);
            w[42] = noise_metadata_schedule_154_0_e1713;
        }
        if (active[0] & 0x140) != 0 {
            let noise_metadata_schedule_155_0_e1718: f64 = (4.0 - params[97]);
            let noise_metadata_schedule_155_0_e1720: f64 = (noise_metadata_schedule_155_0_e1718 + params[121]);
            let noise_metadata_schedule_155_0_e1721: f64 = (w[274] * noise_metadata_schedule_155_0_e1720);
            let noise_metadata_schedule_155_0_e1723: f64 = (noise_metadata_schedule_155_0_e1721 / params[19]);
            let noise_metadata_schedule_155_0_e1724: f64 = (noise_metadata_schedule_155_0_e1723).exp();
            let noise_metadata_schedule_155_0_e1725: f64 = (params[18] * noise_metadata_schedule_155_0_e1724);
            let noise_metadata_schedule_155_0_e1727: f64 = (-params[111]);
            let noise_metadata_schedule_155_0_e1729: f64 = (noise_metadata_schedule_155_0_e1727 * w[10]);
            let noise_metadata_schedule_155_0_e1731: f64 = (noise_metadata_schedule_155_0_e1729 / params[19]);
            let noise_metadata_schedule_155_0_e1732: f64 = (noise_metadata_schedule_155_0_e1731).exp();
            let noise_metadata_schedule_155_0_e1733: f64 = (noise_metadata_schedule_155_0_e1725 * noise_metadata_schedule_155_0_e1732);
            w[44] = noise_metadata_schedule_155_0_e1733;
        }
        if (active[0] & 0x144) != 0 {
            let noise_metadata_schedule_156_0_e1736: f64 = if params[24] == 1.0 { 1.0 } else { 0.0 };
            w[497] = noise_metadata_schedule_156_0_e1736;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_157_0_e1748,) = {
    if (w[497] != 0.0) {
        let noise_metadata_schedule_157_0_e1740: f64 = (-params[107]);
        let noise_metadata_schedule_157_0_e1742: f64 = (noise_metadata_schedule_157_0_e1740 * w[10]);
        let noise_metadata_schedule_157_0_e1744: f64 = (noise_metadata_schedule_157_0_e1742 / params[17]);
        let noise_metadata_schedule_157_0_e1745: f64 = (noise_metadata_schedule_157_0_e1744).exp();
        let noise_metadata_schedule_157_0_e1746: f64 = (params[25] * noise_metadata_schedule_157_0_e1745);
        (noise_metadata_schedule_157_0_e1746,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_157_0_e1748;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_158_0_e1758,) = {
    if (w[497] != 0.0) {
        let noise_metadata_schedule_158_0_e1752: f64 = (-params[106]);
        let noise_metadata_schedule_158_0_e1754: f64 = (noise_metadata_schedule_158_0_e1752 * w[10]);
        let noise_metadata_schedule_158_0_e1755: f64 = (noise_metadata_schedule_158_0_e1754).exp();
        let noise_metadata_schedule_158_0_e1756: f64 = (params[28] * noise_metadata_schedule_158_0_e1755);
        (noise_metadata_schedule_158_0_e1756,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_158_0_e1758;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_159_0_e1770,) = {
    if (w[497] != 0.0) {
        let noise_metadata_schedule_159_0_e1762: f64 = (-params[108]);
        let noise_metadata_schedule_159_0_e1764: f64 = (noise_metadata_schedule_159_0_e1762 * w[10]);
        let noise_metadata_schedule_159_0_e1766: f64 = (noise_metadata_schedule_159_0_e1764 / params[19]);
        let noise_metadata_schedule_159_0_e1767: f64 = (noise_metadata_schedule_159_0_e1766).exp();
        let noise_metadata_schedule_159_0_e1768: f64 = (params[26] * noise_metadata_schedule_159_0_e1767);
        (noise_metadata_schedule_159_0_e1768,)
    } else {
        (w[45],)
    }
};
            w[45] = noise_metadata_schedule_159_0_e1770;
        }
        if (active[0] & 0x9fe00) != 0 {
            let noise_metadata_schedule_160_0_e1775: f64 = (4.0 - params[103]);
            let noise_metadata_schedule_160_0_e1777: f64 = (noise_metadata_schedule_160_0_e1775 + params[121]);
            let noise_metadata_schedule_160_0_e1778: f64 = (w[274] * noise_metadata_schedule_160_0_e1777);
            let noise_metadata_schedule_160_0_e1779: f64 = (noise_metadata_schedule_160_0_e1778).exp();
            let noise_metadata_schedule_160_0_e1780: f64 = (params[29] * noise_metadata_schedule_160_0_e1779);
            let noise_metadata_schedule_160_0_e1782: f64 = (-params[112]);
            let noise_metadata_schedule_160_0_e1784: f64 = (noise_metadata_schedule_160_0_e1782 * w[10]);
            let noise_metadata_schedule_160_0_e1785: f64 = (noise_metadata_schedule_160_0_e1784).exp();
            let noise_metadata_schedule_160_0_e1786: f64 = (noise_metadata_schedule_160_0_e1780 * noise_metadata_schedule_160_0_e1785);
            w[43] = noise_metadata_schedule_160_0_e1786;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_161_0_e1792: f64 = (2.0 * params[23]);
            let noise_metadata_schedule_161_0_e1793: f64 = (6.0 - noise_metadata_schedule_161_0_e1792);
            let noise_metadata_schedule_161_0_e1794: f64 = (w[274] * noise_metadata_schedule_161_0_e1793);
            let noise_metadata_schedule_161_0_e1795: f64 = (noise_metadata_schedule_161_0_e1794).exp();
            let noise_metadata_schedule_161_0_e1796: f64 = (params[22] * noise_metadata_schedule_161_0_e1795);
            let noise_metadata_schedule_161_0_e1798: f64 = (-params[113]);
            let noise_metadata_schedule_161_0_e1800: f64 = (noise_metadata_schedule_161_0_e1798 * w[10]);
            let noise_metadata_schedule_161_0_e1802: f64 = (noise_metadata_schedule_161_0_e1800 / params[23]);
            let noise_metadata_schedule_161_0_e1803: f64 = (noise_metadata_schedule_161_0_e1802).exp();
            let noise_metadata_schedule_161_0_e1804: f64 = (noise_metadata_schedule_161_0_e1796 * noise_metadata_schedule_161_0_e1803);
            w[46] = noise_metadata_schedule_161_0_e1804;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_162_0_e1809: f64 = (4.0 / params[146]);
            let noise_metadata_schedule_162_0_e1810: f64 = (w[274] * noise_metadata_schedule_162_0_e1809);
            let noise_metadata_schedule_162_0_e1811: f64 = (noise_metadata_schedule_162_0_e1810).exp();
            let noise_metadata_schedule_162_0_e1812: f64 = (params[145] * noise_metadata_schedule_162_0_e1811);
            let noise_metadata_schedule_162_0_e1814: f64 = (-params[113]);
            let noise_metadata_schedule_162_0_e1816: f64 = (noise_metadata_schedule_162_0_e1814 * w[10]);
            let noise_metadata_schedule_162_0_e1818: f64 = (noise_metadata_schedule_162_0_e1816 / params[146]);
            let noise_metadata_schedule_162_0_e1819: f64 = (noise_metadata_schedule_162_0_e1818).exp();
            let noise_metadata_schedule_162_0_e1820: f64 = (noise_metadata_schedule_162_0_e1812 * noise_metadata_schedule_162_0_e1819);
            w[47] = noise_metadata_schedule_162_0_e1820;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_163_0_e1823: f64 = (w[4]).sqrt();
            let noise_metadata_schedule_163_0_e1824: f64 = (params[151] * noise_metadata_schedule_163_0_e1823);
            let noise_metadata_schedule_163_0_e1827: f64 = (params[153] * w[12]);
            let noise_metadata_schedule_163_0_e1828: f64 = (noise_metadata_schedule_163_0_e1827).exp();
            let noise_metadata_schedule_163_0_e1829: f64 = (noise_metadata_schedule_163_0_e1824 * noise_metadata_schedule_163_0_e1828);
            w[350] = noise_metadata_schedule_163_0_e1829;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_164_0_e1832: f64 = (w[70] * w[72]);
            let noise_metadata_schedule_164_0_e1834: f64 = (-0.5);
            let noise_metadata_schedule_164_0_e1835: f64 = (noise_metadata_schedule_164_0_e1832).powf(noise_metadata_schedule_164_0_e1834);
            w[275] = noise_metadata_schedule_164_0_e1835;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_165_0_e1838: f64 = (1.0 / w[73]);
            w[276] = noise_metadata_schedule_165_0_e1838;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_166_0_e1841: f64 = (params[35] * w[70]);
            let noise_metadata_schedule_166_0_e1843: f64 = (noise_metadata_schedule_166_0_e1841 * w[70]);
            let noise_metadata_schedule_166_0_e1845: f64 = (noise_metadata_schedule_166_0_e1843 * w[275]);
            let noise_metadata_schedule_166_0_e1847: f64 = (noise_metadata_schedule_166_0_e1845 * w[276]);
            let noise_metadata_schedule_166_0_e1849: f64 = (noise_metadata_schedule_166_0_e1847 * params[66]);
            let noise_metadata_schedule_166_0_e1851: f64 = (noise_metadata_schedule_166_0_e1849 * w[65]);
            let noise_metadata_schedule_166_0_e1853: f64 = (noise_metadata_schedule_166_0_e1851 * w[72]);
            let noise_metadata_schedule_166_0_e1855: f64 = (noise_metadata_schedule_166_0_e1853 * w[72]);
            w[61] = noise_metadata_schedule_166_0_e1855;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_167_0_e1858: f64 = (params[34] * w[275]);
            let noise_metadata_schedule_167_0_e1860: f64 = (noise_metadata_schedule_167_0_e1858 * w[14]);
            let noise_metadata_schedule_167_0_e1862: f64 = (noise_metadata_schedule_167_0_e1860 * w[14]);
            let noise_metadata_schedule_167_0_e1864: f64 = (noise_metadata_schedule_167_0_e1862 * w[64]);
            let noise_metadata_schedule_167_0_e1866: f64 = (noise_metadata_schedule_167_0_e1864 * w[64]);
            let noise_metadata_schedule_167_0_e1868: f64 = (noise_metadata_schedule_167_0_e1866 * w[73]);
            let noise_metadata_schedule_167_0_e1871: f64 = (params[35] - w[61]);
            let noise_metadata_schedule_167_0_e1872: f64 = (noise_metadata_schedule_167_0_e1871).exp();
            let noise_metadata_schedule_167_0_e1873: f64 = (noise_metadata_schedule_167_0_e1868 * noise_metadata_schedule_167_0_e1872);
            w[58] = noise_metadata_schedule_167_0_e1873;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_168_0_e1876: f64 = (1.0 / w[19]);
            w[67] = noise_metadata_schedule_168_0_e1876;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_169_0_e1879: f64 = (w[85] * w[86]);
            let noise_metadata_schedule_169_0_e1881: f64 = (-0.5);
            let noise_metadata_schedule_169_0_e1882: f64 = (noise_metadata_schedule_169_0_e1879).powf(noise_metadata_schedule_169_0_e1881);
            w[277] = noise_metadata_schedule_169_0_e1882;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_170_0_e1885: f64 = (1.0 / w[90]);
            w[278] = noise_metadata_schedule_170_0_e1885;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_171_0_e1888: f64 = (params[37] * w[85]);
            let noise_metadata_schedule_171_0_e1890: f64 = (noise_metadata_schedule_171_0_e1888 * w[85]);
            let noise_metadata_schedule_171_0_e1892: f64 = (noise_metadata_schedule_171_0_e1890 * w[277]);
            let noise_metadata_schedule_171_0_e1894: f64 = (noise_metadata_schedule_171_0_e1892 * w[278]);
            let noise_metadata_schedule_171_0_e1896: f64 = (noise_metadata_schedule_171_0_e1894 * w[75]);
            let noise_metadata_schedule_171_0_e1898: f64 = (noise_metadata_schedule_171_0_e1896 * w[67]);
            let noise_metadata_schedule_171_0_e1900: f64 = (noise_metadata_schedule_171_0_e1898 * w[86]);
            let noise_metadata_schedule_171_0_e1902: f64 = (noise_metadata_schedule_171_0_e1900 * w[86]);
            w[83] = noise_metadata_schedule_171_0_e1902;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_172_0_e1905: f64 = (params[36] * w[277]);
            let noise_metadata_schedule_172_0_e1907: f64 = (noise_metadata_schedule_172_0_e1905 * w[19]);
            let noise_metadata_schedule_172_0_e1909: f64 = (noise_metadata_schedule_172_0_e1907 * w[19]);
            let noise_metadata_schedule_172_0_e1911: f64 = (noise_metadata_schedule_172_0_e1909 * w[66]);
            let noise_metadata_schedule_172_0_e1913: f64 = (noise_metadata_schedule_172_0_e1911 * w[66]);
            let noise_metadata_schedule_172_0_e1915: f64 = (noise_metadata_schedule_172_0_e1913 * w[90]);
            let noise_metadata_schedule_172_0_e1918: f64 = (params[37] - w[83]);
            let noise_metadata_schedule_172_0_e1919: f64 = (noise_metadata_schedule_172_0_e1918).exp();
            let noise_metadata_schedule_172_0_e1920: f64 = (noise_metadata_schedule_172_0_e1915 * noise_metadata_schedule_172_0_e1919);
            w[84] = noise_metadata_schedule_172_0_e1920;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_173_0_e1923: f64 = (w[274] * params[96]);
            let noise_metadata_schedule_173_0_e1924: f64 = (noise_metadata_schedule_173_0_e1923).exp();
            w[275] = noise_metadata_schedule_173_0_e1924;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_174_0_e1927: f64 = (params[14] * w[275]);
            let noise_metadata_schedule_174_0_e1929: f64 = (noise_metadata_schedule_174_0_e1927 * w[27]);
            w[40] = noise_metadata_schedule_174_0_e1929;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_175_0_e1932: f64 = (params[13] * w[275]);
            let noise_metadata_schedule_175_0_e1934: f64 = (noise_metadata_schedule_175_0_e1932 * w[276]);
            w[41] = noise_metadata_schedule_175_0_e1934;
        }
        if (active[0] & 0xffe00) != 0 {
            let noise_metadata_schedule_176_0_e1939: f64 = (4.0 - params[141]);
            let noise_metadata_schedule_176_0_e1940: f64 = (w[274] * noise_metadata_schedule_176_0_e1939);
            let noise_metadata_schedule_176_0_e1941: f64 = (noise_metadata_schedule_176_0_e1940).exp();
            let noise_metadata_schedule_176_0_e1942: f64 = (params[133] * noise_metadata_schedule_176_0_e1941);
            let noise_metadata_schedule_176_0_e1944: f64 = (-params[140]);
            let noise_metadata_schedule_176_0_e1946: f64 = (noise_metadata_schedule_176_0_e1944 * w[10]);
            let noise_metadata_schedule_176_0_e1947: f64 = (noise_metadata_schedule_176_0_e1946).exp();
            let noise_metadata_schedule_176_0_e1948: f64 = (noise_metadata_schedule_176_0_e1942 * noise_metadata_schedule_176_0_e1947);
            w[104] = noise_metadata_schedule_176_0_e1948;
        }
        if (active[0] & 0xe6000) != 0 {
            let noise_metadata_schedule_178_0_e1969: f64 = (1.0 - params[141]);
            let noise_metadata_schedule_178_0_e1970: f64 = (w[274] * noise_metadata_schedule_178_0_e1969);
            let noise_metadata_schedule_178_0_e1971: f64 = (noise_metadata_schedule_178_0_e1970).exp();
            let noise_metadata_schedule_178_0_e1972: f64 = (params[135] * noise_metadata_schedule_178_0_e1971);
            w[106] = noise_metadata_schedule_178_0_e1972;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_185_0_e2032: f64 = (w[2] - 300.0);
            w[100] = noise_metadata_schedule_185_0_e2032;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_186_0_e2035: f64 = if w[2] < 525.0 { 1.0 } else { 0.0 };
            w[498] = noise_metadata_schedule_186_0_e2035;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_187_0_e2051,) = {
    if (w[498] != 0.0) {
        let noise_metadata_schedule_187_0_e2041: f64 = (0.00072 * w[100]);
        let noise_metadata_schedule_187_0_e2042: f64 = (1.0 + noise_metadata_schedule_187_0_e2041);
        let noise_metadata_schedule_187_0_e2045: f64 = (1.6e-6 * w[100]);
        let noise_metadata_schedule_187_0_e2047: f64 = (noise_metadata_schedule_187_0_e2045 * w[100]);
        let noise_metadata_schedule_187_0_e2048: f64 = (noise_metadata_schedule_187_0_e2042 - noise_metadata_schedule_187_0_e2047);
        let noise_metadata_schedule_187_0_e2049: f64 = (w[1] * noise_metadata_schedule_187_0_e2048);
        (noise_metadata_schedule_187_0_e2049,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_187_0_e2051;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_188_0_e2058,) = {
    if (w[498] == 0.0) {
        let noise_metadata_schedule_188_0_e2056: f64 = (w[1] * 1.081);
        (noise_metadata_schedule_188_0_e2056,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_188_0_e2058;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_189_0_e2062: f64 = (w[274] * params[96]);
            let noise_metadata_schedule_189_0_e2063: f64 = (noise_metadata_schedule_189_0_e2062).exp();
            let noise_metadata_schedule_189_0_e2064: f64 = (params[92] * noise_metadata_schedule_189_0_e2063);
            w[99] = noise_metadata_schedule_189_0_e2064;
        }
        if (active[0] & 0xa900000) != 0 {
            let noise_metadata_schedule_190_0_e2067: f64 = if params[57] > 0.0 { 1.0 } else { 0.0 };
            w[499] = noise_metadata_schedule_190_0_e2067;
        }
        if (active[0] & 0xa900000) != 0 {
            let (noise_metadata_schedule_191_0_e2073,) = {
    if (w[499] != 0.0) {
        let noise_metadata_schedule_191_0_e2071: f64 = (1.0 / w[32]);
        (noise_metadata_schedule_191_0_e2071,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_191_0_e2073;
        }
        if (active[0] & 0xa900000) != 0 {
            let noise_metadata_schedule_192_0_e2076: f64 = if w[108] > w[341] { 1.0 } else { 0.0 };
            w[500] = noise_metadata_schedule_192_0_e2076;
        }
        if (active[0] & 0xa900000) != 0 {
            let (noise_metadata_schedule_193_0_e2082,) = {
    if ((w[499] != 0.0) && (w[500] != 0.0)) {
        (w[341],)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_193_0_e2082;
        }
        if (active[0] & 0xa900000) != 0 {
            let (noise_metadata_schedule_194_0_e2087,) = {
    if (w[499] == 0.0) {
        (0.0,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_194_0_e2087;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_195_0_e2090: f64 = if params[58] > 0.0 { 1.0 } else { 0.0 };
            w[501] = noise_metadata_schedule_195_0_e2090;
        }
        if (active[0] & 0x1200000) != 0 {
            let (noise_metadata_schedule_196_0_e2096,) = {
    if (w[501] != 0.0) {
        let noise_metadata_schedule_196_0_e2094: f64 = (1.0 / w[33]);
        (noise_metadata_schedule_196_0_e2094,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_196_0_e2096;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_197_0_e2099: f64 = if w[109] > w[341] { 1.0 } else { 0.0 };
            w[502] = noise_metadata_schedule_197_0_e2099;
        }
        if (active[0] & 0x1200000) != 0 {
            let (noise_metadata_schedule_198_0_e2105,) = {
    if ((w[501] != 0.0) && (w[502] != 0.0)) {
        (w[341],)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_198_0_e2105;
        }
        if (active[0] & 0x1200000) != 0 {
            let (noise_metadata_schedule_199_0_e2110,) = {
    if (w[501] == 0.0) {
        (0.0,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_199_0_e2110;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_200_0_e2113: f64 = if params[59] > 0.0 { 1.0 } else { 0.0 };
            w[503] = noise_metadata_schedule_200_0_e2113;
        }
        if (active[0] & 0x4400000) != 0 {
            let (noise_metadata_schedule_201_0_e2119,) = {
    if (w[503] != 0.0) {
        let noise_metadata_schedule_201_0_e2117: f64 = (1.0 / w[34]);
        (noise_metadata_schedule_201_0_e2117,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_201_0_e2119;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_202_0_e2122: f64 = if w[110] > w[341] { 1.0 } else { 0.0 };
            w[504] = noise_metadata_schedule_202_0_e2122;
        }
        if (active[0] & 0x4400000) != 0 {
            let (noise_metadata_schedule_203_0_e2128,) = {
    if ((w[503] != 0.0) && (w[504] != 0.0)) {
        (w[341],)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_203_0_e2128;
        }
        if (active[0] & 0x4400000) != 0 {
            let (noise_metadata_schedule_204_0_e2133,) = {
    if (w[503] == 0.0) {
        (0.0,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_204_0_e2133;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_205_0_e2136: f64 = (params[3] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            w[244] = noise_metadata_schedule_205_0_e2136;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_206_0_e2139: f64 = (params[3] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[8])));
            w[245] = noise_metadata_schedule_206_0_e2139;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_207_0_e2142: f64 = (params[3] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[4])));
            w[246] = noise_metadata_schedule_207_0_e2142;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_208_0_e2145: f64 = (params[3] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            w[247] = noise_metadata_schedule_208_0_e2145;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_209_0_e2148: f64 = (params[3] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            w[248] = noise_metadata_schedule_209_0_e2148;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_210_0_e2151: f64 = (params[3] * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[7])));
            w[253] = noise_metadata_schedule_210_0_e2151;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_211_0_e2154: f64 = (params[3] * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            w[250] = noise_metadata_schedule_211_0_e2154;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_213_0_e2160: f64 = (params[3] * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            w[260] = noise_metadata_schedule_213_0_e2160;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_215_0_e2166: f64 = (params[3] * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[0])));
            w[264] = noise_metadata_schedule_215_0_e2166;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_216_0_e2169: f64 = (params[3] * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[7])));
            w[252] = noise_metadata_schedule_216_0_e2169;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_217_0_e2172: f64 = (params[3] * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
            w[251] = noise_metadata_schedule_217_0_e2172;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_218_0_e2175: f64 = (w[248] + w[245]);
            let noise_metadata_schedule_218_0_e2177: f64 = (noise_metadata_schedule_218_0_e2175 - w[250]);
            let noise_metadata_schedule_218_0_e2179: f64 = (noise_metadata_schedule_218_0_e2177 - w[252]);
            w[249] = noise_metadata_schedule_218_0_e2179;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_219_0_e2181: f64 = (-w[264]);
            let noise_metadata_schedule_219_0_e2183: f64 = (noise_metadata_schedule_219_0_e2181 + w[260]);
            let noise_metadata_schedule_219_0_e2185: f64 = (noise_metadata_schedule_219_0_e2183 + w[249]);
            let noise_metadata_schedule_219_0_e2187: f64 = (noise_metadata_schedule_219_0_e2185 - w[251]);
            w[262] = noise_metadata_schedule_219_0_e2187;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_220_0_e2190: f64 = (w[264] + w[262]);
            w[261] = noise_metadata_schedule_220_0_e2190;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_221_0_e2193: f64 = (w[253] - w[252]);
            w[255] = noise_metadata_schedule_221_0_e2193;
        }
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_222_0_e2196: f64 = (w[255] - w[251]);
            w[254] = noise_metadata_schedule_222_0_e2196;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_223_0_e2199: f64 = (w[245] * w[8]);
            let noise_metadata_schedule_223_0_e2201: f64 = if noise_metadata_schedule_223_0_e2199 < params[147] { 1.0 } else { 0.0 };
            w[505] = noise_metadata_schedule_223_0_e2201;
        }
        if (active[0] & 0x387e7) != 0 {
            let (noise_metadata_schedule_224_0_e2208,) = {
    if (w[505] != 0.0) {
        let noise_metadata_schedule_224_0_e2205: f64 = (w[245] * w[8]);
        let noise_metadata_schedule_224_0_e2206: f64 = (noise_metadata_schedule_224_0_e2205).exp();
        (noise_metadata_schedule_224_0_e2206,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_224_0_e2208;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_225_0_e2214,) = {
    if (w[505] == 0.0) {
        let noise_metadata_schedule_225_0_e2212: f64 = (params[147]).exp();
        (noise_metadata_schedule_225_0_e2212,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_225_0_e2214;
        }
        if (active[0] & 0x387e7) != 0 {
            let (noise_metadata_schedule_226_0_e2227,) = {
    if (w[505] == 0.0) {
        let noise_metadata_schedule_226_0_e2221: f64 = (w[245] * w[8]);
        let noise_metadata_schedule_226_0_e2223: f64 = (noise_metadata_schedule_226_0_e2221 - params[147]);
        let noise_metadata_schedule_226_0_e2224: f64 = (1.0 + noise_metadata_schedule_226_0_e2223);
        let noise_metadata_schedule_226_0_e2225: f64 = (w[295] * noise_metadata_schedule_226_0_e2224);
        (noise_metadata_schedule_226_0_e2225,)
    } else {
        (w[265],)
    }
};
            w[265] = noise_metadata_schedule_226_0_e2227;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_227_0_e2230: f64 = (w[246] * w[8]);
            let noise_metadata_schedule_227_0_e2232: f64 = (noise_metadata_schedule_227_0_e2230 / w[48]);
            let noise_metadata_schedule_227_0_e2234: f64 = if noise_metadata_schedule_227_0_e2232 < params[147] { 1.0 } else { 0.0 };
            w[506] = noise_metadata_schedule_227_0_e2234;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_228_0_e2243,) = {
    if (w[506] != 0.0) {
        let noise_metadata_schedule_228_0_e2238: f64 = (w[246] * w[8]);
        let noise_metadata_schedule_228_0_e2240: f64 = (noise_metadata_schedule_228_0_e2238 / w[48]);
        let noise_metadata_schedule_228_0_e2241: f64 = (noise_metadata_schedule_228_0_e2240).exp();
        (noise_metadata_schedule_228_0_e2241,)
    } else {
        (w[266],)
    }
};
            w[266] = noise_metadata_schedule_228_0_e2243;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_229_0_e2249,) = {
    if (w[506] == 0.0) {
        let noise_metadata_schedule_229_0_e2247: f64 = (params[147]).exp();
        (noise_metadata_schedule_229_0_e2247,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_229_0_e2249;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_230_0_e2264,) = {
    if (w[506] == 0.0) {
        let noise_metadata_schedule_230_0_e2256: f64 = (w[246] * w[8]);
        let noise_metadata_schedule_230_0_e2258: f64 = (noise_metadata_schedule_230_0_e2256 / w[48]);
        let noise_metadata_schedule_230_0_e2260: f64 = (noise_metadata_schedule_230_0_e2258 - params[147]);
        let noise_metadata_schedule_230_0_e2261: f64 = (1.0 + noise_metadata_schedule_230_0_e2260);
        let noise_metadata_schedule_230_0_e2262: f64 = (w[295] * noise_metadata_schedule_230_0_e2261);
        (noise_metadata_schedule_230_0_e2262,)
    } else {
        (w[266],)
    }
};
            w[266] = noise_metadata_schedule_230_0_e2264;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_231_0_e2267: f64 = (w[249] * w[8]);
            let noise_metadata_schedule_231_0_e2269: f64 = if noise_metadata_schedule_231_0_e2267 < params[147] { 1.0 } else { 0.0 };
            w[507] = noise_metadata_schedule_231_0_e2269;
        }
        if (active[0] & 0x41800) != 0 {
            let (noise_metadata_schedule_232_0_e2276,) = {
    if (w[507] != 0.0) {
        let noise_metadata_schedule_232_0_e2273: f64 = (w[249] * w[8]);
        let noise_metadata_schedule_232_0_e2274: f64 = (noise_metadata_schedule_232_0_e2273).exp();
        (noise_metadata_schedule_232_0_e2274,)
    } else {
        (w[268],)
    }
};
            w[268] = noise_metadata_schedule_232_0_e2276;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_233_0_e2282,) = {
    if (w[507] == 0.0) {
        let noise_metadata_schedule_233_0_e2280: f64 = (params[147]).exp();
        (noise_metadata_schedule_233_0_e2280,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_233_0_e2282;
        }
        if (active[0] & 0x41800) != 0 {
            let (noise_metadata_schedule_234_0_e2295,) = {
    if (w[507] == 0.0) {
        let noise_metadata_schedule_234_0_e2289: f64 = (w[249] * w[8]);
        let noise_metadata_schedule_234_0_e2291: f64 = (noise_metadata_schedule_234_0_e2289 - params[147]);
        let noise_metadata_schedule_234_0_e2292: f64 = (1.0 + noise_metadata_schedule_234_0_e2291);
        let noise_metadata_schedule_234_0_e2293: f64 = (w[295] * noise_metadata_schedule_234_0_e2292);
        (noise_metadata_schedule_234_0_e2293,)
    } else {
        (w[268],)
    }
};
            w[268] = noise_metadata_schedule_234_0_e2295;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_235_0_e2298: f64 = (w[248] * w[8]);
            let noise_metadata_schedule_235_0_e2300: f64 = if noise_metadata_schedule_235_0_e2298 < params[147] { 1.0 } else { 0.0 };
            w[508] = noise_metadata_schedule_235_0_e2300;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_236_0_e2307,) = {
    if (w[508] != 0.0) {
        let noise_metadata_schedule_236_0_e2304: f64 = (w[248] * w[8]);
        let noise_metadata_schedule_236_0_e2305: f64 = (noise_metadata_schedule_236_0_e2304).exp();
        (noise_metadata_schedule_236_0_e2305,)
    } else {
        (w[267],)
    }
};
            w[267] = noise_metadata_schedule_236_0_e2307;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_237_0_e2313,) = {
    if (w[508] == 0.0) {
        let noise_metadata_schedule_237_0_e2311: f64 = (params[147]).exp();
        (noise_metadata_schedule_237_0_e2311,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_237_0_e2313;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_238_0_e2326,) = {
    if (w[508] == 0.0) {
        let noise_metadata_schedule_238_0_e2320: f64 = (w[248] * w[8]);
        let noise_metadata_schedule_238_0_e2322: f64 = (noise_metadata_schedule_238_0_e2320 - params[147]);
        let noise_metadata_schedule_238_0_e2323: f64 = (1.0 + noise_metadata_schedule_238_0_e2322);
        let noise_metadata_schedule_238_0_e2324: f64 = (w[295] * noise_metadata_schedule_238_0_e2323);
        (noise_metadata_schedule_238_0_e2324,)
    } else {
        (w[267],)
    }
};
            w[267] = noise_metadata_schedule_238_0_e2326;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_239_0_e2329: f64 = (w[261] * w[8]);
            let noise_metadata_schedule_239_0_e2331: f64 = if noise_metadata_schedule_239_0_e2329 < params[147] { 1.0 } else { 0.0 };
            w[509] = noise_metadata_schedule_239_0_e2331;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_240_0_e2338,) = {
    if (w[509] != 0.0) {
        let noise_metadata_schedule_240_0_e2335: f64 = (w[261] * w[8]);
        let noise_metadata_schedule_240_0_e2336: f64 = (noise_metadata_schedule_240_0_e2335).exp();
        (noise_metadata_schedule_240_0_e2336,)
    } else {
        (w[269],)
    }
};
            w[269] = noise_metadata_schedule_240_0_e2338;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_241_0_e2344,) = {
    if (w[509] == 0.0) {
        let noise_metadata_schedule_241_0_e2342: f64 = (params[147]).exp();
        (noise_metadata_schedule_241_0_e2342,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_241_0_e2344;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_242_0_e2357,) = {
    if (w[509] == 0.0) {
        let noise_metadata_schedule_242_0_e2351: f64 = (w[261] * w[8]);
        let noise_metadata_schedule_242_0_e2353: f64 = (noise_metadata_schedule_242_0_e2351 - params[147]);
        let noise_metadata_schedule_242_0_e2354: f64 = (1.0 + noise_metadata_schedule_242_0_e2353);
        let noise_metadata_schedule_242_0_e2355: f64 = (w[295] * noise_metadata_schedule_242_0_e2354);
        (noise_metadata_schedule_242_0_e2355,)
    } else {
        (w[269],)
    }
};
            w[269] = noise_metadata_schedule_242_0_e2357;
        }
        if (active[0] & 0xfffe7) != 0 {
            let noise_metadata_schedule_243_0_e2360: f64 = (w[253] * w[8]);
            let noise_metadata_schedule_243_0_e2362: f64 = if noise_metadata_schedule_243_0_e2360 < params[147] { 1.0 } else { 0.0 };
            w[510] = noise_metadata_schedule_243_0_e2362;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_244_0_e2369,) = {
    if (w[510] != 0.0) {
        let noise_metadata_schedule_244_0_e2366: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_244_0_e2367: f64 = (noise_metadata_schedule_244_0_e2366).exp();
        (noise_metadata_schedule_244_0_e2367,)
    } else {
        (w[256],)
    }
};
            w[256] = noise_metadata_schedule_244_0_e2369;
        }
        if (active[0] & 0xfffe7) != 0 {
            let (noise_metadata_schedule_245_0_e2375,) = {
    if (w[510] == 0.0) {
        let noise_metadata_schedule_245_0_e2373: f64 = (params[147]).exp();
        (noise_metadata_schedule_245_0_e2373,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_245_0_e2375;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_246_0_e2388,) = {
    if (w[510] == 0.0) {
        let noise_metadata_schedule_246_0_e2382: f64 = (w[253] * w[8]);
        let noise_metadata_schedule_246_0_e2384: f64 = (noise_metadata_schedule_246_0_e2382 - params[147]);
        let noise_metadata_schedule_246_0_e2385: f64 = (1.0 + noise_metadata_schedule_246_0_e2384);
        let noise_metadata_schedule_246_0_e2386: f64 = (w[295] * noise_metadata_schedule_246_0_e2385);
        (noise_metadata_schedule_246_0_e2386,)
    } else {
        (w[256],)
    }
};
            w[256] = noise_metadata_schedule_246_0_e2388;
        }
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_247_0_e2391: f64 = (w[254] * w[8]);
            let noise_metadata_schedule_247_0_e2393: f64 = if noise_metadata_schedule_247_0_e2391 < params[147] { 1.0 } else { 0.0 };
            w[511] = noise_metadata_schedule_247_0_e2393;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_248_0_e2400,) = {
    if (w[511] != 0.0) {
        let noise_metadata_schedule_248_0_e2397: f64 = (w[254] * w[8]);
        let noise_metadata_schedule_248_0_e2398: f64 = (noise_metadata_schedule_248_0_e2397).exp();
        (noise_metadata_schedule_248_0_e2398,)
    } else {
        (w[257],)
    }
};
            w[257] = noise_metadata_schedule_248_0_e2400;
        }
        if (active[0] & 0xdffe7) != 0 {
            let (noise_metadata_schedule_249_0_e2406,) = {
    if (w[511] == 0.0) {
        let noise_metadata_schedule_249_0_e2404: f64 = (params[147]).exp();
        (noise_metadata_schedule_249_0_e2404,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_249_0_e2406;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_250_0_e2419,) = {
    if (w[511] == 0.0) {
        let noise_metadata_schedule_250_0_e2413: f64 = (w[254] * w[8]);
        let noise_metadata_schedule_250_0_e2415: f64 = (noise_metadata_schedule_250_0_e2413 - params[147]);
        let noise_metadata_schedule_250_0_e2416: f64 = (1.0 + noise_metadata_schedule_250_0_e2415);
        let noise_metadata_schedule_250_0_e2417: f64 = (w[295] * noise_metadata_schedule_250_0_e2416);
        (noise_metadata_schedule_250_0_e2417,)
    } else {
        (w[257],)
    }
};
            w[257] = noise_metadata_schedule_250_0_e2419;
        }
        if (active[0] & 0xdffe7) != 0 {
            let noise_metadata_schedule_251_0_e2422: f64 = (w[255] * w[8]);
            let noise_metadata_schedule_251_0_e2424: f64 = if noise_metadata_schedule_251_0_e2422 < params[147] { 1.0 } else { 0.0 };
            w[512] = noise_metadata_schedule_251_0_e2424;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_252_0_e2431,) = {
    if (w[512] != 0.0) {
        let noise_metadata_schedule_252_0_e2428: f64 = (w[255] * w[8]);
        let noise_metadata_schedule_252_0_e2429: f64 = (noise_metadata_schedule_252_0_e2428).exp();
        (noise_metadata_schedule_252_0_e2429,)
    } else {
        (w[258],)
    }
};
            w[258] = noise_metadata_schedule_252_0_e2431;
        }
        if (active[0] & 0xdffe7) != 0 {
            let (noise_metadata_schedule_253_0_e2437,) = {
    if (w[512] == 0.0) {
        let noise_metadata_schedule_253_0_e2435: f64 = (params[147]).exp();
        (noise_metadata_schedule_253_0_e2435,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_253_0_e2437;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_254_0_e2450,) = {
    if (w[512] == 0.0) {
        let noise_metadata_schedule_254_0_e2444: f64 = (w[255] * w[8]);
        let noise_metadata_schedule_254_0_e2446: f64 = (noise_metadata_schedule_254_0_e2444 - params[147]);
        let noise_metadata_schedule_254_0_e2447: f64 = (1.0 + noise_metadata_schedule_254_0_e2446);
        let noise_metadata_schedule_254_0_e2448: f64 = (w[295] * noise_metadata_schedule_254_0_e2447);
        (noise_metadata_schedule_254_0_e2448,)
    } else {
        (w[258],)
    }
};
            w[258] = noise_metadata_schedule_254_0_e2450;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_255_0_e2453: f64 = (w[261] - w[16]);
            let noise_metadata_schedule_255_0_e2455: f64 = (noise_metadata_schedule_255_0_e2453 * w[8]);
            let noise_metadata_schedule_255_0_e2457: f64 = if noise_metadata_schedule_255_0_e2455 < params[147] { 1.0 } else { 0.0 };
            w[513] = noise_metadata_schedule_255_0_e2457;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_257_0_e2472,) = {
    if (w[513] == 0.0) {
        let noise_metadata_schedule_257_0_e2470: f64 = (params[147]).exp();
        (noise_metadata_schedule_257_0_e2470,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_257_0_e2472;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_259_0_e2490: f64 = (w[249] - w[16]);
            let noise_metadata_schedule_259_0_e2492: f64 = (noise_metadata_schedule_259_0_e2490 * w[8]);
            let noise_metadata_schedule_259_0_e2494: f64 = if noise_metadata_schedule_259_0_e2492 < params[147] { 1.0 } else { 0.0 };
            w[514] = noise_metadata_schedule_259_0_e2494;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_261_0_e2509,) = {
    if (w[514] == 0.0) {
        let noise_metadata_schedule_261_0_e2507: f64 = (params[147]).exp();
        (noise_metadata_schedule_261_0_e2507,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_261_0_e2509;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_263_0_e2527: f64 = (w[245] - w[16]);
            let noise_metadata_schedule_263_0_e2529: f64 = (noise_metadata_schedule_263_0_e2527 * w[8]);
            let noise_metadata_schedule_263_0_e2531: f64 = if noise_metadata_schedule_263_0_e2529 < params[147] { 1.0 } else { 0.0 };
            w[515] = noise_metadata_schedule_263_0_e2531;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_264_0_e2540,) = {
    if (w[515] != 0.0) {
        let noise_metadata_schedule_264_0_e2535: f64 = (w[245] - w[16]);
        let noise_metadata_schedule_264_0_e2537: f64 = (noise_metadata_schedule_264_0_e2535 * w[8]);
        let noise_metadata_schedule_264_0_e2538: f64 = (noise_metadata_schedule_264_0_e2537).exp();
        (noise_metadata_schedule_264_0_e2538,)
    } else {
        (w[271],)
    }
};
            w[271] = noise_metadata_schedule_264_0_e2540;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_265_0_e2546,) = {
    if (w[515] == 0.0) {
        let noise_metadata_schedule_265_0_e2544: f64 = (params[147]).exp();
        (noise_metadata_schedule_265_0_e2544,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_265_0_e2546;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_266_0_e2561,) = {
    if (w[515] == 0.0) {
        let noise_metadata_schedule_266_0_e2553: f64 = (w[245] - w[16]);
        let noise_metadata_schedule_266_0_e2555: f64 = (noise_metadata_schedule_266_0_e2553 * w[8]);
        let noise_metadata_schedule_266_0_e2557: f64 = (noise_metadata_schedule_266_0_e2555 - params[147]);
        let noise_metadata_schedule_266_0_e2558: f64 = (1.0 + noise_metadata_schedule_266_0_e2557);
        let noise_metadata_schedule_266_0_e2559: f64 = (w[295] * noise_metadata_schedule_266_0_e2558);
        (noise_metadata_schedule_266_0_e2559,)
    } else {
        (w[271],)
    }
};
            w[271] = noise_metadata_schedule_266_0_e2561;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_267_0_e2564: f64 = (w[244] - w[16]);
            let noise_metadata_schedule_267_0_e2566: f64 = (noise_metadata_schedule_267_0_e2564 * w[8]);
            let noise_metadata_schedule_267_0_e2568: f64 = if noise_metadata_schedule_267_0_e2566 < params[147] { 1.0 } else { 0.0 };
            w[516] = noise_metadata_schedule_267_0_e2568;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_268_0_e2577,) = {
    if (w[516] != 0.0) {
        let noise_metadata_schedule_268_0_e2572: f64 = (w[244] - w[16]);
        let noise_metadata_schedule_268_0_e2574: f64 = (noise_metadata_schedule_268_0_e2572 * w[8]);
        let noise_metadata_schedule_268_0_e2575: f64 = (noise_metadata_schedule_268_0_e2574).exp();
        (noise_metadata_schedule_268_0_e2575,)
    } else {
        (w[273],)
    }
};
            w[273] = noise_metadata_schedule_268_0_e2577;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_269_0_e2583,) = {
    if (w[516] == 0.0) {
        let noise_metadata_schedule_269_0_e2581: f64 = (params[147]).exp();
        (noise_metadata_schedule_269_0_e2581,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_269_0_e2583;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_270_0_e2598,) = {
    if (w[516] == 0.0) {
        let noise_metadata_schedule_270_0_e2590: f64 = (w[244] - w[16]);
        let noise_metadata_schedule_270_0_e2592: f64 = (noise_metadata_schedule_270_0_e2590 * w[8]);
        let noise_metadata_schedule_270_0_e2594: f64 = (noise_metadata_schedule_270_0_e2592 - params[147]);
        let noise_metadata_schedule_270_0_e2595: f64 = (1.0 + noise_metadata_schedule_270_0_e2594);
        let noise_metadata_schedule_270_0_e2596: f64 = (w[295] * noise_metadata_schedule_270_0_e2595);
        (noise_metadata_schedule_270_0_e2596,)
    } else {
        (w[273],)
    }
};
            w[273] = noise_metadata_schedule_270_0_e2598;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_271_0_e2602: f64 = (4.0 * w[271]);
            let noise_metadata_schedule_271_0_e2603: f64 = (1.0 + noise_metadata_schedule_271_0_e2602);
            let noise_metadata_schedule_271_0_e2604: f64 = (noise_metadata_schedule_271_0_e2603).sqrt();
            w[111] = noise_metadata_schedule_271_0_e2604;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_272_0_e2608: f64 = (4.0 * w[273]);
            let noise_metadata_schedule_272_0_e2609: f64 = (1.0 + noise_metadata_schedule_272_0_e2608);
            let noise_metadata_schedule_272_0_e2610: f64 = (noise_metadata_schedule_272_0_e2609).sqrt();
            w[112] = noise_metadata_schedule_272_0_e2610;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_273_0_e2613: f64 = (2.0 * w[273]);
            let noise_metadata_schedule_273_0_e2616: f64 = (1.0 + w[112]);
            let noise_metadata_schedule_273_0_e2617: f64 = (noise_metadata_schedule_273_0_e2613 / noise_metadata_schedule_273_0_e2616);
            w[113] = noise_metadata_schedule_273_0_e2617;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_274_0_e2620: f64 = if w[113] < params[149] { 1.0 } else { 0.0 };
            w[517] = noise_metadata_schedule_274_0_e2620;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_275_0_e2624,) = {
    if (w[517] != 0.0) {
        (params[149],)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_275_0_e2624;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_276_0_e2628: f64 = (w[111] - w[112]);
            let noise_metadata_schedule_276_0_e2631: f64 = (w[111] + 1.0);
            let noise_metadata_schedule_276_0_e2634: f64 = (w[112] + 1.0);
            let noise_metadata_schedule_276_0_e2635: f64 = (noise_metadata_schedule_276_0_e2631 / noise_metadata_schedule_276_0_e2634);
            let noise_metadata_schedule_276_0_e2636: f64 = (noise_metadata_schedule_276_0_e2635).ln();
            let noise_metadata_schedule_276_0_e2637: f64 = (noise_metadata_schedule_276_0_e2628 - noise_metadata_schedule_276_0_e2636);
            let noise_metadata_schedule_276_0_e2638: f64 = (w[6] * noise_metadata_schedule_276_0_e2637);
            w[114] = noise_metadata_schedule_276_0_e2638;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_277_0_e2641: f64 = (w[114] + w[250]);
            let noise_metadata_schedule_277_0_e2643: f64 = (noise_metadata_schedule_277_0_e2641 / w[31]);
            w[115] = noise_metadata_schedule_277_0_e2643;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_278_0_e2646: f64 = if w[115] > 0.0 { 1.0 } else { 0.0 };
            w[518] = noise_metadata_schedule_278_0_e2646;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_279_0_e2649: f64 = if w[244] < 100.0 { 1.0 } else { 0.0 };
            w[519] = noise_metadata_schedule_279_0_e2649;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_280_0_e2655,) = {
    if ((w[518] != 0.0) && (w[519] != 0.0)) {
        (w[244],)
    } else {
        (w[297],)
    }
};
            w[297] = noise_metadata_schedule_280_0_e2655;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_281_0_e2669,) = {
    if ((w[518] != 0.0) && (w[519] == 0.0)) {
        let noise_metadata_schedule_281_0_e2664: f64 = (w[244] - 100.0);
        let noise_metadata_schedule_281_0_e2665: f64 = (1.0 + noise_metadata_schedule_281_0_e2664);
        let noise_metadata_schedule_281_0_e2666: f64 = (noise_metadata_schedule_281_0_e2665).ln();
        let noise_metadata_schedule_281_0_e2667: f64 = (100.0 + noise_metadata_schedule_281_0_e2666);
        (noise_metadata_schedule_281_0_e2667,)
    } else {
        (w[297],)
    }
};
            w[297] = noise_metadata_schedule_281_0_e2669;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_282_0_e2690,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_282_0_e2674: f64 = (2.0 * w[6]);
        let noise_metadata_schedule_282_0_e2677: f64 = (0.5 * w[115]);
        let noise_metadata_schedule_282_0_e2679: f64 = (noise_metadata_schedule_282_0_e2677 * w[31]);
        let noise_metadata_schedule_282_0_e2681: f64 = (noise_metadata_schedule_282_0_e2679 * w[8]);
        let noise_metadata_schedule_282_0_e2683: f64 = (noise_metadata_schedule_282_0_e2681 + 1.0);
        let noise_metadata_schedule_282_0_e2684: f64 = (noise_metadata_schedule_282_0_e2683).ln();
        let noise_metadata_schedule_282_0_e2685: f64 = (noise_metadata_schedule_282_0_e2674 * noise_metadata_schedule_282_0_e2684);
        let noise_metadata_schedule_282_0_e2686: f64 = (w[16] + noise_metadata_schedule_282_0_e2685);
        let noise_metadata_schedule_282_0_e2688: f64 = (noise_metadata_schedule_282_0_e2686 - w[297]);
        (noise_metadata_schedule_282_0_e2688,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_282_0_e2690;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_283_0_e2696,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_283_0_e2694: f64 = (0.2 * w[16]);
        (noise_metadata_schedule_283_0_e2694,)
    } else {
        (w[292],)
    }
};
            w[292] = noise_metadata_schedule_283_0_e2696;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_284_0_e2702,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_284_0_e2700: f64 = (w[292] * w[292]);
        (noise_metadata_schedule_284_0_e2700,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_284_0_e2702;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_285_0_e2708,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_285_0_e2706: f64 = (w[116] * w[116]);
        (noise_metadata_schedule_285_0_e2706,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_285_0_e2708;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_286_0_e2711: f64 = if w[116] < 0.0 { 1.0 } else { 0.0 };
            w[520] = noise_metadata_schedule_286_0_e2711;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_287_0_e2726,) = {
    if ((w[518] != 0.0) && (w[520] != 0.0)) {
        let noise_metadata_schedule_287_0_e2717: f64 = (0.5 * w[281]);
        let noise_metadata_schedule_287_0_e2720: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_287_0_e2721: f64 = (noise_metadata_schedule_287_0_e2720).sqrt();
        let noise_metadata_schedule_287_0_e2723: f64 = (noise_metadata_schedule_287_0_e2721 - w[116]);
        let noise_metadata_schedule_287_0_e2724: f64 = (noise_metadata_schedule_287_0_e2717 / noise_metadata_schedule_287_0_e2723);
        (noise_metadata_schedule_287_0_e2724,)
    } else {
        (w[117],)
    }
};
            w[117] = noise_metadata_schedule_287_0_e2726;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_288_0_e2740,) = {
    if ((w[518] != 0.0) && (w[520] == 0.0)) {
        let noise_metadata_schedule_288_0_e2734: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_288_0_e2735: f64 = (noise_metadata_schedule_288_0_e2734).sqrt();
        let noise_metadata_schedule_288_0_e2737: f64 = (noise_metadata_schedule_288_0_e2735 + w[116]);
        let noise_metadata_schedule_288_0_e2738: f64 = (0.5 * noise_metadata_schedule_288_0_e2737);
        (noise_metadata_schedule_288_0_e2738,)
    } else {
        (w[117],)
    }
};
            w[117] = noise_metadata_schedule_288_0_e2740;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_289_0_e2758,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_289_0_e2746: f64 = (params[62] * params[61]);
        let noise_metadata_schedule_289_0_e2747: f64 = (w[117] + noise_metadata_schedule_289_0_e2746);
        let noise_metadata_schedule_289_0_e2748: f64 = (w[117] * noise_metadata_schedule_289_0_e2747);
        let noise_metadata_schedule_289_0_e2753: f64 = (params[62] * w[31]);
        let noise_metadata_schedule_289_0_e2754: f64 = (w[117] + noise_metadata_schedule_289_0_e2753);
        let noise_metadata_schedule_289_0_e2755: f64 = (params[61] * noise_metadata_schedule_289_0_e2754);
        let noise_metadata_schedule_289_0_e2756: f64 = (noise_metadata_schedule_289_0_e2748 / noise_metadata_schedule_289_0_e2755);
        (noise_metadata_schedule_289_0_e2756,)
    } else {
        (w[118],)
    }
};
            w[118] = noise_metadata_schedule_289_0_e2758;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_290_0_e2764,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_290_0_e2762: f64 = (w[115] / w[118]);
        (noise_metadata_schedule_290_0_e2762,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_290_0_e2764;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_291_0_e2772,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_291_0_e2768: f64 = (w[285] - 1.0);
        let noise_metadata_schedule_291_0_e2770: f64 = (noise_metadata_schedule_291_0_e2768 / params[63]);
        (noise_metadata_schedule_291_0_e2770,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_291_0_e2772;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_292_0_e2775: f64 = if w[285] < 1.0 { 1.0 } else { 0.0 };
            w[521] = noise_metadata_schedule_292_0_e2775;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_293_0_e2789,) = {
    if ((w[518] != 0.0) && (w[521] != 0.0)) {
        let noise_metadata_schedule_293_0_e2783: f64 = (w[279]).exp();
        let noise_metadata_schedule_293_0_e2784: f64 = (1.0 + noise_metadata_schedule_293_0_e2783);
        let noise_metadata_schedule_293_0_e2785: f64 = (noise_metadata_schedule_293_0_e2784).ln();
        let noise_metadata_schedule_293_0_e2786: f64 = (params[63] * noise_metadata_schedule_293_0_e2785);
        let noise_metadata_schedule_293_0_e2787: f64 = (1.0 + noise_metadata_schedule_293_0_e2786);
        (noise_metadata_schedule_293_0_e2787,)
    } else {
        (w[283],)
    }
};
            w[283] = noise_metadata_schedule_293_0_e2789;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_294_0_e2805,) = {
    if ((w[518] != 0.0) && (w[521] == 0.0)) {
        let noise_metadata_schedule_294_0_e2798: f64 = (-w[279]);
        let noise_metadata_schedule_294_0_e2799: f64 = (noise_metadata_schedule_294_0_e2798).exp();
        let noise_metadata_schedule_294_0_e2800: f64 = (1.0 + noise_metadata_schedule_294_0_e2799);
        let noise_metadata_schedule_294_0_e2801: f64 = (noise_metadata_schedule_294_0_e2800).ln();
        let noise_metadata_schedule_294_0_e2802: f64 = (params[63] * noise_metadata_schedule_294_0_e2801);
        let noise_metadata_schedule_294_0_e2803: f64 = (w[285] + noise_metadata_schedule_294_0_e2802);
        (noise_metadata_schedule_294_0_e2803,)
    } else {
        (w[283],)
    }
};
            w[283] = noise_metadata_schedule_294_0_e2805;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_295_0_e2822,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_295_0_e2812: f64 = (-1.0);
        let noise_metadata_schedule_295_0_e2814: f64 = (noise_metadata_schedule_295_0_e2812 / params[63]);
        let noise_metadata_schedule_295_0_e2815: f64 = (noise_metadata_schedule_295_0_e2814).exp();
        let noise_metadata_schedule_295_0_e2816: f64 = (1.0 + noise_metadata_schedule_295_0_e2815);
        let noise_metadata_schedule_295_0_e2817: f64 = (noise_metadata_schedule_295_0_e2816).ln();
        let noise_metadata_schedule_295_0_e2818: f64 = (params[63] * noise_metadata_schedule_295_0_e2817);
        let noise_metadata_schedule_295_0_e2819: f64 = (1.0 + noise_metadata_schedule_295_0_e2818);
        let noise_metadata_schedule_295_0_e2820: f64 = (w[283] / noise_metadata_schedule_295_0_e2819);
        (noise_metadata_schedule_295_0_e2820,)
    } else {
        (w[119],)
    }
};
            w[119] = noise_metadata_schedule_295_0_e2822;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_296_0_e2830,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_296_0_e2827: f64 = (params[62] * params[61]);
        let noise_metadata_schedule_296_0_e2828: f64 = (w[117] / noise_metadata_schedule_296_0_e2827);
        (noise_metadata_schedule_296_0_e2828,)
    } else {
        (w[120],)
    }
};
            w[120] = noise_metadata_schedule_296_0_e2830;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_297_0_e2855,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_297_0_e2836: f64 = (4.0 * w[119]);
        let noise_metadata_schedule_297_0_e2838: f64 = (noise_metadata_schedule_297_0_e2836 * w[120]);
        let noise_metadata_schedule_297_0_e2841: f64 = (1.0 + w[120]);
        let noise_metadata_schedule_297_0_e2842: f64 = (noise_metadata_schedule_297_0_e2838 * noise_metadata_schedule_297_0_e2841);
        let noise_metadata_schedule_297_0_e2843: f64 = (1.0 + noise_metadata_schedule_297_0_e2842);
        let noise_metadata_schedule_297_0_e2844: f64 = (noise_metadata_schedule_297_0_e2843).sqrt();
        let noise_metadata_schedule_297_0_e2845: f64 = (1.0 + noise_metadata_schedule_297_0_e2844);
        let noise_metadata_schedule_297_0_e2848: f64 = (2.0 * w[119]);
        let noise_metadata_schedule_297_0_e2851: f64 = (1.0 + w[120]);
        let noise_metadata_schedule_297_0_e2852: f64 = (noise_metadata_schedule_297_0_e2848 * noise_metadata_schedule_297_0_e2851);
        let noise_metadata_schedule_297_0_e2853: f64 = (noise_metadata_schedule_297_0_e2845 / noise_metadata_schedule_297_0_e2852);
        (noise_metadata_schedule_297_0_e2853,)
    } else {
        (w[121],)
    }
};
            w[121] = noise_metadata_schedule_297_0_e2855;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_298_0_e2871,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_298_0_e2859: f64 = (1.0 - w[121]);
        let noise_metadata_schedule_298_0_e2862: f64 = (w[113] * w[121]);
        let noise_metadata_schedule_298_0_e2863: f64 = (noise_metadata_schedule_298_0_e2859 + noise_metadata_schedule_298_0_e2862);
        let noise_metadata_schedule_298_0_e2867: f64 = (w[113] * w[121]);
        let noise_metadata_schedule_298_0_e2868: f64 = (1.0 + noise_metadata_schedule_298_0_e2867);
        let noise_metadata_schedule_298_0_e2869: f64 = (noise_metadata_schedule_298_0_e2863 / noise_metadata_schedule_298_0_e2868);
        (noise_metadata_schedule_298_0_e2869,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_298_0_e2871;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_299_0_e2883,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_299_0_e2875: f64 = (0.5 * w[115]);
        let noise_metadata_schedule_299_0_e2877: f64 = (noise_metadata_schedule_299_0_e2875 * w[31]);
        let noise_metadata_schedule_299_0_e2879: f64 = (noise_metadata_schedule_299_0_e2877 * w[122]);
        let noise_metadata_schedule_299_0_e2881: f64 = (noise_metadata_schedule_299_0_e2879 * w[8]);
        (noise_metadata_schedule_299_0_e2881,)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_299_0_e2883;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_300_0_e2897,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_300_0_e2887: f64 = (2.0 * w[124]);
        let noise_metadata_schedule_300_0_e2891: f64 = (w[113] + w[124]);
        let noise_metadata_schedule_300_0_e2893: f64 = (noise_metadata_schedule_300_0_e2891 + 1.0);
        let noise_metadata_schedule_300_0_e2894: f64 = (w[113] * noise_metadata_schedule_300_0_e2893);
        let noise_metadata_schedule_300_0_e2895: f64 = (noise_metadata_schedule_300_0_e2887 + noise_metadata_schedule_300_0_e2894);
        (noise_metadata_schedule_300_0_e2895,)
    } else {
        (w[286],)
    }
};
            w[286] = noise_metadata_schedule_300_0_e2897;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_301_0_e2905,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_301_0_e2902: f64 = (w[124] - 1.0);
        let noise_metadata_schedule_301_0_e2903: f64 = (0.5 * noise_metadata_schedule_301_0_e2902);
        (noise_metadata_schedule_301_0_e2903,)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_301_0_e2905;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_302_0_e2913,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_302_0_e2909: f64 = (w[125] * w[125]);
        let noise_metadata_schedule_302_0_e2911: f64 = (noise_metadata_schedule_302_0_e2909 + w[286]);
        (noise_metadata_schedule_302_0_e2911,)
    } else {
        (w[280],)
    }
};
            w[280] = noise_metadata_schedule_302_0_e2913;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_303_0_e2916: f64 = if w[124] >= 1.0 { 1.0 } else { 0.0 };
            w[522] = noise_metadata_schedule_303_0_e2916;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_304_0_e2925,) = {
    if ((w[518] != 0.0) && (w[522] != 0.0)) {
        let noise_metadata_schedule_304_0_e2922: f64 = (w[280]).sqrt();
        let noise_metadata_schedule_304_0_e2923: f64 = (w[125] + noise_metadata_schedule_304_0_e2922);
        (noise_metadata_schedule_304_0_e2923,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_304_0_e2925;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_305_0_e2937,) = {
    if ((w[518] != 0.0) && (w[522] == 0.0)) {
        let noise_metadata_schedule_305_0_e2932: f64 = (w[280]).sqrt();
        let noise_metadata_schedule_305_0_e2934: f64 = (noise_metadata_schedule_305_0_e2932 - w[125]);
        let noise_metadata_schedule_305_0_e2935: f64 = (w[286] / noise_metadata_schedule_305_0_e2934);
        (noise_metadata_schedule_305_0_e2935,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_305_0_e2937;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_306_0_e2940: f64 = if w[126] < params[148] { 1.0 } else { 0.0 };
            w[523] = noise_metadata_schedule_306_0_e2940;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_307_0_e2946,) = {
    if ((w[518] != 0.0) && (w[523] != 0.0)) {
        (params[148],)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_307_0_e2946;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_308_0_e2959,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_308_0_e2951: f64 = (w[126] + 1.0);
        let noise_metadata_schedule_308_0_e2952: f64 = (w[126] * noise_metadata_schedule_308_0_e2951);
        let noise_metadata_schedule_308_0_e2955: f64 = (w[16] * w[8]);
        let noise_metadata_schedule_308_0_e2956: f64 = (noise_metadata_schedule_308_0_e2955).exp();
        let noise_metadata_schedule_308_0_e2957: f64 = (noise_metadata_schedule_308_0_e2952 * noise_metadata_schedule_308_0_e2956);
        (noise_metadata_schedule_308_0_e2957,)
    } else {
        (w[128],)
    }
};
            w[128] = noise_metadata_schedule_308_0_e2959;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_309_0_e2969,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_309_0_e2963: f64 = (0.5 * params[61]);
        let noise_metadata_schedule_309_0_e2966: f64 = (w[115] - params[62]);
        let noise_metadata_schedule_309_0_e2967: f64 = (noise_metadata_schedule_309_0_e2963 * noise_metadata_schedule_309_0_e2966);
        (noise_metadata_schedule_309_0_e2967,)
    } else {
        (w[130],)
    }
};
            w[130] = noise_metadata_schedule_309_0_e2969;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_310_0_e2979,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_310_0_e2973: f64 = (params[61] * w[31]);
        let noise_metadata_schedule_310_0_e2975: f64 = (noise_metadata_schedule_310_0_e2973 * params[62]);
        let noise_metadata_schedule_310_0_e2977: f64 = (noise_metadata_schedule_310_0_e2975 * w[115]);
        (noise_metadata_schedule_310_0_e2977,)
    } else {
        (w[131],)
    }
};
            w[131] = noise_metadata_schedule_310_0_e2979;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_311_0_e2990,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_311_0_e2984: f64 = (w[130] * w[130]);
        let noise_metadata_schedule_311_0_e2986: f64 = (noise_metadata_schedule_311_0_e2984 + w[131]);
        let noise_metadata_schedule_311_0_e2987: f64 = (noise_metadata_schedule_311_0_e2986).sqrt();
        let noise_metadata_schedule_311_0_e2988: f64 = (w[130] + noise_metadata_schedule_311_0_e2987);
        (noise_metadata_schedule_311_0_e2988,)
    } else {
        (w[132],)
    }
};
            w[132] = noise_metadata_schedule_311_0_e2990;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_312_0_e2993: f64 = if params[73] == 0.0 { 1.0 } else { 0.0 };
            w[524] = noise_metadata_schedule_312_0_e2993;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_313_0_e3001,) = {
    if ((w[518] != 0.0) && (w[524] != 0.0)) {
        let noise_metadata_schedule_313_0_e2999: f64 = (w[17] * 0.1);
        (noise_metadata_schedule_313_0_e2999,)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_313_0_e3001;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_314_0_e3018,) = {
    if ((w[518] != 0.0) && (w[524] == 0.0)) {
        let noise_metadata_schedule_314_0_e3010: f64 = (2.0 * w[115]);
        let noise_metadata_schedule_314_0_e3013: f64 = (w[115] + w[118]);
        let noise_metadata_schedule_314_0_e3014: f64 = (noise_metadata_schedule_314_0_e3010 / noise_metadata_schedule_314_0_e3013);
        let noise_metadata_schedule_314_0_e3015: f64 = (0.1 + noise_metadata_schedule_314_0_e3014);
        let noise_metadata_schedule_314_0_e3016: f64 = (w[17] * noise_metadata_schedule_314_0_e3015);
        (noise_metadata_schedule_314_0_e3016,)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_314_0_e3018;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_315_0_e3028,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_315_0_e3022: f64 = (params[62] * w[115]);
        let noise_metadata_schedule_315_0_e3025: f64 = (params[62] + w[115]);
        let noise_metadata_schedule_315_0_e3026: f64 = (noise_metadata_schedule_315_0_e3022 / noise_metadata_schedule_315_0_e3025);
        (noise_metadata_schedule_315_0_e3026,)
    } else {
        (w[134],)
    }
};
            w[134] = noise_metadata_schedule_315_0_e3028;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_316_0_e3036,) = {
    if (w[518] != 0.0) {
        let noise_metadata_schedule_316_0_e3033: f64 = (params[62] + w[115]);
        let noise_metadata_schedule_316_0_e3034: f64 = (params[62] / noise_metadata_schedule_316_0_e3033);
        (noise_metadata_schedule_316_0_e3034,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_316_0_e3036;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_318_0_e3052,) = {
    if (w[518] == 0.0) {
        let noise_metadata_schedule_318_0_e3046: f64 = (2.0 * w[271]);
        let noise_metadata_schedule_318_0_e3049: f64 = (1.0 + w[111]);
        let noise_metadata_schedule_318_0_e3050: f64 = (noise_metadata_schedule_318_0_e3046 / noise_metadata_schedule_318_0_e3049);
        (noise_metadata_schedule_318_0_e3050,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_318_0_e3052;
        }
        if (active[0] & 0x187e7) != 0 {
            let (noise_metadata_schedule_319_0_e3057,) = {
    if (w[518] == 0.0) {
        (w[265],)
    } else {
        (w[128],)
    }
};
            w[128] = noise_metadata_schedule_319_0_e3057;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_320_0_e3059: f64 = (w[250]).abs();
            let noise_metadata_schedule_320_0_e3062: f64 = (1e-5 * w[6]);
            let noise_metadata_schedule_320_0_e3065: f64 = (w[114]).abs();
            let noise_metadata_schedule_320_0_e3068: f64 = (1e-40 * w[6]);
            let noise_metadata_schedule_320_0_e3071: f64 = (w[111] + w[112]);
            let noise_metadata_schedule_320_0_e3072: f64 = (noise_metadata_schedule_320_0_e3068 * noise_metadata_schedule_320_0_e3071);
            let noise_metadata_schedule_320_0_e3074: f64 = if ((noise_metadata_schedule_320_0_e3059 < noise_metadata_schedule_320_0_e3062) || (noise_metadata_schedule_320_0_e3065 < noise_metadata_schedule_320_0_e3072)) { 1.0 } else { 0.0 };
            w[525] = noise_metadata_schedule_320_0_e3074;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_321_0_e3085,) = {
    if ((w[518] == 0.0) && (w[525] != 0.0)) {
        let noise_metadata_schedule_321_0_e3082: f64 = (w[126] + w[113]);
        let noise_metadata_schedule_321_0_e3083: f64 = (0.5 * noise_metadata_schedule_321_0_e3082);
        (noise_metadata_schedule_321_0_e3083,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_321_0_e3085;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_322_0_e3096,) = {
    if ((w[518] == 0.0) && (w[525] != 0.0)) {
        let noise_metadata_schedule_322_0_e3093: f64 = (w[135] + 1.0);
        let noise_metadata_schedule_322_0_e3094: f64 = (w[135] / noise_metadata_schedule_322_0_e3093);
        (noise_metadata_schedule_322_0_e3094,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_322_0_e3096;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_323_0_e3110,) = {
    if ((w[518] == 0.0) && (w[525] == 0.0)) {
        let noise_metadata_schedule_323_0_e3105: f64 = (w[114] + w[245]);
        let noise_metadata_schedule_323_0_e3107: f64 = (noise_metadata_schedule_323_0_e3105 - w[244]);
        let noise_metadata_schedule_323_0_e3108: f64 = (w[114] / noise_metadata_schedule_323_0_e3107);
        (noise_metadata_schedule_323_0_e3108,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_323_0_e3110;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_324_0_e3115,) = {
    if (w[518] == 0.0) {
        (w[250],)
    } else {
        (w[132],)
    }
};
            w[132] = noise_metadata_schedule_324_0_e3115;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_325_0_e3122,) = {
    if (w[518] == 0.0) {
        let noise_metadata_schedule_325_0_e3120: f64 = (0.1 * w[17]);
        (noise_metadata_schedule_325_0_e3120,)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_325_0_e3122;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_326_0_e3127,) = {
    if (w[518] == 0.0) {
        (w[115],)
    } else {
        (w[134],)
    }
};
            w[134] = noise_metadata_schedule_326_0_e3127;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_327_0_e3136,) = {
    if (w[518] == 0.0) {
        let noise_metadata_schedule_327_0_e3133: f64 = (w[134] / params[62]);
        let noise_metadata_schedule_327_0_e3134: f64 = (1.0 - noise_metadata_schedule_327_0_e3133);
        (noise_metadata_schedule_327_0_e3134,)
    } else {
        (w[210],)
    }
};
            w[210] = noise_metadata_schedule_327_0_e3136;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_328_0_e3141: f64 = (-1.0);
            let noise_metadata_schedule_328_0_e3143: f64 = (noise_metadata_schedule_328_0_e3141 / params[67]);
            let noise_metadata_schedule_328_0_e3144: f64 = (3.0_f64).powf(noise_metadata_schedule_328_0_e3143);
            let noise_metadata_schedule_328_0_e3145: f64 = (1.0 - noise_metadata_schedule_328_0_e3144);
            let noise_metadata_schedule_328_0_e3146: f64 = (w[14] * noise_metadata_schedule_328_0_e3145);
            w[136] = noise_metadata_schedule_328_0_e3146;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_329_0_e3149: f64 = (0.1 * w[14]);
            w[293] = noise_metadata_schedule_329_0_e3149;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_330_0_e3152: f64 = (w[246] - w[136]);
            let noise_metadata_schedule_330_0_e3154: f64 = (noise_metadata_schedule_330_0_e3152 / w[293]);
            w[279] = noise_metadata_schedule_330_0_e3154;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_331_0_e3157: f64 = if w[246] < w[136] { 1.0 } else { 0.0 };
            w[526] = noise_metadata_schedule_331_0_e3157;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_332_0_e3169,) = {
    if (w[526] != 0.0) {
        let noise_metadata_schedule_332_0_e3163: f64 = (w[279]).exp();
        let noise_metadata_schedule_332_0_e3164: f64 = (1.0 + noise_metadata_schedule_332_0_e3163);
        let noise_metadata_schedule_332_0_e3165: f64 = (noise_metadata_schedule_332_0_e3164).ln();
        let noise_metadata_schedule_332_0_e3166: f64 = (w[293] * noise_metadata_schedule_332_0_e3165);
        let noise_metadata_schedule_332_0_e3167: f64 = (w[246] - noise_metadata_schedule_332_0_e3166);
        (noise_metadata_schedule_332_0_e3167,)
    } else {
        (w[137],)
    }
};
            w[137] = noise_metadata_schedule_332_0_e3169;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_333_0_e3183,) = {
    if (w[526] == 0.0) {
        let noise_metadata_schedule_333_0_e3176: f64 = (-w[279]);
        let noise_metadata_schedule_333_0_e3177: f64 = (noise_metadata_schedule_333_0_e3176).exp();
        let noise_metadata_schedule_333_0_e3178: f64 = (1.0 + noise_metadata_schedule_333_0_e3177);
        let noise_metadata_schedule_333_0_e3179: f64 = (noise_metadata_schedule_333_0_e3178).ln();
        let noise_metadata_schedule_333_0_e3180: f64 = (w[293] * noise_metadata_schedule_333_0_e3179);
        let noise_metadata_schedule_333_0_e3181: f64 = (w[136] - noise_metadata_schedule_333_0_e3180);
        (noise_metadata_schedule_333_0_e3181,)
    } else {
        (w[137],)
    }
};
            w[137] = noise_metadata_schedule_333_0_e3183;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_334_0_e3187: f64 = (w[137] * w[65]);
            let noise_metadata_schedule_334_0_e3188: f64 = (1.0 - noise_metadata_schedule_334_0_e3187);
            let noise_metadata_schedule_334_0_e3191: f64 = (1.0 - params[67]);
            let noise_metadata_schedule_334_0_e3192: f64 = (noise_metadata_schedule_334_0_e3188).powf(noise_metadata_schedule_334_0_e3191);
            w[59] = noise_metadata_schedule_334_0_e3192;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_335_0_e3196: f64 = (1.0 - params[67]);
            let noise_metadata_schedule_335_0_e3197: f64 = (w[14] / noise_metadata_schedule_335_0_e3196);
            let noise_metadata_schedule_335_0_e3200: f64 = (1.0 - w[59]);
            let noise_metadata_schedule_335_0_e3201: f64 = (noise_metadata_schedule_335_0_e3197 * noise_metadata_schedule_335_0_e3200);
            let noise_metadata_schedule_335_0_e3205: f64 = (w[246] - w[137]);
            let noise_metadata_schedule_335_0_e3206: f64 = (3.0 * noise_metadata_schedule_335_0_e3205);
            let noise_metadata_schedule_335_0_e3207: f64 = (noise_metadata_schedule_335_0_e3201 + noise_metadata_schedule_335_0_e3206);
            w[138] = noise_metadata_schedule_335_0_e3207;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_336_0_e3210: f64 = if params[74] == 1.0 { 1.0 } else { 0.0 };
            w[527] = noise_metadata_schedule_336_0_e3210;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_337_0_e3214,) = {
    if (w[527] != 0.0) {
        (w[244],)
    } else {
        (w[139],)
    }
};
            w[139] = noise_metadata_schedule_337_0_e3214;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_338_0_e3217: f64 = if params[74] == 2.0 { 1.0 } else { 0.0 };
            w[528] = noise_metadata_schedule_338_0_e3217;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_339_0_e3226,) = {
    if ((w[527] == 0.0) && (w[528] != 0.0)) {
        let noise_metadata_schedule_339_0_e3224: f64 = (w[244] + w[132]);
        (noise_metadata_schedule_339_0_e3224,)
    } else {
        (w[139],)
    }
};
            w[139] = noise_metadata_schedule_339_0_e3226;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_340_0_e3234,) = {
    if ((w[527] == 0.0) && (w[528] == 0.0)) {
        (w[245],)
    } else {
        (w[139],)
    }
};
            w[139] = noise_metadata_schedule_340_0_e3234;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_341_0_e3237: f64 = (2.0 - w[25]);
            let noise_metadata_schedule_341_0_e3240: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_341_0_e3241: f64 = (noise_metadata_schedule_341_0_e3237 / noise_metadata_schedule_341_0_e3240);
            w[140] = noise_metadata_schedule_341_0_e3241;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_342_0_e3246: f64 = (-1.0);
            let noise_metadata_schedule_342_0_e3248: f64 = (noise_metadata_schedule_342_0_e3246 / params[72]);
            let noise_metadata_schedule_342_0_e3249: f64 = (w[140]).powf(noise_metadata_schedule_342_0_e3248);
            let noise_metadata_schedule_342_0_e3250: f64 = (1.0 - noise_metadata_schedule_342_0_e3249);
            let noise_metadata_schedule_342_0_e3251: f64 = (w[17] * noise_metadata_schedule_342_0_e3250);
            w[141] = noise_metadata_schedule_342_0_e3251;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_343_0_e3254: f64 = (w[139] - w[141]);
            let noise_metadata_schedule_343_0_e3256: f64 = (noise_metadata_schedule_343_0_e3254 / w[133]);
            w[279] = noise_metadata_schedule_343_0_e3256;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_344_0_e3259: f64 = if w[139] < w[141] { 1.0 } else { 0.0 };
            w[529] = noise_metadata_schedule_344_0_e3259;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_345_0_e3271,) = {
    if (w[529] != 0.0) {
        let noise_metadata_schedule_345_0_e3265: f64 = (w[279]).exp();
        let noise_metadata_schedule_345_0_e3266: f64 = (1.0 + noise_metadata_schedule_345_0_e3265);
        let noise_metadata_schedule_345_0_e3267: f64 = (noise_metadata_schedule_345_0_e3266).ln();
        let noise_metadata_schedule_345_0_e3268: f64 = (w[133] * noise_metadata_schedule_345_0_e3267);
        let noise_metadata_schedule_345_0_e3269: f64 = (w[139] - noise_metadata_schedule_345_0_e3268);
        (noise_metadata_schedule_345_0_e3269,)
    } else {
        (w[142],)
    }
};
            w[142] = noise_metadata_schedule_345_0_e3271;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let (noise_metadata_schedule_346_0_e3285,) = {
    if (w[529] == 0.0) {
        let noise_metadata_schedule_346_0_e3278: f64 = (-w[279]);
        let noise_metadata_schedule_346_0_e3279: f64 = (noise_metadata_schedule_346_0_e3278).exp();
        let noise_metadata_schedule_346_0_e3280: f64 = (1.0 + noise_metadata_schedule_346_0_e3279);
        let noise_metadata_schedule_346_0_e3281: f64 = (noise_metadata_schedule_346_0_e3280).ln();
        let noise_metadata_schedule_346_0_e3282: f64 = (w[133] * noise_metadata_schedule_346_0_e3281);
        let noise_metadata_schedule_346_0_e3283: f64 = (w[141] - noise_metadata_schedule_346_0_e3282);
        (noise_metadata_schedule_346_0_e3283,)
    } else {
        (w[142],)
    }
};
            w[142] = noise_metadata_schedule_346_0_e3285;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_347_0_e3288: f64 = (w[210]).powf(params[76]);
            w[143] = noise_metadata_schedule_347_0_e3288;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_348_0_e3292: f64 = (1.0 - params[72]);
            let noise_metadata_schedule_348_0_e3293: f64 = (w[17] / noise_metadata_schedule_348_0_e3292);
            let noise_metadata_schedule_348_0_e3299: f64 = (w[142] / w[17]);
            let noise_metadata_schedule_348_0_e3300: f64 = (1.0 - noise_metadata_schedule_348_0_e3299);
            let noise_metadata_schedule_348_0_e3303: f64 = (1.0 - params[72]);
            let noise_metadata_schedule_348_0_e3304: f64 = (noise_metadata_schedule_348_0_e3300).powf(noise_metadata_schedule_348_0_e3303);
            let noise_metadata_schedule_348_0_e3305: f64 = (w[143] * noise_metadata_schedule_348_0_e3304);
            let noise_metadata_schedule_348_0_e3306: f64 = (1.0 - noise_metadata_schedule_348_0_e3305);
            let noise_metadata_schedule_348_0_e3307: f64 = (noise_metadata_schedule_348_0_e3293 * noise_metadata_schedule_348_0_e3306);
            let noise_metadata_schedule_348_0_e3310: f64 = (w[143] * w[140]);
            let noise_metadata_schedule_348_0_e3313: f64 = (w[139] - w[142]);
            let noise_metadata_schedule_348_0_e3314: f64 = (noise_metadata_schedule_348_0_e3310 * noise_metadata_schedule_348_0_e3313);
            let noise_metadata_schedule_348_0_e3315: f64 = (noise_metadata_schedule_348_0_e3307 + noise_metadata_schedule_348_0_e3314);
            w[144] = noise_metadata_schedule_348_0_e3315;
        }
        if (active[0] & 0x9ffe7) != 0 {
            let noise_metadata_schedule_349_0_e3318: f64 = (1.0 - w[25]);
            let noise_metadata_schedule_349_0_e3320: f64 = (noise_metadata_schedule_349_0_e3318 * w[144]);
            let noise_metadata_schedule_349_0_e3323: f64 = (w[25] * w[244]);
            let noise_metadata_schedule_349_0_e3324: f64 = (noise_metadata_schedule_349_0_e3320 + noise_metadata_schedule_349_0_e3323);
            w[145] = noise_metadata_schedule_349_0_e3324;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_350_0_e3327: f64 = (4.0 * w[35]);
            let noise_metadata_schedule_350_0_e3329: f64 = (noise_metadata_schedule_350_0_e3327 / w[36]);
            w[146] = noise_metadata_schedule_350_0_e3329;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_351_0_e3332: f64 = (w[146] * w[266]);
            w[147] = noise_metadata_schedule_351_0_e3332;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_352_0_e3337: f64 = (1.0 + w[147]);
            let noise_metadata_schedule_352_0_e3338: f64 = (noise_metadata_schedule_352_0_e3337).sqrt();
            let noise_metadata_schedule_352_0_e3339: f64 = (1.0 + noise_metadata_schedule_352_0_e3338);
            let noise_metadata_schedule_352_0_e3340: f64 = (w[147] / noise_metadata_schedule_352_0_e3339);
            w[149] = noise_metadata_schedule_352_0_e3340;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_353_0_e3344: f64 = (1.0 / w[49]);
            let noise_metadata_schedule_353_0_e3345: f64 = (w[128]).powf(noise_metadata_schedule_353_0_e3344);
            w[129] = noise_metadata_schedule_353_0_e3345;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_354_0_e3348: f64 = (w[146] * w[129]);
            w[148] = noise_metadata_schedule_354_0_e3348;
        }
        if (active[0] & 0x187e7) != 0 {
            let noise_metadata_schedule_355_0_e3353: f64 = (1.0 + w[148]);
            let noise_metadata_schedule_355_0_e3354: f64 = (noise_metadata_schedule_355_0_e3353).sqrt();
            let noise_metadata_schedule_355_0_e3355: f64 = (1.0 + noise_metadata_schedule_355_0_e3354);
            let noise_metadata_schedule_355_0_e3356: f64 = (w[148] / noise_metadata_schedule_355_0_e3355);
            w[150] = noise_metadata_schedule_355_0_e3356;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_356_0_e3359: f64 = if params[92] == 0.0 { 1.0 } else { 0.0 };
            w[530] = noise_metadata_schedule_356_0_e3359;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_357_0_e3371,) = {
    if (w[530] != 0.0) {
        let noise_metadata_schedule_357_0_e3364: f64 = (w[138] / w[41]);
        let noise_metadata_schedule_357_0_e3365: f64 = (1.0 + noise_metadata_schedule_357_0_e3364);
        let noise_metadata_schedule_357_0_e3368: f64 = (w[145] / w[40]);
        let noise_metadata_schedule_357_0_e3369: f64 = (noise_metadata_schedule_357_0_e3365 + noise_metadata_schedule_357_0_e3368);
        (noise_metadata_schedule_357_0_e3369,)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_357_0_e3371;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_358_0_e3384,) = {
    if (w[530] == 0.0) {
        let noise_metadata_schedule_358_0_e3376: f64 = (w[138] / w[41]);
        let noise_metadata_schedule_358_0_e3378: f64 = (noise_metadata_schedule_358_0_e3376 + 1.0);
        let noise_metadata_schedule_358_0_e3380: f64 = (noise_metadata_schedule_358_0_e3378 * w[99]);
        let noise_metadata_schedule_358_0_e3382: f64 = (noise_metadata_schedule_358_0_e3380 * w[8]);
        (noise_metadata_schedule_358_0_e3382,)
    } else {
        (w[289],)
    }
};
            w[289] = noise_metadata_schedule_358_0_e3384;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_359_0_e3396,) = {
    if (w[530] == 0.0) {
        let noise_metadata_schedule_359_0_e3388: f64 = (-w[145]);
        let noise_metadata_schedule_359_0_e3390: f64 = (noise_metadata_schedule_359_0_e3388 / w[40]);
        let noise_metadata_schedule_359_0_e3392: f64 = (noise_metadata_schedule_359_0_e3390 * w[99]);
        let noise_metadata_schedule_359_0_e3394: f64 = (noise_metadata_schedule_359_0_e3392 * w[8]);
        (noise_metadata_schedule_359_0_e3394,)
    } else {
        (w[290],)
    }
};
            w[290] = noise_metadata_schedule_359_0_e3396;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let (noise_metadata_schedule_360_0_e3412,) = {
    if (w[530] == 0.0) {
        let noise_metadata_schedule_360_0_e3400: f64 = (w[289]).exp();
        let noise_metadata_schedule_360_0_e3402: f64 = (w[290]).exp();
        let noise_metadata_schedule_360_0_e3403: f64 = (noise_metadata_schedule_360_0_e3400 - noise_metadata_schedule_360_0_e3402);
        let noise_metadata_schedule_360_0_e3406: f64 = (w[99] * w[8]);
        let noise_metadata_schedule_360_0_e3407: f64 = (noise_metadata_schedule_360_0_e3406).exp();
        let noise_metadata_schedule_360_0_e3409: f64 = (noise_metadata_schedule_360_0_e3407 - 1.0);
        let noise_metadata_schedule_360_0_e3410: f64 = (noise_metadata_schedule_360_0_e3403 / noise_metadata_schedule_360_0_e3409);
        (noise_metadata_schedule_360_0_e3410,)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_360_0_e3412;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_361_0_e3415: f64 = (0.1 * 0.1);
            w[281] = noise_metadata_schedule_361_0_e3415;
        }
        if (active[0] & 0x9ffc7) != 0 {
            let noise_metadata_schedule_362_0_e3418: f64 = (w[151] * w[151]);
            w[282] = noise_metadata_schedule_362_0_e3418;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_363_0_e3421: f64 = if w[151] < 0.0 { 1.0 } else { 0.0 };
            w[531] = noise_metadata_schedule_363_0_e3421;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_364_0_e3434,) = {
    if (w[531] != 0.0) {
        let noise_metadata_schedule_364_0_e3425: f64 = (0.5 * w[281]);
        let noise_metadata_schedule_364_0_e3428: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_364_0_e3429: f64 = (noise_metadata_schedule_364_0_e3428).sqrt();
        let noise_metadata_schedule_364_0_e3431: f64 = (noise_metadata_schedule_364_0_e3429 - w[151]);
        let noise_metadata_schedule_364_0_e3432: f64 = (noise_metadata_schedule_364_0_e3425 / noise_metadata_schedule_364_0_e3431);
        (noise_metadata_schedule_364_0_e3432,)
    } else {
        (w[152],)
    }
};
            w[152] = noise_metadata_schedule_364_0_e3434;
        }
        if (active[0] & 0x187c7) != 0 {
            let (noise_metadata_schedule_365_0_e3446,) = {
    if (w[531] == 0.0) {
        let noise_metadata_schedule_365_0_e3440: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_365_0_e3441: f64 = (noise_metadata_schedule_365_0_e3440).sqrt();
        let noise_metadata_schedule_365_0_e3443: f64 = (noise_metadata_schedule_365_0_e3441 + w[151]);
        let noise_metadata_schedule_365_0_e3444: f64 = (0.5 * noise_metadata_schedule_365_0_e3443);
        (noise_metadata_schedule_365_0_e3444,)
    } else {
        (w[152],)
    }
};
            w[152] = noise_metadata_schedule_365_0_e3446;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_366_0_e3452: f64 = (w[149] + w[150]);
            let noise_metadata_schedule_366_0_e3453: f64 = (0.5 * noise_metadata_schedule_366_0_e3452);
            let noise_metadata_schedule_366_0_e3454: f64 = (1.0 + noise_metadata_schedule_366_0_e3453);
            let noise_metadata_schedule_366_0_e3455: f64 = (w[152] * noise_metadata_schedule_366_0_e3454);
            w[153] = noise_metadata_schedule_366_0_e3455;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_367_0_e3458: f64 = (params[15] * w[35]);
            let noise_metadata_schedule_367_0_e3460: f64 = (noise_metadata_schedule_367_0_e3458 * w[129]);
            w[154] = noise_metadata_schedule_367_0_e3460;
        }
        if (active[0] & 0x187c7) != 0 {
            let noise_metadata_schedule_368_0_e3463: f64 = (w[35] * w[266]);
            w[155] = noise_metadata_schedule_368_0_e3463;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_369_0_e3466: f64 = (w[155] - w[154]);
            let noise_metadata_schedule_369_0_e3468: f64 = (noise_metadata_schedule_369_0_e3466 / w[153]);
            w[156] = noise_metadata_schedule_369_0_e3468;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_370_0_e3471: f64 = w[246];
            let noise_metadata_schedule_370_0_e3473: f64 = (noise_metadata_schedule_370_0_e3471 / 0.0001);
            w[279] = noise_metadata_schedule_370_0_e3473;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_371_0_e3476: f64 = if w[246] < 0.0 { 1.0 } else { 0.0 };
            w[532] = noise_metadata_schedule_371_0_e3476;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_372_0_e3488,) = {
    if (w[532] != 0.0) {
        let noise_metadata_schedule_372_0_e3482: f64 = (w[279]).exp();
        let noise_metadata_schedule_372_0_e3483: f64 = (1.0 + noise_metadata_schedule_372_0_e3482);
        let noise_metadata_schedule_372_0_e3484: f64 = (noise_metadata_schedule_372_0_e3483).ln();
        let noise_metadata_schedule_372_0_e3485: f64 = (0.0001 * noise_metadata_schedule_372_0_e3484);
        let noise_metadata_schedule_372_0_e3486: f64 = noise_metadata_schedule_372_0_e3485;
        (noise_metadata_schedule_372_0_e3486,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_372_0_e3488;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_373_0_e3502,) = {
    if (w[532] == 0.0) {
        let noise_metadata_schedule_373_0_e3495: f64 = (-w[279]);
        let noise_metadata_schedule_373_0_e3496: f64 = (noise_metadata_schedule_373_0_e3495).exp();
        let noise_metadata_schedule_373_0_e3497: f64 = (1.0 + noise_metadata_schedule_373_0_e3496);
        let noise_metadata_schedule_373_0_e3498: f64 = (noise_metadata_schedule_373_0_e3497).ln();
        let noise_metadata_schedule_373_0_e3499: f64 = (0.0001 * noise_metadata_schedule_373_0_e3498);
        let noise_metadata_schedule_373_0_e3500: f64 = (w[246] + noise_metadata_schedule_373_0_e3499);
        (noise_metadata_schedule_373_0_e3500,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_373_0_e3502;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_374_0_e3505: f64 = (w[296] / params[152]);
            w[298] = noise_metadata_schedule_374_0_e3505;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_375_0_e3508: f64 = if w[298] < params[147] { 1.0 } else { 0.0 };
            w[533] = noise_metadata_schedule_375_0_e3508;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_376_0_e3513,) = {
    if (w[533] != 0.0) {
        let noise_metadata_schedule_376_0_e3511: f64 = (w[298]).exp();
        (noise_metadata_schedule_376_0_e3511,)
    } else {
        (w[299],)
    }
};
            w[299] = noise_metadata_schedule_376_0_e3513;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_377_0_e3519,) = {
    if (w[533] == 0.0) {
        let noise_metadata_schedule_377_0_e3517: f64 = (params[147]).exp();
        (noise_metadata_schedule_377_0_e3517,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_377_0_e3519;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_378_0_e3530,) = {
    if (w[533] == 0.0) {
        let noise_metadata_schedule_378_0_e3526: f64 = (w[298] - params[147]);
        let noise_metadata_schedule_378_0_e3527: f64 = (1.0 + noise_metadata_schedule_378_0_e3526);
        let noise_metadata_schedule_378_0_e3528: f64 = (w[295] * noise_metadata_schedule_378_0_e3527);
        (noise_metadata_schedule_378_0_e3528,)
    } else {
        (w[299],)
    }
};
            w[299] = noise_metadata_schedule_378_0_e3530;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_379_0_e3534: f64 = (w[299] - 1.0);
            let noise_metadata_schedule_379_0_e3535: f64 = (w[350] * noise_metadata_schedule_379_0_e3534);
            w[351] = noise_metadata_schedule_379_0_e3535;
        }
        if (active[0] & 0x6) != 0 {
            let noise_metadata_schedule_380_0_e3538: f64 = (w[246] - params[154]);
            let noise_metadata_schedule_380_0_e3540: f64 = (noise_metadata_schedule_380_0_e3538 / 0.001);
            w[279] = noise_metadata_schedule_380_0_e3540;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_381_0_e3543: f64 = if w[246] < params[154] { 1.0 } else { 0.0 };
            w[534] = noise_metadata_schedule_381_0_e3543;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_382_0_e3555,) = {
    if (w[534] != 0.0) {
        let noise_metadata_schedule_382_0_e3549: f64 = (w[279]).exp();
        let noise_metadata_schedule_382_0_e3550: f64 = (1.0 + noise_metadata_schedule_382_0_e3549);
        let noise_metadata_schedule_382_0_e3551: f64 = (noise_metadata_schedule_382_0_e3550).ln();
        let noise_metadata_schedule_382_0_e3552: f64 = (0.001 * noise_metadata_schedule_382_0_e3551);
        let noise_metadata_schedule_382_0_e3553: f64 = (w[246] - noise_metadata_schedule_382_0_e3552);
        (noise_metadata_schedule_382_0_e3553,)
    } else {
        (w[300],)
    }
};
            w[300] = noise_metadata_schedule_382_0_e3555;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_383_0_e3569,) = {
    if (w[534] == 0.0) {
        let noise_metadata_schedule_383_0_e3562: f64 = (-w[279]);
        let noise_metadata_schedule_383_0_e3563: f64 = (noise_metadata_schedule_383_0_e3562).exp();
        let noise_metadata_schedule_383_0_e3564: f64 = (1.0 + noise_metadata_schedule_383_0_e3563);
        let noise_metadata_schedule_383_0_e3565: f64 = (noise_metadata_schedule_383_0_e3564).ln();
        let noise_metadata_schedule_383_0_e3566: f64 = (0.001 * noise_metadata_schedule_383_0_e3565);
        let noise_metadata_schedule_383_0_e3567: f64 = (params[154] - noise_metadata_schedule_383_0_e3566);
        (noise_metadata_schedule_383_0_e3567,)
    } else {
        (w[300],)
    }
};
            w[300] = noise_metadata_schedule_383_0_e3569;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_384_0_e3572: f64 = (params[155] * w[300]);
            let noise_metadata_schedule_384_0_e3575: f64 = (params[154] - w[300]);
            let noise_metadata_schedule_384_0_e3577: f64 = {let pb=noise_metadata_schedule_384_0_e3575;pb*pb};
            let noise_metadata_schedule_384_0_e3578: f64 = (noise_metadata_schedule_384_0_e3572 * noise_metadata_schedule_384_0_e3577);
            w[352] = noise_metadata_schedule_384_0_e3578;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_385_0_e3581: f64 = (w[246] * w[8]);
            let noise_metadata_schedule_385_0_e3583: f64 = (noise_metadata_schedule_385_0_e3581 / params[17]);
            let noise_metadata_schedule_385_0_e3585: f64 = if noise_metadata_schedule_385_0_e3583 < params[147] { 1.0 } else { 0.0 };
            w[535] = noise_metadata_schedule_385_0_e3585;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_386_0_e3594,) = {
    if (w[535] != 0.0) {
        let noise_metadata_schedule_386_0_e3589: f64 = (w[246] * w[8]);
        let noise_metadata_schedule_386_0_e3591: f64 = (noise_metadata_schedule_386_0_e3589 / params[17]);
        let noise_metadata_schedule_386_0_e3592: f64 = (noise_metadata_schedule_386_0_e3591).exp();
        (noise_metadata_schedule_386_0_e3592,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_386_0_e3594;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_387_0_e3600,) = {
    if (w[535] == 0.0) {
        let noise_metadata_schedule_387_0_e3598: f64 = (params[147]).exp();
        (noise_metadata_schedule_387_0_e3598,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_387_0_e3600;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_388_0_e3615,) = {
    if (w[535] == 0.0) {
        let noise_metadata_schedule_388_0_e3607: f64 = (w[246] * w[8]);
        let noise_metadata_schedule_388_0_e3609: f64 = (noise_metadata_schedule_388_0_e3607 / params[17]);
        let noise_metadata_schedule_388_0_e3611: f64 = (noise_metadata_schedule_388_0_e3609 - params[147]);
        let noise_metadata_schedule_388_0_e3612: f64 = (1.0 + noise_metadata_schedule_388_0_e3611);
        let noise_metadata_schedule_388_0_e3613: f64 = (w[295] * noise_metadata_schedule_388_0_e3612);
        (noise_metadata_schedule_388_0_e3613,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_388_0_e3615;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_389_0_e3618: f64 = if params[24] == 1.0 { 1.0 } else { 0.0 };
            w[536] = noise_metadata_schedule_389_0_e3618;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_390_0_e3621: f64 = (w[246] - w[55]);
            let noise_metadata_schedule_390_0_e3623: f64 = (noise_metadata_schedule_390_0_e3621 * w[8]);
            let noise_metadata_schedule_390_0_e3625: f64 = if noise_metadata_schedule_390_0_e3623 < params[147] { 1.0 } else { 0.0 };
            w[537] = noise_metadata_schedule_390_0_e3625;
        }
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_391_0_e3636,) = {
    if ((w[536] != 0.0) && (w[537] != 0.0)) {
        let noise_metadata_schedule_391_0_e3631: f64 = (w[246] - w[55]);
        let noise_metadata_schedule_391_0_e3633: f64 = (noise_metadata_schedule_391_0_e3631 * w[8]);
        let noise_metadata_schedule_391_0_e3634: f64 = (noise_metadata_schedule_391_0_e3633).exp();
        (noise_metadata_schedule_391_0_e3634,)
    } else {
        (w[298],)
    }
};
            w[298] = noise_metadata_schedule_391_0_e3636;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_392_0_e3644,) = {
    if ((w[536] != 0.0) && (w[537] == 0.0)) {
        let noise_metadata_schedule_392_0_e3642: f64 = (params[147]).exp();
        (noise_metadata_schedule_392_0_e3642,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_392_0_e3644;
        }
        if (active[0] & 0x144) != 0 {
            let (noise_metadata_schedule_393_0_e3661,) = {
    if ((w[536] != 0.0) && (w[537] == 0.0)) {
        let noise_metadata_schedule_393_0_e3653: f64 = (w[246] - w[55]);
        let noise_metadata_schedule_393_0_e3655: f64 = (noise_metadata_schedule_393_0_e3653 * w[8]);
        let noise_metadata_schedule_393_0_e3657: f64 = (noise_metadata_schedule_393_0_e3655 - params[147]);
        let noise_metadata_schedule_393_0_e3658: f64 = (1.0 + noise_metadata_schedule_393_0_e3657);
        let noise_metadata_schedule_393_0_e3659: f64 = (w[295] * noise_metadata_schedule_393_0_e3658);
        (noise_metadata_schedule_393_0_e3659,)
    } else {
        (w[298],)
    }
};
            w[298] = noise_metadata_schedule_393_0_e3661;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_394_0_e3664: f64 = (w[156] / w[35]);
            let noise_metadata_schedule_394_0_e3666: f64 = (noise_metadata_schedule_394_0_e3664 - 1000.0);
            let noise_metadata_schedule_394_0_e3668: f64 = if noise_metadata_schedule_394_0_e3666 < 40.0 { 1.0 } else { 0.0 };
            w[538] = noise_metadata_schedule_394_0_e3668;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_395_0_e3679,) = {
    if ((w[536] != 0.0) && (w[538] != 0.0)) {
        let noise_metadata_schedule_395_0_e3674: f64 = (w[156] / w[35]);
        let noise_metadata_schedule_395_0_e3676: f64 = (noise_metadata_schedule_395_0_e3674 - 1000.0);
        let noise_metadata_schedule_395_0_e3677: f64 = (noise_metadata_schedule_395_0_e3676).exp();
        (noise_metadata_schedule_395_0_e3677,)
    } else {
        (w[299],)
    }
};
            w[299] = noise_metadata_schedule_395_0_e3679;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_396_0_e3687,) = {
    if ((w[536] != 0.0) && (w[538] == 0.0)) {
        let noise_metadata_schedule_396_0_e3685: f64 = (40.0_f64).exp();
        (noise_metadata_schedule_396_0_e3685,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_396_0_e3687;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_397_0_e3704,) = {
    if ((w[536] != 0.0) && (w[538] == 0.0)) {
        let noise_metadata_schedule_397_0_e3696: f64 = (w[156] / w[35]);
        let noise_metadata_schedule_397_0_e3698: f64 = (noise_metadata_schedule_397_0_e3696 - 1000.0);
        let noise_metadata_schedule_397_0_e3700: f64 = (noise_metadata_schedule_397_0_e3698 - 40.0);
        let noise_metadata_schedule_397_0_e3701: f64 = (1.0 + noise_metadata_schedule_397_0_e3700);
        let noise_metadata_schedule_397_0_e3702: f64 = (w[295] * noise_metadata_schedule_397_0_e3701);
        (noise_metadata_schedule_397_0_e3702,)
    } else {
        (w[299],)
    }
};
            w[299] = noise_metadata_schedule_397_0_e3704;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_398_0_e3747,) = {
    if (w[536] != 0.0) {
        let noise_metadata_schedule_398_0_e3709: f64 = (w[296] - 1.0);
        let noise_metadata_schedule_398_0_e3710: f64 = (w[42] * noise_metadata_schedule_398_0_e3709);
        let noise_metadata_schedule_398_0_e3713: f64 = (w[53] * 2.0);
        let noise_metadata_schedule_398_0_e3716: f64 = (w[296] - 1.0);
        let noise_metadata_schedule_398_0_e3717: f64 = (noise_metadata_schedule_398_0_e3713 * noise_metadata_schedule_398_0_e3716);
        let noise_metadata_schedule_398_0_e3722: f64 = (4.0 * w[298]);
        let noise_metadata_schedule_398_0_e3723: f64 = (1.0 + noise_metadata_schedule_398_0_e3722);
        let noise_metadata_schedule_398_0_e3724: f64 = (noise_metadata_schedule_398_0_e3723).sqrt();
        let noise_metadata_schedule_398_0_e3725: f64 = (1.0 + noise_metadata_schedule_398_0_e3724);
        let noise_metadata_schedule_398_0_e3726: f64 = (noise_metadata_schedule_398_0_e3717 / noise_metadata_schedule_398_0_e3725);
        let noise_metadata_schedule_398_0_e3730: f64 = (w[145] / w[40]);
        let noise_metadata_schedule_398_0_e3731: f64 = (1.0 + noise_metadata_schedule_398_0_e3730);
        let noise_metadata_schedule_398_0_e3732: f64 = (noise_metadata_schedule_398_0_e3726 * noise_metadata_schedule_398_0_e3731);
        let noise_metadata_schedule_398_0_e3733: f64 = (noise_metadata_schedule_398_0_e3710 + noise_metadata_schedule_398_0_e3732);
        let noise_metadata_schedule_398_0_e3737: f64 = (w[128] - 1.0);
        let noise_metadata_schedule_398_0_e3738: f64 = (w[54] * noise_metadata_schedule_398_0_e3737);
        let noise_metadata_schedule_398_0_e3740: f64 = (noise_metadata_schedule_398_0_e3738 * w[299]);
        let noise_metadata_schedule_398_0_e3743: f64 = (1.0 + w[299]);
        let noise_metadata_schedule_398_0_e3744: f64 = (noise_metadata_schedule_398_0_e3740 / noise_metadata_schedule_398_0_e3743);
        let noise_metadata_schedule_398_0_e3745: f64 = (noise_metadata_schedule_398_0_e3733 + noise_metadata_schedule_398_0_e3744);
        (noise_metadata_schedule_398_0_e3745,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_398_0_e3747;
        }
        if (active[0] & 0x44) != 0 {
            let noise_metadata_schedule_399_0_e3750: f64 = if params[93] == 0.0 { 1.0 } else { 0.0 };
            w[539] = noise_metadata_schedule_399_0_e3750;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_400_0_e3761,) = {
    if ((w[536] == 0.0) && (w[539] != 0.0)) {
        let noise_metadata_schedule_400_0_e3758: f64 = (w[296] - 1.0);
        let noise_metadata_schedule_400_0_e3759: f64 = (w[42] * noise_metadata_schedule_400_0_e3758);
        (noise_metadata_schedule_400_0_e3759,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_400_0_e3761;
        }
        if (active[0] & 0x44) != 0 {
            let (noise_metadata_schedule_401_0_e3791,) = {
    if ((w[536] == 0.0) && (w[539] == 0.0)) {
        let noise_metadata_schedule_401_0_e3770: f64 = (1.0 - params[93]);
        let noise_metadata_schedule_401_0_e3773: f64 = (w[296] - 1.0);
        let noise_metadata_schedule_401_0_e3774: f64 = (noise_metadata_schedule_401_0_e3770 * noise_metadata_schedule_401_0_e3773);
        let noise_metadata_schedule_401_0_e3778: f64 = (w[296] + w[128]);
        let noise_metadata_schedule_401_0_e3780: f64 = (noise_metadata_schedule_401_0_e3778 - 2.0);
        let noise_metadata_schedule_401_0_e3781: f64 = (params[93] * noise_metadata_schedule_401_0_e3780);
        let noise_metadata_schedule_401_0_e3785: f64 = (w[145] / w[40]);
        let noise_metadata_schedule_401_0_e3786: f64 = (1.0 + noise_metadata_schedule_401_0_e3785);
        let noise_metadata_schedule_401_0_e3787: f64 = (noise_metadata_schedule_401_0_e3781 * noise_metadata_schedule_401_0_e3786);
        let noise_metadata_schedule_401_0_e3788: f64 = (noise_metadata_schedule_401_0_e3774 + noise_metadata_schedule_401_0_e3787);
        let noise_metadata_schedule_401_0_e3789: f64 = (w[42] * noise_metadata_schedule_401_0_e3788);
        (noise_metadata_schedule_401_0_e3789,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_401_0_e3791;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_402_0_e3794: f64 = (w[247] * w[8]);
            let noise_metadata_schedule_402_0_e3796: f64 = (noise_metadata_schedule_402_0_e3794 / params[19]);
            let noise_metadata_schedule_402_0_e3798: f64 = if noise_metadata_schedule_402_0_e3796 < params[147] { 1.0 } else { 0.0 };
            w[540] = noise_metadata_schedule_402_0_e3798;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_403_0_e3807,) = {
    if (w[540] != 0.0) {
        let noise_metadata_schedule_403_0_e3802: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_403_0_e3804: f64 = (noise_metadata_schedule_403_0_e3802 / params[19]);
        let noise_metadata_schedule_403_0_e3805: f64 = (noise_metadata_schedule_403_0_e3804).exp();
        (noise_metadata_schedule_403_0_e3805,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_403_0_e3807;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_404_0_e3813,) = {
    if (w[540] == 0.0) {
        let noise_metadata_schedule_404_0_e3811: f64 = (params[147]).exp();
        (noise_metadata_schedule_404_0_e3811,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_404_0_e3813;
        }
        if (active[0] & 0x7c4) != 0 {
            let (noise_metadata_schedule_405_0_e3828,) = {
    if (w[540] == 0.0) {
        let noise_metadata_schedule_405_0_e3820: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_405_0_e3822: f64 = (noise_metadata_schedule_405_0_e3820 / params[19]);
        let noise_metadata_schedule_405_0_e3824: f64 = (noise_metadata_schedule_405_0_e3822 - params[147]);
        let noise_metadata_schedule_405_0_e3825: f64 = (1.0 + noise_metadata_schedule_405_0_e3824);
        let noise_metadata_schedule_405_0_e3826: f64 = (w[295] * noise_metadata_schedule_405_0_e3825);
        (noise_metadata_schedule_405_0_e3826,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_405_0_e3828;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_406_0_e3831: f64 = if params[24] == 1.0 { 1.0 } else { 0.0 };
            w[541] = noise_metadata_schedule_406_0_e3831;
        }
        if (active[0] & 0x187c6) != 0 {
            let noise_metadata_schedule_407_0_e3834: f64 = (w[247] - w[55]);
            let noise_metadata_schedule_407_0_e3836: f64 = (noise_metadata_schedule_407_0_e3834 * w[8]);
            let noise_metadata_schedule_407_0_e3838: f64 = if noise_metadata_schedule_407_0_e3836 < params[147] { 1.0 } else { 0.0 };
            w[542] = noise_metadata_schedule_407_0_e3838;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_408_0_e3849,) = {
    if ((w[541] != 0.0) && (w[542] != 0.0)) {
        let noise_metadata_schedule_408_0_e3844: f64 = (w[247] - w[55]);
        let noise_metadata_schedule_408_0_e3846: f64 = (noise_metadata_schedule_408_0_e3844 * w[8]);
        let noise_metadata_schedule_408_0_e3847: f64 = (noise_metadata_schedule_408_0_e3846).exp();
        (noise_metadata_schedule_408_0_e3847,)
    } else {
        (w[298],)
    }
};
            w[298] = noise_metadata_schedule_408_0_e3849;
        }
        if (active[0] & 0x187c6) != 0 {
            let (noise_metadata_schedule_409_0_e3857,) = {
    if ((w[541] != 0.0) && (w[542] == 0.0)) {
        let noise_metadata_schedule_409_0_e3855: f64 = (params[147]).exp();
        (noise_metadata_schedule_409_0_e3855,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_409_0_e3857;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_410_0_e3874,) = {
    if ((w[541] != 0.0) && (w[542] == 0.0)) {
        let noise_metadata_schedule_410_0_e3866: f64 = (w[247] - w[55]);
        let noise_metadata_schedule_410_0_e3868: f64 = (noise_metadata_schedule_410_0_e3866 * w[8]);
        let noise_metadata_schedule_410_0_e3870: f64 = (noise_metadata_schedule_410_0_e3868 - params[147]);
        let noise_metadata_schedule_410_0_e3871: f64 = (1.0 + noise_metadata_schedule_410_0_e3870);
        let noise_metadata_schedule_410_0_e3872: f64 = (w[295] * noise_metadata_schedule_410_0_e3871);
        (noise_metadata_schedule_410_0_e3872,)
    } else {
        (w[298],)
    }
};
            w[298] = noise_metadata_schedule_410_0_e3874;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_411_0_e3899,) = {
    if (w[541] != 0.0) {
        let noise_metadata_schedule_411_0_e3879: f64 = (w[296] - 1.0);
        let noise_metadata_schedule_411_0_e3880: f64 = (w[44] * noise_metadata_schedule_411_0_e3879);
        let noise_metadata_schedule_411_0_e3883: f64 = (w[45] * 2.0);
        let noise_metadata_schedule_411_0_e3886: f64 = (w[296] - 1.0);
        let noise_metadata_schedule_411_0_e3887: f64 = (noise_metadata_schedule_411_0_e3883 * noise_metadata_schedule_411_0_e3886);
        let noise_metadata_schedule_411_0_e3892: f64 = (4.0 * w[298]);
        let noise_metadata_schedule_411_0_e3893: f64 = (1.0 + noise_metadata_schedule_411_0_e3892);
        let noise_metadata_schedule_411_0_e3894: f64 = (noise_metadata_schedule_411_0_e3893).sqrt();
        let noise_metadata_schedule_411_0_e3895: f64 = (1.0 + noise_metadata_schedule_411_0_e3894);
        let noise_metadata_schedule_411_0_e3896: f64 = (noise_metadata_schedule_411_0_e3887 / noise_metadata_schedule_411_0_e3895);
        let noise_metadata_schedule_411_0_e3897: f64 = (noise_metadata_schedule_411_0_e3880 + noise_metadata_schedule_411_0_e3896);
        (noise_metadata_schedule_411_0_e3897,)
    } else {
        (w[159],)
    }
};
            w[159] = noise_metadata_schedule_411_0_e3899;
        }
        if (active[0] & 0x140) != 0 {
            let (noise_metadata_schedule_412_0_e3908,) = {
    if (w[541] == 0.0) {
        let noise_metadata_schedule_412_0_e3905: f64 = (w[296] - 1.0);
        let noise_metadata_schedule_412_0_e3906: f64 = (w[44] * noise_metadata_schedule_412_0_e3905);
        (noise_metadata_schedule_412_0_e3906,)
    } else {
        (w[159],)
    }
};
            w[159] = noise_metadata_schedule_412_0_e3908;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_413_0_e3911: f64 = (w[246] * w[8]);
            let noise_metadata_schedule_413_0_e3913: f64 = (noise_metadata_schedule_413_0_e3911 / params[21]);
            let noise_metadata_schedule_413_0_e3915: f64 = if noise_metadata_schedule_413_0_e3913 < params[147] { 1.0 } else { 0.0 };
            w[543] = noise_metadata_schedule_413_0_e3915;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_414_0_e3924,) = {
    if (w[543] != 0.0) {
        let noise_metadata_schedule_414_0_e3919: f64 = (w[246] * w[8]);
        let noise_metadata_schedule_414_0_e3921: f64 = (noise_metadata_schedule_414_0_e3919 / params[21]);
        let noise_metadata_schedule_414_0_e3922: f64 = (noise_metadata_schedule_414_0_e3921).exp();
        (noise_metadata_schedule_414_0_e3922,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_414_0_e3924;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_415_0_e3930,) = {
    if (w[543] == 0.0) {
        let noise_metadata_schedule_415_0_e3928: f64 = (params[147]).exp();
        (noise_metadata_schedule_415_0_e3928,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_415_0_e3930;
        }
        if (active[0] & 0x784) != 0 {
            let (noise_metadata_schedule_416_0_e3945,) = {
    if (w[543] == 0.0) {
        let noise_metadata_schedule_416_0_e3937: f64 = (w[246] * w[8]);
        let noise_metadata_schedule_416_0_e3939: f64 = (noise_metadata_schedule_416_0_e3937 / params[21]);
        let noise_metadata_schedule_416_0_e3941: f64 = (noise_metadata_schedule_416_0_e3939 - params[147]);
        let noise_metadata_schedule_416_0_e3942: f64 = (1.0 + noise_metadata_schedule_416_0_e3941);
        let noise_metadata_schedule_416_0_e3943: f64 = (w[295] * noise_metadata_schedule_416_0_e3942);
        (noise_metadata_schedule_416_0_e3943,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_416_0_e3945;
        }
        if (active[0] & 0x84) != 0 {
            let noise_metadata_schedule_417_0_e3949: f64 = (w[296] - 1.0);
            let noise_metadata_schedule_417_0_e3950: f64 = (w[38] * noise_metadata_schedule_417_0_e3949);
            w[160] = noise_metadata_schedule_417_0_e3950;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_418_0_e3953: f64 = (w[247] * w[8]);
            let noise_metadata_schedule_418_0_e3955: f64 = (noise_metadata_schedule_418_0_e3953 / params[23]);
            let noise_metadata_schedule_418_0_e3957: f64 = if noise_metadata_schedule_418_0_e3955 < params[147] { 1.0 } else { 0.0 };
            w[544] = noise_metadata_schedule_418_0_e3957;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_419_0_e3966,) = {
    if (w[544] != 0.0) {
        let noise_metadata_schedule_419_0_e3961: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_419_0_e3963: f64 = (noise_metadata_schedule_419_0_e3961 / params[23]);
        let noise_metadata_schedule_419_0_e3964: f64 = (noise_metadata_schedule_419_0_e3963).exp();
        (noise_metadata_schedule_419_0_e3964,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_419_0_e3966;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_420_0_e3972,) = {
    if (w[544] == 0.0) {
        let noise_metadata_schedule_420_0_e3970: f64 = (params[147]).exp();
        (noise_metadata_schedule_420_0_e3970,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_420_0_e3972;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_421_0_e3987,) = {
    if (w[544] == 0.0) {
        let noise_metadata_schedule_421_0_e3979: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_421_0_e3981: f64 = (noise_metadata_schedule_421_0_e3979 / params[23]);
        let noise_metadata_schedule_421_0_e3983: f64 = (noise_metadata_schedule_421_0_e3981 - params[147]);
        let noise_metadata_schedule_421_0_e3984: f64 = (1.0 + noise_metadata_schedule_421_0_e3983);
        let noise_metadata_schedule_421_0_e3985: f64 = (w[295] * noise_metadata_schedule_421_0_e3984);
        (noise_metadata_schedule_421_0_e3985,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_421_0_e3987;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_422_0_e3991: f64 = (w[296] - 1.0);
            let noise_metadata_schedule_422_0_e3992: f64 = (w[46] * noise_metadata_schedule_422_0_e3991);
            w[162] = noise_metadata_schedule_422_0_e3992;
        }
        if (active[0] & 0x18786) != 0 {
            let noise_metadata_schedule_423_0_e3995: f64 = (w[249] * w[8]);
            let noise_metadata_schedule_423_0_e3997: f64 = (noise_metadata_schedule_423_0_e3995 / params[32]);
            let noise_metadata_schedule_423_0_e3999: f64 = if noise_metadata_schedule_423_0_e3997 < params[147] { 1.0 } else { 0.0 };
            w[545] = noise_metadata_schedule_423_0_e3999;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_424_0_e4008,) = {
    if (w[545] != 0.0) {
        let noise_metadata_schedule_424_0_e4003: f64 = (w[249] * w[8]);
        let noise_metadata_schedule_424_0_e4005: f64 = (noise_metadata_schedule_424_0_e4003 / params[32]);
        let noise_metadata_schedule_424_0_e4006: f64 = (noise_metadata_schedule_424_0_e4005).exp();
        (noise_metadata_schedule_424_0_e4006,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_424_0_e4008;
        }
        if (active[0] & 0x18786) != 0 {
            let (noise_metadata_schedule_425_0_e4014,) = {
    if (w[545] == 0.0) {
        let noise_metadata_schedule_425_0_e4012: f64 = (params[147]).exp();
        (noise_metadata_schedule_425_0_e4012,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_425_0_e4014;
        }
        if (active[0] & 0x780) != 0 {
            let (noise_metadata_schedule_426_0_e4029,) = {
    if (w[545] == 0.0) {
        let noise_metadata_schedule_426_0_e4021: f64 = (w[249] * w[8]);
        let noise_metadata_schedule_426_0_e4023: f64 = (noise_metadata_schedule_426_0_e4021 / params[32]);
        let noise_metadata_schedule_426_0_e4025: f64 = (noise_metadata_schedule_426_0_e4023 - params[147]);
        let noise_metadata_schedule_426_0_e4026: f64 = (1.0 + noise_metadata_schedule_426_0_e4025);
        let noise_metadata_schedule_426_0_e4027: f64 = (w[295] * noise_metadata_schedule_426_0_e4026);
        (noise_metadata_schedule_426_0_e4027,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_426_0_e4029;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_427_0_e4033: f64 = (w[296] - 1.0);
            let noise_metadata_schedule_427_0_e4034: f64 = (w[39] * noise_metadata_schedule_427_0_e4033);
            w[161] = noise_metadata_schedule_427_0_e4034;
        }
        if (active[0] & 0x18186) != 0 {
            let noise_metadata_schedule_428_0_e4037: f64 = (w[247] * w[8]);
            let noise_metadata_schedule_428_0_e4039: f64 = (noise_metadata_schedule_428_0_e4037 / params[146]);
            let noise_metadata_schedule_428_0_e4041: f64 = if noise_metadata_schedule_428_0_e4039 < params[147] { 1.0 } else { 0.0 };
            w[546] = noise_metadata_schedule_428_0_e4041;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_429_0_e4050,) = {
    if (w[546] != 0.0) {
        let noise_metadata_schedule_429_0_e4045: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_429_0_e4047: f64 = (noise_metadata_schedule_429_0_e4045 / params[146]);
        let noise_metadata_schedule_429_0_e4048: f64 = (noise_metadata_schedule_429_0_e4047).exp();
        (noise_metadata_schedule_429_0_e4048,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_429_0_e4050;
        }
        if (active[0] & 0x18186) != 0 {
            let (noise_metadata_schedule_430_0_e4056,) = {
    if (w[546] == 0.0) {
        let noise_metadata_schedule_430_0_e4054: f64 = (params[147]).exp();
        (noise_metadata_schedule_430_0_e4054,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_430_0_e4056;
        }
        if (active[0] & 0x180) != 0 {
            let (noise_metadata_schedule_431_0_e4071,) = {
    if (w[546] == 0.0) {
        let noise_metadata_schedule_431_0_e4063: f64 = (w[247] * w[8]);
        let noise_metadata_schedule_431_0_e4065: f64 = (noise_metadata_schedule_431_0_e4063 / params[146]);
        let noise_metadata_schedule_431_0_e4067: f64 = (noise_metadata_schedule_431_0_e4065 - params[147]);
        let noise_metadata_schedule_431_0_e4068: f64 = (1.0 + noise_metadata_schedule_431_0_e4067);
        let noise_metadata_schedule_431_0_e4069: f64 = (w[295] * noise_metadata_schedule_431_0_e4068);
        (noise_metadata_schedule_431_0_e4069,)
    } else {
        (w[296],)
    }
};
            w[296] = noise_metadata_schedule_431_0_e4071;
        }
        if (active[0] & 0x180) != 0 {
            let noise_metadata_schedule_432_0_e4075: f64 = (w[296] - 1.0);
            let noise_metadata_schedule_432_0_e4076: f64 = (w[47] * noise_metadata_schedule_432_0_e4075);
            w[163] = noise_metadata_schedule_432_0_e4076;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_433_0_e4087: f64 = if (((params[34] > 0.0) && (params[35] > 0.0)) && (w[246] < 0.0)) { 1.0 } else { 0.0 };
            w[547] = noise_metadata_schedule_433_0_e4087;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_434_0_e4093: f64 = (2.0 * w[59]);
            let noise_metadata_schedule_434_0_e4094: f64 = (w[62] / noise_metadata_schedule_434_0_e4093);
            let noise_metadata_schedule_434_0_e4095: f64 = (1.0 - noise_metadata_schedule_434_0_e4094);
            let noise_metadata_schedule_434_0_e4096: f64 = (w[61] * noise_metadata_schedule_434_0_e4095);
            let noise_metadata_schedule_434_0_e4098: f64 = if noise_metadata_schedule_434_0_e4096 < params[147] { 1.0 } else { 0.0 };
            w[548] = noise_metadata_schedule_434_0_e4098;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_435_0_e4113,) = {
    if ((w[547] != 0.0) && (w[548] != 0.0)) {
        let noise_metadata_schedule_435_0_e4107: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_435_0_e4108: f64 = (w[62] / noise_metadata_schedule_435_0_e4107);
        let noise_metadata_schedule_435_0_e4109: f64 = (1.0 - noise_metadata_schedule_435_0_e4108);
        let noise_metadata_schedule_435_0_e4110: f64 = (w[61] * noise_metadata_schedule_435_0_e4109);
        let noise_metadata_schedule_435_0_e4111: f64 = (noise_metadata_schedule_435_0_e4110).exp();
        (noise_metadata_schedule_435_0_e4111,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_435_0_e4113;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_436_0_e4121,) = {
    if ((w[547] != 0.0) && (w[548] == 0.0)) {
        let noise_metadata_schedule_436_0_e4119: f64 = (params[147]).exp();
        (noise_metadata_schedule_436_0_e4119,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_436_0_e4121;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_437_0_e4142,) = {
    if ((w[547] != 0.0) && (w[548] == 0.0)) {
        let noise_metadata_schedule_437_0_e4133: f64 = (2.0 * w[59]);
        let noise_metadata_schedule_437_0_e4134: f64 = (w[62] / noise_metadata_schedule_437_0_e4133);
        let noise_metadata_schedule_437_0_e4135: f64 = (1.0 - noise_metadata_schedule_437_0_e4134);
        let noise_metadata_schedule_437_0_e4136: f64 = (w[61] * noise_metadata_schedule_437_0_e4135);
        let noise_metadata_schedule_437_0_e4138: f64 = (noise_metadata_schedule_437_0_e4136 - params[147]);
        let noise_metadata_schedule_437_0_e4139: f64 = (1.0 + noise_metadata_schedule_437_0_e4138);
        let noise_metadata_schedule_437_0_e4140: f64 = (w[295] * noise_metadata_schedule_437_0_e4139);
        (noise_metadata_schedule_437_0_e4140,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_437_0_e4142;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_438_0_e4148,) = {
    if (w[547] != 0.0) {
        let noise_metadata_schedule_438_0_e4146: f64 = (w[246] * w[65]);
        (noise_metadata_schedule_438_0_e4146,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_438_0_e4148;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_439_0_e4192,) = {
    if (w[547] != 0.0) {
        let noise_metadata_schedule_439_0_e4152: f64 = (w[275] * w[275]);
        let noise_metadata_schedule_439_0_e4154: f64 = (noise_metadata_schedule_439_0_e4152 + 1e-30);
        let noise_metadata_schedule_439_0_e4155: f64 = (noise_metadata_schedule_439_0_e4154).sqrt();
        let noise_metadata_schedule_439_0_e4157: f64 = (-2.0);
        let noise_metadata_schedule_439_0_e4159: f64 = (noise_metadata_schedule_439_0_e4157 - params[67]);
        let noise_metadata_schedule_439_0_e4160: f64 = (noise_metadata_schedule_439_0_e4155).powf(noise_metadata_schedule_439_0_e4159);
        let noise_metadata_schedule_439_0_e4165: f64 = (params[67] * params[67]);
        let noise_metadata_schedule_439_0_e4166: f64 = (1.0 - noise_metadata_schedule_439_0_e4165);
        let noise_metadata_schedule_439_0_e4169: f64 = (3.0 * w[275]);
        let noise_metadata_schedule_439_0_e4172: f64 = (params[67] - 1.0);
        let noise_metadata_schedule_439_0_e4173: f64 = (noise_metadata_schedule_439_0_e4169 * noise_metadata_schedule_439_0_e4172);
        let noise_metadata_schedule_439_0_e4174: f64 = (noise_metadata_schedule_439_0_e4166 - noise_metadata_schedule_439_0_e4173);
        let noise_metadata_schedule_439_0_e4175: f64 = (params[67] * noise_metadata_schedule_439_0_e4174);
        let noise_metadata_schedule_439_0_e4178: f64 = (6.0 * w[275]);
        let noise_metadata_schedule_439_0_e4180: f64 = (noise_metadata_schedule_439_0_e4178 * w[275]);
        let noise_metadata_schedule_439_0_e4183: f64 = (params[67] - 1.0);
        let noise_metadata_schedule_439_0_e4185: f64 = (noise_metadata_schedule_439_0_e4183 + w[275]);
        let noise_metadata_schedule_439_0_e4186: f64 = (noise_metadata_schedule_439_0_e4180 * noise_metadata_schedule_439_0_e4185);
        let noise_metadata_schedule_439_0_e4187: f64 = (noise_metadata_schedule_439_0_e4175 - noise_metadata_schedule_439_0_e4186);
        let noise_metadata_schedule_439_0_e4188: f64 = (noise_metadata_schedule_439_0_e4160 * noise_metadata_schedule_439_0_e4187);
        let noise_metadata_schedule_439_0_e4190: f64 = (noise_metadata_schedule_439_0_e4188 * 0.16666666666666666);
        (noise_metadata_schedule_439_0_e4190,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_439_0_e4192;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_440_0_e4204,) = {
    if (w[547] != 0.0) {
        let noise_metadata_schedule_440_0_e4196: f64 = (w[246] * w[62]);
        let noise_metadata_schedule_440_0_e4198: f64 = (noise_metadata_schedule_440_0_e4196 * w[61]);
        let noise_metadata_schedule_440_0_e4201: f64 = (w[70] * w[60]);
        let noise_metadata_schedule_440_0_e4202: f64 = (noise_metadata_schedule_440_0_e4198 / noise_metadata_schedule_440_0_e4201);
        (noise_metadata_schedule_440_0_e4202,)
    } else {
        (w[275],)
    }
};
            w[275] = noise_metadata_schedule_440_0_e4204;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_441_0_e4207: f64 = (-0.001);
            let noise_metadata_schedule_441_0_e4208: f64 = if w[275] < noise_metadata_schedule_441_0_e4207 { 1.0 } else { 0.0 };
            w[549] = noise_metadata_schedule_441_0_e4208;
        }
        if (active[0] & 0x18006) != 0 {
            let noise_metadata_schedule_442_0_e4211: f64 = if w[275] < params[147] { 1.0 } else { 0.0 };
            w[550] = noise_metadata_schedule_442_0_e4211;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_443_0_e4220,) = {
    if (((w[547] != 0.0) && (w[549] != 0.0)) && (w[550] != 0.0)) {
        let noise_metadata_schedule_443_0_e4218: f64 = (w[275]).exp();
        (noise_metadata_schedule_443_0_e4218,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_443_0_e4220;
        }
        if (active[0] & 0x18006) != 0 {
            let (noise_metadata_schedule_444_0_e4230,) = {
    if (((w[547] != 0.0) && (w[549] != 0.0)) && (w[550] == 0.0)) {
        let noise_metadata_schedule_444_0_e4228: f64 = (params[147]).exp();
        (noise_metadata_schedule_444_0_e4228,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_444_0_e4230;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_445_0_e4245,) = {
    if (((w[547] != 0.0) && (w[549] != 0.0)) && (w[550] == 0.0)) {
        let noise_metadata_schedule_445_0_e4241: f64 = (w[275] - params[147]);
        let noise_metadata_schedule_445_0_e4242: f64 = (1.0 + noise_metadata_schedule_445_0_e4241);
        let noise_metadata_schedule_445_0_e4243: f64 = (w[295] * noise_metadata_schedule_445_0_e4242);
        (noise_metadata_schedule_445_0_e4243,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_445_0_e4245;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_446_0_e4260,) = {
    if ((w[547] != 0.0) && (w[549] != 0.0)) {
        let noise_metadata_schedule_446_0_e4250: f64 = (-w[246]);
        let noise_metadata_schedule_446_0_e4254: f64 = (1.0 - w[91]);
        let noise_metadata_schedule_446_0_e4256: f64 = (noise_metadata_schedule_446_0_e4254 / w[275]);
        let noise_metadata_schedule_446_0_e4257: f64 = (1.0 + noise_metadata_schedule_446_0_e4256);
        let noise_metadata_schedule_446_0_e4258: f64 = (noise_metadata_schedule_446_0_e4250 * noise_metadata_schedule_446_0_e4257);
        (noise_metadata_schedule_446_0_e4258,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_446_0_e4260;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_447_0_e4283,) = {
    if ((w[547] != 0.0) && (w[549] == 0.0)) {
        let noise_metadata_schedule_447_0_e4267: f64 = (w[246] * 0.5);
        let noise_metadata_schedule_447_0_e4269: f64 = (noise_metadata_schedule_447_0_e4267 * w[275]);
        let noise_metadata_schedule_447_0_e4273: f64 = (w[275] * 0.3333333333333333);
        let noise_metadata_schedule_447_0_e4277: f64 = (0.25 * w[275]);
        let noise_metadata_schedule_447_0_e4278: f64 = (1.0 + noise_metadata_schedule_447_0_e4277);
        let noise_metadata_schedule_447_0_e4279: f64 = (noise_metadata_schedule_447_0_e4273 * noise_metadata_schedule_447_0_e4278);
        let noise_metadata_schedule_447_0_e4280: f64 = (1.0 + noise_metadata_schedule_447_0_e4279);
        let noise_metadata_schedule_447_0_e4281: f64 = (noise_metadata_schedule_447_0_e4269 * noise_metadata_schedule_447_0_e4280);
        (noise_metadata_schedule_447_0_e4281,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_447_0_e4283;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_448_0_e4299,) = {
    if (w[547] != 0.0) {
        let noise_metadata_schedule_448_0_e4287: f64 = (2.0 * w[58]);
        let noise_metadata_schedule_448_0_e4289: f64 = (noise_metadata_schedule_448_0_e4287 * w[69]);
        let noise_metadata_schedule_448_0_e4291: f64 = (noise_metadata_schedule_448_0_e4289 * w[59]);
        let noise_metadata_schedule_448_0_e4293: f64 = (noise_metadata_schedule_448_0_e4291 * w[68]);
        let noise_metadata_schedule_448_0_e4295: f64 = (noise_metadata_schedule_448_0_e4293 * w[65]);
        let noise_metadata_schedule_448_0_e4297: f64 = (noise_metadata_schedule_448_0_e4295 * w[63]);
        (noise_metadata_schedule_448_0_e4297,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_448_0_e4299;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_450_0_e4309,) = {
    if (w[547] == 0.0) {
        (0.0,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_450_0_e4309;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_451_0_e4320: f64 = if (((params[36] > 0.0) && (params[37] > 0.0)) && (w[244] < 0.0)) { 1.0 } else { 0.0 };
            w[551] = noise_metadata_schedule_451_0_e4320;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_452_0_e4332,) = {
    if (w[551] != 0.0) {
        let noise_metadata_schedule_452_0_e4325: f64 = (w[244] * w[67]);
        let noise_metadata_schedule_452_0_e4326: f64 = (1.0 - noise_metadata_schedule_452_0_e4325);
        let noise_metadata_schedule_452_0_e4329: f64 = (1.0 - w[76]);
        let noise_metadata_schedule_452_0_e4330: f64 = (noise_metadata_schedule_452_0_e4326).powf(noise_metadata_schedule_452_0_e4329);
        (noise_metadata_schedule_452_0_e4330,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_452_0_e4332;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_453_0_e4338: f64 = (2.0 * w[77]);
            let noise_metadata_schedule_453_0_e4339: f64 = (w[79] / noise_metadata_schedule_453_0_e4338);
            let noise_metadata_schedule_453_0_e4340: f64 = (1.0 - noise_metadata_schedule_453_0_e4339);
            let noise_metadata_schedule_453_0_e4341: f64 = (w[83] * noise_metadata_schedule_453_0_e4340);
            let noise_metadata_schedule_453_0_e4343: f64 = if noise_metadata_schedule_453_0_e4341 < params[147] { 1.0 } else { 0.0 };
            w[552] = noise_metadata_schedule_453_0_e4343;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_454_0_e4358,) = {
    if ((w[551] != 0.0) && (w[552] != 0.0)) {
        let noise_metadata_schedule_454_0_e4352: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_454_0_e4353: f64 = (w[79] / noise_metadata_schedule_454_0_e4352);
        let noise_metadata_schedule_454_0_e4354: f64 = (1.0 - noise_metadata_schedule_454_0_e4353);
        let noise_metadata_schedule_454_0_e4355: f64 = (w[83] * noise_metadata_schedule_454_0_e4354);
        let noise_metadata_schedule_454_0_e4356: f64 = (noise_metadata_schedule_454_0_e4355).exp();
        (noise_metadata_schedule_454_0_e4356,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_454_0_e4358;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_455_0_e4366,) = {
    if ((w[551] != 0.0) && (w[552] == 0.0)) {
        let noise_metadata_schedule_455_0_e4364: f64 = (params[147]).exp();
        (noise_metadata_schedule_455_0_e4364,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_455_0_e4366;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_456_0_e4387,) = {
    if ((w[551] != 0.0) && (w[552] == 0.0)) {
        let noise_metadata_schedule_456_0_e4378: f64 = (2.0 * w[77]);
        let noise_metadata_schedule_456_0_e4379: f64 = (w[79] / noise_metadata_schedule_456_0_e4378);
        let noise_metadata_schedule_456_0_e4380: f64 = (1.0 - noise_metadata_schedule_456_0_e4379);
        let noise_metadata_schedule_456_0_e4381: f64 = (w[83] * noise_metadata_schedule_456_0_e4380);
        let noise_metadata_schedule_456_0_e4383: f64 = (noise_metadata_schedule_456_0_e4381 - params[147]);
        let noise_metadata_schedule_456_0_e4384: f64 = (1.0 + noise_metadata_schedule_456_0_e4383);
        let noise_metadata_schedule_456_0_e4385: f64 = (w[295] * noise_metadata_schedule_456_0_e4384);
        (noise_metadata_schedule_456_0_e4385,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_456_0_e4387;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_457_0_e4393,) = {
    if (w[551] != 0.0) {
        let noise_metadata_schedule_457_0_e4391: f64 = (w[244] * w[67]);
        (noise_metadata_schedule_457_0_e4391,)
    } else {
        (w[277],)
    }
};
            w[277] = noise_metadata_schedule_457_0_e4393;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_458_0_e4437,) = {
    if (w[551] != 0.0) {
        let noise_metadata_schedule_458_0_e4397: f64 = (w[277] * w[277]);
        let noise_metadata_schedule_458_0_e4399: f64 = (noise_metadata_schedule_458_0_e4397 + 1e-30);
        let noise_metadata_schedule_458_0_e4400: f64 = (noise_metadata_schedule_458_0_e4399).sqrt();
        let noise_metadata_schedule_458_0_e4402: f64 = (-2.0);
        let noise_metadata_schedule_458_0_e4404: f64 = (noise_metadata_schedule_458_0_e4402 - w[76]);
        let noise_metadata_schedule_458_0_e4405: f64 = (noise_metadata_schedule_458_0_e4400).powf(noise_metadata_schedule_458_0_e4404);
        let noise_metadata_schedule_458_0_e4410: f64 = (w[76] * w[76]);
        let noise_metadata_schedule_458_0_e4411: f64 = (1.0 - noise_metadata_schedule_458_0_e4410);
        let noise_metadata_schedule_458_0_e4414: f64 = (3.0 * w[277]);
        let noise_metadata_schedule_458_0_e4417: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_458_0_e4418: f64 = (noise_metadata_schedule_458_0_e4414 * noise_metadata_schedule_458_0_e4417);
        let noise_metadata_schedule_458_0_e4419: f64 = (noise_metadata_schedule_458_0_e4411 - noise_metadata_schedule_458_0_e4418);
        let noise_metadata_schedule_458_0_e4420: f64 = (w[76] * noise_metadata_schedule_458_0_e4419);
        let noise_metadata_schedule_458_0_e4423: f64 = (6.0 * w[277]);
        let noise_metadata_schedule_458_0_e4425: f64 = (noise_metadata_schedule_458_0_e4423 * w[277]);
        let noise_metadata_schedule_458_0_e4428: f64 = (w[76] - 1.0);
        let noise_metadata_schedule_458_0_e4430: f64 = (noise_metadata_schedule_458_0_e4428 + w[277]);
        let noise_metadata_schedule_458_0_e4431: f64 = (noise_metadata_schedule_458_0_e4425 * noise_metadata_schedule_458_0_e4430);
        let noise_metadata_schedule_458_0_e4432: f64 = (noise_metadata_schedule_458_0_e4420 - noise_metadata_schedule_458_0_e4431);
        let noise_metadata_schedule_458_0_e4433: f64 = (noise_metadata_schedule_458_0_e4405 * noise_metadata_schedule_458_0_e4432);
        let noise_metadata_schedule_458_0_e4435: f64 = (noise_metadata_schedule_458_0_e4433 * 0.16666666666666666);
        (noise_metadata_schedule_458_0_e4435,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_458_0_e4437;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_459_0_e4449,) = {
    if (w[551] != 0.0) {
        let noise_metadata_schedule_459_0_e4441: f64 = (w[244] * w[79]);
        let noise_metadata_schedule_459_0_e4443: f64 = (noise_metadata_schedule_459_0_e4441 * w[83]);
        let noise_metadata_schedule_459_0_e4446: f64 = (w[85] * w[80]);
        let noise_metadata_schedule_459_0_e4447: f64 = (noise_metadata_schedule_459_0_e4443 / noise_metadata_schedule_459_0_e4446);
        (noise_metadata_schedule_459_0_e4447,)
    } else {
        (w[277],)
    }
};
            w[277] = noise_metadata_schedule_459_0_e4449;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_460_0_e4452: f64 = (-0.001);
            let noise_metadata_schedule_460_0_e4453: f64 = if w[277] < noise_metadata_schedule_460_0_e4452 { 1.0 } else { 0.0 };
            w[553] = noise_metadata_schedule_460_0_e4453;
        }
        if (active[0] & 0x18002) != 0 {
            let noise_metadata_schedule_461_0_e4456: f64 = if w[277] < params[147] { 1.0 } else { 0.0 };
            w[554] = noise_metadata_schedule_461_0_e4456;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_462_0_e4465,) = {
    if (((w[551] != 0.0) && (w[553] != 0.0)) && (w[554] != 0.0)) {
        let noise_metadata_schedule_462_0_e4463: f64 = (w[277]).exp();
        (noise_metadata_schedule_462_0_e4463,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_462_0_e4465;
        }
        if (active[0] & 0x18002) != 0 {
            let (noise_metadata_schedule_463_0_e4475,) = {
    if (((w[551] != 0.0) && (w[553] != 0.0)) && (w[554] == 0.0)) {
        let noise_metadata_schedule_463_0_e4473: f64 = (params[147]).exp();
        (noise_metadata_schedule_463_0_e4473,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_463_0_e4475;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_464_0_e4490,) = {
    if (((w[551] != 0.0) && (w[553] != 0.0)) && (w[554] == 0.0)) {
        let noise_metadata_schedule_464_0_e4486: f64 = (w[277] - params[147]);
        let noise_metadata_schedule_464_0_e4487: f64 = (1.0 + noise_metadata_schedule_464_0_e4486);
        let noise_metadata_schedule_464_0_e4488: f64 = (w[295] * noise_metadata_schedule_464_0_e4487);
        (noise_metadata_schedule_464_0_e4488,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_464_0_e4490;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_465_0_e4505,) = {
    if ((w[551] != 0.0) && (w[553] != 0.0)) {
        let noise_metadata_schedule_465_0_e4495: f64 = (-w[244]);
        let noise_metadata_schedule_465_0_e4499: f64 = (1.0 - w[92]);
        let noise_metadata_schedule_465_0_e4501: f64 = (noise_metadata_schedule_465_0_e4499 / w[277]);
        let noise_metadata_schedule_465_0_e4502: f64 = (1.0 + noise_metadata_schedule_465_0_e4501);
        let noise_metadata_schedule_465_0_e4503: f64 = (noise_metadata_schedule_465_0_e4495 * noise_metadata_schedule_465_0_e4502);
        (noise_metadata_schedule_465_0_e4503,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_465_0_e4505;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_466_0_e4528,) = {
    if ((w[551] != 0.0) && (w[553] == 0.0)) {
        let noise_metadata_schedule_466_0_e4512: f64 = (w[244] * 0.5);
        let noise_metadata_schedule_466_0_e4514: f64 = (noise_metadata_schedule_466_0_e4512 * w[277]);
        let noise_metadata_schedule_466_0_e4518: f64 = (w[277] * 0.3333333333333333);
        let noise_metadata_schedule_466_0_e4522: f64 = (0.25 * w[277]);
        let noise_metadata_schedule_466_0_e4523: f64 = (1.0 + noise_metadata_schedule_466_0_e4522);
        let noise_metadata_schedule_466_0_e4524: f64 = (noise_metadata_schedule_466_0_e4518 * noise_metadata_schedule_466_0_e4523);
        let noise_metadata_schedule_466_0_e4525: f64 = (1.0 + noise_metadata_schedule_466_0_e4524);
        let noise_metadata_schedule_466_0_e4526: f64 = (noise_metadata_schedule_466_0_e4514 * noise_metadata_schedule_466_0_e4525);
        (noise_metadata_schedule_466_0_e4526,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_466_0_e4528;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_8(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_467_0_e4544,) = {
    if (w[551] != 0.0) {
        let noise_metadata_schedule_467_0_e4532: f64 = (2.0 * w[84]);
        let noise_metadata_schedule_467_0_e4534: f64 = (noise_metadata_schedule_467_0_e4532 * w[81]);
        let noise_metadata_schedule_467_0_e4536: f64 = (noise_metadata_schedule_467_0_e4534 * w[77]);
        let noise_metadata_schedule_467_0_e4538: f64 = (noise_metadata_schedule_467_0_e4536 * w[78]);
        let noise_metadata_schedule_467_0_e4540: f64 = (noise_metadata_schedule_467_0_e4538 * w[67]);
        let noise_metadata_schedule_467_0_e4542: f64 = (noise_metadata_schedule_467_0_e4540 * w[89]);
        (noise_metadata_schedule_467_0_e4542,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_467_0_e4544;
        }
        if (active[0] & 0x18000) != 0 {
            let (noise_metadata_schedule_469_0_e4554,) = {
    if (w[551] == 0.0) {
        (0.0,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_469_0_e4554;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_474_0_e4581: f64 = (2.0 * w[43]);
            let noise_metadata_schedule_474_0_e4584: f64 = (w[268] - 1.0);
            let noise_metadata_schedule_474_0_e4585: f64 = (noise_metadata_schedule_474_0_e4581 * noise_metadata_schedule_474_0_e4584);
            let noise_metadata_schedule_474_0_e4590: f64 = (4.0 * w[43]);
            let noise_metadata_schedule_474_0_e4592: f64 = (noise_metadata_schedule_474_0_e4590 / w[37]);
            let noise_metadata_schedule_474_0_e4594: f64 = (noise_metadata_schedule_474_0_e4592 * w[268]);
            let noise_metadata_schedule_474_0_e4595: f64 = (1.0 + noise_metadata_schedule_474_0_e4594);
            let noise_metadata_schedule_474_0_e4596: f64 = (noise_metadata_schedule_474_0_e4595).sqrt();
            let noise_metadata_schedule_474_0_e4597: f64 = (1.0 + noise_metadata_schedule_474_0_e4596);
            let noise_metadata_schedule_474_0_e4598: f64 = (noise_metadata_schedule_474_0_e4585 / noise_metadata_schedule_474_0_e4597);
            w[164] = noise_metadata_schedule_474_0_e4598;
        }
        if (active[0] & 0x60000) != 0 {
            let noise_metadata_schedule_475_0_e4601: f64 = if params[8] == 1.0 { 1.0 } else { 0.0 };
            w[555] = noise_metadata_schedule_475_0_e4601;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_476_0_e4630,) = {
    if (w[555] != 0.0) {
        let noise_metadata_schedule_476_0_e4605: f64 = (params[143] * 2.0);
        let noise_metadata_schedule_476_0_e4607: f64 = (noise_metadata_schedule_476_0_e4605 * w[104]);
        let noise_metadata_schedule_476_0_e4610: f64 = (w[265] - w[256]);
        let noise_metadata_schedule_476_0_e4611: f64 = (noise_metadata_schedule_476_0_e4607 * noise_metadata_schedule_476_0_e4610);
        let noise_metadata_schedule_476_0_e4617: f64 = (w[104] / w[106]);
        let noise_metadata_schedule_476_0_e4618: f64 = (4.0 * noise_metadata_schedule_476_0_e4617);
        let noise_metadata_schedule_476_0_e4622: f64 = (params[144] * w[256]);
        let noise_metadata_schedule_476_0_e4623: f64 = (w[265] + noise_metadata_schedule_476_0_e4622);
        let noise_metadata_schedule_476_0_e4624: f64 = (noise_metadata_schedule_476_0_e4618 * noise_metadata_schedule_476_0_e4623);
        let noise_metadata_schedule_476_0_e4625: f64 = (1.0 + noise_metadata_schedule_476_0_e4624);
        let noise_metadata_schedule_476_0_e4626: f64 = (noise_metadata_schedule_476_0_e4625).sqrt();
        let noise_metadata_schedule_476_0_e4627: f64 = (1.0 + noise_metadata_schedule_476_0_e4626);
        let noise_metadata_schedule_476_0_e4628: f64 = (noise_metadata_schedule_476_0_e4611 / noise_metadata_schedule_476_0_e4627);
        (noise_metadata_schedule_476_0_e4628,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_476_0_e4630;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_477_0_e4661,) = {
    if (w[555] != 0.0) {
        let noise_metadata_schedule_477_0_e4634: f64 = (1.0 - params[143]);
        let noise_metadata_schedule_477_0_e4636: f64 = (noise_metadata_schedule_477_0_e4634 * 2.0);
        let noise_metadata_schedule_477_0_e4638: f64 = (noise_metadata_schedule_477_0_e4636 * w[104]);
        let noise_metadata_schedule_477_0_e4641: f64 = (w[268] - w[258]);
        let noise_metadata_schedule_477_0_e4642: f64 = (noise_metadata_schedule_477_0_e4638 * noise_metadata_schedule_477_0_e4641);
        let noise_metadata_schedule_477_0_e4648: f64 = (w[104] / w[106]);
        let noise_metadata_schedule_477_0_e4649: f64 = (4.0 * noise_metadata_schedule_477_0_e4648);
        let noise_metadata_schedule_477_0_e4653: f64 = (params[144] * w[258]);
        let noise_metadata_schedule_477_0_e4654: f64 = (w[268] + noise_metadata_schedule_477_0_e4653);
        let noise_metadata_schedule_477_0_e4655: f64 = (noise_metadata_schedule_477_0_e4649 * noise_metadata_schedule_477_0_e4654);
        let noise_metadata_schedule_477_0_e4656: f64 = (1.0 + noise_metadata_schedule_477_0_e4655);
        let noise_metadata_schedule_477_0_e4657: f64 = (noise_metadata_schedule_477_0_e4656).sqrt();
        let noise_metadata_schedule_477_0_e4658: f64 = (1.0 + noise_metadata_schedule_477_0_e4657);
        let noise_metadata_schedule_477_0_e4659: f64 = (noise_metadata_schedule_477_0_e4642 / noise_metadata_schedule_477_0_e4658);
        (noise_metadata_schedule_477_0_e4659,)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_477_0_e4661;
        }
        if (active[0] & 0x20000) != 0 {
            let (noise_metadata_schedule_478_0_e4687,) = {
    if (w[555] == 0.0) {
        let noise_metadata_schedule_478_0_e4666: f64 = (params[143] * 2.0);
        let noise_metadata_schedule_478_0_e4668: f64 = (noise_metadata_schedule_478_0_e4666 * w[104]);
        let noise_metadata_schedule_478_0_e4671: f64 = (w[265] - 1.0);
        let noise_metadata_schedule_478_0_e4672: f64 = (noise_metadata_schedule_478_0_e4668 * noise_metadata_schedule_478_0_e4671);
        let noise_metadata_schedule_478_0_e4678: f64 = (w[104] / w[106]);
        let noise_metadata_schedule_478_0_e4679: f64 = (4.0 * noise_metadata_schedule_478_0_e4678);
        let noise_metadata_schedule_478_0_e4681: f64 = (noise_metadata_schedule_478_0_e4679 * w[265]);
        let noise_metadata_schedule_478_0_e4682: f64 = (1.0 + noise_metadata_schedule_478_0_e4681);
        let noise_metadata_schedule_478_0_e4683: f64 = (noise_metadata_schedule_478_0_e4682).sqrt();
        let noise_metadata_schedule_478_0_e4684: f64 = (1.0 + noise_metadata_schedule_478_0_e4683);
        let noise_metadata_schedule_478_0_e4685: f64 = (noise_metadata_schedule_478_0_e4672 / noise_metadata_schedule_478_0_e4684);
        (noise_metadata_schedule_478_0_e4685,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_478_0_e4687;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_479_0_e4715,) = {
    if (w[555] == 0.0) {
        let noise_metadata_schedule_479_0_e4692: f64 = (1.0 - params[143]);
        let noise_metadata_schedule_479_0_e4694: f64 = (noise_metadata_schedule_479_0_e4692 * 2.0);
        let noise_metadata_schedule_479_0_e4696: f64 = (noise_metadata_schedule_479_0_e4694 * w[104]);
        let noise_metadata_schedule_479_0_e4699: f64 = (w[268] - 1.0);
        let noise_metadata_schedule_479_0_e4700: f64 = (noise_metadata_schedule_479_0_e4696 * noise_metadata_schedule_479_0_e4699);
        let noise_metadata_schedule_479_0_e4706: f64 = (w[104] / w[106]);
        let noise_metadata_schedule_479_0_e4707: f64 = (4.0 * noise_metadata_schedule_479_0_e4706);
        let noise_metadata_schedule_479_0_e4709: f64 = (noise_metadata_schedule_479_0_e4707 * w[268]);
        let noise_metadata_schedule_479_0_e4710: f64 = (1.0 + noise_metadata_schedule_479_0_e4709);
        let noise_metadata_schedule_479_0_e4711: f64 = (noise_metadata_schedule_479_0_e4710).sqrt();
        let noise_metadata_schedule_479_0_e4712: f64 = (1.0 + noise_metadata_schedule_479_0_e4711);
        let noise_metadata_schedule_479_0_e4713: f64 = (noise_metadata_schedule_479_0_e4700 / noise_metadata_schedule_479_0_e4712);
        (noise_metadata_schedule_479_0_e4713,)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_479_0_e4715;
        }
        if (active[0] & 0x80000) != 0 {
            w[180] = 0.0;
        }
        if (active[0] & 0xdfe00) != 0 {
            let noise_metadata_schedule_482_0_e4749: f64 = if ((params[5] > 0.0) && (params[33] > 0.0)) { 1.0 } else { 0.0 };
            w[556] = noise_metadata_schedule_482_0_e4749;
        }
        if (active[0] & 0x1800) != 0 {
            let (noise_metadata_schedule_483_0_e4755,) = {
    if (w[556] != 0.0) {
        let noise_metadata_schedule_483_0_e4753: f64 = (w[164] * w[157]);
        (noise_metadata_schedule_483_0_e4753,)
    } else {
        (w[164],)
    }
};
            w[164] = noise_metadata_schedule_483_0_e4755;
        }
        if (active[0] & 0x40000) != 0 {
            let (noise_metadata_schedule_484_0_e4761,) = {
    if (w[556] != 0.0) {
        let noise_metadata_schedule_484_0_e4759: f64 = (w[179] * w[157]);
        (noise_metadata_schedule_484_0_e4759,)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_484_0_e4761;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_485_0_e4786,) = {
    if (w[556] != 0.0) {
        let noise_metadata_schedule_485_0_e4765: f64 = (params[33] * 2.0);
        let noise_metadata_schedule_485_0_e4767: f64 = (noise_metadata_schedule_485_0_e4765 * w[43]);
        let noise_metadata_schedule_485_0_e4770: f64 = (w[269] - 1.0);
        let noise_metadata_schedule_485_0_e4771: f64 = (noise_metadata_schedule_485_0_e4767 * noise_metadata_schedule_485_0_e4770);
        let noise_metadata_schedule_485_0_e4776: f64 = (4.0 * w[43]);
        let noise_metadata_schedule_485_0_e4778: f64 = (noise_metadata_schedule_485_0_e4776 / w[37]);
        let noise_metadata_schedule_485_0_e4780: f64 = (noise_metadata_schedule_485_0_e4778 * w[269]);
        let noise_metadata_schedule_485_0_e4781: f64 = (1.0 + noise_metadata_schedule_485_0_e4780);
        let noise_metadata_schedule_485_0_e4782: f64 = (noise_metadata_schedule_485_0_e4781).sqrt();
        let noise_metadata_schedule_485_0_e4783: f64 = (1.0 + noise_metadata_schedule_485_0_e4782);
        let noise_metadata_schedule_485_0_e4784: f64 = (noise_metadata_schedule_485_0_e4771 / noise_metadata_schedule_485_0_e4783);
        (noise_metadata_schedule_485_0_e4784,)
    } else {
        (w[171],)
    }
};
            w[171] = noise_metadata_schedule_485_0_e4786;
        }
        if (active[0] & 0x86000) != 0 {
            let noise_metadata_schedule_486_0_e4789: f64 = if params[8] == 1.0 { 1.0 } else { 0.0 };
            w[557] = noise_metadata_schedule_486_0_e4789;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_487_0_e4824,) = {
    if ((w[556] != 0.0) && (w[557] != 0.0)) {
        let noise_metadata_schedule_487_0_e4795: f64 = (1.0 - params[143]);
        let noise_metadata_schedule_487_0_e4797: f64 = (noise_metadata_schedule_487_0_e4795 * params[33]);
        let noise_metadata_schedule_487_0_e4799: f64 = (noise_metadata_schedule_487_0_e4797 * 2.0);
        let noise_metadata_schedule_487_0_e4801: f64 = (noise_metadata_schedule_487_0_e4799 * w[104]);
        let noise_metadata_schedule_487_0_e4804: f64 = (w[269] - w[257]);
        let noise_metadata_schedule_487_0_e4805: f64 = (noise_metadata_schedule_487_0_e4801 * noise_metadata_schedule_487_0_e4804);
        let noise_metadata_schedule_487_0_e4810: f64 = (4.0 * w[104]);
        let noise_metadata_schedule_487_0_e4812: f64 = (noise_metadata_schedule_487_0_e4810 / w[106]);
        let noise_metadata_schedule_487_0_e4816: f64 = (params[144] * w[257]);
        let noise_metadata_schedule_487_0_e4817: f64 = (w[269] + noise_metadata_schedule_487_0_e4816);
        let noise_metadata_schedule_487_0_e4818: f64 = (noise_metadata_schedule_487_0_e4812 * noise_metadata_schedule_487_0_e4817);
        let noise_metadata_schedule_487_0_e4819: f64 = (1.0 + noise_metadata_schedule_487_0_e4818);
        let noise_metadata_schedule_487_0_e4820: f64 = (noise_metadata_schedule_487_0_e4819).sqrt();
        let noise_metadata_schedule_487_0_e4821: f64 = (1.0 + noise_metadata_schedule_487_0_e4820);
        let noise_metadata_schedule_487_0_e4822: f64 = (noise_metadata_schedule_487_0_e4805 / noise_metadata_schedule_487_0_e4821);
        (noise_metadata_schedule_487_0_e4822,)
    } else {
        (w[172],)
    }
};
            w[172] = noise_metadata_schedule_487_0_e4824;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_488_0_e4856,) = {
    if ((w[556] != 0.0) && (w[557] == 0.0)) {
        let noise_metadata_schedule_488_0_e4831: f64 = (1.0 - params[143]);
        let noise_metadata_schedule_488_0_e4833: f64 = (noise_metadata_schedule_488_0_e4831 * params[33]);
        let noise_metadata_schedule_488_0_e4835: f64 = (noise_metadata_schedule_488_0_e4833 * 2.0);
        let noise_metadata_schedule_488_0_e4837: f64 = (noise_metadata_schedule_488_0_e4835 * w[104]);
        let noise_metadata_schedule_488_0_e4840: f64 = (w[269] - 1.0);
        let noise_metadata_schedule_488_0_e4841: f64 = (noise_metadata_schedule_488_0_e4837 * noise_metadata_schedule_488_0_e4840);
        let noise_metadata_schedule_488_0_e4846: f64 = (4.0 * w[104]);
        let noise_metadata_schedule_488_0_e4848: f64 = (noise_metadata_schedule_488_0_e4846 / w[106]);
        let noise_metadata_schedule_488_0_e4850: f64 = (noise_metadata_schedule_488_0_e4848 * w[269]);
        let noise_metadata_schedule_488_0_e4851: f64 = (1.0 + noise_metadata_schedule_488_0_e4850);
        let noise_metadata_schedule_488_0_e4852: f64 = (noise_metadata_schedule_488_0_e4851).sqrt();
        let noise_metadata_schedule_488_0_e4853: f64 = (1.0 + noise_metadata_schedule_488_0_e4852);
        let noise_metadata_schedule_488_0_e4854: f64 = (noise_metadata_schedule_488_0_e4841 / noise_metadata_schedule_488_0_e4853);
        (noise_metadata_schedule_488_0_e4854,)
    } else {
        (w[172],)
    }
};
            w[172] = noise_metadata_schedule_488_0_e4856;
        }
        if (active[0] & 0x9fe00) != 0 {
            let noise_metadata_schedule_489_0_e4859: f64 = if params[5] == 1.0 { 1.0 } else { 0.0 };
            w[558] = noise_metadata_schedule_489_0_e4859;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_490_0_e4871,) = {
    if ((w[556] != 0.0) && (w[558] != 0.0)) {
        let noise_metadata_schedule_490_0_e4866: f64 = (w[43] + w[104]);
        let noise_metadata_schedule_490_0_e4867: f64 = (params[33] * noise_metadata_schedule_490_0_e4866);
        let noise_metadata_schedule_490_0_e4869: f64 = (noise_metadata_schedule_490_0_e4867 * w[32]);
        (noise_metadata_schedule_490_0_e4869,)
    } else {
        (w[291],)
    }
};
            w[291] = noise_metadata_schedule_490_0_e4871;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_491_0_e4884,) = {
    if ((w[556] != 0.0) && (w[558] != 0.0)) {
        let noise_metadata_schedule_491_0_e4879: f64 = (w[291] * w[8]);
        let noise_metadata_schedule_491_0_e4880: f64 = (noise_metadata_schedule_491_0_e4879).ln();
        let noise_metadata_schedule_491_0_e4881: f64 = (2.0 - noise_metadata_schedule_491_0_e4880);
        let noise_metadata_schedule_491_0_e4882: f64 = (w[6] * noise_metadata_schedule_491_0_e4881);
        (noise_metadata_schedule_491_0_e4882,)
    } else {
        (w[173],)
    }
};
            w[173] = noise_metadata_schedule_491_0_e4884;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_492_0_e4892,) = {
    if ((w[556] != 0.0) && (w[558] != 0.0)) {
        let noise_metadata_schedule_492_0_e4890: f64 = (w[261] - w[173]);
        (noise_metadata_schedule_492_0_e4890,)
    } else {
        (w[284],)
    }
};
            w[284] = noise_metadata_schedule_492_0_e4892;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_493_0_e4900,) = {
    if ((w[556] != 0.0) && (w[558] != 0.0)) {
        let noise_metadata_schedule_493_0_e4898: f64 = (0.11 * 0.11);
        (noise_metadata_schedule_493_0_e4898,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_493_0_e4900;
        }
        if (active[0] & 0x9fe00) != 0 {
            let (noise_metadata_schedule_494_0_e4908,) = {
    if ((w[556] != 0.0) && (w[558] != 0.0)) {
        let noise_metadata_schedule_494_0_e4906: f64 = (w[284] * w[284]);
        (noise_metadata_schedule_494_0_e4906,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_494_0_e4908;
        }
        if (active[0] & 0x86000) != 0 {
            let noise_metadata_schedule_495_0_e4911: f64 = if w[284] < 0.0 { 1.0 } else { 0.0 };
            w[559] = noise_metadata_schedule_495_0_e4911;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_496_0_e4928,) = {
    if (((w[556] != 0.0) && (w[558] != 0.0)) && (w[559] != 0.0)) {
        let noise_metadata_schedule_496_0_e4919: f64 = (0.5 * w[281]);
        let noise_metadata_schedule_496_0_e4922: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_496_0_e4923: f64 = (noise_metadata_schedule_496_0_e4922).sqrt();
        let noise_metadata_schedule_496_0_e4925: f64 = (noise_metadata_schedule_496_0_e4923 - w[284]);
        let noise_metadata_schedule_496_0_e4926: f64 = (noise_metadata_schedule_496_0_e4919 / noise_metadata_schedule_496_0_e4925);
        (noise_metadata_schedule_496_0_e4926,)
    } else {
        (w[174],)
    }
};
            w[174] = noise_metadata_schedule_496_0_e4928;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_497_0_e4944,) = {
    if (((w[556] != 0.0) && (w[558] != 0.0)) && (w[559] == 0.0)) {
        let noise_metadata_schedule_497_0_e4938: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_497_0_e4939: f64 = (noise_metadata_schedule_497_0_e4938).sqrt();
        let noise_metadata_schedule_497_0_e4941: f64 = (noise_metadata_schedule_497_0_e4939 + w[284]);
        let noise_metadata_schedule_497_0_e4942: f64 = (0.5 * noise_metadata_schedule_497_0_e4941);
        (noise_metadata_schedule_497_0_e4942,)
    } else {
        (w[174],)
    }
};
            w[174] = noise_metadata_schedule_497_0_e4944;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_498_0_e4960,) = {
    if ((w[556] != 0.0) && (w[558] != 0.0)) {
        let noise_metadata_schedule_498_0_e4952: f64 = (w[171] + w[172]);
        let noise_metadata_schedule_498_0_e4954: f64 = (noise_metadata_schedule_498_0_e4952 * w[32]);
        let noise_metadata_schedule_498_0_e4955: f64 = (w[291] + noise_metadata_schedule_498_0_e4954);
        let noise_metadata_schedule_498_0_e4957: f64 = (noise_metadata_schedule_498_0_e4955 + w[174]);
        let noise_metadata_schedule_498_0_e4958: f64 = (w[174] / noise_metadata_schedule_498_0_e4957);
        (noise_metadata_schedule_498_0_e4958,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_498_0_e4960;
        }
        if (active[0] & 0x86000) != 0 {
            let (noise_metadata_schedule_502_0_e4988,) = {
    if ((w[556] != 0.0) && (w[558] == 0.0)) {
        (1.0,)
    } else {
        (w[175],)
    }
};
            w[175] = noise_metadata_schedule_502_0_e4988;
        }
        if (active[0] & 0x6000) != 0 {
            let (noise_metadata_schedule_503_0_e4994,) = {
    if (w[556] != 0.0) {
        let noise_metadata_schedule_503_0_e4992: f64 = (w[175] * w[171]);
        (noise_metadata_schedule_503_0_e4992,)
    } else {
        (w[176],)
    }
};
            w[176] = noise_metadata_schedule_503_0_e4994;
        }
        if (active[0] & 0x80000) != 0 {
            let (noise_metadata_schedule_504_0_e5000,) = {
    if (w[556] != 0.0) {
        let noise_metadata_schedule_504_0_e4998: f64 = (w[175] * w[172]);
        (noise_metadata_schedule_504_0_e4998,)
    } else {
        (w[180],)
    }
};
            w[180] = noise_metadata_schedule_504_0_e5000;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_505_0_e5003: f64 = if params[84] == 1.0 { 1.0 } else { 0.0 };
            w[560] = noise_metadata_schedule_505_0_e5003;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_506_0_e5009,) = {
    if (w[560] != 0.0) {
        let noise_metadata_schedule_506_0_e5007: f64 = (w[248] + w[244]);
        (noise_metadata_schedule_506_0_e5007,)
    } else {
        (w[347],)
    }
};
            w[347] = noise_metadata_schedule_506_0_e5009;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_507_0_e5015,) = {
    if (w[560] != 0.0) {
        let noise_metadata_schedule_507_0_e5013: f64 = (1e-6 * 1e-6);
        (noise_metadata_schedule_507_0_e5013,)
    } else {
        (w[281],)
    }
};
            w[281] = noise_metadata_schedule_507_0_e5015;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_508_0_e5027,) = {
    if (w[560] != 0.0) {
        let noise_metadata_schedule_508_0_e5018: f64 = (-1.0);
        let noise_metadata_schedule_508_0_e5020: f64 = (noise_metadata_schedule_508_0_e5018 * w[347]);
        let noise_metadata_schedule_508_0_e5022: f64 = (-1.0);
        let noise_metadata_schedule_508_0_e5023: f64 = (noise_metadata_schedule_508_0_e5020 * noise_metadata_schedule_508_0_e5022);
        let noise_metadata_schedule_508_0_e5025: f64 = (noise_metadata_schedule_508_0_e5023 * w[347]);
        (noise_metadata_schedule_508_0_e5025,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_508_0_e5027;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_509_0_e5029: f64 = (-1.0);
            let noise_metadata_schedule_509_0_e5031: f64 = (noise_metadata_schedule_509_0_e5029 * w[347]);
            let noise_metadata_schedule_509_0_e5033: f64 = if noise_metadata_schedule_509_0_e5031 < 0.0 { 1.0 } else { 0.0 };
            w[561] = noise_metadata_schedule_509_0_e5033;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_510_0_e5051,) = {
    if ((w[560] != 0.0) && (w[561] != 0.0)) {
        let noise_metadata_schedule_510_0_e5039: f64 = (0.5 * w[281]);
        let noise_metadata_schedule_510_0_e5042: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_510_0_e5043: f64 = (noise_metadata_schedule_510_0_e5042).sqrt();
        let noise_metadata_schedule_510_0_e5045: f64 = (-1.0);
        let noise_metadata_schedule_510_0_e5047: f64 = (noise_metadata_schedule_510_0_e5045 * w[347]);
        let noise_metadata_schedule_510_0_e5048: f64 = (noise_metadata_schedule_510_0_e5043 - noise_metadata_schedule_510_0_e5047);
        let noise_metadata_schedule_510_0_e5049: f64 = (noise_metadata_schedule_510_0_e5039 / noise_metadata_schedule_510_0_e5048);
        (noise_metadata_schedule_510_0_e5049,)
    } else {
        (w[348],)
    }
};
            w[348] = noise_metadata_schedule_510_0_e5051;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_511_0_e5068,) = {
    if ((w[560] != 0.0) && (w[561] == 0.0)) {
        let noise_metadata_schedule_511_0_e5059: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_511_0_e5060: f64 = (noise_metadata_schedule_511_0_e5059).sqrt();
        let noise_metadata_schedule_511_0_e5062: f64 = (-1.0);
        let noise_metadata_schedule_511_0_e5064: f64 = (noise_metadata_schedule_511_0_e5062 * w[347]);
        let noise_metadata_schedule_511_0_e5065: f64 = (noise_metadata_schedule_511_0_e5060 + noise_metadata_schedule_511_0_e5064);
        let noise_metadata_schedule_511_0_e5066: f64 = (0.5 * noise_metadata_schedule_511_0_e5065);
        (noise_metadata_schedule_511_0_e5066,)
    } else {
        (w[348],)
    }
};
            w[348] = noise_metadata_schedule_511_0_e5068;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_512_0_e5078,) = {
    if (w[560] != 0.0) {
        let noise_metadata_schedule_512_0_e5074: f64 = (w[343]).powf(params[82]);
        let noise_metadata_schedule_512_0_e5075: f64 = (1.0 - noise_metadata_schedule_512_0_e5074);
        let noise_metadata_schedule_512_0_e5076: f64 = (1.0 / noise_metadata_schedule_512_0_e5075);
        (noise_metadata_schedule_512_0_e5076,)
    } else {
        (w[349],)
    }
};
            w[349] = noise_metadata_schedule_512_0_e5078;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_513_0_e5084,) = {
    if (w[560] != 0.0) {
        let noise_metadata_schedule_513_0_e5082: f64 = (w[343] * params[81]);
        (noise_metadata_schedule_513_0_e5082,)
    } else {
        (w[344],)
    }
};
            w[344] = noise_metadata_schedule_513_0_e5084;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_514_0_e5100,) = {
    if (w[560] != 0.0) {
        let noise_metadata_schedule_514_0_e5088: f64 = (w[349] * w[349]);
        let noise_metadata_schedule_514_0_e5092: f64 = (params[82] - 1.0);
        let noise_metadata_schedule_514_0_e5093: f64 = (w[343]).powf(noise_metadata_schedule_514_0_e5092);
        let noise_metadata_schedule_514_0_e5094: f64 = (noise_metadata_schedule_514_0_e5088 * noise_metadata_schedule_514_0_e5093);
        let noise_metadata_schedule_514_0_e5096: f64 = (noise_metadata_schedule_514_0_e5094 * params[82]);
        let noise_metadata_schedule_514_0_e5098: f64 = (noise_metadata_schedule_514_0_e5096 / params[81]);
        (noise_metadata_schedule_514_0_e5098,)
    } else {
        (w[346],)
    }
};
            w[346] = noise_metadata_schedule_514_0_e5100;
        }
        if (active[0] & 0x1fe00) != 0 {
            let noise_metadata_schedule_515_0_e5103: f64 = if w[348] < w[344] { 1.0 } else { 0.0 };
            w[562] = noise_metadata_schedule_515_0_e5103;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_9(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_516_0_e5117,) = {
    if ((w[560] != 0.0) && (w[562] != 0.0)) {
        let noise_metadata_schedule_516_0_e5111: f64 = (w[348] / params[81]);
        let noise_metadata_schedule_516_0_e5113: f64 = (noise_metadata_schedule_516_0_e5111).powf(params[82]);
        let noise_metadata_schedule_516_0_e5114: f64 = (1.0 - noise_metadata_schedule_516_0_e5113);
        let noise_metadata_schedule_516_0_e5115: f64 = (1.0 / noise_metadata_schedule_516_0_e5114);
        (noise_metadata_schedule_516_0_e5115,)
    } else {
        (w[345],)
    }
};
            w[345] = noise_metadata_schedule_516_0_e5117;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_517_0_e5130,) = {
    if ((w[560] != 0.0) && (w[562] == 0.0)) {
        let noise_metadata_schedule_517_0_e5125: f64 = (w[348] - w[344]);
        let noise_metadata_schedule_517_0_e5127: f64 = (noise_metadata_schedule_517_0_e5125 * w[346]);
        let noise_metadata_schedule_517_0_e5128: f64 = (w[349] + noise_metadata_schedule_517_0_e5127);
        (noise_metadata_schedule_517_0_e5128,)
    } else {
        (w[345],)
    }
};
            w[345] = noise_metadata_schedule_517_0_e5130;
        }
        if (active[0] & 0x1fe00) != 0 {
            let (noise_metadata_schedule_518_0_e5135,) = {
    if (w[560] == 0.0) {
        (1.0,)
    } else {
        (w[345],)
    }
};
            w[345] = noise_metadata_schedule_518_0_e5135;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_519_0_e5138: f64 = (w[82] * w[345]);
            w[82] = noise_metadata_schedule_519_0_e5138;
        }
        if (active[0] & 0x1800) != 0 {
            let noise_metadata_schedule_520_0_e5141: f64 = (w[164] * w[345]);
            w[164] = noise_metadata_schedule_520_0_e5141;
        }
        if (active[0] & 0x600) != 0 {
            let noise_metadata_schedule_521_0_e5144: f64 = (w[161] * w[345]);
            w[161] = noise_metadata_schedule_521_0_e5144;
        }
        if (active[0] & 0x6000) != 0 {
            let noise_metadata_schedule_522_0_e5147: f64 = (w[176] * w[345]);
            w[176] = noise_metadata_schedule_522_0_e5147;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_523_0_e5151: f64 = (w[138] / w[41]);
            let noise_metadata_schedule_523_0_e5152: f64 = (1.0 + noise_metadata_schedule_523_0_e5151);
            let noise_metadata_schedule_523_0_e5155: f64 = (w[145] / w[40]);
            let noise_metadata_schedule_523_0_e5156: f64 = (noise_metadata_schedule_523_0_e5152 + noise_metadata_schedule_523_0_e5155);
            w[183] = noise_metadata_schedule_523_0_e5156;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_524_0_e5159: f64 = (0.1 * 0.1);
            w[281] = noise_metadata_schedule_524_0_e5159;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_525_0_e5162: f64 = (w[183] * w[183]);
            w[282] = noise_metadata_schedule_525_0_e5162;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_526_0_e5165: f64 = if w[183] < 0.0 { 1.0 } else { 0.0 };
            w[563] = noise_metadata_schedule_526_0_e5165;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_527_0_e5178,) = {
    if (w[563] != 0.0) {
        let noise_metadata_schedule_527_0_e5169: f64 = (0.5 * w[281]);
        let noise_metadata_schedule_527_0_e5172: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_527_0_e5173: f64 = (noise_metadata_schedule_527_0_e5172).sqrt();
        let noise_metadata_schedule_527_0_e5175: f64 = (noise_metadata_schedule_527_0_e5173 - w[183]);
        let noise_metadata_schedule_527_0_e5176: f64 = (noise_metadata_schedule_527_0_e5169 / noise_metadata_schedule_527_0_e5175);
        (noise_metadata_schedule_527_0_e5176,)
    } else {
        (w[184],)
    }
};
            w[184] = noise_metadata_schedule_527_0_e5178;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_528_0_e5190,) = {
    if (w[563] == 0.0) {
        let noise_metadata_schedule_528_0_e5184: f64 = (w[282] + w[281]);
        let noise_metadata_schedule_528_0_e5185: f64 = (noise_metadata_schedule_528_0_e5184).sqrt();
        let noise_metadata_schedule_528_0_e5187: f64 = (noise_metadata_schedule_528_0_e5185 + w[183]);
        let noise_metadata_schedule_528_0_e5188: f64 = (0.5 * noise_metadata_schedule_528_0_e5187);
        (noise_metadata_schedule_528_0_e5188,)
    } else {
        (w[184],)
    }
};
            w[184] = noise_metadata_schedule_528_0_e5190;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_529_0_e5196: f64 = (w[149] + w[150]);
            let noise_metadata_schedule_529_0_e5197: f64 = (0.5 * noise_metadata_schedule_529_0_e5196);
            let noise_metadata_schedule_529_0_e5198: f64 = (1.0 + noise_metadata_schedule_529_0_e5197);
            let noise_metadata_schedule_529_0_e5199: f64 = (w[184] * noise_metadata_schedule_529_0_e5198);
            w[185] = noise_metadata_schedule_529_0_e5199;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_530_0_e5202: f64 = (w[29] / w[185]);
            w[187] = noise_metadata_schedule_530_0_e5202;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_531_0_e5205: f64 = if w[187] < w[340] { 1.0 } else { 0.0 };
            w[564] = noise_metadata_schedule_531_0_e5205;
        }
        if (active[0] & 0x22) != 0 {
            let (noise_metadata_schedule_532_0_e5209,) = {
    if (w[564] != 0.0) {
        (w[340],)
    } else {
        (w[187],)
    }
};
            w[187] = noise_metadata_schedule_532_0_e5209;
        }
        if (active[0] & 0x22) != 0 {
            let noise_metadata_schedule_533_0_e5212: f64 = (3.0 * w[187]);
            w[186] = noise_metadata_schedule_533_0_e5212;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_535_0_e5226: f64 = if w[156] > 0.0 { 1.0 } else { 0.0 };
            w[565] = noise_metadata_schedule_535_0_e5226;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_536_0_e5229: f64 = if params[39] == 1.0 { 1.0 } else { 0.0 };
            w[566] = noise_metadata_schedule_536_0_e5229;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_537_0_e5232: f64 = if w[244] < params[44] { 1.0 } else { 0.0 };
            w[567] = noise_metadata_schedule_537_0_e5232;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_538_0_e5234: f64 = (-w[156]);
            let noise_metadata_schedule_538_0_e5236: f64 = (noise_metadata_schedule_538_0_e5234 / params[42]);
            let noise_metadata_schedule_538_0_e5238: f64 = if noise_metadata_schedule_538_0_e5236 < params[147] { 1.0 } else { 0.0 };
            w[568] = noise_metadata_schedule_538_0_e5238;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_539_0_e5252,) = {
    if ((((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) && (w[568] != 0.0)) {
        let noise_metadata_schedule_539_0_e5247: f64 = (-w[156]);
        let noise_metadata_schedule_539_0_e5249: f64 = (noise_metadata_schedule_539_0_e5247 / params[42]);
        let noise_metadata_schedule_539_0_e5250: f64 = (noise_metadata_schedule_539_0_e5249).exp();
        (noise_metadata_schedule_539_0_e5250,)
    } else {
        (w[332],)
    }
};
            w[332] = noise_metadata_schedule_539_0_e5252;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_540_0_e5264,) = {
    if ((((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) && (w[568] == 0.0)) {
        let noise_metadata_schedule_540_0_e5262: f64 = (params[147]).exp();
        (noise_metadata_schedule_540_0_e5262,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_540_0_e5264;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_541_0_e5284,) = {
    if ((((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) && (w[568] == 0.0)) {
        let noise_metadata_schedule_541_0_e5276: f64 = (-w[156]);
        let noise_metadata_schedule_541_0_e5278: f64 = (noise_metadata_schedule_541_0_e5276 / params[42]);
        let noise_metadata_schedule_541_0_e5280: f64 = (noise_metadata_schedule_541_0_e5278 - params[147]);
        let noise_metadata_schedule_541_0_e5281: f64 = (1.0 + noise_metadata_schedule_541_0_e5280);
        let noise_metadata_schedule_541_0_e5282: f64 = (w[295] * noise_metadata_schedule_541_0_e5281);
        (noise_metadata_schedule_541_0_e5282,)
    } else {
        (w[332],)
    }
};
            w[332] = noise_metadata_schedule_541_0_e5284;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_542_0_e5296,) = {
    if (((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) {
        let noise_metadata_schedule_542_0_e5292: f64 = (params[44] - w[244]);
        let noise_metadata_schedule_542_0_e5294: f64 = (noise_metadata_schedule_542_0_e5292 * w[332]);
        (noise_metadata_schedule_542_0_e5294,)
    } else {
        (w[333],)
    }
};
            w[333] = noise_metadata_schedule_542_0_e5296;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_543_0_e5298: f64 = (-w[334]);
            let noise_metadata_schedule_543_0_e5301: f64 = (w[333]).powf(params[41]);
            let noise_metadata_schedule_543_0_e5302: f64 = (noise_metadata_schedule_543_0_e5298 * noise_metadata_schedule_543_0_e5301);
            let noise_metadata_schedule_543_0_e5304: f64 = if noise_metadata_schedule_543_0_e5302 < params[147] { 1.0 } else { 0.0 };
            w[569] = noise_metadata_schedule_543_0_e5304;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_544_0_e5320,) = {
    if ((((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) && (w[569] != 0.0)) {
        let noise_metadata_schedule_544_0_e5313: f64 = (-w[334]);
        let noise_metadata_schedule_544_0_e5316: f64 = (w[333]).powf(params[41]);
        let noise_metadata_schedule_544_0_e5317: f64 = (noise_metadata_schedule_544_0_e5313 * noise_metadata_schedule_544_0_e5316);
        let noise_metadata_schedule_544_0_e5318: f64 = (noise_metadata_schedule_544_0_e5317).exp();
        (noise_metadata_schedule_544_0_e5318,)
    } else {
        (w[337],)
    }
};
            w[337] = noise_metadata_schedule_544_0_e5320;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_545_0_e5332,) = {
    if ((((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) && (w[569] == 0.0)) {
        let noise_metadata_schedule_545_0_e5330: f64 = (params[147]).exp();
        (noise_metadata_schedule_545_0_e5330,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_545_0_e5332;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_546_0_e5354,) = {
    if ((((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) && (w[569] == 0.0)) {
        let noise_metadata_schedule_546_0_e5344: f64 = (-w[334]);
        let noise_metadata_schedule_546_0_e5347: f64 = (w[333]).powf(params[41]);
        let noise_metadata_schedule_546_0_e5348: f64 = (noise_metadata_schedule_546_0_e5344 * noise_metadata_schedule_546_0_e5347);
        let noise_metadata_schedule_546_0_e5350: f64 = (noise_metadata_schedule_546_0_e5348 - params[147]);
        let noise_metadata_schedule_546_0_e5351: f64 = (1.0 + noise_metadata_schedule_546_0_e5350);
        let noise_metadata_schedule_546_0_e5352: f64 = (w[295] * noise_metadata_schedule_546_0_e5351);
        (noise_metadata_schedule_546_0_e5352,)
    } else {
        (w[337],)
    }
};
            w[337] = noise_metadata_schedule_546_0_e5354;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_547_0_e5368,) = {
    if (((w[565] != 0.0) && (w[566] != 0.0)) && (w[567] != 0.0)) {
        let noise_metadata_schedule_547_0_e5362: f64 = (params[40] / w[334]);
        let noise_metadata_schedule_547_0_e5364: f64 = (noise_metadata_schedule_547_0_e5362 * w[333]);
        let noise_metadata_schedule_547_0_e5366: f64 = (noise_metadata_schedule_547_0_e5364 * w[337]);
        (noise_metadata_schedule_547_0_e5366,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_547_0_e5368;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_548_0_e5371: f64 = if params[39] == 2.0 { 1.0 } else { 0.0 };
            w[570] = noise_metadata_schedule_548_0_e5371;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_549_0_e5374: f64 = if w[244] < w[16] { 1.0 } else { 0.0 };
            w[571] = noise_metadata_schedule_549_0_e5374;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_550_0_e5391,) = {
    if ((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) {
        let noise_metadata_schedule_550_0_e5385: f64 = (2.0 * params[46]);
        let noise_metadata_schedule_550_0_e5388: f64 = (params[45] * params[45]);
        let noise_metadata_schedule_550_0_e5389: f64 = (noise_metadata_schedule_550_0_e5385 / noise_metadata_schedule_550_0_e5388);
        (noise_metadata_schedule_550_0_e5389,)
    } else {
        (w[196],)
    }
};
            w[196] = noise_metadata_schedule_550_0_e5391;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_551_0_e5406,) = {
    if ((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) {
        let noise_metadata_schedule_551_0_e5402: f64 = (w[16] - w[244]);
        let noise_metadata_schedule_551_0_e5404: f64 = (noise_metadata_schedule_551_0_e5402 / w[210]);
        (noise_metadata_schedule_551_0_e5404,)
    } else {
        (w[280],)
    }
};
            w[280] = noise_metadata_schedule_551_0_e5406;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_552_0_e5422,) = {
    if ((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) {
        let noise_metadata_schedule_552_0_e5417: f64 = (2.0 * w[280]);
        let noise_metadata_schedule_552_0_e5419: f64 = (noise_metadata_schedule_552_0_e5417 / w[196]);
        let noise_metadata_schedule_552_0_e5420: f64 = (noise_metadata_schedule_552_0_e5419).sqrt();
        (noise_metadata_schedule_552_0_e5420,)
    } else {
        (w[197],)
    }
};
            w[197] = noise_metadata_schedule_552_0_e5422;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_553_0_e5425: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[572] = noise_metadata_schedule_553_0_e5425;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_554_0_e5438,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[572] != 0.0)) {
        (params[45],)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_554_0_e5438;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_555_0_e5456,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[572] == 0.0)) {
        let noise_metadata_schedule_555_0_e5453: f64 = (0.5 * w[122]);
        let noise_metadata_schedule_555_0_e5454: f64 = (1.0 - noise_metadata_schedule_555_0_e5453);
        (noise_metadata_schedule_555_0_e5454,)
    } else {
        (w[123],)
    }
};
            w[123] = noise_metadata_schedule_555_0_e5456;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_556_0_e5474,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[572] == 0.0)) {
        let noise_metadata_schedule_556_0_e5470: f64 = (params[45] * w[123]);
        let noise_metadata_schedule_556_0_e5472: f64 = (noise_metadata_schedule_556_0_e5470 * w[123]);
        (noise_metadata_schedule_556_0_e5472,)
    } else {
        (w[198],)
    }
};
            w[198] = noise_metadata_schedule_556_0_e5474;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_557_0_e5496,) = {
    if ((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) {
        let noise_metadata_schedule_557_0_e5485: f64 = (w[197] * w[198]);
        let noise_metadata_schedule_557_0_e5488: f64 = (w[197] * w[197]);
        let noise_metadata_schedule_557_0_e5491: f64 = (w[198] * w[198]);
        let noise_metadata_schedule_557_0_e5492: f64 = (noise_metadata_schedule_557_0_e5488 + noise_metadata_schedule_557_0_e5491);
        let noise_metadata_schedule_557_0_e5493: f64 = (noise_metadata_schedule_557_0_e5492).sqrt();
        let noise_metadata_schedule_557_0_e5494: f64 = (noise_metadata_schedule_557_0_e5485 / noise_metadata_schedule_557_0_e5493);
        (noise_metadata_schedule_557_0_e5494,)
    } else {
        (w[199],)
    }
};
            w[199] = noise_metadata_schedule_557_0_e5496;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_558_0_e5511,) = {
    if ((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) {
        let noise_metadata_schedule_558_0_e5507: f64 = (w[16] - w[244]);
        let noise_metadata_schedule_558_0_e5509: f64 = (noise_metadata_schedule_558_0_e5507 / w[199]);
        (noise_metadata_schedule_558_0_e5509,)
    } else {
        (w[200],)
    }
};
            w[200] = noise_metadata_schedule_558_0_e5511;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_559_0_e5530,) = {
    if ((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) {
        let noise_metadata_schedule_559_0_e5523: f64 = (0.5 * w[199]);
        let noise_metadata_schedule_559_0_e5525: f64 = (noise_metadata_schedule_559_0_e5523 * w[196]);
        let noise_metadata_schedule_559_0_e5527: f64 = (noise_metadata_schedule_559_0_e5525 * w[210]);
        let noise_metadata_schedule_559_0_e5528: f64 = (w[200] + noise_metadata_schedule_559_0_e5527);
        (noise_metadata_schedule_559_0_e5528,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_559_0_e5530;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_560_0_e5533: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[573] = noise_metadata_schedule_560_0_e5533;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_561_0_e5546,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[573] != 0.0)) {
        (w[201],)
    } else {
        (w[202],)
    }
};
            w[202] = noise_metadata_schedule_561_0_e5546;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_562_0_e5570,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[573] == 0.0)) {
        let noise_metadata_schedule_562_0_e5561: f64 = (2.0 * params[47]);
        let noise_metadata_schedule_562_0_e5565: f64 = (2.0 * w[122]);
        let noise_metadata_schedule_562_0_e5566: f64 = (1.0 + noise_metadata_schedule_562_0_e5565);
        let noise_metadata_schedule_562_0_e5567: f64 = (noise_metadata_schedule_562_0_e5561 * noise_metadata_schedule_562_0_e5566);
        let noise_metadata_schedule_562_0_e5568: f64 = (1.0 + noise_metadata_schedule_562_0_e5567);
        (noise_metadata_schedule_562_0_e5568,)
    } else {
        (w[203],)
    }
};
            w[203] = noise_metadata_schedule_562_0_e5570;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_563_0_e5592,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[573] == 0.0)) {
        let noise_metadata_schedule_563_0_e5584: f64 = (1.0 + params[47]);
        let noise_metadata_schedule_563_0_e5588: f64 = (2.0 * params[47]);
        let noise_metadata_schedule_563_0_e5589: f64 = (1.0 + noise_metadata_schedule_563_0_e5588);
        let noise_metadata_schedule_563_0_e5590: f64 = (noise_metadata_schedule_563_0_e5584 / noise_metadata_schedule_563_0_e5589);
        (noise_metadata_schedule_563_0_e5590,)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_563_0_e5592;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_564_0_e5620,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[573] == 0.0)) {
        let noise_metadata_schedule_564_0_e5607: f64 = (0.5 * w[199]);
        let noise_metadata_schedule_564_0_e5609: f64 = (noise_metadata_schedule_564_0_e5607 * w[196]);
        let noise_metadata_schedule_564_0_e5614: f64 = (params[62] * w[203]);
        let noise_metadata_schedule_564_0_e5615: f64 = (w[156] / noise_metadata_schedule_564_0_e5614);
        let noise_metadata_schedule_564_0_e5616: f64 = (w[204] - noise_metadata_schedule_564_0_e5615);
        let noise_metadata_schedule_564_0_e5617: f64 = (noise_metadata_schedule_564_0_e5609 * noise_metadata_schedule_564_0_e5616);
        let noise_metadata_schedule_564_0_e5618: f64 = (w[200] - noise_metadata_schedule_564_0_e5617);
        (noise_metadata_schedule_564_0_e5618,)
    } else {
        (w[205],)
    }
};
            w[205] = noise_metadata_schedule_564_0_e5620;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_565_0_e5650,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[573] == 0.0)) {
        let noise_metadata_schedule_565_0_e5634: f64 = (w[205] - w[201]);
        let noise_metadata_schedule_565_0_e5637: f64 = (w[205] - w[201]);
        let noise_metadata_schedule_565_0_e5638: f64 = (noise_metadata_schedule_565_0_e5634 * noise_metadata_schedule_565_0_e5637);
        let noise_metadata_schedule_565_0_e5641: f64 = (0.1 * w[200]);
        let noise_metadata_schedule_565_0_e5643: f64 = (noise_metadata_schedule_565_0_e5641 * w[200]);
        let noise_metadata_schedule_565_0_e5645: f64 = (noise_metadata_schedule_565_0_e5643 * w[134]);
        let noise_metadata_schedule_565_0_e5647: f64 = (noise_metadata_schedule_565_0_e5645 / params[62]);
        let noise_metadata_schedule_565_0_e5648: f64 = (noise_metadata_schedule_565_0_e5638 + noise_metadata_schedule_565_0_e5647);
        (noise_metadata_schedule_565_0_e5648,)
    } else {
        (w[280],)
    }
};
            w[280] = noise_metadata_schedule_565_0_e5650;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_566_0_e5671,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[573] == 0.0)) {
        let noise_metadata_schedule_566_0_e5665: f64 = (w[205] + w[201]);
        let noise_metadata_schedule_566_0_e5667: f64 = (w[280]).sqrt();
        let noise_metadata_schedule_566_0_e5668: f64 = (noise_metadata_schedule_566_0_e5665 + noise_metadata_schedule_566_0_e5667);
        let noise_metadata_schedule_566_0_e5669: f64 = (0.5 * noise_metadata_schedule_566_0_e5668);
        (noise_metadata_schedule_566_0_e5669,)
    } else {
        (w[202],)
    }
};
            w[202] = noise_metadata_schedule_566_0_e5671;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_567_0_e5686,) = {
    if ((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) {
        let noise_metadata_schedule_567_0_e5682: f64 = (w[202] - w[200]);
        let noise_metadata_schedule_567_0_e5684: f64 = (noise_metadata_schedule_567_0_e5682 / w[202]);
        (noise_metadata_schedule_567_0_e5684,)
    } else {
        (w[287],)
    }
};
            w[287] = noise_metadata_schedule_567_0_e5686;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_568_0_e5688: f64 = (w[287]).abs();
            let noise_metadata_schedule_568_0_e5690: f64 = if noise_metadata_schedule_568_0_e5688 > 1e-7 { 1.0 } else { 0.0 };
            w[574] = noise_metadata_schedule_568_0_e5690;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_569_0_e5707,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[574] != 0.0)) {
        let noise_metadata_schedule_569_0_e5703: f64 = (0.5 * w[199]);
        let noise_metadata_schedule_569_0_e5705: f64 = (noise_metadata_schedule_569_0_e5703 / w[287]);
        (noise_metadata_schedule_569_0_e5705,)
    } else {
        (w[206],)
    }
};
            w[206] = noise_metadata_schedule_569_0_e5707;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_10(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_570_0_e5744,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[574] != 0.0)) {
        let noise_metadata_schedule_570_0_e5720: f64 = (w[0] / w[98]);
        let noise_metadata_schedule_570_0_e5722: f64 = (noise_metadata_schedule_570_0_e5720 * w[202]);
        let noise_metadata_schedule_570_0_e5724: f64 = (noise_metadata_schedule_570_0_e5722 * w[206]);
        let noise_metadata_schedule_570_0_e5726: f64 = (-w[98]);
        let noise_metadata_schedule_570_0_e5728: f64 = (noise_metadata_schedule_570_0_e5726 / w[202]);
        let noise_metadata_schedule_570_0_e5729: f64 = (noise_metadata_schedule_570_0_e5728).exp();
        let noise_metadata_schedule_570_0_e5731: f64 = (-w[98]);
        let noise_metadata_schedule_570_0_e5733: f64 = (noise_metadata_schedule_570_0_e5731 / w[202]);
        let noise_metadata_schedule_570_0_e5737: f64 = (w[198] / w[206]);
        let noise_metadata_schedule_570_0_e5738: f64 = (1.0 + noise_metadata_schedule_570_0_e5737);
        let noise_metadata_schedule_570_0_e5739: f64 = (noise_metadata_schedule_570_0_e5733 * noise_metadata_schedule_570_0_e5738);
        let noise_metadata_schedule_570_0_e5740: f64 = (noise_metadata_schedule_570_0_e5739).exp();
        let noise_metadata_schedule_570_0_e5741: f64 = (noise_metadata_schedule_570_0_e5729 - noise_metadata_schedule_570_0_e5740);
        let noise_metadata_schedule_570_0_e5742: f64 = (noise_metadata_schedule_570_0_e5724 * noise_metadata_schedule_570_0_e5741);
        (noise_metadata_schedule_570_0_e5742,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_570_0_e5744;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_571_0_e5766,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] != 0.0)) && (w[571] != 0.0)) && (w[574] == 0.0)) {
        let noise_metadata_schedule_571_0_e5758: f64 = (w[0] * w[198]);
        let noise_metadata_schedule_571_0_e5760: f64 = (-w[98]);
        let noise_metadata_schedule_571_0_e5762: f64 = (noise_metadata_schedule_571_0_e5760 / w[202]);
        let noise_metadata_schedule_571_0_e5763: f64 = (noise_metadata_schedule_571_0_e5762).exp();
        let noise_metadata_schedule_571_0_e5764: f64 = (noise_metadata_schedule_571_0_e5758 * noise_metadata_schedule_571_0_e5763);
        (noise_metadata_schedule_571_0_e5764,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_571_0_e5766;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_572_0_e5769: f64 = if params[39] == 3.0 { 1.0 } else { 0.0 };
            w[575] = noise_metadata_schedule_572_0_e5769;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_573_0_e5772: f64 = if w[244] < params[44] { 1.0 } else { 0.0 };
            w[576] = noise_metadata_schedule_573_0_e5772;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_574_0_e5800,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) {
        let noise_metadata_schedule_574_0_e5786: f64 = (params[44] - w[244]);
        let noise_metadata_schedule_574_0_e5788: f64 = (noise_metadata_schedule_574_0_e5786).powf(params[41]);
        let noise_metadata_schedule_574_0_e5793: f64 = (params[48] + w[156]);
        let noise_metadata_schedule_574_0_e5794: f64 = (w[156] / noise_metadata_schedule_574_0_e5793);
        let noise_metadata_schedule_574_0_e5795: f64 = (1.0 - noise_metadata_schedule_574_0_e5794);
        let noise_metadata_schedule_574_0_e5797: f64 = (noise_metadata_schedule_574_0_e5795).powf(params[49]);
        let noise_metadata_schedule_574_0_e5798: f64 = (noise_metadata_schedule_574_0_e5788 * noise_metadata_schedule_574_0_e5797);
        (noise_metadata_schedule_574_0_e5798,)
    } else {
        (w[211],)
    }
};
            w[211] = noise_metadata_schedule_574_0_e5800;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_575_0_e5803: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[577] = noise_metadata_schedule_575_0_e5803;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_576_0_e5819,) = {
    if ((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[577] != 0.0)) {
        (w[211],)
    } else {
        (w[212],)
    }
};
            w[212] = noise_metadata_schedule_576_0_e5819;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_577_0_e5840,) = {
    if ((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[577] == 0.0)) {
        let noise_metadata_schedule_577_0_e5836: f64 = (w[156] - params[52]);
        let noise_metadata_schedule_577_0_e5838: f64 = (noise_metadata_schedule_577_0_e5836 / params[48]);
        (noise_metadata_schedule_577_0_e5838,)
    } else {
        (w[213],)
    }
};
            w[213] = noise_metadata_schedule_577_0_e5840;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_578_0_e5861,) = {
    if ((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[577] == 0.0)) {
        let noise_metadata_schedule_578_0_e5857: f64 = (w[213] - 1.0);
        let noise_metadata_schedule_578_0_e5859: f64 = (noise_metadata_schedule_578_0_e5857 / params[51]);
        (noise_metadata_schedule_578_0_e5859,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_578_0_e5861;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_579_0_e5864: f64 = if w[213] < 1.0 { 1.0 } else { 0.0 };
            w[578] = noise_metadata_schedule_579_0_e5864;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_580_0_e5891,) = {
    if (((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[577] == 0.0)) && (w[578] != 0.0)) {
        let noise_metadata_schedule_580_0_e5885: f64 = (w[279]).exp();
        let noise_metadata_schedule_580_0_e5886: f64 = (1.0 + noise_metadata_schedule_580_0_e5885);
        let noise_metadata_schedule_580_0_e5887: f64 = (noise_metadata_schedule_580_0_e5886).ln();
        let noise_metadata_schedule_580_0_e5888: f64 = (params[51] * noise_metadata_schedule_580_0_e5887);
        let noise_metadata_schedule_580_0_e5889: f64 = (1.0 + noise_metadata_schedule_580_0_e5888);
        (noise_metadata_schedule_580_0_e5889,)
    } else {
        (w[214],)
    }
};
            w[214] = noise_metadata_schedule_580_0_e5891;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_581_0_e5920,) = {
    if (((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[577] == 0.0)) && (w[578] == 0.0)) {
        let noise_metadata_schedule_581_0_e5913: f64 = (-w[279]);
        let noise_metadata_schedule_581_0_e5914: f64 = (noise_metadata_schedule_581_0_e5913).exp();
        let noise_metadata_schedule_581_0_e5915: f64 = (1.0 + noise_metadata_schedule_581_0_e5914);
        let noise_metadata_schedule_581_0_e5916: f64 = (noise_metadata_schedule_581_0_e5915).ln();
        let noise_metadata_schedule_581_0_e5917: f64 = (params[51] * noise_metadata_schedule_581_0_e5916);
        let noise_metadata_schedule_581_0_e5918: f64 = (w[213] + noise_metadata_schedule_581_0_e5917);
        (noise_metadata_schedule_581_0_e5918,)
    } else {
        (w[214],)
    }
};
            w[214] = noise_metadata_schedule_581_0_e5920;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_582_0_e5941,) = {
    if ((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[577] == 0.0)) {
        let noise_metadata_schedule_582_0_e5938: f64 = (w[214]).powf(params[50]);
        let noise_metadata_schedule_582_0_e5939: f64 = (w[211] * noise_metadata_schedule_582_0_e5938);
        (noise_metadata_schedule_582_0_e5939,)
    } else {
        (w[212],)
    }
};
            w[212] = noise_metadata_schedule_582_0_e5941;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_583_0_e5943: f64 = (-w[334]);
            let noise_metadata_schedule_583_0_e5945: f64 = (noise_metadata_schedule_583_0_e5943 * w[212]);
            let noise_metadata_schedule_583_0_e5947: f64 = if noise_metadata_schedule_583_0_e5945 < params[147] { 1.0 } else { 0.0 };
            w[579] = noise_metadata_schedule_583_0_e5947;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_584_0_e5967,) = {
    if ((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[579] != 0.0)) {
        let noise_metadata_schedule_584_0_e5962: f64 = (-w[334]);
        let noise_metadata_schedule_584_0_e5964: f64 = (noise_metadata_schedule_584_0_e5962 * w[212]);
        let noise_metadata_schedule_584_0_e5965: f64 = (noise_metadata_schedule_584_0_e5964).exp();
        (noise_metadata_schedule_584_0_e5965,)
    } else {
        (w[337],)
    }
};
            w[337] = noise_metadata_schedule_584_0_e5967;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_585_0_e5985,) = {
    if ((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[579] == 0.0)) {
        let noise_metadata_schedule_585_0_e5983: f64 = (params[147]).exp();
        (noise_metadata_schedule_585_0_e5983,)
    } else {
        (w[295],)
    }
};
            w[295] = noise_metadata_schedule_585_0_e5985;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_586_0_e6011,) = {
    if ((((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) && (w[579] == 0.0)) {
        let noise_metadata_schedule_586_0_e6003: f64 = (-w[334]);
        let noise_metadata_schedule_586_0_e6005: f64 = (noise_metadata_schedule_586_0_e6003 * w[212]);
        let noise_metadata_schedule_586_0_e6007: f64 = (noise_metadata_schedule_586_0_e6005 - params[147]);
        let noise_metadata_schedule_586_0_e6008: f64 = (1.0 + noise_metadata_schedule_586_0_e6007);
        let noise_metadata_schedule_586_0_e6009: f64 = (w[295] * noise_metadata_schedule_586_0_e6008);
        (noise_metadata_schedule_586_0_e6009,)
    } else {
        (w[337],)
    }
};
            w[337] = noise_metadata_schedule_586_0_e6011;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_587_0_e6033,) = {
    if (((((w[565] != 0.0) && (w[566] == 0.0)) && (w[570] == 0.0)) && (w[575] != 0.0)) && (w[576] != 0.0)) {
        let noise_metadata_schedule_587_0_e6025: f64 = (params[40] / w[334]);
        let noise_metadata_schedule_587_0_e6028: f64 = (params[44] - w[244]);
        let noise_metadata_schedule_587_0_e6029: f64 = (noise_metadata_schedule_587_0_e6025 * noise_metadata_schedule_587_0_e6028);
        let noise_metadata_schedule_587_0_e6031: f64 = (noise_metadata_schedule_587_0_e6029 * w[337]);
        (noise_metadata_schedule_587_0_e6031,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_587_0_e6033;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_588_0_e6036: f64 = if w[207] > 0.0 { 1.0 } else { 0.0 };
            w[580] = noise_metadata_schedule_588_0_e6036;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_589_0_e6039: f64 = if params[53] == 1.0 { 1.0 } else { 0.0 };
            w[581] = noise_metadata_schedule_589_0_e6039;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_590_0_e6065,) = {
    if (((w[565] != 0.0) && (w[580] != 0.0)) && (w[581] != 0.0)) {
        let noise_metadata_schedule_590_0_e6049: f64 = (w[30] + w[186]);
        let noise_metadata_schedule_590_0_e6050: f64 = (w[156] * noise_metadata_schedule_590_0_e6049);
        let noise_metadata_schedule_590_0_e6051: f64 = (w[6] / noise_metadata_schedule_590_0_e6050);
        let noise_metadata_schedule_590_0_e6054: f64 = (w[153] / w[35]);
        let noise_metadata_schedule_590_0_e6056: f64 = (noise_metadata_schedule_590_0_e6054 * w[42]);
        let noise_metadata_schedule_590_0_e6057: f64 = (noise_metadata_schedule_590_0_e6051 + noise_metadata_schedule_590_0_e6056);
        let noise_metadata_schedule_590_0_e6061: f64 = (w[30] + w[186]);
        let noise_metadata_schedule_590_0_e6062: f64 = (w[28] / noise_metadata_schedule_590_0_e6061);
        let noise_metadata_schedule_590_0_e6063: f64 = (noise_metadata_schedule_590_0_e6057 + noise_metadata_schedule_590_0_e6062);
        (noise_metadata_schedule_590_0_e6063,)
    } else {
        (w[208],)
    }
};
            w[208] = noise_metadata_schedule_590_0_e6065;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_591_0_e6068: f64 = if params[39] == 3.0 { 1.0 } else { 0.0 };
            w[582] = noise_metadata_schedule_591_0_e6068;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_592_0_e6082,) = {
    if ((((w[565] != 0.0) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] != 0.0)) {
        let noise_metadata_schedule_592_0_e6078: f64 = (w[207] - w[208]);
        let noise_metadata_schedule_592_0_e6080: f64 = (noise_metadata_schedule_592_0_e6078 / 1e-6);
        (noise_metadata_schedule_592_0_e6080,)
    } else {
        (w[279],)
    }
};
            w[279] = noise_metadata_schedule_592_0_e6082;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_593_0_e6085: f64 = if w[207] < w[208] { 1.0 } else { 0.0 };
            w[583] = noise_metadata_schedule_593_0_e6085;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_594_0_e6105,) = {
    if (((((w[565] != 0.0) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] != 0.0)) && (w[583] != 0.0)) {
        let noise_metadata_schedule_594_0_e6099: f64 = (w[279]).exp();
        let noise_metadata_schedule_594_0_e6100: f64 = (1.0 + noise_metadata_schedule_594_0_e6099);
        let noise_metadata_schedule_594_0_e6101: f64 = (noise_metadata_schedule_594_0_e6100).ln();
        let noise_metadata_schedule_594_0_e6102: f64 = (1e-6 * noise_metadata_schedule_594_0_e6101);
        let noise_metadata_schedule_594_0_e6103: f64 = (w[207] - noise_metadata_schedule_594_0_e6102);
        (noise_metadata_schedule_594_0_e6103,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_594_0_e6105;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_595_0_e6127,) = {
    if (((((w[565] != 0.0) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] != 0.0)) && (w[583] == 0.0)) {
        let noise_metadata_schedule_595_0_e6120: f64 = (-w[279]);
        let noise_metadata_schedule_595_0_e6121: f64 = (noise_metadata_schedule_595_0_e6120).exp();
        let noise_metadata_schedule_595_0_e6122: f64 = (1.0 + noise_metadata_schedule_595_0_e6121);
        let noise_metadata_schedule_595_0_e6123: f64 = (noise_metadata_schedule_595_0_e6122).ln();
        let noise_metadata_schedule_595_0_e6124: f64 = (1e-6 * noise_metadata_schedule_595_0_e6123);
        let noise_metadata_schedule_595_0_e6125: f64 = (w[208] - noise_metadata_schedule_595_0_e6124);
        (noise_metadata_schedule_595_0_e6125,)
    } else {
        (w[207],)
    }
};
            w[207] = noise_metadata_schedule_595_0_e6127;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_596_0_e6139,) = {
    if ((((w[565] != 0.0) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] != 0.0)) {
        let noise_metadata_schedule_596_0_e6137: f64 = (w[156] * w[207]);
        (noise_metadata_schedule_596_0_e6137,)
    } else {
        (w[209],)
    }
};
            w[209] = noise_metadata_schedule_596_0_e6139;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_597_0_e6158,) = {
    if ((((w[565] != 0.0) && (w[580] != 0.0)) && (w[581] != 0.0)) && (w[582] == 0.0)) {
        let noise_metadata_schedule_597_0_e6150: f64 = (w[156] * w[207]);
        let noise_metadata_schedule_597_0_e6152: f64 = (noise_metadata_schedule_597_0_e6150 * w[208]);
        let noise_metadata_schedule_597_0_e6155: f64 = (w[207] + w[208]);
        let noise_metadata_schedule_597_0_e6156: f64 = (noise_metadata_schedule_597_0_e6152 / noise_metadata_schedule_597_0_e6155);
        (noise_metadata_schedule_597_0_e6156,)
    } else {
        (w[209],)
    }
};
            w[209] = noise_metadata_schedule_597_0_e6158;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_598_0_e6169,) = {
    if (((w[565] != 0.0) && (w[580] != 0.0)) && (w[581] == 0.0)) {
        let noise_metadata_schedule_598_0_e6167: f64 = (w[156] * w[207]);
        (noise_metadata_schedule_598_0_e6167,)
    } else {
        (w[209],)
    }
};
            w[209] = noise_metadata_schedule_598_0_e6169;
        }
        if (active[0] & 0xff00038) != 0 {
            let noise_metadata_schedule_683_0_e7006: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_683_0_e7008: f64 = (noise_metadata_schedule_683_0_e7006 * w[2]);
            w[302] = noise_metadata_schedule_683_0_e7008;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_684_0_e7011: f64 = (w[302] / w[28]);
            w[303] = noise_metadata_schedule_684_0_e7011;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_685_0_e7014: f64 = (w[302] / w[30]);
            w[304] = noise_metadata_schedule_685_0_e7014;
        }
        if (active[0] & 0xa900000) != 0 {
            let noise_metadata_schedule_686_0_e7017: f64 = (w[302] * w[108]);
            w[305] = noise_metadata_schedule_686_0_e7017;
        }
        if (active[0] & 0x1200000) != 0 {
            let noise_metadata_schedule_687_0_e7020: f64 = (w[302] * w[109]);
            w[306] = noise_metadata_schedule_687_0_e7020;
        }
        if (active[0] & 0x4400000) != 0 {
            let noise_metadata_schedule_688_0_e7023: f64 = (w[302] * w[110]);
            w[307] = noise_metadata_schedule_688_0_e7023;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_689_0_e7026: f64 = (w[302] / w[186]);
            let noise_metadata_schedule_689_0_e7029: f64 = (4.0 * w[267]);
            let noise_metadata_schedule_689_0_e7031: f64 = (noise_metadata_schedule_689_0_e7029 + 5.0);
            let noise_metadata_schedule_689_0_e7032: f64 = (noise_metadata_schedule_689_0_e7026 * noise_metadata_schedule_689_0_e7031);
            let noise_metadata_schedule_689_0_e7034: f64 = (noise_metadata_schedule_689_0_e7032 * 0.3333333333333333);
            w[308] = noise_metadata_schedule_689_0_e7034;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_690_0_e7037: f64 = (w[155] + w[154]);
            let noise_metadata_schedule_690_0_e7039: f64 = (noise_metadata_schedule_690_0_e7037 / w[153]);
            w[327] = noise_metadata_schedule_690_0_e7039;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_691_0_e7042: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_691_0_e7044: f64 = (w[327]).abs();
            let noise_metadata_schedule_691_0_e7045: f64 = (noise_metadata_schedule_691_0_e7042 * noise_metadata_schedule_691_0_e7044);
            w[309] = noise_metadata_schedule_691_0_e7045;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_692_0_e7048: f64 = if params[130] > 0.0 { 1.0 } else { 0.0 };
            w[600] = noise_metadata_schedule_692_0_e7048;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_693_0_e7055,) = {
    if (w[600] != 0.0) {
        let noise_metadata_schedule_693_0_e7052: f64 = (w[209] / w[327]);
        let noise_metadata_schedule_693_0_e7053: f64 = (noise_metadata_schedule_693_0_e7052).abs();
        (noise_metadata_schedule_693_0_e7053,)
    } else {
        (w[328],)
    }
};
            w[328] = noise_metadata_schedule_693_0_e7055;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_694_0_e7060,) = {
    if (w[600] == 0.0) {
        (0.0,)
    } else {
        (w[328],)
    }
};
            w[328] = noise_metadata_schedule_694_0_e7060;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_695_0_e7063: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_695_0_e7065: f64 = (noise_metadata_schedule_695_0_e7063 * w[209]);
            let noise_metadata_schedule_695_0_e7068: f64 = (w[328] + 1.0);
            let noise_metadata_schedule_695_0_e7069: f64 = (noise_metadata_schedule_695_0_e7065 * noise_metadata_schedule_695_0_e7068);
            w[321] = noise_metadata_schedule_695_0_e7069;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_704_0_e7121: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_704_0_e7124: f64 = (w[158] + w[160]);
            let noise_metadata_schedule_704_0_e7126: f64 = (noise_metadata_schedule_704_0_e7124 - w[57]);
            let noise_metadata_schedule_704_0_e7128: f64 = (noise_metadata_schedule_704_0_e7126 + w[352]);
            let noise_metadata_schedule_704_0_e7130: f64 = (noise_metadata_schedule_704_0_e7128 + w[351]);
            let noise_metadata_schedule_704_0_e7131: f64 = (noise_metadata_schedule_704_0_e7130).abs();
            let noise_metadata_schedule_704_0_e7132: f64 = (noise_metadata_schedule_704_0_e7121 * noise_metadata_schedule_704_0_e7131);
            w[310] = noise_metadata_schedule_704_0_e7132;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_705_0_e7135: f64 = (w[158] + w[159]);
            w[322] = noise_metadata_schedule_705_0_e7135;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_706_0_e7138: f64 = (w[322]).abs();
            let noise_metadata_schedule_706_0_e7140: f64 = (noise_metadata_schedule_706_0_e7138).powf(params[126]);
            let noise_metadata_schedule_706_0_e7141: f64 = (params[128] * noise_metadata_schedule_706_0_e7140);
            w[311] = noise_metadata_schedule_706_0_e7141;
        }
        if (active[0] & 0x40) != 0 {
            let noise_metadata_schedule_707_0_e7144: f64 = if w[322] < 0.0 { 1.0 } else { 0.0 };
            w[604] = noise_metadata_schedule_707_0_e7144;
        }
        if (active[0] & 0x40) != 0 {
            let (noise_metadata_schedule_708_0_e7149,) = {
    if (w[604] != 0.0) {
        let noise_metadata_schedule_708_0_e7147: f64 = (-w[311]);
        (noise_metadata_schedule_708_0_e7147,)
    } else {
        (w[311],)
    }
};
            w[311] = noise_metadata_schedule_708_0_e7149;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_709_0_e7152: f64 = (w[160] + w[162]);
            let noise_metadata_schedule_709_0_e7154: f64 = (noise_metadata_schedule_709_0_e7152 + w[163]);
            w[323] = noise_metadata_schedule_709_0_e7154;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_710_0_e7157: f64 = (w[323]).abs();
            let noise_metadata_schedule_710_0_e7159: f64 = (noise_metadata_schedule_710_0_e7157).powf(params[127]);
            let noise_metadata_schedule_710_0_e7160: f64 = (params[129] * noise_metadata_schedule_710_0_e7159);
            w[312] = noise_metadata_schedule_710_0_e7160;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_711_0_e7163: f64 = if w[323] < 0.0 { 1.0 } else { 0.0 };
            w[605] = noise_metadata_schedule_711_0_e7163;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_712_0_e7168,) = {
    if (w[605] != 0.0) {
        let noise_metadata_schedule_712_0_e7166: f64 = (-w[312]);
        (noise_metadata_schedule_712_0_e7166,)
    } else {
        (w[312],)
    }
};
            w[312] = noise_metadata_schedule_712_0_e7168;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_713_0_e7171: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_713_0_e7174: f64 = (w[159] + w[162]);
            let noise_metadata_schedule_713_0_e7176: f64 = (noise_metadata_schedule_713_0_e7174 + w[163]);
            let noise_metadata_schedule_713_0_e7177: f64 = (noise_metadata_schedule_713_0_e7176).abs();
            let noise_metadata_schedule_713_0_e7178: f64 = (noise_metadata_schedule_713_0_e7171 * noise_metadata_schedule_713_0_e7177);
            w[313] = noise_metadata_schedule_713_0_e7178;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_714_0_e7181: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_714_0_e7183: f64 = (w[161]).abs();
            let noise_metadata_schedule_714_0_e7184: f64 = (noise_metadata_schedule_714_0_e7181 * noise_metadata_schedule_714_0_e7183);
            w[314] = noise_metadata_schedule_714_0_e7184;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_715_0_e7187: f64 = (w[161]).abs();
            let noise_metadata_schedule_715_0_e7189: f64 = (noise_metadata_schedule_715_0_e7187).powf(params[126]);
            let noise_metadata_schedule_715_0_e7190: f64 = (params[128] * noise_metadata_schedule_715_0_e7189);
            w[315] = noise_metadata_schedule_715_0_e7190;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_716_0_e7193: f64 = if w[161] < 0.0 { 1.0 } else { 0.0 };
            w[606] = noise_metadata_schedule_716_0_e7193;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_717_0_e7198,) = {
    if (w[606] != 0.0) {
        let noise_metadata_schedule_717_0_e7196: f64 = (-w[315]);
        (noise_metadata_schedule_717_0_e7196,)
    } else {
        (w[315],)
    }
};
            w[315] = noise_metadata_schedule_717_0_e7198;
        }
        if (active[0] & 0x18000) != 0 {
            let noise_metadata_schedule_718_0_e7201: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_718_0_e7203: f64 = (w[82]).abs();
            let noise_metadata_schedule_718_0_e7204: f64 = (noise_metadata_schedule_718_0_e7201 * noise_metadata_schedule_718_0_e7203);
            w[316] = noise_metadata_schedule_718_0_e7204;
        }
        if (active[0] & 0x800) != 0 {
            let noise_metadata_schedule_719_0_e7207: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_719_0_e7209: f64 = (w[164]).abs();
            let noise_metadata_schedule_719_0_e7210: f64 = (noise_metadata_schedule_719_0_e7207 * noise_metadata_schedule_719_0_e7209);
            w[317] = noise_metadata_schedule_719_0_e7210;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_11(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 616], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_720_0_e7215: f64 = (params[5] * params[33]);
            let noise_metadata_schedule_720_0_e7216: f64 = (1.0 - noise_metadata_schedule_720_0_e7215);
            let noise_metadata_schedule_720_0_e7217: f64 = (params[128] * noise_metadata_schedule_720_0_e7216);
            let noise_metadata_schedule_720_0_e7219: f64 = (w[164]).abs();
            let noise_metadata_schedule_720_0_e7223: f64 = (params[5] * params[33]);
            let noise_metadata_schedule_720_0_e7224: f64 = (1.0 - noise_metadata_schedule_720_0_e7223);
            let noise_metadata_schedule_720_0_e7225: f64 = (noise_metadata_schedule_720_0_e7219 / noise_metadata_schedule_720_0_e7224);
            let noise_metadata_schedule_720_0_e7227: f64 = (noise_metadata_schedule_720_0_e7225).powf(params[126]);
            let noise_metadata_schedule_720_0_e7228: f64 = (noise_metadata_schedule_720_0_e7217 * noise_metadata_schedule_720_0_e7227);
            w[319] = noise_metadata_schedule_720_0_e7228;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_721_0_e7231: f64 = if w[164] < 0.0 { 1.0 } else { 0.0 };
            w[607] = noise_metadata_schedule_721_0_e7231;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_722_0_e7236,) = {
    if (w[607] != 0.0) {
        let noise_metadata_schedule_722_0_e7234: f64 = (-w[319]);
        (noise_metadata_schedule_722_0_e7234,)
    } else {
        (w[319],)
    }
};
            w[319] = noise_metadata_schedule_722_0_e7236;
        }
        if (active[0] & 0x2000) != 0 {
            let noise_metadata_schedule_723_0_e7239: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_723_0_e7241: f64 = (w[176]).abs();
            let noise_metadata_schedule_723_0_e7242: f64 = (noise_metadata_schedule_723_0_e7239 * noise_metadata_schedule_723_0_e7241);
            let noise_metadata_schedule_723_0_e7244: f64 = (noise_metadata_schedule_723_0_e7242 * params[5]);
            w[318] = noise_metadata_schedule_723_0_e7244;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_724_0_e7247: f64 = if params[33] == 0.0 { 1.0 } else { 0.0 };
            w[608] = noise_metadata_schedule_724_0_e7247;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_725_0_e7251,) = {
    if (w[608] != 0.0) {
        (0.0,)
    } else {
        (w[320],)
    }
};
            w[320] = noise_metadata_schedule_725_0_e7251;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_726_0_e7267,) = {
    if (w[608] == 0.0) {
        let noise_metadata_schedule_726_0_e7256: f64 = (params[128] * params[5]);
        let noise_metadata_schedule_726_0_e7258: f64 = (noise_metadata_schedule_726_0_e7256 * params[33]);
        let noise_metadata_schedule_726_0_e7260: f64 = (w[176]).abs();
        let noise_metadata_schedule_726_0_e7262: f64 = (noise_metadata_schedule_726_0_e7260 / params[33]);
        let noise_metadata_schedule_726_0_e7264: f64 = (noise_metadata_schedule_726_0_e7262).powf(params[126]);
        let noise_metadata_schedule_726_0_e7265: f64 = (noise_metadata_schedule_726_0_e7258 * noise_metadata_schedule_726_0_e7264);
        (noise_metadata_schedule_726_0_e7265,)
    } else {
        (w[320],)
    }
};
            w[320] = noise_metadata_schedule_726_0_e7267;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_727_0_e7270: f64 = if w[176] < 0.0 { 1.0 } else { 0.0 };
            w[609] = noise_metadata_schedule_727_0_e7270;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_728_0_e7275,) = {
    if (w[609] != 0.0) {
        let noise_metadata_schedule_728_0_e7273: f64 = (-w[320]);
        (noise_metadata_schedule_728_0_e7273,)
    } else {
        (w[320],)
    }
};
            w[320] = noise_metadata_schedule_728_0_e7275;
        }
        if (active[0] & 0x20000) != 0 {
            let noise_metadata_schedule_729_0_e7278: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_729_0_e7280: f64 = (w[182]).abs();
            let noise_metadata_schedule_729_0_e7281: f64 = (noise_metadata_schedule_729_0_e7278 * noise_metadata_schedule_729_0_e7280);
            w[324] = noise_metadata_schedule_729_0_e7281;
        }
        if (active[0] & 0x40000) != 0 {
            let noise_metadata_schedule_730_0_e7284: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_730_0_e7286: f64 = (w[179]).abs();
            let noise_metadata_schedule_730_0_e7287: f64 = (noise_metadata_schedule_730_0_e7284 * noise_metadata_schedule_730_0_e7286);
            w[325] = noise_metadata_schedule_730_0_e7287;
        }
        if (active[0] & 0x80000) != 0 {
            let noise_metadata_schedule_731_0_e7290: f64 = (2.0 * 1.6021918e-19);
            let noise_metadata_schedule_731_0_e7292: f64 = (w[180]).abs();
            let noise_metadata_schedule_731_0_e7293: f64 = (noise_metadata_schedule_731_0_e7290 * noise_metadata_schedule_731_0_e7292);
            w[326] = noise_metadata_schedule_731_0_e7293;
        }
    }
}

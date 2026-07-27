#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

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
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 341];
        let noise_source_0_active = {
            params[1] != 0.0
        };
        let noise_source_1_active = {
            params[1] != 0.0
        };
        let noise_source_2_active = {
            params[1] != 0.0
        };
        let noise_source_3_active = {
            params[1] != 0.0
        };
        let noise_source_4_active = {
            params[1] != 0.0
        };
        let noise_source_5_active = {
            params[1] != 0.0
        };
        let noise_source_6_active = {
            params[1] != 0.0
        };
        let noise_source_7_active = {
            params[1] != 0.0
        };
        let noise_source_8_active = {
            params[1] != 0.0
        };
        let noise_source_9_active = {
            params[1] != 0.0
        };
        let noise_source_10_active = {
            params[1] != 0.0
        };
        let noise_source_11_active = {
            params[1] != 0.0
        };
        let noise_source_12_active = {
            params[1] != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active, noise_source_4_active, noise_source_5_active, noise_source_6_active, noise_source_7_active, noise_source_8_active, noise_source_9_active, noise_source_10_active, noise_source_11_active, noise_source_12_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3) | ((noise_source_4_active as u128) << 4) | ((noise_source_5_active as u128) << 5) | ((noise_source_6_active as u128) << 6) | ((noise_source_7_active as u128) << 7) | ((noise_source_8_active as u128) << 8) | ((noise_source_9_active as u128) << 9) | ((noise_source_10_active as u128) << 10) | ((noise_source_11_active as u128) << 11) | ((noise_source_12_active as u128) << 12)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_2(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_3(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_4(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_5(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_6(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_7(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e5755: f64 = 1.0;
            let noise_0_psd_e178: f64 = 2.0;
            let noise_0_psd_e180: f64 = (noise_0_psd_e178 * 1.602189e-19);
            let noise_0_psd_e182: f64 = (w[87]).abs();
            let noise_0_psd_e183: f64 = (noise_0_psd_e180 * noise_0_psd_e182);
            let noise_0_psd_e5756: f64 = (noise_0_psd_e5755 * noise_0_psd_e183);
            let psd = noise_0_psd_e5756;
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
            let noise_1_psd_e5758: f64 = 1.0;
            let noise_1_psd_e191: f64 = params[98];
            let noise_1_psd_e194: f64 = w[87];
            let noise_1_psd_e195: f64 = (noise_1_psd_e194).abs();
            let noise_1_psd_e197: f64 = (noise_1_psd_e195).powf(params[99]);
            let noise_1_psd_e198: f64 = (noise_1_psd_e191 * noise_1_psd_e197);
            let noise_1_psd_e5759: f64 = (noise_1_psd_e5758 * noise_1_psd_e198);
            let psd = noise_1_psd_e5759;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(params[100]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[2] {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_2_psd_e5761: f64 = 1.0;
            let noise_2_psd_e207: f64 = 2.0;
            let noise_2_psd_e209: f64 = (noise_2_psd_e207 * 1.602189e-19);
            let noise_2_psd_e211: f64 = (w[88]).abs();
            let noise_2_psd_e212: f64 = (noise_2_psd_e209 * noise_2_psd_e211);
            let noise_2_psd_e5762: f64 = (noise_2_psd_e5761 * noise_2_psd_e212);
            let psd = noise_2_psd_e5762;
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
            let noise_3_psd_e5764: f64 = 1.0;
            let noise_3_psd_e220: f64 = params[98];
            let noise_3_psd_e223: f64 = w[88];
            let noise_3_psd_e224: f64 = (noise_3_psd_e223).abs();
            let noise_3_psd_e226: f64 = (noise_3_psd_e224).powf(params[99]);
            let noise_3_psd_e227: f64 = (noise_3_psd_e220 * noise_3_psd_e226);
            let noise_3_psd_e5765: f64 = (noise_3_psd_e5764 * noise_3_psd_e227);
            let psd = noise_3_psd_e5765;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = Some(params[100]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[4] {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_4_psd_e5767: f64 = 1.0;
            let noise_4_psd_e236: f64 = 2.0;
            let noise_4_psd_e238: f64 = (noise_4_psd_e236 * 1.602189e-19);
            let noise_4_psd_e240: f64 = (w[76]).abs();
            let noise_4_psd_e241: f64 = (noise_4_psd_e238 * noise_4_psd_e240);
            let noise_4_psd_e5768: f64 = (noise_4_psd_e5767 * noise_4_psd_e241);
            let psd = noise_4_psd_e5768;
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
            let noise_5_psd_e5770: f64 = 1.0;
            let noise_5_psd_e249: f64 = 2.0;
            let noise_5_psd_e251: f64 = (noise_5_psd_e249 * 1.602189e-19);
            let noise_5_psd_e253: f64 = (w[91]).abs();
            let noise_5_psd_e254: f64 = (noise_5_psd_e251 * noise_5_psd_e253);
            let noise_5_psd_e5771: f64 = (noise_5_psd_e5770 * noise_5_psd_e254);
            let psd = noise_5_psd_e5771;
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
            let noise_6_psd_e5773: f64 = 1.0;
            let noise_6_psd_e262: f64 = 1.0;
            let noise_6_psd_e264: f64 = (noise_6_psd_e262 * params[98]);
            let noise_6_psd_e267: f64 = w[91];
            let noise_6_psd_e268: f64 = (noise_6_psd_e267).abs();
            let noise_6_psd_e270: f64 = (noise_6_psd_e268).powf(params[99]);
            let noise_6_psd_e271: f64 = (noise_6_psd_e264 * noise_6_psd_e270);
            let noise_6_psd_e5774: f64 = (noise_6_psd_e5773 * noise_6_psd_e271);
            let psd = noise_6_psd_e5774;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = Some(params[100]);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[7] {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_7_psd_e5776: f64 = 1.0;
            let noise_7_psd_e280: f64 = 4.0;
            let noise_7_psd_e282: f64 = (noise_7_psd_e280 * 1.380662e-23);
            let noise_7_psd_e284: f64 = (noise_7_psd_e282 * w[39]);
            let noise_7_psd_e286: f64 = (noise_7_psd_e284 * w[53]);
            let noise_7_psd_e5777: f64 = (noise_7_psd_e5776 * noise_7_psd_e286);
            let psd = noise_7_psd_e5777;
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
            let noise_8_psd_e5779: f64 = 1.0;
            let noise_8_psd_e294: f64 = 4.0;
            let noise_8_psd_e296: f64 = (noise_8_psd_e294 * 1.380662e-23);
            let noise_8_psd_e298: f64 = (noise_8_psd_e296 * w[39]);
            let noise_8_psd_e300: f64 = (w[97]).abs();
            let noise_8_psd_e303: f64 = (1e-10 * w[54]);
            let noise_8_psd_e304: f64 = (noise_8_psd_e300 + noise_8_psd_e303);
            let noise_8_psd_e306: f64 = (w[154]).abs();
            let noise_8_psd_e308: f64 = (noise_8_psd_e306 + 1e-10);
            let noise_8_psd_e309: f64 = (noise_8_psd_e304 / noise_8_psd_e308);
            let noise_8_psd_e310: f64 = (noise_8_psd_e298 * noise_8_psd_e309);
            let noise_8_psd_e5780: f64 = (noise_8_psd_e5779 * noise_8_psd_e310);
            let psd = noise_8_psd_e5780;
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
            let noise_9_psd_e5782: f64 = 1.0;
            let noise_9_psd_e318: f64 = 4.0;
            let noise_9_psd_e320: f64 = (noise_9_psd_e318 * 1.380662e-23);
            let noise_9_psd_e322: f64 = (noise_9_psd_e320 * w[39]);
            let noise_9_psd_e324: f64 = (noise_9_psd_e322 * w[55]);
            let noise_9_psd_e5783: f64 = (noise_9_psd_e5782 * noise_9_psd_e324);
            let psd = noise_9_psd_e5783;
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
            let noise_10_psd_e5785: f64 = 1.0;
            let noise_10_psd_e332: f64 = 4.0;
            let noise_10_psd_e334: f64 = (noise_10_psd_e332 * 1.380662e-23);
            let noise_10_psd_e336: f64 = (noise_10_psd_e334 * w[39]);
            let noise_10_psd_e338: f64 = (noise_10_psd_e336 * w[81]);
            let noise_10_psd_e340: f64 = (noise_10_psd_e338 * w[56]);
            let noise_10_psd_e5786: f64 = (noise_10_psd_e5785 * noise_10_psd_e340);
            let psd = noise_10_psd_e5786;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[11] {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_11_psd_e5788: f64 = 1.0;
            let noise_11_psd_e348: f64 = 4.0;
            let noise_11_psd_e350: f64 = (noise_11_psd_e348 * 1.380662e-23);
            let noise_11_psd_e352: f64 = (noise_11_psd_e350 * w[39]);
            let noise_11_psd_e354: f64 = (noise_11_psd_e352 * w[57]);
            let noise_11_psd_e5789: f64 = (noise_11_psd_e5788 * noise_11_psd_e354);
            let psd = noise_11_psd_e5789;
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
            let noise_12_psd_e5791: f64 = 1.0;
            let noise_12_psd_e362: f64 = 4.0;
            let noise_12_psd_e364: f64 = (noise_12_psd_e362 * 1.380662e-23);
            let noise_12_psd_e366: f64 = (noise_12_psd_e364 * w[39]);
            let noise_12_psd_e368: f64 = (noise_12_psd_e366 * w[86]);
            let noise_12_psd_e370: f64 = (noise_12_psd_e368 * w[58]);
            let noise_12_psd_e5792: f64 = (noise_12_psd_e5791 * noise_12_psd_e370);
            let psd = noise_12_psd_e5792;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1fff) != 0 {
            let noise_metadata_schedule_0_0_e376: f64 = if ctx.analysis_initial_step() { 1.0 } else { 0.0 };
            w[172] = noise_metadata_schedule_0_0_e376;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_7_0_e420: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            w[175] = noise_metadata_schedule_7_0_e420;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_8_0_e426,) = {
    if ((w[172] != 0.0) && (w[175] != 0.0)) {
        (params[10],)
    } else {
        (w[165],)
    }
};
            w[165] = noise_metadata_schedule_8_0_e426;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_9_0_e435,) = {
    if ((w[172] != 0.0) && (w[175] == 0.0)) {
        let noise_metadata_schedule_9_0_e433: f64 = 1e-12;
        (noise_metadata_schedule_9_0_e433,)
    } else {
        (w[165],)
    }
};
            w[165] = noise_metadata_schedule_9_0_e435;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_10_0_e437: f64 = if self.param_given[11] { 1.0 } else { 0.0 };
            w[176] = noise_metadata_schedule_10_0_e437;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_11_0_e443,) = {
    if ((w[172] != 0.0) && (w[176] != 0.0)) {
        (params[11],)
    } else {
        (w[166],)
    }
};
            w[166] = noise_metadata_schedule_11_0_e443;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_12_0_e452,) = {
    if ((w[172] != 0.0) && (w[176] == 0.0)) {
        let noise_metadata_schedule_12_0_e450: f64 = 1.0;
        (noise_metadata_schedule_12_0_e450,)
    } else {
        (w[166],)
    }
};
            w[166] = noise_metadata_schedule_12_0_e452;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_13_0_e454: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            w[177] = noise_metadata_schedule_13_0_e454;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_14_0_e461,) = {
    if ((w[172] != 0.0) && (w[177] != 0.0)) {
        let noise_metadata_schedule_14_0_e459: f64 = 1.0;
        (noise_metadata_schedule_14_0_e459,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_14_0_e461;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_15_0_e463: f64 = if self.param_given[4] { 1.0 } else { 0.0 };
            w[178] = noise_metadata_schedule_15_0_e463;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_16_0_e473,) = {
    if (((w[172] != 0.0) && (w[177] == 0.0)) && (w[178] != 0.0)) {
        let noise_metadata_schedule_16_0_e471: f64 = (-1.0);
        (noise_metadata_schedule_16_0_e471,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_16_0_e473;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_17_0_e475: f64 = if self.param_given[5] { 1.0 } else { 0.0 };
            w[179] = noise_metadata_schedule_17_0_e475;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_18_0_e487,) = {
    if ((((w[172] != 0.0) && (w[177] == 0.0)) && (w[178] == 0.0)) && (w[179] != 0.0)) {
        (params[5],)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_18_0_e487;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_19_0_e501,) = {
    if ((((w[172] != 0.0) && (w[177] == 0.0)) && (w[178] == 0.0)) && (w[179] == 0.0)) {
        let noise_metadata_schedule_19_0_e499: f64 = 1.0;
        (noise_metadata_schedule_19_0_e499,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_19_0_e501;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_20_0_e506,) = {
    if (w[172] != 0.0) {
        let noise_metadata_schedule_20_0_e504: f64 = (params[12]).ln();
        (noise_metadata_schedule_20_0_e504,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_20_0_e506;
        }
        if (active[0] & 0x1410) != 0 {
            let (noise_metadata_schedule_21_0_e517,) = {
    if (w[172] != 0.0) {
        let (noise_metadata_schedule_21_0_e515,) = {
            if (params[74] > 0.0) {
                let noise_metadata_schedule_21_0_e513: f64 = (1.0 / params[74]);
                (noise_metadata_schedule_21_0_e513,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_21_0_e515,)
    } else {
        (w[46],)
    }
};
            w[46] = noise_metadata_schedule_21_0_e517;
        }
        if (active[0] & 0x116f) != 0 {
            let (noise_metadata_schedule_22_0_e528,) = {
    if (w[172] != 0.0) {
        let (noise_metadata_schedule_22_0_e526,) = {
            if (params[75] > 0.0) {
                let noise_metadata_schedule_22_0_e524: f64 = (1.0 / params[75]);
                (noise_metadata_schedule_22_0_e524,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_22_0_e526,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_22_0_e528;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_23_0_e539,) = {
    if (w[172] != 0.0) {
        let (noise_metadata_schedule_23_0_e537,) = {
            if (params[20] > 0.0) {
                let noise_metadata_schedule_23_0_e535: f64 = (1.0 / params[20]);
                (noise_metadata_schedule_23_0_e535,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_23_0_e537,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_23_0_e539;
        }
        if (active[0] & 0x1fff) != 0 {
            let (noise_metadata_schedule_27_0_e576,) = {
    if (w[172] != 0.0) {
        let noise_metadata_schedule_27_0_e574: f64 = (273.15 + params[13]);
        (noise_metadata_schedule_27_0_e574,)
    } else {
        (w[40],)
    }
};
            w[40] = noise_metadata_schedule_27_0_e576;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_29_0_e578: f64 = ctx.temperature();
            let noise_metadata_schedule_29_0_e580: f64 = (noise_metadata_schedule_29_0_e578 + params[0]);
            let noise_metadata_schedule_29_0_e582: f64 = (noise_metadata_schedule_29_0_e580 - 273.15);
            w[38] = noise_metadata_schedule_29_0_e582;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_32_0_e592: f64 = (params[14] + 1.0);
            let noise_metadata_schedule_32_0_e593: f64 = if w[38] < noise_metadata_schedule_32_0_e592 { 1.0 } else { 0.0 };
            w[182] = noise_metadata_schedule_32_0_e593;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_33_0_e604,) = {
    if (w[182] != 0.0) {
        let noise_metadata_schedule_33_0_e598: f64 = (w[38] - params[14]);
        let noise_metadata_schedule_33_0_e600: f64 = (noise_metadata_schedule_33_0_e598 - 1.0);
        let noise_metadata_schedule_33_0_e601: f64 = (noise_metadata_schedule_33_0_e600).exp();
        let noise_metadata_schedule_33_0_e602: f64 = (params[14] + noise_metadata_schedule_33_0_e601);
        (noise_metadata_schedule_33_0_e602,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_33_0_e604;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_34_0_e608: f64 = (params[15] - 1.0);
            let noise_metadata_schedule_34_0_e609: f64 = if w[38] > noise_metadata_schedule_34_0_e608 { 1.0 } else { 0.0 };
            w[183] = noise_metadata_schedule_34_0_e609;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_35_0_e623,) = {
    if ((w[182] == 0.0) && (w[183] != 0.0)) {
        let noise_metadata_schedule_35_0_e617: f64 = (params[15] - w[38]);
        let noise_metadata_schedule_35_0_e619: f64 = (noise_metadata_schedule_35_0_e617 - 1.0);
        let noise_metadata_schedule_35_0_e620: f64 = (noise_metadata_schedule_35_0_e619).exp();
        let noise_metadata_schedule_35_0_e621: f64 = (params[15] - noise_metadata_schedule_35_0_e620);
        (noise_metadata_schedule_35_0_e621,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_35_0_e623;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_36_0_e631,) = {
    if ((w[182] == 0.0) && (w[183] == 0.0)) {
        (w[38],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_36_0_e631;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_37_0_e634: f64 = (w[38] + 273.15);
            w[39] = noise_metadata_schedule_37_0_e634;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_38_0_e637: f64 = (1.380662e-23 * w[39]);
            let noise_metadata_schedule_38_0_e639: f64 = (noise_metadata_schedule_38_0_e637 / 1.602189e-19);
            w[73] = noise_metadata_schedule_38_0_e639;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_39_0_e642: f64 = (w[39] / w[40]);
            w[41] = noise_metadata_schedule_39_0_e642;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_41_0_e655: f64 = if params[90] > 0.0 { 1.0 } else { 0.0 };
            w[184] = noise_metadata_schedule_41_0_e655;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_42_0_e674,) = {
    if (w[184] != 0.0) {
        let noise_metadata_schedule_42_0_e659: f64 = (params[89] * w[73]);
        let noise_metadata_schedule_42_0_e661: f64 = (-params[88]);
        let noise_metadata_schedule_42_0_e664: f64 = (params[89] * w[73]);
        let noise_metadata_schedule_42_0_e665: f64 = (noise_metadata_schedule_42_0_e661 / noise_metadata_schedule_42_0_e664);
        let noise_metadata_schedule_42_0_e666: f64 = (noise_metadata_schedule_42_0_e665).exp();
        let noise_metadata_schedule_42_0_e669: f64 = (w[166] / params[90]);
        let noise_metadata_schedule_42_0_e670: f64 = (noise_metadata_schedule_42_0_e666 + noise_metadata_schedule_42_0_e669);
        let noise_metadata_schedule_42_0_e671: f64 = (noise_metadata_schedule_42_0_e670).ln();
        let noise_metadata_schedule_42_0_e672: f64 = (noise_metadata_schedule_42_0_e659 * noise_metadata_schedule_42_0_e671);
        (noise_metadata_schedule_42_0_e672,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_42_0_e674;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_43_0_e679,) = {
    if (w[184] == 0.0) {
        (0.0,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_43_0_e679;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_44_0_e684: f64 = (params[122] / params[28]);
            let noise_metadata_schedule_44_0_e685: f64 = (w[41]).powf(noise_metadata_schedule_44_0_e684);
            let noise_metadata_schedule_44_0_e686: f64 = (params[26] * noise_metadata_schedule_44_0_e685);
            let noise_metadata_schedule_44_0_e688: f64 = (-params[113]);
            let noise_metadata_schedule_44_0_e691: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_44_0_e692: f64 = (noise_metadata_schedule_44_0_e688 * noise_metadata_schedule_44_0_e691);
            let noise_metadata_schedule_44_0_e695: f64 = (w[73] * params[28]);
            let noise_metadata_schedule_44_0_e696: f64 = (noise_metadata_schedule_44_0_e692 / noise_metadata_schedule_44_0_e695);
            let noise_metadata_schedule_44_0_e697: f64 = (noise_metadata_schedule_44_0_e696).exp();
            let noise_metadata_schedule_44_0_e698: f64 = (noise_metadata_schedule_44_0_e686 * noise_metadata_schedule_44_0_e697);
            w[0] = noise_metadata_schedule_44_0_e698;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_45_0_e701: f64 = if w[0] > 0.0 { 1.0 } else { 0.0 };
            w[185] = noise_metadata_schedule_45_0_e701;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_46_0_e708: f64 = if ((params[72] > 0.0) && (w[166] > params[72])) { 1.0 } else { 0.0 };
            w[186] = noise_metadata_schedule_46_0_e708;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_47_0_e737,) = {
    if ((w[185] != 0.0) && (w[186] != 0.0)) {
        let noise_metadata_schedule_47_0_e714: f64 = (params[28] * w[73]);
        let noise_metadata_schedule_47_0_e718: f64 = (0.5 * w[166]);
        let noise_metadata_schedule_47_0_e721: f64 = (4.0 / params[72]);
        let noise_metadata_schedule_47_0_e723: f64 = (noise_metadata_schedule_47_0_e721).powf(params[73]);
        let noise_metadata_schedule_47_0_e724: f64 = (noise_metadata_schedule_47_0_e718 * noise_metadata_schedule_47_0_e723);
        let noise_metadata_schedule_47_0_e728: f64 = (1.0 - params[73]);
        let noise_metadata_schedule_47_0_e729: f64 = (1.0 / noise_metadata_schedule_47_0_e728);
        let noise_metadata_schedule_47_0_e730: f64 = (noise_metadata_schedule_47_0_e724).powf(noise_metadata_schedule_47_0_e729);
        let noise_metadata_schedule_47_0_e732: f64 = (noise_metadata_schedule_47_0_e730 / w[0]);
        let noise_metadata_schedule_47_0_e733: f64 = (1.0 + noise_metadata_schedule_47_0_e732);
        let noise_metadata_schedule_47_0_e734: f64 = (noise_metadata_schedule_47_0_e733).ln();
        let noise_metadata_schedule_47_0_e735: f64 = (noise_metadata_schedule_47_0_e714 * noise_metadata_schedule_47_0_e734);
        (noise_metadata_schedule_47_0_e735,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_47_0_e737;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_48_0_e753,) = {
    if ((w[185] != 0.0) && (w[186] == 0.0)) {
        let noise_metadata_schedule_48_0_e744: f64 = (params[28] * w[73]);
        let noise_metadata_schedule_48_0_e748: f64 = (w[166] / w[0]);
        let noise_metadata_schedule_48_0_e749: f64 = (1.0 + noise_metadata_schedule_48_0_e748);
        let noise_metadata_schedule_48_0_e750: f64 = (noise_metadata_schedule_48_0_e749).ln();
        let noise_metadata_schedule_48_0_e751: f64 = (noise_metadata_schedule_48_0_e744 * noise_metadata_schedule_48_0_e750);
        (noise_metadata_schedule_48_0_e751,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_48_0_e753;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_49_0_e758,) = {
    if (w[185] == 0.0) {
        (0.0,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_49_0_e758;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_50_0_e763: f64 = (params[125] / params[29]);
            let noise_metadata_schedule_50_0_e764: f64 = (w[41]).powf(noise_metadata_schedule_50_0_e763);
            let noise_metadata_schedule_50_0_e765: f64 = (params[27] * noise_metadata_schedule_50_0_e764);
            let noise_metadata_schedule_50_0_e767: f64 = (-params[121]);
            let noise_metadata_schedule_50_0_e770: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_50_0_e771: f64 = (noise_metadata_schedule_50_0_e767 * noise_metadata_schedule_50_0_e770);
            let noise_metadata_schedule_50_0_e774: f64 = (w[73] * params[29]);
            let noise_metadata_schedule_50_0_e775: f64 = (noise_metadata_schedule_50_0_e771 / noise_metadata_schedule_50_0_e774);
            let noise_metadata_schedule_50_0_e776: f64 = (noise_metadata_schedule_50_0_e775).exp();
            let noise_metadata_schedule_50_0_e777: f64 = (noise_metadata_schedule_50_0_e765 * noise_metadata_schedule_50_0_e776);
            w[1] = noise_metadata_schedule_50_0_e777;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_51_0_e784: f64 = if ((w[0] > 0.0) && (w[1] > 0.0)) { 1.0 } else { 0.0 };
            w[187] = noise_metadata_schedule_51_0_e784;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_52_0_e791: f64 = if ((params[74] > 0.0) && (w[166] > params[74])) { 1.0 } else { 0.0 };
            w[188] = noise_metadata_schedule_52_0_e791;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_53_0_e822,) = {
    if ((w[187] != 0.0) && (w[188] != 0.0)) {
        let noise_metadata_schedule_53_0_e797: f64 = (params[29] * w[73]);
        let noise_metadata_schedule_53_0_e801: f64 = (0.5 * w[166]);
        let noise_metadata_schedule_53_0_e804: f64 = (4.0 / params[74]);
        let noise_metadata_schedule_53_0_e806: f64 = (noise_metadata_schedule_53_0_e804).powf(params[73]);
        let noise_metadata_schedule_53_0_e807: f64 = (noise_metadata_schedule_53_0_e801 * noise_metadata_schedule_53_0_e806);
        let noise_metadata_schedule_53_0_e811: f64 = (1.0 - params[73]);
        let noise_metadata_schedule_53_0_e812: f64 = (1.0 / noise_metadata_schedule_53_0_e811);
        let noise_metadata_schedule_53_0_e813: f64 = (noise_metadata_schedule_53_0_e807).powf(noise_metadata_schedule_53_0_e812);
        let noise_metadata_schedule_53_0_e816: f64 = (w[0] * w[1]);
        let noise_metadata_schedule_53_0_e817: f64 = (noise_metadata_schedule_53_0_e813 / noise_metadata_schedule_53_0_e816);
        let noise_metadata_schedule_53_0_e818: f64 = (1.0 + noise_metadata_schedule_53_0_e817);
        let noise_metadata_schedule_53_0_e819: f64 = (noise_metadata_schedule_53_0_e818).ln();
        let noise_metadata_schedule_53_0_e820: f64 = (noise_metadata_schedule_53_0_e797 * noise_metadata_schedule_53_0_e819);
        (noise_metadata_schedule_53_0_e820,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_53_0_e822;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_54_0_e840,) = {
    if ((w[187] != 0.0) && (w[188] == 0.0)) {
        let noise_metadata_schedule_54_0_e829: f64 = (params[29] * w[73]);
        let noise_metadata_schedule_54_0_e834: f64 = (w[0] * w[1]);
        let noise_metadata_schedule_54_0_e835: f64 = (w[166] / noise_metadata_schedule_54_0_e834);
        let noise_metadata_schedule_54_0_e836: f64 = (1.0 + noise_metadata_schedule_54_0_e835);
        let noise_metadata_schedule_54_0_e837: f64 = (noise_metadata_schedule_54_0_e836).ln();
        let noise_metadata_schedule_54_0_e838: f64 = (noise_metadata_schedule_54_0_e829 * noise_metadata_schedule_54_0_e837);
        (noise_metadata_schedule_54_0_e838,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_54_0_e840;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_55_0_e845,) = {
    if (w[187] == 0.0) {
        (0.0,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_55_0_e845;
        }
        if (active[0] & 0x116f) != 0 {
            let noise_metadata_schedule_56_0_e850: f64 = (params[122] / params[33]);
            let noise_metadata_schedule_56_0_e851: f64 = (w[41]).powf(noise_metadata_schedule_56_0_e850);
            let noise_metadata_schedule_56_0_e852: f64 = (params[31] * noise_metadata_schedule_56_0_e851);
            let noise_metadata_schedule_56_0_e854: f64 = (-params[120]);
            let noise_metadata_schedule_56_0_e857: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_56_0_e858: f64 = (noise_metadata_schedule_56_0_e854 * noise_metadata_schedule_56_0_e857);
            let noise_metadata_schedule_56_0_e861: f64 = (w[73] * params[33]);
            let noise_metadata_schedule_56_0_e862: f64 = (noise_metadata_schedule_56_0_e858 / noise_metadata_schedule_56_0_e861);
            let noise_metadata_schedule_56_0_e863: f64 = (noise_metadata_schedule_56_0_e862).exp();
            let noise_metadata_schedule_56_0_e864: f64 = (noise_metadata_schedule_56_0_e852 * noise_metadata_schedule_56_0_e863);
            w[5] = noise_metadata_schedule_56_0_e864;
        }
        if (active[0] & 0x116f) != 0 {
            let noise_metadata_schedule_57_0_e867: f64 = if w[5] > 0.0 { 1.0 } else { 0.0 };
            w[189] = noise_metadata_schedule_57_0_e867;
        }
        if (active[0] & 0x116f) != 0 {
            let noise_metadata_schedule_58_0_e874: f64 = if ((params[75] > 0.0) && (w[166] > params[75])) { 1.0 } else { 0.0 };
            w[190] = noise_metadata_schedule_58_0_e874;
        }
        if (active[0] & 0x116f) != 0 {
            let (noise_metadata_schedule_59_0_e893,) = {
    if ((w[189] != 0.0) && (w[190] != 0.0)) {
        let noise_metadata_schedule_59_0_e880: f64 = (params[33] * w[73]);
        let noise_metadata_schedule_59_0_e884: f64 = (w[166] * w[166]);
        let noise_metadata_schedule_59_0_e886: f64 = (noise_metadata_schedule_59_0_e884 * w[47]);
        let noise_metadata_schedule_59_0_e888: f64 = (noise_metadata_schedule_59_0_e886 / w[5]);
        let noise_metadata_schedule_59_0_e889: f64 = (1.0 + noise_metadata_schedule_59_0_e888);
        let noise_metadata_schedule_59_0_e890: f64 = (noise_metadata_schedule_59_0_e889).ln();
        let noise_metadata_schedule_59_0_e891: f64 = (noise_metadata_schedule_59_0_e880 * noise_metadata_schedule_59_0_e890);
        (noise_metadata_schedule_59_0_e891,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_59_0_e893;
        }
        if (active[0] & 0x116f) != 0 {
            let (noise_metadata_schedule_60_0_e909,) = {
    if ((w[189] != 0.0) && (w[190] == 0.0)) {
        let noise_metadata_schedule_60_0_e900: f64 = (params[33] * w[73]);
        let noise_metadata_schedule_60_0_e904: f64 = (w[166] / w[5]);
        let noise_metadata_schedule_60_0_e905: f64 = (1.0 + noise_metadata_schedule_60_0_e904);
        let noise_metadata_schedule_60_0_e906: f64 = (noise_metadata_schedule_60_0_e905).ln();
        let noise_metadata_schedule_60_0_e907: f64 = (noise_metadata_schedule_60_0_e900 * noise_metadata_schedule_60_0_e906);
        (noise_metadata_schedule_60_0_e907,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_60_0_e909;
        }
        if (active[0] & 0x116f) != 0 {
            let (noise_metadata_schedule_61_0_e914,) = {
    if (w[189] == 0.0) {
        (0.0,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_61_0_e914;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_62_0_e919: f64 = (params[123] / params[56]);
            let noise_metadata_schedule_62_0_e920: f64 = (w[41]).powf(noise_metadata_schedule_62_0_e919);
            let noise_metadata_schedule_62_0_e921: f64 = (params[54] * noise_metadata_schedule_62_0_e920);
            let noise_metadata_schedule_62_0_e923: f64 = (-params[114]);
            let noise_metadata_schedule_62_0_e926: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_62_0_e927: f64 = (noise_metadata_schedule_62_0_e923 * noise_metadata_schedule_62_0_e926);
            let noise_metadata_schedule_62_0_e930: f64 = (w[73] * params[56]);
            let noise_metadata_schedule_62_0_e931: f64 = (noise_metadata_schedule_62_0_e927 / noise_metadata_schedule_62_0_e930);
            let noise_metadata_schedule_62_0_e932: f64 = (noise_metadata_schedule_62_0_e931).exp();
            let noise_metadata_schedule_62_0_e933: f64 = (noise_metadata_schedule_62_0_e921 * noise_metadata_schedule_62_0_e932);
            w[3] = noise_metadata_schedule_62_0_e933;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_63_0_e936: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[191] = noise_metadata_schedule_63_0_e936;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_64_0_e949,) = {
    if (w[191] != 0.0) {
        let noise_metadata_schedule_64_0_e940: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_64_0_e944: f64 = (w[166] / w[3]);
        let noise_metadata_schedule_64_0_e945: f64 = (1.0 + noise_metadata_schedule_64_0_e944);
        let noise_metadata_schedule_64_0_e946: f64 = (noise_metadata_schedule_64_0_e945).ln();
        let noise_metadata_schedule_64_0_e947: f64 = (noise_metadata_schedule_64_0_e940 * noise_metadata_schedule_64_0_e946);
        (noise_metadata_schedule_64_0_e947,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_64_0_e949;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_65_0_e954,) = {
    if (w[191] == 0.0) {
        (0.0,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_65_0_e954;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_66_0_e959: f64 = (params[124] / params[59]);
            let noise_metadata_schedule_66_0_e960: f64 = (w[41]).powf(noise_metadata_schedule_66_0_e959);
            let noise_metadata_schedule_66_0_e961: f64 = (params[58] * noise_metadata_schedule_66_0_e960);
            let noise_metadata_schedule_66_0_e963: f64 = (-params[117]);
            let noise_metadata_schedule_66_0_e966: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_66_0_e967: f64 = (noise_metadata_schedule_66_0_e963 * noise_metadata_schedule_66_0_e966);
            let noise_metadata_schedule_66_0_e970: f64 = (w[73] * params[59]);
            let noise_metadata_schedule_66_0_e971: f64 = (noise_metadata_schedule_66_0_e967 / noise_metadata_schedule_66_0_e970);
            let noise_metadata_schedule_66_0_e972: f64 = (noise_metadata_schedule_66_0_e971).exp();
            let noise_metadata_schedule_66_0_e973: f64 = (noise_metadata_schedule_66_0_e961 * noise_metadata_schedule_66_0_e972);
            w[6] = noise_metadata_schedule_66_0_e973;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_67_0_e976: f64 = if w[6] > 0.0 { 1.0 } else { 0.0 };
            w[192] = noise_metadata_schedule_67_0_e976;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_68_0_e989,) = {
    if (w[192] != 0.0) {
        let noise_metadata_schedule_68_0_e980: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_68_0_e984: f64 = (w[166] / w[6]);
        let noise_metadata_schedule_68_0_e985: f64 = (1.0 + noise_metadata_schedule_68_0_e984);
        let noise_metadata_schedule_68_0_e986: f64 = (noise_metadata_schedule_68_0_e985).ln();
        let noise_metadata_schedule_68_0_e987: f64 = (noise_metadata_schedule_68_0_e980 * noise_metadata_schedule_68_0_e986);
        (noise_metadata_schedule_68_0_e987,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_68_0_e989;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_69_0_e994,) = {
    if (w[192] == 0.0) {
        (0.0,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_69_0_e994;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_70_0_e999: f64 = (params[123] / params[61]);
            let noise_metadata_schedule_70_0_e1000: f64 = (w[41]).powf(noise_metadata_schedule_70_0_e999);
            let noise_metadata_schedule_70_0_e1001: f64 = (params[60] * noise_metadata_schedule_70_0_e1000);
            let noise_metadata_schedule_70_0_e1003: f64 = (-params[115]);
            let noise_metadata_schedule_70_0_e1006: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_70_0_e1007: f64 = (noise_metadata_schedule_70_0_e1003 * noise_metadata_schedule_70_0_e1006);
            let noise_metadata_schedule_70_0_e1010: f64 = (w[73] * params[61]);
            let noise_metadata_schedule_70_0_e1011: f64 = (noise_metadata_schedule_70_0_e1007 / noise_metadata_schedule_70_0_e1010);
            let noise_metadata_schedule_70_0_e1012: f64 = (noise_metadata_schedule_70_0_e1011).exp();
            let noise_metadata_schedule_70_0_e1013: f64 = (noise_metadata_schedule_70_0_e1001 * noise_metadata_schedule_70_0_e1012);
            w[4] = noise_metadata_schedule_70_0_e1013;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_71_0_e1016: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[193] = noise_metadata_schedule_71_0_e1016;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_72_0_e1029,) = {
    if (w[193] != 0.0) {
        let noise_metadata_schedule_72_0_e1020: f64 = (params[61] * w[73]);
        let noise_metadata_schedule_72_0_e1024: f64 = (w[166] / w[4]);
        let noise_metadata_schedule_72_0_e1025: f64 = (1.0 + noise_metadata_schedule_72_0_e1024);
        let noise_metadata_schedule_72_0_e1026: f64 = (noise_metadata_schedule_72_0_e1025).ln();
        let noise_metadata_schedule_72_0_e1027: f64 = (noise_metadata_schedule_72_0_e1020 * noise_metadata_schedule_72_0_e1026);
        (noise_metadata_schedule_72_0_e1027,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_72_0_e1029;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_73_0_e1034,) = {
    if (w[193] == 0.0) {
        (0.0,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_73_0_e1034;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_74_0_e1039: f64 = (params[124] / params[63]);
            let noise_metadata_schedule_74_0_e1040: f64 = (w[41]).powf(noise_metadata_schedule_74_0_e1039);
            let noise_metadata_schedule_74_0_e1041: f64 = (params[62] * noise_metadata_schedule_74_0_e1040);
            let noise_metadata_schedule_74_0_e1043: f64 = (-params[118]);
            let noise_metadata_schedule_74_0_e1046: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_74_0_e1047: f64 = (noise_metadata_schedule_74_0_e1043 * noise_metadata_schedule_74_0_e1046);
            let noise_metadata_schedule_74_0_e1050: f64 = (w[73] * params[63]);
            let noise_metadata_schedule_74_0_e1051: f64 = (noise_metadata_schedule_74_0_e1047 / noise_metadata_schedule_74_0_e1050);
            let noise_metadata_schedule_74_0_e1052: f64 = (noise_metadata_schedule_74_0_e1051).exp();
            let noise_metadata_schedule_74_0_e1053: f64 = (noise_metadata_schedule_74_0_e1041 * noise_metadata_schedule_74_0_e1052);
            w[7] = noise_metadata_schedule_74_0_e1053;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_75_0_e1056: f64 = if w[7] > 0.0 { 1.0 } else { 0.0 };
            w[194] = noise_metadata_schedule_75_0_e1056;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_76_0_e1069,) = {
    if (w[194] != 0.0) {
        let noise_metadata_schedule_76_0_e1060: f64 = (params[63] * w[73]);
        let noise_metadata_schedule_76_0_e1064: f64 = (w[166] / w[7]);
        let noise_metadata_schedule_76_0_e1065: f64 = (1.0 + noise_metadata_schedule_76_0_e1064);
        let noise_metadata_schedule_76_0_e1066: f64 = (noise_metadata_schedule_76_0_e1065).ln();
        let noise_metadata_schedule_76_0_e1067: f64 = (noise_metadata_schedule_76_0_e1060 * noise_metadata_schedule_76_0_e1066);
        (noise_metadata_schedule_76_0_e1067,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_76_0_e1069;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_77_0_e1074,) = {
    if (w[194] == 0.0) {
        (0.0,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_77_0_e1074;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_78_0_e1079: f64 = (params[123] / params[61]);
            let noise_metadata_schedule_78_0_e1080: f64 = (w[41]).powf(noise_metadata_schedule_78_0_e1079);
            let noise_metadata_schedule_78_0_e1081: f64 = (params[64] * noise_metadata_schedule_78_0_e1080);
            let noise_metadata_schedule_78_0_e1083: f64 = (-params[115]);
            let noise_metadata_schedule_78_0_e1086: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_78_0_e1087: f64 = (noise_metadata_schedule_78_0_e1083 * noise_metadata_schedule_78_0_e1086);
            let noise_metadata_schedule_78_0_e1090: f64 = (w[73] * params[61]);
            let noise_metadata_schedule_78_0_e1091: f64 = (noise_metadata_schedule_78_0_e1087 / noise_metadata_schedule_78_0_e1090);
            let noise_metadata_schedule_78_0_e1092: f64 = (noise_metadata_schedule_78_0_e1091).exp();
            let noise_metadata_schedule_78_0_e1093: f64 = (noise_metadata_schedule_78_0_e1081 * noise_metadata_schedule_78_0_e1092);
            w[8] = noise_metadata_schedule_78_0_e1093;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_79_0_e1096: f64 = if w[8] > 0.0 { 1.0 } else { 0.0 };
            w[195] = noise_metadata_schedule_79_0_e1096;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_80_0_e1109,) = {
    if (w[195] != 0.0) {
        let noise_metadata_schedule_80_0_e1100: f64 = (params[61] * w[73]);
        let noise_metadata_schedule_80_0_e1104: f64 = (w[166] / w[8]);
        let noise_metadata_schedule_80_0_e1105: f64 = (1.0 + noise_metadata_schedule_80_0_e1104);
        let noise_metadata_schedule_80_0_e1106: f64 = (noise_metadata_schedule_80_0_e1105).ln();
        let noise_metadata_schedule_80_0_e1107: f64 = (noise_metadata_schedule_80_0_e1100 * noise_metadata_schedule_80_0_e1106);
        (noise_metadata_schedule_80_0_e1107,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_80_0_e1109;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_81_0_e1114,) = {
    if (w[195] == 0.0) {
        (0.0,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_81_0_e1114;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_82_0_e1119: f64 = (params[124] / params[63]);
            let noise_metadata_schedule_82_0_e1120: f64 = (w[41]).powf(noise_metadata_schedule_82_0_e1119);
            let noise_metadata_schedule_82_0_e1121: f64 = (params[65] * noise_metadata_schedule_82_0_e1120);
            let noise_metadata_schedule_82_0_e1123: f64 = (-params[118]);
            let noise_metadata_schedule_82_0_e1126: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_82_0_e1127: f64 = (noise_metadata_schedule_82_0_e1123 * noise_metadata_schedule_82_0_e1126);
            let noise_metadata_schedule_82_0_e1130: f64 = (w[73] * params[63]);
            let noise_metadata_schedule_82_0_e1131: f64 = (noise_metadata_schedule_82_0_e1127 / noise_metadata_schedule_82_0_e1130);
            let noise_metadata_schedule_82_0_e1132: f64 = (noise_metadata_schedule_82_0_e1131).exp();
            let noise_metadata_schedule_82_0_e1133: f64 = (noise_metadata_schedule_82_0_e1121 * noise_metadata_schedule_82_0_e1132);
            w[9] = noise_metadata_schedule_82_0_e1133;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_83_0_e1136: f64 = if w[9] > 0.0 { 1.0 } else { 0.0 };
            w[196] = noise_metadata_schedule_83_0_e1136;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_84_0_e1149,) = {
    if (w[196] != 0.0) {
        let noise_metadata_schedule_84_0_e1140: f64 = (params[63] * w[73]);
        let noise_metadata_schedule_84_0_e1144: f64 = (w[166] / w[9]);
        let noise_metadata_schedule_84_0_e1145: f64 = (1.0 + noise_metadata_schedule_84_0_e1144);
        let noise_metadata_schedule_84_0_e1146: f64 = (noise_metadata_schedule_84_0_e1145).ln();
        let noise_metadata_schedule_84_0_e1147: f64 = (noise_metadata_schedule_84_0_e1140 * noise_metadata_schedule_84_0_e1146);
        (noise_metadata_schedule_84_0_e1147,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_84_0_e1149;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_85_0_e1154,) = {
    if (w[196] == 0.0) {
        (0.0,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_85_0_e1154;
        }
        if (active[0] & 0x1fff) != 0 {
            w[138] = (ctx.node_voltage(self.nodes[3]) - 0.0);
        }
        if (active[0] & 0x1fff) != 0 {
            let noise_metadata_schedule_95_0_e1236: f64 = ctx.temperature();
            let noise_metadata_schedule_95_0_e1238: f64 = (noise_metadata_schedule_95_0_e1236 + params[0]);
            let noise_metadata_schedule_95_0_e1240: f64 = (noise_metadata_schedule_95_0_e1238 + w[138]);
            let noise_metadata_schedule_95_0_e1242: f64 = (noise_metadata_schedule_95_0_e1240 - 273.15);
            w[38] = noise_metadata_schedule_95_0_e1242;
        }
        if (active[0] & 0x1fff) != 0 {
            let noise_metadata_schedule_96_0_e1246: f64 = (params[14] + 1.0);
            let noise_metadata_schedule_96_0_e1247: f64 = if w[38] < noise_metadata_schedule_96_0_e1246 { 1.0 } else { 0.0 };
            w[199] = noise_metadata_schedule_96_0_e1247;
        }
        if (active[0] & 0x1fff) != 0 {
            let (noise_metadata_schedule_97_0_e1258,) = {
    if (w[199] != 0.0) {
        let noise_metadata_schedule_97_0_e1252: f64 = (w[38] - params[14]);
        let noise_metadata_schedule_97_0_e1254: f64 = (noise_metadata_schedule_97_0_e1252 - 1.0);
        let noise_metadata_schedule_97_0_e1255: f64 = (noise_metadata_schedule_97_0_e1254).exp();
        let noise_metadata_schedule_97_0_e1256: f64 = (params[14] + noise_metadata_schedule_97_0_e1255);
        (noise_metadata_schedule_97_0_e1256,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_97_0_e1258;
        }
        if (active[0] & 0x1fff) != 0 {
            let noise_metadata_schedule_98_0_e1262: f64 = (params[15] - 1.0);
            let noise_metadata_schedule_98_0_e1263: f64 = if w[38] > noise_metadata_schedule_98_0_e1262 { 1.0 } else { 0.0 };
            w[200] = noise_metadata_schedule_98_0_e1263;
        }
        if (active[0] & 0x1fff) != 0 {
            let (noise_metadata_schedule_99_0_e1277,) = {
    if ((w[199] == 0.0) && (w[200] != 0.0)) {
        let noise_metadata_schedule_99_0_e1271: f64 = (params[15] - w[38]);
        let noise_metadata_schedule_99_0_e1273: f64 = (noise_metadata_schedule_99_0_e1271 - 1.0);
        let noise_metadata_schedule_99_0_e1274: f64 = (noise_metadata_schedule_99_0_e1273).exp();
        let noise_metadata_schedule_99_0_e1275: f64 = (params[15] - noise_metadata_schedule_99_0_e1274);
        (noise_metadata_schedule_99_0_e1275,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_99_0_e1277;
        }
        if (active[0] & 0x1fff) != 0 {
            let (noise_metadata_schedule_100_0_e1285,) = {
    if ((w[199] == 0.0) && (w[200] == 0.0)) {
        (w[38],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_100_0_e1285;
        }
        if (active[0] & 0x1fff) != 0 {
            let noise_metadata_schedule_101_0_e1288: f64 = (w[38] + 273.15);
            w[39] = noise_metadata_schedule_101_0_e1288;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_102_0_e1291: f64 = (1.380662e-23 * w[39]);
            let noise_metadata_schedule_102_0_e1293: f64 = (noise_metadata_schedule_102_0_e1291 / 1.602189e-19);
            w[73] = noise_metadata_schedule_102_0_e1293;
        }
        if (active[0] & 0x1fff) != 0 {
            let noise_metadata_schedule_103_0_e1296: f64 = (w[39] / w[40]);
            w[41] = noise_metadata_schedule_103_0_e1296;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_104_0_e1299: f64 = (w[39] - w[40]);
            w[42] = noise_metadata_schedule_104_0_e1299;
        }
        if (active[0] & 0x1410) != 0 {
            let noise_metadata_schedule_105_0_e1303: f64 = (w[41]).powf(params[126]);
            let noise_metadata_schedule_105_0_e1304: f64 = (params[72] * noise_metadata_schedule_105_0_e1303);
            w[2] = noise_metadata_schedule_105_0_e1304;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_106_0_e1306: f64 = if self.param_given[109] { 1.0 } else { 0.0 };
            w[201] = noise_metadata_schedule_106_0_e1306;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_107_0_e1314,) = {
    if (w[201] != 0.0) {
        let noise_metadata_schedule_107_0_e1311: f64 = (w[41]).powf(params[109]);
        let noise_metadata_schedule_107_0_e1312: f64 = (params[16] * noise_metadata_schedule_107_0_e1311);
        (noise_metadata_schedule_107_0_e1312,)
    } else {
        (w[12],)
    }
};
            w[12] = noise_metadata_schedule_107_0_e1314;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_108_0_e1323,) = {
    if (w[201] == 0.0) {
        let noise_metadata_schedule_108_0_e1320: f64 = (w[41]).powf(params[107]);
        let noise_metadata_schedule_108_0_e1321: f64 = (params[16] * noise_metadata_schedule_108_0_e1320);
        (noise_metadata_schedule_108_0_e1321,)
    } else {
        (w[12],)
    }
};
            w[12] = noise_metadata_schedule_108_0_e1323;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_109_0_e1325: f64 = if self.param_given[108] { 1.0 } else { 0.0 };
            w[202] = noise_metadata_schedule_109_0_e1325;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_110_0_e1333,) = {
    if (w[202] != 0.0) {
        let noise_metadata_schedule_110_0_e1330: f64 = (w[41]).powf(params[108]);
        let noise_metadata_schedule_110_0_e1331: f64 = (params[17] * noise_metadata_schedule_110_0_e1330);
        (noise_metadata_schedule_110_0_e1331,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_110_0_e1333;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_111_0_e1342,) = {
    if (w[202] == 0.0) {
        let noise_metadata_schedule_111_0_e1339: f64 = (w[41]).powf(params[107]);
        let noise_metadata_schedule_111_0_e1340: f64 = (params[17] * noise_metadata_schedule_111_0_e1339);
        (noise_metadata_schedule_111_0_e1340,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_111_0_e1342;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_112_0_e1344: f64 = if self.param_given[106] { 1.0 } else { 0.0 };
            w[203] = noise_metadata_schedule_112_0_e1344;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_113_0_e1352,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_113_0_e1349: f64 = (w[41]).powf(params[106]);
        let noise_metadata_schedule_113_0_e1350: f64 = (params[21] * noise_metadata_schedule_113_0_e1349);
        (noise_metadata_schedule_113_0_e1350,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_113_0_e1352;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_114_0_e1361,) = {
    if (w[203] == 0.0) {
        let noise_metadata_schedule_114_0_e1358: f64 = (w[41]).powf(params[104]);
        let noise_metadata_schedule_114_0_e1359: f64 = (params[21] * noise_metadata_schedule_114_0_e1358);
        (noise_metadata_schedule_114_0_e1359,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_114_0_e1361;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_115_0_e1363: f64 = if self.param_given[105] { 1.0 } else { 0.0 };
            w[204] = noise_metadata_schedule_115_0_e1363;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_116_0_e1371,) = {
    if (w[204] != 0.0) {
        let noise_metadata_schedule_116_0_e1368: f64 = (w[41]).powf(params[105]);
        let noise_metadata_schedule_116_0_e1369: f64 = (params[22] * noise_metadata_schedule_116_0_e1368);
        (noise_metadata_schedule_116_0_e1369,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_116_0_e1371;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_117_0_e1380,) = {
    if (w[204] == 0.0) {
        let noise_metadata_schedule_117_0_e1377: f64 = (w[41]).powf(params[104]);
        let noise_metadata_schedule_117_0_e1378: f64 = (params[22] * noise_metadata_schedule_117_0_e1377);
        (noise_metadata_schedule_117_0_e1378,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_117_0_e1380;
        }
        if (active[0] & 0x800) != 0 {
            let noise_metadata_schedule_118_0_e1384: f64 = (w[41]).powf(params[103]);
            let noise_metadata_schedule_118_0_e1385: f64 = (params[23] * noise_metadata_schedule_118_0_e1384);
            w[16] = noise_metadata_schedule_118_0_e1385;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_120_0_e1392: f64 = if self.param_given[110] { 1.0 } else { 0.0 };
            w[205] = noise_metadata_schedule_120_0_e1392;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_121_0_e1400,) = {
    if (w[205] != 0.0) {
        let noise_metadata_schedule_121_0_e1397: f64 = (w[41]).powf(params[110]);
        let noise_metadata_schedule_121_0_e1398: f64 = (params[25] * noise_metadata_schedule_121_0_e1397);
        (noise_metadata_schedule_121_0_e1398,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_121_0_e1400;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_122_0_e1409,) = {
    if (w[205] == 0.0) {
        let noise_metadata_schedule_122_0_e1406: f64 = (w[41]).powf(params[107]);
        let noise_metadata_schedule_122_0_e1407: f64 = (params[25] * noise_metadata_schedule_122_0_e1406);
        (noise_metadata_schedule_122_0_e1407,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_122_0_e1409;
        }
        if (active[0] & 0x1410) != 0 {
            let noise_metadata_schedule_124_0_e1421: f64 = (params[122] / params[28]);
            let noise_metadata_schedule_124_0_e1422: f64 = (w[41]).powf(noise_metadata_schedule_124_0_e1421);
            let noise_metadata_schedule_124_0_e1423: f64 = (params[26] * noise_metadata_schedule_124_0_e1422);
            let noise_metadata_schedule_124_0_e1425: f64 = (-params[113]);
            let noise_metadata_schedule_124_0_e1428: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_124_0_e1429: f64 = (noise_metadata_schedule_124_0_e1425 * noise_metadata_schedule_124_0_e1428);
            let noise_metadata_schedule_124_0_e1432: f64 = (w[73] * params[28]);
            let noise_metadata_schedule_124_0_e1433: f64 = (noise_metadata_schedule_124_0_e1429 / noise_metadata_schedule_124_0_e1432);
            let noise_metadata_schedule_124_0_e1434: f64 = (noise_metadata_schedule_124_0_e1433).exp();
            let noise_metadata_schedule_124_0_e1435: f64 = (noise_metadata_schedule_124_0_e1423 * noise_metadata_schedule_124_0_e1434);
            w[0] = noise_metadata_schedule_124_0_e1435;
        }
        if (active[0] & 0x1410) != 0 {
            let noise_metadata_schedule_125_0_e1440: f64 = (params[125] / params[29]);
            let noise_metadata_schedule_125_0_e1441: f64 = (w[41]).powf(noise_metadata_schedule_125_0_e1440);
            let noise_metadata_schedule_125_0_e1442: f64 = (params[27] * noise_metadata_schedule_125_0_e1441);
            let noise_metadata_schedule_125_0_e1444: f64 = (-params[121]);
            let noise_metadata_schedule_125_0_e1447: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_125_0_e1448: f64 = (noise_metadata_schedule_125_0_e1444 * noise_metadata_schedule_125_0_e1447);
            let noise_metadata_schedule_125_0_e1451: f64 = (w[73] * params[29]);
            let noise_metadata_schedule_125_0_e1452: f64 = (noise_metadata_schedule_125_0_e1448 / noise_metadata_schedule_125_0_e1451);
            let noise_metadata_schedule_125_0_e1453: f64 = (noise_metadata_schedule_125_0_e1452).exp();
            let noise_metadata_schedule_125_0_e1454: f64 = (noise_metadata_schedule_125_0_e1442 * noise_metadata_schedule_125_0_e1453);
            w[1] = noise_metadata_schedule_125_0_e1454;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_126_0_e1459: f64 = (params[122] / params[33]);
            let noise_metadata_schedule_126_0_e1460: f64 = (w[41]).powf(noise_metadata_schedule_126_0_e1459);
            let noise_metadata_schedule_126_0_e1461: f64 = (params[31] * noise_metadata_schedule_126_0_e1460);
            let noise_metadata_schedule_126_0_e1463: f64 = (-params[120]);
            let noise_metadata_schedule_126_0_e1466: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_126_0_e1467: f64 = (noise_metadata_schedule_126_0_e1463 * noise_metadata_schedule_126_0_e1466);
            let noise_metadata_schedule_126_0_e1470: f64 = (w[73] * params[33]);
            let noise_metadata_schedule_126_0_e1471: f64 = (noise_metadata_schedule_126_0_e1467 / noise_metadata_schedule_126_0_e1470);
            let noise_metadata_schedule_126_0_e1472: f64 = (noise_metadata_schedule_126_0_e1471).exp();
            let noise_metadata_schedule_126_0_e1473: f64 = (noise_metadata_schedule_126_0_e1461 * noise_metadata_schedule_126_0_e1472);
            w[5] = noise_metadata_schedule_126_0_e1473;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_127_0_e1478: f64 = (params[123] / params[56]);
            let noise_metadata_schedule_127_0_e1479: f64 = (w[41]).powf(noise_metadata_schedule_127_0_e1478);
            let noise_metadata_schedule_127_0_e1480: f64 = (params[54] * noise_metadata_schedule_127_0_e1479);
            let noise_metadata_schedule_127_0_e1482: f64 = (-params[114]);
            let noise_metadata_schedule_127_0_e1485: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_127_0_e1486: f64 = (noise_metadata_schedule_127_0_e1482 * noise_metadata_schedule_127_0_e1485);
            let noise_metadata_schedule_127_0_e1489: f64 = (w[73] * params[56]);
            let noise_metadata_schedule_127_0_e1490: f64 = (noise_metadata_schedule_127_0_e1486 / noise_metadata_schedule_127_0_e1489);
            let noise_metadata_schedule_127_0_e1491: f64 = (noise_metadata_schedule_127_0_e1490).exp();
            let noise_metadata_schedule_127_0_e1492: f64 = (noise_metadata_schedule_127_0_e1480 * noise_metadata_schedule_127_0_e1491);
            w[3] = noise_metadata_schedule_127_0_e1492;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_128_0_e1497: f64 = (params[124] / params[59]);
            let noise_metadata_schedule_128_0_e1498: f64 = (w[41]).powf(noise_metadata_schedule_128_0_e1497);
            let noise_metadata_schedule_128_0_e1499: f64 = (params[58] * noise_metadata_schedule_128_0_e1498);
            let noise_metadata_schedule_128_0_e1501: f64 = (-params[117]);
            let noise_metadata_schedule_128_0_e1504: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_128_0_e1505: f64 = (noise_metadata_schedule_128_0_e1501 * noise_metadata_schedule_128_0_e1504);
            let noise_metadata_schedule_128_0_e1508: f64 = (w[73] * params[59]);
            let noise_metadata_schedule_128_0_e1509: f64 = (noise_metadata_schedule_128_0_e1505 / noise_metadata_schedule_128_0_e1508);
            let noise_metadata_schedule_128_0_e1510: f64 = (noise_metadata_schedule_128_0_e1509).exp();
            let noise_metadata_schedule_128_0_e1511: f64 = (noise_metadata_schedule_128_0_e1499 * noise_metadata_schedule_128_0_e1510);
            w[6] = noise_metadata_schedule_128_0_e1511;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_131_0_e1554: f64 = (params[123] / params[61]);
            let noise_metadata_schedule_131_0_e1555: f64 = (w[41]).powf(noise_metadata_schedule_131_0_e1554);
            let noise_metadata_schedule_131_0_e1556: f64 = (params[64] * noise_metadata_schedule_131_0_e1555);
            let noise_metadata_schedule_131_0_e1558: f64 = (-params[115]);
            let noise_metadata_schedule_131_0_e1561: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_131_0_e1562: f64 = (noise_metadata_schedule_131_0_e1558 * noise_metadata_schedule_131_0_e1561);
            let noise_metadata_schedule_131_0_e1565: f64 = (w[73] * params[61]);
            let noise_metadata_schedule_131_0_e1566: f64 = (noise_metadata_schedule_131_0_e1562 / noise_metadata_schedule_131_0_e1565);
            let noise_metadata_schedule_131_0_e1567: f64 = (noise_metadata_schedule_131_0_e1566).exp();
            let noise_metadata_schedule_131_0_e1568: f64 = (noise_metadata_schedule_131_0_e1556 * noise_metadata_schedule_131_0_e1567);
            w[8] = noise_metadata_schedule_131_0_e1568;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_132_0_e1573: f64 = (params[124] / params[63]);
            let noise_metadata_schedule_132_0_e1574: f64 = (w[41]).powf(noise_metadata_schedule_132_0_e1573);
            let noise_metadata_schedule_132_0_e1575: f64 = (params[65] * noise_metadata_schedule_132_0_e1574);
            let noise_metadata_schedule_132_0_e1577: f64 = (-params[118]);
            let noise_metadata_schedule_132_0_e1580: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_132_0_e1581: f64 = (noise_metadata_schedule_132_0_e1577 * noise_metadata_schedule_132_0_e1580);
            let noise_metadata_schedule_132_0_e1584: f64 = (w[73] * params[63]);
            let noise_metadata_schedule_132_0_e1585: f64 = (noise_metadata_schedule_132_0_e1581 / noise_metadata_schedule_132_0_e1584);
            let noise_metadata_schedule_132_0_e1586: f64 = (noise_metadata_schedule_132_0_e1585).exp();
            let noise_metadata_schedule_132_0_e1587: f64 = (noise_metadata_schedule_132_0_e1575 * noise_metadata_schedule_132_0_e1586);
            w[9] = noise_metadata_schedule_132_0_e1587;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_135_0_e1630: f64 = (w[42] * params[129]);
            let noise_metadata_schedule_135_0_e1631: f64 = (1.0 + noise_metadata_schedule_135_0_e1630);
            let noise_metadata_schedule_135_0_e1632: f64 = (params[28] * noise_metadata_schedule_135_0_e1631);
            w[27] = noise_metadata_schedule_135_0_e1632;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_136_0_e1637: f64 = (w[42] * params[129]);
            let noise_metadata_schedule_136_0_e1638: f64 = (1.0 + noise_metadata_schedule_136_0_e1637);
            let noise_metadata_schedule_136_0_e1639: f64 = (params[29] * noise_metadata_schedule_136_0_e1638);
            w[28] = noise_metadata_schedule_136_0_e1639;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_139_0_e1660: f64 = (w[42] * params[92]);
            let noise_metadata_schedule_139_0_e1661: f64 = (params[91] + noise_metadata_schedule_139_0_e1660);
            let noise_metadata_schedule_139_0_e1662: f64 = (w[42] * noise_metadata_schedule_139_0_e1661);
            let noise_metadata_schedule_139_0_e1663: f64 = (1.0 + noise_metadata_schedule_139_0_e1662);
            let noise_metadata_schedule_139_0_e1664: f64 = (params[88] * noise_metadata_schedule_139_0_e1663);
            w[31] = noise_metadata_schedule_139_0_e1664;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_140_0_e1669: f64 = (w[42] * params[93]);
            let noise_metadata_schedule_140_0_e1670: f64 = (1.0 + noise_metadata_schedule_140_0_e1669);
            let noise_metadata_schedule_140_0_e1671: f64 = (params[89] * noise_metadata_schedule_140_0_e1670);
            w[32] = noise_metadata_schedule_140_0_e1671;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_141_0_e1675: f64 = (w[73] / w[41]);
            let noise_metadata_schedule_141_0_e1676: f64 = (2.0 * noise_metadata_schedule_141_0_e1675);
            let noise_metadata_schedule_141_0_e1679: f64 = (0.5 * params[37]);
            let noise_metadata_schedule_141_0_e1681: f64 = (noise_metadata_schedule_141_0_e1679 * w[41]);
            let noise_metadata_schedule_141_0_e1683: f64 = (noise_metadata_schedule_141_0_e1681 / w[73]);
            let noise_metadata_schedule_141_0_e1684: f64 = (noise_metadata_schedule_141_0_e1683).exp();
            let noise_metadata_schedule_141_0_e1686: f64 = (-0.5);
            let noise_metadata_schedule_141_0_e1688: f64 = (noise_metadata_schedule_141_0_e1686 * params[37]);
            let noise_metadata_schedule_141_0_e1690: f64 = (noise_metadata_schedule_141_0_e1688 * w[41]);
            let noise_metadata_schedule_141_0_e1692: f64 = (noise_metadata_schedule_141_0_e1690 / w[73]);
            let noise_metadata_schedule_141_0_e1693: f64 = (noise_metadata_schedule_141_0_e1692).exp();
            let noise_metadata_schedule_141_0_e1694: f64 = (noise_metadata_schedule_141_0_e1684 - noise_metadata_schedule_141_0_e1693);
            let noise_metadata_schedule_141_0_e1695: f64 = (noise_metadata_schedule_141_0_e1694).ln();
            let noise_metadata_schedule_141_0_e1696: f64 = (noise_metadata_schedule_141_0_e1676 * noise_metadata_schedule_141_0_e1695);
            w[206] = noise_metadata_schedule_141_0_e1696;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_142_0_e1699: f64 = (w[206] * w[41]);
            let noise_metadata_schedule_142_0_e1702: f64 = (3.0 * w[73]);
            let noise_metadata_schedule_142_0_e1704: f64 = (w[41]).ln();
            let noise_metadata_schedule_142_0_e1705: f64 = (noise_metadata_schedule_142_0_e1702 * noise_metadata_schedule_142_0_e1704);
            let noise_metadata_schedule_142_0_e1706: f64 = (noise_metadata_schedule_142_0_e1699 - noise_metadata_schedule_142_0_e1705);
            let noise_metadata_schedule_142_0_e1710: f64 = (w[41] - 1.0);
            let noise_metadata_schedule_142_0_e1711: f64 = (params[114] * noise_metadata_schedule_142_0_e1710);
            let noise_metadata_schedule_142_0_e1712: f64 = (noise_metadata_schedule_142_0_e1706 - noise_metadata_schedule_142_0_e1711);
            w[207] = noise_metadata_schedule_142_0_e1712;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_143_0_e1716: f64 = (2.0 * w[73]);
            let noise_metadata_schedule_143_0_e1722: f64 = (-w[207]);
            let noise_metadata_schedule_143_0_e1724: f64 = (noise_metadata_schedule_143_0_e1722 / w[73]);
            let noise_metadata_schedule_143_0_e1725: f64 = (noise_metadata_schedule_143_0_e1724).exp();
            let noise_metadata_schedule_143_0_e1726: f64 = (4.0 * noise_metadata_schedule_143_0_e1725);
            let noise_metadata_schedule_143_0_e1727: f64 = (1.0 + noise_metadata_schedule_143_0_e1726);
            let noise_metadata_schedule_143_0_e1728: f64 = (noise_metadata_schedule_143_0_e1727).sqrt();
            let noise_metadata_schedule_143_0_e1729: f64 = (1.0 + noise_metadata_schedule_143_0_e1728);
            let noise_metadata_schedule_143_0_e1730: f64 = (0.5 * noise_metadata_schedule_143_0_e1729);
            let noise_metadata_schedule_143_0_e1731: f64 = (noise_metadata_schedule_143_0_e1730).ln();
            let noise_metadata_schedule_143_0_e1732: f64 = (noise_metadata_schedule_143_0_e1716 * noise_metadata_schedule_143_0_e1731);
            let noise_metadata_schedule_143_0_e1733: f64 = (w[207] + noise_metadata_schedule_143_0_e1732);
            w[20] = noise_metadata_schedule_143_0_e1733;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_144_0_e1737: f64 = (w[73] / w[41]);
            let noise_metadata_schedule_144_0_e1738: f64 = (2.0 * noise_metadata_schedule_144_0_e1737);
            let noise_metadata_schedule_144_0_e1741: f64 = (0.5 * params[42]);
            let noise_metadata_schedule_144_0_e1743: f64 = (noise_metadata_schedule_144_0_e1741 * w[41]);
            let noise_metadata_schedule_144_0_e1745: f64 = (noise_metadata_schedule_144_0_e1743 / w[73]);
            let noise_metadata_schedule_144_0_e1746: f64 = (noise_metadata_schedule_144_0_e1745).exp();
            let noise_metadata_schedule_144_0_e1748: f64 = (-0.5);
            let noise_metadata_schedule_144_0_e1750: f64 = (noise_metadata_schedule_144_0_e1748 * params[42]);
            let noise_metadata_schedule_144_0_e1752: f64 = (noise_metadata_schedule_144_0_e1750 * w[41]);
            let noise_metadata_schedule_144_0_e1754: f64 = (noise_metadata_schedule_144_0_e1752 / w[73]);
            let noise_metadata_schedule_144_0_e1755: f64 = (noise_metadata_schedule_144_0_e1754).exp();
            let noise_metadata_schedule_144_0_e1756: f64 = (noise_metadata_schedule_144_0_e1746 - noise_metadata_schedule_144_0_e1755);
            let noise_metadata_schedule_144_0_e1757: f64 = (noise_metadata_schedule_144_0_e1756).ln();
            let noise_metadata_schedule_144_0_e1758: f64 = (noise_metadata_schedule_144_0_e1738 * noise_metadata_schedule_144_0_e1757);
            w[208] = noise_metadata_schedule_144_0_e1758;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_145_0_e1761: f64 = (w[208] * w[41]);
            let noise_metadata_schedule_145_0_e1764: f64 = (3.0 * w[73]);
            let noise_metadata_schedule_145_0_e1766: f64 = (w[41]).ln();
            let noise_metadata_schedule_145_0_e1767: f64 = (noise_metadata_schedule_145_0_e1764 * noise_metadata_schedule_145_0_e1766);
            let noise_metadata_schedule_145_0_e1768: f64 = (noise_metadata_schedule_145_0_e1761 - noise_metadata_schedule_145_0_e1767);
            let noise_metadata_schedule_145_0_e1772: f64 = (w[41] - 1.0);
            let noise_metadata_schedule_145_0_e1773: f64 = (params[115] * noise_metadata_schedule_145_0_e1772);
            let noise_metadata_schedule_145_0_e1774: f64 = (noise_metadata_schedule_145_0_e1768 - noise_metadata_schedule_145_0_e1773);
            w[209] = noise_metadata_schedule_145_0_e1774;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_146_0_e1778: f64 = (2.0 * w[73]);
            let noise_metadata_schedule_146_0_e1784: f64 = (-w[209]);
            let noise_metadata_schedule_146_0_e1786: f64 = (noise_metadata_schedule_146_0_e1784 / w[73]);
            let noise_metadata_schedule_146_0_e1787: f64 = (noise_metadata_schedule_146_0_e1786).exp();
            let noise_metadata_schedule_146_0_e1788: f64 = (4.0 * noise_metadata_schedule_146_0_e1787);
            let noise_metadata_schedule_146_0_e1789: f64 = (1.0 + noise_metadata_schedule_146_0_e1788);
            let noise_metadata_schedule_146_0_e1790: f64 = (noise_metadata_schedule_146_0_e1789).sqrt();
            let noise_metadata_schedule_146_0_e1791: f64 = (1.0 + noise_metadata_schedule_146_0_e1790);
            let noise_metadata_schedule_146_0_e1792: f64 = (0.5 * noise_metadata_schedule_146_0_e1791);
            let noise_metadata_schedule_146_0_e1793: f64 = (noise_metadata_schedule_146_0_e1792).ln();
            let noise_metadata_schedule_146_0_e1794: f64 = (noise_metadata_schedule_146_0_e1778 * noise_metadata_schedule_146_0_e1793);
            let noise_metadata_schedule_146_0_e1795: f64 = (w[209] + noise_metadata_schedule_146_0_e1794);
            w[21] = noise_metadata_schedule_146_0_e1795;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_154_0_e1889: f64 = (w[41]).powf(params[122]);
            let noise_metadata_schedule_154_0_e1890: f64 = (params[19] * noise_metadata_schedule_154_0_e1889);
            let noise_metadata_schedule_154_0_e1892: f64 = (-params[113]);
            let noise_metadata_schedule_154_0_e1895: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_154_0_e1896: f64 = (noise_metadata_schedule_154_0_e1892 * noise_metadata_schedule_154_0_e1895);
            let noise_metadata_schedule_154_0_e1898: f64 = (noise_metadata_schedule_154_0_e1896 / w[73]);
            let noise_metadata_schedule_154_0_e1899: f64 = (noise_metadata_schedule_154_0_e1898).exp();
            let noise_metadata_schedule_154_0_e1900: f64 = (noise_metadata_schedule_154_0_e1890 * noise_metadata_schedule_154_0_e1899);
            w[33] = noise_metadata_schedule_154_0_e1900;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_155_0_e1904: f64 = (w[41]).powf(params[112]);
            let noise_metadata_schedule_155_0_e1905: f64 = (params[18] * noise_metadata_schedule_155_0_e1904);
            w[34] = noise_metadata_schedule_155_0_e1905;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_156_0_e1907: f64 = (-w[31]);
            let noise_metadata_schedule_156_0_e1910: f64 = (w[32] * w[73]);
            let noise_metadata_schedule_156_0_e1911: f64 = (noise_metadata_schedule_156_0_e1907 / noise_metadata_schedule_156_0_e1910);
            let noise_metadata_schedule_156_0_e1912: f64 = (noise_metadata_schedule_156_0_e1911).exp();
            w[35] = noise_metadata_schedule_156_0_e1912;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_157_0_e1917: f64 = (w[42] * params[130]);
            let noise_metadata_schedule_157_0_e1918: f64 = (1.0 + noise_metadata_schedule_157_0_e1917);
            let noise_metadata_schedule_157_0_e1919: f64 = (params[70] * noise_metadata_schedule_157_0_e1918);
            w[36] = noise_metadata_schedule_157_0_e1919;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_158_0_e1924: f64 = (w[42] * params[131]);
            let noise_metadata_schedule_158_0_e1925: f64 = (1.0 + noise_metadata_schedule_158_0_e1924);
            let noise_metadata_schedule_158_0_e1926: f64 = (params[71] * noise_metadata_schedule_158_0_e1925);
            w[37] = noise_metadata_schedule_158_0_e1926;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_159_0_e1934,) = {
    if (w[12] > 0.001) {
        let noise_metadata_schedule_159_0_e1932: f64 = (1.0 / w[12]);
        (noise_metadata_schedule_159_0_e1932,)
    } else {
        (1000.0,)
    }
};
            w[53] = noise_metadata_schedule_159_0_e1934;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_160_0_e1942,) = {
    if (w[13] > 0.001) {
        let noise_metadata_schedule_160_0_e1940: f64 = (1.0 / w[13]);
        (noise_metadata_schedule_160_0_e1940,)
    } else {
        (1000.0,)
    }
};
            w[54] = noise_metadata_schedule_160_0_e1942;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_161_0_e1950,) = {
    if (w[14] > 0.001) {
        let noise_metadata_schedule_161_0_e1948: f64 = (1.0 / w[14]);
        (noise_metadata_schedule_161_0_e1948,)
    } else {
        (1000.0,)
    }
};
            w[55] = noise_metadata_schedule_161_0_e1950;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_162_0_e1958,) = {
    if (w[15] > 0.001) {
        let noise_metadata_schedule_162_0_e1956: f64 = (1.0 / w[15]);
        (noise_metadata_schedule_162_0_e1956,)
    } else {
        (1000.0,)
    }
};
            w[56] = noise_metadata_schedule_162_0_e1958;
        }
        if (active[0] & 0x800) != 0 {
            let (noise_metadata_schedule_163_0_e1966,) = {
    if (w[16] > 0.001) {
        let noise_metadata_schedule_163_0_e1964: f64 = (1.0 / w[16]);
        (noise_metadata_schedule_163_0_e1964,)
    } else {
        (1000.0,)
    }
};
            w[57] = noise_metadata_schedule_163_0_e1966;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_164_0_e1974,) = {
    if (w[18] > 0.001) {
        let noise_metadata_schedule_164_0_e1972: f64 = (1.0 / w[18]);
        (noise_metadata_schedule_164_0_e1972,)
    } else {
        (1000.0,)
    }
};
            w[58] = noise_metadata_schedule_164_0_e1974;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_167_0_e1998,) = {
    if (w[36] > 0.0) {
        let noise_metadata_schedule_167_0_e1996: f64 = (1.0 / w[36]);
        (noise_metadata_schedule_167_0_e1996,)
    } else {
        (0.0,)
    }
};
            w[43] = noise_metadata_schedule_167_0_e1998;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_168_0_e2006,) = {
    if (w[37] > 0.0) {
        let noise_metadata_schedule_168_0_e2004: f64 = (1.0 / w[37]);
        (noise_metadata_schedule_168_0_e2004,)
    } else {
        (0.0,)
    }
};
            w[44] = noise_metadata_schedule_168_0_e2006;
        }
        if (active[0] & 0x1410) != 0 {
            let (noise_metadata_schedule_169_0_e2014,) = {
    if (w[2] > 0.0) {
        let noise_metadata_schedule_169_0_e2012: f64 = (1.0 / w[2]);
        (noise_metadata_schedule_169_0_e2012,)
    } else {
        (0.0,)
    }
};
            w[45] = noise_metadata_schedule_169_0_e2014;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_170_0_e2022,) = {
    if (w[34] > 0.0) {
        let noise_metadata_schedule_170_0_e2020: f64 = (1.0 / w[34]);
        (noise_metadata_schedule_170_0_e2020,)
    } else {
        (0.0,)
    }
};
            w[48] = noise_metadata_schedule_170_0_e2022;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_171_0_e2025: f64 = (w[162] * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[8])));
            w[143] = noise_metadata_schedule_171_0_e2025;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_172_0_e2028: f64 = (w[162] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[8])));
            w[145] = noise_metadata_schedule_172_0_e2028;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_173_0_e2031: f64 = (w[162] * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[5])));
            w[144] = noise_metadata_schedule_173_0_e2031;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_174_0_e2034: f64 = (w[162] * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[4])));
            w[148] = noise_metadata_schedule_174_0_e2034;
        }
        if (active[0] & 0x116f) != 0 {
            let noise_metadata_schedule_176_0_e2040: f64 = (w[162] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[9])));
            w[146] = noise_metadata_schedule_176_0_e2040;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_181_0_e2049: f64 = (w[162] * (ctx.node_voltage(self.nodes[4]) - ctx.node_voltage(self.nodes[5])));
            w[154] = noise_metadata_schedule_181_0_e2049;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_188_0_e2057: f64 = (-w[20]);
            let noise_metadata_schedule_188_0_e2059: f64 = (noise_metadata_schedule_188_0_e2057 * params[34]);
            w[212] = noise_metadata_schedule_188_0_e2059;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_189_0_e2062: f64 = if params[39] <= 0.0 { 1.0 } else { 0.0 };
            w[223] = noise_metadata_schedule_189_0_e2062;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_190_0_e2068,) = {
    if (w[223] != 0.0) {
        let noise_metadata_schedule_190_0_e2066: f64 = (w[143] + w[212]);
        (noise_metadata_schedule_190_0_e2066,)
    } else {
        (w[213],)
    }
};
            w[213] = noise_metadata_schedule_190_0_e2068;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_191_0_e2071: f64 = if w[213] > 0.0 { 1.0 } else { 0.0 };
            w[224] = noise_metadata_schedule_191_0_e2071;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_192_0_e2082,) = {
    if ((w[223] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_192_0_e2077: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_192_0_e2079: f64 = (-params[38]);
        let noise_metadata_schedule_192_0_e2080: f64 = (noise_metadata_schedule_192_0_e2077).powf(noise_metadata_schedule_192_0_e2079);
        (noise_metadata_schedule_192_0_e2080,)
    } else {
        (w[214],)
    }
};
            w[214] = noise_metadata_schedule_192_0_e2082;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_193_0_e2100,) = {
    if ((w[223] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_193_0_e2091: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_193_0_e2092: f64 = (w[214] * noise_metadata_schedule_193_0_e2091);
        let noise_metadata_schedule_193_0_e2093: f64 = (1.0 - noise_metadata_schedule_193_0_e2092);
        let noise_metadata_schedule_193_0_e2094: f64 = (w[20] * noise_metadata_schedule_193_0_e2093);
        let noise_metadata_schedule_193_0_e2097: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_193_0_e2098: f64 = (noise_metadata_schedule_193_0_e2094 / noise_metadata_schedule_193_0_e2097);
        (noise_metadata_schedule_193_0_e2098,)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_193_0_e2100;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_194_0_e2122,) = {
    if ((w[223] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_194_0_e2108: f64 = (0.5 * params[38]);
        let noise_metadata_schedule_194_0_e2110: f64 = (noise_metadata_schedule_194_0_e2108 * w[213]);
        let noise_metadata_schedule_194_0_e2114: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_194_0_e2115: f64 = (w[20] * noise_metadata_schedule_194_0_e2114);
        let noise_metadata_schedule_194_0_e2116: f64 = (noise_metadata_schedule_194_0_e2110 / noise_metadata_schedule_194_0_e2115);
        let noise_metadata_schedule_194_0_e2117: f64 = (1.0 + noise_metadata_schedule_194_0_e2116);
        let noise_metadata_schedule_194_0_e2118: f64 = (w[213] * noise_metadata_schedule_194_0_e2117);
        let noise_metadata_schedule_194_0_e2120: f64 = (noise_metadata_schedule_194_0_e2118 * w[214]);
        (noise_metadata_schedule_194_0_e2120,)
    } else {
        (w[216],)
    }
};
            w[216] = noise_metadata_schedule_194_0_e2122;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_195_0_e2145,) = {
    if ((w[223] != 0.0) && (w[224] == 0.0)) {
        let noise_metadata_schedule_195_0_e2132: f64 = (w[143] / w[20]);
        let noise_metadata_schedule_195_0_e2133: f64 = (1.0 - noise_metadata_schedule_195_0_e2132);
        let noise_metadata_schedule_195_0_e2136: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_195_0_e2137: f64 = (noise_metadata_schedule_195_0_e2133).powf(noise_metadata_schedule_195_0_e2136);
        let noise_metadata_schedule_195_0_e2138: f64 = (1.0 - noise_metadata_schedule_195_0_e2137);
        let noise_metadata_schedule_195_0_e2139: f64 = (w[20] * noise_metadata_schedule_195_0_e2138);
        let noise_metadata_schedule_195_0_e2142: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_195_0_e2143: f64 = (noise_metadata_schedule_195_0_e2139 / noise_metadata_schedule_195_0_e2142);
        (noise_metadata_schedule_195_0_e2143,)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_195_0_e2145;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_196_0_e2152,) = {
    if ((w[223] != 0.0) && (w[224] == 0.0)) {
        (0.0,)
    } else {
        (w[216],)
    }
};
            w[216] = noise_metadata_schedule_196_0_e2152;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_197_0_e2158,) = {
    if (w[223] != 0.0) {
        let noise_metadata_schedule_197_0_e2156: f64 = (w[215] + w[216]);
        (noise_metadata_schedule_197_0_e2156,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_197_0_e2158;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_198_0_e2172,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_198_0_e2163: f64 = (w[212] * w[212]);
        let noise_metadata_schedule_198_0_e2166: f64 = (4.0 * params[39]);
        let noise_metadata_schedule_198_0_e2168: f64 = (noise_metadata_schedule_198_0_e2166 * params[39]);
        let noise_metadata_schedule_198_0_e2169: f64 = (noise_metadata_schedule_198_0_e2163 + noise_metadata_schedule_198_0_e2168);
        let noise_metadata_schedule_198_0_e2170: f64 = (noise_metadata_schedule_198_0_e2169).sqrt();
        (noise_metadata_schedule_198_0_e2170,)
    } else {
        (w[217],)
    }
};
            w[217] = noise_metadata_schedule_198_0_e2172;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_199_0_e2182,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_199_0_e2176: f64 = (-0.5);
        let noise_metadata_schedule_199_0_e2179: f64 = (w[212] + w[217]);
        let noise_metadata_schedule_199_0_e2180: f64 = (noise_metadata_schedule_199_0_e2176 * noise_metadata_schedule_199_0_e2179);
        (noise_metadata_schedule_199_0_e2180,)
    } else {
        (w[218],)
    }
};
            w[218] = noise_metadata_schedule_199_0_e2182;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_200_0_e2202,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_200_0_e2186: f64 = (-w[20]);
        let noise_metadata_schedule_200_0_e2190: f64 = (w[218] / w[20]);
        let noise_metadata_schedule_200_0_e2191: f64 = (1.0 - noise_metadata_schedule_200_0_e2190);
        let noise_metadata_schedule_200_0_e2194: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_200_0_e2195: f64 = (noise_metadata_schedule_200_0_e2191).powf(noise_metadata_schedule_200_0_e2194);
        let noise_metadata_schedule_200_0_e2196: f64 = (noise_metadata_schedule_200_0_e2186 * noise_metadata_schedule_200_0_e2195);
        let noise_metadata_schedule_200_0_e2199: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_200_0_e2200: f64 = (noise_metadata_schedule_200_0_e2196 / noise_metadata_schedule_200_0_e2199);
        (noise_metadata_schedule_200_0_e2200,)
    } else {
        (w[219],)
    }
};
            w[219] = noise_metadata_schedule_200_0_e2202;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_201_0_e2209,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_201_0_e2207: f64 = (w[143] + w[212]);
        (noise_metadata_schedule_201_0_e2207,)
    } else {
        (w[220],)
    }
};
            w[220] = noise_metadata_schedule_201_0_e2209;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_202_0_e2223,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_202_0_e2214: f64 = (w[220] * w[220]);
        let noise_metadata_schedule_202_0_e2217: f64 = (4.0 * params[39]);
        let noise_metadata_schedule_202_0_e2219: f64 = (noise_metadata_schedule_202_0_e2217 * params[39]);
        let noise_metadata_schedule_202_0_e2220: f64 = (noise_metadata_schedule_202_0_e2214 + noise_metadata_schedule_202_0_e2219);
        let noise_metadata_schedule_202_0_e2221: f64 = (noise_metadata_schedule_202_0_e2220).sqrt();
        (noise_metadata_schedule_202_0_e2221,)
    } else {
        (w[221],)
    }
};
            w[221] = noise_metadata_schedule_202_0_e2223;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_203_0_e2234,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_203_0_e2229: f64 = (w[220] - w[221]);
        let noise_metadata_schedule_203_0_e2230: f64 = (0.5 * noise_metadata_schedule_203_0_e2229);
        let noise_metadata_schedule_203_0_e2232: f64 = (noise_metadata_schedule_203_0_e2230 - w[212]);
        (noise_metadata_schedule_203_0_e2232,)
    } else {
        (w[222],)
    }
};
            w[222] = noise_metadata_schedule_203_0_e2234;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_204_0_e2254,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_204_0_e2238: f64 = (-w[20]);
        let noise_metadata_schedule_204_0_e2242: f64 = (w[222] / w[20]);
        let noise_metadata_schedule_204_0_e2243: f64 = (1.0 - noise_metadata_schedule_204_0_e2242);
        let noise_metadata_schedule_204_0_e2246: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_204_0_e2247: f64 = (noise_metadata_schedule_204_0_e2243).powf(noise_metadata_schedule_204_0_e2246);
        let noise_metadata_schedule_204_0_e2248: f64 = (noise_metadata_schedule_204_0_e2238 * noise_metadata_schedule_204_0_e2247);
        let noise_metadata_schedule_204_0_e2251: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_204_0_e2252: f64 = (noise_metadata_schedule_204_0_e2248 / noise_metadata_schedule_204_0_e2251);
        (noise_metadata_schedule_204_0_e2252,)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_204_0_e2254;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_205_0_e2292,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_205_0_e2260: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_205_0_e2262: f64 = (-params[38]);
        let noise_metadata_schedule_205_0_e2263: f64 = (noise_metadata_schedule_205_0_e2260).powf(noise_metadata_schedule_205_0_e2262);
        let noise_metadata_schedule_205_0_e2266: f64 = (w[143] - w[222]);
        let noise_metadata_schedule_205_0_e2268: f64 = (noise_metadata_schedule_205_0_e2266 + w[218]);
        let noise_metadata_schedule_205_0_e2269: f64 = (noise_metadata_schedule_205_0_e2263 * noise_metadata_schedule_205_0_e2268);
        let noise_metadata_schedule_205_0_e2273: f64 = (0.5 * params[38]);
        let noise_metadata_schedule_205_0_e2276: f64 = (w[143] - w[222]);
        let noise_metadata_schedule_205_0_e2278: f64 = (noise_metadata_schedule_205_0_e2276 + w[218]);
        let noise_metadata_schedule_205_0_e2279: f64 = (noise_metadata_schedule_205_0_e2273 * noise_metadata_schedule_205_0_e2278);
        let noise_metadata_schedule_205_0_e2283: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_205_0_e2284: f64 = (w[20] * noise_metadata_schedule_205_0_e2283);
        let noise_metadata_schedule_205_0_e2285: f64 = (noise_metadata_schedule_205_0_e2279 / noise_metadata_schedule_205_0_e2284);
        let noise_metadata_schedule_205_0_e2286: f64 = (1.0 + noise_metadata_schedule_205_0_e2285);
        let noise_metadata_schedule_205_0_e2287: f64 = (noise_metadata_schedule_205_0_e2269 * noise_metadata_schedule_205_0_e2286);
        let noise_metadata_schedule_205_0_e2288: f64 = (w[215] + noise_metadata_schedule_205_0_e2287);
        let noise_metadata_schedule_205_0_e2290: f64 = (noise_metadata_schedule_205_0_e2288 - w[219]);
        (noise_metadata_schedule_205_0_e2290,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_205_0_e2292;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_206_0_e2294: f64 = (-w[21]);
            let noise_metadata_schedule_206_0_e2296: f64 = (noise_metadata_schedule_206_0_e2294 * params[34]);
            w[225] = noise_metadata_schedule_206_0_e2296;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_207_0_e2299: f64 = if params[44] <= 0.0 { 1.0 } else { 0.0 };
            w[246] = noise_metadata_schedule_207_0_e2299;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_208_0_e2305,) = {
    if (w[246] != 0.0) {
        let noise_metadata_schedule_208_0_e2303: f64 = (w[144] + w[225]);
        (noise_metadata_schedule_208_0_e2303,)
    } else {
        (w[226],)
    }
};
            w[226] = noise_metadata_schedule_208_0_e2305;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_209_0_e2308: f64 = if w[226] > 0.0 { 1.0 } else { 0.0 };
            w[247] = noise_metadata_schedule_209_0_e2308;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_210_0_e2321,) = {
    if ((w[246] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_210_0_e2314: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_210_0_e2316: f64 = (-1.0);
        let noise_metadata_schedule_210_0_e2318: f64 = (noise_metadata_schedule_210_0_e2316 - params[43]);
        let noise_metadata_schedule_210_0_e2319: f64 = (noise_metadata_schedule_210_0_e2314).powf(noise_metadata_schedule_210_0_e2318);
        (noise_metadata_schedule_210_0_e2319,)
    } else {
        (w[227],)
    }
};
            w[227] = noise_metadata_schedule_210_0_e2321;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_211_0_e2343,) = {
    if ((w[246] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_211_0_e2330: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_211_0_e2331: f64 = (w[227] * noise_metadata_schedule_211_0_e2330);
        let noise_metadata_schedule_211_0_e2334: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_211_0_e2335: f64 = (noise_metadata_schedule_211_0_e2331 * noise_metadata_schedule_211_0_e2334);
        let noise_metadata_schedule_211_0_e2336: f64 = (1.0 - noise_metadata_schedule_211_0_e2335);
        let noise_metadata_schedule_211_0_e2337: f64 = (w[21] * noise_metadata_schedule_211_0_e2336);
        let noise_metadata_schedule_211_0_e2340: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_211_0_e2341: f64 = (noise_metadata_schedule_211_0_e2337 / noise_metadata_schedule_211_0_e2340);
        (noise_metadata_schedule_211_0_e2341,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_211_0_e2343;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_212_0_e2363,) = {
    if ((w[246] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_212_0_e2350: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_212_0_e2353: f64 = (0.5 * params[43]);
        let noise_metadata_schedule_212_0_e2355: f64 = (noise_metadata_schedule_212_0_e2353 * w[226]);
        let noise_metadata_schedule_212_0_e2357: f64 = (noise_metadata_schedule_212_0_e2355 / w[21]);
        let noise_metadata_schedule_212_0_e2358: f64 = (noise_metadata_schedule_212_0_e2350 + noise_metadata_schedule_212_0_e2357);
        let noise_metadata_schedule_212_0_e2359: f64 = (w[226] * noise_metadata_schedule_212_0_e2358);
        let noise_metadata_schedule_212_0_e2361: f64 = (noise_metadata_schedule_212_0_e2359 * w[227]);
        (noise_metadata_schedule_212_0_e2361,)
    } else {
        (w[229],)
    }
};
            w[229] = noise_metadata_schedule_212_0_e2363;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_213_0_e2369: f64 = (-params[45]);
            let noise_metadata_schedule_213_0_e2371: f64 = if ((params[45] > 0.0) && (w[144] < noise_metadata_schedule_213_0_e2369)) { 1.0 } else { 0.0 };
            w[248] = noise_metadata_schedule_213_0_e2371;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_214_0_e2410,) = {
    if (((w[246] != 0.0) && (w[247] == 0.0)) && (w[248] != 0.0)) {
        let noise_metadata_schedule_214_0_e2383: f64 = (params[45] / w[21]);
        let noise_metadata_schedule_214_0_e2384: f64 = (1.0 + noise_metadata_schedule_214_0_e2383);
        let noise_metadata_schedule_214_0_e2387: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_214_0_e2388: f64 = (noise_metadata_schedule_214_0_e2384).powf(noise_metadata_schedule_214_0_e2387);
        let noise_metadata_schedule_214_0_e2392: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_214_0_e2395: f64 = (w[144] + params[45]);
        let noise_metadata_schedule_214_0_e2396: f64 = (noise_metadata_schedule_214_0_e2392 * noise_metadata_schedule_214_0_e2395);
        let noise_metadata_schedule_214_0_e2399: f64 = (w[21] + params[45]);
        let noise_metadata_schedule_214_0_e2400: f64 = (noise_metadata_schedule_214_0_e2396 / noise_metadata_schedule_214_0_e2399);
        let noise_metadata_schedule_214_0_e2401: f64 = (1.0 - noise_metadata_schedule_214_0_e2400);
        let noise_metadata_schedule_214_0_e2402: f64 = (noise_metadata_schedule_214_0_e2388 * noise_metadata_schedule_214_0_e2401);
        let noise_metadata_schedule_214_0_e2403: f64 = (1.0 - noise_metadata_schedule_214_0_e2402);
        let noise_metadata_schedule_214_0_e2404: f64 = (w[21] * noise_metadata_schedule_214_0_e2403);
        let noise_metadata_schedule_214_0_e2407: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_214_0_e2408: f64 = (noise_metadata_schedule_214_0_e2404 / noise_metadata_schedule_214_0_e2407);
        (noise_metadata_schedule_214_0_e2408,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_214_0_e2410;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_215_0_e2436,) = {
    if (((w[246] != 0.0) && (w[247] == 0.0)) && (w[248] == 0.0)) {
        let noise_metadata_schedule_215_0_e2423: f64 = (w[144] / w[21]);
        let noise_metadata_schedule_215_0_e2424: f64 = (1.0 - noise_metadata_schedule_215_0_e2423);
        let noise_metadata_schedule_215_0_e2427: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_215_0_e2428: f64 = (noise_metadata_schedule_215_0_e2424).powf(noise_metadata_schedule_215_0_e2427);
        let noise_metadata_schedule_215_0_e2429: f64 = (1.0 - noise_metadata_schedule_215_0_e2428);
        let noise_metadata_schedule_215_0_e2430: f64 = (w[21] * noise_metadata_schedule_215_0_e2429);
        let noise_metadata_schedule_215_0_e2433: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_215_0_e2434: f64 = (noise_metadata_schedule_215_0_e2430 / noise_metadata_schedule_215_0_e2433);
        (noise_metadata_schedule_215_0_e2434,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_215_0_e2436;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_216_0_e2443,) = {
    if ((w[246] != 0.0) && (w[247] == 0.0)) {
        (0.0,)
    } else {
        (w[229],)
    }
};
            w[229] = noise_metadata_schedule_216_0_e2443;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_217_0_e2449,) = {
    if (w[246] != 0.0) {
        let noise_metadata_schedule_217_0_e2447: f64 = (w[228] + w[229]);
        (noise_metadata_schedule_217_0_e2447,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_217_0_e2449;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_218_0_e2456: f64 = if ((params[45] > 0.0) && (params[46] > 0.0)) { 1.0 } else { 0.0 };
            w[249] = noise_metadata_schedule_218_0_e2456;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_219_0_e2469,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_219_0_e2463: f64 = (params[45] + w[225]);
        let noise_metadata_schedule_219_0_e2466: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_219_0_e2467: f64 = (noise_metadata_schedule_219_0_e2463 / noise_metadata_schedule_219_0_e2466);
        (noise_metadata_schedule_219_0_e2467,)
    } else {
        (w[230],)
    }
};
            w[230] = noise_metadata_schedule_219_0_e2469;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_220_0_e2508,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_220_0_e2476: f64 = (2.0 * w[230]);
        let noise_metadata_schedule_220_0_e2479: f64 = (w[230] - 1.0);
        let noise_metadata_schedule_220_0_e2482: f64 = (w[230] - 1.0);
        let noise_metadata_schedule_220_0_e2483: f64 = (noise_metadata_schedule_220_0_e2479 * noise_metadata_schedule_220_0_e2482);
        let noise_metadata_schedule_220_0_e2486: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_220_0_e2488: f64 = (noise_metadata_schedule_220_0_e2486 * params[44]);
        let noise_metadata_schedule_220_0_e2489: f64 = (noise_metadata_schedule_220_0_e2483 + noise_metadata_schedule_220_0_e2488);
        let noise_metadata_schedule_220_0_e2490: f64 = (noise_metadata_schedule_220_0_e2489).sqrt();
        let noise_metadata_schedule_220_0_e2493: f64 = (w[230] + 1.0);
        let noise_metadata_schedule_220_0_e2496: f64 = (w[230] + 1.0);
        let noise_metadata_schedule_220_0_e2497: f64 = (noise_metadata_schedule_220_0_e2493 * noise_metadata_schedule_220_0_e2496);
        let noise_metadata_schedule_220_0_e2500: f64 = (4.0 * params[46]);
        let noise_metadata_schedule_220_0_e2502: f64 = (noise_metadata_schedule_220_0_e2500 * params[46]);
        let noise_metadata_schedule_220_0_e2503: f64 = (noise_metadata_schedule_220_0_e2497 + noise_metadata_schedule_220_0_e2502);
        let noise_metadata_schedule_220_0_e2504: f64 = (noise_metadata_schedule_220_0_e2503).sqrt();
        let noise_metadata_schedule_220_0_e2505: f64 = (noise_metadata_schedule_220_0_e2490 + noise_metadata_schedule_220_0_e2504);
        let noise_metadata_schedule_220_0_e2506: f64 = (noise_metadata_schedule_220_0_e2476 / noise_metadata_schedule_220_0_e2505);
        (noise_metadata_schedule_220_0_e2506,)
    } else {
        (w[231],)
    }
};
            w[231] = noise_metadata_schedule_220_0_e2508;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_221_0_e2525,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_221_0_e2517: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_221_0_e2518: f64 = (w[231] * noise_metadata_schedule_221_0_e2517);
        let noise_metadata_schedule_221_0_e2520: f64 = (noise_metadata_schedule_221_0_e2518 - params[45]);
        let noise_metadata_schedule_221_0_e2522: f64 = (noise_metadata_schedule_221_0_e2520 - w[225]);
        let noise_metadata_schedule_221_0_e2523: f64 = (0.5 * noise_metadata_schedule_221_0_e2522);
        (noise_metadata_schedule_221_0_e2523,)
    } else {
        (w[232],)
    }
};
            w[232] = noise_metadata_schedule_221_0_e2525;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_222_0_e2548,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_222_0_e2535: f64 = (w[232] / w[21]);
        let noise_metadata_schedule_222_0_e2536: f64 = (1.0 - noise_metadata_schedule_222_0_e2535);
        let noise_metadata_schedule_222_0_e2539: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_222_0_e2540: f64 = (noise_metadata_schedule_222_0_e2536).powf(noise_metadata_schedule_222_0_e2539);
        let noise_metadata_schedule_222_0_e2541: f64 = (1.0 - noise_metadata_schedule_222_0_e2540);
        let noise_metadata_schedule_222_0_e2542: f64 = (w[21] * noise_metadata_schedule_222_0_e2541);
        let noise_metadata_schedule_222_0_e2545: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_222_0_e2546: f64 = (noise_metadata_schedule_222_0_e2542 / noise_metadata_schedule_222_0_e2545);
        (noise_metadata_schedule_222_0_e2546,)
    } else {
        (w[233],)
    }
};
            w[233] = noise_metadata_schedule_222_0_e2548;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_223_0_e2565,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_223_0_e2555: f64 = (2.0 * w[144]);
        let noise_metadata_schedule_223_0_e2557: f64 = (noise_metadata_schedule_223_0_e2555 + params[45]);
        let noise_metadata_schedule_223_0_e2559: f64 = (noise_metadata_schedule_223_0_e2557 + w[225]);
        let noise_metadata_schedule_223_0_e2562: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_223_0_e2563: f64 = (noise_metadata_schedule_223_0_e2559 / noise_metadata_schedule_223_0_e2562);
        (noise_metadata_schedule_223_0_e2563,)
    } else {
        (w[234],)
    }
};
            w[234] = noise_metadata_schedule_223_0_e2565;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_224_0_e2604,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_224_0_e2572: f64 = (2.0 * w[234]);
        let noise_metadata_schedule_224_0_e2575: f64 = (w[234] - 1.0);
        let noise_metadata_schedule_224_0_e2578: f64 = (w[234] - 1.0);
        let noise_metadata_schedule_224_0_e2579: f64 = (noise_metadata_schedule_224_0_e2575 * noise_metadata_schedule_224_0_e2578);
        let noise_metadata_schedule_224_0_e2582: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_224_0_e2584: f64 = (noise_metadata_schedule_224_0_e2582 * params[44]);
        let noise_metadata_schedule_224_0_e2585: f64 = (noise_metadata_schedule_224_0_e2579 + noise_metadata_schedule_224_0_e2584);
        let noise_metadata_schedule_224_0_e2586: f64 = (noise_metadata_schedule_224_0_e2585).sqrt();
        let noise_metadata_schedule_224_0_e2589: f64 = (w[234] + 1.0);
        let noise_metadata_schedule_224_0_e2592: f64 = (w[234] + 1.0);
        let noise_metadata_schedule_224_0_e2593: f64 = (noise_metadata_schedule_224_0_e2589 * noise_metadata_schedule_224_0_e2592);
        let noise_metadata_schedule_224_0_e2596: f64 = (4.0 * params[46]);
        let noise_metadata_schedule_224_0_e2598: f64 = (noise_metadata_schedule_224_0_e2596 * params[46]);
        let noise_metadata_schedule_224_0_e2599: f64 = (noise_metadata_schedule_224_0_e2593 + noise_metadata_schedule_224_0_e2598);
        let noise_metadata_schedule_224_0_e2600: f64 = (noise_metadata_schedule_224_0_e2599).sqrt();
        let noise_metadata_schedule_224_0_e2601: f64 = (noise_metadata_schedule_224_0_e2586 + noise_metadata_schedule_224_0_e2600);
        let noise_metadata_schedule_224_0_e2602: f64 = (noise_metadata_schedule_224_0_e2572 / noise_metadata_schedule_224_0_e2601);
        (noise_metadata_schedule_224_0_e2602,)
    } else {
        (w[235],)
    }
};
            w[235] = noise_metadata_schedule_224_0_e2604;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_225_0_e2621,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_225_0_e2613: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_225_0_e2614: f64 = (w[235] * noise_metadata_schedule_225_0_e2613);
        let noise_metadata_schedule_225_0_e2616: f64 = (noise_metadata_schedule_225_0_e2614 - params[45]);
        let noise_metadata_schedule_225_0_e2618: f64 = (noise_metadata_schedule_225_0_e2616 - w[225]);
        let noise_metadata_schedule_225_0_e2619: f64 = (0.5 * noise_metadata_schedule_225_0_e2618);
        (noise_metadata_schedule_225_0_e2619,)
    } else {
        (w[236],)
    }
};
            w[236] = noise_metadata_schedule_225_0_e2621;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_226_0_e2644,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_226_0_e2631: f64 = (w[236] / w[21]);
        let noise_metadata_schedule_226_0_e2632: f64 = (1.0 - noise_metadata_schedule_226_0_e2631);
        let noise_metadata_schedule_226_0_e2635: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_226_0_e2636: f64 = (noise_metadata_schedule_226_0_e2632).powf(noise_metadata_schedule_226_0_e2635);
        let noise_metadata_schedule_226_0_e2637: f64 = (1.0 - noise_metadata_schedule_226_0_e2636);
        let noise_metadata_schedule_226_0_e2638: f64 = (w[21] * noise_metadata_schedule_226_0_e2637);
        let noise_metadata_schedule_226_0_e2641: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_226_0_e2642: f64 = (noise_metadata_schedule_226_0_e2638 / noise_metadata_schedule_226_0_e2641);
        (noise_metadata_schedule_226_0_e2642,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_226_0_e2644;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_227_0_e2655,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_227_0_e2652: f64 = (w[235] + 1.0);
        let noise_metadata_schedule_227_0_e2653: f64 = (0.5 * noise_metadata_schedule_227_0_e2652);
        (noise_metadata_schedule_227_0_e2653,)
    } else {
        (w[237],)
    }
};
            w[237] = noise_metadata_schedule_227_0_e2655;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_228_0_e2669,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_228_0_e2663: f64 = (params[45] / w[21]);
        let noise_metadata_schedule_228_0_e2664: f64 = (1.0 + noise_metadata_schedule_228_0_e2663);
        let noise_metadata_schedule_228_0_e2666: f64 = (-params[43]);
        let noise_metadata_schedule_228_0_e2667: f64 = (noise_metadata_schedule_228_0_e2664).powf(noise_metadata_schedule_228_0_e2666);
        (noise_metadata_schedule_228_0_e2667,)
    } else {
        (w[238],)
    }
};
            w[238] = noise_metadata_schedule_228_0_e2669;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_229_0_e2683,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_229_0_e2677: f64 = (w[225] / w[21]);
        let noise_metadata_schedule_229_0_e2678: f64 = (1.0 + noise_metadata_schedule_229_0_e2677);
        let noise_metadata_schedule_229_0_e2680: f64 = (-params[43]);
        let noise_metadata_schedule_229_0_e2681: f64 = (noise_metadata_schedule_229_0_e2678).powf(noise_metadata_schedule_229_0_e2680);
        (noise_metadata_schedule_229_0_e2681,)
    } else {
        (w[239],)
    }
};
            w[239] = noise_metadata_schedule_229_0_e2683;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_230_0_e2698,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_230_0_e2690: f64 = (1.0 - w[237]);
        let noise_metadata_schedule_230_0_e2692: f64 = (noise_metadata_schedule_230_0_e2690 * w[238]);
        let noise_metadata_schedule_230_0_e2695: f64 = (w[237] * w[239]);
        let noise_metadata_schedule_230_0_e2696: f64 = (noise_metadata_schedule_230_0_e2692 + noise_metadata_schedule_230_0_e2695);
        (noise_metadata_schedule_230_0_e2696,)
    } else {
        (w[240],)
    }
};
            w[240] = noise_metadata_schedule_230_0_e2698;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_231_0_e2711,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_231_0_e2705: f64 = (w[144] - w[236]);
        let noise_metadata_schedule_231_0_e2707: f64 = (noise_metadata_schedule_231_0_e2705 + w[232]);
        let noise_metadata_schedule_231_0_e2709: f64 = (noise_metadata_schedule_231_0_e2707 * w[240]);
        (noise_metadata_schedule_231_0_e2709,)
    } else {
        (w[241],)
    }
};
            w[241] = noise_metadata_schedule_231_0_e2711;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_232_0_e2722,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_232_0_e2718: f64 = (w[241] + w[228]);
        let noise_metadata_schedule_232_0_e2720: f64 = (noise_metadata_schedule_232_0_e2718 - w[233]);
        (noise_metadata_schedule_232_0_e2720,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_232_0_e2722;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_233_0_e2739,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_233_0_e2730: f64 = (w[225] * w[225]);
        let noise_metadata_schedule_233_0_e2733: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_233_0_e2735: f64 = (noise_metadata_schedule_233_0_e2733 * params[44]);
        let noise_metadata_schedule_233_0_e2736: f64 = (noise_metadata_schedule_233_0_e2730 + noise_metadata_schedule_233_0_e2735);
        let noise_metadata_schedule_233_0_e2737: f64 = (noise_metadata_schedule_233_0_e2736).sqrt();
        (noise_metadata_schedule_233_0_e2737,)
    } else {
        (w[242],)
    }
};
            w[242] = noise_metadata_schedule_233_0_e2739;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_234_0_e2752,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_234_0_e2746: f64 = (-0.5);
        let noise_metadata_schedule_234_0_e2749: f64 = (w[225] + w[242]);
        let noise_metadata_schedule_234_0_e2750: f64 = (noise_metadata_schedule_234_0_e2746 * noise_metadata_schedule_234_0_e2749);
        (noise_metadata_schedule_234_0_e2750,)
    } else {
        (w[232],)
    }
};
            w[232] = noise_metadata_schedule_234_0_e2752;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_235_0_e2775,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_235_0_e2759: f64 = (-w[21]);
        let noise_metadata_schedule_235_0_e2763: f64 = (w[232] / w[21]);
        let noise_metadata_schedule_235_0_e2764: f64 = (1.0 - noise_metadata_schedule_235_0_e2763);
        let noise_metadata_schedule_235_0_e2767: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_235_0_e2768: f64 = (noise_metadata_schedule_235_0_e2764).powf(noise_metadata_schedule_235_0_e2767);
        let noise_metadata_schedule_235_0_e2769: f64 = (noise_metadata_schedule_235_0_e2759 * noise_metadata_schedule_235_0_e2768);
        let noise_metadata_schedule_235_0_e2772: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_235_0_e2773: f64 = (noise_metadata_schedule_235_0_e2769 / noise_metadata_schedule_235_0_e2772);
        (noise_metadata_schedule_235_0_e2773,)
    } else {
        (w[243],)
    }
};
            w[243] = noise_metadata_schedule_235_0_e2775;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_236_0_e2785,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_236_0_e2783: f64 = (w[144] + w[225]);
        (noise_metadata_schedule_236_0_e2783,)
    } else {
        (w[244],)
    }
};
            w[244] = noise_metadata_schedule_236_0_e2785;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_237_0_e2802,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_237_0_e2793: f64 = (w[244] * w[244]);
        let noise_metadata_schedule_237_0_e2796: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_237_0_e2798: f64 = (noise_metadata_schedule_237_0_e2796 * params[44]);
        let noise_metadata_schedule_237_0_e2799: f64 = (noise_metadata_schedule_237_0_e2793 + noise_metadata_schedule_237_0_e2798);
        let noise_metadata_schedule_237_0_e2800: f64 = (noise_metadata_schedule_237_0_e2799).sqrt();
        (noise_metadata_schedule_237_0_e2800,)
    } else {
        (w[245],)
    }
};
            w[245] = noise_metadata_schedule_237_0_e2802;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_238_0_e2816,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_238_0_e2811: f64 = (w[244] - w[245]);
        let noise_metadata_schedule_238_0_e2812: f64 = (0.5 * noise_metadata_schedule_238_0_e2811);
        let noise_metadata_schedule_238_0_e2814: f64 = (noise_metadata_schedule_238_0_e2812 - w[225]);
        (noise_metadata_schedule_238_0_e2814,)
    } else {
        (w[236],)
    }
};
            w[236] = noise_metadata_schedule_238_0_e2816;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_239_0_e2839,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_239_0_e2823: f64 = (-w[21]);
        let noise_metadata_schedule_239_0_e2827: f64 = (w[236] / w[21]);
        let noise_metadata_schedule_239_0_e2828: f64 = (1.0 - noise_metadata_schedule_239_0_e2827);
        let noise_metadata_schedule_239_0_e2831: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_239_0_e2832: f64 = (noise_metadata_schedule_239_0_e2828).powf(noise_metadata_schedule_239_0_e2831);
        let noise_metadata_schedule_239_0_e2833: f64 = (noise_metadata_schedule_239_0_e2823 * noise_metadata_schedule_239_0_e2832);
        let noise_metadata_schedule_239_0_e2836: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_239_0_e2837: f64 = (noise_metadata_schedule_239_0_e2833 / noise_metadata_schedule_239_0_e2836);
        (noise_metadata_schedule_239_0_e2837,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_239_0_e2839;
        }
        if (active[0] & 0x1413) != 0 {
            let (noise_metadata_schedule_240_0_e2862,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_240_0_e2848: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_240_0_e2850: f64 = (-params[43]);
        let noise_metadata_schedule_240_0_e2851: f64 = (noise_metadata_schedule_240_0_e2848).powf(noise_metadata_schedule_240_0_e2850);
        let noise_metadata_schedule_240_0_e2854: f64 = (w[144] - w[236]);
        let noise_metadata_schedule_240_0_e2856: f64 = (noise_metadata_schedule_240_0_e2854 + w[232]);
        let noise_metadata_schedule_240_0_e2857: f64 = (noise_metadata_schedule_240_0_e2851 * noise_metadata_schedule_240_0_e2856);
        let noise_metadata_schedule_240_0_e2858: f64 = (w[228] + noise_metadata_schedule_240_0_e2857);
        let noise_metadata_schedule_240_0_e2860: f64 = (noise_metadata_schedule_240_0_e2858 - w[243]);
        (noise_metadata_schedule_240_0_e2860,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_240_0_e2862;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_241_0_e2866: f64 = (w[27] * w[73]);
            let noise_metadata_schedule_241_0_e2867: f64 = (1.0 / noise_metadata_schedule_241_0_e2866);
            w[112] = noise_metadata_schedule_241_0_e2867;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_242_0_e2870: f64 = if w[143] < w[61] { 1.0 } else { 0.0 };
            w[250] = noise_metadata_schedule_242_0_e2870;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_243_0_e2877,) = {
    if (w[250] != 0.0) {
        let noise_metadata_schedule_243_0_e2874: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_243_0_e2875: f64 = (noise_metadata_schedule_243_0_e2874).exp();
        (noise_metadata_schedule_243_0_e2875,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_243_0_e2877;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_244_0_e2893,) = {
    if (w[250] == 0.0) {
        let noise_metadata_schedule_244_0_e2882: f64 = (w[61] * w[112]);
        let noise_metadata_schedule_244_0_e2883: f64 = (noise_metadata_schedule_244_0_e2882).exp();
        let noise_metadata_schedule_244_0_e2887: f64 = (w[143] - w[61]);
        let noise_metadata_schedule_244_0_e2889: f64 = (noise_metadata_schedule_244_0_e2887 * w[112]);
        let noise_metadata_schedule_244_0_e2890: f64 = (1.0 + noise_metadata_schedule_244_0_e2889);
        let noise_metadata_schedule_244_0_e2891: f64 = (noise_metadata_schedule_244_0_e2883 * noise_metadata_schedule_244_0_e2890);
        (noise_metadata_schedule_244_0_e2891,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_244_0_e2893;
        }
        if (active[0] & 0x1410) != 0 {
            let noise_metadata_schedule_245_0_e2897: f64 = (w[109] - 1.0);
            let noise_metadata_schedule_245_0_e2898: f64 = (w[0] * noise_metadata_schedule_245_0_e2897);
            w[74] = noise_metadata_schedule_245_0_e2898;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_246_0_e2902: f64 = (w[28] * w[73]);
            let noise_metadata_schedule_246_0_e2903: f64 = (1.0 / noise_metadata_schedule_246_0_e2902);
            w[112] = noise_metadata_schedule_246_0_e2903;
        }
        if (active[0] & 0x157f) != 0 {
            let noise_metadata_schedule_247_0_e2906: f64 = if w[144] < w[62] { 1.0 } else { 0.0 };
            w[251] = noise_metadata_schedule_247_0_e2906;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_248_0_e2913,) = {
    if (w[251] != 0.0) {
        let noise_metadata_schedule_248_0_e2910: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_248_0_e2911: f64 = (noise_metadata_schedule_248_0_e2910).exp();
        (noise_metadata_schedule_248_0_e2911,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_248_0_e2913;
        }
        if (active[0] & 0x157f) != 0 {
            let (noise_metadata_schedule_249_0_e2929,) = {
    if (w[251] == 0.0) {
        let noise_metadata_schedule_249_0_e2918: f64 = (w[62] * w[112]);
        let noise_metadata_schedule_249_0_e2919: f64 = (noise_metadata_schedule_249_0_e2918).exp();
        let noise_metadata_schedule_249_0_e2923: f64 = (w[144] - w[62]);
        let noise_metadata_schedule_249_0_e2925: f64 = (noise_metadata_schedule_249_0_e2923 * w[112]);
        let noise_metadata_schedule_249_0_e2926: f64 = (1.0 + noise_metadata_schedule_249_0_e2925);
        let noise_metadata_schedule_249_0_e2927: f64 = (noise_metadata_schedule_249_0_e2919 * noise_metadata_schedule_249_0_e2926);
        (noise_metadata_schedule_249_0_e2927,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_249_0_e2929;
        }
        if (active[0] & 0x1410) != 0 {
            let noise_metadata_schedule_250_0_e2932: f64 = (w[0] * w[1]);
            let noise_metadata_schedule_250_0_e2935: f64 = (w[109] - 1.0);
            let noise_metadata_schedule_250_0_e2936: f64 = (noise_metadata_schedule_250_0_e2932 * noise_metadata_schedule_250_0_e2935);
            w[75] = noise_metadata_schedule_250_0_e2936;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_251_0_e2940: f64 = (w[114] * w[44]);
            let noise_metadata_schedule_251_0_e2941: f64 = (1.0 + noise_metadata_schedule_251_0_e2940);
            let noise_metadata_schedule_251_0_e2944: f64 = (w[116] * w[43]);
            let noise_metadata_schedule_251_0_e2945: f64 = (noise_metadata_schedule_251_0_e2941 + noise_metadata_schedule_251_0_e2944);
            let noise_metadata_schedule_251_0_e2947: f64 = (noise_metadata_schedule_251_0_e2945 - 0.0001);
            w[78] = noise_metadata_schedule_251_0_e2947;
        }
        if (active[0] & 0x1413) != 0 {
            let noise_metadata_schedule_252_0_e2951: f64 = (w[78] * w[78]);
            let noise_metadata_schedule_252_0_e2953: f64 = (noise_metadata_schedule_252_0_e2951 + 1e-8);
            let noise_metadata_schedule_252_0_e2954: f64 = (noise_metadata_schedule_252_0_e2953).sqrt();
            let noise_metadata_schedule_252_0_e2956: f64 = (noise_metadata_schedule_252_0_e2954 + w[78]);
            let noise_metadata_schedule_252_0_e2957: f64 = (0.5 * noise_metadata_schedule_252_0_e2956);
            let noise_metadata_schedule_252_0_e2959: f64 = (noise_metadata_schedule_252_0_e2957 + 0.0001);
            w[79] = noise_metadata_schedule_252_0_e2959;
        }
        if (active[0] & 0x1410) != 0 {
            let noise_metadata_schedule_253_0_e2962: f64 = (w[74] * w[45]);
            let noise_metadata_schedule_253_0_e2965: f64 = (w[75] * w[46]);
            let noise_metadata_schedule_253_0_e2966: f64 = (noise_metadata_schedule_253_0_e2962 + noise_metadata_schedule_253_0_e2965);
            w[80] = noise_metadata_schedule_253_0_e2966;
        }
        if (active[0] & 0x1410) != 0 {
            let noise_metadata_schedule_254_0_e2969: f64 = if params[30] < 0.5 { 1.0 } else { 0.0 };
            w[252] = noise_metadata_schedule_254_0_e2969;
        }
        if (active[0] & 0x1410) != 0 {
            let (noise_metadata_schedule_255_0_e2981,) = {
    if (w[252] != 0.0) {
        let noise_metadata_schedule_255_0_e2974: f64 = (1.0 / params[73]);
        let noise_metadata_schedule_255_0_e2975: f64 = (w[79]).powf(noise_metadata_schedule_255_0_e2974);
        let noise_metadata_schedule_255_0_e2978: f64 = (4.0 * w[80]);
        let noise_metadata_schedule_255_0_e2979: f64 = (noise_metadata_schedule_255_0_e2975 + noise_metadata_schedule_255_0_e2978);
        (noise_metadata_schedule_255_0_e2979,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_255_0_e2981;
        }
        if (active[0] & 0x410) != 0 {
            let noise_metadata_schedule_256_0_e2984: f64 = if w[108] > 1e-8 { 1.0 } else { 0.0 };
            w[253] = noise_metadata_schedule_256_0_e2984;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_257_0_e2996,) = {
    if ((w[252] != 0.0) && (w[253] != 0.0)) {
        let noise_metadata_schedule_257_0_e2992: f64 = (w[108]).powf(params[73]);
        let noise_metadata_schedule_257_0_e2993: f64 = (w[79] + noise_metadata_schedule_257_0_e2992);
        let noise_metadata_schedule_257_0_e2994: f64 = (0.5 * noise_metadata_schedule_257_0_e2993);
        (noise_metadata_schedule_257_0_e2994,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_257_0_e2996;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_258_0_e3009,) = {
    if ((w[252] != 0.0) && (w[253] == 0.0)) {
        let noise_metadata_schedule_258_0_e3005: f64 = (1e-8_f64).powf(params[73]);
        let noise_metadata_schedule_258_0_e3006: f64 = (w[79] + noise_metadata_schedule_258_0_e3005);
        let noise_metadata_schedule_258_0_e3007: f64 = (0.5 * noise_metadata_schedule_258_0_e3006);
        (noise_metadata_schedule_258_0_e3007,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_258_0_e3009;
        }
        if (active[0] & 0x1410) != 0 {
            let (noise_metadata_schedule_259_0_e3018,) = {
    if (w[252] == 0.0) {
        let noise_metadata_schedule_259_0_e3015: f64 = (4.0 * w[80]);
        let noise_metadata_schedule_259_0_e3016: f64 = (1.0 + noise_metadata_schedule_259_0_e3015);
        (noise_metadata_schedule_259_0_e3016,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_259_0_e3018;
        }
        if (active[0] & 0x410) != 0 {
            let noise_metadata_schedule_260_0_e3021: f64 = if w[108] > 1e-8 { 1.0 } else { 0.0 };
            w[254] = noise_metadata_schedule_260_0_e3021;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_261_0_e3036,) = {
    if ((w[252] == 0.0) && (w[254] != 0.0)) {
        let noise_metadata_schedule_261_0_e3028: f64 = (0.5 * w[79]);
        let noise_metadata_schedule_261_0_e3032: f64 = (w[108]).powf(params[73]);
        let noise_metadata_schedule_261_0_e3033: f64 = (1.0 + noise_metadata_schedule_261_0_e3032);
        let noise_metadata_schedule_261_0_e3034: f64 = (noise_metadata_schedule_261_0_e3028 * noise_metadata_schedule_261_0_e3033);
        (noise_metadata_schedule_261_0_e3034,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_261_0_e3036;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_262_0_e3052,) = {
    if ((w[252] == 0.0) && (w[254] == 0.0)) {
        let noise_metadata_schedule_262_0_e3044: f64 = (0.5 * w[79]);
        let noise_metadata_schedule_262_0_e3048: f64 = (1e-8_f64).powf(params[73]);
        let noise_metadata_schedule_262_0_e3049: f64 = (1.0 + noise_metadata_schedule_262_0_e3048);
        let noise_metadata_schedule_262_0_e3050: f64 = (noise_metadata_schedule_262_0_e3044 * noise_metadata_schedule_262_0_e3049);
        (noise_metadata_schedule_262_0_e3050,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_262_0_e3052;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_264_0_e3058: f64 = (w[74] / w[81]);
            w[76] = noise_metadata_schedule_264_0_e3058;
        }
        if (active[0] & 0x116f) != 0 {
            let noise_metadata_schedule_266_0_e3062: f64 = if params[31] > 0.0 { 1.0 } else { 0.0 };
            w[255] = noise_metadata_schedule_266_0_e3062;
        }
        if (active[0] & 0x116f) != 0 {
            let (noise_metadata_schedule_267_0_e3070,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_267_0_e3067: f64 = (params[33] * w[73]);
        let noise_metadata_schedule_267_0_e3068: f64 = (1.0 / noise_metadata_schedule_267_0_e3067);
        (noise_metadata_schedule_267_0_e3068,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_267_0_e3070;
        }
        if (active[0] & 0x116f) != 0 {
            let noise_metadata_schedule_268_0_e3073: f64 = if w[146] < w[63] { 1.0 } else { 0.0 };
            w[256] = noise_metadata_schedule_268_0_e3073;
        }
        if (active[0] & 0x116f) != 0 {
            let (noise_metadata_schedule_269_0_e3082,) = {
    if ((w[255] != 0.0) && (w[256] != 0.0)) {
        let noise_metadata_schedule_269_0_e3079: f64 = (w[146] * w[112]);
        let noise_metadata_schedule_269_0_e3080: f64 = (noise_metadata_schedule_269_0_e3079).exp();
        (noise_metadata_schedule_269_0_e3080,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_269_0_e3082;
        }
        if (active[0] & 0x116f) != 0 {
            let (noise_metadata_schedule_270_0_e3100,) = {
    if ((w[255] != 0.0) && (w[256] == 0.0)) {
        let noise_metadata_schedule_270_0_e3089: f64 = (w[63] * w[112]);
        let noise_metadata_schedule_270_0_e3090: f64 = (noise_metadata_schedule_270_0_e3089).exp();
        let noise_metadata_schedule_270_0_e3094: f64 = (w[146] - w[63]);
        let noise_metadata_schedule_270_0_e3096: f64 = (noise_metadata_schedule_270_0_e3094 * w[112]);
        let noise_metadata_schedule_270_0_e3097: f64 = (1.0 + noise_metadata_schedule_270_0_e3096);
        let noise_metadata_schedule_270_0_e3098: f64 = (noise_metadata_schedule_270_0_e3090 * noise_metadata_schedule_270_0_e3097);
        (noise_metadata_schedule_270_0_e3098,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_270_0_e3100;
        }
        if (active[0] & 0x110f) != 0 {
            let noise_metadata_schedule_271_0_e3103: f64 = if w[144] < w[63] { 1.0 } else { 0.0 };
            w[257] = noise_metadata_schedule_271_0_e3103;
        }
        if (active[0] & 0x110f) != 0 {
            let (noise_metadata_schedule_272_0_e3112,) = {
    if ((w[255] != 0.0) && (w[257] != 0.0)) {
        let noise_metadata_schedule_272_0_e3109: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_272_0_e3110: f64 = (noise_metadata_schedule_272_0_e3109).exp();
        (noise_metadata_schedule_272_0_e3110,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_272_0_e3112;
        }
        if (active[0] & 0x110f) != 0 {
            let (noise_metadata_schedule_273_0_e3130,) = {
    if ((w[255] != 0.0) && (w[257] == 0.0)) {
        let noise_metadata_schedule_273_0_e3119: f64 = (w[63] * w[112]);
        let noise_metadata_schedule_273_0_e3120: f64 = (noise_metadata_schedule_273_0_e3119).exp();
        let noise_metadata_schedule_273_0_e3124: f64 = (w[144] - w[63]);
        let noise_metadata_schedule_273_0_e3126: f64 = (noise_metadata_schedule_273_0_e3124 * w[112]);
        let noise_metadata_schedule_273_0_e3127: f64 = (1.0 + noise_metadata_schedule_273_0_e3126);
        let noise_metadata_schedule_273_0_e3128: f64 = (noise_metadata_schedule_273_0_e3120 * noise_metadata_schedule_273_0_e3127);
        (noise_metadata_schedule_273_0_e3128,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_273_0_e3130;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_274_0_e3146,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_274_0_e3135: f64 = (params[32] * w[109]);
        let noise_metadata_schedule_274_0_e3138: f64 = (1.0 - params[32]);
        let noise_metadata_schedule_274_0_e3140: f64 = (noise_metadata_schedule_274_0_e3138 * w[111]);
        let noise_metadata_schedule_274_0_e3141: f64 = (noise_metadata_schedule_274_0_e3135 + noise_metadata_schedule_274_0_e3140);
        let noise_metadata_schedule_274_0_e3143: f64 = (noise_metadata_schedule_274_0_e3141 - 1.0);
        let noise_metadata_schedule_274_0_e3144: f64 = (w[5] * noise_metadata_schedule_274_0_e3143);
        (noise_metadata_schedule_274_0_e3144,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_274_0_e3146;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_275_0_e3152,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_275_0_e3150: f64 = (w[82] * w[47]);
        (noise_metadata_schedule_275_0_e3150,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_275_0_e3152;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_276_0_e3160,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_276_0_e3157: f64 = (4.0 * w[85]);
        let noise_metadata_schedule_276_0_e3158: f64 = (1.0 + noise_metadata_schedule_276_0_e3157);
        (noise_metadata_schedule_276_0_e3158,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_276_0_e3160;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_277_0_e3163: f64 = if w[108] > 1e-8 { 1.0 } else { 0.0 };
            w[258] = noise_metadata_schedule_277_0_e3163;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_278_0_e3174,) = {
    if ((w[255] != 0.0) && (w[258] != 0.0)) {
        let noise_metadata_schedule_278_0_e3170: f64 = (w[108]).sqrt();
        let noise_metadata_schedule_278_0_e3171: f64 = (1.0 + noise_metadata_schedule_278_0_e3170);
        let noise_metadata_schedule_278_0_e3172: f64 = (0.5 * noise_metadata_schedule_278_0_e3171);
        (noise_metadata_schedule_278_0_e3172,)
    } else {
        (w[86],)
    }
};
            w[86] = noise_metadata_schedule_278_0_e3174;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_279_0_e3186,) = {
    if ((w[255] != 0.0) && (w[258] == 0.0)) {
        let noise_metadata_schedule_279_0_e3182: f64 = (1e-8_f64).sqrt();
        let noise_metadata_schedule_279_0_e3183: f64 = (1.0 + noise_metadata_schedule_279_0_e3182);
        let noise_metadata_schedule_279_0_e3184: f64 = (0.5 * noise_metadata_schedule_279_0_e3183);
        (noise_metadata_schedule_279_0_e3184,)
    } else {
        (w[86],)
    }
};
            w[86] = noise_metadata_schedule_279_0_e3186;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_281_0_e3196,) = {
    if (w[255] == 0.0) {
        (1.0,)
    } else {
        (w[86],)
    }
};
            w[86] = noise_metadata_schedule_281_0_e3196;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_282_0_e3199: f64 = if params[55] == 1.0 { 1.0 } else { 0.0 };
            w[259] = noise_metadata_schedule_282_0_e3199;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_283_0_e3207,) = {
    if (w[259] != 0.0) {
        let noise_metadata_schedule_283_0_e3204: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_283_0_e3205: f64 = (1.0 / noise_metadata_schedule_283_0_e3204);
        (noise_metadata_schedule_283_0_e3205,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_283_0_e3207;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_284_0_e3210: f64 = if w[143] < w[65] { 1.0 } else { 0.0 };
            w[260] = noise_metadata_schedule_284_0_e3210;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_285_0_e3219,) = {
    if ((w[259] != 0.0) && (w[260] != 0.0)) {
        let noise_metadata_schedule_285_0_e3216: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_285_0_e3217: f64 = (noise_metadata_schedule_285_0_e3216).exp();
        (noise_metadata_schedule_285_0_e3217,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_285_0_e3219;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_286_0_e3237,) = {
    if ((w[259] != 0.0) && (w[260] == 0.0)) {
        let noise_metadata_schedule_286_0_e3226: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_286_0_e3227: f64 = (noise_metadata_schedule_286_0_e3226).exp();
        let noise_metadata_schedule_286_0_e3231: f64 = (w[143] - w[65]);
        let noise_metadata_schedule_286_0_e3233: f64 = (noise_metadata_schedule_286_0_e3231 * w[112]);
        let noise_metadata_schedule_286_0_e3234: f64 = (1.0 + noise_metadata_schedule_286_0_e3233);
        let noise_metadata_schedule_286_0_e3235: f64 = (noise_metadata_schedule_286_0_e3227 * noise_metadata_schedule_286_0_e3234);
        (noise_metadata_schedule_286_0_e3235,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_286_0_e3237;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_287_0_e3245,) = {
    if (w[259] != 0.0) {
        let noise_metadata_schedule_287_0_e3242: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_287_0_e3243: f64 = (1.0 / noise_metadata_schedule_287_0_e3242);
        (noise_metadata_schedule_287_0_e3243,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_287_0_e3245;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_288_0_e3248: f64 = if w[143] < w[66] { 1.0 } else { 0.0 };
            w[261] = noise_metadata_schedule_288_0_e3248;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_289_0_e3257,) = {
    if ((w[259] != 0.0) && (w[261] != 0.0)) {
        let noise_metadata_schedule_289_0_e3254: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_289_0_e3255: f64 = (noise_metadata_schedule_289_0_e3254).exp();
        (noise_metadata_schedule_289_0_e3255,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_289_0_e3257;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_290_0_e3275,) = {
    if ((w[259] != 0.0) && (w[261] == 0.0)) {
        let noise_metadata_schedule_290_0_e3264: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_290_0_e3265: f64 = (noise_metadata_schedule_290_0_e3264).exp();
        let noise_metadata_schedule_290_0_e3269: f64 = (w[143] - w[66]);
        let noise_metadata_schedule_290_0_e3271: f64 = (noise_metadata_schedule_290_0_e3269 * w[112]);
        let noise_metadata_schedule_290_0_e3272: f64 = (1.0 + noise_metadata_schedule_290_0_e3271);
        let noise_metadata_schedule_290_0_e3273: f64 = (noise_metadata_schedule_290_0_e3265 * noise_metadata_schedule_290_0_e3272);
        (noise_metadata_schedule_290_0_e3273,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_290_0_e3275;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_291_0_e3278: f64 = if params[57] > 0.0 { 1.0 } else { 0.0 };
            w[262] = noise_metadata_schedule_291_0_e3278;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_292_0_e3302,) = {
    if ((w[259] != 0.0) && (w[262] != 0.0)) {
        let noise_metadata_schedule_292_0_e3287: f64 = (w[79] - 1.0);
        let noise_metadata_schedule_292_0_e3288: f64 = (params[57] * noise_metadata_schedule_292_0_e3287);
        let noise_metadata_schedule_292_0_e3289: f64 = (1.0 + noise_metadata_schedule_292_0_e3288);
        let noise_metadata_schedule_292_0_e3290: f64 = (w[3] * noise_metadata_schedule_292_0_e3289);
        let noise_metadata_schedule_292_0_e3293: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_292_0_e3294: f64 = (noise_metadata_schedule_292_0_e3290 * noise_metadata_schedule_292_0_e3293);
        let noise_metadata_schedule_292_0_e3298: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_292_0_e3299: f64 = (w[6] * noise_metadata_schedule_292_0_e3298);
        let noise_metadata_schedule_292_0_e3300: f64 = (noise_metadata_schedule_292_0_e3294 + noise_metadata_schedule_292_0_e3299);
        (noise_metadata_schedule_292_0_e3300,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_292_0_e3302;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_293_0_e3319,) = {
    if ((w[259] != 0.0) && (w[262] == 0.0)) {
        let noise_metadata_schedule_293_0_e3310: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_293_0_e3311: f64 = (w[3] * noise_metadata_schedule_293_0_e3310);
        let noise_metadata_schedule_293_0_e3315: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_293_0_e3316: f64 = (w[6] * noise_metadata_schedule_293_0_e3315);
        let noise_metadata_schedule_293_0_e3317: f64 = (noise_metadata_schedule_293_0_e3311 + noise_metadata_schedule_293_0_e3316);
        (noise_metadata_schedule_293_0_e3317,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_293_0_e3319;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_294_0_e3322: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[263] = noise_metadata_schedule_294_0_e3322;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_295_0_e3331,) = {
    if ((w[259] != 0.0) && (w[263] != 0.0)) {
        let noise_metadata_schedule_295_0_e3327: f64 = (-w[31]);
        let noise_metadata_schedule_295_0_e3329: f64 = (noise_metadata_schedule_295_0_e3327 - w[143]);
        (noise_metadata_schedule_295_0_e3329,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_295_0_e3331;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_296_0_e3341,) = {
    if ((w[259] != 0.0) && (w[263] != 0.0)) {
        let noise_metadata_schedule_296_0_e3338: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_296_0_e3339: f64 = (1.0 / noise_metadata_schedule_296_0_e3338);
        (noise_metadata_schedule_296_0_e3339,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_296_0_e3341;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_297_0_e3344: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[264] = noise_metadata_schedule_297_0_e3344;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_298_0_e3355,) = {
    if (((w[259] != 0.0) && (w[263] != 0.0)) && (w[264] != 0.0)) {
        let noise_metadata_schedule_298_0_e3352: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_298_0_e3353: f64 = (noise_metadata_schedule_298_0_e3352).exp();
        (noise_metadata_schedule_298_0_e3353,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_298_0_e3355;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_299_0_e3375,) = {
    if (((w[259] != 0.0) && (w[263] != 0.0)) && (w[264] == 0.0)) {
        let noise_metadata_schedule_299_0_e3364: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_299_0_e3365: f64 = (noise_metadata_schedule_299_0_e3364).exp();
        let noise_metadata_schedule_299_0_e3369: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_299_0_e3371: f64 = (noise_metadata_schedule_299_0_e3369 * w[112]);
        let noise_metadata_schedule_299_0_e3372: f64 = (1.0 + noise_metadata_schedule_299_0_e3371);
        let noise_metadata_schedule_299_0_e3373: f64 = (noise_metadata_schedule_299_0_e3365 * noise_metadata_schedule_299_0_e3372);
        (noise_metadata_schedule_299_0_e3373,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_299_0_e3375;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_300_0_e3387,) = {
    if ((w[259] != 0.0) && (w[263] != 0.0)) {
        let noise_metadata_schedule_300_0_e3383: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_300_0_e3384: f64 = (params[90] * noise_metadata_schedule_300_0_e3383);
        let noise_metadata_schedule_300_0_e3385: f64 = (w[87] - noise_metadata_schedule_300_0_e3384);
        (noise_metadata_schedule_300_0_e3385,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_300_0_e3387;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_301_0_e3391,) = {
    if (w[259] != 0.0) {
        (0.0,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_301_0_e3391;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_302_0_e3394: f64 = if params[55] == 0.0 { 1.0 } else { 0.0 };
            w[265] = noise_metadata_schedule_302_0_e3394;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_303_0_e3401,) = {
    if ((w[259] == 0.0) && (w[265] != 0.0)) {
        (0.0,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_303_0_e3401;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_304_0_e3412,) = {
    if ((w[259] == 0.0) && (w[265] != 0.0)) {
        let noise_metadata_schedule_304_0_e3409: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_304_0_e3410: f64 = (1.0 / noise_metadata_schedule_304_0_e3409);
        (noise_metadata_schedule_304_0_e3410,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_304_0_e3412;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_305_0_e3415: f64 = if w[145] < w[65] { 1.0 } else { 0.0 };
            w[266] = noise_metadata_schedule_305_0_e3415;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_306_0_e3427,) = {
    if (((w[259] == 0.0) && (w[265] != 0.0)) && (w[266] != 0.0)) {
        let noise_metadata_schedule_306_0_e3424: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_306_0_e3425: f64 = (noise_metadata_schedule_306_0_e3424).exp();
        (noise_metadata_schedule_306_0_e3425,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_306_0_e3427;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_307_0_e3448,) = {
    if (((w[259] == 0.0) && (w[265] != 0.0)) && (w[266] == 0.0)) {
        let noise_metadata_schedule_307_0_e3437: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_307_0_e3438: f64 = (noise_metadata_schedule_307_0_e3437).exp();
        let noise_metadata_schedule_307_0_e3442: f64 = (w[145] - w[65]);
        let noise_metadata_schedule_307_0_e3444: f64 = (noise_metadata_schedule_307_0_e3442 * w[112]);
        let noise_metadata_schedule_307_0_e3445: f64 = (1.0 + noise_metadata_schedule_307_0_e3444);
        let noise_metadata_schedule_307_0_e3446: f64 = (noise_metadata_schedule_307_0_e3438 * noise_metadata_schedule_307_0_e3445);
        (noise_metadata_schedule_307_0_e3446,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_307_0_e3448;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_308_0_e3459,) = {
    if ((w[259] == 0.0) && (w[265] != 0.0)) {
        let noise_metadata_schedule_308_0_e3456: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_308_0_e3457: f64 = (1.0 / noise_metadata_schedule_308_0_e3456);
        (noise_metadata_schedule_308_0_e3457,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_308_0_e3459;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_309_0_e3462: f64 = if w[145] < w[66] { 1.0 } else { 0.0 };
            w[267] = noise_metadata_schedule_309_0_e3462;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_310_0_e3474,) = {
    if (((w[259] == 0.0) && (w[265] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_310_0_e3471: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_310_0_e3472: f64 = (noise_metadata_schedule_310_0_e3471).exp();
        (noise_metadata_schedule_310_0_e3472,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_310_0_e3474;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_311_0_e3495,) = {
    if (((w[259] == 0.0) && (w[265] != 0.0)) && (w[267] == 0.0)) {
        let noise_metadata_schedule_311_0_e3484: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_311_0_e3485: f64 = (noise_metadata_schedule_311_0_e3484).exp();
        let noise_metadata_schedule_311_0_e3489: f64 = (w[145] - w[66]);
        let noise_metadata_schedule_311_0_e3491: f64 = (noise_metadata_schedule_311_0_e3489 * w[112]);
        let noise_metadata_schedule_311_0_e3492: f64 = (1.0 + noise_metadata_schedule_311_0_e3491);
        let noise_metadata_schedule_311_0_e3493: f64 = (noise_metadata_schedule_311_0_e3485 * noise_metadata_schedule_311_0_e3492);
        (noise_metadata_schedule_311_0_e3493,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_311_0_e3495;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_312_0_e3512,) = {
    if ((w[259] == 0.0) && (w[265] != 0.0)) {
        let noise_metadata_schedule_312_0_e3503: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_312_0_e3504: f64 = (w[3] * noise_metadata_schedule_312_0_e3503);
        let noise_metadata_schedule_312_0_e3508: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_312_0_e3509: f64 = (w[6] * noise_metadata_schedule_312_0_e3508);
        let noise_metadata_schedule_312_0_e3510: f64 = (noise_metadata_schedule_312_0_e3504 + noise_metadata_schedule_312_0_e3509);
        (noise_metadata_schedule_312_0_e3510,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_312_0_e3512;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_313_0_e3515: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[268] = noise_metadata_schedule_313_0_e3515;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_314_0_e3527,) = {
    if (((w[259] == 0.0) && (w[265] != 0.0)) && (w[268] != 0.0)) {
        let noise_metadata_schedule_314_0_e3523: f64 = (-w[31]);
        let noise_metadata_schedule_314_0_e3525: f64 = (noise_metadata_schedule_314_0_e3523 - w[143]);
        (noise_metadata_schedule_314_0_e3525,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_314_0_e3527;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_315_0_e3540,) = {
    if (((w[259] == 0.0) && (w[265] != 0.0)) && (w[268] != 0.0)) {
        let noise_metadata_schedule_315_0_e3537: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_315_0_e3538: f64 = (1.0 / noise_metadata_schedule_315_0_e3537);
        (noise_metadata_schedule_315_0_e3538,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_315_0_e3540;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_316_0_e3543: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[269] = noise_metadata_schedule_316_0_e3543;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_317_0_e3557,) = {
    if ((((w[259] == 0.0) && (w[265] != 0.0)) && (w[268] != 0.0)) && (w[269] != 0.0)) {
        let noise_metadata_schedule_317_0_e3554: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_317_0_e3555: f64 = (noise_metadata_schedule_317_0_e3554).exp();
        (noise_metadata_schedule_317_0_e3555,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_317_0_e3557;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_318_0_e3580,) = {
    if ((((w[259] == 0.0) && (w[265] != 0.0)) && (w[268] != 0.0)) && (w[269] == 0.0)) {
        let noise_metadata_schedule_318_0_e3569: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_318_0_e3570: f64 = (noise_metadata_schedule_318_0_e3569).exp();
        let noise_metadata_schedule_318_0_e3574: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_318_0_e3576: f64 = (noise_metadata_schedule_318_0_e3574 * w[112]);
        let noise_metadata_schedule_318_0_e3577: f64 = (1.0 + noise_metadata_schedule_318_0_e3576);
        let noise_metadata_schedule_318_0_e3578: f64 = (noise_metadata_schedule_318_0_e3570 * noise_metadata_schedule_318_0_e3577);
        (noise_metadata_schedule_318_0_e3578,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_318_0_e3580;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_319_0_e3595,) = {
    if (((w[259] == 0.0) && (w[265] != 0.0)) && (w[268] != 0.0)) {
        let noise_metadata_schedule_319_0_e3591: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_319_0_e3592: f64 = (params[90] * noise_metadata_schedule_319_0_e3591);
        let noise_metadata_schedule_319_0_e3593: f64 = (w[88] - noise_metadata_schedule_319_0_e3592);
        (noise_metadata_schedule_319_0_e3593,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_319_0_e3595;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_320_0_e3607,) = {
    if ((w[259] == 0.0) && (w[265] == 0.0)) {
        let noise_metadata_schedule_320_0_e3604: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_320_0_e3605: f64 = (1.0 / noise_metadata_schedule_320_0_e3604);
        (noise_metadata_schedule_320_0_e3605,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_320_0_e3607;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_321_0_e3610: f64 = if w[143] < w[65] { 1.0 } else { 0.0 };
            w[270] = noise_metadata_schedule_321_0_e3610;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_322_0_e3623,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_322_0_e3620: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_322_0_e3621: f64 = (noise_metadata_schedule_322_0_e3620).exp();
        (noise_metadata_schedule_322_0_e3621,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_322_0_e3623;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_323_0_e3645,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[270] == 0.0)) {
        let noise_metadata_schedule_323_0_e3634: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_323_0_e3635: f64 = (noise_metadata_schedule_323_0_e3634).exp();
        let noise_metadata_schedule_323_0_e3639: f64 = (w[143] - w[65]);
        let noise_metadata_schedule_323_0_e3641: f64 = (noise_metadata_schedule_323_0_e3639 * w[112]);
        let noise_metadata_schedule_323_0_e3642: f64 = (1.0 + noise_metadata_schedule_323_0_e3641);
        let noise_metadata_schedule_323_0_e3643: f64 = (noise_metadata_schedule_323_0_e3635 * noise_metadata_schedule_323_0_e3642);
        (noise_metadata_schedule_323_0_e3643,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_323_0_e3645;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_324_0_e3657,) = {
    if ((w[259] == 0.0) && (w[265] == 0.0)) {
        let noise_metadata_schedule_324_0_e3654: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_324_0_e3655: f64 = (1.0 / noise_metadata_schedule_324_0_e3654);
        (noise_metadata_schedule_324_0_e3655,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_324_0_e3657;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_325_0_e3660: f64 = if w[143] < w[66] { 1.0 } else { 0.0 };
            w[271] = noise_metadata_schedule_325_0_e3660;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_326_0_e3673,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[271] != 0.0)) {
        let noise_metadata_schedule_326_0_e3670: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_326_0_e3671: f64 = (noise_metadata_schedule_326_0_e3670).exp();
        (noise_metadata_schedule_326_0_e3671,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_326_0_e3673;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_327_0_e3695,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[271] == 0.0)) {
        let noise_metadata_schedule_327_0_e3684: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_327_0_e3685: f64 = (noise_metadata_schedule_327_0_e3684).exp();
        let noise_metadata_schedule_327_0_e3689: f64 = (w[143] - w[66]);
        let noise_metadata_schedule_327_0_e3691: f64 = (noise_metadata_schedule_327_0_e3689 * w[112]);
        let noise_metadata_schedule_327_0_e3692: f64 = (1.0 + noise_metadata_schedule_327_0_e3691);
        let noise_metadata_schedule_327_0_e3693: f64 = (noise_metadata_schedule_327_0_e3685 * noise_metadata_schedule_327_0_e3692);
        (noise_metadata_schedule_327_0_e3693,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_327_0_e3695;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_328_0_e3698: f64 = if params[57] > 0.0 { 1.0 } else { 0.0 };
            w[272] = noise_metadata_schedule_328_0_e3698;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_329_0_e3728,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_329_0_e3712: f64 = (w[79] - 1.0);
        let noise_metadata_schedule_329_0_e3713: f64 = (params[57] * noise_metadata_schedule_329_0_e3712);
        let noise_metadata_schedule_329_0_e3714: f64 = (1.0 + noise_metadata_schedule_329_0_e3713);
        let noise_metadata_schedule_329_0_e3715: f64 = (w[3] * noise_metadata_schedule_329_0_e3714);
        let noise_metadata_schedule_329_0_e3718: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_329_0_e3719: f64 = (noise_metadata_schedule_329_0_e3715 * noise_metadata_schedule_329_0_e3718);
        let noise_metadata_schedule_329_0_e3723: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_329_0_e3724: f64 = (w[6] * noise_metadata_schedule_329_0_e3723);
        let noise_metadata_schedule_329_0_e3725: f64 = (noise_metadata_schedule_329_0_e3719 + noise_metadata_schedule_329_0_e3724);
        let noise_metadata_schedule_329_0_e3726: f64 = (params[55] * noise_metadata_schedule_329_0_e3725);
        (noise_metadata_schedule_329_0_e3726,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_329_0_e3728;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_330_0_e3751,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[272] == 0.0)) {
        let noise_metadata_schedule_330_0_e3741: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_330_0_e3742: f64 = (w[3] * noise_metadata_schedule_330_0_e3741);
        let noise_metadata_schedule_330_0_e3746: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_330_0_e3747: f64 = (w[6] * noise_metadata_schedule_330_0_e3746);
        let noise_metadata_schedule_330_0_e3748: f64 = (noise_metadata_schedule_330_0_e3742 + noise_metadata_schedule_330_0_e3747);
        let noise_metadata_schedule_330_0_e3749: f64 = (params[55] * noise_metadata_schedule_330_0_e3748);
        (noise_metadata_schedule_330_0_e3749,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_330_0_e3751;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_331_0_e3754: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[273] = noise_metadata_schedule_331_0_e3754;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_332_0_e3767,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[273] != 0.0)) {
        let noise_metadata_schedule_332_0_e3763: f64 = (-w[31]);
        let noise_metadata_schedule_332_0_e3765: f64 = (noise_metadata_schedule_332_0_e3763 - w[143]);
        (noise_metadata_schedule_332_0_e3765,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_332_0_e3767;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_333_0_e3781,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[273] != 0.0)) {
        let noise_metadata_schedule_333_0_e3778: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_333_0_e3779: f64 = (1.0 / noise_metadata_schedule_333_0_e3778);
        (noise_metadata_schedule_333_0_e3779,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_333_0_e3781;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_334_0_e3784: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[274] = noise_metadata_schedule_334_0_e3784;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_335_0_e3799,) = {
    if ((((w[259] == 0.0) && (w[265] == 0.0)) && (w[273] != 0.0)) && (w[274] != 0.0)) {
        let noise_metadata_schedule_335_0_e3796: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_335_0_e3797: f64 = (noise_metadata_schedule_335_0_e3796).exp();
        (noise_metadata_schedule_335_0_e3797,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_335_0_e3799;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_336_0_e3823,) = {
    if ((((w[259] == 0.0) && (w[265] == 0.0)) && (w[273] != 0.0)) && (w[274] == 0.0)) {
        let noise_metadata_schedule_336_0_e3812: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_336_0_e3813: f64 = (noise_metadata_schedule_336_0_e3812).exp();
        let noise_metadata_schedule_336_0_e3817: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_336_0_e3819: f64 = (noise_metadata_schedule_336_0_e3817 * w[112]);
        let noise_metadata_schedule_336_0_e3820: f64 = (1.0 + noise_metadata_schedule_336_0_e3819);
        let noise_metadata_schedule_336_0_e3821: f64 = (noise_metadata_schedule_336_0_e3813 * noise_metadata_schedule_336_0_e3820);
        (noise_metadata_schedule_336_0_e3821,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_336_0_e3823;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_337_0_e3841,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[273] != 0.0)) {
        let noise_metadata_schedule_337_0_e3834: f64 = (params[55] * params[90]);
        let noise_metadata_schedule_337_0_e3837: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_337_0_e3838: f64 = (noise_metadata_schedule_337_0_e3834 * noise_metadata_schedule_337_0_e3837);
        let noise_metadata_schedule_337_0_e3839: f64 = (w[87] - noise_metadata_schedule_337_0_e3838);
        (noise_metadata_schedule_337_0_e3839,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_337_0_e3841;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_338_0_e3853,) = {
    if ((w[259] == 0.0) && (w[265] == 0.0)) {
        let noise_metadata_schedule_338_0_e3850: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_338_0_e3851: f64 = (1.0 / noise_metadata_schedule_338_0_e3850);
        (noise_metadata_schedule_338_0_e3851,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_338_0_e3853;
        }
        if (active[0] & 0x16c) != 0 {
            let noise_metadata_schedule_339_0_e3856: f64 = if w[145] < w[65] { 1.0 } else { 0.0 };
            w[275] = noise_metadata_schedule_339_0_e3856;
        }
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_340_0_e3869,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_340_0_e3866: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_340_0_e3867: f64 = (noise_metadata_schedule_340_0_e3866).exp();
        (noise_metadata_schedule_340_0_e3867,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_340_0_e3869;
        }
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_341_0_e3891,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[275] == 0.0)) {
        let noise_metadata_schedule_341_0_e3880: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_341_0_e3881: f64 = (noise_metadata_schedule_341_0_e3880).exp();
        let noise_metadata_schedule_341_0_e3885: f64 = (w[145] - w[65]);
        let noise_metadata_schedule_341_0_e3887: f64 = (noise_metadata_schedule_341_0_e3885 * w[112]);
        let noise_metadata_schedule_341_0_e3888: f64 = (1.0 + noise_metadata_schedule_341_0_e3887);
        let noise_metadata_schedule_341_0_e3889: f64 = (noise_metadata_schedule_341_0_e3881 * noise_metadata_schedule_341_0_e3888);
        (noise_metadata_schedule_341_0_e3889,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_341_0_e3891;
        }
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_342_0_e3903,) = {
    if ((w[259] == 0.0) && (w[265] == 0.0)) {
        let noise_metadata_schedule_342_0_e3900: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_342_0_e3901: f64 = (1.0 / noise_metadata_schedule_342_0_e3900);
        (noise_metadata_schedule_342_0_e3901,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_342_0_e3903;
        }
        if (active[0] & 0x6c) != 0 {
            let noise_metadata_schedule_343_0_e3906: f64 = if w[145] < w[66] { 1.0 } else { 0.0 };
            w[276] = noise_metadata_schedule_343_0_e3906;
        }
        if (active[0] & 0x6c) != 0 {
            let (noise_metadata_schedule_344_0_e3919,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[276] != 0.0)) {
        let noise_metadata_schedule_344_0_e3916: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_344_0_e3917: f64 = (noise_metadata_schedule_344_0_e3916).exp();
        (noise_metadata_schedule_344_0_e3917,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_344_0_e3919;
        }
        if (active[0] & 0x6c) != 0 {
            let (noise_metadata_schedule_345_0_e3941,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[276] == 0.0)) {
        let noise_metadata_schedule_345_0_e3930: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_345_0_e3931: f64 = (noise_metadata_schedule_345_0_e3930).exp();
        let noise_metadata_schedule_345_0_e3935: f64 = (w[145] - w[66]);
        let noise_metadata_schedule_345_0_e3937: f64 = (noise_metadata_schedule_345_0_e3935 * w[112]);
        let noise_metadata_schedule_345_0_e3938: f64 = (1.0 + noise_metadata_schedule_345_0_e3937);
        let noise_metadata_schedule_345_0_e3939: f64 = (noise_metadata_schedule_345_0_e3931 * noise_metadata_schedule_345_0_e3938);
        (noise_metadata_schedule_345_0_e3939,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_345_0_e3941;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_346_0_e3963,) = {
    if ((w[259] == 0.0) && (w[265] == 0.0)) {
        let noise_metadata_schedule_346_0_e3949: f64 = (1.0 - params[55]);
        let noise_metadata_schedule_346_0_e3953: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_346_0_e3954: f64 = (w[3] * noise_metadata_schedule_346_0_e3953);
        let noise_metadata_schedule_346_0_e3958: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_346_0_e3959: f64 = (w[6] * noise_metadata_schedule_346_0_e3958);
        let noise_metadata_schedule_346_0_e3960: f64 = (noise_metadata_schedule_346_0_e3954 + noise_metadata_schedule_346_0_e3959);
        let noise_metadata_schedule_346_0_e3961: f64 = (noise_metadata_schedule_346_0_e3949 * noise_metadata_schedule_346_0_e3960);
        (noise_metadata_schedule_346_0_e3961,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_346_0_e3963;
        }
        if (active[0] & 0x10c) != 0 {
            let noise_metadata_schedule_347_0_e3966: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[277] = noise_metadata_schedule_347_0_e3966;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_348_0_e3979,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[277] != 0.0)) {
        let noise_metadata_schedule_348_0_e3975: f64 = (-w[31]);
        let noise_metadata_schedule_348_0_e3977: f64 = (noise_metadata_schedule_348_0_e3975 - w[143]);
        (noise_metadata_schedule_348_0_e3977,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_348_0_e3979;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_349_0_e3993,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[277] != 0.0)) {
        let noise_metadata_schedule_349_0_e3990: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_349_0_e3991: f64 = (1.0 / noise_metadata_schedule_349_0_e3990);
        (noise_metadata_schedule_349_0_e3991,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_349_0_e3993;
        }
        if (active[0] & 0x10c) != 0 {
            let noise_metadata_schedule_350_0_e3996: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[278] = noise_metadata_schedule_350_0_e3996;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_351_0_e4011,) = {
    if ((((w[259] == 0.0) && (w[265] == 0.0)) && (w[277] != 0.0)) && (w[278] != 0.0)) {
        let noise_metadata_schedule_351_0_e4008: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_351_0_e4009: f64 = (noise_metadata_schedule_351_0_e4008).exp();
        (noise_metadata_schedule_351_0_e4009,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_351_0_e4011;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_352_0_e4035,) = {
    if ((((w[259] == 0.0) && (w[265] == 0.0)) && (w[277] != 0.0)) && (w[278] == 0.0)) {
        let noise_metadata_schedule_352_0_e4024: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_352_0_e4025: f64 = (noise_metadata_schedule_352_0_e4024).exp();
        let noise_metadata_schedule_352_0_e4029: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_352_0_e4031: f64 = (noise_metadata_schedule_352_0_e4029 * w[112]);
        let noise_metadata_schedule_352_0_e4032: f64 = (1.0 + noise_metadata_schedule_352_0_e4031);
        let noise_metadata_schedule_352_0_e4033: f64 = (noise_metadata_schedule_352_0_e4025 * noise_metadata_schedule_352_0_e4032);
        (noise_metadata_schedule_352_0_e4033,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_352_0_e4035;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_353_0_e4055,) = {
    if (((w[259] == 0.0) && (w[265] == 0.0)) && (w[277] != 0.0)) {
        let noise_metadata_schedule_353_0_e4046: f64 = (1.0 - params[55]);
        let noise_metadata_schedule_353_0_e4048: f64 = (noise_metadata_schedule_353_0_e4046 * params[90]);
        let noise_metadata_schedule_353_0_e4051: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_353_0_e4052: f64 = (noise_metadata_schedule_353_0_e4048 * noise_metadata_schedule_353_0_e4051);
        let noise_metadata_schedule_353_0_e4053: f64 = (w[88] - noise_metadata_schedule_353_0_e4052);
        (noise_metadata_schedule_353_0_e4053,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_353_0_e4055;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_354_0_e4059: f64 = (params[61] * w[73]);
            let noise_metadata_schedule_354_0_e4060: f64 = (1.0 / noise_metadata_schedule_354_0_e4059);
            w[112] = noise_metadata_schedule_354_0_e4060;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_355_0_e4063: f64 = if w[144] < w[67] { 1.0 } else { 0.0 };
            w[279] = noise_metadata_schedule_355_0_e4063;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_356_0_e4070,) = {
    if (w[279] != 0.0) {
        let noise_metadata_schedule_356_0_e4067: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_356_0_e4068: f64 = (noise_metadata_schedule_356_0_e4067).exp();
        (noise_metadata_schedule_356_0_e4068,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_356_0_e4070;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_357_0_e4086,) = {
    if (w[279] == 0.0) {
        let noise_metadata_schedule_357_0_e4075: f64 = (w[67] * w[112]);
        let noise_metadata_schedule_357_0_e4076: f64 = (noise_metadata_schedule_357_0_e4075).exp();
        let noise_metadata_schedule_357_0_e4080: f64 = (w[144] - w[67]);
        let noise_metadata_schedule_357_0_e4082: f64 = (noise_metadata_schedule_357_0_e4080 * w[112]);
        let noise_metadata_schedule_357_0_e4083: f64 = (1.0 + noise_metadata_schedule_357_0_e4082);
        let noise_metadata_schedule_357_0_e4084: f64 = (noise_metadata_schedule_357_0_e4076 * noise_metadata_schedule_357_0_e4083);
        (noise_metadata_schedule_357_0_e4084,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_357_0_e4086;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_358_0_e4090: f64 = (params[63] * w[73]);
            let noise_metadata_schedule_358_0_e4091: f64 = (1.0 / noise_metadata_schedule_358_0_e4090);
            w[112] = noise_metadata_schedule_358_0_e4091;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_359_0_e4094: f64 = if w[144] < w[68] { 1.0 } else { 0.0 };
            w[280] = noise_metadata_schedule_359_0_e4094;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_360_0_e4101,) = {
    if (w[280] != 0.0) {
        let noise_metadata_schedule_360_0_e4098: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_360_0_e4099: f64 = (noise_metadata_schedule_360_0_e4098).exp();
        (noise_metadata_schedule_360_0_e4099,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_360_0_e4101;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_361_0_e4117,) = {
    if (w[280] == 0.0) {
        let noise_metadata_schedule_361_0_e4106: f64 = (w[68] * w[112]);
        let noise_metadata_schedule_361_0_e4107: f64 = (noise_metadata_schedule_361_0_e4106).exp();
        let noise_metadata_schedule_361_0_e4111: f64 = (w[144] - w[68]);
        let noise_metadata_schedule_361_0_e4113: f64 = (noise_metadata_schedule_361_0_e4111 * w[112]);
        let noise_metadata_schedule_361_0_e4114: f64 = (1.0 + noise_metadata_schedule_361_0_e4113);
        let noise_metadata_schedule_361_0_e4115: f64 = (noise_metadata_schedule_361_0_e4107 * noise_metadata_schedule_361_0_e4114);
        (noise_metadata_schedule_361_0_e4115,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_361_0_e4117;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_363_0_e4135: f64 = if ((params[64] > 0.0) || (params[65] > 0.0)) { 1.0 } else { 0.0 };
            w[281] = noise_metadata_schedule_363_0_e4135;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_364_0_e4143,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_364_0_e4140: f64 = (params[61] * w[73]);
        let noise_metadata_schedule_364_0_e4141: f64 = (1.0 / noise_metadata_schedule_364_0_e4140);
        (noise_metadata_schedule_364_0_e4141,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_364_0_e4143;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_365_0_e4146: f64 = if w[146] < w[69] { 1.0 } else { 0.0 };
            w[282] = noise_metadata_schedule_365_0_e4146;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_366_0_e4155,) = {
    if ((w[281] != 0.0) && (w[282] != 0.0)) {
        let noise_metadata_schedule_366_0_e4152: f64 = (w[146] * w[112]);
        let noise_metadata_schedule_366_0_e4153: f64 = (noise_metadata_schedule_366_0_e4152).exp();
        (noise_metadata_schedule_366_0_e4153,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_366_0_e4155;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_367_0_e4173,) = {
    if ((w[281] != 0.0) && (w[282] == 0.0)) {
        let noise_metadata_schedule_367_0_e4162: f64 = (w[69] * w[112]);
        let noise_metadata_schedule_367_0_e4163: f64 = (noise_metadata_schedule_367_0_e4162).exp();
        let noise_metadata_schedule_367_0_e4167: f64 = (w[146] - w[69]);
        let noise_metadata_schedule_367_0_e4169: f64 = (noise_metadata_schedule_367_0_e4167 * w[112]);
        let noise_metadata_schedule_367_0_e4170: f64 = (1.0 + noise_metadata_schedule_367_0_e4169);
        let noise_metadata_schedule_367_0_e4171: f64 = (noise_metadata_schedule_367_0_e4163 * noise_metadata_schedule_367_0_e4170);
        (noise_metadata_schedule_367_0_e4171,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_367_0_e4173;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_368_0_e4181,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_368_0_e4178: f64 = (params[63] * w[73]);
        let noise_metadata_schedule_368_0_e4179: f64 = (1.0 / noise_metadata_schedule_368_0_e4178);
        (noise_metadata_schedule_368_0_e4179,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_368_0_e4181;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_369_0_e4184: f64 = if w[146] < w[70] { 1.0 } else { 0.0 };
            w[283] = noise_metadata_schedule_369_0_e4184;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_370_0_e4193,) = {
    if ((w[281] != 0.0) && (w[283] != 0.0)) {
        let noise_metadata_schedule_370_0_e4190: f64 = (w[146] * w[112]);
        let noise_metadata_schedule_370_0_e4191: f64 = (noise_metadata_schedule_370_0_e4190).exp();
        (noise_metadata_schedule_370_0_e4191,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_370_0_e4193;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_371_0_e4211,) = {
    if ((w[281] != 0.0) && (w[283] == 0.0)) {
        let noise_metadata_schedule_371_0_e4200: f64 = (w[70] * w[112]);
        let noise_metadata_schedule_371_0_e4201: f64 = (noise_metadata_schedule_371_0_e4200).exp();
        let noise_metadata_schedule_371_0_e4205: f64 = (w[146] - w[70]);
        let noise_metadata_schedule_371_0_e4207: f64 = (noise_metadata_schedule_371_0_e4205 * w[112]);
        let noise_metadata_schedule_371_0_e4208: f64 = (1.0 + noise_metadata_schedule_371_0_e4207);
        let noise_metadata_schedule_371_0_e4209: f64 = (noise_metadata_schedule_371_0_e4201 * noise_metadata_schedule_371_0_e4208);
        (noise_metadata_schedule_371_0_e4209,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_371_0_e4211;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_372_0_e4225,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_372_0_e4216: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_372_0_e4217: f64 = (w[8] * noise_metadata_schedule_372_0_e4216);
        let noise_metadata_schedule_372_0_e4221: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_372_0_e4222: f64 = (w[9] * noise_metadata_schedule_372_0_e4221);
        let noise_metadata_schedule_372_0_e4223: f64 = (noise_metadata_schedule_372_0_e4217 + noise_metadata_schedule_372_0_e4222);
        (noise_metadata_schedule_372_0_e4223,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_372_0_e4225;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_373_0_e4230,) = {
    if (w[281] == 0.0) {
        (0.0,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_373_0_e4230;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_374_0_e4233: f64 = (w[144] / w[73]);
            w[108] = noise_metadata_schedule_374_0_e4233;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_375_0_e4236: f64 = if w[108] < w[113] { 1.0 } else { 0.0 };
            w[284] = noise_metadata_schedule_375_0_e4236;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_376_0_e4241,) = {
    if (w[284] != 0.0) {
        let noise_metadata_schedule_376_0_e4239: f64 = (w[108]).exp();
        (noise_metadata_schedule_376_0_e4239,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_376_0_e4241;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_377_0_e4253,) = {
    if (w[284] == 0.0) {
        let noise_metadata_schedule_377_0_e4245: f64 = (w[113]).exp();
        let noise_metadata_schedule_377_0_e4249: f64 = (w[108] - w[113]);
        let noise_metadata_schedule_377_0_e4250: f64 = (1.0 + noise_metadata_schedule_377_0_e4249);
        let noise_metadata_schedule_377_0_e4251: f64 = (noise_metadata_schedule_377_0_e4245 * noise_metadata_schedule_377_0_e4250);
        (noise_metadata_schedule_377_0_e4251,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_377_0_e4253;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_378_0_e4256: f64 = (w[148] / w[73]);
            w[108] = noise_metadata_schedule_378_0_e4256;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_379_0_e4259: f64 = if w[108] < w[113] { 1.0 } else { 0.0 };
            w[285] = noise_metadata_schedule_379_0_e4259;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_380_0_e4264,) = {
    if (w[285] != 0.0) {
        let noise_metadata_schedule_380_0_e4262: f64 = (w[108]).exp();
        (noise_metadata_schedule_380_0_e4262,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_380_0_e4264;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_381_0_e4276,) = {
    if (w[285] == 0.0) {
        let noise_metadata_schedule_381_0_e4268: f64 = (w[113]).exp();
        let noise_metadata_schedule_381_0_e4272: f64 = (w[108] - w[113]);
        let noise_metadata_schedule_381_0_e4273: f64 = (1.0 + noise_metadata_schedule_381_0_e4272);
        let noise_metadata_schedule_381_0_e4274: f64 = (noise_metadata_schedule_381_0_e4268 * noise_metadata_schedule_381_0_e4273);
        (noise_metadata_schedule_381_0_e4274,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_381_0_e4276;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_382_0_e4280: f64 = (w[33] * w[109]);
            let noise_metadata_schedule_382_0_e4281: f64 = (1.0 + noise_metadata_schedule_382_0_e4280);
            let noise_metadata_schedule_382_0_e4282: f64 = (noise_metadata_schedule_382_0_e4281).sqrt();
            w[103] = noise_metadata_schedule_382_0_e4282;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_383_0_e4286: f64 = (w[33] * w[111]);
            let noise_metadata_schedule_383_0_e4287: f64 = (1.0 + noise_metadata_schedule_383_0_e4286);
            let noise_metadata_schedule_383_0_e4288: f64 = (noise_metadata_schedule_383_0_e4287).sqrt();
            w[104] = noise_metadata_schedule_383_0_e4288;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_385_0_e4294: f64 = (w[103] + 1.0);
            let noise_metadata_schedule_385_0_e4297: f64 = (w[104] + 1.0);
            let noise_metadata_schedule_385_0_e4298: f64 = (noise_metadata_schedule_385_0_e4294 / noise_metadata_schedule_385_0_e4297);
            w[105] = noise_metadata_schedule_385_0_e4298;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_386_0_e4303: f64 = (w[103] - w[104]);
            let noise_metadata_schedule_386_0_e4305: f64 = (w[105]).ln();
            let noise_metadata_schedule_386_0_e4306: f64 = (noise_metadata_schedule_386_0_e4303 - noise_metadata_schedule_386_0_e4305);
            let noise_metadata_schedule_386_0_e4307: f64 = (w[73] * noise_metadata_schedule_386_0_e4306);
            let noise_metadata_schedule_386_0_e4308: f64 = (w[154] + noise_metadata_schedule_386_0_e4307);
            let noise_metadata_schedule_386_0_e4310: f64 = (noise_metadata_schedule_386_0_e4308 * w[54]);
            w[106] = noise_metadata_schedule_386_0_e4310;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_387_0_e4313: f64 = (w[48] * w[106]);
            let noise_metadata_schedule_387_0_e4318: f64 = (0.5 * w[48]);
            let noise_metadata_schedule_387_0_e4320: f64 = (noise_metadata_schedule_387_0_e4318 * w[49]);
            let noise_metadata_schedule_387_0_e4323: f64 = (w[154] * w[154]);
            let noise_metadata_schedule_387_0_e4325: f64 = (noise_metadata_schedule_387_0_e4323 + 0.01);
            let noise_metadata_schedule_387_0_e4326: f64 = (noise_metadata_schedule_387_0_e4325).sqrt();
            let noise_metadata_schedule_387_0_e4327: f64 = (noise_metadata_schedule_387_0_e4320 * noise_metadata_schedule_387_0_e4326);
            let noise_metadata_schedule_387_0_e4328: f64 = (1.0 + noise_metadata_schedule_387_0_e4327);
            let noise_metadata_schedule_387_0_e4329: f64 = (w[54] * noise_metadata_schedule_387_0_e4328);
            let noise_metadata_schedule_387_0_e4330: f64 = (noise_metadata_schedule_387_0_e4313 / noise_metadata_schedule_387_0_e4329);
            w[107] = noise_metadata_schedule_387_0_e4330;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_388_0_e4335: f64 = (w[107] * w[107]);
            let noise_metadata_schedule_388_0_e4336: f64 = (1.0 + noise_metadata_schedule_388_0_e4335);
            let noise_metadata_schedule_388_0_e4337: f64 = (noise_metadata_schedule_388_0_e4336).sqrt();
            let noise_metadata_schedule_388_0_e4338: f64 = (w[106] / noise_metadata_schedule_388_0_e4337);
            w[97] = noise_metadata_schedule_388_0_e4338;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_429_0_e4707: f64 = (w[165] * w[143]);
            let noise_metadata_schedule_429_0_e4708: f64 = (w[87] + noise_metadata_schedule_429_0_e4707);
            w[87] = noise_metadata_schedule_429_0_e4708;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_430_0_e4712: f64 = (w[165] * w[145]);
            let noise_metadata_schedule_430_0_e4713: f64 = (w[88] + noise_metadata_schedule_430_0_e4712);
            w[88] = noise_metadata_schedule_430_0_e4713;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_431_0_e4717: f64 = (w[165] * w[146]);
            let noise_metadata_schedule_431_0_e4718: f64 = (w[91] + noise_metadata_schedule_431_0_e4717);
            w[91] = noise_metadata_schedule_431_0_e4718;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_434_0_e4731: f64 = w[162];
            let noise_metadata_schedule_434_0_e4733: f64 = (noise_metadata_schedule_434_0_e4731 * w[87]);
            w[87] = noise_metadata_schedule_434_0_e4733;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_435_0_e4736: f64 = w[162];
            let noise_metadata_schedule_435_0_e4738: f64 = (noise_metadata_schedule_435_0_e4736 * w[88]);
            w[88] = noise_metadata_schedule_435_0_e4738;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 341], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_436_0_e4741: f64 = w[162];
            let noise_metadata_schedule_436_0_e4743: f64 = (noise_metadata_schedule_436_0_e4741 * w[76]);
            w[76] = noise_metadata_schedule_436_0_e4743;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_441_0_e4766: f64 = w[162];
            let noise_metadata_schedule_441_0_e4768: f64 = (noise_metadata_schedule_441_0_e4766 * w[91]);
            w[91] = noise_metadata_schedule_441_0_e4768;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_443_0_e4774: f64 = w[162];
            let noise_metadata_schedule_443_0_e4776: f64 = (noise_metadata_schedule_443_0_e4774 * w[97]);
            w[97] = noise_metadata_schedule_443_0_e4776;
        }
    }
}

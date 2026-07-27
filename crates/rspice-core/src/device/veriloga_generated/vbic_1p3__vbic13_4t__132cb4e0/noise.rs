#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 15] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI_SHOT_NOISE", label: Some("Ibei shot noise"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_IBEI_FLICKER_NOISE", label: Some("Ibei flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_EI_IBEX_SHOT_NOISE", label: Some("Ibex shot noise"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_EI_IBEX_FLICKER_NOISE", label: Some("Ibex flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BP_IBEP_SHOT_NOISE", label: Some("Ibep shot noise"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BX_BP_IBEP_FLICKER_NOISE", label: Some("Ibep flicker noise"), kind: GeneratedNoiseKind::Flicker, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CX_RCX_THERMAL_NOISE", label: Some("rcx thermal noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CX_CI_RCI_THERMAL_NOISE", label: Some("rci thermal noise"), kind: GeneratedNoiseKind::White, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BX_RBX_THERMAL_NOISE", label: Some("rbx thermal noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_BI_RBI_THERMAL_NOISE", label: Some("rbi thermal noise"), kind: GeneratedNoiseKind::White, equation: 41, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE_THERMAL_NOISE", label: Some("re thermal noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CX_RBP_THERMAL_NOISE", label: Some("rbp thermal noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "cx", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BX_SI_PARASITIC_TRANSPORT_CURRENT_SHOT_NOISE", label: Some("parasitic transport current shot noise"), kind: GeneratedNoiseKind::White, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bx", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS_THERMAL_NOISE", label: Some("rs thermal noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 359];
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
        let noise_source_13_active = {
            params[1] != 0.0
        };
        let noise_source_14_active = {
            params[1] != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active, noise_source_4_active, noise_source_5_active, noise_source_6_active, noise_source_7_active, noise_source_8_active, noise_source_9_active, noise_source_10_active, noise_source_11_active, noise_source_12_active, noise_source_13_active, noise_source_14_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3) | ((noise_source_4_active as u128) << 4) | ((noise_source_5_active as u128) << 5) | ((noise_source_6_active as u128) << 6) | ((noise_source_7_active as u128) << 7) | ((noise_source_8_active as u128) << 8) | ((noise_source_9_active as u128) << 9) | ((noise_source_10_active as u128) << 10) | ((noise_source_11_active as u128) << 11) | ((noise_source_12_active as u128) << 12) | ((noise_source_13_active as u128) << 13) | ((noise_source_14_active as u128) << 14)];
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
            let noise_0_psd_e6270: f64 = 1.0;
            let noise_0_psd_e183: f64 = 2.0;
            let noise_0_psd_e185: f64 = (noise_0_psd_e183 * 1.602189e-19);
            let noise_0_psd_e187: f64 = (w[87]).abs();
            let noise_0_psd_e188: f64 = (noise_0_psd_e185 * noise_0_psd_e187);
            let noise_0_psd_e6271: f64 = (noise_0_psd_e6270 * noise_0_psd_e188);
            let psd = noise_0_psd_e6271;
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
            let noise_1_psd_e6273: f64 = 1.0;
            let noise_1_psd_e196: f64 = params[98];
            let noise_1_psd_e199: f64 = w[87];
            let noise_1_psd_e200: f64 = (noise_1_psd_e199).abs();
            let noise_1_psd_e202: f64 = (noise_1_psd_e200).powf(params[99]);
            let noise_1_psd_e203: f64 = (noise_1_psd_e196 * noise_1_psd_e202);
            let noise_1_psd_e6274: f64 = (noise_1_psd_e6273 * noise_1_psd_e203);
            let psd = noise_1_psd_e6274;
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
            let noise_2_psd_e6276: f64 = 1.0;
            let noise_2_psd_e212: f64 = 2.0;
            let noise_2_psd_e214: f64 = (noise_2_psd_e212 * 1.602189e-19);
            let noise_2_psd_e216: f64 = (w[88]).abs();
            let noise_2_psd_e217: f64 = (noise_2_psd_e214 * noise_2_psd_e216);
            let noise_2_psd_e6277: f64 = (noise_2_psd_e6276 * noise_2_psd_e217);
            let psd = noise_2_psd_e6277;
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
            let noise_3_psd_e6279: f64 = 1.0;
            let noise_3_psd_e225: f64 = params[98];
            let noise_3_psd_e228: f64 = w[88];
            let noise_3_psd_e229: f64 = (noise_3_psd_e228).abs();
            let noise_3_psd_e231: f64 = (noise_3_psd_e229).powf(params[99]);
            let noise_3_psd_e232: f64 = (noise_3_psd_e225 * noise_3_psd_e231);
            let noise_3_psd_e6280: f64 = (noise_3_psd_e6279 * noise_3_psd_e232);
            let psd = noise_3_psd_e6280;
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
            let noise_4_psd_e6282: f64 = 1.0;
            let noise_4_psd_e241: f64 = 2.0;
            let noise_4_psd_e243: f64 = (noise_4_psd_e241 * 1.602189e-19);
            let noise_4_psd_e245: f64 = (w[76]).abs();
            let noise_4_psd_e246: f64 = (noise_4_psd_e243 * noise_4_psd_e245);
            let noise_4_psd_e6283: f64 = (noise_4_psd_e6282 * noise_4_psd_e246);
            let psd = noise_4_psd_e6283;
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
            let noise_5_psd_e6285: f64 = 1.0;
            let noise_5_psd_e254: f64 = 2.0;
            let noise_5_psd_e256: f64 = (noise_5_psd_e254 * 1.602189e-19);
            let noise_5_psd_e258: f64 = (w[91]).abs();
            let noise_5_psd_e259: f64 = (noise_5_psd_e256 * noise_5_psd_e258);
            let noise_5_psd_e6286: f64 = (noise_5_psd_e6285 * noise_5_psd_e259);
            let psd = noise_5_psd_e6286;
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
            let noise_6_psd_e6288: f64 = 1.0;
            let noise_6_psd_e267: f64 = 1.0;
            let noise_6_psd_e269: f64 = (noise_6_psd_e267 * params[98]);
            let noise_6_psd_e272: f64 = w[91];
            let noise_6_psd_e273: f64 = (noise_6_psd_e272).abs();
            let noise_6_psd_e275: f64 = (noise_6_psd_e273).powf(params[99]);
            let noise_6_psd_e276: f64 = (noise_6_psd_e269 * noise_6_psd_e275);
            let noise_6_psd_e6289: f64 = (noise_6_psd_e6288 * noise_6_psd_e276);
            let psd = noise_6_psd_e6289;
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
            let noise_7_psd_e6291: f64 = 1.0;
            let noise_7_psd_e285: f64 = 4.0;
            let noise_7_psd_e287: f64 = (noise_7_psd_e285 * 1.380662e-23);
            let noise_7_psd_e289: f64 = (noise_7_psd_e287 * w[39]);
            let noise_7_psd_e291: f64 = (noise_7_psd_e289 * w[53]);
            let noise_7_psd_e6292: f64 = (noise_7_psd_e6291 * noise_7_psd_e291);
            let psd = noise_7_psd_e6292;
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
            let noise_8_psd_e6294: f64 = 1.0;
            let noise_8_psd_e299: f64 = 4.0;
            let noise_8_psd_e301: f64 = (noise_8_psd_e299 * 1.380662e-23);
            let noise_8_psd_e303: f64 = (noise_8_psd_e301 * w[39]);
            let noise_8_psd_e305: f64 = (w[97]).abs();
            let noise_8_psd_e308: f64 = (1e-10 * w[54]);
            let noise_8_psd_e309: f64 = (noise_8_psd_e305 + noise_8_psd_e308);
            let noise_8_psd_e311: f64 = (w[154]).abs();
            let noise_8_psd_e313: f64 = (noise_8_psd_e311 + 1e-10);
            let noise_8_psd_e314: f64 = (noise_8_psd_e309 / noise_8_psd_e313);
            let noise_8_psd_e315: f64 = (noise_8_psd_e303 * noise_8_psd_e314);
            let noise_8_psd_e6295: f64 = (noise_8_psd_e6294 * noise_8_psd_e315);
            let psd = noise_8_psd_e6295;
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
            let noise_9_psd_e6297: f64 = 1.0;
            let noise_9_psd_e323: f64 = 4.0;
            let noise_9_psd_e325: f64 = (noise_9_psd_e323 * 1.380662e-23);
            let noise_9_psd_e327: f64 = (noise_9_psd_e325 * w[39]);
            let noise_9_psd_e329: f64 = (noise_9_psd_e327 * w[55]);
            let noise_9_psd_e6298: f64 = (noise_9_psd_e6297 * noise_9_psd_e329);
            let psd = noise_9_psd_e6298;
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
            let noise_10_psd_e6300: f64 = 1.0;
            let noise_10_psd_e337: f64 = 4.0;
            let noise_10_psd_e339: f64 = (noise_10_psd_e337 * 1.380662e-23);
            let noise_10_psd_e341: f64 = (noise_10_psd_e339 * w[39]);
            let noise_10_psd_e343: f64 = (noise_10_psd_e341 * w[81]);
            let noise_10_psd_e345: f64 = (noise_10_psd_e343 * w[56]);
            let noise_10_psd_e6301: f64 = (noise_10_psd_e6300 * noise_10_psd_e345);
            let psd = noise_10_psd_e6301;
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
            let noise_11_psd_e6303: f64 = 1.0;
            let noise_11_psd_e353: f64 = 4.0;
            let noise_11_psd_e355: f64 = (noise_11_psd_e353 * 1.380662e-23);
            let noise_11_psd_e357: f64 = (noise_11_psd_e355 * w[39]);
            let noise_11_psd_e359: f64 = (noise_11_psd_e357 * w[57]);
            let noise_11_psd_e6304: f64 = (noise_11_psd_e6303 * noise_11_psd_e359);
            let psd = noise_11_psd_e6304;
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
            let noise_12_psd_e6306: f64 = 1.0;
            let noise_12_psd_e367: f64 = 4.0;
            let noise_12_psd_e369: f64 = (noise_12_psd_e367 * 1.380662e-23);
            let noise_12_psd_e371: f64 = (noise_12_psd_e369 * w[39]);
            let noise_12_psd_e373: f64 = (noise_12_psd_e371 * w[86]);
            let noise_12_psd_e375: f64 = (noise_12_psd_e373 * w[58]);
            let noise_12_psd_e6307: f64 = (noise_12_psd_e6306 * noise_12_psd_e375);
            let psd = noise_12_psd_e6307;
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
            let noise_13_psd_e6309: f64 = 1.0;
            let noise_13_psd_e383: f64 = 2.0;
            let noise_13_psd_e385: f64 = (noise_13_psd_e383 * 1.602189e-19);
            let noise_13_psd_e387: f64 = (w[84]).abs();
            let noise_13_psd_e388: f64 = (noise_13_psd_e385 * noise_13_psd_e387);
            let noise_13_psd_e6310: f64 = (noise_13_psd_e6309 * noise_13_psd_e388);
            let psd = noise_13_psd_e6310;
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
            let noise_14_psd_e6312: f64 = 1.0;
            let noise_14_psd_e396: f64 = 4.0;
            let noise_14_psd_e398: f64 = (noise_14_psd_e396 * 1.380662e-23);
            let noise_14_psd_e400: f64 = (noise_14_psd_e398 * w[39]);
            let noise_14_psd_e402: f64 = (noise_14_psd_e400 * w[59]);
            let noise_14_psd_e6313: f64 = (noise_14_psd_e6312 * noise_14_psd_e402);
            let psd = noise_14_psd_e6313;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x7fff) != 0 {
            let noise_metadata_schedule_0_0_e408: f64 = if ctx.analysis_initial_step() { 1.0 } else { 0.0 };
            w[172] = noise_metadata_schedule_0_0_e408;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_7_0_e452: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            w[175] = noise_metadata_schedule_7_0_e452;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_8_0_e458,) = {
    if ((w[172] != 0.0) && (w[175] != 0.0)) {
        (params[10],)
    } else {
        (w[165],)
    }
};
            w[165] = noise_metadata_schedule_8_0_e458;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_9_0_e467,) = {
    if ((w[172] != 0.0) && (w[175] == 0.0)) {
        let noise_metadata_schedule_9_0_e465: f64 = 1e-12;
        (noise_metadata_schedule_9_0_e465,)
    } else {
        (w[165],)
    }
};
            w[165] = noise_metadata_schedule_9_0_e467;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_10_0_e469: f64 = if self.param_given[11] { 1.0 } else { 0.0 };
            w[176] = noise_metadata_schedule_10_0_e469;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_11_0_e475,) = {
    if ((w[172] != 0.0) && (w[176] != 0.0)) {
        (params[11],)
    } else {
        (w[166],)
    }
};
            w[166] = noise_metadata_schedule_11_0_e475;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_12_0_e484,) = {
    if ((w[172] != 0.0) && (w[176] == 0.0)) {
        let noise_metadata_schedule_12_0_e482: f64 = 1.0;
        (noise_metadata_schedule_12_0_e482,)
    } else {
        (w[166],)
    }
};
            w[166] = noise_metadata_schedule_12_0_e484;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_13_0_e486: f64 = if self.param_given[3] { 1.0 } else { 0.0 };
            w[177] = noise_metadata_schedule_13_0_e486;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_14_0_e493,) = {
    if ((w[172] != 0.0) && (w[177] != 0.0)) {
        let noise_metadata_schedule_14_0_e491: f64 = 1.0;
        (noise_metadata_schedule_14_0_e491,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_14_0_e493;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_15_0_e495: f64 = if self.param_given[4] { 1.0 } else { 0.0 };
            w[178] = noise_metadata_schedule_15_0_e495;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_16_0_e505,) = {
    if (((w[172] != 0.0) && (w[177] == 0.0)) && (w[178] != 0.0)) {
        let noise_metadata_schedule_16_0_e503: f64 = (-1.0);
        (noise_metadata_schedule_16_0_e503,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_16_0_e505;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_17_0_e507: f64 = if self.param_given[5] { 1.0 } else { 0.0 };
            w[179] = noise_metadata_schedule_17_0_e507;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_18_0_e519,) = {
    if ((((w[172] != 0.0) && (w[177] == 0.0)) && (w[178] == 0.0)) && (w[179] != 0.0)) {
        (params[5],)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_18_0_e519;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_19_0_e533,) = {
    if ((((w[172] != 0.0) && (w[177] == 0.0)) && (w[178] == 0.0)) && (w[179] == 0.0)) {
        let noise_metadata_schedule_19_0_e531: f64 = 1.0;
        (noise_metadata_schedule_19_0_e531,)
    } else {
        (w[162],)
    }
};
            w[162] = noise_metadata_schedule_19_0_e533;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_20_0_e538,) = {
    if (w[172] != 0.0) {
        let noise_metadata_schedule_20_0_e536: f64 = (params[12]).ln();
        (noise_metadata_schedule_20_0_e536,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_20_0_e538;
        }
        if (active[0] & 0x3410) != 0 {
            let (noise_metadata_schedule_21_0_e549,) = {
    if (w[172] != 0.0) {
        let (noise_metadata_schedule_21_0_e547,) = {
            if (params[74] > 0.0) {
                let noise_metadata_schedule_21_0_e545: f64 = (1.0 / params[74]);
                (noise_metadata_schedule_21_0_e545,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_21_0_e547,)
    } else {
        (w[46],)
    }
};
            w[46] = noise_metadata_schedule_21_0_e549;
        }
        if (active[0] & 0x316f) != 0 {
            let (noise_metadata_schedule_22_0_e560,) = {
    if (w[172] != 0.0) {
        let (noise_metadata_schedule_22_0_e558,) = {
            if (params[75] > 0.0) {
                let noise_metadata_schedule_22_0_e556: f64 = (1.0 / params[75]);
                (noise_metadata_schedule_22_0_e556,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_22_0_e558,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_22_0_e560;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_23_0_e571,) = {
    if (w[172] != 0.0) {
        let (noise_metadata_schedule_23_0_e569,) = {
            if (params[20] > 0.0) {
                let noise_metadata_schedule_23_0_e567: f64 = (1.0 / params[20]);
                (noise_metadata_schedule_23_0_e567,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_23_0_e569,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_23_0_e571;
        }
        if (active[0] & 0x7fff) != 0 {
            let (noise_metadata_schedule_27_0_e608,) = {
    if (w[172] != 0.0) {
        let noise_metadata_schedule_27_0_e606: f64 = (273.15 + params[13]);
        (noise_metadata_schedule_27_0_e606,)
    } else {
        (w[40],)
    }
};
            w[40] = noise_metadata_schedule_27_0_e608;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_29_0_e610: f64 = ctx.temperature();
            let noise_metadata_schedule_29_0_e612: f64 = (noise_metadata_schedule_29_0_e610 + params[0]);
            let noise_metadata_schedule_29_0_e614: f64 = (noise_metadata_schedule_29_0_e612 - 273.15);
            w[38] = noise_metadata_schedule_29_0_e614;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_32_0_e624: f64 = (params[14] + 1.0);
            let noise_metadata_schedule_32_0_e625: f64 = if w[38] < noise_metadata_schedule_32_0_e624 { 1.0 } else { 0.0 };
            w[182] = noise_metadata_schedule_32_0_e625;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_33_0_e636,) = {
    if (w[182] != 0.0) {
        let noise_metadata_schedule_33_0_e630: f64 = (w[38] - params[14]);
        let noise_metadata_schedule_33_0_e632: f64 = (noise_metadata_schedule_33_0_e630 - 1.0);
        let noise_metadata_schedule_33_0_e633: f64 = (noise_metadata_schedule_33_0_e632).exp();
        let noise_metadata_schedule_33_0_e634: f64 = (params[14] + noise_metadata_schedule_33_0_e633);
        (noise_metadata_schedule_33_0_e634,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_33_0_e636;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_34_0_e640: f64 = (params[15] - 1.0);
            let noise_metadata_schedule_34_0_e641: f64 = if w[38] > noise_metadata_schedule_34_0_e640 { 1.0 } else { 0.0 };
            w[183] = noise_metadata_schedule_34_0_e641;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_35_0_e655,) = {
    if ((w[182] == 0.0) && (w[183] != 0.0)) {
        let noise_metadata_schedule_35_0_e649: f64 = (params[15] - w[38]);
        let noise_metadata_schedule_35_0_e651: f64 = (noise_metadata_schedule_35_0_e649 - 1.0);
        let noise_metadata_schedule_35_0_e652: f64 = (noise_metadata_schedule_35_0_e651).exp();
        let noise_metadata_schedule_35_0_e653: f64 = (params[15] - noise_metadata_schedule_35_0_e652);
        (noise_metadata_schedule_35_0_e653,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_35_0_e655;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_36_0_e663,) = {
    if ((w[182] == 0.0) && (w[183] == 0.0)) {
        (w[38],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_36_0_e663;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_37_0_e666: f64 = (w[38] + 273.15);
            w[39] = noise_metadata_schedule_37_0_e666;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_38_0_e669: f64 = (1.380662e-23 * w[39]);
            let noise_metadata_schedule_38_0_e671: f64 = (noise_metadata_schedule_38_0_e669 / 1.602189e-19);
            w[73] = noise_metadata_schedule_38_0_e671;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_39_0_e674: f64 = (w[39] / w[40]);
            w[41] = noise_metadata_schedule_39_0_e674;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_41_0_e687: f64 = if params[90] > 0.0 { 1.0 } else { 0.0 };
            w[184] = noise_metadata_schedule_41_0_e687;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_42_0_e706,) = {
    if (w[184] != 0.0) {
        let noise_metadata_schedule_42_0_e691: f64 = (params[89] * w[73]);
        let noise_metadata_schedule_42_0_e693: f64 = (-params[88]);
        let noise_metadata_schedule_42_0_e696: f64 = (params[89] * w[73]);
        let noise_metadata_schedule_42_0_e697: f64 = (noise_metadata_schedule_42_0_e693 / noise_metadata_schedule_42_0_e696);
        let noise_metadata_schedule_42_0_e698: f64 = (noise_metadata_schedule_42_0_e697).exp();
        let noise_metadata_schedule_42_0_e701: f64 = (w[166] / params[90]);
        let noise_metadata_schedule_42_0_e702: f64 = (noise_metadata_schedule_42_0_e698 + noise_metadata_schedule_42_0_e701);
        let noise_metadata_schedule_42_0_e703: f64 = (noise_metadata_schedule_42_0_e702).ln();
        let noise_metadata_schedule_42_0_e704: f64 = (noise_metadata_schedule_42_0_e691 * noise_metadata_schedule_42_0_e703);
        (noise_metadata_schedule_42_0_e704,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_42_0_e706;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_43_0_e711,) = {
    if (w[184] == 0.0) {
        (0.0,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_43_0_e711;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_44_0_e716: f64 = (params[122] / params[28]);
            let noise_metadata_schedule_44_0_e717: f64 = (w[41]).powf(noise_metadata_schedule_44_0_e716);
            let noise_metadata_schedule_44_0_e718: f64 = (params[26] * noise_metadata_schedule_44_0_e717);
            let noise_metadata_schedule_44_0_e720: f64 = (-params[113]);
            let noise_metadata_schedule_44_0_e723: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_44_0_e724: f64 = (noise_metadata_schedule_44_0_e720 * noise_metadata_schedule_44_0_e723);
            let noise_metadata_schedule_44_0_e727: f64 = (w[73] * params[28]);
            let noise_metadata_schedule_44_0_e728: f64 = (noise_metadata_schedule_44_0_e724 / noise_metadata_schedule_44_0_e727);
            let noise_metadata_schedule_44_0_e729: f64 = (noise_metadata_schedule_44_0_e728).exp();
            let noise_metadata_schedule_44_0_e730: f64 = (noise_metadata_schedule_44_0_e718 * noise_metadata_schedule_44_0_e729);
            w[0] = noise_metadata_schedule_44_0_e730;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_45_0_e733: f64 = if w[0] > 0.0 { 1.0 } else { 0.0 };
            w[185] = noise_metadata_schedule_45_0_e733;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_46_0_e740: f64 = if ((params[72] > 0.0) && (w[166] > params[72])) { 1.0 } else { 0.0 };
            w[186] = noise_metadata_schedule_46_0_e740;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_47_0_e769,) = {
    if ((w[185] != 0.0) && (w[186] != 0.0)) {
        let noise_metadata_schedule_47_0_e746: f64 = (params[28] * w[73]);
        let noise_metadata_schedule_47_0_e750: f64 = (0.5 * w[166]);
        let noise_metadata_schedule_47_0_e753: f64 = (4.0 / params[72]);
        let noise_metadata_schedule_47_0_e755: f64 = (noise_metadata_schedule_47_0_e753).powf(params[73]);
        let noise_metadata_schedule_47_0_e756: f64 = (noise_metadata_schedule_47_0_e750 * noise_metadata_schedule_47_0_e755);
        let noise_metadata_schedule_47_0_e760: f64 = (1.0 - params[73]);
        let noise_metadata_schedule_47_0_e761: f64 = (1.0 / noise_metadata_schedule_47_0_e760);
        let noise_metadata_schedule_47_0_e762: f64 = (noise_metadata_schedule_47_0_e756).powf(noise_metadata_schedule_47_0_e761);
        let noise_metadata_schedule_47_0_e764: f64 = (noise_metadata_schedule_47_0_e762 / w[0]);
        let noise_metadata_schedule_47_0_e765: f64 = (1.0 + noise_metadata_schedule_47_0_e764);
        let noise_metadata_schedule_47_0_e766: f64 = (noise_metadata_schedule_47_0_e765).ln();
        let noise_metadata_schedule_47_0_e767: f64 = (noise_metadata_schedule_47_0_e746 * noise_metadata_schedule_47_0_e766);
        (noise_metadata_schedule_47_0_e767,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_47_0_e769;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_48_0_e785,) = {
    if ((w[185] != 0.0) && (w[186] == 0.0)) {
        let noise_metadata_schedule_48_0_e776: f64 = (params[28] * w[73]);
        let noise_metadata_schedule_48_0_e780: f64 = (w[166] / w[0]);
        let noise_metadata_schedule_48_0_e781: f64 = (1.0 + noise_metadata_schedule_48_0_e780);
        let noise_metadata_schedule_48_0_e782: f64 = (noise_metadata_schedule_48_0_e781).ln();
        let noise_metadata_schedule_48_0_e783: f64 = (noise_metadata_schedule_48_0_e776 * noise_metadata_schedule_48_0_e782);
        (noise_metadata_schedule_48_0_e783,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_48_0_e785;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_49_0_e790,) = {
    if (w[185] == 0.0) {
        (0.0,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_49_0_e790;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_50_0_e795: f64 = (params[125] / params[29]);
            let noise_metadata_schedule_50_0_e796: f64 = (w[41]).powf(noise_metadata_schedule_50_0_e795);
            let noise_metadata_schedule_50_0_e797: f64 = (params[27] * noise_metadata_schedule_50_0_e796);
            let noise_metadata_schedule_50_0_e799: f64 = (-params[121]);
            let noise_metadata_schedule_50_0_e802: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_50_0_e803: f64 = (noise_metadata_schedule_50_0_e799 * noise_metadata_schedule_50_0_e802);
            let noise_metadata_schedule_50_0_e806: f64 = (w[73] * params[29]);
            let noise_metadata_schedule_50_0_e807: f64 = (noise_metadata_schedule_50_0_e803 / noise_metadata_schedule_50_0_e806);
            let noise_metadata_schedule_50_0_e808: f64 = (noise_metadata_schedule_50_0_e807).exp();
            let noise_metadata_schedule_50_0_e809: f64 = (noise_metadata_schedule_50_0_e797 * noise_metadata_schedule_50_0_e808);
            w[1] = noise_metadata_schedule_50_0_e809;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_51_0_e816: f64 = if ((w[0] > 0.0) && (w[1] > 0.0)) { 1.0 } else { 0.0 };
            w[187] = noise_metadata_schedule_51_0_e816;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_52_0_e823: f64 = if ((params[74] > 0.0) && (w[166] > params[74])) { 1.0 } else { 0.0 };
            w[188] = noise_metadata_schedule_52_0_e823;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_53_0_e854,) = {
    if ((w[187] != 0.0) && (w[188] != 0.0)) {
        let noise_metadata_schedule_53_0_e829: f64 = (params[29] * w[73]);
        let noise_metadata_schedule_53_0_e833: f64 = (0.5 * w[166]);
        let noise_metadata_schedule_53_0_e836: f64 = (4.0 / params[74]);
        let noise_metadata_schedule_53_0_e838: f64 = (noise_metadata_schedule_53_0_e836).powf(params[73]);
        let noise_metadata_schedule_53_0_e839: f64 = (noise_metadata_schedule_53_0_e833 * noise_metadata_schedule_53_0_e838);
        let noise_metadata_schedule_53_0_e843: f64 = (1.0 - params[73]);
        let noise_metadata_schedule_53_0_e844: f64 = (1.0 / noise_metadata_schedule_53_0_e843);
        let noise_metadata_schedule_53_0_e845: f64 = (noise_metadata_schedule_53_0_e839).powf(noise_metadata_schedule_53_0_e844);
        let noise_metadata_schedule_53_0_e848: f64 = (w[0] * w[1]);
        let noise_metadata_schedule_53_0_e849: f64 = (noise_metadata_schedule_53_0_e845 / noise_metadata_schedule_53_0_e848);
        let noise_metadata_schedule_53_0_e850: f64 = (1.0 + noise_metadata_schedule_53_0_e849);
        let noise_metadata_schedule_53_0_e851: f64 = (noise_metadata_schedule_53_0_e850).ln();
        let noise_metadata_schedule_53_0_e852: f64 = (noise_metadata_schedule_53_0_e829 * noise_metadata_schedule_53_0_e851);
        (noise_metadata_schedule_53_0_e852,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_53_0_e854;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_54_0_e872,) = {
    if ((w[187] != 0.0) && (w[188] == 0.0)) {
        let noise_metadata_schedule_54_0_e861: f64 = (params[29] * w[73]);
        let noise_metadata_schedule_54_0_e866: f64 = (w[0] * w[1]);
        let noise_metadata_schedule_54_0_e867: f64 = (w[166] / noise_metadata_schedule_54_0_e866);
        let noise_metadata_schedule_54_0_e868: f64 = (1.0 + noise_metadata_schedule_54_0_e867);
        let noise_metadata_schedule_54_0_e869: f64 = (noise_metadata_schedule_54_0_e868).ln();
        let noise_metadata_schedule_54_0_e870: f64 = (noise_metadata_schedule_54_0_e861 * noise_metadata_schedule_54_0_e869);
        (noise_metadata_schedule_54_0_e870,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_54_0_e872;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_55_0_e877,) = {
    if (w[187] == 0.0) {
        (0.0,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_55_0_e877;
        }
        if (active[0] & 0x316f) != 0 {
            let noise_metadata_schedule_56_0_e882: f64 = (params[122] / params[33]);
            let noise_metadata_schedule_56_0_e883: f64 = (w[41]).powf(noise_metadata_schedule_56_0_e882);
            let noise_metadata_schedule_56_0_e884: f64 = (params[31] * noise_metadata_schedule_56_0_e883);
            let noise_metadata_schedule_56_0_e886: f64 = (-params[120]);
            let noise_metadata_schedule_56_0_e889: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_56_0_e890: f64 = (noise_metadata_schedule_56_0_e886 * noise_metadata_schedule_56_0_e889);
            let noise_metadata_schedule_56_0_e893: f64 = (w[73] * params[33]);
            let noise_metadata_schedule_56_0_e894: f64 = (noise_metadata_schedule_56_0_e890 / noise_metadata_schedule_56_0_e893);
            let noise_metadata_schedule_56_0_e895: f64 = (noise_metadata_schedule_56_0_e894).exp();
            let noise_metadata_schedule_56_0_e896: f64 = (noise_metadata_schedule_56_0_e884 * noise_metadata_schedule_56_0_e895);
            w[5] = noise_metadata_schedule_56_0_e896;
        }
        if (active[0] & 0x316f) != 0 {
            let noise_metadata_schedule_57_0_e899: f64 = if w[5] > 0.0 { 1.0 } else { 0.0 };
            w[189] = noise_metadata_schedule_57_0_e899;
        }
        if (active[0] & 0x316f) != 0 {
            let noise_metadata_schedule_58_0_e906: f64 = if ((params[75] > 0.0) && (w[166] > params[75])) { 1.0 } else { 0.0 };
            w[190] = noise_metadata_schedule_58_0_e906;
        }
        if (active[0] & 0x316f) != 0 {
            let (noise_metadata_schedule_59_0_e925,) = {
    if ((w[189] != 0.0) && (w[190] != 0.0)) {
        let noise_metadata_schedule_59_0_e912: f64 = (params[33] * w[73]);
        let noise_metadata_schedule_59_0_e916: f64 = (w[166] * w[166]);
        let noise_metadata_schedule_59_0_e918: f64 = (noise_metadata_schedule_59_0_e916 * w[47]);
        let noise_metadata_schedule_59_0_e920: f64 = (noise_metadata_schedule_59_0_e918 / w[5]);
        let noise_metadata_schedule_59_0_e921: f64 = (1.0 + noise_metadata_schedule_59_0_e920);
        let noise_metadata_schedule_59_0_e922: f64 = (noise_metadata_schedule_59_0_e921).ln();
        let noise_metadata_schedule_59_0_e923: f64 = (noise_metadata_schedule_59_0_e912 * noise_metadata_schedule_59_0_e922);
        (noise_metadata_schedule_59_0_e923,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_59_0_e925;
        }
        if (active[0] & 0x316f) != 0 {
            let (noise_metadata_schedule_60_0_e941,) = {
    if ((w[189] != 0.0) && (w[190] == 0.0)) {
        let noise_metadata_schedule_60_0_e932: f64 = (params[33] * w[73]);
        let noise_metadata_schedule_60_0_e936: f64 = (w[166] / w[5]);
        let noise_metadata_schedule_60_0_e937: f64 = (1.0 + noise_metadata_schedule_60_0_e936);
        let noise_metadata_schedule_60_0_e938: f64 = (noise_metadata_schedule_60_0_e937).ln();
        let noise_metadata_schedule_60_0_e939: f64 = (noise_metadata_schedule_60_0_e932 * noise_metadata_schedule_60_0_e938);
        (noise_metadata_schedule_60_0_e939,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_60_0_e941;
        }
        if (active[0] & 0x316f) != 0 {
            let (noise_metadata_schedule_61_0_e946,) = {
    if (w[189] == 0.0) {
        (0.0,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_61_0_e946;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_62_0_e951: f64 = (params[123] / params[56]);
            let noise_metadata_schedule_62_0_e952: f64 = (w[41]).powf(noise_metadata_schedule_62_0_e951);
            let noise_metadata_schedule_62_0_e953: f64 = (params[54] * noise_metadata_schedule_62_0_e952);
            let noise_metadata_schedule_62_0_e955: f64 = (-params[114]);
            let noise_metadata_schedule_62_0_e958: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_62_0_e959: f64 = (noise_metadata_schedule_62_0_e955 * noise_metadata_schedule_62_0_e958);
            let noise_metadata_schedule_62_0_e962: f64 = (w[73] * params[56]);
            let noise_metadata_schedule_62_0_e963: f64 = (noise_metadata_schedule_62_0_e959 / noise_metadata_schedule_62_0_e962);
            let noise_metadata_schedule_62_0_e964: f64 = (noise_metadata_schedule_62_0_e963).exp();
            let noise_metadata_schedule_62_0_e965: f64 = (noise_metadata_schedule_62_0_e953 * noise_metadata_schedule_62_0_e964);
            w[3] = noise_metadata_schedule_62_0_e965;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_63_0_e968: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[191] = noise_metadata_schedule_63_0_e968;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_64_0_e981,) = {
    if (w[191] != 0.0) {
        let noise_metadata_schedule_64_0_e972: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_64_0_e976: f64 = (w[166] / w[3]);
        let noise_metadata_schedule_64_0_e977: f64 = (1.0 + noise_metadata_schedule_64_0_e976);
        let noise_metadata_schedule_64_0_e978: f64 = (noise_metadata_schedule_64_0_e977).ln();
        let noise_metadata_schedule_64_0_e979: f64 = (noise_metadata_schedule_64_0_e972 * noise_metadata_schedule_64_0_e978);
        (noise_metadata_schedule_64_0_e979,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_64_0_e981;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_65_0_e986,) = {
    if (w[191] == 0.0) {
        (0.0,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_65_0_e986;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_66_0_e991: f64 = (params[124] / params[59]);
            let noise_metadata_schedule_66_0_e992: f64 = (w[41]).powf(noise_metadata_schedule_66_0_e991);
            let noise_metadata_schedule_66_0_e993: f64 = (params[58] * noise_metadata_schedule_66_0_e992);
            let noise_metadata_schedule_66_0_e995: f64 = (-params[117]);
            let noise_metadata_schedule_66_0_e998: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_66_0_e999: f64 = (noise_metadata_schedule_66_0_e995 * noise_metadata_schedule_66_0_e998);
            let noise_metadata_schedule_66_0_e1002: f64 = (w[73] * params[59]);
            let noise_metadata_schedule_66_0_e1003: f64 = (noise_metadata_schedule_66_0_e999 / noise_metadata_schedule_66_0_e1002);
            let noise_metadata_schedule_66_0_e1004: f64 = (noise_metadata_schedule_66_0_e1003).exp();
            let noise_metadata_schedule_66_0_e1005: f64 = (noise_metadata_schedule_66_0_e993 * noise_metadata_schedule_66_0_e1004);
            w[6] = noise_metadata_schedule_66_0_e1005;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_67_0_e1008: f64 = if w[6] > 0.0 { 1.0 } else { 0.0 };
            w[192] = noise_metadata_schedule_67_0_e1008;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_68_0_e1021,) = {
    if (w[192] != 0.0) {
        let noise_metadata_schedule_68_0_e1012: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_68_0_e1016: f64 = (w[166] / w[6]);
        let noise_metadata_schedule_68_0_e1017: f64 = (1.0 + noise_metadata_schedule_68_0_e1016);
        let noise_metadata_schedule_68_0_e1018: f64 = (noise_metadata_schedule_68_0_e1017).ln();
        let noise_metadata_schedule_68_0_e1019: f64 = (noise_metadata_schedule_68_0_e1012 * noise_metadata_schedule_68_0_e1018);
        (noise_metadata_schedule_68_0_e1019,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_68_0_e1021;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_69_0_e1026,) = {
    if (w[192] == 0.0) {
        (0.0,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_69_0_e1026;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_70_0_e1031: f64 = (params[123] / params[61]);
            let noise_metadata_schedule_70_0_e1032: f64 = (w[41]).powf(noise_metadata_schedule_70_0_e1031);
            let noise_metadata_schedule_70_0_e1033: f64 = (params[60] * noise_metadata_schedule_70_0_e1032);
            let noise_metadata_schedule_70_0_e1035: f64 = (-params[115]);
            let noise_metadata_schedule_70_0_e1038: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_70_0_e1039: f64 = (noise_metadata_schedule_70_0_e1035 * noise_metadata_schedule_70_0_e1038);
            let noise_metadata_schedule_70_0_e1042: f64 = (w[73] * params[61]);
            let noise_metadata_schedule_70_0_e1043: f64 = (noise_metadata_schedule_70_0_e1039 / noise_metadata_schedule_70_0_e1042);
            let noise_metadata_schedule_70_0_e1044: f64 = (noise_metadata_schedule_70_0_e1043).exp();
            let noise_metadata_schedule_70_0_e1045: f64 = (noise_metadata_schedule_70_0_e1033 * noise_metadata_schedule_70_0_e1044);
            w[4] = noise_metadata_schedule_70_0_e1045;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_71_0_e1048: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[193] = noise_metadata_schedule_71_0_e1048;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_72_0_e1061,) = {
    if (w[193] != 0.0) {
        let noise_metadata_schedule_72_0_e1052: f64 = (params[61] * w[73]);
        let noise_metadata_schedule_72_0_e1056: f64 = (w[166] / w[4]);
        let noise_metadata_schedule_72_0_e1057: f64 = (1.0 + noise_metadata_schedule_72_0_e1056);
        let noise_metadata_schedule_72_0_e1058: f64 = (noise_metadata_schedule_72_0_e1057).ln();
        let noise_metadata_schedule_72_0_e1059: f64 = (noise_metadata_schedule_72_0_e1052 * noise_metadata_schedule_72_0_e1058);
        (noise_metadata_schedule_72_0_e1059,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_72_0_e1061;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_73_0_e1066,) = {
    if (w[193] == 0.0) {
        (0.0,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_73_0_e1066;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_74_0_e1071: f64 = (params[124] / params[63]);
            let noise_metadata_schedule_74_0_e1072: f64 = (w[41]).powf(noise_metadata_schedule_74_0_e1071);
            let noise_metadata_schedule_74_0_e1073: f64 = (params[62] * noise_metadata_schedule_74_0_e1072);
            let noise_metadata_schedule_74_0_e1075: f64 = (-params[118]);
            let noise_metadata_schedule_74_0_e1078: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_74_0_e1079: f64 = (noise_metadata_schedule_74_0_e1075 * noise_metadata_schedule_74_0_e1078);
            let noise_metadata_schedule_74_0_e1082: f64 = (w[73] * params[63]);
            let noise_metadata_schedule_74_0_e1083: f64 = (noise_metadata_schedule_74_0_e1079 / noise_metadata_schedule_74_0_e1082);
            let noise_metadata_schedule_74_0_e1084: f64 = (noise_metadata_schedule_74_0_e1083).exp();
            let noise_metadata_schedule_74_0_e1085: f64 = (noise_metadata_schedule_74_0_e1073 * noise_metadata_schedule_74_0_e1084);
            w[7] = noise_metadata_schedule_74_0_e1085;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_75_0_e1088: f64 = if w[7] > 0.0 { 1.0 } else { 0.0 };
            w[194] = noise_metadata_schedule_75_0_e1088;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_76_0_e1101,) = {
    if (w[194] != 0.0) {
        let noise_metadata_schedule_76_0_e1092: f64 = (params[63] * w[73]);
        let noise_metadata_schedule_76_0_e1096: f64 = (w[166] / w[7]);
        let noise_metadata_schedule_76_0_e1097: f64 = (1.0 + noise_metadata_schedule_76_0_e1096);
        let noise_metadata_schedule_76_0_e1098: f64 = (noise_metadata_schedule_76_0_e1097).ln();
        let noise_metadata_schedule_76_0_e1099: f64 = (noise_metadata_schedule_76_0_e1092 * noise_metadata_schedule_76_0_e1098);
        (noise_metadata_schedule_76_0_e1099,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_76_0_e1101;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_77_0_e1106,) = {
    if (w[194] == 0.0) {
        (0.0,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_77_0_e1106;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_78_0_e1111: f64 = (params[123] / params[61]);
            let noise_metadata_schedule_78_0_e1112: f64 = (w[41]).powf(noise_metadata_schedule_78_0_e1111);
            let noise_metadata_schedule_78_0_e1113: f64 = (params[64] * noise_metadata_schedule_78_0_e1112);
            let noise_metadata_schedule_78_0_e1115: f64 = (-params[115]);
            let noise_metadata_schedule_78_0_e1118: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_78_0_e1119: f64 = (noise_metadata_schedule_78_0_e1115 * noise_metadata_schedule_78_0_e1118);
            let noise_metadata_schedule_78_0_e1122: f64 = (w[73] * params[61]);
            let noise_metadata_schedule_78_0_e1123: f64 = (noise_metadata_schedule_78_0_e1119 / noise_metadata_schedule_78_0_e1122);
            let noise_metadata_schedule_78_0_e1124: f64 = (noise_metadata_schedule_78_0_e1123).exp();
            let noise_metadata_schedule_78_0_e1125: f64 = (noise_metadata_schedule_78_0_e1113 * noise_metadata_schedule_78_0_e1124);
            w[8] = noise_metadata_schedule_78_0_e1125;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_79_0_e1128: f64 = if w[8] > 0.0 { 1.0 } else { 0.0 };
            w[195] = noise_metadata_schedule_79_0_e1128;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_80_0_e1141,) = {
    if (w[195] != 0.0) {
        let noise_metadata_schedule_80_0_e1132: f64 = (params[61] * w[73]);
        let noise_metadata_schedule_80_0_e1136: f64 = (w[166] / w[8]);
        let noise_metadata_schedule_80_0_e1137: f64 = (1.0 + noise_metadata_schedule_80_0_e1136);
        let noise_metadata_schedule_80_0_e1138: f64 = (noise_metadata_schedule_80_0_e1137).ln();
        let noise_metadata_schedule_80_0_e1139: f64 = (noise_metadata_schedule_80_0_e1132 * noise_metadata_schedule_80_0_e1138);
        (noise_metadata_schedule_80_0_e1139,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_80_0_e1141;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_81_0_e1146,) = {
    if (w[195] == 0.0) {
        (0.0,)
    } else {
        (w[69],)
    }
};
            w[69] = noise_metadata_schedule_81_0_e1146;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_82_0_e1151: f64 = (params[124] / params[63]);
            let noise_metadata_schedule_82_0_e1152: f64 = (w[41]).powf(noise_metadata_schedule_82_0_e1151);
            let noise_metadata_schedule_82_0_e1153: f64 = (params[65] * noise_metadata_schedule_82_0_e1152);
            let noise_metadata_schedule_82_0_e1155: f64 = (-params[118]);
            let noise_metadata_schedule_82_0_e1158: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_82_0_e1159: f64 = (noise_metadata_schedule_82_0_e1155 * noise_metadata_schedule_82_0_e1158);
            let noise_metadata_schedule_82_0_e1162: f64 = (w[73] * params[63]);
            let noise_metadata_schedule_82_0_e1163: f64 = (noise_metadata_schedule_82_0_e1159 / noise_metadata_schedule_82_0_e1162);
            let noise_metadata_schedule_82_0_e1164: f64 = (noise_metadata_schedule_82_0_e1163).exp();
            let noise_metadata_schedule_82_0_e1165: f64 = (noise_metadata_schedule_82_0_e1153 * noise_metadata_schedule_82_0_e1164);
            w[9] = noise_metadata_schedule_82_0_e1165;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_83_0_e1168: f64 = if w[9] > 0.0 { 1.0 } else { 0.0 };
            w[196] = noise_metadata_schedule_83_0_e1168;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_84_0_e1181,) = {
    if (w[196] != 0.0) {
        let noise_metadata_schedule_84_0_e1172: f64 = (params[63] * w[73]);
        let noise_metadata_schedule_84_0_e1176: f64 = (w[166] / w[9]);
        let noise_metadata_schedule_84_0_e1177: f64 = (1.0 + noise_metadata_schedule_84_0_e1176);
        let noise_metadata_schedule_84_0_e1178: f64 = (noise_metadata_schedule_84_0_e1177).ln();
        let noise_metadata_schedule_84_0_e1179: f64 = (noise_metadata_schedule_84_0_e1172 * noise_metadata_schedule_84_0_e1178);
        (noise_metadata_schedule_84_0_e1179,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_84_0_e1181;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_85_0_e1186,) = {
    if (w[196] == 0.0) {
        (0.0,)
    } else {
        (w[70],)
    }
};
            w[70] = noise_metadata_schedule_85_0_e1186;
        }
        if (active[0] & 0x7fff) != 0 {
            w[138] = (ctx.node_voltage(self.nodes[4]) - 0.0);
        }
        if (active[0] & 0x7fff) != 0 {
            let noise_metadata_schedule_95_0_e1268: f64 = ctx.temperature();
            let noise_metadata_schedule_95_0_e1270: f64 = (noise_metadata_schedule_95_0_e1268 + params[0]);
            let noise_metadata_schedule_95_0_e1272: f64 = (noise_metadata_schedule_95_0_e1270 + w[138]);
            let noise_metadata_schedule_95_0_e1274: f64 = (noise_metadata_schedule_95_0_e1272 - 273.15);
            w[38] = noise_metadata_schedule_95_0_e1274;
        }
        if (active[0] & 0x7fff) != 0 {
            let noise_metadata_schedule_96_0_e1278: f64 = (params[14] + 1.0);
            let noise_metadata_schedule_96_0_e1279: f64 = if w[38] < noise_metadata_schedule_96_0_e1278 { 1.0 } else { 0.0 };
            w[199] = noise_metadata_schedule_96_0_e1279;
        }
        if (active[0] & 0x7fff) != 0 {
            let (noise_metadata_schedule_97_0_e1290,) = {
    if (w[199] != 0.0) {
        let noise_metadata_schedule_97_0_e1284: f64 = (w[38] - params[14]);
        let noise_metadata_schedule_97_0_e1286: f64 = (noise_metadata_schedule_97_0_e1284 - 1.0);
        let noise_metadata_schedule_97_0_e1287: f64 = (noise_metadata_schedule_97_0_e1286).exp();
        let noise_metadata_schedule_97_0_e1288: f64 = (params[14] + noise_metadata_schedule_97_0_e1287);
        (noise_metadata_schedule_97_0_e1288,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_97_0_e1290;
        }
        if (active[0] & 0x7fff) != 0 {
            let noise_metadata_schedule_98_0_e1294: f64 = (params[15] - 1.0);
            let noise_metadata_schedule_98_0_e1295: f64 = if w[38] > noise_metadata_schedule_98_0_e1294 { 1.0 } else { 0.0 };
            w[200] = noise_metadata_schedule_98_0_e1295;
        }
        if (active[0] & 0x7fff) != 0 {
            let (noise_metadata_schedule_99_0_e1309,) = {
    if ((w[199] == 0.0) && (w[200] != 0.0)) {
        let noise_metadata_schedule_99_0_e1303: f64 = (params[15] - w[38]);
        let noise_metadata_schedule_99_0_e1305: f64 = (noise_metadata_schedule_99_0_e1303 - 1.0);
        let noise_metadata_schedule_99_0_e1306: f64 = (noise_metadata_schedule_99_0_e1305).exp();
        let noise_metadata_schedule_99_0_e1307: f64 = (params[15] - noise_metadata_schedule_99_0_e1306);
        (noise_metadata_schedule_99_0_e1307,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_99_0_e1309;
        }
        if (active[0] & 0x7fff) != 0 {
            let (noise_metadata_schedule_100_0_e1317,) = {
    if ((w[199] == 0.0) && (w[200] == 0.0)) {
        (w[38],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_100_0_e1317;
        }
        if (active[0] & 0x7fff) != 0 {
            let noise_metadata_schedule_101_0_e1320: f64 = (w[38] + 273.15);
            w[39] = noise_metadata_schedule_101_0_e1320;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_102_0_e1323: f64 = (1.380662e-23 * w[39]);
            let noise_metadata_schedule_102_0_e1325: f64 = (noise_metadata_schedule_102_0_e1323 / 1.602189e-19);
            w[73] = noise_metadata_schedule_102_0_e1325;
        }
        if (active[0] & 0x7fff) != 0 {
            let noise_metadata_schedule_103_0_e1328: f64 = (w[39] / w[40]);
            w[41] = noise_metadata_schedule_103_0_e1328;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_104_0_e1331: f64 = (w[39] - w[40]);
            w[42] = noise_metadata_schedule_104_0_e1331;
        }
        if (active[0] & 0x3410) != 0 {
            let noise_metadata_schedule_105_0_e1335: f64 = (w[41]).powf(params[126]);
            let noise_metadata_schedule_105_0_e1336: f64 = (params[72] * noise_metadata_schedule_105_0_e1335);
            w[2] = noise_metadata_schedule_105_0_e1336;
        }
        if (active[0] & 0x80) != 0 {
            let noise_metadata_schedule_106_0_e1338: f64 = if self.param_given[109] { 1.0 } else { 0.0 };
            w[201] = noise_metadata_schedule_106_0_e1338;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_107_0_e1346,) = {
    if (w[201] != 0.0) {
        let noise_metadata_schedule_107_0_e1343: f64 = (w[41]).powf(params[109]);
        let noise_metadata_schedule_107_0_e1344: f64 = (params[16] * noise_metadata_schedule_107_0_e1343);
        (noise_metadata_schedule_107_0_e1344,)
    } else {
        (w[12],)
    }
};
            w[12] = noise_metadata_schedule_107_0_e1346;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_108_0_e1355,) = {
    if (w[201] == 0.0) {
        let noise_metadata_schedule_108_0_e1352: f64 = (w[41]).powf(params[107]);
        let noise_metadata_schedule_108_0_e1353: f64 = (params[16] * noise_metadata_schedule_108_0_e1352);
        (noise_metadata_schedule_108_0_e1353,)
    } else {
        (w[12],)
    }
};
            w[12] = noise_metadata_schedule_108_0_e1355;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_109_0_e1357: f64 = if self.param_given[108] { 1.0 } else { 0.0 };
            w[202] = noise_metadata_schedule_109_0_e1357;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_110_0_e1365,) = {
    if (w[202] != 0.0) {
        let noise_metadata_schedule_110_0_e1362: f64 = (w[41]).powf(params[108]);
        let noise_metadata_schedule_110_0_e1363: f64 = (params[17] * noise_metadata_schedule_110_0_e1362);
        (noise_metadata_schedule_110_0_e1363,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_110_0_e1365;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_111_0_e1374,) = {
    if (w[202] == 0.0) {
        let noise_metadata_schedule_111_0_e1371: f64 = (w[41]).powf(params[107]);
        let noise_metadata_schedule_111_0_e1372: f64 = (params[17] * noise_metadata_schedule_111_0_e1371);
        (noise_metadata_schedule_111_0_e1372,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_111_0_e1374;
        }
        if (active[0] & 0x200) != 0 {
            let noise_metadata_schedule_112_0_e1376: f64 = if self.param_given[106] { 1.0 } else { 0.0 };
            w[203] = noise_metadata_schedule_112_0_e1376;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_113_0_e1384,) = {
    if (w[203] != 0.0) {
        let noise_metadata_schedule_113_0_e1381: f64 = (w[41]).powf(params[106]);
        let noise_metadata_schedule_113_0_e1382: f64 = (params[21] * noise_metadata_schedule_113_0_e1381);
        (noise_metadata_schedule_113_0_e1382,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_113_0_e1384;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_114_0_e1393,) = {
    if (w[203] == 0.0) {
        let noise_metadata_schedule_114_0_e1390: f64 = (w[41]).powf(params[104]);
        let noise_metadata_schedule_114_0_e1391: f64 = (params[21] * noise_metadata_schedule_114_0_e1390);
        (noise_metadata_schedule_114_0_e1391,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_114_0_e1393;
        }
        if (active[0] & 0x400) != 0 {
            let noise_metadata_schedule_115_0_e1395: f64 = if self.param_given[105] { 1.0 } else { 0.0 };
            w[204] = noise_metadata_schedule_115_0_e1395;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_116_0_e1403,) = {
    if (w[204] != 0.0) {
        let noise_metadata_schedule_116_0_e1400: f64 = (w[41]).powf(params[105]);
        let noise_metadata_schedule_116_0_e1401: f64 = (params[22] * noise_metadata_schedule_116_0_e1400);
        (noise_metadata_schedule_116_0_e1401,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_116_0_e1403;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_117_0_e1412,) = {
    if (w[204] == 0.0) {
        let noise_metadata_schedule_117_0_e1409: f64 = (w[41]).powf(params[104]);
        let noise_metadata_schedule_117_0_e1410: f64 = (params[22] * noise_metadata_schedule_117_0_e1409);
        (noise_metadata_schedule_117_0_e1410,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_117_0_e1412;
        }
        if (active[0] & 0x800) != 0 {
            let noise_metadata_schedule_118_0_e1416: f64 = (w[41]).powf(params[103]);
            let noise_metadata_schedule_118_0_e1417: f64 = (params[23] * noise_metadata_schedule_118_0_e1416);
            w[16] = noise_metadata_schedule_118_0_e1417;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_119_0_e1421: f64 = (w[41]).powf(params[111]);
            let noise_metadata_schedule_119_0_e1422: f64 = (params[24] * noise_metadata_schedule_119_0_e1421);
            w[17] = noise_metadata_schedule_119_0_e1422;
        }
        if (active[0] & 0x1000) != 0 {
            let noise_metadata_schedule_120_0_e1424: f64 = if self.param_given[110] { 1.0 } else { 0.0 };
            w[205] = noise_metadata_schedule_120_0_e1424;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_121_0_e1432,) = {
    if (w[205] != 0.0) {
        let noise_metadata_schedule_121_0_e1429: f64 = (w[41]).powf(params[110]);
        let noise_metadata_schedule_121_0_e1430: f64 = (params[25] * noise_metadata_schedule_121_0_e1429);
        (noise_metadata_schedule_121_0_e1430,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_121_0_e1432;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_122_0_e1441,) = {
    if (w[205] == 0.0) {
        let noise_metadata_schedule_122_0_e1438: f64 = (w[41]).powf(params[107]);
        let noise_metadata_schedule_122_0_e1439: f64 = (params[25] * noise_metadata_schedule_122_0_e1438);
        (noise_metadata_schedule_122_0_e1439,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_122_0_e1441;
        }
        if (active[0] & 0x3410) != 0 {
            let noise_metadata_schedule_124_0_e1453: f64 = (params[122] / params[28]);
            let noise_metadata_schedule_124_0_e1454: f64 = (w[41]).powf(noise_metadata_schedule_124_0_e1453);
            let noise_metadata_schedule_124_0_e1455: f64 = (params[26] * noise_metadata_schedule_124_0_e1454);
            let noise_metadata_schedule_124_0_e1457: f64 = (-params[113]);
            let noise_metadata_schedule_124_0_e1460: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_124_0_e1461: f64 = (noise_metadata_schedule_124_0_e1457 * noise_metadata_schedule_124_0_e1460);
            let noise_metadata_schedule_124_0_e1464: f64 = (w[73] * params[28]);
            let noise_metadata_schedule_124_0_e1465: f64 = (noise_metadata_schedule_124_0_e1461 / noise_metadata_schedule_124_0_e1464);
            let noise_metadata_schedule_124_0_e1466: f64 = (noise_metadata_schedule_124_0_e1465).exp();
            let noise_metadata_schedule_124_0_e1467: f64 = (noise_metadata_schedule_124_0_e1455 * noise_metadata_schedule_124_0_e1466);
            w[0] = noise_metadata_schedule_124_0_e1467;
        }
        if (active[0] & 0x3410) != 0 {
            let noise_metadata_schedule_125_0_e1472: f64 = (params[125] / params[29]);
            let noise_metadata_schedule_125_0_e1473: f64 = (w[41]).powf(noise_metadata_schedule_125_0_e1472);
            let noise_metadata_schedule_125_0_e1474: f64 = (params[27] * noise_metadata_schedule_125_0_e1473);
            let noise_metadata_schedule_125_0_e1476: f64 = (-params[121]);
            let noise_metadata_schedule_125_0_e1479: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_125_0_e1480: f64 = (noise_metadata_schedule_125_0_e1476 * noise_metadata_schedule_125_0_e1479);
            let noise_metadata_schedule_125_0_e1483: f64 = (w[73] * params[29]);
            let noise_metadata_schedule_125_0_e1484: f64 = (noise_metadata_schedule_125_0_e1480 / noise_metadata_schedule_125_0_e1483);
            let noise_metadata_schedule_125_0_e1485: f64 = (noise_metadata_schedule_125_0_e1484).exp();
            let noise_metadata_schedule_125_0_e1486: f64 = (noise_metadata_schedule_125_0_e1474 * noise_metadata_schedule_125_0_e1485);
            w[1] = noise_metadata_schedule_125_0_e1486;
        }
        if (active[0] & 0x3000) != 0 {
            let noise_metadata_schedule_126_0_e1491: f64 = (params[122] / params[33]);
            let noise_metadata_schedule_126_0_e1492: f64 = (w[41]).powf(noise_metadata_schedule_126_0_e1491);
            let noise_metadata_schedule_126_0_e1493: f64 = (params[31] * noise_metadata_schedule_126_0_e1492);
            let noise_metadata_schedule_126_0_e1495: f64 = (-params[120]);
            let noise_metadata_schedule_126_0_e1498: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_126_0_e1499: f64 = (noise_metadata_schedule_126_0_e1495 * noise_metadata_schedule_126_0_e1498);
            let noise_metadata_schedule_126_0_e1502: f64 = (w[73] * params[33]);
            let noise_metadata_schedule_126_0_e1503: f64 = (noise_metadata_schedule_126_0_e1499 / noise_metadata_schedule_126_0_e1502);
            let noise_metadata_schedule_126_0_e1504: f64 = (noise_metadata_schedule_126_0_e1503).exp();
            let noise_metadata_schedule_126_0_e1505: f64 = (noise_metadata_schedule_126_0_e1493 * noise_metadata_schedule_126_0_e1504);
            w[5] = noise_metadata_schedule_126_0_e1505;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_127_0_e1510: f64 = (params[123] / params[56]);
            let noise_metadata_schedule_127_0_e1511: f64 = (w[41]).powf(noise_metadata_schedule_127_0_e1510);
            let noise_metadata_schedule_127_0_e1512: f64 = (params[54] * noise_metadata_schedule_127_0_e1511);
            let noise_metadata_schedule_127_0_e1514: f64 = (-params[114]);
            let noise_metadata_schedule_127_0_e1517: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_127_0_e1518: f64 = (noise_metadata_schedule_127_0_e1514 * noise_metadata_schedule_127_0_e1517);
            let noise_metadata_schedule_127_0_e1521: f64 = (w[73] * params[56]);
            let noise_metadata_schedule_127_0_e1522: f64 = (noise_metadata_schedule_127_0_e1518 / noise_metadata_schedule_127_0_e1521);
            let noise_metadata_schedule_127_0_e1523: f64 = (noise_metadata_schedule_127_0_e1522).exp();
            let noise_metadata_schedule_127_0_e1524: f64 = (noise_metadata_schedule_127_0_e1512 * noise_metadata_schedule_127_0_e1523);
            w[3] = noise_metadata_schedule_127_0_e1524;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_128_0_e1529: f64 = (params[124] / params[59]);
            let noise_metadata_schedule_128_0_e1530: f64 = (w[41]).powf(noise_metadata_schedule_128_0_e1529);
            let noise_metadata_schedule_128_0_e1531: f64 = (params[58] * noise_metadata_schedule_128_0_e1530);
            let noise_metadata_schedule_128_0_e1533: f64 = (-params[117]);
            let noise_metadata_schedule_128_0_e1536: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_128_0_e1537: f64 = (noise_metadata_schedule_128_0_e1533 * noise_metadata_schedule_128_0_e1536);
            let noise_metadata_schedule_128_0_e1540: f64 = (w[73] * params[59]);
            let noise_metadata_schedule_128_0_e1541: f64 = (noise_metadata_schedule_128_0_e1537 / noise_metadata_schedule_128_0_e1540);
            let noise_metadata_schedule_128_0_e1542: f64 = (noise_metadata_schedule_128_0_e1541).exp();
            let noise_metadata_schedule_128_0_e1543: f64 = (noise_metadata_schedule_128_0_e1531 * noise_metadata_schedule_128_0_e1542);
            w[6] = noise_metadata_schedule_128_0_e1543;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_131_0_e1586: f64 = (params[123] / params[61]);
            let noise_metadata_schedule_131_0_e1587: f64 = (w[41]).powf(noise_metadata_schedule_131_0_e1586);
            let noise_metadata_schedule_131_0_e1588: f64 = (params[64] * noise_metadata_schedule_131_0_e1587);
            let noise_metadata_schedule_131_0_e1590: f64 = (-params[115]);
            let noise_metadata_schedule_131_0_e1593: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_131_0_e1594: f64 = (noise_metadata_schedule_131_0_e1590 * noise_metadata_schedule_131_0_e1593);
            let noise_metadata_schedule_131_0_e1597: f64 = (w[73] * params[61]);
            let noise_metadata_schedule_131_0_e1598: f64 = (noise_metadata_schedule_131_0_e1594 / noise_metadata_schedule_131_0_e1597);
            let noise_metadata_schedule_131_0_e1599: f64 = (noise_metadata_schedule_131_0_e1598).exp();
            let noise_metadata_schedule_131_0_e1600: f64 = (noise_metadata_schedule_131_0_e1588 * noise_metadata_schedule_131_0_e1599);
            w[8] = noise_metadata_schedule_131_0_e1600;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_132_0_e1605: f64 = (params[124] / params[63]);
            let noise_metadata_schedule_132_0_e1606: f64 = (w[41]).powf(noise_metadata_schedule_132_0_e1605);
            let noise_metadata_schedule_132_0_e1607: f64 = (params[65] * noise_metadata_schedule_132_0_e1606);
            let noise_metadata_schedule_132_0_e1609: f64 = (-params[118]);
            let noise_metadata_schedule_132_0_e1612: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_132_0_e1613: f64 = (noise_metadata_schedule_132_0_e1609 * noise_metadata_schedule_132_0_e1612);
            let noise_metadata_schedule_132_0_e1616: f64 = (w[73] * params[63]);
            let noise_metadata_schedule_132_0_e1617: f64 = (noise_metadata_schedule_132_0_e1613 / noise_metadata_schedule_132_0_e1616);
            let noise_metadata_schedule_132_0_e1618: f64 = (noise_metadata_schedule_132_0_e1617).exp();
            let noise_metadata_schedule_132_0_e1619: f64 = (noise_metadata_schedule_132_0_e1607 * noise_metadata_schedule_132_0_e1618);
            w[9] = noise_metadata_schedule_132_0_e1619;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_135_0_e1662: f64 = (w[42] * params[129]);
            let noise_metadata_schedule_135_0_e1663: f64 = (1.0 + noise_metadata_schedule_135_0_e1662);
            let noise_metadata_schedule_135_0_e1664: f64 = (params[28] * noise_metadata_schedule_135_0_e1663);
            w[27] = noise_metadata_schedule_135_0_e1664;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_136_0_e1669: f64 = (w[42] * params[129]);
            let noise_metadata_schedule_136_0_e1670: f64 = (1.0 + noise_metadata_schedule_136_0_e1669);
            let noise_metadata_schedule_136_0_e1671: f64 = (params[29] * noise_metadata_schedule_136_0_e1670);
            w[28] = noise_metadata_schedule_136_0_e1671;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_139_0_e1692: f64 = (w[42] * params[92]);
            let noise_metadata_schedule_139_0_e1693: f64 = (params[91] + noise_metadata_schedule_139_0_e1692);
            let noise_metadata_schedule_139_0_e1694: f64 = (w[42] * noise_metadata_schedule_139_0_e1693);
            let noise_metadata_schedule_139_0_e1695: f64 = (1.0 + noise_metadata_schedule_139_0_e1694);
            let noise_metadata_schedule_139_0_e1696: f64 = (params[88] * noise_metadata_schedule_139_0_e1695);
            w[31] = noise_metadata_schedule_139_0_e1696;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_140_0_e1701: f64 = (w[42] * params[93]);
            let noise_metadata_schedule_140_0_e1702: f64 = (1.0 + noise_metadata_schedule_140_0_e1701);
            let noise_metadata_schedule_140_0_e1703: f64 = (params[89] * noise_metadata_schedule_140_0_e1702);
            w[32] = noise_metadata_schedule_140_0_e1703;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_141_0_e1707: f64 = (w[73] / w[41]);
            let noise_metadata_schedule_141_0_e1708: f64 = (2.0 * noise_metadata_schedule_141_0_e1707);
            let noise_metadata_schedule_141_0_e1711: f64 = (0.5 * params[37]);
            let noise_metadata_schedule_141_0_e1713: f64 = (noise_metadata_schedule_141_0_e1711 * w[41]);
            let noise_metadata_schedule_141_0_e1715: f64 = (noise_metadata_schedule_141_0_e1713 / w[73]);
            let noise_metadata_schedule_141_0_e1716: f64 = (noise_metadata_schedule_141_0_e1715).exp();
            let noise_metadata_schedule_141_0_e1718: f64 = (-0.5);
            let noise_metadata_schedule_141_0_e1720: f64 = (noise_metadata_schedule_141_0_e1718 * params[37]);
            let noise_metadata_schedule_141_0_e1722: f64 = (noise_metadata_schedule_141_0_e1720 * w[41]);
            let noise_metadata_schedule_141_0_e1724: f64 = (noise_metadata_schedule_141_0_e1722 / w[73]);
            let noise_metadata_schedule_141_0_e1725: f64 = (noise_metadata_schedule_141_0_e1724).exp();
            let noise_metadata_schedule_141_0_e1726: f64 = (noise_metadata_schedule_141_0_e1716 - noise_metadata_schedule_141_0_e1725);
            let noise_metadata_schedule_141_0_e1727: f64 = (noise_metadata_schedule_141_0_e1726).ln();
            let noise_metadata_schedule_141_0_e1728: f64 = (noise_metadata_schedule_141_0_e1708 * noise_metadata_schedule_141_0_e1727);
            w[206] = noise_metadata_schedule_141_0_e1728;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_142_0_e1731: f64 = (w[206] * w[41]);
            let noise_metadata_schedule_142_0_e1734: f64 = (3.0 * w[73]);
            let noise_metadata_schedule_142_0_e1736: f64 = (w[41]).ln();
            let noise_metadata_schedule_142_0_e1737: f64 = (noise_metadata_schedule_142_0_e1734 * noise_metadata_schedule_142_0_e1736);
            let noise_metadata_schedule_142_0_e1738: f64 = (noise_metadata_schedule_142_0_e1731 - noise_metadata_schedule_142_0_e1737);
            let noise_metadata_schedule_142_0_e1742: f64 = (w[41] - 1.0);
            let noise_metadata_schedule_142_0_e1743: f64 = (params[114] * noise_metadata_schedule_142_0_e1742);
            let noise_metadata_schedule_142_0_e1744: f64 = (noise_metadata_schedule_142_0_e1738 - noise_metadata_schedule_142_0_e1743);
            w[207] = noise_metadata_schedule_142_0_e1744;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_143_0_e1748: f64 = (2.0 * w[73]);
            let noise_metadata_schedule_143_0_e1754: f64 = (-w[207]);
            let noise_metadata_schedule_143_0_e1756: f64 = (noise_metadata_schedule_143_0_e1754 / w[73]);
            let noise_metadata_schedule_143_0_e1757: f64 = (noise_metadata_schedule_143_0_e1756).exp();
            let noise_metadata_schedule_143_0_e1758: f64 = (4.0 * noise_metadata_schedule_143_0_e1757);
            let noise_metadata_schedule_143_0_e1759: f64 = (1.0 + noise_metadata_schedule_143_0_e1758);
            let noise_metadata_schedule_143_0_e1760: f64 = (noise_metadata_schedule_143_0_e1759).sqrt();
            let noise_metadata_schedule_143_0_e1761: f64 = (1.0 + noise_metadata_schedule_143_0_e1760);
            let noise_metadata_schedule_143_0_e1762: f64 = (0.5 * noise_metadata_schedule_143_0_e1761);
            let noise_metadata_schedule_143_0_e1763: f64 = (noise_metadata_schedule_143_0_e1762).ln();
            let noise_metadata_schedule_143_0_e1764: f64 = (noise_metadata_schedule_143_0_e1748 * noise_metadata_schedule_143_0_e1763);
            let noise_metadata_schedule_143_0_e1765: f64 = (w[207] + noise_metadata_schedule_143_0_e1764);
            w[20] = noise_metadata_schedule_143_0_e1765;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_144_0_e1769: f64 = (w[73] / w[41]);
            let noise_metadata_schedule_144_0_e1770: f64 = (2.0 * noise_metadata_schedule_144_0_e1769);
            let noise_metadata_schedule_144_0_e1773: f64 = (0.5 * params[42]);
            let noise_metadata_schedule_144_0_e1775: f64 = (noise_metadata_schedule_144_0_e1773 * w[41]);
            let noise_metadata_schedule_144_0_e1777: f64 = (noise_metadata_schedule_144_0_e1775 / w[73]);
            let noise_metadata_schedule_144_0_e1778: f64 = (noise_metadata_schedule_144_0_e1777).exp();
            let noise_metadata_schedule_144_0_e1780: f64 = (-0.5);
            let noise_metadata_schedule_144_0_e1782: f64 = (noise_metadata_schedule_144_0_e1780 * params[42]);
            let noise_metadata_schedule_144_0_e1784: f64 = (noise_metadata_schedule_144_0_e1782 * w[41]);
            let noise_metadata_schedule_144_0_e1786: f64 = (noise_metadata_schedule_144_0_e1784 / w[73]);
            let noise_metadata_schedule_144_0_e1787: f64 = (noise_metadata_schedule_144_0_e1786).exp();
            let noise_metadata_schedule_144_0_e1788: f64 = (noise_metadata_schedule_144_0_e1778 - noise_metadata_schedule_144_0_e1787);
            let noise_metadata_schedule_144_0_e1789: f64 = (noise_metadata_schedule_144_0_e1788).ln();
            let noise_metadata_schedule_144_0_e1790: f64 = (noise_metadata_schedule_144_0_e1770 * noise_metadata_schedule_144_0_e1789);
            w[208] = noise_metadata_schedule_144_0_e1790;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_145_0_e1793: f64 = (w[208] * w[41]);
            let noise_metadata_schedule_145_0_e1796: f64 = (3.0 * w[73]);
            let noise_metadata_schedule_145_0_e1798: f64 = (w[41]).ln();
            let noise_metadata_schedule_145_0_e1799: f64 = (noise_metadata_schedule_145_0_e1796 * noise_metadata_schedule_145_0_e1798);
            let noise_metadata_schedule_145_0_e1800: f64 = (noise_metadata_schedule_145_0_e1793 - noise_metadata_schedule_145_0_e1799);
            let noise_metadata_schedule_145_0_e1804: f64 = (w[41] - 1.0);
            let noise_metadata_schedule_145_0_e1805: f64 = (params[115] * noise_metadata_schedule_145_0_e1804);
            let noise_metadata_schedule_145_0_e1806: f64 = (noise_metadata_schedule_145_0_e1800 - noise_metadata_schedule_145_0_e1805);
            w[209] = noise_metadata_schedule_145_0_e1806;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_146_0_e1810: f64 = (2.0 * w[73]);
            let noise_metadata_schedule_146_0_e1816: f64 = (-w[209]);
            let noise_metadata_schedule_146_0_e1818: f64 = (noise_metadata_schedule_146_0_e1816 / w[73]);
            let noise_metadata_schedule_146_0_e1819: f64 = (noise_metadata_schedule_146_0_e1818).exp();
            let noise_metadata_schedule_146_0_e1820: f64 = (4.0 * noise_metadata_schedule_146_0_e1819);
            let noise_metadata_schedule_146_0_e1821: f64 = (1.0 + noise_metadata_schedule_146_0_e1820);
            let noise_metadata_schedule_146_0_e1822: f64 = (noise_metadata_schedule_146_0_e1821).sqrt();
            let noise_metadata_schedule_146_0_e1823: f64 = (1.0 + noise_metadata_schedule_146_0_e1822);
            let noise_metadata_schedule_146_0_e1824: f64 = (0.5 * noise_metadata_schedule_146_0_e1823);
            let noise_metadata_schedule_146_0_e1825: f64 = (noise_metadata_schedule_146_0_e1824).ln();
            let noise_metadata_schedule_146_0_e1826: f64 = (noise_metadata_schedule_146_0_e1810 * noise_metadata_schedule_146_0_e1825);
            let noise_metadata_schedule_146_0_e1827: f64 = (w[209] + noise_metadata_schedule_146_0_e1826);
            w[21] = noise_metadata_schedule_146_0_e1827;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_154_0_e1921: f64 = (w[41]).powf(params[122]);
            let noise_metadata_schedule_154_0_e1922: f64 = (params[19] * noise_metadata_schedule_154_0_e1921);
            let noise_metadata_schedule_154_0_e1924: f64 = (-params[113]);
            let noise_metadata_schedule_154_0_e1927: f64 = (1.0 - w[41]);
            let noise_metadata_schedule_154_0_e1928: f64 = (noise_metadata_schedule_154_0_e1924 * noise_metadata_schedule_154_0_e1927);
            let noise_metadata_schedule_154_0_e1930: f64 = (noise_metadata_schedule_154_0_e1928 / w[73]);
            let noise_metadata_schedule_154_0_e1931: f64 = (noise_metadata_schedule_154_0_e1930).exp();
            let noise_metadata_schedule_154_0_e1932: f64 = (noise_metadata_schedule_154_0_e1922 * noise_metadata_schedule_154_0_e1931);
            w[33] = noise_metadata_schedule_154_0_e1932;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_155_0_e1936: f64 = (w[41]).powf(params[112]);
            let noise_metadata_schedule_155_0_e1937: f64 = (params[18] * noise_metadata_schedule_155_0_e1936);
            w[34] = noise_metadata_schedule_155_0_e1937;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_156_0_e1939: f64 = (-w[31]);
            let noise_metadata_schedule_156_0_e1942: f64 = (w[32] * w[73]);
            let noise_metadata_schedule_156_0_e1943: f64 = (noise_metadata_schedule_156_0_e1939 / noise_metadata_schedule_156_0_e1942);
            let noise_metadata_schedule_156_0_e1944: f64 = (noise_metadata_schedule_156_0_e1943).exp();
            w[35] = noise_metadata_schedule_156_0_e1944;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_157_0_e1949: f64 = (w[42] * params[130]);
            let noise_metadata_schedule_157_0_e1950: f64 = (1.0 + noise_metadata_schedule_157_0_e1949);
            let noise_metadata_schedule_157_0_e1951: f64 = (params[70] * noise_metadata_schedule_157_0_e1950);
            w[36] = noise_metadata_schedule_157_0_e1951;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_158_0_e1956: f64 = (w[42] * params[131]);
            let noise_metadata_schedule_158_0_e1957: f64 = (1.0 + noise_metadata_schedule_158_0_e1956);
            let noise_metadata_schedule_158_0_e1958: f64 = (params[71] * noise_metadata_schedule_158_0_e1957);
            w[37] = noise_metadata_schedule_158_0_e1958;
        }
        if (active[0] & 0x80) != 0 {
            let (noise_metadata_schedule_159_0_e1966,) = {
    if (w[12] > 0.001) {
        let noise_metadata_schedule_159_0_e1964: f64 = (1.0 / w[12]);
        (noise_metadata_schedule_159_0_e1964,)
    } else {
        (1000.0,)
    }
};
            w[53] = noise_metadata_schedule_159_0_e1966;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_160_0_e1974,) = {
    if (w[13] > 0.001) {
        let noise_metadata_schedule_160_0_e1972: f64 = (1.0 / w[13]);
        (noise_metadata_schedule_160_0_e1972,)
    } else {
        (1000.0,)
    }
};
            w[54] = noise_metadata_schedule_160_0_e1974;
        }
        if (active[0] & 0x200) != 0 {
            let (noise_metadata_schedule_161_0_e1982,) = {
    if (w[14] > 0.001) {
        let noise_metadata_schedule_161_0_e1980: f64 = (1.0 / w[14]);
        (noise_metadata_schedule_161_0_e1980,)
    } else {
        (1000.0,)
    }
};
            w[55] = noise_metadata_schedule_161_0_e1982;
        }
        if (active[0] & 0x400) != 0 {
            let (noise_metadata_schedule_162_0_e1990,) = {
    if (w[15] > 0.001) {
        let noise_metadata_schedule_162_0_e1988: f64 = (1.0 / w[15]);
        (noise_metadata_schedule_162_0_e1988,)
    } else {
        (1000.0,)
    }
};
            w[56] = noise_metadata_schedule_162_0_e1990;
        }
        if (active[0] & 0x800) != 0 {
            let (noise_metadata_schedule_163_0_e1998,) = {
    if (w[16] > 0.001) {
        let noise_metadata_schedule_163_0_e1996: f64 = (1.0 / w[16]);
        (noise_metadata_schedule_163_0_e1996,)
    } else {
        (1000.0,)
    }
};
            w[57] = noise_metadata_schedule_163_0_e1998;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_164_0_e2006,) = {
    if (w[18] > 0.001) {
        let noise_metadata_schedule_164_0_e2004: f64 = (1.0 / w[18]);
        (noise_metadata_schedule_164_0_e2004,)
    } else {
        (1000.0,)
    }
};
            w[58] = noise_metadata_schedule_164_0_e2006;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_165_0_e2014,) = {
    if (w[17] > 0.001) {
        let noise_metadata_schedule_165_0_e2012: f64 = (1.0 / w[17]);
        (noise_metadata_schedule_165_0_e2012,)
    } else {
        (1000.0,)
    }
};
            w[59] = noise_metadata_schedule_165_0_e2014;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_167_0_e2030,) = {
    if (w[36] > 0.0) {
        let noise_metadata_schedule_167_0_e2028: f64 = (1.0 / w[36]);
        (noise_metadata_schedule_167_0_e2028,)
    } else {
        (0.0,)
    }
};
            w[43] = noise_metadata_schedule_167_0_e2030;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_168_0_e2038,) = {
    if (w[37] > 0.0) {
        let noise_metadata_schedule_168_0_e2036: f64 = (1.0 / w[37]);
        (noise_metadata_schedule_168_0_e2036,)
    } else {
        (0.0,)
    }
};
            w[44] = noise_metadata_schedule_168_0_e2038;
        }
        if (active[0] & 0x3410) != 0 {
            let (noise_metadata_schedule_169_0_e2046,) = {
    if (w[2] > 0.0) {
        let noise_metadata_schedule_169_0_e2044: f64 = (1.0 / w[2]);
        (noise_metadata_schedule_169_0_e2044,)
    } else {
        (0.0,)
    }
};
            w[45] = noise_metadata_schedule_169_0_e2046;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_170_0_e2054,) = {
    if (w[34] > 0.0) {
        let noise_metadata_schedule_170_0_e2052: f64 = (1.0 / w[34]);
        (noise_metadata_schedule_170_0_e2052,)
    } else {
        (0.0,)
    }
};
            w[48] = noise_metadata_schedule_170_0_e2054;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_171_0_e2057: f64 = (w[162] * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
            w[143] = noise_metadata_schedule_171_0_e2057;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_172_0_e2060: f64 = (w[162] * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[9])));
            w[145] = noise_metadata_schedule_172_0_e2060;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_173_0_e2063: f64 = (w[162] * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[6])));
            w[144] = noise_metadata_schedule_173_0_e2063;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_174_0_e2066: f64 = (w[162] * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[5])));
            w[148] = noise_metadata_schedule_174_0_e2066;
        }
        if (active[0] & 0x316f) != 0 {
            let noise_metadata_schedule_176_0_e2072: f64 = (w[162] * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[10])));
            w[146] = noise_metadata_schedule_176_0_e2072;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_181_0_e2081: f64 = (w[162] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            w[154] = noise_metadata_schedule_181_0_e2081;
        }
        if (active[0] & 0x216f) != 0 {
            let noise_metadata_schedule_186_0_e2088: f64 = (w[162] * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
            w[147] = noise_metadata_schedule_186_0_e2088;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_191_0_e2096: f64 = (-w[20]);
            let noise_metadata_schedule_191_0_e2098: f64 = (noise_metadata_schedule_191_0_e2096 * params[34]);
            w[212] = noise_metadata_schedule_191_0_e2098;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_192_0_e2101: f64 = if params[39] <= 0.0 { 1.0 } else { 0.0 };
            w[223] = noise_metadata_schedule_192_0_e2101;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_193_0_e2107,) = {
    if (w[223] != 0.0) {
        let noise_metadata_schedule_193_0_e2105: f64 = (w[143] + w[212]);
        (noise_metadata_schedule_193_0_e2105,)
    } else {
        (w[213],)
    }
};
            w[213] = noise_metadata_schedule_193_0_e2107;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_194_0_e2110: f64 = if w[213] > 0.0 { 1.0 } else { 0.0 };
            w[224] = noise_metadata_schedule_194_0_e2110;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_195_0_e2121,) = {
    if ((w[223] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_195_0_e2116: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_195_0_e2118: f64 = (-params[38]);
        let noise_metadata_schedule_195_0_e2119: f64 = (noise_metadata_schedule_195_0_e2116).powf(noise_metadata_schedule_195_0_e2118);
        (noise_metadata_schedule_195_0_e2119,)
    } else {
        (w[214],)
    }
};
            w[214] = noise_metadata_schedule_195_0_e2121;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_196_0_e2139,) = {
    if ((w[223] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_196_0_e2130: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_196_0_e2131: f64 = (w[214] * noise_metadata_schedule_196_0_e2130);
        let noise_metadata_schedule_196_0_e2132: f64 = (1.0 - noise_metadata_schedule_196_0_e2131);
        let noise_metadata_schedule_196_0_e2133: f64 = (w[20] * noise_metadata_schedule_196_0_e2132);
        let noise_metadata_schedule_196_0_e2136: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_196_0_e2137: f64 = (noise_metadata_schedule_196_0_e2133 / noise_metadata_schedule_196_0_e2136);
        (noise_metadata_schedule_196_0_e2137,)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_196_0_e2139;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_197_0_e2161,) = {
    if ((w[223] != 0.0) && (w[224] != 0.0)) {
        let noise_metadata_schedule_197_0_e2147: f64 = (0.5 * params[38]);
        let noise_metadata_schedule_197_0_e2149: f64 = (noise_metadata_schedule_197_0_e2147 * w[213]);
        let noise_metadata_schedule_197_0_e2153: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_197_0_e2154: f64 = (w[20] * noise_metadata_schedule_197_0_e2153);
        let noise_metadata_schedule_197_0_e2155: f64 = (noise_metadata_schedule_197_0_e2149 / noise_metadata_schedule_197_0_e2154);
        let noise_metadata_schedule_197_0_e2156: f64 = (1.0 + noise_metadata_schedule_197_0_e2155);
        let noise_metadata_schedule_197_0_e2157: f64 = (w[213] * noise_metadata_schedule_197_0_e2156);
        let noise_metadata_schedule_197_0_e2159: f64 = (noise_metadata_schedule_197_0_e2157 * w[214]);
        (noise_metadata_schedule_197_0_e2159,)
    } else {
        (w[216],)
    }
};
            w[216] = noise_metadata_schedule_197_0_e2161;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_198_0_e2184,) = {
    if ((w[223] != 0.0) && (w[224] == 0.0)) {
        let noise_metadata_schedule_198_0_e2171: f64 = (w[143] / w[20]);
        let noise_metadata_schedule_198_0_e2172: f64 = (1.0 - noise_metadata_schedule_198_0_e2171);
        let noise_metadata_schedule_198_0_e2175: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_198_0_e2176: f64 = (noise_metadata_schedule_198_0_e2172).powf(noise_metadata_schedule_198_0_e2175);
        let noise_metadata_schedule_198_0_e2177: f64 = (1.0 - noise_metadata_schedule_198_0_e2176);
        let noise_metadata_schedule_198_0_e2178: f64 = (w[20] * noise_metadata_schedule_198_0_e2177);
        let noise_metadata_schedule_198_0_e2181: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_198_0_e2182: f64 = (noise_metadata_schedule_198_0_e2178 / noise_metadata_schedule_198_0_e2181);
        (noise_metadata_schedule_198_0_e2182,)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_198_0_e2184;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_199_0_e2191,) = {
    if ((w[223] != 0.0) && (w[224] == 0.0)) {
        (0.0,)
    } else {
        (w[216],)
    }
};
            w[216] = noise_metadata_schedule_199_0_e2191;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_200_0_e2197,) = {
    if (w[223] != 0.0) {
        let noise_metadata_schedule_200_0_e2195: f64 = (w[215] + w[216]);
        (noise_metadata_schedule_200_0_e2195,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_200_0_e2197;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_201_0_e2211,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_201_0_e2202: f64 = (w[212] * w[212]);
        let noise_metadata_schedule_201_0_e2205: f64 = (4.0 * params[39]);
        let noise_metadata_schedule_201_0_e2207: f64 = (noise_metadata_schedule_201_0_e2205 * params[39]);
        let noise_metadata_schedule_201_0_e2208: f64 = (noise_metadata_schedule_201_0_e2202 + noise_metadata_schedule_201_0_e2207);
        let noise_metadata_schedule_201_0_e2209: f64 = (noise_metadata_schedule_201_0_e2208).sqrt();
        (noise_metadata_schedule_201_0_e2209,)
    } else {
        (w[217],)
    }
};
            w[217] = noise_metadata_schedule_201_0_e2211;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_202_0_e2221,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_202_0_e2215: f64 = (-0.5);
        let noise_metadata_schedule_202_0_e2218: f64 = (w[212] + w[217]);
        let noise_metadata_schedule_202_0_e2219: f64 = (noise_metadata_schedule_202_0_e2215 * noise_metadata_schedule_202_0_e2218);
        (noise_metadata_schedule_202_0_e2219,)
    } else {
        (w[218],)
    }
};
            w[218] = noise_metadata_schedule_202_0_e2221;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_203_0_e2241,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_203_0_e2225: f64 = (-w[20]);
        let noise_metadata_schedule_203_0_e2229: f64 = (w[218] / w[20]);
        let noise_metadata_schedule_203_0_e2230: f64 = (1.0 - noise_metadata_schedule_203_0_e2229);
        let noise_metadata_schedule_203_0_e2233: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_203_0_e2234: f64 = (noise_metadata_schedule_203_0_e2230).powf(noise_metadata_schedule_203_0_e2233);
        let noise_metadata_schedule_203_0_e2235: f64 = (noise_metadata_schedule_203_0_e2225 * noise_metadata_schedule_203_0_e2234);
        let noise_metadata_schedule_203_0_e2238: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_203_0_e2239: f64 = (noise_metadata_schedule_203_0_e2235 / noise_metadata_schedule_203_0_e2238);
        (noise_metadata_schedule_203_0_e2239,)
    } else {
        (w[219],)
    }
};
            w[219] = noise_metadata_schedule_203_0_e2241;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_204_0_e2248,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_204_0_e2246: f64 = (w[143] + w[212]);
        (noise_metadata_schedule_204_0_e2246,)
    } else {
        (w[220],)
    }
};
            w[220] = noise_metadata_schedule_204_0_e2248;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_205_0_e2262,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_205_0_e2253: f64 = (w[220] * w[220]);
        let noise_metadata_schedule_205_0_e2256: f64 = (4.0 * params[39]);
        let noise_metadata_schedule_205_0_e2258: f64 = (noise_metadata_schedule_205_0_e2256 * params[39]);
        let noise_metadata_schedule_205_0_e2259: f64 = (noise_metadata_schedule_205_0_e2253 + noise_metadata_schedule_205_0_e2258);
        let noise_metadata_schedule_205_0_e2260: f64 = (noise_metadata_schedule_205_0_e2259).sqrt();
        (noise_metadata_schedule_205_0_e2260,)
    } else {
        (w[221],)
    }
};
            w[221] = noise_metadata_schedule_205_0_e2262;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_206_0_e2273,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_206_0_e2268: f64 = (w[220] - w[221]);
        let noise_metadata_schedule_206_0_e2269: f64 = (0.5 * noise_metadata_schedule_206_0_e2268);
        let noise_metadata_schedule_206_0_e2271: f64 = (noise_metadata_schedule_206_0_e2269 - w[212]);
        (noise_metadata_schedule_206_0_e2271,)
    } else {
        (w[222],)
    }
};
            w[222] = noise_metadata_schedule_206_0_e2273;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_207_0_e2293,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_207_0_e2277: f64 = (-w[20]);
        let noise_metadata_schedule_207_0_e2281: f64 = (w[222] / w[20]);
        let noise_metadata_schedule_207_0_e2282: f64 = (1.0 - noise_metadata_schedule_207_0_e2281);
        let noise_metadata_schedule_207_0_e2285: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_207_0_e2286: f64 = (noise_metadata_schedule_207_0_e2282).powf(noise_metadata_schedule_207_0_e2285);
        let noise_metadata_schedule_207_0_e2287: f64 = (noise_metadata_schedule_207_0_e2277 * noise_metadata_schedule_207_0_e2286);
        let noise_metadata_schedule_207_0_e2290: f64 = (1.0 - params[38]);
        let noise_metadata_schedule_207_0_e2291: f64 = (noise_metadata_schedule_207_0_e2287 / noise_metadata_schedule_207_0_e2290);
        (noise_metadata_schedule_207_0_e2291,)
    } else {
        (w[215],)
    }
};
            w[215] = noise_metadata_schedule_207_0_e2293;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_208_0_e2331,) = {
    if (w[223] == 0.0) {
        let noise_metadata_schedule_208_0_e2299: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_208_0_e2301: f64 = (-params[38]);
        let noise_metadata_schedule_208_0_e2302: f64 = (noise_metadata_schedule_208_0_e2299).powf(noise_metadata_schedule_208_0_e2301);
        let noise_metadata_schedule_208_0_e2305: f64 = (w[143] - w[222]);
        let noise_metadata_schedule_208_0_e2307: f64 = (noise_metadata_schedule_208_0_e2305 + w[218]);
        let noise_metadata_schedule_208_0_e2308: f64 = (noise_metadata_schedule_208_0_e2302 * noise_metadata_schedule_208_0_e2307);
        let noise_metadata_schedule_208_0_e2312: f64 = (0.5 * params[38]);
        let noise_metadata_schedule_208_0_e2315: f64 = (w[143] - w[222]);
        let noise_metadata_schedule_208_0_e2317: f64 = (noise_metadata_schedule_208_0_e2315 + w[218]);
        let noise_metadata_schedule_208_0_e2318: f64 = (noise_metadata_schedule_208_0_e2312 * noise_metadata_schedule_208_0_e2317);
        let noise_metadata_schedule_208_0_e2322: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_208_0_e2323: f64 = (w[20] * noise_metadata_schedule_208_0_e2322);
        let noise_metadata_schedule_208_0_e2324: f64 = (noise_metadata_schedule_208_0_e2318 / noise_metadata_schedule_208_0_e2323);
        let noise_metadata_schedule_208_0_e2325: f64 = (1.0 + noise_metadata_schedule_208_0_e2324);
        let noise_metadata_schedule_208_0_e2326: f64 = (noise_metadata_schedule_208_0_e2308 * noise_metadata_schedule_208_0_e2325);
        let noise_metadata_schedule_208_0_e2327: f64 = (w[215] + noise_metadata_schedule_208_0_e2326);
        let noise_metadata_schedule_208_0_e2329: f64 = (noise_metadata_schedule_208_0_e2327 - w[219]);
        (noise_metadata_schedule_208_0_e2329,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_208_0_e2331;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_209_0_e2333: f64 = (-w[21]);
            let noise_metadata_schedule_209_0_e2335: f64 = (noise_metadata_schedule_209_0_e2333 * params[34]);
            w[225] = noise_metadata_schedule_209_0_e2335;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_210_0_e2338: f64 = if params[44] <= 0.0 { 1.0 } else { 0.0 };
            w[246] = noise_metadata_schedule_210_0_e2338;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_211_0_e2344,) = {
    if (w[246] != 0.0) {
        let noise_metadata_schedule_211_0_e2342: f64 = (w[144] + w[225]);
        (noise_metadata_schedule_211_0_e2342,)
    } else {
        (w[226],)
    }
};
            w[226] = noise_metadata_schedule_211_0_e2344;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_212_0_e2347: f64 = if w[226] > 0.0 { 1.0 } else { 0.0 };
            w[247] = noise_metadata_schedule_212_0_e2347;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_213_0_e2360,) = {
    if ((w[246] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_213_0_e2353: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_213_0_e2355: f64 = (-1.0);
        let noise_metadata_schedule_213_0_e2357: f64 = (noise_metadata_schedule_213_0_e2355 - params[43]);
        let noise_metadata_schedule_213_0_e2358: f64 = (noise_metadata_schedule_213_0_e2353).powf(noise_metadata_schedule_213_0_e2357);
        (noise_metadata_schedule_213_0_e2358,)
    } else {
        (w[227],)
    }
};
            w[227] = noise_metadata_schedule_213_0_e2360;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_214_0_e2382,) = {
    if ((w[246] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_214_0_e2369: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_214_0_e2370: f64 = (w[227] * noise_metadata_schedule_214_0_e2369);
        let noise_metadata_schedule_214_0_e2373: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_214_0_e2374: f64 = (noise_metadata_schedule_214_0_e2370 * noise_metadata_schedule_214_0_e2373);
        let noise_metadata_schedule_214_0_e2375: f64 = (1.0 - noise_metadata_schedule_214_0_e2374);
        let noise_metadata_schedule_214_0_e2376: f64 = (w[21] * noise_metadata_schedule_214_0_e2375);
        let noise_metadata_schedule_214_0_e2379: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_214_0_e2380: f64 = (noise_metadata_schedule_214_0_e2376 / noise_metadata_schedule_214_0_e2379);
        (noise_metadata_schedule_214_0_e2380,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_214_0_e2382;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_215_0_e2402,) = {
    if ((w[246] != 0.0) && (w[247] != 0.0)) {
        let noise_metadata_schedule_215_0_e2389: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_215_0_e2392: f64 = (0.5 * params[43]);
        let noise_metadata_schedule_215_0_e2394: f64 = (noise_metadata_schedule_215_0_e2392 * w[226]);
        let noise_metadata_schedule_215_0_e2396: f64 = (noise_metadata_schedule_215_0_e2394 / w[21]);
        let noise_metadata_schedule_215_0_e2397: f64 = (noise_metadata_schedule_215_0_e2389 + noise_metadata_schedule_215_0_e2396);
        let noise_metadata_schedule_215_0_e2398: f64 = (w[226] * noise_metadata_schedule_215_0_e2397);
        let noise_metadata_schedule_215_0_e2400: f64 = (noise_metadata_schedule_215_0_e2398 * w[227]);
        (noise_metadata_schedule_215_0_e2400,)
    } else {
        (w[229],)
    }
};
            w[229] = noise_metadata_schedule_215_0_e2402;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_216_0_e2408: f64 = (-params[45]);
            let noise_metadata_schedule_216_0_e2410: f64 = if ((params[45] > 0.0) && (w[144] < noise_metadata_schedule_216_0_e2408)) { 1.0 } else { 0.0 };
            w[248] = noise_metadata_schedule_216_0_e2410;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_217_0_e2449,) = {
    if (((w[246] != 0.0) && (w[247] == 0.0)) && (w[248] != 0.0)) {
        let noise_metadata_schedule_217_0_e2422: f64 = (params[45] / w[21]);
        let noise_metadata_schedule_217_0_e2423: f64 = (1.0 + noise_metadata_schedule_217_0_e2422);
        let noise_metadata_schedule_217_0_e2426: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_217_0_e2427: f64 = (noise_metadata_schedule_217_0_e2423).powf(noise_metadata_schedule_217_0_e2426);
        let noise_metadata_schedule_217_0_e2431: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_217_0_e2434: f64 = (w[144] + params[45]);
        let noise_metadata_schedule_217_0_e2435: f64 = (noise_metadata_schedule_217_0_e2431 * noise_metadata_schedule_217_0_e2434);
        let noise_metadata_schedule_217_0_e2438: f64 = (w[21] + params[45]);
        let noise_metadata_schedule_217_0_e2439: f64 = (noise_metadata_schedule_217_0_e2435 / noise_metadata_schedule_217_0_e2438);
        let noise_metadata_schedule_217_0_e2440: f64 = (1.0 - noise_metadata_schedule_217_0_e2439);
        let noise_metadata_schedule_217_0_e2441: f64 = (noise_metadata_schedule_217_0_e2427 * noise_metadata_schedule_217_0_e2440);
        let noise_metadata_schedule_217_0_e2442: f64 = (1.0 - noise_metadata_schedule_217_0_e2441);
        let noise_metadata_schedule_217_0_e2443: f64 = (w[21] * noise_metadata_schedule_217_0_e2442);
        let noise_metadata_schedule_217_0_e2446: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_217_0_e2447: f64 = (noise_metadata_schedule_217_0_e2443 / noise_metadata_schedule_217_0_e2446);
        (noise_metadata_schedule_217_0_e2447,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_217_0_e2449;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_218_0_e2475,) = {
    if (((w[246] != 0.0) && (w[247] == 0.0)) && (w[248] == 0.0)) {
        let noise_metadata_schedule_218_0_e2462: f64 = (w[144] / w[21]);
        let noise_metadata_schedule_218_0_e2463: f64 = (1.0 - noise_metadata_schedule_218_0_e2462);
        let noise_metadata_schedule_218_0_e2466: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_218_0_e2467: f64 = (noise_metadata_schedule_218_0_e2463).powf(noise_metadata_schedule_218_0_e2466);
        let noise_metadata_schedule_218_0_e2468: f64 = (1.0 - noise_metadata_schedule_218_0_e2467);
        let noise_metadata_schedule_218_0_e2469: f64 = (w[21] * noise_metadata_schedule_218_0_e2468);
        let noise_metadata_schedule_218_0_e2472: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_218_0_e2473: f64 = (noise_metadata_schedule_218_0_e2469 / noise_metadata_schedule_218_0_e2472);
        (noise_metadata_schedule_218_0_e2473,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_218_0_e2475;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_219_0_e2482,) = {
    if ((w[246] != 0.0) && (w[247] == 0.0)) {
        (0.0,)
    } else {
        (w[229],)
    }
};
            w[229] = noise_metadata_schedule_219_0_e2482;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_220_0_e2488,) = {
    if (w[246] != 0.0) {
        let noise_metadata_schedule_220_0_e2486: f64 = (w[228] + w[229]);
        (noise_metadata_schedule_220_0_e2486,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_220_0_e2488;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_221_0_e2495: f64 = if ((params[45] > 0.0) && (params[46] > 0.0)) { 1.0 } else { 0.0 };
            w[249] = noise_metadata_schedule_221_0_e2495;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_222_0_e2508,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_222_0_e2502: f64 = (params[45] + w[225]);
        let noise_metadata_schedule_222_0_e2505: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_222_0_e2506: f64 = (noise_metadata_schedule_222_0_e2502 / noise_metadata_schedule_222_0_e2505);
        (noise_metadata_schedule_222_0_e2506,)
    } else {
        (w[230],)
    }
};
            w[230] = noise_metadata_schedule_222_0_e2508;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_223_0_e2547,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_223_0_e2515: f64 = (2.0 * w[230]);
        let noise_metadata_schedule_223_0_e2518: f64 = (w[230] - 1.0);
        let noise_metadata_schedule_223_0_e2521: f64 = (w[230] - 1.0);
        let noise_metadata_schedule_223_0_e2522: f64 = (noise_metadata_schedule_223_0_e2518 * noise_metadata_schedule_223_0_e2521);
        let noise_metadata_schedule_223_0_e2525: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_223_0_e2527: f64 = (noise_metadata_schedule_223_0_e2525 * params[44]);
        let noise_metadata_schedule_223_0_e2528: f64 = (noise_metadata_schedule_223_0_e2522 + noise_metadata_schedule_223_0_e2527);
        let noise_metadata_schedule_223_0_e2529: f64 = (noise_metadata_schedule_223_0_e2528).sqrt();
        let noise_metadata_schedule_223_0_e2532: f64 = (w[230] + 1.0);
        let noise_metadata_schedule_223_0_e2535: f64 = (w[230] + 1.0);
        let noise_metadata_schedule_223_0_e2536: f64 = (noise_metadata_schedule_223_0_e2532 * noise_metadata_schedule_223_0_e2535);
        let noise_metadata_schedule_223_0_e2539: f64 = (4.0 * params[46]);
        let noise_metadata_schedule_223_0_e2541: f64 = (noise_metadata_schedule_223_0_e2539 * params[46]);
        let noise_metadata_schedule_223_0_e2542: f64 = (noise_metadata_schedule_223_0_e2536 + noise_metadata_schedule_223_0_e2541);
        let noise_metadata_schedule_223_0_e2543: f64 = (noise_metadata_schedule_223_0_e2542).sqrt();
        let noise_metadata_schedule_223_0_e2544: f64 = (noise_metadata_schedule_223_0_e2529 + noise_metadata_schedule_223_0_e2543);
        let noise_metadata_schedule_223_0_e2545: f64 = (noise_metadata_schedule_223_0_e2515 / noise_metadata_schedule_223_0_e2544);
        (noise_metadata_schedule_223_0_e2545,)
    } else {
        (w[231],)
    }
};
            w[231] = noise_metadata_schedule_223_0_e2547;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_224_0_e2564,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_224_0_e2556: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_224_0_e2557: f64 = (w[231] * noise_metadata_schedule_224_0_e2556);
        let noise_metadata_schedule_224_0_e2559: f64 = (noise_metadata_schedule_224_0_e2557 - params[45]);
        let noise_metadata_schedule_224_0_e2561: f64 = (noise_metadata_schedule_224_0_e2559 - w[225]);
        let noise_metadata_schedule_224_0_e2562: f64 = (0.5 * noise_metadata_schedule_224_0_e2561);
        (noise_metadata_schedule_224_0_e2562,)
    } else {
        (w[232],)
    }
};
            w[232] = noise_metadata_schedule_224_0_e2564;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_225_0_e2587,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_225_0_e2574: f64 = (w[232] / w[21]);
        let noise_metadata_schedule_225_0_e2575: f64 = (1.0 - noise_metadata_schedule_225_0_e2574);
        let noise_metadata_schedule_225_0_e2578: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_225_0_e2579: f64 = (noise_metadata_schedule_225_0_e2575).powf(noise_metadata_schedule_225_0_e2578);
        let noise_metadata_schedule_225_0_e2580: f64 = (1.0 - noise_metadata_schedule_225_0_e2579);
        let noise_metadata_schedule_225_0_e2581: f64 = (w[21] * noise_metadata_schedule_225_0_e2580);
        let noise_metadata_schedule_225_0_e2584: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_225_0_e2585: f64 = (noise_metadata_schedule_225_0_e2581 / noise_metadata_schedule_225_0_e2584);
        (noise_metadata_schedule_225_0_e2585,)
    } else {
        (w[233],)
    }
};
            w[233] = noise_metadata_schedule_225_0_e2587;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_226_0_e2604,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_226_0_e2594: f64 = (2.0 * w[144]);
        let noise_metadata_schedule_226_0_e2596: f64 = (noise_metadata_schedule_226_0_e2594 + params[45]);
        let noise_metadata_schedule_226_0_e2598: f64 = (noise_metadata_schedule_226_0_e2596 + w[225]);
        let noise_metadata_schedule_226_0_e2601: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_226_0_e2602: f64 = (noise_metadata_schedule_226_0_e2598 / noise_metadata_schedule_226_0_e2601);
        (noise_metadata_schedule_226_0_e2602,)
    } else {
        (w[234],)
    }
};
            w[234] = noise_metadata_schedule_226_0_e2604;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_227_0_e2643,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_227_0_e2611: f64 = (2.0 * w[234]);
        let noise_metadata_schedule_227_0_e2614: f64 = (w[234] - 1.0);
        let noise_metadata_schedule_227_0_e2617: f64 = (w[234] - 1.0);
        let noise_metadata_schedule_227_0_e2618: f64 = (noise_metadata_schedule_227_0_e2614 * noise_metadata_schedule_227_0_e2617);
        let noise_metadata_schedule_227_0_e2621: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_227_0_e2623: f64 = (noise_metadata_schedule_227_0_e2621 * params[44]);
        let noise_metadata_schedule_227_0_e2624: f64 = (noise_metadata_schedule_227_0_e2618 + noise_metadata_schedule_227_0_e2623);
        let noise_metadata_schedule_227_0_e2625: f64 = (noise_metadata_schedule_227_0_e2624).sqrt();
        let noise_metadata_schedule_227_0_e2628: f64 = (w[234] + 1.0);
        let noise_metadata_schedule_227_0_e2631: f64 = (w[234] + 1.0);
        let noise_metadata_schedule_227_0_e2632: f64 = (noise_metadata_schedule_227_0_e2628 * noise_metadata_schedule_227_0_e2631);
        let noise_metadata_schedule_227_0_e2635: f64 = (4.0 * params[46]);
        let noise_metadata_schedule_227_0_e2637: f64 = (noise_metadata_schedule_227_0_e2635 * params[46]);
        let noise_metadata_schedule_227_0_e2638: f64 = (noise_metadata_schedule_227_0_e2632 + noise_metadata_schedule_227_0_e2637);
        let noise_metadata_schedule_227_0_e2639: f64 = (noise_metadata_schedule_227_0_e2638).sqrt();
        let noise_metadata_schedule_227_0_e2640: f64 = (noise_metadata_schedule_227_0_e2625 + noise_metadata_schedule_227_0_e2639);
        let noise_metadata_schedule_227_0_e2641: f64 = (noise_metadata_schedule_227_0_e2611 / noise_metadata_schedule_227_0_e2640);
        (noise_metadata_schedule_227_0_e2641,)
    } else {
        (w[235],)
    }
};
            w[235] = noise_metadata_schedule_227_0_e2643;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_228_0_e2660,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_228_0_e2652: f64 = (params[45] - w[225]);
        let noise_metadata_schedule_228_0_e2653: f64 = (w[235] * noise_metadata_schedule_228_0_e2652);
        let noise_metadata_schedule_228_0_e2655: f64 = (noise_metadata_schedule_228_0_e2653 - params[45]);
        let noise_metadata_schedule_228_0_e2657: f64 = (noise_metadata_schedule_228_0_e2655 - w[225]);
        let noise_metadata_schedule_228_0_e2658: f64 = (0.5 * noise_metadata_schedule_228_0_e2657);
        (noise_metadata_schedule_228_0_e2658,)
    } else {
        (w[236],)
    }
};
            w[236] = noise_metadata_schedule_228_0_e2660;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_229_0_e2683,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_229_0_e2670: f64 = (w[236] / w[21]);
        let noise_metadata_schedule_229_0_e2671: f64 = (1.0 - noise_metadata_schedule_229_0_e2670);
        let noise_metadata_schedule_229_0_e2674: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_229_0_e2675: f64 = (noise_metadata_schedule_229_0_e2671).powf(noise_metadata_schedule_229_0_e2674);
        let noise_metadata_schedule_229_0_e2676: f64 = (1.0 - noise_metadata_schedule_229_0_e2675);
        let noise_metadata_schedule_229_0_e2677: f64 = (w[21] * noise_metadata_schedule_229_0_e2676);
        let noise_metadata_schedule_229_0_e2680: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_229_0_e2681: f64 = (noise_metadata_schedule_229_0_e2677 / noise_metadata_schedule_229_0_e2680);
        (noise_metadata_schedule_229_0_e2681,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_229_0_e2683;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_230_0_e2694,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_230_0_e2691: f64 = (w[235] + 1.0);
        let noise_metadata_schedule_230_0_e2692: f64 = (0.5 * noise_metadata_schedule_230_0_e2691);
        (noise_metadata_schedule_230_0_e2692,)
    } else {
        (w[237],)
    }
};
            w[237] = noise_metadata_schedule_230_0_e2694;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_231_0_e2708,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_231_0_e2702: f64 = (params[45] / w[21]);
        let noise_metadata_schedule_231_0_e2703: f64 = (1.0 + noise_metadata_schedule_231_0_e2702);
        let noise_metadata_schedule_231_0_e2705: f64 = (-params[43]);
        let noise_metadata_schedule_231_0_e2706: f64 = (noise_metadata_schedule_231_0_e2703).powf(noise_metadata_schedule_231_0_e2705);
        (noise_metadata_schedule_231_0_e2706,)
    } else {
        (w[238],)
    }
};
            w[238] = noise_metadata_schedule_231_0_e2708;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_232_0_e2722,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_232_0_e2716: f64 = (w[225] / w[21]);
        let noise_metadata_schedule_232_0_e2717: f64 = (1.0 + noise_metadata_schedule_232_0_e2716);
        let noise_metadata_schedule_232_0_e2719: f64 = (-params[43]);
        let noise_metadata_schedule_232_0_e2720: f64 = (noise_metadata_schedule_232_0_e2717).powf(noise_metadata_schedule_232_0_e2719);
        (noise_metadata_schedule_232_0_e2720,)
    } else {
        (w[239],)
    }
};
            w[239] = noise_metadata_schedule_232_0_e2722;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_233_0_e2737,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_233_0_e2729: f64 = (1.0 - w[237]);
        let noise_metadata_schedule_233_0_e2731: f64 = (noise_metadata_schedule_233_0_e2729 * w[238]);
        let noise_metadata_schedule_233_0_e2734: f64 = (w[237] * w[239]);
        let noise_metadata_schedule_233_0_e2735: f64 = (noise_metadata_schedule_233_0_e2731 + noise_metadata_schedule_233_0_e2734);
        (noise_metadata_schedule_233_0_e2735,)
    } else {
        (w[240],)
    }
};
            w[240] = noise_metadata_schedule_233_0_e2737;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_234_0_e2750,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_234_0_e2744: f64 = (w[144] - w[236]);
        let noise_metadata_schedule_234_0_e2746: f64 = (noise_metadata_schedule_234_0_e2744 + w[232]);
        let noise_metadata_schedule_234_0_e2748: f64 = (noise_metadata_schedule_234_0_e2746 * w[240]);
        (noise_metadata_schedule_234_0_e2748,)
    } else {
        (w[241],)
    }
};
            w[241] = noise_metadata_schedule_234_0_e2750;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_235_0_e2761,) = {
    if ((w[246] == 0.0) && (w[249] != 0.0)) {
        let noise_metadata_schedule_235_0_e2757: f64 = (w[241] + w[228]);
        let noise_metadata_schedule_235_0_e2759: f64 = (noise_metadata_schedule_235_0_e2757 - w[233]);
        (noise_metadata_schedule_235_0_e2759,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_235_0_e2761;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_236_0_e2778,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_236_0_e2769: f64 = (w[225] * w[225]);
        let noise_metadata_schedule_236_0_e2772: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_236_0_e2774: f64 = (noise_metadata_schedule_236_0_e2772 * params[44]);
        let noise_metadata_schedule_236_0_e2775: f64 = (noise_metadata_schedule_236_0_e2769 + noise_metadata_schedule_236_0_e2774);
        let noise_metadata_schedule_236_0_e2776: f64 = (noise_metadata_schedule_236_0_e2775).sqrt();
        (noise_metadata_schedule_236_0_e2776,)
    } else {
        (w[242],)
    }
};
            w[242] = noise_metadata_schedule_236_0_e2778;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_237_0_e2791,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_237_0_e2785: f64 = (-0.5);
        let noise_metadata_schedule_237_0_e2788: f64 = (w[225] + w[242]);
        let noise_metadata_schedule_237_0_e2789: f64 = (noise_metadata_schedule_237_0_e2785 * noise_metadata_schedule_237_0_e2788);
        (noise_metadata_schedule_237_0_e2789,)
    } else {
        (w[232],)
    }
};
            w[232] = noise_metadata_schedule_237_0_e2791;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_238_0_e2814,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_238_0_e2798: f64 = (-w[21]);
        let noise_metadata_schedule_238_0_e2802: f64 = (w[232] / w[21]);
        let noise_metadata_schedule_238_0_e2803: f64 = (1.0 - noise_metadata_schedule_238_0_e2802);
        let noise_metadata_schedule_238_0_e2806: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_238_0_e2807: f64 = (noise_metadata_schedule_238_0_e2803).powf(noise_metadata_schedule_238_0_e2806);
        let noise_metadata_schedule_238_0_e2808: f64 = (noise_metadata_schedule_238_0_e2798 * noise_metadata_schedule_238_0_e2807);
        let noise_metadata_schedule_238_0_e2811: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_238_0_e2812: f64 = (noise_metadata_schedule_238_0_e2808 / noise_metadata_schedule_238_0_e2811);
        (noise_metadata_schedule_238_0_e2812,)
    } else {
        (w[243],)
    }
};
            w[243] = noise_metadata_schedule_238_0_e2814;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_239_0_e2824,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_239_0_e2822: f64 = (w[144] + w[225]);
        (noise_metadata_schedule_239_0_e2822,)
    } else {
        (w[244],)
    }
};
            w[244] = noise_metadata_schedule_239_0_e2824;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_240_0_e2841,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_240_0_e2832: f64 = (w[244] * w[244]);
        let noise_metadata_schedule_240_0_e2835: f64 = (4.0 * params[44]);
        let noise_metadata_schedule_240_0_e2837: f64 = (noise_metadata_schedule_240_0_e2835 * params[44]);
        let noise_metadata_schedule_240_0_e2838: f64 = (noise_metadata_schedule_240_0_e2832 + noise_metadata_schedule_240_0_e2837);
        let noise_metadata_schedule_240_0_e2839: f64 = (noise_metadata_schedule_240_0_e2838).sqrt();
        (noise_metadata_schedule_240_0_e2839,)
    } else {
        (w[245],)
    }
};
            w[245] = noise_metadata_schedule_240_0_e2841;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_241_0_e2855,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_241_0_e2850: f64 = (w[244] - w[245]);
        let noise_metadata_schedule_241_0_e2851: f64 = (0.5 * noise_metadata_schedule_241_0_e2850);
        let noise_metadata_schedule_241_0_e2853: f64 = (noise_metadata_schedule_241_0_e2851 - w[225]);
        (noise_metadata_schedule_241_0_e2853,)
    } else {
        (w[236],)
    }
};
            w[236] = noise_metadata_schedule_241_0_e2855;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_242_0_e2878,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_242_0_e2862: f64 = (-w[21]);
        let noise_metadata_schedule_242_0_e2866: f64 = (w[236] / w[21]);
        let noise_metadata_schedule_242_0_e2867: f64 = (1.0 - noise_metadata_schedule_242_0_e2866);
        let noise_metadata_schedule_242_0_e2870: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_242_0_e2871: f64 = (noise_metadata_schedule_242_0_e2867).powf(noise_metadata_schedule_242_0_e2870);
        let noise_metadata_schedule_242_0_e2872: f64 = (noise_metadata_schedule_242_0_e2862 * noise_metadata_schedule_242_0_e2871);
        let noise_metadata_schedule_242_0_e2875: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_242_0_e2876: f64 = (noise_metadata_schedule_242_0_e2872 / noise_metadata_schedule_242_0_e2875);
        (noise_metadata_schedule_242_0_e2876,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_242_0_e2878;
        }
        if (active[0] & 0x3413) != 0 {
            let (noise_metadata_schedule_243_0_e2901,) = {
    if ((w[246] == 0.0) && (w[249] == 0.0)) {
        let noise_metadata_schedule_243_0_e2887: f64 = (1.0 - params[34]);
        let noise_metadata_schedule_243_0_e2889: f64 = (-params[43]);
        let noise_metadata_schedule_243_0_e2890: f64 = (noise_metadata_schedule_243_0_e2887).powf(noise_metadata_schedule_243_0_e2889);
        let noise_metadata_schedule_243_0_e2893: f64 = (w[144] - w[236]);
        let noise_metadata_schedule_243_0_e2895: f64 = (noise_metadata_schedule_243_0_e2893 + w[232]);
        let noise_metadata_schedule_243_0_e2896: f64 = (noise_metadata_schedule_243_0_e2890 * noise_metadata_schedule_243_0_e2895);
        let noise_metadata_schedule_243_0_e2897: f64 = (w[228] + noise_metadata_schedule_243_0_e2896);
        let noise_metadata_schedule_243_0_e2899: f64 = (noise_metadata_schedule_243_0_e2897 - w[243]);
        (noise_metadata_schedule_243_0_e2899,)
    } else {
        (w[116],)
    }
};
            w[116] = noise_metadata_schedule_243_0_e2901;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_244_0_e2905: f64 = (w[27] * w[73]);
            let noise_metadata_schedule_244_0_e2906: f64 = (1.0 / noise_metadata_schedule_244_0_e2905);
            w[112] = noise_metadata_schedule_244_0_e2906;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_245_0_e2909: f64 = if w[143] < w[61] { 1.0 } else { 0.0 };
            w[250] = noise_metadata_schedule_245_0_e2909;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_246_0_e2916,) = {
    if (w[250] != 0.0) {
        let noise_metadata_schedule_246_0_e2913: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_246_0_e2914: f64 = (noise_metadata_schedule_246_0_e2913).exp();
        (noise_metadata_schedule_246_0_e2914,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_246_0_e2916;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_247_0_e2932,) = {
    if (w[250] == 0.0) {
        let noise_metadata_schedule_247_0_e2921: f64 = (w[61] * w[112]);
        let noise_metadata_schedule_247_0_e2922: f64 = (noise_metadata_schedule_247_0_e2921).exp();
        let noise_metadata_schedule_247_0_e2926: f64 = (w[143] - w[61]);
        let noise_metadata_schedule_247_0_e2928: f64 = (noise_metadata_schedule_247_0_e2926 * w[112]);
        let noise_metadata_schedule_247_0_e2929: f64 = (1.0 + noise_metadata_schedule_247_0_e2928);
        let noise_metadata_schedule_247_0_e2930: f64 = (noise_metadata_schedule_247_0_e2922 * noise_metadata_schedule_247_0_e2929);
        (noise_metadata_schedule_247_0_e2930,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_247_0_e2932;
        }
        if (active[0] & 0x3410) != 0 {
            let noise_metadata_schedule_248_0_e2936: f64 = (w[109] - 1.0);
            let noise_metadata_schedule_248_0_e2937: f64 = (w[0] * noise_metadata_schedule_248_0_e2936);
            w[74] = noise_metadata_schedule_248_0_e2937;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_249_0_e2941: f64 = (w[28] * w[73]);
            let noise_metadata_schedule_249_0_e2942: f64 = (1.0 / noise_metadata_schedule_249_0_e2941);
            w[112] = noise_metadata_schedule_249_0_e2942;
        }
        if (active[0] & 0x357f) != 0 {
            let noise_metadata_schedule_250_0_e2945: f64 = if w[144] < w[62] { 1.0 } else { 0.0 };
            w[251] = noise_metadata_schedule_250_0_e2945;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_251_0_e2952,) = {
    if (w[251] != 0.0) {
        let noise_metadata_schedule_251_0_e2949: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_251_0_e2950: f64 = (noise_metadata_schedule_251_0_e2949).exp();
        (noise_metadata_schedule_251_0_e2950,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_251_0_e2952;
        }
        if (active[0] & 0x357f) != 0 {
            let (noise_metadata_schedule_252_0_e2968,) = {
    if (w[251] == 0.0) {
        let noise_metadata_schedule_252_0_e2957: f64 = (w[62] * w[112]);
        let noise_metadata_schedule_252_0_e2958: f64 = (noise_metadata_schedule_252_0_e2957).exp();
        let noise_metadata_schedule_252_0_e2962: f64 = (w[144] - w[62]);
        let noise_metadata_schedule_252_0_e2964: f64 = (noise_metadata_schedule_252_0_e2962 * w[112]);
        let noise_metadata_schedule_252_0_e2965: f64 = (1.0 + noise_metadata_schedule_252_0_e2964);
        let noise_metadata_schedule_252_0_e2966: f64 = (noise_metadata_schedule_252_0_e2958 * noise_metadata_schedule_252_0_e2965);
        (noise_metadata_schedule_252_0_e2966,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_252_0_e2968;
        }
        if (active[0] & 0x3410) != 0 {
            let noise_metadata_schedule_253_0_e2971: f64 = (w[0] * w[1]);
            let noise_metadata_schedule_253_0_e2974: f64 = (w[109] - 1.0);
            let noise_metadata_schedule_253_0_e2975: f64 = (noise_metadata_schedule_253_0_e2971 * noise_metadata_schedule_253_0_e2974);
            w[75] = noise_metadata_schedule_253_0_e2975;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_254_0_e2979: f64 = (w[114] * w[44]);
            let noise_metadata_schedule_254_0_e2980: f64 = (1.0 + noise_metadata_schedule_254_0_e2979);
            let noise_metadata_schedule_254_0_e2983: f64 = (w[116] * w[43]);
            let noise_metadata_schedule_254_0_e2984: f64 = (noise_metadata_schedule_254_0_e2980 + noise_metadata_schedule_254_0_e2983);
            let noise_metadata_schedule_254_0_e2986: f64 = (noise_metadata_schedule_254_0_e2984 - 0.0001);
            w[78] = noise_metadata_schedule_254_0_e2986;
        }
        if (active[0] & 0x3413) != 0 {
            let noise_metadata_schedule_255_0_e2990: f64 = (w[78] * w[78]);
            let noise_metadata_schedule_255_0_e2992: f64 = (noise_metadata_schedule_255_0_e2990 + 1e-8);
            let noise_metadata_schedule_255_0_e2993: f64 = (noise_metadata_schedule_255_0_e2992).sqrt();
            let noise_metadata_schedule_255_0_e2995: f64 = (noise_metadata_schedule_255_0_e2993 + w[78]);
            let noise_metadata_schedule_255_0_e2996: f64 = (0.5 * noise_metadata_schedule_255_0_e2995);
            let noise_metadata_schedule_255_0_e2998: f64 = (noise_metadata_schedule_255_0_e2996 + 0.0001);
            w[79] = noise_metadata_schedule_255_0_e2998;
        }
        if (active[0] & 0x3410) != 0 {
            let noise_metadata_schedule_256_0_e3001: f64 = (w[74] * w[45]);
            let noise_metadata_schedule_256_0_e3004: f64 = (w[75] * w[46]);
            let noise_metadata_schedule_256_0_e3005: f64 = (noise_metadata_schedule_256_0_e3001 + noise_metadata_schedule_256_0_e3004);
            w[80] = noise_metadata_schedule_256_0_e3005;
        }
        if (active[0] & 0x3410) != 0 {
            let noise_metadata_schedule_257_0_e3008: f64 = if params[30] < 0.5 { 1.0 } else { 0.0 };
            w[252] = noise_metadata_schedule_257_0_e3008;
        }
        if (active[0] & 0x3410) != 0 {
            let (noise_metadata_schedule_258_0_e3020,) = {
    if (w[252] != 0.0) {
        let noise_metadata_schedule_258_0_e3013: f64 = (1.0 / params[73]);
        let noise_metadata_schedule_258_0_e3014: f64 = (w[79]).powf(noise_metadata_schedule_258_0_e3013);
        let noise_metadata_schedule_258_0_e3017: f64 = (4.0 * w[80]);
        let noise_metadata_schedule_258_0_e3018: f64 = (noise_metadata_schedule_258_0_e3014 + noise_metadata_schedule_258_0_e3017);
        (noise_metadata_schedule_258_0_e3018,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_258_0_e3020;
        }
        if (active[0] & 0x410) != 0 {
            let noise_metadata_schedule_259_0_e3023: f64 = if w[108] > 1e-8 { 1.0 } else { 0.0 };
            w[253] = noise_metadata_schedule_259_0_e3023;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_260_0_e3035,) = {
    if ((w[252] != 0.0) && (w[253] != 0.0)) {
        let noise_metadata_schedule_260_0_e3031: f64 = (w[108]).powf(params[73]);
        let noise_metadata_schedule_260_0_e3032: f64 = (w[79] + noise_metadata_schedule_260_0_e3031);
        let noise_metadata_schedule_260_0_e3033: f64 = (0.5 * noise_metadata_schedule_260_0_e3032);
        (noise_metadata_schedule_260_0_e3033,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_260_0_e3035;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_261_0_e3048,) = {
    if ((w[252] != 0.0) && (w[253] == 0.0)) {
        let noise_metadata_schedule_261_0_e3044: f64 = (1e-8_f64).powf(params[73]);
        let noise_metadata_schedule_261_0_e3045: f64 = (w[79] + noise_metadata_schedule_261_0_e3044);
        let noise_metadata_schedule_261_0_e3046: f64 = (0.5 * noise_metadata_schedule_261_0_e3045);
        (noise_metadata_schedule_261_0_e3046,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_261_0_e3048;
        }
        if (active[0] & 0x3410) != 0 {
            let (noise_metadata_schedule_262_0_e3057,) = {
    if (w[252] == 0.0) {
        let noise_metadata_schedule_262_0_e3054: f64 = (4.0 * w[80]);
        let noise_metadata_schedule_262_0_e3055: f64 = (1.0 + noise_metadata_schedule_262_0_e3054);
        (noise_metadata_schedule_262_0_e3055,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_262_0_e3057;
        }
        if (active[0] & 0x410) != 0 {
            let noise_metadata_schedule_263_0_e3060: f64 = if w[108] > 1e-8 { 1.0 } else { 0.0 };
            w[254] = noise_metadata_schedule_263_0_e3060;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_264_0_e3075,) = {
    if ((w[252] == 0.0) && (w[254] != 0.0)) {
        let noise_metadata_schedule_264_0_e3067: f64 = (0.5 * w[79]);
        let noise_metadata_schedule_264_0_e3071: f64 = (w[108]).powf(params[73]);
        let noise_metadata_schedule_264_0_e3072: f64 = (1.0 + noise_metadata_schedule_264_0_e3071);
        let noise_metadata_schedule_264_0_e3073: f64 = (noise_metadata_schedule_264_0_e3067 * noise_metadata_schedule_264_0_e3072);
        (noise_metadata_schedule_264_0_e3073,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_264_0_e3075;
        }
        if (active[0] & 0x410) != 0 {
            let (noise_metadata_schedule_265_0_e3091,) = {
    if ((w[252] == 0.0) && (w[254] == 0.0)) {
        let noise_metadata_schedule_265_0_e3083: f64 = (0.5 * w[79]);
        let noise_metadata_schedule_265_0_e3087: f64 = (1e-8_f64).powf(params[73]);
        let noise_metadata_schedule_265_0_e3088: f64 = (1.0 + noise_metadata_schedule_265_0_e3087);
        let noise_metadata_schedule_265_0_e3089: f64 = (noise_metadata_schedule_265_0_e3083 * noise_metadata_schedule_265_0_e3088);
        (noise_metadata_schedule_265_0_e3089,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_265_0_e3091;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_267_0_e3097: f64 = (w[74] / w[81]);
            w[76] = noise_metadata_schedule_267_0_e3097;
        }
        if (active[0] & 0x316f) != 0 {
            let noise_metadata_schedule_269_0_e3101: f64 = if params[31] > 0.0 { 1.0 } else { 0.0 };
            w[255] = noise_metadata_schedule_269_0_e3101;
        }
        if (active[0] & 0x316f) != 0 {
            let (noise_metadata_schedule_270_0_e3109,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_270_0_e3106: f64 = (params[33] * w[73]);
        let noise_metadata_schedule_270_0_e3107: f64 = (1.0 / noise_metadata_schedule_270_0_e3106);
        (noise_metadata_schedule_270_0_e3107,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_270_0_e3109;
        }
        if (active[0] & 0x316f) != 0 {
            let noise_metadata_schedule_271_0_e3112: f64 = if w[146] < w[63] { 1.0 } else { 0.0 };
            w[256] = noise_metadata_schedule_271_0_e3112;
        }
        if (active[0] & 0x316f) != 0 {
            let (noise_metadata_schedule_272_0_e3121,) = {
    if ((w[255] != 0.0) && (w[256] != 0.0)) {
        let noise_metadata_schedule_272_0_e3118: f64 = (w[146] * w[112]);
        let noise_metadata_schedule_272_0_e3119: f64 = (noise_metadata_schedule_272_0_e3118).exp();
        (noise_metadata_schedule_272_0_e3119,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_272_0_e3121;
        }
        if (active[0] & 0x316f) != 0 {
            let (noise_metadata_schedule_273_0_e3139,) = {
    if ((w[255] != 0.0) && (w[256] == 0.0)) {
        let noise_metadata_schedule_273_0_e3128: f64 = (w[63] * w[112]);
        let noise_metadata_schedule_273_0_e3129: f64 = (noise_metadata_schedule_273_0_e3128).exp();
        let noise_metadata_schedule_273_0_e3133: f64 = (w[146] - w[63]);
        let noise_metadata_schedule_273_0_e3135: f64 = (noise_metadata_schedule_273_0_e3133 * w[112]);
        let noise_metadata_schedule_273_0_e3136: f64 = (1.0 + noise_metadata_schedule_273_0_e3135);
        let noise_metadata_schedule_273_0_e3137: f64 = (noise_metadata_schedule_273_0_e3129 * noise_metadata_schedule_273_0_e3136);
        (noise_metadata_schedule_273_0_e3137,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_273_0_e3139;
        }
        if (active[0] & 0x310f) != 0 {
            let noise_metadata_schedule_274_0_e3142: f64 = if w[144] < w[63] { 1.0 } else { 0.0 };
            w[257] = noise_metadata_schedule_274_0_e3142;
        }
        if (active[0] & 0x310f) != 0 {
            let (noise_metadata_schedule_275_0_e3151,) = {
    if ((w[255] != 0.0) && (w[257] != 0.0)) {
        let noise_metadata_schedule_275_0_e3148: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_275_0_e3149: f64 = (noise_metadata_schedule_275_0_e3148).exp();
        (noise_metadata_schedule_275_0_e3149,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_275_0_e3151;
        }
        if (active[0] & 0x310f) != 0 {
            let (noise_metadata_schedule_276_0_e3169,) = {
    if ((w[255] != 0.0) && (w[257] == 0.0)) {
        let noise_metadata_schedule_276_0_e3158: f64 = (w[63] * w[112]);
        let noise_metadata_schedule_276_0_e3159: f64 = (noise_metadata_schedule_276_0_e3158).exp();
        let noise_metadata_schedule_276_0_e3163: f64 = (w[144] - w[63]);
        let noise_metadata_schedule_276_0_e3165: f64 = (noise_metadata_schedule_276_0_e3163 * w[112]);
        let noise_metadata_schedule_276_0_e3166: f64 = (1.0 + noise_metadata_schedule_276_0_e3165);
        let noise_metadata_schedule_276_0_e3167: f64 = (noise_metadata_schedule_276_0_e3159 * noise_metadata_schedule_276_0_e3166);
        (noise_metadata_schedule_276_0_e3167,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_276_0_e3169;
        }
        if (active[0] & 0x3000) != 0 {
            let (noise_metadata_schedule_277_0_e3185,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_277_0_e3174: f64 = (params[32] * w[109]);
        let noise_metadata_schedule_277_0_e3177: f64 = (1.0 - params[32]);
        let noise_metadata_schedule_277_0_e3179: f64 = (noise_metadata_schedule_277_0_e3177 * w[111]);
        let noise_metadata_schedule_277_0_e3180: f64 = (noise_metadata_schedule_277_0_e3174 + noise_metadata_schedule_277_0_e3179);
        let noise_metadata_schedule_277_0_e3182: f64 = (noise_metadata_schedule_277_0_e3180 - 1.0);
        let noise_metadata_schedule_277_0_e3183: f64 = (w[5] * noise_metadata_schedule_277_0_e3182);
        (noise_metadata_schedule_277_0_e3183,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_277_0_e3185;
        }
        if (active[0] & 0x3000) != 0 {
            let (noise_metadata_schedule_278_0_e3191,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_278_0_e3189: f64 = (w[82] * w[47]);
        (noise_metadata_schedule_278_0_e3189,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_278_0_e3191;
        }
        if (active[0] & 0x3000) != 0 {
            let (noise_metadata_schedule_279_0_e3199,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_279_0_e3196: f64 = (4.0 * w[85]);
        let noise_metadata_schedule_279_0_e3197: f64 = (1.0 + noise_metadata_schedule_279_0_e3196);
        (noise_metadata_schedule_279_0_e3197,)
    } else {
        (w[108],)
    }
};
            w[108] = noise_metadata_schedule_279_0_e3199;
        }
        if (active[0] & 0x3000) != 0 {
            let noise_metadata_schedule_280_0_e3202: f64 = if w[108] > 1e-8 { 1.0 } else { 0.0 };
            w[258] = noise_metadata_schedule_280_0_e3202;
        }
        if (active[0] & 0x3000) != 0 {
            let (noise_metadata_schedule_281_0_e3213,) = {
    if ((w[255] != 0.0) && (w[258] != 0.0)) {
        let noise_metadata_schedule_281_0_e3209: f64 = (w[108]).sqrt();
        let noise_metadata_schedule_281_0_e3210: f64 = (1.0 + noise_metadata_schedule_281_0_e3209);
        let noise_metadata_schedule_281_0_e3211: f64 = (0.5 * noise_metadata_schedule_281_0_e3210);
        (noise_metadata_schedule_281_0_e3211,)
    } else {
        (w[86],)
    }
};
            w[86] = noise_metadata_schedule_281_0_e3213;
        }
        if (active[0] & 0x3000) != 0 {
            let (noise_metadata_schedule_282_0_e3225,) = {
    if ((w[255] != 0.0) && (w[258] == 0.0)) {
        let noise_metadata_schedule_282_0_e3221: f64 = (1e-8_f64).sqrt();
        let noise_metadata_schedule_282_0_e3222: f64 = (1.0 + noise_metadata_schedule_282_0_e3221);
        let noise_metadata_schedule_282_0_e3223: f64 = (0.5 * noise_metadata_schedule_282_0_e3222);
        (noise_metadata_schedule_282_0_e3223,)
    } else {
        (w[86],)
    }
};
            w[86] = noise_metadata_schedule_282_0_e3225;
        }
        if (active[0] & 0x216f) != 0 {
            let noise_metadata_schedule_283_0_e3228: f64 = if w[147] < w[63] { 1.0 } else { 0.0 };
            w[259] = noise_metadata_schedule_283_0_e3228;
        }
        if (active[0] & 0x216f) != 0 {
            let (noise_metadata_schedule_284_0_e3237,) = {
    if ((w[255] != 0.0) && (w[259] != 0.0)) {
        let noise_metadata_schedule_284_0_e3234: f64 = (w[147] * w[112]);
        let noise_metadata_schedule_284_0_e3235: f64 = (noise_metadata_schedule_284_0_e3234).exp();
        (noise_metadata_schedule_284_0_e3235,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_284_0_e3237;
        }
        if (active[0] & 0x216f) != 0 {
            let (noise_metadata_schedule_285_0_e3255,) = {
    if ((w[255] != 0.0) && (w[259] == 0.0)) {
        let noise_metadata_schedule_285_0_e3244: f64 = (w[63] * w[112]);
        let noise_metadata_schedule_285_0_e3245: f64 = (noise_metadata_schedule_285_0_e3244).exp();
        let noise_metadata_schedule_285_0_e3249: f64 = (w[147] - w[63]);
        let noise_metadata_schedule_285_0_e3251: f64 = (noise_metadata_schedule_285_0_e3249 * w[112]);
        let noise_metadata_schedule_285_0_e3252: f64 = (1.0 + noise_metadata_schedule_285_0_e3251);
        let noise_metadata_schedule_285_0_e3253: f64 = (noise_metadata_schedule_285_0_e3245 * noise_metadata_schedule_285_0_e3252);
        (noise_metadata_schedule_285_0_e3253,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_285_0_e3255;
        }
        if (active[0] & 0x2000) != 0 {
            let (noise_metadata_schedule_286_0_e3263,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_286_0_e3260: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_286_0_e3261: f64 = (w[5] * noise_metadata_schedule_286_0_e3260);
        (noise_metadata_schedule_286_0_e3261,)
    } else {
        (w[83],)
    }
};
            w[83] = noise_metadata_schedule_286_0_e3263;
        }
        if (active[0] & 0x2000) != 0 {
            let (noise_metadata_schedule_287_0_e3271,) = {
    if (w[255] != 0.0) {
        let noise_metadata_schedule_287_0_e3267: f64 = (w[82] - w[83]);
        let noise_metadata_schedule_287_0_e3269: f64 = (noise_metadata_schedule_287_0_e3267 / w[86]);
        (noise_metadata_schedule_287_0_e3269,)
    } else {
        (w[84],)
    }
};
            w[84] = noise_metadata_schedule_287_0_e3271;
        }
        if (active[0] & 0x1000) != 0 {
            let (noise_metadata_schedule_289_0_e3281,) = {
    if (w[255] == 0.0) {
        (1.0,)
    } else {
        (w[86],)
    }
};
            w[86] = noise_metadata_schedule_289_0_e3281;
        }
        if (active[0] & 0x2000) != 0 {
            let (noise_metadata_schedule_290_0_e3286,) = {
    if (w[255] == 0.0) {
        (0.0,)
    } else {
        (w[84],)
    }
};
            w[84] = noise_metadata_schedule_290_0_e3286;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_291_0_e3289: f64 = if params[55] == 1.0 { 1.0 } else { 0.0 };
            w[260] = noise_metadata_schedule_291_0_e3289;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_292_0_e3297,) = {
    if (w[260] != 0.0) {
        let noise_metadata_schedule_292_0_e3294: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_292_0_e3295: f64 = (1.0 / noise_metadata_schedule_292_0_e3294);
        (noise_metadata_schedule_292_0_e3295,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_292_0_e3297;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_293_0_e3300: f64 = if w[143] < w[65] { 1.0 } else { 0.0 };
            w[261] = noise_metadata_schedule_293_0_e3300;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_294_0_e3309,) = {
    if ((w[260] != 0.0) && (w[261] != 0.0)) {
        let noise_metadata_schedule_294_0_e3306: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_294_0_e3307: f64 = (noise_metadata_schedule_294_0_e3306).exp();
        (noise_metadata_schedule_294_0_e3307,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_294_0_e3309;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_295_0_e3327,) = {
    if ((w[260] != 0.0) && (w[261] == 0.0)) {
        let noise_metadata_schedule_295_0_e3316: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_295_0_e3317: f64 = (noise_metadata_schedule_295_0_e3316).exp();
        let noise_metadata_schedule_295_0_e3321: f64 = (w[143] - w[65]);
        let noise_metadata_schedule_295_0_e3323: f64 = (noise_metadata_schedule_295_0_e3321 * w[112]);
        let noise_metadata_schedule_295_0_e3324: f64 = (1.0 + noise_metadata_schedule_295_0_e3323);
        let noise_metadata_schedule_295_0_e3325: f64 = (noise_metadata_schedule_295_0_e3317 * noise_metadata_schedule_295_0_e3324);
        (noise_metadata_schedule_295_0_e3325,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_295_0_e3327;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_296_0_e3335,) = {
    if (w[260] != 0.0) {
        let noise_metadata_schedule_296_0_e3332: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_296_0_e3333: f64 = (1.0 / noise_metadata_schedule_296_0_e3332);
        (noise_metadata_schedule_296_0_e3333,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_296_0_e3335;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_297_0_e3338: f64 = if w[143] < w[66] { 1.0 } else { 0.0 };
            w[262] = noise_metadata_schedule_297_0_e3338;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_298_0_e3347,) = {
    if ((w[260] != 0.0) && (w[262] != 0.0)) {
        let noise_metadata_schedule_298_0_e3344: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_298_0_e3345: f64 = (noise_metadata_schedule_298_0_e3344).exp();
        (noise_metadata_schedule_298_0_e3345,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_298_0_e3347;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_299_0_e3365,) = {
    if ((w[260] != 0.0) && (w[262] == 0.0)) {
        let noise_metadata_schedule_299_0_e3354: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_299_0_e3355: f64 = (noise_metadata_schedule_299_0_e3354).exp();
        let noise_metadata_schedule_299_0_e3359: f64 = (w[143] - w[66]);
        let noise_metadata_schedule_299_0_e3361: f64 = (noise_metadata_schedule_299_0_e3359 * w[112]);
        let noise_metadata_schedule_299_0_e3362: f64 = (1.0 + noise_metadata_schedule_299_0_e3361);
        let noise_metadata_schedule_299_0_e3363: f64 = (noise_metadata_schedule_299_0_e3355 * noise_metadata_schedule_299_0_e3362);
        (noise_metadata_schedule_299_0_e3363,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_299_0_e3365;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_300_0_e3368: f64 = if params[57] > 0.0 { 1.0 } else { 0.0 };
            w[263] = noise_metadata_schedule_300_0_e3368;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_301_0_e3392,) = {
    if ((w[260] != 0.0) && (w[263] != 0.0)) {
        let noise_metadata_schedule_301_0_e3377: f64 = (w[79] - 1.0);
        let noise_metadata_schedule_301_0_e3378: f64 = (params[57] * noise_metadata_schedule_301_0_e3377);
        let noise_metadata_schedule_301_0_e3379: f64 = (1.0 + noise_metadata_schedule_301_0_e3378);
        let noise_metadata_schedule_301_0_e3380: f64 = (w[3] * noise_metadata_schedule_301_0_e3379);
        let noise_metadata_schedule_301_0_e3383: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_301_0_e3384: f64 = (noise_metadata_schedule_301_0_e3380 * noise_metadata_schedule_301_0_e3383);
        let noise_metadata_schedule_301_0_e3388: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_301_0_e3389: f64 = (w[6] * noise_metadata_schedule_301_0_e3388);
        let noise_metadata_schedule_301_0_e3390: f64 = (noise_metadata_schedule_301_0_e3384 + noise_metadata_schedule_301_0_e3389);
        (noise_metadata_schedule_301_0_e3390,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_301_0_e3392;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_302_0_e3409,) = {
    if ((w[260] != 0.0) && (w[263] == 0.0)) {
        let noise_metadata_schedule_302_0_e3400: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_302_0_e3401: f64 = (w[3] * noise_metadata_schedule_302_0_e3400);
        let noise_metadata_schedule_302_0_e3405: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_302_0_e3406: f64 = (w[6] * noise_metadata_schedule_302_0_e3405);
        let noise_metadata_schedule_302_0_e3407: f64 = (noise_metadata_schedule_302_0_e3401 + noise_metadata_schedule_302_0_e3406);
        (noise_metadata_schedule_302_0_e3407,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_302_0_e3409;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_303_0_e3412: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[264] = noise_metadata_schedule_303_0_e3412;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_304_0_e3421,) = {
    if ((w[260] != 0.0) && (w[264] != 0.0)) {
        let noise_metadata_schedule_304_0_e3417: f64 = (-w[31]);
        let noise_metadata_schedule_304_0_e3419: f64 = (noise_metadata_schedule_304_0_e3417 - w[143]);
        (noise_metadata_schedule_304_0_e3419,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_304_0_e3421;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_305_0_e3431,) = {
    if ((w[260] != 0.0) && (w[264] != 0.0)) {
        let noise_metadata_schedule_305_0_e3428: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_305_0_e3429: f64 = (1.0 / noise_metadata_schedule_305_0_e3428);
        (noise_metadata_schedule_305_0_e3429,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_305_0_e3431;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_306_0_e3434: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[265] = noise_metadata_schedule_306_0_e3434;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_307_0_e3445,) = {
    if (((w[260] != 0.0) && (w[264] != 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_307_0_e3442: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_307_0_e3443: f64 = (noise_metadata_schedule_307_0_e3442).exp();
        (noise_metadata_schedule_307_0_e3443,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_307_0_e3445;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_308_0_e3465,) = {
    if (((w[260] != 0.0) && (w[264] != 0.0)) && (w[265] == 0.0)) {
        let noise_metadata_schedule_308_0_e3454: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_308_0_e3455: f64 = (noise_metadata_schedule_308_0_e3454).exp();
        let noise_metadata_schedule_308_0_e3459: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_308_0_e3461: f64 = (noise_metadata_schedule_308_0_e3459 * w[112]);
        let noise_metadata_schedule_308_0_e3462: f64 = (1.0 + noise_metadata_schedule_308_0_e3461);
        let noise_metadata_schedule_308_0_e3463: f64 = (noise_metadata_schedule_308_0_e3455 * noise_metadata_schedule_308_0_e3462);
        (noise_metadata_schedule_308_0_e3463,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_308_0_e3465;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_309_0_e3477,) = {
    if ((w[260] != 0.0) && (w[264] != 0.0)) {
        let noise_metadata_schedule_309_0_e3473: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_309_0_e3474: f64 = (params[90] * noise_metadata_schedule_309_0_e3473);
        let noise_metadata_schedule_309_0_e3475: f64 = (w[87] - noise_metadata_schedule_309_0_e3474);
        (noise_metadata_schedule_309_0_e3475,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_309_0_e3477;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_310_0_e3481,) = {
    if (w[260] != 0.0) {
        (0.0,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_310_0_e3481;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_311_0_e3484: f64 = if params[55] == 0.0 { 1.0 } else { 0.0 };
            w[266] = noise_metadata_schedule_311_0_e3484;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_312_0_e3491,) = {
    if ((w[260] == 0.0) && (w[266] != 0.0)) {
        (0.0,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_312_0_e3491;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_313_0_e3502,) = {
    if ((w[260] == 0.0) && (w[266] != 0.0)) {
        let noise_metadata_schedule_313_0_e3499: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_313_0_e3500: f64 = (1.0 / noise_metadata_schedule_313_0_e3499);
        (noise_metadata_schedule_313_0_e3500,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_313_0_e3502;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_314_0_e3505: f64 = if w[145] < w[65] { 1.0 } else { 0.0 };
            w[267] = noise_metadata_schedule_314_0_e3505;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_315_0_e3517,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_315_0_e3514: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_315_0_e3515: f64 = (noise_metadata_schedule_315_0_e3514).exp();
        (noise_metadata_schedule_315_0_e3515,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_315_0_e3517;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_316_0_e3538,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] == 0.0)) {
        let noise_metadata_schedule_316_0_e3527: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_316_0_e3528: f64 = (noise_metadata_schedule_316_0_e3527).exp();
        let noise_metadata_schedule_316_0_e3532: f64 = (w[145] - w[65]);
        let noise_metadata_schedule_316_0_e3534: f64 = (noise_metadata_schedule_316_0_e3532 * w[112]);
        let noise_metadata_schedule_316_0_e3535: f64 = (1.0 + noise_metadata_schedule_316_0_e3534);
        let noise_metadata_schedule_316_0_e3536: f64 = (noise_metadata_schedule_316_0_e3528 * noise_metadata_schedule_316_0_e3535);
        (noise_metadata_schedule_316_0_e3536,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_316_0_e3538;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_317_0_e3549,) = {
    if ((w[260] == 0.0) && (w[266] != 0.0)) {
        let noise_metadata_schedule_317_0_e3546: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_317_0_e3547: f64 = (1.0 / noise_metadata_schedule_317_0_e3546);
        (noise_metadata_schedule_317_0_e3547,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_317_0_e3549;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_318_0_e3552: f64 = if w[145] < w[66] { 1.0 } else { 0.0 };
            w[268] = noise_metadata_schedule_318_0_e3552;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_319_0_e3564,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[268] != 0.0)) {
        let noise_metadata_schedule_319_0_e3561: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_319_0_e3562: f64 = (noise_metadata_schedule_319_0_e3561).exp();
        (noise_metadata_schedule_319_0_e3562,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_319_0_e3564;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_320_0_e3585,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[268] == 0.0)) {
        let noise_metadata_schedule_320_0_e3574: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_320_0_e3575: f64 = (noise_metadata_schedule_320_0_e3574).exp();
        let noise_metadata_schedule_320_0_e3579: f64 = (w[145] - w[66]);
        let noise_metadata_schedule_320_0_e3581: f64 = (noise_metadata_schedule_320_0_e3579 * w[112]);
        let noise_metadata_schedule_320_0_e3582: f64 = (1.0 + noise_metadata_schedule_320_0_e3581);
        let noise_metadata_schedule_320_0_e3583: f64 = (noise_metadata_schedule_320_0_e3575 * noise_metadata_schedule_320_0_e3582);
        (noise_metadata_schedule_320_0_e3583,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_320_0_e3585;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_321_0_e3602,) = {
    if ((w[260] == 0.0) && (w[266] != 0.0)) {
        let noise_metadata_schedule_321_0_e3593: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_321_0_e3594: f64 = (w[3] * noise_metadata_schedule_321_0_e3593);
        let noise_metadata_schedule_321_0_e3598: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_321_0_e3599: f64 = (w[6] * noise_metadata_schedule_321_0_e3598);
        let noise_metadata_schedule_321_0_e3600: f64 = (noise_metadata_schedule_321_0_e3594 + noise_metadata_schedule_321_0_e3599);
        (noise_metadata_schedule_321_0_e3600,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_321_0_e3602;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_322_0_e3605: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[269] = noise_metadata_schedule_322_0_e3605;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_323_0_e3617,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[269] != 0.0)) {
        let noise_metadata_schedule_323_0_e3613: f64 = (-w[31]);
        let noise_metadata_schedule_323_0_e3615: f64 = (noise_metadata_schedule_323_0_e3613 - w[143]);
        (noise_metadata_schedule_323_0_e3615,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_323_0_e3617;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_324_0_e3630,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[269] != 0.0)) {
        let noise_metadata_schedule_324_0_e3627: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_324_0_e3628: f64 = (1.0 / noise_metadata_schedule_324_0_e3627);
        (noise_metadata_schedule_324_0_e3628,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_324_0_e3630;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_325_0_e3633: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[270] = noise_metadata_schedule_325_0_e3633;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_326_0_e3647,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[269] != 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_326_0_e3644: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_326_0_e3645: f64 = (noise_metadata_schedule_326_0_e3644).exp();
        (noise_metadata_schedule_326_0_e3645,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_326_0_e3647;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_327_0_e3670,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[269] != 0.0)) && (w[270] == 0.0)) {
        let noise_metadata_schedule_327_0_e3659: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_327_0_e3660: f64 = (noise_metadata_schedule_327_0_e3659).exp();
        let noise_metadata_schedule_327_0_e3664: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_327_0_e3666: f64 = (noise_metadata_schedule_327_0_e3664 * w[112]);
        let noise_metadata_schedule_327_0_e3667: f64 = (1.0 + noise_metadata_schedule_327_0_e3666);
        let noise_metadata_schedule_327_0_e3668: f64 = (noise_metadata_schedule_327_0_e3660 * noise_metadata_schedule_327_0_e3667);
        (noise_metadata_schedule_327_0_e3668,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_327_0_e3670;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_328_0_e3685,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[269] != 0.0)) {
        let noise_metadata_schedule_328_0_e3681: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_328_0_e3682: f64 = (params[90] * noise_metadata_schedule_328_0_e3681);
        let noise_metadata_schedule_328_0_e3683: f64 = (w[88] - noise_metadata_schedule_328_0_e3682);
        (noise_metadata_schedule_328_0_e3683,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_328_0_e3685;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_329_0_e3697,) = {
    if ((w[260] == 0.0) && (w[266] == 0.0)) {
        let noise_metadata_schedule_329_0_e3694: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_329_0_e3695: f64 = (1.0 / noise_metadata_schedule_329_0_e3694);
        (noise_metadata_schedule_329_0_e3695,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_329_0_e3697;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_330_0_e3700: f64 = if w[143] < w[65] { 1.0 } else { 0.0 };
            w[271] = noise_metadata_schedule_330_0_e3700;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_331_0_e3713,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[271] != 0.0)) {
        let noise_metadata_schedule_331_0_e3710: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_331_0_e3711: f64 = (noise_metadata_schedule_331_0_e3710).exp();
        (noise_metadata_schedule_331_0_e3711,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_331_0_e3713;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_332_0_e3735,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[271] == 0.0)) {
        let noise_metadata_schedule_332_0_e3724: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_332_0_e3725: f64 = (noise_metadata_schedule_332_0_e3724).exp();
        let noise_metadata_schedule_332_0_e3729: f64 = (w[143] - w[65]);
        let noise_metadata_schedule_332_0_e3731: f64 = (noise_metadata_schedule_332_0_e3729 * w[112]);
        let noise_metadata_schedule_332_0_e3732: f64 = (1.0 + noise_metadata_schedule_332_0_e3731);
        let noise_metadata_schedule_332_0_e3733: f64 = (noise_metadata_schedule_332_0_e3725 * noise_metadata_schedule_332_0_e3732);
        (noise_metadata_schedule_332_0_e3733,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_332_0_e3735;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_333_0_e3747,) = {
    if ((w[260] == 0.0) && (w[266] == 0.0)) {
        let noise_metadata_schedule_333_0_e3744: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_333_0_e3745: f64 = (1.0 / noise_metadata_schedule_333_0_e3744);
        (noise_metadata_schedule_333_0_e3745,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_333_0_e3747;
        }
        if (active[0] & 0x6f) != 0 {
            let noise_metadata_schedule_334_0_e3750: f64 = if w[143] < w[66] { 1.0 } else { 0.0 };
            w[272] = noise_metadata_schedule_334_0_e3750;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_335_0_e3763,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_335_0_e3760: f64 = (w[143] * w[112]);
        let noise_metadata_schedule_335_0_e3761: f64 = (noise_metadata_schedule_335_0_e3760).exp();
        (noise_metadata_schedule_335_0_e3761,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_335_0_e3763;
        }
        if (active[0] & 0x6f) != 0 {
            let (noise_metadata_schedule_336_0_e3785,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[272] == 0.0)) {
        let noise_metadata_schedule_336_0_e3774: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_336_0_e3775: f64 = (noise_metadata_schedule_336_0_e3774).exp();
        let noise_metadata_schedule_336_0_e3779: f64 = (w[143] - w[66]);
        let noise_metadata_schedule_336_0_e3781: f64 = (noise_metadata_schedule_336_0_e3779 * w[112]);
        let noise_metadata_schedule_336_0_e3782: f64 = (1.0 + noise_metadata_schedule_336_0_e3781);
        let noise_metadata_schedule_336_0_e3783: f64 = (noise_metadata_schedule_336_0_e3775 * noise_metadata_schedule_336_0_e3782);
        (noise_metadata_schedule_336_0_e3783,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_336_0_e3785;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_337_0_e3788: f64 = if params[57] > 0.0 { 1.0 } else { 0.0 };
            w[273] = noise_metadata_schedule_337_0_e3788;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_338_0_e3818,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[273] != 0.0)) {
        let noise_metadata_schedule_338_0_e3802: f64 = (w[79] - 1.0);
        let noise_metadata_schedule_338_0_e3803: f64 = (params[57] * noise_metadata_schedule_338_0_e3802);
        let noise_metadata_schedule_338_0_e3804: f64 = (1.0 + noise_metadata_schedule_338_0_e3803);
        let noise_metadata_schedule_338_0_e3805: f64 = (w[3] * noise_metadata_schedule_338_0_e3804);
        let noise_metadata_schedule_338_0_e3808: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_338_0_e3809: f64 = (noise_metadata_schedule_338_0_e3805 * noise_metadata_schedule_338_0_e3808);
        let noise_metadata_schedule_338_0_e3813: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_338_0_e3814: f64 = (w[6] * noise_metadata_schedule_338_0_e3813);
        let noise_metadata_schedule_338_0_e3815: f64 = (noise_metadata_schedule_338_0_e3809 + noise_metadata_schedule_338_0_e3814);
        let noise_metadata_schedule_338_0_e3816: f64 = (params[55] * noise_metadata_schedule_338_0_e3815);
        (noise_metadata_schedule_338_0_e3816,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_338_0_e3818;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_339_0_e3841,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[273] == 0.0)) {
        let noise_metadata_schedule_339_0_e3831: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_339_0_e3832: f64 = (w[3] * noise_metadata_schedule_339_0_e3831);
        let noise_metadata_schedule_339_0_e3836: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_339_0_e3837: f64 = (w[6] * noise_metadata_schedule_339_0_e3836);
        let noise_metadata_schedule_339_0_e3838: f64 = (noise_metadata_schedule_339_0_e3832 + noise_metadata_schedule_339_0_e3837);
        let noise_metadata_schedule_339_0_e3839: f64 = (params[55] * noise_metadata_schedule_339_0_e3838);
        (noise_metadata_schedule_339_0_e3839,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_339_0_e3841;
        }
        if (active[0] & 0x16f) != 0 {
            let noise_metadata_schedule_340_0_e3844: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[274] = noise_metadata_schedule_340_0_e3844;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_341_0_e3857,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[274] != 0.0)) {
        let noise_metadata_schedule_341_0_e3853: f64 = (-w[31]);
        let noise_metadata_schedule_341_0_e3855: f64 = (noise_metadata_schedule_341_0_e3853 - w[143]);
        (noise_metadata_schedule_341_0_e3855,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_341_0_e3857;
        }
        if (active[0] & 0x16f) != 0 {
            let (noise_metadata_schedule_342_0_e3871,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[274] != 0.0)) {
        let noise_metadata_schedule_342_0_e3868: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_342_0_e3869: f64 = (1.0 / noise_metadata_schedule_342_0_e3868);
        (noise_metadata_schedule_342_0_e3869,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_342_0_e3871;
        }
        if (active[0] & 0x10f) != 0 {
            let noise_metadata_schedule_343_0_e3874: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[275] = noise_metadata_schedule_343_0_e3874;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_344_0_e3889,) = {
    if ((((w[260] == 0.0) && (w[266] == 0.0)) && (w[274] != 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_344_0_e3886: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_344_0_e3887: f64 = (noise_metadata_schedule_344_0_e3886).exp();
        (noise_metadata_schedule_344_0_e3887,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_344_0_e3889;
        }
        if (active[0] & 0x10f) != 0 {
            let (noise_metadata_schedule_345_0_e3913,) = {
    if ((((w[260] == 0.0) && (w[266] == 0.0)) && (w[274] != 0.0)) && (w[275] == 0.0)) {
        let noise_metadata_schedule_345_0_e3902: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_345_0_e3903: f64 = (noise_metadata_schedule_345_0_e3902).exp();
        let noise_metadata_schedule_345_0_e3907: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_345_0_e3909: f64 = (noise_metadata_schedule_345_0_e3907 * w[112]);
        let noise_metadata_schedule_345_0_e3910: f64 = (1.0 + noise_metadata_schedule_345_0_e3909);
        let noise_metadata_schedule_345_0_e3911: f64 = (noise_metadata_schedule_345_0_e3903 * noise_metadata_schedule_345_0_e3910);
        (noise_metadata_schedule_345_0_e3911,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_345_0_e3913;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_346_0_e3931,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[274] != 0.0)) {
        let noise_metadata_schedule_346_0_e3924: f64 = (params[55] * params[90]);
        let noise_metadata_schedule_346_0_e3927: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_346_0_e3928: f64 = (noise_metadata_schedule_346_0_e3924 * noise_metadata_schedule_346_0_e3927);
        let noise_metadata_schedule_346_0_e3929: f64 = (w[87] - noise_metadata_schedule_346_0_e3928);
        (noise_metadata_schedule_346_0_e3929,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_346_0_e3931;
        }
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_347_0_e3943,) = {
    if ((w[260] == 0.0) && (w[266] == 0.0)) {
        let noise_metadata_schedule_347_0_e3940: f64 = (params[56] * w[73]);
        let noise_metadata_schedule_347_0_e3941: f64 = (1.0 / noise_metadata_schedule_347_0_e3940);
        (noise_metadata_schedule_347_0_e3941,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_347_0_e3943;
        }
        if (active[0] & 0x16c) != 0 {
            let noise_metadata_schedule_348_0_e3946: f64 = if w[145] < w[65] { 1.0 } else { 0.0 };
            w[276] = noise_metadata_schedule_348_0_e3946;
        }
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_349_0_e3959,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[276] != 0.0)) {
        let noise_metadata_schedule_349_0_e3956: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_349_0_e3957: f64 = (noise_metadata_schedule_349_0_e3956).exp();
        (noise_metadata_schedule_349_0_e3957,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_349_0_e3959;
        }
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_350_0_e3981,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[276] == 0.0)) {
        let noise_metadata_schedule_350_0_e3970: f64 = (w[65] * w[112]);
        let noise_metadata_schedule_350_0_e3971: f64 = (noise_metadata_schedule_350_0_e3970).exp();
        let noise_metadata_schedule_350_0_e3975: f64 = (w[145] - w[65]);
        let noise_metadata_schedule_350_0_e3977: f64 = (noise_metadata_schedule_350_0_e3975 * w[112]);
        let noise_metadata_schedule_350_0_e3978: f64 = (1.0 + noise_metadata_schedule_350_0_e3977);
        let noise_metadata_schedule_350_0_e3979: f64 = (noise_metadata_schedule_350_0_e3971 * noise_metadata_schedule_350_0_e3978);
        (noise_metadata_schedule_350_0_e3979,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_350_0_e3981;
        }
        if (active[0] & 0x16c) != 0 {
            let (noise_metadata_schedule_351_0_e3993,) = {
    if ((w[260] == 0.0) && (w[266] == 0.0)) {
        let noise_metadata_schedule_351_0_e3990: f64 = (params[59] * w[73]);
        let noise_metadata_schedule_351_0_e3991: f64 = (1.0 / noise_metadata_schedule_351_0_e3990);
        (noise_metadata_schedule_351_0_e3991,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_351_0_e3993;
        }
        if (active[0] & 0x6c) != 0 {
            let noise_metadata_schedule_352_0_e3996: f64 = if w[145] < w[66] { 1.0 } else { 0.0 };
            w[277] = noise_metadata_schedule_352_0_e3996;
        }
        if (active[0] & 0x6c) != 0 {
            let (noise_metadata_schedule_353_0_e4009,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[277] != 0.0)) {
        let noise_metadata_schedule_353_0_e4006: f64 = (w[145] * w[112]);
        let noise_metadata_schedule_353_0_e4007: f64 = (noise_metadata_schedule_353_0_e4006).exp();
        (noise_metadata_schedule_353_0_e4007,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_353_0_e4009;
        }
        if (active[0] & 0x6c) != 0 {
            let (noise_metadata_schedule_354_0_e4031,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[277] == 0.0)) {
        let noise_metadata_schedule_354_0_e4020: f64 = (w[66] * w[112]);
        let noise_metadata_schedule_354_0_e4021: f64 = (noise_metadata_schedule_354_0_e4020).exp();
        let noise_metadata_schedule_354_0_e4025: f64 = (w[145] - w[66]);
        let noise_metadata_schedule_354_0_e4027: f64 = (noise_metadata_schedule_354_0_e4025 * w[112]);
        let noise_metadata_schedule_354_0_e4028: f64 = (1.0 + noise_metadata_schedule_354_0_e4027);
        let noise_metadata_schedule_354_0_e4029: f64 = (noise_metadata_schedule_354_0_e4021 * noise_metadata_schedule_354_0_e4028);
        (noise_metadata_schedule_354_0_e4029,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_354_0_e4031;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_355_0_e4053,) = {
    if ((w[260] == 0.0) && (w[266] == 0.0)) {
        let noise_metadata_schedule_355_0_e4039: f64 = (1.0 - params[55]);
        let noise_metadata_schedule_355_0_e4043: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_355_0_e4044: f64 = (w[3] * noise_metadata_schedule_355_0_e4043);
        let noise_metadata_schedule_355_0_e4048: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_355_0_e4049: f64 = (w[6] * noise_metadata_schedule_355_0_e4048);
        let noise_metadata_schedule_355_0_e4050: f64 = (noise_metadata_schedule_355_0_e4044 + noise_metadata_schedule_355_0_e4049);
        let noise_metadata_schedule_355_0_e4051: f64 = (noise_metadata_schedule_355_0_e4039 * noise_metadata_schedule_355_0_e4050);
        (noise_metadata_schedule_355_0_e4051,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_355_0_e4053;
        }
        if (active[0] & 0x10c) != 0 {
            let noise_metadata_schedule_356_0_e4056: f64 = if params[88] > 0.0 { 1.0 } else { 0.0 };
            w[278] = noise_metadata_schedule_356_0_e4056;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_357_0_e4069,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[278] != 0.0)) {
        let noise_metadata_schedule_357_0_e4065: f64 = (-w[31]);
        let noise_metadata_schedule_357_0_e4067: f64 = (noise_metadata_schedule_357_0_e4065 - w[143]);
        (noise_metadata_schedule_357_0_e4067,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_357_0_e4069;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_358_0_e4083,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[278] != 0.0)) {
        let noise_metadata_schedule_358_0_e4080: f64 = (w[32] * w[73]);
        let noise_metadata_schedule_358_0_e4081: f64 = (1.0 / noise_metadata_schedule_358_0_e4080);
        (noise_metadata_schedule_358_0_e4081,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_358_0_e4083;
        }
        if (active[0] & 0x10c) != 0 {
            let noise_metadata_schedule_359_0_e4086: f64 = if w[150] < w[64] { 1.0 } else { 0.0 };
            w[279] = noise_metadata_schedule_359_0_e4086;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_360_0_e4101,) = {
    if ((((w[260] == 0.0) && (w[266] == 0.0)) && (w[278] != 0.0)) && (w[279] != 0.0)) {
        let noise_metadata_schedule_360_0_e4098: f64 = (w[150] * w[112]);
        let noise_metadata_schedule_360_0_e4099: f64 = (noise_metadata_schedule_360_0_e4098).exp();
        (noise_metadata_schedule_360_0_e4099,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_360_0_e4101;
        }
        if (active[0] & 0x10c) != 0 {
            let (noise_metadata_schedule_361_0_e4125,) = {
    if ((((w[260] == 0.0) && (w[266] == 0.0)) && (w[278] != 0.0)) && (w[279] == 0.0)) {
        let noise_metadata_schedule_361_0_e4114: f64 = (w[64] * w[112]);
        let noise_metadata_schedule_361_0_e4115: f64 = (noise_metadata_schedule_361_0_e4114).exp();
        let noise_metadata_schedule_361_0_e4119: f64 = (w[150] - w[64]);
        let noise_metadata_schedule_361_0_e4121: f64 = (noise_metadata_schedule_361_0_e4119 * w[112]);
        let noise_metadata_schedule_361_0_e4122: f64 = (1.0 + noise_metadata_schedule_361_0_e4121);
        let noise_metadata_schedule_361_0_e4123: f64 = (noise_metadata_schedule_361_0_e4115 * noise_metadata_schedule_361_0_e4122);
        (noise_metadata_schedule_361_0_e4123,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_361_0_e4125;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_362_0_e4145,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[278] != 0.0)) {
        let noise_metadata_schedule_362_0_e4136: f64 = (1.0 - params[55]);
        let noise_metadata_schedule_362_0_e4138: f64 = (noise_metadata_schedule_362_0_e4136 * params[90]);
        let noise_metadata_schedule_362_0_e4141: f64 = (w[111] - w[35]);
        let noise_metadata_schedule_362_0_e4142: f64 = (noise_metadata_schedule_362_0_e4138 * noise_metadata_schedule_362_0_e4141);
        let noise_metadata_schedule_362_0_e4143: f64 = (w[88] - noise_metadata_schedule_362_0_e4142);
        (noise_metadata_schedule_362_0_e4143,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_362_0_e4145;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_363_0_e4149: f64 = (params[61] * w[73]);
            let noise_metadata_schedule_363_0_e4150: f64 = (1.0 / noise_metadata_schedule_363_0_e4149);
            w[112] = noise_metadata_schedule_363_0_e4150;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_364_0_e4153: f64 = if w[144] < w[67] { 1.0 } else { 0.0 };
            w[280] = noise_metadata_schedule_364_0_e4153;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_365_0_e4160,) = {
    if (w[280] != 0.0) {
        let noise_metadata_schedule_365_0_e4157: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_365_0_e4158: f64 = (noise_metadata_schedule_365_0_e4157).exp();
        (noise_metadata_schedule_365_0_e4158,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_365_0_e4160;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_366_0_e4176,) = {
    if (w[280] == 0.0) {
        let noise_metadata_schedule_366_0_e4165: f64 = (w[67] * w[112]);
        let noise_metadata_schedule_366_0_e4166: f64 = (noise_metadata_schedule_366_0_e4165).exp();
        let noise_metadata_schedule_366_0_e4170: f64 = (w[144] - w[67]);
        let noise_metadata_schedule_366_0_e4172: f64 = (noise_metadata_schedule_366_0_e4170 * w[112]);
        let noise_metadata_schedule_366_0_e4173: f64 = (1.0 + noise_metadata_schedule_366_0_e4172);
        let noise_metadata_schedule_366_0_e4174: f64 = (noise_metadata_schedule_366_0_e4166 * noise_metadata_schedule_366_0_e4173);
        (noise_metadata_schedule_366_0_e4174,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_366_0_e4176;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_367_0_e4180: f64 = (params[63] * w[73]);
            let noise_metadata_schedule_367_0_e4181: f64 = (1.0 / noise_metadata_schedule_367_0_e4180);
            w[112] = noise_metadata_schedule_367_0_e4181;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_368_0_e4184: f64 = if w[144] < w[68] { 1.0 } else { 0.0 };
            w[281] = noise_metadata_schedule_368_0_e4184;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_369_0_e4191,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_369_0_e4188: f64 = (w[144] * w[112]);
        let noise_metadata_schedule_369_0_e4189: f64 = (noise_metadata_schedule_369_0_e4188).exp();
        (noise_metadata_schedule_369_0_e4189,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_369_0_e4191;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_370_0_e4207,) = {
    if (w[281] == 0.0) {
        let noise_metadata_schedule_370_0_e4196: f64 = (w[68] * w[112]);
        let noise_metadata_schedule_370_0_e4197: f64 = (noise_metadata_schedule_370_0_e4196).exp();
        let noise_metadata_schedule_370_0_e4201: f64 = (w[144] - w[68]);
        let noise_metadata_schedule_370_0_e4203: f64 = (noise_metadata_schedule_370_0_e4201 * w[112]);
        let noise_metadata_schedule_370_0_e4204: f64 = (1.0 + noise_metadata_schedule_370_0_e4203);
        let noise_metadata_schedule_370_0_e4205: f64 = (noise_metadata_schedule_370_0_e4197 * noise_metadata_schedule_370_0_e4204);
        (noise_metadata_schedule_370_0_e4205,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_370_0_e4207;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_372_0_e4225: f64 = if ((params[64] > 0.0) || (params[65] > 0.0)) { 1.0 } else { 0.0 };
            w[282] = noise_metadata_schedule_372_0_e4225;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_373_0_e4233,) = {
    if (w[282] != 0.0) {
        let noise_metadata_schedule_373_0_e4230: f64 = (params[61] * w[73]);
        let noise_metadata_schedule_373_0_e4231: f64 = (1.0 / noise_metadata_schedule_373_0_e4230);
        (noise_metadata_schedule_373_0_e4231,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_373_0_e4233;
        }
        if (active[0] & 0x160) != 0 {
            let noise_metadata_schedule_374_0_e4236: f64 = if w[146] < w[69] { 1.0 } else { 0.0 };
            w[283] = noise_metadata_schedule_374_0_e4236;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_375_0_e4245,) = {
    if ((w[282] != 0.0) && (w[283] != 0.0)) {
        let noise_metadata_schedule_375_0_e4242: f64 = (w[146] * w[112]);
        let noise_metadata_schedule_375_0_e4243: f64 = (noise_metadata_schedule_375_0_e4242).exp();
        (noise_metadata_schedule_375_0_e4243,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_375_0_e4245;
        }
        if (active[0] & 0x160) != 0 {
            let (noise_metadata_schedule_376_0_e4263,) = {
    if ((w[282] != 0.0) && (w[283] == 0.0)) {
        let noise_metadata_schedule_376_0_e4252: f64 = (w[69] * w[112]);
        let noise_metadata_schedule_376_0_e4253: f64 = (noise_metadata_schedule_376_0_e4252).exp();
        let noise_metadata_schedule_376_0_e4257: f64 = (w[146] - w[69]);
        let noise_metadata_schedule_376_0_e4259: f64 = (noise_metadata_schedule_376_0_e4257 * w[112]);
        let noise_metadata_schedule_376_0_e4260: f64 = (1.0 + noise_metadata_schedule_376_0_e4259);
        let noise_metadata_schedule_376_0_e4261: f64 = (noise_metadata_schedule_376_0_e4253 * noise_metadata_schedule_376_0_e4260);
        (noise_metadata_schedule_376_0_e4261,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_376_0_e4263;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_377_0_e4271,) = {
    if (w[282] != 0.0) {
        let noise_metadata_schedule_377_0_e4268: f64 = (params[63] * w[73]);
        let noise_metadata_schedule_377_0_e4269: f64 = (1.0 / noise_metadata_schedule_377_0_e4268);
        (noise_metadata_schedule_377_0_e4269,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_377_0_e4271;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_378_0_e4274: f64 = if w[146] < w[70] { 1.0 } else { 0.0 };
            w[284] = noise_metadata_schedule_378_0_e4274;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_379_0_e4283,) = {
    if ((w[282] != 0.0) && (w[284] != 0.0)) {
        let noise_metadata_schedule_379_0_e4280: f64 = (w[146] * w[112]);
        let noise_metadata_schedule_379_0_e4281: f64 = (noise_metadata_schedule_379_0_e4280).exp();
        (noise_metadata_schedule_379_0_e4281,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_379_0_e4283;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_380_0_e4301,) = {
    if ((w[282] != 0.0) && (w[284] == 0.0)) {
        let noise_metadata_schedule_380_0_e4290: f64 = (w[70] * w[112]);
        let noise_metadata_schedule_380_0_e4291: f64 = (noise_metadata_schedule_380_0_e4290).exp();
        let noise_metadata_schedule_380_0_e4295: f64 = (w[146] - w[70]);
        let noise_metadata_schedule_380_0_e4297: f64 = (noise_metadata_schedule_380_0_e4295 * w[112]);
        let noise_metadata_schedule_380_0_e4298: f64 = (1.0 + noise_metadata_schedule_380_0_e4297);
        let noise_metadata_schedule_380_0_e4299: f64 = (noise_metadata_schedule_380_0_e4291 * noise_metadata_schedule_380_0_e4298);
        (noise_metadata_schedule_380_0_e4299,)
    } else {
        (w[110],)
    }
};
            w[110] = noise_metadata_schedule_380_0_e4301;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_381_0_e4315,) = {
    if (w[282] != 0.0) {
        let noise_metadata_schedule_381_0_e4306: f64 = (w[109] - 1.0);
        let noise_metadata_schedule_381_0_e4307: f64 = (w[8] * noise_metadata_schedule_381_0_e4306);
        let noise_metadata_schedule_381_0_e4311: f64 = (w[110] - 1.0);
        let noise_metadata_schedule_381_0_e4312: f64 = (w[9] * noise_metadata_schedule_381_0_e4311);
        let noise_metadata_schedule_381_0_e4313: f64 = (noise_metadata_schedule_381_0_e4307 + noise_metadata_schedule_381_0_e4312);
        (noise_metadata_schedule_381_0_e4313,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_381_0_e4315;
        }
        if (active[0] & 0x60) != 0 {
            let (noise_metadata_schedule_382_0_e4320,) = {
    if (w[282] == 0.0) {
        (0.0,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_382_0_e4320;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_383_0_e4323: f64 = (w[144] / w[73]);
            w[108] = noise_metadata_schedule_383_0_e4323;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_384_0_e4326: f64 = if w[108] < w[113] { 1.0 } else { 0.0 };
            w[285] = noise_metadata_schedule_384_0_e4326;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_385_0_e4331,) = {
    if (w[285] != 0.0) {
        let noise_metadata_schedule_385_0_e4329: f64 = (w[108]).exp();
        (noise_metadata_schedule_385_0_e4329,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_385_0_e4331;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_386_0_e4343,) = {
    if (w[285] == 0.0) {
        let noise_metadata_schedule_386_0_e4335: f64 = (w[113]).exp();
        let noise_metadata_schedule_386_0_e4339: f64 = (w[108] - w[113]);
        let noise_metadata_schedule_386_0_e4340: f64 = (1.0 + noise_metadata_schedule_386_0_e4339);
        let noise_metadata_schedule_386_0_e4341: f64 = (noise_metadata_schedule_386_0_e4335 * noise_metadata_schedule_386_0_e4340);
        (noise_metadata_schedule_386_0_e4341,)
    } else {
        (w[109],)
    }
};
            w[109] = noise_metadata_schedule_386_0_e4343;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_387_0_e4346: f64 = (w[148] / w[73]);
            w[108] = noise_metadata_schedule_387_0_e4346;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_388_0_e4349: f64 = if w[108] < w[113] { 1.0 } else { 0.0 };
            w[286] = noise_metadata_schedule_388_0_e4349;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 359], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_389_0_e4354,) = {
    if (w[286] != 0.0) {
        let noise_metadata_schedule_389_0_e4352: f64 = (w[108]).exp();
        (noise_metadata_schedule_389_0_e4352,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_389_0_e4354;
        }
        if (active[0] & 0x100) != 0 {
            let (noise_metadata_schedule_390_0_e4366,) = {
    if (w[286] == 0.0) {
        let noise_metadata_schedule_390_0_e4358: f64 = (w[113]).exp();
        let noise_metadata_schedule_390_0_e4362: f64 = (w[108] - w[113]);
        let noise_metadata_schedule_390_0_e4363: f64 = (1.0 + noise_metadata_schedule_390_0_e4362);
        let noise_metadata_schedule_390_0_e4364: f64 = (noise_metadata_schedule_390_0_e4358 * noise_metadata_schedule_390_0_e4363);
        (noise_metadata_schedule_390_0_e4364,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_390_0_e4366;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_391_0_e4370: f64 = (w[33] * w[109]);
            let noise_metadata_schedule_391_0_e4371: f64 = (1.0 + noise_metadata_schedule_391_0_e4370);
            let noise_metadata_schedule_391_0_e4372: f64 = (noise_metadata_schedule_391_0_e4371).sqrt();
            w[103] = noise_metadata_schedule_391_0_e4372;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_392_0_e4376: f64 = (w[33] * w[111]);
            let noise_metadata_schedule_392_0_e4377: f64 = (1.0 + noise_metadata_schedule_392_0_e4376);
            let noise_metadata_schedule_392_0_e4378: f64 = (noise_metadata_schedule_392_0_e4377).sqrt();
            w[104] = noise_metadata_schedule_392_0_e4378;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_394_0_e4384: f64 = (w[103] + 1.0);
            let noise_metadata_schedule_394_0_e4387: f64 = (w[104] + 1.0);
            let noise_metadata_schedule_394_0_e4388: f64 = (noise_metadata_schedule_394_0_e4384 / noise_metadata_schedule_394_0_e4387);
            w[105] = noise_metadata_schedule_394_0_e4388;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_395_0_e4393: f64 = (w[103] - w[104]);
            let noise_metadata_schedule_395_0_e4395: f64 = (w[105]).ln();
            let noise_metadata_schedule_395_0_e4396: f64 = (noise_metadata_schedule_395_0_e4393 - noise_metadata_schedule_395_0_e4395);
            let noise_metadata_schedule_395_0_e4397: f64 = (w[73] * noise_metadata_schedule_395_0_e4396);
            let noise_metadata_schedule_395_0_e4398: f64 = (w[154] + noise_metadata_schedule_395_0_e4397);
            let noise_metadata_schedule_395_0_e4400: f64 = (noise_metadata_schedule_395_0_e4398 * w[54]);
            w[106] = noise_metadata_schedule_395_0_e4400;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_396_0_e4403: f64 = (w[48] * w[106]);
            let noise_metadata_schedule_396_0_e4408: f64 = (0.5 * w[48]);
            let noise_metadata_schedule_396_0_e4410: f64 = (noise_metadata_schedule_396_0_e4408 * w[49]);
            let noise_metadata_schedule_396_0_e4413: f64 = (w[154] * w[154]);
            let noise_metadata_schedule_396_0_e4415: f64 = (noise_metadata_schedule_396_0_e4413 + 0.01);
            let noise_metadata_schedule_396_0_e4416: f64 = (noise_metadata_schedule_396_0_e4415).sqrt();
            let noise_metadata_schedule_396_0_e4417: f64 = (noise_metadata_schedule_396_0_e4410 * noise_metadata_schedule_396_0_e4416);
            let noise_metadata_schedule_396_0_e4418: f64 = (1.0 + noise_metadata_schedule_396_0_e4417);
            let noise_metadata_schedule_396_0_e4419: f64 = (w[54] * noise_metadata_schedule_396_0_e4418);
            let noise_metadata_schedule_396_0_e4420: f64 = (noise_metadata_schedule_396_0_e4403 / noise_metadata_schedule_396_0_e4419);
            w[107] = noise_metadata_schedule_396_0_e4420;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_397_0_e4425: f64 = (w[107] * w[107]);
            let noise_metadata_schedule_397_0_e4426: f64 = (1.0 + noise_metadata_schedule_397_0_e4425);
            let noise_metadata_schedule_397_0_e4427: f64 = (noise_metadata_schedule_397_0_e4426).sqrt();
            let noise_metadata_schedule_397_0_e4428: f64 = (w[106] / noise_metadata_schedule_397_0_e4427);
            w[97] = noise_metadata_schedule_397_0_e4428;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_450_0_e4914: f64 = (w[165] * w[143]);
            let noise_metadata_schedule_450_0_e4915: f64 = (w[87] + noise_metadata_schedule_450_0_e4914);
            w[87] = noise_metadata_schedule_450_0_e4915;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_451_0_e4919: f64 = (w[165] * w[145]);
            let noise_metadata_schedule_451_0_e4920: f64 = (w[88] + noise_metadata_schedule_451_0_e4919);
            w[88] = noise_metadata_schedule_451_0_e4920;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_452_0_e4924: f64 = (w[165] * w[146]);
            let noise_metadata_schedule_452_0_e4925: f64 = (w[91] + noise_metadata_schedule_452_0_e4924);
            w[91] = noise_metadata_schedule_452_0_e4925;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_456_0_e4943: f64 = w[162];
            let noise_metadata_schedule_456_0_e4945: f64 = (noise_metadata_schedule_456_0_e4943 * w[87]);
            w[87] = noise_metadata_schedule_456_0_e4945;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_457_0_e4948: f64 = w[162];
            let noise_metadata_schedule_457_0_e4950: f64 = (noise_metadata_schedule_457_0_e4948 * w[88]);
            w[88] = noise_metadata_schedule_457_0_e4950;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_458_0_e4953: f64 = w[162];
            let noise_metadata_schedule_458_0_e4955: f64 = (noise_metadata_schedule_458_0_e4953 * w[76]);
            w[76] = noise_metadata_schedule_458_0_e4955;
        }
        if (active[0] & 0x60) != 0 {
            let noise_metadata_schedule_463_0_e4978: f64 = w[162];
            let noise_metadata_schedule_463_0_e4980: f64 = (noise_metadata_schedule_463_0_e4978 * w[91]);
            w[91] = noise_metadata_schedule_463_0_e4980;
        }
        if (active[0] & 0x100) != 0 {
            let noise_metadata_schedule_465_0_e4986: f64 = w[162];
            let noise_metadata_schedule_465_0_e4988: f64 = (noise_metadata_schedule_465_0_e4986 * w[97]);
            w[97] = noise_metadata_schedule_465_0_e4988;
        }
        if (active[0] & 0x2000) != 0 {
            let noise_metadata_schedule_471_0_e5008: f64 = w[162];
            let noise_metadata_schedule_471_0_e5010: f64 = (noise_metadata_schedule_471_0_e5008 * w[84]);
            w[84] = noise_metadata_schedule_471_0_e5010;
        }
    }
}

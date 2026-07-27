#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("rb"), kind: GeneratedNoiseKind::White, equation: 26, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_C_RCX", label: Some("rcx"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_EI_E_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("ibe"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 386];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            w[364] != 0.0
        };
        let noise_source_1_active = {
            w[365] != 0.0
        };
        let noise_source_2_active = {
            w[366] != 0.0
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
        self.noise_metadata_schedule_part_7(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_8(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_9(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e7957: f64 = 1.0;
            let noise_0_psd_e220: f64 = (w[361] / w[156]);
            let noise_0_psd_e7958: f64 = (noise_0_psd_e7957 * noise_0_psd_e220);
            let psd = noise_0_psd_e7958;
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
            let noise_1_psd_e7960: f64 = 1.0;
            let noise_1_psd_e228: f64 = (w[361] / w[40]);
            let noise_1_psd_e7961: f64 = (noise_1_psd_e7960 * noise_1_psd_e228);
            let psd = noise_1_psd_e7961;
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
            let noise_2_psd_e7963: f64 = 1.0;
            let noise_2_psd_e236: f64 = (w[361] / w[41]);
            let noise_2_psd_e7964: f64 = (noise_2_psd_e7963 * noise_2_psd_e236);
            let psd = noise_2_psd_e7964;
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
            let noise_3_psd_e7966: f64 = 1.0;
            let noise_3_psd_e7967: f64 = (noise_3_psd_e7966 * w[362]);
            let psd = noise_3_psd_e7967;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = Some(1.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[4] {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_4_psd_e7969: f64 = 1.0;
            let noise_4_psd_e247: f64 = (w[195]).abs();
            let noise_4_psd_e248: f64 = (w[363] * noise_4_psd_e247);
            let noise_4_psd_e7970: f64 = (noise_4_psd_e7969 * noise_4_psd_e248);
            let psd = noise_4_psd_e7970;
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
            let noise_5_psd_e7972: f64 = 1.0;
            let noise_5_psd_e253: f64 = (w[132]).abs();
            let noise_5_psd_e254: f64 = (w[363] * noise_5_psd_e253);
            let noise_5_psd_e7973: f64 = (noise_5_psd_e7972 * noise_5_psd_e254);
            let psd = noise_5_psd_e7973;
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
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386]) {
        let params = &*self.params;
        let noise_activation_schedule_762_0_e7692: f64 = if ((params[23] >= params[111]) || (params[26] >= params[111])) { 1.0 } else { 0.0 };
        w[364] = noise_activation_schedule_762_0_e7692;
        let noise_activation_schedule_763_0_e7695: f64 = if params[29] >= params[111] { 1.0 } else { 0.0 };
        w[365] = noise_activation_schedule_763_0_e7695;
        let noise_activation_schedule_764_0_e7698: f64 = if params[28] >= params[111] { 1.0 } else { 0.0 };
        w[366] = noise_activation_schedule_764_0_e7698;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_0_0_e259: f64 = (params[110] * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            w[183] = noise_metadata_schedule_0_0_e259;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_1_0_e262: f64 = (params[110] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[5])));
            w[184] = noise_metadata_schedule_1_0_e262;
        }
        if (active[0] & 0x39) != 0 {
            let noise_metadata_schedule_2_0_e265: f64 = (params[110] * (ctx.node_voltage(self.nodes[6]) - ctx.node_voltage(self.nodes[7])));
            w[185] = noise_metadata_schedule_2_0_e265;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_3_0_e268: f64 = (w[185] - w[184]);
            w[186] = noise_metadata_schedule_3_0_e268;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_9_0_e280: f64 = (params[108] + 273.15);
            w[8] = noise_metadata_schedule_9_0_e280;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_10_0_e281: f64 = ctx.temperature();
            w[9] = noise_metadata_schedule_10_0_e281;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_11_0_e284: f64 = (1.3806226e-23 * w[8]);
            let noise_metadata_schedule_11_0_e286: f64 = (noise_metadata_schedule_11_0_e284 / 1.602176462e-19);
            w[177] = noise_metadata_schedule_11_0_e286;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_12_0_e289: f64 = (params[88] * w[8]);
            w[172] = noise_metadata_schedule_12_0_e289;
        }
        if (active[0] & 0x39) != 0 {
            let noise_metadata_schedule_13_0_e293: f64 = (params[76] + params[77]);
            let noise_metadata_schedule_13_0_e294: f64 = (0.5 * noise_metadata_schedule_13_0_e293);
            w[173] = noise_metadata_schedule_13_0_e294;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_14_0_e298: f64 = (params[76] + params[78]);
            let noise_metadata_schedule_14_0_e299: f64 = (0.5 * noise_metadata_schedule_14_0_e298);
            w[174] = noise_metadata_schedule_14_0_e299;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_15_0_e303: f64 = (params[79] + params[78]);
            let noise_metadata_schedule_15_0_e304: f64 = (0.5 * noise_metadata_schedule_15_0_e303);
            w[175] = noise_metadata_schedule_15_0_e304;
        }
        if (active[0] & 0x39) != 0 {
            let noise_metadata_schedule_16_0_e308: f64 = (1.602176462e-19 * params[80]);
            let noise_metadata_schedule_16_0_e310: f64 = (noise_metadata_schedule_16_0_e308 / 1.3806226e-23);
            let noise_metadata_schedule_16_0_e311: f64 = (3.0 - noise_metadata_schedule_16_0_e310);
            w[168] = noise_metadata_schedule_16_0_e311;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_17_0_e314: f64 = (w[168] + 1.0);
            let noise_metadata_schedule_17_0_e316: f64 = (noise_metadata_schedule_17_0_e314 - params[87]);
            w[169] = noise_metadata_schedule_17_0_e316;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_20_0_e327: f64 = (params[76] - params[77]);
            w[176] = noise_metadata_schedule_20_0_e327;
        }
        if (active[0] & 0x21) != 0 {
            w[27] = params[34];
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_26_0_e348: f64 = (w[9] + params[109]);
            let noise_metadata_schedule_26_0_e350: f64 = noise_metadata_schedule_26_0_e348;
            w[4] = noise_metadata_schedule_26_0_e350;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_27_0_e353: f64 = (-100.0);
            let noise_metadata_schedule_27_0_e355: f64 = (noise_metadata_schedule_27_0_e353 + 273.15);
            let noise_metadata_schedule_27_0_e356: f64 = if w[4] < noise_metadata_schedule_27_0_e355 { 1.0 } else { 0.0 };
            w[247] = noise_metadata_schedule_27_0_e356;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_28_0_e363,) = {
    if (w[247] != 0.0) {
        let noise_metadata_schedule_28_0_e359: f64 = (-100.0);
        let noise_metadata_schedule_28_0_e361: f64 = (noise_metadata_schedule_28_0_e359 + 273.15);
        (noise_metadata_schedule_28_0_e361,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_28_0_e363;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_29_0_e367: f64 = (326.85 + 273.15);
            let noise_metadata_schedule_29_0_e368: f64 = if w[4] > noise_metadata_schedule_29_0_e367 { 1.0 } else { 0.0 };
            w[248] = noise_metadata_schedule_29_0_e368;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_30_0_e377,) = {
    if ((w[247] == 0.0) && (w[248] != 0.0)) {
        let noise_metadata_schedule_30_0_e375: f64 = (326.85 + 273.15);
        (noise_metadata_schedule_30_0_e375,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_30_0_e377;
        }
        if (active[0] & 0x39) != 0 {
            let noise_metadata_schedule_31_0_e380: f64 = (1.3806226e-23 * w[4]);
            let noise_metadata_schedule_31_0_e382: f64 = (noise_metadata_schedule_31_0_e380 / 1.602176462e-19);
            w[2] = noise_metadata_schedule_31_0_e382;
        }
        if (active[0] & 0x39) != 0 {
            let noise_metadata_schedule_32_0_e385: f64 = (1.0 / w[2]);
            w[3] = noise_metadata_schedule_32_0_e385;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_33_0_e388: f64 = (w[4] - w[8]);
            w[7] = noise_metadata_schedule_33_0_e388;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_34_0_e391: f64 = (w[4] / w[8]);
            w[5] = noise_metadata_schedule_34_0_e391;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_35_0_e393: f64 = (w[5]).ln();
            w[6] = noise_metadata_schedule_35_0_e393;
        }
        if (active[0] & 0x39) != 0 {
            let noise_metadata_schedule_36_0_e397: f64 = (w[5] - 1.0);
            let noise_metadata_schedule_36_0_e398: f64 = (w[3] * noise_metadata_schedule_36_0_e397);
            w[10] = noise_metadata_schedule_36_0_e398;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_37_0_e401: f64 = (0.5 * params[35]);
            let noise_metadata_schedule_37_0_e403: f64 = (noise_metadata_schedule_37_0_e401 / w[177]);
            w[178] = noise_metadata_schedule_37_0_e403;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_38_0_e406: f64 = (2.0 * w[177]);
            let noise_metadata_schedule_38_0_e408: f64 = (w[178]).exp();
            let noise_metadata_schedule_38_0_e410: f64 = (-w[178]);
            let noise_metadata_schedule_38_0_e411: f64 = (noise_metadata_schedule_38_0_e410).exp();
            let noise_metadata_schedule_38_0_e412: f64 = (noise_metadata_schedule_38_0_e408 - noise_metadata_schedule_38_0_e411);
            let noise_metadata_schedule_38_0_e413: f64 = (noise_metadata_schedule_38_0_e412).ln();
            let noise_metadata_schedule_38_0_e414: f64 = (noise_metadata_schedule_38_0_e406 * noise_metadata_schedule_38_0_e413);
            w[96] = noise_metadata_schedule_38_0_e414;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_39_0_e417: f64 = (w[96] * w[5]);
            let noise_metadata_schedule_39_0_e421: f64 = (1.0 - w[5]);
            let noise_metadata_schedule_39_0_e422: f64 = (w[173] * noise_metadata_schedule_39_0_e421);
            let noise_metadata_schedule_39_0_e423: f64 = (noise_metadata_schedule_39_0_e417 + noise_metadata_schedule_39_0_e422);
            let noise_metadata_schedule_39_0_e426: f64 = (w[168] * w[2]);
            let noise_metadata_schedule_39_0_e428: f64 = (noise_metadata_schedule_39_0_e426 * w[6]);
            let noise_metadata_schedule_39_0_e429: f64 = (noise_metadata_schedule_39_0_e423 - noise_metadata_schedule_39_0_e428);
            w[97] = noise_metadata_schedule_39_0_e429;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_40_0_e433: f64 = (2.0 * w[2]);
            let noise_metadata_schedule_40_0_e439: f64 = (-w[97]);
            let noise_metadata_schedule_40_0_e441: f64 = (noise_metadata_schedule_40_0_e439 * w[3]);
            let noise_metadata_schedule_40_0_e442: f64 = (noise_metadata_schedule_40_0_e441).exp();
            let noise_metadata_schedule_40_0_e443: f64 = (4.0 * noise_metadata_schedule_40_0_e442);
            let noise_metadata_schedule_40_0_e444: f64 = (1.0 + noise_metadata_schedule_40_0_e443);
            let noise_metadata_schedule_40_0_e445: f64 = (noise_metadata_schedule_40_0_e444).sqrt();
            let noise_metadata_schedule_40_0_e446: f64 = (1.0 + noise_metadata_schedule_40_0_e445);
            let noise_metadata_schedule_40_0_e447: f64 = (0.5 * noise_metadata_schedule_40_0_e446);
            let noise_metadata_schedule_40_0_e448: f64 = (noise_metadata_schedule_40_0_e447).ln();
            let noise_metadata_schedule_40_0_e449: f64 = (noise_metadata_schedule_40_0_e433 * noise_metadata_schedule_40_0_e448);
            let noise_metadata_schedule_40_0_e450: f64 = (w[97] + noise_metadata_schedule_40_0_e449);
            w[16] = noise_metadata_schedule_40_0_e450;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_41_0_e455: f64 = (params[35] / w[16]);
            let noise_metadata_schedule_41_0_e456: f64 = (noise_metadata_schedule_41_0_e455).ln();
            let noise_metadata_schedule_41_0_e457: f64 = (params[36] * noise_metadata_schedule_41_0_e456);
            let noise_metadata_schedule_41_0_e458: f64 = (noise_metadata_schedule_41_0_e457).exp();
            let noise_metadata_schedule_41_0_e459: f64 = (params[34] * noise_metadata_schedule_41_0_e458);
            w[23] = noise_metadata_schedule_41_0_e459;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_42_0_e462: f64 = (params[37] * w[16]);
            let noise_metadata_schedule_42_0_e464: f64 = (noise_metadata_schedule_42_0_e462 / params[35]);
            w[43] = noise_metadata_schedule_42_0_e464;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_43_0_e467: f64 = (0.5 * params[38]);
            let noise_metadata_schedule_43_0_e469: f64 = (noise_metadata_schedule_43_0_e467 / w[177]);
            w[178] = noise_metadata_schedule_43_0_e469;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_44_0_e472: f64 = (2.0 * w[177]);
            let noise_metadata_schedule_44_0_e474: f64 = (w[178]).exp();
            let noise_metadata_schedule_44_0_e476: f64 = (-w[178]);
            let noise_metadata_schedule_44_0_e477: f64 = (noise_metadata_schedule_44_0_e476).exp();
            let noise_metadata_schedule_44_0_e478: f64 = (noise_metadata_schedule_44_0_e474 - noise_metadata_schedule_44_0_e477);
            let noise_metadata_schedule_44_0_e479: f64 = (noise_metadata_schedule_44_0_e478).ln();
            let noise_metadata_schedule_44_0_e480: f64 = (noise_metadata_schedule_44_0_e472 * noise_metadata_schedule_44_0_e479);
            w[96] = noise_metadata_schedule_44_0_e480;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_45_0_e483: f64 = (w[96] * w[5]);
            let noise_metadata_schedule_45_0_e487: f64 = (1.0 - w[5]);
            let noise_metadata_schedule_45_0_e488: f64 = (w[173] * noise_metadata_schedule_45_0_e487);
            let noise_metadata_schedule_45_0_e489: f64 = (noise_metadata_schedule_45_0_e483 + noise_metadata_schedule_45_0_e488);
            let noise_metadata_schedule_45_0_e492: f64 = (w[168] * w[2]);
            let noise_metadata_schedule_45_0_e494: f64 = (noise_metadata_schedule_45_0_e492 * w[6]);
            let noise_metadata_schedule_45_0_e495: f64 = (noise_metadata_schedule_45_0_e489 - noise_metadata_schedule_45_0_e494);
            w[97] = noise_metadata_schedule_45_0_e495;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_46_0_e499: f64 = (2.0 * w[2]);
            let noise_metadata_schedule_46_0_e505: f64 = (-w[97]);
            let noise_metadata_schedule_46_0_e507: f64 = (noise_metadata_schedule_46_0_e505 * w[3]);
            let noise_metadata_schedule_46_0_e508: f64 = (noise_metadata_schedule_46_0_e507).exp();
            let noise_metadata_schedule_46_0_e509: f64 = (4.0 * noise_metadata_schedule_46_0_e508);
            let noise_metadata_schedule_46_0_e510: f64 = (1.0 + noise_metadata_schedule_46_0_e509);
            let noise_metadata_schedule_46_0_e511: f64 = (noise_metadata_schedule_46_0_e510).sqrt();
            let noise_metadata_schedule_46_0_e512: f64 = (1.0 + noise_metadata_schedule_46_0_e511);
            let noise_metadata_schedule_46_0_e513: f64 = (0.5 * noise_metadata_schedule_46_0_e512);
            let noise_metadata_schedule_46_0_e514: f64 = (noise_metadata_schedule_46_0_e513).ln();
            let noise_metadata_schedule_46_0_e515: f64 = (noise_metadata_schedule_46_0_e499 * noise_metadata_schedule_46_0_e514);
            let noise_metadata_schedule_46_0_e516: f64 = (w[97] + noise_metadata_schedule_46_0_e515);
            w[22] = noise_metadata_schedule_46_0_e516;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_47_0_e521: f64 = (params[38] / w[22]);
            let noise_metadata_schedule_47_0_e522: f64 = (noise_metadata_schedule_47_0_e521).ln();
            let noise_metadata_schedule_47_0_e523: f64 = (params[39] * noise_metadata_schedule_47_0_e522);
            let noise_metadata_schedule_47_0_e524: f64 = (noise_metadata_schedule_47_0_e523).exp();
            let noise_metadata_schedule_47_0_e525: f64 = (w[27] * noise_metadata_schedule_47_0_e524);
            w[26] = noise_metadata_schedule_47_0_e525;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_48_0_e528: f64 = (params[40] * w[22]);
            let noise_metadata_schedule_48_0_e530: f64 = (noise_metadata_schedule_48_0_e528 / params[38]);
            w[44] = noise_metadata_schedule_48_0_e530;
        }
        if (active[0] & 0x19) != 0 {
            let noise_metadata_schedule_49_0_e534: f64 = (params[82] * w[6]);
            let noise_metadata_schedule_49_0_e537: f64 = (params[77] * w[10]);
            let noise_metadata_schedule_49_0_e538: f64 = (noise_metadata_schedule_49_0_e534 + noise_metadata_schedule_49_0_e537);
            let noise_metadata_schedule_49_0_e539: f64 = (noise_metadata_schedule_49_0_e538).exp();
            let noise_metadata_schedule_49_0_e540: f64 = (params[15] * noise_metadata_schedule_49_0_e539);
            w[13] = noise_metadata_schedule_49_0_e540;
        }
        if (active[0] & 0x19) != 0 {
            let noise_metadata_schedule_50_0_e544: f64 = (0.5 * w[168]);
            let noise_metadata_schedule_50_0_e546: f64 = (noise_metadata_schedule_50_0_e544 * w[6]);
            let noise_metadata_schedule_50_0_e549: f64 = (0.5 * w[173]);
            let noise_metadata_schedule_50_0_e551: f64 = (noise_metadata_schedule_50_0_e549 * w[10]);
            let noise_metadata_schedule_50_0_e552: f64 = (noise_metadata_schedule_50_0_e546 + noise_metadata_schedule_50_0_e551);
            let noise_metadata_schedule_50_0_e553: f64 = (noise_metadata_schedule_50_0_e552).exp();
            let noise_metadata_schedule_50_0_e554: f64 = (params[17] * noise_metadata_schedule_50_0_e553);
            w[12] = noise_metadata_schedule_50_0_e554;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_51_0_e557: f64 = (0.5 * params[42]);
            let noise_metadata_schedule_51_0_e559: f64 = (noise_metadata_schedule_51_0_e557 / w[177]);
            w[178] = noise_metadata_schedule_51_0_e559;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_52_0_e562: f64 = (2.0 * w[177]);
            let noise_metadata_schedule_52_0_e564: f64 = (w[178]).exp();
            let noise_metadata_schedule_52_0_e566: f64 = (-w[178]);
            let noise_metadata_schedule_52_0_e567: f64 = (noise_metadata_schedule_52_0_e566).exp();
            let noise_metadata_schedule_52_0_e568: f64 = (noise_metadata_schedule_52_0_e564 - noise_metadata_schedule_52_0_e567);
            let noise_metadata_schedule_52_0_e569: f64 = (noise_metadata_schedule_52_0_e568).ln();
            let noise_metadata_schedule_52_0_e570: f64 = (noise_metadata_schedule_52_0_e562 * noise_metadata_schedule_52_0_e569);
            w[96] = noise_metadata_schedule_52_0_e570;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_53_0_e573: f64 = (w[96] * w[5]);
            let noise_metadata_schedule_53_0_e577: f64 = (1.0 - w[5]);
            let noise_metadata_schedule_53_0_e578: f64 = (w[174] * noise_metadata_schedule_53_0_e577);
            let noise_metadata_schedule_53_0_e579: f64 = (noise_metadata_schedule_53_0_e573 + noise_metadata_schedule_53_0_e578);
            let noise_metadata_schedule_53_0_e582: f64 = (w[168] * w[2]);
            let noise_metadata_schedule_53_0_e584: f64 = (noise_metadata_schedule_53_0_e582 * w[6]);
            let noise_metadata_schedule_53_0_e585: f64 = (noise_metadata_schedule_53_0_e579 - noise_metadata_schedule_53_0_e584);
            w[97] = noise_metadata_schedule_53_0_e585;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_54_0_e589: f64 = (2.0 * w[2]);
            let noise_metadata_schedule_54_0_e595: f64 = (-w[97]);
            let noise_metadata_schedule_54_0_e597: f64 = (noise_metadata_schedule_54_0_e595 * w[3]);
            let noise_metadata_schedule_54_0_e598: f64 = (noise_metadata_schedule_54_0_e597).exp();
            let noise_metadata_schedule_54_0_e599: f64 = (4.0 * noise_metadata_schedule_54_0_e598);
            let noise_metadata_schedule_54_0_e600: f64 = (1.0 + noise_metadata_schedule_54_0_e599);
            let noise_metadata_schedule_54_0_e601: f64 = (noise_metadata_schedule_54_0_e600).sqrt();
            let noise_metadata_schedule_54_0_e602: f64 = (1.0 + noise_metadata_schedule_54_0_e601);
            let noise_metadata_schedule_54_0_e603: f64 = (0.5 * noise_metadata_schedule_54_0_e602);
            let noise_metadata_schedule_54_0_e604: f64 = (noise_metadata_schedule_54_0_e603).ln();
            let noise_metadata_schedule_54_0_e605: f64 = (noise_metadata_schedule_54_0_e589 * noise_metadata_schedule_54_0_e604);
            let noise_metadata_schedule_54_0_e606: f64 = (w[97] + noise_metadata_schedule_54_0_e605);
            w[17] = noise_metadata_schedule_54_0_e606;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_55_0_e611: f64 = (params[42] / w[17]);
            let noise_metadata_schedule_55_0_e612: f64 = (noise_metadata_schedule_55_0_e611).ln();
            let noise_metadata_schedule_55_0_e613: f64 = (params[43] * noise_metadata_schedule_55_0_e612);
            let noise_metadata_schedule_55_0_e614: f64 = (noise_metadata_schedule_55_0_e613).exp();
            let noise_metadata_schedule_55_0_e615: f64 = (params[41] * noise_metadata_schedule_55_0_e614);
            w[24] = noise_metadata_schedule_55_0_e615;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_56_0_e619: f64 = (w[169] * w[6]);
            let noise_metadata_schedule_56_0_e622: f64 = (params[78] * w[10]);
            let noise_metadata_schedule_56_0_e623: f64 = (noise_metadata_schedule_56_0_e619 + noise_metadata_schedule_56_0_e622);
            let noise_metadata_schedule_56_0_e624: f64 = (noise_metadata_schedule_56_0_e623).exp();
            let noise_metadata_schedule_56_0_e625: f64 = (params[19] * noise_metadata_schedule_56_0_e624);
            w[14] = noise_metadata_schedule_56_0_e625;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_57_0_e629: f64 = (params[81] * w[6]);
            let noise_metadata_schedule_57_0_e632: f64 = (params[76] * w[10]);
            let noise_metadata_schedule_57_0_e633: f64 = (noise_metadata_schedule_57_0_e629 + noise_metadata_schedule_57_0_e632);
            let noise_metadata_schedule_57_0_e634: f64 = (noise_metadata_schedule_57_0_e633).exp();
            let noise_metadata_schedule_57_0_e635: f64 = (params[1] * noise_metadata_schedule_57_0_e634);
            w[11] = noise_metadata_schedule_57_0_e635;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_58_0_e639: f64 = (params[95] * w[6]);
            let noise_metadata_schedule_58_0_e642: f64 = (params[83] * w[10]);
            let noise_metadata_schedule_58_0_e643: f64 = (noise_metadata_schedule_58_0_e639 - noise_metadata_schedule_58_0_e642);
            let noise_metadata_schedule_58_0_e644: f64 = (noise_metadata_schedule_58_0_e643).exp();
            let noise_metadata_schedule_58_0_e645: f64 = (params[9] * noise_metadata_schedule_58_0_e644);
            w[15] = noise_metadata_schedule_58_0_e645;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_59_0_e649: f64 = (params[87] - w[172]);
            let noise_metadata_schedule_59_0_e651: f64 = (noise_metadata_schedule_59_0_e649 * w[6]);
            let noise_metadata_schedule_59_0_e652: f64 = (noise_metadata_schedule_59_0_e651).exp();
            let noise_metadata_schedule_59_0_e653: f64 = (params[62] * noise_metadata_schedule_59_0_e652);
            w[33] = noise_metadata_schedule_59_0_e653;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_60_0_e657: f64 = (params[87] * w[6]);
            let noise_metadata_schedule_60_0_e658: f64 = (noise_metadata_schedule_60_0_e657).exp();
            let noise_metadata_schedule_60_0_e659: f64 = (params[61] * noise_metadata_schedule_60_0_e658);
            w[31] = noise_metadata_schedule_60_0_e659;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_61_0_e662: f64 = (1.0 / w[31]);
            w[32] = noise_metadata_schedule_61_0_e662;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_62_0_e667: f64 = (params[89] * w[7]);
            let noise_metadata_schedule_62_0_e668: f64 = (1.0 + noise_metadata_schedule_62_0_e667);
            let noise_metadata_schedule_62_0_e669: f64 = (params[64] * noise_metadata_schedule_62_0_e668);
            w[34] = noise_metadata_schedule_62_0_e669;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_63_0_e672: f64 = if params[65] > 0.0 { 1.0 } else { 0.0 };
            w[249] = noise_metadata_schedule_63_0_e672;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_64_0_e682,) = {
    if (w[249] != 0.0) {
        let noise_metadata_schedule_64_0_e678: f64 = (params[90] * w[7]);
        let noise_metadata_schedule_64_0_e679: f64 = (1.0 - noise_metadata_schedule_64_0_e678);
        let noise_metadata_schedule_64_0_e680: f64 = (params[65] * noise_metadata_schedule_64_0_e679);
        (noise_metadata_schedule_64_0_e680,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_64_0_e682;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_65_0_e686,) = {
    if (w[249] != 0.0) {
        (params[64],)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_65_0_e686;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_66_0_e697,) = {
    if (w[249] == 0.0) {
        let noise_metadata_schedule_66_0_e693: f64 = (params[89] * w[7]);
        let noise_metadata_schedule_66_0_e694: f64 = (1.0 + noise_metadata_schedule_66_0_e693);
        let noise_metadata_schedule_66_0_e695: f64 = (params[64] * noise_metadata_schedule_66_0_e694);
        (noise_metadata_schedule_66_0_e695,)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_66_0_e697;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_67_0_e702,) = {
    if (w[249] == 0.0) {
        (params[65],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_67_0_e702;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_68_0_e707: f64 = (params[85] * w[7]);
            let noise_metadata_schedule_68_0_e708: f64 = (1.0 + noise_metadata_schedule_68_0_e707);
            let noise_metadata_schedule_68_0_e711: f64 = (params[86] * w[7]);
            let noise_metadata_schedule_68_0_e713: f64 = (noise_metadata_schedule_68_0_e711 * w[7]);
            let noise_metadata_schedule_68_0_e714: f64 = (noise_metadata_schedule_68_0_e708 + noise_metadata_schedule_68_0_e713);
            let noise_metadata_schedule_68_0_e715: f64 = (params[54] * noise_metadata_schedule_68_0_e714);
            w[42] = noise_metadata_schedule_68_0_e715;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_78_0_e779: f64 = (params[91] * w[6]);
            let noise_metadata_schedule_78_0_e780: f64 = (noise_metadata_schedule_78_0_e779).exp();
            let noise_metadata_schedule_78_0_e781: f64 = (params[23] * noise_metadata_schedule_78_0_e780);
            w[37] = noise_metadata_schedule_78_0_e781;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_79_0_e784: f64 = (0.5 * params[46]);
            let noise_metadata_schedule_79_0_e786: f64 = (noise_metadata_schedule_79_0_e784 / w[177]);
            w[178] = noise_metadata_schedule_79_0_e786;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_80_0_e789: f64 = (2.0 * w[177]);
            let noise_metadata_schedule_80_0_e791: f64 = (w[178]).exp();
            let noise_metadata_schedule_80_0_e793: f64 = (-w[178]);
            let noise_metadata_schedule_80_0_e794: f64 = (noise_metadata_schedule_80_0_e793).exp();
            let noise_metadata_schedule_80_0_e795: f64 = (noise_metadata_schedule_80_0_e791 - noise_metadata_schedule_80_0_e794);
            let noise_metadata_schedule_80_0_e796: f64 = (noise_metadata_schedule_80_0_e795).ln();
            let noise_metadata_schedule_80_0_e797: f64 = (noise_metadata_schedule_80_0_e789 * noise_metadata_schedule_80_0_e796);
            w[96] = noise_metadata_schedule_80_0_e797;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_81_0_e800: f64 = (w[96] * w[5]);
            let noise_metadata_schedule_81_0_e804: f64 = (1.0 - w[5]);
            let noise_metadata_schedule_81_0_e805: f64 = (w[174] * noise_metadata_schedule_81_0_e804);
            let noise_metadata_schedule_81_0_e806: f64 = (noise_metadata_schedule_81_0_e800 + noise_metadata_schedule_81_0_e805);
            let noise_metadata_schedule_81_0_e809: f64 = (w[168] * w[2]);
            let noise_metadata_schedule_81_0_e811: f64 = (noise_metadata_schedule_81_0_e809 * w[6]);
            let noise_metadata_schedule_81_0_e812: f64 = (noise_metadata_schedule_81_0_e806 - noise_metadata_schedule_81_0_e811);
            w[97] = noise_metadata_schedule_81_0_e812;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_82_0_e816: f64 = (2.0 * w[2]);
            let noise_metadata_schedule_82_0_e822: f64 = (-w[97]);
            let noise_metadata_schedule_82_0_e824: f64 = (noise_metadata_schedule_82_0_e822 * w[3]);
            let noise_metadata_schedule_82_0_e825: f64 = (noise_metadata_schedule_82_0_e824).exp();
            let noise_metadata_schedule_82_0_e826: f64 = (4.0 * noise_metadata_schedule_82_0_e825);
            let noise_metadata_schedule_82_0_e827: f64 = (1.0 + noise_metadata_schedule_82_0_e826);
            let noise_metadata_schedule_82_0_e828: f64 = (noise_metadata_schedule_82_0_e827).sqrt();
            let noise_metadata_schedule_82_0_e829: f64 = (1.0 + noise_metadata_schedule_82_0_e828);
            let noise_metadata_schedule_82_0_e830: f64 = (0.5 * noise_metadata_schedule_82_0_e829);
            let noise_metadata_schedule_82_0_e831: f64 = (noise_metadata_schedule_82_0_e830).ln();
            let noise_metadata_schedule_82_0_e832: f64 = (noise_metadata_schedule_82_0_e816 * noise_metadata_schedule_82_0_e831);
            let noise_metadata_schedule_82_0_e833: f64 = (w[97] + noise_metadata_schedule_82_0_e832);
            w[18] = noise_metadata_schedule_82_0_e833;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_83_0_e838: f64 = (params[46] / w[18]);
            let noise_metadata_schedule_83_0_e839: f64 = (noise_metadata_schedule_83_0_e838).ln();
            let noise_metadata_schedule_83_0_e840: f64 = (params[47] * noise_metadata_schedule_83_0_e839);
            let noise_metadata_schedule_83_0_e841: f64 = (noise_metadata_schedule_83_0_e840).exp();
            let noise_metadata_schedule_83_0_e842: f64 = (params[45] * noise_metadata_schedule_83_0_e841);
            w[25] = noise_metadata_schedule_83_0_e842;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_84_0_e845: f64 = (0.5 * params[51]);
            let noise_metadata_schedule_84_0_e847: f64 = (noise_metadata_schedule_84_0_e845 / w[177]);
            w[178] = noise_metadata_schedule_84_0_e847;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_85_0_e850: f64 = (2.0 * w[177]);
            let noise_metadata_schedule_85_0_e852: f64 = (w[178]).exp();
            let noise_metadata_schedule_85_0_e854: f64 = (-w[178]);
            let noise_metadata_schedule_85_0_e855: f64 = (noise_metadata_schedule_85_0_e854).exp();
            let noise_metadata_schedule_85_0_e856: f64 = (noise_metadata_schedule_85_0_e852 - noise_metadata_schedule_85_0_e855);
            let noise_metadata_schedule_85_0_e857: f64 = (noise_metadata_schedule_85_0_e856).ln();
            let noise_metadata_schedule_85_0_e858: f64 = (noise_metadata_schedule_85_0_e850 * noise_metadata_schedule_85_0_e857);
            w[96] = noise_metadata_schedule_85_0_e858;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_86_0_e861: f64 = (w[96] * w[5]);
            let noise_metadata_schedule_86_0_e865: f64 = (1.0 - w[5]);
            let noise_metadata_schedule_86_0_e866: f64 = (w[175] * noise_metadata_schedule_86_0_e865);
            let noise_metadata_schedule_86_0_e867: f64 = (noise_metadata_schedule_86_0_e861 + noise_metadata_schedule_86_0_e866);
            let noise_metadata_schedule_86_0_e870: f64 = (w[168] * w[2]);
            let noise_metadata_schedule_86_0_e872: f64 = (noise_metadata_schedule_86_0_e870 * w[6]);
            let noise_metadata_schedule_86_0_e873: f64 = (noise_metadata_schedule_86_0_e867 - noise_metadata_schedule_86_0_e872);
            w[97] = noise_metadata_schedule_86_0_e873;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_91_0_e927: f64 = (params[97] * w[6]);
            let noise_metadata_schedule_91_0_e928: f64 = (noise_metadata_schedule_91_0_e927).exp();
            let noise_metadata_schedule_91_0_e929: f64 = (params[7] * noise_metadata_schedule_91_0_e928);
            w[200] = noise_metadata_schedule_91_0_e929;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_92_0_e933: f64 = (params[83] * w[3]);
            let noise_metadata_schedule_92_0_e936: f64 = (params[84] * w[6]);
            let noise_metadata_schedule_92_0_e937: f64 = (noise_metadata_schedule_92_0_e936).exp();
            let noise_metadata_schedule_92_0_e939: f64 = (noise_metadata_schedule_92_0_e937 - 1.0);
            let noise_metadata_schedule_92_0_e940: f64 = (noise_metadata_schedule_92_0_e933 * noise_metadata_schedule_92_0_e939);
            let noise_metadata_schedule_92_0_e941: f64 = (noise_metadata_schedule_92_0_e940).exp();
            let noise_metadata_schedule_92_0_e942: f64 = (params[6] / noise_metadata_schedule_92_0_e941);
            w[202] = noise_metadata_schedule_92_0_e942;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_93_0_e945: f64 = if params[0] <= 200.0 { 1.0 } else { 0.0 };
            w[252] = noise_metadata_schedule_93_0_e945;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_94_0_e957,) = {
    if (w[252] != 0.0) {
        let noise_metadata_schedule_94_0_e952: f64 = (params[102] * w[7]);
        let noise_metadata_schedule_94_0_e953: f64 = (params[101] + noise_metadata_schedule_94_0_e952);
        let noise_metadata_schedule_94_0_e954: f64 = (w[7] * noise_metadata_schedule_94_0_e953);
        let noise_metadata_schedule_94_0_e955: f64 = (1.0 + noise_metadata_schedule_94_0_e954);
        (noise_metadata_schedule_94_0_e955,)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_94_0_e957;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_95_0_e965,) = {
    if (w[252] == 0.0) {
        let noise_metadata_schedule_95_0_e962: f64 = (params[98] * w[6]);
        let noise_metadata_schedule_95_0_e963: f64 = (noise_metadata_schedule_95_0_e962).exp();
        (noise_metadata_schedule_95_0_e963,)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_95_0_e965;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_96_0_e968: f64 = (params[12] * w[204]);
            w[203] = noise_metadata_schedule_96_0_e968;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_97_0_e971: f64 = (params[13] * w[204]);
            let noise_metadata_schedule_97_0_e974: f64 = (w[176] * w[10]);
            let noise_metadata_schedule_97_0_e975: f64 = (noise_metadata_schedule_97_0_e974).exp();
            let noise_metadata_schedule_97_0_e976: f64 = (noise_metadata_schedule_97_0_e971 * noise_metadata_schedule_97_0_e975);
            w[205] = noise_metadata_schedule_97_0_e976;
        }
        if (active[0] & 0x21) != 0 {
            w[206] = params[14];
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_99_0_e981: f64 = (params[93] * w[6]);
            let noise_metadata_schedule_99_0_e982: f64 = (noise_metadata_schedule_99_0_e981).exp();
            let noise_metadata_schedule_99_0_e983: f64 = (params[29] * noise_metadata_schedule_99_0_e982);
            w[40] = noise_metadata_schedule_99_0_e983;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_100_0_e987: f64 = (params[92] * w[6]);
            let noise_metadata_schedule_100_0_e988: f64 = (noise_metadata_schedule_100_0_e987).exp();
            let noise_metadata_schedule_100_0_e989: f64 = (params[26] * noise_metadata_schedule_100_0_e988);
            w[39] = noise_metadata_schedule_100_0_e989;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_101_0_e993: f64 = (params[94] * w[6]);
            let noise_metadata_schedule_101_0_e994: f64 = (noise_metadata_schedule_101_0_e993).exp();
            let noise_metadata_schedule_101_0_e995: f64 = (params[28] * noise_metadata_schedule_101_0_e994);
            w[41] = noise_metadata_schedule_101_0_e995;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_103_0_e1014: f64 = if ((params[103] != 0.0) && (params[104] >= params[111])) { 1.0 } else { 0.0 };
            w[253] = noise_metadata_schedule_103_0_e1014;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_104_0_e1022,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_104_0_e1018: f64 = (w[9] + params[109]);
        let noise_metadata_schedule_104_0_e1020: f64 = (noise_metadata_schedule_104_0_e1018 + (ctx.node_voltage(self.nodes[4]) - 0.0));
        (noise_metadata_schedule_104_0_e1020,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_104_0_e1022;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_105_0_e1025: f64 = (-100.0);
            let noise_metadata_schedule_105_0_e1027: f64 = (noise_metadata_schedule_105_0_e1025 + 273.15);
            let noise_metadata_schedule_105_0_e1028: f64 = if w[4] < noise_metadata_schedule_105_0_e1027 { 1.0 } else { 0.0 };
            w[254] = noise_metadata_schedule_105_0_e1028;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_106_0_e1037,) = {
    if ((w[253] != 0.0) && (w[254] != 0.0)) {
        let noise_metadata_schedule_106_0_e1033: f64 = (-100.0);
        let noise_metadata_schedule_106_0_e1035: f64 = (noise_metadata_schedule_106_0_e1033 + 273.15);
        (noise_metadata_schedule_106_0_e1035,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_106_0_e1037;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_107_0_e1041: f64 = (326.85 + 273.15);
            let noise_metadata_schedule_107_0_e1042: f64 = if w[4] > noise_metadata_schedule_107_0_e1041 { 1.0 } else { 0.0 };
            w[255] = noise_metadata_schedule_107_0_e1042;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_108_0_e1053,) = {
    if (((w[253] != 0.0) && (w[254] == 0.0)) && (w[255] != 0.0)) {
        let noise_metadata_schedule_108_0_e1051: f64 = (326.85 + 273.15);
        (noise_metadata_schedule_108_0_e1051,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_108_0_e1053;
        }
        if (active[0] & 0x39) != 0 {
            let (noise_metadata_schedule_109_0_e1061,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_109_0_e1057: f64 = (1.3806226e-23 * w[4]);
        let noise_metadata_schedule_109_0_e1059: f64 = (noise_metadata_schedule_109_0_e1057 / 1.602176462e-19);
        (noise_metadata_schedule_109_0_e1059,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_109_0_e1061;
        }
        if (active[0] & 0x39) != 0 {
            let (noise_metadata_schedule_110_0_e1067,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_110_0_e1065: f64 = (1.0 / w[2]);
        (noise_metadata_schedule_110_0_e1065,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_110_0_e1067;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_111_0_e1073,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_111_0_e1071: f64 = (w[4] - w[8]);
        (noise_metadata_schedule_111_0_e1071,)
    } else {
        (w[7],)
    }
};
            w[7] = noise_metadata_schedule_111_0_e1073;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_112_0_e1079,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_112_0_e1077: f64 = (w[4] / w[8]);
        (noise_metadata_schedule_112_0_e1077,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_112_0_e1079;
        }
        if (active[0] & 0x3f) != 0 {
            let (noise_metadata_schedule_113_0_e1084,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_113_0_e1082: f64 = (w[5]).ln();
        (noise_metadata_schedule_113_0_e1082,)
    } else {
        (w[6],)
    }
};
            w[6] = noise_metadata_schedule_113_0_e1084;
        }
        if (active[0] & 0x39) != 0 {
            let (noise_metadata_schedule_114_0_e1092,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_114_0_e1089: f64 = (w[5] - 1.0);
        let noise_metadata_schedule_114_0_e1090: f64 = (w[3] * noise_metadata_schedule_114_0_e1089);
        (noise_metadata_schedule_114_0_e1090,)
    } else {
        (w[10],)
    }
};
            w[10] = noise_metadata_schedule_114_0_e1092;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_115_0_e1100,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_115_0_e1096: f64 = (0.5 * params[35]);
        let noise_metadata_schedule_115_0_e1098: f64 = (noise_metadata_schedule_115_0_e1096 / w[177]);
        (noise_metadata_schedule_115_0_e1098,)
    } else {
        (w[178],)
    }
};
            w[178] = noise_metadata_schedule_115_0_e1100;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_116_0_e1114,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_116_0_e1104: f64 = (2.0 * w[177]);
        let noise_metadata_schedule_116_0_e1106: f64 = (w[178]).exp();
        let noise_metadata_schedule_116_0_e1108: f64 = (-w[178]);
        let noise_metadata_schedule_116_0_e1109: f64 = (noise_metadata_schedule_116_0_e1108).exp();
        let noise_metadata_schedule_116_0_e1110: f64 = (noise_metadata_schedule_116_0_e1106 - noise_metadata_schedule_116_0_e1109);
        let noise_metadata_schedule_116_0_e1111: f64 = (noise_metadata_schedule_116_0_e1110).ln();
        let noise_metadata_schedule_116_0_e1112: f64 = (noise_metadata_schedule_116_0_e1104 * noise_metadata_schedule_116_0_e1111);
        (noise_metadata_schedule_116_0_e1112,)
    } else {
        (w[96],)
    }
};
            w[96] = noise_metadata_schedule_116_0_e1114;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_117_0_e1132,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_117_0_e1118: f64 = (w[96] * w[5]);
        let noise_metadata_schedule_117_0_e1122: f64 = (1.0 - w[5]);
        let noise_metadata_schedule_117_0_e1123: f64 = (w[173] * noise_metadata_schedule_117_0_e1122);
        let noise_metadata_schedule_117_0_e1124: f64 = (noise_metadata_schedule_117_0_e1118 + noise_metadata_schedule_117_0_e1123);
        let noise_metadata_schedule_117_0_e1127: f64 = (w[168] * w[2]);
        let noise_metadata_schedule_117_0_e1129: f64 = (noise_metadata_schedule_117_0_e1127 * w[6]);
        let noise_metadata_schedule_117_0_e1130: f64 = (noise_metadata_schedule_117_0_e1124 - noise_metadata_schedule_117_0_e1129);
        (noise_metadata_schedule_117_0_e1130,)
    } else {
        (w[97],)
    }
};
            w[97] = noise_metadata_schedule_117_0_e1132;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_118_0_e1156,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_118_0_e1137: f64 = (2.0 * w[2]);
        let noise_metadata_schedule_118_0_e1143: f64 = (-w[97]);
        let noise_metadata_schedule_118_0_e1145: f64 = (noise_metadata_schedule_118_0_e1143 * w[3]);
        let noise_metadata_schedule_118_0_e1146: f64 = (noise_metadata_schedule_118_0_e1145).exp();
        let noise_metadata_schedule_118_0_e1147: f64 = (4.0 * noise_metadata_schedule_118_0_e1146);
        let noise_metadata_schedule_118_0_e1148: f64 = (1.0 + noise_metadata_schedule_118_0_e1147);
        let noise_metadata_schedule_118_0_e1149: f64 = (noise_metadata_schedule_118_0_e1148).sqrt();
        let noise_metadata_schedule_118_0_e1150: f64 = (1.0 + noise_metadata_schedule_118_0_e1149);
        let noise_metadata_schedule_118_0_e1151: f64 = (0.5 * noise_metadata_schedule_118_0_e1150);
        let noise_metadata_schedule_118_0_e1152: f64 = (noise_metadata_schedule_118_0_e1151).ln();
        let noise_metadata_schedule_118_0_e1153: f64 = (noise_metadata_schedule_118_0_e1137 * noise_metadata_schedule_118_0_e1152);
        let noise_metadata_schedule_118_0_e1154: f64 = (w[97] + noise_metadata_schedule_118_0_e1153);
        (noise_metadata_schedule_118_0_e1154,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_118_0_e1156;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_119_0_e1168,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_119_0_e1162: f64 = (params[35] / w[16]);
        let noise_metadata_schedule_119_0_e1163: f64 = (noise_metadata_schedule_119_0_e1162).ln();
        let noise_metadata_schedule_119_0_e1164: f64 = (params[36] * noise_metadata_schedule_119_0_e1163);
        let noise_metadata_schedule_119_0_e1165: f64 = (noise_metadata_schedule_119_0_e1164).exp();
        let noise_metadata_schedule_119_0_e1166: f64 = (params[34] * noise_metadata_schedule_119_0_e1165);
        (noise_metadata_schedule_119_0_e1166,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_119_0_e1168;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_120_0_e1176,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_120_0_e1172: f64 = (params[37] * w[16]);
        let noise_metadata_schedule_120_0_e1174: f64 = (noise_metadata_schedule_120_0_e1172 / params[35]);
        (noise_metadata_schedule_120_0_e1174,)
    } else {
        (w[43],)
    }
};
            w[43] = noise_metadata_schedule_120_0_e1176;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_121_0_e1184,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_121_0_e1180: f64 = (0.5 * params[38]);
        let noise_metadata_schedule_121_0_e1182: f64 = (noise_metadata_schedule_121_0_e1180 / w[177]);
        (noise_metadata_schedule_121_0_e1182,)
    } else {
        (w[178],)
    }
};
            w[178] = noise_metadata_schedule_121_0_e1184;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_122_0_e1198,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_122_0_e1188: f64 = (2.0 * w[177]);
        let noise_metadata_schedule_122_0_e1190: f64 = (w[178]).exp();
        let noise_metadata_schedule_122_0_e1192: f64 = (-w[178]);
        let noise_metadata_schedule_122_0_e1193: f64 = (noise_metadata_schedule_122_0_e1192).exp();
        let noise_metadata_schedule_122_0_e1194: f64 = (noise_metadata_schedule_122_0_e1190 - noise_metadata_schedule_122_0_e1193);
        let noise_metadata_schedule_122_0_e1195: f64 = (noise_metadata_schedule_122_0_e1194).ln();
        let noise_metadata_schedule_122_0_e1196: f64 = (noise_metadata_schedule_122_0_e1188 * noise_metadata_schedule_122_0_e1195);
        (noise_metadata_schedule_122_0_e1196,)
    } else {
        (w[96],)
    }
};
            w[96] = noise_metadata_schedule_122_0_e1198;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_123_0_e1216,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_123_0_e1202: f64 = (w[96] * w[5]);
        let noise_metadata_schedule_123_0_e1206: f64 = (1.0 - w[5]);
        let noise_metadata_schedule_123_0_e1207: f64 = (w[173] * noise_metadata_schedule_123_0_e1206);
        let noise_metadata_schedule_123_0_e1208: f64 = (noise_metadata_schedule_123_0_e1202 + noise_metadata_schedule_123_0_e1207);
        let noise_metadata_schedule_123_0_e1211: f64 = (w[168] * w[2]);
        let noise_metadata_schedule_123_0_e1213: f64 = (noise_metadata_schedule_123_0_e1211 * w[6]);
        let noise_metadata_schedule_123_0_e1214: f64 = (noise_metadata_schedule_123_0_e1208 - noise_metadata_schedule_123_0_e1213);
        (noise_metadata_schedule_123_0_e1214,)
    } else {
        (w[97],)
    }
};
            w[97] = noise_metadata_schedule_123_0_e1216;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_124_0_e1240,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_124_0_e1221: f64 = (2.0 * w[2]);
        let noise_metadata_schedule_124_0_e1227: f64 = (-w[97]);
        let noise_metadata_schedule_124_0_e1229: f64 = (noise_metadata_schedule_124_0_e1227 * w[3]);
        let noise_metadata_schedule_124_0_e1230: f64 = (noise_metadata_schedule_124_0_e1229).exp();
        let noise_metadata_schedule_124_0_e1231: f64 = (4.0 * noise_metadata_schedule_124_0_e1230);
        let noise_metadata_schedule_124_0_e1232: f64 = (1.0 + noise_metadata_schedule_124_0_e1231);
        let noise_metadata_schedule_124_0_e1233: f64 = (noise_metadata_schedule_124_0_e1232).sqrt();
        let noise_metadata_schedule_124_0_e1234: f64 = (1.0 + noise_metadata_schedule_124_0_e1233);
        let noise_metadata_schedule_124_0_e1235: f64 = (0.5 * noise_metadata_schedule_124_0_e1234);
        let noise_metadata_schedule_124_0_e1236: f64 = (noise_metadata_schedule_124_0_e1235).ln();
        let noise_metadata_schedule_124_0_e1237: f64 = (noise_metadata_schedule_124_0_e1221 * noise_metadata_schedule_124_0_e1236);
        let noise_metadata_schedule_124_0_e1238: f64 = (w[97] + noise_metadata_schedule_124_0_e1237);
        (noise_metadata_schedule_124_0_e1238,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_124_0_e1240;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_125_0_e1252,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_125_0_e1246: f64 = (params[38] / w[22]);
        let noise_metadata_schedule_125_0_e1247: f64 = (noise_metadata_schedule_125_0_e1246).ln();
        let noise_metadata_schedule_125_0_e1248: f64 = (params[39] * noise_metadata_schedule_125_0_e1247);
        let noise_metadata_schedule_125_0_e1249: f64 = (noise_metadata_schedule_125_0_e1248).exp();
        let noise_metadata_schedule_125_0_e1250: f64 = (w[27] * noise_metadata_schedule_125_0_e1249);
        (noise_metadata_schedule_125_0_e1250,)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_125_0_e1252;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_126_0_e1260,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_126_0_e1256: f64 = (params[40] * w[22]);
        let noise_metadata_schedule_126_0_e1258: f64 = (noise_metadata_schedule_126_0_e1256 / params[38]);
        (noise_metadata_schedule_126_0_e1258,)
    } else {
        (w[44],)
    }
};
            w[44] = noise_metadata_schedule_126_0_e1260;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_127_0_e1273,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_127_0_e1265: f64 = (params[82] * w[6]);
        let noise_metadata_schedule_127_0_e1268: f64 = (params[77] * w[10]);
        let noise_metadata_schedule_127_0_e1269: f64 = (noise_metadata_schedule_127_0_e1265 + noise_metadata_schedule_127_0_e1268);
        let noise_metadata_schedule_127_0_e1270: f64 = (noise_metadata_schedule_127_0_e1269).exp();
        let noise_metadata_schedule_127_0_e1271: f64 = (params[15] * noise_metadata_schedule_127_0_e1270);
        (noise_metadata_schedule_127_0_e1271,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_127_0_e1273;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_128_0_e1290,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_128_0_e1278: f64 = (0.5 * w[168]);
        let noise_metadata_schedule_128_0_e1280: f64 = (noise_metadata_schedule_128_0_e1278 * w[6]);
        let noise_metadata_schedule_128_0_e1283: f64 = (0.5 * w[173]);
        let noise_metadata_schedule_128_0_e1285: f64 = (noise_metadata_schedule_128_0_e1283 * w[10]);
        let noise_metadata_schedule_128_0_e1286: f64 = (noise_metadata_schedule_128_0_e1280 + noise_metadata_schedule_128_0_e1285);
        let noise_metadata_schedule_128_0_e1287: f64 = (noise_metadata_schedule_128_0_e1286).exp();
        let noise_metadata_schedule_128_0_e1288: f64 = (params[17] * noise_metadata_schedule_128_0_e1287);
        (noise_metadata_schedule_128_0_e1288,)
    } else {
        (w[12],)
    }
};
            w[12] = noise_metadata_schedule_128_0_e1290;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_129_0_e1298,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_129_0_e1294: f64 = (0.5 * params[42]);
        let noise_metadata_schedule_129_0_e1296: f64 = (noise_metadata_schedule_129_0_e1294 / w[177]);
        (noise_metadata_schedule_129_0_e1296,)
    } else {
        (w[178],)
    }
};
            w[178] = noise_metadata_schedule_129_0_e1298;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_130_0_e1312,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_130_0_e1302: f64 = (2.0 * w[177]);
        let noise_metadata_schedule_130_0_e1304: f64 = (w[178]).exp();
        let noise_metadata_schedule_130_0_e1306: f64 = (-w[178]);
        let noise_metadata_schedule_130_0_e1307: f64 = (noise_metadata_schedule_130_0_e1306).exp();
        let noise_metadata_schedule_130_0_e1308: f64 = (noise_metadata_schedule_130_0_e1304 - noise_metadata_schedule_130_0_e1307);
        let noise_metadata_schedule_130_0_e1309: f64 = (noise_metadata_schedule_130_0_e1308).ln();
        let noise_metadata_schedule_130_0_e1310: f64 = (noise_metadata_schedule_130_0_e1302 * noise_metadata_schedule_130_0_e1309);
        (noise_metadata_schedule_130_0_e1310,)
    } else {
        (w[96],)
    }
};
            w[96] = noise_metadata_schedule_130_0_e1312;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_131_0_e1330,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_131_0_e1316: f64 = (w[96] * w[5]);
        let noise_metadata_schedule_131_0_e1320: f64 = (1.0 - w[5]);
        let noise_metadata_schedule_131_0_e1321: f64 = (w[174] * noise_metadata_schedule_131_0_e1320);
        let noise_metadata_schedule_131_0_e1322: f64 = (noise_metadata_schedule_131_0_e1316 + noise_metadata_schedule_131_0_e1321);
        let noise_metadata_schedule_131_0_e1325: f64 = (w[168] * w[2]);
        let noise_metadata_schedule_131_0_e1327: f64 = (noise_metadata_schedule_131_0_e1325 * w[6]);
        let noise_metadata_schedule_131_0_e1328: f64 = (noise_metadata_schedule_131_0_e1322 - noise_metadata_schedule_131_0_e1327);
        (noise_metadata_schedule_131_0_e1328,)
    } else {
        (w[97],)
    }
};
            w[97] = noise_metadata_schedule_131_0_e1330;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_132_0_e1354,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_132_0_e1335: f64 = (2.0 * w[2]);
        let noise_metadata_schedule_132_0_e1341: f64 = (-w[97]);
        let noise_metadata_schedule_132_0_e1343: f64 = (noise_metadata_schedule_132_0_e1341 * w[3]);
        let noise_metadata_schedule_132_0_e1344: f64 = (noise_metadata_schedule_132_0_e1343).exp();
        let noise_metadata_schedule_132_0_e1345: f64 = (4.0 * noise_metadata_schedule_132_0_e1344);
        let noise_metadata_schedule_132_0_e1346: f64 = (1.0 + noise_metadata_schedule_132_0_e1345);
        let noise_metadata_schedule_132_0_e1347: f64 = (noise_metadata_schedule_132_0_e1346).sqrt();
        let noise_metadata_schedule_132_0_e1348: f64 = (1.0 + noise_metadata_schedule_132_0_e1347);
        let noise_metadata_schedule_132_0_e1349: f64 = (0.5 * noise_metadata_schedule_132_0_e1348);
        let noise_metadata_schedule_132_0_e1350: f64 = (noise_metadata_schedule_132_0_e1349).ln();
        let noise_metadata_schedule_132_0_e1351: f64 = (noise_metadata_schedule_132_0_e1335 * noise_metadata_schedule_132_0_e1350);
        let noise_metadata_schedule_132_0_e1352: f64 = (w[97] + noise_metadata_schedule_132_0_e1351);
        (noise_metadata_schedule_132_0_e1352,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_132_0_e1354;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_133_0_e1366,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_133_0_e1360: f64 = (params[42] / w[17]);
        let noise_metadata_schedule_133_0_e1361: f64 = (noise_metadata_schedule_133_0_e1360).ln();
        let noise_metadata_schedule_133_0_e1362: f64 = (params[43] * noise_metadata_schedule_133_0_e1361);
        let noise_metadata_schedule_133_0_e1363: f64 = (noise_metadata_schedule_133_0_e1362).exp();
        let noise_metadata_schedule_133_0_e1364: f64 = (params[41] * noise_metadata_schedule_133_0_e1363);
        (noise_metadata_schedule_133_0_e1364,)
    } else {
        (w[24],)
    }
};
            w[24] = noise_metadata_schedule_133_0_e1366;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_134_0_e1379,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_134_0_e1371: f64 = (w[169] * w[6]);
        let noise_metadata_schedule_134_0_e1374: f64 = (params[78] * w[10]);
        let noise_metadata_schedule_134_0_e1375: f64 = (noise_metadata_schedule_134_0_e1371 + noise_metadata_schedule_134_0_e1374);
        let noise_metadata_schedule_134_0_e1376: f64 = (noise_metadata_schedule_134_0_e1375).exp();
        let noise_metadata_schedule_134_0_e1377: f64 = (params[19] * noise_metadata_schedule_134_0_e1376);
        (noise_metadata_schedule_134_0_e1377,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_134_0_e1379;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_135_0_e1392,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_135_0_e1384: f64 = (params[81] * w[6]);
        let noise_metadata_schedule_135_0_e1387: f64 = (params[76] * w[10]);
        let noise_metadata_schedule_135_0_e1388: f64 = (noise_metadata_schedule_135_0_e1384 + noise_metadata_schedule_135_0_e1387);
        let noise_metadata_schedule_135_0_e1389: f64 = (noise_metadata_schedule_135_0_e1388).exp();
        let noise_metadata_schedule_135_0_e1390: f64 = (params[1] * noise_metadata_schedule_135_0_e1389);
        (noise_metadata_schedule_135_0_e1390,)
    } else {
        (w[11],)
    }
};
            w[11] = noise_metadata_schedule_135_0_e1392;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_136_0_e1405,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_136_0_e1397: f64 = (params[95] * w[6]);
        let noise_metadata_schedule_136_0_e1400: f64 = (params[83] * w[10]);
        let noise_metadata_schedule_136_0_e1401: f64 = (noise_metadata_schedule_136_0_e1397 - noise_metadata_schedule_136_0_e1400);
        let noise_metadata_schedule_136_0_e1402: f64 = (noise_metadata_schedule_136_0_e1401).exp();
        let noise_metadata_schedule_136_0_e1403: f64 = (params[9] * noise_metadata_schedule_136_0_e1402);
        (noise_metadata_schedule_136_0_e1403,)
    } else {
        (w[15],)
    }
};
            w[15] = noise_metadata_schedule_136_0_e1405;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_137_0_e1416,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_137_0_e1410: f64 = (params[87] - w[172]);
        let noise_metadata_schedule_137_0_e1412: f64 = (noise_metadata_schedule_137_0_e1410 * w[6]);
        let noise_metadata_schedule_137_0_e1413: f64 = (noise_metadata_schedule_137_0_e1412).exp();
        let noise_metadata_schedule_137_0_e1414: f64 = (params[62] * noise_metadata_schedule_137_0_e1413);
        (noise_metadata_schedule_137_0_e1414,)
    } else {
        (w[33],)
    }
};
            w[33] = noise_metadata_schedule_137_0_e1416;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_138_0_e1425,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_138_0_e1421: f64 = (params[87] * w[6]);
        let noise_metadata_schedule_138_0_e1422: f64 = (noise_metadata_schedule_138_0_e1421).exp();
        let noise_metadata_schedule_138_0_e1423: f64 = (params[61] * noise_metadata_schedule_138_0_e1422);
        (noise_metadata_schedule_138_0_e1423,)
    } else {
        (w[31],)
    }
};
            w[31] = noise_metadata_schedule_138_0_e1425;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_139_0_e1431,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_139_0_e1429: f64 = (1.0 / w[31]);
        (noise_metadata_schedule_139_0_e1429,)
    } else {
        (w[32],)
    }
};
            w[32] = noise_metadata_schedule_139_0_e1431;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_140_0_e1441,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_140_0_e1437: f64 = (params[89] * w[7]);
        let noise_metadata_schedule_140_0_e1438: f64 = (1.0 + noise_metadata_schedule_140_0_e1437);
        let noise_metadata_schedule_140_0_e1439: f64 = (params[64] * noise_metadata_schedule_140_0_e1438);
        (noise_metadata_schedule_140_0_e1439,)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_140_0_e1441;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_141_0_e1444: f64 = if params[65] > 0.0 { 1.0 } else { 0.0 };
            w[256] = noise_metadata_schedule_141_0_e1444;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_142_0_e1456,) = {
    if ((w[253] != 0.0) && (w[256] != 0.0)) {
        let noise_metadata_schedule_142_0_e1452: f64 = (params[90] * w[7]);
        let noise_metadata_schedule_142_0_e1453: f64 = (1.0 - noise_metadata_schedule_142_0_e1452);
        let noise_metadata_schedule_142_0_e1454: f64 = (params[65] * noise_metadata_schedule_142_0_e1453);
        (noise_metadata_schedule_142_0_e1454,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_142_0_e1456;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_143_0_e1462,) = {
    if ((w[253] != 0.0) && (w[256] != 0.0)) {
        (params[64],)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_143_0_e1462;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_144_0_e1475,) = {
    if ((w[253] != 0.0) && (w[256] == 0.0)) {
        let noise_metadata_schedule_144_0_e1471: f64 = (params[89] * w[7]);
        let noise_metadata_schedule_144_0_e1472: f64 = (1.0 + noise_metadata_schedule_144_0_e1471);
        let noise_metadata_schedule_144_0_e1473: f64 = (params[64] * noise_metadata_schedule_144_0_e1472);
        (noise_metadata_schedule_144_0_e1473,)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_144_0_e1475;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_145_0_e1482,) = {
    if ((w[253] != 0.0) && (w[256] == 0.0)) {
        (params[65],)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_145_0_e1482;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_146_0_e1498,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_146_0_e1488: f64 = (params[85] * w[7]);
        let noise_metadata_schedule_146_0_e1489: f64 = (1.0 + noise_metadata_schedule_146_0_e1488);
        let noise_metadata_schedule_146_0_e1492: f64 = (params[86] * w[7]);
        let noise_metadata_schedule_146_0_e1494: f64 = (noise_metadata_schedule_146_0_e1492 * w[7]);
        let noise_metadata_schedule_146_0_e1495: f64 = (noise_metadata_schedule_146_0_e1489 + noise_metadata_schedule_146_0_e1494);
        let noise_metadata_schedule_146_0_e1496: f64 = (params[54] * noise_metadata_schedule_146_0_e1495);
        (noise_metadata_schedule_146_0_e1496,)
    } else {
        (w[42],)
    }
};
            w[42] = noise_metadata_schedule_146_0_e1498;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_156_0_e1582,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_156_0_e1578: f64 = (params[91] * w[6]);
        let noise_metadata_schedule_156_0_e1579: f64 = (noise_metadata_schedule_156_0_e1578).exp();
        let noise_metadata_schedule_156_0_e1580: f64 = (params[23] * noise_metadata_schedule_156_0_e1579);
        (noise_metadata_schedule_156_0_e1580,)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_156_0_e1582;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_157_0_e1590,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_157_0_e1586: f64 = (0.5 * params[46]);
        let noise_metadata_schedule_157_0_e1588: f64 = (noise_metadata_schedule_157_0_e1586 / w[177]);
        (noise_metadata_schedule_157_0_e1588,)
    } else {
        (w[178],)
    }
};
            w[178] = noise_metadata_schedule_157_0_e1590;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_158_0_e1604,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_158_0_e1594: f64 = (2.0 * w[177]);
        let noise_metadata_schedule_158_0_e1596: f64 = (w[178]).exp();
        let noise_metadata_schedule_158_0_e1598: f64 = (-w[178]);
        let noise_metadata_schedule_158_0_e1599: f64 = (noise_metadata_schedule_158_0_e1598).exp();
        let noise_metadata_schedule_158_0_e1600: f64 = (noise_metadata_schedule_158_0_e1596 - noise_metadata_schedule_158_0_e1599);
        let noise_metadata_schedule_158_0_e1601: f64 = (noise_metadata_schedule_158_0_e1600).ln();
        let noise_metadata_schedule_158_0_e1602: f64 = (noise_metadata_schedule_158_0_e1594 * noise_metadata_schedule_158_0_e1601);
        (noise_metadata_schedule_158_0_e1602,)
    } else {
        (w[96],)
    }
};
            w[96] = noise_metadata_schedule_158_0_e1604;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_159_0_e1622,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_159_0_e1608: f64 = (w[96] * w[5]);
        let noise_metadata_schedule_159_0_e1612: f64 = (1.0 - w[5]);
        let noise_metadata_schedule_159_0_e1613: f64 = (w[174] * noise_metadata_schedule_159_0_e1612);
        let noise_metadata_schedule_159_0_e1614: f64 = (noise_metadata_schedule_159_0_e1608 + noise_metadata_schedule_159_0_e1613);
        let noise_metadata_schedule_159_0_e1617: f64 = (w[168] * w[2]);
        let noise_metadata_schedule_159_0_e1619: f64 = (noise_metadata_schedule_159_0_e1617 * w[6]);
        let noise_metadata_schedule_159_0_e1620: f64 = (noise_metadata_schedule_159_0_e1614 - noise_metadata_schedule_159_0_e1619);
        (noise_metadata_schedule_159_0_e1620,)
    } else {
        (w[97],)
    }
};
            w[97] = noise_metadata_schedule_159_0_e1622;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_160_0_e1646,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_160_0_e1627: f64 = (2.0 * w[2]);
        let noise_metadata_schedule_160_0_e1633: f64 = (-w[97]);
        let noise_metadata_schedule_160_0_e1635: f64 = (noise_metadata_schedule_160_0_e1633 * w[3]);
        let noise_metadata_schedule_160_0_e1636: f64 = (noise_metadata_schedule_160_0_e1635).exp();
        let noise_metadata_schedule_160_0_e1637: f64 = (4.0 * noise_metadata_schedule_160_0_e1636);
        let noise_metadata_schedule_160_0_e1638: f64 = (1.0 + noise_metadata_schedule_160_0_e1637);
        let noise_metadata_schedule_160_0_e1639: f64 = (noise_metadata_schedule_160_0_e1638).sqrt();
        let noise_metadata_schedule_160_0_e1640: f64 = (1.0 + noise_metadata_schedule_160_0_e1639);
        let noise_metadata_schedule_160_0_e1641: f64 = (0.5 * noise_metadata_schedule_160_0_e1640);
        let noise_metadata_schedule_160_0_e1642: f64 = (noise_metadata_schedule_160_0_e1641).ln();
        let noise_metadata_schedule_160_0_e1643: f64 = (noise_metadata_schedule_160_0_e1627 * noise_metadata_schedule_160_0_e1642);
        let noise_metadata_schedule_160_0_e1644: f64 = (w[97] + noise_metadata_schedule_160_0_e1643);
        (noise_metadata_schedule_160_0_e1644,)
    } else {
        (w[18],)
    }
};
            w[18] = noise_metadata_schedule_160_0_e1646;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_161_0_e1658,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_161_0_e1652: f64 = (params[46] / w[18]);
        let noise_metadata_schedule_161_0_e1653: f64 = (noise_metadata_schedule_161_0_e1652).ln();
        let noise_metadata_schedule_161_0_e1654: f64 = (params[47] * noise_metadata_schedule_161_0_e1653);
        let noise_metadata_schedule_161_0_e1655: f64 = (noise_metadata_schedule_161_0_e1654).exp();
        let noise_metadata_schedule_161_0_e1656: f64 = (params[45] * noise_metadata_schedule_161_0_e1655);
        (noise_metadata_schedule_161_0_e1656,)
    } else {
        (w[25],)
    }
};
            w[25] = noise_metadata_schedule_161_0_e1658;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_169_0_e1769,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_169_0_e1765: f64 = (params[97] * w[6]);
        let noise_metadata_schedule_169_0_e1766: f64 = (noise_metadata_schedule_169_0_e1765).exp();
        let noise_metadata_schedule_169_0_e1767: f64 = (params[7] * noise_metadata_schedule_169_0_e1766);
        (noise_metadata_schedule_169_0_e1767,)
    } else {
        (w[200],)
    }
};
            w[200] = noise_metadata_schedule_169_0_e1769;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_170_0_e1785,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_170_0_e1774: f64 = (params[83] * w[3]);
        let noise_metadata_schedule_170_0_e1777: f64 = (params[84] * w[6]);
        let noise_metadata_schedule_170_0_e1778: f64 = (noise_metadata_schedule_170_0_e1777).exp();
        let noise_metadata_schedule_170_0_e1780: f64 = (noise_metadata_schedule_170_0_e1778 - 1.0);
        let noise_metadata_schedule_170_0_e1781: f64 = (noise_metadata_schedule_170_0_e1774 * noise_metadata_schedule_170_0_e1780);
        let noise_metadata_schedule_170_0_e1782: f64 = (noise_metadata_schedule_170_0_e1781).exp();
        let noise_metadata_schedule_170_0_e1783: f64 = (params[6] / noise_metadata_schedule_170_0_e1782);
        (noise_metadata_schedule_170_0_e1783,)
    } else {
        (w[202],)
    }
};
            w[202] = noise_metadata_schedule_170_0_e1785;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_171_0_e1788: f64 = if params[0] <= 200.0 { 1.0 } else { 0.0 };
            w[259] = noise_metadata_schedule_171_0_e1788;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_172_0_e1802,) = {
    if ((w[253] != 0.0) && (w[259] != 0.0)) {
        let noise_metadata_schedule_172_0_e1797: f64 = (params[102] * w[7]);
        let noise_metadata_schedule_172_0_e1798: f64 = (params[101] + noise_metadata_schedule_172_0_e1797);
        let noise_metadata_schedule_172_0_e1799: f64 = (w[7] * noise_metadata_schedule_172_0_e1798);
        let noise_metadata_schedule_172_0_e1800: f64 = (1.0 + noise_metadata_schedule_172_0_e1799);
        (noise_metadata_schedule_172_0_e1800,)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_172_0_e1802;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_173_0_e1812,) = {
    if ((w[253] != 0.0) && (w[259] == 0.0)) {
        let noise_metadata_schedule_173_0_e1809: f64 = (params[98] * w[6]);
        let noise_metadata_schedule_173_0_e1810: f64 = (noise_metadata_schedule_173_0_e1809).exp();
        (noise_metadata_schedule_173_0_e1810,)
    } else {
        (w[204],)
    }
};
            w[204] = noise_metadata_schedule_173_0_e1812;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_174_0_e1818,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_174_0_e1816: f64 = (params[12] * w[204]);
        (noise_metadata_schedule_174_0_e1816,)
    } else {
        (w[203],)
    }
};
            w[203] = noise_metadata_schedule_174_0_e1818;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_175_0_e1829,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_175_0_e1822: f64 = (params[13] * w[204]);
        let noise_metadata_schedule_175_0_e1825: f64 = (w[176] * w[10]);
        let noise_metadata_schedule_175_0_e1826: f64 = (noise_metadata_schedule_175_0_e1825).exp();
        let noise_metadata_schedule_175_0_e1827: f64 = (noise_metadata_schedule_175_0_e1822 * noise_metadata_schedule_175_0_e1826);
        (noise_metadata_schedule_175_0_e1827,)
    } else {
        (w[205],)
    }
};
            w[205] = noise_metadata_schedule_175_0_e1829;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_176_0_e1833,) = {
    if (w[253] != 0.0) {
        (params[14],)
    } else {
        (w[206],)
    }
};
            w[206] = noise_metadata_schedule_176_0_e1833;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_177_0_e1842,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_177_0_e1838: f64 = (params[93] * w[6]);
        let noise_metadata_schedule_177_0_e1839: f64 = (noise_metadata_schedule_177_0_e1838).exp();
        let noise_metadata_schedule_177_0_e1840: f64 = (params[29] * noise_metadata_schedule_177_0_e1839);
        (noise_metadata_schedule_177_0_e1840,)
    } else {
        (w[40],)
    }
};
            w[40] = noise_metadata_schedule_177_0_e1842;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_178_0_e1851,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_178_0_e1847: f64 = (params[92] * w[6]);
        let noise_metadata_schedule_178_0_e1848: f64 = (noise_metadata_schedule_178_0_e1847).exp();
        let noise_metadata_schedule_178_0_e1849: f64 = (params[26] * noise_metadata_schedule_178_0_e1848);
        (noise_metadata_schedule_178_0_e1849,)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_178_0_e1851;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_179_0_e1860,) = {
    if (w[253] != 0.0) {
        let noise_metadata_schedule_179_0_e1856: f64 = (params[94] * w[6]);
        let noise_metadata_schedule_179_0_e1857: f64 = (noise_metadata_schedule_179_0_e1856).exp();
        let noise_metadata_schedule_179_0_e1858: f64 = (params[28] * noise_metadata_schedule_179_0_e1857);
        (noise_metadata_schedule_179_0_e1858,)
    } else {
        (w[41],)
    }
};
            w[41] = noise_metadata_schedule_179_0_e1860;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_181_0_e1878: f64 = if w[25] <= 1e-30 { 1.0 } else { 0.0 };
            w[260] = noise_metadata_schedule_181_0_e1878;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_182_0_e1884,) = {
    if (w[260] != 0.0) {
        let noise_metadata_schedule_182_0_e1882: f64 = (w[24] * params[49]);
        (noise_metadata_schedule_182_0_e1882,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_182_0_e1884;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_185_0_e1900,) = {
    if (w[260] != 0.0) {
        let noise_metadata_schedule_185_0_e1897: f64 = (1.0 - params[49]);
        let noise_metadata_schedule_185_0_e1898: f64 = (w[24] * noise_metadata_schedule_185_0_e1897);
        (noise_metadata_schedule_185_0_e1898,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_185_0_e1900;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_186_0_e1903: f64 = if params[44] < 100.0 { 1.0 } else { 0.0 };
            w[261] = noise_metadata_schedule_186_0_e1903;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_187_0_e1906: f64 = if w[113] > 0.0 { 1.0 } else { 0.0 };
            w[262] = noise_metadata_schedule_187_0_e1906;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_188_0_e1916,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_188_0_e1914: f64 = (params[43] / 4.0);
        (noise_metadata_schedule_188_0_e1914,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_188_0_e1916;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_189_0_e1926,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_189_0_e1924: f64 = (params[44] - w[17]);
        (noise_metadata_schedule_189_0_e1924,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_189_0_e1926;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_190_0_e1943,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_190_0_e1935: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_190_0_e1936: f64 = (-noise_metadata_schedule_190_0_e1935);
        let noise_metadata_schedule_190_0_e1938: f64 = (noise_metadata_schedule_190_0_e1936 / params[43]);
        let noise_metadata_schedule_190_0_e1939: f64 = (noise_metadata_schedule_190_0_e1938).exp();
        let noise_metadata_schedule_190_0_e1940: f64 = (1.0 - noise_metadata_schedule_190_0_e1939);
        let noise_metadata_schedule_190_0_e1941: f64 = (w[17] * noise_metadata_schedule_190_0_e1940);
        (noise_metadata_schedule_190_0_e1941,)
    } else {
        (w[52],)
    }
};
            w[52] = noise_metadata_schedule_190_0_e1943;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_191_0_e1953,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_191_0_e1951: f64 = (2.4 * w[113]);
        (noise_metadata_schedule_191_0_e1951,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_191_0_e1953;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_192_0_e1971,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_192_0_e1962: f64 = (w[50] - params[43]);
        let noise_metadata_schedule_192_0_e1965: f64 = (params[44] / w[17]);
        let noise_metadata_schedule_192_0_e1966: f64 = (noise_metadata_schedule_192_0_e1965).ln();
        let noise_metadata_schedule_192_0_e1967: f64 = (noise_metadata_schedule_192_0_e1962 * noise_metadata_schedule_192_0_e1966);
        let noise_metadata_schedule_192_0_e1968: f64 = (noise_metadata_schedule_192_0_e1967).exp();
        let noise_metadata_schedule_192_0_e1969: f64 = (w[113] * noise_metadata_schedule_192_0_e1968);
        (noise_metadata_schedule_192_0_e1969,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_192_0_e1971;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_193_0_e1983,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_193_0_e1979: f64 = (w[52] - w[183]);
        let noise_metadata_schedule_193_0_e1981: f64 = (noise_metadata_schedule_193_0_e1979 * w[3]);
        (noise_metadata_schedule_193_0_e1981,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_193_0_e1983;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_194_0_e1986: f64 = if w[56] < 80.0 { 1.0 } else { 0.0 };
            w[263] = noise_metadata_schedule_194_0_e1986;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_195_0_e1997,) = {
    if ((((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) && (w[263] != 0.0)) {
        let noise_metadata_schedule_195_0_e1995: f64 = (w[56]).exp();
        (noise_metadata_schedule_195_0_e1995,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_195_0_e1997;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_197_0_e2028,) = {
    if ((((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) && (w[263] != 0.0)) {
        let noise_metadata_schedule_197_0_e2023: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_197_0_e2024: f64 = (noise_metadata_schedule_197_0_e2023).ln();
        let noise_metadata_schedule_197_0_e2025: f64 = (w[2] * noise_metadata_schedule_197_0_e2024);
        let noise_metadata_schedule_197_0_e2026: f64 = (w[52] - noise_metadata_schedule_197_0_e2025);
        (noise_metadata_schedule_197_0_e2026,)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_197_0_e2028;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_199_0_e2050,) = {
    if ((((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) && (w[263] == 0.0)) {
        (w[183],)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_199_0_e2050;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_200_0_e2064,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_200_0_e2058: f64 = (0.1 * w[51]);
        let noise_metadata_schedule_200_0_e2061: f64 = (4.0 * w[2]);
        let noise_metadata_schedule_200_0_e2062: f64 = (noise_metadata_schedule_200_0_e2058 + noise_metadata_schedule_200_0_e2061);
        (noise_metadata_schedule_200_0_e2062,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_200_0_e2064;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_201_0_e2076,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_201_0_e2072: f64 = (w[51] + w[58]);
        let noise_metadata_schedule_201_0_e2074: f64 = (noise_metadata_schedule_201_0_e2072 / w[55]);
        (noise_metadata_schedule_201_0_e2074,)
    } else {
        (w[59],)
    }
};
            w[59] = noise_metadata_schedule_201_0_e2076;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_202_0_e2079: f64 = if w[59] < 80.0 { 1.0 } else { 0.0 };
            w[264] = noise_metadata_schedule_202_0_e2079;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_203_0_e2090,) = {
    if ((((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) && (w[264] != 0.0)) {
        let noise_metadata_schedule_203_0_e2088: f64 = (w[59]).exp();
        (noise_metadata_schedule_203_0_e2088,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_203_0_e2090;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_205_0_e2130,) = {
    if ((((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) && (w[264] != 0.0)) {
        let noise_metadata_schedule_205_0_e2113: f64 = (-w[51]);
        let noise_metadata_schedule_205_0_e2117: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_205_0_e2118: f64 = (noise_metadata_schedule_205_0_e2117).ln();
        let noise_metadata_schedule_205_0_e2121: f64 = (w[51] + w[52]);
        let noise_metadata_schedule_205_0_e2122: f64 = (-noise_metadata_schedule_205_0_e2121);
        let noise_metadata_schedule_205_0_e2124: f64 = (noise_metadata_schedule_205_0_e2122 / w[55]);
        let noise_metadata_schedule_205_0_e2125: f64 = (noise_metadata_schedule_205_0_e2124).exp();
        let noise_metadata_schedule_205_0_e2126: f64 = (noise_metadata_schedule_205_0_e2118 - noise_metadata_schedule_205_0_e2125);
        let noise_metadata_schedule_205_0_e2127: f64 = (w[55] * noise_metadata_schedule_205_0_e2126);
        let noise_metadata_schedule_205_0_e2128: f64 = (noise_metadata_schedule_205_0_e2113 + noise_metadata_schedule_205_0_e2127);
        (noise_metadata_schedule_205_0_e2128,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_205_0_e2130;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_207_0_e2152,) = {
    if ((((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) && (w[264] == 0.0)) {
        (w[58],)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_207_0_e2152;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_208_0_e2162,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_208_0_e2160: f64 = (w[183] - w[58]);
        (noise_metadata_schedule_208_0_e2160,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_208_0_e2162;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_209_0_e2175,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_209_0_e2171: f64 = (w[58] / w[17]);
        let noise_metadata_schedule_209_0_e2172: f64 = (1.0 - noise_metadata_schedule_209_0_e2171);
        let noise_metadata_schedule_209_0_e2173: f64 = (noise_metadata_schedule_209_0_e2172).ln();
        (noise_metadata_schedule_209_0_e2173,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_209_0_e2175;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_210_0_e2188,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_210_0_e2184: f64 = (w[60] / w[17]);
        let noise_metadata_schedule_210_0_e2185: f64 = (1.0 - noise_metadata_schedule_210_0_e2184);
        let noise_metadata_schedule_210_0_e2186: f64 = (noise_metadata_schedule_210_0_e2185).ln();
        (noise_metadata_schedule_210_0_e2186,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_210_0_e2188;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_211_0_e2198,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_211_0_e2196: f64 = (1.0 - params[43]);
        (noise_metadata_schedule_211_0_e2196,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_211_0_e2198;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_212_0_e2208,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_212_0_e2206: f64 = (1.0 - w[50]);
        (noise_metadata_schedule_212_0_e2206,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_212_0_e2208;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_217_0_e2285,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_217_0_e2278: f64 = (w[66] * w[67]);
        let noise_metadata_schedule_217_0_e2279: f64 = (noise_metadata_schedule_217_0_e2278).exp();
        let noise_metadata_schedule_217_0_e2280: f64 = (1.0 - noise_metadata_schedule_217_0_e2279);
        let noise_metadata_schedule_217_0_e2281: f64 = (w[113] * noise_metadata_schedule_217_0_e2280);
        let noise_metadata_schedule_217_0_e2283: f64 = (noise_metadata_schedule_217_0_e2281 / w[67]);
        (noise_metadata_schedule_217_0_e2283,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_217_0_e2285;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_218_0_e2302,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_218_0_e2295: f64 = (w[65] * w[68]);
        let noise_metadata_schedule_218_0_e2296: f64 = (noise_metadata_schedule_218_0_e2295).exp();
        let noise_metadata_schedule_218_0_e2297: f64 = (1.0 - noise_metadata_schedule_218_0_e2296);
        let noise_metadata_schedule_218_0_e2298: f64 = (w[54] * noise_metadata_schedule_218_0_e2297);
        let noise_metadata_schedule_218_0_e2300: f64 = (noise_metadata_schedule_218_0_e2298 / w[68]);
        (noise_metadata_schedule_218_0_e2300,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_218_0_e2302;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_219_0_e2319,) = {
    if (((w[260] != 0.0) && (w[261] != 0.0)) && (w[262] != 0.0)) {
        let noise_metadata_schedule_219_0_e2312: f64 = (w[66] * w[68]);
        let noise_metadata_schedule_219_0_e2313: f64 = (noise_metadata_schedule_219_0_e2312).exp();
        let noise_metadata_schedule_219_0_e2314: f64 = (1.0 - noise_metadata_schedule_219_0_e2313);
        let noise_metadata_schedule_219_0_e2315: f64 = (w[54] * noise_metadata_schedule_219_0_e2314);
        let noise_metadata_schedule_219_0_e2317: f64 = (noise_metadata_schedule_219_0_e2315 / w[68]);
        (noise_metadata_schedule_219_0_e2317,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_219_0_e2319;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_223_0_e2358: f64 = if w[113] > 0.0 { 1.0 } else { 0.0 };
            w[265] = noise_metadata_schedule_223_0_e2358;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_224_0_e2376,) = {
    if (((w[260] != 0.0) && (w[261] == 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_224_0_e2368: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_224_0_e2369: f64 = (-noise_metadata_schedule_224_0_e2368);
        let noise_metadata_schedule_224_0_e2371: f64 = (noise_metadata_schedule_224_0_e2369 / params[43]);
        let noise_metadata_schedule_224_0_e2372: f64 = (noise_metadata_schedule_224_0_e2371).exp();
        let noise_metadata_schedule_224_0_e2373: f64 = (1.0 - noise_metadata_schedule_224_0_e2372);
        let noise_metadata_schedule_224_0_e2374: f64 = (w[17] * noise_metadata_schedule_224_0_e2373);
        (noise_metadata_schedule_224_0_e2374,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_224_0_e2376;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_225_0_e2389,) = {
    if (((w[260] != 0.0) && (w[261] == 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_225_0_e2385: f64 = (w[76] - w[183]);
        let noise_metadata_schedule_225_0_e2387: f64 = (noise_metadata_schedule_225_0_e2385 * w[3]);
        (noise_metadata_schedule_225_0_e2387,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_225_0_e2389;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_226_0_e2403,) = {
    if (((w[260] != 0.0) && (w[261] == 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_226_0_e2398: f64 = (w[80] * w[80]);
        let noise_metadata_schedule_226_0_e2400: f64 = (noise_metadata_schedule_226_0_e2398 + 1.921812);
        let noise_metadata_schedule_226_0_e2401: f64 = (noise_metadata_schedule_226_0_e2400).sqrt();
        (noise_metadata_schedule_226_0_e2401,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_226_0_e2403;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_227_0_e2416,) = {
    if (((w[260] != 0.0) && (w[261] == 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_227_0_e2412: f64 = (w[80] + w[81]);
        let noise_metadata_schedule_227_0_e2414: f64 = (noise_metadata_schedule_227_0_e2412 * 0.5);
        (noise_metadata_schedule_227_0_e2414,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_227_0_e2416;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_228_0_e2429,) = {
    if (((w[260] != 0.0) && (w[261] == 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_228_0_e2426: f64 = (w[2] * w[82]);
        let noise_metadata_schedule_228_0_e2427: f64 = (w[76] - noise_metadata_schedule_228_0_e2426);
        (noise_metadata_schedule_228_0_e2427,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_228_0_e2429;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_230_0_e2454,) = {
    if (((w[260] != 0.0) && (w[261] == 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_230_0_e2450: f64 = (w[77] / w[17]);
        let noise_metadata_schedule_230_0_e2451: f64 = (1.0 - noise_metadata_schedule_230_0_e2450);
        let noise_metadata_schedule_230_0_e2452: f64 = (noise_metadata_schedule_230_0_e2451).ln();
        (noise_metadata_schedule_230_0_e2452,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_230_0_e2454;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_233_0_e2508,) = {
    if (((w[260] != 0.0) && (w[261] == 0.0)) && (w[265] != 0.0)) {
        let noise_metadata_schedule_233_0_e2498: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_233_0_e2499: f64 = (w[78] * noise_metadata_schedule_233_0_e2498);
        let noise_metadata_schedule_233_0_e2500: f64 = (noise_metadata_schedule_233_0_e2499).exp();
        let noise_metadata_schedule_233_0_e2501: f64 = (1.0 - noise_metadata_schedule_233_0_e2500);
        let noise_metadata_schedule_233_0_e2502: f64 = (w[17] * noise_metadata_schedule_233_0_e2501);
        let noise_metadata_schedule_233_0_e2505: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_233_0_e2506: f64 = (noise_metadata_schedule_233_0_e2502 / noise_metadata_schedule_233_0_e2505);
        (noise_metadata_schedule_233_0_e2506,)
    } else {
        (w[79],)
    }
};
            w[79] = noise_metadata_schedule_233_0_e2508;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_237_0_e2550,) = {
    if (w[260] == 0.0) {
        (w[24],)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_237_0_e2550;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_238_0_e2557,) = {
    if (w[260] == 0.0) {
        let noise_metadata_schedule_238_0_e2555: f64 = (w[25] * params[49]);
        (noise_metadata_schedule_238_0_e2555,)
    } else {
        (w[112],)
    }
};
            w[112] = noise_metadata_schedule_238_0_e2557;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_239_0_e2560: f64 = if params[48] < 100.0 { 1.0 } else { 0.0 };
            w[266] = noise_metadata_schedule_239_0_e2560;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_240_0_e2563: f64 = if w[112] > 0.0 { 1.0 } else { 0.0 };
            w[267] = noise_metadata_schedule_240_0_e2563;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_241_0_e2574,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_241_0_e2572: f64 = (params[47] / 4.0);
        (noise_metadata_schedule_241_0_e2572,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_241_0_e2574;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_242_0_e2585,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_242_0_e2583: f64 = (params[48] - w[18]);
        (noise_metadata_schedule_242_0_e2583,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_242_0_e2585;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_243_0_e2603,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_243_0_e2595: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_243_0_e2596: f64 = (-noise_metadata_schedule_243_0_e2595);
        let noise_metadata_schedule_243_0_e2598: f64 = (noise_metadata_schedule_243_0_e2596 / params[47]);
        let noise_metadata_schedule_243_0_e2599: f64 = (noise_metadata_schedule_243_0_e2598).exp();
        let noise_metadata_schedule_243_0_e2600: f64 = (1.0 - noise_metadata_schedule_243_0_e2599);
        let noise_metadata_schedule_243_0_e2601: f64 = (w[18] * noise_metadata_schedule_243_0_e2600);
        (noise_metadata_schedule_243_0_e2601,)
    } else {
        (w[52],)
    }
};
            w[52] = noise_metadata_schedule_243_0_e2603;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_244_0_e2614,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_244_0_e2612: f64 = (2.4 * w[112]);
        (noise_metadata_schedule_244_0_e2612,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_244_0_e2614;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_245_0_e2633,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_245_0_e2624: f64 = (w[50] - params[47]);
        let noise_metadata_schedule_245_0_e2627: f64 = (params[48] / w[18]);
        let noise_metadata_schedule_245_0_e2628: f64 = (noise_metadata_schedule_245_0_e2627).ln();
        let noise_metadata_schedule_245_0_e2629: f64 = (noise_metadata_schedule_245_0_e2624 * noise_metadata_schedule_245_0_e2628);
        let noise_metadata_schedule_245_0_e2630: f64 = (noise_metadata_schedule_245_0_e2629).exp();
        let noise_metadata_schedule_245_0_e2631: f64 = (w[112] * noise_metadata_schedule_245_0_e2630);
        (noise_metadata_schedule_245_0_e2631,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_245_0_e2633;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_246_0_e2646,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_246_0_e2642: f64 = (w[52] - w[184]);
        let noise_metadata_schedule_246_0_e2644: f64 = (noise_metadata_schedule_246_0_e2642 * w[3]);
        (noise_metadata_schedule_246_0_e2644,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_246_0_e2646;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_247_0_e2649: f64 = if w[56] < 80.0 { 1.0 } else { 0.0 };
            w[268] = noise_metadata_schedule_247_0_e2649;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_248_0_e2661,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) && (w[268] != 0.0)) {
        let noise_metadata_schedule_248_0_e2659: f64 = (w[56]).exp();
        (noise_metadata_schedule_248_0_e2659,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_248_0_e2661;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_250_0_e2694,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) && (w[268] != 0.0)) {
        let noise_metadata_schedule_250_0_e2689: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_250_0_e2690: f64 = (noise_metadata_schedule_250_0_e2689).ln();
        let noise_metadata_schedule_250_0_e2691: f64 = (w[2] * noise_metadata_schedule_250_0_e2690);
        let noise_metadata_schedule_250_0_e2692: f64 = (w[52] - noise_metadata_schedule_250_0_e2691);
        (noise_metadata_schedule_250_0_e2692,)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_250_0_e2694;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_252_0_e2718,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) && (w[268] == 0.0)) {
        (w[184],)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_252_0_e2718;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_253_0_e2733,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_253_0_e2727: f64 = (0.1 * w[51]);
        let noise_metadata_schedule_253_0_e2730: f64 = (4.0 * w[2]);
        let noise_metadata_schedule_253_0_e2731: f64 = (noise_metadata_schedule_253_0_e2727 + noise_metadata_schedule_253_0_e2730);
        (noise_metadata_schedule_253_0_e2731,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_253_0_e2733;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_254_0_e2746,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_254_0_e2742: f64 = (w[51] + w[58]);
        let noise_metadata_schedule_254_0_e2744: f64 = (noise_metadata_schedule_254_0_e2742 / w[55]);
        (noise_metadata_schedule_254_0_e2744,)
    } else {
        (w[59],)
    }
};
            w[59] = noise_metadata_schedule_254_0_e2746;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_255_0_e2749: f64 = if w[59] < 80.0 { 1.0 } else { 0.0 };
            w[269] = noise_metadata_schedule_255_0_e2749;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_256_0_e2761,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) && (w[269] != 0.0)) {
        let noise_metadata_schedule_256_0_e2759: f64 = (w[59]).exp();
        (noise_metadata_schedule_256_0_e2759,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_256_0_e2761;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_258_0_e2803,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) && (w[269] != 0.0)) {
        let noise_metadata_schedule_258_0_e2786: f64 = (-w[51]);
        let noise_metadata_schedule_258_0_e2790: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_258_0_e2791: f64 = (noise_metadata_schedule_258_0_e2790).ln();
        let noise_metadata_schedule_258_0_e2794: f64 = (w[51] + w[52]);
        let noise_metadata_schedule_258_0_e2795: f64 = (-noise_metadata_schedule_258_0_e2794);
        let noise_metadata_schedule_258_0_e2797: f64 = (noise_metadata_schedule_258_0_e2795 / w[55]);
        let noise_metadata_schedule_258_0_e2798: f64 = (noise_metadata_schedule_258_0_e2797).exp();
        let noise_metadata_schedule_258_0_e2799: f64 = (noise_metadata_schedule_258_0_e2791 - noise_metadata_schedule_258_0_e2798);
        let noise_metadata_schedule_258_0_e2800: f64 = (w[55] * noise_metadata_schedule_258_0_e2799);
        let noise_metadata_schedule_258_0_e2801: f64 = (noise_metadata_schedule_258_0_e2786 + noise_metadata_schedule_258_0_e2800);
        (noise_metadata_schedule_258_0_e2801,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_258_0_e2803;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_260_0_e2827,) = {
    if ((((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) && (w[269] == 0.0)) {
        (w[58],)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_260_0_e2827;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_261_0_e2838,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_261_0_e2836: f64 = (w[184] - w[58]);
        (noise_metadata_schedule_261_0_e2836,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_261_0_e2838;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_262_0_e2852,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_262_0_e2848: f64 = (w[58] / w[18]);
        let noise_metadata_schedule_262_0_e2849: f64 = (1.0 - noise_metadata_schedule_262_0_e2848);
        let noise_metadata_schedule_262_0_e2850: f64 = (noise_metadata_schedule_262_0_e2849).ln();
        (noise_metadata_schedule_262_0_e2850,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_262_0_e2852;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_263_0_e2866,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_263_0_e2862: f64 = (w[60] / w[18]);
        let noise_metadata_schedule_263_0_e2863: f64 = (1.0 - noise_metadata_schedule_263_0_e2862);
        let noise_metadata_schedule_263_0_e2864: f64 = (noise_metadata_schedule_263_0_e2863).ln();
        (noise_metadata_schedule_263_0_e2864,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_263_0_e2866;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_264_0_e2877,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_264_0_e2875: f64 = (1.0 - params[47]);
        (noise_metadata_schedule_264_0_e2875,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_264_0_e2877;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_265_0_e2888,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_265_0_e2886: f64 = (1.0 - w[50]);
        (noise_metadata_schedule_265_0_e2886,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_265_0_e2888;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_270_0_e2970,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_270_0_e2963: f64 = (w[66] * w[67]);
        let noise_metadata_schedule_270_0_e2964: f64 = (noise_metadata_schedule_270_0_e2963).exp();
        let noise_metadata_schedule_270_0_e2965: f64 = (1.0 - noise_metadata_schedule_270_0_e2964);
        let noise_metadata_schedule_270_0_e2966: f64 = (w[112] * noise_metadata_schedule_270_0_e2965);
        let noise_metadata_schedule_270_0_e2968: f64 = (noise_metadata_schedule_270_0_e2966 / w[67]);
        (noise_metadata_schedule_270_0_e2968,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_270_0_e2970;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_271_0_e2988,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_271_0_e2981: f64 = (w[65] * w[68]);
        let noise_metadata_schedule_271_0_e2982: f64 = (noise_metadata_schedule_271_0_e2981).exp();
        let noise_metadata_schedule_271_0_e2983: f64 = (1.0 - noise_metadata_schedule_271_0_e2982);
        let noise_metadata_schedule_271_0_e2984: f64 = (w[54] * noise_metadata_schedule_271_0_e2983);
        let noise_metadata_schedule_271_0_e2986: f64 = (noise_metadata_schedule_271_0_e2984 / w[68]);
        (noise_metadata_schedule_271_0_e2986,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_271_0_e2988;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_272_0_e3006,) = {
    if (((w[260] == 0.0) && (w[266] != 0.0)) && (w[267] != 0.0)) {
        let noise_metadata_schedule_272_0_e2999: f64 = (w[66] * w[68]);
        let noise_metadata_schedule_272_0_e3000: f64 = (noise_metadata_schedule_272_0_e2999).exp();
        let noise_metadata_schedule_272_0_e3001: f64 = (1.0 - noise_metadata_schedule_272_0_e3000);
        let noise_metadata_schedule_272_0_e3002: f64 = (w[54] * noise_metadata_schedule_272_0_e3001);
        let noise_metadata_schedule_272_0_e3004: f64 = (noise_metadata_schedule_272_0_e3002 / w[68]);
        (noise_metadata_schedule_272_0_e3004,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_272_0_e3006;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_276_0_e3048: f64 = if w[112] > 0.0 { 1.0 } else { 0.0 };
            w[270] = noise_metadata_schedule_276_0_e3048;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_277_0_e3067,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_277_0_e3059: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_277_0_e3060: f64 = (-noise_metadata_schedule_277_0_e3059);
        let noise_metadata_schedule_277_0_e3062: f64 = (noise_metadata_schedule_277_0_e3060 / params[47]);
        let noise_metadata_schedule_277_0_e3063: f64 = (noise_metadata_schedule_277_0_e3062).exp();
        let noise_metadata_schedule_277_0_e3064: f64 = (1.0 - noise_metadata_schedule_277_0_e3063);
        let noise_metadata_schedule_277_0_e3065: f64 = (w[18] * noise_metadata_schedule_277_0_e3064);
        (noise_metadata_schedule_277_0_e3065,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_277_0_e3067;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_278_0_e3081,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_278_0_e3077: f64 = (w[76] - w[184]);
        let noise_metadata_schedule_278_0_e3079: f64 = (noise_metadata_schedule_278_0_e3077 * w[3]);
        (noise_metadata_schedule_278_0_e3079,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_278_0_e3081;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_279_0_e3096,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_279_0_e3091: f64 = (w[80] * w[80]);
        let noise_metadata_schedule_279_0_e3093: f64 = (noise_metadata_schedule_279_0_e3091 + 1.921812);
        let noise_metadata_schedule_279_0_e3094: f64 = (noise_metadata_schedule_279_0_e3093).sqrt();
        (noise_metadata_schedule_279_0_e3094,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_279_0_e3096;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_280_0_e3110,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_280_0_e3106: f64 = (w[80] + w[81]);
        let noise_metadata_schedule_280_0_e3108: f64 = (noise_metadata_schedule_280_0_e3106 * 0.5);
        (noise_metadata_schedule_280_0_e3108,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_280_0_e3110;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_281_0_e3124,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_281_0_e3121: f64 = (w[2] * w[82]);
        let noise_metadata_schedule_281_0_e3122: f64 = (w[76] - noise_metadata_schedule_281_0_e3121);
        (noise_metadata_schedule_281_0_e3122,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_281_0_e3124;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_283_0_e3151,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_283_0_e3147: f64 = (w[77] / w[18]);
        let noise_metadata_schedule_283_0_e3148: f64 = (1.0 - noise_metadata_schedule_283_0_e3147);
        let noise_metadata_schedule_283_0_e3149: f64 = (noise_metadata_schedule_283_0_e3148).ln();
        (noise_metadata_schedule_283_0_e3149,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_283_0_e3151;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_286_0_e3208,) = {
    if (((w[260] == 0.0) && (w[266] == 0.0)) && (w[270] != 0.0)) {
        let noise_metadata_schedule_286_0_e3198: f64 = (1.0 - params[47]);
        let noise_metadata_schedule_286_0_e3199: f64 = (w[78] * noise_metadata_schedule_286_0_e3198);
        let noise_metadata_schedule_286_0_e3200: f64 = (noise_metadata_schedule_286_0_e3199).exp();
        let noise_metadata_schedule_286_0_e3201: f64 = (1.0 - noise_metadata_schedule_286_0_e3200);
        let noise_metadata_schedule_286_0_e3202: f64 = (w[18] * noise_metadata_schedule_286_0_e3201);
        let noise_metadata_schedule_286_0_e3205: f64 = (1.0 - params[47]);
        let noise_metadata_schedule_286_0_e3206: f64 = (noise_metadata_schedule_286_0_e3202 / noise_metadata_schedule_286_0_e3205);
        (noise_metadata_schedule_286_0_e3206,)
    } else {
        (w[79],)
    }
};
            w[79] = noise_metadata_schedule_286_0_e3208;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_290_0_e3257,) = {
    if (w[260] == 0.0) {
        let noise_metadata_schedule_290_0_e3254: f64 = (1.0 - params[49]);
        let noise_metadata_schedule_290_0_e3255: f64 = (w[25] * noise_metadata_schedule_290_0_e3254);
        (noise_metadata_schedule_290_0_e3255,)
    } else {
        (w[113],)
    }
};
            w[113] = noise_metadata_schedule_290_0_e3257;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_291_0_e3260: f64 = if params[48] < 100.0 { 1.0 } else { 0.0 };
            w[271] = noise_metadata_schedule_291_0_e3260;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_292_0_e3263: f64 = if w[113] > 0.0 { 1.0 } else { 0.0 };
            w[272] = noise_metadata_schedule_292_0_e3263;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_293_0_e3274,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_293_0_e3272: f64 = (params[47] / 4.0);
        (noise_metadata_schedule_293_0_e3272,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_293_0_e3274;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_294_0_e3285,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_294_0_e3283: f64 = (params[48] - w[18]);
        (noise_metadata_schedule_294_0_e3283,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_294_0_e3285;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_295_0_e3303,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_295_0_e3295: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_295_0_e3296: f64 = (-noise_metadata_schedule_295_0_e3295);
        let noise_metadata_schedule_295_0_e3298: f64 = (noise_metadata_schedule_295_0_e3296 / params[47]);
        let noise_metadata_schedule_295_0_e3299: f64 = (noise_metadata_schedule_295_0_e3298).exp();
        let noise_metadata_schedule_295_0_e3300: f64 = (1.0 - noise_metadata_schedule_295_0_e3299);
        let noise_metadata_schedule_295_0_e3301: f64 = (w[18] * noise_metadata_schedule_295_0_e3300);
        (noise_metadata_schedule_295_0_e3301,)
    } else {
        (w[52],)
    }
};
            w[52] = noise_metadata_schedule_295_0_e3303;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_296_0_e3314,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_296_0_e3312: f64 = (2.4 * w[113]);
        (noise_metadata_schedule_296_0_e3312,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_296_0_e3314;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_297_0_e3333,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_297_0_e3324: f64 = (w[50] - params[47]);
        let noise_metadata_schedule_297_0_e3327: f64 = (params[48] / w[18]);
        let noise_metadata_schedule_297_0_e3328: f64 = (noise_metadata_schedule_297_0_e3327).ln();
        let noise_metadata_schedule_297_0_e3329: f64 = (noise_metadata_schedule_297_0_e3324 * noise_metadata_schedule_297_0_e3328);
        let noise_metadata_schedule_297_0_e3330: f64 = (noise_metadata_schedule_297_0_e3329).exp();
        let noise_metadata_schedule_297_0_e3331: f64 = (w[113] * noise_metadata_schedule_297_0_e3330);
        (noise_metadata_schedule_297_0_e3331,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_297_0_e3333;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_298_0_e3346,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_298_0_e3342: f64 = (w[52] - w[183]);
        let noise_metadata_schedule_298_0_e3344: f64 = (noise_metadata_schedule_298_0_e3342 * w[3]);
        (noise_metadata_schedule_298_0_e3344,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_298_0_e3346;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_299_0_e3349: f64 = if w[56] < 80.0 { 1.0 } else { 0.0 };
            w[273] = noise_metadata_schedule_299_0_e3349;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_300_0_e3361,) = {
    if ((((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) && (w[273] != 0.0)) {
        let noise_metadata_schedule_300_0_e3359: f64 = (w[56]).exp();
        (noise_metadata_schedule_300_0_e3359,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_300_0_e3361;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_302_0_e3394,) = {
    if ((((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) && (w[273] != 0.0)) {
        let noise_metadata_schedule_302_0_e3389: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_302_0_e3390: f64 = (noise_metadata_schedule_302_0_e3389).ln();
        let noise_metadata_schedule_302_0_e3391: f64 = (w[2] * noise_metadata_schedule_302_0_e3390);
        let noise_metadata_schedule_302_0_e3392: f64 = (w[52] - noise_metadata_schedule_302_0_e3391);
        (noise_metadata_schedule_302_0_e3392,)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_302_0_e3394;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_304_0_e3418,) = {
    if ((((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) && (w[273] == 0.0)) {
        (w[183],)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_304_0_e3418;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_305_0_e3433,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_305_0_e3427: f64 = (0.1 * w[51]);
        let noise_metadata_schedule_305_0_e3430: f64 = (4.0 * w[2]);
        let noise_metadata_schedule_305_0_e3431: f64 = (noise_metadata_schedule_305_0_e3427 + noise_metadata_schedule_305_0_e3430);
        (noise_metadata_schedule_305_0_e3431,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_305_0_e3433;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_306_0_e3446,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_306_0_e3442: f64 = (w[51] + w[58]);
        let noise_metadata_schedule_306_0_e3444: f64 = (noise_metadata_schedule_306_0_e3442 / w[55]);
        (noise_metadata_schedule_306_0_e3444,)
    } else {
        (w[59],)
    }
};
            w[59] = noise_metadata_schedule_306_0_e3446;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_307_0_e3449: f64 = if w[59] < 80.0 { 1.0 } else { 0.0 };
            w[274] = noise_metadata_schedule_307_0_e3449;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_308_0_e3461,) = {
    if ((((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) && (w[274] != 0.0)) {
        let noise_metadata_schedule_308_0_e3459: f64 = (w[59]).exp();
        (noise_metadata_schedule_308_0_e3459,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_308_0_e3461;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_310_0_e3503,) = {
    if ((((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) && (w[274] != 0.0)) {
        let noise_metadata_schedule_310_0_e3486: f64 = (-w[51]);
        let noise_metadata_schedule_310_0_e3490: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_310_0_e3491: f64 = (noise_metadata_schedule_310_0_e3490).ln();
        let noise_metadata_schedule_310_0_e3494: f64 = (w[51] + w[52]);
        let noise_metadata_schedule_310_0_e3495: f64 = (-noise_metadata_schedule_310_0_e3494);
        let noise_metadata_schedule_310_0_e3497: f64 = (noise_metadata_schedule_310_0_e3495 / w[55]);
        let noise_metadata_schedule_310_0_e3498: f64 = (noise_metadata_schedule_310_0_e3497).exp();
        let noise_metadata_schedule_310_0_e3499: f64 = (noise_metadata_schedule_310_0_e3491 - noise_metadata_schedule_310_0_e3498);
        let noise_metadata_schedule_310_0_e3500: f64 = (w[55] * noise_metadata_schedule_310_0_e3499);
        let noise_metadata_schedule_310_0_e3501: f64 = (noise_metadata_schedule_310_0_e3486 + noise_metadata_schedule_310_0_e3500);
        (noise_metadata_schedule_310_0_e3501,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_310_0_e3503;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_312_0_e3527,) = {
    if ((((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) && (w[274] == 0.0)) {
        (w[58],)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_312_0_e3527;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_313_0_e3538,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_313_0_e3536: f64 = (w[183] - w[58]);
        (noise_metadata_schedule_313_0_e3536,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_313_0_e3538;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_314_0_e3552,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_314_0_e3548: f64 = (w[58] / w[18]);
        let noise_metadata_schedule_314_0_e3549: f64 = (1.0 - noise_metadata_schedule_314_0_e3548);
        let noise_metadata_schedule_314_0_e3550: f64 = (noise_metadata_schedule_314_0_e3549).ln();
        (noise_metadata_schedule_314_0_e3550,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_314_0_e3552;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_315_0_e3566,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_315_0_e3562: f64 = (w[60] / w[18]);
        let noise_metadata_schedule_315_0_e3563: f64 = (1.0 - noise_metadata_schedule_315_0_e3562);
        let noise_metadata_schedule_315_0_e3564: f64 = (noise_metadata_schedule_315_0_e3563).ln();
        (noise_metadata_schedule_315_0_e3564,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_315_0_e3566;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_316_0_e3577,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_316_0_e3575: f64 = (1.0 - params[47]);
        (noise_metadata_schedule_316_0_e3575,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_316_0_e3577;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_317_0_e3588,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_317_0_e3586: f64 = (1.0 - w[50]);
        (noise_metadata_schedule_317_0_e3586,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_317_0_e3588;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_322_0_e3670,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_322_0_e3663: f64 = (w[66] * w[67]);
        let noise_metadata_schedule_322_0_e3664: f64 = (noise_metadata_schedule_322_0_e3663).exp();
        let noise_metadata_schedule_322_0_e3665: f64 = (1.0 - noise_metadata_schedule_322_0_e3664);
        let noise_metadata_schedule_322_0_e3666: f64 = (w[113] * noise_metadata_schedule_322_0_e3665);
        let noise_metadata_schedule_322_0_e3668: f64 = (noise_metadata_schedule_322_0_e3666 / w[67]);
        (noise_metadata_schedule_322_0_e3668,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_322_0_e3670;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_323_0_e3688,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_323_0_e3681: f64 = (w[65] * w[68]);
        let noise_metadata_schedule_323_0_e3682: f64 = (noise_metadata_schedule_323_0_e3681).exp();
        let noise_metadata_schedule_323_0_e3683: f64 = (1.0 - noise_metadata_schedule_323_0_e3682);
        let noise_metadata_schedule_323_0_e3684: f64 = (w[54] * noise_metadata_schedule_323_0_e3683);
        let noise_metadata_schedule_323_0_e3686: f64 = (noise_metadata_schedule_323_0_e3684 / w[68]);
        (noise_metadata_schedule_323_0_e3686,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_323_0_e3688;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_324_0_e3706,) = {
    if (((w[260] == 0.0) && (w[271] != 0.0)) && (w[272] != 0.0)) {
        let noise_metadata_schedule_324_0_e3699: f64 = (w[66] * w[68]);
        let noise_metadata_schedule_324_0_e3700: f64 = (noise_metadata_schedule_324_0_e3699).exp();
        let noise_metadata_schedule_324_0_e3701: f64 = (1.0 - noise_metadata_schedule_324_0_e3700);
        let noise_metadata_schedule_324_0_e3702: f64 = (w[54] * noise_metadata_schedule_324_0_e3701);
        let noise_metadata_schedule_324_0_e3704: f64 = (noise_metadata_schedule_324_0_e3702 / w[68]);
        (noise_metadata_schedule_324_0_e3704,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_324_0_e3706;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_328_0_e3748: f64 = if w[113] > 0.0 { 1.0 } else { 0.0 };
            w[275] = noise_metadata_schedule_328_0_e3748;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_329_0_e3767,) = {
    if (((w[260] == 0.0) && (w[271] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_329_0_e3759: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_329_0_e3760: f64 = (-noise_metadata_schedule_329_0_e3759);
        let noise_metadata_schedule_329_0_e3762: f64 = (noise_metadata_schedule_329_0_e3760 / params[47]);
        let noise_metadata_schedule_329_0_e3763: f64 = (noise_metadata_schedule_329_0_e3762).exp();
        let noise_metadata_schedule_329_0_e3764: f64 = (1.0 - noise_metadata_schedule_329_0_e3763);
        let noise_metadata_schedule_329_0_e3765: f64 = (w[18] * noise_metadata_schedule_329_0_e3764);
        (noise_metadata_schedule_329_0_e3765,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_329_0_e3767;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_330_0_e3781,) = {
    if (((w[260] == 0.0) && (w[271] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_330_0_e3777: f64 = (w[76] - w[183]);
        let noise_metadata_schedule_330_0_e3779: f64 = (noise_metadata_schedule_330_0_e3777 * w[3]);
        (noise_metadata_schedule_330_0_e3779,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_330_0_e3781;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_331_0_e3796,) = {
    if (((w[260] == 0.0) && (w[271] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_331_0_e3791: f64 = (w[80] * w[80]);
        let noise_metadata_schedule_331_0_e3793: f64 = (noise_metadata_schedule_331_0_e3791 + 1.921812);
        let noise_metadata_schedule_331_0_e3794: f64 = (noise_metadata_schedule_331_0_e3793).sqrt();
        (noise_metadata_schedule_331_0_e3794,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_331_0_e3796;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_332_0_e3810,) = {
    if (((w[260] == 0.0) && (w[271] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_332_0_e3806: f64 = (w[80] + w[81]);
        let noise_metadata_schedule_332_0_e3808: f64 = (noise_metadata_schedule_332_0_e3806 * 0.5);
        (noise_metadata_schedule_332_0_e3808,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_332_0_e3810;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_333_0_e3824,) = {
    if (((w[260] == 0.0) && (w[271] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_333_0_e3821: f64 = (w[2] * w[82]);
        let noise_metadata_schedule_333_0_e3822: f64 = (w[76] - noise_metadata_schedule_333_0_e3821);
        (noise_metadata_schedule_333_0_e3822,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_333_0_e3824;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_335_0_e3851,) = {
    if (((w[260] == 0.0) && (w[271] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_335_0_e3847: f64 = (w[77] / w[18]);
        let noise_metadata_schedule_335_0_e3848: f64 = (1.0 - noise_metadata_schedule_335_0_e3847);
        let noise_metadata_schedule_335_0_e3849: f64 = (noise_metadata_schedule_335_0_e3848).ln();
        (noise_metadata_schedule_335_0_e3849,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_335_0_e3851;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_338_0_e3908,) = {
    if (((w[260] == 0.0) && (w[271] == 0.0)) && (w[275] != 0.0)) {
        let noise_metadata_schedule_338_0_e3898: f64 = (1.0 - params[47]);
        let noise_metadata_schedule_338_0_e3899: f64 = (w[78] * noise_metadata_schedule_338_0_e3898);
        let noise_metadata_schedule_338_0_e3900: f64 = (noise_metadata_schedule_338_0_e3899).exp();
        let noise_metadata_schedule_338_0_e3901: f64 = (1.0 - noise_metadata_schedule_338_0_e3900);
        let noise_metadata_schedule_338_0_e3902: f64 = (w[18] * noise_metadata_schedule_338_0_e3901);
        let noise_metadata_schedule_338_0_e3905: f64 = (1.0 - params[47]);
        let noise_metadata_schedule_338_0_e3906: f64 = (noise_metadata_schedule_338_0_e3902 / noise_metadata_schedule_338_0_e3905);
        (noise_metadata_schedule_338_0_e3906,)
    } else {
        (w[79],)
    }
};
            w[79] = noise_metadata_schedule_338_0_e3908;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_342_0_e3951: f64 = if params[44] < 100.0 { 1.0 } else { 0.0 };
            w[276] = noise_metadata_schedule_342_0_e3951;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_343_0_e3954: f64 = if w[111] > 0.0 { 1.0 } else { 0.0 };
            w[277] = noise_metadata_schedule_343_0_e3954;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_344_0_e3962,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_344_0_e3960: f64 = (params[43] / 4.0);
        (noise_metadata_schedule_344_0_e3960,)
    } else {
        (w[50],)
    }
};
            w[50] = noise_metadata_schedule_344_0_e3962;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_345_0_e3970,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_345_0_e3968: f64 = (params[44] - w[17]);
        (noise_metadata_schedule_345_0_e3968,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_345_0_e3970;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_346_0_e3985,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_346_0_e3977: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_346_0_e3978: f64 = (-noise_metadata_schedule_346_0_e3977);
        let noise_metadata_schedule_346_0_e3980: f64 = (noise_metadata_schedule_346_0_e3978 / params[43]);
        let noise_metadata_schedule_346_0_e3981: f64 = (noise_metadata_schedule_346_0_e3980).exp();
        let noise_metadata_schedule_346_0_e3982: f64 = (1.0 - noise_metadata_schedule_346_0_e3981);
        let noise_metadata_schedule_346_0_e3983: f64 = (w[17] * noise_metadata_schedule_346_0_e3982);
        (noise_metadata_schedule_346_0_e3983,)
    } else {
        (w[52],)
    }
};
            w[52] = noise_metadata_schedule_346_0_e3985;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_347_0_e3993,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_347_0_e3991: f64 = (2.4 * w[111]);
        (noise_metadata_schedule_347_0_e3991,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_347_0_e3993;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_348_0_e4009,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_348_0_e4000: f64 = (w[50] - params[43]);
        let noise_metadata_schedule_348_0_e4003: f64 = (params[44] / w[17]);
        let noise_metadata_schedule_348_0_e4004: f64 = (noise_metadata_schedule_348_0_e4003).ln();
        let noise_metadata_schedule_348_0_e4005: f64 = (noise_metadata_schedule_348_0_e4000 * noise_metadata_schedule_348_0_e4004);
        let noise_metadata_schedule_348_0_e4006: f64 = (noise_metadata_schedule_348_0_e4005).exp();
        let noise_metadata_schedule_348_0_e4007: f64 = (w[111] * noise_metadata_schedule_348_0_e4006);
        (noise_metadata_schedule_348_0_e4007,)
    } else {
        (w[54],)
    }
};
            w[54] = noise_metadata_schedule_348_0_e4009;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_349_0_e4019,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_349_0_e4015: f64 = (w[52] - w[184]);
        let noise_metadata_schedule_349_0_e4017: f64 = (noise_metadata_schedule_349_0_e4015 * w[3]);
        (noise_metadata_schedule_349_0_e4017,)
    } else {
        (w[56],)
    }
};
            w[56] = noise_metadata_schedule_349_0_e4019;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_350_0_e4022: f64 = if w[56] < 80.0 { 1.0 } else { 0.0 };
            w[278] = noise_metadata_schedule_350_0_e4022;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_351_0_e4031,) = {
    if (((w[276] != 0.0) && (w[277] != 0.0)) && (w[278] != 0.0)) {
        let noise_metadata_schedule_351_0_e4029: f64 = (w[56]).exp();
        (noise_metadata_schedule_351_0_e4029,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_351_0_e4031;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_353_0_e4058,) = {
    if (((w[276] != 0.0) && (w[277] != 0.0)) && (w[278] != 0.0)) {
        let noise_metadata_schedule_353_0_e4053: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_353_0_e4054: f64 = (noise_metadata_schedule_353_0_e4053).ln();
        let noise_metadata_schedule_353_0_e4055: f64 = (w[2] * noise_metadata_schedule_353_0_e4054);
        let noise_metadata_schedule_353_0_e4056: f64 = (w[52] - noise_metadata_schedule_353_0_e4055);
        (noise_metadata_schedule_353_0_e4056,)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_353_0_e4058;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_355_0_e4076,) = {
    if (((w[276] != 0.0) && (w[277] != 0.0)) && (w[278] == 0.0)) {
        (w[184],)
    } else {
        (w[58],)
    }
};
            w[58] = noise_metadata_schedule_355_0_e4076;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_356_0_e4088,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_356_0_e4082: f64 = (0.1 * w[51]);
        let noise_metadata_schedule_356_0_e4085: f64 = (4.0 * w[2]);
        let noise_metadata_schedule_356_0_e4086: f64 = (noise_metadata_schedule_356_0_e4082 + noise_metadata_schedule_356_0_e4085);
        (noise_metadata_schedule_356_0_e4086,)
    } else {
        (w[55],)
    }
};
            w[55] = noise_metadata_schedule_356_0_e4088;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_357_0_e4098,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_357_0_e4094: f64 = (w[51] + w[58]);
        let noise_metadata_schedule_357_0_e4096: f64 = (noise_metadata_schedule_357_0_e4094 / w[55]);
        (noise_metadata_schedule_357_0_e4096,)
    } else {
        (w[59],)
    }
};
            w[59] = noise_metadata_schedule_357_0_e4098;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_358_0_e4101: f64 = if w[59] < 80.0 { 1.0 } else { 0.0 };
            w[279] = noise_metadata_schedule_358_0_e4101;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_359_0_e4110,) = {
    if (((w[276] != 0.0) && (w[277] != 0.0)) && (w[279] != 0.0)) {
        let noise_metadata_schedule_359_0_e4108: f64 = (w[59]).exp();
        (noise_metadata_schedule_359_0_e4108,)
    } else {
        (w[57],)
    }
};
            w[57] = noise_metadata_schedule_359_0_e4110;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_361_0_e4146,) = {
    if (((w[276] != 0.0) && (w[277] != 0.0)) && (w[279] != 0.0)) {
        let noise_metadata_schedule_361_0_e4129: f64 = (-w[51]);
        let noise_metadata_schedule_361_0_e4133: f64 = (1.0 + w[57]);
        let noise_metadata_schedule_361_0_e4134: f64 = (noise_metadata_schedule_361_0_e4133).ln();
        let noise_metadata_schedule_361_0_e4137: f64 = (w[51] + w[52]);
        let noise_metadata_schedule_361_0_e4138: f64 = (-noise_metadata_schedule_361_0_e4137);
        let noise_metadata_schedule_361_0_e4140: f64 = (noise_metadata_schedule_361_0_e4138 / w[55]);
        let noise_metadata_schedule_361_0_e4141: f64 = (noise_metadata_schedule_361_0_e4140).exp();
        let noise_metadata_schedule_361_0_e4142: f64 = (noise_metadata_schedule_361_0_e4134 - noise_metadata_schedule_361_0_e4141);
        let noise_metadata_schedule_361_0_e4143: f64 = (w[55] * noise_metadata_schedule_361_0_e4142);
        let noise_metadata_schedule_361_0_e4144: f64 = (noise_metadata_schedule_361_0_e4129 + noise_metadata_schedule_361_0_e4143);
        (noise_metadata_schedule_361_0_e4144,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_361_0_e4146;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_363_0_e4164,) = {
    if (((w[276] != 0.0) && (w[277] != 0.0)) && (w[279] == 0.0)) {
        (w[58],)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_363_0_e4164;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_364_0_e4172,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_364_0_e4170: f64 = (w[184] - w[58]);
        (noise_metadata_schedule_364_0_e4170,)
    } else {
        (w[61],)
    }
};
            w[61] = noise_metadata_schedule_364_0_e4172;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_365_0_e4183,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_365_0_e4179: f64 = (w[58] / w[17]);
        let noise_metadata_schedule_365_0_e4180: f64 = (1.0 - noise_metadata_schedule_365_0_e4179);
        let noise_metadata_schedule_365_0_e4181: f64 = (noise_metadata_schedule_365_0_e4180).ln();
        (noise_metadata_schedule_365_0_e4181,)
    } else {
        (w[65],)
    }
};
            w[65] = noise_metadata_schedule_365_0_e4183;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_366_0_e4194,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_366_0_e4190: f64 = (w[60] / w[17]);
        let noise_metadata_schedule_366_0_e4191: f64 = (1.0 - noise_metadata_schedule_366_0_e4190);
        let noise_metadata_schedule_366_0_e4192: f64 = (noise_metadata_schedule_366_0_e4191).ln();
        (noise_metadata_schedule_366_0_e4192,)
    } else {
        (w[66],)
    }
};
            w[66] = noise_metadata_schedule_366_0_e4194;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_367_0_e4202,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_367_0_e4200: f64 = (1.0 - params[43]);
        (noise_metadata_schedule_367_0_e4200,)
    } else {
        (w[67],)
    }
};
            w[67] = noise_metadata_schedule_367_0_e4202;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_368_0_e4210,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_368_0_e4208: f64 = (1.0 - w[50]);
        (noise_metadata_schedule_368_0_e4208,)
    } else {
        (w[68],)
    }
};
            w[68] = noise_metadata_schedule_368_0_e4210;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_373_0_e4277,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_373_0_e4270: f64 = (w[66] * w[67]);
        let noise_metadata_schedule_373_0_e4271: f64 = (noise_metadata_schedule_373_0_e4270).exp();
        let noise_metadata_schedule_373_0_e4272: f64 = (1.0 - noise_metadata_schedule_373_0_e4271);
        let noise_metadata_schedule_373_0_e4273: f64 = (w[111] * noise_metadata_schedule_373_0_e4272);
        let noise_metadata_schedule_373_0_e4275: f64 = (noise_metadata_schedule_373_0_e4273 / w[67]);
        (noise_metadata_schedule_373_0_e4275,)
    } else {
        (w[62],)
    }
};
            w[62] = noise_metadata_schedule_373_0_e4277;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_374_0_e4292,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_374_0_e4285: f64 = (w[65] * w[68]);
        let noise_metadata_schedule_374_0_e4286: f64 = (noise_metadata_schedule_374_0_e4285).exp();
        let noise_metadata_schedule_374_0_e4287: f64 = (1.0 - noise_metadata_schedule_374_0_e4286);
        let noise_metadata_schedule_374_0_e4288: f64 = (w[54] * noise_metadata_schedule_374_0_e4287);
        let noise_metadata_schedule_374_0_e4290: f64 = (noise_metadata_schedule_374_0_e4288 / w[68]);
        (noise_metadata_schedule_374_0_e4290,)
    } else {
        (w[63],)
    }
};
            w[63] = noise_metadata_schedule_374_0_e4292;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_375_0_e4307,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_375_0_e4300: f64 = (w[66] * w[68]);
        let noise_metadata_schedule_375_0_e4301: f64 = (noise_metadata_schedule_375_0_e4300).exp();
        let noise_metadata_schedule_375_0_e4302: f64 = (1.0 - noise_metadata_schedule_375_0_e4301);
        let noise_metadata_schedule_375_0_e4303: f64 = (w[54] * noise_metadata_schedule_375_0_e4302);
        let noise_metadata_schedule_375_0_e4305: f64 = (noise_metadata_schedule_375_0_e4303 / w[68]);
        (noise_metadata_schedule_375_0_e4305,)
    } else {
        (w[64],)
    }
};
            w[64] = noise_metadata_schedule_375_0_e4307;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_376_0_e4323,) = {
    if ((w[276] != 0.0) && (w[277] != 0.0)) {
        let noise_metadata_schedule_376_0_e4313: f64 = (w[62] + w[63]);
        let noise_metadata_schedule_376_0_e4315: f64 = (noise_metadata_schedule_376_0_e4313 - w[64]);
        let noise_metadata_schedule_376_0_e4317: f64 = (noise_metadata_schedule_376_0_e4315 * w[17]);
        let noise_metadata_schedule_376_0_e4320: f64 = (w[53] * w[61]);
        let noise_metadata_schedule_376_0_e4321: f64 = (noise_metadata_schedule_376_0_e4317 + noise_metadata_schedule_376_0_e4320);
        (noise_metadata_schedule_376_0_e4321,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_376_0_e4323;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_378_0_e4337,) = {
    if ((w[276] != 0.0) && (w[277] == 0.0)) {
        (0.0,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_378_0_e4337;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_379_0_e4340: f64 = if w[111] > 0.0 { 1.0 } else { 0.0 };
            w[280] = noise_metadata_schedule_379_0_e4340;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_380_0_e4356,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_380_0_e4348: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_380_0_e4349: f64 = (-noise_metadata_schedule_380_0_e4348);
        let noise_metadata_schedule_380_0_e4351: f64 = (noise_metadata_schedule_380_0_e4349 / params[43]);
        let noise_metadata_schedule_380_0_e4352: f64 = (noise_metadata_schedule_380_0_e4351).exp();
        let noise_metadata_schedule_380_0_e4353: f64 = (1.0 - noise_metadata_schedule_380_0_e4352);
        let noise_metadata_schedule_380_0_e4354: f64 = (w[17] * noise_metadata_schedule_380_0_e4353);
        (noise_metadata_schedule_380_0_e4354,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_380_0_e4356;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_381_0_e4367,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_381_0_e4363: f64 = (w[76] - w[184]);
        let noise_metadata_schedule_381_0_e4365: f64 = (noise_metadata_schedule_381_0_e4363 * w[3]);
        (noise_metadata_schedule_381_0_e4365,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_381_0_e4367;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_382_0_e4379,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_382_0_e4374: f64 = (w[80] * w[80]);
        let noise_metadata_schedule_382_0_e4376: f64 = (noise_metadata_schedule_382_0_e4374 + 1.921812);
        let noise_metadata_schedule_382_0_e4377: f64 = (noise_metadata_schedule_382_0_e4376).sqrt();
        (noise_metadata_schedule_382_0_e4377,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_382_0_e4379;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_383_0_e4390,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_383_0_e4386: f64 = (w[80] + w[81]);
        let noise_metadata_schedule_383_0_e4388: f64 = (noise_metadata_schedule_383_0_e4386 * 0.5);
        (noise_metadata_schedule_383_0_e4388,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_383_0_e4390;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_384_0_e4401,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_384_0_e4398: f64 = (w[2] * w[82]);
        let noise_metadata_schedule_384_0_e4399: f64 = (w[76] - noise_metadata_schedule_384_0_e4398);
        (noise_metadata_schedule_384_0_e4399,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_384_0_e4401;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_386_0_e4422,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_386_0_e4418: f64 = (w[77] / w[17]);
        let noise_metadata_schedule_386_0_e4419: f64 = (1.0 - noise_metadata_schedule_386_0_e4418);
        let noise_metadata_schedule_386_0_e4420: f64 = (noise_metadata_schedule_386_0_e4419).ln();
        (noise_metadata_schedule_386_0_e4420,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_386_0_e4422;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_389_0_e4470,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_389_0_e4460: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_389_0_e4461: f64 = (w[78] * noise_metadata_schedule_389_0_e4460);
        let noise_metadata_schedule_389_0_e4462: f64 = (noise_metadata_schedule_389_0_e4461).exp();
        let noise_metadata_schedule_389_0_e4463: f64 = (1.0 - noise_metadata_schedule_389_0_e4462);
        let noise_metadata_schedule_389_0_e4464: f64 = (w[17] * noise_metadata_schedule_389_0_e4463);
        let noise_metadata_schedule_389_0_e4467: f64 = (1.0 - params[43]);
        let noise_metadata_schedule_389_0_e4468: f64 = (noise_metadata_schedule_389_0_e4464 / noise_metadata_schedule_389_0_e4467);
        (noise_metadata_schedule_389_0_e4468,)
    } else {
        (w[79],)
    }
};
            w[79] = noise_metadata_schedule_389_0_e4470;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_390_0_e4485,) = {
    if ((w[276] == 0.0) && (w[280] != 0.0)) {
        let noise_metadata_schedule_390_0_e4480: f64 = (w[184] - w[77]);
        let noise_metadata_schedule_390_0_e4481: f64 = (2.4 * noise_metadata_schedule_390_0_e4480);
        let noise_metadata_schedule_390_0_e4482: f64 = (w[79] + noise_metadata_schedule_390_0_e4481);
        let noise_metadata_schedule_390_0_e4483: f64 = (w[111] * noise_metadata_schedule_390_0_e4482);
        (noise_metadata_schedule_390_0_e4483,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_390_0_e4485;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_392_0_e4501,) = {
    if ((w[276] == 0.0) && (w[280] == 0.0)) {
        (0.0,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_392_0_e4501;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_394_0_e4507: f64 = if w[111] > 0.0 { 1.0 } else { 0.0 };
            w[281] = noise_metadata_schedule_394_0_e4507;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_395_0_e4520,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_395_0_e4512: f64 = (2.4_f64).ln();
        let noise_metadata_schedule_395_0_e4513: f64 = (-noise_metadata_schedule_395_0_e4512);
        let noise_metadata_schedule_395_0_e4515: f64 = (noise_metadata_schedule_395_0_e4513 / params[43]);
        let noise_metadata_schedule_395_0_e4516: f64 = (noise_metadata_schedule_395_0_e4515).exp();
        let noise_metadata_schedule_395_0_e4517: f64 = (1.0 - noise_metadata_schedule_395_0_e4516);
        let noise_metadata_schedule_395_0_e4518: f64 = (w[17] * noise_metadata_schedule_395_0_e4517);
        (noise_metadata_schedule_395_0_e4518,)
    } else {
        (w[282],)
    }
};
            w[282] = noise_metadata_schedule_395_0_e4520;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_396_0_e4528,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_396_0_e4524: f64 = (w[282] - w[184]);
        let noise_metadata_schedule_396_0_e4526: f64 = (noise_metadata_schedule_396_0_e4524 * w[3]);
        (noise_metadata_schedule_396_0_e4526,)
    } else {
        (w[283],)
    }
};
            w[283] = noise_metadata_schedule_396_0_e4528;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_397_0_e4537,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_397_0_e4532: f64 = (w[283] * w[283]);
        let noise_metadata_schedule_397_0_e4534: f64 = (noise_metadata_schedule_397_0_e4532 + 1.921812);
        let noise_metadata_schedule_397_0_e4535: f64 = (noise_metadata_schedule_397_0_e4534).sqrt();
        (noise_metadata_schedule_397_0_e4535,)
    } else {
        (w[284],)
    }
};
            w[284] = noise_metadata_schedule_397_0_e4537;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_398_0_e4545,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_398_0_e4541: f64 = (w[283] + w[284]);
        let noise_metadata_schedule_398_0_e4543: f64 = (noise_metadata_schedule_398_0_e4541 * 0.5);
        (noise_metadata_schedule_398_0_e4543,)
    } else {
        (w[285],)
    }
};
            w[285] = noise_metadata_schedule_398_0_e4545;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_399_0_e4553,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_399_0_e4550: f64 = (w[2] * w[285]);
        let noise_metadata_schedule_399_0_e4551: f64 = (w[282] - noise_metadata_schedule_399_0_e4550);
        (noise_metadata_schedule_399_0_e4551,)
    } else {
        (w[286],)
    }
};
            w[286] = noise_metadata_schedule_399_0_e4553;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_400_0_e4559,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_400_0_e4557: f64 = (w[285] / w[284]);
        (noise_metadata_schedule_400_0_e4557,)
    } else {
        (w[287],)
    }
};
            w[287] = noise_metadata_schedule_400_0_e4559;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_401_0_e4584,) = {
    if (w[281] != 0.0) {
        let noise_metadata_schedule_401_0_e4563: f64 = (-params[43]);
        let noise_metadata_schedule_401_0_e4567: f64 = (w[286] / w[17]);
        let noise_metadata_schedule_401_0_e4568: f64 = (1.0 - noise_metadata_schedule_401_0_e4567);
        let noise_metadata_schedule_401_0_e4569: f64 = (noise_metadata_schedule_401_0_e4568).ln();
        let noise_metadata_schedule_401_0_e4570: f64 = (noise_metadata_schedule_401_0_e4563 * noise_metadata_schedule_401_0_e4569);
        let noise_metadata_schedule_401_0_e4571: f64 = (noise_metadata_schedule_401_0_e4570).exp();
        let noise_metadata_schedule_401_0_e4572: f64 = (w[111] * noise_metadata_schedule_401_0_e4571);
        let noise_metadata_schedule_401_0_e4574: f64 = (noise_metadata_schedule_401_0_e4572 * w[287]);
        let noise_metadata_schedule_401_0_e4577: f64 = (2.4 * w[111]);
        let noise_metadata_schedule_401_0_e4580: f64 = (1.0 - w[287]);
        let noise_metadata_schedule_401_0_e4581: f64 = (noise_metadata_schedule_401_0_e4577 * noise_metadata_schedule_401_0_e4580);
        let noise_metadata_schedule_401_0_e4582: f64 = (noise_metadata_schedule_401_0_e4574 + noise_metadata_schedule_401_0_e4581);
        (noise_metadata_schedule_401_0_e4582,)
    } else {
        (w[107],)
    }
};
            w[107] = noise_metadata_schedule_401_0_e4584;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_402_0_e4589,) = {
    if (w[281] == 0.0) {
        (0.0,)
    } else {
        (w[107],)
    }
};
            w[107] = noise_metadata_schedule_402_0_e4589;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_403_0_e4592: f64 = if params[65] > 0.0 { 1.0 } else { 0.0 };
            w[288] = noise_metadata_schedule_403_0_e4592;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_404_0_e4598,) = {
    if (w[288] != 0.0) {
        let noise_metadata_schedule_404_0_e4596: f64 = (w[38] - w[184]);
        (noise_metadata_schedule_404_0_e4596,)
    } else {
        (w[143],)
    }
};
            w[143] = noise_metadata_schedule_404_0_e4598;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_405_0_e4605,) = {
    if (w[288] == 0.0) {
        let noise_metadata_schedule_405_0_e4603: f64 = (w[186] - w[34]);
        (noise_metadata_schedule_405_0_e4603,)
    } else {
        (w[143],)
    }
};
            w[143] = noise_metadata_schedule_405_0_e4605;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_406_0_e4608: f64 = (w[143] * w[3]);
            let noise_metadata_schedule_406_0_e4610: f64 = (noise_metadata_schedule_406_0_e4608 - 1.0);
            w[289] = noise_metadata_schedule_406_0_e4610;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_407_0_e4615: f64 = (w[289] * w[289]);
            let noise_metadata_schedule_407_0_e4617: f64 = (noise_metadata_schedule_407_0_e4615 + 1.921812);
            let noise_metadata_schedule_407_0_e4618: f64 = (noise_metadata_schedule_407_0_e4617).sqrt();
            let noise_metadata_schedule_407_0_e4619: f64 = (w[289] + noise_metadata_schedule_407_0_e4618);
            let noise_metadata_schedule_407_0_e4621: f64 = (noise_metadata_schedule_407_0_e4619 / 2.0);
            let noise_metadata_schedule_407_0_e4622: f64 = (1.0 + noise_metadata_schedule_407_0_e4621);
            let noise_metadata_schedule_407_0_e4624: f64 = (noise_metadata_schedule_407_0_e4622 * w[2]);
            w[290] = noise_metadata_schedule_407_0_e4624;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_408_0_e4627: f64 = (w[290] / w[33]);
            w[291] = noise_metadata_schedule_408_0_e4627;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_409_0_e4630: f64 = (w[290] * w[32]);
            w[292] = noise_metadata_schedule_409_0_e4630;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_410_0_e4634: f64 = (w[291]).ln();
            let noise_metadata_schedule_410_0_e4635: f64 = (params[67] * noise_metadata_schedule_410_0_e4634);
            let noise_metadata_schedule_410_0_e4636: f64 = (noise_metadata_schedule_410_0_e4635).exp();
            let noise_metadata_schedule_410_0_e4637: f64 = (1.0 + noise_metadata_schedule_410_0_e4636);
            let noise_metadata_schedule_410_0_e4638: f64 = (noise_metadata_schedule_410_0_e4637).ln();
            let noise_metadata_schedule_410_0_e4640: f64 = (noise_metadata_schedule_410_0_e4638 / params[67]);
            let noise_metadata_schedule_410_0_e4641: f64 = (noise_metadata_schedule_410_0_e4640).exp();
            w[293] = noise_metadata_schedule_410_0_e4641;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_411_0_e4644: f64 = (w[292] / w[293]);
            w[294] = noise_metadata_schedule_411_0_e4644;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_412_0_e4647: f64 = (w[290] - w[33]);
            let noise_metadata_schedule_412_0_e4649: f64 = (noise_metadata_schedule_412_0_e4647 / params[63]);
            w[295] = noise_metadata_schedule_412_0_e4649;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_413_0_e4656: f64 = (w[295] * w[295]);
            let noise_metadata_schedule_413_0_e4658: f64 = (noise_metadata_schedule_413_0_e4656 + params[66]);
            let noise_metadata_schedule_413_0_e4659: f64 = (noise_metadata_schedule_413_0_e4658).sqrt();
            let noise_metadata_schedule_413_0_e4660: f64 = (w[295] + noise_metadata_schedule_413_0_e4659);
            let noise_metadata_schedule_413_0_e4661: f64 = (0.5 * noise_metadata_schedule_413_0_e4660);
            let noise_metadata_schedule_413_0_e4662: f64 = (1.0 + noise_metadata_schedule_413_0_e4661);
            let noise_metadata_schedule_413_0_e4663: f64 = (w[294] * noise_metadata_schedule_413_0_e4662);
            w[142] = noise_metadata_schedule_413_0_e4663;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_414_0_e4670: f64 = if ((w[107] > 0.0) && (w[111] > 0.0)) { 1.0 } else { 0.0 };
            w[296] = noise_metadata_schedule_414_0_e4670;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_415_0_e4676,) = {
    if (w[296] != 0.0) {
        let noise_metadata_schedule_415_0_e4674: f64 = (w[111] / w[107]);
        (noise_metadata_schedule_415_0_e4674,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_415_0_e4676;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_416_0_e4682,) = {
    if (w[296] != 0.0) {
        let noise_metadata_schedule_416_0_e4680: f64 = (w[103] / w[111]);
        (noise_metadata_schedule_416_0_e4680,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_416_0_e4682;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_417_0_e4687,) = {
    if (w[296] == 0.0) {
        (1.0,)
    } else {
        (w[114],)
    }
};
            w[114] = noise_metadata_schedule_417_0_e4687;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_418_0_e4692,) = {
    if (w[296] == 0.0) {
        (0.0,)
    } else {
        (w[103],)
    }
};
            w[103] = noise_metadata_schedule_418_0_e4692;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_419_0_e4695: f64 = if w[23] > 0.0 { 1.0 } else { 0.0 };
            w[297] = noise_metadata_schedule_419_0_e4695;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_420_0_e4708,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_420_0_e4700: f64 = (w[43]).ln();
        let noise_metadata_schedule_420_0_e4701: f64 = (-noise_metadata_schedule_420_0_e4700);
        let noise_metadata_schedule_420_0_e4703: f64 = (noise_metadata_schedule_420_0_e4701 / params[36]);
        let noise_metadata_schedule_420_0_e4704: f64 = (noise_metadata_schedule_420_0_e4703).exp();
        let noise_metadata_schedule_420_0_e4705: f64 = (1.0 - noise_metadata_schedule_420_0_e4704);
        let noise_metadata_schedule_420_0_e4706: f64 = (w[16] * noise_metadata_schedule_420_0_e4705);
        (noise_metadata_schedule_420_0_e4706,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_420_0_e4708;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_421_0_e4716,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_421_0_e4712: f64 = (w[76] - w[185]);
        let noise_metadata_schedule_421_0_e4714: f64 = (noise_metadata_schedule_421_0_e4712 * w[3]);
        (noise_metadata_schedule_421_0_e4714,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_421_0_e4716;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_422_0_e4725,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_422_0_e4720: f64 = (w[80] * w[80]);
        let noise_metadata_schedule_422_0_e4722: f64 = (noise_metadata_schedule_422_0_e4720 + 1.921812);
        let noise_metadata_schedule_422_0_e4723: f64 = (noise_metadata_schedule_422_0_e4722).sqrt();
        (noise_metadata_schedule_422_0_e4723,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_422_0_e4725;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_423_0_e4733,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_423_0_e4729: f64 = (w[80] + w[81]);
        let noise_metadata_schedule_423_0_e4731: f64 = (noise_metadata_schedule_423_0_e4729 * 0.5);
        (noise_metadata_schedule_423_0_e4731,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_423_0_e4733;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_424_0_e4741,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_424_0_e4738: f64 = (w[2] * w[82]);
        let noise_metadata_schedule_424_0_e4739: f64 = (w[76] - noise_metadata_schedule_424_0_e4738);
        (noise_metadata_schedule_424_0_e4739,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_424_0_e4741;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_426_0_e4756,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_426_0_e4752: f64 = (w[77] / w[16]);
        let noise_metadata_schedule_426_0_e4753: f64 = (1.0 - noise_metadata_schedule_426_0_e4752);
        let noise_metadata_schedule_426_0_e4754: f64 = (noise_metadata_schedule_426_0_e4753).ln();
        (noise_metadata_schedule_426_0_e4754,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_426_0_e4756;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_429_0_e4795,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_429_0_e4785: f64 = (1.0 - params[36]);
        let noise_metadata_schedule_429_0_e4786: f64 = (w[78] * noise_metadata_schedule_429_0_e4785);
        let noise_metadata_schedule_429_0_e4787: f64 = (noise_metadata_schedule_429_0_e4786).exp();
        let noise_metadata_schedule_429_0_e4788: f64 = (1.0 - noise_metadata_schedule_429_0_e4787);
        let noise_metadata_schedule_429_0_e4789: f64 = (w[16] * noise_metadata_schedule_429_0_e4788);
        let noise_metadata_schedule_429_0_e4792: f64 = (1.0 - params[36]);
        let noise_metadata_schedule_429_0_e4793: f64 = (noise_metadata_schedule_429_0_e4789 / noise_metadata_schedule_429_0_e4792);
        (noise_metadata_schedule_429_0_e4793,)
    } else {
        (w[79],)
    }
};
            w[79] = noise_metadata_schedule_429_0_e4795;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_430_0_e4807,) = {
    if (w[297] != 0.0) {
        let noise_metadata_schedule_430_0_e4802: f64 = (w[185] - w[77]);
        let noise_metadata_schedule_430_0_e4803: f64 = (w[43] * noise_metadata_schedule_430_0_e4802);
        let noise_metadata_schedule_430_0_e4804: f64 = (w[79] + noise_metadata_schedule_430_0_e4803);
        let noise_metadata_schedule_430_0_e4805: f64 = (w[23] * noise_metadata_schedule_430_0_e4804);
        (noise_metadata_schedule_430_0_e4805,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_430_0_e4807;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_432_0_e4817,) = {
    if (w[297] == 0.0) {
        (0.0,)
    } else {
        (w[98],)
    }
};
            w[98] = noise_metadata_schedule_432_0_e4817;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_433_0_e4820: f64 = (w[98] / w[23]);
            w[102] = noise_metadata_schedule_433_0_e4820;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_434_0_e4823: f64 = if params[0] <= 200.0 { 1.0 } else { 0.0 };
            w[298] = noise_metadata_schedule_434_0_e4823;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_435_0_e4826: f64 = if w[26] > 0.0 { 1.0 } else { 0.0 };
            w[299] = noise_metadata_schedule_435_0_e4826;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_436_0_e4841,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_436_0_e4833: f64 = (w[44]).ln();
        let noise_metadata_schedule_436_0_e4834: f64 = (-noise_metadata_schedule_436_0_e4833);
        let noise_metadata_schedule_436_0_e4836: f64 = (noise_metadata_schedule_436_0_e4834 / params[39]);
        let noise_metadata_schedule_436_0_e4837: f64 = (noise_metadata_schedule_436_0_e4836).exp();
        let noise_metadata_schedule_436_0_e4838: f64 = (1.0 - noise_metadata_schedule_436_0_e4837);
        let noise_metadata_schedule_436_0_e4839: f64 = (w[22] * noise_metadata_schedule_436_0_e4838);
        (noise_metadata_schedule_436_0_e4839,)
    } else {
        (w[76],)
    }
};
            w[76] = noise_metadata_schedule_436_0_e4841;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_437_0_e4851,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_437_0_e4847: f64 = (w[76] - w[185]);
        let noise_metadata_schedule_437_0_e4849: f64 = (noise_metadata_schedule_437_0_e4847 * w[3]);
        (noise_metadata_schedule_437_0_e4849,)
    } else {
        (w[80],)
    }
};
            w[80] = noise_metadata_schedule_437_0_e4851;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_438_0_e4862,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_438_0_e4857: f64 = (w[80] * w[80]);
        let noise_metadata_schedule_438_0_e4859: f64 = (noise_metadata_schedule_438_0_e4857 + 1.921812);
        let noise_metadata_schedule_438_0_e4860: f64 = (noise_metadata_schedule_438_0_e4859).sqrt();
        (noise_metadata_schedule_438_0_e4860,)
    } else {
        (w[81],)
    }
};
            w[81] = noise_metadata_schedule_438_0_e4862;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_439_0_e4872,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_439_0_e4868: f64 = (w[80] + w[81]);
        let noise_metadata_schedule_439_0_e4870: f64 = (noise_metadata_schedule_439_0_e4868 * 0.5);
        (noise_metadata_schedule_439_0_e4870,)
    } else {
        (w[82],)
    }
};
            w[82] = noise_metadata_schedule_439_0_e4872;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_440_0_e4882,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_440_0_e4879: f64 = (w[2] * w[82]);
        let noise_metadata_schedule_440_0_e4880: f64 = (w[76] - noise_metadata_schedule_440_0_e4879);
        (noise_metadata_schedule_440_0_e4880,)
    } else {
        (w[77],)
    }
};
            w[77] = noise_metadata_schedule_440_0_e4882;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_442_0_e4901,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_442_0_e4897: f64 = (w[77] / w[22]);
        let noise_metadata_schedule_442_0_e4898: f64 = (1.0 - noise_metadata_schedule_442_0_e4897);
        let noise_metadata_schedule_442_0_e4899: f64 = (noise_metadata_schedule_442_0_e4898).ln();
        (noise_metadata_schedule_442_0_e4899,)
    } else {
        (w[78],)
    }
};
            w[78] = noise_metadata_schedule_442_0_e4901;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_445_0_e4946,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_445_0_e4936: f64 = (1.0 - params[39]);
        let noise_metadata_schedule_445_0_e4937: f64 = (w[78] * noise_metadata_schedule_445_0_e4936);
        let noise_metadata_schedule_445_0_e4938: f64 = (noise_metadata_schedule_445_0_e4937).exp();
        let noise_metadata_schedule_445_0_e4939: f64 = (1.0 - noise_metadata_schedule_445_0_e4938);
        let noise_metadata_schedule_445_0_e4940: f64 = (w[22] * noise_metadata_schedule_445_0_e4939);
        let noise_metadata_schedule_445_0_e4943: f64 = (1.0 - params[39]);
        let noise_metadata_schedule_445_0_e4944: f64 = (noise_metadata_schedule_445_0_e4940 / noise_metadata_schedule_445_0_e4943);
        (noise_metadata_schedule_445_0_e4944,)
    } else {
        (w[79],)
    }
};
            w[79] = noise_metadata_schedule_445_0_e4946;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_446_0_e4960,) = {
    if ((w[298] != 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_446_0_e4955: f64 = (w[185] - w[77]);
        let noise_metadata_schedule_446_0_e4956: f64 = (w[44] * noise_metadata_schedule_446_0_e4955);
        let noise_metadata_schedule_446_0_e4957: f64 = (w[79] + noise_metadata_schedule_446_0_e4956);
        let noise_metadata_schedule_446_0_e4958: f64 = (w[26] * noise_metadata_schedule_446_0_e4957);
        (noise_metadata_schedule_446_0_e4958,)
    } else {
        (w[100],)
    }
};
            w[100] = noise_metadata_schedule_446_0_e4960;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_448_0_e4974,) = {
    if ((w[298] != 0.0) && (w[299] == 0.0)) {
        (0.0,)
    } else {
        (w[100],)
    }
};
            w[100] = noise_metadata_schedule_448_0_e4974;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_449_0_e4980,) = {
    if (w[298] != 0.0) {
        let noise_metadata_schedule_449_0_e4978: f64 = (w[100] / w[26]);
        (noise_metadata_schedule_449_0_e4978,)
    } else {
        (w[101],)
    }
};
            w[101] = noise_metadata_schedule_449_0_e4980;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_450_0_e4984,) = {
    if (w[298] != 0.0) {
        (w[22],)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_450_0_e4984;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_451_0_e4988,) = {
    if (w[298] != 0.0) {
        (params[39],)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_451_0_e4988;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_452_0_e4993,) = {
    if (w[298] == 0.0) {
        (w[102],)
    } else {
        (w[101],)
    }
};
            w[101] = noise_metadata_schedule_452_0_e4993;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_453_0_e4998,) = {
    if (w[298] == 0.0) {
        (w[16],)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_453_0_e4998;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_454_0_e5003,) = {
    if (w[298] == 0.0) {
        (params[36],)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_454_0_e5003;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_455_0_e5006: f64 = if params[7] == 0.0 { 1.0 } else { 0.0 };
            w[300] = noise_metadata_schedule_455_0_e5006;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_456_0_e5010,) = {
    if (w[300] != 0.0) {
        (1.0,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_456_0_e5010;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_457_0_e5017,) = {
    if (w[300] == 0.0) {
        let noise_metadata_schedule_457_0_e5015: f64 = (params[8] * w[2]);
        (noise_metadata_schedule_457_0_e5015,)
    } else {
        (w[301],)
    }
};
            w[301] = noise_metadata_schedule_457_0_e5017;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_458_0_e5026,) = {
    if (w[300] == 0.0) {
        let noise_metadata_schedule_458_0_e5022: f64 = (w[20] - w[185]);
        let noise_metadata_schedule_458_0_e5024: f64 = (noise_metadata_schedule_458_0_e5022 / w[301]);
        (noise_metadata_schedule_458_0_e5024,)
    } else {
        (w[302],)
    }
};
            w[302] = noise_metadata_schedule_458_0_e5026;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_459_0_e5044,) = {
    if (w[300] == 0.0) {
        let noise_metadata_schedule_459_0_e5034: f64 = (w[302] * w[302]);
        let noise_metadata_schedule_459_0_e5036: f64 = (noise_metadata_schedule_459_0_e5034 + 1.921812);
        let noise_metadata_schedule_459_0_e5037: f64 = (noise_metadata_schedule_459_0_e5036).sqrt();
        let noise_metadata_schedule_459_0_e5038: f64 = (w[302] + noise_metadata_schedule_459_0_e5037);
        let noise_metadata_schedule_459_0_e5039: f64 = (w[301] * noise_metadata_schedule_459_0_e5038);
        let noise_metadata_schedule_459_0_e5041: f64 = (noise_metadata_schedule_459_0_e5039 * 0.5);
        let noise_metadata_schedule_459_0_e5042: f64 = (w[20] - noise_metadata_schedule_459_0_e5041);
        (noise_metadata_schedule_459_0_e5042,)
    } else {
        (w[303],)
    }
};
            w[303] = noise_metadata_schedule_459_0_e5044;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_460_0_e5061,) = {
    if (w[300] == 0.0) {
        let noise_metadata_schedule_460_0_e5053: f64 = (w[303] / w[20]);
        let noise_metadata_schedule_460_0_e5054: f64 = (1.0 - noise_metadata_schedule_460_0_e5053);
        let noise_metadata_schedule_460_0_e5055: f64 = (noise_metadata_schedule_460_0_e5054).ln();
        let noise_metadata_schedule_460_0_e5056: f64 = (w[21] * noise_metadata_schedule_460_0_e5055);
        let noise_metadata_schedule_460_0_e5057: f64 = (noise_metadata_schedule_460_0_e5056).exp();
        let noise_metadata_schedule_460_0_e5058: f64 = (1.0 - noise_metadata_schedule_460_0_e5057);
        let noise_metadata_schedule_460_0_e5059: f64 = (w[200] * noise_metadata_schedule_460_0_e5058);
        (noise_metadata_schedule_460_0_e5059,)
    } else {
        (w[304],)
    }
};
            w[304] = noise_metadata_schedule_460_0_e5061;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_461_0_e5063: f64 = (w[304]).abs();
            let noise_metadata_schedule_461_0_e5065: f64 = if noise_metadata_schedule_461_0_e5063 >= 0.001 { 1.0 } else { 0.0 };
            w[305] = noise_metadata_schedule_461_0_e5065;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_462_0_e5077,) = {
    if ((w[300] == 0.0) && (w[305] != 0.0)) {
        let noise_metadata_schedule_462_0_e5071: f64 = (w[304]).exp();
        let noise_metadata_schedule_462_0_e5073: f64 = (noise_metadata_schedule_462_0_e5071 - 1.0);
        let noise_metadata_schedule_462_0_e5075: f64 = (noise_metadata_schedule_462_0_e5073 / w[304]);
        (noise_metadata_schedule_462_0_e5075,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_462_0_e5077;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_463_0_e5089,) = {
    if ((w[300] == 0.0) && (w[305] == 0.0)) {
        let noise_metadata_schedule_463_0_e5086: f64 = (w[304] * 0.5);
        let noise_metadata_schedule_463_0_e5087: f64 = (1.0 + noise_metadata_schedule_463_0_e5086);
        (noise_metadata_schedule_463_0_e5087,)
    } else {
        (w[201],)
    }
};
            w[201] = noise_metadata_schedule_463_0_e5089;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_464_0_e5092: f64 = (w[201] * w[101]);
            w[159] = noise_metadata_schedule_464_0_e5092;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_465_0_e5096: f64 = (w[159] / w[202]);
            let noise_metadata_schedule_465_0_e5097: f64 = (1.0 + noise_metadata_schedule_465_0_e5096);
            let noise_metadata_schedule_465_0_e5100: f64 = (w[103] / params[5]);
            let noise_metadata_schedule_465_0_e5101: f64 = (noise_metadata_schedule_465_0_e5097 + noise_metadata_schedule_465_0_e5100);
            w[116] = noise_metadata_schedule_465_0_e5101;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_466_0_e5104: f64 = (20.0 * w[116]);
            let noise_metadata_schedule_466_0_e5106: f64 = (noise_metadata_schedule_466_0_e5104 - 1.0);
            w[131] = noise_metadata_schedule_466_0_e5106;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_467_0_e5112: f64 = (w[131] * w[131]);
            let noise_metadata_schedule_467_0_e5114: f64 = (noise_metadata_schedule_467_0_e5112 + 1.921812);
            let noise_metadata_schedule_467_0_e5115: f64 = (noise_metadata_schedule_467_0_e5114).sqrt();
            let noise_metadata_schedule_467_0_e5116: f64 = (w[131] + noise_metadata_schedule_467_0_e5115);
            let noise_metadata_schedule_467_0_e5118: f64 = (noise_metadata_schedule_467_0_e5116 / 2.0);
            let noise_metadata_schedule_467_0_e5119: f64 = (1.0 + noise_metadata_schedule_467_0_e5118);
            let noise_metadata_schedule_467_0_e5120: f64 = (0.025 * noise_metadata_schedule_467_0_e5119);
            w[115] = noise_metadata_schedule_467_0_e5120;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_468_0_e5125: f64 = (w[114] - 1.0);
            let noise_metadata_schedule_468_0_e5126: f64 = (params[55] * noise_metadata_schedule_468_0_e5125);
            let noise_metadata_schedule_468_0_e5127: f64 = (w[42] + noise_metadata_schedule_468_0_e5126);
            let noise_metadata_schedule_468_0_e5131: f64 = (1.0 / w[114]);
            let noise_metadata_schedule_468_0_e5133: f64 = (noise_metadata_schedule_468_0_e5131 - 1.0);
            let noise_metadata_schedule_468_0_e5134: f64 = (params[56] * noise_metadata_schedule_468_0_e5133);
            let noise_metadata_schedule_468_0_e5135: f64 = (noise_metadata_schedule_468_0_e5127 + noise_metadata_schedule_468_0_e5134);
            w[117] = noise_metadata_schedule_468_0_e5135;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_469_0_e5138: f64 = if params[10] == 1.0 { 1.0 } else { 0.0 };
            w[306] = noise_metadata_schedule_469_0_e5138;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_470_0_e5146,) = {
    if (w[306] != 0.0) {
        let noise_metadata_schedule_470_0_e5142: f64 = (w[117] / w[42]);
        let noise_metadata_schedule_470_0_e5144: f64 = (noise_metadata_schedule_470_0_e5142 - 1.0);
        (noise_metadata_schedule_470_0_e5144,)
    } else {
        (w[130],)
    }
};
            w[130] = noise_metadata_schedule_470_0_e5146;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_471_0_e5154,) = {
    if (w[306] != 0.0) {
        let noise_metadata_schedule_471_0_e5151: f64 = (1.0 + w[130]);
        let noise_metadata_schedule_471_0_e5152: f64 = (w[15] / noise_metadata_schedule_471_0_e5151);
        (noise_metadata_schedule_471_0_e5152,)
    } else {
        (w[118],)
    }
};
            w[118] = noise_metadata_schedule_471_0_e5154;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_472_0_e5159,) = {
    if (w[306] == 0.0) {
        (w[15],)
    } else {
        (w[118],)
    }
};
            w[118] = noise_metadata_schedule_472_0_e5159;
        }
        if (active[0] & 0x21) != 0 {
            w[119] = params[11];
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_474_0_e5164: f64 = (params[3] * w[2]);
            let noise_metadata_schedule_474_0_e5165: f64 = (w[185] / noise_metadata_schedule_474_0_e5164);
            w[180] = noise_metadata_schedule_474_0_e5165;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_475_0_e5168: f64 = if w[180] > 80.0 { 1.0 } else { 0.0 };
            w[307] = noise_metadata_schedule_475_0_e5168;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_476_0_e5176,) = {
    if (w[307] != 0.0) {
        let noise_metadata_schedule_476_0_e5173: f64 = (w[180] - 80.0);
        let noise_metadata_schedule_476_0_e5174: f64 = (1.0 + noise_metadata_schedule_476_0_e5173);
        (noise_metadata_schedule_476_0_e5174,)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_476_0_e5176;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_477_0_e5180,) = {
    if (w[307] != 0.0) {
        (80.0,)
    } else {
        (w[180],)
    }
};
            w[180] = noise_metadata_schedule_477_0_e5180;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_478_0_e5185,) = {
    if (w[307] == 0.0) {
        (1.0,)
    } else {
        (w[179],)
    }
};
            w[179] = noise_metadata_schedule_478_0_e5185;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_479_0_e5188: f64 = { let limexp_arg = w[180]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_479_0_e5189: f64 = (w[179] * noise_metadata_schedule_479_0_e5188);
            w[179] = noise_metadata_schedule_479_0_e5189;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_480_0_e5192: f64 = (w[11] * w[179]);
            w[120] = noise_metadata_schedule_480_0_e5192;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_481_0_e5196: f64 = (params[4] * w[2]);
            let noise_metadata_schedule_481_0_e5197: f64 = (w[184] / noise_metadata_schedule_481_0_e5196);
            w[182] = noise_metadata_schedule_481_0_e5197;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_482_0_e5200: f64 = if w[182] > 80.0 { 1.0 } else { 0.0 };
            w[308] = noise_metadata_schedule_482_0_e5200;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_483_0_e5208,) = {
    if (w[308] != 0.0) {
        let noise_metadata_schedule_483_0_e5205: f64 = (w[182] - 80.0);
        let noise_metadata_schedule_483_0_e5206: f64 = (1.0 + noise_metadata_schedule_483_0_e5205);
        (noise_metadata_schedule_483_0_e5206,)
    } else {
        (w[181],)
    }
};
            w[181] = noise_metadata_schedule_483_0_e5208;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_484_0_e5212,) = {
    if (w[308] != 0.0) {
        (80.0,)
    } else {
        (w[182],)
    }
};
            w[182] = noise_metadata_schedule_484_0_e5212;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_485_0_e5217,) = {
    if (w[308] == 0.0) {
        (1.0,)
    } else {
        (w[181],)
    }
};
            w[181] = noise_metadata_schedule_485_0_e5217;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_486_0_e5220: f64 = { let limexp_arg = w[182]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
            let noise_metadata_schedule_486_0_e5221: f64 = (w[181] * noise_metadata_schedule_486_0_e5220);
            w[181] = noise_metadata_schedule_486_0_e5221;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_487_0_e5224: f64 = (w[11] * w[181]);
            w[121] = noise_metadata_schedule_487_0_e5224;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_488_0_e5227: f64 = if params[13] != 0.0 { 1.0 } else { 0.0 };
            w[309] = noise_metadata_schedule_488_0_e5227;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_489_0_e5251,) = {
    if (w[309] != 0.0) {
        let noise_metadata_schedule_489_0_e5231: f64 = (w[120] / w[118]);
        let noise_metadata_schedule_489_0_e5234: f64 = (w[121] / w[119]);
        let noise_metadata_schedule_489_0_e5235: f64 = (noise_metadata_schedule_489_0_e5231 + noise_metadata_schedule_489_0_e5234);
        let noise_metadata_schedule_489_0_e5240: f64 = (w[120] / w[142]);
        let noise_metadata_schedule_489_0_e5241: f64 = (w[120] * noise_metadata_schedule_489_0_e5240);
        let noise_metadata_schedule_489_0_e5244: f64 = (w[205] / w[203]);
        let noise_metadata_schedule_489_0_e5245: f64 = (noise_metadata_schedule_489_0_e5241 * noise_metadata_schedule_489_0_e5244);
        let noise_metadata_schedule_489_0_e5246: f64 = (noise_metadata_schedule_489_0_e5245).ln();
        let noise_metadata_schedule_489_0_e5247: f64 = (0.6666 * noise_metadata_schedule_489_0_e5246);
        let noise_metadata_schedule_489_0_e5248: f64 = (noise_metadata_schedule_489_0_e5247).exp();
        let noise_metadata_schedule_489_0_e5249: f64 = (noise_metadata_schedule_489_0_e5235 + noise_metadata_schedule_489_0_e5248);
        (noise_metadata_schedule_489_0_e5249,)
    } else {
        (w[123],)
    }
};
            w[123] = noise_metadata_schedule_489_0_e5251;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_490_0_e5279,) = {
    if (w[309] != 0.0) {
        let noise_metadata_schedule_490_0_e5255: f64 = (w[120] / w[118]);
        let noise_metadata_schedule_490_0_e5258: f64 = (w[121] / w[119]);
        let noise_metadata_schedule_490_0_e5259: f64 = (noise_metadata_schedule_490_0_e5255 + noise_metadata_schedule_490_0_e5258);
        let noise_metadata_schedule_490_0_e5262: f64 = (w[120] / w[203]);
        let noise_metadata_schedule_490_0_e5263: f64 = (noise_metadata_schedule_490_0_e5259 + noise_metadata_schedule_490_0_e5262);
        let noise_metadata_schedule_490_0_e5268: f64 = (w[120] / w[142]);
        let noise_metadata_schedule_490_0_e5269: f64 = (w[120] * noise_metadata_schedule_490_0_e5268);
        let noise_metadata_schedule_490_0_e5272: f64 = (w[205] / w[203]);
        let noise_metadata_schedule_490_0_e5273: f64 = (noise_metadata_schedule_490_0_e5269 * noise_metadata_schedule_490_0_e5272);
        let noise_metadata_schedule_490_0_e5274: f64 = (noise_metadata_schedule_490_0_e5273).ln();
        let noise_metadata_schedule_490_0_e5275: f64 = (0.6666 * noise_metadata_schedule_490_0_e5274);
        let noise_metadata_schedule_490_0_e5276: f64 = (noise_metadata_schedule_490_0_e5275).exp();
        let noise_metadata_schedule_490_0_e5277: f64 = (noise_metadata_schedule_490_0_e5263 + noise_metadata_schedule_490_0_e5276);
        (noise_metadata_schedule_490_0_e5277,)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_490_0_e5279;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_491_0_e5290,) = {
    if (w[309] == 0.0) {
        let noise_metadata_schedule_491_0_e5284: f64 = (w[120] / w[118]);
        let noise_metadata_schedule_491_0_e5287: f64 = (w[121] / w[119]);
        let noise_metadata_schedule_491_0_e5288: f64 = (noise_metadata_schedule_491_0_e5284 + noise_metadata_schedule_491_0_e5287);
        (noise_metadata_schedule_491_0_e5288,)
    } else {
        (w[123],)
    }
};
            w[123] = noise_metadata_schedule_491_0_e5290;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_492_0_e5305,) = {
    if (w[309] == 0.0) {
        let noise_metadata_schedule_492_0_e5295: f64 = (w[120] / w[118]);
        let noise_metadata_schedule_492_0_e5298: f64 = (w[121] / w[119]);
        let noise_metadata_schedule_492_0_e5299: f64 = (noise_metadata_schedule_492_0_e5295 + noise_metadata_schedule_492_0_e5298);
        let noise_metadata_schedule_492_0_e5302: f64 = (w[120] / w[203]);
        let noise_metadata_schedule_492_0_e5303: f64 = (noise_metadata_schedule_492_0_e5299 + noise_metadata_schedule_492_0_e5302);
        (noise_metadata_schedule_492_0_e5303,)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_492_0_e5305;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_493_0_e5309: f64 = (w[115] * w[115]);
            let noise_metadata_schedule_493_0_e5311: f64 = (noise_metadata_schedule_493_0_e5309 + w[123]);
            let noise_metadata_schedule_493_0_e5312: f64 = (noise_metadata_schedule_493_0_e5311).sqrt();
            let noise_metadata_schedule_493_0_e5313: f64 = (w[115] + noise_metadata_schedule_493_0_e5312);
            w[128] = noise_metadata_schedule_493_0_e5313;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_494_0_e5317: f64 = (w[115] * w[115]);
            let noise_metadata_schedule_494_0_e5319: f64 = (noise_metadata_schedule_494_0_e5317 + w[124]);
            let noise_metadata_schedule_494_0_e5320: f64 = (noise_metadata_schedule_494_0_e5319).sqrt();
            let noise_metadata_schedule_494_0_e5321: f64 = (w[115] + noise_metadata_schedule_494_0_e5320);
            w[129] = noise_metadata_schedule_494_0_e5321;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_495_0_e5324: f64 = (w[124] - w[123]);
            w[207] = noise_metadata_schedule_495_0_e5324;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_496_0_e5326: f64 = (w[207]).abs();
            let noise_metadata_schedule_496_0_e5328: f64 = if noise_metadata_schedule_496_0_e5326 > 1e-8 { 1.0 } else { 0.0 };
            w[310] = noise_metadata_schedule_496_0_e5328;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_497_0_e5342,) = {
    if (w[310] != 0.0) {
        let noise_metadata_schedule_497_0_e5334: f64 = (1.0 + w[206]);
        let noise_metadata_schedule_497_0_e5335: f64 = (w[142] / noise_metadata_schedule_497_0_e5334);
        let noise_metadata_schedule_497_0_e5337: f64 = (noise_metadata_schedule_497_0_e5335 / w[120]);
        let noise_metadata_schedule_497_0_e5339: f64 = (noise_metadata_schedule_497_0_e5337 * w[128]);
        let noise_metadata_schedule_497_0_e5340: f64 = (1.0 - noise_metadata_schedule_497_0_e5339);
        (noise_metadata_schedule_497_0_e5340,)
    } else {
        (w[150],)
    }
};
            w[150] = noise_metadata_schedule_497_0_e5342;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_498_0_e5358,) = {
    if (w[310] != 0.0) {
        let noise_metadata_schedule_498_0_e5348: f64 = (1.0 + w[206]);
        let noise_metadata_schedule_498_0_e5349: f64 = (w[142] / noise_metadata_schedule_498_0_e5348);
        let noise_metadata_schedule_498_0_e5351: f64 = (noise_metadata_schedule_498_0_e5349 / w[120]);
        let noise_metadata_schedule_498_0_e5354: f64 = (w[129] - w[128]);
        let noise_metadata_schedule_498_0_e5355: f64 = (noise_metadata_schedule_498_0_e5351 * noise_metadata_schedule_498_0_e5354);
        let noise_metadata_schedule_498_0_e5356: f64 = (1.0 + noise_metadata_schedule_498_0_e5355);
        (noise_metadata_schedule_498_0_e5356,)
    } else {
        (w[151],)
    }
};
            w[151] = noise_metadata_schedule_498_0_e5358;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_499_0_e5364,) = {
    if (w[310] != 0.0) {
        let noise_metadata_schedule_499_0_e5362: f64 = (w[150] / w[151]);
        (noise_metadata_schedule_499_0_e5362,)
    } else {
        (w[149],)
    }
};
            w[149] = noise_metadata_schedule_499_0_e5364;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_8(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_500_0_e5382,) = {
    if (w[310] != 0.0) {
        let noise_metadata_schedule_500_0_e5368: f64 = (w[149] * w[149]);
        let noise_metadata_schedule_500_0_e5370: f64 = (noise_metadata_schedule_500_0_e5368 + 0.01);
        let noise_metadata_schedule_500_0_e5371: f64 = (noise_metadata_schedule_500_0_e5370).sqrt();
        let noise_metadata_schedule_500_0_e5373: f64 = (noise_metadata_schedule_500_0_e5371 + w[149]);
        let noise_metadata_schedule_500_0_e5377: f64 = (1.0 + 0.01);
        let noise_metadata_schedule_500_0_e5378: f64 = (noise_metadata_schedule_500_0_e5377).sqrt();
        let noise_metadata_schedule_500_0_e5379: f64 = (1.0 + noise_metadata_schedule_500_0_e5378);
        let noise_metadata_schedule_500_0_e5380: f64 = (noise_metadata_schedule_500_0_e5373 / noise_metadata_schedule_500_0_e5379);
        (noise_metadata_schedule_500_0_e5380,)
    } else {
        (w[146],)
    }
};
            w[146] = noise_metadata_schedule_500_0_e5382;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_501_0_e5387,) = {
    if (w[310] == 0.0) {
        (0.0,)
    } else {
        (w[146],)
    }
};
            w[146] = noise_metadata_schedule_501_0_e5387;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_502_0_e5390: f64 = if params[2] == 0.0 { 1.0 } else { 0.0 };
            w[311] = noise_metadata_schedule_502_0_e5390;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_503_0_e5393: f64 = if params[13] != 0.0 { 1.0 } else { 0.0 };
            w[312] = noise_metadata_schedule_503_0_e5393;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_504_0_e5427,) = {
    if ((w[311] != 0.0) && (w[312] != 0.0)) {
        let noise_metadata_schedule_504_0_e5399: f64 = (w[120] / w[118]);
        let noise_metadata_schedule_504_0_e5402: f64 = (w[121] / w[119]);
        let noise_metadata_schedule_504_0_e5403: f64 = (noise_metadata_schedule_504_0_e5399 + noise_metadata_schedule_504_0_e5402);
        let noise_metadata_schedule_504_0_e5406: f64 = (w[120] / w[203]);
        let noise_metadata_schedule_504_0_e5408: f64 = (noise_metadata_schedule_504_0_e5406 * w[146]);
        let noise_metadata_schedule_504_0_e5410: f64 = (noise_metadata_schedule_504_0_e5408 * w[146]);
        let noise_metadata_schedule_504_0_e5411: f64 = (noise_metadata_schedule_504_0_e5403 + noise_metadata_schedule_504_0_e5410);
        let noise_metadata_schedule_504_0_e5416: f64 = (w[120] / w[142]);
        let noise_metadata_schedule_504_0_e5417: f64 = (w[120] * noise_metadata_schedule_504_0_e5416);
        let noise_metadata_schedule_504_0_e5420: f64 = (w[205] / w[203]);
        let noise_metadata_schedule_504_0_e5421: f64 = (noise_metadata_schedule_504_0_e5417 * noise_metadata_schedule_504_0_e5420);
        let noise_metadata_schedule_504_0_e5422: f64 = (noise_metadata_schedule_504_0_e5421).ln();
        let noise_metadata_schedule_504_0_e5423: f64 = (0.6666 * noise_metadata_schedule_504_0_e5422);
        let noise_metadata_schedule_504_0_e5424: f64 = (noise_metadata_schedule_504_0_e5423).exp();
        let noise_metadata_schedule_504_0_e5425: f64 = (noise_metadata_schedule_504_0_e5411 + noise_metadata_schedule_504_0_e5424);
        (noise_metadata_schedule_504_0_e5425,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_504_0_e5427;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_505_0_e5448,) = {
    if ((w[311] != 0.0) && (w[312] == 0.0)) {
        let noise_metadata_schedule_505_0_e5434: f64 = (w[120] / w[118]);
        let noise_metadata_schedule_505_0_e5437: f64 = (w[121] / w[119]);
        let noise_metadata_schedule_505_0_e5438: f64 = (noise_metadata_schedule_505_0_e5434 + noise_metadata_schedule_505_0_e5437);
        let noise_metadata_schedule_505_0_e5441: f64 = (w[120] / w[203]);
        let noise_metadata_schedule_505_0_e5443: f64 = (noise_metadata_schedule_505_0_e5441 * w[146]);
        let noise_metadata_schedule_505_0_e5445: f64 = (noise_metadata_schedule_505_0_e5443 * w[146]);
        let noise_metadata_schedule_505_0_e5446: f64 = (noise_metadata_schedule_505_0_e5438 + noise_metadata_schedule_505_0_e5445);
        (noise_metadata_schedule_505_0_e5446,)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_505_0_e5448;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_506_0_e5459,) = {
    if (w[311] != 0.0) {
        let noise_metadata_schedule_506_0_e5453: f64 = (w[115] * w[115]);
        let noise_metadata_schedule_506_0_e5455: f64 = (noise_metadata_schedule_506_0_e5453 + w[122]);
        let noise_metadata_schedule_506_0_e5456: f64 = (noise_metadata_schedule_506_0_e5455).sqrt();
        let noise_metadata_schedule_506_0_e5457: f64 = (w[115] + noise_metadata_schedule_506_0_e5456);
        (noise_metadata_schedule_506_0_e5457,)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_506_0_e5459;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_507_0_e5466,) = {
    if (w[311] == 0.0) {
        let noise_metadata_schedule_507_0_e5464: f64 = (1.0 / 3.0);
        (noise_metadata_schedule_507_0_e5464,)
    } else {
        (w[83],)
    }
};
            w[83] = noise_metadata_schedule_507_0_e5466;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_508_0_e5474,) = {
    if (w[311] == 0.0) {
        let noise_metadata_schedule_508_0_e5470: f64 = (-2.0);
        let noise_metadata_schedule_508_0_e5472: f64 = (noise_metadata_schedule_508_0_e5470 * w[115]);
        (noise_metadata_schedule_508_0_e5472,)
    } else {
        (w[84],)
    }
};
            w[84] = noise_metadata_schedule_508_0_e5474;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_509_0_e5481: f64 = if ((params[9] == 1000000.0) && (params[12] == 1000000.0)) { 1.0 } else { 0.0 };
            w[313] = noise_metadata_schedule_509_0_e5481;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_510_0_e5488,) = {
    if ((w[311] == 0.0) && (w[313] != 0.0)) {
        (0.0,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_510_0_e5488;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_511_0_e5511,) = {
    if ((w[311] == 0.0) && (w[313] == 0.0)) {
        let noise_metadata_schedule_511_0_e5496: f64 = (w[120] / w[118]);
        let noise_metadata_schedule_511_0_e5499: f64 = (w[121] / w[119]);
        let noise_metadata_schedule_511_0_e5500: f64 = (noise_metadata_schedule_511_0_e5496 + noise_metadata_schedule_511_0_e5499);
        let noise_metadata_schedule_511_0_e5503: f64 = (w[120] / w[203]);
        let noise_metadata_schedule_511_0_e5505: f64 = (noise_metadata_schedule_511_0_e5503 * w[146]);
        let noise_metadata_schedule_511_0_e5507: f64 = (noise_metadata_schedule_511_0_e5505 * w[146]);
        let noise_metadata_schedule_511_0_e5508: f64 = (noise_metadata_schedule_511_0_e5500 + noise_metadata_schedule_511_0_e5507);
        let noise_metadata_schedule_511_0_e5509: f64 = (-noise_metadata_schedule_511_0_e5508);
        (noise_metadata_schedule_511_0_e5509,)
    } else {
        (w[85],)
    }
};
            w[85] = noise_metadata_schedule_511_0_e5511;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_512_0_e5525,) = {
    if (w[311] == 0.0) {
        let noise_metadata_schedule_512_0_e5515: f64 = (-w[120]);
        let noise_metadata_schedule_512_0_e5517: f64 = (noise_metadata_schedule_512_0_e5515 * w[120]);
        let noise_metadata_schedule_512_0_e5519: f64 = (noise_metadata_schedule_512_0_e5517 / w[142]);
        let noise_metadata_schedule_512_0_e5521: f64 = (noise_metadata_schedule_512_0_e5519 * w[205]);
        let noise_metadata_schedule_512_0_e5523: f64 = (noise_metadata_schedule_512_0_e5521 / w[203]);
        (noise_metadata_schedule_512_0_e5523,)
    } else {
        (w[86],)
    }
};
            w[86] = noise_metadata_schedule_512_0_e5525;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_513_0_e5532,) = {
    if (w[311] == 0.0) {
        let noise_metadata_schedule_513_0_e5530: f64 = (w[84] * w[84]);
        (noise_metadata_schedule_513_0_e5530,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_513_0_e5532;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_514_0_e5541,) = {
    if (w[311] == 0.0) {
        let noise_metadata_schedule_514_0_e5538: f64 = (w[87] * w[83]);
        let noise_metadata_schedule_514_0_e5539: f64 = (w[85] - noise_metadata_schedule_514_0_e5538);
        (noise_metadata_schedule_514_0_e5539,)
    } else {
        (w[88],)
    }
};
            w[88] = noise_metadata_schedule_514_0_e5541;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_515_0_e5560,) = {
    if (w[311] == 0.0) {
        let noise_metadata_schedule_515_0_e5546: f64 = (2.0 * w[84]);
        let noise_metadata_schedule_515_0_e5548: f64 = (noise_metadata_schedule_515_0_e5546 * w[87]);
        let noise_metadata_schedule_515_0_e5550: f64 = (noise_metadata_schedule_515_0_e5548 / 27.0);
        let noise_metadata_schedule_515_0_e5553: f64 = (w[84] * w[85]);
        let noise_metadata_schedule_515_0_e5555: f64 = (noise_metadata_schedule_515_0_e5553 * w[83]);
        let noise_metadata_schedule_515_0_e5556: f64 = (noise_metadata_schedule_515_0_e5550 - noise_metadata_schedule_515_0_e5555);
        let noise_metadata_schedule_515_0_e5558: f64 = (noise_metadata_schedule_515_0_e5556 + w[86]);
        (noise_metadata_schedule_515_0_e5558,)
    } else {
        (w[89],)
    }
};
            w[89] = noise_metadata_schedule_515_0_e5560;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_516_0_e5577,) = {
    if (w[311] == 0.0) {
        let noise_metadata_schedule_516_0_e5565: f64 = (w[89] * w[89]);
        let noise_metadata_schedule_516_0_e5567: f64 = (noise_metadata_schedule_516_0_e5565 * 0.25);
        let noise_metadata_schedule_516_0_e5570: f64 = (w[88] * w[88]);
        let noise_metadata_schedule_516_0_e5572: f64 = (noise_metadata_schedule_516_0_e5570 * w[88]);
        let noise_metadata_schedule_516_0_e5574: f64 = (noise_metadata_schedule_516_0_e5572 / 27.0);
        let noise_metadata_schedule_516_0_e5575: f64 = (noise_metadata_schedule_516_0_e5567 + noise_metadata_schedule_516_0_e5574);
        (noise_metadata_schedule_516_0_e5575,)
    } else {
        (w[90],)
    }
};
            w[90] = noise_metadata_schedule_516_0_e5577;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_517_0_e5579: f64 = (w[90]).abs();
            let noise_metadata_schedule_517_0_e5581: f64 = if noise_metadata_schedule_517_0_e5579 < 1e-10 { 1.0 } else { 0.0 };
            w[314] = noise_metadata_schedule_517_0_e5581;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_518_0_e5596,) = {
    if ((w[311] == 0.0) && (w[314] != 0.0)) {
        let noise_metadata_schedule_518_0_e5588: f64 = (3.0 * w[89]);
        let noise_metadata_schedule_518_0_e5590: f64 = (noise_metadata_schedule_518_0_e5588 / w[88]);
        let noise_metadata_schedule_518_0_e5593: f64 = (w[84] * w[83]);
        let noise_metadata_schedule_518_0_e5594: f64 = (noise_metadata_schedule_518_0_e5590 - noise_metadata_schedule_518_0_e5593);
        (noise_metadata_schedule_518_0_e5594,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_518_0_e5596;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_519_0_e5599: f64 = if w[90] > 0.0 { 1.0 } else { 0.0 };
            w[315] = noise_metadata_schedule_519_0_e5599;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_520_0_e5612,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) {
        let noise_metadata_schedule_520_0_e5608: f64 = (-w[89]);
        let noise_metadata_schedule_520_0_e5610: f64 = (noise_metadata_schedule_520_0_e5608 * 0.5);
        (noise_metadata_schedule_520_0_e5610,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_520_0_e5612;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_521_0_e5623,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) {
        let noise_metadata_schedule_521_0_e5621: f64 = (w[90]).sqrt();
        (noise_metadata_schedule_521_0_e5621,)
    } else {
        (w[93],)
    }
};
            w[93] = noise_metadata_schedule_521_0_e5623;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_522_0_e5635,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) {
        let noise_metadata_schedule_522_0_e5633: f64 = (w[92] + w[93]);
        (noise_metadata_schedule_522_0_e5633,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_522_0_e5635;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_523_0_e5638: f64 = if w[87] > 0.0 { 1.0 } else { 0.0 };
            w[316] = noise_metadata_schedule_523_0_e5638;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_524_0_e5654,) = {
    if ((((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) && (w[316] != 0.0)) {
        let noise_metadata_schedule_524_0_e5650: f64 = (w[87]).ln();
        let noise_metadata_schedule_524_0_e5651: f64 = (w[83] * noise_metadata_schedule_524_0_e5650);
        let noise_metadata_schedule_524_0_e5652: f64 = (noise_metadata_schedule_524_0_e5651).exp();
        (noise_metadata_schedule_524_0_e5652,)
    } else {
        (w[94],)
    }
};
            w[94] = noise_metadata_schedule_524_0_e5654;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_525_0_e5673,) = {
    if ((((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) && (w[316] == 0.0)) {
        let noise_metadata_schedule_525_0_e5667: f64 = (-w[87]);
        let noise_metadata_schedule_525_0_e5668: f64 = (noise_metadata_schedule_525_0_e5667).ln();
        let noise_metadata_schedule_525_0_e5669: f64 = (w[83] * noise_metadata_schedule_525_0_e5668);
        let noise_metadata_schedule_525_0_e5670: f64 = (noise_metadata_schedule_525_0_e5669).exp();
        let noise_metadata_schedule_525_0_e5671: f64 = (-noise_metadata_schedule_525_0_e5670);
        (noise_metadata_schedule_525_0_e5671,)
    } else {
        (w[94],)
    }
};
            w[94] = noise_metadata_schedule_525_0_e5673;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_526_0_e5685,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) {
        let noise_metadata_schedule_526_0_e5683: f64 = (w[92] - w[93]);
        (noise_metadata_schedule_526_0_e5683,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_526_0_e5685;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_527_0_e5688: f64 = if w[87] > 0.0 { 1.0 } else { 0.0 };
            w[317] = noise_metadata_schedule_527_0_e5688;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_528_0_e5704,) = {
    if ((((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) && (w[317] != 0.0)) {
        let noise_metadata_schedule_528_0_e5700: f64 = (w[87]).ln();
        let noise_metadata_schedule_528_0_e5701: f64 = (w[83] * noise_metadata_schedule_528_0_e5700);
        let noise_metadata_schedule_528_0_e5702: f64 = (noise_metadata_schedule_528_0_e5701).exp();
        (noise_metadata_schedule_528_0_e5702,)
    } else {
        (w[95],)
    }
};
            w[95] = noise_metadata_schedule_528_0_e5704;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_529_0_e5723,) = {
    if ((((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) && (w[317] == 0.0)) {
        let noise_metadata_schedule_529_0_e5717: f64 = (-w[87]);
        let noise_metadata_schedule_529_0_e5718: f64 = (noise_metadata_schedule_529_0_e5717).ln();
        let noise_metadata_schedule_529_0_e5719: f64 = (w[83] * noise_metadata_schedule_529_0_e5718);
        let noise_metadata_schedule_529_0_e5720: f64 = (noise_metadata_schedule_529_0_e5719).exp();
        let noise_metadata_schedule_529_0_e5721: f64 = (-noise_metadata_schedule_529_0_e5720);
        (noise_metadata_schedule_529_0_e5721,)
    } else {
        (w[95],)
    }
};
            w[95] = noise_metadata_schedule_529_0_e5723;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_530_0_e5739,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] != 0.0)) {
        let noise_metadata_schedule_530_0_e5733: f64 = (w[94] + w[95]);
        let noise_metadata_schedule_530_0_e5736: f64 = (w[84] * w[83]);
        let noise_metadata_schedule_530_0_e5737: f64 = (noise_metadata_schedule_530_0_e5733 - noise_metadata_schedule_530_0_e5736);
        (noise_metadata_schedule_530_0_e5737,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_530_0_e5739;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_531_0_e5763,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] == 0.0)) {
        let noise_metadata_schedule_531_0_e5749: f64 = (-w[89]);
        let noise_metadata_schedule_531_0_e5751: f64 = (noise_metadata_schedule_531_0_e5749 * 0.5);
        let noise_metadata_schedule_531_0_e5753: f64 = (-27.0);
        let noise_metadata_schedule_531_0_e5756: f64 = (w[88] * w[88]);
        let noise_metadata_schedule_531_0_e5758: f64 = (noise_metadata_schedule_531_0_e5756 * w[88]);
        let noise_metadata_schedule_531_0_e5759: f64 = (noise_metadata_schedule_531_0_e5753 / noise_metadata_schedule_531_0_e5758);
        let noise_metadata_schedule_531_0_e5760: f64 = (noise_metadata_schedule_531_0_e5759).sqrt();
        let noise_metadata_schedule_531_0_e5761: f64 = (noise_metadata_schedule_531_0_e5751 * noise_metadata_schedule_531_0_e5760);
        (noise_metadata_schedule_531_0_e5761,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_531_0_e5763;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_532_0_e5776,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] == 0.0)) {
        let noise_metadata_schedule_532_0_e5774: f64 = (w[87] * w[87]);
        (noise_metadata_schedule_532_0_e5774,)
    } else {
        (w[92],)
    }
};
            w[92] = noise_metadata_schedule_532_0_e5776;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_533_0_e5779: f64 = if w[87] >= 0.0 { 1.0 } else { 0.0 };
            w[318] = noise_metadata_schedule_533_0_e5779;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_534_0_e5802,) = {
    if ((((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] == 0.0)) && (w[318] != 0.0)) {
        let noise_metadata_schedule_534_0_e5792: f64 = (3.141592653589793 / 2.0);
        let noise_metadata_schedule_534_0_e5796: f64 = (1.0 - w[92]);
        let noise_metadata_schedule_534_0_e5797: f64 = (w[92] / noise_metadata_schedule_534_0_e5796);
        let noise_metadata_schedule_534_0_e5798: f64 = (noise_metadata_schedule_534_0_e5797).sqrt();
        let noise_metadata_schedule_534_0_e5799: f64 = (noise_metadata_schedule_534_0_e5798).atan();
        let noise_metadata_schedule_534_0_e5800: f64 = (noise_metadata_schedule_534_0_e5792 - noise_metadata_schedule_534_0_e5799);
        (noise_metadata_schedule_534_0_e5800,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_534_0_e5802;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_535_0_e5826,) = {
    if ((((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] == 0.0)) && (w[318] == 0.0)) {
        let noise_metadata_schedule_535_0_e5816: f64 = (3.141592653589793 / 2.0);
        let noise_metadata_schedule_535_0_e5820: f64 = (1.0 - w[92]);
        let noise_metadata_schedule_535_0_e5821: f64 = (w[92] / noise_metadata_schedule_535_0_e5820);
        let noise_metadata_schedule_535_0_e5822: f64 = (noise_metadata_schedule_535_0_e5821).sqrt();
        let noise_metadata_schedule_535_0_e5823: f64 = (noise_metadata_schedule_535_0_e5822).atan();
        let noise_metadata_schedule_535_0_e5824: f64 = (noise_metadata_schedule_535_0_e5816 + noise_metadata_schedule_535_0_e5823);
        (noise_metadata_schedule_535_0_e5824,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_535_0_e5826;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_536_0_e5852,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] == 0.0)) {
        let noise_metadata_schedule_536_0_e5836: f64 = (-4.0);
        let noise_metadata_schedule_536_0_e5838: f64 = (noise_metadata_schedule_536_0_e5836 * w[88]);
        let noise_metadata_schedule_536_0_e5840: f64 = (noise_metadata_schedule_536_0_e5838 * w[83]);
        let noise_metadata_schedule_536_0_e5841: f64 = (noise_metadata_schedule_536_0_e5840).sqrt();
        let noise_metadata_schedule_536_0_e5844: f64 = (w[83] * w[87]);
        let noise_metadata_schedule_536_0_e5845: f64 = (noise_metadata_schedule_536_0_e5844).cos();
        let noise_metadata_schedule_536_0_e5846: f64 = (noise_metadata_schedule_536_0_e5841 * noise_metadata_schedule_536_0_e5845);
        let noise_metadata_schedule_536_0_e5849: f64 = (w[84] * w[83]);
        let noise_metadata_schedule_536_0_e5850: f64 = (noise_metadata_schedule_536_0_e5846 - noise_metadata_schedule_536_0_e5849);
        (noise_metadata_schedule_536_0_e5850,)
    } else {
        (w[87],)
    }
};
            w[87] = noise_metadata_schedule_536_0_e5852;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_537_0_e5863,) = {
    if (((w[311] == 0.0) && (w[314] == 0.0)) && (w[315] == 0.0)) {
        (w[87],)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_537_0_e5863;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_538_0_e5868,) = {
    if (w[311] == 0.0) {
        (w[91],)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_538_0_e5868;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_539_0_e5871: f64 = if w[125] < 1e-20 { 1.0 } else { 0.0 };
            w[319] = noise_metadata_schedule_539_0_e5871;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_540_0_e5875,) = {
    if (w[319] != 0.0) {
        (1e-20,)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_540_0_e5875;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_541_0_e5878: f64 = (w[120] / w[125]);
            w[126] = noise_metadata_schedule_541_0_e5878;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_542_0_e5881: f64 = (w[121] / w[125]);
            w[127] = noise_metadata_schedule_542_0_e5881;
        }
        if (active[0] & 0x21) != 0 {
            let noise_metadata_schedule_543_0_e5884: f64 = if w[126] < 1e-20 { 1.0 } else { 0.0 };
            w[320] = noise_metadata_schedule_543_0_e5884;
        }
        if (active[0] & 0x21) != 0 {
            let (noise_metadata_schedule_544_0_e5888,) = {
    if (w[320] != 0.0) {
        (1e-20,)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_544_0_e5888;
        }
        if (active[0] & 0x19) != 0 {
            let noise_metadata_schedule_556_0_e5950: f64 = if params[15] > 0.0 { 1.0 } else { 0.0 };
            w[321] = noise_metadata_schedule_556_0_e5950;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_9(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 386], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_557_0_e5958,) = {
    if (w[321] != 0.0) {
        let noise_metadata_schedule_557_0_e5955: f64 = (params[16] * w[2]);
        let noise_metadata_schedule_557_0_e5956: f64 = (w[185] / noise_metadata_schedule_557_0_e5955);
        (noise_metadata_schedule_557_0_e5956,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_557_0_e5958;
        }
        if (active[0] & 0x19) != 0 {
            let noise_metadata_schedule_558_0_e5961: f64 = if w[48] > 80.0 { 1.0 } else { 0.0 };
            w[322] = noise_metadata_schedule_558_0_e5961;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_559_0_e5971,) = {
    if ((w[321] != 0.0) && (w[322] != 0.0)) {
        let noise_metadata_schedule_559_0_e5968: f64 = (w[48] - 80.0);
        let noise_metadata_schedule_559_0_e5969: f64 = (1.0 + noise_metadata_schedule_559_0_e5968);
        (noise_metadata_schedule_559_0_e5969,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_559_0_e5971;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_560_0_e5977,) = {
    if ((w[321] != 0.0) && (w[322] != 0.0)) {
        (80.0,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_560_0_e5977;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_561_0_e5984,) = {
    if ((w[321] != 0.0) && (w[322] == 0.0)) {
        (1.0,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_561_0_e5984;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_562_0_e5995,) = {
    if (w[321] != 0.0) {
        let noise_metadata_schedule_562_0_e5989: f64 = { let limexp_arg = w[48]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_562_0_e5990: f64 = (w[49] * noise_metadata_schedule_562_0_e5989);
        let noise_metadata_schedule_562_0_e5992: f64 = (noise_metadata_schedule_562_0_e5990 - 1.0);
        let noise_metadata_schedule_562_0_e5993: f64 = (w[13] * noise_metadata_schedule_562_0_e5992);
        (noise_metadata_schedule_562_0_e5993,)
    } else {
        (w[134],)
    }
};
            w[134] = noise_metadata_schedule_562_0_e5995;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_563_0_e6000,) = {
    if (w[321] == 0.0) {
        (0.0,)
    } else {
        (w[134],)
    }
};
            w[134] = noise_metadata_schedule_563_0_e6000;
        }
        if (active[0] & 0x19) != 0 {
            let noise_metadata_schedule_564_0_e6003: f64 = if params[17] > 0.0 { 1.0 } else { 0.0 };
            w[323] = noise_metadata_schedule_564_0_e6003;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_565_0_e6011,) = {
    if (w[323] != 0.0) {
        let noise_metadata_schedule_565_0_e6008: f64 = (params[18] * w[2]);
        let noise_metadata_schedule_565_0_e6009: f64 = (w[185] / noise_metadata_schedule_565_0_e6008);
        (noise_metadata_schedule_565_0_e6009,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_565_0_e6011;
        }
        if (active[0] & 0x19) != 0 {
            let noise_metadata_schedule_566_0_e6014: f64 = if w[48] > 80.0 { 1.0 } else { 0.0 };
            w[324] = noise_metadata_schedule_566_0_e6014;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_567_0_e6024,) = {
    if ((w[323] != 0.0) && (w[324] != 0.0)) {
        let noise_metadata_schedule_567_0_e6021: f64 = (w[48] - 80.0);
        let noise_metadata_schedule_567_0_e6022: f64 = (1.0 + noise_metadata_schedule_567_0_e6021);
        (noise_metadata_schedule_567_0_e6022,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_567_0_e6024;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_568_0_e6030,) = {
    if ((w[323] != 0.0) && (w[324] != 0.0)) {
        (80.0,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_568_0_e6030;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_569_0_e6037,) = {
    if ((w[323] != 0.0) && (w[324] == 0.0)) {
        (1.0,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_569_0_e6037;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_570_0_e6048,) = {
    if (w[323] != 0.0) {
        let noise_metadata_schedule_570_0_e6042: f64 = { let limexp_arg = w[48]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_570_0_e6043: f64 = (w[49] * noise_metadata_schedule_570_0_e6042);
        let noise_metadata_schedule_570_0_e6045: f64 = (noise_metadata_schedule_570_0_e6043 - 1.0);
        let noise_metadata_schedule_570_0_e6046: f64 = (w[12] * noise_metadata_schedule_570_0_e6045);
        (noise_metadata_schedule_570_0_e6046,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_570_0_e6048;
        }
        if (active[0] & 0x19) != 0 {
            let (noise_metadata_schedule_571_0_e6053,) = {
    if (w[323] == 0.0) {
        (0.0,)
    } else {
        (w[135],)
    }
};
            w[135] = noise_metadata_schedule_571_0_e6053;
        }
        if (active[0] & 0x19) != 0 {
            let noise_metadata_schedule_572_0_e6056: f64 = (w[134] + w[135]);
            w[195] = noise_metadata_schedule_572_0_e6056;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_573_0_e6059: f64 = if params[19] > 0.0 { 1.0 } else { 0.0 };
            w[325] = noise_metadata_schedule_573_0_e6059;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_574_0_e6067,) = {
    if (w[325] != 0.0) {
        let noise_metadata_schedule_574_0_e6064: f64 = (params[20] * w[2]);
        let noise_metadata_schedule_574_0_e6065: f64 = (w[184] / noise_metadata_schedule_574_0_e6064);
        (noise_metadata_schedule_574_0_e6065,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_574_0_e6067;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_575_0_e6070: f64 = if w[48] > 80.0 { 1.0 } else { 0.0 };
            w[326] = noise_metadata_schedule_575_0_e6070;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_576_0_e6080,) = {
    if ((w[325] != 0.0) && (w[326] != 0.0)) {
        let noise_metadata_schedule_576_0_e6077: f64 = (w[48] - 80.0);
        let noise_metadata_schedule_576_0_e6078: f64 = (1.0 + noise_metadata_schedule_576_0_e6077);
        (noise_metadata_schedule_576_0_e6078,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_576_0_e6080;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_577_0_e6086,) = {
    if ((w[325] != 0.0) && (w[326] != 0.0)) {
        (80.0,)
    } else {
        (w[48],)
    }
};
            w[48] = noise_metadata_schedule_577_0_e6086;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_578_0_e6093,) = {
    if ((w[325] != 0.0) && (w[326] == 0.0)) {
        (1.0,)
    } else {
        (w[49],)
    }
};
            w[49] = noise_metadata_schedule_578_0_e6093;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_579_0_e6104,) = {
    if (w[325] != 0.0) {
        let noise_metadata_schedule_579_0_e6098: f64 = { let limexp_arg = w[48]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let noise_metadata_schedule_579_0_e6099: f64 = (w[49] * noise_metadata_schedule_579_0_e6098);
        let noise_metadata_schedule_579_0_e6101: f64 = (noise_metadata_schedule_579_0_e6099 - 1.0);
        let noise_metadata_schedule_579_0_e6102: f64 = (w[14] * noise_metadata_schedule_579_0_e6101);
        (noise_metadata_schedule_579_0_e6102,)
    } else {
        (w[192],)
    }
};
            w[192] = noise_metadata_schedule_579_0_e6104;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_580_0_e6109,) = {
    if (w[325] == 0.0) {
        (0.0,)
    } else {
        (w[192],)
    }
};
            w[192] = noise_metadata_schedule_580_0_e6109;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_581_0_e6112: f64 = (w[195] + w[192]);
            w[136] = noise_metadata_schedule_581_0_e6112;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_645_0_e6766: f64 = if w[37] > 0.0 { 1.0 } else { 0.0 };
            w[340] = noise_metadata_schedule_645_0_e6766;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_646_0_e6786,) = {
    if (w[340] != 0.0) {
        let noise_metadata_schedule_646_0_e6771: f64 = (w[102] / params[24]);
        let noise_metadata_schedule_646_0_e6772: f64 = (1.0 + noise_metadata_schedule_646_0_e6771);
        let noise_metadata_schedule_646_0_e6775: f64 = (w[103] / params[25]);
        let noise_metadata_schedule_646_0_e6776: f64 = (noise_metadata_schedule_646_0_e6772 + noise_metadata_schedule_646_0_e6775);
        let noise_metadata_schedule_646_0_e6779: f64 = (w[126] / w[118]);
        let noise_metadata_schedule_646_0_e6780: f64 = (noise_metadata_schedule_646_0_e6776 + noise_metadata_schedule_646_0_e6779);
        let noise_metadata_schedule_646_0_e6783: f64 = (w[127] / w[119]);
        let noise_metadata_schedule_646_0_e6784: f64 = (noise_metadata_schedule_646_0_e6780 + noise_metadata_schedule_646_0_e6783);
        (noise_metadata_schedule_646_0_e6784,)
    } else {
        (w[160],)
    }
};
            w[160] = noise_metadata_schedule_646_0_e6786;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_647_0_e6799,) = {
    if (w[340] != 0.0) {
        let noise_metadata_schedule_647_0_e6792: f64 = (w[160] * w[160]);
        let noise_metadata_schedule_647_0_e6794: f64 = (noise_metadata_schedule_647_0_e6792 + 0.01);
        let noise_metadata_schedule_647_0_e6795: f64 = (noise_metadata_schedule_647_0_e6794).sqrt();
        let noise_metadata_schedule_647_0_e6796: f64 = (w[160] + noise_metadata_schedule_647_0_e6795);
        let noise_metadata_schedule_647_0_e6797: f64 = (0.5 * noise_metadata_schedule_647_0_e6796);
        (noise_metadata_schedule_647_0_e6797,)
    } else {
        (w[161],)
    }
};
            w[161] = noise_metadata_schedule_647_0_e6799;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_648_0_e6805,) = {
    if (w[340] != 0.0) {
        let noise_metadata_schedule_648_0_e6803: f64 = (w[37] / w[161]);
        (noise_metadata_schedule_648_0_e6803,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_648_0_e6805;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_649_0_e6808: f64 = if w[136] > 0.0 { 1.0 } else { 0.0 };
            w[341] = noise_metadata_schedule_649_0_e6808;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_650_0_e6820,) = {
    if ((w[340] != 0.0) && (w[341] != 0.0)) {
        let noise_metadata_schedule_650_0_e6814: f64 = (params[27] * w[158]);
        let noise_metadata_schedule_650_0_e6816: f64 = (noise_metadata_schedule_650_0_e6814 * w[136]);
        let noise_metadata_schedule_650_0_e6818: f64 = (noise_metadata_schedule_650_0_e6816 * w[3]);
        (noise_metadata_schedule_650_0_e6818,)
    } else {
        (w[157],)
    }
};
            w[157] = noise_metadata_schedule_650_0_e6820;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_651_0_e6823: f64 = if w[157] < 1e-6 { 1.0 } else { 0.0 };
            w[342] = noise_metadata_schedule_651_0_e6823;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_652_0_e6837,) = {
    if (((w[340] != 0.0) && (w[341] != 0.0)) && (w[342] != 0.0)) {
        let noise_metadata_schedule_652_0_e6833: f64 = (0.5 * w[157]);
        let noise_metadata_schedule_652_0_e6834: f64 = (1.0 - noise_metadata_schedule_652_0_e6833);
        let noise_metadata_schedule_652_0_e6835: f64 = (w[158] * noise_metadata_schedule_652_0_e6834);
        (noise_metadata_schedule_652_0_e6835,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_652_0_e6837;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_653_0_e6853,) = {
    if (((w[340] != 0.0) && (w[341] != 0.0)) && (w[342] == 0.0)) {
        let noise_metadata_schedule_653_0_e6847: f64 = (w[157] + 1.0);
        let noise_metadata_schedule_653_0_e6848: f64 = (noise_metadata_schedule_653_0_e6847).ln();
        let noise_metadata_schedule_653_0_e6849: f64 = (w[158] * noise_metadata_schedule_653_0_e6848);
        let noise_metadata_schedule_653_0_e6851: f64 = (noise_metadata_schedule_653_0_e6849 / w[157]);
        (noise_metadata_schedule_653_0_e6851,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_653_0_e6853;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_654_0_e6858,) = {
    if (w[340] == 0.0) {
        (0.0,)
    } else {
        (w[158],)
    }
};
            w[158] = noise_metadata_schedule_654_0_e6858;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_655_0_e6861: f64 = (w[158] + w[39]);
            w[156] = noise_metadata_schedule_655_0_e6861;
        }
        if (active[0] & 0x20) != 0 {
            w[211] = w[126];
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_726_0_e7532: f64 = if ((params[73] != 0.0) && (params[54] != 0.0)) { 1.0 } else { 0.0 };
            w[355] = noise_metadata_schedule_726_0_e7532;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_730_0_e7554,) = {
    if (w[355] != 0.0) {
        ((ctx.node_voltage(self.nodes[9]) - 0.0),)
    } else {
        (w[211],)
    }
};
            w[211] = noise_metadata_schedule_730_0_e7554;
        }
        if (active[0] & 0x18) != 0 {
            let noise_metadata_schedule_749_0_e7627: f64 = (params[110] * w[195]);
            w[195] = noise_metadata_schedule_749_0_e7627;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_752_0_e7637: f64 = (w[211] - w[127]);
            let noise_metadata_schedule_752_0_e7638: f64 = (params[110] * noise_metadata_schedule_752_0_e7637);
            w[132] = noise_metadata_schedule_752_0_e7638;
        }
        if (active[0] & 0x7) != 0 {
            let noise_metadata_schedule_761_0_e7683: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_761_0_e7685: f64 = (noise_metadata_schedule_761_0_e7683 * w[4]);
            w[361] = noise_metadata_schedule_761_0_e7685;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_765_0_e7701: f64 = (w[195]).abs();
            let noise_metadata_schedule_765_0_e7703: f64 = (noise_metadata_schedule_765_0_e7701).powf(params[75]);
            let noise_metadata_schedule_765_0_e7704: f64 = (params[74] * noise_metadata_schedule_765_0_e7703);
            w[362] = noise_metadata_schedule_765_0_e7704;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_766_0_e7707: f64 = (2.0 * 1.602176462e-19);
            w[363] = noise_metadata_schedule_766_0_e7707;
        }
    }
}

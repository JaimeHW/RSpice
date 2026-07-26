#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("Rb"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE", label: Some("Re"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_C_CI_RC", label: Some("Rc"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER_IBE", label: Some("flicker_Ibe"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("Ibe"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("It"), kind: GeneratedNoiseKind::White, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 128];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            w[125] != 0.0
        };
        let noise_source_1_active = {
            w[126] != 0.0
        };
        let noise_source_2_active = {
            w[127] != 0.0
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
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e2354: f64 = 1.0;
            let noise_0_psd_e2355: f64 = (noise_0_psd_e2354 * w[72]);
            let psd = noise_0_psd_e2355;
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
            let noise_1_psd_e2357: f64 = 1.0;
            let noise_1_psd_e2358: f64 = (noise_1_psd_e2357 * w[73]);
            let psd = noise_1_psd_e2358;
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
            let noise_2_psd_e2360: f64 = 1.0;
            let noise_2_psd_e2361: f64 = (noise_2_psd_e2360 * w[74]);
            let psd = noise_2_psd_e2361;
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
            let noise_3_psd_e2363: f64 = 1.0;
            let noise_3_psd_e429: f64 = (w[9] * w[37]);
            let (noise_3_psd_e436,) = {
    if (noise_3_psd_e429 >= 0.0) {
        let noise_3_psd_e433: f64 = 1.0;
        (noise_3_psd_e433,)
    } else {
        let noise_3_psd_e435: f64 = (-1.0);
        (noise_3_psd_e435,)
    }
};
            let noise_3_psd_e438: f64 = (noise_3_psd_e436 * w[71]);
            let noise_3_psd_e2364: f64 = (noise_3_psd_e2363 * noise_3_psd_e438);
            let psd = noise_3_psd_e2364;
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
            let noise_4_psd_e2366: f64 = 1.0;
            let noise_4_psd_e444: f64 = (w[37]).abs();
            let noise_4_psd_e445: f64 = (w[70] * noise_4_psd_e444);
            let noise_4_psd_e2367: f64 = (noise_4_psd_e2366 * noise_4_psd_e445);
            let psd = noise_4_psd_e2367;
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
            let noise_5_psd_e2369: f64 = 1.0;
            let noise_5_psd_e451: f64 = (w[44] - w[45]);
            let noise_5_psd_e452: f64 = (noise_5_psd_e451).abs();
            let noise_5_psd_e453: f64 = (w[70] * noise_5_psd_e452);
            let noise_5_psd_e2370: f64 = (noise_5_psd_e2369 * noise_5_psd_e453);
            let psd = noise_5_psd_e2370;
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
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 128]) {
        let params = &*self.params;
        let noise_activation_schedule_4_0_e502: f64 = (params.p43 * params.p42);
        w[3] = noise_activation_schedule_4_0_e502;
        let noise_activation_schedule_196_0_e2249: f64 = (params.p31 * params.p13);
        let noise_activation_schedule_196_0_e2250: f64 = (params.p12 + noise_activation_schedule_196_0_e2249);
        let noise_activation_schedule_196_0_e2252: f64 = (noise_activation_schedule_196_0_e2250 / w[3]);
        w[50] = noise_activation_schedule_196_0_e2252;
        let noise_activation_schedule_197_0_e2256: f64 = (params.p31 * params.p15);
        let noise_activation_schedule_197_0_e2257: f64 = (params.p14 + noise_activation_schedule_197_0_e2256);
        let noise_activation_schedule_197_0_e2259: f64 = (noise_activation_schedule_197_0_e2257 / w[3]);
        w[48] = noise_activation_schedule_197_0_e2259;
        let noise_activation_schedule_198_0_e2263: f64 = (params.p31 * params.p67);
        let noise_activation_schedule_198_0_e2264: f64 = (params.p66 + noise_activation_schedule_198_0_e2263);
        let noise_activation_schedule_198_0_e2266: f64 = (noise_activation_schedule_198_0_e2264 / w[3]);
        w[49] = noise_activation_schedule_198_0_e2266;
        let noise_activation_schedule_199_0_e2273: f64 = if ((w[50] > 0.0) && (w[50] >= params.p46)) { 1.0 } else { 0.0 };
        w[125] = noise_activation_schedule_199_0_e2273;
        let noise_activation_schedule_201_0_e2295: f64 = if ((w[48] > 0.0) && (w[48] >= params.p46)) { 1.0 } else { 0.0 };
        w[126] = noise_activation_schedule_201_0_e2295;
        let noise_activation_schedule_203_0_e2317: f64 = if ((w[49] > 0.0) && (w[49] >= params.p46)) { 1.0 } else { 0.0 };
        w[127] = noise_activation_schedule_203_0_e2317;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 128], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_0_0_e456: f64 = ctx.temperature();
            let noise_metadata_schedule_0_0_e458: f64 = (noise_metadata_schedule_0_0_e456 + (ctx.node_voltage(self.nodes[3]) - 0.0));
            let noise_metadata_schedule_0_0_e460: f64 = (noise_metadata_schedule_0_0_e458 + params.p45);
            w[12] = noise_metadata_schedule_0_0_e460;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_1_0_e463: f64 = (1026.85 + 273.15);
            let noise_metadata_schedule_1_0_e466: f64 = (-100.0);
            let noise_metadata_schedule_1_0_e468: f64 = (noise_metadata_schedule_1_0_e466 + 273.15);
            let (noise_metadata_schedule_1_0_e475,) = {
    if (w[12] > noise_metadata_schedule_1_0_e468) {
        (w[12],)
    } else {
        let noise_metadata_schedule_1_0_e472: f64 = (-100.0);
        let noise_metadata_schedule_1_0_e474: f64 = (noise_metadata_schedule_1_0_e472 + 273.15);
        (noise_metadata_schedule_1_0_e474,)
    }
};
            let (noise_metadata_schedule_1_0_e492,) = {
    if (noise_metadata_schedule_1_0_e463 < noise_metadata_schedule_1_0_e475) {
        let noise_metadata_schedule_1_0_e479: f64 = (1026.85 + 273.15);
        (noise_metadata_schedule_1_0_e479,)
    } else {
        let noise_metadata_schedule_1_0_e482: f64 = (-100.0);
        let noise_metadata_schedule_1_0_e484: f64 = (noise_metadata_schedule_1_0_e482 + 273.15);
        let (noise_metadata_schedule_1_0_e491,) = {
            if (w[12] > noise_metadata_schedule_1_0_e484) {
                (w[12],)
            } else {
                let noise_metadata_schedule_1_0_e488: f64 = (-100.0);
                let noise_metadata_schedule_1_0_e490: f64 = (noise_metadata_schedule_1_0_e488 + 273.15);
                (noise_metadata_schedule_1_0_e490,)
            }
        };
        (noise_metadata_schedule_1_0_e491,)
    }
};
            w[10] = noise_metadata_schedule_1_0_e492;
        }
        if (active[0] & 0x7) != 0 {
            let noise_metadata_schedule_4_0_e502: f64 = (params.p43 * params.p42);
            w[3] = noise_metadata_schedule_4_0_e502;
        }
        if (active[0] & 0x18) != 0 {
            let noise_metadata_schedule_5_0_e505: f64 = (params.p29 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            w[95] = noise_metadata_schedule_5_0_e505;
        }
        if (active[0] & 0x18) != 0 {
            let noise_metadata_schedule_6_0_e510: f64 = (w[95]).min(0.0);
            let noise_metadata_schedule_6_0_e511: f64 = (-noise_metadata_schedule_6_0_e510);
            let noise_metadata_schedule_6_0_e513: f64 = (noise_metadata_schedule_6_0_e511).powf(params.p80);
            let noise_metadata_schedule_6_0_e514: f64 = (params.p79 * noise_metadata_schedule_6_0_e513);
            let noise_metadata_schedule_6_0_e515: f64 = (1.0 + noise_metadata_schedule_6_0_e514);
            w[94] = noise_metadata_schedule_6_0_e515;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_7_0_e518: f64 = (params.p25 + 273.15);
            w[11] = noise_metadata_schedule_7_0_e518;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_8_0_e521: f64 = (8.6170869e-5 * w[10]);
            w[15] = noise_metadata_schedule_8_0_e521;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_9_0_e524: f64 = (w[10] / w[11]);
            w[13] = noise_metadata_schedule_9_0_e524;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_10_0_e526: f64 = (w[13]).ln();
            w[14] = noise_metadata_schedule_10_0_e526;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_11_0_e529: f64 = (params.p77 * w[14]);
            let noise_metadata_schedule_11_0_e530: f64 = (noise_metadata_schedule_11_0_e529).exp();
            w[18] = noise_metadata_schedule_11_0_e530;
        }
        if (active[0] & 0x18) != 0 {
            let noise_metadata_schedule_12_0_e533: f64 = (params.p52 * w[18]);
            let noise_metadata_schedule_12_0_e535: f64 = (noise_metadata_schedule_12_0_e533 * w[94]);
            w[16] = noise_metadata_schedule_12_0_e535;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_14_0_e546,) = {
    if (params.p53 > 0.0) {
        let noise_metadata_schedule_14_0_e544: f64 = (1.0 / params.p53);
        (noise_metadata_schedule_14_0_e544,)
    } else {
        (0.0,)
    }
};
            w[64] = noise_metadata_schedule_14_0_e546;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_15_0_e554,) = {
    if (params.p62 > 0.0) {
        let noise_metadata_schedule_15_0_e552: f64 = (1.0 / params.p62);
        (noise_metadata_schedule_15_0_e552,)
    } else {
        (0.0,)
    }
};
            w[65] = noise_metadata_schedule_15_0_e554;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_16_0_e562,) = {
    if (params.p54 > 0.0) {
        let noise_metadata_schedule_16_0_e560: f64 = (1.0 / params.p54);
        (noise_metadata_schedule_16_0_e560,)
    } else {
        (0.0,)
    }
};
            w[66] = noise_metadata_schedule_16_0_e562;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_17_0_e570,) = {
    if (params.p63 > 0.0) {
        let noise_metadata_schedule_17_0_e568: f64 = (1.0 / params.p63);
        (noise_metadata_schedule_17_0_e568,)
    } else {
        (0.0,)
    }
};
            w[67] = noise_metadata_schedule_17_0_e570;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_18_0_e573: f64 = (params.p22 * w[14]);
            let noise_metadata_schedule_18_0_e577: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_18_0_e578: f64 = (params.p21 * noise_metadata_schedule_18_0_e577);
            let noise_metadata_schedule_18_0_e580: f64 = (noise_metadata_schedule_18_0_e578 / w[15]);
            let noise_metadata_schedule_18_0_e581: f64 = (noise_metadata_schedule_18_0_e573 + noise_metadata_schedule_18_0_e580);
            w[68] = noise_metadata_schedule_18_0_e581;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_19_0_e584: f64 = (params.p23 * w[14]);
            w[92] = noise_metadata_schedule_19_0_e584;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_20_0_e587: f64 = (w[68]).exp();
            let noise_metadata_schedule_20_0_e588: f64 = (params.p0 * noise_metadata_schedule_20_0_e587);
            w[19] = noise_metadata_schedule_20_0_e588;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_21_0_e591: f64 = (w[92]).exp();
            let noise_metadata_schedule_21_0_e592: f64 = (params.p2 * noise_metadata_schedule_21_0_e591);
            w[93] = noise_metadata_schedule_21_0_e592;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_22_0_e596: f64 = (w[68] / params.p59);
            let noise_metadata_schedule_22_0_e597: f64 = (noise_metadata_schedule_22_0_e596).exp();
            let noise_metadata_schedule_22_0_e598: f64 = (params.p58 * noise_metadata_schedule_22_0_e597);
            let noise_metadata_schedule_22_0_e600: f64 = (noise_metadata_schedule_22_0_e598 / w[18]);
            w[20] = noise_metadata_schedule_22_0_e600;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_24_0_e614: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_24_0_e615: f64 = (params.p7 * noise_metadata_schedule_24_0_e614);
            let noise_metadata_schedule_24_0_e616: f64 = (1.0 + noise_metadata_schedule_24_0_e615);
            let noise_metadata_schedule_24_0_e617: f64 = (params.p47 * noise_metadata_schedule_24_0_e616);
            w[28] = noise_metadata_schedule_24_0_e617;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_25_0_e623: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_25_0_e624: f64 = (params.p6 * noise_metadata_schedule_25_0_e623);
            let noise_metadata_schedule_25_0_e625: f64 = (1.0 + noise_metadata_schedule_25_0_e624);
            let noise_metadata_schedule_25_0_e626: f64 = (params.p5 * noise_metadata_schedule_25_0_e625);
            w[30] = noise_metadata_schedule_25_0_e626;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_26_0_e632: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_26_0_e633: f64 = (params.p10 * noise_metadata_schedule_26_0_e632);
            let noise_metadata_schedule_26_0_e634: f64 = (1.0 + noise_metadata_schedule_26_0_e633);
            let noise_metadata_schedule_26_0_e635: f64 = (params.p9 * noise_metadata_schedule_26_0_e634);
            w[31] = noise_metadata_schedule_26_0_e635;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_27_0_e641: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_27_0_e642: f64 = (params.p55 * noise_metadata_schedule_27_0_e641);
            let noise_metadata_schedule_27_0_e643: f64 = (1.0 + noise_metadata_schedule_27_0_e642);
            let noise_metadata_schedule_27_0_e644: f64 = (params.p56 * noise_metadata_schedule_27_0_e643);
            w[29] = noise_metadata_schedule_27_0_e644;
        }
        if (active[0] & 0x3b) != 0 {
            w[9] = params.p29;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_66_0_e930: f64 = (w[9] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[6])));
            w[76] = noise_metadata_schedule_66_0_e930;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_67_0_e933: f64 = (w[9] * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[4])));
            w[77] = noise_metadata_schedule_67_0_e933;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_69_0_e939: f64 = (w[9] * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
            w[79] = noise_metadata_schedule_69_0_e939;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_70_0_e942: f64 = (w[9] * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
            w[80] = noise_metadata_schedule_70_0_e942;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_71_0_e945: f64 = if w[19] > 0.0 { 1.0 } else { 0.0 };
            w[105] = noise_metadata_schedule_71_0_e945;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_72_0_e953,) = {
    if (w[105] != 0.0) {
        let noise_metadata_schedule_72_0_e950: f64 = (params.p1 * w[15]);
        let noise_metadata_schedule_72_0_e951: f64 = (w[76] / noise_metadata_schedule_72_0_e950);
        (noise_metadata_schedule_72_0_e951,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_72_0_e953;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_73_0_e964,) = {
    if (w[105] != 0.0) {
        let noise_metadata_schedule_73_0_e956: f64 = (-w[76]);
        let noise_metadata_schedule_73_0_e958: f64 = (noise_metadata_schedule_73_0_e956 - w[30]);
        let noise_metadata_schedule_73_0_e961: f64 = (params.p11 * w[15]);
        let noise_metadata_schedule_73_0_e962: f64 = (noise_metadata_schedule_73_0_e958 / noise_metadata_schedule_73_0_e961);
        (noise_metadata_schedule_73_0_e962,)
    } else {
        (w[90],)
    }
};
            w[90] = noise_metadata_schedule_73_0_e964;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_74_0_e973,) = {
    if (w[105] != 0.0) {
        let noise_metadata_schedule_74_0_e967: f64 = (-w[30]);
        let noise_metadata_schedule_74_0_e970: f64 = (params.p11 * w[15]);
        let noise_metadata_schedule_74_0_e971: f64 = (noise_metadata_schedule_74_0_e967 / noise_metadata_schedule_74_0_e970);
        (noise_metadata_schedule_74_0_e971,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_74_0_e973;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_75_0_e976: f64 = if w[0] > 80.0 { 1.0 } else { 0.0 };
            w[106] = noise_metadata_schedule_75_0_e976;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_76_0_e986,) = {
    if ((w[105] != 0.0) && (w[106] != 0.0)) {
        let noise_metadata_schedule_76_0_e983: f64 = (w[0] - 80.0);
        let noise_metadata_schedule_76_0_e984: f64 = (1.0 + noise_metadata_schedule_76_0_e983);
        (noise_metadata_schedule_76_0_e984,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_76_0_e986;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_77_0_e992,) = {
    if ((w[105] != 0.0) && (w[106] != 0.0)) {
        (80.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_77_0_e992;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_78_0_e999,) = {
    if ((w[105] != 0.0) && (w[106] == 0.0)) {
        (1.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_78_0_e999;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_79_0_e1006,) = {
    if (w[105] != 0.0) {
        let noise_metadata_schedule_79_0_e1003: f64 = (w[0]).exp();
        let noise_metadata_schedule_79_0_e1004: f64 = (w[1] * noise_metadata_schedule_79_0_e1003);
        (noise_metadata_schedule_79_0_e1004,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_79_0_e1006;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_80_0_e1078,) = {
    if (w[105] != 0.0) {
        let noise_metadata_schedule_80_0_e1014: f64 = (-37.0);
        let (noise_metadata_schedule_80_0_e1041,) = {
            if ((!(w[90] >= 37.0)) && (!(w[90] <= noise_metadata_schedule_80_0_e1014))) {
                let noise_metadata_schedule_80_0_e1019: f64 = (w[90]).exp();
                let noise_metadata_schedule_80_0_e1021: f64 = (noise_metadata_schedule_80_0_e1019 + 1.0);
                let noise_metadata_schedule_80_0_e1022: f64 = (noise_metadata_schedule_80_0_e1021).ln();
                (noise_metadata_schedule_80_0_e1022,)
            } else {
                let noise_metadata_schedule_80_0_e1029: f64 = (-37.0);
                let (noise_metadata_schedule_80_0_e1040,) = {
                    if ((!(w[90] >= 37.0)) && (w[90] <= noise_metadata_schedule_80_0_e1029)) {
                        let noise_metadata_schedule_80_0_e1033: f64 = (w[90]).exp();
                        (noise_metadata_schedule_80_0_e1033,)
                    } else {
                        let (noise_metadata_schedule_80_0_e1039,) = {
                            if (w[90] >= 37.0) {
                                (w[90],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_80_0_e1039,)
                    }
                };
                (noise_metadata_schedule_80_0_e1040,)
            }
        };
        let noise_metadata_schedule_80_0_e1048: f64 = (-37.0);
        let (noise_metadata_schedule_80_0_e1075,) = {
            if ((!(w[91] >= 37.0)) && (!(w[91] <= noise_metadata_schedule_80_0_e1048))) {
                let noise_metadata_schedule_80_0_e1053: f64 = (w[91]).exp();
                let noise_metadata_schedule_80_0_e1055: f64 = (noise_metadata_schedule_80_0_e1053 + 1.0);
                let noise_metadata_schedule_80_0_e1056: f64 = (noise_metadata_schedule_80_0_e1055).ln();
                (noise_metadata_schedule_80_0_e1056,)
            } else {
                let noise_metadata_schedule_80_0_e1063: f64 = (-37.0);
                let (noise_metadata_schedule_80_0_e1074,) = {
                    if ((!(w[91] >= 37.0)) && (w[91] <= noise_metadata_schedule_80_0_e1063)) {
                        let noise_metadata_schedule_80_0_e1067: f64 = (w[91]).exp();
                        (noise_metadata_schedule_80_0_e1067,)
                    } else {
                        let (noise_metadata_schedule_80_0_e1073,) = {
                            if (w[91] >= 37.0) {
                                (w[91],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_80_0_e1073,)
                    }
                };
                (noise_metadata_schedule_80_0_e1074,)
            }
        };
        let noise_metadata_schedule_80_0_e1076: f64 = (noise_metadata_schedule_80_0_e1041 - noise_metadata_schedule_80_0_e1075);
        (noise_metadata_schedule_80_0_e1076,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_80_0_e1078;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_81_0_e1099,) = {
    if (w[105] != 0.0) {
        let noise_metadata_schedule_81_0_e1083: f64 = (w[1] - 1.0);
        let noise_metadata_schedule_81_0_e1084: f64 = (w[19] * noise_metadata_schedule_81_0_e1083);
        let noise_metadata_schedule_81_0_e1087: f64 = (w[28] * w[2]);
        let noise_metadata_schedule_81_0_e1091: f64 = (w[76]).abs();
        let noise_metadata_schedule_81_0_e1093: f64 = (noise_metadata_schedule_81_0_e1091).powf(w[31]);
        let noise_metadata_schedule_81_0_e1094: f64 = (params.p8 * noise_metadata_schedule_81_0_e1093);
        let noise_metadata_schedule_81_0_e1095: f64 = (1.0 + noise_metadata_schedule_81_0_e1094);
        let noise_metadata_schedule_81_0_e1096: f64 = (noise_metadata_schedule_81_0_e1087 / noise_metadata_schedule_81_0_e1095);
        let noise_metadata_schedule_81_0_e1097: f64 = (noise_metadata_schedule_81_0_e1084 - noise_metadata_schedule_81_0_e1096);
        (noise_metadata_schedule_81_0_e1097,)
    } else {
        (w[35],)
    }
};
            w[35] = noise_metadata_schedule_81_0_e1099;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_82_0_e1104,) = {
    if (w[105] == 0.0) {
        (0.0,)
    } else {
        (w[35],)
    }
};
            w[35] = noise_metadata_schedule_82_0_e1104;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_83_0_e1107: f64 = if w[93] > 0.0 { 1.0 } else { 0.0 };
            w[107] = noise_metadata_schedule_83_0_e1107;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_84_0_e1115,) = {
    if (w[107] != 0.0) {
        let noise_metadata_schedule_84_0_e1111: f64 = (params.p4 - w[76]);
        let noise_metadata_schedule_84_0_e1113: f64 = (noise_metadata_schedule_84_0_e1111).max(0.001);
        (noise_metadata_schedule_84_0_e1113,)
    } else {
        (w[101],)
    }
};
            w[101] = noise_metadata_schedule_84_0_e1115;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_85_0_e1130,) = {
    if (w[107] != 0.0) {
        let noise_metadata_schedule_85_0_e1118: f64 = (-1.0);
        let noise_metadata_schedule_85_0_e1120: f64 = (noise_metadata_schedule_85_0_e1118 * w[76]);
        let noise_metadata_schedule_85_0_e1122: f64 = (noise_metadata_schedule_85_0_e1120 * params.p4);
        let noise_metadata_schedule_85_0_e1125: f64 = (params.p3 * w[15]);
        let noise_metadata_schedule_85_0_e1127: f64 = (noise_metadata_schedule_85_0_e1125 * w[101]);
        let noise_metadata_schedule_85_0_e1128: f64 = (noise_metadata_schedule_85_0_e1122 / noise_metadata_schedule_85_0_e1127);
        (noise_metadata_schedule_85_0_e1128,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_85_0_e1130;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_86_0_e1133: f64 = if w[0] > 80.0 { 1.0 } else { 0.0 };
            w[108] = noise_metadata_schedule_86_0_e1133;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_87_0_e1143,) = {
    if ((w[107] != 0.0) && (w[108] != 0.0)) {
        let noise_metadata_schedule_87_0_e1140: f64 = (w[0] - 80.0);
        let noise_metadata_schedule_87_0_e1141: f64 = (1.0 + noise_metadata_schedule_87_0_e1140);
        (noise_metadata_schedule_87_0_e1141,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_87_0_e1143;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_88_0_e1149,) = {
    if ((w[107] != 0.0) && (w[108] != 0.0)) {
        (80.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_88_0_e1149;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_89_0_e1156,) = {
    if ((w[107] != 0.0) && (w[108] == 0.0)) {
        (1.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_89_0_e1156;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_90_0_e1163,) = {
    if (w[107] != 0.0) {
        let noise_metadata_schedule_90_0_e1160: f64 = (w[0]).exp();
        let noise_metadata_schedule_90_0_e1161: f64 = (w[1] * noise_metadata_schedule_90_0_e1160);
        (noise_metadata_schedule_90_0_e1161,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_90_0_e1163;
        }
        if (active[0] & 0x18) != 0 {
            let (noise_metadata_schedule_91_0_e1171,) = {
    if (w[107] != 0.0) {
        let noise_metadata_schedule_91_0_e1168: f64 = (w[1] - 1.0);
        let noise_metadata_schedule_91_0_e1169: f64 = (w[93] * noise_metadata_schedule_91_0_e1168);
        (noise_metadata_schedule_91_0_e1169,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_91_0_e1171;
        }
        if (active[0] & 0x18) != 0 {
            let (noise_metadata_schedule_92_0_e1176,) = {
    if (w[107] == 0.0) {
        (0.0,)
    } else {
        (w[47],)
    }
};
            w[47] = noise_metadata_schedule_92_0_e1176;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_93_0_e1179: f64 = if w[20] > 0.0 { 1.0 } else { 0.0 };
            w[109] = noise_metadata_schedule_93_0_e1179;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_94_0_e1187,) = {
    if (w[109] != 0.0) {
        let noise_metadata_schedule_94_0_e1184: f64 = (params.p59 * w[15]);
        let noise_metadata_schedule_94_0_e1185: f64 = (w[76] / noise_metadata_schedule_94_0_e1184);
        (noise_metadata_schedule_94_0_e1185,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_94_0_e1187;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 128], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_95_0_e1198,) = {
    if (w[109] != 0.0) {
        let noise_metadata_schedule_95_0_e1190: f64 = (-w[76]);
        let noise_metadata_schedule_95_0_e1192: f64 = (noise_metadata_schedule_95_0_e1190 - w[30]);
        let noise_metadata_schedule_95_0_e1195: f64 = (params.p57 * w[15]);
        let noise_metadata_schedule_95_0_e1196: f64 = (noise_metadata_schedule_95_0_e1192 / noise_metadata_schedule_95_0_e1195);
        (noise_metadata_schedule_95_0_e1196,)
    } else {
        (w[90],)
    }
};
            w[90] = noise_metadata_schedule_95_0_e1198;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_96_0_e1207,) = {
    if (w[109] != 0.0) {
        let noise_metadata_schedule_96_0_e1201: f64 = (-w[30]);
        let noise_metadata_schedule_96_0_e1204: f64 = (params.p57 * w[15]);
        let noise_metadata_schedule_96_0_e1205: f64 = (noise_metadata_schedule_96_0_e1201 / noise_metadata_schedule_96_0_e1204);
        (noise_metadata_schedule_96_0_e1205,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_96_0_e1207;
        }
        if (active[0] & 0x38) != 0 {
            let noise_metadata_schedule_97_0_e1210: f64 = if w[0] > 80.0 { 1.0 } else { 0.0 };
            w[110] = noise_metadata_schedule_97_0_e1210;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_98_0_e1220,) = {
    if ((w[109] != 0.0) && (w[110] != 0.0)) {
        let noise_metadata_schedule_98_0_e1217: f64 = (w[0] - 80.0);
        let noise_metadata_schedule_98_0_e1218: f64 = (1.0 + noise_metadata_schedule_98_0_e1217);
        (noise_metadata_schedule_98_0_e1218,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_98_0_e1220;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_99_0_e1226,) = {
    if ((w[109] != 0.0) && (w[110] != 0.0)) {
        (80.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_99_0_e1226;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_100_0_e1233,) = {
    if ((w[109] != 0.0) && (w[110] == 0.0)) {
        (1.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_100_0_e1233;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_101_0_e1240,) = {
    if (w[109] != 0.0) {
        let noise_metadata_schedule_101_0_e1237: f64 = (w[0]).exp();
        let noise_metadata_schedule_101_0_e1238: f64 = (w[1] * noise_metadata_schedule_101_0_e1237);
        (noise_metadata_schedule_101_0_e1238,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_101_0_e1240;
        }
        if (active[0] & 0x38) != 0 {
            let (noise_metadata_schedule_102_0_e1312,) = {
    if (w[109] != 0.0) {
        let noise_metadata_schedule_102_0_e1248: f64 = (-37.0);
        let (noise_metadata_schedule_102_0_e1275,) = {
            if ((!(w[90] >= 37.0)) && (!(w[90] <= noise_metadata_schedule_102_0_e1248))) {
                let noise_metadata_schedule_102_0_e1253: f64 = (w[90]).exp();
                let noise_metadata_schedule_102_0_e1255: f64 = (noise_metadata_schedule_102_0_e1253 + 1.0);
                let noise_metadata_schedule_102_0_e1256: f64 = (noise_metadata_schedule_102_0_e1255).ln();
                (noise_metadata_schedule_102_0_e1256,)
            } else {
                let noise_metadata_schedule_102_0_e1263: f64 = (-37.0);
                let (noise_metadata_schedule_102_0_e1274,) = {
                    if ((!(w[90] >= 37.0)) && (w[90] <= noise_metadata_schedule_102_0_e1263)) {
                        let noise_metadata_schedule_102_0_e1267: f64 = (w[90]).exp();
                        (noise_metadata_schedule_102_0_e1267,)
                    } else {
                        let (noise_metadata_schedule_102_0_e1273,) = {
                            if (w[90] >= 37.0) {
                                (w[90],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_102_0_e1273,)
                    }
                };
                (noise_metadata_schedule_102_0_e1274,)
            }
        };
        let noise_metadata_schedule_102_0_e1282: f64 = (-37.0);
        let (noise_metadata_schedule_102_0_e1309,) = {
            if ((!(w[91] >= 37.0)) && (!(w[91] <= noise_metadata_schedule_102_0_e1282))) {
                let noise_metadata_schedule_102_0_e1287: f64 = (w[91]).exp();
                let noise_metadata_schedule_102_0_e1289: f64 = (noise_metadata_schedule_102_0_e1287 + 1.0);
                let noise_metadata_schedule_102_0_e1290: f64 = (noise_metadata_schedule_102_0_e1289).ln();
                (noise_metadata_schedule_102_0_e1290,)
            } else {
                let noise_metadata_schedule_102_0_e1297: f64 = (-37.0);
                let (noise_metadata_schedule_102_0_e1308,) = {
                    if ((!(w[91] >= 37.0)) && (w[91] <= noise_metadata_schedule_102_0_e1297)) {
                        let noise_metadata_schedule_102_0_e1301: f64 = (w[91]).exp();
                        (noise_metadata_schedule_102_0_e1301,)
                    } else {
                        let (noise_metadata_schedule_102_0_e1307,) = {
                            if (w[91] >= 37.0) {
                                (w[91],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_102_0_e1307,)
                    }
                };
                (noise_metadata_schedule_102_0_e1308,)
            }
        };
        let noise_metadata_schedule_102_0_e1310: f64 = (noise_metadata_schedule_102_0_e1275 - noise_metadata_schedule_102_0_e1309);
        (noise_metadata_schedule_102_0_e1310,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_102_0_e1312;
        }
        if (active[0] & 0x18) != 0 {
            let (noise_metadata_schedule_103_0_e1333,) = {
    if (w[109] != 0.0) {
        let noise_metadata_schedule_103_0_e1317: f64 = (w[1] - 1.0);
        let noise_metadata_schedule_103_0_e1318: f64 = (w[20] * noise_metadata_schedule_103_0_e1317);
        let noise_metadata_schedule_103_0_e1325: f64 = (w[76]).abs();
        let noise_metadata_schedule_103_0_e1327: f64 = (noise_metadata_schedule_103_0_e1325).powf(w[31]);
        let noise_metadata_schedule_103_0_e1328: f64 = (params.p8 * noise_metadata_schedule_103_0_e1327);
        let noise_metadata_schedule_103_0_e1329: f64 = (1.0 + noise_metadata_schedule_103_0_e1328);
        let noise_metadata_schedule_103_0_e1330: f64 = 0.0;
        let noise_metadata_schedule_103_0_e1331: f64 = (noise_metadata_schedule_103_0_e1318 - noise_metadata_schedule_103_0_e1330);
        (noise_metadata_schedule_103_0_e1331,)
    } else {
        (w[36],)
    }
};
            w[36] = noise_metadata_schedule_103_0_e1333;
        }
        if (active[0] & 0x18) != 0 {
            let (noise_metadata_schedule_104_0_e1338,) = {
    if (w[109] == 0.0) {
        (0.0,)
    } else {
        (w[36],)
    }
};
            w[36] = noise_metadata_schedule_104_0_e1338;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_105_0_e1341: f64 = if w[19] > 0.0 { 1.0 } else { 0.0 };
            w[111] = noise_metadata_schedule_105_0_e1341;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_106_0_e1349,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_106_0_e1346: f64 = (params.p61 * w[15]);
        let noise_metadata_schedule_106_0_e1347: f64 = (w[77] / noise_metadata_schedule_106_0_e1346);
        (noise_metadata_schedule_106_0_e1347,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_106_0_e1349;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_107_0_e1360,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_107_0_e1352: f64 = (-w[77]);
        let noise_metadata_schedule_107_0_e1354: f64 = (noise_metadata_schedule_107_0_e1352 - w[30]);
        let noise_metadata_schedule_107_0_e1357: f64 = (params.p57 * w[15]);
        let noise_metadata_schedule_107_0_e1358: f64 = (noise_metadata_schedule_107_0_e1354 / noise_metadata_schedule_107_0_e1357);
        (noise_metadata_schedule_107_0_e1358,)
    } else {
        (w[90],)
    }
};
            w[90] = noise_metadata_schedule_107_0_e1360;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_108_0_e1369,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_108_0_e1363: f64 = (-w[30]);
        let noise_metadata_schedule_108_0_e1366: f64 = (params.p57 * w[15]);
        let noise_metadata_schedule_108_0_e1367: f64 = (noise_metadata_schedule_108_0_e1363 / noise_metadata_schedule_108_0_e1366);
        (noise_metadata_schedule_108_0_e1367,)
    } else {
        (w[91],)
    }
};
            w[91] = noise_metadata_schedule_108_0_e1369;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_109_0_e1372: f64 = if w[0] > 80.0 { 1.0 } else { 0.0 };
            w[112] = noise_metadata_schedule_109_0_e1372;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_110_0_e1382,) = {
    if ((w[111] != 0.0) && (w[112] != 0.0)) {
        let noise_metadata_schedule_110_0_e1379: f64 = (w[0] - 80.0);
        let noise_metadata_schedule_110_0_e1380: f64 = (1.0 + noise_metadata_schedule_110_0_e1379);
        (noise_metadata_schedule_110_0_e1380,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_110_0_e1382;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_111_0_e1388,) = {
    if ((w[111] != 0.0) && (w[112] != 0.0)) {
        (80.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_111_0_e1388;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_112_0_e1395,) = {
    if ((w[111] != 0.0) && (w[112] == 0.0)) {
        (1.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_112_0_e1395;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_113_0_e1402,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_113_0_e1399: f64 = (w[0]).exp();
        let noise_metadata_schedule_113_0_e1400: f64 = (w[1] * noise_metadata_schedule_113_0_e1399);
        (noise_metadata_schedule_113_0_e1400,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_113_0_e1402;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_114_0_e1474,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_114_0_e1410: f64 = (-37.0);
        let (noise_metadata_schedule_114_0_e1437,) = {
            if ((!(w[90] >= 37.0)) && (!(w[90] <= noise_metadata_schedule_114_0_e1410))) {
                let noise_metadata_schedule_114_0_e1415: f64 = (w[90]).exp();
                let noise_metadata_schedule_114_0_e1417: f64 = (noise_metadata_schedule_114_0_e1415 + 1.0);
                let noise_metadata_schedule_114_0_e1418: f64 = (noise_metadata_schedule_114_0_e1417).ln();
                (noise_metadata_schedule_114_0_e1418,)
            } else {
                let noise_metadata_schedule_114_0_e1425: f64 = (-37.0);
                let (noise_metadata_schedule_114_0_e1436,) = {
                    if ((!(w[90] >= 37.0)) && (w[90] <= noise_metadata_schedule_114_0_e1425)) {
                        let noise_metadata_schedule_114_0_e1429: f64 = (w[90]).exp();
                        (noise_metadata_schedule_114_0_e1429,)
                    } else {
                        let (noise_metadata_schedule_114_0_e1435,) = {
                            if (w[90] >= 37.0) {
                                (w[90],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_114_0_e1435,)
                    }
                };
                (noise_metadata_schedule_114_0_e1436,)
            }
        };
        let noise_metadata_schedule_114_0_e1444: f64 = (-37.0);
        let (noise_metadata_schedule_114_0_e1471,) = {
            if ((!(w[91] >= 37.0)) && (!(w[91] <= noise_metadata_schedule_114_0_e1444))) {
                let noise_metadata_schedule_114_0_e1449: f64 = (w[91]).exp();
                let noise_metadata_schedule_114_0_e1451: f64 = (noise_metadata_schedule_114_0_e1449 + 1.0);
                let noise_metadata_schedule_114_0_e1452: f64 = (noise_metadata_schedule_114_0_e1451).ln();
                (noise_metadata_schedule_114_0_e1452,)
            } else {
                let noise_metadata_schedule_114_0_e1459: f64 = (-37.0);
                let (noise_metadata_schedule_114_0_e1470,) = {
                    if ((!(w[91] >= 37.0)) && (w[91] <= noise_metadata_schedule_114_0_e1459)) {
                        let noise_metadata_schedule_114_0_e1463: f64 = (w[91]).exp();
                        (noise_metadata_schedule_114_0_e1463,)
                    } else {
                        let (noise_metadata_schedule_114_0_e1469,) = {
                            if (w[91] >= 37.0) {
                                (w[91],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_114_0_e1469,)
                    }
                };
                (noise_metadata_schedule_114_0_e1470,)
            }
        };
        let noise_metadata_schedule_114_0_e1472: f64 = (noise_metadata_schedule_114_0_e1437 - noise_metadata_schedule_114_0_e1471);
        (noise_metadata_schedule_114_0_e1472,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_114_0_e1474;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_115_0_e1495,) = {
    if (w[111] != 0.0) {
        let noise_metadata_schedule_115_0_e1479: f64 = (w[1] - 1.0);
        let noise_metadata_schedule_115_0_e1480: f64 = (w[19] * noise_metadata_schedule_115_0_e1479);
        let noise_metadata_schedule_115_0_e1483: f64 = (w[29] * w[2]);
        let noise_metadata_schedule_115_0_e1487: f64 = (w[77]).abs();
        let noise_metadata_schedule_115_0_e1489: f64 = (noise_metadata_schedule_115_0_e1487).powf(w[31]);
        let noise_metadata_schedule_115_0_e1490: f64 = (params.p8 * noise_metadata_schedule_115_0_e1489);
        let noise_metadata_schedule_115_0_e1491: f64 = (1.0 + noise_metadata_schedule_115_0_e1490);
        let noise_metadata_schedule_115_0_e1492: f64 = (noise_metadata_schedule_115_0_e1483 / noise_metadata_schedule_115_0_e1491);
        let noise_metadata_schedule_115_0_e1493: f64 = (noise_metadata_schedule_115_0_e1480 - noise_metadata_schedule_115_0_e1492);
        (noise_metadata_schedule_115_0_e1493,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_115_0_e1495;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_116_0_e1500,) = {
    if (w[111] == 0.0) {
        (0.0,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_116_0_e1500;
        }
        if (active[0] & 0x18) != 0 {
            let noise_metadata_schedule_130_0_e1674: f64 = (w[35] - w[47]);
            let noise_metadata_schedule_130_0_e1676: f64 = (noise_metadata_schedule_130_0_e1674 / w[16]);
            let noise_metadata_schedule_130_0_e1678: f64 = (noise_metadata_schedule_130_0_e1676 + w[36]);
            w[37] = noise_metadata_schedule_130_0_e1678;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_132_0_e1688: f64 = (w[77] * params.p81);
            let noise_metadata_schedule_132_0_e1689: f64 = (1.0 + noise_metadata_schedule_132_0_e1688);
            let noise_metadata_schedule_132_0_e1690: f64 = (w[66] * noise_metadata_schedule_132_0_e1689);
            w[66] = noise_metadata_schedule_132_0_e1690;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_133_0_e1693: f64 = (w[35] * w[66]);
            let noise_metadata_schedule_133_0_e1696: f64 = (w[38] * w[67]);
            let noise_metadata_schedule_133_0_e1697: f64 = (noise_metadata_schedule_133_0_e1693 + noise_metadata_schedule_133_0_e1696);
            w[42] = noise_metadata_schedule_133_0_e1697;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_134_0_e1701: f64 = (w[76] * w[65]);
            let noise_metadata_schedule_134_0_e1702: f64 = (1.0 - noise_metadata_schedule_134_0_e1701);
            let noise_metadata_schedule_134_0_e1705: f64 = (w[77] * w[64]);
            let noise_metadata_schedule_134_0_e1706: f64 = (noise_metadata_schedule_134_0_e1702 - noise_metadata_schedule_134_0_e1705);
            w[41] = noise_metadata_schedule_134_0_e1706;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_135_0_e1711: f64 = (4.0 * w[42]);
            let noise_metadata_schedule_135_0_e1712: f64 = (1.0 + noise_metadata_schedule_135_0_e1711);
            let noise_metadata_schedule_135_0_e1713: f64 = (noise_metadata_schedule_135_0_e1712).abs();
            let noise_metadata_schedule_135_0_e1715: f64 = (noise_metadata_schedule_135_0_e1713).powf(params.p82);
            let noise_metadata_schedule_135_0_e1716: f64 = (1.0 + noise_metadata_schedule_135_0_e1715);
            w[96] = noise_metadata_schedule_135_0_e1716;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_136_0_e1719: f64 = (2.0 * w[41]);
            let noise_metadata_schedule_136_0_e1721: f64 = (noise_metadata_schedule_136_0_e1719 / w[96]);
            w[43] = noise_metadata_schedule_136_0_e1721;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_137_0_e1724: f64 = (w[38] * w[43]);
            w[45] = noise_metadata_schedule_137_0_e1724;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_138_0_e1727: f64 = (w[35] * w[43]);
            w[44] = noise_metadata_schedule_138_0_e1727;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_140_0_e1746: f64 = (w[79] / params.p48);
            let noise_metadata_schedule_140_0_e1747: f64 = (noise_metadata_schedule_140_0_e1746).abs();
            let noise_metadata_schedule_140_0_e1749: f64 = (noise_metadata_schedule_140_0_e1747).powf(params.p49);
            let noise_metadata_schedule_140_0_e1750: f64 = (1.0 + noise_metadata_schedule_140_0_e1749);
            w[99] = noise_metadata_schedule_140_0_e1750;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_141_0_e1754: f64 = (w[80] / params.p50);
            let noise_metadata_schedule_141_0_e1755: f64 = (noise_metadata_schedule_141_0_e1754).abs();
            let noise_metadata_schedule_141_0_e1757: f64 = (noise_metadata_schedule_141_0_e1755).powf(params.p51);
            let noise_metadata_schedule_141_0_e1758: f64 = (1.0 + noise_metadata_schedule_141_0_e1757);
            w[100] = noise_metadata_schedule_141_0_e1758;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_142_0_e1762: f64 = (w[14] * params.p37);
            let noise_metadata_schedule_142_0_e1763: f64 = (noise_metadata_schedule_142_0_e1762).exp();
            let noise_metadata_schedule_142_0_e1764: f64 = (params.p12 * noise_metadata_schedule_142_0_e1763);
            let noise_metadata_schedule_142_0_e1768: f64 = (1.0 / params.p49);
            let noise_metadata_schedule_142_0_e1769: f64 = (w[99]).powf(noise_metadata_schedule_142_0_e1768);
            let noise_metadata_schedule_142_0_e1770: f64 = (noise_metadata_schedule_142_0_e1764 * noise_metadata_schedule_142_0_e1769);
            w[51] = noise_metadata_schedule_142_0_e1770;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_143_0_e1774: f64 = (w[14] * params.p78);
            let noise_metadata_schedule_143_0_e1775: f64 = (noise_metadata_schedule_143_0_e1774).exp();
            let noise_metadata_schedule_143_0_e1776: f64 = (params.p66 * noise_metadata_schedule_143_0_e1775);
            w[52] = noise_metadata_schedule_143_0_e1776;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_144_0_e1780: f64 = (w[14] * params.p38);
            let noise_metadata_schedule_144_0_e1781: f64 = (noise_metadata_schedule_144_0_e1780).exp();
            let noise_metadata_schedule_144_0_e1782: f64 = (params.p14 * noise_metadata_schedule_144_0_e1781);
            let noise_metadata_schedule_144_0_e1786: f64 = (1.0 / params.p51);
            let noise_metadata_schedule_144_0_e1787: f64 = (w[100]).powf(noise_metadata_schedule_144_0_e1786);
            let noise_metadata_schedule_144_0_e1788: f64 = (noise_metadata_schedule_144_0_e1782 * noise_metadata_schedule_144_0_e1787);
            w[53] = noise_metadata_schedule_144_0_e1788;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_150_0_e1819: f64 = if params.p32 == 1.0 { 1.0 } else { 0.0 };
            w[115] = noise_metadata_schedule_150_0_e1819;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_151_0_e1832,) = {
    if (w[115] != 0.0) {
        let noise_metadata_schedule_151_0_e1824: f64 = ((ctx.node_voltage(self.nodes[8]) - 0.0)).abs();
        let noise_metadata_schedule_151_0_e1826: f64 = (noise_metadata_schedule_151_0_e1824 / params.p20);
        let noise_metadata_schedule_151_0_e1828: f64 = (noise_metadata_schedule_151_0_e1826).powf(params.p44);
        let noise_metadata_schedule_151_0_e1829: f64 = (1.0 + noise_metadata_schedule_151_0_e1828);
        let noise_metadata_schedule_151_0_e1830: f64 = (w[51] / noise_metadata_schedule_151_0_e1829);
        (noise_metadata_schedule_151_0_e1830,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_151_0_e1832;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_152_0_e1837,) = {
    if (w[115] == 0.0) {
        (w[51],)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_152_0_e1837;
        }
        if (active[0] & 0x7) != 0 {
            let noise_metadata_schedule_153_0_e1840: f64 = if params.p31 == 1.0 { 1.0 } else { 0.0 };
            w[116] = noise_metadata_schedule_153_0_e1840;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_154_0_e1846,) = {
    if (w[116] != 0.0) {
        let noise_metadata_schedule_154_0_e1844: f64 = (w[51] + params.p13);
        (noise_metadata_schedule_154_0_e1844,)
    } else {
        (w[51],)
    }
};
            w[51] = noise_metadata_schedule_154_0_e1846;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_155_0_e1852,) = {
    if (w[116] != 0.0) {
        let noise_metadata_schedule_155_0_e1850: f64 = (w[52] + params.p67);
        (noise_metadata_schedule_155_0_e1850,)
    } else {
        (w[52],)
    }
};
            w[52] = noise_metadata_schedule_155_0_e1852;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_156_0_e1858,) = {
    if (w[116] != 0.0) {
        let noise_metadata_schedule_156_0_e1856: f64 = (w[53] + params.p15);
        (noise_metadata_schedule_156_0_e1856,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_156_0_e1858;
        }
        if (active[0] & 0x7) != 0 {
            let noise_metadata_schedule_195_0_e2243: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_195_0_e2245: f64 = (noise_metadata_schedule_195_0_e2243 * w[10]);
            w[69] = noise_metadata_schedule_195_0_e2245;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 128], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_196_0_e2249: f64 = (params.p31 * params.p13);
            let noise_metadata_schedule_196_0_e2250: f64 = (params.p12 + noise_metadata_schedule_196_0_e2249);
            let noise_metadata_schedule_196_0_e2252: f64 = (noise_metadata_schedule_196_0_e2250 / w[3]);
            w[50] = noise_metadata_schedule_196_0_e2252;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_197_0_e2256: f64 = (params.p31 * params.p15);
            let noise_metadata_schedule_197_0_e2257: f64 = (params.p14 + noise_metadata_schedule_197_0_e2256);
            let noise_metadata_schedule_197_0_e2259: f64 = (noise_metadata_schedule_197_0_e2257 / w[3]);
            w[48] = noise_metadata_schedule_197_0_e2259;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_198_0_e2263: f64 = (params.p31 * params.p67);
            let noise_metadata_schedule_198_0_e2264: f64 = (params.p66 + noise_metadata_schedule_198_0_e2263);
            let noise_metadata_schedule_198_0_e2266: f64 = (noise_metadata_schedule_198_0_e2264 / w[3]);
            w[49] = noise_metadata_schedule_198_0_e2266;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_199_0_e2273: f64 = if ((w[50] > 0.0) && (w[50] >= params.p46)) { 1.0 } else { 0.0 };
            w[125] = noise_metadata_schedule_199_0_e2273;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_200_0_e2288,) = {
    if (w[125] != 0.0) {
        let noise_metadata_schedule_200_0_e2277: f64 = (w[51] / w[3]);
        let (noise_metadata_schedule_200_0_e2286,) = {
            if (noise_metadata_schedule_200_0_e2277 >= params.p46) {
                let noise_metadata_schedule_200_0_e2283: f64 = (w[51] / w[3]);
                let noise_metadata_schedule_200_0_e2284: f64 = (w[69] / noise_metadata_schedule_200_0_e2283);
                (noise_metadata_schedule_200_0_e2284,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_200_0_e2286,)
    } else {
        (w[72],)
    }
};
            w[72] = noise_metadata_schedule_200_0_e2288;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_201_0_e2295: f64 = if ((w[48] > 0.0) && (w[48] >= params.p46)) { 1.0 } else { 0.0 };
            w[126] = noise_metadata_schedule_201_0_e2295;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_202_0_e2310,) = {
    if (w[126] != 0.0) {
        let noise_metadata_schedule_202_0_e2299: f64 = (w[53] / w[3]);
        let (noise_metadata_schedule_202_0_e2308,) = {
            if (noise_metadata_schedule_202_0_e2299 >= params.p46) {
                let noise_metadata_schedule_202_0_e2305: f64 = (w[53] / w[3]);
                let noise_metadata_schedule_202_0_e2306: f64 = (w[69] / noise_metadata_schedule_202_0_e2305);
                (noise_metadata_schedule_202_0_e2306,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_202_0_e2308,)
    } else {
        (w[73],)
    }
};
            w[73] = noise_metadata_schedule_202_0_e2310;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_203_0_e2317: f64 = if ((w[49] > 0.0) && (w[49] >= params.p46)) { 1.0 } else { 0.0 };
            w[127] = noise_metadata_schedule_203_0_e2317;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_204_0_e2332,) = {
    if (w[127] != 0.0) {
        let noise_metadata_schedule_204_0_e2321: f64 = (w[52] / w[3]);
        let (noise_metadata_schedule_204_0_e2330,) = {
            if (noise_metadata_schedule_204_0_e2321 >= params.p46) {
                let noise_metadata_schedule_204_0_e2327: f64 = (w[52] / w[3]);
                let noise_metadata_schedule_204_0_e2328: f64 = (w[69] / noise_metadata_schedule_204_0_e2327);
                (noise_metadata_schedule_204_0_e2328,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_204_0_e2330,)
    } else {
        (w[74],)
    }
};
            w[74] = noise_metadata_schedule_204_0_e2332;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_205_0_e2339: f64 = if ((params.p28 > 0.0) && (params.p27 > 0.0)) { 1.0 } else { 0.0 };
            let (noise_metadata_schedule_205_0_e2349,) = {
    if (noise_metadata_schedule_205_0_e2339 > 0.0) {
        let noise_metadata_schedule_205_0_e2344: f64 = (w[37]).abs();
        let noise_metadata_schedule_205_0_e2346: f64 = (noise_metadata_schedule_205_0_e2344).powf(params.p28);
        let noise_metadata_schedule_205_0_e2347: f64 = (params.p27 * noise_metadata_schedule_205_0_e2346);
        (noise_metadata_schedule_205_0_e2347,)
    } else {
        (0.0,)
    }
};
            w[71] = noise_metadata_schedule_205_0_e2349;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_206_0_e2352: f64 = (2.0 * 1.6021918e-19);
            w[70] = noise_metadata_schedule_206_0_e2352;
        }
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 4] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("Rb"), kind: GeneratedNoiseKind::White, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE", label: Some("Re"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER_IBE", label: Some("flicker_Ibe"), kind: GeneratedNoiseKind::Flicker, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("Ibe"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 75];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            w[73] != 0.0
        };
        let noise_source_1_active = {
            w[74] != 0.0
        };
        let noise_source_2_active = {
            true
        };
        let noise_source_3_active = {
            true
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e1049: f64 = 1.0;
            let noise_0_psd_e1050: f64 = (noise_0_psd_e1049 * w[38]);
            let psd = noise_0_psd_e1050;
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
            let noise_1_psd_e1052: f64 = 1.0;
            let noise_1_psd_e1053: f64 = (noise_1_psd_e1052 * w[39]);
            let psd = noise_1_psd_e1053;
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
            let noise_2_psd_e1055: f64 = 1.0;
            let noise_2_psd_e281: f64 = (w[9] * w[24]);
            let (noise_2_psd_e288,) = {
    if (noise_2_psd_e281 >= 0.0) {
        let noise_2_psd_e285: f64 = 1.0;
        (noise_2_psd_e285,)
    } else {
        let noise_2_psd_e287: f64 = (-1.0);
        (noise_2_psd_e287,)
    }
};
            let noise_2_psd_e290: f64 = (noise_2_psd_e288 * w[37]);
            let noise_2_psd_e1056: f64 = (noise_2_psd_e1055 * noise_2_psd_e290);
            let psd = noise_2_psd_e1056;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = Some(1.0);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[3] {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_3_psd_e1058: f64 = 1.0;
            let noise_3_psd_e296: f64 = (w[24]).abs();
            let noise_3_psd_e297: f64 = (w[36] * noise_3_psd_e296);
            let noise_3_psd_e1059: f64 = (noise_3_psd_e1058 * noise_3_psd_e297);
            let psd = noise_3_psd_e1059;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 75]) {
        let params = &*self.params;
        let noise_activation_schedule_4_0_e346: f64 = (params[43] * params[42]);
        w[3] = noise_activation_schedule_4_0_e346;
        let noise_activation_schedule_83_0_e973: f64 = (params[31] * params[13]);
        let noise_activation_schedule_83_0_e974: f64 = (params[12] + noise_activation_schedule_83_0_e973);
        let noise_activation_schedule_83_0_e976: f64 = (noise_activation_schedule_83_0_e974 / w[3]);
        w[28] = noise_activation_schedule_83_0_e976;
        let noise_activation_schedule_84_0_e980: f64 = (params[31] * params[15]);
        let noise_activation_schedule_84_0_e981: f64 = (params[14] + noise_activation_schedule_84_0_e980);
        let noise_activation_schedule_84_0_e983: f64 = (noise_activation_schedule_84_0_e981 / w[3]);
        w[27] = noise_activation_schedule_84_0_e983;
        let noise_activation_schedule_85_0_e990: f64 = if ((w[28] > 0.0) && (w[28] >= params[46])) { 1.0 } else { 0.0 };
        w[73] = noise_activation_schedule_85_0_e990;
        let noise_activation_schedule_87_0_e1012: f64 = if ((w[27] > 0.0) && (w[27] >= params[46])) { 1.0 } else { 0.0 };
        w[74] = noise_activation_schedule_87_0_e1012;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 75], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_0_0_e300: f64 = ctx.temperature();
            let noise_metadata_schedule_0_0_e302: f64 = (noise_metadata_schedule_0_0_e300 + (ctx.node_voltage(self.nodes[2]) - 0.0));
            let noise_metadata_schedule_0_0_e304: f64 = (noise_metadata_schedule_0_0_e302 + params[45]);
            w[12] = noise_metadata_schedule_0_0_e304;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_1_0_e307: f64 = (1026.85 + 273.15);
            let noise_metadata_schedule_1_0_e310: f64 = (-100.0);
            let noise_metadata_schedule_1_0_e312: f64 = (noise_metadata_schedule_1_0_e310 + 273.15);
            let (noise_metadata_schedule_1_0_e319,) = {
    if (w[12] > noise_metadata_schedule_1_0_e312) {
        (w[12],)
    } else {
        let noise_metadata_schedule_1_0_e316: f64 = (-100.0);
        let noise_metadata_schedule_1_0_e318: f64 = (noise_metadata_schedule_1_0_e316 + 273.15);
        (noise_metadata_schedule_1_0_e318,)
    }
};
            let (noise_metadata_schedule_1_0_e336,) = {
    if (noise_metadata_schedule_1_0_e307 < noise_metadata_schedule_1_0_e319) {
        let noise_metadata_schedule_1_0_e323: f64 = (1026.85 + 273.15);
        (noise_metadata_schedule_1_0_e323,)
    } else {
        let noise_metadata_schedule_1_0_e326: f64 = (-100.0);
        let noise_metadata_schedule_1_0_e328: f64 = (noise_metadata_schedule_1_0_e326 + 273.15);
        let (noise_metadata_schedule_1_0_e335,) = {
            if (w[12] > noise_metadata_schedule_1_0_e328) {
                (w[12],)
            } else {
                let noise_metadata_schedule_1_0_e332: f64 = (-100.0);
                let noise_metadata_schedule_1_0_e334: f64 = (noise_metadata_schedule_1_0_e332 + 273.15);
                (noise_metadata_schedule_1_0_e334,)
            }
        };
        (noise_metadata_schedule_1_0_e335,)
    }
};
            w[10] = noise_metadata_schedule_1_0_e336;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_4_0_e346: f64 = (params[43] * params[42]);
            w[3] = noise_metadata_schedule_4_0_e346;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_5_0_e349: f64 = (params[25] + 273.15);
            w[11] = noise_metadata_schedule_5_0_e349;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_6_0_e352: f64 = (8.6170869e-5 * w[10]);
            w[15] = noise_metadata_schedule_6_0_e352;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_7_0_e355: f64 = (w[10] / w[11]);
            w[13] = noise_metadata_schedule_7_0_e355;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_8_0_e357: f64 = (w[13]).ln();
            w[14] = noise_metadata_schedule_8_0_e357;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_9_0_e360: f64 = (params[22] * w[14]);
            let noise_metadata_schedule_9_0_e364: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_9_0_e365: f64 = (params[21] * noise_metadata_schedule_9_0_e364);
            let noise_metadata_schedule_9_0_e367: f64 = (noise_metadata_schedule_9_0_e365 / w[15]);
            let noise_metadata_schedule_9_0_e368: f64 = (noise_metadata_schedule_9_0_e360 + noise_metadata_schedule_9_0_e367);
            w[34] = noise_metadata_schedule_9_0_e368;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_10_0_e371: f64 = (params[23] * w[14]);
            w[54] = noise_metadata_schedule_10_0_e371;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_11_0_e374: f64 = (w[34]).exp();
            let noise_metadata_schedule_11_0_e375: f64 = (params[0] * noise_metadata_schedule_11_0_e374);
            w[16] = noise_metadata_schedule_11_0_e375;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_12_0_e378: f64 = (w[54]).exp();
            let noise_metadata_schedule_12_0_e379: f64 = (params[2] * noise_metadata_schedule_12_0_e378);
            w[55] = noise_metadata_schedule_12_0_e379;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_13_0_e385: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_13_0_e386: f64 = (params[7] * noise_metadata_schedule_13_0_e385);
            let noise_metadata_schedule_13_0_e387: f64 = (1.0 + noise_metadata_schedule_13_0_e386);
            let noise_metadata_schedule_13_0_e388: f64 = (params[47] * noise_metadata_schedule_13_0_e387);
            w[19] = noise_metadata_schedule_13_0_e388;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_14_0_e394: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_14_0_e395: f64 = (params[6] * noise_metadata_schedule_14_0_e394);
            let noise_metadata_schedule_14_0_e396: f64 = (1.0 + noise_metadata_schedule_14_0_e395);
            let noise_metadata_schedule_14_0_e397: f64 = (params[5] * noise_metadata_schedule_14_0_e396);
            w[20] = noise_metadata_schedule_14_0_e397;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_15_0_e403: f64 = (w[13] - 1.0);
            let noise_metadata_schedule_15_0_e404: f64 = (params[10] * noise_metadata_schedule_15_0_e403);
            let noise_metadata_schedule_15_0_e405: f64 = (1.0 + noise_metadata_schedule_15_0_e404);
            let noise_metadata_schedule_15_0_e406: f64 = (params[9] * noise_metadata_schedule_15_0_e405);
            w[21] = noise_metadata_schedule_15_0_e406;
        }
        if (active[0] & 0xf) != 0 {
            w[9] = params[29];
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_29_0_e503: f64 = (w[9] * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[4])));
            w[40] = noise_metadata_schedule_29_0_e503;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_30_0_e506: f64 = (w[9] * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[3])));
            w[41] = noise_metadata_schedule_30_0_e506;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_31_0_e509: f64 = (w[9] * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[4])));
            w[42] = noise_metadata_schedule_31_0_e509;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_32_0_e512: f64 = if w[16] > 0.0 { 1.0 } else { 0.0 };
            w[63] = noise_metadata_schedule_32_0_e512;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_33_0_e520,) = {
    if (w[63] != 0.0) {
        let noise_metadata_schedule_33_0_e517: f64 = (params[1] * w[15]);
        let noise_metadata_schedule_33_0_e518: f64 = (w[40] / noise_metadata_schedule_33_0_e517);
        (noise_metadata_schedule_33_0_e518,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_33_0_e520;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_34_0_e531,) = {
    if (w[63] != 0.0) {
        let noise_metadata_schedule_34_0_e523: f64 = (-w[40]);
        let noise_metadata_schedule_34_0_e525: f64 = (noise_metadata_schedule_34_0_e523 - w[20]);
        let noise_metadata_schedule_34_0_e528: f64 = (params[11] * w[15]);
        let noise_metadata_schedule_34_0_e529: f64 = (noise_metadata_schedule_34_0_e525 / noise_metadata_schedule_34_0_e528);
        (noise_metadata_schedule_34_0_e529,)
    } else {
        (w[52],)
    }
};
            w[52] = noise_metadata_schedule_34_0_e531;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_35_0_e540,) = {
    if (w[63] != 0.0) {
        let noise_metadata_schedule_35_0_e534: f64 = (-w[20]);
        let noise_metadata_schedule_35_0_e537: f64 = (params[11] * w[15]);
        let noise_metadata_schedule_35_0_e538: f64 = (noise_metadata_schedule_35_0_e534 / noise_metadata_schedule_35_0_e537);
        (noise_metadata_schedule_35_0_e538,)
    } else {
        (w[53],)
    }
};
            w[53] = noise_metadata_schedule_35_0_e540;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_36_0_e543: f64 = if w[0] > 80.0 { 1.0 } else { 0.0 };
            w[64] = noise_metadata_schedule_36_0_e543;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_37_0_e553,) = {
    if ((w[63] != 0.0) && (w[64] != 0.0)) {
        let noise_metadata_schedule_37_0_e550: f64 = (w[0] - 80.0);
        let noise_metadata_schedule_37_0_e551: f64 = (1.0 + noise_metadata_schedule_37_0_e550);
        (noise_metadata_schedule_37_0_e551,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_37_0_e553;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_38_0_e559,) = {
    if ((w[63] != 0.0) && (w[64] != 0.0)) {
        (80.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_38_0_e559;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_39_0_e566,) = {
    if ((w[63] != 0.0) && (w[64] == 0.0)) {
        (1.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_39_0_e566;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_40_0_e573,) = {
    if (w[63] != 0.0) {
        let noise_metadata_schedule_40_0_e570: f64 = (w[0]).exp();
        let noise_metadata_schedule_40_0_e571: f64 = (w[1] * noise_metadata_schedule_40_0_e570);
        (noise_metadata_schedule_40_0_e571,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_40_0_e573;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_41_0_e645,) = {
    if (w[63] != 0.0) {
        let noise_metadata_schedule_41_0_e581: f64 = (-37.0);
        let (noise_metadata_schedule_41_0_e608,) = {
            if ((!(w[52] >= 37.0)) && (!(w[52] <= noise_metadata_schedule_41_0_e581))) {
                let noise_metadata_schedule_41_0_e586: f64 = (w[52]).exp();
                let noise_metadata_schedule_41_0_e588: f64 = (noise_metadata_schedule_41_0_e586 + 1.0);
                let noise_metadata_schedule_41_0_e589: f64 = (noise_metadata_schedule_41_0_e588).ln();
                (noise_metadata_schedule_41_0_e589,)
            } else {
                let noise_metadata_schedule_41_0_e596: f64 = (-37.0);
                let (noise_metadata_schedule_41_0_e607,) = {
                    if ((!(w[52] >= 37.0)) && (w[52] <= noise_metadata_schedule_41_0_e596)) {
                        let noise_metadata_schedule_41_0_e600: f64 = (w[52]).exp();
                        (noise_metadata_schedule_41_0_e600,)
                    } else {
                        let (noise_metadata_schedule_41_0_e606,) = {
                            if (w[52] >= 37.0) {
                                (w[52],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_41_0_e606,)
                    }
                };
                (noise_metadata_schedule_41_0_e607,)
            }
        };
        let noise_metadata_schedule_41_0_e615: f64 = (-37.0);
        let (noise_metadata_schedule_41_0_e642,) = {
            if ((!(w[53] >= 37.0)) && (!(w[53] <= noise_metadata_schedule_41_0_e615))) {
                let noise_metadata_schedule_41_0_e620: f64 = (w[53]).exp();
                let noise_metadata_schedule_41_0_e622: f64 = (noise_metadata_schedule_41_0_e620 + 1.0);
                let noise_metadata_schedule_41_0_e623: f64 = (noise_metadata_schedule_41_0_e622).ln();
                (noise_metadata_schedule_41_0_e623,)
            } else {
                let noise_metadata_schedule_41_0_e630: f64 = (-37.0);
                let (noise_metadata_schedule_41_0_e641,) = {
                    if ((!(w[53] >= 37.0)) && (w[53] <= noise_metadata_schedule_41_0_e630)) {
                        let noise_metadata_schedule_41_0_e634: f64 = (w[53]).exp();
                        (noise_metadata_schedule_41_0_e634,)
                    } else {
                        let (noise_metadata_schedule_41_0_e640,) = {
                            if (w[53] >= 37.0) {
                                (w[53],)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_41_0_e640,)
                    }
                };
                (noise_metadata_schedule_41_0_e641,)
            }
        };
        let noise_metadata_schedule_41_0_e643: f64 = (noise_metadata_schedule_41_0_e608 - noise_metadata_schedule_41_0_e642);
        (noise_metadata_schedule_41_0_e643,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_41_0_e645;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_42_0_e666,) = {
    if (w[63] != 0.0) {
        let noise_metadata_schedule_42_0_e650: f64 = (w[1] - 1.0);
        let noise_metadata_schedule_42_0_e651: f64 = (w[16] * noise_metadata_schedule_42_0_e650);
        let noise_metadata_schedule_42_0_e654: f64 = (w[19] * w[2]);
        let noise_metadata_schedule_42_0_e658: f64 = (w[40]).abs();
        let noise_metadata_schedule_42_0_e660: f64 = (noise_metadata_schedule_42_0_e658).powf(w[21]);
        let noise_metadata_schedule_42_0_e661: f64 = (params[8] * noise_metadata_schedule_42_0_e660);
        let noise_metadata_schedule_42_0_e662: f64 = (1.0 + noise_metadata_schedule_42_0_e661);
        let noise_metadata_schedule_42_0_e663: f64 = (noise_metadata_schedule_42_0_e654 / noise_metadata_schedule_42_0_e662);
        let noise_metadata_schedule_42_0_e664: f64 = (noise_metadata_schedule_42_0_e651 - noise_metadata_schedule_42_0_e663);
        (noise_metadata_schedule_42_0_e664,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_42_0_e666;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_43_0_e671,) = {
    if (w[63] == 0.0) {
        (0.0,)
    } else {
        (w[23],)
    }
};
            w[23] = noise_metadata_schedule_43_0_e671;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_44_0_e674: f64 = if w[55] > 0.0 { 1.0 } else { 0.0 };
            w[65] = noise_metadata_schedule_44_0_e674;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_45_0_e682,) = {
    if (w[65] != 0.0) {
        let noise_metadata_schedule_45_0_e678: f64 = (params[4] - w[40]);
        let noise_metadata_schedule_45_0_e680: f64 = (noise_metadata_schedule_45_0_e678).max(0.001);
        (noise_metadata_schedule_45_0_e680,)
    } else {
        (w[60],)
    }
};
            w[60] = noise_metadata_schedule_45_0_e682;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_46_0_e697,) = {
    if (w[65] != 0.0) {
        let noise_metadata_schedule_46_0_e685: f64 = (-1.0);
        let noise_metadata_schedule_46_0_e687: f64 = (noise_metadata_schedule_46_0_e685 * w[40]);
        let noise_metadata_schedule_46_0_e689: f64 = (noise_metadata_schedule_46_0_e687 * params[4]);
        let noise_metadata_schedule_46_0_e692: f64 = (params[3] * w[15]);
        let noise_metadata_schedule_46_0_e694: f64 = (noise_metadata_schedule_46_0_e692 * w[60]);
        let noise_metadata_schedule_46_0_e695: f64 = (noise_metadata_schedule_46_0_e689 / noise_metadata_schedule_46_0_e694);
        (noise_metadata_schedule_46_0_e695,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_46_0_e697;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_47_0_e700: f64 = if w[0] > 80.0 { 1.0 } else { 0.0 };
            w[66] = noise_metadata_schedule_47_0_e700;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_48_0_e710,) = {
    if ((w[65] != 0.0) && (w[66] != 0.0)) {
        let noise_metadata_schedule_48_0_e707: f64 = (w[0] - 80.0);
        let noise_metadata_schedule_48_0_e708: f64 = (1.0 + noise_metadata_schedule_48_0_e707);
        (noise_metadata_schedule_48_0_e708,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_48_0_e710;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_49_0_e716,) = {
    if ((w[65] != 0.0) && (w[66] != 0.0)) {
        (80.0,)
    } else {
        (w[0],)
    }
};
            w[0] = noise_metadata_schedule_49_0_e716;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_50_0_e723,) = {
    if ((w[65] != 0.0) && (w[66] == 0.0)) {
        (1.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_50_0_e723;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_51_0_e730,) = {
    if (w[65] != 0.0) {
        let noise_metadata_schedule_51_0_e727: f64 = (w[0]).exp();
        let noise_metadata_schedule_51_0_e728: f64 = (w[1] * noise_metadata_schedule_51_0_e727);
        (noise_metadata_schedule_51_0_e728,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_51_0_e730;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_52_0_e738,) = {
    if (w[65] != 0.0) {
        let noise_metadata_schedule_52_0_e735: f64 = (w[1] - 1.0);
        let noise_metadata_schedule_52_0_e736: f64 = (w[55] * noise_metadata_schedule_52_0_e735);
        (noise_metadata_schedule_52_0_e736,)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_52_0_e738;
        }
        if (active[0] & 0xc) != 0 {
            let (noise_metadata_schedule_53_0_e743,) = {
    if (w[65] == 0.0) {
        (0.0,)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_53_0_e743;
        }
        if (active[0] & 0xc) != 0 {
            let noise_metadata_schedule_54_0_e746: f64 = (w[23] - w[26]);
            w[24] = noise_metadata_schedule_54_0_e746;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_55_0_e750: f64 = (w[41] / params[48]);
            let noise_metadata_schedule_55_0_e751: f64 = (noise_metadata_schedule_55_0_e750).abs();
            let noise_metadata_schedule_55_0_e753: f64 = (noise_metadata_schedule_55_0_e751).powf(params[49]);
            let noise_metadata_schedule_55_0_e754: f64 = (1.0 + noise_metadata_schedule_55_0_e753);
            w[58] = noise_metadata_schedule_55_0_e754;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_56_0_e758: f64 = (w[42] / params[50]);
            let noise_metadata_schedule_56_0_e759: f64 = (noise_metadata_schedule_56_0_e758).abs();
            let noise_metadata_schedule_56_0_e761: f64 = (noise_metadata_schedule_56_0_e759).powf(params[51]);
            let noise_metadata_schedule_56_0_e762: f64 = (1.0 + noise_metadata_schedule_56_0_e761);
            w[59] = noise_metadata_schedule_56_0_e762;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_57_0_e766: f64 = (w[14] * params[37]);
            let noise_metadata_schedule_57_0_e767: f64 = (noise_metadata_schedule_57_0_e766).exp();
            let noise_metadata_schedule_57_0_e768: f64 = (params[12] * noise_metadata_schedule_57_0_e767);
            let noise_metadata_schedule_57_0_e772: f64 = (1.0 / params[49]);
            let noise_metadata_schedule_57_0_e773: f64 = (w[58]).powf(noise_metadata_schedule_57_0_e772);
            let noise_metadata_schedule_57_0_e774: f64 = (noise_metadata_schedule_57_0_e768 * noise_metadata_schedule_57_0_e773);
            w[29] = noise_metadata_schedule_57_0_e774;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_58_0_e778: f64 = (w[14] * params[38]);
            let noise_metadata_schedule_58_0_e779: f64 = (noise_metadata_schedule_58_0_e778).exp();
            let noise_metadata_schedule_58_0_e780: f64 = (params[14] * noise_metadata_schedule_58_0_e779);
            let noise_metadata_schedule_58_0_e784: f64 = (1.0 / params[51]);
            let noise_metadata_schedule_58_0_e785: f64 = (w[59]).powf(noise_metadata_schedule_58_0_e784);
            let noise_metadata_schedule_58_0_e786: f64 = (noise_metadata_schedule_58_0_e780 * noise_metadata_schedule_58_0_e785);
            w[30] = noise_metadata_schedule_58_0_e786;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_59_0_e789: f64 = if params[31] == 1.0 { 1.0 } else { 0.0 };
            w[67] = noise_metadata_schedule_59_0_e789;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_60_0_e795,) = {
    if (w[67] != 0.0) {
        let noise_metadata_schedule_60_0_e793: f64 = (w[29] + params[13]);
        (noise_metadata_schedule_60_0_e793,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_60_0_e795;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_61_0_e801,) = {
    if (w[67] != 0.0) {
        let noise_metadata_schedule_61_0_e799: f64 = (w[30] + params[15]);
        (noise_metadata_schedule_61_0_e799,)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_61_0_e801;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_67_0_e830: f64 = if params[32] == 1.0 { 1.0 } else { 0.0 };
            w[68] = noise_metadata_schedule_67_0_e830;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_68_0_e843,) = {
    if (w[68] != 0.0) {
        let noise_metadata_schedule_68_0_e835: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0)).abs();
        let noise_metadata_schedule_68_0_e837: f64 = (noise_metadata_schedule_68_0_e835 / params[20]);
        let noise_metadata_schedule_68_0_e839: f64 = (noise_metadata_schedule_68_0_e837).powf(params[44]);
        let noise_metadata_schedule_68_0_e840: f64 = (1.0 + noise_metadata_schedule_68_0_e839);
        let noise_metadata_schedule_68_0_e841: f64 = (w[29] / noise_metadata_schedule_68_0_e840);
        (noise_metadata_schedule_68_0_e841,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_68_0_e843;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_69_0_e848,) = {
    if (w[68] == 0.0) {
        (w[29],)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_69_0_e848;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_82_0_e967: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_82_0_e969: f64 = (noise_metadata_schedule_82_0_e967 * w[10]);
            w[35] = noise_metadata_schedule_82_0_e969;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_83_0_e973: f64 = (params[31] * params[13]);
            let noise_metadata_schedule_83_0_e974: f64 = (params[12] + noise_metadata_schedule_83_0_e973);
            let noise_metadata_schedule_83_0_e976: f64 = (noise_metadata_schedule_83_0_e974 / w[3]);
            w[28] = noise_metadata_schedule_83_0_e976;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 75], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_84_0_e980: f64 = (params[31] * params[15]);
            let noise_metadata_schedule_84_0_e981: f64 = (params[14] + noise_metadata_schedule_84_0_e980);
            let noise_metadata_schedule_84_0_e983: f64 = (noise_metadata_schedule_84_0_e981 / w[3]);
            w[27] = noise_metadata_schedule_84_0_e983;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_85_0_e990: f64 = if ((w[28] > 0.0) && (w[28] >= params[46])) { 1.0 } else { 0.0 };
            w[73] = noise_metadata_schedule_85_0_e990;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_86_0_e1005,) = {
    if (w[73] != 0.0) {
        let noise_metadata_schedule_86_0_e994: f64 = (w[29] / w[3]);
        let (noise_metadata_schedule_86_0_e1003,) = {
            if (noise_metadata_schedule_86_0_e994 >= params[46]) {
                let noise_metadata_schedule_86_0_e1000: f64 = (w[29] / w[3]);
                let noise_metadata_schedule_86_0_e1001: f64 = (w[35] / noise_metadata_schedule_86_0_e1000);
                (noise_metadata_schedule_86_0_e1001,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_86_0_e1003,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_86_0_e1005;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_87_0_e1012: f64 = if ((w[27] > 0.0) && (w[27] >= params[46])) { 1.0 } else { 0.0 };
            w[74] = noise_metadata_schedule_87_0_e1012;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_88_0_e1027,) = {
    if (w[74] != 0.0) {
        let noise_metadata_schedule_88_0_e1016: f64 = (w[30] / w[3]);
        let (noise_metadata_schedule_88_0_e1025,) = {
            if (noise_metadata_schedule_88_0_e1016 >= params[46]) {
                let noise_metadata_schedule_88_0_e1022: f64 = (w[30] / w[3]);
                let noise_metadata_schedule_88_0_e1023: f64 = (w[35] / noise_metadata_schedule_88_0_e1022);
                (noise_metadata_schedule_88_0_e1023,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_88_0_e1025,)
    } else {
        (w[39],)
    }
};
            w[39] = noise_metadata_schedule_88_0_e1027;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_89_0_e1034: f64 = if ((params[28] > 0.0) && (params[27] > 0.0)) { 1.0 } else { 0.0 };
            let (noise_metadata_schedule_89_0_e1044,) = {
    if (noise_metadata_schedule_89_0_e1034 > 0.0) {
        let noise_metadata_schedule_89_0_e1039: f64 = (w[24]).abs();
        let noise_metadata_schedule_89_0_e1041: f64 = (noise_metadata_schedule_89_0_e1039).powf(params[28]);
        let noise_metadata_schedule_89_0_e1042: f64 = (params[27] * noise_metadata_schedule_89_0_e1041);
        (noise_metadata_schedule_89_0_e1042,)
    } else {
        (0.0,)
    }
};
            w[37] = noise_metadata_schedule_89_0_e1044;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_90_0_e1047: f64 = (2.0 * 1.6021918e-19);
            w[36] = noise_metadata_schedule_90_0_e1047;
        }
    }
}

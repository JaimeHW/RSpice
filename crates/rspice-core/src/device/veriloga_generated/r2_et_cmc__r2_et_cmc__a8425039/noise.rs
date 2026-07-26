#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_N2_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_N1_N2_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 6, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "n1", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(1), name: "n2", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 102];
        let noise_source_0_active = {
            true
        };
        let noise_source_1_active = {
            true
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1)];
        w.fill(0.0);
        self.noise_metadata_schedule_part_0(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_1(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_2(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e1618: f64 = 1.0;
            let noise_0_psd_e1619: f64 = (noise_0_psd_e1618 * w[29]);
            let psd = noise_0_psd_e1619;
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
            let noise_1_psd_e1621: f64 = 1.0;
            let noise_1_psd_e1622: f64 = (noise_1_psd_e1621 * w[30]);
            let psd = noise_1_psd_e1622;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(params.p32);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 102], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_1_0_e87: f64 = if self.param_given[10] { 1.0 } else { 0.0 };
            w[51] = noise_metadata_schedule_1_0_e87;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_2_0_e91,) = {
    if (w[51] != 0.0) {
        (params.p10,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_2_0_e91;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_3_0_e98,) = {
    if (w[51] == 0.0) {
        let noise_metadata_schedule_3_0_e96: f64 = 1.0;
        (noise_metadata_schedule_3_0_e96,)
    } else {
        (w[13],)
    }
};
            w[13] = noise_metadata_schedule_3_0_e98;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_4_0_e100: f64 = if self.param_given[11] { 1.0 } else { 0.0 };
            w[52] = noise_metadata_schedule_4_0_e100;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_5_0_e108,) = {
    if (w[52] != 0.0) {
        let noise_metadata_schedule_5_0_e105: f64 = (0.01 * params.p11);
        let noise_metadata_schedule_5_0_e106: f64 = (1.0 - noise_metadata_schedule_5_0_e105);
        (noise_metadata_schedule_5_0_e106,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_5_0_e108;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_6_0_e119,) = {
    if (w[52] == 0.0) {
        let noise_metadata_schedule_6_0_e115: f64 = 0.0;
        let noise_metadata_schedule_6_0_e116: f64 = (0.01 * noise_metadata_schedule_6_0_e115);
        let noise_metadata_schedule_6_0_e117: f64 = (1.0 - noise_metadata_schedule_6_0_e116);
        (noise_metadata_schedule_6_0_e117,)
    } else {
        (w[14],)
    }
};
            w[14] = noise_metadata_schedule_6_0_e119;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_10_0_e135: f64 = (w[14] * w[13]);
            let noise_metadata_schedule_10_0_e137: f64 = (noise_metadata_schedule_10_0_e135 * 1000000.0);
            w[18] = noise_metadata_schedule_10_0_e137;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_11_0_e140: f64 = (273.15 + params.p16);
            w[11] = noise_metadata_schedule_11_0_e140;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_12_0_e141: f64 = ctx.temperature();
            let noise_metadata_schedule_12_0_e143: f64 = (noise_metadata_schedule_12_0_e141 + params.p5);
            let noise_metadata_schedule_12_0_e145: f64 = (noise_metadata_schedule_12_0_e143 - 273.15);
            w[28] = noise_metadata_schedule_12_0_e145;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_15_0_e154: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[56] = noise_metadata_schedule_15_0_e154;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_16_0_e158,) = {
    if (w[56] != 0.0) {
        (params.p23,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_16_0_e158;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_17_0_e161: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[57] = noise_metadata_schedule_17_0_e161;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_18_0_e170,) = {
    if ((w[56] == 0.0) && (w[57] != 0.0)) {
        let noise_metadata_schedule_18_0_e168: f64 = (params.p23 * 0.5);
        (noise_metadata_schedule_18_0_e168,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_18_0_e170;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_19_0_e178,) = {
    if ((w[56] == 0.0) && (w[57] == 0.0)) {
        (0.0,)
    } else {
        (w[17],)
    }
};
            w[17] = noise_metadata_schedule_19_0_e178;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_20_0_e187: f64 = if ((self.param_given[1] && self.param_given[2]) && (!self.param_given[0])) { 1.0 } else { 0.0 };
            w[58] = noise_metadata_schedule_20_0_e187;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_21_0_e194: f64 = if ((params.p2 == 0.0) || (params.p1 == 0.0)) { 1.0 } else { 0.0 };
            w[59] = noise_metadata_schedule_21_0_e194;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_22_0_e200,) = {
    if ((w[58] != 0.0) && (w[59] != 0.0)) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_22_0_e200;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_23_0_e206,) = {
    if ((w[58] != 0.0) && (w[59] != 0.0)) {
        (0.0,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_23_0_e206;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_24_0_e214,) = {
    if ((w[58] != 0.0) && (w[59] != 0.0)) {
        let noise_metadata_schedule_24_0_e212: f64 = (params.p0 * w[18]);
        (noise_metadata_schedule_24_0_e212,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_24_0_e214;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_25_0_e222,) = {
    if ((w[58] != 0.0) && (w[59] != 0.0)) {
        let noise_metadata_schedule_25_0_e220: f64 = (w[20] + params.p22);
        (noise_metadata_schedule_25_0_e220,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_25_0_e222;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_26_0_e228,) = {
    if ((w[58] != 0.0) && (w[59] != 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_26_0_e228;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_27_0_e234,) = {
    if ((w[58] != 0.0) && (w[59] != 0.0)) {
        (1e99,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_27_0_e234;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_28_0_e243,) = {
    if ((w[58] != 0.0) && (w[59] == 0.0)) {
        let noise_metadata_schedule_28_0_e241: f64 = (params.p1 * w[18]);
        (noise_metadata_schedule_28_0_e241,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_28_0_e243;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_29_0_e252,) = {
    if ((w[58] != 0.0) && (w[59] == 0.0)) {
        let noise_metadata_schedule_29_0_e250: f64 = (w[19] + w[17]);
        (noise_metadata_schedule_29_0_e250,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_29_0_e252;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_31_0_e258: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[61] = noise_metadata_schedule_31_0_e258;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_32_0_e271,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] != 0.0)) {
        let noise_metadata_schedule_32_0_e267: f64 = (params.p17 / params.p2);
        let noise_metadata_schedule_32_0_e269: f64 = (noise_metadata_schedule_32_0_e267 * w[3]);
        (noise_metadata_schedule_32_0_e269,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_32_0_e271;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_33_0_e282,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] != 0.0)) {
        let noise_metadata_schedule_33_0_e280: f64 = (w[4] - params.p22);
        (noise_metadata_schedule_33_0_e280,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_33_0_e282;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_35_0_e294,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] != 0.0)) {
        (params.p2,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_35_0_e294;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_36_0_e305,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] != 0.0)) {
        let noise_metadata_schedule_36_0_e303: f64 = (1.0 / w[5]);
        (noise_metadata_schedule_36_0_e303,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_36_0_e305;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_37_0_e317,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] == 0.0)) {
        let noise_metadata_schedule_37_0_e315: f64 = (params.p0 * w[18]);
        (noise_metadata_schedule_37_0_e315,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_37_0_e317;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_38_0_e329,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] == 0.0)) {
        let noise_metadata_schedule_38_0_e327: f64 = (w[20] + params.p22);
        (noise_metadata_schedule_38_0_e327,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_38_0_e329;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_39_0_e339,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] == 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_39_0_e339;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_40_0_e349,) = {
    if (((w[58] != 0.0) && (w[59] == 0.0)) && (w[61] == 0.0)) {
        (1e99,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_40_0_e349;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_41_0_e355: f64 = if (self.param_given[2] && (!self.param_given[1])) { 1.0 } else { 0.0 };
            w[63] = noise_metadata_schedule_41_0_e355;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_42_0_e358: f64 = if params.p2 == 0.0 { 1.0 } else { 0.0 };
            w[64] = noise_metadata_schedule_42_0_e358;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_43_0_e367,) = {
    if (((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] != 0.0)) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_43_0_e367;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_44_0_e376,) = {
    if (((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] != 0.0)) {
        (0.0,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_44_0_e376;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_45_0_e387,) = {
    if (((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] != 0.0)) {
        let noise_metadata_schedule_45_0_e385: f64 = (params.p0 * w[18]);
        (noise_metadata_schedule_45_0_e385,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_45_0_e387;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_46_0_e398,) = {
    if (((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] != 0.0)) {
        let noise_metadata_schedule_46_0_e396: f64 = (w[20] + params.p22);
        (noise_metadata_schedule_46_0_e396,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_46_0_e398;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_47_0_e407,) = {
    if (((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] != 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_47_0_e407;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_48_0_e416,) = {
    if (((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] != 0.0)) {
        (1e99,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_48_0_e416;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_49_0_e419: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            w[65] = noise_metadata_schedule_49_0_e419;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_50_0_e431,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] != 0.0)) {
        (0.0,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_50_0_e431;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_51_0_e443,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] != 0.0)) {
        (0.0,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_51_0_e443;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_52_0_e457,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] != 0.0)) {
        let noise_metadata_schedule_52_0_e455: f64 = (params.p1 * w[18]);
        (noise_metadata_schedule_52_0_e455,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_52_0_e457;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_53_0_e471,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] != 0.0)) {
        let noise_metadata_schedule_53_0_e469: f64 = (w[19] + w[17]);
        (noise_metadata_schedule_53_0_e469,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_53_0_e471;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_54_0_e483,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] != 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_54_0_e483;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_55_0_e495,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] != 0.0)) {
        (0.0,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_55_0_e495;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_56_0_e510,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) {
        let noise_metadata_schedule_56_0_e508: f64 = (params.p0 * w[18]);
        (noise_metadata_schedule_56_0_e508,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_56_0_e510;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_57_0_e525,) = {
    if ((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) {
        let noise_metadata_schedule_57_0_e523: f64 = (w[20] + params.p22);
        (noise_metadata_schedule_57_0_e523,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_57_0_e525;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_59_0_e531: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[67] = noise_metadata_schedule_59_0_e531;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_60_0_e550,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] != 0.0)) {
        let noise_metadata_schedule_60_0_e546: f64 = (params.p2 / params.p17);
        let noise_metadata_schedule_60_0_e548: f64 = (noise_metadata_schedule_60_0_e546 * w[4]);
        (noise_metadata_schedule_60_0_e548,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_60_0_e550;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_61_0_e567,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] != 0.0)) {
        let noise_metadata_schedule_61_0_e565: f64 = (w[3] - w[17]);
        (noise_metadata_schedule_61_0_e565,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_61_0_e567;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_63_0_e585,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] != 0.0)) {
        (params.p2,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_63_0_e585;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_64_0_e602,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] != 0.0)) {
        let noise_metadata_schedule_64_0_e600: f64 = (1.0 / w[5]);
        (noise_metadata_schedule_64_0_e600,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_64_0_e602;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_65_0_e620,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] == 0.0)) {
        let noise_metadata_schedule_65_0_e618: f64 = (params.p1 * w[18]);
        (noise_metadata_schedule_65_0_e618,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_65_0_e620;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 102], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_66_0_e638,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] == 0.0)) {
        let noise_metadata_schedule_66_0_e636: f64 = (w[19] + w[17]);
        (noise_metadata_schedule_66_0_e636,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_66_0_e638;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_67_0_e654,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] == 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_67_0_e654;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_68_0_e670,) = {
    if (((((w[58] == 0.0) && (w[63] != 0.0)) && (w[64] == 0.0)) && (w[65] == 0.0)) && (w[67] == 0.0)) {
        (0.0,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_68_0_e670;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_69_0_e673: f64 = if params.p0 == 0.0 { 1.0 } else { 0.0 };
            w[69] = noise_metadata_schedule_69_0_e673;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_70_0_e683,) = {
    if (((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] != 0.0)) {
        (0.0,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_70_0_e683;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_71_0_e693,) = {
    if (((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] != 0.0)) {
        (0.0,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_71_0_e693;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_72_0_e705,) = {
    if (((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] != 0.0)) {
        let noise_metadata_schedule_72_0_e703: f64 = (params.p1 * w[18]);
        (noise_metadata_schedule_72_0_e703,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_72_0_e705;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_73_0_e717,) = {
    if (((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] != 0.0)) {
        let noise_metadata_schedule_73_0_e715: f64 = (w[19] + w[17]);
        (noise_metadata_schedule_73_0_e715,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_73_0_e717;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_74_0_e727,) = {
    if (((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] != 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_74_0_e727;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_75_0_e737,) = {
    if (((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] != 0.0)) {
        (0.0,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_75_0_e737;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_76_0_e740: f64 = if params.p1 == 0.0 { 1.0 } else { 0.0 };
            w[70] = noise_metadata_schedule_76_0_e740;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_77_0_e753,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] != 0.0)) {
        (0.0,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_77_0_e753;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_78_0_e766,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] != 0.0)) {
        (0.0,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_78_0_e766;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_79_0_e781,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] != 0.0)) {
        let noise_metadata_schedule_79_0_e779: f64 = (params.p0 * w[18]);
        (noise_metadata_schedule_79_0_e779,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_79_0_e781;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_80_0_e796,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] != 0.0)) {
        let noise_metadata_schedule_80_0_e794: f64 = (w[20] + params.p22);
        (noise_metadata_schedule_80_0_e794,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_80_0_e796;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_81_0_e809,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] != 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_81_0_e809;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_82_0_e822,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] != 0.0)) {
        (1e99,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_82_0_e822;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_83_0_e838,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) {
        let noise_metadata_schedule_83_0_e836: f64 = (params.p0 * w[18]);
        (noise_metadata_schedule_83_0_e836,)
    } else {
        (w[20],)
    }
};
            w[20] = noise_metadata_schedule_83_0_e838;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_84_0_e854,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) {
        let noise_metadata_schedule_84_0_e852: f64 = (w[20] + params.p22);
        (noise_metadata_schedule_84_0_e852,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_84_0_e854;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_86_0_e873,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) {
        let noise_metadata_schedule_86_0_e871: f64 = (params.p1 * w[18]);
        (noise_metadata_schedule_86_0_e871,)
    } else {
        (w[19],)
    }
};
            w[19] = noise_metadata_schedule_86_0_e873;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_87_0_e889,) = {
    if ((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) {
        let noise_metadata_schedule_87_0_e887: f64 = (w[19] + w[17]);
        (noise_metadata_schedule_87_0_e887,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_87_0_e889;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_88_0_e892: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[72] = noise_metadata_schedule_88_0_e892;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_90_0_e898: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[74] = noise_metadata_schedule_90_0_e898;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_91_0_e920,) = {
    if ((((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) && (w[72] != 0.0)) && (w[74] != 0.0)) {
        let noise_metadata_schedule_91_0_e917: f64 = (w[3] / w[4]);
        let noise_metadata_schedule_91_0_e918: f64 = (params.p17 * noise_metadata_schedule_91_0_e917);
        (noise_metadata_schedule_91_0_e918,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_91_0_e920;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_92_0_e940,) = {
    if ((((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) && (w[72] != 0.0)) && (w[74] != 0.0)) {
        let noise_metadata_schedule_92_0_e938: f64 = (1.0 / w[5]);
        (noise_metadata_schedule_92_0_e938,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_92_0_e940;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_93_0_e959,) = {
    if ((((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) && (w[72] != 0.0)) && (w[74] == 0.0)) {
        (0.0,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_93_0_e959;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_94_0_e978,) = {
    if ((((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) && (w[72] != 0.0)) && (w[74] == 0.0)) {
        (1e99,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_94_0_e978;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_95_0_e995,) = {
    if (((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) && (w[72] == 0.0)) {
        (1e99,)
    } else {
        (w[5],)
    }
};
            w[5] = noise_metadata_schedule_95_0_e995;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_96_0_e1012,) = {
    if (((((w[58] == 0.0) && (w[63] == 0.0)) && (w[69] == 0.0)) && (w[70] == 0.0)) && (w[72] == 0.0)) {
        (0.0,)
    } else {
        (w[22],)
    }
};
            w[22] = noise_metadata_schedule_96_0_e1012;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_101_0_e1030,) = {
    if (params.p25 != 0.0) {
        let noise_metadata_schedule_101_0_e1028: f64 = (w[3] + params.p24);
        (noise_metadata_schedule_101_0_e1028,)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_101_0_e1030;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_102_0_e1037,) = {
    if (params.p25 == 0.0) {
        let noise_metadata_schedule_102_0_e1035: f64 = (w[19] + params.p24);
        (noise_metadata_schedule_102_0_e1035,)
    } else {
        (w[21],)
    }
};
            w[21] = noise_metadata_schedule_102_0_e1037;
        }
        if (active[0] & 0x3) != 0 {
            w[37] = params.p37;
        }
        if (active[0] & 0x3) != 0 {
            w[38] = params.p38;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_106_0_e1057: f64 = if w[3] > 0.0 { 1.0 } else { 0.0 };
            w[80] = noise_metadata_schedule_106_0_e1057;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_107_0_e1060: f64 = if ((params.p3 != 0.0) && (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[81] = noise_metadata_schedule_107_0_e1060;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_108_0_e1070,) = {
    if ((w[80] != 0.0) && (w[81] != 0.0)) {
        let noise_metadata_schedule_108_0_e1067: f64 = (params.p39 / w[3]);
        let noise_metadata_schedule_108_0_e1068: f64 = (w[37] + noise_metadata_schedule_108_0_e1067);
        (noise_metadata_schedule_108_0_e1068,)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_108_0_e1070;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_109_0_e1080,) = {
    if ((w[80] != 0.0) && (w[81] != 0.0)) {
        let noise_metadata_schedule_109_0_e1077: f64 = (params.p40 / w[3]);
        let noise_metadata_schedule_109_0_e1078: f64 = (w[38] + noise_metadata_schedule_109_0_e1077);
        (noise_metadata_schedule_109_0_e1078,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_109_0_e1080;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_110_0_e1083: f64 = if ((params.p3 != 0.0) || (params.p4 != 0.0)) { 1.0 } else { 0.0 };
            w[82] = noise_metadata_schedule_110_0_e1083;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_111_0_e1098,) = {
    if (((w[80] != 0.0) && (w[81] == 0.0)) && (w[82] != 0.0)) {
        let noise_metadata_schedule_111_0_e1093: f64 = (0.5 * params.p39);
        let noise_metadata_schedule_111_0_e1095: f64 = (noise_metadata_schedule_111_0_e1093 / w[3]);
        let noise_metadata_schedule_111_0_e1096: f64 = (w[37] + noise_metadata_schedule_111_0_e1095);
        (noise_metadata_schedule_111_0_e1096,)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_111_0_e1098;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_112_0_e1113,) = {
    if (((w[80] != 0.0) && (w[81] == 0.0)) && (w[82] != 0.0)) {
        let noise_metadata_schedule_112_0_e1108: f64 = (0.5 * params.p40);
        let noise_metadata_schedule_112_0_e1110: f64 = (noise_metadata_schedule_112_0_e1108 / w[3]);
        let noise_metadata_schedule_112_0_e1111: f64 = (w[38] + noise_metadata_schedule_112_0_e1110);
        (noise_metadata_schedule_112_0_e1111,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_112_0_e1113;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_113_0_e1116: f64 = if w[4] > 0.0 { 1.0 } else { 0.0 };
            w[83] = noise_metadata_schedule_113_0_e1116;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_114_0_e1124,) = {
    if (w[83] != 0.0) {
        let noise_metadata_schedule_114_0_e1121: f64 = (params.p41 / w[4]);
        let noise_metadata_schedule_114_0_e1122: f64 = (w[37] + noise_metadata_schedule_114_0_e1121);
        (noise_metadata_schedule_114_0_e1122,)
    } else {
        (w[37],)
    }
};
            w[37] = noise_metadata_schedule_114_0_e1124;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_115_0_e1132,) = {
    if (w[83] != 0.0) {
        let noise_metadata_schedule_115_0_e1129: f64 = (params.p42 / w[4]);
        let noise_metadata_schedule_115_0_e1130: f64 = (w[38] + noise_metadata_schedule_115_0_e1129);
        (noise_metadata_schedule_115_0_e1130,)
    } else {
        (w[38],)
    }
};
            w[38] = noise_metadata_schedule_115_0_e1132;
        }
        if (active[0] & 0x3) != 0 {
            w[42] = (ctx.node_voltage(self.nodes[2]) - 0.0);
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_129_0_e1209: f64 = (params.p7 * w[42]);
            let noise_metadata_schedule_129_0_e1210: f64 = (w[28] + noise_metadata_schedule_129_0_e1209);
            w[28] = noise_metadata_schedule_129_0_e1210;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_130_0_e1214: f64 = (params.p35 + 1.0);
            let noise_metadata_schedule_130_0_e1215: f64 = if w[28] < noise_metadata_schedule_130_0_e1214 { 1.0 } else { 0.0 };
            w[88] = noise_metadata_schedule_130_0_e1215;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_131_0_e1226,) = {
    if (w[88] != 0.0) {
        let noise_metadata_schedule_131_0_e1220: f64 = (w[28] - params.p35);
        let noise_metadata_schedule_131_0_e1222: f64 = (noise_metadata_schedule_131_0_e1220 - 1.0);
        let noise_metadata_schedule_131_0_e1223: f64 = (noise_metadata_schedule_131_0_e1222).exp();
        let noise_metadata_schedule_131_0_e1224: f64 = (params.p35 + noise_metadata_schedule_131_0_e1223);
        (noise_metadata_schedule_131_0_e1224,)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_131_0_e1226;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_132_0_e1230: f64 = (params.p36 - 1.0);
            let noise_metadata_schedule_132_0_e1231: f64 = if w[28] > noise_metadata_schedule_132_0_e1230 { 1.0 } else { 0.0 };
            w[89] = noise_metadata_schedule_132_0_e1231;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_133_0_e1245,) = {
    if ((w[88] == 0.0) && (w[89] != 0.0)) {
        let noise_metadata_schedule_133_0_e1239: f64 = (params.p36 - w[28]);
        let noise_metadata_schedule_133_0_e1241: f64 = (noise_metadata_schedule_133_0_e1239 - 1.0);
        let noise_metadata_schedule_133_0_e1242: f64 = (noise_metadata_schedule_133_0_e1241).exp();
        let noise_metadata_schedule_133_0_e1243: f64 = (params.p36 - noise_metadata_schedule_133_0_e1242);
        (noise_metadata_schedule_133_0_e1243,)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_133_0_e1245;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_134_0_e1253,) = {
    if ((w[88] == 0.0) && (w[89] == 0.0)) {
        (w[28],)
    } else {
        (w[28],)
    }
};
            w[28] = noise_metadata_schedule_134_0_e1253;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_135_0_e1256: f64 = (w[28] + 273.15);
            w[12] = noise_metadata_schedule_135_0_e1256;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_136_0_e1259: f64 = (w[12] - w[11]);
            w[15] = noise_metadata_schedule_136_0_e1259;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_137_0_e1265: f64 = (w[15] * w[38]);
            let noise_metadata_schedule_137_0_e1266: f64 = (w[37] + noise_metadata_schedule_137_0_e1265);
            let noise_metadata_schedule_137_0_e1267: f64 = (w[15] * noise_metadata_schedule_137_0_e1266);
            let noise_metadata_schedule_137_0_e1268: f64 = (1.0 + noise_metadata_schedule_137_0_e1267);
            w[16] = noise_metadata_schedule_137_0_e1268;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_138_0_e1272: f64 = (0.01 + 0.1);
            let noise_metadata_schedule_138_0_e1273: f64 = if w[16] < noise_metadata_schedule_138_0_e1272 { 1.0 } else { 0.0 };
            w[90] = noise_metadata_schedule_138_0_e1273;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_139_0_e1288,) = {
    if (w[90] != 0.0) {
        let noise_metadata_schedule_139_0_e1280: f64 = (w[16] - 0.01);
        let noise_metadata_schedule_139_0_e1281: f64 = (10.0 * noise_metadata_schedule_139_0_e1280);
        let noise_metadata_schedule_139_0_e1283: f64 = (noise_metadata_schedule_139_0_e1281 - 1.0);
        let noise_metadata_schedule_139_0_e1284: f64 = (noise_metadata_schedule_139_0_e1283).exp();
        let noise_metadata_schedule_139_0_e1285: f64 = (0.1 * noise_metadata_schedule_139_0_e1284);
        let noise_metadata_schedule_139_0_e1286: f64 = (0.01 + noise_metadata_schedule_139_0_e1285);
        (noise_metadata_schedule_139_0_e1286,)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_139_0_e1288;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_140_0_e1293,) = {
    if (w[90] == 0.0) {
        (w[16],)
    } else {
        (w[16],)
    }
};
            w[16] = noise_metadata_schedule_140_0_e1293;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_141_0_e1296: f64 = (w[5] * w[16]);
            w[23] = noise_metadata_schedule_141_0_e1296;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_142_0_e1299: f64 = (w[22] / w[16]);
            w[24] = noise_metadata_schedule_142_0_e1299;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_143_0_e1303: f64 = (w[15] * params.p43);
            let noise_metadata_schedule_143_0_e1304: f64 = (1.0 + noise_metadata_schedule_143_0_e1303);
            let noise_metadata_schedule_143_0_e1306: f64 = (noise_metadata_schedule_143_0_e1304 * params.p30);
            w[25] = noise_metadata_schedule_143_0_e1306;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 102], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_144_0_e1309: f64 = if w[25] < 0.0 { 1.0 } else { 0.0 };
            w[91] = noise_metadata_schedule_144_0_e1309;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_145_0_e1313,) = {
    if (w[91] != 0.0) {
        (0.0,)
    } else {
        (w[25],)
    }
};
            w[25] = noise_metadata_schedule_145_0_e1313;
        }
        if (active[0] & 0x3) != 0 {
            w[33] = (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[1]));
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_147_0_e1325: f64 = if ((w[5] > 0.0) && ((params.p29 > 0.0) || (params.p27 > 0.0))) { 1.0 } else { 0.0 };
            w[92] = noise_metadata_schedule_147_0_e1325;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_148_0_e1331,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_148_0_e1329: f64 = (w[33] / w[21]);
        (noise_metadata_schedule_148_0_e1329,)
    } else {
        (w[34],)
    }
};
            w[34] = noise_metadata_schedule_148_0_e1331;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_149_0_e1337,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_149_0_e1335: f64 = (params.p28 * w[34]);
        (noise_metadata_schedule_149_0_e1335,)
    } else {
        (w[35],)
    }
};
            w[35] = noise_metadata_schedule_149_0_e1337;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_150_0_e1346,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_150_0_e1342: f64 = (w[35] * w[35]);
        let noise_metadata_schedule_150_0_e1343: f64 = (1.0 + noise_metadata_schedule_150_0_e1342);
        let noise_metadata_schedule_150_0_e1344: f64 = (noise_metadata_schedule_150_0_e1343).sqrt();
        (noise_metadata_schedule_150_0_e1344,)
    } else {
        (w[26],)
    }
};
            w[26] = noise_metadata_schedule_150_0_e1346;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_151_0_e1353,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_151_0_e1350: f64 = (w[34]).abs();
        let noise_metadata_schedule_151_0_e1351: f64 = (params.p26 * noise_metadata_schedule_151_0_e1350);
        (noise_metadata_schedule_151_0_e1351,)
    } else {
        (w[36],)
    }
};
            w[36] = noise_metadata_schedule_151_0_e1353;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_152_0_e1365,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_152_0_e1358: f64 = (w[36] * w[36]);
        let noise_metadata_schedule_152_0_e1360: f64 = (noise_metadata_schedule_152_0_e1358 * w[36]);
        let noise_metadata_schedule_152_0_e1361: f64 = (1.0 + noise_metadata_schedule_152_0_e1360);
        let noise_metadata_schedule_152_0_e1363: f64 = (noise_metadata_schedule_152_0_e1361).powf(0.3333333333333333);
        (noise_metadata_schedule_152_0_e1363,)
    } else {
        (w[27],)
    }
};
            w[27] = noise_metadata_schedule_152_0_e1365;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_153_0_e1381,) = {
    if (w[92] != 0.0) {
        let noise_metadata_schedule_153_0_e1369: f64 = (1.0 - params.p29);
        let noise_metadata_schedule_153_0_e1371: f64 = (noise_metadata_schedule_153_0_e1369 - params.p27);
        let noise_metadata_schedule_153_0_e1374: f64 = (params.p29 * w[26]);
        let noise_metadata_schedule_153_0_e1375: f64 = (noise_metadata_schedule_153_0_e1371 + noise_metadata_schedule_153_0_e1374);
        let noise_metadata_schedule_153_0_e1378: f64 = (params.p27 * w[27]);
        let noise_metadata_schedule_153_0_e1379: f64 = (noise_metadata_schedule_153_0_e1375 + noise_metadata_schedule_153_0_e1378);
        (noise_metadata_schedule_153_0_e1379,)
    } else {
        (w[32],)
    }
};
            w[32] = noise_metadata_schedule_153_0_e1381;
        }
        if (active[0] & 0x3) != 0 {
            let (noise_metadata_schedule_154_0_e1386,) = {
    if (w[92] == 0.0) {
        (1.0,)
    } else {
        (w[32],)
    }
};
            w[32] = noise_metadata_schedule_154_0_e1386;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_155_0_e1389: f64 = (w[23] * w[32]);
            w[6] = noise_metadata_schedule_155_0_e1389;
        }
        if (active[0] & 0x2) != 0 {
            w[0] = w[33];
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_157_0_e1393: f64 = (w[0] / w[6]);
            w[1] = noise_metadata_schedule_157_0_e1393;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_163_0_e1421: f64 = if (((params.p6 != 0.0) && (w[5] > 0.0)) && (w[22] > 0.0)) { 1.0 } else { 0.0 };
            w[95] = noise_metadata_schedule_163_0_e1421;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_164_0_e1433,) = {
    if (w[95] != 0.0) {
        let noise_metadata_schedule_164_0_e1425: f64 = (4.0 * 1.3806505e-23);
        let noise_metadata_schedule_164_0_e1427: f64 = (noise_metadata_schedule_164_0_e1425 * w[12]);
        let noise_metadata_schedule_164_0_e1429: f64 = (noise_metadata_schedule_164_0_e1427 * w[24]);
        let noise_metadata_schedule_164_0_e1431: f64 = (noise_metadata_schedule_164_0_e1429 / w[32]);
        (noise_metadata_schedule_164_0_e1431,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_164_0_e1433;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_165_0_e1442: f64 = if (((params.p33 != 0.0) && (w[3] > 0.0)) && (w[4] > 0.0)) { 1.0 } else { 0.0 };
            w[96] = noise_metadata_schedule_165_0_e1442;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_166_0_e1459,) = {
    if ((w[95] != 0.0) && (w[96] != 0.0)) {
        let noise_metadata_schedule_166_0_e1449: f64 = (w[1] / w[4]);
        let noise_metadata_schedule_166_0_e1450: f64 = (noise_metadata_schedule_166_0_e1449).abs();
        let noise_metadata_schedule_166_0_e1452: f64 = (noise_metadata_schedule_166_0_e1450).powf(params.p31);
        let noise_metadata_schedule_166_0_e1453: f64 = (w[25] * noise_metadata_schedule_166_0_e1452);
        let noise_metadata_schedule_166_0_e1455: f64 = (noise_metadata_schedule_166_0_e1453 * w[4]);
        let noise_metadata_schedule_166_0_e1457: f64 = (noise_metadata_schedule_166_0_e1455 / w[3]);
        (noise_metadata_schedule_166_0_e1457,)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_166_0_e1459;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_167_0_e1466: f64 = if ((w[19] > 0.0) && (w[20] > 0.0)) { 1.0 } else { 0.0 };
            w[97] = noise_metadata_schedule_167_0_e1466;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_168_0_e1486,) = {
    if (((w[95] != 0.0) && (w[96] == 0.0)) && (w[97] != 0.0)) {
        let noise_metadata_schedule_168_0_e1476: f64 = (w[1] / w[20]);
        let noise_metadata_schedule_168_0_e1477: f64 = (noise_metadata_schedule_168_0_e1476).abs();
        let noise_metadata_schedule_168_0_e1479: f64 = (noise_metadata_schedule_168_0_e1477).powf(params.p31);
        let noise_metadata_schedule_168_0_e1480: f64 = (w[25] * noise_metadata_schedule_168_0_e1479);
        let noise_metadata_schedule_168_0_e1482: f64 = (noise_metadata_schedule_168_0_e1480 * w[20]);
        let noise_metadata_schedule_168_0_e1484: f64 = (noise_metadata_schedule_168_0_e1482 / w[19]);
        (noise_metadata_schedule_168_0_e1484,)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_168_0_e1486;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_169_0_e1496,) = {
    if (((w[95] != 0.0) && (w[96] == 0.0)) && (w[97] == 0.0)) {
        (0.0,)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_169_0_e1496;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_170_0_e1499: f64 = if w[1] < 0.0 { 1.0 } else { 0.0 };
            w[98] = noise_metadata_schedule_170_0_e1499;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_171_0_e1506,) = {
    if ((w[95] != 0.0) && (w[98] != 0.0)) {
        let noise_metadata_schedule_171_0_e1504: f64 = (-w[30]);
        (noise_metadata_schedule_171_0_e1504,)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_171_0_e1506;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_172_0_e1511,) = {
    if (w[95] == 0.0) {
        (0.0,)
    } else {
        (w[29],)
    }
};
            w[29] = noise_metadata_schedule_172_0_e1511;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_173_0_e1516,) = {
    if (w[95] == 0.0) {
        (0.0,)
    } else {
        (w[30],)
    }
};
            w[30] = noise_metadata_schedule_173_0_e1516;
        }
    }
}

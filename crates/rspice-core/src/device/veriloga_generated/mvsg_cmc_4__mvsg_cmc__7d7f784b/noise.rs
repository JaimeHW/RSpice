#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_SI_G_S_SHOT_INT", label: Some("g-s shot int"), kind: GeneratedNoiseKind::White, equation: 178, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_DI_G_D_SHOT_INT", label: Some("g-d shot int"), kind: GeneratedNoiseKind::White, equation: 179, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_FPS4_G_S_SHOT_EXT", label: Some("g-s shot ext"), kind: GeneratedNoiseKind::White, equation: 180, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "fps4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_FP4_G_D_SHOT_EXT", label: Some("g-d shot ext"), kind: GeneratedNoiseKind::White, equation: 181, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(17), name: "fp4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 182, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_CHANNEL", label: Some("channel"), kind: GeneratedNoiseKind::White, equation: 183, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_FPS1_RFPS1", label: Some("rfps1"), kind: GeneratedNoiseKind::White, equation: 184, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "fps1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS1_FPS2_RFPS2", label: Some("rfps2"), kind: GeneratedNoiseKind::White, equation: 185, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "fps1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "fps2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS2_FPS3_RFPS3", label: Some("rfps3"), kind: GeneratedNoiseKind::White, equation: 186, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "fps2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "fps3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS3_FPS4_RFPS4", label: Some("rfps4"), kind: GeneratedNoiseKind::White, equation: 187, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "fps3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "fps4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP1_DI_RFP1", label: Some("rfp1"), kind: GeneratedNoiseKind::White, equation: 188, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "fp1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP2_FP1_RFP2", label: Some("rfp2"), kind: GeneratedNoiseKind::White, equation: 189, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "fp2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(14), name: "fp1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP3_FP2_RFP3", label: Some("rfp3"), kind: GeneratedNoiseKind::White, equation: 190, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(16), name: "fp3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(15), name: "fp2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP4_FP3_RFP4", label: Some("rfp4"), kind: GeneratedNoiseKind::White, equation: 191, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(17), name: "fp4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(16), name: "fp3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SRC_S_RCS", label: Some("rcs"), kind: GeneratedNoiseKind::White, equation: 192, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(19), name: "src", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DRC_RCD", label: Some("rcd"), kind: GeneratedNoiseKind::White, equation: 193, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(18), name: "drc", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut w = [0.0; 2701];
        self.noise_activation_schedule_part_0(ctx, &mut w);
        let noise_source_0_active = {
            w[2686] != 0.0
        };
        let noise_source_1_active = {
            w[2686] != 0.0
        };
        let noise_source_2_active = {
            w[2686] != 0.0
        };
        let noise_source_3_active = {
            w[2686] != 0.0
        };
        let noise_source_4_active = {
            w[2686] != 0.0
        };
        let noise_source_5_active = {
            w[2686] != 0.0
        };
        let noise_source_6_active = {
            let noise_6_activation_e1975: f64 = if ((w[2686] != 0.0) && (w[2688] != 0.0)) { 1.0 } else { 0.0 };
            noise_6_activation_e1975 != 0.0
        };
        let noise_source_7_active = {
            let noise_7_activation_e1995: f64 = if ((w[2686] != 0.0) && (w[2689] != 0.0)) { 1.0 } else { 0.0 };
            noise_7_activation_e1995 != 0.0
        };
        let noise_source_8_active = {
            let noise_8_activation_e2015: f64 = if ((w[2686] != 0.0) && (w[2690] != 0.0)) { 1.0 } else { 0.0 };
            noise_8_activation_e2015 != 0.0
        };
        let noise_source_9_active = {
            let noise_9_activation_e2035: f64 = if ((w[2686] != 0.0) && (w[2691] != 0.0)) { 1.0 } else { 0.0 };
            noise_9_activation_e2035 != 0.0
        };
        let noise_source_10_active = {
            let noise_10_activation_e2055: f64 = if ((w[2686] != 0.0) && (w[2692] != 0.0)) { 1.0 } else { 0.0 };
            noise_10_activation_e2055 != 0.0
        };
        let noise_source_11_active = {
            let noise_11_activation_e2075: f64 = if ((w[2686] != 0.0) && (w[2693] != 0.0)) { 1.0 } else { 0.0 };
            noise_11_activation_e2075 != 0.0
        };
        let noise_source_12_active = {
            let noise_12_activation_e2095: f64 = if ((w[2686] != 0.0) && (w[2694] != 0.0)) { 1.0 } else { 0.0 };
            noise_12_activation_e2095 != 0.0
        };
        let noise_source_13_active = {
            let noise_13_activation_e2115: f64 = if ((w[2686] != 0.0) && (w[2695] != 0.0)) { 1.0 } else { 0.0 };
            noise_13_activation_e2115 != 0.0
        };
        let noise_source_14_active = {
            let noise_14_activation_e2135: f64 = if ((w[2686] != 0.0) && (w[2696] != 0.0)) { 1.0 } else { 0.0 };
            noise_14_activation_e2135 != 0.0
        };
        let noise_source_15_active = {
            let noise_15_activation_e2149: f64 = if ((w[2686] != 0.0) && (w[2697] != 0.0)) { 1.0 } else { 0.0 };
            noise_15_activation_e2149 != 0.0
        };
        let noise_source_active = [noise_source_0_active, noise_source_1_active, noise_source_2_active, noise_source_3_active, noise_source_4_active, noise_source_5_active, noise_source_6_active, noise_source_7_active, noise_source_8_active, noise_source_9_active, noise_source_10_active, noise_source_11_active, noise_source_12_active, noise_source_13_active, noise_source_14_active, noise_source_15_active];
        let noise_source_active_mask = [(noise_source_0_active as u128) | ((noise_source_1_active as u128) << 1) | ((noise_source_2_active as u128) << 2) | ((noise_source_3_active as u128) << 3) | ((noise_source_4_active as u128) << 4) | ((noise_source_5_active as u128) << 5) | ((noise_source_6_active as u128) << 6) | ((noise_source_7_active as u128) << 7) | ((noise_source_8_active as u128) << 8) | ((noise_source_9_active as u128) << 9) | ((noise_source_10_active as u128) << 10) | ((noise_source_11_active as u128) << 11) | ((noise_source_12_active as u128) << 12) | ((noise_source_13_active as u128) << 13) | ((noise_source_14_active as u128) << 14) | ((noise_source_15_active as u128) << 15)];
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
        self.noise_metadata_schedule_part_12(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_13(ctx, &mut w, &noise_source_active_mask);
        self.noise_metadata_schedule_part_14(ctx, &mut w, &noise_source_active_mask);
        if !noise_source_active[0] {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_0_psd_e45623: f64 = 1.0;
            let noise_0_psd_e1895: f64 = (params.p348 * 1.60219e-19);
            let noise_0_psd_e1900: f64 = (w[130] + w[132]);
            let noise_0_psd_e1901: f64 = (2.0 * noise_0_psd_e1900);
            let noise_0_psd_e1902: f64 = (w[128] + noise_0_psd_e1901);
            let noise_0_psd_e1903: f64 = (noise_0_psd_e1902).abs();
            let noise_0_psd_e1904: f64 = (noise_0_psd_e1895 * noise_0_psd_e1903);
            let noise_0_psd_e45624: f64 = (noise_0_psd_e45623 * noise_0_psd_e1904);
            let psd = noise_0_psd_e45624;
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
            let noise_1_psd_e45626: f64 = 1.0;
            let noise_1_psd_e1912: f64 = (params.p349 * 1.60219e-19);
            let noise_1_psd_e1917: f64 = (w[131] + w[133]);
            let noise_1_psd_e1918: f64 = (2.0 * noise_1_psd_e1917);
            let noise_1_psd_e1919: f64 = (w[129] + noise_1_psd_e1918);
            let noise_1_psd_e1920: f64 = (noise_1_psd_e1919).abs();
            let noise_1_psd_e1921: f64 = (noise_1_psd_e1912 * noise_1_psd_e1920);
            let noise_1_psd_e45627: f64 = (noise_1_psd_e45626 * noise_1_psd_e1921);
            let psd = noise_1_psd_e45627;
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
            let noise_2_psd_e45629: f64 = 1.0;
            let noise_2_psd_e1929: f64 = (params.p348 * 1.60219e-19);
            let noise_2_psd_e1934: f64 = (w[124] + w[126]);
            let noise_2_psd_e1935: f64 = (2.0 * noise_2_psd_e1934);
            let noise_2_psd_e1936: f64 = (w[122] + noise_2_psd_e1935);
            let noise_2_psd_e1937: f64 = (noise_2_psd_e1936).abs();
            let noise_2_psd_e1938: f64 = (noise_2_psd_e1929 * noise_2_psd_e1937);
            let noise_2_psd_e45630: f64 = (noise_2_psd_e45629 * noise_2_psd_e1938);
            let psd = noise_2_psd_e45630;
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
            let noise_3_psd_e45632: f64 = 1.0;
            let noise_3_psd_e1946: f64 = (params.p349 * 1.60219e-19);
            let noise_3_psd_e1951: f64 = (w[125] + w[127]);
            let noise_3_psd_e1952: f64 = (2.0 * noise_3_psd_e1951);
            let noise_3_psd_e1953: f64 = (w[123] + noise_3_psd_e1952);
            let noise_3_psd_e1954: f64 = (noise_3_psd_e1953).abs();
            let noise_3_psd_e1955: f64 = (noise_3_psd_e1946 * noise_3_psd_e1954);
            let noise_3_psd_e45633: f64 = (noise_3_psd_e45632 * noise_3_psd_e1955);
            let psd = noise_3_psd_e45633;
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
            let noise_4_psd_e45635: f64 = 1.0;
            let noise_4_psd_e45636: f64 = (noise_4_psd_e45635 * w[233]);
            let psd = noise_4_psd_e45636;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = Some(params.p352);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[5] {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_5_psd_e45638: f64 = 1.0;
            let noise_5_psd_e45639: f64 = (noise_5_psd_e45638 * w[232]);
            let psd = noise_5_psd_e45639;
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
            let noise_6_psd_e45641: f64 = 1.0;
            let noise_6_psd_e1978: f64 = (4.0 * 1.38062e-23);
            let noise_6_psd_e1980: f64 = (noise_6_psd_e1978 * w[111]);
            let noise_6_psd_e1983: f64 = (params.p29 * params.p79);
            let noise_6_psd_e1986: f64 = (params.p0 * params.p2);
            let noise_6_psd_e1987: f64 = (noise_6_psd_e1983 / noise_6_psd_e1986);
            let noise_6_psd_e1988: f64 = (noise_6_psd_e1980 / noise_6_psd_e1987);
            let noise_6_psd_e45642: f64 = (noise_6_psd_e45641 * noise_6_psd_e1988);
            let psd = noise_6_psd_e45642;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[7] {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_7_psd_e45644: f64 = 1.0;
            let noise_7_psd_e1998: f64 = (4.0 * 1.38062e-23);
            let noise_7_psd_e2000: f64 = (noise_7_psd_e1998 * w[111]);
            let noise_7_psd_e2003: f64 = (params.p29 * params.p101);
            let noise_7_psd_e2006: f64 = (params.p0 * params.p2);
            let noise_7_psd_e2007: f64 = (noise_7_psd_e2003 / noise_7_psd_e2006);
            let noise_7_psd_e2008: f64 = (noise_7_psd_e2000 / noise_7_psd_e2007);
            let noise_7_psd_e45645: f64 = (noise_7_psd_e45644 * noise_7_psd_e2008);
            let psd = noise_7_psd_e45645;
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
            let noise_8_psd_e45647: f64 = 1.0;
            let noise_8_psd_e2018: f64 = (4.0 * 1.38062e-23);
            let noise_8_psd_e2020: f64 = (noise_8_psd_e2018 * w[111]);
            let noise_8_psd_e2023: f64 = (params.p29 * params.p123);
            let noise_8_psd_e2026: f64 = (params.p0 * params.p2);
            let noise_8_psd_e2027: f64 = (noise_8_psd_e2023 / noise_8_psd_e2026);
            let noise_8_psd_e2028: f64 = (noise_8_psd_e2020 / noise_8_psd_e2027);
            let noise_8_psd_e45648: f64 = (noise_8_psd_e45647 * noise_8_psd_e2028);
            let psd = noise_8_psd_e45648;
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
            let noise_9_psd_e45650: f64 = 1.0;
            let noise_9_psd_e2038: f64 = (4.0 * 1.38062e-23);
            let noise_9_psd_e2040: f64 = (noise_9_psd_e2038 * w[111]);
            let noise_9_psd_e2043: f64 = (params.p29 * params.p145);
            let noise_9_psd_e2046: f64 = (params.p0 * params.p2);
            let noise_9_psd_e2047: f64 = (noise_9_psd_e2043 / noise_9_psd_e2046);
            let noise_9_psd_e2048: f64 = (noise_9_psd_e2040 / noise_9_psd_e2047);
            let noise_9_psd_e45651: f64 = (noise_9_psd_e45650 * noise_9_psd_e2048);
            let psd = noise_9_psd_e45651;
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
            let noise_10_psd_e45653: f64 = 1.0;
            let noise_10_psd_e2058: f64 = (4.0 * 1.38062e-23);
            let noise_10_psd_e2060: f64 = (noise_10_psd_e2058 * w[111]);
            let noise_10_psd_e2063: f64 = (params.p29 * params.p167);
            let noise_10_psd_e2066: f64 = (params.p0 * params.p2);
            let noise_10_psd_e2067: f64 = (noise_10_psd_e2063 / noise_10_psd_e2066);
            let noise_10_psd_e2068: f64 = (noise_10_psd_e2060 / noise_10_psd_e2067);
            let noise_10_psd_e45654: f64 = (noise_10_psd_e45653 * noise_10_psd_e2068);
            let psd = noise_10_psd_e45654;
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
            let noise_11_psd_e45656: f64 = 1.0;
            let noise_11_psd_e2078: f64 = (4.0 * 1.38062e-23);
            let noise_11_psd_e2080: f64 = (noise_11_psd_e2078 * w[111]);
            let noise_11_psd_e2083: f64 = (params.p29 * params.p189);
            let noise_11_psd_e2086: f64 = (params.p0 * params.p2);
            let noise_11_psd_e2087: f64 = (noise_11_psd_e2083 / noise_11_psd_e2086);
            let noise_11_psd_e2088: f64 = (noise_11_psd_e2080 / noise_11_psd_e2087);
            let noise_11_psd_e45657: f64 = (noise_11_psd_e45656 * noise_11_psd_e2088);
            let psd = noise_11_psd_e45657;
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
            let noise_12_psd_e45659: f64 = 1.0;
            let noise_12_psd_e2098: f64 = (4.0 * 1.38062e-23);
            let noise_12_psd_e2100: f64 = (noise_12_psd_e2098 * w[111]);
            let noise_12_psd_e2103: f64 = (params.p29 * params.p211);
            let noise_12_psd_e2106: f64 = (params.p0 * params.p2);
            let noise_12_psd_e2107: f64 = (noise_12_psd_e2103 / noise_12_psd_e2106);
            let noise_12_psd_e2108: f64 = (noise_12_psd_e2100 / noise_12_psd_e2107);
            let noise_12_psd_e45660: f64 = (noise_12_psd_e45659 * noise_12_psd_e2108);
            let psd = noise_12_psd_e45660;
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
            let noise_13_psd_e45662: f64 = 1.0;
            let noise_13_psd_e2118: f64 = (4.0 * 1.38062e-23);
            let noise_13_psd_e2120: f64 = (noise_13_psd_e2118 * w[111]);
            let noise_13_psd_e2123: f64 = (params.p29 * params.p233);
            let noise_13_psd_e2126: f64 = (params.p0 * params.p2);
            let noise_13_psd_e2127: f64 = (noise_13_psd_e2123 / noise_13_psd_e2126);
            let noise_13_psd_e2128: f64 = (noise_13_psd_e2120 / noise_13_psd_e2127);
            let noise_13_psd_e45663: f64 = (noise_13_psd_e45662 * noise_13_psd_e2128);
            let psd = noise_13_psd_e45663;
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
            let noise_14_psd_e45665: f64 = 1.0;
            let noise_14_psd_e2138: f64 = (4.0 * 1.38062e-23);
            let noise_14_psd_e2140: f64 = (noise_14_psd_e2138 * w[111]);
            let noise_14_psd_e2142: f64 = (noise_14_psd_e2140 / w[2]);
            let noise_14_psd_e45666: f64 = (noise_14_psd_e45665 * noise_14_psd_e2142);
            let psd = noise_14_psd_e45666;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if !noise_source_active[15] {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let noise_15_psd_e45668: f64 = 1.0;
            let noise_15_psd_e2152: f64 = (4.0 * 1.38062e-23);
            let noise_15_psd_e2154: f64 = (noise_15_psd_e2152 * w[111]);
            let noise_15_psd_e2156: f64 = (noise_15_psd_e2154 / w[1]);
            let noise_15_psd_e45669: f64 = (noise_15_psd_e45668 * noise_15_psd_e2156);
            let psd = noise_15_psd_e45669;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }

    #[inline(never)]
    fn noise_activation_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701]) {
        let params = &*self.params;
        let noise_activation_schedule_12_0_e2232: f64 = if params.p50 == 0.0 { 1.0 } else { 0.0 };
        w[300] = noise_activation_schedule_12_0_e2232;
        let (noise_activation_schedule_13_0_e2240,) = {
    if (w[300] != 0.0) {
        let noise_activation_schedule_13_0_e2236: f64 = (params.p30 / params.p0);
        let noise_activation_schedule_13_0_e2238: f64 = (noise_activation_schedule_13_0_e2236 / params.p2);
        (noise_activation_schedule_13_0_e2238,)
    } else {
        (w[3],)
    }
};
        w[3] = noise_activation_schedule_13_0_e2240;
        let (noise_activation_schedule_14_0_e2248,) = {
    if (w[300] != 0.0) {
        let noise_activation_schedule_14_0_e2244: f64 = (params.p31 / params.p0);
        let noise_activation_schedule_14_0_e2246: f64 = (noise_activation_schedule_14_0_e2244 / params.p2);
        (noise_activation_schedule_14_0_e2246,)
    } else {
        (w[4],)
    }
};
        w[4] = noise_activation_schedule_14_0_e2248;
        let (noise_activation_schedule_15_0_e2263,) = {
    if (w[300] == 0.0) {
        let noise_activation_schedule_15_0_e2253: f64 = (params.p30 / params.p0);
        let noise_activation_schedule_15_0_e2256: f64 = (params.p29 * params.p54);
        let noise_activation_schedule_15_0_e2258: f64 = (noise_activation_schedule_15_0_e2256 / params.p0);
        let noise_activation_schedule_15_0_e2259: f64 = (noise_activation_schedule_15_0_e2253 + noise_activation_schedule_15_0_e2258);
        let noise_activation_schedule_15_0_e2261: f64 = (noise_activation_schedule_15_0_e2259 / params.p2);
        (noise_activation_schedule_15_0_e2261,)
    } else {
        (w[3],)
    }
};
        w[3] = noise_activation_schedule_15_0_e2263;
        let (noise_activation_schedule_16_0_e2278,) = {
    if (w[300] == 0.0) {
        let noise_activation_schedule_16_0_e2268: f64 = (params.p31 / params.p0);
        let noise_activation_schedule_16_0_e2271: f64 = (params.p29 * params.p66);
        let noise_activation_schedule_16_0_e2273: f64 = (noise_activation_schedule_16_0_e2271 / params.p0);
        let noise_activation_schedule_16_0_e2274: f64 = (noise_activation_schedule_16_0_e2268 + noise_activation_schedule_16_0_e2273);
        let noise_activation_schedule_16_0_e2276: f64 = (noise_activation_schedule_16_0_e2274 / params.p2);
        (noise_activation_schedule_16_0_e2276,)
    } else {
        (w[4],)
    }
};
        w[4] = noise_activation_schedule_16_0_e2278;
        let noise_activation_schedule_4648_0_e45310: f64 = if params.p347 == 1.0 { 1.0 } else { 0.0 };
        w[2686] = noise_activation_schedule_4648_0_e45310;
        let noise_activation_schedule_4654_0_e45376: f64 = if ((params.p79 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2688] = noise_activation_schedule_4654_0_e45376;
        let noise_activation_schedule_4655_0_e45383: f64 = if ((params.p101 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2689] = noise_activation_schedule_4655_0_e45383;
        let noise_activation_schedule_4656_0_e45390: f64 = if ((params.p123 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2690] = noise_activation_schedule_4656_0_e45390;
        let noise_activation_schedule_4657_0_e45397: f64 = if ((params.p145 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2691] = noise_activation_schedule_4657_0_e45397;
        let noise_activation_schedule_4658_0_e45404: f64 = if ((params.p167 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2692] = noise_activation_schedule_4658_0_e45404;
        let noise_activation_schedule_4659_0_e45411: f64 = if ((params.p189 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2693] = noise_activation_schedule_4659_0_e45411;
        let noise_activation_schedule_4660_0_e45418: f64 = if ((params.p211 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2694] = noise_activation_schedule_4660_0_e45418;
        let noise_activation_schedule_4661_0_e45425: f64 = if ((params.p233 > params.p354) && (params.p29 != 0.0)) { 1.0 } else { 0.0 };
        w[2695] = noise_activation_schedule_4661_0_e45425;
        let noise_activation_schedule_4662_0_e45432: f64 = if ((w[3] >= params.p353) && (w[3] > 0.0)) { 1.0 } else { 0.0 };
        w[2696] = noise_activation_schedule_4662_0_e45432;
        let noise_activation_schedule_4663_0_e45439: f64 = if ((w[4] >= params.p353) && (w[4] > 0.0)) { 1.0 } else { 0.0 };
        w[2697] = noise_activation_schedule_4663_0_e45439;
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_0(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0xc03f) != 0 {
            let noise_metadata_schedule_1_0_e2189: f64 = (params.p5 + 273.15);
            w[109] = noise_metadata_schedule_1_0_e2189;
        }
        if (active[0] & 0xffff) != 0 {
            let noise_metadata_schedule_2_0_e2190: f64 = ctx.temperature();
            w[108] = noise_metadata_schedule_2_0_e2190;
        }
        if (active[0] & 0xffff) != 0 {
            w[110] = (ctx.node_voltage(self.nodes[4]) - 0.0);
        }
        if (active[0] & 0xffff) != 0 {
            let noise_metadata_schedule_5_0_e2198: f64 = (w[108] + params.p3);
            let noise_metadata_schedule_5_0_e2200: f64 = (noise_metadata_schedule_5_0_e2198 + w[110]);
            w[111] = noise_metadata_schedule_5_0_e2200;
        }
        if (active[0] & 0xffff) != 0 {
            let noise_metadata_schedule_6_0_e2203: f64 = (-270.0);
            let noise_metadata_schedule_6_0_e2205: f64 = (noise_metadata_schedule_6_0_e2203 + 273.15);
            let noise_metadata_schedule_6_0_e2206: f64 = if w[111] < noise_metadata_schedule_6_0_e2205 { 1.0 } else { 0.0 };
            w[298] = noise_metadata_schedule_6_0_e2206;
        }
        if (active[0] & 0xffff) != 0 {
            let (noise_metadata_schedule_7_0_e2213,) = {
    if (w[298] != 0.0) {
        let noise_metadata_schedule_7_0_e2209: f64 = (-270.0);
        let noise_metadata_schedule_7_0_e2211: f64 = (noise_metadata_schedule_7_0_e2209 + 273.15);
        (noise_metadata_schedule_7_0_e2211,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_7_0_e2213;
        }
        if (active[0] & 0xffff) != 0 {
            let noise_metadata_schedule_8_0_e2217: f64 = (1500.0 + 273.15);
            let noise_metadata_schedule_8_0_e2218: f64 = if w[111] > noise_metadata_schedule_8_0_e2217 { 1.0 } else { 0.0 };
            w[299] = noise_metadata_schedule_8_0_e2218;
        }
        if (active[0] & 0xffff) != 0 {
            let (noise_metadata_schedule_9_0_e2227,) = {
    if ((w[298] == 0.0) && (w[299] != 0.0)) {
        let noise_metadata_schedule_9_0_e2225: f64 = (1500.0 + 273.15);
        (noise_metadata_schedule_9_0_e2225,)
    } else {
        (w[111],)
    }
};
            w[111] = noise_metadata_schedule_9_0_e2227;
        }
        if (active[0] & 0x4000) != 0 {
            w[2] = 0.0;
        }
        if (active[0] & 0x8000) != 0 {
            w[1] = 0.0;
        }
        if (active[0] & 0xc000) != 0 {
            let noise_metadata_schedule_12_0_e2232: f64 = if params.p50 == 0.0 { 1.0 } else { 0.0 };
            w[300] = noise_metadata_schedule_12_0_e2232;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_13_0_e2240,) = {
    if (w[300] != 0.0) {
        let noise_metadata_schedule_13_0_e2236: f64 = (params.p30 / params.p0);
        let noise_metadata_schedule_13_0_e2238: f64 = (noise_metadata_schedule_13_0_e2236 / params.p2);
        (noise_metadata_schedule_13_0_e2238,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_13_0_e2240;
        }
        if (active[0] & 0x8000) != 0 {
            let (noise_metadata_schedule_14_0_e2248,) = {
    if (w[300] != 0.0) {
        let noise_metadata_schedule_14_0_e2244: f64 = (params.p31 / params.p0);
        let noise_metadata_schedule_14_0_e2246: f64 = (noise_metadata_schedule_14_0_e2244 / params.p2);
        (noise_metadata_schedule_14_0_e2246,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_14_0_e2248;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_15_0_e2263,) = {
    if (w[300] == 0.0) {
        let noise_metadata_schedule_15_0_e2253: f64 = (params.p30 / params.p0);
        let noise_metadata_schedule_15_0_e2256: f64 = (params.p29 * params.p54);
        let noise_metadata_schedule_15_0_e2258: f64 = (noise_metadata_schedule_15_0_e2256 / params.p0);
        let noise_metadata_schedule_15_0_e2259: f64 = (noise_metadata_schedule_15_0_e2253 + noise_metadata_schedule_15_0_e2258);
        let noise_metadata_schedule_15_0_e2261: f64 = (noise_metadata_schedule_15_0_e2259 / params.p2);
        (noise_metadata_schedule_15_0_e2261,)
    } else {
        (w[3],)
    }
};
            w[3] = noise_metadata_schedule_15_0_e2263;
        }
        if (active[0] & 0x8000) != 0 {
            let (noise_metadata_schedule_16_0_e2278,) = {
    if (w[300] == 0.0) {
        let noise_metadata_schedule_16_0_e2268: f64 = (params.p31 / params.p0);
        let noise_metadata_schedule_16_0_e2271: f64 = (params.p29 * params.p66);
        let noise_metadata_schedule_16_0_e2273: f64 = (noise_metadata_schedule_16_0_e2271 / params.p0);
        let noise_metadata_schedule_16_0_e2274: f64 = (noise_metadata_schedule_16_0_e2268 + noise_metadata_schedule_16_0_e2273);
        let noise_metadata_schedule_16_0_e2276: f64 = (noise_metadata_schedule_16_0_e2274 / params.p2);
        (noise_metadata_schedule_16_0_e2276,)
    } else {
        (w[4],)
    }
};
            w[4] = noise_metadata_schedule_16_0_e2278;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_17_0_e2285: f64 = if ((w[3] >= params.p353) && (w[3] > 0.0)) { 1.0 } else { 0.0 };
            w[301] = noise_metadata_schedule_17_0_e2285;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_18_0_e2307,) = {
    if (w[301] != 0.0) {
        let noise_metadata_schedule_18_0_e2292: f64 = (w[111] - w[109]);
        let noise_metadata_schedule_18_0_e2293: f64 = (params.p48 * noise_metadata_schedule_18_0_e2292);
        let noise_metadata_schedule_18_0_e2294: f64 = (1.0 + noise_metadata_schedule_18_0_e2293);
        let noise_metadata_schedule_18_0_e2298: f64 = (w[111] - w[109]);
        let noise_metadata_schedule_18_0_e2299: f64 = (params.p49 * noise_metadata_schedule_18_0_e2298);
        let noise_metadata_schedule_18_0_e2302: f64 = (w[111] - w[109]);
        let noise_metadata_schedule_18_0_e2303: f64 = (noise_metadata_schedule_18_0_e2299 * noise_metadata_schedule_18_0_e2302);
        let noise_metadata_schedule_18_0_e2304: f64 = (noise_metadata_schedule_18_0_e2294 + noise_metadata_schedule_18_0_e2303);
        let noise_metadata_schedule_18_0_e2305: f64 = (w[3] * noise_metadata_schedule_18_0_e2304);
        (noise_metadata_schedule_18_0_e2305,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_18_0_e2307;
        }
        if (active[0] & 0x4000) != 0 {
            let noise_metadata_schedule_19_0_e2311: f64 = (0.1 * w[3]);
            let noise_metadata_schedule_19_0_e2312: f64 = if w[2] < noise_metadata_schedule_19_0_e2311 { 1.0 } else { 0.0 };
            w[302] = noise_metadata_schedule_19_0_e2312;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_20_0_e2320,) = {
    if ((w[301] != 0.0) && (w[302] != 0.0)) {
        let noise_metadata_schedule_20_0_e2318: f64 = (0.1 * w[3]);
        (noise_metadata_schedule_20_0_e2318,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_20_0_e2320;
        }
        if (active[0] & 0x4000) != 0 {
            let (noise_metadata_schedule_21_0_e2325,) = {
    if (w[301] == 0.0) {
        (0.0,)
    } else {
        (w[2],)
    }
};
            w[2] = noise_metadata_schedule_21_0_e2325;
        }
        if (active[0] & 0x8000) != 0 {
            let noise_metadata_schedule_22_0_e2332: f64 = if ((w[4] >= params.p353) && (w[4] > 0.0)) { 1.0 } else { 0.0 };
            w[303] = noise_metadata_schedule_22_0_e2332;
        }
        if (active[0] & 0x8000) != 0 {
            let (noise_metadata_schedule_23_0_e2354,) = {
    if (w[303] != 0.0) {
        let noise_metadata_schedule_23_0_e2339: f64 = (w[111] - w[109]);
        let noise_metadata_schedule_23_0_e2340: f64 = (params.p48 * noise_metadata_schedule_23_0_e2339);
        let noise_metadata_schedule_23_0_e2341: f64 = (1.0 + noise_metadata_schedule_23_0_e2340);
        let noise_metadata_schedule_23_0_e2345: f64 = (w[111] - w[109]);
        let noise_metadata_schedule_23_0_e2346: f64 = (params.p49 * noise_metadata_schedule_23_0_e2345);
        let noise_metadata_schedule_23_0_e2349: f64 = (w[111] - w[109]);
        let noise_metadata_schedule_23_0_e2350: f64 = (noise_metadata_schedule_23_0_e2346 * noise_metadata_schedule_23_0_e2349);
        let noise_metadata_schedule_23_0_e2351: f64 = (noise_metadata_schedule_23_0_e2341 + noise_metadata_schedule_23_0_e2350);
        let noise_metadata_schedule_23_0_e2352: f64 = (w[4] * noise_metadata_schedule_23_0_e2351);
        (noise_metadata_schedule_23_0_e2352,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_23_0_e2354;
        }
        if (active[0] & 0x8000) != 0 {
            let noise_metadata_schedule_24_0_e2358: f64 = (0.1 * w[4]);
            let noise_metadata_schedule_24_0_e2359: f64 = if w[1] < noise_metadata_schedule_24_0_e2358 { 1.0 } else { 0.0 };
            w[304] = noise_metadata_schedule_24_0_e2359;
        }
        if (active[0] & 0x8000) != 0 {
            let (noise_metadata_schedule_25_0_e2367,) = {
    if ((w[303] != 0.0) && (w[304] != 0.0)) {
        let noise_metadata_schedule_25_0_e2365: f64 = (0.1 * w[4]);
        (noise_metadata_schedule_25_0_e2365,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_25_0_e2367;
        }
        if (active[0] & 0x8000) != 0 {
            let (noise_metadata_schedule_26_0_e2372,) = {
    if (w[303] == 0.0) {
        (0.0,)
    } else {
        (w[1],)
    }
};
            w[1] = noise_metadata_schedule_26_0_e2372;
        }
        if (active[0] & 0x3f) != 0 {
            let noise_metadata_schedule_29_0_e2401: f64 = (1.38062e-23 * w[111]);
            let noise_metadata_schedule_29_0_e2403: f64 = (noise_metadata_schedule_29_0_e2401 / 1.60219e-19);
            w[113] = noise_metadata_schedule_29_0_e2403;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_33_0_e2420: f64 = (w[111] / w[109]);
            let noise_metadata_schedule_33_0_e2422: f64 = {let pb=noise_metadata_schedule_33_0_e2420;pb*pb*pb};
            w[112] = noise_metadata_schedule_33_0_e2422;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_46_0_e2668: f64 = (w[111] - w[109]);
            let noise_metadata_schedule_46_0_e2669: f64 = (params.p8 * noise_metadata_schedule_46_0_e2668);
            let noise_metadata_schedule_46_0_e2670: f64 = (1.0 + noise_metadata_schedule_46_0_e2669);
            let (noise_metadata_schedule_46_0_e2681,) = {
    if (noise_metadata_schedule_46_0_e2670 < 0.01) {
        (0.01,)
    } else {
        let noise_metadata_schedule_46_0_e2678: f64 = (w[111] - w[109]);
        let noise_metadata_schedule_46_0_e2679: f64 = (params.p8 * noise_metadata_schedule_46_0_e2678);
        let noise_metadata_schedule_46_0_e2680: f64 = (1.0 + noise_metadata_schedule_46_0_e2679);
        (noise_metadata_schedule_46_0_e2680,)
    }
};
            let noise_metadata_schedule_46_0_e2682: f64 = (params.p7 * noise_metadata_schedule_46_0_e2681);
            w[19] = noise_metadata_schedule_46_0_e2682;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_71_0_e3165: f64 = (params.p6 * (ctx.node_voltage(self.nodes[5]) - ctx.node_voltage(self.nodes[9])));
            w[44] = noise_metadata_schedule_71_0_e3165;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_72_0_e3168: f64 = (params.p6 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
            w[45] = noise_metadata_schedule_72_0_e3168;
        }
        if (active[0] & 0x30) != 0 {
            w[224] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[226] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[225] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[227] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[228] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[229] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[230] = 1.0;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_91_0_e3295: f64 = if params.p328 == 1.0 { 1.0 } else { 0.0 };
            w[308] = noise_metadata_schedule_91_0_e3295;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_95_0_e3413: f64 = if params.p328 == 2.0 { 1.0 } else { 0.0 };
            w[309] = noise_metadata_schedule_95_0_e3413;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_96_0_e3420,) = {
    if ((w[308] == 0.0) && (w[309] != 0.0)) {
        ((ctx.node_voltage(self.nodes[22]) - 0.0),)
    } else {
        (w[224],)
    }
};
            w[224] = noise_metadata_schedule_96_0_e3420;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_97_0_e3427,) = {
    if ((w[308] == 0.0) && (w[309] != 0.0)) {
        ((ctx.node_voltage(self.nodes[23]) - 0.0),)
    } else {
        (w[225],)
    }
};
            w[225] = noise_metadata_schedule_97_0_e3427;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_98_0_e3439,) = {
    if ((w[308] == 0.0) && (w[309] != 0.0)) {
        let noise_metadata_schedule_98_0_e3434: f64 = (w[225] - w[224]);
        let noise_metadata_schedule_98_0_e3435: f64 = (noise_metadata_schedule_98_0_e3434).abs();
        let noise_metadata_schedule_98_0_e3437: f64 = (noise_metadata_schedule_98_0_e3435 / params.p338);
        (noise_metadata_schedule_98_0_e3437,)
    } else {
        (w[228],)
    }
};
            w[228] = noise_metadata_schedule_98_0_e3439;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_99_0_e3446,) = {
    if ((w[308] == 0.0) && (w[309] != 0.0)) {
        ((ctx.node_voltage(self.nodes[25]) - 0.0),)
    } else {
        (w[226],)
    }
};
            w[226] = noise_metadata_schedule_99_0_e3446;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_100_0_e3453,) = {
    if ((w[308] == 0.0) && (w[309] != 0.0)) {
        ((ctx.node_voltage(self.nodes[26]) - 0.0),)
    } else {
        (w[227],)
    }
};
            w[227] = noise_metadata_schedule_100_0_e3453;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_101_0_e3465,) = {
    if ((w[308] == 0.0) && (w[309] != 0.0)) {
        let noise_metadata_schedule_101_0_e3460: f64 = (w[227] - w[226]);
        let noise_metadata_schedule_101_0_e3461: f64 = (noise_metadata_schedule_101_0_e3460).abs();
        let noise_metadata_schedule_101_0_e3463: f64 = (noise_metadata_schedule_101_0_e3461 / params.p337);
        (noise_metadata_schedule_101_0_e3463,)
    } else {
        (w[229],)
    }
};
            w[229] = noise_metadata_schedule_101_0_e3465;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_102_0_e3478,) = {
    if ((w[308] == 0.0) && (w[309] != 0.0)) {
        let noise_metadata_schedule_102_0_e3473: f64 = (1.0 + w[228]);
        let noise_metadata_schedule_102_0_e3475: f64 = (noise_metadata_schedule_102_0_e3473 + w[229]);
        let noise_metadata_schedule_102_0_e3476: f64 = (1.0 / noise_metadata_schedule_102_0_e3475);
        (noise_metadata_schedule_102_0_e3476,)
    } else {
        (w[230],)
    }
};
            w[230] = noise_metadata_schedule_102_0_e3478;
        }
        if (active[0] & 0x30) != 0 {
            w[1797] = w[45];
        }
        if (active[0] & 0x30) != 0 {
            w[1798] = w[44];
        }
        if (active[0] & 0x30) != 0 {
            w[1803] = w[111];
        }
        if (active[0] & 0x30) != 0 {
            w[1804] = w[109];
        }
        if (active[0] & 0x30) != 0 {
            w[1805] = w[113];
        }
        if (active[0] & 0x30) != 0 {
            w[1806] = params.p0;
        }
        if (active[0] & 0x30) != 0 {
            w[1807] = params.p1;
        }
        if (active[0] & 0x30) != 0 {
            w[1808] = w[19];
        }
        if (active[0] & 0x30) != 0 {
            w[1812] = params.p35;
        }
        if (active[0] & 0x30) != 0 {
            w[1813] = params.p36;
        }
        if (active[0] & 0x30) != 0 {
            w[1814] = params.p37;
        }
        if (active[0] & 0x30) != 0 {
            w[1815] = params.p38;
        }
        if (active[0] & 0x30) != 0 {
            w[1816] = params.p40;
        }
        if (active[0] & 0x30) != 0 {
            w[1817] = params.p41;
        }
        if (active[0] & 0x30) != 0 {
            w[1818] = params.p32;
        }
        if (active[0] & 0x30) != 0 {
            w[1819] = params.p33;
        }
        if (active[0] & 0x30) != 0 {
            w[1820] = params.p34;
        }
        if (active[0] & 0x30) != 0 {
            w[1821] = params.p44;
        }
        if (active[0] & 0x30) != 0 {
            w[1822] = params.p43;
        }
        if (active[0] & 0x30) != 0 {
            w[1823] = params.p46;
        }
        if (active[0] & 0x30) != 0 {
            w[1824] = params.p39;
        }
        if (active[0] & 0x30) != 0 {
            w[1825] = params.p47;
        }
        if (active[0] & 0x30) != 0 {
            w[1826] = params.p45;
        }
        if (active[0] & 0x30) != 0 {
            w[1827] = params.p42;
        }
        if (active[0] & 0x30) != 0 {
            w[1828] = params.p2;
        }
        if (active[0] & 0x30) != 0 {
            w[1829] = params.p6;
        }
        if (active[0] & 0x30) != 0 {
            w[1830] = w[230];
        }
        if (active[0] & 0x30) != 0 {
            w[1835] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[1836] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[1840] = 0.0;
        }
        if (active[0] & 0x20) != 0 {
            w[1845] = 0.0;
        }
        if (active[0] & 0x20) != 0 {
            w[1849] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[1853] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[1855] = 0.0;
        }
        if (active[0] & 0x20) != 0 {
            w[1856] = 0.0;
        }
        if (active[0] & 0x20) != 0 {
            w[1858] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[1866] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            w[1868] = 0.0;
        }
        if (active[0] & 0x20) != 0 {
            w[1879] = 0.0;
        }
        if (active[0] & 0x20) != 0 {
            w[1881] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3117_0_e28252,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3117_0_e28236: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3117_0_e28238: f64 = (noise_metadata_schedule_3117_0_e28236 * w[1798]);
        let noise_metadata_schedule_3117_0_e28239: f64 = (noise_metadata_schedule_3117_0_e28238).tanh();
        let noise_metadata_schedule_3117_0_e28240: f64 = (w[1798] * noise_metadata_schedule_3117_0_e28239);
        (noise_metadata_schedule_3117_0_e28240,)
    } else {
        let (noise_metadata_schedule_3117_0_e28251,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3117_0_e28246: f64 = (w[1798] * w[1798]);
                let noise_metadata_schedule_3117_0_e28248: f64 = (noise_metadata_schedule_3117_0_e28246 + params.p53);
                let noise_metadata_schedule_3117_0_e28249: f64 = (noise_metadata_schedule_3117_0_e28248).sqrt();
                (noise_metadata_schedule_3117_0_e28249,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3117_0_e28251,)
    }
};
            w[1896] = noise_metadata_schedule_3117_0_e28252;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3118_0_e28255: f64 = (w[1797] - w[1798]);
            w[1897] = noise_metadata_schedule_3118_0_e28255;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3119_0_e28258: f64 = (w[1817] * w[1805]);
            w[1831] = noise_metadata_schedule_3119_0_e28258;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_1(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3120_0_e28262: f64 = (2.302585092994046 * w[1805]);
            let noise_metadata_schedule_3120_0_e28263: f64 = (w[1813] / noise_metadata_schedule_3120_0_e28262);
            let noise_metadata_schedule_3120_0_e28266: f64 = (w[1816] * w[1896]);
            let noise_metadata_schedule_3120_0_e28267: f64 = (noise_metadata_schedule_3120_0_e28263 + noise_metadata_schedule_3120_0_e28266);
            w[1833] = noise_metadata_schedule_3120_0_e28267;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3121_0_e28272: f64 = (w[1803] - w[1804]);
            let noise_metadata_schedule_3121_0_e28273: f64 = (w[1823] * noise_metadata_schedule_3121_0_e28272);
            let noise_metadata_schedule_3121_0_e28274: f64 = (w[1812] + noise_metadata_schedule_3121_0_e28273);
            w[1834] = noise_metadata_schedule_3121_0_e28274;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3122_0_e28277: f64 = (w[1803] / w[1804]);
            let noise_metadata_schedule_3122_0_e28279: f64 = (noise_metadata_schedule_3122_0_e28277).powf(w[1825]);
            w[1852] = noise_metadata_schedule_3122_0_e28279;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3123_0_e28282: f64 = if w[1824] != 0.0 { 1.0 } else { 0.0 };
            w[1900] = noise_metadata_schedule_3123_0_e28282;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3124_0_e28298,) = {
    if (w[1900] != 0.0) {
        let noise_metadata_schedule_3124_0_e28288: f64 = (w[1896] / w[1824]);
        let noise_metadata_schedule_3124_0_e28290: f64 = (noise_metadata_schedule_3124_0_e28288).powf(w[1820]);
        let noise_metadata_schedule_3124_0_e28291: f64 = (1.0 + noise_metadata_schedule_3124_0_e28290);
        let noise_metadata_schedule_3124_0_e28294: f64 = (1.0 / w[1820]);
        let noise_metadata_schedule_3124_0_e28295: f64 = (noise_metadata_schedule_3124_0_e28291).powf(noise_metadata_schedule_3124_0_e28294);
        let noise_metadata_schedule_3124_0_e28296: f64 = (w[1896] / noise_metadata_schedule_3124_0_e28295);
        (noise_metadata_schedule_3124_0_e28296,)
    } else {
        (w[1835],)
    }
};
            w[1835] = noise_metadata_schedule_3124_0_e28298;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3125_0_e28303,) = {
    if (w[1900] == 0.0) {
        (0.0,)
    } else {
        (w[1835],)
    }
};
            w[1835] = noise_metadata_schedule_3125_0_e28303;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3126_0_e28307: f64 = (w[1835] * w[1815]);
            let noise_metadata_schedule_3126_0_e28308: f64 = (w[1814] - noise_metadata_schedule_3126_0_e28307);
            let noise_metadata_schedule_3126_0_e28310: f64 = (noise_metadata_schedule_3126_0_e28308 * w[1896]);
            w[1832] = noise_metadata_schedule_3126_0_e28310;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3127_0_e28313: f64 = (w[1834] - w[1832]);
            w[1795] = noise_metadata_schedule_3127_0_e28313;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3128_0_e28316: f64 = (2.0 * w[1833]);
            let noise_metadata_schedule_3128_0_e28318: f64 = (noise_metadata_schedule_3128_0_e28316 * w[1805]);
            w[1837] = noise_metadata_schedule_3128_0_e28318;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3129_0_e28321: f64 = (w[1808] * w[1837]);
            w[1838] = noise_metadata_schedule_3129_0_e28321;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3130_0_e28325: f64 = (params.p51 * w[1831]);
            let noise_metadata_schedule_3130_0_e28327: f64 = (noise_metadata_schedule_3130_0_e28325 / 2.0);
            let noise_metadata_schedule_3130_0_e28328: f64 = (w[1795] - noise_metadata_schedule_3130_0_e28327);
            w[1895] = noise_metadata_schedule_3130_0_e28328;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3131_0_e28372,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3131_0_e28336: f64 = (w[1797] + w[1897]);
        let noise_metadata_schedule_3131_0_e28339: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3131_0_e28342: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3131_0_e28345: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3131_0_e28346: f64 = (noise_metadata_schedule_3131_0_e28342 * noise_metadata_schedule_3131_0_e28345);
        let noise_metadata_schedule_3131_0_e28347: f64 = (noise_metadata_schedule_3131_0_e28346).tanh();
        let noise_metadata_schedule_3131_0_e28348: f64 = (noise_metadata_schedule_3131_0_e28339 * noise_metadata_schedule_3131_0_e28347);
        let noise_metadata_schedule_3131_0_e28349: f64 = (noise_metadata_schedule_3131_0_e28336 + noise_metadata_schedule_3131_0_e28348);
        let noise_metadata_schedule_3131_0_e28350: f64 = (0.5 * noise_metadata_schedule_3131_0_e28349);
        (noise_metadata_schedule_3131_0_e28350,)
    } else {
        let (noise_metadata_schedule_3131_0_e28371,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3131_0_e28357: f64 = (w[1797] + w[1897]);
                let noise_metadata_schedule_3131_0_e28360: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3131_0_e28363: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3131_0_e28364: f64 = (noise_metadata_schedule_3131_0_e28360 * noise_metadata_schedule_3131_0_e28363);
                let noise_metadata_schedule_3131_0_e28366: f64 = (noise_metadata_schedule_3131_0_e28364 + params.p53);
                let noise_metadata_schedule_3131_0_e28367: f64 = (noise_metadata_schedule_3131_0_e28366).sqrt();
                let noise_metadata_schedule_3131_0_e28368: f64 = (noise_metadata_schedule_3131_0_e28357 + noise_metadata_schedule_3131_0_e28367);
                let noise_metadata_schedule_3131_0_e28369: f64 = (0.5 * noise_metadata_schedule_3131_0_e28368);
                (noise_metadata_schedule_3131_0_e28369,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3131_0_e28371,)
    }
};
            let noise_metadata_schedule_3131_0_e28374: f64 = (noise_metadata_schedule_3131_0_e28372 - w[1895]);
            let noise_metadata_schedule_3131_0_e28376: f64 = (noise_metadata_schedule_3131_0_e28374 / w[1831]);
            w[1894] = noise_metadata_schedule_3131_0_e28376;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3132_0_e28379: f64 = if w[1894] > 50.0 { 1.0 } else { 0.0 };
            w[1901] = noise_metadata_schedule_3132_0_e28379;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3133_0_e28383,) = {
    if (w[1901] != 0.0) {
        (0.0,)
    } else {
        (w[1853],)
    }
};
            w[1853] = noise_metadata_schedule_3133_0_e28383;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3134_0_e28386: f64 = (-50.0);
            let noise_metadata_schedule_3134_0_e28387: f64 = if w[1894] < noise_metadata_schedule_3134_0_e28386 { 1.0 } else { 0.0 };
            w[1902] = noise_metadata_schedule_3134_0_e28387;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3135_0_e28394,) = {
    if ((w[1901] == 0.0) && (w[1902] != 0.0)) {
        (1.0,)
    } else {
        (w[1853],)
    }
};
            w[1853] = noise_metadata_schedule_3135_0_e28394;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3136_0_e28407,) = {
    if ((w[1901] == 0.0) && (w[1902] == 0.0)) {
        let noise_metadata_schedule_3136_0_e28403: f64 = (w[1894]).exp();
        let noise_metadata_schedule_3136_0_e28404: f64 = (1.0 + noise_metadata_schedule_3136_0_e28403);
        let noise_metadata_schedule_3136_0_e28405: f64 = (1.0 / noise_metadata_schedule_3136_0_e28404);
        (noise_metadata_schedule_3136_0_e28405,)
    } else {
        (w[1853],)
    }
};
            w[1853] = noise_metadata_schedule_3136_0_e28407;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3137_0_e28451,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3137_0_e28415: f64 = (w[1797] + w[1897]);
        let noise_metadata_schedule_3137_0_e28418: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3137_0_e28421: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3137_0_e28424: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3137_0_e28425: f64 = (noise_metadata_schedule_3137_0_e28421 * noise_metadata_schedule_3137_0_e28424);
        let noise_metadata_schedule_3137_0_e28426: f64 = (noise_metadata_schedule_3137_0_e28425).tanh();
        let noise_metadata_schedule_3137_0_e28427: f64 = (noise_metadata_schedule_3137_0_e28418 * noise_metadata_schedule_3137_0_e28426);
        let noise_metadata_schedule_3137_0_e28428: f64 = (noise_metadata_schedule_3137_0_e28415 + noise_metadata_schedule_3137_0_e28427);
        let noise_metadata_schedule_3137_0_e28429: f64 = (0.5 * noise_metadata_schedule_3137_0_e28428);
        (noise_metadata_schedule_3137_0_e28429,)
    } else {
        let (noise_metadata_schedule_3137_0_e28450,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3137_0_e28436: f64 = (w[1797] + w[1897]);
                let noise_metadata_schedule_3137_0_e28439: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3137_0_e28442: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3137_0_e28443: f64 = (noise_metadata_schedule_3137_0_e28439 * noise_metadata_schedule_3137_0_e28442);
                let noise_metadata_schedule_3137_0_e28445: f64 = (noise_metadata_schedule_3137_0_e28443 + params.p53);
                let noise_metadata_schedule_3137_0_e28446: f64 = (noise_metadata_schedule_3137_0_e28445).sqrt();
                let noise_metadata_schedule_3137_0_e28447: f64 = (noise_metadata_schedule_3137_0_e28436 + noise_metadata_schedule_3137_0_e28446);
                let noise_metadata_schedule_3137_0_e28448: f64 = (0.5 * noise_metadata_schedule_3137_0_e28447);
                (noise_metadata_schedule_3137_0_e28448,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3137_0_e28450,)
    }
};
            let noise_metadata_schedule_3137_0_e28455: f64 = (params.p51 * 0.1);
            let noise_metadata_schedule_3137_0_e28457: f64 = (noise_metadata_schedule_3137_0_e28455 * w[1831]);
            let noise_metadata_schedule_3137_0_e28459: f64 = (noise_metadata_schedule_3137_0_e28457 * w[1853]);
            let noise_metadata_schedule_3137_0_e28460: f64 = (w[1795] - noise_metadata_schedule_3137_0_e28459);
            let noise_metadata_schedule_3137_0_e28461: f64 = (noise_metadata_schedule_3137_0_e28451 - noise_metadata_schedule_3137_0_e28460);
            let noise_metadata_schedule_3137_0_e28463: f64 = (noise_metadata_schedule_3137_0_e28461 / w[1837]);
            w[1854] = noise_metadata_schedule_3137_0_e28463;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3138_0_e28466: f64 = if w[1854] > 50.0 { 1.0 } else { 0.0 };
            w[1903] = noise_metadata_schedule_3138_0_e28466;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3139_0_e28472,) = {
    if (w[1903] != 0.0) {
        let noise_metadata_schedule_3139_0_e28470: f64 = (w[1838] * w[1854]);
        (noise_metadata_schedule_3139_0_e28470,)
    } else {
        (w[1855],)
    }
};
            w[1855] = noise_metadata_schedule_3139_0_e28472;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3140_0_e28475: f64 = (-50.0);
            let noise_metadata_schedule_3140_0_e28476: f64 = if w[1854] < noise_metadata_schedule_3140_0_e28475 { 1.0 } else { 0.0 };
            w[1904] = noise_metadata_schedule_3140_0_e28476;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3141_0_e28486,) = {
    if ((w[1903] == 0.0) && (w[1904] != 0.0)) {
        let noise_metadata_schedule_3141_0_e28483: f64 = (w[1854]).exp();
        let noise_metadata_schedule_3141_0_e28484: f64 = (w[1838] * noise_metadata_schedule_3141_0_e28483);
        (noise_metadata_schedule_3141_0_e28484,)
    } else {
        (w[1855],)
    }
};
            w[1855] = noise_metadata_schedule_3141_0_e28486;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3142_0_e28500,) = {
    if ((w[1903] == 0.0) && (w[1904] == 0.0)) {
        let noise_metadata_schedule_3142_0_e28495: f64 = (w[1854]).exp();
        let noise_metadata_schedule_3142_0_e28496: f64 = (1.0 + noise_metadata_schedule_3142_0_e28495);
        let noise_metadata_schedule_3142_0_e28497: f64 = (noise_metadata_schedule_3142_0_e28496).ln();
        let noise_metadata_schedule_3142_0_e28498: f64 = (w[1838] * noise_metadata_schedule_3142_0_e28497);
        (noise_metadata_schedule_3142_0_e28498,)
    } else {
        (w[1855],)
    }
};
            w[1855] = noise_metadata_schedule_3142_0_e28500;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3143_0_e28506: f64 = (w[1821] * w[1855]);
            let noise_metadata_schedule_3143_0_e28508: f64 = (noise_metadata_schedule_3143_0_e28506 / w[1808]);
            let noise_metadata_schedule_3143_0_e28509: f64 = (1.0 + noise_metadata_schedule_3143_0_e28508);
            let noise_metadata_schedule_3143_0_e28510: f64 = (w[1852] * noise_metadata_schedule_3143_0_e28509);
            let noise_metadata_schedule_3143_0_e28511: f64 = (w[1819] / noise_metadata_schedule_3143_0_e28510);
            w[1841] = noise_metadata_schedule_3143_0_e28511;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3144_0_e28516: f64 = (w[1826] * w[1804]);
            let noise_metadata_schedule_3144_0_e28517: f64 = (1.0 + noise_metadata_schedule_3144_0_e28516);
            let noise_metadata_schedule_3144_0_e28521: f64 = (w[1826] * w[1803]);
            let noise_metadata_schedule_3144_0_e28522: f64 = (1.0 + noise_metadata_schedule_3144_0_e28521);
            let noise_metadata_schedule_3144_0_e28523: f64 = (noise_metadata_schedule_3144_0_e28517 / noise_metadata_schedule_3144_0_e28522);
            let noise_metadata_schedule_3144_0_e28524: f64 = (w[1818] * noise_metadata_schedule_3144_0_e28523);
            let noise_metadata_schedule_3144_0_e28528: f64 = (w[1827] * w[1896]);
            let noise_metadata_schedule_3144_0_e28530: f64 = (noise_metadata_schedule_3144_0_e28528 / w[1807]);
            let noise_metadata_schedule_3144_0_e28531: f64 = (1.0 + noise_metadata_schedule_3144_0_e28530);
            let noise_metadata_schedule_3144_0_e28532: f64 = (noise_metadata_schedule_3144_0_e28524 * noise_metadata_schedule_3144_0_e28531);
            let noise_metadata_schedule_3144_0_e28536: f64 = (w[1822] * w[1855]);
            let noise_metadata_schedule_3144_0_e28538: f64 = (noise_metadata_schedule_3144_0_e28536 / w[1808]);
            let noise_metadata_schedule_3144_0_e28539: f64 = (1.0 + noise_metadata_schedule_3144_0_e28538);
            let noise_metadata_schedule_3144_0_e28540: f64 = (noise_metadata_schedule_3144_0_e28532 / noise_metadata_schedule_3144_0_e28539);
            w[1842] = noise_metadata_schedule_3144_0_e28540;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3145_0_e28543: f64 = (2.0 * w[1853]);
            let noise_metadata_schedule_3145_0_e28545: f64 = (noise_metadata_schedule_3145_0_e28543 * w[1805]);
            let noise_metadata_schedule_3145_0_e28547: f64 = (noise_metadata_schedule_3145_0_e28545 * w[1841]);
            let noise_metadata_schedule_3145_0_e28549: f64 = (noise_metadata_schedule_3145_0_e28547 / w[1807]);
            let noise_metadata_schedule_3145_0_e28552: f64 = (1.0 - w[1853]);
            let noise_metadata_schedule_3145_0_e28554: f64 = (noise_metadata_schedule_3145_0_e28552 * w[1842]);
            let noise_metadata_schedule_3145_0_e28555: f64 = (noise_metadata_schedule_3145_0_e28549 + noise_metadata_schedule_3145_0_e28554);
            w[1843] = noise_metadata_schedule_3145_0_e28555;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3146_0_e28558: f64 = (w[1842] * w[1807]);
            let noise_metadata_schedule_3146_0_e28560: f64 = (noise_metadata_schedule_3146_0_e28558 / w[1841]);
            w[1859] = noise_metadata_schedule_3146_0_e28560;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3147_0_e28565: f64 = (2.0 * w[1855]);
            let noise_metadata_schedule_3147_0_e28567: f64 = (noise_metadata_schedule_3147_0_e28565 / w[1808]);
            let noise_metadata_schedule_3147_0_e28569: f64 = (noise_metadata_schedule_3147_0_e28567 / w[1859]);
            let noise_metadata_schedule_3147_0_e28570: f64 = (1.0 + noise_metadata_schedule_3147_0_e28569);
            let noise_metadata_schedule_3147_0_e28571: f64 = (noise_metadata_schedule_3147_0_e28570).sqrt();
            let noise_metadata_schedule_3147_0_e28572: f64 = (w[1859] * noise_metadata_schedule_3147_0_e28571);
            let noise_metadata_schedule_3147_0_e28574: f64 = (noise_metadata_schedule_3147_0_e28572 - w[1859]);
            w[1860] = noise_metadata_schedule_3147_0_e28574;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3148_0_e28578: f64 = (1.0 - w[1853]);
            let noise_metadata_schedule_3148_0_e28579: f64 = (w[1859] * noise_metadata_schedule_3148_0_e28578);
            let noise_metadata_schedule_3148_0_e28582: f64 = (w[1837] * w[1853]);
            let noise_metadata_schedule_3148_0_e28583: f64 = (noise_metadata_schedule_3148_0_e28579 + noise_metadata_schedule_3148_0_e28582);
            w[1861] = noise_metadata_schedule_3148_0_e28583;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3149_0_e28587: f64 = (1.0 - w[1853]);
            let noise_metadata_schedule_3149_0_e28588: f64 = (w[1860] * noise_metadata_schedule_3149_0_e28587);
            let noise_metadata_schedule_3149_0_e28591: f64 = (w[1837] * w[1853]);
            let noise_metadata_schedule_3149_0_e28592: f64 = (noise_metadata_schedule_3149_0_e28588 + noise_metadata_schedule_3149_0_e28591);
            w[1796] = noise_metadata_schedule_3149_0_e28592;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3150_0_e28650,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3150_0_e28603: f64 = (w[1798] / w[1796]);
        let noise_metadata_schedule_3150_0_e28604: f64 = noise_metadata_schedule_3150_0_e28603;
        let noise_metadata_schedule_3150_0_e28608: f64 = (w[1798] / w[1796]);
        let noise_metadata_schedule_3150_0_e28609: f64 = (-noise_metadata_schedule_3150_0_e28608);
        let noise_metadata_schedule_3150_0_e28612: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3150_0_e28616: f64 = (w[1798] / w[1796]);
        let noise_metadata_schedule_3150_0_e28617: f64 = (-noise_metadata_schedule_3150_0_e28616);
        let noise_metadata_schedule_3150_0_e28618: f64 = (noise_metadata_schedule_3150_0_e28612 * noise_metadata_schedule_3150_0_e28617);
        let noise_metadata_schedule_3150_0_e28619: f64 = (noise_metadata_schedule_3150_0_e28618).tanh();
        let noise_metadata_schedule_3150_0_e28620: f64 = (noise_metadata_schedule_3150_0_e28609 * noise_metadata_schedule_3150_0_e28619);
        let noise_metadata_schedule_3150_0_e28621: f64 = (noise_metadata_schedule_3150_0_e28604 + noise_metadata_schedule_3150_0_e28620);
        let noise_metadata_schedule_3150_0_e28622: f64 = (0.5 * noise_metadata_schedule_3150_0_e28621);
        (noise_metadata_schedule_3150_0_e28622,)
    } else {
        let (noise_metadata_schedule_3150_0_e28649,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3150_0_e28630: f64 = (w[1798] / w[1796]);
                let noise_metadata_schedule_3150_0_e28631: f64 = noise_metadata_schedule_3150_0_e28630;
                let noise_metadata_schedule_3150_0_e28635: f64 = (w[1798] / w[1796]);
                let noise_metadata_schedule_3150_0_e28636: f64 = (-noise_metadata_schedule_3150_0_e28635);
                let noise_metadata_schedule_3150_0_e28640: f64 = (w[1798] / w[1796]);
                let noise_metadata_schedule_3150_0_e28641: f64 = (-noise_metadata_schedule_3150_0_e28640);
                let noise_metadata_schedule_3150_0_e28642: f64 = (noise_metadata_schedule_3150_0_e28636 * noise_metadata_schedule_3150_0_e28641);
                let noise_metadata_schedule_3150_0_e28644: f64 = (noise_metadata_schedule_3150_0_e28642 + params.p53);
                let noise_metadata_schedule_3150_0_e28645: f64 = (noise_metadata_schedule_3150_0_e28644).sqrt();
                let noise_metadata_schedule_3150_0_e28646: f64 = (noise_metadata_schedule_3150_0_e28631 + noise_metadata_schedule_3150_0_e28645);
                let noise_metadata_schedule_3150_0_e28647: f64 = (0.5 * noise_metadata_schedule_3150_0_e28646);
                (noise_metadata_schedule_3150_0_e28647,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3150_0_e28649,)
    }
};
            let noise_metadata_schedule_3150_0_e28652: f64 = (noise_metadata_schedule_3150_0_e28650).powf(w[1820]);
            let noise_metadata_schedule_3150_0_e28653: f64 = (1.0 + noise_metadata_schedule_3150_0_e28652);
            let noise_metadata_schedule_3150_0_e28656: f64 = (1.0 / w[1820]);
            let noise_metadata_schedule_3150_0_e28657: f64 = (noise_metadata_schedule_3150_0_e28653).powf(noise_metadata_schedule_3150_0_e28656);
            let noise_metadata_schedule_3150_0_e28658: f64 = (1.0 / noise_metadata_schedule_3150_0_e28657);
            w[1862] = noise_metadata_schedule_3150_0_e28658;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3151_0_e28661: f64 = (w[1798] * w[1862]);
            w[1863] = noise_metadata_schedule_3151_0_e28661;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3152_0_e28725,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3152_0_e28671: f64 = (-w[1798]);
        let noise_metadata_schedule_3152_0_e28673: f64 = (noise_metadata_schedule_3152_0_e28671 / w[1796]);
        let noise_metadata_schedule_3152_0_e28674: f64 = noise_metadata_schedule_3152_0_e28673;
        let noise_metadata_schedule_3152_0_e28677: f64 = (-w[1798]);
        let noise_metadata_schedule_3152_0_e28679: f64 = (noise_metadata_schedule_3152_0_e28677 / w[1796]);
        let noise_metadata_schedule_3152_0_e28680: f64 = (-noise_metadata_schedule_3152_0_e28679);
        let noise_metadata_schedule_3152_0_e28683: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3152_0_e28686: f64 = (-w[1798]);
        let noise_metadata_schedule_3152_0_e28688: f64 = (noise_metadata_schedule_3152_0_e28686 / w[1796]);
        let noise_metadata_schedule_3152_0_e28689: f64 = (-noise_metadata_schedule_3152_0_e28688);
        let noise_metadata_schedule_3152_0_e28690: f64 = (noise_metadata_schedule_3152_0_e28683 * noise_metadata_schedule_3152_0_e28689);
        let noise_metadata_schedule_3152_0_e28691: f64 = (noise_metadata_schedule_3152_0_e28690).tanh();
        let noise_metadata_schedule_3152_0_e28692: f64 = (noise_metadata_schedule_3152_0_e28680 * noise_metadata_schedule_3152_0_e28691);
        let noise_metadata_schedule_3152_0_e28693: f64 = (noise_metadata_schedule_3152_0_e28674 + noise_metadata_schedule_3152_0_e28692);
        let noise_metadata_schedule_3152_0_e28694: f64 = (0.5 * noise_metadata_schedule_3152_0_e28693);
        (noise_metadata_schedule_3152_0_e28694,)
    } else {
        let (noise_metadata_schedule_3152_0_e28724,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3152_0_e28701: f64 = (-w[1798]);
                let noise_metadata_schedule_3152_0_e28703: f64 = (noise_metadata_schedule_3152_0_e28701 / w[1796]);
                let noise_metadata_schedule_3152_0_e28704: f64 = noise_metadata_schedule_3152_0_e28703;
                let noise_metadata_schedule_3152_0_e28707: f64 = (-w[1798]);
                let noise_metadata_schedule_3152_0_e28709: f64 = (noise_metadata_schedule_3152_0_e28707 / w[1796]);
                let noise_metadata_schedule_3152_0_e28710: f64 = (-noise_metadata_schedule_3152_0_e28709);
                let noise_metadata_schedule_3152_0_e28713: f64 = (-w[1798]);
                let noise_metadata_schedule_3152_0_e28715: f64 = (noise_metadata_schedule_3152_0_e28713 / w[1796]);
                let noise_metadata_schedule_3152_0_e28716: f64 = (-noise_metadata_schedule_3152_0_e28715);
                let noise_metadata_schedule_3152_0_e28717: f64 = (noise_metadata_schedule_3152_0_e28710 * noise_metadata_schedule_3152_0_e28716);
                let noise_metadata_schedule_3152_0_e28719: f64 = (noise_metadata_schedule_3152_0_e28717 + params.p53);
                let noise_metadata_schedule_3152_0_e28720: f64 = (noise_metadata_schedule_3152_0_e28719).sqrt();
                let noise_metadata_schedule_3152_0_e28721: f64 = (noise_metadata_schedule_3152_0_e28704 + noise_metadata_schedule_3152_0_e28720);
                let noise_metadata_schedule_3152_0_e28722: f64 = (0.5 * noise_metadata_schedule_3152_0_e28721);
                (noise_metadata_schedule_3152_0_e28722,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3152_0_e28724,)
    }
};
            let noise_metadata_schedule_3152_0_e28727: f64 = (noise_metadata_schedule_3152_0_e28725).powf(w[1820]);
            let noise_metadata_schedule_3152_0_e28728: f64 = (1.0 + noise_metadata_schedule_3152_0_e28727);
            let noise_metadata_schedule_3152_0_e28731: f64 = (1.0 / w[1820]);
            let noise_metadata_schedule_3152_0_e28732: f64 = (noise_metadata_schedule_3152_0_e28728).powf(noise_metadata_schedule_3152_0_e28731);
            let noise_metadata_schedule_3152_0_e28733: f64 = (1.0 / noise_metadata_schedule_3152_0_e28732);
            w[1864] = noise_metadata_schedule_3152_0_e28733;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3153_0_e28735: f64 = (-w[1798]);
            let noise_metadata_schedule_3153_0_e28737: f64 = (noise_metadata_schedule_3153_0_e28735 * w[1864]);
            w[1865] = noise_metadata_schedule_3153_0_e28737;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3154_0_e28740: f64 = (w[1797] - w[1895]);
            let noise_metadata_schedule_3154_0_e28742: f64 = (noise_metadata_schedule_3154_0_e28740 / w[1831]);
            w[1894] = noise_metadata_schedule_3154_0_e28742;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3155_0_e28745: f64 = if w[1894] > 50.0 { 1.0 } else { 0.0 };
            w[1905] = noise_metadata_schedule_3155_0_e28745;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3156_0_e28749,) = {
    if (w[1905] != 0.0) {
        (0.0,)
    } else {
        (w[1836],)
    }
};
            w[1836] = noise_metadata_schedule_3156_0_e28749;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3157_0_e28752: f64 = (-50.0);
            let noise_metadata_schedule_3157_0_e28753: f64 = if w[1894] < noise_metadata_schedule_3157_0_e28752 { 1.0 } else { 0.0 };
            w[1906] = noise_metadata_schedule_3157_0_e28753;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3158_0_e28760,) = {
    if ((w[1905] == 0.0) && (w[1906] != 0.0)) {
        (1.0,)
    } else {
        (w[1836],)
    }
};
            w[1836] = noise_metadata_schedule_3158_0_e28760;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3159_0_e28773,) = {
    if ((w[1905] == 0.0) && (w[1906] == 0.0)) {
        let noise_metadata_schedule_3159_0_e28769: f64 = (w[1894]).exp();
        let noise_metadata_schedule_3159_0_e28770: f64 = (1.0 + noise_metadata_schedule_3159_0_e28769);
        let noise_metadata_schedule_3159_0_e28771: f64 = (1.0 / noise_metadata_schedule_3159_0_e28770);
        (noise_metadata_schedule_3159_0_e28771,)
    } else {
        (w[1836],)
    }
};
            w[1836] = noise_metadata_schedule_3159_0_e28773;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3160_0_e28776: f64 = (w[1897] - w[1865]);
            let noise_metadata_schedule_3160_0_e28780: f64 = (params.p51 * 0.1);
            let noise_metadata_schedule_3160_0_e28782: f64 = (noise_metadata_schedule_3160_0_e28780 * w[1831]);
            let noise_metadata_schedule_3160_0_e28784: f64 = (noise_metadata_schedule_3160_0_e28782 * w[1836]);
            let noise_metadata_schedule_3160_0_e28785: f64 = (w[1795] - noise_metadata_schedule_3160_0_e28784);
            let noise_metadata_schedule_3160_0_e28786: f64 = (noise_metadata_schedule_3160_0_e28776 - noise_metadata_schedule_3160_0_e28785);
            let noise_metadata_schedule_3160_0_e28788: f64 = (noise_metadata_schedule_3160_0_e28786 / w[1837]);
            w[1839] = noise_metadata_schedule_3160_0_e28788;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3161_0_e28791: f64 = if w[1839] > 50.0 { 1.0 } else { 0.0 };
            w[1907] = noise_metadata_schedule_3161_0_e28791;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3162_0_e28797,) = {
    if (w[1907] != 0.0) {
        let noise_metadata_schedule_3162_0_e28795: f64 = (w[1838] * w[1839]);
        (noise_metadata_schedule_3162_0_e28795,)
    } else {
        (w[1840],)
    }
};
            w[1840] = noise_metadata_schedule_3162_0_e28797;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3163_0_e28800: f64 = (-50.0);
            let noise_metadata_schedule_3163_0_e28801: f64 = if w[1839] < noise_metadata_schedule_3163_0_e28800 { 1.0 } else { 0.0 };
            w[1908] = noise_metadata_schedule_3163_0_e28801;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3164_0_e28811,) = {
    if ((w[1907] == 0.0) && (w[1908] != 0.0)) {
        let noise_metadata_schedule_3164_0_e28808: f64 = (w[1839]).exp();
        let noise_metadata_schedule_3164_0_e28809: f64 = (w[1838] * noise_metadata_schedule_3164_0_e28808);
        (noise_metadata_schedule_3164_0_e28809,)
    } else {
        (w[1840],)
    }
};
            w[1840] = noise_metadata_schedule_3164_0_e28811;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3165_0_e28825,) = {
    if ((w[1907] == 0.0) && (w[1908] == 0.0)) {
        let noise_metadata_schedule_3165_0_e28820: f64 = (w[1839]).exp();
        let noise_metadata_schedule_3165_0_e28821: f64 = (1.0 + noise_metadata_schedule_3165_0_e28820);
        let noise_metadata_schedule_3165_0_e28822: f64 = (noise_metadata_schedule_3165_0_e28821).ln();
        let noise_metadata_schedule_3165_0_e28823: f64 = (w[1838] * noise_metadata_schedule_3165_0_e28822);
        (noise_metadata_schedule_3165_0_e28823,)
    } else {
        (w[1840],)
    }
};
            w[1840] = noise_metadata_schedule_3165_0_e28825;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_2(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3166_0_e28828: f64 = (w[1897] - w[1895]);
            let noise_metadata_schedule_3166_0_e28830: f64 = (noise_metadata_schedule_3166_0_e28828 / w[1831]);
            w[1894] = noise_metadata_schedule_3166_0_e28830;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3167_0_e28833: f64 = if w[1894] > 50.0 { 1.0 } else { 0.0 };
            w[1909] = noise_metadata_schedule_3167_0_e28833;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3168_0_e28837,) = {
    if (w[1909] != 0.0) {
        (0.0,)
    } else {
        (w[1866],)
    }
};
            w[1866] = noise_metadata_schedule_3168_0_e28837;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3169_0_e28840: f64 = (-50.0);
            let noise_metadata_schedule_3169_0_e28841: f64 = if w[1894] < noise_metadata_schedule_3169_0_e28840 { 1.0 } else { 0.0 };
            w[1910] = noise_metadata_schedule_3169_0_e28841;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3170_0_e28848,) = {
    if ((w[1909] == 0.0) && (w[1910] != 0.0)) {
        (1.0,)
    } else {
        (w[1866],)
    }
};
            w[1866] = noise_metadata_schedule_3170_0_e28848;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3171_0_e28861,) = {
    if ((w[1909] == 0.0) && (w[1910] == 0.0)) {
        let noise_metadata_schedule_3171_0_e28857: f64 = (w[1894]).exp();
        let noise_metadata_schedule_3171_0_e28858: f64 = (1.0 + noise_metadata_schedule_3171_0_e28857);
        let noise_metadata_schedule_3171_0_e28859: f64 = (1.0 / noise_metadata_schedule_3171_0_e28858);
        (noise_metadata_schedule_3171_0_e28859,)
    } else {
        (w[1866],)
    }
};
            w[1866] = noise_metadata_schedule_3171_0_e28861;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3172_0_e28864: f64 = (w[1797] - w[1863]);
            let noise_metadata_schedule_3172_0_e28868: f64 = (params.p51 * 0.1);
            let noise_metadata_schedule_3172_0_e28870: f64 = (noise_metadata_schedule_3172_0_e28868 * w[1831]);
            let noise_metadata_schedule_3172_0_e28872: f64 = (noise_metadata_schedule_3172_0_e28870 * w[1866]);
            let noise_metadata_schedule_3172_0_e28873: f64 = (w[1795] - noise_metadata_schedule_3172_0_e28872);
            let noise_metadata_schedule_3172_0_e28874: f64 = (noise_metadata_schedule_3172_0_e28864 - noise_metadata_schedule_3172_0_e28873);
            let noise_metadata_schedule_3172_0_e28876: f64 = (noise_metadata_schedule_3172_0_e28874 / w[1837]);
            w[1867] = noise_metadata_schedule_3172_0_e28876;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3173_0_e28879: f64 = if w[1867] > 50.0 { 1.0 } else { 0.0 };
            w[1911] = noise_metadata_schedule_3173_0_e28879;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3174_0_e28885,) = {
    if (w[1911] != 0.0) {
        let noise_metadata_schedule_3174_0_e28883: f64 = (w[1838] * w[1867]);
        (noise_metadata_schedule_3174_0_e28883,)
    } else {
        (w[1868],)
    }
};
            w[1868] = noise_metadata_schedule_3174_0_e28885;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3175_0_e28888: f64 = (-50.0);
            let noise_metadata_schedule_3175_0_e28889: f64 = if w[1867] < noise_metadata_schedule_3175_0_e28888 { 1.0 } else { 0.0 };
            w[1912] = noise_metadata_schedule_3175_0_e28889;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3176_0_e28899,) = {
    if ((w[1911] == 0.0) && (w[1912] != 0.0)) {
        let noise_metadata_schedule_3176_0_e28896: f64 = (w[1867]).exp();
        let noise_metadata_schedule_3176_0_e28897: f64 = (w[1838] * noise_metadata_schedule_3176_0_e28896);
        (noise_metadata_schedule_3176_0_e28897,)
    } else {
        (w[1868],)
    }
};
            w[1868] = noise_metadata_schedule_3176_0_e28899;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3177_0_e28913,) = {
    if ((w[1911] == 0.0) && (w[1912] == 0.0)) {
        let noise_metadata_schedule_3177_0_e28908: f64 = (w[1867]).exp();
        let noise_metadata_schedule_3177_0_e28909: f64 = (1.0 + noise_metadata_schedule_3177_0_e28908);
        let noise_metadata_schedule_3177_0_e28910: f64 = (noise_metadata_schedule_3177_0_e28909).ln();
        let noise_metadata_schedule_3177_0_e28911: f64 = (w[1838] * noise_metadata_schedule_3177_0_e28910);
        (noise_metadata_schedule_3177_0_e28911,)
    } else {
        (w[1868],)
    }
};
            w[1868] = noise_metadata_schedule_3177_0_e28913;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3178_0_e28916: f64 = (w[1840] - w[1868]);
            let noise_metadata_schedule_3178_0_e28918: f64 = (noise_metadata_schedule_3178_0_e28916 / w[1808]);
            w[1869] = noise_metadata_schedule_3178_0_e28918;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3179_0_e28921: f64 = (w[1869] / w[1861]);
            w[1895] = noise_metadata_schedule_3179_0_e28921;
        }
        if (active[0] & 0x30) != 0 {
            let (noise_metadata_schedule_3180_0_e28947,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3180_0_e28931: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3180_0_e28933: f64 = (noise_metadata_schedule_3180_0_e28931 * w[1895]);
        let noise_metadata_schedule_3180_0_e28934: f64 = (noise_metadata_schedule_3180_0_e28933).tanh();
        let noise_metadata_schedule_3180_0_e28935: f64 = (w[1895] * noise_metadata_schedule_3180_0_e28934);
        (noise_metadata_schedule_3180_0_e28935,)
    } else {
        let (noise_metadata_schedule_3180_0_e28946,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3180_0_e28941: f64 = (w[1895] * w[1895]);
                let noise_metadata_schedule_3180_0_e28943: f64 = (noise_metadata_schedule_3180_0_e28941 + params.p53);
                let noise_metadata_schedule_3180_0_e28944: f64 = (noise_metadata_schedule_3180_0_e28943).sqrt();
                (noise_metadata_schedule_3180_0_e28944,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3180_0_e28946,)
    }
};
            let noise_metadata_schedule_3180_0_e28949: f64 = (noise_metadata_schedule_3180_0_e28947).powf(w[1820]);
            let noise_metadata_schedule_3180_0_e28950: f64 = (1.0 + noise_metadata_schedule_3180_0_e28949);
            let noise_metadata_schedule_3180_0_e28953: f64 = (1.0 / w[1820]);
            let noise_metadata_schedule_3180_0_e28954: f64 = (noise_metadata_schedule_3180_0_e28950).powf(noise_metadata_schedule_3180_0_e28953);
            let noise_metadata_schedule_3180_0_e28955: f64 = (w[1895] / noise_metadata_schedule_3180_0_e28954);
            w[1870] = noise_metadata_schedule_3180_0_e28955;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3181_0_e28958: f64 = (w[1843] * w[1870]);
            w[1871] = noise_metadata_schedule_3181_0_e28958;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_3182_0_e28961: f64 = (w[1829] * w[1806]);
            let noise_metadata_schedule_3182_0_e28963: f64 = (noise_metadata_schedule_3182_0_e28961 * w[1828]);
            let noise_metadata_schedule_3182_0_e28965: f64 = (noise_metadata_schedule_3182_0_e28963 * 0.5);
            let noise_metadata_schedule_3182_0_e28968: f64 = (w[1840] + w[1868]);
            let noise_metadata_schedule_3182_0_e28969: f64 = (noise_metadata_schedule_3182_0_e28965 * noise_metadata_schedule_3182_0_e28968);
            let noise_metadata_schedule_3182_0_e28971: f64 = (noise_metadata_schedule_3182_0_e28969 * w[1871]);
            let noise_metadata_schedule_3182_0_e28973: f64 = (noise_metadata_schedule_3182_0_e28971 * w[1830]);
            w[1789] = noise_metadata_schedule_3182_0_e28973;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3183_0_e28977: f64 = (2.302585092994046 * w[1805]);
            let noise_metadata_schedule_3183_0_e28978: f64 = (w[1813] / noise_metadata_schedule_3183_0_e28977);
            w[1844] = noise_metadata_schedule_3183_0_e28978;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3184_0_e28981: f64 = (2.0 * w[1844]);
            let noise_metadata_schedule_3184_0_e28983: f64 = (noise_metadata_schedule_3184_0_e28981 * w[1805]);
            w[1846] = noise_metadata_schedule_3184_0_e28983;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3185_0_e28986: f64 = (w[1808] * w[1846]);
            w[1847] = noise_metadata_schedule_3185_0_e28986;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3186_0_e28990: f64 = (params.p51 * w[1831]);
            let noise_metadata_schedule_3186_0_e28992: f64 = (noise_metadata_schedule_3186_0_e28990 / 2.0);
            let noise_metadata_schedule_3186_0_e28993: f64 = (w[1834] - noise_metadata_schedule_3186_0_e28992);
            w[1899] = noise_metadata_schedule_3186_0_e28993;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3187_0_e29037,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3187_0_e29001: f64 = (w[1797] + w[1897]);
        let noise_metadata_schedule_3187_0_e29004: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3187_0_e29007: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3187_0_e29010: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3187_0_e29011: f64 = (noise_metadata_schedule_3187_0_e29007 * noise_metadata_schedule_3187_0_e29010);
        let noise_metadata_schedule_3187_0_e29012: f64 = (noise_metadata_schedule_3187_0_e29011).tanh();
        let noise_metadata_schedule_3187_0_e29013: f64 = (noise_metadata_schedule_3187_0_e29004 * noise_metadata_schedule_3187_0_e29012);
        let noise_metadata_schedule_3187_0_e29014: f64 = (noise_metadata_schedule_3187_0_e29001 + noise_metadata_schedule_3187_0_e29013);
        let noise_metadata_schedule_3187_0_e29015: f64 = (0.5 * noise_metadata_schedule_3187_0_e29014);
        (noise_metadata_schedule_3187_0_e29015,)
    } else {
        let (noise_metadata_schedule_3187_0_e29036,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3187_0_e29022: f64 = (w[1797] + w[1897]);
                let noise_metadata_schedule_3187_0_e29025: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3187_0_e29028: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3187_0_e29029: f64 = (noise_metadata_schedule_3187_0_e29025 * noise_metadata_schedule_3187_0_e29028);
                let noise_metadata_schedule_3187_0_e29031: f64 = (noise_metadata_schedule_3187_0_e29029 + params.p53);
                let noise_metadata_schedule_3187_0_e29032: f64 = (noise_metadata_schedule_3187_0_e29031).sqrt();
                let noise_metadata_schedule_3187_0_e29033: f64 = (noise_metadata_schedule_3187_0_e29022 + noise_metadata_schedule_3187_0_e29032);
                let noise_metadata_schedule_3187_0_e29034: f64 = (0.5 * noise_metadata_schedule_3187_0_e29033);
                (noise_metadata_schedule_3187_0_e29034,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3187_0_e29036,)
    }
};
            let noise_metadata_schedule_3187_0_e29039: f64 = (noise_metadata_schedule_3187_0_e29037 - w[1899]);
            let noise_metadata_schedule_3187_0_e29041: f64 = (noise_metadata_schedule_3187_0_e29039 / w[1831]);
            w[1898] = noise_metadata_schedule_3187_0_e29041;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3188_0_e29044: f64 = if w[1898] > 50.0 { 1.0 } else { 0.0 };
            w[1913] = noise_metadata_schedule_3188_0_e29044;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3189_0_e29048,) = {
    if (w[1913] != 0.0) {
        (0.0,)
    } else {
        (w[1856],)
    }
};
            w[1856] = noise_metadata_schedule_3189_0_e29048;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3190_0_e29051: f64 = (-50.0);
            let noise_metadata_schedule_3190_0_e29052: f64 = if w[1898] < noise_metadata_schedule_3190_0_e29051 { 1.0 } else { 0.0 };
            w[1914] = noise_metadata_schedule_3190_0_e29052;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3191_0_e29059,) = {
    if ((w[1913] == 0.0) && (w[1914] != 0.0)) {
        (1.0,)
    } else {
        (w[1856],)
    }
};
            w[1856] = noise_metadata_schedule_3191_0_e29059;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3192_0_e29072,) = {
    if ((w[1913] == 0.0) && (w[1914] == 0.0)) {
        let noise_metadata_schedule_3192_0_e29068: f64 = (w[1898]).exp();
        let noise_metadata_schedule_3192_0_e29069: f64 = (1.0 + noise_metadata_schedule_3192_0_e29068);
        let noise_metadata_schedule_3192_0_e29070: f64 = (1.0 / noise_metadata_schedule_3192_0_e29069);
        (noise_metadata_schedule_3192_0_e29070,)
    } else {
        (w[1856],)
    }
};
            w[1856] = noise_metadata_schedule_3192_0_e29072;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3193_0_e29116,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3193_0_e29080: f64 = (w[1797] + w[1897]);
        let noise_metadata_schedule_3193_0_e29083: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3193_0_e29086: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3193_0_e29089: f64 = (w[1797] - w[1897]);
        let noise_metadata_schedule_3193_0_e29090: f64 = (noise_metadata_schedule_3193_0_e29086 * noise_metadata_schedule_3193_0_e29089);
        let noise_metadata_schedule_3193_0_e29091: f64 = (noise_metadata_schedule_3193_0_e29090).tanh();
        let noise_metadata_schedule_3193_0_e29092: f64 = (noise_metadata_schedule_3193_0_e29083 * noise_metadata_schedule_3193_0_e29091);
        let noise_metadata_schedule_3193_0_e29093: f64 = (noise_metadata_schedule_3193_0_e29080 + noise_metadata_schedule_3193_0_e29092);
        let noise_metadata_schedule_3193_0_e29094: f64 = (0.5 * noise_metadata_schedule_3193_0_e29093);
        (noise_metadata_schedule_3193_0_e29094,)
    } else {
        let (noise_metadata_schedule_3193_0_e29115,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3193_0_e29101: f64 = (w[1797] + w[1897]);
                let noise_metadata_schedule_3193_0_e29104: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3193_0_e29107: f64 = (w[1797] - w[1897]);
                let noise_metadata_schedule_3193_0_e29108: f64 = (noise_metadata_schedule_3193_0_e29104 * noise_metadata_schedule_3193_0_e29107);
                let noise_metadata_schedule_3193_0_e29110: f64 = (noise_metadata_schedule_3193_0_e29108 + params.p53);
                let noise_metadata_schedule_3193_0_e29111: f64 = (noise_metadata_schedule_3193_0_e29110).sqrt();
                let noise_metadata_schedule_3193_0_e29112: f64 = (noise_metadata_schedule_3193_0_e29101 + noise_metadata_schedule_3193_0_e29111);
                let noise_metadata_schedule_3193_0_e29113: f64 = (0.5 * noise_metadata_schedule_3193_0_e29112);
                (noise_metadata_schedule_3193_0_e29113,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3193_0_e29115,)
    }
};
            let noise_metadata_schedule_3193_0_e29120: f64 = (params.p51 * 0.1);
            let noise_metadata_schedule_3193_0_e29122: f64 = (noise_metadata_schedule_3193_0_e29120 * w[1831]);
            let noise_metadata_schedule_3193_0_e29124: f64 = (noise_metadata_schedule_3193_0_e29122 * w[1856]);
            let noise_metadata_schedule_3193_0_e29125: f64 = (w[1834] - noise_metadata_schedule_3193_0_e29124);
            let noise_metadata_schedule_3193_0_e29126: f64 = (noise_metadata_schedule_3193_0_e29116 - noise_metadata_schedule_3193_0_e29125);
            let noise_metadata_schedule_3193_0_e29128: f64 = (noise_metadata_schedule_3193_0_e29126 / w[1846]);
            w[1857] = noise_metadata_schedule_3193_0_e29128;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3194_0_e29131: f64 = if w[1857] > 50.0 { 1.0 } else { 0.0 };
            w[1915] = noise_metadata_schedule_3194_0_e29131;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3195_0_e29137,) = {
    if (w[1915] != 0.0) {
        let noise_metadata_schedule_3195_0_e29135: f64 = (w[1847] * w[1857]);
        (noise_metadata_schedule_3195_0_e29135,)
    } else {
        (w[1858],)
    }
};
            w[1858] = noise_metadata_schedule_3195_0_e29137;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3196_0_e29140: f64 = (-50.0);
            let noise_metadata_schedule_3196_0_e29141: f64 = if w[1857] < noise_metadata_schedule_3196_0_e29140 { 1.0 } else { 0.0 };
            w[1916] = noise_metadata_schedule_3196_0_e29141;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3197_0_e29151,) = {
    if ((w[1915] == 0.0) && (w[1916] != 0.0)) {
        let noise_metadata_schedule_3197_0_e29148: f64 = (w[1857]).exp();
        let noise_metadata_schedule_3197_0_e29149: f64 = (w[1847] * noise_metadata_schedule_3197_0_e29148);
        (noise_metadata_schedule_3197_0_e29149,)
    } else {
        (w[1858],)
    }
};
            w[1858] = noise_metadata_schedule_3197_0_e29151;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3198_0_e29165,) = {
    if ((w[1915] == 0.0) && (w[1916] == 0.0)) {
        let noise_metadata_schedule_3198_0_e29160: f64 = (w[1857]).exp();
        let noise_metadata_schedule_3198_0_e29161: f64 = (1.0 + noise_metadata_schedule_3198_0_e29160);
        let noise_metadata_schedule_3198_0_e29162: f64 = (noise_metadata_schedule_3198_0_e29161).ln();
        let noise_metadata_schedule_3198_0_e29163: f64 = (w[1847] * noise_metadata_schedule_3198_0_e29162);
        (noise_metadata_schedule_3198_0_e29163,)
    } else {
        (w[1858],)
    }
};
            w[1858] = noise_metadata_schedule_3198_0_e29165;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3199_0_e29168: f64 = (w[1819] / w[1852]);
            w[1850] = noise_metadata_schedule_3199_0_e29168;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3200_0_e29173: f64 = (w[1826] * w[1804]);
            let noise_metadata_schedule_3200_0_e29174: f64 = (1.0 + noise_metadata_schedule_3200_0_e29173);
            let noise_metadata_schedule_3200_0_e29178: f64 = (w[1826] * w[1803]);
            let noise_metadata_schedule_3200_0_e29179: f64 = (1.0 + noise_metadata_schedule_3200_0_e29178);
            let noise_metadata_schedule_3200_0_e29180: f64 = (noise_metadata_schedule_3200_0_e29174 / noise_metadata_schedule_3200_0_e29179);
            let noise_metadata_schedule_3200_0_e29181: f64 = (w[1818] * noise_metadata_schedule_3200_0_e29180);
            w[1851] = noise_metadata_schedule_3200_0_e29181;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3201_0_e29184: f64 = (w[1851] * w[1807]);
            let noise_metadata_schedule_3201_0_e29186: f64 = (noise_metadata_schedule_3201_0_e29184 / w[1850]);
            w[1872] = noise_metadata_schedule_3201_0_e29186;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3202_0_e29191: f64 = (2.0 * w[1858]);
            let noise_metadata_schedule_3202_0_e29193: f64 = (noise_metadata_schedule_3202_0_e29191 / w[1808]);
            let noise_metadata_schedule_3202_0_e29195: f64 = (noise_metadata_schedule_3202_0_e29193 / w[1872]);
            let noise_metadata_schedule_3202_0_e29196: f64 = (1.0 + noise_metadata_schedule_3202_0_e29195);
            let noise_metadata_schedule_3202_0_e29197: f64 = (noise_metadata_schedule_3202_0_e29196).sqrt();
            let noise_metadata_schedule_3202_0_e29198: f64 = (w[1872] * noise_metadata_schedule_3202_0_e29197);
            let noise_metadata_schedule_3202_0_e29200: f64 = (noise_metadata_schedule_3202_0_e29198 - w[1872]);
            w[1873] = noise_metadata_schedule_3202_0_e29200;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3203_0_e29204: f64 = (1.0 - w[1856]);
            let noise_metadata_schedule_3203_0_e29205: f64 = (w[1873] * noise_metadata_schedule_3203_0_e29204);
            let noise_metadata_schedule_3203_0_e29208: f64 = (w[1846] * w[1856]);
            let noise_metadata_schedule_3203_0_e29209: f64 = (noise_metadata_schedule_3203_0_e29205 + noise_metadata_schedule_3203_0_e29208);
            w[1874] = noise_metadata_schedule_3203_0_e29209;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3204_0_e29267,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3204_0_e29220: f64 = (w[1798] / w[1874]);
        let noise_metadata_schedule_3204_0_e29221: f64 = noise_metadata_schedule_3204_0_e29220;
        let noise_metadata_schedule_3204_0_e29225: f64 = (w[1798] / w[1874]);
        let noise_metadata_schedule_3204_0_e29226: f64 = (-noise_metadata_schedule_3204_0_e29225);
        let noise_metadata_schedule_3204_0_e29229: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3204_0_e29233: f64 = (w[1798] / w[1874]);
        let noise_metadata_schedule_3204_0_e29234: f64 = (-noise_metadata_schedule_3204_0_e29233);
        let noise_metadata_schedule_3204_0_e29235: f64 = (noise_metadata_schedule_3204_0_e29229 * noise_metadata_schedule_3204_0_e29234);
        let noise_metadata_schedule_3204_0_e29236: f64 = (noise_metadata_schedule_3204_0_e29235).tanh();
        let noise_metadata_schedule_3204_0_e29237: f64 = (noise_metadata_schedule_3204_0_e29226 * noise_metadata_schedule_3204_0_e29236);
        let noise_metadata_schedule_3204_0_e29238: f64 = (noise_metadata_schedule_3204_0_e29221 + noise_metadata_schedule_3204_0_e29237);
        let noise_metadata_schedule_3204_0_e29239: f64 = (0.5 * noise_metadata_schedule_3204_0_e29238);
        (noise_metadata_schedule_3204_0_e29239,)
    } else {
        let (noise_metadata_schedule_3204_0_e29266,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3204_0_e29247: f64 = (w[1798] / w[1874]);
                let noise_metadata_schedule_3204_0_e29248: f64 = noise_metadata_schedule_3204_0_e29247;
                let noise_metadata_schedule_3204_0_e29252: f64 = (w[1798] / w[1874]);
                let noise_metadata_schedule_3204_0_e29253: f64 = (-noise_metadata_schedule_3204_0_e29252);
                let noise_metadata_schedule_3204_0_e29257: f64 = (w[1798] / w[1874]);
                let noise_metadata_schedule_3204_0_e29258: f64 = (-noise_metadata_schedule_3204_0_e29257);
                let noise_metadata_schedule_3204_0_e29259: f64 = (noise_metadata_schedule_3204_0_e29253 * noise_metadata_schedule_3204_0_e29258);
                let noise_metadata_schedule_3204_0_e29261: f64 = (noise_metadata_schedule_3204_0_e29259 + params.p53);
                let noise_metadata_schedule_3204_0_e29262: f64 = (noise_metadata_schedule_3204_0_e29261).sqrt();
                let noise_metadata_schedule_3204_0_e29263: f64 = (noise_metadata_schedule_3204_0_e29248 + noise_metadata_schedule_3204_0_e29262);
                let noise_metadata_schedule_3204_0_e29264: f64 = (0.5 * noise_metadata_schedule_3204_0_e29263);
                (noise_metadata_schedule_3204_0_e29264,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3204_0_e29266,)
    }
};
            let noise_metadata_schedule_3204_0_e29269: f64 = (noise_metadata_schedule_3204_0_e29267).powf(w[1820]);
            let noise_metadata_schedule_3204_0_e29270: f64 = (1.0 + noise_metadata_schedule_3204_0_e29269);
            let noise_metadata_schedule_3204_0_e29273: f64 = (1.0 / w[1820]);
            let noise_metadata_schedule_3204_0_e29274: f64 = (noise_metadata_schedule_3204_0_e29270).powf(noise_metadata_schedule_3204_0_e29273);
            let noise_metadata_schedule_3204_0_e29275: f64 = (1.0 / noise_metadata_schedule_3204_0_e29274);
            w[1875] = noise_metadata_schedule_3204_0_e29275;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3205_0_e29278: f64 = (w[1798] * w[1875]);
            w[1876] = noise_metadata_schedule_3205_0_e29278;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3206_0_e29342,) = {
    if (params.p52 != 0.0) {
        let noise_metadata_schedule_3206_0_e29288: f64 = (-w[1798]);
        let noise_metadata_schedule_3206_0_e29290: f64 = (noise_metadata_schedule_3206_0_e29288 / w[1874]);
        let noise_metadata_schedule_3206_0_e29291: f64 = noise_metadata_schedule_3206_0_e29290;
        let noise_metadata_schedule_3206_0_e29294: f64 = (-w[1798]);
        let noise_metadata_schedule_3206_0_e29296: f64 = (noise_metadata_schedule_3206_0_e29294 / w[1874]);
        let noise_metadata_schedule_3206_0_e29297: f64 = (-noise_metadata_schedule_3206_0_e29296);
        let noise_metadata_schedule_3206_0_e29300: f64 = (0.001 / params.p53);
        let noise_metadata_schedule_3206_0_e29303: f64 = (-w[1798]);
        let noise_metadata_schedule_3206_0_e29305: f64 = (noise_metadata_schedule_3206_0_e29303 / w[1874]);
        let noise_metadata_schedule_3206_0_e29306: f64 = (-noise_metadata_schedule_3206_0_e29305);
        let noise_metadata_schedule_3206_0_e29307: f64 = (noise_metadata_schedule_3206_0_e29300 * noise_metadata_schedule_3206_0_e29306);
        let noise_metadata_schedule_3206_0_e29308: f64 = (noise_metadata_schedule_3206_0_e29307).tanh();
        let noise_metadata_schedule_3206_0_e29309: f64 = (noise_metadata_schedule_3206_0_e29297 * noise_metadata_schedule_3206_0_e29308);
        let noise_metadata_schedule_3206_0_e29310: f64 = (noise_metadata_schedule_3206_0_e29291 + noise_metadata_schedule_3206_0_e29309);
        let noise_metadata_schedule_3206_0_e29311: f64 = (0.5 * noise_metadata_schedule_3206_0_e29310);
        (noise_metadata_schedule_3206_0_e29311,)
    } else {
        let (noise_metadata_schedule_3206_0_e29341,) = {
            if (params.p52 == 0.0) {
                let noise_metadata_schedule_3206_0_e29318: f64 = (-w[1798]);
                let noise_metadata_schedule_3206_0_e29320: f64 = (noise_metadata_schedule_3206_0_e29318 / w[1874]);
                let noise_metadata_schedule_3206_0_e29321: f64 = noise_metadata_schedule_3206_0_e29320;
                let noise_metadata_schedule_3206_0_e29324: f64 = (-w[1798]);
                let noise_metadata_schedule_3206_0_e29326: f64 = (noise_metadata_schedule_3206_0_e29324 / w[1874]);
                let noise_metadata_schedule_3206_0_e29327: f64 = (-noise_metadata_schedule_3206_0_e29326);
                let noise_metadata_schedule_3206_0_e29330: f64 = (-w[1798]);
                let noise_metadata_schedule_3206_0_e29332: f64 = (noise_metadata_schedule_3206_0_e29330 / w[1874]);
                let noise_metadata_schedule_3206_0_e29333: f64 = (-noise_metadata_schedule_3206_0_e29332);
                let noise_metadata_schedule_3206_0_e29334: f64 = (noise_metadata_schedule_3206_0_e29327 * noise_metadata_schedule_3206_0_e29333);
                let noise_metadata_schedule_3206_0_e29336: f64 = (noise_metadata_schedule_3206_0_e29334 + params.p53);
                let noise_metadata_schedule_3206_0_e29337: f64 = (noise_metadata_schedule_3206_0_e29336).sqrt();
                let noise_metadata_schedule_3206_0_e29338: f64 = (noise_metadata_schedule_3206_0_e29321 + noise_metadata_schedule_3206_0_e29337);
                let noise_metadata_schedule_3206_0_e29339: f64 = (0.5 * noise_metadata_schedule_3206_0_e29338);
                (noise_metadata_schedule_3206_0_e29339,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_3206_0_e29341,)
    }
};
            let noise_metadata_schedule_3206_0_e29344: f64 = (noise_metadata_schedule_3206_0_e29342).powf(w[1820]);
            let noise_metadata_schedule_3206_0_e29345: f64 = (1.0 + noise_metadata_schedule_3206_0_e29344);
            let noise_metadata_schedule_3206_0_e29348: f64 = (1.0 / w[1820]);
            let noise_metadata_schedule_3206_0_e29349: f64 = (noise_metadata_schedule_3206_0_e29345).powf(noise_metadata_schedule_3206_0_e29348);
            let noise_metadata_schedule_3206_0_e29350: f64 = (1.0 / noise_metadata_schedule_3206_0_e29349);
            w[1877] = noise_metadata_schedule_3206_0_e29350;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3207_0_e29352: f64 = (-w[1798]);
            let noise_metadata_schedule_3207_0_e29354: f64 = (noise_metadata_schedule_3207_0_e29352 * w[1877]);
            w[1878] = noise_metadata_schedule_3207_0_e29354;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3208_0_e29357: f64 = (w[1797] - w[1899]);
            let noise_metadata_schedule_3208_0_e29359: f64 = (noise_metadata_schedule_3208_0_e29357 / w[1831]);
            w[1898] = noise_metadata_schedule_3208_0_e29359;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3209_0_e29362: f64 = if w[1898] > 50.0 { 1.0 } else { 0.0 };
            w[1917] = noise_metadata_schedule_3209_0_e29362;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3210_0_e29366,) = {
    if (w[1917] != 0.0) {
        (0.0,)
    } else {
        (w[1845],)
    }
};
            w[1845] = noise_metadata_schedule_3210_0_e29366;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3211_0_e29369: f64 = (-50.0);
            let noise_metadata_schedule_3211_0_e29370: f64 = if w[1898] < noise_metadata_schedule_3211_0_e29369 { 1.0 } else { 0.0 };
            w[1918] = noise_metadata_schedule_3211_0_e29370;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_3(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3212_0_e29377,) = {
    if ((w[1917] == 0.0) && (w[1918] != 0.0)) {
        (1.0,)
    } else {
        (w[1845],)
    }
};
            w[1845] = noise_metadata_schedule_3212_0_e29377;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3213_0_e29390,) = {
    if ((w[1917] == 0.0) && (w[1918] == 0.0)) {
        let noise_metadata_schedule_3213_0_e29386: f64 = (w[1898]).exp();
        let noise_metadata_schedule_3213_0_e29387: f64 = (1.0 + noise_metadata_schedule_3213_0_e29386);
        let noise_metadata_schedule_3213_0_e29388: f64 = (1.0 / noise_metadata_schedule_3213_0_e29387);
        (noise_metadata_schedule_3213_0_e29388,)
    } else {
        (w[1845],)
    }
};
            w[1845] = noise_metadata_schedule_3213_0_e29390;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3214_0_e29393: f64 = (w[1897] - w[1878]);
            let noise_metadata_schedule_3214_0_e29397: f64 = (params.p51 * 0.1);
            let noise_metadata_schedule_3214_0_e29399: f64 = (noise_metadata_schedule_3214_0_e29397 * w[1831]);
            let noise_metadata_schedule_3214_0_e29401: f64 = (noise_metadata_schedule_3214_0_e29399 * w[1845]);
            let noise_metadata_schedule_3214_0_e29402: f64 = (w[1834] - noise_metadata_schedule_3214_0_e29401);
            let noise_metadata_schedule_3214_0_e29403: f64 = (noise_metadata_schedule_3214_0_e29393 - noise_metadata_schedule_3214_0_e29402);
            let noise_metadata_schedule_3214_0_e29405: f64 = (noise_metadata_schedule_3214_0_e29403 / w[1846]);
            w[1848] = noise_metadata_schedule_3214_0_e29405;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3215_0_e29408: f64 = if w[1848] > 50.0 { 1.0 } else { 0.0 };
            w[1919] = noise_metadata_schedule_3215_0_e29408;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3216_0_e29414,) = {
    if (w[1919] != 0.0) {
        let noise_metadata_schedule_3216_0_e29412: f64 = (w[1847] * w[1848]);
        (noise_metadata_schedule_3216_0_e29412,)
    } else {
        (w[1849],)
    }
};
            w[1849] = noise_metadata_schedule_3216_0_e29414;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3217_0_e29417: f64 = (-50.0);
            let noise_metadata_schedule_3217_0_e29418: f64 = if w[1848] < noise_metadata_schedule_3217_0_e29417 { 1.0 } else { 0.0 };
            w[1920] = noise_metadata_schedule_3217_0_e29418;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3218_0_e29428,) = {
    if ((w[1919] == 0.0) && (w[1920] != 0.0)) {
        let noise_metadata_schedule_3218_0_e29425: f64 = (w[1848]).exp();
        let noise_metadata_schedule_3218_0_e29426: f64 = (w[1847] * noise_metadata_schedule_3218_0_e29425);
        (noise_metadata_schedule_3218_0_e29426,)
    } else {
        (w[1849],)
    }
};
            w[1849] = noise_metadata_schedule_3218_0_e29428;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3219_0_e29442,) = {
    if ((w[1919] == 0.0) && (w[1920] == 0.0)) {
        let noise_metadata_schedule_3219_0_e29437: f64 = (w[1848]).exp();
        let noise_metadata_schedule_3219_0_e29438: f64 = (1.0 + noise_metadata_schedule_3219_0_e29437);
        let noise_metadata_schedule_3219_0_e29439: f64 = (noise_metadata_schedule_3219_0_e29438).ln();
        let noise_metadata_schedule_3219_0_e29440: f64 = (w[1847] * noise_metadata_schedule_3219_0_e29439);
        (noise_metadata_schedule_3219_0_e29440,)
    } else {
        (w[1849],)
    }
};
            w[1849] = noise_metadata_schedule_3219_0_e29442;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3220_0_e29445: f64 = (w[1897] - w[1899]);
            let noise_metadata_schedule_3220_0_e29447: f64 = (noise_metadata_schedule_3220_0_e29445 / w[1831]);
            w[1898] = noise_metadata_schedule_3220_0_e29447;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3221_0_e29450: f64 = if w[1898] > 50.0 { 1.0 } else { 0.0 };
            w[1921] = noise_metadata_schedule_3221_0_e29450;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3222_0_e29454,) = {
    if (w[1921] != 0.0) {
        (0.0,)
    } else {
        (w[1879],)
    }
};
            w[1879] = noise_metadata_schedule_3222_0_e29454;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3223_0_e29457: f64 = (-50.0);
            let noise_metadata_schedule_3223_0_e29458: f64 = if w[1898] < noise_metadata_schedule_3223_0_e29457 { 1.0 } else { 0.0 };
            w[1922] = noise_metadata_schedule_3223_0_e29458;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3224_0_e29465,) = {
    if ((w[1921] == 0.0) && (w[1922] != 0.0)) {
        (1.0,)
    } else {
        (w[1879],)
    }
};
            w[1879] = noise_metadata_schedule_3224_0_e29465;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3225_0_e29478,) = {
    if ((w[1921] == 0.0) && (w[1922] == 0.0)) {
        let noise_metadata_schedule_3225_0_e29474: f64 = (w[1898]).exp();
        let noise_metadata_schedule_3225_0_e29475: f64 = (1.0 + noise_metadata_schedule_3225_0_e29474);
        let noise_metadata_schedule_3225_0_e29476: f64 = (1.0 / noise_metadata_schedule_3225_0_e29475);
        (noise_metadata_schedule_3225_0_e29476,)
    } else {
        (w[1879],)
    }
};
            w[1879] = noise_metadata_schedule_3225_0_e29478;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3226_0_e29481: f64 = (w[1797] - w[1876]);
            let noise_metadata_schedule_3226_0_e29485: f64 = (params.p51 * 0.1);
            let noise_metadata_schedule_3226_0_e29487: f64 = (noise_metadata_schedule_3226_0_e29485 * w[1831]);
            let noise_metadata_schedule_3226_0_e29489: f64 = (noise_metadata_schedule_3226_0_e29487 * w[1879]);
            let noise_metadata_schedule_3226_0_e29490: f64 = (w[1834] - noise_metadata_schedule_3226_0_e29489);
            let noise_metadata_schedule_3226_0_e29491: f64 = (noise_metadata_schedule_3226_0_e29481 - noise_metadata_schedule_3226_0_e29490);
            let noise_metadata_schedule_3226_0_e29493: f64 = (noise_metadata_schedule_3226_0_e29491 / w[1846]);
            w[1880] = noise_metadata_schedule_3226_0_e29493;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3227_0_e29496: f64 = if w[1880] > 50.0 { 1.0 } else { 0.0 };
            w[1923] = noise_metadata_schedule_3227_0_e29496;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3228_0_e29502,) = {
    if (w[1923] != 0.0) {
        let noise_metadata_schedule_3228_0_e29500: f64 = (w[1847] * w[1880]);
        (noise_metadata_schedule_3228_0_e29500,)
    } else {
        (w[1881],)
    }
};
            w[1881] = noise_metadata_schedule_3228_0_e29502;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3229_0_e29505: f64 = (-50.0);
            let noise_metadata_schedule_3229_0_e29506: f64 = if w[1880] < noise_metadata_schedule_3229_0_e29505 { 1.0 } else { 0.0 };
            w[1924] = noise_metadata_schedule_3229_0_e29506;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3230_0_e29516,) = {
    if ((w[1923] == 0.0) && (w[1924] != 0.0)) {
        let noise_metadata_schedule_3230_0_e29513: f64 = (w[1880]).exp();
        let noise_metadata_schedule_3230_0_e29514: f64 = (w[1847] * noise_metadata_schedule_3230_0_e29513);
        (noise_metadata_schedule_3230_0_e29514,)
    } else {
        (w[1881],)
    }
};
            w[1881] = noise_metadata_schedule_3230_0_e29516;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_3231_0_e29530,) = {
    if ((w[1923] == 0.0) && (w[1924] == 0.0)) {
        let noise_metadata_schedule_3231_0_e29525: f64 = (w[1880]).exp();
        let noise_metadata_schedule_3231_0_e29526: f64 = (1.0 + noise_metadata_schedule_3231_0_e29525);
        let noise_metadata_schedule_3231_0_e29527: f64 = (noise_metadata_schedule_3231_0_e29526).ln();
        let noise_metadata_schedule_3231_0_e29528: f64 = (w[1847] * noise_metadata_schedule_3231_0_e29527);
        (noise_metadata_schedule_3231_0_e29528,)
    } else {
        (w[1881],)
    }
};
            w[1881] = noise_metadata_schedule_3231_0_e29530;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3232_0_e29533: f64 = (w[1849] * w[1849]);
            let noise_metadata_schedule_3232_0_e29535: f64 = (noise_metadata_schedule_3232_0_e29533 + 1e-38);
            w[1882] = noise_metadata_schedule_3232_0_e29535;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3233_0_e29538: f64 = (w[1882] * w[1849]);
            let noise_metadata_schedule_3233_0_e29540: f64 = (noise_metadata_schedule_3233_0_e29538 + 1e-57);
            w[1883] = noise_metadata_schedule_3233_0_e29540;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3234_0_e29543: f64 = (w[1881] * w[1881]);
            let noise_metadata_schedule_3234_0_e29545: f64 = (noise_metadata_schedule_3234_0_e29543 + 1e-38);
            w[1884] = noise_metadata_schedule_3234_0_e29545;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3235_0_e29548: f64 = (w[1884] * w[1881]);
            let noise_metadata_schedule_3235_0_e29550: f64 = (noise_metadata_schedule_3235_0_e29548 + 1e-57);
            w[1885] = noise_metadata_schedule_3235_0_e29550;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3236_0_e29553: f64 = (w[1849] * w[1881]);
            let noise_metadata_schedule_3236_0_e29555: f64 = (noise_metadata_schedule_3236_0_e29553 + 1e-38);
            w[1886] = noise_metadata_schedule_3236_0_e29555;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3237_0_e29558: f64 = (2.0 / 3.0);
            let noise_metadata_schedule_3237_0_e29561: f64 = (w[1882] + w[1884]);
            let noise_metadata_schedule_3237_0_e29563: f64 = (noise_metadata_schedule_3237_0_e29561 + w[1886]);
            let noise_metadata_schedule_3237_0_e29564: f64 = (noise_metadata_schedule_3237_0_e29558 * noise_metadata_schedule_3237_0_e29563);
            let noise_metadata_schedule_3237_0_e29567: f64 = (w[1849] + w[1881]);
            let noise_metadata_schedule_3237_0_e29569: f64 = (noise_metadata_schedule_3237_0_e29567 + 2e-19);
            let noise_metadata_schedule_3237_0_e29570: f64 = (noise_metadata_schedule_3237_0_e29564 / noise_metadata_schedule_3237_0_e29569);
            w[1887] = noise_metadata_schedule_3237_0_e29570;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3238_0_e29574: f64 = (2.0 * w[1883]);
            let noise_metadata_schedule_3238_0_e29577: f64 = (3.0 * w[1885]);
            let noise_metadata_schedule_3238_0_e29578: f64 = (noise_metadata_schedule_3238_0_e29574 + noise_metadata_schedule_3238_0_e29577);
            let noise_metadata_schedule_3238_0_e29581: f64 = (4.0 * w[1882]);
            let noise_metadata_schedule_3238_0_e29583: f64 = (noise_metadata_schedule_3238_0_e29581 * w[1881]);
            let noise_metadata_schedule_3238_0_e29584: f64 = (noise_metadata_schedule_3238_0_e29578 + noise_metadata_schedule_3238_0_e29583);
            let noise_metadata_schedule_3238_0_e29587: f64 = (6.0 * w[1884]);
            let noise_metadata_schedule_3238_0_e29589: f64 = (noise_metadata_schedule_3238_0_e29587 * w[1849]);
            let noise_metadata_schedule_3238_0_e29590: f64 = (noise_metadata_schedule_3238_0_e29584 + noise_metadata_schedule_3238_0_e29589);
            let noise_metadata_schedule_3238_0_e29591: f64 = (2.0 * noise_metadata_schedule_3238_0_e29590);
            let noise_metadata_schedule_3238_0_e29595: f64 = (w[1882] + w[1884]);
            let noise_metadata_schedule_3238_0_e29598: f64 = (2.0 * w[1886]);
            let noise_metadata_schedule_3238_0_e29599: f64 = (noise_metadata_schedule_3238_0_e29595 + noise_metadata_schedule_3238_0_e29598);
            let noise_metadata_schedule_3238_0_e29600: f64 = (15.0 * noise_metadata_schedule_3238_0_e29599);
            let noise_metadata_schedule_3238_0_e29601: f64 = (noise_metadata_schedule_3238_0_e29591 / noise_metadata_schedule_3238_0_e29600);
            w[1888] = noise_metadata_schedule_3238_0_e29601;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3239_0_e29604: f64 = (w[1887] - w[1888]);
            w[1889] = noise_metadata_schedule_3239_0_e29604;
        }
        if (active[0] & 0x20) != 0 {
            w[1890] = w[1888];
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3241_0_e29608: f64 = (w[1806] * w[1828]);
            let noise_metadata_schedule_3241_0_e29610: f64 = (noise_metadata_schedule_3241_0_e29608 * w[1807]);
            let noise_metadata_schedule_3241_0_e29612: f64 = (noise_metadata_schedule_3241_0_e29610 * w[1829]);
            let noise_metadata_schedule_3241_0_e29614: f64 = (noise_metadata_schedule_3241_0_e29612 * w[1889]);
            let noise_metadata_schedule_3241_0_e29616: f64 = (noise_metadata_schedule_3241_0_e29614 * w[1830]);
            w[1790] = noise_metadata_schedule_3241_0_e29616;
        }
        if (active[0] & 0x20) != 0 {
            let noise_metadata_schedule_3242_0_e29619: f64 = (w[1806] * w[1828]);
            let noise_metadata_schedule_3242_0_e29621: f64 = (noise_metadata_schedule_3242_0_e29619 * w[1807]);
            let noise_metadata_schedule_3242_0_e29623: f64 = (noise_metadata_schedule_3242_0_e29621 * w[1829]);
            let noise_metadata_schedule_3242_0_e29625: f64 = (noise_metadata_schedule_3242_0_e29623 * w[1890]);
            let noise_metadata_schedule_3242_0_e29627: f64 = (noise_metadata_schedule_3242_0_e29625 * w[1830]);
            w[1791] = noise_metadata_schedule_3242_0_e29627;
        }
        if (active[0] & 0x30) != 0 {
            w[1788] = w[1789];
        }
        if (active[0] & 0x20) != 0 {
            w[117] = w[1790];
        }
        if (active[0] & 0x20) != 0 {
            w[118] = w[1791];
        }
        if (active[0] & 0x30) != 0 {
            w[115] = w[1788];
        }
        if (active[0] & 0x4) != 0 {
            w[122] = 0.0;
        }
        if (active[0] & 0x8) != 0 {
            w[123] = 0.0;
        }
        if (active[0] & 0x4) != 0 {
            w[124] = 0.0;
        }
        if (active[0] & 0x8) != 0 {
            w[125] = 0.0;
        }
        if (active[0] & 0x4) != 0 {
            w[126] = 0.0;
        }
        if (active[0] & 0x8) != 0 {
            w[127] = 0.0;
        }
        if (active[0] & 0x1) != 0 {
            w[128] = 0.0;
        }
        if (active[0] & 0x2) != 0 {
            w[129] = 0.0;
        }
        if (active[0] & 0x1) != 0 {
            w[130] = 0.0;
        }
        if (active[0] & 0x2) != 0 {
            w[131] = 0.0;
        }
        if (active[0] & 0x1) != 0 {
            w[132] = 0.0;
        }
        if (active[0] & 0x2) != 0 {
            w[133] = 0.0;
        }
        if (active[0] & 0xf) != 0 {
            let noise_metadata_schedule_3305_0_e29890: f64 = if params.p254 == 1.0 { 1.0 } else { 0.0 };
            w[1934] = noise_metadata_schedule_3305_0_e29890;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3306_0_e29894,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1935],)
    }
};
            w[1935] = noise_metadata_schedule_3306_0_e29894;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3307_0_e29898,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1936],)
    }
};
            w[1936] = noise_metadata_schedule_3307_0_e29898;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3308_0_e29902,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1937],)
    }
};
            w[1937] = noise_metadata_schedule_3308_0_e29902;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3309_0_e29908,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3309_0_e29906: f64 = (params.p6 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[13])));
        (noise_metadata_schedule_3309_0_e29906,)
    } else {
        (w[1938],)
    }
};
            w[1938] = noise_metadata_schedule_3309_0_e29908;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3310_0_e29912,) = {
    if (w[1934] != 0.0) {
        (w[113],)
    } else {
        (w[1939],)
    }
};
            w[1939] = noise_metadata_schedule_3310_0_e29912;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3311_0_e29916,) = {
    if (w[1934] != 0.0) {
        (params.p260,)
    } else {
        (w[1940],)
    }
};
            w[1940] = noise_metadata_schedule_3311_0_e29916;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3312_0_e29920,) = {
    if (w[1934] != 0.0) {
        (params.p262,)
    } else {
        (w[1941],)
    }
};
            w[1941] = noise_metadata_schedule_3312_0_e29920;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3313_0_e29924,) = {
    if (w[1934] != 0.0) {
        (params.p261,)
    } else {
        (w[1942],)
    }
};
            w[1942] = noise_metadata_schedule_3313_0_e29924;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3314_0_e29928,) = {
    if (w[1934] != 0.0) {
        (params.p258,)
    } else {
        (w[1943],)
    }
};
            w[1943] = noise_metadata_schedule_3314_0_e29928;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3315_0_e29932,) = {
    if (w[1934] != 0.0) {
        (params.p278,)
    } else {
        (w[1944],)
    }
};
            w[1944] = noise_metadata_schedule_3315_0_e29932;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3316_0_e29936,) = {
    if (w[1934] != 0.0) {
        (params.p277,)
    } else {
        (w[1945],)
    }
};
            w[1945] = noise_metadata_schedule_3316_0_e29936;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3317_0_e29940,) = {
    if (w[1934] != 0.0) {
        (w[112],)
    } else {
        (w[1946],)
    }
};
            w[1946] = noise_metadata_schedule_3317_0_e29940;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3318_0_e29944,) = {
    if (w[1934] != 0.0) {
        (params.p0,)
    } else {
        (w[1947],)
    }
};
            w[1947] = noise_metadata_schedule_3318_0_e29944;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3319_0_e29948,) = {
    if (w[1934] != 0.0) {
        (params.p2,)
    } else {
        (w[1948],)
    }
};
            w[1948] = noise_metadata_schedule_3319_0_e29948;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3320_0_e29956,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3320_0_e29952: f64 = (1.0 - params.p255);
        let noise_metadata_schedule_3320_0_e29954: f64 = (noise_metadata_schedule_3320_0_e29952 * params.p259);
        (noise_metadata_schedule_3320_0_e29954,)
    } else {
        (w[1949],)
    }
};
            w[1949] = noise_metadata_schedule_3320_0_e29956;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3321_0_e29960,) = {
    if (w[1934] != 0.0) {
        (params.p276,)
    } else {
        (w[1950],)
    }
};
            w[1950] = noise_metadata_schedule_3321_0_e29960;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3322_0_e29964,) = {
    if (w[1934] != 0.0) {
        (params.p270,)
    } else {
        (w[1951],)
    }
};
            w[1951] = noise_metadata_schedule_3322_0_e29964;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3323_0_e29968,) = {
    if (w[1934] != 0.0) {
        (params.p271,)
    } else {
        (w[1952],)
    }
};
            w[1952] = noise_metadata_schedule_3323_0_e29968;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3324_0_e29976,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3324_0_e29972: f64 = (1.0 - params.p255);
        let noise_metadata_schedule_3324_0_e29974: f64 = (noise_metadata_schedule_3324_0_e29972 * params.p269);
        (noise_metadata_schedule_3324_0_e29974,)
    } else {
        (w[1953],)
    }
};
            w[1953] = noise_metadata_schedule_3324_0_e29976;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_4(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3325_0_e29980,) = {
    if (w[1934] != 0.0) {
        (params.p268,)
    } else {
        (w[1954],)
    }
};
            w[1954] = noise_metadata_schedule_3325_0_e29980;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3326_0_e29984,) = {
    if (w[1934] != 0.0) {
        (params.p257,)
    } else {
        (w[1955],)
    }
};
            w[1955] = noise_metadata_schedule_3326_0_e29984;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3327_0_e29988,) = {
    if (w[1934] != 0.0) {
        (params.p256,)
    } else {
        (w[1956],)
    }
};
            w[1956] = noise_metadata_schedule_3327_0_e29988;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3328_0_e29992,) = {
    if (w[1934] != 0.0) {
        (params.p6,)
    } else {
        (w[1957],)
    }
};
            w[1957] = noise_metadata_schedule_3328_0_e29992;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3329_0_e29996,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1958],)
    }
};
            w[1958] = noise_metadata_schedule_3329_0_e29996;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3330_0_e30000,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1959],)
    }
};
            w[1959] = noise_metadata_schedule_3330_0_e30000;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3331_0_e30004,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1960],)
    }
};
            w[1960] = noise_metadata_schedule_3331_0_e30004;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3332_0_e30008,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1961],)
    }
};
            w[1961] = noise_metadata_schedule_3332_0_e30008;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3333_0_e30012,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1962],)
    }
};
            w[1962] = noise_metadata_schedule_3333_0_e30012;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3334_0_e30016,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1963],)
    }
};
            w[1963] = noise_metadata_schedule_3334_0_e30016;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3335_0_e30020,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1964],)
    }
};
            w[1964] = noise_metadata_schedule_3335_0_e30020;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3336_0_e30024,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1965],)
    }
};
            w[1965] = noise_metadata_schedule_3336_0_e30024;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3337_0_e30028,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1966],)
    }
};
            w[1966] = noise_metadata_schedule_3337_0_e30028;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3338_0_e30032,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1967],)
    }
};
            w[1967] = noise_metadata_schedule_3338_0_e30032;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3339_0_e30036,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1968],)
    }
};
            w[1968] = noise_metadata_schedule_3339_0_e30036;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3340_0_e30040,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1969],)
    }
};
            w[1969] = noise_metadata_schedule_3340_0_e30040;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3341_0_e30044,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1970],)
    }
};
            w[1970] = noise_metadata_schedule_3341_0_e30044;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3342_0_e30048,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1971],)
    }
};
            w[1971] = noise_metadata_schedule_3342_0_e30048;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3343_0_e30052,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1972],)
    }
};
            w[1972] = noise_metadata_schedule_3343_0_e30052;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3344_0_e30056,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1973],)
    }
};
            w[1973] = noise_metadata_schedule_3344_0_e30056;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3345_0_e30060,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1974],)
    }
};
            w[1974] = noise_metadata_schedule_3345_0_e30060;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3346_0_e30064,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1975],)
    }
};
            w[1975] = noise_metadata_schedule_3346_0_e30064;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3347_0_e30068,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1976],)
    }
};
            w[1976] = noise_metadata_schedule_3347_0_e30068;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3348_0_e30072,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1977],)
    }
};
            w[1977] = noise_metadata_schedule_3348_0_e30072;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3349_0_e30076,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1978],)
    }
};
            w[1978] = noise_metadata_schedule_3349_0_e30076;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3350_0_e30080,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1979],)
    }
};
            w[1979] = noise_metadata_schedule_3350_0_e30080;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3351_0_e30084,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1980],)
    }
};
            w[1980] = noise_metadata_schedule_3351_0_e30084;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3352_0_e30088,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1981],)
    }
};
            w[1981] = noise_metadata_schedule_3352_0_e30088;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3353_0_e30092,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1982],)
    }
};
            w[1982] = noise_metadata_schedule_3353_0_e30092;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3354_0_e30096,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1983],)
    }
};
            w[1983] = noise_metadata_schedule_3354_0_e30096;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3355_0_e30100,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1984],)
    }
};
            w[1984] = noise_metadata_schedule_3355_0_e30100;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3356_0_e30104,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1985],)
    }
};
            w[1985] = noise_metadata_schedule_3356_0_e30104;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3357_0_e30108,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1986],)
    }
};
            w[1986] = noise_metadata_schedule_3357_0_e30108;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3358_0_e30112,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1987],)
    }
};
            w[1987] = noise_metadata_schedule_3358_0_e30112;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3359_0_e30116,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1988],)
    }
};
            w[1988] = noise_metadata_schedule_3359_0_e30116;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3360_0_e30120,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1989],)
    }
};
            w[1989] = noise_metadata_schedule_3360_0_e30120;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3361_0_e30124,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1990],)
    }
};
            w[1990] = noise_metadata_schedule_3361_0_e30124;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3362_0_e30133,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3362_0_e30128: f64 = (w[1955] / w[1939]);
        let noise_metadata_schedule_3362_0_e30130: f64 = (-w[1956]);
        let noise_metadata_schedule_3362_0_e30131: f64 = (noise_metadata_schedule_3362_0_e30128 * noise_metadata_schedule_3362_0_e30130);
        (noise_metadata_schedule_3362_0_e30131,)
    } else {
        (w[1970],)
    }
};
            w[1970] = noise_metadata_schedule_3362_0_e30133;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3363_0_e30175,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3363_0_e30141: f64 = (-50.0);
        let (noise_metadata_schedule_3363_0_e30173,) = {
            if ((!(w[1970] > 50.0)) && (!(w[1970] < noise_metadata_schedule_3363_0_e30141))) {
                let noise_metadata_schedule_3363_0_e30146: f64 = (w[1970]).exp();
                (noise_metadata_schedule_3363_0_e30146,)
            } else {
                let noise_metadata_schedule_3363_0_e30153: f64 = (-50.0);
                let (noise_metadata_schedule_3363_0_e30172,) = {
                    if ((!(w[1970] > 50.0)) && (w[1970] < noise_metadata_schedule_3363_0_e30153)) {
                        let noise_metadata_schedule_3363_0_e30157: f64 = (-50.0);
                        let noise_metadata_schedule_3363_0_e30158: f64 = (noise_metadata_schedule_3363_0_e30157).exp();
                        (noise_metadata_schedule_3363_0_e30158,)
                    } else {
                        let (noise_metadata_schedule_3363_0_e30171,) = {
                            if (w[1970] > 50.0) {
                                let noise_metadata_schedule_3363_0_e30163: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3363_0_e30167: f64 = (w[1970] - 50.0);
                                let noise_metadata_schedule_3363_0_e30168: f64 = (1.0 + noise_metadata_schedule_3363_0_e30167);
                                let noise_metadata_schedule_3363_0_e30169: f64 = (noise_metadata_schedule_3363_0_e30163 * noise_metadata_schedule_3363_0_e30168);
                                (noise_metadata_schedule_3363_0_e30169,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3363_0_e30171,)
                    }
                };
                (noise_metadata_schedule_3363_0_e30172,)
            }
        };
        (noise_metadata_schedule_3363_0_e30173,)
    } else {
        (w[1960],)
    }
};
            w[1960] = noise_metadata_schedule_3363_0_e30175;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3364_0_e30186,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3364_0_e30179: f64 = (-w[1938]);
        let noise_metadata_schedule_3364_0_e30181: f64 = (noise_metadata_schedule_3364_0_e30179 - w[1945]);
        let noise_metadata_schedule_3364_0_e30182: f64 = (w[1944] * noise_metadata_schedule_3364_0_e30181);
        let noise_metadata_schedule_3364_0_e30184: f64 = (noise_metadata_schedule_3364_0_e30182 + w[1970]);
        (noise_metadata_schedule_3364_0_e30184,)
    } else {
        (w[1966],)
    }
};
            w[1966] = noise_metadata_schedule_3364_0_e30186;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3365_0_e30195,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3365_0_e30189: f64 = (-w[1944]);
        let noise_metadata_schedule_3365_0_e30191: f64 = (noise_metadata_schedule_3365_0_e30189 * w[1945]);
        let noise_metadata_schedule_3365_0_e30193: f64 = (noise_metadata_schedule_3365_0_e30191 + w[1970]);
        (noise_metadata_schedule_3365_0_e30193,)
    } else {
        (w[1967],)
    }
};
            w[1967] = noise_metadata_schedule_3365_0_e30195;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3366_0_e30237,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3366_0_e30203: f64 = (-50.0);
        let (noise_metadata_schedule_3366_0_e30235,) = {
            if ((!(w[1966] > 50.0)) && (!(w[1966] < noise_metadata_schedule_3366_0_e30203))) {
                let noise_metadata_schedule_3366_0_e30208: f64 = (w[1966]).exp();
                (noise_metadata_schedule_3366_0_e30208,)
            } else {
                let noise_metadata_schedule_3366_0_e30215: f64 = (-50.0);
                let (noise_metadata_schedule_3366_0_e30234,) = {
                    if ((!(w[1966] > 50.0)) && (w[1966] < noise_metadata_schedule_3366_0_e30215)) {
                        let noise_metadata_schedule_3366_0_e30219: f64 = (-50.0);
                        let noise_metadata_schedule_3366_0_e30220: f64 = (noise_metadata_schedule_3366_0_e30219).exp();
                        (noise_metadata_schedule_3366_0_e30220,)
                    } else {
                        let (noise_metadata_schedule_3366_0_e30233,) = {
                            if (w[1966] > 50.0) {
                                let noise_metadata_schedule_3366_0_e30225: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3366_0_e30229: f64 = (w[1966] - 50.0);
                                let noise_metadata_schedule_3366_0_e30230: f64 = (1.0 + noise_metadata_schedule_3366_0_e30229);
                                let noise_metadata_schedule_3366_0_e30231: f64 = (noise_metadata_schedule_3366_0_e30225 * noise_metadata_schedule_3366_0_e30230);
                                (noise_metadata_schedule_3366_0_e30231,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3366_0_e30233,)
                    }
                };
                (noise_metadata_schedule_3366_0_e30234,)
            }
        };
        (noise_metadata_schedule_3366_0_e30235,)
    } else {
        (w[1968],)
    }
};
            w[1968] = noise_metadata_schedule_3366_0_e30237;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_5(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3367_0_e30279,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3367_0_e30245: f64 = (-50.0);
        let (noise_metadata_schedule_3367_0_e30277,) = {
            if ((!(w[1967] > 50.0)) && (!(w[1967] < noise_metadata_schedule_3367_0_e30245))) {
                let noise_metadata_schedule_3367_0_e30250: f64 = (w[1967]).exp();
                (noise_metadata_schedule_3367_0_e30250,)
            } else {
                let noise_metadata_schedule_3367_0_e30257: f64 = (-50.0);
                let (noise_metadata_schedule_3367_0_e30276,) = {
                    if ((!(w[1967] > 50.0)) && (w[1967] < noise_metadata_schedule_3367_0_e30257)) {
                        let noise_metadata_schedule_3367_0_e30261: f64 = (-50.0);
                        let noise_metadata_schedule_3367_0_e30262: f64 = (noise_metadata_schedule_3367_0_e30261).exp();
                        (noise_metadata_schedule_3367_0_e30262,)
                    } else {
                        let (noise_metadata_schedule_3367_0_e30275,) = {
                            if (w[1967] > 50.0) {
                                let noise_metadata_schedule_3367_0_e30267: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3367_0_e30271: f64 = (w[1967] - 50.0);
                                let noise_metadata_schedule_3367_0_e30272: f64 = (1.0 + noise_metadata_schedule_3367_0_e30271);
                                let noise_metadata_schedule_3367_0_e30273: f64 = (noise_metadata_schedule_3367_0_e30267 * noise_metadata_schedule_3367_0_e30272);
                                (noise_metadata_schedule_3367_0_e30273,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3367_0_e30275,)
                    }
                };
                (noise_metadata_schedule_3367_0_e30276,)
            }
        };
        (noise_metadata_schedule_3367_0_e30277,)
    } else {
        (w[1969],)
    }
};
            w[1969] = noise_metadata_schedule_3367_0_e30279;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3368_0_e30285,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3368_0_e30283: f64 = (w[1968] - w[1969]);
        (noise_metadata_schedule_3368_0_e30283,)
    } else {
        (w[1962],)
    }
};
            w[1962] = noise_metadata_schedule_3368_0_e30285;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3369_0_e30297,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3369_0_e30289: f64 = (w[1957] * w[1947]);
        let noise_metadata_schedule_3369_0_e30291: f64 = (noise_metadata_schedule_3369_0_e30289 * w[1948]);
        let noise_metadata_schedule_3369_0_e30293: f64 = (noise_metadata_schedule_3369_0_e30291 * w[1949]);
        let noise_metadata_schedule_3369_0_e30295: f64 = (noise_metadata_schedule_3369_0_e30293 * w[1946]);
        (noise_metadata_schedule_3369_0_e30295,)
    } else {
        (w[1936],)
    }
};
            w[1936] = noise_metadata_schedule_3369_0_e30297;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3370_0_e30307,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3370_0_e30301: f64 = (w[1943] / w[1939]);
        let noise_metadata_schedule_3370_0_e30303: f64 = (noise_metadata_schedule_3370_0_e30301 * w[1938]);
        let noise_metadata_schedule_3370_0_e30305: f64 = (noise_metadata_schedule_3370_0_e30303 + w[1970]);
        (noise_metadata_schedule_3370_0_e30305,)
    } else {
        (w[1972],)
    }
};
            w[1972] = noise_metadata_schedule_3370_0_e30307;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3371_0_e30349,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3371_0_e30315: f64 = (-50.0);
        let (noise_metadata_schedule_3371_0_e30347,) = {
            if ((!(w[1972] > 50.0)) && (!(w[1972] < noise_metadata_schedule_3371_0_e30315))) {
                let noise_metadata_schedule_3371_0_e30320: f64 = (w[1972]).exp();
                (noise_metadata_schedule_3371_0_e30320,)
            } else {
                let noise_metadata_schedule_3371_0_e30327: f64 = (-50.0);
                let (noise_metadata_schedule_3371_0_e30346,) = {
                    if ((!(w[1972] > 50.0)) && (w[1972] < noise_metadata_schedule_3371_0_e30327)) {
                        let noise_metadata_schedule_3371_0_e30331: f64 = (-50.0);
                        let noise_metadata_schedule_3371_0_e30332: f64 = (noise_metadata_schedule_3371_0_e30331).exp();
                        (noise_metadata_schedule_3371_0_e30332,)
                    } else {
                        let (noise_metadata_schedule_3371_0_e30345,) = {
                            if (w[1972] > 50.0) {
                                let noise_metadata_schedule_3371_0_e30337: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3371_0_e30341: f64 = (w[1972] - 50.0);
                                let noise_metadata_schedule_3371_0_e30342: f64 = (1.0 + noise_metadata_schedule_3371_0_e30341);
                                let noise_metadata_schedule_3371_0_e30343: f64 = (noise_metadata_schedule_3371_0_e30337 * noise_metadata_schedule_3371_0_e30342);
                                (noise_metadata_schedule_3371_0_e30343,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3371_0_e30345,)
                    }
                };
                (noise_metadata_schedule_3371_0_e30346,)
            }
        };
        (noise_metadata_schedule_3371_0_e30347,)
    } else {
        (w[1973],)
    }
};
            w[1973] = noise_metadata_schedule_3371_0_e30349;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_3372_0_e30352: f64 = if w[1942] == 1.0 { 1.0 } else { 0.0 };
            w[1991] = noise_metadata_schedule_3372_0_e30352;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3373_0_e30366,) = {
    if ((w[1934] != 0.0) && (w[1991] != 0.0)) {
        let noise_metadata_schedule_3373_0_e30360: f64 = (w[1950] * w[1962]);
        let noise_metadata_schedule_3373_0_e30361: f64 = (w[1973] - noise_metadata_schedule_3373_0_e30360);
        let noise_metadata_schedule_3373_0_e30363: f64 = (noise_metadata_schedule_3373_0_e30361 - w[1960]);
        let noise_metadata_schedule_3373_0_e30364: f64 = (w[1936] * noise_metadata_schedule_3373_0_e30363);
        (noise_metadata_schedule_3373_0_e30364,)
    } else {
        (w[1963],)
    }
};
            w[1963] = noise_metadata_schedule_3373_0_e30366;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3374_0_e30380,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3374_0_e30373: f64 = (-w[1940]);
        let noise_metadata_schedule_3374_0_e30375: f64 = (noise_metadata_schedule_3374_0_e30373 - w[1945]);
        let noise_metadata_schedule_3374_0_e30376: f64 = (w[1944] * noise_metadata_schedule_3374_0_e30375);
        let noise_metadata_schedule_3374_0_e30378: f64 = (noise_metadata_schedule_3374_0_e30376 + w[1970]);
        (noise_metadata_schedule_3374_0_e30378,)
    } else {
        (w[1977],)
    }
};
            w[1977] = noise_metadata_schedule_3374_0_e30380;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3375_0_e30425,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3375_0_e30391: f64 = (-50.0);
        let (noise_metadata_schedule_3375_0_e30423,) = {
            if ((!(w[1977] > 50.0)) && (!(w[1977] < noise_metadata_schedule_3375_0_e30391))) {
                let noise_metadata_schedule_3375_0_e30396: f64 = (w[1977]).exp();
                (noise_metadata_schedule_3375_0_e30396,)
            } else {
                let noise_metadata_schedule_3375_0_e30403: f64 = (-50.0);
                let (noise_metadata_schedule_3375_0_e30422,) = {
                    if ((!(w[1977] > 50.0)) && (w[1977] < noise_metadata_schedule_3375_0_e30403)) {
                        let noise_metadata_schedule_3375_0_e30407: f64 = (-50.0);
                        let noise_metadata_schedule_3375_0_e30408: f64 = (noise_metadata_schedule_3375_0_e30407).exp();
                        (noise_metadata_schedule_3375_0_e30408,)
                    } else {
                        let (noise_metadata_schedule_3375_0_e30421,) = {
                            if (w[1977] > 50.0) {
                                let noise_metadata_schedule_3375_0_e30413: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3375_0_e30417: f64 = (w[1977] - 50.0);
                                let noise_metadata_schedule_3375_0_e30418: f64 = (1.0 + noise_metadata_schedule_3375_0_e30417);
                                let noise_metadata_schedule_3375_0_e30419: f64 = (noise_metadata_schedule_3375_0_e30413 * noise_metadata_schedule_3375_0_e30418);
                                (noise_metadata_schedule_3375_0_e30419,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3375_0_e30421,)
                    }
                };
                (noise_metadata_schedule_3375_0_e30422,)
            }
        };
        (noise_metadata_schedule_3375_0_e30423,)
    } else {
        (w[1978],)
    }
};
            w[1978] = noise_metadata_schedule_3375_0_e30425;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3376_0_e30434,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3376_0_e30432: f64 = (w[1978] - w[1969]);
        (noise_metadata_schedule_3376_0_e30432,)
    } else {
        (w[1979],)
    }
};
            w[1979] = noise_metadata_schedule_3376_0_e30434;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3377_0_e30447,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3377_0_e30441: f64 = (w[1943] / w[1939]);
        let noise_metadata_schedule_3377_0_e30443: f64 = (noise_metadata_schedule_3377_0_e30441 * w[1940]);
        let noise_metadata_schedule_3377_0_e30445: f64 = (noise_metadata_schedule_3377_0_e30443 + w[1970]);
        (noise_metadata_schedule_3377_0_e30445,)
    } else {
        (w[1980],)
    }
};
            w[1980] = noise_metadata_schedule_3377_0_e30447;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3378_0_e30492,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3378_0_e30458: f64 = (-50.0);
        let (noise_metadata_schedule_3378_0_e30490,) = {
            if ((!(w[1980] > 50.0)) && (!(w[1980] < noise_metadata_schedule_3378_0_e30458))) {
                let noise_metadata_schedule_3378_0_e30463: f64 = (w[1980]).exp();
                (noise_metadata_schedule_3378_0_e30463,)
            } else {
                let noise_metadata_schedule_3378_0_e30470: f64 = (-50.0);
                let (noise_metadata_schedule_3378_0_e30489,) = {
                    if ((!(w[1980] > 50.0)) && (w[1980] < noise_metadata_schedule_3378_0_e30470)) {
                        let noise_metadata_schedule_3378_0_e30474: f64 = (-50.0);
                        let noise_metadata_schedule_3378_0_e30475: f64 = (noise_metadata_schedule_3378_0_e30474).exp();
                        (noise_metadata_schedule_3378_0_e30475,)
                    } else {
                        let (noise_metadata_schedule_3378_0_e30488,) = {
                            if (w[1980] > 50.0) {
                                let noise_metadata_schedule_3378_0_e30480: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3378_0_e30484: f64 = (w[1980] - 50.0);
                                let noise_metadata_schedule_3378_0_e30485: f64 = (1.0 + noise_metadata_schedule_3378_0_e30484);
                                let noise_metadata_schedule_3378_0_e30486: f64 = (noise_metadata_schedule_3378_0_e30480 * noise_metadata_schedule_3378_0_e30485);
                                (noise_metadata_schedule_3378_0_e30486,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3378_0_e30488,)
                    }
                };
                (noise_metadata_schedule_3378_0_e30489,)
            }
        };
        (noise_metadata_schedule_3378_0_e30490,)
    } else {
        (w[1981],)
    }
};
            w[1981] = noise_metadata_schedule_3378_0_e30492;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3379_0_e30505,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3379_0_e30500: f64 = (w[1950] * w[1979]);
        let noise_metadata_schedule_3379_0_e30501: f64 = (w[1981] - noise_metadata_schedule_3379_0_e30500);
        let noise_metadata_schedule_3379_0_e30503: f64 = (noise_metadata_schedule_3379_0_e30501 - w[1960]);
        (noise_metadata_schedule_3379_0_e30503,)
    } else {
        (w[1982],)
    }
};
            w[1982] = noise_metadata_schedule_3379_0_e30505;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3380_0_e30520,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3380_0_e30514: f64 = (w[1950] * w[1962]);
        let noise_metadata_schedule_3380_0_e30515: f64 = (w[1973] - noise_metadata_schedule_3380_0_e30514);
        let noise_metadata_schedule_3380_0_e30517: f64 = (noise_metadata_schedule_3380_0_e30515 - w[1960]);
        let noise_metadata_schedule_3380_0_e30518: f64 = (w[1936] * noise_metadata_schedule_3380_0_e30517);
        (noise_metadata_schedule_3380_0_e30518,)
    } else {
        (w[1983],)
    }
};
            w[1983] = noise_metadata_schedule_3380_0_e30520;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_3381_0_e30523: f64 = if w[1942] > 0.0 { 1.0 } else { 0.0 };
            w[1992] = noise_metadata_schedule_3381_0_e30523;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3382_0_e30534,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3382_0_e30532: f64 = (w[1942] * w[1943]);
        (noise_metadata_schedule_3382_0_e30532,)
    } else {
        (w[1976],)
    }
};
            w[1976] = noise_metadata_schedule_3382_0_e30534;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3383_0_e30549,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3383_0_e30543: f64 = (w[1976] / w[1939]);
        let noise_metadata_schedule_3383_0_e30545: f64 = (noise_metadata_schedule_3383_0_e30543 * w[1940]);
        let noise_metadata_schedule_3383_0_e30547: f64 = (noise_metadata_schedule_3383_0_e30545 + w[1970]);
        (noise_metadata_schedule_3383_0_e30547,)
    } else {
        (w[1984],)
    }
};
            w[1984] = noise_metadata_schedule_3383_0_e30549;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3384_0_e30596,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3384_0_e30562: f64 = (-50.0);
        let (noise_metadata_schedule_3384_0_e30594,) = {
            if ((!(w[1984] > 50.0)) && (!(w[1984] < noise_metadata_schedule_3384_0_e30562))) {
                let noise_metadata_schedule_3384_0_e30567: f64 = (w[1984]).exp();
                (noise_metadata_schedule_3384_0_e30567,)
            } else {
                let noise_metadata_schedule_3384_0_e30574: f64 = (-50.0);
                let (noise_metadata_schedule_3384_0_e30593,) = {
                    if ((!(w[1984] > 50.0)) && (w[1984] < noise_metadata_schedule_3384_0_e30574)) {
                        let noise_metadata_schedule_3384_0_e30578: f64 = (-50.0);
                        let noise_metadata_schedule_3384_0_e30579: f64 = (noise_metadata_schedule_3384_0_e30578).exp();
                        (noise_metadata_schedule_3384_0_e30579,)
                    } else {
                        let (noise_metadata_schedule_3384_0_e30592,) = {
                            if (w[1984] > 50.0) {
                                let noise_metadata_schedule_3384_0_e30584: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3384_0_e30588: f64 = (w[1984] - 50.0);
                                let noise_metadata_schedule_3384_0_e30589: f64 = (1.0 + noise_metadata_schedule_3384_0_e30588);
                                let noise_metadata_schedule_3384_0_e30590: f64 = (noise_metadata_schedule_3384_0_e30584 * noise_metadata_schedule_3384_0_e30589);
                                (noise_metadata_schedule_3384_0_e30590,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3384_0_e30592,)
                    }
                };
                (noise_metadata_schedule_3384_0_e30593,)
            }
        };
        (noise_metadata_schedule_3384_0_e30594,)
    } else {
        (w[1985],)
    }
};
            w[1985] = noise_metadata_schedule_3384_0_e30596;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3385_0_e30611,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3385_0_e30606: f64 = (w[1950] * w[1979]);
        let noise_metadata_schedule_3385_0_e30607: f64 = (w[1985] - noise_metadata_schedule_3385_0_e30606);
        let noise_metadata_schedule_3385_0_e30609: f64 = (noise_metadata_schedule_3385_0_e30607 - w[1960]);
        (noise_metadata_schedule_3385_0_e30609,)
    } else {
        (w[1986],)
    }
};
            w[1986] = noise_metadata_schedule_3385_0_e30611;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3386_0_e30626,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3386_0_e30620: f64 = (w[1976] / w[1939]);
        let noise_metadata_schedule_3386_0_e30622: f64 = (noise_metadata_schedule_3386_0_e30620 * w[1938]);
        let noise_metadata_schedule_3386_0_e30624: f64 = (noise_metadata_schedule_3386_0_e30622 + w[1970]);
        (noise_metadata_schedule_3386_0_e30624,)
    } else {
        (w[1987],)
    }
};
            w[1987] = noise_metadata_schedule_3386_0_e30626;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3387_0_e30673,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3387_0_e30639: f64 = (-50.0);
        let (noise_metadata_schedule_3387_0_e30671,) = {
            if ((!(w[1987] > 50.0)) && (!(w[1987] < noise_metadata_schedule_3387_0_e30639))) {
                let noise_metadata_schedule_3387_0_e30644: f64 = (w[1987]).exp();
                (noise_metadata_schedule_3387_0_e30644,)
            } else {
                let noise_metadata_schedule_3387_0_e30651: f64 = (-50.0);
                let (noise_metadata_schedule_3387_0_e30670,) = {
                    if ((!(w[1987] > 50.0)) && (w[1987] < noise_metadata_schedule_3387_0_e30651)) {
                        let noise_metadata_schedule_3387_0_e30655: f64 = (-50.0);
                        let noise_metadata_schedule_3387_0_e30656: f64 = (noise_metadata_schedule_3387_0_e30655).exp();
                        (noise_metadata_schedule_3387_0_e30656,)
                    } else {
                        let (noise_metadata_schedule_3387_0_e30669,) = {
                            if (w[1987] > 50.0) {
                                let noise_metadata_schedule_3387_0_e30661: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3387_0_e30665: f64 = (w[1987] - 50.0);
                                let noise_metadata_schedule_3387_0_e30666: f64 = (1.0 + noise_metadata_schedule_3387_0_e30665);
                                let noise_metadata_schedule_3387_0_e30667: f64 = (noise_metadata_schedule_3387_0_e30661 * noise_metadata_schedule_3387_0_e30666);
                                (noise_metadata_schedule_3387_0_e30667,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3387_0_e30669,)
                    }
                };
                (noise_metadata_schedule_3387_0_e30670,)
            }
        };
        (noise_metadata_schedule_3387_0_e30671,)
    } else {
        (w[1988],)
    }
};
            w[1988] = noise_metadata_schedule_3387_0_e30673;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3388_0_e30686,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3388_0_e30682: f64 = (w[1936] * w[1982]);
        let noise_metadata_schedule_3388_0_e30684: f64 = (noise_metadata_schedule_3388_0_e30682 / w[1986]);
        (noise_metadata_schedule_3388_0_e30684,)
    } else {
        (w[1989],)
    }
};
            w[1989] = noise_metadata_schedule_3388_0_e30686;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3389_0_e30703,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] != 0.0)) {
        let noise_metadata_schedule_3389_0_e30697: f64 = (w[1950] * w[1962]);
        let noise_metadata_schedule_3389_0_e30698: f64 = (w[1988] - noise_metadata_schedule_3389_0_e30697);
        let noise_metadata_schedule_3389_0_e30700: f64 = (noise_metadata_schedule_3389_0_e30698 - w[1960]);
        let noise_metadata_schedule_3389_0_e30701: f64 = (w[1989] * noise_metadata_schedule_3389_0_e30700);
        (noise_metadata_schedule_3389_0_e30701,)
    } else {
        (w[1990],)
    }
};
            w[1990] = noise_metadata_schedule_3389_0_e30703;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3390_0_e30715,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1992] == 0.0)) {
        let noise_metadata_schedule_3390_0_e30713: f64 = (w[1936] * w[1982]);
        (noise_metadata_schedule_3390_0_e30713,)
    } else {
        (w[1990],)
    }
};
            w[1990] = noise_metadata_schedule_3390_0_e30715;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3391_0_e30726,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3391_0_e30722: f64 = (w[1941] * w[1941]);
        let noise_metadata_schedule_3391_0_e30724: f64 = (noise_metadata_schedule_3391_0_e30722 * w[1939]);
        (noise_metadata_schedule_3391_0_e30724,)
    } else {
        (w[1959],)
    }
};
            w[1959] = noise_metadata_schedule_3391_0_e30726;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3392_0_e30741,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3392_0_e30735: f64 = (w[1959] / 2.0);
        let noise_metadata_schedule_3392_0_e30736: f64 = (w[1940] - noise_metadata_schedule_3392_0_e30735);
        let noise_metadata_schedule_3392_0_e30737: f64 = (w[1938] - noise_metadata_schedule_3392_0_e30736);
        let noise_metadata_schedule_3392_0_e30739: f64 = (noise_metadata_schedule_3392_0_e30737 / w[1959]);
        (noise_metadata_schedule_3392_0_e30739,)
    } else {
        (w[1971],)
    }
};
            w[1971] = noise_metadata_schedule_3392_0_e30741;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_3393_0_e30744: f64 = if w[1971] > 50.0 { 1.0 } else { 0.0 };
            w[1993] = noise_metadata_schedule_3393_0_e30744;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3394_0_e30753,) = {
    if (((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1993] != 0.0)) {
        (0.0,)
    } else {
        (w[1961],)
    }
};
            w[1961] = noise_metadata_schedule_3394_0_e30753;
        }
        if (active[0] & 0x4) != 0 {
            let noise_metadata_schedule_3395_0_e30756: f64 = (-50.0);
            let noise_metadata_schedule_3395_0_e30757: f64 = if w[1971] < noise_metadata_schedule_3395_0_e30756 { 1.0 } else { 0.0 };
            w[1994] = noise_metadata_schedule_3395_0_e30757;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3396_0_e30769,) = {
    if ((((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1993] == 0.0)) && (w[1994] != 0.0)) {
        (1.0,)
    } else {
        (w[1961],)
    }
};
            w[1961] = noise_metadata_schedule_3396_0_e30769;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_6(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3397_0_e30787,) = {
    if ((((w[1934] != 0.0) && (w[1991] == 0.0)) && (w[1993] == 0.0)) && (w[1994] == 0.0)) {
        let noise_metadata_schedule_3397_0_e30783: f64 = (w[1971]).exp();
        let noise_metadata_schedule_3397_0_e30784: f64 = (1.0 + noise_metadata_schedule_3397_0_e30783);
        let noise_metadata_schedule_3397_0_e30785: f64 = (1.0 / noise_metadata_schedule_3397_0_e30784);
        (noise_metadata_schedule_3397_0_e30785,)
    } else {
        (w[1961],)
    }
};
            w[1961] = noise_metadata_schedule_3397_0_e30787;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3398_0_e30802,) = {
    if ((w[1934] != 0.0) && (w[1991] == 0.0)) {
        let noise_metadata_schedule_3398_0_e30794: f64 = (w[1961] * w[1983]);
        let noise_metadata_schedule_3398_0_e30797: f64 = (1.0 - w[1961]);
        let noise_metadata_schedule_3398_0_e30799: f64 = (noise_metadata_schedule_3398_0_e30797 * w[1990]);
        let noise_metadata_schedule_3398_0_e30800: f64 = (noise_metadata_schedule_3398_0_e30794 + noise_metadata_schedule_3398_0_e30799);
        (noise_metadata_schedule_3398_0_e30800,)
    } else {
        (w[1963],)
    }
};
            w[1963] = noise_metadata_schedule_3398_0_e30802;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3399_0_e30848,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3399_0_e30805: f64 = (-w[1938]);
        let (noise_metadata_schedule_3399_0_e30838,) = {
            if (params.p52 != 0.0) {
                let noise_metadata_schedule_3399_0_e30813: f64 = (w[1938] / w[1951]);
                let noise_metadata_schedule_3399_0_e30816: f64 = (0.001 / params.p53);
                let noise_metadata_schedule_3399_0_e30819: f64 = (w[1938] / w[1951]);
                let noise_metadata_schedule_3399_0_e30820: f64 = (noise_metadata_schedule_3399_0_e30816 * noise_metadata_schedule_3399_0_e30819);
                let noise_metadata_schedule_3399_0_e30821: f64 = (noise_metadata_schedule_3399_0_e30820).tanh();
                let noise_metadata_schedule_3399_0_e30822: f64 = (noise_metadata_schedule_3399_0_e30813 * noise_metadata_schedule_3399_0_e30821);
                (noise_metadata_schedule_3399_0_e30822,)
            } else {
                let (noise_metadata_schedule_3399_0_e30837,) = {
                    if (params.p52 == 0.0) {
                        let noise_metadata_schedule_3399_0_e30828: f64 = (w[1938] / w[1951]);
                        let noise_metadata_schedule_3399_0_e30831: f64 = (w[1938] / w[1951]);
                        let noise_metadata_schedule_3399_0_e30832: f64 = (noise_metadata_schedule_3399_0_e30828 * noise_metadata_schedule_3399_0_e30831);
                        let noise_metadata_schedule_3399_0_e30834: f64 = (noise_metadata_schedule_3399_0_e30832 + params.p53);
                        let noise_metadata_schedule_3399_0_e30835: f64 = (noise_metadata_schedule_3399_0_e30834).sqrt();
                        (noise_metadata_schedule_3399_0_e30835,)
                    } else {
                        (0.0,)
                    }
                };
                (noise_metadata_schedule_3399_0_e30837,)
            }
        };
        let noise_metadata_schedule_3399_0_e30840: f64 = (noise_metadata_schedule_3399_0_e30838).powf(w[1952]);
        let noise_metadata_schedule_3399_0_e30841: f64 = (1.0 + noise_metadata_schedule_3399_0_e30840);
        let noise_metadata_schedule_3399_0_e30844: f64 = (1.0 / w[1952]);
        let noise_metadata_schedule_3399_0_e30845: f64 = (noise_metadata_schedule_3399_0_e30841).powf(noise_metadata_schedule_3399_0_e30844);
        let noise_metadata_schedule_3399_0_e30846: f64 = (noise_metadata_schedule_3399_0_e30805 / noise_metadata_schedule_3399_0_e30845);
        (noise_metadata_schedule_3399_0_e30846,)
    } else {
        (w[1964],)
    }
};
            w[1964] = noise_metadata_schedule_3399_0_e30848;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3400_0_e30863,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3400_0_e30851: f64 = (-w[1957]);
        let noise_metadata_schedule_3400_0_e30853: f64 = (noise_metadata_schedule_3400_0_e30851 * w[1947]);
        let noise_metadata_schedule_3400_0_e30855: f64 = (noise_metadata_schedule_3400_0_e30853 * w[1948]);
        let noise_metadata_schedule_3400_0_e30857: f64 = (noise_metadata_schedule_3400_0_e30855 * w[1953]);
        let noise_metadata_schedule_3400_0_e30859: f64 = (noise_metadata_schedule_3400_0_e30857 * w[1946]);
        let noise_metadata_schedule_3400_0_e30861: f64 = noise_metadata_schedule_3400_0_e30859;
        (noise_metadata_schedule_3400_0_e30861,)
    } else {
        (w[1937],)
    }
};
            w[1937] = noise_metadata_schedule_3400_0_e30863;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3401_0_e30871,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3401_0_e30867: f64 = (w[1954] / w[1939]);
        let noise_metadata_schedule_3401_0_e30869: f64 = (noise_metadata_schedule_3401_0_e30867 * w[1964]);
        (noise_metadata_schedule_3401_0_e30869,)
    } else {
        (w[1974],)
    }
};
            w[1974] = noise_metadata_schedule_3401_0_e30871;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3402_0_e30913,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3402_0_e30879: f64 = (-50.0);
        let (noise_metadata_schedule_3402_0_e30911,) = {
            if ((!(w[1974] > 50.0)) && (!(w[1974] < noise_metadata_schedule_3402_0_e30879))) {
                let noise_metadata_schedule_3402_0_e30884: f64 = (w[1974]).exp();
                (noise_metadata_schedule_3402_0_e30884,)
            } else {
                let noise_metadata_schedule_3402_0_e30891: f64 = (-50.0);
                let (noise_metadata_schedule_3402_0_e30910,) = {
                    if ((!(w[1974] > 50.0)) && (w[1974] < noise_metadata_schedule_3402_0_e30891)) {
                        let noise_metadata_schedule_3402_0_e30895: f64 = (-50.0);
                        let noise_metadata_schedule_3402_0_e30896: f64 = (noise_metadata_schedule_3402_0_e30895).exp();
                        (noise_metadata_schedule_3402_0_e30896,)
                    } else {
                        let (noise_metadata_schedule_3402_0_e30909,) = {
                            if (w[1974] > 50.0) {
                                let noise_metadata_schedule_3402_0_e30901: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3402_0_e30905: f64 = (w[1974] - 50.0);
                                let noise_metadata_schedule_3402_0_e30906: f64 = (1.0 + noise_metadata_schedule_3402_0_e30905);
                                let noise_metadata_schedule_3402_0_e30907: f64 = (noise_metadata_schedule_3402_0_e30901 * noise_metadata_schedule_3402_0_e30906);
                                (noise_metadata_schedule_3402_0_e30907,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3402_0_e30909,)
                    }
                };
                (noise_metadata_schedule_3402_0_e30910,)
            }
        };
        (noise_metadata_schedule_3402_0_e30911,)
    } else {
        (w[1975],)
    }
};
            w[1975] = noise_metadata_schedule_3402_0_e30913;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3403_0_e30921,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3403_0_e30918: f64 = (w[1975] - 1.0);
        let noise_metadata_schedule_3403_0_e30919: f64 = (w[1937] * noise_metadata_schedule_3403_0_e30918);
        (noise_metadata_schedule_3403_0_e30919,)
    } else {
        (w[1965],)
    }
};
            w[1965] = noise_metadata_schedule_3403_0_e30921;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3404_0_e30927,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3404_0_e30925: f64 = (w[1963] + w[1965]);
        (noise_metadata_schedule_3404_0_e30925,)
    } else {
        (w[1958],)
    }
};
            w[1958] = noise_metadata_schedule_3404_0_e30927;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3405_0_e30931,) = {
    if (w[1934] != 0.0) {
        (w[1958],)
    } else {
        (w[1935],)
    }
};
            w[1935] = noise_metadata_schedule_3405_0_e30931;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3406_0_e30935,) = {
    if (w[1934] != 0.0) {
        (w[1936],)
    } else {
        (w[124],)
    }
};
            w[124] = noise_metadata_schedule_3406_0_e30935;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3407_0_e30939,) = {
    if (w[1934] != 0.0) {
        (w[1937],)
    } else {
        (w[126],)
    }
};
            w[126] = noise_metadata_schedule_3407_0_e30939;
        }
        if (active[0] & 0x4) != 0 {
            let (noise_metadata_schedule_3408_0_e30943,) = {
    if (w[1934] != 0.0) {
        (w[1935],)
    } else {
        (w[122],)
    }
};
            w[122] = noise_metadata_schedule_3408_0_e30943;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3409_0_e30947,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1995],)
    }
};
            w[1995] = noise_metadata_schedule_3409_0_e30947;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3410_0_e30951,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1996],)
    }
};
            w[1996] = noise_metadata_schedule_3410_0_e30951;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3411_0_e30955,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[1997],)
    }
};
            w[1997] = noise_metadata_schedule_3411_0_e30955;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3412_0_e30961,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3412_0_e30959: f64 = (params.p6 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[17])));
        (noise_metadata_schedule_3412_0_e30959,)
    } else {
        (w[1998],)
    }
};
            w[1998] = noise_metadata_schedule_3412_0_e30961;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3413_0_e30965,) = {
    if (w[1934] != 0.0) {
        (w[113],)
    } else {
        (w[1999],)
    }
};
            w[1999] = noise_metadata_schedule_3413_0_e30965;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3414_0_e30969,) = {
    if (w[1934] != 0.0) {
        (params.p265,)
    } else {
        (w[2000],)
    }
};
            w[2000] = noise_metadata_schedule_3414_0_e30969;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3415_0_e30973,) = {
    if (w[1934] != 0.0) {
        (params.p267,)
    } else {
        (w[2001],)
    }
};
            w[2001] = noise_metadata_schedule_3415_0_e30973;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3416_0_e30977,) = {
    if (w[1934] != 0.0) {
        (params.p266,)
    } else {
        (w[2002],)
    }
};
            w[2002] = noise_metadata_schedule_3416_0_e30977;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3417_0_e30981,) = {
    if (w[1934] != 0.0) {
        (params.p263,)
    } else {
        (w[2003],)
    }
};
            w[2003] = noise_metadata_schedule_3417_0_e30981;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3418_0_e30985,) = {
    if (w[1934] != 0.0) {
        (params.p281,)
    } else {
        (w[2004],)
    }
};
            w[2004] = noise_metadata_schedule_3418_0_e30985;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3419_0_e30989,) = {
    if (w[1934] != 0.0) {
        (params.p280,)
    } else {
        (w[2005],)
    }
};
            w[2005] = noise_metadata_schedule_3419_0_e30989;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3420_0_e30993,) = {
    if (w[1934] != 0.0) {
        (w[112],)
    } else {
        (w[2006],)
    }
};
            w[2006] = noise_metadata_schedule_3420_0_e30993;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3421_0_e30997,) = {
    if (w[1934] != 0.0) {
        (params.p0,)
    } else {
        (w[2007],)
    }
};
            w[2007] = noise_metadata_schedule_3421_0_e30997;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3422_0_e31001,) = {
    if (w[1934] != 0.0) {
        (params.p2,)
    } else {
        (w[2008],)
    }
};
            w[2008] = noise_metadata_schedule_3422_0_e31001;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3423_0_e31009,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3423_0_e31005: f64 = (1.0 - params.p255);
        let noise_metadata_schedule_3423_0_e31007: f64 = (noise_metadata_schedule_3423_0_e31005 * params.p264);
        (noise_metadata_schedule_3423_0_e31007,)
    } else {
        (w[2009],)
    }
};
            w[2009] = noise_metadata_schedule_3423_0_e31009;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3424_0_e31013,) = {
    if (w[1934] != 0.0) {
        (params.p279,)
    } else {
        (w[2010],)
    }
};
            w[2010] = noise_metadata_schedule_3424_0_e31013;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3425_0_e31017,) = {
    if (w[1934] != 0.0) {
        (params.p274,)
    } else {
        (w[2011],)
    }
};
            w[2011] = noise_metadata_schedule_3425_0_e31017;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3426_0_e31021,) = {
    if (w[1934] != 0.0) {
        (params.p275,)
    } else {
        (w[2012],)
    }
};
            w[2012] = noise_metadata_schedule_3426_0_e31021;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3427_0_e31029,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3427_0_e31025: f64 = (1.0 - params.p255);
        let noise_metadata_schedule_3427_0_e31027: f64 = (noise_metadata_schedule_3427_0_e31025 * params.p273);
        (noise_metadata_schedule_3427_0_e31027,)
    } else {
        (w[2013],)
    }
};
            w[2013] = noise_metadata_schedule_3427_0_e31029;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3428_0_e31033,) = {
    if (w[1934] != 0.0) {
        (params.p272,)
    } else {
        (w[2014],)
    }
};
            w[2014] = noise_metadata_schedule_3428_0_e31033;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3429_0_e31037,) = {
    if (w[1934] != 0.0) {
        (params.p257,)
    } else {
        (w[2015],)
    }
};
            w[2015] = noise_metadata_schedule_3429_0_e31037;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3430_0_e31041,) = {
    if (w[1934] != 0.0) {
        (params.p256,)
    } else {
        (w[2016],)
    }
};
            w[2016] = noise_metadata_schedule_3430_0_e31041;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3431_0_e31045,) = {
    if (w[1934] != 0.0) {
        (params.p6,)
    } else {
        (w[2017],)
    }
};
            w[2017] = noise_metadata_schedule_3431_0_e31045;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3432_0_e31049,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2018],)
    }
};
            w[2018] = noise_metadata_schedule_3432_0_e31049;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3433_0_e31053,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2019],)
    }
};
            w[2019] = noise_metadata_schedule_3433_0_e31053;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3434_0_e31057,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2020],)
    }
};
            w[2020] = noise_metadata_schedule_3434_0_e31057;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3435_0_e31061,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2021],)
    }
};
            w[2021] = noise_metadata_schedule_3435_0_e31061;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3436_0_e31065,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2022],)
    }
};
            w[2022] = noise_metadata_schedule_3436_0_e31065;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3437_0_e31069,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2023],)
    }
};
            w[2023] = noise_metadata_schedule_3437_0_e31069;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3438_0_e31073,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2024],)
    }
};
            w[2024] = noise_metadata_schedule_3438_0_e31073;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_7(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3439_0_e31077,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2025],)
    }
};
            w[2025] = noise_metadata_schedule_3439_0_e31077;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3440_0_e31081,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2026],)
    }
};
            w[2026] = noise_metadata_schedule_3440_0_e31081;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3441_0_e31085,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2027],)
    }
};
            w[2027] = noise_metadata_schedule_3441_0_e31085;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3442_0_e31089,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2028],)
    }
};
            w[2028] = noise_metadata_schedule_3442_0_e31089;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3443_0_e31093,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2029],)
    }
};
            w[2029] = noise_metadata_schedule_3443_0_e31093;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3444_0_e31097,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2030],)
    }
};
            w[2030] = noise_metadata_schedule_3444_0_e31097;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3445_0_e31101,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2031],)
    }
};
            w[2031] = noise_metadata_schedule_3445_0_e31101;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3446_0_e31105,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2032],)
    }
};
            w[2032] = noise_metadata_schedule_3446_0_e31105;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3447_0_e31109,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2033],)
    }
};
            w[2033] = noise_metadata_schedule_3447_0_e31109;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3448_0_e31113,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2034],)
    }
};
            w[2034] = noise_metadata_schedule_3448_0_e31113;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3449_0_e31117,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2035],)
    }
};
            w[2035] = noise_metadata_schedule_3449_0_e31117;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3450_0_e31121,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2036],)
    }
};
            w[2036] = noise_metadata_schedule_3450_0_e31121;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3451_0_e31125,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2037],)
    }
};
            w[2037] = noise_metadata_schedule_3451_0_e31125;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3452_0_e31129,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2038],)
    }
};
            w[2038] = noise_metadata_schedule_3452_0_e31129;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3453_0_e31133,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2039],)
    }
};
            w[2039] = noise_metadata_schedule_3453_0_e31133;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3454_0_e31137,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2040],)
    }
};
            w[2040] = noise_metadata_schedule_3454_0_e31137;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3455_0_e31141,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2041],)
    }
};
            w[2041] = noise_metadata_schedule_3455_0_e31141;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3456_0_e31145,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2042],)
    }
};
            w[2042] = noise_metadata_schedule_3456_0_e31145;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3457_0_e31149,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2043],)
    }
};
            w[2043] = noise_metadata_schedule_3457_0_e31149;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3458_0_e31153,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2044],)
    }
};
            w[2044] = noise_metadata_schedule_3458_0_e31153;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3459_0_e31157,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2045],)
    }
};
            w[2045] = noise_metadata_schedule_3459_0_e31157;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3460_0_e31161,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2046],)
    }
};
            w[2046] = noise_metadata_schedule_3460_0_e31161;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3461_0_e31165,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2047],)
    }
};
            w[2047] = noise_metadata_schedule_3461_0_e31165;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3462_0_e31169,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2048],)
    }
};
            w[2048] = noise_metadata_schedule_3462_0_e31169;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3463_0_e31173,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2049],)
    }
};
            w[2049] = noise_metadata_schedule_3463_0_e31173;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3464_0_e31177,) = {
    if (w[1934] != 0.0) {
        (0.0,)
    } else {
        (w[2050],)
    }
};
            w[2050] = noise_metadata_schedule_3464_0_e31177;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3465_0_e31186,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3465_0_e31181: f64 = (w[2015] / w[1999]);
        let noise_metadata_schedule_3465_0_e31183: f64 = (-w[2016]);
        let noise_metadata_schedule_3465_0_e31184: f64 = (noise_metadata_schedule_3465_0_e31181 * noise_metadata_schedule_3465_0_e31183);
        (noise_metadata_schedule_3465_0_e31184,)
    } else {
        (w[2030],)
    }
};
            w[2030] = noise_metadata_schedule_3465_0_e31186;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3466_0_e31228,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3466_0_e31194: f64 = (-50.0);
        let (noise_metadata_schedule_3466_0_e31226,) = {
            if ((!(w[2030] > 50.0)) && (!(w[2030] < noise_metadata_schedule_3466_0_e31194))) {
                let noise_metadata_schedule_3466_0_e31199: f64 = (w[2030]).exp();
                (noise_metadata_schedule_3466_0_e31199,)
            } else {
                let noise_metadata_schedule_3466_0_e31206: f64 = (-50.0);
                let (noise_metadata_schedule_3466_0_e31225,) = {
                    if ((!(w[2030] > 50.0)) && (w[2030] < noise_metadata_schedule_3466_0_e31206)) {
                        let noise_metadata_schedule_3466_0_e31210: f64 = (-50.0);
                        let noise_metadata_schedule_3466_0_e31211: f64 = (noise_metadata_schedule_3466_0_e31210).exp();
                        (noise_metadata_schedule_3466_0_e31211,)
                    } else {
                        let (noise_metadata_schedule_3466_0_e31224,) = {
                            if (w[2030] > 50.0) {
                                let noise_metadata_schedule_3466_0_e31216: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3466_0_e31220: f64 = (w[2030] - 50.0);
                                let noise_metadata_schedule_3466_0_e31221: f64 = (1.0 + noise_metadata_schedule_3466_0_e31220);
                                let noise_metadata_schedule_3466_0_e31222: f64 = (noise_metadata_schedule_3466_0_e31216 * noise_metadata_schedule_3466_0_e31221);
                                (noise_metadata_schedule_3466_0_e31222,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3466_0_e31224,)
                    }
                };
                (noise_metadata_schedule_3466_0_e31225,)
            }
        };
        (noise_metadata_schedule_3466_0_e31226,)
    } else {
        (w[2020],)
    }
};
            w[2020] = noise_metadata_schedule_3466_0_e31228;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3467_0_e31239,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3467_0_e31232: f64 = (-w[1998]);
        let noise_metadata_schedule_3467_0_e31234: f64 = (noise_metadata_schedule_3467_0_e31232 - w[2005]);
        let noise_metadata_schedule_3467_0_e31235: f64 = (w[2004] * noise_metadata_schedule_3467_0_e31234);
        let noise_metadata_schedule_3467_0_e31237: f64 = (noise_metadata_schedule_3467_0_e31235 + w[2030]);
        (noise_metadata_schedule_3467_0_e31237,)
    } else {
        (w[2026],)
    }
};
            w[2026] = noise_metadata_schedule_3467_0_e31239;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3468_0_e31248,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3468_0_e31242: f64 = (-w[2004]);
        let noise_metadata_schedule_3468_0_e31244: f64 = (noise_metadata_schedule_3468_0_e31242 * w[2005]);
        let noise_metadata_schedule_3468_0_e31246: f64 = (noise_metadata_schedule_3468_0_e31244 + w[2030]);
        (noise_metadata_schedule_3468_0_e31246,)
    } else {
        (w[2027],)
    }
};
            w[2027] = noise_metadata_schedule_3468_0_e31248;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3469_0_e31290,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3469_0_e31256: f64 = (-50.0);
        let (noise_metadata_schedule_3469_0_e31288,) = {
            if ((!(w[2026] > 50.0)) && (!(w[2026] < noise_metadata_schedule_3469_0_e31256))) {
                let noise_metadata_schedule_3469_0_e31261: f64 = (w[2026]).exp();
                (noise_metadata_schedule_3469_0_e31261,)
            } else {
                let noise_metadata_schedule_3469_0_e31268: f64 = (-50.0);
                let (noise_metadata_schedule_3469_0_e31287,) = {
                    if ((!(w[2026] > 50.0)) && (w[2026] < noise_metadata_schedule_3469_0_e31268)) {
                        let noise_metadata_schedule_3469_0_e31272: f64 = (-50.0);
                        let noise_metadata_schedule_3469_0_e31273: f64 = (noise_metadata_schedule_3469_0_e31272).exp();
                        (noise_metadata_schedule_3469_0_e31273,)
                    } else {
                        let (noise_metadata_schedule_3469_0_e31286,) = {
                            if (w[2026] > 50.0) {
                                let noise_metadata_schedule_3469_0_e31278: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3469_0_e31282: f64 = (w[2026] - 50.0);
                                let noise_metadata_schedule_3469_0_e31283: f64 = (1.0 + noise_metadata_schedule_3469_0_e31282);
                                let noise_metadata_schedule_3469_0_e31284: f64 = (noise_metadata_schedule_3469_0_e31278 * noise_metadata_schedule_3469_0_e31283);
                                (noise_metadata_schedule_3469_0_e31284,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3469_0_e31286,)
                    }
                };
                (noise_metadata_schedule_3469_0_e31287,)
            }
        };
        (noise_metadata_schedule_3469_0_e31288,)
    } else {
        (w[2028],)
    }
};
            w[2028] = noise_metadata_schedule_3469_0_e31290;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3470_0_e31332,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3470_0_e31298: f64 = (-50.0);
        let (noise_metadata_schedule_3470_0_e31330,) = {
            if ((!(w[2027] > 50.0)) && (!(w[2027] < noise_metadata_schedule_3470_0_e31298))) {
                let noise_metadata_schedule_3470_0_e31303: f64 = (w[2027]).exp();
                (noise_metadata_schedule_3470_0_e31303,)
            } else {
                let noise_metadata_schedule_3470_0_e31310: f64 = (-50.0);
                let (noise_metadata_schedule_3470_0_e31329,) = {
                    if ((!(w[2027] > 50.0)) && (w[2027] < noise_metadata_schedule_3470_0_e31310)) {
                        let noise_metadata_schedule_3470_0_e31314: f64 = (-50.0);
                        let noise_metadata_schedule_3470_0_e31315: f64 = (noise_metadata_schedule_3470_0_e31314).exp();
                        (noise_metadata_schedule_3470_0_e31315,)
                    } else {
                        let (noise_metadata_schedule_3470_0_e31328,) = {
                            if (w[2027] > 50.0) {
                                let noise_metadata_schedule_3470_0_e31320: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3470_0_e31324: f64 = (w[2027] - 50.0);
                                let noise_metadata_schedule_3470_0_e31325: f64 = (1.0 + noise_metadata_schedule_3470_0_e31324);
                                let noise_metadata_schedule_3470_0_e31326: f64 = (noise_metadata_schedule_3470_0_e31320 * noise_metadata_schedule_3470_0_e31325);
                                (noise_metadata_schedule_3470_0_e31326,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3470_0_e31328,)
                    }
                };
                (noise_metadata_schedule_3470_0_e31329,)
            }
        };
        (noise_metadata_schedule_3470_0_e31330,)
    } else {
        (w[2029],)
    }
};
            w[2029] = noise_metadata_schedule_3470_0_e31332;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3471_0_e31338,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3471_0_e31336: f64 = (w[2028] - w[2029]);
        (noise_metadata_schedule_3471_0_e31336,)
    } else {
        (w[2022],)
    }
};
            w[2022] = noise_metadata_schedule_3471_0_e31338;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3472_0_e31350,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3472_0_e31342: f64 = (w[2017] * w[2007]);
        let noise_metadata_schedule_3472_0_e31344: f64 = (noise_metadata_schedule_3472_0_e31342 * w[2008]);
        let noise_metadata_schedule_3472_0_e31346: f64 = (noise_metadata_schedule_3472_0_e31344 * w[2009]);
        let noise_metadata_schedule_3472_0_e31348: f64 = (noise_metadata_schedule_3472_0_e31346 * w[2006]);
        (noise_metadata_schedule_3472_0_e31348,)
    } else {
        (w[1996],)
    }
};
            w[1996] = noise_metadata_schedule_3472_0_e31350;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3473_0_e31360,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3473_0_e31354: f64 = (w[2003] / w[1999]);
        let noise_metadata_schedule_3473_0_e31356: f64 = (noise_metadata_schedule_3473_0_e31354 * w[1998]);
        let noise_metadata_schedule_3473_0_e31358: f64 = (noise_metadata_schedule_3473_0_e31356 + w[2030]);
        (noise_metadata_schedule_3473_0_e31358,)
    } else {
        (w[2032],)
    }
};
            w[2032] = noise_metadata_schedule_3473_0_e31360;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3474_0_e31402,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3474_0_e31368: f64 = (-50.0);
        let (noise_metadata_schedule_3474_0_e31400,) = {
            if ((!(w[2032] > 50.0)) && (!(w[2032] < noise_metadata_schedule_3474_0_e31368))) {
                let noise_metadata_schedule_3474_0_e31373: f64 = (w[2032]).exp();
                (noise_metadata_schedule_3474_0_e31373,)
            } else {
                let noise_metadata_schedule_3474_0_e31380: f64 = (-50.0);
                let (noise_metadata_schedule_3474_0_e31399,) = {
                    if ((!(w[2032] > 50.0)) && (w[2032] < noise_metadata_schedule_3474_0_e31380)) {
                        let noise_metadata_schedule_3474_0_e31384: f64 = (-50.0);
                        let noise_metadata_schedule_3474_0_e31385: f64 = (noise_metadata_schedule_3474_0_e31384).exp();
                        (noise_metadata_schedule_3474_0_e31385,)
                    } else {
                        let (noise_metadata_schedule_3474_0_e31398,) = {
                            if (w[2032] > 50.0) {
                                let noise_metadata_schedule_3474_0_e31390: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3474_0_e31394: f64 = (w[2032] - 50.0);
                                let noise_metadata_schedule_3474_0_e31395: f64 = (1.0 + noise_metadata_schedule_3474_0_e31394);
                                let noise_metadata_schedule_3474_0_e31396: f64 = (noise_metadata_schedule_3474_0_e31390 * noise_metadata_schedule_3474_0_e31395);
                                (noise_metadata_schedule_3474_0_e31396,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3474_0_e31398,)
                    }
                };
                (noise_metadata_schedule_3474_0_e31399,)
            }
        };
        (noise_metadata_schedule_3474_0_e31400,)
    } else {
        (w[2033],)
    }
};
            w[2033] = noise_metadata_schedule_3474_0_e31402;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_3475_0_e31405: f64 = if w[2002] == 1.0 { 1.0 } else { 0.0 };
            w[2051] = noise_metadata_schedule_3475_0_e31405;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_8(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3476_0_e31419,) = {
    if ((w[1934] != 0.0) && (w[2051] != 0.0)) {
        let noise_metadata_schedule_3476_0_e31413: f64 = (w[2010] * w[2022]);
        let noise_metadata_schedule_3476_0_e31414: f64 = (w[2033] - noise_metadata_schedule_3476_0_e31413);
        let noise_metadata_schedule_3476_0_e31416: f64 = (noise_metadata_schedule_3476_0_e31414 - w[2020]);
        let noise_metadata_schedule_3476_0_e31417: f64 = (w[1996] * noise_metadata_schedule_3476_0_e31416);
        (noise_metadata_schedule_3476_0_e31417,)
    } else {
        (w[2023],)
    }
};
            w[2023] = noise_metadata_schedule_3476_0_e31419;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3477_0_e31433,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3477_0_e31426: f64 = (-w[2000]);
        let noise_metadata_schedule_3477_0_e31428: f64 = (noise_metadata_schedule_3477_0_e31426 - w[2005]);
        let noise_metadata_schedule_3477_0_e31429: f64 = (w[2004] * noise_metadata_schedule_3477_0_e31428);
        let noise_metadata_schedule_3477_0_e31431: f64 = (noise_metadata_schedule_3477_0_e31429 + w[2030]);
        (noise_metadata_schedule_3477_0_e31431,)
    } else {
        (w[2037],)
    }
};
            w[2037] = noise_metadata_schedule_3477_0_e31433;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3478_0_e31478,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3478_0_e31444: f64 = (-50.0);
        let (noise_metadata_schedule_3478_0_e31476,) = {
            if ((!(w[2037] > 50.0)) && (!(w[2037] < noise_metadata_schedule_3478_0_e31444))) {
                let noise_metadata_schedule_3478_0_e31449: f64 = (w[2037]).exp();
                (noise_metadata_schedule_3478_0_e31449,)
            } else {
                let noise_metadata_schedule_3478_0_e31456: f64 = (-50.0);
                let (noise_metadata_schedule_3478_0_e31475,) = {
                    if ((!(w[2037] > 50.0)) && (w[2037] < noise_metadata_schedule_3478_0_e31456)) {
                        let noise_metadata_schedule_3478_0_e31460: f64 = (-50.0);
                        let noise_metadata_schedule_3478_0_e31461: f64 = (noise_metadata_schedule_3478_0_e31460).exp();
                        (noise_metadata_schedule_3478_0_e31461,)
                    } else {
                        let (noise_metadata_schedule_3478_0_e31474,) = {
                            if (w[2037] > 50.0) {
                                let noise_metadata_schedule_3478_0_e31466: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3478_0_e31470: f64 = (w[2037] - 50.0);
                                let noise_metadata_schedule_3478_0_e31471: f64 = (1.0 + noise_metadata_schedule_3478_0_e31470);
                                let noise_metadata_schedule_3478_0_e31472: f64 = (noise_metadata_schedule_3478_0_e31466 * noise_metadata_schedule_3478_0_e31471);
                                (noise_metadata_schedule_3478_0_e31472,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3478_0_e31474,)
                    }
                };
                (noise_metadata_schedule_3478_0_e31475,)
            }
        };
        (noise_metadata_schedule_3478_0_e31476,)
    } else {
        (w[2038],)
    }
};
            w[2038] = noise_metadata_schedule_3478_0_e31478;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3479_0_e31487,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3479_0_e31485: f64 = (w[2038] - w[2029]);
        (noise_metadata_schedule_3479_0_e31485,)
    } else {
        (w[2039],)
    }
};
            w[2039] = noise_metadata_schedule_3479_0_e31487;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3480_0_e31500,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3480_0_e31494: f64 = (w[2003] / w[1999]);
        let noise_metadata_schedule_3480_0_e31496: f64 = (noise_metadata_schedule_3480_0_e31494 * w[2000]);
        let noise_metadata_schedule_3480_0_e31498: f64 = (noise_metadata_schedule_3480_0_e31496 + w[2030]);
        (noise_metadata_schedule_3480_0_e31498,)
    } else {
        (w[2040],)
    }
};
            w[2040] = noise_metadata_schedule_3480_0_e31500;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3481_0_e31545,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3481_0_e31511: f64 = (-50.0);
        let (noise_metadata_schedule_3481_0_e31543,) = {
            if ((!(w[2040] > 50.0)) && (!(w[2040] < noise_metadata_schedule_3481_0_e31511))) {
                let noise_metadata_schedule_3481_0_e31516: f64 = (w[2040]).exp();
                (noise_metadata_schedule_3481_0_e31516,)
            } else {
                let noise_metadata_schedule_3481_0_e31523: f64 = (-50.0);
                let (noise_metadata_schedule_3481_0_e31542,) = {
                    if ((!(w[2040] > 50.0)) && (w[2040] < noise_metadata_schedule_3481_0_e31523)) {
                        let noise_metadata_schedule_3481_0_e31527: f64 = (-50.0);
                        let noise_metadata_schedule_3481_0_e31528: f64 = (noise_metadata_schedule_3481_0_e31527).exp();
                        (noise_metadata_schedule_3481_0_e31528,)
                    } else {
                        let (noise_metadata_schedule_3481_0_e31541,) = {
                            if (w[2040] > 50.0) {
                                let noise_metadata_schedule_3481_0_e31533: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3481_0_e31537: f64 = (w[2040] - 50.0);
                                let noise_metadata_schedule_3481_0_e31538: f64 = (1.0 + noise_metadata_schedule_3481_0_e31537);
                                let noise_metadata_schedule_3481_0_e31539: f64 = (noise_metadata_schedule_3481_0_e31533 * noise_metadata_schedule_3481_0_e31538);
                                (noise_metadata_schedule_3481_0_e31539,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3481_0_e31541,)
                    }
                };
                (noise_metadata_schedule_3481_0_e31542,)
            }
        };
        (noise_metadata_schedule_3481_0_e31543,)
    } else {
        (w[2041],)
    }
};
            w[2041] = noise_metadata_schedule_3481_0_e31545;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3482_0_e31558,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3482_0_e31553: f64 = (w[2010] * w[2039]);
        let noise_metadata_schedule_3482_0_e31554: f64 = (w[2041] - noise_metadata_schedule_3482_0_e31553);
        let noise_metadata_schedule_3482_0_e31556: f64 = (noise_metadata_schedule_3482_0_e31554 - w[2020]);
        (noise_metadata_schedule_3482_0_e31556,)
    } else {
        (w[2042],)
    }
};
            w[2042] = noise_metadata_schedule_3482_0_e31558;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3483_0_e31573,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3483_0_e31567: f64 = (w[2010] * w[2022]);
        let noise_metadata_schedule_3483_0_e31568: f64 = (w[2033] - noise_metadata_schedule_3483_0_e31567);
        let noise_metadata_schedule_3483_0_e31570: f64 = (noise_metadata_schedule_3483_0_e31568 - w[2020]);
        let noise_metadata_schedule_3483_0_e31571: f64 = (w[1996] * noise_metadata_schedule_3483_0_e31570);
        (noise_metadata_schedule_3483_0_e31571,)
    } else {
        (w[2043],)
    }
};
            w[2043] = noise_metadata_schedule_3483_0_e31573;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_3484_0_e31576: f64 = if w[2002] > 0.0 { 1.0 } else { 0.0 };
            w[2052] = noise_metadata_schedule_3484_0_e31576;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3485_0_e31587,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3485_0_e31585: f64 = (w[2002] * w[2003]);
        (noise_metadata_schedule_3485_0_e31585,)
    } else {
        (w[2036],)
    }
};
            w[2036] = noise_metadata_schedule_3485_0_e31587;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3486_0_e31602,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3486_0_e31596: f64 = (w[2036] / w[1999]);
        let noise_metadata_schedule_3486_0_e31598: f64 = (noise_metadata_schedule_3486_0_e31596 * w[2000]);
        let noise_metadata_schedule_3486_0_e31600: f64 = (noise_metadata_schedule_3486_0_e31598 + w[2030]);
        (noise_metadata_schedule_3486_0_e31600,)
    } else {
        (w[2044],)
    }
};
            w[2044] = noise_metadata_schedule_3486_0_e31602;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3487_0_e31649,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3487_0_e31615: f64 = (-50.0);
        let (noise_metadata_schedule_3487_0_e31647,) = {
            if ((!(w[2044] > 50.0)) && (!(w[2044] < noise_metadata_schedule_3487_0_e31615))) {
                let noise_metadata_schedule_3487_0_e31620: f64 = (w[2044]).exp();
                (noise_metadata_schedule_3487_0_e31620,)
            } else {
                let noise_metadata_schedule_3487_0_e31627: f64 = (-50.0);
                let (noise_metadata_schedule_3487_0_e31646,) = {
                    if ((!(w[2044] > 50.0)) && (w[2044] < noise_metadata_schedule_3487_0_e31627)) {
                        let noise_metadata_schedule_3487_0_e31631: f64 = (-50.0);
                        let noise_metadata_schedule_3487_0_e31632: f64 = (noise_metadata_schedule_3487_0_e31631).exp();
                        (noise_metadata_schedule_3487_0_e31632,)
                    } else {
                        let (noise_metadata_schedule_3487_0_e31645,) = {
                            if (w[2044] > 50.0) {
                                let noise_metadata_schedule_3487_0_e31637: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3487_0_e31641: f64 = (w[2044] - 50.0);
                                let noise_metadata_schedule_3487_0_e31642: f64 = (1.0 + noise_metadata_schedule_3487_0_e31641);
                                let noise_metadata_schedule_3487_0_e31643: f64 = (noise_metadata_schedule_3487_0_e31637 * noise_metadata_schedule_3487_0_e31642);
                                (noise_metadata_schedule_3487_0_e31643,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3487_0_e31645,)
                    }
                };
                (noise_metadata_schedule_3487_0_e31646,)
            }
        };
        (noise_metadata_schedule_3487_0_e31647,)
    } else {
        (w[2045],)
    }
};
            w[2045] = noise_metadata_schedule_3487_0_e31649;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3488_0_e31664,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3488_0_e31659: f64 = (w[2010] * w[2039]);
        let noise_metadata_schedule_3488_0_e31660: f64 = (w[2045] - noise_metadata_schedule_3488_0_e31659);
        let noise_metadata_schedule_3488_0_e31662: f64 = (noise_metadata_schedule_3488_0_e31660 - w[2020]);
        (noise_metadata_schedule_3488_0_e31662,)
    } else {
        (w[2046],)
    }
};
            w[2046] = noise_metadata_schedule_3488_0_e31664;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3489_0_e31679,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3489_0_e31673: f64 = (w[2036] / w[1999]);
        let noise_metadata_schedule_3489_0_e31675: f64 = (noise_metadata_schedule_3489_0_e31673 * w[1998]);
        let noise_metadata_schedule_3489_0_e31677: f64 = (noise_metadata_schedule_3489_0_e31675 + w[2030]);
        (noise_metadata_schedule_3489_0_e31677,)
    } else {
        (w[2047],)
    }
};
            w[2047] = noise_metadata_schedule_3489_0_e31679;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3490_0_e31726,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3490_0_e31692: f64 = (-50.0);
        let (noise_metadata_schedule_3490_0_e31724,) = {
            if ((!(w[2047] > 50.0)) && (!(w[2047] < noise_metadata_schedule_3490_0_e31692))) {
                let noise_metadata_schedule_3490_0_e31697: f64 = (w[2047]).exp();
                (noise_metadata_schedule_3490_0_e31697,)
            } else {
                let noise_metadata_schedule_3490_0_e31704: f64 = (-50.0);
                let (noise_metadata_schedule_3490_0_e31723,) = {
                    if ((!(w[2047] > 50.0)) && (w[2047] < noise_metadata_schedule_3490_0_e31704)) {
                        let noise_metadata_schedule_3490_0_e31708: f64 = (-50.0);
                        let noise_metadata_schedule_3490_0_e31709: f64 = (noise_metadata_schedule_3490_0_e31708).exp();
                        (noise_metadata_schedule_3490_0_e31709,)
                    } else {
                        let (noise_metadata_schedule_3490_0_e31722,) = {
                            if (w[2047] > 50.0) {
                                let noise_metadata_schedule_3490_0_e31714: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3490_0_e31718: f64 = (w[2047] - 50.0);
                                let noise_metadata_schedule_3490_0_e31719: f64 = (1.0 + noise_metadata_schedule_3490_0_e31718);
                                let noise_metadata_schedule_3490_0_e31720: f64 = (noise_metadata_schedule_3490_0_e31714 * noise_metadata_schedule_3490_0_e31719);
                                (noise_metadata_schedule_3490_0_e31720,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3490_0_e31722,)
                    }
                };
                (noise_metadata_schedule_3490_0_e31723,)
            }
        };
        (noise_metadata_schedule_3490_0_e31724,)
    } else {
        (w[2048],)
    }
};
            w[2048] = noise_metadata_schedule_3490_0_e31726;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3491_0_e31739,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3491_0_e31735: f64 = (w[1996] * w[2042]);
        let noise_metadata_schedule_3491_0_e31737: f64 = (noise_metadata_schedule_3491_0_e31735 / w[2046]);
        (noise_metadata_schedule_3491_0_e31737,)
    } else {
        (w[2049],)
    }
};
            w[2049] = noise_metadata_schedule_3491_0_e31739;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3492_0_e31756,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] != 0.0)) {
        let noise_metadata_schedule_3492_0_e31750: f64 = (w[2010] * w[2022]);
        let noise_metadata_schedule_3492_0_e31751: f64 = (w[2048] - noise_metadata_schedule_3492_0_e31750);
        let noise_metadata_schedule_3492_0_e31753: f64 = (noise_metadata_schedule_3492_0_e31751 - w[2020]);
        let noise_metadata_schedule_3492_0_e31754: f64 = (w[2049] * noise_metadata_schedule_3492_0_e31753);
        (noise_metadata_schedule_3492_0_e31754,)
    } else {
        (w[2050],)
    }
};
            w[2050] = noise_metadata_schedule_3492_0_e31756;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3493_0_e31768,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2052] == 0.0)) {
        let noise_metadata_schedule_3493_0_e31766: f64 = (w[1996] * w[2042]);
        (noise_metadata_schedule_3493_0_e31766,)
    } else {
        (w[2050],)
    }
};
            w[2050] = noise_metadata_schedule_3493_0_e31768;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3494_0_e31779,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3494_0_e31775: f64 = (w[2001] * w[2001]);
        let noise_metadata_schedule_3494_0_e31777: f64 = (noise_metadata_schedule_3494_0_e31775 * w[1999]);
        (noise_metadata_schedule_3494_0_e31777,)
    } else {
        (w[2019],)
    }
};
            w[2019] = noise_metadata_schedule_3494_0_e31779;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3495_0_e31794,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3495_0_e31788: f64 = (w[2019] / 2.0);
        let noise_metadata_schedule_3495_0_e31789: f64 = (w[2000] - noise_metadata_schedule_3495_0_e31788);
        let noise_metadata_schedule_3495_0_e31790: f64 = (w[1998] - noise_metadata_schedule_3495_0_e31789);
        let noise_metadata_schedule_3495_0_e31792: f64 = (noise_metadata_schedule_3495_0_e31790 / w[2019]);
        (noise_metadata_schedule_3495_0_e31792,)
    } else {
        (w[2031],)
    }
};
            w[2031] = noise_metadata_schedule_3495_0_e31794;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_3496_0_e31797: f64 = if w[2031] > 50.0 { 1.0 } else { 0.0 };
            w[2053] = noise_metadata_schedule_3496_0_e31797;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3497_0_e31806,) = {
    if (((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2053] != 0.0)) {
        (0.0,)
    } else {
        (w[2021],)
    }
};
            w[2021] = noise_metadata_schedule_3497_0_e31806;
        }
        if (active[0] & 0x8) != 0 {
            let noise_metadata_schedule_3498_0_e31809: f64 = (-50.0);
            let noise_metadata_schedule_3498_0_e31810: f64 = if w[2031] < noise_metadata_schedule_3498_0_e31809 { 1.0 } else { 0.0 };
            w[2054] = noise_metadata_schedule_3498_0_e31810;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3499_0_e31822,) = {
    if ((((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2053] == 0.0)) && (w[2054] != 0.0)) {
        (1.0,)
    } else {
        (w[2021],)
    }
};
            w[2021] = noise_metadata_schedule_3499_0_e31822;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3500_0_e31840,) = {
    if ((((w[1934] != 0.0) && (w[2051] == 0.0)) && (w[2053] == 0.0)) && (w[2054] == 0.0)) {
        let noise_metadata_schedule_3500_0_e31836: f64 = (w[2031]).exp();
        let noise_metadata_schedule_3500_0_e31837: f64 = (1.0 + noise_metadata_schedule_3500_0_e31836);
        let noise_metadata_schedule_3500_0_e31838: f64 = (1.0 / noise_metadata_schedule_3500_0_e31837);
        (noise_metadata_schedule_3500_0_e31838,)
    } else {
        (w[2021],)
    }
};
            w[2021] = noise_metadata_schedule_3500_0_e31840;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3501_0_e31855,) = {
    if ((w[1934] != 0.0) && (w[2051] == 0.0)) {
        let noise_metadata_schedule_3501_0_e31847: f64 = (w[2021] * w[2043]);
        let noise_metadata_schedule_3501_0_e31850: f64 = (1.0 - w[2021]);
        let noise_metadata_schedule_3501_0_e31852: f64 = (noise_metadata_schedule_3501_0_e31850 * w[2050]);
        let noise_metadata_schedule_3501_0_e31853: f64 = (noise_metadata_schedule_3501_0_e31847 + noise_metadata_schedule_3501_0_e31852);
        (noise_metadata_schedule_3501_0_e31853,)
    } else {
        (w[2023],)
    }
};
            w[2023] = noise_metadata_schedule_3501_0_e31855;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3502_0_e31901,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3502_0_e31858: f64 = (-w[1998]);
        let (noise_metadata_schedule_3502_0_e31891,) = {
            if (params.p52 != 0.0) {
                let noise_metadata_schedule_3502_0_e31866: f64 = (w[1998] / w[2011]);
                let noise_metadata_schedule_3502_0_e31869: f64 = (0.001 / params.p53);
                let noise_metadata_schedule_3502_0_e31872: f64 = (w[1998] / w[2011]);
                let noise_metadata_schedule_3502_0_e31873: f64 = (noise_metadata_schedule_3502_0_e31869 * noise_metadata_schedule_3502_0_e31872);
                let noise_metadata_schedule_3502_0_e31874: f64 = (noise_metadata_schedule_3502_0_e31873).tanh();
                let noise_metadata_schedule_3502_0_e31875: f64 = (noise_metadata_schedule_3502_0_e31866 * noise_metadata_schedule_3502_0_e31874);
                (noise_metadata_schedule_3502_0_e31875,)
            } else {
                let (noise_metadata_schedule_3502_0_e31890,) = {
                    if (params.p52 == 0.0) {
                        let noise_metadata_schedule_3502_0_e31881: f64 = (w[1998] / w[2011]);
                        let noise_metadata_schedule_3502_0_e31884: f64 = (w[1998] / w[2011]);
                        let noise_metadata_schedule_3502_0_e31885: f64 = (noise_metadata_schedule_3502_0_e31881 * noise_metadata_schedule_3502_0_e31884);
                        let noise_metadata_schedule_3502_0_e31887: f64 = (noise_metadata_schedule_3502_0_e31885 + params.p53);
                        let noise_metadata_schedule_3502_0_e31888: f64 = (noise_metadata_schedule_3502_0_e31887).sqrt();
                        (noise_metadata_schedule_3502_0_e31888,)
                    } else {
                        (0.0,)
                    }
                };
                (noise_metadata_schedule_3502_0_e31890,)
            }
        };
        let noise_metadata_schedule_3502_0_e31893: f64 = (noise_metadata_schedule_3502_0_e31891).powf(w[2012]);
        let noise_metadata_schedule_3502_0_e31894: f64 = (1.0 + noise_metadata_schedule_3502_0_e31893);
        let noise_metadata_schedule_3502_0_e31897: f64 = (1.0 / w[2012]);
        let noise_metadata_schedule_3502_0_e31898: f64 = (noise_metadata_schedule_3502_0_e31894).powf(noise_metadata_schedule_3502_0_e31897);
        let noise_metadata_schedule_3502_0_e31899: f64 = (noise_metadata_schedule_3502_0_e31858 / noise_metadata_schedule_3502_0_e31898);
        (noise_metadata_schedule_3502_0_e31899,)
    } else {
        (w[2024],)
    }
};
            w[2024] = noise_metadata_schedule_3502_0_e31901;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3503_0_e31916,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3503_0_e31904: f64 = (-w[2017]);
        let noise_metadata_schedule_3503_0_e31906: f64 = (noise_metadata_schedule_3503_0_e31904 * w[2007]);
        let noise_metadata_schedule_3503_0_e31908: f64 = (noise_metadata_schedule_3503_0_e31906 * w[2008]);
        let noise_metadata_schedule_3503_0_e31910: f64 = (noise_metadata_schedule_3503_0_e31908 * w[2013]);
        let noise_metadata_schedule_3503_0_e31912: f64 = (noise_metadata_schedule_3503_0_e31910 * w[2006]);
        let noise_metadata_schedule_3503_0_e31914: f64 = noise_metadata_schedule_3503_0_e31912;
        (noise_metadata_schedule_3503_0_e31914,)
    } else {
        (w[1997],)
    }
};
            w[1997] = noise_metadata_schedule_3503_0_e31916;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3504_0_e31924,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3504_0_e31920: f64 = (w[2014] / w[1999]);
        let noise_metadata_schedule_3504_0_e31922: f64 = (noise_metadata_schedule_3504_0_e31920 * w[2024]);
        (noise_metadata_schedule_3504_0_e31922,)
    } else {
        (w[2034],)
    }
};
            w[2034] = noise_metadata_schedule_3504_0_e31924;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_9(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3505_0_e31966,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3505_0_e31932: f64 = (-50.0);
        let (noise_metadata_schedule_3505_0_e31964,) = {
            if ((!(w[2034] > 50.0)) && (!(w[2034] < noise_metadata_schedule_3505_0_e31932))) {
                let noise_metadata_schedule_3505_0_e31937: f64 = (w[2034]).exp();
                (noise_metadata_schedule_3505_0_e31937,)
            } else {
                let noise_metadata_schedule_3505_0_e31944: f64 = (-50.0);
                let (noise_metadata_schedule_3505_0_e31963,) = {
                    if ((!(w[2034] > 50.0)) && (w[2034] < noise_metadata_schedule_3505_0_e31944)) {
                        let noise_metadata_schedule_3505_0_e31948: f64 = (-50.0);
                        let noise_metadata_schedule_3505_0_e31949: f64 = (noise_metadata_schedule_3505_0_e31948).exp();
                        (noise_metadata_schedule_3505_0_e31949,)
                    } else {
                        let (noise_metadata_schedule_3505_0_e31962,) = {
                            if (w[2034] > 50.0) {
                                let noise_metadata_schedule_3505_0_e31954: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3505_0_e31958: f64 = (w[2034] - 50.0);
                                let noise_metadata_schedule_3505_0_e31959: f64 = (1.0 + noise_metadata_schedule_3505_0_e31958);
                                let noise_metadata_schedule_3505_0_e31960: f64 = (noise_metadata_schedule_3505_0_e31954 * noise_metadata_schedule_3505_0_e31959);
                                (noise_metadata_schedule_3505_0_e31960,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3505_0_e31962,)
                    }
                };
                (noise_metadata_schedule_3505_0_e31963,)
            }
        };
        (noise_metadata_schedule_3505_0_e31964,)
    } else {
        (w[2035],)
    }
};
            w[2035] = noise_metadata_schedule_3505_0_e31966;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3506_0_e31974,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3506_0_e31971: f64 = (w[2035] - 1.0);
        let noise_metadata_schedule_3506_0_e31972: f64 = (w[1997] * noise_metadata_schedule_3506_0_e31971);
        (noise_metadata_schedule_3506_0_e31972,)
    } else {
        (w[2025],)
    }
};
            w[2025] = noise_metadata_schedule_3506_0_e31974;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3507_0_e31980,) = {
    if (w[1934] != 0.0) {
        let noise_metadata_schedule_3507_0_e31978: f64 = (w[2023] + w[2025]);
        (noise_metadata_schedule_3507_0_e31978,)
    } else {
        (w[2018],)
    }
};
            w[2018] = noise_metadata_schedule_3507_0_e31980;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3508_0_e31984,) = {
    if (w[1934] != 0.0) {
        (w[2018],)
    } else {
        (w[1995],)
    }
};
            w[1995] = noise_metadata_schedule_3508_0_e31984;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3509_0_e31988,) = {
    if (w[1934] != 0.0) {
        (w[1996],)
    } else {
        (w[125],)
    }
};
            w[125] = noise_metadata_schedule_3509_0_e31988;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3510_0_e31992,) = {
    if (w[1934] != 0.0) {
        (w[1997],)
    } else {
        (w[127],)
    }
};
            w[127] = noise_metadata_schedule_3510_0_e31992;
        }
        if (active[0] & 0x8) != 0 {
            let (noise_metadata_schedule_3511_0_e31996,) = {
    if (w[1934] != 0.0) {
        (w[1995],)
    } else {
        (w[123],)
    }
};
            w[123] = noise_metadata_schedule_3511_0_e31996;
        }
        if (active[0] & 0x3) != 0 {
            let noise_metadata_schedule_3719_0_e34496: f64 = if params.p255 != 0.0 { 1.0 } else { 0.0 };
            w[2176] = noise_metadata_schedule_3719_0_e34496;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3720_0_e34502,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2177],)
    }
};
            w[2177] = noise_metadata_schedule_3720_0_e34502;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3721_0_e34508,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2178],)
    }
};
            w[2178] = noise_metadata_schedule_3721_0_e34508;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3722_0_e34514,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2179],)
    }
};
            w[2179] = noise_metadata_schedule_3722_0_e34514;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3723_0_e34522,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3723_0_e34520: f64 = (params.p6 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        (noise_metadata_schedule_3723_0_e34520,)
    } else {
        (w[2180],)
    }
};
            w[2180] = noise_metadata_schedule_3723_0_e34522;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3724_0_e34528,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[113],)
    } else {
        (w[2181],)
    }
};
            w[2181] = noise_metadata_schedule_3724_0_e34528;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3725_0_e34534,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p260,)
    } else {
        (w[2182],)
    }
};
            w[2182] = noise_metadata_schedule_3725_0_e34534;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3726_0_e34540,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p262,)
    } else {
        (w[2183],)
    }
};
            w[2183] = noise_metadata_schedule_3726_0_e34540;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3727_0_e34546,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p261,)
    } else {
        (w[2184],)
    }
};
            w[2184] = noise_metadata_schedule_3727_0_e34546;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3728_0_e34552,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p258,)
    } else {
        (w[2185],)
    }
};
            w[2185] = noise_metadata_schedule_3728_0_e34552;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3729_0_e34558,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p278,)
    } else {
        (w[2186],)
    }
};
            w[2186] = noise_metadata_schedule_3729_0_e34558;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3730_0_e34564,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p277,)
    } else {
        (w[2187],)
    }
};
            w[2187] = noise_metadata_schedule_3730_0_e34564;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3731_0_e34570,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[112],)
    } else {
        (w[2188],)
    }
};
            w[2188] = noise_metadata_schedule_3731_0_e34570;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3732_0_e34576,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p0,)
    } else {
        (w[2189],)
    }
};
            w[2189] = noise_metadata_schedule_3732_0_e34576;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3733_0_e34582,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p2,)
    } else {
        (w[2190],)
    }
};
            w[2190] = noise_metadata_schedule_3733_0_e34582;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3734_0_e34590,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3734_0_e34588: f64 = (params.p255 * params.p259);
        (noise_metadata_schedule_3734_0_e34588,)
    } else {
        (w[2191],)
    }
};
            w[2191] = noise_metadata_schedule_3734_0_e34590;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3735_0_e34596,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p276,)
    } else {
        (w[2192],)
    }
};
            w[2192] = noise_metadata_schedule_3735_0_e34596;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3736_0_e34602,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p270,)
    } else {
        (w[2193],)
    }
};
            w[2193] = noise_metadata_schedule_3736_0_e34602;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3737_0_e34608,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p271,)
    } else {
        (w[2194],)
    }
};
            w[2194] = noise_metadata_schedule_3737_0_e34608;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3738_0_e34616,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3738_0_e34614: f64 = (params.p255 * params.p269);
        (noise_metadata_schedule_3738_0_e34614,)
    } else {
        (w[2195],)
    }
};
            w[2195] = noise_metadata_schedule_3738_0_e34616;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3739_0_e34622,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p268,)
    } else {
        (w[2196],)
    }
};
            w[2196] = noise_metadata_schedule_3739_0_e34622;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3740_0_e34628,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p257,)
    } else {
        (w[2197],)
    }
};
            w[2197] = noise_metadata_schedule_3740_0_e34628;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3741_0_e34634,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p256,)
    } else {
        (w[2198],)
    }
};
            w[2198] = noise_metadata_schedule_3741_0_e34634;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3742_0_e34640,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p6,)
    } else {
        (w[2199],)
    }
};
            w[2199] = noise_metadata_schedule_3742_0_e34640;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3743_0_e34646,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2200],)
    }
};
            w[2200] = noise_metadata_schedule_3743_0_e34646;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3744_0_e34652,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2201],)
    }
};
            w[2201] = noise_metadata_schedule_3744_0_e34652;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3745_0_e34658,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2202],)
    }
};
            w[2202] = noise_metadata_schedule_3745_0_e34658;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3746_0_e34664,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2203],)
    }
};
            w[2203] = noise_metadata_schedule_3746_0_e34664;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3747_0_e34670,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2204],)
    }
};
            w[2204] = noise_metadata_schedule_3747_0_e34670;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3748_0_e34676,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2205],)
    }
};
            w[2205] = noise_metadata_schedule_3748_0_e34676;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3749_0_e34682,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2206],)
    }
};
            w[2206] = noise_metadata_schedule_3749_0_e34682;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3750_0_e34688,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2207],)
    }
};
            w[2207] = noise_metadata_schedule_3750_0_e34688;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3751_0_e34694,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2208],)
    }
};
            w[2208] = noise_metadata_schedule_3751_0_e34694;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3752_0_e34700,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2209],)
    }
};
            w[2209] = noise_metadata_schedule_3752_0_e34700;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3753_0_e34706,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2210],)
    }
};
            w[2210] = noise_metadata_schedule_3753_0_e34706;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3754_0_e34712,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2211],)
    }
};
            w[2211] = noise_metadata_schedule_3754_0_e34712;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3755_0_e34718,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2212],)
    }
};
            w[2212] = noise_metadata_schedule_3755_0_e34718;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3756_0_e34724,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2213],)
    }
};
            w[2213] = noise_metadata_schedule_3756_0_e34724;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3757_0_e34730,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2214],)
    }
};
            w[2214] = noise_metadata_schedule_3757_0_e34730;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3758_0_e34736,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2215],)
    }
};
            w[2215] = noise_metadata_schedule_3758_0_e34736;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3759_0_e34742,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2216],)
    }
};
            w[2216] = noise_metadata_schedule_3759_0_e34742;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_10(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3760_0_e34748,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2217],)
    }
};
            w[2217] = noise_metadata_schedule_3760_0_e34748;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3761_0_e34754,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2218],)
    }
};
            w[2218] = noise_metadata_schedule_3761_0_e34754;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3762_0_e34760,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2219],)
    }
};
            w[2219] = noise_metadata_schedule_3762_0_e34760;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3763_0_e34766,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2220],)
    }
};
            w[2220] = noise_metadata_schedule_3763_0_e34766;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3764_0_e34772,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2221],)
    }
};
            w[2221] = noise_metadata_schedule_3764_0_e34772;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3765_0_e34778,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2222],)
    }
};
            w[2222] = noise_metadata_schedule_3765_0_e34778;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3766_0_e34784,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2223],)
    }
};
            w[2223] = noise_metadata_schedule_3766_0_e34784;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3767_0_e34790,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2224],)
    }
};
            w[2224] = noise_metadata_schedule_3767_0_e34790;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3768_0_e34796,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2225],)
    }
};
            w[2225] = noise_metadata_schedule_3768_0_e34796;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3769_0_e34802,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2226],)
    }
};
            w[2226] = noise_metadata_schedule_3769_0_e34802;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3770_0_e34808,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2227],)
    }
};
            w[2227] = noise_metadata_schedule_3770_0_e34808;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3771_0_e34814,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2228],)
    }
};
            w[2228] = noise_metadata_schedule_3771_0_e34814;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3772_0_e34820,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2229],)
    }
};
            w[2229] = noise_metadata_schedule_3772_0_e34820;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3773_0_e34826,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2230],)
    }
};
            w[2230] = noise_metadata_schedule_3773_0_e34826;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3774_0_e34832,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2231],)
    }
};
            w[2231] = noise_metadata_schedule_3774_0_e34832;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3775_0_e34838,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2232],)
    }
};
            w[2232] = noise_metadata_schedule_3775_0_e34838;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3776_0_e34849,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3776_0_e34844: f64 = (w[2197] / w[2181]);
        let noise_metadata_schedule_3776_0_e34846: f64 = (-w[2198]);
        let noise_metadata_schedule_3776_0_e34847: f64 = (noise_metadata_schedule_3776_0_e34844 * noise_metadata_schedule_3776_0_e34846);
        (noise_metadata_schedule_3776_0_e34847,)
    } else {
        (w[2212],)
    }
};
            w[2212] = noise_metadata_schedule_3776_0_e34849;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3777_0_e34893,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3777_0_e34859: f64 = (-50.0);
        let (noise_metadata_schedule_3777_0_e34891,) = {
            if ((!(w[2212] > 50.0)) && (!(w[2212] < noise_metadata_schedule_3777_0_e34859))) {
                let noise_metadata_schedule_3777_0_e34864: f64 = (w[2212]).exp();
                (noise_metadata_schedule_3777_0_e34864,)
            } else {
                let noise_metadata_schedule_3777_0_e34871: f64 = (-50.0);
                let (noise_metadata_schedule_3777_0_e34890,) = {
                    if ((!(w[2212] > 50.0)) && (w[2212] < noise_metadata_schedule_3777_0_e34871)) {
                        let noise_metadata_schedule_3777_0_e34875: f64 = (-50.0);
                        let noise_metadata_schedule_3777_0_e34876: f64 = (noise_metadata_schedule_3777_0_e34875).exp();
                        (noise_metadata_schedule_3777_0_e34876,)
                    } else {
                        let (noise_metadata_schedule_3777_0_e34889,) = {
                            if (w[2212] > 50.0) {
                                let noise_metadata_schedule_3777_0_e34881: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3777_0_e34885: f64 = (w[2212] - 50.0);
                                let noise_metadata_schedule_3777_0_e34886: f64 = (1.0 + noise_metadata_schedule_3777_0_e34885);
                                let noise_metadata_schedule_3777_0_e34887: f64 = (noise_metadata_schedule_3777_0_e34881 * noise_metadata_schedule_3777_0_e34886);
                                (noise_metadata_schedule_3777_0_e34887,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3777_0_e34889,)
                    }
                };
                (noise_metadata_schedule_3777_0_e34890,)
            }
        };
        (noise_metadata_schedule_3777_0_e34891,)
    } else {
        (w[2202],)
    }
};
            w[2202] = noise_metadata_schedule_3777_0_e34893;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3778_0_e34906,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3778_0_e34899: f64 = (-w[2180]);
        let noise_metadata_schedule_3778_0_e34901: f64 = (noise_metadata_schedule_3778_0_e34899 - w[2187]);
        let noise_metadata_schedule_3778_0_e34902: f64 = (w[2186] * noise_metadata_schedule_3778_0_e34901);
        let noise_metadata_schedule_3778_0_e34904: f64 = (noise_metadata_schedule_3778_0_e34902 + w[2212]);
        (noise_metadata_schedule_3778_0_e34904,)
    } else {
        (w[2208],)
    }
};
            w[2208] = noise_metadata_schedule_3778_0_e34906;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3779_0_e34917,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3779_0_e34911: f64 = (-w[2186]);
        let noise_metadata_schedule_3779_0_e34913: f64 = (noise_metadata_schedule_3779_0_e34911 * w[2187]);
        let noise_metadata_schedule_3779_0_e34915: f64 = (noise_metadata_schedule_3779_0_e34913 + w[2212]);
        (noise_metadata_schedule_3779_0_e34915,)
    } else {
        (w[2209],)
    }
};
            w[2209] = noise_metadata_schedule_3779_0_e34917;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3780_0_e34961,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3780_0_e34927: f64 = (-50.0);
        let (noise_metadata_schedule_3780_0_e34959,) = {
            if ((!(w[2208] > 50.0)) && (!(w[2208] < noise_metadata_schedule_3780_0_e34927))) {
                let noise_metadata_schedule_3780_0_e34932: f64 = (w[2208]).exp();
                (noise_metadata_schedule_3780_0_e34932,)
            } else {
                let noise_metadata_schedule_3780_0_e34939: f64 = (-50.0);
                let (noise_metadata_schedule_3780_0_e34958,) = {
                    if ((!(w[2208] > 50.0)) && (w[2208] < noise_metadata_schedule_3780_0_e34939)) {
                        let noise_metadata_schedule_3780_0_e34943: f64 = (-50.0);
                        let noise_metadata_schedule_3780_0_e34944: f64 = (noise_metadata_schedule_3780_0_e34943).exp();
                        (noise_metadata_schedule_3780_0_e34944,)
                    } else {
                        let (noise_metadata_schedule_3780_0_e34957,) = {
                            if (w[2208] > 50.0) {
                                let noise_metadata_schedule_3780_0_e34949: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3780_0_e34953: f64 = (w[2208] - 50.0);
                                let noise_metadata_schedule_3780_0_e34954: f64 = (1.0 + noise_metadata_schedule_3780_0_e34953);
                                let noise_metadata_schedule_3780_0_e34955: f64 = (noise_metadata_schedule_3780_0_e34949 * noise_metadata_schedule_3780_0_e34954);
                                (noise_metadata_schedule_3780_0_e34955,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3780_0_e34957,)
                    }
                };
                (noise_metadata_schedule_3780_0_e34958,)
            }
        };
        (noise_metadata_schedule_3780_0_e34959,)
    } else {
        (w[2210],)
    }
};
            w[2210] = noise_metadata_schedule_3780_0_e34961;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3781_0_e35005,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3781_0_e34971: f64 = (-50.0);
        let (noise_metadata_schedule_3781_0_e35003,) = {
            if ((!(w[2209] > 50.0)) && (!(w[2209] < noise_metadata_schedule_3781_0_e34971))) {
                let noise_metadata_schedule_3781_0_e34976: f64 = (w[2209]).exp();
                (noise_metadata_schedule_3781_0_e34976,)
            } else {
                let noise_metadata_schedule_3781_0_e34983: f64 = (-50.0);
                let (noise_metadata_schedule_3781_0_e35002,) = {
                    if ((!(w[2209] > 50.0)) && (w[2209] < noise_metadata_schedule_3781_0_e34983)) {
                        let noise_metadata_schedule_3781_0_e34987: f64 = (-50.0);
                        let noise_metadata_schedule_3781_0_e34988: f64 = (noise_metadata_schedule_3781_0_e34987).exp();
                        (noise_metadata_schedule_3781_0_e34988,)
                    } else {
                        let (noise_metadata_schedule_3781_0_e35001,) = {
                            if (w[2209] > 50.0) {
                                let noise_metadata_schedule_3781_0_e34993: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3781_0_e34997: f64 = (w[2209] - 50.0);
                                let noise_metadata_schedule_3781_0_e34998: f64 = (1.0 + noise_metadata_schedule_3781_0_e34997);
                                let noise_metadata_schedule_3781_0_e34999: f64 = (noise_metadata_schedule_3781_0_e34993 * noise_metadata_schedule_3781_0_e34998);
                                (noise_metadata_schedule_3781_0_e34999,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3781_0_e35001,)
                    }
                };
                (noise_metadata_schedule_3781_0_e35002,)
            }
        };
        (noise_metadata_schedule_3781_0_e35003,)
    } else {
        (w[2211],)
    }
};
            w[2211] = noise_metadata_schedule_3781_0_e35005;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3782_0_e35013,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3782_0_e35011: f64 = (w[2210] - w[2211]);
        (noise_metadata_schedule_3782_0_e35011,)
    } else {
        (w[2204],)
    }
};
            w[2204] = noise_metadata_schedule_3782_0_e35013;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3783_0_e35027,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3783_0_e35019: f64 = (w[2199] * w[2189]);
        let noise_metadata_schedule_3783_0_e35021: f64 = (noise_metadata_schedule_3783_0_e35019 * w[2190]);
        let noise_metadata_schedule_3783_0_e35023: f64 = (noise_metadata_schedule_3783_0_e35021 * w[2191]);
        let noise_metadata_schedule_3783_0_e35025: f64 = (noise_metadata_schedule_3783_0_e35023 * w[2188]);
        (noise_metadata_schedule_3783_0_e35025,)
    } else {
        (w[2178],)
    }
};
            w[2178] = noise_metadata_schedule_3783_0_e35027;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3784_0_e35039,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3784_0_e35033: f64 = (w[2185] / w[2181]);
        let noise_metadata_schedule_3784_0_e35035: f64 = (noise_metadata_schedule_3784_0_e35033 * w[2180]);
        let noise_metadata_schedule_3784_0_e35037: f64 = (noise_metadata_schedule_3784_0_e35035 + w[2212]);
        (noise_metadata_schedule_3784_0_e35037,)
    } else {
        (w[2214],)
    }
};
            w[2214] = noise_metadata_schedule_3784_0_e35039;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3785_0_e35083,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3785_0_e35049: f64 = (-50.0);
        let (noise_metadata_schedule_3785_0_e35081,) = {
            if ((!(w[2214] > 50.0)) && (!(w[2214] < noise_metadata_schedule_3785_0_e35049))) {
                let noise_metadata_schedule_3785_0_e35054: f64 = (w[2214]).exp();
                (noise_metadata_schedule_3785_0_e35054,)
            } else {
                let noise_metadata_schedule_3785_0_e35061: f64 = (-50.0);
                let (noise_metadata_schedule_3785_0_e35080,) = {
                    if ((!(w[2214] > 50.0)) && (w[2214] < noise_metadata_schedule_3785_0_e35061)) {
                        let noise_metadata_schedule_3785_0_e35065: f64 = (-50.0);
                        let noise_metadata_schedule_3785_0_e35066: f64 = (noise_metadata_schedule_3785_0_e35065).exp();
                        (noise_metadata_schedule_3785_0_e35066,)
                    } else {
                        let (noise_metadata_schedule_3785_0_e35079,) = {
                            if (w[2214] > 50.0) {
                                let noise_metadata_schedule_3785_0_e35071: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3785_0_e35075: f64 = (w[2214] - 50.0);
                                let noise_metadata_schedule_3785_0_e35076: f64 = (1.0 + noise_metadata_schedule_3785_0_e35075);
                                let noise_metadata_schedule_3785_0_e35077: f64 = (noise_metadata_schedule_3785_0_e35071 * noise_metadata_schedule_3785_0_e35076);
                                (noise_metadata_schedule_3785_0_e35077,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3785_0_e35079,)
                    }
                };
                (noise_metadata_schedule_3785_0_e35080,)
            }
        };
        (noise_metadata_schedule_3785_0_e35081,)
    } else {
        (w[2215],)
    }
};
            w[2215] = noise_metadata_schedule_3785_0_e35083;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_3786_0_e35086: f64 = if w[2184] == 1.0 { 1.0 } else { 0.0 };
            w[2233] = noise_metadata_schedule_3786_0_e35086;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3787_0_e35102,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] != 0.0)) {
        let noise_metadata_schedule_3787_0_e35096: f64 = (w[2192] * w[2204]);
        let noise_metadata_schedule_3787_0_e35097: f64 = (w[2215] - noise_metadata_schedule_3787_0_e35096);
        let noise_metadata_schedule_3787_0_e35099: f64 = (noise_metadata_schedule_3787_0_e35097 - w[2202]);
        let noise_metadata_schedule_3787_0_e35100: f64 = (w[2178] * noise_metadata_schedule_3787_0_e35099);
        (noise_metadata_schedule_3787_0_e35100,)
    } else {
        (w[2205],)
    }
};
            w[2205] = noise_metadata_schedule_3787_0_e35102;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3788_0_e35118,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3788_0_e35111: f64 = (-w[2182]);
        let noise_metadata_schedule_3788_0_e35113: f64 = (noise_metadata_schedule_3788_0_e35111 - w[2187]);
        let noise_metadata_schedule_3788_0_e35114: f64 = (w[2186] * noise_metadata_schedule_3788_0_e35113);
        let noise_metadata_schedule_3788_0_e35116: f64 = (noise_metadata_schedule_3788_0_e35114 + w[2212]);
        (noise_metadata_schedule_3788_0_e35116,)
    } else {
        (w[2219],)
    }
};
            w[2219] = noise_metadata_schedule_3788_0_e35118;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3789_0_e35165,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3789_0_e35131: f64 = (-50.0);
        let (noise_metadata_schedule_3789_0_e35163,) = {
            if ((!(w[2219] > 50.0)) && (!(w[2219] < noise_metadata_schedule_3789_0_e35131))) {
                let noise_metadata_schedule_3789_0_e35136: f64 = (w[2219]).exp();
                (noise_metadata_schedule_3789_0_e35136,)
            } else {
                let noise_metadata_schedule_3789_0_e35143: f64 = (-50.0);
                let (noise_metadata_schedule_3789_0_e35162,) = {
                    if ((!(w[2219] > 50.0)) && (w[2219] < noise_metadata_schedule_3789_0_e35143)) {
                        let noise_metadata_schedule_3789_0_e35147: f64 = (-50.0);
                        let noise_metadata_schedule_3789_0_e35148: f64 = (noise_metadata_schedule_3789_0_e35147).exp();
                        (noise_metadata_schedule_3789_0_e35148,)
                    } else {
                        let (noise_metadata_schedule_3789_0_e35161,) = {
                            if (w[2219] > 50.0) {
                                let noise_metadata_schedule_3789_0_e35153: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3789_0_e35157: f64 = (w[2219] - 50.0);
                                let noise_metadata_schedule_3789_0_e35158: f64 = (1.0 + noise_metadata_schedule_3789_0_e35157);
                                let noise_metadata_schedule_3789_0_e35159: f64 = (noise_metadata_schedule_3789_0_e35153 * noise_metadata_schedule_3789_0_e35158);
                                (noise_metadata_schedule_3789_0_e35159,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3789_0_e35161,)
                    }
                };
                (noise_metadata_schedule_3789_0_e35162,)
            }
        };
        (noise_metadata_schedule_3789_0_e35163,)
    } else {
        (w[2220],)
    }
};
            w[2220] = noise_metadata_schedule_3789_0_e35165;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3790_0_e35176,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3790_0_e35174: f64 = (w[2220] - w[2211]);
        (noise_metadata_schedule_3790_0_e35174,)
    } else {
        (w[2221],)
    }
};
            w[2221] = noise_metadata_schedule_3790_0_e35176;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3791_0_e35191,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3791_0_e35185: f64 = (w[2185] / w[2181]);
        let noise_metadata_schedule_3791_0_e35187: f64 = (noise_metadata_schedule_3791_0_e35185 * w[2182]);
        let noise_metadata_schedule_3791_0_e35189: f64 = (noise_metadata_schedule_3791_0_e35187 + w[2212]);
        (noise_metadata_schedule_3791_0_e35189,)
    } else {
        (w[2222],)
    }
};
            w[2222] = noise_metadata_schedule_3791_0_e35191;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_11(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3792_0_e35238,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3792_0_e35204: f64 = (-50.0);
        let (noise_metadata_schedule_3792_0_e35236,) = {
            if ((!(w[2222] > 50.0)) && (!(w[2222] < noise_metadata_schedule_3792_0_e35204))) {
                let noise_metadata_schedule_3792_0_e35209: f64 = (w[2222]).exp();
                (noise_metadata_schedule_3792_0_e35209,)
            } else {
                let noise_metadata_schedule_3792_0_e35216: f64 = (-50.0);
                let (noise_metadata_schedule_3792_0_e35235,) = {
                    if ((!(w[2222] > 50.0)) && (w[2222] < noise_metadata_schedule_3792_0_e35216)) {
                        let noise_metadata_schedule_3792_0_e35220: f64 = (-50.0);
                        let noise_metadata_schedule_3792_0_e35221: f64 = (noise_metadata_schedule_3792_0_e35220).exp();
                        (noise_metadata_schedule_3792_0_e35221,)
                    } else {
                        let (noise_metadata_schedule_3792_0_e35234,) = {
                            if (w[2222] > 50.0) {
                                let noise_metadata_schedule_3792_0_e35226: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3792_0_e35230: f64 = (w[2222] - 50.0);
                                let noise_metadata_schedule_3792_0_e35231: f64 = (1.0 + noise_metadata_schedule_3792_0_e35230);
                                let noise_metadata_schedule_3792_0_e35232: f64 = (noise_metadata_schedule_3792_0_e35226 * noise_metadata_schedule_3792_0_e35231);
                                (noise_metadata_schedule_3792_0_e35232,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3792_0_e35234,)
                    }
                };
                (noise_metadata_schedule_3792_0_e35235,)
            }
        };
        (noise_metadata_schedule_3792_0_e35236,)
    } else {
        (w[2223],)
    }
};
            w[2223] = noise_metadata_schedule_3792_0_e35238;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3793_0_e35253,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3793_0_e35248: f64 = (w[2192] * w[2221]);
        let noise_metadata_schedule_3793_0_e35249: f64 = (w[2223] - noise_metadata_schedule_3793_0_e35248);
        let noise_metadata_schedule_3793_0_e35251: f64 = (noise_metadata_schedule_3793_0_e35249 - w[2202]);
        (noise_metadata_schedule_3793_0_e35251,)
    } else {
        (w[2224],)
    }
};
            w[2224] = noise_metadata_schedule_3793_0_e35253;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3794_0_e35270,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3794_0_e35264: f64 = (w[2192] * w[2204]);
        let noise_metadata_schedule_3794_0_e35265: f64 = (w[2215] - noise_metadata_schedule_3794_0_e35264);
        let noise_metadata_schedule_3794_0_e35267: f64 = (noise_metadata_schedule_3794_0_e35265 - w[2202]);
        let noise_metadata_schedule_3794_0_e35268: f64 = (w[2178] * noise_metadata_schedule_3794_0_e35267);
        (noise_metadata_schedule_3794_0_e35268,)
    } else {
        (w[2225],)
    }
};
            w[2225] = noise_metadata_schedule_3794_0_e35270;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_3795_0_e35273: f64 = if w[2184] > 0.0 { 1.0 } else { 0.0 };
            w[2234] = noise_metadata_schedule_3795_0_e35273;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3796_0_e35286,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3796_0_e35284: f64 = (w[2184] * w[2185]);
        (noise_metadata_schedule_3796_0_e35284,)
    } else {
        (w[2218],)
    }
};
            w[2218] = noise_metadata_schedule_3796_0_e35286;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3797_0_e35303,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3797_0_e35297: f64 = (w[2218] / w[2181]);
        let noise_metadata_schedule_3797_0_e35299: f64 = (noise_metadata_schedule_3797_0_e35297 * w[2182]);
        let noise_metadata_schedule_3797_0_e35301: f64 = (noise_metadata_schedule_3797_0_e35299 + w[2212]);
        (noise_metadata_schedule_3797_0_e35301,)
    } else {
        (w[2226],)
    }
};
            w[2226] = noise_metadata_schedule_3797_0_e35303;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3798_0_e35352,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3798_0_e35318: f64 = (-50.0);
        let (noise_metadata_schedule_3798_0_e35350,) = {
            if ((!(w[2226] > 50.0)) && (!(w[2226] < noise_metadata_schedule_3798_0_e35318))) {
                let noise_metadata_schedule_3798_0_e35323: f64 = (w[2226]).exp();
                (noise_metadata_schedule_3798_0_e35323,)
            } else {
                let noise_metadata_schedule_3798_0_e35330: f64 = (-50.0);
                let (noise_metadata_schedule_3798_0_e35349,) = {
                    if ((!(w[2226] > 50.0)) && (w[2226] < noise_metadata_schedule_3798_0_e35330)) {
                        let noise_metadata_schedule_3798_0_e35334: f64 = (-50.0);
                        let noise_metadata_schedule_3798_0_e35335: f64 = (noise_metadata_schedule_3798_0_e35334).exp();
                        (noise_metadata_schedule_3798_0_e35335,)
                    } else {
                        let (noise_metadata_schedule_3798_0_e35348,) = {
                            if (w[2226] > 50.0) {
                                let noise_metadata_schedule_3798_0_e35340: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3798_0_e35344: f64 = (w[2226] - 50.0);
                                let noise_metadata_schedule_3798_0_e35345: f64 = (1.0 + noise_metadata_schedule_3798_0_e35344);
                                let noise_metadata_schedule_3798_0_e35346: f64 = (noise_metadata_schedule_3798_0_e35340 * noise_metadata_schedule_3798_0_e35345);
                                (noise_metadata_schedule_3798_0_e35346,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3798_0_e35348,)
                    }
                };
                (noise_metadata_schedule_3798_0_e35349,)
            }
        };
        (noise_metadata_schedule_3798_0_e35350,)
    } else {
        (w[2227],)
    }
};
            w[2227] = noise_metadata_schedule_3798_0_e35352;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3799_0_e35369,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3799_0_e35364: f64 = (w[2192] * w[2221]);
        let noise_metadata_schedule_3799_0_e35365: f64 = (w[2227] - noise_metadata_schedule_3799_0_e35364);
        let noise_metadata_schedule_3799_0_e35367: f64 = (noise_metadata_schedule_3799_0_e35365 - w[2202]);
        (noise_metadata_schedule_3799_0_e35367,)
    } else {
        (w[2228],)
    }
};
            w[2228] = noise_metadata_schedule_3799_0_e35369;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3800_0_e35386,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3800_0_e35380: f64 = (w[2218] / w[2181]);
        let noise_metadata_schedule_3800_0_e35382: f64 = (noise_metadata_schedule_3800_0_e35380 * w[2180]);
        let noise_metadata_schedule_3800_0_e35384: f64 = (noise_metadata_schedule_3800_0_e35382 + w[2212]);
        (noise_metadata_schedule_3800_0_e35384,)
    } else {
        (w[2229],)
    }
};
            w[2229] = noise_metadata_schedule_3800_0_e35386;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3801_0_e35435,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3801_0_e35401: f64 = (-50.0);
        let (noise_metadata_schedule_3801_0_e35433,) = {
            if ((!(w[2229] > 50.0)) && (!(w[2229] < noise_metadata_schedule_3801_0_e35401))) {
                let noise_metadata_schedule_3801_0_e35406: f64 = (w[2229]).exp();
                (noise_metadata_schedule_3801_0_e35406,)
            } else {
                let noise_metadata_schedule_3801_0_e35413: f64 = (-50.0);
                let (noise_metadata_schedule_3801_0_e35432,) = {
                    if ((!(w[2229] > 50.0)) && (w[2229] < noise_metadata_schedule_3801_0_e35413)) {
                        let noise_metadata_schedule_3801_0_e35417: f64 = (-50.0);
                        let noise_metadata_schedule_3801_0_e35418: f64 = (noise_metadata_schedule_3801_0_e35417).exp();
                        (noise_metadata_schedule_3801_0_e35418,)
                    } else {
                        let (noise_metadata_schedule_3801_0_e35431,) = {
                            if (w[2229] > 50.0) {
                                let noise_metadata_schedule_3801_0_e35423: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3801_0_e35427: f64 = (w[2229] - 50.0);
                                let noise_metadata_schedule_3801_0_e35428: f64 = (1.0 + noise_metadata_schedule_3801_0_e35427);
                                let noise_metadata_schedule_3801_0_e35429: f64 = (noise_metadata_schedule_3801_0_e35423 * noise_metadata_schedule_3801_0_e35428);
                                (noise_metadata_schedule_3801_0_e35429,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3801_0_e35431,)
                    }
                };
                (noise_metadata_schedule_3801_0_e35432,)
            }
        };
        (noise_metadata_schedule_3801_0_e35433,)
    } else {
        (w[2230],)
    }
};
            w[2230] = noise_metadata_schedule_3801_0_e35435;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3802_0_e35450,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3802_0_e35446: f64 = (w[2178] * w[2224]);
        let noise_metadata_schedule_3802_0_e35448: f64 = (noise_metadata_schedule_3802_0_e35446 / w[2228]);
        (noise_metadata_schedule_3802_0_e35448,)
    } else {
        (w[2231],)
    }
};
            w[2231] = noise_metadata_schedule_3802_0_e35450;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3803_0_e35469,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] != 0.0)) {
        let noise_metadata_schedule_3803_0_e35463: f64 = (w[2192] * w[2204]);
        let noise_metadata_schedule_3803_0_e35464: f64 = (w[2230] - noise_metadata_schedule_3803_0_e35463);
        let noise_metadata_schedule_3803_0_e35466: f64 = (noise_metadata_schedule_3803_0_e35464 - w[2202]);
        let noise_metadata_schedule_3803_0_e35467: f64 = (w[2231] * noise_metadata_schedule_3803_0_e35466);
        (noise_metadata_schedule_3803_0_e35467,)
    } else {
        (w[2232],)
    }
};
            w[2232] = noise_metadata_schedule_3803_0_e35469;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3804_0_e35483,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2234] == 0.0)) {
        let noise_metadata_schedule_3804_0_e35481: f64 = (w[2178] * w[2224]);
        (noise_metadata_schedule_3804_0_e35481,)
    } else {
        (w[2232],)
    }
};
            w[2232] = noise_metadata_schedule_3804_0_e35483;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3805_0_e35496,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3805_0_e35492: f64 = (w[2183] * w[2183]);
        let noise_metadata_schedule_3805_0_e35494: f64 = (noise_metadata_schedule_3805_0_e35492 * w[2181]);
        (noise_metadata_schedule_3805_0_e35494,)
    } else {
        (w[2201],)
    }
};
            w[2201] = noise_metadata_schedule_3805_0_e35496;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3806_0_e35513,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3806_0_e35507: f64 = (w[2201] / 2.0);
        let noise_metadata_schedule_3806_0_e35508: f64 = (w[2182] - noise_metadata_schedule_3806_0_e35507);
        let noise_metadata_schedule_3806_0_e35509: f64 = (w[2180] - noise_metadata_schedule_3806_0_e35508);
        let noise_metadata_schedule_3806_0_e35511: f64 = (noise_metadata_schedule_3806_0_e35509 / w[2201]);
        (noise_metadata_schedule_3806_0_e35511,)
    } else {
        (w[2213],)
    }
};
            w[2213] = noise_metadata_schedule_3806_0_e35513;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_3807_0_e35516: f64 = if w[2213] > 50.0 { 1.0 } else { 0.0 };
            w[2235] = noise_metadata_schedule_3807_0_e35516;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3808_0_e35527,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2235] != 0.0)) {
        (0.0,)
    } else {
        (w[2203],)
    }
};
            w[2203] = noise_metadata_schedule_3808_0_e35527;
        }
        if (active[0] & 0x1) != 0 {
            let noise_metadata_schedule_3809_0_e35530: f64 = (-50.0);
            let noise_metadata_schedule_3809_0_e35531: f64 = if w[2213] < noise_metadata_schedule_3809_0_e35530 { 1.0 } else { 0.0 };
            w[2236] = noise_metadata_schedule_3809_0_e35531;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3810_0_e35545,) = {
    if (((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2235] == 0.0)) && (w[2236] != 0.0)) {
        (1.0,)
    } else {
        (w[2203],)
    }
};
            w[2203] = noise_metadata_schedule_3810_0_e35545;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3811_0_e35565,) = {
    if (((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) && (w[2235] == 0.0)) && (w[2236] == 0.0)) {
        let noise_metadata_schedule_3811_0_e35561: f64 = (w[2213]).exp();
        let noise_metadata_schedule_3811_0_e35562: f64 = (1.0 + noise_metadata_schedule_3811_0_e35561);
        let noise_metadata_schedule_3811_0_e35563: f64 = (1.0 / noise_metadata_schedule_3811_0_e35562);
        (noise_metadata_schedule_3811_0_e35563,)
    } else {
        (w[2203],)
    }
};
            w[2203] = noise_metadata_schedule_3811_0_e35565;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3812_0_e35582,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2233] == 0.0)) {
        let noise_metadata_schedule_3812_0_e35574: f64 = (w[2203] * w[2225]);
        let noise_metadata_schedule_3812_0_e35577: f64 = (1.0 - w[2203]);
        let noise_metadata_schedule_3812_0_e35579: f64 = (noise_metadata_schedule_3812_0_e35577 * w[2232]);
        let noise_metadata_schedule_3812_0_e35580: f64 = (noise_metadata_schedule_3812_0_e35574 + noise_metadata_schedule_3812_0_e35579);
        (noise_metadata_schedule_3812_0_e35580,)
    } else {
        (w[2205],)
    }
};
            w[2205] = noise_metadata_schedule_3812_0_e35582;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3813_0_e35630,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3813_0_e35587: f64 = (-w[2180]);
        let (noise_metadata_schedule_3813_0_e35620,) = {
            if (params.p52 != 0.0) {
                let noise_metadata_schedule_3813_0_e35595: f64 = (w[2180] / w[2193]);
                let noise_metadata_schedule_3813_0_e35598: f64 = (0.001 / params.p53);
                let noise_metadata_schedule_3813_0_e35601: f64 = (w[2180] / w[2193]);
                let noise_metadata_schedule_3813_0_e35602: f64 = (noise_metadata_schedule_3813_0_e35598 * noise_metadata_schedule_3813_0_e35601);
                let noise_metadata_schedule_3813_0_e35603: f64 = (noise_metadata_schedule_3813_0_e35602).tanh();
                let noise_metadata_schedule_3813_0_e35604: f64 = (noise_metadata_schedule_3813_0_e35595 * noise_metadata_schedule_3813_0_e35603);
                (noise_metadata_schedule_3813_0_e35604,)
            } else {
                let (noise_metadata_schedule_3813_0_e35619,) = {
                    if (params.p52 == 0.0) {
                        let noise_metadata_schedule_3813_0_e35610: f64 = (w[2180] / w[2193]);
                        let noise_metadata_schedule_3813_0_e35613: f64 = (w[2180] / w[2193]);
                        let noise_metadata_schedule_3813_0_e35614: f64 = (noise_metadata_schedule_3813_0_e35610 * noise_metadata_schedule_3813_0_e35613);
                        let noise_metadata_schedule_3813_0_e35616: f64 = (noise_metadata_schedule_3813_0_e35614 + params.p53);
                        let noise_metadata_schedule_3813_0_e35617: f64 = (noise_metadata_schedule_3813_0_e35616).sqrt();
                        (noise_metadata_schedule_3813_0_e35617,)
                    } else {
                        (0.0,)
                    }
                };
                (noise_metadata_schedule_3813_0_e35619,)
            }
        };
        let noise_metadata_schedule_3813_0_e35622: f64 = (noise_metadata_schedule_3813_0_e35620).powf(w[2194]);
        let noise_metadata_schedule_3813_0_e35623: f64 = (1.0 + noise_metadata_schedule_3813_0_e35622);
        let noise_metadata_schedule_3813_0_e35626: f64 = (1.0 / w[2194]);
        let noise_metadata_schedule_3813_0_e35627: f64 = (noise_metadata_schedule_3813_0_e35623).powf(noise_metadata_schedule_3813_0_e35626);
        let noise_metadata_schedule_3813_0_e35628: f64 = (noise_metadata_schedule_3813_0_e35587 / noise_metadata_schedule_3813_0_e35627);
        (noise_metadata_schedule_3813_0_e35628,)
    } else {
        (w[2206],)
    }
};
            w[2206] = noise_metadata_schedule_3813_0_e35630;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3814_0_e35647,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3814_0_e35635: f64 = (-w[2199]);
        let noise_metadata_schedule_3814_0_e35637: f64 = (noise_metadata_schedule_3814_0_e35635 * w[2189]);
        let noise_metadata_schedule_3814_0_e35639: f64 = (noise_metadata_schedule_3814_0_e35637 * w[2190]);
        let noise_metadata_schedule_3814_0_e35641: f64 = (noise_metadata_schedule_3814_0_e35639 * w[2195]);
        let noise_metadata_schedule_3814_0_e35643: f64 = (noise_metadata_schedule_3814_0_e35641 * w[2188]);
        let noise_metadata_schedule_3814_0_e35645: f64 = noise_metadata_schedule_3814_0_e35643;
        (noise_metadata_schedule_3814_0_e35645,)
    } else {
        (w[2179],)
    }
};
            w[2179] = noise_metadata_schedule_3814_0_e35647;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3815_0_e35657,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3815_0_e35653: f64 = (w[2196] / w[2181]);
        let noise_metadata_schedule_3815_0_e35655: f64 = (noise_metadata_schedule_3815_0_e35653 * w[2206]);
        (noise_metadata_schedule_3815_0_e35655,)
    } else {
        (w[2216],)
    }
};
            w[2216] = noise_metadata_schedule_3815_0_e35657;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3816_0_e35701,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3816_0_e35667: f64 = (-50.0);
        let (noise_metadata_schedule_3816_0_e35699,) = {
            if ((!(w[2216] > 50.0)) && (!(w[2216] < noise_metadata_schedule_3816_0_e35667))) {
                let noise_metadata_schedule_3816_0_e35672: f64 = (w[2216]).exp();
                (noise_metadata_schedule_3816_0_e35672,)
            } else {
                let noise_metadata_schedule_3816_0_e35679: f64 = (-50.0);
                let (noise_metadata_schedule_3816_0_e35698,) = {
                    if ((!(w[2216] > 50.0)) && (w[2216] < noise_metadata_schedule_3816_0_e35679)) {
                        let noise_metadata_schedule_3816_0_e35683: f64 = (-50.0);
                        let noise_metadata_schedule_3816_0_e35684: f64 = (noise_metadata_schedule_3816_0_e35683).exp();
                        (noise_metadata_schedule_3816_0_e35684,)
                    } else {
                        let (noise_metadata_schedule_3816_0_e35697,) = {
                            if (w[2216] > 50.0) {
                                let noise_metadata_schedule_3816_0_e35689: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3816_0_e35693: f64 = (w[2216] - 50.0);
                                let noise_metadata_schedule_3816_0_e35694: f64 = (1.0 + noise_metadata_schedule_3816_0_e35693);
                                let noise_metadata_schedule_3816_0_e35695: f64 = (noise_metadata_schedule_3816_0_e35689 * noise_metadata_schedule_3816_0_e35694);
                                (noise_metadata_schedule_3816_0_e35695,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3816_0_e35697,)
                    }
                };
                (noise_metadata_schedule_3816_0_e35698,)
            }
        };
        (noise_metadata_schedule_3816_0_e35699,)
    } else {
        (w[2217],)
    }
};
            w[2217] = noise_metadata_schedule_3816_0_e35701;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3817_0_e35711,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3817_0_e35708: f64 = (w[2217] - 1.0);
        let noise_metadata_schedule_3817_0_e35709: f64 = (w[2179] * noise_metadata_schedule_3817_0_e35708);
        (noise_metadata_schedule_3817_0_e35709,)
    } else {
        (w[2207],)
    }
};
            w[2207] = noise_metadata_schedule_3817_0_e35711;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3818_0_e35719,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3818_0_e35717: f64 = (w[2205] + w[2207]);
        (noise_metadata_schedule_3818_0_e35717,)
    } else {
        (w[2200],)
    }
};
            w[2200] = noise_metadata_schedule_3818_0_e35719;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3819_0_e35725,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2200],)
    } else {
        (w[2177],)
    }
};
            w[2177] = noise_metadata_schedule_3819_0_e35725;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3820_0_e35731,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2178],)
    } else {
        (w[130],)
    }
};
            w[130] = noise_metadata_schedule_3820_0_e35731;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3821_0_e35737,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2179],)
    } else {
        (w[132],)
    }
};
            w[132] = noise_metadata_schedule_3821_0_e35737;
        }
        if (active[0] & 0x1) != 0 {
            let (noise_metadata_schedule_3822_0_e35743,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2177],)
    } else {
        (w[128],)
    }
};
            w[128] = noise_metadata_schedule_3822_0_e35743;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3823_0_e35749,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2237],)
    }
};
            w[2237] = noise_metadata_schedule_3823_0_e35749;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3824_0_e35755,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2238],)
    }
};
            w[2238] = noise_metadata_schedule_3824_0_e35755;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_12(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3825_0_e35761,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2239],)
    }
};
            w[2239] = noise_metadata_schedule_3825_0_e35761;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3826_0_e35769,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3826_0_e35767: f64 = (params.p6 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[5])));
        (noise_metadata_schedule_3826_0_e35767,)
    } else {
        (w[2240],)
    }
};
            w[2240] = noise_metadata_schedule_3826_0_e35769;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3827_0_e35775,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[113],)
    } else {
        (w[2241],)
    }
};
            w[2241] = noise_metadata_schedule_3827_0_e35775;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3828_0_e35781,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p265,)
    } else {
        (w[2242],)
    }
};
            w[2242] = noise_metadata_schedule_3828_0_e35781;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3829_0_e35787,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p267,)
    } else {
        (w[2243],)
    }
};
            w[2243] = noise_metadata_schedule_3829_0_e35787;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3830_0_e35793,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p266,)
    } else {
        (w[2244],)
    }
};
            w[2244] = noise_metadata_schedule_3830_0_e35793;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3831_0_e35799,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p263,)
    } else {
        (w[2245],)
    }
};
            w[2245] = noise_metadata_schedule_3831_0_e35799;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3832_0_e35805,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p281,)
    } else {
        (w[2246],)
    }
};
            w[2246] = noise_metadata_schedule_3832_0_e35805;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3833_0_e35811,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p280,)
    } else {
        (w[2247],)
    }
};
            w[2247] = noise_metadata_schedule_3833_0_e35811;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3834_0_e35817,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[112],)
    } else {
        (w[2248],)
    }
};
            w[2248] = noise_metadata_schedule_3834_0_e35817;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3835_0_e35823,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p0,)
    } else {
        (w[2249],)
    }
};
            w[2249] = noise_metadata_schedule_3835_0_e35823;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3836_0_e35829,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p2,)
    } else {
        (w[2250],)
    }
};
            w[2250] = noise_metadata_schedule_3836_0_e35829;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3837_0_e35837,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3837_0_e35835: f64 = (params.p255 * params.p264);
        (noise_metadata_schedule_3837_0_e35835,)
    } else {
        (w[2251],)
    }
};
            w[2251] = noise_metadata_schedule_3837_0_e35837;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3838_0_e35843,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p279,)
    } else {
        (w[2252],)
    }
};
            w[2252] = noise_metadata_schedule_3838_0_e35843;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3839_0_e35849,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p274,)
    } else {
        (w[2253],)
    }
};
            w[2253] = noise_metadata_schedule_3839_0_e35849;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3840_0_e35855,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p275,)
    } else {
        (w[2254],)
    }
};
            w[2254] = noise_metadata_schedule_3840_0_e35855;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3841_0_e35863,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3841_0_e35861: f64 = (params.p255 * params.p273);
        (noise_metadata_schedule_3841_0_e35861,)
    } else {
        (w[2255],)
    }
};
            w[2255] = noise_metadata_schedule_3841_0_e35863;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3842_0_e35869,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p272,)
    } else {
        (w[2256],)
    }
};
            w[2256] = noise_metadata_schedule_3842_0_e35869;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3843_0_e35875,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p257,)
    } else {
        (w[2257],)
    }
};
            w[2257] = noise_metadata_schedule_3843_0_e35875;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3844_0_e35881,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p256,)
    } else {
        (w[2258],)
    }
};
            w[2258] = noise_metadata_schedule_3844_0_e35881;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3845_0_e35887,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (params.p6,)
    } else {
        (w[2259],)
    }
};
            w[2259] = noise_metadata_schedule_3845_0_e35887;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3846_0_e35893,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2260],)
    }
};
            w[2260] = noise_metadata_schedule_3846_0_e35893;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3847_0_e35899,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2261],)
    }
};
            w[2261] = noise_metadata_schedule_3847_0_e35899;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3848_0_e35905,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2262],)
    }
};
            w[2262] = noise_metadata_schedule_3848_0_e35905;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3849_0_e35911,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2263],)
    }
};
            w[2263] = noise_metadata_schedule_3849_0_e35911;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3850_0_e35917,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2264],)
    }
};
            w[2264] = noise_metadata_schedule_3850_0_e35917;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3851_0_e35923,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2265],)
    }
};
            w[2265] = noise_metadata_schedule_3851_0_e35923;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3852_0_e35929,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2266],)
    }
};
            w[2266] = noise_metadata_schedule_3852_0_e35929;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3853_0_e35935,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2267],)
    }
};
            w[2267] = noise_metadata_schedule_3853_0_e35935;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3854_0_e35941,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2268],)
    }
};
            w[2268] = noise_metadata_schedule_3854_0_e35941;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3855_0_e35947,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2269],)
    }
};
            w[2269] = noise_metadata_schedule_3855_0_e35947;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3856_0_e35953,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2270],)
    }
};
            w[2270] = noise_metadata_schedule_3856_0_e35953;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3857_0_e35959,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2271],)
    }
};
            w[2271] = noise_metadata_schedule_3857_0_e35959;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3858_0_e35965,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2272],)
    }
};
            w[2272] = noise_metadata_schedule_3858_0_e35965;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3859_0_e35971,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2273],)
    }
};
            w[2273] = noise_metadata_schedule_3859_0_e35971;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3860_0_e35977,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2274],)
    }
};
            w[2274] = noise_metadata_schedule_3860_0_e35977;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3861_0_e35983,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2275],)
    }
};
            w[2275] = noise_metadata_schedule_3861_0_e35983;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3862_0_e35989,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2276],)
    }
};
            w[2276] = noise_metadata_schedule_3862_0_e35989;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3863_0_e35995,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2277],)
    }
};
            w[2277] = noise_metadata_schedule_3863_0_e35995;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3864_0_e36001,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2278],)
    }
};
            w[2278] = noise_metadata_schedule_3864_0_e36001;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3865_0_e36007,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2279],)
    }
};
            w[2279] = noise_metadata_schedule_3865_0_e36007;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3866_0_e36013,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2280],)
    }
};
            w[2280] = noise_metadata_schedule_3866_0_e36013;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3867_0_e36019,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2281],)
    }
};
            w[2281] = noise_metadata_schedule_3867_0_e36019;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3868_0_e36025,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2282],)
    }
};
            w[2282] = noise_metadata_schedule_3868_0_e36025;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3869_0_e36031,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2283],)
    }
};
            w[2283] = noise_metadata_schedule_3869_0_e36031;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3870_0_e36037,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2284],)
    }
};
            w[2284] = noise_metadata_schedule_3870_0_e36037;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3871_0_e36043,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2285],)
    }
};
            w[2285] = noise_metadata_schedule_3871_0_e36043;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3872_0_e36049,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2286],)
    }
};
            w[2286] = noise_metadata_schedule_3872_0_e36049;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3873_0_e36055,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2287],)
    }
};
            w[2287] = noise_metadata_schedule_3873_0_e36055;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3874_0_e36061,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2288],)
    }
};
            w[2288] = noise_metadata_schedule_3874_0_e36061;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_13(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3875_0_e36067,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2289],)
    }
};
            w[2289] = noise_metadata_schedule_3875_0_e36067;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3876_0_e36073,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2290],)
    }
};
            w[2290] = noise_metadata_schedule_3876_0_e36073;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3877_0_e36079,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2291],)
    }
};
            w[2291] = noise_metadata_schedule_3877_0_e36079;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3878_0_e36085,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (0.0,)
    } else {
        (w[2292],)
    }
};
            w[2292] = noise_metadata_schedule_3878_0_e36085;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3879_0_e36096,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3879_0_e36091: f64 = (w[2257] / w[2241]);
        let noise_metadata_schedule_3879_0_e36093: f64 = (-w[2258]);
        let noise_metadata_schedule_3879_0_e36094: f64 = (noise_metadata_schedule_3879_0_e36091 * noise_metadata_schedule_3879_0_e36093);
        (noise_metadata_schedule_3879_0_e36094,)
    } else {
        (w[2272],)
    }
};
            w[2272] = noise_metadata_schedule_3879_0_e36096;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3880_0_e36140,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3880_0_e36106: f64 = (-50.0);
        let (noise_metadata_schedule_3880_0_e36138,) = {
            if ((!(w[2272] > 50.0)) && (!(w[2272] < noise_metadata_schedule_3880_0_e36106))) {
                let noise_metadata_schedule_3880_0_e36111: f64 = (w[2272]).exp();
                (noise_metadata_schedule_3880_0_e36111,)
            } else {
                let noise_metadata_schedule_3880_0_e36118: f64 = (-50.0);
                let (noise_metadata_schedule_3880_0_e36137,) = {
                    if ((!(w[2272] > 50.0)) && (w[2272] < noise_metadata_schedule_3880_0_e36118)) {
                        let noise_metadata_schedule_3880_0_e36122: f64 = (-50.0);
                        let noise_metadata_schedule_3880_0_e36123: f64 = (noise_metadata_schedule_3880_0_e36122).exp();
                        (noise_metadata_schedule_3880_0_e36123,)
                    } else {
                        let (noise_metadata_schedule_3880_0_e36136,) = {
                            if (w[2272] > 50.0) {
                                let noise_metadata_schedule_3880_0_e36128: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3880_0_e36132: f64 = (w[2272] - 50.0);
                                let noise_metadata_schedule_3880_0_e36133: f64 = (1.0 + noise_metadata_schedule_3880_0_e36132);
                                let noise_metadata_schedule_3880_0_e36134: f64 = (noise_metadata_schedule_3880_0_e36128 * noise_metadata_schedule_3880_0_e36133);
                                (noise_metadata_schedule_3880_0_e36134,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3880_0_e36136,)
                    }
                };
                (noise_metadata_schedule_3880_0_e36137,)
            }
        };
        (noise_metadata_schedule_3880_0_e36138,)
    } else {
        (w[2262],)
    }
};
            w[2262] = noise_metadata_schedule_3880_0_e36140;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3881_0_e36153,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3881_0_e36146: f64 = (-w[2240]);
        let noise_metadata_schedule_3881_0_e36148: f64 = (noise_metadata_schedule_3881_0_e36146 - w[2247]);
        let noise_metadata_schedule_3881_0_e36149: f64 = (w[2246] * noise_metadata_schedule_3881_0_e36148);
        let noise_metadata_schedule_3881_0_e36151: f64 = (noise_metadata_schedule_3881_0_e36149 + w[2272]);
        (noise_metadata_schedule_3881_0_e36151,)
    } else {
        (w[2268],)
    }
};
            w[2268] = noise_metadata_schedule_3881_0_e36153;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3882_0_e36164,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3882_0_e36158: f64 = (-w[2246]);
        let noise_metadata_schedule_3882_0_e36160: f64 = (noise_metadata_schedule_3882_0_e36158 * w[2247]);
        let noise_metadata_schedule_3882_0_e36162: f64 = (noise_metadata_schedule_3882_0_e36160 + w[2272]);
        (noise_metadata_schedule_3882_0_e36162,)
    } else {
        (w[2269],)
    }
};
            w[2269] = noise_metadata_schedule_3882_0_e36164;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3883_0_e36208,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3883_0_e36174: f64 = (-50.0);
        let (noise_metadata_schedule_3883_0_e36206,) = {
            if ((!(w[2268] > 50.0)) && (!(w[2268] < noise_metadata_schedule_3883_0_e36174))) {
                let noise_metadata_schedule_3883_0_e36179: f64 = (w[2268]).exp();
                (noise_metadata_schedule_3883_0_e36179,)
            } else {
                let noise_metadata_schedule_3883_0_e36186: f64 = (-50.0);
                let (noise_metadata_schedule_3883_0_e36205,) = {
                    if ((!(w[2268] > 50.0)) && (w[2268] < noise_metadata_schedule_3883_0_e36186)) {
                        let noise_metadata_schedule_3883_0_e36190: f64 = (-50.0);
                        let noise_metadata_schedule_3883_0_e36191: f64 = (noise_metadata_schedule_3883_0_e36190).exp();
                        (noise_metadata_schedule_3883_0_e36191,)
                    } else {
                        let (noise_metadata_schedule_3883_0_e36204,) = {
                            if (w[2268] > 50.0) {
                                let noise_metadata_schedule_3883_0_e36196: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3883_0_e36200: f64 = (w[2268] - 50.0);
                                let noise_metadata_schedule_3883_0_e36201: f64 = (1.0 + noise_metadata_schedule_3883_0_e36200);
                                let noise_metadata_schedule_3883_0_e36202: f64 = (noise_metadata_schedule_3883_0_e36196 * noise_metadata_schedule_3883_0_e36201);
                                (noise_metadata_schedule_3883_0_e36202,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3883_0_e36204,)
                    }
                };
                (noise_metadata_schedule_3883_0_e36205,)
            }
        };
        (noise_metadata_schedule_3883_0_e36206,)
    } else {
        (w[2270],)
    }
};
            w[2270] = noise_metadata_schedule_3883_0_e36208;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3884_0_e36252,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3884_0_e36218: f64 = (-50.0);
        let (noise_metadata_schedule_3884_0_e36250,) = {
            if ((!(w[2269] > 50.0)) && (!(w[2269] < noise_metadata_schedule_3884_0_e36218))) {
                let noise_metadata_schedule_3884_0_e36223: f64 = (w[2269]).exp();
                (noise_metadata_schedule_3884_0_e36223,)
            } else {
                let noise_metadata_schedule_3884_0_e36230: f64 = (-50.0);
                let (noise_metadata_schedule_3884_0_e36249,) = {
                    if ((!(w[2269] > 50.0)) && (w[2269] < noise_metadata_schedule_3884_0_e36230)) {
                        let noise_metadata_schedule_3884_0_e36234: f64 = (-50.0);
                        let noise_metadata_schedule_3884_0_e36235: f64 = (noise_metadata_schedule_3884_0_e36234).exp();
                        (noise_metadata_schedule_3884_0_e36235,)
                    } else {
                        let (noise_metadata_schedule_3884_0_e36248,) = {
                            if (w[2269] > 50.0) {
                                let noise_metadata_schedule_3884_0_e36240: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3884_0_e36244: f64 = (w[2269] - 50.0);
                                let noise_metadata_schedule_3884_0_e36245: f64 = (1.0 + noise_metadata_schedule_3884_0_e36244);
                                let noise_metadata_schedule_3884_0_e36246: f64 = (noise_metadata_schedule_3884_0_e36240 * noise_metadata_schedule_3884_0_e36245);
                                (noise_metadata_schedule_3884_0_e36246,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3884_0_e36248,)
                    }
                };
                (noise_metadata_schedule_3884_0_e36249,)
            }
        };
        (noise_metadata_schedule_3884_0_e36250,)
    } else {
        (w[2271],)
    }
};
            w[2271] = noise_metadata_schedule_3884_0_e36252;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3885_0_e36260,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3885_0_e36258: f64 = (w[2270] - w[2271]);
        (noise_metadata_schedule_3885_0_e36258,)
    } else {
        (w[2264],)
    }
};
            w[2264] = noise_metadata_schedule_3885_0_e36260;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3886_0_e36274,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3886_0_e36266: f64 = (w[2259] * w[2249]);
        let noise_metadata_schedule_3886_0_e36268: f64 = (noise_metadata_schedule_3886_0_e36266 * w[2250]);
        let noise_metadata_schedule_3886_0_e36270: f64 = (noise_metadata_schedule_3886_0_e36268 * w[2251]);
        let noise_metadata_schedule_3886_0_e36272: f64 = (noise_metadata_schedule_3886_0_e36270 * w[2248]);
        (noise_metadata_schedule_3886_0_e36272,)
    } else {
        (w[2238],)
    }
};
            w[2238] = noise_metadata_schedule_3886_0_e36274;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3887_0_e36286,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3887_0_e36280: f64 = (w[2245] / w[2241]);
        let noise_metadata_schedule_3887_0_e36282: f64 = (noise_metadata_schedule_3887_0_e36280 * w[2240]);
        let noise_metadata_schedule_3887_0_e36284: f64 = (noise_metadata_schedule_3887_0_e36282 + w[2272]);
        (noise_metadata_schedule_3887_0_e36284,)
    } else {
        (w[2274],)
    }
};
            w[2274] = noise_metadata_schedule_3887_0_e36286;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3888_0_e36330,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3888_0_e36296: f64 = (-50.0);
        let (noise_metadata_schedule_3888_0_e36328,) = {
            if ((!(w[2274] > 50.0)) && (!(w[2274] < noise_metadata_schedule_3888_0_e36296))) {
                let noise_metadata_schedule_3888_0_e36301: f64 = (w[2274]).exp();
                (noise_metadata_schedule_3888_0_e36301,)
            } else {
                let noise_metadata_schedule_3888_0_e36308: f64 = (-50.0);
                let (noise_metadata_schedule_3888_0_e36327,) = {
                    if ((!(w[2274] > 50.0)) && (w[2274] < noise_metadata_schedule_3888_0_e36308)) {
                        let noise_metadata_schedule_3888_0_e36312: f64 = (-50.0);
                        let noise_metadata_schedule_3888_0_e36313: f64 = (noise_metadata_schedule_3888_0_e36312).exp();
                        (noise_metadata_schedule_3888_0_e36313,)
                    } else {
                        let (noise_metadata_schedule_3888_0_e36326,) = {
                            if (w[2274] > 50.0) {
                                let noise_metadata_schedule_3888_0_e36318: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3888_0_e36322: f64 = (w[2274] - 50.0);
                                let noise_metadata_schedule_3888_0_e36323: f64 = (1.0 + noise_metadata_schedule_3888_0_e36322);
                                let noise_metadata_schedule_3888_0_e36324: f64 = (noise_metadata_schedule_3888_0_e36318 * noise_metadata_schedule_3888_0_e36323);
                                (noise_metadata_schedule_3888_0_e36324,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3888_0_e36326,)
                    }
                };
                (noise_metadata_schedule_3888_0_e36327,)
            }
        };
        (noise_metadata_schedule_3888_0_e36328,)
    } else {
        (w[2275],)
    }
};
            w[2275] = noise_metadata_schedule_3888_0_e36330;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_3889_0_e36333: f64 = if w[2244] == 1.0 { 1.0 } else { 0.0 };
            w[2293] = noise_metadata_schedule_3889_0_e36333;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3890_0_e36349,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] != 0.0)) {
        let noise_metadata_schedule_3890_0_e36343: f64 = (w[2252] * w[2264]);
        let noise_metadata_schedule_3890_0_e36344: f64 = (w[2275] - noise_metadata_schedule_3890_0_e36343);
        let noise_metadata_schedule_3890_0_e36346: f64 = (noise_metadata_schedule_3890_0_e36344 - w[2262]);
        let noise_metadata_schedule_3890_0_e36347: f64 = (w[2238] * noise_metadata_schedule_3890_0_e36346);
        (noise_metadata_schedule_3890_0_e36347,)
    } else {
        (w[2265],)
    }
};
            w[2265] = noise_metadata_schedule_3890_0_e36349;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3891_0_e36365,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3891_0_e36358: f64 = (-w[2242]);
        let noise_metadata_schedule_3891_0_e36360: f64 = (noise_metadata_schedule_3891_0_e36358 - w[2247]);
        let noise_metadata_schedule_3891_0_e36361: f64 = (w[2246] * noise_metadata_schedule_3891_0_e36360);
        let noise_metadata_schedule_3891_0_e36363: f64 = (noise_metadata_schedule_3891_0_e36361 + w[2272]);
        (noise_metadata_schedule_3891_0_e36363,)
    } else {
        (w[2279],)
    }
};
            w[2279] = noise_metadata_schedule_3891_0_e36365;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3892_0_e36412,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3892_0_e36378: f64 = (-50.0);
        let (noise_metadata_schedule_3892_0_e36410,) = {
            if ((!(w[2279] > 50.0)) && (!(w[2279] < noise_metadata_schedule_3892_0_e36378))) {
                let noise_metadata_schedule_3892_0_e36383: f64 = (w[2279]).exp();
                (noise_metadata_schedule_3892_0_e36383,)
            } else {
                let noise_metadata_schedule_3892_0_e36390: f64 = (-50.0);
                let (noise_metadata_schedule_3892_0_e36409,) = {
                    if ((!(w[2279] > 50.0)) && (w[2279] < noise_metadata_schedule_3892_0_e36390)) {
                        let noise_metadata_schedule_3892_0_e36394: f64 = (-50.0);
                        let noise_metadata_schedule_3892_0_e36395: f64 = (noise_metadata_schedule_3892_0_e36394).exp();
                        (noise_metadata_schedule_3892_0_e36395,)
                    } else {
                        let (noise_metadata_schedule_3892_0_e36408,) = {
                            if (w[2279] > 50.0) {
                                let noise_metadata_schedule_3892_0_e36400: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3892_0_e36404: f64 = (w[2279] - 50.0);
                                let noise_metadata_schedule_3892_0_e36405: f64 = (1.0 + noise_metadata_schedule_3892_0_e36404);
                                let noise_metadata_schedule_3892_0_e36406: f64 = (noise_metadata_schedule_3892_0_e36400 * noise_metadata_schedule_3892_0_e36405);
                                (noise_metadata_schedule_3892_0_e36406,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3892_0_e36408,)
                    }
                };
                (noise_metadata_schedule_3892_0_e36409,)
            }
        };
        (noise_metadata_schedule_3892_0_e36410,)
    } else {
        (w[2280],)
    }
};
            w[2280] = noise_metadata_schedule_3892_0_e36412;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3893_0_e36423,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3893_0_e36421: f64 = (w[2280] - w[2271]);
        (noise_metadata_schedule_3893_0_e36421,)
    } else {
        (w[2281],)
    }
};
            w[2281] = noise_metadata_schedule_3893_0_e36423;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3894_0_e36438,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3894_0_e36432: f64 = (w[2245] / w[2241]);
        let noise_metadata_schedule_3894_0_e36434: f64 = (noise_metadata_schedule_3894_0_e36432 * w[2242]);
        let noise_metadata_schedule_3894_0_e36436: f64 = (noise_metadata_schedule_3894_0_e36434 + w[2272]);
        (noise_metadata_schedule_3894_0_e36436,)
    } else {
        (w[2282],)
    }
};
            w[2282] = noise_metadata_schedule_3894_0_e36438;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3895_0_e36485,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3895_0_e36451: f64 = (-50.0);
        let (noise_metadata_schedule_3895_0_e36483,) = {
            if ((!(w[2282] > 50.0)) && (!(w[2282] < noise_metadata_schedule_3895_0_e36451))) {
                let noise_metadata_schedule_3895_0_e36456: f64 = (w[2282]).exp();
                (noise_metadata_schedule_3895_0_e36456,)
            } else {
                let noise_metadata_schedule_3895_0_e36463: f64 = (-50.0);
                let (noise_metadata_schedule_3895_0_e36482,) = {
                    if ((!(w[2282] > 50.0)) && (w[2282] < noise_metadata_schedule_3895_0_e36463)) {
                        let noise_metadata_schedule_3895_0_e36467: f64 = (-50.0);
                        let noise_metadata_schedule_3895_0_e36468: f64 = (noise_metadata_schedule_3895_0_e36467).exp();
                        (noise_metadata_schedule_3895_0_e36468,)
                    } else {
                        let (noise_metadata_schedule_3895_0_e36481,) = {
                            if (w[2282] > 50.0) {
                                let noise_metadata_schedule_3895_0_e36473: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3895_0_e36477: f64 = (w[2282] - 50.0);
                                let noise_metadata_schedule_3895_0_e36478: f64 = (1.0 + noise_metadata_schedule_3895_0_e36477);
                                let noise_metadata_schedule_3895_0_e36479: f64 = (noise_metadata_schedule_3895_0_e36473 * noise_metadata_schedule_3895_0_e36478);
                                (noise_metadata_schedule_3895_0_e36479,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3895_0_e36481,)
                    }
                };
                (noise_metadata_schedule_3895_0_e36482,)
            }
        };
        (noise_metadata_schedule_3895_0_e36483,)
    } else {
        (w[2283],)
    }
};
            w[2283] = noise_metadata_schedule_3895_0_e36485;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3896_0_e36500,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3896_0_e36495: f64 = (w[2252] * w[2281]);
        let noise_metadata_schedule_3896_0_e36496: f64 = (w[2283] - noise_metadata_schedule_3896_0_e36495);
        let noise_metadata_schedule_3896_0_e36498: f64 = (noise_metadata_schedule_3896_0_e36496 - w[2262]);
        (noise_metadata_schedule_3896_0_e36498,)
    } else {
        (w[2284],)
    }
};
            w[2284] = noise_metadata_schedule_3896_0_e36500;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3897_0_e36517,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3897_0_e36511: f64 = (w[2252] * w[2264]);
        let noise_metadata_schedule_3897_0_e36512: f64 = (w[2275] - noise_metadata_schedule_3897_0_e36511);
        let noise_metadata_schedule_3897_0_e36514: f64 = (noise_metadata_schedule_3897_0_e36512 - w[2262]);
        let noise_metadata_schedule_3897_0_e36515: f64 = (w[2238] * noise_metadata_schedule_3897_0_e36514);
        (noise_metadata_schedule_3897_0_e36515,)
    } else {
        (w[2285],)
    }
};
            w[2285] = noise_metadata_schedule_3897_0_e36517;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_3898_0_e36520: f64 = if w[2244] > 0.0 { 1.0 } else { 0.0 };
            w[2294] = noise_metadata_schedule_3898_0_e36520;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3899_0_e36533,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3899_0_e36531: f64 = (w[2244] * w[2245]);
        (noise_metadata_schedule_3899_0_e36531,)
    } else {
        (w[2278],)
    }
};
            w[2278] = noise_metadata_schedule_3899_0_e36533;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3900_0_e36550,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3900_0_e36544: f64 = (w[2278] / w[2241]);
        let noise_metadata_schedule_3900_0_e36546: f64 = (noise_metadata_schedule_3900_0_e36544 * w[2242]);
        let noise_metadata_schedule_3900_0_e36548: f64 = (noise_metadata_schedule_3900_0_e36546 + w[2272]);
        (noise_metadata_schedule_3900_0_e36548,)
    } else {
        (w[2286],)
    }
};
            w[2286] = noise_metadata_schedule_3900_0_e36550;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3901_0_e36599,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3901_0_e36565: f64 = (-50.0);
        let (noise_metadata_schedule_3901_0_e36597,) = {
            if ((!(w[2286] > 50.0)) && (!(w[2286] < noise_metadata_schedule_3901_0_e36565))) {
                let noise_metadata_schedule_3901_0_e36570: f64 = (w[2286]).exp();
                (noise_metadata_schedule_3901_0_e36570,)
            } else {
                let noise_metadata_schedule_3901_0_e36577: f64 = (-50.0);
                let (noise_metadata_schedule_3901_0_e36596,) = {
                    if ((!(w[2286] > 50.0)) && (w[2286] < noise_metadata_schedule_3901_0_e36577)) {
                        let noise_metadata_schedule_3901_0_e36581: f64 = (-50.0);
                        let noise_metadata_schedule_3901_0_e36582: f64 = (noise_metadata_schedule_3901_0_e36581).exp();
                        (noise_metadata_schedule_3901_0_e36582,)
                    } else {
                        let (noise_metadata_schedule_3901_0_e36595,) = {
                            if (w[2286] > 50.0) {
                                let noise_metadata_schedule_3901_0_e36587: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3901_0_e36591: f64 = (w[2286] - 50.0);
                                let noise_metadata_schedule_3901_0_e36592: f64 = (1.0 + noise_metadata_schedule_3901_0_e36591);
                                let noise_metadata_schedule_3901_0_e36593: f64 = (noise_metadata_schedule_3901_0_e36587 * noise_metadata_schedule_3901_0_e36592);
                                (noise_metadata_schedule_3901_0_e36593,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3901_0_e36595,)
                    }
                };
                (noise_metadata_schedule_3901_0_e36596,)
            }
        };
        (noise_metadata_schedule_3901_0_e36597,)
    } else {
        (w[2287],)
    }
};
            w[2287] = noise_metadata_schedule_3901_0_e36599;
        }
    }

    #[inline(never)]
    fn noise_metadata_schedule_part_14(&self, ctx: &GeneratedEvalContext<'_>, w: &mut [f64; 2701], active: &[u128; 1]) {
        let params = &*self.params;
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3902_0_e36616,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3902_0_e36611: f64 = (w[2252] * w[2281]);
        let noise_metadata_schedule_3902_0_e36612: f64 = (w[2287] - noise_metadata_schedule_3902_0_e36611);
        let noise_metadata_schedule_3902_0_e36614: f64 = (noise_metadata_schedule_3902_0_e36612 - w[2262]);
        (noise_metadata_schedule_3902_0_e36614,)
    } else {
        (w[2288],)
    }
};
            w[2288] = noise_metadata_schedule_3902_0_e36616;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3903_0_e36633,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3903_0_e36627: f64 = (w[2278] / w[2241]);
        let noise_metadata_schedule_3903_0_e36629: f64 = (noise_metadata_schedule_3903_0_e36627 * w[2240]);
        let noise_metadata_schedule_3903_0_e36631: f64 = (noise_metadata_schedule_3903_0_e36629 + w[2272]);
        (noise_metadata_schedule_3903_0_e36631,)
    } else {
        (w[2289],)
    }
};
            w[2289] = noise_metadata_schedule_3903_0_e36633;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3904_0_e36682,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3904_0_e36648: f64 = (-50.0);
        let (noise_metadata_schedule_3904_0_e36680,) = {
            if ((!(w[2289] > 50.0)) && (!(w[2289] < noise_metadata_schedule_3904_0_e36648))) {
                let noise_metadata_schedule_3904_0_e36653: f64 = (w[2289]).exp();
                (noise_metadata_schedule_3904_0_e36653,)
            } else {
                let noise_metadata_schedule_3904_0_e36660: f64 = (-50.0);
                let (noise_metadata_schedule_3904_0_e36679,) = {
                    if ((!(w[2289] > 50.0)) && (w[2289] < noise_metadata_schedule_3904_0_e36660)) {
                        let noise_metadata_schedule_3904_0_e36664: f64 = (-50.0);
                        let noise_metadata_schedule_3904_0_e36665: f64 = (noise_metadata_schedule_3904_0_e36664).exp();
                        (noise_metadata_schedule_3904_0_e36665,)
                    } else {
                        let (noise_metadata_schedule_3904_0_e36678,) = {
                            if (w[2289] > 50.0) {
                                let noise_metadata_schedule_3904_0_e36670: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3904_0_e36674: f64 = (w[2289] - 50.0);
                                let noise_metadata_schedule_3904_0_e36675: f64 = (1.0 + noise_metadata_schedule_3904_0_e36674);
                                let noise_metadata_schedule_3904_0_e36676: f64 = (noise_metadata_schedule_3904_0_e36670 * noise_metadata_schedule_3904_0_e36675);
                                (noise_metadata_schedule_3904_0_e36676,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3904_0_e36678,)
                    }
                };
                (noise_metadata_schedule_3904_0_e36679,)
            }
        };
        (noise_metadata_schedule_3904_0_e36680,)
    } else {
        (w[2290],)
    }
};
            w[2290] = noise_metadata_schedule_3904_0_e36682;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3905_0_e36697,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3905_0_e36693: f64 = (w[2238] * w[2284]);
        let noise_metadata_schedule_3905_0_e36695: f64 = (noise_metadata_schedule_3905_0_e36693 / w[2288]);
        (noise_metadata_schedule_3905_0_e36695,)
    } else {
        (w[2291],)
    }
};
            w[2291] = noise_metadata_schedule_3905_0_e36697;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3906_0_e36716,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] != 0.0)) {
        let noise_metadata_schedule_3906_0_e36710: f64 = (w[2252] * w[2264]);
        let noise_metadata_schedule_3906_0_e36711: f64 = (w[2290] - noise_metadata_schedule_3906_0_e36710);
        let noise_metadata_schedule_3906_0_e36713: f64 = (noise_metadata_schedule_3906_0_e36711 - w[2262]);
        let noise_metadata_schedule_3906_0_e36714: f64 = (w[2291] * noise_metadata_schedule_3906_0_e36713);
        (noise_metadata_schedule_3906_0_e36714,)
    } else {
        (w[2292],)
    }
};
            w[2292] = noise_metadata_schedule_3906_0_e36716;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3907_0_e36730,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2294] == 0.0)) {
        let noise_metadata_schedule_3907_0_e36728: f64 = (w[2238] * w[2284]);
        (noise_metadata_schedule_3907_0_e36728,)
    } else {
        (w[2292],)
    }
};
            w[2292] = noise_metadata_schedule_3907_0_e36730;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3908_0_e36743,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3908_0_e36739: f64 = (w[2243] * w[2243]);
        let noise_metadata_schedule_3908_0_e36741: f64 = (noise_metadata_schedule_3908_0_e36739 * w[2241]);
        (noise_metadata_schedule_3908_0_e36741,)
    } else {
        (w[2261],)
    }
};
            w[2261] = noise_metadata_schedule_3908_0_e36743;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3909_0_e36760,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3909_0_e36754: f64 = (w[2261] / 2.0);
        let noise_metadata_schedule_3909_0_e36755: f64 = (w[2242] - noise_metadata_schedule_3909_0_e36754);
        let noise_metadata_schedule_3909_0_e36756: f64 = (w[2240] - noise_metadata_schedule_3909_0_e36755);
        let noise_metadata_schedule_3909_0_e36758: f64 = (noise_metadata_schedule_3909_0_e36756 / w[2261]);
        (noise_metadata_schedule_3909_0_e36758,)
    } else {
        (w[2273],)
    }
};
            w[2273] = noise_metadata_schedule_3909_0_e36760;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_3910_0_e36763: f64 = if w[2273] > 50.0 { 1.0 } else { 0.0 };
            w[2295] = noise_metadata_schedule_3910_0_e36763;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3911_0_e36774,) = {
    if ((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2295] != 0.0)) {
        (0.0,)
    } else {
        (w[2263],)
    }
};
            w[2263] = noise_metadata_schedule_3911_0_e36774;
        }
        if (active[0] & 0x2) != 0 {
            let noise_metadata_schedule_3912_0_e36777: f64 = (-50.0);
            let noise_metadata_schedule_3912_0_e36778: f64 = if w[2273] < noise_metadata_schedule_3912_0_e36777 { 1.0 } else { 0.0 };
            w[2296] = noise_metadata_schedule_3912_0_e36778;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3913_0_e36792,) = {
    if (((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2295] == 0.0)) && (w[2296] != 0.0)) {
        (1.0,)
    } else {
        (w[2263],)
    }
};
            w[2263] = noise_metadata_schedule_3913_0_e36792;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3914_0_e36812,) = {
    if (((((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) && (w[2295] == 0.0)) && (w[2296] == 0.0)) {
        let noise_metadata_schedule_3914_0_e36808: f64 = (w[2273]).exp();
        let noise_metadata_schedule_3914_0_e36809: f64 = (1.0 + noise_metadata_schedule_3914_0_e36808);
        let noise_metadata_schedule_3914_0_e36810: f64 = (1.0 / noise_metadata_schedule_3914_0_e36809);
        (noise_metadata_schedule_3914_0_e36810,)
    } else {
        (w[2263],)
    }
};
            w[2263] = noise_metadata_schedule_3914_0_e36812;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3915_0_e36829,) = {
    if (((w[1934] != 0.0) && (w[2176] != 0.0)) && (w[2293] == 0.0)) {
        let noise_metadata_schedule_3915_0_e36821: f64 = (w[2263] * w[2285]);
        let noise_metadata_schedule_3915_0_e36824: f64 = (1.0 - w[2263]);
        let noise_metadata_schedule_3915_0_e36826: f64 = (noise_metadata_schedule_3915_0_e36824 * w[2292]);
        let noise_metadata_schedule_3915_0_e36827: f64 = (noise_metadata_schedule_3915_0_e36821 + noise_metadata_schedule_3915_0_e36826);
        (noise_metadata_schedule_3915_0_e36827,)
    } else {
        (w[2265],)
    }
};
            w[2265] = noise_metadata_schedule_3915_0_e36829;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3916_0_e36877,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3916_0_e36834: f64 = (-w[2240]);
        let (noise_metadata_schedule_3916_0_e36867,) = {
            if (params.p52 != 0.0) {
                let noise_metadata_schedule_3916_0_e36842: f64 = (w[2240] / w[2253]);
                let noise_metadata_schedule_3916_0_e36845: f64 = (0.001 / params.p53);
                let noise_metadata_schedule_3916_0_e36848: f64 = (w[2240] / w[2253]);
                let noise_metadata_schedule_3916_0_e36849: f64 = (noise_metadata_schedule_3916_0_e36845 * noise_metadata_schedule_3916_0_e36848);
                let noise_metadata_schedule_3916_0_e36850: f64 = (noise_metadata_schedule_3916_0_e36849).tanh();
                let noise_metadata_schedule_3916_0_e36851: f64 = (noise_metadata_schedule_3916_0_e36842 * noise_metadata_schedule_3916_0_e36850);
                (noise_metadata_schedule_3916_0_e36851,)
            } else {
                let (noise_metadata_schedule_3916_0_e36866,) = {
                    if (params.p52 == 0.0) {
                        let noise_metadata_schedule_3916_0_e36857: f64 = (w[2240] / w[2253]);
                        let noise_metadata_schedule_3916_0_e36860: f64 = (w[2240] / w[2253]);
                        let noise_metadata_schedule_3916_0_e36861: f64 = (noise_metadata_schedule_3916_0_e36857 * noise_metadata_schedule_3916_0_e36860);
                        let noise_metadata_schedule_3916_0_e36863: f64 = (noise_metadata_schedule_3916_0_e36861 + params.p53);
                        let noise_metadata_schedule_3916_0_e36864: f64 = (noise_metadata_schedule_3916_0_e36863).sqrt();
                        (noise_metadata_schedule_3916_0_e36864,)
                    } else {
                        (0.0,)
                    }
                };
                (noise_metadata_schedule_3916_0_e36866,)
            }
        };
        let noise_metadata_schedule_3916_0_e36869: f64 = (noise_metadata_schedule_3916_0_e36867).powf(w[2254]);
        let noise_metadata_schedule_3916_0_e36870: f64 = (1.0 + noise_metadata_schedule_3916_0_e36869);
        let noise_metadata_schedule_3916_0_e36873: f64 = (1.0 / w[2254]);
        let noise_metadata_schedule_3916_0_e36874: f64 = (noise_metadata_schedule_3916_0_e36870).powf(noise_metadata_schedule_3916_0_e36873);
        let noise_metadata_schedule_3916_0_e36875: f64 = (noise_metadata_schedule_3916_0_e36834 / noise_metadata_schedule_3916_0_e36874);
        (noise_metadata_schedule_3916_0_e36875,)
    } else {
        (w[2266],)
    }
};
            w[2266] = noise_metadata_schedule_3916_0_e36877;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3917_0_e36894,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3917_0_e36882: f64 = (-w[2259]);
        let noise_metadata_schedule_3917_0_e36884: f64 = (noise_metadata_schedule_3917_0_e36882 * w[2249]);
        let noise_metadata_schedule_3917_0_e36886: f64 = (noise_metadata_schedule_3917_0_e36884 * w[2250]);
        let noise_metadata_schedule_3917_0_e36888: f64 = (noise_metadata_schedule_3917_0_e36886 * w[2255]);
        let noise_metadata_schedule_3917_0_e36890: f64 = (noise_metadata_schedule_3917_0_e36888 * w[2248]);
        let noise_metadata_schedule_3917_0_e36892: f64 = noise_metadata_schedule_3917_0_e36890;
        (noise_metadata_schedule_3917_0_e36892,)
    } else {
        (w[2239],)
    }
};
            w[2239] = noise_metadata_schedule_3917_0_e36894;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3918_0_e36904,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3918_0_e36900: f64 = (w[2256] / w[2241]);
        let noise_metadata_schedule_3918_0_e36902: f64 = (noise_metadata_schedule_3918_0_e36900 * w[2266]);
        (noise_metadata_schedule_3918_0_e36902,)
    } else {
        (w[2276],)
    }
};
            w[2276] = noise_metadata_schedule_3918_0_e36904;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3919_0_e36948,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3919_0_e36914: f64 = (-50.0);
        let (noise_metadata_schedule_3919_0_e36946,) = {
            if ((!(w[2276] > 50.0)) && (!(w[2276] < noise_metadata_schedule_3919_0_e36914))) {
                let noise_metadata_schedule_3919_0_e36919: f64 = (w[2276]).exp();
                (noise_metadata_schedule_3919_0_e36919,)
            } else {
                let noise_metadata_schedule_3919_0_e36926: f64 = (-50.0);
                let (noise_metadata_schedule_3919_0_e36945,) = {
                    if ((!(w[2276] > 50.0)) && (w[2276] < noise_metadata_schedule_3919_0_e36926)) {
                        let noise_metadata_schedule_3919_0_e36930: f64 = (-50.0);
                        let noise_metadata_schedule_3919_0_e36931: f64 = (noise_metadata_schedule_3919_0_e36930).exp();
                        (noise_metadata_schedule_3919_0_e36931,)
                    } else {
                        let (noise_metadata_schedule_3919_0_e36944,) = {
                            if (w[2276] > 50.0) {
                                let noise_metadata_schedule_3919_0_e36936: f64 = (50.0_f64).exp();
                                let noise_metadata_schedule_3919_0_e36940: f64 = (w[2276] - 50.0);
                                let noise_metadata_schedule_3919_0_e36941: f64 = (1.0 + noise_metadata_schedule_3919_0_e36940);
                                let noise_metadata_schedule_3919_0_e36942: f64 = (noise_metadata_schedule_3919_0_e36936 * noise_metadata_schedule_3919_0_e36941);
                                (noise_metadata_schedule_3919_0_e36942,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_3919_0_e36944,)
                    }
                };
                (noise_metadata_schedule_3919_0_e36945,)
            }
        };
        (noise_metadata_schedule_3919_0_e36946,)
    } else {
        (w[2277],)
    }
};
            w[2277] = noise_metadata_schedule_3919_0_e36948;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3920_0_e36958,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3920_0_e36955: f64 = (w[2277] - 1.0);
        let noise_metadata_schedule_3920_0_e36956: f64 = (w[2239] * noise_metadata_schedule_3920_0_e36955);
        (noise_metadata_schedule_3920_0_e36956,)
    } else {
        (w[2267],)
    }
};
            w[2267] = noise_metadata_schedule_3920_0_e36958;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3921_0_e36966,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        let noise_metadata_schedule_3921_0_e36964: f64 = (w[2265] + w[2267]);
        (noise_metadata_schedule_3921_0_e36964,)
    } else {
        (w[2260],)
    }
};
            w[2260] = noise_metadata_schedule_3921_0_e36966;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3922_0_e36972,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2260],)
    } else {
        (w[2237],)
    }
};
            w[2237] = noise_metadata_schedule_3922_0_e36972;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3923_0_e36978,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2238],)
    } else {
        (w[131],)
    }
};
            w[131] = noise_metadata_schedule_3923_0_e36978;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3924_0_e36984,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2239],)
    } else {
        (w[133],)
    }
};
            w[133] = noise_metadata_schedule_3924_0_e36984;
        }
        if (active[0] & 0x2) != 0 {
            let (noise_metadata_schedule_3925_0_e36990,) = {
    if ((w[1934] != 0.0) && (w[2176] != 0.0)) {
        (w[2237],)
    } else {
        (w[129],)
    }
};
            w[129] = noise_metadata_schedule_3925_0_e36990;
        }
        if (active[0] & 0x20) != 0 {
            w[231] = 0.0;
        }
        if (active[0] & 0x20) != 0 {
            w[232] = 0.0;
        }
        if (active[0] & 0x30) != 0 {
            let noise_metadata_schedule_4648_0_e45310: f64 = if params.p347 == 1.0 { 1.0 } else { 0.0 };
            w[2686] = noise_metadata_schedule_4648_0_e45310;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_4649_0_e45329,) = {
    if (w[2686] != 0.0) {
        let noise_metadata_schedule_4649_0_e45315: f64 = (params.p0 * params.p2);
        let noise_metadata_schedule_4649_0_e45317: f64 = (noise_metadata_schedule_4649_0_e45315 / params.p1);
        let noise_metadata_schedule_4649_0_e45318: f64 = (params.p350 * noise_metadata_schedule_4649_0_e45317);
        let noise_metadata_schedule_4649_0_e45320: f64 = (w[115]).abs();
        let noise_metadata_schedule_4649_0_e45323: f64 = (params.p0 * params.p2);
        let noise_metadata_schedule_4649_0_e45324: f64 = (noise_metadata_schedule_4649_0_e45320 / noise_metadata_schedule_4649_0_e45323);
        let noise_metadata_schedule_4649_0_e45326: f64 = (noise_metadata_schedule_4649_0_e45324).powf(params.p351);
        let noise_metadata_schedule_4649_0_e45327: f64 = (noise_metadata_schedule_4649_0_e45318 * noise_metadata_schedule_4649_0_e45326);
        (noise_metadata_schedule_4649_0_e45327,)
    } else {
        (w[233],)
    }
};
            w[233] = noise_metadata_schedule_4649_0_e45329;
        }
        if (active[0] & 0x10) != 0 {
            let noise_metadata_schedule_4650_0_e45332: f64 = if w[115] < 0.0 { 1.0 } else { 0.0 };
            w[2687] = noise_metadata_schedule_4650_0_e45332;
        }
        if (active[0] & 0x10) != 0 {
            let (noise_metadata_schedule_4651_0_e45339,) = {
    if ((w[2686] != 0.0) && (w[2687] != 0.0)) {
        let noise_metadata_schedule_4651_0_e45337: f64 = (-w[233]);
        (noise_metadata_schedule_4651_0_e45337,)
    } else {
        (w[233],)
    }
};
            w[233] = noise_metadata_schedule_4651_0_e45339;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_4652_0_e45345,) = {
    if (w[2686] != 0.0) {
        let noise_metadata_schedule_4652_0_e45343: f64 = 0.0;
        (noise_metadata_schedule_4652_0_e45343,)
    } else {
        (w[231],)
    }
};
            w[231] = noise_metadata_schedule_4652_0_e45345;
        }
        if (active[0] & 0x20) != 0 {
            let (noise_metadata_schedule_4653_0_e45369,) = {
    if (w[2686] != 0.0) {
        let noise_metadata_schedule_4653_0_e45349: f64 = (4.0 * 1.38062e-23);
        let noise_metadata_schedule_4653_0_e45351: f64 = (noise_metadata_schedule_4653_0_e45349 * w[111]);
        let noise_metadata_schedule_4653_0_e45353: f64 = (noise_metadata_schedule_4653_0_e45351 * w[231]);
        let noise_metadata_schedule_4653_0_e45356: f64 = (w[117] + w[118]);
        let noise_metadata_schedule_4653_0_e45357: f64 = (noise_metadata_schedule_4653_0_e45353 * noise_metadata_schedule_4653_0_e45356);
        let noise_metadata_schedule_4653_0_e45360: f64 = (params.p0 * params.p2);
        let noise_metadata_schedule_4653_0_e45362: f64 = (noise_metadata_schedule_4653_0_e45360 * params.p1);
        let noise_metadata_schedule_4653_0_e45364: f64 = (noise_metadata_schedule_4653_0_e45362 * params.p6);
        let noise_metadata_schedule_4653_0_e45366: f64 = (noise_metadata_schedule_4653_0_e45364 * params.p7);
        let noise_metadata_schedule_4653_0_e45367: f64 = (noise_metadata_schedule_4653_0_e45357 / noise_metadata_schedule_4653_0_e45366);
        (noise_metadata_schedule_4653_0_e45367,)
    } else {
        (w[232],)
    }
};
            w[232] = noise_metadata_schedule_4653_0_e45369;
        }
    }
}

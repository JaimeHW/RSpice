#![allow(dead_code, non_snake_case, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseKind};

const LIMEXP_MAX: f64 = 5.54062238439351e34;
const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 4] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("Rb"), kind: GeneratedNoiseKind::White, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_E_EI_RE", label: Some("Re"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "e", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER_IBE", label: Some("flicker_Ibe"), kind: GeneratedNoiseKind::Flicker, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("Ibe"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_source(&self, source_index: usize, ctx: &GeneratedEvalContext<'_>) -> Result<GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError> {
        if source_index >= NOISE_SOURCES.len() {
            return Err(GeneratedNoiseEvaluationError::SourceIndexOutOfRange { index: source_index, count: NOISE_SOURCES.len() });
        }
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let params = &*self.params;
        let mut noise_variable_0 = 0.0;
        let mut noise_variable_1 = 0.0;
        let mut noise_variable_2 = 0.0;
        let mut noise_variable_3 = 0.0;
        let mut noise_variable_4 = 0.0;
        let mut noise_variable_5 = 0.0;
        let mut noise_variable_6 = 0.0;
        let mut noise_variable_7 = 0.0;
        let mut noise_variable_8 = 0.0;
        let mut noise_variable_9 = 0.0;
        let mut noise_variable_10 = 0.0;
        let mut noise_variable_11 = 0.0;
        let mut noise_variable_12 = 0.0;
        let mut noise_variable_13 = 0.0;
        let mut noise_variable_14 = 0.0;
        let mut noise_variable_15 = 0.0;
        let mut noise_variable_16 = 0.0;
        let mut noise_variable_17 = 0.0;
        let mut noise_variable_18 = 0.0;
        let mut noise_variable_19 = 0.0;
        let mut noise_variable_20 = 0.0;
        let mut noise_variable_21 = 0.0;
        let mut noise_variable_22 = 0.0;
        let mut noise_variable_23 = 0.0;
        let mut noise_variable_24 = 0.0;
        let mut noise_variable_25 = 0.0;
        let mut noise_variable_26 = 0.0;
        let mut noise_variable_27 = 0.0;
        let mut noise_variable_28 = 0.0;
        let mut noise_variable_29 = 0.0;
        let mut noise_variable_30 = 0.0;
        let mut noise_variable_31 = 0.0;
        let mut noise_variable_32 = 0.0;
        let mut noise_variable_33 = 0.0;
        let mut noise_variable_34 = 0.0;
        let mut noise_variable_35 = 0.0;
        let mut noise_variable_36 = 0.0;
        let mut noise_variable_37 = 0.0;
        let mut noise_variable_38 = 0.0;
        let mut noise_variable_39 = 0.0;
        let mut noise_variable_40 = 0.0;
        let mut noise_variable_41 = 0.0;
        let mut noise_variable_42 = 0.0;
        let mut noise_variable_43 = 0.0;
        let mut noise_variable_44 = 0.0;
        let mut noise_variable_45 = 0.0;
        let mut noise_variable_46 = 0.0;
        let mut noise_variable_47 = 0.0;
        let mut noise_variable_48 = 0.0;
        let mut noise_variable_49 = 0.0;
        let mut noise_variable_50 = 0.0;
        let mut noise_variable_51 = 0.0;
        let mut noise_variable_52 = 0.0;
        let mut noise_variable_53 = 0.0;
        let mut noise_variable_54 = 0.0;
        let mut noise_variable_55 = 0.0;
        let mut noise_variable_56 = 0.0;
        let mut noise_variable_57 = 0.0;
        let mut noise_variable_58 = 0.0;
        let mut noise_variable_59 = 0.0;
        let mut noise_variable_60 = 0.0;
        let mut noise_variable_61 = 0.0;
        let mut noise_variable_62 = 0.0;
        let mut noise_variable_63 = 0.0;
        let mut noise_variable_64 = 0.0;
        let mut noise_variable_65 = 0.0;
        let mut noise_variable_66 = 0.0;
        let mut noise_variable_67 = 0.0;
        let mut noise_variable_68 = 0.0;
        let mut noise_variable_69 = 0.0;
        let mut noise_variable_70 = 0.0;
        let mut noise_variable_71 = 0.0;
        let mut noise_variable_72 = 0.0;
        let mut noise_variable_73 = 0.0;
        let mut noise_variable_74 = 0.0;
        if matches!(source_index, 0 | 1) {
            let noise_activation_schedule_4_e346: f64 = (params.p43 * params.p42);
            noise_variable_3 = noise_activation_schedule_4_e346;
        }
        if matches!(source_index, 0) {
            let noise_activation_schedule_83_e973: f64 = (params.p31 * params.p13);
            let noise_activation_schedule_83_e974: f64 = (params.p12 + noise_activation_schedule_83_e973);
            let noise_activation_schedule_83_e976: f64 = (noise_activation_schedule_83_e974 / noise_variable_3);
            noise_variable_28 = noise_activation_schedule_83_e976;
        }
        if matches!(source_index, 1) {
            let noise_activation_schedule_84_e980: f64 = (params.p31 * params.p15);
            let noise_activation_schedule_84_e981: f64 = (params.p14 + noise_activation_schedule_84_e980);
            let noise_activation_schedule_84_e983: f64 = (noise_activation_schedule_84_e981 / noise_variable_3);
            noise_variable_27 = noise_activation_schedule_84_e983;
        }
        if matches!(source_index, 0) {
            let noise_activation_schedule_85_e990: f64 = if ((noise_variable_28 > 0.0) && (noise_variable_28 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_73 = noise_activation_schedule_85_e990;
        }
        if matches!(source_index, 1) {
            let noise_activation_schedule_87_e1012: f64 = if ((noise_variable_27 > 0.0) && (noise_variable_27 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_74 = noise_activation_schedule_87_e1012;
        }
        let noise_source_active = match source_index {
            0 => {
                noise_variable_73 != 0.0
            }
            1 => {
                noise_variable_74 != 0.0
            }
            2 => {
                true
            }
            3 => {
                true
            }
            _ => unreachable!("noise source index was range checked"),
        };
        if !noise_source_active { return Ok(GeneratedNoiseEvaluation { active: false, psd: 0.0, exponent: None, table_operands: Vec::new() }); }
        noise_variable_0 = 0.0;
        noise_variable_1 = 0.0;
        noise_variable_2 = 0.0;
        noise_variable_3 = 0.0;
        noise_variable_4 = 0.0;
        noise_variable_5 = 0.0;
        noise_variable_6 = 0.0;
        noise_variable_7 = 0.0;
        noise_variable_8 = 0.0;
        noise_variable_9 = 0.0;
        noise_variable_10 = 0.0;
        noise_variable_11 = 0.0;
        noise_variable_12 = 0.0;
        noise_variable_13 = 0.0;
        noise_variable_14 = 0.0;
        noise_variable_15 = 0.0;
        noise_variable_16 = 0.0;
        noise_variable_17 = 0.0;
        noise_variable_18 = 0.0;
        noise_variable_19 = 0.0;
        noise_variable_20 = 0.0;
        noise_variable_21 = 0.0;
        noise_variable_22 = 0.0;
        noise_variable_23 = 0.0;
        noise_variable_24 = 0.0;
        noise_variable_25 = 0.0;
        noise_variable_26 = 0.0;
        noise_variable_27 = 0.0;
        noise_variable_28 = 0.0;
        noise_variable_29 = 0.0;
        noise_variable_30 = 0.0;
        noise_variable_31 = 0.0;
        noise_variable_32 = 0.0;
        noise_variable_33 = 0.0;
        noise_variable_34 = 0.0;
        noise_variable_35 = 0.0;
        noise_variable_36 = 0.0;
        noise_variable_37 = 0.0;
        noise_variable_38 = 0.0;
        noise_variable_39 = 0.0;
        noise_variable_40 = 0.0;
        noise_variable_41 = 0.0;
        noise_variable_42 = 0.0;
        noise_variable_43 = 0.0;
        noise_variable_44 = 0.0;
        noise_variable_45 = 0.0;
        noise_variable_46 = 0.0;
        noise_variable_47 = 0.0;
        noise_variable_48 = 0.0;
        noise_variable_49 = 0.0;
        noise_variable_50 = 0.0;
        noise_variable_51 = 0.0;
        noise_variable_52 = 0.0;
        noise_variable_53 = 0.0;
        noise_variable_54 = 0.0;
        noise_variable_55 = 0.0;
        noise_variable_56 = 0.0;
        noise_variable_57 = 0.0;
        noise_variable_58 = 0.0;
        noise_variable_59 = 0.0;
        noise_variable_60 = 0.0;
        noise_variable_61 = 0.0;
        noise_variable_62 = 0.0;
        noise_variable_63 = 0.0;
        noise_variable_64 = 0.0;
        noise_variable_65 = 0.0;
        noise_variable_66 = 0.0;
        noise_variable_67 = 0.0;
        noise_variable_68 = 0.0;
        noise_variable_69 = 0.0;
        noise_variable_70 = 0.0;
        noise_variable_71 = 0.0;
        noise_variable_72 = 0.0;
        noise_variable_73 = 0.0;
        noise_variable_74 = 0.0;
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_0_e300: f64 = ctx.temperature();
            let noise_metadata_schedule_0_e302: f64 = (noise_metadata_schedule_0_e300 + (ctx.node_voltage(self.nodes[2]) - 0.0));
            let noise_metadata_schedule_0_e304: f64 = (noise_metadata_schedule_0_e302 + params.p45);
            noise_variable_12 = noise_metadata_schedule_0_e304;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_1_e307: f64 = (1026.85 + 273.15);
            let noise_metadata_schedule_1_e310: f64 = (-100.0);
            let noise_metadata_schedule_1_e312: f64 = (noise_metadata_schedule_1_e310 + 273.15);
            let (noise_metadata_schedule_1_e319,) = {
    if (noise_variable_12 > noise_metadata_schedule_1_e312) {
        (noise_variable_12,)
    } else {
        let noise_metadata_schedule_1_e316: f64 = (-100.0);
        let noise_metadata_schedule_1_e318: f64 = (noise_metadata_schedule_1_e316 + 273.15);
        (noise_metadata_schedule_1_e318,)
    }
};
            let (noise_metadata_schedule_1_e336,) = {
    if (noise_metadata_schedule_1_e307 < noise_metadata_schedule_1_e319) {
        let noise_metadata_schedule_1_e323: f64 = (1026.85 + 273.15);
        (noise_metadata_schedule_1_e323,)
    } else {
        let noise_metadata_schedule_1_e326: f64 = (-100.0);
        let noise_metadata_schedule_1_e328: f64 = (noise_metadata_schedule_1_e326 + 273.15);
        let (noise_metadata_schedule_1_e335,) = {
            if (noise_variable_12 > noise_metadata_schedule_1_e328) {
                (noise_variable_12,)
            } else {
                let noise_metadata_schedule_1_e332: f64 = (-100.0);
                let noise_metadata_schedule_1_e334: f64 = (noise_metadata_schedule_1_e332 + 273.15);
                (noise_metadata_schedule_1_e334,)
            }
        };
        (noise_metadata_schedule_1_e335,)
    }
};
            noise_variable_10 = noise_metadata_schedule_1_e336;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_4_e346: f64 = (params.p43 * params.p42);
            noise_variable_3 = noise_metadata_schedule_4_e346;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_5_e349: f64 = (params.p25 + 273.15);
            noise_variable_11 = noise_metadata_schedule_5_e349;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_6_e352: f64 = (8.6170869e-5 * noise_variable_10);
            noise_variable_15 = noise_metadata_schedule_6_e352;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_7_e355: f64 = (noise_variable_10 / noise_variable_11);
            noise_variable_13 = noise_metadata_schedule_7_e355;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            let noise_metadata_schedule_8_e357: f64 = (noise_variable_13).ln();
            noise_variable_14 = noise_metadata_schedule_8_e357;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_9_e360: f64 = (params.p22 * noise_variable_14);
            let noise_metadata_schedule_9_e364: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_9_e365: f64 = (params.p21 * noise_metadata_schedule_9_e364);
            let noise_metadata_schedule_9_e367: f64 = (noise_metadata_schedule_9_e365 / noise_variable_15);
            let noise_metadata_schedule_9_e368: f64 = (noise_metadata_schedule_9_e360 + noise_metadata_schedule_9_e367);
            noise_variable_34 = noise_metadata_schedule_9_e368;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_10_e371: f64 = (params.p23 * noise_variable_14);
            noise_variable_54 = noise_metadata_schedule_10_e371;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_11_e374: f64 = (noise_variable_34).exp();
            let noise_metadata_schedule_11_e375: f64 = (params.p0 * noise_metadata_schedule_11_e374);
            noise_variable_16 = noise_metadata_schedule_11_e375;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_12_e378: f64 = (noise_variable_54).exp();
            let noise_metadata_schedule_12_e379: f64 = (params.p2 * noise_metadata_schedule_12_e378);
            noise_variable_55 = noise_metadata_schedule_12_e379;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_13_e385: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_13_e386: f64 = (params.p7 * noise_metadata_schedule_13_e385);
            let noise_metadata_schedule_13_e387: f64 = (1.0 + noise_metadata_schedule_13_e386);
            let noise_metadata_schedule_13_e388: f64 = (params.p47 * noise_metadata_schedule_13_e387);
            noise_variable_19 = noise_metadata_schedule_13_e388;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_14_e394: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_14_e395: f64 = (params.p6 * noise_metadata_schedule_14_e394);
            let noise_metadata_schedule_14_e396: f64 = (1.0 + noise_metadata_schedule_14_e395);
            let noise_metadata_schedule_14_e397: f64 = (params.p5 * noise_metadata_schedule_14_e396);
            noise_variable_20 = noise_metadata_schedule_14_e397;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_15_e403: f64 = (noise_variable_13 - 1.0);
            let noise_metadata_schedule_15_e404: f64 = (params.p10 * noise_metadata_schedule_15_e403);
            let noise_metadata_schedule_15_e405: f64 = (1.0 + noise_metadata_schedule_15_e404);
            let noise_metadata_schedule_15_e406: f64 = (params.p9 * noise_metadata_schedule_15_e405);
            noise_variable_21 = noise_metadata_schedule_15_e406;
        }
        if matches!(source_index, 0 | 1 | 2 | 3) {
            noise_variable_9 = params.p29;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_29_e503: f64 = (noise_variable_9 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_40 = noise_metadata_schedule_29_e503;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_30_e506: f64 = (noise_variable_9 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[3])));
            noise_variable_41 = noise_metadata_schedule_30_e506;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_31_e509: f64 = (noise_variable_9 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[4])));
            noise_variable_42 = noise_metadata_schedule_31_e509;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_32_e512: f64 = if noise_variable_16 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_63 = noise_metadata_schedule_32_e512;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_33_e520,) = {
    if (noise_variable_63 != 0.0) {
        let noise_metadata_schedule_33_e517: f64 = (params.p1 * noise_variable_15);
        let noise_metadata_schedule_33_e518: f64 = (noise_variable_40 / noise_metadata_schedule_33_e517);
        (noise_metadata_schedule_33_e518,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_33_e520;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_34_e531,) = {
    if (noise_variable_63 != 0.0) {
        let noise_metadata_schedule_34_e523: f64 = (-noise_variable_40);
        let noise_metadata_schedule_34_e525: f64 = (noise_metadata_schedule_34_e523 - noise_variable_20);
        let noise_metadata_schedule_34_e528: f64 = (params.p11 * noise_variable_15);
        let noise_metadata_schedule_34_e529: f64 = (noise_metadata_schedule_34_e525 / noise_metadata_schedule_34_e528);
        (noise_metadata_schedule_34_e529,)
    } else {
        (noise_variable_52,)
    }
};
            noise_variable_52 = noise_metadata_schedule_34_e531;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_35_e540,) = {
    if (noise_variable_63 != 0.0) {
        let noise_metadata_schedule_35_e534: f64 = (-noise_variable_20);
        let noise_metadata_schedule_35_e537: f64 = (params.p11 * noise_variable_15);
        let noise_metadata_schedule_35_e538: f64 = (noise_metadata_schedule_35_e534 / noise_metadata_schedule_35_e537);
        (noise_metadata_schedule_35_e538,)
    } else {
        (noise_variable_53,)
    }
};
            noise_variable_53 = noise_metadata_schedule_35_e540;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_36_e543: f64 = if noise_variable_0 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_64 = noise_metadata_schedule_36_e543;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_37_e553,) = {
    if ((noise_variable_63 != 0.0) && (noise_variable_64 != 0.0)) {
        let noise_metadata_schedule_37_e550: f64 = (noise_variable_0 - 80.0);
        let noise_metadata_schedule_37_e551: f64 = (1.0 + noise_metadata_schedule_37_e550);
        (noise_metadata_schedule_37_e551,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_37_e553;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_38_e559,) = {
    if ((noise_variable_63 != 0.0) && (noise_variable_64 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_38_e559;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_39_e566,) = {
    if ((noise_variable_63 != 0.0) && (noise_variable_64 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_39_e566;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_40_e573,) = {
    if (noise_variable_63 != 0.0) {
        let noise_metadata_schedule_40_e570: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_40_e571: f64 = (noise_variable_1 * noise_metadata_schedule_40_e570);
        (noise_metadata_schedule_40_e571,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_40_e573;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_41_e645,) = {
    if (noise_variable_63 != 0.0) {
        let noise_metadata_schedule_41_e581: f64 = (-37.0);
        let (noise_metadata_schedule_41_e608,) = {
            if ((!(noise_variable_52 >= 37.0)) && (!(noise_variable_52 <= noise_metadata_schedule_41_e581))) {
                let noise_metadata_schedule_41_e586: f64 = (noise_variable_52).exp();
                let noise_metadata_schedule_41_e588: f64 = (noise_metadata_schedule_41_e586 + 1.0);
                let noise_metadata_schedule_41_e589: f64 = (noise_metadata_schedule_41_e588).ln();
                (noise_metadata_schedule_41_e589,)
            } else {
                let noise_metadata_schedule_41_e596: f64 = (-37.0);
                let (noise_metadata_schedule_41_e607,) = {
                    if ((!(noise_variable_52 >= 37.0)) && (noise_variable_52 <= noise_metadata_schedule_41_e596)) {
                        let noise_metadata_schedule_41_e600: f64 = (noise_variable_52).exp();
                        (noise_metadata_schedule_41_e600,)
                    } else {
                        let (noise_metadata_schedule_41_e606,) = {
                            if (noise_variable_52 >= 37.0) {
                                (noise_variable_52,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_41_e606,)
                    }
                };
                (noise_metadata_schedule_41_e607,)
            }
        };
        let noise_metadata_schedule_41_e615: f64 = (-37.0);
        let (noise_metadata_schedule_41_e642,) = {
            if ((!(noise_variable_53 >= 37.0)) && (!(noise_variable_53 <= noise_metadata_schedule_41_e615))) {
                let noise_metadata_schedule_41_e620: f64 = (noise_variable_53).exp();
                let noise_metadata_schedule_41_e622: f64 = (noise_metadata_schedule_41_e620 + 1.0);
                let noise_metadata_schedule_41_e623: f64 = (noise_metadata_schedule_41_e622).ln();
                (noise_metadata_schedule_41_e623,)
            } else {
                let noise_metadata_schedule_41_e630: f64 = (-37.0);
                let (noise_metadata_schedule_41_e641,) = {
                    if ((!(noise_variable_53 >= 37.0)) && (noise_variable_53 <= noise_metadata_schedule_41_e630)) {
                        let noise_metadata_schedule_41_e634: f64 = (noise_variable_53).exp();
                        (noise_metadata_schedule_41_e634,)
                    } else {
                        let (noise_metadata_schedule_41_e640,) = {
                            if (noise_variable_53 >= 37.0) {
                                (noise_variable_53,)
                            } else {
                                (0.0,)
                            }
                        };
                        (noise_metadata_schedule_41_e640,)
                    }
                };
                (noise_metadata_schedule_41_e641,)
            }
        };
        let noise_metadata_schedule_41_e643: f64 = (noise_metadata_schedule_41_e608 - noise_metadata_schedule_41_e642);
        (noise_metadata_schedule_41_e643,)
    } else {
        (noise_variable_2,)
    }
};
            noise_variable_2 = noise_metadata_schedule_41_e645;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_42_e666,) = {
    if (noise_variable_63 != 0.0) {
        let noise_metadata_schedule_42_e650: f64 = (noise_variable_1 - 1.0);
        let noise_metadata_schedule_42_e651: f64 = (noise_variable_16 * noise_metadata_schedule_42_e650);
        let noise_metadata_schedule_42_e654: f64 = (noise_variable_19 * noise_variable_2);
        let noise_metadata_schedule_42_e658: f64 = (noise_variable_40).abs();
        let noise_metadata_schedule_42_e660: f64 = (noise_metadata_schedule_42_e658).powf(noise_variable_21);
        let noise_metadata_schedule_42_e661: f64 = (params.p8 * noise_metadata_schedule_42_e660);
        let noise_metadata_schedule_42_e662: f64 = (1.0 + noise_metadata_schedule_42_e661);
        let noise_metadata_schedule_42_e663: f64 = (noise_metadata_schedule_42_e654 / noise_metadata_schedule_42_e662);
        let noise_metadata_schedule_42_e664: f64 = (noise_metadata_schedule_42_e651 - noise_metadata_schedule_42_e663);
        (noise_metadata_schedule_42_e664,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_42_e666;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_43_e671,) = {
    if (noise_variable_63 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_23,)
    }
};
            noise_variable_23 = noise_metadata_schedule_43_e671;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_44_e674: f64 = if noise_variable_55 > 0.0 { 1.0 } else { 0.0 };
            noise_variable_65 = noise_metadata_schedule_44_e674;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_45_e682,) = {
    if (noise_variable_65 != 0.0) {
        let noise_metadata_schedule_45_e678: f64 = (params.p4 - noise_variable_40);
        let noise_metadata_schedule_45_e680: f64 = (noise_metadata_schedule_45_e678).max(0.001);
        (noise_metadata_schedule_45_e680,)
    } else {
        (noise_variable_60,)
    }
};
            noise_variable_60 = noise_metadata_schedule_45_e682;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_46_e697,) = {
    if (noise_variable_65 != 0.0) {
        let noise_metadata_schedule_46_e685: f64 = (-1.0);
        let noise_metadata_schedule_46_e687: f64 = (noise_metadata_schedule_46_e685 * noise_variable_40);
        let noise_metadata_schedule_46_e689: f64 = (noise_metadata_schedule_46_e687 * params.p4);
        let noise_metadata_schedule_46_e692: f64 = (params.p3 * noise_variable_15);
        let noise_metadata_schedule_46_e694: f64 = (noise_metadata_schedule_46_e692 * noise_variable_60);
        let noise_metadata_schedule_46_e695: f64 = (noise_metadata_schedule_46_e689 / noise_metadata_schedule_46_e694);
        (noise_metadata_schedule_46_e695,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_46_e697;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_47_e700: f64 = if noise_variable_0 > 80.0 { 1.0 } else { 0.0 };
            noise_variable_66 = noise_metadata_schedule_47_e700;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_48_e710,) = {
    if ((noise_variable_65 != 0.0) && (noise_variable_66 != 0.0)) {
        let noise_metadata_schedule_48_e707: f64 = (noise_variable_0 - 80.0);
        let noise_metadata_schedule_48_e708: f64 = (1.0 + noise_metadata_schedule_48_e707);
        (noise_metadata_schedule_48_e708,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_48_e710;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_49_e716,) = {
    if ((noise_variable_65 != 0.0) && (noise_variable_66 != 0.0)) {
        (80.0,)
    } else {
        (noise_variable_0,)
    }
};
            noise_variable_0 = noise_metadata_schedule_49_e716;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_50_e723,) = {
    if ((noise_variable_65 != 0.0) && (noise_variable_66 == 0.0)) {
        (1.0,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_50_e723;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_51_e730,) = {
    if (noise_variable_65 != 0.0) {
        let noise_metadata_schedule_51_e727: f64 = (noise_variable_0).exp();
        let noise_metadata_schedule_51_e728: f64 = (noise_variable_1 * noise_metadata_schedule_51_e727);
        (noise_metadata_schedule_51_e728,)
    } else {
        (noise_variable_1,)
    }
};
            noise_variable_1 = noise_metadata_schedule_51_e730;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_52_e738,) = {
    if (noise_variable_65 != 0.0) {
        let noise_metadata_schedule_52_e735: f64 = (noise_variable_1 - 1.0);
        let noise_metadata_schedule_52_e736: f64 = (noise_variable_55 * noise_metadata_schedule_52_e735);
        (noise_metadata_schedule_52_e736,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_52_e738;
        }
        if matches!(source_index, 2 | 3) {
            let (noise_metadata_schedule_53_e743,) = {
    if (noise_variable_65 == 0.0) {
        (0.0,)
    } else {
        (noise_variable_26,)
    }
};
            noise_variable_26 = noise_metadata_schedule_53_e743;
        }
        if matches!(source_index, 2 | 3) {
            let noise_metadata_schedule_54_e746: f64 = (noise_variable_23 - noise_variable_26);
            noise_variable_24 = noise_metadata_schedule_54_e746;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_55_e750: f64 = (noise_variable_41 / params.p48);
            let noise_metadata_schedule_55_e751: f64 = (noise_metadata_schedule_55_e750).abs();
            let noise_metadata_schedule_55_e753: f64 = (noise_metadata_schedule_55_e751).powf(params.p49);
            let noise_metadata_schedule_55_e754: f64 = (1.0 + noise_metadata_schedule_55_e753);
            noise_variable_58 = noise_metadata_schedule_55_e754;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_56_e758: f64 = (noise_variable_42 / params.p50);
            let noise_metadata_schedule_56_e759: f64 = (noise_metadata_schedule_56_e758).abs();
            let noise_metadata_schedule_56_e761: f64 = (noise_metadata_schedule_56_e759).powf(params.p51);
            let noise_metadata_schedule_56_e762: f64 = (1.0 + noise_metadata_schedule_56_e761);
            noise_variable_59 = noise_metadata_schedule_56_e762;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_57_e766: f64 = (noise_variable_14 * params.p37);
            let noise_metadata_schedule_57_e767: f64 = (noise_metadata_schedule_57_e766).exp();
            let noise_metadata_schedule_57_e768: f64 = (params.p12 * noise_metadata_schedule_57_e767);
            let noise_metadata_schedule_57_e772: f64 = (1.0 / params.p49);
            let noise_metadata_schedule_57_e773: f64 = (noise_variable_58).powf(noise_metadata_schedule_57_e772);
            let noise_metadata_schedule_57_e774: f64 = (noise_metadata_schedule_57_e768 * noise_metadata_schedule_57_e773);
            noise_variable_29 = noise_metadata_schedule_57_e774;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_58_e778: f64 = (noise_variable_14 * params.p38);
            let noise_metadata_schedule_58_e779: f64 = (noise_metadata_schedule_58_e778).exp();
            let noise_metadata_schedule_58_e780: f64 = (params.p14 * noise_metadata_schedule_58_e779);
            let noise_metadata_schedule_58_e784: f64 = (1.0 / params.p51);
            let noise_metadata_schedule_58_e785: f64 = (noise_variable_59).powf(noise_metadata_schedule_58_e784);
            let noise_metadata_schedule_58_e786: f64 = (noise_metadata_schedule_58_e780 * noise_metadata_schedule_58_e785);
            noise_variable_30 = noise_metadata_schedule_58_e786;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_59_e789: f64 = if params.p31 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_67 = noise_metadata_schedule_59_e789;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_60_e795,) = {
    if (noise_variable_67 != 0.0) {
        let noise_metadata_schedule_60_e793: f64 = (noise_variable_29 + params.p13);
        (noise_metadata_schedule_60_e793,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_60_e795;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_61_e801,) = {
    if (noise_variable_67 != 0.0) {
        let noise_metadata_schedule_61_e799: f64 = (noise_variable_30 + params.p15);
        (noise_metadata_schedule_61_e799,)
    } else {
        (noise_variable_30,)
    }
};
            noise_variable_30 = noise_metadata_schedule_61_e801;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_67_e830: f64 = if params.p32 == 1.0 { 1.0 } else { 0.0 };
            noise_variable_68 = noise_metadata_schedule_67_e830;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_68_e843,) = {
    if (noise_variable_68 != 0.0) {
        let noise_metadata_schedule_68_e835: f64 = ((ctx.node_voltage(self.nodes[6]) - 0.0)).abs();
        let noise_metadata_schedule_68_e837: f64 = (noise_metadata_schedule_68_e835 / params.p20);
        let noise_metadata_schedule_68_e839: f64 = (noise_metadata_schedule_68_e837).powf(params.p44);
        let noise_metadata_schedule_68_e840: f64 = (1.0 + noise_metadata_schedule_68_e839);
        let noise_metadata_schedule_68_e841: f64 = (noise_variable_29 / noise_metadata_schedule_68_e840);
        (noise_metadata_schedule_68_e841,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_68_e843;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_69_e848,) = {
    if (noise_variable_68 == 0.0) {
        (noise_variable_29,)
    } else {
        (noise_variable_29,)
    }
};
            noise_variable_29 = noise_metadata_schedule_69_e848;
        }
        if matches!(source_index, 0 | 1) {
            let noise_metadata_schedule_82_e967: f64 = (4.0 * 1.3806226e-23);
            let noise_metadata_schedule_82_e969: f64 = (noise_metadata_schedule_82_e967 * noise_variable_10);
            noise_variable_35 = noise_metadata_schedule_82_e969;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_83_e973: f64 = (params.p31 * params.p13);
            let noise_metadata_schedule_83_e974: f64 = (params.p12 + noise_metadata_schedule_83_e973);
            let noise_metadata_schedule_83_e976: f64 = (noise_metadata_schedule_83_e974 / noise_variable_3);
            noise_variable_28 = noise_metadata_schedule_83_e976;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_84_e980: f64 = (params.p31 * params.p15);
            let noise_metadata_schedule_84_e981: f64 = (params.p14 + noise_metadata_schedule_84_e980);
            let noise_metadata_schedule_84_e983: f64 = (noise_metadata_schedule_84_e981 / noise_variable_3);
            noise_variable_27 = noise_metadata_schedule_84_e983;
        }
        if matches!(source_index, 0) {
            let noise_metadata_schedule_85_e990: f64 = if ((noise_variable_28 > 0.0) && (noise_variable_28 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_73 = noise_metadata_schedule_85_e990;
        }
        if matches!(source_index, 0) {
            let (noise_metadata_schedule_86_e1005,) = {
    if (noise_variable_73 != 0.0) {
        let noise_metadata_schedule_86_e994: f64 = (noise_variable_29 / noise_variable_3);
        let (noise_metadata_schedule_86_e1003,) = {
            if (noise_metadata_schedule_86_e994 >= params.p46) {
                let noise_metadata_schedule_86_e1000: f64 = (noise_variable_29 / noise_variable_3);
                let noise_metadata_schedule_86_e1001: f64 = (noise_variable_35 / noise_metadata_schedule_86_e1000);
                (noise_metadata_schedule_86_e1001,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_86_e1003,)
    } else {
        (noise_variable_38,)
    }
};
            noise_variable_38 = noise_metadata_schedule_86_e1005;
        }
        if matches!(source_index, 1) {
            let noise_metadata_schedule_87_e1012: f64 = if ((noise_variable_27 > 0.0) && (noise_variable_27 >= params.p46)) { 1.0 } else { 0.0 };
            noise_variable_74 = noise_metadata_schedule_87_e1012;
        }
        if matches!(source_index, 1) {
            let (noise_metadata_schedule_88_e1027,) = {
    if (noise_variable_74 != 0.0) {
        let noise_metadata_schedule_88_e1016: f64 = (noise_variable_30 / noise_variable_3);
        let (noise_metadata_schedule_88_e1025,) = {
            if (noise_metadata_schedule_88_e1016 >= params.p46) {
                let noise_metadata_schedule_88_e1022: f64 = (noise_variable_30 / noise_variable_3);
                let noise_metadata_schedule_88_e1023: f64 = (noise_variable_35 / noise_metadata_schedule_88_e1022);
                (noise_metadata_schedule_88_e1023,)
            } else {
                (0.0,)
            }
        };
        (noise_metadata_schedule_88_e1025,)
    } else {
        (noise_variable_39,)
    }
};
            noise_variable_39 = noise_metadata_schedule_88_e1027;
        }
        if matches!(source_index, 2) {
            let noise_metadata_schedule_89_e1034: f64 = if ((params.p28 > 0.0) && (params.p27 > 0.0)) { 1.0 } else { 0.0 };
            let (noise_metadata_schedule_89_e1044,) = {
    if (noise_metadata_schedule_89_e1034 > 0.0) {
        let noise_metadata_schedule_89_e1039: f64 = (noise_variable_24).abs();
        let noise_metadata_schedule_89_e1041: f64 = (noise_metadata_schedule_89_e1039).powf(params.p28);
        let noise_metadata_schedule_89_e1042: f64 = (params.p27 * noise_metadata_schedule_89_e1041);
        (noise_metadata_schedule_89_e1042,)
    } else {
        (0.0,)
    }
};
            noise_variable_37 = noise_metadata_schedule_89_e1044;
        }
        if matches!(source_index, 3) {
            let noise_metadata_schedule_90_e1047: f64 = (2.0 * 1.6021918e-19);
            noise_variable_36 = noise_metadata_schedule_90_e1047;
        }
        match source_index {
            0 => {
                let noise_0_psd_e1049: f64 = 1.0;
                let noise_0_psd_e1050: f64 = (noise_0_psd_e1049 * noise_variable_38);
                let psd = noise_0_psd_e1050;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            1 => {
                let noise_1_psd_e1052: f64 = 1.0;
                let noise_1_psd_e1053: f64 = (noise_1_psd_e1052 * noise_variable_39);
                let psd = noise_1_psd_e1053;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            2 => {
                let noise_2_psd_e1055: f64 = 1.0;
                let noise_2_psd_e281: f64 = (noise_variable_9 * noise_variable_24);
                let (noise_2_psd_e288,) = {
    if (noise_2_psd_e281 >= 0.0) {
        let noise_2_psd_e285: f64 = 1.0;
        (noise_2_psd_e285,)
    } else {
        let noise_2_psd_e287: f64 = (-1.0);
        (noise_2_psd_e287,)
    }
};
                let noise_2_psd_e290: f64 = (noise_2_psd_e288 * noise_variable_37);
                let noise_2_psd_e1056: f64 = (noise_2_psd_e1055 * noise_2_psd_e290);
                let psd = noise_2_psd_e1056;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
                let exponent: Option<f64> = Some(1.0);
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            3 => {
                let noise_3_psd_e1058: f64 = 1.0;
                let noise_3_psd_e296: f64 = (noise_variable_24).abs();
                let noise_3_psd_e297: f64 = (noise_variable_36 * noise_3_psd_e296);
                let noise_3_psd_e1059: f64 = (noise_3_psd_e1058 * noise_3_psd_e297);
                let psd = noise_3_psd_e1059;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
                if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
                let exponent: Option<f64> = None;
                if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
                let table_operands = vec![];
                let psd = psd * self.multiplicity;
                if !psd.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
                Ok(GeneratedNoiseEvaluation { active: true, psd, exponent, table_operands })
            }
            _ => unreachable!("noise source index was range checked"),
        }
    }
}

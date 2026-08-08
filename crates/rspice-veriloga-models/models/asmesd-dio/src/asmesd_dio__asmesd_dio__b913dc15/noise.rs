#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

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
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6])];
            let A = 0e0f64;
            let D = 1.7314999999999998e2f64;
            let G = 1.3e3f64;
            let I = 1.7314999999999998e2f64;
            let L = 1e0f64;
            let M = 0.0f64;
            let Z = 3.0015e2f64;
            let AD = parameters[29];
            let AE = node_potentials[3];
            let AF = node_potentials[4];
            let AO = 8e1f64;
            let AU = 3.7e1f64;
            let BJ = parameters[4];
            let BT = parameters[49];
            let BU = parameters[51];
            let BV = parameters[12];
            let BX = parameters[14];
            let BZ = parameters[31];
            let CB = parameters[13];
            let CD = parameters[15];
            let CJ = parameters[30];
            let CR = parameters[46];
            let DH = parameters[28];
            let DI = parameters[27];
            let DN = -1e0f64;
            let B = (temperature + node_potentials[2]) + parameters[45];
            let C = if B > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
            let E = if C != 0.0 {
                B
            } else {
                D
            };
            let F = if 1.3e3f64 < E { 1.0 } else { 0.0 };
            let K;
            if F != 0.0 {
                K = G;
            } else {
                let H = if B > 1.7314999999999998e2f64 { 1.0 } else { 0.0 };
                let J = if H != 0.0 {
                    B
                } else {
                    I
                };
                K = J;
            }
            if M != 0.0 {
            } else {
            }
            let N = if K > parameters[26] { 1.0 } else { 0.0 };
            if N != 0.0 {
            } else {
            }
            let O = parameters[43] * parameters[42];
            let P = parameters[25] + 2.7315e2f64;
            let Q = 8.6170869e-5f64 * K;
            let R = K / P;
            let S = R.ln();
            let T = R - L;
            let U = parameters[0] * (((parameters[22] * S) + ((parameters[21] * T) / Q)).exp());
            let V = parameters[2] * ((parameters[23] * S).exp());
            let W = parameters[47] * (L + (parameters[7] * T));
            let X = parameters[5] * (L + (parameters[6] * T));
            let Y = parameters[9] * (L + (parameters[10] * T));
            let AA = K / Z;
            let AB = (-(Q + Q)) * ((1.5e0f64 * (AA.ln())) + (1.6021918e-19f64 * (((-(1.16e0f64 - (((7.02e-4f64 * K) * K) / (1.108e3f64 + K)))) / (1.3806226e-23f64 * (K + K))) + 1.3454442398941469e20f64)));
            let AC = (AA * ((parameters[17] - AB) / (P / Z))) + AB;
            let AG = AD * (AE - AF);
            let AH = AD * (node_potentials[0] - AE);
            let AI = AD * (node_potentials[1] - AF);
            let AJ = if U > A { 1.0 } else { 0.0 };
            let BQ;
            if AJ != 0.0 {
                let AK = AG / (parameters[1] * Q);
                let AL = parameters[11] * Q;
                let AM = ((-AG) - X) / AL;
                let AN = (-X) / AL;
                let AP = if AK > AO { 1.0 } else { 0.0 };
                let AR;
                let AS;
                if AP != 0.0 {
                    let AQ = L + (AK - AO);
                    AR = AQ;
                    AS = AO;
                } else {
                    AR = L;
                    AS = AK;
                }
                let AT = AR * (AS.exp());
                let AV = if AM >= AU { 1.0 } else { 0.0 };
                let BD;
                if AV != 0.0 {
                    BD = AM;
                } else {
                    let AW = if AM <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let BE = if AW != 0.0 {
                        let AX = AM.exp();
                        AX
                    } else {
                        let AY = ((AM.exp()) + L).ln();
                        AY
                    };
                    BD = BE;
                }
                let AZ = if AN >= AU { 1.0 } else { 0.0 };
                let BF;
                if AZ != 0.0 {
                    BF = AN;
                } else {
                    let BA = if AN <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let BG = if BA != 0.0 {
                        let BB = AN.exp();
                        BB
                    } else {
                        let BC = ((AN.exp()) + L).ln();
                        BC
                    };
                    BF = BG;
                }
                let BH = (U * (AT - L)) - ((W * (BD - BF)) / (L + (parameters[8] * ((AG.abs()).powf(Y)))));
                BQ = BH;
            } else {
                BQ = A;
            }
            let BI = if V > A { 1.0 } else { 0.0 };
            let BR;
            if BI != 0.0 {
                let BK = ((-1e0f64 * AG) * BJ) / ((parameters[3] * Q) * (if (BJ - AG) >= 1e-3f64 { (BJ - AG) } else { 1e-3f64 }));
                let BL = if BK > AO { 1.0 } else { 0.0 };
                let BN;
                let BO;
                if BL != 0.0 {
                    let BM = L + (BK - AO);
                    BN = BM;
                    BO = AO;
                } else {
                    BN = L;
                    BO = BK;
                }
                let BP = V * ((BN * (BO.exp())) - L);
                BR = BP;
            } else {
                BR = A;
            }
            let BS = BQ - BR;
            let BW = (BV * ((S * parameters[37]).exp())) * ((L + (((AH / parameters[48]).abs()).powf(BT))).powf((L / BT)));
            let BY = (BX * ((S * parameters[38]).exp())) * ((L + (((AI / parameters[50]).abs()).powf(BU))).powf((L / BU)));
            let CA = if BZ == L { 1.0 } else { 0.0 };
            let CG;
            let DA;
            if CA != 0.0 {
                let CC = BW + CB;
                let CE = BY + CD;
                CG = CC;
                DA = CE;
            } else {
                CG = BW;
                DA = BY;
            }
            let CF = if parameters[32] == L { 1.0 } else { 0.0 };
            let CT = if CF != 0.0 {
                let CH = CG / (L + (((node_potentials[6].abs()) / parameters[20]).powf(parameters[44])));
                CH
            } else {
                CG
            };
            let CI = if (AG + ((-AC) * parameters[24])) > A { 1.0 } else { 0.0 };
            if CI != 0.0 {
            } else {
            }
            let CK = if parameters[33] > A { 1.0 } else { 0.0 };
            let CL = if (if CJ == L { 1.0 } else { 0.0 }) != 0.0 && CK != 0.0 { 1.0 } else { 0.0 };
            if CL != 0.0 {
            } else {
                let CM = if (if (if CJ == 2e0f64 { 1.0 } else { 0.0 }) != 0.0 && CK != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if parameters[35] > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if CM != 0.0 {
                } else {
                    let CN = if CJ == -1e0f64 { 1.0 } else { 0.0 };
                    if CN != 0.0 {
                    } else {
                    }
                }
            }
            let CO = 5.5224904e-23f64 * K;
            let CP = (BV + (BZ * CB)) / O;
            let CQ = (BX + (BZ * CD)) / O;
            let CS = if (if CP > A { 1.0 } else { 0.0 }) != 0.0 && (if CP >= CR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DR;
            let DS;
            if CS != 0.0 {
                let CU = CT / O;
                let CV = if CU > CR { 1.0 } else { 0.0 };
                if CV != 0.0 {
                } else {
                }
                let CW = if CU >= CR { 1.0 } else { 0.0 };
                let CY = if CW != 0.0 {
                    let CX = CO / CU;
                    CX
                } else {
                    A
                };
                DR = L;
                DS = CY;
            } else {
                DR = A;
                DS = A;
            }
            let CZ = if (if CQ > A { 1.0 } else { 0.0 }) != 0.0 && (if CQ >= CR { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let DT;
            let DU;
            if CZ != 0.0 {
                let DB = DA / O;
                let DC = if DB > CR { 1.0 } else { 0.0 };
                if DC != 0.0 {
                } else {
                }
                let DD = if DB >= CR { 1.0 } else { 0.0 };
                let DF = if DD != 0.0 {
                    let DE = CO / DB;
                    DE
                } else {
                    A
                };
                DT = L;
                DU = DF;
            } else {
                DT = A;
                DU = A;
            }
            let DG = AD * BS;
            let DJ = if (if (if DH > A { 1.0 } else { 0.0 }) != 0.0 && (if DI > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) > A { 1.0 } else { 0.0 };
            let DL = if DJ != 0.0 {
                let DK = DI * ((BS.abs()).powf(DH));
                DK
            } else {
                A
            };
            let DM = if DG >= A { 1.0 } else { 0.0 };
            let DO = if DM != 0.0 {
                L
            } else {
                DN
            };
            let DP = DO * DL;
            let DQ = 3.2043836e-19f64 * (BS.abs());
        if DR == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DS;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DT == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DU;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = DP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(L);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = DQ;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

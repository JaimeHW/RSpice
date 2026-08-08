#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 2] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_S_THERMAL", label: Some("thermal"), kind: GeneratedNoiseKind::White, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_D_S_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3])];
            let A = 0e0f64;
            let B = 1.0359399871014713e-10f64;
            let C = parameters[13];
            let F = parameters[25];
            let H = 3e0f64;
            let K = parameters[35];
            let M = parameters[22];
            let O = parameters[30];
            let Q = parameters[0];
            let S = 5e-1f64;
            let T = 3.333333333333e-1f64;
            let V = parameters[3];
            let Y = 2.7315e2f64;
            let AA = parameters[4];
            let AC = 2.9815e2f64;
            let AG = 1e-1f64;
            let AI = 1e0f64;
            let AP = 1.16e0f64;
            let AQ = 7.02e-4f64;
            let AR = 1.108e3f64;
            let AZ = 2e-1f64;
            let BM = parameters[38];
            let BN = 1e-6f64;
            let BV = parameters[39];
            let CA = parameters[40];
            let CC = parameters[17];
            let CH = parameters[8];
            let CL = node_potentials[3];
            let CQ = -1e0f64;
            let CU = 2e0f64;
            let DC = 2.5e-1f64;
            let DO = 1.3e0f64;
            let DP = 1.6e0f64;
            let DV = 1.55e0f64;
            let EC = 1e-64f64;
            let HD = parameters[21];
            let HS = 4e0f64;
            let IL = 0e0f64;
            let IU = parameters[1];
            let IX = parameters[41];
            let JG = parameters[58];
            let JH = 7e1f64;
            let D = B / C;
            let E = (D * parameters[14]).sqrt();
            let G = E * F;
            let I = (H * D) * parameters[28];
            let J = D * parameters[29];
            let L = K + K;
            let N = C / (B * M);
            let P = (O + O) / C;
            let R = if Q > A { 1.0 } else { 0.0 };
            let U = if R != 0.0 {
                S
            } else {
                T
            };
            let W = if V == 1e21f64 { 1.0 } else { 0.0 };
            let AE = if W != 0.0 {
                let X = temperature + parameters[2];
                X
            } else {
                let Z = V + Y;
                Z
            };
            let AB = if AA == 1e21f64 { 1.0 } else { 0.0 };
            let AS = if AB != 0.0 {
                AC
            } else {
                let AD = AA + Y;
                AD
            };
            let AF = AE * 8.617333262e-5f64;
            let AH = AG * AF;
            let AJ = AI / AF;
            let AK = AF + AF;
            let AL = AK + AK;
            let AM = AF * AF;
            let AN = AM + AM;
            let AO = 1.6e1f64 * AM;
            let AT = AE - AS;
            let AU = AE / AS;
            let AV = parameters[15] - (parameters[16] * AT);
            let AW = parameters[19] * (AU.powf(parameters[20]));
            let AX = parameters[23] * (AU.powf(parameters[24]));
            let AY = parameters[33] * (AI + (parameters[34] * AT));
            let BA = ((((parameters[18] * AU) - ((H * AF) * (AU.ln()))) - ((AP - (((AQ * AS) * AS) / (AS + AR))) * AU)) + (AP - (((AQ * AE) * AE) / (AE + AR)))) - AZ;
            let BB = (S * (BA + (((BA * BA) + AM).sqrt()))) + AZ;
            let BC = BB.sqrt();
            let BD = AI / AX;
            let BE = E * AX;
            let BF = E * AY;
            let BG = parameters[32] / AY;
            let BH = parameters[5] + parameters[26];
            let BI = parameters[6] + parameters[27];
            let BJ = AX * BH;
            let BK = AF * ((((S * BJ) * AJ).ln()) - 6e-1f64);
            let BL = AI / ((BI * BH).sqrt());
            let CR;
            if R != 0.0 {
                let BO = if BM != BN { 1.0 } else { 0.0 };
                let BQ = if BO != 0.0 {
                    let BP = (BL * (BM - BN)) + AV;
                    BP
                } else {
                    AV
                };
                CR = BQ;
            } else {
                let BR = if BM != BN { 1.0 } else { 0.0 };
                let BU = if BR != 0.0 {
                    let BS = (BL * (BN - BM)) - AV;
                    BS
                } else {
                    let BT = -AV;
                    BT
                };
                CR = BU;
            }
            let BW = if BV != BN { 1.0 } else { 0.0 };
            let BY = if BW != 0.0 {
                let BX = AW * (AI + ((BV - BN) * BL));
                BX
            } else {
                AW
            };
            let BZ = BI * BY;
            let CB = if CA != BN { 1.0 } else { 0.0 };
            let CE = if CB != 0.0 {
                let CD = CC + ((CA - BN) * BL);
                CD
            } else {
                CC
            };
            let CF = CE * BC;
            let CG = if P == A { 1.0 } else { 0.0 };
            let CS = if CG != 0.0 {
                A
            } else {
                let CI = 2.8e-1f64 * ((BH / (parameters[31] * CH)) - AG);
                let CJ = AI / (AI + (S * (CI + (((CI * CI) + 1.936e-3f64).sqrt()))));
                let CK = (P * CJ) * CJ;
                CK
            };
            let CM = Q * (node_potentials[1] - CL);
            let CN = Q * (node_potentials[2] - CL);
            let CO = Q * (node_potentials[0] - CL);
            let CP = if (CO - CN) < A { 1.0 } else { 0.0 };
            let CX;
            let CZ;
            let IS;
            if CP != 0.0 {
                CX = CO;
                CZ = CN;
                IS = CQ;
            } else {
                CX = CN;
                CZ = CO;
                IS = AI;
            }
            let CT = (((CM - CR) - CS) + BB) + CF;
            let CV = ((CT * CT) + (CU * AO)).sqrt();
            let CW = S * (CT + CV);
            let CY = BB + CX;
            let DA = BB + CZ;
            let DB = (I * parameters[7]) / BI;
            let DD = (CW + ((DC * CE) * CE)).sqrt();
            let DE = CW - BB;
            let DF = DD - (S * CE);
            let DG = (((DE - (CE * DF)) + BB) + AH).sqrt();
            let DH = (CE - (((J * CH) / BH) * (((S * (CY + (((CY * CY) + AO).sqrt()))).sqrt()) + ((S * (DA + (((DA * DA) + AO).sqrt()))).sqrt())))) + (DB * DG);
            let DI = ((DH * DH) + AH).sqrt();
            let DJ = S * (DH + DI);
            let DK = (CW + ((DC * DJ) * DJ)).sqrt();
            let DL = DE - (DJ * (DK - (S * DJ)));
            let DM = (DL - CX) * AJ;
            let DN = if DM > -3.5e-1f64 { 1.0 } else { 0.0 };
            let EE;
            if DN != 0.0 {
                let DQ = CU / ((DO + DM) - ((DM + DP).ln()));
                let DR = AI + DM;
                let DS = (CU + DQ) / (DR + (DQ.ln()));
                let DT = (DR + (DS.ln())) / (CU + DS);
                EE = DT;
            } else {
                let DU = if DM > -1.5e1f64 { 1.0 } else { 0.0 };
                let EF;
                if DU != 0.0 {
                    let DW = DV + ((-DM).exp());
                    let DX = AI + DM;
                    let DY = (CU + DW) / (DX + (DW.ln()));
                    let DZ = (DX + (DY.ln())) / (CU + DY);
                    EF = DZ;
                } else {
                    let EA = if DM > -2.3e1f64 { 1.0 } else { 0.0 };
                    let EG = if EA != 0.0 {
                        let EB = AI / (CU + ((-DM).exp()));
                        EB
                    } else {
                        let ED = (DM.exp()) + EC;
                        ED
                    };
                    EF = EG;
                }
                EE = EF;
            }
            let EH = EE * (AI + EE);
            let EI = EH.sqrt();
            let EJ = AF / BJ;
            let EK = (DC + (EI * EJ)).sqrt();
            let EL = BJ * (EK - S);
            let EM = CZ - CX;
            let EN = S * EM;
            let EO = AO * ((F * (EI - (EL * AJ))) + 1.5625e-2f64);
            let EP = ((EL * EL) + EO).sqrt();
            let EQ = EN - EL;
            let ER = ((EQ * EQ) + EO).sqrt();
            let ES = EP - ER;
            let ET = (DC + ((EI - (7.5e-1f64 * (EH.ln()))) * EJ)).sqrt();
            let EU = (BJ * (ET - S)) + BK;
            let EV = EN - EU;
            let EW = ((EU * EU) + EO).sqrt();
            let EX = ((EV * EV) + EO).sqrt();
            let EY = ((((DL - EN) - CX) - EW) + EX) * AJ;
            let EZ = if EY > -3.5e-1f64 { 1.0 } else { 0.0 };
            let FM;
            if EZ != 0.0 {
                let FA = CU / ((DO + EY) - ((EY + DP).ln()));
                let FB = AI + EY;
                let FC = (CU + FA) / (FB + (FA.ln()));
                let FD = (FB + (FC.ln())) / (CU + FC);
                FM = FD;
            } else {
                let FE = if EY > -1.5e1f64 { 1.0 } else { 0.0 };
                let FN;
                if FE != 0.0 {
                    let FF = DV + ((-EY).exp());
                    let FG = AI + EY;
                    let FH = (CU + FF) / (FG + (FF.ln()));
                    let FI = (FG + (FH.ln())) / (CU + FH);
                    FN = FI;
                } else {
                    let FJ = if EY > -2.3e1f64 { 1.0 } else { 0.0 };
                    let FO = if FJ != 0.0 {
                        let FK = AI / (CU + ((-EY).exp()));
                        FK
                    } else {
                        let FL = (EY.exp()) + EC;
                        FL
                    };
                    FN = FO;
                }
                FM = FN;
            }
            let FP = FM * (AI + FM);
            let FQ = (BH - (G * ((AI + ((EN - ES) / BE)).ln()))) + ((EN + ES) * BD);
            let FR = AG * BH;
            let FS = ((FQ * FQ) + (FR * FR)).sqrt();
            let FT = S * (FQ + FS);
            let FU = (DL - CZ) * AJ;
            let FV = if FU > -3.5e-1f64 { 1.0 } else { 0.0 };
            let GI;
            if FV != 0.0 {
                let FW = CU / ((DO + FU) - ((FU + DP).ln()));
                let FX = AI + FU;
                let FY = (CU + FW) / (FX + (FW.ln()));
                let FZ = (FX + (FY.ln())) / (CU + FY);
                GI = FZ;
            } else {
                let GA = if FU > -1.5e1f64 { 1.0 } else { 0.0 };
                let GJ;
                if GA != 0.0 {
                    let GB = DV + ((-FU).exp());
                    let GC = AI + FU;
                    let GD = (CU + GB) / (GC + (GB.ln()));
                    let GE = (GC + (GD.ln())) / (CU + GD);
                    GJ = GE;
                } else {
                    let GF = if FU > -2.3e1f64 { 1.0 } else { 0.0 };
                    let GK = if GF != 0.0 {
                        let GG = AI / (CU + ((-FU).exp()));
                        GG
                    } else {
                        let GH = (FU.exp()) + EC;
                        GH
                    };
                    GJ = GK;
                }
                GI = GJ;
            }
            let GL = DC + EH;
            let GM = DC + (GI * (AI + GI));
            let GN = GL.sqrt();
            let GO = GM.sqrt();
            let GP = GN + GO;
            let GQ = GP * GP;
            let GR = DL + BB;
            let GS = GR + BN;
            let GT = CU * (GS.sqrt());
            let GU = CE / GT;
            let GV = CE / (GT + CE);
            let GW = AI + GU;
            let GX = (-GW) * AF;
            let GY = GX * (((1.33333332e0f64 * ((GM + (GO * GN)) + GL)) / GP) - AI);
            let GZ = ((-5e-1f64 * CE) * GT) - (GV * GY);
            let HA = if M == A { 1.0 } else { 0.0 };
            let HQ;
            let IG;
            let IH;
            let II;
            let IK;
            if HA != 0.0 {
                let HB = ((DL * DL) + AN).sqrt();
                let HC = S * (DL + HB);
                let HE = AI + (HD * HC);
                let HF = BZ / (FT * HE);
                HQ = HF;
                IG = HC;
                IH = HE;
                II = HB;
                IK = IL;
            } else {
                let HG = GZ + (U * GY);
                let HH = if HG > A { 1.0 } else { 0.0 };
                let HK = if HH != 0.0 {
                    let HI = AI + (N * HG);
                    HI
                } else {
                    let HJ = AI - (N * HG);
                    HJ
                };
                let HL = (BZ * (AI + (N * CF))) / (FT * HK);
                HQ = HL;
                IG = A;
                IH = A;
                II = A;
                IK = HK;
            }
            let HM = GR + AL;
            let HN = HM.sqrt();
            let HO = AI + (CE / (CU * HN));
            let HP = EH - FP;
            let HR = (AN * HO) * HQ;
            let HT = HQ * (GY.abs());
            let HU = CW / CV;
            let HV = ((-(GR / DK)) * ((((DB * (DJ / (DI + DI))) * DF) / (DD * DG)) * HU)) + ((AI - (DJ / (DK + DK))) * HU);
            let HW = (EE * AJ) * HV;
            let HX = (AF / ((HS * EK) * EI)) * HW;
            let HY = ((AL + AL) * F) * ((HW * (AF / (EI + EI))) - HX);
            let HZ = (((EL * HX) + HY) * (AI / EP)) - (((EQ * (-HX)) + HY) * (AI / ER));
            let IA = ((AF * (EI - 1.5e0f64)) / ((HS * ET) * EH)) * HW;
            let IB = (FM * AJ) * ((HV - (((EU * IA) + HY) * (AI / EW))) + (((EV * (-IA)) + HY) * (AI / EX)));
            let IC = (AI / FS) * ((-((-(G / ((BE + EN) - ES))) * HZ)) + (HZ * BD));
            let ID = (GX * 6.6666666e-1f64) / GQ;
            let IE = (((((-GU) * GY) / (((CU + GU) + GU) * GS)) * HV) + ((ID * (GN + (CU * GO))) * HW)) + ((ID * (GO + (CU * GN))) * ((GI * AJ) * HV));
            let IF = (-GV) * (((GW - (GY / ((CU * GW) * GS))) * HV) + IE);
            let IN = if HA != 0.0 {
                let IJ = (-IC) - (((HD * IG) / (IH * II)) * HV);
                IJ
            } else {
                let IM = (-IC) + ((N / IK) * (IF + (U * IE)));
                IM
            };
            let IO = HR * (((((((-CE) / (((HS * HO) * HN) * HM)) * HV) + IN) * HP) + HW) - IB);
            let IP = EM - (L * EL);
            let IQ = if (if IP > A { 1.0 } else { 0.0 }) != 0.0 && (if BG > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if IQ != 0.0 {
                let IR = if ((-BF) * (AI / IP)) < -3.5e1f64 { 1.0 } else { 0.0 };
                if IR != 0.0 {
                } else {
                }
            } else {
            }
            let IT = if IS == AI { 1.0 } else { 0.0 };
            if IT != 0.0 {
            } else {
            }
            let JO;
            let JP;
            let JQ;
            let JR;
            let JS;
            if IU != 0.0 {
                let IV = (5.5224904e-23f64 * AE) * HT;
                let IW = ((parameters[42] * IO) * IO) / (((BI * CH) * BH) * C);
                JO = AI;
                JP = IV;
                JQ = AI;
                JR = IW;
                JS = IX;
            } else {
                JO = A;
                JP = A;
                JQ = A;
                JR = A;
                JS = A;
            }
            let IY = if parameters[37] > A { 1.0 } else { 0.0 };
            let IZ = if (if parameters[9] == A { 1.0 } else { 0.0 }) != 0.0 && IY != 0.0 { 1.0 } else { 0.0 };
            if IZ != 0.0 {
            } else {
            }
            let JA = if (if parameters[11] == A { 1.0 } else { 0.0 }) != 0.0 && IY != 0.0 { 1.0 } else { 0.0 };
            if JA != 0.0 {
            } else {
            }
            let JB = if (if parameters[10] == A { 1.0 } else { 0.0 }) != 0.0 && IY != 0.0 { 1.0 } else { 0.0 };
            if JB != 0.0 {
            } else {
            }
            let JC = if (if parameters[12] == A { 1.0 } else { 0.0 }) != 0.0 && IY != 0.0 { 1.0 } else { 0.0 };
            if JC != 0.0 {
            } else {
            }
            let JD = -CO;
            let JE = AF * parameters[43];
            let JF = if ((JD * AU) / JE) < -4e1f64 { 1.0 } else { 0.0 };
            if JF != 0.0 {
            } else {
            }
            let JI = if (((JD + JG) * AU) / JE) > JH { 1.0 } else { 0.0 };
            if JI != 0.0 {
            } else {
            }
            let JJ = -CN;
            let JK = if ((JJ * AU) / JE) < -4e1f64 { 1.0 } else { 0.0 };
            if JK != 0.0 {
            } else {
            }
            let JL = if (((JJ + JG) * AU) / JE) > JH { 1.0 } else { 0.0 };
            if JL != 0.0 {
            } else {
            }
            let JM = if CO > A { 1.0 } else { 0.0 };
            if JM != 0.0 {
            } else {
            }
            let JN = if CN > A { 1.0 } else { 0.0 };
            if JN != 0.0 {
            } else {
            }
        if JO == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JP;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if JQ == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = JR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(JS);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};

use rspice_veriloga_runtime::rspice_limited_exp;
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 8] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS", label: Some("ids"), kind: GeneratedNoiseKind::White, equation: 65, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_FP4_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 66, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(18), name: "fp4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_FP4S_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 67, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(22), name: "fp4s", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 68, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 69, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 70, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 71, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 108, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18]), ctx.node_voltage(self.nodes[19]), ctx.node_voltage(self.nodes[20]), ctx.node_voltage(self.nodes[21]), ctx.node_voltage(self.nodes[22])];
            let A = 0e0f64;
            let B = 1e0f64;
            let C = 1e-2f64;
            let D = 0.0f64;
            let E = parameters[31];
            let F = parameters[32];
            let H = parameters[34];
            let I = parameters[149];
            let M = node_potentials[7];
            let N = node_potentials[8];
            let P = node_potentials[9];
            let S = node_potentials[3];
            let W = -1e0f64;
            let Z = 1e-1f64;
            let AB = node_potentials[0];
            let AC = node_potentials[2];
            let AH = 8.617087e-5f64;
            let AJ = parameters[81];
            let AM = node_potentials[6];
            let AN = node_potentials[5];
            let AO = 5e-1f64;
            let AQ = parameters[128];
            let AW = 2e0f64;
            let AY = node_potentials[1];
            let BD = 3e0f64;
            let BF = parameters[10];
            let BH = 4e0f64;
            let BN = parameters[89];
            let BO = parameters[91];
            let BR = parameters[90];
            let BS = parameters[92];
            let BV = parameters[13];
            let BX = parameters[17];
            let BZ = parameters[95];
            let CA = parameters[36];
            let CC = parameters[96];
            let CD = parameters[37];
            let CO = parameters[9];
            let CP = parameters[1];
            let DN = parameters[23];
            let DX = parameters[3];
            let DY = parameters[4];
            let EA = 1.602176634e-19f64;
            let EB = 3.24e17f64;
            let EG = 1e-4f64;
            let EJ = 2.718281828459045e0f64;
            let ET = parameters[28];
            let EV = 6.666666666666666e-1f64;
            let FB = 2e2f64;
            let FH = 1e-19f64;
            let FP = parameters[29];
            let FU = 3.7e1f64;
            let HU = parameters[14];
            let HV = parameters[15];
            let HW = parameters[16];
            let IA = parameters[18];
            let KM = parameters[5];
            let KN = parameters[25];
            let KV = 8e1f64;
            let KY = parameters[6];
            let LI = parameters[56];
            let LL = parameters[57];
            let LM = parameters[63];
            let LN = parameters[71];
            let LQ = parameters[60];
            let LR = parameters[64];
            let LS = parameters[72];
            let LV = parameters[67];
            let LW = parameters[75];
            let LX = parameters[77];
            let LY = parameters[61];
            let LZ = parameters[79];
            let MC = 1e-3f64;
            let ME = parameters[69];
            let MF = parameters[65];
            let MG = parameters[73];
            let MI = parameters[68];
            let MJ = parameters[76];
            let MK = parameters[78];
            let ML = parameters[62];
            let MM = parameters[80];
            let MP = parameters[70];
            let MQ = parameters[66];
            let MR = parameters[74];
            let NC = parameters[58];
            let OA = parameters[59];
            let SH = 9e-1f64;
            let SJ = parameters[42];
            let TQ = parameters[43];
            let UF = node_potentials[18];
            let UG = node_potentials[22];
            let VA = parameters[150];
            let VC = node_potentials[15];
            let VJ = -1e0f64;
            let VP = parameters[165];
            let VQ = parameters[166];
            let VS = parameters[159];
            let VT = parameters[162];
            let VU = parameters[167];
            let VV = parameters[168];
            let VX = parameters[160];
            let VZ = parameters[161];
            let WA = parameters[158];
            let WP = parameters[169];
            let XI = parameters[170];
            let YV = parameters[163];
            let YW = parameters[164];
            let AFR = parameters[151];
            let AFT = node_potentials[19];
            let AGA = -1e0f64;
            let APT = parameters[152];
            let APV = node_potentials[16];
            let AQD = -1e0f64;
            let AQJ = parameters[178];
            let AQK = parameters[179];
            let AQM = parameters[172];
            let AQN = parameters[175];
            let AQO = parameters[180];
            let AQP = parameters[181];
            let AQR = parameters[173];
            let AQT = parameters[174];
            let AQU = parameters[171];
            let ARJ = parameters[182];
            let ASC = parameters[183];
            let ATP = parameters[176];
            let ATQ = parameters[177];
            let BAL = parameters[153];
            let BAN = node_potentials[20];
            let BAV = -1e0f64;
            let BKQ = parameters[154];
            let BKS = node_potentials[17];
            let BLA = -1e0f64;
            let BLG = parameters[191];
            let BLH = parameters[192];
            let BLJ = parameters[185];
            let BLK = parameters[188];
            let BLL = parameters[193];
            let BLM = parameters[194];
            let BLO = parameters[186];
            let BLQ = parameters[187];
            let BLR = parameters[184];
            let BMG = parameters[195];
            let BMZ = parameters[196];
            let BOM = parameters[189];
            let BON = parameters[190];
            let BVI = parameters[155];
            let BVK = node_potentials[21];
            let BVS = -1e0f64;
            let CFN = parameters[156];
            let CFW = -1e0f64;
            let CGC = parameters[204];
            let CGD = parameters[205];
            let CGF = parameters[198];
            let CGG = parameters[201];
            let CGH = parameters[206];
            let CGI = parameters[207];
            let CGK = parameters[199];
            let CGM = parameters[200];
            let CGN = parameters[197];
            let CHC = parameters[208];
            let CHV = parameters[209];
            let CJI = parameters[202];
            let CJJ = parameters[203];
            let CQE = parameters[157];
            let CQN = -1e0f64;
            let DAI = parameters[255];
            let DAK = parameters[258];
            let DAL = parameters[256];
            let DAM = parameters[257];
            let DAS = node_potentials[10];
            let DAV = parameters[210];
            let DAX = parameters[214];
            let DAY = parameters[213];
            let DAZ = parameters[211];
            let DCX = parameters[261];
            let DCY = 1e-22f64;
            let DDB = parameters[262];
            let DDC = parameters[263];
            let DDH = parameters[264];
            if D != 0.0 {
                let G = if (if E == A { 1.0 } else { 0.0 }) != 0.0 || (if F == A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if G != 0.0 {
                } else {
                }
            } else {
            }
            let J = if I == B { 1.0 } else { 0.0 };
            let RB;
            if J != 0.0 {
                let K = if H == A { 1.0 } else { 0.0 };
                let RC = if K != 0.0 {
                    B
                } else {
                    H
                };
                RB = RC;
            } else {
                RB = H;
            }
            let L = parameters[0] + 2.7315e2f64;
            let O = M - N;
            let Q = P - N;
            let R = P - M;
            let T = S - N;
            let U = S - M;
            let V = if O < A { 1.0 } else { 0.0 };
            let Y;
            let DV;
            let EE;
            let KZ;
            if V != 0.0 {
                let X = W * O;
                Y = X;
                DV = U;
                EE = R;
                KZ = W;
            } else {
                Y = O;
                DV = T;
                EE = Q;
                KZ = B;
            }
            let AA = (((Y * Y) + C).sqrt()) - Z;
            let AD = AB - AC;
            let AE = AD * AD;
            let AF = ((AE + C).sqrt()) - Z;
            let AG = (temperature + node_potentials[4]) + parameters[274];
            let AI = AH * AG;
            let AK = if AJ == A { 1.0 } else { 0.0 };
            let CS;
            let CW;
            let DC;
            let DI;
            let DL;
            let DQ;
            let DT;
            let HD;
            let HI;
            let RF;
            let RL;
            let SM;
            let SP;
            let SV;
            let TT;
            let TX;
            let UA;
            if AK != 0.0 {
                CS = A;
                CW = A;
                DC = A;
                DI = A;
                DL = A;
                DQ = A;
                DT = A;
                HD = A;
                HI = A;
                RF = A;
                RL = A;
                SM = A;
                SP = A;
                SV = A;
                TT = A;
                TX = A;
                UA = A;
            } else {
                let AL = if AJ == B { 1.0 } else { 0.0 };
                let CT;
                let CX;
                let DD;
                let DJ;
                let DM;
                let DR;
                let DU;
                let HE;
                let HJ;
                let RG;
                let RM;
                let SN;
                let SQ;
                let SW;
                let TU;
                let TY;
                let UB;
                if AL != 0.0 {
                    let AP = AN - AI;
                    let AR = AO * ((AN + AI) + (((AP * AP) + ((2.5e-1f64 * AQ) * AQ)).sqrt()));
                    let AS = parameters[100] + (parameters[101] * (rspice_limited_exp((-1e0f64 / AR))));
                    let AT = parameters[104] + (parameters[105] * (rspice_limited_exp((-1e0f64 / AR))));
                    let AU = parameters[106] + (parameters[107] * (rspice_limited_exp((-1e0f64 / AR))));
                    let AV = parameters[102] + (parameters[103] * (rspice_limited_exp((-1e0f64 / AR))));
                    CT = A;
                    CX = A;
                    DD = A;
                    DJ = A;
                    DM = AV;
                    DR = A;
                    DU = AS;
                    HE = A;
                    HJ = A;
                    RG = A;
                    RM = A;
                    SN = AT;
                    SQ = A;
                    SW = A;
                    TU = A;
                    TY = A;
                    UB = AU;
                } else {
                    let AX = if AJ == AW { 1.0 } else { 0.0 };
                    let CU;
                    let CY;
                    let DE;
                    let DK;
                    let DS;
                    let HF;
                    let HK;
                    let RH;
                    let RN;
                    let SR;
                    let SX;
                    let TV;
                    let TZ;
                    if AX != 0.0 {
                        let AZ = parameters[113] * AM;
                        let BA = (((-parameters[116]) * AN) + (parameters[117] * AM)) + parameters[118];
                        let BB = parameters[114] * AM;
                        let BC = parameters[115] * AM;
                        CU = BB;
                        CY = A;
                        DE = A;
                        DK = BC;
                        DS = AZ;
                        HF = A;
                        HK = A;
                        RH = A;
                        RN = A;
                        SR = A;
                        SX = A;
                        TV = A;
                        TZ = BA;
                    } else {
                        let BE = if AJ == BD { 1.0 } else { 0.0 };
                        let CZ;
                        let DF;
                        let HG;
                        let HL;
                        let RI;
                        let RO;
                        let SS;
                        let SY;
                        let TW;
                        if BE != 0.0 {
                            let BG = (AN / parameters[121]) * ((AG / L).powf(parameters[126]));
                            CZ = A;
                            DF = A;
                            HG = A;
                            HL = A;
                            RI = A;
                            RO = A;
                            SS = A;
                            SY = A;
                            TW = BG;
                        } else {
                            let BI = if AJ == BH { 1.0 } else { 0.0 };
                            let DA;
                            let DG;
                            let HH;
                            let HM;
                            let RJ;
                            let RP;
                            let ST;
                            let SZ;
                            if BI != 0.0 {
                                let BJ = node_potentials[12] - (AD.abs());
                                let BK = AO * (BJ + (((BJ * BJ) + 2.5000000000000003e-61f64).sqrt()));
                                let BL = node_potentials[14] - ((AY - AC).abs());
                                let BM = AO * (BL + (((BL * BL) + 2.5000000000000003e-61f64).sqrt()));
                                let BP = (BK * BN) / (((BK * BK) + (BN * BN)).sqrt());
                                let BQ = ((BO * BF).abs()) * BP;
                                let BT = (BM * BR) / (((BM * BM) + (BR * BR)).sqrt());
                                let BU = ((BS * BF).abs()) * BT;
                                let BW = ((parameters[93] * BV).abs()) * BT;
                                let BY = ((parameters[94] * BX).abs()) * BT;
                                let CB = ((BZ * CA).abs()) * BP;
                                let CE = ((CC * CD).abs()) * BP;
                                DA = BQ;
                                DG = BU;
                                HH = BW;
                                HM = BY;
                                RJ = CB;
                                RP = A;
                                ST = CE;
                                SZ = A;
                            } else {
                                let CF = if AJ == 5e0f64 { 1.0 } else { 0.0 };
                                let DB;
                                let DH;
                                let RK;
                                let RQ;
                                let SU;
                                let TA;
                                if CF != 0.0 {
                                    let CG = (AN * BN) / (((AN * AN) + (BN * BN)).sqrt());
                                    let CH = ((BO * BF).abs()) * CG;
                                    let CI = ((BZ * CA).abs()) * CG;
                                    let CJ = ((CC * CD).abs()) * CG;
                                    let CK = (AM * BR) / (((AM * AM) + (BR * BR)).sqrt());
                                    let CL = ((BS * BF).abs()) * CK;
                                    let CM = ((parameters[147] * CA).abs()) * CK;
                                    let CN = ((parameters[148] * CD).abs()) * CK;
                                    DB = CH;
                                    DH = CL;
                                    RK = CI;
                                    RQ = CM;
                                    SU = CJ;
                                    TA = CN;
                                } else {
                                    DB = A;
                                    DH = A;
                                    RK = A;
                                    RQ = A;
                                    SU = A;
                                    TA = A;
                                }
                                DA = DB;
                                DG = DH;
                                HH = A;
                                HM = A;
                                RJ = RK;
                                RP = RQ;
                                ST = SU;
                                SZ = TA;
                            }
                            CZ = DA;
                            DF = DG;
                            HG = HH;
                            HL = HM;
                            RI = RJ;
                            RO = RP;
                            SS = ST;
                            SY = SZ;
                            TW = A;
                        }
                        CU = A;
                        CY = CZ;
                        DE = DF;
                        DK = A;
                        DS = A;
                        HF = HG;
                        HK = HL;
                        RH = RI;
                        RN = RO;
                        SR = SS;
                        SX = SY;
                        TV = TW;
                        TZ = A;
                    }
                    CT = CU;
                    CX = CY;
                    DD = DE;
                    DJ = DK;
                    DM = A;
                    DR = DS;
                    DU = A;
                    HE = HF;
                    HJ = HK;
                    RG = RH;
                    RM = RN;
                    SN = A;
                    SQ = SR;
                    SW = SX;
                    TU = TV;
                    TY = TZ;
                    UB = A;
                }
                CS = CT;
                CW = CX;
                DC = DD;
                DI = DJ;
                DL = DM;
                DQ = DR;
                DT = DU;
                HD = HE;
                HI = HJ;
                RF = RG;
                RL = RM;
                SM = SN;
                SP = SQ;
                SV = SW;
                TT = TU;
                TX = TY;
                UA = UB;
            }
            let CQ = CO / CP;
            let CR = CO / parameters[2];
            let CV = AI * ((B + parameters[26]) + ((parameters[27] + CS) * AA));
            let DO = AG / L;
            let DP = DO - B;
            let DW = ((((((BF + CW) + DC) - ((((parameters[22] + DI) - DL) * (AA * DN)) / (((AA * AA) + (DN * DN)).sqrt()))) - (DP * parameters[24])) + DQ) + DT) + (((CR / (CR + CQ)) * parameters[11]) * DV);
            let DZ = AW * DY;
            let EC = (DZ * EA) * EB;
            let ED = DW + (CV * (((DX / ((EC * CV) * CV)) * parameters[30]).ln()));
            let EF = EE - ED;
            let EH = ((AO * (EF + (((EF * EF) + EG).sqrt()))) + ED) - DW;
            let EI = CQ / (5.19105229416e-2f64 * CV);
            let EK = EJ / EI;
            let EL = B / EI;
            let EM = CQ / EA;
            let EN = AO * EH;
            let EO = EH * EH;
            let EP = EN + (AO * ((EO + 3.6e-1f64).sqrt()));
            let EQ = EP * EP;
            let ER = EK * EK;
            let ES = EL * EL;
            let EU = ET / BD;
            let EW = (EM * EP).powf(EV);
            let EX = (AW * ET) / BD;
            let EY = ((EP + (CV * (B - ((EI * ((EP * EK) / ((EQ + ER).sqrt()))).ln())))) - (EU * EW)) / ((EP * (B + (CV / ((EP * EL) / ((EQ + ES).sqrt()))))) + (EX * EW));
            let EZ = AW * CV;
            let FA = EH / EZ;
            let FC = if FA < FB { 1.0 } else { 0.0 };
            let FF = if FC != 0.0 {
                let FD = ((EZ * EM) * (((BD * FA) / BH) + (((rspice_limited_exp((FA / BH))) + (rspice_limited_exp(((-3e0f64 * FA) / BH)))).ln()))) / ((B / EY) + ((EM / EB) * (rspice_limited_exp(((-1e0f64 * EH) / EZ)))));
                FD
            } else {
                let FE = ((EZ * EM) * FA) / ((B / EY) + ((EM / EB) * (rspice_limited_exp(((-1e0f64 * EH) / EZ)))));
                FE
            };
            let FG = EH - (FF / EM);
            let FI = if ((FG - EH).abs()) > FH { 1.0 } else { 0.0 };
            let HR;
            if FI != 0.0 {
                let FJ = EH - FG;
                let FK = (AO * FJ) + (AO * (((FJ * FJ) + 4e-18f64).sqrt()));
                let FL = EM.powf(EV);
                let FM = FK.powf(EV);
                let FN = FK.powf(-3.333333333333333e-1f64);
                let FO = ET * FL;
                let FQ = FP * FL;
                let FR = FG / CV;
                let FS = FR - ((FO * FM) / CV);
                let FT = FR - ((FQ * FM) / CV);
                let FV = if FS >= FU { 1.0 } else { 0.0 };
                let GC;
                if FV != 0.0 {
                    GC = FS;
                } else {
                    let FW = if FS <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let GD = if FW != 0.0 {
                        A
                    } else {
                        let FX = ((FS.exp()) + B).ln();
                        FX
                    };
                    GC = GD;
                }
                let FY = if FT >= FU { 1.0 } else { 0.0 };
                let GE;
                if FY != 0.0 {
                    GE = FT;
                } else {
                    let FZ = if FT <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let GF = if FZ != 0.0 {
                        A
                    } else {
                        let GA = ((FT.exp()) + B).ln();
                        GA
                    };
                    GE = GF;
                }
                let GB = EB * CV;
                let GG = rspice_limited_exp(FS);
                let GH = rspice_limited_exp(FT);
                let GI = FG - ((((EM * FK) - (GB * GC)) - (GB * GE)) / (((-1e0f64 * EM) - (((GG * EB) * (B + (EV * (FO * FN)))) / (B + GG))) - (((GH * EB) * (B + (EV * (FQ * FN)))) / (B + GH))));
                let GJ = EH - GI;
                let GK = (AO * GJ) + (AO * (((GJ * GJ) + 4e-18f64).sqrt()));
                let GL = GK.powf(-3.333333333333333e-1f64);
                let GM = GK.powf(EV);
                let GN = GI / CV;
                let GO = GN - ((FO * GM) / CV);
                let GP = GN - ((FQ * GM) / CV);
                let GQ = if GO >= FU { 1.0 } else { 0.0 };
                let GW;
                if GQ != 0.0 {
                    GW = GO;
                } else {
                    let GR = if GO <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let GX = if GR != 0.0 {
                        A
                    } else {
                        let GS = ((GO.exp()) + B).ln();
                        GS
                    };
                    GW = GX;
                }
                let GT = if GP >= FU { 1.0 } else { 0.0 };
                let GY;
                if GT != 0.0 {
                    GY = GP;
                } else {
                    let GU = if GP <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let GZ = if GU != 0.0 {
                        A
                    } else {
                        let GV = ((GP.exp()) + B).ln();
                        GV
                    };
                    GY = GZ;
                }
                let HA = rspice_limited_exp(GO);
                let HB = rspice_limited_exp(GP);
                let HC = GI - ((((EM * GK) - (GB * GW)) - (GB * GY)) / (((-1e0f64 * EM) - (((HA * EB) * (B + (EV * (FO * GL)))) / (B + HA))) - (((HB * EB) * (B + (EV * (FQ * GL)))) / (B + HB))));
                HR = HC;
            } else {
                HR = FG;
            }
            let HN = DO.powf(parameters[20]);
            let HO = (BV - HD) * HN;
            let HP = DO.powf(parameters[19]);
            let HQ = CQ / CO;
            let HS = HQ * ((EH - HR).abs());
            let HT = CR / CO;
            let HX = HW * (HT * ((DV - HR).abs()));
            let HY = EN + (AO * ((EO + 3.6e-1f64).sqrt()));
            let HZ = ((AW * ((BX - HI) * HP)) / (HO / (((B + (HU * HS)) + (HV * (HS * HS))) + HX))) * DX;
            let IB = Y * ((B + ((Y / ((HZ * HY) / (HZ + HY))).powf(IA))).powf((-1e0f64 / IA)));
            let IC = EH - IB;
            let ID = (AO * IC) + (AO * (((IC * IC) + 3.6e-1f64).sqrt()));
            let IE = ID * ID;
            let IF = (EM * ID).powf(EV);
            let IG = ((ID + (CV * (B - ((EI * ((ID * EK) / ((IE + ER).sqrt()))).ln())))) - (EU * IF)) / ((ID * (B + (CV / ((ID * EL) / ((IE + ES).sqrt()))))) + (EX * IF));
            let IH = IC / EZ;
            let II = if IH < FB { 1.0 } else { 0.0 };
            let IL = if II != 0.0 {
                let IJ = ((EZ * EM) * (((BD * IH) / BH) + (((rspice_limited_exp((IH / BH))) + (rspice_limited_exp(((-3e0f64 * IH) / BH)))).ln()))) / ((B / IG) + ((EM / EB) * (rspice_limited_exp(((-1e0f64 * IC) / EZ)))));
                IJ
            } else {
                let IK = ((EZ * EM) * IH) / ((B / IG) + ((EM / EB) * (rspice_limited_exp(((-1e0f64 * IC) / EZ)))));
                IK
            };
            let IM = IC - (IL / EM);
            let IN = if ((IM - IC).abs()) > FH { 1.0 } else { 0.0 };
            let KG;
            if IN != 0.0 {
                let IO = IC - IM;
                let IP = (AO * IO) + (AO * (((IO * IO) + 4e-18f64).sqrt()));
                let IQ = EM.powf(EV);
                let IR = IP.powf(EV);
                let IS = IP.powf(-3.333333333333333e-1f64);
                let IT = ET * IQ;
                let IU = FP * IQ;
                let IV = IM / CV;
                let IW = IV - ((IT * IR) / CV);
                let IX = IV - ((IU * IR) / CV);
                let IY = if IW >= FU { 1.0 } else { 0.0 };
                let JF;
                if IY != 0.0 {
                    JF = IW;
                } else {
                    let IZ = if IW <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let JG = if IZ != 0.0 {
                        A
                    } else {
                        let JA = ((IW.exp()) + B).ln();
                        JA
                    };
                    JF = JG;
                }
                let JB = if IX >= FU { 1.0 } else { 0.0 };
                let JH;
                if JB != 0.0 {
                    JH = IX;
                } else {
                    let JC = if IX <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let JI = if JC != 0.0 {
                        A
                    } else {
                        let JD = ((IX.exp()) + B).ln();
                        JD
                    };
                    JH = JI;
                }
                let JE = EB * CV;
                let JJ = rspice_limited_exp(IW);
                let JK = rspice_limited_exp(IX);
                let JL = IM - ((((EM * IP) - (JE * JF)) - (JE * JH)) / (((-1e0f64 * EM) - (((JJ * EB) * (B + (EV * (IT * IS)))) / (B + JJ))) - (((JK * EB) * (B + (EV * (IU * IS)))) / (B + JK))));
                let JM = IC - JL;
                let JN = (AO * JM) + (AO * (((JM * JM) + 4e-18f64).sqrt()));
                let JO = JN.powf(EV);
                let JP = JL / CV;
                let JQ = JP - ((IT * JO) / CV);
                let JR = JP - ((IU * JO) / CV);
                let JS = if JQ >= FU { 1.0 } else { 0.0 };
                let JY;
                if JS != 0.0 {
                    JY = JQ;
                } else {
                    let JT = if JQ <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let JZ = if JT != 0.0 {
                        A
                    } else {
                        let JU = ((JQ.exp()) + B).ln();
                        JU
                    };
                    JY = JZ;
                }
                let JV = if JR >= FU { 1.0 } else { 0.0 };
                let KA;
                if JV != 0.0 {
                    KA = JR;
                } else {
                    let JW = if JR <= -3.7e1f64 { 1.0 } else { 0.0 };
                    let KB = if JW != 0.0 {
                        A
                    } else {
                        let JX = ((JR.exp()) + B).ln();
                        JX
                    };
                    KA = KB;
                }
                let KC = rspice_limited_exp(JQ);
                let KD = rspice_limited_exp(JR);
                let KE = (JL - ((((EM * JN) - (JE * JY)) - (JE * KA)) / (((-1e0f64 * EM) - (((KC * EB) * (B + (EV * (IT * (JN.powf(-3.333333333333333e-1f64)))))) / (B + KC))) - (((KD * EB) * (B + (EV * (IU * (JN.powf(-3.333333333333333e-1f64)))))) / (B + KD))))) + IB;
                KG = KE;
            } else {
                let KF = IM + IB;
                KG = KF;
            }
            let KH = AO * (HR + KG);
            let KI = KG - HR;
            let KJ = EH - KH;
            let KK = HQ * (KJ.abs());
            let KL = HO / (((B + (HU * KK)) + ((HV * KK) * KK)) + HX);
            let KO = (B + (((KN * KN) * KI) * KI)).sqrt();
            let KP = ((((((KL * CQ) * DY) * KM) / DX) * (B + (parameters[21] * (AA - IB)))) / KO) * ((KJ + CV) * KI);
            let KQ = parameters[270] * (B + (parameters[272] * DP));
            let KR = parameters[268] * (B + (parameters[273] * DP));
            let KS = if (parameters[269] * (B + (parameters[271] * DP))) > A { 1.0 } else { 0.0 };
            if KS != 0.0 {
                let KT = AF - KR;
                let KU = if KT > A { 1.0 } else { 0.0 };
                if KU != 0.0 {
                    let KW = if (KT / (KQ * AI)) > KV { 1.0 } else { 0.0 };
                    if KW != 0.0 {
                    } else {
                    }
                } else {
                    let KX = if (KT / (KQ * AI)) > KV { 1.0 } else { 0.0 };
                    if KX != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let LA = DY * KM;
            let LB = (EH + CV) - KH;
            let LC = KJ + (((AO * KI) * KI) / (6e0f64 * LB));
            let LD = (CO / (CP + (parameters[231] / (B + ((1e26f64 * (((((CQ * DY) * KM) * DX) * LC) / parameters[233])).powf(parameters[232])))))) * DY;
            let LE = KI * KI;
            let LF = (-(((LD * DX) * KM) * AO)) * (((EH - ((HR + (AW * KG)) / BD)) + ((8.333333333333333e-2f64 * LE) / LB)) + ((8.333333333333333e-3f64 * (LE * KI)) / (LB * LB)));
            let LG = (-1e0f64 * (((LD * KM) * DX) * LC)) - LF;
            let LH = if KZ < A { 1.0 } else { 0.0 };
            let DBG;
            let DBI;
            if LH != 0.0 {
                DBG = LG;
                DBI = LF;
            } else {
                DBG = LF;
                DBI = LG;
            }
            let LJ = if LI == A { 1.0 } else { 0.0 };
            let QN;
            let QU;
            if LJ != 0.0 {
                QN = A;
                QU = A;
            } else {
                let LK = if LI == B { 1.0 } else { 0.0 };
                let QO;
                let QV;
                if LK != 0.0 {
                    let LO = (DY * DX) * KM;
                    let LP = (LO * ((LM + (DP * LN)).abs())) * ((rspice_limited_exp((Q / ((LL * AH) * AG)))) - B);
                    let LT = (LO * ((LR + (DP * LS)).abs())) * ((rspice_limited_exp((R / ((LQ * AH) * AG)))) - B);
                    QO = LP;
                    QV = LT;
                } else {
                    let LU = if LI == AW { 1.0 } else { 0.0 };
                    let QP;
                    let QW;
                    if LU != 0.0 {
                        let MA = (DY * DX) * KM;
                        let MB = -Q;
                        let MD = MB - (AO * (MB - (((MB * MB) + MC).sqrt())));
                        let MH = ((MA * ((LM * ((LN * DP).exp())).abs())) * ((rspice_limited_exp(((Q - (LV + (DP * LW))) / (((LL + (DP * LX)) * AH) * L)))) - B)) * (B + (((MD / CP) * (MF * ((MG * DP).exp()))) * (rspice_limited_exp((((MD.sqrt()) + ME) / (((LY + (DP * LZ)) * AH) * L))))));
                        let MN = -R;
                        let MO = MN - (AO * (MN - (((MN * MN) + MC).sqrt())));
                        let MS = ((MA * ((LR * ((LS * DP).exp())).abs())) * ((rspice_limited_exp(((R - (MI + (DP * MJ))) / (((LQ + (DP * MK)) * AH) * L)))) - B)) * (B + (((MO / CP) * (MQ * ((MR * DP).exp()))) * (rspice_limited_exp((((MO.sqrt()) + MP) / (((ML + (DP * MM)) * AH) * L))))));
                        QP = MH;
                        QW = MS;
                    } else {
                        let MT = if LI == BD { 1.0 } else { 0.0 };
                        let QQ;
                        let QX;
                        if MT != 0.0 {
                            let MU = LV + (DP * LW);
                            let MV = LL + (DP * LX);
                            let MW = LY + (DP * LZ);
                            let MX = MF * ((MG * DP).exp());
                            let MY = (DY * DX) * KM;
                            let MZ = (MY * LM) * ((LN * DP).exp());
                            let NA = if MZ > A { 1.0 } else { 0.0 };
                            let QR;
                            if NA != 0.0 {
                                let NB = if Q > A { 1.0 } else { 0.0 };
                                let NF = if NB != 0.0 {
                                    let ND = (Q.powf(NC)) / (MV * AI);
                                    ND
                                } else {
                                    let NE = Q / (MV * AI);
                                    NE
                                };
                                let NG = if NF > KV { 1.0 } else { 0.0 };
                                let NI;
                                let NJ;
                                if NG != 0.0 {
                                    let NH = B + (NF - KV);
                                    NI = NH;
                                    NJ = KV;
                                } else {
                                    NI = B;
                                    NJ = NF;
                                }
                                let NK = (MZ * ((NI * (NJ.exp())) - B)) * (((-MU) / (MV * AI)).exp());
                                let NL = -Q;
                                let NM = NL - (AO * (NL - (((NL * NL) + MC).sqrt())));
                                let NN = ((NM.sqrt()) + ME) / (MW * AI);
                                let NO = if NN > KV { 1.0 } else { 0.0 };
                                let NQ;
                                let NR;
                                if NO != 0.0 {
                                    let NP = B + (NN - KV);
                                    NQ = NP;
                                    NR = KV;
                                } else {
                                    NQ = B;
                                    NR = NN;
                                }
                                let NS = NK * (B + (((NM * MX) * NQ) * (NR.exp())));
                                QR = NS;
                            } else {
                                QR = A;
                            }
                            let NT = MI + (DP * MJ);
                            let NU = LQ + (DP * MK);
                            let NV = ML + (DP * MM);
                            let NW = MQ * ((MR * DP).exp());
                            let NX = (MY * LR) * ((LS * DP).exp());
                            let NY = if NX > A { 1.0 } else { 0.0 };
                            let QY;
                            if NY != 0.0 {
                                let NZ = if R > A { 1.0 } else { 0.0 };
                                let OD = if NZ != 0.0 {
                                    let OB = (R.powf(OA)) / (NU * AI);
                                    OB
                                } else {
                                    let OC = R / (NU * AI);
                                    OC
                                };
                                let OE = if OD > KV { 1.0 } else { 0.0 };
                                let OG;
                                let OH;
                                if OE != 0.0 {
                                    let OF = B + (OD - KV);
                                    OG = OF;
                                    OH = KV;
                                } else {
                                    OG = B;
                                    OH = OD;
                                }
                                let OI = (NX * ((OG * (OH.exp())) - B)) * (((-NT) / (NU * AI)).exp());
                                let OJ = -R;
                                let OK = OJ - (AO * (OJ - (((OJ * OJ) + MC).sqrt())));
                                let OL = ((OK.sqrt()) + MP) / (NV * AI);
                                let OM = if OL > KV { 1.0 } else { 0.0 };
                                let OO;
                                let OP;
                                if OM != 0.0 {
                                    let ON = B + (OL - KV);
                                    OO = ON;
                                    OP = KV;
                                } else {
                                    OO = B;
                                    OP = OL;
                                }
                                let OQ = OI * (B + (((OK * NW) * OO) * (OP.exp())));
                                QY = OQ;
                            } else {
                                QY = A;
                            }
                            QQ = QR;
                            QX = QY;
                        } else {
                            let OR = if LI == BH { 1.0 } else { 0.0 };
                            let QS;
                            let QZ;
                            if OR != 0.0 {
                                let OS = LV + (DP * LW);
                                let OT = LL + (DP * LX);
                                let OU = LY + (DP * LZ);
                                let OV = (DY * DX) * KM;
                                let OW = (OV * MF) * ((MG * DP).exp());
                                let OX = (OV * LM) * ((LN * DP).exp());
                                let OY = if OX > A { 1.0 } else { 0.0 };
                                let QT;
                                if OY != 0.0 {
                                    let OZ = if Q > A { 1.0 } else { 0.0 };
                                    let PC = if OZ != 0.0 {
                                        let PA = (Q.powf(NC)) / (OT * AI);
                                        PA
                                    } else {
                                        let PB = Q / (OT * AI);
                                        PB
                                    };
                                    let PD = if PC > KV { 1.0 } else { 0.0 };
                                    let PF;
                                    let PG;
                                    if PD != 0.0 {
                                        let PE = B + (PC - KV);
                                        PF = PE;
                                        PG = KV;
                                    } else {
                                        PF = B;
                                        PG = PC;
                                    }
                                    let PH = (OX * ((PF * (PG.exp())) - B)) * (((-OS) / (OT * AI)).exp());
                                    let PI = -Q;
                                    let PJ = OU * AI;
                                    let PK = (((PI - (AO * (PI - ((PI * PI).sqrt())))).sqrt()) + ME) / PJ;
                                    let PL = if PK > KV { 1.0 } else { 0.0 };
                                    let PN;
                                    let PO;
                                    if PL != 0.0 {
                                        let PM = B + (PK - KV);
                                        PN = PM;
                                        PO = KV;
                                    } else {
                                        PN = B;
                                        PO = PK;
                                    }
                                    let PP = PH - (OW * ((PN * (PO.exp())) - ((ME / PJ).exp())));
                                    QT = PP;
                                } else {
                                    QT = A;
                                }
                                let PQ = MI + (DP * MJ);
                                let PR = LQ + (DP * MK);
                                let PS = ML + (DP * MM);
                                let PT = (OV * MQ) * ((MR * DP).exp());
                                let PU = (OV * LR) * ((LS * DP).exp());
                                let PV = if PU > A { 1.0 } else { 0.0 };
                                let RA;
                                if PV != 0.0 {
                                    let PW = if R > A { 1.0 } else { 0.0 };
                                    let PZ = if PW != 0.0 {
                                        let PX = (R.powf(OA)) / (PR * AI);
                                        PX
                                    } else {
                                        let PY = R / (PR * AI);
                                        PY
                                    };
                                    let QA = if PZ > KV { 1.0 } else { 0.0 };
                                    let QC;
                                    let QD;
                                    if QA != 0.0 {
                                        let QB = B + (PZ - KV);
                                        QC = QB;
                                        QD = KV;
                                    } else {
                                        QC = B;
                                        QD = PZ;
                                    }
                                    let QE = (PU * ((QC * (QD.exp())) - B)) * (((-PQ) / (PR * AI)).exp());
                                    let QF = -R;
                                    let QG = PS * AI;
                                    let QH = (((QF - (AO * (QF - ((QF * QF).sqrt())))).sqrt()) + MP) / QG;
                                    let QI = if QH > KV { 1.0 } else { 0.0 };
                                    let QK;
                                    let QL;
                                    if QI != 0.0 {
                                        let QJ = B + (QH - KV);
                                        QK = QJ;
                                        QL = KV;
                                    } else {
                                        QK = B;
                                        QL = QH;
                                    }
                                    let QM = QE - (PT * ((QK * (QL.exp())) - ((MP / QG).exp())));
                                    RA = QM;
                                } else {
                                    RA = A;
                                }
                                QS = QT;
                                QZ = RA;
                            } else {
                                QS = A;
                                QZ = A;
                            }
                            QQ = QS;
                            QX = QZ;
                        }
                        QP = QQ;
                        QW = QX;
                    }
                    QO = QP;
                    QV = QW;
                }
                QN = QO;
                QU = QV;
            }
            if LJ != 0.0 {
            } else {
            }
            let RD = if RB == B { 1.0 } else { 0.0 };
            let UP;
            let UR;
            if RD != 0.0 {
                let RE = B - (parameters[50] * DP);
                let RR = ((parameters[12] / EA) * DV) * CR;
                let RS = (((CA * RE) - RF) - RL) + RR;
                let RT = B + RS;
                let RU = RS - B;
                let RV = parameters[35] * (DO.powf(parameters[51]));
                let RW = LA * ((EA * (RT - (AO * (RT - (((RU * RU) + MC).sqrt()))))) * (B + (parameters[38] * ID)));
                let RX = RW * RV;
                let RY = parameters[46] / (RW * (parameters[40] * (DO.powf(parameters[52]))));
                let RZ = if (if parameter_given[45] { 1.0 } else { 0.0 }) != A { 1.0 } else { 0.0 };
                let SL = if RZ != 0.0 {
                    let SA = B + parameters[45];
                    let SB = (SA.sqrt()) * KP;
                    let SC = SB / RX;
                    let SD = SC * AW;
                    let SE = SA + (SC * SC);
                    let SF = B - (((SB * AW) / (((SE - SD).sqrt()) + ((SE + SD).sqrt()))) / RX);
                    SF
                } else {
                    let SG = (KP / RX).abs();
                    let SI = SG - SH;
                    let SK = (B - ((AO * (((SG + SH) - (((SI * SI) + 1.0000000000000002e-2f64).sqrt())) - -5.538513813741708e-3f64)).powf(SJ))).powf((B / SJ));
                    SK
                };
                let SO = (((parameters[48] * (B + (parameters[54] * DP))) / LA) + (RY / SL)) + SM;
                let TB = (((CD * RE) - SP) - SV) + RR;
                let TC = B + TB;
                let TD = TB - B;
                let TE = LA * ((EA * (TC - (AO * (TC - (((TD * TD) + MC).sqrt()))))) * (B + (parameters[39] * ID)));
                let TF = TE * RV;
                let TG = parameters[47] / (TE * (parameters[41] * (DO.powf(parameters[53]))));
                let TH = if (if parameter_given[44] { 1.0 } else { 0.0 }) != A { 1.0 } else { 0.0 };
                let TS = if TH != 0.0 {
                    let TI = B + parameters[44];
                    let TJ = (TI.sqrt()) * KP;
                    let TK = TJ / TF;
                    let TL = TK * AW;
                    let TM = TI + (TK * TK);
                    let TN = B - (((TJ * AW) / (((TM - TL).sqrt()) + ((TM + TL).sqrt()))) / TF);
                    TN
                } else {
                    let TO = (KP / TF).abs();
                    let TP = TO - SH;
                    let TR = (B - ((AO * (((TO + SH) - (((TP * TP) + 1.0000000000000002e-2f64).sqrt())) - -5.538513813741708e-3f64)).powf(TQ))).powf((B / TQ));
                    TR
                };
                let UC = B / ((((((parameters[49] * (B + (parameters[55] * DP))) / LA) + (TG / TS)) + TT) + TX) + UA);
                let UD = B / SO;
                let UE = if I == A { 1.0 } else { 0.0 };
                if UE != 0.0 {
                } else {
                }
                UP = UC;
                UR = UD;
            } else {
                let UH = if I == A { 1.0 } else { 0.0 };
                if UH != 0.0 {
                } else {
                }
                UP = A;
                UR = A;
            }
            let UI = if parameters[260] == B { 1.0 } else { 0.0 };
            let DEQ;
            let DER;
            let DES;
            let DEV;
            let DEY;
            let DFB;
            let DFE;
            let DFH;
            let DFK;
            let DFN;
            if UI != 0.0 {
                let UJ = KL / KO;
                let UK = KG * KG;
                let UL = HR * HR;
                let UM = ((((parameters[265] / (((if KP >= 1e-10f64 { KP } else { 1e-10f64 }) * DX) * DX)) * (((((((((5.522438177818063e-23f64 * AG) * EA) * DY) * KM) * CQ) * EA) * DY) * KM) * CQ)) * (UJ * UJ)) * (((EO * KI) + (((UK * KG) - (UL * HR)) / BD)) - (EH * (UK - UL)))) * KY;
                let DET;
                let DEW;
                let DEZ;
                let DFC;
                let DFF;
                let DFI;
                let DFL;
                let DFO;
                if RD != 0.0 {
                    let UN = if I == A { 1.0 } else { 0.0 };
                    let DEU;
                    let DEX;
                    let DFA;
                    let DFD;
                    let DFG;
                    let DFJ;
                    let DFM;
                    let DFP;
                    if UN != 0.0 {
                        let UO = (BH * AI) * EA;
                        let UQ = (UO * UP) * KY;
                        let US = (UO * UR) * KY;
                        DEU = B;
                        DEX = UQ;
                        DFA = B;
                        DFD = US;
                        DFG = A;
                        DFJ = A;
                        DFM = A;
                        DFP = A;
                    } else {
                        let UT = (BH * AI) * EA;
                        let UU = (UT * UP) * KY;
                        let UV = (UT * UR) * KY;
                        DEU = A;
                        DEX = A;
                        DFA = A;
                        DFD = A;
                        DFG = B;
                        DFJ = UU;
                        DFM = B;
                        DFP = UV;
                    }
                    DET = DEU;
                    DEW = DEX;
                    DEZ = DFA;
                    DFC = DFD;
                    DFF = DFG;
                    DFI = DFJ;
                    DFL = DFM;
                    DFO = DFP;
                } else {
                    DET = A;
                    DEW = A;
                    DEZ = A;
                    DFC = A;
                    DFF = A;
                    DFI = A;
                    DFL = A;
                    DFO = A;
                }
                DEQ = B;
                DER = UM;
                DES = DET;
                DEV = DEW;
                DEY = DEZ;
                DFB = DFC;
                DFE = DFF;
                DFH = DFI;
                DFK = DFL;
                DFN = DFO;
            } else {
                DEQ = A;
                DER = A;
                DES = A;
                DEV = A;
                DEY = A;
                DFB = A;
                DFE = A;
                DFH = A;
                DFK = A;
                DFN = A;
            }
            let UW = if LI != A { 1.0 } else { 0.0 };
            let DFQ;
            let DFR;
            let DFS;
            let DFT;
            if UW != 0.0 {
                let UX = (3.204353268e-19f64 * (QN.abs())) * KY;
                let UY = (3.204353268e-19f64 * (QU.abs())) * KY;
                DFQ = B;
                DFR = UX;
                DFS = B;
                DFT = UY;
            } else {
                DFQ = A;
                DFR = A;
                DFS = A;
                DFT = A;
            }
            let UZ = if I == A { 1.0 } else { 0.0 };
            let DCN;
            if UZ != 0.0 {
                let VB = if VA != A { 1.0 } else { 0.0 };
                let DCO;
                if VB != 0.0 {
                    let VD = VC - M;
                    let VE = if VA == B { 1.0 } else { 0.0 };
                    let VL;
                    let VM;
                    if VE != 0.0 {
                        let VF = P - VC;
                        VL = VF;
                        VM = R;
                    } else {
                        let VG = AC - M;
                        let VH = AC - VC;
                        VL = VH;
                        VM = VG;
                    }
                    let VI = if VD < A { 1.0 } else { 0.0 };
                    let VN;
                    let WC;
                    let AAU;
                    if VI != 0.0 {
                        let VK = VJ * VD;
                        VN = VK;
                        WC = VL;
                        AAU = VJ;
                    } else {
                        VN = VD;
                        WC = VM;
                        AAU = B;
                    }
                    let VO = (((VN * VN) + C).sqrt()) - Z;
                    let VR = AI * ((B + VP) + (VQ * VO));
                    let VW = (VS + (DP * VT)) - ((VU * (VO * VV)) / (((VO * VO) + (VV * VV)).sqrt()));
                    let VY = CO / VX;
                    let WB = VW + (VR * (((VZ / ((EC * VR) * VR)) * WA).ln()));
                    let WD = WC - WB;
                    let WE = ((AO * (WD + (((WD * WD) + EG).sqrt()))) + WB) - VW;
                    let WF = VY / (5.19105229416e-2f64 * VR);
                    let WG = EJ / WF;
                    let WH = B / WF;
                    let WI = VY / EA;
                    let WJ = AO * WE;
                    let WK = WE * WE;
                    let WL = WJ + (AO * ((WK + 3.6e-1f64).sqrt()));
                    let WM = WL * WL;
                    let WN = WG * WG;
                    let WO = WH * WH;
                    let WQ = WP / BD;
                    let WR = (WI * WL).powf(EV);
                    let WS = (AW * WP) / BD;
                    let WT = ((WL + (VR * (B - ((WF * ((WL * WG) / ((WM + WN).sqrt()))).ln())))) - (WQ * WR)) / ((WL * (B + (VR / ((WL * WH) / ((WM + WO).sqrt()))))) + (WS * WR));
                    let WU = AW * VR;
                    let WV = WE / WU;
                    let WW = if WV < FB { 1.0 } else { 0.0 };
                    let WZ = if WW != 0.0 {
                        let WX = ((WU * WI) * (((BD * WV) / BH) + (((rspice_limited_exp((WV / BH))) + (rspice_limited_exp(((-3e0f64 * WV) / BH)))).ln()))) / ((B / WT) + ((WI / EB) * (rspice_limited_exp(((-1e0f64 * WE) / WU)))));
                        WX
                    } else {
                        let WY = ((WU * WI) * WV) / ((B / WT) + ((WI / EB) * (rspice_limited_exp(((-1e0f64 * WE) / WU)))));
                        WY
                    };
                    let XA = WE - (WZ / WI);
                    let XB = if ((XA - WE).abs()) > FH { 1.0 } else { 0.0 };
                    let YX;
                    if XB != 0.0 {
                        let XC = WE - XA;
                        let XD = (AO * XC) + (AO * (((XC * XC) + 4e-18f64).sqrt()));
                        let XE = WI.powf(EV);
                        let XF = XD.powf(EV);
                        let XG = XD.powf(-3.333333333333333e-1f64);
                        let XH = WP * XE;
                        let XJ = XI * XE;
                        let XK = XA / VR;
                        let XL = XK - ((XH * XF) / VR);
                        let XM = XK - ((XJ * XF) / VR);
                        let XN = if XL >= FU { 1.0 } else { 0.0 };
                        let XU;
                        if XN != 0.0 {
                            XU = XL;
                        } else {
                            let XO = if XL <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let XV = if XO != 0.0 {
                                A
                            } else {
                                let XP = ((XL.exp()) + B).ln();
                                XP
                            };
                            XU = XV;
                        }
                        let XQ = if XM >= FU { 1.0 } else { 0.0 };
                        let XW;
                        if XQ != 0.0 {
                            XW = XM;
                        } else {
                            let XR = if XM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let XX = if XR != 0.0 {
                                A
                            } else {
                                let XS = ((XM.exp()) + B).ln();
                                XS
                            };
                            XW = XX;
                        }
                        let XT = EB * VR;
                        let XY = rspice_limited_exp(XL);
                        let XZ = rspice_limited_exp(XM);
                        let YA = XA - ((((WI * XD) - (XT * XU)) - (XT * XW)) / (((-1e0f64 * WI) - (((XY * EB) * (B + (EV * (XH * XG)))) / (B + XY))) - (((XZ * EB) * (B + (EV * (XJ * XG)))) / (B + XZ))));
                        let YB = WE - YA;
                        let YC = (AO * YB) + (AO * (((YB * YB) + 4e-18f64).sqrt()));
                        let YD = YC.powf(-3.333333333333333e-1f64);
                        let YE = YC.powf(EV);
                        let YF = YA / VR;
                        let YG = YF - ((XH * YE) / VR);
                        let YH = YF - ((XJ * YE) / VR);
                        let YI = if YG >= FU { 1.0 } else { 0.0 };
                        let YO;
                        if YI != 0.0 {
                            YO = YG;
                        } else {
                            let YJ = if YG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let YP = if YJ != 0.0 {
                                A
                            } else {
                                let YK = ((YG.exp()) + B).ln();
                                YK
                            };
                            YO = YP;
                        }
                        let YL = if YH >= FU { 1.0 } else { 0.0 };
                        let YQ;
                        if YL != 0.0 {
                            YQ = YH;
                        } else {
                            let YM = if YH <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let YR = if YM != 0.0 {
                                A
                            } else {
                                let YN = ((YH.exp()) + B).ln();
                                YN
                            };
                            YQ = YR;
                        }
                        let YS = rspice_limited_exp(YG);
                        let YT = rspice_limited_exp(YH);
                        let YU = YA - ((((WI * YC) - (XT * YO)) - (XT * YQ)) / (((-1e0f64 * WI) - (((YS * EB) * (B + (EV * (XH * YD)))) / (B + YS))) - (((YT * EB) * (B + (EV * (XJ * YD)))) / (B + YT))));
                        YX = YU;
                    } else {
                        YX = XA;
                    }
                    let YY = (VY / CO) * ((WE - YX).abs());
                    let YZ = WJ + (AO * ((WK + 3.6e-1f64).sqrt()));
                    let ZA = ((AW * (YW * HP)) / ((YV * HN) / (((B + (HU * YY)) + (HV * (YY * YY))) + (HW * (HT * ((DV - YX).abs())))))) * VZ;
                    let ZB = WE - (VN * ((B + ((VN / ((ZA * YZ) / (ZA + YZ))).powf(IA))).powf((-1e0f64 / IA))));
                    let ZC = (AO * ZB) + (AO * (((ZB * ZB) + 3.6e-1f64).sqrt()));
                    let ZD = ZC * ZC;
                    let ZE = (WI * ZC).powf(EV);
                    let ZF = ((ZC + (VR * (B - ((WF * ((ZC * WG) / ((ZD + WN).sqrt()))).ln())))) - (WQ * ZE)) / ((ZC * (B + (VR / ((ZC * WH) / ((ZD + WO).sqrt()))))) + (WS * ZE));
                    let ZG = ZB / WU;
                    let ZH = if ZG < FB { 1.0 } else { 0.0 };
                    let ZK = if ZH != 0.0 {
                        let ZI = ((WU * WI) * (((BD * ZG) / BH) + (((rspice_limited_exp((ZG / BH))) + (rspice_limited_exp(((-3e0f64 * ZG) / BH)))).ln()))) / ((B / ZF) + ((WI / EB) * (rspice_limited_exp(((-1e0f64 * ZB) / WU)))));
                        ZI
                    } else {
                        let ZJ = ((WU * WI) * ZG) / ((B / ZF) + ((WI / EB) * (rspice_limited_exp(((-1e0f64 * ZB) / WU)))));
                        ZJ
                    };
                    let ZL = ZB - (ZK / WI);
                    let ZM = if ((ZL - ZB).abs()) > FH { 1.0 } else { 0.0 };
                    if ZM != 0.0 {
                        let ZN = ZB - ZL;
                        let ZO = (AO * ZN) + (AO * (((ZN * ZN) + 4e-18f64).sqrt()));
                        let ZP = WI.powf(EV);
                        let ZQ = ZO.powf(EV);
                        let ZR = ZO.powf(-3.333333333333333e-1f64);
                        let ZS = WP * ZP;
                        let ZT = XI * ZP;
                        let ZU = ZL / VR;
                        let ZV = ZU - ((ZS * ZQ) / VR);
                        let ZW = ZU - ((ZT * ZQ) / VR);
                        let ZX = if ZV >= FU { 1.0 } else { 0.0 };
                        let AAE;
                        if ZX != 0.0 {
                            AAE = ZV;
                        } else {
                            let ZY = if ZV <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AAF = if ZY != 0.0 {
                                A
                            } else {
                                let ZZ = ((ZV.exp()) + B).ln();
                                ZZ
                            };
                            AAE = AAF;
                        }
                        let AAA = if ZW >= FU { 1.0 } else { 0.0 };
                        let AAG;
                        if AAA != 0.0 {
                            AAG = ZW;
                        } else {
                            let AAB = if ZW <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AAH = if AAB != 0.0 {
                                A
                            } else {
                                let AAC = ((ZW.exp()) + B).ln();
                                AAC
                            };
                            AAG = AAH;
                        }
                        let AAD = EB * VR;
                        let AAI = rspice_limited_exp(ZV);
                        let AAJ = rspice_limited_exp(ZW);
                        let AAK = ZL - ((((WI * ZO) - (AAD * AAE)) - (AAD * AAG)) / (((-1e0f64 * WI) - (((AAI * EB) * (B + (EV * (ZS * ZR)))) / (B + AAI))) - (((AAJ * EB) * (B + (EV * (ZT * ZR)))) / (B + AAJ))));
                        let AAL = ZB - AAK;
                        let AAM = ((AO * AAL) + (AO * (((AAL * AAL) + 4e-18f64).sqrt()))).powf(EV);
                        let AAN = AAK / VR;
                        let AAO = AAN - ((ZS * AAM) / VR);
                        let AAP = AAN - ((ZT * AAM) / VR);
                        let AAQ = if AAO >= FU { 1.0 } else { 0.0 };
                        if AAQ != 0.0 {
                        } else {
                            let AAR = if AAO <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AAR != 0.0 {
                            } else {
                            }
                        }
                        let AAS = if AAP >= FU { 1.0 } else { 0.0 };
                        if AAS != 0.0 {
                        } else {
                            let AAT = if AAP <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AAT != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let AAV = if AAU < A { 1.0 } else { 0.0 };
                    if AAV != 0.0 {
                    } else {
                    }
                    DCO = VR;
                } else {
                    DCO = CV;
                }
                DCN = DCO;
            } else {
                let AAW = if VA != A { 1.0 } else { 0.0 };
                let DCP;
                if AAW != 0.0 {
                    let AAX = if VA == B { 1.0 } else { 0.0 };
                    let AAZ = if AAX != 0.0 {
                        R
                    } else {
                        let AAY = AC - M;
                        AAY
                    };
                    let ABA = AI * (B + VP);
                    let ABB = VS + (DP * VT);
                    let ABC = CO / VX;
                    let ABD = ABB + (ABA * (((VZ / ((EC * ABA) * ABA)) * WA).ln()));
                    let ABE = AAZ - ABD;
                    let ABF = ((AO * (ABE + (((ABE * ABE) + EG).sqrt()))) + ABD) - ABB;
                    let ABG = ABC / (5.19105229416e-2f64 * ABA);
                    let ABH = EJ / ABG;
                    let ABI = B / ABG;
                    let ABJ = ABC / EA;
                    let ABK = AO * ABF;
                    let ABL = ABF * ABF;
                    let ABM = ABK + (AO * ((ABL + 3.6e-1f64).sqrt()));
                    let ABN = ABM * ABM;
                    let ABO = ABH * ABH;
                    let ABP = ABI * ABI;
                    let ABQ = WP / BD;
                    let ABR = (ABJ * ABM).powf(EV);
                    let ABS = (AW * WP) / BD;
                    let ABT = ((ABM + (ABA * (B - ((ABG * ((ABM * ABH) / ((ABN + ABO).sqrt()))).ln())))) - (ABQ * ABR)) / ((ABM * (B + (ABA / ((ABM * ABI) / ((ABN + ABP).sqrt()))))) + (ABS * ABR));
                    let ABU = AW * ABA;
                    let ABV = ABF / ABU;
                    let ABW = if ABV < FB { 1.0 } else { 0.0 };
                    let ABZ = if ABW != 0.0 {
                        let ABX = ((ABU * ABJ) * (((BD * ABV) / BH) + (((rspice_limited_exp((ABV / BH))) + (rspice_limited_exp(((-3e0f64 * ABV) / BH)))).ln()))) / ((B / ABT) + ((ABJ / EB) * (rspice_limited_exp(((-1e0f64 * ABF) / ABU)))));
                        ABX
                    } else {
                        let ABY = ((ABU * ABJ) * ABV) / ((B / ABT) + ((ABJ / EB) * (rspice_limited_exp(((-1e0f64 * ABF) / ABU)))));
                        ABY
                    };
                    let ACA = ABF - (ABZ / ABJ);
                    let ACB = if ((ACA - ABF).abs()) > FH { 1.0 } else { 0.0 };
                    let ADU;
                    if ACB != 0.0 {
                        let ACC = ABF - ACA;
                        let ACD = (AO * ACC) + (AO * (((ACC * ACC) + 4e-18f64).sqrt()));
                        let ACE = ABJ.powf(EV);
                        let ACF = ACD.powf(EV);
                        let ACG = ACD.powf(-3.333333333333333e-1f64);
                        let ACH = WP * ACE;
                        let ACI = XI * ACE;
                        let ACJ = ACA / ABA;
                        let ACK = ACJ - ((ACH * ACF) / ABA);
                        let ACL = ACJ - ((ACI * ACF) / ABA);
                        let ACM = if ACK >= FU { 1.0 } else { 0.0 };
                        let ACT;
                        if ACM != 0.0 {
                            ACT = ACK;
                        } else {
                            let ACN = if ACK <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ACU = if ACN != 0.0 {
                                A
                            } else {
                                let ACO = ((ACK.exp()) + B).ln();
                                ACO
                            };
                            ACT = ACU;
                        }
                        let ACP = if ACL >= FU { 1.0 } else { 0.0 };
                        let ACV;
                        if ACP != 0.0 {
                            ACV = ACL;
                        } else {
                            let ACQ = if ACL <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ACW = if ACQ != 0.0 {
                                A
                            } else {
                                let ACR = ((ACL.exp()) + B).ln();
                                ACR
                            };
                            ACV = ACW;
                        }
                        let ACS = EB * ABA;
                        let ACX = rspice_limited_exp(ACK);
                        let ACY = rspice_limited_exp(ACL);
                        let ACZ = ACA - ((((ABJ * ACD) - (ACS * ACT)) - (ACS * ACV)) / (((-1e0f64 * ABJ) - (((ACX * EB) * (B + (EV * (ACH * ACG)))) / (B + ACX))) - (((ACY * EB) * (B + (EV * (ACI * ACG)))) / (B + ACY))));
                        let ADA = ABF - ACZ;
                        let ADB = (AO * ADA) + (AO * (((ADA * ADA) + 4e-18f64).sqrt()));
                        let ADC = ADB.powf(-3.333333333333333e-1f64);
                        let ADD = ADB.powf(EV);
                        let ADE = ACZ / ABA;
                        let ADF = ADE - ((ACH * ADD) / ABA);
                        let ADG = ADE - ((ACI * ADD) / ABA);
                        let ADH = if ADF >= FU { 1.0 } else { 0.0 };
                        let ADN;
                        if ADH != 0.0 {
                            ADN = ADF;
                        } else {
                            let ADI = if ADF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ADO = if ADI != 0.0 {
                                A
                            } else {
                                let ADJ = ((ADF.exp()) + B).ln();
                                ADJ
                            };
                            ADN = ADO;
                        }
                        let ADK = if ADG >= FU { 1.0 } else { 0.0 };
                        let ADP;
                        if ADK != 0.0 {
                            ADP = ADG;
                        } else {
                            let ADL = if ADG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ADQ = if ADL != 0.0 {
                                A
                            } else {
                                let ADM = ((ADG.exp()) + B).ln();
                                ADM
                            };
                            ADP = ADQ;
                        }
                        let ADR = rspice_limited_exp(ADF);
                        let ADS = rspice_limited_exp(ADG);
                        let ADT = ACZ - ((((ABJ * ADB) - (ACS * ADN)) - (ACS * ADP)) / (((-1e0f64 * ABJ) - (((ADR * EB) * (B + (EV * (ACH * ADC)))) / (B + ADR))) - (((ADS * EB) * (B + (EV * (ACI * ADC)))) / (B + ADS))));
                        ADU = ADT;
                    } else {
                        ADU = ACA;
                    }
                    let ADV = (ABC / CO) * ((ABF - ADU).abs());
                    let ADW = ABK + (AO * ((ABL + 3.6e-1f64).sqrt()));
                    let ADX = ((AW * (YW * HP)) / ((YV * HN) / (((B + (HU * ADV)) + (HV * (ADV * ADV))) + (HW * (HT * ((DV - ADU).abs())))))) * VZ;
                    let ADY = ABF - (A * ((B + ((A / ((ADX * ADW) / (ADX + ADW))).powf(IA))).powf((-1e0f64 / IA))));
                    let ADZ = (AO * ADY) + (AO * (((ADY * ADY) + 3.6e-1f64).sqrt()));
                    let AEA = ADZ * ADZ;
                    let AEB = (ABJ * ADZ).powf(EV);
                    let AEC = ((ADZ + (ABA * (B - ((ABG * ((ADZ * ABH) / ((AEA + ABO).sqrt()))).ln())))) - (ABQ * AEB)) / ((ADZ * (B + (ABA / ((ADZ * ABI) / ((AEA + ABP).sqrt()))))) + (ABS * AEB));
                    let AED = ADY / ABU;
                    let AEE = if AED < FB { 1.0 } else { 0.0 };
                    let AEH = if AEE != 0.0 {
                        let AEF = ((ABU * ABJ) * (((BD * AED) / BH) + (((rspice_limited_exp((AED / BH))) + (rspice_limited_exp(((-3e0f64 * AED) / BH)))).ln()))) / ((B / AEC) + ((ABJ / EB) * (rspice_limited_exp(((-1e0f64 * ADY) / ABU)))));
                        AEF
                    } else {
                        let AEG = ((ABU * ABJ) * AED) / ((B / AEC) + ((ABJ / EB) * (rspice_limited_exp(((-1e0f64 * ADY) / ABU)))));
                        AEG
                    };
                    let AEI = ADY - (AEH / ABJ);
                    let AEJ = if ((AEI - ADY).abs()) > FH { 1.0 } else { 0.0 };
                    if AEJ != 0.0 {
                        let AEK = ADY - AEI;
                        let AEL = (AO * AEK) + (AO * (((AEK * AEK) + 4e-18f64).sqrt()));
                        let AEM = ABJ.powf(EV);
                        let AEN = AEL.powf(EV);
                        let AEO = AEL.powf(-3.333333333333333e-1f64);
                        let AEP = WP * AEM;
                        let AEQ = XI * AEM;
                        let AER = AEI / ABA;
                        let AES = AER - ((AEP * AEN) / ABA);
                        let AET = AER - ((AEQ * AEN) / ABA);
                        let AEU = if AES >= FU { 1.0 } else { 0.0 };
                        let AFB;
                        if AEU != 0.0 {
                            AFB = AES;
                        } else {
                            let AEV = if AES <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AFC = if AEV != 0.0 {
                                A
                            } else {
                                let AEW = ((AES.exp()) + B).ln();
                                AEW
                            };
                            AFB = AFC;
                        }
                        let AEX = if AET >= FU { 1.0 } else { 0.0 };
                        let AFD;
                        if AEX != 0.0 {
                            AFD = AET;
                        } else {
                            let AEY = if AET <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AFE = if AEY != 0.0 {
                                A
                            } else {
                                let AEZ = ((AET.exp()) + B).ln();
                                AEZ
                            };
                            AFD = AFE;
                        }
                        let AFA = EB * ABA;
                        let AFF = rspice_limited_exp(AES);
                        let AFG = rspice_limited_exp(AET);
                        let AFH = AEI - ((((ABJ * AEL) - (AFA * AFB)) - (AFA * AFD)) / (((-1e0f64 * ABJ) - (((AFF * EB) * (B + (EV * (AEP * AEO)))) / (B + AFF))) - (((AFG * EB) * (B + (EV * (AEQ * AEO)))) / (B + AFG))));
                        let AFI = ADY - AFH;
                        let AFJ = ((AO * AFI) + (AO * (((AFI * AFI) + 4e-18f64).sqrt()))).powf(EV);
                        let AFK = AFH / ABA;
                        let AFL = AFK - ((AEP * AFJ) / ABA);
                        let AFM = AFK - ((AEQ * AFJ) / ABA);
                        let AFN = if AFL >= FU { 1.0 } else { 0.0 };
                        if AFN != 0.0 {
                        } else {
                            let AFO = if AFL <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AFO != 0.0 {
                            } else {
                            }
                        }
                        let AFP = if AFM >= FU { 1.0 } else { 0.0 };
                        if AFP != 0.0 {
                        } else {
                            let AFQ = if AFM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AFQ != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCP = ABA;
                } else {
                    DCP = CV;
                }
                DCN = DCP;
            }
            let DCL;
            if UZ != 0.0 {
                let AFS = if AFR != A { 1.0 } else { 0.0 };
                let DCM;
                if AFS != 0.0 {
                    let AFU = N - AFT;
                    let AFV = if AFR == B { 1.0 } else { 0.0 };
                    let AGC;
                    let AGD;
                    if AFV != 0.0 {
                        let AFW = P - AFT;
                        AGC = Q;
                        AGD = AFW;
                    } else {
                        let AFX = AC - AFT;
                        let AFY = AC - N;
                        AGC = AFY;
                        AGD = AFX;
                    }
                    let AFZ = if AFU < A { 1.0 } else { 0.0 };
                    let AGE;
                    let AGK;
                    let AKY;
                    if AFZ != 0.0 {
                        let AGB = AGA * AFU;
                        AGE = AGB;
                        AGK = AGC;
                        AKY = AGA;
                    } else {
                        AGE = AFU;
                        AGK = AGD;
                        AKY = B;
                    }
                    let AGF = (((AGE * AGE) + C).sqrt()) - Z;
                    let AGG = AI * ((B + VP) + (VQ * AGF));
                    let AGH = (VS + (DP * VT)) - ((VU * (AGF * VV)) / (((AGF * AGF) + (VV * VV)).sqrt()));
                    let AGI = CO / VX;
                    let AGJ = AGH + (AGG * (((VZ / ((EC * AGG) * AGG)) * WA).ln()));
                    let AGL = AGK - AGJ;
                    let AGM = ((AO * (AGL + (((AGL * AGL) + EG).sqrt()))) + AGJ) - AGH;
                    let AGN = AGI / (5.19105229416e-2f64 * AGG);
                    let AGO = EJ / AGN;
                    let AGP = B / AGN;
                    let AGQ = AGI / EA;
                    let AGR = AO * AGM;
                    let AGS = AGM * AGM;
                    let AGT = AGR + (AO * ((AGS + 3.6e-1f64).sqrt()));
                    let AGU = AGT * AGT;
                    let AGV = AGO * AGO;
                    let AGW = AGP * AGP;
                    let AGX = WP / BD;
                    let AGY = (AGQ * AGT).powf(EV);
                    let AGZ = (AW * WP) / BD;
                    let AHA = ((AGT + (AGG * (B - ((AGN * ((AGT * AGO) / ((AGU + AGV).sqrt()))).ln())))) - (AGX * AGY)) / ((AGT * (B + (AGG / ((AGT * AGP) / ((AGU + AGW).sqrt()))))) + (AGZ * AGY));
                    let AHB = AW * AGG;
                    let AHC = AGM / AHB;
                    let AHD = if AHC < FB { 1.0 } else { 0.0 };
                    let AHG = if AHD != 0.0 {
                        let AHE = ((AHB * AGQ) * (((BD * AHC) / BH) + (((rspice_limited_exp((AHC / BH))) + (rspice_limited_exp(((-3e0f64 * AHC) / BH)))).ln()))) / ((B / AHA) + ((AGQ / EB) * (rspice_limited_exp(((-1e0f64 * AGM) / AHB)))));
                        AHE
                    } else {
                        let AHF = ((AHB * AGQ) * AHC) / ((B / AHA) + ((AGQ / EB) * (rspice_limited_exp(((-1e0f64 * AGM) / AHB)))));
                        AHF
                    };
                    let AHH = AGM - (AHG / AGQ);
                    let AHI = if ((AHH - AGM).abs()) > FH { 1.0 } else { 0.0 };
                    let AJB;
                    if AHI != 0.0 {
                        let AHJ = AGM - AHH;
                        let AHK = (AO * AHJ) + (AO * (((AHJ * AHJ) + 4e-18f64).sqrt()));
                        let AHL = AGQ.powf(EV);
                        let AHM = AHK.powf(EV);
                        let AHN = AHK.powf(-3.333333333333333e-1f64);
                        let AHO = WP * AHL;
                        let AHP = XI * AHL;
                        let AHQ = AHH / AGG;
                        let AHR = AHQ - ((AHO * AHM) / AGG);
                        let AHS = AHQ - ((AHP * AHM) / AGG);
                        let AHT = if AHR >= FU { 1.0 } else { 0.0 };
                        let AIA;
                        if AHT != 0.0 {
                            AIA = AHR;
                        } else {
                            let AHU = if AHR <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AIB = if AHU != 0.0 {
                                A
                            } else {
                                let AHV = ((AHR.exp()) + B).ln();
                                AHV
                            };
                            AIA = AIB;
                        }
                        let AHW = if AHS >= FU { 1.0 } else { 0.0 };
                        let AIC;
                        if AHW != 0.0 {
                            AIC = AHS;
                        } else {
                            let AHX = if AHS <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AID = if AHX != 0.0 {
                                A
                            } else {
                                let AHY = ((AHS.exp()) + B).ln();
                                AHY
                            };
                            AIC = AID;
                        }
                        let AHZ = EB * AGG;
                        let AIE = rspice_limited_exp(AHR);
                        let AIF = rspice_limited_exp(AHS);
                        let AIG = AHH - ((((AGQ * AHK) - (AHZ * AIA)) - (AHZ * AIC)) / (((-1e0f64 * AGQ) - (((AIE * EB) * (B + (EV * (AHO * AHN)))) / (B + AIE))) - (((AIF * EB) * (B + (EV * (AHP * AHN)))) / (B + AIF))));
                        let AIH = AGM - AIG;
                        let AII = (AO * AIH) + (AO * (((AIH * AIH) + 4e-18f64).sqrt()));
                        let AIJ = AII.powf(-3.333333333333333e-1f64);
                        let AIK = AII.powf(EV);
                        let AIL = AIG / AGG;
                        let AIM = AIL - ((AHO * AIK) / AGG);
                        let AIN = AIL - ((AHP * AIK) / AGG);
                        let AIO = if AIM >= FU { 1.0 } else { 0.0 };
                        let AIU;
                        if AIO != 0.0 {
                            AIU = AIM;
                        } else {
                            let AIP = if AIM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AIV = if AIP != 0.0 {
                                A
                            } else {
                                let AIQ = ((AIM.exp()) + B).ln();
                                AIQ
                            };
                            AIU = AIV;
                        }
                        let AIR = if AIN >= FU { 1.0 } else { 0.0 };
                        let AIW;
                        if AIR != 0.0 {
                            AIW = AIN;
                        } else {
                            let AIS = if AIN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AIX = if AIS != 0.0 {
                                A
                            } else {
                                let AIT = ((AIN.exp()) + B).ln();
                                AIT
                            };
                            AIW = AIX;
                        }
                        let AIY = rspice_limited_exp(AIM);
                        let AIZ = rspice_limited_exp(AIN);
                        let AJA = AIG - ((((AGQ * AII) - (AHZ * AIU)) - (AHZ * AIW)) / (((-1e0f64 * AGQ) - (((AIY * EB) * (B + (EV * (AHO * AIJ)))) / (B + AIY))) - (((AIZ * EB) * (B + (EV * (AHP * AIJ)))) / (B + AIZ))));
                        AJB = AJA;
                    } else {
                        AJB = AHH;
                    }
                    let AJC = (AGI / CO) * ((AGM - AJB).abs());
                    let AJD = AGR + (AO * ((AGS + 3.6e-1f64).sqrt()));
                    let AJE = ((AW * (YW * HP)) / ((YV * HN) / (((B + (HU * AJC)) + (HV * (AJC * AJC))) + (HW * (HT * ((DV - AJB).abs())))))) * VZ;
                    let AJF = AGM - (AGE * ((B + ((AGE / ((AJE * AJD) / (AJE + AJD))).powf(IA))).powf((-1e0f64 / IA))));
                    let AJG = (AO * AJF) + (AO * (((AJF * AJF) + 3.6e-1f64).sqrt()));
                    let AJH = AJG * AJG;
                    let AJI = (AGQ * AJG).powf(EV);
                    let AJJ = ((AJG + (AGG * (B - ((AGN * ((AJG * AGO) / ((AJH + AGV).sqrt()))).ln())))) - (AGX * AJI)) / ((AJG * (B + (AGG / ((AJG * AGP) / ((AJH + AGW).sqrt()))))) + (AGZ * AJI));
                    let AJK = AJF / AHB;
                    let AJL = if AJK < FB { 1.0 } else { 0.0 };
                    let AJO = if AJL != 0.0 {
                        let AJM = ((AHB * AGQ) * (((BD * AJK) / BH) + (((rspice_limited_exp((AJK / BH))) + (rspice_limited_exp(((-3e0f64 * AJK) / BH)))).ln()))) / ((B / AJJ) + ((AGQ / EB) * (rspice_limited_exp(((-1e0f64 * AJF) / AHB)))));
                        AJM
                    } else {
                        let AJN = ((AHB * AGQ) * AJK) / ((B / AJJ) + ((AGQ / EB) * (rspice_limited_exp(((-1e0f64 * AJF) / AHB)))));
                        AJN
                    };
                    let AJP = AJF - (AJO / AGQ);
                    let AJQ = if ((AJP - AJF).abs()) > FH { 1.0 } else { 0.0 };
                    if AJQ != 0.0 {
                        let AJR = AJF - AJP;
                        let AJS = (AO * AJR) + (AO * (((AJR * AJR) + 4e-18f64).sqrt()));
                        let AJT = AGQ.powf(EV);
                        let AJU = AJS.powf(EV);
                        let AJV = AJS.powf(-3.333333333333333e-1f64);
                        let AJW = WP * AJT;
                        let AJX = XI * AJT;
                        let AJY = AJP / AGG;
                        let AJZ = AJY - ((AJW * AJU) / AGG);
                        let AKA = AJY - ((AJX * AJU) / AGG);
                        let AKB = if AJZ >= FU { 1.0 } else { 0.0 };
                        let AKI;
                        if AKB != 0.0 {
                            AKI = AJZ;
                        } else {
                            let AKC = if AJZ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AKJ = if AKC != 0.0 {
                                A
                            } else {
                                let AKD = ((AJZ.exp()) + B).ln();
                                AKD
                            };
                            AKI = AKJ;
                        }
                        let AKE = if AKA >= FU { 1.0 } else { 0.0 };
                        let AKK;
                        if AKE != 0.0 {
                            AKK = AKA;
                        } else {
                            let AKF = if AKA <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AKL = if AKF != 0.0 {
                                A
                            } else {
                                let AKG = ((AKA.exp()) + B).ln();
                                AKG
                            };
                            AKK = AKL;
                        }
                        let AKH = EB * AGG;
                        let AKM = rspice_limited_exp(AJZ);
                        let AKN = rspice_limited_exp(AKA);
                        let AKO = AJP - ((((AGQ * AJS) - (AKH * AKI)) - (AKH * AKK)) / (((-1e0f64 * AGQ) - (((AKM * EB) * (B + (EV * (AJW * AJV)))) / (B + AKM))) - (((AKN * EB) * (B + (EV * (AJX * AJV)))) / (B + AKN))));
                        let AKP = AJF - AKO;
                        let AKQ = ((AO * AKP) + (AO * (((AKP * AKP) + 4e-18f64).sqrt()))).powf(EV);
                        let AKR = AKO / AGG;
                        let AKS = AKR - ((AJW * AKQ) / AGG);
                        let AKT = AKR - ((AJX * AKQ) / AGG);
                        let AKU = if AKS >= FU { 1.0 } else { 0.0 };
                        if AKU != 0.0 {
                        } else {
                            let AKV = if AKS <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AKV != 0.0 {
                            } else {
                            }
                        }
                        let AKW = if AKT >= FU { 1.0 } else { 0.0 };
                        if AKW != 0.0 {
                        } else {
                            let AKX = if AKT <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AKX != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let AKZ = if AKY < A { 1.0 } else { 0.0 };
                    if AKZ != 0.0 {
                    } else {
                    }
                    DCM = AGG;
                } else {
                    DCM = DCN;
                }
                DCL = DCM;
            } else {
                let ALA = if AFR != A { 1.0 } else { 0.0 };
                let DCQ;
                if ALA != 0.0 {
                    let ALB = if AFR == B { 1.0 } else { 0.0 };
                    if ALB != 0.0 {
                    } else {
                    }
                    let ALC = AI * (B + VP);
                    let ALD = VS + (DP * VT);
                    let ALE = CO / VX;
                    let ALF = ALD + (ALC * (((VZ / ((EC * ALC) * ALC)) * WA).ln()));
                    let ALG = A - ALF;
                    let ALH = ((AO * (ALG + (((ALG * ALG) + EG).sqrt()))) + ALF) - ALD;
                    let ALI = ALE / (5.19105229416e-2f64 * ALC);
                    let ALJ = EJ / ALI;
                    let ALK = B / ALI;
                    let ALL = ALE / EA;
                    let ALM = AO * ALH;
                    let ALN = ALH * ALH;
                    let ALO = ALM + (AO * ((ALN + 3.6e-1f64).sqrt()));
                    let ALP = ALO * ALO;
                    let ALQ = ALJ * ALJ;
                    let ALR = ALK * ALK;
                    let ALS = WP / BD;
                    let ALT = (ALL * ALO).powf(EV);
                    let ALU = (AW * WP) / BD;
                    let ALV = ((ALO + (ALC * (B - ((ALI * ((ALO * ALJ) / ((ALP + ALQ).sqrt()))).ln())))) - (ALS * ALT)) / ((ALO * (B + (ALC / ((ALO * ALK) / ((ALP + ALR).sqrt()))))) + (ALU * ALT));
                    let ALW = AW * ALC;
                    let ALX = ALH / ALW;
                    let ALY = if ALX < FB { 1.0 } else { 0.0 };
                    let AMB = if ALY != 0.0 {
                        let ALZ = ((ALW * ALL) * (((BD * ALX) / BH) + (((rspice_limited_exp((ALX / BH))) + (rspice_limited_exp(((-3e0f64 * ALX) / BH)))).ln()))) / ((B / ALV) + ((ALL / EB) * (rspice_limited_exp(((-1e0f64 * ALH) / ALW)))));
                        ALZ
                    } else {
                        let AMA = ((ALW * ALL) * ALX) / ((B / ALV) + ((ALL / EB) * (rspice_limited_exp(((-1e0f64 * ALH) / ALW)))));
                        AMA
                    };
                    let AMC = ALH - (AMB / ALL);
                    let AMD = if ((AMC - ALH).abs()) > FH { 1.0 } else { 0.0 };
                    let ANW;
                    if AMD != 0.0 {
                        let AME = ALH - AMC;
                        let AMF = (AO * AME) + (AO * (((AME * AME) + 4e-18f64).sqrt()));
                        let AMG = ALL.powf(EV);
                        let AMH = AMF.powf(EV);
                        let AMI = AMF.powf(-3.333333333333333e-1f64);
                        let AMJ = WP * AMG;
                        let AMK = XI * AMG;
                        let AML = AMC / ALC;
                        let AMM = AML - ((AMJ * AMH) / ALC);
                        let AMN = AML - ((AMK * AMH) / ALC);
                        let AMO = if AMM >= FU { 1.0 } else { 0.0 };
                        let AMV;
                        if AMO != 0.0 {
                            AMV = AMM;
                        } else {
                            let AMP = if AMM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AMW = if AMP != 0.0 {
                                A
                            } else {
                                let AMQ = ((AMM.exp()) + B).ln();
                                AMQ
                            };
                            AMV = AMW;
                        }
                        let AMR = if AMN >= FU { 1.0 } else { 0.0 };
                        let AMX;
                        if AMR != 0.0 {
                            AMX = AMN;
                        } else {
                            let AMS = if AMN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AMY = if AMS != 0.0 {
                                A
                            } else {
                                let AMT = ((AMN.exp()) + B).ln();
                                AMT
                            };
                            AMX = AMY;
                        }
                        let AMU = EB * ALC;
                        let AMZ = rspice_limited_exp(AMM);
                        let ANA = rspice_limited_exp(AMN);
                        let ANB = AMC - ((((ALL * AMF) - (AMU * AMV)) - (AMU * AMX)) / (((-1e0f64 * ALL) - (((AMZ * EB) * (B + (EV * (AMJ * AMI)))) / (B + AMZ))) - (((ANA * EB) * (B + (EV * (AMK * AMI)))) / (B + ANA))));
                        let ANC = ALH - ANB;
                        let AND = (AO * ANC) + (AO * (((ANC * ANC) + 4e-18f64).sqrt()));
                        let ANE = AND.powf(-3.333333333333333e-1f64);
                        let ANF = AND.powf(EV);
                        let ANG = ANB / ALC;
                        let ANH = ANG - ((AMJ * ANF) / ALC);
                        let ANI = ANG - ((AMK * ANF) / ALC);
                        let ANJ = if ANH >= FU { 1.0 } else { 0.0 };
                        let ANP;
                        if ANJ != 0.0 {
                            ANP = ANH;
                        } else {
                            let ANK = if ANH <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ANQ = if ANK != 0.0 {
                                A
                            } else {
                                let ANL = ((ANH.exp()) + B).ln();
                                ANL
                            };
                            ANP = ANQ;
                        }
                        let ANM = if ANI >= FU { 1.0 } else { 0.0 };
                        let ANR;
                        if ANM != 0.0 {
                            ANR = ANI;
                        } else {
                            let ANN = if ANI <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ANS = if ANN != 0.0 {
                                A
                            } else {
                                let ANO = ((ANI.exp()) + B).ln();
                                ANO
                            };
                            ANR = ANS;
                        }
                        let ANT = rspice_limited_exp(ANH);
                        let ANU = rspice_limited_exp(ANI);
                        let ANV = ANB - ((((ALL * AND) - (AMU * ANP)) - (AMU * ANR)) / (((-1e0f64 * ALL) - (((ANT * EB) * (B + (EV * (AMJ * ANE)))) / (B + ANT))) - (((ANU * EB) * (B + (EV * (AMK * ANE)))) / (B + ANU))));
                        ANW = ANV;
                    } else {
                        ANW = AMC;
                    }
                    let ANX = (ALE / CO) * ((ALH - ANW).abs());
                    let ANY = ALM + (AO * ((ALN + 3.6e-1f64).sqrt()));
                    let ANZ = ((AW * (YW * HP)) / ((YV * HN) / (((B + (HU * ANX)) + (HV * (ANX * ANX))) + (HW * (HT * ((DV - ANW).abs())))))) * VZ;
                    let AOA = ALH - (A * ((B + ((A / ((ANZ * ANY) / (ANZ + ANY))).powf(IA))).powf((-1e0f64 / IA))));
                    let AOB = (AO * AOA) + (AO * (((AOA * AOA) + 3.6e-1f64).sqrt()));
                    let AOC = AOB * AOB;
                    let AOD = (ALL * AOB).powf(EV);
                    let AOE = ((AOB + (ALC * (B - ((ALI * ((AOB * ALJ) / ((AOC + ALQ).sqrt()))).ln())))) - (ALS * AOD)) / ((AOB * (B + (ALC / ((AOB * ALK) / ((AOC + ALR).sqrt()))))) + (ALU * AOD));
                    let AOF = AOA / ALW;
                    let AOG = if AOF < FB { 1.0 } else { 0.0 };
                    let AOJ = if AOG != 0.0 {
                        let AOH = ((ALW * ALL) * (((BD * AOF) / BH) + (((rspice_limited_exp((AOF / BH))) + (rspice_limited_exp(((-3e0f64 * AOF) / BH)))).ln()))) / ((B / AOE) + ((ALL / EB) * (rspice_limited_exp(((-1e0f64 * AOA) / ALW)))));
                        AOH
                    } else {
                        let AOI = ((ALW * ALL) * AOF) / ((B / AOE) + ((ALL / EB) * (rspice_limited_exp(((-1e0f64 * AOA) / ALW)))));
                        AOI
                    };
                    let AOK = AOA - (AOJ / ALL);
                    let AOL = if ((AOK - AOA).abs()) > FH { 1.0 } else { 0.0 };
                    if AOL != 0.0 {
                        let AOM = AOA - AOK;
                        let AON = (AO * AOM) + (AO * (((AOM * AOM) + 4e-18f64).sqrt()));
                        let AOO = ALL.powf(EV);
                        let AOP = AON.powf(EV);
                        let AOQ = AON.powf(-3.333333333333333e-1f64);
                        let AOR = WP * AOO;
                        let AOS = XI * AOO;
                        let AOT = AOK / ALC;
                        let AOU = AOT - ((AOR * AOP) / ALC);
                        let AOV = AOT - ((AOS * AOP) / ALC);
                        let AOW = if AOU >= FU { 1.0 } else { 0.0 };
                        let APD;
                        if AOW != 0.0 {
                            APD = AOU;
                        } else {
                            let AOX = if AOU <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let APE = if AOX != 0.0 {
                                A
                            } else {
                                let AOY = ((AOU.exp()) + B).ln();
                                AOY
                            };
                            APD = APE;
                        }
                        let AOZ = if AOV >= FU { 1.0 } else { 0.0 };
                        let APF;
                        if AOZ != 0.0 {
                            APF = AOV;
                        } else {
                            let APA = if AOV <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let APG = if APA != 0.0 {
                                A
                            } else {
                                let APB = ((AOV.exp()) + B).ln();
                                APB
                            };
                            APF = APG;
                        }
                        let APC = EB * ALC;
                        let APH = rspice_limited_exp(AOU);
                        let API = rspice_limited_exp(AOV);
                        let APJ = AOK - ((((ALL * AON) - (APC * APD)) - (APC * APF)) / (((-1e0f64 * ALL) - (((APH * EB) * (B + (EV * (AOR * AOQ)))) / (B + APH))) - (((API * EB) * (B + (EV * (AOS * AOQ)))) / (B + API))));
                        let APK = AOA - APJ;
                        let APL = ((AO * APK) + (AO * (((APK * APK) + 4e-18f64).sqrt()))).powf(EV);
                        let APM = APJ / ALC;
                        let APN = APM - ((AOR * APL) / ALC);
                        let APO = APM - ((AOS * APL) / ALC);
                        let APP = if APN >= FU { 1.0 } else { 0.0 };
                        if APP != 0.0 {
                        } else {
                            let APQ = if APN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if APQ != 0.0 {
                            } else {
                            }
                        }
                        let APR = if APO >= FU { 1.0 } else { 0.0 };
                        if APR != 0.0 {
                        } else {
                            let APS = if APO <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if APS != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCQ = ALC;
                } else {
                    DCQ = DCN;
                }
                DCL = DCQ;
            }
            let DCJ;
            if UZ != 0.0 {
                let APU = if APT != A { 1.0 } else { 0.0 };
                let DCK;
                if APU != 0.0 {
                    let APW = APV - VC;
                    let APX = if APT == B { 1.0 } else { 0.0 };
                    let AQF;
                    let AQG;
                    if APX != 0.0 {
                        let APY = P - VC;
                        let APZ = P - APV;
                        AQF = APZ;
                        AQG = APY;
                    } else {
                        let AQA = AC - VC;
                        let AQB = AC - APV;
                        AQF = AQB;
                        AQG = AQA;
                    }
                    let AQC = if APW < A { 1.0 } else { 0.0 };
                    let AQH;
                    let AQW;
                    let AVO;
                    if AQC != 0.0 {
                        let AQE = AQD * APW;
                        AQH = AQE;
                        AQW = AQF;
                        AVO = AQD;
                    } else {
                        AQH = APW;
                        AQW = AQG;
                        AVO = B;
                    }
                    let AQI = (((AQH * AQH) + C).sqrt()) - Z;
                    let AQL = AI * ((B + AQJ) + (AQK * AQI));
                    let AQQ = (AQM - (DP * AQN)) - ((AQO * (AQI * AQP)) / (((AQI * AQI) + (AQP * AQP)).sqrt()));
                    let AQS = CO / AQR;
                    let AQV = AQQ + (AQL * (((AQT / ((EC * AQL) * AQL)) * AQU).ln()));
                    let AQX = AQW - AQV;
                    let AQY = ((AO * (AQX + (((AQX * AQX) + EG).sqrt()))) + AQV) - AQQ;
                    let AQZ = AQS / (5.19105229416e-2f64 * AQL);
                    let ARA = EJ / AQZ;
                    let ARB = B / AQZ;
                    let ARC = AQS / EA;
                    let ARD = AO * AQY;
                    let ARE = AQY * AQY;
                    let ARF = ARD + (AO * ((ARE + 3.6e-1f64).sqrt()));
                    let ARG = ARF * ARF;
                    let ARH = ARA * ARA;
                    let ARI = ARB * ARB;
                    let ARK = ARJ / BD;
                    let ARL = (ARC * ARF).powf(EV);
                    let ARM = (AW * ARJ) / BD;
                    let ARN = ((ARF + (AQL * (B - ((AQZ * ((ARF * ARA) / ((ARG + ARH).sqrt()))).ln())))) - (ARK * ARL)) / ((ARF * (B + (AQL / ((ARF * ARB) / ((ARG + ARI).sqrt()))))) + (ARM * ARL));
                    let ARO = AW * AQL;
                    let ARP = AQY / ARO;
                    let ARQ = if ARP < FB { 1.0 } else { 0.0 };
                    let ART = if ARQ != 0.0 {
                        let ARR = ((ARO * ARC) * (((BD * ARP) / BH) + (((rspice_limited_exp((ARP / BH))) + (rspice_limited_exp(((-3e0f64 * ARP) / BH)))).ln()))) / ((B / ARN) + ((ARC / EB) * (rspice_limited_exp(((-1e0f64 * AQY) / ARO)))));
                        ARR
                    } else {
                        let ARS = ((ARO * ARC) * ARP) / ((B / ARN) + ((ARC / EB) * (rspice_limited_exp(((-1e0f64 * AQY) / ARO)))));
                        ARS
                    };
                    let ARU = AQY - (ART / ARC);
                    let ARV = if ((ARU - AQY).abs()) > FH { 1.0 } else { 0.0 };
                    let ATR;
                    if ARV != 0.0 {
                        let ARW = AQY - ARU;
                        let ARX = (AO * ARW) + (AO * (((ARW * ARW) + 4e-18f64).sqrt()));
                        let ARY = ARC.powf(EV);
                        let ARZ = ARX.powf(EV);
                        let ASA = ARX.powf(-3.333333333333333e-1f64);
                        let ASB = ARJ * ARY;
                        let ASD = ASC * ARY;
                        let ASE = ARU / AQL;
                        let ASF = ASE - ((ASB * ARZ) / AQL);
                        let ASG = ASE - ((ASD * ARZ) / AQL);
                        let ASH = if ASF >= FU { 1.0 } else { 0.0 };
                        let ASO;
                        if ASH != 0.0 {
                            ASO = ASF;
                        } else {
                            let ASI = if ASF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ASP = if ASI != 0.0 {
                                A
                            } else {
                                let ASJ = ((ASF.exp()) + B).ln();
                                ASJ
                            };
                            ASO = ASP;
                        }
                        let ASK = if ASG >= FU { 1.0 } else { 0.0 };
                        let ASQ;
                        if ASK != 0.0 {
                            ASQ = ASG;
                        } else {
                            let ASL = if ASG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ASR = if ASL != 0.0 {
                                A
                            } else {
                                let ASM = ((ASG.exp()) + B).ln();
                                ASM
                            };
                            ASQ = ASR;
                        }
                        let ASN = EB * AQL;
                        let ASS = rspice_limited_exp(ASF);
                        let AST = rspice_limited_exp(ASG);
                        let ASU = ARU - ((((ARC * ARX) - (ASN * ASO)) - (ASN * ASQ)) / (((-1e0f64 * ARC) - (((ASS * EB) * (B + (EV * (ASB * ASA)))) / (B + ASS))) - (((AST * EB) * (B + (EV * (ASD * ASA)))) / (B + AST))));
                        let ASV = AQY - ASU;
                        let ASW = (AO * ASV) + (AO * (((ASV * ASV) + 4e-18f64).sqrt()));
                        let ASX = ASW.powf(-3.333333333333333e-1f64);
                        let ASY = ASW.powf(EV);
                        let ASZ = ASU / AQL;
                        let ATA = ASZ - ((ASB * ASY) / AQL);
                        let ATB = ASZ - ((ASD * ASY) / AQL);
                        let ATC = if ATA >= FU { 1.0 } else { 0.0 };
                        let ATI;
                        if ATC != 0.0 {
                            ATI = ATA;
                        } else {
                            let ATD = if ATA <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ATJ = if ATD != 0.0 {
                                A
                            } else {
                                let ATE = ((ATA.exp()) + B).ln();
                                ATE
                            };
                            ATI = ATJ;
                        }
                        let ATF = if ATB >= FU { 1.0 } else { 0.0 };
                        let ATK;
                        if ATF != 0.0 {
                            ATK = ATB;
                        } else {
                            let ATG = if ATB <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let ATL = if ATG != 0.0 {
                                A
                            } else {
                                let ATH = ((ATB.exp()) + B).ln();
                                ATH
                            };
                            ATK = ATL;
                        }
                        let ATM = rspice_limited_exp(ATA);
                        let ATN = rspice_limited_exp(ATB);
                        let ATO = ASU - ((((ARC * ASW) - (ASN * ATI)) - (ASN * ATK)) / (((-1e0f64 * ARC) - (((ATM * EB) * (B + (EV * (ASB * ASX)))) / (B + ATM))) - (((ATN * EB) * (B + (EV * (ASD * ASX)))) / (B + ATN))));
                        ATR = ATO;
                    } else {
                        ATR = ARU;
                    }
                    let ATS = (AQS / CO) * ((AQY - ATR).abs());
                    let ATT = ARD + (AO * ((ARE + 3.6e-1f64).sqrt()));
                    let ATU = ((AW * (ATQ * HP)) / ((ATP * HN) / (((B + (HU * ATS)) + (HV * (ATS * ATS))) + (HW * (HT * ((DV - ATR).abs())))))) * AQT;
                    let ATV = AQY - (AQH * ((B + ((AQH / ((ATU * ATT) / (ATU + ATT))).powf(IA))).powf((-1e0f64 / IA))));
                    let ATW = (AO * ATV) + (AO * (((ATV * ATV) + 3.6e-1f64).sqrt()));
                    let ATX = ATW * ATW;
                    let ATY = (ARC * ATW).powf(EV);
                    let ATZ = ((ATW + (AQL * (B - ((AQZ * ((ATW * ARA) / ((ATX + ARH).sqrt()))).ln())))) - (ARK * ATY)) / ((ATW * (B + (AQL / ((ATW * ARB) / ((ATX + ARI).sqrt()))))) + (ARM * ATY));
                    let AUA = ATV / ARO;
                    let AUB = if AUA < FB { 1.0 } else { 0.0 };
                    let AUE = if AUB != 0.0 {
                        let AUC = ((ARO * ARC) * (((BD * AUA) / BH) + (((rspice_limited_exp((AUA / BH))) + (rspice_limited_exp(((-3e0f64 * AUA) / BH)))).ln()))) / ((B / ATZ) + ((ARC / EB) * (rspice_limited_exp(((-1e0f64 * ATV) / ARO)))));
                        AUC
                    } else {
                        let AUD = ((ARO * ARC) * AUA) / ((B / ATZ) + ((ARC / EB) * (rspice_limited_exp(((-1e0f64 * ATV) / ARO)))));
                        AUD
                    };
                    let AUF = ATV - (AUE / ARC);
                    let AUG = if ((AUF - ATV).abs()) > FH { 1.0 } else { 0.0 };
                    if AUG != 0.0 {
                        let AUH = ATV - AUF;
                        let AUI = (AO * AUH) + (AO * (((AUH * AUH) + 4e-18f64).sqrt()));
                        let AUJ = ARC.powf(EV);
                        let AUK = AUI.powf(EV);
                        let AUL = AUI.powf(-3.333333333333333e-1f64);
                        let AUM = ARJ * AUJ;
                        let AUN = ASC * AUJ;
                        let AUO = AUF / AQL;
                        let AUP = AUO - ((AUM * AUK) / AQL);
                        let AUQ = AUO - ((AUN * AUK) / AQL);
                        let AUR = if AUP >= FU { 1.0 } else { 0.0 };
                        let AUY;
                        if AUR != 0.0 {
                            AUY = AUP;
                        } else {
                            let AUS = if AUP <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AUZ = if AUS != 0.0 {
                                A
                            } else {
                                let AUT = ((AUP.exp()) + B).ln();
                                AUT
                            };
                            AUY = AUZ;
                        }
                        let AUU = if AUQ >= FU { 1.0 } else { 0.0 };
                        let AVA;
                        if AUU != 0.0 {
                            AVA = AUQ;
                        } else {
                            let AUV = if AUQ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AVB = if AUV != 0.0 {
                                A
                            } else {
                                let AUW = ((AUQ.exp()) + B).ln();
                                AUW
                            };
                            AVA = AVB;
                        }
                        let AUX = EB * AQL;
                        let AVC = rspice_limited_exp(AUP);
                        let AVD = rspice_limited_exp(AUQ);
                        let AVE = AUF - ((((ARC * AUI) - (AUX * AUY)) - (AUX * AVA)) / (((-1e0f64 * ARC) - (((AVC * EB) * (B + (EV * (AUM * AUL)))) / (B + AVC))) - (((AVD * EB) * (B + (EV * (AUN * AUL)))) / (B + AVD))));
                        let AVF = ATV - AVE;
                        let AVG = ((AO * AVF) + (AO * (((AVF * AVF) + 4e-18f64).sqrt()))).powf(EV);
                        let AVH = AVE / AQL;
                        let AVI = AVH - ((AUM * AVG) / AQL);
                        let AVJ = AVH - ((AUN * AVG) / AQL);
                        let AVK = if AVI >= FU { 1.0 } else { 0.0 };
                        if AVK != 0.0 {
                        } else {
                            let AVL = if AVI <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AVL != 0.0 {
                            } else {
                            }
                        }
                        let AVM = if AVJ >= FU { 1.0 } else { 0.0 };
                        if AVM != 0.0 {
                        } else {
                            let AVN = if AVJ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if AVN != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let AVP = if AVO < A { 1.0 } else { 0.0 };
                    if AVP != 0.0 {
                    } else {
                    }
                    DCK = AQL;
                } else {
                    DCK = DCL;
                }
                DCJ = DCK;
            } else {
                let AVQ = if APT != A { 1.0 } else { 0.0 };
                let DCR;
                if AVQ != 0.0 {
                    let AVR = if APT == B { 1.0 } else { 0.0 };
                    let AVT = if AVR != 0.0 {
                        R
                    } else {
                        let AVS = AC - M;
                        AVS
                    };
                    let AVU = AI * (B + AQJ);
                    let AVV = AQM - (DP * AQN);
                    let AVW = CO / AQR;
                    let AVX = AVV + (AVU * (((AQT / ((EC * AVU) * AVU)) * AQU).ln()));
                    let AVY = AVT - AVX;
                    let AVZ = ((AO * (AVY + (((AVY * AVY) + EG).sqrt()))) + AVX) - AVV;
                    let AWA = AVW / (5.19105229416e-2f64 * AVU);
                    let AWB = EJ / AWA;
                    let AWC = B / AWA;
                    let AWD = AVW / EA;
                    let AWE = AO * AVZ;
                    let AWF = AVZ * AVZ;
                    let AWG = AWE + (AO * ((AWF + 3.6e-1f64).sqrt()));
                    let AWH = AWG * AWG;
                    let AWI = AWB * AWB;
                    let AWJ = AWC * AWC;
                    let AWK = ARJ / BD;
                    let AWL = (AWD * AWG).powf(EV);
                    let AWM = (AW * ARJ) / BD;
                    let AWN = ((AWG + (AVU * (B - ((AWA * ((AWG * AWB) / ((AWH + AWI).sqrt()))).ln())))) - (AWK * AWL)) / ((AWG * (B + (AVU / ((AWG * AWC) / ((AWH + AWJ).sqrt()))))) + (AWM * AWL));
                    let AWO = AW * AVU;
                    let AWP = AVZ / AWO;
                    let AWQ = if AWP < FB { 1.0 } else { 0.0 };
                    let AWT = if AWQ != 0.0 {
                        let AWR = ((AWO * AWD) * (((BD * AWP) / BH) + (((rspice_limited_exp((AWP / BH))) + (rspice_limited_exp(((-3e0f64 * AWP) / BH)))).ln()))) / ((B / AWN) + ((AWD / EB) * (rspice_limited_exp(((-1e0f64 * AVZ) / AWO)))));
                        AWR
                    } else {
                        let AWS = ((AWO * AWD) * AWP) / ((B / AWN) + ((AWD / EB) * (rspice_limited_exp(((-1e0f64 * AVZ) / AWO)))));
                        AWS
                    };
                    let AWU = AVZ - (AWT / AWD);
                    let AWV = if ((AWU - AVZ).abs()) > FH { 1.0 } else { 0.0 };
                    let AYO;
                    if AWV != 0.0 {
                        let AWW = AVZ - AWU;
                        let AWX = (AO * AWW) + (AO * (((AWW * AWW) + 4e-18f64).sqrt()));
                        let AWY = AWD.powf(EV);
                        let AWZ = AWX.powf(EV);
                        let AXA = AWX.powf(-3.333333333333333e-1f64);
                        let AXB = ARJ * AWY;
                        let AXC = ASC * AWY;
                        let AXD = AWU / AVU;
                        let AXE = AXD - ((AXB * AWZ) / AVU);
                        let AXF = AXD - ((AXC * AWZ) / AVU);
                        let AXG = if AXE >= FU { 1.0 } else { 0.0 };
                        let AXN;
                        if AXG != 0.0 {
                            AXN = AXE;
                        } else {
                            let AXH = if AXE <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AXO = if AXH != 0.0 {
                                A
                            } else {
                                let AXI = ((AXE.exp()) + B).ln();
                                AXI
                            };
                            AXN = AXO;
                        }
                        let AXJ = if AXF >= FU { 1.0 } else { 0.0 };
                        let AXP;
                        if AXJ != 0.0 {
                            AXP = AXF;
                        } else {
                            let AXK = if AXF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AXQ = if AXK != 0.0 {
                                A
                            } else {
                                let AXL = ((AXF.exp()) + B).ln();
                                AXL
                            };
                            AXP = AXQ;
                        }
                        let AXM = EB * AVU;
                        let AXR = rspice_limited_exp(AXE);
                        let AXS = rspice_limited_exp(AXF);
                        let AXT = AWU - ((((AWD * AWX) - (AXM * AXN)) - (AXM * AXP)) / (((-1e0f64 * AWD) - (((AXR * EB) * (B + (EV * (AXB * AXA)))) / (B + AXR))) - (((AXS * EB) * (B + (EV * (AXC * AXA)))) / (B + AXS))));
                        let AXU = AVZ - AXT;
                        let AXV = (AO * AXU) + (AO * (((AXU * AXU) + 4e-18f64).sqrt()));
                        let AXW = AXV.powf(-3.333333333333333e-1f64);
                        let AXX = AXV.powf(EV);
                        let AXY = AXT / AVU;
                        let AXZ = AXY - ((AXB * AXX) / AVU);
                        let AYA = AXY - ((AXC * AXX) / AVU);
                        let AYB = if AXZ >= FU { 1.0 } else { 0.0 };
                        let AYH;
                        if AYB != 0.0 {
                            AYH = AXZ;
                        } else {
                            let AYC = if AXZ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AYI = if AYC != 0.0 {
                                A
                            } else {
                                let AYD = ((AXZ.exp()) + B).ln();
                                AYD
                            };
                            AYH = AYI;
                        }
                        let AYE = if AYA >= FU { 1.0 } else { 0.0 };
                        let AYJ;
                        if AYE != 0.0 {
                            AYJ = AYA;
                        } else {
                            let AYF = if AYA <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AYK = if AYF != 0.0 {
                                A
                            } else {
                                let AYG = ((AYA.exp()) + B).ln();
                                AYG
                            };
                            AYJ = AYK;
                        }
                        let AYL = rspice_limited_exp(AXZ);
                        let AYM = rspice_limited_exp(AYA);
                        let AYN = AXT - ((((AWD * AXV) - (AXM * AYH)) - (AXM * AYJ)) / (((-1e0f64 * AWD) - (((AYL * EB) * (B + (EV * (AXB * AXW)))) / (B + AYL))) - (((AYM * EB) * (B + (EV * (AXC * AXW)))) / (B + AYM))));
                        AYO = AYN;
                    } else {
                        AYO = AWU;
                    }
                    let AYP = (AVW / CO) * ((AVZ - AYO).abs());
                    let AYQ = AWE + (AO * ((AWF + 3.6e-1f64).sqrt()));
                    let AYR = ((AW * (ATQ * HP)) / ((ATP * HN) / (((B + (HU * AYP)) + (HV * (AYP * AYP))) + (HW * (HT * ((DV - AYO).abs())))))) * AQT;
                    let AYS = AVZ - (A * ((B + ((A / ((AYR * AYQ) / (AYR + AYQ))).powf(IA))).powf((-1e0f64 / IA))));
                    let AYT = (AO * AYS) + (AO * (((AYS * AYS) + 3.6e-1f64).sqrt()));
                    let AYU = AYT * AYT;
                    let AYV = (AWD * AYT).powf(EV);
                    let AYW = ((AYT + (AVU * (B - ((AWA * ((AYT * AWB) / ((AYU + AWI).sqrt()))).ln())))) - (AWK * AYV)) / ((AYT * (B + (AVU / ((AYT * AWC) / ((AYU + AWJ).sqrt()))))) + (AWM * AYV));
                    let AYX = AYS / AWO;
                    let AYY = if AYX < FB { 1.0 } else { 0.0 };
                    let AZB = if AYY != 0.0 {
                        let AYZ = ((AWO * AWD) * (((BD * AYX) / BH) + (((rspice_limited_exp((AYX / BH))) + (rspice_limited_exp(((-3e0f64 * AYX) / BH)))).ln()))) / ((B / AYW) + ((AWD / EB) * (rspice_limited_exp(((-1e0f64 * AYS) / AWO)))));
                        AYZ
                    } else {
                        let AZA = ((AWO * AWD) * AYX) / ((B / AYW) + ((AWD / EB) * (rspice_limited_exp(((-1e0f64 * AYS) / AWO)))));
                        AZA
                    };
                    let AZC = AYS - (AZB / AWD);
                    let AZD = if ((AZC - AYS).abs()) > FH { 1.0 } else { 0.0 };
                    if AZD != 0.0 {
                        let AZE = AYS - AZC;
                        let AZF = (AO * AZE) + (AO * (((AZE * AZE) + 4e-18f64).sqrt()));
                        let AZG = AWD.powf(EV);
                        let AZH = AZF.powf(EV);
                        let AZI = AZF.powf(-3.333333333333333e-1f64);
                        let AZJ = ARJ * AZG;
                        let AZK = ASC * AZG;
                        let AZL = AZC / AVU;
                        let AZM = AZL - ((AZJ * AZH) / AVU);
                        let AZN = AZL - ((AZK * AZH) / AVU);
                        let AZO = if AZM >= FU { 1.0 } else { 0.0 };
                        let AZV;
                        if AZO != 0.0 {
                            AZV = AZM;
                        } else {
                            let AZP = if AZM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AZW = if AZP != 0.0 {
                                A
                            } else {
                                let AZQ = ((AZM.exp()) + B).ln();
                                AZQ
                            };
                            AZV = AZW;
                        }
                        let AZR = if AZN >= FU { 1.0 } else { 0.0 };
                        let AZX;
                        if AZR != 0.0 {
                            AZX = AZN;
                        } else {
                            let AZS = if AZN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let AZY = if AZS != 0.0 {
                                A
                            } else {
                                let AZT = ((AZN.exp()) + B).ln();
                                AZT
                            };
                            AZX = AZY;
                        }
                        let AZU = EB * AVU;
                        let AZZ = rspice_limited_exp(AZM);
                        let BAA = rspice_limited_exp(AZN);
                        let BAB = AZC - ((((AWD * AZF) - (AZU * AZV)) - (AZU * AZX)) / (((-1e0f64 * AWD) - (((AZZ * EB) * (B + (EV * (AZJ * AZI)))) / (B + AZZ))) - (((BAA * EB) * (B + (EV * (AZK * AZI)))) / (B + BAA))));
                        let BAC = AYS - BAB;
                        let BAD = ((AO * BAC) + (AO * (((BAC * BAC) + 4e-18f64).sqrt()))).powf(EV);
                        let BAE = BAB / AVU;
                        let BAF = BAE - ((AZJ * BAD) / AVU);
                        let BAG = BAE - ((AZK * BAD) / AVU);
                        let BAH = if BAF >= FU { 1.0 } else { 0.0 };
                        if BAH != 0.0 {
                        } else {
                            let BAI = if BAF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BAI != 0.0 {
                            } else {
                            }
                        }
                        let BAJ = if BAG >= FU { 1.0 } else { 0.0 };
                        if BAJ != 0.0 {
                        } else {
                            let BAK = if BAG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BAK != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCR = AVU;
                } else {
                    DCR = DCL;
                }
                DCJ = DCR;
            }
            let DCH;
            if UZ != 0.0 {
                let BAM = if BAL != A { 1.0 } else { 0.0 };
                let DCI;
                if BAM != 0.0 {
                    let BAO = AFT - BAN;
                    let BAP = if BAL == B { 1.0 } else { 0.0 };
                    let BAX;
                    let BAY;
                    if BAP != 0.0 {
                        let BAQ = P - BAN;
                        let BAR = P - AFT;
                        BAX = BAR;
                        BAY = BAQ;
                    } else {
                        let BAS = AC - BAN;
                        let BAT = AC - AFT;
                        BAX = BAT;
                        BAY = BAS;
                    }
                    let BAU = if BAO < A { 1.0 } else { 0.0 };
                    let BAZ;
                    let BBF;
                    let BFT;
                    if BAU != 0.0 {
                        let BAW = BAV * BAO;
                        BAZ = BAW;
                        BBF = BAX;
                        BFT = BAV;
                    } else {
                        BAZ = BAO;
                        BBF = BAY;
                        BFT = B;
                    }
                    let BBA = (((BAZ * BAZ) + C).sqrt()) - Z;
                    let BBB = AI * ((B + AQJ) + (AQK * BBA));
                    let BBC = (AQM + (DP * AQN)) - ((AQO * (BBA * AQP)) / (((BBA * BBA) + (AQP * AQP)).sqrt()));
                    let BBD = CO / AQR;
                    let BBE = BBC + (BBB * (((AQT / ((EC * BBB) * BBB)) * AQU).ln()));
                    let BBG = BBF - BBE;
                    let BBH = ((AO * (BBG + (((BBG * BBG) + EG).sqrt()))) + BBE) - BBC;
                    let BBI = BBD / (5.19105229416e-2f64 * BBB);
                    let BBJ = EJ / BBI;
                    let BBK = B / BBI;
                    let BBL = BBD / EA;
                    let BBM = AO * BBH;
                    let BBN = BBH * BBH;
                    let BBO = BBM + (AO * ((BBN + 3.6e-1f64).sqrt()));
                    let BBP = BBO * BBO;
                    let BBQ = BBJ * BBJ;
                    let BBR = BBK * BBK;
                    let BBS = ARJ / BD;
                    let BBT = (BBL * BBO).powf(EV);
                    let BBU = (AW * ARJ) / BD;
                    let BBV = ((BBO + (BBB * (B - ((BBI * ((BBO * BBJ) / ((BBP + BBQ).sqrt()))).ln())))) - (BBS * BBT)) / ((BBO * (B + (BBB / ((BBO * BBK) / ((BBP + BBR).sqrt()))))) + (BBU * BBT));
                    let BBW = AW * BBB;
                    let BBX = BBH / BBW;
                    let BBY = if BBX < FB { 1.0 } else { 0.0 };
                    let BCB = if BBY != 0.0 {
                        let BBZ = ((BBW * BBL) * (((BD * BBX) / BH) + (((rspice_limited_exp((BBX / BH))) + (rspice_limited_exp(((-3e0f64 * BBX) / BH)))).ln()))) / ((B / BBV) + ((BBL / EB) * (rspice_limited_exp(((-1e0f64 * BBH) / BBW)))));
                        BBZ
                    } else {
                        let BCA = ((BBW * BBL) * BBX) / ((B / BBV) + ((BBL / EB) * (rspice_limited_exp(((-1e0f64 * BBH) / BBW)))));
                        BCA
                    };
                    let BCC = BBH - (BCB / BBL);
                    let BCD = if ((BCC - BBH).abs()) > FH { 1.0 } else { 0.0 };
                    let BDW;
                    if BCD != 0.0 {
                        let BCE = BBH - BCC;
                        let BCF = (AO * BCE) + (AO * (((BCE * BCE) + 4e-18f64).sqrt()));
                        let BCG = BBL.powf(EV);
                        let BCH = BCF.powf(EV);
                        let BCI = BCF.powf(-3.333333333333333e-1f64);
                        let BCJ = ARJ * BCG;
                        let BCK = ASC * BCG;
                        let BCL = BCC / BBB;
                        let BCM = BCL - ((BCJ * BCH) / BBB);
                        let BCN = BCL - ((BCK * BCH) / BBB);
                        let BCO = if BCM >= FU { 1.0 } else { 0.0 };
                        let BCV;
                        if BCO != 0.0 {
                            BCV = BCM;
                        } else {
                            let BCP = if BCM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BCW = if BCP != 0.0 {
                                A
                            } else {
                                let BCQ = ((BCM.exp()) + B).ln();
                                BCQ
                            };
                            BCV = BCW;
                        }
                        let BCR = if BCN >= FU { 1.0 } else { 0.0 };
                        let BCX;
                        if BCR != 0.0 {
                            BCX = BCN;
                        } else {
                            let BCS = if BCN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BCY = if BCS != 0.0 {
                                A
                            } else {
                                let BCT = ((BCN.exp()) + B).ln();
                                BCT
                            };
                            BCX = BCY;
                        }
                        let BCU = EB * BBB;
                        let BCZ = rspice_limited_exp(BCM);
                        let BDA = rspice_limited_exp(BCN);
                        let BDB = BCC - ((((BBL * BCF) - (BCU * BCV)) - (BCU * BCX)) / (((-1e0f64 * BBL) - (((BCZ * EB) * (B + (EV * (BCJ * BCI)))) / (B + BCZ))) - (((BDA * EB) * (B + (EV * (BCK * BCI)))) / (B + BDA))));
                        let BDC = BBH - BDB;
                        let BDD = (AO * BDC) + (AO * (((BDC * BDC) + 4e-18f64).sqrt()));
                        let BDE = BDD.powf(-3.333333333333333e-1f64);
                        let BDF = BDD.powf(EV);
                        let BDG = BDB / BBB;
                        let BDH = BDG - ((BCJ * BDF) / BBB);
                        let BDI = BDG - ((BCK * BDF) / BBB);
                        let BDJ = if BDH >= FU { 1.0 } else { 0.0 };
                        let BDP;
                        if BDJ != 0.0 {
                            BDP = BDH;
                        } else {
                            let BDK = if BDH <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BDQ = if BDK != 0.0 {
                                A
                            } else {
                                let BDL = ((BDH.exp()) + B).ln();
                                BDL
                            };
                            BDP = BDQ;
                        }
                        let BDM = if BDI >= FU { 1.0 } else { 0.0 };
                        let BDR;
                        if BDM != 0.0 {
                            BDR = BDI;
                        } else {
                            let BDN = if BDI <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BDS = if BDN != 0.0 {
                                A
                            } else {
                                let BDO = ((BDI.exp()) + B).ln();
                                BDO
                            };
                            BDR = BDS;
                        }
                        let BDT = rspice_limited_exp(BDH);
                        let BDU = rspice_limited_exp(BDI);
                        let BDV = BDB - ((((BBL * BDD) - (BCU * BDP)) - (BCU * BDR)) / (((-1e0f64 * BBL) - (((BDT * EB) * (B + (EV * (BCJ * BDE)))) / (B + BDT))) - (((BDU * EB) * (B + (EV * (BCK * BDE)))) / (B + BDU))));
                        BDW = BDV;
                    } else {
                        BDW = BCC;
                    }
                    let BDX = (BBD / CO) * ((BBH - BDW).abs());
                    let BDY = BBM + (AO * ((BBN + 3.6e-1f64).sqrt()));
                    let BDZ = ((AW * (ATQ * HP)) / ((ATP * HN) / (((B + (HU * BDX)) + (HV * (BDX * BDX))) + (HW * (HT * ((DV - BDW).abs())))))) * AQT;
                    let BEA = BBH - (BAZ * ((B + ((BAZ / ((BDZ * BDY) / (BDZ + BDY))).powf(IA))).powf((-1e0f64 / IA))));
                    let BEB = (AO * BEA) + (AO * (((BEA * BEA) + 3.6e-1f64).sqrt()));
                    let BEC = BEB * BEB;
                    let BED = (BBL * BEB).powf(EV);
                    let BEE = ((BEB + (BBB * (B - ((BBI * ((BEB * BBJ) / ((BEC + BBQ).sqrt()))).ln())))) - (BBS * BED)) / ((BEB * (B + (BBB / ((BEB * BBK) / ((BEC + BBR).sqrt()))))) + (BBU * BED));
                    let BEF = BEA / BBW;
                    let BEG = if BEF < FB { 1.0 } else { 0.0 };
                    let BEJ = if BEG != 0.0 {
                        let BEH = ((BBW * BBL) * (((BD * BEF) / BH) + (((rspice_limited_exp((BEF / BH))) + (rspice_limited_exp(((-3e0f64 * BEF) / BH)))).ln()))) / ((B / BEE) + ((BBL / EB) * (rspice_limited_exp(((-1e0f64 * BEA) / BBW)))));
                        BEH
                    } else {
                        let BEI = ((BBW * BBL) * BEF) / ((B / BEE) + ((BBL / EB) * (rspice_limited_exp(((-1e0f64 * BEA) / BBW)))));
                        BEI
                    };
                    let BEK = BEA - (BEJ / BBL);
                    let BEL = if ((BEK - BEA).abs()) > FH { 1.0 } else { 0.0 };
                    if BEL != 0.0 {
                        let BEM = BEA - BEK;
                        let BEN = (AO * BEM) + (AO * (((BEM * BEM) + 4e-18f64).sqrt()));
                        let BEO = BBL.powf(EV);
                        let BEP = BEN.powf(EV);
                        let BEQ = BEN.powf(-3.333333333333333e-1f64);
                        let BER = ARJ * BEO;
                        let BES = ASC * BEO;
                        let BET = BEK / BBB;
                        let BEU = BET - ((BER * BEP) / BBB);
                        let BEV = BET - ((BES * BEP) / BBB);
                        let BEW = if BEU >= FU { 1.0 } else { 0.0 };
                        let BFD;
                        if BEW != 0.0 {
                            BFD = BEU;
                        } else {
                            let BEX = if BEU <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BFE = if BEX != 0.0 {
                                A
                            } else {
                                let BEY = ((BEU.exp()) + B).ln();
                                BEY
                            };
                            BFD = BFE;
                        }
                        let BEZ = if BEV >= FU { 1.0 } else { 0.0 };
                        let BFF;
                        if BEZ != 0.0 {
                            BFF = BEV;
                        } else {
                            let BFA = if BEV <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BFG = if BFA != 0.0 {
                                A
                            } else {
                                let BFB = ((BEV.exp()) + B).ln();
                                BFB
                            };
                            BFF = BFG;
                        }
                        let BFC = EB * BBB;
                        let BFH = rspice_limited_exp(BEU);
                        let BFI = rspice_limited_exp(BEV);
                        let BFJ = BEK - ((((BBL * BEN) - (BFC * BFD)) - (BFC * BFF)) / (((-1e0f64 * BBL) - (((BFH * EB) * (B + (EV * (BER * BEQ)))) / (B + BFH))) - (((BFI * EB) * (B + (EV * (BES * BEQ)))) / (B + BFI))));
                        let BFK = BEA - BFJ;
                        let BFL = ((AO * BFK) + (AO * (((BFK * BFK) + 4e-18f64).sqrt()))).powf(EV);
                        let BFM = BFJ / BBB;
                        let BFN = BFM - ((BER * BFL) / BBB);
                        let BFO = BFM - ((BES * BFL) / BBB);
                        let BFP = if BFN >= FU { 1.0 } else { 0.0 };
                        if BFP != 0.0 {
                        } else {
                            let BFQ = if BFN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BFQ != 0.0 {
                            } else {
                            }
                        }
                        let BFR = if BFO >= FU { 1.0 } else { 0.0 };
                        if BFR != 0.0 {
                        } else {
                            let BFS = if BFO <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BFS != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let BFU = if BFT < A { 1.0 } else { 0.0 };
                    if BFU != 0.0 {
                    } else {
                    }
                    DCI = BBB;
                } else {
                    DCI = DCJ;
                }
                DCH = DCI;
            } else {
                let BFV = if BAL != A { 1.0 } else { 0.0 };
                let DCS;
                if BFV != 0.0 {
                    let BFW = if BAL == B { 1.0 } else { 0.0 };
                    let BFY = if BFW != 0.0 {
                        Q
                    } else {
                        let BFX = AC - N;
                        BFX
                    };
                    let BFZ = AI * (B + AQJ);
                    let BGA = AQM + (DP * AQN);
                    let BGB = CO / AQR;
                    let BGC = BGA + (BFZ * (((AQT / ((EC * BFZ) * BFZ)) * AQU).ln()));
                    let BGD = BFY - BGC;
                    let BGE = ((AO * (BGD + (((BGD * BGD) + EG).sqrt()))) + BGC) - BGA;
                    let BGF = BGB / (5.19105229416e-2f64 * BFZ);
                    let BGG = EJ / BGF;
                    let BGH = B / BGF;
                    let BGI = BGB / EA;
                    let BGJ = AO * BGE;
                    let BGK = BGE * BGE;
                    let BGL = BGJ + (AO * ((BGK + 3.6e-1f64).sqrt()));
                    let BGM = BGL * BGL;
                    let BGN = BGG * BGG;
                    let BGO = BGH * BGH;
                    let BGP = ARJ / BD;
                    let BGQ = (BGI * BGL).powf(EV);
                    let BGR = (AW * ARJ) / BD;
                    let BGS = ((BGL + (BFZ * (B - ((BGF * ((BGL * BGG) / ((BGM + BGN).sqrt()))).ln())))) - (BGP * BGQ)) / ((BGL * (B + (BFZ / ((BGL * BGH) / ((BGM + BGO).sqrt()))))) + (BGR * BGQ));
                    let BGT = AW * BFZ;
                    let BGU = BGE / BGT;
                    let BGV = if BGU < FB { 1.0 } else { 0.0 };
                    let BGY = if BGV != 0.0 {
                        let BGW = ((BGT * BGI) * (((BD * BGU) / BH) + (((rspice_limited_exp((BGU / BH))) + (rspice_limited_exp(((-3e0f64 * BGU) / BH)))).ln()))) / ((B / BGS) + ((BGI / EB) * (rspice_limited_exp(((-1e0f64 * BGE) / BGT)))));
                        BGW
                    } else {
                        let BGX = ((BGT * BGI) * BGU) / ((B / BGS) + ((BGI / EB) * (rspice_limited_exp(((-1e0f64 * BGE) / BGT)))));
                        BGX
                    };
                    let BGZ = BGE - (BGY / BGI);
                    let BHA = if ((BGZ - BGE).abs()) > FH { 1.0 } else { 0.0 };
                    let BIT;
                    if BHA != 0.0 {
                        let BHB = BGE - BGZ;
                        let BHC = (AO * BHB) + (AO * (((BHB * BHB) + 4e-18f64).sqrt()));
                        let BHD = BGI.powf(EV);
                        let BHE = BHC.powf(EV);
                        let BHF = BHC.powf(-3.333333333333333e-1f64);
                        let BHG = ARJ * BHD;
                        let BHH = ASC * BHD;
                        let BHI = BGZ / BFZ;
                        let BHJ = BHI - ((BHG * BHE) / BFZ);
                        let BHK = BHI - ((BHH * BHE) / BFZ);
                        let BHL = if BHJ >= FU { 1.0 } else { 0.0 };
                        let BHS;
                        if BHL != 0.0 {
                            BHS = BHJ;
                        } else {
                            let BHM = if BHJ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BHT = if BHM != 0.0 {
                                A
                            } else {
                                let BHN = ((BHJ.exp()) + B).ln();
                                BHN
                            };
                            BHS = BHT;
                        }
                        let BHO = if BHK >= FU { 1.0 } else { 0.0 };
                        let BHU;
                        if BHO != 0.0 {
                            BHU = BHK;
                        } else {
                            let BHP = if BHK <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BHV = if BHP != 0.0 {
                                A
                            } else {
                                let BHQ = ((BHK.exp()) + B).ln();
                                BHQ
                            };
                            BHU = BHV;
                        }
                        let BHR = EB * BFZ;
                        let BHW = rspice_limited_exp(BHJ);
                        let BHX = rspice_limited_exp(BHK);
                        let BHY = BGZ - ((((BGI * BHC) - (BHR * BHS)) - (BHR * BHU)) / (((-1e0f64 * BGI) - (((BHW * EB) * (B + (EV * (BHG * BHF)))) / (B + BHW))) - (((BHX * EB) * (B + (EV * (BHH * BHF)))) / (B + BHX))));
                        let BHZ = BGE - BHY;
                        let BIA = (AO * BHZ) + (AO * (((BHZ * BHZ) + 4e-18f64).sqrt()));
                        let BIB = BIA.powf(-3.333333333333333e-1f64);
                        let BIC = BIA.powf(EV);
                        let BID = BHY / BFZ;
                        let BIE = BID - ((BHG * BIC) / BFZ);
                        let BIF = BID - ((BHH * BIC) / BFZ);
                        let BIG = if BIE >= FU { 1.0 } else { 0.0 };
                        let BIM;
                        if BIG != 0.0 {
                            BIM = BIE;
                        } else {
                            let BIH = if BIE <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BIN = if BIH != 0.0 {
                                A
                            } else {
                                let BII = ((BIE.exp()) + B).ln();
                                BII
                            };
                            BIM = BIN;
                        }
                        let BIJ = if BIF >= FU { 1.0 } else { 0.0 };
                        let BIO;
                        if BIJ != 0.0 {
                            BIO = BIF;
                        } else {
                            let BIK = if BIF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BIP = if BIK != 0.0 {
                                A
                            } else {
                                let BIL = ((BIF.exp()) + B).ln();
                                BIL
                            };
                            BIO = BIP;
                        }
                        let BIQ = rspice_limited_exp(BIE);
                        let BIR = rspice_limited_exp(BIF);
                        let BIS = BHY - ((((BGI * BIA) - (BHR * BIM)) - (BHR * BIO)) / (((-1e0f64 * BGI) - (((BIQ * EB) * (B + (EV * (BHG * BIB)))) / (B + BIQ))) - (((BIR * EB) * (B + (EV * (BHH * BIB)))) / (B + BIR))));
                        BIT = BIS;
                    } else {
                        BIT = BGZ;
                    }
                    let BIU = (BGB / CO) * ((BGE - BIT).abs());
                    let BIV = BGJ + (AO * ((BGK + 3.6e-1f64).sqrt()));
                    let BIW = ((AW * (ATQ * HP)) / ((ATP * HN) / (((B + (HU * BIU)) + (HV * (BIU * BIU))) + (HW * (HT * ((DV - BIT).abs())))))) * AQT;
                    let BIX = BGE - (A * ((B + ((A / ((BIW * BIV) / (BIW + BIV))).powf(IA))).powf((-1e0f64 / IA))));
                    let BIY = (AO * BIX) + (AO * (((BIX * BIX) + 3.6e-1f64).sqrt()));
                    let BIZ = BIY * BIY;
                    let BJA = (BGI * BIY).powf(EV);
                    let BJB = ((BIY + (BFZ * (B - ((BGF * ((BIY * BGG) / ((BIZ + BGN).sqrt()))).ln())))) - (BGP * BJA)) / ((BIY * (B + (BFZ / ((BIY * BGH) / ((BIZ + BGO).sqrt()))))) + (BGR * BJA));
                    let BJC = BIX / BGT;
                    let BJD = if BJC < FB { 1.0 } else { 0.0 };
                    let BJG = if BJD != 0.0 {
                        let BJE = ((BGT * BGI) * (((BD * BJC) / BH) + (((rspice_limited_exp((BJC / BH))) + (rspice_limited_exp(((-3e0f64 * BJC) / BH)))).ln()))) / ((B / BJB) + ((BGI / EB) * (rspice_limited_exp(((-1e0f64 * BIX) / BGT)))));
                        BJE
                    } else {
                        let BJF = ((BGT * BGI) * BJC) / ((B / BJB) + ((BGI / EB) * (rspice_limited_exp(((-1e0f64 * BIX) / BGT)))));
                        BJF
                    };
                    let BJH = BIX - (BJG / BGI);
                    let BJI = if ((BJH - BIX).abs()) > FH { 1.0 } else { 0.0 };
                    if BJI != 0.0 {
                        let BJJ = BIX - BJH;
                        let BJK = (AO * BJJ) + (AO * (((BJJ * BJJ) + 4e-18f64).sqrt()));
                        let BJL = BGI.powf(EV);
                        let BJM = BJK.powf(EV);
                        let BJN = BJK.powf(-3.333333333333333e-1f64);
                        let BJO = ARJ * BJL;
                        let BJP = ASC * BJL;
                        let BJQ = BJH / BFZ;
                        let BJR = BJQ - ((BJO * BJM) / BFZ);
                        let BJS = BJQ - ((BJP * BJM) / BFZ);
                        let BJT = if BJR >= FU { 1.0 } else { 0.0 };
                        let BKA;
                        if BJT != 0.0 {
                            BKA = BJR;
                        } else {
                            let BJU = if BJR <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BKB = if BJU != 0.0 {
                                A
                            } else {
                                let BJV = ((BJR.exp()) + B).ln();
                                BJV
                            };
                            BKA = BKB;
                        }
                        let BJW = if BJS >= FU { 1.0 } else { 0.0 };
                        let BKC;
                        if BJW != 0.0 {
                            BKC = BJS;
                        } else {
                            let BJX = if BJS <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BKD = if BJX != 0.0 {
                                A
                            } else {
                                let BJY = ((BJS.exp()) + B).ln();
                                BJY
                            };
                            BKC = BKD;
                        }
                        let BJZ = EB * BFZ;
                        let BKE = rspice_limited_exp(BJR);
                        let BKF = rspice_limited_exp(BJS);
                        let BKG = BJH - ((((BGI * BJK) - (BJZ * BKA)) - (BJZ * BKC)) / (((-1e0f64 * BGI) - (((BKE * EB) * (B + (EV * (BJO * BJN)))) / (B + BKE))) - (((BKF * EB) * (B + (EV * (BJP * BJN)))) / (B + BKF))));
                        let BKH = BIX - BKG;
                        let BKI = ((AO * BKH) + (AO * (((BKH * BKH) + 4e-18f64).sqrt()))).powf(EV);
                        let BKJ = BKG / BFZ;
                        let BKK = BKJ - ((BJO * BKI) / BFZ);
                        let BKL = BKJ - ((BJP * BKI) / BFZ);
                        let BKM = if BKK >= FU { 1.0 } else { 0.0 };
                        if BKM != 0.0 {
                        } else {
                            let BKN = if BKK <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BKN != 0.0 {
                            } else {
                            }
                        }
                        let BKO = if BKL >= FU { 1.0 } else { 0.0 };
                        if BKO != 0.0 {
                        } else {
                            let BKP = if BKL <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BKP != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCS = BFZ;
                } else {
                    DCS = DCJ;
                }
                DCH = DCS;
            }
            let DCF;
            if UZ != 0.0 {
                let BKR = if BKQ != A { 1.0 } else { 0.0 };
                let DCG;
                if BKR != 0.0 {
                    let BKT = BKS - APV;
                    let BKU = if BKQ == B { 1.0 } else { 0.0 };
                    let BLC;
                    let BLD;
                    if BKU != 0.0 {
                        let BKV = P - APV;
                        let BKW = P - BKS;
                        BLC = BKW;
                        BLD = BKV;
                    } else {
                        let BKX = AC - APV;
                        let BKY = AC - BKS;
                        BLC = BKY;
                        BLD = BKX;
                    }
                    let BKZ = if BKT < A { 1.0 } else { 0.0 };
                    let BLE;
                    let BLT;
                    let BQL;
                    if BKZ != 0.0 {
                        let BLB = BLA * BKT;
                        BLE = BLB;
                        BLT = BLC;
                        BQL = BLA;
                    } else {
                        BLE = BKT;
                        BLT = BLD;
                        BQL = B;
                    }
                    let BLF = (((BLE * BLE) + C).sqrt()) - Z;
                    let BLI = AI * ((B + BLG) + (BLH * BLF));
                    let BLN = (BLJ - (DP * BLK)) - ((BLL * (BLF * BLM)) / (((BLF * BLF) + (BLM * BLM)).sqrt()));
                    let BLP = CO / BLO;
                    let BLS = BLN + (BLI * (((BLQ / ((EC * BLI) * BLI)) * BLR).ln()));
                    let BLU = BLT - BLS;
                    let BLV = ((AO * (BLU + (((BLU * BLU) + EG).sqrt()))) + BLS) - BLN;
                    let BLW = BLP / (5.19105229416e-2f64 * BLI);
                    let BLX = EJ / BLW;
                    let BLY = B / BLW;
                    let BLZ = BLP / EA;
                    let BMA = AO * BLV;
                    let BMB = BLV * BLV;
                    let BMC = BMA + (AO * ((BMB + 3.6e-1f64).sqrt()));
                    let BMD = BMC * BMC;
                    let BME = BLX * BLX;
                    let BMF = BLY * BLY;
                    let BMH = BMG / BD;
                    let BMI = (BLZ * BMC).powf(EV);
                    let BMJ = (AW * BMG) / BD;
                    let BMK = ((BMC + (BLI * (B - ((BLW * ((BMC * BLX) / ((BMD + BME).sqrt()))).ln())))) - (BMH * BMI)) / ((BMC * (B + (BLI / ((BMC * BLY) / ((BMD + BMF).sqrt()))))) + (BMJ * BMI));
                    let BML = AW * BLI;
                    let BMM = BLV / BML;
                    let BMN = if BMM < FB { 1.0 } else { 0.0 };
                    let BMQ = if BMN != 0.0 {
                        let BMO = ((BML * BLZ) * (((BD * BMM) / BH) + (((rspice_limited_exp((BMM / BH))) + (rspice_limited_exp(((-3e0f64 * BMM) / BH)))).ln()))) / ((B / BMK) + ((BLZ / EB) * (rspice_limited_exp(((-1e0f64 * BLV) / BML)))));
                        BMO
                    } else {
                        let BMP = ((BML * BLZ) * BMM) / ((B / BMK) + ((BLZ / EB) * (rspice_limited_exp(((-1e0f64 * BLV) / BML)))));
                        BMP
                    };
                    let BMR = BLV - (BMQ / BLZ);
                    let BMS = if ((BMR - BLV).abs()) > FH { 1.0 } else { 0.0 };
                    let BOO;
                    if BMS != 0.0 {
                        let BMT = BLV - BMR;
                        let BMU = (AO * BMT) + (AO * (((BMT * BMT) + 4e-18f64).sqrt()));
                        let BMV = BLZ.powf(EV);
                        let BMW = BMU.powf(EV);
                        let BMX = BMU.powf(-3.333333333333333e-1f64);
                        let BMY = BMG * BMV;
                        let BNA = BMZ * BMV;
                        let BNB = BMR / BLI;
                        let BNC = BNB - ((BMY * BMW) / BLI);
                        let BND = BNB - ((BNA * BMW) / BLI);
                        let BNE = if BNC >= FU { 1.0 } else { 0.0 };
                        let BNL;
                        if BNE != 0.0 {
                            BNL = BNC;
                        } else {
                            let BNF = if BNC <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BNM = if BNF != 0.0 {
                                A
                            } else {
                                let BNG = ((BNC.exp()) + B).ln();
                                BNG
                            };
                            BNL = BNM;
                        }
                        let BNH = if BND >= FU { 1.0 } else { 0.0 };
                        let BNN;
                        if BNH != 0.0 {
                            BNN = BND;
                        } else {
                            let BNI = if BND <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BNO = if BNI != 0.0 {
                                A
                            } else {
                                let BNJ = ((BND.exp()) + B).ln();
                                BNJ
                            };
                            BNN = BNO;
                        }
                        let BNK = EB * BLI;
                        let BNP = rspice_limited_exp(BNC);
                        let BNQ = rspice_limited_exp(BND);
                        let BNR = BMR - ((((BLZ * BMU) - (BNK * BNL)) - (BNK * BNN)) / (((-1e0f64 * BLZ) - (((BNP * EB) * (B + (EV * (BMY * BMX)))) / (B + BNP))) - (((BNQ * EB) * (B + (EV * (BNA * BMX)))) / (B + BNQ))));
                        let BNS = BLV - BNR;
                        let BNT = (AO * BNS) + (AO * (((BNS * BNS) + 4e-18f64).sqrt()));
                        let BNU = BNT.powf(-3.333333333333333e-1f64);
                        let BNV = BNT.powf(EV);
                        let BNW = BNR / BLI;
                        let BNX = BNW - ((BMY * BNV) / BLI);
                        let BNY = BNW - ((BNA * BNV) / BLI);
                        let BNZ = if BNX >= FU { 1.0 } else { 0.0 };
                        let BOF;
                        if BNZ != 0.0 {
                            BOF = BNX;
                        } else {
                            let BOA = if BNX <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BOG = if BOA != 0.0 {
                                A
                            } else {
                                let BOB = ((BNX.exp()) + B).ln();
                                BOB
                            };
                            BOF = BOG;
                        }
                        let BOC = if BNY >= FU { 1.0 } else { 0.0 };
                        let BOH;
                        if BOC != 0.0 {
                            BOH = BNY;
                        } else {
                            let BOD = if BNY <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BOI = if BOD != 0.0 {
                                A
                            } else {
                                let BOE = ((BNY.exp()) + B).ln();
                                BOE
                            };
                            BOH = BOI;
                        }
                        let BOJ = rspice_limited_exp(BNX);
                        let BOK = rspice_limited_exp(BNY);
                        let BOL = BNR - ((((BLZ * BNT) - (BNK * BOF)) - (BNK * BOH)) / (((-1e0f64 * BLZ) - (((BOJ * EB) * (B + (EV * (BMY * BNU)))) / (B + BOJ))) - (((BOK * EB) * (B + (EV * (BNA * BNU)))) / (B + BOK))));
                        BOO = BOL;
                    } else {
                        BOO = BMR;
                    }
                    let BOP = (BLP / CO) * ((BLV - BOO).abs());
                    let BOQ = BMA + (AO * ((BMB + 3.6e-1f64).sqrt()));
                    let BOR = ((AW * (BON * HP)) / ((BOM * HN) / (((B + (HU * BOP)) + (HV * (BOP * BOP))) + (HW * (HT * ((DV - BOO).abs())))))) * BLQ;
                    let BOS = BLV - (BLE * ((B + ((BLE / ((BOR * BOQ) / (BOR + BOQ))).powf(IA))).powf((-1e0f64 / IA))));
                    let BOT = (AO * BOS) + (AO * (((BOS * BOS) + 3.6e-1f64).sqrt()));
                    let BOU = BOT * BOT;
                    let BOV = (BLZ * BOT).powf(EV);
                    let BOW = ((BOT + (BLI * (B - ((BLW * ((BOT * BLX) / ((BOU + BME).sqrt()))).ln())))) - (BMH * BOV)) / ((BOT * (B + (BLI / ((BOT * BLY) / ((BOU + BMF).sqrt()))))) + (BMJ * BOV));
                    let BOX = BOS / BML;
                    let BOY = if BOX < FB { 1.0 } else { 0.0 };
                    let BPB = if BOY != 0.0 {
                        let BOZ = ((BML * BLZ) * (((BD * BOX) / BH) + (((rspice_limited_exp((BOX / BH))) + (rspice_limited_exp(((-3e0f64 * BOX) / BH)))).ln()))) / ((B / BOW) + ((BLZ / EB) * (rspice_limited_exp(((-1e0f64 * BOS) / BML)))));
                        BOZ
                    } else {
                        let BPA = ((BML * BLZ) * BOX) / ((B / BOW) + ((BLZ / EB) * (rspice_limited_exp(((-1e0f64 * BOS) / BML)))));
                        BPA
                    };
                    let BPC = BOS - (BPB / BLZ);
                    let BPD = if ((BPC - BOS).abs()) > FH { 1.0 } else { 0.0 };
                    if BPD != 0.0 {
                        let BPE = BOS - BPC;
                        let BPF = (AO * BPE) + (AO * (((BPE * BPE) + 4e-18f64).sqrt()));
                        let BPG = BLZ.powf(EV);
                        let BPH = BPF.powf(EV);
                        let BPI = BPF.powf(-3.333333333333333e-1f64);
                        let BPJ = BMG * BPG;
                        let BPK = BMZ * BPG;
                        let BPL = BPC / BLI;
                        let BPM = BPL - ((BPJ * BPH) / BLI);
                        let BPN = BPL - ((BPK * BPH) / BLI);
                        let BPO = if BPM >= FU { 1.0 } else { 0.0 };
                        let BPV;
                        if BPO != 0.0 {
                            BPV = BPM;
                        } else {
                            let BPP = if BPM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BPW = if BPP != 0.0 {
                                A
                            } else {
                                let BPQ = ((BPM.exp()) + B).ln();
                                BPQ
                            };
                            BPV = BPW;
                        }
                        let BPR = if BPN >= FU { 1.0 } else { 0.0 };
                        let BPX;
                        if BPR != 0.0 {
                            BPX = BPN;
                        } else {
                            let BPS = if BPN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BPY = if BPS != 0.0 {
                                A
                            } else {
                                let BPT = ((BPN.exp()) + B).ln();
                                BPT
                            };
                            BPX = BPY;
                        }
                        let BPU = EB * BLI;
                        let BPZ = rspice_limited_exp(BPM);
                        let BQA = rspice_limited_exp(BPN);
                        let BQB = BPC - ((((BLZ * BPF) - (BPU * BPV)) - (BPU * BPX)) / (((-1e0f64 * BLZ) - (((BPZ * EB) * (B + (EV * (BPJ * BPI)))) / (B + BPZ))) - (((BQA * EB) * (B + (EV * (BPK * BPI)))) / (B + BQA))));
                        let BQC = BOS - BQB;
                        let BQD = ((AO * BQC) + (AO * (((BQC * BQC) + 4e-18f64).sqrt()))).powf(EV);
                        let BQE = BQB / BLI;
                        let BQF = BQE - ((BPJ * BQD) / BLI);
                        let BQG = BQE - ((BPK * BQD) / BLI);
                        let BQH = if BQF >= FU { 1.0 } else { 0.0 };
                        if BQH != 0.0 {
                        } else {
                            let BQI = if BQF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BQI != 0.0 {
                            } else {
                            }
                        }
                        let BQJ = if BQG >= FU { 1.0 } else { 0.0 };
                        if BQJ != 0.0 {
                        } else {
                            let BQK = if BQG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BQK != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let BQM = if BQL < A { 1.0 } else { 0.0 };
                    if BQM != 0.0 {
                    } else {
                    }
                    DCG = BLI;
                } else {
                    DCG = DCH;
                }
                DCF = DCG;
            } else {
                let BQN = if BKQ != A { 1.0 } else { 0.0 };
                let DCT;
                if BQN != 0.0 {
                    let BQO = if BKQ == B { 1.0 } else { 0.0 };
                    let BQQ = if BQO != 0.0 {
                        R
                    } else {
                        let BQP = AC - M;
                        BQP
                    };
                    let BQR = AI * (B + BLG);
                    let BQS = BLJ - (DP * BLK);
                    let BQT = CO / BLO;
                    let BQU = BQS + (BQR * (((BLQ / ((EC * BQR) * BQR)) * BLR).ln()));
                    let BQV = BQQ - BQU;
                    let BQW = ((AO * (BQV + (((BQV * BQV) + EG).sqrt()))) + BQU) - BQS;
                    let BQX = BQT / (5.19105229416e-2f64 * BQR);
                    let BQY = EJ / BQX;
                    let BQZ = B / BQX;
                    let BRA = BQT / EA;
                    let BRB = AO * BQW;
                    let BRC = BQW * BQW;
                    let BRD = BRB + (AO * ((BRC + 3.6e-1f64).sqrt()));
                    let BRE = BRD * BRD;
                    let BRF = BQY * BQY;
                    let BRG = BQZ * BQZ;
                    let BRH = BMG / BD;
                    let BRI = (BRA * BRD).powf(EV);
                    let BRJ = (AW * BMG) / BD;
                    let BRK = ((BRD + (BQR * (B - ((BQX * ((BRD * BQY) / ((BRE + BRF).sqrt()))).ln())))) - (BRH * BRI)) / ((BRD * (B + (BQR / ((BRD * BQZ) / ((BRE + BRG).sqrt()))))) + (BRJ * BRI));
                    let BRL = AW * BQR;
                    let BRM = BQW / BRL;
                    let BRN = if BRM < FB { 1.0 } else { 0.0 };
                    let BRQ = if BRN != 0.0 {
                        let BRO = ((BRL * BRA) * (((BD * BRM) / BH) + (((rspice_limited_exp((BRM / BH))) + (rspice_limited_exp(((-3e0f64 * BRM) / BH)))).ln()))) / ((B / BRK) + ((BRA / EB) * (rspice_limited_exp(((-1e0f64 * BQW) / BRL)))));
                        BRO
                    } else {
                        let BRP = ((BRL * BRA) * BRM) / ((B / BRK) + ((BRA / EB) * (rspice_limited_exp(((-1e0f64 * BQW) / BRL)))));
                        BRP
                    };
                    let BRR = BQW - (BRQ / BRA);
                    let BRS = if ((BRR - BQW).abs()) > FH { 1.0 } else { 0.0 };
                    let BTL;
                    if BRS != 0.0 {
                        let BRT = BQW - BRR;
                        let BRU = (AO * BRT) + (AO * (((BRT * BRT) + 4e-18f64).sqrt()));
                        let BRV = BRA.powf(EV);
                        let BRW = BRU.powf(EV);
                        let BRX = BRU.powf(-3.333333333333333e-1f64);
                        let BRY = BMG * BRV;
                        let BRZ = BMZ * BRV;
                        let BSA = BRR / BQR;
                        let BSB = BSA - ((BRY * BRW) / BQR);
                        let BSC = BSA - ((BRZ * BRW) / BQR);
                        let BSD = if BSB >= FU { 1.0 } else { 0.0 };
                        let BSK;
                        if BSD != 0.0 {
                            BSK = BSB;
                        } else {
                            let BSE = if BSB <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BSL = if BSE != 0.0 {
                                A
                            } else {
                                let BSF = ((BSB.exp()) + B).ln();
                                BSF
                            };
                            BSK = BSL;
                        }
                        let BSG = if BSC >= FU { 1.0 } else { 0.0 };
                        let BSM;
                        if BSG != 0.0 {
                            BSM = BSC;
                        } else {
                            let BSH = if BSC <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BSN = if BSH != 0.0 {
                                A
                            } else {
                                let BSI = ((BSC.exp()) + B).ln();
                                BSI
                            };
                            BSM = BSN;
                        }
                        let BSJ = EB * BQR;
                        let BSO = rspice_limited_exp(BSB);
                        let BSP = rspice_limited_exp(BSC);
                        let BSQ = BRR - ((((BRA * BRU) - (BSJ * BSK)) - (BSJ * BSM)) / (((-1e0f64 * BRA) - (((BSO * EB) * (B + (EV * (BRY * BRX)))) / (B + BSO))) - (((BSP * EB) * (B + (EV * (BRZ * BRX)))) / (B + BSP))));
                        let BSR = BQW - BSQ;
                        let BSS = (AO * BSR) + (AO * (((BSR * BSR) + 4e-18f64).sqrt()));
                        let BST = BSS.powf(-3.333333333333333e-1f64);
                        let BSU = BSS.powf(EV);
                        let BSV = BSQ / BQR;
                        let BSW = BSV - ((BRY * BSU) / BQR);
                        let BSX = BSV - ((BRZ * BSU) / BQR);
                        let BSY = if BSW >= FU { 1.0 } else { 0.0 };
                        let BTE;
                        if BSY != 0.0 {
                            BTE = BSW;
                        } else {
                            let BSZ = if BSW <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BTF = if BSZ != 0.0 {
                                A
                            } else {
                                let BTA = ((BSW.exp()) + B).ln();
                                BTA
                            };
                            BTE = BTF;
                        }
                        let BTB = if BSX >= FU { 1.0 } else { 0.0 };
                        let BTG;
                        if BTB != 0.0 {
                            BTG = BSX;
                        } else {
                            let BTC = if BSX <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BTH = if BTC != 0.0 {
                                A
                            } else {
                                let BTD = ((BSX.exp()) + B).ln();
                                BTD
                            };
                            BTG = BTH;
                        }
                        let BTI = rspice_limited_exp(BSW);
                        let BTJ = rspice_limited_exp(BSX);
                        let BTK = BSQ - ((((BRA * BSS) - (BSJ * BTE)) - (BSJ * BTG)) / (((-1e0f64 * BRA) - (((BTI * EB) * (B + (EV * (BRY * BST)))) / (B + BTI))) - (((BTJ * EB) * (B + (EV * (BRZ * BST)))) / (B + BTJ))));
                        BTL = BTK;
                    } else {
                        BTL = BRR;
                    }
                    let BTM = (BQT / CO) * ((BQW - BTL).abs());
                    let BTN = BRB + (AO * ((BRC + 3.6e-1f64).sqrt()));
                    let BTO = ((AW * (BON * HP)) / ((BOM * HN) / (((B + (HU * BTM)) + (HV * (BTM * BTM))) + (HW * (HT * ((DV - BTL).abs())))))) * BLQ;
                    let BTP = BQW - (A * ((B + ((A / ((BTO * BTN) / (BTO + BTN))).powf(IA))).powf((-1e0f64 / IA))));
                    let BTQ = (AO * BTP) + (AO * (((BTP * BTP) + 3.6e-1f64).sqrt()));
                    let BTR = BTQ * BTQ;
                    let BTS = (BRA * BTQ).powf(EV);
                    let BTT = ((BTQ + (BQR * (B - ((BQX * ((BTQ * BQY) / ((BTR + BRF).sqrt()))).ln())))) - (BRH * BTS)) / ((BTQ * (B + (BQR / ((BTQ * BQZ) / ((BTR + BRG).sqrt()))))) + (BRJ * BTS));
                    let BTU = BTP / BRL;
                    let BTV = if BTU < FB { 1.0 } else { 0.0 };
                    let BTY = if BTV != 0.0 {
                        let BTW = ((BRL * BRA) * (((BD * BTU) / BH) + (((rspice_limited_exp((BTU / BH))) + (rspice_limited_exp(((-3e0f64 * BTU) / BH)))).ln()))) / ((B / BTT) + ((BRA / EB) * (rspice_limited_exp(((-1e0f64 * BTP) / BRL)))));
                        BTW
                    } else {
                        let BTX = ((BRL * BRA) * BTU) / ((B / BTT) + ((BRA / EB) * (rspice_limited_exp(((-1e0f64 * BTP) / BRL)))));
                        BTX
                    };
                    let BTZ = BTP - (BTY / BRA);
                    let BUA = if ((BTZ - BTP).abs()) > FH { 1.0 } else { 0.0 };
                    if BUA != 0.0 {
                        let BUB = BTP - BTZ;
                        let BUC = (AO * BUB) + (AO * (((BUB * BUB) + 4e-18f64).sqrt()));
                        let BUD = BRA.powf(EV);
                        let BUE = BUC.powf(EV);
                        let BUF = BUC.powf(-3.333333333333333e-1f64);
                        let BUG = BMG * BUD;
                        let BUH = BMZ * BUD;
                        let BUI = BTZ / BQR;
                        let BUJ = BUI - ((BUG * BUE) / BQR);
                        let BUK = BUI - ((BUH * BUE) / BQR);
                        let BUL = if BUJ >= FU { 1.0 } else { 0.0 };
                        let BUS;
                        if BUL != 0.0 {
                            BUS = BUJ;
                        } else {
                            let BUM = if BUJ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BUT = if BUM != 0.0 {
                                A
                            } else {
                                let BUN = ((BUJ.exp()) + B).ln();
                                BUN
                            };
                            BUS = BUT;
                        }
                        let BUO = if BUK >= FU { 1.0 } else { 0.0 };
                        let BUU;
                        if BUO != 0.0 {
                            BUU = BUK;
                        } else {
                            let BUP = if BUK <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BUV = if BUP != 0.0 {
                                A
                            } else {
                                let BUQ = ((BUK.exp()) + B).ln();
                                BUQ
                            };
                            BUU = BUV;
                        }
                        let BUR = EB * BQR;
                        let BUW = rspice_limited_exp(BUJ);
                        let BUX = rspice_limited_exp(BUK);
                        let BUY = BTZ - ((((BRA * BUC) - (BUR * BUS)) - (BUR * BUU)) / (((-1e0f64 * BRA) - (((BUW * EB) * (B + (EV * (BUG * BUF)))) / (B + BUW))) - (((BUX * EB) * (B + (EV * (BUH * BUF)))) / (B + BUX))));
                        let BUZ = BTP - BUY;
                        let BVA = ((AO * BUZ) + (AO * (((BUZ * BUZ) + 4e-18f64).sqrt()))).powf(EV);
                        let BVB = BUY / BQR;
                        let BVC = BVB - ((BUG * BVA) / BQR);
                        let BVD = BVB - ((BUH * BVA) / BQR);
                        let BVE = if BVC >= FU { 1.0 } else { 0.0 };
                        if BVE != 0.0 {
                        } else {
                            let BVF = if BVC <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BVF != 0.0 {
                            } else {
                            }
                        }
                        let BVG = if BVD >= FU { 1.0 } else { 0.0 };
                        if BVG != 0.0 {
                        } else {
                            let BVH = if BVD <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if BVH != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCT = BQR;
                } else {
                    DCT = DCH;
                }
                DCF = DCT;
            }
            let DCD;
            if UZ != 0.0 {
                let BVJ = if BVI != A { 1.0 } else { 0.0 };
                let DCE;
                if BVJ != 0.0 {
                    let BVL = BAN - BVK;
                    let BVM = if BVI == B { 1.0 } else { 0.0 };
                    let BVU;
                    let BVV;
                    if BVM != 0.0 {
                        let BVN = P - BVK;
                        let BVO = P - BAN;
                        BVU = BVO;
                        BVV = BVN;
                    } else {
                        let BVP = AC - BVK;
                        let BVQ = AC - BAN;
                        BVU = BVQ;
                        BVV = BVP;
                    }
                    let BVR = if BVL < A { 1.0 } else { 0.0 };
                    let BVW;
                    let BWC;
                    let CAQ;
                    if BVR != 0.0 {
                        let BVT = BVS * BVL;
                        BVW = BVT;
                        BWC = BVU;
                        CAQ = BVS;
                    } else {
                        BVW = BVL;
                        BWC = BVV;
                        CAQ = B;
                    }
                    let BVX = (((BVW * BVW) + C).sqrt()) - Z;
                    let BVY = AI * ((B + BLG) + (BLH * BVX));
                    let BVZ = (BLJ + (DP * BLK)) - ((BLL * (BVX * BLM)) / (((BVX * BVX) + (BLM * BLM)).sqrt()));
                    let BWA = CO / BLO;
                    let BWB = BVZ + (BVY * (((BLQ / ((EC * BVY) * BVY)) * BLR).ln()));
                    let BWD = BWC - BWB;
                    let BWE = ((AO * (BWD + (((BWD * BWD) + EG).sqrt()))) + BWB) - BVZ;
                    let BWF = BWA / (5.19105229416e-2f64 * BVY);
                    let BWG = EJ / BWF;
                    let BWH = B / BWF;
                    let BWI = BWA / EA;
                    let BWJ = AO * BWE;
                    let BWK = BWE * BWE;
                    let BWL = BWJ + (AO * ((BWK + 3.6e-1f64).sqrt()));
                    let BWM = BWL * BWL;
                    let BWN = BWG * BWG;
                    let BWO = BWH * BWH;
                    let BWP = BMG / BD;
                    let BWQ = (BWI * BWL).powf(EV);
                    let BWR = (AW * BMG) / BD;
                    let BWS = ((BWL + (BVY * (B - ((BWF * ((BWL * BWG) / ((BWM + BWN).sqrt()))).ln())))) - (BWP * BWQ)) / ((BWL * (B + (BVY / ((BWL * BWH) / ((BWM + BWO).sqrt()))))) + (BWR * BWQ));
                    let BWT = AW * BVY;
                    let BWU = BWE / BWT;
                    let BWV = if BWU < FB { 1.0 } else { 0.0 };
                    let BWY = if BWV != 0.0 {
                        let BWW = ((BWT * BWI) * (((BD * BWU) / BH) + (((rspice_limited_exp((BWU / BH))) + (rspice_limited_exp(((-3e0f64 * BWU) / BH)))).ln()))) / ((B / BWS) + ((BWI / EB) * (rspice_limited_exp(((-1e0f64 * BWE) / BWT)))));
                        BWW
                    } else {
                        let BWX = ((BWT * BWI) * BWU) / ((B / BWS) + ((BWI / EB) * (rspice_limited_exp(((-1e0f64 * BWE) / BWT)))));
                        BWX
                    };
                    let BWZ = BWE - (BWY / BWI);
                    let BXA = if ((BWZ - BWE).abs()) > FH { 1.0 } else { 0.0 };
                    let BYT;
                    if BXA != 0.0 {
                        let BXB = BWE - BWZ;
                        let BXC = (AO * BXB) + (AO * (((BXB * BXB) + 4e-18f64).sqrt()));
                        let BXD = BWI.powf(EV);
                        let BXE = BXC.powf(EV);
                        let BXF = BXC.powf(-3.333333333333333e-1f64);
                        let BXG = BMG * BXD;
                        let BXH = BMZ * BXD;
                        let BXI = BWZ / BVY;
                        let BXJ = BXI - ((BXG * BXE) / BVY);
                        let BXK = BXI - ((BXH * BXE) / BVY);
                        let BXL = if BXJ >= FU { 1.0 } else { 0.0 };
                        let BXS;
                        if BXL != 0.0 {
                            BXS = BXJ;
                        } else {
                            let BXM = if BXJ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BXT = if BXM != 0.0 {
                                A
                            } else {
                                let BXN = ((BXJ.exp()) + B).ln();
                                BXN
                            };
                            BXS = BXT;
                        }
                        let BXO = if BXK >= FU { 1.0 } else { 0.0 };
                        let BXU;
                        if BXO != 0.0 {
                            BXU = BXK;
                        } else {
                            let BXP = if BXK <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BXV = if BXP != 0.0 {
                                A
                            } else {
                                let BXQ = ((BXK.exp()) + B).ln();
                                BXQ
                            };
                            BXU = BXV;
                        }
                        let BXR = EB * BVY;
                        let BXW = rspice_limited_exp(BXJ);
                        let BXX = rspice_limited_exp(BXK);
                        let BXY = BWZ - ((((BWI * BXC) - (BXR * BXS)) - (BXR * BXU)) / (((-1e0f64 * BWI) - (((BXW * EB) * (B + (EV * (BXG * BXF)))) / (B + BXW))) - (((BXX * EB) * (B + (EV * (BXH * BXF)))) / (B + BXX))));
                        let BXZ = BWE - BXY;
                        let BYA = (AO * BXZ) + (AO * (((BXZ * BXZ) + 4e-18f64).sqrt()));
                        let BYB = BYA.powf(-3.333333333333333e-1f64);
                        let BYC = BYA.powf(EV);
                        let BYD = BXY / BVY;
                        let BYE = BYD - ((BXG * BYC) / BVY);
                        let BYF = BYD - ((BXH * BYC) / BVY);
                        let BYG = if BYE >= FU { 1.0 } else { 0.0 };
                        let BYM;
                        if BYG != 0.0 {
                            BYM = BYE;
                        } else {
                            let BYH = if BYE <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BYN = if BYH != 0.0 {
                                A
                            } else {
                                let BYI = ((BYE.exp()) + B).ln();
                                BYI
                            };
                            BYM = BYN;
                        }
                        let BYJ = if BYF >= FU { 1.0 } else { 0.0 };
                        let BYO;
                        if BYJ != 0.0 {
                            BYO = BYF;
                        } else {
                            let BYK = if BYF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let BYP = if BYK != 0.0 {
                                A
                            } else {
                                let BYL = ((BYF.exp()) + B).ln();
                                BYL
                            };
                            BYO = BYP;
                        }
                        let BYQ = rspice_limited_exp(BYE);
                        let BYR = rspice_limited_exp(BYF);
                        let BYS = BXY - ((((BWI * BYA) - (BXR * BYM)) - (BXR * BYO)) / (((-1e0f64 * BWI) - (((BYQ * EB) * (B + (EV * (BXG * BYB)))) / (B + BYQ))) - (((BYR * EB) * (B + (EV * (BXH * BYB)))) / (B + BYR))));
                        BYT = BYS;
                    } else {
                        BYT = BWZ;
                    }
                    let BYU = (BWA / CO) * ((BWE - BYT).abs());
                    let BYV = BWJ + (AO * ((BWK + 3.6e-1f64).sqrt()));
                    let BYW = ((AW * (BON * HP)) / ((BOM * HN) / (((B + (HU * BYU)) + (HV * (BYU * BYU))) + (HW * (HT * ((DV - BYT).abs())))))) * BLQ;
                    let BYX = BWE - (BVW * ((B + ((BVW / ((BYW * BYV) / (BYW + BYV))).powf(IA))).powf((-1e0f64 / IA))));
                    let BYY = (AO * BYX) + (AO * (((BYX * BYX) + 3.6e-1f64).sqrt()));
                    let BYZ = BYY * BYY;
                    let BZA = (BWI * BYY).powf(EV);
                    let BZB = ((BYY + (BVY * (B - ((BWF * ((BYY * BWG) / ((BYZ + BWN).sqrt()))).ln())))) - (BWP * BZA)) / ((BYY * (B + (BVY / ((BYY * BWH) / ((BYZ + BWO).sqrt()))))) + (BWR * BZA));
                    let BZC = BYX / BWT;
                    let BZD = if BZC < FB { 1.0 } else { 0.0 };
                    let BZG = if BZD != 0.0 {
                        let BZE = ((BWT * BWI) * (((BD * BZC) / BH) + (((rspice_limited_exp((BZC / BH))) + (rspice_limited_exp(((-3e0f64 * BZC) / BH)))).ln()))) / ((B / BZB) + ((BWI / EB) * (rspice_limited_exp(((-1e0f64 * BYX) / BWT)))));
                        BZE
                    } else {
                        let BZF = ((BWT * BWI) * BZC) / ((B / BZB) + ((BWI / EB) * (rspice_limited_exp(((-1e0f64 * BYX) / BWT)))));
                        BZF
                    };
                    let BZH = BYX - (BZG / BWI);
                    let BZI = if ((BZH - BYX).abs()) > FH { 1.0 } else { 0.0 };
                    if BZI != 0.0 {
                        let BZJ = BYX - BZH;
                        let BZK = (AO * BZJ) + (AO * (((BZJ * BZJ) + 4e-18f64).sqrt()));
                        let BZL = BWI.powf(EV);
                        let BZM = BZK.powf(EV);
                        let BZN = BZK.powf(-3.333333333333333e-1f64);
                        let BZO = BMG * BZL;
                        let BZP = BMZ * BZL;
                        let BZQ = BZH / BVY;
                        let BZR = BZQ - ((BZO * BZM) / BVY);
                        let BZS = BZQ - ((BZP * BZM) / BVY);
                        let BZT = if BZR >= FU { 1.0 } else { 0.0 };
                        let CAA;
                        if BZT != 0.0 {
                            CAA = BZR;
                        } else {
                            let BZU = if BZR <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CAB = if BZU != 0.0 {
                                A
                            } else {
                                let BZV = ((BZR.exp()) + B).ln();
                                BZV
                            };
                            CAA = CAB;
                        }
                        let BZW = if BZS >= FU { 1.0 } else { 0.0 };
                        let CAC;
                        if BZW != 0.0 {
                            CAC = BZS;
                        } else {
                            let BZX = if BZS <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CAD = if BZX != 0.0 {
                                A
                            } else {
                                let BZY = ((BZS.exp()) + B).ln();
                                BZY
                            };
                            CAC = CAD;
                        }
                        let BZZ = EB * BVY;
                        let CAE = rspice_limited_exp(BZR);
                        let CAF = rspice_limited_exp(BZS);
                        let CAG = BZH - ((((BWI * BZK) - (BZZ * CAA)) - (BZZ * CAC)) / (((-1e0f64 * BWI) - (((CAE * EB) * (B + (EV * (BZO * BZN)))) / (B + CAE))) - (((CAF * EB) * (B + (EV * (BZP * BZN)))) / (B + CAF))));
                        let CAH = BYX - CAG;
                        let CAI = ((AO * CAH) + (AO * (((CAH * CAH) + 4e-18f64).sqrt()))).powf(EV);
                        let CAJ = CAG / BVY;
                        let CAK = CAJ - ((BZO * CAI) / BVY);
                        let CAL = CAJ - ((BZP * CAI) / BVY);
                        let CAM = if CAK >= FU { 1.0 } else { 0.0 };
                        if CAM != 0.0 {
                        } else {
                            let CAN = if CAK <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CAN != 0.0 {
                            } else {
                            }
                        }
                        let CAO = if CAL >= FU { 1.0 } else { 0.0 };
                        if CAO != 0.0 {
                        } else {
                            let CAP = if CAL <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CAP != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let CAR = if CAQ < A { 1.0 } else { 0.0 };
                    if CAR != 0.0 {
                    } else {
                    }
                    DCE = BVY;
                } else {
                    DCE = DCF;
                }
                DCD = DCE;
            } else {
                let CAS = if BVI != A { 1.0 } else { 0.0 };
                let DCU;
                if CAS != 0.0 {
                    let CAT = if BVI == B { 1.0 } else { 0.0 };
                    let CAV = if CAT != 0.0 {
                        Q
                    } else {
                        let CAU = AC - N;
                        CAU
                    };
                    let CAW = AI * (B + BLG);
                    let CAX = BLJ + (DP * BLK);
                    let CAY = CO / BLO;
                    let CAZ = CAX + (CAW * (((BLQ / ((EC * CAW) * CAW)) * BLR).ln()));
                    let CBA = CAV - CAZ;
                    let CBB = ((AO * (CBA + (((CBA * CBA) + EG).sqrt()))) + CAZ) - CAX;
                    let CBC = CAY / (5.19105229416e-2f64 * CAW);
                    let CBD = EJ / CBC;
                    let CBE = B / CBC;
                    let CBF = CAY / EA;
                    let CBG = AO * CBB;
                    let CBH = CBB * CBB;
                    let CBI = CBG + (AO * ((CBH + 3.6e-1f64).sqrt()));
                    let CBJ = CBI * CBI;
                    let CBK = CBD * CBD;
                    let CBL = CBE * CBE;
                    let CBM = BMG / BD;
                    let CBN = (CBF * CBI).powf(EV);
                    let CBO = (AW * BMG) / BD;
                    let CBP = ((CBI + (CAW * (B - ((CBC * ((CBI * CBD) / ((CBJ + CBK).sqrt()))).ln())))) - (CBM * CBN)) / ((CBI * (B + (CAW / ((CBI * CBE) / ((CBJ + CBL).sqrt()))))) + (CBO * CBN));
                    let CBQ = AW * CAW;
                    let CBR = CBB / CBQ;
                    let CBS = if CBR < FB { 1.0 } else { 0.0 };
                    let CBV = if CBS != 0.0 {
                        let CBT = ((CBQ * CBF) * (((BD * CBR) / BH) + (((rspice_limited_exp((CBR / BH))) + (rspice_limited_exp(((-3e0f64 * CBR) / BH)))).ln()))) / ((B / CBP) + ((CBF / EB) * (rspice_limited_exp(((-1e0f64 * CBB) / CBQ)))));
                        CBT
                    } else {
                        let CBU = ((CBQ * CBF) * CBR) / ((B / CBP) + ((CBF / EB) * (rspice_limited_exp(((-1e0f64 * CBB) / CBQ)))));
                        CBU
                    };
                    let CBW = CBB - (CBV / CBF);
                    let CBX = if ((CBW - CBB).abs()) > FH { 1.0 } else { 0.0 };
                    let CDQ;
                    if CBX != 0.0 {
                        let CBY = CBB - CBW;
                        let CBZ = (AO * CBY) + (AO * (((CBY * CBY) + 4e-18f64).sqrt()));
                        let CCA = CBF.powf(EV);
                        let CCB = CBZ.powf(EV);
                        let CCC = CBZ.powf(-3.333333333333333e-1f64);
                        let CCD = BMG * CCA;
                        let CCE = BMZ * CCA;
                        let CCF = CBW / CAW;
                        let CCG = CCF - ((CCD * CCB) / CAW);
                        let CCH = CCF - ((CCE * CCB) / CAW);
                        let CCI = if CCG >= FU { 1.0 } else { 0.0 };
                        let CCP;
                        if CCI != 0.0 {
                            CCP = CCG;
                        } else {
                            let CCJ = if CCG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CCQ = if CCJ != 0.0 {
                                A
                            } else {
                                let CCK = ((CCG.exp()) + B).ln();
                                CCK
                            };
                            CCP = CCQ;
                        }
                        let CCL = if CCH >= FU { 1.0 } else { 0.0 };
                        let CCR;
                        if CCL != 0.0 {
                            CCR = CCH;
                        } else {
                            let CCM = if CCH <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CCS = if CCM != 0.0 {
                                A
                            } else {
                                let CCN = ((CCH.exp()) + B).ln();
                                CCN
                            };
                            CCR = CCS;
                        }
                        let CCO = EB * CAW;
                        let CCT = rspice_limited_exp(CCG);
                        let CCU = rspice_limited_exp(CCH);
                        let CCV = CBW - ((((CBF * CBZ) - (CCO * CCP)) - (CCO * CCR)) / (((-1e0f64 * CBF) - (((CCT * EB) * (B + (EV * (CCD * CCC)))) / (B + CCT))) - (((CCU * EB) * (B + (EV * (CCE * CCC)))) / (B + CCU))));
                        let CCW = CBB - CCV;
                        let CCX = (AO * CCW) + (AO * (((CCW * CCW) + 4e-18f64).sqrt()));
                        let CCY = CCX.powf(-3.333333333333333e-1f64);
                        let CCZ = CCX.powf(EV);
                        let CDA = CCV / CAW;
                        let CDB = CDA - ((CCD * CCZ) / CAW);
                        let CDC = CDA - ((CCE * CCZ) / CAW);
                        let CDD = if CDB >= FU { 1.0 } else { 0.0 };
                        let CDJ;
                        if CDD != 0.0 {
                            CDJ = CDB;
                        } else {
                            let CDE = if CDB <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CDK = if CDE != 0.0 {
                                A
                            } else {
                                let CDF = ((CDB.exp()) + B).ln();
                                CDF
                            };
                            CDJ = CDK;
                        }
                        let CDG = if CDC >= FU { 1.0 } else { 0.0 };
                        let CDL;
                        if CDG != 0.0 {
                            CDL = CDC;
                        } else {
                            let CDH = if CDC <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CDM = if CDH != 0.0 {
                                A
                            } else {
                                let CDI = ((CDC.exp()) + B).ln();
                                CDI
                            };
                            CDL = CDM;
                        }
                        let CDN = rspice_limited_exp(CDB);
                        let CDO = rspice_limited_exp(CDC);
                        let CDP = CCV - ((((CBF * CCX) - (CCO * CDJ)) - (CCO * CDL)) / (((-1e0f64 * CBF) - (((CDN * EB) * (B + (EV * (CCD * CCY)))) / (B + CDN))) - (((CDO * EB) * (B + (EV * (CCE * CCY)))) / (B + CDO))));
                        CDQ = CDP;
                    } else {
                        CDQ = CBW;
                    }
                    let CDR = (CAY / CO) * ((CBB - CDQ).abs());
                    let CDS = CBG + (AO * ((CBH + 3.6e-1f64).sqrt()));
                    let CDT = ((AW * (BON * HP)) / ((BOM * HN) / (((B + (HU * CDR)) + (HV * (CDR * CDR))) + (HW * (HT * ((DV - CDQ).abs())))))) * BLQ;
                    let CDU = CBB - (A * ((B + ((A / ((CDT * CDS) / (CDT + CDS))).powf(IA))).powf((-1e0f64 / IA))));
                    let CDV = (AO * CDU) + (AO * (((CDU * CDU) + 3.6e-1f64).sqrt()));
                    let CDW = CDV * CDV;
                    let CDX = (CBF * CDV).powf(EV);
                    let CDY = ((CDV + (CAW * (B - ((CBC * ((CDV * CBD) / ((CDW + CBK).sqrt()))).ln())))) - (CBM * CDX)) / ((CDV * (B + (CAW / ((CDV * CBE) / ((CDW + CBL).sqrt()))))) + (CBO * CDX));
                    let CDZ = CDU / CBQ;
                    let CEA = if CDZ < FB { 1.0 } else { 0.0 };
                    let CED = if CEA != 0.0 {
                        let CEB = ((CBQ * CBF) * (((BD * CDZ) / BH) + (((rspice_limited_exp((CDZ / BH))) + (rspice_limited_exp(((-3e0f64 * CDZ) / BH)))).ln()))) / ((B / CDY) + ((CBF / EB) * (rspice_limited_exp(((-1e0f64 * CDU) / CBQ)))));
                        CEB
                    } else {
                        let CEC = ((CBQ * CBF) * CDZ) / ((B / CDY) + ((CBF / EB) * (rspice_limited_exp(((-1e0f64 * CDU) / CBQ)))));
                        CEC
                    };
                    let CEE = CDU - (CED / CBF);
                    let CEF = if ((CEE - CDU).abs()) > FH { 1.0 } else { 0.0 };
                    if CEF != 0.0 {
                        let CEG = CDU - CEE;
                        let CEH = (AO * CEG) + (AO * (((CEG * CEG) + 4e-18f64).sqrt()));
                        let CEI = CBF.powf(EV);
                        let CEJ = CEH.powf(EV);
                        let CEK = CEH.powf(-3.333333333333333e-1f64);
                        let CEL = BMG * CEI;
                        let CEM = BMZ * CEI;
                        let CEN = CEE / CAW;
                        let CEO = CEN - ((CEL * CEJ) / CAW);
                        let CEP = CEN - ((CEM * CEJ) / CAW);
                        let CEQ = if CEO >= FU { 1.0 } else { 0.0 };
                        let CEX;
                        if CEQ != 0.0 {
                            CEX = CEO;
                        } else {
                            let CER = if CEO <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CEY = if CER != 0.0 {
                                A
                            } else {
                                let CES = ((CEO.exp()) + B).ln();
                                CES
                            };
                            CEX = CEY;
                        }
                        let CET = if CEP >= FU { 1.0 } else { 0.0 };
                        let CEZ;
                        if CET != 0.0 {
                            CEZ = CEP;
                        } else {
                            let CEU = if CEP <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CFA = if CEU != 0.0 {
                                A
                            } else {
                                let CEV = ((CEP.exp()) + B).ln();
                                CEV
                            };
                            CEZ = CFA;
                        }
                        let CEW = EB * CAW;
                        let CFB = rspice_limited_exp(CEO);
                        let CFC = rspice_limited_exp(CEP);
                        let CFD = CEE - ((((CBF * CEH) - (CEW * CEX)) - (CEW * CEZ)) / (((-1e0f64 * CBF) - (((CFB * EB) * (B + (EV * (CEL * CEK)))) / (B + CFB))) - (((CFC * EB) * (B + (EV * (CEM * CEK)))) / (B + CFC))));
                        let CFE = CDU - CFD;
                        let CFF = ((AO * CFE) + (AO * (((CFE * CFE) + 4e-18f64).sqrt()))).powf(EV);
                        let CFG = CFD / CAW;
                        let CFH = CFG - ((CEL * CFF) / CAW);
                        let CFI = CFG - ((CEM * CFF) / CAW);
                        let CFJ = if CFH >= FU { 1.0 } else { 0.0 };
                        if CFJ != 0.0 {
                        } else {
                            let CFK = if CFH <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CFK != 0.0 {
                            } else {
                            }
                        }
                        let CFL = if CFI >= FU { 1.0 } else { 0.0 };
                        if CFL != 0.0 {
                        } else {
                            let CFM = if CFI <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CFM != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCU = CAW;
                } else {
                    DCU = DCF;
                }
                DCD = DCU;
            }
            let DCB;
            if UZ != 0.0 {
                let CFO = if CFN != A { 1.0 } else { 0.0 };
                let DCC;
                if CFO != 0.0 {
                    let CFP = UF - BKS;
                    let CFQ = if CFN == B { 1.0 } else { 0.0 };
                    let CFY;
                    let CFZ;
                    if CFQ != 0.0 {
                        let CFR = P - BKS;
                        let CFS = P - UF;
                        CFY = CFS;
                        CFZ = CFR;
                    } else {
                        let CFT = AC - BKS;
                        let CFU = AC - UF;
                        CFY = CFU;
                        CFZ = CFT;
                    }
                    let CFV = if CFP < A { 1.0 } else { 0.0 };
                    let CGA;
                    let CGP;
                    let CLH;
                    if CFV != 0.0 {
                        let CFX = CFW * CFP;
                        CGA = CFX;
                        CGP = CFY;
                        CLH = CFW;
                    } else {
                        CGA = CFP;
                        CGP = CFZ;
                        CLH = B;
                    }
                    let CGB = (((CGA * CGA) + C).sqrt()) - Z;
                    let CGE = AI * ((B + CGC) + (CGD * CGB));
                    let CGJ = (CGF - (DP * CGG)) - ((CGH * (CGB * CGI)) / (((CGB * CGB) + (CGI * CGI)).sqrt()));
                    let CGL = CO / CGK;
                    let CGO = CGJ + (CGE * (((CGM / ((EC * CGE) * CGE)) * CGN).ln()));
                    let CGQ = CGP - CGO;
                    let CGR = ((AO * (CGQ + (((CGQ * CGQ) + EG).sqrt()))) + CGO) - CGJ;
                    let CGS = CGL / (5.19105229416e-2f64 * CGE);
                    let CGT = EJ / CGS;
                    let CGU = B / CGS;
                    let CGV = CGL / EA;
                    let CGW = AO * CGR;
                    let CGX = CGR * CGR;
                    let CGY = CGW + (AO * ((CGX + 3.6e-1f64).sqrt()));
                    let CGZ = CGY * CGY;
                    let CHA = CGT * CGT;
                    let CHB = CGU * CGU;
                    let CHD = CHC / BD;
                    let CHE = (CGV * CGY).powf(EV);
                    let CHF = (AW * CHC) / BD;
                    let CHG = ((CGY + (CGE * (B - ((CGS * ((CGY * CGT) / ((CGZ + CHA).sqrt()))).ln())))) - (CHD * CHE)) / ((CGY * (B + (CGE / ((CGY * CGU) / ((CGZ + CHB).sqrt()))))) + (CHF * CHE));
                    let CHH = AW * CGE;
                    let CHI = CGR / CHH;
                    let CHJ = if CHI < FB { 1.0 } else { 0.0 };
                    let CHM = if CHJ != 0.0 {
                        let CHK = ((CHH * CGV) * (((BD * CHI) / BH) + (((rspice_limited_exp((CHI / BH))) + (rspice_limited_exp(((-3e0f64 * CHI) / BH)))).ln()))) / ((B / CHG) + ((CGV / EB) * (rspice_limited_exp(((-1e0f64 * CGR) / CHH)))));
                        CHK
                    } else {
                        let CHL = ((CHH * CGV) * CHI) / ((B / CHG) + ((CGV / EB) * (rspice_limited_exp(((-1e0f64 * CGR) / CHH)))));
                        CHL
                    };
                    let CHN = CGR - (CHM / CGV);
                    let CHO = if ((CHN - CGR).abs()) > FH { 1.0 } else { 0.0 };
                    let CJK;
                    if CHO != 0.0 {
                        let CHP = CGR - CHN;
                        let CHQ = (AO * CHP) + (AO * (((CHP * CHP) + 4e-18f64).sqrt()));
                        let CHR = CGV.powf(EV);
                        let CHS = CHQ.powf(EV);
                        let CHT = CHQ.powf(-3.333333333333333e-1f64);
                        let CHU = CHC * CHR;
                        let CHW = CHV * CHR;
                        let CHX = CHN / CGE;
                        let CHY = CHX - ((CHU * CHS) / CGE);
                        let CHZ = CHX - ((CHW * CHS) / CGE);
                        let CIA = if CHY >= FU { 1.0 } else { 0.0 };
                        let CIH;
                        if CIA != 0.0 {
                            CIH = CHY;
                        } else {
                            let CIB = if CHY <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CII = if CIB != 0.0 {
                                A
                            } else {
                                let CIC = ((CHY.exp()) + B).ln();
                                CIC
                            };
                            CIH = CII;
                        }
                        let CID = if CHZ >= FU { 1.0 } else { 0.0 };
                        let CIJ;
                        if CID != 0.0 {
                            CIJ = CHZ;
                        } else {
                            let CIE = if CHZ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CIK = if CIE != 0.0 {
                                A
                            } else {
                                let CIF = ((CHZ.exp()) + B).ln();
                                CIF
                            };
                            CIJ = CIK;
                        }
                        let CIG = EB * CGE;
                        let CIL = rspice_limited_exp(CHY);
                        let CIM = rspice_limited_exp(CHZ);
                        let CIN = CHN - ((((CGV * CHQ) - (CIG * CIH)) - (CIG * CIJ)) / (((-1e0f64 * CGV) - (((CIL * EB) * (B + (EV * (CHU * CHT)))) / (B + CIL))) - (((CIM * EB) * (B + (EV * (CHW * CHT)))) / (B + CIM))));
                        let CIO = CGR - CIN;
                        let CIP = (AO * CIO) + (AO * (((CIO * CIO) + 4e-18f64).sqrt()));
                        let CIQ = CIP.powf(-3.333333333333333e-1f64);
                        let CIR = CIP.powf(EV);
                        let CIS = CIN / CGE;
                        let CIT = CIS - ((CHU * CIR) / CGE);
                        let CIU = CIS - ((CHW * CIR) / CGE);
                        let CIV = if CIT >= FU { 1.0 } else { 0.0 };
                        let CJB;
                        if CIV != 0.0 {
                            CJB = CIT;
                        } else {
                            let CIW = if CIT <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CJC = if CIW != 0.0 {
                                A
                            } else {
                                let CIX = ((CIT.exp()) + B).ln();
                                CIX
                            };
                            CJB = CJC;
                        }
                        let CIY = if CIU >= FU { 1.0 } else { 0.0 };
                        let CJD;
                        if CIY != 0.0 {
                            CJD = CIU;
                        } else {
                            let CIZ = if CIU <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CJE = if CIZ != 0.0 {
                                A
                            } else {
                                let CJA = ((CIU.exp()) + B).ln();
                                CJA
                            };
                            CJD = CJE;
                        }
                        let CJF = rspice_limited_exp(CIT);
                        let CJG = rspice_limited_exp(CIU);
                        let CJH = CIN - ((((CGV * CIP) - (CIG * CJB)) - (CIG * CJD)) / (((-1e0f64 * CGV) - (((CJF * EB) * (B + (EV * (CHU * CIQ)))) / (B + CJF))) - (((CJG * EB) * (B + (EV * (CHW * CIQ)))) / (B + CJG))));
                        CJK = CJH;
                    } else {
                        CJK = CHN;
                    }
                    let CJL = (CGL / CO) * ((CGR - CJK).abs());
                    let CJM = CGW + (AO * ((CGX + 3.6e-1f64).sqrt()));
                    let CJN = ((AW * (CJJ * HP)) / ((CJI * HN) / (((B + (HU * CJL)) + (HV * (CJL * CJL))) + (HW * (HT * ((DV - CJK).abs())))))) * CGM;
                    let CJO = CGR - (CGA * ((B + ((CGA / ((CJN * CJM) / (CJN + CJM))).powf(IA))).powf((-1e0f64 / IA))));
                    let CJP = (AO * CJO) + (AO * (((CJO * CJO) + 3.6e-1f64).sqrt()));
                    let CJQ = CJP * CJP;
                    let CJR = (CGV * CJP).powf(EV);
                    let CJS = ((CJP + (CGE * (B - ((CGS * ((CJP * CGT) / ((CJQ + CHA).sqrt()))).ln())))) - (CHD * CJR)) / ((CJP * (B + (CGE / ((CJP * CGU) / ((CJQ + CHB).sqrt()))))) + (CHF * CJR));
                    let CJT = CJO / CHH;
                    let CJU = if CJT < FB { 1.0 } else { 0.0 };
                    let CJX = if CJU != 0.0 {
                        let CJV = ((CHH * CGV) * (((BD * CJT) / BH) + (((rspice_limited_exp((CJT / BH))) + (rspice_limited_exp(((-3e0f64 * CJT) / BH)))).ln()))) / ((B / CJS) + ((CGV / EB) * (rspice_limited_exp(((-1e0f64 * CJO) / CHH)))));
                        CJV
                    } else {
                        let CJW = ((CHH * CGV) * CJT) / ((B / CJS) + ((CGV / EB) * (rspice_limited_exp(((-1e0f64 * CJO) / CHH)))));
                        CJW
                    };
                    let CJY = CJO - (CJX / CGV);
                    let CJZ = if ((CJY - CJO).abs()) > FH { 1.0 } else { 0.0 };
                    if CJZ != 0.0 {
                        let CKA = CJO - CJY;
                        let CKB = (AO * CKA) + (AO * (((CKA * CKA) + 4e-18f64).sqrt()));
                        let CKC = CGV.powf(EV);
                        let CKD = CKB.powf(EV);
                        let CKE = CKB.powf(-3.333333333333333e-1f64);
                        let CKF = CHC * CKC;
                        let CKG = CHV * CKC;
                        let CKH = CJY / CGE;
                        let CKI = CKH - ((CKF * CKD) / CGE);
                        let CKJ = CKH - ((CKG * CKD) / CGE);
                        let CKK = if CKI >= FU { 1.0 } else { 0.0 };
                        let CKR;
                        if CKK != 0.0 {
                            CKR = CKI;
                        } else {
                            let CKL = if CKI <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CKS = if CKL != 0.0 {
                                A
                            } else {
                                let CKM = ((CKI.exp()) + B).ln();
                                CKM
                            };
                            CKR = CKS;
                        }
                        let CKN = if CKJ >= FU { 1.0 } else { 0.0 };
                        let CKT;
                        if CKN != 0.0 {
                            CKT = CKJ;
                        } else {
                            let CKO = if CKJ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CKU = if CKO != 0.0 {
                                A
                            } else {
                                let CKP = ((CKJ.exp()) + B).ln();
                                CKP
                            };
                            CKT = CKU;
                        }
                        let CKQ = EB * CGE;
                        let CKV = rspice_limited_exp(CKI);
                        let CKW = rspice_limited_exp(CKJ);
                        let CKX = CJY - ((((CGV * CKB) - (CKQ * CKR)) - (CKQ * CKT)) / (((-1e0f64 * CGV) - (((CKV * EB) * (B + (EV * (CKF * CKE)))) / (B + CKV))) - (((CKW * EB) * (B + (EV * (CKG * CKE)))) / (B + CKW))));
                        let CKY = CJO - CKX;
                        let CKZ = ((AO * CKY) + (AO * (((CKY * CKY) + 4e-18f64).sqrt()))).powf(EV);
                        let CLA = CKX / CGE;
                        let CLB = CLA - ((CKF * CKZ) / CGE);
                        let CLC = CLA - ((CKG * CKZ) / CGE);
                        let CLD = if CLB >= FU { 1.0 } else { 0.0 };
                        if CLD != 0.0 {
                        } else {
                            let CLE = if CLB <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CLE != 0.0 {
                            } else {
                            }
                        }
                        let CLF = if CLC >= FU { 1.0 } else { 0.0 };
                        if CLF != 0.0 {
                        } else {
                            let CLG = if CLC <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CLG != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let CLI = if CLH < A { 1.0 } else { 0.0 };
                    if CLI != 0.0 {
                    } else {
                    }
                    DCC = CGE;
                } else {
                    DCC = DCD;
                }
                DCB = DCC;
            } else {
                let CLJ = if CFN != A { 1.0 } else { 0.0 };
                let DCV;
                if CLJ != 0.0 {
                    let CLK = if CFN == B { 1.0 } else { 0.0 };
                    let CLM = if CLK != 0.0 {
                        R
                    } else {
                        let CLL = AC - M;
                        CLL
                    };
                    let CLN = AI * (B + CGC);
                    let CLO = CGF - (DP * CGG);
                    let CLP = CO / CGK;
                    let CLQ = CLO + (CLN * (((CGM / ((EC * CLN) * CLN)) * CGN).ln()));
                    let CLR = CLM - CLQ;
                    let CLS = ((AO * (CLR + (((CLR * CLR) + EG).sqrt()))) + CLQ) - CLO;
                    let CLT = CLP / (5.19105229416e-2f64 * CLN);
                    let CLU = EJ / CLT;
                    let CLV = B / CLT;
                    let CLW = CLP / EA;
                    let CLX = AO * CLS;
                    let CLY = CLS * CLS;
                    let CLZ = CLX + (AO * ((CLY + 3.6e-1f64).sqrt()));
                    let CMA = CLZ * CLZ;
                    let CMB = CLU * CLU;
                    let CMC = CLV * CLV;
                    let CMD = CHC / BD;
                    let CME = (CLW * CLZ).powf(EV);
                    let CMF = (AW * CHC) / BD;
                    let CMG = ((CLZ + (CLN * (B - ((CLT * ((CLZ * CLU) / ((CMA + CMB).sqrt()))).ln())))) - (CMD * CME)) / ((CLZ * (B + (CLN / ((CLZ * CLV) / ((CMA + CMC).sqrt()))))) + (CMF * CME));
                    let CMH = AW * CLN;
                    let CMI = CLS / CMH;
                    let CMJ = if CMI < FB { 1.0 } else { 0.0 };
                    let CMM = if CMJ != 0.0 {
                        let CMK = ((CMH * CLW) * (((BD * CMI) / BH) + (((rspice_limited_exp((CMI / BH))) + (rspice_limited_exp(((-3e0f64 * CMI) / BH)))).ln()))) / ((B / CMG) + ((CLW / EB) * (rspice_limited_exp(((-1e0f64 * CLS) / CMH)))));
                        CMK
                    } else {
                        let CML = ((CMH * CLW) * CMI) / ((B / CMG) + ((CLW / EB) * (rspice_limited_exp(((-1e0f64 * CLS) / CMH)))));
                        CML
                    };
                    let CMN = CLS - (CMM / CLW);
                    let CMO = if ((CMN - CLS).abs()) > FH { 1.0 } else { 0.0 };
                    let COH;
                    if CMO != 0.0 {
                        let CMP = CLS - CMN;
                        let CMQ = (AO * CMP) + (AO * (((CMP * CMP) + 4e-18f64).sqrt()));
                        let CMR = CLW.powf(EV);
                        let CMS = CMQ.powf(EV);
                        let CMT = CMQ.powf(-3.333333333333333e-1f64);
                        let CMU = CHC * CMR;
                        let CMV = CHV * CMR;
                        let CMW = CMN / CLN;
                        let CMX = CMW - ((CMU * CMS) / CLN);
                        let CMY = CMW - ((CMV * CMS) / CLN);
                        let CMZ = if CMX >= FU { 1.0 } else { 0.0 };
                        let CNG;
                        if CMZ != 0.0 {
                            CNG = CMX;
                        } else {
                            let CNA = if CMX <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CNH = if CNA != 0.0 {
                                A
                            } else {
                                let CNB = ((CMX.exp()) + B).ln();
                                CNB
                            };
                            CNG = CNH;
                        }
                        let CNC = if CMY >= FU { 1.0 } else { 0.0 };
                        let CNI;
                        if CNC != 0.0 {
                            CNI = CMY;
                        } else {
                            let CND = if CMY <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CNJ = if CND != 0.0 {
                                A
                            } else {
                                let CNE = ((CMY.exp()) + B).ln();
                                CNE
                            };
                            CNI = CNJ;
                        }
                        let CNF = EB * CLN;
                        let CNK = rspice_limited_exp(CMX);
                        let CNL = rspice_limited_exp(CMY);
                        let CNM = CMN - ((((CLW * CMQ) - (CNF * CNG)) - (CNF * CNI)) / (((-1e0f64 * CLW) - (((CNK * EB) * (B + (EV * (CMU * CMT)))) / (B + CNK))) - (((CNL * EB) * (B + (EV * (CMV * CMT)))) / (B + CNL))));
                        let CNN = CLS - CNM;
                        let CNO = (AO * CNN) + (AO * (((CNN * CNN) + 4e-18f64).sqrt()));
                        let CNP = CNO.powf(-3.333333333333333e-1f64);
                        let CNQ = CNO.powf(EV);
                        let CNR = CNM / CLN;
                        let CNS = CNR - ((CMU * CNQ) / CLN);
                        let CNT = CNR - ((CMV * CNQ) / CLN);
                        let CNU = if CNS >= FU { 1.0 } else { 0.0 };
                        let COA;
                        if CNU != 0.0 {
                            COA = CNS;
                        } else {
                            let CNV = if CNS <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let COB = if CNV != 0.0 {
                                A
                            } else {
                                let CNW = ((CNS.exp()) + B).ln();
                                CNW
                            };
                            COA = COB;
                        }
                        let CNX = if CNT >= FU { 1.0 } else { 0.0 };
                        let COC;
                        if CNX != 0.0 {
                            COC = CNT;
                        } else {
                            let CNY = if CNT <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let COD = if CNY != 0.0 {
                                A
                            } else {
                                let CNZ = ((CNT.exp()) + B).ln();
                                CNZ
                            };
                            COC = COD;
                        }
                        let COE = rspice_limited_exp(CNS);
                        let COF = rspice_limited_exp(CNT);
                        let COG = CNM - ((((CLW * CNO) - (CNF * COA)) - (CNF * COC)) / (((-1e0f64 * CLW) - (((COE * EB) * (B + (EV * (CMU * CNP)))) / (B + COE))) - (((COF * EB) * (B + (EV * (CMV * CNP)))) / (B + COF))));
                        COH = COG;
                    } else {
                        COH = CMN;
                    }
                    let COI = (CLP / CO) * ((CLS - COH).abs());
                    let COJ = CLX + (AO * ((CLY + 3.6e-1f64).sqrt()));
                    let COK = ((AW * (CJJ * HP)) / ((CJI * HN) / (((B + (HU * COI)) + (HV * (COI * COI))) + (HW * (HT * ((DV - COH).abs())))))) * CGM;
                    let COL = CLS - (A * ((B + ((A / ((COK * COJ) / (COK + COJ))).powf(IA))).powf((-1e0f64 / IA))));
                    let COM = (AO * COL) + (AO * (((COL * COL) + 3.6e-1f64).sqrt()));
                    let CON = COM * COM;
                    let COO = (CLW * COM).powf(EV);
                    let COP = ((COM + (CLN * (B - ((CLT * ((COM * CLU) / ((CON + CMB).sqrt()))).ln())))) - (CMD * COO)) / ((COM * (B + (CLN / ((COM * CLV) / ((CON + CMC).sqrt()))))) + (CMF * COO));
                    let COQ = COL / CMH;
                    let COR = if COQ < FB { 1.0 } else { 0.0 };
                    let COU = if COR != 0.0 {
                        let COS = ((CMH * CLW) * (((BD * COQ) / BH) + (((rspice_limited_exp((COQ / BH))) + (rspice_limited_exp(((-3e0f64 * COQ) / BH)))).ln()))) / ((B / COP) + ((CLW / EB) * (rspice_limited_exp(((-1e0f64 * COL) / CMH)))));
                        COS
                    } else {
                        let COT = ((CMH * CLW) * COQ) / ((B / COP) + ((CLW / EB) * (rspice_limited_exp(((-1e0f64 * COL) / CMH)))));
                        COT
                    };
                    let COV = COL - (COU / CLW);
                    let COW = if ((COV - COL).abs()) > FH { 1.0 } else { 0.0 };
                    if COW != 0.0 {
                        let COX = COL - COV;
                        let COY = (AO * COX) + (AO * (((COX * COX) + 4e-18f64).sqrt()));
                        let COZ = CLW.powf(EV);
                        let CPA = COY.powf(EV);
                        let CPB = COY.powf(-3.333333333333333e-1f64);
                        let CPC = CHC * COZ;
                        let CPD = CHV * COZ;
                        let CPE = COV / CLN;
                        let CPF = CPE - ((CPC * CPA) / CLN);
                        let CPG = CPE - ((CPD * CPA) / CLN);
                        let CPH = if CPF >= FU { 1.0 } else { 0.0 };
                        let CPO;
                        if CPH != 0.0 {
                            CPO = CPF;
                        } else {
                            let CPI = if CPF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CPP = if CPI != 0.0 {
                                A
                            } else {
                                let CPJ = ((CPF.exp()) + B).ln();
                                CPJ
                            };
                            CPO = CPP;
                        }
                        let CPK = if CPG >= FU { 1.0 } else { 0.0 };
                        let CPQ;
                        if CPK != 0.0 {
                            CPQ = CPG;
                        } else {
                            let CPL = if CPG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CPR = if CPL != 0.0 {
                                A
                            } else {
                                let CPM = ((CPG.exp()) + B).ln();
                                CPM
                            };
                            CPQ = CPR;
                        }
                        let CPN = EB * CLN;
                        let CPS = rspice_limited_exp(CPF);
                        let CPT = rspice_limited_exp(CPG);
                        let CPU = COV - ((((CLW * COY) - (CPN * CPO)) - (CPN * CPQ)) / (((-1e0f64 * CLW) - (((CPS * EB) * (B + (EV * (CPC * CPB)))) / (B + CPS))) - (((CPT * EB) * (B + (EV * (CPD * CPB)))) / (B + CPT))));
                        let CPV = COL - CPU;
                        let CPW = ((AO * CPV) + (AO * (((CPV * CPV) + 4e-18f64).sqrt()))).powf(EV);
                        let CPX = CPU / CLN;
                        let CPY = CPX - ((CPC * CPW) / CLN);
                        let CPZ = CPX - ((CPD * CPW) / CLN);
                        let CQA = if CPY >= FU { 1.0 } else { 0.0 };
                        if CQA != 0.0 {
                        } else {
                            let CQB = if CPY <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CQB != 0.0 {
                            } else {
                            }
                        }
                        let CQC = if CPZ >= FU { 1.0 } else { 0.0 };
                        if CQC != 0.0 {
                        } else {
                            let CQD = if CPZ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CQD != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCV = CLN;
                } else {
                    DCV = DCD;
                }
                DCB = DCV;
            }
            let DBZ;
            if UZ != 0.0 {
                let CQF = if CQE != A { 1.0 } else { 0.0 };
                let DCA;
                if CQF != 0.0 {
                    let CQG = BVK - UG;
                    let CQH = if CQE == B { 1.0 } else { 0.0 };
                    let CQP;
                    let CQQ;
                    if CQH != 0.0 {
                        let CQI = P - UG;
                        let CQJ = P - BVK;
                        CQP = CQJ;
                        CQQ = CQI;
                    } else {
                        let CQK = AC - UG;
                        let CQL = AC - BVK;
                        CQP = CQL;
                        CQQ = CQK;
                    }
                    let CQM = if CQG < A { 1.0 } else { 0.0 };
                    let CQR;
                    let CQX;
                    let CVL;
                    if CQM != 0.0 {
                        let CQO = CQN * CQG;
                        CQR = CQO;
                        CQX = CQP;
                        CVL = CQN;
                    } else {
                        CQR = CQG;
                        CQX = CQQ;
                        CVL = B;
                    }
                    let CQS = (((CQR * CQR) + C).sqrt()) - Z;
                    let CQT = AI * ((B + CGC) + (CGD * CQS));
                    let CQU = (CGF + (DP * CGG)) - ((CGH * (CQS * CGI)) / (((CQS * CQS) + (CGI * CGI)).sqrt()));
                    let CQV = CO / CGK;
                    let CQW = CQU + (CQT * (((CGM / ((EC * CQT) * CQT)) * CGN).ln()));
                    let CQY = CQX - CQW;
                    let CQZ = ((AO * (CQY + (((CQY * CQY) + EG).sqrt()))) + CQW) - CQU;
                    let CRA = CQV / (5.19105229416e-2f64 * CQT);
                    let CRB = EJ / CRA;
                    let CRC = B / CRA;
                    let CRD = CQV / EA;
                    let CRE = AO * CQZ;
                    let CRF = CQZ * CQZ;
                    let CRG = CRE + (AO * ((CRF + 3.6e-1f64).sqrt()));
                    let CRH = CRG * CRG;
                    let CRI = CRB * CRB;
                    let CRJ = CRC * CRC;
                    let CRK = CHC / BD;
                    let CRL = (CRD * CRG).powf(EV);
                    let CRM = (AW * CHC) / BD;
                    let CRN = ((CRG + (CQT * (B - ((CRA * ((CRG * CRB) / ((CRH + CRI).sqrt()))).ln())))) - (CRK * CRL)) / ((CRG * (B + (CQT / ((CRG * CRC) / ((CRH + CRJ).sqrt()))))) + (CRM * CRL));
                    let CRO = AW * CQT;
                    let CRP = CQZ / CRO;
                    let CRQ = if CRP < FB { 1.0 } else { 0.0 };
                    let CRT = if CRQ != 0.0 {
                        let CRR = ((CRO * CRD) * (((BD * CRP) / BH) + (((rspice_limited_exp((CRP / BH))) + (rspice_limited_exp(((-3e0f64 * CRP) / BH)))).ln()))) / ((B / CRN) + ((CRD / EB) * (rspice_limited_exp(((-1e0f64 * CQZ) / CRO)))));
                        CRR
                    } else {
                        let CRS = ((CRO * CRD) * CRP) / ((B / CRN) + ((CRD / EB) * (rspice_limited_exp(((-1e0f64 * CQZ) / CRO)))));
                        CRS
                    };
                    let CRU = CQZ - (CRT / CRD);
                    let CRV = if ((CRU - CQZ).abs()) > FH { 1.0 } else { 0.0 };
                    let CTO;
                    if CRV != 0.0 {
                        let CRW = CQZ - CRU;
                        let CRX = (AO * CRW) + (AO * (((CRW * CRW) + 4e-18f64).sqrt()));
                        let CRY = CRD.powf(EV);
                        let CRZ = CRX.powf(EV);
                        let CSA = CRX.powf(-3.333333333333333e-1f64);
                        let CSB = CHC * CRY;
                        let CSC = CHV * CRY;
                        let CSD = CRU / CQT;
                        let CSE = CSD - ((CSB * CRZ) / CQT);
                        let CSF = CSD - ((CSC * CRZ) / CQT);
                        let CSG = if CSE >= FU { 1.0 } else { 0.0 };
                        let CSN;
                        if CSG != 0.0 {
                            CSN = CSE;
                        } else {
                            let CSH = if CSE <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CSO = if CSH != 0.0 {
                                A
                            } else {
                                let CSI = ((CSE.exp()) + B).ln();
                                CSI
                            };
                            CSN = CSO;
                        }
                        let CSJ = if CSF >= FU { 1.0 } else { 0.0 };
                        let CSP;
                        if CSJ != 0.0 {
                            CSP = CSF;
                        } else {
                            let CSK = if CSF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CSQ = if CSK != 0.0 {
                                A
                            } else {
                                let CSL = ((CSF.exp()) + B).ln();
                                CSL
                            };
                            CSP = CSQ;
                        }
                        let CSM = EB * CQT;
                        let CSR = rspice_limited_exp(CSE);
                        let CSS = rspice_limited_exp(CSF);
                        let CST = CRU - ((((CRD * CRX) - (CSM * CSN)) - (CSM * CSP)) / (((-1e0f64 * CRD) - (((CSR * EB) * (B + (EV * (CSB * CSA)))) / (B + CSR))) - (((CSS * EB) * (B + (EV * (CSC * CSA)))) / (B + CSS))));
                        let CSU = CQZ - CST;
                        let CSV = (AO * CSU) + (AO * (((CSU * CSU) + 4e-18f64).sqrt()));
                        let CSW = CSV.powf(-3.333333333333333e-1f64);
                        let CSX = CSV.powf(EV);
                        let CSY = CST / CQT;
                        let CSZ = CSY - ((CSB * CSX) / CQT);
                        let CTA = CSY - ((CSC * CSX) / CQT);
                        let CTB = if CSZ >= FU { 1.0 } else { 0.0 };
                        let CTH;
                        if CTB != 0.0 {
                            CTH = CSZ;
                        } else {
                            let CTC = if CSZ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CTI = if CTC != 0.0 {
                                A
                            } else {
                                let CTD = ((CSZ.exp()) + B).ln();
                                CTD
                            };
                            CTH = CTI;
                        }
                        let CTE = if CTA >= FU { 1.0 } else { 0.0 };
                        let CTJ;
                        if CTE != 0.0 {
                            CTJ = CTA;
                        } else {
                            let CTF = if CTA <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CTK = if CTF != 0.0 {
                                A
                            } else {
                                let CTG = ((CTA.exp()) + B).ln();
                                CTG
                            };
                            CTJ = CTK;
                        }
                        let CTL = rspice_limited_exp(CSZ);
                        let CTM = rspice_limited_exp(CTA);
                        let CTN = CST - ((((CRD * CSV) - (CSM * CTH)) - (CSM * CTJ)) / (((-1e0f64 * CRD) - (((CTL * EB) * (B + (EV * (CSB * CSW)))) / (B + CTL))) - (((CTM * EB) * (B + (EV * (CSC * CSW)))) / (B + CTM))));
                        CTO = CTN;
                    } else {
                        CTO = CRU;
                    }
                    let CTP = (CQV / CO) * ((CQZ - CTO).abs());
                    let CTQ = CRE + (AO * ((CRF + 3.6e-1f64).sqrt()));
                    let CTR = ((AW * (CJJ * HP)) / ((CJI * HN) / (((B + (HU * CTP)) + (HV * (CTP * CTP))) + (HW * (HT * ((DV - CTO).abs())))))) * CGM;
                    let CTS = CQZ - (CQR * ((B + ((CQR / ((CTR * CTQ) / (CTR + CTQ))).powf(IA))).powf((-1e0f64 / IA))));
                    let CTT = (AO * CTS) + (AO * (((CTS * CTS) + 3.6e-1f64).sqrt()));
                    let CTU = CTT * CTT;
                    let CTV = (CRD * CTT).powf(EV);
                    let CTW = ((CTT + (CQT * (B - ((CRA * ((CTT * CRB) / ((CTU + CRI).sqrt()))).ln())))) - (CRK * CTV)) / ((CTT * (B + (CQT / ((CTT * CRC) / ((CTU + CRJ).sqrt()))))) + (CRM * CTV));
                    let CTX = CTS / CRO;
                    let CTY = if CTX < FB { 1.0 } else { 0.0 };
                    let CUB = if CTY != 0.0 {
                        let CTZ = ((CRO * CRD) * (((BD * CTX) / BH) + (((rspice_limited_exp((CTX / BH))) + (rspice_limited_exp(((-3e0f64 * CTX) / BH)))).ln()))) / ((B / CTW) + ((CRD / EB) * (rspice_limited_exp(((-1e0f64 * CTS) / CRO)))));
                        CTZ
                    } else {
                        let CUA = ((CRO * CRD) * CTX) / ((B / CTW) + ((CRD / EB) * (rspice_limited_exp(((-1e0f64 * CTS) / CRO)))));
                        CUA
                    };
                    let CUC = CTS - (CUB / CRD);
                    let CUD = if ((CUC - CTS).abs()) > FH { 1.0 } else { 0.0 };
                    if CUD != 0.0 {
                        let CUE = CTS - CUC;
                        let CUF = (AO * CUE) + (AO * (((CUE * CUE) + 4e-18f64).sqrt()));
                        let CUG = CRD.powf(EV);
                        let CUH = CUF.powf(EV);
                        let CUI = CUF.powf(-3.333333333333333e-1f64);
                        let CUJ = CHC * CUG;
                        let CUK = CHV * CUG;
                        let CUL = CUC / CQT;
                        let CUM = CUL - ((CUJ * CUH) / CQT);
                        let CUN = CUL - ((CUK * CUH) / CQT);
                        let CUO = if CUM >= FU { 1.0 } else { 0.0 };
                        let CUV;
                        if CUO != 0.0 {
                            CUV = CUM;
                        } else {
                            let CUP = if CUM <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CUW = if CUP != 0.0 {
                                A
                            } else {
                                let CUQ = ((CUM.exp()) + B).ln();
                                CUQ
                            };
                            CUV = CUW;
                        }
                        let CUR = if CUN >= FU { 1.0 } else { 0.0 };
                        let CUX;
                        if CUR != 0.0 {
                            CUX = CUN;
                        } else {
                            let CUS = if CUN <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CUY = if CUS != 0.0 {
                                A
                            } else {
                                let CUT = ((CUN.exp()) + B).ln();
                                CUT
                            };
                            CUX = CUY;
                        }
                        let CUU = EB * CQT;
                        let CUZ = rspice_limited_exp(CUM);
                        let CVA = rspice_limited_exp(CUN);
                        let CVB = CUC - ((((CRD * CUF) - (CUU * CUV)) - (CUU * CUX)) / (((-1e0f64 * CRD) - (((CUZ * EB) * (B + (EV * (CUJ * CUI)))) / (B + CUZ))) - (((CVA * EB) * (B + (EV * (CUK * CUI)))) / (B + CVA))));
                        let CVC = CTS - CVB;
                        let CVD = ((AO * CVC) + (AO * (((CVC * CVC) + 4e-18f64).sqrt()))).powf(EV);
                        let CVE = CVB / CQT;
                        let CVF = CVE - ((CUJ * CVD) / CQT);
                        let CVG = CVE - ((CUK * CVD) / CQT);
                        let CVH = if CVF >= FU { 1.0 } else { 0.0 };
                        if CVH != 0.0 {
                        } else {
                            let CVI = if CVF <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CVI != 0.0 {
                            } else {
                            }
                        }
                        let CVJ = if CVG >= FU { 1.0 } else { 0.0 };
                        if CVJ != 0.0 {
                        } else {
                            let CVK = if CVG <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if CVK != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let CVM = if CVL < A { 1.0 } else { 0.0 };
                    if CVM != 0.0 {
                    } else {
                    }
                    DCA = CQT;
                } else {
                    DCA = DCB;
                }
                DBZ = DCA;
            } else {
                let CVN = if CQE != A { 1.0 } else { 0.0 };
                let DCW;
                if CVN != 0.0 {
                    let CVO = if CQE == B { 1.0 } else { 0.0 };
                    let CVQ = if CVO != 0.0 {
                        Q
                    } else {
                        let CVP = AC - N;
                        CVP
                    };
                    let CVR = AI * (B + CGC);
                    let CVS = CGF + (DP * CGG);
                    let CVT = CO / CGK;
                    let CVU = CVS + (CVR * (((CGM / ((EC * CVR) * CVR)) * CGN).ln()));
                    let CVV = CVQ - CVU;
                    let CVW = ((AO * (CVV + (((CVV * CVV) + EG).sqrt()))) + CVU) - CVS;
                    let CVX = CVT / (5.19105229416e-2f64 * CVR);
                    let CVY = EJ / CVX;
                    let CVZ = B / CVX;
                    let CWA = CVT / EA;
                    let CWB = AO * CVW;
                    let CWC = CVW * CVW;
                    let CWD = CWB + (AO * ((CWC + 3.6e-1f64).sqrt()));
                    let CWE = CWD * CWD;
                    let CWF = CVY * CVY;
                    let CWG = CVZ * CVZ;
                    let CWH = CHC / BD;
                    let CWI = (CWA * CWD).powf(EV);
                    let CWJ = (AW * CHC) / BD;
                    let CWK = ((CWD + (CVR * (B - ((CVX * ((CWD * CVY) / ((CWE + CWF).sqrt()))).ln())))) - (CWH * CWI)) / ((CWD * (B + (CVR / ((CWD * CVZ) / ((CWE + CWG).sqrt()))))) + (CWJ * CWI));
                    let CWL = AW * CVR;
                    let CWM = CVW / CWL;
                    let CWN = if CWM < FB { 1.0 } else { 0.0 };
                    let CWQ = if CWN != 0.0 {
                        let CWO = ((CWL * CWA) * (((BD * CWM) / BH) + (((rspice_limited_exp((CWM / BH))) + (rspice_limited_exp(((-3e0f64 * CWM) / BH)))).ln()))) / ((B / CWK) + ((CWA / EB) * (rspice_limited_exp(((-1e0f64 * CVW) / CWL)))));
                        CWO
                    } else {
                        let CWP = ((CWL * CWA) * CWM) / ((B / CWK) + ((CWA / EB) * (rspice_limited_exp(((-1e0f64 * CVW) / CWL)))));
                        CWP
                    };
                    let CWR = CVW - (CWQ / CWA);
                    let CWS = if ((CWR - CVW).abs()) > FH { 1.0 } else { 0.0 };
                    let CYL;
                    if CWS != 0.0 {
                        let CWT = CVW - CWR;
                        let CWU = (AO * CWT) + (AO * (((CWT * CWT) + 4e-18f64).sqrt()));
                        let CWV = CWA.powf(EV);
                        let CWW = CWU.powf(EV);
                        let CWX = CWU.powf(-3.333333333333333e-1f64);
                        let CWY = CHC * CWV;
                        let CWZ = CHV * CWV;
                        let CXA = CWR / CVR;
                        let CXB = CXA - ((CWY * CWW) / CVR);
                        let CXC = CXA - ((CWZ * CWW) / CVR);
                        let CXD = if CXB >= FU { 1.0 } else { 0.0 };
                        let CXK;
                        if CXD != 0.0 {
                            CXK = CXB;
                        } else {
                            let CXE = if CXB <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CXL = if CXE != 0.0 {
                                A
                            } else {
                                let CXF = ((CXB.exp()) + B).ln();
                                CXF
                            };
                            CXK = CXL;
                        }
                        let CXG = if CXC >= FU { 1.0 } else { 0.0 };
                        let CXM;
                        if CXG != 0.0 {
                            CXM = CXC;
                        } else {
                            let CXH = if CXC <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CXN = if CXH != 0.0 {
                                A
                            } else {
                                let CXI = ((CXC.exp()) + B).ln();
                                CXI
                            };
                            CXM = CXN;
                        }
                        let CXJ = EB * CVR;
                        let CXO = rspice_limited_exp(CXB);
                        let CXP = rspice_limited_exp(CXC);
                        let CXQ = CWR - ((((CWA * CWU) - (CXJ * CXK)) - (CXJ * CXM)) / (((-1e0f64 * CWA) - (((CXO * EB) * (B + (EV * (CWY * CWX)))) / (B + CXO))) - (((CXP * EB) * (B + (EV * (CWZ * CWX)))) / (B + CXP))));
                        let CXR = CVW - CXQ;
                        let CXS = (AO * CXR) + (AO * (((CXR * CXR) + 4e-18f64).sqrt()));
                        let CXT = CXS.powf(-3.333333333333333e-1f64);
                        let CXU = CXS.powf(EV);
                        let CXV = CXQ / CVR;
                        let CXW = CXV - ((CWY * CXU) / CVR);
                        let CXX = CXV - ((CWZ * CXU) / CVR);
                        let CXY = if CXW >= FU { 1.0 } else { 0.0 };
                        let CYE;
                        if CXY != 0.0 {
                            CYE = CXW;
                        } else {
                            let CXZ = if CXW <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CYF = if CXZ != 0.0 {
                                A
                            } else {
                                let CYA = ((CXW.exp()) + B).ln();
                                CYA
                            };
                            CYE = CYF;
                        }
                        let CYB = if CXX >= FU { 1.0 } else { 0.0 };
                        let CYG;
                        if CYB != 0.0 {
                            CYG = CXX;
                        } else {
                            let CYC = if CXX <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CYH = if CYC != 0.0 {
                                A
                            } else {
                                let CYD = ((CXX.exp()) + B).ln();
                                CYD
                            };
                            CYG = CYH;
                        }
                        let CYI = rspice_limited_exp(CXW);
                        let CYJ = rspice_limited_exp(CXX);
                        let CYK = CXQ - ((((CWA * CXS) - (CXJ * CYE)) - (CXJ * CYG)) / (((-1e0f64 * CWA) - (((CYI * EB) * (B + (EV * (CWY * CXT)))) / (B + CYI))) - (((CYJ * EB) * (B + (EV * (CWZ * CXT)))) / (B + CYJ))));
                        CYL = CYK;
                    } else {
                        CYL = CWR;
                    }
                    let CYM = (CVT / CO) * ((CVW - CYL).abs());
                    let CYN = CWB + (AO * ((CWC + 3.6e-1f64).sqrt()));
                    let CYO = ((AW * (CJJ * HP)) / ((CJI * HN) / (((B + (HU * CYM)) + (HV * (CYM * CYM))) + (HW * (HT * ((DV - CYL).abs())))))) * CGM;
                    let CYP = CVW - (A * ((B + ((A / ((CYO * CYN) / (CYO + CYN))).powf(IA))).powf((-1e0f64 / IA))));
                    let CYQ = (AO * CYP) + (AO * (((CYP * CYP) + 3.6e-1f64).sqrt()));
                    let CYR = CYQ * CYQ;
                    let CYS = (CWA * CYQ).powf(EV);
                    let CYT = ((CYQ + (CVR * (B - ((CVX * ((CYQ * CVY) / ((CYR + CWF).sqrt()))).ln())))) - (CWH * CYS)) / ((CYQ * (B + (CVR / ((CYQ * CVZ) / ((CYR + CWG).sqrt()))))) + (CWJ * CYS));
                    let CYU = CYP / CWL;
                    let CYV = if CYU < FB { 1.0 } else { 0.0 };
                    let CYY = if CYV != 0.0 {
                        let CYW = ((CWL * CWA) * (((BD * CYU) / BH) + (((rspice_limited_exp((CYU / BH))) + (rspice_limited_exp(((-3e0f64 * CYU) / BH)))).ln()))) / ((B / CYT) + ((CWA / EB) * (rspice_limited_exp(((-1e0f64 * CYP) / CWL)))));
                        CYW
                    } else {
                        let CYX = ((CWL * CWA) * CYU) / ((B / CYT) + ((CWA / EB) * (rspice_limited_exp(((-1e0f64 * CYP) / CWL)))));
                        CYX
                    };
                    let CYZ = CYP - (CYY / CWA);
                    let CZA = if ((CYZ - CYP).abs()) > FH { 1.0 } else { 0.0 };
                    if CZA != 0.0 {
                        let CZB = CYP - CYZ;
                        let CZC = (AO * CZB) + (AO * (((CZB * CZB) + 4e-18f64).sqrt()));
                        let CZD = CWA.powf(EV);
                        let CZE = CZC.powf(EV);
                        let CZF = CZC.powf(-3.333333333333333e-1f64);
                        let CZG = CHC * CZD;
                        let CZH = CHV * CZD;
                        let CZI = CYZ / CVR;
                        let CZJ = CZI - ((CZG * CZE) / CVR);
                        let CZK = CZI - ((CZH * CZE) / CVR);
                        let CZL = if CZJ >= FU { 1.0 } else { 0.0 };
                        let CZS;
                        if CZL != 0.0 {
                            CZS = CZJ;
                        } else {
                            let CZM = if CZJ <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CZT = if CZM != 0.0 {
                                A
                            } else {
                                let CZN = ((CZJ.exp()) + B).ln();
                                CZN
                            };
                            CZS = CZT;
                        }
                        let CZO = if CZK >= FU { 1.0 } else { 0.0 };
                        let CZU;
                        if CZO != 0.0 {
                            CZU = CZK;
                        } else {
                            let CZP = if CZK <= -3.7e1f64 { 1.0 } else { 0.0 };
                            let CZV = if CZP != 0.0 {
                                A
                            } else {
                                let CZQ = ((CZK.exp()) + B).ln();
                                CZQ
                            };
                            CZU = CZV;
                        }
                        let CZR = EB * CVR;
                        let CZW = rspice_limited_exp(CZJ);
                        let CZX = rspice_limited_exp(CZK);
                        let CZY = CYZ - ((((CWA * CZC) - (CZR * CZS)) - (CZR * CZU)) / (((-1e0f64 * CWA) - (((CZW * EB) * (B + (EV * (CZG * CZF)))) / (B + CZW))) - (((CZX * EB) * (B + (EV * (CZH * CZF)))) / (B + CZX))));
                        let CZZ = CYP - CZY;
                        let DAA = ((AO * CZZ) + (AO * (((CZZ * CZZ) + 4e-18f64).sqrt()))).powf(EV);
                        let DAB = CZY / CVR;
                        let DAC = DAB - ((CZG * DAA) / CVR);
                        let DAD = DAB - ((CZH * DAA) / CVR);
                        let DAE = if DAC >= FU { 1.0 } else { 0.0 };
                        if DAE != 0.0 {
                        } else {
                            let DAF = if DAC <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if DAF != 0.0 {
                            } else {
                            }
                        }
                        let DAG = if DAD >= FU { 1.0 } else { 0.0 };
                        if DAG != 0.0 {
                        } else {
                            let DAH = if DAD <= -3.7e1f64 { 1.0 } else { 0.0 };
                            if DAH != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    DCW = CVR;
                } else {
                    DCW = DCB;
                }
                DBZ = DCW;
            }
            let DAJ = if DAI == B { 1.0 } else { 0.0 };
            if DAJ != 0.0 {
                let DAN = if ((DAK * (DAL + ((DY / BD) / DAM))) / ((DAM * KM) * DX)) > A { 1.0 } else { 0.0 };
                if DAN != 0.0 {
                } else {
                }
            } else {
                let DAO = if DAI == AW { 1.0 } else { 0.0 };
                if DAO != 0.0 {
                    let DAP = (DAM * KM) * DX;
                    let DAQ = (DAK * ((DZ / BD) / DAM)) / DAP;
                    let DAR = if ((DAK * (DAL + ((DY / BD) / DAM))) / DAP) > A { 1.0 } else { 0.0 };
                    if DAR != 0.0 {
                    } else {
                    }
                    let DAT = if DAQ > A { 1.0 } else { 0.0 };
                    if DAT != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            let DAU = if DAI == AW { 1.0 } else { 0.0 };
            let DBE;
            let DBF;
            if DAU != 0.0 {
                let DAW = (LA * DAV) * (DAS - AC);
                let DBA = (if ((LA * DAZ) - ((LA * (if DAY <= (DAZ / (AW * DAX)) { DAY } else { (DAZ / (AW * DAX)) })) * ((AD * DAX) / ((AE + (DAX * DAX)).sqrt())))) >= A { ((LA * DAZ) - ((LA * (if DAY <= (DAZ / (AW * DAX)) { DAY } else { (DAZ / (AW * DAX)) })) * ((AD * DAX) / ((AE + (DAX * DAX)).sqrt())))) } else { A }) * (DAS - AB);
                DBE = DBA;
                DBF = DAW;
            } else {
                let DBB = (LA * DAV) * (AY - AC);
                let DBC = (if ((LA * DAZ) - ((LA * (if DAY <= (DAZ / (AW * DAX)) { DAY } else { (DAZ / (AW * DAX)) })) * ((AD * DAX) / ((AE + (DAX * DAX)).sqrt())))) >= A { ((LA * DAZ) - ((LA * (if DAY <= (DAZ / (AW * DAX)) { DAY } else { (DAZ / (AW * DAX)) })) * ((AD * DAX) / ((AE + (DAX * DAX)).sqrt())))) } else { A }) * (AY - AB);
                DBE = DBC;
                DBF = DBB;
            }
            let DBD = (LA * parameters[212]) * AD;
            let DBH = DBG + ((-DBE) + DBD);
            let DBJ = DBI + ((-DBF) - DBD);
            let DBK = parameters[279] + (DP * parameters[285]);
            let DBL = parameters[275] + (DP * parameters[283]);
            let DBM = parameters[277] * ((parameters[281] * DP).exp());
            let DBN = parameters[276] + (DP * parameters[284]);
            let DBO = if ((AB - S) - (parameters[280] + (DP * parameters[286]))) >= A { ((AB - S) - (parameters[280] + (DP * parameters[286]))) } else { A };
            let DBP = if (LA * (parameters[278] * ((parameters[282] * DP).exp()))) > A { 1.0 } else { 0.0 };
            if DBP != 0.0 {
                let DBQ = if DBO > A { 1.0 } else { 0.0 };
                if DBQ != 0.0 {
                    let DBR = if (DBO / (DBN * AI)) > KV { 1.0 } else { 0.0 };
                    if DBR != 0.0 {
                    } else {
                    }
                } else {
                    let DBS = if (DBO / (DBN * AI)) > KV { 1.0 } else { 0.0 };
                    if DBS != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let DBT = if ((AC - S) - DBK) >= A { ((AC - S) - DBK) } else { A };
            let DBU = if (LA * DBM) > A { 1.0 } else { 0.0 };
            if DBU != 0.0 {
                let DBV = if DBT > A { 1.0 } else { 0.0 };
                if DBV != 0.0 {
                    let DBW = if (DBT / (DBL * AI)) > KV { 1.0 } else { 0.0 };
                    if DBW != 0.0 {
                    } else {
                    }
                } else {
                    let DBX = if (DBT / (DBL * AI)) > KV { 1.0 } else { 0.0 };
                    if DBX != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let DBY = if parameters[259] == B { 1.0 } else { 0.0 };
            let DFU;
            let DFV;
            let DFW;
            if DBY != 0.0 {
                let DCZ = if DBH >= DCY { DBH } else { DCY };
                let DDA = if DBJ >= DCY { DBJ } else { DCY };
                let DDD = ((((((DBZ * EA) * EA) * EA) / ((LA * DX) * DX)) * (KP * KP)) * ((DX / ((KJ + DBZ) * (if KI >= 1e-12f64 { KI } else { 1e-12f64 }))) / (CQ * CQ))) * (((((((DCX * DBZ) * CQ) * (B / DCZ)) * (B - (DBH / DDA))) + ((DCX + ((DDB * DBZ) * CQ)) * ((DCZ / DDA).ln()))) + ((DDB + ((DDC * DBZ) * CQ)) * (DBJ - DBH))) + ((DDC / AW) * ((DBH * DBH) - (DBJ * DBJ))));
                let DDF = if LH != 0.0 {
                    let DDE = -DDD;
                    DDE
                } else {
                    DDD
                };
                let DDG = DDF * parameters[8];
                DFU = B;
                DFV = DDG;
                DFW = DDH;
            } else {
                DFU = A;
                DFV = A;
                DFW = A;
            }
            if DAU != 0.0 {
            } else {
            }
            if UZ != 0.0 {
                let DDI = if VA != A { 1.0 } else { 0.0 };
                if DDI != 0.0 {
                    let DDJ = if VA == B { 1.0 } else { 0.0 };
                    if DDJ != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DDK = if VA != A { 1.0 } else { 0.0 };
                if DDK != 0.0 {
                    let DDL = if VA == B { 1.0 } else { 0.0 };
                    if DDL != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if UZ != 0.0 {
                let DDM = if AFR != A { 1.0 } else { 0.0 };
                if DDM != 0.0 {
                    let DDN = if AFR == B { 1.0 } else { 0.0 };
                    if DDN != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DDO = if AFR != A { 1.0 } else { 0.0 };
                if DDO != 0.0 {
                    let DDP = if AFR == B { 1.0 } else { 0.0 };
                    if DDP != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if UZ != 0.0 {
                let DDQ = if APT != A { 1.0 } else { 0.0 };
                if DDQ != 0.0 {
                    let DDR = if APT == B { 1.0 } else { 0.0 };
                    if DDR != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DDS = if APT != A { 1.0 } else { 0.0 };
                if DDS != 0.0 {
                    let DDT = if APT == B { 1.0 } else { 0.0 };
                    if DDT != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if UZ != 0.0 {
                let DDU = if BAL != A { 1.0 } else { 0.0 };
                if DDU != 0.0 {
                    let DDV = if BAL == B { 1.0 } else { 0.0 };
                    if DDV != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DDW = if BAL != A { 1.0 } else { 0.0 };
                if DDW != 0.0 {
                    let DDX = if BAL == B { 1.0 } else { 0.0 };
                    if DDX != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if UZ != 0.0 {
                let DDY = if BKQ != A { 1.0 } else { 0.0 };
                if DDY != 0.0 {
                    let DDZ = if BKQ == B { 1.0 } else { 0.0 };
                    if DDZ != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DEA = if BKQ != A { 1.0 } else { 0.0 };
                if DEA != 0.0 {
                    let DEB = if BKQ == B { 1.0 } else { 0.0 };
                    if DEB != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if UZ != 0.0 {
                let DEC = if BVI != A { 1.0 } else { 0.0 };
                if DEC != 0.0 {
                    let DED = if BVI == B { 1.0 } else { 0.0 };
                    if DED != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DEE = if BVI != A { 1.0 } else { 0.0 };
                if DEE != 0.0 {
                    let DEF = if BVI == B { 1.0 } else { 0.0 };
                    if DEF != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if UZ != 0.0 {
                let DEG = if CFN != A { 1.0 } else { 0.0 };
                if DEG != 0.0 {
                    let DEH = if CFN == B { 1.0 } else { 0.0 };
                    if DEH != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DEI = if CFN != A { 1.0 } else { 0.0 };
                if DEI != 0.0 {
                    let DEJ = if CFN == B { 1.0 } else { 0.0 };
                    if DEJ != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if UZ != 0.0 {
                let DEK = if CQE != A { 1.0 } else { 0.0 };
                if DEK != 0.0 {
                    let DEL = if CQE == B { 1.0 } else { 0.0 };
                    if DEL != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let DEM = if CQE != A { 1.0 } else { 0.0 };
                if DEM != 0.0 {
                    let DEN = if CQE == B { 1.0 } else { 0.0 };
                    if DEN != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            let DEO = if (if E == B { 1.0 } else { 0.0 }) != 0.0 && (if F > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if DEO != 0.0 {
            } else {
            }
            let DEP = if KZ > A { 1.0 } else { 0.0 };
            if DEP != 0.0 {
            } else {
            }
        if DEQ == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DER;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DES == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DEV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DEY == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DFB;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DFE == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DFH;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DFK == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DFN;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DFQ == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DFR;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DFS == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DFT;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if DFU == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = DFV;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(DFW);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

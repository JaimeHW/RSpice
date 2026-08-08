#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Lanes, rspice_eval_ddt, rspice_eval_idt, rspice_limexp, rspice_limited_exp, rspice_limited_exp_derivative};
impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let multiplicity = self.multiplicity;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 10542 => 0usize, 10544 => 1usize, 10546 => 2usize, 10548 => 3usize, 10550 => 4usize, 10552 => 5usize, 10554 => 6usize, 10556 => 7usize, 10558 => 8usize, 10560 => 9usize, 10562 => 10usize, _ => usize::MAX };
            rspice_eval_ddt(
                &mut ddt_state.ddt_current,
                &mut ddt_state.ddt_previous,
                &mut ddt_state.ddt_older,
                &mut ddt_state.ddt_initialized,
                &mut ddt_state.ddt_derivative_current,
                &mut ddt_state.ddt_derivative_previous,
                ddt_active,
                ddt_coefficients.derivative_scale,
                ddt_coefficients.previous_value_scale,
                ddt_coefficients.older_value_scale,
                ddt_coefficients.previous_derivative_scale,
                slot,
                value,
            )
        };
            let A = 0e0f64;
            let B = if parameter_given[6] { 1.0 } else { 0.0 };
            let C = 1e0f64;
            let D = if parameter_given[7] { 1.0 } else { 0.0 };
            let E = 1e-2f64;
            let F = if parameter_given[10] { 1.0 } else { 0.0 };
            let G = parameters[10];
            let I = if parameter_given[11] { 1.0 } else { 0.0 };
            let J = parameters[11];
            let L = if parameter_given[3] { 1.0 } else { 0.0 };
            let M = if parameter_given[4] { 1.0 } else { 0.0 };
            let N = -1e0f64;
            let O = if parameter_given[5] { 1.0 } else { 0.0 };
            let P = parameters[5];
            let R = parameters[74];
            let V = parameters[75];
            let Z = parameters[20];
            let AD = parameters[79];
            let AH = parameters[80];
            let AM = 2.7315e2f64;
            let AS = parameters[14];
            let AW = parameters[15];
            let BC = 1.380662e-23f64;
            let BD = 1.602189e-19f64;
            let BG = parameters[26];
            let BH = parameters[90];
            let BJ = parameters[89];
            let BL = parameters[88];
            let BO = parameters[122];
            let BP = parameters[28];
            let BW = parameters[72];
            let BY = 5e-1f64;
            let BZ = 4e0f64;
            let CA = parameters[73];
            let CD = parameters[27];
            let CE = parameters[29];
            let CN = parameters[31];
            let CO = parameters[33];
            let CX = parameters[54];
            let CY = parameters[123];
            let CZ = parameters[56];
            let DB = parameters[114];
            let DH = parameters[58];
            let DI = parameters[124];
            let DJ = parameters[59];
            let DQ = parameters[60];
            let DR = parameters[61];
            let DU = parameters[115];
            let EB = parameters[62];
            let EC = parameters[63];
            let EL = parameters[64];
            let EP = parameters[65];
            let ET = parameters[66];
            let EU = parameters[67];
            let EW = parameters[116];
            let FC = parameters[68];
            let FD = parameters[69];
            let FK = node_potentials[4];
            let FY = parameters[126];
            let GA = if parameter_given[109] { 1.0 } else { 0.0 };
            let GB = parameters[16];
            let GC = parameters[109];
            let GE = parameters[107];
            let GG = if parameter_given[108] { 1.0 } else { 0.0 };
            let GH = parameters[17];
            let GI = parameters[108];
            let GL = if parameter_given[106] { 1.0 } else { 0.0 };
            let GM = parameters[21];
            let GN = parameters[106];
            let GP = parameters[104];
            let GR = if parameter_given[105] { 1.0 } else { 0.0 };
            let GS = parameters[22];
            let GT = parameters[105];
            let GW = parameters[23];
            let GX = parameters[103];
            let GZ = parameters[24];
            let HA = parameters[111];
            let HC = if parameter_given[110] { 1.0 } else { 0.0 };
            let HD = parameters[25];
            let HE = parameters[110];
            let HH = parameters[101];
            let HI = parameters[132];
            let JL = parameters[129];
            let JP = parameters[84];
            let JQ = parameters[127];
            let JS = parameters[86];
            let JT = parameters[128];
            let JV = parameters[92];
            let JX = parameters[93];
            let JZ = 2e0f64;
            let KC = parameters[37];
            let KM = 3e0f64;
            let KZ = parameters[42];
            let LQ = parameters[50];
            let MH = parameters[36];
            let MJ = parameters[38];
            let ML = parameters[41];
            let MN = parameters[43];
            let MQ = parameters[48];
            let MS = parameters[49];
            let MU = parameters[51];
            let MW = parameters[19];
            let NB = parameters[18];
            let NC = parameters[112];
            let NI = parameters[70];
            let NJ = parameters[130];
            let NL = parameters[71];
            let NM = parameters[131];
            let NP = 1e-3f64;
            let NS = 1e3f64;
            let PI = node_potentials[8];
            let PJ = node_potentials[9];
            let PL = node_potentials[7];
            let PN = node_potentials[6];
            let PP = node_potentials[5];
            let PS = node_potentials[10];
            let PU = node_potentials[1];
            let PV = node_potentials[2];
            let PY = node_potentials[0];
            let QG = node_potentials[11];
            let QK = node_potentials[12];
            let QL = node_potentials[13];
            let QN = parameters[34];
            let QP = parameters[39];
            let RN = -5e-1f64;
            let SK = parameters[44];
            let SX = parameters[45];
            let TR = parameters[46];
            let VH = -5e-1f64;
            let WZ = 1e-4f64;
            let XB = 1e-8f64;
            let YN = parameters[32];
            let YX = 5.0005e-1f64;
            let ZI = parameters[55];
            let AAA = parameters[57];
            let AHR = parameters[83];
            let AHT = 2e-2f64;
            let AHV = 1.01e0f64;
            let AIO = parameters[85];
            let AIR = parameters[87];
            let AJK = parameters[97];
            let AJL = parameters[95];
            let AJN = parameters[94];
            let AJP = 1e-1f64;
            let AJW = parameters[96];
            let AMD = parameters[52];
            let ANB = -5e-1f64;
            let AOS = -5e-1f64;
            let ASG = -5e-1f64;
            let ASZ = 1.44e0f64;
            let ATF = parameters[76];
            let ATG = parameters[77];
            let ATI = parameters[78];
            let ATR = parameters[81];
            let ATS = parameters[47];
            let ATY = parameters[53];
            let ATZ = parameters[35];
            let AUB = parameters[40];
            let AUD = parameters[102];
            let AUF = parameters[82];
            let AUH = 3.333333333333333e-1f64;
            let AVA = parameters[1];
            let AVB = 0e0f64;
            let AVC = 0e0f64;
            let AVD = 0e0f64;
            let AVE = 0e0f64;
            let AVF = 0e0f64;
            let AVG = 0e0f64;
            let AVH = 0e0f64;
            let AVI = 0e0f64;
            let AVJ = 0e0f64;
            let AVK = 0e0f64;
            let AVL = 0e0f64;
            let AVM = 0e0f64;
            let AVN = 0e0f64;
            let AVO = 0e0f64;
            let AVP = 0e0f64;
            let AWF = 1e0f64;
            let AWG = 1e0f64;
            let AWH = 1e0f64;
            let AWI = 1e0f64;
            let AWJ = 1e0f64;
            let AWK = 1e0f64;
            let AWL = 1e0f64;
            let AWM = 1e0f64;
            let AWN = 1e0f64;
            let AWO = 1e0f64;
            let AWP = 1e0f64;
            let AWQ = 1e0f64;
            let AWR = 1e0f64;
            let AWS = 1e0f64;
            let AWT = 1e0f64;
            let BAG = -1e0f64;
            let BCG = 2e0f64;
            let BCY = 0e0f64;
            let BES = Lanes([0e0f64; 3]);
            let BGC = Lanes([0e0f64; 3]);
            let BHI = Lanes([0e0f64; 5]);
            let BHJ = Lanes([0e0f64; 6]);
            let BKI = Lanes([0e0f64; 4]);
            let BLE = Lanes([0e0f64; 4]);
            let BLQ = Lanes([0e0f64; 3]);
            let BNF = Lanes([0e0f64; 5]);
            let BNP = Lanes([0e0f64; 4]);
            let BOB = Lanes([0e0f64; 2]);
            let BOI = Lanes([0e0f64; 3]);
            let BSA = Lanes([0e0f64; 3]);
            let BUI = ddt_scale();
            if B != 0.0 {
            } else {
            }
            if D != 0.0 {
            } else {
            }
            let ALO = if F != 0.0 {
                G
            } else {
                let H = ctx.simparam_or("gmin", 1e-12f64);
                H
            };
            let BM = if I != 0.0 {
                J
            } else {
                let K = ctx.simparam_or("pnjmaxi", C);
                K
            };
            let PF;
            if L != 0.0 {
                PF = C;
            } else {
                let PG;
                if M != 0.0 {
                    PG = N;
                } else {
                    let PH = if O != 0.0 {
                        P
                    } else {
                        C
                    };
                    PG = PH;
                }
                PF = PG;
            }
            let Q = parameters[12].ln();
            let S = if R > A { 1.0 } else { 0.0 };
            let U = if S != 0.0 {
                let T = C / R;
                T
            } else {
                A
            };
            let W = if V > A { 1.0 } else { 0.0 };
            let Y = if W != 0.0 {
                let X = C / V;
                X
            } else {
                A
            };
            let AA = if Z > A { 1.0 } else { 0.0 };
            let AC = if AA != 0.0 {
                let AB = C / Z;
                AB
            } else {
                A
            };
            let AE = if AD > A { 1.0 } else { 0.0 };
            let AG = if AE != 0.0 {
                let AF = C / AD;
                AF
            } else {
                A
            };
            let AI = if AH > A { 1.0 } else { 0.0 };
            let AK = if AI != 0.0 {
                let AJ = C / AH;
                AJ
            } else {
                A
            };
            let AL = if AI != 0.0 {
                A
            } else {
                C
            };
            let AN = AM + parameters[13];
            let AO = temperature + parameters[0];
            let AP = AO - AM;
            let AQ = if AP < parameters[8] { 1.0 } else { 0.0 };
            if AQ != 0.0 {
            } else {
            }
            let AR = if AP > parameters[9] { 1.0 } else { 0.0 };
            if AR != 0.0 {
            } else {
            }
            let AT = AS + C;
            let AU = if AP < AT { 1.0 } else { 0.0 };
            let AZ;
            if AU != 0.0 {
                let AV = AS + (((AP - AS) - C).exp());
                AZ = AV;
            } else {
                let AX = if AP > (AW - C) { 1.0 } else { 0.0 };
                let BA = if AX != 0.0 {
                    let AY = AW - (((AW - AP) - C).exp());
                    AY
                } else {
                    AP
                };
                AZ = BA;
            }
            let BB = AZ + AM;
            let BE = (BC * BB) / BD;
            let BF = BB / AN;
            let BI = if BH > A { 1.0 } else { 0.0 };
            let AAP = if BI != 0.0 {
                let BK = BJ * BE;
                let BN = BK * (((((-BL) / BK).exp()) + (BM / BH)).ln());
                BN
            } else {
                A
            };
            let BQ = BO / BP;
            let BR = -parameters[113];
            let BS = C - BF;
            let BT = BE * BP;
            let BU = (BG * (BF.powf(BQ))) * (((BR * BS) / BT).exp());
            let BV = if BU > A { 1.0 } else { 0.0 };
            let VX;
            if BV != 0.0 {
                let BX = if (if BW > A { 1.0 } else { 0.0 }) != 0.0 && (if BM > BW { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let VY = if BX != 0.0 {
                    let CB = BT * ((C + ((((BY * BM) * ((BZ / BW).powf(CA))).powf((C / (C - CA)))) / BU)).ln());
                    CB
                } else {
                    let CC = BT * ((C + (BM / BU)).ln());
                    CC
                };
                VX = VY;
            } else {
                VX = A;
            }
            let CF = parameters[125] / CE;
            let CG = -parameters[121];
            let CH = BE * CE;
            let CI = (CD * (BF.powf(CF))) * (((CG * BS) / CH).exp());
            let CJ = if BV != 0.0 && (if CI > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let WK;
            if CJ != 0.0 {
                let CK = if S != 0.0 && (if BM > R { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let WL = if CK != 0.0 {
                    let CL = CH * ((C + ((((BY * BM) * ((BZ / R).powf(CA))).powf((C / (C - CA)))) / (BU * CI))).ln());
                    CL
                } else {
                    let CM = CH * ((C + (BM / (BU * CI))).ln());
                    CM
                };
                WK = WL;
            } else {
                WK = A;
            }
            let CP = BO / CO;
            let CQ = -parameters[120];
            let CR = BE * CO;
            let CS = (CN * (BF.powf(CP))) * (((CQ * BS) / CR).exp());
            let CT = if CS > A { 1.0 } else { 0.0 };
            let XZ;
            if CT != 0.0 {
                let CU = if W != 0.0 && (if BM > V { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let YA = if CU != 0.0 {
                    let CV = CR * ((C + (((BM * BM) * Y) / CS)).ln());
                    CV
                } else {
                    let CW = CR * ((C + (BM / CS)).ln());
                    CW
                };
                XZ = YA;
            } else {
                XZ = A;
            }
            let DA = CY / CZ;
            let DC = -DB;
            let DD = BE * CZ;
            let DE = (CX * (BF.powf(DA))) * (((DC * BS) / DD).exp());
            let DF = if DE > A { 1.0 } else { 0.0 };
            let ZL = if DF != 0.0 {
                let DG = DD * ((C + (BM / DE)).ln());
                DG
            } else {
                A
            };
            let DK = DI / DJ;
            let DL = -parameters[117];
            let DM = BE * DJ;
            let DN = (DH * (BF.powf(DK))) * (((DL * BS) / DM).exp());
            let DO = if DN > A { 1.0 } else { 0.0 };
            let ZT = if DO != 0.0 {
                let DP = DM * ((C + (BM / DN)).ln());
                DP
            } else {
                A
            };
            let DS = CY / DR;
            let DT = BF.powf(DS);
            let DV = -DU;
            let DW = BE * DR;
            let DX = ((DV * BS) / DW).exp();
            let DY = (DQ * DT) * DX;
            let DZ = if DY > A { 1.0 } else { 0.0 };
            let AEU = if DZ != 0.0 {
                let EA = DW * ((C + (BM / DY)).ln());
                EA
            } else {
                A
            };
            let ED = DI / EC;
            let EE = BF.powf(ED);
            let EF = -parameters[118];
            let EG = BE * EC;
            let EH = ((EF * BS) / EG).exp();
            let EI = (EB * EE) * EH;
            let EJ = if EI > A { 1.0 } else { 0.0 };
            let AFC = if EJ != 0.0 {
                let EK = EG * ((C + (BM / EI)).ln());
                EK
            } else {
                A
            };
            let EM = (EL * DT) * DX;
            let EN = if EM > A { 1.0 } else { 0.0 };
            let AFP = if EN != 0.0 {
                let EO = DW * ((C + (BM / EM)).ln());
                EO
            } else {
                A
            };
            let EQ = (EP * EE) * EH;
            let ER = if EQ > A { 1.0 } else { 0.0 };
            let AFW = if ER != 0.0 {
                let ES = EG * ((C + (BM / EQ)).ln());
                ES
            } else {
                A
            };
            let EV = CY / EU;
            let EX = -EW;
            let EY = BE * EU;
            let EZ = (ET * (BF.powf(EV))) * (((EX * BS) / EY).exp());
            let FA = if EZ > A { 1.0 } else { 0.0 };
            let AKD = if FA != 0.0 {
                let FB = EY * ((C + (BM / EZ)).ln());
                FB
            } else {
                A
            };
            let FE = DI / FD;
            let FF = -parameters[119];
            let FG = BE * FD;
            let FH = (FC * (BF.powf(FE))) * (((FF * BS) / FG).exp());
            let FI = if FH > A { 1.0 } else { 0.0 };
            let AKL = if FI != 0.0 {
                let FJ = FG * ((C + (BM / FH)).ln());
                FJ
            } else {
                A
            };
            let FL = (AO + FK) - AM;
            let FM = if FL < AT { 1.0 } else { 0.0 };
            let FS;
            let AWU;
            if FM != 0.0 {
                let FN = ((FL - AS) - C).exp();
                let BAI = AWG * FN;
                let FO = AS + FN;
                FS = FO;
                AWU = BAI;
            } else {
                let FP = if FL > (AW - C) { 1.0 } else { 0.0 };
                let FT;
                let AWV;
                if FP != 0.0 {
                    let FQ = ((AW - FL) - C).exp();
                    let FR = AW - FQ;
                    let BAH = ((AWG * BAG) * FQ) * BAG;
                    FT = FR;
                    AWV = BAH;
                } else {
                    FT = FL;
                    AWV = AWG;
                }
                FS = FT;
                AWU = AWV;
            }
            let FU = FS + AM;
            let FV = (BC * FU) / BD;
            let BAJ = (AWU * BC) / BD;
            let FW = FU / AN;
            let BAK = AWU / AN;
            let FX = FU - AN;
            let FZ = BW * (FW.powf(FY));
            let BAL = (BAK * (FY * (FW.powf((FY - AWF))))) * BW;
            let NO;
            let AWW;
            if GA != 0.0 {
                let GD = GB * (FW.powf(GC));
                let BAN = (BAK * (GC * (FW.powf((GC - AWF))))) * GB;
                NO = GD;
                AWW = BAN;
            } else {
                let GF = GB * (FW.powf(GE));
                let BAM = (BAK * (GE * (FW.powf((GE - AWF))))) * GB;
                NO = GF;
                AWW = BAM;
            }
            let NU;
            let AWX;
            if GG != 0.0 {
                let GJ = GH * (FW.powf(GI));
                let BAP = (BAK * (GI * (FW.powf((GI - AWF))))) * GH;
                NU = GJ;
                AWX = BAP;
            } else {
                let GK = GH * (FW.powf(GE));
                let BAO = (BAK * (GE * (FW.powf((GE - AWF))))) * GH;
                NU = GK;
                AWX = BAO;
            }
            let NY;
            let AWY;
            if GL != 0.0 {
                let GO = GM * (FW.powf(GN));
                let BAR = (BAK * (GN * (FW.powf((GN - AWF))))) * GM;
                NY = GO;
                AWY = BAR;
            } else {
                let GQ = GM * (FW.powf(GP));
                let BAQ = (BAK * (GP * (FW.powf((GP - AWF))))) * GM;
                NY = GQ;
                AWY = BAQ;
            }
            let OC;
            let AWZ;
            if GR != 0.0 {
                let GU = GS * (FW.powf(GT));
                let BAT = (BAK * (GT * (FW.powf((GT - AWF))))) * GS;
                OC = GU;
                AWZ = BAT;
            } else {
                let GV = GS * (FW.powf(GP));
                let BAS = (BAK * (GP * (FW.powf((GP - AWF))))) * GS;
                OC = GV;
                AWZ = BAS;
            }
            let GY = GW * (FW.powf(GX));
            let BAU = (BAK * (GX * (FW.powf((GX - AWF))))) * GW;
            let HB = GZ * (FW.powf(HA));
            let BAV = (BAK * (HA * (FW.powf((HA - AWF))))) * GZ;
            let OJ;
            let AXA;
            if HC != 0.0 {
                let HF = HD * (FW.powf(HE));
                let BAX = (BAK * (HE * (FW.powf((HE - AWF))))) * HD;
                OJ = HF;
                AXA = BAX;
            } else {
                let HG = HD * (FW.powf(GE));
                let BAW = (BAK * (GE * (FW.powf((GE - AWF))))) * HD;
                OJ = HG;
                AXA = BAW;
            }
            let HJ = HH * (C + (FX * HI));
            let BAY = (AWU * HI) * HH;
            let HK = BG * (FW.powf(BQ));
            let HL = C - FW;
            let BAZ = BAK * BAG;
            let HM = BR * HL;
            let BBA = BAZ * BR;
            let HN = FV * BP;
            let HO = HM / HN;
            let HP = HO.exp();
            let HQ = HK * HP;
            let BBB = (((BAK * (BQ * (FW.powf((BQ - AWF))))) * BG) * HP) + ((((BBA - ((BAJ * BP) * HO)) / HN) * HP) * HK);
            let HR = CD * (FW.powf(CF));
            let HS = FV * CE;
            let HT = (CG * HL) / HS;
            let HU = HT.exp();
            let HV = HR * HU;
            let BBC = (((BAK * (CF * (FW.powf((CF - AWF))))) * CD) * HU) + (((((BAZ * CG) - ((BAJ * CE) * HT)) / HS) * HU) * HR);
            let HW = CN * (FW.powf(CP));
            let HX = FV * CO;
            let BBD = BAJ * CO;
            let HY = (CQ * HL) / HX;
            let HZ = HY.exp();
            let IA = HW * HZ;
            let BBE = (((BAK * (CP * (FW.powf((CP - AWF))))) * CN) * HZ) + (((((BAZ * CQ) - (BBD * HY)) / HX) * HZ) * HW);
            let IB = CX * (FW.powf(DA));
            let IC = FV * CZ;
            let BBF = BAJ * CZ;
            let ID = (DC * HL) / IC;
            let IE = ID.exp();
            let IF = IB * IE;
            let BBG = (((BAK * (DA * (FW.powf((DA - AWF))))) * CX) * IE) + (((((BAZ * DC) - (BBF * ID)) / IC) * IE) * IB);
            let IG = DH * (FW.powf(DK));
            let IH = FV * DJ;
            let BBH = BAJ * DJ;
            let II = (DL * HL) / IH;
            let IJ = II.exp();
            let IK = IG * IJ;
            let BBI = (((BAK * (DK * (FW.powf((DK - AWF))))) * DH) * IJ) + (((((BAZ * DL) - (BBH * II)) / IH) * IJ) * IG);
            let IL = FW.powf(DS);
            let BBJ = BAK * (DS * (FW.powf((DS - AWF))));
            let IM = DQ * IL;
            let IN = FV * DR;
            let BBK = BAJ * DR;
            let IO = (DV * HL) / IN;
            let IP = IO.exp();
            let BBL = (((BAZ * DV) - (BBK * IO)) / IN) * IP;
            let IQ = IM * IP;
            let BBM = ((BBJ * DQ) * IP) + (BBL * IM);
            let IR = FW.powf(ED);
            let BBN = BAK * (ED * (FW.powf((ED - AWF))));
            let IS = EB * IR;
            let IT = FV * EC;
            let BBO = BAJ * EC;
            let IU = (EF * HL) / IT;
            let IV = IU.exp();
            let BBP = (((BAZ * EF) - (BBO * IU)) / IT) * IV;
            let IW = IS * IV;
            let BBQ = ((BBN * EB) * IV) + (BBP * IS);
            let IX = EL * IL;
            let IY = IX * IP;
            let BBR = ((BBJ * EL) * IP) + (BBL * IX);
            let IZ = EP * IR;
            let JA = IZ * IV;
            let BBS = ((BBN * EP) * IV) + (BBP * IZ);
            let JB = ET * (FW.powf(EV));
            let JC = FV * EU;
            let BBT = BAJ * EU;
            let JD = (EX * HL) / JC;
            let JE = JD.exp();
            let JF = JB * JE;
            let BBU = (((BAK * (EV * (FW.powf((EV - AWF))))) * ET) * JE) + (((((BAZ * EX) - (BBT * JD)) / JC) * JE) * JB);
            let JG = FC * (FW.powf(FE));
            let JH = FV * FD;
            let BBV = BAJ * FD;
            let JI = (FF * HL) / JH;
            let JJ = JI.exp();
            let JK = JG * JJ;
            let BBW = (((BAK * (FE * (FW.powf((FE - AWF))))) * FC) * JJ) + (((((BAZ * FF) - (BBV * JI)) / JH) * JJ) * JG);
            let BBX = AWU * JL;
            let JM = C + (FX * JL);
            let JN = BP * JM;
            let BBY = BBX * BP;
            let JO = CE * JM;
            let BBZ = BBX * CE;
            let JR = JP * (C + (FX * JQ));
            let BCA = (AWU * JQ) * JP;
            let JU = JS * (C + (FX * JT));
            let BCB = (AWU * JT) * JS;
            let JW = parameters[91] + (FX * JV);
            let JY = BJ * (C + (FX * JX));
            let KA = FV / FW;
            let KB = JZ * KA;
            let BCC = ((BAJ - (BAK * KA)) / FW) * JZ;
            let KD = BY * KC;
            let KE = (KD * FW) / FV;
            let KF = KE.exp();
            let KG = -5e-1f64 * KC;
            let KH = (KG * FW) / FV;
            let KI = KH.exp();
            let KJ = KF - KI;
            let KK = KJ.ln();
            let KL = KB * KK;
            let KN = KM * FV;
            let KO = FW.ln();
            let KP = KN * KO;
            let BCD = ((BAJ * KM) * KO) + ((BAK * (AWF / FW)) * KN);
            let KQ = FW - C;
            let KR = ((KL * FW) - KP) - (DB * KQ);
            let BCE = (((((BCC * KK) + (((((((BAK * KD) - (BAJ * KE)) / FV) * KF) - ((((BAK * KG) - (BAJ * KH)) / FV) * KI)) * (AWF / KJ)) * KB)) * FW) + (BAK * KL)) - BCD) - (BAK * DB);
            let KS = JZ * FV;
            let BCF = BAJ * JZ;
            let KT = (-KR) / FV;
            let KU = KT.exp();
            let KV = (C + (BZ * KU)).sqrt();
            let KW = BY * (C + KV);
            let KX = KW.ln();
            let KY = KR + (KS * KX);
            let BCH = BCE + ((BCF * KX) + (((((((((BCE * BAG) - (BAJ * KT)) / FV) * KU) * BZ) * (AWF / (BCG * KV))) * BY) * (AWF / KW)) * KS));
            let LA = BY * KZ;
            let LB = (LA * FW) / FV;
            let LC = LB.exp();
            let LD = -5e-1f64 * KZ;
            let LE = (LD * FW) / FV;
            let LF = LE.exp();
            let LG = LC - LF;
            let LH = LG.ln();
            let LI = KB * LH;
            let LJ = ((LI * FW) - KP) - (DU * KQ);
            let BCI = (((((BCC * LH) + (((((((BAK * LA) - (BAJ * LB)) / FV) * LC) - ((((BAK * LD) - (BAJ * LE)) / FV) * LF)) * (AWF / LG)) * KB)) * FW) + (BAK * LI)) - BCD) - (BAK * DU);
            let LK = (-LJ) / FV;
            let LL = LK.exp();
            let LM = (C + (BZ * LL)).sqrt();
            let LN = BY * (C + LM);
            let LO = LN.ln();
            let LP = LJ + (KS * LO);
            let BCJ = BCI + ((BCF * LO) + (((((((((BCI * BAG) - (BAJ * LK)) / FV) * LL) * BZ) * (AWF / (BCG * LM))) * BY) * (AWF / LN)) * KS));
            let LR = BY * LQ;
            let LS = (LR * FW) / FV;
            let LT = LS.exp();
            let LU = -5e-1f64 * LQ;
            let LV = (LU * FW) / FV;
            let LW = LV.exp();
            let LX = LT - LW;
            let LY = LX.ln();
            let LZ = KB * LY;
            let MA = ((LZ * FW) - KP) - (EW * KQ);
            let BCK = (((((BCC * LY) + (((((((BAK * LR) - (BAJ * LS)) / FV) * LT) - ((((BAK * LU) - (BAJ * LV)) / FV) * LW)) * (AWF / LX)) * KB)) * FW) + (BAK * LZ)) - BCD) - (BAK * EW);
            let MB = (-MA) / FV;
            let MC = MB.exp();
            let MD = (C + (BZ * MC)).sqrt();
            let ME = BY * (C + MD);
            let MF = ME.ln();
            let MG = MA + (KS * MF);
            let BCL = BCK + ((BCF * MF) + (((((((((BCK * BAG) - (BAJ * MB)) / FV) * MC) * BZ) * (AWF / (BCG * MD))) * BY) * (AWF / ME)) * KS));
            let MI = KC / KY;
            let MK = MH * (MI.powf(MJ));
            let BCM = ((((BCH * MI) * BAG) / KY) * (MJ * (MI.powf((MJ - AWF))))) * MH;
            let MM = KZ / LP;
            let MO = MM.powf(MN);
            let BCN = (((BCJ * MM) * BAG) / LP) * (MN * (MM.powf((MN - AWF))));
            let MP = ML * MO;
            let BCO = BCN * ML;
            let MR = MQ * MO;
            let BCP = BCN * MQ;
            let MT = LQ / MG;
            let MV = MS * (MT.powf(MU));
            let BCQ = ((((BCL * MT) * BAG) / MG) * (MU * (MT.powf((MU - AWF))))) * MS;
            let MX = MW * (FW.powf(BO));
            let MY = HM / FV;
            let MZ = MY.exp();
            let NA = MX * MZ;
            let BCR = (((BAK * (BO * (FW.powf((BO - AWF))))) * MW) * MZ) + ((((BBA - (BAJ * MY)) / FV) * MZ) * MX);
            let ND = NB * (FW.powf(NC));
            let BCS = (BAK * (NC * (FW.powf((NC - AWF))))) * NB;
            let NE = -(BL * (C + (FX * JW)));
            let BCT = (((AWU * JW) + ((AWU * JV) * FX)) * BL) * BAG;
            let NF = JY * FV;
            let BCU = (((AWU * JX) * BJ) * FV) + (BAJ * JY);
            let NG = NE / NF;
            let NH = NG.exp();
            let BCV = ((BCT - (BCU * NG)) / NF) * NH;
            let NK = NI * (C + (FX * NJ));
            let BCW = (AWU * NJ) * NI;
            let NN = NL * (C + (FX * NM));
            let BCX = (AWU * NM) * NL;
            let NQ = if NO > NP { 1.0 } else { 0.0 };
            let NT;
            let AXB;
            if NQ != 0.0 {
                let NR = C / NO;
                let BCZ = ((AWW * NR) * BAG) / NO;
                NT = NR;
                AXB = BCZ;
            } else {
                NT = NS;
                AXB = BCY;
            }
            let NV = if NU > NP { 1.0 } else { 0.0 };
            let NX;
            let AXC;
            if NV != 0.0 {
                let NW = C / NU;
                let BDA = ((AWX * NW) * BAG) / NU;
                NX = NW;
                AXC = BDA;
            } else {
                NX = NS;
                AXC = BCY;
            }
            let NZ = if NY > NP { 1.0 } else { 0.0 };
            let OB;
            let AXD;
            if NZ != 0.0 {
                let OA = C / NY;
                let BDB = ((AWY * OA) * BAG) / NY;
                OB = OA;
                AXD = BDB;
            } else {
                OB = NS;
                AXD = BCY;
            }
            let OD = if OC > NP { 1.0 } else { 0.0 };
            let OF;
            let AXE;
            if OD != 0.0 {
                let OE = C / OC;
                let BDC = ((AWZ * OE) * BAG) / OC;
                OF = OE;
                AXE = BDC;
            } else {
                OF = NS;
                AXE = BCY;
            }
            let OG = if GY > NP { 1.0 } else { 0.0 };
            let OI;
            let AXF;
            if OG != 0.0 {
                let OH = C / GY;
                let BDD = ((BAU * OH) * BAG) / GY;
                OI = OH;
                AXF = BDD;
            } else {
                OI = NS;
                AXF = BCY;
            }
            let OK = if OJ > NP { 1.0 } else { 0.0 };
            let OM;
            let AXG;
            if OK != 0.0 {
                let OL = C / OJ;
                let BDE = ((AXA * OL) * BAG) / OJ;
                OM = OL;
                AXG = BDE;
            } else {
                OM = NS;
                AXG = BCY;
            }
            let ON = if HB > NP { 1.0 } else { 0.0 };
            let OP;
            let AXH;
            if ON != 0.0 {
                let OO = C / HB;
                let BDF = ((BAV * OO) * BAG) / HB;
                OP = OO;
                AXH = BDF;
            } else {
                OP = NS;
                AXH = BCY;
            }
            let OQ = if HJ > NP { 1.0 } else { 0.0 };
            let OS;
            let AXI;
            if OQ != 0.0 {
                let OR = C / HJ;
                let BDG = ((BAY * OR) * BAG) / HJ;
                OS = OR;
                AXI = BDG;
            } else {
                OS = NS;
                AXI = BCY;
            }
            let OT = if NK > A { 1.0 } else { 0.0 };
            let OV;
            let AXJ;
            if OT != 0.0 {
                let OU = C / NK;
                let BDH = ((BCW * OU) * BAG) / NK;
                OV = OU;
                AXJ = BDH;
            } else {
                OV = A;
                AXJ = BCY;
            }
            let OW = if NN > A { 1.0 } else { 0.0 };
            let OY;
            let AXK;
            if OW != 0.0 {
                let OX = C / NN;
                let BDI = ((BCX * OX) * BAG) / NN;
                OY = OX;
                AXK = BDI;
            } else {
                OY = A;
                AXK = BCY;
            }
            let OZ = if FZ > A { 1.0 } else { 0.0 };
            let PB;
            let AXL;
            if OZ != 0.0 {
                let PA = C / FZ;
                let BDJ = ((BAL * PA) * BAG) / FZ;
                PB = PA;
                AXL = BDJ;
            } else {
                PB = A;
                AXL = BCY;
            }
            let PC = if ND > A { 1.0 } else { 0.0 };
            let PE;
            let AXM;
            if PC != 0.0 {
                let PD = C / ND;
                let BDK = ((BCS * PD) * BAG) / ND;
                PE = PD;
                AXM = BDK;
            } else {
                PE = A;
                AXM = BCY;
            }
            let PK = PF * (PI - PJ);
            let BDL = (Lanes([AWH, 0.0]) - Lanes([0.0, AWI])) * PF;
            let PM = PF * (PL - PJ);
            let BDM = (Lanes([AWJ, 0.0]) - Lanes([0.0, AWI])) * PF;
            let PO = PF * (PI - PN);
            let BDN = (Lanes([0.0, AWH]) - Lanes([AWK, 0.0])) * PF;
            let PQ = PF * (PI - PP);
            let BDO = (Lanes([0.0, AWH]) - Lanes([AWL, 0.0])) * PF;
            let PR = PF * (PL - PP);
            let BDP = (Lanes([0.0, AWJ]) - Lanes([AWL, 0.0])) * PF;
            let PT = PF * (PL - PS);
            let BDQ = (Lanes([AWJ, 0.0]) - Lanes([0.0, AWM])) * PF;
            let PW = PU - PV;
            let BDR = Lanes([AWN, 0.0]) - Lanes([0.0, AWO]);
            let PX = PF * (PN - PJ);
            let BDS = (Lanes([AWK, 0.0]) - Lanes([0.0, AWI])) * PF;
            let PZ = PU - PY;
            let BDT = Lanes([0.0, AWN]) - Lanes([AWP, 0.0]);
            let QA = PY - PP;
            let BDU = Lanes([AWP, 0.0]) - Lanes([0.0, AWL]);
            let QB = PF * (PP - PN);
            let BDV = (Lanes([AWL, 0.0]) - Lanes([0.0, AWK])) * PF;
            let QC = PU - PL;
            let BDW = Lanes([AWN, 0.0]) - Lanes([0.0, AWJ]);
            let QD = PL - PI;
            let BDX = Lanes([AWJ, 0.0]) - Lanes([0.0, AWH]);
            let QE = PV - PJ;
            let BDY = Lanes([AWO, 0.0]) - Lanes([0.0, AWI]);
            let QF = PS - PP;
            let BDZ = Lanes([0.0, AWM]) - Lanes([AWL, 0.0]);
            let QH = PF * (QG - PS);
            let BEA = (Lanes([0.0, AWQ]) - Lanes([AWM, 0.0])) * PF;
            let QI = PF * (PL - QG);
            let BEB = (Lanes([AWJ, 0.0]) - Lanes([0.0, AWQ])) * PF;
            let QJ = node_potentials[3] - QG;
            let BEC = Lanes([AWR, 0.0]) - Lanes([0.0, AWQ]);
            let QM = -KY;
            let BED = BCH * BAG;
            let QO = QM * QN;
            let BEE = BED * QN;
            let QQ = if QP <= A { 1.0 } else { 0.0 };
            let WW;
            let AXN;
            if QQ != 0.0 {
                let QR = PK + QO;
                let BEP = Lanes([0.0, BDL[0], BDL[1]]);
                let BEQ = BEP + Lanes([BEE, 0.0, 0.0]);
                let QS = if QR > A { 1.0 } else { 0.0 };
                let RI;
                let RJ;
                let AXO;
                let AXP;
                if QS != 0.0 {
                    let QT = C - QN;
                    let QU = QT.powf((-MJ));
                    let QV = C - (QU * QT);
                    let QW = C - MJ;
                    let QX = (KY * QV) / QW;
                    let QY = BY * MJ;
                    let QZ = KY * QT;
                    let RA = (QY * QR) / QZ;
                    let RB = C + RA;
                    let RC = (QR * RB) * QU;
                    let BET = ((BEQ * RB) + ((((BEQ * QY) - Lanes([((BCH * QT) * RA), 0.0, 0.0])) / QZ) * QR)) * QU;
                    let BEU = Lanes([((BCH * QV) / QW), 0.0, 0.0]);
                    RI = QX;
                    RJ = RC;
                    AXO = BEU;
                    AXP = BET;
                } else {
                    let RD = PK / KY;
                    let RE = C - RD;
                    let RF = C - MJ;
                    let RG = C - (RE.powf(RF));
                    let RH = (KY * RG) / RF;
                    let BER = (Lanes([(BCH * RG), 0.0, 0.0]) + ((((((BEP - Lanes([(BCH * RD), 0.0, 0.0])) / KY) * BAG) * (RF * (RE.powf((RF - AWF))))) * BAG) * KY)) / RF;
                    RI = RH;
                    RJ = A;
                    AXO = BER;
                    AXP = BES;
                }
                let RK = RI + RJ;
                let BEV = AXO + AXP;
                WW = RK;
                AXN = BEV;
            } else {
                let BEF = BEE * QO;
                let RL = (BZ * QP) * QP;
                let RM = ((QO * QO) + RL).sqrt();
                let RO = RN * (QO + RM);
                let BEG = (BEE + ((BEF + BEF) * (AWF / (BCG * RM)))) * RN;
                let RP = RO / KY;
                let RQ = C - RP;
                let RR = C - MJ;
                let RS = RQ.powf(RR);
                let BEH = RR - AWF;
                let RT = PK + QO;
                let BEI = Lanes([0.0, BDL[0], BDL[1]]);
                let BEJ = Lanes([BEE, 0.0, 0.0]);
                let BEK = BEI + BEJ;
                let BEL = BEK * RT;
                let RU = ((RT * RT) + RL).sqrt();
                let RV = (BY * (RT - RU)) - QO;
                let BEM = ((BEK - ((BEL + BEL) * (AWF / (BCG * RU)))) * BY) - BEJ;
                let RW = RV / KY;
                let RX = C - RW;
                let RY = RX.powf(RR);
                let RZ = C - QN;
                let SA = RZ.powf((-MJ));
                let SB = (PK - RV) + RO;
                let BEN = (BEI - BEM) + Lanes([BEG, 0.0, 0.0]);
                let SC = SA * SB;
                let SD = BY * MJ;
                let SE = KY * RZ;
                let SF = (SD * SB) / SE;
                let SG = C + SF;
                let SH = (((QM * RY) / RR) + (SC * SG)) - ((QM * RS) / RR);
                let BEO = (((Lanes([(BED * RY), 0.0, 0.0]) + (((((BEM - Lanes([(BCH * RW), 0.0, 0.0])) / KY) * BAG) * (RR * (RX.powf(BEH)))) * QM)) / RR) + (((BEN * SA) * SG) + ((((BEN * SD) - Lanes([((BCH * RZ) * SF), 0.0, 0.0])) / SE) * SC))) - Lanes([(((BED * RS) + (((((BEG - (BCH * RP)) / KY) * BAG) * (RR * (RQ.powf(BEH)))) * QM)) / RR), 0.0, 0.0]);
                WW = SH;
                AXN = BEO;
            }
            let SI = -LP;
            let BEW = BCJ * BAG;
            let SJ = SI * QN;
            let BEX = BEW * QN;
            let SL = if SK <= A { 1.0 } else { 0.0 };
            let WX;
            let AXQ;
            if SL != 0.0 {
                let SM = PO + SJ;
                let BFX = Lanes([0.0, BDN[0], BDN[1]]);
                let BFY = BFX + Lanes([BEX, 0.0, 0.0]);
                let SN = if SM > A { 1.0 } else { 0.0 };
                let TN;
                let TP;
                let AXR;
                let AXS;
                if SN != 0.0 {
                    let SO = C - QN;
                    let SP = SO.powf((-1e0f64 - MN));
                    let SQ = C - ((SP * SO) * SO);
                    let SR = C - MN;
                    let SS = (LP * SQ) / SR;
                    let ST = BY * MN;
                    let SU = (ST * SM) / LP;
                    let SV = SO + SU;
                    let SW = (SM * SV) * SP;
                    let BGD = ((BFY * SV) + ((((BFY * ST) - Lanes([(BCJ * SU), 0.0, 0.0])) / LP) * SM)) * SP;
                    let BGE = Lanes([((BCJ * SQ) / SR), 0.0, 0.0]);
                    TN = SS;
                    TP = SW;
                    AXR = BGE;
                    AXS = BGD;
                } else {
                    let SY = if (if SX > A { 1.0 } else { 0.0 }) != 0.0 && (if PO < (-SX) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let TO;
                    let AXT;
                    if SY != 0.0 {
                        let SZ = SX / LP;
                        let TA = C + SZ;
                        let TB = C - MN;
                        let TC = TA.powf(TB);
                        let BGA = BDN * TB;
                        let TD = LP + SX;
                        let TE = (TB * (PO + SX)) / TD;
                        let TF = C - TE;
                        let TG = C - (TC * TF);
                        let TH = (LP * TG) / TB;
                        let BGB = (Lanes([(BCJ * TG), 0.0, 0.0]) + (((Lanes([(((((BCJ * SZ) * BAG) / LP) * (TB * (TA.powf((TB - AWF))))) * TF), 0.0, 0.0]) + ((((Lanes([0.0, BGA[0], BGA[1]]) - Lanes([(BCJ * TE), 0.0, 0.0])) / TD) * BAG) * TC)) * BAG) * LP)) / TB;
                        TO = TH;
                        AXT = BGB;
                    } else {
                        let TI = PO / LP;
                        let TJ = C - TI;
                        let TK = C - MN;
                        let TL = C - (TJ.powf(TK));
                        let TM = (LP * TL) / TK;
                        let BFZ = (Lanes([(BCJ * TL), 0.0, 0.0]) + ((((((BFX - Lanes([(BCJ * TI), 0.0, 0.0])) / LP) * BAG) * (TK * (TJ.powf((TK - AWF))))) * BAG) * LP)) / TK;
                        TO = TM;
                        AXT = BFZ;
                    }
                    TN = TO;
                    TP = A;
                    AXR = AXT;
                    AXS = BGC;
                }
                let TQ = TN + TP;
                let BGF = AXR + AXS;
                WX = TQ;
                AXQ = BGF;
            } else {
                let TS = if (if SX > A { 1.0 } else { 0.0 }) != 0.0 && (if TR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let WY;
                let AXU;
                if TS != 0.0 {
                    let TT = SX - SJ;
                    let BFH = BEX * BAG;
                    let TU = (SX + SJ) / TT;
                    let BFI = (BEX - (BFH * TU)) / TT;
                    let TV = TU - C;
                    let BFJ = BFI * TV;
                    let TW = (BZ * SK) * SK;
                    let TX = ((TV * TV) + TW).sqrt();
                    let TY = TU + C;
                    let BFK = BFI * TY;
                    let TZ = (BZ * TR) * TR;
                    let UA = ((TY * TY) + TZ).sqrt();
                    let UB = TX + UA;
                    let UC = (JZ * TU) / UB;
                    let UD = BY * (((UC * TT) - SX) - SJ);
                    let BFL = ((((((BFI * JZ) - ((((BFJ + BFJ) * (AWF / (BCG * TX))) + ((BFK + BFK) * (AWF / (BCG * UA)))) * UC)) / UB) * TT) + (BFH * UC)) - BEX) * BY;
                    let UE = UD / LP;
                    let UF = C - UE;
                    let UG = C - MN;
                    let BFM = UG - AWF;
                    let UH = C - (UF.powf(UG));
                    let BFN = BDN * JZ;
                    let BFO = Lanes([BEX, 0.0, 0.0]);
                    let UI = (((JZ * PO) + SX) + SJ) / TT;
                    let BFP = ((Lanes([0.0, BFN[0], BFN[1]]) + BFO) - Lanes([(BFH * UI), 0.0, 0.0])) / TT;
                    let UJ = UI - C;
                    let BFQ = BFP * UJ;
                    let UK = ((UJ * UJ) + TW).sqrt();
                    let UL = UI + C;
                    let BFR = BFP * UL;
                    let UM = ((UL * UL) + TZ).sqrt();
                    let UN = UK + UM;
                    let UO = (JZ * UI) / UN;
                    let BFS = ((BFP * JZ) - ((((BFQ + BFQ) * (AWF / (BCG * UK))) + ((BFR + BFR) * (AWF / (BCG * UM)))) * UO)) / UN;
                    let UP = BY * (((UO * TT) - SX) - SJ);
                    let BFT = (((BFS * TT) + Lanes([(BFH * UO), 0.0, 0.0])) - BFO) * BY;
                    let UQ = UP / LP;
                    let UR = C - UQ;
                    let US = C - (UR.powf(UG));
                    let UT = BY * (UO + C);
                    let BFU = BFS * BY;
                    let UU = SX / LP;
                    let UV = C + UU;
                    let UW = -MN;
                    let UX = UV.powf(UW);
                    let BFV = UW - AWF;
                    let UY = SJ / LP;
                    let UZ = C + UY;
                    let VA = UZ.powf(UW);
                    let VB = C - UT;
                    let VC = (VB * UX) + (UT * VA);
                    let VD = (PO - UP) + UD;
                    let VE = ((VD * VC) + ((LP * US) / UG)) - ((LP * UH) / UG);
                    let BFW = (((((Lanes([0.0, BDN[0], BDN[1]]) - BFT) + Lanes([BFL, 0.0, 0.0])) * VC) + (((((BFU * BAG) * UX) + Lanes([(((((BCJ * UU) * BAG) / LP) * (UW * (UV.powf(BFV)))) * VB), 0.0, 0.0])) + ((BFU * VA) + Lanes([((((BEX - (BCJ * UY)) / LP) * (UW * (UZ.powf(BFV)))) * UT), 0.0, 0.0]))) * VD)) + ((Lanes([(BCJ * US), 0.0, 0.0]) + ((((((BFT - Lanes([(BCJ * UQ), 0.0, 0.0])) / LP) * BAG) * (UG * (UR.powf(BFM)))) * BAG) * LP)) / UG)) - Lanes([(((BCJ * UH) + ((((((BFL - (BCJ * UE)) / LP) * BAG) * (UG * (UF.powf(BFM)))) * BAG) * LP)) / UG), 0.0, 0.0]);
                    WY = VE;
                    AXU = BFW;
                } else {
                    let BEY = BEX * SJ;
                    let VF = (BZ * SK) * SK;
                    let VG = ((SJ * SJ) + VF).sqrt();
                    let VI = VH * (SJ + VG);
                    let BEZ = (BEX + ((BEY + BEY) * (AWF / (BCG * VG)))) * VH;
                    let VJ = VI / LP;
                    let VK = C - VJ;
                    let VL = C - MN;
                    let VM = VK.powf(VL);
                    let BFA = VL - AWF;
                    let VN = PO + SJ;
                    let BFB = Lanes([0.0, BDN[0], BDN[1]]);
                    let BFC = Lanes([BEX, 0.0, 0.0]);
                    let BFD = BFB + BFC;
                    let BFE = BFD * VN;
                    let VO = ((VN * VN) + VF).sqrt();
                    let VP = (BY * (VN - VO)) - SJ;
                    let BFF = ((BFD - ((BFE + BFE) * (AWF / (BCG * VO)))) * BY) - BFC;
                    let VQ = VP / LP;
                    let VR = C - VQ;
                    let VS = VR.powf(VL);
                    let VT = (C - QN).powf((-MN));
                    let VU = (((SI * VS) / VL) + (VT * ((PO - VP) + VI))) - ((SI * VM) / VL);
                    let BFG = (((Lanes([(BEW * VS), 0.0, 0.0]) + (((((BFF - Lanes([(BCJ * VQ), 0.0, 0.0])) / LP) * BAG) * (VL * (VR.powf(BFA)))) * SI)) / VL) + (((BFB - BFF) + Lanes([BEZ, 0.0, 0.0])) * VT)) - Lanes([(((BEW * VM) + (((((BEZ - (BCJ * VJ)) / LP) * BAG) * (VL * (VK.powf(BFA)))) * SI)) / VL), 0.0, 0.0]);
                    WY = VU;
                    AXU = BFG;
                }
                WX = WY;
                AXQ = AXU;
            }
            let VV = JN * FV;
            let VW = C / VV;
            let BGG = ((((BBY * FV) + (BAJ * JN)) * VW) * BAG) / VV;
            let VZ = if PK < VX { 1.0 } else { 0.0 };
            let WF;
            let AXV;
            if VZ != 0.0 {
                let BGJ = BDL * VW;
                let WA = (PK * VW).exp();
                let BGK = (Lanes([0.0, BGJ[0], BGJ[1]]) + Lanes([(BGG * PK), 0.0, 0.0])) * WA;
                WF = WA;
                AXV = BGK;
            } else {
                let WB = (VX * VW).exp();
                let WC = PK - VX;
                let BGH = BDL * VW;
                let WD = C + (WC * VW);
                let WE = WB * WD;
                let BGI = Lanes([(((BGG * VX) * WB) * WD), 0.0, 0.0]) + ((Lanes([0.0, BGH[0], BGH[1]]) + Lanes([(BGG * WC), 0.0, 0.0])) * WB);
                WF = WE;
                AXV = BGI;
            }
            let WG = WF - C;
            let WH = HQ * WG;
            let BGL = Lanes([(BBB * WG), 0.0, 0.0]) + (AXV * HQ);
            let WI = JO * FV;
            let WJ = C / WI;
            let BGM = ((((BBZ * FV) + (BAJ * JO)) * WJ) * BAG) / WI;
            let WM = if PO < WK { 1.0 } else { 0.0 };
            let WT;
            let AXW;
            if WM != 0.0 {
                let BGP = BDN * WJ;
                let WN = (PO * WJ).exp();
                let BGQ = (Lanes([0.0, BGP[0], BGP[1]]) + Lanes([(BGM * PO), 0.0, 0.0])) * WN;
                WT = WN;
                AXW = BGQ;
            } else {
                let WO = (WK * WJ).exp();
                let WP = PO - WK;
                let BGN = BDN * WJ;
                let WQ = C + (WP * WJ);
                let WR = WO * WQ;
                let BGO = Lanes([(((BGM * WK) * WO) * WQ), 0.0, 0.0]) + ((Lanes([0.0, BGN[0], BGN[1]]) + Lanes([(BGM * WP), 0.0, 0.0])) * WO);
                WT = WR;
                AXW = BGO;
            }
            let WS = HQ * HV;
            let WU = WT - C;
            let WV = WS * WU;
            let BGR = Lanes([(((BBB * HV) + (BBC * HQ)) * WU), 0.0, 0.0]) + (AXW * WS);
            let BGS = (AXN * OY) + Lanes([(AXK * WW), 0.0, 0.0]);
            let BGT = (AXQ * OV) + Lanes([(AXJ * WX), 0.0, 0.0]);
            let BGU = Lanes([BGS[0], 0.0, BGS[1], BGS[2]]) + Lanes([BGT[0], BGT[1], BGT[2], 0.0]);
            let XA = ((C + (WW * OY)) + (WX * OV)) - WZ;
            let BGV = BGU * XA;
            let XC = ((XA * XA) + XB).sqrt();
            let BGW = (((BGV + BGV) * (AWF / (BCG * XC))) + BGU) * BY;
            let XD = (BY * (XC + XA)) + WZ;
            let BGX = (BGL * PB) + Lanes([(AXL * WH), 0.0, 0.0]);
            let BGY = BGR * U;
            let XE = (WH * PB) + (WV * U);
            let BGZ = Lanes([BGX[0], 0.0, BGX[1], BGX[2]]) + Lanes([BGY[0], BGY[1], BGY[2], 0.0]);
            let XF = if parameters[30] < BY { 1.0 } else { 0.0 };
            let XS;
            let AXX;
            if XF != 0.0 {
                let XG = C / CA;
                let XH = (XD.powf(XG)) + (BZ * XE);
                let BHD = (BGW * (XG * (XD.powf((XG - AWF))))) + (BGZ * BZ);
                let XI = if XH > XB { 1.0 } else { 0.0 };
                let XT;
                let AXY;
                if XI != 0.0 {
                    let XJ = BY * (XD + (XH.powf(CA)));
                    let BHF = (BGW + (BHD * (CA * (XH.powf((CA - AWF)))))) * BY;
                    XT = XJ;
                    AXY = BHF;
                } else {
                    let XK = BY * (XD + (XB.powf(CA)));
                    let BHE = BGW * BY;
                    XT = XK;
                    AXY = BHE;
                }
                XS = XT;
                AXX = AXY;
            } else {
                let BHA = BGZ * BZ;
                let XL = C + (BZ * XE);
                let XM = if XL > XB { 1.0 } else { 0.0 };
                let XU;
                let AXZ;
                if XM != 0.0 {
                    let XN = BY * XD;
                    let XO = C + (XL.powf(CA));
                    let XP = XN * XO;
                    let BHC = ((BGW * BY) * XO) + ((BHA * (CA * (XL.powf((CA - AWF))))) * XN);
                    XU = XP;
                    AXZ = BHC;
                } else {
                    let XQ = C + (XB.powf(CA));
                    let XR = (BY * XD) * XQ;
                    let BHB = (BGW * BY) * XQ;
                    XU = XR;
                    AXZ = BHB;
                }
                XS = XU;
                AXX = AXZ;
            }
            let XV = WV / XS;
            let BHG = (Lanes([BGR[0], BGR[1], BGR[2], 0.0]) - (AXX * XV)) / XS;
            let XW = WH / XS;
            let BHH = (Lanes([BGL[0], 0.0, BGL[1], BGL[2]]) - (AXX * XW)) / XS;
            let XX = if CN > A { 1.0 } else { 0.0 };
            let AHN;
            let ALI;
            let ATV;
            let AYA;
            let AYB;
            let AYC;
            if XX != 0.0 {
                let XY = C / HX;
                let BHK = ((BBD * XY) * BAG) / HX;
                let YB = if PT < XZ { 1.0 } else { 0.0 };
                let YO;
                let AYD;
                if YB != 0.0 {
                    let BHN = BDQ * XY;
                    let YC = (PT * XY).exp();
                    let BHO = (Lanes([0.0, BHN[0], BHN[1]]) + Lanes([(BHK * PT), 0.0, 0.0])) * YC;
                    YO = YC;
                    AYD = BHO;
                } else {
                    let YD = (XZ * XY).exp();
                    let YE = PT - XZ;
                    let BHL = BDQ * XY;
                    let YF = C + (YE * XY);
                    let YG = YD * YF;
                    let BHM = Lanes([(((BHK * XZ) * YD) * YF), 0.0, 0.0]) + ((Lanes([0.0, BHL[0], BHL[1]]) + Lanes([(BHK * YE), 0.0, 0.0])) * YD);
                    YO = YG;
                    AYD = BHM;
                }
                let YH = if PO < XZ { 1.0 } else { 0.0 };
                let YQ;
                let AYE;
                if YH != 0.0 {
                    let BHR = BDN * XY;
                    let YI = (PO * XY).exp();
                    let BHS = (Lanes([0.0, BHR[0], BHR[1]]) + Lanes([(BHK * PO), 0.0, 0.0])) * YI;
                    YQ = YI;
                    AYE = BHS;
                } else {
                    let YJ = (XZ * XY).exp();
                    let YK = PO - XZ;
                    let BHP = BDN * XY;
                    let YL = C + (YK * XY);
                    let YM = YJ * YL;
                    let BHQ = Lanes([(((BHK * XZ) * YJ) * YL), 0.0, 0.0]) + ((Lanes([0.0, BHP[0], BHP[1]]) + Lanes([(BHK * YK), 0.0, 0.0])) * YJ);
                    YQ = YM;
                    AYE = BHQ;
                }
                let BHT = AYD * YN;
                let YP = C - YN;
                let BHU = AYE * YP;
                let YR = ((YN * YO) + (YP * YQ)) - C;
                let YS = IA * YR;
                let BHV = Lanes([(BBE * YR), 0.0, 0.0, 0.0, 0.0]) + ((Lanes([BHT[0], 0.0, BHT[1], 0.0, BHT[2]]) + Lanes([BHU[0], BHU[1], 0.0, BHU[2], 0.0])) * IA);
                let BHW = (BHV * Y) * BZ;
                let YT = C + (BZ * (YS * Y));
                let YU = if YT > XB { 1.0 } else { 0.0 };
                let ZG;
                let AYF;
                if YU != 0.0 {
                    let YV = YT.sqrt();
                    let YW = BY * (C + YV);
                    let BHX = (BHW * (AWF / (BCG * YV))) * BY;
                    ZG = YW;
                    AYF = BHX;
                } else {
                    ZG = YX;
                    AYF = BHI;
                }
                let YY = if QH < XZ { 1.0 } else { 0.0 };
                let ZE;
                let AYG;
                if YY != 0.0 {
                    let BIA = BEA * XY;
                    let YZ = (QH * XY).exp();
                    let BIB = (Lanes([0.0, BIA[0], BIA[1]]) + Lanes([(BHK * QH), 0.0, 0.0])) * YZ;
                    ZE = YZ;
                    AYG = BIB;
                } else {
                    let ZA = (XZ * XY).exp();
                    let ZB = QH - XZ;
                    let BHY = BEA * XY;
                    let ZC = C + (ZB * XY);
                    let ZD = ZA * ZC;
                    let BHZ = Lanes([(((BHK * XZ) * ZA) * ZC), 0.0, 0.0]) + ((Lanes([0.0, BHY[0], BHY[1]]) + Lanes([(BHK * ZB), 0.0, 0.0])) * ZA);
                    ZE = ZD;
                    AYG = BHZ;
                }
                let ZF = ZE - C;
                let BIC = Lanes([(BBE * ZF), 0.0, 0.0]) + (AYG * IA);
                let ZH = (YS - (IA * ZF)) / ZG;
                let BID = AYF * ZH;
                let BIE = ((Lanes([BHV[0], BHV[1], BHV[2], BHV[3], BHV[4], 0.0]) - Lanes([BIC[0], 0.0, 0.0, 0.0, BIC[1], BIC[2]])) - Lanes([BID[0], BID[1], BID[2], BID[3], BID[4], 0.0])) / ZG;
                AHN = ZG;
                ALI = ZH;
                ATV = YS;
                AYA = AYF;
                AYB = BIE;
                AYC = BHV;
            } else {
                AHN = C;
                ALI = A;
                ATV = A;
                AYA = BHI;
                AYB = BHJ;
                AYC = BHI;
            }
            let ZJ = if ZI == C { 1.0 } else { 0.0 };
            let AKX;
            let ALC;
            let AYH;
            let AYI;
            if ZJ != 0.0 {
                let ZK = C / IC;
                let BKJ = ((BBF * ZK) * BAG) / IC;
                let ZM = if PK < ZL { 1.0 } else { 0.0 };
                let AAE;
                let AYJ;
                if ZM != 0.0 {
                    let BKM = BDL * ZK;
                    let ZN = (PK * ZK).exp();
                    let BKN = (Lanes([0.0, BKM[0], BKM[1]]) + Lanes([(BKJ * PK), 0.0, 0.0])) * ZN;
                    AAE = ZN;
                    AYJ = BKN;
                } else {
                    let ZO = (ZL * ZK).exp();
                    let ZP = PK - ZL;
                    let BKK = BDL * ZK;
                    let ZQ = C + (ZP * ZK);
                    let ZR = ZO * ZQ;
                    let BKL = Lanes([(((BKJ * ZL) * ZO) * ZQ), 0.0, 0.0]) + ((Lanes([0.0, BKK[0], BKK[1]]) + Lanes([(BKJ * ZP), 0.0, 0.0])) * ZO);
                    AAE = ZR;
                    AYJ = BKL;
                }
                let ZS = C / IH;
                let BKO = ((BBH * ZS) * BAG) / IH;
                let ZU = if PK < ZT { 1.0 } else { 0.0 };
                let AAG;
                let AYK;
                if ZU != 0.0 {
                    let BKR = BDL * ZS;
                    let ZV = (PK * ZS).exp();
                    let BKS = (Lanes([0.0, BKR[0], BKR[1]]) + Lanes([(BKO * PK), 0.0, 0.0])) * ZV;
                    AAG = ZV;
                    AYK = BKS;
                } else {
                    let ZW = (ZT * ZS).exp();
                    let ZX = PK - ZT;
                    let BKP = BDL * ZS;
                    let ZY = C + (ZX * ZS);
                    let ZZ = ZW * ZY;
                    let BKQ = Lanes([(((BKO * ZT) * ZW) * ZY), 0.0, 0.0]) + ((Lanes([0.0, BKP[0], BKP[1]]) + Lanes([(BKO * ZX), 0.0, 0.0])) * ZW);
                    AAG = ZZ;
                    AYK = BKQ;
                }
                let AAB = if AAA > A { 1.0 } else { 0.0 };
                let AAW;
                let AYL;
                if AAB != 0.0 {
                    let AAC = C + (AAA * (XD - C));
                    let AAD = IF * AAC;
                    let AAF = AAE - C;
                    let BKV = AYJ * AAD;
                    let AAH = AAG - C;
                    let BKW = Lanes([(BBI * AAH), 0.0, 0.0]) + (AYK * IK);
                    let AAI = (AAD * AAF) + (IK * AAH);
                    let BKX = (((Lanes([(BBG * AAC), 0.0, 0.0, 0.0]) + ((BGW * AAA) * IF)) * AAF) + Lanes([BKV[0], 0.0, BKV[1], BKV[2]])) + Lanes([BKW[0], 0.0, BKW[1], BKW[2]]);
                    AAW = AAI;
                    AYL = BKX;
                } else {
                    let AAJ = AAE - C;
                    let AAK = AAG - C;
                    let AAL = (IF * AAJ) + (IK * AAK);
                    let BKT = (Lanes([(BBG * AAJ), 0.0, 0.0]) + (AYJ * IF)) + (Lanes([(BBI * AAK), 0.0, 0.0]) + (AYK * IK));
                    let BKU = Lanes([BKT[0], 0.0, BKT[1], BKT[2]]);
                    AAW = AAL;
                    AYL = BKU;
                }
                let AAM = if BL > A { 1.0 } else { 0.0 };
                let AKY;
                let AYM;
                if AAM != 0.0 {
                    let AAN = NE - PK;
                    let BKY = Lanes([BCT, 0.0, 0.0]) - Lanes([0.0, BDL[0], BDL[1]]);
                    let AAO = C / NF;
                    let BKZ = ((BCU * AAO) * BAG) / NF;
                    let AAQ = if AAN < AAP { 1.0 } else { 0.0 };
                    let AAX;
                    let AYN;
                    if AAQ != 0.0 {
                        let AAR = (AAN * AAO).exp();
                        let BLB = ((BKY * AAO) + Lanes([(BKZ * AAN), 0.0, 0.0])) * AAR;
                        AAX = AAR;
                        AYN = BLB;
                    } else {
                        let AAS = (AAP * AAO).exp();
                        let AAT = AAN - AAP;
                        let AAU = C + (AAT * AAO);
                        let AAV = AAS * AAU;
                        let BLA = Lanes([(((BKZ * AAP) * AAS) * AAU), 0.0, 0.0]) + (((BKY * AAO) + Lanes([(BKZ * AAT), 0.0, 0.0])) * AAS);
                        AAX = AAV;
                        AYN = BLA;
                    }
                    let BLC = (AYN - Lanes([BCV, 0.0, 0.0])) * BH;
                    let AAY = AAW - (BH * (AAX - NH));
                    let BLD = AYL - Lanes([BLC[0], 0.0, BLC[1], BLC[2]]);
                    AKY = AAY;
                    AYM = BLD;
                } else {
                    AKY = AAW;
                    AYM = AYL;
                }
                AKX = AKY;
                ALC = A;
                AYH = AYM;
                AYI = BLE;
            } else {
                let AAZ = if ZI == A { 1.0 } else { 0.0 };
                let AKZ;
                let ALD;
                let AYO;
                let AYP;
                if AAZ != 0.0 {
                    let ABA = C / IC;
                    let BJQ = ((BBF * ABA) * BAG) / IC;
                    let ABB = if PM < ZL { 1.0 } else { 0.0 };
                    let ABO;
                    let AYQ;
                    if ABB != 0.0 {
                        let BJT = BDM * ABA;
                        let ABC = (PM * ABA).exp();
                        let BJU = (Lanes([0.0, BJT[0], BJT[1]]) + Lanes([(BJQ * PM), 0.0, 0.0])) * ABC;
                        ABO = ABC;
                        AYQ = BJU;
                    } else {
                        let ABD = (ZL * ABA).exp();
                        let ABE = PM - ZL;
                        let BJR = BDM * ABA;
                        let ABF = C + (ABE * ABA);
                        let ABG = ABD * ABF;
                        let BJS = Lanes([(((BJQ * ZL) * ABD) * ABF), 0.0, 0.0]) + ((Lanes([0.0, BJR[0], BJR[1]]) + Lanes([(BJQ * ABE), 0.0, 0.0])) * ABD);
                        ABO = ABG;
                        AYQ = BJS;
                    }
                    let ABH = C / IH;
                    let BJV = ((BBH * ABH) * BAG) / IH;
                    let ABI = if PM < ZT { 1.0 } else { 0.0 };
                    let ABQ;
                    let AYR;
                    if ABI != 0.0 {
                        let BJY = BDM * ABH;
                        let ABJ = (PM * ABH).exp();
                        let BJZ = (Lanes([0.0, BJY[0], BJY[1]]) + Lanes([(BJV * PM), 0.0, 0.0])) * ABJ;
                        ABQ = ABJ;
                        AYR = BJZ;
                    } else {
                        let ABK = (ZT * ABH).exp();
                        let ABL = PM - ZT;
                        let BJW = BDM * ABH;
                        let ABM = C + (ABL * ABH);
                        let ABN = ABK * ABM;
                        let BJX = Lanes([(((BJV * ZT) * ABK) * ABM), 0.0, 0.0]) + ((Lanes([0.0, BJW[0], BJW[1]]) + Lanes([(BJV * ABL), 0.0, 0.0])) * ABK);
                        ABQ = ABN;
                        AYR = BJX;
                    }
                    let ABP = ABO - C;
                    let ABR = ABQ - C;
                    let ABS = (IF * ABP) + (IK * ABR);
                    let BKA = (Lanes([(BBG * ABP), 0.0, 0.0]) + (AYQ * IF)) + (Lanes([(BBI * ABR), 0.0, 0.0]) + (AYR * IK));
                    let ABT = if BL > A { 1.0 } else { 0.0 };
                    let ALE;
                    let AYS;
                    if ABT != 0.0 {
                        let ABU = NE - PK;
                        let BKC = Lanes([BCT, 0.0, 0.0]) - Lanes([0.0, BDL[0], BDL[1]]);
                        let ABV = C / NF;
                        let BKD = ((BCU * ABV) * BAG) / NF;
                        let ABW = if ABU < AAP { 1.0 } else { 0.0 };
                        let ACC;
                        let AYT;
                        if ABW != 0.0 {
                            let ABX = (ABU * ABV).exp();
                            let BKF = ((BKC * ABV) + Lanes([(BKD * ABU), 0.0, 0.0])) * ABX;
                            ACC = ABX;
                            AYT = BKF;
                        } else {
                            let ABY = (AAP * ABV).exp();
                            let ABZ = ABU - AAP;
                            let ACA = C + (ABZ * ABV);
                            let ACB = ABY * ACA;
                            let BKE = Lanes([(((BKD * AAP) * ABY) * ACA), 0.0, 0.0]) + (((BKC * ABV) + Lanes([(BKD * ABZ), 0.0, 0.0])) * ABY);
                            ACC = ACB;
                            AYT = BKE;
                        }
                        let BKG = (AYT - Lanes([BCV, 0.0, 0.0])) * BH;
                        let ACD = ABS - (BH * (ACC - NH));
                        let BKH = Lanes([BKA[0], BKA[1], 0.0, BKA[2]]) - Lanes([BKG[0], 0.0, BKG[1], BKG[2]]);
                        ALE = ACD;
                        AYS = BKH;
                    } else {
                        let BKB = Lanes([BKA[0], BKA[1], 0.0, BKA[2]]);
                        ALE = ABS;
                        AYS = BKB;
                    }
                    AKZ = A;
                    ALD = ALE;
                    AYO = BKI;
                    AYP = AYS;
                } else {
                    let ACE = C / IC;
                    let BIF = ((BBF * ACE) * BAG) / IC;
                    let ACF = if PK < ZL { 1.0 } else { 0.0 };
                    let ACV;
                    let AYU;
                    if ACF != 0.0 {
                        let BII = BDL * ACE;
                        let ACG = (PK * ACE).exp();
                        let BIJ = (Lanes([0.0, BII[0], BII[1]]) + Lanes([(BIF * PK), 0.0, 0.0])) * ACG;
                        ACV = ACG;
                        AYU = BIJ;
                    } else {
                        let ACH = (ZL * ACE).exp();
                        let ACI = PK - ZL;
                        let BIG = BDL * ACE;
                        let ACJ = C + (ACI * ACE);
                        let ACK = ACH * ACJ;
                        let BIH = Lanes([(((BIF * ZL) * ACH) * ACJ), 0.0, 0.0]) + ((Lanes([0.0, BIG[0], BIG[1]]) + Lanes([(BIF * ACI), 0.0, 0.0])) * ACH);
                        ACV = ACK;
                        AYU = BIH;
                    }
                    let ACL = C / IH;
                    let BIK = ((BBH * ACL) * BAG) / IH;
                    let ACM = if PK < ZT { 1.0 } else { 0.0 };
                    let ACX;
                    let AYV;
                    if ACM != 0.0 {
                        let BIN = BDL * ACL;
                        let ACN = (PK * ACL).exp();
                        let BIO = (Lanes([0.0, BIN[0], BIN[1]]) + Lanes([(BIK * PK), 0.0, 0.0])) * ACN;
                        ACX = ACN;
                        AYV = BIO;
                    } else {
                        let ACO = (ZT * ACL).exp();
                        let ACP = PK - ZT;
                        let BIL = BDL * ACL;
                        let ACQ = C + (ACP * ACL);
                        let ACR = ACO * ACQ;
                        let BIM = Lanes([(((BIK * ZT) * ACO) * ACQ), 0.0, 0.0]) + ((Lanes([0.0, BIL[0], BIL[1]]) + Lanes([(BIK * ACP), 0.0, 0.0])) * ACO);
                        ACX = ACR;
                        AYV = BIM;
                    }
                    let ACS = if AAA > A { 1.0 } else { 0.0 };
                    let ADM;
                    let AYW;
                    if ACS != 0.0 {
                        let ACT = C + (AAA * (XD - C));
                        let ACU = IF * ACT;
                        let ACW = ACV - C;
                        let BIR = AYU * ACU;
                        let ACY = ACX - C;
                        let BIS = Lanes([(BBI * ACY), 0.0, 0.0]) + (AYV * IK);
                        let ACZ = ZI * ((ACU * ACW) + (IK * ACY));
                        let BIT = ((((Lanes([(BBG * ACT), 0.0, 0.0, 0.0]) + ((BGW * AAA) * IF)) * ACW) + Lanes([BIR[0], 0.0, BIR[1], BIR[2]])) + Lanes([BIS[0], 0.0, BIS[1], BIS[2]])) * ZI;
                        ADM = ACZ;
                        AYW = BIT;
                    } else {
                        let ADA = ACV - C;
                        let ADB = ACX - C;
                        let ADC = ZI * ((IF * ADA) + (IK * ADB));
                        let BIP = ((Lanes([(BBG * ADA), 0.0, 0.0]) + (AYU * IF)) + (Lanes([(BBI * ADB), 0.0, 0.0]) + (AYV * IK))) * ZI;
                        let BIQ = Lanes([BIP[0], 0.0, BIP[1], BIP[2]]);
                        ADM = ADC;
                        AYW = BIQ;
                    }
                    let ADD = if BL > A { 1.0 } else { 0.0 };
                    let ALA;
                    let AYX;
                    if ADD != 0.0 {
                        let ADE = NE - PK;
                        let BIU = Lanes([BCT, 0.0, 0.0]) - Lanes([0.0, BDL[0], BDL[1]]);
                        let ADF = C / NF;
                        let BIV = ((BCU * ADF) * BAG) / NF;
                        let ADG = if ADE < AAP { 1.0 } else { 0.0 };
                        let ADO;
                        let AYY;
                        if ADG != 0.0 {
                            let ADH = (ADE * ADF).exp();
                            let BIX = ((BIU * ADF) + Lanes([(BIV * ADE), 0.0, 0.0])) * ADH;
                            ADO = ADH;
                            AYY = BIX;
                        } else {
                            let ADI = (AAP * ADF).exp();
                            let ADJ = ADE - AAP;
                            let ADK = C + (ADJ * ADF);
                            let ADL = ADI * ADK;
                            let BIW = Lanes([(((BIV * AAP) * ADI) * ADK), 0.0, 0.0]) + (((BIU * ADF) + Lanes([(BIV * ADJ), 0.0, 0.0])) * ADI);
                            ADO = ADL;
                            AYY = BIW;
                        }
                        let ADN = ZI * BH;
                        let BIY = (AYY - Lanes([BCV, 0.0, 0.0])) * ADN;
                        let ADP = ADM - (ADN * (ADO - NH));
                        let BIZ = AYW - Lanes([BIY[0], 0.0, BIY[1], BIY[2]]);
                        ALA = ADP;
                        AYX = BIZ;
                    } else {
                        ALA = ADM;
                        AYX = AYW;
                    }
                    let ADQ = if PM < ZL { 1.0 } else { 0.0 };
                    let AED;
                    let AYZ;
                    if ADQ != 0.0 {
                        let BJC = BDM * ACE;
                        let ADR = (PM * ACE).exp();
                        let BJD = (Lanes([0.0, BJC[0], BJC[1]]) + Lanes([(BIF * PM), 0.0, 0.0])) * ADR;
                        AED = ADR;
                        AYZ = BJD;
                    } else {
                        let ADS = (ZL * ACE).exp();
                        let ADT = PM - ZL;
                        let BJA = BDM * ACE;
                        let ADU = C + (ADT * ACE);
                        let ADV = ADS * ADU;
                        let BJB = Lanes([(((BIF * ZL) * ADS) * ADU), 0.0, 0.0]) + ((Lanes([0.0, BJA[0], BJA[1]]) + Lanes([(BIF * ADT), 0.0, 0.0])) * ADS);
                        AED = ADV;
                        AYZ = BJB;
                    }
                    let ADW = if PM < ZT { 1.0 } else { 0.0 };
                    let AEF;
                    let AZA;
                    if ADW != 0.0 {
                        let BJG = BDM * ACL;
                        let ADX = (PM * ACL).exp();
                        let BJH = (Lanes([0.0, BJG[0], BJG[1]]) + Lanes([(BIK * PM), 0.0, 0.0])) * ADX;
                        AEF = ADX;
                        AZA = BJH;
                    } else {
                        let ADY = (ZT * ACL).exp();
                        let ADZ = PM - ZT;
                        let BJE = BDM * ACL;
                        let AEA = C + (ADZ * ACL);
                        let AEB = ADY * AEA;
                        let BJF = Lanes([(((BIK * ZT) * ADY) * AEA), 0.0, 0.0]) + ((Lanes([0.0, BJE[0], BJE[1]]) + Lanes([(BIK * ADZ), 0.0, 0.0])) * ADY);
                        AEF = AEB;
                        AZA = BJF;
                    }
                    let AEC = C - ZI;
                    let AEE = AED - C;
                    let AEG = AEF - C;
                    let AEH = AEC * ((IF * AEE) + (IK * AEG));
                    let BJI = ((Lanes([(BBG * AEE), 0.0, 0.0]) + (AYZ * IF)) + (Lanes([(BBI * AEG), 0.0, 0.0]) + (AZA * IK))) * AEC;
                    let ALF;
                    let AZB;
                    if ADD != 0.0 {
                        let AEI = NE - PK;
                        let BJK = Lanes([BCT, 0.0, 0.0]) - Lanes([0.0, BDL[0], BDL[1]]);
                        let AEJ = C / NF;
                        let BJL = ((BCU * AEJ) * BAG) / NF;
                        let AEK = if AEI < AAP { 1.0 } else { 0.0 };
                        let AER;
                        let AZC;
                        if AEK != 0.0 {
                            let AEL = (AEI * AEJ).exp();
                            let BJN = ((BJK * AEJ) + Lanes([(BJL * AEI), 0.0, 0.0])) * AEL;
                            AER = AEL;
                            AZC = BJN;
                        } else {
                            let AEM = (AAP * AEJ).exp();
                            let AEN = AEI - AAP;
                            let AEO = C + (AEN * AEJ);
                            let AEP = AEM * AEO;
                            let BJM = Lanes([(((BJL * AAP) * AEM) * AEO), 0.0, 0.0]) + (((BJK * AEJ) + Lanes([(BJL * AEN), 0.0, 0.0])) * AEM);
                            AER = AEP;
                            AZC = BJM;
                        }
                        let AEQ = AEC * BH;
                        let BJO = (AZC - Lanes([BCV, 0.0, 0.0])) * AEQ;
                        let AES = AEH - (AEQ * (AER - NH));
                        let BJP = Lanes([BJI[0], BJI[1], 0.0, BJI[2]]) - Lanes([BJO[0], 0.0, BJO[1], BJO[2]]);
                        ALF = AES;
                        AZB = BJP;
                    } else {
                        let BJJ = Lanes([BJI[0], BJI[1], 0.0, BJI[2]]);
                        ALF = AEH;
                        AZB = BJJ;
                    }
                    AKZ = ALA;
                    ALD = ALF;
                    AYO = AYX;
                    AYP = AZB;
                }
                AKX = AKZ;
                ALC = ALD;
                AYH = AYO;
                AYI = AYP;
            }
            let AET = C / IN;
            let BLF = ((BBK * AET) * BAG) / IN;
            let AEV = if PO < AEU { 1.0 } else { 0.0 };
            let AFJ;
            let AZD;
            if AEV != 0.0 {
                let BLI = BDN * AET;
                let AEW = (PO * AET).exp();
                let BLJ = (Lanes([0.0, BLI[0], BLI[1]]) + Lanes([(BLF * PO), 0.0, 0.0])) * AEW;
                AFJ = AEW;
                AZD = BLJ;
            } else {
                let AEX = (AEU * AET).exp();
                let AEY = PO - AEU;
                let BLG = BDN * AET;
                let AEZ = C + (AEY * AET);
                let AFA = AEX * AEZ;
                let BLH = Lanes([(((BLF * AEU) * AEX) * AEZ), 0.0, 0.0]) + ((Lanes([0.0, BLG[0], BLG[1]]) + Lanes([(BLF * AEY), 0.0, 0.0])) * AEX);
                AFJ = AFA;
                AZD = BLH;
            }
            let AFB = C / IT;
            let BLK = ((BBO * AFB) * BAG) / IT;
            let AFD = if PO < AFC { 1.0 } else { 0.0 };
            let AFL;
            let AZE;
            if AFD != 0.0 {
                let BLN = BDN * AFB;
                let AFE = (PO * AFB).exp();
                let BLO = (Lanes([0.0, BLN[0], BLN[1]]) + Lanes([(BLK * PO), 0.0, 0.0])) * AFE;
                AFL = AFE;
                AZE = BLO;
            } else {
                let AFF = (AFC * AFB).exp();
                let AFG = PO - AFC;
                let BLL = BDN * AFB;
                let AFH = C + (AFG * AFB);
                let AFI = AFF * AFH;
                let BLM = Lanes([(((BLK * AFC) * AFF) * AFH), 0.0, 0.0]) + ((Lanes([0.0, BLL[0], BLL[1]]) + Lanes([(BLK * AFG), 0.0, 0.0])) * AFF);
                AFL = AFI;
                AZE = BLM;
            }
            let AFK = AFJ - C;
            let AFM = AFL - C;
            let AFN = (IQ * AFK) + (IW * AFM);
            let BLP = (Lanes([(BBM * AFK), 0.0, 0.0]) + (AZD * IQ)) + (Lanes([(BBQ * AFM), 0.0, 0.0]) + (AZE * IW));
            let AFO = if (if EL > A { 1.0 } else { 0.0 }) != 0.0 || (if EP > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ALG;
            let AZF;
            if AFO != 0.0 {
                let AFQ = if PT < AFP { 1.0 } else { 0.0 };
                let AGD;
                let AZG;
                if AFQ != 0.0 {
                    let BLT = BDQ * AET;
                    let AFR = (PT * AET).exp();
                    let BLU = (Lanes([0.0, BLT[0], BLT[1]]) + Lanes([(BLF * PT), 0.0, 0.0])) * AFR;
                    AGD = AFR;
                    AZG = BLU;
                } else {
                    let AFS = (AFP * AET).exp();
                    let AFT = PT - AFP;
                    let BLR = BDQ * AET;
                    let AFU = C + (AFT * AET);
                    let AFV = AFS * AFU;
                    let BLS = Lanes([(((BLF * AFP) * AFS) * AFU), 0.0, 0.0]) + ((Lanes([0.0, BLR[0], BLR[1]]) + Lanes([(BLF * AFT), 0.0, 0.0])) * AFS);
                    AGD = AFV;
                    AZG = BLS;
                }
                let AFX = if PT < AFW { 1.0 } else { 0.0 };
                let AGF;
                let AZH;
                if AFX != 0.0 {
                    let BLX = BDQ * AFB;
                    let AFY = (PT * AFB).exp();
                    let BLY = (Lanes([0.0, BLX[0], BLX[1]]) + Lanes([(BLK * PT), 0.0, 0.0])) * AFY;
                    AGF = AFY;
                    AZH = BLY;
                } else {
                    let AFZ = (AFW * AFB).exp();
                    let AGA = PT - AFW;
                    let BLV = BDQ * AFB;
                    let AGB = C + (AGA * AFB);
                    let AGC = AFZ * AGB;
                    let BLW = Lanes([(((BLK * AFW) * AFZ) * AGB), 0.0, 0.0]) + ((Lanes([0.0, BLV[0], BLV[1]]) + Lanes([(BLK * AGA), 0.0, 0.0])) * AFZ);
                    AGF = AGC;
                    AZH = BLW;
                }
                let AGE = AGD - C;
                let AGG = AGF - C;
                let AGH = (IY * AGE) + (JA * AGG);
                let BLZ = (Lanes([(BBR * AGE), 0.0, 0.0]) + (AZG * IY)) + (Lanes([(BBS * AGG), 0.0, 0.0]) + (AZH * JA));
                ALG = AGH;
                AZF = BLZ;
            } else {
                ALG = A;
                AZF = BLQ;
            }
            let AGI = PO / FV;
            let BMA = Lanes([0.0, BDN[0], BDN[1]]);
            let BMB = (BMA - Lanes([(BAJ * AGI), 0.0, 0.0])) / FV;
            let AGJ = if AGI < Q { 1.0 } else { 0.0 };
            let AGS;
            let AZI;
            if AGJ != 0.0 {
                let AGK = AGI.exp();
                let BMD = BMB * AGK;
                AGS = AGK;
                AZI = BMD;
            } else {
                let AGL = Q.exp();
                let AGM = AGL * (C + (AGI - Q));
                let BMC = BMB * AGL;
                AGS = AGM;
                AZI = BMC;
            }
            let AGN = PQ / FV;
            let BME = (Lanes([0.0, BDO[0], BDO[1]]) - Lanes([(BAJ * AGN), 0.0, 0.0])) / FV;
            let AGO = if AGN < Q { 1.0 } else { 0.0 };
            let AGU;
            let AZJ;
            if AGO != 0.0 {
                let AGP = AGN.exp();
                let BMG = BME * AGP;
                AGU = AGP;
                AZJ = BMG;
            } else {
                let AGQ = Q.exp();
                let AGR = AGQ * (C + (AGN - Q));
                let BMF = BME * AGQ;
                AGU = AGR;
                AZJ = BMF;
            }
            let AGT = (C + (NA * AGS)).sqrt();
            let BMH = (Lanes([(BCR * AGS), 0.0, 0.0]) + (AZI * NA)) * (AWF / (BCG * AGT));
            let AGV = (C + (NA * AGU)).sqrt();
            let BMI = (Lanes([(BCR * AGU), 0.0, 0.0]) + (AZJ * NA)) * (AWF / (BCG * AGV));
            let AGW = QA * NT;
            let BMJ = BDU * NT;
            let BMK = Lanes([BMJ[0], 0.0, BMJ[1]]) + Lanes([0.0, (AXB * QA), 0.0]);
            let AGX = AGV + C;
            let AGY = (AGT + C) / AGX;
            let BML = BMI * AGY;
            let BMM = Lanes([BMH[0], 0.0, BMH[1], BMH[2]]);
            let AGZ = (AGT - AGV) - (AGY.ln());
            let AHA = QB + (FV * AGZ);
            let AHB = AHA * NX;
            let BMN = ((Lanes([0.0, BDV[0], BDV[1], 0.0]) + (Lanes([(BAJ * AGZ), 0.0, 0.0, 0.0]) + (((BMM - Lanes([BMI[0], BMI[1], 0.0, BMI[2]])) - (((BMM - Lanes([BML[0], BML[1], 0.0, BML[2]])) / AGX) * (AWF / AGY))) * FV))) * NX) + Lanes([(AXC * AHA), 0.0, 0.0, 0.0]);
            let AHC = (BY * PE) * AC;
            let BMO = BDV * QB;
            let AHD = ((QB * QB) + E).sqrt();
            let BMP = ((BMO + BMO) * (AWF / (BCG * AHD))) * AHC;
            let AHE = C + (AHC * AHD);
            let AHF = NX * AHE;
            let AHG = (PE * AHB) / AHF;
            let BMQ = (Lanes([(AXC * AHE), 0.0, 0.0]) + ((Lanes([(((AXM * BY) * AC) * AHD), 0.0, 0.0]) + Lanes([0.0, BMP[0], BMP[1]])) * NX)) * AHG;
            let BMR = (((Lanes([(AXM * AHB), 0.0, 0.0, 0.0]) + (BMN * PE)) - Lanes([BMQ[0], BMQ[1], BMQ[2], 0.0])) / AHF) * AHG;
            let AHH = (C + (AHG * AHG)).sqrt();
            let AHI = AHB / AHH;
            let BMS = (BMN - (((BMR + BMR) * (AWF / (BCG * AHH))) * AHI)) / AHH;
            let AHJ = QC * OB;
            let BMT = BDW * OB;
            let BMU = Lanes([BMT[0], 0.0, BMT[1]]) + Lanes([0.0, (AXD * QC), 0.0]);
            let AHK = QD * XS;
            let BMV = BDX * XS;
            let BMW = AXX * QD;
            let AHL = AHK * OF;
            let BMX = ((Lanes([0.0, 0.0, BMV[0], BMV[1], 0.0]) + Lanes([BMW[0], BMW[1], 0.0, BMW[2], BMW[3]])) * OF) + Lanes([(AXE * AHK), 0.0, 0.0, 0.0, 0.0]);
            let AHM = QE * OI;
            let BMY = BDY * OI;
            let BMZ = Lanes([BMY[0], 0.0, BMY[1]]) + Lanes([0.0, (AXF * QE), 0.0]);
            let AHO = QF * AHN;
            let BNA = BDZ * AHN;
            let BNB = AYA * QF;
            let AHP = AHO * OM;
            let BNC = ((Lanes([0.0, BNA[0], 0.0, 0.0, 0.0, BNA[1]]) + Lanes([BNB[0], 0.0, BNB[1], BNB[2], BNB[3], BNB[4]])) * OM) + Lanes([(AXG * AHO), 0.0, 0.0, 0.0, 0.0, 0.0]);
            let AHQ = QJ * OP;
            let BND = BEC * OP;
            let BNE = Lanes([BND[0], 0.0, BND[1]]) + Lanes([0.0, (AXH * QJ), 0.0]);
            let AHS = if AHR > A { 1.0 } else { 0.0 };
            let AJY;
            let AZK;
            if AHS != 0.0 {
                let AHU = AHT * (JR + C);
                let AHW = C / (AHV - MN);
                let AHX = AHU.powf(AHW);
                let AHY = (LP - PO) - AHX;
                let BNG = Lanes([((BCA * AHT) * (AHW * (AHU.powf((AHW - AWF))))), 0.0, 0.0]);
                let BNH = (Lanes([BCJ, 0.0, 0.0]) - BMA) - BNG;
                let BNI = BNH * AHY;
                let AHZ = ((AHY * AHY) + E).sqrt();
                let AIA = (BY * (AHZ + AHY)) + AHX;
                let BNJ = ((((BNI + BNI) * (AWF / (BCG * AHZ))) + BNH) * BY) + BNG;
                let AIB = -JR;
                let AIC = MN - C;
                let AID = AIA.powf(AIC);
                let AIE = AIB * AID;
                let BNK = Lanes([((BCA * BAG) * AID), 0.0, 0.0]) + ((BNJ * (AIC * (AIA.powf((AIC - AWF))))) * AIB);
                let AIF = if AIE < Q { 1.0 } else { 0.0 };
                let AIK;
                let AZL;
                if AIF != 0.0 {
                    let AIG = AIE.exp();
                    let BNM = BNK * AIG;
                    AIK = AIG;
                    AZL = BNM;
                } else {
                    let AIH = Q.exp();
                    let AII = AIH * (C + (AIE - Q));
                    let BNL = BNK * AIH;
                    AIK = AII;
                    AZL = BNL;
                }
                let AIJ = AHR * AIA;
                let AIL = AIJ * AIK;
                let AIM = (QL - XV) - AFN;
                let AIN = AIM * AIL;
                let BNN = (((BNJ * AHR) * AIK) + (AZL * AIJ)) * AIM;
                let BNO = (((Lanes([0.0, 0.0, 0.0, 0.0, AWT]) - Lanes([BHG[0], BHG[1], BHG[2], BHG[3], 0.0])) - Lanes([BLP[0], BLP[1], BLP[2], 0.0, 0.0])) * AIL) + Lanes([BNN[0], BNN[1], BNN[2], 0.0, 0.0]);
                AJY = AIN;
                AZK = BNO;
            } else {
                AJY = A;
                AZK = BNF;
            }
            let AIP = if AIO > A { 1.0 } else { 0.0 };
            let ALP;
            let AZM;
            if AIP != 0.0 {
                let AIQ = AHT * (JU + C);
                let AIS = C / (AHV - AIR);
                let AIT = AIQ.powf(AIS);
                let BNQ = BDP * BAG;
                let AIU = (A - PR) - AIT;
                let BNR = Lanes([((BCB * AHT) * (AIS * (AIQ.powf((AIS - AWF))))), 0.0, 0.0]);
                let BNS = Lanes([0.0, BNQ[0], BNQ[1]]) - BNR;
                let BNT = BNS * AIU;
                let AIV = ((AIU * AIU) + E).sqrt();
                let AIW = (BY * (AIV + AIU)) + AIT;
                let BNU = ((((BNT + BNT) * (AWF / (BCG * AIV))) + BNS) * BY) + BNR;
                let AIX = -JU;
                let AIY = AIR - C;
                let AIZ = AIW.powf(AIY);
                let AJA = AIX * AIZ;
                let BNV = Lanes([((BCB * BAG) * AIZ), 0.0, 0.0]) + ((BNU * (AIY * (AIW.powf((AIY - AWF))))) * AIX);
                let AJB = if AJA < Q { 1.0 } else { 0.0 };
                let AJG;
                let AZN;
                if AJB != 0.0 {
                    let AJC = AJA.exp();
                    let BNX = BNV * AJC;
                    AJG = AJC;
                    AZN = BNX;
                } else {
                    let AJD = Q.exp();
                    let AJE = AJD * (C + (AJA - Q));
                    let BNW = BNV * AJD;
                    AJG = AJE;
                    AZN = BNW;
                }
                let AJF = AIO * AIW;
                let AJH = AJF * AJG;
                let AJI = -AGW;
                let AJJ = AJI * AJH;
                let BNY = (BMK * BAG) * AJH;
                let BNZ = (((BNU * AIO) * AJG) + (AZN * AJF)) * AJI;
                let BOA = Lanes([BNY[0], BNY[1], BNY[2], 0.0]) + Lanes([0.0, BNZ[0], BNZ[1], BNZ[2]]);
                ALP = AJJ;
                AZM = BOA;
            } else {
                ALP = A;
                AZM = BNP;
            }
            let AJM = if (if AJK > A { 1.0 } else { 0.0 }) != 0.0 && (if AJL > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let AJZ;
            let AZO;
            if AJM != 0.0 {
                let AJO = if AJN > A { 1.0 } else { 0.0 };
                let AJT;
                let AZP;
                if AJO != 0.0 {
                    let BOC = (BDN / AJN) * BAG;
                    let AJQ = (C - (PO / AJN)) - AJP;
                    let BOD = BOC * AJQ;
                    let AJR = ((AJQ * AJQ) + WZ).sqrt();
                    let AJS = AJL * (AJP + (BY * (AJQ + AJR)));
                    let BOE = ((BOC + ((BOD + BOD) * (AWF / (BCG * AJR)))) * BY) * AJL;
                    AJT = AJS;
                    AZP = BOE;
                } else {
                    AJT = AJL;
                    AZP = BOB;
                }
                let AJU = XW / AJT;
                let BOF = AZP * AJU;
                let AJV = AJU - C;
                let AJX = AJK * (AJV.powf(AJW));
                let BOG = (((BHH - Lanes([0.0, BOF[0], BOF[1], 0.0])) / AJT) * (AJW * (AJV.powf((AJW - AWF))))) * AJK;
                AJZ = AJX;
                AZO = BOG;
            } else {
                AJZ = A;
                AZO = BKI;
            }
            let AKA = (AFN - AJY) - AJZ;
            let BOH = (Lanes([BLP[0], BLP[1], BLP[2], 0.0, 0.0]) - AZK) - Lanes([AZO[0], AZO[1], AZO[2], AZO[3], 0.0]);
            let AKB = if (if ET > A { 1.0 } else { 0.0 }) != 0.0 || (if FC > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let ALH;
            let AZQ;
            if AKB != 0.0 {
                let AKC = C / JC;
                let BOJ = ((BBT * AKC) * BAG) / JC;
                let AKE = if QH < AKD { 1.0 } else { 0.0 };
                let AKS;
                let AZR;
                if AKE != 0.0 {
                    let BOM = BEA * AKC;
                    let AKF = (QH * AKC).exp();
                    let BON = (Lanes([0.0, BOM[0], BOM[1]]) + Lanes([(BOJ * QH), 0.0, 0.0])) * AKF;
                    AKS = AKF;
                    AZR = BON;
                } else {
                    let AKG = (AKD * AKC).exp();
                    let AKH = QH - AKD;
                    let BOK = BEA * AKC;
                    let AKI = C + (AKH * AKC);
                    let AKJ = AKG * AKI;
                    let BOL = Lanes([(((BOJ * AKD) * AKG) * AKI), 0.0, 0.0]) + ((Lanes([0.0, BOK[0], BOK[1]]) + Lanes([(BOJ * AKH), 0.0, 0.0])) * AKG);
                    AKS = AKJ;
                    AZR = BOL;
                }
                let AKK = C / JH;
                let BOO = ((BBV * AKK) * BAG) / JH;
                let AKM = if QH < AKL { 1.0 } else { 0.0 };
                let AKU;
                let AZS;
                if AKM != 0.0 {
                    let BOR = BEA * AKK;
                    let AKN = (QH * AKK).exp();
                    let BOS = (Lanes([0.0, BOR[0], BOR[1]]) + Lanes([(BOO * QH), 0.0, 0.0])) * AKN;
                    AKU = AKN;
                    AZS = BOS;
                } else {
                    let AKO = (AKL * AKK).exp();
                    let AKP = QH - AKL;
                    let BOP = BEA * AKK;
                    let AKQ = C + (AKP * AKK);
                    let AKR = AKO * AKQ;
                    let BOQ = Lanes([(((BOO * AKL) * AKO) * AKQ), 0.0, 0.0]) + ((Lanes([0.0, BOP[0], BOP[1]]) + Lanes([(BOO * AKP), 0.0, 0.0])) * AKO);
                    AKU = AKR;
                    AZS = BOQ;
                }
                let AKT = AKS - C;
                let AKV = AKU - C;
                let AKW = (JF * AKT) + (JK * AKV);
                let BOT = (Lanes([(BBU * AKT), 0.0, 0.0]) + (AZR * JF)) + (Lanes([(BBW * AKV), 0.0, 0.0]) + (AZS * JK));
                ALH = AKW;
                AZQ = BOT;
            } else {
                ALH = A;
                AZQ = BOI;
            }
            let BOU = BDL * AKX;
            let BOV = (AYH * PK) + Lanes([0.0, 0.0, BOU[0], BOU[1]]);
            let BOW = BDN * AKA;
            let ALB = QL - XV;
            let BOX = Lanes([0.0, 0.0, 0.0, 0.0, AWT]);
            let BOY = BDS * ALB;
            let BOZ = (Lanes([BOV[0], BOV[1], BOV[2], BOV[3], 0.0]) + ((BOH * PO) + Lanes([0.0, BOW[0], BOW[1], 0.0, 0.0]))) + (((BOX - Lanes([BHG[0], BHG[1], BHG[2], BHG[3], 0.0])) * PX) + Lanes([0.0, BOY[0], 0.0, BOY[1], 0.0]));
            let BPA = BDM * ALC;
            let BPB = (AYI * PM) + Lanes([0.0, BPA[0], 0.0, BPA[1]]);
            let BPC = Lanes([BOZ[0], BOZ[1], 0.0, BOZ[2], BOZ[3], BOZ[4]]) + Lanes([BPB[0], 0.0, BPB[1], BPB[2], BPB[3], 0.0]);
            let BPD = BDQ * ALG;
            let BPE = (AZF * PT) + Lanes([0.0, BPD[0], BPD[1]]);
            let BPF = Lanes([BPC[0], BPC[1], BPC[2], BPC[3], BPC[4], 0.0, BPC[5]]) + Lanes([BPE[0], 0.0, BPE[1], 0.0, 0.0, BPE[2], 0.0]);
            let BPG = BEC * AHQ;
            let BPH = (BNE * QJ) + Lanes([BPG[0], 0.0, BPG[1]]);
            let BPI = BEA * ALH;
            let BPJ = (AZQ * QH) + Lanes([0.0, BPI[0], BPI[1]]);
            let BPK = BEB * ALI;
            let BPL = (AYB * QI) + Lanes([0.0, 0.0, BPK[0], 0.0, 0.0, BPK[1]]);
            let BPM = ((Lanes([0.0, BPF[0], BPF[1], BPF[2], BPF[3], BPF[4], BPF[5], 0.0, BPF[6]]) + Lanes([BPH[0], BPH[1], 0.0, 0.0, 0.0, 0.0, 0.0, BPH[2], 0.0])) + Lanes([0.0, BPJ[0], 0.0, 0.0, 0.0, 0.0, BPJ[1], BPJ[2], 0.0])) + Lanes([0.0, BPL[0], BPL[1], BPL[2], BPL[3], 0.0, BPL[4], BPL[5], 0.0]);
            let BPN = BDU * AGW;
            let BPO = (BMK * QA) + Lanes([BPN[0], 0.0, BPN[1]]);
            let BPP = BDV * AHI;
            let BPQ = (BMS * QB) + Lanes([0.0, BPP[0], BPP[1], 0.0]);
            let BPR = (Lanes([0.0, BPM[0], BPM[1], 0.0, BPM[2], BPM[3], BPM[4], BPM[5], BPM[6], BPM[7], BPM[8]]) + Lanes([BPO[0], 0.0, BPO[1], BPO[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, BPQ[0], BPQ[1], BPQ[2], 0.0, BPQ[3], 0.0, 0.0, 0.0, 0.0]);
            let BPS = BDW * AHJ;
            let BPT = (BMU * QC) + Lanes([BPS[0], 0.0, BPS[1]]);
            let BPU = BDX * AHL;
            let BPV = (BMX * QD) + Lanes([0.0, 0.0, BPU[0], BPU[1], 0.0]);
            let BPW = (Lanes([BPR[0], 0.0, BPR[1], BPR[2], BPR[3], BPR[4], BPR[5], BPR[6], BPR[7], BPR[8], BPR[9], BPR[10]]) + Lanes([0.0, BPT[0], 0.0, BPT[1], 0.0, 0.0, BPT[2], 0.0, 0.0, 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, BPV[0], 0.0, BPV[1], BPV[2], BPV[3], BPV[4], 0.0, 0.0, 0.0]);
            let BPX = BDY * AHM;
            let BPY = (BMZ * QE) + Lanes([BPX[0], 0.0, BPX[1]]);
            let BPZ = BDZ * AHP;
            let BQA = (BNC * QF) + Lanes([0.0, BPZ[0], 0.0, 0.0, 0.0, BPZ[1]]);
            let ALJ = -parameters[2];
            let ALK = ALJ * ((((((((((((((AKX * PK) + (AKA * PO)) + (ALB * PX)) + (ALC * PM)) + (ALG * PT)) + (AHQ * QJ)) + (ALH * QH)) + (ALI * QI)) + (AGW * QA)) + (AHI * QB)) + (AHJ * QC)) + (AHL * QD)) + (AHM * QE)) + (AHP * QF));
            let BQB = ((Lanes([BPW[0], BPW[1], 0.0, BPW[2], BPW[3], BPW[4], BPW[5], BPW[6], BPW[7], BPW[8], BPW[9], BPW[10], BPW[11]]) + Lanes([0.0, 0.0, BPY[0], 0.0, BPY[1], 0.0, 0.0, 0.0, 0.0, BPY[2], 0.0, 0.0, 0.0])) + Lanes([0.0, 0.0, 0.0, 0.0, BQA[0], BQA[1], BQA[2], BQA[3], BQA[4], 0.0, BQA[5], 0.0, 0.0])) * ALJ;
            let ALL = FK * OS;
            let BQC = (AWG * OS) + (AXI * FK);
            let ALM = QL - XW;
            let BQD = BOX - Lanes([BHH[0], BHH[1], BHH[2], BHH[3], 0.0]);
            let ALN = QL - QK;
            let BQE = Lanes([0.0, AWT]) - Lanes([AWS, 0.0]);
            let BQF = BDL * ALO;
            let BQG = BDM * ALO;
            let BQH = BDQ * ALO;
            let BQI = BDN * ALO;
            let BQJ = BDP * ALO;
            let BQK = BEA * ALO;
            let ALQ = PF * (AKX + (ALO * PK));
            let BQL = (AYH + Lanes([0.0, 0.0, BQF[0], BQF[1]])) * PF;
            let ALR = PF * (ALC + (ALO * PM));
            let BQM = (AYI + Lanes([0.0, BQG[0], 0.0, BQG[1]])) * PF;
            let ALS = PF * QL;
            let BQN = AWT * PF;
            let ALT = PF * XV;
            let BQO = BHG * PF;
            let ALU = PF * (AKA + (ALO * PO));
            let BQP = (BOH + Lanes([0.0, BQI[0], BQI[1], 0.0, 0.0])) * PF;
            let ALV = PF * (ALP + (ALO * PR));
            let BQQ = (AZM + Lanes([0.0, 0.0, BQJ[0], BQJ[1]])) * PF;
            let ALW = PF * (ALG + (ALO * PT));
            let BQR = (AZF + Lanes([0.0, BQH[0], BQH[1]])) * PF;
            let ALX = PF * AHI;
            let BQS = BMS * PF;
            let ALY = PF * (ALH + (ALO * QH));
            let BQT = (AZQ + Lanes([0.0, BQK[0], BQK[1]])) * PF;
            let ALZ = PF * ALI;
            let BQU = AYB * PF;
            let AMA = if MS > A { 1.0 } else { 0.0 };
            let ATW;
            let AZT;
            if AMA != 0.0 {
                let AMB = -MG;
                let BQV = BCL * BAG;
                let AMC = AMB * QN;
                let BQW = BQV * QN;
                let AME = if AMD <= A { 1.0 } else { 0.0 };
                let ATX;
                let AZU;
                if AME != 0.0 {
                    let AMF = QH + AMC;
                    let BRH = Lanes([0.0, BEA[0], BEA[1]]);
                    let BRI = BRH + Lanes([BQW, 0.0, 0.0]);
                    let AMG = if AMF > A { 1.0 } else { 0.0 };
                    let AMW;
                    let AMX;
                    let AZV;
                    let AZW;
                    if AMG != 0.0 {
                        let AMH = C - QN;
                        let AMI = AMH.powf((-MU));
                        let AMJ = C - (AMI * AMH);
                        let AMK = C - MU;
                        let AML = (MG * AMJ) / AMK;
                        let AMM = BY * MU;
                        let AMN = MG * AMH;
                        let AMO = (AMM * AMF) / AMN;
                        let AMP = C + AMO;
                        let AMQ = (AMF * AMP) * AMI;
                        let BRK = ((BRI * AMP) + ((((BRI * AMM) - Lanes([((BCL * AMH) * AMO), 0.0, 0.0])) / AMN) * AMF)) * AMI;
                        let BRL = Lanes([((BCL * AMJ) / AMK), 0.0, 0.0]);
                        AMW = AML;
                        AMX = AMQ;
                        AZV = BRL;
                        AZW = BRK;
                    } else {
                        let AMR = QH / MG;
                        let AMS = C - AMR;
                        let AMT = C - MU;
                        let AMU = C - (AMS.powf(AMT));
                        let AMV = (MG * AMU) / AMT;
                        let BRJ = (Lanes([(BCL * AMU), 0.0, 0.0]) + ((((((BRH - Lanes([(BCL * AMR), 0.0, 0.0])) / MG) * BAG) * (AMT * (AMS.powf((AMT - AWF))))) * BAG) * MG)) / AMT;
                        AMW = AMV;
                        AMX = A;
                        AZV = BRJ;
                        AZW = BOI;
                    }
                    let AMY = AMW + AMX;
                    let BRM = AZV + AZW;
                    ATX = AMY;
                    AZU = BRM;
                } else {
                    let BQX = BQW * AMC;
                    let AMZ = (BZ * AMD) * AMD;
                    let ANA = ((AMC * AMC) + AMZ).sqrt();
                    let ANC = ANB * (AMC + ANA);
                    let BQY = (BQW + ((BQX + BQX) * (AWF / (BCG * ANA)))) * ANB;
                    let AND = ANC / MG;
                    let ANE = C - AND;
                    let ANF = C - MU;
                    let ANG = ANE.powf(ANF);
                    let BQZ = ANF - AWF;
                    let ANH = QH + AMC;
                    let BRA = Lanes([0.0, BEA[0], BEA[1]]);
                    let BRB = Lanes([BQW, 0.0, 0.0]);
                    let BRC = BRA + BRB;
                    let BRD = BRC * ANH;
                    let ANI = ((ANH * ANH) + AMZ).sqrt();
                    let ANJ = (BY * (ANH - ANI)) - AMC;
                    let BRE = ((BRC - ((BRD + BRD) * (AWF / (BCG * ANI)))) * BY) - BRB;
                    let ANK = ANJ / MG;
                    let ANL = C - ANK;
                    let ANM = ANL.powf(ANF);
                    let ANN = C - QN;
                    let ANO = ANN.powf((-MU));
                    let ANP = (QH - ANJ) + ANC;
                    let BRF = (BRA - BRE) + Lanes([BQY, 0.0, 0.0]);
                    let ANQ = ANO * ANP;
                    let ANR = BY * MU;
                    let ANS = MG * ANN;
                    let ANT = (ANR * ANP) / ANS;
                    let ANU = C + ANT;
                    let ANV = (((AMB * ANM) / ANF) + (ANQ * ANU)) - ((AMB * ANG) / ANF);
                    let BRG = (((Lanes([(BQV * ANM), 0.0, 0.0]) + (((((BRE - Lanes([(BCL * ANK), 0.0, 0.0])) / MG) * BAG) * (ANF * (ANL.powf(BQZ)))) * AMB)) / ANF) + (((BRF * ANO) * ANU) + ((((BRF * ANR) - Lanes([((BCL * ANN) * ANT), 0.0, 0.0])) / ANS) * ANQ))) - Lanes([(((BQV * ANG) + (((((BQY - (BCL * AND)) / MG) * BAG) * (ANF * (ANE.powf(BQZ)))) * AMB)) / ANF), 0.0, 0.0]);
                    ATX = ANV;
                    AZU = BRG;
                }
                ATW = ATX;
                AZT = AZU;
            } else {
                ATW = A;
                AZT = BOI;
            }
            let ATP;
            let AZX;
            if QQ != 0.0 {
                let ANW = PM + QO;
                let BRX = Lanes([0.0, BDM[0], BDM[1]]);
                let BRY = BRX + Lanes([BEE, 0.0, 0.0]);
                let ANX = if ANW > A { 1.0 } else { 0.0 };
                let AON;
                let AOO;
                let AZY;
                let AZZ;
                if ANX != 0.0 {
                    let ANY = C - QN;
                    let ANZ = ANY.powf((-MJ));
                    let AOA = C - (ANZ * ANY);
                    let AOB = C - MJ;
                    let AOC = (KY * AOA) / AOB;
                    let AOD = BY * MJ;
                    let AOE = KY * ANY;
                    let AOF = (AOD * ANW) / AOE;
                    let AOG = C + AOF;
                    let AOH = (ANW * AOG) * ANZ;
                    let BSB = ((BRY * AOG) + ((((BRY * AOD) - Lanes([((BCH * ANY) * AOF), 0.0, 0.0])) / AOE) * ANW)) * ANZ;
                    let BSC = Lanes([((BCH * AOA) / AOB), 0.0, 0.0]);
                    AON = AOC;
                    AOO = AOH;
                    AZY = BSC;
                    AZZ = BSB;
                } else {
                    let AOI = PM / KY;
                    let AOJ = C - AOI;
                    let AOK = C - MJ;
                    let AOL = C - (AOJ.powf(AOK));
                    let AOM = (KY * AOL) / AOK;
                    let BRZ = (Lanes([(BCH * AOL), 0.0, 0.0]) + ((((((BRX - Lanes([(BCH * AOI), 0.0, 0.0])) / KY) * BAG) * (AOK * (AOJ.powf((AOK - AWF))))) * BAG) * KY)) / AOK;
                    AON = AOM;
                    AOO = A;
                    AZY = BRZ;
                    AZZ = BSA;
                }
                let AOP = AON + AOO;
                let BSD = AZY + AZZ;
                ATP = AOP;
                AZX = BSD;
            } else {
                let BRN = BEE * QO;
                let AOQ = (BZ * QP) * QP;
                let AOR = ((QO * QO) + AOQ).sqrt();
                let AOT = AOS * (QO + AOR);
                let BRO = (BEE + ((BRN + BRN) * (AWF / (BCG * AOR)))) * AOS;
                let AOU = AOT / KY;
                let AOV = C - AOU;
                let AOW = C - MJ;
                let AOX = AOV.powf(AOW);
                let BRP = AOW - AWF;
                let AOY = PM + QO;
                let BRQ = Lanes([0.0, BDM[0], BDM[1]]);
                let BRR = Lanes([BEE, 0.0, 0.0]);
                let BRS = BRQ + BRR;
                let BRT = BRS * AOY;
                let AOZ = ((AOY * AOY) + AOQ).sqrt();
                let APA = (BY * (AOY - AOZ)) - QO;
                let BRU = ((BRS - ((BRT + BRT) * (AWF / (BCG * AOZ)))) * BY) - BRR;
                let APB = APA / KY;
                let APC = C - APB;
                let APD = APC.powf(AOW);
                let APE = C - QN;
                let APF = APE.powf((-MJ));
                let APG = (PM - APA) + AOT;
                let BRV = (BRQ - BRU) + Lanes([BRO, 0.0, 0.0]);
                let APH = APF * APG;
                let API = BY * MJ;
                let APJ = KY * APE;
                let APK = (API * APG) / APJ;
                let APL = C + APK;
                let APM = (((QM * APD) / AOW) + (APH * APL)) - ((QM * AOX) / AOW);
                let BRW = (((Lanes([(BED * APD), 0.0, 0.0]) + (((((BRU - Lanes([(BCH * APB), 0.0, 0.0])) / KY) * BAG) * (AOW * (APC.powf(BRP)))) * QM)) / AOW) + (((BRV * APF) * APL) + ((((BRV * API) - Lanes([((BCH * APE) * APK), 0.0, 0.0])) / APJ) * APH))) - Lanes([(((BED * AOX) + (((((BRO - (BCH * AOU)) / KY) * BAG) * (AOW * (AOV.powf(BRP)))) * QM)) / AOW), 0.0, 0.0]);
                ATP = APM;
                AZX = BRW;
            }
            let ATT;
            let BAA;
            if SL != 0.0 {
                let APN = PT + SJ;
                let BTD = Lanes([0.0, BDQ[0], BDQ[1]]);
                let BTE = BTD + Lanes([BEX, 0.0, 0.0]);
                let APO = if APN > A { 1.0 } else { 0.0 };
                let AQN;
                let AQP;
                let BAB;
                let BAC;
                if APO != 0.0 {
                    let APP = C - QN;
                    let APQ = APP.powf((-1e0f64 - MN));
                    let APR = C - ((APQ * APP) * APP);
                    let APS = C - MN;
                    let APT = (LP * APR) / APS;
                    let APU = BY * MN;
                    let APV = (APU * APN) / LP;
                    let APW = APP + APV;
                    let APX = (APN * APW) * APQ;
                    let BTI = ((BTE * APW) + ((((BTE * APU) - Lanes([(BCJ * APV), 0.0, 0.0])) / LP) * APN)) * APQ;
                    let BTJ = Lanes([((BCJ * APR) / APS), 0.0, 0.0]);
                    AQN = APT;
                    AQP = APX;
                    BAB = BTJ;
                    BAC = BTI;
                } else {
                    let APY = if (if SX > A { 1.0 } else { 0.0 }) != 0.0 && (if PT < (-SX) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let AQO;
                    let BAD;
                    if APY != 0.0 {
                        let APZ = SX / LP;
                        let AQA = C + APZ;
                        let AQB = C - MN;
                        let AQC = AQA.powf(AQB);
                        let BTG = BDQ * AQB;
                        let AQD = LP + SX;
                        let AQE = (AQB * (PT + SX)) / AQD;
                        let AQF = C - AQE;
                        let AQG = C - (AQC * AQF);
                        let AQH = (LP * AQG) / AQB;
                        let BTH = (Lanes([(BCJ * AQG), 0.0, 0.0]) + (((Lanes([(((((BCJ * APZ) * BAG) / LP) * (AQB * (AQA.powf((AQB - AWF))))) * AQF), 0.0, 0.0]) + ((((Lanes([0.0, BTG[0], BTG[1]]) - Lanes([(BCJ * AQE), 0.0, 0.0])) / AQD) * BAG) * AQC)) * BAG) * LP)) / AQB;
                        AQO = AQH;
                        BAD = BTH;
                    } else {
                        let AQI = PT / LP;
                        let AQJ = C - AQI;
                        let AQK = C - MN;
                        let AQL = C - (AQJ.powf(AQK));
                        let AQM = (LP * AQL) / AQK;
                        let BTF = (Lanes([(BCJ * AQL), 0.0, 0.0]) + ((((((BTD - Lanes([(BCJ * AQI), 0.0, 0.0])) / LP) * BAG) * (AQK * (AQJ.powf((AQK - AWF))))) * BAG) * LP)) / AQK;
                        AQO = AQM;
                        BAD = BTF;
                    }
                    AQN = AQO;
                    AQP = A;
                    BAB = BAD;
                    BAC = BLQ;
                }
                let AQQ = AQN + AQP;
                let BTK = BAB + BAC;
                ATT = AQQ;
                BAA = BTK;
            } else {
                let AQR = if (if SX > A { 1.0 } else { 0.0 }) != 0.0 && (if TR > A { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let ATU;
                let BAE;
                if AQR != 0.0 {
                    let AQS = SX - SJ;
                    let BSN = BEX * BAG;
                    let AQT = (SX + SJ) / AQS;
                    let BSO = (BEX - (BSN * AQT)) / AQS;
                    let AQU = AQT - C;
                    let BSP = BSO * AQU;
                    let AQV = (BZ * SK) * SK;
                    let AQW = ((AQU * AQU) + AQV).sqrt();
                    let AQX = AQT + C;
                    let BSQ = BSO * AQX;
                    let AQY = (BZ * TR) * TR;
                    let AQZ = ((AQX * AQX) + AQY).sqrt();
                    let ARA = AQW + AQZ;
                    let ARB = (JZ * AQT) / ARA;
                    let ARC = BY * (((ARB * AQS) - SX) - SJ);
                    let BSR = ((((((BSO * JZ) - ((((BSP + BSP) * (AWF / (BCG * AQW))) + ((BSQ + BSQ) * (AWF / (BCG * AQZ)))) * ARB)) / ARA) * AQS) + (BSN * ARB)) - BEX) * BY;
                    let ARD = ARC / LP;
                    let ARE = C - ARD;
                    let ARF = C - MN;
                    let BSS = ARF - AWF;
                    let ARG = C - (ARE.powf(ARF));
                    let BST = BDQ * JZ;
                    let BSU = Lanes([BEX, 0.0, 0.0]);
                    let ARH = (((JZ * PT) + SX) + SJ) / AQS;
                    let BSV = ((Lanes([0.0, BST[0], BST[1]]) + BSU) - Lanes([(BSN * ARH), 0.0, 0.0])) / AQS;
                    let ARI = ARH - C;
                    let BSW = BSV * ARI;
                    let ARJ = ((ARI * ARI) + AQV).sqrt();
                    let ARK = ARH + C;
                    let BSX = BSV * ARK;
                    let ARL = ((ARK * ARK) + AQY).sqrt();
                    let ARM = ARJ + ARL;
                    let ARN = (JZ * ARH) / ARM;
                    let BSY = ((BSV * JZ) - ((((BSW + BSW) * (AWF / (BCG * ARJ))) + ((BSX + BSX) * (AWF / (BCG * ARL)))) * ARN)) / ARM;
                    let ARO = BY * (((ARN * AQS) - SX) - SJ);
                    let BSZ = (((BSY * AQS) + Lanes([(BSN * ARN), 0.0, 0.0])) - BSU) * BY;
                    let ARP = ARO / LP;
                    let ARQ = C - ARP;
                    let ARR = C - (ARQ.powf(ARF));
                    let ARS = BY * (ARN + C);
                    let BTA = BSY * BY;
                    let ART = SX / LP;
                    let ARU = C + ART;
                    let ARV = -MN;
                    let ARW = ARU.powf(ARV);
                    let BTB = ARV - AWF;
                    let ARX = SJ / LP;
                    let ARY = C + ARX;
                    let ARZ = ARY.powf(ARV);
                    let ASA = C - ARS;
                    let ASB = (ASA * ARW) + (ARS * ARZ);
                    let ASC = (PT - ARO) + ARC;
                    let ASD = ((ASC * ASB) + ((LP * ARR) / ARF)) - ((LP * ARG) / ARF);
                    let BTC = (((((Lanes([0.0, BDQ[0], BDQ[1]]) - BSZ) + Lanes([BSR, 0.0, 0.0])) * ASB) + (((((BTA * BAG) * ARW) + Lanes([(((((BCJ * ART) * BAG) / LP) * (ARV * (ARU.powf(BTB)))) * ASA), 0.0, 0.0])) + ((BTA * ARZ) + Lanes([((((BEX - (BCJ * ARX)) / LP) * (ARV * (ARY.powf(BTB)))) * ARS), 0.0, 0.0]))) * ASC)) + ((Lanes([(BCJ * ARR), 0.0, 0.0]) + ((((((BSZ - Lanes([(BCJ * ARP), 0.0, 0.0])) / LP) * BAG) * (ARF * (ARQ.powf(BSS)))) * BAG) * LP)) / ARF)) - Lanes([(((BCJ * ARG) + ((((((BSR - (BCJ * ARD)) / LP) * BAG) * (ARF * (ARE.powf(BSS)))) * BAG) * LP)) / ARF), 0.0, 0.0]);
                    ATU = ASD;
                    BAE = BTC;
                } else {
                    let BSE = BEX * SJ;
                    let ASE = (BZ * SK) * SK;
                    let ASF = ((SJ * SJ) + ASE).sqrt();
                    let ASH = ASG * (SJ + ASF);
                    let BSF = (BEX + ((BSE + BSE) * (AWF / (BCG * ASF)))) * ASG;
                    let ASI = ASH / LP;
                    let ASJ = C - ASI;
                    let ASK = C - MN;
                    let ASL = ASJ.powf(ASK);
                    let BSG = ASK - AWF;
                    let ASM = PT + SJ;
                    let BSH = Lanes([0.0, BDQ[0], BDQ[1]]);
                    let BSI = Lanes([BEX, 0.0, 0.0]);
                    let BSJ = BSH + BSI;
                    let BSK = BSJ * ASM;
                    let ASN = ((ASM * ASM) + ASE).sqrt();
                    let ASO = (BY * (ASM - ASN)) - SJ;
                    let BSL = ((BSJ - ((BSK + BSK) * (AWF / (BCG * ASN)))) * BY) - BSI;
                    let ASP = ASO / LP;
                    let ASQ = C - ASP;
                    let ASR = ASQ.powf(ASK);
                    let ASS = (C - QN).powf((-MN));
                    let AST = (((SI * ASR) / ASK) + (ASS * ((PT - ASO) + ASH))) - ((SI * ASL) / ASK);
                    let BSM = (((Lanes([(BEW * ASR), 0.0, 0.0]) + (((((BSL - Lanes([(BCJ * ASP), 0.0, 0.0])) / LP) * BAG) * (ASK * (ASQ.powf(BSG)))) * SI)) / ASK) + (((BSH - BSL) + Lanes([BSF, 0.0, 0.0])) * ASS)) - Lanes([(((BEW * ASL) + (((((BSF - (BCJ * ASI)) / LP) * BAG) * (ASK * (ASJ.powf(BSG)))) * SI)) / ASK), 0.0, 0.0]);
                    ATU = AST;
                    BAE = BSM;
                }
                ATT = ATU;
                BAA = BAE;
            }
            let ASU = if WH > A { 1.0 } else { 0.0 };
            let ASV = if ASU != 0.0 {
                C
            } else {
                A
            };
            let ASW = (WH * ASV) * AK;
            let BTL = (BGL * ASV) * AK;
            let ASX = ASW + C;
            let ASY = ASW / ASX;
            let BTM = (BTL - (BTL * ASY)) / ASX;
            let ATA = (PO * AG) / ASZ;
            let BTN = (BDN * AG) / ASZ;
            let ATB = if ATA < Q { 1.0 } else { 0.0 };
            let ATJ;
            let BAF;
            if ATB != 0.0 {
                let ATC = ATA.exp();
                let BTP = BTN * ATC;
                ATJ = ATC;
                BAF = BTP;
            } else {
                let ATD = Q.exp();
                let ATE = ATD * (C + (ATA - Q));
                let BTO = BTN * ATD;
                ATJ = ATE;
                BAF = BTO;
            }
            let ATH = ATF * (C + (ATG * XD));
            let ATK = ATI * ATJ;
            let BTQ = BTM * ASY;
            let ATL = AL + (ASY * ASY);
            let BTR = (BAF * ATI) * ATL;
            let BTS = (BTQ + BTQ) * ATK;
            let ATM = C + ((ATK * ATL) * ASV);
            let ATN = ATH * ATM;
            let BTT = (Lanes([(BCM * WW), 0.0, 0.0]) + (AXN * MK)) * ZI;
            let BTU = BGL * ATN;
            let ATO = (ATN * WH) / XS;
            let ATQ = C - ZI;
            let BTV = Lanes([(BCP * ATT), 0.0, 0.0]) + (BAA * MR);
            let BTW = BEA * ATY;
            let AUA = PW * ATZ;
            let BTX = BDR * ATZ;
            let AUC = PZ * AUB;
            let BTY = BDT * AUB;
            let AUE = FK * AUD;
            let BTZ = AWG * AUD;
            let AUG = AUF * QK;
            let BUA = AWS * AUF;
            let AUI = (AUF * QL) * AUH;
            let BUB = (AWT * AUF) * AUH;
            let AUJ = PF * (((MK * WW) * ZI) + ATO);
            let BUC = (Lanes([BTT[0], 0.0, BTT[1], BTT[2]]) + ((((((((BGW * ATG) * ATF) * ATM) + (((Lanes([0.0, BTR[0], BTR[1], 0.0]) + Lanes([BTS[0], 0.0, BTS[1], BTS[2]])) * ASV) * ATH)) * WH) + Lanes([BTU[0], 0.0, BTU[1], BTU[2]])) - (AXX * ATO)) / XS)) * PF;
            let AUK = PF * ((MK * ATP) * ATQ);
            let BUD = ((Lanes([(BCM * ATP), 0.0, 0.0]) + (AZX * MK)) * ATQ) * PF;
            let AUL = PF * (((MP * WX) + (ATR * WV)) + (ATS * AGT));
            let BUE = (((Lanes([(BCO * WX), 0.0, 0.0]) + (AXQ * MP)) + (BGR * ATR)) + (BMH * ATS)) * PF;
            let AUM = PF * (ATS * AGV);
            let BUF = (BMI * ATS) * PF;
            let AUN = PF * ((MR * ATT) + (ATR * ATV));
            let BUG = (Lanes([BTV[0], 0.0, BTV[1], 0.0, BTV[2]]) + (AYC * ATR)) * PF;
            let AUO = PF * ((MV * ATW) + (ATY * QH));
            let BUH = ((Lanes([(BCQ * ATW), 0.0, 0.0]) + (AZT * MV)) + Lanes([0.0, BTW[0], BTW[1]])) * PF;
            let AUP = ddt(10542, AUJ);
            let BUJ = BUC * BUI;
            let AUQ = ddt(10544, AUK);
            let BUK = BUD * BUI;
            let AUR = ddt(10546, AUL);
            let BUL = BUE * BUI;
            let AUS = ddt(10548, AUM);
            let BUM = BUF * BUI;
            let AUT = ddt(10550, AUN);
            let BUN = BUG * BUI;
            let AUU = ddt(10552, AUA);
            let BUO = BTX * BUI;
            let AUV = ddt(10554, AUC);
            let BUP = BTY * BUI;
            let AUW = ddt(10556, AUO);
            let BUQ = BUH * BUI;
            let AUX = ddt(10558, AUG);
            let BUR = BUA * BUI;
            let AUY = ddt(10560, AUI);
            let BUS = BUB * BUI;
            let AUZ = ddt(10562, AUE);
            let BUT = BTZ * BUI;
            let AVQ;
            let AVR;
            let AVS;
            let AVT;
            let AVU;
            let AVV;
            let AVW;
            let AVX;
            let AVY;
            let AVZ;
            let AWA;
            let AWB;
            let AWC;
            let AWD;
            let AWE;
            if AVA != 0.0 {
                AVQ = AVB;
                AVR = AVC;
                AVS = AVD;
                AVT = AVE;
                AVU = AVF;
                AVV = AVG;
                AVW = AVH;
                AVX = AVI;
                AVY = AVJ;
                AVZ = AVK;
                AWA = AVL;
                AWB = AVM;
                AWC = AVN;
                AWD = AVO;
                AWE = AVP;
            } else {
                AVQ = A;
                AVR = A;
                AVS = A;
                AVT = A;
                AVU = A;
                AVV = A;
                AVW = A;
                AVX = A;
                AVY = A;
                AVZ = A;
                AWA = A;
                AWB = A;
                AWC = A;
                AWD = A;
                AWE = A;
            }
            let BUU = BQL[0];
            let BUV = BQL[1];
            let BUW = BQL[2];
            let BUX = BQL[3];
            let BUY = BQM[0];
            let BUZ = BQM[1];
            let BVA = BQM[2];
            let BVB = BQM[3];
            let BVC = BQN;
            let BVD = BQO[0];
            let BVE = BQO[1];
            let BVF = BQO[2];
            let BVG = BQO[3];
            let BVH = BQP[0];
            let BVI = BQP[1];
            let BVJ = BQP[2];
            let BVK = BQP[3];
            let BVL = BQP[4];
            let BVM = BQQ[0];
            let BVN = BQQ[1];
            let BVO = BQQ[2];
            let BVP = BQQ[3];
            let BVQ = BQR[0];
            let BVR = BQR[1];
            let BVS = BQR[2];
            let BVT = BMK[0];
            let BVU = BMK[1];
            let BVV = BMK[2];
            let BVW = BQS[0];
            let BVX = BQS[1];
            let BVY = BQS[2];
            let BVZ = BQS[3];
            let BWA = BMU[0];
            let BWB = BMU[1];
            let BWC = BMU[2];
            let BWD = BMX[0];
            let BWE = BMX[1];
            let BWF = BMX[2];
            let BWG = BMX[3];
            let BWH = BMX[4];
            let BWI = BMZ[0];
            let BWJ = BMZ[1];
            let BWK = BMZ[2];
            let BWL = BNC[0];
            let BWM = BNC[1];
            let BWN = BNC[2];
            let BWO = BNC[3];
            let BWP = BNC[4];
            let BWQ = BNC[5];
            let BWR = BQT[0];
            let BWS = BQT[1];
            let BWT = BQT[2];
            let BWU = BQU[0];
            let BWV = BQU[1];
            let BWW = BQU[2];
            let BWX = BQU[3];
            let BWY = BQU[4];
            let BWZ = BQU[5];
            let BXA = BNE[0];
            let BXB = BNE[1];
            let BXC = BNE[2];
            let BXD = BQD[0];
            let BXE = BQD[1];
            let BXF = BQD[2];
            let BXG = BQD[3];
            let BXH = BQD[4];
            let BXI = BQE[0];
            let BXJ = BQE[1];
            let BXK = BQC;
            let BXL = BQB[0];
            let BXM = BQB[1];
            let BXN = BQB[2];
            let BXO = BQB[3];
            let BXP = BQB[4];
            let BXQ = BQB[5];
            let BXR = BQB[6];
            let BXS = BQB[7];
            let BXT = BQB[8];
            let BXU = BQB[9];
            let BXV = BQB[10];
            let BXW = BQB[11];
            let BXX = BQB[12];
            let BXY = BUJ[0];
            let BXZ = BUJ[1];
            let BYA = BUJ[2];
            let BYB = BUJ[3];
            let BYC = BUK[0];
            let BYD = BUK[1];
            let BYE = BUK[2];
            let BYF = BUL[0];
            let BYG = BUL[1];
            let BYH = BUL[2];
            let BYI = BUM[0];
            let BYJ = BUM[1];
            let BYK = BUM[2];
            let BYL = BUN[0];
            let BYM = BUN[1];
            let BYN = BUN[2];
            let BYO = BUN[3];
            let BYP = BUN[4];
            let BYQ = BUO[0];
            let BYR = BUO[1];
            let BYS = BUP[0];
            let BYT = BUP[1];
            let BYU = BUQ[0];
            let BYV = BUQ[1];
            let BYW = BUQ[2];
            let BYX = BUR;
            let BYY = BUS;
            let BYZ = BUT;
            let BZA = BUC[0];
            let BZB = BUC[1];
            let BZC = BUC[2];
            let BZD = BUC[3];
            let BZE = BUD[0];
            let BZF = BUD[1];
            let BZG = BUD[2];
            let BZH = BUE[0];
            let BZI = BUE[1];
            let BZJ = BUE[2];
            let BZK = BUF[0];
            let BZL = BUF[1];
            let BZM = BUF[2];
            let BZN = BUG[0];
            let BZO = BUG[1];
            let BZP = BUG[2];
            let BZQ = BUG[3];
            let BZR = BUG[4];
            let BZS = BTX[0];
            let BZT = BTX[1];
            let BZU = BTY[0];
            let BZV = BTY[1];
            let BZW = BUH[0];
            let BZX = BUH[1];
            let BZY = BUH[2];
            let BZZ = BUA;
            let CAA = BUB;
            let CAB = BTZ;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (ALQ),
            [4, 6, 8, 9],
            [BUU, BUV, BUW, BUX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (ALR),
            [4, 7, 8, 9],
            [BUY, BUZ, BVA, BVB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            Some(9),
            multiplicity * (ALS),
            [13],
            [BVC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * (ALT),
            [4, 6, 8, 9],
            [BVD, BVE, BVF, BVG],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (ALU),
            [4, 6, 8, 9, 13],
            [BVH, BVI, BVJ, BVK, BVL],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(5),
            multiplicity * (ALV),
            [0, 4, 5, 7],
            [BVM, BVN, BVO, BVP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(10),
            multiplicity * (ALW),
            [4, 7, 10],
            [BVQ, BVR, BVS],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(5),
            multiplicity * (AGW),
            [0, 4, 5],
            [BVT, BVU, BVV],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (ALX),
            [4, 5, 6, 8],
            [BVW, BVX, BVY, BVZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(7),
            multiplicity * (AHJ),
            [1, 4, 7],
            [BWA, BWB, BWC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (AHL),
            [4, 6, 7, 8, 9],
            [BWD, BWE, BWF, BWG, BWH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(9),
            multiplicity * (AHM),
            [2, 4, 9],
            [BWI, BWJ, BWK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (AHP),
            [4, 5, 6, 7, 8, 10],
            [BWL, BWM, BWN, BWO, BWP, BWQ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (ALY),
            [4, 10, 11],
            [BWR, BWS, BWT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (ALZ),
            [4, 6, 7, 8, 10, 11],
            [BWU, BWV, BWW, BWX, BWY, BWZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(11),
            multiplicity * (AHQ),
            [3, 4, 11],
            [BXA, BXB, BXC],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * (ALM),
            [4, 6, 8, 9, 13],
            [BXD, BXE, BXF, BXG, BXH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(13),
            None,
            multiplicity * (ALN),
            [12, 13],
            [BXI, BXJ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (ALL),
            [4],
            [BXK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<13, 0>(
            Some(4),
            None,
            multiplicity * (ALK),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            [BXL, BXM, BXN, BXO, BXP, BXQ, BXR, BXS, BXT, BXU, BXV, BXW, BXX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (AUP),
            [4, 6, 8, 9],
            [BXY, BXZ, BYA, BYB],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(9),
            multiplicity * (AUQ),
            [4, 7, 9],
            [BYC, BYD, BYE],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(6),
            multiplicity * (AUR),
            [4, 6, 8],
            [BYF, BYG, BYH],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (AUS),
            [4, 5, 8],
            [BYI, BYJ, BYK],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (AUT),
            [4, 6, 7, 8, 10],
            [BYL, BYM, BYN, BYO, BYP],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (AUU),
            [1, 2],
            [BYQ, BYR],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (AUV),
            [0, 1],
            [BYS, BYT],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (AUW),
            [4, 10, 11],
            [BYU, BYV, BYW],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (AUX),
            [12],
            [BYX],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (AUY),
            [13],
            [BYY],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (AUZ),
            [4],
            [BYZ],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(9),
            multiplicity * (AVQ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(9),
            multiplicity * (AVR),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(9),
            multiplicity * (AVS),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(9),
            multiplicity * (AVT),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(9),
            multiplicity * (AVU),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(10),
            multiplicity * (AVV),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(10),
            multiplicity * (AVW),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(5),
            multiplicity * (AVX),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (AVY),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(7),
            multiplicity * (AVZ),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (AWA),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(9),
            multiplicity * (AWB),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(5),
            multiplicity * (AWC),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(11),
            multiplicity * (AWD),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(11),
            multiplicity * (AWE),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = ALQ;
        self.canonical_reactive[1] = ALR;
        self.canonical_reactive[2] = ALS;
        self.canonical_reactive[3] = ALT;
        self.canonical_reactive[4] = ALU;
        self.canonical_reactive[5] = ALV;
        self.canonical_reactive[6] = ALW;
        self.canonical_reactive[7] = AGW;
        self.canonical_reactive[8] = ALX;
        self.canonical_reactive[9] = AHJ;
        self.canonical_reactive[10] = AHL;
        self.canonical_reactive[11] = AHM;
        self.canonical_reactive[12] = AHP;
        self.canonical_reactive[13] = ALY;
        self.canonical_reactive[14] = ALZ;
        self.canonical_reactive[15] = AHQ;
        self.canonical_reactive[16] = ALM;
        self.canonical_reactive[17] = ALN;
        self.canonical_reactive[18] = ALL;
        self.canonical_reactive[19] = ALK;
        self.canonical_reactive[20] = AUJ;
        self.canonical_reactive[21] = BZA;
        self.canonical_reactive[22] = BZB;
        self.canonical_reactive[23] = BZC;
        self.canonical_reactive[24] = BZD;
        self.canonical_reactive[25] = AUK;
        self.canonical_reactive[26] = BZE;
        self.canonical_reactive[27] = BZF;
        self.canonical_reactive[28] = BZG;
        self.canonical_reactive[29] = AUL;
        self.canonical_reactive[30] = BZH;
        self.canonical_reactive[31] = BZI;
        self.canonical_reactive[32] = BZJ;
        self.canonical_reactive[33] = AUM;
        self.canonical_reactive[34] = BZK;
        self.canonical_reactive[35] = BZL;
        self.canonical_reactive[36] = BZM;
        self.canonical_reactive[37] = AUN;
        self.canonical_reactive[38] = BZN;
        self.canonical_reactive[39] = BZO;
        self.canonical_reactive[40] = BZP;
        self.canonical_reactive[41] = BZQ;
        self.canonical_reactive[42] = BZR;
        self.canonical_reactive[43] = AUA;
        self.canonical_reactive[44] = BZS;
        self.canonical_reactive[45] = BZT;
        self.canonical_reactive[46] = AUC;
        self.canonical_reactive[47] = BZU;
        self.canonical_reactive[48] = BZV;
        self.canonical_reactive[49] = AUO;
        self.canonical_reactive[50] = BZW;
        self.canonical_reactive[51] = BZX;
        self.canonical_reactive[52] = BZY;
        self.canonical_reactive[53] = AUG;
        self.canonical_reactive[54] = BZZ;
        self.canonical_reactive[55] = AUI;
        self.canonical_reactive[56] = CAA;
        self.canonical_reactive[57] = AUE;
        self.canonical_reactive[58] = CAB;
        self.canonical_reactive[59] = AVQ;
        self.canonical_reactive[60] = AVR;
        self.canonical_reactive[61] = AVS;
        self.canonical_reactive[62] = AVT;
        self.canonical_reactive[63] = AVU;
        self.canonical_reactive[64] = AVV;
        self.canonical_reactive[65] = AVW;
        self.canonical_reactive[66] = AVX;
        self.canonical_reactive[67] = AVY;
        self.canonical_reactive[68] = AVZ;
        self.canonical_reactive[69] = AWA;
        self.canonical_reactive[70] = AWB;
        self.canonical_reactive[71] = AWC;
        self.canonical_reactive[72] = AWD;
        self.canonical_reactive[73] = AWE;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(9),
            &[4, 6, 8, 9],
            &[cached[21], cached[22], cached[23], cached[24]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(9),
            &[4, 7, 9],
            &[cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 6, 8],
            &[cached[30], cached[31], cached[32]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[4, 5, 8],
            &[cached[34], cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(10),
            &[4, 6, 7, 8, 10],
            &[cached[38], cached[39], cached[40], cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[47], cached[48]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(10),
            &[4, 10, 11],
            &[cached[50], cached[51], cached[52]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            None,
            &[12],
            &[cached[54]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            None,
            &[13],
            &[cached[56]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[58]],
            &[],
            &[],
            multiplicity,
        );
    }

}

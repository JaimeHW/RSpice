use crate::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mextram504Polarity {
    Npn,
    Pnp,
}

impl Mextram504Polarity {
    pub fn type_sign(self) -> Value {
        match self {
            Self::Npn => 1.0,
            Self::Pnp => -1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mextram504Model {
    pub polarity: Mextram504Polarity,
    pub level: Value,
    pub tref: Value,
    pub dta: Value,
    pub exmod: Value,
    pub exphi: Value,
    pub exavl: Value,
    pub exsub: Value,
    pub is_: Value,
    pub ik: Value,
    pub ver: Value,
    pub vef: Value,
    pub bf: Value,
    pub ibf: Value,
    pub mlf: Value,
    pub xibi: Value,
    pub izeb: Value,
    pub nzeb: Value,
    pub bri: Value,
    pub ibr: Value,
    pub vlr: Value,
    pub xext: Value,
    pub wavl: Value,
    pub vavl: Value,
    pub sfh: Value,
    pub re: Value,
    pub rbc: Value,
    pub rbv: Value,
    pub rcc: Value,
    pub rcblx: Value,
    pub rcbli: Value,
    pub rcv: Value,
    pub scrcv: Value,
    pub ihc: Value,
    pub axi: Value,
    pub cje: Value,
    pub vde: Value,
    pub pe: Value,
    pub xcje: Value,
    pub cbeo: Value,
    pub cjc: Value,
    pub vdc: Value,
    pub pc: Value,
    pub xp: Value,
    pub mc: Value,
    pub xcjc: Value,
    pub cbco: Value,
    pub mtau: Value,
    pub taue: Value,
    pub taub: Value,
    pub tepi: Value,
    pub taur: Value,
    pub deg: Value,
    pub xrec: Value,
    pub xqb: Value,
    pub aqbo: Value,
    pub ae: Value,
    pub ab: Value,
    pub aex: Value,
    pub aepi: Value,
    pub ac: Value,
    pub acbl: Value,
    pub dais: Value,
    pub dvgbf: Value,
    pub dvgbr: Value,
    pub vgb: Value,
    pub vgc: Value,
    pub vgj: Value,
    pub vgzeb: Value,
    pub avgeb: Value,
    pub tvgeb: Value,
    pub dvgte: Value,
    pub af: Value,
    pub kf: Value,
    pub kfn: Value,
    pub kavl: Value,
    pub kc: Value,
    pub ke: Value,
    pub ftaun: Value,
    pub iss: Value,
    pub icss: Value,
    pub iks: Value,
    pub cjs: Value,
    pub vds: Value,
    pub ps: Value,
    pub vgs: Value,
    pub as_: Value,
    pub asub: Value,
    pub mult: Value,
    pub type_: Value,
    pub gmin: Value,
}

impl Mextram504Model {
    pub fn from_params(
        params: &HashMap<String, Value>,
        instance_params: &HashMap<String, Value>,
        polarity: Mextram504Polarity,
    ) -> Self {
        let val = |name: &str, default: Value| param(params, name).unwrap_or(default);
        let nonneg = |name: &str, default: Value| val(name, default).max(0.0);
        let at_least = |name: &str, default: Value, min: Value| val(name, default).max(min);
        let bounded = |name: &str, default: Value, min: Value, max: Value| {
            val(name, default).max(min).min(max)
        };
        let discrete = |name: &str, default: Value, min: Value, max: Value| {
            val(name, default).round().max(min).min(max)
        };
        let mult = param(instance_params, "M")
            .or_else(|| param(instance_params, "MULT"))
            .or_else(|| param(params, "M"))
            .or_else(|| param(params, "MULT"))
            .unwrap_or(1.0)
            .max(1.0e-30);
        Self {
            polarity,
            level: val("LEVEL", 504.0),
            tref: val("TREF", 25.0),
            dta: val("DTA", 0.0),
            exmod: discrete("EXMOD", 1.0, 0.0, 2.0),
            exphi: discrete("EXPHI", 1.0, 0.0, 1.0),
            exavl: discrete("EXAVL", 0.0, 0.0, 1.0),
            exsub: discrete("EXSUB", 0.0, 0.0, 1.0),
            is_: nonneg("IS", 22.0e-18),
            ik: at_least("IK", 0.1, 1.0e-12),
            ver: at_least("VER", 2.5, 0.01),
            vef: at_least("VEF", 44.0, 0.01),
            bf: at_least("BF", 215.0, 1.0e-4),
            ibf: nonneg("IBF", 2.7e-15),
            mlf: at_least("MLF", 2.0, 0.1),
            xibi: bounded("XIBI", 0.0, 0.0, 1.0),
            izeb: nonneg("IZEB", 0.0),
            nzeb: nonneg("NZEB", 22.0),
            bri: at_least("BRI", 7.0, 1.0e-4),
            ibr: nonneg("IBR", 1.0e-15),
            vlr: val("VLR", 0.2),
            xext: bounded("XEXT", 0.63, 0.0, 1.0),
            wavl: at_least("WAVL", 1.1e-6, 1.0e-9),
            vavl: at_least("VAVL", 3.0, 0.01),
            sfh: nonneg("SFH", 0.3),
            re: at_least("RE", 5.0, 1.0e-3),
            rbc: at_least("RBC", 23.0, 1.0e-3),
            rbv: at_least("RBV", 18.0, 1.0e-3),
            rcc: at_least("RCC", 12.0, 1.0e-3),
            rcblx: nonneg("RCBLX", 0.0),
            rcbli: nonneg("RCBLI", 0.0),
            rcv: at_least("RCV", 150.0, 1.0e-3),
            scrcv: at_least("SCRCV", 1250.0, 1.0e-3),
            ihc: at_least("IHC", 4.0e-3, 1.0e-12),
            axi: at_least("AXI", 0.3, 0.02),
            cje: nonneg("CJE", 73.0e-15),
            vde: at_least("VDE", 0.95, 0.05),
            pe: bounded("PE", 0.4, 0.01, 0.99),
            xcje: bounded("XCJE", 0.4, 0.0, 1.0),
            cbeo: nonneg("CBEO", 0.0),
            cjc: nonneg("CJC", 78.0e-15),
            vdc: at_least("VDC", 0.68, 0.05),
            pc: bounded("PC", 0.5, 0.01, 0.99),
            xp: bounded("XP", 0.35, 0.0, 0.99),
            mc: bounded("MC", 0.5, 0.0, 1.0),
            xcjc: bounded("XCJC", 32.0e-3, 0.0, 1.0),
            cbco: nonneg("CBCO", 0.0),
            mtau: at_least("MTAU", 1.0, 0.1),
            taue: nonneg("TAUE", 2.0e-12),
            taub: nonneg("TAUB", 4.2e-12),
            tepi: nonneg("TEPI", 41.0e-12),
            taur: nonneg("TAUR", 520.0e-12),
            deg: val("DEG", 0.0),
            xrec: nonneg("XREC", 0.0),
            xqb: bounded("XQB", 1.0 / 3.0, 0.0, 1.0),
            aqbo: val("AQBO", 0.3),
            ae: val("AE", 0.0),
            ab: val("AB", 1.0),
            aex: val("AEX", 0.62),
            aepi: val("AEPI", 2.5),
            ac: val("AC", 2.0),
            acbl: nonneg("ACBL", 2.0),
            dais: val("DAIS", 0.0),
            dvgbf: val("DVGBF", 50.0e-3),
            dvgbr: val("DVGBR", 45.0e-3),
            vgb: at_least("VGB", 1.17, 0.1),
            vgc: at_least("VGC", 1.18, 0.1),
            vgj: at_least("VGJ", 1.15, 0.1),
            vgzeb: at_least("VGZEB", 1.15, 0.1),
            avgeb: val("AVGEB", 4.73e-4),
            tvgeb: nonneg("TVGEB", 636.0),
            dvgte: val("DVGTE", 0.05),
            af: at_least("AF", 2.0, 0.01),
            kf: nonneg("KF", 20.0e-12),
            kfn: nonneg("KFN", 20.0e-12),
            kavl: discrete("KAVL", 0.0, 0.0, 1.0),
            kc: discrete("KC", 0.0, 0.0, 2.0),
            ke: bounded("KE", 0.0, 0.0, 1.0),
            ftaun: bounded("FTAUN", 0.0, 0.0, 1.0),
            iss: nonneg("ISS", 48.0e-18),
            icss: val("ICSS", -1.0),
            iks: at_least("IKS", 250.0e-6, 1.0e-12),
            cjs: nonneg("CJS", 315.0e-15),
            vds: at_least("VDS", 0.62, 0.05),
            ps: bounded("PS", 0.34, 0.01, 0.99),
            vgs: at_least("VGS", 1.20, 0.1),
            as_: val("AS", 1.58),
            asub: val("ASUB", 2.0),
            mult,
            type_: polarity.type_sign(),
            gmin: bounded("GMIN", 1.0e-13, 0.0, 1.0e-10),
        }
    }
}

#[inline]
fn param(params: &HashMap<String, Value>, name: &str) -> Option<Value> {
    params.get(name).copied().filter(|value| value.is_finite())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fgummel_model_params() -> HashMap<String, Value> {
        HashMap::from([
            ("LEVEL".to_string(), 504.0),
            ("IS".to_string(), 22.0e-18),
            ("IK".to_string(), 0.1),
            ("BF".to_string(), 215.0),
            ("VER".to_string(), 2.5),
            ("VEF".to_string(), 44.0),
            ("RE".to_string(), 5.0),
            ("RBC".to_string(), 23.0),
            ("RBV".to_string(), 18.0),
            ("RCC".to_string(), 12.0),
            ("RCV".to_string(), 150.0),
            ("TAUE".to_string(), 2.0e-12),
            ("CJS".to_string(), 315.0e-15),
            ("AS".to_string(), 1.58),
            ("ASUB".to_string(), 2.0),
            ("MULT".to_string(), 1.5),
        ])
    }

    #[test]
    fn xyce_fgummel_card_core_params_parse_and_model_mult_is_used() {
        let model = Mextram504Model::from_params(
            &fgummel_model_params(),
            &HashMap::new(),
            Mextram504Polarity::Npn,
        );

        assert_eq!(model.polarity, Mextram504Polarity::Npn);
        assert_eq!(model.level, 504.0);
        assert_eq!(model.is_, 22.0e-18);
        assert_eq!(model.ik, 0.1);
        assert_eq!(model.bf, 215.0);
        assert_eq!(model.ver, 2.5);
        assert_eq!(model.vef, 44.0);
        assert_eq!(model.re, 5.0);
        assert_eq!(model.rbc, 23.0);
        assert_eq!(model.rbv, 18.0);
        assert_eq!(model.rcc, 12.0);
        assert_eq!(model.rcblx, 0.0);
        assert_eq!(model.rcbli, 0.0);
        assert_eq!(model.rcv, 150.0);
        assert_eq!(model.taue, 2.0e-12);
        assert_eq!(model.cjs, 315.0e-15);
        assert_eq!(model.as_, 1.58);
        assert_eq!(model.asub, 2.0);
        assert_eq!(model.mult, 1.5);
    }

    #[test]
    fn instance_m_overrides_model_mult() {
        let instance_params = HashMap::from([("M".to_string(), 2.0)]);

        let model = Mextram504Model::from_params(
            &fgummel_model_params(),
            &instance_params,
            Mextram504Polarity::Npn,
        );

        assert_eq!(model.mult, 2.0);
    }

    #[test]
    fn omitted_params_use_xyce_mextram504_defaults() {
        let model =
            Mextram504Model::from_params(&HashMap::new(), &HashMap::new(), Mextram504Polarity::Npn);

        assert_eq!(model.level, 504.0);
        assert_eq!(model.is_, 22.0e-18);
        assert_eq!(model.ik, 0.1);
        assert_eq!(model.ver, 2.5);
        assert_eq!(model.vef, 44.0);
        assert_eq!(model.bf, 215.0);
        assert_eq!(model.ibf, 2.7e-15);
        assert_eq!(model.mlf, 2.0);
        assert_eq!(model.nzeb, 22.0);
        assert_eq!(model.bri, 7.0);
        assert_eq!(model.ibr, 1.0e-15);
        assert_eq!(model.xext, 0.63);
        assert_eq!(model.wavl, 1.1e-6);
        assert_eq!(model.vavl, 3.0);
        assert_eq!(model.sfh, 0.3);
        assert_eq!(model.re, 5.0);
        assert_eq!(model.rbc, 23.0);
        assert_eq!(model.rbv, 18.0);
        assert_eq!(model.rcc, 12.0);
        assert_eq!(model.rcv, 150.0);
        assert_eq!(model.scrcv, 1250.0);
        assert_eq!(model.ihc, 4.0e-3);
        assert_eq!(model.axi, 0.3);
        assert_eq!(model.cje, 73.0e-15);
        assert_eq!(model.vde, 0.95);
        assert_eq!(model.pe, 0.4);
        assert_eq!(model.xcje, 0.4);
        assert_eq!(model.cjc, 78.0e-15);
        assert_eq!(model.vdc, 0.68);
        assert_eq!(model.pc, 0.5);
        assert_eq!(model.xp, 0.35);
        assert_eq!(model.xcjc, 32.0e-3);
        assert_eq!(model.taue, 2.0e-12);
        assert_eq!(model.taub, 4.2e-12);
        assert_eq!(model.tepi, 41.0e-12);
        assert_eq!(model.taur, 520.0e-12);
        assert_eq!(model.xqb, 1.0 / 3.0);
        assert_eq!(model.aqbo, 0.3);
        assert_eq!(model.aex, 0.62);
        assert_eq!(model.aepi, 2.5);
        assert_eq!(model.ac, 2.0);
        assert_eq!(model.acbl, 2.0);
        assert_eq!(model.dvgbf, 50.0e-3);
        assert_eq!(model.dvgbr, 45.0e-3);
        assert_eq!(model.vgc, 1.18);
        assert_eq!(model.avgeb, 4.73e-4);
        assert_eq!(model.tvgeb, 636.0);
        assert_eq!(model.dvgte, 0.05);
        assert_eq!(model.af, 2.0);
        assert_eq!(model.kf, 20.0e-12);
        assert_eq!(model.kfn, 20.0e-12);
        assert_eq!(model.kavl, 0.0);
        assert_eq!(model.kc, 0.0);
        assert_eq!(model.ke, 0.0);
        assert_eq!(model.ftaun, 0.0);
        assert_eq!(model.iss, 48.0e-18);
        assert_eq!(model.icss, -1.0);
        assert_eq!(model.iks, 250.0e-6);
        assert_eq!(model.cjs, 315.0e-15);
        assert_eq!(model.vds, 0.62);
        assert_eq!(model.ps, 0.34);
        assert_eq!(model.vgs, 1.20);
        assert_eq!(model.as_, 1.58);
        assert_eq!(model.asub, 2.0);
        assert_eq!(model.mult, 1.0);
        assert_eq!(model.type_, 1.0);
        assert_eq!(model.gmin, 1.0e-13);
    }

    #[test]
    fn discrete_flags_are_canonicalized_and_type_follows_polarity() {
        let params = HashMap::from([
            ("EXMOD".to_string(), 1.5),
            ("KC".to_string(), 1.5),
            ("KAVL".to_string(), 0.6),
            ("TYPE".to_string(), 1.0),
            ("MULT".to_string(), -2.0),
        ]);

        let model = Mextram504Model::from_params(&params, &HashMap::new(), Mextram504Polarity::Pnp);

        assert_eq!(model.exmod, 2.0);
        assert_eq!(model.kc, 2.0);
        assert_eq!(model.kavl, 1.0);
        assert_eq!(model.type_, -1.0);
        assert_eq!(model.mult, 1.0e-30);
    }
}

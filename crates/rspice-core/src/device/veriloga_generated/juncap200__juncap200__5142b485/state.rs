#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
pub struct Parameters {
    pub level: f64,
    pub type_: f64,
    pub dta: f64,
    pub ab: f64,
    pub ls: f64,
    pub lg: f64,
    pub mult: f64,
    pub imax: f64,
    pub trj: f64,
    pub frev: f64,
    pub cjorbot: f64,
    pub cjorsti: f64,
    pub cjorgat: f64,
    pub vbirbot: f64,
    pub vbirsti: f64,
    pub vbirgat: f64,
    pub pbot: f64,
    pub psti: f64,
    pub pgat: f64,
    pub phigbot: f64,
    pub phigsti: f64,
    pub phiggat: f64,
    pub idsatrbot: f64,
    pub idsatrsti: f64,
    pub idsatrgat: f64,
    pub csrhbot: f64,
    pub csrhsti: f64,
    pub csrhgat: f64,
    pub xjunsti: f64,
    pub xjungat: f64,
    pub ctatbot: f64,
    pub ctatsti: f64,
    pub ctatgat: f64,
    pub mefftatbot: f64,
    pub mefftatsti: f64,
    pub mefftatgat: f64,
    pub cbbtbot: f64,
    pub cbbtsti: f64,
    pub cbbtgat: f64,
    pub fbbtrbot: f64,
    pub fbbtrsti: f64,
    pub fbbtrgat: f64,
    pub stfbbtbot: f64,
    pub stfbbtsti: f64,
    pub stfbbtgat: f64,
    pub vbrbot: f64,
    pub vbrsti: f64,
    pub vbrgat: f64,
    pub pbrbot: f64,
    pub pbrsti: f64,
    pub pbrgat: f64,
    pub swjunexp: f64,
    pub vjunref: f64,
    pub fjunq: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        let mut params = Self {
            level: 0.0,
            type_: 0.0,
            dta: 0.0,
            ab: 0.0,
            ls: 0.0,
            lg: 0.0,
            mult: 0.0,
            imax: 0.0,
            trj: 0.0,
            frev: 0.0,
            cjorbot: 0.0,
            cjorsti: 0.0,
            cjorgat: 0.0,
            vbirbot: 0.0,
            vbirsti: 0.0,
            vbirgat: 0.0,
            pbot: 0.0,
            psti: 0.0,
            pgat: 0.0,
            phigbot: 0.0,
            phigsti: 0.0,
            phiggat: 0.0,
            idsatrbot: 0.0,
            idsatrsti: 0.0,
            idsatrgat: 0.0,
            csrhbot: 0.0,
            csrhsti: 0.0,
            csrhgat: 0.0,
            xjunsti: 0.0,
            xjungat: 0.0,
            ctatbot: 0.0,
            ctatsti: 0.0,
            ctatgat: 0.0,
            mefftatbot: 0.0,
            mefftatsti: 0.0,
            mefftatgat: 0.0,
            cbbtbot: 0.0,
            cbbtsti: 0.0,
            cbbtgat: 0.0,
            fbbtrbot: 0.0,
            fbbtrsti: 0.0,
            fbbtrgat: 0.0,
            stfbbtbot: 0.0,
            stfbbtsti: 0.0,
            stfbbtgat: 0.0,
            vbrbot: 0.0,
            vbrsti: 0.0,
            vbrgat: 0.0,
            pbrbot: 0.0,
            pbrsti: 0.0,
            pbrgat: 0.0,
            swjunexp: 0.0,
            vjunref: 0.0,
            fjunq: 0.0,
        };
        params.level = 200.0;
        validate_parameter_level(params.level).expect("generated Verilog-A parameter default must satisfy declared range");
        params.type_ = 1.0;
        validate_parameter_type_(params.type_).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dta = 0.0;
        validate_parameter_dta(params.dta).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ab = 1e-12;
        validate_parameter_ab(params.ab).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ls = 1e-6;
        validate_parameter_ls(params.ls).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lg = 1e-6;
        validate_parameter_lg(params.lg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mult = 1.0;
        validate_parameter_mult(params.mult).expect("generated Verilog-A parameter default must satisfy declared range");
        params.imax = 1000.0;
        validate_parameter_imax(params.imax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.trj = 21.0;
        validate_parameter_trj(params.trj).expect("generated Verilog-A parameter default must satisfy declared range");
        params.frev = 1000.0;
        validate_parameter_frev(params.frev).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorbot = 0.001;
        validate_parameter_cjorbot(params.cjorbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorsti = 1e-9;
        validate_parameter_cjorsti(params.cjorsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjorgat = 1e-9;
        validate_parameter_cjorgat(params.cjorgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirbot = 1.0;
        validate_parameter_vbirbot(params.vbirbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirsti = 1.0;
        validate_parameter_vbirsti(params.vbirsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbirgat = 1.0;
        validate_parameter_vbirgat(params.vbirgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbot = 0.5;
        validate_parameter_pbot(params.pbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.psti = 0.5;
        validate_parameter_psti(params.psti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pgat = 0.5;
        validate_parameter_pgat(params.pgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phigbot = 1.16;
        validate_parameter_phigbot(params.phigbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phigsti = 1.16;
        validate_parameter_phigsti(params.phigsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phiggat = 1.16;
        validate_parameter_phiggat(params.phiggat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrbot = 1e-12;
        validate_parameter_idsatrbot(params.idsatrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrsti = 1e-18;
        validate_parameter_idsatrsti(params.idsatrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.idsatrgat = 1e-18;
        validate_parameter_idsatrgat(params.idsatrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhbot = 100.0;
        validate_parameter_csrhbot(params.csrhbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhsti = 0.0001;
        validate_parameter_csrhsti(params.csrhsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.csrhgat = 0.0001;
        validate_parameter_csrhgat(params.csrhgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xjunsti = 1e-7;
        validate_parameter_xjunsti(params.xjunsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xjungat = 1e-7;
        validate_parameter_xjungat(params.xjungat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatbot = 100.0;
        validate_parameter_ctatbot(params.ctatbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatsti = 0.0001;
        validate_parameter_ctatsti(params.ctatsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctatgat = 0.0001;
        validate_parameter_ctatgat(params.ctatgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatbot = 0.25;
        validate_parameter_mefftatbot(params.mefftatbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatsti = 0.25;
        validate_parameter_mefftatsti(params.mefftatsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mefftatgat = 0.25;
        validate_parameter_mefftatgat(params.mefftatgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtbot = 1e-12;
        validate_parameter_cbbtbot(params.cbbtbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtsti = 1e-18;
        validate_parameter_cbbtsti(params.cbbtsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cbbtgat = 1e-18;
        validate_parameter_cbbtgat(params.cbbtgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrbot = 1000000000.0;
        validate_parameter_fbbtrbot(params.fbbtrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrsti = 1000000000.0;
        validate_parameter_fbbtrsti(params.fbbtrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fbbtrgat = 1000000000.0;
        validate_parameter_fbbtrgat(params.fbbtrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtbot = -0.001;
        validate_parameter_stfbbtbot(params.stfbbtbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtsti = -0.001;
        validate_parameter_stfbbtsti(params.stfbbtsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.stfbbtgat = -0.001;
        validate_parameter_stfbbtgat(params.stfbbtgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrbot = 10.0;
        validate_parameter_vbrbot(params.vbrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrsti = 10.0;
        validate_parameter_vbrsti(params.vbrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbrgat = 10.0;
        validate_parameter_vbrgat(params.vbrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrbot = 4.0;
        validate_parameter_pbrbot(params.pbrbot).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrsti = 4.0;
        validate_parameter_pbrsti(params.pbrsti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pbrgat = 4.0;
        validate_parameter_pbrgat(params.pbrgat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.swjunexp = 0.0;
        validate_parameter_swjunexp(params.swjunexp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vjunref = 2.5;
        validate_parameter_vjunref(params.vjunref).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fjunq = 0.03;
        validate_parameter_fjunq(params.fjunq).expect("generated Verilog-A parameter default must satisfy declared range");
        params
    }
}

fn validate_parameter_level(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LEVEL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_type_(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TYPE' must be finite, got {}", value));
    }
    if value < -1.0 {
        return Err(format!("parameter 'TYPE' must be >= -1.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'TYPE' must be <= 1.0, got {}", value));
    }
    if value == 0.0 {
        return Err(format!("parameter 'TYPE' must not equal 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dta(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DTA' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ab(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AB' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AB' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ls(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LG' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LG' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mult(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MULT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'MULT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_imax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IMAX' must be finite, got {}", value));
    }
    if value < 1e-12 {
        return Err(format!("parameter 'IMAX' must be >= 1e-12, got {}", value));
    }
    Ok(())
}

fn validate_parameter_trj(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TRJ' must be finite, got {}", value));
    }
    if value < -250.0 {
        return Err(format!("parameter 'TRJ' must be >= -250.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_frev(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FREV' must be finite, got {}", value));
    }
    if value < 10.0 {
        return Err(format!("parameter 'FREV' must be >= 10.0, got {}", value));
    }
    if value > 10000000000.0 {
        return Err(format!("parameter 'FREV' must be <= 10000000000.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORBOT' must be finite, got {}", value));
    }
    if value < 1e-12 {
        return Err(format!("parameter 'CJORBOT' must be >= 1e-12, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORSTI' must be finite, got {}", value));
    }
    if value < 1e-18 {
        return Err(format!("parameter 'CJORSTI' must be >= 1e-18, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjorgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CJORGAT' must be finite, got {}", value));
    }
    if value < 1e-18 {
        return Err(format!("parameter 'CJORGAT' must be >= 1e-18, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRBOT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRBOT' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRSTI' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRSTI' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbirgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBIRGAT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'VBIRGAT' must be >= 0.05, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBOT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PBOT' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PBOT' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_psti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PSTI' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PSTI' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PSTI' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PGAT' must be finite, got {}", value));
    }
    if value < 0.05 {
        return Err(format!("parameter 'PGAT' must be >= 0.05, got {}", value));
    }
    if value > 0.95 {
        return Err(format!("parameter 'PGAT' must be <= 0.95, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phigbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGBOT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phigsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGSTI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phiggat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHIGGAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_idsatrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IDSATRGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IDSATRGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_csrhgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CSRHGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CSRHGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xjunsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XJUNSTI' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'XJUNSTI' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xjungat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XJUNGAT' must be finite, got {}", value));
    }
    if value < 1e-9 {
        return Err(format!("parameter 'XJUNGAT' must be >= 1e-9, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctatgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CTATGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CTATGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATBOT' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATBOT' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATSTI' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATSTI' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mefftatgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'MEFFTATGAT' must be finite, got {}", value));
    }
    if value < 0.01 {
        return Err(format!("parameter 'MEFFTATGAT' must be >= 0.01, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTBOT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTBOT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTSTI' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTSTI' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cbbtgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'CBBTGAT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'CBBTGAT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRBOT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRSTI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fbbtrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FBBTRGAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTBOT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTSTI' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_stfbbtgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'STFBBTGAT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRBOT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRBOT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRSTI' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRSTI' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VBRGAT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'VBRGAT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrbot(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRBOT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRBOT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrsti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRSTI' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRSTI' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pbrgat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PBRGAT' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'PBRGAT' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_swjunexp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'SWJUNEXP' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'SWJUNEXP' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'SWJUNEXP' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vjunref(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VJUNREF' must be finite, got {}", value));
    }
    if value < 0.5 {
        return Err(format!("parameter 'VJUNREF' must be >= 0.5, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fjunq(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'FJUNQ' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'FJUNQ' must be >= 0.0, got {}", value));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub nodes: [usize; 2],
    pub branches: [usize; 0],
    pub params: Parameters,
    pub(crate) param_given: [bool; 54],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 1],
    pub(crate) ddt_state_previous: [f64; 1],
    pub(crate) ddt_state_initialized: [bool; 1],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 2;
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 2;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 54;
    pub const VARIABLE_COUNT: usize = 707;
    pub const DDT_STATE_COUNT: usize = 1;
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::default(),
            param_given: [false; Self::PARAMETER_COUNT],
            multiplicity: 1.0,
            ddt_state_current: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_previous: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_initialized: [false; Self::DDT_STATE_COUNT],
            time: 0.0,
            timestep: 0.0,
        }
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "level" => { validate_parameter_level(value)?; self.params.level = value; self.mark_param_given(0); Ok(()) }
            "type" => { validate_parameter_type_(value)?; self.params.type_ = value; self.mark_param_given(1); Ok(()) }
            "dta" => { validate_parameter_dta(value)?; self.params.dta = value; self.mark_param_given(2); Ok(()) }
            "ab" => { validate_parameter_ab(value)?; self.params.ab = value; self.mark_param_given(3); Ok(()) }
            "ls" => { validate_parameter_ls(value)?; self.params.ls = value; self.mark_param_given(4); Ok(()) }
            "lg" => { validate_parameter_lg(value)?; self.params.lg = value; self.mark_param_given(5); Ok(()) }
            "mult" => { validate_parameter_mult(value)?; self.params.mult = value; self.mark_param_given(6); Ok(()) }
            "imax" => { validate_parameter_imax(value)?; self.params.imax = value; self.mark_param_given(7); Ok(()) }
            "trj" => { validate_parameter_trj(value)?; self.params.trj = value; self.mark_param_given(8); Ok(()) }
            "frev" => { validate_parameter_frev(value)?; self.params.frev = value; self.mark_param_given(9); Ok(()) }
            "cjorbot" => { validate_parameter_cjorbot(value)?; self.params.cjorbot = value; self.mark_param_given(10); Ok(()) }
            "cjorsti" => { validate_parameter_cjorsti(value)?; self.params.cjorsti = value; self.mark_param_given(11); Ok(()) }
            "cjorgat" => { validate_parameter_cjorgat(value)?; self.params.cjorgat = value; self.mark_param_given(12); Ok(()) }
            "vbirbot" => { validate_parameter_vbirbot(value)?; self.params.vbirbot = value; self.mark_param_given(13); Ok(()) }
            "vbirsti" => { validate_parameter_vbirsti(value)?; self.params.vbirsti = value; self.mark_param_given(14); Ok(()) }
            "vbirgat" => { validate_parameter_vbirgat(value)?; self.params.vbirgat = value; self.mark_param_given(15); Ok(()) }
            "pbot" => { validate_parameter_pbot(value)?; self.params.pbot = value; self.mark_param_given(16); Ok(()) }
            "psti" => { validate_parameter_psti(value)?; self.params.psti = value; self.mark_param_given(17); Ok(()) }
            "pgat" => { validate_parameter_pgat(value)?; self.params.pgat = value; self.mark_param_given(18); Ok(()) }
            "phigbot" => { validate_parameter_phigbot(value)?; self.params.phigbot = value; self.mark_param_given(19); Ok(()) }
            "phigsti" => { validate_parameter_phigsti(value)?; self.params.phigsti = value; self.mark_param_given(20); Ok(()) }
            "phiggat" => { validate_parameter_phiggat(value)?; self.params.phiggat = value; self.mark_param_given(21); Ok(()) }
            "idsatrbot" => { validate_parameter_idsatrbot(value)?; self.params.idsatrbot = value; self.mark_param_given(22); Ok(()) }
            "idsatrsti" => { validate_parameter_idsatrsti(value)?; self.params.idsatrsti = value; self.mark_param_given(23); Ok(()) }
            "idsatrgat" => { validate_parameter_idsatrgat(value)?; self.params.idsatrgat = value; self.mark_param_given(24); Ok(()) }
            "csrhbot" => { validate_parameter_csrhbot(value)?; self.params.csrhbot = value; self.mark_param_given(25); Ok(()) }
            "csrhsti" => { validate_parameter_csrhsti(value)?; self.params.csrhsti = value; self.mark_param_given(26); Ok(()) }
            "csrhgat" => { validate_parameter_csrhgat(value)?; self.params.csrhgat = value; self.mark_param_given(27); Ok(()) }
            "xjunsti" => { validate_parameter_xjunsti(value)?; self.params.xjunsti = value; self.mark_param_given(28); Ok(()) }
            "xjungat" => { validate_parameter_xjungat(value)?; self.params.xjungat = value; self.mark_param_given(29); Ok(()) }
            "ctatbot" => { validate_parameter_ctatbot(value)?; self.params.ctatbot = value; self.mark_param_given(30); Ok(()) }
            "ctatsti" => { validate_parameter_ctatsti(value)?; self.params.ctatsti = value; self.mark_param_given(31); Ok(()) }
            "ctatgat" => { validate_parameter_ctatgat(value)?; self.params.ctatgat = value; self.mark_param_given(32); Ok(()) }
            "mefftatbot" => { validate_parameter_mefftatbot(value)?; self.params.mefftatbot = value; self.mark_param_given(33); Ok(()) }
            "mefftatsti" => { validate_parameter_mefftatsti(value)?; self.params.mefftatsti = value; self.mark_param_given(34); Ok(()) }
            "mefftatgat" => { validate_parameter_mefftatgat(value)?; self.params.mefftatgat = value; self.mark_param_given(35); Ok(()) }
            "cbbtbot" => { validate_parameter_cbbtbot(value)?; self.params.cbbtbot = value; self.mark_param_given(36); Ok(()) }
            "cbbtsti" => { validate_parameter_cbbtsti(value)?; self.params.cbbtsti = value; self.mark_param_given(37); Ok(()) }
            "cbbtgat" => { validate_parameter_cbbtgat(value)?; self.params.cbbtgat = value; self.mark_param_given(38); Ok(()) }
            "fbbtrbot" => { validate_parameter_fbbtrbot(value)?; self.params.fbbtrbot = value; self.mark_param_given(39); Ok(()) }
            "fbbtrsti" => { validate_parameter_fbbtrsti(value)?; self.params.fbbtrsti = value; self.mark_param_given(40); Ok(()) }
            "fbbtrgat" => { validate_parameter_fbbtrgat(value)?; self.params.fbbtrgat = value; self.mark_param_given(41); Ok(()) }
            "stfbbtbot" => { validate_parameter_stfbbtbot(value)?; self.params.stfbbtbot = value; self.mark_param_given(42); Ok(()) }
            "stfbbtsti" => { validate_parameter_stfbbtsti(value)?; self.params.stfbbtsti = value; self.mark_param_given(43); Ok(()) }
            "stfbbtgat" => { validate_parameter_stfbbtgat(value)?; self.params.stfbbtgat = value; self.mark_param_given(44); Ok(()) }
            "vbrbot" => { validate_parameter_vbrbot(value)?; self.params.vbrbot = value; self.mark_param_given(45); Ok(()) }
            "vbrsti" => { validate_parameter_vbrsti(value)?; self.params.vbrsti = value; self.mark_param_given(46); Ok(()) }
            "vbrgat" => { validate_parameter_vbrgat(value)?; self.params.vbrgat = value; self.mark_param_given(47); Ok(()) }
            "pbrbot" => { validate_parameter_pbrbot(value)?; self.params.pbrbot = value; self.mark_param_given(48); Ok(()) }
            "pbrsti" => { validate_parameter_pbrsti(value)?; self.params.pbrsti = value; self.mark_param_given(49); Ok(()) }
            "pbrgat" => { validate_parameter_pbrgat(value)?; self.params.pbrgat = value; self.mark_param_given(50); Ok(()) }
            "swjunexp" => { validate_parameter_swjunexp(value)?; self.params.swjunexp = value; self.mark_param_given(51); Ok(()) }
            "vjunref" => { validate_parameter_vjunref(value)?; self.params.vjunref = value; self.mark_param_given(52); Ok(()) }
            "fjunq" => { validate_parameter_fjunq(value)?; self.params.fjunq = value; self.mark_param_given(53); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'JUNCAP200'", name)),
        }
    }

    #[inline]
    fn mark_param_given(&mut self, index: usize) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        self.param_given[index] = true;
    }

    #[inline]
    pub fn set_multiplicity(&mut self, multiplicity: f64) {
        if multiplicity.is_finite() && multiplicity > 0.0 {
            self.multiplicity = multiplicity;
        }
    }

    #[inline]
    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {
        self.time = time;
        self.timestep = timestep;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.ddt_state_previous[index] = self.ddt_state_current[index];
            self.ddt_state_initialized[index] = true;
            index += 1;
        }
    }

    #[inline]
    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {
        debug_assert!(slot < Self::DDT_STATE_COUNT, "generated ddt state slot out of range");
        let previous = if self.ddt_state_initialized[slot] {
            self.ddt_state_previous[slot]
        } else {
            value
        };
        self.ddt_state_current[slot] = value;
        if self.timestep.abs() > Self::DDT_EPSILON {
            (value - previous) / self.timestep
        } else {
            self.ddt_state_previous[slot] = value;
            self.ddt_state_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.timestep.abs() > Self::DDT_EPSILON {
            derivative / self.timestep
        } else {
            0.0
        }
    }
}

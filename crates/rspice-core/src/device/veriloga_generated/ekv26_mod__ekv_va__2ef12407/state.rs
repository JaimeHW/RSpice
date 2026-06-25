#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
pub struct Parameters {
    pub type_: f64,
    pub noise: f64,
    pub trise: f64,
    pub temp: f64,
    pub tnom: f64,
    pub l: f64,
    pub w: f64,
    pub m: f64,
    pub ns: f64,
    pub dtemp: f64,
    pub as_: f64,
    pub ad: f64,
    pub ps: f64,
    pub pd: f64,
    pub nrs: f64,
    pub nrd: f64,
    pub cox: f64,
    pub xj: f64,
    pub vto: f64,
    pub tcv: f64,
    pub gamma: f64,
    pub phi: f64,
    pub kp: f64,
    pub bex: f64,
    pub theta: f64,
    pub e0: f64,
    pub ucrit: f64,
    pub ucex: f64,
    pub lambda: f64,
    pub dl: f64,
    pub dw: f64,
    pub weta: f64,
    pub leta: f64,
    pub q0: f64,
    pub lk: f64,
    pub iba: f64,
    pub ibb: f64,
    pub ibbt: f64,
    pub ibn: f64,
    pub rsh: f64,
    pub hdif: f64,
    pub avto: f64,
    pub akp: f64,
    pub agamma: f64,
    pub af: f64,
    pub kf: f64,
    pub xd_n: f64,
    pub xd_js: f64,
    pub xd_jsw: f64,
    pub xd_jswg: f64,
    pub xd_mj: f64,
    pub xd_mjsw: f64,
    pub xd_mjswg: f64,
    pub xd_pb: f64,
    pub xd_pbsw: f64,
    pub xd_pbswg: f64,
    pub xd_cj: f64,
    pub xd_cjsw: f64,
    pub xd_cjswg: f64,
    pub xd_gmin: f64,
    pub xd_xjbv: f64,
    pub xd_bv: f64,
    pub xd_njts: f64,
    pub xd_njtssw: f64,
    pub xd_njtsswg: f64,
    pub xd_vts: f64,
    pub xd_vtssw: f64,
    pub xd_vtsswg: f64,
    pub tp_xti: f64,
    pub tp_cj: f64,
    pub tp_cjsw: f64,
    pub tp_cjswg: f64,
    pub tp_pb: f64,
    pub tp_pbsw: f64,
    pub tp_pbswg: f64,
    pub tp_njts: f64,
    pub tp_njtssw: f64,
    pub tp_njtsswg: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        let mut params = Self {
            type_: 0.0,
            noise: 0.0,
            trise: 0.0,
            temp: 0.0,
            tnom: 0.0,
            l: 0.0,
            w: 0.0,
            m: 0.0,
            ns: 0.0,
            dtemp: 0.0,
            as_: 0.0,
            ad: 0.0,
            ps: 0.0,
            pd: 0.0,
            nrs: 0.0,
            nrd: 0.0,
            cox: 0.0,
            xj: 0.0,
            vto: 0.0,
            tcv: 0.0,
            gamma: 0.0,
            phi: 0.0,
            kp: 0.0,
            bex: 0.0,
            theta: 0.0,
            e0: 0.0,
            ucrit: 0.0,
            ucex: 0.0,
            lambda: 0.0,
            dl: 0.0,
            dw: 0.0,
            weta: 0.0,
            leta: 0.0,
            q0: 0.0,
            lk: 0.0,
            iba: 0.0,
            ibb: 0.0,
            ibbt: 0.0,
            ibn: 0.0,
            rsh: 0.0,
            hdif: 0.0,
            avto: 0.0,
            akp: 0.0,
            agamma: 0.0,
            af: 0.0,
            kf: 0.0,
            xd_n: 0.0,
            xd_js: 0.0,
            xd_jsw: 0.0,
            xd_jswg: 0.0,
            xd_mj: 0.0,
            xd_mjsw: 0.0,
            xd_mjswg: 0.0,
            xd_pb: 0.0,
            xd_pbsw: 0.0,
            xd_pbswg: 0.0,
            xd_cj: 0.0,
            xd_cjsw: 0.0,
            xd_cjswg: 0.0,
            xd_gmin: 0.0,
            xd_xjbv: 0.0,
            xd_bv: 0.0,
            xd_njts: 0.0,
            xd_njtssw: 0.0,
            xd_njtsswg: 0.0,
            xd_vts: 0.0,
            xd_vtssw: 0.0,
            xd_vtsswg: 0.0,
            tp_xti: 0.0,
            tp_cj: 0.0,
            tp_cjsw: 0.0,
            tp_cjswg: 0.0,
            tp_pb: 0.0,
            tp_pbsw: 0.0,
            tp_pbswg: 0.0,
            tp_njts: 0.0,
            tp_njtssw: 0.0,
            tp_njtsswg: 0.0,
        };
        params.type_ = 1.0;
        validate_parameter_type_(params.type_).expect("generated Verilog-A parameter default must satisfy declared range");
        params.noise = 1.0;
        validate_parameter_noise(params.noise).expect("generated Verilog-A parameter default must satisfy declared range");
        params.trise = 0.0;
        validate_parameter_trise(params.trise).expect("generated Verilog-A parameter default must satisfy declared range");
        params.temp = 1e21;
        validate_parameter_temp(params.temp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tnom = 1e21;
        validate_parameter_tnom(params.tnom).expect("generated Verilog-A parameter default must satisfy declared range");
        params.l = 1e-5;
        validate_parameter_l(params.l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.w = 1e-5;
        validate_parameter_w(params.w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.m = 1.0;
        validate_parameter_m(params.m).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ns = 1.0;
        validate_parameter_ns(params.ns).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dtemp = 0.0;
        validate_parameter_dtemp(params.dtemp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.as_ = 0.0;
        validate_parameter_as_(params.as_).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ad = 0.0;
        validate_parameter_ad(params.ad).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ps = 0.0;
        validate_parameter_ps(params.ps).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pd = 0.0;
        validate_parameter_pd(params.pd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nrs = 1.0;
        validate_parameter_nrs(params.nrs).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nrd = 1.0;
        validate_parameter_nrd(params.nrd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cox = 0.002;
        validate_parameter_cox(params.cox).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xj = 3e-7;
        validate_parameter_xj(params.xj).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vto = 0.5;
        validate_parameter_vto(params.vto).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tcv = 0.001;
        validate_parameter_tcv(params.tcv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gamma = 0.7;
        validate_parameter_gamma(params.gamma).expect("generated Verilog-A parameter default must satisfy declared range");
        params.phi = 0.5;
        validate_parameter_phi(params.phi).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kp = 0.00015;
        validate_parameter_kp(params.kp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.bex = -1.5;
        validate_parameter_bex(params.bex).expect("generated Verilog-A parameter default must satisfy declared range");
        params.theta = 0.0;
        validate_parameter_theta(params.theta).expect("generated Verilog-A parameter default must satisfy declared range");
        params.e0 = 100000000.0;
        validate_parameter_e0(params.e0).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ucrit = 2000000.0;
        validate_parameter_ucrit(params.ucrit).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ucex = 0.8;
        validate_parameter_ucex(params.ucex).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lambda = 0.8;
        validate_parameter_lambda(params.lambda).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dl = -1e-8;
        validate_parameter_dl(params.dl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dw = -1e-8;
        validate_parameter_dw(params.dw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.weta = 0.2;
        validate_parameter_weta(params.weta).expect("generated Verilog-A parameter default must satisfy declared range");
        params.leta = 0.3;
        validate_parameter_leta(params.leta).expect("generated Verilog-A parameter default must satisfy declared range");
        params.q0 = 0.00023;
        validate_parameter_q0(params.q0).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lk = 4e-7;
        validate_parameter_lk(params.lk).expect("generated Verilog-A parameter default must satisfy declared range");
        params.iba = 500000000.0;
        validate_parameter_iba(params.iba).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ibb = 400000000.0;
        validate_parameter_ibb(params.ibb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ibbt = 0.0009;
        validate_parameter_ibbt(params.ibbt).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ibn = 1.0;
        validate_parameter_ibn(params.ibn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsh = 0.0;
        validate_parameter_rsh(params.rsh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.hdif = 5e-7;
        validate_parameter_hdif(params.hdif).expect("generated Verilog-A parameter default must satisfy declared range");
        params.avto = 1e-6;
        validate_parameter_avto(params.avto).expect("generated Verilog-A parameter default must satisfy declared range");
        params.akp = 1e-6;
        validate_parameter_akp(params.akp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.agamma = 1e-6;
        validate_parameter_agamma(params.agamma).expect("generated Verilog-A parameter default must satisfy declared range");
        params.af = 1.0;
        validate_parameter_af(params.af).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kf = 0.0;
        validate_parameter_kf(params.kf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_n = 1.0;
        validate_parameter_xd_n(params.xd_n).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_js = 1e-9;
        validate_parameter_xd_js(params.xd_js).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_jsw = 1e-12;
        validate_parameter_xd_jsw(params.xd_jsw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_jswg = 1e-12;
        validate_parameter_xd_jswg(params.xd_jswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_mj = 0.9;
        validate_parameter_xd_mj(params.xd_mj).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_mjsw = 0.7;
        validate_parameter_xd_mjsw(params.xd_mjsw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_mjswg = 0.7;
        validate_parameter_xd_mjswg(params.xd_mjswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_pb = 0.8;
        validate_parameter_xd_pb(params.xd_pb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_pbsw = 0.6;
        validate_parameter_xd_pbsw(params.xd_pbsw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_pbswg = 0.6;
        validate_parameter_xd_pbswg(params.xd_pbswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_cj = 1e-9;
        validate_parameter_xd_cj(params.xd_cj).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_cjsw = 1e-12;
        validate_parameter_xd_cjsw(params.xd_cjsw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_cjswg = 1e-12;
        validate_parameter_xd_cjswg(params.xd_cjswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_gmin = 0.0;
        validate_parameter_xd_gmin(params.xd_gmin).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_xjbv = 0.0;
        validate_parameter_xd_xjbv(params.xd_xjbv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_bv = 10.0;
        validate_parameter_xd_bv(params.xd_bv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_njts = 1.0;
        validate_parameter_xd_njts(params.xd_njts).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_njtssw = 1.0;
        validate_parameter_xd_njtssw(params.xd_njtssw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_njtsswg = 1.0;
        validate_parameter_xd_njtsswg(params.xd_njtsswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_vts = 0.0;
        validate_parameter_xd_vts(params.xd_vts).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_vtssw = 0.0;
        validate_parameter_xd_vtssw(params.xd_vtssw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xd_vtsswg = 0.0;
        validate_parameter_xd_vtsswg(params.xd_vtsswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_xti = 3.0;
        validate_parameter_tp_xti(params.tp_xti).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_cj = 0.0;
        validate_parameter_tp_cj(params.tp_cj).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_cjsw = 0.0;
        validate_parameter_tp_cjsw(params.tp_cjsw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_cjswg = 0.0;
        validate_parameter_tp_cjswg(params.tp_cjswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_pb = 0.0;
        validate_parameter_tp_pb(params.tp_pb).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_pbsw = 0.0;
        validate_parameter_tp_pbsw(params.tp_pbsw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_pbswg = 0.0;
        validate_parameter_tp_pbswg(params.tp_pbswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_njts = 0.0;
        validate_parameter_tp_njts(params.tp_njts).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_njtssw = 0.0;
        validate_parameter_tp_njtssw(params.tp_njtssw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tp_njtsswg = 0.0;
        validate_parameter_tp_njtsswg(params.tp_njtsswg).expect("generated Verilog-A parameter default must satisfy declared range");
        params
    }
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

fn validate_parameter_noise(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'Noise' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'Noise' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'Noise' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_trise(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'Trise' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_temp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TEMP' must be finite, got {}", value));
    }
    if value < 273.15 {
        return Err(format!("parameter 'TEMP' must be >= 273.15, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tnom(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TNOM' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'L' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'L' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'W' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'W' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_m(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'M' must be finite, got {}", value));
    }
    if value < 1.0 {
        return Err(format!("parameter 'M' must be >= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ns(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NS' must be finite, got {}", value));
    }
    if value < 1.0 {
        return Err(format!("parameter 'NS' must be >= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dtemp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DTEMP' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_as_(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ad(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ps(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'PD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nrs(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NRS' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NRS' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nrd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'NRD' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'NRD' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cox(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'COX' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'COX' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xj(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'XJ' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'XJ' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vto(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'VTO' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tcv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'TCV' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gamma(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'GAMMA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'GAMMA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_phi(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'PHI' must be finite, got {}", value));
    }
    if value < 0.2 {
        return Err(format!("parameter 'PHI' must be >= 0.2, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KP' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'KP' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_bex(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'BEX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_theta(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'THETA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'THETA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_e0(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'E0' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ucrit(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'UCRIT' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'UCRIT' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ucex(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'UCEX' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lambda(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LAMBDA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LAMBDA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DL' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'DW' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_weta(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'WETA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'WETA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_leta(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LETA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LETA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_q0(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'Q0' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'Q0' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lk(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'LK' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'LK' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_iba(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IBA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IBA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ibb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IBB' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IBB' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ibbt(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IBBT' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ibn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'IBN' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'IBN' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'RSH' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'RSH' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_hdif(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'HDIF' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'HDIF' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_avto(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AVTO' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AVTO' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_akp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AKP' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AKP' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_agamma(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AGAMMA' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'AGAMMA' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_af(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'AF' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'AF' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'KF' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'KF' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_n(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_n' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_n' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_js(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_js' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_js' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_jsw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_jsw' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_jsw' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_jswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_jswg' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_jswg' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_mj(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_mj' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_mj' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'xd_mj' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_mjsw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_mjsw' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_mjsw' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'xd_mjsw' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_mjswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_mjswg' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_mjswg' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'xd_mjswg' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_pb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_pb' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'xd_pb' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_pbsw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_pbsw' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'xd_pbsw' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_pbswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_pbswg' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'xd_pbswg' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_cj(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_cj' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_cj' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_cjsw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_cjsw' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_cjsw' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_cjswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_cjswg' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_cjswg' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_gmin(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_gmin' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_gmin' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_xjbv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_xjbv' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_xjbv' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_bv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_bv' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_bv' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_njts(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_njts' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_njts' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_njtssw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_njtssw' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_njtssw' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_njtsswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_njtsswg' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_njtsswg' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_vts(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_vts' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_vts' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_vtssw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_vtssw' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_vtssw' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xd_vtsswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xd_vtsswg' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'xd_vtsswg' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_xti(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_xti' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_cj(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_cj' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_cjsw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_cjsw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_cjswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_cjswg' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_pb(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_pb' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_pbsw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_pbsw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_pbswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_pbswg' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_njts(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_njts' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'tp_njts' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_njtssw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_njtssw' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'tp_njtssw' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tp_njtsswg(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tp_njtsswg' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'tp_njtsswg' must be >= 0.0, got {}", value));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub nodes: [usize; 4],
    pub branches: [usize; 0],
    pub params: Parameters,
    pub(crate) param_given: [bool; 78],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 5],
    pub(crate) ddt_state_previous: [f64; 5],
    pub(crate) ddt_state_initialized: [bool; 5],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 4;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 78;
    pub const VARIABLE_COUNT: usize = 271;
    pub const DDT_STATE_COUNT: usize = 5;
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
            "type" => { validate_parameter_type_(value)?; self.params.type_ = value; self.mark_param_given(0); Ok(()) }
            "noise" => { validate_parameter_noise(value)?; self.params.noise = value; self.mark_param_given(1); Ok(()) }
            "trise" => { validate_parameter_trise(value)?; self.params.trise = value; self.mark_param_given(2); Ok(()) }
            "temp" => { validate_parameter_temp(value)?; self.params.temp = value; self.mark_param_given(3); Ok(()) }
            "tnom" => { validate_parameter_tnom(value)?; self.params.tnom = value; self.mark_param_given(4); Ok(()) }
            "l" => { validate_parameter_l(value)?; self.params.l = value; self.mark_param_given(5); Ok(()) }
            "w" => { validate_parameter_w(value)?; self.params.w = value; self.mark_param_given(6); Ok(()) }
            "m" => { validate_parameter_m(value)?; self.params.m = value; self.mark_param_given(7); Ok(()) }
            "ns" => { validate_parameter_ns(value)?; self.params.ns = value; self.mark_param_given(8); Ok(()) }
            "dtemp" => { validate_parameter_dtemp(value)?; self.params.dtemp = value; self.mark_param_given(9); Ok(()) }
            "as" => { validate_parameter_as_(value)?; self.params.as_ = value; self.mark_param_given(10); Ok(()) }
            "ad" => { validate_parameter_ad(value)?; self.params.ad = value; self.mark_param_given(11); Ok(()) }
            "ps" => { validate_parameter_ps(value)?; self.params.ps = value; self.mark_param_given(12); Ok(()) }
            "pd" => { validate_parameter_pd(value)?; self.params.pd = value; self.mark_param_given(13); Ok(()) }
            "nrs" => { validate_parameter_nrs(value)?; self.params.nrs = value; self.mark_param_given(14); Ok(()) }
            "nrd" => { validate_parameter_nrd(value)?; self.params.nrd = value; self.mark_param_given(15); Ok(()) }
            "cox" => { validate_parameter_cox(value)?; self.params.cox = value; self.mark_param_given(16); Ok(()) }
            "xj" => { validate_parameter_xj(value)?; self.params.xj = value; self.mark_param_given(17); Ok(()) }
            "vto" => { validate_parameter_vto(value)?; self.params.vto = value; self.mark_param_given(18); Ok(()) }
            "tcv" => { validate_parameter_tcv(value)?; self.params.tcv = value; self.mark_param_given(19); Ok(()) }
            "gamma" => { validate_parameter_gamma(value)?; self.params.gamma = value; self.mark_param_given(20); Ok(()) }
            "phi" => { validate_parameter_phi(value)?; self.params.phi = value; self.mark_param_given(21); Ok(()) }
            "kp" => { validate_parameter_kp(value)?; self.params.kp = value; self.mark_param_given(22); Ok(()) }
            "bex" => { validate_parameter_bex(value)?; self.params.bex = value; self.mark_param_given(23); Ok(()) }
            "theta" => { validate_parameter_theta(value)?; self.params.theta = value; self.mark_param_given(24); Ok(()) }
            "e0" => { validate_parameter_e0(value)?; self.params.e0 = value; self.mark_param_given(25); Ok(()) }
            "ucrit" => { validate_parameter_ucrit(value)?; self.params.ucrit = value; self.mark_param_given(26); Ok(()) }
            "ucex" => { validate_parameter_ucex(value)?; self.params.ucex = value; self.mark_param_given(27); Ok(()) }
            "lambda" => { validate_parameter_lambda(value)?; self.params.lambda = value; self.mark_param_given(28); Ok(()) }
            "dl" => { validate_parameter_dl(value)?; self.params.dl = value; self.mark_param_given(29); Ok(()) }
            "dw" => { validate_parameter_dw(value)?; self.params.dw = value; self.mark_param_given(30); Ok(()) }
            "weta" => { validate_parameter_weta(value)?; self.params.weta = value; self.mark_param_given(31); Ok(()) }
            "leta" => { validate_parameter_leta(value)?; self.params.leta = value; self.mark_param_given(32); Ok(()) }
            "q0" => { validate_parameter_q0(value)?; self.params.q0 = value; self.mark_param_given(33); Ok(()) }
            "lk" => { validate_parameter_lk(value)?; self.params.lk = value; self.mark_param_given(34); Ok(()) }
            "iba" => { validate_parameter_iba(value)?; self.params.iba = value; self.mark_param_given(35); Ok(()) }
            "ibb" => { validate_parameter_ibb(value)?; self.params.ibb = value; self.mark_param_given(36); Ok(()) }
            "ibbt" => { validate_parameter_ibbt(value)?; self.params.ibbt = value; self.mark_param_given(37); Ok(()) }
            "ibn" => { validate_parameter_ibn(value)?; self.params.ibn = value; self.mark_param_given(38); Ok(()) }
            "rsh" => { validate_parameter_rsh(value)?; self.params.rsh = value; self.mark_param_given(39); Ok(()) }
            "hdif" => { validate_parameter_hdif(value)?; self.params.hdif = value; self.mark_param_given(40); Ok(()) }
            "avto" => { validate_parameter_avto(value)?; self.params.avto = value; self.mark_param_given(41); Ok(()) }
            "akp" => { validate_parameter_akp(value)?; self.params.akp = value; self.mark_param_given(42); Ok(()) }
            "agamma" => { validate_parameter_agamma(value)?; self.params.agamma = value; self.mark_param_given(43); Ok(()) }
            "af" => { validate_parameter_af(value)?; self.params.af = value; self.mark_param_given(44); Ok(()) }
            "kf" => { validate_parameter_kf(value)?; self.params.kf = value; self.mark_param_given(45); Ok(()) }
            "xd_n" => { validate_parameter_xd_n(value)?; self.params.xd_n = value; self.mark_param_given(46); Ok(()) }
            "xd_js" => { validate_parameter_xd_js(value)?; self.params.xd_js = value; self.mark_param_given(47); Ok(()) }
            "xd_jsw" => { validate_parameter_xd_jsw(value)?; self.params.xd_jsw = value; self.mark_param_given(48); Ok(()) }
            "xd_jswg" => { validate_parameter_xd_jswg(value)?; self.params.xd_jswg = value; self.mark_param_given(49); Ok(()) }
            "xd_mj" => { validate_parameter_xd_mj(value)?; self.params.xd_mj = value; self.mark_param_given(50); Ok(()) }
            "xd_mjsw" => { validate_parameter_xd_mjsw(value)?; self.params.xd_mjsw = value; self.mark_param_given(51); Ok(()) }
            "xd_mjswg" => { validate_parameter_xd_mjswg(value)?; self.params.xd_mjswg = value; self.mark_param_given(52); Ok(()) }
            "xd_pb" => { validate_parameter_xd_pb(value)?; self.params.xd_pb = value; self.mark_param_given(53); Ok(()) }
            "xd_pbsw" => { validate_parameter_xd_pbsw(value)?; self.params.xd_pbsw = value; self.mark_param_given(54); Ok(()) }
            "xd_pbswg" => { validate_parameter_xd_pbswg(value)?; self.params.xd_pbswg = value; self.mark_param_given(55); Ok(()) }
            "xd_cj" => { validate_parameter_xd_cj(value)?; self.params.xd_cj = value; self.mark_param_given(56); Ok(()) }
            "xd_cjsw" => { validate_parameter_xd_cjsw(value)?; self.params.xd_cjsw = value; self.mark_param_given(57); Ok(()) }
            "xd_cjswg" => { validate_parameter_xd_cjswg(value)?; self.params.xd_cjswg = value; self.mark_param_given(58); Ok(()) }
            "xd_gmin" => { validate_parameter_xd_gmin(value)?; self.params.xd_gmin = value; self.mark_param_given(59); Ok(()) }
            "xd_xjbv" => { validate_parameter_xd_xjbv(value)?; self.params.xd_xjbv = value; self.mark_param_given(60); Ok(()) }
            "xd_bv" => { validate_parameter_xd_bv(value)?; self.params.xd_bv = value; self.mark_param_given(61); Ok(()) }
            "xd_njts" => { validate_parameter_xd_njts(value)?; self.params.xd_njts = value; self.mark_param_given(62); Ok(()) }
            "xd_njtssw" => { validate_parameter_xd_njtssw(value)?; self.params.xd_njtssw = value; self.mark_param_given(63); Ok(()) }
            "xd_njtsswg" => { validate_parameter_xd_njtsswg(value)?; self.params.xd_njtsswg = value; self.mark_param_given(64); Ok(()) }
            "xd_vts" => { validate_parameter_xd_vts(value)?; self.params.xd_vts = value; self.mark_param_given(65); Ok(()) }
            "xd_vtssw" => { validate_parameter_xd_vtssw(value)?; self.params.xd_vtssw = value; self.mark_param_given(66); Ok(()) }
            "xd_vtsswg" => { validate_parameter_xd_vtsswg(value)?; self.params.xd_vtsswg = value; self.mark_param_given(67); Ok(()) }
            "tp_xti" => { validate_parameter_tp_xti(value)?; self.params.tp_xti = value; self.mark_param_given(68); Ok(()) }
            "tp_cj" => { validate_parameter_tp_cj(value)?; self.params.tp_cj = value; self.mark_param_given(69); Ok(()) }
            "tp_cjsw" => { validate_parameter_tp_cjsw(value)?; self.params.tp_cjsw = value; self.mark_param_given(70); Ok(()) }
            "tp_cjswg" => { validate_parameter_tp_cjswg(value)?; self.params.tp_cjswg = value; self.mark_param_given(71); Ok(()) }
            "tp_pb" => { validate_parameter_tp_pb(value)?; self.params.tp_pb = value; self.mark_param_given(72); Ok(()) }
            "tp_pbsw" => { validate_parameter_tp_pbsw(value)?; self.params.tp_pbsw = value; self.mark_param_given(73); Ok(()) }
            "tp_pbswg" => { validate_parameter_tp_pbswg(value)?; self.params.tp_pbswg = value; self.mark_param_given(74); Ok(()) }
            "tp_njts" => { validate_parameter_tp_njts(value)?; self.params.tp_njts = value; self.mark_param_given(75); Ok(()) }
            "tp_njtssw" => { validate_parameter_tp_njtssw(value)?; self.params.tp_njtssw = value; self.mark_param_given(76); Ok(()) }
            "tp_njtsswg" => { validate_parameter_tp_njtsswg(value)?; self.params.tp_njtsswg = value; self.mark_param_given(77); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'ekv_va'", name)),
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

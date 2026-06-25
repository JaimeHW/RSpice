#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
pub struct Parameters {
    pub w: f64,
    pub l: f64,
    pub wd: f64,
    pub a1: f64,
    pub p1: f64,
    pub c1: f64,
    pub a2: f64,
    pub p2: f64,
    pub c2: f64,
    pub trise: f64,
    pub nsmm_rsh: f64,
    pub nsmm_w: f64,
    pub nsmm_l: f64,
    pub sw_noise: f64,
    pub sw_et: f64,
    pub sw_lin: f64,
    pub sw_mman: f64,
    pub version: f64,
    pub subversion: f64,
    pub revision: f64,
    pub level: f64,
    pub type_: f64,
    pub scale: f64,
    pub shrink: f64,
    pub tmin: f64,
    pub tmax: f64,
    pub rthresh: f64,
    pub imax: f64,
    pub tnom: f64,
    pub lmin: f64,
    pub lmax: f64,
    pub wmin: f64,
    pub wmax: f64,
    pub jmax: f64,
    pub vmax: f64,
    pub tminclip: f64,
    pub tmaxclip: f64,
    pub rsh: f64,
    pub xw: f64,
    pub nwxw: f64,
    pub wexw: f64,
    pub fdrw: f64,
    pub fdxwinf: f64,
    pub xl: f64,
    pub xlw: f64,
    pub dxlsat: f64,
    pub nst: f64,
    pub ats: f64,
    pub atsl: f64,
    pub dfinf: f64,
    pub dfw: f64,
    pub dfl: f64,
    pub dfwl: f64,
    pub sw_dfgeo: f64,
    pub dp: f64,
    pub dpw: f64,
    pub dpwe: f64,
    pub dpl: f64,
    pub dple: f64,
    pub dpwl: f64,
    pub ecrit: f64,
    pub ecorn: f64,
    pub sw_vsatt: f64,
    pub sw_accpo: f64,
    pub grpo: f64,
    pub du: f64,
    pub rc: f64,
    pub rcw: f64,
    pub fc: f64,
    pub isa: f64,
    pub na: f64,
    pub ca: f64,
    pub cja: f64,
    pub pa: f64,
    pub ma: f64,
    pub aja: f64,
    pub isp: f64,
    pub np: f64,
    pub cp: f64,
    pub cjp: f64,
    pub pp: f64,
    pub mp: f64,
    pub ajp: f64,
    pub vbv: f64,
    pub ibv: f64,
    pub nbv: f64,
    pub kfn: f64,
    pub afn: f64,
    pub bfn: f64,
    pub sw_fngeo: f64,
    pub ea: f64,
    pub xis: f64,
    pub xvsat: f64,
    pub tc1: f64,
    pub tc2: f64,
    pub tc1l: f64,
    pub tc2l: f64,
    pub tc1w: f64,
    pub tc2w: f64,
    pub tc1wl: f64,
    pub tc2wl: f64,
    pub tc1rc: f64,
    pub tc2rc: f64,
    pub tc1dp: f64,
    pub tc2dp: f64,
    pub tc1vbv: f64,
    pub tc2vbv: f64,
    pub tc1nbv: f64,
    pub tc1kfn: f64,
    pub tegth: f64,
    pub gth0: f64,
    pub gthp: f64,
    pub gtha: f64,
    pub gthc: f64,
    pub cth0: f64,
    pub cthp: f64,
    pub ctha: f64,
    pub cthc: f64,
    pub nsig_rsh: f64,
    pub nsig_w: f64,
    pub nsig_l: f64,
    pub sig_rsh: f64,
    pub sig_w: f64,
    pub sig_l: f64,
    pub smm_rsh: f64,
    pub smm_w: f64,
    pub smm_l: f64,
    pub sw_mmgeo: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        let mut params = Self {
            w: 0.0,
            l: 0.0,
            wd: 0.0,
            a1: 0.0,
            p1: 0.0,
            c1: 0.0,
            a2: 0.0,
            p2: 0.0,
            c2: 0.0,
            trise: 0.0,
            nsmm_rsh: 0.0,
            nsmm_w: 0.0,
            nsmm_l: 0.0,
            sw_noise: 0.0,
            sw_et: 0.0,
            sw_lin: 0.0,
            sw_mman: 0.0,
            version: 0.0,
            subversion: 0.0,
            revision: 0.0,
            level: 0.0,
            type_: 0.0,
            scale: 0.0,
            shrink: 0.0,
            tmin: 0.0,
            tmax: 0.0,
            rthresh: 0.0,
            imax: 0.0,
            tnom: 0.0,
            lmin: 0.0,
            lmax: 0.0,
            wmin: 0.0,
            wmax: 0.0,
            jmax: 0.0,
            vmax: 0.0,
            tminclip: 0.0,
            tmaxclip: 0.0,
            rsh: 0.0,
            xw: 0.0,
            nwxw: 0.0,
            wexw: 0.0,
            fdrw: 0.0,
            fdxwinf: 0.0,
            xl: 0.0,
            xlw: 0.0,
            dxlsat: 0.0,
            nst: 0.0,
            ats: 0.0,
            atsl: 0.0,
            dfinf: 0.0,
            dfw: 0.0,
            dfl: 0.0,
            dfwl: 0.0,
            sw_dfgeo: 0.0,
            dp: 0.0,
            dpw: 0.0,
            dpwe: 0.0,
            dpl: 0.0,
            dple: 0.0,
            dpwl: 0.0,
            ecrit: 0.0,
            ecorn: 0.0,
            sw_vsatt: 0.0,
            sw_accpo: 0.0,
            grpo: 0.0,
            du: 0.0,
            rc: 0.0,
            rcw: 0.0,
            fc: 0.0,
            isa: 0.0,
            na: 0.0,
            ca: 0.0,
            cja: 0.0,
            pa: 0.0,
            ma: 0.0,
            aja: 0.0,
            isp: 0.0,
            np: 0.0,
            cp: 0.0,
            cjp: 0.0,
            pp: 0.0,
            mp: 0.0,
            ajp: 0.0,
            vbv: 0.0,
            ibv: 0.0,
            nbv: 0.0,
            kfn: 0.0,
            afn: 0.0,
            bfn: 0.0,
            sw_fngeo: 0.0,
            ea: 0.0,
            xis: 0.0,
            xvsat: 0.0,
            tc1: 0.0,
            tc2: 0.0,
            tc1l: 0.0,
            tc2l: 0.0,
            tc1w: 0.0,
            tc2w: 0.0,
            tc1wl: 0.0,
            tc2wl: 0.0,
            tc1rc: 0.0,
            tc2rc: 0.0,
            tc1dp: 0.0,
            tc2dp: 0.0,
            tc1vbv: 0.0,
            tc2vbv: 0.0,
            tc1nbv: 0.0,
            tc1kfn: 0.0,
            tegth: 0.0,
            gth0: 0.0,
            gthp: 0.0,
            gtha: 0.0,
            gthc: 0.0,
            cth0: 0.0,
            cthp: 0.0,
            ctha: 0.0,
            cthc: 0.0,
            nsig_rsh: 0.0,
            nsig_w: 0.0,
            nsig_l: 0.0,
            sig_rsh: 0.0,
            sig_w: 0.0,
            sig_l: 0.0,
            smm_rsh: 0.0,
            smm_w: 0.0,
            smm_l: 0.0,
            sw_mmgeo: 0.0,
        };
        params.w = 1e-6;
        validate_parameter_w(params.w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.l = 1e-6;
        validate_parameter_l(params.l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wd = 0.0;
        validate_parameter_wd(params.wd).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a1 = 0.0;
        validate_parameter_a1(params.a1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1 = 0.0;
        validate_parameter_p1(params.p1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.c1 = 0.0;
        validate_parameter_c1(params.c1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.a2 = 0.0;
        validate_parameter_a2(params.a2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p2 = 0.0;
        validate_parameter_p2(params.p2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.c2 = 0.0;
        validate_parameter_c2(params.c2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.trise = 0.0;
        validate_parameter_trise(params.trise).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsmm_rsh = 0.0;
        validate_parameter_nsmm_rsh(params.nsmm_rsh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsmm_w = 0.0;
        validate_parameter_nsmm_w(params.nsmm_w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsmm_l = 0.0;
        validate_parameter_nsmm_l(params.nsmm_l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_noise = 1.0;
        validate_parameter_sw_noise(params.sw_noise).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_et = 1.0;
        validate_parameter_sw_et(params.sw_et).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_lin = 0.0;
        validate_parameter_sw_lin(params.sw_lin).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_mman = 0.0;
        validate_parameter_sw_mman(params.sw_mman).expect("generated Verilog-A parameter default must satisfy declared range");
        params.version = 1.0;
        validate_parameter_version(params.version).expect("generated Verilog-A parameter default must satisfy declared range");
        params.subversion = 1.0;
        validate_parameter_subversion(params.subversion).expect("generated Verilog-A parameter default must satisfy declared range");
        params.revision = 2.0;
        validate_parameter_revision(params.revision).expect("generated Verilog-A parameter default must satisfy declared range");
        params.level = 1003.0;
        validate_parameter_level(params.level).expect("generated Verilog-A parameter default must satisfy declared range");
        params.type_ = -1.0;
        validate_parameter_type_(params.type_).expect("generated Verilog-A parameter default must satisfy declared range");
        params.scale = 1.0;
        validate_parameter_scale(params.scale).expect("generated Verilog-A parameter default must satisfy declared range");
        params.shrink = 0.0;
        validate_parameter_shrink(params.shrink).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tmin = -100.0;
        validate_parameter_tmin(params.tmin).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tmax = 500.0;
        validate_parameter_tmax(params.tmax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rthresh = 0.001;
        validate_parameter_rthresh(params.rthresh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.imax = 1.0;
        validate_parameter_imax(params.imax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tnom = 27.0;
        validate_parameter_tnom(params.tnom).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lmin = 0.0;
        validate_parameter_lmin(params.lmin).expect("generated Verilog-A parameter default must satisfy declared range");
        params.lmax = 9900000000.0;
        validate_parameter_lmax(params.lmax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wmin = 0.0;
        validate_parameter_wmin(params.wmin).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wmax = 9900000000.0;
        validate_parameter_wmax(params.wmax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.jmax = 100.0;
        validate_parameter_jmax(params.jmax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vmax = 9900000000.0;
        validate_parameter_vmax(params.vmax).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tminclip = -100.0;
        validate_parameter_tminclip(params.tminclip).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tmaxclip = 500.0;
        validate_parameter_tmaxclip(params.tmaxclip).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rsh = 100.0;
        validate_parameter_rsh(params.rsh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xw = 0.0;
        validate_parameter_xw(params.xw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nwxw = 0.0;
        validate_parameter_nwxw(params.nwxw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.wexw = 0.0;
        validate_parameter_wexw(params.wexw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fdrw = 1.0;
        validate_parameter_fdrw(params.fdrw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fdxwinf = 0.0;
        validate_parameter_fdxwinf(params.fdxwinf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xl = 0.0;
        validate_parameter_xl(params.xl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xlw = 0.0;
        validate_parameter_xlw(params.xlw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dxlsat = 0.0;
        validate_parameter_dxlsat(params.dxlsat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nst = 1.0;
        validate_parameter_nst(params.nst).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ats = 0.0;
        validate_parameter_ats(params.ats).expect("generated Verilog-A parameter default must satisfy declared range");
        params.atsl = 0.0;
        validate_parameter_atsl(params.atsl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dfinf = 0.01;
        validate_parameter_dfinf(params.dfinf).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dfw = 0.0;
        validate_parameter_dfw(params.dfw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dfl = 0.0;
        validate_parameter_dfl(params.dfl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dfwl = 0.0;
        validate_parameter_dfwl(params.dfwl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_dfgeo = 1.0;
        validate_parameter_sw_dfgeo(params.sw_dfgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dp = 2.0;
        validate_parameter_dp(params.dp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dpw = 0.0;
        validate_parameter_dpw(params.dpw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dpwe = 0.5;
        validate_parameter_dpwe(params.dpwe).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dpl = 0.0;
        validate_parameter_dpl(params.dpl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dple = 2.0;
        validate_parameter_dple(params.dple).expect("generated Verilog-A parameter default must satisfy declared range");
        params.dpwl = 0.0;
        validate_parameter_dpwl(params.dpwl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ecrit = 4.0;
        validate_parameter_ecrit(params.ecrit).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ecorn = 0.4;
        validate_parameter_ecorn(params.ecorn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_vsatt = 0.0;
        validate_parameter_sw_vsatt(params.sw_vsatt).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_accpo = 0.0;
        validate_parameter_sw_accpo(params.sw_accpo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.grpo = 1e-12;
        validate_parameter_grpo(params.grpo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.du = 0.02;
        validate_parameter_du(params.du).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rc = 0.0;
        validate_parameter_rc(params.rc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.rcw = 0.0;
        validate_parameter_rcw(params.rcw).expect("generated Verilog-A parameter default must satisfy declared range");
        params.fc = 0.9;
        validate_parameter_fc(params.fc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.isa = 0.0;
        validate_parameter_isa(params.isa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.na = 1.0;
        validate_parameter_na(params.na).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ca = 0.0;
        validate_parameter_ca(params.ca).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cja = 0.0;
        validate_parameter_cja(params.cja).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pa = 0.75;
        validate_parameter_pa(params.pa).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ma = 0.33;
        validate_parameter_ma(params.ma).expect("generated Verilog-A parameter default must satisfy declared range");
        params.aja = -0.5;
        validate_parameter_aja(params.aja).expect("generated Verilog-A parameter default must satisfy declared range");
        params.isp = 0.0;
        validate_parameter_isp(params.isp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.np = 1.0;
        validate_parameter_np(params.np).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cp = 0.0;
        validate_parameter_cp(params.cp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cjp = 0.0;
        validate_parameter_cjp(params.cjp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.pp = 0.75;
        validate_parameter_pp(params.pp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.mp = 0.33;
        validate_parameter_mp(params.mp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ajp = -0.5;
        validate_parameter_ajp(params.ajp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.vbv = 0.0;
        validate_parameter_vbv(params.vbv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ibv = 1e-6;
        validate_parameter_ibv(params.ibv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nbv = 1.0;
        validate_parameter_nbv(params.nbv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.kfn = 0.0;
        validate_parameter_kfn(params.kfn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.afn = 2.0;
        validate_parameter_afn(params.afn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.bfn = 1.0;
        validate_parameter_bfn(params.bfn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_fngeo = 0.0;
        validate_parameter_sw_fngeo(params.sw_fngeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ea = 1.12;
        validate_parameter_ea(params.ea).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xis = 3.0;
        validate_parameter_xis(params.xis).expect("generated Verilog-A parameter default must satisfy declared range");
        params.xvsat = 0.0;
        validate_parameter_xvsat(params.xvsat).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1 = 0.0;
        validate_parameter_tc1(params.tc1).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc2 = 0.0;
        validate_parameter_tc2(params.tc2).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1l = 0.0;
        validate_parameter_tc1l(params.tc1l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc2l = 0.0;
        validate_parameter_tc2l(params.tc2l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1w = 0.0;
        validate_parameter_tc1w(params.tc1w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc2w = 0.0;
        validate_parameter_tc2w(params.tc2w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1wl = 0.0;
        validate_parameter_tc1wl(params.tc1wl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc2wl = 0.0;
        validate_parameter_tc2wl(params.tc2wl).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1rc = 0.0;
        validate_parameter_tc1rc(params.tc1rc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc2rc = 0.0;
        validate_parameter_tc2rc(params.tc2rc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1dp = 0.0;
        validate_parameter_tc1dp(params.tc1dp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc2dp = 0.0;
        validate_parameter_tc2dp(params.tc2dp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1vbv = 0.0;
        validate_parameter_tc1vbv(params.tc1vbv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc2vbv = 0.0;
        validate_parameter_tc2vbv(params.tc2vbv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1nbv = 0.0;
        validate_parameter_tc1nbv(params.tc1nbv).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tc1kfn = 0.0;
        validate_parameter_tc1kfn(params.tc1kfn).expect("generated Verilog-A parameter default must satisfy declared range");
        params.tegth = 0.0;
        validate_parameter_tegth(params.tegth).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gth0 = 1000000.0;
        validate_parameter_gth0(params.gth0).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gthp = 0.0;
        validate_parameter_gthp(params.gthp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gtha = 0.0;
        validate_parameter_gtha(params.gtha).expect("generated Verilog-A parameter default must satisfy declared range");
        params.gthc = 0.0;
        validate_parameter_gthc(params.gthc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cth0 = 0.0;
        validate_parameter_cth0(params.cth0).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cthp = 0.0;
        validate_parameter_cthp(params.cthp).expect("generated Verilog-A parameter default must satisfy declared range");
        params.ctha = 0.0;
        validate_parameter_ctha(params.ctha).expect("generated Verilog-A parameter default must satisfy declared range");
        params.cthc = 0.0;
        validate_parameter_cthc(params.cthc).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsig_rsh = 0.0;
        validate_parameter_nsig_rsh(params.nsig_rsh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsig_w = 0.0;
        validate_parameter_nsig_w(params.nsig_w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.nsig_l = 0.0;
        validate_parameter_nsig_l(params.nsig_l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sig_rsh = 0.0;
        validate_parameter_sig_rsh(params.sig_rsh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sig_w = 0.0;
        validate_parameter_sig_w(params.sig_w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sig_l = 0.0;
        validate_parameter_sig_l(params.sig_l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.smm_rsh = 0.0;
        validate_parameter_smm_rsh(params.smm_rsh).expect("generated Verilog-A parameter default must satisfy declared range");
        params.smm_w = 0.0;
        validate_parameter_smm_w(params.smm_w).expect("generated Verilog-A parameter default must satisfy declared range");
        params.smm_l = 0.0;
        validate_parameter_smm_l(params.smm_l).expect("generated Verilog-A parameter default must satisfy declared range");
        params.sw_mmgeo = 0.0;
        validate_parameter_sw_mmgeo(params.sw_mmgeo).expect("generated Verilog-A parameter default must satisfy declared range");
        params
    }
}

fn validate_parameter_w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'w' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'w' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'l' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'l' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wd(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'wd' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'wd' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'a1' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'a1' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_p1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'p1' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'p1' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_c1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'c1' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'c1' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_a2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'a2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'a2' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_p2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'p2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'p2' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_c2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'c2' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'c2' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_trise(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'trise' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsmm_rsh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nsmm_rsh' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsmm_w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nsmm_w' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsmm_l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nsmm_l' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_noise(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_noise' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_noise' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_noise' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_et(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_et' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_et' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_et' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_lin(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_lin' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_lin' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_lin' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_mman(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_mman' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_mman' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_mman' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_version(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'version' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_subversion(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'subversion' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_revision(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'revision' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_level(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'level' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_type_(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'type' must be finite, got {}", value));
    }
    if value < -1.0 {
        return Err(format!("parameter 'type' must be >= -1.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'type' must be <= 1.0, got {}", value));
    }
    if value == 0.0 {
        return Err(format!("parameter 'type' must not equal 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_scale(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'scale' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'scale' must be > 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'scale' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_shrink(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'shrink' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'shrink' must be >= 0.0, got {}", value));
    }
    if value >= 100.0 {
        return Err(format!("parameter 'shrink' must be < 100.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tmin(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tmin' must be finite, got {}", value));
    }
    if value < -250.0 {
        return Err(format!("parameter 'tmin' must be >= -250.0, got {}", value));
    }
    if value > 27.0 {
        return Err(format!("parameter 'tmin' must be <= 27.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tmax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tmax' must be finite, got {}", value));
    }
    if value < 27.0 {
        return Err(format!("parameter 'tmax' must be >= 27.0, got {}", value));
    }
    if value > 1000.0 {
        return Err(format!("parameter 'tmax' must be <= 1000.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rthresh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'rthresh' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'rthresh' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_imax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'imax' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'imax' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tnom(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tnom' must be finite, got {}", value));
    }
    if value < -250.0 {
        return Err(format!("parameter 'tnom' must be >= -250.0, got {}", value));
    }
    if value > 1000.0 {
        return Err(format!("parameter 'tnom' must be <= 1000.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lmin(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'lmin' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'lmin' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_lmax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'lmax' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wmin(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'wmin' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'wmin' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wmax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'wmax' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_jmax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'jmax' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'jmax' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vmax(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'vmax' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'vmax' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tminclip(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tminclip' must be finite, got {}", value));
    }
    if value < -250.0 {
        return Err(format!("parameter 'tminclip' must be >= -250.0, got {}", value));
    }
    if value > 27.0 {
        return Err(format!("parameter 'tminclip' must be <= 27.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tmaxclip(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tmaxclip' must be finite, got {}", value));
    }
    if value < 27.0 {
        return Err(format!("parameter 'tmaxclip' must be >= 27.0, got {}", value));
    }
    if value > 1000.0 {
        return Err(format!("parameter 'tmaxclip' must be <= 1000.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rsh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'rsh' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'rsh' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nwxw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nwxw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_wexw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'wexw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fdrw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'fdrw' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'fdrw' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fdxwinf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'fdxwinf' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xl' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xlw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xlw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dxlsat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dxlsat' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nst(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nst' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'nst' must be >= 0.1, got {}", value));
    }
    if value > 5.0 {
        return Err(format!("parameter 'nst' must be <= 5.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ats(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ats' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ats' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_atsl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'atsl' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'atsl' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dfinf(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dfinf' must be finite, got {}", value));
    }
    if value < 0.0001 {
        return Err(format!("parameter 'dfinf' must be >= 0.0001, got {}", value));
    }
    if value > 10.0 {
        return Err(format!("parameter 'dfinf' must be <= 10.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dfw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dfw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dfl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dfl' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dfwl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dfwl' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_dfgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_dfgeo' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_dfgeo' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_dfgeo' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dp' must be finite, got {}", value));
    }
    if value < 0.1 {
        return Err(format!("parameter 'dp' must be >= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dpw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dpw' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dpwe(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dpwe' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dpl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dpl' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dple(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dple' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_dpwl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'dpwl' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ecrit(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ecrit' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ecrit' must be >= 0.0, got {}", value));
    }
    if value > 1000.0 {
        return Err(format!("parameter 'ecrit' must be <= 1000.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ecorn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ecorn' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ecorn' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_vsatt(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_vsatt' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_vsatt' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_vsatt' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_accpo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_accpo' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_accpo' must be >= 0.0, got {}", value));
    }
    if value > 3.0 {
        return Err(format!("parameter 'sw_accpo' must be <= 3.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_grpo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'grpo' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'grpo' must be > 0.0, got {}", value));
    }
    if value > 0.1 {
        return Err(format!("parameter 'grpo' must be <= 0.1, got {}", value));
    }
    Ok(())
}

fn validate_parameter_du(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'du' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'du' must be >= 0.0, got {}", value));
    }
    if value > 1000.0 {
        return Err(format!("parameter 'du' must be <= 1000.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'rc' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'rc' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_rcw(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'rcw' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'rcw' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_fc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'fc' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'fc' must be >= 0.0, got {}", value));
    }
    if value > 0.99 {
        return Err(format!("parameter 'fc' must be <= 0.99, got {}", value));
    }
    Ok(())
}

fn validate_parameter_isa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'isa' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'isa' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_na(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'na' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'na' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ca(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ca' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ca' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cja(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'cja' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'cja' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pa(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'pa' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'pa' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ma(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ma' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'ma' must be > 0.0, got {}", value));
    }
    if value >= 1.0 {
        return Err(format!("parameter 'ma' must be < 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_aja(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'aja' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_isp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'isp' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'isp' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_np(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'np' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'np' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'cp' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'cp' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cjp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'cjp' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'cjp' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_pp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'pp' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'pp' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_mp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'mp' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'mp' must be > 0.0, got {}", value));
    }
    if value >= 1.0 {
        return Err(format!("parameter 'mp' must be < 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ajp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ajp' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_vbv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'vbv' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'vbv' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ibv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ibv' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'ibv' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nbv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nbv' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'nbv' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_kfn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'kfn' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'kfn' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_afn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'afn' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'afn' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_bfn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'bfn' must be finite, got {}", value));
    }
    if value <= 0.0 {
        return Err(format!("parameter 'bfn' must be > 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_fngeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_fngeo' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_fngeo' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_fngeo' must be <= 1.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ea(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ea' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xis(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xis' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_xvsat(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'xvsat' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc2(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc2' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1l' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc2l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc2l' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1w' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc2w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc2w' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1wl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1wl' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc2wl(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc2wl' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1rc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1rc' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc2rc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc2rc' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1dp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1dp' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc2dp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc2dp' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1vbv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1vbv' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc2vbv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc2vbv' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1nbv(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1nbv' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tc1kfn(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tc1kfn' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_tegth(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'tegth' must be finite, got {}", value));
    }
    if value > 0.0 {
        return Err(format!("parameter 'tegth' must be <= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gth0(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'gth0' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'gth0' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gthp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'gthp' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'gthp' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gtha(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'gtha' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'gtha' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_gthc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'gthc' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'gthc' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cth0(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'cth0' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'cth0' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cthp(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'cthp' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'cthp' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_ctha(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'ctha' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'ctha' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_cthc(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'cthc' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'cthc' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsig_rsh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nsig_rsh' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsig_w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nsig_w' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_nsig_l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'nsig_l' must be finite, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sig_rsh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sig_rsh' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sig_rsh' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sig_w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sig_w' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sig_w' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sig_l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sig_l' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sig_l' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_smm_rsh(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'smm_rsh' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'smm_rsh' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_smm_w(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'smm_w' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'smm_w' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_smm_l(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'smm_l' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'smm_l' must be >= 0.0, got {}", value));
    }
    Ok(())
}

fn validate_parameter_sw_mmgeo(value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter 'sw_mmgeo' must be finite, got {}", value));
    }
    if value < 0.0 {
        return Err(format!("parameter 'sw_mmgeo' must be >= 0.0, got {}", value));
    }
    if value > 1.0 {
        return Err(format!("parameter 'sw_mmgeo' must be <= 1.0, got {}", value));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub nodes: [usize; 6],
    pub branches: [usize; 2],
    pub params: Parameters,
    pub(crate) param_given: [bool; 128],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 3],
    pub(crate) ddt_state_previous: [f64; 3],
    pub(crate) ddt_state_initialized: [bool; 3],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 2;
    pub const NODE_COUNT: usize = 6;
    pub const INTERNAL_NODE_NAMES: [&str; 2] = ["i1", "i2"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 128;
    pub const VARIABLE_COUNT: usize = 329;
    pub const DDT_STATE_COUNT: usize = 3;
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
            "w" => { validate_parameter_w(value)?; self.params.w = value; self.mark_param_given(0); Ok(()) }
            "l" => { validate_parameter_l(value)?; self.params.l = value; self.mark_param_given(1); Ok(()) }
            "wd" => { validate_parameter_wd(value)?; self.params.wd = value; self.mark_param_given(2); Ok(()) }
            "a1" => { validate_parameter_a1(value)?; self.params.a1 = value; self.mark_param_given(3); Ok(()) }
            "p1" => { validate_parameter_p1(value)?; self.params.p1 = value; self.mark_param_given(4); Ok(()) }
            "c1" => { validate_parameter_c1(value)?; self.params.c1 = value; self.mark_param_given(5); Ok(()) }
            "a2" => { validate_parameter_a2(value)?; self.params.a2 = value; self.mark_param_given(6); Ok(()) }
            "p2" => { validate_parameter_p2(value)?; self.params.p2 = value; self.mark_param_given(7); Ok(()) }
            "c2" => { validate_parameter_c2(value)?; self.params.c2 = value; self.mark_param_given(8); Ok(()) }
            "trise" => { validate_parameter_trise(value)?; self.params.trise = value; self.mark_param_given(9); Ok(()) }
            "dtemp" => { validate_parameter_trise(value)?; self.params.trise = value; self.mark_param_given(9); Ok(()) }
            "dta" => { validate_parameter_trise(value)?; self.params.trise = value; self.mark_param_given(9); Ok(()) }
            "nsmm_rsh" => { validate_parameter_nsmm_rsh(value)?; self.params.nsmm_rsh = value; self.mark_param_given(10); Ok(()) }
            "nsmm_w" => { validate_parameter_nsmm_w(value)?; self.params.nsmm_w = value; self.mark_param_given(11); Ok(()) }
            "nsmm_l" => { validate_parameter_nsmm_l(value)?; self.params.nsmm_l = value; self.mark_param_given(12); Ok(()) }
            "sw_noise" => { validate_parameter_sw_noise(value)?; self.params.sw_noise = value; self.mark_param_given(13); Ok(()) }
            "sw_et" => { validate_parameter_sw_et(value)?; self.params.sw_et = value; self.mark_param_given(14); Ok(()) }
            "sw_lin" => { validate_parameter_sw_lin(value)?; self.params.sw_lin = value; self.mark_param_given(15); Ok(()) }
            "sw_mman" => { validate_parameter_sw_mman(value)?; self.params.sw_mman = value; self.mark_param_given(16); Ok(()) }
            "version" => { validate_parameter_version(value)?; self.params.version = value; self.mark_param_given(17); Ok(()) }
            "subversion" => { validate_parameter_subversion(value)?; self.params.subversion = value; self.mark_param_given(18); Ok(()) }
            "revision" => { validate_parameter_revision(value)?; self.params.revision = value; self.mark_param_given(19); Ok(()) }
            "level" => { validate_parameter_level(value)?; self.params.level = value; self.mark_param_given(20); Ok(()) }
            "type" => { validate_parameter_type_(value)?; self.params.type_ = value; self.mark_param_given(21); Ok(()) }
            "scale" => { validate_parameter_scale(value)?; self.params.scale = value; self.mark_param_given(22); Ok(()) }
            "shrink" => { validate_parameter_shrink(value)?; self.params.shrink = value; self.mark_param_given(23); Ok(()) }
            "tmin" => { validate_parameter_tmin(value)?; self.params.tmin = value; self.mark_param_given(24); Ok(()) }
            "tmax" => { validate_parameter_tmax(value)?; self.params.tmax = value; self.mark_param_given(25); Ok(()) }
            "rthresh" => { validate_parameter_rthresh(value)?; self.params.rthresh = value; self.mark_param_given(26); Ok(()) }
            "imax" => { validate_parameter_imax(value)?; self.params.imax = value; self.mark_param_given(27); Ok(()) }
            "tnom" => { validate_parameter_tnom(value)?; self.params.tnom = value; self.mark_param_given(28); Ok(()) }
            "lmin" => { validate_parameter_lmin(value)?; self.params.lmin = value; self.mark_param_given(29); Ok(()) }
            "lmax" => { validate_parameter_lmax(value)?; self.params.lmax = value; self.mark_param_given(30); Ok(()) }
            "wmin" => { validate_parameter_wmin(value)?; self.params.wmin = value; self.mark_param_given(31); Ok(()) }
            "wmax" => { validate_parameter_wmax(value)?; self.params.wmax = value; self.mark_param_given(32); Ok(()) }
            "jmax" => { validate_parameter_jmax(value)?; self.params.jmax = value; self.mark_param_given(33); Ok(()) }
            "vmax" => { validate_parameter_vmax(value)?; self.params.vmax = value; self.mark_param_given(34); Ok(()) }
            "tminclip" => { validate_parameter_tminclip(value)?; self.params.tminclip = value; self.mark_param_given(35); Ok(()) }
            "tmaxclip" => { validate_parameter_tmaxclip(value)?; self.params.tmaxclip = value; self.mark_param_given(36); Ok(()) }
            "rsh" => { validate_parameter_rsh(value)?; self.params.rsh = value; self.mark_param_given(37); Ok(()) }
            "xw" => { validate_parameter_xw(value)?; self.params.xw = value; self.mark_param_given(38); Ok(()) }
            "nwxw" => { validate_parameter_nwxw(value)?; self.params.nwxw = value; self.mark_param_given(39); Ok(()) }
            "wexw" => { validate_parameter_wexw(value)?; self.params.wexw = value; self.mark_param_given(40); Ok(()) }
            "fdrw" => { validate_parameter_fdrw(value)?; self.params.fdrw = value; self.mark_param_given(41); Ok(()) }
            "fdxwinf" => { validate_parameter_fdxwinf(value)?; self.params.fdxwinf = value; self.mark_param_given(42); Ok(()) }
            "xl" => { validate_parameter_xl(value)?; self.params.xl = value; self.mark_param_given(43); Ok(()) }
            "xlw" => { validate_parameter_xlw(value)?; self.params.xlw = value; self.mark_param_given(44); Ok(()) }
            "dxlsat" => { validate_parameter_dxlsat(value)?; self.params.dxlsat = value; self.mark_param_given(45); Ok(()) }
            "nst" => { validate_parameter_nst(value)?; self.params.nst = value; self.mark_param_given(46); Ok(()) }
            "ats" => { validate_parameter_ats(value)?; self.params.ats = value; self.mark_param_given(47); Ok(()) }
            "atsinf" => { validate_parameter_ats(value)?; self.params.ats = value; self.mark_param_given(47); Ok(()) }
            "atsl" => { validate_parameter_atsl(value)?; self.params.atsl = value; self.mark_param_given(48); Ok(()) }
            "dfinf" => { validate_parameter_dfinf(value)?; self.params.dfinf = value; self.mark_param_given(49); Ok(()) }
            "dfw" => { validate_parameter_dfw(value)?; self.params.dfw = value; self.mark_param_given(50); Ok(()) }
            "dfl" => { validate_parameter_dfl(value)?; self.params.dfl = value; self.mark_param_given(51); Ok(()) }
            "dfwl" => { validate_parameter_dfwl(value)?; self.params.dfwl = value; self.mark_param_given(52); Ok(()) }
            "sw_dfgeo" => { validate_parameter_sw_dfgeo(value)?; self.params.sw_dfgeo = value; self.mark_param_given(53); Ok(()) }
            "dp" => { validate_parameter_dp(value)?; self.params.dp = value; self.mark_param_given(54); Ok(()) }
            "dpinf" => { validate_parameter_dp(value)?; self.params.dp = value; self.mark_param_given(54); Ok(()) }
            "dpw" => { validate_parameter_dpw(value)?; self.params.dpw = value; self.mark_param_given(55); Ok(()) }
            "dpwe" => { validate_parameter_dpwe(value)?; self.params.dpwe = value; self.mark_param_given(56); Ok(()) }
            "dpl" => { validate_parameter_dpl(value)?; self.params.dpl = value; self.mark_param_given(57); Ok(()) }
            "dple" => { validate_parameter_dple(value)?; self.params.dple = value; self.mark_param_given(58); Ok(()) }
            "dpwl" => { validate_parameter_dpwl(value)?; self.params.dpwl = value; self.mark_param_given(59); Ok(()) }
            "ecrit" => { validate_parameter_ecrit(value)?; self.params.ecrit = value; self.mark_param_given(60); Ok(()) }
            "ecorn" => { validate_parameter_ecorn(value)?; self.params.ecorn = value; self.mark_param_given(61); Ok(()) }
            "sw_vsatt" => { validate_parameter_sw_vsatt(value)?; self.params.sw_vsatt = value; self.mark_param_given(62); Ok(()) }
            "sw_accpo" => { validate_parameter_sw_accpo(value)?; self.params.sw_accpo = value; self.mark_param_given(63); Ok(()) }
            "grpo" => { validate_parameter_grpo(value)?; self.params.grpo = value; self.mark_param_given(64); Ok(()) }
            "du" => { validate_parameter_du(value)?; self.params.du = value; self.mark_param_given(65); Ok(()) }
            "rc" => { validate_parameter_rc(value)?; self.params.rc = value; self.mark_param_given(66); Ok(()) }
            "rcw" => { validate_parameter_rcw(value)?; self.params.rcw = value; self.mark_param_given(67); Ok(()) }
            "fc" => { validate_parameter_fc(value)?; self.params.fc = value; self.mark_param_given(68); Ok(()) }
            "isa" => { validate_parameter_isa(value)?; self.params.isa = value; self.mark_param_given(69); Ok(()) }
            "na" => { validate_parameter_na(value)?; self.params.na = value; self.mark_param_given(70); Ok(()) }
            "ca" => { validate_parameter_ca(value)?; self.params.ca = value; self.mark_param_given(71); Ok(()) }
            "cja" => { validate_parameter_cja(value)?; self.params.cja = value; self.mark_param_given(72); Ok(()) }
            "pa" => { validate_parameter_pa(value)?; self.params.pa = value; self.mark_param_given(73); Ok(()) }
            "ma" => { validate_parameter_ma(value)?; self.params.ma = value; self.mark_param_given(74); Ok(()) }
            "aja" => { validate_parameter_aja(value)?; self.params.aja = value; self.mark_param_given(75); Ok(()) }
            "isp" => { validate_parameter_isp(value)?; self.params.isp = value; self.mark_param_given(76); Ok(()) }
            "np" => { validate_parameter_np(value)?; self.params.np = value; self.mark_param_given(77); Ok(()) }
            "cp" => { validate_parameter_cp(value)?; self.params.cp = value; self.mark_param_given(78); Ok(()) }
            "cjp" => { validate_parameter_cjp(value)?; self.params.cjp = value; self.mark_param_given(79); Ok(()) }
            "pp" => { validate_parameter_pp(value)?; self.params.pp = value; self.mark_param_given(80); Ok(()) }
            "mp" => { validate_parameter_mp(value)?; self.params.mp = value; self.mark_param_given(81); Ok(()) }
            "ajp" => { validate_parameter_ajp(value)?; self.params.ajp = value; self.mark_param_given(82); Ok(()) }
            "vbv" => { validate_parameter_vbv(value)?; self.params.vbv = value; self.mark_param_given(83); Ok(()) }
            "ibv" => { validate_parameter_ibv(value)?; self.params.ibv = value; self.mark_param_given(84); Ok(()) }
            "nbv" => { validate_parameter_nbv(value)?; self.params.nbv = value; self.mark_param_given(85); Ok(()) }
            "kfn" => { validate_parameter_kfn(value)?; self.params.kfn = value; self.mark_param_given(86); Ok(()) }
            "afn" => { validate_parameter_afn(value)?; self.params.afn = value; self.mark_param_given(87); Ok(()) }
            "bfn" => { validate_parameter_bfn(value)?; self.params.bfn = value; self.mark_param_given(88); Ok(()) }
            "sw_fngeo" => { validate_parameter_sw_fngeo(value)?; self.params.sw_fngeo = value; self.mark_param_given(89); Ok(()) }
            "ea" => { validate_parameter_ea(value)?; self.params.ea = value; self.mark_param_given(90); Ok(()) }
            "xis" => { validate_parameter_xis(value)?; self.params.xis = value; self.mark_param_given(91); Ok(()) }
            "xvsat" => { validate_parameter_xvsat(value)?; self.params.xvsat = value; self.mark_param_given(92); Ok(()) }
            "tc1" => { validate_parameter_tc1(value)?; self.params.tc1 = value; self.mark_param_given(93); Ok(()) }
            "tc2" => { validate_parameter_tc2(value)?; self.params.tc2 = value; self.mark_param_given(94); Ok(()) }
            "tc1l" => { validate_parameter_tc1l(value)?; self.params.tc1l = value; self.mark_param_given(95); Ok(()) }
            "tc2l" => { validate_parameter_tc2l(value)?; self.params.tc2l = value; self.mark_param_given(96); Ok(()) }
            "tc1w" => { validate_parameter_tc1w(value)?; self.params.tc1w = value; self.mark_param_given(97); Ok(()) }
            "tc2w" => { validate_parameter_tc2w(value)?; self.params.tc2w = value; self.mark_param_given(98); Ok(()) }
            "tc1wl" => { validate_parameter_tc1wl(value)?; self.params.tc1wl = value; self.mark_param_given(99); Ok(()) }
            "tc2wl" => { validate_parameter_tc2wl(value)?; self.params.tc2wl = value; self.mark_param_given(100); Ok(()) }
            "tc1rc" => { validate_parameter_tc1rc(value)?; self.params.tc1rc = value; self.mark_param_given(101); Ok(()) }
            "tc2rc" => { validate_parameter_tc2rc(value)?; self.params.tc2rc = value; self.mark_param_given(102); Ok(()) }
            "tc1dp" => { validate_parameter_tc1dp(value)?; self.params.tc1dp = value; self.mark_param_given(103); Ok(()) }
            "tc2dp" => { validate_parameter_tc2dp(value)?; self.params.tc2dp = value; self.mark_param_given(104); Ok(()) }
            "tc1vbv" => { validate_parameter_tc1vbv(value)?; self.params.tc1vbv = value; self.mark_param_given(105); Ok(()) }
            "tc2vbv" => { validate_parameter_tc2vbv(value)?; self.params.tc2vbv = value; self.mark_param_given(106); Ok(()) }
            "tc1nbv" => { validate_parameter_tc1nbv(value)?; self.params.tc1nbv = value; self.mark_param_given(107); Ok(()) }
            "tc1kfn" => { validate_parameter_tc1kfn(value)?; self.params.tc1kfn = value; self.mark_param_given(108); Ok(()) }
            "tegth" => { validate_parameter_tegth(value)?; self.params.tegth = value; self.mark_param_given(109); Ok(()) }
            "gth0" => { validate_parameter_gth0(value)?; self.params.gth0 = value; self.mark_param_given(110); Ok(()) }
            "gthp" => { validate_parameter_gthp(value)?; self.params.gthp = value; self.mark_param_given(111); Ok(()) }
            "gtha" => { validate_parameter_gtha(value)?; self.params.gtha = value; self.mark_param_given(112); Ok(()) }
            "gthc" => { validate_parameter_gthc(value)?; self.params.gthc = value; self.mark_param_given(113); Ok(()) }
            "cth0" => { validate_parameter_cth0(value)?; self.params.cth0 = value; self.mark_param_given(114); Ok(()) }
            "cthp" => { validate_parameter_cthp(value)?; self.params.cthp = value; self.mark_param_given(115); Ok(()) }
            "ctha" => { validate_parameter_ctha(value)?; self.params.ctha = value; self.mark_param_given(116); Ok(()) }
            "cthc" => { validate_parameter_cthc(value)?; self.params.cthc = value; self.mark_param_given(117); Ok(()) }
            "nsig_rsh" => { validate_parameter_nsig_rsh(value)?; self.params.nsig_rsh = value; self.mark_param_given(118); Ok(()) }
            "nsig_w" => { validate_parameter_nsig_w(value)?; self.params.nsig_w = value; self.mark_param_given(119); Ok(()) }
            "nsig_l" => { validate_parameter_nsig_l(value)?; self.params.nsig_l = value; self.mark_param_given(120); Ok(()) }
            "sig_rsh" => { validate_parameter_sig_rsh(value)?; self.params.sig_rsh = value; self.mark_param_given(121); Ok(()) }
            "sig_w" => { validate_parameter_sig_w(value)?; self.params.sig_w = value; self.mark_param_given(122); Ok(()) }
            "sig_l" => { validate_parameter_sig_l(value)?; self.params.sig_l = value; self.mark_param_given(123); Ok(()) }
            "smm_rsh" => { validate_parameter_smm_rsh(value)?; self.params.smm_rsh = value; self.mark_param_given(124); Ok(()) }
            "smm_w" => { validate_parameter_smm_w(value)?; self.params.smm_w = value; self.mark_param_given(125); Ok(()) }
            "smm_l" => { validate_parameter_smm_l(value)?; self.params.smm_l = value; self.mark_param_given(126); Ok(()) }
            "sw_mmgeo" => { validate_parameter_sw_mmgeo(value)?; self.params.sw_mmgeo = value; self.mark_param_given(127); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'r3_cmc'", name)),
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

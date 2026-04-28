use super::*;

impl Jfet {
    /// Set model parameters from a HashMap (for .MODEL statement parsing)
    pub fn with_model_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let mut p = self.params.clone();

        if let Some(level) = params.get("LEVEL").copied().filter(|v| v.is_finite()) {
            p.hfet_level = level.round() as i32;
        }

        if let Some(v) = params
            .get("VTO")
            .or_else(|| params.get("VT0"))
            .copied()
            .filter(|v| v.is_finite())
        {
            p.vto = v;
        }

        let beta_from_card = params
            .get("BETA")
            .copied()
            .or_else(|| {
                (!matches!(p.channel_model, JfetChannelModel::LegacyMesfet))
                    .then(|| params.get("B").copied())
                    .flatten()
            })
            .filter(|v| v.is_finite() && *v >= 0.0);
        let idss_from_card = params
            .get("IDSS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0);
        if let Some(beta) = beta_from_card {
            p.beta = beta;
        } else if let Some(idss) = idss_from_card {
            let vto2 = p.vto * p.vto;
            if vto2 > 1e-30 {
                p.beta = idss / vto2;
            }
        }

        if let Some(v) = params
            .get("LAMBDA")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.lambda = v;
        }
        if let Some(v) = params
            .get("IS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.is = v;
        }
        if let Some(v) = params
            .get("JS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.is = v;
        }
        if let Some(v) = params
            .get("CGS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.cgs = v;
        }
        if let Some(v) = params
            .get("CGD")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.cgd = v;
        }
        if let Some(v) = params
            .get("PB")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.pb = v;
        }
        if let Some(v) = params
            .get("M")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            if matches!(p.channel_model, JfetChannelModel::Hfet1) {
                p.hfet_m = v;
            } else {
                p.m = v;
            }
        }

        if matches!(p.channel_model, JfetChannelModel::Hfet1) {
            if let Some(v) = params
                .get("RD")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rd = v;
            }
            if let Some(v) = params
                .get("RS")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rs = v;
            }
            if let Some(v) = params
                .get("RDI")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.hfet_rdi = v;
            }
            if let Some(v) = params
                .get("RSI")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.hfet_rsi = v;
            }
        } else {
            if let Some(v) = params
                .get("RD")
                .or_else(|| params.get("RDI"))
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rd = v;
            }
            if let Some(v) = params
                .get("RS")
                .or_else(|| params.get("RSI"))
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rs = v;
            }
        }

        if let Some(v) = params
            .get("FC")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0 && *v < 1.0)
        {
            p.fc = v;
        }
        if let Some(v) = params
            .get("N")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.n = v;
        }
        if let Some(v) = params.get("ETA").copied().filter(|v| v.is_finite()) {
            p.eta = v;
        }
        if let Some(v) = params.get("THETA").copied().filter(|v| v.is_finite()) {
            p.mesa_theta = v;
        }
        if let Some(v) = params.get("ALPHA").copied().filter(|v| v.is_finite()) {
            p.mesa_alpha = v;
        }
        if let Some(v) = params
            .get("B")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mes_b = v;
        }
        if let Some(v) = params.get("TC").copied().filter(|v| v.is_finite()) {
            p.mesa_tc = v;
        }
        if let Some(v) = params
            .get("ZETA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_zeta = v;
        }
        if let Some(v) = params
            .get("LAMBDAHF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_lambdahf = v;
        }
        if let Some(v) = params
            .get("SIGMA0")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.sigma0 = v;
        }
        if let Some(v) = params
            .get("KF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.kf = v;
        }
        if let Some(v) = params
            .get("AF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.af = v;
        }
        if let Some(v) = params
            .get("EF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.ef = v;
        }

        if let Some(v) = params
            .get("MC")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_mc = v;
        }
        if let Some(v) = params
            .get("GAMMA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_gamma = v;
        }
        if let Some(v) = params.get("VSIGMAT").copied().filter(|v| v.is_finite()) {
            p.hfet_vsigmat = v;
        }
        if let Some(v) = params
            .get("VSIGMA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_vsigma = v;
        }
        if let Some(v) = params
            .get("MU")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_mu = v;
        }
        if let Some(v) = params
            .get("DI")
            .or_else(|| params.get("D"))
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_di = v;
        }
        if let Some(v) = params
            .get("DU")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_du = v;
        }
        if let Some(v) = params
            .get("ND")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_nd = v;
        }
        if let Some(v) = params
            .get("NDU")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_ndu = v;
        }
        if let Some(v) = params
            .get("TH")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_th = v;
        }
        if let Some(v) = params
            .get("NDELTA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_ndelta = v;
        }
        if let Some(v) = params
            .get("DELTA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_delta = v;
        }
        if let Some(v) = params
            .get("VS")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_vs = v;
        }
        if let Some(v) = params
            .get("NMAX")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_nmax = v;
        }
        if let Some(v) = params
            .get("DELTAD")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_deltad = v;
        }
        if let Some(v) = params
            .get("EPSI")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_epsi = v;
        }
        if let Some(v) = params
            .get("CAS")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_cas = v;
        }
        if let Some(v) = params
            .get("CBS")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_cbs = v;
        }
        if let Some(v) = params
            .get("JS1S")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js1s = v;
        }
        if let Some(v) = params
            .get("JS2S")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js2s = v;
        }
        if let Some(v) = params
            .get("JS1D")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js1d = v;
        }
        if let Some(v) = params
            .get("JS2D")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js2d = v;
        }
        if let Some(v) = params
            .get("M1S")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m1s = v;
        }
        if let Some(v) = params
            .get("M2S")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m2s = v;
        }
        if let Some(v) = params
            .get("M1D")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m1d = v;
        }
        if let Some(v) = params
            .get("M2D")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m2d = v;
        }
        if let Some(v) = params
            .get("RGS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_rgs = v;
        }
        if let Some(v) = params
            .get("RGD")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_rgd = v;
        }
        if let Some(v) = params
            .get("GGR")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_ggr = v;
        }
        if let Some(v) = params.get("DEL").copied().filter(|v| v.is_finite()) {
            p.hfet_del = v;
        }
        if let Some(v) = params
            .get("ETA1")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_eta1 = v;
        }
        if let Some(v) = params
            .get("D1")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_d1 = v;
        }
        if let Some(v) = params.get("VT1").copied().filter(|v| v.is_finite()) {
            p.hfet_vt1 = v;
        }
        if let Some(v) = params
            .get("P")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_p = v;
        }
        if let Some(v) = params.get("KAPPA").copied().filter(|v| v.is_finite()) {
            p.hfet_kappa = v;
        }
        if let Some(v) = params
            .get("DELF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_delf_freq = v;
        }
        if let Some(v) = params
            .get("FGDS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_fgds = v;
        }
        if let Some(v) = params
            .get("CDS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_cds = v;
        }
        if let Some(v) = params
            .get("ASTAR")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_astar = v;
        }
        if let Some(v) = params
            .get("PHIB")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_phib = v;
        }
        if let Some(v) = params.get("XCHI").copied().filter(|v| v.is_finite()) {
            p.mesa_xchi = v;
        }
        if let Some(v) = params.get("TF").copied().filter(|v| v.is_finite()) {
            let tf_k = v + 273.15;
            if matches!(p.hfet_level, 2..=4) {
                p.mesa_tf = tf_k;
            } else {
                p.hfet_tf = tf_k;
            }
        }
        if let Some(v) = params
            .get("FLO")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_flo = v;
        }
        if let Some(v) = params
            .get("DELFO")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_delfo = v;
        }
        if let Some(v) = params
            .get("TNOM")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.tnom = v;
        }
        self.params = p;
        self
    }
}

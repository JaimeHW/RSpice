use super::*;

impl<'a> NetlistGenerator<'a> {
    pub(super) fn get_bjt_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            // Explicit model selected by user: trust it and do NOT inject a generic
            // .MODEL card that could silently override a library model.
            return model_name.to_string();
        }

        let polarity = if matches!(
            component.kind,
            ComponentType::NpnBjt | ComponentType::NpnBjt4 | ComponentType::NpnBjt5
        ) {
            "NPN"
        } else {
            "PNP"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} {} (BF=100 IS=1e-15)", model_name, polarity),
            );
        }

        model_name
    }

    /// Get a native VBIC 1.3 model for the five-terminal thermal BJT —
    /// the only built-in family that solves the dT terminal.
    pub(super) fn get_vbic_bjt_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let polarity = if component.kind == ComponentType::NpnBjt5 {
            "NPN"
        } else {
            "PNP"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} {} (LEVEL=4 IS=1e-16 RTH=50)", model_name, polarity),
            );
        }

        model_name
    }

    /// Get a MESFET model name and add to models. HFET behavior comes from
    /// binding an NHFET/PHFET (or LEVEL=5/6) library card instead.
    pub(super) fn get_mesfet_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let polarity = if component.kind == ComponentType::Nmesfet {
            "NMF"
        } else {
            "PMF"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} {} (VTO=-2 BETA=1e-4)", model_name, polarity),
            );
        }

        model_name
    }

    /// Get a partially-depleted BSIMSOI card for the five-terminal SOI
    /// MOSFET; RBODY keeps the body contact resistively tied.
    pub(super) fn get_soi_mosfet_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let n_channel = component.kind == ComponentType::NmosSoi;
        let model_name = format!(
            "{}_{}",
            if n_channel { "nmossoi" } else { "pmossoi" },
            component.name
        );

        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} {} (LEVEL=57 RBODY=1)",
                    model_name,
                    if n_channel { "NMOS" } else { "PMOS" }
                ),
            );
        }

        model_name
    }

    /// Get a current-controlled switch model (CSW) and add to models.
    pub(super) fn get_iswitch_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("isw_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let params = crate::properties::parse_params_string(&component.params);
            let it = Self::get_param_owned(&params, "it", "", "1m");
            let ih = Self::get_param_owned(&params, "ih", "", "0");
            let ron = Self::get_param_owned(&params, "ron", "", "1");
            let roff = Self::get_param_owned(&params, "roff", "", "1meg");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} CSW (IT={} IH={} RON={} ROFF={})",
                    model_name, it, ih, ron, roff
                ),
            );
        }

        model_name
    }

    /// Get a lossy transmission-line model card (LTRA by default, TXL when
    /// selected) carrying the per-length RLGC values.
    pub(super) fn get_lossy_tline_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let params = crate::properties::parse_params_string(&component.params);
        let txl = params.get("kind").is_some_and(|kind| kind == "txl");
        let model_name = format!("{}_{}", if txl { "txl" } else { "ltra" }, component.name);

        if !self.models.contains_key(&model_name) {
            let r = Self::get_param_owned(&params, "r", "", "1");
            let l = Self::get_param_owned(&params, "l", "", "250n");
            let c = Self::get_param_owned(&params, "c", "", "100p");
            let len = Self::get_param_owned(&params, "len", "", "1");
            let card = if txl {
                let g = Self::get_param_owned(&params, "g", "", "0");
                format!(
                    ".MODEL {} TXL (R={} L={} G={} C={} LENGTH={})",
                    model_name, r, l, g, c, len
                )
            } else {
                // The native LTRA runtime requires G=0.
                format!(
                    ".MODEL {} LTRA (R={} L={} C={} G=0 LEN={})",
                    model_name, r, l, c, len
                )
            };
            self.models.insert(model_name.clone(), card);
        }

        model_name
    }

    /// Get a two-conductor CPL model card. R/L/C/G are upper-triangle
    /// (11, 12, 22) per-length entries; the core validates passivity.
    pub(super) fn get_cpl_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("cpl_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let params = crate::properties::parse_params_string(&component.params);
            let entry = |key: &str, default: &str| Self::get_param_owned(&params, key, "", default);
            // The CPL card body is re-read from raw source text by a
            // line-oriented parser: one matrix key per `+` continuation.
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} CPL\n+ R = ({} {} {})\n+ L = ({} {} {})\n+ C = ({} {} {})\n+ G = ({} {} {})\n+ LENGTH = {}",
                    model_name,
                    entry("r11", "0.1"),
                    entry("r12", "0"),
                    entry("r22", "0.1"),
                    entry("l11", "380n"),
                    entry("l12", "60n"),
                    entry("l22", "380n"),
                    entry("c11", "120p"),
                    entry("c12", "-12p"),
                    entry("c22", "120p"),
                    entry("g11", "0"),
                    entry("g12", "0"),
                    entry("g22", "0"),
                    entry("length", "0.1"),
                ),
            );
        }

        model_name
    }

    /// Get a memristor model card (Xyce TEAM, LEVEL=2).
    pub(super) fn get_memristor_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("mem_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let params = crate::properties::parse_params_string(&component.params);
            let ron = Self::get_param_owned(&params, "ron", "", "50");
            let roff = Self::get_param_owned(&params, "roff", "", "1k");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} MEMRISTOR (LEVEL=2 RON={} ROFF={})",
                    model_name, ron, roff
                ),
            );
        }

        model_name
    }

    /// Get MOSFET model name and add to models
    pub(super) fn get_mosfet_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            // Explicit model selected by user: trust it and do NOT inject a generic
            // .MODEL card that could silently override a library model.
            return model_name.to_string();
        }

        let polarity = if component.kind == ComponentType::Nmos {
            "NMOS"
        } else {
            "PMOS"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} {} (LEVEL=1 VTO={} KP=2e-5)",
                    model_name,
                    polarity,
                    if component.kind == ComponentType::Nmos {
                        "0.7"
                    } else {
                        "-0.7"
                    }
                ),
            );
        }

        model_name
    }

    /// Get VDMOS power-MOSFET model name and add to models.
    ///
    /// The VDMOS body diode, drift resistance, and thermal behavior all
    /// live on the model card; the default card gives a generic enhancement
    /// power FET so a bare placement simulates out of the box.
    pub(super) fn get_vdmos_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let n_channel = component.kind == ComponentType::NVdmos;
        let model_name = format!(
            "{}_{}",
            if n_channel { "nvdmos" } else { "pvdmos" },
            component.name
        );

        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} {} (VTO={} KP=20 RD=0.1 RS=0.05)",
                    model_name,
                    if n_channel { "NVDMOS" } else { "PVDMOS" },
                    if n_channel { "3" } else { "-3" }
                ),
            );
        }

        model_name
    }

    /// Get diode model name and add to models.
    ///
    /// A bare diode placement gets a generic junction card so the deck is
    /// always executable; an explicit model binding is trusted verbatim.
    pub(super) fn get_diode_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            return model_name.to_string();
        }

        let model_name = format!("d_{}", component.name);

        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} D (IS=1e-14 N=1 RS=0)", model_name),
            );
        }

        model_name
    }

    /// Get the Jiles-Atherton magnetic-core model for a saturable inductor.
    ///
    /// The core parameters (saturation magnetization, anhysteretic shape,
    /// pinning, geometry, turns) ride on a `.MODEL <name> CORE` card built
    /// from the component's property values. An explicit `model=` override
    /// binds a library core model instead.
    pub(super) fn get_saturable_core_model(&mut self, component: &Component) -> String {
        let mut params = crate::properties::parse_params_string(&component.params);
        if let Some(model_name) = params
            .remove("model")
            .map(|m| m.trim().to_string())
            .filter(|m| !m.is_empty())
        {
            return model_name;
        }

        let model_name = format!("core_{}", component.name);

        if !self.models.contains_key(&model_name) {
            let ms = Self::get_param_owned(&params, "ms", "", "0.4");
            let a = Self::get_param_owned(&params, "a", "", "200");
            let k = Self::get_param_owned(&params, "k", "", "150");
            let c = Self::get_param_owned(&params, "c", "", "0.1");
            let alpha = Self::get_param_owned(&params, "alpha", "", "1e-4");
            let area = Self::get_param_owned(&params, "area", "", "1e-4");
            let length = Self::get_param_owned(&params, "length", "", "0.1");
            let turns = Self::get_param_owned(&params, "n", "", "100");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} CORE (MS={} A={} K={} C={} ALPHA={} AREA={} LENGTH={} N={})",
                    model_name, ms, a, k, c, alpha, area, length, turns
                ),
            );
        }

        model_name
    }

    /// Get voltage-controlled switch model name and add to models
    pub(super) fn get_switch_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            // Explicit model selected by user: trust it and do NOT inject a generic
            // .MODEL card that could silently override a library model.
            return model_name.to_string();
        }

        let model_name = format!("sw_{}", component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            let params = crate::properties::parse_params_string(&component.params);
            let vt = Self::get_param_owned(&params, "vt", "", "0");
            let vh = Self::get_param_owned(&params, "vh", "", "0");
            let ron = Self::get_param_owned(&params, "ron", "", "1");
            let roff = Self::get_param_owned(&params, "roff", "", "1e12");
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} SW (VT={} VH={} RON={} ROFF={})",
                    model_name, vt, vh, ron, roff
                ),
            );
        }

        model_name
    }

    /// Get JFET model name and add to models
    pub(super) fn get_jfet_model(
        &mut self,
        component: &Component,
        explicit_model: Option<&str>,
    ) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            // Explicit model selected by user: trust it and do NOT inject a generic
            // .MODEL card that could silently override a library model.
            return model_name.to_string();
        }

        let polarity = if component.kind == ComponentType::Njfet {
            "NJF"
        } else {
            "PJF"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} {} (VTO=-2 BETA=1e-4)", model_name, polarity),
            );
        }

        model_name
    }

    //-------------------------------------------------------------------------
    // Phase 5: Model Generation
    //-------------------------------------------------------------------------

    /// Generate model statements
    pub(super) fn generate_models(&mut self) {
        if !self.models.is_empty() {
            self.lines.push(String::new());
            self.lines.push("* Models".to_string());
            for model_line in self.models.values() {
                self.lines.push(model_line.clone());
            }
        }
    }
}

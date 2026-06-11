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

        let polarity = if component.kind == ComponentType::NpnBjt {
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

    /// Get MOSFET model name and add to models
    pub(super) fn get_mosfet_model(&mut self, component: &Component) -> String {
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
    pub(super) fn get_jfet_model(&mut self, component: &Component) -> String {
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

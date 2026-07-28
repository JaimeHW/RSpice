//! SPICE value and card formatting.
//!
//! Writes numbers in the engineering form a SPICE parser expects, and lays
//! out cards with continuations where a line would otherwise be too long.

use super::*;

impl<'a> NetlistGenerator<'a> {
    pub(super) fn get_node_name(&self, point: Point) -> String {
        if let Some(&net_id) = self.point_to_net.get(&point)
            && let Some(net) = self.nets.iter().find(|n| n.id == net_id)
        {
            return net.spice_name();
        }
        // Floating terminal - assign a unique net
        format!(
            "float_{}",
            point.x.unsigned_abs() * 10000 + point.y.unsigned_abs()
        )
    }

    pub(super) fn quote_path_for_netlist(path: &str) -> String {
        let escaped = path.replace('"', "\\\"");
        format!("\"{}\"", escaped)
    }

    pub(super) fn instance_name(&self, component: &Component) -> String {
        let base = component.spice_instance_name();
        let prefix = component
            .library_cell
            .as_ref()
            .filter(|binding| binding.netlist_template.is_some())
            .and_then(crate::state::LibraryCellInstance::effective_reference_prefix)
            .unwrap_or_else(|| component.kind.spice_prefix());

        if prefix.is_empty() || base.is_empty() {
            return base;
        }

        if base.len() >= prefix.len() && base[..prefix.len()].eq_ignore_ascii_case(prefix) {
            base
        } else {
            format!("{}{}", prefix, base)
        }
    }

    pub(super) fn filter_component_params(&self, params: &str, excluded: &[&str]) -> String {
        if params.trim().is_empty() {
            return String::new();
        }

        let mut params_map = crate::state::parse_params_string(params);
        for key in excluded {
            params_map.remove(&key.to_ascii_lowercase());
        }
        crate::state::format_params_string(&params_map)
    }

    pub(super) fn format_nodes(&self, nodes: &[String], expected: usize) -> String {
        if nodes.len() >= expected {
            nodes[..expected].join(" ")
        } else {
            // Pad with ground if not enough terminals
            let mut result = nodes.to_vec();
            while result.len() < expected {
                result.push("0".to_string());
            }
            result.join(" ")
        }
    }

    /// Format component value with SI prefixes
    pub(super) fn format_value(&self, value: &str) -> String {
        if value.is_empty() {
            return "1".to_string();
        }
        // Already has SPICE-compatible format
        value.to_string()
    }

    /// Format component parameters for SPICE netlist
    ///
    /// Converts the component.params string into proper SPICE format.
    /// Parameters are appended after the value/model in the netlist line.
    ///
    /// # SPICE Parameter Format (Cadence Spectre Parity)
    ///
    /// Passive components: `R1 net1 net2 1k m=2 tc1=0.01`
    /// Sources: `V1 net1 0 DC 5 acmag=1`
    /// MOSFETs: `M1 d g s b nmos w=1u l=180n`
    ///
    /// # Arguments
    /// * `params` - The component.params string (e.g., "m=2 tc1=0.01")
    ///
    /// # Returns
    /// Formatted parameter string with leading space if non-empty
    pub(super) fn format_params(&self, params: &str) -> String {
        let trimmed = params.trim();
        if trimmed.is_empty() {
            String::new()
        } else {
            // Ensure single space separation from previous content
            format!(" {}", trimmed)
        }
    }

    /// Format component value with optional parameters appended
    ///
    /// Commercial-grade helper that combines value and params into
    /// proper SPICE format: `value [params]`
    pub(super) fn format_value_with_params(&self, value: &str, params: &str) -> String {
        let formatted_value = self.format_value(value);
        let formatted_params = self.format_params(params);
        format!("{}{}", formatted_value, formatted_params)
    }

    /// Format source value specification
    pub(super) fn format_source_value(&self, component: &Component) -> String {
        let value = &component.value;

        match component.kind {
            ComponentType::VoltageSource | ComponentType::CurrentSource => {
                format!("DC {}", if value.is_empty() { "0" } else { value })
            }
            ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => {
                format!("AC {}", if value.is_empty() { "1" } else { value })
            }
            ComponentType::VoltageSourcePulse => {
                // PULSE(V1 V2 TD TR TF PW PER)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "PULSE")
                {
                    return literal;
                }
                let v1 = Self::get_param_owned(&params, "v1", value, "0");
                let v2 = Self::get_param_owned(&params, "v2", "", "1");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let tr = Self::get_param_owned(&params, "tr", "", "1n");
                let tf = Self::get_param_owned(&params, "tf", "", "1n");
                let pw = Self::get_param_owned(&params, "pw", "", "1u");
                let per = Self::get_param_owned_with_aliases(&params, &["per", "period"], "", "2u");
                format!("PULSE({} {} {} {} {} {} {})", v1, v2, td, tr, tf, pw, per)
            }
            ComponentType::CurrentSourcePulse => {
                // PULSE(I1 I2 TD TR TF PW PER)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "PULSE")
                {
                    return literal;
                }
                let i1 = Self::get_param_owned(&params, "i1", value, "0");
                let i2 = Self::get_param_owned(&params, "i2", "", "1m");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let tr = Self::get_param_owned(&params, "tr", "", "1n");
                let tf = Self::get_param_owned(&params, "tf", "", "1n");
                let pw = Self::get_param_owned(&params, "pw", "", "1u");
                let per = Self::get_param_owned_with_aliases(&params, &["per", "period"], "", "2u");
                format!("PULSE({} {} {} {} {} {} {})", i1, i2, td, tr, tf, pw, per)
            }
            ComponentType::VoltageSourceSin => {
                // SIN(VO VA FREQ TD THETA PHASE)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "SIN")
                {
                    return literal;
                }
                let vo = Self::get_param_owned(&params, "vo", value, "0");
                let va = Self::get_param_owned(&params, "va", "", "1");
                let freq = Self::get_param_owned(&params, "freq", "", "1Meg");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let theta = Self::get_param_owned(&params, "theta", "", "0");
                let phase = Self::get_param_owned(&params, "phase", "", "0");
                format!("SIN({} {} {} {} {} {})", vo, va, freq, td, theta, phase)
            }
            ComponentType::CurrentSourceSin => {
                // SIN(IO IA FREQ TD THETA PHASE)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "SIN")
                {
                    return literal;
                }
                let io = Self::get_param_owned(&params, "io", value, "0");
                let ia = Self::get_param_owned(&params, "ia", "", "1m");
                let freq = Self::get_param_owned(&params, "freq", "", "1Meg");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let theta = Self::get_param_owned(&params, "theta", "", "0");
                let phase = Self::get_param_owned(&params, "phase", "", "0");
                format!("SIN({} {} {} {} {} {})", io, ia, freq, td, theta, phase)
            }
            ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
                // PWL(T1 V1 T2 V2 ...)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "PWL")
                {
                    return literal;
                }
                let pwl_data = Self::get_param_owned(&params, "pwl_data", value, "0 0 1n 1");
                format!("PWL({})", pwl_data)
            }
            ComponentType::VoltageSourceExp => {
                // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "EXP")
                {
                    return literal;
                }
                let v1 = Self::get_param_owned(&params, "v1", value, "0");
                let v2 = Self::get_param_owned(&params, "v2", "", "1");
                let td1 = Self::get_param_owned(&params, "td1", "", "0");
                let tau1 = Self::get_param_owned(&params, "tau1", "", "1u");
                let td2 = Self::get_param_owned(&params, "td2", "", "5u");
                let tau2 = Self::get_param_owned(&params, "tau2", "", "1u");
                format!("EXP({} {} {} {} {} {})", v1, v2, td1, tau1, td2, tau2)
            }
            ComponentType::CurrentSourceExp => {
                // EXP(I1 I2 TD1 TAU1 TD2 TAU2)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "EXP")
                {
                    return literal;
                }
                let i1 = Self::get_param_owned(&params, "i1", value, "0");
                let i2 = Self::get_param_owned(&params, "i2", "", "1m");
                let td1 = Self::get_param_owned(&params, "td1", "", "0");
                let tau1 = Self::get_param_owned(&params, "tau1", "", "1u");
                let td2 = Self::get_param_owned(&params, "td2", "", "5u");
                let tau2 = Self::get_param_owned(&params, "tau2", "", "1u");
                format!("EXP({} {} {} {} {} {})", i1, i2, td1, tau1, td2, tau2)
            }
            ComponentType::CurrentSourceNoise => {
                // DC <offset> TRNOISE(NA NT NALPHA NAMP): white noise via
                // NA/NT, optional 1/f via NALPHA/NAMP. The core requires
                // NT > 0 whenever NA or NAMP is nonzero, and 0 < NALPHA < 2
                // when NAMP is nonzero.
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "TRNOISE")
                {
                    return literal;
                }
                let na = Self::get_param_owned(&params, "na", value, "1n");
                let nt = Self::get_param_owned(&params, "nt", "", "1u");
                let nalpha = Self::get_param_owned(&params, "nalpha", "", "0");
                let namp = Self::get_param_owned(&params, "namp", "", "0");
                let dc = Self::get_param_owned(&params, "dc", "", "0");
                format!("DC {} TRNOISE({} {} {} {})", dc, na, nt, nalpha, namp)
            }
            ComponentType::VoltageSourceSffm => {
                // SFFM(VO VA FC MDI FS)
                let params = crate::state::parse_params_string(&component.params);
                if params.is_empty()
                    && let Some(literal) = Self::legacy_waveform_literal(value, "SFFM")
                {
                    return literal;
                }
                let vo = Self::get_param_owned(&params, "vo", value, "0");
                let va = Self::get_param_owned(&params, "va", "", "1");
                let fc = Self::get_param_owned(&params, "fc", "", "1Meg");
                let mdi = Self::get_param_owned(&params, "mdi", "", "1");
                let fs = Self::get_param_owned(&params, "fs", "", "1k");
                format!("SFFM({} {} {} {} {})", vo, va, fc, mdi, fs)
            }
            _ => {
                if value.is_empty() {
                    "DC 0".to_string()
                } else {
                    value.to_string()
                }
            }
        }
    }

    /// Helper to get param value with fallbacks (returns owned String)
    pub(super) fn get_param_owned(
        params: &HashMap<String, String>,
        key: &str,
        value_fallback: &str,
        default: &str,
    ) -> String {
        if let Some(v) = params.get(key)
            && !v.is_empty()
        {
            return v.clone();
        }
        if !value_fallback.is_empty() {
            value_fallback.to_string()
        } else {
            default.to_string()
        }
    }

    fn get_param_owned_with_aliases(
        params: &HashMap<String, String>,
        keys: &[&str],
        value_fallback: &str,
        default: &str,
    ) -> String {
        for key in keys {
            if let Some(value) = params.get(*key)
                && !value.is_empty()
            {
                return value.clone();
            }
        }
        if !value_fallback.is_empty() {
            value_fallback.to_owned()
        } else {
            default.to_owned()
        }
    }

    /// Accept the waveform literals emitted by older project/example data
    /// without ever prefixing the function name a second time. Canonical
    /// property-backed components store the first positional value in
    /// `Component::value` and the remaining fields in `Component::params`.
    fn legacy_waveform_literal(value: &str, waveform: &str) -> Option<String> {
        let literal = value.trim();
        if literal.contains(['\r', '\n']) || !literal.ends_with(')') {
            return None;
        }
        let (name, _) = literal.split_once('(')?;
        name.trim()
            .eq_ignore_ascii_case(waveform)
            .then(|| literal.to_owned())
    }

    /// Extract optional explicit model name and params string with model= removed.
    ///
    /// Users can provide model either in the primary value field (e.g. "2N2222")
    /// or as `model=<name>` in params. When both are present, params wins.
    pub(super) fn extract_model_override(component: &Component) -> (Option<String>, String) {
        let mut params_map = crate::state::parse_params_string(&component.params);
        let explicit_from_params = params_map
            .remove("model")
            .map(|m| m.trim().to_string())
            .filter(|m| !m.is_empty());
        let params_without_model =
            crate::state::format_params_string(&params_map);

        let explicit_model = explicit_from_params.or_else(|| {
            let value_model = component.value.trim();
            if value_model.is_empty() {
                None
            } else {
                Some(value_model.to_string())
            }
        });

        (explicit_model, params_without_model)
    }
}

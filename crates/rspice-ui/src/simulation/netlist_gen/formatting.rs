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
            .filter(|binding| binding.netlist_template.is_some() || binding.is_executable_builtin())
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
        let original = params.trim();
        let mut runtime_params = crate::state::parse_params_string(original);
        let removed_editor_metadata = runtime_params.remove("model_library").is_some()
            | runtime_params.remove("model_corner").is_some();
        let canonical;
        let trimmed = if removed_editor_metadata {
            canonical = crate::state::format_params_string(&runtime_params);
            canonical.trim()
        } else {
            original
        };
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

    /// Absolute spelling of a data-file reference stored in a component.
    ///
    /// Project files record the path relative to the project folder so a design
    /// survives being moved or handed to someone else; the engine opens what it
    /// is given and does not resolve against the deck, so the reference has to
    /// be made absolute here. An already-absolute path is the user's own choice
    /// of a file outside the project and is left alone.
    pub(super) fn resolve_data_file_path(&self, path: &str) -> String {
        let trimmed = path.trim();
        let Some(root) = self.hierarchy.and_then(HierarchySource::data_root) else {
            return trimmed.to_owned();
        };
        if trimmed.is_empty() || std::path::Path::new(trimmed).is_absolute() {
            return trimmed.to_owned();
        }
        root.join(trimmed).to_string_lossy().into_owned()
    }

    /// Format source value specification
    pub(super) fn format_source_value(&self, component: &Component) -> String {
        let value = &component.value;

        match component.kind {
            ComponentType::VoltageSource | ComponentType::CurrentSource => {
                format!("DC {}", if value.is_empty() { "0" } else { value })
            }
            ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => {
                let params = crate::state::parse_params_string(&component.params);
                let phase =
                    Self::get_param_owned_with_aliases(&params, &["acphase", "phase"], "", "0");
                let dc = Self::get_param_owned(&params, "dc", "", "0");
                format!(
                    "DC {} AC {} {}",
                    dc,
                    if value.is_empty() { "1" } else { value },
                    phase
                )
            }
            ComponentType::VoltageSourcePulse => {
                // PULSE(V1 V2 TD TR TF PW PER)
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &["v1", "v2", "td", "tr", "tf", "pw", "per", "period", "phase"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "PULSE")
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
                let phase = Self::get_param_owned(&params, "phase", "", "0");
                format!(
                    "PULSE({})",
                    Self::waveform_arguments(&[v1, v2, td, tr, tf, pw, per], &[(phase, "0")])
                )
            }
            ComponentType::CurrentSourcePulse => {
                // PULSE(I1 I2 TD TR TF PW PER)
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &["i1", "i2", "td", "tr", "tf", "pw", "per", "period", "phase"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "PULSE")
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
                let phase = Self::get_param_owned(&params, "phase", "", "0");
                format!(
                    "PULSE({})",
                    Self::waveform_arguments(&[i1, i2, td, tr, tf, pw, per], &[(phase, "0")])
                )
            }
            ComponentType::VoltageSourceSin => {
                // SIN(VO VA FREQ TD THETA PHASE)
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &["vo", "va", "freq", "td", "theta", "phase"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "SIN")
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
                if !Self::has_any_source_parameter(
                    &params,
                    &["io", "ia", "freq", "td", "theta", "phase"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "SIN")
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
                if !Self::has_any_source_parameter(&params, &["pwl_data", "td", "repeat"])
                    && let Some(literal) = Self::legacy_waveform_literal(value, "PWL")
                {
                    return literal;
                }
                let pwl_data = Self::get_param_owned(&params, "pwl_data", value, "0 0 1n 1");
                let mut specification = format!("PWL({})", pwl_data);
                if let Some(delay) = params.get("td").filter(|delay| {
                    !delay.trim().is_empty() && !matches!(delay.trim(), "0" | "0.0" | "0s")
                }) {
                    specification.push_str(&format!(" TD={delay}"));
                }
                if params
                    .get("repeat")
                    .is_some_and(|repeat| repeat.eq_ignore_ascii_case("true") || repeat == "1")
                {
                    specification.push_str(" R=0");
                }
                specification
            }
            ComponentType::VoltageSourcePwlFile | ComponentType::CurrentSourcePwlFile => {
                // PWL FILE="path" [TD=][R=][TSCALE=][VSCALE=][TOFFSET=][VOFFSET=]
                let params = crate::state::parse_params_string(&component.params);
                let path = Self::get_param_owned(&params, "file", value, "");
                // The path is always quoted: the reader refuses a bare path
                // containing '=', and a directory name may hold spaces.
                let mut specification =
                    format!("PWL FILE=\"{}\"", self.resolve_data_file_path(&path));

                // TSCALE has no zero-valued spelling and the offsets have no
                // unit one, so each modifier states its own unset value rather
                // than sharing a single "is this blank" test.
                for (key, keyword, unset) in [
                    ("td", "TD", "0"),
                    ("r", "R", ""),
                    ("tscale", "TSCALE", "1"),
                    ("vscale", "VSCALE", "1"),
                    ("toffset", "TOFFSET", "0"),
                    ("voffset", "VOFFSET", "0"),
                ] {
                    let value = Self::get_param_owned(&params, key, "", "");
                    if !value.trim().is_empty() && value.trim() != unset {
                        specification.push_str(&format!(" {keyword}={}", value.trim()));
                    }
                }
                specification
            }
            ComponentType::VoltageSourceExp => {
                // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &["v1", "v2", "td1", "tau1", "td2", "tau2"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "EXP")
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
                if !Self::has_any_source_parameter(
                    &params,
                    &["i1", "i2", "td1", "tau1", "td2", "tau2"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "EXP")
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
            ComponentType::VoltageSourceNoise | ComponentType::CurrentSourceNoise => {
                // DC <offset> TRNOISE(NA NT NALPHA NAMP): white noise via
                // NA/NT, optional 1/f via NALPHA/NAMP. The core requires
                // NT > 0 whenever NA or NAMP is nonzero, and 0 < NALPHA < 2
                // when NAMP is nonzero.
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &["dc", "na", "nt", "nalpha", "namp", "isnoisy"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "TRNOISE")
                {
                    return literal;
                }
                let na = Self::get_param_owned(&params, "na", value, "1n");
                let nt = Self::get_param_owned(&params, "nt", "", "1u");
                let nalpha = Self::get_param_owned(&params, "nalpha", "", "0");
                let namp = Self::get_param_owned(&params, "namp", "", "0");
                let dc = Self::get_param_owned(&params, "dc", "", "0");
                if params
                    .get("isnoisy")
                    .is_some_and(|enabled| enabled.eq_ignore_ascii_case("false") || enabled == "0")
                {
                    format!("DC {dc}")
                } else {
                    format!("DC {} TRNOISE({} {} {} {})", dc, na, nt, nalpha, namp)
                }
            }
            ComponentType::VoltageSourceSffm | ComponentType::CurrentSourceSffm => {
                // SFFM(VO VA FC MDI FS TD PHASEM PHASEC)
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &["vo", "va", "fc", "mdi", "fs", "td", "phasem", "phasec"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "SFFM")
                {
                    return literal;
                }
                let vo = Self::get_param_owned(&params, "vo", value, "0");
                let va = Self::get_param_owned(&params, "va", "", "1");
                let fc = Self::get_param_owned(&params, "fc", "", "1Meg");
                let mdi = Self::get_param_owned(&params, "mdi", "", "1");
                let fs = Self::get_param_owned(&params, "fs", "", "1k");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let phasem = Self::get_param_owned(&params, "phasem", "", "0");
                let phasec = Self::get_param_owned(&params, "phasec", "", "0");
                format!(
                    "SFFM({})",
                    Self::waveform_arguments(
                        &[vo, va, fc, mdi, fs],
                        &[(td, "0"), (phasem, "0"), (phasec, "0")]
                    )
                )
            }
            ComponentType::VoltageSourceAm | ComponentType::CurrentSourceAm => {
                // AM(VO VMO VMA FM FC TD PHASEM PHASEC)
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &["vo", "vmo", "vma", "fm", "fc", "td", "phasem", "phasec"],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "AM")
                {
                    return literal;
                }
                let vo = Self::get_param_owned(&params, "vo", value, "0");
                let vmo = Self::get_param_owned(&params, "vmo", "", "0");
                let vma = Self::get_param_owned(&params, "vma", "", "1");
                let fm = Self::get_param_owned(&params, "fm", "", "1k");
                let fc = Self::get_param_owned(&params, "fc", "", "1Meg");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let phasem = Self::get_param_owned(&params, "phasem", "", "0");
                let phasec = Self::get_param_owned(&params, "phasec", "", "0");
                format!(
                    "AM({})",
                    Self::waveform_arguments(
                        &[vo, vmo, vma, fm, fc],
                        &[(td, "0"), (phasem, "0"), (phasec, "0")]
                    )
                )
            }
            ComponentType::VoltageSourcePat | ComponentType::CurrentSourcePat => {
                // PAT(VHI VLO TD TR TF TSAMPLE DATA [R=n]). The engine rejects a
                // non-positive TR/TF/TSAMPLE and a DATA string without its
                // leading B, so both are normalized here rather than handed to
                // the parser as an error the user cannot trace back to a field.
                let params = crate::state::parse_params_string(&component.params);
                if !Self::has_any_source_parameter(
                    &params,
                    &[
                        "vhi",
                        "vlo",
                        "td",
                        "tr",
                        "tf",
                        "tsample",
                        "data",
                        "repeat_count",
                    ],
                ) && let Some(literal) = Self::legacy_waveform_literal(value, "PAT")
                {
                    return literal;
                }
                let vhi = Self::get_param_owned(&params, "vhi", value, "1");
                let vlo = Self::get_param_owned(&params, "vlo", "", "0");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let tr = Self::get_param_owned(&params, "tr", "", "1n");
                let tf = Self::get_param_owned(&params, "tf", "", "1n");
                let tsample = Self::get_param_owned(&params, "tsample", "", "1u");
                let data =
                    Self::pattern_bit_string(&Self::get_param_owned(&params, "data", "", "b0101"));
                let repeat = Self::get_param_owned(&params, "repeat_count", "", "0");
                let mut specification = format!("PAT({vhi} {vlo} {td} {tr} {tf} {tsample} {data}");
                if !matches!(repeat.trim(), "" | "0") {
                    specification.push_str(&format!(" R={}", repeat.trim()));
                }
                specification.push(')');
                specification
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

    fn has_any_source_parameter(params: &HashMap<String, String>, keys: &[&str]) -> bool {
        keys.iter().any(|key| params.contains_key(*key))
    }

    /// Join a waveform's mandatory arguments with the optional tail that
    /// follows them, dropping trailing entries still at their default.
    ///
    /// These tails are positional: emitting PHASEC means emitting TD and
    /// PHASEM ahead of it. Writing them unconditionally would work, but it
    /// would also append ` 0 0 0` to every pulse and FM source in every deck
    /// RSpice generates, so the tail is trimmed back to the last field that
    /// carries information.
    fn waveform_arguments(head: &[String], tail: &[(String, &str)]) -> String {
        let mut arguments = head.to_vec();
        let last_meaningful = tail
            .iter()
            .rposition(|(value, default)| value.trim() != *default);
        if let Some(last) = last_meaningful {
            arguments.extend(tail[..=last].iter().map(|(value, _)| value.clone()));
        }
        arguments.join(" ")
    }

    /// Normalize a PAT bit pattern to the `B<bits>` form the parser demands.
    ///
    /// Typing `0101` is the obvious mistake to make in a field labelled "bit
    /// pattern", and the parser's answer to it is a deck-level syntax error
    /// naming neither the device nor the field. Anything that is not a bit is
    /// left in place so a genuinely malformed pattern still fails loudly
    /// instead of being silently rewritten into a valid one.
    fn pattern_bit_string(data: &str) -> String {
        let trimmed = data.trim();
        let bits = trimmed.strip_prefix(['b', 'B']).unwrap_or(trimmed);
        if bits.is_empty() || !bits.chars().all(|bit| matches!(bit, '0' | '1')) {
            return trimmed.to_owned();
        }
        format!("b{bits}")
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
        params_map.remove("model_library");
        params_map.remove("model_corner");
        let params_without_model = crate::state::format_params_string(&params_map);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pwl_delay_and_repeat_are_present_in_the_canonical_source_specification() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);
        let mut source = Component::new(1, ComponentType::VoltageSourcePwl, Point::origin())
            .with_name_value("V1", "0 0 1u 1");
        source.params = "td=2u repeat=true".to_owned();

        assert_eq!(
            generator.format_source_value(&source),
            "PWL(0 0 1u 1) TD=2u R=0"
        );
    }

    #[test]
    fn disabled_transient_noise_source_emits_only_its_dc_bias() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);
        let mut source = Component::new(1, ComponentType::CurrentSourceNoise, Point::origin())
            .with_name_value("I1", "1n");
        source.params = "dc=2m isnoisy=false nt=1u".to_owned();

        assert_eq!(generator.format_source_value(&source), "DC 2m");
    }

    #[test]
    fn ac_phase_is_positional_and_editor_catalog_metadata_is_not_netlisted() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);
        let mut source = Component::new(1, ComponentType::VoltageSourceAc, Point::origin())
            .with_name_value("V1", "2");
        source.params = "acphase=90 model_library=vendor model_corner=fast".to_owned();

        assert_eq!(generator.format_source_value(&source), "DC 0 AC 2 90");
        assert_eq!(generator.format_params(&source.params), " acphase=90");
        assert!(!generator.format_params(&source.params).contains("model_"));
    }
}

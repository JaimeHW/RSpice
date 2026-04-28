use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn normalize_variable_name(name: &str) -> String {
        name.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase()
    }

    pub(in crate::testing::ngspice_runner) fn default_transient_max_step(
        tstep: f64,
        tstop: f64,
        tstart: f64,
    ) -> f64 {
        let analysis_window = (tstop - tstart).max(0.0);
        let fallback_window = if analysis_window > 0.0 {
            analysis_window
        } else {
            tstop.abs().max(tstep.abs())
        };
        let window_limit = fallback_window / 50.0;

        if tstep > 0.0 {
            tstep.min(window_limit)
        } else {
            window_limit
        }
    }

    pub(in crate::testing::ngspice_runner) fn evaluate_ac_complex_value(
        func: &str,
        value: num_complex::Complex64,
        abs_tol: f64,
    ) -> f64 {
        match func {
            "mag" | "vm" | "v" | "i" => value.norm(),
            "vr" | "ir" => value.re,
            "vi" | "ii" => value.im,
            "ph" => value.arg(),
            "vp" | "ip" => value.arg().to_degrees(),
            "db" | "vdb" => {
                let mag = value.norm().max(abs_tol);
                20.0 * mag.log10()
            }
            _ => value.norm(),
        }
    }

    pub(in crate::testing::ngspice_runner) fn evaluate_reference_complex_output(
        var_name: &str,
        value: num_complex::Complex64,
        abs_tol: f64,
    ) -> f64 {
        let normalized = Self::normalize_variable_name(var_name);
        match Self::parse_ac_probe(&normalized) {
            Some(AcProbe::Voltage { func, .. }) | Some(AcProbe::Current { func, .. }) => {
                Self::evaluate_ac_complex_value(func, value, abs_tol)
            }
            None => value.re,
        }
    }

    pub(in crate::testing::ngspice_runner) fn parse_voltage_probe(
        var: &str,
    ) -> Option<(String, Option<String>)> {
        let normalized = Self::normalize_variable_name(var);
        if !(normalized.starts_with("v(") && normalized.ends_with(')')) {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if let Some((a, b)) = inner.split_once(',') {
            Some((a.to_string(), Some(b.to_string())))
        } else {
            Some((inner.to_string(), None))
        }
    }

    pub(in crate::testing::ngspice_runner) fn parse_current_probe(var: &str) -> Option<String> {
        let normalized = Self::normalize_variable_name(var);
        if normalized.starts_with("i(") && normalized.ends_with(')') {
            let inner = &normalized[2..normalized.len() - 1];
            return if inner.is_empty() {
                None
            } else {
                Some(inner.to_string())
            };
        }
        normalized
            .strip_suffix("#branch")
            .and_then(|name| (!name.is_empty()).then(|| name.to_string()))
    }

    pub(in crate::testing::ngspice_runner) fn branch_probe_names_from_netlist(
        netlist: &Netlist,
    ) -> Vec<String> {
        netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                crate::netlist::ElementKind::Inductor { .. }
                | crate::netlist::ElementKind::JilesAthertonInductor { .. }
                | crate::netlist::ElementKind::VoltageSource(_)
                | crate::netlist::ElementKind::Ccvs { .. }
                | crate::netlist::ElementKind::BehavioralVoltage { .. } => {
                    Some(element.name.clone())
                }
                _ => None,
            })
            .collect()
    }

    pub(in crate::testing::ngspice_runner) fn parse_ac_probe(var: &str) -> Option<AcProbe> {
        let normalized = Self::normalize_variable_name(var);
        for func in [
            "vdb", "db", "vm", "mag", "vr", "ir", "vi", "ii", "vp", "ip", "ph",
        ] {
            let prefix = format!("{func}(");
            if normalized.starts_with(&prefix) && normalized.ends_with(')') {
                let inner = &normalized[prefix.len()..normalized.len() - 1];
                if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(inner) {
                    return Some(AcProbe::Voltage {
                        func,
                        node_pos,
                        node_neg,
                    });
                }
                if let Some(branch) = Self::parse_current_probe(inner) {
                    return Some(AcProbe::Current { func, branch });
                }
                return None;
            }
        }

        if let Some((node_pos, node_neg)) = Self::parse_voltage_probe(&normalized) {
            return Some(AcProbe::Voltage {
                func: "v",
                node_pos,
                node_neg,
            });
        }
        if let Some(branch) = Self::parse_current_probe(&normalized) {
            return Some(AcProbe::Current { func: "i", branch });
        }

        None
    }

    pub(in crate::testing::ngspice_runner) fn combine_reference_tables(
        axis_name: String,
        tables: Vec<ReferenceTable>,
    ) -> ReferenceTable {
        let mut combined = ReferenceTable {
            x_name: axis_name,
            variables: BTreeMap::new(),
        };

        for table in tables {
            for (name, mut series) in table.variables {
                if let Some(existing) = combined.variables.get_mut(&name) {
                    existing.x.append(&mut series.x);
                    existing.y.append(&mut series.y);
                } else {
                    combined.variables.insert(name, series);
                }
            }
        }

        combined
    }

    pub(in crate::testing::ngspice_runner) fn resolve_node_index(
        node_to_idx: &HashMap<String, usize>,
        node: &str,
    ) -> Option<usize> {
        if let Some(idx) = node_to_idx.get(&node.to_ascii_lowercase()) {
            return Some(*idx);
        }
        node.parse::<usize>().ok()
    }

    pub(in crate::testing::ngspice_runner) fn reference_node_exists(
        node_to_idx: &HashMap<String, usize>,
        node: &str,
    ) -> bool {
        node_to_idx.contains_key(&node.to_ascii_lowercase())
    }
}

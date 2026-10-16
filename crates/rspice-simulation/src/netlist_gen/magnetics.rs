//! Transformer and coupled inductor cards.
//!
//! Writes a transformer as its two winding inductors and their coupling, and
//! the mutual-inductance cards the drawing's couplings ask for. Which cards
//! those are and what they are named is decided in
//! `rspice_design::schematic::component`, which the design check reads too.

use super::*;
use rspice_design::schematic::component::{TransformerCards, coupling_cards};

impl<'a> NetlistGenerator<'a> {
    pub(super) fn collect_transformer_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        for component in self.schematic.components.clone() {
            if component.kind != ComponentType::Transformer {
                continue;
            }

            if let Some(transformer_lines) = self.transformer_instance_lines(&component) {
                lines.extend(transformer_lines);
            }
        }
        lines
    }

    fn transformer_instance_lines(&mut self, component: &Component) -> Option<Vec<String>> {
        let terminals = component.terminal_positions();
        let node_names: Vec<String> = terminals
            .iter()
            .map(|(_, pos)| self.get_node_name(*pos))
            .collect();
        if node_names.len() != 4 {
            self.errors.push(format!(
                "Transformer '{}' must expose exactly four winding terminals",
                component.spice_instance_name()
            ));
            return None;
        }

        let primary_inductance = component.value.trim();
        if primary_inductance.is_empty() {
            self.errors.push(format!(
                "Transformer '{}' is missing a primary inductance",
                component.spice_instance_name()
            ));
            return None;
        }
        if let Ok(value) = primary_inductance.parse::<f64>()
            && (!value.is_finite() || value <= 0.0)
        {
            self.errors.push(format!(
                "Transformer '{}' has invalid primary inductance {}",
                component.spice_instance_name(),
                primary_inductance
            ));
            return None;
        }

        let params = rspice_design::parameters::parse_params_string(&component.params);
        let ratio = params
            .get("turns_ratio")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .unwrap_or("1");
        if let Ok(value) = ratio.parse::<f64>()
            && (!value.is_finite() || value <= 0.0)
        {
            self.errors.push(format!(
                "Transformer '{}' has invalid turns ratio {}",
                component.spice_instance_name(),
                ratio
            ));
            return None;
        }

        let explicit_secondary = params
            .get("ls")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        if let Some(value) = explicit_secondary {
            if let Ok(parsed) = value.parse::<f64>()
                && (!parsed.is_finite() || parsed <= 0.0)
            {
                self.errors.push(format!(
                    "Transformer '{}' has invalid secondary inductance {}",
                    component.spice_instance_name(),
                    value
                ));
                return None;
            }
            if ratio != "1" {
                self.warnings.push(format!(
                    "Transformer '{}' specifies both turns_ratio and secondary inductance; using explicit secondary inductance",
                    component.spice_instance_name()
                ));
            }
        }

        let coupling = params
            .get("k")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .unwrap_or("0.999");
        if let Ok(value) = coupling.parse::<f64>()
            && (!value.is_finite() || !(-1.0..=1.0).contains(&value))
        {
            self.errors.push(format!(
                "Transformer '{}' has invalid coupling factor {} (expected -1 <= k <= 1)",
                component.spice_instance_name(),
                coupling
            ));
            return None;
        }

        let secondary_inductance = explicit_secondary
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| Self::derive_secondary_inductance(primary_inductance, ratio));

        let mut primary_params = HashMap::new();
        if let Some(value) = params
            .get("rp")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            primary_params.insert("r".to_string(), value.to_string());
        }
        if let Some(value) = params
            .get("icp")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            primary_params.insert("ic".to_string(), value.to_string());
        }

        let mut secondary_params = HashMap::new();
        if let Some(value) = params
            .get("rs")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            secondary_params.insert("r".to_string(), value.to_string());
        }
        if let Some(value) = params
            .get("ics")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            secondary_params.insert("ic".to_string(), value.to_string());
        }

        let TransformerCards {
            primary: primary_name,
            secondary: secondary_name,
            coupling: coupling_name,
        } = component.transformer_cards()?;

        let primary_nodes = self.format_nodes(&node_names[0..2], 2);
        let secondary_nodes = self.format_nodes(&node_names[2..4], 2);
        let primary_suffix = rspice_design::parameters::format_params_string(&primary_params);
        let secondary_suffix = rspice_design::parameters::format_params_string(&secondary_params);
        let primary_line = if primary_suffix.is_empty() {
            format!("{} {} {}", primary_name, primary_nodes, primary_inductance)
        } else {
            format!(
                "{} {} {} {}",
                primary_name, primary_nodes, primary_inductance, primary_suffix
            )
        };
        let secondary_line = if secondary_suffix.is_empty() {
            format!(
                "{} {} {}",
                secondary_name, secondary_nodes, secondary_inductance
            )
        } else {
            format!(
                "{} {} {} {}",
                secondary_name, secondary_nodes, secondary_inductance, secondary_suffix
            )
        };
        let coupling_line = format!(
            "{} {} {} {}",
            coupling_name, primary_name, secondary_name, coupling
        );

        Some(vec![primary_line, secondary_line, coupling_line])
    }

    fn derive_secondary_inductance(primary_inductance: &str, turns_ratio: &str) -> String {
        format!(
            "(({})*(({})*({})))",
            primary_inductance, turns_ratio, turns_ratio
        )
    }

    /// The mutual-inductance cards the cellview's couplings ask for, as
    /// [`coupling_cards`] decides them; each coupling it refuses is an error.
    pub(super) fn collect_coupling_lines(&mut self) -> Vec<String> {
        let couplings = coupling_cards(&self.schematic.components);
        self.errors
            .extend(couplings.refusals.iter().map(ToString::to_string));
        couplings
            .cards
            .into_iter()
            .map(|card| {
                format!(
                    "{} {} {}",
                    card.name,
                    card.windings.join(" "),
                    card.coefficient
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;
    use rspice_core::{Engine, Netlist};
    use rspice_design::schematic::net_label::NetLabel;

    fn circuit(kind: ComponentType, coefficient: &str, reversed: bool) -> Schematic {
        let mut schematic = Schematic::default();
        let mut source = Component::new(1, ComponentType::VoltageSource, Point::origin())
            .with_name_value("V1", "0.5");
        source.params = "ac=1".to_owned();
        let resistor = Component::new(2, ComponentType::Resistor, Point::new(200, 0))
            .with_name_value("R1", "50");
        let mut add = |component: Component, nodes: &[&str]| {
            let document = schematic.document_mut_for_test();
            for ((_, point), node) in component.terminal_positions().iter().zip(nodes) {
                document.net_labels.push(NetLabel::new(
                    document.net_labels.len() as u64 + 1,
                    *point,
                    *node,
                ));
            }
            document.components.push(component);
        };
        add(source, &["in", "0"]);
        add(resistor, &["in", "p"]);
        let secondary = if reversed { ["0", "s"] } else { ["s", "0"] };
        if kind == ComponentType::Transformer {
            let mut transformer =
                Component::new(3, kind, Point::new(400, 0)).with_name_value("T1", "10m");
            transformer.params = format!("ls=40m k={coefficient}");
            add(transformer, &["p", "0", secondary[0], secondary[1]]);
        } else {
            let mut primary = Component::new(3, ComponentType::Inductor, Point::new(400, 0))
                .with_name_value("L1", "10m");
            if kind == ComponentType::Inductor {
                primary.params = format!("coupled_to=L2 coupling_factor={coefficient}");
            }
            add(primary, &["p", "0"]);
            add(
                Component::new(4, ComponentType::Inductor, Point::new(600, 0))
                    .with_name_value("L2", "40m"),
                &secondary,
            );
            if kind == ComponentType::CoupledInductor {
                let mut coupling =
                    Component::new(5, kind, Point::new(800, 0)).with_name_value("K1", coefficient);
                coupling.params = "inductors=\"L1 L2\"".to_owned();
                add(coupling, &[]);
            }
        }
        schematic
    }

    fn generated(schematic: &Schematic) -> super::super::NetlistResult {
        let buffers: HashMap<String, SchematicDocument> = HashMap::new();
        let hierarchy = HierarchySource::from_buffers(&buffers);
        generate_netlist_hierarchical(
            schematic,
            &[],
            &hierarchy,
            &crate::netlist_gen::NetlistSourceData::new(&crate::netlist_gen::FixtureSourceFiles),
        )
    }

    #[test]
    fn signed_schematic_coupling_preserves_observable_secondary_polarity() {
        for kind in [
            ComponentType::CoupledInductor,
            ComponentType::Inductor,
            ComponentType::Transformer,
        ] {
            for coefficient in [-1.0, -0.75, 0.0, 0.75, 1.0] {
                for reversed in [false, true] {
                    let schematic = circuit(kind, &coefficient.to_string(), reversed);
                    let generated = generated(&schematic);
                    assert!(
                        generated.errors.is_empty(),
                        "{kind:?}, {coefficient}: {:?}",
                        generated.errors
                    );
                    let deck = Netlist::parse(&generated.netlist).unwrap();
                    let engine = Engine::default();
                    for point in engine.run_ac(&deck, &[100.0, 1000.0, 10000.0]).unwrap() {
                        let omega = std::f64::consts::TAU * point.frequency;
                        let orientation = if reversed { -1.0 } else { 1.0 };
                        let mutual = coefficient * orientation * 0.02;
                        let expected = Complex64::new(0.0, omega * mutual)
                            / Complex64::new(50.0, omega * 0.01);
                        let secondary = point
                            .node_names
                            .iter()
                            .position(|node| node.eq_ignore_ascii_case("s"))
                            .unwrap_or_else(|| {
                                panic!(
                                    "Secondary node missing from {:?}:\n{}",
                                    point.node_names, generated.netlist
                                )
                            });
                        assert!(
                            (point.voltages[secondary] - expected).norm() < 2e-12,
                            "{kind:?}, k={coefficient}, reversed={reversed}: {} vs {expected}\n{}",
                            point.voltages[secondary],
                            generated.netlist
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn invalid_schematic_coupling_is_refused_without_clamping() {
        for kind in [
            ComponentType::CoupledInductor,
            ComponentType::Inductor,
            ComponentType::Transformer,
        ] {
            for coefficient in ["-1.01", "1.01", "NaN", "inf", "-inf"] {
                let schematic = circuit(kind, coefficient, false);
                let before = schematic.document().components.clone();
                let result = generated(&schematic);
                assert!(
                    result
                        .errors
                        .iter()
                        .any(|error| error.contains("invalid coupling")),
                    "{kind:?}, k={coefficient}: {:?}",
                    result.errors
                );
                assert_eq!(schematic.document().components, before);
            }
        }
    }

    #[test]
    fn inductor_coupling_property_accepts_signed_values_and_keeps_absence_distinct() {
        use rspice_design::properties::PropertyCatalog;
        use rspice_design::properties::PropertyValue;

        let registry = PropertyCatalog::new();
        let definition = registry
            .get(ComponentType::Inductor)
            .unwrap()
            .get("coupling_factor")
            .unwrap();
        for value in [-1.0, -0.75, -0.0, 0.0, 0.75, 1.0] {
            definition.validate(&PropertyValue::number(value)).unwrap();
        }
        for value in [-1.01, 1.01, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(definition.validate(&PropertyValue::number(value)).is_err());
        }

        for params in ["", "coupling_factor=0", "coupling_factor=-0"] {
            let mut schematic = circuit(ComponentType::Inductor, "0", false);
            schematic.document_mut_for_test().components[2].params = params.to_owned();
            let result = generated(&schematic);
            assert!(result.errors.is_empty(), "{:?}", result.errors);
            let deck = Netlist::parse(&result.netlist).unwrap();
            assert!(!deck.elements.iter().any(|element| matches!(
                element.kind,
                rspice_core::netlist::ElementKind::Coupling { .. }
            )));
        }
        let mut schematic = circuit(ComponentType::Inductor, "-0.75", false);
        schematic.document_mut_for_test().components[2].params = "coupling_factor=-0.75".to_owned();
        assert!(
            generated(&schematic)
                .errors
                .iter()
                .any(|error| error.contains("no target winding"))
        );
        schematic.document_mut_for_test().components[2].params = "coupled_to=L2".to_owned();
        assert!(
            generated(&schematic)
                .errors
                .iter()
                .any(|error| error.contains("missing a coupling factor"))
        );
    }

    #[test]
    fn reciprocal_coupling_definitions_must_agree_in_polarity() {
        let mut schematic = circuit(ComponentType::Inductor, "-0.75", false);
        schematic.document_mut_for_test().components[3].params =
            "coupled_to=L1 coupling_factor=-0.75".to_owned();
        let result = generated(&schematic);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let deck = Netlist::parse(&result.netlist).unwrap();
        assert_eq!(
            deck.elements
                .iter()
                .filter(|element| matches!(
                    element.kind,
                    rspice_core::netlist::ElementKind::Coupling { .. }
                ))
                .count(),
            1
        );

        schematic.document_mut_for_test().components[3].params =
            "coupled_to=L1 coupling_factor=0.75".to_owned();
        assert!(
            generated(&schematic)
                .errors
                .iter()
                .any(|error| error.contains("Conflicting coupling definitions"))
        );
    }
}

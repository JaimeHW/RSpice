//! Resistor, capacitor and inductor cards.
//!
//! A part with a tolerance writes its value as a deck parameter with a
//! `.TOLERANCE` card, which every analysis reads at nominal and a Monte Carlo
//! draws from the part's own band. A resistor's `lser` becomes an inductor in
//! series with its `+` terminal, outside the resistor and the `cpar` the
//! engine writes across it.

use super::*;
use rspice_design::schematic::tolerance::{
    DISTRIBUTION_KEY, TOLERANCE_KEY, ToleranceDistribution, part_tolerance,
};

impl NetlistGenerator<'_> {
    pub(super) fn passive_line(
        &mut self,
        component: &Component,
        node_names: &[String],
        instance_name: &str,
    ) -> String {
        let mut params = rspice_design::parameters::parse_params_string(&component.params);
        if component.kind == ComponentType::Inductor {
            params.remove("coupled_to");
            params.remove("coupling_factor");
        }
        let tolerance = part_tolerance(&params).unwrap_or_else(|error| {
            self.errors.push(format!(
                "{} '{}': {error}",
                component.kind.display_name(),
                component.name
            ));
            None
        });
        params.remove(TOLERANCE_KEY);
        params.remove(DISTRIBUTION_KEY);
        params.remove("lser");

        let mut value = self.format_value(component.value.trim());
        // A tolerance is a part's own statistic, so it is written where it
        // belongs to one part: the root deck. A cell's parameter is shared
        // by every instance of the cell, which would draw them all as one.
        if let Some(tolerance) = tolerance.filter(|tolerance| tolerance.percent > 0.0) {
            if self.hierarchy_path.is_root() {
                let parameter = format!("__rspice_tol_{}", component.id);
                let distribution = match tolerance.distribution {
                    ToleranceDistribution::Gaussian => "GAUSS SIGMA=3",
                    ToleranceDistribution::Uniform => "UNIFORM",
                };
                self.lines.push(format!(".param {parameter}={value}"));
                self.lines.push(format!(
                    ".TOLERANCE {parameter} {} {distribution}",
                    tolerance.percent / 100.0
                ));
                value = format!("{{{parameter}}}");
            } else {
                self.warnings.push(format!(
                    "{} '{}' sits inside a cell, where every instance would share one draw, \
                     so a Monte Carlo holds its {}% tolerance at nominal",
                    component.kind.display_name(),
                    component.name,
                    tolerance.percent
                ));
            }
        }

        // Parameters the deck card takes are written as authored; only a
        // part whose editor-side keys were peeled off is rewritten.
        let peeled = [
            TOLERANCE_KEY,
            DISTRIBUTION_KEY,
            "lser",
            "coupled_to",
            "coupling_factor",
        ];
        let authored = rspice_design::parameters::parse_params_string(&component.params);
        let params_text = if peeled.iter().any(|key| authored.contains_key(*key)) {
            self.format_params(&rspice_design::parameters::format_params_string(&params))
        } else {
            self.format_params(&component.params)
        };
        let nodes = self.format_nodes(node_names, 2);
        match component.resistor_series_inductance() {
            Some(inductor) => {
                let internal = format!("__rspice_part_{}_lser", component.id);
                let negative = nodes.split_once(' ').map_or("0", |(_, negative)| negative);
                let positive = nodes.split_once(' ').map_or(nodes.as_str(), |(p, _)| p);
                self.lines.push(format!(
                    "{} {positive} {internal} {}",
                    inductor.name, inductor.value
                ));
                format!("{instance_name} {internal} {negative} {value}{params_text}")
            }
            None => format!("{instance_name} {nodes} {value}{params_text}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deck(parts: &[(ComponentType, &str, &str, &str)]) -> String {
        let mut state = Schematic::default();
        for (index, (kind, name, value, params)) in parts.iter().enumerate() {
            let mut component =
                Component::new(index as u64 + 1, *kind, Point::new(0, index as i32 * 200));
            component.name = (*name).to_owned();
            component.value = (*value).to_owned();
            component.params = (*params).to_owned();
            state.document_mut_for_test().components.push(component);
        }
        NetlistGenerator::new(
            &state,
            crate::netlist_gen::NetlistSourceData::new(&crate::netlist_gen::FixtureSourceFiles),
        )
        .generate()
    }

    fn card<'a>(deck: &'a str, head: &str) -> &'a str {
        deck.lines()
            .find(|line| line.starts_with(head))
            .unwrap_or_else(|| panic!("no {head} card in\n{deck}"))
    }

    #[test]
    fn a_toleranced_part_writes_a_nominal_parameter_and_its_tolerance_card() {
        let deck = deck(&[
            (ComponentType::Resistor, "R1", "1k", "tol=5% tc1=0.001"),
            (
                ComponentType::Capacitor,
                "C1",
                "159n",
                "tol=10 tol_dist=uniform",
            ),
            (ComponentType::Inductor, "L1", "1u", "tol=0"),
        ]);
        assert!(deck.contains("\n.param __rspice_tol_1=1k\n"), "{deck}");
        assert!(deck.contains("\n.TOLERANCE __rspice_tol_1 0.05 GAUSS SIGMA=3\n"));
        assert!(card(&deck, "R1 ").ends_with(" {__rspice_tol_1} tc1=0.001"));
        assert!(deck.contains("\n.TOLERANCE __rspice_tol_2 0.1 UNIFORM\n"));
        assert!(card(&deck, "C1 ").ends_with(" {__rspice_tol_2}"));
        // A zero tolerance is a nominal part and writes no statistic.
        assert!(card(&deck, "L1 ").ends_with(" 1u"));
        assert!(!deck.contains("__rspice_tol_3"));
        let parsed = rspice_core::Netlist::parse(&deck).unwrap();
        let stated = parsed
            .options
            .part_tolerances
            .iter()
            .map(|tolerance| (tolerance.parameter.as_str(), tolerance.relative))
            .collect::<Vec<_>>();
        assert_eq!(stated, [("__RSPICE_TOL_1", 0.05), ("__RSPICE_TOL_2", 0.1)]);
    }

    #[test]
    fn a_resistor_series_inductance_sits_outside_its_parallel_capacitance() {
        let deck = deck(&[(ComponentType::Resistor, "R1", "50", "cpar=0.2p lser=1.5n")]);
        let inductor = card(&deck, "L__RSPICE_PART_1_LSER ");
        let resistor = card(&deck, "R1 ");
        let fields = |line: &str| {
            line.split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        };
        let (inductor, resistor) = (fields(inductor), fields(resistor));
        assert_eq!(inductor[2], "__rspice_part_1_lser");
        assert_eq!(inductor[3], "1.5n");
        assert_eq!(resistor[1], "__rspice_part_1_lser");
        assert_eq!(resistor[3..], ["50", "cpar=0.2p"]);
        // The engine writes the capacitance across the resistor alone.
        let parsed = rspice_core::Netlist::parse(&deck).unwrap();
        let cpar = parsed
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("CR1#PAR"))
            .expect("the engine expands cpar");
        assert!(cpar.nodes[0].eq_ignore_ascii_case("__rspice_part_1_lser"));
    }
}

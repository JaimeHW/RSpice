//! The S-parameter sheet's share of the strip stack: which of a network
//! run's retained columns it draws at first, and the Plot menu of the
//! functions ADS and ViVA's Direct Plot read from them.
//!
//! Every function in the menu is a calculator expression over the run's
//! own coefficients (`kf(S11,S12,S21,S22)`), added to the analysis' strip
//! as any expression is, so it plots against frequency, reads at the
//! cursors, and is kept by the run's view.

use egui::Ui;
use rspice_results::network_matrix::trace_identity;
use rspice_ui_kit::widgets::Button;

/// Whether a retained column is one of a network's scattering coefficients
/// (`|S21|`, `phase(S21)`), rather than a noise column of the same run.
pub fn is_coefficient(name: &str) -> bool {
    let name = name.trim();
    let name = name
        .strip_prefix("phase(")
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap_or(name);
    trace_identity(name).is_some()
}

/// What the run offers the Plot menu, read from its column names and
/// metadata without a sample.
pub struct NetworkOffer<'a> {
    /// The run's coefficients: output port, input port (both from one), and
    /// the name the calculator reads the coefficient by.
    pub coefficients: &'a [(usize, usize, String)],
    /// Each port's reference resistance in ohms.
    pub references: &'a [f64],
    /// Whether the run kept its noise figures (F, Fmin and Rn).
    pub noise: bool,
}

/// One function the menu offers.
#[derive(Debug, Clone, PartialEq)]
pub struct PlotEntry {
    pub label: String,
    pub expression: String,
    pub hint: String,
}

/// A titled run of entries; a nested group opens as a submenu.
#[derive(Debug, Clone, PartialEq)]
pub struct PlotGroup {
    pub title: &'static str,
    pub nested: bool,
    pub entries: Vec<PlotEntry>,
}

fn entry(label: impl Into<String>, expression: String, hint: impl Into<String>) -> PlotEntry {
    PlotEntry {
        label: label.into(),
        expression,
        hint: hint.into(),
    }
}

/// The menu's functions for a network: group delay and VSWR for any port
/// count; stability factors, gains, and Z, Y and H parameters for a
/// two-port; noise figures where the run kept them.
pub fn plot_groups(offer: &NetworkOffer<'_>) -> Vec<PlotGroup> {
    let name = |output: usize, input: usize| {
        offer
            .coefficients
            .iter()
            .find(|(row, column, _)| (*row, *column) == (output, input))
            .map(|(_, _, name)| name.as_str())
    };
    let mut groups = Vec::new();
    let two_port = match (name(1, 1), name(1, 2), name(2, 1), name(2, 2)) {
        (Some(s11), Some(s12), Some(s21), Some(s22)) if offer.references.len() == 2 => {
            Some(format!("{s11},{s12},{s21},{s22}"))
        }
        _ => None,
    };
    if let Some(s) = &two_port {
        groups.push(PlotGroup {
            title: "Stability",
            nested: false,
            entries: vec![
                entry("K", format!("kf({s})"), "Rollett stability factor"),
                entry(
                    "B1",
                    format!("b1f({s})"),
                    "Positive, with K above one, when unconditionally stable",
                ),
                entry(
                    "μ",
                    format!("mu({s})"),
                    "Distance to the nearest unstable load; above one when stable",
                ),
                entry(
                    "μ′",
                    format!("mu_prime({s})"),
                    "Distance to the nearest unstable source",
                ),
            ],
        });
        groups.push(PlotGroup {
            title: "Gain",
            nested: false,
            entries: vec![
                entry(
                    "GT",
                    format!("dB10(gt({s}))"),
                    "Transducer gain between the port terminations",
                ),
                entry(
                    "GA",
                    format!("dB10(ga({s}))"),
                    "Available gain from the port 1 termination",
                ),
                entry(
                    "GP",
                    format!("dB10(gp({s}))"),
                    "Operating power gain into the port 2 termination",
                ),
                entry(
                    "MAG / MSG",
                    format!("dB10(gmax({s}))"),
                    "Maximum available gain where stable, otherwise maximum stable gain",
                ),
            ],
        });
    }
    let ports = offer
        .coefficients
        .iter()
        .map(|(row, column, _)| (*row).max(*column))
        .max()
        .unwrap_or(0);
    let reflections = (1..=ports)
        .filter_map(|port| {
            name(port, port).map(|s| {
                entry(
                    format!("VSWR at port {port}"),
                    format!("vswr({s})"),
                    "Voltage standing-wave ratio of the port's reflection",
                )
            })
        })
        .collect::<Vec<_>>();
    if !reflections.is_empty() {
        groups.push(PlotGroup {
            title: "Reflection",
            nested: false,
            entries: reflections,
        });
    }
    let delays = offer
        .coefficients
        .iter()
        .filter(|(row, column, _)| row != column)
        .map(|(_, _, s)| {
            entry(
                format!("Group delay of {s}"),
                format!("groupDelay({s})"),
                "−dφ/dω of the transmission, in seconds",
            )
        })
        .collect::<Vec<_>>();
    if !delays.is_empty() {
        groups.push(PlotGroup {
            title: "Group delay",
            nested: true,
            entries: delays,
        });
    }
    if let Some(s) = &two_port {
        let references = match offer.references {
            [first, second] if *first == 50.0 && *second == 50.0 => String::new(),
            [first, second] => format!(",{first},{second}"),
            _ => String::new(),
        };
        for (title, function, letter, what) in [
            ("Z-parameters", "zp", "Z", "Open-circuit impedance"),
            ("Y-parameters", "yp", "Y", "Short-circuit admittance"),
            ("H-parameters", "hp", "H", "Hybrid parameter"),
        ] {
            groups.push(PlotGroup {
                title,
                nested: true,
                entries: [(1, 1), (1, 2), (2, 1), (2, 2)]
                    .into_iter()
                    .map(|(row, column)| {
                        entry(
                            format!("{letter}{row}{column}"),
                            format!("{function}({row},{column},{s}{references})"),
                            format!("{what} {letter}{row}{column} on the port references"),
                        )
                    })
                    .collect(),
            });
        }
    }
    if offer.noise {
        groups.push(PlotGroup {
            title: "Noise",
            nested: false,
            entries: vec![
                entry(
                    "NF",
                    "dB10(F)".to_owned(),
                    "Noise figure with the port terminations as the source",
                ),
                entry(
                    "NFmin",
                    "dB10(Fmin)".to_owned(),
                    "Minimum noise figure, at the optimum source",
                ),
                entry(
                    "Rn",
                    "Rn".to_owned(),
                    "Equivalent noise resistance: how fast the noise figure rises away from the optimum",
                ),
            ],
        });
    }
    groups
}

/// The sheet's Plot menu. `shown` says whether an expression is on the
/// strip and drawn; the result is the expression the reader toggled.
pub fn plot_menu(
    ui: &mut Ui,
    groups: &[PlotGroup],
    shown: impl Fn(&str) -> bool,
) -> Option<String> {
    let button = Button::new("Plot")
        .menu()
        .accessible_label("Plot a function of the network")
        .show(ui);
    let mut toggled = None;
    egui::Popup::menu(&button)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            let mut rows = |ui: &mut Ui, entries: &[PlotEntry]| {
                for entry in entries {
                    let mut on = shown(&entry.expression);
                    if ui
                        .checkbox(&mut on, entry.label.as_str())
                        .on_hover_text(format!("{}\n{}", entry.hint, entry.expression))
                        .clicked()
                    {
                        toggled = Some(entry.expression.clone());
                    }
                }
            };
            for (index, group) in groups.iter().enumerate() {
                if group.nested {
                    ui.menu_button(group.title, |ui| rows(ui, &group.entries));
                    continue;
                }
                if index > 0 {
                    ui.separator();
                }
                crate::chrome::bars::bar_label(ui, group.title);
                rows(ui, &group.entries);
            }
        });
    toggled
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coefficients(ports: usize) -> Vec<(usize, usize, String)> {
        (1..=ports)
            .flat_map(|row| (1..=ports).map(move |column| (row, column, format!("S{row}{column}"))))
            .collect()
    }

    fn expressions(groups: &[PlotGroup]) -> Vec<&str> {
        groups
            .iter()
            .flat_map(|group| &group.entries)
            .map(|entry| entry.expression.as_str())
            .collect()
    }

    #[test]
    fn coefficients_are_told_from_the_runs_other_columns() {
        for name in ["|S21|", "phase(S21)", "S11", "|Sdd21|"] {
            assert!(is_coefficient(name), "{name}");
        }
        for name in ["|Sopt|", "Rn", "F", "Fmin", "|CY(1,1)|", "phase(Sopt)"] {
            assert!(!is_coefficient(name), "{name}");
        }
    }

    /// A two-port on 50 Ω offers the ViVA Direct Plot list over its own
    /// coefficients; references other than 50 Ω are spelled in the call.
    #[test]
    fn a_two_port_offers_stability_gains_and_parameter_forms() {
        let coefficients = coefficients(2);
        let groups = plot_groups(&NetworkOffer {
            coefficients: &coefficients,
            references: &[50.0, 50.0],
            noise: true,
        });
        let offered = expressions(&groups);
        for expected in [
            "kf(S11,S12,S21,S22)",
            "mu(S11,S12,S21,S22)",
            "dB10(gt(S11,S12,S21,S22))",
            "dB10(gmax(S11,S12,S21,S22))",
            "vswr(S22)",
            "groupDelay(S21)",
            "zp(2,1,S11,S12,S21,S22)",
            "hp(1,1,S11,S12,S21,S22)",
            "dB10(Fmin)",
            "Rn",
        ] {
            assert!(offered.contains(&expected), "{expected} in {offered:?}");
        }
        let unequal = plot_groups(&NetworkOffer {
            coefficients: &coefficients,
            references: &[50.0, 75.0],
            noise: false,
        });
        let offered = expressions(&unequal);
        assert!(offered.contains(&"yp(1,2,S11,S12,S21,S22,50,75)"));
        assert!(!offered.contains(&"Rn"));
    }

    /// A three-port has no two-port figures: only its reflections and
    /// transmissions are offered.
    #[test]
    fn a_three_port_offers_only_what_any_network_has() {
        let coefficients = coefficients(3);
        let groups = plot_groups(&NetworkOffer {
            coefficients: &coefficients,
            references: &[50.0; 3],
            noise: false,
        });
        let titles = groups.iter().map(|group| group.title).collect::<Vec<_>>();
        assert_eq!(titles, ["Reflection", "Group delay"]);
        assert_eq!(groups[0].entries.len(), 3);
        assert_eq!(groups[1].entries.len(), 6);
    }
}

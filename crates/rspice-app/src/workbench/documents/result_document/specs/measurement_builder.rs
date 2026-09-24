//! Guided construction of native measurement cards; the card remains the saved authority.

use egui::Ui;

const OPERATIONS: &[(&str, &str)] = &[
    ("AVG", "Average"),
    ("RMS", "RMS"),
    ("MIN", "Minimum"),
    ("MAX", "Maximum"),
    ("PP", "Peak to peak"),
    ("INTEG", "Integral"),
    ("MIN_AT", "Minimum location"),
    ("MAX_AT", "Maximum location"),
    ("FIND", "Value at a point or event"),
    ("DERIV", "Derivative"),
    ("WHEN", "Crossing location"),
    ("TRIG", "Trigger to target delay"),
    ("PARAM", "Expression using measurements"),
    ("ERROR", "Compare with reference table"),
];

#[derive(Debug, Clone)]
struct EventDraft {
    at: bool,
    signal: String,
    value: String,
    edge: &'static str,
    occurrence: String,
    delay: String,
}

impl Default for EventDraft {
    fn default() -> Self {
        Self {
            at: false,
            signal: "V(out)".into(),
            value: String::new(),
            edge: "RISE",
            occurrence: "1".into(),
            delay: String::new(),
        }
    }
}

impl EventDraft {
    fn condition(&self) -> Result<String, String> {
        let signal = required(&self.signal, "Event signal")?;
        let value = required(&self.value, "Event threshold")?;
        let occurrence = required(&self.occurrence, "Event occurrence")?;
        Ok(format!("{signal}={value} {}={occurrence}", self.edge))
    }

    fn clause(&self, kind: &str) -> Result<String, String> {
        let mut card = if self.at {
            format!("{kind} AT={}", required(&self.value, "Axis value")?)
        } else {
            format!("{kind} {}", self.condition()?)
        };
        qualifier(&mut card, "TD", &self.delay)?;
        Ok(card)
    }

    fn show(&mut self, ui: &mut Ui, label: &str, allow_at: bool) {
        ui.push_id(label, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(label);
                if allow_at {
                    ui.selectable_value(&mut self.at, true, "Axis value");
                    ui.selectable_value(&mut self.at, false, "Crossing");
                }
                if allow_at && self.at {
                    field(ui, "At", &mut self.value, "1u or {sample_time}");
                } else {
                    field(ui, "Signal", &mut self.signal, "V(out)");
                    field(ui, "Threshold", &mut self.value, "0.5 or V(in)");
                    egui::ComboBox::from_id_salt("edge")
                        .selected_text(self.edge)
                        .show_ui(ui, |ui| {
                            for edge in ["RISE", "FALL", "CROSS"] {
                                ui.selectable_value(&mut self.edge, edge, edge);
                            }
                        });
                    field(ui, "Occurrence", &mut self.occurrence, "1 or LAST");
                }
                field(ui, "Ignore before", &mut self.delay, "optional TD");
            });
        });
    }
}

#[derive(Debug, Clone)]
pub(super) struct MeasurementBuilder {
    family: &'static str,
    operation: &'static str,
    signal: String,
    from: String,
    to: String,
    event: EventDraft,
    target: EventDraft,
    expression: String,
    options: String,
    reference_path: String,
    reference_column: String,
    reference_axis_column: String,
    reference_norm: &'static str,
}

impl Default for MeasurementBuilder {
    fn default() -> Self {
        Self {
            family: "TRAN",
            operation: "AVG",
            signal: "V(out)".into(),
            from: String::new(),
            to: String::new(),
            event: EventDraft::default(),
            target: EventDraft::default(),
            expression: String::new(),
            options: String::new(),
            reference_path: String::new(),
            reference_column: "1".into(),
            reference_axis_column: "0".into(),
            reference_norm: "L2NORM",
        }
    }
}

fn required<'a>(value: &'a str, label: &str) -> Result<&'a str, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("{label} is required"));
    }
    if value.contains(['\r', '\n', '\0']) {
        return Err(format!("{label} must be a single line"));
    }
    Ok(value)
}

fn qualifier(card: &mut String, key: &str, value: &str) -> Result<(), String> {
    if !value.trim().is_empty() {
        card.push_str(&format!(" {key}={}", required(value, key)?));
    }
    Ok(())
}

fn field(ui: &mut Ui, label: &str, value: &mut String, hint: &str) {
    ui.label(label);
    ui.add(
        egui::TextEdit::singleline(value)
            .desired_width(130.0)
            .hint_text(hint),
    );
}

impl MeasurementBuilder {
    pub(super) fn set_reference_path(&mut self, path: &str) {
        self.reference_path = path.to_owned();
    }
    pub(super) fn for_card(card: &str) -> Self {
        let family = card.split_whitespace().nth(1).unwrap_or_default();
        Self {
            family: ["TRAN", "AC", "DC", "NOISE"]
                .into_iter()
                .find(|candidate| candidate.eq_ignore_ascii_case(family))
                .unwrap_or("TRAN"),
            ..Self::default()
        }
    }

    fn card(&self, name: &str) -> Result<String, String> {
        let name = required(name, "Measurement name")?;
        if name.chars().any(char::is_whitespace) {
            return Err("Measurement name must be one token".into());
        }
        let body = match self.operation {
            "ERROR" => {
                let path = required(&self.reference_path, "Reference name")?;
                if path.contains('"') {
                    return Err("Reference name cannot contain quote characters".into());
                }
                let mut body = format!(
                    "ERROR {} FILE=\"{path}\" COMP_FUNCTION={} DEPVARCOL={}",
                    required(&self.signal, "Signal")?,
                    self.reference_norm,
                    required(&self.reference_column, "Comparison column")?
                );
                if self.family != "DC" {
                    qualifier(
                        &mut body,
                        "INDEPVARCOL",
                        required(&self.reference_axis_column, "Reference axis column")?,
                    )?;
                }
                body
            }
            "PARAM" => {
                let expression = required(&self.expression, "Expression")?;
                if expression.contains('\'') {
                    return Err("Enter the expression without quote characters".into());
                }
                format!("PARAM='{expression}'")
            }
            "TRIG" => format!(
                "{} {}",
                self.event.clause("TRIG")?,
                self.target.clause("TARG")?
            ),
            "WHEN" => {
                let mut body = format!("WHEN {}", self.event.condition()?);
                qualifier(&mut body, "TD", &self.event.delay)?;
                body
            }
            "FIND" | "DERIV" => {
                let signal = required(&self.signal, "Signal")?;
                let condition = if self.event.at {
                    format!("AT={}", required(&self.event.value, "Axis value")?)
                } else {
                    format!("WHEN {}", self.event.condition()?)
                };
                let mut body = format!("{} {signal} {condition}", self.operation);
                qualifier(&mut body, "TD", &self.event.delay)?;
                body
            }
            operation => format!("{operation} {}", required(&self.signal, "Signal")?),
        };
        let mut card = format!(".MEAS {} {name} {body}", self.family);
        if !matches!(self.operation, "PARAM" | "ERROR") {
            qualifier(&mut card, "FROM", &self.from)?;
            qualifier(&mut card, "TO", &self.to)?;
        }
        if !self.options.trim().is_empty() {
            card.push(' ');
            card.push_str(required(&self.options, "Additional options")?);
        }
        Ok(card)
    }

    /// Return a replacement only when the user applies the displayed card.
    pub(super) fn show(&mut self, ui: &mut Ui, name: &str) -> Option<String> {
        ui.label("Build a measurement card, then choose Use card to apply it.");
        ui.horizontal_wrapped(|ui| {
            ui.label("Analysis");
            egui::ComboBox::from_id_salt("measure-family")
                .selected_text(self.family)
                .show_ui(ui, |ui| {
                    for family in ["TRAN", "AC", "DC", "NOISE"] {
                        ui.selectable_value(&mut self.family, family, family);
                    }
                });
            egui::ComboBox::from_id_salt("measure-operation")
                .selected_text(
                    OPERATIONS
                        .iter()
                        .find(|(code, _)| *code == self.operation)
                        .map_or(self.operation, |(_, label)| *label),
                )
                .show_ui(ui, |ui| {
                    for &(code, label) in OPERATIONS {
                        ui.selectable_value(&mut self.operation, code, label);
                    }
                });
            if !matches!(self.operation, "TRIG" | "WHEN" | "PARAM") {
                field(ui, "Signal", &mut self.signal, "V(out), I(V1), VM(out)");
            }
        });
        match self.operation {
            "ERROR" => {
                ui.horizontal_wrapped(|ui| {
                    field(
                        ui,
                        "Reference name",
                        &mut self.reference_path,
                        "reference.csv",
                    );
                    field(
                        ui,
                        "Comparison column",
                        &mut self.reference_column,
                        "zero-based index",
                    );
                    if self.family != "DC" {
                        field(
                            ui,
                            "Axis column",
                            &mut self.reference_axis_column,
                            "zero-based index",
                        );
                    }
                    egui::ComboBox::from_id_salt("comparison-norm")
                        .selected_text(self.reference_norm)
                        .show_ui(ui, |ui| {
                            for (norm, label) in [
                                ("L2NORM", "L2 · Euclidean error"),
                                ("L1NORM", "L1 · Sum of absolute errors"),
                                ("INFNORM", "Maximum absolute error"),
                            ] {
                                ui.selectable_value(&mut self.reference_norm, norm, label);
                            }
                        });
                });
                ui.small("Attach the matching reference table below. DC compares rows in order; other analyses interpolate at the reference coordinates.");
            }
            "PARAM" => {
                ui.horizontal_wrapped(|ui| {
                    field(ui, "Expression", &mut self.expression, "gain / reference");
                });
            }
            "TRIG" => {
                self.event.show(ui, "Trigger", true);
                self.target.show(ui, "Target", true);
            }
            "FIND" | "DERIV" => self.event.show(ui, "Sample at", true),
            "WHEN" => self.event.show(ui, "Crossing", false),
            _ => {}
        }
        if !matches!(self.operation, "PARAM" | "ERROR") {
            ui.horizontal_wrapped(|ui| {
                field(ui, "From", &mut self.from, "optional axis value");
                field(ui, "To", &mut self.to, "optional axis value");
            });
            ui.small(match self.family {
                "TRAN" => "Axis values are seconds. Window reductions use accepted samples within From/To.",
                "DC" => "Axis values use the swept source or parameter's units.",
                _ => "Axis values are frequencies in Hz.",
            });
        }
        ui.horizontal_wrapped(|ui| {
            field(
                ui,
                "Additional options",
                &mut self.options,
                "GOAL=... TOL=... MINVAL=...",
            );
        });
        ui.small("Fields accept engineering notation and {design_parameters}. Additional options follow the active netlist dialect.");
        match self.card(name) {
            Ok(card) => {
                ui.monospace(&card);
                if ui.button("Use card").clicked() {
                    return Some(card);
                }
            }
            Err(error) => {
                ui.label(error);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests;

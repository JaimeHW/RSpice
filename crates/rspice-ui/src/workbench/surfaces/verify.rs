//! Verification evidence, specifications, checks, reliability, and history.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::state::SpecEntry;
use crate::ui::tokens::Tokens;

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot};
use super::super::state::VerificationPage;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("workbench.verify.surface")
            .show(ui, |ui| {
                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    ui.add_space(22.0);
                    ui.vertical(|ui| {
                        heading(
                            ui,
                            "Verification-evidence owner",
                            app.state.workbench.verification_page.label(),
                            "Traceable checks, specification verdicts, reliability evidence, and run history.",
                        );
                    });
                });
                ui.add_space(14.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(22.0);
                    for page in VerificationPage::ALL {
                        if ui
                            .selectable_label(app.state.workbench.verification_page == page, page.label())
                            .clicked()
                        {
                            app.state.workbench.verification_page = page;
                        }
                    }
                });
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    ui.add_space(22.0);
                    ui.vertical(|ui| {
                        ui.set_max_width(1080.0);
                        match app.state.workbench.verification_page {
                            VerificationPage::Cockpit => cockpit(ui, app),
                            VerificationPage::Specifications => specifications(ui, app),
                            VerificationPage::Checks => checks(ui, app),
                            VerificationPage::Reliability => reliability(ui, app),
                            VerificationPage::History => history(ui, app),
                        }
                    });
                });
                ui.add_space(24.0);
            });
    });
}

fn cockpit(ui: &mut Ui, app: &mut RSpiceApp) {
    let (spec_pass, spec_fail, spec_missing) = spec_summary(app);
    ui.horizontal_wrapped(|ui| {
        ui.set_width(330.0);
        card(ui, "Specifications", |ui| {
            property_row(ui, "Passing", &spec_pass.to_string());
            property_row(ui, "Failing", &spec_fail.to_string());
            property_row(ui, "No evidence", &spec_missing.to_string());
            let t = Tokens::get(ui.ctx());
            status_dot(
                ui,
                if spec_fail > 0 {
                    t.color.err
                } else if spec_missing > 0 {
                    t.color.warn
                } else {
                    t.color.ok
                },
                if spec_fail > 0 {
                    "Specification failures"
                } else if spec_missing > 0 {
                    "Evidence incomplete"
                } else {
                    "Specifications pass"
                },
            );
        });
        ui.set_width(330.0);
        card(ui, "Schematic checks", |ui| {
            if let Some(result) = &app.state.dialogs.drc_results {
                let summary = result.summary();
                property_row(ui, "Critical", &summary.critical.to_string());
                property_row(ui, "Errors", &summary.errors.to_string());
                property_row(ui, "Advisories", &summary.warnings.to_string());
                status_dot(
                    ui,
                    if summary.passed {
                        Tokens::get(ui.ctx()).color.ok
                    } else {
                        Tokens::get(ui.ctx()).color.err
                    },
                    &summary.display(),
                );
            } else {
                property_row(ui, "Status", "Not run");
            }
            if ui.button("Run schematic checks").clicked() {
                Command::RunChecks.execute(app);
            }
        });
        ui.set_width(330.0);
        card(ui, "Reliability and SOA", |ui| {
            property_row(
                ui,
                "Aging records",
                &app.state.simulation.reliability_results.len().to_string(),
            );
            property_row(
                ui,
                "SOA violations",
                &app.state.simulation.soa_violations.len().to_string(),
            );
            property_row(
                ui,
                "Completed runs",
                &app.state.simulation.runs.len().to_string(),
            );
            let t = Tokens::get(ui.ctx());
            status_dot(
                ui,
                if app.state.simulation.soa_violations.is_empty() {
                    t.color.ok
                } else {
                    t.color.err
                },
                if app.state.simulation.soa_violations.is_empty() {
                    "No recorded SOA violations"
                } else {
                    "SOA violations require review"
                },
            );
        });
    });
}

fn specifications(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Specification matrix", |ui| {
        ui.label("Bounds are project design intent. Verdicts are evaluated against the newest available .MEAS value.");
        ui.add_space(8.0);
        let verdicts: Vec<_> = app
            .state
            .workspace
            .specs
            .iter()
            .map(|spec| latest_measurement(app, &spec.measurement))
            .collect();
        let mut remove = None;
        for (index, spec) in app.state.workspace.specs.iter_mut().enumerate() {
            let t = Tokens::get(ui.ctx());
            egui::Frame::new()
                .fill(if app.state.workbench.selected_spec == Some(index) {
                    t.color.accent_dim
                } else {
                    t.color.bg_inset
                })
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    if ui
                        .interact(
                            ui.max_rect(),
                            ui.id().with(("spec", index)),
                            egui::Sense::click(),
                        )
                        .clicked()
                    {
                        app.state.workbench.selected_spec = Some(index);
                    }
                    ui.horizontal(|ui| {
                        ui.label(format!("Spec {}", index + 1));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("Remove").clicked() {
                                remove = Some(index);
                            }
                            verdict_badge(ui, spec, verdicts[index]);
                        });
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Measurement");
                        ui.add(
                            egui::TextEdit::singleline(&mut spec.measurement).desired_width(150.0),
                        );
                        optional_bound(ui, "Min", &mut spec.min);
                        optional_bound(ui, "Max", &mut spec.max);
                        ui.label("Unit");
                        ui.add(egui::TextEdit::singleline(&mut spec.unit).desired_width(70.0));
                    });
                });
            ui.add_space(6.0);
        }
        if let Some(index) = remove {
            app.state.workspace.specs.remove(index);
            app.state.workbench.selected_spec = None;
        }
        if ui.button("+ Add specification").clicked() {
            app.state.workspace.specs.push(SpecEntry {
                measurement: "measurement_name".to_owned(),
                min: None,
                max: None,
                unit: String::new(),
            });
            app.state.workbench.selected_spec = Some(app.state.workspace.specs.len() - 1);
        }
    });
}

fn checks(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Schematic checks", |ui| {
        ui.horizontal(|ui| {
            if ui.button("Run checks now").clicked() {
                Command::RunChecks.execute(app);
            }
            if ui
                .add_enabled(
                    Command::ClearChecks.is_enabled(app),
                    egui::Button::new("Clear evidence"),
                )
                .clicked()
            {
                Command::ClearChecks.execute(app);
            }
        });
        ui.add_space(8.0);
        let current =
            app.state.dialogs.drc_checked_version == app.state.schematic.topology_version();
        property_row(
            ui,
            "Evidence state",
            if current {
                "Current"
            } else {
                "Stale / not run"
            },
        );
        if let Some(result) = &app.state.dialogs.drc_results {
            property_row(ui, "Runtime", &format!("{} ms", result.duration_ms));
            for violation in result.violations() {
                ui.separator();
                ui.horizontal(|ui| {
                    let color = match violation.severity {
                        crate::services::drc::DrcSeverity::Critical
                        | crate::services::drc::DrcSeverity::Error => {
                            Tokens::get(ui.ctx()).color.err
                        }
                        crate::services::drc::DrcSeverity::Warning => {
                            Tokens::get(ui.ctx()).color.warn
                        }
                        crate::services::drc::DrcSeverity::Info => {
                            Tokens::get(ui.ctx()).color.text_dim
                        }
                    };
                    ui.label(
                        egui::RichText::new(violation.severity.display_name())
                            .color(color)
                            .strong(),
                    );
                    ui.vertical(|ui| {
                        ui.label(&violation.message);
                        ui.label(
                            egui::RichText::new(format!(
                                "{} · {}",
                                violation.location.display(),
                                violation.violation_type.suggested_fix()
                            ))
                            .small()
                            .color(Tokens::get(ui.ctx()).color.text_dim),
                        );
                    });
                });
            }
        } else {
            ui.label("Run checks to create current verification evidence.");
        }
    });
}

fn reliability(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Safe operating area", |ui| {
        if app.state.simulation.soa_violations.is_empty() {
            status_dot(ui, Tokens::get(ui.ctx()).color.ok, "No recorded violations");
        }
        for violation in &app.state.simulation.soa_violations {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(format!("{:?}", violation.severity))
                        .color(Tokens::get(ui.ctx()).color.err),
                );
                ui.label(&violation.device_id);
                ui.label(format!(
                    "{:?}: {:.6} (limit {:.6})",
                    violation.parameter, violation.actual_value, violation.limit_value
                ));
                ui.label(format!("t = {:.6} s", violation.time));
            });
        }
    });
    ui.add_space(12.0);
    card(ui, "Aging and reliability", |ui| {
        if app.state.simulation.reliability_results.is_empty() {
            ui.label("No reliability analysis evidence is available in the active result set.");
        }
        for result in &app.state.simulation.reliability_results {
            ui.collapsing(&result.device_id, |ui| {
                property_row(ui, "Stress", &format!("{:?}", result.stress));
                for (lifetime, shift) in &result.shifts {
                    property_row(
                        ui,
                        lifetime,
                        &format!(
                            "ΔVth {:.3e} · Δμ {:.3e} · ΔRds {:.3e}",
                            shift.vth_shift, shift.mobility_shift, shift.rds_shift
                        ),
                    );
                }
            });
        }
        if ui.button("Configure reliability analysis…").clicked() {
            app.state.workbench.workspace = super::super::state::Workspace::Simulate;
            app.state.workbench.active_analysis =
                crate::common::simulation_analysis_tabs::TAB_RELIABILITY;
        }
    });
}

fn history(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Verification run history", |ui| {
        if app.state.simulation.runs.is_empty() {
            ui.label("No simulation runs have produced verification evidence.");
        }
        for (index, run) in app.state.simulation.runs.iter().enumerate() {
            ui.horizontal(|ui| {
                status_dot(
                    ui,
                    if run.success {
                        Tokens::get(ui.ctx()).color.ok
                    } else {
                        Tokens::get(ui.ctx()).color.err
                    },
                    &run.label,
                );
                ui.label(format!(
                    "{} analyses · {:.3} s",
                    run.analyses.len(),
                    run.elapsed_time
                ));
                if ui.button("Open dataset").clicked() {
                    app.state.simulation.active_run_idx = Some(index);
                    app.state.workbench.workspace = super::super::state::Workspace::Results;
                }
            });
        }
    });
}

fn optional_bound(ui: &mut Ui, label: &str, value: &mut Option<f64>) {
    let mut enabled = value.is_some();
    ui.checkbox(&mut enabled, label);
    if enabled {
        let mut number = value.unwrap_or(0.0);
        if ui
            .add(
                egui::DragValue::new(&mut number)
                    .speed(0.01)
                    .max_decimals(12),
            )
            .changed()
            || value.is_none()
        {
            *value = Some(number);
        }
    } else {
        *value = None;
    }
}

fn latest_measurement(app: &RSpiceApp, name: &str) -> Option<f64> {
    app.state.simulation.runs.iter().find_map(|run| {
        run.analyses.iter().find_map(|analysis| {
            analysis
                .measurements
                .iter()
                .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
                .and_then(|measurement| measurement.value)
        })
    })
}

fn verdict_badge(ui: &mut Ui, spec: &SpecEntry, value: Option<f64>) {
    let t = Tokens::get(ui.ctx());
    let (text, color) = match value {
        Some(value) if spec.passes(value) => {
            (format!("PASS · {value:.6} {}", spec.unit), t.color.ok)
        }
        Some(value) => (format!("FAIL · {value:.6} {}", spec.unit), t.color.err),
        None => ("NO EVIDENCE".to_owned(), t.color.warn),
    };
    ui.label(egui::RichText::new(text).color(color).strong());
}

fn spec_summary(app: &RSpiceApp) -> (usize, usize, usize) {
    let mut pass = 0;
    let mut fail = 0;
    let mut missing = 0;
    for spec in &app.state.workspace.specs {
        match latest_measurement(app, &spec.measurement) {
            Some(value) if spec.passes(value) => pass += 1,
            Some(_) => fail += 1,
            None => missing += 1,
        }
    }
    (pass, fail, missing)
}

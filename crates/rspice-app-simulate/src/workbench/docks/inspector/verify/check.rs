//! The check selected in the Checks view: what its row does not say.
//!
//! An SOA rule's limits and thresholds, its worst point, how long it stayed
//! over and how often, and its derating; an `.ASSERT` card as written, with
//! every stretch it broke; an HDL record's words and the call that reported
//! it. Facts only: nothing here is advice.

use egui::Ui;
use rspice_core::ModelFinishPoint;
use rspice_core::config::HdlPoint;
use rspice_core::netlist::{AssertAnalysis, AssertLevel};
use rspice_results::verification::{AssertionCheck, Check, CheckRow, SoaCheck};
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::section_header;

use crate::workbench::app_state::Model;
use crate::workbench::design_system::{card_well, property_row};
use crate::workbench::state::VerificationSessionState;
use crate::workbench::surfaces::verify::checks::{
    selected_check, severity_task, soa_parameter_display, status_word,
};

/// A value in engineering notation with its unit, as the table states it.
fn value(value: f64, unit: &str) -> String {
    rspice_results::number_format::fmt_si_significant(value, unit, 4)
}

/// A swept value or a time, in the units of the analysis it was checked on.
fn on_axis(analysis: AssertAnalysis, at: f64) -> String {
    match analysis {
        AssertAnalysis::Tran => value(at, "s"),
        AssertAnalysis::Dc | AssertAnalysis::Op => value(at, ""),
    }
}

pub(super) fn check(ui: &mut Ui, model: &Model, session: &VerificationSessionState) {
    let Some(row) = selected_check(model, session) else {
        muted_inspector_copy(ui, "No check selected");
        return;
    };
    match &row.check {
        Check::Soa(soa) => soa_rule(ui, model, row, soa),
        Check::Assertion(assertion) => assertion_card(ui, model, row, assertion),
        Check::Hdl {
            severity,
            text,
            at,
            origin,
        } => {
            section_header(ui, severity_task(*severity), Some(status_word(row.status)));
            property_row(ui, "Severity", severity.label());
            property_row(ui, "Text", text.trim());
            property_row(ui, "Time", &hdl_time(*at));
            if !origin.scope.is_empty() {
                property_row(ui, "Scope", &origin.scope);
            }
            if !origin.file.is_empty() {
                property_row(ui, "Source", &format!("{}:{}", origin.file, origin.line));
            }
        }
        Check::Finish(finish) => {
            section_header(ui, finish.task.name(), Some(status_word(row.status)));
            property_row(ui, "Instance", &finish.instance);
            property_row(ui, "Model", &finish.model);
            property_row(ui, "Time", &finish_time(finish.point));
            if let Some(message) = &finish.message {
                property_row(ui, "Message", message.trim());
            }
            if let Some(source) = &finish.source {
                property_row(ui, "Source", &format!("{}:{}", source.file, source.line));
            }
        }
    }
}

fn hdl_time(at: HdlPoint) -> String {
    match at {
        HdlPoint::Transient { time } => value(time, "s"),
        HdlPoint::DcSweep { value: swept } => value(swept, ""),
        HdlPoint::OperatingPoint => "Operating point".to_owned(),
        HdlPoint::Initialization => "Initialization".to_owned(),
    }
}

fn finish_time(point: ModelFinishPoint) -> String {
    match point {
        ModelFinishPoint::Transient { time } => value(time, "s"),
        ModelFinishPoint::DcSweep { value: swept } => value(swept, ""),
        ModelFinishPoint::Frequency { frequency } => value(frequency, "Hz"),
        ModelFinishPoint::OperatingPoint => "Operating point".to_owned(),
        ModelFinishPoint::Initialization => "Initialization".to_owned(),
    }
}

/// The analysis a check came from, in the run Verify reads.
fn analysis_of<'m>(model: &'m Model, row: &CheckRow) -> Option<&'m crate::state::AnalysisResult> {
    model.simulation.active_run()?.analyses.get(row.analysis)
}

fn soa_rule(ui: &mut Ui, model: &Model, row: &CheckRow, soa: &SoaCheck) {
    let title = format!("{} {}", soa.device, soa_parameter_display(soa.parameter));
    section_header(ui, &title, Some(status_word(row.status)));
    if !soa.description.is_empty() {
        property_row(ui, "Rule", &soa.description);
    }
    property_row(ui, "Limit", &value(soa.limit, &soa.unit));
    let thresholds = analysis_of(model, row).and_then(|analysis| {
        let Some(crate::state::AnalysisResultPayload::Soa { evaluations, .. }) =
            &analysis.result_payload
        else {
            return None;
        };
        evaluations
            .iter()
            .find(|evaluation| {
                evaluation.device_id == soa.device && evaluation.parameter == soa.parameter
            })
            .map(|evaluation| evaluation.thresholds)
    });
    if let Some(thresholds) = thresholds {
        let fraction = |fraction: Option<f64>| {
            fraction.map_or_else(
                || "Off".to_owned(),
                |fraction| format!("{:.0} % of limit", fraction * 100.0),
            )
        };
        property_row(ui, "Warning at", &fraction(thresholds.warning_fraction));
        property_row(ui, "Critical at", &fraction(thresholds.critical_fraction));
    }
    property_row(ui, "Observed worst", &value(soa.observed, &soa.unit));
    if let Some(margin) = soa.margin {
        property_row(ui, "Margin", &format!("{:+.1} %", margin * 100.0));
    }
    property_row(ui, "When", &value(soa.worst_time_s, "s"));

    if let Some(duration) = soa.duration {
        section_header(ui, "Duration", None);
        property_row(ui, "Minimum", &value(duration.minimum_duration_s, "s"));
        property_row(ui, "Over limit", &value(duration.total_exceedance_s, "s"));
        property_row(ui, "Longest", &value(duration.longest_excursion_s, "s"));
        property_row(
            ui,
            "Excursions",
            &format!(
                "{} counted \u{00b7} {} rejected \u{00b7} {} clipped",
                duration.qualified_excursions,
                duration.rejected_excursions,
                duration.clipped_excursions
            ),
        );
        if let Some(cumulative) = duration.cumulative {
            property_row(ui, "Peak exposure", &value(cumulative.peak_exposure_s, "s"));
            property_row(
                ui,
                "Final exposure",
                &value(cumulative.final_exposure_s, "s"),
            );
            property_row(
                ui,
                "Recovery",
                &cumulative
                    .recovery_time_s
                    .map_or_else(|| "None".to_owned(), |recovery| value(recovery, "s")),
            );
        }
    }
    if let Some(derating) = soa.derating {
        section_header(ui, "Derating", None);
        property_row(ui, "Rated power", &value(derating.rated_power_w, "W"));
        property_row(ui, "Slope", &value(derating.curve.watts_per_kelvin, "W/K"));
        property_row(
            ui,
            "Above",
            &format!(
                "{:.1} \u{00b0}C",
                derating.curve.reference_temperature_kelvin - 273.15
            ),
        );
    }
}

fn assertion_card(ui: &mut Ui, model: &Model, row: &CheckRow, assertion: &AssertionCheck) {
    section_header(ui, &assertion.name, Some(status_word(row.status)));
    let report =
        analysis_of(model, row).and_then(|analysis| {
            analysis.checks.assertions.iter().find(|report| {
                report.name == assertion.name && report.instance == assertion.instance
            })
        });
    if let Some(card) = report.and_then(|report| card_text(model, &report.origin)) {
        card_well(ui, Ok(&card), 0.0);
    }
    property_row(ui, "Checks", &assertion.target);
    if let Some(instance) = &assertion.instance {
        property_row(ui, "Instance", instance);
    }
    property_row(
        ui,
        "Analysis",
        match assertion.analysis {
            AssertAnalysis::Op => "Operating point",
            AssertAnalysis::Dc => "DC sweep",
            AssertAnalysis::Tran => "Transient",
        },
    );
    property_row(
        ui,
        "Level",
        match assertion.level {
            AssertLevel::Notice => "Notice",
            AssertLevel::Warning => "Warning",
            AssertLevel::Error => "Error",
        },
    );
    if let Some(message) = &assertion.message {
        property_row(ui, "Message", message);
    }
    if let Some(report) = report {
        property_row(ui, "Written at", &report.origin.to_string());
        if let Some(error) = &report.error {
            property_row(ui, "Not evaluated", error);
        }
        if !report.violations.is_empty() {
            let count = report.violations.len().to_string();
            section_header(ui, "Violations", Some(&count));
            for (index, violation) in report.violations.iter().enumerate() {
                property_row(
                    ui,
                    &(index + 1).to_string(),
                    &stretch(assertion.analysis, violation),
                );
            }
        }
    }
}

/// One stretch an assertion broke: where it starts and ends, and its value
/// farthest outside the band and where that was.
fn stretch(analysis: AssertAnalysis, violation: &rspice_core::analysis::AssertViolation) -> String {
    let span = match (violation.start, violation.end) {
        (Some(start), Some(end)) => format!(
            "{} \u{2013} {}",
            on_axis(analysis, start),
            on_axis(analysis, end)
        ),
        (Some(start), None) => format!("{} \u{2013} stop", on_axis(analysis, start)),
        _ => "Operating point".to_owned(),
    };
    match (violation.extreme, violation.at) {
        (Some(extreme), Some(at)) => format!(
            "{span} \u{00b7} {} at {}",
            value(extreme, ""),
            on_axis(analysis, at)
        ),
        (Some(extreme), None) => format!("{span} \u{00b7} {}", value(extreme, "")),
        _ => span,
    }
}

/// The `.ASSERT` card as the deck the run executed wrote it, with its
/// continuation lines; `None` when the card is in an included file or the
/// deck was not kept.
fn card_text(
    model: &Model,
    origin: &rspice_core::netlist::NetlistSourceLocation,
) -> Option<String> {
    if origin.path.is_some() || origin.line == 0 {
        return None;
    }
    let run = model.simulation.active_run()?;
    let deck = model.simulation.retained.executed_decks.get(run.id)?;
    let mut lines = deck.point(0)?.deck.lines().skip(origin.line - 1);
    let mut card = lines.next()?.trim().to_owned();
    for line in lines {
        let Some(continued) = line.trim_start().strip_prefix('+') else {
            break;
        };
        card.push(' ');
        card.push_str(continued.trim());
    }
    Some(card)
}

#[cfg(test)]
mod tests;

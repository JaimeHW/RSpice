//! The status grammar as PNGs for a person to look at: the four toast
//! severities, a command form's refusal, and a panel built from rows the
//! product's panels really show — one of each panel primitive, on the panel
//! inset and rhythm — in both themes, with the panel's measured geometry
//! written beside it.
//!
//! Nothing here asserts; the contract tests beside each primitive do. The one
//! test is ignored and writes to `RSPICE_RASTER_DIR` (default: the system
//! temp directory).

use egui::Ui;

use crate::panels::{
    Problem, PropertyRowAction, PropertyRowCaption, WorkbenchIcon, code_inspector_section,
    inspector, panel_body, problem_line, property_row, property_row_input,
    property_row_input_action, property_row_input_captioned, property_row_input_with_hint,
    property_row_problem, section_block_gap, section_header, section_header_action,
    section_header_explained, subsection_header,
};
use crate::raster::{Canvas, render};
use crate::tokens::Mode;
use crate::widgets::{Button, CommandForm, NotificationCategory, ToastKind, Toasts};

/// One toast as the review seeds it: severity, title, detail.
type ReviewToast = (ToastKind, &'static str, &'static str);

#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_status_grammar_for_review() {
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let write = |stem: &str, canvas: &Canvas| {
        let height = canvas.content_height().max(1);
        let path = directory.join(format!("{stem}.png"));
        std::fs::write(&path, canvas.png(height)).expect("write review render");
        eprintln!("{} {}x{height}", path.display(), canvas.width());
    };

    // Three toasts is the stack's ceiling, so the error gets a canvas of its
    // own.
    let toast_sets: [(&str, &[ReviewToast]); 2] = [
        (
            "k1-toasts-success-info-warn",
            &[
                (
                    ToastKind::Success,
                    "Run 41 complete",
                    "Transient \u{b7} 2.1 s \u{b7} 4 outputs",
                ),
                (
                    ToastKind::Info,
                    "Project opened",
                    "ota_5t \u{b7} 3 schematics",
                ),
                (
                    ToastKind::Warn,
                    "Model fallback",
                    "M3 uses the default level-1 card",
                ),
            ],
        ),
        (
            "k1-toasts-error",
            &[(
                ToastKind::Error,
                "Run 40 failed",
                "Timestep too small at 1.204 \u{b5}s",
            )],
        ),
    ];
    for (stem, notices) in toast_sets {
        let mut toasts = Toasts::default();
        let mut seeded = false;
        let canvas = render(egui::vec2(560.0, 360.0), |ui, background| {
            ui.ctx()
                .global_style_mut(|style| style.animation_time = 0.0);
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |_| {});
            if !seeded {
                for (kind, title, message) in notices {
                    toasts.notify_with_title(
                        ui.ctx(),
                        NotificationCategory::System,
                        *kind,
                        *title,
                        *message,
                    );
                }
                seeded = true;
            }
            let _ = toasts.show(ui.ctx(), 0.0, 0.0, false);
        });
        write(stem, &canvas);
    }

    let form = render(egui::vec2(760.0, 520.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |_| {});
        let mut names = String::from("OUT");
        let mut direction = 1;
        let _ = CommandForm::new("Add interface pins", "Add")
            .context(Some("work/ota_5t/schematic"))
            .status(Err("OUT: an interface port named 'OUT' already exists"))
            .status_lines(2)
            .show(ui.ctx(), |rows| {
                let field = rows.text(
                    "Names",
                    egui::Id::new("k1-review.names"),
                    &mut names,
                    "IN OUT VDD",
                );
                rows.segmented(
                    "Direction",
                    "k1-review.direction",
                    &["Input", "Output", "Inout", "Supply"],
                    &mut direction,
                );
                Some(field.id)
            });
    });
    write("k1-command-form-refusal", &form);

    let light = crate::Theme {
        mode: Mode::Light,
        ..crate::Theme::default()
    };
    for (stem, theme) in [
        ("k3-panel-grammar-dark", crate::Theme::default()),
        ("k3-panel-grammar-light", light),
    ] {
        let mut instance = String::from("D1");
        let mut value = String::new();
        let mut temperature = String::new();
        let mut perimeter = String::from("0");
        let mut watch = String::new();
        let mut policy = String::from("uncaught exceptions");
        crate::raster::write_panel_review(
            &directory,
            stem,
            theme,
            egui::vec2(312.0, 1400.0),
            10.0,
            |ui, _| {
                let panel = crate::tokens::Tokens::get(ui.ctx()).color.bg_panel;
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(panel))
                    .show(ui, |ui| {
                        panel_grammar_sheet(
                            ui,
                            &mut instance,
                            &mut value,
                            &mut temperature,
                            &mut perimeter,
                            &mut watch,
                            &mut policy,
                        );
                    });
            },
        );
    }
}

/// Rows the product's panels really show, one of each panel primitive: the
/// design inspector on a diode bound to its catalog model, the Verilog-A
/// inspector on a module that failed to compile, the Waves details panel's
/// measurements, the Automation inspector's debugger and watches, and the
/// source-document inspector's file operations.
#[allow(clippy::too_many_arguments)]
fn panel_grammar_sheet(
    ui: &mut Ui,
    instance: &mut String,
    value: &mut String,
    temperature: &mut String,
    perimeter: &mut String,
    watch: &mut String,
    policy: &mut String,
) {
    panel_body(ui, |ui| {
        panel_grammar_rows(ui, instance, value, temperature, perimeter, watch, policy);
    });
}

#[allow(clippy::too_many_arguments)]
fn panel_grammar_rows(
    ui: &mut Ui,
    instance: &mut String,
    value: &mut String,
    temperature: &mut String,
    perimeter: &mut String,
    watch: &mut String,
    policy: &mut String,
) {
    let _ = section_header_action(ui, "Identity", "Properties\u{2026}", true);
    property_row_input(ui, "Instance", instance, false);
    let _ = property_row_input_action(
        ui,
        "Value",
        value,
        false,
        PropertyRowAction {
            icon: WorkbenchIcon::Sliders,
            label: "Tune value",
            enabled: true,
            disabled_reason: None,
        },
    );
    property_row(ui, "Library cell", "primitives/Diode");
    property_row(ui, "View", "symbol \u{b7} spice");
    section_block_gap(ui);
    let _ = Button::new("Open model source\u{2026}").ghost().show(ui);

    section_header(ui, "Simulation parameters", Some("editable"));
    property_row(ui, "Model", "junction");
    property_row(ui, "Section", "Not selected");
    property_row_input_with_hint(
        ui,
        "Temperature",
        temperature,
        "inherit \u{b7} 27 \u{b0}C",
        false,
    );
    property_row_input_captioned(
        ui,
        "Perimeter",
        perimeter,
        "",
        false,
        PropertyRowCaption {
            unit: Some("m"),
            description: None,
        },
    );

    section_header(ui, "Checks", Some("stale"));
    property_row_problem(
        ui,
        "Connectivity",
        "pending recheck",
        Some(Problem::Warning),
    );
    property_row_problem(
        ui,
        "Floating nets",
        "pending recheck",
        Some(Problem::Warning),
    );
    property_row(ui, "Last checked", "rerun for topology revision 3");

    section_header_explained(
        ui,
        "Measurements",
        "linear interpolation",
        "Values between samples are read off a straight line between them.",
    );
    crate::widgets::measurement_table(
        ui,
        &[
            ("V(out) min", "4.100000 V"),
            ("V(out) max", "5.900000 V"),
            ("V(out) rms", "5.040311 V"),
        ],
    );

    code_inspector_section(
        ui,
        "Diagnostics",
        Some("1 error"),
        Some(Problem::Failure),
        |ui| {
            problem_line(
                ui,
                Problem::Failure,
                "Code generation error: Cannot compile expression: Unknown identifier: gain",
            );
        },
    );
    code_inspector_section(ui, "Parameters", Some("compile required"), None, |ui| {
        inspector::muted_inspector_copy(
            ui,
            "Compile the module to list the parameters an instance takes.",
        );
    });

    code_inspector_section(ui, "Debug", Some("inactive"), None, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::splat(4.0);
            let _ = Button::new("Start").show(ui);
            let _ = Button::new("Continue").enabled(false).show(ui);
            let _ = Button::new("Pause").enabled(false).show(ui);
            let _ = Button::new("Step over").enabled(false).show(ui);
        });
        section_block_gap(ui);
        property_row(ui, "Current line", "\u{2014}");
        property_row(ui, "Exception policy", "uncaught exceptions");
        egui::ComboBox::from_id_salt("k3-review.exception-policy")
            .selected_text(policy.as_str())
            .width(ui.available_width().max(1.0))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    policy,
                    "uncaught exceptions".to_owned(),
                    "uncaught exceptions",
                );
            });
        subsection_header(ui, "WATCHES");
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(watch)
                    .hint_text("expression")
                    .desired_width((ui.available_width() - 48.0).max(60.0)),
            );
            let _ = Button::new("Add").show(ui);
        });
    });

    code_inspector_section(ui, "File operations", None, None, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::splat(6.0);
            let _ = Button::new("New source\u{2026}").show(ui);
            let _ = Button::new("Rename\u{2026}").show(ui);
            let _ = Button::new("Move\u{2026}").show(ui);
            let _ = Button::new("Delete\u{2026}").show(ui);
        });
    });
}


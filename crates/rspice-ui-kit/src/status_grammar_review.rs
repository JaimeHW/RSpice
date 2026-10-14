//! The status grammar as PNGs for a person to look at: the four toast
//! severities, a command form's refusal, and an inspector of property rows,
//! problem marks, sentence-case headers, a code inspector warning, the
//! bordered and filled destructive buttons and a stacked field label, in both
//! themes.
//!
//! Nothing here asserts; the contract tests beside each primitive do. The one
//! test is ignored and writes to `RSPICE_RASTER_DIR` (default: the system
//! temp directory).

use egui::{Color32, Ui};

use crate::panels::{
    Ink, Problem, code_inspector_section, inspector, property_row, property_row_ink,
    property_row_problem, schematic_property_row,
};
use crate::raster::{Canvas, render, render_themed};
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

    let inspector_pass = |ui: &mut Ui, background: Color32| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                inspector::begin_inspector_sections(ui);
                inspector::section_header(ui, "Terminal placement", Some("3 pins"));
                property_row(ui, "Instance", "R1");
                property_row_problem(ui, "Pin 1", "placed", None);
                property_row_problem(ui, "Pin 2", "unplaced", Some(Problem::Warning));
                property_row_problem(ui, "Pin 3", "off grid", Some(Problem::Failure));
                property_row_ink(ui, "Source", "inherited", Ink::Faint);
                inspector::inspector_problem(ui, Problem::Warning, "Two pins share one location.");
                inspector::schematic_section_header(
                    ui,
                    "Drawing sheet",
                    Some("inherited \u{b7} project default"),
                );
                schematic_property_row(ui, "Format", "A3 landscape");
                let _ =
                    inspector::schematic_section_header_action(ui, "Net classes", "+ Add", true);
                let _ =
                    crate::widgets::section_header(ui, "Run history for DC sweep", Some("Clear"));
                crate::widgets::pane_section_label(ui, "Environment");
                code_inspector_section(
                    ui,
                    "Diagnostics",
                    Some("2 advisories"),
                    Some(Problem::Warning),
                    |ui| {
                        crate::widgets::problem_line(
                            ui,
                            Problem::Warning,
                            "Parameter gain is declared and never read.",
                        );
                        crate::widgets::problem_line(
                            ui,
                            Problem::Warning,
                            "Net vx is created implicitly.",
                        );
                    },
                );
                code_inspector_section(ui, "Debugger", Some("Paused"), None, |_| {});
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    let _ = Button::new("Remove").destructive(true).show(ui);
                    let _ = Button::new("Duplicate").show(ui);
                    let _ = Button::new("Delete").accent().destructive(true).show(ui);
                });
                ui.add_space(10.0);
                let mut name = String::from("vout");
                crate::widgets::field_label(ui, "Net name", |ui| {
                    ui.text_edit_singleline(&mut name);
                });
                ui.add_space(6.0);
                crate::widgets::read_only_value(ui, "Document naming policy", "As typed");
            });
    };
    write(
        "k1-inspector-grammar-dark",
        &render(egui::vec2(360.0, 900.0), inspector_pass),
    );
    let light = crate::Theme {
        mode: Mode::Light,
        ..crate::Theme::default()
    };
    write(
        "k1-inspector-grammar-light",
        &render_themed(light, egui::vec2(360.0, 900.0), inspector_pass),
    );
}

//! The recovery and safe-mode pages.
//!
//! A recovery candidate is presented with the exact contract it would restore
//! — the source it came from, when it was captured, and what it does not
//! include — because recovering is a decision the engineer has to make with
//! full knowledge, not a convenience. Safe mode states which features it
//! disables rather than starting silently degraded.

use super::*;

pub(super) fn recovery_page(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    launcher_page_heading(
        ui,
        "STARTUP RECOVERY · NON-DESTRUCTIVE",
        "Recover interrupted work",
        "Recovery opens comparison copies. It never overwrites the saved project, immutable results, or approved evidence until you explicitly accept changes.",
        layout,
    );
    let footer_height = launcher_footer_reserve(
        ui,
        layout,
        &[
            ("Discard selected checkpoint…", false),
            ("Recovery options", false),
            ("Open recovery comparison", true),
        ],
    );
    let regions = launcher_page_regions(ui, footer_height);
    let mut body_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(regions.body)
            .layout(egui::Layout::top_down(Align::Min)),
    );
    egui::ScrollArea::vertical()
        .id_salt("workbench.project_launcher.recovery")
        .auto_shrink([false, false])
        .show(&mut body_ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    if let Some(notice) =
                        app.state.workbench.project_launcher_recovery.notice.clone()
                    {
                        let color = match notice.tone {
                            RecoveryNoticeTone::Info => t.color.ok,
                            RecoveryNoticeTone::Warning => t.color.warn,
                        };
                        Frame::new()
                            .fill(color.gamma_multiply(0.10))
                            .stroke(Stroke::new(1.0, color.gamma_multiply(0.65)))
                            .corner_radius(t.radius)
                            .inner_margin(Margin::symmetric(12, 9))
                            .outer_margin(Margin::symmetric(16, 10))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(notice.message)
                                        .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                                        .color(t.color.text),
                                );
                            });
                    }

                    let candidates = app
                        .state
                        .workbench
                        .project_launcher_recovery
                        .candidates
                        .clone();
                    let selected = app
                        .state
                        .workbench
                        .project_launcher_recovery
                        .selected_checkpoint
                        .clone();
                    Frame::new()
                        .inner_margin(Margin::symmetric(16, 0))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            if candidates.is_empty() {
                                Frame::new()
                                    .inner_margin(Margin::symmetric(18, 28))
                                    .show(ui, |ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.label(
                                                egui::RichText::new("No interrupted work found")
                                                    .font(theme::sans(
                                                        tokens::FS_2,
                                                        FontWeight::SemiBold,
                                                    ))
                                                    .color(t.color.text),
                                            );
                                            ui.label(
                                                egui::RichText::new(
                                                    "No eligible interrupted-session checkpoint is associated with a recent local schematic.",
                                                )
                                                .font(theme::sans(
                                                    tokens::FS_1,
                                                    FontWeight::Regular,
                                                ))
                                                .color(t.color.text_dim),
                                            );
                                        });
                                    });
                            } else {
                                for candidate in candidates {
                                    let is_selected =
                                        selected.as_ref() == Some(&candidate.checkpoint);
                                    let row = recovery_row(ui, &candidate, is_selected, layout);
                                    if response_activated(ui, &row) {
                                        app.state
                                            .workbench
                                            .project_launcher_recovery
                                            .select(candidate.checkpoint.clone());
                                    }
                                    ui.painter().hline(
                                        ui.max_rect().x_range(),
                                        ui.cursor().top(),
                                        Stroke::new(1.0, t.color.border),
                                    );
                                }
                            }
                        });

                    if let Some(candidate) = app
                        .state
                        .workbench
                        .project_launcher_recovery
                        .selected()
                        .cloned()
                    {
                        recovery_contract(ui, &candidate, layout);
                    }
        });

    const LEGACY_CHECKPOINT_ADVISORY: &str = "Legacy checkpoint ownership cannot be proven; open it non-destructively or use explicit recovery maintenance or migration";

    let selected = app
        .state
        .workbench
        .project_launcher_recovery
        .selected()
        .cloned();
    launcher_page_footer(ui, layout, regions.footer, |ui| {
        let can_discard = selected
            .as_ref()
            .is_some_and(RecoveryCandidate::can_discard);
        let discard = Button::new("Discard selected checkpoint…")
            .enabled(can_discard)
            .show(ui);
        let discard = if selected
            .as_ref()
            .is_some_and(RecoveryCandidate::is_legacy_checkpoint)
        {
            // A legacy checkpoint may still be discardable, so this advisory
            // has to read in both states; each call is inert in the other one.
            discard
                .on_hover_text(LEGACY_CHECKPOINT_ADVISORY)
                .on_disabled_hover_text(LEGACY_CHECKPOINT_ADVISORY)
        } else {
            discard
        };
        if discard.clicked()
            && let Some(candidate) = selected.clone()
        {
            *action = Some(LauncherAction::RequestDiscard(candidate));
        }
        if Button::new("Recovery options").show(ui).clicked() {
            *action = Some(LauncherAction::Page(ProjectLauncherPage::SafeMode));
        }
        let recoverable = selected
            .as_ref()
            .is_some_and(|candidate| candidate.integrity.is_recoverable());
        let replacement_block_reason = recoverable
            .then(|| recovery_replacement_block_reason(&app.state))
            .flatten();
        let response = Button::new("Open recovery comparison")
            .accent()
            .enabled(recoverable && replacement_block_reason.is_none())
            .show(ui);
        let response = if let Some(reason) = replacement_block_reason {
            response.on_disabled_hover_text(reason)
        } else if !recoverable && selected.is_some() {
            response.on_disabled_hover_text("The selected checkpoint failed integrity validation")
        } else {
            response
        };
        if response.clicked()
            && let Some(candidate) = selected
        {
            *action = Some(LauncherAction::Recover(candidate));
        }
    });
}

pub(super) fn recovery_row(
    ui: &mut Ui,
    candidate: &RecoveryCandidate,
    selected: bool,
    layout: LauncherLayout,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let response = Frame::new()
        .fill(if selected {
            t.color.bg_hover
        } else {
            egui::Color32::TRANSPARENT
        })
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_height(54.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 11.0;
                let (radio_rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
                ui.painter().circle_stroke(
                    radio_rect.center(),
                    6.0,
                    Stroke::new(
                        1.0,
                        if selected {
                            t.color.accent
                        } else {
                            t.color.border_strong
                        },
                    ),
                );
                if selected {
                    ui.painter()
                        .circle_filled(radio_rect.center(), 3.0, t.color.accent);
                }

                let trailing_width = if layout.compact { 64.0 } else { 211.0 };
                let content_width = (ui.available_width() - trailing_width - 11.0).max(1.0);
                ui.allocate_ui_with_layout(
                    vec2(content_width, 54.0),
                    egui::Layout::top_down(Align::Min).with_cross_align(Align::Min),
                    |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.add_space(7.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "{} · interrupted session",
                                candidate.display_name
                            ))
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                        );
                        ui.label(
                            egui::RichText::new(recovery_summary(candidate))
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                    },
                );
                if !layout.compact {
                    ui.add_sized(
                        [130.0, 54.0],
                        egui::Label::new(
                            egui::RichText::new(&candidate.age)
                                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                                .color(t.color.text_faint),
                        ),
                    );
                }
                ui.add_sized(
                    [if layout.compact { 64.0 } else { 70.0 }, 54.0],
                    egui::Label::new(
                        egui::RichText::new(if candidate.integrity.is_recoverable() {
                            "review"
                        } else {
                            "blocked"
                        })
                        .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                        .color(if candidate.integrity.is_recoverable() {
                            t.color.warn
                        } else {
                            t.color.err
                        }),
                    ),
                );
            });
        })
        .response
        .interact(Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::RadioButton,
            ui.is_enabled(),
            selected,
            format!("Select recovery checkpoint for {}", candidate.display_name),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(recovery_summary(candidate));
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub(super) fn recovery_summary(candidate: &RecoveryCandidate) -> String {
    match &candidate.integrity {
        RecoveryIntegrity::Verified {
            baseline_available,
            baseline_note,
            components,
            wires,
            changed_objects,
        } => {
            let delta = changed_objects.map_or_else(
                || "change count unavailable".to_owned(),
                |count| format!("{count} structural changes"),
            );
            format!(
                "{components} components · {wires} wires · {delta} · {}",
                if *baseline_available && baseline_note.is_none() {
                    "checkpoint and baseline verified"
                } else if *baseline_available {
                    "checkpoint verified · saved baseline changed or unrecorded"
                } else {
                    "checkpoint verified · baseline unavailable"
                }
            )
        }
        RecoveryIntegrity::Invalid(error) => error.clone(),
    }
}

pub(super) fn recovery_contract(
    ui: &mut Ui,
    candidate: &RecoveryCandidate,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .outer_margin(Margin::symmetric(16, 15))
        .show(ui, |ui| {
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, t.color.border),
            );
            recovery_contract_row(
                ui,
                "Saved source",
                &candidate.original.display().to_string(),
                layout,
            );
            recovery_contract_row(
                ui,
                "Recovery point",
                &candidate.checkpoint.display().to_string(),
                layout,
            );
            match &candidate.integrity {
                RecoveryIntegrity::Verified {
                    baseline_available,
                    baseline_note,
                    ..
                } => {
                    recovery_contract_row(
                        ui,
                        "Protected data",
                        "Saved source and checkpoint remain unchanged; recovery opens as an unsaved project.",
                        layout,
                    );
                    recovery_contract_row(
                        ui,
                        "Recommended action",
                        if *baseline_available {
                            "Compare the editable recovery candidate with the read-only saved baseline."
                        } else {
                            baseline_note.as_deref().unwrap_or(
                                "Review the verified checkpoint without a saved baseline.",
                            )
                        },
                        layout,
                    );
                    if let Some(note) = baseline_note {
                        recovery_contract_row(ui, "Baseline status", note, layout);
                    }
                }
                RecoveryIntegrity::Invalid(error) => {
                    recovery_contract_row(ui, "Integrity", error, layout);
                    recovery_contract_row(
                        ui,
                        "Required action",
                        "Retain or discard the checkpoint; invalid content is never opened.",
                        layout,
                    );
                }
            }
        });
}

pub(super) fn recovery_contract_row(ui: &mut Ui, label: &str, value: &str, layout: LauncherLayout) {
    let t = Tokens::get(ui.ctx());
    if layout.phone {
        // The mockup collapses the two-column grid into two ruled cells, not
        // one card-like stacked row.
        for (text, strong) in [(label, true), (value, false)] {
            Frame::new()
                .inner_margin(Margin::symmetric(9, 8))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.set_min_height(18.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(text)
                                .font(theme::sans(
                                    tokens::FS_2,
                                    if strong {
                                        FontWeight::SemiBold
                                    } else {
                                        FontWeight::Regular
                                    },
                                ))
                                .color(if strong {
                                    t.color.text_dim
                                } else {
                                    t.color.text
                                }),
                        )
                        .wrap(),
                    );
                });
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, t.color.border),
            );
        }
        return;
    }
    Frame::new()
        .inner_margin(Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            let add_label = |ui: &mut Ui| {
                ui.label(
                    egui::RichText::new(label)
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
            };
            let add_value = |ui: &mut Ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                            .color(t.color.text),
                    )
                    .wrap(),
                );
            };
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(170.0, 18.0),
                    egui::Layout::left_to_right(Align::Center),
                    add_label,
                );
                add_value(ui);
            });
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
}

pub(super) fn safe_mode_page(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    launcher_page_heading(
        ui,
        "SAFE MODE · STARTUP ISOLATION",
        "Start with optional subsystems disabled",
        "Safe mode changes only the current launch. It is intended for crash isolation, display recovery, extension diagnosis, and project repair.",
        layout,
    );
    let active = app.state.workbench.safe_mode.active;
    let footer_height = launcher_footer_reserve(
        ui,
        layout,
        &[
            ("Open diagnostic folder", false),
            ("Start RSpice in safe mode", true),
        ],
    );
    let regions = launcher_page_regions(ui, footer_height);
    let mut body_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(regions.body)
            .layout(egui::Layout::top_down(Align::Min)),
    );
    egui::ScrollArea::vertical()
        .id_salt("workbench.project_launcher.safe-mode")
        .auto_shrink([false, false])
        .show(&mut body_ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    if active {
                        Frame::new()
                            .fill(t.color.ok.gamma_multiply(0.10))
                            .stroke(Stroke::new(1.0, t.color.ok.gamma_multiply(0.65)))
                            .corner_radius(t.radius)
                            .inner_margin(Margin::symmetric(12, 9))
                            .outer_margin(Margin::symmetric(16, 10))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new("Safe mode is active for this launch")
                                        .font(theme::sans(
                                            tokens::FS_1,
                                            FontWeight::SemiBold,
                                        ))
                                        .color(t.color.text),
                                );
                            });
                    }

                    ui.add_enabled_ui(!active, |ui| {
                        safe_mode_option(
                            ui,
                            &mut app
                                .state
                                .workbench
                                .safe_mode
                                .draft
                                .disable_third_party_extensions,
                            "Disable third-party extensions",
                            "Built-in signed components remain available.",
                            None,
                        );
                        safe_mode_option(
                            ui,
                            &mut app
                                .state
                                .workbench
                                .safe_mode
                                .draft
                                .disable_gpu_acceleration,
                            "Disable GPU acceleration",
                            "Use software rendering and conservative canvas limits.",
                            (!software_rendering_supported()).then_some(
                                "This renderer was selected before the launcher opened and cannot switch to a verified software adapter in this build.",
                            ),
                        );
                        safe_mode_option(
                            ui,
                            &mut app
                                .state
                                .workbench
                                .safe_mode
                                .draft
                                .isolate_prior_documents,
                            "Do not reopen prior documents",
                            "Start in the empty workbench with project files closed.",
                            None,
                        );
                        safe_mode_option(
                            ui,
                            &mut app.state.workbench.safe_mode.draft.reset_layout,
                            "Reset dock and monitor geometry",
                            "Recover panels and windows to the primary display.",
                            None,
                        );
                        safe_mode_option(
                            ui,
                            &mut app
                                .state
                                .workbench
                                .safe_mode
                                .draft
                                .open_project_read_only,
                            "Open project read-only",
                            "Prevent working-document writes while diagnosing content.",
                            None,
                        );
                    });

        });

    let options = app.state.workbench.safe_mode.draft;
    launcher_page_footer(ui, layout, regions.footer, |ui| {
        let diagnostics = Button::new("Open diagnostic folder")
            .enabled(diagnostics_folder_supported())
            .show(ui);
        let diagnostics = if diagnostics_folder_supported() {
            diagnostics
        } else {
            diagnostics.on_disabled_hover_text(
                "Diagnostic folders can be revealed only by supported desktop file managers",
            )
        };
        if diagnostics.clicked() {
            *action = Some(LauncherAction::OpenDiagnosticsFolder);
        }
        let response = Button::new("Start RSpice in safe mode")
            .accent()
            .enabled(!active && options.has_effect())
            .show(ui);
        let response = if active {
            response.on_disabled_hover_text("Safe mode is already active for this launch")
        } else if !options.has_effect() {
            response.on_disabled_hover_text("Select at least one isolation option")
        } else {
            response
        };
        if response.clicked() {
            *action = Some(LauncherAction::StartSafeMode(options));
        }
    });
}

pub(super) fn safe_mode_option(
    ui: &mut Ui,
    checked: &mut bool,
    title: &str,
    detail: &str,
    unavailable_reason: Option<&str>,
) {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled() && unavailable_reason.is_none();
    let text_color = if enabled {
        t.color.text
    } else {
        t.color.text_faint
    };
    let (row_rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width().max(1.0), SAFE_MODE_OPTION_HEIGHT),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let content_rect = safe_mode_option_content_rect(row_rect);
    let mut row = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(egui::Layout::left_to_right(Align::Center)),
    );
    row.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 11.0;
        let (check_rect, _) = ui.allocate_exact_size(Vec2::splat(20.0), Sense::hover());
        let box_rect = Rect::from_center_size(check_rect.center(), Vec2::splat(13.0));
        ui.painter().rect(
            box_rect,
            t.radius.min(2.0),
            if *checked {
                t.color.accent
            } else {
                t.color.bg_inset
            },
            Stroke::new(
                1.0,
                if *checked {
                    t.color.accent
                } else {
                    t.color.border_strong
                },
            ),
            egui::StrokeKind::Inside,
        );
        if *checked {
            Icon::Check.paint(ui.painter(), box_rect.shrink(2.0), t.color.accent_ink);
        }
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                egui::RichText::new(title)
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(text_color),
            );
            ui.label(
                egui::RichText::new(detail)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(if enabled {
                        t.color.text_dim
                    } else {
                        t.color.text_faint
                    }),
            );
        });
    });
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, enabled, *checked, title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(detail);
    });
    let response = if let Some(reason) = unavailable_reason {
        response.on_hover_text(reason)
    } else {
        response
    };
    theme::paint_focus_ring(ui, &response, response.rect);
    if enabled && response_activated(ui, &response) {
        *checked = !*checked;
    }
    ui.painter().hline(
        row_rect.x_range(),
        row_rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
}

pub(super) fn safe_mode_option_content_rect(row: Rect) -> Rect {
    row.shrink2(vec2(SAFE_MODE_OPTION_HORIZONTAL_INSET, 0.0))
}

pub(super) fn launcher_page_heading(
    ui: &mut Ui,
    eyebrow: &str,
    title: &str,
    detail: &str,
    layout: LauncherLayout,
) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .inner_margin(if layout.phone {
            Margin::symmetric(11, 9)
        } else {
            Margin::symmetric(16, 10)
        })
        .show(ui, |ui| {
            ui.set_height(
                (if layout.phone {
                    LAUNCHER_PHONE_HEADING_MIN_HEIGHT
                } else {
                    LAUNCHER_PAGE_HEADING_MIN_HEIGHT
                }) - if layout.phone { 18.0 } else { 20.0 },
            );
            ui.with_layout(
                egui::Layout::left_to_right(if layout.compact {
                    Align::Min
                } else {
                    Align::Center
                })
                .with_main_wrap(true),
                |ui| {
                    ui.spacing_mut().item_spacing = vec2(12.0, 3.0);
                    ui.label(
                        egui::RichText::new(eyebrow)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint)
                            .extra_letter_spacing(0.09 * tokens::FS_0),
                    );
                    let heading = ui.label(
                        egui::RichText::new(title)
                            .font(theme::sans(15.0, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.ctx().accesskit_node_builder(heading.id, |node| {
                        node.set_role(egui::accesskit::Role::Heading);
                        node.set_label(title);
                        node.set_level(3);
                    });
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(detail)
                                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                                .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                },
            );
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
}

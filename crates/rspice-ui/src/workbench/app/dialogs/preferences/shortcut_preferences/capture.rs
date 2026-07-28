//! Capturing a key chord from live input.
//!
//! A capture is only committed when the chord is complete — a bare modifier or
//! an in-progress multi-stroke sequence stays pending rather than binding
//! something the user did not finish typing. Escape always cancels and leaves
//! the previous binding intact, so entering capture can never lose a shortcut
//! by accident.

use super::*;

pub(super) fn key_capture_control(
    ui: &mut Ui,
    label: &str,
    accessible: &str,
    recording: bool,
    empty: bool,
    max_width: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let enabled = ui.is_enabled();
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let color = if empty {
        t.color.text_faint
    } else {
        t.color.text
    };
    let galley = ui.fonts_mut(|fonts| fonts.layout_no_wrap(label.to_owned(), font, color));
    let width = (galley.size().x + 14.0)
        .max(KEY_CAPTURE_MIN_WIDTH)
        .min(max_width.max(1.0));
    let height = if ui.ctx().content_rect().width() <= 560.0 {
        44.0
    } else {
        t.metrics.ctl_h.max(25.0)
    };
    let (rect, mut response) = ui.allocate_exact_size(
        egui::vec2(width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, accessible));
    let hover = ui.ctx().animate_bool_with_time(
        response.id,
        enabled && response.hovered(),
        ui.style().animation_time,
    );
    let fill = if recording {
        mix(t.color.bg_inset, t.color.accent_dim, 0.65)
    } else if response.is_pointer_button_down_on() {
        t.color.bg_active
    } else {
        mix(t.color.bg_inset, t.color.bg_hover, hover)
    };
    let opacity = if enabled { 1.0 } else { 0.4 };
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(2),
        fill.gamma_multiply(opacity),
    );
    let border_color = t.color.border_strong.gamma_multiply(opacity);
    if empty {
        paint_dashed_rect(ui.painter(), rect, border_color);
    } else {
        ui.painter().rect_stroke(
            rect,
            egui::CornerRadius::same(2),
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );
    }
    ui.painter().with_clip_rect(rect).galley(
        egui::pos2(
            rect.center().x - galley.size().x * 0.5,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        color.gamma_multiply(opacity),
    );
    if enabled {
        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    response
}

pub(super) fn paint_dashed_rect(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let stroke = Stroke::new(1.0, color);
    let step = 5.0;
    let dash = 3.0;
    let mut x = rect.left() + 1.0;
    while x < rect.right() - 1.0 {
        let end = (x + dash).min(rect.right() - 1.0);
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(end, rect.top())],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(x, rect.bottom()), egui::pos2(end, rect.bottom())],
            stroke,
        );
        x += step;
    }
    let mut y = rect.top() + 1.0;
    while y < rect.bottom() - 1.0 {
        let end = (y + dash).min(rect.bottom() - 1.0);
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.left(), end)],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(rect.right(), y), egui::pos2(rect.right(), end)],
            stroke,
        );
        y += step;
    }
}

pub(super) fn process_key_capture(ctx: &Context, editor: &mut ShortcutEditorState) {
    let Some(target) = editor.recording else {
        return;
    };
    let events = ctx.input(|input| {
        input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::Key {
                    key,
                    pressed: true,
                    repeat: false,
                    modifiers,
                    ..
                } => Some((*key, *modifiers)),
                _ => None,
            })
            .collect::<Vec<_>>()
    });

    for (key, modifiers) in events {
        if editor.recording != Some(target) {
            break;
        }
        ctx.input_mut(|input| {
            input.consume_key(modifiers, key);
        });
        if key == Key::Escape {
            cancel_capture(editor);
            return;
        }
        let no_modifiers = modifiers_are_empty(modifiers);
        if key == Key::Enter && no_modifiers && !editor.capture_strokes.is_empty() {
            finish_capture(editor, target);
            return;
        }
        if key == Key::Backspace && no_modifiers && editor.capture_strokes.is_empty() {
            clear_capture(editor, target);
            return;
        }
        editor.capture_strokes.push(ShortcutStroke::new(
            key,
            modifiers.command,
            modifiers.alt,
            modifiers.shift,
        ));
        editor.capture_last_input_at = Some(ctx.input(|input| input.time));
        if editor.capture_strokes.len() == MAX_SHORTCUT_SEQUENCE_STROKES {
            finish_capture(editor, target);
            return;
        }
    }

    if editor.recording != Some(target) {
        return;
    }

    let timeout = editor
        .draft
        .as_ref()
        .expect("editor draft")
        .policies()
        .chord_timeout()
        .seconds();
    if let (Some(timeout), Some(last)) = (timeout, editor.capture_last_input_at) {
        let now = ctx.input(|input| input.time);
        if !editor.capture_strokes.is_empty() && now - last >= timeout {
            finish_capture(editor, target);
        } else {
            ctx.request_repaint_after(Duration::from_secs_f64((timeout - (now - last)).max(0.01)));
        }
    }
}

pub(super) fn finish_capture(editor: &mut ShortcutEditorState, target: ShortcutCaptureTarget) {
    let strokes = std::mem::take(&mut editor.capture_strokes);
    let Ok(sequence) = ShortcutSequence::new(strokes) else {
        cancel_capture(editor);
        return;
    };
    let platforms = binding_platforms(
        editor.draft.as_ref().expect("editor draft"),
        target.command,
        target.slot,
    );
    match editor.draft.as_mut().expect("editor draft").set_binding(
        target.command,
        target.slot,
        platforms,
        Some(sequence),
    ) {
        Ok(()) => mark_editor_changed(editor),
        Err(error) => {
            editor.error_summary = Some(format!(
                "Could not record {} for {}: {error}",
                target.slot.label(),
                target.command.spec().label
            ))
        }
    }
    editor.recording = None;
    editor.capture_last_input_at = None;
}

pub(super) fn clear_capture(editor: &mut ShortcutEditorState, target: ShortcutCaptureTarget) {
    let platforms = binding_platforms(
        editor.draft.as_ref().expect("editor draft"),
        target.command,
        target.slot,
    );
    match editor.draft.as_mut().expect("editor draft").set_binding(
        target.command,
        target.slot,
        platforms,
        None,
    ) {
        Ok(()) => mark_editor_changed(editor),
        Err(error) => editor.error_summary = Some(error.to_string()),
    }
    cancel_capture(editor);
}

pub(super) fn cancel_capture(editor: &mut ShortcutEditorState) {
    editor.recording = None;
    editor.capture_strokes.clear();
    editor.capture_last_input_at = None;
}

pub(super) fn modifiers_are_empty(modifiers: Modifiers) -> bool {
    !modifiers.alt
        && !modifiers.ctrl
        && !modifiers.shift
        && !modifiers.mac_cmd
        && !modifiers.command
}

pub(super) fn binding_platforms(
    profile: &ShortcutPreferences,
    command: Command,
    slot: ShortcutBindingSlot,
) -> Vec<CommandPlatform> {
    if let Some(binding) = profile
        .resolved_bindings(command)
        .into_iter()
        .find(|binding| binding.slot() == slot)
    {
        return binding.platforms().to_vec();
    }
    match slot {
        ShortcutBindingSlot::Primary
            if command.primary_is_reserved_on(CommandPlatform::Browser) =>
        {
            vec![CommandPlatform::Desktop]
        }
        ShortcutBindingSlot::Primary => CommandPlatform::ALL.to_vec(),
        ShortcutBindingSlot::Alternate => vec![
            CommandPlatform::Browser,
            CommandPlatform::Tablet,
            CommandPlatform::Phone,
        ],
    }
}

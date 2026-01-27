//! Toolbar for egui Application
//!
//! Main toolbar with simulation controls, zoom, and tool selection.

use egui::{pos2, Color32, Rect, RichText, Stroke, Ui, Vec2};

use crate::common::app::AppState;
use crate::state::{ComponentType, Tool};

/// Standard font size for toolbar labels
const TOOLBAR_FONT_SIZE: f32 = 14.0;

// =============================================================================
// Icon Types and Vector Drawing
// =============================================================================

/// Icon types for toolbar buttons (drawn procedurally, not Unicode)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconType {
    // Tool icons
    Select,
    Wire,
    Component,
    Probe,
    // Zoom controls
    ZoomIn,
    ZoomOut,
    ZoomFit,
    // Simulation controls
    Play,
    Stop,
    Settings,
    // Status indicators
    Spinner,
    Checkmark,
    DirtyDot,
    // Icon rail icons
    Folder,
    Keyboard,
    Waveform,
    Shell,
}

/// Paint an icon into a given rect using the painter
pub fn paint_icon(ui: &mut Ui, rect: Rect, icon: IconType, color: Color32) {
    let painter = ui.painter();
    let stroke = Stroke::new(1.5, color);
    let thin_stroke = Stroke::new(1.0, color);
    let center = rect.center();

    // Shrink slightly for padding - proportional to rect size
    let inset = (rect.width().min(rect.height()) * 0.15).max(2.0);
    let inner = rect.shrink(inset);
    let w = inner.width();
    let h = inner.height();

    match icon {
        IconType::Select => {
            // Corner brackets for selection
            let corners = [
                inner.left_top(),
                inner.right_top(),
                inner.right_bottom(),
                inner.left_bottom(),
            ];
            let bracket_len = 4.0;
            for (i, &corner) in corners.iter().enumerate() {
                let next = corners[(i + 1) % 4];
                let prev = corners[(i + 3) % 4];
                let to_next = (next - corner).normalized() * bracket_len;
                let to_prev = (prev - corner).normalized() * bracket_len;
                painter.line_segment([corner, corner + to_next], stroke);
                painter.line_segment([corner, corner + to_prev], stroke);
            }
        }
        IconType::Wire => {
            // Polyline with bend
            let p1 = pos2(inner.left(), center.y);
            let p2 = pos2(center.x - 2.0, center.y);
            let p3 = pos2(center.x + 2.0, center.y - 4.0);
            let p4 = pos2(inner.right(), center.y - 4.0);
            painter.line_segment([p1, p2], stroke);
            painter.line_segment([p2, p3], stroke);
            painter.line_segment([p3, p4], stroke);
            painter.circle_filled(p1, 2.0, color);
            painter.circle_filled(p4, 2.0, color);
        }
        IconType::Component => {
            // Plus sign in a circle
            let r = w.min(h) / 2.0 - 1.0;
            painter.circle_stroke(center, r, stroke);
            let arm = r * 0.5;
            painter.line_segment(
                [
                    pos2(center.x - arm, center.y),
                    pos2(center.x + arm, center.y),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    pos2(center.x, center.y - arm),
                    pos2(center.x, center.y + arm),
                ],
                stroke,
            );
        }
        IconType::Probe => {
            // Oscilloscope-style probe: angled tip with grip
            // Probe body - diagonal shaft
            let tip = pos2(inner.left() + 2.0, inner.bottom() - 2.0);
            let shaft_end = pos2(inner.right() - 3.0, inner.top() + 3.0);
            painter.line_segment([tip, shaft_end], Stroke::new(2.0, color));

            // Probe tip - small filled circle
            painter.circle_filled(tip, 2.5, color);

            // Grip lines on handle
            let grip_start = shaft_end - egui::vec2(3.0, -3.0);
            painter.line_segment(
                [
                    pos2(grip_start.x - 1.5, grip_start.y - 1.5),
                    pos2(grip_start.x + 1.5, grip_start.y + 1.5),
                ],
                thin_stroke,
            );
            painter.line_segment(
                [
                    pos2(grip_start.x - 3.5, grip_start.y - 0.5),
                    pos2(grip_start.x - 0.5, grip_start.y + 2.5),
                ],
                thin_stroke,
            );

            // Small wire coming from probe
            painter.line_segment([shaft_end, pos2(inner.right(), inner.top())], thin_stroke);
        }
        IconType::ZoomIn => {
            // Magnifying glass with plus
            let r = w.min(h) * 0.3;
            let glass_center = pos2(center.x - 2.0, center.y - 2.0);
            painter.circle_stroke(glass_center, r, stroke);
            // Handle
            let handle_start = glass_center + egui::vec2(r * 0.7, r * 0.7);
            let handle_end = pos2(inner.right(), inner.bottom());
            painter.line_segment([handle_start, handle_end], Stroke::new(2.0, color));
            // Plus inside
            let arm = r * 0.5;
            painter.line_segment(
                [
                    pos2(glass_center.x - arm, glass_center.y),
                    pos2(glass_center.x + arm, glass_center.y),
                ],
                thin_stroke,
            );
            painter.line_segment(
                [
                    pos2(glass_center.x, glass_center.y - arm),
                    pos2(glass_center.x, glass_center.y + arm),
                ],
                thin_stroke,
            );
        }
        IconType::ZoomOut => {
            // Magnifying glass with minus
            let r = w.min(h) * 0.3;
            let glass_center = pos2(center.x - 2.0, center.y - 2.0);
            painter.circle_stroke(glass_center, r, stroke);
            let handle_start = glass_center + egui::vec2(r * 0.7, r * 0.7);
            let handle_end = pos2(inner.right(), inner.bottom());
            painter.line_segment([handle_start, handle_end], Stroke::new(2.0, color));
            // Minus inside
            let arm = r * 0.5;
            painter.line_segment(
                [
                    pos2(glass_center.x - arm, glass_center.y),
                    pos2(glass_center.x + arm, glass_center.y),
                ],
                thin_stroke,
            );
        }
        IconType::ZoomFit => {
            // Four corners with arrows pointing inward
            let corner_size = 4.0;
            let arrow_size = 2.5;
            // Top-left
            painter.line_segment(
                [
                    inner.left_top(),
                    pos2(inner.left() + corner_size, inner.top()),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    inner.left_top(),
                    pos2(inner.left(), inner.top() + corner_size),
                ],
                stroke,
            );
            // Top-right
            painter.line_segment(
                [
                    inner.right_top(),
                    pos2(inner.right() - corner_size, inner.top()),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    inner.right_top(),
                    pos2(inner.right(), inner.top() + corner_size),
                ],
                stroke,
            );
            // Bottom-left
            painter.line_segment(
                [
                    inner.left_bottom(),
                    pos2(inner.left() + corner_size, inner.bottom()),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    inner.left_bottom(),
                    pos2(inner.left(), inner.bottom() - corner_size),
                ],
                stroke,
            );
            // Bottom-right
            painter.line_segment(
                [
                    inner.right_bottom(),
                    pos2(inner.right() - corner_size, inner.bottom()),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    inner.right_bottom(),
                    pos2(inner.right(), inner.bottom() - corner_size),
                ],
                stroke,
            );
            // Center dot
            painter.circle_filled(center, arrow_size, color);
        }
        IconType::Play => {
            // Right-pointing triangle
            let tri_w = w;
            let tri_h = h;
            let points = vec![
                pos2(center.x - tri_w * 0.4, center.y - tri_h * 0.5),
                pos2(center.x + tri_w * 0.6, center.y),
                pos2(center.x - tri_w * 0.4, center.y + tri_h * 0.5),
            ];
            painter.add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
        }
        IconType::Stop => {
            // Filled square
            let sq_size = w.min(h);
            let sq_rect = Rect::from_center_size(center, egui::vec2(sq_size, sq_size));
            painter.rect_filled(sq_rect, 1.0, color);
        }
        IconType::Settings => {
            // Proper cogwheel gear icon
            let r = w.min(h) * 0.45; // Full size gear
            let teeth = 8;
            let tooth_height = r * 0.3;
            let tooth_width = std::f32::consts::TAU / (teeth as f32 * 3.0); // Width of each tooth

            // Build gear outline as a polygon
            let mut points = Vec::new();
            for i in 0..(teeth * 2) {
                let base_angle = (i as f32 / (teeth * 2) as f32) * std::f32::consts::TAU;
                let is_tooth = i % 2 == 0;
                let radius = if is_tooth { r } else { r - tooth_height };

                // Add two points for each segment (flat top/bottom)
                let angle1 = base_angle - tooth_width * 0.4;
                let angle2 = base_angle + tooth_width * 0.4;
                points.push(pos2(
                    center.x + angle1.cos() * radius,
                    center.y + angle1.sin() * radius,
                ));
                points.push(pos2(
                    center.x + angle2.cos() * radius,
                    center.y + angle2.sin() * radius,
                ));
            }

            // Draw gear body
            painter.add(egui::Shape::convex_polygon(
                points,
                Color32::TRANSPARENT,
                Stroke::new(1.8, color),
            ));

            // Center hole
            let hole_r = r * 0.25;
            painter.circle_stroke(center, hole_r, Stroke::new(1.8, color));
        }
        IconType::Spinner => {
            // Circular arc (static representation of spinner)
            let r = w.min(h) * 0.35;
            let arc_start = -std::f32::consts::FRAC_PI_4;
            let arc_sweep = std::f32::consts::PI * 1.5;
            let segments = 12;
            for i in 0..segments {
                let t0 = arc_start + (i as f32 / segments as f32) * arc_sweep;
                let t1 = arc_start + ((i + 1) as f32 / segments as f32) * arc_sweep;
                let p0 = pos2(center.x + t0.cos() * r, center.y + t0.sin() * r);
                let p1 = pos2(center.x + t1.cos() * r, center.y + t1.sin() * r);
                painter.line_segment([p0, p1], stroke);
            }
        }
        IconType::Checkmark => {
            // Checkmark
            let p1 = pos2(inner.left() + 2.0, center.y);
            let p2 = pos2(center.x - 1.0, inner.bottom() - 2.0);
            let p3 = pos2(inner.right() - 2.0, inner.top() + 2.0);
            painter.line_segment([p1, p2], Stroke::new(2.0, color));
            painter.line_segment([p2, p3], Stroke::new(2.0, color));
        }
        IconType::DirtyDot => {
            // Filled circle
            let r = w.min(h) * 0.3;
            painter.circle_filled(center, r, color);
        }
        IconType::Folder => {
            // Folder shape
            let folder_w = w * 0.8;
            let folder_h = h * 0.65;
            let tab_w = folder_w * 0.35;
            let tab_h = h * 0.15;
            let base_top = center.y - folder_h * 0.3;

            // Tab
            painter.line_segment(
                [
                    pos2(center.x - folder_w * 0.5, base_top),
                    pos2(center.x - folder_w * 0.5 + tab_w * 0.2, base_top - tab_h),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    pos2(center.x - folder_w * 0.5 + tab_w * 0.2, base_top - tab_h),
                    pos2(center.x - folder_w * 0.5 + tab_w, base_top - tab_h),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    pos2(center.x - folder_w * 0.5 + tab_w, base_top - tab_h),
                    pos2(center.x - folder_w * 0.5 + tab_w + tab_w * 0.2, base_top),
                ],
                stroke,
            );
            // Body rectangle
            let body = Rect::from_min_size(
                pos2(center.x - folder_w * 0.5, base_top),
                egui::vec2(folder_w, folder_h),
            );
            painter.rect_stroke(body, 1.0, stroke);
        }
        IconType::Keyboard => {
            // Rectangle with key dots
            let kb_w = w * 0.85;
            let kb_h = h * 0.55;
            let kb_rect = Rect::from_center_size(center, egui::vec2(kb_w, kb_h));
            painter.rect_stroke(kb_rect, 2.0, stroke);

            // Key grid (3x2)
            let key_spacing_x = kb_w / 4.0;
            let key_spacing_y = kb_h / 3.0;
            for row in 0..2 {
                for col in 0..3 {
                    let kx = kb_rect.left() + key_spacing_x * (col as f32 + 0.75);
                    let ky = kb_rect.top() + key_spacing_y * (row as f32 + 0.75);
                    painter.rect_filled(
                        Rect::from_center_size(pos2(kx, ky), egui::vec2(3.0, 2.5)),
                        0.5,
                        color,
                    );
                }
            }
        }
        IconType::Waveform => {
            // Sine wave
            let wave_w = w * 0.8;
            let wave_h = h * 0.4;
            let segments = 16;
            let start_x = center.x - wave_w * 0.5;
            for i in 0..segments {
                let t0 = (i as f32 / segments as f32) * std::f32::consts::TAU * 1.5;
                let t1 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU * 1.5;
                let x0 = start_x + (i as f32 / segments as f32) * wave_w;
                let x1 = start_x + ((i + 1) as f32 / segments as f32) * wave_w;
                let y0 = center.y - t0.sin() * wave_h;
                let y1 = center.y - t1.sin() * wave_h;
                painter.line_segment([pos2(x0, y0), pos2(x1, y1)], stroke);
            }
        }
        IconType::Shell => {
            // Terminal prompt: >_
            let prompt_x = center.x - w * 0.25;
            // >
            painter.line_segment(
                [
                    pos2(prompt_x, center.y - h * 0.2),
                    pos2(prompt_x + w * 0.2, center.y),
                ],
                Stroke::new(2.0, color),
            );
            painter.line_segment(
                [
                    pos2(prompt_x + w * 0.2, center.y),
                    pos2(prompt_x, center.y + h * 0.2),
                ],
                Stroke::new(2.0, color),
            );
            // _ (cursor)
            painter.line_segment(
                [
                    pos2(prompt_x + w * 0.3, center.y + h * 0.2),
                    pos2(prompt_x + w * 0.5, center.y + h * 0.2),
                ],
                Stroke::new(2.0, color),
            );
        }
    }
}

// =============================================================================
// Toolbar Rendering
// =============================================================================

/// Render the main toolbar
pub fn render_toolbar(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // =====================================================================
        // Tool Selection
        // =====================================================================
        ui.label(RichText::new("Tools:").size(TOOLBAR_FONT_SIZE));
        ui.add_space(4.0);

        // Select tool
        let is_select = state.schematic.tool.is_select();
        if icon_button(ui, IconType::Select, is_select)
            .on_hover_text("Select (S)")
            .clicked()
        {
            state.schematic.tool = Tool::Select;
        }

        // Wire tool
        let is_wire = state.schematic.tool.is_wire();
        if icon_button(ui, IconType::Wire, is_wire)
            .on_hover_text("Wire (W)")
            .clicked()
        {
            state.schematic.tool = Tool::Wire;
        }

        // Component dropdown - use icon_button for consistent sizing
        let is_place = state.schematic.tool.is_place_tool();
        let component_id = ui.make_persistent_id("component_menu");
        let component_response = icon_button(ui, IconType::Component, is_place);
        if component_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(component_id));
        }
        let hover_text = if is_place {
            format!("Placing: {}", state.schematic.tool.display_name())
        } else {
            "Add Component".to_string()
        };

        egui::popup_below_widget(
            ui,
            component_id,
            &component_response,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.set_min_width(140.0);
                ui.label("Components");
                ui.separator();

                if ui.button("Resistor (R+Shift)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::Resistor);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("Capacitor (C)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::Capacitor);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("Inductor (L)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::Inductor);
                    ui.memory_mut(|mem| mem.close_popup());
                }

                ui.separator();

                if ui.button("Voltage Source (V)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::VoltageSource);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("Current Source (I)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::CurrentSource);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("Ground (G)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::Ground);
                    ui.memory_mut(|mem| mem.close_popup());
                }

                ui.separator();

                if ui.button("Diode (D)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::Diode);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("NMOS (M)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::Nmos);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("PMOS").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::Pmos);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("NPN BJT (Q)").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::NpnBjt);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                if ui.button("PNP BJT").clicked() {
                    state.schematic.tool = Tool::Place(ComponentType::PnpBjt);
                    ui.memory_mut(|mem| mem.close_popup());
                }
            },
        );
        component_response.on_hover_text(hover_text);

        // Probe tool
        let is_probe = matches!(state.schematic.tool, Tool::Probe);
        if icon_button(ui, IconType::Probe, is_probe)
            .on_hover_text("Probe (P)")
            .clicked()
        {
            state.schematic.tool = Tool::Probe;
        }

        // Current tool indicator
        ui.separator();
        let tool_name = state.schematic.tool.display_name();
        ui.label(RichText::new(format!("Mode: {}", tool_name)).size(TOOLBAR_FONT_SIZE));

        ui.separator();

        // =====================================================================
        // Zoom Controls
        // =====================================================================
        ui.label(RichText::new("Zoom:").size(TOOLBAR_FONT_SIZE));
        ui.add_space(4.0);

        if icon_button(ui, IconType::ZoomOut, false)
            .on_hover_text("Zoom Out (Ctrl+-)")
            .clicked()
        {
            state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
        }

        // Zoom percentage display
        let zoom_pct = (state.schematic.zoom * 100.0) as i32;
        let zoom_text = format!("{}%", zoom_pct);
        ui.label(RichText::new(zoom_text).size(TOOLBAR_FONT_SIZE));

        if icon_button(ui, IconType::ZoomIn, false)
            .on_hover_text("Zoom In (Ctrl++)")
            .clicked()
        {
            state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
        }

        if icon_button(ui, IconType::ZoomFit, false)
            .on_hover_text("Zoom to Fit")
            .clicked()
        {
            state.schematic.zoom_to_fit(800.0, 600.0);
        }

        ui.separator();

        // =====================================================================
        // Simulation Controls
        // =====================================================================
        ui.label(RichText::new("Simulation:").size(TOOLBAR_FONT_SIZE));
        ui.add_space(4.0);

        // Run button - log once when components become available
        static LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !LOGGED.load(std::sync::atomic::Ordering::Relaxed)
            && !state.schematic.components.is_empty()
        {
            log::info!(
                "Run button now enabled: {} components",
                state.schematic.components.len()
            );
            LOGGED.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        let run_enabled = !state.schematic.components.is_empty() && !state.simulation.is_running;

        // Run button with dropdown for enabled analyses
        let run_id = ui.make_persistent_id("run_analyses_dropdown");
        let enabled_count = state.dialogs.enabled_analyses.len();

        // Create a custom button with dropdown arrow
        let button_text = if enabled_count > 0 {
            format!("Run ({})", enabled_count)
        } else {
            "Run".to_string()
        };

        let run_response = icon_text_button(ui, IconType::Play, &button_text, run_enabled);

        // Capture rect for popup positioning BEFORE consuming response
        let run_rect = run_response.rect;

        // Left-click runs
        if run_response.clicked() {
            log::info!(
                "Toolbar Run clicked! Components: {}",
                state.schematic.components.len()
            );
            state.simulation.trigger_simulation = true;
            state
                .console_messages
                .push(crate::common::app::ConsoleMessage::info(
                    "Simulation started...",
                ));
        }

        run_response
            .on_hover_text("Run Simulation")
            .on_disabled_hover_text(if state.simulation.is_running {
                "Simulation running"
            } else {
                "Add components first"
            });

        // Dropdown arrow button (separate small button)
        let dropdown_btn = ui.add_sized(
            [20.0, 24.0],
            egui::Button::new(RichText::new("▼").size(10.0)).min_size(egui::vec2(20.0, 24.0)),
        );

        if dropdown_btn.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(run_id));
        }

        dropdown_btn.on_hover_text("Show enabled analyses");

        // Dropdown popup showing enabled analyses
        let popup_open = ui.memory(|mem| mem.is_popup_open(run_id));
        if popup_open {
            egui::Area::new(run_id)
                .order(egui::Order::Foreground)
                .fixed_pos(run_rect.left_bottom())
                .show(ui.ctx(), |ui| {
                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                        ui.set_min_width(180.0);
                        ui.label(RichText::new("Enabled Analyses").strong().size(13.0));
                        ui.separator();

                        // Analysis name lookup
                        let analysis_names: &[(usize, &str)] = &[
                            (0, "DC Operating Point"),
                            (1, "Transient"),
                            (2, "AC Analysis"),
                            (3, "DC Sweep"),
                            (4, "Noise"),
                            (5, "Pole-Zero"),
                            (6, "Sensitivity"),
                            (7, "Monte Carlo"),
                            (8, "PSS"),
                            (9, "Stability (STB)"),
                            (10, "Temperature"),
                            (11, "Harmonic Balance"),
                            (12, "S-Parameter"),
                            (13, "PAC"),
                            (14, "PNoise"),
                            (15, "PXF"),
                            (16, "PSTB"),
                            (17, "Transfer Func"),
                            (18, "Corner"),
                            (19, "Envelope"),
                            (20, "Fourier"),
                            (21, "Reliability"),
                            (22, "Optimization"),
                            (23, "Safety (SOA)"),
                        ];

                        if enabled_count == 0 {
                            ui.label(RichText::new("No analyses enabled").weak().italics());
                            ui.add_space(4.0);
                        } else {
                            // Show enabled analyses with toggle checkboxes
                            for &(idx, name) in analysis_names {
                                let mut enabled = state.dialogs.enabled_analyses.contains(&idx);
                                if enabled {
                                    if ui.checkbox(&mut enabled, name).changed() {
                                        if enabled {
                                            state.dialogs.enabled_analyses.insert(idx);
                                        } else {
                                            state.dialogs.enabled_analyses.remove(&idx);
                                        }
                                    }
                                }
                            }
                        }

                        ui.separator();

                        // Quick actions
                        ui.horizontal(|ui| {
                            if ui.small_button("Setup...").clicked() {
                                state.dialogs.simulation_dialog = true;
                                ui.memory_mut(|mem| mem.close_popup());
                            }
                            if ui.small_button("Clear All").clicked() {
                                state.dialogs.enabled_analyses.clear();
                            }
                        });
                    });
                });

            // Close on click outside
            if ui.input(|i| i.pointer.any_click()) && !ui.memory(|mem| mem.is_popup_open(run_id)) {
                // Already closed
            } else if ui.input(|i| i.pointer.any_click()) {
                let popup_rect =
                    egui::Rect::from_min_size(run_rect.left_bottom(), egui::vec2(200.0, 300.0));
                if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                    if !popup_rect.contains(pos) && !run_rect.contains(pos) {
                        ui.memory_mut(|mem| mem.close_popup());
                    }
                }
            }
        }

        // Stop button
        let stop_enabled = state.simulation.is_running;
        if icon_text_button(ui, IconType::Stop, "Stop", stop_enabled)
            .on_hover_text("Stop Simulation")
            .clicked()
        {
            state.simulation.is_running = false;
            state
                .console_messages
                .push(crate::common::app::ConsoleMessage::warning(
                    "Simulation stopped",
                ));
        }

        // Setup button
        if icon_text_button(ui, IconType::Settings, "Setup", true)
            .on_hover_text("Simulation Setup")
            .clicked()
        {
            state.dialogs.simulation_dialog = true;
        }

        // =====================================================================
        // Status indicator (right aligned)
        // =====================================================================
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if state.simulation.is_running {
                status_indicator(
                    ui,
                    IconType::Spinner,
                    "Simulating...",
                    Color32::from_rgb(100, 200, 255),
                );
            } else if !state.simulation.waveforms.is_empty() {
                status_indicator(
                    ui,
                    IconType::Checkmark,
                    "Results ready",
                    Color32::from_rgb(100, 200, 100),
                );
            }

            // Dirty indicator
            if state.schematic.is_dirty {
                status_indicator(ui, IconType::DirtyDot, "", Color32::from_rgb(255, 180, 50));
            }
        });
    });
}

/// Create a tool button with custom vector icon and active highlighting
fn icon_button(ui: &mut Ui, icon: IconType, active: bool) -> egui::Response {
    let size = Vec2::splat(28.0);
    let fill = if active {
        Color32::from_rgb(60, 100, 160)
    } else {
        ui.visuals().widgets.inactive.bg_fill
    };

    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.visuals().widgets.style(&response);
        let rounding = visuals.rounding;
        let stroke = visuals.bg_stroke;

        ui.painter().rect(rect, rounding, fill, stroke);

        let icon_color = if active {
            Color32::WHITE
        } else {
            visuals.text_color()
        };
        paint_icon(ui, rect, icon, icon_color);
    }

    response
}

/// Create a button with both icon and text label
fn icon_text_button(ui: &mut Ui, icon: IconType, text: &str, enabled: bool) -> egui::Response {
    let icon_size = 20.0; // Larger icons for visibility
    let padding = 6.0;
    let font_size = 14.0; // Explicit font size for readability
    let text_galley = ui.fonts(|fonts| {
        fonts.layout_no_wrap(
            text.to_string(),
            egui::FontId::proportional(font_size),
            if enabled {
                ui.visuals().widgets.inactive.text_color()
            } else {
                ui.visuals().widgets.noninteractive.text_color()
            },
        )
    });
    let total_width = icon_size + padding + text_galley.size().x + padding * 2.0;
    let height = 28.0;

    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(total_width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );

    if ui.is_rect_visible(rect) {
        let visuals = if enabled {
            ui.visuals().widgets.style(&response)
        } else {
            &ui.visuals().widgets.noninteractive
        };
        let rounding = visuals.rounding;
        let fill = visuals.bg_fill;
        let stroke = visuals.bg_stroke;

        ui.painter().rect(rect, rounding, fill, stroke);

        // Draw icon on left
        let icon_rect = Rect::from_min_size(
            pos2(rect.left() + padding, rect.center().y - icon_size * 0.5),
            Vec2::splat(icon_size),
        );
        let icon_color = if enabled {
            visuals.text_color()
        } else {
            ui.visuals().widgets.noninteractive.text_color()
        };
        paint_icon(ui, icon_rect, icon, icon_color);

        // Draw text on right
        let text_pos = pos2(
            icon_rect.right() + padding,
            rect.center().y - text_galley.size().y * 0.5,
        );
        ui.painter().galley(text_pos, text_galley, Color32::WHITE);
    }

    response
}

/// Display a status indicator with icon and optional text
fn status_indicator(ui: &mut Ui, icon: IconType, text: &str, color: Color32) {
    let icon_size = 14.0;

    // Allocate space for icon
    let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(icon_size), egui::Sense::hover());

    if ui.is_rect_visible(icon_rect) {
        paint_icon(ui, icon_rect, icon, color);
    }

    // Add text label if provided
    if !text.is_empty() {
        ui.add_space(2.0);
        ui.colored_label(color, text);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_types_exist() {
        // Verify all IconType enum variants compile
        // Tool icons
        let _ = IconType::Select;
        let _ = IconType::Wire;
        let _ = IconType::Component;
        let _ = IconType::Probe;
        // Zoom controls
        let _ = IconType::ZoomIn;
        let _ = IconType::ZoomOut;
        let _ = IconType::ZoomFit;
        // Simulation controls
        let _ = IconType::Play;
        let _ = IconType::Stop;
        let _ = IconType::Settings;
        // Status indicators
        let _ = IconType::Spinner;
        let _ = IconType::Checkmark;
        let _ = IconType::DirtyDot;
        // Icon rail icons
        let _ = IconType::Folder;
        let _ = IconType::Keyboard;
        let _ = IconType::Waveform;
        let _ = IconType::Shell;
    }
}

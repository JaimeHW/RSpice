use egui::{Color32, Rect, Stroke, Ui, pos2};

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
    Results,
    PanelBottom,
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
        IconType::Results => {
            // Results icon: stack of horizontal lines with bullet points
            // Account for bullet offset in center calculation for visual balance
            let bullet_offset = 3.0; // Space for bullet points on left
            let line_w = w * 0.65; // Slightly narrower to fit bullets
            let line_spacing = h * 0.25;

            // Shift content right by half the bullet offset for visual centering
            let content_center_x = center.x + bullet_offset * 0.5;
            let start_x = content_center_x - line_w * 0.5;
            let end_x = content_center_x + line_w * 0.5;

            // Three horizontal lines representing analysis results
            for i in 0..3 {
                let y = center.y + (i as f32 - 1.0) * line_spacing;
                painter.line_segment([pos2(start_x, y), pos2(end_x, y)], stroke);

                painter.circle_filled(pos2(start_x - bullet_offset, y), 1.5, color);
            }
        }
        IconType::PanelBottom => {
            // Layout icon with bottom panel highlighted
            // Outer frame
            let frame = Rect::from_center_size(center, egui::vec2(w * 0.9, h * 0.75));
            painter.rect_stroke(frame, 1.0, stroke);

            // Separator line (approx 2/3 down)
            let split_y = frame.top() + frame.height() * 0.65;
            painter.line_segment(
                [pos2(frame.left(), split_y), pos2(frame.right(), split_y)],
                stroke,
            );

            // Fill bottom section to indicate it's the active part
            let bottom_rect = Rect::from_min_max(pos2(frame.left(), split_y), frame.right_bottom());
            // Use a slightly transparent fill for the "active" look
            let fill_color = color.linear_multiply(0.3);
            painter.rect_filled(bottom_rect, 0.0, fill_color);
        }
    }
}

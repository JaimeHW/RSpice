use super::error::SymbolError;
use super::types::{PathCommand, Symbol, SymbolPath};

/// Parse an SVG string into a Symbol
pub fn parse_svg(svg_data: &str) -> Result<Symbol, SymbolError> {
    // Preprocess SVG to remove broken filter references
    // Many Inkscape SVGs have filter:url(#filterXXX) references to non-existent filters
    // which causes usvg to skip those elements entirely
    let sanitized_svg = svg_data
        .replace(";filter:url(#filter5647)", "")
        .replace("filter:url(#filter5647);", "")
        .replace("filter:url(#filter5647)", "");

    // Parse using usvg
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(&sanitized_svg, &options)
        .map_err(|e| SymbolError::ParseError(format!("Failed to parse SVG: {}", e)))?;

    let mut paths = Vec::new();
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    // Extract paths from the tree
    extract_paths(
        tree.root(),
        &mut paths,
        &mut min_x,
        &mut min_y,
        &mut max_x,
        &mut max_y,
    );

    // Log number of paths found (before normalization)
    log::debug!(
        "Parsed SVG: {} paths, raw bounds=({:.1},{:.1},{:.1},{:.1})",
        paths.len(),
        min_x,
        min_y,
        max_x,
        max_y
    );

    // Normalize coordinates: translate so bounds start at (0,0)
    // This is necessary because usvg preserves SVG coordinates with transforms applied
    // center() uses bounds-based calculation, so paths must be normalized to match
    let offset_x = if min_x != f32::MAX { min_x } else { 0.0 };
    let offset_y = if min_y != f32::MAX { min_y } else { 0.0 };

    for path in &mut paths {
        for cmd in &mut path.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    *x -= offset_x;
                    *y -= offset_y;
                }
                PathCommand::LineTo(x, y) => {
                    *x -= offset_x;
                    *y -= offset_y;
                }
                PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    ctrl1.0 -= offset_x;
                    ctrl1.1 -= offset_y;
                    ctrl2.0 -= offset_x;
                    ctrl2.1 -= offset_y;
                    end.0 -= offset_x;
                    end.1 -= offset_y;
                }
                PathCommand::Close => {}
            }
        }
    }

    // Update bounds to reflect normalization
    let (norm_width, norm_height) = if min_x != f32::MAX {
        (max_x - offset_x, max_y - offset_y)
    } else {
        (40.0, 40.0) // Default fallback
    };

    // Get viewBox
    let view_box = tree.size();
    let vb = (0.0, 0.0, view_box.width(), view_box.height());

    // Scale paths to match viewBox coordinate space
    // usvg may extract paths at different DPI than viewBox dimensions
    let scale_x = if norm_width > 0.0 {
        vb.2 / norm_width
    } else {
        1.0
    };
    let scale_y = if norm_height > 0.0 {
        vb.3 / norm_height
    } else {
        1.0
    };

    log::info!(
        "SCALING PATHS: ({:.3}, {:.3}) - norm=({:.1}x{:.1}) viewBox=({:.1}x{:.1})",
        scale_x,
        scale_y,
        norm_width,
        norm_height,
        vb.2,
        vb.3
    );

    for path in &mut paths {
        for cmd in &mut path.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    *x *= scale_x;
                    *y *= scale_y;
                }
                PathCommand::LineTo(x, y) => {
                    *x *= scale_x;
                    *y *= scale_y;
                }
                PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    ctrl1.0 *= scale_x;
                    ctrl1.1 *= scale_y;
                    ctrl2.0 *= scale_x;
                    ctrl2.1 *= scale_y;
                    end.0 *= scale_x;
                    end.1 *= scale_y;
                }
                PathCommand::Close => {}
            }
        }
    }

    // Set final bounds to viewBox dimensions (paths are now in viewBox space)
    let final_bounds = (0.0, 0.0, vb.2, vb.3);

    log::debug!(
        "Final bounds (viewBox space): ({:.1},{:.1},{:.1},{:.1})",
        final_bounds.0,
        final_bounds.1,
        final_bounds.2,
        final_bounds.3
    );

    Ok(Symbol {
        name: String::new(),
        paths,
        pins: Vec::new(),
        bounds: final_bounds,
        view_box: vb,
        // Default target dimensions - will be overridden by load_embedded for known components
        target_width: 30.0,
        target_height: 30.0,
    })
}

/// Recursively extract paths from usvg tree (usvg 0.44 API)
/// IMPORTANT: usvg does NOT automatically apply group transforms to path data.
/// We must manually apply path.abs_transform() to get coordinates in document space.
fn extract_paths(
    group: &usvg::Group,
    paths: &mut Vec<SymbolPath>,
    min_x: &mut f32,
    min_y: &mut f32,
    max_x: &mut f32,
    max_y: &mut f32,
) {
    for node in group.children() {
        match node {
            usvg::Node::Path(path) => {
                let mut commands = Vec::new();
                let data = path.data();

                // Get the absolute transform (includes all parent group transforms)
                // This is REQUIRED because usvg preserves group transforms separately
                let t = path.abs_transform();

                // Transform helper: applies the 2D affine transformation
                let transform = |x: f32, y: f32| -> (f32, f32) {
                    (t.sx * x + t.kx * y + t.tx, t.ky * x + t.sy * y + t.ty)
                };

                for segment in data.segments() {
                    match segment {
                        usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            commands.push(PathCommand::MoveTo(x, y));
                        }
                        usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            commands.push(PathCommand::LineTo(x, y));
                        }
                        usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p) => {
                            let (x1, y1) = transform(p1.x, p1.y);
                            let (x2, y2) = transform(p2.x, p2.y);
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            update_bounds(x1, y1, min_x, min_y, max_x, max_y);
                            update_bounds(x2, y2, min_x, min_y, max_x, max_y);
                            commands.push(PathCommand::CurveTo {
                                ctrl1: (x1, y1),
                                ctrl2: (x2, y2),
                                end: (x, y),
                            });
                        }
                        usvg::tiny_skia_path::PathSegment::QuadTo(p1, p) => {
                            let (x1, y1) = transform(p1.x, p1.y);
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            update_bounds(x1, y1, min_x, min_y, max_x, max_y);
                            // Convert quadratic to cubic bezier approximation
                            commands.push(PathCommand::CurveTo {
                                ctrl1: (x1, y1),
                                ctrl2: (x1, y1),
                                end: (x, y),
                            });
                        }
                        usvg::tiny_skia_path::PathSegment::Close => {
                            commands.push(PathCommand::Close);
                        }
                    }
                }

                if !commands.is_empty() {
                    paths.push(SymbolPath {
                        commands,
                        filled: path.fill().is_some(),
                    });
                }
            }
            usvg::Node::Group(nested_group) => {
                extract_paths(nested_group, paths, min_x, min_y, max_x, max_y);
            }
            _ => {}
        }
    }
}

fn update_bounds(
    x: f32,
    y: f32,
    min_x: &mut f32,
    min_y: &mut f32,
    max_x: &mut f32,
    max_y: &mut f32,
) {
    *min_x = min_x.min(x);
    *min_y = min_y.min(y);
    *max_x = max_x.max(x);
    *max_y = max_y.max(y);
}

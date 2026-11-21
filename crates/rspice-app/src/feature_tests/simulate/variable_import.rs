//! The design variable import dialog drawn on the stage: every refused row is
//! inside the dialog that refuses it.

use super::workflow_dialogs;
use crate::workbench::RSpiceApp;
use rspice_app_simulate::workbench::surfaces::simulate::fixtures::{open, plan};

/// Every refused row is reachable inside the dialog it is refused in.
///
/// The table's scroll viewport was a hard 220 points regardless of the room
/// the split gave it, so a five-row sheet showed two rows over three hundred
/// points of blank dialog. The rows below the fold were the refusals, which
/// is what the import is being refused for — an operator was shown a
/// refusal count and none of the refusals.
///
/// Measured through the tick box each row publishes: it is the control the
/// row is reached by, and the one that has to be inside the surface.
#[test]
fn every_refused_row_is_inside_the_dialog_that_refuses_it() {
    const SHEET: &str = "name,quantity,expression,minimum,maximum,scope,description\n\
         RFB,resistance,47k,10k,100k,testbench,feedback resistor\n\
         CFB,capacitance,2.2p,,,testbench,feedback capacitor\n\
         RLOAD,resistance,1k,,,project,collides with the plan\n\
         IBIAS,current,banana,,,testbench,not a current\n\
         VREF,voltage,2.5V,3V,2V,testbench,inverted bounds\n";

    // The 1000-point gate as well as the two widths the panes sit side by
    // side at. Below the split the dialog stacks, the table follows the
    // column mapping down the body, and the body's own scroll is what
    // reaches it — a different layout, with its own way of leaving a
    // refusal below the fold, and the one this was not measuring.
    for width in [1000.0f32, 1600.0, 2560.0] {
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app.state.model);
        let rows = open(&app.state.model, &mut app.state.simulate, plan_id, SHEET)
            .rows
            .len();
        assert!(
            rows >= 5,
            "the fixture sheet must actually produce a table worth scrolling"
        );

        let mut run = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 1_000.0),
                    )),
                    ..Default::default()
                },
                |ctx| workflow_dialogs::show_workflow_dialogs_on_stage(ctx, &mut app),
            )
        };
        let _ = run();
        let _ = run();
        let output = run();

        // Measured against what is painted, not against what the tree
        // publishes: a row scrolled out of the table's viewport still has
        // AccessKit bounds, and it is exactly the row nobody can read.
        fn painted(
            shape: &egui::epaint::Shape,
            clip: egui::Rect,
            out: &mut Vec<(String, egui::Rect)>,
        ) {
            match shape {
                egui::epaint::Shape::Text(text) => {
                    let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
                    if clip.contains_rect(rect) {
                        out.push((text.galley.job.text.clone(), rect));
                    }
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        painted(shape, clip, out);
                    }
                }
                _ => {}
            }
        }

        let mut spans = Vec::new();
        for shape in &output.shapes {
            painted(&shape.shape, shape.clip_rect, &mut spans);
        }
        let missing: Vec<&str> = ["RFB", "CFB", "RLOAD", "IBIAS", "VREF"]
            .into_iter()
            .filter(|name| !spans.iter().any(|(text, _)| text == name))
            .collect();
        assert!(
            missing.is_empty(),
            "on a {width:.0}-point viewport the dialog refuses {rows} rows and paints none \
             of {missing:?}"
        );

        // And what it paints is inside the surface it is painted on.
        let surface = output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("AccessKit tree update")
            .nodes
            .iter()
            .find(|(_, node)| node.role() == egui::accesskit::Role::Dialog)
            .and_then(|(_, node)| node.bounds())
            .expect("the dialog publishes its bounds");
        for (text, rect) in &spans {
            if !["RFB", "CFB", "RLOAD", "IBIAS", "VREF"].contains(&text.as_str()) {
                continue;
            }
            assert!(
                f64::from(rect.bottom()) <= surface.y1 + 0.5
                    && f64::from(rect.top()) >= surface.y0 - 0.5,
                "{text} is painted at {:.0}..{:.0}, outside a surface spanning {:.0}..{:.0}",
                rect.top(),
                rect.bottom(),
                surface.y0,
                surface.y1
            );
        }
    }
}

//! A form waiting for a trace the reader clicks in the strip.
//!
//! Add measurement's Pick steps the form aside and arms the strip; the next
//! curve or legend entry the reader clicks fills the argument, and the form
//! comes back. Any other form, or none, leaves the strip's clicks to its
//! cursors.

use rspice_results_ui::document_dialogs::measurement::{
    MeasurementDraft, MeasurementPreview, fill_picked,
};
use rspice_results_ui::session::TracePick;

use super::super::ResultsParts;

/// Fill `draft` from a trace the strip took since the last frame.
pub(super) fn follow(
    state: &mut ResultsParts<'_>,
    draft: &mut MeasurementDraft,
    preview: &MeasurementPreview,
) {
    if let TracePick::Picked { source, name } =
        std::mem::take(&mut state.results.session.trace_pick)
    {
        fill_picked(draft, preview, &source, &name);
    }
}

/// Arm the strip while a form waits for a trace, and disarm it otherwise.
pub(super) fn arm(state: &mut ResultsParts<'_>, waiting: bool) {
    let pick = &mut state.results.session.trace_pick;
    if !waiting {
        *pick = TracePick::Off;
    } else if *pick == TracePick::Off {
        *pick = TracePick::Armed;
    }
}

#[cfg(test)]
mod tests {
    use rspice_results::calculator::catalogue;
    use rspice_results_ui::document_dialogs::measurement::MeasurementDraft;
    use rspice_results_ui::session::TracePick;

    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::{ResultsTestHost, waves};
    use rspice_results_ui::document_dialogs::ResultDocumentDialog;

    const FRAME: egui::Vec2 = egui::vec2(1200.0, 760.0);

    /// One frame of the strip with the open form over it, as the workbench
    /// draws them.
    fn frame(
        ctx: &egui::Context,
        host: &mut ResultsTestHost,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
            events,
            ..Default::default()
        };
        ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                waves::show(ui, &mut host.results_parts());
            });
            super::super::show(ctx, &mut host.results_parts());
        })
    }

    fn click(ctx: &egui::Context, host: &mut ResultsTestHost, at: egui::Pos2) {
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        frame(ctx, host, vec![egui::Event::PointerMoved(at)]);
        frame(ctx, host, vec![button(true)]);
        frame(ctx, host, vec![button(false)]);
    }

    /// Where the frame drew the widget spoken as `label`.
    fn spoken(output: &egui::FullOutput, label: &str) -> Option<egui::Rect> {
        output
            .platform_output
            .accesskit_update
            .as_ref()?
            .nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .and_then(|(_, node)| node.bounds())
            .map(|bounds| {
                egui::Rect::from_min_max(
                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                )
            })
    }

    /// The plot area: what the strip's full-height vertical grid rules span.
    fn plot_area(output: &egui::FullOutput) -> egui::Rect {
        fn walk(shape: &egui::Shape, area: &mut egui::Rect) {
            match shape {
                egui::Shape::LineSegment { points, .. }
                    if (points[0].x - points[1].x).abs() < 1e-3
                        && (points[0].y - points[1].y).abs() > 200.0 =>
                {
                    *area = area.union(egui::Rect::from_two_pos(points[0], points[1]));
                }
                egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, area)),
                _ => {}
            }
        }
        let mut area = egui::Rect::NOTHING;
        for clipped in &output.shapes {
            walk(&clipped.shape, &mut area);
        }
        assert!(area.is_positive(), "the strip drew no plot");
        area
    }

    fn draft(host: &ResultsTestHost) -> MeasurementDraft {
        match host.results.result_document_dialog.clone() {
            Some(ResultDocumentDialog::Measurement(draft)) => draft,
            other => panic!("the measurement form is open: {other:?}"),
        }
    }

    /// Pick on a signal argument steps the form aside, and the curve the
    /// reader then clicks in the strip fills that argument and brings the
    /// form back with the call rewritten, as ViVA's calculator takes a
    /// waveform clicked in the graph. Until then the click moves no cursor.
    #[test]
    fn a_trace_clicked_in_the_strip_fills_the_signal_the_form_is_picking() {
        let mut host = ResultsTestHost::default();
        host.workbench.workspace = crate::workbench::state::Workspace::Results;
        host.model.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                RetainedTrace::new("V(in)", vec![0.0, 1e-6], vec![0.0, 0.0], ""),
                RetainedTrace::new("V(out)", vec![0.0, 1e-6], vec![1.0, 1.0], ""),
            ]),
        );
        host.model.simulation.complete_run();
        *host.results_parts().result_document_dialog =
            Some(ResultDocumentDialog::Measurement(MeasurementDraft {
                expression: "rms(V(in))".to_owned(),
                function: catalogue::find("rms"),
                arguments: vec!["V(in)".to_owned()],
                ..MeasurementDraft::default()
            }));
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        frame(&ctx, &mut host, Vec::new());
        let output = frame(&ctx, &mut host, Vec::new());
        let pick = spoken(&output, "Pick").expect("a signal argument offers Pick");
        click(&ctx, &mut host, pick.center());

        // The prompt's first frame only sizes it.
        frame(&ctx, &mut host, Vec::new());
        let output = frame(&ctx, &mut host, Vec::new());
        assert_eq!(draft(&host).picking, Some(0));
        assert_eq!(host.results.results.session.trace_pick, TracePick::Armed);
        assert!(
            spoken(&output, "Pick").is_none(),
            "the form steps aside while the reader picks"
        );
        let back = spoken(&output, "Back to form").expect("the prompt offers the way back");
        // The prompt sits at the strip's foot, clear of the plot and of the
        // legend above it, which a click can also take.
        let area = plot_area(&output);
        assert!(
            back.top() > area.bottom(),
            "the prompt {back:?} covers the plot {area:?}"
        );

        // V(out) sits at the top of the plot, V(in) at its foot.
        let cursors = host.results.results.session.cursors;
        click(
            &ctx,
            &mut host,
            egui::pos2(area.center().x, area.top() + area.height() * 0.1),
        );
        let output = frame(&ctx, &mut host, Vec::new());
        let filled = draft(&host);
        // rms's window ends are optional, and stay empty.
        assert_eq!(filled.arguments, ["V(out)", "", ""]);
        assert_eq!(filled.expression, "rms(V(out))");
        assert_eq!(filled.picking, None);
        assert_eq!(host.results.results.session.trace_pick, TracePick::Off);
        assert_eq!(
            host.results.results.session.cursors, cursors,
            "the pick moved a cursor"
        );
        assert!(spoken(&output, "Pick").is_some(), "the form is back");
    }
}

//! The transfer-function form: the two ports of the transfer, what the design
//! itself offers for them, and which derived quantities the run reports.
//!
//! The ports are the same two quantities the noise form asks for — one
//! independent source in, one measured expression out — so they are offered
//! from the same elaborated vocabulary rather than typed blind.

use egui::Ui;

use crate::services::simulation_runner::TfRunConfig;
use crate::simulation::dialog::XfDialogState;

use super::{
    NOISE_DOMAIN_PRESET_LIMIT, NOISE_INPUT_CUSTOM_CHOICE, NOISE_OUTPUT_CUSTOM_CHOICE, NoiseDomain,
    action_line, action_line_enabled, choice_row, enabled_choice_row, field_advisory, field_note,
    noise_domain_advisory, noise_domain_hint, noise_domain_row, property_row,
};

pub(super) const XF_FIELD_LABELS: [&str; 8] = [
    "Input source",
    "Output expression",
    "Solve point",
    "Transfer gain",
    "Input resistance",
    "Output resistance",
    "Normalize",
    "Accuracy",
];

pub(super) const XF_SOLVE_POINT: &str = "DC operating point";

/// The action that fills the two fields above it from the design's own deck.
pub(super) const XF_INFER_LABEL: &str = "Infer from deck";

pub(super) const XF_NORMALIZATION_CHOICES: &[&str] =
    &["Disabled", "Relative to nominal", "Per source unit"];

pub(super) const XF_ACCURACY_CHOICES: &[&str] = &["Fast", "Balanced", "Accurate", "Robust"];

/// The output presets the transfer-function form offers: every elaborated
/// node, spelled as the voltage measured there.
///
/// The field takes an expression, not a node — `validate_output_expression`
/// admits `V(node)`, `V(node,ref)` and `I(element)` and nothing else — so a
/// picker that offered bare node names would be one whose every choice fails
/// validation. A differential pair or a branch current is typed through the
/// exact-expression escape beside the selector.
pub(super) fn xf_output_presets(nodes: &[String]) -> Vec<String> {
    nodes
        .iter()
        .take(NOISE_DOMAIN_PRESET_LIMIT)
        .map(|node| format!("V({node})"))
        .collect()
}

/// The design's own answer to the transfer-function form's two ports.
///
/// A pre-fill, not a run. Pressing this writes the deck's only independent
/// source and the node that source does not connect to into the two fields
/// above, where both stay editable and the reader still presses Run — which is
/// the whole reason a positional guess at the output is acceptable here and was
/// not acceptable in the PAC, PXF and PNOISE runners that ran on one.
///
/// When the deck names no single obvious pair the action is offered and
/// refused, with the reason in its place: a button whose only answer is a
/// refusal teaches nothing, and a button that has silently vanished teaches
/// less.
///
/// The note retires itself. It says what the deck offers only while the form
/// does not already say it, so a reader who has pressed the action — or typed
/// the same two names — is not told a third time.
pub(super) fn xf_inference_action(
    ui: &mut Ui,
    setup: &mut crate::simulation::dialog::XfDialogState,
    inference: Option<&Result<TfRunConfig, String>>,
) {
    // `None` is "not measured", which only a caller that painted this form
    // without resolving a design can produce. There is nothing honest to say
    // about a deck nobody read.
    let Some(inference) = inference else {
        return;
    };
    match inference {
        Ok(config) => {
            if action_line(ui, XF_INFER_LABEL) {
                setup.input_source.clone_from(&config.input_source);
                setup
                    .output_expression
                    .clone_from(&config.output_expression);
            }
            if setup.input_source != config.input_source
                || setup.output_expression != config.output_expression
            {
                field_note(
                    ui,
                    &format!(
                        "This design offers input {} and output {}.",
                        config.input_source, config.output_expression
                    ),
                );
            }
        }
        Err(reason) => {
            action_line_enabled(ui, XF_INFER_LABEL, false);
            field_advisory(ui, reason);
        }
    }
}

/// Render the transfer-function fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut XfDialogState,
    noise_domain: NoiseDomain<'_>,
    tf_inference: Option<&Result<TfRunConfig, String>>,
) {
    // The two ports are the same two quantities the noise form asks
    // for — one independent source in, one measured expression out —
    // so they are offered from the same elaborated vocabulary the
    // noise rows read rather than typed blind against a design the
    // form already knows. A name this design does not carry is not a
    // suggestion; it is a run that fails at validation.
    let offered_sources = noise_domain
        .sources
        .get(..NOISE_DOMAIN_PRESET_LIMIT)
        .unwrap_or(noise_domain.sources);
    noise_domain_row(
        ui,
        XF_FIELD_LABELS[0],
        "xf-input",
        &noise_domain_hint(
            "sources",
            offered_sources.len(),
            noise_domain.sources.len(),
            noise_domain.unavailable.is_some(),
        ),
        offered_sources,
        NOISE_INPUT_CUSTOM_CHOICE,
        &mut setup.input_source,
    );
    let offered_outputs = xf_output_presets(noise_domain.nodes);
    noise_domain_row(
        ui,
        XF_FIELD_LABELS[1],
        "xf-output",
        &noise_domain_hint(
            "nodes",
            offered_outputs.len(),
            noise_domain.nodes.len(),
            noise_domain.unavailable.is_some(),
        ),
        &offered_outputs,
        NOISE_OUTPUT_CUSTOM_CHOICE,
        &mut setup.output_expression,
    );
    if let Some(reason) = noise_domain.unavailable {
        noise_domain_advisory(ui, reason);
    }
    xf_inference_action(ui, setup, tf_inference);
    property_row(ui, XF_FIELD_LABELS[2], XF_SOLVE_POINT);
    enabled_choice_row(ui, XF_FIELD_LABELS[3], &mut setup.transfer_gain);
    enabled_choice_row(ui, XF_FIELD_LABELS[4], &mut setup.input_resistance);
    enabled_choice_row(ui, XF_FIELD_LABELS[5], &mut setup.output_resistance);
    choice_row(
        ui,
        XF_FIELD_LABELS[6],
        XF_NORMALIZATION_CHOICES,
        &mut setup.normalization_idx,
    );
    choice_row(
        ui,
        XF_FIELD_LABELS[7],
        XF_ACCURACY_CHOICES,
        &mut setup.accuracy_idx,
    );
}

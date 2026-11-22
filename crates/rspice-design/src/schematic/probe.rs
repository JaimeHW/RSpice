//! Durable schematic probe flags.
//!
//! A probe flag is authored canvas intent: it retains where the user placed
//! the crosshair and, when resolved, the exact raw output expression it owns.
//! Unbound flags deliberately carry no fabricated electrical expression.

use serde::{Deserialize, Serialize};

use rspice_design_model::Point;

fn enabled_by_default() -> bool {
    true
}

pub const MAX_SCHEMATIC_PROBE_REFERENCE_LEN: usize = 256;
const PROBE_CROSSHAIR_HALF_SPAN: i32 = 13;
const PROBE_LABEL_X_OFFSET: i32 = 13;
const PROBE_LABEL_ASCENT: i32 = 22;
const PROBE_LABEL_ADVANCE: i32 = 7;
/// The advance of one label character as the canvas draws it: monospace at
/// the small text size.
const PROBE_LABEL_GLYPH_ADVANCE: f32 = 6.7 * super::label_layout::SMALL_TEXT_WORLD / 11.0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicProbe {
    pub id: u64,
    pub position: Point,
    pub reference: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_expression: Option<String>,
    /// Plan/output identity that makes this marker an executable output
    /// request rather than a decorative annotation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<rspice_vocabulary::product::SimulationPlanId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_output_id: Option<rspice_vocabulary::product::SavedOutputId>,
    /// Disabled probes remain authored on the sheet but do not request future
    /// materialization. Legacy probes migrate enabled.
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    /// Whether a newly materialized quantity should be shown initially.
    #[serde(default = "enabled_by_default")]
    pub plot_on_materialization: bool,
}

impl SchematicProbe {
    pub fn new(
        id: u64,
        position: Point,
        reference: impl Into<String>,
        source_expression: Option<String>,
    ) -> Result<Self, String> {
        let probe = Self {
            id,
            position,
            reference: reference.into(),
            source_expression,
            plan_id: None,
            saved_output_id: None,
            enabled: true,
            plot_on_materialization: true,
        };
        probe.validate()?;
        Ok(probe)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id == 0 {
            return Err("probe identity must not be zero".to_owned());
        }
        let reference = self.reference.trim();
        if reference.is_empty() {
            return Err("probe reference is required".to_owned());
        }
        if reference.chars().count() > MAX_SCHEMATIC_PROBE_REFERENCE_LEN {
            return Err(format!(
                "probe reference exceeds {MAX_SCHEMATIC_PROBE_REFERENCE_LEN} characters"
            ));
        }
        if reference.chars().any(char::is_control) {
            return Err("probe reference contains a control character".to_owned());
        }
        if let Some(expression) = &self.source_expression {
            let expression = expression.trim();
            rspice_vocabulary::raw_probe::validate_raw_probe(expression)
                .map_err(|error| format!("probe source expression is invalid: {error}"))?;
        }
        Ok(())
    }

    pub fn bind_saved_output(
        &mut self,
        plan_id: rspice_vocabulary::product::SimulationPlanId,
        saved_output_id: rspice_vocabulary::product::SavedOutputId,
    ) {
        self.plan_id = Some(plan_id);
        self.saved_output_id = Some(saved_output_id);
    }

    /// Conservative authored-space bounds used for fit, culling and
    /// hardcopy policy, where a box too large costs nothing. The canvas font
    /// scales with the drawing, so its advance is stable in schematic
    /// coordinates.
    #[must_use]
    pub fn world_bounds(&self) -> (Point, Point) {
        // Status copy (for example, "pending next run") is painted beside
        // the reference. Reserve its maximum advance so fit/culling cannot
        // clip a probe merely because its runtime state changed.
        let label_characters =
            i32::try_from(self.reference.chars().count().saturating_add(22)).unwrap_or(i32::MAX);
        let label_width = label_characters.saturating_mul(PROBE_LABEL_ADVANCE);
        (
            Point::new(
                self.position.x.saturating_sub(PROBE_CROSSHAIR_HALF_SPAN),
                self.position.y.saturating_sub(PROBE_LABEL_ASCENT),
            ),
            Point::new(
                self.position
                    .x
                    .saturating_add(PROBE_LABEL_X_OFFSET)
                    .saturating_add(label_width),
                self.position.y.saturating_add(PROBE_CROSSHAIR_HALF_SPAN),
            ),
        )
    }

    /// The bounds of what the canvas draws: the crosshair, and beside it a
    /// label `label_characters` long (the reference, with any status the
    /// host paints after it).
    #[must_use]
    pub fn drawn_bounds(&self, label_characters: usize) -> (Point, Point) {
        let label_width = (label_characters as f32 * PROBE_LABEL_GLYPH_ADVANCE).ceil() as i32;
        (
            Point::new(
                self.position.x.saturating_sub(PROBE_CROSSHAIR_HALF_SPAN),
                self.position.y.saturating_sub(PROBE_LABEL_ASCENT),
            ),
            Point::new(
                self.position
                    .x
                    .saturating_add(PROBE_LABEL_X_OFFSET)
                    .saturating_add(label_width)
                    .max(self.position.x.saturating_add(PROBE_CROSSHAIR_HALF_SPAN)),
                self.position.y.saturating_add(PROBE_CROSSHAIR_HALF_SPAN),
            ),
        )
    }

    /// The crosshair and its reference as drawn, whatever status a run
    /// paints after them: what selection layout and window selection read,
    /// so probes align by what is seen and a snug window takes one.
    #[must_use]
    pub fn layout_bounds(&self) -> (Point, Point) {
        self.drawn_bounds(self.reference.chars().count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_identity_and_optional_binding_fail_closed() {
        assert!(SchematicProbe::new(0, Point::origin(), "P1", None).is_err());
        assert!(SchematicProbe::new(1, Point::origin(), " ", None).is_err());
        assert!(SchematicProbe::new(1, Point::origin(), "V(out)", Some(" ".to_owned())).is_err());
        assert_eq!(
            SchematicProbe::new(1, Point::new(20, 30), "V(out)", Some("V(out)".to_owned()))
                .unwrap()
                .source_expression
                .as_deref(),
            Some("V(out)")
        );
    }

    #[test]
    fn legacy_probe_migrates_enabled_with_plot_intent() {
        let probe = SchematicProbe::new(7, Point::new(20, 30), "V(out)", Some("V(out)".to_owned()))
            .expect("probe");
        let mut wire = serde_json::to_value(probe).expect("serialize");
        let object = wire.as_object_mut().expect("object");
        object.remove("plan_id");
        object.remove("saved_output_id");
        object.remove("enabled");
        object.remove("plot_on_materialization");

        let restored: SchematicProbe = serde_json::from_value(wire).expect("legacy probe");
        assert!(restored.enabled);
        assert!(restored.plot_on_materialization);
        assert!(restored.plan_id.is_none());
        assert!(restored.saved_output_id.is_none());
    }
}

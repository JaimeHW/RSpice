//! Immutable family source selection, policy validation, and commits.
use super::*;
use rspice_results_ui::studio::dock::family::{self as presentation, FamilyDraft, FamilyHost};
struct Host<'a>(&'a mut RSpiceApp);
pub(super) fn family_slice_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::slice(ui, &mut Host(app))
}
pub(super) fn family_encoding_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::encoding(ui, &mut Host(app))
}
pub(super) fn family_filter_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::filter(ui, &mut Host(app))
}
impl FamilyHost for Host<'_> {
    fn draft(&mut self) -> FamilyDraft<'_> {
        let s = &mut self.0.state.workbench.visualization_studio;
        FamilyDraft {
            x: &mut s.draft_family_x_dimension,
            family: &mut s.draft_family_dimension,
            color: &mut s.draft_family_color_dimension,
            dash: &mut s.draft_family_dash_dimension,
            marker: &mut s.draft_family_marker_dimension,
            query: &mut s.family_query,
            exclude_missing: &mut s.draft_family_exclude_missing,
        }
    }
    fn manifest(&self) -> Result<FamilyManifest, String> {
        active_family_manifest(self.0)
    }
    fn matching_indices(&self, manifest: &FamilyManifest) -> Result<Vec<usize>, String> {
        let query = &self.0.state.workbench.visualization_studio.family_query;
        manifest.matching_source_indices(query)
    }
    fn trace_count(&self) -> usize {
        let app = &self.0;
        app.state
            .simulation
            .active_analysis()
            .map_or(0, |analysis| analysis.waveforms.len())
    }
    fn validate(&self, manifest: &FamilyManifest, indices: Vec<usize>) -> Result<(), String> {
        let app = &self.0;
        if indices.is_empty() {
            return Err("The current filter selects no retained family points.".to_owned());
        }
        let policy = build_family_policy_draft(app, manifest)?;
        SourceSampleSelection::new(DatasetId::new(), 0, indices)?
            .with_family_presentation(manifest, &policy)?;
        Ok::<_, String>(())
    }
    fn apply(&mut self) {
        apply_family_policy_draft(self.0);
    }
}

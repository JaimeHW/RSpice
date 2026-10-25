"""Regression tests for the rspice-app slice coupling ratchet."""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from check_app_slices import (  # noqa: E402
    CEILINGS,
    GROUPS,
    SLICE_RULES,
    SLICES,
    compare,
    edge_group,
    measure,
    slice_of,
)


def measure_tree(files: dict[str, str], lower_model: dict[str, str] | None = None):
    with tempfile.TemporaryDirectory() as directory:
        trees = {"app": files, "state": lower_model or {}}
        for tree, contents in trees.items():
            for path, text in contents.items():
                target = Path(directory) / tree / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(text, encoding="utf-8")
        lower_crates = {"model": Path(directory) / "state"} if lower_model else None
        return measure(Path(directory) / "app", lower_crates)


PROJECT = "pub fn project_title() {}\n"


class SliceRuleTests(unittest.TestCase):
    def test_sample_paths_land_in_their_slice(self) -> None:
        expected = {
            "model": ["state/schematic/state.rs", "workbench/preferences.rs", "simulation/table_route.rs",
                      "workbench/app/schematic/edit_authority.rs", "state/stimulus_library.rs",
                      "workbench/app_state/run_identity.rs"],
            "services": ["io/file_exchange.rs", "workbench/workflows/export_workflow.rs",
                         "workbench/lifecycle/project_lifecycle.rs", "workbench/app_state/session/autosave.rs",
                         "build_identity.rs"],
            "run": ["simulation/controller.rs", "workbench/app_state/run_preflight.rs"],
            "framework": ["workbench/chrome/toolbar.rs", "workbench/documents.rs", "workbench/docks/navigator.rs",
                          "workbench/framework/requests.rs"],
            "session": ["workbench/app_state.rs", "workbench/state.rs", "workbench/app/dialogs/state.rs",
                        "workbench/app_state/session/session_store.rs"],
            "design": ["schematic/view/interaction.rs", "schematic/view/sheet_visibility.rs",
                       "schematic/view/drawing_sheet.rs",
                       "schematic/symbol_editor/canvas.rs", "workbench/surfaces/stimulus.rs"],
            "code": ["workbench/surfaces/netlist.rs", "workbench/documents/code_workspace.rs"],
            "simulate": ["workbench/surfaces/verify.rs", "workbench/tools/jobs_manager.rs"],
            "results": ["workbench/menu_bar/waveform_export.rs", "analysis/calculator.rs",
                        "workbench/docks/inspector/run_details.rs"],
            "models": ["workbench/surfaces/models.rs", "properties/model_browser.rs"],
            "project": ["workbench/surfaces/project.rs", "workbench/app/actions/file.rs"],
            "sheets": ["workbench/chrome/sheet_strip.rs", "workbench/app/drawing_sheet_edit.rs"],
            "hardcopy": ["workbench/hardcopy_adapters/sources.rs", "workbench/menu_bar/export_actions.rs"],
            "cloud": ["services/cloud_account.rs", "workbench/live_session.rs"],
            "settings": ["workbench/shortcuts/profile.rs", "workbench/app/dialogs/preferences.rs"],
            "shell": ["lib.rs", "workbench/app.rs", "workbench/app/dialogs/help.rs",
                      "workbench/app_state/session/preference_runtime.rs"],
        }
        self.assertEqual(set(expected), set(SLICES))
        for slice_name, paths in expected.items():
            for path in paths:
                self.assertEqual(slice_of(path), slice_name, path)

    def test_an_unmatched_path_has_no_slice_and_every_rule_names_a_known_slice(self) -> None:
        self.assertIsNone(slice_of("brand_new_module.rs"))
        self.assertEqual({slice_name for _, slice_name in SLICE_RULES} - set(SLICES), set())

    def test_only_peer_features_and_upward_lower_references_are_ratcheted(self) -> None:
        self.assertEqual(edge_group("design", "code"), "feature->feature")
        self.assertEqual(edge_group("framework", "results"), "lower->upper")
        self.assertEqual(edge_group("framework", "shell"), "lower->upper")
        self.assertEqual(edge_group("model", "session"), "lower->upper")
        self.assertEqual(edge_group("services", "session"), "lower->upper")
        self.assertEqual(edge_group("services", "run"), "lower->upper")
        self.assertEqual(edge_group("run", "framework"), "lower->upper")
        self.assertIsNone(edge_group("framework", "run"))
        self.assertIsNone(edge_group("services", "model"))
        self.assertIsNone(edge_group("design", "shell"))
        self.assertIsNone(edge_group("design", "model"))


class MeasurementTests(unittest.TestCase):
    def test_a_feature_reaching_another_feature_is_counted(self) -> None:
        values, contributors, _ = measure_tree({
            "workbench/surfaces/project.rs": PROJECT,
            "workbench/surfaces/results.rs": (
                "use crate::workbench::surfaces::project::project_title;\n"
                "pub fn show() { project_title(); }\n"
            ),
        })
        # The import and the call each count.
        self.assertEqual(values["feature->feature"], {"results->project": 2})
        self.assertEqual(
            dict(contributors[("feature->feature", "results->project")]),
            {"workbench/surfaces/results.rs": 2},
        )

    def test_test_modules_and_test_only_files_are_not_production_coupling(self) -> None:
        values, _, stats = measure_tree({
            "lib.rs": "mod workbench;\n",
            "workbench.rs": "mod surfaces;\n",
            "workbench/surfaces.rs": "pub mod project;\npub mod results;\n",
            "workbench/surfaces/project.rs": PROJECT,
            "workbench/surfaces/results.rs": (
                "pub fn show() {}\n"
                "#[cfg(test)]\nmod extra;\n"
                "#[cfg(test)]\nmod tests {\n"
                "    use crate::workbench::surfaces::project::project_title;\n"
                "    #[test]\n    fn elides() { project_title(); }\n"
                "}\n"
            ),
            "workbench/surfaces/results/extra.rs": (
                "use crate::workbench::surfaces::project::project_title;\n"
                "pub fn helper() { project_title(); }\n"
            ),
        })
        self.assertEqual(values["feature->feature"], {})
        self.assertEqual((stats["files"], stats["production"]), (6, 5))

    def test_a_variant_or_test_item_spelled_like_another_slices_item_is_not_coupling(self) -> None:
        values, _, _ = measure_tree({
            "workbench/app_state/session/serialization.rs": "pub struct WithoutRuns(pub u32);\n",
            "services/session.rs": "pub enum SessionKept { Whole, WithoutRuns(u32) }\n",
            "workbench/lifecycle/recovery.rs": (
                "use crate::services::session::SessionKept;\n"
                "pub fn lean(kept: &SessionKept) -> bool { matches!(kept, SessionKept::WithoutRuns(_)) }\n"
            ),
            "workbench/docks/inspector/executed_deck.rs": "pub fn record() {}\n",
            "simulation/controller/prepared_run.rs": (
                "pub fn prepare() {\n    #[cfg(test)]\n    thread_preparations::record();\n}\n"
                "#[cfg(test)]\nmod thread_preparations {\n    pub fn record() {}\n}\n"
            ),
        })
        self.assertEqual(values["lower->upper"], {})

    def test_a_name_the_lower_crate_defines_never_resolves_to_a_feature_item(self) -> None:
        app = {
            "lib.rs": "pub use rspice_app_state::{simulation, state};\nmod analysis;\nmod workbench;\n",
            "analysis.rs": "pub fn plan() {}\npub fn run_label() {}\n",
            "workbench/surfaces/verify.rs": (
                "use crate::analysis::*;\n"
                "pub fn show() {\n"
                "    let _ = crate::simulation::plan::SimulationPlan;\n"
                "    run_label();\n"
                "}\n"
            ),
        }
        state = {
            "lib.rs": "pub mod simulation;\npub mod state;\n",
            "simulation.rs": "pub mod plan;\n",
            "simulation/plan.rs": "pub struct SimulationPlan;\n",
            "state.rs": "pub fn run_label() {}\n",
        }
        # Read alone, the shell's tree takes the moved `plan` module for the
        # results function and the moved `run_label` for the results one.
        self.assertEqual(measure_tree(app)[0]["feature->feature"], {"simulate->results": 3})
        values, _, stats = measure_tree(app, state)
        # Only the glob import is a reference to results; the lower crate's files are not measured.
        self.assertEqual(values["feature->feature"], {"simulate->results": 1})
        self.assertEqual(stats["files"], 3)

    def test_a_use_inside_an_inline_module_re_exports_like_one_in_a_file(self) -> None:
        values, _, _ = measure_tree({
            "lib.rs": "mod analysis;\nmod workbench;\n",
            "analysis.rs": "pub fn plan_label() {}\n",
            "workbench.rs": (
                "pub(crate) mod lifecycle {\n"
                "    pub(crate) use rspice_app_services::workbench::lifecycle::recovery;\n"
                "}\n"
                "pub(crate) mod browser {\n"
                "    pub(crate) use crate::analysis::plan_label;\n"
                "}\n"
                "mod tools;\n"
            ),
            "workbench/tools.rs": "mod notification_center;\n",
            "workbench/tools/notification_center.rs": (
                "pub fn show() {\n"
                "    crate::workbench::lifecycle::recovery::discard();\n"
                "    crate::workbench::browser::plan_label();\n"
                "}\n"
            ),
        })
        # The first path leaves the crate; the second ends in results, not the shell.
        self.assertEqual(values["lower->upper"], {"framework->results": 1})

    def test_whole_app_methods_and_parameters_outside_their_home_are_counted(self) -> None:
        values, _, _ = measure_tree({
            "workbench/app.rs": "pub struct RSpiceApp;\nimpl RSpiceApp {\n    pub fn frame(&mut self) {}\n}\n",
            "workbench/app_state.rs": "pub struct AppState;\nimpl AppState {\n    pub fn get(&self) {}\n}\n",
            "state/history.rs": "impl crate::workbench::app_state::AppState {\n    pub fn undo(&mut self) {}\n}\n",
            "workbench/surfaces/project.rs": (
                "use crate::workbench::app::RSpiceApp;\n"
                "impl RSpiceApp {\n    pub fn open(&mut self) {}\n}\n"
                "impl Default for RSpiceApp {\n    fn default() -> Self { RSpiceApp }\n}\n"
                "pub fn render(app: &mut RSpiceApp) {}\n"
                "pub fn peek<'a>(app: &'a crate::workbench::app::RSpiceApp) {}\n"
                "#[cfg(test)]\nmod tests {\n    use super::*;\n"
                "    fn helper(app: &mut RSpiceApp) {}\n"
                "    impl RSpiceApp {\n        fn probe(&self) {}\n    }\n}\n"
            ),
        })
        self.assertEqual(values["whole-app"], {
            "impl RSpiceApp methods outside shell": 1,
            "impl AppState methods outside session": 1,
            "&mut RSpiceApp params outside shell": 1,
            "&RSpiceApp params outside shell": 1,
        })


class CeilingTests(unittest.TestCase):
    MEASURED = {
        "feature->feature": {"design->code": 3},
        "lower->upper": {"framework->results": 2},
        "whole-app": {"impl RSpiceApp methods outside shell": 5},
    }

    def test_a_measurement_equal_to_its_ceilings_passes(self) -> None:
        self.assertEqual(compare(self.MEASURED, self.MEASURED), [])

    def test_growth_names_the_edge_the_excess_and_the_files(self) -> None:
        ceilings = json.loads(json.dumps(self.MEASURED))
        ceilings["feature->feature"]["design->code"] = 2
        contributors = {("feature->feature", "design->code"): {"schematic/a.rs": 2, "schematic/b.rs": 1}}
        self.assertEqual(
            compare(self.MEASURED, ceilings, contributors),
            ["feature->feature design->code: 3 exceeds the ceiling 2 by 1; "
             "top files: schematic/a.rs 2, schematic/b.rs 1"],
        )

    def test_a_fall_below_the_ceiling_must_lower_it(self) -> None:
        ceilings = json.loads(json.dumps(self.MEASURED))
        ceilings["whole-app"]["impl RSpiceApp methods outside shell"] = 7
        self.assertEqual(
            compare(self.MEASURED, ceilings),
            ["whole-app impl RSpiceApp methods outside shell: 5 is below the ceiling 7; lower the ceiling to 5"],
        )

    def test_an_entry_for_an_edge_that_measures_zero_is_stale(self) -> None:
        ceilings = json.loads(json.dumps(self.MEASURED))
        ceilings["lower->upper"]["model->session"] = 4
        self.assertEqual(
            compare(self.MEASURED, ceilings),
            ["lower->upper model->session: measures 0; delete its ceiling entry"],
        )

    def test_an_edge_without_an_entry_is_new_coupling(self) -> None:
        ceilings = json.loads(json.dumps(self.MEASURED))
        del ceilings["feature->feature"]["design->code"]
        ceilings["surprise"] = {}
        self.assertEqual(
            compare(self.MEASURED, ceilings),
            ["unknown ceiling group 'surprise'",
             "feature->feature design->code: 3 reference(s) with no ceiling entry; this coupling is new"],
        )

    def test_the_committed_ceilings_are_positive_counts_in_known_groups(self) -> None:
        ceilings = json.loads(CEILINGS.read_text(encoding="utf-8"))
        self.assertEqual(list(ceilings), list(GROUPS))
        for group, entries in ceilings.items():
            self.assertEqual(list(entries), sorted(entries), group)
            for key, value in entries.items():
                self.assertIsInstance(value, int, key)
                self.assertGreater(value, 0, key)


if __name__ == "__main__":
    unittest.main()

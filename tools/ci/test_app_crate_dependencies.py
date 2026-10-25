"""Regression tests for the application crate dependency policy."""

import unittest

from check_app_crate_dependencies import violations


class DependencyPolicyTests(unittest.TestCase):
    def test_extracted_crates_accept_allowed_and_optional_lower_dependencies(self) -> None:
        self.assertEqual(
            violations(
                {"rspice-vocabulary": {"rspice-design-model", "serde"}},
                {"rspice-vocabulary": {"rspice-vocabulary", "rspice-design-model", "serde"}},
            ),
            [],
        )
        self.assertEqual(
            violations(
                {
                    "rspice-formats": {
                        "rspice-vocabulary", "rspice-results", "rspice-core", "rspice-model-library",
                    },
                    "rspice-project": {"rspice-formats"},
                    "rspice-hardcopy": {"rspice-project-contract"},
                    "rspice-ui-kit": {"egui", "rspice-results"},
                    "rspice-schematic-editor": {"egui", "rspice-ui-kit", "rspice-design"},
                    "rspice-results-ui": {"rspice-results", "rspice-core", "rspice-vocabulary"},
                },
                {
                    "rspice-formats": {
                        "rspice-vocabulary", "rspice-results", "rspice-core", "rspice-model-library",
                    },
                    "rspice-project": {"rspice-project", "rspice-formats", "rspice-results"},
                    "rspice-hardcopy": {"rspice-hardcopy", "rspice-project-contract", "rspice-design"},
                    "rspice-ui-kit": {"egui", "rspice-results", "rspice-vocabulary", "rspice-units"},
                    "rspice-schematic-editor": {
                        "egui", "rspice-ui-kit", "rspice-design", "rspice-core", "rspice-model-library",
                    },
                    "rspice-results-ui": {
                        "rspice-results", "rspice-core", "rspice-vocabulary", "rspice-design-model",
                    },
                },
                {"rspice-formats", "csv", "serde"},
                {"rspice-formats", "serde_json", "sha2"},
                {"rspice-formats", "serde_json"},
            ),
            [],
        )

    def test_an_upward_edge_or_transitive_gui_and_engine_dependency_fails(self) -> None:
        issues = violations(
            {"rspice-vocabulary": {"rspice-results"}},
            {"rspice-vocabulary": {"rspice-vocabulary", "egui", "rspice-core"},
             "rspice-worker": {"rspice-worker", "wgpu", "rspice-app"}},
        )
        self.assertEqual(len(issues), 5)
        self.assertTrue(any("forbidden application dependency" in issue for issue in issues))
        self.assertTrue(any("GUI package" in issue for issue in issues))
        self.assertTrue(any("simulator package" in issue for issue in issues))
        self.assertEqual(
            violations({}, {}, {"rspice-formats", "rspice-core"}),
            ["rspice-formats without optional features reaches simulator package rspice-core"],
        )
        self.assertEqual(
            violations({}, {}, native_bundle_formats={"rspice-formats", "rspice-core"}),
            ["rspice-formats with only native-bundle reaches simulator package rspice-core"],
        )
        self.assertEqual(
            violations({}, {}, table_schema_formats={"rspice-formats", "rspice-core"}),
            ["rspice-formats with only table-schema reaches simulator package rspice-core"],
        )
        self.assertEqual(
            violations({}, {"rspice-ui-kit": {"egui", "rspice-core", "rspice-project", "eframe"}}),
            [
                "rspice-ui-kit reaches non-presentation package eframe",
                "rspice-ui-kit reaches non-presentation package rspice-core",
                "rspice-ui-kit reaches non-presentation package rspice-project",
            ],
        )

        self.assertEqual(
            violations({}, {"rspice-results-ui": {
                "rspice-results", "rspice-app", "rspice-project-contract", "rspice-worker", "winit",
            }}),
            [
                "rspice-results-ui reaches application service or host package rspice-app",
                "rspice-results-ui reaches application service or host package rspice-project-contract",
                "rspice-results-ui reaches application service or host package rspice-worker",
                "rspice-results-ui reaches application service or host package winit",
            ],
        )

        self.assertEqual(
            violations({}, {"rspice-schematic-editor": {
                "rspice-design", "rspice-app", "rspice-project", "rspice-simulation", "eframe",
            }}),
            [
                "rspice-schematic-editor reaches application service or host package eframe",
                "rspice-schematic-editor reaches application service or host package rspice-app",
                "rspice-schematic-editor reaches application service or host package rspice-project",
                "rspice-schematic-editor reaches application service or host package rspice-simulation",
            ],
        )

        self.assertEqual(
            violations(
                {"rspice-app-state": {"rspice-design", "rspice-ui-kit", "rspice-app"}},
                {
                    "rspice-app-state": {
                        "rspice-app-state", "rspice-design", "egui", "eframe", "rspice-worker",
                    },
                    "rspice-worker": {"rspice-worker", "rspice-app-state"},
                },
            ),
            [
                "rspice-app-state has forbidden application dependency rspice-app",
                "rspice-app-state reaches application or host package eframe",
                "rspice-app-state reaches application or host package rspice-worker",
                "rspice-worker reaches application package rspice-app-state",
            ],
        )

        self.assertEqual(
            violations(
                {"rspice-app-services": {"rspice-app-state", "rspice-app"}},
                {
                    "rspice-app-services": {
                        "rspice-app-services", "rspice-app-state", "egui", "rfd", "winit",
                        "rspice-worker",
                    },
                    "rspice-schematic-editor": {"rspice-schematic-editor", "rspice-app-services"},
                    "rspice-worker": {"rspice-worker", "rspice-app-services"},
                },
            ),
            [
                "rspice-app-services has forbidden application dependency rspice-app",
                "rspice-app-services reaches application or host package rspice-worker",
                "rspice-app-services reaches application or host package winit",
                "rspice-schematic-editor reaches application service or host package "
                "rspice-app-services",
                "rspice-worker reaches application package rspice-app-services",
            ],
        )
        self.assertEqual(
            violations(
                {"rspice-app-runs": {"rspice-app-services", "rspice-app"}},
                {
                    "rspice-app-runs": {
                        "rspice-app-runs", "rspice-app-services", "egui", "rfd", "eframe",
                    },
                    "rspice-results-ui": {"rspice-results-ui", "rspice-app-runs"},
                    "rspice-worker": {"rspice-worker", "rspice-app-runs"},
                },
            ),
            [
                "rspice-app-runs has forbidden application dependency rspice-app",
                "rspice-app-runs reaches application or host package eframe",
                "rspice-results-ui reaches application service or host package rspice-app-runs",
                "rspice-worker reaches application package rspice-app-runs",
            ],
        )

        issues = violations(
            {
                "rspice-formats": {"rspice-project"},
                "rspice-project-contract": {"rspice-project"},
                "rspice-worker": {"rspice-project"},
            },
            {"rspice-simulation": {"rspice-project-contract", "rspice-project"},
             "rspice-worker": {"rspice-simulation", "rspice-project"}},
        )
        self.assertEqual(
            issues,
            [
                "rspice-formats has forbidden application dependency rspice-project",
                "rspice-project-contract has forbidden application dependency rspice-project",
                "rspice-worker has forbidden application dependency rspice-project",
                "rspice-simulation reaches project aggregate rspice-project",
                "rspice-worker reaches project aggregate rspice-project",
            ],
        )


if __name__ == "__main__":
    unittest.main()

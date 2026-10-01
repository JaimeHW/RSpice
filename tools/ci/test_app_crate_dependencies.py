"""Regression tests for the application crate dependency policy."""

import unittest

from check_app_crate_dependencies import violations


class DependencyPolicyTests(unittest.TestCase):
    def test_extracted_crates_accept_allowed_and_optional_lower_dependencies(self) -> None:
        self.assertEqual(
            violations(
                {"rspice-app-types": {"rspice-design-model", "serde"}},
                {"rspice-app-types": {"rspice-app-types", "rspice-design-model", "serde"}},
            ),
            [],
        )
        self.assertEqual(
            violations(
                {
                    "rspice-formats": {
                        "rspice-app-types", "rspice-results", "rspice-core", "rspice-model-library",
                    },
                    "rspice-project": {"rspice-formats"},
                    "rspice-hardcopy": {"rspice-project-contract"},
                },
                {
                    "rspice-formats": {
                        "rspice-app-types", "rspice-results", "rspice-core", "rspice-model-library",
                    },
                    "rspice-project": {"rspice-project", "rspice-formats", "rspice-results"},
                    "rspice-hardcopy": {"rspice-hardcopy", "rspice-project-contract", "rspice-design"},
                },
                {"rspice-formats", "csv", "serde"},
                {"rspice-formats", "serde_json", "sha2"},
                {"rspice-formats", "serde_json"},
            ),
            [],
        )

    def test_an_upward_edge_or_transitive_gui_and_engine_dependency_fails(self) -> None:
        issues = violations(
            {"rspice-app-types": {"rspice-results"}},
            {"rspice-app-types": {"rspice-app-types", "egui", "rspice-core"}},
        )
        self.assertEqual(len(issues), 3)
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

        issues = violations(
            {
                "rspice-formats": {"rspice-project"},
                "rspice-project-contract": {"rspice-project"},
            },
            {"rspice-simulation": {"rspice-project-contract", "rspice-project"}},
        )
        self.assertEqual(
            issues,
            [
                "rspice-formats has forbidden application dependency rspice-project",
                "rspice-project-contract has forbidden application dependency rspice-project",
                "rspice-simulation reaches project aggregate rspice-project",
            ],
        )


if __name__ == "__main__":
    unittest.main()

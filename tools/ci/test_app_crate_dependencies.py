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
                {"rspice-formats": {"rspice-app-types", "rspice-results", "rspice-core"}},
                {"rspice-formats": {"rspice-app-types", "rspice-results", "rspice-core"}},
                {"rspice-formats", "csv", "serde"},
                {"rspice-formats", "serde_json", "sha2"},
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


if __name__ == "__main__":
    unittest.main()

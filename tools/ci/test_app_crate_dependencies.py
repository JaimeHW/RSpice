"""Regression tests for the application crate dependency policy."""

import unittest

from check_app_crate_dependencies import violations


class DependencyPolicyTests(unittest.TestCase):
    def test_portable_value_crate_accepts_its_existing_lower_dependencies(self) -> None:
        self.assertEqual(
            violations(
                {"rspice-app-types": {"rspice-design-model", "serde"}},
                {"rspice-app-types": {"rspice-app-types", "rspice-design-model", "serde"}},
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


if __name__ == "__main__":
    unittest.main()
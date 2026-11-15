"""Regression tests for the gate on tests skipped without a stated reason."""

from __future__ import annotations

import contextlib
import io
import tempfile
import unittest
from pathlib import Path

from check_test_skips import ignore_findings, main, mask, runtime_skips


def ignores(source: str) -> list[int]:
    """The lines of every reasonless `ignore` in `source`."""
    return [finding.line for finding in ignore_findings("t.rs", source, mask(source))]


def skips(source: str) -> list[tuple[str, int, str | None]]:
    """`(test, line, reason)` for every runtime skip in `source`."""
    return [
        (skip.test, skip.line, skip.reason)
        for skip in runtime_skips("t.rs", source, mask(source))
    ]


class IgnoreReasonTests(unittest.TestCase):
    def test_a_bare_ignore_is_a_finding(self) -> None:
        self.assertEqual(ignores("#[test]\n#[ignore]\nfn slow() {}\n"), [2])

    def test_an_ignore_with_a_reason_passes(self) -> None:
        self.assertEqual(ignores('#[test]\n#[ignore = "needs the oracle"]\nfn slow() {}\n'), [])

    def test_an_empty_or_blank_reason_is_a_finding(self) -> None:
        self.assertEqual(ignores('#[test]\n#[ignore = ""]\nfn a() {}\n'), [2])
        self.assertEqual(ignores('#[test]\n#[ignore = "  "]\nfn a() {}\n'), [2])

    def test_an_ignore_inside_cfg_attr_needs_its_reason_too(self) -> None:
        self.assertEqual(ignores("#[test]\n#[cfg_attr(debug_assertions, ignore)]\nfn a() {}\n"), [2])
        source = (
            "#[test]\n"
            "#[cfg_attr(\n"
            "    debug_assertions,\n"
            '    ignore = "minutes in a debug build"\n'
            ")]\n"
            "fn a() {}\n"
        )
        self.assertEqual(ignores(source), [])

    def test_other_attributes_naming_ignore_are_not_test_ignores(self) -> None:
        source = '#[serde(ignore)]\nstruct S;\n#[allow(clippy::ignore_me)]\nfn a() {}\n'
        self.assertEqual(ignores(source), [])

    def test_ignore_text_in_comments_and_strings_is_not_an_attribute(self) -> None:
        source = (
            "/// Write `#[ignore]` with a reason.\n"
            "// #[ignore]\n"
            "/* outer /* #[ignore] */ still a comment #[ignore] */\n"
            'const TEXT: &str = "#[ignore]";\n'
            'const RAW: &str = r#"a " quote, then #[ignore]"#;\n'
            "#[test]\n"
            "fn a() {}\n"
        )
        self.assertEqual(ignores(source), [])

    def test_char_literals_and_lifetimes_do_not_open_strings(self) -> None:
        source = (
            "fn quote() -> char { '\"' }\n"
            "fn first<'a>(text: &'a str) -> &'a str { text }\n"
            "#[test]\n"
            "#[ignore]\n"
            "fn a() {}\n"
        )
        self.assertEqual(ignores(source), [4])


class RuntimeSkipTests(unittest.TestCase):
    def test_a_silent_early_return_is_an_unannounced_skip(self) -> None:
        source = (
            "#[test]\n"
            "fn needs_file() {\n"
            '    let Ok(text) = std::fs::read_to_string("x") else {\n'
            "        return;\n"
            "    };\n"
            '    assert_eq!(text, "x");\n'
            "}\n"
        )
        self.assertEqual(skips(source), [("needs_file", 4, None)])

    def test_a_printed_message_in_the_block_is_the_reason(self) -> None:
        source = (
            "#[test]\n"
            "fn needs_file() {\n"
            '    let Ok(text) = std::fs::read_to_string("x") else {\n'
            '        eprintln!("x is not present;\n'
            '                   skipping {}", 1);\n'
            "        return;\n"
            "    };\n"
            '    assert_eq!(text, "x");\n'
            "}\n"
        )
        self.assertEqual(skips(source), [("needs_file", 6, "x is not present; skipping {}")])

    def test_a_message_outside_the_returning_block_does_not_count(self) -> None:
        source = (
            "#[test]\n"
            "fn needs_file() {\n"
            '    println!("starting");\n'
            "    if missing() {\n"
            "        return;\n"
            "    }\n"
            "    assert!(true);\n"
            "}\n"
        )
        self.assertEqual(skips(source), [("needs_file", 5, None)])

    def test_a_return_ok_from_a_result_test_is_a_skip_and_an_error_is_not(self) -> None:
        source = (
            "#[test]\n"
            "fn returns_result() -> Result<(), String> {\n"
            "    if missing() {\n"
            "        return Ok(());\n"
            "    }\n"
            "    if broken() {\n"
            '        return Err("broken".into());\n'
            "    }\n"
            "    assert!(true);\n"
            "    Ok(())\n"
            "}\n"
        )
        self.assertEqual(skips(source), [("returns_result", 4, None)])

    def test_a_return_after_the_first_assertion_has_already_checked_something(self) -> None:
        source = (
            "#[test]\n"
            "fn loops() {\n"
            "    for step in steps() {\n"
            "        assert!(step.valid());\n"
            "        if step.last() {\n"
            "            return;\n"
            "        }\n"
            "    }\n"
            "}\n"
            "#[test]\n"
            "fn helper_checked() {\n"
            "    assert_close(1.0, 1.0);\n"
            "    if done() { return; }\n"
            "    more();\n"
            "}\n"
        )
        self.assertEqual(skips(source), [])

    def test_a_return_ending_the_body_is_not_early(self) -> None:
        source = "#[test]\nfn ends() {\n    work();\n    return;\n}\n"
        self.assertEqual(skips(source), [])

    def test_returns_inside_closures_async_blocks_and_nested_functions_stay_there(self) -> None:
        source = (
            "#[test]\n"
            "fn nested() {\n"
            "    let pick = |value: i32| -> i32 {\n"
            "        if value < 0 { return 0; }\n"
            "        value\n"
            "    };\n"
            "    let empty = || { return; };\n"
            "    let task = async move { return; };\n"
            "    fn helper() { if missing() { return; } }\n"
            "    assert_eq!(pick(1), 1);\n"
            "}\n"
        )
        self.assertEqual(skips(source), [])

    def test_only_test_functions_are_read(self) -> None:
        source = (
            "fn helper() {\n"
            "    if missing() { return; }\n"
            "}\n"
            "macro_rules! case {\n"
            "    ($name:ident) => {\n"
            "        #[test]\n"
            "        fn $name() { if missing() { return; } }\n"
            "    };\n"
            "}\n"
        )
        self.assertEqual(skips(source), [])

    def test_a_brace_in_a_string_or_char_does_not_move_the_body(self) -> None:
        source = (
            "#[test]\n"
            "fn braces() {\n"
            "    let open = '{';\n"
            '    let text = "}}}";\n'
            "    if missing() {\n"
            "        return;\n"
            "    }\n"
            "    assert!(open != '}');\n"
            "}\n"
        )
        self.assertEqual(skips(source), [("braces", 6, None)])


class MainTests(unittest.TestCase):
    def run_main(self, files: dict[str, str]) -> tuple[int, str, str]:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name, text in files.items():
                (root / name).write_text(text, encoding="utf-8")
            stdout, stderr = io.StringIO(), io.StringIO()
            with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
                status = main(["--root", str(root)])
            return status, stdout.getvalue(), stderr.getvalue()

    def test_a_clean_tree_passes_and_lists_its_announced_skips(self) -> None:
        status, stdout, stderr = self.run_main(
            {
                "a.rs": (
                    "#[test]\n"
                    '#[ignore = "slow"]\n'
                    "fn slow() {}\n"
                    "#[test]\n"
                    "fn optional() {\n"
                    "    if missing() {\n"
                    '        println!("fixture absent; skipping");\n'
                    "        return;\n"
                    "    }\n"
                    "    assert!(true);\n"
                    "}\n"
                )
            }
        )
        self.assertEqual(status, 0, stderr)
        self.assertIn("1 runtime skip(s), each announced:", stdout)
        self.assertIn("a.rs:8: optional: fixture absent; skipping", stdout)
        self.assertEqual(stderr, "")

    def test_every_reasonless_skip_is_reported_and_fails(self) -> None:
        status, _, stderr = self.run_main(
            {
                "a.rs": "#[test]\n#[ignore]\nfn slow() {}\n",
                "b.rs": "#[test]\nfn silent() {\n    if missing() {\n        return;\n    }\n    assert!(true);\n}\n",
            }
        )
        self.assertEqual(status, 1)
        self.assertIn("2 test skip(s) without a stated reason", stderr)
        self.assertIn("a.rs:2: ignore without a reason", stderr)
        self.assertIn("b.rs:4: test `silent` returns before checking anything", stderr)


if __name__ == "__main__":
    unittest.main()

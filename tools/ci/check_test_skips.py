#!/usr/bin/env python3
"""Fail on a Rust test that is skipped without saying why.

A test can stay out of a run in two ways while the suite still reports green.

* `#[ignore]`. libtest lists the test as ignored. The attribute must carry
  its reason, `#[ignore = "..."]` (inside `cfg_attr(...)` too), so the run
  prints why and the reason stays beside the test it excuses.
* An early `return` from the test body before it has asserted anything.
  libtest reports "ok" for a test that checked nothing. Such a runtime skip
  is for conditions only the run can decide (an optional source file, a host
  capability), and it must announce itself: a `println!`/`eprintln!` in the
  same block, before the `return`, whose message is the reason. Every
  announced skip is listed on each run, so the set stays visible in the CI
  log. A test that needs an opt-in resource, such as an oracle binary named
  by an environment variable, is better written `#[ignore = "requires ..."]`
  and fails when run without it.

A `return` after the body's first assertion (`assert*!`, `panic!`,
`unreachable!`, or a call to an `assert*` helper) ends a test that has
already checked something, so it is not a skip.

The scan reads every tracked `.rs` file. Comments and string literals are
masked before any structure is read, so text about `#[ignore]` in a doc
comment or a format string is never mistaken for the attribute.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# Directories never holding the workspace's own sources: build output, other
# worktrees under `.claude`, and dependency caches.
SKIPPED_DIRECTORIES = {"target", "node_modules", ".git", ".claude"}

TEST_ATTRIBUTE = re.compile(r"#\s*\[\s*test\s*\]")
IGNORE_WORD = re.compile(r"\bignore\b")
RETURN_WORD = re.compile(r"\breturn\b")
# The returns that end a test as a pass: `return;` and, in a test returning a
# `Result`, `return Ok(());`. Any other `return` carries a value or an error.
SKIP_RETURN = re.compile(r"return\s*(;|Ok\s*\(\s*\(\s*\)\s*\)\s*;?)")
PRINT_MACRO = re.compile(r"\b(?:e?println|e?print|writeln)\s*!\s*\(")
# The first check a test makes: an assertion macro, a panic, or a call to an
# assertion helper such as `assert_close(...)`.
ASSERTION = re.compile(r"\b(?:\w*assert\w*|panic|unreachable)\s*(?:!\s*[(\[{]|\()")
FN_WORD = re.compile(r"\bfn\b")
# A closure's head ends in its parameter list (`|a, b|` or `||`), optionally
# with a return type, or is an async block.
CLOSURE_HEAD = re.compile(r"(\|[^|]*\|\s*(->\s*[^{]*)?|\basync(\s+move)?)\s*$")


@dataclass(frozen=True)
class Finding:
    path: str
    line: int
    message: str

    def render(self) -> str:
        return f"{self.path}:{self.line}: {self.message}"


# Where a comment or a literal can start. The prefix of a byte or raw string
# (`b"`, `r#"`) is read back from the quote, which keeps this a plain
# character search.
LITERAL_START = re.compile(r"//|/\*|[\"']")
STRING_LITERAL = re.compile(r'"(?:[^"\\]|\\.)*"', re.DOTALL)
# One character or escape between quotes, so a lifetime or a label (`'a`,
# `'outer:`) never matches.
CHAR_LITERAL = re.compile(r"'(?:[^'\\\n]|\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\}|.))'")
NOT_NEWLINE = re.compile(r"[^\n]")
BLOCK_COMMENT_EDGE = re.compile(r"/\*|\*/")


def blank(text: str) -> str:
    if "\n" not in text:
        return " " * len(text)
    return NOT_NEWLINE.sub(" ", text)


def is_word(char: str) -> bool:
    return char.isalnum() or char == "_"


def raw_string_hashes(source: str, quote: int) -> str | None:
    """The `#`s of a raw string whose opening quote is at `quote`, if it is one."""
    cursor = quote
    while cursor > 0 and source[cursor - 1] == "#":
        cursor -= 1
    if cursor == 0 or source[cursor - 1] != "r":
        return None
    before = cursor - 2
    if before >= 0 and source[before] == "b":
        before -= 1
    if before >= 0 and is_word(source[before]):
        return None
    return source[cursor:quote]


def mask(source: str) -> str:
    """`source` with comment and literal contents replaced by spaces.

    Quote characters stay, so a string literal's span can still be found and
    its text read back from the original. Newlines stay, so offsets map to the
    same lines.
    """
    pieces = []
    position = 0
    while True:
        match = LITERAL_START.search(source, position)
        if match is None:
            pieces.append(source[position:])
            return "".join(pieces)
        start = match.start()
        pieces.append(source[position:start])
        token = match.group()
        if token == "//":
            stop = source.find("\n", start)
            stop = len(source) if stop < 0 else stop
            pieces.append(blank(source[start:stop]))
            position = stop
        elif token == "/*":
            depth = 0
            stop = len(source)
            for edge in BLOCK_COMMENT_EDGE.finditer(source, start):
                depth += 1 if edge.group() == "/*" else -1
                if depth == 0:
                    stop = edge.end()
                    break
            pieces.append(blank(source[start:stop]))
            position = stop
        elif token == '"':
            hashes = raw_string_hashes(source, start)
            if hashes is not None:
                close = source.find('"' + hashes, start + 1)
                close = len(source) if close < 0 else close
            else:
                literal = STRING_LITERAL.match(source, start)
                close = literal.end() - 1 if literal else len(source)
            pieces.append('"' + blank(source[start + 1 : close]) + source[close : close + 1])
            position = close + 1
        else:
            literal = CHAR_LITERAL.match(source, start)
            if literal is None:
                pieces.append(token)
                position = start + 1
            else:
                pieces.append("'" + blank(source[start + 1 : literal.end() - 1]) + "'")
                position = literal.end()


def matching(masked: str, open_index: int, opener: str, closer: str) -> int:
    """The index of the bracket closing the one at `open_index`."""
    depth = 0
    pattern = BRACKETS[opener]
    for bracket in pattern.finditer(masked, open_index):
        depth += 1 if bracket.group() == opener else -1
        if depth == 0:
            return bracket.start()
    return len(masked) - 1


BRACKETS = {
    "{": re.compile(r"[{}]"),
    "[": re.compile(r"[\[\]]"),
    "(": re.compile(r"[()]"),
}


def line_of(source: str, index: int) -> int:
    return source.count("\n", 0, index) + 1


def attributes(masked: str) -> list[tuple[int, int]]:
    """The `(start, end)` spans of every `#[...]` and `#![...]` attribute."""
    spans = []
    for match in re.finditer(r"#!?\s*\[", masked):
        open_index = match.end() - 1
        spans.append((match.start(), matching(masked, open_index, "[", "]") + 1))
    return spans


def ignore_findings(path: str, source: str, masked: str) -> list[Finding]:
    """Every `ignore` in an attribute that gives no reason."""
    findings = []
    for start, end in attributes(masked):
        body = masked[start:end]
        if not re.match(r"#!?\s*\[\s*(ignore\b|cfg_attr\s*\()", body):
            continue
        for word in IGNORE_WORD.finditer(body):
            position = start + word.end()
            rest = masked[position:end]
            assignment = re.match(r"\s*=\s*\"", rest)
            if assignment is None:
                findings.append(
                    Finding(
                        path,
                        line_of(source, position),
                        'ignore without a reason; write #[ignore = "why"]',
                    )
                )
                continue
            literal_start = position + assignment.end()
            literal_end = masked.find('"', literal_start)
            if not source[literal_start:literal_end].strip():
                findings.append(
                    Finding(path, line_of(source, position), "ignore with an empty reason")
                )
    return findings


@dataclass(frozen=True)
class RuntimeSkip:
    path: str
    line: int
    test: str
    reason: str | None


FUNCTION_NAME = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)")
# What may stand between `#[test]` and its `fn`, once any further attributes
# are removed: the qualifiers a function item takes.
ITEM_QUALIFIERS = re.compile(r"\s*((pub(\s*\([^)]*\))?|async|unsafe|const|extern)\s+)*")


def test_bodies(masked: str) -> list[tuple[str, int, int]]:
    """`(name, body_open, body_close)` for every `#[test]` function.

    A `#[test]` not followed by a function item, such as one inside a
    `macro_rules!` template whose function is named `$name`, is left alone.
    """
    bodies = []
    for match in TEST_ATTRIBUTE.finditer(masked):
        function = FUNCTION_NAME.search(masked, match.end())
        if function is None:
            continue
        between = re.sub(r"#\s*\[[^\]]*\]", " ", masked[match.end() : function.start()])
        if ITEM_QUALIFIERS.fullmatch(between) is None:
            continue
        cursor = function.end()
        depth = 0
        while cursor < len(masked):
            char = masked[cursor]
            if char in "(<":
                depth += 1
            elif char in ")>" and not masked.startswith("->", cursor - 1):
                depth -= 1
            elif char == "{" and depth <= 0:
                break
            elif char == ";" and depth <= 0:
                cursor = -1
                break
            cursor += 1
        if cursor < 0 or cursor >= len(masked):
            continue
        bodies.append((function.group(1), cursor, matching(masked, cursor, "{", "}")))
    return bodies


def enclosing_blocks(masked: str, body_open: int, index: int) -> list[int]:
    """Open-brace indices enclosing `index`, innermost first, within a body."""
    stack = []
    for brace in BRACKETS["{"].finditer(masked, body_open, index):
        if brace.group() == "{":
            stack.append(brace.start())
        elif stack:
            stack.pop()
    return list(reversed(stack))


def block_head(masked: str, open_index: int) -> str:
    """The code between the previous statement boundary and a block's brace."""
    cursor = open_index - 1
    while cursor >= 0 and masked[cursor] not in ";{}":
        cursor -= 1
    return masked[cursor + 1 : open_index]


def announcement(source: str, masked: str, block_open: int, return_index: int) -> str | None:
    """The message printed in the block before its `return`, if any."""
    region = masked[block_open:return_index]
    for call in reversed(list(PRINT_MACRO.finditer(region))):
        open_index = block_open + call.end() - 1
        close = matching(masked, open_index, "(", ")")
        text = source[open_index + 1 : close]
        literal = re.search(r'"((?:[^"\\]|\\.)*)"', text, re.DOTALL)
        if literal and literal.group(1).strip():
            return " ".join(literal.group(1).split())
    return None


def runtime_skips(path: str, source: str, masked: str) -> list[RuntimeSkip]:
    """Each early pass-return from a test body, with its announced reason."""
    skips = []
    for name, body_open, body_close in test_bodies(masked):
        first_assertion = ASSERTION.search(masked, body_open, body_close)
        unchecked_end = first_assertion.start() if first_assertion else body_close
        for word in RETURN_WORD.finditer(masked, body_open, unchecked_end):
            if not SKIP_RETURN.match(masked, word.start()):
                continue
            blocks = enclosing_blocks(masked, body_open, word.start())
            statement_end = masked.find(";", word.start())
            if (
                len(blocks) == 1
                and 0 <= statement_end < body_close
                and not masked[statement_end + 1 : body_close].strip()
            ):
                # The body's last statement: the test ends where it would.
                continue
            heads = [block_head(masked, block) for block in blocks[:-1]]
            # A return inside a closure, an async block or a nested function
            # leaves that, not the test.
            if any(CLOSURE_HEAD.search(head) or FN_WORD.search(head) for head in heads):
                continue
            skips.append(
                RuntimeSkip(
                    path,
                    line_of(source, word.start()),
                    name,
                    announcement(source, masked, blocks[0], word.start()),
                )
            )
    return skips


def tracked_rust_files(root: Path) -> list[Path]:
    """Every `.rs` file git tracks under `root`, or every one on disk."""
    try:
        listed = subprocess.run(
            ["git", "-C", str(root), "ls-files", "-z", "--", "*.rs"],
            check=True,
            capture_output=True,
        ).stdout.decode("utf-8")
        return sorted(root / name for name in listed.split("\0") if name)
    except (OSError, subprocess.CalledProcessError):
        found = []
        for directory, subdirectories, files in os.walk(root):
            subdirectories[:] = [name for name in subdirectories if name not in SKIPPED_DIRECTORIES]
            found.extend(Path(directory) / name for name in files if name.endswith(".rs"))
        return sorted(found)


def scan(root: Path) -> tuple[list[Finding], list[RuntimeSkip]]:
    findings: list[Finding] = []
    skips: list[RuntimeSkip] = []
    for file in tracked_rust_files(root):
        if not file.is_file():
            continue
        source = file.read_text(encoding="utf-8", errors="replace")
        if "ignore" not in source and "#[test]" not in source.replace(" ", ""):
            continue
        relative = file.relative_to(root).as_posix()
        masked = mask(source)
        findings.extend(ignore_findings(relative, source, masked))
        for skip in runtime_skips(relative, source, masked):
            skips.append(skip)
            if skip.reason is None:
                findings.append(
                    Finding(
                        relative,
                        skip.line,
                        f"test `{skip.test}` returns before checking anything and says "
                        "nothing; print why before the return, or make it "
                        '#[ignore = "requires ..."]',
                    )
                )
    return findings, skips


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", type=Path, default=ROOT, help="workspace root to scan")
    arguments = parser.parse_args(argv)
    findings, skips = scan(arguments.root)
    announced = [skip for skip in skips if skip.reason is not None]
    print(f"{len(announced)} runtime skip(s), each announced:")
    for skip in announced:
        print(f"  {skip.path}:{skip.line}: {skip.test}: {skip.reason}")
    if findings:
        print(f"\n{len(findings)} test skip(s) without a stated reason:", file=sys.stderr)
        for finding in findings:
            print(f"  {finding.render()}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())

//! Test deck discovery from directories and ngspice-style Makefiles.

use super::*;

impl TestRunner {
    /// Discover all .cir test files in a subdirectory
    pub fn discover_tests(&self, subdir: &str) -> Vec<PathBuf> {
        let dir = self.test_dir.join(subdir);
        if !dir.exists() {
            return Vec::new();
        }

        let mut tests = Vec::new();
        let mut seen = BTreeSet::new();
        if let Some(mut suite_tests) = self.discover_tests_from_makefile(&dir) {
            for path in suite_tests.drain(..) {
                Self::push_discovered_test(&mut tests, &mut seen, path);
            }
        }

        for path in Self::discover_circuit_files_in_dir(&dir) {
            Self::push_discovered_test(&mut tests, &mut seen, path);
        }

        tests.sort();
        tests
    }

    pub(super) fn discover_circuit_files_in_dir(dir: &Path) -> Vec<PathBuf> {
        let mut tests = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
                {
                    tests.push(path);
                }
            }
        }
        tests.sort();
        tests
    }

    pub(super) fn push_discovered_test(
        tests: &mut Vec<PathBuf>,
        seen: &mut BTreeSet<String>,
        path: PathBuf,
    ) {
        if seen.insert(Self::discovery_path_key(&path)) {
            tests.push(path);
        }
    }

    pub(super) fn discovery_path_key(path: &Path) -> String {
        path.to_string_lossy()
            .replace('\\', "/")
            .to_ascii_lowercase()
    }

    pub(super) fn discover_tests_from_makefile(&self, dir: &Path) -> Option<Vec<PathBuf>> {
        let makefile = dir.join("Makefile.am");
        let content = fs::read_to_string(&makefile).ok()?;

        let mut in_tests_block = false;
        let mut tokens: Vec<String> = Vec::new();

        for raw_line in content.lines() {
            let no_comment = raw_line
                .split_once('#')
                .map(|(head, _)| head)
                .unwrap_or(raw_line);
            let trimmed = no_comment.trim();
            if trimmed.is_empty() {
                continue;
            }

            if !in_tests_block {
                let Some((lhs, rhs)) = trimmed.split_once('=') else {
                    continue;
                };
                if lhs.trim() != "TESTS" {
                    continue;
                }
                in_tests_block = true;
                let continuation = rhs.trim_end().ends_with('\\');
                tokens.extend(
                    rhs.trim_end_matches('\\')
                        .split_whitespace()
                        .map(str::to_string),
                );
                if !continuation {
                    break;
                }
                continue;
            }

            let continuation = trimmed.ends_with('\\');
            tokens.extend(
                trimmed
                    .trim_end_matches('\\')
                    .split_whitespace()
                    .map(str::to_string),
            );
            if !continuation {
                break;
            }
        }

        if tokens.is_empty() {
            return None;
        }

        let mut tests = Vec::new();
        for token in tokens {
            if token.contains('$') {
                continue;
            }
            if !token.to_ascii_lowercase().ends_with(".cir") {
                continue;
            }
            let path = dir.join(&token);
            if path.exists() {
                tests.push(path);
            }
        }

        if tests.is_empty() { None } else { Some(tests) }
    }
}

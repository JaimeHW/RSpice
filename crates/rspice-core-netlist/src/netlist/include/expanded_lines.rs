//! Where each line of an include-expanded text was written.

use std::path::Path;
use std::sync::Arc;

use super::NetlistSourceLocation;

/// The file and line that wrote each line of an include-expanded text.
///
/// Expanding a deck's `.include` and `.lib` files splices their lines in
/// place, so line 274 of the expansion is a line of a file the user can open
/// (`net/esd.cir:4`) that nobody would find by counting. A host that runs the
/// expanded text instead of the files, as the application's prepared route
/// and the browser worker do, keeps this map beside the text and reads every
/// location a parse or an elaboration of that text reports through it. Lines
/// are kept as runs of consecutive lines of one file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExpandedLineMap {
    runs: Vec<LineRun>,
    lines: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LineRun {
    /// The run's first expanded line, 1-based.
    first: usize,
    path: Option<Arc<Path>>,
    /// The run's first line in its file, 1-based.
    source_first: usize,
    len: usize,
}

impl ExpandedLineMap {
    /// Record the next expanded line as written at `origin`.
    pub(crate) fn push(&mut self, origin: &NetlistSourceLocation) {
        self.lines += 1;
        if let Some(last) = self.runs.last_mut()
            && last.source_first + last.len == origin.line
            && last.path.as_deref() == origin.path.as_deref()
        {
            last.len += 1;
            return;
        }
        let path = match (self.runs.last(), origin.path.as_deref()) {
            (
                Some(LineRun {
                    path: Some(last), ..
                }),
                Some(path),
            ) if **last == *path => Some(Arc::clone(last)),
            (_, path) => path.map(Arc::from),
        };
        self.runs.push(LineRun {
            first: self.lines,
            path,
            source_first: origin.line,
            len: 1,
        });
    }

    /// How many expanded lines the map covers.
    pub fn len(&self) -> usize {
        self.lines
    }

    pub fn is_empty(&self) -> bool {
        self.lines == 0
    }

    /// Where 1-based expanded line `expanded_line` was written; `None` past
    /// the lines the map covers.
    pub fn locate(&self, expanded_line: usize) -> Option<NetlistSourceLocation> {
        if expanded_line == 0 || expanded_line > self.lines {
            return None;
        }
        let index = self
            .runs
            .partition_point(|run| run.first <= expanded_line)
            .checked_sub(1)?;
        let run = &self.runs[index];
        let line = run.source_first + (expanded_line - run.first);
        Some(NetlistSourceLocation {
            path: run.path.as_deref().map(Path::to_path_buf),
            line,
        })
    }

    /// The map of the first `lines` expanded lines only.
    #[must_use]
    pub fn truncated(&self, lines: usize) -> Self {
        if lines >= self.lines {
            return self.clone();
        }
        let mut runs = Vec::new();
        for run in &self.runs {
            if run.first > lines {
                break;
            }
            let mut run = run.clone();
            run.len = run.len.min(lines + 1 - run.first);
            runs.push(run);
        }
        Self { runs, lines }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(path: &str, line: usize) -> NetlistSourceLocation {
        NetlistSourceLocation::in_file(path, line)
    }

    #[test]
    fn a_line_is_found_in_the_file_that_wrote_it_across_nested_files() {
        let mut map = ExpandedLineMap::default();
        // deck.cir 1-2, a.inc 1-2, b.inc 1-3, a.inc 4, deck.cir 4-5
        for origin in [
            at("/d/deck.cir", 1),
            at("/d/deck.cir", 2),
            at("/d/a.inc", 1),
            at("/d/a.inc", 2),
            at("/d/b.inc", 1),
            at("/d/b.inc", 2),
            at("/d/b.inc", 3),
            at("/d/a.inc", 4),
            at("/d/deck.cir", 4),
            at("/d/deck.cir", 5),
        ] {
            map.push(&origin);
        }
        assert_eq!(map.len(), 10);
        assert_eq!(map.runs.len(), 5, "consecutive lines share a run");
        assert_eq!(map.locate(0), None);
        assert_eq!(map.locate(2), Some(at("/d/deck.cir", 2)));
        assert_eq!(map.locate(3), Some(at("/d/a.inc", 1)));
        assert_eq!(map.locate(7), Some(at("/d/b.inc", 3)));
        assert_eq!(map.locate(8), Some(at("/d/a.inc", 4)));
        assert_eq!(map.locate(10), Some(at("/d/deck.cir", 5)));
        assert_eq!(map.locate(11), None);

        let head = map.truncated(6);
        assert_eq!(head.len(), 6);
        assert_eq!(head.locate(6), Some(at("/d/b.inc", 2)));
        assert_eq!(head.locate(7), None);
        assert_eq!(map.truncated(0).locate(1), None);
        assert_eq!(map.truncated(99), map);
    }
}

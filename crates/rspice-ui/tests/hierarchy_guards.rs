//! Ratchets for the hierarchy program.
//!
//! Every table in this file is a ceiling measured against the tree, not a
//! target negotiated with it. The numbers only go down: an entry that grows
//! fails the build, an entry that reaches zero fails until it is deleted, and
//! a *new* entry is a regression rather than paperwork — the code that
//! produced it belongs somewhere else.
//!
//! The four guards cover the four ways the hierarchy leaked out of the model
//! that owns it:
//!
//! - [`path_grammar_is_owned_by_one_module`] — three modules spell the
//!   instance-path grammar out by hand, each free to disagree with the rest.
//! - [`interactive_surfaces_do_not_read_raw_workspace_buffers`] — docks,
//!   dialogs and probes index the live workspace instead of a projection.
//! - [`every_command_has_a_user_route`] — a command nobody can reach.
//! - [`persisted_policies_reach_a_projection`] — a policy the project stores,
//!   the manager edits, and nothing downstream reads.
//!
//! Each table entry carries the reason it exists, which is also the condition
//! that retires it. An entry with no reason is an entry nobody will remove.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

// =============================================================================
// Reading the shipped half of a source file
// =============================================================================

/// What a byte of a Rust source file is part of.
///
/// The guards below ask two different questions of the same file, and they
/// need two different views of it. Counting `"/top"` has to keep string
/// literals and drop comments — prose describing a defect is not the defect.
/// Matching braces or scanning for a `Command::` token has to drop both, or a
/// `"{"` inside a test fixture unbalances the scan and a variant named inside
/// a doc comment counts as a route.
///
/// One classification pass answers both.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Class {
    Code,
    Comment,
    /// Inside a string, byte-string, or character literal.
    Text,
}

/// The two views of one source file, and the test-only modules it declares.
///
/// Blanking rather than deleting is what makes the two views agree: both are
/// byte-for-byte as long as the original, so the `#[cfg(test)]` item ranges
/// found in the literal-free view cut the same items out of the other.
struct Source {
    /// Comments blanked, string and character literals kept.
    production: String,
    /// Comments, strings and character literals all blanked.
    code: String,
    /// Modules declared as `#[cfg(test)] mod <name>;` or `mod <name> { .. }`.
    ///
    /// A test module that outgrew its parent lives in a sibling file, and that
    /// file carries no `#[cfg(test)]` of its own — so the only record that it
    /// is test-only is the declaration here. Without reading it, every
    /// `<stem>/tests.rs` in the crate would be scanned as shipped code.
    test_only_modules: Vec<String>,
}

impl Source {
    fn read(path: &Path) -> Self {
        let raw = fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let class = classify(&raw);
        let blank = |drop_text: bool| -> Vec<u8> {
            raw.as_bytes()
                .iter()
                .zip(&class)
                .map(|(byte, class)| match class {
                    Class::Comment => b' ',
                    Class::Text if drop_text => b' ',
                    _ => *byte,
                })
                .collect()
        };
        let mut production = blank(false);
        let mut code = blank(true);
        let ranges = {
            let scanned = std::str::from_utf8(&code).expect("blanking preserves UTF-8");
            test_only_ranges(scanned)
        };
        let mut test_only_modules = Vec::new();
        for (start, end) in ranges.iter().rev() {
            let region =
                std::str::from_utf8(&code[*start..*end]).expect("blanking preserves UTF-8");
            if let Some(index) = region.find("mod ") {
                let name: String = region[index + "mod ".len()..]
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    test_only_modules.push(name);
                }
            }
            production.drain(*start..*end);
            code.drain(*start..*end);
        }
        Self {
            production: String::from_utf8(production).expect("whole items removed"),
            code: String::from_utf8(code).expect("whole items removed"),
            test_only_modules,
        }
    }
}

/// Classify every byte of a Rust source file.
fn classify(source: &str) -> Vec<Class> {
    let bytes = source.as_bytes();
    let mut class = vec![Class::Code; bytes.len()];
    let mut index = 0usize;
    while index < bytes.len() {
        match bytes[index] {
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                let end = source[index..]
                    .find('\n')
                    .map_or(bytes.len(), |offset| index + offset);
                class[index..end].fill(Class::Comment);
                index = end;
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                let mut depth = 1usize;
                let mut scan = index + 2;
                while scan < bytes.len() && depth > 0 {
                    if bytes[scan] == b'/' && bytes.get(scan + 1) == Some(&b'*') {
                        depth += 1;
                        scan += 2;
                    } else if bytes[scan] == b'*' && bytes.get(scan + 1) == Some(&b'/') {
                        depth -= 1;
                        scan += 2;
                    } else {
                        scan += 1;
                    }
                }
                class[index..scan].fill(Class::Comment);
                index = scan;
            }
            b'"' => {
                let end = string_end(bytes, index + 1);
                class[index + 1..end.min(bytes.len())].fill(Class::Text);
                index = (end + 1).min(bytes.len());
            }
            b'r' | b'b' if raw_string_hashes(bytes, index).is_some() => {
                let (quote, hashes) = raw_string_hashes(bytes, index).expect("checked above");
                let terminator = format!("\"{}", "#".repeat(hashes));
                let body = quote + 1;
                let end = source[body..]
                    .find(&terminator)
                    .map_or(bytes.len(), |offset| body + offset);
                class[body..end].fill(Class::Text);
                index = (end + terminator.len()).min(bytes.len());
            }
            b'\'' => match char_literal_end(source, index) {
                Some(end) => {
                    class[index + 1..end].fill(Class::Text);
                    index = end + 1;
                }
                // A lifetime, not a literal.
                None => index += 1,
            },
            _ => index += 1,
        }
    }
    class
}

/// Index of the closing quote of a plain string literal starting at `from`.
fn string_end(bytes: &[u8], from: usize) -> usize {
    let mut index = from;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 2,
            b'"' => return index,
            _ => index += 1,
        }
    }
    bytes.len()
}

/// For a raw string opener at `index`, the quote position and hash count.
///
/// Recognizes `r"`, `r#"`, `br##"` and the rest of the family. Returns `None`
/// when `index` is an ordinary identifier that happens to start with `r` or
/// `b`, which is the common case.
fn raw_string_hashes(bytes: &[u8], index: usize) -> Option<(usize, usize)> {
    // A preceding identifier byte means this is the middle of a name.
    if index > 0 && (bytes[index - 1].is_ascii_alphanumeric() || bytes[index - 1] == b'_') {
        return None;
    }
    let mut scan = index;
    if bytes[scan] == b'b' {
        scan += 1;
    }
    if bytes.get(scan) != Some(&b'r') {
        return None;
    }
    scan += 1;
    let hashes_from = scan;
    while bytes.get(scan) == Some(&b'#') {
        scan += 1;
    }
    if bytes.get(scan) == Some(&b'"') {
        Some((scan, scan - hashes_from))
    } else {
        None
    }
}

/// Index of the closing quote of a character literal opening at `index`.
///
/// `'/'` is a literal and `&'a str` is not, and the crate is full of both. The
/// discriminator is exact rather than heuristic: decode one character (or one
/// escape) after the tick and require a closing tick immediately after it.
fn char_literal_end(source: &str, index: usize) -> Option<usize> {
    let rest = &source[index + 1..];
    let mut chars = rest.char_indices();
    let (_, first) = chars.next()?;
    if first == '\\' {
        return rest.find('\'').map(|offset| index + 1 + offset);
    }
    let after = index + 1 + first.len_utf8();
    (source.as_bytes().get(after) == Some(&b'\'')).then_some(after)
}

/// Byte ranges covering each `#[cfg(test)]` item, attribute included.
///
/// The item ends at the close of its first balanced brace group, or at the
/// first `;` if it has none — which is what covers `#[cfg(test)] mod tests;`
/// and `#[cfg(test)] use ...;` alongside an inline module.
fn test_only_ranges(code: &str) -> Vec<(usize, usize)> {
    const ATTRIBUTE: &str = "#[cfg(test)]";
    let bytes = code.as_bytes();
    let mut ranges = Vec::new();
    let mut from = 0usize;
    while let Some(offset) = code[from..].find(ATTRIBUTE) {
        let start = from + offset;
        let mut index = start + ATTRIBUTE.len();
        let mut depth = 0usize;
        while index < bytes.len() {
            match bytes[index] {
                b'{' => depth += 1,
                b'}' => {
                    index += 1;
                    if depth <= 1 {
                        break;
                    }
                    depth -= 1;
                    continue;
                }
                b';' if depth == 0 => {
                    index += 1;
                    break;
                }
                _ => {}
            }
            index += 1;
        }
        ranges.push((start, index));
        from = index;
    }
    ranges
}

// =============================================================================
// Locating the crate's shipped source files
// =============================================================================

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn design_model_src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-design-model/src")
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|error| panic!("read {}: {error}", dir.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// `/`-separated path of a file relative to a scan root.
fn relative(root: &Path, file: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .display()
        .to_string()
        .replace('\\', "/")
}

/// Every shipped `.rs` file under a scan root, with its `/`-separated path.
///
/// A file reached only through a `#[cfg(test)] mod` declaration is not shipped
/// and is excluded, along with everything beneath it. That is the exact rule
/// rather than a name convention: `source_guard.rs` and `fixture_root.rs` are
/// test-only without being called `tests.rs`, and a guard that scanned them
/// would hold ceilings over code the product does not contain.
fn production_files(root: &Path) -> Vec<(String, PathBuf)> {
    let files = rust_sources(root);
    let mut test_only: BTreeSet<String> = BTreeSet::new();
    for file in &files {
        let path = relative(root, file);
        let directory = path.trim_end_matches(".rs");
        for module in Source::read(file).test_only_modules {
            test_only.insert(if path == "lib.rs" {
                format!("{module}.rs")
            } else {
                format!("{directory}/{module}.rs")
            });
        }
    }
    files
        .into_iter()
        .filter_map(|file| {
            let path = relative(root, &file);
            // Exclusion is inherited: `#[cfg(test)] mod fixtures;` makes
            // `fixtures.rs` test-only and everything under `fixtures/` with it.
            let mut ancestor = Some(path.clone());
            while let Some(current) = ancestor {
                if test_only.contains(&current) {
                    return None;
                }
                ancestor = current
                    .trim_end_matches(".rs")
                    .rsplit_once('/')
                    .map(|(directory, _)| format!("{directory}.rs"));
            }
            Some((path, file))
        })
        .collect()
}

fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut rest = haystack;
    while let Some(index) = rest.find(needle) {
        count += 1;
        rest = &rest[index + needle.len()..];
    }
    count
}

/// Render a measured table in the shape the source tables are written in, so a
/// lane lowering a ceiling can paste the new numbers rather than transcribe
/// them.
fn as_table(measured: &BTreeMap<String, usize>) -> String {
    measured
        .iter()
        .map(|(path, count)| format!("    (\"{path}\", {count}),"))
        .collect::<Vec<_>>()
        .join("\n")
}

// =============================================================================
// G1 — one module owns the instance-path grammar
// =============================================================================

/// The spellings of the hierarchical instance path.
///
/// A path such as `/top/XAFE/M1` has a grammar: it is rooted at the top cell,
/// separated by `/`, and its leaf may carry a `.`- or `:`-qualified suffix.
/// Three modules re-implement pieces of that grammar inline, and each one is
/// free to disagree with the others about the root name, whether a leading
/// separator is present, and what a trailing wildcard means.
///
/// These are the patterns that spell it out by hand. The set is the grammar,
/// not a list of known offenders: a pattern that matches nothing today is
/// still listed, because listing it is what stops it appearing tomorrow.
const PATH_GRAMMAR_PATTERNS: &[&str] = &[
    "\"/top\"",
    "split(['/', '.', ':'])",
    "split(['/', '.'])",
    "rmatch_indices(['/'",
    "trim_start_matches('/')",
    "format!(\"{instance_path}/",
    "starts_with(\"/top\")",
];

/// The module allowed to spell the grammar out, once.
///
/// `state/hierarchy_path.rs` is the path type, and it holds the grammar: it
/// parses every accepted spelling, emits the one canonical form, and owns the
/// fold that decides when two paths name the same instance. It is exempt
/// because a path type containing path syntax is the point of having one — the
/// table below is the list of modules that have not yet been moved onto it.
const PATH_GRAMMAR_OWNER: &str = "state/hierarchy_path.rs";

/// Files spelling the path grammar inline, with their exact occurrence counts.
///
/// Measured 2026-08-17 over the shipped source: comments are excluded, string
/// literals are not, and `#[cfg(test)]` items are cut out — a fixture path in
/// a test is not a production grammar site.
const PATH_GRAMMAR_SITES: &[(&str, usize)] = &[
    // `simulation/veriloga.rs` strips a leading separator from a *portable
    // data-file* path, not from a hierarchy path. The pattern set cannot tell
    // the two apart, so this is a ceiling on that one site; it retires when
    // the module takes its path handling from a path type of its own rather
    // than by editing this table.
    ("simulation/veriloga.rs", 1),
];

/// Occurrences of the literal root-cell name across the shipped crate.
///
/// The per-file table above already refuses a new file, so this is not a
/// second gate on the same fact — it is the burn-down number. `"/top"` is the
/// grammar's most quoted fragment and the one a new feature reaches for first,
/// so the program tracks the total rather than inferring it from the rows.
const MAX_ROOT_CELL_LITERALS: usize = 0;

#[test]
fn path_grammar_is_owned_by_one_module() {
    let root = src_dir();
    let allowed: BTreeMap<&str, usize> = PATH_GRAMMAR_SITES.iter().copied().collect();
    assert_eq!(
        allowed.len(),
        PATH_GRAMMAR_SITES.len(),
        "PATH_GRAMMAR_SITES contains a duplicate path"
    );

    let mut measured: BTreeMap<String, usize> = BTreeMap::new();
    let mut root_literals = 0usize;
    for (path, file) in production_files(&root) {
        if path == PATH_GRAMMAR_OWNER {
            continue;
        }
        let production = Source::read(&file).production;
        let count: usize = PATH_GRAMMAR_PATTERNS
            .iter()
            .map(|pattern| count_occurrences(&production, pattern))
            .sum();
        root_literals += count_occurrences(&production, "\"/top\"");
        if count > 0 {
            measured.insert(path, count);
        }
    }

    let mut failures = String::new();
    let mut new_sites = Vec::new();
    let mut grown = Vec::new();
    for (path, count) in &measured {
        match allowed.get(path.as_str()) {
            None => new_sites.push(format!("  {path}: {count}")),
            Some(ceiling) if count > ceiling => {
                grown.push(format!("  {path}: {count}, ceiling is {ceiling}"));
            }
            Some(_) => {}
        }
    }
    let retired: Vec<String> = allowed
        .keys()
        .filter(|path| !measured.contains_key(**path))
        .map(|path| format!("  {path}"))
        .collect();

    if !new_sites.is_empty() {
        failures.push_str(&format!(
            "\nNew modules spelling the hierarchical-path grammar:\n{}\n\
             The instance path is a type, not a string shape. Take it from \
             `{PATH_GRAMMAR_OWNER}` instead of re-deriving it here.\n",
            new_sites.join("\n")
        ));
    }
    if !grown.is_empty() {
        failures.push_str(&format!(
            "\nPath-grammar sites that grew:\n{}\n\
             These are being retired, not extended.\n",
            grown.join("\n")
        ));
    }
    if !retired.is_empty() {
        failures.push_str(&format!(
            "\nPATH_GRAMMAR_SITES entries with nothing left to hold:\n{}\n\
             Delete them.\n",
            retired.join("\n")
        ));
    }
    if !failures.is_empty() {
        failures.push_str(&format!("\nMeasured table:\n{}\n", as_table(&measured)));
    }
    assert!(failures.is_empty(), "{failures}");

    assert!(
        root_literals <= MAX_ROOT_CELL_LITERALS,
        "the literal root-cell name appears {root_literals} times, ceiling is \
         {MAX_ROOT_CELL_LITERALS}.\nThe root cell is whatever the workspace \
         says it is; a literal is a second answer to that question."
    );
    assert_eq!(
        root_literals, MAX_ROOT_CELL_LITERALS,
        "the literal root-cell name is down to {root_literals} from \
         {MAX_ROOT_CELL_LITERALS}. Lower MAX_ROOT_CELL_LITERALS to hold the ground."
    );
}

// =============================================================================
// G2 — interactive surfaces read a projection, not the live workspace
// =============================================================================

/// The one `HierarchySource` constructor that indexes live workspace buffers.
///
/// `from_design_projection` is what every other caller uses: it binds a frozen
/// projection, so what it answers stays true for as long as the projection
/// does. This one takes a `schematic_buffers` map instead, which means
/// whatever it answers is true only until the next edit. A contract-carrying
/// twin of it used to sit beside it; the contract is a builder now, and the
/// buffers still come from here, so this name is the whole entrance.
const RAW_BUFFER_CONSTRUCTORS: &[&str] = &["HierarchySource::from_workspace("];

/// Callers indexing raw workspace buffers, with their exact call counts.
///
/// Re-measured 2026-08-18, after every interactive surface moved onto the
/// design projection. One caller is left, and it is the one that must stay:
/// the hierarchy-creation dialog resolves *hypothetical* buffers — a candidate
/// design that does not exist yet — so there is no projection to read. Both
/// its baseline and its candidate are maps it assembled itself.
///
/// `services/drc/extraction.rs`, `simulation/netlist_gen.rs` and
/// `simulation/netlist_gen/subcircuits.rs` also call these constructors, but
/// only from their own test modules, so they are not sites. The definition
/// site needs no exemption either: it declares `from_workspace`, it does not
/// call `HierarchySource::from_workspace`.
const RAW_BUFFER_READERS: &[(&str, usize)] = &[("workbench/app/dialogs/hierarchy/create.rs", 3)];

#[test]
fn interactive_surfaces_do_not_read_raw_workspace_buffers() {
    let root = src_dir();
    let allowed: BTreeMap<&str, usize> = RAW_BUFFER_READERS.iter().copied().collect();
    assert_eq!(
        allowed.len(),
        RAW_BUFFER_READERS.len(),
        "RAW_BUFFER_READERS contains a duplicate path"
    );

    let mut measured: BTreeMap<String, usize> = BTreeMap::new();
    for (path, file) in production_files(&root) {
        let code = Source::read(&file).code;
        let count: usize = RAW_BUFFER_CONSTRUCTORS
            .iter()
            .map(|constructor| count_occurrences(&code, constructor))
            .sum();
        if count > 0 {
            measured.insert(path, count);
        }
    }

    let mut new_readers = Vec::new();
    let mut grown = Vec::new();
    for (path, count) in &measured {
        match allowed.get(path.as_str()) {
            None => new_readers.push(format!("  {path}: {count}")),
            Some(ceiling) if count > ceiling => {
                grown.push(format!("  {path}: {count}, ceiling is {ceiling}"));
            }
            Some(_) => {}
        }
    }
    let retired: Vec<String> = allowed
        .keys()
        .filter(|path| !measured.contains_key(**path))
        .map(|path| format!("  {path}"))
        .collect();

    let mut failures = String::new();
    if !new_readers.is_empty() {
        failures.push_str(&format!(
            "\nNew callers indexing raw workspace buffers:\n{}\n\
             Resolve the hierarchy once into a projection and read that. A \
             surface that indexes `schematic_buffers` answers a question that \
             stops being true at the next keystroke.\n",
            new_readers.join("\n")
        ));
    }
    if !grown.is_empty() {
        failures.push_str(&format!(
            "\nRaw-buffer readers that grew:\n{}\n\
             These are being retired, not extended.\n",
            grown.join("\n")
        ));
    }
    if !retired.is_empty() {
        failures.push_str(&format!(
            "\nRAW_BUFFER_READERS entries with nothing left to hold:\n{}\n\
             Delete them.\n",
            retired.join("\n")
        ));
    }
    if !failures.is_empty() {
        failures.push_str(&format!("\nMeasured table:\n{}\n", as_table(&measured)));
    }
    assert!(failures.is_empty(), "{failures}");
}

// =============================================================================
// G3 — every registered command is reachable
// =============================================================================

/// Files that route a command to the user, and what each of them is.
///
/// A route is somewhere a person can cause the command to run. The command
/// module itself is not on this list: `commands.rs` dispatches every variant
/// by definition, so counting it would make this guard pass unconditionally.
const COMMAND_ROUTE_FILES: &[(&str, &str)] = &[
    ("workbench/chrome/title_bar.rs", "menu"),
    ("workbench/menu_bar.rs", "menu"),
    ("workbench/menu_bar/", "menu"),
    ("workbench/chrome/toolbar.rs", "toolbar"),
    ("schematic/view/context_menu.rs", "schematic context menu"),
];

/// Dispatch. Nothing under here counts as a route: `execute` names every
/// variant, so a scan that included it would answer its own question.
const COMMAND_DISPATCH_MODULE: &str = "workbench/commands";

/// Where `COMMAND_REGISTRY` declares which commands the product has.
const COMMAND_REGISTRY_FILE: &str = "workbench/commands/vocabulary.rs";

/// Where the palette exclusions and the shortcut table are declared.
const COMMAND_ROUTING_FILE: &str = "workbench/commands/registry.rs";

/// Registered commands with no route, and why.
///
/// An entry is a claim that the product deliberately ships a command nobody
/// can invoke. There is no such thing: the fix is a route, or deletion.
const UNROUTED_COMMANDS: &[(&str, &str)] = &[];

/// The `Command` variants `COMMAND_REGISTRY` lists.
///
/// Payloads are dropped: a menu that routes `Command::ExportNetlist(format)`
/// routes every format, so the variant is the unit of reachability.
fn registered_commands(code: &str) -> BTreeSet<String> {
    let body = braced_body(code, "pub const COMMAND_REGISTRY: &[Command] = &[")
        .expect("COMMAND_REGISTRY is declared in vocabulary.rs");
    let mut variants = BTreeSet::new();
    let mut rest = body.as_str();
    while let Some(index) = rest.find("Command::") {
        rest = &rest[index + "Command::".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !name.is_empty() {
            variants.insert(name);
        }
    }
    variants
}

/// Variants `palette_visible` excludes, and so which ones need another route.
fn palette_excluded(code: &str) -> BTreeSet<String> {
    let body = braced_body(code, "pub const fn palette_visible(self) -> bool {")
        .expect("palette_visible is declared in registry.rs");
    variant_names(&body)
}

/// Variants `shortcut_bindings` maps to a real binding.
///
/// The arms mapping to `NONE` are the point: a command listed there has a
/// declared *absence* of a shortcut, which is not a route.
fn shortcut_bound(code: &str) -> BTreeSet<String> {
    let body = braced_body(
        code,
        "pub const fn shortcut_bindings(self) -> &'static [ShortcutBinding] {",
    )
    .expect("shortcut_bindings is declared in registry.rs");
    let mut bound = BTreeSet::new();
    for arm in body.split(',') {
        let Some((left, right)) = arm.split_once("=>") else {
            continue;
        };
        if right.trim() == "NONE" {
            continue;
        }
        bound.extend(variant_names(left));
    }
    bound
}

/// `Self::Variant` names appearing in a fragment of an `impl Command` block.
fn variant_names(fragment: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut rest = fragment;
    while let Some(index) = rest.find("Self::") {
        rest = &rest[index + "Self::".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !name.is_empty() {
            names.insert(name);
        }
    }
    names
}

/// The balanced-brace body following `signature`, braces excluded.
fn braced_body(code: &str, signature: &str) -> Option<String> {
    let start = code.find(signature)? + signature.len();
    let opener = signature.trim_end().chars().last()?;
    let (open, close) = match opener {
        '[' => (b'[', b']'),
        _ => (b'{', b'}'),
    };
    let bytes = code.as_bytes();
    let mut depth = 1usize;
    let mut index = start;
    while index < bytes.len() {
        if bytes[index] == open {
            depth += 1;
        } else if bytes[index] == close {
            depth -= 1;
            if depth == 0 {
                return Some(code[start..index].to_owned());
            }
        }
        index += 1;
    }
    None
}

/// Variants a route file offers the user, by variant name.
///
/// Two occurrences are not routes and are excluded:
///
/// - A `Command::X | Command::Y => ...` match arm maps a command to an icon or
///   a label. It is presentation, not reachability, and counting it would have
///   reported `ClearConsole` as routed by the trash-icon table.
/// - A name inside a comment or a string literal, which the classification
///   pass has already blanked.
fn routed_variants(code: &str) -> BTreeSet<String> {
    let mut routed = BTreeSet::new();
    let mut rest = code;
    while let Some(index) = rest.find("Command::") {
        rest = &rest[index + "Command::".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() {
            continue;
        }
        if is_route_occurrence(&rest[name.len()..]) {
            routed.insert(name);
        }
    }
    routed
}

/// Whether what follows a variant name is a route rather than a match arm.
fn is_route_occurrence(after: &str) -> bool {
    let after = skip_payload(after);
    for (index, character) in after.char_indices() {
        match character {
            '=' if after[index..].starts_with("=>") => return false,
            ';' | ',' | ')' | '{' | '}' => return true,
            _ => {}
        }
    }
    true
}

/// Step over a `(...)` payload so the delimiter scan sees what follows the
/// whole pattern rather than its first argument.
fn skip_payload(after: &str) -> &str {
    if !after.starts_with('(') {
        return after;
    }
    let bytes = after.as_bytes();
    let mut depth = 0usize;
    for (index, byte) in bytes.iter().enumerate() {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return &after[index + 1..];
                }
            }
            _ => {}
        }
    }
    after
}

#[test]
fn every_command_has_a_user_route() {
    let root = src_dir();
    let mut registry_code = None;
    let mut routing_code = None;
    let mut routes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for (path, file) in production_files(&root) {
        let code = Source::read(&file).code;
        if path == COMMAND_REGISTRY_FILE {
            registry_code = Some(code.clone());
        }
        if path == COMMAND_ROUTING_FILE {
            routing_code = Some(code.clone());
        }
        if let Some((_, kind)) = COMMAND_ROUTE_FILES
            .iter()
            .find(|(route, _)| path == *route || path.starts_with(*route))
        {
            for variant in routed_variants(&code) {
                routes
                    .entry(variant)
                    .or_default()
                    .insert((*kind).to_owned());
            }
        }
        // A control that runs a command in place — a dock button, a landing
        // page tile — is as much a route as a menu item. The dispatcher's own
        // recursive calls are not, so its module is excluded.
        if !path.starts_with(COMMAND_DISPATCH_MODULE) {
            let mut rest = code.as_str();
            while let Some(index) = rest.find("Command::") {
                rest = &rest[index + "Command::".len()..];
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() && rest[name.len()..].starts_with(".execute(") {
                    routes
                        .entry(name)
                        .or_default()
                        .insert("in-place control".to_owned());
                }
            }
        }
    }

    let registry_code = registry_code.expect("vocabulary.rs is in the shipped source");
    let routing_code = routing_code.expect("registry.rs is in the shipped source");
    let registered = registered_commands(&registry_code);
    assert!(
        registered.len() > 100,
        "COMMAND_REGISTRY parsed as only {} variants — the parse broke, not \
         the registry",
        registered.len()
    );
    let excluded = palette_excluded(&routing_code);
    for variant in shortcut_bound(&routing_code) {
        routes
            .entry(variant)
            .or_default()
            .insert("shortcut".to_owned());
    }

    let allowed: BTreeMap<&str, &str> = UNROUTED_COMMANDS.iter().copied().collect();
    assert_eq!(
        allowed.len(),
        UNROUTED_COMMANDS.len(),
        "UNROUTED_COMMANDS contains a duplicate variant"
    );

    let mut unrouted = Vec::new();
    let mut now_routed = Vec::new();
    for variant in &registered {
        // The palette lists everything it does not exclude, so palette
        // visibility is itself a route.
        let routed = !excluded.contains(variant) || routes.contains_key(variant);
        match (routed, allowed.contains_key(variant.as_str())) {
            (false, false) => unrouted.push(format!("  Command::{variant}")),
            (true, true) => now_routed.push(format!(
                "  Command::{variant} — reached by {}",
                routes
                    .get(variant)
                    .map(|kinds| kinds.iter().cloned().collect::<Vec<_>>().join(", "))
                    .unwrap_or_else(|| "the command palette".to_owned())
            )),
            _ => {}
        }
    }
    let missing: Vec<String> = allowed
        .keys()
        .filter(|variant| !registered.contains(**variant))
        .map(|variant| format!("  Command::{variant}"))
        .collect();

    let mut failures = String::new();
    if !unrouted.is_empty() {
        failures.push_str(&format!(
            "\nRegistered commands no user can reach:\n{}\n\
             Give each a menu item, a toolbar button, a context-menu entry, a \
             shortcut, or palette visibility — or delete the registry row. A \
             command that only the dispatcher names is a feature that shipped \
             switched off.\n",
            unrouted.join("\n")
        ));
    }
    if !now_routed.is_empty() {
        failures.push_str(&format!(
            "\nUNROUTED_COMMANDS entries that are now routed:\n{}\n\
             Delete them.\n",
            now_routed.join("\n")
        ));
    }
    if !missing.is_empty() {
        failures.push_str(&format!(
            "\nUNROUTED_COMMANDS entries for commands no longer registered:\n{}\n\
             Delete them.\n",
            missing.join("\n")
        ));
    }
    assert!(failures.is_empty(), "{failures}");
}

// =============================================================================
// G4 — a persisted policy reaches a projection
// =============================================================================

/// A persisted policy record whose fields must reach something downstream.
///
/// `receivers` are the identifiers the code binds the record to. Requiring the
/// receiver — `contract.policy.global_promotion`, not a bare
/// `global_promotion` — is what keeps a common field name such as
/// `schema_version` from matching some other type's field in the same file.
struct PolicyRecord {
    struct_name: &'static str,
    declared_in: &'static str,
    receivers: &'static [&'static str],
}

/// The persisted connectivity contract and the sheet catalog's settings.
const POLICY_RECORDS: &[PolicyRecord] = &[
    PolicyRecord {
        struct_name: "ConnectivityContract",
        declared_in: "state/connectivity_contract.rs",
        receivers: &["connectivity", "contract"],
    },
    PolicyRecord {
        struct_name: "ConnectivityPolicy",
        declared_in: "state/connectivity_contract.rs",
        receivers: &["policy"],
    },
    PolicyRecord {
        struct_name: "SheetCatalogSettings",
        declared_in: "design_management/sheets.rs",
        receivers: &["settings"],
    },
];

/// Fields that are a nested record rather than a setting.
///
/// `ConnectivityContract::policy` reaches every projection trivially — the
/// generator clones the whole contract — so judging it would say nothing.
/// Its leaves are judged as `ConnectivityPolicy` instead.
const NESTED_RECORD_FIELDS: &[(&str, &str)] = &[("ConnectivityContract", "policy")];

/// What a projection is, and which module produces it.
///
/// A persisted policy earns its place by changing something the user or the
/// engine can see: the generated deck, the ERC/DRC report, the materialized
/// projection buffers, the outcome of deleting a sheet, or — for the sheet
/// catalog — the page numbers a hardcopy prints. This is where each of those
/// is built. Reads anywhere else are how a dead control looks alive: the
/// manager dialog that edits the field reads it back, and the project summary
/// prints its label, and neither is evidence that setting it does anything.
const PROJECTION_MODULES: &[(&str, &str)] = &[
    ("simulation/netlist_gen.rs", "the generated deck"),
    ("simulation/netlist_gen/", "the generated deck"),
    ("services/drc.rs", "the ERC/DRC report"),
    ("services/drc/", "the ERC/DRC report"),
    ("workbench/app_state/design_checks.rs", "the ERC/DRC report"),
    (
        "state/workspace/materialize.rs",
        "the materialized projection buffers",
    ),
    (
        "state/workspace/open_documents.rs",
        "the materialized projection buffers",
    ),
    (
        "design_management/sheets.rs",
        "the sheet catalog's page projection",
    ),
    // Deleting a sheet either refuses while objects reference it or moves them
    // to a reviewed replacement. Which of the two happens is a user-visible
    // outcome the catalog's own settings decide, so this is where that
    // decision is projected — not a dialog reading its own field back.
    (
        "workbench/app/actions/sheets.rs",
        "the outcome of deleting a sheet",
    ),
];

/// Fields a projection reads only through a method on their own type.
///
/// A contract may expose a resolved answer rather than the field, and the
/// generator is right to ask for the answer. Naming the accessor here keeps
/// that legitimate indirection from reading as a dead field, and keeps the
/// check from matching a method name as ubiquitous as `validate`.
const FIELD_ACCESSORS: &[(&str, &str, &str)] = &[(
    "ConnectivityContract",
    "dialect_aliases",
    "dialect_canonical_name",
)];

/// Persisted fields that reach no projection, and why.
///
/// Each entry is a claim that the field legitimately cannot change the deck,
/// the report, or the materialized buffers. One of the two is that claim — a
/// schema tag. The other is not: it is a control the sheet manager renders,
/// validates and saves, that nothing downstream reads. A user setting it is
/// choosing between identical outcomes. It is listed so the number is visible
/// and falling, not so it can be left alone.
///
/// `ConnectivityPolicy::width_mismatch` left this table when the ERC began
/// refusing a mismatched vector connection because of it.
/// `SheetCatalogSettings::delete_behavior` left it because the claim was
/// wrong: sheet deletion does not always take the same branch, and the branch
/// it takes is now recorded as the projection it always was.
const INERT_POLICY_FIELDS: &[(&str, &str, &str)] = &[
    (
        "ConnectivityContract",
        "schema_version",
        "persisted schema tag read by the load-time migration gate; \
         bookkeeping, not a setting",
    ),
    (
        "SheetCatalogSettings",
        "connector_policy",
        "the sheet manager renders and persists it; off-sheet connectors are \
         matched by name regardless, so the policy selects nothing",
    ),
];

/// Field names declared by `pub struct <name>` in a source file.
fn struct_fields(code: &str, struct_name: &str) -> Vec<String> {
    let signature = format!("pub struct {struct_name} {{");
    let body = braced_body(code, &signature)
        .unwrap_or_else(|| panic!("`{struct_name}` is not declared as a braced pub struct"));
    body.lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("pub ")?;
            let (name, _) = rest.split_once(':')?;
            let name = name.trim();
            name.chars()
                .all(|c| c.is_alphanumeric() || c == '_')
                .then(|| name.to_owned())
        })
        .collect()
}

/// Whether `code` reads `receiver.field` anywhere.
///
/// An optional `()` between the receiver and the dot covers an accessor —
/// `catalog.settings().page_numbering` — which is how the shipped code reaches
/// a private record.
fn reads_field(code: &str, receivers: &[&str], field: &str) -> bool {
    let needle = format!(".{field}");
    let mut consumed = 0usize;
    let mut rest = code;
    while let Some(index) = rest.find(&needle) {
        let absolute = consumed + index;
        let after = &rest[index + needle.len()..];
        let boundary = after
            .chars()
            .next()
            .is_none_or(|next| !next.is_alphanumeric() && next != '_');
        if boundary {
            let before = code[..absolute].trim_end();
            let before = before.strip_suffix("()").unwrap_or(before);
            let receiver: String = before
                .chars()
                .rev()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            if receivers.contains(&receiver.as_str()) {
                return true;
            }
        }
        consumed = absolute + needle.len();
        rest = &rest[index + needle.len()..];
    }
    false
}

#[test]
fn persisted_policies_reach_a_projection() {
    // Both crates are scanned: the connectivity contract is the GUI crate's,
    // the sheet catalog is the design-management authority's, and both are
    // persisted by the same project file.
    let mut declarations: BTreeMap<String, String> = BTreeMap::new();
    let mut projections: Vec<(String, String, String)> = Vec::new();
    for root in [src_dir(), design_model_src_dir()] {
        for (path, file) in production_files(&root) {
            let declares = POLICY_RECORDS
                .iter()
                .any(|record| record.declared_in == path);
            let projects = PROJECTION_MODULES
                .iter()
                .find(|(module, _)| path == *module || path.starts_with(*module));
            if !declares && projects.is_none() {
                continue;
            }
            let code = Source::read(&file).code;
            if declares {
                declarations.insert(path.clone(), code.clone());
            }
            if let Some((_, output)) = projects {
                projections.push((path, (*output).to_owned(), code));
            }
        }
    }
    assert!(
        !projections.is_empty(),
        "no projection module was found — PROJECTION_MODULES has gone stale, \
         and a guard that scans nothing passes forever"
    );

    let allowed: BTreeMap<(&str, &str), &str> = INERT_POLICY_FIELDS
        .iter()
        .map(|(record, field, reason)| ((*record, *field), *reason))
        .collect();
    assert_eq!(
        allowed.len(),
        INERT_POLICY_FIELDS.len(),
        "INERT_POLICY_FIELDS contains a duplicate (record, field) pair"
    );

    let mut declared_fields: BTreeSet<(&str, String)> = BTreeSet::new();
    let mut inert = Vec::new();
    let mut now_live = Vec::new();

    for record in POLICY_RECORDS {
        let code = declarations
            .get(record.declared_in)
            .unwrap_or_else(|| panic!("{} is not in the shipped source", record.declared_in));
        for field in struct_fields(code, record.struct_name) {
            if NESTED_RECORD_FIELDS
                .iter()
                .any(|(owner, nested)| *owner == record.struct_name && *nested == field)
            {
                continue;
            }
            declared_fields.insert((record.struct_name, field.clone()));
            let accessor = FIELD_ACCESSORS
                .iter()
                .find(|(owner, name, _)| *owner == record.struct_name && *name == field)
                .map(|(_, _, accessor)| *accessor);
            let reached = projections.iter().find(|(_, _, projection)| {
                reads_field(projection, record.receivers, &field)
                    || accessor
                        .is_some_and(|accessor| projection.contains(&format!(".{accessor}(")))
            });
            match (reached, allowed.get(&(record.struct_name, field.as_str()))) {
                (None, None) => inert.push(format!("  {}::{field}", record.struct_name)),
                (Some((path, output, _)), Some(_)) => now_live.push(format!(
                    "  {}::{field} — {output}, via {path}",
                    record.struct_name
                )),
                _ => {}
            }
        }
    }

    let missing: Vec<String> = allowed
        .keys()
        .filter(|(record, field)| !declared_fields.contains(&(*record, (*field).to_owned())))
        .map(|(record, field)| format!("  {record}::{field}"))
        .collect();

    let mut failures = String::new();
    if !inert.is_empty() {
        failures.push_str(&format!(
            "\nPersisted policy fields that reach no projection:\n{}\n\
             The project stores the field and the manager edits it, so a user \
             believes it does something. Wire it into the deck, the report or \
             the materialized buffers, delete it, or say in \
             INERT_POLICY_FIELDS why it legitimately cannot move any of them.\n",
            inert.join("\n")
        ));
    }
    if !now_live.is_empty() {
        failures.push_str(&format!(
            "\nINERT_POLICY_FIELDS entries that now reach a projection:\n{}\n\
             Delete them.\n",
            now_live.join("\n")
        ));
    }
    if !missing.is_empty() {
        failures.push_str(&format!(
            "\nINERT_POLICY_FIELDS entries for fields that no longer exist:\n{}\n\
             Delete them.\n",
            missing.join("\n")
        ));
    }
    assert!(failures.is_empty(), "{failures}");
}

//! The guard that keeps the Verilog-A and mixed elaboration seam typed.
//!
//! Every refusal raised while binding a deck instance to a Verilog-A or
//! Verilog-AMS master reports through [`crate::ElaborationError`], so a
//! frontend reads [`crate::ElaborationErrorKind`] instead of matching prose
//! and so one class of failure cannot be a circuit error at one site and a
//! netlist error at the next. That is a property of the source, not of any
//! one run: a refusal nothing reaches is exactly the one a formatted string
//! would slip back into, because no test would ever render it.
//!
//! So this scans the seam's own text. Four of the six files are wholly
//! elaboration and may not name the untyped constructors at all. The other
//! two — the builder, which is mostly other device families, and the mixed
//! circuit, which is mostly the transient stepper — carry non-elaboration
//! refusals that legitimately mention Verilog-A, so those are enumerated by
//! name with the reason each is not an elaboration site. The list is matched
//! exactly, so removing one is as loud as adding one.

/// Words that appear in an elaboration refusal's message and essentially
/// nowhere else in a `SimulationError::Circuit`/`Netlist` argument.
const SEAM_MARKERS: &[&str] = &[
    "Verilog-A",
    "Verilog-AMS",
    "mixed instance",
    "mixed Verilog",
];

/// The untyped constructors this seam may not use.
const UNTYPED: &[&str] = &["SimulationError::Circuit(", "SimulationError::Netlist("];

/// The wholly-elaboration modules, which may not name an untyped constructor
/// at all.
fn seam_modules() -> Vec<(&'static str, &'static str)> {
    vec![
        ("mixed_modules.rs", include_str!("mixed_modules.rs")),
        ("connect_modules.rs", include_str!("connect_modules.rs")),
        ("veriloga_sources.rs", include_str!("veriloga_sources.rs")),
        ("veriloga_cache.rs", include_str!("veriloga_cache.rs")),
    ]
}

/// A line that only talks about code rather than raising a refusal.
fn is_comment(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("//") || trimmed.starts_with("*")
}

/// Every untyped construction in `source` whose message mentions the seam,
/// rendered as one whitespace-normalized line so it can be recognized.
///
/// A construction's message may be wrapped over several lines, so the window
/// is the construction's own line and the six after it; comment lines inside
/// the window are dropped, because a doc comment is not a diagnostic.
fn untyped_seam_sites(source: &str) -> Vec<String> {
    let lines: Vec<&str> = source.lines().collect();
    let mut sites = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if is_comment(line) || !UNTYPED.iter().any(|untyped| line.contains(untyped)) {
            continue;
        }
        let end = (index + 7).min(lines.len());
        let window = lines[index..end]
            .iter()
            .filter(|line| !is_comment(line))
            .flat_map(|line| line.split_whitespace())
            .collect::<Vec<_>>()
            .join(" ");
        if SEAM_MARKERS.iter().any(|marker| window.contains(marker)) {
            sites.push(window);
        }
    }
    sites
}

#[test]
fn the_elaboration_seam_raises_no_formatted_string_refusals() {
    for (name, source) in seam_modules() {
        let offenders: Vec<&str> = source
            .lines()
            .filter(|line| {
                !is_comment(line) && UNTYPED.iter().any(|untyped| line.contains(untyped))
            })
            .collect();
        assert!(
            offenders.is_empty(),
            "{name} raises an untyped refusal; every failure on this seam is an \
             ElaborationError so the UI reads its kind rather than its prose:\n{}",
            offenders.join("\n")
        );
    }
}

#[test]
fn the_only_untyped_verilog_a_refusals_left_are_not_elaboration() {
    /// Each entry is a unique fragment of a refusal that mentions Verilog-A
    /// and is deliberately *not* an elaboration error, with why.
    const ALLOWED: &[(&str, &str)] = &[
        (
            "Verilog-A nominal temperature must be finite",
            "a `.OPTIONS TNOM` value, validated once for the deck before any \
             instance is bound; it belongs to no instance and no master",
        ),
        (
            "Selecting Verilog-AMS connectrules requires the veriloga feature",
            "a build-configuration refusal raised only when the veriloga \
             feature is off, which is exactly when this seam does not exist",
        ),
        (
            "mixed Verilog-AMS modules cannot advance to t=",
            "the transient stepper refusing an interval the companion rule \
             cannot represent, long after elaboration finished",
        ),
        (
            "static history stamp exceeds the circuit topology",
            "a stamping failure during a solve, not while binding the instance",
        ),
    ];

    let mut sites = untyped_seam_sites(include_str!("../builder.rs"));
    sites.extend(untyped_seam_sites(include_str!(
        "../../circuit/mixed_signal.rs"
    )));

    for site in &sites {
        assert!(
            ALLOWED.iter().any(|(fragment, _)| site.contains(fragment)),
            "a new untyped Verilog-A refusal appeared; raise it as an \
             ElaborationError, or add it here with the reason it is not one:\n{site}"
        );
    }
    for (fragment, reason) in ALLOWED {
        assert!(
            sites.iter().any(|site| site.contains(fragment)),
            "the allowance for '{fragment}' ({reason}) matches nothing any more; \
             delete it rather than leaving the list stale"
        );
    }
    assert_eq!(
        sites.len(),
        ALLOWED.len(),
        "every remaining untyped Verilog-A refusal must be accounted for once: {sites:#?}"
    );
}

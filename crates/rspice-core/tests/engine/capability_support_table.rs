//! The root README's analysis support table is the capability matrix.
//!
//! `README.md` shows, per analysis and per circuit family, whether the
//! analysis runs, runs on the instances a condition admits, or refuses. The
//! table is written from `engine::capability_cell`, the table every analysis
//! consults before solver work, so the README cannot claim a cell the engine
//! does not enforce. This test fails when the README's copy differs;
//! `RSPICE_WRITE_SUPPORT_TABLE=1` rewrites the README's copy instead.

use std::fmt::Write as _;

use rspice_core::engine::{CapabilityAnalysis, CapabilityFamily, CellSupport, capability_cell};

const README: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../README.md");
const BEGIN: &str = "<!-- support table: written from engine::capability_cell by \
                     crates/rspice-core/tests/engine/capability_support_table.rs -->";
const END: &str = "<!-- end of support table -->";

/// `text` with its first letter capitalized.
fn capitalized(text: &str) -> String {
    let mut characters = text.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

/// `text` safe inside a Markdown table cell.
fn cell_text(text: &str) -> String {
    text.replace('|', "\\|")
}

/// The table and its notes, between the markers.
fn support_table() -> String {
    let mut conditions: Vec<&str> = Vec::new();
    let mut refusals: Vec<(&str, &str)> = Vec::new();
    let mut table = String::new();
    let _ = write!(table, "| Analysis |");
    for family in CapabilityFamily::ALL {
        let _ = write!(table, " {} |", capitalized(family.label()));
    }
    table.push_str("\n| :--- |");
    for _ in CapabilityFamily::ALL {
        table.push_str(" :---: |");
    }
    table.push('\n');
    for analysis in CapabilityAnalysis::ALL {
        let _ = write!(table, "| {} |", capitalized(analysis.label()));
        for family in CapabilityFamily::ALL {
            let entry = match capability_cell(family, analysis) {
                CellSupport::Supported { .. } => "yes".to_owned(),
                CellSupport::Restricted { condition, .. } => {
                    let index = conditions
                        .iter()
                        .position(|known| *known == condition)
                        .unwrap_or_else(|| {
                            conditions.push(condition);
                            conditions.len() - 1
                        });
                    format!("if ({})", index + 1)
                }
                CellSupport::Refused { token, reason } => {
                    let index = refusals
                        .iter()
                        .position(|known| *known == (token, reason))
                        .unwrap_or_else(|| {
                            refusals.push((token, reason));
                            refusals.len() - 1
                        });
                    format!("no ({})", char::from(b'a' + index as u8))
                }
            };
            let _ = write!(table, " {entry} |");
        }
        table.push('\n');
    }
    table.push_str(
        "\nA restricted cell runs the instances its condition admits and refuses the others by \
         name:\n\n",
    );
    for (index, condition) in conditions.iter().enumerate() {
        let _ = writeln!(table, "- ({}) {}", index + 1, cell_text(condition));
    }
    table.push_str("\nA refused cell refuses on the family's first instance:\n\n");
    for (index, (token, reason)) in refusals.iter().enumerate() {
        let _ = writeln!(
            table,
            "- ({}) `{token}`: {}",
            char::from(b'a' + index as u8),
            cell_text(reason)
        );
    }
    format!("{BEGIN}\n\n{table}\n{END}")
}

#[test]
fn the_readme_support_table_is_the_capability_matrix() {
    let readme = std::fs::read_to_string(README).expect("read README.md");
    let readme = readme.replace("\r\n", "\n");
    let start = readme
        .find(BEGIN)
        .expect("README.md carries the support table's opening marker");
    let end = readme[start..]
        .find(END)
        .map(|offset| start + offset + END.len())
        .expect("README.md carries the support table's closing marker");
    let written = support_table();
    if std::env::var_os("RSPICE_WRITE_SUPPORT_TABLE").is_some() {
        let rewritten = format!("{}{written}{}", &readme[..start], &readme[end..]);
        std::fs::write(README, rewritten).expect("write README.md");
        eprintln!(
            "RSPICE_WRITE_SUPPORT_TABLE is set: rewrote README.md's support table instead of checking it"
        );
        return;
    }
    assert!(
        readme[start..end] == written,
        "README.md's support table differs from engine::capability_cell; rewrite it with \
         `RSPICE_WRITE_SUPPORT_TABLE=1 cargo test -p rspice-core --test engine \
         the_readme_support_table_is_the_capability_matrix`.\n\nexpected:\n{written}"
    );
}

use std::collections::BTreeMap;

use super::projection::{ShortcutReferenceModel, ShortcutReferenceRow, ShortcutReferenceStatus};
use super::schema::ShortcutArtifactScope;
use crate::workbench::commands::CommandPlatform;
use crate::workbench::shortcuts::ShortcutBindingSlot;

/// Deterministic UTF-8 Markdown generated from the same immutable reference
/// model as JSON. No live preference state is queried during rendering.
#[must_use]
pub fn serialize_shortcut_reference_markdown(model: &ShortcutReferenceModel) -> String {
    let manifest = model.manifest();
    let contexts = manifest.coverage.contexts.join(", ");
    let platforms = manifest
        .coverage
        .platforms
        .iter()
        .map(|platform| platform.label())
        .collect::<Vec<_>>()
        .join(", ");
    let mut output = String::from("# RSpice keyboard shortcuts\n\n");
    output.push_str(&format!(
        "- Schema: `rspice.shortcuts/{}`\n- Scope: {}\n- Context coverage: {}\n- Platform coverage: {}\n- Explicit platform mappings: {}\n",
        manifest.schema_version,
        scope_label(manifest.scope),
        escape_markdown(&contexts),
        escape_markdown(&platforms),
        if manifest.platform_mappings_included { "Yes" } else { "No" },
    ));
    if manifest.unknown_commands_omitted > 0 {
        output.push_str(&format!(
            "- Unknown future commands omitted: {}\n",
            manifest.unknown_commands_omitted
        ));
    }

    output.push_str("\n## Policies\n\n");
    if model.policy_summary().is_empty() {
        output.push_str("Materialized bindings; no execution policy records are embedded.\n");
    } else {
        for (name, value) in model.policy_summary() {
            output.push_str(&format!(
                "- {}: `{}`\n",
                escape_markdown(name),
                escape_markdown(value)
            ));
        }
    }

    let mut by_context = BTreeMap::<String, Vec<&ShortcutReferenceRow>>::new();
    for row in model.rows() {
        by_context.entry(row.context.clone()).or_default().push(row);
    }
    for (context, rows) in by_context {
        output.push_str(&format!("\n## {}\n\n", escape_markdown(&context)));
        output.push_str("| Command | Stable ID | Primary | Alternate | Platform | Status |\n");
        output.push_str("|---|---|---|---|---|---|\n");

        let mut combined = BTreeMap::<(String, Option<CommandPlatform>), CombinedRow>::new();
        for row in rows {
            let entry = combined
                .entry((row.command_id.clone(), row.platform))
                .or_insert_with(|| CombinedRow::from_row(row));
            entry.absorb(row);
        }
        for row in combined.into_values() {
            output.push_str(&format!(
                "| {} | `{}` | {} | {} | {} | {} |\n",
                escape_markdown(&row.command_label),
                escape_markdown(&row.command_id),
                escape_markdown(row.primary.as_deref().unwrap_or("—")),
                escape_markdown(row.alternate.as_deref().unwrap_or("—")),
                row.platform.map_or("All covered", CommandPlatform::label),
                row.status,
            ));
        }
    }
    output
}

struct CombinedRow {
    command_id: String,
    command_label: String,
    platform: Option<CommandPlatform>,
    primary: Option<String>,
    alternate: Option<String>,
    status: &'static str,
}

impl CombinedRow {
    fn from_row(row: &ShortcutReferenceRow) -> Self {
        Self {
            command_id: row.command_id.clone(),
            command_label: row.command_label.clone(),
            platform: row.platform,
            primary: None,
            alternate: None,
            status: status_label(row.status),
        }
    }

    fn absorb(&mut self, row: &ShortcutReferenceRow) {
        match row.slot {
            Some(ShortcutBindingSlot::Primary) => {
                self.primary = Some(row.display_sequence.clone());
            }
            Some(ShortcutBindingSlot::Alternate) => {
                self.alternate = Some(row.display_sequence.clone());
            }
            None => {}
        }
        if row.status == ShortcutReferenceStatus::UserOverride {
            self.status = status_label(row.status);
        }
    }
}

const fn scope_label(scope: ShortcutArtifactScope) -> &'static str {
    match scope {
        ShortcutArtifactScope::UserOverrides => "User overrides and platform exceptions",
        ShortcutArtifactScope::CompleteResolved => "Complete resolved shortcut map",
        ShortcutArtifactScope::CurrentWorkspace => "Current workspace",
    }
}

const fn status_label(status: ShortcutReferenceStatus) -> &'static str {
    match status {
        ShortcutReferenceStatus::Default => "Default",
        ShortcutReferenceStatus::UserOverride => "User override",
        ShortcutReferenceStatus::Unbound => "Unbound",
    }
}

fn escape_markdown(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace("\r\n", "<br>")
        .replace(['\r', '\n'], "<br>")
}

#[cfg(test)]
mod tests {
    use egui::os::OperatingSystem;

    use super::*;
    use crate::common::shortcut_artifacts::projection::{
        ShortcutExportRequest, ShortcutExportScope, build_shortcut_reference_model,
    };
    use crate::workbench::ShortcutPreferences;

    #[test]
    fn markdown_is_deterministic_and_uses_the_shared_reference_rows() {
        let model = build_shortcut_reference_model(
            &ShortcutPreferences::default(),
            &ShortcutExportRequest {
                scope: ShortcutExportScope::CompleteResolved,
                include_platform_mappings: false,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: OperatingSystem::Windows,
                current_contexts: Vec::new(),
            },
        )
        .unwrap();
        let first = serialize_shortcut_reference_markdown(&model);
        let second = serialize_shortcut_reference_markdown(&model);
        assert_eq!(first, second);
        assert!(first.starts_with("# RSpice keyboard shortcuts\n"));
        assert!(first.contains("| Save | `save-project` | Ctrl+S |"));
        assert!(!first.contains("protected-override-acknowledgements"));
    }

    #[test]
    fn markdown_escaping_is_total_for_tables_and_line_breaks() {
        assert_eq!(escape_markdown("a|b\\c\nd"), r"a\|b\\c<br>d");
    }
}

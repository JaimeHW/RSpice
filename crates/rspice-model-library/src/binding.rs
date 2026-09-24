//! Materialized model cards bound to a named process corner.

use rspice_app_types::product::ProcessCorner;

/// One explicit foundry/library model binding for a process point.
///
/// Model cards are fully materialized from an authenticated in-memory source
/// snapshot before a worker request is created. Workers never receive a model
/// path that could be reopened after verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerModelBinding {
    pub process: ProcessCorner,
    pub source_label: String,
    pub section: Option<String>,
    pub materialized_model_cards: String,
}

impl CornerModelBinding {
    pub fn validate(&self) -> Result<(), String> {
        let label = self.source_label.trim();
        if label.is_empty() {
            return Err("Corner model binding requires a source label".to_owned());
        }
        if label.chars().any(char::is_control) {
            return Err(format!(
                "Corner model source label contains a control character and cannot be represented safely: {label:?}"
            ));
        }
        if let Some(section) = self.section.as_deref() {
            let section = section.trim();
            if section.is_empty() {
                return Err("Corner model section cannot be empty".to_owned());
            }
            if section.chars().any(|character| {
                character.is_whitespace()
                    || character == '"'
                    || character == '\''
                    || character.is_control()
            }) {
                return Err(format!(
                    "Corner model section contains an unsupported character: {section}"
                ));
            }
        }
        if self.materialized_model_cards.trim().is_empty() {
            return Err(format!(
                "Corner model binding '{label}' contains no materialized model cards"
            ));
        }
        for line in self.materialized_model_cards.lines() {
            if rspice_core::netlist::is_spice_end_card(
                line,
                rspice_core::config::ExpressionDialect::Ngspice,
            ) {
                return Err(format!(
                    "Corner model binding '{label}' contains a terminal .end card"
                ));
            }
            if rspice_core::netlist::parse_include_directive(line).is_some()
                || rspice_core::netlist::parse_lib_directive(line).is_some()
            {
                return Err(format!(
                    "Corner model binding '{label}' contains an unresolved include/library directive"
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_rejects_commented_terminal_cards_including_the_first_record() {
        for terminal in [".end; done", ".END // done", " .end $ done", ".end"] {
            for cards in [terminal.to_owned(), format!(".model fast D\n{terminal}")] {
                let binding = CornerModelBinding {
                    process: ProcessCorner::FF,
                    source_label: "models.lib [ff]".to_owned(),
                    section: Some("ff".to_owned()),
                    materialized_model_cards: cards,
                };
                assert!(
                    binding
                        .validate()
                        .expect_err("model payloads cannot terminate the deck")
                        .contains("terminal .end")
                );
            }
        }
    }

    #[test]
    fn binding_rejects_unsafe_labels_and_unresolved_directives() {
        let binding = CornerModelBinding {
            process: ProcessCorner::FF,
            source_label: "models.lib\n.end".to_owned(),
            section: Some("ff".to_owned()),
            materialized_model_cards: ".model fast D (IS=1e-12)".to_owned(),
        };

        assert!(binding.validate().is_err());

        let unresolved = CornerModelBinding {
            process: ProcessCorner::FF,
            source_label: "models.lib [ff]".to_owned(),
            section: Some("ff".to_owned()),
            materialized_model_cards: ".include \"late.inc\"".to_owned(),
        };
        assert!(unresolved.validate().is_err());
    }
}

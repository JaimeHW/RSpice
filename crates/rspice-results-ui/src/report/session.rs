//! Report editor selection and drafts; canonical document edits remain app-owned.
use rspice_app_types::product::ResultDocumentId;
use rspice_results::report_document::{ReportBlockId, ReportPageId};
use serde::{Deserialize, Serialize};

/// Report-composer selection and transactional editor presentation. The
/// canonical report documents remain project-owned; only stable
/// selection identity is restored with the application session. Dialog drafts
/// never persist until their domain transaction commits.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportAuthoringState {
    #[serde(default)]
    pub selected_document: Option<ResultDocumentId>,
    #[serde(default)]
    pub selected_page: Option<ReportPageId>,
    #[serde(skip)]
    pub create_document_open: bool,
    #[serde(skip)]
    pub create_document_title: String,
    #[serde(skip)]
    pub create_document_template: usize,
    #[serde(skip)]
    pub add_page_open: bool,
    #[serde(skip)]
    pub add_page_title: String,
    #[serde(skip)]
    pub page_properties_open: bool,
    #[serde(skip)]
    pub page_properties_page: Option<ReportPageId>,
    #[serde(skip)]
    pub page_title_draft: String,
    #[serde(skip)]
    pub inline_page_settings_page: Option<ReportPageId>,
    #[serde(skip)]
    pub inline_page_title_draft: String,
    #[serde(skip)]
    pub selected_report_block: Option<ReportBlockId>,
    #[serde(skip)]
    pub add_report_element_open: bool,
    #[serde(skip)]
    pub add_report_element_kind: usize,
    #[serde(skip)]
    pub add_report_element_title: String,
    #[serde(skip)]
    pub add_report_element_primary: String,
    #[serde(skip)]
    pub add_report_element_secondary: String,
    #[serde(skip)]
    pub add_report_element_tertiary: String,
    #[serde(skip)]
    pub add_report_element_style: usize,
    #[serde(skip)]
    pub add_report_element_status: usize,
    #[serde(skip)]
    pub add_report_element_source_run: usize,
    #[serde(skip)]
    pub remove_report_block_open: bool,
    #[serde(skip)]
    pub insert_result_document_open: bool,
    #[serde(skip)]
    pub insert_result_document_index: usize,
    #[serde(skip)]
    pub insert_result_caption: String,
    #[serde(skip)]
    pub insert_result_alternative_text: String,
    #[serde(skip)]
    pub insert_result_sizing: usize,
    #[serde(skip)]
    pub insert_result_frozen: bool,
    #[serde(skip)]
    pub report_template_draft: usize,
    #[serde(skip)]
    pub page_update_policy_draft: usize,
    #[serde(skip)]
    pub transaction_error: Option<String>,
}

impl ReportAuthoringState {
    /// Reset the form for an element kind while retaining its source selection.
    pub fn reset_add_report_element_kind(&mut self, kind_index: usize) {
        self.add_report_element_kind = kind_index.min(6);
        let (title, primary, secondary) = match self.add_report_element_kind {
            1 => ("Data table", "Value", "0"),
            2 => ("Datasheet", "Parameter", "Value"),
            3 => ("Requirement", "State the requirement.", "REQ-1"),
            4 => ("Specification", "V(out)", "<= 1 V"),
            5 => ("rspice-local-session", "Review note.", ""),
            6 => (
                "Verification evidence",
                "Summarize the retained evidence.",
                "",
            ),
            _ => (
                "Engineering summary",
                "Describe the conclusion and its supporting evidence.",
                "",
            ),
        };
        self.add_report_element_title = title.to_owned();
        self.add_report_element_primary = primary.to_owned();
        self.add_report_element_secondary = secondary.to_owned();
        self.add_report_element_tertiary.clear();
        self.add_report_element_style = 0;
        self.add_report_element_status = 0;
        self.transaction_error = None;
    }

    /// Whether the current form can submit an element with the available source.
    pub fn valid_add_report_element_draft(&self, source_available: bool) -> bool {
        let kind = self.add_report_element_kind.min(6);
        let title = self.add_report_element_title.trim();
        let primary = self.add_report_element_primary.trim();
        let secondary = self.add_report_element_secondary.trim();
        let source_valid = !matches!(kind, 1 | 2 | 3 | 4 | 6) || source_available;
        let title_limit = if kind == 5 { 256 } else { 512 };
        let primary_limit = match kind {
            1 | 2 => 256,
            4 => 4_096,
            _ => 65_536,
        };
        let secondary_valid = match kind {
            1 | 2 => !secondary.is_empty() && secondary.len() <= 16_384,
            3 => {
                !secondary.is_empty()
                    && secondary.len() <= 256
                    && !secondary.chars().any(|character| {
                        character.is_control()
                            || character.is_whitespace()
                            || matches!(character, '/' | '\\')
                    })
            }
            4 => !secondary.is_empty() && secondary.len() <= 4_096,
            _ => true,
        };
        let tertiary_valid = match kind {
            1 | 2 => self.add_report_element_tertiary.trim().len() <= 64,
            3 => self.add_report_element_tertiary.trim().len() <= 512,
            4 => self.add_report_element_tertiary.trim().len() <= 4_096,
            _ => self.add_report_element_tertiary.trim().is_empty(),
        };
        !title.is_empty()
            && title.len() <= title_limit
            && !title.chars().any(char::is_control)
            && !primary.is_empty()
            && primary.len() <= primary_limit
            && !primary
                .chars()
                .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
            && secondary_valid
            && tertiary_valid
            && source_valid
    }
}

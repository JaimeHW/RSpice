//! What one feature shows that another reads, for a port whose answers are
//! values every part of the workbench can name.
//!
//! A port here names nothing of the shell's or of a feature's, so the code
//! that answers from a feature's reads, wherever it sits, takes the port and
//! not the feature.

use rspice_results::result_presentation::ResultViewer;
use rspice_results::visualization_document::{PageId, PaneId};
use rspice_vocabulary::product::ResultDocumentId;

/// What the results workspace shows that another feature reads.
pub(crate) trait ResultsPort {
    /// The viewer the results workspace shows a dataset in.
    fn result_viewer(&self) -> ResultViewer;
    /// The measurement the histogram viewer shows, when one was chosen.
    fn histogram_selection(&self) -> Option<&str>;
    /// The pane of the result document `document` the reader last selected.
    fn persistent_document_pane(&self, document: ResultDocumentId) -> Option<PaneId>;
    /// The page of the result document `document` the reader last showed.
    fn persistent_document_page(&self, document: ResultDocumentId) -> Option<PageId>;
}

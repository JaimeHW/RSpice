//! The source edit the Verilog-A interface tests share with the shell's
//! frame driver (veriloga_flow_tests): the text of the Verilog-A source in
//! front replaced as a reader's edit. The interface tests themselves are the
//! code feature's, in feature_tests/code/veriloga_interface.

use super::*;
use crate::workbench::documents::code_workspace;

/// Replace the open view's root source with `source`.
pub(crate) fn edit_source(app: &mut RSpiceApp, source: &str) {
    let selected = code_workspace::selected_veriloga_source_in(&app.state.model)
        .expect("a Verilog-A view is open");
    let path = selected.document().logical_path().to_owned();
    assert!(
        code_workspace::replace_selected_veriloga_file(
            &mut app.code_workspace_parts(),
            &selected,
            &path,
            source.to_owned()
        )
        .expect("the edit applies")
    );
}

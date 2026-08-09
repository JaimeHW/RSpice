//! Revision-bound active-source buffer reuse for immediate-mode editor frames.
//!
//! The authoritative bytes remain in `ProjectSourceRegistry`. This cache moves
//! one active `String` between frames so a 16 MiB source is not cloned at the
//! display refresh rate. Every take/return is sealed to project, bundle,
//! revision, closure digest, language, and logical path.

use crate::state::ProjectSourceLanguage;
use crate::workbench::RSpiceApp;

use super::{CodeSourceEditorBufferCache, CodeSourceEditorBufferIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CodeSourceEditorCommit {
    Unchanged,
    Committed,
    Rejected,
}

pub(crate) fn take_project_source_editor_buffer(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    identity: &CodeSourceEditorBufferIdentity,
) -> Option<String> {
    let cached = app
        .state
        .ui
        .code_workspace
        .source_editor_buffer_caches
        .remove(&language);
    let bundle = app
        .state
        .workspace
        .project_sources
        .get_bundle(identity.bundle_id)?;
    if app.state.workspace.project.id() != identity.project_id
        || bundle.language() != language
        || bundle.revision().get() != identity.bundle_revision
        || bundle.closure_digest() != identity.closure_digest
    {
        return None;
    }
    if let Some(cache) = cached
        && cache.identity.eq(identity)
    {
        return Some(cache.content);
    }
    bundle
        .file_content(identity.path.as_ref())
        .map(str::to_owned)
}

pub(crate) fn return_project_source_editor_buffer(
    app: &mut RSpiceApp,
    language: ProjectSourceLanguage,
    expected: &CodeSourceEditorBufferIdentity,
    content: String,
    commit: CodeSourceEditorCommit,
) {
    app.state
        .ui
        .code_workspace
        .source_editor_buffer_caches
        .remove(&language);
    if commit == CodeSourceEditorCommit::Rejected {
        return;
    }
    let Some(bundle) = app
        .state
        .workspace
        .project_sources
        .get_bundle(expected.bundle_id)
    else {
        return;
    };
    if app.state.workspace.project.id() != expected.project_id
        || bundle.language() != language
        || !bundle.contains_file(expected.path.as_ref())
    {
        return;
    }
    let identity = match commit {
        CodeSourceEditorCommit::Unchanged
            if bundle.revision().get() == expected.bundle_revision
                && bundle.closure_digest() == expected.closure_digest =>
        {
            expected.clone()
        }
        CodeSourceEditorCommit::Committed
            if bundle.file_content(expected.path.as_ref()) == Some(content.as_str()) =>
        {
            CodeSourceEditorBufferIdentity {
                project_id: expected.project_id,
                bundle_id: expected.bundle_id,
                bundle_revision: bundle.revision().get(),
                closure_digest: bundle.closure_digest(),
                path: expected.path.clone(),
            }
        }
        CodeSourceEditorCommit::Unchanged
        | CodeSourceEditorCommit::Committed
        | CodeSourceEditorCommit::Rejected => return,
    };
    app.state
        .ui
        .code_workspace
        .source_editor_buffer_caches
        .insert(language, CodeSourceEditorBufferCache { identity, content });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ProjectSourceOwner, ProjectSourceRole};

    fn automation_identity(app: &RSpiceApp) -> CodeSourceEditorBufferIdentity {
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::code_workspace(language))
            .expect("test Automation bundle");
        let path = bundle
            .paths_for_role(ProjectSourceRole::AutomationEntry)
            .next()
            .expect("Automation entry");
        CodeSourceEditorBufferIdentity {
            project_id: app.state.workspace.project.id(),
            bundle_id: bundle.id(),
            bundle_revision: bundle.revision().get(),
            closure_digest: bundle.closure_digest(),
            path: std::sync::Arc::from(path),
        }
    }

    #[test]
    fn unchanged_buffer_reuses_allocation_and_stale_identity_fails_closed() {
        let mut app = RSpiceApp::test_instance();
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let identity = automation_identity(&app);
        let content = take_project_source_editor_buffer(&mut app, language, &identity).unwrap();
        let allocation = content.as_ptr();
        return_project_source_editor_buffer(
            &mut app,
            language,
            &identity,
            content,
            CodeSourceEditorCommit::Unchanged,
        );
        let reused = take_project_source_editor_buffer(&mut app, language, &identity).unwrap();
        assert_eq!(reused.as_ptr(), allocation);
        return_project_source_editor_buffer(
            &mut app,
            language,
            &identity,
            reused,
            CodeSourceEditorCommit::Unchanged,
        );

        app.state
            .workspace
            .replace_project_source_bundle_file(
                identity.bundle_id,
                identity.path.as_ref(),
                "def execute():\n    return 2\n".to_owned(),
            )
            .unwrap();
        assert!(take_project_source_editor_buffer(&mut app, language, &identity).is_none());
        assert!(
            app.state
                .ui
                .code_workspace
                .source_editor_buffer_caches
                .get(&language)
                .is_none()
        );
    }

    #[test]
    fn committed_buffer_rebinds_to_new_revision_and_rejected_buffer_is_discarded() {
        let mut app = RSpiceApp::test_instance();
        let language = ProjectSourceLanguage::RSpiceAutomation;
        let identity = automation_identity(&app);
        let mut content = take_project_source_editor_buffer(&mut app, language, &identity).unwrap();
        content.push_str("\nresult = 3\n");
        app.state
            .workspace
            .replace_project_source_bundle_file(
                identity.bundle_id,
                identity.path.as_ref(),
                content.clone(),
            )
            .unwrap();
        let allocation = content.as_ptr();
        return_project_source_editor_buffer(
            &mut app,
            language,
            &identity,
            content,
            CodeSourceEditorCommit::Committed,
        );
        let revised = automation_identity(&app);
        assert!(revised.bundle_revision > identity.bundle_revision);
        let reused = take_project_source_editor_buffer(&mut app, language, &revised).unwrap();
        assert_eq!(reused.as_ptr(), allocation);

        return_project_source_editor_buffer(
            &mut app,
            language,
            &revised,
            reused,
            CodeSourceEditorCommit::Rejected,
        );
        assert!(
            app.state
                .ui
                .code_workspace
                .source_editor_buffer_caches
                .get(&language)
                .is_none()
        );
    }
}

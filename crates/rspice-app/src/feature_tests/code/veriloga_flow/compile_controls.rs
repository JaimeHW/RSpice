//! The compile controls of the Verilog-A editor, through whole application
//! frames: its keys, its Cancel, its count and what it says about a result it
//! could not use.

use std::sync::{Arc, mpsc};
use std::time::Duration;

use egui::Key;

use super::language_tools::{CTRL_SHIFT, EDITOR, NONE, op_amp, source};
use super::*;
use rspice_app_code::workbench::documents::code_workspace::{
    self, PendingVerilogACompile, VerilogACompileOutcome,
};
use rspice_simulation::project_veriloga::compile_service::CompileCancellation;

const COMPILE: &str = "Compile Verilog-A/AMS (F7)";
const CANCEL: &str = "Cancel Verilog-A/AMS compile";
const STALE: &str = "Result discarded: the source changed";
const DISCARDED: &str = "The source changed during the compile, so its result was discarded.";

/// A compile three seconds in, which answers only when the test sends.
fn running_compile(user: &mut User) -> (mpsc::Sender<VerilogACompileOutcome>, CompileCancellation) {
    let selected =
        code_workspace::selected_veriloga_source_in(&user.app.state.model).expect("the open view");
    let token = selected.token(user.app.state.model.workspace.content.project.id());
    let (sender, receiver) = mpsc::channel();
    let mut pending = PendingVerilogACompile::new(token, receiver, None, Default::default());
    pending.started = web_time::Instant::now() - Duration::from_secs(3);
    let cancellation = pending.cancellation.clone();
    user.app.state.code.code_workspace.veriloga.pending = Some(pending);
    user.settle();
    (sender, cancellation)
}

/// What the strip above the source says.
fn strip(user: &User) -> Option<String> {
    let selected =
        code_workspace::selected_veriloga_source_in(&user.app.state.model).expect("the open view");
    code_workspace::veriloga_compile_status(
        &user.app.state.code.code_workspace.veriloga,
        user.app.state.model.workspace.content.project.id(),
        &selected,
        user.app.state.workbench.messages(),
    )
    .map(|(status, _)| status)
}

fn compiled(user: &User) -> bool {
    user.app
        .state
        .code
        .code_workspace
        .veriloga
        .receipt
        .is_some()
}

fn wait_for_the_result(user: &mut User) {
    user.wait_for("the compile result", |app| {
        let veriloga = &app.state.code.code_workspace.veriloga;
        veriloga.compile_request.is_none() && veriloga.pending.is_none()
    });
    user.settle();
}

#[test]
fn both_build_keys_compile_from_inside_the_editor() {
    for (key, modifiers) in [(Key::F7, NONE), (Key::B, CTRL_SHIFT)] {
        let mut user = op_amp();
        user.click(EDITOR);
        assert!(!compiled(&user));

        user.key(key, modifiers);
        wait_for_the_result(&mut user);

        assert!(compiled(&user), "{key:?} compiled the source");
        assert!(
            source(&user).contains("module"),
            "and typed nothing into it"
        );
    }
}

#[test]
fn a_running_compile_counts_and_cancel_stops_it() {
    let mut user = op_amp();
    user.expect(COMPILE);
    let (_sender, cancellation) = running_compile(&mut user);

    assert_eq!(strip(&user).as_deref(), Some("Compiling… 3 s"));
    assert!(user.find(COMPILE).is_none(), "the control is Cancel now");
    assert!(
        !user
            .app
            .command_key_enabled(CodeCommand::CompileVerilogA.key())
    );

    user.click(CANCEL);

    assert!(
        user.app
            .state
            .code
            .code_workspace
            .veriloga
            .pending
            .is_none()
    );
    assert!(
        rspice_veriloga::PipelineControl::is_cancelled(&cancellation),
        "the work itself is told to stop, not only forgotten"
    );
    assert_eq!(user.prompt(), "Compile cancelled");
    user.expect(COMPILE);
    assert!(
        user.app
            .command_key_enabled(CodeCommand::CompileVerilogA.key())
    );
}

/// Editing under a running compile stops it, and the reader is told why no
/// result arrives. The next compile clears the message.
#[test]
fn an_edit_under_a_running_compile_stops_it_and_says_so() {
    let mut user = op_amp();
    let text = source(&user);
    let (_sender, cancellation) = running_compile(&mut user);

    super::super::veriloga_interface_tests::edit_source(&mut user.app, &format!("{text}// x\n"));
    user.settle();

    assert!(
        user.app
            .state
            .code
            .code_workspace
            .veriloga
            .pending
            .is_none()
    );
    assert!(rspice_veriloga::PipelineControl::is_cancelled(
        &cancellation
    ));
    assert_eq!(user.prompt(), DISCARDED);
    assert_eq!(strip(&user).as_deref(), Some(STALE));

    compile(&mut user);
    assert_ne!(strip(&user).as_deref(), Some(STALE));
    assert_ne!(user.prompt(), DISCARDED);
}

/// A result that lands for a source the reader has since moved off is not
/// published, and is not dropped in silence.
#[test]
fn a_result_for_a_source_that_moved_on_is_discarded_aloud() {
    let mut user = op_amp();
    let (sender, _cancellation) = running_compile(&mut user);
    let selected =
        code_workspace::selected_veriloga_source_in(&user.app.state.model).expect("the open view");
    let path = selected.document().logical_path().to_owned();
    let text = format!("{}// x\n", source(&user));
    user.app
        .state
        .model
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(selected.bundle().id(), &path, text)
        .expect("the file is held");
    sender
        .send(VerilogACompileOutcome::Failure(Vec::new()))
        .expect("the compile is waiting");

    user.settle();
    user.settle();

    assert!(
        user.app
            .state
            .code
            .code_workspace
            .veriloga
            .pending
            .is_none()
    );
    assert_eq!(user.prompt(), DISCARDED);
    assert_eq!(strip(&user).as_deref(), Some(STALE));
}

#[test]
fn a_finished_compile_says_how_long_it_took_until_the_source_changes() {
    let mut user = op_amp();
    compile(&mut user);
    let measured = user
        .app
        .state
        .code
        .code_workspace
        .veriloga
        .last_compile_duration
        .as_mut()
        .expect("the compile was timed");
    measured.1 = Duration::from_millis(1500);
    user.settle();
    assert_eq!(strip(&user).as_deref(), Some("Compiled · 1.5 s"));

    let text = source(&user);
    super::super::veriloga_interface_tests::edit_source(&mut user.app, &format!("{text}// x\n"));
    user.settle();
    assert_ne!(strip(&user).as_deref(), Some("Compiled · 1.5 s"));
}

/// A compile the editor starts that is held in a constant function that
/// never returns (the op-amp with a generate loop bounded by one) ends when
/// Cancel stops it, when an edit voids it and when the project closes: its
/// thread answers instead of spinning on.
#[test]
fn a_compile_held_in_a_constant_function_ends_on_cancel_an_edit_or_closing() {
    const SPIN: &str = "  function integer spin(input integer n);\n    begin spin = n; while (spin > 0) spin = spin + 0; end\n  endfunction\n  localparam integer N = spin(1);\n  genvar i;\n  generate for (i = 0; i < N; i = i + 1) begin : g end endgenerate\n";
    const DECLARED: &str = "  real vin, vslew, vtarget, tau;\n";
    for stop in ["cancel", "edit", "close"] {
        let mut user = op_amp();
        let text = source(&user).replace(DECLARED, &format!("{DECLARED}{SPIN}"));
        assert!(text.contains(SPIN), "the source spins");
        super::super::veriloga_interface_tests::edit_source(&mut user.app, &text);
        user.settle();
        user.click(EDITOR);
        user.key(Key::F7, NONE);
        user.wait_for("the compile to start", |app| {
            app.state.code.code_workspace.veriloga.pending.is_some()
        });
        let receiver = Arc::clone(
            &user
                .app
                .state
                .code
                .code_workspace
                .veriloga
                .pending
                .as_ref()
                .expect("the compile is running")
                .receiver,
        );
        std::thread::sleep(Duration::from_millis(300));
        assert!(
            matches!(
                receiver.lock().expect("the channel").try_recv(),
                Err(mpsc::TryRecvError::Empty)
            ),
            "{stop}: the compile is held in the function"
        );

        match stop {
            "cancel" => user.click(CANCEL),
            "edit" => {
                super::super::veriloga_interface_tests::edit_source(
                    &mut user.app,
                    &format!("{text}// x\n"),
                );
                user.settle();
            }
            _ => {
                assert!(user.app.state.close_project_discard(
                    rspice_app_services::workbench::lifecycle::project_lifecycle::ProjectCloseDestination::EmptyWorkbench,
                ));
                user.settle();
            }
        }

        let answered = receiver
            .lock()
            .expect("the channel")
            .recv_timeout(Duration::from_secs(120));
        assert!(answered.is_ok(), "{stop}: the stopped compile answers");
    }
}

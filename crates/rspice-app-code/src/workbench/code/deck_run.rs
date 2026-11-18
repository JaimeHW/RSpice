//! Run on the Netlist workspace while a deck is what Run runs.
//!
//! The press checks the deck as a run reads it, includes read where they are,
//! and starts the run in the same press when the check finds nothing to stop
//! it. A deck the check stops does not run: the deck comes to the front,
//! where the editor marks its errors, and the notice names the deck and why.
//! A check that would read a Verilog-A source still compiling on another
//! thread waits for that compile beside the frame instead of holding it: the
//! status bar says so, the frame loop runs the deck once the compile ends,
//! and Stop drops it.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::code::CodeFeature;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::run_view::{RunCodeFacts, RunView};
use crate::workbench::slots::{manual_deck_in, veriloga_compilation_in};
use crate::workbench::workflows::netlist_workflow::record_include_resolutions;
use rspice_app_runs::workbench::waiting;

/// Check the deck Run runs and run it, or say why it did not run.
pub fn request_deck_run(cx: &mut Cx<'_, CodeFeature>) {
    cx.runs.waiting.deck_run = false;
    if waiting::compiling() {
        cx.runs.waiting.deck_run = true;
        return;
    }
    // The deck's files are looked at on disk and saved first, so the run
    // reads what their tabs show; a file changed on disk under edits holds
    // the run until the reader answers for it.
    match crate::workbench::documents::deck_files::sync_before_run(
        cx.model,
        &mut cx.state.netlist,
    ) {
        Ok(notices) => {
            for notice in notices {
                cx.wb.prompt(notice);
            }
        }
        Err(why) => {
            cx.wb.prompt(why);
            return;
        }
    }
    let Some((name, text)) =
        manual_deck_in(&cx.state.netlist).map(|deck| (deck.name.to_owned(), deck.text.to_owned()))
    else {
        return;
    };
    let prepared = {
        let code = RunCodeFacts {
            veriloga_compilation: veriloga_compilation_in(&cx.state.code_workspace),
            manual_deck: manual_deck_in(&cx.state.netlist),
        };
        let host = RunView::new(
            &mut *cx.model,
            &mut *cx.runs,
            &mut *cx.wb,
            &*cx.platform,
            &mut cx.peers,
            code,
            cx.commands,
        );
        let controller = &mut *cx.handles.run_controller;
        waiting::unless_compiling(|| controller.prepare_deck_run(&host))
    };
    let Some(prepared) = prepared else {
        cx.runs.waiting.deck_run = true;
        return;
    };
    match prepared {
        Ok(metadata) => {
            record_include_resolutions(
                &mut cx.state.code_workspace.include_resolutions,
                &metadata.sealed_source_dependencies,
            );
            cx.wb.run_press_route = Some(cx.wb.current_route());
            cx.model.simulation.execution.request_manual_deck_run();
        }
        Err(error) => {
            crate::workbench::documents::deck_files::show_deck(cx.model, &mut cx.state.netlist);
            let notice = match failure_line(&error, &text) {
                Some(line) => format!("{name} did not run: line {line}: {error}"),
                None => format!("{name} did not run: {error}"),
            };
            cx.model
                .record(LogSeverity::Error, LogSource::Netlist, notice.clone());
            cx.wb.prompt(notice);
        }
    }
}

/// The line of the deck the check's failure is on: the line it names, or
/// else the card of the instance it names first, as "Q1 (BJT) references
/// unknown model" names Q1.
fn failure_line(
    error: &rspice_simulation::preparation::PreparationError,
    text: &str,
) -> Option<usize> {
    error.line().or_else(|| {
        let instance = error.message().split_whitespace().next()?;
        text.lines()
            .position(|card| {
                card.split_whitespace()
                    .next()
                    .is_some_and(|first| first.eq_ignore_ascii_case(instance))
            })
            .map(|index| index + 1)
    })
}

/// Run the deck whose run waits, once no source its check reads is
/// compiling. Called once per frame; does nothing while the compile runs.
pub fn poll_waiting_deck_run(cx: &mut Cx<'_, CodeFeature>) {
    if cx.runs.waiting.deck_run && !waiting::compiling() {
        request_deck_run(cx);
    }
}

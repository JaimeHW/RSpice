//! A running compile stops where it is told to, not when it is next done.

use rspice_veriloga::{
    CompileError, PipelineCancelled, PipelineControl, PipelinePhase, RuntimeQualificationOptions,
    VerilogACompiler,
};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A module with `count` assignments in a chain, each reading the one before.
fn assignment_chain(count: usize) -> String {
    let declarations = (0..=count)
        .map(|index| format!("x{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let assignments = (1..=count)
        .map(|index| format!("x{index} = x{} + V(a);\n", index - 1))
        .collect::<String>();
    format!(
        "`include \"disciplines.vams\"\nmodule m(a);\ninout a; electrical a;\nreal {declarations};\nanalog begin\nx0 = V(a);\n{assignments}I(a) <+ x{count};\nend\nendmodule\n"
    )
}

struct StopAtPoll {
    polls: AtomicUsize,
    stop_at: usize,
}

impl StopAtPoll {
    fn at(stop_at: usize) -> Self {
        Self {
            polls: AtomicUsize::new(0),
            stop_at,
        }
    }
}

impl PipelineControl for StopAtPoll {
    fn is_cancelled(&self) -> bool {
        self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.stop_at
    }
}

fn compile(source: &str, control: &StopAtPoll) -> Result<(), CompileError> {
    VerilogACompiler::default()
        .compile_runtime_with_qualifications_and_control(
            source,
            Some("m"),
            RuntimeQualificationOptions::NONE,
            control,
        )
        .map(drop)
}

/// How many times a whole compile of `source` asks whether to stop.
fn polls(source: &str) -> usize {
    let control = StopAtPoll::at(usize::MAX);
    compile(source, &control).expect("the module compiles");
    control.polls.load(Ordering::Relaxed)
}

/// The passes over a module's assignments take the bulk of a large compile,
/// so each asks between assignments: more assignments, more places to stop.
#[test]
fn the_long_passes_ask_between_assignments() {
    let few = polls(&assignment_chain(1));
    let many = polls(&assignment_chain(100));
    assert!(
        many >= few + 100,
        "100 more assignments added only {} polls",
        many - few
    );
}

#[test]
fn a_stop_in_the_middle_of_code_generation_ends_the_compile() {
    let source = assignment_chain(100);
    // Past the front end's phase checkpoints, well inside the generator.
    let stop_at = polls(&assignment_chain(1)) + 50;
    let error = compile(&source, &StopAtPoll::at(stop_at)).expect_err("the compile stops");
    assert!(
        matches!(
            error,
            CompileError::Cancelled(PipelineCancelled {
                phase: PipelinePhase::BytecodeGeneration
            })
        ),
        "{error}"
    );
}

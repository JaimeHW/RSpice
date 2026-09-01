//! The canonical state layout, checked against the runtime it has to describe.
//!
//! [`CanonicalStateLayout`] numbers every dynamic operator's state record from
//! the canonical IR alone. These tests pin that numbering against the bytecode
//! generator's own allocation, which is the numbering the shipped runtimes,
//! their checkpoints and the conformance identity pins are all written in.
//!
//! The two spaces are not the same size in general — see the module docs on
//! `canonical_ir::state` for why a noise-shadowed assignment replay makes one
//! canonical site own two bytecode slots — so the pin is stated where they must
//! coincide: a module whose dynamic operators all live in one contribution and
//! whose assignments carry no noise.

use rspice_veriloga::canonical_ir::{
    CanonicalStateFamily, CanonicalStateLayout, CanonicalStateOperator,
};
use rspice_veriloga::codegen::Instruction;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

/// The bytecode slot each instruction names, grouped the way the runtime groups
/// its accepted-state arrays.
///
/// Deliberately written out here rather than reusing the compiler's own
/// `bytecode_slot`: a pin that shares its subject's implementation checks
/// nothing.
fn bytecode_family(instruction: &Instruction) -> Option<(CanonicalStateFamily, usize)> {
    let entry = match instruction {
        Instruction::DdtState(slot)
        | Instruction::IdtState(slot)
        | Instruction::IdtModState(slot)
        | Instruction::LimitState(slot)
        | Instruction::CanonicalLimitState(slot) => (CanonicalStateFamily::Integration, *slot),
        Instruction::AbsDelayState(slot)
        | Instruction::AbsDelayStateMax(slot)
        | Instruction::AbsDelayStateDerivative(slot)
        | Instruction::AbsDelayStateDerivativeMax(slot) => {
            (CanonicalStateFamily::DelayBuffer, *slot)
        }
        Instruction::TransitionState(slot) | Instruction::TransitionStateDerivative(slot) => {
            (CanonicalStateFamily::TransitionFilter, *slot)
        }
        Instruction::SlewState(slot) | Instruction::SlewStateDerivative(slot) => {
            (CanonicalStateFamily::SlewFilter, *slot)
        }
        Instruction::CrossState(slot)
        | Instruction::LastCrossingState(slot)
        | Instruction::AboveState(slot) => (CanonicalStateFamily::CrossDetector, *slot),
        Instruction::LaplaceState(slot) | Instruction::LaplaceStateDerivative(slot) => {
            (CanonicalStateFamily::LaplaceFilter, *slot)
        }
        Instruction::TableLookup(slot) => (CanonicalStateFamily::LookupTable, *slot),
        _ => return None,
    };
    Some(entry)
}

fn bytecode_slots(
    program: &rspice_veriloga::codegen::BytecodeProgram,
    family: CanonicalStateFamily,
) -> Vec<usize> {
    program
        .instructions
        .iter()
        .filter_map(bytecode_family)
        .filter_map(|(found, slot)| (found == family).then_some(slot))
        .collect()
}

/// One contribution holding one of everything the scalar and detector families
/// cover, so the layout and the generator have to agree about all of them at
/// once rather than one family at a time.
const MIXED: &str = r#"
module state_layout_mixed(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ ddt(V(p, n))
             + idt(V(p, n))
             + idtmod(V(p, n), 0.0, 2.0, 0.0)
             + ddt(V(p, n) * 2.0)
             + transition(V(p, n) > 1.0 ? 1.0 : 0.0, 0.0, 1e-9, 1e-9)
             + absdelay(V(p, n), 1e-9)
             + slew(V(p, n), 1e6)
             + cross(V(p, n) - 1.0, 1)
             + above(V(p, n), 1.0)
             + last_crossing(V(p, n) - 2.0, 1);
  end
endmodule
"#;

#[test]
fn layout_numbering_matches_the_bytecode_allocation_family_by_family() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(MIXED).expect("compile bytecode model");
    let artifact = compiler
        .compile_canonical_ir(MIXED)
        .expect("compile canonical IR");
    let layout = CanonicalStateLayout::from_hir(&artifact.hir);
    let program = &model.stamp_programs[0].value_program;

    for family in [
        CanonicalStateFamily::Integration,
        CanonicalStateFamily::DelayBuffer,
        CanonicalStateFamily::TransitionFilter,
        CanonicalStateFamily::SlewFilter,
        CanonicalStateFamily::CrossDetector,
    ] {
        let generator = bytecode_slots(program, family);
        assert!(
            !generator.is_empty(),
            "{family:?} is not exercised by the fixture, so the pin proves nothing"
        );
        // The generator allocates in emission order and the layout numbers in
        // executed-root order. For a single contribution the two are the same
        // walk, so the slots must appear in the same sequence, dense from zero.
        let expected: Vec<usize> = (0..generator.len()).collect();
        assert_eq!(
            generator, expected,
            "{family:?} bytecode slots are not the dense sequence from zero"
        );
        assert_eq!(
            layout.family_len(family),
            generator.len(),
            "{family:?} layout holds {} records where the generator allocated {}",
            layout.family_len(family),
            generator.len()
        );

        let layout_slots: Vec<usize> = layout
            .sites()
            .iter()
            .filter(|site| site.family() == family)
            .map(|site| site.slot as usize)
            .collect();
        assert_eq!(
            layout_slots, generator,
            "{family:?} layout numbering disagrees with the generator's allocation"
        );
    }
}

#[test]
fn every_state_site_is_classified_and_keyed_by_its_operator_expression() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let artifact = compiler
        .compile_canonical_ir(MIXED)
        .expect("compile canonical IR");
    let layout = CanonicalStateLayout::from_hir(&artifact.hir);

    for site in layout.sites() {
        assert_eq!(
            layout.site(site.operator).map(|found| found.slot),
            Some(site.slot),
            "site {site:?} is not reachable by the expression that owns it"
        );
    }

    let kinds: Vec<CanonicalStateOperator> = layout.sites().iter().map(|site| site.kind).collect();
    for expected in [
        CanonicalStateOperator::Ddt,
        CanonicalStateOperator::Idt,
        CanonicalStateOperator::IdtMod,
        CanonicalStateOperator::Transition,
        CanonicalStateOperator::Slew,
        CanonicalStateOperator::Absdelay,
        CanonicalStateOperator::Cross,
        CanonicalStateOperator::Above,
    ] {
        assert!(
            kinds.contains(&expected),
            "the fixture's {} did not reach the layout: {kinds:?}",
            expected.name()
        );
    }
}

/// A module with no dynamic operators owns no records, and says so rather than
/// producing an empty-but-present family the runtime would then size arrays
/// from.
#[test]
fn a_static_module_owns_no_state_records() {
    let source = r#"
module state_layout_static(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n) * 1e-3;
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let artifact = compiler
        .compile_canonical_ir(source)
        .expect("compile canonical IR");
    let layout = CanonicalStateLayout::from_hir(&artifact.hir);
    assert!(layout.sites().is_empty(), "{:?}", layout.sites());
    for operator in CanonicalStateOperator::ALL {
        assert_eq!(layout.family_len(operator.family()), 0);
    }
}

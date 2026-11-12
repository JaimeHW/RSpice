//! Which nodes of a compiled Verilog-A module's instance its DC equations tie
//! together, read from the compiled module for the DC ground-path walk.
//!
//! The walk cannot see inside a module, so every component a module terminal
//! reaches used to stay uncertain: a node only a module's capacitor or its
//! controlled current reaches could not be proven to have no DC path, and the
//! operating point held every node through the floor instead of that one.

use rspice_veriloga::CompiledModel;
use rspice_veriloga::codegen::{BytecodeProgram, Instruction, StampIndex};

use crate::netlist::{Element, ModuleDcConduction};

/// The DC conduction `model` gives an instance card with `node_count` nodes;
/// `None` when the compiled module does not say.
///
/// A module's nodes are its terminals, its internal nodes and ground. Each
/// contribution ties nodes as the walk ties a deck element's:
///
/// - A potential contribution, an indirect one or one that reads a branch
///   current, and every source branch, conducts like a voltage source: it
///   ties every node it names.
/// - A current contribution whose DC derivatives name a voltage of its own
///   branch conducts, and ties its branch to every node its value reads. A
///   derivative that is a `ddt()` operand's alone vanishes at DC
///   ([`vanishes_at_dc`]) and names nothing.
/// - Any other current contribution is a controlled current, open between
///   its terminals as a VCCS's output is, and its sense nodes draw nothing;
///   a contribution whose only derivatives vanish at DC carries nothing
///   there, as a capacitor does.
///
/// A digital module with no analog equations ties nothing: its ports reach
/// the analog side only through the converters the build inserts.
///
/// Filters and switch branches keep their own states, which these rules do
/// not read, so a module with any is not answered.
pub(super) fn module_dc_conduction(
    model: &CompiledModel,
    has_digital_content: bool,
    node_count: usize,
) -> Option<ModuleDcConduction> {
    if !model.laplace_filters.is_empty()
        || !model.zi_filters.is_empty()
        || !model.switch_branch_variables.is_empty()
    {
        return None;
    }
    let analog = !model.stamp_programs.is_empty() || !model.branch_sources.is_empty();
    if has_digital_content {
        // A digital port can span several card nodes, which the terminal
        // numbering below does not map.
        return (!analog).then(ModuleDcConduction::default);
    }
    let terminals = model.num_terminals;
    if node_count != terminals {
        return None;
    }
    let ground = terminals + model.internal_nodes;
    let index = |node: &StampIndex| match node {
        StampIndex::Terminal(terminal) => (*terminal < terminals).then_some(*terminal),
        StampIndex::Internal(internal) => {
            (*internal < model.internal_nodes).then_some(terminals + internal)
        }
        StampIndex::Ground => Some(ground),
        StampIndex::Branch(_) => None,
    };
    let mut parent: Vec<usize> = (0..=ground).collect();
    fn root(parent: &mut [usize], mut node: usize) -> usize {
        while parent[node] != node {
            parent[node] = parent[parent[node]];
            node = parent[node];
        }
        node
    }
    let mut tie = |nodes: &[usize]| {
        if let Some((&first, rest)) = nodes.split_first() {
            for &node in rest {
                let (left, right) = (root(&mut parent, first), root(&mut parent, node));
                parent[right] = left;
            }
        }
    };
    for source in &model.branch_sources {
        let ends: Option<Vec<usize>> = [&source.pos, &source.neg].into_iter().map(index).collect();
        tie(&ends?);
    }
    for program in &model.stamp_programs {
        // A location's column is the right-hand side's placeholder; its row
        // is an end of the contribution's branch.
        let branch: Vec<&StampIndex> = program
            .stamp_locations
            .iter()
            .map(|location| &location.row)
            .collect();
        let dc_jacobian: Vec<_> = program
            .jacobian_programs
            .iter()
            .filter(|entry| !vanishes_at_dc(&entry.program))
            .collect();
        let jacobian: Vec<&StampIndex> = dc_jacobian
            .iter()
            .flat_map(|entry| [&entry.row, &entry.col])
            .collect();
        let reads_branch = branch
            .iter()
            .chain(&jacobian)
            .any(|node| matches!(node, StampIndex::Branch(_)));
        let named: Vec<usize> = branch
            .iter()
            .chain(&jacobian)
            .filter_map(|node| index(node))
            .collect();
        if program.branch_ordinal.is_some() || program.indirect || reads_branch {
            tie(&named);
            continue;
        }
        let ends: Vec<usize> = branch.iter().filter_map(|node| index(node)).collect();
        let conducts = dc_jacobian
            .iter()
            .filter_map(|entry| index(&entry.col))
            .any(|column| ends.contains(&column));
        if conducts {
            tie(&named);
        }
    }
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); ground + 1];
    for terminal in 0..terminals {
        members[root(&mut parent, terminal)].push(terminal);
    }
    let ground_root = root(&mut parent, ground);
    let grounded = std::mem::take(&mut members[ground_root]);
    Some(ModuleDcConduction {
        tied: members.into_iter().filter(|set| set.len() > 1).collect(),
        grounded,
    })
}

/// Whether a derivative program is zero at DC whatever its operands: a
/// `ddt()` value or derivative, a zero constant, or sums, products, quotients
/// and negations that keep one. Any other instruction, a jump among them,
/// is taken to be nonzero.
fn vanishes_at_dc(program: &BytecodeProgram) -> bool {
    let mut stack: Vec<bool> = Vec::new();
    for instruction in &program.instructions {
        let vanishes = match instruction {
            Instruction::PushConst(value) => *value == 0.0,
            Instruction::PushParam(_)
            | Instruction::PushParamGiven(_)
            | Instruction::PushVoltage(..)
            | Instruction::PushInternalVoltage(_)
            | Instruction::PushVariable(_)
            | Instruction::PushTemperature
            | Instruction::PushVt
            | Instruction::PushMfactor => false,
            Instruction::Neg => match stack.pop() {
                Some(operand) => operand,
                None => return false,
            },
            Instruction::DdtState(_) => {
                if stack.pop().is_none() {
                    return false;
                }
                true
            }
            Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::DdtDerivativeState(_) => {
                let (Some(right), Some(left)) = (stack.pop(), stack.pop()) else {
                    return false;
                };
                match instruction {
                    Instruction::Add | Instruction::Sub => left && right,
                    Instruction::Mul => left || right,
                    Instruction::Div => left,
                    _ => true,
                }
            }
            _ => return false,
        };
        stack.push(vanishes);
    }
    stack.len() == 1 && stack[0]
}

/// [`module_dc_conduction`] for an instance card of a cached module.
pub(super) fn cached_module_dc_conduction(
    entry: &super::veriloga_cache::CachedVerilogAModel,
    element: &Element,
) -> Option<ModuleDcConduction> {
    let has_digital_content = entry
        .canonical_ir
        .as_deref()
        .is_some_and(|artifact| artifact.digital.has_executable_content());
    module_dc_conduction(&entry.model, has_digital_content, element.nodes.len())
}

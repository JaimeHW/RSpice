//! Whether a Verilog-A module's contributions connect one of its electrical
//! ports to node 0.
//!
//! A deck that names no node 0 lets the simulator take one of its nets as the
//! reference, but only when nothing in it connects node 0 already. A module
//! does when a branch it stamps, a contribution or source (`I(a) <+`,
//! `V(a) <+`), or a flow it probes on a branch nothing drives, which is an
//! ideal short, reaches one of its electrical ports single-ended or through a
//! net it declares `ground`. A read does not: `V(a)` in an expression, a
//! Verilog-AMS process's analog read or a task's argument stamps nothing in
//! node 0's row or column.
//!
//! The answer is computed once per module, from its canonical IR, by the same
//! code for a module compiled at run time and for one compiled ahead of time
//! into a generated model, which carries it as static metadata. Each instance
//! then decides it with its own parameter values, so a `.va` file and the
//! generated model built from it give the same answer for the same card.
//!
//! ## Joins and their guards
//!
//! Every branch joins the nets it connects into one group; a single-ended
//! branch joins its net's group to ground, which is group 0. A port reaches
//! node 0 when a chain of joins connects its group to group 0.
//!
//! A branch that sits inside an `if` or `case` exists only when its
//! conditions hold, so its join carries them as a guard. A guard the instance
//! can decide before it solves reads only literals, public parameters,
//! `$param_given` and `$port_connected`, directly or through a variable the
//! module last set from those alone. Anything else, a probe, a variable the
//! solution moves or a simulator query, is [`GroundGuardOp::Unknown`], and a
//! join whose guard is unknown counts: the build never takes a reference it
//! cannot justify.

/// One step of a guard, a postfix program over a stack of values.
///
/// A value is a number or unknown; a condition is true when its value is not
/// zero. Logical operators follow Kleene's three-valued logic, so `unknown &&
/// false` is false, and every other operator yields unknown from an unknown
/// operand.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroundGuardOp<'a> {
    /// A literal.
    Number(f64),
    /// A public parameter's value on the instance, by its declared name.
    Parameter(&'a str),
    /// `$param_given` of a public parameter.
    ParameterGiven(&'a str),
    /// `$port_connected` of a port, by its index in the module's port list.
    PortConnected(u32),
    /// A value the instance cannot know before it solves.
    Unknown,
    /// Logical negation.
    Not,
    /// Arithmetic negation.
    Negate,
    /// A real converted to an `integer`: the nearest one, halves away from
    /// zero, and unknown outside the 32-bit range, where the module refuses
    /// it.
    Integer,
    Add,
    Subtract,
    Multiply,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    /// `condition ? then : else`, pushed in that order.
    Select,
}

/// A join between two groups that happens only when its guard holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroundJoin {
    /// The groups it joins; group 0 is ground.
    pub groups: [u32; 2],
    /// Where its guard sits in [`GroundReferenceGraph::guards`]: start and
    /// length.
    pub guard: [u32; 2],
}

/// The joins that remain to decide once every unconditional join is made.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroundReferenceGraph<'a> {
    /// How many groups the unconditional joins leave, ground included.
    pub group_count: u32,
    /// The groups that hold an electrical port.
    pub port_groups: &'a [u32],
    pub joins: &'a [GroundJoin],
    pub guards: &'a [GroundGuardOp<'a>],
}

/// Whether a module's contributions connect an electrical port to node 0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroundReference<'a> {
    /// No port reaches ground, whatever the instance's values.
    Never,
    /// A port reaches ground through joins that always happen.
    Always,
    /// A port reaches ground only through guarded joins.
    Conditional(GroundReferenceGraph<'a>),
}

/// What an instance knows before it solves. `None` is unknown.
pub trait GroundReferenceInputs {
    /// A public parameter's effective value, by declared name.
    fn parameter(&self, name: &str) -> Option<f64>;
    /// Whether the card gave a public parameter.
    fn parameter_given(&self, name: &str) -> Option<bool>;
    /// Whether the card connects a port, as `$port_connected` answers it on
    /// this instance's route.
    fn port_connected(&self, port: u32) -> Option<bool>;
}

impl GroundReference<'_> {
    /// Whether this instance's contributions connect an electrical port to
    /// node 0.
    pub fn references_ground(&self, inputs: &dyn GroundReferenceInputs) -> bool {
        match self {
            Self::Never => false,
            Self::Always => true,
            Self::Conditional(graph) => graph.references_ground(inputs),
        }
    }
}

impl GroundReferenceGraph<'_> {
    /// Make every join whose guard is true or unknown, then ask whether a
    /// port's group reaches ground.
    pub fn references_ground(&self, inputs: &dyn GroundReferenceInputs) -> bool {
        let mut parent: Vec<u32> = (0..self.group_count.max(1)).collect();
        fn root(parent: &mut [u32], mut group: u32) -> u32 {
            while parent[group as usize] != group {
                let next = parent[parent[group as usize] as usize];
                parent[group as usize] = next;
                group = next;
            }
            group
        }
        let group_count = parent.len();
        let in_range = |group: u32| (group as usize) < group_count;
        for join in self.joins {
            let [left, right] = join.groups;
            if !in_range(left) || !in_range(right) {
                // A graph that names a group it does not have cannot be read,
                // and counts.
                return true;
            }
            let [start, len] = join.guard;
            let guard = (start as usize)
                .checked_add(len as usize)
                .and_then(|end| self.guards.get(start as usize..end));
            if guard.is_some_and(|guard| evaluate_guard(guard, inputs) == Some(false)) {
                continue;
            }
            let (left, right) = (root(&mut parent, left), root(&mut parent, right));
            let (low, high) = (left.min(right), left.max(right));
            parent[high as usize] = low;
        }
        self.port_groups
            .iter()
            .any(|&group| !in_range(group) || root(&mut parent, group) == root(&mut parent, 0))
    }
}

/// A guard's truth on one instance: `None` when it cannot be known before the
/// solve, or the program is malformed.
pub fn evaluate_guard(
    program: &[GroundGuardOp<'_>],
    inputs: &dyn GroundReferenceInputs,
) -> Option<bool> {
    let truth = |value: Option<f64>| value.map(|value| value != 0.0);
    let number = |truth: bool| Some(if truth { 1.0 } else { 0.0 });
    let mut stack: Vec<Option<f64>> = Vec::with_capacity(program.len());
    for op in program {
        let value = match *op {
            GroundGuardOp::Number(value) => Some(value),
            GroundGuardOp::Parameter(name) => inputs.parameter(name),
            GroundGuardOp::ParameterGiven(name) => inputs.parameter_given(name).and_then(number),
            GroundGuardOp::PortConnected(port) => inputs.port_connected(port).and_then(number),
            GroundGuardOp::Unknown => None,
            GroundGuardOp::Not => truth(stack.pop()?).and_then(|value| number(!value)),
            GroundGuardOp::Negate => stack.pop()?.map(|value| -value),
            GroundGuardOp::Integer => stack
                .pop()?
                .and_then(|value| super::integer::real_to_integer(value).ok().map(f64::from)),
            GroundGuardOp::Select => {
                let otherwise = stack.pop()?;
                let then = stack.pop()?;
                match truth(stack.pop()?) {
                    Some(true) => then,
                    Some(false) => otherwise,
                    None => then.filter(|_| then == otherwise),
                }
            }
            GroundGuardOp::And | GroundGuardOp::Or => {
                let right = truth(stack.pop()?);
                let left = truth(stack.pop()?);
                let (absorbing, other) = (*op == GroundGuardOp::Or, *op != GroundGuardOp::Or);
                if left == Some(absorbing) || right == Some(absorbing) {
                    number(absorbing)
                } else if left == Some(other) && right == Some(other) {
                    number(other)
                } else {
                    None
                }
            }
            binary => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                match (left, right) {
                    (Some(left), Some(right)) => match binary {
                        GroundGuardOp::Add => Some(left + right),
                        GroundGuardOp::Subtract => Some(left - right),
                        GroundGuardOp::Multiply => Some(left * right),
                        GroundGuardOp::Equal => number(left == right),
                        GroundGuardOp::NotEqual => number(left != right),
                        GroundGuardOp::Less => number(left < right),
                        GroundGuardOp::LessEqual => number(left <= right),
                        GroundGuardOp::Greater => number(left > right),
                        GroundGuardOp::GreaterEqual => number(left >= right),
                        _ => return None,
                    },
                    _ => None,
                }
            }
        };
        stack.push(value);
    }
    match stack.as_slice() {
        [value] => truth(*value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Card {
        mode: Option<f64>,
        connected: bool,
    }

    impl GroundReferenceInputs for Card {
        fn parameter(&self, name: &str) -> Option<f64> {
            (name == "mode").then_some(self.mode).flatten()
        }
        fn parameter_given(&self, name: &str) -> Option<bool> {
            (name == "mode").then_some(self.mode.is_some())
        }
        fn port_connected(&self, port: u32) -> Option<bool> {
            (port == 0).then_some(self.connected)
        }
    }

    fn card(mode: Option<f64>) -> Card {
        Card {
            mode,
            connected: true,
        }
    }

    use GroundGuardOp as Op;

    #[test]
    fn a_guard_reads_the_instances_values_in_three_valued_logic() {
        let mode_is_two = [Op::Parameter("mode"), Op::Number(2.0), Op::Equal];
        assert_eq!(evaluate_guard(&mode_is_two, &card(Some(2.0))), Some(true));
        assert_eq!(evaluate_guard(&mode_is_two, &card(Some(1.0))), Some(false));
        assert_eq!(evaluate_guard(&mode_is_two, &card(None)), None);

        // `unknown && false` is false; `unknown || true` is true.
        let and = [Op::Unknown, Op::Parameter("mode"), Op::And];
        assert_eq!(evaluate_guard(&and, &card(Some(0.0))), Some(false));
        assert_eq!(evaluate_guard(&and, &card(Some(1.0))), None);
        let or = [Op::Unknown, Op::Parameter("mode"), Op::Or];
        assert_eq!(evaluate_guard(&or, &card(Some(1.0))), Some(true));
        assert_eq!(evaluate_guard(&or, &card(Some(0.0))), None);

        let given = [Op::ParameterGiven("mode"), Op::Not];
        assert_eq!(evaluate_guard(&given, &card(None)), Some(true));
        let connected = [Op::PortConnected(0)];
        let open = Card {
            mode: None,
            connected: false,
        };
        assert_eq!(evaluate_guard(&connected, &open), Some(false));

        // 1.5 assigned to an integer is 2.
        let rounded = [
            Op::Parameter("mode"),
            Op::Integer,
            Op::Number(2.0),
            Op::Equal,
        ];
        assert_eq!(evaluate_guard(&rounded, &card(Some(1.5))), Some(true));
        // An unknown condition selects a value only when both arms agree.
        let select = [Op::Unknown, Op::Number(3.0), Op::Number(3.0), Op::Select];
        assert_eq!(evaluate_guard(&select, &card(None)), Some(true));
        // A malformed program cannot be read.
        assert_eq!(evaluate_guard(&[Op::And], &card(None)), None);
    }

    #[test]
    fn a_conditional_port_reaches_ground_only_when_its_joins_hold() {
        // Group 1 holds the port. It reaches ground through group 2 when
        // `mode == 2`, and group 2 is ground only when the port is connected.
        let guards = [
            Op::Parameter("mode"),
            Op::Number(2.0),
            Op::Equal,
            Op::PortConnected(0),
        ];
        let joins = [
            GroundJoin {
                groups: [1, 2],
                guard: [0, 3],
            },
            GroundJoin {
                groups: [2, 0],
                guard: [3, 1],
            },
        ];
        let reference = GroundReference::Conditional(GroundReferenceGraph {
            group_count: 3,
            port_groups: &[1],
            joins: &joins,
            guards: &guards,
        });
        assert!(reference.references_ground(&card(Some(2.0))));
        assert!(!reference.references_ground(&card(Some(1.0))));
        // Unknown counts.
        assert!(reference.references_ground(&card(None)));
        let open = Card {
            mode: Some(2.0),
            connected: false,
        };
        assert!(!reference.references_ground(&open));
        assert!(!GroundReference::Never.references_ground(&card(None)));
        assert!(GroundReference::Always.references_ground(&card(None)));
    }
}

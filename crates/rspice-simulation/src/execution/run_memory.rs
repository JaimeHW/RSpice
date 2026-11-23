//! What the reader's own deck may hold on the machine it is checked and run
//! on.
//!
//! The engine ships fixed limits for what one run may read and materialize.
//! They guard a service against input it did not write, and stay there: the
//! cloud's engine and `rspice --shipped-limits` keep them. The reader's own
//! deck on their own machine is bounded by that machine instead, as Spectre,
//! HSPICE, ngspice, LTspice and QSPICE bound it: every limit on the size of
//! what the deck is and holds is sized to the memory installed, at what one
//! unit of it costs, measured. The Netlist page's check and a run read the
//! same limits, [`deck_limits`], so neither refuses what the other takes.
//!
//! The limits that are not about size stay as the engine ships them: how
//! deep includes and subcircuits nest, how many events one mixed-signal step
//! has, how many threads a run uses and how many runs one analysis makes.
//! They stop cycles, runaways and typing slips, and the other tools have
//! them too.
//!
//! Once a deck is flattened its devices say what its run needs, and
//! [`check_fits`] weighs that against the machine: a run that needs more
//! than the memory installed is refused before it starts, and one that needs
//! more than the memory free but fits the memory installed is the reader's
//! to start, the system making room by paging other programs out.
//!
//! A browser tab cannot say how much memory its machine has; its run has
//! wasm32's 4 GB of address space, and its limits are sized to that.

mod machine;

pub use machine::machine_memory;
#[cfg(any(test, feature = "test-support"))]
pub use machine::{forget_machine_memory_here, pretend_machine_memory_here};

use crate::preparation::{PreparationError, PreparationStage};
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};

/// The memory of the machine a deck is checked and run on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineMemory {
    /// The memory installed, which bounds the largest design the machine
    /// can run. A run's limits are sized to it, so they do not move from
    /// one preparation to the next.
    pub installed_bytes: u64,
    /// The memory free as the run is prepared, which is what this run can
    /// have without the system paging other programs out.
    pub free_bytes: u64,
}

/// What one device of the flattened deck costs a run at its peak, from the
/// deck's flattening through the solved circuit and its first results.
///
/// Measured on the desktop app running a design of 32^4 resistors placed
/// through four levels of hierarchy: 1,082,401 devices peaked at 5.0 GB.
pub(crate) const RUN_BYTES_PER_DEVICE: u64 = 5 * 1024;

/// What one circuit node, or one unknown of the system the run solves, costs
/// a run at its peak: its name and index, its row of the matrix and its
/// entries in every solution and history vector.
pub(crate) const RUN_BYTES_PER_UNKNOWN: u64 = 4 * 1024;

/// What one retained result value costs: the sample, its place in a column
/// and the column's share of what indexes it.
const RUN_BYTES_PER_RESULT_VALUE: u64 = 16;

/// What one byte of deck text costs the app at its peak while a run of the
/// deck is prepared: the editor's text, the deck with its includes read in,
/// the sealed copy the run executes and what the parse keeps of each line's
/// text.
///
/// Measured on the release app preparing, from the Netlist workspace, a
/// 100 MB deck of comment lines 100 characters long (1.0 M lines, no
/// devices): it peaked at 6.6 bytes for each byte, once its lines are
/// counted at [`RUN_BYTES_PER_SOURCE_LINE`].
const RUN_BYTES_PER_SOURCE_BYTE: u64 = 7;

/// What one line of deck text costs beyond its bytes while a run of the
/// deck is prepared: its entry in the line index of each copy, and its
/// place among the parse's logical lines. A device line costs its device
/// besides, at [`RUN_BYTES_PER_DEVICE`].
///
/// Measured on the release app preparing, from the Netlist workspace, a
/// 100 MB deck of 50 M two-character comment lines: 170 bytes a line, its
/// bytes at [`RUN_BYTES_PER_SOURCE_BYTE`] taken out.
const RUN_BYTES_PER_SOURCE_LINE: u64 = 170;

/// What one byte of a data file a source reads (a `PWL file=`, a code
/// model's table) costs the run that reads it: the file's text while it is
/// parsed.
///
/// Measured on the release app running a deck whose one source reads a
/// 100 MB PWL file of 6.3 M points written with 24 digits apiece: it peaked
/// at 1.1 bytes for each byte of the file, its values counted at
/// [`RUN_BYTES_PER_DATA_VALUE`].
const RUN_BYTES_PER_DATA_BYTE: u64 = 2;

/// What one value of a data file costs the run that reads it: the parsed
/// value, its share of the point it is in, and the copy the shared cache
/// keeps between runs.
///
/// Measured on the release app running a deck whose one source reads a
/// 100 MB PWL file of 25 M points written as short integers: 34 bytes a
/// value, the file's bytes at [`RUN_BYTES_PER_DATA_BYTE`] taken out.
const RUN_BYTES_PER_DATA_VALUE: u64 = 34;

/// The share of the memory installed the shared caches may keep between
/// runs: one byte in this many. The caches hold what runs read so that a
/// later run, or the next point of a sweep, does not read it again. What
/// they cannot hold is read again rather than refused, so the share bounds
/// what the app keeps idle, not what a run may read: a sixteenth leaves an
/// 8 GB machine the 512 MB the engine ships and gives a 32 GB one 2 GB.
const SHARED_CACHE_SHARE: u64 = 16;

/// The memory a browser tab's run can address: wasm32's 4 GB.
const BROWSER_MEMORY_BYTES: u64 = 4 * 1024 * 1024 * 1024;

/// How the reader's own deck is bounded in one field of [`ResourceLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldPolicy {
    /// Sized to the memory installed: as many units as that memory pays for
    /// at `bytes_each` a unit, and never fewer than the engine ships.
    Sized { bytes_each: u64 },
    /// Kept as the engine ships it: the guard is not about size.
    Kept,
}

/// Every field of [`ResourceLimits`], by the resource it limits, and how the
/// reader's own deck is bounded in it, with why.
pub const POLICY: [(ResourceKind, FieldPolicy); 17] = {
    use FieldPolicy::{Kept, Sized};
    [
        // The root deck's text. Post-layout decks run to hundreds of MB and
        // millions of lines, and the other tools read any that fits.
        (
            ResourceKind::NetlistBytes,
            Sized {
                bytes_each: RUN_BYTES_PER_SOURCE_BYTE,
            },
        ),
        (
            ResourceKind::NetlistLines,
            Sized {
                bytes_each: RUN_BYTES_PER_SOURCE_LINE,
            },
        ),
        // The deck with its includes read in, and the files it includes: the
        // same text, read and parsed the same way, so the same cost.
        (
            ResourceKind::ExpandedSourceBytes,
            Sized {
                bytes_each: RUN_BYTES_PER_SOURCE_BYTE,
            },
        ),
        (
            ResourceKind::DependencySourceBytes,
            Sized {
                bytes_each: RUN_BYTES_PER_SOURCE_BYTE,
            },
        ),
        // The data files the deck's sources read: a long PWL stimulus is as
        // much the reader's own deck as its text.
        (
            ResourceKind::ExternalDataBytes,
            Sized {
                bytes_each: RUN_BYTES_PER_DATA_BYTE,
            },
        ),
        (
            ResourceKind::ExternalDataValues,
            Sized {
                bytes_each: RUN_BYTES_PER_DATA_VALUE,
            },
        ),
        // What the caches keep between runs: a share of the machine, as
        // SHARED_CACHE_SHARE says why.
        (
            ResourceKind::SharedCacheBytes,
            Sized {
                bytes_each: SHARED_CACHE_SHARE,
            },
        ),
        // How deep includes nest: a cycle guard. A deck that includes itself
        // nests without end, and no deck needs 64 levels.
        (ResourceKind::IncludeDepth, Kept),
        // How deep subcircuits nest: a recursion guard, as include depth is.
        (ResourceKind::HierarchyDepth, Kept),
        // What the flattened deck holds, and the system its run solves.
        (
            ResourceKind::FlattenedElements,
            Sized {
                bytes_each: RUN_BYTES_PER_DEVICE,
            },
        ),
        (
            ResourceKind::CircuitNodes,
            Sized {
                bytes_each: RUN_BYTES_PER_UNKNOWN,
            },
        ),
        (
            ResourceKind::MatrixUnknowns,
            Sized {
                bytes_each: RUN_BYTES_PER_UNKNOWN,
            },
        ),
        // The values one result keeps.
        (
            ResourceKind::ResultValues,
            Sized {
                bytes_each: RUN_BYTES_PER_RESULT_VALUE,
            },
        ),
        // The histories the deck's delay lines keep: bytes the budget counts
        // as the memory they take, spare capacity included, so one byte
        // each.
        (ResourceKind::TransportHistoryBytes, Sized { bytes_each: 1 }),
        // How many events one step of a mixed-signal run has: a runaway
        // guard. A zero-delay loop between an observer and the circuit has
        // no end, whatever the machine.
        (ResourceKind::MixedIntervalEvents, Kept),
        // How many threads a run uses: the reader's own setting, which the
        // host's cores bound, not its memory.
        (ResourceKind::ParallelWorkers, Kept),
        // How many runs one analysis makes: a guard against a sweep typed
        // with a step a thousand times too fine. Each run's results are
        // bounded by the values they keep, which is sized.
        (ResourceKind::BatchRuns, Kept),
    ]
};

/// The field of `limits` that limits `kind`; `None` for a resource no field
/// limits on its own.
fn field(limits: &mut ResourceLimits, kind: ResourceKind) -> Option<&mut usize> {
    Some(match kind {
        ResourceKind::NetlistBytes => &mut limits.max_netlist_bytes,
        ResourceKind::NetlistLines => &mut limits.max_netlist_lines,
        ResourceKind::ExpandedSourceBytes => &mut limits.max_expanded_source_bytes,
        ResourceKind::DependencySourceBytes => &mut limits.max_dependency_source_bytes,
        ResourceKind::ExternalDataBytes => &mut limits.max_external_data_bytes,
        ResourceKind::ExternalDataValues => &mut limits.max_external_data_values,
        ResourceKind::SharedCacheBytes => &mut limits.max_shared_cache_bytes,
        ResourceKind::IncludeDepth => &mut limits.max_include_depth,
        ResourceKind::HierarchyDepth => &mut limits.max_hierarchy_depth,
        ResourceKind::FlattenedElements => &mut limits.max_flattened_elements,
        ResourceKind::CircuitNodes => &mut limits.max_circuit_nodes,
        ResourceKind::MatrixUnknowns => &mut limits.max_matrix_unknowns,
        ResourceKind::ResultValues => &mut limits.max_result_values,
        ResourceKind::TransportHistoryBytes => &mut limits.max_transport_history_bytes,
        ResourceKind::MixedIntervalEvents => &mut limits.max_mixed_interval_events,
        ResourceKind::ParallelWorkers => &mut limits.max_parallel_workers,
        ResourceKind::BatchRuns => &mut limits.max_batch_runs,
        // Each delay line's own history is bounded by the bytes they all
        // keep together.
        ResourceKind::TransportHistoryRecords => return None,
    })
}

/// `field` of `limits`, read.
fn value(mut limits: ResourceLimits, kind: ResourceKind) -> Option<usize> {
    field(&mut limits, kind).map(|value| *value)
}

/// The limits the reader's own deck is checked and run under where it is:
/// on a machine whose `memory` is known, every limit on its size sized to
/// the memory installed; in a browser, to wasm32's address space; on a host
/// that cannot say, as the engine ships them.
#[must_use]
pub fn deck_limits(memory: Option<MachineMemory>) -> ResourceLimits {
    let shipped = ResourceLimits::default();
    match memory {
        Some(memory) => sized_to(shipped, memory.installed_bytes),
        None if cfg!(target_arch = "wasm32") => sized_to(shipped, BROWSER_MEMORY_BYTES),
        None => shipped,
    }
}

/// `limits` with every field [`POLICY`] sizes sized to `installed` bytes of
/// memory. A field never falls below the value it has: a deck those admit
/// is refused, if at all, by [`check_fits`], which says why.
pub(crate) fn sized_to(mut limits: ResourceLimits, installed: u64) -> ResourceLimits {
    for (kind, policy) in POLICY {
        let (FieldPolicy::Sized { bytes_each }, Some(limit)) = (policy, field(&mut limits, kind))
        else {
            continue;
        };
        let fits = usize::try_from(installed / bytes_each).unwrap_or(usize::MAX);
        *limit = (*limit).max(fits);
    }
    limits
}

/// How a run of a flattened deck meets the machine it is prepared on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryFit {
    /// It fits the memory free.
    Fits,
    /// It needs more than the memory free and fits the memory installed: it
    /// runs once the reader says to, the system paging other programs out
    /// to make room.
    Warn(MemoryWarning),
    /// It needs more than the memory installed, which no paging makes room
    /// for: refused before it starts.
    Refuse(PreparationError),
}

/// A run that needs more than the memory free and fits the memory installed:
/// what it needs, and what the machine has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryWarning {
    /// The devices of the flattened deck.
    pub devices: usize,
    /// The memory the run needs, about.
    pub needed_bytes: u64,
    /// The machine's memory as the run was prepared.
    pub memory: MachineMemory,
}

impl MemoryWarning {
    /// What the run needs and what the machine has, in one sentence, and
    /// what running it anyway does.
    #[must_use]
    pub fn message(&self) -> String {
        format!(
            "This design has {} devices. Running it needs about {} of memory, and this \
             computer has {} free of {}. It can run, but other programs slow while it does.",
            grouped(self.devices),
            bytes(self.needed_bytes),
            bytes(self.memory.free_bytes),
            bytes(self.memory.installed_bytes)
        )
    }

    /// The same in the few words a status line has room for.
    #[must_use]
    pub fn status(&self) -> String {
        format!(
            "Needs about {} of memory · {} free",
            bytes(self.needed_bytes),
            bytes(self.memory.free_bytes)
        )
    }
}

/// Weigh a run of `devices` against `memory`: it fits what is free, needs
/// more than is free and fits what is installed, or needs more than is
/// installed.
pub fn check_fits(devices: usize, memory: MachineMemory) -> MemoryFit {
    let needed = (devices as u64).saturating_mul(RUN_BYTES_PER_DEVICE);
    if needed <= memory.free_bytes {
        return MemoryFit::Fits;
    }
    if needed <= memory.installed_bytes {
        return MemoryFit::Warn(MemoryWarning {
            devices,
            needed_bytes: needed,
            memory,
        });
    }
    MemoryFit::Refuse(PreparationError::new(
        PreparationStage::SourceChecks,
        format!(
            "This design has {} devices. Running it needs about {} of memory, more than the {} \
             this computer has. Run it on a computer with more memory.",
            grouped(devices),
            bytes(needed),
            bytes(memory.installed_bytes)
        ),
    ))
}

/// `error` in the reader's words when it went past a limit sized to where
/// the deck runs, as [`sized_refusal`] says them; else as it was. It stays a
/// resource limit.
pub(crate) fn too_big_for(
    error: PreparationError,
    memory: Option<MachineMemory>,
) -> PreparationError {
    match error
        .resource_limit()
        .and_then(|limit| sized_refusal(limit, memory))
    {
        Some(words) => error.sized_as(words),
        None => error,
    }
}

/// What `refusal` refused, in the reader's words, when the limit it went
/// past is one [`deck_limits`] sized to where the deck runs: on a machine,
/// that reading or running it needs more memory than the machine has; in a
/// browser, that it is too large for the browser and the desktop app runs
/// it. `None` for a limit the engine ships unchanged, which its catalogue
/// sentence says.
#[must_use]
pub fn sized_refusal(
    refusal: &ResourceLimitError,
    memory: Option<MachineMemory>,
) -> Option<String> {
    said_where(
        refusal,
        memory,
        memory.is_none() && cfg!(target_arch = "wasm32"),
    )
}

/// [`sized_refusal`] where the deck runs on `memory`, or in a browser when
/// `browser`.
fn said_where(
    refusal: &ResourceLimitError,
    memory: Option<MachineMemory>,
    browser: bool,
) -> Option<String> {
    let policy = POLICY
        .iter()
        .find(|(kind, _)| *kind == refusal.resource)
        .map(|(_, policy)| *policy)?;
    if policy == FieldPolicy::Kept {
        return None;
    }
    let limits = if browser {
        sized_to(ResourceLimits::default(), BROWSER_MEMORY_BYTES)
    } else {
        deck_limits(memory)
    };
    let sized = value(limits, refusal.resource)?;
    let shipped = value(ResourceLimits::default(), refusal.resource)?;
    // A limit the deck's place left as shipped, or one a caller set itself,
    // is a fixed cap, which its catalogue sentence says.
    if refusal.limit != sized || (!browser && sized == shipped) {
        return None;
    }
    let subject = subject(refusal)?;
    Some(match memory {
        _ if browser => format!(
            "{subject}, too large for the browser, which can take at most {}. The desktop app \
             runs it.",
            amount(refusal.resource, refusal.limit)
        ),
        Some(memory) => format!(
            "{subject}. {} it needs more than the {} of memory this computer has. Run it on a \
             computer with more memory.",
            verb(refusal.resource),
            bytes(memory.installed_bytes)
        ),
        None => return None,
    })
}

/// What went past its limit and how large it is: "This deck is 1.2 GB".
fn subject(refusal: &ResourceLimitError) -> Option<String> {
    // The engine stops counting at the first unit past the limit, so a
    // request one past it is said as more than the limit.
    let requested = if refusal.requested == refusal.limit.saturating_add(1) {
        format!("more than {}", amount(refusal.resource, refusal.limit))
    } else {
        amount(refusal.resource, refusal.requested)
    };
    Some(match refusal.resource {
        ResourceKind::NetlistBytes => format!("This deck is {requested}"),
        ResourceKind::NetlistLines => format!("This deck has {requested} lines"),
        ResourceKind::ExpandedSourceBytes => {
            format!("This deck comes to {requested} with the files it includes read in")
        }
        ResourceKind::DependencySourceBytes => {
            format!("The files this deck includes come to {requested}")
        }
        ResourceKind::ExternalDataBytes => format!("A data file a source reads is {requested}"),
        ResourceKind::ExternalDataValues => {
            format!("A data file a source reads holds {requested} values")
        }
        ResourceKind::SharedCacheBytes => {
            format!("The compiled devices kept between runs come to {requested}")
        }
        ResourceKind::FlattenedElements => format!("This design has {requested} devices"),
        ResourceKind::CircuitNodes => format!("This design has {requested} nodes"),
        ResourceKind::MatrixUnknowns => {
            format!("This design has {requested} unknowns to solve for")
        }
        ResourceKind::ResultValues => format!("This result would hold {requested} values"),
        ResourceKind::TransportHistoryBytes => {
            format!("The delay lines' histories need {requested}")
        }
        ResourceKind::IncludeDepth
        | ResourceKind::HierarchyDepth
        | ResourceKind::TransportHistoryRecords
        | ResourceKind::MixedIntervalEvents
        | ResourceKind::ParallelWorkers
        | ResourceKind::BatchRuns => return None,
    })
}

/// What it is that needs the memory: reading text and data, or running.
fn verb(resource: ResourceKind) -> &'static str {
    match resource {
        ResourceKind::NetlistBytes
        | ResourceKind::NetlistLines
        | ResourceKind::ExpandedSourceBytes
        | ResourceKind::DependencySourceBytes
        | ResourceKind::ExternalDataBytes
        | ResourceKind::ExternalDataValues => "Reading",
        ResourceKind::SharedCacheBytes => "Keeping",
        _ => "Running",
    }
}

/// `count` of `resource` as a person reads it: bytes as memory, anything
/// else as a count.
fn amount(resource: ResourceKind, count: usize) -> String {
    match resource {
        ResourceKind::NetlistBytes
        | ResourceKind::ExpandedSourceBytes
        | ResourceKind::DependencySourceBytes
        | ResourceKind::ExternalDataBytes
        | ResourceKind::SharedCacheBytes
        | ResourceKind::TransportHistoryBytes => bytes(count as u64),
        _ => grouped(count),
    }
}

/// `count` with its thousands grouped, `1,048,577`.
fn grouped(count: usize) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (at, digit) in digits.chars().enumerate() {
        if at > 0 && (digits.len() - at).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// `count` bytes as the operating system states memory, `3.1 GB`.
fn bytes(count: u64) -> String {
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * MB;
    let count = count as f64;
    if count >= GB {
        format!("{:.1} GB", count / GB)
    } else {
        format!("{:.0} MB", (count / MB).max(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GB: u64 = 1024 * 1024 * 1024;

    /// The policy table names every field of the limits once, and each field
    /// it sizes follows the memory installed at its cost, while each it
    /// keeps stays as shipped however much memory there is.
    #[test]
    fn every_field_is_sized_or_kept_as_the_table_says() {
        let shipped = ResourceLimits::default();
        let mut named = Vec::new();
        for kind in POLICY.iter().map(|(kind, _)| *kind) {
            assert!(!named.contains(&kind), "{kind} is named once");
            named.push(kind);
        }
        // Every field is reached through one kind: a field the table missed
        // keeps the value it had.
        let mut reached = ResourceLimits::unlimited();
        for kind in &named {
            *field(&mut reached, *kind).expect("a field") = 0;
        }
        let fields = serde_json::to_value(reached).expect("the limits serialize");
        let fields = fields.as_object().expect("a table of fields");
        assert_eq!(fields.len(), POLICY.len(), "{fields:?}");
        assert!(
            fields.values().all(|value| *value == 0),
            "every field of the limits is in the table: {fields:?}"
        );

        let sized = sized_to(shipped, 64 * GB);
        for (kind, policy) in POLICY {
            let (was, now) = (value(shipped, kind).unwrap(), value(sized, kind).unwrap());
            match policy {
                FieldPolicy::Sized { bytes_each } => {
                    assert_eq!(now as u64, (64 * GB / bytes_each).max(was as u64), "{kind}");
                    assert!(now > was, "{kind} grows on a 64 GB machine");
                }
                FieldPolicy::Kept => assert_eq!(now, was, "{kind} stays as shipped"),
            }
        }
    }

    /// The limits a check and a run read: sized on a machine that says its
    /// memory, as shipped on a desktop host that cannot.
    #[test]
    fn a_deck_is_bounded_by_the_machine_it_is_on() {
        let memory = MachineMemory {
            installed_bytes: 32 * GB,
            free_bytes: 8 * GB,
        };
        let limits = deck_limits(Some(memory));
        assert_eq!(limits, sized_to(ResourceLimits::default(), 32 * GB));
        assert!(limits.max_netlist_bytes > 1024 * 1024 * 1024, "{limits:?}");
        assert_eq!(
            limits.max_flattened_elements as u64,
            32 * GB / RUN_BYTES_PER_DEVICE
        );
        // How much is free moves from one preparation to the next; the
        // limits do not.
        assert_eq!(
            deck_limits(Some(MachineMemory {
                free_bytes: GB,
                ..memory
            })),
            limits
        );
        #[cfg(not(target_arch = "wasm32"))]
        assert_eq!(deck_limits(None), ResourceLimits::default());
    }

    /// The browser's run has wasm32's 4 GB of address space, and its limits
    /// are sized to that.
    #[test]
    fn a_browser_is_bounded_by_its_address_space() {
        let browser = sized_to(ResourceLimits::default(), BROWSER_MEMORY_BYTES);
        #[cfg(target_arch = "wasm32")]
        assert_eq!(deck_limits(None), browser);
        assert_eq!(
            browser.max_netlist_bytes as u64,
            BROWSER_MEMORY_BYTES / RUN_BYTES_PER_SOURCE_BYTE
        );
        assert_eq!(
            browser.max_flattened_elements as u64,
            BROWSER_MEMORY_BYTES / RUN_BYTES_PER_DEVICE
        );
        assert_eq!(
            browser.max_include_depth,
            ResourceLimits::default().max_include_depth
        );
    }

    /// A small machine never narrows a limit below the shipped one: a run
    /// it cannot hold is refused in words, by what it needs, instead.
    #[test]
    fn a_small_machine_keeps_the_shipped_limits() {
        assert_eq!(
            sized_to(ResourceLimits::default(), GB / 4),
            ResourceLimits::default()
        );
    }

    /// A run that fits what is free runs; one between free and installed
    /// warns with the numbers; one past installed is refused in words.
    #[test]
    fn a_run_fits_warns_or_is_refused_by_the_memory_it_needs() {
        let memory = MachineMemory {
            installed_bytes: 16 * GB,
            free_bytes: 3 * GB,
        };
        assert_eq!(check_fits(1000, memory), MemoryFit::Fits);
        let MemoryFit::Warn(warning) = check_fits(1_048_577, memory) else {
            panic!("five gigabytes fit in sixteen, not in three");
        };
        assert_eq!(
            warning.message(),
            "This design has 1,048,577 devices. Running it needs about 5.0 GB of memory, and \
             this computer has 3.0 GB free of 16.0 GB. It can run, but other programs slow \
             while it does."
        );
        assert_eq!(
            warning.status(),
            "Needs about 5.0 GB of memory · 3.0 GB free"
        );
        let small = MachineMemory {
            installed_bytes: GB,
            free_bytes: GB / 2,
        };
        let MemoryFit::Refuse(refused) = check_fits(250_000, small) else {
            panic!("1.2 GB does not fit in 1 GB");
        };
        assert_eq!(
            refused.message(),
            "This design has 250,000 devices. Running it needs about 1.2 GB of memory, more than \
             the 1.0 GB this computer has. Run it on a computer with more memory."
        );
    }

    #[test]
    fn a_deck_that_outgrew_its_count_says_the_machine_cannot_hold_it() {
        let memory = MachineMemory {
            installed_bytes: 16 * GB,
            free_bytes: 3 * GB,
        };
        let limit = deck_limits(Some(memory)).max_flattened_elements;
        let error = PreparationError::from_parse(
            PreparationStage::ModelBindings,
            "Executable hierarchy cannot authenticate project model use",
            rspice_core::netlist::ParseWithAbortError::Parse(
                rspice_core::netlist::ParseError::ResourceLimit(ResourceLimitError {
                    resource: ResourceKind::FlattenedElements,
                    requested: limit + 1,
                    limit,
                }),
            ),
        );
        let refused = too_big_for(error, Some(memory));
        assert_eq!(
            refused.message(),
            "This design has more than 3,355,443 devices. Running it needs more than the 16.0 GB \
             of memory this computer has. Run it on a computer with more memory."
        );
        assert!(
            refused.resource_limit().is_some(),
            "it stays a resource limit"
        );
        assert_eq!(
            refused.catalogued_limit(),
            None,
            "its words are the reader's"
        );
    }

    /// A deck past a sized limit is said to need more memory than the
    /// machine has; past a fixed one, or one a caller set, it keeps the
    /// engine's catalogue sentence.
    #[test]
    fn only_a_sized_limit_is_said_in_memory_words() {
        let memory = MachineMemory {
            installed_bytes: 32 * GB,
            free_bytes: 8 * GB,
        };
        let sized = deck_limits(Some(memory)).max_netlist_bytes;
        let past = |limit: usize| ResourceLimitError {
            resource: ResourceKind::NetlistBytes,
            requested: limit + 100 * 1024 * 1024,
            limit,
        };
        let words = sized_refusal(&past(sized), Some(memory)).expect("a sized limit");
        assert!(
            words.starts_with("This deck is ")
                && words.ends_with(
                    ". Reading it needs more than the 32.0 GB of memory this computer has. Run \
                     it on a computer with more memory."
                ),
            "{words}"
        );
        let shipped = ResourceLimits::default().max_netlist_bytes;
        assert_eq!(sized_refusal(&past(shipped), Some(memory)), None);
        assert_eq!(sized_refusal(&past(sized - 1), Some(memory)), None);
        let depth = ResourceLimitError {
            resource: ResourceKind::IncludeDepth,
            requested: 65,
            limit: 64,
        };
        assert_eq!(sized_refusal(&depth, Some(memory)), None);
        #[cfg(not(target_arch = "wasm32"))]
        assert_eq!(sized_refusal(&past(shipped), None), None);
    }

    /// A deck past the browser's sized limit is too large for the browser,
    /// and the desktop app runs it; past a limit the browser keeps fixed, it
    /// keeps the catalogue sentence.
    #[test]
    fn a_deck_too_large_for_the_browser_is_sent_to_the_desktop_app() {
        let browser = sized_to(ResourceLimits::default(), BROWSER_MEMORY_BYTES);
        let past = ResourceLimitError {
            resource: ResourceKind::NetlistBytes,
            requested: browser.max_netlist_bytes + 100 * 1024 * 1024,
            limit: browser.max_netlist_bytes,
        };
        let words = said_where(&past, None, true).expect("a sized limit");
        assert!(
            words.starts_with("This deck is ")
                && words.ends_with(&format!(
                    ", too large for the browser, which can take at most {}. The desktop app \
                     runs it.",
                    bytes(browser.max_netlist_bytes as u64)
                )),
            "{words}"
        );
        let depth = ResourceLimitError {
            resource: ResourceKind::IncludeDepth,
            requested: 65,
            limit: 64,
        };
        assert_eq!(said_where(&depth, None, true), None);
    }
}

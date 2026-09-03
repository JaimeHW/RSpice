use super::*;

#[cfg(feature = "veriloga")]
// Bump whenever a persisted runtime artifact or its integrity contract changes.
// Version 16 adds model/instance/dual parameter-storage semantics to canonical IR.
// Version 17 adds exact legacy Laplace Jacobian semantics.
// Version 18 retains exact transfer-function DC actions in compiled Laplace filters.
// Version 19 adds signed slew IR, runtime, and checkpoint semantics.
// Version 20 adds transactional event-state variable metadata.
// Version 21 retracts front-end acceptance: reserved Verilog-AMS digital
// keywords, discrete-discipline `posedge`/`negedge`, and `analog final` are now
// refused. The record carries no compiler identity of its own, so this constant
// is the only thing standing between a stale artifact and a source the current
// compiler will not compile.
// Version 22 extends front-end acceptance the other way: the IEEE 1364 digital
// subset (`always`, `initial`, `reg`, `wire`, `assign`, four-state literals,
// vector ranges) now parses and is semantically analyzed, so the same source
// text produces a different parse than version 21 did.
// Version 23 gives the digital subset a canonical form: `always` and
// `initial` processes lower to CFG functions with a `Wait` terminator, and the
// artifact carries them in a new `digital` field. The same source that
// version 22 refused at the artifact boundary now produces an artifact, so the
// record's identity has to change even though a continuous-domain model's
// artifact is byte-identical to the one version 22 wrote.
// Version 24 widens that canonical form. Process-local variables, `for`,
// `while`, `repeat`, and the wildcard `case` forms lower where version 23
// refused them, a continuous assignment lowers to a driver process and a new
// `drivers` list, and two IEEE 1364-2005 defects are corrected: a
// concatenation target now resizes its right-hand side (section 5.2.1) and a
// case item is matched by identity rather than by `==` (section 9.5). Source
// that version 23 refused now compiles, and source it compiled can produce a
// different artifact, so the record's identity has to change.
// Version 25 elaborates a digital module hierarchy. A module that instantiates
// another module with discrete-domain content was refused by version 24; it now
// flattens into one plan whose signals carry instance-qualified names and whose
// processes and drivers carry per-instance identities. Source that version 24
// refused compiles, and a hierarchy's artifact is a different artifact, so the
// record's identity has to change.
// Version 26 adds generalized transient integration and exact accepted history
// to compiled Laplace filters. Cached version-25 realizations do not carry the
// older state and derivative lanes required by trapezoidal and Gear-2 stepping.
// Version 27 widens the digital front end at the declaration boundary. A port
// with no net declaration of its own is now the implicit net of IEEE 1364-2005
// section 12.3.3 rather than an undeclared name; `output reg [3:0] q;` is now
// section 12.3.4's compact spelling of the two-declaration form; a packed range
// and a replication count may name a parameter, which section 12.2 fixes at
// elaboration; and the eight gate primitives of section 7.2 lower to the same
// drivers their operator spellings do. Source that version 26 refused now
// compiles, and a structural design's artifact is a different artifact, so the
// record's identity has to change.
// Version 28 closes the last of the digital-Verilog language gaps the oracle
// corpus named. The bitwise XNOR of IEEE 1364-2005 section 4.1.9 in both its
// spellings, the case equality of section 4.1.8, and the reduction operators of
// section 4.1.10 all lex, parse and lower; a generate region of section 12.4 is
// unrolled at elaboration time; a port may be connected to a bit- or
// part-select of a net (section 12.3.9); a child module's own parameters are
// fixed at their declared defaults (section 12.2) instead of being refused; and
// a sized literal keeps the width its author wrote (section 3.5.1) rather than
// widening to thirty-two. Source that version 27 refused now compiles, and
// source it compiled can produce a different artifact — a concatenation holding
// a sized literal is a different width and therefore a different value — so the
// record's identity has to change.
// Version 29 sizes a digital expression by its context, IEEE 1364-2005 section
// 5.4.1. The assignment's left-hand side is part of the expression's context,
// so the operands of a context-determined operator are extended to the width of
// the largest expression including the target and the operation runs at that
// width; an unsized literal takes a context wider than its 32-bit floor. A
// version-28 artifact computed `a * b` at the operand width and widened the
// product afterwards, which is a different number rather than a narrower one —
// `{cout, sum} = a + b` had a `cout` that could never be 1 — so every cached
// digital artifact has to be rebuilt and the record's identity has to change.
// Version 30 signs a digital expression, IEEE 1364-2005 section 5.4.2. A net,
// variable or port declared `signed`, an `integer`, a plain decimal number and
// a literal carrying the `s` base marker are signed; a bit-select, a
// part-select and a concatenation are not, and one unsigned operand makes the
// whole expression unsigned. A signed operand is sign-extended to its context
// instead of zero-extended, a comparison between two signed operands is made
// on signed numbers, division truncates toward zero with the modulus following
// its first operand, and `>>>` shifts in the sign bit. A version-29 artifact
// carried every one of those declarations and read none of them, so it
// compiled a `reg signed` into an unsigned device: `p = a` widened -1 into 15
// and `a < 0` was false for every value. `4'd9` also decodes to its declared
// four bits now rather than to the 32-bit unsized floor. Both change the value
// an artifact computes, so every cached digital artifact has to be rebuilt and
// the record's identity has to change.
// Version 31 groups each syntactic Verilog-A noise primitive as one coherent
// process and serializes its derivative-shadow replay plus every signed circuit
// injection. A version-30 artifact has neither the schema marker nor those
// routing programs, so accepting it would silently revert repeated uses of one
// process to independent sources and lose cancellation/correlation.
// Version 32 gives the discrete domain a second value type: Verilog-AMS
// LRM 2.4 section 3.7's real net. `wreal`, and the four resolved spellings
// beside it, declare a net carrying a real rather than four-state bits; a
// plan signal now records which, a real net starts at 0.0 rather than at
// `z`, and real arithmetic, real comparison and a real conditional are
// three new value kinds a process function can hold. A version-31 artifact
// could not contain any of them — `wreal` was refused at the keyword — so
// nothing cached is being reinterpreted here. What changes is the *shape*
// of the record: a plan serialized by version 32 carries a field version
// 31's reader does not know, and a version-31 plan decodes under version 32
// only because that field defaults. Rebuilding rather than leaning on the
// default is the fail-closed reading, so the record's identity changes.
// Version 33 gives the discrete domain a real *variable*, which is where a
// real-number model keeps state. `real` is now a discrete-domain signal class
// beside `wreal` — written procedurally rather than driven, per IEEE 1364-2005
// section 6.2 — reached either by `output real` or by a module-level `real` a
// process writes in a module with no analog block. `$realtobits` and
// `$bitstoreal` are two more value kinds, a `parameter real` folds into a real
// expression, and a deferred nonblocking update carries either a four-state
// value or a real. A version-32 artifact could contain none of that: every one
// of those constructs was refused by name, so nothing cached is being
// reinterpreted. What changes is the shape of the record again — a plan
// serialized by version 33 can carry a real variable and a deferred real update
// that version 32's reader has no case for — so the record's identity changes
// rather than relying on a decode that would silently drop them.
// Version 34 reserves the six Verilog-AMS connect keywords — `connect`,
// `connectrules`, `endconnectrules`, `resolveto`, `merged` and `split` — and
// reads the two constructs they belong to: `connectmodule`, which LRM 2.4
// Syntax 7-4 makes a third `module_keyword` and which the parser refused
// outright before, and the `connectrules` block of Syntax 7-5. It also reads
// `` `default_discipline `` (section 10.2), which every module now carries.
//
// Unlike versions 32 and 33, this is not a new shape inside the plan: no
// executable form of a connect module exists yet, so nothing new reaches a
// cached artifact. What changes is what the *front end accepts* — a source
// that was a hard parse error under version 33 compiles under 34, and six
// identifiers that were legal names are not any more. A cached artifact
// therefore no longer stands for the same compile, and the fail-closed
// reading is to rebuild rather than to reason about which sources are
// unaffected.
//
// Version 35 is version 34's situation again, for one line of the front end.
// A compiler directive the preprocessor does not know now takes its operand
// with it, per the preprocessor's own rule that a line opening with a backtick
// is a directive line. Under version 34 the operand stayed in the token
// stream, so `` `default_nettype wire `` was a hard parse error — the `wire`
// reached the top-level item loop alone — and under 35 the whole line is
// dropped and the file compiles. That is the same kind of change as 34's:
// nothing in a cached plan is reinterpreted, but a cached artifact no longer
// stands for the same compile, and the fail-closed reading is to rebuild.
//
// Version 36 is that situation once more, and this time the change is on *this*
// side of the boundary rather than the compiler's: a `.VERILOGA` include is now
// compiled with `enable_ams` on (see [`deck_include_compiler_options`]), so a
// `.va` whose module carries digital content compiles instead of being refused
// at code generation, and reaches the mixed host that executes it.
//
// Nothing cached is being reinterpreted, and for a stronger reason than 34's
// and 35's. A mixed module under version 35 did not compile at all, so no
// version-35 record for one exists to be misread; and for an analog-only module
// the option changes nothing an artifact can carry, because its entire effect
// in the compiler is to skip a check that an analog-only module passes. What
// changes is once again what the front end accepts, so a cached artifact no
// longer stands for the same compile, and the fail-closed reading is to rebuild
// rather than to reason about which sources are unaffected.
//
// Version 37 changes the shape of a cached plan and what the front end
// accepts, both at once, for the same construct: Verilog-AMS LRM 2.4 section
// 7.3.3's probe of a continuous net from a discrete context. A plan now carries
// an analog-probe table and process functions carry a value kind that reads
// one, neither of which a version-36 reader has a case for; and a process that
// wrote `V(p, n)` was refused by name under 36, so a source that did not
// compile now does. It also moves the ownership rule for a module-level `real`
// off the *module* and onto the *name*, per section 7.3's "Write operations of
// nets and variables are only allowed from the context of their domain", which
// makes a third class of source compile that did not: one whose analog body
// neither reads nor writes the variable a process owns.
//
// Nothing cached is reinterpreted — every construct involved was refused under
// 36, so no version-36 record contains one — but a cached artifact no longer
// stands for the same compile, and the fail-closed reading is to rebuild.
//
// Version 38 follows the canonical IR's own schema from 12 to 13. The HIR now
// carries `HirExecutedCorrespondence`: which executed expression each
// structured-body expression is a second lowering of, which is what lets a
// CFG-sourced backend name the state record an operator owns. A version-37
// record predates the map and would deserialize with an empty one, and an empty
// correspondence is not "no operators" — it is "every operator unmapped", which
// a CFG-sourced consumer must refuse. Rebuilding is both the fail-closed and the
// only useful reading. `CANONICAL_IR_SCHEMA_VERSION` would refuse such a record
// on its own; this constant moves with it so the refusal happens at the cache
// boundary, where the diagnostic names the cache file.
//
// Version 39 re-indexes the compiled model itself. The bytecode generator
// numbered a module's analog-operator records once per *emission*; the compiler
// now rewrites every slot index into the canonical per-*site* numbering before
// the model leaves `compile_file_runtime_with_metadata`. A version-38 record
// holds a `CompiledModel` whose `DdtState`, `CrossState`, `LaplaceState` and
// sibling instructions carry the emission numbers, and nothing in the record
// says which numbering it is in. Deserializing one beside a runtime that
// allocates per site would read a record at one index and write it at another,
// so the record has to be rebuilt rather than reinterpreted.
//
// Version 40 changes what a compiled model's Zi placeholder means. A cached
// version-39 model records every Zi site's unfrozen placeholder as a one-by-one
// filter regardless of the coefficient lists the source declares; the site's
// real widths only appeared once an evaluation froze the per-instance values.
// A transient resume validates a checkpoint against a rebuilt device before it
// has evaluated anything, so a version-39 placeholder reports the wrong shape
// at exactly the point the comparison is made. The widths are syntactic, so
// they now travel with the compiled model. Rebuilding is the only reading: the
// old record's placeholder shape is not recoverable from the record.
// Version 41 changes which value an equation reads. Through version 40 the
// bytecode's stamp entries read a variable's slot after the whole assignment
// pass had run, so an equation over a scratch variable a later statement
// reassigns read the last write rather than the definition reaching the point
// the contribution was written at. The compiler now redirects such a read to a
// snapshot slot filled where the definition reaches. A version-40 record holds
// a `CompiledModel` with neither the snapshot variables nor the redirected
// stamp programs, and nothing in the record distinguishes one written before
// the repair from one written after — reading it would silently stamp the
// defective value on the VM and JIT routes. Rebuilding is the only reading.
// Version 42 carries the same repair onto the JIT and WASM routes. A version-41
// record holds the snapshot slots and the redirected stamp programs but not the
// plan saying where each copy belongs in the statement sequence, and the
// executable routes replay statements rather than steps: they would run a
// version-41 model with the copies never made and the equations still reading
// the slot the assignment pass finishes with. The plan is not recoverable from
// the record — it names the statement each definition was written at, and the
// record keeps no statement sequence. Rebuilding is the only reading.
pub(super) const VERILOGA_CACHE_RECORD_VERSION: u32 = 42;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_FILE: &str = ".rspice-veriloga-cache.lock";
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_WAIT_TIMEOUT: Duration = Duration::from_secs(15);
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_STALE_TIMEOUT: Duration = Duration::from_secs(180);
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_POLL_INTERVAL: Duration = Duration::from_millis(25);
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_DEFAULT_MAX_ENTRIES: usize = 512;
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_DEFAULT_MAX_BYTES: u64 = 512 * 1024 * 1024;
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_MAX_ENTRIES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_ENTRIES";
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_MAX_BYTES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_BYTES";

/// On-disk Verilog-A cache statistics.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogACacheStats {
    /// Cache root directory.
    pub root: PathBuf,
    /// Number of persisted records.
    pub entry_count: usize,
    /// Total persisted bytes.
    pub total_bytes: u64,
    /// Active maximum entry budget.
    pub max_entries: usize,
    /// Active maximum byte budget.
    pub max_bytes: u64,
}

/// Process-lifetime telemetry for runtime Verilog-A cache lookups and compiles.
///
/// Counters are monotonic and lock-free. Take two snapshots to measure an
/// interval without globally resetting data used by another simulation.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VerilogACacheTelemetry {
    /// All attempted runtime lookups, including cancelled and invalid requests.
    pub lookups: u64,
    /// Fresh entries served from the bounded in-memory cache.
    pub memory_hits: u64,
    /// In-memory entries rejected after dependency fingerprint validation.
    pub stale_memory_entries: u64,
    /// Fresh, integrity-checked records restored from disk.
    pub disk_hits: u64,
    /// Lookups for which neither cache tier supplied an entry.
    pub misses: u64,
    /// Compiler invocations entered after a cache miss.
    pub compilations_started: u64,
    /// Compiler invocations that produced both runtime artifacts.
    pub compilations_succeeded: u64,
    /// Compiler invocations that returned a non-cancellation error.
    pub compilations_failed: u64,
    /// Compiler invocations stopped through cooperative cancellation.
    pub compilations_cancelled: u64,
    /// Saturating sum of compiler wall time across all completed invocations.
    pub total_compilation_nanos: u64,
    /// Compiled entries retained in memory but not persisted successfully.
    pub persistence_failures: u64,
}

#[cfg(feature = "veriloga")]
struct VerilogACacheTelemetryCounters {
    lookups: std::sync::atomic::AtomicU64,
    memory_hits: std::sync::atomic::AtomicU64,
    stale_memory_entries: std::sync::atomic::AtomicU64,
    disk_hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
    compilations_started: std::sync::atomic::AtomicU64,
    compilations_succeeded: std::sync::atomic::AtomicU64,
    compilations_failed: std::sync::atomic::AtomicU64,
    compilations_cancelled: std::sync::atomic::AtomicU64,
    total_compilation_nanos: std::sync::atomic::AtomicU64,
    persistence_failures: std::sync::atomic::AtomicU64,
}

#[cfg(feature = "veriloga")]
impl VerilogACacheTelemetryCounters {
    const fn new() -> Self {
        Self {
            lookups: std::sync::atomic::AtomicU64::new(0),
            memory_hits: std::sync::atomic::AtomicU64::new(0),
            stale_memory_entries: std::sync::atomic::AtomicU64::new(0),
            disk_hits: std::sync::atomic::AtomicU64::new(0),
            misses: std::sync::atomic::AtomicU64::new(0),
            compilations_started: std::sync::atomic::AtomicU64::new(0),
            compilations_succeeded: std::sync::atomic::AtomicU64::new(0),
            compilations_failed: std::sync::atomic::AtomicU64::new(0),
            compilations_cancelled: std::sync::atomic::AtomicU64::new(0),
            total_compilation_nanos: std::sync::atomic::AtomicU64::new(0),
            persistence_failures: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

#[cfg(feature = "veriloga")]
static VERILOGA_CACHE_TELEMETRY: VerilogACacheTelemetryCounters =
    VerilogACacheTelemetryCounters::new();

/// Snapshot process-lifetime cache and compiler telemetry.
#[cfg(feature = "veriloga")]
pub fn veriloga_cache_telemetry() -> VerilogACacheTelemetry {
    use std::sync::atomic::Ordering::Relaxed;

    VerilogACacheTelemetry {
        lookups: VERILOGA_CACHE_TELEMETRY.lookups.load(Relaxed),
        memory_hits: VERILOGA_CACHE_TELEMETRY.memory_hits.load(Relaxed),
        stale_memory_entries: VERILOGA_CACHE_TELEMETRY.stale_memory_entries.load(Relaxed),
        disk_hits: VERILOGA_CACHE_TELEMETRY.disk_hits.load(Relaxed),
        misses: VERILOGA_CACHE_TELEMETRY.misses.load(Relaxed),
        compilations_started: VERILOGA_CACHE_TELEMETRY.compilations_started.load(Relaxed),
        compilations_succeeded: VERILOGA_CACHE_TELEMETRY
            .compilations_succeeded
            .load(Relaxed),
        compilations_failed: VERILOGA_CACHE_TELEMETRY.compilations_failed.load(Relaxed),
        compilations_cancelled: VERILOGA_CACHE_TELEMETRY
            .compilations_cancelled
            .load(Relaxed),
        total_compilation_nanos: VERILOGA_CACHE_TELEMETRY
            .total_compilation_nanos
            .load(Relaxed),
        persistence_failures: VERILOGA_CACHE_TELEMETRY.persistence_failures.load(Relaxed),
    }
}

/// A single Verilog-A cache entry from disk.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogACacheEntry {
    /// Serialized cache file location.
    pub cache_path: PathBuf,
    /// Canonical source file path.
    pub source_path: PathBuf,
    /// Canonical dependency list used for freshness checks.
    pub dependencies: Vec<PathBuf>,
    /// Serialized file size.
    pub size_bytes: u64,
    /// Record modification timestamp.
    pub modified_ns: Option<u128>,
}

/// Result of a cache prune/clear operation.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogACachePruneReport {
    /// Number of removed records.
    pub removed_entries: usize,
    /// Total bytes reclaimed.
    pub reclaimed_bytes: u64,
    /// Final cache stats after pruning.
    pub stats: VerilogACacheStats,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct VerilogADependencyFingerprint {
    pub(super) canonical_path: PathBuf,
    pub(super) modified_ns: Option<u128>,
    pub(super) file_len: u64,
    pub(super) content_hash: [u8; 32],
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct VerilogADiskCacheRecord {
    pub(super) version: u32,
    pub(super) source_path: PathBuf,
    pub(super) dependencies: Vec<VerilogADependencyFingerprint>,
    pub(super) model: rspice_veriloga::CompiledModel,
    pub(super) canonical_ir: Option<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub(super) struct CachedVerilogAModel {
    // Every production constructor validates the model/artifact pair before
    // this entry is admitted to the in-memory cache. Cache hits therefore only
    // need the comparatively cheap dependency freshness check.
    pub(super) dependencies: Vec<VerilogADependencyFingerprint>,
    pub(super) model: std::sync::Arc<rspice_veriloga::CompiledModel>,
    pub(super) canonical_ir:
        Option<std::sync::Arc<rspice_veriloga::canonical_ir::CanonicalIrArtifact>>,
}

#[cfg(feature = "veriloga")]
type VerilogAModelCache = crate::resource::BoundedCache<PathBuf, CachedVerilogAModel>;

#[cfg(feature = "veriloga")]
#[derive(Serialize)]
struct BorrowedVerilogACacheRecord<'a> {
    version: u32,
    source_path: &'a Path,
    dependencies: &'a [VerilogADependencyFingerprint],
    model: &'a rspice_veriloga::CompiledModel,
    canonical_ir: Option<&'a rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
}

#[cfg(feature = "veriloga")]
impl<'a> BorrowedVerilogACacheRecord<'a> {
    fn new(source_path: &'a Path, entry: &'a CachedVerilogAModel) -> Self {
        Self {
            version: VERILOGA_CACHE_RECORD_VERSION,
            source_path,
            dependencies: &entry.dependencies,
            model: entry.model.as_ref(),
            canonical_ir: entry.canonical_ir.as_deref(),
        }
    }
}

#[cfg(feature = "veriloga")]
#[derive(Default)]
struct CountingWriter {
    bytes: usize,
}

#[cfg(feature = "veriloga")]
impl std::io::Write for CountingWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.bytes = self.bytes.saturating_add(bytes.len());
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "veriloga")]
fn veriloga_model_cache_entry_bytes(
    key: &Path,
    entry: &CachedVerilogAModel,
) -> Result<usize, String> {
    let record = BorrowedVerilogACacheRecord::new(key, entry);
    let mut counter = CountingWriter::default();
    serde_json::to_writer(&mut counter, &record)
        .map_err(|error| format!("failed to size Verilog-A cache entry: {error}"))?;
    let key_bytes = key.to_string_lossy().len();
    Ok(crate::resource::estimated_cache_entry_bytes::<
        PathBuf,
        CachedVerilogAModel,
    >(key_bytes, counter.bytes))
}

#[cfg(feature = "veriloga")]
fn retain_veriloga_model(
    key: PathBuf,
    entry: CachedVerilogAModel,
    max_bytes: usize,
    required: bool,
) -> Result<bool, String> {
    let retained_bytes = veriloga_model_cache_entry_bytes(&key, &entry)?;
    if let Err(error) =
        ResourceLimitError::ensure(ResourceKind::SharedCacheBytes, retained_bytes, max_bytes)
    {
        if required {
            return Err(error.to_string());
        }
        return Ok(false);
    }

    let mut cache = veriloga_model_cache()
        .write()
        .map_err(|_| "failed to acquire Verilog-A cache lock".to_owned())?;
    cache.enforce_limit(max_bytes);
    cache.remove(&key);
    cache.insert_or_get(key.clone(), entry, retained_bytes, max_bytes);
    let retained = cache.get(&key).is_some();
    if required && !retained {
        return Err(format!(
            "unable to retain Verilog-A runtime '{}' in the shared cache",
            key.display()
        ));
    }
    Ok(retained)
}

/// Verify that cached bytecode and canonical IR can safely be paired at runtime.
///
/// A persisted entry is an optimization, never an authority. In particular,
/// older compiler builds can deserialize after an IR-digest change while still
/// looking structurally valid to serde. Treat that as a cache miss here rather
/// than allowing a stale artifact to reach the native JIT.
#[cfg(feature = "veriloga")]
fn validate_runtime_artifact_pair(
    model: &rspice_veriloga::CompiledModel,
    canonical_ir: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<(), String> {
    #[cfg(feature = "veriloga-native")]
    let artifact = canonical_ir.ok_or_else(|| {
        "native Verilog-A runtime cache entry requires canonical IR (no interpreter fallback)"
            .to_string()
    })?;
    #[cfg(not(feature = "veriloga-native"))]
    let Some(artifact) = canonical_ir else {
        return Ok(());
    };

    artifact.validate().map_err(|diagnostics| {
        let detail = diagnostics
            .first()
            .map(|diagnostic| diagnostic.message.as_str())
            .unwrap_or("canonical artifact validation failed");
        format!("canonical Verilog-A artifact failed integrity validation: {detail}")
    })?;

    if artifact.metadata.source_digest != model.source_digest {
        return Err(format!(
            "canonical Verilog-A source digest '{}' does not match compiled model digest '{}'",
            artifact.metadata.source_digest, model.source_digest
        ));
    }
    if artifact.mir.module_name != model.name {
        return Err(format!(
            "canonical Verilog-A module '{}' does not match compiled model '{}'",
            artifact.mir.module_name, model.name
        ));
    }
    if artifact.mir.equations.len() != model.stamp_programs.len() {
        return Err(format!(
            "canonical Verilog-A equation count {} does not match compiled stamp count {}",
            artifact.mir.equations.len(),
            model.stamp_programs.len()
        ));
    }

    Ok(())
}

#[cfg(feature = "veriloga")]
pub(super) fn veriloga_model_cache() -> &'static RwLock<VerilogAModelCache> {
    static CACHE: OnceLock<RwLock<VerilogAModelCache>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(VerilogAModelCache::default()))
}

#[cfg(feature = "veriloga")]
pub(super) fn clear_in_memory_veriloga_cache() {
    if let Ok(mut cache) = veriloga_model_cache().write() {
        cache.clear();
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn canonicalize_for_cache(path: &Path) -> PathBuf {
    if is_sealed_veriloga_virtual_path(path) {
        return PathBuf::from(path.to_string_lossy().replace('\\', "/"));
    }
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(feature = "veriloga")]
fn is_sealed_veriloga_virtual_path(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized.split_once('/').is_some_and(|(root, _)| {
        root.eq_ignore_ascii_case("__rspice_project__")
            || root.eq_ignore_ascii_case("__rspice_pdk__")
            || root.eq_ignore_ascii_case("__rspice_model_library__")
    })
}

#[cfg(feature = "veriloga")]
pub(super) fn normalize_model_key(name: &str) -> String {
    name.to_ascii_uppercase()
}

#[cfg(feature = "veriloga")]
pub(super) fn metadata_modified_ns(metadata: &std::fs::Metadata) -> Option<u128> {
    use std::time::UNIX_EPOCH;

    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
}

#[cfg(feature = "veriloga")]
enum VerilogADependencyReadError {
    Io(std::io::Error),
    ResourceLimit(ResourceLimitError),
    Cancelled,
}

#[cfg(feature = "veriloga")]
fn hash_dependency_file_with_limits(
    path: &Path,
    bytes_already_read: usize,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<([u8; 32], std::fs::Metadata, usize), VerilogADependencyReadError> {
    if abort.is_aborted() {
        return Err(VerilogADependencyReadError::Cancelled);
    }
    let mut file = std::fs::File::open(path)?;
    let metadata = file.metadata()?;
    let metadata_bytes = usize::try_from(metadata.len()).unwrap_or(usize::MAX);
    ResourceLimitError::ensure(
        ResourceKind::DependencySourceBytes,
        bytes_already_read.saturating_add(metadata_bytes),
        limits.max_dependency_source_bytes,
    )?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut bytes_read = 0_usize;

    loop {
        if abort.is_aborted() {
            return Err(VerilogADependencyReadError::Cancelled);
        }
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        bytes_read = bytes_read.saturating_add(read);
        ResourceLimitError::ensure(
            ResourceKind::DependencySourceBytes,
            bytes_already_read.saturating_add(bytes_read),
            limits.max_dependency_source_bytes,
        )?;
        hasher.update(&buffer[..read]);
    }

    Ok((*hasher.finalize().as_bytes(), metadata, bytes_read))
}

#[cfg(feature = "veriloga")]
impl From<std::io::Error> for VerilogADependencyReadError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[cfg(feature = "veriloga")]
impl From<ResourceLimitError> for VerilogADependencyReadError {
    fn from(error: ResourceLimitError) -> Self {
        Self::ResourceLimit(error)
    }
}

#[cfg(feature = "veriloga")]
// Reached only through the not-wasm32 `fingerprint_paths` below; the allow
// covers the browser target's view, where there is no disk to fingerprint.
#[allow(dead_code)]
fn fingerprint_paths_with_limits(
    paths: &[PathBuf],
    limits: ResourceLimits,
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    fingerprint_paths_with_limits_and_abort(paths, limits, &NoAbort)
}

#[cfg(feature = "veriloga")]
fn fingerprint_paths_with_limits_and_abort(
    paths: &[PathBuf],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    let mut canonical_paths: Vec<PathBuf> =
        paths.iter().map(|p| canonicalize_for_cache(p)).collect();
    canonical_paths.sort();
    canonical_paths.dedup();

    let mut fingerprints = Vec::with_capacity(canonical_paths.len());
    let mut dependency_bytes = 0_usize;
    for canonical_path in canonical_paths {
        match hash_dependency_file_with_limits(&canonical_path, dependency_bytes, limits, abort) {
            Ok((content_hash, metadata, bytes_read)) => {
                dependency_bytes = dependency_bytes.saturating_add(bytes_read);
                fingerprints.push(VerilogADependencyFingerprint {
                    canonical_path,
                    modified_ns: metadata_modified_ns(&metadata),
                    file_len: metadata.len(),
                    content_hash,
                });
            }
            Err(VerilogADependencyReadError::ResourceLimit(error)) => return Err(error.into()),
            Err(VerilogADependencyReadError::Cancelled) => {
                return Err(SimulationError::Aborted);
            }
            Err(VerilogADependencyReadError::Io(error)) => {
                return Err(SimulationError::Netlist(format!(
                    "Verilog-A dependency '{}' does not exist or is unreadable: {}",
                    canonical_path.display(),
                    error
                )));
            }
        }
    }

    Ok(fingerprints)
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn fingerprint_paths(
    paths: &[PathBuf],
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    fingerprint_paths_with_limits(paths, ResourceLimits::default())
}

#[cfg(feature = "veriloga")]
fn dependencies_are_fresh_with_limits_and_abort(
    dependencies: &[VerilogADependencyFingerprint],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<bool, SimulationError> {
    let mut dependency_bytes = 0_usize;
    for dependency in dependencies {
        match hash_dependency_file_with_limits(
            &dependency.canonical_path,
            dependency_bytes,
            limits,
            abort,
        ) {
            Ok((content_hash, _, bytes_read)) => {
                dependency_bytes = dependency_bytes.saturating_add(bytes_read);
                if content_hash != dependency.content_hash {
                    return Ok(false);
                }
            }
            Err(VerilogADependencyReadError::ResourceLimit(error)) => return Err(error.into()),
            Err(VerilogADependencyReadError::Cancelled) => {
                return Err(SimulationError::Aborted);
            }
            Err(VerilogADependencyReadError::Io(_)) => return Ok(false),
        }
    }
    Ok(true)
}

#[cfg(feature = "veriloga")]
pub(super) fn parse_cache_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(feature = "veriloga")]
pub(super) fn parse_cache_env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(feature = "veriloga")]
pub(super) fn veriloga_cache_limits() -> (usize, u64) {
    (
        parse_cache_env_usize(
            VERILOGA_CACHE_MAX_ENTRIES_ENV,
            VERILOGA_CACHE_DEFAULT_MAX_ENTRIES,
        ),
        parse_cache_env_u64(
            VERILOGA_CACHE_MAX_BYTES_ENV,
            VERILOGA_CACHE_DEFAULT_MAX_BYTES,
        ),
    )
}

// `std::env::temp_dir` (and the lock's `Instant`/`thread::sleep`) abort on
// wasm32-unknown-unknown rather than erroring, so the entire disk side of
// the cache only exists on native targets; `with_veriloga_cache_disk_lock`
// is the single gate.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn veriloga_cache_root() -> PathBuf {
    if let Some(override_dir) = std::env::var_os("RSPICE_VERILOGA_CACHE_DIR") {
        return PathBuf::from(override_dir);
    }

    if let Some(cache_dir) = dirs::cache_dir() {
        return cache_dir.join("rspice").join("veriloga");
    }

    std::env::temp_dir().join("rspice-veriloga-cache")
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn cache_record_path_with_root(source_path: &Path, cache_root: &Path) -> PathBuf {
    let canonical = canonicalize_for_cache(source_path);
    let mut hasher = blake3::Hasher::new();
    hasher.update(canonical.to_string_lossy().as_bytes());
    let key = hasher.finalize().to_hex().to_string();
    cache_root.join(format!("{key}.json"))
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub(super) struct VerilogACacheFileInfo {
    path: PathBuf,
    size_bytes: u64,
    modified_ns: Option<u128>,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
#[derive(Debug)]
pub(super) struct VerilogACacheDiskLock {
    lock_path: PathBuf,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
enum VerilogACacheLockError {
    Cancelled,
    Other(String),
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
impl VerilogACacheDiskLock {
    fn acquire(root: &Path) -> Result<Self, String> {
        match Self::acquire_with_abort(root, &NoAbort) {
            Ok(lock) => Ok(lock),
            Err(VerilogACacheLockError::Other(error)) => Err(error),
            Err(VerilogACacheLockError::Cancelled) => {
                unreachable!("the no-op abort signal cannot cancel")
            }
        }
    }

    fn acquire_with_abort(
        root: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<Self, VerilogACacheLockError> {
        std::fs::create_dir_all(root).map_err(|e| {
            VerilogACacheLockError::Other({
                format!(
                    "failed to create cache directory '{}': {}",
                    root.display(),
                    e
                )
            })
        })?;
        let lock_path = root.join(VERILOGA_CACHE_LOCK_FILE);
        let start = Instant::now();

        loop {
            if abort.is_aborted() {
                return Err(VerilogACacheLockError::Cancelled);
            }
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&lock_path)
            {
                Ok(mut file) => {
                    let timestamp_ns = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .map(|d| d.as_nanos())
                        .unwrap_or(0);
                    let _ = writeln!(
                        file,
                        "pid={} timestamp_ns={}",
                        std::process::id(),
                        timestamp_ns
                    );
                    return Ok(Self { lock_path });
                }
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                    if cache_lock_is_stale(&lock_path) {
                        let _ = std::fs::remove_file(&lock_path);
                        continue;
                    }

                    if start.elapsed() >= VERILOGA_CACHE_LOCK_WAIT_TIMEOUT {
                        return Err(VerilogACacheLockError::Other(format!(
                            "timed out waiting for Verilog-A cache lock '{}'",
                            lock_path.display()
                        )));
                    }

                    std::thread::sleep(VERILOGA_CACHE_LOCK_POLL_INTERVAL);
                }
                Err(err) => {
                    return Err(VerilogACacheLockError::Other(format!(
                        "failed to acquire Verilog-A cache lock '{}': {}",
                        lock_path.display(),
                        err
                    )));
                }
            }
        }
    }
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
impl Drop for VerilogACacheDiskLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.lock_path);
    }
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn cache_lock_is_stale(lock_path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(lock_path) else {
        return false;
    };
    let Ok(modified) = metadata.modified() else {
        return false;
    };
    match modified.elapsed() {
        Ok(elapsed) => elapsed > VERILOGA_CACHE_LOCK_STALE_TIMEOUT,
        Err(_) => false,
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn with_veriloga_cache_disk_lock<T>(
    operation: &str,
    f: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = f;
        Err(format!(
            "{}: the Verilog-A disk cache is unavailable in the browser build",
            operation
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let root = veriloga_cache_root();
        let _lock =
            VerilogACacheDiskLock::acquire(&root).map_err(|e| format!("{}: {}", operation, e))?;
        f(&root)
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn list_cache_files(cache_root: &Path) -> Result<Vec<VerilogACacheFileInfo>, String> {
    if !cache_root.exists() {
        return Ok(Vec::new());
    }

    let dir_iter = std::fs::read_dir(cache_root).map_err(|e| {
        format!(
            "failed to list cache directory '{}': {}",
            cache_root.display(),
            e
        )
    })?;
    let mut files = Vec::new();
    for entry in dir_iter {
        let entry = entry.map_err(|e| format!("failed to read cache directory entry: {}", e))?;
        let path = entry.path();
        if !matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("json" | "bin")
        ) {
            continue;
        }
        let metadata = entry.metadata().map_err(|e| {
            format!(
                "failed to read cache file metadata '{}': {}",
                path.display(),
                e
            )
        })?;
        if !metadata.is_file() {
            continue;
        }
        files.push(VerilogACacheFileInfo {
            path,
            size_bytes: metadata.len(),
            modified_ns: metadata_modified_ns(&metadata),
        });
    }
    Ok(files)
}

#[cfg(feature = "veriloga")]
pub(super) fn cache_stats_from_files(
    cache_root: &Path,
    files: &[VerilogACacheFileInfo],
) -> VerilogACacheStats {
    let (max_entries, max_bytes) = veriloga_cache_limits();
    VerilogACacheStats {
        root: cache_root.to_path_buf(),
        entry_count: files.len(),
        total_bytes: files
            .iter()
            .map(|file| file.size_bytes)
            .fold(0_u64, u64::saturating_add),
        max_entries,
        max_bytes,
    }
}

#[cfg(feature = "veriloga")]
enum VerilogACacheRecordReadError {
    Invalid(String),
    ResourceLimit(ResourceLimitError),
    Cancelled,
}

/// The message an interrupted or oversized cache-record read carries.
#[cfg(feature = "veriloga")]
const CACHE_RECORD_READ: &str = "Verilog-A cache record";

#[cfg(feature = "veriloga")]
fn read_cache_record_with_limits(
    path: &Path,
    limits: ResourceLimits,
) -> Result<VerilogADiskCacheRecord, VerilogACacheRecordReadError> {
    read_cache_record_with_limits_and_abort(path, limits, &NoAbort)
}

#[cfg(feature = "veriloga")]
fn read_cache_record_with_limits_and_abort(
    path: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<VerilogADiskCacheRecord, VerilogACacheRecordReadError> {
    let file = std::fs::File::open(path).map_err(|error| {
        VerilogACacheRecordReadError::Invalid(format!(
            "failed to read cache record '{}': {}",
            path.display(),
            error
        ))
    })?;
    let metadata = file.metadata().map_err(|error| {
        VerilogACacheRecordReadError::Invalid(format!(
            "failed to inspect cache record '{}': {}",
            path.display(),
            error
        ))
    })?;
    let metadata_bytes = usize::try_from(metadata.len()).unwrap_or(usize::MAX);
    ResourceLimitError::ensure(
        ResourceKind::SharedCacheBytes,
        metadata_bytes,
        limits.max_shared_cache_bytes,
    )
    .map_err(VerilogACacheRecordReadError::ResourceLimit)?;

    let buffered = std::io::BufReader::new(file);
    let mut reader = crate::abort_signal::AbortReader::with_byte_cap(
        buffered,
        abort,
        CACHE_RECORD_READ,
        limits.max_shared_cache_bytes,
    );
    match serde_json::from_reader::<_, VerilogADiskCacheRecord>(&mut reader) {
        Ok(record) => Ok(record),
        Err(_) if reader.was_cancelled() => Err(VerilogACacheRecordReadError::Cancelled),
        Err(_) if reader.exceeded_cap() => Err(VerilogACacheRecordReadError::ResourceLimit(
            ResourceLimitError {
                resource: ResourceKind::SharedCacheBytes,
                requested: limits.max_shared_cache_bytes.saturating_add(1),
                limit: limits.max_shared_cache_bytes,
            },
        )),
        Err(error) => Err(VerilogACacheRecordReadError::Invalid(format!(
            "failed to deserialize cache record '{}': {}",
            path.display(),
            error
        ))),
    }
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
struct LimitedWriter<W> {
    inner: W,
    written: u64,
    limit: u64,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
impl<W> LimitedWriter<W> {
    fn new(inner: W, limit: u64) -> Self {
        Self {
            inner,
            written: 0,
            limit,
        }
    }
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
impl<W: std::io::Write> std::io::Write for LimitedWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let requested = self
            .written
            .saturating_add(u64::try_from(bytes.len()).unwrap_or(u64::MAX));
        if requested > self.limit {
            return Err(std::io::Error::other(format!(
                "Verilog-A cache record exceeds the {} byte write limit",
                self.limit
            )));
        }
        let written = self.inner.write(bytes)?;
        self.written = self
            .written
            .saturating_add(u64::try_from(written).unwrap_or(u64::MAX));
        Ok(written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn persist_model_to_disk_locked_with_limits(
    source_path: &Path,
    entry: &CachedVerilogAModel,
    cache_root: &Path,
    limits: ResourceLimits,
) -> Result<(), String> {
    let cache_path = cache_record_path_with_root(source_path, cache_root);
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create cache directory: {}", e))?;
    }

    let canonical_source = canonicalize_for_cache(source_path);
    let record = BorrowedVerilogACacheRecord::new(&canonical_source, entry);
    let tmp_path = cache_path.with_extension(format!("tmp.{}", std::process::id()));
    let (_, disk_max_bytes) = veriloga_cache_limits();
    let resource_max_bytes = u64::try_from(limits.max_shared_cache_bytes).unwrap_or(u64::MAX);
    let write_limit = disk_max_bytes.min(resource_max_bytes);
    let write_result = (|| {
        let file = std::fs::File::create(&tmp_path)
            .map_err(|error| format!("failed to create Verilog-A cache record: {error}"))?;
        let buffered = std::io::BufWriter::new(file);
        let mut writer = LimitedWriter::new(buffered, write_limit);
        serde_json::to_writer(&mut writer, &record)
            .map_err(|error| format!("failed to serialize Verilog-A cache record: {error}"))?;
        writer
            .flush()
            .map_err(|error| format!("failed to flush Verilog-A cache record: {error}"))
    })();
    if let Err(error) = write_result {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(error);
    }

    if let Err(rename_err) = std::fs::rename(&tmp_path, &cache_path) {
        // std::fs::rename does not replace existing files on Windows.
        if cache_path.exists() {
            std::fs::remove_file(&cache_path).map_err(|e| {
                let _ = std::fs::remove_file(&tmp_path);
                format!(
                    "failed to replace existing Verilog-A cache record '{}': {} (rename error: {})",
                    cache_path.display(),
                    e,
                    rename_err
                )
            })?;
            std::fs::rename(&tmp_path, &cache_path).map_err(|e| {
                let _ = std::fs::remove_file(&tmp_path);
                format!(
                    "failed to finalize Verilog-A cache record '{}' after replacement: {}",
                    cache_path.display(),
                    e
                )
            })?;
        } else {
            let _ = std::fs::remove_file(&tmp_path);
            return Err(format!(
                "failed to finalize Verilog-A cache record '{}': {}",
                cache_path.display(),
                rename_err
            ));
        }
    }

    Ok(())
}

#[cfg(all(test, feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn persist_model_to_disk_locked(
    source_path: &Path,
    entry: &CachedVerilogAModel,
    cache_root: &Path,
) -> Result<(), String> {
    persist_model_to_disk_locked_with_limits(
        source_path,
        entry,
        cache_root,
        ResourceLimits::default(),
    )
}

#[cfg(feature = "veriloga")]
pub(super) fn remove_cache_file(path: &Path) {
    if let Err(err) = std::fs::remove_file(path)
        && err.kind() != std::io::ErrorKind::NotFound
    {
        log::warn!(
            "failed to remove stale/corrupt Verilog-A cache file '{}': {}",
            path.display(),
            err
        );
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn prune_veriloga_cache_locked(
    cache_root: &Path,
) -> Result<VerilogACachePruneReport, String> {
    let (max_entries, max_bytes) = veriloga_cache_limits();
    let mut files = list_cache_files(cache_root)?;
    files.sort_by(|a, b| {
        let left = a.modified_ns.unwrap_or(0);
        let right = b.modified_ns.unwrap_or(0);
        left.cmp(&right).then_with(|| a.path.cmp(&b.path))
    });

    let mut entry_count = files.len();
    let mut total_bytes = files
        .iter()
        .map(|file| file.size_bytes)
        .fold(0_u64, u64::saturating_add);
    let mut removed_entries = 0_usize;
    let mut reclaimed_bytes = 0_u64;

    for file in files {
        if entry_count <= max_entries && total_bytes <= max_bytes {
            break;
        }

        match std::fs::remove_file(&file.path) {
            Ok(()) => {
                entry_count = entry_count.saturating_sub(1);
                total_bytes = total_bytes.saturating_sub(file.size_bytes);
                removed_entries += 1;
                reclaimed_bytes = reclaimed_bytes.saturating_add(file.size_bytes);
            }
            Err(err) => {
                log::warn!(
                    "failed to evict Verilog-A cache record '{}': {}",
                    file.path.display(),
                    err
                );
            }
        }
    }

    let stats = VerilogACacheStats {
        root: cache_root.to_path_buf(),
        entry_count,
        total_bytes,
        max_entries,
        max_bytes,
    };

    Ok(VerilogACachePruneReport {
        removed_entries,
        reclaimed_bytes,
        stats,
    })
}

#[cfg(feature = "veriloga")]
fn persist_model_to_disk_with_limits(
    source_path: &Path,
    entry: &CachedVerilogAModel,
    limits: ResourceLimits,
) -> Result<(), String> {
    // No disk in the browser build: the in-memory cache is the cache.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = source_path;
        let _ = entry.canonical_ir.as_ref();
        let _ = limits;
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    with_veriloga_cache_disk_lock("persist Verilog-A cache record", |cache_root| {
        persist_model_to_disk_locked_with_limits(source_path, entry, cache_root, limits)?;
        if let Err(err) = prune_veriloga_cache_locked(cache_root) {
            log::warn!("failed to prune Verilog-A cache after write: {}", err);
        }
        Ok(())
    })
}

#[cfg(feature = "veriloga")]
pub(super) fn persist_model_to_disk(
    source_path: &Path,
    entry: &CachedVerilogAModel,
) -> Result<(), String> {
    persist_model_to_disk_with_limits(source_path, entry, ResourceLimits::default())
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
fn load_model_from_disk_locked_with_limits(
    source_path: &Path,
    cache_root: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<CachedVerilogAModel>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let cache_path = cache_record_path_with_root(source_path, cache_root);
    let record = match read_cache_record_with_limits_and_abort(&cache_path, limits, abort) {
        Ok(record) => record,
        Err(VerilogACacheRecordReadError::Invalid(error)) => {
            if cache_path.exists() {
                log::warn!("{}", error);
                remove_cache_file(&cache_path);
            }
            return Ok(None);
        }
        Err(VerilogACacheRecordReadError::ResourceLimit(error)) => {
            log::debug!(
                "skipping Verilog-A cache record '{}' under the active resource policy: {}",
                cache_path.display(),
                error
            );
            return Ok(None);
        }
        Err(VerilogACacheRecordReadError::Cancelled) => {
            return Err(SimulationError::Aborted);
        }
    };

    if record.version != VERILOGA_CACHE_RECORD_VERSION {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    let requested_source = canonicalize_for_cache(source_path);
    let record_source = canonicalize_for_cache(&record.source_path);
    if requested_source != record_source {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if !dependencies_are_fresh_with_limits_and_abort(&record.dependencies, limits, abort)? {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    if let Err(error) = validate_runtime_artifact_pair(&record.model, record.canonical_ir.as_ref())
    {
        log::warn!(
            "discarding invalid Verilog-A cache record '{}': {}",
            cache_path.display(),
            error
        );
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    Ok(Some(CachedVerilogAModel {
        dependencies: record.dependencies,
        model: std::sync::Arc::new(record.model),
        canonical_ir: record.canonical_ir.map(std::sync::Arc::new),
    }))
}

#[cfg(feature = "veriloga")]
fn load_model_from_disk_with_limits_and_abort(
    source_path: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Option<CachedVerilogAModel>, SimulationError> {
    // No disk in the browser build: only the in-memory cache can hit.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = source_path;
        let _ = limits;
        let _ = abort;
        Ok(None)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let cache_root = veriloga_cache_root();
        let _lock = match VerilogACacheDiskLock::acquire_with_abort(&cache_root, abort) {
            Ok(lock) => lock,
            Err(VerilogACacheLockError::Cancelled) => {
                return Err(SimulationError::Aborted);
            }
            Err(VerilogACacheLockError::Other(error)) => {
                log::warn!("load Verilog-A cache record: {}", error);
                return Ok(None);
            }
        };
        load_model_from_disk_locked_with_limits(source_path, &cache_root, limits, abort)
    }
}

#[cfg(all(test, feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn load_model_from_disk_locked(
    source_path: &Path,
    cache_root: &Path,
) -> Result<Option<CachedVerilogAModel>, String> {
    load_model_from_disk_locked_with_limits(
        source_path,
        cache_root,
        ResourceLimits::default(),
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Query on-disk Verilog-A cache statistics.
#[cfg(feature = "veriloga")]
pub fn veriloga_cache_stats() -> Result<VerilogACacheStats, String> {
    with_veriloga_cache_disk_lock("inspect Verilog-A cache", |cache_root| {
        let files = list_cache_files(cache_root)?;
        Ok(cache_stats_from_files(cache_root, &files))
    })
}

/// List persisted Verilog-A cache entries including dependency paths.
#[cfg(feature = "veriloga")]
pub fn veriloga_cache_entries() -> Result<Vec<VerilogACacheEntry>, String> {
    with_veriloga_cache_disk_lock("list Verilog-A cache entries", |cache_root| {
        let mut files = list_cache_files(cache_root)?;
        files.sort_by(|a, b| a.path.cmp(&b.path));

        let mut entries = Vec::with_capacity(files.len());
        for file in files {
            let record = match read_cache_record_with_limits(&file.path, ResourceLimits::default())
            {
                Ok(record) => record,
                Err(VerilogACacheRecordReadError::Invalid(error)) => {
                    log::debug!(
                        "discarding invalid Verilog-A cache record '{}': {}",
                        file.path.display(),
                        error
                    );
                    remove_cache_file(&file.path);
                    continue;
                }
                Err(VerilogACacheRecordReadError::ResourceLimit(error)) => {
                    log::warn!(
                        "skipping oversized Verilog-A cache record '{}': {}",
                        file.path.display(),
                        error
                    );
                    continue;
                }
                Err(VerilogACacheRecordReadError::Cancelled) => {
                    unreachable!("the no-op abort signal cannot cancel")
                }
            };
            if record.version != VERILOGA_CACHE_RECORD_VERSION {
                remove_cache_file(&file.path);
                continue;
            }
            if validate_runtime_artifact_pair(&record.model, record.canonical_ir.as_ref()).is_err()
            {
                remove_cache_file(&file.path);
                continue;
            }
            entries.push(VerilogACacheEntry {
                cache_path: file.path,
                source_path: canonicalize_for_cache(&record.source_path),
                dependencies: record
                    .dependencies
                    .into_iter()
                    .map(|dep| dep.canonical_path)
                    .collect(),
                size_bytes: file.size_bytes,
                modified_ns: file.modified_ns,
            });
        }

        entries.sort_by(|a, b| a.source_path.cmp(&b.source_path));
        Ok(entries)
    })
}

/// Prune on-disk Verilog-A cache to configured limits.
#[cfg(feature = "veriloga")]
pub fn prune_veriloga_cache() -> Result<VerilogACachePruneReport, String> {
    with_veriloga_cache_disk_lock("prune Verilog-A cache", |cache_root| {
        prune_veriloga_cache_locked(cache_root)
    })
}

/// Clear all on-disk and in-memory Verilog-A cache entries.
#[cfg(feature = "veriloga")]
pub fn clear_veriloga_cache() -> Result<VerilogACachePruneReport, String> {
    with_veriloga_cache_disk_lock("clear Verilog-A cache", |cache_root| {
        let files = list_cache_files(cache_root)?;
        let mut removed_entries = 0_usize;
        let mut reclaimed_bytes = 0_u64;
        for file in files {
            match std::fs::remove_file(&file.path) {
                Ok(()) => {
                    removed_entries += 1;
                    reclaimed_bytes = reclaimed_bytes.saturating_add(file.size_bytes);
                }
                Err(err) => {
                    log::warn!(
                        "failed to remove Verilog-A cache entry '{}': {}",
                        file.path.display(),
                        err
                    );
                }
            }
        }
        clear_in_memory_veriloga_cache();
        let empty_files = list_cache_files(cache_root)?;
        let stats = cache_stats_from_files(cache_root, &empty_files);
        Ok(VerilogACachePruneReport {
            removed_entries,
            reclaimed_bytes,
            stats,
        })
    })
}

#[cfg(all(test, feature = "veriloga"))]
pub(super) fn resolve_cached_or_compile_veriloga(
    path: &Path,
) -> Result<CachedVerilogAModel, SimulationError> {
    resolve_cached_or_compile_veriloga_with_limits(path, ResourceLimits::default())
}

/// How a `.VERILOGA` include named by a deck is compiled.
///
/// The one departure from the compiler's own defaults is `enable_ams`, and it
/// is what gives a mixed module a deck route at all. With it off, code
/// generation refuses the first digital declaration it meets, so a `.va` whose
/// module carries both an analog block and a process could not be read by a
/// deck; with it on, the front end emits the analog half as bytecode and hands
/// the discrete half to the canonical plan, which is what
/// [`MixedSignalHost`](crate::xspice::verilog::MixedSignalHost) executes.
///
/// Enabling it is only sound because the *builder* decides which half of the
/// artifact each module reaches: a module whose canonical plan is empty is
/// built as an ordinary [`VerilogADevice`](crate::device::veriloga::VerilogADevice)
/// exactly as before, and a module whose plan is not empty is built as a mixed
/// host that executes it. What `enable_ams` must never do is let an analog-only
/// device route silently drop a process, and here it cannot: the same predicate
/// that lets the digital half through the compiler selects the route that runs
/// it.
///
/// For a module with no digital content the option changes nothing at all. Its
/// entire effect inside the compiler is to skip `reject_digital_content`, which
/// on an analog-only module is a no-op — so the `CompiledModel` and the
/// canonical artifact a deck's analog `.va` produces are the ones it produced
/// before.
#[cfg(feature = "veriloga")]
fn deck_include_compiler_options() -> rspice_veriloga::CompilerOptions {
    rspice_veriloga::CompilerOptions {
        enable_ams: true,
        ..rspice_veriloga::CompilerOptions::default()
    }
}

#[cfg(feature = "veriloga")]
struct VerilogACompileControl<'a> {
    abort: &'a dyn AbortSignal,
}

#[cfg(feature = "veriloga")]
impl rspice_veriloga::PipelineControl for VerilogACompileControl<'_> {
    fn is_cancelled(&self) -> bool {
        self.abort.is_aborted()
    }

    fn phase_completed(
        &self,
        _timing: rspice_veriloga::PhaseTiming,
        metrics: &rspice_veriloga::PipelineMetrics,
    ) {
        const EXPECTED_RUNTIME_PHASES: f64 = 9.0;
        self.abort.observe_progress(
            (metrics.phases.len() as f64 / EXPECTED_RUNTIME_PHASES).clamp(0.0, 1.0),
        );
    }
}

#[cfg(all(test, feature = "veriloga"))]
pub(super) fn resolve_cached_or_compile_veriloga_with_limits(
    path: &Path,
    limits: ResourceLimits,
) -> Result<CachedVerilogAModel, SimulationError> {
    resolve_cached_or_compile_veriloga_with_limits_and_abort(path, limits, &NoAbort)
}

#[cfg(feature = "veriloga")]
pub(super) fn resolve_cached_or_compile_veriloga_with_limits_and_abort(
    path: &Path,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<CachedVerilogAModel, SimulationError> {
    use std::sync::atomic::Ordering::Relaxed;

    VERILOGA_CACHE_TELEMETRY.lookups.fetch_add(1, Relaxed);
    check_build_abort(abort)?;
    let canonical = canonicalize_for_cache(path);
    let memory_entry = if let Ok(mut cache) = veriloga_model_cache().write() {
        cache.enforce_limit(limits.max_shared_cache_bytes);
        cache.get_cloned(&canonical)
    } else {
        None
    };

    if let Some(entry) = memory_entry {
        if dependencies_are_fresh_with_limits_and_abort(&entry.dependencies, limits, abort)? {
            VERILOGA_CACHE_TELEMETRY.memory_hits.fetch_add(1, Relaxed);
            log::debug!("Verilog-A cache hit (memory): '{}'", canonical.display());
            return Ok(entry);
        }
        VERILOGA_CACHE_TELEMETRY
            .stale_memory_entries
            .fetch_add(1, Relaxed);
        if let Ok(mut cache) = veriloga_model_cache().write()
            && cache
                .get(&canonical)
                .is_some_and(|current| current.dependencies == entry.dependencies)
        {
            cache.remove(&canonical);
        }
    }

    // A sealed virtual key is an authenticated in-memory capability,
    // never a filesystem locator. A missing registration must fail closed;
    // disk cache and ambient files are not eligible fallbacks.
    if is_sealed_veriloga_virtual_path(path) {
        VERILOGA_CACHE_TELEMETRY.misses.fetch_add(1, Relaxed);
        return Err(SimulationError::Netlist(format!(
            "Sealed Verilog-A runtime '{}' is not installed for this execution",
            path.display()
        )));
    }

    check_build_abort(abort)?;
    if let Some(entry) = load_model_from_disk_with_limits_and_abort(&canonical, limits, abort)? {
        VERILOGA_CACHE_TELEMETRY.disk_hits.fetch_add(1, Relaxed);
        if let Err(error) = retain_veriloga_model(
            canonical.clone(),
            entry.clone(),
            limits.max_shared_cache_bytes,
            false,
        ) {
            log::warn!(
                "failed to retain Verilog-A disk cache hit for '{}': {}",
                canonical.display(),
                error
            );
        }
        log::debug!("Verilog-A cache hit (disk): '{}'", canonical.display());
        return Ok(entry);
    }

    VERILOGA_CACHE_TELEMETRY.misses.fetch_add(1, Relaxed);
    check_build_abort(abort)?;
    let source_metadata = std::fs::metadata(&canonical).map_err(|error| {
        SimulationError::Netlist(format!(
            "Verilog-A source '{}' does not exist or is unreadable: {}",
            canonical.display(),
            error
        ))
    })?;
    ResourceLimitError::ensure(
        ResourceKind::DependencySourceBytes,
        usize::try_from(source_metadata.len()).unwrap_or(usize::MAX),
        limits.max_dependency_source_bytes,
    )?;

    log::info!("Verilog-A cache miss, compiling '{}'", canonical.display());
    let compiler = rspice_veriloga::VerilogACompiler::new(deck_include_compiler_options());
    let control = VerilogACompileControl { abort };
    VERILOGA_CACHE_TELEMETRY
        .compilations_started
        .fetch_add(1, Relaxed);
    let compile_started = crate::time_compat::Instant::now();
    let compiled = compiler.compile_file_runtime_with_metadata_and_control(path, None, &control);
    let compilation_nanos = u64::try_from(compile_started.elapsed().as_nanos()).unwrap_or(u64::MAX);
    let _ = VERILOGA_CACHE_TELEMETRY
        .total_compilation_nanos
        .fetch_update(Relaxed, Relaxed, |total| {
            Some(total.saturating_add(compilation_nanos))
        });
    let compiled = match compiled {
        Ok(compiled) => {
            VERILOGA_CACHE_TELEMETRY
                .compilations_succeeded
                .fetch_add(1, Relaxed);
            compiled
        }
        Err(rspice_veriloga::CompileError::Cancelled(_)) => {
            VERILOGA_CACHE_TELEMETRY
                .compilations_cancelled
                .fetch_add(1, Relaxed);
            return Err(SimulationError::Aborted);
        }
        Err(error) => {
            VERILOGA_CACHE_TELEMETRY
                .compilations_failed
                .fetch_add(1, Relaxed);
            return Err(SimulationError::Netlist(format!(
                "Failed to compile Verilog-A '{}': {}",
                path.display(),
                error
            )));
        }
    };

    check_build_abort(abort)?;
    validate_runtime_artifact_pair(&compiled.model, Some(&compiled.canonical_ir)).map_err(
        |error| {
            SimulationError::Netlist(format!(
                "Compiled Verilog-A runtime artifacts for '{}' failed integrity validation: {}",
                path.display(),
                error
            ))
        },
    )?;
    let dependencies =
        fingerprint_paths_with_limits_and_abort(&compiled.dependencies, limits, abort)?;
    let entry = CachedVerilogAModel {
        dependencies,
        model: std::sync::Arc::new(compiled.model),
        canonical_ir: Some(std::sync::Arc::new(compiled.canonical_ir)),
    };

    check_build_abort(abort)?;
    if let Err(error) = retain_veriloga_model(
        canonical.clone(),
        entry.clone(),
        limits.max_shared_cache_bytes,
        false,
    ) {
        log::warn!(
            "failed to retain compiled Verilog-A runtime for '{}': {}",
            canonical.display(),
            error
        );
    }

    if let Err(err) = persist_model_to_disk_with_limits(&canonical, &entry, limits) {
        VERILOGA_CACHE_TELEMETRY
            .persistence_failures
            .fetch_add(1, Relaxed);
        log::warn!(
            "Failed to persist Verilog-A cache entry for '{}': {}",
            canonical.display(),
            err
        );
    }

    Ok(entry)
}

#[cfg(feature = "veriloga")]
fn register_precompiled_veriloga_entry_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
    canonical_ir: Option<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<(), String> {
    #[cfg(feature = "veriloga-native")]
    if canonical_ir.is_none() {
        return Err(
            "native Verilog-A registration requires canonical IR; use register_precompiled_veriloga_runtime_with_dependencies"
                .to_string(),
        );
    }

    validate_runtime_artifact_pair(&model, canonical_ir.as_ref())?;

    let canonical_source = canonicalize_for_cache(source_path.as_ref());

    #[cfg(not(target_arch = "wasm32"))]
    let dependency_fingerprints = {
        let mut dependency_paths = dependencies.to_vec();
        if dependency_paths.is_empty() {
            dependency_paths.push(canonical_source.clone());
        }
        fingerprint_paths(&dependency_paths)
            .map_err(|e| format!("dependency fingerprinting failed: {}", e))?
    };
    // The browser build has no filesystem to fingerprint against; an empty
    // dependency set always reads as fresh, so the registered artifact is
    // simply trusted for the session.
    #[cfg(target_arch = "wasm32")]
    let dependency_fingerprints = {
        let _ = dependencies;
        Vec::new()
    };

    let entry = CachedVerilogAModel {
        dependencies: dependency_fingerprints,
        model: std::sync::Arc::new(model),
        canonical_ir: canonical_ir.map(std::sync::Arc::new),
    };

    retain_veriloga_model(
        canonical_source.clone(),
        entry.clone(),
        ResourceLimits::default().max_shared_cache_bytes,
        true,
    )?;

    if let Err(err) = persist_model_to_disk(&canonical_source, &entry) {
        log::warn!(
            "Failed to persist precompiled Verilog-A cache for '{}': {}",
            canonical_source.display(),
            err
        );
    }

    Ok(())
}

/// Register a precompiled Verilog-A runtime artifact in the global engine cache.
///
/// Native builds require this paired model/canonical-IR artifact so the runtime
/// cannot silently fall back to bytecode-only construction.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_runtime_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
    canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<(), String> {
    register_precompiled_veriloga_entry_with_dependencies(
        source_path,
        dependencies,
        model,
        Some(canonical_ir),
    )
}

/// One exact sealed Verilog-A runtime prepared for atomic registration.
///
/// `aliases` are the model identities that a generated deck may use for this
/// artifact. They are checked case-insensitively within each project or signed
/// package authority represented by one submitted runtime set; the compiled
/// module name is always included automatically.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub struct ProjectVerilogARuntimeRegistration {
    /// Normalized project- or signed-PDK-scoped virtual source identity.
    pub source_key: PathBuf,
    /// Additional netlist model aliases claimed by this artifact.
    pub aliases: Vec<String>,
    /// Validated executable bytecode model.
    pub model: rspice_veriloga::CompiledModel,
    /// Canonical IR paired with `model` for the native runtime.
    pub canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
}

#[cfg(feature = "veriloga")]
#[derive(Debug)]
struct PreparedProjectVerilogARegistration {
    key: PathBuf,
    folded_key: String,
    authority_scope: String,
    aliases: BTreeSet<String>,
    artifact_fingerprint: [u8; 32],
    entry: CachedVerilogAModel,
}

#[cfg(feature = "veriloga")]
#[derive(Default)]
struct ArtifactFingerprintWriter(blake3::Hasher);

#[cfg(feature = "veriloga")]
impl std::io::Write for ArtifactFingerprintWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "veriloga")]
fn runtime_artifact_fingerprint(
    model: &rspice_veriloga::CompiledModel,
    canonical_ir: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<[u8; 32], String> {
    let mut writer = ArtifactFingerprintWriter::default();
    serde_json::to_writer(&mut writer, &(model, canonical_ir))
        .map_err(|error| format!("failed to fingerprint Verilog-A runtime artifact: {error}"))?;
    Ok(*writer.0.finalize().as_bytes())
}

#[cfg(feature = "veriloga")]
fn validate_sealed_runtime_source_key(
    source_key: &Path,
) -> Result<(PathBuf, String, String), String> {
    let key_text = source_key
        .to_str()
        .ok_or_else(|| "sealed Verilog-A runtime keys must contain valid UTF-8 text".to_owned())?;
    let components = key_text.split('/').collect::<Vec<_>>();
    let namespace = components.first().copied().unwrap_or_default();
    if key_text.contains('\\')
        || key_text.chars().any(char::is_control)
        || components.len() < 3
        || !matches!(
            namespace,
            "__rspice_project__" | "__rspice_pdk__" | "__rspice_model_library__"
        )
        || components
            .iter()
            .any(|component| component.is_empty() || matches!(*component, "." | ".."))
    {
        return Err(
            "sealed Verilog-A runtime keys must be normalized content-addressed virtual paths under __rspice_project__/, __rspice_pdk__/, or __rspice_model_library__/"
                .to_owned(),
        );
    }

    let normalized = canonicalize_for_cache(source_key);
    Ok((
        normalized,
        key_text.to_ascii_lowercase(),
        format!(
            "{}:{}",
            namespace.to_ascii_lowercase(),
            components[1].to_ascii_lowercase()
        ),
    ))
}

#[cfg(feature = "veriloga")]
fn validate_project_runtime_alias(alias: &str) -> Result<String, String> {
    if alias.is_empty()
        || alias.trim() != alias
        || alias
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(format!(
            "sealed Verilog-A runtime alias '{alias}' is not a valid model identity"
        ));
    }
    Ok(normalize_model_key(alias))
}

#[cfg(feature = "veriloga")]
fn prepare_project_veriloga_registration(
    registration: ProjectVerilogARuntimeRegistration,
) -> Result<PreparedProjectVerilogARegistration, String> {
    let (key, folded_key, authority_scope) =
        validate_sealed_runtime_source_key(&registration.source_key)?;
    validate_runtime_artifact_pair(&registration.model, Some(&registration.canonical_ir))?;

    let mut aliases = BTreeSet::new();
    aliases.insert(validate_project_runtime_alias(
        registration.model.name.as_str(),
    )?);
    for alias in registration.aliases {
        aliases.insert(validate_project_runtime_alias(&alias)?);
    }
    let artifact_fingerprint =
        runtime_artifact_fingerprint(&registration.model, Some(&registration.canonical_ir))?;
    Ok(PreparedProjectVerilogARegistration {
        key,
        folded_key,
        authority_scope,
        aliases,
        artifact_fingerprint,
        entry: CachedVerilogAModel {
            dependencies: Vec::new(),
            model: std::sync::Arc::new(registration.model),
            canonical_ir: Some(std::sync::Arc::new(registration.canonical_ir)),
        },
    })
}

#[cfg(feature = "veriloga")]
fn register_project_veriloga_runtimes_for_session_with_limit(
    registrations: impl IntoIterator<Item = ProjectVerilogARuntimeRegistration>,
    max_shared_cache_bytes: usize,
) -> Result<(), String> {
    let mut prepared_by_key = BTreeMap::<PathBuf, PreparedProjectVerilogARegistration>::new();
    for registration in registrations {
        let prepared = prepare_project_veriloga_registration(registration)?;
        if let Some(existing) = prepared_by_key.get_mut(&prepared.key) {
            if existing.artifact_fingerprint != prepared.artifact_fingerprint {
                return Err(format!(
                    "sealed Verilog-A runtime key '{}' is claimed by differing artifacts",
                    prepared.key.display()
                ));
            }
            existing.aliases.extend(prepared.aliases);
        } else {
            prepared_by_key.insert(prepared.key.clone(), prepared);
        }
    }

    let prepared = prepared_by_key.into_values().collect::<Vec<_>>();
    if prepared.is_empty() {
        return Ok(());
    }
    let mut folded_keys = BTreeMap::<&str, [u8; 32]>::new();
    let mut scoped_aliases = BTreeMap::<(&str, &str), [u8; 32]>::new();
    for runtime in &prepared {
        if let Some(existing) =
            folded_keys.insert(runtime.folded_key.as_str(), runtime.artifact_fingerprint)
            && existing != runtime.artifact_fingerprint
        {
            return Err(format!(
                "case-colliding sealed Verilog-A runtime key '{}' is claimed by differing artifacts",
                runtime.key.display()
            ));
        }
        for alias in &runtime.aliases {
            if let Some(existing) = scoped_aliases.insert(
                (runtime.authority_scope.as_str(), alias.as_str()),
                runtime.artifact_fingerprint,
            ) && existing != runtime.artifact_fingerprint
            {
                return Err(format!(
                    "sealed Verilog-A alias '{alias}' is claimed by differing artifacts in authority scope '{}'",
                    runtime.authority_scope
                ));
            }
        }
    }

    let incoming_bytes = prepared
        .iter()
        .map(|runtime| veriloga_model_cache_entry_bytes(&runtime.key, &runtime.entry))
        .collect::<Result<Vec<_>, _>>()?;
    let aggregate_incoming_bytes = incoming_bytes
        .iter()
        .copied()
        .fold(0usize, usize::saturating_add);
    ResourceLimitError::ensure(
        ResourceKind::SharedCacheBytes,
        aggregate_incoming_bytes,
        max_shared_cache_bytes,
    )
    .map_err(|error| error.to_string())?;

    let mut cache = veriloga_model_cache()
        .write()
        .map_err(|_| "failed to acquire Verilog-A cache lock".to_owned())?;

    // Compare the whole candidate set against the live registry while holding
    // the one cache lock. Nothing can change between validation and commit.
    for (cached_key, cached_entry) in cache.iter() {
        if !is_sealed_veriloga_virtual_path(cached_key) {
            continue;
        }
        let (_, cached_folded_key, _) = validate_sealed_runtime_source_key(cached_key)?;
        let cached_fingerprint = runtime_artifact_fingerprint(
            cached_entry.model.as_ref(),
            cached_entry.canonical_ir.as_deref(),
        )?;
        for runtime in &prepared {
            if cached_folded_key == runtime.folded_key
                && cached_fingerprint != runtime.artifact_fingerprint
            {
                return Err(format!(
                    "sealed Verilog-A runtime key '{}' collides with a differing installed artifact",
                    runtime.key.display()
                ));
            }
        }
    }

    let mut replacements = Vec::with_capacity(prepared.len());
    for runtime in prepared {
        let entry = if let Some(installed) = cache.get(&runtime.key) {
            let installed_fingerprint = runtime_artifact_fingerprint(
                installed.model.as_ref(),
                installed.canonical_ir.as_deref(),
            )?;
            if installed_fingerprint != runtime.artifact_fingerprint {
                return Err(format!(
                    "sealed Verilog-A runtime key '{}' is already installed with a differing artifact",
                    runtime.key.display()
                ));
            }
            installed.clone()
        } else {
            runtime.entry
        };
        let retained_bytes = veriloga_model_cache_entry_bytes(&runtime.key, &entry)?;
        replacements.push((runtime.key, entry, retained_bytes));
    }

    let replacement_bytes = replacements
        .iter()
        .map(|(_, _, retained_bytes)| *retained_bytes)
        .fold(0usize, usize::saturating_add);
    ResourceLimitError::ensure(
        ResourceKind::SharedCacheBytes,
        replacement_bytes,
        max_shared_cache_bytes,
    )
    .map_err(|error| error.to_string())?;
    cache.try_replace_batch(replacements, max_shared_cache_bytes)
}

/// Atomically register a set of exact in-memory project- or signed-PDK-owned
/// Verilog-A runtimes for this process session.
///
/// The entire set is validated, collision-checked, and budgeted before the
/// shared cache changes. Duplicate or case-colliding keys and model aliases
/// may only identify byte-for-byte identical runtime artifacts. Any validation,
/// collision, allocation, or resource failure leaves the cache unchanged.
/// File-backed cache persistence is intentionally bypassed for these sealed
/// sealed artifacts.
#[cfg(feature = "veriloga")]
pub fn register_project_veriloga_runtimes_for_session(
    registrations: impl IntoIterator<Item = ProjectVerilogARuntimeRegistration>,
) -> Result<(), String> {
    register_project_veriloga_runtimes_for_session_with_limits(
        registrations,
        ResourceLimits::default(),
    )
}

/// Atomically register project-owned Verilog-A runtimes under caller-selected
/// resource governance.
///
/// This is equivalent to [`register_project_veriloga_runtimes_for_session`]
/// except that `limits.max_shared_cache_bytes` bounds the aggregate retained
/// size of the requested runtime set.
#[cfg(feature = "veriloga")]
pub fn register_project_veriloga_runtimes_for_session_with_limits(
    registrations: impl IntoIterator<Item = ProjectVerilogARuntimeRegistration>,
    limits: ResourceLimits,
) -> Result<(), String> {
    register_project_veriloga_runtimes_for_session_with_limit(
        registrations,
        limits.max_shared_cache_bytes,
    )
}

/// Register one exact in-memory project-owned Verilog-A runtime for this
/// process session.
///
/// This compatibility wrapper delegates to the atomic plural API. The compiled
/// module identity is registered as the runtime's sole model alias.
#[cfg(feature = "veriloga")]
pub fn register_project_veriloga_runtime_for_session(
    source_key: impl AsRef<Path>,
    model: rspice_veriloga::CompiledModel,
    canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<(), String> {
    register_project_veriloga_runtimes_for_session([ProjectVerilogARuntimeRegistration {
        source_key: source_key.as_ref().to_path_buf(),
        aliases: Vec::new(),
        model,
        canonical_ir,
    }])
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This allows UI workflows to compile once on import and reuse the compiled
/// artifact during simulation without recompilation. Native JIT builds should
/// use [`register_precompiled_veriloga_runtime_with_dependencies`] so the cache
/// carries canonical IR as well as the compiled model.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    register_precompiled_veriloga_entry_with_dependencies(source_path, dependencies, model, None)
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This compatibility wrapper fingerprints only the source file path.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model(
    source_path: impl AsRef<Path>,
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    let dependency = vec![canonicalize_for_cache(source_path.as_ref())];
    register_precompiled_veriloga_model_with_dependencies(source_path, &dependency, model)
}

#[cfg(all(test, feature = "veriloga", not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_test_root(name: &str) -> PathBuf {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is after the Unix epoch")
            .as_nanos();
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "rspice-veriloga-cache-{name}-{}-{nonce}-{sequence}",
            std::process::id()
        ))
    }

    fn compiled_entry(source_path: &Path) -> CachedVerilogAModel {
        std::fs::write(
            source_path,
            r#"
`include "disciplines.vams"
module cached_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        )
        .expect("write temporary Verilog-A source");

        let runtime = rspice_veriloga::VerilogACompiler::default()
            .compile_file_runtime_with_metadata(source_path, None)
            .expect("compile runtime artifacts");
        let dependencies = fingerprint_paths(&runtime.dependencies).expect("fingerprint source");
        CachedVerilogAModel {
            dependencies,
            model: std::sync::Arc::new(runtime.model),
            canonical_ir: Some(std::sync::Arc::new(runtime.canonical_ir)),
        }
    }

    fn project_registration(
        source_key: impl Into<PathBuf>,
        module_name: &str,
        aliases: &[&str],
    ) -> ProjectVerilogARuntimeRegistration {
        let report = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(
                &format!(
                    "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                ),
                None,
            )
            .expect("compile project runtime");
        ProjectVerilogARuntimeRegistration {
            source_key: source_key.into(),
            aliases: aliases.iter().map(|alias| (*alias).to_owned()).collect(),
            model: report.model,
            canonical_ir: report.canonical_ir,
        }
    }

    fn remove_project_runtime_keys(keys: &[&Path]) {
        let mut cache = veriloga_model_cache().write().expect("cache lock");
        for key in keys {
            cache.remove(&canonicalize_for_cache(key));
        }
    }

    #[test]
    fn disk_cache_loads_a_valid_paired_runtime_artifact() {
        let root = unique_test_root("valid");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");

        persist_model_to_disk_locked(&source_path, &entry, &cache_root)
            .expect("persist valid cache record");
        let loaded = load_model_from_disk_locked(&source_path, &cache_root)
            .expect("load valid cache record")
            .expect("valid cache record is retained");

        assert_eq!(loaded.model.name, entry.model.name);
        assert_eq!(
            loaded
                .canonical_ir
                .as_ref()
                .expect("canonical IR")
                .hir_digest,
            entry
                .canonical_ir
                .as_ref()
                .expect("canonical IR")
                .hir_digest
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn disk_cache_discards_a_stale_canonical_artifact() {
        let root = unique_test_root("stale-artifact");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let mut entry = compiled_entry(&source_path);
        let artifact = std::sync::Arc::make_mut(
            entry
                .canonical_ir
                .as_mut()
                .expect("compiled entry carries canonical IR"),
        );
        artifact.hir_digest = "stale-hir-digest".into();
        let cache_root = root.join("cache");
        let cache_path = cache_record_path_with_root(&source_path, &cache_root);

        persist_model_to_disk_locked(&source_path, &entry, &cache_root)
            .expect("persist stale cache record");
        assert!(cache_path.is_file(), "test must materialize a cache record");
        assert!(
            load_model_from_disk_locked(&source_path, &cache_root)
                .expect("stale cache load is recoverable")
                .is_none(),
            "stale canonical IR must force a cache miss"
        );
        assert!(
            !cache_path.exists(),
            "stale cache record must be removed to prevent repeated failures"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn disk_cache_discards_pre_event_state_records() {
        let root = unique_test_root("stale-event-state-version");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");
        let cache_path = cache_record_path_with_root(&source_path, &cache_root);

        persist_model_to_disk_locked(&source_path, &entry, &cache_root)
            .expect("persist current cache record");
        let file = std::fs::File::open(&cache_path).expect("open current cache record");
        let mut record: serde_json::Value =
            serde_json::from_reader(file).expect("decode current cache record");
        record["version"] = serde_json::Value::from(19_u32);
        let file = std::fs::File::create(&cache_path).expect("replace cache record version");
        serde_json::to_writer(file, &record).expect("encode stale cache record");

        assert!(
            load_model_from_disk_locked(&source_path, &cache_root)
                .expect("stale-version cache load is recoverable")
                .is_none(),
            "pre-event-state cache records must force a miss"
        );
        assert!(
            !cache_path.exists(),
            "stale-version cache record must be removed"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn oversized_disk_cache_read_becomes_a_miss_without_deleting_the_record() {
        let root = unique_test_root("bounded-read");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");
        persist_model_to_disk_locked(&source_path, &entry, &cache_root)
            .expect("persist valid cache record");
        let cache_path = cache_record_path_with_root(&source_path, &cache_root);
        let cache_bytes = usize::try_from(
            std::fs::metadata(&cache_path)
                .expect("cache metadata")
                .len(),
        )
        .expect("cache record fits usize");
        let limits = ResourceLimits {
            max_shared_cache_bytes: cache_bytes.saturating_sub(1),
            ..ResourceLimits::default()
        };

        assert!(
            load_model_from_disk_locked_with_limits(&source_path, &cache_root, limits, &NoAbort,)
                .expect("an oversized optimization is a recoverable cache miss")
                .is_none()
        );
        assert!(
            cache_path.is_file(),
            "a caller-specific limit must not destroy a valid shared record"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn disk_cache_write_streams_within_the_resource_budget() {
        let root = unique_test_root("bounded-write");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");
        let cache_path = cache_record_path_with_root(&source_path, &cache_root);
        let limits = ResourceLimits {
            max_shared_cache_bytes: 1,
            ..ResourceLimits::default()
        };

        let error =
            persist_model_to_disk_locked_with_limits(&source_path, &entry, &cache_root, limits)
                .expect_err("oversized cache record must not be persisted");
        assert!(error.contains("exceeds the 1 byte write limit"), "{error}");
        assert!(!cache_path.exists());
        assert!(
            !cache_path
                .with_extension(format!("tmp.{}", std::process::id()))
                .exists(),
            "failed bounded writes must clean up their temporary file"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn dependency_fingerprints_enforce_aggregate_source_bytes() {
        let root = unique_test_root("dependency-budget");
        std::fs::create_dir_all(&root).expect("create temporary dependency root");
        let first = root.join("first.va");
        let second = root.join("second.va");
        std::fs::write(&first, b"0123456789").expect("write first dependency");
        std::fs::write(&second, b"abcdefghij").expect("write second dependency");
        let limits = ResourceLimits {
            max_dependency_source_bytes: 15,
            ..ResourceLimits::default()
        };

        let error = fingerprint_paths_with_limits(&[first, second], limits)
            .expect_err("aggregate dependency bytes must be bounded");
        let SimulationError::ResourceLimit(error) = error else {
            panic!("unexpected fingerprint error: {error}");
        };
        assert_eq!(error.resource, ResourceKind::DependencySourceBytes);
        assert_eq!(error.requested, 20);
        assert_eq!(error.limit, 15);

        std::fs::remove_dir_all(root).expect("remove temporary dependency root");
    }

    #[test]
    fn oversized_model_is_not_admitted_to_the_shared_memory_cache() {
        let root = unique_test_root("memory-budget");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let key = canonicalize_for_cache(&source_path);
        let retained_bytes =
            veriloga_model_cache_entry_bytes(&key, &entry).expect("size compiled model");

        assert!(
            !retain_veriloga_model(
                key.clone(),
                entry.clone(),
                retained_bytes.saturating_sub(1),
                false,
            )
            .expect("optional cache insertion is recoverable")
        );
        assert!(
            veriloga_model_cache()
                .read()
                .expect("cache lock")
                .get(&key)
                .is_none()
        );
        let error = retain_veriloga_model(key, entry, retained_bytes.saturating_sub(1), true)
            .expect_err("required project retention must fail closed");
        assert!(
            error.contains("shared_cache_bytes limit exceeded"),
            "{error}"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn project_owned_runtime_registration_requires_no_ambient_source_file() {
        let source_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000001/0123456789abcdef/model.va",
        );
        assert!(!source_key.exists());
        let report = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(
                "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
                None,
            )
            .expect("compile in-memory project source");

        register_project_veriloga_runtime_for_session(
            &source_key,
            report.model,
            report.canonical_ir,
        )
        .expect("register in-memory project runtime");

        let key = canonicalize_for_cache(&source_key);
        let mut cache = veriloga_model_cache().write().expect("cache lock");
        let entry = cache.get(&key).expect("session entry");
        assert!(entry.dependencies.is_empty());
        assert_eq!(entry.model.name.as_str(), "owned");
        assert!(entry.canonical_ir.is_some());
        cache.remove(&key);
    }

    #[test]
    fn model_library_runtime_registration_requires_no_ambient_source_file() {
        let source_key =
            PathBuf::from("__rspice_model_library__/0123456789abcdef/root-digest/retained.va");
        assert!(!source_key.exists());
        let report = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(
                "module retained(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
                None,
            )
            .expect("compile in-memory model-library source");

        register_project_veriloga_runtime_for_session(
            &source_key,
            report.model,
            report.canonical_ir,
        )
        .expect("register sealed model-library runtime");

        let key = canonicalize_for_cache(&source_key);
        let mut cache = veriloga_model_cache().write().expect("cache lock");
        let entry = cache.get(&key).expect("session entry");
        assert!(entry.dependencies.is_empty());
        assert_eq!(entry.model.name.as_str(), "retained");
        cache.remove(&key);
    }

    #[test]
    fn same_file_name_in_distinct_projects_resolves_only_its_registered_runtime() {
        let compile = |module_name: &str| {
            rspice_veriloga::VerilogACompiler::default()
                .compile_runtime(
                    &format!(
                        "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                    ),
                    None,
                )
                .expect("compile project runtime")
        };
        let first_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000011/digest/model.va",
        );
        let second_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000022/digest/model.va",
        );
        let first = compile("first_owned");
        let second = compile("second_owned");
        register_project_veriloga_runtime_for_session(&first_key, first.model, first.canonical_ir)
            .unwrap();
        register_project_veriloga_runtime_for_session(
            &second_key,
            second.model,
            second.canonical_ir,
        )
        .unwrap();

        assert_eq!(
            resolve_cached_or_compile_veriloga(&first_key)
                .unwrap()
                .model
                .name
                .as_str(),
            "first_owned"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&second_key)
                .unwrap()
                .model
                .name
                .as_str(),
            "second_owned"
        );

        let mut cache = veriloga_model_cache().write().unwrap();
        cache.remove(&canonicalize_for_cache(&first_key));
        cache.remove(&canonicalize_for_cache(&second_key));
    }

    #[test]
    fn plural_project_runtime_registration_installs_the_entire_batch() {
        let first_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000101/a/first.va");
        let second_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000101/b/second.va");

        register_project_veriloga_runtimes_for_session([
            project_registration(&first_key, "batch_first", &["FIRST_ALIAS"]),
            project_registration(&second_key, "batch_second", &["SECOND_ALIAS"]),
        ])
        .expect("register complete project runtime batch");

        assert_eq!(
            resolve_cached_or_compile_veriloga(&first_key)
                .expect("first runtime")
                .model
                .name
                .as_str(),
            "batch_first"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&second_key)
                .expect("second runtime")
                .model
                .name
                .as_str(),
            "batch_second"
        );
        remove_project_runtime_keys(&[&first_key, &second_key]);
    }

    #[test]
    fn plural_registration_rolls_back_every_entry_on_an_installed_key_collision() {
        let project = "00000000-0000-0000-0000-000000000102";
        let installed_key = PathBuf::from(format!("__rspice_project__/{project}/stable/model.va"));
        let candidate_key =
            PathBuf::from(format!("__rspice_project__/{project}/candidate/model.va"));
        register_project_veriloga_runtimes_for_session([project_registration(
            &installed_key,
            "stable_model",
            &[],
        )])
        .expect("install stable runtime");

        let error = register_project_veriloga_runtimes_for_session([
            project_registration(&candidate_key, "candidate_model", &[]),
            project_registration(&installed_key, "conflicting_model", &[]),
        ])
        .expect_err("a differing installed artifact must reject the whole batch");
        assert!(error.contains("differing installed artifact"), "{error}");
        assert_eq!(
            resolve_cached_or_compile_veriloga(&installed_key)
                .expect("stable runtime remains installed")
                .model
                .name
                .as_str(),
            "stable_model"
        );
        assert!(
            resolve_cached_or_compile_veriloga(&candidate_key).is_err(),
            "a rejected batch must not publish an earlier candidate"
        );
        remove_project_runtime_keys(&[&installed_key, &candidate_key]);
    }

    #[test]
    fn plural_registration_rolls_back_on_aggregate_resource_failure() {
        let stable_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000103/stable/model.va",
        );
        let candidate_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000103/candidate/model.va",
        );
        register_project_veriloga_runtimes_for_session([project_registration(
            &stable_key,
            "resource_stable",
            &[],
        )])
        .expect("install stable runtime");

        let candidate = project_registration(&candidate_key, "resource_candidate", &[]);
        let prepared = prepare_project_veriloga_registration(candidate.clone())
            .expect("prepare candidate runtime");
        let required = veriloga_model_cache_entry_bytes(&prepared.key, &prepared.entry)
            .expect("size candidate runtime");
        let error = register_project_veriloga_runtimes_for_session_with_limit(
            [candidate],
            required.saturating_sub(1),
        )
        .expect_err("an undersized aggregate budget must reject the whole batch");
        assert!(
            error.contains("shared_cache_bytes limit exceeded"),
            "{error}"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&stable_key)
                .expect("stable runtime remains installed")
                .model
                .name
                .as_str(),
            "resource_stable"
        );
        assert!(resolve_cached_or_compile_veriloga(&candidate_key).is_err());
        remove_project_runtime_keys(&[&stable_key, &candidate_key]);
    }

    #[test]
    fn differing_case_colliding_keys_are_rejected_without_partial_installation() {
        let upper_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000104/digest/Model.va",
        );
        let lower_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000104/digest/model.va",
        );
        let error = register_project_veriloga_runtimes_for_session([
            project_registration(&upper_key, "upper_model", &[]),
            project_registration(&lower_key, "lower_model", &[]),
        ])
        .expect_err("case-colliding keys with differing artifacts must fail");
        assert!(error.contains("case-colliding"), "{error}");
        assert!(resolve_cached_or_compile_veriloga(&upper_key).is_err());
        assert!(resolve_cached_or_compile_veriloga(&lower_key).is_err());
    }

    #[test]
    fn differing_case_colliding_aliases_are_rejected_within_one_project() {
        let first_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000105/a/first.va");
        let second_key =
            PathBuf::from("__rspice_project__/00000000-0000-0000-0000-000000000105/b/second.va");
        let error = register_project_veriloga_runtimes_for_session([
            project_registration(&first_key, "alias_first", &["SharedAlias"]),
            project_registration(&second_key, "alias_second", &["sharedalias"]),
        ])
        .expect_err("case-colliding aliases with differing artifacts must fail");
        assert!(error.contains("alias 'SHAREDALIAS'"), "{error}");
        assert!(resolve_cached_or_compile_veriloga(&first_key).is_err());
        assert!(resolve_cached_or_compile_veriloga(&second_key).is_err());
    }

    #[test]
    fn identical_reinstall_is_idempotent_and_preserves_the_cached_artifact() {
        let key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000106/digest/model.va",
        );
        let registration = project_registration(&key, "idempotent_model", &["stable_alias"]);
        register_project_veriloga_runtimes_for_session([registration.clone()])
            .expect("initial registration");
        let before = resolve_cached_or_compile_veriloga(&key).expect("initial cached runtime");

        register_project_veriloga_runtimes_for_session([registration])
            .expect("identical reinstall");
        let after = resolve_cached_or_compile_veriloga(&key).expect("reinstalled cached runtime");
        assert!(std::sync::Arc::ptr_eq(&before.model, &after.model));
        remove_project_runtime_keys(&[&key]);
    }

    #[test]
    fn cache_telemetry_reports_memory_hits_as_monotonic_deltas() {
        let key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000109/digest/telemetry.va",
        );
        register_project_veriloga_runtimes_for_session([project_registration(
            &key,
            "telemetry_model",
            &[],
        )])
        .expect("register telemetry runtime");
        let before = veriloga_cache_telemetry();

        resolve_cached_or_compile_veriloga(&key).expect("first memory hit");
        resolve_cached_or_compile_veriloga(&key).expect("second memory hit");
        let after = veriloga_cache_telemetry();

        assert!(after.lookups.saturating_sub(before.lookups) >= 2);
        assert!(after.memory_hits.saturating_sub(before.memory_hits) >= 2);
        remove_project_runtime_keys(&[&key]);
    }

    #[test]
    fn cache_resolution_honors_cancellation_before_io() {
        let before = veriloga_cache_telemetry();
        let error = resolve_cached_or_compile_veriloga_with_limits_and_abort(
            Path::new("must-not-be-read.va"),
            ResourceLimits::default(),
            &crate::abort_signal::ImmediateAbort,
        )
        .expect_err("immediate cancellation must stop cache resolution");
        let after = veriloga_cache_telemetry();

        assert!(matches!(error, SimulationError::Aborted));
        assert!(after.lookups > before.lookups);
    }

    struct AbortOnCompilerProgress {
        aborted: std::sync::atomic::AtomicBool,
    }

    impl AbortSignal for AbortOnCompilerProgress {
        fn is_aborted(&self) -> bool {
            self.aborted.load(std::sync::atomic::Ordering::Relaxed)
        }

        fn observe_progress(&self, fraction: f64) {
            if fraction > 0.0 {
                self.aborted
                    .store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    #[test]
    fn cache_maps_compiler_cancellation_to_simulation_abort() {
        let root = unique_test_root("compile-cancellation");
        std::fs::create_dir_all(&root).expect("create cancellation fixture root");
        let source = root.join("cancel.va");
        std::fs::write(
            &source,
            "module cancel(p,n); inout p,n; electrical p,n; analog I(p,n) <+ V(p,n); endmodule\n",
        )
        .expect("write cancellation fixture");
        let abort = AbortOnCompilerProgress {
            aborted: std::sync::atomic::AtomicBool::new(false),
        };
        let before = veriloga_cache_telemetry();

        let error = resolve_cached_or_compile_veriloga_with_limits_and_abort(
            &source,
            ResourceLimits::default(),
            &abort,
        )
        .expect_err("compiler progress callback must cancel cache resolution");
        let after = veriloga_cache_telemetry();

        assert!(matches!(error, SimulationError::Aborted));
        assert!(after.compilations_started > before.compilations_started);
        assert!(after.compilations_cancelled > before.compilations_cancelled);
        std::fs::remove_dir_all(root).expect("remove cancellation fixture root");
    }

    #[test]
    fn same_filename_and_alias_remain_isolated_between_projects() {
        let first_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000107/digest/device.va",
        );
        let second_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000108/digest/device.va",
        );
        register_project_veriloga_runtimes_for_session([
            project_registration(&first_key, "isolated_first", &["device_alias"]),
            project_registration(&second_key, "isolated_second", &["DEVICE_ALIAS"]),
        ])
        .expect("project-scoped aliases may be reused across projects");

        assert_eq!(
            resolve_cached_or_compile_veriloga(&first_key)
                .expect("first project runtime")
                .model
                .name
                .as_str(),
            "isolated_first"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&second_key)
                .expect("second project runtime")
                .model
                .name
                .as_str(),
            "isolated_second"
        );
        remove_project_runtime_keys(&[&first_key, &second_key]);
    }

    #[test]
    fn project_virtual_cache_identity_is_lexical_even_if_an_ambient_path_exists() {
        let unique = unique_test_root("virtual-lexical")
            .file_name()
            .expect("unique component")
            .to_string_lossy()
            .into_owned();
        let source_key = PathBuf::from(format!(
            "__rspice_project__/{unique}/0123456789abcdef/model.va"
        ));
        let parent = source_key.parent().expect("virtual key parent");
        std::fs::create_dir_all(parent).expect("materialize adversarial ambient directory");
        std::fs::write(
            &source_key,
            "ambient bytes must never define cache identity",
        )
        .expect("materialize adversarial ambient file");

        assert_eq!(canonicalize_for_cache(&source_key), source_key);

        std::fs::remove_dir_all(PathBuf::from("__rspice_project__").join(&unique))
            .expect("remove adversarial ambient path");
        assert_eq!(canonicalize_for_cache(&source_key), source_key);
    }

    #[test]
    fn missing_project_virtual_runtime_never_falls_back_to_ambient_io() {
        let source_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000099/missing/model.va",
        );
        let error = resolve_cached_or_compile_veriloga(&source_key)
            .expect_err("unregistered project runtime must fail closed");
        assert!(
            error
                .to_string()
                .contains("is not installed for this execution")
        );
    }

    #[test]
    fn case_altered_project_virtual_path_never_compiles_an_ambient_file() {
        let unique = unique_test_root("virtual-case")
            .file_name()
            .expect("unique component")
            .to_string_lossy()
            .into_owned();
        let exact_source_key = PathBuf::from(format!(
            "__rspice_project__/{unique}/0123456789abcdef/model.va"
        ));
        std::fs::create_dir_all(exact_source_key.parent().expect("virtual key parent"))
            .expect("materialize adversarial ambient directory");
        std::fs::write(
            &exact_source_key,
            "module ambient_escape(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
        )
        .expect("materialize compilable adversarial ambient source");

        let altered_source_key = PathBuf::from(format!(
            "__RSPICE_PROJECT__/{unique}/0123456789abcdef/model.va"
        ));
        let error = resolve_cached_or_compile_veriloga(&altered_source_key)
            .expect_err("case-altered project key must fail before ambient compilation");
        assert!(
            error
                .to_string()
                .contains("is not installed for this execution"),
            "unexpected error: {error}"
        );

        std::fs::remove_dir_all(PathBuf::from("__rspice_project__").join(unique))
            .expect("remove adversarial ambient path");
    }
}

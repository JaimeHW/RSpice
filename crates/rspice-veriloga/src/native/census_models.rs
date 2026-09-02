//! One front-end compile of the shipped model set, shared by every census.
//!
//! Five release-qualification censuses walk the same 43 shipped modules:
//! [`code_identity`](super::code_identity),
//! [`branch_agreement`](super::branch_agreement), and the residual, Jacobian
//! and state-slot censuses in [`cfg_census`](super::cfg_census). Each one used
//! to open by compiling the whole corpus from source, and the front end is by
//! far the expensive half — measured at 836 s of a 1001 s identity census,
//! with one model alone costing 241 s. Running them therefore paid for that
//! same compile once per census.
//!
//! [`shipped_census_models`] is the single provider they all now consume. It
//! yields one module at a time, front-end compiled, and keeps the artifacts in
//! an on-disk cache under the cargo target directory.
//!
//! On disk rather than in memory, for two reasons. The identity census is
//! known to peak near 19 GB of working set on one module, so holding all 43
//! front-end artifacts live at once would be an out-of-memory risk on a 32 GB
//! host; the iterator keeps exactly one alive. And the gate runs the three
//! censuses as three separate cargo processes, which no in-process map can
//! span.
//!
//! # What makes a cached entry answerable
//!
//! A cache that silently answered for changed inputs would turn these gates
//! into a lie, so the key covers every input that can change what the front
//! end produces:
//!
//! 1. `source_tree_digest` and `generator_digest`, read from the shipped model
//!    manifest;
//! 2. the contents of every file under the model root the censuses actually
//!    compile, which is what catches a source edited in place without
//!    regenerating the manifest;
//! 3. this test binary itself, because the manifest covers the models and the
//!    generator but *not* the compiler crate — without it, editing the parser,
//!    the analyzer or HIR/MIR lowering would leave a stale cache hiding the
//!    very regression the census exists to catch;
//! 4. [`CACHE_SCHEMA_VERSION`], bumped by hand when the cached shape changes;
//! 5. per entry, the source path, the module name and the compile profile's
//!    defines and undefines.
//!
//! Entries are written to a temporary file and renamed into place, so a killed
//! run cannot leave a truncated entry behind. Any failure to read one back is
//! a miss that recompiles, never an error and never a pass — including a
//! [`CanonicalIrArtifact::validate`] that refuses what came back, which is the
//! backstop against an encoding that stops being faithful.
//!
//! A whole entry set is over ten gigabytes, so opening the cache drops every
//! keyed directory but the current one rather than stranding one per revision.
//!
//! Set `RSPICE_CENSUS_CACHE=0` to compile from source and write nothing, which
//! is how a cached census is checked against an uncached one.
//!
//! # This module requires `serde_json`'s `float_roundtrip` feature
//!
//! It is declared in this crate's `[dev-dependencies]` and must stay there.
//! `serde_json`'s default float parser is *not* correctly rounded, and the
//! shipped models are full of constants it reads back wrong by one unit in the
//! last place: `1.3806505e-23` returns as `1.3806504999999999e-23`. That is
//! enough to move the canonical IR's own HIR digest, so without the feature
//! this cache hands back artifacts that are not what the compiler produced and
//! the censuses fail on the first cache hit. The feature pulls in no crate and
//! does not move `Cargo.lock`. `finite_floats_survive_the_encoding_the_cache_uses`
//! fails without it, so it cannot be dropped as unused silently.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::canonical_ir::CanonicalIrArtifact;
use crate::codegen::CompiledModel;
use crate::rust_backend::{discover_veriloga_sources, parse_generated_builtin_manifest};
use crate::{CompilerOptions, VerilogACompiler};

/// Bump by hand whenever [`CachedEntry`] changes shape.
const CACHE_SCHEMA_VERSION: u32 = 1;

const CACHE_DIR_NAME: &str = "veriloga-census-cache";

/// The model tree the censuses compile.
pub(crate) fn shipped_model_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
}

/// One shipped module, front-end compiled, with the seconds that cost.
pub(crate) struct CensusModel {
    pub name: String,
    /// The source the module was declared in, for the messages a census
    /// panics with.
    pub path: PathBuf,
    pub model: CompiledModel,
    pub canonical_ir: CanonicalIrArtifact,
    /// Seconds spent obtaining the two artifacts, whether by compiling them
    /// or by reading them back.
    pub compile_seconds: f64,
    pub from_cache: bool,
}

/// Every shipped module, in the order the censuses have always seen them.
pub(crate) fn shipped_census_models() -> impl Iterator<Item = CensusModel> {
    shipped_census_models_matching(None)
}

/// The same sequence, narrowed to modules whose name contains `substring`.
///
/// The filter is applied before compiling rather than after, so a run
/// investigating one model does not pay for the other forty-two.
pub(crate) fn shipped_census_models_matching(
    substring: Option<&str>,
) -> impl Iterator<Item = CensusModel> {
    let root = shipped_model_root();
    let candidates = discover_veriloga_sources(&root).expect("discover shipped Verilog-A sources");
    let mut pending = Vec::new();
    for candidate in candidates {
        for module in &candidate.modules {
            if substring.is_some_and(|substring| !module.contains(substring)) {
                continue;
            }
            pending.push(PendingModel {
                path: candidate.path.clone(),
                module: module.clone(),
                defines: candidate.compile_profile.defines.clone(),
                undefines: candidate.compile_profile.undefines.clone(),
            });
        }
    }
    let cache = CacheDir::open(&root);
    ShippedCensusModels {
        root,
        cache,
        pending: pending.into_iter(),
    }
}

/// One discovered module, before anything has been compiled for it.
struct PendingModel {
    path: PathBuf,
    module: String,
    defines: Vec<(String, Option<String>)>,
    undefines: Vec<String>,
}

struct ShippedCensusModels {
    root: PathBuf,
    cache: Option<CacheDir>,
    pending: std::vec::IntoIter<PendingModel>,
}

impl Iterator for ShippedCensusModels {
    type Item = CensusModel;

    fn next(&mut self) -> Option<CensusModel> {
        let pending = self.pending.next()?;
        let started = Instant::now();

        let entry_file = self
            .cache
            .as_ref()
            .map(|cache| cache.entry_file(&self.root, &pending));
        if let (Some(cache), Some(entry_file)) = (self.cache.as_ref(), entry_file.as_ref())
            && let Some((model, canonical_ir)) = cache.load(entry_file, &pending.module)
        {
            return Some(CensusModel {
                name: pending.module,
                path: pending.path,
                model,
                canonical_ir,
                compile_seconds: started.elapsed().as_secs_f64(),
                from_cache: true,
            });
        }

        let mut options = CompilerOptions::default();
        options.include_paths.push(self.root.clone());
        options.defines = pending.defines.clone();
        options.undefines = pending.undefines.clone();
        let compiler = VerilogACompiler::new(options);
        let runtime = compiler
            .compile_file_runtime_with_metadata(&pending.path, Some(&pending.module))
            .unwrap_or_else(|error| {
                panic!(
                    "compile {} :: {}: {error}",
                    pending.path.display(),
                    pending.module
                )
            });
        let compile_seconds = started.elapsed().as_secs_f64();

        if let (Some(cache), Some(entry_file)) = (self.cache.as_ref(), entry_file.as_ref()) {
            cache.store(
                entry_file,
                &pending.module,
                &runtime.model,
                &runtime.canonical_ir,
            );
        }

        Some(CensusModel {
            name: pending.module,
            path: pending.path,
            model: runtime.model,
            canonical_ir: runtime.canonical_ir,
            compile_seconds,
            from_cache: false,
        })
    }
}

/// What a cache entry holds, and what it has to agree with to be answerable.
#[derive(serde::Deserialize)]
struct CachedEntry {
    /// Hex of the entry key. Stored as well as spelled in the file name so a
    /// file that somehow ends up under the wrong name still cannot answer.
    entry_key: String,
    module: String,
    model: CompiledModel,
    canonical_ir: CanonicalIrArtifact,
}

/// The same entry, borrowed, for the write side.
///
/// Cloning the two artifacts to serialize them would double the live bytes of
/// the largest models for no reason; the field names match [`CachedEntry`], so
/// what this writes is what that reads.
#[derive(serde::Serialize)]
struct CachedEntryRef<'a> {
    entry_key: &'a str,
    module: &'a str,
    model: &'a CompiledModel,
    canonical_ir: &'a CanonicalIrArtifact,
}

/// A keyed directory of cached front-end artifacts.
struct CacheDir {
    dir: PathBuf,
    /// The digest of every global input; folded into each entry key.
    key: blake3::Hash,
}

impl CacheDir {
    fn open(root: &Path) -> Option<Self> {
        if std::env::var("RSPICE_CENSUS_CACHE").is_ok_and(|value| value == "0") {
            eprintln!("census-cache enabled=false reason=RSPICE_CENSUS_CACHE=0");
            return None;
        }
        let started = Instant::now();
        match Self::keyed(root) {
            Ok(cache) => {
                eprintln!(
                    "census-cache enabled=true key_seconds={:.2} dir={}",
                    started.elapsed().as_secs_f64(),
                    cache.dir.display()
                );
                Some(cache)
            }
            Err(reason) => {
                eprintln!("census-cache enabled=false reason={reason}");
                None
            }
        }
    }

    fn keyed(root: &Path) -> Result<Self, String> {
        let target =
            target_dir().ok_or_else(|| "cannot locate the cargo target dir".to_string())?;

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"rspice-veriloga census front-end cache");
        hasher.update(&CACHE_SCHEMA_VERSION.to_le_bytes());

        // The shipped model manifest's own digests, read rather than
        // recomputed: they cover the model sources and the generator that
        // turned them into crates.
        let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("rspice-veriloga-models")
            .join("manifest.txt");
        let manifest_text = std::fs::read_to_string(&manifest_path)
            .map_err(|error| format!("read {}: {error}", manifest_path.display()))?;
        let manifest = parse_generated_builtin_manifest(&manifest_text)
            .ok_or_else(|| format!("parse {}", manifest_path.display()))?;
        hasher.update(manifest.source_tree_digest.as_bytes());
        hasher.update(manifest.generator_digest.as_bytes());

        // The sources the census actually compiles. The manifest's digest is
        // only as fresh as the last regeneration, and a model edited in place
        // without one is exactly the change a stale cache would hide.
        hasher.update(
            tree_digest(root)
                .map_err(|error| format!("digest {}: {error}", root.display()))?
                .as_bytes(),
        );

        // This binary. Nothing above covers the compiler crate itself.
        let exe =
            std::env::current_exe().map_err(|error| format!("locate the test binary: {error}"))?;
        let exe_bytes =
            std::fs::read(&exe).map_err(|error| format!("read {}: {error}", exe.display()))?;
        hasher.update(blake3::hash(&exe_bytes).as_bytes());

        let key = hasher.finalize();
        let root = target.join(CACHE_DIR_NAME);
        let dir = root.join(hex(key.as_bytes(), 16));
        std::fs::create_dir_all(&dir)
            .map_err(|error| format!("create {}: {error}", dir.display()))?;
        evict_other_keys(&root, &dir);
        Ok(Self { dir, key })
    }

    /// Where one module's entry lives, and what it has to claim to be it.
    fn entry_file(&self, root: &Path, pending: &PendingModel) -> EntryFile {
        let relative = pending.path.strip_prefix(root).unwrap_or(&pending.path);
        let descriptor = serde_json::to_string(&(
            relative.to_string_lossy(),
            &pending.module,
            &pending.defines,
            &pending.undefines,
        ))
        .unwrap_or_default();
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.key.as_bytes());
        hasher.update(descriptor.as_bytes());
        let entry_key = hex(hasher.finalize().as_bytes(), 32);

        let stem: String = pending
            .module
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() || character == '_' || character == '-' {
                    character
                } else {
                    '_'
                }
            })
            .collect();
        EntryFile {
            path: self.dir.join(format!("{stem}.{}.json", &entry_key[..16])),
            entry_key,
        }
    }

    /// Read an entry back, or answer `None` and let the caller compile.
    ///
    /// Every failure — a missing file, a truncated one, a shape this revision
    /// no longer parses, a key that does not match — is a miss. None of them
    /// is an error, and none of them is a pass.
    fn load(
        &self,
        entry: &EntryFile,
        module: &str,
    ) -> Option<(CompiledModel, CanonicalIrArtifact)> {
        // Read whole and parse from the slice rather than through a reader:
        // `serde_json`'s reader path pulls a byte at a time and is several
        // times slower, and the encoded bytes are small beside the artifacts
        // they decode into.
        let encoded = std::fs::read(&entry.path).ok()?;
        let cached: CachedEntry = serde_json::from_slice(&encoded).ok()?;
        if cached.entry_key != entry.entry_key || cached.module != module {
            return None;
        }
        // The artifact digests its own HIR and MIR, so it can say whether what
        // came back is what went in. Anything a lossy encoding quietly changed
        // fails here and is recompiled — which is how the census keeps meaning
        // what it says even if a future field stops round-tripping.
        if let Err(errors) = cached.canonical_ir.validate() {
            eprintln!(
                "census-cache model={module} loaded=false reason=validate:{}",
                errors
                    .first()
                    .map_or_else(|| "unknown".to_string(), |first| first.message.to_string())
            );
            return None;
        }
        Some((cached.model, cached.canonical_ir))
    }

    fn store(
        &self,
        entry: &EntryFile,
        module: &str,
        model: &CompiledModel,
        canonical_ir: &CanonicalIrArtifact,
    ) {
        let started = Instant::now();
        let cached = CachedEntryRef {
            entry_key: &entry.entry_key,
            module,
            model,
            canonical_ir,
        };

        // JSON has no spelling for an infinity or a NaN, so `serde_json`
        // writes one as `null`. A bare `f64` then refuses to load, which this
        // cache would treat as a permanent miss; but an `Option<f64>` loads
        // `null` as `None`, which is a silent change to the artifact. Compact
        // models are full of `from (0:inf)` ranges, so this is not
        // hypothetical. An entry carrying one is simply not cached.
        match count_non_finite_floats(&cached) {
            Ok(0) => {}
            Ok(count) => {
                eprintln!(
                    "census-cache model={module} stored=false reason=non_finite_floats:{count}"
                );
                return;
            }
            Err(error) => {
                eprintln!("census-cache model={module} stored=false reason=scan:{error}");
                return;
            }
        }

        match write_atomically(&entry.path, &cached) {
            Ok(bytes) => eprintln!(
                "census-cache model={module} stored=true bytes={bytes} store_seconds={:.2}",
                started.elapsed().as_secs_f64()
            ),
            Err(error) => {
                eprintln!("census-cache model={module} stored=false reason=write:{error}");
            }
        }
    }
}

struct EntryFile {
    path: PathBuf,
    entry_key: String,
}

/// Serialize into a temporary file and rename it into place.
///
/// A census that is killed part way through must not leave a half-written
/// entry that a later run deserializes into something plausible.
fn write_atomically(path: &Path, entry: &CachedEntryRef<'_>) -> std::io::Result<u64> {
    let temporary = path.with_extension(format!("tmp{}", std::process::id()));
    let result = (|| -> std::io::Result<u64> {
        let file = std::fs::File::create(&temporary)?;
        let mut writer = std::io::BufWriter::with_capacity(1 << 20, file);
        serde_json::to_writer(&mut writer, entry)?;
        writer.flush()?;
        let bytes = writer.get_ref().metadata()?.len();
        drop(writer);
        std::fs::rename(&temporary, path)?;
        Ok(bytes)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    result
}

/// Drop every keyed directory but the one this run answers from.
///
/// An entry set for the whole corpus is over ten gigabytes, and the key
/// includes the test binary, so without this every edit to the compiler crate
/// would strand another ten gigabytes in the target directory — on a host that
/// already runs into disk pressure. Only one test binary is current per target
/// directory at a time, so the others cannot answer for anything.
fn evict_other_keys(root: &Path, keep: &Path) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path == keep || !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        match std::fs::remove_dir_all(&path) {
            Ok(()) => eprintln!("census-cache evicted={}", path.display()),
            Err(error) => {
                eprintln!(
                    "census-cache evict_failed={} reason={error}",
                    path.display()
                );
            }
        }
    }
}

/// The cargo target directory this test binary was built into.
///
/// Derived from the running binary rather than from a compile-time constant,
/// so a run under `CARGO_TARGET_DIR` caches beside its own artifacts.
fn target_dir() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    for ancestor in exe.ancestors().skip(1) {
        if ancestor.join("CACHEDIR.TAG").is_file() {
            return Some(ancestor.to_path_buf());
        }
    }
    // `<target>/<profile>/deps/<binary>`.
    exe.ancestors().nth(3).map(Path::to_path_buf)
}

/// A content digest over every file under `root`, in sorted path order.
fn tree_digest(root: &Path) -> std::io::Result<blake3::Hash> {
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    files.sort();
    let mut hasher = blake3::Hasher::new();
    for file in &files {
        let relative = file.strip_prefix(root).unwrap_or(file);
        hasher.update(relative.to_string_lossy().as_bytes());
        hasher.update(b"\0");
        hasher.update(blake3::hash(&std::fs::read(file)?).as_bytes());
    }
    Ok(hasher.finalize())
}

fn collect_files(directory: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

fn hex(bytes: &[u8], count: usize) -> String {
    bytes
        .iter()
        .take(count)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// How many non-finite floats a value would hand to a serializer.
///
/// Counted by driving `serde` with a serializer that keeps no output at all,
/// so the answer is exact and costs no memory. JSON is otherwise lossless for
/// these artifacts: every other scalar `serde_json` writes round-trips
/// exactly, and `CompiledModel` already travels this way in production.
fn count_non_finite_floats<T>(value: &T) -> Result<usize, serde_json::Error>
where
    T: serde::Serialize,
{
    let mut counter = NonFiniteFloats(0);
    value.serialize(&mut counter)?;
    Ok(counter.0)
}

struct NonFiniteFloats(usize);

impl serde::Serializer for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i8(self, _: i8) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i16(self, _: i16) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i32(self, _: i32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i64(self, _: i64) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_i128(self, _: i128) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u8(self, _: u8) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u16(self, _: u16) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u32(self, _: u32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u64(self, _: u64) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_u128(self, _: u128) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_f32(self, value: f32) -> Result<(), Self::Error> {
        if !value.is_finite() {
            self.0 += 1;
        }
        Ok(())
    }
    fn serialize_f64(self, value: f64) -> Result<(), Self::Error> {
        if !value.is_finite() {
            self.0 += 1;
        }
        Ok(())
    }
    fn serialize_char(self, _: char) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_str(self, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_none(self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_some<T>(self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn serialize_newtype_struct<T>(self, _: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }
    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

impl serde::ser::SerializeSeq for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeTuple for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeTupleStruct for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeTupleVariant for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeMap for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        key.serialize(&mut **self)
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeStruct for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl serde::ser::SerializeStructVariant for &mut NonFiniteFloats {
    type Ok = ();
    type Error = serde_json::Error;
    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_finite_value_reports_no_non_finite_floats() {
        let value = (1.0_f64, vec![0.5_f64, -3.25], Some(7.0_f64), None::<f64>);
        assert_eq!(count_non_finite_floats(&value).unwrap(), 0);
    }

    #[test]
    fn the_scan_finds_every_shape_json_would_lose() {
        // The three losses that matter, in the three places they hide: a bare
        // field, an `Option` that would come back `None`, and a value nested
        // inside a sequence.
        let value = (
            f64::INFINITY,
            Some(f64::NEG_INFINITY),
            vec![1.0_f64, f64::NAN],
        );
        assert_eq!(count_non_finite_floats(&value).unwrap(), 3);
    }

    #[test]
    fn an_infinite_option_really_does_come_back_as_none() {
        // The reason the scan exists: `serde_json` loses this one silently
        // rather than refusing it.
        let encoded = serde_json::to_string(&Some(f64::INFINITY)).unwrap();
        let decoded: Option<f64> = serde_json::from_str(&encoded).unwrap();
        assert_eq!(encoded, "null");
        assert_eq!(decoded, None);
    }

    #[test]
    fn finite_floats_survive_the_encoding_the_cache_uses() {
        // `serde_json`'s default float parser is *not* correctly rounded, and
        // the shipped models are full of constants it gets wrong by one unit
        // in the last place — `1.3806505e-23` came back as
        // `1.3806504999999999e-23` and moved the canonical IR's own HIR
        // digest. The crate's `float_roundtrip` dev-dependency feature is what
        // makes this hold; without it these assertions fail.
        for value in [
            1.380_650_5e-23_f64,
            6.25e41,
            6.188e40,
            1.602_176_634e-19,
            f64::MIN_POSITIVE,
            f64::MAX,
            -0.0,
        ] {
            let encoded = serde_json::to_string(&value).unwrap();
            let decoded: f64 = serde_json::from_str(&encoded).unwrap();
            assert_eq!(
                decoded.to_bits(),
                value.to_bits(),
                "{value:e} did not survive {encoded}"
            );
        }
    }
}

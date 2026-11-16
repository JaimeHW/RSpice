//! Native images that outlive the process that compiled them.
//!
//! Compiling a large compact model to native code takes seconds, and every
//! process that simulated it used to pay for that again. A process may install
//! a [`NativeImageStore`]: each image compiled here is then offered to it, and
//! the next process asks it before compiling. A stored image is verified like
//! freshly emitted code before it runs, and a refused one is compiled afresh.

use super::abi::call_helper_addresses;
use super::cache::{NativeCompileCacheKey, NativeCompileRole};
use super::model::{CodeOffset, NativeCurrentDependencies, NativeEntryOffsets};
use super::model_plan::NativeAssignmentCoverage;
use super::{JitResult, NativeModel};
use crate::codegen::CompiledModel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

/// The layout version of [`StoredNativeImage`]. An image of any other version
/// is refused and compiled afresh, and the version is part of every
/// [`NativeImageKey`], so two versions never share a file.
const STORED_IMAGE_FORMAT: u32 = 2;

/// A compiled native image another process can install.
///
/// It is only ever read back by the build that wrote it: the caller files it
/// under that build's identity.
#[derive(Serialize, Deserialize)]
pub struct StoredNativeImage {
    format: u32,
    machine: StoredMachineImage,
}

/// The machine an image was compiled for, and the image.
#[derive(Serialize, Deserialize)]
pub(crate) enum StoredMachineImage {
    X64(super::x64::StoredX64Image),
    AArch64(super::aarch64::stored::StoredA64Image),
}

impl StoredNativeImage {
    pub(crate) fn new(machine: StoredMachineImage) -> Self {
        Self {
            format: STORED_IMAGE_FORMAT,
            machine,
        }
    }

    /// The image, or why this build cannot read it.
    pub(crate) fn into_machine(self) -> Result<StoredMachineImage, String> {
        if self.format != STORED_IMAGE_FORMAT {
            return Err(format!(
                "format {} where this build reads {STORED_IMAGE_FORMAT}",
                self.format
            ));
        }
        Ok(self.machine)
    }
}

/// What an image needs besides its bytes to become a [`NativeModel`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StoredImageLayout {
    pub(crate) entries: NativeEntryOffsets,
    pub(crate) entry_starts: Vec<CodeOffset>,
    pub(crate) current_dependencies: NativeCurrentDependencies,
    pub(crate) prelude_slots: usize,
    pub(crate) assignment_coverage: NativeAssignmentCoverage,
    pub(crate) one_step_dae_split_safe: bool,
}

/// An emitted and verified image that is not executable yet, in the form a
/// process installs and the form another process can install.
pub(crate) trait StorableImage: Sized {
    fn install(self, model: &CompiledModel) -> JitResult<NativeModel>;

    /// The image with every helper address replaced by the helper's position,
    /// or `None` when it calls something no other process could find.
    fn to_stored(&self) -> Option<StoredNativeImage>;

    /// Verify a stored image as freshly emitted code is verified, then give
    /// each helper call this process's address for that helper.
    fn from_stored(stored: StoredNativeImage, model: &CompiledModel) -> JitResult<Self>;
}

/// Every function compiled code may call, by address and by position.
pub(crate) struct CallHelpers {
    addresses: &'static [usize],
    positions: HashMap<usize, usize>,
}

impl CallHelpers {
    pub(crate) fn get() -> &'static Self {
        static HELPERS: OnceLock<CallHelpers> = OnceLock::new();
        HELPERS.get_or_init(|| {
            let addresses = call_helper_addresses();
            let mut positions = HashMap::with_capacity(addresses.len());
            for (position, &address) in addresses.iter().enumerate() {
                // A linker may fold helpers with identical code into one;
                // either position then names the same code.
                positions.entry(address).or_insert(position);
            }
            Self {
                addresses,
                positions,
            }
        })
    }

    pub(crate) fn position(&self, address: usize) -> Option<usize> {
        self.positions.get(&address).copied()
    }

    pub(crate) fn address(&self, position: usize) -> Option<usize> {
        self.addresses.get(position).copied()
    }
}

/// An image as one byte string rather than a sequence of numbers.
pub(crate) mod image_bytes {
    use serde::de::{Error, SeqAccess, Visitor};
    use serde::{Deserializer, Serializer};

    pub(crate) fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(bytes)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<u8>, D::Error> {
        struct Bytes;

        impl<'de> Visitor<'de> for Bytes {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an image byte string")
            }

            fn visit_bytes<E: Error>(self, bytes: &[u8]) -> Result<Vec<u8>, E> {
                Ok(bytes.to_vec())
            }

            fn visit_byte_buf<E: Error>(self, bytes: Vec<u8>) -> Result<Vec<u8>, E> {
                Ok(bytes)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut items: A) -> Result<Vec<u8>, A::Error> {
                let mut bytes = Vec::with_capacity(items.size_hint().unwrap_or(0));
                while let Some(byte) = items.next_element()? {
                    bytes.push(byte);
                }
                Ok(bytes)
            }
        }

        deserializer.deserialize_byte_buf(Bytes)
    }
}

/// The content identity of a stored image: what was compiled, and for what.
///
/// It says nothing about the compiler build. A store keeps the images of
/// different builds apart itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NativeImageKey([u8; 32]);

impl NativeImageKey {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// `None` for the bytecode contract lane, whose images are tied to one
    /// model allocation and have no content identity to file them under.
    fn for_compilation(key: &NativeCompileCacheKey) -> Option<Self> {
        match key {
            #[cfg(feature = "native-bytecode-contract-tests")]
            NativeCompileCacheKey::Bytecode { .. } => None,
            NativeCompileCacheKey::CanonicalMir {
                role,
                mir_digest,
                source_digest,
                module,
                layout,
            } => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"rspice native image\0");
                hasher.update(&STORED_IMAGE_FORMAT.to_le_bytes());
                hasher.update(std::env::consts::ARCH.as_bytes());
                hasher.update(b"\0");
                hasher.update(&[match role {
                    NativeCompileRole::Evaluation => 0,
                    NativeCompileRole::Observation => 1,
                }]);
                for part in [mir_digest, source_digest, module] {
                    hasher.update(&(part.len() as u64).to_le_bytes());
                    hasher.update(part.as_bytes());
                }
                hasher.update(layout.as_bytes());
                Some(Self(*hasher.finalize().as_bytes()))
            }
        }
    }
}

/// Where native images are kept between processes.
///
/// A store must hand an image back only to the build of this library that
/// stored it; nothing in the image says which build that was.
pub trait NativeImageStore: Send + Sync {
    fn load(&self, key: &NativeImageKey) -> Option<StoredNativeImage>;
    fn store(&self, key: &NativeImageKey, image: &StoredNativeImage);
}

static STORE: OnceLock<&'static dyn NativeImageStore> = OnceLock::new();

/// Install this process's image store. The first one installed stays.
pub fn install_native_image_store(store: &'static dyn NativeImageStore) {
    let _ = STORE.set(store);
}

/// The process's image store, if one is installed.
pub(crate) fn installed_store() -> Option<&'static dyn NativeImageStore> {
    STORE.get().copied()
}

/// Install `key`'s stored image, or emit it and offer it to the store.
pub(crate) fn install_through_store<I: StorableImage>(
    store: Option<&dyn NativeImageStore>,
    key: &NativeCompileCacheKey,
    model: &CompiledModel,
    emit: impl FnOnce() -> JitResult<I>,
) -> JitResult<NativeModel> {
    let Some((store, key)) = store.zip(NativeImageKey::for_compilation(key)) else {
        return emit()?.install(model);
    };
    if let Some(stored) = store.load(&key) {
        match I::from_stored(stored, model).and_then(|image| image.install(model)) {
            Ok(native) => {
                log::debug!(
                    "[JIT] Model '{}' installed from a stored image, its helper calls bound to \
                     this process's helpers (the first at {:#x})",
                    model.name,
                    CallHelpers::get().address(0).unwrap_or_default()
                );
                return Ok(native);
            }
            Err(error) => log::warn!(
                "[JIT] Stored native image for '{}' refused, compiling it again: {error}",
                model.name
            ),
        }
    }
    let image = emit()?;
    let stored = image.to_stored();
    let native = image.install(model)?;
    match stored {
        Some(stored) => store.store(&key, &stored),
        None => log::debug!(
            "[JIT] Native image for '{}' calls code another process cannot locate; not stored",
            model.name
        ),
    }
    Ok(native)
}

#[cfg(all(test, any(target_arch = "x86_64", target_arch = "aarch64")))]
mod tests {
    use super::{NativeImageKey, NativeImageStore, StoredNativeImage, install_through_store};
    use crate::canonical_ir::CanonicalIrArtifact;
    use crate::codegen::CompiledModel;
    use crate::native::JitResult;
    use crate::native::cache::{NativeCompileCacheKey, NativeCompileRole};
    use crate::{CompilerOptions, VerilogACompiler};
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[cfg(target_arch = "x86_64")]
    type Image = crate::native::x64::X64Image;
    #[cfg(target_arch = "aarch64")]
    type Image = crate::native::aarch64::stored::A64Image;

    #[cfg(target_arch = "x86_64")]
    fn emit(model: &CompiledModel, artifact: &CanonicalIrArtifact) -> JitResult<Image> {
        crate::native::x64::emit_model_with_canonical_ir(model, artifact)
    }

    #[cfg(target_arch = "aarch64")]
    fn emit(model: &CompiledModel, artifact: &CanonicalIrArtifact) -> JitResult<Image> {
        let plan = crate::jit::cfg_plan_builder::build_default_model_plan(model, artifact)?;
        crate::native::aarch64::emit_model_plan(model, &plan)
    }

    const SOURCE: &str = r#"
module stored_image(p, n);
  inout p, n;
  electrical p, n;
  parameter real is = 1e-14;
  analog begin
    I(p, n) <+ is * (limexp(V(p, n) / 0.025) - 1) + sin(V(p, n));
  end
endmodule
"#;

    fn compiled() -> (CompiledModel, CanonicalIrArtifact) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(SOURCE).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(SOURCE)
            .expect("compile canonical IR");
        (model, artifact)
    }

    fn key(
        role: NativeCompileRole,
        model: &CompiledModel,
        artifact: &CanonicalIrArtifact,
    ) -> NativeCompileCacheKey {
        NativeCompileCacheKey::CanonicalMir {
            role,
            mir_digest: artifact.mir_digest.clone(),
            source_digest: model.source_digest.clone(),
            module: model.name.clone(),
            layout: blake3::hash(b"layout"),
        }
    }

    #[derive(Default)]
    struct MemoryStore {
        images: Mutex<HashMap<NativeImageKey, StoredNativeImage>>,
        loads: Mutex<usize>,
    }

    impl NativeImageStore for MemoryStore {
        fn load(&self, key: &NativeImageKey) -> Option<StoredNativeImage> {
            *self.loads.lock().unwrap() += 1;
            let images = self.images.lock().unwrap();
            let stored = images.get(key)?;
            // A real store hands back a copy decoded from its own encoding.
            let encoded = serde_json::to_vec(stored).expect("encode stored image");
            Some(serde_json::from_slice(&encoded).expect("decode stored image"))
        }

        fn store(&self, key: &NativeImageKey, image: &StoredNativeImage) {
            let encoded = serde_json::to_vec(image).expect("encode stored image");
            let decoded = serde_json::from_slice(&encoded).expect("decode stored image");
            self.images.lock().unwrap().insert(*key, decoded);
        }
    }

    /// The second process installs the first one's image without compiling,
    /// and an image under another key is never handed to it.
    #[test]
    fn a_stored_image_is_installed_instead_of_compiled() {
        let (model, artifact) = compiled();
        let key = |role| key(role, &model, &artifact);
        let store = MemoryStore::default();

        let compiled = install_through_store(
            Some(&store),
            &key(NativeCompileRole::Evaluation),
            &model,
            || emit(&model, &artifact),
        )
        .expect("compile and store");
        assert_eq!(store.images.lock().unwrap().len(), 1);

        let installed = install_through_store(
            Some(&store),
            &key(NativeCompileRole::Evaluation),
            &model,
            || -> JitResult<Image> { panic!("a stored image must not be compiled again") },
        )
        .expect("install the stored image");
        assert_eq!(installed.plan_stats(), compiled.plan_stats());
        assert_eq!(installed.code_size_bytes(), compiled.code_size_bytes());
        assert_eq!(*store.loads.lock().unwrap(), 2);

        let mut emitted = false;
        install_through_store(
            Some(&store),
            &key(NativeCompileRole::Observation),
            &model,
            || {
                emitted = true;
                emit(&model, &artifact)
            },
        )
        .expect("compile under a second key");
        assert!(emitted, "another role's image is another image");
        assert_eq!(store.images.lock().unwrap().len(), 2);
    }

    /// A stored image that fails verification is never run: the image is
    /// compiled again and the store is handed the good one.
    #[test]
    fn a_damaged_stored_image_is_compiled_again_and_replaced() {
        let (model, artifact) = compiled();
        let key = key(NativeCompileRole::Evaluation, &model, &artifact);
        let store = MemoryStore::default();
        install_through_store(Some(&store), &key, &model, || emit(&model, &artifact))
            .expect("compile and store");

        let image_key = NativeImageKey::for_compilation(&key).expect("a content key");
        let mut images = store.images.lock().unwrap();
        let good = serde_json::to_value(&images[&image_key]).expect("encode image");
        let mut damaged = good.clone();
        let machine = damaged["machine"]
            .as_object_mut()
            .and_then(|machine| machine.values_mut().next())
            .expect("one machine image");
        let bytes = machine["bytes"].as_array_mut().expect("image bytes");
        let len = bytes.len();
        bytes.truncate(len - 4);
        images.insert(
            image_key,
            serde_json::from_value(damaged).expect("decode damaged image"),
        );
        drop(images);

        let mut emitted = false;
        install_through_store(Some(&store), &key, &model, || {
            emitted = true;
            emit(&model, &artifact)
        })
        .expect("compile the damaged image again");
        assert!(emitted, "a damaged image is compiled again, not run");
        let replaced =
            serde_json::to_value(&store.images.lock().unwrap()[&image_key]).expect("encode");
        assert_eq!(replaced, good, "the store holds the good image again");
    }
}

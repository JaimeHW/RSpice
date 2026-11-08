//! Native images that outlive the process that compiled them.
//!
//! Compiling a large compact model to native code takes seconds, and every
//! process that simulated it used to pay for that again. A process may install
//! a [`NativeImageStore`]: each image compiled here is then offered to it, and
//! the next process asks it before compiling. A stored image is verified like
//! freshly emitted code before it runs, and a refused one is compiled afresh.

use super::cache::{NativeCompileCacheKey, NativeCompileRole};
use super::x64::{StoredNativeImage, X64Image};
use super::{JitResult, NativeModel};
use crate::codegen::CompiledModel;
use std::sync::OnceLock;

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
pub(crate) fn install_through_store(
    store: Option<&dyn NativeImageStore>,
    key: &NativeCompileCacheKey,
    model: &CompiledModel,
    emit: impl FnOnce() -> JitResult<X64Image>,
) -> JitResult<NativeModel> {
    let Some((store, key)) = store.zip(NativeImageKey::for_compilation(key)) else {
        return emit()?.install(model);
    };
    if let Some(stored) = store.load(&key) {
        match X64Image::from_stored(stored, model).and_then(|image| image.install(model)) {
            Ok(native) => {
                log::debug!("[JIT] Model '{}' installed from a stored image", model.name);
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

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use super::{NativeImageKey, NativeImageStore, install_through_store};
    use crate::native::cache::{NativeCompileCacheKey, NativeCompileRole};
    use crate::native::x64::{StoredNativeImage, emit_model_with_canonical_ir};
    use crate::{CompilerOptions, VerilogACompiler};
    use std::collections::HashMap;
    use std::sync::Mutex;

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
        let source = r#"
module stored_image(p, n);
  inout p, n;
  electrical p, n;
  parameter real is = 1e-14;
  analog begin
    I(p, n) <+ is * (limexp(V(p, n) / 0.025) - 1) + sin(V(p, n));
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let key = |role| NativeCompileCacheKey::CanonicalMir {
            role,
            mir_digest: artifact.mir_digest.clone(),
            source_digest: model.source_digest.clone(),
            module: model.name.clone(),
            layout: blake3::hash(b"layout"),
        };
        let store = MemoryStore::default();

        let compiled = install_through_store(
            Some(&store),
            &key(NativeCompileRole::Evaluation),
            &model,
            || emit_model_with_canonical_ir(&model, &artifact),
        )
        .expect("compile and store");
        assert_eq!(store.images.lock().unwrap().len(), 1);

        let installed = install_through_store(
            Some(&store),
            &key(NativeCompileRole::Evaluation),
            &model,
            || panic!("a stored image must not be compiled again"),
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
                emit_model_with_canonical_ir(&model, &artifact)
            },
        )
        .expect("compile under a second key");
        assert!(emitted, "another role's image is another image");
        assert_eq!(store.images.lock().unwrap().len(), 2);
    }
}

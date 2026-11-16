//! An emitted AArch64 image between code generation and execution, and the
//! form of it that another process can make executable.
//!
//! The machine code is position independent except for one thing: a helper
//! call loads the helper's absolute address into X16 with one MOVZ and three
//! MOVKs and calls the register. Those addresses move with every process, so
//! a stored image carries each helper's position in
//! [`call_helper_addresses`](crate::native::abi) in the same four immediates
//! instead, and the process that installs it writes its own addresses back.
//! Which words those are is never recorded: the verifier finds them, in the
//! image that was stored and again in the image that is loaded.

use crate::native::stored::{StoredImageLayout, image_bytes};
use serde::{Deserialize, Serialize};

/// An AArch64 image another process can install.
#[derive(Serialize, Deserialize)]
pub(crate) struct StoredA64Image {
    /// The image with each helper address replaced by its position.
    #[serde(with = "image_bytes")]
    bytes: Vec<u8>,
    /// Each function's byte range, in image order.
    functions: Vec<(u32, u32)>,
    layout: StoredImageLayout,
}

#[cfg(target_arch = "aarch64")]
pub(crate) use emitted::A64Image;

#[cfg(target_arch = "aarch64")]
mod emitted {
    use super::super::image::{A64FinishedImage, A64ImageBuilder};
    #[cfg(windows)]
    use super::super::unwind;
    use super::super::unwind::A64UnwindFunction;
    use super::super::verifier::{helper_call_address, set_helper_call_address};
    use super::StoredA64Image;
    use crate::codegen::CompiledModel;
    use crate::native::model::{NativeEntryStarts, NativeModel, NativeRequiredStorage};
    use crate::native::runtime::ExecutableMemory;
    use crate::native::stored::{
        CallHelpers, StorableImage, StoredImageLayout, StoredMachineImage, StoredNativeImage,
    };
    use crate::native::{JitError, JitResult};

    /// An emitted and verified AArch64 image that is not executable yet.
    pub(crate) struct A64Image {
        bytes: Vec<u8>,
        functions: Vec<(u32, u32)>,
        unwind_functions: Vec<A64UnwindFunction>,
        layout: StoredImageLayout,
        helper_address_sites: Vec<usize>,
        unrelocatable_calls: usize,
    }

    impl A64Image {
        /// `layout` without its entry starts, which are the finished image's.
        pub(crate) fn new(finished: A64FinishedImage, mut layout: StoredImageLayout) -> Self {
            layout.entry_starts = finished.entry_starts;
            Self {
                bytes: finished.bytes,
                functions: finished.functions,
                unwind_functions: finished.unwind_functions,
                layout,
                helper_address_sites: finished.helper_address_sites,
                unrelocatable_calls: finished.unrelocatable_calls,
            }
        }
    }

    impl StorableImage for A64Image {
        fn install(self, model: &CompiledModel) -> JitResult<NativeModel> {
            #[cfg(windows)]
            let executable = {
                let mut bytes = self.bytes;
                let runtime_functions =
                    unwind::append_windows_unwind_data(&mut bytes, &self.unwind_functions)?;
                ExecutableMemory::allocate_with_aarch64_unwind(&bytes, &runtime_functions)?
            };
            #[cfg(unix)]
            let executable = ExecutableMemory::allocate_with_aarch64_unwind(
                &self.bytes,
                &self.unwind_functions,
            )?;
            #[cfg(not(any(unix, windows)))]
            let executable = ExecutableMemory::allocate(&self.bytes)?;
            let layout = self.layout;
            NativeModel::from_executable_image_with_dependencies(
                model.num_terminals,
                model.internal_nodes,
                model.num_variables,
                model.parameters.len(),
                model.branch_sources.len(),
                model.stamp_programs.len(),
                executable,
                layout.entries,
                NativeEntryStarts::new(layout.entry_starts),
                layout.current_dependencies,
                NativeRequiredStorage::for_model(model).with_prelude_slots(layout.prelude_slots),
                layout.assignment_coverage,
            )
            .map(|native| native.with_one_step_dae_split_safe(layout.one_step_dae_split_safe))
        }

        fn to_stored(&self) -> Option<StoredNativeImage> {
            if self.unrelocatable_calls != 0 {
                return None;
            }
            let helpers = CallHelpers::get();
            let mut bytes = self.bytes.clone();
            for &site in &self.helper_address_sites {
                let address = usize::try_from(helper_call_address(&bytes, site)?).ok()?;
                let position = helpers.position(address)?;
                set_helper_call_address(&mut bytes, site, position as u64)?;
            }
            Some(StoredNativeImage::new(StoredMachineImage::AArch64(
                StoredA64Image {
                    bytes,
                    functions: self.functions.clone(),
                    layout: self.layout.clone(),
                },
            )))
        }

        fn from_stored(stored: StoredNativeImage, model: &CompiledModel) -> JitResult<Self> {
            let refused = |detail: String| JitError::Verifier {
                model: model.name.clone(),
                detail: format!("stored native image: {detail}").into(),
            };
            let StoredA64Image {
                bytes,
                functions,
                layout,
            } = match stored.into_machine().map_err(refused)? {
                StoredMachineImage::AArch64(image) => image,
                StoredMachineImage::X64(_) => {
                    return Err(refused("an x64 image on an AArch64 host".into()));
                }
            };
            let stored_starts = layout.entry_starts.clone();
            let mut finished =
                A64ImageBuilder::from_published(bytes, &functions)?.finish_image()?;
            if finished.entry_starts != stored_starts {
                return Err(refused(
                    "its entry starts are not its functions' starts".into(),
                ));
            }
            if finished.unrelocatable_calls != 0 {
                return Err(refused(format!(
                    "{} call(s) through a register no helper load names",
                    finished.unrelocatable_calls
                )));
            }
            let helpers = CallHelpers::get();
            for &site in &finished.helper_address_sites {
                let position = helper_call_address(&finished.bytes, site)
                    .ok_or_else(|| refused(format!("truncated helper call at byte {site}")))?;
                let address = usize::try_from(position)
                    .ok()
                    .and_then(|position| helpers.address(position))
                    .ok_or_else(|| {
                        refused(format!("call at byte {site} names helper {position}"))
                    })?;
                set_helper_call_address(&mut finished.bytes, site, address as u64)
                    .ok_or_else(|| refused(format!("truncated helper call at byte {site}")))?;
            }
            Ok(Self::new(finished, layout))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::super::verifier::{
            HELPER_CALL_BLR_OFFSET, helper_call_address, set_helper_call_address,
        };
        use super::A64Image;
        use crate::codegen::CompiledModel;
        use crate::native::JitResult;
        use crate::native::aarch64::stored::StoredA64Image;
        use crate::native::abi::CALL_HELPER_NAMES;
        use crate::native::stored::{StorableImage, StoredMachineImage, StoredNativeImage};
        use crate::{CompilerOptions, VerilogACompiler};

        fn emitted() -> (CompiledModel, A64Image) {
            let source = r#"
module stored_a64_image(p, n);
  inout p, n;
  electrical p, n;
  parameter real is = 1e-14;
  analog begin
    I(p, n) <+ is * (limexp(V(p, n) / 0.025) - 1) + sin(V(p, n)) + exp(-V(p, n));
  end
endmodule
"#;
            let compiler = VerilogACompiler::new(CompilerOptions::default());
            let model = compiler.compile(source).expect("compile bytecode model");
            let artifact = compiler
                .compile_canonical_ir(source)
                .expect("compile canonical IR");
            let plan = crate::jit::cfg_plan_builder::build_default_model_plan(&model, &artifact)
                .expect("plan the model");
            let image =
                crate::native::aarch64::emit_model_plan(&model, &plan).expect("emit native image");
            (model, image)
        }

        fn stored(image: &A64Image) -> StoredA64Image {
            match image
                .to_stored()
                .expect("every helper is listed")
                .into_machine()
                .expect("this build's format")
            {
                StoredMachineImage::AArch64(stored) => stored,
                StoredMachineImage::X64(_) => panic!("an AArch64 image is stored as one"),
            }
        }

        fn restore(stored: StoredA64Image, model: &CompiledModel) -> JitResult<A64Image> {
            A64Image::from_stored(
                StoredNativeImage::new(StoredMachineImage::AArch64(stored)),
                model,
            )
        }

        /// Stored and restored in one process, an image comes back byte for
        /// byte, and what was stored holds no address of this process.
        #[test]
        fn a_stored_image_restores_to_the_image_that_was_emitted() {
            let (model, image) = emitted();
            assert!(
                image.helper_address_sites.len() >= 3,
                "the fixture calls helpers: {:?}",
                image.helper_address_sites
            );
            assert_eq!(image.unrelocatable_calls, 0);
            let stored = stored(&image);
            for &site in &image.helper_address_sites {
                let position = helper_call_address(&stored.bytes, site).expect("a helper call");
                assert!(position < CALL_HELPER_NAMES.len() as u64);
            }

            let restored = restore(stored, &model).expect("restore the stored image");
            assert_eq!(restored.bytes, image.bytes);
            assert_eq!(restored.functions, image.functions);
            assert_eq!(restored.helper_address_sites, image.helper_address_sites);
            assert_eq!(restored.layout.entries, image.layout.entries);
            assert_eq!(restored.layout.entry_starts, image.layout.entry_starts);
            restored
                .install(&model)
                .expect("install the restored image");
        }

        #[test]
        fn a_stored_image_that_names_an_unknown_helper_or_holds_damaged_code_is_refused() {
            let (model, image) = emitted();
            let site = image.helper_address_sites[0];
            let refused = |stored: StoredA64Image| {
                restore(stored, &model)
                    .err()
                    .expect("a damaged image is refused")
                    .to_string()
            };

            let mut damaged = stored(&image);
            set_helper_call_address(&mut damaged.bytes, site, CALL_HELPER_NAMES.len() as u64)
                .expect("a helper call");
            let error = refused(damaged);
            assert!(error.contains("names helper"), "{error}");

            // The helper call becomes `SVC #0`, which no image holds.
            let mut damaged = stored(&image);
            let call = site + HELPER_CALL_BLR_OFFSET;
            damaged.bytes[call..call + 4].copy_from_slice(&0xD400_0001_u32.to_le_bytes());
            refused(damaged);

            // A call through X17, which no helper load names.
            let mut damaged = stored(&image);
            damaged.bytes[call..call + 4].copy_from_slice(&0xD63F_0220_u32.to_le_bytes());
            refused(damaged);

            let mut damaged = stored(&image);
            let len = damaged.bytes.len();
            damaged.bytes.truncate(len - 4);
            refused(damaged);

            let mut damaged = stored(&image);
            damaged
                .bytes
                .extend_from_slice(&0xD503_201F_u32.to_le_bytes());
            let error = refused(damaged);
            assert!(error.contains("past its last function"), "{error}");

            let mut damaged = stored(&image);
            damaged.layout.entry_starts.pop();
            let error = refused(damaged);
            assert!(error.contains("entry starts"), "{error}");

            let mut damaged = stored(&image);
            assert!(
                damaged.functions.len() >= 2,
                "the fixture has several functions"
            );
            damaged.functions[1].0 = damaged.functions[0].0;
            refused(damaged);
        }

        /// An image of another format, or of another machine, is refused
        /// before any of its code is read.
        #[test]
        fn a_stored_image_of_another_format_or_machine_is_refused() {
            let (model, image) = emitted();
            let encoded =
                serde_json::to_value(image.to_stored().expect("store")).expect("encode image");

            let mut other_format = encoded.clone();
            other_format["format"] = serde_json::json!(1);
            let other_format: StoredNativeImage =
                serde_json::from_value(other_format).expect("decode image");
            let error = A64Image::from_stored(other_format, &model)
                .err()
                .expect("another format is refused")
                .to_string();
            assert!(error.contains("format 1"), "{error}");

            let a64 = &encoded["machine"]["AArch64"];
            let mut x64 = encoded.clone();
            x64["machine"] = serde_json::json!({ "X64": {
                "bytes": a64["bytes"],
                "code_len": a64["bytes"].as_array().expect("image bytes").len(),
                "runtime_functions": [],
                "layout": a64["layout"],
            }});
            let x64: StoredNativeImage = serde_json::from_value(x64).expect("decode x64 image");
            let error = A64Image::from_stored(x64, &model)
                .err()
                .expect("an x64 image is refused")
                .to_string();
            assert!(error.contains("x64 image"), "{error}");
        }
    }
}

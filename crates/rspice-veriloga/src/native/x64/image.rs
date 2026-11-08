//! An emitted x64 image between code generation and execution, and the form
//! of it that another process can make executable.
//!
//! The machine code is position independent except for one thing: a helper
//! call loads the helper's absolute address with `movabs reg, imm64` and calls
//! the register. Those addresses move with every process, so a stored image
//! names each helper by its position in [`call_helper_addresses`] instead, and
//! the process that installs it writes its own addresses back. Which bytes are
//! such an address is never recorded; the decoder that verifies an image finds
//! them, in the image that was stored and again in the image that is loaded.

use super::super::abi::call_helper_addresses;
use super::super::model::{
    CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeEntryStarts, NativeModel,
    NativeRequiredStorage,
};
use super::super::model_plan::NativeAssignmentCoverage;
use super::super::runtime::ExecutableMemory;
#[cfg(all(windows, target_arch = "x86_64"))]
use super::super::runtime::WindowsX64RuntimeFunction;
use super::verify_x64_image_layout;
use crate::codegen::CompiledModel;
use crate::native::{JitError, JitResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

const ADDRESS_BYTES: usize = 8;

/// The indirect calls of a verified image.
#[derive(Debug)]
pub(super) struct X64ImageCalls {
    /// Image offsets of the absolute address each helper call loads.
    pub(super) absolute: Vec<usize>,
    /// Indirect calls through anything else.
    pub(super) other: usize,
}

/// One Windows x64 `RUNTIME_FUNCTION`, as offsets from the image base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct X64RuntimeFunction {
    pub(super) begin_address: u32,
    pub(super) end_address: u32,
    pub(super) unwind_info_address: u32,
}

/// What an image needs besides its bytes to become a [`NativeModel`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct X64ImageLayout {
    pub(super) entries: NativeEntryOffsets,
    pub(super) entry_starts: Vec<CodeOffset>,
    pub(super) current_dependencies: NativeCurrentDependencies,
    pub(super) prelude_slots: usize,
    pub(super) assignment_coverage: NativeAssignmentCoverage,
    pub(super) one_step_dae_split_safe: bool,
}

/// An emitted and verified image that is not executable yet.
pub(crate) struct X64Image {
    /// The code, followed on Windows by its unwind data.
    bytes: Vec<u8>,
    code_len: usize,
    runtime_functions: Vec<X64RuntimeFunction>,
    layout: X64ImageLayout,
    calls: X64ImageCalls,
}

/// A compiled native image another process can install.
///
/// It is only ever read back by the build that wrote it: the caller files it
/// under that build's identity.
#[derive(Serialize, Deserialize)]
pub struct StoredNativeImage {
    /// [`X64Image::bytes`] with each helper address replaced by its position.
    #[serde(with = "image_bytes")]
    bytes: Vec<u8>,
    code_len: usize,
    runtime_functions: Vec<X64RuntimeFunction>,
    layout: X64ImageLayout,
}

impl X64Image {
    pub(super) fn new(
        bytes: Vec<u8>,
        code_len: usize,
        runtime_functions: Vec<X64RuntimeFunction>,
        layout: X64ImageLayout,
        calls: X64ImageCalls,
    ) -> Self {
        Self {
            bytes,
            code_len,
            runtime_functions,
            layout,
            calls,
        }
    }

    pub(crate) fn install(self, model: &CompiledModel) -> JitResult<NativeModel> {
        #[cfg(all(windows, target_arch = "x86_64"))]
        let executable = {
            let functions = self
                .runtime_functions
                .iter()
                .map(|function| WindowsX64RuntimeFunction {
                    begin_address: function.begin_address,
                    end_address: function.end_address,
                    unwind_info_address: function.unwind_info_address,
                })
                .collect::<Vec<_>>();
            ExecutableMemory::allocate_with_windows_unwind(&self.bytes, &functions)?
        };
        #[cfg(not(all(windows, target_arch = "x86_64")))]
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

    /// The image with every helper address replaced by the helper's position,
    /// or `None` when it calls something no other process could find.
    pub(crate) fn to_stored(&self) -> Option<StoredNativeImage> {
        if self.calls.other != 0 {
            return None;
        }
        let helpers = CallHelpers::get();
        let mut bytes = self.bytes.clone();
        for &offset in &self.calls.absolute {
            let slot = bytes.get_mut(offset..offset + ADDRESS_BYTES)?;
            let address = usize::try_from(u64::from_le_bytes(slot.try_into().ok()?)).ok()?;
            let position = helpers.position(address)?;
            slot.copy_from_slice(&(position as u64).to_le_bytes());
        }
        Some(StoredNativeImage {
            bytes,
            code_len: self.code_len,
            runtime_functions: self.runtime_functions.clone(),
            layout: self.layout.clone(),
        })
    }

    /// Verify a stored image as freshly emitted code is verified, then give
    /// each helper call this process's address for that helper.
    pub(crate) fn from_stored(stored: StoredNativeImage, model: &CompiledModel) -> JitResult<Self> {
        let StoredNativeImage {
            mut bytes,
            code_len,
            runtime_functions,
            layout,
        } = stored;
        let refused = |detail: String| JitError::Verifier {
            model: model.name.clone(),
            detail: format!("stored native image: {detail}").into(),
        };
        if code_len == 0 || code_len > bytes.len() {
            return Err(refused(format!(
                "code length {code_len} for an image of {} bytes",
                bytes.len()
            )));
        }
        validate_runtime_functions(&runtime_functions, code_len, bytes.len()).map_err(refused)?;

        let calls = verify_x64_image_layout(model, &bytes[..code_len], &layout.entry_starts)?;
        if calls.other != 0 {
            return Err(refused(format!(
                "{} indirect call(s) without a helper address",
                calls.other
            )));
        }
        let helpers = CallHelpers::get();
        for &offset in &calls.absolute {
            let slot = &mut bytes[offset..offset + ADDRESS_BYTES];
            let position = u64::from_le_bytes(slot.try_into().expect("an eight-byte slot"));
            let address = usize::try_from(position)
                .ok()
                .and_then(|position| helpers.address(position))
                .ok_or_else(|| refused(format!("call at byte {offset} names helper {position}")))?;
            slot.copy_from_slice(&(address as u64).to_le_bytes());
        }
        Ok(Self {
            bytes,
            code_len,
            runtime_functions,
            layout,
            calls,
        })
    }
}

/// The unwind table of a stored image must describe its own code, in order,
/// and point into its own unwind data, as `append_windows_x64_unwind_metadata`
/// writes it. Off Windows an image has none.
fn validate_runtime_functions(
    functions: &[X64RuntimeFunction],
    code_len: usize,
    image_len: usize,
) -> Result<(), String> {
    if !cfg!(all(windows, target_arch = "x86_64")) {
        return if functions.is_empty() && code_len == image_len {
            Ok(())
        } else {
            Err("unwind data on a host that registers none".into())
        };
    }
    let mut previous_end = 0;
    for (index, function) in functions.iter().enumerate() {
        let begin = function.begin_address as usize;
        let end = function.end_address as usize;
        let unwind = function.unwind_info_address as usize;
        if begin < previous_end
            || end <= begin
            || end > code_len
            || unwind < code_len
            || unwind >= image_len
            || unwind % 4 != 0
        {
            return Err(format!(
                "runtime function {index} spans {begin}..{end} with unwind data at {unwind} \
                 for {code_len} code bytes in {image_len}"
            ));
        }
        previous_end = end;
    }
    Ok(())
}

/// Every function compiled code may call, by address and by position.
struct CallHelpers {
    addresses: &'static [usize],
    positions: HashMap<usize, usize>,
}

impl CallHelpers {
    fn get() -> &'static Self {
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

    fn position(&self, address: usize) -> Option<usize> {
        self.positions.get(&address).copied()
    }

    fn address(&self, position: usize) -> Option<usize> {
        self.addresses.get(position).copied()
    }
}

/// The image as one byte string rather than a sequence of numbers.
mod image_bytes {
    use serde::de::{Error, SeqAccess, Visitor};
    use serde::{Deserializer, Serializer};

    pub(super) fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(bytes)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
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

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use super::{ADDRESS_BYTES, X64Image, X64RuntimeFunction};
    use crate::codegen::CompiledModel;
    use crate::native::abi::CALL_HELPER_NAMES;
    use crate::native::x64::emit_model_with_canonical_ir;
    use crate::{CompilerOptions, VerilogACompiler};

    fn emitted() -> (CompiledModel, X64Image) {
        let source = r#"
module stored_x64_image(p, n);
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
        let image = emit_model_with_canonical_ir(&model, &artifact).expect("emit native image");
        (model, image)
    }

    fn address_at(bytes: &[u8], offset: usize) -> u64 {
        u64::from_le_bytes(
            bytes[offset..offset + ADDRESS_BYTES]
                .try_into()
                .expect("an eight-byte slot"),
        )
    }

    /// Stored and restored in one process, an image comes back byte for
    /// byte, and what was stored holds no address of this process.
    #[test]
    fn a_stored_image_restores_to_the_image_that_was_emitted() {
        let (model, image) = emitted();
        assert!(
            image.calls.absolute.len() >= 3,
            "the fixture calls helpers: {:?}",
            image.calls
        );
        let stored = image.to_stored().expect("every helper is listed");
        for &offset in &image.calls.absolute {
            assert!(address_at(&stored.bytes, offset) < CALL_HELPER_NAMES.len() as u64);
        }

        let restored = X64Image::from_stored(stored, &model).expect("restore the stored image");
        assert_eq!(restored.bytes, image.bytes);
        assert_eq!(restored.code_len, image.code_len);
        assert_eq!(restored.runtime_functions, image.runtime_functions);
        assert_eq!(restored.layout.entries, image.layout.entries);
        assert_eq!(restored.calls.absolute, image.calls.absolute);
        restored
            .install(&model)
            .expect("install the restored image");
    }

    #[test]
    fn a_stored_image_that_names_an_unknown_helper_or_holds_damaged_code_is_refused() {
        let (model, image) = emitted();
        let offset = image.calls.absolute[0];

        let mut stored = image.to_stored().expect("store");
        stored.bytes[offset..offset + ADDRESS_BYTES].copy_from_slice(&u64::MAX.to_le_bytes());
        let error = X64Image::from_stored(stored, &model)
            .err()
            .expect("an unknown helper is refused");
        assert!(error.to_string().contains("names helper"), "{error}");

        // The call after the address becomes `ud2`, which no image holds.
        let mut stored = image.to_stored().expect("store");
        let call = offset + ADDRESS_BYTES;
        stored.bytes[call..call + 2].copy_from_slice(&[0x0F, 0x0B]);
        assert!(X64Image::from_stored(stored, &model).is_err());

        let mut stored = image.to_stored().expect("store");
        stored.code_len = stored.bytes.len() + 1;
        assert!(X64Image::from_stored(stored, &model).is_err());

        let mut stored = image.to_stored().expect("store");
        let misplaced = X64RuntimeFunction {
            begin_address: 0,
            end_address: 1,
            unwind_info_address: 1,
        };
        match stored.runtime_functions.first_mut() {
            Some(function) => function.unwind_info_address = 1,
            None => stored.runtime_functions.push(misplaced),
        }
        assert!(X64Image::from_stored(stored, &model).is_err());
    }

    /// A helper missing from the list cannot be named by a stored image, so
    /// an image that calls it would never be stored. The list names every
    /// helper the ABI defines, in the order it defines them.
    #[test]
    fn the_call_helper_list_names_every_helper_the_abi_defines() {
        let source = include_str!("../abi.rs");
        let tests = source
            .find("\nmod tests {")
            .expect("the ABI has a test module");
        let defined: Vec<&str> = source[..tests]
            .split("extern \"C\" fn ")
            .skip(1)
            .filter_map(|rest| rest.split_once('(').map(|(name, _)| name))
            .collect();
        assert_eq!(CALL_HELPER_NAMES, defined.as_slice());
    }
}

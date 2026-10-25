//! The catalog descriptors of a generated model, and the parameter and
//! stage tables its constructors check and install.

use super::Value;

/// Version of the immutable catalog contract emitted beside generated models.
///
/// This is intentionally independent of checkpoint and stamp-workspace
/// versions: consumers use it to decide whether a persisted schematic binding
/// can be reconstructed from the compiled model catalog.
pub const GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedVerilogAParameterScope {
    Model,
    Instance,
    /// Model-card storage supplies the default and an instance assignment may
    /// override it for one concrete device.
    Dual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedVerilogATerminalDirection {
    Input,
    Output,
    InOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedVerilogATerminalDescriptor {
    pub name: &'static str,
    pub direction: GeneratedVerilogATerminalDirection,
    pub discipline: &'static str,
    /// Canonical operating-point parameter for current entering this terminal.
    pub current_parameter: &'static str,
}

/// A static numeric range endpoint from a Verilog-A parameter declaration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedVerilogAParameterBound {
    pub value: Value,
    pub exclusive: bool,
}

impl GeneratedVerilogAParameterBound {
    #[inline]
    pub const fn inclusive(value: Value) -> Self {
        Self {
            value,
            exclusive: false,
        }
    }

    #[inline]
    pub const fn exclusive(value: Value) -> Self {
        Self {
            value,
            exclusive: true,
        }
    }
}

/// Immutable, engine-neutral parameter metadata emitted by the compiler.
///
/// `default` is `None` when the source default is an expression rather than a
/// source-level numeric literal. `has_dynamic_constraints` tells editors that
/// the generated model remains the final authority because at least one range
/// condition depends on another parameter or a compiled expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedVerilogAParameterDescriptor {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub scope: GeneratedVerilogAParameterScope,
    pub is_integer: bool,
    pub is_array: bool,
    pub default: Option<Value>,
    pub minimum: Option<GeneratedVerilogAParameterBound>,
    pub maximum: Option<GeneratedVerilogAParameterBound>,
    pub excluded_values: &'static [Value],
    pub has_dynamic_constraints: bool,
}

impl GeneratedVerilogAParameterDescriptor {
    const fn with_scope(
        name: &'static str,
        default: Option<Value>,
        scope: GeneratedVerilogAParameterScope,
    ) -> Self {
        Self {
            name,
            aliases: &[],
            scope,
            is_integer: false,
            is_array: false,
            default,
            minimum: None,
            maximum: None,
            excluded_values: &[],
            has_dynamic_constraints: false,
        }
    }

    #[inline]
    pub const fn model(name: &'static str, default: Option<Value>) -> Self {
        Self::with_scope(name, default, GeneratedVerilogAParameterScope::Model)
    }

    #[inline]
    pub const fn instance(name: &'static str, default: Option<Value>) -> Self {
        Self::with_scope(name, default, GeneratedVerilogAParameterScope::Instance)
    }

    #[inline]
    pub const fn dual(name: &'static str, default: Option<Value>) -> Self {
        Self::with_scope(name, default, GeneratedVerilogAParameterScope::Dual)
    }

    #[inline]
    pub const fn integer(mut self) -> Self {
        self.is_integer = true;
        self
    }

    /// Mark an unpacked parameter array, which has no scalar default.
    #[inline]
    pub const fn array(mut self) -> Self {
        self.is_array = true;
        self.default = None;
        self
    }

    #[inline]
    pub const fn aliases(mut self, aliases: &'static [&'static str]) -> Self {
        self.aliases = aliases;
        self
    }

    #[inline]
    pub const fn minimum(mut self, minimum: GeneratedVerilogAParameterBound) -> Self {
        self.minimum = Some(minimum);
        self
    }

    #[inline]
    pub const fn maximum(mut self, maximum: GeneratedVerilogAParameterBound) -> Self {
        self.maximum = Some(maximum);
        self
    }

    #[inline]
    pub const fn excluded_values(mut self, excluded_values: &'static [Value]) -> Self {
        self.excluded_values = excluded_values;
        self
    }

    #[inline]
    pub const fn dynamic_constraints(mut self) -> Self {
        self.has_dynamic_constraints = true;
        self
    }
}

/// Stable identity and exact external contract for one compiled Verilog-A
/// catalog entry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedVerilogAModelDescriptor {
    pub abi_version: u32,
    /// Case-preserving name accepted by the generated model registry.
    pub model_name: &'static str,
    /// Module name written in the Verilog-A source.
    pub module_name: &'static str,
    /// Digest of the preprocessed source closure used for generation.
    pub source_digest: &'static str,
    /// Full BLAKE3 identity of that exact preprocessed source closure.
    pub source_identity: &'static str,
    /// Digest covering the executable instance/checkpoint layout.
    pub checkpoint_identity: &'static str,
    /// Exact shape of accepted dynamic state persisted for this model.
    pub accepted_state_shape_identity: GeneratedVerilogAAcceptedStateShapeIdentity,
    /// External terminals in the exact positional order expected by netlists.
    pub terminals: &'static [GeneratedVerilogATerminalDescriptor],
    pub parameters: &'static [GeneratedVerilogAParameterDescriptor],
    pub total_node_count: usize,
    pub internal_node_names: &'static [&'static str],
    /// Sorted indices into `internal_node_names` occupied by mathematical states.
    /// Hosts must exclude these unknowns from electrical shunts and voltage clamps.
    pub internal_state_nodes: &'static [usize],
    pub branch_count: usize,
    /// Every matrix location and right-hand-side row the model's stamps can
    /// write, whatever its parameters and bias.
    pub stamp_pattern: GeneratedVerilogAStampPattern,
}

/// The matrix locations a generated model's stamp calls name, read from the
/// calls themselves.
///
/// Axes are device-local: nodes `0..total_node_count` in the model's own order,
/// then branch unknowns from `total_node_count`. A location is listed when
/// some stamp call can write it, which is a superset of what any one
/// evaluation writes; a location a call writes only with a zero value at a
/// given bias is still listed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedVerilogAStampPattern {
    /// `(row, column)` locations the conduction stamp can write at DC: those
    /// of every equation that is not a sum of `ddt()` terms, and the structural
    /// incidence of every source branch.
    pub resistive: &'static [(usize, usize)],
    /// `(row, column)` locations the conduction stamp writes only through a
    /// time derivative: those of the equations whose contribution is a sum of
    /// `ddt()` terms. They are zero at DC and carry the integration companion
    /// in a transient step.
    pub dynamic: &'static [(usize, usize)],
    /// `(row, column)` locations the small-signal reactive stamp can write.
    pub reactive: &'static [(usize, usize)],
    /// Rows the conduction stamp can write on the right-hand side.
    pub rhs_rows: &'static [usize],
}

impl GeneratedVerilogAStampPattern {
    /// No location at all, for a model whose stamps write nothing.
    pub const EMPTY: Self = Self {
        resistive: &[],
        dynamic: &[],
        reactive: &[],
        rhs_rows: &[],
    };
}

/// Typed BLAKE3 identity of a generated model's accepted-state layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GeneratedVerilogAAcceptedStateShapeIdentity([u8; 32]);

impl GeneratedVerilogAAcceptedStateShapeIdentity {
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn from_hex(value: &str) -> Result<Self, String> {
        if value.len() != 64 {
            return Err(
                "accepted-state shape identity must contain 64 lowercase hex digits".into(),
            );
        }
        let mut bytes = [0u8; 32];
        for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
            let nybble = |byte| match byte {
                b'0'..=b'9' => Some(byte - b'0'),
                b'a'..=b'f' => Some(byte - b'a' + 10),
                _ => None,
            };
            let high = nybble(pair[0]).ok_or_else(|| {
                "accepted-state shape identity must contain 64 lowercase hex digits".to_string()
            })?;
            let low = nybble(pair[1]).ok_or_else(|| {
                "accepted-state shape identity must contain 64 lowercase hex digits".to_string()
            })?;
            bytes[index] = high << 4 | low;
        }
        Ok(Self(bytes))
    }
}

impl std::fmt::Display for GeneratedVerilogAAcceptedStateShapeIdentity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// Provenance of a parameter assignment applied to a generated Verilog-A model.
///
/// CMC Verilog-A distinguishes model-card parameters from per-instance
/// parameters. Keeping that provenance through the engine boundary is also
/// required for `$param_given`: a value written through the wrong storage
/// class must be rejected rather than silently changing the corresponding
/// given flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedParameterOrigin {
    /// A model-authoring or standalone caller intentionally targets whatever
    /// storage class the Verilog-A declaration assigns to the parameter.
    DeclaredScope,
    /// The assignment came from a SPICE `.model` card.
    ModelCard,
    /// The assignment came from a concrete device instance.
    Instance,
}

/// One numeric parameter assignment supplied to a generated model instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedParameterAssignment<'a> {
    pub name: &'a str,
    pub value: Value,
    pub origin: GeneratedParameterOrigin,
}

impl<'a> GeneratedParameterAssignment<'a> {
    #[inline]
    pub const fn new(name: &'a str, value: Value, origin: GeneratedParameterOrigin) -> Self {
        Self {
            name,
            value,
            origin,
        }
    }

    #[inline]
    pub const fn for_declared_scope(name: &'a str, value: Value) -> Self {
        Self::new(name, value, GeneratedParameterOrigin::DeclaredScope)
    }
}

/// Installs one generated invalidation stage into its canonical destination
/// slots. The generator proves the mapping shape, while these checks keep a
/// stale or corrupt generated table from becoming unchecked memory access.
///
/// Kept out of line because installation is cold-path work and duplicating the
/// loop into every model would recreate the compile-time cost this table form
/// removes.
#[doc(hidden)]
#[inline(never)]
pub fn install_generated_stage_values(destination: &mut [f64], values: &[f64], slots: &[u32]) {
    assert_eq!(
        values.len(),
        slots.len(),
        "generated stage value/slot count mismatch"
    );
    for (&value, &slot) in values.iter().zip(slots) {
        let slot = usize::try_from(slot).expect("generated stage slot does not fit usize");
        let destination = destination
            .get_mut(slot)
            .expect("generated stage slot is outside canonical storage");
        *destination = value;
    }
}

/// Look up a lower-case generated parameter name in parallel sorted tables.
#[doc(hidden)]
pub fn find_generated_parameter_index(
    sorted_names: &[&str],
    parameter_indices: &[u16],
    name: &str,
) -> Option<usize> {
    assert_eq!(
        sorted_names.len(),
        parameter_indices.len(),
        "generated parameter lookup table length mismatch"
    );
    let mut left = 0usize;
    let mut right = sorted_names.len();
    while left < right {
        let middle = left + (right - left) / 2;
        if sorted_names[middle] < name {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    (sorted_names.get(left).copied() == Some(name)).then(|| usize::from(parameter_indices[left]))
}

/// One literal or referenced bound in generated parameter metadata.
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct GeneratedParameterBound {
    pub value: f64,
    pub label: &'static str,
}

/// Encoded generated parameter-bound index used for the absence sentinel.
///
/// Positive values are one-based indices into a generated bound pool. Keeping
/// the sentinel explicit avoids relying on an enum's undocumented layout.
#[doc(hidden)]
pub const GENERATED_PARAMETER_BOUND_NONE: u16 = 0;

#[doc(hidden)]
pub const GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
#[doc(hidden)]
pub const GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

#[doc(hidden)]
pub fn validate_generated_finite_parameter(name: &str, value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!(
            "parameter '{}' must be finite, got {}",
            name, value
        ));
    }
    Ok(())
}

#[doc(hidden)]
pub fn validate_generated_parameter_bounds(
    name: &str,
    value: f64,
    flags: u8,
    min: Option<GeneratedParameterBound>,
    max: Option<GeneratedParameterBound>,
    excluded: &[GeneratedParameterBound],
) -> Result<(), String> {
    if let Some(min) = min {
        if flags & GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!(
                    "parameter '{}' must be > {}, got {}",
                    name, min.label, value
                ));
            }
        } else if value < min.value {
            return Err(format!(
                "parameter '{}' must be >= {}, got {}",
                name, min.label, value
            ));
        }
    }
    if let Some(max) = max {
        if flags & GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!(
                    "parameter '{}' must be < {}, got {}",
                    name, max.label, value
                ));
            }
        } else if value > max.value {
            return Err(format!(
                "parameter '{}' must be <= {}, got {}",
                name, max.label, value
            ));
        }
    }
    for excluded in excluded {
        if value == excluded.value {
            return Err(format!(
                "parameter '{}' must not equal {}, got {}",
                name, excluded.label, value
            ));
        }
    }
    Ok(())
}

/// Resolve a compact, one-based generated parameter-bound index.
#[doc(hidden)]
#[inline]
pub fn resolve_generated_parameter_bound(
    pool: &[GeneratedParameterBound],
    encoded: u16,
) -> Option<GeneratedParameterBound> {
    if encoded == GENERATED_PARAMETER_BOUND_NONE {
        return None;
    }
    let index = usize::from(encoded - 1);
    Some(
        *pool
            .get(index)
            .expect("generated parameter-bound index is outside its pool"),
    )
}

/// Validate bounds stored as compact indices into a generated per-device pool.
#[doc(hidden)]
pub fn validate_generated_parameter_bound_indices(
    name: &str,
    value: f64,
    flags: u8,
    pool: &[GeneratedParameterBound],
    min: u16,
    max: u16,
    excluded: &[u16],
) -> Result<(), String> {
    validate_generated_parameter_bounds(
        name,
        value,
        flags,
        resolve_generated_parameter_bound(pool, min),
        resolve_generated_parameter_bound(pool, max),
        &[],
    )?;
    for &encoded in excluded {
        let excluded = resolve_generated_parameter_bound(pool, encoded)
            .expect("generated parameter exclusion uses the absence sentinel");
        if value == excluded.value {
            return Err(format!(
                "parameter '{}' must not equal {}, got {}",
                name, excluded.label, value
            ));
        }
    }
    Ok(())
}

/// Allocate a large zeroed generated state array directly on the heap.
#[doc(hidden)]
pub fn boxed_zero_f64_array<const N: usize>() -> Box<[f64; N]> {
    let mut boxed = Box::<[f64; N]>::new_uninit();
    // SAFETY: every bit pattern is written before the box is assumed
    // initialized, and all-zero bytes are valid `0.0` values.
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
}

/// Allocate a large false-filled generated state array directly on the heap.
#[doc(hidden)]
pub fn boxed_zero_bool_array<const N: usize>() -> Box<[bool; N]> {
    let mut boxed = Box::<[bool; N]>::new_uninit();
    // SAFETY: every bit pattern is written before the box is assumed
    // initialized, and all-zero bytes are valid `false` values.
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
}

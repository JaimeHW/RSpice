//! Stable, engine-neutral runtime ABI for precompiled Verilog-A models.
//!
//! Generated model crates call these concrete types directly.
//!
//! **This crate must not depend on `rspice-core`.** The dependency edges run
//! `rspice-core` -> `rspice-veriloga-models` -> the 42 generated model crates,
//! and each of those leaves depends on exactly this crate for its ABI. Moving
//! these types into `rspice-core`, or adding a `rspice-core` dependency here,
//! closes that path into a cycle.
//!
//! That is also what buys the build headroom: because the leaves stop at this
//! crate, Cargo compiles and caches all 42 independently instead of folding
//! them into one `rspice-core` translation unit. Measured 2026-08-01, the
//! split cut peak single-`rustc` memory for the full corpus from 9.96 GB to
//! 2.59 GB, back under the 3 GB build gate.

// Fixed-arity stamp entry points deliberately keep small derivative sets in
// scalar arguments so generated call sites inline without temporary arrays.
#![allow(clippy::too_many_arguments)]

pub type Value = f64;

/// Version of the immutable catalog contract emitted beside generated models.
///
/// This is intentionally independent of checkpoint and stamp-workspace
/// versions: consumers use it to decide whether a persisted schematic binding
/// can be reconstructed from the compiled model catalog.
pub const GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION: u32 = 2;

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
    pub default: Option<Value>,
    pub minimum: Option<GeneratedVerilogAParameterBound>,
    pub maximum: Option<GeneratedVerilogAParameterBound>,
    pub excluded_values: &'static [Value],
    pub has_dynamic_constraints: bool,
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
    /// Digest covering the executable instance/checkpoint layout.
    pub checkpoint_identity: &'static str,
    /// External terminals in the exact positional order expected by netlists.
    pub terminals: &'static [GeneratedVerilogATerminalDescriptor],
    pub parameters: &'static [GeneratedVerilogAParameterDescriptor],
    pub total_node_count: usize,
    pub internal_node_names: &'static [&'static str],
    pub branch_count: usize,
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

/// Packed partial derivatives used by precompiled Verilog-A model bodies.
///
/// This lives in the shared leaf dependency so every generated device does not
/// have to parse its own copy. Widths emitted by the shipped catalog use the
/// loop-free fixed types below. This const-generic form is retained only as a
/// correctness fallback for future widths above the fixed ceiling.
#[doc(hidden)]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Lanes<const N: usize>(pub [f64; N]);

impl<const N: usize> core::ops::Add for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] + rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Sub for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] - rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Mul<f64> for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] * rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Div<f64> for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] / rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Index<usize> for Lanes<N> {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: usize) -> &f64 {
        &self.0[index]
    }
}

/// Defines one fixed-width derivative value with no source or IR loop in its
/// arithmetic implementations. LLVM can inline these scalar expressions
/// directly without rediscovering and rotating thousands of tiny loops in a
/// wide generated stamp.
macro_rules! define_fixed_lanes {
    ($name:ident, $width:literal, [$($index:tt),+ $(,)?]) => {
        #[doc(hidden)]
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $name(pub [f64; $width]);

        impl core::ops::Add for $name {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self([$((self.0[$index] + rhs.0[$index])),+])
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self([$((self.0[$index] - rhs.0[$index])),+])
            }
        }

        impl core::ops::Mul<f64> for $name {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: f64) -> Self {
                Self([$((self.0[$index] * rhs)),+])
            }
        }

        impl core::ops::Div<f64> for $name {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: f64) -> Self {
                Self([$((self.0[$index] / rhs)),+])
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = f64;

            #[inline(always)]
            fn index(&self, index: usize) -> &f64 {
                &self.0[index]
            }
        }
    };
}

define_fixed_lanes!(L2, 2, [0, 1]);
define_fixed_lanes!(L3, 3, [0, 1, 2]);
define_fixed_lanes!(L4, 4, [0, 1, 2, 3]);
define_fixed_lanes!(L5, 5, [0, 1, 2, 3, 4]);
define_fixed_lanes!(L6, 6, [0, 1, 2, 3, 4, 5]);
define_fixed_lanes!(L7, 7, [0, 1, 2, 3, 4, 5, 6]);
define_fixed_lanes!(L8, 8, [0, 1, 2, 3, 4, 5, 6, 7]);
define_fixed_lanes!(L9, 9, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
define_fixed_lanes!(L10, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
define_fixed_lanes!(L11, 11, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
define_fixed_lanes!(L12, 12, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
define_fixed_lanes!(L13, 13, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
define_fixed_lanes!(L14, 14, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
define_fixed_lanes!(L15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
define_fixed_lanes!(
    L16,
    16,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
define_fixed_lanes!(
    L17,
    17,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);
define_fixed_lanes!(
    L18,
    18,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
);
define_fixed_lanes!(
    L19,
    19,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
    ]
);
define_fixed_lanes!(
    L20,
    20,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
    ]
);
define_fixed_lanes!(
    L21,
    21,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    ]
);
define_fixed_lanes!(
    L22,
    22,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21
    ]
);
define_fixed_lanes!(
    L23,
    23,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22
    ]
);
define_fixed_lanes!(
    L24,
    24,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
    ]
);
define_fixed_lanes!(
    L25,
    25,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24
    ]
);
define_fixed_lanes!(
    L26,
    26,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25
    ]
);
define_fixed_lanes!(
    L27,
    27,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26
    ]
);
define_fixed_lanes!(
    L28,
    28,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27
    ]
);
define_fixed_lanes!(
    L29,
    29,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28
    ]
);
define_fixed_lanes!(
    L30,
    30,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29
    ]
);
define_fixed_lanes!(
    L31,
    31,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30
    ]
);
define_fixed_lanes!(
    L32,
    32,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31
    ]
);

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

/// Apply dependency-ordered generated parameter-default aliases.
///
/// Validation runs immediately after every copy, before a later alias may read
/// the destination, preserving Verilog-A declaration-order semantics.
#[doc(hidden)]
pub fn install_generated_parameter_aliases(
    values: &mut [f64],
    aliases: &[(u16, u16)],
    validate: fn(usize, f64) -> Result<(), String>,
) -> Result<(), String> {
    for &(destination, source) in aliases {
        let source = *values
            .get(usize::from(source))
            .expect("generated parameter-alias source is outside parameter storage");
        let destination = usize::from(destination);
        *values
            .get_mut(destination)
            .expect("generated parameter-alias destination is outside parameter storage") = source;
        validate(destination, source)?;
    }
    Ok(())
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

#[doc(hidden)]
#[inline(always)]
pub fn rspice_limexp(x: f64) -> f64 {
    if x < 80.0 {
        x.exp()
    } else {
        (80.0f64).exp() * (x - 80.0 + 1.0)
    }
}

#[doc(hidden)]
#[inline(always)]
pub fn rspice_limited_exp(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34 * (x - 80.0 + 1.0)
    } else if x < -80.0 {
        1.804851387e-35
    } else {
        x.exp()
    }
}

#[doc(hidden)]
#[inline(always)]
pub fn rspice_limited_exp_derivative(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34
    } else if x < -80.0 {
        0.0
    } else {
        x.exp()
    }
}

/// Evaluate one generated `idt` state slot.
///
/// A non-integrating step returns and records the initial condition, so the
/// next integrating step starts there rather than at a value left by the last
/// operating-point solve.
#[doc(hidden)]
#[inline]
pub fn rspice_eval_idt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    active: bool,
    step: f64,
    slot: usize,
    value: f64,
    ic: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated idt state slot out of range");
    let started_from = if initialized[slot] {
        previous[slot]
    } else {
        ic
    };
    let total = if active {
        started_from + value * step
    } else {
        ic
    };
    current[slot] = total;
    if !active {
        previous[slot] = total;
        initialized[slot] = true;
    }
    total
}

/// Evaluate one generated `ddt` state slot.
#[doc(hidden)]
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn rspice_eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    active: bool,
    scale: f64,
    previous_value_scale: f64,
    older_value_scale: f64,
    previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] {
        previous[slot]
    } else {
        value
    };
    let older_value = if initialized[slot] {
        older[slot]
    } else {
        value
    };
    current[slot] = value;
    if active {
        let result = value * scale
            - previous_value * previous_value_scale
            - older_value * older_value_scale
            - derivative_previous[slot] * previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
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

#[doc(hidden)]
pub fn validate_generated_parameter(
    name: &str,
    value: f64,
    integer: bool,
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_generated_finite_parameter(name, value)?;
    if integer && value.fract() != 0.0 {
        return Err(format!(
            "parameter '{}' must be an integer, got {}",
            name, value
        ));
    }
    if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!(
            "parameter '{}' must fit in a 32-bit signed integer, got {}",
            name, value
        ));
    }
    if let Some((min, label)) = min {
        if min_exclusive {
            if value <= min {
                return Err(format!(
                    "parameter '{}' must be > {}, got {}",
                    name, label, value
                ));
            }
        } else if value < min {
            return Err(format!(
                "parameter '{}' must be >= {}, got {}",
                name, label, value
            ));
        }
    }
    if let Some((max, label)) = max {
        if max_exclusive {
            if value >= max {
                return Err(format!(
                    "parameter '{}' must be < {}, got {}",
                    name, label, value
                ));
            }
        } else if value > max {
            return Err(format!(
                "parameter '{}' must be <= {}, got {}",
                name, label, value
            ));
        }
    }
    for (excluded, label) in excluded {
        if value == *excluded {
            return Err(format!(
                "parameter '{}' must not equal {}, got {}",
                name, label, value
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

use rspice_matrix::{ComplexMatrix, CscIndex, StaticMatrix};

const DEFAULT_GMIN: Value = 1.0e-12;
const K_BOLTZMANN: Value = 1.380649e-23;
const Q_ELECTRON: Value = 1.602176634e-19;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedDdtCoefficients {
    pub active: bool,
    pub derivative_scale: Value,
    pub previous_value_scale: Value,
    pub older_value_scale: Value,
    pub previous_derivative_scale: Value,
}

impl GeneratedDdtCoefficients {
    #[inline]
    pub const fn inactive() -> Self {
        Self {
            active: false,
            derivative_scale: 0.0,
            previous_value_scale: 0.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        }
    }

    #[inline]
    pub fn from_companion_values(
        coeff_g: Value,
        coeff_v_n: Value,
        coeff_v_n_minus_1: Value,
        needs_two_history: bool,
        needs_current_history: bool,
        timestep: Value,
    ) -> Self {
        const DDT_EPSILON: Value = 1.0e-20;
        if !timestep.is_finite() || timestep.abs() <= DDT_EPSILON {
            return Self::inactive();
        }

        let inverse_timestep = 1.0 / timestep;
        Self {
            active: true,
            derivative_scale: coeff_g * inverse_timestep,
            previous_value_scale: coeff_v_n * inverse_timestep,
            older_value_scale: if needs_two_history {
                coeff_v_n_minus_1 * inverse_timestep
            } else {
                0.0
            },
            previous_derivative_scale: if needs_current_history { 1.0 } else { 0.0 },
        }
    }

    /// Scale the complete dynamic residual without changing its history
    /// representation. Xyce OneStep order two halves the enclosing static
    /// device residual, so generated `ddt` terms are doubled before that
    /// common scaling to retain `(Q_n - Q_previous) / h` at unit weight.
    #[inline]
    pub fn scaled(self, factor: Value) -> Self {
        debug_assert!(factor.is_finite() && factor >= 0.0);
        if !self.active || !factor.is_finite() || factor < 0.0 {
            return Self::inactive();
        }
        Self {
            active: true,
            derivative_scale: self.derivative_scale * factor,
            previous_value_scale: self.previous_value_scale * factor,
            older_value_scale: self.older_value_scale * factor,
            previous_derivative_scale: self.previous_derivative_scale * factor,
        }
    }
}

impl Default for GeneratedDdtCoefficients {
    #[inline]
    fn default() -> Self {
        Self::inactive()
    }
}

/// Accepted dynamic history needed to resume a generated Verilog-A instance.
///
/// Parameters, topology, static caches, and scratch buffers are deliberately
/// excluded: a resumed circuit rebuilds those from the canonical model and
/// validates their source digest before this state is restored.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeneratedVerilogAPersistentState {
    pub ddt_previous: Vec<Value>,
    pub ddt_older: Vec<Value>,
    pub ddt_derivative_previous: Vec<Value>,
    pub ddt_initialized: Vec<bool>,
    pub idt_previous: Vec<Value>,
    pub idt_initialized: Vec<bool>,
    pub limiter_anchor: Vec<Value>,
    pub limiter_initialized: Vec<bool>,
}

/// Mutable evaluation state captured around rejected nonlinear trial points.
///
/// Parameter values, topology, static caches, and linked matrix locations are
/// deliberately excluded. Generated models pack only their DDT/IDT and limiter
/// state into these two contiguous buffers.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeneratedVerilogARollbackState {
    pub values: Vec<Value>,
    pub flags: Vec<bool>,
}

pub const GENERATED_PERSISTENT_STATE_VERSION: u32 = 1;

/// Persistent state plus exact generated-model and instance provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedVerilogAInstanceCheckpoint {
    pub instance_name: String,
    pub model_name: String,
    pub model_identity: String,
    pub state_version: u32,
    pub state: GeneratedVerilogAPersistentState,
}

/// Which unknown a lane of a packed derivative array belongs to.
///
/// A packed array is sized to the whole device, so a model with more lanes than
/// any one equation uses carries `Unused` entries rather than resizing per
/// equation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedStampLane {
    Node(usize),
    Branch(usize),
    Unused,
}

/// A recoverable failure reported while evaluating generated device code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedEvaluationError {
    AnalogLoopLimit {
        phase: &'static str,
        iterations: usize,
        limit: usize,
    },
}

impl std::fmt::Display for GeneratedEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnalogLoopLimit {
                phase,
                iterations,
                limit,
            } => write!(
                f,
                "generated Verilog-A {phase} exceeded its analog-loop limit: {iterations} iterations (limit {limit})"
            ),
        }
    }
}

impl std::error::Error for GeneratedEvaluationError {}

/// Generated evaluation failure with concrete model and instance provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedVerilogAEvaluationError {
    pub instance_name: String,
    pub model_name: &'static str,
    pub source: GeneratedEvaluationError,
}

impl std::fmt::Display for GeneratedVerilogAEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "generated Verilog-A instance '{}' (model '{}') failed: {}",
            self.instance_name, self.model_name, self.source
        )
    }
}

impl std::error::Error for GeneratedVerilogAEvaluationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Canonical Verilog-A noise primitive represented by generated device code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedNoiseKind {
    White,
    Flicker,
    Table,
}

/// One endpoint of a generated noise contribution in device-local topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoiseEndpoint {
    /// Index into the generated instance's complete node array. `None` is ground.
    pub local_node: Option<usize>,
    pub name: &'static str,
    pub is_internal: bool,
}

/// Immutable metadata emitted from a canonical Verilog-A noise contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoiseDescriptor {
    pub mechanism: &'static str,
    pub label: Option<&'static str>,
    pub kind: GeneratedNoiseKind,
    pub equation: usize,
    pub is_current: bool,
    pub branch_ordinal: Option<usize>,
    pub pos: GeneratedNoiseEndpoint,
    pub neg: GeneratedNoiseEndpoint,
    pub table_len: usize,
    pub table_log_interp: bool,
}

/// Evaluated, frequency-independent data for one generated noise primitive.
///
/// Table operands retain their canonical flat ordering. Interpretation and
/// interpolation belong to the analysis layer, not the generated-device ABI.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedNoiseEvaluation {
    pub active: bool,
    pub psd: Value,
    pub exponent: Option<Value>,
    pub table_operands: Vec<Value>,
}

/// Allocation-free view produced by a generated one-pass noise evaluator.
///
/// Table operands borrow a generated stack buffer for the duration of the
/// visitor call. Consumers that retain an evaluation can materialize the
/// existing owned representation with [`GeneratedNoiseEvaluationRef::to_owned`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedNoiseEvaluationRef<'a> {
    pub active: bool,
    pub psd: Value,
    pub exponent: Option<Value>,
    pub table_operands: &'a [Value],
}

impl GeneratedNoiseEvaluationRef<'_> {
    #[inline]
    pub fn to_owned(self) -> GeneratedNoiseEvaluation {
        GeneratedNoiseEvaluation {
            active: self.active,
            psd: self.psd,
            exponent: self.exponent,
            table_operands: self.table_operands.to_vec(),
        }
    }
}

/// Type-erased sink used by generated devices to report every noise source in
/// one dependency-schedule traversal.
pub trait GeneratedNoiseVisitor {
    /// Return `false` to stop evaluation after the current source.
    fn visit(&mut self, index: usize, evaluation: GeneratedNoiseEvaluationRef<'_>) -> bool;
}

impl<F> GeneratedNoiseVisitor for F
where
    F: for<'a> FnMut(usize, GeneratedNoiseEvaluationRef<'a>) -> bool,
{
    #[inline]
    fn visit(&mut self, index: usize, evaluation: GeneratedNoiseEvaluationRef<'_>) -> bool {
        self(index, evaluation)
    }
}

/// A generated noise evaluator rejected invalid model state or output.
#[derive(Debug, Clone, PartialEq)]
pub enum GeneratedNoiseEvaluationError {
    SourceIndexOutOfRange {
        index: usize,
        count: usize,
    },
    NonFinite {
        index: usize,
        quantity: &'static str,
        value: Value,
    },
    NegativePower {
        index: usize,
        value: Value,
    },
    InvalidMultiplicity {
        value: Value,
    },
    AnalogLoopLimit {
        iterations: usize,
        limit: usize,
    },
}

impl std::fmt::Display for GeneratedNoiseEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceIndexOutOfRange { index, count } => write!(
                f,
                "generated Verilog-A noise source index {index} is outside the {count}-source catalog"
            ),
            Self::NonFinite {
                index,
                quantity,
                value,
            } => write!(
                f,
                "generated Verilog-A noise source {index} produced non-finite {quantity} {value}"
            ),
            Self::NegativePower { index, value } => write!(
                f,
                "generated Verilog-A noise source {index} produced negative power {value}"
            ),
            Self::InvalidMultiplicity { value } => write!(
                f,
                "generated Verilog-A noise evaluation requires a finite positive multiplicity, found {value}"
            ),
            Self::AnalogLoopLimit { iterations, limit } => write!(
                f,
                "generated Verilog-A noise evaluation exceeded its analog-loop limit: {iterations} iterations (limit {limit})"
            ),
        }
    }
}

impl std::error::Error for GeneratedNoiseEvaluationError {}

/// Engine-neutral location at which a generated noise source is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedNoiseInjection {
    Current {
        node_pos: usize,
        node_neg: usize,
    },
    /// A potential-noise EMF applied to the equation for this concrete 1-based
    /// circuit branch ordinal returned by `CircuitData::allocate_branch`. This
    /// is not the descriptor's device-local branch ordinal; the analysis layer
    /// owns its conversion to a matrix axis.
    Potential {
        branch: usize,
    },
}

/// Canonical descriptor paired with topology mapped to a concrete instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedMappedNoiseDescriptor {
    pub descriptor: GeneratedNoiseDescriptor,
    pub injection: GeneratedNoiseInjection,
}

/// Generated noise topology was inconsistent with its concrete instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratedNoiseTopologyError {
    LocalNodeOutOfRange {
        endpoint: &'static str,
        local_node: usize,
        node_count: usize,
    },
    CurrentSourceHasBranch {
        branch_ordinal: usize,
    },
    PotentialSourceMissingBranch,
    BranchOrdinalOutOfRange {
        branch_ordinal: usize,
        branch_count: usize,
    },
}

impl std::fmt::Display for GeneratedNoiseTopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalNodeOutOfRange {
                endpoint,
                local_node,
                node_count,
            } => write!(
                f,
                "generated Verilog-A noise {endpoint} endpoint references local node {local_node}, but the instance has {node_count} nodes"
            ),
            Self::CurrentSourceHasBranch { branch_ordinal } => write!(
                f,
                "generated Verilog-A current-noise source unexpectedly references branch ordinal {branch_ordinal}"
            ),
            Self::PotentialSourceMissingBranch => write!(
                f,
                "generated Verilog-A potential-noise source has no branch equation"
            ),
            Self::BranchOrdinalOutOfRange {
                branch_ordinal,
                branch_count,
            } => write!(
                f,
                "generated Verilog-A potential-noise source references branch ordinal {branch_ordinal}, but the instance has {branch_count} branches"
            ),
        }
    }
}

impl std::error::Error for GeneratedNoiseTopologyError {}

impl GeneratedNoiseDescriptor {
    /// Validate and map device-local topology without assuming an engine matrix
    /// layout. Ground is represented by circuit node zero.
    pub fn map_topology(
        self,
        nodes: &[usize],
        branches: &[usize],
    ) -> Result<GeneratedMappedNoiseDescriptor, GeneratedNoiseTopologyError> {
        // Validate both canonical endpoints even when a potential contribution
        // is ultimately injected on its branch-equation row. This keeps corrupt
        // generated metadata from being silently masked by injection topology.
        let node_pos = map_generated_noise_endpoint(self.pos, "positive", nodes)?;
        let node_neg = map_generated_noise_endpoint(self.neg, "negative", nodes)?;
        let injection = if self.is_current {
            if let Some(branch_ordinal) = self.branch_ordinal {
                return Err(GeneratedNoiseTopologyError::CurrentSourceHasBranch { branch_ordinal });
            }
            GeneratedNoiseInjection::Current { node_pos, node_neg }
        } else {
            let branch_ordinal = self
                .branch_ordinal
                .ok_or(GeneratedNoiseTopologyError::PotentialSourceMissingBranch)?;
            let branch = branches.get(branch_ordinal).copied().ok_or(
                GeneratedNoiseTopologyError::BranchOrdinalOutOfRange {
                    branch_ordinal,
                    branch_count: branches.len(),
                },
            )?;
            GeneratedNoiseInjection::Potential { branch }
        };
        Ok(GeneratedMappedNoiseDescriptor {
            descriptor: self,
            injection,
        })
    }
}

fn map_generated_noise_endpoint(
    endpoint: GeneratedNoiseEndpoint,
    endpoint_role: &'static str,
    nodes: &[usize],
) -> Result<usize, GeneratedNoiseTopologyError> {
    endpoint.local_node.map_or(Ok(0), |local_node| {
        nodes
            .get(local_node)
            .copied()
            .ok_or(GeneratedNoiseTopologyError::LocalNodeOutOfRange {
                endpoint: endpoint_role,
                local_node,
                node_count: nodes.len(),
            })
    })
}

#[derive(Debug, Clone, Copy)]
pub enum GeneratedAnalysisKind {
    Dc,
    Ac,
    Tran,
    Noise,
    Ic,
}

/// Controls whether a generated evaluation may apply Newton limiting and
/// advance per-instance limiter history. This is intentionally orthogonal to
/// the physical analysis queried through Verilog-A `$analysis(...)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedEvaluationMode {
    NewtonLimited,
    StaticProbe,
    /// Evaluate the transient model's physical static `F-B` contribution
    /// without advancing or contributing `ddt`/`idt` operators. This is used
    /// only to capture Xyce OneStep's accepted static-history vector.
    StaticDaeProbe,
    SmallSignal,
}

impl GeneratedEvaluationMode {
    #[inline]
    pub const fn default_for_analysis(analysis: GeneratedAnalysisKind) -> Self {
        match analysis {
            GeneratedAnalysisKind::Ac | GeneratedAnalysisKind::Noise => Self::SmallSignal,
            GeneratedAnalysisKind::Dc | GeneratedAnalysisKind::Tran | GeneratedAnalysisKind::Ic => {
                Self::NewtonLimited
            }
        }
    }
}

/// Simulator-owned parameters visible to generated Verilog-A `$simparam` calls.
///
/// `Option` distinguishes an explicitly configured zero from an unavailable
/// parameter, in which case the model-provided fallback remains authoritative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedSimulationParameters {
    gmin: Option<Value>,
    pnjmaxi: Option<Value>,
}

impl GeneratedSimulationParameters {
    #[inline]
    pub const fn new() -> Self {
        Self {
            gmin: Some(DEFAULT_GMIN),
            pnjmaxi: None,
        }
    }

    #[inline]
    pub fn set_gmin(&mut self, value: Value) {
        self.gmin = value.is_finite().then_some(value.max(0.0));
    }

    #[inline]
    pub fn set_pnjmaxi(&mut self, value: Option<Value>) {
        self.pnjmaxi = value.filter(|value| value.is_finite() && *value >= 0.0);
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<Value> {
        if name.eq_ignore_ascii_case("gmin") {
            self.gmin
        } else if name.eq_ignore_ascii_case("pnjmaxi") {
            self.pnjmaxi
        } else {
            None
        }
    }
}

impl Default for GeneratedSimulationParameters {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratedAnalysisKind {
    #[inline]
    fn matches_query(self, query: &str) -> bool {
        match query {
            "dc" | "op" => matches!(self, Self::Dc),
            "ac" => matches!(self, Self::Ac),
            "tran" => matches!(self, Self::Tran),
            "noise" => matches!(self, Self::Noise),
            "ic" => matches!(self, Self::Ic),
            "static" => matches!(self, Self::Dc | Self::Ic),
            "smallsig" => matches!(self, Self::Ac | Self::Noise),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedEvalContext<'a> {
    voltages: &'a [Value],
    temperature: Value,
    num_nodes: usize,
    analysis: GeneratedAnalysisKind,
    analysis_initial_step: bool,
    analysis_final_step: bool,
    simparams: GeneratedSimulationParameters,
    evaluation_mode: GeneratedEvaluationMode,
    evaluation_error: std::cell::Cell<Option<GeneratedEvaluationError>>,
}

impl<'a> GeneratedEvalContext<'a> {
    #[inline]
    pub fn new(voltages: &'a [Value], temperature: Value, num_nodes: usize) -> Self {
        Self::with_analysis(voltages, temperature, num_nodes, GeneratedAnalysisKind::Dc)
    }

    #[inline]
    pub fn with_analysis(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
    ) -> Self {
        Self::with_analysis_step(voltages, temperature, num_nodes, analysis, false, false)
    }

    #[inline]
    pub fn with_analysis_step(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
    ) -> Self {
        Self::with_analysis_step_and_simparams(
            voltages,
            temperature,
            num_nodes,
            analysis,
            initial,
            final_step,
            GeneratedSimulationParameters::default(),
        )
    }

    #[inline]
    pub fn with_analysis_step_and_simparams(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
        simparams: GeneratedSimulationParameters,
    ) -> Self {
        let evaluation_mode = GeneratedEvaluationMode::default_for_analysis(analysis);
        Self::with_analysis_step_simparams_and_mode(
            voltages,
            temperature,
            num_nodes,
            analysis,
            initial,
            final_step,
            simparams,
            evaluation_mode,
        )
    }

    #[inline]
    pub fn with_analysis_step_simparams_and_mode(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
        simparams: GeneratedSimulationParameters,
        evaluation_mode: GeneratedEvaluationMode,
    ) -> Self {
        Self {
            voltages,
            temperature,
            num_nodes,
            analysis,
            analysis_initial_step: initial,
            analysis_final_step: final_step,
            simparams,
            evaluation_mode,
            evaluation_error: std::cell::Cell::new(None),
        }
    }

    #[inline]
    pub fn limiting_enabled(&self) -> bool {
        matches!(self.evaluation_mode, GeneratedEvaluationMode::NewtonLimited)
    }

    /// Whether transient dynamic operators may contribute and update their
    /// trial state during this evaluation.
    #[inline]
    pub fn dynamic_operators_enabled(&self) -> bool {
        !matches!(
            self.evaluation_mode,
            GeneratedEvaluationMode::StaticDaeProbe
        )
    }

    #[inline]
    pub fn report_analog_loop_limit(&self, phase: &'static str, iterations: usize, limit: usize) {
        if self.evaluation_error.get().is_none() {
            self.evaluation_error
                .set(Some(GeneratedEvaluationError::AnalogLoopLimit {
                    phase,
                    iterations,
                    limit,
                }));
        }
    }

    #[inline]
    pub fn take_evaluation_error(&self) -> Option<GeneratedEvaluationError> {
        self.evaluation_error.take()
    }

    #[inline]
    pub fn simparam_or(&self, name: &str, fallback: Value) -> Value {
        self.simparams.get(name).unwrap_or(fallback)
    }

    #[inline]
    pub fn has_simparam(&self, name: &str) -> bool {
        self.simparams.get(name).is_some()
    }

    #[inline]
    pub fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn temperature(&self) -> Value {
        self.temperature
    }

    #[inline]
    pub fn thermal_voltage(&self) -> Value {
        self.temperature * K_BOLTZMANN / Q_ELECTRON
    }

    #[inline]
    pub fn branch_current(&self, branch_ordinal: usize) -> Value {
        if branch_ordinal == 0 {
            0.0
        } else {
            self.voltages
                .get(self.num_nodes + branch_ordinal - 1)
                .copied()
                .unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn analysis(&self, query: &str) -> bool {
        self.analysis.matches_query(query)
    }

    #[inline]
    pub fn analysis_dc(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Dc)
    }

    #[inline]
    pub fn analysis_ac(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Ac)
    }

    #[inline]
    pub fn analysis_tran(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Tran)
    }

    #[inline]
    pub fn analysis_noise(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Noise)
    }

    #[inline]
    pub fn analysis_ic(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Ic)
    }

    #[inline]
    pub fn analysis_static(&self) -> bool {
        matches!(
            self.analysis,
            GeneratedAnalysisKind::Dc | GeneratedAnalysisKind::Ic
        )
    }

    #[inline]
    pub fn analysis_smallsig(&self) -> bool {
        matches!(
            self.analysis,
            GeneratedAnalysisKind::Ac | GeneratedAnalysisKind::Noise
        )
    }

    #[inline]
    pub fn analysis_initial_step(&self) -> bool {
        self.analysis_initial_step
    }

    #[inline]
    pub fn analysis_final_step(&self) -> bool {
        self.analysis_final_step
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeneratedStaticStampCache {
    node_count: usize,
    branch_count: usize,
    node_axes: Vec<Option<usize>>,
    branch_axes: Vec<Option<usize>>,
    axis_matrix_indices: Vec<Option<usize>>,
    matrix_axis_lookup: Vec<(usize, usize)>,
    slots: GeneratedStaticStampSlots,
}

#[derive(Debug, Clone, Default)]
struct GeneratedStaticStampSlots {
    logical_len: usize,
    linked_len: usize,
    dense: Vec<Option<CscIndex>>,
    entries: Vec<(usize, CscIndex)>,
}

impl GeneratedStaticStampSlots {
    const MAX_DENSE_LOGICAL_LEN: usize = 1_024;

    #[inline]
    fn clear(&mut self) {
        self.logical_len = 0;
        self.linked_len = 0;
        self.dense.clear();
        self.entries.clear();
    }

    #[inline]
    fn reset(&mut self, logical_len: usize) {
        self.logical_len = logical_len;
        self.linked_len = 0;
        self.dense.clear();
        self.entries.clear();
    }

    #[inline]
    fn push(&mut self, key: usize, index: CscIndex) {
        debug_assert!(
            self.entries
                .last()
                .is_none_or(|&(previous, _)| previous < key),
            "generated static stamp slots must be linked in key order"
        );
        self.linked_len += 1;
        self.entries.push((key, index));
    }

    fn finish(&mut self) {
        let use_dense = self.logical_len <= Self::MAX_DENSE_LOGICAL_LEN
            || self.entries.len().saturating_mul(3) >= self.logical_len;
        if !use_dense {
            return;
        }

        self.dense.resize(self.logical_len, None);
        for (key, index) in self.entries.drain(..) {
            self.dense[key] = Some(index);
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.logical_len
    }

    #[inline]
    fn get(&self, key: usize) -> Option<CscIndex> {
        if !self.dense.is_empty() {
            return self.dense.get(key).copied().flatten();
        }
        self.entries
            .binary_search_by_key(&key, |&(entry_key, _)| entry_key)
            .ok()
            .map(|position| self.entries[position].1)
    }

    #[inline]
    fn linked_len(&self) -> usize {
        self.linked_len
    }
}

impl GeneratedStaticStampCache {
    #[inline]
    pub fn link(
        &mut self,
        matrix: &StaticMatrix,
        nodes: &[usize],
        branches: &[usize],
        num_nodes: usize,
    ) {
        self.rebuild_axis_indices(nodes, branches, num_nodes);
        let width = self.axis_count();
        self.slots.reset(width * width);
        for row_axis in 0..width {
            let Some(row) = self.axis_matrix_indices[row_axis] else {
                continue;
            };
            for col_axis in 0..width {
                let Some(col) = self.axis_matrix_indices[col_axis] else {
                    continue;
                };
                if let Some(index) = matrix.get_index(row, col) {
                    self.slots.push(row_axis * width + col_axis, index);
                }
            }
        }
        self.slots.finish();
    }

    /// Number of CSC locations retained after linking.
    #[inline]
    pub fn linked_slot_count(&self) -> usize {
        self.slots.linked_len()
    }

    #[inline]
    pub fn axis_indices_match(
        &self,
        nodes: &[usize],
        branches: &[usize],
        num_nodes: usize,
    ) -> bool {
        let axis_count = nodes.len() + branches.len();
        let expected_first_branch = branches
            .first()
            .copied()
            .map(|branch| num_nodes + branch - 1);
        let first_branch_axis = nodes.len();
        let branch_index_matches = match expected_first_branch {
            Some(expected) => {
                self.axis_matrix_indices
                    .get(first_branch_axis)
                    .copied()
                    .flatten()
                    == Some(expected)
            }
            None => true,
        };
        self.axis_matrix_indices.len() == axis_count && branch_index_matches
    }

    /// Discard linked matrix locations without reallocating the surrounding
    /// cache object.
    #[inline]
    pub fn clear_linked_slots(&mut self) {
        self.slots.clear();
    }

    /// Rebuild the device-local-to-global axis mapping without linking matrix
    /// locations. The engine uses this for matrix-free evaluation paths.
    #[inline]
    pub fn rebuild_axis_indices(&mut self, nodes: &[usize], branches: &[usize], num_nodes: usize) {
        self.node_count = nodes.len();
        self.branch_count = branches.len();
        self.axis_matrix_indices.clear();
        self.axis_matrix_indices
            .reserve(self.node_count + self.branch_count);
        self.axis_matrix_indices
            .extend(nodes.iter().copied().map(Self::node_matrix_index));
        self.axis_matrix_indices.extend(
            branches
                .iter()
                .copied()
                .map(|branch| Self::branch_matrix_index(num_nodes, branch)),
        );

        self.node_axes.clear();
        self.node_axes.reserve(self.node_count);
        self.node_axes.extend(
            self.axis_matrix_indices
                .iter()
                .take(self.node_count)
                .enumerate()
                .map(|(axis, matrix_index)| matrix_index.is_some().then_some(axis)),
        );

        self.branch_axes.clear();
        self.branch_axes.reserve(self.branch_count);
        self.branch_axes.extend(
            self.axis_matrix_indices
                .iter()
                .skip(self.node_count)
                .take(self.branch_count)
                .enumerate()
                .map(|(branch, matrix_index)| {
                    matrix_index.is_some().then_some(self.node_count + branch)
                }),
        );

        self.matrix_axis_lookup.clear();
        self.matrix_axis_lookup.extend(
            self.axis_matrix_indices
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(axis, matrix_index)| matrix_index.map(|index| (index, axis))),
        );
        self.matrix_axis_lookup
            .sort_unstable_by_key(|&(matrix_index, axis)| (matrix_index, axis));
    }

    #[inline]
    fn axis_count(&self) -> usize {
        self.node_count + self.branch_count
    }

    #[inline]
    fn node_axis(&self, node_index: usize) -> Option<usize> {
        self.node_axes.get(node_index).copied().flatten()
    }

    #[inline]
    fn branch_axis(&self, branch_index: usize) -> Option<usize> {
        self.branch_axes.get(branch_index).copied().flatten()
    }

    #[inline]
    fn axis_matrix_index(&self, axis: usize) -> Option<usize> {
        self.axis_matrix_indices.get(axis).copied().flatten()
    }

    #[inline]
    fn slot_for_axes(&self, row_axis: usize, col_axis: usize) -> Option<CscIndex> {
        let width = self.axis_count();
        if width == 0 || self.slots.len() != width * width {
            return None;
        }
        self.slots
            .get(row_axis.checked_mul(width)?.checked_add(col_axis)?)
    }

    #[inline]
    fn slot_for_matrix_indices(&self, row: usize, col: usize) -> Option<CscIndex> {
        let row_axis = self.axis_for_matrix_index(row)?;
        let col_axis = self.axis_for_matrix_index(col)?;
        self.slot_for_axes(row_axis, col_axis)
    }

    #[inline]
    fn axis_for_matrix_index(&self, matrix_index: usize) -> Option<usize> {
        self.matrix_axis_lookup
            .binary_search_by_key(&matrix_index, |&(index, _axis)| index)
            .ok()
            .map(|idx| self.matrix_axis_lookup[idx].1)
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn branch_matrix_index(num_nodes: usize, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }
}

enum GeneratedMatrixTarget<'a> {
    Static { matrix: &'a mut StaticMatrix },
    AcReal { matrix: &'a mut ComplexMatrix },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedDerivative {
    axis: GeneratedDerivativeAxis,
    value: Value,
}

impl GeneratedDerivative {
    #[inline]
    pub const fn node(node: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Node(node),
            value,
        }
    }

    #[inline]
    pub const fn branch(branch_ordinal: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Branch(branch_ordinal),
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneratedDerivativeAxis {
    Node(usize),
    Branch(usize),
}

pub struct GeneratedStamper<'a> {
    matrix: GeneratedMatrixTarget<'a>,
    cache: Option<&'a GeneratedStaticStampCache>,
    rhs: Option<&'a mut [Value]>,
    /// Device-local terminal currents accumulated from the exact flow
    /// contributions emitted by generated code. The slice may cover only the
    /// external-node prefix; contributions involving later internal nodes are
    /// deliberately ignored while their external endpoint is still recorded.
    terminal_currents: Option<&'a mut [Value]>,
    voltages: &'a [Value],
    num_nodes: usize,
}

impl<'a> GeneratedStamper<'a> {
    #[inline]
    pub fn new(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static { matrix },
            cache: None,
            rhs: Some(rhs),
            terminal_currents: None,
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn new_with_static_cache(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static { matrix },
            cache: Some(cache),
            rhs: Some(rhs),
            terminal_currents: None,
            voltages,
            num_nodes,
        }
    }

    /// Construct a static stamper that also observes exact device-local
    /// terminal currents.
    ///
    /// Generated contributions use local node ordinals, which preserves the
    /// identity of shorted terminals and avoids trying to infer per-terminal
    /// current from an assembled nodal row. `terminal_currents` is reset by
    /// the caller and accumulated in the same sign convention as Verilog-A
    /// flow: positive current enters the contribution's positive endpoint.
    #[inline]
    pub fn new_with_static_cache_and_terminal_currents(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
        terminal_currents: &'a mut [Value],
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static { matrix },
            cache: Some(cache),
            rhs: Some(rhs),
            terminal_currents: Some(terminal_currents),
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn new_ac_real(
        matrix: &'a mut ComplexMatrix,
        voltages: &'a [Value],
        num_nodes: usize,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::AcReal { matrix },
            cache: None,
            rhs: None,
            terminal_currents: None,
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn new_ac_real_with_static_cache(
        matrix: &'a mut ComplexMatrix,
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::AcReal { matrix },
            cache: Some(cache),
            rhs: None,
            terminal_currents: None,
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn stamp_current_const(&mut self, pos: Option<usize>, neg: Option<usize>, value: Value) {
        if self.rhs.is_none() {
            return;
        }
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        self.add_current_rhs_pair(pos_row, neg_row, value);
    }

    #[inline]
    pub fn stamp_current_node1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0) - derivative1 * self.node_value(node1)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node3(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value(node0)
                - derivative1 * self.node_value(node1)
                - derivative2 * self.node_value(node2)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if let Some(col) = Self::node_matrix_index(node2) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.branch_value(branch0)
                - derivative1 * self.branch_value(branch1)
        } else {
            0.0
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node1_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0) - derivative1 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value(node0)
                - derivative1 * self.node_value(node1)
                - derivative2 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for derivative in derivatives {
            if needs_rhs {
                equivalent -= derivative.value * self.axis_value(derivative.axis);
            }
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative.value);
            }
        }

        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_dense(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.node_value(node);
            }
            if let Some(col) = Self::node_matrix_index(node) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.branch_value(branch);
            }
            if let Some(col) = self.branch_matrix_index(branch) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }

        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_const_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
    ) {
        if self.rhs.is_none() {
            return;
        }
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        self.add_current_rhs_axis_pair(pos_axis, neg_axis, value);
    }

    #[inline]
    pub fn stamp_current_node1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value_local(node0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.node_value_local(node1)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node3_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.node_value_local(node1)
                - derivative2 * self.node_value_local(node2)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if let Some(col_axis) = self.node_axis_local(node2) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.branch_value_local(branch0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.branch_value_local(branch0)
                - derivative1 * self.branch_value_local(branch1)
        } else {
            0.0
        };
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node1_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.branch_value_local(branch0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.node_value_local(node1)
                - derivative2 * self.branch_value_local(branch0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    /// Stamp a current whose derivatives arrive as one packed array.
    ///
    /// The `stamp_current_node*_local` family above takes its derivatives as
    /// loose arguments, one pair per unknown, which needs a distinct entry
    /// point for every arity a model happens to use. A backend that carries
    /// derivatives as a fixed-width array has them all in one place already,
    /// so this takes the array and the lane-to-unknown map beside it.
    ///
    /// `lanes` and `derivatives` are indexed alike: lane *k* of the array
    /// belongs to the unknown `lanes[k]` names.
    #[inline]
    pub fn stamp_current_packed<const LANES: usize>(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        lanes: &[GeneratedStampLane; LANES],
        derivatives: &[Value; LANES],
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (lane, derivative) in lanes.iter().zip(derivatives.iter()) {
            // A packed array covers every lane the *device* uses, so a given
            // equation leaves most of them at zero. Skipping those is not an
            // optimization: `0.0 * unknown` is `NaN` when the unknown is, which
            // happens while Newton is diverging, and it would poison the
            // equivalent source for a lane this equation does not even touch.
            // The loose-argument calls never see such a lane at all.
            if *derivative == 0.0 {
                continue;
            }
            let (col_axis, unknown) = match *lane {
                GeneratedStampLane::Node(node) => {
                    (self.node_axis_local(node), self.node_value_local(node))
                }
                GeneratedStampLane::Branch(branch) => (
                    self.branch_axis_local(branch),
                    self.branch_value_local(branch),
                ),
                GeneratedStampLane::Unused => continue,
            };
            if needs_rhs {
                equivalent -= *derivative * unknown;
            }
            if let Some(col_axis) = col_axis {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, *derivative);
            }
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let needs_rhs = self.rhs.is_some();
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative.value * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative.value,
                );
            }

            if needs_rhs {
                self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            }
            return;
        }

        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for derivative in derivatives {
            if needs_rhs {
                equivalent -= derivative.value * self.axis_value_local(derivative.axis);
            }
            if let Some(col_axis) = self.derivative_axis_local(derivative.axis) {
                self.add_current_derivative_axis_pair(
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative.value,
                );
            }
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let needs_rhs = self.rhs.is_some();
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }

            if needs_rhs {
                self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            }
            return;
        }

        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.node_value_local(node);
            }
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.branch_value_local(branch);
            }
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_indexed_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        debug_assert_eq!(node_derivative_indices.len(), node_derivatives.len());
        debug_assert_eq!(branch_derivative_indices.len(), branch_derivatives.len());

        let Some(cache) = self.cache else {
            return;
        };
        let pos_axis = pos.and_then(|node| cache.node_axis(node));
        let neg_axis = neg.and_then(|node| cache.node_axis(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
        }
    }

    #[inline(always)]
    pub fn stamp_current_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivative_indices: [usize; NODE_COUNT],
        node_derivatives: [Value; NODE_COUNT],
        branch_derivative_indices: [usize; BRANCH_COUNT],
        branch_derivatives: [Value; BRANCH_COUNT],
        derivative_scale: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let needs_rhs = self.rhs.is_some();
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivative_indices
                .iter()
                .copied()
                .zip(node_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branch_derivative_indices
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }

            if needs_rhs {
                self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            }
            return;
        }

        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.node_value_local(node);
            }
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.branch_value_local(branch);
            }
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_indexed_ad_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        self.observe_terminal_current_pair(pos, neg, value);
        debug_assert!(
            node_derivative_indices
                .iter()
                .all(|&index| index < node_derivatives.len())
        );
        debug_assert!(
            branch_derivative_indices
                .iter()
                .all(|&index| index < branch_derivatives.len())
        );

        let Some(cache) = self.cache else {
            return;
        };
        let pos_axis = pos.and_then(|node| cache.node_axis(node));
        let neg_axis = neg.and_then(|node| cache.node_axis(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for node in node_derivative_indices.iter().copied() {
            let derivative = derivative_scale * node_derivatives[node];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }
        for branch in branch_derivative_indices.iter().copied() {
            let derivative = derivative_scale * branch_derivatives[branch];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    fn add_current_derivative_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        col: usize,
        derivative: Value,
    ) {
        if let Some(row) = pos_row {
            self.add_real(row, col, derivative);
        }
        if let Some(row) = neg_row {
            self.add_real(row, col, -derivative);
        }
    }

    #[inline]
    fn add_current_rhs_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        equivalent: Value,
    ) {
        if let Some(rhs) = &mut self.rhs {
            if let Some(row) = pos_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot -= equivalent;
            }
            if let Some(row) = neg_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot += equivalent;
            }
        }
    }

    #[inline]
    pub fn stamp_potential_branch(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch_ordinal: usize,
        multiplicity: Value,
    ) {
        let Some(branch) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(node) = pos.filter(|node| *node > 0) {
            self.add_real(node - 1, branch, multiplicity);
            self.add_real(branch, node - 1, 1.0);
        }
        if let Some(node) = neg.filter(|node| *node > 0) {
            self.add_real(node - 1, branch, -multiplicity);
            self.add_real(branch, node - 1, -1.0);
        }
    }

    #[inline]
    pub fn stamp_potential_const(&mut self, branch_ordinal: usize, value: Value) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        self.add_potential_rhs(row, value);
    }

    #[inline]
    pub fn stamp_potential_node1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value - derivative0 * self.node_value(node0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent =
            value - derivative0 * self.node_value(node0) - derivative1 * self.node_value(node1);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value - derivative0 * self.branch_value(branch0);
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative0);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch2(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.branch_value(branch0)
            - derivative1 * self.branch_value(branch1);
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node1_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent =
            value - derivative0 * self.node_value(node0) - derivative1 * self.branch_value(branch0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value(node0)
            - derivative1 * self.node_value(node1)
            - derivative2 * self.branch_value(branch0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_real(row, col, -derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative2);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let mut equivalent = value;
        for derivative in derivatives {
            equivalent -= derivative.value * self.axis_value(derivative.axis);
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_real(row, col, -derivative.value);
            }
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    pub fn stamp_potential_dense(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let mut equivalent = value;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.axis_value(GeneratedDerivativeAxis::Node(node));
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Node(node)) {
                self.add_real(row, col, -derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.axis_value(GeneratedDerivativeAxis::Branch(branch));
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Branch(branch)) {
                self.add_real(row, col, -derivative);
            }
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    pub fn stamp_potential_branch_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch_index: usize,
        multiplicity: Value,
    ) {
        let Some(branch_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let branch_current = multiplicity * self.branch_value_local(branch_index);
        self.observe_terminal_current_pair(pos, neg, branch_current);
        if let Some(pos_axis) = pos.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(pos_axis, branch_axis, multiplicity);
            self.add_real_axis(branch_axis, pos_axis, 1.0);
        }
        if let Some(neg_axis) = neg.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(neg_axis, branch_axis, -multiplicity);
            self.add_real_axis(branch_axis, neg_axis, -1.0);
        }
    }

    /// Keep an allocated but inactive potential-source unknown nonsingular
    /// without coupling it to either endpoint. The unit diagonal constrains the
    /// unused branch current to zero. It is intentionally not scaled by device
    /// multiplicity: this is a solver identity row, not a physical contribution.
    #[inline]
    pub fn stamp_inactive_potential_branch(&mut self, branch_ordinal: usize) {
        if let Some(branch) = self.branch_matrix_index(branch_ordinal) {
            self.add_real(branch, branch, 1.0);
        }
    }

    /// Local-index counterpart of [`Self::stamp_inactive_potential_branch`]
    /// for generated devices linked through a static stamp cache.
    #[inline]
    pub fn stamp_inactive_potential_branch_local(&mut self, branch_index: usize) {
        if let Some(branch_axis) = self.branch_axis_local(branch_index) {
            self.add_real_axis(branch_axis, branch_axis, 1.0);
        }
    }

    #[inline]
    pub fn stamp_potential_const_local(&mut self, branch_index: usize, value: Value) {
        if let Some(row_axis) = self.branch_axis_local(branch_index) {
            self.add_potential_rhs_axis(row_axis, value);
        }
    }

    #[inline]
    pub fn stamp_potential_node1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value - derivative0 * self.node_value_local(node0);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value_local(node0)
            - derivative1 * self.node_value_local(node1);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value - derivative0 * self.branch_value_local(branch0);
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch2_local(
        &mut self,
        branch_index: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.branch_value_local(branch0)
            - derivative1 * self.branch_value_local(branch1);
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch1) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node1_branch1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value_local(node0)
            - derivative1 * self.branch_value_local(branch0);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2_branch1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value_local(node0)
            - derivative1 * self.node_value_local(node1)
            - derivative2 * self.branch_value_local(branch0);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative2);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_local(
        &mut self,
        branch_index: usize,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch_index) else {
                return;
            };
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                equivalent -= derivative.value * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative.value,
                );
            }
            self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
            return;
        }

        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let mut equivalent = value;
        for derivative in derivatives {
            equivalent -= derivative.value * self.axis_value_local(derivative.axis);
        }

        for derivative in derivatives {
            if let Some(col_axis) = self.derivative_axis_local(derivative.axis) {
                self.add_real_axis(row_axis, col_axis, -derivative.value);
            }
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_dense_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch_index) else {
                return;
            };
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
            return;
        }

        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let mut equivalent = value;
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.node_value_local(node);
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.branch_value_local(branch);
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_indexed_dense_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
    ) {
        debug_assert_eq!(node_derivative_indices.len(), node_derivatives.len());
        debug_assert_eq!(branch_derivative_indices.len(), branch_derivatives.len());

        let Some(cache) = self.cache else {
            return;
        };
        let Some(row_axis) = cache.branch_axis(branch_index) else {
            return;
        };
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
    }

    #[inline(always)]
    pub fn stamp_potential_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivative_indices: [usize; NODE_COUNT],
        node_derivatives: [Value; NODE_COUNT],
        branch_derivative_indices: [usize; BRANCH_COUNT],
        branch_derivatives: [Value; BRANCH_COUNT],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch_index) else {
                return;
            };
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivative_indices
                .iter()
                .copied()
                .zip(node_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            for (branch, derivative) in branch_derivative_indices
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
            return;
        }

        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let mut equivalent = value;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.node_value_local(node);
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.branch_value_local(branch);
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_indexed_ad_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
    ) {
        debug_assert!(
            node_derivative_indices
                .iter()
                .all(|&index| index < node_derivatives.len())
        );
        debug_assert!(
            branch_derivative_indices
                .iter()
                .all(|&index| index < branch_derivatives.len())
        );

        let Some(cache) = self.cache else {
            return;
        };
        let Some(row_axis) = cache.branch_axis(branch_index) else {
            return;
        };
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for node in node_derivative_indices.iter().copied() {
            let derivative = node_derivatives[node];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        for branch in branch_derivative_indices.iter().copied() {
            let derivative = branch_derivatives[branch];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
    }

    #[inline]
    fn add_potential_rhs(&mut self, row: usize, equivalent: Value) {
        if equivalent == 0.0 {
            return;
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    fn add_potential_rhs_axis(&mut self, row_axis: usize, equivalent: Value) {
        if let Some(row) = self.axis_matrix_index_local(row_axis) {
            self.add_potential_rhs(row, equivalent);
        }
    }

    #[inline]
    fn node_value(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn observe_terminal_current_pair(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
    ) {
        let Some(currents) = &mut self.terminal_currents else {
            return;
        };
        if let Some(slot) = pos.and_then(|node| currents.get_mut(node)) {
            *slot += value;
        }
        if let Some(slot) = neg.and_then(|node| currents.get_mut(node)) {
            *slot -= value;
        }
    }

    #[inline]
    fn node_value_local(&self, node_index: usize) -> Value {
        self.node_axis_local(node_index)
            .and_then(|axis| self.axis_matrix_index_local(axis))
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn branch_value_local(&self, branch_index: usize) -> Value {
        self.branch_axis_local(branch_index)
            .and_then(|axis| self.axis_matrix_index_local(axis))
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn branch_value(&self, branch: usize) -> Value {
        self.branch_matrix_index(branch)
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn axis_value(&self, axis: GeneratedDerivativeAxis) -> Value {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_value(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_value(branch),
        }
    }

    #[inline]
    fn axis_value_local(&self, axis: GeneratedDerivativeAxis) -> Value {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_value_local(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_value_local(branch),
        }
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => Self::node_matrix_index(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index(branch),
        }
    }

    #[inline]
    fn derivative_axis_local(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_axis_local(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_axis_local(branch),
        }
    }

    #[inline]
    fn derivative_axis_cached(
        cache: &GeneratedStaticStampCache,
        axis: GeneratedDerivativeAxis,
    ) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => cache.node_axis(node),
            GeneratedDerivativeAxis::Branch(branch) => cache.branch_axis(branch),
        }
    }

    #[inline]
    fn node_axis_local(&self, node_index: usize) -> Option<usize> {
        self.static_cache()?.node_axis(node_index)
    }

    #[inline]
    fn branch_axis_local(&self, branch_index: usize) -> Option<usize> {
        self.static_cache()?.branch_axis(branch_index)
    }

    #[inline]
    fn axis_matrix_index_local(&self, axis: usize) -> Option<usize> {
        self.static_cache()?.axis_matrix_index(axis)
    }

    #[inline]
    fn static_cache(&self) -> Option<&GeneratedStaticStampCache> {
        self.cache
    }

    #[inline]
    fn axis_value_cached(&self, cache: &GeneratedStaticStampCache, axis: usize) -> Value {
        cache
            .axis_matrix_index(axis)
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn add_current_derivative_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_current_rhs_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        equivalent: Value,
    ) {
        let pos_row = pos_axis.and_then(|axis| cache.axis_matrix_index(axis));
        let neg_row = neg_axis.and_then(|axis| cache.axis_matrix_index(axis));
        self.add_current_rhs_pair(pos_row, neg_row, equivalent);
    }

    #[inline]
    fn add_potential_rhs_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        row_axis: usize,
        equivalent: Value,
    ) {
        if let Some(row) = cache.axis_matrix_index(row_axis) {
            self.add_potential_rhs(row, equivalent);
        }
    }

    #[inline]
    fn add_real_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        row_axis: usize,
        col_axis: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let slot = if slots_ready {
            debug_assert!(row_axis < width);
            debug_assert!(col_axis < width);
            cache.slots.get(row_axis * width + col_axis)
        } else {
            None
        };
        if let Some(index) = slot {
            match &mut self.matrix {
                GeneratedMatrixTarget::Static { matrix } => matrix.stamp_direct(index, value),
                GeneratedMatrixTarget::AcReal { matrix } => matrix.stamp_direct_real(index, value),
            }
            return;
        }

        let Some(row) = cache.axis_matrix_index(row_axis) else {
            return;
        };
        let Some(col) = cache.axis_matrix_index(col_axis) else {
            return;
        };
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => matrix.add(row, col, value),
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }

    #[inline]
    fn add_current_derivative_axis_pair(
        &mut self,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(cache) = self.cache {
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_real_axis(row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_real_axis(row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_current_rhs_axis_pair(
        &mut self,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        equivalent: Value,
    ) {
        if let Some(cache) = self.cache {
            self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            return;
        }
        let pos_row = pos_axis.and_then(|axis| self.axis_matrix_index_local(axis));
        let neg_row = neg_axis.and_then(|axis| self.axis_matrix_index_local(axis));
        self.add_current_rhs_pair(pos_row, neg_row, equivalent);
    }

    #[inline]
    fn add_real_axis(&mut self, row_axis: usize, col_axis: usize, value: Value) {
        if value == 0.0 {
            return;
        }
        if let Some(cache) = self.cache {
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, value);
            return;
        }

        let Some(row) = self.axis_matrix_index_local(row_axis) else {
            return;
        };
        let Some(col) = self.axis_matrix_index_local(col_axis) else {
            return;
        };
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => {
                if let Some(index) = self
                    .cache
                    .and_then(|cache| cache.slot_for_axes(row_axis, col_axis))
                {
                    matrix.stamp_direct(index, value);
                } else {
                    matrix.add(row, col, value);
                }
            }
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }

    #[inline]
    fn add_real(&mut self, row: usize, col: usize, value: Value) {
        if value == 0.0 {
            return;
        }
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => {
                if let Some(index) = self
                    .cache
                    .and_then(|cache| cache.slot_for_matrix_indices(row, col))
                {
                    matrix.stamp_direct(index, value);
                } else {
                    matrix.add(row, col, value);
                }
            }
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }
}

pub struct GeneratedReactiveStamper<'a> {
    matrix: &'a mut ComplexMatrix,
    cache: Option<&'a GeneratedStaticStampCache>,
    nodes: &'a [usize],
    branches: &'a [usize],
    num_nodes: usize,
    omega: Value,
}

impl<'a> GeneratedReactiveStamper<'a> {
    #[inline]
    pub fn new(matrix: &'a mut ComplexMatrix, num_nodes: usize, omega: Value) -> Self {
        Self {
            matrix,
            cache: None,
            nodes: &[],
            branches: &[],
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn new_with_static_cache(
        matrix: &'a mut ComplexMatrix,
        num_nodes: usize,
        omega: Value,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix,
            cache: Some(cache),
            nodes: &[],
            branches: &[],
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn new_with_local_maps(
        matrix: &'a mut ComplexMatrix,
        nodes: &'a [usize],
        branches: &'a [usize],
        num_nodes: usize,
        omega: Value,
    ) -> Self {
        Self {
            matrix,
            cache: None,
            nodes,
            branches,
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn new_with_local_maps_and_static_cache(
        matrix: &'a mut ComplexMatrix,
        nodes: &'a [usize],
        branches: &'a [usize],
        num_nodes: usize,
        omega: Value,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix,
            cache: Some(cache),
            nodes,
            branches,
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn stamp_current_reactive(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            let neg_axis = neg
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = self.derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative.value,
                );
            }
            return;
        }

        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_current_reactive_derivative_pair(
                    pos_row,
                    neg_row,
                    col,
                    self.omega * derivative.value,
                );
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_dense(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            let neg_axis = neg
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let derivative_scale = self.omega * derivative_scale;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = Self::node_matrix_index(node)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = self
                    .branch_matrix_index(branch)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            return;
        }

        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let derivative_scale = self.omega * derivative_scale;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = Self::node_matrix_index(node) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index(branch) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached_local(cache, derivative.axis)
                else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative.value,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index_local(derivative.axis) {
                self.add_current_reactive_derivative_pair(
                    pos_row,
                    neg_row,
                    col,
                    self.omega * derivative.value,
                );
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let derivative_scale = self.omega * derivative_scale;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let derivative_scale = self.omega * derivative_scale;
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_indexed_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let derivative_scale = self.omega * derivative_scale;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let derivative_scale = self.omega * derivative_scale;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node3(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = Self::node_matrix_index(node2) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            if let Some(col_axis) = cache.node_axis(node0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node3_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            if let Some(col_axis) = cache.node_axis(node2) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative2,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node2) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index_local(branch1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative2,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    fn add_current_reactive_derivative_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        col: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row) = pos_row {
            self.add_imag(row, col, derivative);
        }
        if let Some(row) = neg_row {
            self.add_imag(row, col, -derivative);
        }
    }

    #[inline]
    fn add_current_reactive_derivative_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_imag_axis_cached(cache, slots_ready, width, row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_imag_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_imag_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        row_axis: usize,
        col_axis: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let slot = if slots_ready {
            debug_assert!(row_axis < width);
            debug_assert!(col_axis < width);
            cache.slots.get(row_axis * width + col_axis)
        } else {
            None
        };
        if let Some(index) = slot {
            self.matrix.stamp_direct_imag(index, value);
            return;
        }

        let Some(row) = cache.axis_matrix_index(row_axis) else {
            return;
        };
        let Some(col) = cache.axis_matrix_index(col_axis) else {
            return;
        };
        self.matrix.add_imag(row, col, value);
    }

    #[inline]
    fn add_imag(&mut self, row: usize, col: usize, value: Value) {
        if value == 0.0 {
            return;
        }
        if let Some(index) = self
            .cache
            .and_then(|cache| cache.slot_for_matrix_indices(row, col))
        {
            self.matrix.stamp_direct_imag(index, value);
        } else {
            self.matrix.add_imag(row, col, value);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch1(
        &mut self,
        branch_ordinal: usize,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch2(
        &mut self,
        branch_ordinal: usize,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1_branch1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2_branch1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative2);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive(
        &mut self,
        branch_ordinal: usize,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = self
                .branch_matrix_index(branch_ordinal)
                .and_then(|index| cache.axis_for_matrix_index(index))
            else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = self.derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative.value,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_imag(row, col, -self.omega * derivative.value);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_dense(
        &mut self,
        branch_ordinal: usize,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = self
                .branch_matrix_index(branch_ordinal)
                .and_then(|index| cache.axis_for_matrix_index(index))
            else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = Self::node_matrix_index(node)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = self
                    .branch_matrix_index(branch)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Node(node)) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Branch(branch)) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_local(
        &mut self,
        branch: usize,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached_local(cache, derivative.axis)
                else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative.value,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index_local(derivative.axis) {
                self.add_imag(row, col, -self.omega * derivative.value);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_dense_local(
        &mut self,
        branch: usize,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_indexed_dense_local(
        &mut self,
        branch: usize,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            if let Some(col_axis) = cache.node_axis(node0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch1_local(
        &mut self,
        branch: usize,
        branch0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            if let Some(col_axis) = cache.branch_axis(branch0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch2_local(
        &mut self,
        branch: usize,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch1) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index_local(branch1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1_branch1_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2_branch1_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative2,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative2);
        }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn node_matrix_index_local(&self, node: usize) -> Option<usize> {
        self.nodes
            .get(node)
            .copied()
            .and_then(Self::node_matrix_index)
    }

    #[inline]
    fn branch_matrix_index_local(&self, branch: usize) -> Option<usize> {
        self.branches
            .get(branch)
            .copied()
            .and_then(|branch| self.branch_matrix_index(branch))
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => Self::node_matrix_index(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index(branch),
        }
    }

    #[inline]
    fn axis_matrix_index_local(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_matrix_index_local(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index_local(branch),
        }
    }

    #[inline]
    fn derivative_axis_cached(
        &self,
        cache: &GeneratedStaticStampCache,
        axis: GeneratedDerivativeAxis,
    ) -> Option<usize> {
        self.axis_matrix_index(axis)
            .and_then(|index| cache.axis_for_matrix_index(index))
    }

    #[inline]
    fn derivative_axis_cached_local(
        cache: &GeneratedStaticStampCache,
        axis: GeneratedDerivativeAxis,
    ) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => cache.node_axis(node),
            GeneratedDerivativeAxis::Branch(branch) => cache.branch_axis(branch),
        }
    }
}

#[cfg(test)]
mod fixed_lane_tests {
    use super::*;

    #[test]
    fn descriptor_v2_unifies_terminal_current_and_dual_scope_metadata() {
        const TERMINALS: [GeneratedVerilogATerminalDescriptor; 1] =
            [GeneratedVerilogATerminalDescriptor {
                name: "FG",
                direction: GeneratedVerilogATerminalDirection::InOut,
                discipline: "electrical",
                current_parameter: "ifg",
            }];
        const PARAMETERS: [GeneratedVerilogAParameterDescriptor; 1] =
            [GeneratedVerilogAParameterDescriptor {
                name: "rth0",
                aliases: &["rth"],
                scope: GeneratedVerilogAParameterScope::Dual,
                is_integer: false,
                default: Some(0.0),
                minimum: Some(GeneratedVerilogAParameterBound {
                    value: 0.0,
                    exclusive: false,
                }),
                maximum: None,
                excluded_values: &[],
                has_dynamic_constraints: false,
            }];

        assert_eq!(GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION, 2);
        assert_eq!(TERMINALS[0].name, "FG");
        assert_eq!(TERMINALS[0].current_parameter, "ifg");
        assert_eq!(PARAMETERS[0].scope, GeneratedVerilogAParameterScope::Dual);
        assert_eq!(
            GeneratedParameterAssignment::new("rth", 1.0, GeneratedParameterOrigin::ModelCard)
                .origin,
            GeneratedParameterOrigin::ModelCard
        );
    }

    #[test]
    fn generated_ddt_coefficients_scale_the_complete_dynamic_residual() {
        let coefficients = GeneratedDdtCoefficients {
            active: true,
            derivative_scale: 2.0,
            previous_value_scale: -3.0,
            older_value_scale: 5.0,
            previous_derivative_scale: -7.0,
        };

        assert_eq!(
            coefficients.scaled(0.5),
            GeneratedDdtCoefficients {
                active: true,
                derivative_scale: 1.0,
                previous_value_scale: -1.5,
                older_value_scale: 2.5,
                previous_derivative_scale: -3.5,
            }
        );
        assert_eq!(
            GeneratedDdtCoefficients::inactive().scaled(2.0),
            GeneratedDdtCoefficients::inactive()
        );
    }

    #[test]
    fn static_dae_probe_is_the_only_mode_that_disables_dynamic_operators() {
        let voltages = [0.0];
        let dynamic = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Tran,
            false,
            false,
            GeneratedSimulationParameters::default(),
            GeneratedEvaluationMode::NewtonLimited,
        );
        let static_probe = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Tran,
            false,
            false,
            GeneratedSimulationParameters::default(),
            GeneratedEvaluationMode::StaticProbe,
        );
        let static_dae_probe = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Tran,
            false,
            false,
            GeneratedSimulationParameters::default(),
            GeneratedEvaluationMode::StaticDaeProbe,
        );

        assert!(dynamic.dynamic_operators_enabled());
        assert!(static_probe.dynamic_operators_enabled());
        assert!(!static_dae_probe.dynamic_operators_enabled());
    }

    #[test]
    fn inactive_potential_branch_pins_exact_unit_diagonal() {
        for cached in [false, true] {
            let mut matrix =
                StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).expect("one-entry branch matrix");
            let mut stamp_rhs = [0.0];
            let voltages = [17.0];
            let mut cache = GeneratedStaticStampCache::default();
            cache.link(&matrix, &[], &[1], 0);
            if cached {
                let mut stamper = GeneratedStamper::new_with_static_cache(
                    &mut matrix,
                    &mut stamp_rhs,
                    &voltages,
                    0,
                    &cache,
                );
                stamper.stamp_inactive_potential_branch_local(0);
            } else {
                let mut stamper = GeneratedStamper::new(&mut matrix, &mut stamp_rhs, &voltages, 0);
                stamper.stamp_inactive_potential_branch(1);
            }
            let solution = matrix.solve(&[2.5]).expect("unit diagonal solves");
            assert_eq!(solution, vec![2.5], "cached={cached}");
            assert_eq!(stamp_rhs, [0.0], "cached={cached}");
        }
    }

    #[test]
    fn terminal_current_observer_preserves_local_lead_identity_and_sign() {
        let mut matrix = StaticMatrix::from_triplets(
            3,
            3,
            &[
                (0, 2, 0.0),
                (1, 2, 0.0),
                (2, 0, 0.0),
                (2, 1, 0.0),
                (2, 2, 0.0),
            ],
        )
        .expect("potential-source topology");
        let nodes = [1, 1, 2];
        let branches = [1];
        let voltages = [0.0, 0.0, -0.25];
        let mut rhs = [0.0; 3];
        let mut currents = [0.0; 3];
        let mut cache = GeneratedStaticStampCache::default();
        cache.link(&matrix, &nodes, &branches, 2);

        {
            let mut stamper = GeneratedStamper::new_with_static_cache_and_terminal_currents(
                &mut matrix,
                &mut rhs,
                &voltages,
                2,
                &cache,
                &mut currents,
            );
            stamper.stamp_current_const_local(Some(0), Some(2), 2.0);
            stamper.stamp_current_const_local(Some(1), Some(2), 3.0);
            stamper.stamp_potential_branch_local(Some(0), Some(2), 0, 4.0);
        }

        assert_eq!(
            currents,
            [1.0, 3.0, -4.0],
            "shorted external terminals retain distinct local lead currents"
        );
        assert_eq!(rhs, [-5.0, 5.0, 0.0]);
        assert_eq!(currents.iter().sum::<Value>(), 0.0);
    }

    fn left<const N: usize>() -> [f64; N] {
        let values = [
            0.0,
            -0.0,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(0x7ff8_0000_0000_0042),
            -3.5,
        ];
        core::array::from_fn(|index| values[index % values.len()])
    }

    fn right<const N: usize>() -> [f64; N] {
        let values = [
            -0.0,
            0.0,
            -f64::MIN_POSITIVE,
            f64::from_bits(2),
            2.0,
            -7.25,
            f64::from_bits(0x7ff8_0000_0000_00a5),
            f64::INFINITY,
        ];
        core::array::from_fn(|index| values[(index + 3) % values.len()])
    }

    fn assert_same_bits<const N: usize>(actual: [f64; N], expected: [f64; N]) {
        for index in 0..N {
            assert_eq!(
                actual[index].to_bits(),
                expected[index].to_bits(),
                "lane {index} differs: actual={:?}, expected={:?}",
                actual[index],
                expected[index]
            );
        }
    }

    macro_rules! assert_fixed_width {
        ($name:ident, $width:literal) => {{
            let lhs = left::<$width>();
            let rhs = right::<$width>();
            assert_same_bits(($name(lhs) + $name(rhs)).0, (Lanes(lhs) + Lanes(rhs)).0);
            assert_same_bits(($name(lhs) - $name(rhs)).0, (Lanes(lhs) - Lanes(rhs)).0);
            assert_same_bits(($name(lhs) * -3.25).0, (Lanes(lhs) * -3.25).0);
            assert_same_bits(($name(lhs) / -3.25).0, (Lanes(lhs) / -3.25).0);
            for index in 0..$width {
                assert_eq!($name(lhs)[index].to_bits(), lhs[index].to_bits());
            }
            assert_eq!(
                core::mem::size_of::<$name>(),
                core::mem::size_of::<[f64; $width]>()
            );
            assert_eq!(
                core::mem::align_of::<$name>(),
                core::mem::align_of::<[f64; $width]>()
            );
        }};
    }

    #[test]
    fn every_fixed_lane_width_matches_the_generic_fallback_bit_for_bit() {
        assert_fixed_width!(L2, 2);
        assert_fixed_width!(L3, 3);
        assert_fixed_width!(L4, 4);
        assert_fixed_width!(L5, 5);
        assert_fixed_width!(L6, 6);
        assert_fixed_width!(L7, 7);
        assert_fixed_width!(L8, 8);
        assert_fixed_width!(L9, 9);
        assert_fixed_width!(L10, 10);
        assert_fixed_width!(L11, 11);
        assert_fixed_width!(L12, 12);
        assert_fixed_width!(L13, 13);
        assert_fixed_width!(L14, 14);
        assert_fixed_width!(L15, 15);
        assert_fixed_width!(L16, 16);
        assert_fixed_width!(L17, 17);
        assert_fixed_width!(L18, 18);
        assert_fixed_width!(L19, 19);
        assert_fixed_width!(L20, 20);
        assert_fixed_width!(L21, 21);
        assert_fixed_width!(L22, 22);
        assert_fixed_width!(L23, 23);
        assert_fixed_width!(L24, 24);
        assert_fixed_width!(L25, 25);
        assert_fixed_width!(L26, 26);
        assert_fixed_width!(L27, 27);
        assert_fixed_width!(L28, 28);
        assert_fixed_width!(L29, 29);
        assert_fixed_width!(L30, 30);
        assert_fixed_width!(L31, 31);
        assert_fixed_width!(L32, 32);
    }

    #[test]
    fn generated_stage_installation_handles_empty_contiguous_and_sparse_maps() {
        let mut destination = [10.0, 11.0, 12.0, 13.0, 14.0];
        install_generated_stage_values(&mut destination, &[], &[]);
        assert_eq!(destination, [10.0, 11.0, 12.0, 13.0, 14.0]);

        install_generated_stage_values(&mut destination, &[1.0, 2.0], &[1, 2]);
        assert_eq!(destination, [10.0, 1.0, 2.0, 13.0, 14.0]);

        install_generated_stage_values(&mut destination, &[7.0, 8.0], &[4, 0]);
        assert_eq!(destination, [8.0, 1.0, 2.0, 13.0, 7.0]);
    }

    #[test]
    #[should_panic(expected = "generated stage value/slot count mismatch")]
    fn generated_stage_installation_rejects_length_mismatch() {
        install_generated_stage_values(&mut [0.0; 2], &[1.0], &[]);
    }

    #[test]
    #[should_panic(expected = "generated stage slot is outside canonical storage")]
    fn generated_stage_installation_rejects_out_of_range_slots() {
        install_generated_stage_values(&mut [0.0; 2], &[1.0], &[2]);
    }

    #[test]
    fn generated_parameter_aliases_are_ordered_and_validated_after_each_copy() {
        fn validate(index: usize, value: f64) -> Result<(), String> {
            if matches!(index, 2 | 3) && value == 3.5 {
                Ok(())
            } else {
                Err(format!("unexpected alias {index}={value}"))
            }
        }

        let mut values = [3.5, 0.0, 0.0, 0.0];
        install_generated_parameter_aliases(&mut values, &[(2, 0), (3, 2)], validate)
            .expect("alias chain");
        assert_eq!(values, [3.5, 0.0, 3.5, 3.5]);
    }

    #[test]
    fn generated_parameter_aliases_stop_before_later_operations_on_validation_error() {
        fn reject_first(_: usize, _: f64) -> Result<(), String> {
            Err("invalid default".to_string())
        }

        let mut values = [2.0, 0.0, 0.0];
        assert_eq!(
            install_generated_parameter_aliases(&mut values, &[(1, 0), (2, 1)], reject_first),
            Err("invalid default".to_string())
        );
        assert_eq!(values, [2.0, 2.0, 0.0]);
    }

    #[test]
    #[should_panic(expected = "generated parameter-alias source is outside parameter storage")]
    fn generated_parameter_aliases_reject_out_of_range_sources() {
        install_generated_parameter_aliases(&mut [0.0; 1], &[(0, 1)], |_, _| Ok(()))
            .expect("validation does not fail");
    }

    #[test]
    fn generated_parameter_lookup_is_sorted_deterministic_and_compact() {
        let names = ["alpha", "alpha", "beta", "zeta"];
        let indices = [7, 9, 2, 4];
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "alpha"),
            Some(7)
        );
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "beta"),
            Some(2)
        );
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "gamma"),
            None
        );
        assert_eq!(
            find_generated_parameter_index(&names, &indices, "Alpha"),
            None
        );
        assert_eq!(core::mem::size_of_val(&indices[0]), 2);
    }

    #[test]
    #[should_panic(expected = "generated parameter lookup table length mismatch")]
    fn generated_parameter_lookup_rejects_mismatched_tables() {
        find_generated_parameter_index(&["alpha"], &[], "alpha");
    }

    #[test]
    fn generated_parameter_bound_indices_preserve_sentinel_values_and_diagnostics() {
        const POOL: [GeneratedParameterBound; 2] = [
            GeneratedParameterBound {
                value: -1.0,
                label: "-1.0",
            },
            GeneratedParameterBound {
                value: 2.0,
                label: "2.0",
            },
        ];

        assert_eq!(GENERATED_PARAMETER_BOUND_NONE, 0);
        assert!(resolve_generated_parameter_bound(&POOL, 0).is_none());
        let resolved = resolve_generated_parameter_bound(&POOL, 2).expect("bound exists");
        assert_eq!(resolved.value.to_bits(), 2.0f64.to_bits());
        assert_eq!(resolved.label, "2.0");
        assert_eq!(core::mem::size_of_val(&2u16), core::mem::size_of::<u16>());

        assert_eq!(
            validate_generated_parameter_bound_indices("gain", -2.0, 0, &POOL, 1, 0, &[]),
            Err("parameter 'gain' must be >= -1.0, got -2".to_string())
        );
        assert_eq!(
            validate_generated_parameter_bound_indices("gain", 2.0, 0, &POOL, 0, 0, &[2]),
            Err("parameter 'gain' must not equal 2.0, got 2".to_string())
        );
    }

    #[test]
    #[should_panic(expected = "generated parameter-bound index is outside its pool")]
    fn generated_parameter_bound_indices_reject_out_of_range_values() {
        resolve_generated_parameter_bound(&[], 1);
    }

    #[test]
    #[should_panic(expected = "generated parameter exclusion uses the absence sentinel")]
    fn generated_parameter_bound_indices_reject_none_as_an_exclusion() {
        validate_generated_parameter_bound_indices("gain", 0.0, 0, &[], 0, 0, &[0])
            .expect("validation should panic before returning");
    }
}

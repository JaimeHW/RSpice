//! The starter model library compiled into the binary.
//!
//! These are the `builtin` pack of the repository model tree
//! (`models/spice/builtin/lib`), the only pack RSpice authors itself and so the
//! only one whose licensing is unconditionally clear. They are embedded rather
//! than loaded from disk so the starter set exists on every platform, including
//! the browser build, which has no filesystem to read the vendored packs from.
//!
//! The text lives here, in a leaf module, because three subsystems need it —
//! library loading, netlist source mapping and engine model fallback — and each
//! previously carried its own `include_str!` at its own relative depth. One
//! owner means a file rename cannot leave two of them pointing at the old path,
//! and the generator (`tools/models/build_builtin.py`) has a single target.

/// Diode, rectifier, zener and LED cards.
pub(crate) const DIODE_LIB: &str = include_str!("../../../models/spice/builtin/lib/diode.lib");

/// Bipolar junction transistor cards.
pub(crate) const BJT_LIB: &str = include_str!("../../../models/spice/builtin/lib/bjt.lib");

/// Junction FET cards.
pub(crate) const JFET_LIB: &str = include_str!("../../../models/spice/builtin/lib/jfet.lib");

/// MOSFET cards.
pub(crate) const MOSFET_LIB: &str = include_str!("../../../models/spice/builtin/lib/mosfet.lib");

/// Operational amplifier macromodels.
pub(crate) const OPAMP_LIB: &str = include_str!("../../../models/spice/builtin/lib/opamp.lib");

/// Timer, regulator and other integrated-circuit macromodels.
pub(crate) const IC_LIB: &str = include_str!("../../../models/spice/builtin/lib/ic.lib");

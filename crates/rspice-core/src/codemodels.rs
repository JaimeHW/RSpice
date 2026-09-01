//! The XSPICE code models compiled into this build.
//!
//! Two catalogues of names: the code models themselves, and the ngspice `.cm`
//! bundles they came from. Both answer the same question -- *does this build
//! already know that name?* -- and both are asked from two directions.
//! `xspice` asks while registering and instantiating a model. The parser asks
//! while reading a deck, to tell an `A` card referencing a built-in from one
//! referencing a subcircuit, and to recognize a `.codemodel` directive as a
//! no-op rather than a missing file.
//!
//! The tables lived on `CodeModelRegistry`, which put a name list eight layers
//! up from the parser that classifies against it -- and none of these four
//! functions ever touched a registry instance; they were associated purely for
//! grouping. A catalogue of names is data, so it sits at the bottom and both
//! directions read down into it.

/// Every XSPICE code model registered by `CodeModelRegistry::with_builtins`.
pub(crate) const BUILTIN_MODEL_NAMES: &[&str] = &[
    "adc_bridge",
    "astate",
    "aswitch",
    "bidi_bridge",
    "capacitor",
    "capacitoric",
    "climit",
    "cmeter",
    "core",
    "cpline",
    "cpmlin",
    "d_and",
    "d_buffer",
    "d_cosim",
    "d_dff",
    "d_dlatch",
    "d_dt",
    "d_fdiv",
    "d_genlut",
    "d_inverter",
    "d_jkff",
    "d_lut",
    "d_nand",
    "d_nor",
    "d_open_c",
    "d_open_e",
    "d_or",
    "d_osc",
    "d_process",
    "d_pulldown",
    "d_pullup",
    "d_pwm",
    "d_ram",
    "d_source",
    "d_srff",
    "d_srlatch",
    "d_state",
    "d_tff",
    "d_to_real",
    "d_tristate",
    "d_xnor",
    "d_xor",
    "dac_bridge",
    "delay",
    "differentiator",
    "divide",
    "divider",
    "file_source",
    "filesource",
    "gain",
    "hyst",
    "icm_spice2poly",
    "ilimit",
    "inductor",
    "inductoric",
    "int",
    "integrator",
    "lcouple",
    "limit",
    "lmeter",
    "memristor",
    "mlin",
    "msopen",
    "mult",
    "multi_input_pwl",
    "nco",
    "oneshot",
    "potentiometer",
    "print_param_types",
    "pspice_d_stim",
    "pswitch",
    "pwl",
    "pwlts",
    "r_to_v",
    "real_delay",
    "real_gain",
    "real_to_v",
    "s_h",
    "s_xfer",
    "seegen",
    "seegenerator",
    "sidiode",
    "sine",
    "slew",
    "spice2poly",
    "square",
    "summer",
    "table2d",
    "table3d",
    "tline",
    "triangle",
    // RSpice's own: the analog-to-real observer ngspice has no counterpart for.
    "v_to_real",
    "xfer",
    "xyce_d_and",
    "xyce_d_add",
    "xyce_d_dff",
    "xyce_d_dlatch",
    "xyce_d_jkff",
    "xyce_d_buffer",
    "xyce_d_inverter",
    "xyce_d_nand",
    "xyce_d_nor",
    "xyce_d_or",
    "xyce_d_tff",
    "xyce_d_xnor",
    "xyce_d_xor",
    "xyce_legacy_d_and",
    "xyce_legacy_d_dff",
    "xyce_legacy_d_inverter",
    "xyce_legacy_d_nand",
    "xyce_legacy_d_nor",
    "xyce_legacy_d_or",
    "xyce_legacy_d_xnor",
    "xyce_legacy_d_xor",
    "zener",
];

/// The ngspice-46 `.cm` bundles whose models are compiled in.
pub(crate) const BUILTIN_CODEMODEL_LIBRARY_NAMES: &[&str] = &[
    "analog.cm",
    "digital.cm",
    "spice2poly.cm",
    "table.cm",
    "tlines.cm",
    "xtradev.cm",
    "xtraevt.cm",
];

/// Check whether a name belongs to a built-in code model.
pub(crate) fn is_builtin_model_name(name: &str) -> bool {
    BUILTIN_MODEL_NAMES
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(name))
}

/// Check whether an ngspice `.codemodel` path names a built-in bundle.
///
/// RSpice compiles the ngspice-46 XSPICE models into the binary instead of
/// loading their generated `.cm` shared libraries. Netlists may still carry
/// `codemodel .../analog.cm` style directives from ngspice startup files;
/// those are compatibility no-ops when the basename is one of the official
/// bundles listed above.
pub(crate) fn is_builtin_codemodel_library_path(path: &str) -> bool {
    let trimmed = path
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim_matches(|ch| matches!(ch, ';' | ','));
    let normalized = trimmed.replace('\\', "/");
    let Some(name) = normalized.rsplit('/').next() else {
        return false;
    };
    BUILTIN_CODEMODEL_LIBRARY_NAMES
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(name))
}

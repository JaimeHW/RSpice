//! Shared SPICE ground-name handling.

/// Return true when `name` is a SPICE ground alias that resolves to node 0.
pub(crate) fn is_spice_ground_name(name: &str) -> bool {
    let trimmed = name.trim();
    trimmed == "0"
        || trimmed.eq_ignore_ascii_case("gnd")
        || trimmed.eq_ignore_ascii_case("ground")
        || trimmed.eq_ignore_ascii_case("gnd!")
}

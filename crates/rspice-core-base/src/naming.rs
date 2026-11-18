//! SPICE naming rules.
//!
//! Rules about what a name *means* in a deck, as opposed to how it is spelled
//! in any particular vendor's dialect. Everything here is a leaf: it depends
//! on nothing else in the crate, because everything else depends on it.
//!
//! These predicates used to live in `compat`, beside the LTspice RAW reader.
//! That put a module documented as "compatibility readers" underneath the
//! solver, the device models, the circuit store and the parser — eight
//! modules taking a dependency on a foreign-format reader to ask whether a
//! node is ground. The rule is not a compatibility shim; it is part of what
//! SPICE means.

/// Return true when `name` is the canonical SPICE ground node.
///
/// Dialect-specific aliases are normalized during netlist elaboration.
/// Keeping this low-level predicate literal prevents an ordinary node named
/// `GND` from silently collapsing to zero when Xyce `REPLACEGROUND` is
/// disabled.
#[doc(hidden)]
pub fn is_spice_ground_name(name: &str) -> bool {
    name.trim() == "0"
}

/// The file a path written in a deck names on this host: an `.include`, a
/// PWL or code-model data file, a Verilog source, an HDL run directory.
///
/// Both `/` and `\` separate directories whatever the host, so a deck written
/// on Windows names the same files on macOS and Linux. A path absolute on
/// this host keeps its root; any other is relative, with empty and `.`
/// components dropped. Surrounding quotes are not part of the path.
pub fn source_path_literal_to_host_path(path: &str) -> std::path::PathBuf {
    let path = path.trim().trim_matches('"').trim_matches('\'');
    let native = std::path::Path::new(path);
    if native.is_absolute() {
        native.to_path_buf()
    } else {
        spice_relative_path(path)
    }
}

/// A relative deck path's components, split at either separator.
#[doc(hidden)]
pub fn spice_relative_path(path: &str) -> std::path::PathBuf {
    path.split(['/', '\\'])
        .filter(|component| !component.is_empty() && *component != ".")
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn a_windows_deck_path_names_the_same_file_on_every_host() {
        for (written, components) in [
            (r"runs\x", ["runs", "x"].as_slice()),
            (r"runs\", ["runs"].as_slice()),
            (r"..\stim\new.csv", ["..", "stim", "new.csv"].as_slice()),
            (r".\lib\nmos.lib", ["lib", "nmos.lib"].as_slice()),
            ("runs/x", ["runs", "x"].as_slice()),
            (r#""runs\x""#, ["runs", "x"].as_slice()),
        ] {
            assert_eq!(
                source_path_literal_to_host_path(written),
                components.iter().collect::<PathBuf>(),
                "{written}"
            );
        }
    }

    #[test]
    fn a_drive_or_share_path_is_absolute_where_the_host_says_so() {
        for written in [r"C:\runs\x", r"\\server\share\runs\x", r"C:\"] {
            let host = source_path_literal_to_host_path(written);
            if cfg!(windows) {
                assert_eq!(host, Path::new(written), "{written}");
                assert!(host.is_absolute(), "{written}");
            } else {
                // Elsewhere a Windows root is no root: the path is relative,
                // split at its backslashes as `.include` splits it.
                assert!(host.is_relative(), "{written}");
                assert!(
                    host.components()
                        .all(|part| !part.as_os_str().to_string_lossy().contains('\\')),
                    "{written} -> {}",
                    host.display()
                );
            }
        }
    }

    #[test]
    fn a_host_absolute_path_keeps_its_root() {
        let written = if cfg!(windows) {
            r"C:\runs\x"
        } else {
            "/runs/x"
        };
        assert_eq!(
            source_path_literal_to_host_path(written),
            Path::new(written)
        );
    }

    #[test]
    fn canonical_ground_is_node_zero() {
        assert!(is_spice_ground_name("0"));
        // Surrounding whitespace is incidental to tokenization, not part of
        // the name.
        assert!(is_spice_ground_name(" 0 "));
    }

    #[test]
    fn ground_aliases_are_not_canonical_ground() {
        // The whole point of keeping this literal: these are ordinary nodes
        // unless elaboration has been told to fold them, and folding them
        // here would do it even when Xyce REPLACEGROUND is off.
        for alias in ["GND", "gnd", "GND!", "0.0", "00", "ground"] {
            assert!(
                !is_spice_ground_name(alias),
                "{alias} must not be treated as canonical ground"
            );
        }
    }
}

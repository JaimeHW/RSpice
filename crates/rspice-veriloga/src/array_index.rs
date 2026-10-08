//! The shared runtime array-index contract.
pub(crate) use rspice_veriloga_runtime::array_index::*;

/// Format the source identity of one validated physical array slot.
pub(crate) fn element_name(name: &str, layout: &UnpackedArrayLayout, offset: usize) -> String {
    use std::fmt::Write;
    let mut name = name.to_owned();
    for coordinate in layout.indices(offset).expect("validated array cell") {
        write!(name, "[{coordinate}]").expect("string write");
    }
    name
}

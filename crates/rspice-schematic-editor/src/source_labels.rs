//! The canvas's cache of the value text an instance shows.
//!
//! The text itself is `rspice_design::schematic::value_label`, which the SVG
//! export and the printed page read too.

use rspice_design::schematic::{component::Component, value_label::component_value_label};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

thread_local! {
    /// Formatted value labels per component id. The label is a pure
    /// function of (kind, value, params), which change only on property
    /// edits — never during pan/zoom/drag — and painting is single-threaded.
    static LABEL_CACHE: RefCell<HashMap<u64, (u64, Rc<str>)>> =
        RefCell::new(HashMap::new());
}

/// Backstop far above any real schematic's component count; entries for
/// deleted components linger until then, which is harmless.
const LABEL_CACHE_CAP: usize = 16_384;

/// Cached [`component_value_label`] for the per-frame paint path: source
/// labels parse a params string and build a multi-line String, which must
/// not repeat per component per frame.
pub fn component_value_label_cached(component: &Component) -> Rc<str> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    component.kind.hash(&mut hasher);
    component.value.hash(&mut hasher);
    component.params.hash(&mut hasher);
    let content = hasher.finish();

    LABEL_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((hash, label)) = cache.get(&component.id)
            && *hash == content
        {
            return Rc::clone(label);
        }
        if cache.len() >= LABEL_CACHE_CAP {
            cache.clear();
        }
        let label: Rc<str> = Rc::from(component_value_label(component).as_ref());
        cache.insert(component.id, (content, Rc::clone(&label)));
        label
    })
}

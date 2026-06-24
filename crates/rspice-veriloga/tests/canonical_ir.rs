use serde::{Deserialize, Serialize};
use std::hash::Hash;

use rspice_veriloga::canonical_ir::{ModuleId, ParamId, PortId, SourceId};

#[test]
fn typed_ids_are_dense_copyable_and_displayable() {
    let module = ModuleId::new(7);
    let source = SourceId::new(3);
    let port = PortId::new(2);
    let param = ParamId::new(5);

    assert_eq!(module.index(), 7);
    assert_eq!(source.index(), 3);
    assert_eq!(port.index(), 2);
    assert_eq!(param.index(), 5);
    assert_eq!(module.to_string(), "ModuleId(7)");
    assert_eq!(port.next(), PortId::new(3));
}

#[test]
fn typed_ids_expose_expected_trait_surface() {
    fn assert_id_traits<T>()
    where
        T: Copy + Ord + Hash + Serialize + for<'de> Deserialize<'de>,
    {
    }

    assert_id_traits::<ModuleId>();
}

#[test]
fn typed_ids_convert_to_and_from_usize() {
    let id = ParamId::from(42usize);

    assert_eq!(usize::from(id), 42);
}

#[test]
#[should_panic(expected = "canonical IR id overflow")]
fn next_panics_on_overflow() {
    let _ = ModuleId::new(u32::MAX).next();
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic(expected = "canonical IR id index exceeds u32::MAX")]
fn from_usize_panics_when_index_exceeds_u32_max() {
    let _ = ModuleId::from(u32::MAX as usize + 1);
}

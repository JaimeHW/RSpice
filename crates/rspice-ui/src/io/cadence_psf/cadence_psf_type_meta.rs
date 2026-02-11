use std::collections::{HashMap, HashSet};

use super::{CadencePsfError, DataType, TypeDecl, TypeKind};

#[derive(Debug, Default)]
pub(super) struct TypeMetaCache {
    contains_array: HashMap<u32, bool>,
    min_encoded_size: HashMap<u32, usize>,
}

impl TypeMetaCache {
    pub(super) fn contains_array(
        &mut self,
        type_id: u32,
        types: &HashMap<u32, TypeDecl>,
    ) -> Result<bool, CadencePsfError> {
        let mut visiting = HashSet::new();
        self.contains_array_inner(type_id, types, &mut visiting)
    }

    fn contains_array_inner(
        &mut self,
        type_id: u32,
        types: &HashMap<u32, TypeDecl>,
        visiting: &mut HashSet<u32>,
    ) -> Result<bool, CadencePsfError> {
        if let Some(cached) = self.contains_array.get(&type_id) {
            return Ok(*cached);
        }
        if !visiting.insert(type_id) {
            return Err(CadencePsfError::new(format!(
                "cyclic type definition detected while checking type {}",
                type_id
            )));
        }

        let decl = types
            .get(&type_id)
            .ok_or_else(|| CadencePsfError::new(format!("missing type declaration {}", type_id)))?;
        let has_array = match &decl.kind {
            TypeKind::Primitive(_) => false,
            TypeKind::Array { .. } => true,
            TypeKind::Struct { members } => {
                let mut nested_has_array = false;
                for member_id in members {
                    if self.contains_array_inner(*member_id, types, visiting)? {
                        nested_has_array = true;
                        break;
                    }
                }
                nested_has_array
            }
        };

        visiting.remove(&type_id);
        self.contains_array.insert(type_id, has_array);
        Ok(has_array)
    }

    pub(super) fn min_encoded_size(
        &mut self,
        type_id: u32,
        types: &HashMap<u32, TypeDecl>,
    ) -> Result<usize, CadencePsfError> {
        let mut visiting = HashSet::new();
        self.min_encoded_size_inner(type_id, types, &mut visiting)
    }

    fn min_encoded_size_inner(
        &mut self,
        type_id: u32,
        types: &HashMap<u32, TypeDecl>,
        visiting: &mut HashSet<u32>,
    ) -> Result<usize, CadencePsfError> {
        if let Some(cached) = self.min_encoded_size.get(&type_id) {
            return Ok(*cached);
        }
        if !visiting.insert(type_id) {
            return Err(CadencePsfError::new(format!(
                "cyclic type definition detected while sizing type {}",
                type_id
            )));
        }

        let decl = types
            .get(&type_id)
            .ok_or_else(|| CadencePsfError::new(format!("missing type declaration {}", type_id)))?;
        let min_size = match &decl.kind {
            TypeKind::Primitive(dtype) => match dtype {
                DataType::Int8 => 4,
                DataType::Int32 => 4,
                DataType::Real => 8,
                DataType::Complex => 16,
                DataType::String => 4,
                // Unknown scalar handling reads one aligned 32-bit word.
                DataType::Other(_) => 4,
                // These values are only valid as composite type declarations.
                DataType::Array | DataType::Struct => {
                    return Err(CadencePsfError::new(format!(
                        "invalid primitive descriptor {:?} on type {}",
                        dtype, type_id
                    )));
                }
            },
            // Variable-length arrays can legally encode zero elements.
            TypeKind::Array { .. } => 4,
            TypeKind::Struct { members } => {
                let mut total = 0usize;
                for member_id in members {
                    let member_size = self.min_encoded_size_inner(*member_id, types, visiting)?;
                    total = total.checked_add(member_size).ok_or_else(|| {
                        CadencePsfError::new(format!(
                            "minimum encoded-size overflow while sizing type {}",
                            type_id
                        ))
                    })?;
                }
                total
            }
        };

        visiting.remove(&type_id);
        self.min_encoded_size.insert(type_id, min_size);
        Ok(min_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_type(name: &str, kind: TypeKind) -> TypeDecl {
        TypeDecl {
            name: name.to_string(),
            kind,
        }
    }

    #[test]
    fn test_contains_array_for_nested_struct() {
        let mut types = HashMap::new();
        types.insert(
            1,
            make_type(
                "root",
                TypeKind::Struct {
                    members: vec![2, 3],
                },
            ),
        );
        types.insert(2, make_type("gain", TypeKind::Primitive(DataType::Real)));
        types.insert(
            3,
            make_type(
                "taps",
                TypeKind::Array {
                    element_type_raw: DataType::Real.to_u32(),
                },
            ),
        );

        let mut cache = TypeMetaCache::default();
        assert!(cache
            .contains_array(1, &types)
            .expect("contains-array lookup should succeed"));
        assert!(!cache
            .contains_array(2, &types)
            .expect("contains-array lookup should succeed"));
        assert!(cache
            .contains_array(3, &types)
            .expect("contains-array lookup should succeed"));
    }

    #[test]
    fn test_min_encoded_size_for_struct_and_array() {
        let mut types = HashMap::new();
        types.insert(
            1,
            make_type(
                "root",
                TypeKind::Struct {
                    members: vec![2, 3, 4],
                },
            ),
        );
        types.insert(2, make_type("real", TypeKind::Primitive(DataType::Real)));
        types.insert(
            3,
            make_type("complex", TypeKind::Primitive(DataType::Complex)),
        );
        types.insert(
            4,
            make_type(
                "arr",
                TypeKind::Array {
                    element_type_raw: DataType::Int32.to_u32(),
                },
            ),
        );

        let mut cache = TypeMetaCache::default();
        // real (8) + complex (16) + array count prefix (4)
        assert_eq!(
            cache
                .min_encoded_size(1, &types)
                .expect("size lookup should succeed"),
            28
        );
        assert_eq!(
            cache
                .min_encoded_size(4, &types)
                .expect("size lookup should succeed"),
            4
        );
    }

    #[test]
    fn test_contains_array_rejects_cycles() {
        let mut types = HashMap::new();
        types.insert(1, make_type("loop", TypeKind::Struct { members: vec![1] }));

        let mut cache = TypeMetaCache::default();
        let err = cache
            .contains_array(1, &types)
            .expect_err("cyclic types should fail");
        assert!(err.to_string().contains("cyclic"));
    }

    #[test]
    fn test_min_encoded_size_rejects_cycles() {
        let mut types = HashMap::new();
        types.insert(1, make_type("loop", TypeKind::Struct { members: vec![1] }));

        let mut cache = TypeMetaCache::default();
        let err = cache
            .min_encoded_size(1, &types)
            .expect_err("cyclic types should fail");
        assert!(err.to_string().contains("cyclic"));
    }
}

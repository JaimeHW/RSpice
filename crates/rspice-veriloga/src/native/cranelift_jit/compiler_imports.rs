use super::*;

impl JitCompiler {
    /// Import standard math functions
    fn import_math_functions(
        &self,
        module: &mut JITModule,
    ) -> JitResult<HashMap<&'static str, FuncId>> {
        let mut funcs = HashMap::new();
        let ptr_type = self.isa.pointer_type();
        let _ = ptr_type; // Suppress warning

        // Define signatures for math functions
        let math_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F64));
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };

        let math2_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(types::F64));
            sig.params.push(AbiParam::new(types::F64));
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };

        // Import single-arg math functions
        for name in [
            "exp", "log", "log10", "sqrt", "sin", "cos", "tan", "sinh", "cosh", "tanh", "asin",
            "acos", "atan", "floor", "ceil", "fabs",
        ] {
            let id = module
                .declare_function(name, Linkage::Import, &math_sig)
                .map_err(|e| JitError::Module(e.to_string()))?;
            funcs.insert(name, id);
        }

        // Import two-arg math functions
        for name in ["pow", "atan2", "fmin", "fmax"] {
            let id = module
                .declare_function(name, Linkage::Import, &math2_sig)
                .map_err(|e| JitError::Module(e.to_string()))?;
            funcs.insert(name, id);
        }

        // Import rspice helper functions for $table_model
        // Signature: fn(tables_ptr, tables_len, table_id, input) -> f64
        let table_lookup_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(ptr_type)); // tables_ptr
            sig.params.push(AbiParam::new(ptr_type)); // tables_len (usize)
            sig.params.push(AbiParam::new(ptr_type)); // table_id (usize)
            sig.params.push(AbiParam::new(types::F64)); // input
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };
        let id = module
            .declare_function("rspice_table_lookup", Linkage::Import, &table_lookup_sig)
            .map_err(|e| JitError::Module(e.to_string()))?;
        funcs.insert("rspice_table_lookup", id);

        // Import rspice helper functions for $limit
        // Signature: fn(state_prev, state_idx, new_value, step_limit) -> f64
        let limit_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(ptr_type)); // state_prev
            sig.params.push(AbiParam::new(ptr_type)); // state_idx (usize)
            sig.params.push(AbiParam::new(types::F64)); // new_value
            sig.params.push(AbiParam::new(types::F64)); // step_limit
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };
        let id = module
            .declare_function("rspice_limit", Linkage::Import, &limit_sig)
            .map_err(|e| JitError::Module(e.to_string()))?;
        funcs.insert("rspice_limit", id);

        // Import rspice helper functions for Laplace state-space filters
        // Signature: fn(filters_ptr, filters_len, filter_id, input, timestep) -> f64
        let laplace_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(ptr_type)); // filters_ptr
            sig.params.push(AbiParam::new(ptr_type)); // filters_len (usize)
            sig.params.push(AbiParam::new(ptr_type)); // filter_id (usize)
            sig.params.push(AbiParam::new(types::F64)); // input
            sig.params.push(AbiParam::new(types::F64)); // timestep
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };
        let id = module
            .declare_function("rspice_laplace_step", Linkage::Import, &laplace_sig)
            .map_err(|e| JitError::Module(e.to_string()))?;
        funcs.insert("rspice_laplace_step", id);

        // Import rspice helper functions for PushCurrent lookup
        // Signature: fn(branch_ptr, branch_len, currents_ptr, currents_len, num_terminals, pos, neg) -> f64
        let current_lookup_sig = {
            let mut sig = module.make_signature();
            sig.params.push(AbiParam::new(ptr_type)); // branch_ptr
            sig.params.push(AbiParam::new(ptr_type)); // branch_len
            sig.params.push(AbiParam::new(ptr_type)); // currents_ptr
            sig.params.push(AbiParam::new(ptr_type)); // currents_len
            sig.params.push(AbiParam::new(ptr_type)); // num_terminals
            sig.params.push(AbiParam::new(ptr_type)); // pos
            sig.params.push(AbiParam::new(ptr_type)); // neg
            sig.returns.push(AbiParam::new(types::F64));
            sig
        };
        let id = module
            .declare_function(
                "rspice_current_lookup",
                Linkage::Import,
                &current_lookup_sig,
            )
            .map_err(|e| JitError::Module(e.to_string()))?;
        funcs.insert("rspice_current_lookup", id);

        // Import rspice_limexp for limited exponential (prevents overflow)
        // Signature: fn(value: f64) -> f64
        let id = module
            .declare_function("rspice_limexp", Linkage::Import, &math_sig)
            .map_err(|e| JitError::Module(e.to_string()))?;
        funcs.insert("rspice_limexp", id);

        Ok(funcs)
    }

}

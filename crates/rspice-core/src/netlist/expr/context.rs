use super::*;

//=============================================================================
// Expression Evaluator
//=============================================================================

/// User-defined function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    /// Function name
    pub name: String,
    /// Argument names
    pub args: Vec<String>,
    /// Function body expression string
    pub body: String,
}

impl FunctionDef {
    /// Create a new function definition
    pub fn new(name: &str, args: Vec<String>, body: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            args: args.into_iter().map(|a| a.to_uppercase()).collect(),
            body: body.to_string(),
        }
    }
}

/// Context for parameter substitution during evaluation
#[derive(Debug, Clone, Default)]
pub struct ParamContext {
    params: HashMap<String, Value>,
    /// User-defined functions (.FUNC)
    functions: HashMap<String, FunctionDef>,
}

impl ParamContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a parameter value
    pub fn set(&mut self, name: &str, value: Value) {
        self.params.insert(name.to_uppercase(), value);
    }

    /// Get a parameter value
    pub fn get(&self, name: &str) -> Option<Value> {
        self.params.get(&name.to_uppercase()).copied()
    }

    /// Merge another context into this one
    pub fn merge(&mut self, other: &ParamContext) {
        for (k, v) in &other.params {
            self.params.insert(k.clone(), *v);
        }
        for (k, v) in &other.functions {
            self.functions.insert(k.clone(), v.clone());
        }
    }

    /// Get all parameters as a vector of (name, value) tuples
    pub fn all_params(&self) -> Vec<(String, Value)> {
        self.params.iter().map(|(k, v)| (k.clone(), *v)).collect()
    }

    /// Define a user function
    ///
    /// # Example
    /// ```ignore
    /// ctx.define_function("SQUARE", vec!["X".to_string()], "X*X");
    /// ctx.define_function("RPAR", vec!["R1".to_string(), "R2".to_string()], "R1*R2/(R1+R2)");
    /// ```
    pub fn define_function(&mut self, name: &str, args: Vec<String>, body: &str) {
        let func = FunctionDef::new(name, args, body);
        self.functions.insert(func.name.clone(), func);
    }

    /// Check if a user function is defined
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(&name.to_uppercase())
    }

    /// Get a user function definition
    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(&name.to_uppercase())
    }

    /// Call a user-defined function with the given argument values
    ///
    /// Creates a temporary context with the arguments bound to their values
    /// and evaluates the function body.
    pub fn call_function(&self, name: &str, arg_values: &[Value]) -> Result<Value, ExprError> {
        let func = self
            .get_function(name)
            .ok_or_else(|| ExprError::UnknownFunction(name.to_string()))?;

        if arg_values.len() != func.args.len() {
            return Err(ExprError::WrongArgCount(name.to_string()));
        }

        // Create temporary context with arguments bound
        let mut temp_ctx = self.clone();
        for (arg_name, &arg_value) in func.args.iter().zip(arg_values.iter()) {
            temp_ctx.set(arg_name, arg_value);
        }

        // Parse and evaluate the function body
        let expr = parse_expression(&func.body)?;
        evaluate(&expr, &temp_ctx)
    }
}

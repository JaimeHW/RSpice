//! Symbol-table infrastructure for Verilog-A semantic analysis.

use crate::ast::PortDirection;
use crate::source::Span;
use crate::types::{ParameterRange as TypedParameterRange, ValueType};
use smol_str::SmolStr;
use std::collections::HashMap;

/// Hierarchical symbol table with nested scopes
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Stack of scopes (index 0 = global/module scope)
    scopes: Vec<Scope>,
    /// Current scope index
    current: usize,
}

/// A single scope containing symbols
#[derive(Debug, Clone)]
struct Scope {
    /// Parent scope index (None for module scope)
    parent: Option<usize>,
    /// Symbols in this scope
    symbols: HashMap<SmolStr, Symbol>,
}

/// A symbol in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: SmolStr,
    pub kind: SymbolKind,
    pub value_type: ValueType,
    pub span: Span,
    pub attrs: SymbolAttrs,
}

/// Symbol kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Port,
    Parameter,
    /// aliasparam name: reserves the identifier so nothing else may
    /// reuse it; not referenceable in the module body
    ParamAlias,
    Variable,
    Node,
    Branch,
    LoopVar,
}

/// Additional symbol attributes
#[derive(Debug, Clone, Default)]
pub struct SymbolAttrs {
    pub direction: Option<PortDirection>,
    pub discipline: Option<SmolStr>,
    pub range: Option<TypedParameterRange>,
    pub is_state: bool,
    pub used: bool,
    /// Whether this is an internal node (not in port list)
    pub is_internal: bool,
    /// Whether this node is declared ground
    pub is_ground: bool,
    /// Index in internal nodes array (for VM access)
    pub internal_node_index: Option<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope {
                parent: None,
                symbols: HashMap::new(),
            }],
            current: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        let parent = self.current;
        self.scopes.push(Scope {
            parent: Some(parent),
            symbols: HashMap::new(),
        });
        self.current = self.scopes.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current].parent {
            self.current = parent;
        }
    }

    pub fn define(&mut self, symbol: Symbol) -> Result<(), Box<Symbol>> {
        let scope = &mut self.scopes[self.current];
        if scope.symbols.contains_key(&symbol.name) {
            return Err(Box::new(scope.symbols.get(&symbol.name).unwrap().clone()));
        }
        scope.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut scope_idx = Some(self.current);
        while let Some(idx) = scope_idx {
            let scope = &self.scopes[idx];
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
            scope_idx = scope.parent;
        }
        None
    }

    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.scopes[self.current].symbols.get(name)
    }

    pub fn mark_used(&mut self, name: &str) {
        let mut scope_idx = Some(self.current);
        while let Some(idx) = scope_idx {
            let scope = &mut self.scopes[idx];
            if let Some(sym) = scope.symbols.get_mut(name) {
                sym.attrs.used = true;
                return;
            }
            scope_idx = scope.parent;
        }
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut idx = self.current;
        while let Some(parent) = self.scopes[idx].parent {
            depth += 1;
            idx = parent;
        }
        depth
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

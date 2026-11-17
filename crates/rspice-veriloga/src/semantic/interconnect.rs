//! Port nets that declare nothing and only join discrete ports below them.
//!
//! Verilog-AMS LRM 2.4 Annex F.2.1 resolves the discipline of an undeclared
//! net bottom-up, from the ports it connects in the instances below it. A
//! structural module such as Figure 7-3's `digital_blk` declares nothing for
//! its port and only joins three discrete ports with it, so the port is a
//! discrete net: a wire of IEEE 1364-2005 section 12.3.3, not an analog node
//! its children would each need a connect module to reach.

use crate::ast::{Connection, Expression, Item, Module, SourceFile};
use crate::disciplines::{DisciplineDb, Domain};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

/// The deepest instance chain followed; a deeper one, or a recursive one,
/// leaves the port undetermined, as an undeclared port has always been.
const MAX_DEPTH: usize = 256;

/// For each module, its ports that declare no discipline and no storage, are
/// read by no analog block, and join only discrete ports of the instances
/// below them (at least one).
pub(super) fn implicitly_discrete_ports(
    source: &SourceFile,
    db: &DisciplineDb,
) -> HashMap<SmolStr, HashSet<SmolStr>> {
    let modules: HashMap<&str, &Module> = source
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Module(module) => Some((module.name.as_str(), module)),
            _ => None,
        })
        .collect();
    let mut resolver = Resolver {
        modules: &modules,
        db,
        memo: HashMap::new(),
    };
    let mut found: HashMap<SmolStr, HashSet<SmolStr>> = HashMap::new();
    for module in modules.values() {
        for port in &module.ports {
            if resolver.domain(module, &port.name, 0) == Some(Domain::Discrete)
                && declared_domain(module, &port.name, db).is_none()
            {
                found
                    .entry(module.name.clone())
                    .or_default()
                    .insert(port.name.clone());
            }
        }
    }
    found
}

/// The domain a module's own declarations give `name`: its discipline, or a
/// net or variable declaration of the digital domain.
fn declared_domain(module: &Module, name: &str, db: &DisciplineDb) -> Option<Domain> {
    let discipline_domain = |discipline: &SmolStr| db.get_discipline(discipline).map(|d| d.domain);
    if let Some(domain) = module
        .port_declarations
        .iter()
        .filter(|declaration| declaration.names.iter().any(|port| port == name))
        .find_map(|declaration| declaration.discipline.as_ref().and_then(discipline_domain))
    {
        return Some(domain);
    }
    if let Some(domain) = module
        .nets
        .iter()
        .filter(|net| net.names.iter().any(|net| net == name))
        .find_map(|net| net.discipline.as_ref().and_then(discipline_domain))
    {
        return Some(domain);
    }
    let digital = module
        .digital_nets
        .iter()
        .flat_map(|net| &net.items)
        .chain(
            module
                .digital_variables
                .iter()
                .flat_map(|variable| &variable.items),
        )
        .any(|item| item.name == name);
    digital.then_some(Domain::Discrete)
}

struct Resolver<'a> {
    modules: &'a HashMap<&'a str, &'a Module>,
    db: &'a DisciplineDb,
    memo: HashMap<(SmolStr, SmolStr), Option<Domain>>,
}

impl Resolver<'_> {
    /// The domain of port `name` of `module`: declared, or for an undeclared
    /// port of a module with no analog block, the one domain every port it
    /// joins below shares. Continuous wins over discrete (Annex F.2.1 step
    /// 4b); a port that joins nothing, or a cell this source does not
    /// define, leaves it undetermined.
    fn domain(&mut self, module: &Module, name: &str, depth: usize) -> Option<Domain> {
        if let Some(domain) = declared_domain(module, name, self.db) {
            return Some(domain);
        }
        if module.analog_block.is_some() || depth > MAX_DEPTH {
            return None;
        }
        let key = (module.name.clone(), SmolStr::new(name));
        if let Some(domain) = self.memo.get(&key) {
            return *domain;
        }
        // A cycle reads as undetermined while it is being resolved.
        self.memo.insert(key.clone(), None);
        let mut joined = false;
        let mut discrete = true;
        let mut continuous = false;
        for instance in &module.instances {
            let Some(child) = self.modules.get(instance.module.as_str()).copied() else {
                if instance
                    .connections
                    .iter()
                    .any(|connection| names(connection, name))
                {
                    discrete = false;
                }
                continue;
            };
            for (index, connection) in instance.connections.iter().enumerate() {
                if !names(connection, name) {
                    continue;
                }
                let port = match connection {
                    Connection::Named { port, .. } => Some(port.clone()),
                    Connection::Ordered { .. } => {
                        child.ports.get(index).map(|port| port.name.clone())
                    }
                };
                joined = true;
                match port.and_then(|port| self.domain(child, &port, depth + 1)) {
                    Some(Domain::Discrete) => {}
                    Some(Domain::Continuous) => continuous = true,
                    None => discrete = false,
                }
            }
        }
        let domain = if continuous {
            Some(Domain::Continuous)
        } else if joined && discrete {
            Some(Domain::Discrete)
        } else {
            None
        };
        self.memo.insert(key, domain);
        domain
    }
}

/// Whether a connection's actual is exactly the net `name`.
fn names(connection: &Connection, name: &str) -> bool {
    let signal = match connection {
        Connection::Ordered { signal, .. } | Connection::Named { signal, .. } => signal,
    };
    matches!(signal, Some(Expression::Identifier(identifier)) if identifier.name == name)
}

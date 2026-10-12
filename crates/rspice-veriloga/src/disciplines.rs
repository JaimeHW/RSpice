//! Standard Verilog-AMS disciplines and natures
//!
//! This module provides the built-in disciplines and natures defined in
//! the Verilog-AMS standard, including electrical, thermal, and mechanical.
//!
//! These are typically included via `include "disciplines.vams"`

use crate::ast::AccessKind;
use crate::source::Span;
use std::collections::HashMap;

/// A nature defines a physical quantity (potential or flow)
#[derive(Debug, Clone)]
pub struct Nature {
    /// Name of the nature (e.g., "Voltage", "Current")
    pub name: String,
    /// Parent declaration, retained for physical compatibility checks.
    pub base: Option<String>,
    /// Units string (e.g., "V", "A")
    pub units: String,
    /// Abstol (absolute tolerance for convergence)
    pub abstol: f64,
    /// Access function name (e.g., "V" for voltage)
    pub access: String,
    /// The nature this is derived from (for ddt/idt)
    pub idt_nature: Option<String>,
    pub ddt_nature: Option<String>,
    /// Source location (for user-defined natures)
    pub span: Option<Span>,
}

impl Nature {
    /// Create a built-in nature
    pub fn builtin(name: &str, units: &str, abstol: f64, access: &str) -> Self {
        Self {
            name: name.to_string(),
            base: None,
            units: units.to_string(),
            abstol,
            access: access.to_string(),
            idt_nature: None,
            ddt_nature: None,
            span: None,
        }
    }
}

/// A discipline combines potential and flow natures
#[derive(Debug, Clone)]
pub struct Discipline {
    /// Name of the discipline (e.g., "electrical")
    pub name: String,
    /// Domain (continuous or discrete)
    pub domain: Domain,
    /// Potential nature (e.g., voltage)
    pub potential: Option<String>,
    /// Flow nature (e.g., current)
    pub flow: Option<String>,
    /// Source location (for user-defined disciplines)
    pub span: Option<Span>,
}

/// Discipline domain
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Domain {
    /// Continuous-time domain (analog)
    #[default]
    Continuous,
    /// Discrete-time domain (digital)
    Discrete,
}

impl Domain {
    /// The keyword a `domain` declaration names this with.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Continuous => "continuous",
            Self::Discrete => "discrete",
        }
    }
}

impl Discipline {
    /// Create a built-in discipline
    pub fn builtin(
        name: &str,
        domain: Domain,
        potential: Option<&str>,
        flow: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_string(),
            domain,
            potential: potential.map(|s| s.to_string()),
            flow: flow.map(|s| s.to_string()),
            span: None,
        }
    }
}

/// Registry of known natures and disciplines
#[derive(Debug, Clone, Default)]
pub struct DisciplineDb {
    pub natures: HashMap<String, Nature>,
    pub disciplines: HashMap<String, Discipline>,
}

impl DisciplineDb {
    /// Resolve against one discipline: the same nature may be a potential in
    /// one discipline and a flow in another. Access-name tables cannot express
    /// that distinction, or distinguish inherited natures sharing an accessor.
    pub fn access_kind(&self, discipline: &str, access: &str) -> Option<AccessKind> {
        let discipline = self.get_discipline(discipline)?;
        let matches = |nature: &Option<String>| {
            nature
                .as_deref()
                .and_then(|name| self.get_nature(name))
                .is_some_and(|nature| nature.access == access)
        };
        match (matches(&discipline.potential), matches(&discipline.flow)) {
            (true, false) => Some(AccessKind::Potential),
            (false, true) => Some(AccessKind::Flow),
            _ => None,
        }
    }

    /// Create a new empty database
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a database with standard Verilog-AMS disciplines
    pub fn with_standard() -> Self {
        let mut db = Self::new();
        db.add_standard_natures();
        db.add_standard_disciplines();
        db
    }

    /// Add a nature
    pub fn add_nature(&mut self, nature: Nature) {
        self.natures.insert(nature.name.clone(), nature);
    }

    /// Add a discipline
    pub fn add_discipline(&mut self, discipline: Discipline) {
        self.disciplines.insert(discipline.name.clone(), discipline);
    }

    /// Get a nature by name
    pub fn get_nature(&self, name: &str) -> Option<&Nature> {
        self.natures.get(name)
    }

    /// Root declaration shared by related natures. Invalid public database
    /// graphs return None rather than looping on a cycle.
    pub fn nature_base(&self, name: &str) -> Option<&str> {
        let mut nature = self.get_nature(name)?;
        for _ in 0..self.natures.len() {
            match &nature.base {
                Some(base) => nature = self.get_nature(base)?,
                None => return Some(&nature.name),
            }
        }
        None
    }

    /// Get a discipline by name
    pub fn get_discipline(&self, name: &str) -> Option<&Discipline> {
        self.disciplines.get(name)
    }

    /// Add the natures of the standard `disciplines.vams` (Verilog-AMS LRM
    /// 2.4, Annex D), attribute for attribute. A source that includes the
    /// header redeclares them, and a connection library compiled against it
    /// is compared with them field by field, so a built-in that differs in
    /// any attribute is a conflict the author never wrote.
    fn add_standard_natures(&mut self) {
        // (name, units, access, abstol, idt_nature, ddt_nature)
        type Row = (
            &'static str,
            &'static str,
            &'static str,
            f64,
            Option<&'static str>,
            Option<&'static str>,
        );
        const NATURES: [Row; 16] = [
            ("Current", "A", "I", 1e-12, Some("Charge"), None),
            ("Charge", "coul", "Q", 1e-14, None, Some("Current")),
            ("Voltage", "V", "V", 1e-6, Some("Flux"), None),
            ("Flux", "Wb", "Phi", 1e-9, None, Some("Voltage")),
            ("Magneto_Motive_Force", "A*turn", "MMF", 1e-12, None, None),
            ("Temperature", "K", "Temp", 1e-4, None, None),
            ("Power", "W", "Pwr", 1e-9, None, None),
            ("Position", "m", "Pos", 1e-6, None, Some("Velocity")),
            (
                "Velocity",
                "m/s",
                "Vel",
                1e-6,
                Some("Position"),
                Some("Acceleration"),
            ),
            (
                "Acceleration",
                "m/s^2",
                "Acc",
                1e-6,
                Some("Velocity"),
                Some("Impulse"),
            ),
            ("Impulse", "m/s^3", "Imp", 1e-6, Some("Acceleration"), None),
            ("Force", "N", "F", 1e-6, None, None),
            (
                "Angle",
                "rads",
                "Theta",
                1e-6,
                None,
                Some("Angular_Velocity"),
            ),
            (
                "Angular_Velocity",
                "rads/s",
                "Omega",
                1e-6,
                Some("Angle"),
                Some("Angular_Acceleration"),
            ),
            (
                "Angular_Acceleration",
                "rads/s^2",
                "Alpha",
                1e-6,
                Some("Angular_Velocity"),
                None,
            ),
            ("Angular_Force", "N*m", "Tau", 1e-6, None, None),
        ];
        for (name, units, access, abstol, idt_nature, ddt_nature) in NATURES {
            let mut nature = Nature::builtin(name, units, abstol, access);
            nature.idt_nature = idt_nature.map(str::to_owned);
            nature.ddt_nature = ddt_nature.map(str::to_owned);
            self.add_nature(nature);
        }
    }

    /// Add standard disciplines
    fn add_standard_disciplines(&mut self) {
        // Electrical disciplines
        self.add_discipline(Discipline::builtin(
            "electrical",
            Domain::Continuous,
            Some("Voltage"),
            Some("Current"),
        ));
        self.add_discipline(Discipline::builtin(
            "voltage",
            Domain::Continuous,
            Some("Voltage"),
            None,
        ));
        self.add_discipline(Discipline::builtin(
            "current",
            Domain::Continuous,
            None,
            Some("Current"),
        ));

        // Thermal discipline
        self.add_discipline(Discipline::builtin(
            "thermal",
            Domain::Continuous,
            Some("Temperature"),
            Some("Power"),
        ));

        // Mechanical - translational
        self.add_discipline(Discipline::builtin(
            "kinematic",
            Domain::Continuous,
            Some("Position"),
            Some("Force"),
        ));
        self.add_discipline(Discipline::builtin(
            "kinematic_v",
            Domain::Continuous,
            Some("Velocity"),
            Some("Force"),
        ));

        // Mechanical - rotational
        self.add_discipline(Discipline::builtin(
            "rotational",
            Domain::Continuous,
            Some("Angle"),
            Some("Angular_Force"),
        ));
        self.add_discipline(Discipline::builtin(
            "rotational_omega",
            Domain::Continuous,
            Some("Angular_Velocity"),
            Some("Angular_Force"),
        ));

        // Magnetic
        self.add_discipline(Discipline::builtin(
            "magnetic",
            Domain::Continuous,
            Some("Magneto_Motive_Force"),
            Some("Flux"),
        ));

        // Logic/Digital (for Verilog-AMS)
        self.add_discipline(Discipline::builtin("logic", Domain::Discrete, None, None));
        // Retain the conventional name previously accepted on wreal declarations
        // as its own explicit discipline, rather than discarding the annotation.
        self.add_discipline(Discipline::builtin(
            "ddiscrete",
            Domain::Discrete,
            None,
            None,
        ));
    }

    /// Resolve an access function to its nature
    pub fn resolve_access(&self, access: &str) -> Option<&Nature> {
        self.natures.values().find(|n| n.access == access)
    }

    /// Check if two disciplines are compatible for connection
    pub fn are_compatible(&self, d1: &str, d2: &str) -> bool {
        let (Some(a), Some(b)) = (self.get_discipline(d1), self.get_discipline(d2)) else {
            return false;
        };
        a.domain == b.domain
            && self.natures_compatible(a.potential.as_deref(), b.potential.as_deref())
            && self.natures_compatible(a.flow.as_deref(), b.flow.as_deref())
    }

    fn natures_compatible(&self, left: Option<&str>, right: Option<&str>) -> bool {
        let (Some(left), Some(right)) = (left, right) else {
            // A missing binding cannot conflict with the other discipline.
            return true;
        };
        let (Some(a), Some(b)) = (self.get_nature(left), self.get_nature(right)) else {
            return false;
        };
        a.units == b.units
            || self
                .nature_base(left)
                .is_some_and(|base| Some(base) == self.nature_base(right))
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessKind, Discipline, DisciplineDb, Domain, Nature};

    #[test]
    fn compatibility_checks_every_bound_nature_through_both_public_apis() {
        let mut db = DisciplineDb::with_standard();
        db.add_nature(Nature::builtin("OtherVoltage", "V", 1e-8, "OV"));
        db.add_discipline(Discipline::builtin(
            "same_units",
            Domain::Continuous,
            Some("OtherVoltage"),
            None,
        ));
        db.add_discipline(Discipline::builtin(
            "temperature",
            Domain::Continuous,
            Some("Temperature"),
            None,
        ));
        db.add_discipline(Discipline::builtin(
            "mismatched_flow",
            Domain::Continuous,
            Some("Voltage"),
            Some("Power"),
        ));
        db.add_discipline(Discipline::builtin(
            "natureless",
            Domain::Continuous,
            None,
            None,
        ));
        for (left, right, expected) in [
            ("voltage", "temperature", false),
            ("electrical", "mismatched_flow", false),
            ("electrical", "voltage", true),
            ("electrical", "current", true),
            ("voltage", "same_units", true),
            ("electrical", "natureless", true),
            ("electrical", "logic", false),
            ("unknown", "unknown", false),
        ] {
            for (a, b) in [(left, right), (right, left)] {
                assert_eq!(db.are_compatible(a, b), expected, "{a}/{b}");
                assert_eq!(
                    crate::connect::disciplines_compatible(&db, a, b),
                    expected,
                    "connect {a}/{b}"
                );
            }
        }
    }

    /// Every nature and discipline a `disciplines.vams` text declares, as the
    /// compiler registers them, keyed by name.
    fn declared_by(header: &str) -> (Vec<(String, NatureRow)>, Vec<(String, DisciplineRow)>) {
        use crate::ast::Item;
        let source = crate::preprocessor::Preprocessor::new()
            .preprocess_source(header)
            .unwrap();
        let tokens = crate::Lexer::new(&source, crate::source::SourceId::new(1))
            .collect_tokens()
            .unwrap();
        let ast = crate::Parser::new(&tokens).parse().unwrap();
        let (db, _) = crate::semantic::SemanticAnalyzer::new()
            .connection_definitions(&ast)
            .unwrap();
        let mut natures = Vec::new();
        let mut disciplines = Vec::new();
        for item in &ast.items {
            match item {
                Item::Nature(nature) => {
                    let name = nature.name.as_str();
                    natures.push((name.to_owned(), nature_row(&db.natures[name])));
                }
                Item::Discipline(discipline) => {
                    let name = discipline.name.as_str();
                    disciplines.push((name.to_owned(), discipline_row(&db.disciplines[name])));
                }
                _ => {}
            }
        }
        natures.sort();
        disciplines.sort();
        (natures, disciplines)
    }

    type NatureRow = (
        Option<String>,
        String,
        u64,
        String,
        Option<String>,
        Option<String>,
    );
    type DisciplineRow = (String, Option<String>, Option<String>);

    fn nature_row(nature: &Nature) -> NatureRow {
        (
            nature.base.clone(),
            nature.units.clone(),
            nature.abstol.to_bits(),
            nature.access.clone(),
            nature.idt_nature.clone(),
            nature.ddt_nature.clone(),
        )
    }

    fn discipline_row(discipline: &Discipline) -> DisciplineRow {
        (
            format!("{:?}", discipline.domain),
            discipline.potential.clone(),
            discipline.flow.clone(),
        )
    }

    /// A source that includes `disciplines.vams` redeclares every built-in, and
    /// a connection library compiled against it is checked against the
    /// device's natures field by field. So the built-ins are the standard
    /// header, attribute for attribute and name for name, and so is the copy
    /// the preprocessor falls back to when no header is on disk. The standard
    /// is Accellera's header as the repository vendors it beside the models
    /// that ship their own copy.
    #[test]
    fn built_ins_and_the_fallback_header_are_the_standard_header() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../models/veriloga/vbic_1.3/vacode/disciplines.vams");
        let vendored = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let standard = declared_by(&vendored);
        assert_eq!(standard.0.len(), 16, "the standard declares 16 natures");
        assert_eq!(standard.1.len(), 11, "the standard declares 11 disciplines");

        let built_in = DisciplineDb::with_standard();
        let mut natures: Vec<_> = built_in
            .natures
            .iter()
            .map(|(name, nature)| (name.clone(), nature_row(nature)))
            .collect();
        natures.sort();
        let mut disciplines: Vec<_> = built_in
            .disciplines
            .iter()
            .map(|(name, discipline)| (name.clone(), discipline_row(discipline)))
            .collect();
        disciplines.sort();
        assert_eq!(natures, standard.0, "built-in natures");
        assert_eq!(disciplines, standard.1, "built-in disciplines");

        assert_eq!(
            declared_by(crate::stdlib::DISCIPLINES_VAMS),
            standard,
            "the fallback header"
        );
    }

    #[test]
    fn magnetic_access_functions_keep_their_standard_roles() {
        let disciplines = DisciplineDb::with_standard();
        assert_eq!(
            disciplines.access_kind("magnetic", "MMF"),
            Some(AccessKind::Potential)
        );
        assert_eq!(
            disciplines.access_kind("magnetic", "Phi"),
            Some(AccessKind::Flow)
        );
    }
}

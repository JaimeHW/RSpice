//! Standard Verilog-AMS disciplines and natures
//!
//! This module provides the built-in disciplines and natures defined in
//! the Verilog-AMS standard, including electrical, thermal, and mechanical.
//!
//! These are typically included via `include "disciplines.vams"`

use crate::source::Span;
use std::collections::HashMap;

/// A nature defines a physical quantity (potential or flow)
#[derive(Debug, Clone)]
pub struct Nature {
    /// Name of the nature (e.g., "Voltage", "Current")
    pub name: String,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Domain {
    /// Continuous-time domain (analog)
    #[default]
    Continuous,
    /// Discrete-time domain (digital)
    Discrete,
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

    /// Get a discipline by name
    pub fn get_discipline(&self, name: &str) -> Option<&Discipline> {
        self.disciplines.get(name)
    }

    /// Add standard electrical natures
    fn add_standard_natures(&mut self) {
        // Electrical
        self.add_nature(Nature::builtin("Voltage", "V", 1.0e-6, "V"));
        self.add_nature(Nature::builtin("Current", "A", 1.0e-12, "I"));

        // Charge/Flux (for capacitor/inductor modeling)
        let mut charge = Nature::builtin("Charge", "coul", 1.0e-14, "Q");
        charge.ddt_nature = Some("Current".to_string());
        self.add_nature(charge);

        let mut flux = Nature::builtin("Flux", "Wb", 1.0e-9, "Phi");
        flux.ddt_nature = Some("Voltage".to_string());
        self.add_nature(flux);

        // Thermal
        self.add_nature(Nature::builtin("Temperature", "K", 1.0e-4, "Temp"));
        self.add_nature(Nature::builtin("Power", "W", 1.0e-9, "Pwr"));

        // Mechanical - translational
        self.add_nature(Nature::builtin("Position", "m", 1.0e-6, "Pos"));
        self.add_nature(Nature::builtin("Velocity", "m/s", 1.0e-6, "Vel"));
        self.add_nature(Nature::builtin("Force", "N", 1.0e-6, "F"));

        // Mechanical - rotational
        self.add_nature(Nature::builtin("Angle", "rad", 1.0e-6, "Theta"));
        self.add_nature(Nature::builtin("AngularVelocity", "rad/s", 1.0e-6, "Omega"));
        self.add_nature(Nature::builtin("Torque", "N*m", 1.0e-6, "Tau"));

        // Magnetic
        self.add_nature(Nature::builtin("Mmf", "A*turn", 1.0e-6, "MMF"));

        // Fluid
        self.add_nature(Nature::builtin("Pressure", "Pa", 1.0e-3, "P"));
        self.add_nature(Nature::builtin("VolumeFlowRate", "m^3/s", 1.0e-12, "Flow"));
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
            Some("Torque"),
        ));
        self.add_discipline(Discipline::builtin(
            "rotational_omega",
            Domain::Continuous,
            Some("AngularVelocity"),
            Some("Torque"),
        ));

        // Magnetic
        self.add_discipline(Discipline::builtin(
            "magnetic",
            Domain::Continuous,
            Some("Mmf"),
            Some("Flux"),
        ));

        // Logic/Digital (for Verilog-AMS)
        self.add_discipline(Discipline::builtin("logic", Domain::Discrete, None, None));
    }

    /// Resolve an access function to its nature
    pub fn resolve_access(&self, access: &str) -> Option<&Nature> {
        self.natures.values().find(|n| n.access == access)
    }

    /// Check if two disciplines are compatible for connection
    pub fn are_compatible(&self, d1: &str, d2: &str) -> bool {
        if d1 == d2 {
            return true;
        }

        let disc1 = self.get_discipline(d1);
        let disc2 = self.get_discipline(d2);

        match (disc1, disc2) {
            (Some(a), Some(b)) => {
                // Same domain and at least one nature in common
                a.domain == b.domain && (a.potential == b.potential || a.flow == b.flow)
            }
            _ => false,
        }
    }
}

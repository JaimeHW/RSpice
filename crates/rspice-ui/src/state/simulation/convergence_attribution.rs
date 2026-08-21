//! What a failed run named on the drawing.
//!
//! The engine can say which nodes had no DC path, which matrix rows carried
//! no equation, and which KCL equations a Newton abort left violated. This is
//! the frontend mirror of that: the same facts in the words a schematic
//! author uses, carried by the result the run produced so a surface can mark
//! the objects instead of asking anyone to read node names out of a paragraph.

/// Which failure the engine attributed a run to.
///
/// A frontend mirror of the engine's own class, kept separate so the design
/// surfaces can be told apart by the words a schematic author uses rather
/// than by the words the solver does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergenceFailureClass {
    /// Nothing conducts at DC to the named nodes.
    NoDcPathToGround,
    /// The accepted bias at the named nodes survives only on the simulator's
    /// numerical conditioning.
    ConditioningDependentBias,
    /// No equation constrains the named rows.
    SingularSystem,
    /// Newton ran out of iterations with the named nodes worst-violated.
    NewtonNonConvergence,
}

impl ConvergenceFailureClass {
    /// The headline a surface shows above the named objects.
    #[must_use]
    pub fn headline(self) -> &'static str {
        match self {
            Self::NoDcPathToGround => "No DC path to ground",
            Self::ConditioningDependentBias => "Bias depends on numerical conditioning",
            Self::SingularSystem => "Singular system",
            Self::NewtonNonConvergence => "Did not converge",
        }
    }
}

/// Whether an attributed object is a conductor or a device branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergenceSiteKind {
    /// A circuit node, which the schematic draws as a conductor.
    Node,
    /// A branch current, which the schematic draws as a device.
    Branch,
}

/// One design object a failed run was attributed to.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvergenceSite {
    /// The node or device name as the engine spelled it.
    pub name: String,
    /// What kind of object `name` addresses.
    pub kind: ConvergenceSiteKind,
    /// Scaled residual where the class measured one; `1.0` is exactly at
    /// tolerance.
    #[serde(default)]
    pub residual: Option<f64>,
}

/// The design objects a failed run named, as the schematic can address them.
///
/// This is evidence about one run at the topology it ran against. It carries
/// no geometry on purpose: a conductor is found by name through the run's
/// cross-probe map, so a schematic edited since the run refuses to point
/// anywhere rather than pointing at where a wire used to be.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConvergenceAttribution {
    /// Which failure this attributes.
    pub class: ConvergenceFailureClass,
    /// The named objects, worst first where the class ranks them.
    pub sites: Vec<ConvergenceSite>,
    /// Objects the engine measured but did not name. A surface that shows
    /// the sites must say the list is partial when this is non-zero.
    #[serde(default)]
    pub elided_sites: usize,
}

impl ConvergenceAttribution {
    /// The node names, in attribution order.
    pub fn nets(&self) -> impl Iterator<Item = &str> {
        self.sites
            .iter()
            .filter(|site| site.kind == ConvergenceSiteKind::Node)
            .map(|site| site.name.as_str())
    }

    /// The device instance names, in attribution order.
    pub fn devices(&self) -> impl Iterator<Item = &str> {
        self.sites
            .iter()
            .filter(|site| site.kind == ConvergenceSiteKind::Branch)
            .map(|site| site.name.as_str())
    }

    /// One line naming what failed and where, for a console row or a button.
    pub fn summary(&self) -> String {
        let named: Vec<&str> = self.sites.iter().map(|site| site.name.as_str()).collect();
        let tail = if self.elided_sites > 0 {
            format!(" (and {} more)", self.elided_sites)
        } else {
            String::new()
        };
        format!("{}: {}{tail}", self.class.headline(), named.join(", "))
    }
}

impl From<&rspice_core::diagnostics::ConvergenceDiagnostic> for ConvergenceAttribution {
    fn from(diagnostic: &rspice_core::diagnostics::ConvergenceDiagnostic) -> Self {
        use rspice_core::diagnostics as core;
        Self {
            class: match diagnostic.class {
                core::ConvergenceFailureClass::NoDcPathToGround => {
                    ConvergenceFailureClass::NoDcPathToGround
                }
                core::ConvergenceFailureClass::ConditioningDependentBias => {
                    ConvergenceFailureClass::ConditioningDependentBias
                }
                core::ConvergenceFailureClass::SingularSystem => {
                    ConvergenceFailureClass::SingularSystem
                }
                // The engine's class list is non-exhaustive. A class this
                // build does not know is still a solve that gave up, so it
                // reports as one rather than dropping the named objects.
                _ => ConvergenceFailureClass::NewtonNonConvergence,
            },
            sites: diagnostic
                .sites
                .iter()
                .map(|site| ConvergenceSite {
                    name: site.name.clone(),
                    kind: match site.kind {
                        core::ConvergenceSiteKind::Node => ConvergenceSiteKind::Node,
                        core::ConvergenceSiteKind::Branch => ConvergenceSiteKind::Branch,
                    },
                    residual: site.residual,
                })
                .collect(),
            elided_sites: diagnostic.elided_sites,
        }
    }
}

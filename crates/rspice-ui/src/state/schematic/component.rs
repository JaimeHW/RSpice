//! Component Struct
//!
//! A placed component on the schematic with position, rotation, and properties.

use super::component_type::ComponentType;
use super::point::{LabelPosition, Point};
use super::rotation::Rotation;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::path::PathBuf;

// =============================================================================
// Component
// =============================================================================

/// Per-instance label visibility override used by reviewed bulk edits.
///
/// `Inherit` delegates to the device-local canvas annotation policy. Explicit
/// values are durable schematic presentation data and therefore participate
/// in schematic undo/redo and project serialization.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComponentDisplayMode {
    #[default]
    Inherit,
    NameAndValue,
    NameOnly,
    ValueOnly,
    Hidden,
}

impl ComponentDisplayMode {
    pub const ALL_EXPLICIT: [Self; 4] = [
        Self::NameAndValue,
        Self::NameOnly,
        Self::ValueOnly,
        Self::Hidden,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::NameAndValue => "name and value",
            Self::NameOnly => "name only",
            Self::ValueOnly => "value only",
            Self::Hidden => "hidden",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let normalized = value.trim().to_ascii_lowercase().replace(['_', '-'], " ");
        match normalized.as_str() {
            "inherit" | "inherited" => Some(Self::Inherit),
            "name and value" | "both" | "full" => Some(Self::NameAndValue),
            "name only" | "name" => Some(Self::NameOnly),
            "value only" | "value" => Some(Self::ValueOnly),
            "hidden" | "hide" | "none" => Some(Self::Hidden),
            _ => None,
        }
    }

    pub const fn show_name(
        self,
        inherited: crate::state::SchematicParameterLabelVisibility,
    ) -> bool {
        match self {
            Self::Inherit => matches!(
                inherited,
                crate::state::SchematicParameterLabelVisibility::NamesAndValues
            ),
            Self::NameAndValue | Self::NameOnly => true,
            Self::ValueOnly | Self::Hidden => false,
        }
    }

    pub const fn show_value(
        self,
        inherited: crate::state::SchematicParameterLabelVisibility,
    ) -> bool {
        match self {
            Self::Inherit => !matches!(
                inherited,
                crate::state::SchematicParameterLabelVisibility::Hidden
            ),
            Self::NameAndValue | Self::ValueOnly => true,
            Self::NameOnly | Self::Hidden => false,
        }
    }
}

/// Library/cell/view binding for a generic hierarchical instance.
///
/// This mirrors commercial LCV instance binding where a placed instance points
/// to a library master and selected view.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LibraryCellInstance {
    /// Source library name.
    pub library: String,
    /// Source cell name.
    pub cell: String,
    /// Bound view name (for example: "schematic", "symbol", "veriloga").
    pub view: String,
    /// Optional Verilog-A source file path for netlisting.
    #[serde(default)]
    pub source_path: Option<PathBuf>,
    /// Optional master name override used during netlisting
    /// (for example: Verilog-A module or subcircuit name).
    #[serde(default)]
    pub module_name: Option<String>,
    /// Constrained instance-line template owned by a model-bound symbol.
    /// Supported placeholders are validated before publication and again at
    /// netlist generation; legacy bindings leave this unset and use the
    /// standard X-instance form.
    #[serde(default)]
    pub netlist_template: Option<String>,
    /// Optional named `.lib` section required to expose this model from its
    /// authoritative source file.
    #[serde(default)]
    pub model_section: Option<String>,
    /// Preferred reference-designator prefix for newly placed instances.
    /// This does not rewrite existing instances when a library definition is
    /// revised.
    #[serde(default)]
    pub reference_prefix: Option<String>,
    /// Ordered, case-insensitive parameter allowlist for model-bound
    /// instances. Empty preserves legacy source-backed cell behavior.
    #[serde(default)]
    pub parameter_order: Vec<String>,
    /// Terminal order used for schematic connectivity and netlist emission.
    #[serde(default)]
    pub terminal_order: Vec<String>,
    /// Per-terminal directions, parallel to `terminal_order`. When present
    /// and consistent, the instance renders with the direction-aware
    /// generated symbol instead of the generic distributed block.
    #[serde(default)]
    pub terminal_dirs: Vec<super::port::PortDirection>,
    /// Distinguishes an explicitly bound zero-port interface from legacy
    /// bindings whose absent terminal metadata still requires compatibility
    /// fallback geometry.
    #[serde(default)]
    pub interface_bound: bool,
    /// Executable built-in XSPICE binding. Unlike an ordinary L/C/V binding,
    /// this is resolved from RSpice's compiled code-model registry and needs
    /// no source file or project-cell master.
    #[serde(default)]
    pub builtin_xspice: Option<BuiltinXspiceInstance>,
    /// Executable build-time generated Verilog-A binding. The frozen identity
    /// is revalidated against the running engine before editing or netlisting.
    #[serde(default)]
    pub generated_veriloga: Option<GeneratedVerilogAInstance>,
}

/// Frozen catalog identity for a build-time generated Verilog-A placement.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneratedVerilogAInstance {
    pub schema_revision: u32,
    pub stable_id: String,
    pub descriptor_abi_version: u32,
    pub model_name: String,
    pub module_name: String,
    pub source_digest: String,
    pub checkpoint_identity: String,
    pub descriptor_signature: String,
}

/// Frozen executable contract for one built-in XSPICE catalog placement.
///
/// The exact connection shaping is persisted with the instance. A future
/// registry change therefore makes an old placement explicitly stale instead
/// of silently changing its pin order or scalar/vector interpretation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuiltinXspiceInstance {
    pub schema_revision: u32,
    pub stable_id: String,
    pub model_type: String,
    pub symbol_asset: String,
    pub schema_signature: String,
    pub ports: Vec<BuiltinXspicePortBinding>,
}

/// One code-model port and the schematic terminal indices that implement it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuiltinXspicePortBinding {
    pub name: String,
    pub direction: BuiltinXspicePortDirection,
    pub port_type: BuiltinXspicePortType,
    pub is_vector: bool,
    /// Number of logical elements materialized for this port. Scalar ports
    /// always use one. A nullable vector may use zero, which emits a literal
    /// null connection and deliberately owns no schematic terminals.
    #[serde(default = "default_xspice_port_width")]
    pub vector_width: usize,
    pub null_allowed: bool,
    pub terminals: Vec<usize>,
}

const fn default_xspice_port_width() -> usize {
    1
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltinXspicePortDirection {
    In,
    Out,
    InOut,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltinXspicePortType {
    Voltage,
    DifferentialVoltage,
    Conductance,
    DifferentialConductance,
    Hybrid,
    DifferentialHybrid,
    Current,
    DifferentialCurrent,
    VoltageName,
    Digital,
    Real,
    Integer,
    UserDefined,
}

/// Validate the deliberately small instance-template language used by
/// model-bound symbols. Keeping the grammar here prevents authoring,
/// placement, and netlist generation from accepting different contracts.
pub fn validate_library_netlist_template(template: &str) -> Result<(), String> {
    let template = template.trim();
    if template.is_empty() {
        return Err("template is empty".to_owned());
    }
    if template.len() > 512 {
        return Err("template exceeds the 512-byte limit".to_owned());
    }
    if template.chars().any(char::is_control) {
        return Err("template contains a line break or control character".to_owned());
    }
    let tokens = template.split_ascii_whitespace().collect::<Vec<_>>();
    let Some(reference) = tokens.first().copied() else {
        return Err("template is empty".to_owned());
    };
    let reference_is_bound = reference == "{ref}"
        || reference.strip_suffix("{name}").is_some_and(|prefix| {
            !prefix.is_empty()
                && prefix
                    .chars()
                    .all(|character| character.is_ascii_alphabetic())
        });
    if !reference_is_bound {
        return Err(
            "template must start with {ref} or an ASCII device prefix followed by {name}"
                .to_owned(),
        );
    }
    if !matches!(
        tokens.as_slice(),
        [_, "{nodes}", "{model}"] | [_, "{nodes}", "{model}", "{params}"]
    ) {
        return Err(
            "template must be <reference> {nodes} {model} with optional trailing {params}"
                .to_owned(),
        );
    }
    Ok(())
}

/// The one sentence every authoring surface shows for an iterated designator.
pub const ITERATED_INSTANCE_REJECTION: &str =
    "Iterated instances are not supported — place N instances or use the array tool";

impl LibraryCellInstance {
    /// Create a new library/cell/view binding.
    pub fn new(
        library: impl Into<String>,
        cell: impl Into<String>,
        view: impl Into<String>,
    ) -> Self {
        Self {
            library: library.into(),
            cell: cell.into(),
            view: view.into(),
            source_path: None,
            module_name: None,
            netlist_template: None,
            model_section: None,
            reference_prefix: None,
            parameter_order: Vec::new(),
            terminal_order: Vec::new(),
            terminal_dirs: Vec::new(),
            interface_bound: false,
            builtin_xspice: None,
            generated_veriloga: None,
        }
    }

    pub fn is_builtin_xspice(&self) -> bool {
        self.builtin_xspice.is_some()
    }

    pub fn is_generated_veriloga(&self) -> bool {
        self.generated_veriloga.is_some()
    }

    pub fn is_executable_builtin(&self) -> bool {
        self.is_builtin_xspice() || self.is_generated_veriloga()
    }

    /// Bind this instance to a cell interface: terminal order and
    /// directions in one move, so the pair can never go out of step.
    pub fn bind_interface(&mut self, ports: &[super::port::PortSpec]) {
        self.terminal_order = ports.iter().map(|port| port.name.clone()).collect();
        self.terminal_dirs = ports.iter().map(|port| port.direction).collect();
        self.interface_bound = true;
    }

    /// Conductors behind each bound terminal, parallel to `terminal_order`.
    ///
    /// A terminal named `DATA[7:0]` is one drawn pin carrying eight nodes.
    /// The widths are read from the frozen terminal names rather than stored
    /// beside them, so a placement can never remember a width its own
    /// interface contradicts.
    pub fn terminal_widths(&self) -> Vec<usize> {
        self.terminal_order
            .iter()
            .map(|terminal| super::bus::declared_width(terminal))
            .collect()
    }

    /// Total nodes an instance card emits for this binding.
    pub fn terminal_node_count(&self) -> usize {
        self.terminal_widths().iter().sum()
    }

    pub fn effective_reference_prefix(&self) -> Option<&str> {
        self.reference_prefix
            .as_deref()
            .map(str::trim)
            .filter(|prefix| {
                !prefix.is_empty()
                    && prefix.len() <= 8
                    && prefix
                        .chars()
                        .all(|character| character.is_ascii_alphabetic())
            })
    }

    /// The bound interface as port specs, when directions are available
    /// and consistent with the terminal order.
    pub fn interface(&self) -> Option<Vec<super::port::PortSpec>> {
        if (!self.interface_bound && self.terminal_order.is_empty())
            || self.terminal_dirs.len() != self.terminal_order.len()
        {
            return None;
        }
        Some(
            self.terminal_order
                .iter()
                .zip(&self.terminal_dirs)
                .map(|(name, &direction)| super::port::PortSpec {
                    name: name.clone(),
                    direction,
                })
                .collect(),
        )
    }
}

/// How many identical copies of its master one placed instance stands for.
///
/// The flattener reads `M=` on an X line as a physical multiplier and accepts
/// any finite, strictly positive value, so half a unit device is a legitimate
/// request and is deliberately not rounded to a whole count here.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct InstanceMultiplicity(f64);

impl InstanceMultiplicity {
    /// Reserved parameter spelling this value owns.
    pub const PARAMETER_NAME: &'static str = "m";

    /// Shown wherever `m` is refused as an ordinary parameter name, so the
    /// authoring gate and the netlist gate say the same thing.
    pub const RESERVED_GUIDANCE: &'static str =
        "`m` is the instance multiplicity; set it on the instance";

    pub fn new(value: f64) -> Result<Self, String> {
        if !value.is_finite() || value <= 0.0 {
            return Err(format!(
                "multiplicity must be a finite number greater than zero, not `{value}`"
            ));
        }
        Ok(Self(value))
    }

    pub fn parse(text: &str) -> Result<Self, String> {
        let text = text.trim();
        let value = text.parse::<f64>().map_err(|_| {
            format!("multiplicity must be a finite number greater than zero, not `{text}`")
        })?;
        Self::new(value)
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for InstanceMultiplicity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for InstanceMultiplicity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(f64::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

/// The drawn block of a bound cell instance: every pin with the body edge it
/// belongs to, and the body outline those leads land on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InstanceBlock {
    pub pins: Vec<super::symbol_gen::GeneratedPin>,
    pub body: (Point, Point),
}

/// A placed component on the schematic
///
/// Components are circuit elements placed on the schematic canvas.
/// Each component has a position, rotation, reference designator (name),
/// value, and optional parameters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(from = "PersistedComponent")]
pub struct Component {
    /// Unique identifier within the schematic
    pub id: u64,

    /// Component type (resistor, capacitor, etc.)
    pub kind: ComponentType,

    /// Position on grid (in grid units, not pixels)
    pub pos: Point,

    /// Rotation (0, 90, 180, or 270 degrees)
    pub rotation: Rotation,

    /// Component reference designator (e.g., "R1", "C2", "U3")
    pub name: String,

    /// Component value (e.g., "1k", "10u", "LM741")
    pub value: String,

    /// Additional SPICE parameters (e.g., "TC=0.01,0.001")
    pub params: String,

    /// Optional schematic symbol variant identifier.
    ///
    /// This is a UI-only appearance choice and is never emitted into the
    /// generated SPICE netlist.
    #[serde(default)]
    pub symbol_variant: Option<String>,

    /// Name label position (Auto for smart placement, Custom for user-defined)
    #[serde(default)]
    pub name_label_pos: LabelPosition,

    /// Value label position (Auto for smart placement, Custom for user-defined)
    #[serde(default)]
    pub value_label_pos: LabelPosition,

    /// Durable per-instance label visibility override.
    #[serde(default)]
    pub display_mode: ComponentDisplayMode,

    /// Horizontal mirror (flip left/right)
    ///
    /// When true, the component is mirrored horizontally (about Y-axis).
    /// This is essential for proper transistor orientation (e.g., NMOS with
    /// drain on left vs right). Matches Cadence Virtuoso 'H' key behavior.
    #[serde(default)]
    pub mirror_h: bool,

    /// Vertical mirror (flip up/down)
    ///
    /// When true, the component is mirrored vertically (about X-axis).
    /// Matches Cadence Virtuoso 'V' key behavior.
    #[serde(default)]
    pub mirror_v: bool,

    /// Optional bound library/cell/view metadata for hierarchical instances.
    ///
    /// When present, this component is a generic cell instance (X element style)
    /// and uses dynamic terminal layout derived from `terminal_order`.
    #[serde(default)]
    pub library_cell: Option<LibraryCellInstance>,

    /// Typed instance multiplicity emitted as `m=` on a cell instance line.
    ///
    /// `None` is the engine's implicit one. Holding the value here instead of
    /// in `params` is what keeps it out of free-text parameter input, where a
    /// misspelling would reach the flattener as an ordinary formal parameter
    /// and quietly stop multiplying anything.
    #[serde(default)]
    pub multiplicity: Option<InstanceMultiplicity>,
}

/// Component fields exactly as a project file on disk spells them.
///
/// Decoding through this shape gives [`Component`] one migration point: a `m=`
/// written into `params` before the multiplicity became a typed field moves
/// onto the instance as the document loads, so a design keeps the multiplier
/// it was simulated with.
#[derive(Deserialize)]
struct PersistedComponent {
    id: u64,
    kind: ComponentType,
    pos: Point,
    rotation: Rotation,
    name: String,
    value: String,
    params: String,
    #[serde(default)]
    symbol_variant: Option<String>,
    #[serde(default)]
    name_label_pos: LabelPosition,
    #[serde(default)]
    value_label_pos: LabelPosition,
    #[serde(default)]
    display_mode: ComponentDisplayMode,
    #[serde(default)]
    mirror_h: bool,
    #[serde(default)]
    mirror_v: bool,
    #[serde(default)]
    library_cell: Option<LibraryCellInstance>,
    #[serde(default)]
    multiplicity: Option<InstanceMultiplicity>,
}

impl From<PersistedComponent> for Component {
    fn from(persisted: PersistedComponent) -> Self {
        let mut component = Self {
            id: persisted.id,
            kind: persisted.kind,
            pos: persisted.pos,
            rotation: persisted.rotation,
            name: persisted.name,
            value: persisted.value,
            params: persisted.params,
            symbol_variant: persisted.symbol_variant,
            name_label_pos: persisted.name_label_pos,
            value_label_pos: persisted.value_label_pos,
            display_mode: persisted.display_mode,
            mirror_h: persisted.mirror_h,
            mirror_v: persisted.mirror_v,
            library_cell: persisted.library_cell,
            multiplicity: persisted.multiplicity,
        };
        component.adopt_params_multiplicity();
        component
    }
}

impl Component {
    /// Create a new component with default values
    ///
    /// # Arguments
    /// * `id` - Unique identifier
    /// * `kind` - Component type
    /// * `pos` - Position on the grid
    pub fn new(id: u64, kind: ComponentType, pos: Point) -> Self {
        Self {
            id,
            kind,
            pos,
            rotation: Rotation::default(),
            name: String::new(),
            value: String::new(),
            params: String::new(),
            symbol_variant: None,
            name_label_pos: LabelPosition::Auto,
            value_label_pos: LabelPosition::Auto,
            display_mode: ComponentDisplayMode::Inherit,
            mirror_h: false,
            mirror_v: false,
            library_cell: None,
            multiplicity: None,
        }
    }

    /// Move a free-text `m=` on a cell instance onto the typed multiplicity.
    ///
    /// On an X line `m` is the flattener's physical multiplier rather than a
    /// subcircuit parameter, so it is instance state and never parameter text.
    /// A spelling the typed field cannot hold stays in `params`, where netlist
    /// generation names it instead of discarding it. Primitive devices keep
    /// their own `m` parameter untouched — there it really is a device
    /// parameter on the card.
    pub fn adopt_params_multiplicity(&mut self) {
        if self.kind != ComponentType::CellInstance {
            return;
        }
        let mut params = crate::state::parse_params_string(&self.params);
        let Some(authored) = params.get(InstanceMultiplicity::PARAMETER_NAME) else {
            return;
        };
        let Ok(multiplicity) = InstanceMultiplicity::parse(authored) else {
            return;
        };
        self.multiplicity = Some(multiplicity);
        params.remove(InstanceMultiplicity::PARAMETER_NAME);
        self.params = crate::state::format_params_string(&params);
    }

    /// Create a component with name and value
    pub fn with_name_value(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.name = name.into();
        self.value = value.into();
        self
    }

    /// Create a component with a specific schematic symbol variant.
    pub fn with_symbol_variant(mut self, symbol_variant: impl Into<String>) -> Self {
        self.symbol_variant = Some(symbol_variant.into());
        self
    }

    /// Create a component with rotation
    pub fn with_rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    /// Create a component with horizontal mirror
    ///
    /// Horizontal mirror flips the component about the Y-axis (left/right swap).
    pub fn with_mirror_h(mut self, mirror_h: bool) -> Self {
        self.mirror_h = mirror_h;
        self
    }

    /// Create a component with vertical mirror
    ///
    /// Vertical mirror flips the component about the X-axis (up/down swap).
    pub fn with_mirror_v(mut self, mirror_v: bool) -> Self {
        self.mirror_v = mirror_v;
        self
    }

    /// Attach a library/cell/view binding to this instance.
    pub fn with_library_cell(mut self, library_cell: LibraryCellInstance) -> Self {
        self.library_cell = Some(library_cell);
        self
    }

    /// Toggle horizontal mirror
    pub fn toggle_mirror_h(&mut self) {
        self.mirror_h = !self.mirror_h;
    }

    /// Toggle vertical mirror
    pub fn toggle_mirror_v(&mut self) {
        self.mirror_v = !self.mirror_v;
    }

    /// Get terminal positions in world coordinates
    ///
    /// Returns terminal positions accounting for component position, rotation, and mirror.
    /// Each terminal is identified by name (e.g., "+", "-", "B", "C", "E").
    ///
    /// This is used for:
    /// - Wire snapping to terminals
    /// - Netlist generation
    /// - Rubber-banding during component moves
    pub fn terminal_positions(&self) -> Vec<(&'static str, Point)> {
        if self.library_cell.is_some() {
            return self
                .instance_pin_layout()
                .into_iter()
                .enumerate()
                .map(|(idx, (_, offset))| {
                    let transformed = self.transform_point(offset);
                    (
                        Self::terminal_label(idx),
                        Point::new(
                            self.pos.x.saturating_add(transformed.x),
                            self.pos.y.saturating_add(transformed.y),
                        ),
                    )
                })
                .collect();
        }

        // Terminal offsets are component-type specific and defined in ComponentType
        self.kind
            .terminal_offsets()
            .iter()
            .map(|&(name, offset)| {
                let transformed = self.transform_point(offset);
                (
                    name,
                    Point::new(
                        self.pos.x.saturating_add(transformed.x),
                        self.pos.y.saturating_add(transformed.y),
                    ),
                )
            })
            .collect()
    }

    /// Get terminal positions in world coordinates, using a resolved authored
    /// cell symbol when one is available for a placed cell instance.
    pub fn terminal_positions_resolved(
        &self,
        resolved_symbol: Option<&crate::state::ResolvedCellSymbol>,
    ) -> Vec<(String, Point)> {
        if self.kind == ComponentType::CellInstance
            && let Some(resolved_symbol) = resolved_symbol
        {
            return resolved_symbol
                .connectable_pins()
                .map(|pin| {
                    let transformed =
                        self.transform_point(resolved_symbol.effective_pin_offset(pin));
                    (
                        pin.name.clone(),
                        Point::new(
                            self.pos.x.saturating_add(transformed.x),
                            self.pos.y.saturating_add(transformed.y),
                        ),
                    )
                })
                .collect();
        }

        self.terminal_positions()
            .into_iter()
            .map(|(name, pos)| (name.to_owned(), pos))
            .collect()
    }

    /// Local pin layout of a bound cell instance, in interface order:
    /// `(port name when known, unrotated offset)`. Direction-aware when the
    /// binding carries directions; the legacy distributed block otherwise.
    /// Drawing and terminal extraction both read this, so the symbol and
    /// the connectivity can never disagree.
    pub(crate) fn instance_pin_layout(&self) -> Vec<(Option<String>, Point)> {
        self.instance_block_pins()
            .into_iter()
            .map(|pin| ((!pin.name.is_empty()).then_some(pin.name), pin.offset))
            .collect()
    }

    /// Local pin layout with the body edge each pin is drawn against.
    ///
    /// Everything that draws a block — canvas, SVG export, printed sheet —
    /// needs the edge, not just the point: it is what decides which way a
    /// lead runs and which way its name reads. An unnamed legacy terminal
    /// comes back with an empty name.
    pub(crate) fn instance_block_pins(&self) -> Vec<super::symbol_gen::GeneratedPin> {
        self.instance_block().pins
    }

    /// The whole drawn block: its pins, their edges, and the body they meet.
    pub(crate) fn instance_block(&self) -> InstanceBlock {
        use super::symbol_gen::GeneratedPin;

        let Some(cell) = &self.library_cell else {
            return InstanceBlock {
                pins: Vec::new(),
                body: (Point::origin(), Point::origin()),
            };
        };

        if let Some(ports) = cell.interface() {
            let generated = super::symbol_gen::generate_symbol(&ports);
            let body = generated.body_bounds();
            return InstanceBlock {
                pins: generated.pins,
                body,
            };
        }

        // A cell bound by terminal order alone has no direction contract, so
        // its pins are distributed left and right over the full body height.
        // The body is drawn one pin pitch past them, keeping every lead on
        // the outline rather than at a corner.
        let pin_count = if cell.terminal_order.is_empty() {
            self.kind.terminal_count()
        } else {
            cell.terminal_order.len()
        };
        let (width, height) = self.symbol_dimensions();
        let half_width = super::symbol_gen::body_half_width(width);
        let half_height =
            super::symbol_gen::body_half_height(height) + super::symbol_gen::GENERATED_STUB;
        let pins = self
            .dynamic_terminal_offsets(pin_count)
            .into_iter()
            .enumerate()
            .map(|(idx, offset)| {
                let left = offset.x < 0;
                GeneratedPin {
                    name: cell.terminal_order.get(idx).cloned().unwrap_or_default(),
                    direction: if left {
                        crate::state::PortDirection::In
                    } else {
                        crate::state::PortDirection::Out
                    },
                    side: if left {
                        crate::state::SymbolPinSide::Left
                    } else {
                        crate::state::SymbolPinSide::Right
                    },
                    offset,
                }
            })
            .collect();
        InstanceBlock {
            pins,
            body: (
                Point::new(-half_width, -half_height),
                Point::new(half_width, half_height),
            ),
        }
    }

    fn dynamic_terminal_offsets(&self, pin_count: usize) -> Vec<Point> {
        if pin_count == 0 {
            return Vec::new();
        }

        let (w, h) = self.symbol_dimensions();
        let hw = w / 2;
        let hh = h / 2;

        if pin_count == 1 {
            return vec![Point::new(-hw, 0)];
        }
        if pin_count == 2 {
            return vec![Point::new(-hw, 0), Point::new(hw, 0)];
        }

        let left_count = pin_count.div_ceil(2);
        let right_count = pin_count - left_count;
        let left_y = Self::distributed_pin_axis(left_count, hh);
        let right_y = Self::distributed_pin_axis(right_count, hh);

        let mut points = Vec::with_capacity(pin_count);
        for y in left_y {
            points.push(Point::new(-hw, y));
        }
        for y in right_y {
            points.push(Point::new(hw, y));
        }
        points
    }

    fn distributed_pin_axis(count: usize, half_height: i32) -> Vec<i32> {
        if count == 0 {
            return Vec::new();
        }
        if count == 1 {
            return vec![0];
        }

        let span = half_height * 2;
        (0..count)
            .map(|idx| -half_height + ((idx as i32 * span) / ((count - 1) as i32)))
            .collect()
    }

    fn terminal_label(idx: usize) -> &'static str {
        const LABELS: [&str; 24] = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
            "17", "18", "19", "20", "21", "22", "23", "24",
        ];
        LABELS.get(idx).copied().unwrap_or("P")
    }

    /// Box that authored artwork for this instance is drawn in, and the
    /// terminal offsets it is matched against.
    ///
    /// Artwork is authored against the nominal block. A body widened to
    /// print long pin names must therefore stretch the leads that reach it,
    /// never the drawing — otherwise every extra character of a port name
    /// would distort the glyph a little further.
    pub(crate) fn artwork_dimensions(&self) -> (i32, i32) {
        let (width, height) = self.symbol_dimensions();
        (width.min(super::symbol_gen::GENERATED_WIDTH), height)
    }

    /// Terminal offsets as authored artwork draws them, in interface order.
    pub(crate) fn artwork_pin_offsets(&self) -> Vec<Point> {
        let (artwork_width, _) = self.artwork_dimensions();
        self.instance_block_pins()
            .into_iter()
            .map(|pin| match pin.side {
                crate::state::SymbolPinSide::Left => Point::new(-artwork_width / 2, pin.offset.y),
                crate::state::SymbolPinSide::Right => Point::new(artwork_width / 2, pin.offset.y),
                crate::state::SymbolPinSide::Top | crate::state::SymbolPinSide::Bottom => {
                    pin.offset
                }
            })
            .collect()
    }

    /// Leads that carry authored artwork out to terminals it does not reach,
    /// as `(artwork edge, terminal)` pairs in unrotated symbol coordinates.
    pub(crate) fn artwork_lead_extensions(&self) -> Vec<(Point, Point)> {
        self.artwork_pin_offsets()
            .into_iter()
            .zip(self.instance_block_pins())
            .filter(|(edge, pin)| *edge != pin.offset)
            .map(|(edge, pin)| (edge, pin.offset))
            .collect()
    }

    /// Effective symbol dimensions, allowing generic block scaling for bound library cells.
    pub(crate) fn symbol_dimensions(&self) -> (i32, i32) {
        if let Some(cell) = &self.library_cell {
            if let Some(ports) = cell.interface() {
                return super::symbol_gen::generate_symbol(&ports).dimensions();
            }
            let pin_count = if cell.terminal_order.is_empty() {
                2
            } else {
                cell.terminal_order.len()
            };
            let rows = pin_count.div_ceil(2).max(2) as i32;
            let height = (rows * 20).max(40);
            return (60, height);
        }
        self.kind.symbol_dimensions()
    }

    /// Transform a point from component-local coordinates to world coordinates
    ///
    /// Applies both mirror and rotation transforms in the correct order:
    /// 1. Apply mirror (about component origin)
    /// 2. Apply rotation (about component origin)
    ///
    /// This matches the standard EDA convention (Cadence Virtuoso, etc.)
    pub fn transform_point(&self, p: Point) -> Point {
        // Step 1: Apply mirror transforms
        let x = if self.mirror_h {
            p.x.saturating_neg()
        } else {
            p.x
        };
        let y = if self.mirror_v {
            p.y.saturating_neg()
        } else {
            p.y
        };
        let mirrored = Point::new(x, y);

        // Step 2: Apply rotation
        match self.rotation {
            Rotation::R0 => mirrored,
            Rotation::R90 => Point::new(mirrored.y.saturating_neg(), mirrored.x),
            Rotation::R180 => Point::new(mirrored.x.saturating_neg(), mirrored.y.saturating_neg()),
            Rotation::R270 => Point::new(mirrored.y, mirrored.x.saturating_neg()),
        }
    }

    /// Rotate a point by the component's rotation (legacy method)
    ///
    /// NOTE: For new code, prefer transform_point() which also applies mirror.
    /// This method is kept for backward compatibility.
    #[inline]
    pub fn rotate_point(&self, p: Point) -> Point {
        match self.rotation {
            Rotation::R0 => p,
            Rotation::R90 => Point::new(p.y.saturating_neg(), p.x),
            Rotation::R180 => Point::new(p.x.saturating_neg(), p.y.saturating_neg()),
            Rotation::R270 => Point::new(p.y, p.x.saturating_neg()),
        }
    }

    /// Get the bounding box of this component in grid coordinates
    ///
    /// Returns (min_x, min_y, max_x, max_y) representing the
    /// footprint of the component symbol, matching the actual rendered size.
    pub fn bounding_box(&self) -> (i32, i32, i32, i32) {
        // Use actual symbol dimensions for accurate hit detection
        let (width, height) = self.symbol_dimensions();
        let half_w = width / 2;
        let half_h = height / 2;

        // Swap dimensions if rotated 90 or 270 degrees
        let (hw, hh) = if self.rotation.is_vertical() {
            (half_h, half_w)
        } else {
            (half_w, half_h)
        };

        (
            self.pos.x - hw,
            self.pos.y - hh,
            self.pos.x + hw,
            self.pos.y + hh,
        )
    }

    /// Check if a grid point is within this component's bounding box
    pub fn contains_point(&self, p: Point) -> bool {
        let (min_x, min_y, max_x, max_y) = self.bounding_box();
        p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
    }

    /// Get the SPICE netlist line for this component
    ///
    /// Note: This is a simplified version. Full netlist generation
    /// requires node connectivity information.
    pub fn spice_instance_name(&self) -> String {
        if self.name.is_empty() {
            format!("{}{}", self.kind.spice_prefix(), self.id)
        } else {
            self.name.clone()
        }
    }

    /// Validate against the effective emitted device prefix. Model-bound
    /// library cells own their declared primitive prefix rather than the
    /// generic `X` prefix of ordinary hierarchical instances.
    pub fn validate_reference_designator(&self, value: &str) -> Result<(), String> {
        let prefix = self
            .library_cell
            .as_ref()
            .filter(|binding| binding.netlist_template.is_some() || binding.is_executable_builtin())
            .and_then(LibraryCellInstance::effective_reference_prefix)
            .unwrap_or_else(|| self.kind.spice_prefix());
        if prefix.is_empty() {
            return Err(
                "This component type does not own a SPICE reference designator.".to_owned(),
            );
        }
        if !value
            .get(..prefix.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
        {
            return Err(format!(
                "{} designators must begin with `{prefix}`.",
                self.kind.display_name()
            ));
        }
        let suffix = &value[prefix.len()..];
        // An iterated designator such as `X<0:3>` asks one drawn instance to
        // stand for four. RSpice places instances one for one, so the name is
        // refused where it is typed rather than netlisted as a single instance
        // whose name the engine would then mangle.
        if suffix.contains(['<', '>', '[', ']']) {
            return Err(ITERATED_INSTANCE_REJECTION.to_owned());
        }
        if suffix.is_empty()
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(format!(
                "Enter `{prefix}` followed by one or more ASCII letters, digits, or underscores."
            ));
        }
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_display_mode_parses_mockup_values_and_inherits_canvas_policy() {
        assert_eq!(
            ComponentDisplayMode::parse("name-and-value"),
            Some(ComponentDisplayMode::NameAndValue)
        );
        assert_eq!(
            ComponentDisplayMode::parse("value only"),
            Some(ComponentDisplayMode::ValueOnly)
        );
        assert!(ComponentDisplayMode::parse("sometimes").is_none());
        assert!(
            !ComponentDisplayMode::Inherit
                .show_name(crate::state::SchematicParameterLabelVisibility::ValuesOnly)
        );
        assert!(
            ComponentDisplayMode::Inherit
                .show_value(crate::state::SchematicParameterLabelVisibility::ValuesOnly)
        );
        assert!(
            !ComponentDisplayMode::Hidden
                .show_value(crate::state::SchematicParameterLabelVisibility::NamesAndValues)
        );
    }
    use crate::state::{
        Cell, ComponentType, Library, LibraryManager, PortDirection, PortSpec, SchematicState,
        SymbolDocument, SymbolPin, SymbolResolver, View, ViewType,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn resolved_amp_symbol() -> crate::state::ResolvedCellSymbol {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };

        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let buffers = HashMap::<String, SchematicState>::new();
        SymbolResolver::new(&libraries, &buffers)
            .resolve_binding(&binding)
            .expect("symbol resolves")
    }

    #[test]
    fn resolved_instance_terminals_use_authored_symbol_offsets() {
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        let component = Component::new(7, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding)
            .with_rotation(Rotation::R90)
            .with_mirror_h(true);
        let resolved = resolved_amp_symbol();

        let terminals = component.terminal_positions_resolved(Some(&resolved));

        assert_eq!(
            terminals,
            vec![
                ("IN".to_owned(), Point::new(110, 90)),
                ("OUT".to_owned(), Point::new(80, -20)),
            ]
        );
    }

    #[test]
    fn resolved_instance_terminals_are_relative_to_symbol_origin() {
        let document = SymbolDocument {
            origin: Point::new(20, -10),
            pins: vec![SymbolPin::new(
                "OUT",
                PortDirection::Out,
                Some(Point::new(70, 20)),
            )],
            ..SymbolDocument::default()
        };
        let resolved = crate::state::ResolvedCellSymbol::from_authored_document(
            document,
            &[port("OUT", PortDirection::Out)],
        );
        let component = Component::new(8, ComponentType::CellInstance, Point::new(100, 50));

        let terminals = component.terminal_positions_resolved(Some(&resolved));

        assert_eq!(terminals, vec![("OUT".to_owned(), Point::new(150, 80))]);
    }

    #[test]
    fn model_bound_template_grammar_is_single_line_and_positional() {
        assert!(validate_library_netlist_template("M{name} {nodes} {model} {params}").is_ok());
        assert!(validate_library_netlist_template("{ref} {nodes} {model}").is_ok());
        for invalid in [
            ".include {nodes} {model}",
            "M{name} {model} {nodes}",
            "M{name} {nodes} {model}\n.end",
            "M{name} {nodes} {model} fixed=1",
            "M{name} {nodes} {model} {params} {params}",
        ] {
            assert!(
                validate_library_netlist_template(invalid).is_err(),
                "unexpectedly accepted {invalid:?}"
            );
        }
    }

    #[test]
    fn model_bound_instance_uses_its_primitive_reference_prefix() {
        let mut binding = LibraryCellInstance::new("models", "nmos_18", "spice");
        binding.netlist_template = Some("M{name} {nodes} {model} {params}".to_owned());
        binding.reference_prefix = Some("M".to_owned());
        let component = Component::new(1, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding)
            .with_name_value("M17", "nmos_18");

        assert!(component.validate_reference_designator("M17").is_ok());
        assert!(component.validate_reference_designator("X17").is_err());
        assert!(component.validate_reference_designator("M").is_err());
    }

    #[test]
    fn multiplicity_accepts_every_value_the_flattener_does() {
        assert_eq!(InstanceMultiplicity::parse("4").unwrap().value(), 4.0);
        assert_eq!(InstanceMultiplicity::parse(" 0.5 ").unwrap().value(), 0.5);
        assert_eq!(InstanceMultiplicity::new(4.0).unwrap().to_string(), "4");
        for refused in ["0", "-2", "inf", "nan", "2k", ""] {
            assert!(
                InstanceMultiplicity::parse(refused).is_err(),
                "unexpectedly accepted {refused:?}"
            );
        }
    }

    #[test]
    fn legacy_cell_instance_multiplicity_survives_as_a_typed_field() {
        let mut component = Component::new(5, ComponentType::CellInstance, Point::origin())
            .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"))
            .with_name_value("X5", "amp");
        component.params = "m=3 gain=2".to_owned();

        let mut persisted = serde_json::to_value(&component).expect("component encodes");
        persisted
            .as_object_mut()
            .expect("component encodes as an object")
            .remove("multiplicity");
        let loaded: Component = serde_json::from_value(persisted).expect("legacy component loads");

        assert_eq!(
            loaded.multiplicity.map(InstanceMultiplicity::value),
            Some(3.0)
        );
        assert_eq!(loaded.params, "gain=2");
    }

    #[test]
    fn a_bound_interface_reports_its_conductors_per_terminal() {
        let mut binding = LibraryCellInstance::new("work", "reg4", "schematic");
        binding.terminal_order = vec![
            "DATA[3:0]".to_owned(),
            "EN".to_owned(),
            "ADDR<0:2>".to_owned(),
        ];

        assert_eq!(binding.terminal_widths(), [4, 1, 3]);
        assert_eq!(binding.terminal_node_count(), 8);

        // A placement written before vectors existed carries scalar names, so
        // it reports one conductor per terminal without a migration.
        let legacy = LibraryCellInstance::new("work", "amp", "schematic");
        assert!(legacy.terminal_widths().is_empty());
        assert_eq!(legacy.terminal_node_count(), 0);
    }

    #[test]
    fn an_iterated_designator_is_refused_in_the_words_the_product_uses() {
        let instance = Component::new(9, ComponentType::CellInstance, Point::origin())
            .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"))
            .with_name_value("X9", "amp");

        for iterated in ["X<0:3>", "X[0:3]", "X0<3>"] {
            assert_eq!(
                instance.validate_reference_designator(iterated),
                Err(ITERATED_INSTANCE_REJECTION.to_owned()),
                "{iterated} was not refused as an iterated instance"
            );
        }
        assert!(instance.validate_reference_designator("X9").is_ok());
    }

    #[test]
    fn a_primitive_keeps_its_own_m_parameter_and_an_unusable_one_is_never_dropped() {
        let mut resistor =
            Component::new(6, ComponentType::Resistor, Point::origin()).with_name_value("R6", "1k");
        resistor.params = "m=2".to_owned();
        resistor.adopt_params_multiplicity();
        assert_eq!(resistor.params, "m=2");
        assert!(resistor.multiplicity.is_none());

        let mut instance = Component::new(7, ComponentType::CellInstance, Point::origin())
            .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"))
            .with_name_value("X7", "amp");
        instance.params = "m={copies}".to_owned();
        instance.adopt_params_multiplicity();
        assert_eq!(instance.params, "m={copies}");
        assert!(instance.multiplicity.is_none());
    }
}

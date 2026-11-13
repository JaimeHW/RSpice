//! The data browser's instance tree and its query: a signal's hierarchical
//! name split into the instances that own it, the kind of quantity it is, and
//! the glob a reader filters the list with.

use rspice_vocabulary::natural_order::natural_cmp;

/// What the browser's query asks for.
///
/// A query that uses glob syntax (`*`, `?`, `[...]`) is a pattern over a
/// signal's whole hierarchical name, anchored at both ends and matched without
/// regard to case, as EZwave and SimVision filter. Plain text is found
/// anywhere in a row's words — its name, its analysis and run, the state words
/// a reader can ask about — as the browser always has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BrowserQuery {
    Empty,
    /// Lowercased text to find anywhere.
    Text(String),
    Glob(Glob),
}

impl BrowserQuery {
    pub(super) fn parse(raw: &str) -> Self {
        let raw = raw.trim();
        if raw.is_empty() {
            Self::Empty
        } else if raw.contains(['*', '?', '[']) {
            Self::Glob(Glob::parse(raw))
        } else {
            Self::Text(raw.to_lowercase())
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Whether a row's own words answer the query. A pattern names signals,
    /// so it never matches a label or a state word.
    pub(super) fn matches_word(&self, word: &str) -> bool {
        match self {
            Self::Empty => true,
            Self::Text(text) => word.to_lowercase().contains(text.as_str()),
            Self::Glob(_) => false,
        }
    }

    /// Whether a signal's name answers the query. A pattern matches the full
    /// name (`V(x1.x2.out)`) or the hierarchical path inside its access
    /// function (`x1.x2.out`), so `x1.*` lists everything under `x1` and
    /// `V(*)` every voltage.
    pub(super) fn matches_name(&self, name: &str) -> bool {
        match self {
            Self::Empty => true,
            Self::Text(text) => name.to_lowercase().contains(text.as_str()),
            Self::Glob(glob) => {
                glob.matches(name) || {
                    let path = hierarchical_path(name);
                    path.len() != name.len() && glob.matches(path)
                }
            }
        }
    }
}

/// One element of a glob pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
enum GlobToken {
    /// `*`: any run of characters, hierarchy separators included.
    Any,
    /// `?`: one character.
    One,
    /// `[...]` or `[!...]`: one character in (or not in) the ranges.
    Class {
        negated: bool,
        ranges: Vec<(char, char)>,
    },
    Literal(char),
}

/// A case-insensitive glob over a whole name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Glob {
    tokens: Vec<GlobToken>,
}

impl Glob {
    pub(super) fn parse(pattern: &str) -> Self {
        let chars = pattern.to_lowercase().chars().collect::<Vec<_>>();
        let mut tokens = Vec::new();
        let mut index = 0;
        while index < chars.len() {
            match chars[index] {
                '*' => {
                    // A run of stars is one star; it matches the same names.
                    if tokens.last() != Some(&GlobToken::Any) {
                        tokens.push(GlobToken::Any);
                    }
                    index += 1;
                }
                '?' => {
                    tokens.push(GlobToken::One);
                    index += 1;
                }
                '[' => match Self::parse_class(&chars[index + 1..]) {
                    Some((token, consumed)) => {
                        tokens.push(token);
                        index += 1 + consumed;
                    }
                    // An unclosed bracket is the character itself.
                    None => {
                        tokens.push(GlobToken::Literal('['));
                        index += 1;
                    }
                },
                literal => {
                    tokens.push(GlobToken::Literal(literal));
                    index += 1;
                }
            }
        }
        Self { tokens }
    }

    /// The class after a `[`, and how many characters it took up to and
    /// including its `]`. A `]` straight after the opening (or after `!` or
    /// `^`) is a member, as in POSIX.
    fn parse_class(chars: &[char]) -> Option<(GlobToken, usize)> {
        let mut index = 0;
        let negated = matches!(chars.first(), Some('!' | '^'));
        if negated {
            index += 1;
        }
        let first_member = index;
        let mut ranges = Vec::new();
        while index < chars.len() {
            let start = chars[index];
            if start == ']' && index > first_member {
                return Some((GlobToken::Class { negated, ranges }, index + 1));
            }
            if chars.get(index + 1) == Some(&'-')
                && let Some(&end) = chars.get(index + 2)
                && end != ']'
            {
                ranges.push((start.min(end), start.max(end)));
                index += 3;
            } else {
                ranges.push((start, start));
                index += 1;
            }
        }
        None
    }

    pub(super) fn matches(&self, name: &str) -> bool {
        let text = name.to_lowercase().chars().collect::<Vec<_>>();
        let tokens = &self.tokens;
        // Iterative matching with one backtrack point per star: linear in
        // practice, and never exponential however many stars a pattern has.
        let (mut t, mut p) = (0, 0);
        let mut backtrack: Option<(usize, usize)> = None;
        while t < text.len() {
            let advanced = match tokens.get(p) {
                Some(GlobToken::Any) => {
                    backtrack = Some((p, t));
                    p += 1;
                    continue;
                }
                Some(GlobToken::One) => true,
                Some(GlobToken::Literal(literal)) => *literal == text[t],
                Some(GlobToken::Class { negated, ranges }) => {
                    ranges
                        .iter()
                        .any(|(low, high)| (*low..=*high).contains(&text[t]))
                        != *negated
                }
                None => false,
            };
            if advanced {
                t += 1;
                p += 1;
            } else if let Some((star, from)) = backtrack {
                p = star + 1;
                t = from + 1;
                backtrack = Some((star, from + 1));
            } else {
                return false;
            }
        }
        tokens[p..].iter().all(|token| *token == GlobToken::Any)
    }
}

/// The hierarchical path inside a name's access function: `x1.x2.out` for
/// `V(x1.x2.out)`, the name itself when it has none. A differential
/// `V(a,b)` names two nodes, so it has no one path and stays whole.
pub(super) fn hierarchical_path(name: &str) -> &str {
    match access_function(name) {
        Some((_, inner)) if !inner.contains(',') => inner,
        _ => name,
    }
}

/// `("V", "x1.out")` for `V(x1.out)`: a SPICE access function is a short
/// run of letters wrapping the whole rest of the name.
fn access_function(name: &str) -> Option<(&str, &str)> {
    let open = name.find('(')?;
    let function = &name[..open];
    let inner = name[open + 1..].strip_suffix(')')?;
    (!function.is_empty()
        && function.len() <= 4
        && function.chars().all(|c| c.is_ascii_alphabetic()))
    .then_some((function, inner))
}

/// A signal's place in the design: the instances that own it, outermost
/// first, and the name it has inside the innermost of them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HierarchicalName {
    pub(super) instances: Vec<String>,
    /// The name inside its instance, keeping the access function:
    /// `V(out)` for `V(x1.x2.out)`.
    pub(super) leaf: String,
}

/// Split a name at its hierarchy separators: `.` as SPICE subcircuit paths
/// write them, and `:` as Xyce does. A separator inside a bus range
/// (`b[7:0]`) is not one, a device parameter (`@m1[id]`) is not split, and a
/// name that would leave an empty segment is kept whole.
pub(super) fn split_hierarchy(name: &str) -> HierarchicalName {
    let whole = || HierarchicalName {
        instances: Vec::new(),
        leaf: name.to_owned(),
    };
    let path = hierarchical_path(name);
    if path.starts_with('@') {
        return whole();
    }
    let mut segments = Vec::new();
    let mut depth = 0_usize;
    let mut start = 0;
    for (index, character) in path.char_indices() {
        match character {
            '[' | '(' => depth += 1,
            ']' | ')' => depth = depth.saturating_sub(1),
            '.' | ':' if depth == 0 => {
                segments.push(&path[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    segments.push(&path[start..]);
    if segments.len() < 2 || segments.iter().any(|segment| segment.is_empty()) {
        return whole();
    }
    let leaf_path = segments.pop().unwrap_or_default();
    let leaf = match access_function(name) {
        Some((function, _)) if path.len() != name.len() => format!("{function}({leaf_path})"),
        _ => leaf_path.to_owned(),
    };
    HierarchicalName {
        instances: segments.into_iter().map(str::to_owned).collect(),
        leaf,
    }
}

/// Whether `name` is a node the simulator made for itself rather than one the
/// design names: a device's internal node, such as a MOSFET's gate behind its
/// gate resistance (`xldo.MP.__gint`), or the branch of a resistor the
/// simulator inserted (`I(xldo.MP.__rg)`).
///
/// Nothing in a result marks a node as generated, so the name decides it. The
/// engine names every node and element it generates `<instance>.__<role>`
/// (`__dint`, `__sint`, `__gint`, `__body.internal`, `__rd`, `__rg`), and a
/// Verilog-A module's own unknowns that the compiler named begin `__` too
/// (`is_compiler_owned_name` in the engine's builder). A design's own net can
/// not take that form without its author choosing it, so any segment of the
/// hierarchical path that begins `__` marks the whole quantity generated.
pub(super) fn is_generated_node(name: &str) -> bool {
    let path = hierarchical_path(name);
    if path.starts_with('@') {
        return false;
    }
    let mut depth = 0_usize;
    let mut segment_start = true;
    let mut chars = path.chars().peekable();
    while let Some(character) = chars.next() {
        match character {
            '[' | '(' => depth += 1,
            ']' | ')' => depth = depth.saturating_sub(1),
            '.' | ':' if depth == 0 => {
                segment_start = true;
                continue;
            }
            '_' if segment_start && depth == 0 && chars.peek() == Some(&'_') => return true,
            _ => {}
        }
        segment_start = false;
    }
    false
}

/// What kind of quantity a browser row is, by the mark its row wears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SignalKind {
    Voltage,
    Current,
    Digital,
    Real,
    Bus,
    /// Any other quantity, by its own short mark: `°` for a phase, `dB` for
    /// a magnitude, `P` for a power, `N` for a noise density.
    Other(&'static str),
}

impl SignalKind {
    /// A waveform's kind: an event net's access function first (`D()` is a
    /// logic net, `E()` a real one, whatever unit its column reads in), then
    /// the kind the unit gives it.
    pub(super) fn of_waveform(name: &str, kind_label: &str) -> Self {
        match access_function(name).map(|(function, _)| function) {
            Some("D" | "d") => return Self::Digital,
            Some("E" | "e") => return Self::Real,
            _ => {}
        }
        match kind_label {
            "Voltage" => Self::Voltage,
            "Current" | "Current phase" => Self::Current,
            "Phase" => Self::Other("°"),
            "Power" => Self::Other("P"),
            "Magnitude" => Self::Other("dB"),
            "Noise PSD" => Self::Other("N"),
            _ => Self::Other("~"),
        }
    }

    /// The row's mark: one or two characters in the kind column.
    pub(super) const fn glyph(self) -> &'static str {
        match self {
            Self::Voltage => "V",
            Self::Current => "I",
            Self::Digital => "D",
            Self::Real => "R",
            Self::Bus => "B",
            Self::Other(glyph) => glyph,
        }
    }

    /// Where the kind lists among its siblings: node voltages, branch
    /// currents, then the event side, a bus ahead of the nets it gathers.
    pub(super) const fn rank(self) -> u8 {
        match self {
            Self::Voltage => 0,
            Self::Current => 1,
            Self::Bus => 2,
            Self::Digital => 3,
            Self::Real => 4,
            Self::Other(_) => 5,
        }
    }

    /// What the mark says, for the row's tooltip and its spoken name.
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Voltage => "Voltage",
            Self::Current => "Current",
            Self::Digital => "Digital net",
            Self::Real => "Real net",
            Self::Bus => "Digital bus",
            Self::Other("°") => "Phase",
            Self::Other("P") => "Power",
            Self::Other("dB") => "Magnitude",
            Self::Other("N") => "Noise density",
            Self::Other(_) => "Quantity",
        }
    }
}

/// A leaf of one analysis's tree: a signal or an artifact, by its index in
/// the analysis's filtered lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TreeItem {
    Signal(usize),
    Artifact(usize),
}

/// One instance of the design and everything listed under it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct InstanceNode {
    /// The instance's own name, as its first signal spells it, or the first
    /// that spells it with lowercase letters when that one was in capitals.
    pub(super) name: String,
    /// Its path from the top, lowercased and joined with `.`: SPICE names
    /// are case-blind, so `X1.mid` and `x1.n1` share one instance.
    pub(super) key: String,
    /// Its path from the top as the design spells it, for the row's tooltip.
    pub(super) path: String,
    pub(super) children: Vec<InstanceNode>,
    pub(super) items: Vec<TreeItem>,
    /// Leaves under this instance, at every depth.
    pub(super) count: usize,
}

impl InstanceNode {
    /// Put every level in the order a hierarchy browser lists it: instances
    /// by name, then quantities by kind and by name, numbers counted (`L2`
    /// before `L10`). `leaf` gives an item's kind rank and its name inside
    /// its instance.
    pub(super) fn sort(&mut self, leaf: &impl Fn(TreeItem) -> (u8, String)) {
        let mut keyed = self
            .items
            .drain(..)
            .map(|item| (leaf(item), item))
            .collect::<Vec<_>>();
        keyed.sort_by(|((left_rank, left), _), ((right_rank, right), _)| {
            left_rank
                .cmp(right_rank)
                .then_with(|| natural_cmp(left, right))
        });
        self.items = keyed.into_iter().map(|(_, item)| item).collect();
        self.children
            .sort_by(|left, right| natural_cmp(&left.name, &right.name));
        for child in &mut self.children {
            child.sort(leaf);
        }
    }

    /// Every leaf under this node, in the order the tree lists them.
    pub(super) fn leaves(&self) -> Vec<TreeItem> {
        let mut leaves = Vec::with_capacity(self.count);
        self.collect_leaves(&mut leaves);
        leaves
    }

    fn collect_leaves(&self, into: &mut Vec<TreeItem>) {
        for child in &self.children {
            child.collect_leaves(into);
        }
        into.extend(self.items.iter().copied());
    }
}

/// Build one analysis's instance tree from its leaves' hierarchical names.
/// Instances and leaves keep the order they first appear in, which is the
/// order the run retained them, until [`InstanceNode::sort`] orders them;
/// the root holds the top-level leaves.
pub(super) fn build_tree<'a>(
    leaves: impl IntoIterator<Item = (TreeItem, &'a str)>,
) -> InstanceNode {
    let mut root = InstanceNode::default();
    for (item, name) in leaves {
        let split = split_hierarchy(name);
        let mut node = &mut root;
        node.count += 1;
        for instance in &split.instances {
            let (key, path) = if node.key.is_empty() {
                (instance.to_lowercase(), instance.clone())
            } else {
                (
                    format!("{}.{}", node.key, instance.to_lowercase()),
                    format!("{}.{instance}", node.path),
                )
            };
            let position = match node.children.iter().position(|child| child.key == key) {
                Some(position) => {
                    // SPICE hands back the names it folds in capitals, so a
                    // spelling with lowercase letters in it is the design's.
                    let child = &mut node.children[position];
                    if !child.name.chars().any(char::is_lowercase)
                        && instance.chars().any(char::is_lowercase)
                    {
                        let parent = child.path.len() - child.name.len();
                        child.path = format!("{}{instance}", &child.path[..parent]);
                        child.name = instance.clone();
                    }
                    position
                }
                None => {
                    node.children.push(InstanceNode {
                        name: instance.clone(),
                        key,
                        path,
                        ..InstanceNode::default()
                    });
                    node.children.len() - 1
                }
            };
            node = &mut node.children[position];
            node.count += 1;
        }
        node.items.push(item);
    }
    root
}

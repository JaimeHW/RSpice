//! Validated bus and tap edits bound to their borrowed document.

use super::{
    bus::{
        Bus, BusDeclaration, BusParseError, BusPropertyImpact, BusSlice, BusTap, BusTapOrientation,
        BusTargetKind,
    },
    document::SchematicDocument,
    identity::SchematicIdentity,
    wire::Wire,
};
use rspice_design_model::Point;

/// Source attachment, destination and orientation for a tap request.
#[derive(Debug, Clone, Copy)]
pub struct BusTapGeometry {
    pub bus_id: u64,
    pub bus_point: Point,
    pub connection_point: Point,
    pub orientation: BusTapOrientation,
}

/// Validated placement. The exclusive borrows remain held until commit or drop.
#[derive(Debug)]
pub struct BusPlacement<'a> {
    document: &'a mut SchematicDocument,
    identity: &'a mut SchematicIdentity,
    candidate: Bus,
}

impl<'a> BusPlacement<'a> {
    pub fn prepare(
        document: &'a mut SchematicDocument,
        identity: &'a mut SchematicIdentity,
        points: Vec<Point>,
        declaration: Option<BusDeclaration>,
    ) -> Result<Self, BusParseError> {
        let candidate = Bus::new(0, points, declaration)?;
        Ok(Self {
            document,
            identity,
            candidate,
        })
    }

    pub fn document(&self) -> &SchematicDocument {
        self.document
    }

    pub fn commit(self) -> u64 {
        let id = self.identity.allocate(self.document);
        self.document.buses.push(Bus {
            id,
            ..self.candidate
        });
        id
    }
}

/// A validated tap and any declaration adopted by its unnamed source bus.
#[derive(Debug)]
pub struct BusTapPlacement<'a> {
    document: &'a mut SchematicDocument,
    identity: &'a mut SchematicIdentity,
    candidate: BusTap,
    declaration: Option<&'a BusDeclaration>,
}

impl<'a> BusTapPlacement<'a> {
    pub fn prepare(
        document: &'a mut SchematicDocument,
        identity: &'a mut SchematicIdentity,
        geometry: BusTapGeometry,
        slice: BusSlice,
    ) -> Result<Self, BusParseError> {
        let bus = document
            .buses
            .iter()
            .find(|bus| bus.id == geometry.bus_id)
            .ok_or(BusParseError::InvalidBusReference)?;
        let candidate = BusTap::new(
            0,
            bus,
            geometry.bus_point,
            geometry.connection_point,
            slice,
            geometry.orientation,
        )?;
        Ok(Self {
            document,
            identity,
            candidate,
            declaration: None,
        })
    }

    pub fn prepare_configured(
        document: &'a mut SchematicDocument,
        identity: &'a mut SchematicIdentity,
        geometry: BusTapGeometry,
        declaration: &'a BusDeclaration,
        slice: &BusSlice,
    ) -> Result<Self, BusParseError> {
        let source = document
            .buses
            .iter()
            .find(|bus| bus.id == geometry.bus_id)
            .ok_or(BusParseError::InvalidBusReference)?;
        if let Some(existing) = &source.declaration
            && existing != declaration
        {
            return Err(BusParseError::DeclarationMismatch);
        }
        let mut typed_source = source.clone();
        typed_source.declaration = Some(declaration.clone());
        let candidate = BusTap::new(
            0,
            &typed_source,
            geometry.bus_point,
            geometry.connection_point,
            slice.clone(),
            geometry.orientation,
        )?;
        Ok(Self {
            document,
            identity,
            candidate,
            declaration: Some(declaration),
        })
    }

    pub fn document(&self) -> &SchematicDocument {
        self.document
    }

    pub fn commit(self) -> u64 {
        if let Some(declaration) = self.declaration
            && let Some(source) = self
                .document
                .buses
                .iter_mut()
                .find(|bus| bus.id == self.candidate.bus_id)
        {
            source
                .declaration
                .get_or_insert_with(|| declaration.clone());
        }
        let id = self.identity.allocate(self.document);
        self.document.bus_taps.push(BusTap {
            id,
            ..self.candidate
        });
        id
    }
}

/// A checked connected-network property edit, bound to its original document.
#[derive(Debug)]
pub struct BusPropertyEdit<'a> {
    document: &'a mut SchematicDocument,
    buses: Vec<Bus>,
    taps: Vec<BusTap>,
}

impl<'a> BusPropertyEdit<'a> {
    pub fn prepare(
        document: &'a mut SchematicDocument,
        expected: &Bus,
        declaration: Option<&BusDeclaration>,
    ) -> Result<Option<Self>, BusParseError> {
        let (buses, taps, impact) = build_bus_property_candidates(document, expected, declaration)?;
        if !impact.has_changes() {
            return Ok(None);
        }
        Ok(Some(Self {
            document,
            buses,
            taps,
        }))
    }

    pub fn document(&self) -> &SchematicDocument {
        self.document
    }

    pub fn commit(self) {
        self.document.buses = self.buses;
        self.document.bus_taps = self.taps;
    }
}

/// A checked tap edit; its source and destination cannot change before commit.
#[derive(Debug)]
pub struct BusTapPropertyEdit<'a> {
    document: &'a mut SchematicDocument,
    candidate: BusTap,
}

impl<'a> BusTapPropertyEdit<'a> {
    pub fn prepare(
        document: &'a mut SchematicDocument,
        expected: &BusTap,
        geometry: BusTapGeometry,
        slice: BusSlice,
    ) -> Result<Option<Self>, BusParseError> {
        let candidate = build_bus_tap_property_candidate(document, expected, geometry, slice)?;
        if &candidate == expected {
            return Ok(None);
        }
        Ok(Some(Self {
            document,
            candidate,
        }))
    }

    pub fn document(&self) -> &SchematicDocument {
        self.document
    }

    pub fn commit(self) {
        let tap_id = self.candidate.id;
        if let Some(tap) = self
            .document
            .bus_taps
            .iter_mut()
            .find(|tap| tap.id == tap_id)
        {
            *tap = self.candidate;
        }
    }
}

/// Validate a bus-network edit without changing the document.
pub fn validate_bus_properties(
    document: &SchematicDocument,
    expected: &Bus,
    declaration: Option<&BusDeclaration>,
) -> Result<BusPropertyImpact, BusParseError> {
    let (_, _, impact) = build_bus_property_candidates(document, expected, declaration)?;
    Ok(impact)
}

/// Validate a tap edit, including its expected object and destination contract.
pub fn validate_bus_tap_properties(
    document: &SchematicDocument,
    expected: &BusTap,
    geometry: BusTapGeometry,
    slice: BusSlice,
) -> Result<bool, BusParseError> {
    let candidate = build_bus_tap_property_candidate(document, expected, geometry, slice)?;
    Ok(&candidate != expected)
}

fn build_bus_property_candidates(
    document: &SchematicDocument,
    expected: &Bus,
    declaration: Option<&BusDeclaration>,
) -> Result<(Vec<Bus>, Vec<BusTap>, BusPropertyImpact), BusParseError> {
    let current = document
        .buses
        .iter()
        .find(|bus| bus.id == expected.id)
        .ok_or(BusParseError::InvalidBusReference)?;
    if current != expected {
        return Err(BusParseError::StaleObject);
    }
    let selected_direction_reversed = current
        .declaration
        .as_ref()
        .zip(declaration)
        .is_some_and(|(before, after)| before.direction() != after.direction());
    let mut candidate_buses = document.buses.clone();
    let candidate = candidate_buses
        .iter_mut()
        .find(|bus| bus.id == expected.id)
        .expect("validated bus identity remains present");
    candidate.declaration = declaration.cloned();
    candidate.validate()?;

    let connectivity = bus_connectivity(document, expected.id);
    let connected = &connectivity.connected;
    let mut candidate_taps = document.bus_taps.clone();
    let Some(selected_declaration) = declaration else {
        let has_dependency = document.bus_taps.iter().any(|tap| {
            tap.bus_id == expected.id
                || (tap.target_kind() == BusTargetKind::Bus
                    && connectivity
                        .targets_by_tap
                        .get(&tap.id)
                        .is_some_and(|targets| targets.contains(&expected.id)))
        });
        if has_dependency {
            return Err(BusParseError::UndeclaredBus);
        }
        let impact = property_impact(document, &candidate_buses, &candidate_taps, connected.len());
        return Ok((candidate_buses, candidate_taps, impact));
    };

    for bus in &mut candidate_buses {
        if bus.id == expected.id || !connected.contains(&bus.id) {
            continue;
        }
        let declaration = bus
            .declaration
            .as_mut()
            .ok_or(BusParseError::UndeclaredBus)?;
        declaration.name.clone_from(&selected_declaration.name);
        declaration.notation = selected_declaration.notation;
        // Apply the selected bus's direction *delta* to the whole connected
        // vector network. A pure rename/notation edit must preserve each
        // connected declaration's own orientation; an intentional reversal
        // reverses every connected declaration exactly once.
        if selected_direction_reversed {
            std::mem::swap(&mut declaration.msb, &mut declaration.lsb);
        }
        declaration.validate()?;
    }

    let bus_indices: std::collections::HashMap<u64, usize> = candidate_buses
        .iter()
        .enumerate()
        .map(|(index, bus)| (bus.id, index))
        .collect();
    for tap in &mut candidate_taps {
        if !connected.contains(&tap.bus_id) {
            continue;
        }
        let source = bus_indices
            .get(&tap.bus_id)
            .map(|index| &candidate_buses[*index])
            .ok_or(BusParseError::InvalidBusReference)?;
        let source_before = document
            .buses
            .iter()
            .find(|bus| bus.id == tap.bus_id)
            .ok_or(BusParseError::InvalidBusReference)?;
        let source_declaration = source
            .declaration
            .as_ref()
            .ok_or(BusParseError::UndeclaredBus)?;
        if tap.target_kind() == BusTargetKind::Bus {
            let targets = connectivity
                .targets_by_tap
                .get(&tap.id)
                .map(Vec::as_slice)
                .unwrap_or_default();
            match targets {
                [] => {
                    let source_direction_reversed = source_before
                        .declaration
                        .as_ref()
                        .is_some_and(|before| before.direction() != source_declaration.direction());
                    tap.slice.name.clone_from(&source_declaration.name);
                    tap.slice.notation = source_declaration.notation;
                    if source_direction_reversed {
                        std::mem::swap(&mut tap.slice.msb, &mut tap.slice.lsb);
                    }
                }
                [target_id] => {
                    let target_declaration = bus_indices
                        .get(target_id)
                        .map(|index| &candidate_buses[*index])
                        .ok_or(BusParseError::InvalidDestination)?
                        .declaration
                        .as_ref()
                        .ok_or(BusParseError::InvalidDestination)?;
                    tap.slice.name.clone_from(&target_declaration.name);
                    tap.slice.msb = target_declaration.msb;
                    tap.slice.lsb = target_declaration.lsb;
                    tap.slice.notation = target_declaration.notation;
                }
                _ => return Err(BusParseError::InvalidDestination),
            }
        } else {
            tap.slice.name.clone_from(&source_declaration.name);
            tap.slice.notation = source_declaration.notation;
        }
        tap.validate_against_bus(source)?;
        validate_tap_destination(&document.wires, &candidate_buses, tap)?;
    }
    let impact = property_impact(document, &candidate_buses, &candidate_taps, connected.len());
    Ok((candidate_buses, candidate_taps, impact))
}

fn build_bus_tap_property_candidate(
    document: &SchematicDocument,
    expected: &BusTap,
    geometry: BusTapGeometry,
    slice: BusSlice,
) -> Result<BusTap, BusParseError> {
    let current = document
        .bus_taps
        .iter()
        .find(|tap| tap.id == expected.id)
        .ok_or(BusParseError::InvalidBusReference)?;
    if current != expected {
        return Err(BusParseError::StaleObject);
    }
    let bus = document
        .buses
        .iter()
        .find(|bus| bus.id == geometry.bus_id)
        .ok_or(BusParseError::InvalidBusReference)?;
    let candidate = BusTap::new(
        expected.id,
        bus,
        geometry.bus_point,
        geometry.connection_point,
        slice,
        geometry.orientation,
    )?;
    validate_tap_destination(&document.wires, &document.buses, &candidate)?;
    Ok(candidate)
}

pub fn simplify_polyline(points: Vec<Point>) -> Vec<Point> {
    let mut result = Vec::with_capacity(points.len());
    for point in points {
        if result.last() == Some(&point) {
            continue;
        }
        while result.len() >= 2 {
            let a: Point = result[result.len() - 2];
            let b: Point = result[result.len() - 1];
            let collinear = (i128::from(b.x) - i128::from(a.x))
                * (i128::from(point.y) - i128::from(b.y))
                == (i128::from(b.y) - i128::from(a.y)) * (i128::from(point.x) - i128::from(b.x));
            if !collinear {
                break;
            }
            result.pop();
        }
        result.push(point);
    }
    result
}

#[derive(Debug, Default)]
struct BusConnectivity {
    connected: std::collections::HashSet<u64>,
    targets_by_tap: std::collections::HashMap<u64, Vec<u64>>,
}

fn bus_connectivity(document: &SchematicDocument, seed: u64) -> BusConnectivity {
    let mut adjacency: std::collections::HashMap<u64, Vec<u64>> = document
        .buses
        .iter()
        .map(|bus| (bus.id, Vec::new()))
        .collect();
    let mut targets_by_tap = std::collections::HashMap::new();
    for tap in &document.bus_taps {
        if tap.target_kind() != BusTargetKind::Bus {
            continue;
        }
        let targets: Vec<u64> = document
            .buses
            .iter()
            .filter(|bus| bus.id != tap.bus_id && bus.contains_point(tap.connection_point))
            .map(|bus| bus.id)
            .collect();
        for target in &targets {
            adjacency.entry(tap.bus_id).or_default().push(*target);
            adjacency.entry(*target).or_default().push(tap.bus_id);
        }
        targets_by_tap.insert(tap.id, targets);
    }

    let mut connected = std::collections::HashSet::new();
    let mut pending = std::collections::VecDeque::from([seed]);
    while let Some(bus_id) = pending.pop_front() {
        if !connected.insert(bus_id) {
            continue;
        }
        if let Some(neighbors) = adjacency.get(&bus_id) {
            pending.extend(neighbors.iter().copied());
        }
    }
    BusConnectivity {
        connected,
        targets_by_tap,
    }
}

fn property_impact(
    document: &SchematicDocument,
    candidate_buses: &[Bus],
    candidate_taps: &[BusTap],
    connected_buses: usize,
) -> BusPropertyImpact {
    BusPropertyImpact {
        connected_buses,
        buses_changed: document
            .buses
            .iter()
            .zip(candidate_buses)
            .filter(|(stored, candidate)| stored != candidate)
            .count(),
        taps_changed: document
            .bus_taps
            .iter()
            .zip(candidate_taps)
            .filter(|(stored, candidate)| stored != candidate)
            .count(),
    }
}

fn validate_tap_destination(
    wires: &[Wire],
    buses: &[Bus],
    candidate: &BusTap,
) -> Result<(), BusParseError> {
    let touches_wire = wires
        .iter()
        .any(|wire| wire.contains_point(candidate.connection_point));
    let buses_at_destination: Vec<&Bus> = buses
        .iter()
        .filter(|bus| bus.contains_point(candidate.connection_point))
        .collect();
    let source_collision = buses_at_destination
        .iter()
        .any(|bus| bus.id == candidate.bus_id);
    let destination_buses: Vec<&Bus> = buses_at_destination
        .into_iter()
        .filter(|bus| bus.id != candidate.bus_id)
        .collect();

    if source_collision {
        return Err(BusParseError::InvalidDestination);
    }

    match candidate.target_kind() {
        BusTargetKind::Wire if !touches_wire && destination_buses.is_empty() => Ok(()),
        BusTargetKind::Wire if touches_wire && destination_buses.is_empty() => Ok(()),
        BusTargetKind::Bus if !touches_wire && destination_buses.is_empty() => Ok(()),
        BusTargetKind::Bus
            if !touches_wire
                && destination_buses.len() == 1
                && destination_matches_slice(&destination_buses, &candidate.slice) =>
        {
            Ok(())
        }
        BusTargetKind::Wire | BusTargetKind::Bus => Err(BusParseError::InvalidDestination),
    }
}

fn destination_matches_slice(destination_buses: &[&Bus], slice: &BusSlice) -> bool {
    let Ok(expected) =
        BusDeclaration::new(slice.name.clone(), slice.msb, slice.lsb, slice.notation)
    else {
        return false;
    };
    destination_buses
        .iter()
        .any(|bus| bus.declaration.as_ref() == Some(&expected))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discarded_preparations_preserve_document_and_identity() {
        let mut document = SchematicDocument::default();
        let mut identity = SchematicIdentity::with_cursor(41);
        let points = vec![Point::origin(), Point::new(20, 0)];
        drop(BusPlacement::prepare(&mut document, &mut identity, points.clone(), None).unwrap());
        assert!(document.buses.is_empty());
        assert_eq!(identity.cursor(), 41);

        let bus_id = BusPlacement::prepare(&mut document, &mut identity, points, None)
            .unwrap()
            .commit();
        assert_eq!(bus_id, 41);
        let before_buses = document.buses.clone();
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let slice = BusSlice::parse("DATA[3]").unwrap();
        let geometry = BusTapGeometry {
            bus_id,
            bus_point: Point::new(10, 0),
            connection_point: Point::new(10, 5),
            orientation: BusTapOrientation::Down,
        };
        drop(
            BusTapPlacement::prepare_configured(
                &mut document,
                &mut identity,
                geometry,
                &declaration,
                &slice,
            )
            .unwrap(),
        );
        assert_eq!(document.buses, before_buses);
        assert!(document.bus_taps.is_empty());
        assert_eq!(identity.cursor(), 42);

        drop(
            BusPropertyEdit::prepare(&mut document, &before_buses[0], Some(&declaration))
                .unwrap()
                .unwrap(),
        );
        assert_eq!(document.buses, before_buses);
        assert!(document.bus_taps.is_empty());
        assert_eq!(identity.cursor(), 42);

        let tap_id = BusTapPlacement::prepare_configured(
            &mut document,
            &mut identity,
            geometry,
            &declaration,
            &slice,
        )
        .unwrap()
        .commit();
        assert_eq!(tap_id, 42);
        let before_buses = document.buses.clone();
        let before_taps = document.bus_taps.clone();
        drop(
            BusTapPropertyEdit::prepare(
                &mut document,
                &before_taps[0],
                BusTapGeometry {
                    connection_point: Point::new(10, 6),
                    ..geometry
                },
                slice,
            )
            .unwrap()
            .unwrap(),
        );
        assert_eq!(document.buses, before_buses);
        assert_eq!(document.bus_taps, before_taps);
        assert_eq!(identity.cursor(), 43);
    }
}

//! Validated bus mutations and their atomic history boundaries.
use super::super::bus::{Bus, BusDeclaration, BusParseError, BusSlice, BusTap};
use super::super::bus_edit::{
    BusPlacement, BusPropertyEdit, BusTapGeometry, BusTapPlacement, BusTapPropertyEdit,
};
use super::{DocumentEdit, Schematic};
use rspice_design_model::Point;

impl Schematic {
    pub fn add_bus(
        &mut self,
        points: Vec<Point>,
        declaration: Option<BusDeclaration>,
    ) -> Result<DocumentEdit<u64>, BusParseError> {
        let placement =
            BusPlacement::prepare(&mut self.document, &mut self.identity, points, declaration)?;
        self.history.begin(placement.document(), "draw bus");
        let id = placement.commit();
        self.invalidate_topology();
        Ok(DocumentEdit {
            value: id,
            committed: self.end_operation(),
        })
    }

    pub fn place_bus_tap(
        &mut self,
        geometry: BusTapGeometry,
        slice: BusSlice,
    ) -> Result<DocumentEdit<u64>, BusParseError> {
        let placement =
            BusTapPlacement::prepare(&mut self.document, &mut self.identity, geometry, slice)?;
        self.history.begin(placement.document(), "place bus tap");
        let id = placement.commit();
        self.invalidate_topology();
        Ok(DocumentEdit {
            value: id,
            committed: self.end_operation(),
        })
    }

    pub fn place_configured_bus_tap(
        &mut self,
        geometry: BusTapGeometry,
        declaration: &BusDeclaration,
        slice: &BusSlice,
    ) -> Result<DocumentEdit<u64>, BusParseError> {
        let placement = BusTapPlacement::prepare_configured(
            &mut self.document,
            &mut self.identity,
            geometry,
            declaration,
            slice,
        )?;
        self.history.begin(placement.document(), "place bus tap");
        let id = placement.commit();
        self.invalidate_topology();
        Ok(DocumentEdit {
            value: id,
            committed: self.end_operation(),
        })
    }

    pub fn edit_bus_properties(
        &mut self,
        expected: &Bus,
        declaration: Option<BusDeclaration>,
    ) -> Result<Option<DocumentEdit<()>>, BusParseError> {
        let Some(change) =
            BusPropertyEdit::prepare(&mut self.document, expected, declaration.as_ref())?
        else {
            return Ok(None);
        };
        self.history.begin(change.document(), "edit bus properties");
        change.commit();
        self.invalidate_topology();
        Ok(Some(DocumentEdit {
            value: (),
            committed: self.end_operation(),
        }))
    }

    pub fn edit_bus_tap_properties(
        &mut self,
        expected: &BusTap,
        geometry: BusTapGeometry,
        slice: BusSlice,
    ) -> Result<Option<DocumentEdit<()>>, BusParseError> {
        let Some(change) =
            BusTapPropertyEdit::prepare(&mut self.document, expected, geometry, slice)?
        else {
            return Ok(None);
        };
        self.history
            .begin(change.document(), "edit bus tap properties");
        change.commit();
        self.invalidate_topology();
        Ok(Some(DocumentEdit {
            value: (),
            committed: self.end_operation(),
        }))
    }
}

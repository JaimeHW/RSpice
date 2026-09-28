//! Editor permission checks around design-owned named-net authority.
use crate::state::SchematicState;
use rspice_design::schematic::owned::named_net;
pub(crate) use rspice_design::schematic::owned::named_net::{
    NamedNetTarget, NetMembership, net_name_eq, port_terminal,
};
pub(crate) fn validate_named_net_rename(
    schematic: &SchematicState,
    target: &NamedNetTarget,
    candidate: &str,
) -> Result<String, String> {
    if schematic.read_only {
        return Err("The active schematic is read-only.".to_owned());
    }
    named_net::validate_named_net_rename(schematic.document(), target, candidate)
}
pub(crate) fn apply_named_net_rename(
    schematic: &mut SchematicState,
    target: NamedNetTarget,
    candidate: String,
) -> Result<bool, String> {
    if schematic.read_only {
        return Err("The active schematic is read-only.".to_owned());
    }
    let Some(edit) = named_net::apply_named_net_rename(&mut schematic.design, target, candidate)?
    else {
        return Ok(false);
    };
    schematic.finish_document_edit(edit.committed);
    Ok(edit.committed)
}

//! Where the application can be, and whether it can go there.
//!
//! `surface_catalog` names every canonical surface and its metadata,
//! `surface_route` is the stable address of one — surface plus, when it
//! applies, one exact product object. `navigation` performs a transition and
//! reports what the browser history should do about it; `availability`
//! answers whether a route may be entered at all, which is licensing and
//! capability, not layout.

pub(crate) mod availability;
pub(crate) mod navigation;
pub(crate) mod surface_catalog;
pub(crate) mod surface_route;

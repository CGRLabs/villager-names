//! Villager Names — every villager gets a one-word name from the region it
//! lives in, shown only while you look at it.
//!
//! Ported from the retired native `.hnmod` to a sandboxed wasm `.hmod`. The
//! region tables and the naming rules live in `names` and `rules` (pure,
//! unit-tested); `server` drives them from the authoritative level through the
//! mod API's handles.

mod names;
mod rules;
mod server;

pub use server::VillagerNames;

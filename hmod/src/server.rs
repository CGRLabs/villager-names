//! The server half of Villager Names, written against the new mod API.
//!
//! It runs on the authoritative level (`level_tick_post`, server side only).
//! The retired native build reached into `Game`'s mob list and had to flag each
//! renamed mob for sync by hand, which is why names never reached the window
//! client in 0.1.x. `EntityRef::set_custom_name` sends the change to the
//! players tracking the mob itself, so there is nothing to flag here.

use mod_api::{
    self, EntityRef, Hooks, HostSide, LevelRef, LevelTickPostEvent, LivingRef, Registrar,
    RegistrationContext,
};

use crate::names::REGIONS;
use crate::rules::{self, Decision};

/// The villager type every name is given to.
const VILLAGER: &str = "minecraft:villager";

/// A villager as one pass sees it: what to call it, and where it stands.
struct Seen {
    entity: EntityRef,
    id: u32,
    region: &'static rules::Region,
    /// `Entity.getName`: the custom name, else the type's (its profession).
    resolved: String,
}

/// Take one snapshot of the level's adult villagers.
fn survey(host: &dyn mod_api::Host, level: LevelRef) -> Vec<Seen> {
    let Some(entities) = level.all_entities(host) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for e in entities {
        let kind = e.kind(host).map(|k| k.id(host));
        if kind.as_deref() != Some(VILLAGER) {
            continue;
        }
        // Babies are left unnamed: a name is something a villager grows into,
        // and a named baby reads oddly in a village of named adults.
        let Some(living): Option<LivingRef> = host.living(e) else { continue };
        if living.baby(host).unwrap_or(false) || living.health(host).unwrap_or(0.0) <= 0.0 {
            continue;
        }
        let Some(pos) = e.position(host) else { continue };
        let Some(resolved) = e.name(host) else { continue };

        // The biome the villager stands in, as the game reads it. A villager in
        // a biome with no names of its own takes the first region's.
        let block = [pos[0].floor() as i32, pos[1].floor() as i32, pos[2].floor() as i32];
        let region = level
            .biome_at(host, block)
            .as_deref()
            .and_then(rules::region_of)
            .unwrap_or(REGIONS[0]);

        out.push(Seen { entity: e, id: e.id(), region, resolved });
    }
    out
}

/// One authoritative level tick: name the villagers that have no name.
fn tick(host: &mut dyn mod_api::ServerHost, level: LevelRef) {
    let seen = survey(host, level);

    for s in seen {
        if rules::decide(s.region, s.id, &s.resolved) != Decision::Name {
            continue;
        }
        let name = rules::name_for(s.region, s.id);
        let text = mod_api::Text::literal(name);
        // False for a stale handle or a kind that keeps no name; either way
        // there is nothing more to do for this villager.
        if !matches!(s.entity.set_custom_name(host, Some(text)), Ok(true)) {
            continue;
        }
        // Hover-only, as vanilla draws a name tag: the game shows a name when
        // it is visible at any distance, or when the mob is under the
        // crosshair. This is the latter.
        let _ = s.entity.set_custom_name_visible(host, false);
    }
}

#[derive(Default)]
pub struct VillagerNames;

#[mod_api::horizon_mod]
impl Hooks for VillagerNames {
    fn register(&self, _r: &mut dyn Registrar, _cx: &RegistrationContext) {}

    fn level_tick_post(&self, host: HostSide<'_>, event: &mut LevelTickPostEvent) {
        if let HostSide::Server(host) = host {
            tick(host, event.level);
        }
    }
}

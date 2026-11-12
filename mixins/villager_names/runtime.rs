// Villager Names: every villager gets a name from the region it lives in,
// and keeps the trade it had.
//
// The name is a given name from the region's list, a family name from the
// same region, and the trade the villager keeps: "Elsbeth Ashdown, the
// Fletcher". The trade is part of the name on purpose: Horizon's trade
// screen shows a villager's custom name *instead of* its profession
// (trading.rs: `let name = custom.unwrap_or_else(|| …the profession…)`), so
// naming a villager would hide its job. Putting the trade in the name gives
// it back.
//
// A villager's name never changes once it is given: it is derived from the
// villager's own identity, so the same villager has the same name in every
// session and on every machine. A villager the player named with a name tag
// is left alone.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::mobs::Species;

pub const ID: &str = "villager_names";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str =
    "Every villager gets a name from its own region, and keeps the trade it had.";

/// One region's names.
pub struct Region {
    /// The region's own name, for the logs and the docs.
    pub id: &'static str,
    /// The biomes whose villagers take this region's names.
    pub biomes: &'static [&'static str],
    /// The everyday names: most villagers have one of these.
    pub common: &'static [&'static str],
    /// The less usual ones.
    pub uncommon: &'static [&'static str],
    /// The rare ones: a villager you have to look for.
    pub rare: &'static [&'static str],
    /// The family names the villagers of this region share.
    pub families: &'static [&'static str],
}

/// Every region, in the order they are listed.
pub const REGIONS: &[&Region] = &[
    &crate::villager_names_lowlands::REGION,
    &crate::villager_names_northlands::REGION,
    &crate::villager_names_hills::REGION,
    &crate::villager_names_dunes::REGION,
    &crate::villager_names_savanna::REGION,
    &crate::villager_names_greendeep::REGION,
    &crate::villager_names_underhollow::REGION,
    &crate::villager_names_emberdeep::REGION,
    &crate::villager_names_shore::REGION,
    &crate::villager_names_void::REGION,
];

/// The names we gave, so a villager that changes trade gets its name brought
/// up to date — and one the player renamed is never touched.
static NAMED: LazyLock<Mutex<HashMap<u32, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// The region a biome's villagers belong to. None for a biome with no names
/// of its own: the villager then takes the first region's names.
pub fn region_of(biome: &str) -> Option<&'static Region> {
    REGIONS.iter().copied().find(|r| r.biomes.contains(&biome))
}

/// The names of the region a villager lives in, and the trade it keeps.
///
/// `id` is the villager's entity id: the same villager always gets the same
/// name.
pub fn name_for(region: &Region, id: u32, profession: &str) -> String {
    let seed = mix(id);
    let given = if seed % 20 == 0 {
        // One in twenty is a rare name.
        pick(region.rare, seed >> 5)
    } else if seed % 4 == 0 {
        pick(region.uncommon, seed >> 5)
    } else {
        pick(region.common, seed >> 5)
    };
    let family = pick(region.families, seed >> 11);
    let mut name = format!("{given} {family}");
    // A villager without a trade, and a nitwit, have none to name.
    if !matches!(profession, "" | "none" | "nitwit") {
        let trade = crate::mobs::render::PROFESSIONS
            .iter()
            .find(|p| **p == profession)
            .copied()
            .unwrap_or(profession);
        name.push_str(", the ");
        name.push_str(&capitalise(trade));
    }
    name
}

/// The trade a villager keeps, from its variant.
pub fn profession_of(m: &crate::mobs::Mob) -> &'static str {
    crate::mobs::render::PROFESSIONS
        [m.extra.variant2 as usize % crate::mobs::render::PROFESSIONS.len()]
}

/// Whether a villager is one of ours to name: an adult villager that nobody
/// has named yet.
pub fn wants_a_name(m: &crate::mobs::Mob) -> bool {
    m.kind.species == Species::Villager
        && !m.is_baby()
        && !m.removed
        && m.health > 0.0
}

/// Whether the name on a villager is the one we gave it.
pub fn is_ours(id: u32, name: &str) -> bool {
    NAMED.lock().unwrap().get(&id).map(String::as_str) == Some(name)
}

/// Remember the name we gave a villager.
pub fn remember(id: u32, name: &str) {
    NAMED.lock().unwrap().insert(id, name.to_owned());
}

/// Forget the villagers that are gone, so the list does not grow for ever.
pub fn forget_gone(living: &std::collections::HashSet<u32>) {
    NAMED.lock().unwrap().retain(|id, _| living.contains(id));
}

/// A spread of the id, so neighbouring villagers do not get the same name.
fn mix(id: u32) -> u32 {
    let mut x = id.wrapping_mul(0x9E37_79B9);
    x ^= x >> 15;
    x = x.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 13;
    x
}

fn pick(list: &[&'static str], seed: u32) -> &'static str {
    if list.is_empty() {
        return "";
    }
    list[(seed as usize) % list.len()]
}

fn capitalise(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_region_has_its_names() {
        for region in REGIONS {
            assert!(!region.biomes.is_empty(), "{} has no biomes", region.id);
            assert!(region.common.len() >= 20, "{} has too few common names", region.id);
            assert!(!region.uncommon.is_empty(), "{} has no uncommon names", region.id);
            assert!(!region.rare.is_empty(), "{} has no rare names", region.id);
            assert!(
                region.families.len() >= 12,
                "{} has too few family names",
                region.id
            );
        }
    }

    #[test]
    fn no_region_repeats_a_name() {
        for region in REGIONS {
            for list in [
                region.common,
                region.uncommon,
                region.rare,
                region.families,
            ] {
                let unique: HashSet<&&str> = list.iter().collect();
                assert_eq!(
                    unique.len(),
                    list.len(),
                    "{} repeats a name",
                    region.id
                );
            }
        }
    }

    #[test]
    fn no_biome_is_in_two_regions() {
        let mut seen = HashSet::new();
        for region in REGIONS {
            for biome in region.biomes {
                assert!(seen.insert(*biome), "{} is in two regions", biome);
            }
        }
    }

    #[test]
    fn a_name_is_a_given_name_a_family_and_the_trade() {
        let region = REGIONS[0];
        let name = name_for(region, 12_345, "farmer");
        let (head, trade) = name.split_once(", ").expect("the name has no trade");
        assert!(head.split(' ').count() == 2, "the name has no family name: {name}");
        assert!(region.common.contains(&head.split(' ').next().unwrap())
            || region.uncommon.contains(&head.split(' ').next().unwrap())
            || region.rare.contains(&head.split(' ').next().unwrap()));
        assert_eq!(trade, "the Farmer");
    }

    #[test]
    fn a_villager_without_a_trade_gets_no_epithet() {
        let region = REGIONS[0];
        for profession in ["none", "nitwit", ""] {
            let name = name_for(region, 12_345, profession);
            assert!(!name.contains(','), "a villager with no trade has one: {name}");
        }
    }

    #[test]
    fn the_same_villager_always_has_the_same_name() {
        let region = REGIONS[0];
        assert_eq!(
            name_for(region, 4242, "farmer"),
            name_for(region, 4242, "farmer")
        );
    }

    #[test]
    fn a_rare_name_is_rare() {
        let region = REGIONS[0];
        let rare = (0..2_000)
            .filter(|id| {
                let name = name_for(region, *id, "farmer");
                let given = name.split(' ').next().unwrap_or("");
                region.rare.contains(&given)
            })
            .count();
        // One in twenty is the rule; a few either side of it is the test.
        assert!((50..160).contains(&rare), "{rare} of 2000 took a rare name");
    }
}

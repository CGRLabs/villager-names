//! Pure, host-free rules for Villager Names.
//!
//! A name is derived from the villager's own entity id and the region it stands
//! in, so the same villager always has the same name — in this session, in the
//! next one, and on someone else's machine. Nothing is remembered between
//! ticks: the id is the memory.

use crate::names::REGIONS;

/// One region's names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    /// The region's own name, for the docs and the README.
    pub id: &'static str,
    /// The biomes whose villagers take this region's names.
    pub biomes: &'static [&'static str],
    /// The everyday names: most villagers have one of these.
    pub common: &'static [&'static str],
    /// The less usual ones.
    pub uncommon: &'static [&'static str],
    /// The rare ones: a villager you have to look for.
    pub rare: &'static [&'static str],
    /// The family names older versions put in a name. Kept so those names are
    /// still recognised as ours and shortened to the one word.
    pub families: &'static [&'static str],
}

/// The region a biome's villagers belong to.
///
/// None for a biome with no names of its own: the caller then takes the first
/// region, so no villager is ever left without a name.
pub fn region_of(biome: &str) -> Option<&'static Region> {
    REGIONS.iter().copied().find(|r| r.biomes.contains(&biome))
}

/// The name of a villager: one given name from the region it lives in.
pub fn name_for(region: &Region, id: u32) -> String {
    given_for(region, id).to_owned()
}

/// The given name the id picks from the region's lists.
fn given_for(region: &Region, id: u32) -> &'static str {
    let seed = mix(id);
    if seed % 20 == 0 {
        // One in twenty is a rare name.
        pick(region.rare, seed >> 5)
    } else if seed % 4 == 0 {
        pick(region.uncommon, seed >> 5)
    } else {
        pick(region.common, seed >> 5)
    }
}

/// Whether a name already on a villager is one we gave it: this version's one
/// word, or the `Given Family, the Trade` of an older one. The given and family
/// name are ours even if the trade has since changed.
pub fn named_by_us(region: &Region, id: u32, current: &str) -> bool {
    if current == name_for(region, id) {
        return true;
    }
    let head = current.split_once(", ").map_or(current, |(head, _)| head);
    let seed = mix(id);
    let legacy = format!("{} {}", given_for(region, id), pick(region.families, seed >> 11));
    head == legacy
}

/// The names a villager answers when nobody has named it.
///
/// `Entity.getName` falls back to the type's name, and `Villager.getTypeName`
/// returns its profession's: `entity.minecraft.villager.farmer` in the language
/// table, which reads `Farmer`. Taken from the game's own `en_us.json` rather
/// than typed by hand, so the list is exactly what the fallback can produce.
/// `Villager` appears twice in that file (a profession and the plain type
/// name); once here is enough.
const PROFESSION_NAMES: &[&str] = &[
    "Armorer", "Butcher", "Cartographer", "Cleric", "Farmer", "Fisherman", "Fletcher",
    "Leatherworker", "Librarian", "Mason", "Nitwit", "Shepherd", "Toolsmith", "Villager",
    "Weaponsmith",
];

/// Whether a resolved name means the villager has no name of its own.
///
/// A villager nobody named reads as its profession — `Farmer`, `Librarian` — and
/// a reader that hands back the raw translation key instead spells it
/// `entity.minecraft.villager.farmer`. Both mean "unnamed". A name a player gave
/// is neither, which is what keeps a name tag's work safe.
///
/// The one way to be wrong: a player who names a villager exactly a profession
/// name. That reads as unnamed and is replaced. The alternative — asking the
/// game whether a custom name is set — is not in the API: `EntityRef` exposes
/// the resolved `name` and `custom_name_visible`, never the custom name itself.
pub fn is_unnamed(resolved: &str) -> bool {
    let resolved = resolved.trim();
    resolved.is_empty()
        || resolved.starts_with("entity.")
        || PROFESSION_NAMES.contains(&resolved)
}

/// What to do with one villager this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    /// Leave it exactly as it is.
    Leave,
    /// Write this name, and keep it hidden until looked at.
    Name,
}

/// What one pass over a villager should do, given its resolved name.
///
/// `resolved` is `Entity.getName`: the custom name, else the type's. A name we
/// gave — this version's or an older one's — is replaced with the current one
/// word, because a villager that moved region or was named by an older version
/// should read the same as its neighbours. A player's name is never touched.
pub fn decide(region: &Region, id: u32, resolved: &str) -> Decision {
    let want = name_for(region, id);
    if resolved == want {
        // Already exactly right: nothing to write.
        Decision::Leave
    } else if is_unnamed(resolved) || named_by_us(region, id, resolved) {
        Decision::Name
    } else {
        // Someone else's name: a name tag, another mod, or a player.
        Decision::Leave
    }
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
            assert!(region.families.len() >= 12, "{} has too few family names", region.id);
        }
    }

    #[test]
    fn no_region_repeats_a_name() {
        for region in REGIONS {
            for list in [region.common, region.uncommon, region.rare, region.families] {
                let unique: HashSet<&&str> = list.iter().collect();
                assert_eq!(unique.len(), list.len(), "{} repeats a name", region.id);
            }
        }
    }

    #[test]
    fn no_biome_is_in_two_regions() {
        let mut seen = HashSet::new();
        for region in REGIONS {
            for biome in region.biomes {
                assert!(seen.insert(*biome), "{biome} is in two regions");
            }
        }
    }

    #[test]
    fn a_name_is_one_word_from_the_region() {
        let region = REGIONS[0];
        let name = name_for(region, 12_345);
        assert!(!name.contains(' '), "the name is not one word: {name}");
        assert!(
            region.common.contains(&name.as_str())
                || region.uncommon.contains(&name.as_str())
                || region.rare.contains(&name.as_str()),
            "{name} is not one of the region's names"
        );
    }

    #[test]
    fn every_name_is_one_word() {
        let region = REGIONS[0];
        for id in 0..2_000 {
            let name = name_for(region, id);
            assert!(!name.is_empty() && !name.contains(' '), "not one word: {name}");
        }
    }

    #[test]
    fn the_same_villager_always_has_the_same_name() {
        let region = REGIONS[0];
        assert_eq!(name_for(region, 4242), name_for(region, 4242));
        // Across every region: the name is a function of the id, so a reload or
        // another machine cannot produce a different one.
        for region in REGIONS {
            assert_eq!(name_for(region, 7), name_for(region, 7));
        }
    }

    #[test]
    fn a_rare_name_is_rare() {
        let region = REGIONS[0];
        let rare = (0..2_000)
            .filter(|id| region.rare.contains(&name_for(region, *id).as_str()))
            .count();
        // One in twenty is the rule; a few either side of it is the test.
        assert!((50..160).contains(&rare), "{rare} of 2000 took a rare name");
    }

    #[test]
    fn an_unnamed_villager_reads_as_its_profession() {
        // What `Entity.getName` resolves to for a villager nobody named: the
        // profession's display name, or the raw key where a reader keeps it.
        for profession in PROFESSION_NAMES {
            assert!(is_unnamed(profession), "{profession} should read as unnamed");
        }
        assert!(is_unnamed("entity.minecraft.villager.farmer"));
        assert!(is_unnamed("entity.minecraft.villager.none"));
        assert!(is_unnamed(""));
        assert!(is_unnamed("   "));
    }

    #[test]
    fn a_players_name_is_never_taken_for_unnamed() {
        assert!(!is_unnamed("Bartholomew"));
        assert!(!is_unnamed("Elsbeth"));
        assert!(!is_unnamed("entity"));
        // Not a profession, so a name tag's work stands.
        assert!(!is_unnamed("Old Tom"));
    }

    #[test]
    fn every_profession_name_is_one_the_language_file_has() {
        // The list is transcribed from the game's `en_us.json`; this pins the
        // ones the fallback can actually produce so a typo cannot silently
        // stop a profession from being recognised.
        for name in ["Farmer", "Librarian", "Weaponsmith", "Nitwit", "Villager"] {
            assert!(PROFESSION_NAMES.contains(&name), "{name} is missing");
        }
        let unique: HashSet<&&str> = PROFESSION_NAMES.iter().collect();
        assert_eq!(unique.len(), PROFESSION_NAMES.len(), "the list repeats a name");
    }

    #[test]
    fn an_unnamed_villager_is_given_its_name() {
        let region = REGIONS[0];
        let want = name_for(region, 4242);
        // The realistic read, and the raw-key one.
        assert_eq!(decide(region, 4242, "Farmer"), Decision::Name);
        assert_eq!(decide(region, 4242, "entity.minecraft.villager.farmer"), Decision::Name);
        assert_eq!(decide(region, 4242, ""), Decision::Name);
        // ... and once named, the next tick writes nothing.
        assert_eq!(decide(region, 4242, &want), Decision::Leave);
    }

    #[test]
    fn a_players_own_name_is_never_touched() {
        let region = REGIONS[0];
        assert_eq!(decide(region, 4242, "Bartholomew"), Decision::Leave);
    }

    #[test]
    fn a_name_from_an_earlier_version_is_shortened() {
        let region = REGIONS[0];
        let seed = mix(4242);
        let family = pick(region.families, seed >> 11);
        let given = name_for(region, 4242);
        let legacy = format!("{given} {family}, the Farmer");
        // Older versions wrote "Given Family, the Trade": still ours, so it is
        // replaced with the one word rather than treated as a player's name.
        assert!(named_by_us(region, 4242, &legacy));
        assert_eq!(decide(region, 4242, &legacy), Decision::Name);
        // Even when the trade has since changed.
        let legacy = format!("{given} {family}, the Cleric");
        assert_eq!(decide(region, 4242, &legacy), Decision::Name);
    }

    #[test]
    fn a_name_is_not_rewritten_because_the_villager_moved() {
        // A name never changes once it is given. The old native build kept a
        // session map of the names it had written, which let it rename a
        // villager that crossed into another region; that map is gone, and with
        // it the ability to tell our own name from a player's after a move. The
        // name is left alone, which is both the stated design rule and the only
        // answer that cannot overwrite a name tag.
        let from = REGIONS[0];
        let to = REGIONS[1];
        let old = name_for(from, 4242);
        let new = name_for(to, 4242);
        assert_ne!(old, new, "the two regions must disagree, or this proves nothing");
        assert_eq!(decide(to, 4242, &old), Decision::Leave);
        assert_eq!(decide(from, 4242, &old), Decision::Leave);
    }

    #[test]
    fn a_biome_picks_its_region_and_an_unknown_one_falls_back() {
        assert_eq!(region_of("minecraft:plains").map(|r| r.id), Some("lowlands"));
        assert_eq!(region_of("minecraft:desert").map(|r| r.id), Some("dunes"));
        assert_eq!(region_of("minecraft:nowhere"), None);
    }
}

// The Emberdeep: the burning underworld - nether, crimson woods, basalt. Ash folk names.
use crate::villager_names::Region;

pub const REGION: Region = Region {
    id: "emberdeep",
    biomes: &[
        "minecraft:nether_wastes",
        "minecraft:crimson_forest",
        "minecraft:warped_forest",
        "minecraft:soul_sand_valley",
        "minecraft:basalt_deltas",
    ],
    common: &[
        "Cindral", "Slag", "Pyrel", "Calx", "Nyle", "Ember", "Ferrite", "Obsid",
        "Char", "Kindle", "Cinder", "Flint", "Scoria", "Tuff", "Obscur", "Ash",
        "Soot", "Coale", "Pyre", "Clinker", "Slake", "Brim", "Fettle", "Grist",
        "Lye", "Sinter", "Marl", "Dross",
    ],
    uncommon: &[
        "Brand", "Kiln", "Cullet", "Fumar", "Fyrel", "Mica", "Pyrra", "Scori",
        "Spar", "Tephra", "Pum", "Opal",
    ],
    rare: &[
        "Cinderborn", "Emberwrath", "Slagcrown", "Fyrelord", "Charwalker", "Ashmourn",
    ],
    families: &[
        "Ashfall", "Cinderhold", "Slagmoor", "Obsidian", "Basalt", "Kilnmoor",
        "Emberwick", "Scoriacrest", "Charhollow", "Vitreous", "Molten", "Flintvale",
        "Pyrewood", "Ferrum", "Tundish", "Ashvale",
    ],
};

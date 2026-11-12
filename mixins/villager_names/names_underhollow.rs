// The Underhollow: the caves under the world - mushroom, dripstone and deep dark. Fungal folk names, grown rather than born.
use crate::villager_names::Region;

pub const REGION: Region = Region {
    id: "underhollow",
    biomes: &[
        "minecraft:mushroom_fields", "minecraft:dripstone_caves", "minecraft:lush_caves",
        "minecraft:deep_dark",
    ],
    common: &[
        "Bracken", "Bulb", "Caplin", "Coral", "Duskcap", "Dusken", "Frond", "Fuzz", "Gills",
        "Hollow", "Lichen", "Loam", "Morel", "Moss", "Mould", "Mycel", "Nettle", "Peat",
        "Pith", "Quill", "Rind", "Sap", "Sedge", "Spor", "Sprout", "Tallow", "Tangle", "Thuff",
    ],
    uncommon: &[
        "Bolete", "Chantrelle", "Duskveil", "Fleckcap", "Gillcap", "Glowcap", "Mycelid",
        "Puffball", "Spore", "Stemcap", "Thrum", "Truffle",
    ],
    rare: &[
        "Capwise", "Gildcap", "Mycelia", "Sporrum", "Thuffel", "Umbra",
    ],
    families: &[
        "Brackenhollow", "Caplinmoor", "Duskcapp", "Gillsmoor", "Loamwood", "Mycellark",
        "Nettlecote", "Pithgate", "Quillshade", "Sedgewick", "Sporewood", "Sporhallow",
        "Stemcote", "Tallowholt", "Thuffholt", "Trufflewell",
    ],
};

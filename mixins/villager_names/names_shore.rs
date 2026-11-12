// The Shore: the cold coasts - ocean, river and shingle. Celtic names, from the Atlantic.
use crate::villager_names::Region;

pub const REGION: Region = Region {
    id: "shore",
    biomes: &[
        "minecraft:ocean",
        "minecraft:deep_ocean",
        "minecraft:warm_ocean",
        "minecraft:deep_lukewarm_ocean",
        "minecraft:lukewarm_ocean",
        "minecraft:cold_ocean",
        "minecraft:deep_cold_ocean",
        "minecraft:frozen_ocean",
        "minecraft:deep_frozen_ocean",
        "minecraft:river",
        "minecraft:frozen_river",
        "minecraft:beach",
        "minecraft:snowy_beach",
        "minecraft:stony_shore",
    ],
    common: &[
        "Aife", "Aoife", "Bran", "Branwen", "Caradoc", "Corran", "Cuan", "Dwynwen", "Eamon",
        "Efa", "Eira", "Fionn", "Gareth", "Gwyn", "Isolde", "Keeva", "Lir", "Mabon", "Nessa",
        "Nia", "Rhiannon", "Rhys", "Rua", "Sile", "Sionn", "Taliesin", "Una", "Wynn",
    ],
    uncommon: &[
        "Aneurin", "Bedwyr", "Carys", "Dilys", "Elowen", "Fflur", "Gwilym", "Hesper", "Idris",
        "Kian", "Mair", "Nev",
    ],
    rare: &[
        "Aonb", "Boann", "Danu", "Lugh", "Morrig", "Nuada",
    ],
    families: &[
        "Bosunrest", "Brackenlow", "Cobblecay", "Dunmorrow", "Fernquay", "Gullstone",
        "Highstrand", "Kelpsea", "Longstrand", "Lowtide", "Mermaidreach", "Ravenshore",
        "Saltcote", "Saltmarsh", "Tidewater", "Windlass",
    ],
};

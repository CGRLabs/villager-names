// The Void: the far end of the world - end islands, highlands, barrens. Thin, quiet names.
use crate::villager_names::Region;

pub const REGION: Region = Region {
    id: "void",
    biomes: &[
        "minecraft:the_end",
        "minecraft:end_highlands",
        "minecraft:end_midlands",
        "minecraft:end_barrens",
        "minecraft:small_end_islands",
        "minecraft:the_void",
    ],
    common: &[
        "Vail", "Nhem", "Sere", "Umbr", "Ossa", "Elian", "Vesper", "Ilex",
        "Orrin", "Tace", "Nul", "Vael", "Thes", "Ysm", "Cael", "Onne",
        "Nym", "Vyr", "Tyll", "Ovel", "Umber", "Sylm", "Nill", "Vane",
        "Vire", "Nesh", "Ovan", "Serin",
    ],
    uncommon: &[
        "Nheme", "Sereth", "Umbril", "Osseth", "Elivar", "Ilexir", "Orrith",
        "Tacel", "Nulith", "Vaelen", "Thesen", "Ysmir",
    ],
    rare: &[
        "Voidtouched", "Endwrack", "Nullsong", "Emptysong", "Farsight", "Lastlight",
    ],
    families: &[
        "Voidmere", "Endrest", "Umbralis", "Osset", "Vaelmoor", "Nulplain", "Thesmere",
        "Ossuary", "Vesperholt", "Caelgrove", "Ilexmoor", "Ysmere", "Onnefield",
        "Orrinmoor", "Tacebarrow", "Vailwater",
    ],
};

// The Lowlands: the temperate overworld — plains, meadows and the deciduous
// woods. English names, from the old country: sober, plain, farm and wood.

use crate::villager_names::Region;

pub const REGION: Region = Region {
    id: "lowlands",
    biomes: &[
        "minecraft:plains",
        "minecraft:sunflower_plains",
        "minecraft:meadow",
        "minecraft:forest",
        "minecraft:flower_forest",
        "minecraft:birch_forest",
        "minecraft:dark_forest",
        "minecraft:grove",
        "minecraft:old_growth_birch_forest",
        "minecraft:pale_garden",
    ],
    common: &[
        "Aldric", "Alwin", "Baldric", "Baldwin", "Beorn", "Colm", "Cuthbert", "Cuthwine",
        "Dunstan", "Eadgyth", "Eadric", "Edgith", "Godwin", "Gytha", "Hamo", "Hilda",
        "Leofric", "Maldwyn", "Ordric", "Osgyth", "Osric", "Oda", "Reynald", "Sewin",
        "Sigebert", "Thurstan", "Wulfgyth", "Wulfric",
    ],
    uncommon: &[
        "Aelfgifu", "Beornwulf", "Cuthberht", "Cynewulf", "Eadweard", "Freothryth", "Godgyth",
        "Hrothmund", "Osberht", "Seaxmund", "Wulfheard", "Wynflaed",
    ],
    rare: &[
        "Artgal", "Dunmail", "Ealdgyth", "Merewal", "Wendelin", "Aedwen",
    ],
    families: &[
        "Aldermere", "Ashdown", "Barleyleigh", "Birchwood", "Coldbrook", "Dunmere", "Ealdham",
        "Fernhollow", "Greenhollow", "Hartwood", "Kettleby", "Millbrook", "Oakenshaw",
        "Redfern", "Stonebridge", "Willowby",
    ],
};

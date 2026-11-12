// The Hills: the wind-beaten border country - moor, scree and pine. Scottish names: blunt, weathered, spare.
use crate::villager_names::Region;

pub const REGION: Region = Region {
    id: "hills",
    biomes: &[
        "minecraft:windswept_hills", "minecraft:windswept_gravelly_hills",
        "minecraft:windswept_forest",
    ],
    common: &[
        "Ailsa", "Blair", "Bran", "Cail", "Callum", "Dougal", "Eilidh", "Ewan", "Fergus",
        "Findlay", "Gair", "Gille", "Hamish", "Iain", "Johnnie", "Kenneth", "Kirsty",
        "Lachlan", "Mairi", "Murdoch", "Neil", "Ranald", "Rhona", "Sandy", "Seonaid", "Tam",
        "Torquil", "Wilkie",
    ],
    uncommon: &[
        "Alasdair", "Angus", "Brodie", "Erskine", "Gregor", "Innes", "Malcolm", "Moray",
        "Ninian", "Roderick", "Somerled", "Struan",
    ],
    rare: &[
        "Eachuisge", "Fachan", "Gillean", "Selkie", "Sith", "Tarranach",
    ],
    families: &[
        "Blackcleuch", "Braefoot", "Braeval", "Cairnhead", "Coldstruth", "Glenbeg", "Glenmara",
        "Greyfell", "Heatherden", "Langmoss", "Moorhaugh", "Rannoch", "Scaur", "Thirlmere",
        "Tollbar", "Windlaw",
    ],
};

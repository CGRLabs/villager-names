//! Every region's names, one table each.
//!
//! Carried over verbatim from the retired native build's `names_*.rs`, one
//! module per region, so a region's names stay together and easy to extend.
//! The rules that read these tables live in `rules.rs`.

use crate::rules::Region;

// The Lowlands: the temperate overworld — plains, meadows and the deciduous
// woods. English names, from the old country: sober, plain, farm and wood.


pub const LOWLANDS: Region = Region {
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

// The Northlands: the frozen north - snow, ice and black pine. Norse names, plain and hard.

pub const NORTHLANDS: Region = Region {
    id: "northlands",
    biomes: &[
        "minecraft:snowy_plains",
        "minecraft:snowy_slopes",
        "minecraft:frozen_peaks",
        "minecraft:jagged_peaks",
        "minecraft:ice_spikes",
        "minecraft:stony_peaks",
        "minecraft:taiga",
        "minecraft:snowy_taiga",
        "minecraft:old_growth_pine_taiga",
        "minecraft:old_growth_spruce_taiga",
    ],
    common: &[
        "Agnar", "Aslaug", "Bjorn", "Brynjolf", "Eirik", "Gudbrand", "Gudrun", "Gunnar",
        "Halfdan", "Helga", "Hildur", "Ingemar", "Ingvild", "Kjell", "Ragnhild", "Rolf",
        "Runa", "Sigrid", "Sigurd", "Solveig", "Svanhild", "Torben", "Torvald", "Ulf",
        "Ulrika", "Vebjorn", "Ylva", "Yrsa",
    ],
    uncommon: &[
        "Asbjorn", "Asta", "Eystein", "Geir", "Gudbjorn", "Gudrid", "Hrafn", "Hrolf", "Ketil",
        "Sigvaldi", "Thorgunn", "Thorsten",
    ],
    rare: &[
        "Balder", "Gridr", "Nibel", "Sif", "Vanheim", "Volsun",
    ],
    families: &[
        "Bjornvik", "Brekka", "Eldhval", "Frostskar", "Hagalund", "Hvitberg", "Isdal",
        "Jarnfjell", "Nattfjell", "Ravnholt", "Skjold", "Stormskog", "Ulvstad", "Vargholm",
        "Vindstille", "Vinterfjell",
    ],
};

// The Hills: the wind-beaten border country - moor, scree and pine. Scottish names: blunt, weathered, spare.

pub const HILLS: Region = Region {
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

// The Dunes: the hot dry south - desert, badlands and red earth, and the long roads between.

pub const DUNES: Region = Region {
    id: "dunes",
    biomes: &[
        "minecraft:desert",
        "minecraft:badlands",
        "minecraft:eroded_badlands",
        "minecraft:wooded_badlands",
    ],
    common: &[
        "Amina", "Aicha", "Amira", "Amal", "Bilal", "Dalia", "Firas", "Ghada",
        "Hakim", "Halim", "Idris", "Jamal", "Karim", "Layla", "Lubna", "Mounir",
        "Nadira", "Nasir", "Nabil", "Qasim", "Rania", "Rashid", "Safiya", "Salma",
        "Soraya", "Tarik", "Yusuf", "Zayd",
    ],
    uncommon: &[
        "Farid", "Zahra", "Yasin", "Wajid", "Sabra", "Malika", "Kenza", "Meriem",
        "Souad", "Adil", "Rachida", "Brahim",
    ],
    rare: &[
        "Almas", "Marid", "Ghul", "Saif", "Maruf", "Nasr",
    ],
    families: &[
        "Al-Sahra", "Benhadda", "El Mansur", "Qadim", "Ras El Ain", "Sidi",
        "Taziri", "Ouarzazate", "Reggada", "Ghat", "Ouled", "Tighremt",
        "Djerba", "Mhamid", "Sbait", "Taghazout",
    ],
};

// The Savanna: the warm grasslands of the south - gold grass, flat acacia, long dust roads. West African names, from the old country: warm, many-voiced, and hard to wear out.

pub const SAVANNA: Region = Region {
    id: "savanna",
    biomes: &[
        "minecraft:savanna",
        "minecraft:savanna_plateau",
        "minecraft:windswept_savanna",
    ],
    common: &[
        "Abeni", "Abubakar", "Adama", "Akua", "Amara", "Chiamaka", "Chike", "Diarra", "Emeka",
        "Fatu", "Gambo", "Halima", "Ibrahim", "Ife", "Jibril", "Kadija", "Kofi", "Kwame",
        "Mamadou", "Ndidi", "Nia", "Oluwaseun", "Seyni", "Thandiwe", "Umaru", "Yewande",
        "Zainab", "Zuri",
    ],
    uncommon: &[
        "Adwoa", "Amadou", "Anu", "Assane", "Binta", "Demba", "Fanta", "Folasade", "Jatau",
        "Naima", "Olumide", "Yaa",
    ],
    rare: &[
        "Adanlo", "Kesi", "Ogun", "Obatala", "Sundiata", "Yemoja",
    ],
    families: &[
        "Adinkra", "Bambara", "Dagbon", "Ewe", "Fante", "Hausa", "Igbo", "Kanem", "Mandinka",
        "Nupe", "Oyo", "Sene", "Tuareg", "Wolof", "Yoruba", "Zarma",
    ],
};

// The Green Deep: the hot, wet overworld of jungle, swamp and mangrove. Indian names, plain.


pub const GREENDEEP: Region = Region {
    id: "greendeep",
    biomes: &[
        "minecraft:jungle",
        "minecraft:sparse_jungle",
        "minecraft:bamboo_jungle",
        "minecraft:swamp",
        "minecraft:mangrove_swamp",
    ],
    common: &[
        "Aditi", "Anjali", "Arjun", "Bharat", "Deepak", "Isha", "Kavya",
        "Kiran", "Lakshmi", "Manoj", "Meera", "Naveen", "Nisha", "Pankaj",
        "Priya", "Radha", "Rani", "Rohan", "Sanjay", "Sarita", "Sunita",
        "Thiru", "Trisha", "Uday", "Vidya", "Vikram", "Yash", "Zoya",
    ],
    uncommon: &[
        "Chandra", "Chitra", "Devika", "Divya", "Jayashree", "Keshav", "Meenakshi",
        "Neelam", "Parvati", "Rajendra", "Sharada", "Sundari",
    ],
    rare: &[
        "Bheema", "Garuda", "Hanuman", "Indra", "Krishna", "Saraswati",
    ],
    families: &[
        "Coromandel", "Doab", "Godavari", "Kaveri", "Koneru", "Konkan", "Malabar",
        "Narmada", "Nilgiri", "Sewalika", "Tamiraparani", "Trichinopoly",
        "Tungabhadra", "Vanapalli", "Vindhya", "Yamuna",
    ],
};

// The Underhollow: the caves under the world - mushroom, dripstone and deep dark. Fungal folk names, grown rather than born.

pub const UNDERHOLLOW: Region = Region {
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

// The Emberdeep: the burning underworld - nether, crimson woods, basalt. Ash folk names.

pub const EMBERDEEP: Region = Region {
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

// The Shore: the cold coasts - ocean, river and shingle. Celtic names, from the Atlantic.

pub const SHORE: Region = Region {
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

// The Void: the far end of the world - end islands, highlands, barrens. Thin, quiet names.

pub const VOID: Region = Region {
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

/// Every region, in the order they are listed.
pub const REGIONS: &[&Region] = &[
    &LOWLANDS,
    &NORTHLANDS,
    &HILLS,
    &DUNES,
    &SAVANNA,
    &GREENDEEP,
    &UNDERHOLLOW,
    &EMBERDEEP,
    &SHORE,
    &VOID,
];


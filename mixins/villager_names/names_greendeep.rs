// The Green Deep: the hot, wet overworld of jungle, swamp and mangrove. Indian names, plain.

use crate::villager_names::Region;

pub const REGION: Region = Region {
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

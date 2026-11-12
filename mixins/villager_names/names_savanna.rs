// The Savanna: the warm grasslands of the south - gold grass, flat acacia, long dust roads. West African names, from the old country: warm, many-voiced, and hard to wear out.
use crate::villager_names::Region;

pub const REGION: Region = Region {
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

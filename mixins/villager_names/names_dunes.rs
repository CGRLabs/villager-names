// The Dunes: the hot dry south - desert, badlands and red earth, and the long roads between.
use crate::villager_names::Region;

pub const REGION: Region = Region {
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

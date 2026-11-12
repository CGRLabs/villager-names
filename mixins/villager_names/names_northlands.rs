// The Northlands: the frozen north - snow, ice and black pine. Norse names, plain and hard.
use crate::villager_names::Region;

pub const REGION: Region = Region {
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

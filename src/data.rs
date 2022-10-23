#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Damage types

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
pub enum DT {
    Melee(MeleeDamageType),
    Ranged,
    Magic,
    Typeless,
}

#[derive(Debug, Default, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MeleeDamageType {
    Stab,
    Slash,
    Crush,
    #[default]
    Default,
}

/// Slot

/// Style

/// Skill

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Skill {
    Attack,
    Strength,
    Defence,
    Ranged,
    Prayer,
    Magic,
    Runecraft,
    Construction,
    Hitpoints,
    Agility,
    Herblore,
    Thieving,
    Crafting,
    Fletching,
    Slayer,
    Hunter,
    Mining,
    Smithing,
    Fishing,
    Cooking,
    Firemaking,
    Woodcutting,
    Farming,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MonsterType {
    Demon,
    Draconic,
    Fiery,
    Golem,
    Kalphite,
    Leafy,
    Penance,
    Shade,
    Spectral,
    Undead,
    Vampyre(VampyreTier),
    Xerician,
    Wilderness,
    Toa,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum VampyreTier {
    One,
    Two,
    Three,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MonsterLocation {
    Wilderness,
    Cox,
    Tob,
    Toa,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Slayer {
    AberrantSpectres,
    AbyssalDemons,
    AdamantDragons,
    Ankous,
    Aviansie,
    Bandits,
    Banshees,
    Basilisks,
    Bats,
    Bears,
    Birds,
    BlackDemons,
    BlackDragons,
    BlackKnights,
    Bloodvelds,
    BlueDragons,
    BrineRats,
    BronzeDragons,
    Catablepon,
    CaveBugs,
    CaveCrawlers,
    CaveHorrors,
    CaveSlimes,
    CaveKraken,
    ChaosDruids,
    Cockatrices,
    Cows,
    Dagannoths,
    DustDevils,
    FossilIslandWyverns,
    Goblins,
    GreaterDemons,
    GreenDragons,
    Hellhounds,
    Hydras,
    Jellies,
    Kalphites,
    Kurasks,
    LavaDragons,
    Lizardmen,
    MithrilDragons,
    Nechryael,
    RedDragons,
    Revenants,
    RuneDragons,
    Scorpions,
    Shades,
    SkeletalWyverns,
    Skeletons,
    SmokeDevils,
    Suqahs,
    Trolls,
    Vampyres,
    Wyrms,
}

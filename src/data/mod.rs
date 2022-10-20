pub mod data {

    use strum_macros::EnumIter;

    /// Damage types

    #[derive(Debug)]
    pub enum DT {
        Melee(MeleeDamageType),
        Ranged,
        Magic,
        Typeless,
    }

    #[derive(Debug, EnumIter)]
    pub enum MeleeDamageType {
        Stab,
        Slash,
        Crush,
    }

    /// Slot

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Slot {
        Head,
        Cape,
        Neck,
        Ammunition,
        Weapon,
        Body,
        Shield,
        Legs,
        Hands,
        Feet,
        Ring,
    }

    /// Style

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum StyleName {
        Slash,
        Stab,
        Accurate,
        Rapid,
        Longrange,
        Chop,
        Smash,
        Block,
        Hack,
        Lunge,
        Swipe,
        Pound,
        Pummel,
        Spike,
        Impale,
        Jab,
        Fend,
        Bash,
        Reap,
        Punch,
        Kick,
        Flick,
        Lash,
        Deflect,
        ShortFuse,
        MediumFuse,
        LongFuse,
        Spell,
        Focus,
        StandardSpell,
        DefensiveSpell,
        NpcMelee,
        NpcRanged,
        NpcMagic,
    }

    /// Stances

    #[derive(Debug)]
    pub enum Stance {
        Melee(MeleeStance),
        Ranged(RangedStance),
        Magic(MagicStance),
    }

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
    pub enum MeleeStance {
        Accurate,
        Aggressive,
        Controlled,
        Defensive,
    }

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
    pub enum RangedStance {
        Accurate,
        Rapid,
        Longrange,
    }

    #[derive(Debug)]
    pub enum MagicStance {
        Accurate,
        Longrange,
        NoStyle(SpellStyle),
    }

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
    pub enum SpellStyle {
        StandardSpell,
        DefensiveSpell,
    }

    /// Skill

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
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

    #[derive(Debug)]
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

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
    pub enum VampyreTier {
        One,
        Two,
        Three,
    }

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
    pub enum MonsterLocation {
        Wilderness,
        Cox,
        Tob,
        Toa,
    }

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
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

    #[derive(Debug)]
    pub enum Effect {
        StaffOfTheDead,
        StaminaPotion,
        DivinePotion,
        Overload,
        RegenerateSpecialEnergy,
        UpdateStats,
        Olm(OlmEffect),
        PrayerDrain,
        Frozen,
    }

    #[derive(Debug, EnumIter, PartialEq, Eq, Hash)]
    pub enum OlmEffect {
        Burn,
        Acid,
        FallingCrystal,
    }
}

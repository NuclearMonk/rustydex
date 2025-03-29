use std::str::FromStr;

use ratatui::style::Color;

pub type PokemonName= String;

pub enum MonStat {
    HP,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}

impl FromStr for MonStat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hp" => Ok(MonStat::HP),
            "attack" => Ok(MonStat::Attack),
            "defense" => Ok(MonStat::Defense),
            "special-attack" => Ok(MonStat::SpecialAttack),
            "special-defense" => Ok(MonStat::SpecialDefense),
            "speed" => Ok(MonStat::Speed),
            _ => Err(()),
        }
    }
}

impl ToString for MonStat {
    fn to_string(&self) -> String {
        match self {
            Self::HP => "hp".to_owned(),
            Self::Attack => "attack".to_owned(),
            Self::Defense => "defense".to_owned(),
            Self::SpecialAttack => "special attack".to_owned(),
            Self::SpecialDefense => "special defense".to_owned(),
            Self::Speed => "speed".to_owned(),
        }
    }
}

impl MonStat {
    pub fn to_short_string(&self) -> String {
        match self {
            Self::HP => "HP".to_owned(),
            Self::Attack => "ATK".to_owned(),
            Self::Defense => "DEF".to_owned(),
            Self::SpecialAttack => "S.ATK".to_owned(),
            Self::SpecialDefense => "S.DEF".to_owned(),
            Self::Speed => "SPD".to_owned(),
        }
    }

    pub fn bg(&self) -> Color {
        match self {
            Self::HP => Color::from_u32(0x009EE865),
            Self::Attack => Color::from_u32(0x00F5DE69),
            Self::Defense => Color::from_u32(0x00F09A65),
            Self::SpecialAttack => Color::from_u32(0x0066D8F6),
            Self::SpecialDefense => Color::from_u32(0x00899EEA),
            Self::Speed => Color::from_u32(0x00E46CCA),
        }
    }
}

pub enum MonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl FromStr for MonType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(MonType::Normal),
            "fire" => Ok(MonType::Fire),
            "water" => Ok(MonType::Water),
            "electric" => Ok(MonType::Electric),
            "grass" => Ok(MonType::Grass),
            "ice" => Ok(MonType::Ice),
            "fighting" => Ok(MonType::Fighting),
            "poison" => Ok(MonType::Poison),
            "ground" => Ok(MonType::Ground),
            "flying" => Ok(MonType::Flying),
            "psychic" => Ok(MonType::Psychic),
            "bug" => Ok(MonType::Bug),
            "rock" => Ok(MonType::Rock),
            "ghost" => Ok(MonType::Ghost),
            "dragon" => Ok(MonType::Dragon),
            "dark" => Ok(MonType::Dark),
            "steel" => Ok(MonType::Steel),
            "fairy" => Ok(MonType::Fairy),
            _ => Err(()),
        }
    }
}

impl ToString for MonType {
    fn to_string(&self) -> String {
        match self {
            MonType::Normal => "normal".to_owned(),
            MonType::Fire => "fire".to_owned(),
            MonType::Water => "water".to_owned(),
            MonType::Electric => "electric".to_owned(),
            MonType::Grass => "grass".to_owned(),
            MonType::Ice => "ice".to_owned(),
            MonType::Fighting => "fighting".to_owned(),
            MonType::Poison => "poison".to_owned(),
            MonType::Ground => "ground".to_owned(),
            MonType::Flying => "flying".to_owned(),
            MonType::Psychic => "psychic".to_owned(),
            MonType::Bug => "bug".to_owned(),
            MonType::Rock => "rock".to_owned(),
            MonType::Ghost => "ghost".to_owned(),
            MonType::Dragon => "dragon".to_owned(),
            MonType::Dark => "dark".to_owned(),
            MonType::Steel => "steel".to_owned(),
            MonType::Fairy => "fairy".to_owned(),
        }
    }
}


impl MonType
{
    pub fn bg(&self)-> Color
    {
        match self
        {
            MonType::Normal=> Color::from_u32(0x00A8A77A),
            MonType::Fire=> Color::from_u32(0x00EE8130),
            MonType::Water=> Color::from_u32(0x006390F0),
            MonType::Electric=> Color::from_u32(0x00F7D02C),
            MonType::Grass=> Color::from_u32(0x007AC74C),
            MonType::Ice=> Color::from_u32(0x0096D9D6),
            MonType::Fighting=> Color::from_u32(0x00C22E28),
            MonType::Poison=> Color::from_u32(0x00A33EA1),
            MonType::Ground=> Color::from_u32(0x00E2BF65),
            MonType::Flying=> Color::from_u32(0x00A98FF3),
            MonType::Psychic=> Color::from_u32(0x00F95587),
            MonType::Bug=> Color::from_u32(0x00A6B91A),
            MonType::Rock=> Color::from_u32(0x00B6A136),
            MonType::Ghost=> Color::from_u32(0x00735797),
            MonType::Dragon=> Color::from_u32(0x006F35FC),
            MonType::Dark=> Color::from_u32(0x00705746),
            MonType::Steel=> Color::from_u32(0x00B7B7CE),
            MonType::Fairy=> Color::from_u32(0x00D685AD),
        }
    }

    pub fn fg(&self)-> Color
    {
        match self
        {

            MonType::Fighting=> Color::White,
            MonType::Poison=> Color::White,
            MonType::Ghost=> Color::White,
            MonType::Dragon=> Color::White,
            MonType::Dark=> Color::White,
            _ => Color::Black
        }
    }
}
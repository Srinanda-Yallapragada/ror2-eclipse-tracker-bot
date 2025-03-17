use core::fmt;

pub const NUM_SURVIVORS: usize = 16;
pub enum Survivors {
    Acrid = 0,
    Artificer = 1,
    Bandit = 2,
    Captain = 3,
    Commando = 4,
    Engineer = 5,
    Huntress = 6,
    Loader = 7,
    MulT = 8,
    Mercenary = 9,
    Rex = 10,
    Railgunner = 11,
    VoidFiend = 12,
    Seeker = 13,
    FalseSon = 14,
    Chef = 15,
}

impl Survivors {
    pub fn index_to_survivor(index: usize) -> Self {
        match index {
            0 => Survivors::Acrid,
            1 => Survivors::Artificer,
            2 => Survivors::Bandit,
            3 => Survivors::Captain,
            4 => Survivors::Commando,
            5 => Survivors::Engineer,
            6 => Survivors::Huntress,
            7 => Survivors::Loader,
            8 => Survivors::MulT,
            9 => Survivors::Mercenary,
            10 => Survivors::Rex,
            11 => Survivors::Railgunner,
            12 => Survivors::VoidFiend,
            13 => Survivors::Seeker,
            14 => Survivors::FalseSon,
            15 => Survivors::Chef,
            _ => panic!("Invalid survivor index: {}", index),
        }
    }
    pub fn survivor_to_index(&self) -> usize {
        match self {
            Survivors::Acrid => 0,
            Survivors::Artificer => 1,
            Survivors::Bandit => 2,
            Survivors::Captain => 3,
            Survivors::Commando => 4,
            Survivors::Engineer => 5,
            Survivors::Huntress => 6,
            Survivors::Loader => 7,
            Survivors::MulT => 8,
            Survivors::Mercenary => 9,
            Survivors::Rex => 10,
            Survivors::Railgunner => 11,
            Survivors::VoidFiend => 12,
            Survivors::Seeker => 13,
            Survivors::FalseSon => 14,
            Survivors::Chef => 15,
        }
    }
    pub fn survivor_to_name(&self) -> &'static str {
        match self {
            Survivors::Acrid => "Acrid",
            Survivors::Artificer => "Artificer",
            Survivors::Bandit => "Bandit",
            Survivors::Captain => "Captain",
            Survivors::Commando => "Commando",
            Survivors::Engineer => "Engineer",
            Survivors::Huntress => "Huntress",
            Survivors::Loader => "Loader",
            Survivors::MulT => "MulT",
            Survivors::Mercenary => "Mercenary",
            Survivors::Rex => "Rex",
            Survivors::Railgunner => "Railgunner",
            Survivors::VoidFiend => "VoidFiend",
            Survivors::Seeker => "Seeker",
            Survivors::FalseSon => "FalseSon",
            Survivors::Chef => "Chef",
        }
    }

    pub fn name_to_survivor(input: &str) -> Option<Survivors> {
        match input {
            "Acrid" => Some(Survivors::Acrid),
            "Artificer" => Some(Survivors::Artificer),
            "Bandit" => Some(Survivors::Bandit),
            "Captain" => Some(Survivors::Captain),
            "Commando" => Some(Survivors::Commando),
            "Engineer" => Some(Survivors::Engineer),
            "Huntress" => Some(Survivors::Huntress),
            "Loader" => Some(Survivors::Loader),
            "MulT" => Some(Survivors::MulT),
            "Mercenary" => Some(Survivors::Mercenary),
            "Rex" => Some(Survivors::Rex),
            "Railgunner" => Some(Survivors::Railgunner),
            "VoidFiend" => Some(Survivors::VoidFiend),
            "Seeker" => Some(Survivors::Seeker),
            "FalseSon" => Some(Survivors::FalseSon),
            "Chef" => Some(Survivors::Chef),
            _ => None,
        }
    }
}

impl fmt::Display for Survivors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.survivor_to_name())
    }
}

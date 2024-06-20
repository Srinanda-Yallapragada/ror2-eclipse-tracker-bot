use core::fmt;

pub const NUM_SURVIVORS: usize = 13;
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
}

impl Survivors {
    pub fn index_to_survivor(index: usize) -> Option<Self> {
        match index {
            0 => Some(Survivors::Acrid),
            1 => Some(Survivors::Artificer),
            2 => Some(Survivors::Bandit),
            3 => Some(Survivors::Captain),
            4 => Some(Survivors::Commando),
            5 => Some(Survivors::Engineer),
            6 => Some(Survivors::Huntress),
            7 => Some(Survivors::Loader),
            8 => Some(Survivors::MulT),
            9 => Some(Survivors::Mercenary),
            10 => Some(Survivors::Rex),
            11 => Some(Survivors::Railgunner),
            12 => Some(Survivors::VoidFiend),
            _ => None,
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
            _ => None,
        }
    }
}

impl fmt::Display for Survivors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.survivor_to_name())
    }
}

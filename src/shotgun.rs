//Shotgun
pub struct Shotgun {
    shells: Vec<Shell>,
    state: ShotgunState,
}

pub enum ShotgunState {
    Default,
    SawedOff,
    HotPotato,
}

pub enum Shell {
    Live,
    Blank,
    Incendiary,
    BeanBag,
    Electric,
}

use ShotgunState::*;
impl Shotgun {

    pub fn new() -> Shotgun { 
        Shotgun { 
            shells: Vec::new(),
            state: Default,
        }
    }

    pub fn reload(number_shells: u8, wild_card: bool) -> Shotgun {

        Shotgun {
            shells: new_shells,
            state: Default,
        }
    }
}

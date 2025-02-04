use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::audio::Sound;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Ammunition {
    x762x54R,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Magazine {
    MosinNagant(usize),
}

impl Magazine {
    pub fn name(&self) -> &str {
        match self {
            Magazine::MosinNagant(_) => "Mosin Nagant",
        }
    }

    pub fn full(magazine: Self) -> Self {
        match magazine {
            Magazine::MosinNagant(_) => Magazine::MosinNagant(5),
        }
    }

    pub fn ammunition(&self) -> Ammunition {
        match self {
            Magazine::MosinNagant(_) => Ammunition::x762x54R,
        }
    }

    pub fn filled(&self) -> bool {
        match self {
            Magazine::MosinNagant(fill) => *fill > 0,
        }
    }

    fn remove_one(&mut self) {
        match self {
            Magazine::MosinNagant(fill) => {
                if *fill > 0 {
                    *fill -= 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GunFireSoundType {
    MosinNagant,
}

impl GunFireSoundType {
    pub fn fire_sounds(&self) -> Vec<Sound> {
        let pick_from = match self {
            GunFireSoundType::MosinNagant => vec![
                Sound::MosinNagantFire1,
                Sound::MosinNagantFire2,
                Sound::MosinNagantFire3,
                Sound::MosinNagantFire4,
                Sound::MosinNagantFire5,
            ],
        };
        let sound = *pick_from
            .choose(&mut rand::thread_rng())
            .expect("Must one be chosen");

        vec![sound]
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Weapon {
    // ready bullet, filled magazine
    MosinNagantM1924(bool, Option<Magazine>),
}

impl Weapon {
    pub fn name(&self) -> &str {
        match self {
            Weapon::MosinNagantM1924(_, _) => "Mosin Nagant M1924",
        }
    }

    pub fn gun_fire_sound_type(&self) -> GunFireSoundType {
        match self {
            Weapon::MosinNagantM1924(_, _) => GunFireSoundType::MosinNagant,
        }
    }

    pub fn reload_sounds(&self) -> Vec<Sound> {
        let pick_from = match self {
            Weapon::MosinNagantM1924(_, _) => vec![
                Sound::MosinNagantReload1,
                Sound::MosinNagantReload2,
                Sound::MosinNagantReload3,
                Sound::MosinNagantReload4,
            ],
        };
        let sound = *pick_from
            .choose(&mut rand::thread_rng())
            .expect("Must one be chosen");

        vec![sound]
    }

    pub fn magazine(&self) -> &Option<Magazine> {
        match self {
            Weapon::MosinNagantM1924(_, magazine) => magazine,
        }
    }

    pub fn accepted_magazine(&self, magazine: &Magazine) -> bool {
        match magazine {
            Magazine::MosinNagant(_) => true,
        }
    }

    pub fn ammunition(&self) -> Ammunition {
        if let Some(magazine) = self.magazine() {
            return magazine.ammunition();
        }

        // Default value
        match self {
            Weapon::MosinNagantM1924(_, _) => Ammunition::x762x54R,
        }
    }

    pub fn can_fire(&self) -> bool {
        match self {
            Weapon::MosinNagantM1924(ammunition, _) => *ammunition,
        }
    }

    pub fn can_reload(&self) -> bool {
        match self {
            Weapon::MosinNagantM1924(_, magazine) => {
                if let Some(magazine) = magazine {
                    return magazine.filled();
                }
            }
        }

        false
    }

    pub fn reload(&mut self) {
        match self {
            Weapon::MosinNagantM1924(ready_bullet, magazine) => {
                if !*ready_bullet {
                    if let Some(magazine_) = magazine {
                        if magazine_.filled() {
                            magazine_.remove_one();
                            *ready_bullet = true;
                        }

                        if !magazine_.filled() {
                            *magazine = None;
                        }
                    }
                }
            }
        }
    }

    pub fn shot(&mut self) {
        match self {
            Weapon::MosinNagantM1924(ready_bullet, _) => *ready_bullet = false,
        }
    }

    pub fn set_magazine(&mut self, new_magazine: Magazine) {
        match self {
            Weapon::MosinNagantM1924(_, magazine) => *magazine = Some(new_magazine),
        }
    }

    pub fn ok_count_magazines(&self) -> usize {
        match self {
            Weapon::MosinNagantM1924(_, _) => 4,
        }
    }
}

use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};
use crate::player::{Class, Gender, Race};

pub trait CharacterCreateToolkit {
    fn generate_random_string(capitalize: bool) -> String {
        let mut rng = thread_rng();
        let random_length = rng.gen_range(9..=11);

        let string: String = rng
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_alphabetic())
            .take(random_length)
            .map(|c| c as char)
            .collect();

        if capitalize {
            let first_letter = string.chars().next().unwrap();

            format!("{}{}", first_letter.to_uppercase(), string.to_lowercase())
        } else {
            format!("{}", string.to_lowercase())
        }
    }

    fn get_random_race() -> u8 {
        let races = vec![
            Race::HUMAN,
            Race::ORC,
            Race::DWARF,
            Race::NIGHTELF,
            Race::UNDEAD,
            Race::TROLL,
        ];

        let mut rng = thread_rng();
        *races.choose(&mut rng).unwrap()
    }

    fn get_random_class() -> u8 {
        let races = vec![
            Class::WARRIOR,
            Class::ROGUE,
        ];

        let mut rng = thread_rng();
        *races.choose(&mut rng).unwrap()
    }

    fn get_random_gender() -> u8 {
        let races = vec![
            Gender::GENDER_MALE,
            Gender::GENDER_FEMALE,
        ];

        let mut rng = thread_rng();
        *races.choose(&mut rng).unwrap()
    }
}
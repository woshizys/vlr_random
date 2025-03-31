use crate::config::{NAMES_CN, NUMBER_OF_HERO, OCCUPATION_EACH_HERO};
use rand::Rng;

pub fn generate() {
    let mut heros = NAMES_CN.clone().to_vec();
    let mut team = vec![];
    let mut rng = rand::rng();
    for i in 0..5 {
        let number: u32 = rng.random_range(..NUMBER_OF_HERO - i);
        team.push(heros[number as usize]);
        heros.remove(number as usize);
    }
    println!("{:?}", team);
}

pub fn generate_from_occupation(duelist: u32, sentinel: u32, controller: u32, initiator: u32) {
    let mut duelist_hero = vec![];
    let mut sentinel_team = vec![];
    let mut controller_hero = vec![];
    let mut initiator_hero = vec![];
    for (i, occupation) in OCCUPATION_EACH_HERO.enumerate() {
        if occupation == 0 {
            duelist_hero.push(String::from(NAMES_CN[i]));
        } else if occupation == 1 {
            sentinel_team.push(String::from(NAMES_CN[i]));
        } else if occupation == 2 {
            controller_hero.push(String::from(NAMES_CN[i]));
        } else {
            initiator_hero.push(String::from(NAMES_CN[i]));
        }
    }
}
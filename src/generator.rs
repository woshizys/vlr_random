use crate::config::{NAMES_CN, OCCUPATION_EACH_HERO};
use rand::Rng;

pub fn generate(
    players: u32,
    duelist_number: u32,
    sentinel_number: u32,
    controller_number: u32,
    initiator_number: u32,
) -> Vec<String> {
    let mut team = generate_from_occupation(duelist_number, sentinel_number, controller_number, initiator_number);
    let mut hero_names = NAMES_CN.clone().to_vec();
    hero_names.retain(|name| !team.contains(&name.to_string()));
    team.extend(basic_generate(hero_names, players - duelist_number - sentinel_number - controller_number - initiator_number));
    team
}

pub fn generate_from_occupation(
    duelist_number: u32,
    sentinel_number: u32,
    controller_number: u32,
    initiator_number: u32,
) -> Vec<String> {
    let mut duelist_hero = vec![];
    let mut sentinel_team = vec![];
    let mut controller_hero = vec![];
    let mut initiator_hero = vec![];
    let mut team = vec![];
    for (i, occupation) in OCCUPATION_EACH_HERO.iter().enumerate() {
        if *occupation == 0 {
            duelist_hero.push(NAMES_CN[i]);
        } else if *occupation == 1 {
            sentinel_team.push(NAMES_CN[i]);
        } else if *occupation == 2 {
            controller_hero.push(NAMES_CN[i]);
        } else {
            initiator_hero.push(NAMES_CN[i]);
        }
    }
    team.extend(basic_generate(duelist_hero, duelist_number));
    team.extend(basic_generate(sentinel_team, sentinel_number));
    team.extend(basic_generate(controller_hero, controller_number));
    team.extend(basic_generate(initiator_hero, initiator_number));
    team
}

fn basic_generate(hero_names: Vec<&str>, count: u32) -> Vec<String> {
    let mut hero_names = hero_names.clone();
    let hero_number = hero_names.len() as u32;
    let mut team = vec![];
    let mut rng = rand::rng();
    for i in 0..count {
        let rng_range = hero_number - i;
        let number: u32 = rng.random_range(..rng_range);
        team.push(String::from(hero_names[number as usize]));
        hero_names.remove(number as usize);
    }
    team
}
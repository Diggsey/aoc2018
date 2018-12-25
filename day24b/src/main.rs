use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone)]
struct Group {
    side: Side,
    num_units: i32,
    hitpoints: i32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    attack_type: String,
    damage: i32,
    initiative: i32,
    target: Option<usize>,
    is_attacked: bool,
    side_index: i32,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.num_units * self.damage
    }
    fn damage_to(&self, other: &Group) -> i32 {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            self.effective_power()*2
        } else {
            self.effective_power()
        }
    }
    fn attack_priority(&self, other: &Group) -> (i32, i32, i32) {
        (
            self.damage_to(other),
            other.effective_power(),
            other.initiative,
        )
    }
    fn deal_damage(&mut self, damage: i32) -> i32 {
        let mut n = damage / self.hitpoints;
        if n > self.num_units {
            n = self.num_units;
        }

        self.num_units -= n;
        n
    }
}

fn split_sep<'a>(s: &'a str, sep: &str) -> (&'a str, &'a str) {
    let mut iter = s.splitn(2, sep);
    (iter.next().unwrap(), iter.next().unwrap())
}

fn parse_sep<'a, T: FromStr>(s: &mut &str, sep: &str) -> T where <T as FromStr>::Err: Debug {
    let (a, b) = split_sep(s, sep);
    *s = b;
    a.parse().unwrap()
}

fn eat_until_sep<'a>(s: &mut &'a str, sep: &[char]) -> &'a str {
    let offset = s.find(sep).unwrap();
    let result = &s[..offset];
    *s = &s[offset..];
    result
}

fn try_eat(s: &mut &str, p: &str) -> bool {
    if s.starts_with(p) {
        *s = &s[p.len()..];
        true
    } else {
        false
    }
}

fn parse_affinities(s: &mut &str) -> (Vec<String>, Vec<String>) {
    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();

    loop {
        if try_eat(s, "weak to ") {
            weaknesses.extend(
                eat_until_sep(s, &[';', ')']).split(", ").map(String::from)
            );
            if !try_eat(s, "; ") { break; }
        } else if try_eat(s, "immune to ") {
            immunities.extend(
                eat_until_sep(s, &[';', ')']).split(", ").map(String::from)
            );
            if !try_eat(s, "; ") { break; }
        } else {
            break;
        }
    }
    (weaknesses, immunities)
}

fn parse_group(side: Side, mut line: &str, side_index: i32) -> Group {
    let num_units = parse_sep(&mut line, " units each with ");
    let hitpoints = parse_sep(&mut line, " hit points ");
    let (weaknesses, immunities) = if try_eat(&mut line, "(") {
        let aff = parse_affinities(&mut line);
        assert!(try_eat(&mut line, ") "));
        aff
    } else {
        (Vec::new(), Vec::new())
    };
    assert!(try_eat(&mut line, "with an attack that does "));
    let damage = parse_sep(&mut line, " ");
    let attack_type = parse_sep(&mut line, " damage at initiative ");
    let initiative = line.parse().unwrap();

    Group {
        side,
        num_units,
        hitpoints,
        weaknesses,
        immunities,
        attack_type,
        damage,
        initiative,
        target: None,
        is_attacked: false,
        side_index,
    }
}

fn fight_round(groups: &mut [Group]) -> bool {
    // Reset round
    for group in groups.iter_mut() {
        group.target = None;
        group.is_attacked = false;
    }

    let mut any_attacked = false;

    // Perform targeting
    let mut target_ordering: Vec<_> = (0..groups.len()).collect();
    target_ordering.sort_by_key(|&index| (groups[index].effective_power(), groups[index].initiative));
    target_ordering.reverse();

    for attacker_index in target_ordering {
        let attacker = &groups[attacker_index];
        if attacker.num_units <= 0 { continue; }
        let target = groups.iter()
            .enumerate()
            .filter(|(_, group)| {
                (group.side != attacker.side) &&
                (group.num_units > 0) &&
                !group.is_attacked &&
                (attacker.damage_to(group) > 0)
            })
            .max_by_key(|(_, group)| {
                attacker.attack_priority(group)
            })
            .map(|(index, _)| index);

        groups[attacker_index].target = target;
        if let Some(defender_index) = target {
            groups[defender_index].is_attacked = true;
        } 
    }

    // Perform attacking
    let mut attack_ordering: Vec<_> = (0..groups.len()).collect();
    attack_ordering.sort_by_key(|&index| groups[index].initiative);
    attack_ordering.reverse();

    for attacker_index in attack_ordering {
        let attacker = &groups[attacker_index];
        if attacker.num_units <= 0 { continue; }
        if let Some(defender_index) = attacker.target {
            let damage = attacker.damage_to(&groups[defender_index]);
            // print!("{:?} {} attacks", attacker.side, attacker.side_index);
            let n = groups[defender_index].deal_damage(damage);
            if n > 0 {
                any_attacked = true;
            }
            // println!(" defending group {}, killing {} units", groups[defender_index].side_index, n);
        }
    }
    any_attacked
}

fn main() {
    let mut current_side = Side::ImmuneSystem;
    let mut initial_groups = Vec::new();
    let mut side_index = 0;

    for line in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
    {
        if line == "Immune System:" {
            current_side = Side::ImmuneSystem;
            side_index = 1;
            continue;
        } else if line == "Infection:" {
            current_side = Side::Infection;
            side_index = 1;
            continue;
        } else if line.is_empty() {
            continue;
        }

        initial_groups.push(parse_group(current_side, &line, side_index));
        side_index += 1;
    }

    println!("{:?}", initial_groups);

    'next: for boost in 1.. {
        println!("boost: {}", boost);
        let mut groups = initial_groups.clone();

        for group in groups.iter_mut() {
            if group.side == Side::ImmuneSystem {
                group.damage += boost;
            }
        }

        loop {
            if !groups.iter().any(|group| group.num_units > 0 && group.side == Side::ImmuneSystem) {
                continue 'next;
            }
            if !groups.iter().any(|group| group.num_units > 0 && group.side == Side::Infection) {
                println!("Success!");
                break;
            }
            if !fight_round(&mut groups) {
                continue 'next;
            }
        }

        let remaining: i32 = groups.iter().map(|group| if group.side == Side::ImmuneSystem { group.num_units } else { 0 }).sum();
        println!("{}", remaining);
        break;
    }
}

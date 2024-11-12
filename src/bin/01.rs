use std::cmp::PartialEq;
use everybody_codes_2024;
use everybody_codes_2024::solution;

solution!(1);

fn part_one(input: String) -> u32 {
    let enemies: Vec<EnemyType> = input.chars().filter_map(|c| match c {
        'A' => Some(EnemyType::A),
        'B' => Some(EnemyType::B),
        'C' => Some(EnemyType::C),
        _ => None,
    }).collect();

    let mut potions_needed = 0;
    for enemy in enemies {
        println!("{:?}", enemy);
        match enemy {
            EnemyType::A => {}
            EnemyType::B => { potions_needed += 1 }
            EnemyType::C => { potions_needed += 3 }
            _ => {}
        }
    }

    println!("{}", potions_needed);
    potions_needed
}
fn part_two(input: String) -> u32 {
    let enemy_types: Vec<EnemyType> = input.chars().map(|c| c.into()).collect();

    let mut potions_needed = 0;
    for chunk in enemy_types.chunks(2) {
        if let [enemy1, enemy2] = chunk {
            if enemy1 == &EnemyType::x || enemy2 == &EnemyType::x {
                potions_needed += enemy1.get_potions_part_2() + enemy2.get_potions_part_2();
            } else {
                potions_needed += enemy1.get_potions_part_2() + enemy2.get_potions_part_2() + 2;
            }
        }
    }

    println!("{}", potions_needed);
    potions_needed
}

pub fn part_three(input: String) -> u32{
    let enemy_types: Vec<EnemyType> = input.chars().map(|c| c.into()).collect();

    let mut potions_needed = 0;
    for chunk in enemy_types.chunks(3) {
        if let [enemy1, enemy2, enemy3] = chunk {
            let valids= chunk.iter().filter(|e| e != &&EnemyType::x).count() as u32;
            let potions = chunk.iter().map(|e| e.get_potions_part_2()).sum::<u32>();
            println!("{}", valids);
            if valids == 1{
                potions_needed += (potions + (valids *0));
            } else if valids ==2 {
                potions_needed += (potions + (valids *1));
            }
        else if valids ==3 {
            potions_needed += (potions + (valids *2));
        }
        }
    }

    println!("{}", potions_needed);
    potions_needed
}

#[derive(Debug, PartialEq, Eq)]
enum EnemyType {
    A,
    B,
    C,
    D,
    x,
}


impl EnemyType{
    fn get_potions_part_2(&self) -> u32 {
        let potions = match self {
            EnemyType::A => 0,
            EnemyType::B => 1,
            EnemyType::C => 3,
            EnemyType::D => 5,
            EnemyType::x => 0,
        };
        potions
    }
}

impl From<char> for EnemyType{
    fn from(value: char) -> Self {
        match value {
            'A' => EnemyType::A,
            'B' => EnemyType::B,
            'C' => EnemyType::C,
            'D' => EnemyType::D,
            'x' => EnemyType::x,
            _ => panic!("Invalid enemy type")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = "AxBCDDCAxD".to_string();
        assert_eq!(part_two(input), 28);
    }

    #[test]
    fn test_part_three() {
        let input = "xBxAAABCDxCC".to_string();
        assert_eq!(part_three(input), 30);
    }

    #[test]
    fn test_part_threexbx() {
        let input = "xBx".to_string();
        assert_eq!(part_three(input), 1);
    }
}
use std::cmp::Ordering;
use std::io::{self, prelude::*};

fn main() {
    let (map, mut entities) = parse_input();
    print_map(&map);
    print_map_with_entities(&map, &entities);
    for _ in 0..1 {
        entities.sort();
        print_entities(&entities);
        for entity in &entities {
            // move the entity if it is not in range of the goblins
            match get_closest_enemy(entity, &entities) {
                Some(Val) => unimplemented!(),
                None => break,
            }
            // attack if in range
        }
    }
}

fn get_closest_enemy(e: &Entity, entities: &Vec<Entity>) -> Option<&Entity> {
    let closest_entity = None;
    for entity in entities {
        if entity != e && entity.pos.1
    }
    closest_entity
}

fn parse_input() -> (Vec<Vec<char>>, Vec<Entity>) {
    let stdin = io::stdin();
    let mut map = vec![];
    let mut entities = vec![];
    for (i, line) in stdin.lock().lines().enumerate() {
        let mut new_row = vec![];
        for (j, tile) in line.unwrap().chars().enumerate() {
            match tile {
                'G' => {
                    entities.push(Entity {
                        hp: 200,
                        pos: (j, i),
                        race: Race::Goblin,
                    });
                    new_row.push('.');
                }
                'E' => {
                    entities.push(Entity {
                        hp: 200,
                        pos: (j, i),
                        race: Race::Elf,
                    });
                    new_row.push('.');
                }
                _ => new_row.push(tile),
            }
        }
        map.push(new_row);
    }
    (map, entities)
}

fn print_entities(entities: &Vec<Entity>) {
    for e in entities {
        println!("{:?}", e);
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in map {
        for j in i {
            print!("{}", j);
        }
        println!("");
    }
}

fn print_map_with_entities(map: &Vec<Vec<char>>, entities: &Vec<Entity>) {
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let mut t = false;
            for e in entities {
                if e.pos == (i, j) {
                    print!("{}", e.race);
                    t = true;
                    break;
                }
            }
            if !t {
                print!("{}", tile);
            }
        }
        println!("");
    }
}

#[derive(Eq, Debug)]
struct Entity {
    hp: usize,
    pos: (usize, usize),
    race: Race,
}

impl Ord for Entity {
    fn cmp(&self, other: &Entity) -> Ordering {
        match self.pos.1.cmp(&other.pos.1) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.pos.0.cmp(&other.pos.0),
        }
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Entity) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.pos == other.pos
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Race {
    Goblin,
    Elf,
}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Race::Goblin => write!(f, "G"),
            Race::Elf => write!(f, "E"),
        }
    }
}

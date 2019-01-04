use std::io::{self, prelude::*};

fn main() {
    let (map, mut entities) = parse_input();
    print_map(&map);
    print_map_with_entities(&map, &entities);
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
            for e in entities {
                if e.pos == (i, j) {
                    print!("{}", e.race);
                } else {
                    print!("{}", tile);
                }
            }
        }
        println!("");
    }
}

struct Entity {
    hp: usize,
    pos: (usize, usize),
    race: Race,
}

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

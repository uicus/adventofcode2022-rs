use crate::utils;
use std::cmp::{max, min};
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Solid,
    Sand,
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    middle: usize,
}

impl Map {
    fn x_coord(&self, x: isize) -> isize {
        x - (500 - self.middle as isize)
    }

    fn inside(&self, coords: (isize, isize)) -> bool {
        let true_x = self.x_coord(coords.0);
        coords.1 >= 0
            && coords.1 < self.tiles.len() as isize
            && true_x >= 0
            && true_x < self.tiles[0].len() as isize
    }

    fn set(&mut self, coords: (isize, isize), tile: Tile) {
        let true_x = self.x_coord(coords.0);
        self.tiles[coords.1 as usize][true_x as usize] = tile;
    }

    fn get(&self, coords: (isize, isize)) -> Tile {
        let true_x = self.x_coord(coords.0);
        self.tiles[coords.1 as usize][true_x as usize]
    }

    fn fill_between(&mut self, coords1: (isize, isize), coords2: (isize, isize)) {
        if coords1.0 == coords2.0 {
            for y in min(coords1.1, coords2.1)..=max(coords1.1, coords2.1) {
                self.set((coords1.0, y), Tile::Solid);
            }
        } else if coords1.1 == coords2.1 {
            for x in min(coords1.0, coords2.0)..=max(coords1.0, coords2.0) {
                self.set((x, coords1.1), Tile::Solid);
            }
        }
    }

    pub fn count_sand(&self) -> u32 {
        let mut result = 0;
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Empty => {}
                    Tile::Solid => {}
                    Tile::Sand => {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

pub fn drop_sand(map: &mut Map, mut point: (isize, isize)) -> bool {
    if map.get(point) == Tile::Sand {
        return true;
    }
    loop {
        let lower = (point.0, point.1 + 1);
        if !map.inside(lower) {
            return true;
        }
        if map.get(lower) == Tile::Empty {
            point = lower;
            continue;
        }
        let lower_left = (point.0 - 1, point.1 + 1);
        if !map.inside(lower_left) {
            return true;
        }
        if map.get(lower_left) == Tile::Empty {
            point = lower_left;
            continue;
        }
        let lower_right = (point.0 + 1, point.1 + 1);
        if !map.inside(lower_right) {
            return true;
        }
        if map.get(lower_right) == Tile::Empty {
            point = lower_right;
            continue;
        }
        map.set(point, Tile::Sand);
        return false;
    }
}

pub fn create_map(solids: &Vec<Vec<(isize, isize)>>) -> Map {
    let lowermost = solids
        .iter()
        .map(|l| l.iter().map(|(_, y)| y).max().unwrap_or(&0))
        .max()
        .unwrap_or(&0);
    let rightmost = solids
        .iter()
        .map(|l| l.iter().map(|(x, _)| x).max().unwrap_or(&500))
        .max()
        .unwrap_or(&500);
    let leftmost = solids
        .iter()
        .map(|l| l.iter().map(|(x, _)| x).min().unwrap_or(&500))
        .min()
        .unwrap_or(&500);
    let mut map = Map {
        tiles: vec![
            vec![Tile::Empty; (rightmost - leftmost + 1) as usize];
            (lowermost + 1) as usize
        ],
        middle: 500 - *leftmost as usize,
    };
    for solid in solids {
        let mut previous_point = solid[0];
        for point in &solid[1..] {
            map.fill_between(previous_point, *point);
            previous_point = *point;
        }
    }
    map
}

fn parse_point(text: &str) -> Result<(isize, isize), Box<dyn std::error::Error>> {
    let parts = text.split(",").collect::<Vec<_>>();
    if parts.len() != 2 {
        Err(Box::new(utils::ParseInputError))
    } else {
        Ok((parts[0].parse()?, parts[1].parse()?))
    }
}

fn parse_line(line: &str) -> Result<Vec<(isize, isize)>, Box<dyn std::error::Error>> {
    line.split(" -> ").map(parse_point).collect()
}

pub fn read(filename: &str) -> Result<Vec<Vec<(isize, isize)>>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}

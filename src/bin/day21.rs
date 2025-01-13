use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let (part1, part2) = solve();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn solve() -> (u16, u16) {
    let mut win_gold = vec![];
    let mut lose_gold = vec![];

    for weapon_item in weapons() {
        for armor_item in armor().combinations(0).chain(armor().combinations(1)) {
            for ring_item in rings()
                .combinations(0)
                .chain(rings().combinations(1))
                .chain(rings().combinations(2))
            {
                let mut damage = 0;
                let mut armor = 0;
                let mut cost = 0;

                damage += weapon_item.damage;
                cost += weapon_item.cost;

                for a in armor_item.iter() {
                    armor += a.armor;
                    cost += a.cost;
                }

                for r in ring_item {
                    damage += r.damage;
                    armor += r.armor;
                    cost += r.cost;
                }

                let player = Player {
                    hp: 100,
                    damage,
                    armor,
                };

                let boss = boss();

                if battle(player, boss) {
                    win_gold.push(cost);
                } else {
                    lose_gold.push(cost);
                }
            }
        }
    }

    (
        win_gold.into_iter().min().unwrap(),
        lose_gold.into_iter().max().unwrap(),
    )
}

fn battle(mut player: Player, mut boss: Player) -> bool {
    let mut turn = 0;
    while player.hp > 0 && boss.hp > 0 {
        let (current, mut other) = if turn % 2 == 0 {
            (player, boss)
        } else {
            (boss, player)
        };

        let damage = (current.damage - other.armor).max(1);
        other.hp -= damage;

        (player, boss) = if turn % 2 == 0 {
            (current, other)
        } else {
            (other, current)
        };
        turn += 1;
    }
    player.hp > 0
}

#[derive(PartialEq, Debug)]
struct Player {
    hp: i8,
    damage: i8,
    armor: i8,
}

#[derive(Debug, Clone)]
struct GameItem {
    cost: u16,
    damage: i8,
    armor: i8,
}

impl FromStr for GameItem {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = regex::Regex::new("  +").unwrap();
        }
        // Skip the item name, we don't need it.
        let mut line = RE.split(s).skip(1);
        let cost = line.next().unwrap().parse()?;
        let damage = line.next().unwrap().parse()?;
        let armor = line.next().unwrap().parse()?;
        Ok(GameItem {
            cost,
            damage,
            armor,
        })
    }
}

const ITEM_SHOP: &str = "Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

fn weapons() -> impl Iterator<Item = GameItem> {
    ITEM_SHOP.lines().skip(1).take(5).flat_map(GameItem::from_str)
}

fn armor() -> impl Iterator<Item = GameItem> {
    ITEM_SHOP.lines().skip(8).take(5).flat_map(GameItem::from_str)
}

fn rings() -> impl Iterator<Item = GameItem> {
    ITEM_SHOP.lines().skip(15).take(6).flat_map(GameItem::from_str)
}

fn boss() -> Player {
    let s = include_str!("../../puzzles/day21.txt").trim();
    lazy_static! {
        static ref RE: Regex = regex::Regex::new(r"(?P<x>\d+)").unwrap();
    }
    let mut m = RE.captures_iter(s);
    let hp = m.next().unwrap()["x"].parse().unwrap();
    let damage = m.next().unwrap()["x"].parse().unwrap();
    let armor = m.next().unwrap()["x"].parse().unwrap();
    Player { hp, damage, armor }
}

#[cfg(test)]
mod day21 {
    use super::*;

    #[test]
    fn boss_battle() {
        let player = Player {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Player {
            hp: 12,
            damage: 7,
            armor: 2,
        };
        // Player should win
        assert!(battle(player, boss))
    }
}

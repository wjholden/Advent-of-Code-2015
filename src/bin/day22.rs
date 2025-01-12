use std::collections::{BinaryHeap, HashMap, VecDeque};

fn main() {
    println!("Part 1: {}", bfs(Mode::Easy));
    println!("Part 2: {}", bfs(Mode::Hard));
}

/// I can't believe this actually works!
/// https://www.reddit.com/r/adventofcode/comments/174g1o4/comment/k4a1b0k/
/// This problem must have been designed to enable a greedy approach.
/// So long as we build our spell list in non-decreasing order by mana cost,
/// the very first winning move will be the global solution. Must have taken
/// some amazing puzzle engineering to guarantee this was the unique solution...
fn bfs(mode: Mode) -> i16 {
    let spell_list = [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Poison,
        Spell::Shield,
        Spell::Recharge,
    ];

    #[derive(Eq, PartialEq, Debug)]
    struct Spellbook {
        spells: Vec<Spell>
    }
    impl Ord for Spellbook {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let a: i16 = self.spells.iter().map(Spell::mana_cost).sum();
            let b: i16 = other.spells.iter().map(Spell::mana_cost).sum();
            //a.cmp(&b)
            b.cmp(&a) // reversed for min-heap.
        }
    }
    impl PartialOrd for Spellbook {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other)) // not reversed.
        }
    }

    let mut heap = BinaryHeap::new();
    for spell in spell_list.iter() {
        heap.push(Spellbook { spells: vec![spell.clone()] });
    }

    loop {
        let current = heap.pop().unwrap();
        //println!("{current:?}");
        let player = Player {
            hp: 50,
            armor: 0,
            damage: 0,
        };
        let boss = Player::new_boss();
        match battle(player, boss, &current.spells, 500, &mode) {
            Outcome::Win(x) => return x,
            Outcome::Lose => (),
            Outcome::Incomplete(_) => {
                for spell in spell_list.iter() {
                    let mut candidate = current.spells.clone();
                    candidate.push(spell.clone());
                    heap.push(Spellbook { spells: candidate });
                }
            }
        }
    }
}

/// Theoretically DFS would *eventually* find the solution, but there are a
/// **lot** of possible spell sequences that we would have to test. In practice,
/// this isn't going to get you to an answer.
#[allow(dead_code)]
fn dfs(spells: &mut Vec<Spell>) -> i16 {
    let mut mana = i16::MAX;
    for spell in [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Poison,
        Spell::Shield,
        Spell::Recharge,
    ] {
        if !spells.is_empty() && spell == spells[spells.len() - 1] {
            continue
        }

        let player = Player {
            hp: 50,
            armor: 0,
            damage: 0,
        };
        let boss = Player::new_boss();
        spells.push(spell);
        mana = mana.min(match battle(player, boss, spells, 500, &Mode::Easy) {
            Outcome::Win(x) => x,
            Outcome::Lose => mana,
            Outcome::Incomplete(_) => dfs(spells),
        });
        spells.pop();
    }
    //println!("best {mana} at {spells:?}");
    mana
}

enum Mode {
    Easy,
    Hard
}

fn battle(mut player: Player, mut boss: Player, spells: &[Spell], mana: i16, mode: &Mode) -> Outcome {
    let mut cast: HashMap<&Spell, u8> = HashMap::new();
    let mut spells = VecDeque::from_iter(spells);
    let mut turn = 0;
    let mut mana = mana;
    let mut mana_spent = 0;

    loop {
        if spells.is_empty() && turn % 2 == 0 {
            return Outcome::Incomplete(mana_spent);
        }

        // Part 2
        if turn % 2 == 0 {
            match mode {
                Mode::Hard => {
                    player.hp -= 1;
                    if player.hp <= 0 {
                        return Outcome::Lose
                    }
                },
                Mode::Easy => (),
            }
        }

        //println!("-- {} turn --", if turn % 2 == 0 { "Player" } else { "Boss" });
        //println!("- Player has {} hit points, {} armor, {} mana", player.hp, player.armor, mana);
        //println!("- Boss has {} hit points", boss.hp);

        for (spell, count) in cast.iter_mut() {
            if *count > 0 {
                *count -= 1;
                match spell {
                    Spell::Shield => {
                        //println!("Shield's timer is now {count}.");
                        if *count == 0 {
                            player.armor = 0;
                            //println!("Shield wears off, decreasing armor by 7.");
                        }
                    }
                    Spell::Poison => {
                        boss.hp -= 3;
                        //println!("Poison deals 3 damage; its timer is now {count}.");
                    }
                    Spell::Recharge => {
                        mana += 101;
                        //println!("Recharge provides 101 mana, its timer is now {count}.");
                    }
                    _ => unreachable!(),
                }
            }
        }

        if player.hp <= 0 {
            return Outcome::Lose;
        }
        if boss.hp <= 0 {
            return Outcome::Win(mana_spent);
        }

        if turn % 2 == 0 {
            // player's turn
            let spell = spells.pop_front().unwrap();
            //println!("Player casts {spell:?}.");

            // Spell cannot be active
            let count = *cast.entry(spell).or_insert(0);
            if count > 0 {
                //println!("Cannot cast an active spell a second time.");
                return Outcome::Lose;
            }

            // Need mana
            let cost = spell.mana_cost();
            if cost > mana {
                return Outcome::Lose;
            }
            mana -= cost;
            mana_spent += cost;

            // immediate spells
            match spell {
                Spell::MagicMissile => boss.hp -= 4,
                Spell::Drain => {
                    player.hp += 2;
                    boss.hp -= 2;
                },
                Spell::Shield => player.armor = 7,
                _ => (),
            }

            // effect spells
            let effect_turns = spell.turns();
            if effect_turns > 0 {
                *cast.get_mut(spell).unwrap() = effect_turns;
            }
        } else {
            // boss's turn
            let damage = (boss.damage - player.armor).max(1);
            if player.armor > 0 {
                //println!("Boss attacks for {} - {} = {} damage!", boss.damage, player.armor, damage);
            } else {
                //println!("Boss attacks for {} damage!", damage)
            }
            player.hp -= damage;
        }

        turn += 1;
        //println!();
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    Win(i16),
    Lose,
    Incomplete(i16),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn mana_cost(&self) -> i16 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn turns(&self) -> u8 {
        match self {
            Spell::Shield => 6,
            Spell::Poison => 6,
            Spell::Recharge => 5,
            _ => 0,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Player {
    hp: i8,
    damage: i8,
    armor: i8,
}

impl Player {
    fn new_boss() -> Player {
        let v: Vec<_> = include_str!("../../puzzles/day22.txt")
            .trim()
            .lines()
            .map(|line| {
                let l = line.split(": ");
                l.last().unwrap().parse().unwrap()
            })
            .collect();
        Player {
            hp: v[0],
            damage: v[1],
            armor: 0,
        }
    }
}

#[cfg(test)]
mod day22 {
    use super::*;

    #[test]
    fn short_battle() {
        let player = Player {
            hp: 10,
            armor: 0,
            damage: 0,
        };
        let boss = Player {
            hp: 13,
            armor: 0,
            damage: 8,
        };
        let spells = [Spell::Poison, Spell::MagicMissile];
        let cost = spells.iter().map(Spell::mana_cost).sum();
        let mana = 250;
        assert_eq!(battle(player, boss, &spells, mana, &Mode::Easy), Outcome::Win(cost))
    }

    #[test]
    fn double_recharge() {
        let player = Player {
            hp: 10,
            armor: 0,
            damage: 0,
        };
        let boss = Player {
            hp: 13,
            armor: 0,
            damage: 8,
        };
        let spells = [Spell::Recharge, Spell::MagicMissile, 
        Spell::Shield, Spell::Drain, Spell::Recharge];
        assert_eq!(battle(player, boss, &spells, 250, &Mode::Easy), Outcome::Lose)
    }

    #[test]
    fn big_battle() {
        let player = Player {
            hp: 10,
            armor: 0,
            damage: 0,
        };
        let boss = Player {
            hp: 14,
            armor: 0,
            damage: 8,
        };
        let spells = [
            Spell::Recharge,
            Spell::Shield,
            Spell::Drain,
            Spell::Poison,
            Spell::MagicMissile,
        ];
        let cost = spells.iter().map(Spell::mana_cost).sum();
        let mana = 250;
        assert_eq!(battle(player, boss, &spells, mana, &Mode::Easy), Outcome::Win(cost))
    }
}

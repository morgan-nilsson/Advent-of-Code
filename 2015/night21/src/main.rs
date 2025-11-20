/**
 * Weapons:    Cost  Damage  Armor
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
    Defense +3   80     0       3
 */

const WEAPONS: [(i32, i32, i32); 5] = [
    (8, 4, 0),
    (10, 5, 0),
    (25, 6, 0),
    (40, 7, 0),
    (74, 8, 0),
];

const ARMORS: [(i32, i32, i32); 6] = [
    (0, 0, 0),      // No armor option
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];

const RINGS: [(i32, i32, i32); 8] = [
    (0, 0, 0),      // No ring option 1
    (0, 0, 0),      // No ring option 2
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

 /**
  * Hit Points: 100
    Damage: 8
    Armor: 2
  */

const BOSS_HIT_POINTS: i32 = 100;
const BOSS_DAMAGE: i32 = 8;
const BOSS_ARMOR: i32 = 2;

fn main() {

    let mut max_cost_to_lose = i32::MIN;

    for weapon in WEAPONS.iter() {
        for armor in ARMORS.iter() {
            for ring1_index in 0..RINGS.len() {
                for ring2_index in ring1_index + 1..RINGS.len() {
                    let ring1 = RINGS[ring1_index];
                    let ring2 = RINGS[ring2_index];

                    let total_cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                    let total_damage = weapon.1 + armor.1 + ring1.1 + ring2.1;
                    let total_armor = weapon.2 + armor.2 + ring1.2 + ring2.2;

                    let player_hit_points = 100;
                    let boss_hit_points = BOSS_HIT_POINTS;

                    let player_damage_per_turn = (total_damage - BOSS_ARMOR).max(1);
                    let boss_damage_per_turn = (BOSS_DAMAGE - total_armor).max(1);

                    let turns_to_defeat_boss = (boss_hit_points + player_damage_per_turn - 1) / player_damage_per_turn;
                    let turns_to_defeat_player = (player_hit_points + boss_damage_per_turn - 1) / boss_damage_per_turn;

                    if turns_to_defeat_boss > turns_to_defeat_player {
                        max_cost_to_lose = max_cost_to_lose.max(total_cost);
                    }
                }
            }
        }
    }

    println!("Minimum cost to win: {}", max_cost_to_lose);
}

use std::collections::HashMap;


//Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
//Cupid can fly 8 km/s for 17 seconds, but then must rest for 114 seconds.
//Prancer can fly 18 km/s for 6 seconds, but then must rest for 103 seconds.
//Donner can fly 25 km/s for 6 seconds, but then must rest for 145 seconds.
//Dasher can fly 11 km/s for 12 seconds, but then must rest for 125 seconds.
//Comet can fly 21 km/s for 6 seconds, but then must rest for 121 seconds.
//Blitzen can fly 18 km/s for 3 seconds, but then must rest for 50 seconds.
//Vixen can fly 20 km/s for 4 seconds, but then must rest for 75 seconds.
//Dancer can fly 7 km/s for 20 seconds, but then must rest for 119 seconds.

fn main() {

    let mut reindeer_map: HashMap<&str, (u32, u32, u32, u32, u32)> = HashMap::new();
    reindeer_map.insert("Rudolph", (22, 8, 165, 0, 0));
    reindeer_map.insert("Cupid", (8, 17, 114, 0, 0));
    reindeer_map.insert("Prancer", (18, 6, 103, 0, 0));
    reindeer_map.insert("Donner", (25, 6, 145, 0, 0));
    reindeer_map.insert("Dasher", (11, 12, 125, 0, 0));
    reindeer_map.insert("Comet", (21, 6, 121, 0, 0));
    reindeer_map.insert("Blitzen", (18, 3, 50, 0, 0));
    reindeer_map.insert("Vixen", (20, 4, 75, 0, 0));
    reindeer_map.insert("Dancer", (7, 20, 119, 0, 0));

    const MAX_TIME: u32 = 2503;

    for _ in 0..MAX_TIME {
        for (_name, (speed, fly_time, rest_time, time_in_state, distance)) in reindeer_map.iter_mut() {
            if (*time_in_state) < *fly_time {
                *distance += *speed;
            }
            *time_in_state += 1;
            if *time_in_state == *fly_time + *rest_time {
                *time_in_state = 0;
            }
        }

    }
    let max_distance = reindeer_map.values().map(|&(_, _, _, _, d)| d).max().unwrap();
    println!("Max distance: {}", max_distance);
}
static INPUT: usize = 29000000;

fn main() {

    let mut arr = vec![0; INPUT];

    // for every elf
    for i in 1..INPUT {
        // for every house that elf visits
        let mut visits = 0;
        for j in (i..INPUT).step_by(i) {
            arr[j] += i * 11;
            visits += 1;
            if visits == 50 {
                break;
            }
        }

    }

    for i in 0..INPUT {
        if arr[i] >= INPUT {
            println!("House number: {}", i);
            return;
        }
    }

}

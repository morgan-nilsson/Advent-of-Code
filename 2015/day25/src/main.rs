static ROW: u64 = 2947;
static COLUMN: u64 = 3029;
static MULTIPLIER: u64 = 252533;
static MODULUS: u64 = 33554393;
static START_CODE: u64 = 20151125;

fn main() {

    let diagonal_before = ROW + COLUMN - 2;
    let index = (diagonal_before * (diagonal_before + 1)) / 2 + COLUMN - 1;

    let mut value: u64 = START_CODE;
    for _ in 0..index {
        value = (value * MULTIPLIER) % MODULUS;
    }

    println!("The code at row {}, column {} is {}", ROW, COLUMN, value);
}

use md5::compute;

const SECRET_KEY: &str = "ckczppom";

fn main() {

    let mut i = 0;
    loop {
        let key = format!("{}{}", SECRET_KEY, i);
        let digest = compute(key);
        let hex_str = format!("{:x}", digest);

        // Take the first 5 characters
        let first_5 = &hex_str[..5];
        if first_5 == "00000" {
            print!("{}", i);
            return;
        }
        i += 1;
    }

}

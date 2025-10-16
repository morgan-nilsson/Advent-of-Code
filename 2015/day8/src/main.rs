static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut real_size = 0;
    let mut evaluated_size = 0;
    for line in INPUT.lines() {
        let mut line = line.chars();
        let line_len = line.clone().count();
        real_size += line_len;
        loop {
            let c = line.next();
            if c.is_none() == true {
                break;
            }
            let c = c.unwrap();

            match c {
                '\\' => {
                    let next_char = line.next().unwrap();
                    match next_char {
                        '\\' | '"' => {
                            println!("/ or double quote escape sequence");
                            evaluated_size += 1;
                        }
                        'x' => {
                            println!("escape sequence");
                            line.next();
                            line.next();
                            evaluated_size += 1;
                        }
                        _ => {
                            println!("{:?}", line);
                            panic!("Invalid escape sequence")
                        }
                    }

                }
                '\"' => {

                }
                _ => {
                    evaluated_size += 1;
                }
            }
        }
    }

    println!("{} - {} = {}", real_size, evaluated_size, real_size - evaluated_size);
}


fn main() {
    let password = "hxbxwxba";
    // ensure ascii
    if !password.is_ascii() {
        println!("password is not ascii");
        return;
    }

    let mut chars: Vec<char> = password.chars().collect();
    loop {
        for i in (0..chars.len()).rev() {
            if chars[i] == 'z' {
                chars[i] = 'a';
            } else {
                chars[i] = ((chars[i] as u8) + 1) as char;
                break;
            }
        }
        let new_password: String = chars.clone().into_iter().collect();
        println!("Next password to try: {}", new_password);

        if verify_password(&new_password) {
            println!("Next valid password: {}", new_password);
            break;
        }
    }
}

fn verify_password(input: &str) -> bool {
    if input.contains('i') || input.contains('o') || input.contains('l') {
        return false;
    }

    let chars: Vec<char> = input.chars().collect();

    let mut has_run = false;
    for i in 0..chars.len() - 2 {
        if chars[i] as u8 + 1 == chars[i + 1] as u8 &&
           chars[i + 1] as u8 + 1 == chars[i + 2] as u8 {
            has_run = true;
            break;
        }
    }

    let mut pairs = std::collections::HashSet::new();
    let mut i = 0;
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            pairs.insert(chars[i]);
            i += 2;
        } else {
            i += 1;
        }
    }

    has_run && pairs.len() >= 2
}


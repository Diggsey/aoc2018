use std::collections::HashSet;

fn main() {
    let mut current = 0u64;
    let mut seen = HashSet::new();
    loop {
        let mut prev = current | 65536u64;
        current = 13431073u64;

        loop {
            current = (current + (prev & 255))*65899 & 16777215;

            if prev < 256 { break; }

            prev /= 256;
        }

        if !seen.insert(current) {
            break;
        } else {
            println!("{}", current);
        }
    }
}

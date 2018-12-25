fn main() {
    let mut prev = 65536u64;
    let mut current = 13431073u64;

    loop {
        current = (current + (prev & 255))*65899 & 16777215;

        if prev < 256 { break; }

        prev /= 256;
    }

    print!("{}", current);
}

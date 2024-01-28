fn main() {
    let mut a = 1;
    loop {
        let b = a * a;
        if b >= 200 { break; }
        print!("{} ", b);
        a += 1;
    }
}

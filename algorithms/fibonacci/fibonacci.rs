fn main() {
    let mut fib = [1; 12];
    for a in 2..fib.len() {
        fib[a] = fib[a - 2] + fib[a - 1];
    }
    for a in 0..fib.len() {
        println!("{} ", fib[a]);
    }
}

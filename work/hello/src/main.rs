fn main() {
    fn sumup(n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            n + sumup(n - 1)
        }
    }

    println!("{}", sumup(100000));
}


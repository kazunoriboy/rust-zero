fn main() {
    fn sumup_loop(mut n: u64) -> u64 {
        let mut total = 0;
        loop {
            if n == 0 {
                break;
            }
            total += n;
            n -= 1;
        }
        total
    }
    println!("{}", sumup_loop(10));
}


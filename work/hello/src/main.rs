fn main() {
    fn sumup_while(mut n: u64) -> u64 {
        let mut total = 0;
        while n > 0 {
            total += n;
            n -= 1;
        }
        total
    }
    println!("{}", sumup_while(10));
}


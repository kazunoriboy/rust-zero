fn main() {
    fn sumup_for(n: u64) -> u64 {
        let mut total = 0;
        for x in 0..=n {
            total += x;
        }
        total
    }
    println!("{}", sumup_for(10));
}


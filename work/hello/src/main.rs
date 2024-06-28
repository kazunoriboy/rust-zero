fn main() {
    let mut n: u64 = 100;
     
    let a: &u64 = &n;
    // *a = 200;
    println!("*a = {}, addr = {:p}", *a, a);

    let b: &mut u64 = &mut n;
    println!("{:p}", b);
    *b = 200;
    println!("n = {n}");
}

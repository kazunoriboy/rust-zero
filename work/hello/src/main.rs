fn main() {
    let a: &str = " hello";
    // a += ", world!";

    let mut b: String = a.to_string();
    b += ", world!";

    let c: &str = b.trim();
    println!("{c}");
}

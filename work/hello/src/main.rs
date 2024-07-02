fn main() {
    let n = -5;
    if n < 0 {
        println!("n is negative");
    }

    let b = if n > 0 {
        n + n
    } else if n < 0 {
        n * n
    } else {
        1
    };

    println!("b = {}", b);
}


fn main() {
    fn worker() -> u32 {
        println!("worker");
        100
    }

    let handler = std::thread::spawn(worker);

    match handler.join() {
        Ok(n) => println!("{n}"),
        Err(e) => println!("{:?}", e),
    }

}


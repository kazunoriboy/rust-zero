fn main() {
    let mut n = 0;
    let mut m = 0;
    'main_loop: loop {
        println!("loop");
        loop {
            println!("loop in loop");
            if n == 5 {
                break 'main_loop;
            }
            n += 1;
        }
        if (m == 5) {
            break;
        }
    }

}


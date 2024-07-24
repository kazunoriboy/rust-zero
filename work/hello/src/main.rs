fn main() {
    fn add<'a>(x: &'a mut i32, y: &'a i32) {
        *x += *y;
    }

    let mut x = 10;
    {
        let y = 20;
        add(&mut x, &y);
    }
    println!("{}", x);
}


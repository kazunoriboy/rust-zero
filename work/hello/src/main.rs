fn main() {
    let a: i32 = 10;
    let p: i32 = 6;
    let b: &i32 = &p;

    fn square<'a>(x: &'a i32) -> i32 {
        x * x
    }

    let res = square(b);
    println!("res: {}", res);

    #[derive(Debug)]
    struct Foo<'a> {
        x: &'a i32,
    }

    let foo = Foo { x: &a };
    println!("{:?}", foo);
}


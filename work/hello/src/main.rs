fn main() {
    println!("短絡評価");
    println!("{}", a() || b());

    println!("非短絡評価");
    println!("{}", a() | b());
}

fn a() -> bool {
    println!("関数aを実行");
    true
}

fn b() -> bool {
    println!("関数bを実行");
    true
}


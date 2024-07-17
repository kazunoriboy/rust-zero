use std::collections::BTreeSet;

fn main() {
    let mut s = BTreeSet::new();
    s.insert(100);
    s.insert(400);
    s.insert(6);
    s.insert(1);

    for n in s.iter() {
        println!("{}", n);
    }
}


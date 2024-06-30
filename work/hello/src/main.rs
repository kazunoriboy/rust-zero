fn main() {
    make_pair::<u8, bool>(40, false);
    make_pair(10, true);
}
fn make_pair<T1, T2>(a: T1, b: T2) -> (T1, T2) {
    (a, b)
}


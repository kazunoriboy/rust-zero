fn main() {
    fn maybe_fail() -> Option<u32> {
        Some(10)
    }

    let result = maybe_fail();
    let result = result.unwrap();
}


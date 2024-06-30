fn main() {
    struct Buffer<const S: usize> {
        buf: [u8; S],
    }

    let buf = Buffer::<128> {
        buf: [0; 128]
    };
}


fn main() {
    #[derive(Debug)]
    enum Storage {
        HDD { size: u32, rpm: u32 },
        SSD(u32),
    }
    impl Storage {
        fn get_size(&self) -> u32 {
            match self {
                Storage::HDD { size: s, .. } => *s,
                Storage::SSD(s) => *s,
            }
        }

        fn set_size(&mut self, size: u32) {
            match self {
                Storage::HDD { size: s, .. } => *s = size,
                Storage::SSD(s) => *s = size,
            }
        }
    }

    let mut s = Storage::SSD(512);
    println!("{:?}", s);
    let size = s.get_size();
    println!("Size: {}", size);
    s.set_size(1024);
    println!("{:?}", s);

}


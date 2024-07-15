fn main() {
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
}


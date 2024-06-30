fn main() {
    enum Storage {
        HDD{ size: u32, rmp: u32 },
        SSD(u32),
    }

    struct PCSpec {
        cpus: u16,
        memory: u32,
        storage: Storage,
    }

    let spec = PCSpec {
        cpus: 8,
        memory: 16,
        storage: Storage::SSD(1024),
    };

    println!("{}", spec.cpus);
}

fn main() {
    #[derive(Debug)]
    struct PCSpec {
        cpus: u16,
        memory: u32,
        storage: Storage,
    }
    #[derive(Debug)]
    enum Storage {
        HDD { size: u32, rpm: u32 },
        SSD(u32),
    }
    let spec = PCSpec {
        cpus: 4,
        memory: 16,
        storage: Storage::SSD(512),
    };

    match &spec {
        PCSpec {
            storage: Storage::SSD(512),
            ..
        } => {
            println!("You have a 512GB SSD");
        }
        PCSpec {
            cpus: 4 | 8,
            memory: m,
            storage: _,
        } => {
            println!("4 or 8 CPUs");
            println!("Memory: {} GB", *m);
        }
        PCSpec { memory: m, .. } if *m < 4 => {
            println!("4GiB or less memory");
        }
        _ => ()
    }

}


fn main() {
    enum Storage {
        HDD { size: u32, rpm: u32 },
        SSD(u32),
    }

    let hdd = Storage::HDD { size: 512, rpm: 7200 };
    let ssd = Storage::SSD(512);
}

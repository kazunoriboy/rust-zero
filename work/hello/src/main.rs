fn main() {
    fn average(v: &[f32]) -> Option<f32> {
        if v.is_empty() {
            return None;
        }
        let mut total = 0.0;
        for n in v {
            total += n;
        }

        Some(total / v.len() as f32)
    }
}


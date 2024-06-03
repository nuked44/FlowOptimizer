pub struct Machine {
    pub id: usize,
    pub name: String,
    pub throughput_per_min: f64,
}

impl Machine {
    pub fn new(id: usize, name: String, throughput_per_min: f64) -> Machine {
        Machine {
            id,
            name,
            throughput_per_min,
        }
    }
}

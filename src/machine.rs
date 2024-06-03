pub struct Machine {
    pub name: String,
    pub throughput_per_min: f64,
}

impl Machine {
    pub fn new(name: String, throughput_per_min: f64) -> Machine {
        Machine { name, throughput_per_min }
    }
}

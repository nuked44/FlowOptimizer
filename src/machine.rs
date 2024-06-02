pub struct Machine {
    pub name: String,
    pub throughput: f64,
}

impl Machine {
    pub fn new(name: String, throughput: f64) -> Machine {
        Machine { name, throughput }
    }
}
